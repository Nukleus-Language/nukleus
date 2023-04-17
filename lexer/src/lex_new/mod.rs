mod errors;
mod identifier;
mod symbol;
mod value;

use errors::LexError;
use errors::LexcialError;
use value::number_to_token;
use identifier::statement_to_token;
use symbol::symbol_to_token;
use symbol::double_symbol_to_token;

use std::iter::Peekable;
use std::str::Chars;

use crate::tokens_new::*;

#[derive(Debug, Clone, PartialEq)]
enum State {
    StateEmpty,
    StateDefault,
    Number,
    Identifier,
    QuotedString,
    DoubleState,
    Comment,
}

struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    state: State,
    buffer: String,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer {
            code: code.chars().peekable(),
            tokens: Vec::new(),
            state: State::StateEmpty,
            buffer: String::new(),
            line: 1,
            column: 1,
        }
    }
    fn run(&mut self) {
        //let mut state = State::StateDefault;
        while let Some(c) = self.next_char() {
            let peeked_char = self.peek_char();
            println!("---------------------------------");
            println!("Current Char: {}", c);
            println!("Current State: {:?}", self.state);
            println!("Current Buffer: {}", self.buffer);
            if self.state == State::DoubleState {
                self.buffer.clear();
                println!("Double");
                self.state = State::StateEmpty;

                continue;
            }
            if self.state == State::Comment {
                if c == '\n'{
                    self.state = State::StateEmpty;
                    self.buffer.clear();
                    continue;
                }
                continue;
            }
            if c.is_whitespace() && self.state != State::QuotedString {
                self.buffer.clear();
                self.state = State::StateEmpty;
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
                if !peeked_char.is_numeric(){
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

                self.state = State::StateDefault;
                //continue;
            }
            println!(
                "Current First Char: {}",
                self.buffer.chars().next().unwrap()
            );

            // if the first character is a - or number and the next character is a number
            // then it is a number
            let first_char = self.buffer.chars().next().unwrap();
            if self.state == State::StateDefault && (first_char == '-' || first_char.is_numeric() ) {
                self.state = State::Number;
                //self.buffer.push(c);
            } else if self.state == State::Number && c.is_numeric(){
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
                self.state = State::StateEmpty;
                continue;
            }

            // if the first character is a " then it is a string
            if self.state == State::StateDefault && is_quote(first_char) {
                self.state = State::QuotedString;
                //self.buffer.push(c);
                continue;
            } else if self.state == State::QuotedString && !is_quote(c) {
                self.buffer.push(c);
                continue;
            } else if self.state == State::QuotedString && is_quote(c) {
                let string = self.buffer.clone();
                self.buffer.push(c);
                // trim the quotes
                let string = string.trim_matches('"').to_string();
                self.insert_token(Token::TypeValue(TypeValue::QuotedString(string)));
                self.buffer.clear();
                self.state = State::StateEmpty;
                continue;
            }

            // check if is a identifier, statement, or symbol
            if self.state == State::StateDefault && is_first_identifierable(first_char) {
                self.state = State::Identifier;
                //self.buffer.push(c);
            } else if self.state == State::Identifier && is_identifierable(c) {
                self.buffer.push(c);
            }
            if self.state == State::Identifier && !is_identifierable(peeked_char) {
                //let identifier = identifier_to_token(self.buffer.clone(), self.line, self.column);
                let statement = identifier::statement_to_token(self.buffer.clone(), self.line, self.column);
                match statement {
                    Ok(statement) => {
                        self.insert_token(statement);
                        self.buffer.clear();
                        self.state = State::StateEmpty;
                        continue;
                    }
                    Err(_) => {}
                }
                let identifier = Token::TypeValue(TypeValue::Identifier(self.buffer.clone()));
                self.insert_token(identifier);
                self.buffer.clear();
                self.state = State::StateEmpty;
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
    fn peek_char(&mut self) -> char {
        let peek = self.code.peek();
        match peek {
            Some(_) => *peek.unwrap(),
            None => ' ',
        }
    }
    fn insert_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn report_error(&self, error: LexcialError) {
        println!(
            "{} \n-------------> Line: {}, Column: {}",
            error, self.line, self.column
        );
    }
}

fn is_quote(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}

fn is_quoted_string(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}
fn is_identifierable(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
fn is_first_identifierable(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
/*fn operator_to_token(operator: char, line: usize, column: usize) -> Result<Token, LexcialError> {
    match operator {
        '+' => Ok(Token::Operator(Operator::Add)),
        '-' => Ok(Token::Operator(Operator::Subtract)),
        '*' => Ok(Token::Operator(Operator::Multiply)),
        '/' => Ok(Token::Operator(Operator::Divide)),
        '%' => Ok(Token::Operator(Operator::Remainder)),
        '&' => Ok(Token::Operator(Operator::And)),
        '|' => Ok(Token::Operator(Operator::Or)),
        '^' => Ok(Token::Operator(Operator::Xor)),
        _ => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidOperator(operator.to_string()),
            })
        }
    }
}*/


fn typename_to_token(typename: String, line: usize, column: usize) -> Result<Token, LexcialError> {
    match typename.as_str() {
        "void" => Ok(Token::TypeName(TypeName::Void)),
        "bool" => Ok(Token::TypeName(TypeName::Bool)),
        "string" => Ok(Token::TypeName(TypeName::QuotedString)),
        "i8" => Ok(Token::TypeName(TypeName::I8)),
        "i16" => Ok(Token::TypeName(TypeName::I16)),
        "i32" => Ok(Token::TypeName(TypeName::I32)),
        "i64" => Ok(Token::TypeName(TypeName::I64)),
        "u8" => Ok(Token::TypeName(TypeName::U8)),
        "u16" => Ok(Token::TypeName(TypeName::U16)),
        "u32" => Ok(Token::TypeName(TypeName::U32)),
        "u64" => Ok(Token::TypeName(TypeName::U64)),
        _ => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidTypeName(typename.to_string()),
            })
        }
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
    fn lexing_comments(){
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
    fn lexing_negative_number_assign(){
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
