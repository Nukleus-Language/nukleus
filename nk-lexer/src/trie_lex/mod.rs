mod errors;
mod identifier;
mod symbol;
mod trie;
mod value;

use errors::{LexError, LexcialError};

use std::borrow::Cow;
use std::path::PathBuf;

use crate::neo_tokens::{
    Symbol, Token, TokenMetadata, TokenType, TypeValue,
};

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
    code: &'a [u8],
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
    pub fn new(file_path: PathBuf, code: &'a str) -> Self {
        Lexer {
            code: code.as_bytes(),
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

    pub fn run(&mut self) -> Result<(), LexcialError> {
        while let Some(c) = self.next_char() {
            let peeked_char = self.peek_char().unwrap_or('\0');

            if let State::DoubleState = self.state {
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            if let State::Comment = self.state {
                if c == '\n' {
                    self.state = State::EmptyState;
                    self.buffer_st = self.buffer_ed;
                }
                continue;
            }

            if c.is_whitespace() && self.state != State::QuotedString {
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            if self.buffer_ed == self.buffer_st + c.len_utf8() {
                if peeked_char != '\0' {
                    let peeked_index = self.buffer_ed + peeked_char.len_utf8();
                    let double_symbol_str = &self.source[self.buffer_st..peeked_index];
                    if let Ok(double_symbol) =
                        symbol::double_symbol_to_token(double_symbol_str, self.line, self.column)
                    {
                        if double_symbol == TokenType::Symbol(Symbol::Comment) {
                            self.state = State::Comment;
                            continue;
                        }
                        self.insert_token(double_symbol);
                        self.state = State::DoubleState;
                        continue;
                    }
                }

                if let Ok(symbol) = symbol::symbol_to_token(c, self.line, self.column) {
                    self.insert_token(symbol);
                    self.buffer_st = self.buffer_ed;
                    continue;
                }

                if let Ok(operator) = symbol::operator_to_token(c, self.line, self.column) {
                    self.insert_token(operator);
                    self.buffer_st = self.buffer_ed;
                    continue;
                }

                self.state = State::DefaultState;
            }

            let first_char = self.source[self.buffer_st..self.buffer_ed]
                .chars()
                .next()
                .unwrap_or('\0');

            if self.state == State::DefaultState && (first_char == '-' || first_char.is_numeric()) {
                self.state = State::Number;
            }

            if self.state == State::Number && !peeked_char.is_numeric() {
                let number = value::number_to_token(
                    &self.source[self.buffer_st..self.buffer_ed],
                    self.line,
                    self.column,
                )?;
                self.insert_token(number);
                self.buffer_st = self.buffer_ed;
                self.state = State::EmptyState;
                continue;
            }

            if self.state == State::DefaultState && identifier::is_quote(c) {
                self.state = State::QuotedString;
                continue;
            } else if self.state == State::QuotedString {
                if !identifier::is_quote(c) {
                    continue;
                } else {
                    let string = &self.source[self.buffer_st + 1..self.buffer_ed];
                    self.insert_token(TokenType::TypeValue(TypeValue::QuotedString(Cow::Owned(
                        string.to_owned(),
                    ))));
                    self.buffer_st = self.buffer_ed;
                    self.state = State::EmptyState;
                    continue;
                }
            }

            if self.state == State::DefaultState && identifier::is_first_identifierable(first_char)
            {
                self.state = State::Identifier;
            }

            if self.state == State::Identifier && !identifier::is_identifierable(peeked_char) {
                let string = &self.source[self.buffer_st..self.buffer_ed];
                if let Ok(statement) =
                    identifier::statement_to_token(string, self.line, self.column)
                {
                    self.insert_token(statement);
                    self.buffer_st = self.buffer_ed;
                    self.state = State::EmptyState;
                    continue;
                }

                if let Ok(type_name) =
                    identifier::type_name_to_token(string, self.line, self.column)
                {
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

        if let State::QuotedString = self.state {
            return self.report_error(LexcialError {
                line: self.line,
                column: self.column,
                message: LexError::ExpectedQuote(),
            });
        }

        Ok(())
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        if self.buffer_ed < self.code.len() {
            let ch = self.code[self.buffer_ed] as char;
            self.update_position(ch);
            Some(ch)
        } else {
            None
        }
    }

    #[inline]
    fn update_position(&mut self, ch: char) {
        self.column += match ch {
            '\n' => {
                self.line += 1;
                0
            }
            '\t' => 4,
            _ => 1,
        };
        self.buffer_ed += ch.len_utf8();
    }

    fn peek_char(&self) -> Option<char> {
        if self.buffer_ed < self.code.len() {
            Some(self.code[self.buffer_ed] as char)
        } else {
            None
        }
    }

    fn insert_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(
            token,
            TokenMetadata::new(self.line, self.column),
        ));
    }

    fn report_error(&self, error: LexcialError) -> Result<(), LexcialError> {
        let context_window = 10;

        let start = self.buffer_st.saturating_sub(context_window);
        let end = std::cmp::min(self.buffer_ed + context_window, self.source.len());

        let context_snippet = &self.source[start..end];
        let error_pos_in_context = self.source[start..self.buffer_st].chars().count();
        let error_location_marker = " ".repeat(error_pos_in_context) + "^";

        let errortxt = format!(
            "Context:\n{}\n{}\n--> Error at Line: {}, Column: {}: {}",
            context_snippet, error_location_marker, self.line, self.column, error
        );
        let suggestion = match &error.message {
            LexError::InvalidCharacter(c) => {
                format!(
                    "Suggestion: Unexpected character '{}'. Try removing or replacing it.",
                    c
                )
            }
            LexError::InvalidTypeName(t) => {
                format!("Suggestion: Unexpected type '{}'.", t)
            }
            LexError::InvalidNumber(n) => {
                format!("Suggestion: Invalid number '{}'.", n)
            }
            LexError::InvalidIdentifier(i) => {
                format!("Suggestion: Invalid identifier '{}'.", i)
            }
            LexError::InvalidOperator(o) => {
                format!("Suggestion: Invalid operator '{}'.", o)
            }
            LexError::InvalidSymbol(s) => {
                format!("Suggestion: Invalid symbol '{}'.", s)
            }
            LexError::InvalidStatement(s) => {
                format!("Suggestion: Invalid statement '{}'.", s)
            }
            LexError::InvalidDoubleSymbol(s) => {
                format!("Suggestion: Invalid double symbol '{}'.", s)
            }
            LexError::ExpectedQuote() => {
                "Suggestion: Check the syntax around the error line and add a double quote."
                    .to_string()
            }
        };

        eprintln!("{}\n{}", errortxt, suggestion);
        Err(error)
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
    use crate::neo_tokens::{
        Assign, Statement, Symbol, TypeName, TypeValue, Operator
    };
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
        let _ans = vec![TokenType::TypeValue(TypeValue::QuotedString(
            Cow::Borrowed("Hello, world!"),
        ))];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
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
