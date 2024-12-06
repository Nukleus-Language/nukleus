use std::borrow::Cow;

use crate::trie_lex::errors::LexError;
use crate::trie_lex::errors::LexcialError;
// use crate::tokens_new::{TokenType, TypeValue};
use crate::neo_tokens::{TokenType, TypeValue};

#[allow(dead_code)]
pub fn number_to_token(
    number: &str,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    let trimed_number = number;
    let test_parse = trimed_number.trim_matches('-').parse::<u64>();

    match test_parse {
        Ok(_) => Ok(TokenType::TypeValue(TypeValue::Number(Cow::Owned(
            number.to_owned(),
        )))),
        Err(_) => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidNumber(number.to_string()),
        }),
    }
}
