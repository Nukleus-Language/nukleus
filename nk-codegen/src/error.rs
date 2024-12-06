use std::fmt;

#[derive(Debug)]
pub enum CodegenError {
    CompilationError(String),
    FunctionNotFound(String),
    InvalidString(String),
    IoError(std::io::Error),
    ModuleError(String),
    VariableNotFound(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
            CodegenError::FunctionNotFound(name) => write!(f, "Function '{}' not found", name),
            CodegenError::InvalidString(msg) => write!(f, "Invalid string: {}", msg),
            CodegenError::IoError(err) => write!(f, "IO error: {}", err),
            CodegenError::ModuleError(msg) => write!(f, "Module error: {}", msg),
            CodegenError::VariableNotFound(name) => write!(f, "Variable '{}' not found", name),
        }
    }
}

impl std::error::Error for CodegenError {}

impl From<std::io::Error> for CodegenError {
    fn from(err: std::io::Error) -> Self {
        CodegenError::IoError(err)
    }
}

impl From<String> for CodegenError {
    fn from(err: String) -> Self {
        CodegenError::CompilationError(err)
    }
}

impl From<&str> for CodegenError {
    fn from(err: &str) -> Self {
        CodegenError::CompilationError(err.to_string())
    }
} 