mod operators;
mod statements;
mod symbols;
mod types;

use std::fmt;
// use std::path::PathBuf;

pub use operators::Assign;
pub use operators::Logical;
pub use operators::Operator;

pub use statements::Statement;

pub use types::TypeName;
pub use types::TypeValue;

pub use symbols::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub struct TokenMetadata {
    pub line: usize,
    pub column: usize,
}

impl TokenMetadata {
    pub fn new(line: usize, column: usize /* , src_location: Option<PathBuf> */) -> Self {
        TokenMetadata { line, column }
    }
}
impl fmt::Display for TokenMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line: {}, column: {}", self.line, self.column)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Operator(Operator),
    Logical(Logical),
    Assign(Assign),
    Statement(Statement),
    TypeName(TypeName),
    TypeValue(TypeValue),
    Symbol(Symbol),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub metadata: TokenMetadata,
}
impl Token {
    pub fn new(token_type: TokenType, metadata: TokenMetadata) -> Self {
        Token {
            token_type,
            metadata,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Operator(op) => write!(f, "{}", op),
            TokenType::Logical(log) => write!(f, "{}", log),
            TokenType::Assign(assign) => write!(f, "{}", assign),
            TokenType::Statement(stat) => write!(f, "{}", stat),
            TokenType::TypeName(typename) => write!(f, "{}", typename),
            TokenType::TypeValue(typeval) => write!(f, "{}", typeval),
            TokenType::Symbol(symbol) => write!(f, "{}", symbol),
            _ => write!(f, "EOF"),
        }
    }
}
