use astgen::ast::*;
use astgen::AST;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;

/// The basic JIT class.
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

    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let module = JITModule::new(JITBuilder::with_isa(
            isa,
            cranelift_module::default_libcall_names(),
        ));

        // let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }
}

impl JIT {
    pub fn compile(&mut self, input: Vec<AST>) -> Result<*const u8, String> {
        let mut funcid = HashMap::new();
        for ast in input {
            match ast {
                AST::Statement(statement) => match statement {
                    ASTstatement::Import { name } => {
                        
                    }
                    ASTstatement::Function {
                        public: _,
                        name,
                        args,
                        statements,
                        return_type,
                    } => {
                        self.translate(args, statements, return_type)?;
                        // println!("translate");
                        let id = self
                            .module
                            .declare_function(&name, Linkage::Local, &self.ctx.func.signature)
                            .map_err(|e| e.to_string())?;
                        // println!("adfdfad");
                        // println!("asdfasdf {}", self.ctx.func.signature);
                        self.module
                            .define_function(id, &mut self.ctx)
                            .map_err(|e| e.to_string())
                            .expect("Compile Error");
                        // println!("afsdfasdfasdfasd");
                        self.module.clear_context(&mut self.ctx);

                        self.module.finalize_definitions().unwrap();
                        funcid.insert(name, id);
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

        // Tell the builder we're done with this function.
        let code = self.module.get_finalized_function(*funcid.get("main").unwrap());
        println!("code: {:?}", code);
        // return Ok(());
        Ok(code)

        // Ok(())
    }

    fn translate(
        &mut self,
        args: Vec<ASTtypecomp>,
        statements: Vec<AST>,
        return_type: ASTtypename,
    ) -> Result<(), String> {
        let is_void = match return_type {
            ASTtypename::TypeVoid => true,
            _ => false,
        };
        let int = self.module.target_config().pointer_type();
        println!("int: {:?}", int);
        for _p in args.clone() {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(types::I64));
        }

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        let type_return = match return_type {
            ASTtypename::I8 => {
                types::I8
            }
            ASTtypename::I16 => {
                types::I16
            }
            ASTtypename::I32 => {
                types::I32
            }
            ASTtypename::I64 => {
                types::I64
            }
            ASTtypename::TypeVoid => {
                int
            }
            _ => {
                unimplemented!()
            }
        };
        self.ctx.func.signature.returns.push(AbiParam::new(type_return));
        println!("ir code: {}", self.ctx.func.clone());
        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        println!("entry_block: {:?}", entry_block);

        builder.append_block_params_for_function_params(entry_block);
        println!("append_block_params_for_function_params: {:?}", entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);
        println!("seal_block: {:?}", entry_block);

        let variables = declare_variables(
            type_return,
            &mut builder,
            &args,
            &return_type,
            &statements,
            entry_block,
        );
        println!("variables: {:?}", variables);
        // Now translate the statements of the function body.
        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
        };
        for expr in statements {
            trans.translate_expr(expr);
        }

        // the return value is the expression in the function with the `Return` segment anywhere in the function
        if !is_void { 
            let return_variable = trans.variables.get("return").unwrap();
            let return_value = trans.builder.use_var(*return_variable);

            // Emit the return instruction.

            trans.builder.ins().return_(&[return_value]);
        }
        
        // Tell the builder we're done with this function.
        trans.builder.finalize();
        
        Ok(())
    }
}

struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
    fn translate_type(&mut self, typename: ASTtypename) -> Type { 
        match typename {
            ASTtypename::TypeVoid => self.int,
            ASTtypename::I8 => types::I8,
            ASTtypename::I16 => types::I16,
            ASTtypename::I32 => types::I32,
            ASTtypename::I64 => types::I64,
            _ => unimplemented!(),
        }
    }
    fn translate_value(&mut self, value: ASTtypevalue) -> Value {
        match value {
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

            // ASTtypevalue::QuotedString(s) => {
            // self.builder.ins().iconst(types::I32, s.len() as i64)
            // }
            ASTtypevalue::Identifier(id) => {
                let variable = self.variables.get(&id).unwrap();
                self.builder.use_var(*variable)
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
                                ASTtypevalue::Identifier(id) => self.translate_assign(id,op, *right),
                                _ => {
                                    println!("Unsupported statement: {:?}", value);
                                    self.builder.ins().iconst(self.int, 0)
                                }
                            },
                            _ => {
                                println!("Unsupported statement: ");
                                self.builder.ins().iconst(self.int, 0)
                            }
                        }
                    }
                    ASTstatement::Let { name, type_name: _ ,value } => {
                       let value = self.translate_expr(*value.unwrap());
                        self.builder.def_var(*self.variables.get(&name).unwrap(), value);
                        value
                    }
                    ASTstatement::Print { value } => {
                        self.translate_expr(*value)

                        // todo!()
                    }
                    ASTstatement::Println { value } => self.translate_expr(*value),
                    ASTstatement::For {
                    start,
                    end,
                    value,
                    statements,
                    } => {
                        self.translate_for(start, end, value, statements)
                    }
                    ASTstatement::Return { value } => {
                        // if *value == AST::TypeValue(ASTtypevalue::TypeVoid) {
                            // println!("return void");
                            // return self.builder.ins().iconst(self.int, 0);
                        // }
                        let value_val = self.translate_expr(*value.clone());
                        self.builder.def_var(
                            *self.variables.get("return").unwrap(),
                            value_val,
                        );
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

    fn translate_assign(&mut self, name: String,op: ASTOperator ,expr: AST) -> Value {
        let new_value = self.translate_expr(expr);
        let variable = self.variables.get(&name).unwrap();
        let var_value = self.builder.use_var(*variable);
        let oped_value = match op {
            ASTOperator::Assign => {
                new_value
            }
            ASTOperator::AddAssign => {
                self.builder.ins().iadd(var_value, new_value)
            }
            ASTOperator::SubAssign => {
                self.builder.ins().isub(var_value, new_value)
            }
            ASTOperator::MulAssign => {
                self.builder.ins().imul(var_value, new_value)
            }
            ASTOperator::DivAssign => {
                self.builder.ins().udiv(var_value, new_value)
            }
            ASTOperator::RemAssign => {
                self.builder.ins().urem(var_value, new_value)
            }
            ASTOperator::BitAndAssign => {
                self.builder.ins().band(var_value, new_value)
            }
            ASTOperator::BitOrAssign => {
                self.builder.ins().bor(var_value, new_value)
            }
            ASTOperator::BitXorAssign => {
                self.builder.ins().bxor(var_value, new_value)
            }
            _ => {
                println!("Invalid Assign operator: {:?}", op);
                std::process::exit(1);
            }
        };
        self.builder.def_var(*variable, oped_value);
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
        let start_name= match start.clone() {
            ASTtypevalue::Identifier(id) => id,
            _ => {
                println!("Start value of an `for` loop must be an identifier");
                std::process::exit(1);
            }
        };
        let start_value = self.translate_value(start);
        // check if the start_value is an identifier and get the id


        println!("start_value: {}", start_value);
        // let end_value = self.translate_value(end);
        // let update_value = self.translate_value(value);

        let loop_var = *self.variables.get(&start_name).unwrap();

        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        // Jump to the header block to evaluate the loop condition
        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        // Fetch the current value of the loop variable
        let current_value = self.builder.use_var(loop_var);
        let end_value = self.translate_value(end);
        let cmp = self.builder.ins().icmp(IntCC::SignedLessThan, current_value, end_value);
        self.builder.ins().brif(cmp, body_block, &[], exit_block, &[]);

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
        then_body: Vec<AST>,
        else_body: Vec<AST>,
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
        for expr in else_body {
            else_return = self.translate_expr(expr);
        }

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

fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: &[ASTtypecomp],
    _the_return: &ASTtypename,
    stmts: &[AST],
    entry_block: Block,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, param) in params.iter().enumerate() {
        if let ASTtypecomp::Argument { identifier, type_name } = param {
            // Assuming ASTtypevalue has a method to_string() to convert it to a String
            let name = identifier.to_string();
            let type_val = match type_name {
                ASTtypename::I8 => types::I8,
                ASTtypename::I16 => types::I16,
                ASTtypename::I32 => types::I32,
                ASTtypename::I64 => types::I64,
                ASTtypename::TypeVoid => int,
                _ => unimplemented!(),
            };
            let val = builder.block_params(entry_block)[i];
            let var = declare_variable(type_val, builder, &mut variables, &mut index, &name);
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
    }

    variables
}

/// Recursively descend through the AST, translating all implicit
/// variable declarations.
fn declare_variables_in_stmt(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    expr: &AST,
) {
    match expr {
        AST::Statement(statement) => {
            match statement {
                ASTstatement::Let { name, type_name, value } => {
                    let type_val = match type_name{
                        Some(names)=> {
                            match names {
                                ASTtypename::I8 => types::I8,
                                ASTtypename::I16 => types::I16,
                                ASTtypename::I32 => types::I32, 
                                ASTtypename::I64 => types::I64,
                                ASTtypename::TypeVoid => int,
                                _ => unimplemented!(),
                            }
                        }
                        None=> {
                            int
                        }
                    };
                    let _ = declare_variable(type_val, builder, variables, index, name);
                }
                ASTstatement::Assignment {
                left,
                op: _,
                right: _,
                } => match *left.clone() {
                    AST::TypeValue(value) => match value {
                        ASTtypevalue::Identifier(id) => {
                            let name = id.to_string();

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
                ASTstatement::Return { value: _ } => {
                    let _ = declare_variable(int, builder, variables, index, "return");
                    // return ;
                }
                // ... other cases for ASTstatement variants
                _ => {
                    println!("Unsupported statement: {:?}", statement);
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
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, int);
        *index += 1;
    }
    var
}
