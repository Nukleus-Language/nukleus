use astgen::ast::*;
// use astgen::parser_new::Parser;
use astgen::AST;
// use lexer::lex_new_new::Lexer;

use cranelift::prelude::*;
// use cranelift_codegen::ir::entities::FuncRef;
use cranelift_codegen::ir::Signature;
use cranelift_codegen::isa::CallConv;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
// use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct VarType {
    obj: Variable,
    type_name: Option<ASTtypename>,
}

impl VarType {
    fn new(obj: Variable, type_name: Option<ASTtypename>) -> Self {
        Self { obj, type_name }
    }
}

// This is a host function that you need to define and make accessible to the JIT.
extern "C" fn print_function(arg: *const i8) {
    unsafe {
        if !arg.is_null() {
            let mut len = 0;
            // Calculate the length of the string
            while *arg.offset(len) != 0 {
                len += 1;
            }
            // Convert the byte array to a string
            let string =
                std::str::from_utf8(std::slice::from_raw_parts(arg as *const u8, len as usize))
                    .unwrap_or("Invalid string");
            print!("{}", string);
        } else {
            eprintln!("Null pointer received");
        }
    }
}

pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data description, which is to data objects what `ctx` is to functions.
    data_description: DataDescription,
    functions: HashMap<String, Signature>,
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "true").unwrap();
        flag_builder.set("opt_level", "speed_and_size").unwrap();
        flag_builder.set("enable_alias_analysis", "true").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let mut jb = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        jb.symbol("print_function", print_function as *const u8);
        // jb.symbol("println_function", println_function as *const u8);

        let module = JITModule::new(jb);
        let functions = HashMap::new();

        // let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            functions,
            module,
        }
    }
}

impl JIT {
    pub fn compile(
        &mut self,
        input: Vec<AST>,
        file_location: &str,
        is_lib: bool,
    ) -> Result<*const u8, String> {
        let mut funcid = HashMap::new();
        self.define_print_function();
        self.define_println_function();
        // Function Signature Declaration
        for ast in input.clone() {
            if let AST::Statement(statement) = ast { if let ASTstatement::Function {
                    public: _,
                    name,
                    args,
                    statements: _,
                    return_type,
                } = statement {
                let int = self.module.target_config().pointer_type();

                for p in args.clone() {
                    match p {
                        ASTtypecomp::Argument {
                            type_name,
                            identifier: _,
                        } => {
                            self.ctx
                                .func
                                .signature
                                .params
                                .push(AbiParam::new(translate_type(int, type_name)));
                        }
                        _ => {
                            println!("Invalid Type for Argument");
                            std::process::exit(1);
                        }
                    }
                }
                let type_return = translate_type(int, return_type);

                self.ctx
                    .func
                    .signature
                    .returns
                    .push(AbiParam::new(type_return));

                self.functions
                    .insert(name.clone(), self.ctx.func.signature.clone());
                self.module.clear_context(&mut self.ctx);
                // self.module.finalize_definitions().expect("Compile Error");
            } }
        }
        for ast in input {
            match ast {
                AST::Statement(statement) => match statement {
                    ASTstatement::Import { name } => {
                        // Resolve file path based on the operating system
                        let resolved_path = resolve_file_path(&name, file_location)?;
                        let contents = std::fs::read_to_string(&resolved_path).map_err(|e| {
                            format!("Failed to read file '{}': {}", resolved_path.display(), e)
                        })?;
                        let mut new_lexer = lexer::lex_new::Lexer::new(&contents);
                        new_lexer.run();
                        let new_tokens = new_lexer.get_tokens();

                        let mut new_new_lexer = lexer::lex_new_new::Lexer::new(
                            Path::new(&name).to_path_buf(),
                            &contents,
                        );
                        let lex_result = new_new_lexer.run();
                        if lex_result.is_err() {
                            println!("Error: {}", lex_result.err().unwrap());
                        }
                        let new_new_tokens = new_new_lexer.get_tokens();

                        let mut mid_ir = astgen::parser_new::Parser::new(
                            new_new_tokens,
                            Path::new(&name).to_path_buf(),
                            &contents,
                        );
                        let ast_result = mid_ir.run();
                        if ast_result.is_err() {
                            println!("Error: {}", ast_result.err().unwrap());
                        }
                        let ast_new = mid_ir.get_asts();
                        let _ =
                            self.compile(ast_new.clone(), resolved_path.to_str().unwrap(), true);
                        for (name, signature) in self.functions.iter() {
                            println!("Function Name: {}, Signature: {:?}", name, signature);
                            println!();
                        }
                    }
                    ASTstatement::Function {
                        public: _,
                        name,
                        args,
                        statements,
                        return_type,
                    } => {
                        self.ctx.func.signature =
                            self.functions.get(name.as_str()).unwrap().clone();

                        self.translate(
                            args.clone(),
                            statements,
                            return_type,
                            self.functions.clone(),
                        )?;

                        let id = self
                            .module
                            .declare_function(&name, Linkage::Local, &self.ctx.func.signature)
                            .map_err(|e| e.to_string())?;
                        self.module
                            .define_function(id, &mut self.ctx)
                            .map_err(|e| e.to_string())
                            .expect("Compile Error");

                        funcid.insert(name.clone(), id);
                        self.module.clear_context(&mut self.ctx);
                        self.module.finalize_definitions().unwrap();
                    }
                    _ => {
                        println!("Not a Function: {:?}", statement);
                    }
                },
                _ => {
                    println!("Not a Function: {:?}", ast);
                }
            }
        }
        if !is_lib {
            #[cfg(debug_assertions)]
            println!("Finalize");

            let code = self
                .module
                .get_finalized_function(*funcid.get("main").expect("main function not found"));

            println!("code: {:?}", code);
            Ok(code)
        } else {
            Ok(std::ptr::null::<u8>() as *const u8)
        }
    }

