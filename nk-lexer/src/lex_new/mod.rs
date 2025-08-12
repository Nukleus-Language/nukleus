mod errors;

mod identifier;
mod symbol;
mod value;

use errors::LexcialError;

use std::iter::Peekable;
use std::str::Chars;

use inksac::{Color, Style, Stylish};

use crate::tokens_new::{Symbol, Token, TokenMetadata, TokenType, TypeValue};

const ERRORTXTSTYLE: Style = Style {
    foreground: Color::Red,
    background: Color::Empty,
    bold: true,
    dim: false,
    italic: true,
    underline: false,
};

/*const ERRORLINESTYLE: Style = Style {
    forground: Some(Color::Green),
    background: None,
    bold: false,
    dim:false,
    italic: false,
    underline: false
};*/

#[derive(Debug, Clone, PartialEq)]
enum State {
    EmptyState,
    DefaultState,
    Number,
    Identifier,
    QuotedString,
    DoubleState,
    Comment,
}

pub struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    state: State,
    buffer: String,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    #[allow(dead_code)]
    pub fn new(code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            tokens: Vec::new(),
            state: State::EmptyState,
            buffer: String::new(),
            line: 1,
            column: 0,
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self) {
        //let mut state = State::StateDefault;
        while let Some(c) = self.next_char() {
            let peeked_char = self.peek_char();
            // println!("---------------------------------");
            // println!("Current Char: {}", c);
            // println!("Current State: {:?}", self.state);
            // println!("Current Buffer: {}", self.buffer);
            if self.state == State::DoubleState {
                self.buffer.clear();
                self.state = State::EmptyState;

                continue;
            }
            if self.state == State::Comment {
                if c == '\n' {
                    self.state = State::EmptyState;
                    self.buffer.clear();
                    continue;
                }
                continue;
            }
            if c.is_whitespace() && self.state != State::QuotedString {
                self.buffer.clear();
                self.state = State::EmptyState;
                continue;
            }
            // Check if the buffer is empty and the current character when is empty
            if self.buffer.is_empty() {
                // check if is a double symbol
                self.buffer.push(c);
                self.buffer.push(peeked_char);
                // println!("cur {} peek {}", c, peeked_char);
                // println!("Buffer: {}", self.buffer.clone());
                let double_symbol =
                    symbol::double_symbol_to_token(self.buffer.clone(), self.line, self.column);
                match double_symbol {
                    Ok(double_symbol) => {
                        if double_symbol == TokenType::Symbol(Symbol::Comment) {
                            self.state = State::Comment;
                            self.buffer.clear();
                            continue;
                        }
                        self.insert_token(double_symbol);
                        self.buffer.clear();
                        self.state = State::DoubleState;
                        continue;
                    }
                    Err(_) => {
                        self.buffer.clear();
                    }
                }
                // run the symbol_to token to check if is a symbol
                let symbol = symbol::symbol_to_token(c, self.line, self.column);
                match symbol {
                    Ok(symbol) => {
                        self.insert_token(symbol);
                        continue;
                    }
                    Err(_) => {
                        self.buffer.clear();
                    }
                }
                // if !peeked_char.is_numeric() || c != '-' {
                let operator = symbol::operator_to_token(c, self.line, self.column);
                // println!("Operator: {}", operator.unwrap());
                match operator {
                    Ok(operator) => {
                        self.insert_token(operator);
                        continue;
                    }
                    Err(_) => {
                        self.buffer.clear();
                    }
                }
                // }
                self.buffer.push(c);

                self.state = State::DefaultState;
                //continue;
            }
            //println!(
            //    "Current First Char: {}",
            //    self.buffer.chars().next().unwrap()
            //);

            // if the first character is a - or number and the next character is a number
            // then it is a number
            let first_char = self.buffer.chars().next().unwrap();
            if self.state == State::DefaultState && (first_char == '-' || first_char.is_numeric()) {
                self.state = State::Number;
                //self.buffer.push(c);
            } else if self.state == State::Number && c.is_numeric() {
                self.buffer.push(c);
            }
            if self.state == State::Number && !peeked_char.is_numeric() {
                //let number = buffer.parse::<i32>().unwrap();
                let number = value::number_to_token(self.buffer.clone(), self.line, self.column);
                match number {
                    Ok(number) => self.insert_token(number),
                    Err(error) => self.report_error(error),
                }
                self.buffer.clear();
                self.state = State::EmptyState;
                continue;
            }

            // if the first character is a " then it is a string
            if self.state == State::DefaultState && identifier::is_quote(first_char) {
                self.state = State::QuotedString;
                //self.buffer.push(c);
                continue;
            } else if self.state == State::QuotedString && !identifier::is_quote(c) {
                self.buffer.push(c);
                continue;
            } else if self.state == State::QuotedString && identifier::is_quote(c) {
                let string = self.buffer.clone();
                self.buffer.push(c);
                // trim the quotes
                let string = string.trim_matches('"').to_string();
                self.insert_token(TokenType::TypeValue(TypeValue::QuotedString(string)));
                self.buffer.clear();
                self.state = State::EmptyState;
                continue;
            }

            // check if is a identifier, statement, or symbol
            if self.state == State::DefaultState && identifier::is_first_identifierable(first_char)
            {
                self.state = State::Identifier;
                //self.buffer.push(c);
            } else if self.state == State::Identifier && identifier::is_identifierable(c) {
                self.buffer.push(c);
            }
            if self.state == State::Identifier && !identifier::is_identifierable(peeked_char) {
                //let identifier = identifier_to_token(self.buffer.clone(), self.line, self.column);
                let statement =
                    identifier::statement_to_token(self.buffer.clone(), self.line, self.column);
                if let Ok(statement) = statement {
                    self.insert_token(statement);
                    self.buffer.clear();
                    self.state = State::EmptyState;
                    continue;
                }
                let type_name =
                    identifier::type_name_to_token(self.buffer.clone(), self.line, self.column);
                if let Ok(type_name) = type_name {
                    self.insert_token(type_name);
                    self.buffer.clear();
                    self.state = State::EmptyState;
                    continue;
                }
                let identifier = TokenType::TypeValue(TypeValue::Identifier(self.buffer.clone()));
                self.insert_token(identifier);
                self.buffer.clear();
                self.state = State::EmptyState;
                continue;
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.code.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 0;
                Some('\n')
            }
            Some('\t') => {
                self.column += 4;
                Some('\t')
            }
            Some(ch) => {
                self.column += 1; // Update column considering UTF-8 character length
                Some(ch)
            }
            None => None,
        }
    }

    fn peek_char(&mut self) -> char {
        let peek = self.code.peek();
        match peek {
            Some(_) => *peek.unwrap(),
            None => ' ',
        }
    }
    fn insert_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(
            token,
            TokenMetadata::new(self.line, self.column),
        ));
    }

    fn report_error(&self, error: LexcialError) {
        let errortxt = error.to_string().styled(ERRORTXTSTYLE);
        println!(
            "{} \n-------------> Line: {}, Column: {}",
            errortxt, self.line, self.column
        );
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens_new::{Assign, Operator, Statement, Symbol, TypeName, TypeValue};
    #[test]
    fn lexing_numbers() {
        let code = "fn main() -> Void \n{\nlet:i32 a = 5;\nlet:i32 b = 0;\n}";
        let _ans = vec![
            TokenType::Statement(Statement::Function),
            TokenType::TypeValue(TypeValue::Identifier("main".to_string())),
            TokenType::Symbol(Symbol::OpenParen),
            TokenType::Symbol(Symbol::CloseParen),
            TokenType::Symbol(Symbol::Arrow),
            TokenType::TypeName(TypeName::Void),
            TokenType::Symbol(Symbol::OpenBrace),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(5.to_string())),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("b".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(0.to_string())),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_strings() {
        let code = " \"Hello, world!\" ";
        let _ans = [TokenType::TypeValue(TypeValue::QuotedString(
            "Hello, world!".to_string(),
        ))];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_comments() {
        let code = "public fn main() -> Void \n{\n//println(\"Hello, world!\");\nreturn;\n}";
        let _ans = vec![
            TokenType::Statement(Statement::Public),
            TokenType::Statement(Statement::Function),
            TokenType::TypeValue(TypeValue::Identifier("main".to_string())),
            TokenType::Symbol(Symbol::OpenParen),
            TokenType::Symbol(Symbol::CloseParen),
            TokenType::Symbol(Symbol::Arrow),
            TokenType::TypeName(TypeName::Void),
            TokenType::Symbol(Symbol::OpenBrace),
            TokenType::Statement(Statement::Return),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_string_assign() {
        let code = "let:String a = \"Hello, world!\";";
        let _ans = vec![
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::QuotedString),
            TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::QuotedString("Hello, world!".to_string())),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_underbar_started_var() {
        let code = "let:i32 _a = 5;";
        let _ans = vec![
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("_a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(5.to_string())),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    /*#[test]
    fn lexing_negative_number_assign() {
        let code = "let:i32 a = -5;";
        let ans = vec![
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number("-5".to_string())),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }*/
    #[test]
    fn lexing_nested_expression() {
        let code = "let:i32 a = ((5 + a) /2)+2;";
        let ans = vec![
            Token::new(
                TokenType::Statement(Statement::Let),
                TokenMetadata::new(1, 3),
            ),
            Token::new(TokenType::Symbol(Symbol::Colon), TokenMetadata::new(1, 4)),
            Token::new(TokenType::TypeName(TypeName::I32), TokenMetadata::new(1, 7)),
            Token::new(
                TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
                TokenMetadata::new(1, 9),
            ),
            Token::new(TokenType::Assign(Assign::Assign), TokenMetadata::new(1, 11)),
            Token::new(
                TokenType::Symbol(Symbol::OpenParen),
                TokenMetadata::new(1, 13),
            ),
            Token::new(
                TokenType::Symbol(Symbol::OpenParen),
                TokenMetadata::new(1, 14),
            ),
            Token::new(
                TokenType::TypeValue(TypeValue::Number(5.to_string())),
                TokenMetadata::new(1, 15),
            ),
            Token::new(
                TokenType::Operator(Operator::Add),
                TokenMetadata::new(1, 17),
            ),
            Token::new(
                TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
                TokenMetadata::new(1, 19),
            ),
            Token::new(
                TokenType::Symbol(Symbol::CloseParen),
                TokenMetadata::new(1, 20),
            ),
            Token::new(
                TokenType::Operator(Operator::Divide),
                TokenMetadata::new(1, 22),
            ),
            Token::new(
                TokenType::TypeValue(TypeValue::Number(2.to_string())),
                TokenMetadata::new(1, 23),
            ),
            Token::new(
                TokenType::Symbol(Symbol::CloseParen),
                TokenMetadata::new(1, 24),
            ),
            Token::new(
                TokenType::Operator(Operator::Add),
                TokenMetadata::new(1, 25),
            ),
            Token::new(
                TokenType::TypeValue(TypeValue::Number(2.to_string())),
                TokenMetadata::new(1, 26),
            ),
            Token::new(
                TokenType::Symbol(Symbol::Semicolon),
                TokenMetadata::new(1, 27),
            ),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_complex() {
        let code = "fn main() -> Void \n{\nlet:i32 a = 5;\nlet:i32 b = 0;\nprintln(\"Hello, world!\");\nreturn;\n}";
        let _ans = vec![
            TokenType::Statement(Statement::Function),
            TokenType::TypeValue(TypeValue::Identifier("main".to_string())),
            TokenType::Symbol(Symbol::OpenParen),
            TokenType::Symbol(Symbol::CloseParen),
            TokenType::Symbol(Symbol::Arrow),
            TokenType::TypeName(TypeName::Void),
            TokenType::Symbol(Symbol::OpenBrace),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(5.to_string())),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("b".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(0.to_string())),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Println),
            TokenType::Symbol(Symbol::OpenParen),
            TokenType::TypeValue(TypeValue::QuotedString("Hello, world!".to_string())),
            TokenType::Symbol(Symbol::CloseParen),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Return),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
}
