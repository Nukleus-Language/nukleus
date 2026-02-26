#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    LexInvalidCharacter,
    LexInvalidNumber,
    LexInvalidIdentifier,
    LexInvalidOperator,
    LexInvalidSymbol,
    LexInvalidStatement,
    LexInvalidTypeName,
    LexInvalidDoubleSymbol,
    LexExpectedQuote,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LexInvalidCharacter => "NK-LEX-001",
            Self::LexInvalidNumber => "NK-LEX-002",
            Self::LexInvalidIdentifier => "NK-LEX-003",
            Self::LexInvalidOperator => "NK-LEX-004",
            Self::LexInvalidSymbol => "NK-LEX-005",
            Self::LexInvalidStatement => "NK-LEX-006",
            Self::LexInvalidTypeName => "NK-LEX-007",
            Self::LexInvalidDoubleSymbol => "NK-LEX-008",
            Self::LexExpectedQuote => "NK-LEX-009",
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
