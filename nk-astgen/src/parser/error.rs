use lexer::tokens_new::Token;

use std::fmt;

#[allow(missing_docs)]
#[allow(dead_code)]
pub struct AstGenError {
    //pub line: usize,
    //pub column: usize,
    pub message: AstError,
}
impl fmt::Display for AstGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AST Gen Error: {}", self.message)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum AstError {
    ExpectedToken(Token),
    ExpectedStatement(),
    UnexpectedEOF(),
    ExpectedExpression(),
}
impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::ExpectedToken(t) => write!(f, "Expected token: {}", t),
            AstError::ExpectedStatement() => write!(f, "Expected statement"),
            AstError::UnexpectedEOF() => write!(f, "Unexpected EOF"),
            AstError::ExpectedExpression() => write!(f, "Expected expression"),

        }
    }
}
