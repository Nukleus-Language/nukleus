use std::borrow::Cow;

use super::errors::{LexError, LexicalError};
// use crate::tokens_new::{TokenType, TypeValue};
use crate::tokens::{TokenType, TypeValue};

#[allow(dead_code)]
pub fn number_to_token(
    number: &str,
    line: usize,
    column: usize,
) -> Result<TokenType, LexicalError> {
    //check if the number is parseable while not changing the type of number to i32
    let trimed_number = number;
    let test_parse = trimed_number.trim_matches('-').parse::<u64>();

    match test_parse {
        Ok(_) => Ok(TokenType::TypeValue(TypeValue::Number(Cow::Owned(
            number.to_owned(),
        )))),
        Err(_) => Err(LexicalError {
            line,
            column,
            message: LexError::InvalidNumber(number.to_string()),
            note: None,
        }),
    }
}
