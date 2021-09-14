use inkwell::{
    builder::Builder, context::Context, execution_engine::JitFunction, types::IntType,
    values::IntValue, OptimizationLevel,
};

use crate::{Compile, Node, Ops, Result};

type JitFunc = unsafe extern "C" fn() -> i64;

pub struct Jit;

impl Compile for Jit {
    type Output = Result<i64>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let context = Context::create();
        let module = context.create_module("nukleus");

        let builder = context.create_builder();

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);

        let function = module.add_function("jit", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        for node in ast {
            let recursive_builder = RecursiveBuilder::new(i64_type, &builder);
            let return_value = recursive_builder.build(&node);
            builder.build_return(Some(&return_value));
        }
        /*println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );*/

        unsafe {
            let jit_function: JitFunction<JitFunc> = execution_engine.get_function("jit").unwrap();

            Ok(jit_function.call())
        }
    }
}
struct RecursiveBuilder<'a> {
    i64_type: IntType<'a>,
    builder: &'a Builder<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(i64_type: IntType<'a>, builder: &'a Builder) -> Self {
        Self { i64_type, builder }
    }
   pub fn build(&self, ast: &Node) -> IntValue {
        match ast {
            Node::NumericTerm(lhs, rhs, op) => {
                let lhs_eval = self.eval(lhs);
                let rhs_eval = self.eval(rhs);
                match op {
                    Ops::Add => lhs_eval + rhs_eval,
                    Ops::Sub => lhs_eval - rhs_eval,
                    Ops::Multply => lhs_eval * rhs_eval,
                    Ops::Divide => lhs_eval / rhs_eval,
                    Ops::And => lhs_eval & rhs_eval,
                    Ops::Or => lhs_eval | rhs_eval,
                    Ops::Xor => lhs_eval ^ rhs_eval,
                    _ => {
                        // TODO
                        0
                    }
                }
            }
            _ => {
                // TODO
                0
            }
        }
    }
}