    fn translate(
        &mut self,
        args: Vec<ASTtypecomp>,
        statements: Vec<AST>,
        return_type: ASTtypename,
        functions: HashMap<String, Signature>,
    ) -> Result<(), String> {
        let is_void = match return_type {
            ASTtypename::TypeVoid => true,
            _ => false,
        };
        let int = self.module.target_config().pointer_type();
        let type_return = translate_type(int, return_type);
        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();

        builder.append_block_params_for_function_params(entry_block);
        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        let variables = declare_variables(
            type_return,
            &mut builder,
            &args,
            &return_type,
            &statements,
            entry_block,
        );
        // Now translate the statements of the function body.
        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            functions: functions.clone(),
            module: &mut self.module,
        };
        for expr in statements {
            trans.translate_expr(expr);
        }

        // Tell the builder we're done with this function.
        trans.builder.finalize();
        println!("\nircode:\n{}\n", self.ctx.func.clone());
        Ok(())
    }
    pub fn define_print_function(&mut self) {
        let string_ptr = self.module.target_config().pointer_type();
        // let string_ptr
        self.ctx
            .func
            .signature
            .params
            .push(AbiParam::new(string_ptr));
        self.functions.insert(
            "print_function".to_string(),
            self.ctx.func.signature.clone(),
        );
        self.module.clear_context(&mut self.ctx);
    }

    pub fn define_println_function(&mut self) {
        let string_ptr = self.module.target_config().pointer_type();
        self.ctx
            .func
            .signature
            .params
            .push(AbiParam::new(string_ptr));
        self.functions.insert(
            "println_function".to_string(),
            self.ctx.func.signature.clone(),
        );
        self.module.clear_context(&mut self.ctx);
    }
}

struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, VarType>,
    functions: HashMap<String, Signature>,
    module: &'a mut JITModule,
}

