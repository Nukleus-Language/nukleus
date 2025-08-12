use std::fmt;

#[derive(Debug)]
pub enum InterpretingError {
    SyntaxError { source: String },
    RuntimeError { source: String },
}

impl fmt::Display for InterpretingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpretingError::SyntaxError { source } => write!(f, "Syntax error: {}", source),
            InterpretingError::RuntimeError { source } => write!(f, "Runtime error: {}", source),
        }
    }
}

impl std::error::Error for InterpretingError {}       

