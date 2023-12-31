use crate::lex_new_new::errors::LexError;
use crate::lex_new_new::errors::LexcialError;
use crate::tokens_new::{Assign, Logical, Operator, Symbol, TokenType};

#[allow(dead_code)]
pub fn symbol_to_token(
    symbol: char,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    match symbol {
        ',' => Ok(TokenType::Symbol(Symbol::Comma)),
        ':' => Ok(TokenType::Symbol(Symbol::Colon)),
        '.' => Ok(TokenType::Symbol(Symbol::Dot)),
        '=' => Ok(TokenType::Assign(Assign::Assign)),
        //'-' => TokenType::Operator(Operator::Subtract),
        '(' => Ok(TokenType::Symbol(Symbol::OpenParen)),
        '{' => Ok(TokenType::Symbol(Symbol::OpenBrace)),
        '<' => Ok(TokenType::Logical(Logical::LessThan)),
        '[' => Ok(TokenType::Symbol(Symbol::OpenSquare)),
        ')' => Ok(TokenType::Symbol(Symbol::CloseParen)),
        '}' => Ok(TokenType::Symbol(Symbol::CloseBrace)),
        '>' => Ok(TokenType::Logical(Logical::GreaterThan)),
        ']' => Ok(TokenType::Symbol(Symbol::CloseSquare)),
        //'+' => TokenType::Operator(Operator::Add),
        //'%' => TokenType::Operator(Operator::Remainder),
        ';' => Ok(TokenType::Symbol(Symbol::Semicolon)),
        // '/' => Ok(TokenType::Operator(Operator::Divide)),
        //"," => Ok(TokenType::Symbol(Symbol::Comma)),
        //"!" => Ok(TokenType::Logical(Logical::Not)),
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidSymbol(symbol.to_string()),
        }),
    }
}
#[allow(dead_code)]
pub fn double_symbol_to_token(
    double_symbol: &str,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    match double_symbol {
        "==" => Ok(TokenType::Logical(Logical::Equals)),
        "!=" => Ok(TokenType::Logical(Logical::NotEquals)),
        "+=" => Ok(TokenType::Assign(Assign::AddAssign)),
        "-=" => Ok(TokenType::Assign(Assign::SubAssign)),
        "*=" => Ok(TokenType::Assign(Assign::MulAssign)),
        "/=" => Ok(TokenType::Assign(Assign::DivAssign)),
        "%=" => Ok(TokenType::Assign(Assign::RemAssign)),
        "&=" => Ok(TokenType::Assign(Assign::BitAndAssign)),
        "|=" => Ok(TokenType::Assign(Assign::BitOrAssign)),
        "^=" => Ok(TokenType::Assign(Assign::BitXorAssign)),
        "->" => Ok(TokenType::Symbol(Symbol::Arrow)),
        "::" => Ok(TokenType::Symbol(Symbol::DoubleColon)),
        "&&" => Ok(TokenType::Logical(Logical::And)),
        "||" => Ok(TokenType::Logical(Logical::Or)),
        "<<" => Ok(TokenType::Operator(Operator::ShiftLeft)),
        ">>" => Ok(TokenType::Operator(Operator::ShiftRight)),
        "//" => Ok(TokenType::Symbol(Symbol::Comment)),
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidDoubleSymbol(double_symbol.to_string()),
        }),
    }
}
#[allow(dead_code)]
pub fn operator_to_token(
    operator: char,
    line: usize,
    column: usize,
) -> Result<TokenType, LexcialError> {
    match operator {
        '+' => Ok(TokenType::Operator(Operator::Add)),
        '-' => Ok(TokenType::Operator(Operator::Subtract)),
        '*' => Ok(TokenType::Operator(Operator::Multiply)),
        '/' => Ok(TokenType::Operator(Operator::Divide)),
        '%' => Ok(TokenType::Operator(Operator::Remainder)),
        '&' => Ok(TokenType::Operator(Operator::BitAnd)),
        '|' => Ok(TokenType::Operator(Operator::BitOr)),
        '^' => Ok(TokenType::Operator(Operator::BitXor)),
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidOperator(operator.to_string()),
        }),
    }
}