impl FunctionTranslator<'_> {
    fn translate_value(&mut self, value: ASTtypevalue) -> Value {
        match value {
            ASTtypevalue::Char(c) => {
                let imm: i8 = c as i8;
                self.builder.ins().iconst(types::I8, i64::from(imm))
            }
            ASTtypevalue::QuotedString(str) => {
                // Calculate the length of the string (including the null terminator)
                let len = str.len() as i32 + 1; // +1 for null terminator

                // Create a stack slot to hold the string
                let string_slot = self.builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    len as u32,
                    8,
                ));

                // Get a pointer to the stack slot
                let string_ptr = self.builder.ins().stack_addr(types::I64, string_slot, 0);

                // Copy the string into the stack slot
                for (i, byte) in str.bytes().enumerate() {
                    let byte_val = self.builder.ins().iconst(types::I8, i64::from(byte));
                    self.builder
                        .ins()
                        .store(MemFlags::new(), byte_val, string_ptr, i as i32);
                }

                // Add null terminator
                let null_val = self.builder.ins().iconst(types::I8, 0);
                self.builder
                    .ins()
                    .store(MemFlags::new(), null_val, string_ptr, len - 1);

                // Return the pointer to the string as a Value
                string_ptr
            }
            ASTtypevalue::I8(i) => {
                let imm: i8 = i;
                self.builder.ins().iconst(types::I8, i64::from(imm))
            }
            ASTtypevalue::I16(i) => {
                let imm: i16 = i;
                self.builder.ins().iconst(types::I16, i64::from(imm))
            }
            ASTtypevalue::I32(i) => {
                let imm: i32 = i;
                self.builder.ins().iconst(types::I32, i64::from(imm))
            }
            ASTtypevalue::I64(i) => self.builder.ins().iconst(types::I64, i),

            ASTtypevalue::Identifier(id) => {
                if let Some(variable) = self.variables.get(&id) {
                    self.builder.use_var(variable.obj)
                } else {
                    eprintln!("Variable {} not found", id);
                    // Search for simliar variable / Function
                    let suggestion = self.variables.iter().find(|(k, _)| k.contains(&id));
                    if let Some((suggestion, _)) = suggestion {
                        eprintln!("Did you mean {}?", suggestion);
                    }
                    std::process::exit(1);
                }
            }
            ASTtypevalue::FunctionCall { name, args } => {
                // Clone the function reference before entering the loop
                let functioninfo_res = self.functions.get(&name).cloned();

                let functioninfo = match functioninfo_res {
                    Some(functioninfo) => functioninfo,
                    None => {
                        eprintln!("Function {} not found", name);
                        let suggestion = self.functions.iter().find(|(k, _)| k.contains(&name));
                        if let Some((suggestion, _)) = suggestion {
                            eprintln!("Did you mean {}?", suggestion);
                        }
                        std::process::exit(1);
                    }
                };

                // let ref_function = functioninfo.fnref;
                // let entry_block = self.builder.create_block();
                // self.builder.switch_to_block(entry_block);

                let func = self
                    .module
                    .declare_function(&name, Linkage::Import, &functioninfo)
                    .expect("Function Link Error");
                let func = self.module.declare_func_in_func(func, self.builder.func);

                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.translate_expr(arg.clone()));
                }
                // self.module.define_function(func, &mut self.ctx);
                // self
                // .module
                // .get_finalized_function(functioninfo.id);
                // Deref ref_function here as call expects a reference
                let call = self.builder.ins().call(func, arguments.as_slice());
                let results = self.builder.inst_results(call);
                assert_eq!(results.len(), 1);
                
                // self.builder.seal_all_blocks();
                // self.builder.finalize();
                // self.builder.finalize();
                // self.module.clear_context(&mut self.codegen_context);
                // self.module.finalize_definitions()?;
                results[0]

                // self.builder.ins().iconst(self.int, 0)
            }
            ASTtypevalue::Array(values) => {
                // Handle array translation here
                // Iterate through the values and translate each element
                let translated_values: Vec<Value> = values
                    .iter()
                    .map(|val| self.translate_value(val.clone()))
                    .collect();
                // Populate the array with the translated values
                let array_len = translated_values.len() as i32;
                let array_type = self.module.target_config().pointer_type();
                let array_var = self.builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    array_len as u32,
                    8,
                ));
                for (i, val) in translated_values.iter().enumerate() {
                    let index = self.builder.ins().iconst(types::I32, i as i64);
                    let elem_addr = self
                        .builder
                        .ins()
                        .stack_addr(array_type, array_var, i as i32);
                    self.builder
                        .ins()
                        .store(MemFlags::new(), *val, elem_addr, 0);
                }
                let array_ptr = self.builder.ins().stack_load(array_type, array_var, 0);
                array_ptr
            }
            ASTtypevalue::TypeVoid => self.builder.ins().iconst(self.int, 0),
            _ => {
                println!("Unsupported type: {:?}", value);
                self.builder.ins().iconst(self.int, 0)
            }
        }
    }

    fn translate_expr(&mut self, expr: AST) -> Value {
        match expr {
            AST::Statement(statement) => {
                match statement {
                    ASTstatement::Assignment { left, op, right } => {
                        // let lhs = self.translate_expr(*left);
                        // let rhs = self.translate_expr(*right);

                        match *left {
                            AST::TypeValue(value) => match value {
                                ASTtypevalue::Identifier(id) => {
                                    self.translate_assign(id, op, *right)
                                }
                                _ => {
                                    println!("Unsupported statementVal: {:?}", value);
                                    self.builder.ins().iconst(self.int, 0)
                                }
                            },
                            _ => {
                                println!("Unsupported statementVal: ");
                                self.builder.ins().iconst(self.int, 0)
                            }
                        }
                    }
                    ASTstatement::Let {
                        name,
                        type_name: _,
                        value,
                    } => {
                        let value = self.translate_expr(*value.unwrap());
                        self.builder
                            .def_var(self.variables.get(&name).unwrap().obj, value);
                        value
                    }
                    ASTstatement::Print { value, args } => {
                        self.translate_print(*value, args);
                        // Return a dummy value since print does not return anything
                        self.builder.ins().iconst(self.int, 0)
                    }
                    ASTstatement::Println { value, args } => {
                        self.translate_println(*value, args);
                        // Return a dummy value since print does not return anything
                        self.builder.ins().iconst(self.int, 0)
                    }
                    ASTstatement::For {
                        start,
                        end,
                        value,
                        statements,
                    } => self.translate_for(start, end, value, statements),
                    ASTstatement::If {
                        condition,
                        statements,
                        elif,
                        else_statements,
                    } => self.translate_if_else(*condition, statements, elif, else_statements),
                    ASTstatement::Return { value } => {
                        // if *value == AST::TypeValue(ASTtypevalue::TypeVoid) {
                        // println!("return void");
                        // return self.builder.ins().iconst(self.int, 0);
                        // }
                        let value_val = self.translate_expr(*value.clone());
                        self.builder.ins().return_(&[value_val]);
                        // self.builder.seal_all_blocks();
                        // self.builder
                        //     .def_var(*self.variables.get("return").unwrap(), value_val);
                        value_val
                    }
                    _ => {
                        println!("Unsupported statement: {:?}", statement);
                        self.builder.ins().iconst(self.int, 0)
                    }
                }
            }

            AST::Logic(logic) => match logic {
                ASTlogic::BinaryOperation { left, op, right } => {
                    let lhs = self.translate_expr(*left);
                    let rhs = self.translate_expr(*right);
                    match op {
                        ASTOperator::Add => self.builder.ins().iadd(lhs, rhs),
                        ASTOperator::Subtract => self.builder.ins().isub(lhs, rhs),
                        ASTOperator::Multiply => self.builder.ins().imul(lhs, rhs),
                        ASTOperator::Divide => self.builder.ins().udiv(lhs, rhs),
                        ASTOperator::Equals => self.builder.ins().icmp(IntCC::Equal, lhs, rhs),
                        ASTOperator::NotEquals => {
                            self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs)
                        }
                        ASTOperator::Less => {
                            self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs)
                        }
                        ASTOperator::LessEquals => {
                            self.builder
                                .ins()
                                .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs)
                        }
                        ASTOperator::Greater => {
                            self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs)
                        }
                        ASTOperator::GreaterEquals => {
                            self.builder
                                .ins()
                                .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs)
                        }
                        ASTOperator::And => {
                            let lhs_bool = self.builder.ins().icmp_imm(IntCC::NotEqual, lhs, 0);
                            let rhs_bool = self.builder.ins().icmp_imm(IntCC::NotEqual, rhs, 0);
                            self.builder.ins().band(lhs_bool, rhs_bool)
                            // self.builder.ins().icmp_imm(IntCC::NotEqual, and_result, 0)
                        }
                        ASTOperator::Or => {
                            let lhs_bool = self.builder.ins().icmp_imm(IntCC::NotEqual, lhs, 0);
                            let rhs_bool = self.builder.ins().icmp_imm(IntCC::NotEqual, rhs, 0);
                            self.builder.ins().bor(lhs_bool, rhs_bool)
                            // self.builder.ins().icmp_imm(IntCC::NotEqual, or_result, 0)
                        }
                        ASTOperator::BitAnd => self.builder.ins().band(lhs, rhs),
                        ASTOperator::BitOr => self.builder.ins().bor(lhs, rhs),
                        _ => {
                            println!("Unsupported operator: {:?}", op);
                            self.builder.ins().iconst(self.int, 0)
                        }
                    }
                }
            },
            AST::TypeValue(value) => self.translate_value(value),
            _ => {
                println!("Unsupported expression: {:?}", expr);
                self.builder.ins().iconst(self.int, 0)
            }
        }
    }
    // fn translate_print(&mut self, value: AST, args: Vec<AST>) {
    //     let str_val = value.to_string();
    //     let mut formatted_str = str_val.clone();
    //     let mut j: usize = 0;
    //     for i in 0..str_val.matches("{}").count() {
    //         if j < args.len() {
    //             let arg_val = self.translate_expr(args[j].clone());
    //             formatted_str = formatted_str.replacen("{}", self.builder.use_var(arg_val),1);
    //             j += 1;
    //         }
    //     }
    //     let formatted_str_ast = AST::TypeValue(ASTtypevalue::QuotedString(formatted_str));
    //     let formatted_str_val = self.translate_expr(formatted_str_ast);
    //     self.emit_print_call(formatted_str_val);
    // }
    fn translate_print(&mut self, value: AST, args: Vec<AST>) {
        let str_val = value.to_string();
        let splited_str = str_val.split("{}").collect::<Vec<&str>>();
        let mut j: usize = 0;

        for i in 0..splited_str.len() {
            let ast_chunk = AST::TypeValue(ASTtypevalue::QuotedString(splited_str[i].to_string()));
            let chunk_val = self.translate_expr(ast_chunk);
            self.emit_print_call(chunk_val);

            if j < args.len() {
                let arg_val = self.translate_expr(args[j].clone());
                // Check if arg_val is a variable
                if let AST::TypeValue(ASTtypevalue::Identifier(arg)) = args[j].clone() {
                    // let arg_val = self.translate_value(arg.clone());
                    let arg_val = self.variables[&arg.to_string()].obj;
                    let var_val = self.builder.use_var(arg_val);
                    self.emit_print_call(var_val);
                    j += 1;
                    continue;
                }
                self.emit_print_call(arg_val);
                j += 1;
            }
        }
    }

    fn translate_println(&mut self, value: AST, args: Vec<AST>) {
        self.translate_print(value, args);
        // Print a newline after all segments and arguments
        let newline = AST::TypeValue(ASTtypevalue::QuotedString("\n".to_string()));
        let newline_val = self.translate_expr(newline);
        self.emit_print_call(newline_val);
    }

    // Helper function to emit a call to the print function
    fn emit_print_call(&mut self, val: Value) {
        let functioninfo = self
            .functions
            .get("print_function")
            .cloned()
            .expect("print_function not found");
        let func = self
            .module
            .declare_function("print_function", Linkage::Export, &functioninfo)
            .expect("Function Link Error");
        let local_print_func = self.module.declare_func_in_func(func, self.builder.func);
        // let formatted_str = self.translate_value(val);
        // println!("Formatted String: {:?}", formatted_str);
        println!("Value: {:?}", val);
        // let val_type = self.variables.get(&val_name).unwrap().type_name;
        let call = self.builder.ins().call(local_print_func, &[val]);
        let res = self.builder.inst_results(call);
        println!("Result: {:?}", res);
    }

    fn translate_assign(&mut self, name: String, op: ASTOperator, expr: AST) -> Value {
        let new_value = self.translate_expr(expr);
        let variable = self.variables.get(&name).unwrap();
        let var_value = self.builder.use_var(variable.obj);
        let oped_value = match op {
            ASTOperator::Assign => new_value,
            ASTOperator::AddAssign => self.builder.ins().iadd(var_value, new_value),
            ASTOperator::SubAssign => self.builder.ins().isub(var_value, new_value),
            ASTOperator::MulAssign => self.builder.ins().imul(var_value, new_value),
            ASTOperator::DivAssign => self.builder.ins().udiv(var_value, new_value),
            ASTOperator::RemAssign => self.builder.ins().urem(var_value, new_value),
            ASTOperator::BitAndAssign => self.builder.ins().band(var_value, new_value),
            ASTOperator::BitOrAssign => self.builder.ins().bor(var_value, new_value),
            ASTOperator::BitXorAssign => self.builder.ins().bxor(var_value, new_value),
            _ => {
                println!("Invalid Assign operator: {:?}", op);
                std::process::exit(1);
            }
        };
        self.builder.def_var(variable.obj, oped_value);
        oped_value
    }
    fn translate_icmp(&mut self, cmp: IntCC, lhs: AST, rhs: AST) -> Value {
        let lhs = self.translate_expr(lhs);
        let rhs = self.translate_expr(rhs);
        self.builder.ins().icmp(cmp, lhs, rhs)
    }

    fn translate_for(
        &mut self,
        start: ASTtypevalue,
        end: ASTtypevalue,
        value: ASTtypevalue,
        statements: Vec<AST>,
    ) -> Value {
        let start_name = match start.clone() {
            ASTtypevalue::Identifier(id) => id,
            _ => {
                println!("Start value of an `for` loop must be an identifier");
                std::process::exit(1);
            }
        };
        let start_value = self.translate_value(start);
        // check if the start_value is an identifier and get the id

        // println!("start_value: {}", start_value);
        // let end_value = self.translate_value(end);
        // let update_value = self.translate_value(value);

        let loop_var = self.variables.get(&start_name).unwrap().obj;

        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        // Jump to the header block to evaluate the loop condition
        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        // Fetch the current value of the loop variable
        let current_value = self.builder.use_var(loop_var);
        let end_value = self.translate_value(end);
        let cmp = self
            .builder
            .ins()
            .icmp(IntCC::SignedLessThan, current_value, end_value);
        self.builder
            .ins()
            .brif(cmp, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        // Translate the body of the loop
        for stmt in statements {
            self.translate_expr(stmt);
        }

        // Update the loop variable
        let update_value = self.translate_value(value);
        let updated_value = self.builder.ins().iadd(current_value, update_value);
        self.builder.def_var(loop_var, updated_value);

        // Jump back to the header to re-evaluate the loop condition
        self.builder.ins().jump(header_block, &[]);

        self.builder.switch_to_block(exit_block);
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);

        // Just return 0 for now
        self.builder.ins().iconst(self.int, 0)
    }

    fn translate_if_else(
        &mut self,
        condition: AST,
        // statements: Vec<AST>,
        then_body: Vec<AST>,
        // else_body: Vec<AST>,
        elif: Option<Box<AST>>,
        else_body: Option<Vec<AST>>,
    ) -> Value {
        let condition_value = self.translate_expr(condition);

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let merge_block = self.builder.create_block();

        // If-else constructs in the toy language have a return value.
        // In traditional SSA form, this would produce a PHI between
        // the then and else bodies. Cranelift uses block parameters,
        // so set up a parameter in the merge block, and we'll pass
        // the return values to it from the branches.
        self.builder.append_block_param(merge_block, self.int);

        // Test the if condition and conditionally branch.
        self.builder
            .ins()
            .brif(condition_value, then_block, &[], else_block, &[]);

        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        let mut then_return = self.builder.ins().iconst(self.int, 0);
        for expr in then_body {
            then_return = self.translate_expr(expr);
        }

        // Jump to the merge block, passing it the block return value.
        self.builder.ins().jump(merge_block, &[then_return]);

        self.builder.switch_to_block(else_block);
        self.builder.seal_block(else_block);
        let mut else_return = self.builder.ins().iconst(self.int, 0);
        if else_body.is_some() {
            for expr in else_body.unwrap() {
                else_return = self.translate_expr(expr);
            }
            println!("else_return: {}", else_return);
        }
        // let else_return = self.translate_expr(elif.unwrap());
        // for expr in else_body {
        // else_return = self.translate_expr(expr);
        // }

        // Jump to the merge block, passing it the block return value.
        self.builder.ins().jump(merge_block, &[else_return]);

        // Switch to the merge block for subsequent statements.
        self.builder.switch_to_block(merge_block);

        // We've now seen all the predecessors of the merge block.
        self.builder.seal_block(merge_block);

        // Read the value of the if-else by reading the merge block
        // parameter.
        let phi = self.builder.block_params(merge_block)[0];

        phi
    }

    fn translate_while_loop(&mut self, condition: AST, loop_body: Vec<AST>) -> Value {
        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        let condition_value = self.translate_expr(condition);
        self.builder
            .ins()
            .brif(condition_value, body_block, &[], exit_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        for expr in loop_body {
            self.translate_expr(expr);
        }
        self.builder.ins().jump(header_block, &[]);

        self.builder.switch_to_block(exit_block);

        // We've reached the bottom of the loop, so there will be no
        // more backedges to the header to exits to the bottom.
        self.builder.seal_block(header_block);
        self.builder.seal_block(exit_block);

        // Just return 0 for now.
        self.builder.ins().iconst(self.int, 0)
    }

    fn translate_call(&mut self, name: String, args: Vec<AST>) -> Value {
        let mut sig = self.module.make_signature();

        // Add a parameter for each argument.
        for _arg in &args {
            sig.params.push(AbiParam::new(self.int));
        }

        // For simplicity for now, just make all calls return a single I64.
        sig.returns.push(AbiParam::new(self.int));

        let callee = self
            .module
            .declare_function(&name, Linkage::Import, &sig)
            .expect("problem declaring function");
        let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.translate_expr(arg))
        }
        let call = self.builder.ins().call(local_callee, &arg_values);
        self.builder.inst_results(call)[0]
    }

    fn translate_global_data_addr(&mut self, name: String) -> Value {
        let sym = self
            .module
            .declare_data(&name, Linkage::Export, true, false)
            .expect("problem declaring data object");
        let local_id = self.module.declare_data_in_func(sym, self.builder.func);

        let pointer = self.module.target_config().pointer_type();
        self.builder.ins().symbol_value(pointer, local_id)
    }
}
fn translate_type(base_int: types::Type, typename: ASTtypename) -> Type {
    match typename {
        ASTtypename::TypeVoid => base_int,
        ASTtypename::I8 => types::I8,
        ASTtypename::I16 => types::I16,
        ASTtypename::I32 => types::I32,
        ASTtypename::I64 => types::I64,
        ASTtypename::QuotedString => base_int,
        ASTtypename::Array => base_int,
        _ => unimplemented!(),
    }
}

