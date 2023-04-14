mod errors;
mod identifier;
mod symbol;
mod value;

use errors::LexError;
use errors::LexcialError;

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
            if is_whitespace(c) && self.state != State::QuotedString {
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
                    double_symbol_to_token(self.buffer.clone(), self.line, self.column);
                match double_symbol {
                    Ok(double_symbol) => {
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
                let symbol = symbol_to_token(c, self.line, self.column);
                match symbol {
                    Ok(symbol) => {
                        self.insert_token(symbol);
                        continue;
                    }
                    Err(_) => {}
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
            if self.state == State::StateDefault && (first_char == '-' || is_numeric(first_char)) {
                self.state = State::Number;
                //self.buffer.push(c);
            } else if self.state == State::Number && is_numeric(c) {
                self.buffer.push(c);
            }
            if self.state == State::Number && !is_numeric(peeked_char) {
                //let number = buffer.parse::<i32>().unwrap();
                let number = number_to_token(self.buffer.clone(), self.line, self.column);
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
                let statement = statement_to_token(self.buffer.clone(), self.line, self.column);
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

fn is_numeric(c: char) -> bool {
    c.is_numeric()
}
fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}
fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
fn is_quote(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' => true,
        _ => false,
    }
}
fn is_symbol(c: char) -> bool {
    match c {
        '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' | '.' => true,
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
    is_alpha(c) || is_numeric(c) || c == '_'
}
fn is_first_identifierable(c: char) -> bool {
    is_alpha(c) || c == '_'
}
fn operator_to_token(operator: char, line: usize, column: usize) -> Result<Token, LexcialError> {
    match operator {
        '+' => Ok(Token::Operator(Operator::Add)),
        '-' => Ok(Token::Operator(Operator::Subtract)),
        '*' => Ok(Token::Operator(Operator::Multiply)),
        '/' => Ok(Token::Operator(Operator::Divide)),
        '%' => Ok(Token::Operator(Operator::Remainder)),
        _ => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidOperator(operator.to_string()),
            })
        }
    }
}
fn number_to_token(number: String, line: usize, column: usize) -> Result<Token, LexcialError> {
    //check if the number is parseable while not changing the type of number to i32
    //println!("BEF Number: {}", number);
    let test_parse = number.parse::<u64>();
    //println!("AFT Number: {}", number);
    match test_parse {
        Ok(number) => Ok(Token::TypeValue(TypeValue::Number(number.to_string()))),
        Err(_) => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidNumber(number),
            })
        }
    }
}
//fn quoted_string_to_token(string: String, line:usize, column:usize) -> Result<Token, LexcialError> {
//    Ok(Token::String(string))
//}
fn symbol_to_token(symbol: char, line: usize, column: usize) -> Result<Token, LexcialError> {
    match symbol {
        ',' => Ok(Token::Symbol(Symbol::Comma)),
        ':' => Ok(Token::Symbol(Symbol::Colon)),
        '.' => Ok(Token::Symbol(Symbol::Dot)),
        '=' => Ok(Token::Assign(Assign::Assign)),
        //'-' => Token::Operator(Operator::Subtract),
        '(' => Ok(Token::Symbol(Symbol::OpenParen)),
        '{' => Ok(Token::Symbol(Symbol::OpenBrace)),
        //"<' => Ok(Token::Logical(Logical::LessThan)),
        '[' => Ok(Token::Symbol(Symbol::OpenSquare)),
        ')' => Ok(Token::Symbol(Symbol::CloseParen)),
        '}' => Ok(Token::Symbol(Symbol::CloseBrace)),
        //'>' => Ok(Token::Logical(Logical::GreaterThan)),
        ']' => Ok(Token::Symbol(Symbol::CloseSquare)),
        //'+' => Token::Operator(Operator::Add),
        //'%' => Token::Operator(Operator::Remainder),
        ';' => Ok(Token::Symbol(Symbol::Semicolon)),
        '/' => Ok(Token::Operator(Operator::Divide)),
        //"," => Ok(Token::Symbol(Symbol::Comma)),
        //"!" => Ok(Token::Logical(Logical::Not)),
        _ => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidSymbol(symbol.to_string()),
            })
        }
    }
}
fn statement_to_token(
    statement: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match statement.as_str() {
        "let" => Ok(Token::Statement(Statement::Let)),
        "fn" => Ok(Token::Statement(Statement::Function)),
        //"->" => Token::Symbol(Symbol::Arrow),
        //"::" => Token::Symbol(Symbol::DoubleColon),
        "return" => Ok(Token::Statement(Statement::Return)),
        "import" => Ok(Token::Statement(Statement::Import)),
        //"==" => Token::Logical(Logical::Equals),
        //"!=" => Token::Logical(Logical::NotEquals),
        "public" => Ok(Token::Statement(Statement::Public)),
        "if" => Ok(Token::Statement(Statement::If)),
        "else" => Ok(Token::Statement(Statement::Else)),
        "while" => Ok(Token::Statement(Statement::While)),
        "print" => Ok(Token::Statement(Statement::Print)),
        "println" => Ok(Token::Statement(Statement::Println)),
        "for" => Ok(Token::Statement(Statement::For)),
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
                message: LexError::InvalidStatement(statement.to_string()),
            })
        }
    }
}
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

fn double_symbol_to_token(
    double_symbol: String,
    line: usize,
    column: usize,
) -> Result<Token, LexcialError> {
    match double_symbol.as_str() {
        "==" => Ok(Token::Logical(Logical::Equals)),
        "!=" => Ok(Token::Logical(Logical::NotEquals)),
        "->" => Ok(Token::Symbol(Symbol::Arrow)),
        "::" => Ok(Token::Symbol(Symbol::DoubleColon)),
        _ => {
            Err(LexcialError {
                line,
                column,
                message: LexError::InvalidDoubleSymbol(double_symbol.to_string()),
            })
        }
    }
}
/*fn identifier_to_token(identifier: String, line:usize, column:usize) -> Result<Token, LexcialError>{
    match identifier.as_str() {
        _ => Ok(Token::Identifier(identifier)),
    }
}*/

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
            Token::TypeValue(TypeValue::Numbera(-5.to_string())),
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
