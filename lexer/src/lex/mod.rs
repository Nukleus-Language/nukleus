use std::iter::Peekable;
use std::str::Chars;

mod statements;

use statements::*;
use crate::errors::LexerError;
use crate::tokens::*;

struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            column: 1,
        }
    }
    fn run(&mut self) -> Vec<Token> {
        let mut buffer = &str::new();

        
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
                self.column += 1,
                self.next_char();
            }
            Some('\t') => {
                self.column += 4,
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
    fn report_error(&self, error: LexerError) {
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
fn operator_to_token(operator: &str) -> Token{
    match operator {
        "+" => Token::Operator(Operator::Add),
        "-" => Token::Operator(Operator::Subtract),
        "*" => Token::Operator(Operator::Multiply),
        "/" => Token::Operator(Operator::Divide),
        "%" => Token::Operator(Operator::Remainder),
        _ => 
    }
}
fn symbol_to_token(symbol: &str) -> Token{
    match symbol {
        '@' => Token::Symbol(Symbol::At),
        ':' => Token::Symbol(Symbol::Colon),
        '.' => Token::Symbol(Symbol::Dot),
        '=' => Token::Assign(Assign::Assign),
        //'-' => Token::Operator(Operator::Subtract),
        '(' => Token::Symbol(Symbol::OpenParen),
        '{' => Token::Symbol(Symbol::OpenBrace),
        '<' => Token::Logical(Logical::LessThan),
        '[' => Token::Symbol(Symbol::OpenSquare),
        ')' => Token::Symbol(Symbol::CloseParen),
        '}' => Token::Symbol(Symbol::CloseBrace),
        '>' => Token::Logical(Logical::GreaterThan),
        ']' => Token::Symbol(Symbol::CloseSquare),
        //'+' => Token::Operator(Operator::Add),
        //'%' => Token::Operator(Operator::Remainder),
        ';' => Token::Symbol(Symbol::Semicolon),
        '/' => Token::Operator(Operator::Divide),
        ',' => Token::Symbol(Symbol::Comma),
        '!' => Token::Logical(Logical::Not),
    }
}
fn statement_to_token(statement_to_token) -> Token{

}

