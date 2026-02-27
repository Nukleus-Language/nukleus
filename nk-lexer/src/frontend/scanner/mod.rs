mod errors;
mod identifier;
mod symbol;
mod value;

use errors::{LexError, LexicalError};

use crate::neo_tokens::{Symbol, Token, TokenMetadata, TokenType, TypeValue};
use std::borrow::Cow;

#[cfg(test)]
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
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
    tokens: Vec<Token>,
    state: State,
    buffer_st: usize,
    buffer_ed: usize,
    line: usize,
    column: usize,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(_file_path: std::path::PathBuf, code: &'a str) -> Self {
        let estimated_tokens = code.len() / 5;
        Lexer {
            tokens: Vec::with_capacity(estimated_tokens),
            state: State::EmptyState,
            buffer_st: 0,
            buffer_ed: 0,
            line: 1,
            column: 0,
            source: code,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn run(&mut self) -> Result<(), LexicalError> {
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
                    let end = self.buffer_ed - c.len_utf8();
                    let string = &self.source[self.buffer_st + 1..end];
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
            return self.report_error(LexicalError {
                line: self.line,
                column: self.column,
                message: LexError::ExpectedQuote(),
                note: None,
            });
        }

        Ok(())
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        let rest = self.source.get(self.buffer_ed..)?;
        let ch = rest.chars().next()?;
        self.buffer_ed += ch.len_utf8();

        match ch {
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            '\t' => self.column += 4,
            _ => self.column += 1,
        }

        Some(ch)
    }

    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.source.get(self.buffer_ed..)?.chars().next()
    }

    #[inline]
    fn insert_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(
            token,
            TokenMetadata::new(self.line, self.column),
        ));
    }

    fn report_error(&self, error: LexicalError) -> Result<(), LexicalError> {
        let context_window = 10;

        let start = self.buffer_st.saturating_sub(context_window);
        let end = std::cmp::min(self.buffer_ed + context_window, self.source.len());

        let context_snippet = &self.source[start..end];
        let error_pos_in_context = self.source[start..self.buffer_st].chars().count();
        let error_location_marker = " ".repeat(error_pos_in_context) + "^";

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

        let note = format!(
            "context:\n{}\n{}\n{}",
            context_snippet, error_location_marker, suggestion
        );
        Err(error.with_note(note))
    }

    pub fn get_tokens(&self) -> &[Token] {
        self.tokens.as_slice()
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

    fn token_with_type<'a>(tokens: &'a [Token], expected: &TokenType) -> Option<&'a Token> {
        tokens.iter().find(|token| &token.token_type == expected)
    }

    fn first_quoted_string_token(tokens: &[Token]) -> Option<&Token> {
        tokens.iter().find(|token| {
            matches!(
                &token.token_type,
                TokenType::TypeValue(TypeValue::QuotedString(_))
            )
        })
    }

    #[test]
    fn lexing_utf8_multibyte() {
        let code = "let:String msg = \"Hello \u{4E2D}\u{6587}\";";
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        let result = lexer.run();
        assert!(
            result.is_ok(),
            "UTF-8 multibyte lexing failed: {:?}",
            result
        );
        let tokens = lexer.get_tokens();
        assert!(!tokens.is_empty(), "Expected tokens from UTF-8 source");

        let quoted_string_token = first_quoted_string_token(tokens);
        assert!(
            quoted_string_token.is_some(),
            "Expected quoted string token in UTF-8 source"
        );

        let semicolon_token = token_with_type(tokens, &TokenType::Symbol(Symbol::Semicolon));
        assert!(semicolon_token.is_some(), "Expected semicolon token");

        let quoted_string_column = quoted_string_token
            .map(|token| token.metadata.column)
            .unwrap_or_default();
        let semicolon_column = semicolon_token
            .map(|token| token.metadata.column)
            .unwrap_or_default();
        assert_eq!(
            semicolon_column,
            quoted_string_column + 1,
            "Semicolon should appear right after closing quote"
        );
    }

    #[test]
    fn lexing_utf8_multibyte_across_lines_tracks_metadata() {
        let code = "let:String first = \"\u{4E2D}\"\nlet:String second = \"\u{6587}\";";
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        let result = lexer.run();
        assert!(
            result.is_ok(),
            "UTF-8 multiline lexing failed: {:?}",
            result
        );

        let tokens = lexer.get_tokens();
        let second_identifier = token_with_type(
            tokens,
            &TokenType::TypeValue(TypeValue::Identifier("second".to_string())),
        );
        assert!(
            second_identifier.is_some(),
            "Expected second identifier token"
        );
        assert_eq!(
            second_identifier.map(|token| token.metadata.line),
            Some(2),
            "Expected second identifier on line 2"
        );

        let final_semicolon = tokens.last();
        assert!(
            matches!(
                final_semicolon.map(|token| &token.token_type),
                Some(TokenType::Symbol(Symbol::Semicolon))
            ),
            "Expected final token to be semicolon"
        );
        assert_eq!(
            final_semicolon.map(|token| token.metadata.line),
            Some(2),
            "Expected final semicolon on line 2"
        );
    }

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
        assert!(lexer.run().is_ok(), "lexing_numbers failed");
    }
    #[test]
    fn lexing_strings() {
        let code = " \"Hello, world!\" ";
        let _ans = [TokenType::TypeValue(TypeValue::QuotedString(
            Cow::Borrowed("Hello, world!"),
        ))];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        let result = lexer.run();
        assert!(result.is_ok(), "String lexing failed: {:?}", result);
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
        assert!(lexer.run().is_ok(), "lexing_comments failed");
    }
    #[test]
    fn lexing_string_assign() {
        let code = "let:String a = \"Hello, world!\";";
        let _ans = [
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::QuotedString),
            TokenType::TypeValue(TypeValue::Identifier("a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::QuotedString(Cow::Borrowed("Hello, world!"))),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        assert!(lexer.run().is_ok(), "lexing_string_assign failed");
    }
    #[test]
    fn lexing_underbar_started_var() {
        let code = "let:i32 _a = 5;";
        let _ans = [
            TokenType::Statement(Statement::Let),
            TokenType::Symbol(Symbol::Colon),
            TokenType::TypeName(TypeName::I32),
            TokenType::TypeValue(TypeValue::Identifier("_a".to_string())),
            TokenType::Assign(Assign::Assign),
            TokenType::TypeValue(TypeValue::Number(Cow::Borrowed("5"))),
            TokenType::Symbol(Symbol::Semicolon),
        ];
        let mut lexer = Lexer::new(PathBuf::from("test"), code);
        assert!(lexer.run().is_ok(), "lexing_underbar_started_var failed");
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
        assert!(lexer.run().is_ok(), "lexing_nested_expression failed");
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
        assert!(lexer.run().is_ok(), "lexing_complex failed");
    }
}
