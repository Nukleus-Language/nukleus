pub mod compiler;
pub mod core;

pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::jit_inkwell::Jit;
pub use crate::core::ast::{Node, Ops};

pub type Result<T> = anyhow::Result<T>;

//#[cfg(interpriter)]
pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compiling the source: {}", source);
        let ast: Vec<Node> = core::parser::parse(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}

#[cfg(test)]
mod test;
