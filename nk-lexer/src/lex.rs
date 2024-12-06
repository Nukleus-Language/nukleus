use crate::LexerError;
use crate::tokens::*;

//use crate::core::lexer::{Operator, Logical, Assigns};

// Struct to hold the lexer state
struct Lexer {
    tokens: Vec<Token>, // Vector of tokens
    pos: usize,         // Current position in the vector
}

// Returns a new lexer
impl Lexer {
    fn new(code: &str) -> Self {
        // Tokenize the code
        let tokens = lexer(code);

        Lexer { tokens, pos: 0 }
    }
}

// Returns the next token from the lexer
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        }
    }
}

// Returns a vector of tokens from a string
pub fn lexer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut double_state = false;
    let mut string_flag = false;

    // Iterate through the string by character
    for (i, c) in code.chars().enumerate() {
        // Check if is a String
        if c == '"' && !string_flag {
            string_flag = true;
            buffer.clear();
            continue;
        }
        if string_flag {
            buffer.push(c);
            if c == '"' {
                string_flag = false;
                tokens.push(Token::TypeValue(TypeValue::QuotedString(
                    buffer.trim_matches('"').to_string(),
                )));
                buffer.clear();
            }
            continue;
        }

        // Check if the current character is a letter or a digit
        if c.is_alphabetic() || c.is_numeric() || (c == '_' && !buffer.is_empty()) {
            buffer.push(c);

            // If the next character is not a digit, add the numeric literal to the tokens list
            if (i + 1 == code.len() || !code.chars().nth(i + 1).unwrap().is_alphanumeric())
                && buffer.chars().all(char::is_numeric)
            {
                tokens.push(Token::TypeValue(TypeValue::I32(buffer.parse().unwrap())));
                buffer.clear();
            }
            continue;
        }

        // Check if the character has been used in a double state
        if double_state {
            double_state = false;
            continue;
        }
        // Check for Symbol that is made with two Symbols
        match c {
            '-' => {
                // Check if the Double Symbol is a Arrow
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '>' {
                    tokens.push(Token::Symbol(Symbol::Arrow));
                    buffer.clear();
                    double_state = true;
                    continue;
                } else if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::SubAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '=' => {
                // Check if the Double Symbol is a Equals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Logical(Logical::Equals));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '!' => {
                // Check if the Double Symbol is a NotEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Logical(Logical::NotEquals));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '+' => {
                // Check if the Double Symbol is a PlusEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::AddAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '*' => {
                // Check if the Double Symbol is a StarEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::MulAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '/' => {
                // Check if the Double Symbol is a SlashEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::DivAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            '%' => {
                // Check if the Double Symbol is a PercentEquals
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token::Assign(Assign::RemAssign));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }

            ':' => {
                // Check if the Double Symbol is a DoubleColon
                if i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == ':' {
                    tokens.push(Token::Symbol(Symbol::DoubleColon));
                    buffer.clear();
                    double_state = true;
                    continue;
                }
            }
            _ => {}
        }

        // If the buffer is not empty, check if it contains a keyword or identifier
        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Token::Statement(Statement::Let),
                "fn" => Token::Statement(Statement::Function),
                //"->" => Token::Symbol(Symbol::Arrow),
                //"::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                //"==" => Token::Logical(Logical::Equals),
                //"!=" => Token::Logical(Logical::NotEquals),
                "public" => Token::Statement(Statement::Public),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "void" => Token::TypeName(TypeName::Void),
                "bool" => Token::TypeName(TypeName::Bool),
                "String" => Token::TypeName(TypeName::QuotedString),
                "i8" => Token::TypeName(TypeName::I8),
                "i16" => Token::TypeName(TypeName::I16),
                "i32" => Token::TypeName(TypeName::I32),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::TypeValue(TypeValue::Identifier(
                    identifier_parser(buffer.clone()).unwrap(),
                )),
            };
            tokens.push(token);
            buffer.clear();
        }

        // Add the symbol to the tokens list
        let token = match c {
            '*' => Token::Operator(Operator::Multiply),
            '@' => Token::Symbol(Symbol::At),
            //'^' => Tokens::Carat,
            ':' => Token::Symbol(Symbol::Colon),
            '.' => Token::Symbol(Symbol::Dot),
            '=' => Token::Assign(Assign::Assign),
            '-' => Token::Operator(Operator::Subtract),
            '(' => Token::Symbol(Symbol::OpenParen),
            '{' => Token::Symbol(Symbol::OpenBrace),
            '<' => Token::Logical(Logical::LessThan),
            '[' => Token::Symbol(Symbol::OpenSquare),
            ')' => Token::Symbol(Symbol::CloseParen),
            '}' => Token::Symbol(Symbol::CloseBrace),
            '>' => Token::Logical(Logical::GreaterThan),
            ']' => Token::Symbol(Symbol::CloseSquare),
            '+' => Token::Operator(Operator::Add),
            '%' => Token::Operator(Operator::Remainder),
            ';' => Token::Symbol(Symbol::Semicolon),
            '/' => Token::Operator(Operator::Divide),
            ',' => Token::Symbol(Symbol::Comma),
            '!' => Token::Logical(Logical::Not),

            ' ' | '\n' | '\t' | '\u{20}' | '\r' => continue,
            _ => panic!("Unexpected character: {}", c),
        };
        tokens.push(token);

        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Token::Statement(Statement::Let),
                "fn" => Token::Statement(Statement::Function),
                //"->" => Token::Symbol(Symbol::Arrow),
                //"::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                //"==" => Token::Logical(Logical::Equals),
                //"!=" => Token::Logical(Logical::NotEquals),
                "public" => Token::Statement(Statement::Public),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "void" => Token::TypeName(TypeName::Void),
                "bool" => Token::TypeName(TypeName::Bool),
                "String" => Token::TypeName(TypeName::QuotedString),
                "i8" => Token::TypeName(TypeName::I8),
                "i16" => Token::TypeName(TypeName::I16),
                "i32" => Token::TypeName(TypeName::I32),
                "i64" => Token::TypeName(TypeName::I64),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::TypeValue(TypeValue::Identifier(
                    identifier_parser(buffer.clone()).unwrap(),
                )),
            };
            tokens.push(token);
        }
    }

    tokens
}
// a Identifier cannot start with a number and can only contain letters, numbers and underscores
fn identifier_parser(buffer: String) -> Result<String, LexerError> {
    if buffer.chars().next().unwrap().is_numeric() {
        return Err(LexerError::InvalidIdentifierNum(buffer));
    }
    if buffer.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Err(LexerError::InvalidIdentifierChar(buffer));
    }
    Ok(buffer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_identifiers() {
        let code = "let:i32 x = 3 + 4;";
        let expected = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("x".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::I32(3)),
            Token::Operator(Operator::Add),
            Token::TypeValue(TypeValue::I32(4)),
            Token::Symbol(Symbol::Semicolon),
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_underscore_identifiers() {
        let code = "let:i8 x_ = 3 + 4;";
        let expected = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I8),
            Token::TypeValue(TypeValue::Identifier("x_".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::I32(3)),
            Token::Operator(Operator::Add),
            Token::TypeValue(TypeValue::I32(4)),
            Token::Symbol(Symbol::Semicolon),
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_keywords() {
        let code = "fn main\n{\n  let:i32 i=0;\n  return;\n}\n";
        let expected = vec![
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_string())),
            Token::Symbol(Symbol::OpenBrace),
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::I32),
            Token::TypeValue(TypeValue::Identifier("i".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::I32(0)),
            Token::Symbol(Symbol::Semicolon),
            Token::Statement(Statement::Return),
            Token::Symbol(Symbol::Semicolon),
            Token::Symbol(Symbol::CloseBrace),
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_symbols() {
        let code = "fn main() -> i32 {\n  return 0;\n}\n";
        let expected = vec![
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_string())),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Arrow),
            Token::TypeName(TypeName::I32),
            Token::Symbol(Symbol::OpenBrace),
            Token::Statement(Statement::Return),
            Token::TypeValue(TypeValue::I32(0)),
            Token::Symbol(Symbol::Semicolon),
            Token::Symbol(Symbol::CloseBrace),
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_numbers() {
        let code = "1 + 2 - 3 * 4 / 5 % 6";
        let expected = vec![
            Token::TypeValue(TypeValue::I32(1)),
            Token::Operator(Operator::Add),
            Token::TypeValue(TypeValue::I32(2)),
            Token::Operator(Operator::Subtract),
            Token::TypeValue(TypeValue::I32(3)),
            Token::Operator(Operator::Multiply),
            Token::TypeValue(TypeValue::I32(4)),
            Token::Operator(Operator::Divide),
            Token::TypeValue(TypeValue::I32(5)),
            Token::Operator(Operator::Remainder),
            Token::TypeValue(TypeValue::I32(6)),
        ];

        assert_eq!(expected, lexer(code));
    }
    #[test]
    fn lexing_strings() {
        let code = "let:String x = \"hello world\";";
        let expected = vec![
            Token::Statement(Statement::Let),
            Token::Symbol(Symbol::Colon),
            Token::TypeName(TypeName::QuotedString),
            Token::TypeValue(TypeValue::Identifier("x".to_string())),
            Token::Assign(Assign::Assign),
            Token::TypeValue(TypeValue::QuotedString("hello world".to_string())),
            Token::Symbol(Symbol::Semicolon),
        ];

        assert_eq!(expected, lexer(code));
    }
}
