use crate::AST;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASTOperator {
    Add,
    Subtract,
    Or,
    And,
    Multiply,
    Divide,
    Remainder,
    BitAnd,
    BitOr,
    BitXor,
    BitShiftLeft,
    BitShiftRight,
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASTlogic {
    BinaryOperation {
        left: Box<AST>,
        op: ASTOperator,
        right: Box<AST>,
    },
}
