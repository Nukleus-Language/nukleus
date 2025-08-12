use std::fmt;

#[derive(Debug, Clone)]
pub enum AstParseError {
    UnknownToken { token: String },
    ExpectedOther { token: String },
    EndOfFile,
    Unknown,
}

impl fmt::Display for AstParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstParseError::UnknownToken { token } => write!(f, "Unknown token `{}`", token),
            AstParseError::ExpectedOther { token } => write!(f, "Expected `{}`", token),
            AstParseError::EndOfFile => write!(f, "End of file"),
            AstParseError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for AstParseError {}
