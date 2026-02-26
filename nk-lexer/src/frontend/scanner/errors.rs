use std::fmt;

use crate::diagnostics::ErrorCode;
use nk_diagnostics::{Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LexicalError {
    pub line: usize,
    pub column: usize,
    pub message: LexError,
    pub note: Option<String>,
}

impl LexicalError {
    #[inline(always)]
    pub fn new_invalid_statement(statement: &str, line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            message: LexError::InvalidStatement(statement.to_string()),
            note: None,
        }
    }

    #[inline(always)]
    pub fn new_invalid_type(typename: &str, line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            message: LexError::InvalidTypeName(typename.to_string()),
            note: None,
        }
    }

    #[inline(always)]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    #[inline(always)]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let mut diagnostic = Diagnostic::error(
            self.message.code(),
            self.message.to_string(),
            Some(Span::point(self.line, self.column)),
        );
        if let Some(note) = &self.note {
            diagnostic = diagnostic.with_note(note.clone());
        }
        diagnostic
    }
}

impl fmt::Display for LexicalError {
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

impl LexError {
    pub fn code(&self) -> ErrorCode {
        match self {
            LexError::InvalidCharacter(_) => ErrorCode::LexInvalidCharacter,
            LexError::InvalidNumber(_) => ErrorCode::LexInvalidNumber,
            LexError::InvalidIdentifier(_) => ErrorCode::LexInvalidIdentifier,
            LexError::InvalidOperator(_) => ErrorCode::LexInvalidOperator,
            LexError::InvalidSymbol(_) => ErrorCode::LexInvalidSymbol,
            LexError::InvalidStatement(_) => ErrorCode::LexInvalidStatement,
            LexError::InvalidTypeName(_) => ErrorCode::LexInvalidTypeName,
            LexError::InvalidDoubleSymbol(_) => ErrorCode::LexInvalidDoubleSymbol,
            LexError::ExpectedQuote() => ErrorCode::LexExpectedQuote,
        }
    }
}
