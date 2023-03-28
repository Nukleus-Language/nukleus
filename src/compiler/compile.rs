use cranelift::prelude::*;
use cranelift_codegen::ir::{types, AbiParam, FuncRef, Function, Signature, UserFuncName};
use cranelift_codegen::isa::CallConv;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_native;

use snafu::prelude::*;

use crate::core::ast_temp::AST;
use crate::core::lexer::Tokens;

use std::collections::HashMap;

pub fn compile_and_run(ast: Vec<AST>) -> Result<(), Box<dyn std::error::Error>> {
    let target = cranelift_native::builder()
        .unwrap()
        .finish(settings::Flags::new(settings::builder()));
    let builder = JITBuilder::new(cranelift_module::default_libcall_names());
    let mut ctx = codegen::Context::for_function(Function::new());
    let mut module: JITModule = JITModule::new(builder.unwrap());

    let mut signatures = HashMap::new();
    let mut function_ids = HashMap::new();

    // Compile the AST
    let main_id = compile_module(
        &ast,
        &mut signatures,
        &mut function_ids,
        &mut module,
        mut ctx,
    )?;

    // Execute the compiled code
    let code = module.get_finalized_function(main_id);
    let compiled_fn: fn() = unsafe { std::mem::transmute(code) };
    compiled_fn();

    Ok(())
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum CompileError {
    #[snafu(display("Function not found: {}", name))]
    FuncNotFound { name: String },
}

fn compile_module(
    ast: &[AST],
    signatures: &mut HashMap<String, FuncId>,
    function_ids: &mut HashMap<String, FuncId>,
    module: &mut JITModule,
    ctx: mut codegen::Context,
) -> Result<FuncId, CompileError> {
    for stmt in ast {
        match stmt {
            AST::Function {
                public,
                name,
                args,
                statements,
                return_type,
            } => {
                let sig = create_signature(args, return_type.clone());
                let func_id = module
                    .declare_function(&name, Linkage::Export, &sig)
                    .expect("Failed to declare function");
                signatures.insert(name.clone(), func_id);

 

                ctx.func.signature = sig;
                //ctx.func.name = ExternalName::user(0, func_id.as_u32());

                let mut builder_ctx = FunctionBuilderContext::new();
                let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

                let entry_block = builder.create_block();

                builder.switch_to_block(entry_block);
                builder.seal_block(entry_block);

                let mut vars = HashMap::new();
                compile_statements(
                    statements,
                    &mut builder,
                    &mut vars,
                    &function_ids,
                    &signatures,
                )?;

                builder.ins().return_(&[]);
                builder.finalize();

                module
                    .define_function(func_id, &mut ctx)
                    .expect("Failed to define function");
                module.clear_context(&mut ctx);

                if *public {
                    function_ids.insert(name.clone(), func_id);
                }
            }
            _ => {}
        }
    }

    function_ids
        .get("main")
        .cloned()
        .ok_or(CompileError::FuncNotFound {
            name: "main".to_string(),
        })
}

fn create_signature(args: &Vec<AST>, return_type: Tokens) -> Signature {
    //let mut sig = Signature::new(CallConv::triple_default(&triple!("x86_64")));
    let mut sig = Signature::new(CallConv::SystemV);

    // TODO: Add function arguments

    /*for arg in args {
        match arg {
            /*
            AST::Argument { name, type_name } => {

                let param_type = match type_name.as_str() {
                    "int" => types::I32,
                    "float" => types::F32,

                    // Add more types here
                    _ => panic!("Unsupported type: {}", type_name),
                };
                sig.params.push(AbiParam::new(param_type));
            }*/
            //_ => panic!("Invalid AST node in function arguments"),
        }
    }*/

    // Set the return type
    let ret_type = match return_type {
        Tokens::Void => types::INVALID,
        //AST::Integer => types::I32,
        //"float" => types::F32,
        // Add more types here
        _ => panic!("Unsupported return type: {}", return_type),
    };

    if ret_type != types::INVALID {
        sig.returns.push(AbiParam::new(ret_type));
    }

    sig
}

fn compile_statements(
    statements: &[AST],
    builder: &mut FunctionBuilder,
    vars: &mut HashMap<String, Variable>,

    function_ids: &HashMap<String, FuncId>,
    signatures: &HashMap<String, FuncId>,
) -> Result<(), CompileError> {
    for stmt in statements {
        match stmt {
            AST::Let {
                name,
                type_name,
                value,
            } => {
                let var_type = match type_name.as_ref().map(|s| s.as_str()) {
                    Some("int") => types::I32,
                    Some("String") => types::I64, // Assuming pointers are 64-bit
                    _ => unimplemented!(),
                };

                let var = Variable::new(vars.len());
                vars.insert(name.clone(), var);
                builder.declare_var(var, var_type);

                let const_value = match value {
                    Tokens::Integer(i) => builder.ins().iconst(var_type, *i as i64),
                    //Tokens::QuotedString(s) => {
                    //    let data_id = builder.func.dfg.constants.insert(s.clone().into_bytes().into());
                    //    builder.ins().load(var_type, MemFlags::trusted(), data_id, 0)

                    //}
                    _ => unimplemented!(),
                };

                builder.def_var(var, const_value);
            }
            /*AST::Print { value } => {
                let print_func_id = function_ids["print"];
                let print_sig = &signatures["print"];

                let value_var = match value {
                    Tokens::Identifier(id) => *vars.get(id).unwrap(),
                    _ => unimplemented!(),
                };

                let value = builder.use_var(value_var);
                builder
                    .ins()
                    .call(ir::FuncRef::from_u32(print_func_id.as_u32()), &[value]);
            }
            AST::Println { value } => {
                let println_func_id = function_ids["println"];
                let println_sig = &signatures["println"];

                let value_var = match value {
                    Tokens::Identifier(id) => *vars.get(id).unwrap(),
                    _ => unimplemented!(),
                };

                let value = builder.use_var(value_var);
                builder
                    .ins()
                    .call(ir::FuncRef::from_u32(println_func_id.as_u32()), &[value]);
            }*/
            _ => unimplemented!(),
        }
    }

    Ok(())
}
