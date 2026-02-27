use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodegenErrorCode {
    CompilationError,
    FunctionNotFound,
    InvalidAssignOperator,
    InvalidArgumentType,
    InvalidForStartValue,
    InvalidString,
    IoError,
    ModuleError,
    VariableNotFound,
}

impl CodegenErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompilationError => "NK-CG-001",
            Self::FunctionNotFound => "NK-CG-002",
            Self::InvalidAssignOperator => "NK-CG-003",
            Self::InvalidArgumentType => "NK-CG-004",
            Self::InvalidForStartValue => "NK-CG-005",
            Self::InvalidString => "NK-CG-006",
            Self::IoError => "NK-CG-007",
            Self::ModuleError => "NK-CG-008",
            Self::VariableNotFound => "NK-CG-009",
        }
    }
}

impl fmt::Display for CodegenErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub enum CodegenError {
    CompilationError(String),
    FunctionNotFound(String),
    InvalidAssignOperator(String),
    InvalidArgumentType(String),
    InvalidForStartValue(String),
    InvalidString(String),
    IoError(std::io::Error),
    ModuleError(String),
    VariableNotFound(String),
}

impl CodegenError {
    pub fn code(&self) -> CodegenErrorCode {
        match self {
            CodegenError::CompilationError(_) => CodegenErrorCode::CompilationError,
            CodegenError::FunctionNotFound(_) => CodegenErrorCode::FunctionNotFound,
            CodegenError::InvalidAssignOperator(_) => CodegenErrorCode::InvalidAssignOperator,
            CodegenError::InvalidArgumentType(_) => CodegenErrorCode::InvalidArgumentType,
            CodegenError::InvalidForStartValue(_) => CodegenErrorCode::InvalidForStartValue,
            CodegenError::InvalidString(_) => CodegenErrorCode::InvalidString,
            CodegenError::IoError(_) => CodegenErrorCode::IoError,
            CodegenError::ModuleError(_) => CodegenErrorCode::ModuleError,
            CodegenError::VariableNotFound(_) => CodegenErrorCode::VariableNotFound,
        }
    }

    pub fn to_diagnostic(&self) -> nk_diagnostics::Diagnostic {
        nk_diagnostics::Diagnostic::error(self.code().as_str(), self.to_string(), None)
    }
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
            CodegenError::FunctionNotFound(name) => write!(f, "Function '{}' not found", name),
            CodegenError::InvalidAssignOperator(msg) => {
                write!(f, "Invalid assign operator: {}", msg)
            }
            CodegenError::InvalidArgumentType(msg) => write!(f, "Invalid argument type: {}", msg),
            CodegenError::InvalidForStartValue(msg) => {
                write!(f, "Invalid for start value: {}", msg)
            }
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
