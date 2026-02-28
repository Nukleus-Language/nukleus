#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarType {
    I8,
    I16,
    I32,
    I64,
    String,
    Void,
    Array,
}

impl VarType {
    pub fn size(&self) -> usize {
        match self {
            VarType::I8 => 1,
            VarType::I16 => 2,
            VarType::I32 => 4,
            VarType::I64 => 8,
            VarType::String => 8, // pointer size
            VarType::Array => 8,  // pointer size
            VarType::Void => 0,
        }
    }

    pub fn from_ast_type(ast_type: astgen::ast::ASTtypename) -> Self {
        match ast_type {
            astgen::ast::ASTtypename::I8 => VarType::I8,
            astgen::ast::ASTtypename::I16 => VarType::I16,
            astgen::ast::ASTtypename::I32 => VarType::I32,
            astgen::ast::ASTtypename::I64 => VarType::I64,
            astgen::ast::ASTtypename::QuotedString => VarType::String,
            astgen::ast::ASTtypename::Array => VarType::Array,
            astgen::ast::ASTtypename::TypeVoid => VarType::Void,
            _ => VarType::Void,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterSize {
    Byte,    // 8-bit
    Word,    // 16-bit
    DWord,   // 32-bit
    QWord,   // 64-bit
}

use std::fmt;

#[derive(Debug)]
pub enum CodegenError {
    UnsupportedOperation(String),
    ArchitectureError(String),
    InvalidJumpTarget(String),
    RegisterAllocationError(String),
    InvalidFunctionArguments(String),
    NoAvailableRegisters,
    UnsupportedArchitecture,
    InvalidInstruction,
    MemoryAllocation,
    MemoryProtection,
    CompilationError(String),
    UndefinedVariable(String),
    UnsupportedValueType(String),
    UnsupportedOperator(String),
    FunctionError(String),
    FunctionNotFound(String),
    UnsupportedStatement(String),
    UnsupportedExpression(String),
    InvalidReturnType(String),
    StackOverflow,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOperation(e) => write!(f, "Unsupported operation: {}", e),
            Self::ArchitectureError(e) => write!(f, "Architecture error: {}", e),
            Self::InvalidJumpTarget(label) => write!(f, "Invalid jump target: {}", label),
            Self::RegisterAllocationError(e) => write!(f, "Register allocation error: {}", e),
            Self::InvalidFunctionArguments(e) => write!(f, "Invalid function arguments: {}", e),
            Self::NoAvailableRegisters => write!(f, "No available registers"),
            Self::UnsupportedArchitecture => write!(f, "Unsupported architecture"),
            Self::InvalidInstruction => write!(f, "Invalid instruction sequence"),
            Self::MemoryAllocation => write!(f, "Failed to allocate executable memory"),
            Self::MemoryProtection => write!(f, "Failed to set memory protection"),
            Self::CompilationError(e) => write!(f, "Compilation error: {}", e),
            Self::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            Self::UnsupportedValueType(ty) => write!(f, "Unsupported value type: {}", ty),
            Self::UnsupportedOperator(op) => write!(f, "Unsupported operator: {}", op),
            Self::FunctionError(e) => write!(f, "Function error: {}", e),
            Self::FunctionNotFound(name) => write!(f, "Function not found: {}", name),
            Self::UnsupportedStatement(stmt) => write!(f, "Unsupported statement: {}", stmt),
            Self::UnsupportedExpression(expr) => write!(f, "Unsupported expression: {}", expr),
            Self::InvalidReturnType(ty) => write!(f, "Invalid return type: {}", ty),
            Self::StackOverflow => write!(f, "Stack overflow"),
        }
    }
}

impl std::error::Error for CodegenError {} 