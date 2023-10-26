use crate::lex_new::errors::LexError;
use crate::lex_new::errors::LexcialError;
use crate::tokens_new::*;

#[allow(dead_code)]
pub fn statement_to_token(
    statement: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match statement.as_str() {
        "let" => Ok(Token::Statement(Statement::Let)),
        "fn" => Ok(Token::Statement(Statement::Function)),
        "return" => Ok(Token::Statement(Statement::Return)),
        "inject" => Ok(Token::Statement(Statement::Inject)),
        "public" => Ok(Token::Statement(Statement::Public)),
        "if" => Ok(Token::Statement(Statement::If)),
        "else" => Ok(Token::Statement(Statement::Else)),
        "while" => Ok(Token::Statement(Statement::While)),
        "print" => Ok(Token::Statement(Statement::Print)),
        "println" => Ok(Token::Statement(Statement::Println)),
        "scanln" => Ok(Token::Statement(Statement::Scanln)),
        "for" => Ok(Token::Statement(Statement::For)),
        /*"void" => Ok(Token::TypeName(TypeName::Void)),
        "bool" => Ok(Token::TypeName(TypeName::Bool)),
        "string" => Ok(Token::TypeName(TypeName::QuotedString)),
        "i8" => Ok(Token::TypeName(TypeName::I8)),
        "i16" => Ok(Token::TypeName(TypeName::I16)),
        "i32" => Ok(Token::TypeName(TypeName::I32)),
        "i64" => Ok(Token::TypeName(TypeName::I64)),
        "u8" => Ok(Token::TypeName(TypeName::U8)),
        "u16" => Ok(Token::TypeName(TypeName::U16)),
        "u32" => Ok(Token::TypeName(TypeName::U32)),
        "u64" => Ok(Token::TypeName(TypeName::U64)),*/
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidStatement(statement.to_string()),
        }),
    }
}
#[allow(dead_code)]
pub fn type_name_to_token(
    typename: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match typename.as_str() {
        "Void" => Ok(Token::TypeName(TypeName::Void)),
        "Bool" => Ok(Token::TypeName(TypeName::Bool)),
        "String" => Ok(Token::TypeName(TypeName::QuotedString)),
        "i8" => Ok(Token::TypeName(TypeName::I8)),
        "i16" => Ok(Token::TypeName(TypeName::I16)),
        "i32" => Ok(Token::TypeName(TypeName::I32)),
        "i64" => Ok(Token::TypeName(TypeName::I64)),
        "u8" => Ok(Token::TypeName(TypeName::U8)),
        "u16" => Ok(Token::TypeName(TypeName::U16)),
        "u32" => Ok(Token::TypeName(TypeName::U32)),
        "u64" => Ok(Token::TypeName(TypeName::U64)),
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
