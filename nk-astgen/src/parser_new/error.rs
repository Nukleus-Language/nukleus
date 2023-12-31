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
    ExpectedExpression(),
    UnexpectedToken(),
    UnexpectedEOF(),
}
impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::ExpectedToken(t) => write!(
                f,
                "Expected token: {} on line: {} column: {}",
                t.token_type, t.metadata.line, t.metadata.column
            ),
            AstError::ExpectedStatement() => write!(f, "Expected statement"),
            AstError::ExpectedExpression() => write!(f, "Expected expression"),
            AstError::UnexpectedToken() => write!(f, "Unexpected token"),
            AstError::UnexpectedEOF() => write!(f, "Unexpected EOF"),
            
        }
    }
}
