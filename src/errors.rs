use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    Unknownformat { format: String },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::Unknownformat { format } => write!(f, "Unknown file extention `{}`", format),
        }
    }
}

impl std::error::Error for RuntimeError {}
