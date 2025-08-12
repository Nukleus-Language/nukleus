mod errors;

mod identifier;
mod symbol;
mod value;

use errors::{LexError, LexcialError};

use std::borrow::Cow;
use std::iter::Peekable;
use std::path::PathBuf;
use std::str::Chars;

// use crate::tokens_new::{
//     Assign, Operator, Statement, Symbol, Token, TokenMetadata, TokenType, TypeName, TypeValue,
// };
use crate::neo_tokens::{Symbol, Token, TokenMetadata, TokenType, TypeValue};

use inksac::{Color, Style};

const ERRORTXTSTYLE: Style = Style {
    foreground: Color::Red,
    background: Color::Empty,
    bold: true,
    dim: false,
    italic: true,
    underline: false,
};

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
    buffer_st: usize,
    buffer_ed: usize,
    line: usize,
    column: usize,
    file_path: PathBuf,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    #[allow(dead_code)]
    pub fn new(file_path: PathBuf, code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            tokens: Vec::new(),
            state: State::EmptyState,
            buffer_st: 0,
            buffer_ed: 0,
            line: 1,
            column: 0,
            file_path,
            source: code,
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self) -> Result<(), LexcialError> {
        while let Some(c) = self.next_char() {
            let peeked_char = self.peek_char().unwrap_or('\0');

            // println!("---------------------------------");
            // println!("Current Char: {}", c);
            // println!("Current State: {:?}", self.state);
            // println!("Current Buffer: {}", self.source[self.buffer_st..self.buffer_ed].to_string());
            // println!("Current Buffer start: {}", self.buffer_st);
            // println!("Current Buffer end: {}", self.buffer_ed);
            // self.handle_double_state();
            if self.state == State::DoubleState {
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            // Handling Comment State
            if self.state == State::Comment {
                if c == '\n' {
                    self.state = State::EmptyState;
                    self.buffer_st = self.buffer_ed;
                }
                continue;
            }

            // Handling Whitespace
            if c.is_whitespace() && self.state != State::QuotedString {
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            // Check if the buffer is empty and the current character when is empty
            if self.buffer_ed == self.buffer_st + c.len_utf8() {
                // check if is a double symbol
                if peeked_char != '\0' {
                    let peeked_index = self.buffer_ed + peeked_char.len_utf8();
                    let double_symbol_str = &self.source[self.buffer_st..peeked_index];
                    let double_symbol =
                        symbol::double_symbol_to_token(double_symbol_str, self.line, self.column);
                    if let Ok(double_symbol) = double_symbol {
                        if double_symbol == TokenType::Symbol(Symbol::Comment) {
                            self.state = State::Comment;
                            continue;
                        }
                        self.insert_token(double_symbol);
                        self.state = State::DoubleState;
                        continue;
                    }
                }

                // Check for single symbols
                let symbol = symbol::symbol_to_token(c, self.line, self.column);
                if let Ok(symbol) = symbol {
                    self.insert_token(symbol);
                    self.buffer_st = self.buffer_ed;
                    continue;
                }

                // Handling operators
                let operator = symbol::operator_to_token(c, self.line, self.column);
                if let Ok(operator) = operator {
                    self.insert_token(operator);
                    self.buffer_st = self.buffer_ed;
                    continue;
                }

                self.state = State::DefaultState;
            }

            // Handling numbers
            let first_char: char = self.source[self.buffer_st..self.buffer_ed]
                .chars()
                .next()
                .unwrap();
            if self.state == State::DefaultState && (first_char == '-' || first_char.is_numeric()) {
                self.state = State::Number;
            }
            if self.state == State::Number && !peeked_char.is_numeric() {
                let number = value::number_to_token(
                    &self.source[self.buffer_st..self.buffer_ed],
                    self.line,
                    self.column,
                );
                match number {
                    Ok(number) => {
                        self.insert_token(number);
                        self.buffer_st = self.buffer_ed;
                    }
                    Err(error) => return self.report_error(error),
                }

                self.state = State::EmptyState;
                continue;
            }

            // Handling quoted strings
            if self.state == State::DefaultState && identifier::is_quote(first_char) {
                self.state = State::QuotedString;
                continue;
            } else if self.state == State::QuotedString && !identifier::is_quote(c) {
                continue;
            } else if self.state == State::QuotedString && identifier::is_quote(c) {
                let mut string = &self.source[self.buffer_st..self.buffer_ed];
                string = string.trim_matches('"');
                // self.insert_token(TokenType::TypeValue(TypeValue::QuotedString(
                // string.to_string(),
                // )));
                self.insert_token(TokenType::TypeValue(TypeValue::QuotedString(Cow::Owned(
                    string.to_owned(),
                ))));
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            // check if is a identifier, statement, or symbol
            if self.state == State::DefaultState && identifier::is_first_identifierable(first_char)
            {
                self.state = State::Identifier;
            }
            if self.state == State::Identifier && !identifier::is_identifierable(peeked_char) {
                let string = &self.source[self.buffer_st..self.buffer_ed];
                let statement = identifier::statement_to_token(string, self.line, self.column);
                if let Ok(statement) = statement {
                    self.insert_token(statement);
                    self.buffer_st = self.buffer_ed;
                    self.state = State::EmptyState;
                    continue;
                }
                let type_name = identifier::type_name_to_token(string, self.line, self.column);
                if let Ok(type_name) = type_name {
                    self.insert_token(type_name);
                    self.reset_state();
                    continue;
                }
                let identifier = TokenType::TypeValue(TypeValue::Identifier(string.to_string()));
                self.insert_token(identifier);
                self.reset_state();
                continue;
            }
        }
        if self.state == State::QuotedString {
            return self.report_error(LexcialError {
                line: self.line,
                column: self.column,
                message: LexError::ExpectedQuote(),
            });
        }
        Ok(())
    }

    #[inline]
    fn handle_double_state(&mut self) {
        self.buffer_st = self.buffer_ed;
        self.state = State::EmptyState;
    }
    fn next_char(&mut self) -> Option<char> {
        self.code.next().inspect(|&ch| {
            self.update_position(ch);
        })
    }
    #[inline]
    fn update_position(&mut self, ch: char) {
        self.column += match ch {
            '\n' => {
                self.line += 1;
                0
            } // Reset column to 0 for new line
            '\t' => 4, // Assume tab is 4 spaces
            _ => 1,
        };
        self.buffer_ed += ch.len_utf8();
    }
    fn peek_char(&mut self) -> Result<char, ()> {
        self.code.peek().copied().ok_or(())
    }
    fn insert_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(
            token,
            TokenMetadata::new(self.line, self.column),
        ));
    }

    fn report_error(&self, error: LexcialError) -> Result<(), LexcialError> {
        let context_window = 10; // Number of characters to show around the error

        let start = self.buffer_st.saturating_sub(context_window);
        let end = std::cmp::min(self.buffer_ed + context_window, self.source.len());

        let context_snippet = &self.source[start..end];

        // Count the number of characters (not bytes) from the start of the snippet to the error position
        let error_pos_in_context = self.source[start..self.column].chars().count();
        let error_location_marker = " ".repeat(error_pos_in_context.saturating_sub(1)) + "^";

        // Context and Error Information
        let errortxt = format!(
            "Context:\n{}\n{}\n--> Error at {} Position Line: {}, Column: {}: {}",
            context_snippet,
            error_location_marker,
            self.line,
            self.column,
            self.file_path.display(),
            error // Assuming .to_string() returns the formatted error message
        ); // Suggestion for resolution (customize based on your error types)
        let suggestion = match error.message {
            LexError::InvalidCharacter(ref ch) => {
                format!(
                    "Suggestion: Unexpected character '{}'. Try removing or replacing it.",
                    ch
                )
            }
            LexError::InvalidTypeName(ref ch) => {
                format!("Suggestion: Unexpected type'{}'.", ch)
            }
            LexError::InvalidNumber(ref n) => {
                format!("Suggestion: Invalid number '{}'.", n)
            }
            LexError::InvalidIdentifier(ref i) => {
                format!("Suggestion: Invalid identifier '{}'.", i)
            }
            LexError::InvalidOperator(ref o) => {
                format!("Suggestion: Invalid operator '{}'.", o)
            }
            LexError::InvalidSymbol(ref s) => {
                format!("Suggestion: Invalid symbol '{}'.", s)
            }
            LexError::InvalidStatement(ref s) => {
                format!("Suggestion: Invalid statement '{}'.", s)
            }
            LexError::InvalidDoubleSymbol(ref s) => {
                format!("Suggestion: Invalid double symbol '{}'.", s)
            }
            LexError::ExpectedQuote() => {
                "Suggestion: Check the syntax around the error line, and add a double quote."
                    .to_string()
            }
        };

        eprintln!("{}\n{}", errortxt, suggestion);
        Err(error)
        // std::process::exit(1);
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
    #[inline]
    fn reset_state(&mut self) {
        self.state = State::EmptyState;
        self.buffer_st = self.buffer_ed;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::neo_tokens::{Assign, Operator, Statement, Symbol, TypeName, TypeValue};
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
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("5"))),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("b".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("0"))),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    #[test]
    fn lexing_strings() {
        let code = " \"Hello, world!\" ";
        let _ans = [TokenType::TypeValue(TypeValue::QuotedString(
            Cow::Borrowed("Hello, world!"),
        ))];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        let _ = lexer.run();
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
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
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
            TokenType::TypeValue(TypeValue::QuotedString(Cow::Borrowed("Hello, world!"))),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
    // #[test]
    fn lexing_underbar_started_var() {
        let code = "let:i32 _a = 5;";
        let _ans = vec![
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("_a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("5"))),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
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
                TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("5"))),
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
                TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("2"))),
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
                TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("2"))),
                TokenMetadata::new(1, 26),
            ),
            Token::new(
                TokenType::Symbol(Symbol::Semicolon),
                TokenMetadata::new(1, 27),
            ),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
        for token in lexer.tokens.clone() {
            eprintln!("{:?}", token);
        }
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
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("5"))),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("b".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("0"))),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Println),
            TokenType::Symbol(Symbol::OpenParen),
            TokenType::TypeValue(TypeValue::QuotedString(Cow::Borrowed("Hello, world!"))),
            TokenType::Symbol(Symbol::CloseParen),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Statement(Statement::Return),
            TokenType::Symbol(Symbol::Semicolon),
            TokenType::Symbol(Symbol::CloseBrace),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        lexer.run().unwrap();
        println!("{:?}", lexer.tokens);
        // assert_eq!(lexer.tokens, ans);
    }
}
