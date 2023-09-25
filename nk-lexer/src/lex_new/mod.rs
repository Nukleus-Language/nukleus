mod errors;
mod identifier;
mod symbol;
mod value;

use errors::LexcialError;

use std::iter::Peekable;
use std::str::Chars;

use inksac::types::*;

use crate::tokens_new::*;




const ERRORTXTSTYLE: Style = Style {
    forground: Some(Color::Red),
    background: None,
    bold: true,
    dim:false,
    italic: true,
    underline: false
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
            column: 1,
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self) {
        //let mut state = State::StateDefault;
        while let Some(c) = self.next_char() {
            let peeked_char = self.peek_char();
            //println!("---------------------------------");
            //println!("Current Char: {}", c);
            //println!("Current State: {:?}", self.state);
            //println!("Current Buffer: {}", self.buffer);
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
                let double_symbol =
                    symbol::double_symbol_to_token(self.buffer.clone(), self.line, self.column);
                match double_symbol {
                    Ok(double_symbol) => {
                        if double_symbol == Token::Symbol(Symbol::Comment) {
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
                    Err(_) => {}
                }
                if !peeked_char.is_numeric() {
                    let operator = symbol::operator_to_token(c, self.line, self.column);
                    match operator {
                        Ok(operator) => {
                            self.insert_token(operator);
                            continue;
                        }
                        Err(_) => {}
                    }
                }
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
                self.insert_token(Token::TypeValue(TypeValue::QuotedString(string)));
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
                match statement {
                    Ok(statement) => {
                        self.insert_token(statement);
                        self.buffer.clear();
                        self.state = State::EmptyState;
                        continue;
                    }
                    Err(_) => {}
                }
                let type_name =
                    identifier::type_name_to_token(self.buffer.clone(), self.line, self.column);
                match type_name {
                    Ok(type_name) => {
                        self.insert_token(type_name);
                        self.buffer.clear();
                        self.state = State::EmptyState;
                        continue;
                    }
                    Err(_) => {}
                }
                let identifier = Token::TypeValue(TypeValue::Identifier(self.buffer.clone()));
                self.insert_token(identifier);
                self.buffer.clear();
                self.state = State::EmptyState;
                continue;
            }
        }
    }
    #[allow(dead_code)]
    fn next_char(&mut self) -> Option<char> {
        let next = self.code.next();
        match next {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
                //println!("Line Change: {}", self.line);
                //println!("Column Change: {}", self.column);
            }
            Some(' ') => {
                self.column += 1;
                //println!("Column Change: {}", self.column);
            }
            Some('\t') => {
                self.column += 4;
                //println!("Column Change: {}", self.column);
            }
            Some(_) => {
                self.column += 1;
                //println!("Column Change: {}", self.column);
            }
            None => {}
        }
        next
    }
    #[allow(dead_code)]
    fn peek_char(&mut self) -> char {
        let peek = self.code.peek();
        match peek {
            Some(_) => *peek.unwrap(),
            None => ' ',
        }
    }
    #[allow(dead_code)]
    fn insert_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    #[allow(dead_code)]
    fn report_error(&self, error: LexcialError) {
        let errortxt = ColoredString::new(error.to_string().as_str(), ERRORTXTSTYLE);

        println!(
            "{} \n-------------> Line: {}, Column: {}",
            errortxt, self.line, self.column
        );
    }
    #[allow(dead_code)]
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn line_counting() {
        let code = "fn main() -> void \n{\nprintln(\"Hello, world!\");\n}";
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.line, 4);
    }
    #[test]
    fn column_counting() {
        let code = "fn main() -> void\n{\nprintln(\"Hello, world!\");\n}";
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.column, 2);
    }
    #[test]
    fn lexing_numbers() {
        let code = "fn main() -> void \n{\nlet:i32 a = 5;\nlet:i32 b = 0;\n}";
        let ans = vec![
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_string())),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Arrow),
            Token::TypeName(TypeName::Void),
            Token::Symbol(Symbol::OpenBrace),
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("a".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number(5.to_string())),
            Token::Symbol(Symbol::Semicolon),
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("b".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number(0.to_string())),
            Token::Symbol(Symbol::Semicolon),
            Token::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_strings() {
        let code = " \"Hello, world!\" ";
        let ans = vec![Token::TypeValue(TypeValue::QuotedString(
            "Hello, world!".to_string(),
        ))];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_comments() {
        let code = "public fn main() -> void \n{\n//println(\"Hello, world!\");\nreturn;\n}";
        let ans = vec![
            Token::Statement(Statement::Public),
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_string())),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Arrow),
            Token::TypeName(TypeName::Void),
            Token::Symbol(Symbol::OpenBrace),
            Token::Statement(Statement::Return),
            Token::Symbol(Symbol::Semicolon),
            Token::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_string_assign() {
        let code = "let:string a = \"Hello, world!\";";
        let ans = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::QuotedString),
            Token::TypeValue(TypeValue::Identifier("a".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::QuotedString("Hello, world!".to_string())),
            Token::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_underbar_started_var() {
        let code = "let:i32 _a = 5;";
        let ans = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("_a".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number(5.to_string())),
            Token::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_negative_number_assign() {
        let code = "let:i32 a = -5;";
        let ans = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("a".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number("-5".to_string())),
            Token::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_complex() {
        let code = "fn main() -> void \n{\nlet:i32 a = 5;\nlet:i32 b = 0;\nprintln(\"Hello, world!\");\nreturn;\n}";
        let ans = vec![
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_string())),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Arrow),
            Token::TypeName(TypeName::Void),
            Token::Symbol(Symbol::OpenBrace),
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("a".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number(5.to_string())),
            Token::Symbol(Symbol::Semicolon),
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("b".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::Number(0.to_string())),
            Token::Symbol(Symbol::Semicolon),
            Token::Statement(Statement::Println),
            Token::Symbol(Symbol::OpenParen),
            Token::TypeValue(TypeValue::QuotedString("Hello, world!".to_string())),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Semicolon),
            Token::Statement(Statement::Return),
            Token::Symbol(Symbol::Semicolon),
            Token::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(code);
        lexer.run();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens, ans);
    }
}
