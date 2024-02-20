use crate::lex_new_new::errors::LexError;
use crate::lex_new_new::errors::LexcialError;
use crate::tokens_new::{Statement, TokenType, TypeName};

#[allow(dead_code)]
pub fn statement_to_token(
    statement: &str,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    match statement {
        "let" => Ok(TokenType::Statement(Statement::Let)),
        "fn" => Ok(TokenType::Statement(Statement::Function)),
        "return" => Ok(TokenType::Statement(Statement::Return)),
        "inject" => Ok(TokenType::Statement(Statement::Inject)),
        "public" => Ok(TokenType::Statement(Statement::Public)),
        "if" => Ok(TokenType::Statement(Statement::If)),
        "else" => Ok(TokenType::Statement(Statement::Else)),
        "while" => Ok(TokenType::Statement(Statement::While)),
        "print" => Ok(TokenType::Statement(Statement::Print)),
        "println" => Ok(TokenType::Statement(Statement::Println)),
        "scanln" => Ok(TokenType::Statement(Statement::Scanln)),
        "for" => Ok(TokenType::Statement(Statement::For)),
        /*"void" => Ok(TokenType::TypeName(TypeName::Void)),
        "bool" => Ok(TokenType::TypeName(TypeName::Bool)),
        "string" => Ok(TokenType::TypeName(TypeName::QuotedString)),
        "i8" => Ok(TokenType::TypeName(TypeName::I8)),
        "i16" => Ok(TokenType::TypeName(TypeName::I16)),
        "i32" => Ok(TokenType::TypeName(TypeName::I32)),
        "i64" => Ok(TokenType::TypeName(TypeName::I64)),
        "u8" => Ok(TokenType::TypeName(TypeName::U8)),
        "u16" => Ok(TokenType::TypeName(TypeName::U16)),
        "u32" => Ok(TokenType::TypeName(TypeName::U32)),
        "u64" => Ok(TokenType::TypeName(TypeName::U64)),*/
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidStatement(statement.to_string()),
        }),
    }
}
#[allow(dead_code)]
pub fn type_name_to_token(
    typename: &str,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    match typename {
        "Void" => Ok(TokenType::TypeName(TypeName::Void)),
        "Bool" => Ok(TokenType::TypeName(TypeName::Bool)),
        "String" => Ok(TokenType::TypeName(TypeName::QuotedString)),
        "i8" => Ok(TokenType::TypeName(TypeName::I8)),
        "i16" => Ok(TokenType::TypeName(TypeName::I16)),
        "i32" => Ok(TokenType::TypeName(TypeName::I32)),
        "i64" => Ok(TokenType::TypeName(TypeName::I64)),
        "u8" => Ok(TokenType::TypeName(TypeName::U8)),
        "u16" => Ok(TokenType::TypeName(TypeName::U16)),
        "u32" => Ok(TokenType::TypeName(TypeName::U32)),
        "u64" => Ok(TokenType::TypeName(TypeName::U64)),
        "Char" => Ok(TokenType::TypeName(TypeName::Char)),
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidTypeName(typename.to_string()),
        }),
    }
}

#[allow(dead_code)]
pub fn is_quote(c: char) -> bool {
    matches!(c, '"')
}

#[allow(dead_code)]
pub fn is_quoted_string(c: char) -> bool {
    matches!(c, '"')
}

#[allow(dead_code)]
pub fn is_identifierable(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[allow(dead_code)]
pub fn is_first_identifierable(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
