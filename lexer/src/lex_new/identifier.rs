use crate::lex_new::errors::LexError;
use crate::lex_new::errors::LexcialError;

use std::iter::Peekable;
use std::str::Chars;

use crate::tokens_new::*;

pub fn statement_to_token(
    statement: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match statement.as_str() {
        "let" => Ok(Token::Statement(Statement::Let)),
        "fn" => Ok(Token::Statement(Statement::Function)),
        "return" => Ok(Token::Statement(Statement::Return)),
        "import" => Ok(Token::Statement(Statement::Import)),
        "public" => Ok(Token::Statement(Statement::Public)),
        "if" => Ok(Token::Statement(Statement::If)),
        "else" => Ok(Token::Statement(Statement::Else)),
        "while" => Ok(Token::Statement(Statement::While)),
        "print" => Ok(Token::Statement(Statement::Print)),
        "println" => Ok(Token::Statement(Statement::Println)),
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
pub fn type_name_to_token(
    typename: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match typename.as_str() {
        "void" => Ok(Token::TypeName(TypeName::Void)),
        "bool" => Ok(Token::TypeName(TypeName::Bool)),
        "string" => Ok(Token::TypeName(TypeName::QuotedString)),
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
