use lexer::neo_tokens::Token;

use std::fmt;

#[allow(missing_docs)]
#[allow(dead_code)]
pub struct AstGenError {
    //pub line: usize,
    //pub column: usize,
    pub message: AstError,
    pub pretty_display: String,
}
impl AstGenError {
    pub fn new(message: AstError) -> Self {
        AstGenError {
            message,
            pretty_display: "".to_string(),
        }
    }
}
impl fmt::Display for AstGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum AstError {
    ExpectedToken(Token),
    ExpectedStatement(),
    ExpectedExpression(),
    UnexpectedToken(),
    InvalidNumberFormat(String),
    UnexpectedEOF(),
}
impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::ExpectedToken(t) => write!(f, "Expected token: {} ", t.token_type),
            AstError::ExpectedStatement() => write!(f, "Expected statement"),
            AstError::ExpectedExpression() => write!(f, "Expected expression"),
            AstError::UnexpectedToken() => write!(f, "Unexpected token"),
            AstError::InvalidNumberFormat(num) => write!(f, "Invalid number format: {}", num),
            AstError::UnexpectedEOF() => write!(f, "Unexpected EOF"),
        }
    }
}
