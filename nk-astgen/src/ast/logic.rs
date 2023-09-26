use std::fmt;

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
impl fmt::Display for ASTOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTOperator::Add => write!(f, "+"),
            ASTOperator::Subtract => write!(f, "-"),
            ASTOperator::Or => write!(f, "|"),
            ASTOperator::And => write!(f, "&"),
            ASTOperator::Multiply => write!(f, "*"),
            ASTOperator::Divide => write!(f, "/"),
            ASTOperator::Remainder => write!(f, "%"),
            ASTOperator::BitAnd => write!(f, "&"),
            ASTOperator::BitOr => write!(f, "|"),
            ASTOperator::BitXor => write!(f, "^"),
            ASTOperator::BitShiftLeft => write!(f, "<<"),
            ASTOperator::BitShiftRight => write!(f, ">>"),
            ASTOperator::Equals => write!(f, "=="),
            ASTOperator::NotEquals => write!(f, "!="),
            ASTOperator::Less => write!(f, "<"),
            ASTOperator::LessEquals => write!(f, "<="),
            ASTOperator::Greater => write!(f, ">"),
            ASTOperator::GreaterEquals => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ASTlogic {
    BinaryOperation {
        left: Box<AST>,
        op: ASTOperator,
        right: Box<AST>,
    },
}
impl fmt::Display for ASTlogic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTlogic::BinaryOperation { left, op, right } => write!(f, "{} {} {}", left, op, right),
        }
    }
}
