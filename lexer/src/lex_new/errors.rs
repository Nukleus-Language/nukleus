use std::fmt;

pub struct LexcialError {
    pub line: usize,
    pub column: usize,
    pub message: LexError,
}
impl fmt::Display for LexcialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical Error: {}", self.message)
    }
}

pub enum LexError {
    InvalidCharacter(char),
    InvalidNumber(String),
    InvalidIdentifier(String),
    InvalidOperator(String),
    InvalidSymbol(String),
    InvalidStatement(String),
    InvalidTypeName(String),
    InvalidDoubleSymbol(String),
}
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            LexError::InvalidNumber(n) => write!(f, "Invalid number: {}", n),
            LexError::InvalidIdentifier(i) => write!(f, "Invalid identifier: {}", i),
            LexError::InvalidOperator(o) => write!(f, "Invalid operator: {}", o),
            LexError::InvalidSymbol(s) => write!(f, "Invalid symbol: {}", s),
            LexError::InvalidStatement(s) => write!(f, "Invalid statement: {}", s),
            LexError::InvalidTypeName(t) => write!(f, "Invalid type name: {}", t),
            LexError::InvalidDoubleSymbol(s) => write!(f, "Invalid double symbol: {}", s),
        }
    }
}
