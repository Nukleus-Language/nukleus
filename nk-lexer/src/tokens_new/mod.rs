mod operators;
mod statements;
mod symbols;
mod types;

use std::fmt;

pub use operators::Assign;
pub use operators::Logical;
pub use operators::Operator;

pub use statements::Statement;

pub use types::TypeName;
pub use types::TypeValue;

pub use symbols::Symbol;

// FEAT:TASK: add trait for the Tokens for error line, pos support

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum Token {
    Operator(Operator),
    Logical(Logical),
    Assign(Assign),
    Statement(Statement),
    TypeName(TypeName),
    TypeValue(TypeValue),
    Symbol(Symbol),
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(op) => write!(f, "{}", op),
            Token::Logical(log) => write!(f, "{}", log),
            Token::Assign(asn) => write!(f, "{}", asn),
            Token::Statement(st) => write!(f, "{}", st),
            Token::TypeName(tn) => write!(f, "{}", tn),
            Token::TypeValue(tv) => write!(f, "{}", tv),
            Token::Symbol(sym) => write!(f, "{}", sym),
            Token::EOF => write!(f, "EOF"),
        }
    }
}


