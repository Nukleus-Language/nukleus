use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidIdentifierChar(String),
    InvalidIdentifierNum(String),
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidString(String),
    InvalidSymbol(String),
    InvalidToken(String),
    UnexpectedEndOfInput,
    UnknownCharacter(String),
    UnmatchedQuote,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidIdentifierChar(id) =>
                write!(f, "Invalid identifier character in '{}'- identifiers can only contain letters, numbers and underscores", id),
            LexerError::InvalidIdentifierNum(id) =>
                write!(f, "Invalid identifier '{}' - identifiers cannot start with a number", id),
            LexerError::InvalidNumber(n) =>
                write!(f, "Invalid number: {}", n),
            LexerError::InvalidOperator(op) =>
                write!(f, "Invalid operator: {}", op),
            LexerError::InvalidString(s) =>
                write!(f, "Invalid string: {}", s),
            LexerError::InvalidSymbol(s) =>
                write!(f, "Invalid symbol: {}", s),
            LexerError::InvalidToken(t) =>
                write!(f, "Invalid token: {}", t),
            LexerError::UnexpectedEndOfInput =>
                write!(f, "Unexpected end of input"),
            LexerError::UnknownCharacter(c) =>
                write!(f, "Unknown character: {}", c),
            LexerError::UnmatchedQuote =>
                write!(f, "Unmatched quote"),
        }
    }
}

impl std::error::Error for LexerError {}

impl From<std::io::Error> for LexerError {
    fn from(err: std::io::Error) -> Self {
        LexerError::InvalidString(err.to_string())
    }
}

impl From<String> for LexerError {
    fn from(err: String) -> Self {
        LexerError::InvalidToken(err)
    }
}

impl From<&str> for LexerError {
    fn from(err: &str) -> Self {
        LexerError::InvalidToken(err.to_string())
    }
}
