use crate::lex_new_new::errors::LexError;
use crate::lex_new_new::errors::LexcialError;
use crate::tokens_new::*;

#[allow(dead_code)]
pub fn number_to_token(number: &str, line: usize, column: usize) -> Result<Token, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    let trimed_number = number;
    let test_parse = trimed_number.trim_matches('-').parse::<u64>();

    match test_parse {
        Ok(_) => Ok(Token::TypeValue(TypeValue::Number(number.to_string()))),
        Err(_) => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidNumber(number.to_string()),
        }),
    }
}
