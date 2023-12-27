use crate::lex_new_new::errors::LexError;
use crate::lex_new_new::errors::LexcialError;
use crate::tokens_new::*;

#[allow(dead_code)]
pub fn symbol_to_token(symbol: char, line: usize, column: usize) -> Result<Token, LexcialError> {
    match symbol {
        ',' => Ok(Token::Symbol(Symbol::Comma)),
        ':' => Ok(Token::Symbol(Symbol::Colon)),
        '.' => Ok(Token::Symbol(Symbol::Dot)),
        '=' => Ok(Token::Assign(Assign::Assign)),
        //'-' => Token::Operator(Operator::Subtract),
        '(' => Ok(Token::Symbol(Symbol::OpenParen)),
        '{' => Ok(Token::Symbol(Symbol::OpenBrace)),
        '<' => Ok(Token::Logical(Logical::LessThan)),
        '[' => Ok(Token::Symbol(Symbol::OpenSquare)),
        ')' => Ok(Token::Symbol(Symbol::CloseParen)),
        '}' => Ok(Token::Symbol(Symbol::CloseBrace)),
        '>' => Ok(Token::Logical(Logical::GreaterThan)),
        ']' => Ok(Token::Symbol(Symbol::CloseSquare)),
        //'+' => Token::Operator(Operator::Add),
        //'%' => Token::Operator(Operator::Remainder),
        ';' => Ok(Token::Symbol(Symbol::Semicolon)),
        // '/' => Ok(Token::Operator(Operator::Divide)),
        //"," => Ok(Token::Symbol(Symbol::Comma)),
        //"!" => Ok(Token::Logical(Logical::Not)),
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
) -> Result<Token, LexcialError> {
    match double_symbol {
        "==" => Ok(Token::Logical(Logical::Equals)),
        "!=" => Ok(Token::Logical(Logical::NotEquals)),
        "+=" => Ok(Token::Assign(Assign::AddAssign)),
        "-=" => Ok(Token::Assign(Assign::SubAssign)),
        "*=" => Ok(Token::Assign(Assign::MulAssign)),
        "/=" => Ok(Token::Assign(Assign::DivAssign)),
        "%=" => Ok(Token::Assign(Assign::RemAssign)),
        "&=" => Ok(Token::Assign(Assign::BitAndAssign)),
        "|=" => Ok(Token::Assign(Assign::BitOrAssign)),
        "^=" => Ok(Token::Assign(Assign::BitXorAssign)),
        "->" => Ok(Token::Symbol(Symbol::Arrow)),
        "::" => Ok(Token::Symbol(Symbol::DoubleColon)),
        "&&" => Ok(Token::Logical(Logical::And)),
        "||" => Ok(Token::Logical(Logical::Or)),
        "<<" => Ok(Token::Operator(Operator::ShiftLeft)),
        ">>" => Ok(Token::Operator(Operator::ShiftRight)),
        "//" => Ok(Token::Symbol(Symbol::Comment)),
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
) -> Result<Token, LexcialError> {
    match operator {
        '+' => Ok(Token::Operator(Operator::Add)),
        '-' => Ok(Token::Operator(Operator::Subtract)),
        '*' => Ok(Token::Operator(Operator::Multiply)),
        '/' => Ok(Token::Operator(Operator::Divide)),
        '%' => Ok(Token::Operator(Operator::Remainder)),
        '&' => Ok(Token::Operator(Operator::BitAnd)),
        '|' => Ok(Token::Operator(Operator::BitOr)),
        '^' => Ok(Token::Operator(Operator::BitXor)),
        _ => Err(LexcialError {
            line,
            column,
            message: LexError::InvalidOperator(operator.to_string()),
        }),
    }
}