fn declare_signature(
    int: types::Type,
    args: &[ASTtypecomp],
    return_type: &ASTtypename,
) -> Signature {
    let mut sig = Signature::new(CallConv::SystemV);
    let mut params = Vec::new();
    for arg in args {
        match arg {
            ASTtypecomp::Argument {
                type_name,
                identifier: _,
            } => {
                let _type_val = translate_type(int, *type_name);
                params.push(AbiParam::new(translate_type(int, *type_name)));
            }
            _ => {
                println!("Invalid Type for Arguments");
                std::process::exit(1);
            }
        }
    }
    sig.params.append(&mut params);
    let type_val = translate_type(int, *return_type);
    sig.returns.push(AbiParam::new(type_val));

    sig
}
fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: &[ASTtypecomp],
    _the_return: &ASTtypename,
    stmts: &[AST],
    entry_block: Block,
) -> HashMap<String, VarType> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, param) in params.iter().enumerate() {
        if let ASTtypecomp::Argument {
            identifier,
            type_name,
        } = param
        {
            // Assuming ASTtypevalue has a method to_string() to convert it to a String
            let name = identifier.to_string();
            let type_val = translate_type(int, *type_name);

            let val = builder.block_params(entry_block)[i];
            let var = declare_variable(
                type_val,
                builder,
                &mut variables,
                &mut index,
                Some(*type_name),
                &name,
            );
            builder.def_var(var, val);
        } else {
            // Handle other ASTtypecomp variants or skip
            println!("Unsupported param type: {:?}", param);
        }
    }
    // let zero = builder.ins().iconst(int, 0);
    // Assuming you have a fixed name for the return variable, like "return_value"
    // let return_variable = declare_variable(int, builder, &mut variables, &mut index, "retun");
    // builder.def_var(return_variable, zero);

    // if there is no return variable, then the panic occurs
    // if !variables.contains_key("return") {
    // println!("variables: {:?}", variables);
    // panic!("No return variable");
    // }
    for expr in stmts {
        declare_variables_in_stmt(int, builder, &mut variables, &mut index, expr);
        println!("variables{}: {:?}", index, variables);
    }

    variables
}

