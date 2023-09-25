use crate::AST;


#[derive(Debug,  Clone, PartialEq, Eq, Hash)]
pub enum ASTlogic {
    BinaryOperation {
        left: Box<AST>,
        op: Operator,
        right: Box<AST>,
    },
}
