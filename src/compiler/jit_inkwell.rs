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
            //let recursive_builder = RecursiveBuilder::new(i64_type, &builder);
            //let return_value = recursive_builder.build(&node);
            //builder.build_return(Some(&return_value));
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
/*struct RecursiveBuilder<'a> {
    i64_type: IntType<'a>,
    builder: &'a Builder<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(i64_type: IntType<'a>, builder: &'a Builder) -> Self {
        Self { i64_type, builder }
    }
   pub fn build(&self, ast: &Node) -> IntValue {
        match ast {
            //Node::Int(n) => self.i64_type.const_int(*n as u64, true),
            Node::NumericTerm(lhs, rhs, op) => {
                let lhs = self.build(lhs);
                let rhs = self.build(rhs);
                match op {
                    Ops::Add => self.builder.build_int_add(lhs, rhs, "plus_temp"),
                    Ops::Sub => self.builder.build_int_sub(lhs, rhs, "minus_temp"),
                    Ops::Multply => self.builder.build_int_mul(lhs, rhs, "multply_temp"),
                    Ops::Divide => self.builder.build_int_exact_signed_div(lhs, rhs, "div_temp"),
                    Ops::And => self.builder.build_and(lhs, rhs, "and_temp"),
                    Ops::Or => self.builder.build_or(lhs, rhs, "or_temp"),
                    Ops::Xor =>self.builder.build_xor(lhs, rhs, "xor_temp"),
                    Ops::NotEqual => self.builder.build_int_compare(inkwell::IntPredicate::NE,lhs, rhs, "notequal_temp"),
                    Ops::Equal => self.builder.build_int_compare(inkwell::IntPredicate::EQ,lhs, rhs, "equal_temp"),
                    Ops::Leq => self.builder.build_int_compare(inkwell::IntPredicate::SLE,lhs, rhs, "leq_temp"),
                    Ops::Geq => self.builder.build_int_compare(inkwell::IntPredicate::SGE,lhs, rhs, "req_temp"),
                    Ops::Less => self.builder.build_int_compare(inkwell::IntPredicate::SLT,lhs, rhs, "less_temp"),
                    Ops::Greater =>self.builder.build_int_compare(inkwell::IntPredicate::SGT,lhs, rhs, "greater_temp"),
                }
            }
            _ => {
                // TODO
                "0";
            }
        }
    }
}*/