/// Recursively descend through the AST, translating all implicit
/// variable declarations.
fn declare_variables_in_stmt(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, VarType>,
    index: &mut usize,
    expr: &AST,
) {
    match expr {
        AST::Statement(statement) => {
            match statement {
                ASTstatement::Let {
                    name,
                    type_name,
                    value: _,
                } => {
                    let type_val = match type_name {
                        Some(names) => translate_type(int, *names),
                        None => int,
                    };
                    let _ = declare_variable(type_val, builder, variables, index, *type_name, name);
                }
                ASTstatement::Assignment {
                    left,
                    op: _,
                    right: _,
                } => match *left.clone() {
                    AST::TypeValue(value) => match value {
                        ASTtypevalue::Identifier(id) => {
                            let _name = id.to_string();

                            // let _ = declare_variable(int, builder, variables, index, &name);
                        }
                        _ => {
                            println!("Unsupported statement: {:?}", value);
                        }
                    },
                    _ => {
                        println!("Unsupported statement");
                    }
                },
                // ASTstatement::Return { value: _ } => {
                // let _ = declare_variable(int, builder, variables, index, "return");
                // return ;
                // }
                // ... other cases for ASTstatement variants
                _ => {
                    // except of  the ones above, it doesn't allow to declare the variables
                    // println!("Unsupported statement: {:?}", statement);
                }
            }
        }
        // ... other cases for AST variants
        _ => {
            println!("Unsupported expression: {:?}", expr);
        }
    }
}

