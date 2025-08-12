use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LexcialError {
    pub line: usize,
    pub column: usize,
    pub message: LexError,
}

impl LexcialError {
    #[inline(always)]
    pub fn new_invalid_statement(statement: &str, line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            message: LexError::InvalidStatement(statement.to_string()),
        }
    }

    #[inline(always)]
    pub fn new_invalid_type(typename: &str, line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            message: LexError::InvalidTypeName(typename.to_string()),
        }
    }
}

impl fmt::Display for LexcialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical Error: {}", self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum LexError {
    InvalidCharacter(char),
    InvalidNumber(String),
    InvalidIdentifier(String),
    InvalidOperator(String),
    InvalidSymbol(String),
    InvalidStatement(String),
    InvalidTypeName(String),
    InvalidDoubleSymbol(String),
    ExpectedQuote(),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::InvalidCharacter(c) => write!(f, "Invalid identifier character in '{}'- identifiers can only contain letters, numbers and underscores", c),
            LexError::InvalidNumber(n) => write!(f, "Invalid number: {}", n),
            LexError::InvalidIdentifier(i) => write!(f, "Invalid identifier: {}", i),
            LexError::InvalidOperator(o) => write!(f, "Invalid operator: {}", o),
            LexError::InvalidSymbol(s) => write!(f, "Invalid symbol: {}", s),
            LexError::InvalidStatement(s) => write!(f, "Invalid statement: {}", s),
            LexError::InvalidTypeName(t) => write!(f, "Invalid type name: {}", t),
            LexError::InvalidDoubleSymbol(s) => write!(f, "Invalid double symbol: {}", s),
            LexError::ExpectedQuote() => write!(f, "Expected quote"),
        }
    }
}
