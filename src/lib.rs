pub mod compiler;
pub mod core;

pub use crate::compiler::interpreter::Interpreter;
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
//mod test;
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Interpreter::from_source("1 + 2").unwrap() as i64, 3);
        // assert_eq!(Interpreter::source("(1 + 2)").unwrap() as i64, 3);
        assert_eq!(Interpreter::from_source("2 + (2 - 1)").unwrap() as i64, 3);
        assert_eq!(Interpreter::from_source("(2 + 3) - 1").unwrap() as i64, 4);
        assert_eq!(
            Interpreter::from_source("1 + ((2 + 3) - (2 + 3))").unwrap() as i64,
            1
        );
    }
}