/// Declare a single variable declaration.
fn declare_variable(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, VarType>,
    index: &mut usize,
    type_name: Option<ASTtypename>,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), VarType::new(var, type_name));
        builder.declare_var(var, int);
        *index += 1;
    }
    var
}

// Function to resolve the file path based on the operating system
fn resolve_file_path(name: &str, main_file_location: &str) -> Result<PathBuf, String> {
    let mut resolved_path = PathBuf::new();

    // Check the target OS and set the appropriate path
    // Fallback for other POSIX systems
    #[cfg(target_family = "unix")]
    {
        resolved_path = PathBuf::from("/usr/lib/nukleus");
    }
    #[cfg(target_os = "linux")]
    {
        resolved_path = PathBuf::from("/usr/lib/nukleus");
    }

    #[cfg(target_os = "windows")]
    {
        resolved_path = PathBuf::from("C:\\Program Files\\nukleus");
    }

    #[cfg(target_os = "macos")]
    {
        resolved_path = PathBuf::from("/Library/nukleus");
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        resolved_path = PathBuf::from("/usr/local/lib/nukleus");
    }

    // Append the .nk extension to the file name
    let file_name_with_ext = format!("{}.nk", name);

    // Check if the name is a path
    if Path::new(&file_name_with_ext).is_absolute() {
        resolved_path = PathBuf::from(&file_name_with_ext);
    } else {
        // If the file is not found in the global packages directory, check the upper directory of the main file
        let main_file_dir = PathBuf::from(main_file_location);
        let main_file_parent_dir = main_file_dir.parent().unwrap();
        let new_resolved_path = main_file_parent_dir.join(&file_name_with_ext);
        return Ok(new_resolved_path);
    }

    Ok(resolved_path)
}
pub fn save_executable(code: *const u8, file_name: &str) -> Result<(), std::io::Error> {
    let mut file_path = match std::env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path.push(file_name);
            path
        }
        Err(_) => {
            let mut path = std::env::current_dir()?;
            path.push(file_name);
            path
        }
    };

    if cfg!(target_os = "windows") {
        file_path.set_extension("exe");
    }

    let mut file = std::fs::File::create(&file_path)?;
    let mut size = 0;
    while unsafe { *code.add(size) } != 0 {
        size += 1;
    }
    let slice = unsafe { std::slice::from_raw_parts(code, size) };
    file.write_all(slice)?;
    Ok(())
}
