use astgen::ast::{AST, ASTOperator, ASTtypename, ASTtypevalue};

use crate::error::CodegenError;

#[derive(Debug, Clone)]
pub(super) struct FunctionSignature {
    pub(super) params: Vec<ASTtypename>,
    pub(super) return_type: ASTtypename,
}

#[derive(Debug, Clone)]
pub(super) struct LoweredValue {
    pub(super) repr: String,
    pub(super) ty: ASTtypename,
}

pub(super) fn lamina_type(ty: ASTtypename) -> Result<&'static str, CodegenError> {
    match ty {
        ASTtypename::TypeVoid => Ok("void"),
        ASTtypename::I8 => Ok("i8"),
        ASTtypename::I16 => Ok("i16"),
        ASTtypename::I32 => Ok("i32"),
        ASTtypename::I64 => Ok("i64"),
        ASTtypename::U8 => Ok("u8"),
        ASTtypename::U16 => Ok("u16"),
        ASTtypename::U32 => Ok("u32"),
        ASTtypename::U64 => Ok("u64"),
        ASTtypename::Bool => Ok("bool"),
        ASTtypename::Char => Ok("char"),
        ASTtypename::QuotedString | ASTtypename::Array => Ok("ptr"),
        _ => Err(CodegenError::CompilationError(format!(
            "Unsupported type for Lamina backend: {:?}",
            ty
        ))),
    }
}

pub(super) fn lamina_binary_op(op: ASTOperator) -> Result<&'static str, CodegenError> {
    match op {
        ASTOperator::Add | ASTOperator::AddAssign => Ok("add"),
        ASTOperator::Subtract | ASTOperator::SubAssign => Ok("sub"),
        ASTOperator::Multiply | ASTOperator::MulAssign => Ok("mul"),
        ASTOperator::Divide | ASTOperator::DivAssign => Ok("div"),
        ASTOperator::Remainder | ASTOperator::RemAssign => Ok("rem"),
        ASTOperator::BitAnd | ASTOperator::BitAndAssign | ASTOperator::And => Ok("and"),
        ASTOperator::BitOr | ASTOperator::BitOrAssign | ASTOperator::Or => Ok("or"),
        ASTOperator::BitXor | ASTOperator::BitXorAssign => Ok("xor"),
        ASTOperator::BitShiftLeft => Ok("shl"),
        ASTOperator::BitShiftRight => Ok("shr"),
        _ => Err(CodegenError::CompilationError(format!(
            "Unsupported binary operator for Lamina backend: {:?}",
            op
        ))),
    }
}

pub(super) fn lamina_cmp_op(op: ASTOperator) -> Option<&'static str> {
    match op {
        ASTOperator::Equals => Some("eq"),
        ASTOperator::NotEquals => Some("ne"),
        ASTOperator::Less => Some("lt"),
        ASTOperator::LessEquals => Some("le"),
        ASTOperator::Greater => Some("gt"),
        ASTOperator::GreaterEquals => Some("ge"),
        _ => None,
    }
}

pub(super) fn choose_binary_type(lhs: ASTtypename, rhs: ASTtypename) -> ASTtypename {
    if lhs == rhs {
        lhs
    } else if lhs == ASTtypename::TypeVoid {
        rhs
    } else {
        lhs
    }
}

pub(super) fn extract_identifier(value: &ASTtypevalue) -> Result<&str, CodegenError> {
    match value {
        ASTtypevalue::Identifier(name) => Ok(name),
        _ => Err(CodegenError::CompilationError(
            "Expected identifier argument in function signature".to_string(),
        )),
    }
}

pub(super) fn extract_identifier_from_ast(ast: &AST) -> Result<&str, CodegenError> {
    match ast {
        AST::TypeValue(ASTtypevalue::Identifier(name)) => Ok(name),
        _ => Err(CodegenError::CompilationError(
            "Assignment target must be an identifier".to_string(),
        )),
    }
}

pub(super) fn escape_char(c: char) -> String {
    match c {
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '\n' => "\\n".to_string(),
        _ => c.to_string(),
    }
}

pub(super) fn infer_type_from_typevalue(value: &ASTtypevalue) -> ASTtypename {
    match value {
        ASTtypevalue::I8(_) => ASTtypename::I8,
        ASTtypevalue::I16(_) => ASTtypename::I16,
        ASTtypevalue::I32(_) => ASTtypename::I32,
        ASTtypevalue::I64(_) => ASTtypename::I64,
        ASTtypevalue::U8(_) => ASTtypename::U8,
        ASTtypevalue::U16(_) => ASTtypename::U16,
        ASTtypevalue::U32(_) => ASTtypename::U32,
        ASTtypevalue::U64(_) => ASTtypename::U64,
        ASTtypevalue::Bool(_) => ASTtypename::Bool,
        ASTtypevalue::Char(_) => ASTtypename::Char,
        ASTtypevalue::QuotedString(_) => ASTtypename::QuotedString,
        ASTtypevalue::Array(_) => ASTtypename::Array,
        ASTtypevalue::Identifier(_) => ASTtypename::Identifier,
        ASTtypevalue::FunctionCall { .. } => ASTtypename::I64,
        ASTtypevalue::TypeVoid => ASTtypename::TypeVoid,
    }
}
