use crate::errors::LexerError;
use crate::tokens::*;

//use crate::core::lexer::{Operator, Logical, Assigns};

// Struct to hold the lexer state
struct Lexer {
    tokens: Vec<Token>, // Vector of tokens
    pos: usize,          // Current position in the vector
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
    let mut arrow_flag = false;
    let mut equals_flag = false;
    let mut double_colon_flag = false;
    let mut not_equals_flag = false;
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
                tokens.push(Token::QuoatedString(buffer.trim_matches('"').to_string()));
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
                tokens.push(Token::I32(buffer.parse().unwrap()));
                buffer.clear();
            }
            continue;
        }

        // Check if the current character is a Arrow
        if c == '-' && i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '>' {
            buffer.push('>');
            tokens.push(Token::Symbol(Symbol::Arrow));
            buffer.clear();
            arrow_flag = true;
            continue;
        }
        if arrow_flag {
            arrow_flag = false;
            continue;
        }

        // Check if the current character is a Equals
        if c == '=' && i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
            buffer.push('=');
            tokens.push(Token::Logical(Logical::Equals));
            buffer.clear();
            equals_flag = true;
            continue;
        }
        if equals_flag {
            equals_flag = false;
            continue;
        }

        // Check if the current character is a DoubleColon
        if c == ':' && i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == ':' {
            buffer.push(':');
            tokens.push(Token::Symbol(Symbol::DoubleColon));
            buffer.clear();
            double_colon_flag = true;
            continue;
        }
        if double_colon_flag {
            double_colon_flag = false;
            continue;
        }

        // Check if the current character is a NotEquals
        if c == '!' && i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '=' {
            buffer.push('=');
            tokens.push(Token::Logical(Logical::NotEquals));
            buffer.clear();
            not_equals_flag = true;
            continue;
        }
        if not_equals_flag {
            not_equals_flag = false;
            continue;
        }

        // If the buffer is not empty, check if it contains a keyword or identifier
        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Token::Statement(Statement::Let),
                "fn" => Token::Statement(Statement::Function),
                "->" => Token::Symbol(Symbol::Arrow),
                "::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                "==" => Token::Logical(Logical::Equals),
                "!=" => Token::Logical(Logical::NotEquals),
                "public" => Token::Statement(Statement::Public),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "void" => Token::TypeName(TypeName::Void),
                "i32" => Token::TypeName(TypeName::I32),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::Identifier(Identifier_parser(buffer.clone()).unwrap()),
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
                "->" => Token::Symbol(Symbol::Arrow),
                "::" => Token::Symbol(Symbol::DoubleColon),
                "return" => Token::Statement(Statement::Return),
                "import" => Token::Statement(Statement::Import),
                "==" => Token::Logical(Logical::Equals),
                "!=" => Token::Logical(Logical::NotEquals),
                "public" => Token::Statement(Statement::Public),
                "if" => Token::Statement(Statement::If),
                "else" => Token::Statement(Statement::Else),
                "while" => Token::Statement(Statement::While),
                "print" => Token::Statement(Statement::Print),
                "println" => Token::Statement(Statement::Println),
                "for" => Token::Statement(Statement::For),
                "void" => Token::TypeName(TypeName::Void),
                "i32" => Token::TypeName(TypeName::I32),

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Token::Identifier(Identifier_parser(buffer.clone()).unwrap()),
            };
            tokens.push(token);
        }
    }

    tokens
}
// a Identifier cannot start with a number and can only contain letters, numbers and underscores
fn Identifier_parser(buffer: String) -> Result<String, LexerError> {
    if buffer.chars().next().unwrap().is_numeric() {
        return Err(LexerError::InvalidIdentifierNum);
    }
    if buffer.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        return Err(LexerError::InvalidIdentifierChar);
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
            Token::Identifier("x".to_string()),
            Token::Assign(Assign::Assign),
            Token::I32(3),
            Token::Operator(Operator::Add),
            Token::I32(4),
            Token::Symbol(Symbol::Semicolon),
        ];
        assert_eq!(expected, lexer(code));
    }

    /*#[test]
    fn lexer_underscore_identifiers() {
        let code = "let<int> x_ = 3 + 4;";
        let expected = vec![
            Tokens::Let,
            Tokens::OpenAngle,
            Tokens::Identifier("int".to_string()),
            Tokens::CloseAngle,
            Tokens::Identifier("x_".to_string()),
            Tokens::Assign,
            Tokens::Integer(3),
            Tokens::Plus,
            Tokens::Integer(4),
            Tokens::Semicolon,
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_keywords() {
        let code = "fn main\n{\n  let<int> i=0;\n  return;\n}\n";
        let expected = vec![
            Tokens::Function,
            Tokens::Identifier("main".to_string()),
            Tokens::OpenBrace,
            Tokens::Let,
            Tokens::OpenAngle,
            Tokens::Identifier("int".to_string()),
            Tokens::CloseAngle,
            Tokens::Identifier("i".to_string()),
            Tokens::Assign,
            Tokens::Integer(0),
            Tokens::Semicolon,
            Tokens::Return,
            Tokens::Semicolon,
            Tokens::CloseBrace,
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_symbols() {
        let code = "fn main() -> int {\n  return 0;\n}\n";
        let expected = vec![
            Tokens::Function,
            Tokens::Identifier("main".to_string()),
            Tokens::OpenParen,
            Tokens::CloseParen,
            Tokens::Arrow,
            Tokens::Identifier("int".to_string()),
            Tokens::OpenBrace,
            Tokens::Return,
            Tokens::Integer(0),
            Tokens::Semicolon,
            Tokens::CloseBrace,
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
    fn lexer_numbers() {
        let code = "1 + 2 - 3 * 4 / 5 % 6";
        let expected = vec![
            Tokens::Integer(1),
            Tokens::Plus,
            Tokens::Integer(2),
            Tokens::Minus,
            Tokens::Integer(3),
            Tokens::Asterisk,
            Tokens::Integer(4),
            Tokens::Slash,
            Tokens::Integer(5),
            Tokens::Percent,
            Tokens::Integer(6),
        ];

        assert_eq!(expected, lexer(code));
    }
    #[test]
    fn lexing_strings() {
        let code = "let<String> x = \"hello world\";";
        let expected = vec![
            Tokens::Let,
            Tokens::OpenAngle,
            Tokens::Identifier("String".to_string()),
            Tokens::CloseAngle,
            Tokens::Identifier("x".to_string()),
            Tokens::Assign,
            Tokens::QuotedString("hello world".to_string()),
            Tokens::Semicolon,
        ];

        assert_eq!(expected, lexer(code));
    }*/
}
