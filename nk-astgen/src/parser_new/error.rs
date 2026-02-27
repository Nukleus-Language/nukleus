use lexer::neo_tokens::Token;
use nk_diagnostics::{Diagnostic, Span};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseErrorCode {
    ExpectedToken,
    ExpectedStatement,
    ExpectedExpression,
    UnexpectedToken,
    InvalidNumberFormat,
    UnexpectedEOF,
    MismatchedArgumentCount,
}

impl ParseErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExpectedToken => "NK-PARSE-001",
            Self::ExpectedStatement => "NK-PARSE-002",
            Self::ExpectedExpression => "NK-PARSE-003",
            Self::UnexpectedToken => "NK-PARSE-004",
            Self::InvalidNumberFormat => "NK-PARSE-005",
            Self::UnexpectedEOF => "NK-PARSE-006",
            Self::MismatchedArgumentCount => "NK-PARSE-007",
        }
    }
}

impl fmt::Display for ParseErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[allow(missing_docs)]
#[allow(dead_code)]
pub struct AstGenError {
    pub message: AstError,
    pub pretty_display: String,
    pub span: Option<(usize, usize)>,
}

impl AstGenError {
    pub fn new(message: AstError) -> Self {
        AstGenError {
            message,
            pretty_display: String::new(),
            span: None,
        }
    }

    pub fn with_span(mut self, line: usize, column: usize) -> Self {
        self.span = Some((line, column));
        self
    }

    pub fn to_diagnostic(&self) -> Diagnostic {
        let span = self.span.map(|(line, col)| Span::point(line, col));
        let mut diag = Diagnostic::error(self.message.code(), self.message.to_string(), span);
        if !self.pretty_display.is_empty() {
            diag = diag.with_note(self.pretty_display.clone());
        }
        diag
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
    MismatchedArgumentCount(usize, usize),
}
impl AstError {
    pub fn code(&self) -> ParseErrorCode {
        match self {
            AstError::ExpectedToken(_) => ParseErrorCode::ExpectedToken,
            AstError::ExpectedStatement() => ParseErrorCode::ExpectedStatement,
            AstError::ExpectedExpression() => ParseErrorCode::ExpectedExpression,
            AstError::UnexpectedToken() => ParseErrorCode::UnexpectedToken,
            AstError::InvalidNumberFormat(_) => ParseErrorCode::InvalidNumberFormat,
            AstError::UnexpectedEOF() => ParseErrorCode::UnexpectedEOF,
            AstError::MismatchedArgumentCount(_, _) => ParseErrorCode::MismatchedArgumentCount,
        }
    }
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
            AstError::MismatchedArgumentCount(a, b) => {
                write!(f, "Mismatched argument count: {} vs {}", a, b)
            }
        }
    }
}
