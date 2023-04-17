use crate::lex_new::errors::LexError;
use crate::lex_new::errors::LexcialError;

use std::iter::Peekable;
use std::str::Chars;

use crate::tokens_new::*;

pub fn number_to_token(number: String, line: usize, column: usize) -> Result<Token, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    let trimed_number = number.clone();
    let test_parse = trimed_number.trim_matches('-').parse::<u64>();
    
    match test_parse {
        Ok(_) => {
            Ok(Token::TypeValue(TypeValue::Number(number.to_string())))
        }
        Err(_) => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidNumber(number),
            })
        }
    }
}
