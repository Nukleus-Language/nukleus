use crate::lex_new::errors::LexError;
use crate::lex_new::errors::LexcialError;
use crate::tokens_new::*;

#[allow(dead_code)]
pub fn number_to_token(
    number: String,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    let trimed_number = number.clone();
    let test_parse = trimed_number.trim_matches('-').parse::<u64>();

    match test_parse {
        Ok(_) => Ok(TokenType::TypeValue(TypeValue::Number(number))),
        Err(_) => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidNumber(number),
        }),
    }
}
