use super::Tokens;
use super::Operator;
use super::LexerError;

// Struct to hold the lexer state
struct Lexer {
    tokens: Vec<Tokens>, // Vector of tokens
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
    type Item = Tokens;

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
pub fn lexer(code: &str) -> Vec<Tokens> {
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
                tokens.push(Tokens::QuotedString(buffer.trim_matches('"').to_string()));
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
                tokens.push(Tokens::Integer(buffer.parse().unwrap()));
                buffer.clear();
            }
            continue;
        }

        // Check if the current character is a Arrow
        if c == '-' && i + 1 < code.len() && code.chars().nth(i + 1).unwrap() == '>' {
            buffer.push('>');
            tokens.push(Tokens::Arrow);
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
            tokens.push(Tokens::Equals);
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
            tokens.push(Tokens::DoubleColon);
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
            //tokens.push(Token_test::Operator::NotEquals);
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
                "let" => Tokens::Let,
                "fn" => Tokens::Function,
                "->" => Tokens::Arrow,
                "::" => Tokens::DoubleColon,
                "return" => Tokens::Return,
                "import" => Tokens::Import,
                "==" => Tokens::Equals,
                "public" => Tokens::Public,
                "if" => Tokens::If,
                "else" => Tokens::Else,
                "while" => Tokens::While,
                "print" => Tokens::Print,
                "println" => Tokens::Println,
                "for" => Tokens::For,
                "void" => Tokens::Void,

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Tokens::Identifier(Identifier_parser(buffer.clone()).unwrap()),
            };
            tokens.push(token);
            buffer.clear();
        }

        // Add the symbol to the tokens list
        let token = match c {
            '*' => Tokens::Asterisk,
            '@' => Tokens::At,
            '^' => Tokens::Carat,
            ':' => Tokens::Colon,
            '.' => Tokens::Dot,
            '=' => Tokens::Assign,
            '-' => Tokens::Minus,
            '(' => Tokens::OpenParen,
            '{' => Tokens::OpenBrace,
            '<' => Tokens::OpenAngle,
            '[' => Tokens::OpenSquare,
            ')' => Tokens::CloseParen,
            '}' => Tokens::CloseBrace,
            '>' => Tokens::CloseAngle,
            ']' => Tokens::CloseSquare,
            '+' => Tokens::Plus,
            '%' => Tokens::Percent,
            ';' => Tokens::Semicolon,
            '/' => Tokens::Slash,
            ',' => Tokens::Comma,
            '!' => Tokens::Operator(Operator::Not),

            ' ' | '\n' | '\t' | '\u{20}' | '\r' => continue,
            _ => panic!("Unexpected character: {}", c),
        };
        tokens.push(token);

        if !buffer.is_empty() {
            let token = match buffer.as_str() {
                "let" => Tokens::Let,
                "fn" => Tokens::Function,
                "->" => Tokens::Arrow,
                "::" => Tokens::DoubleColon,
                "return" => Tokens::Return,
                "import" => Tokens::Import,
                "==" => Tokens::Equals,
                "public" => Tokens::Public,
                "print" => Tokens::Print,
                "println" => Tokens::Println,
                "if" => Tokens::If,
                "else" => Tokens::Else,
                "while" => Tokens::While,
                "for" => Tokens::For,
                "void" => Tokens::Void,

                " " | "\n" | "\t" | "\u{20}" | "\r" => continue,
                _ => Tokens::Identifier(Identifier_parser(buffer.clone()).unwrap()),
            };
            tokens.push(token);
        }
    }

    tokens
}
// a Identifier cannot start with a number and can only contain letters, numbers and underscores
fn Identifier_parser(buffer: String) -> Result<String, LexerError> {
    
    if buffer.chars().nth(0).unwrap().is_numeric() {
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
        let code = "let<int> x = 3 + 4;";
        let expected = vec![
            Tokens::Let,
            Tokens::OpenAngle,
            Tokens::Identifier("int".to_string()),
            Tokens::CloseAngle,
            Tokens::Identifier("x".to_string()),
            Tokens::Assign,
            Tokens::Integer(3),
            Tokens::Plus,
            Tokens::Integer(4),
            Tokens::Semicolon,
        ];
        assert_eq!(expected, lexer(code));
    }

    #[test]
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
    }
}
