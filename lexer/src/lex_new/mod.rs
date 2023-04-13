use std::iter::Peekable;
use std::str::Chars;
mod errors;
use errors::LexcialError;
use errors::LexError;

//use crate::errors::LexerError;
use crate::tokens_new::*;

#[derive(Debug, Clone, PartialEq)]
enum State {
    StateDefault,
    Number,
    Identifier,
    Operator,
    Symbol,
    QuoatedString,
    Comment,
    DoubleSymbol
}

struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    buffer: String,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            tokens: Vec::new(),
            buffer: String::new(),
            line: 1,
            column: 1,
        }
    }
    fn run(&mut self) {
        let mut state = State::StateDefault;
        while let Some(c) = self.next_char() {
            // Check if the buffer is empty and the current character when is empty
            if self.buffer.is_empty(){
                self.buffer.push(c);
                state = State::StateDefault;
                continue;
            }

            // if the first character is a - or number and the next character is a number
            // then it is a number
            let first_char = self.buffer.chars().nth(0).unwrap();
            if state == State::StateDefault && ((first_char == '-' || is_numeric(first_char)) && is_numeric(*self.peek_char().unwrap())) {
                state = State::Number;
                self.buffer.push(c);
                continue;
            }
            if state == State::Number && is_numeric(c) {
                self.buffer.push(c);
                continue;
            }
            else if state == State::Number && !is_numeric(c) {
                //let number = buffer.parse::<i32>().unwrap();
                let number = number_to_token(&self.buffer, self.line, self.column);
                match number {
                    Ok(number) => self.insert_token(number),
                    Err(error) => self.report_error(error),
                }
                self.buffer.clear();
                state = State::StateDefault;
                continue;
            }
        }
    }
    fn next_char(&mut self) -> Option<char> {
        let next = self.code.next();
        match next {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
                self.next_char();
            }
            Some(' ') => {
                self.column += 1;
                self.next_char();
            }
            Some('\t') => {
                self.column += 4;
                self.next_char();
            }
            Some(_) => self.column += 1,
            None => {}
        }
        next
    }
    fn peek_char(&mut self) -> Option<&char> {
        self.code.peek()
    }
    fn insert_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn report_error(&self, error: LexcialError) {
        println!("Lexial Error: {}", error);
        println!("Line: {}, Column: {}", self.line, self.column);
    }
}

fn is_numeric(c: char) -> bool {
    c.is_numeric()
}
fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}
fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' => true,
        _ => false,
    }
}
fn operator_to_token(operator: &str, line:usize, column:usize) -> Result<Token, LexcialError> {
    match operator {
        "+" => Ok(Token::Operator(Operator::Add)),
        "-" => Ok(Token::Operator(Operator::Subtract)),
        "*" => Ok(Token::Operator(Operator::Multiply)),
        "/" => Ok(Token::Operator(Operator::Divide)),
        "%" => Ok(Token::Operator(Operator::Remainder)),
        _ => return Err(LexcialError{line, column, message: LexError::InvalidOperator(operator.to_string())}),
    }
}
fn number_to_token(number: &str, line:usize, column:usize) -> Result<Token, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    let test_parse = number.parse::<u64>();

    match test_parse {
        Ok(number) => Ok(Token::TypeValue(TypeValue::Number(number.to_string()))),
        Err(_) => return Err(LexcialError{line, column, message: LexError::InvalidNumber(number.to_string())}),
    }
}
fn syambol_to_token(symbol: &str, line:usize, column:usize) -> Result<Token, LexcialError>{
    match symbol {
        "@" => Ok(Token::Symbol(Symbol::At)),
        ":" => Ok(Token::Symbol(Symbol::Colon)),
        "." => Ok(Token::Symbol(Symbol::Dot)),
        "=" => Ok(Token::Assign(Assign::Assign)),
        //'-' => Token::Operator(Operator::Subtract),
        "(" => Ok(Token::Symbol(Symbol::OpenParen)),
        "{" => Ok(Token::Symbol(Symbol::OpenBrace)),
        "<" => Ok(Token::Logical(Logical::LessThan)),
        "[" => Ok(Token::Symbol(Symbol::OpenSquare)),
        ")" => Ok(Token::Symbol(Symbol::CloseParen)),
        "}" => Ok(Token::Symbol(Symbol::CloseBrace)),
        ">" => Ok(Token::Logical(Logical::GreaterThan)),
        "]" => Ok(Token::Symbol(Symbol::CloseSquare)),
        //'+' => Token::Operator(Operator::Add),
        //'%' => Token::Operator(Operator::Remainder),
        ";" => Ok(Token::Symbol(Symbol::Semicolon)),
        "/" => Ok(Token::Operator(Operator::Divide)),
        "," => Ok(Token::Symbol(Symbol::Comma)),
        "!" => Ok(Token::Logical(Logical::Not)),
        _ => return Err(LexcialError{line, column, message: LexError::InvalidSymbol(symbol.to_string())}),
    }
}
/*fn statement_to_token(statement_to_token) -> Token{

}*/


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn line_counting(){
        let code = "fn main() {
            println!(\"Hello, world!\");
        }";
        let mut lexer = Lexer::new(code);
        lexer.run();
        assert_eq!(lexer.line, 3);
    }
}
