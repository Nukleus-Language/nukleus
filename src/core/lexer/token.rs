use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Tokens {
    // Numeric literals
    Integer(usize),
    //Float(f64),
    //Decimal(f64),

    // Identifiers
    Identifier(String),

    // Strings
    QuotedString(String),

    // Type
    Void,
    Int,
    String,
    Bool,
    Float,

    // Symbols
    Asterisk,
    At,
    Carat,
    Colon,
    Dot,
    Assign,
    Minus,
    OpenParen,
    OpenBrace,
    OpenAngle,
    OpenSquare,
    CloseParen,
    CloseBrace,
    CloseAngle,
    CloseSquare,
    Plus,
    Arrow,
    Percent,
    Semicolon,
    Slash,

    // Keywords
    And,
    Break,
    If,
    Else,
    ElseIf,
    False,
    Function,
    Public,
    Not,
    Or,
    For,
    While,
    Print,
    Println,
    Return,
    True,
    Let,
    Equals,
    Import,
    // End of file
    EOF,
}

impl Tokens {
    /// Returns a string representation of the token.
    pub fn as_str(&self) -> &str {
        match *self {
            Tokens::Integer(_) => "integer",
            //Tokens::Float(_) => "float",
            //Tokens::Decimal(_) => "decimal",
            Tokens::Identifier(_) => "identifier",
            Tokens::QuotedString(_) => "string",
            Tokens::Asterisk => "*",
            Tokens::At => "@",
            Tokens::Carat => "^",
            Tokens::Colon => ":",
            Tokens::Dot => ".",
            Tokens::Assign => "=",
            Tokens::Minus => "-",
            Tokens::OpenParen => "(",
            Tokens::OpenBrace => "{",
            Tokens::OpenAngle => "<",
            Tokens::OpenSquare => "[",
            Tokens::CloseParen => ")",
            Tokens::CloseBrace => "}",
            Tokens::CloseAngle => ">",
            Tokens::CloseSquare => "]",
            Tokens::Plus => "+",
            Tokens::Arrow => "->",
            Tokens::Percent => "%",
            Tokens::Semicolon => ";",
            Tokens::Slash => "/",
            Tokens::And => "and",
            Tokens::Break => "break",
            Tokens::If => "if",
            Tokens::Else => "else",
            Tokens::ElseIf => "else if",
            Tokens::False => "false",
            Tokens::Function => "fn",
            Tokens::Print => "print",
            Tokens::Println => "println",
            Tokens::Public => "public",
            Tokens::Not => "not",
            Tokens::Or => "or",
            Tokens::For => "for",
            Tokens::While => "while",
            Tokens::Return => "return",
            Tokens::True => "true",
            Tokens::Let => "let",
            Tokens::Equals => "==",
            Tokens::Import => "import",
            Tokens::Int => "int",
            Tokens::Float => "float",
            Tokens::String => "string",
            Tokens::Bool => "bool",
            Tokens::Void => "void",
            Tokens::EOF => "EOF",
        }
    }
}
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tokens::Integer(n) => write!(f, "Integer({})", n),
            //Tokens::Float(n) => write!(f, "Float({})", n),
            //Tokens::Decimal(n) => write!(f, "Decimal({})", n),
            Tokens::Identifier(ref s) => write!(f, "Identifier({})", s),
            Tokens::QuotedString(ref s) => write!(f, "QuotedString({})", s),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn token_display() {
        let tokens = vec![
            Tokens::Identifier("let".to_string()),
            Tokens::Identifier("x".to_string()),
            Tokens::Assign,
            Tokens::Integer(3),
            Tokens::Plus,
            Tokens::Integer(4),
            Tokens::Semicolon,
        ];

        let expected = "Identifier(let), Identifier(x), =, Integer(3), +, Integer(4), ;";
        let result: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
        let result = result.join(", ");
        assert_eq!(expected, result);
    }
}
