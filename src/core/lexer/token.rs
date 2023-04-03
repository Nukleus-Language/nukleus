use std::fmt;

use super::operator::{Operator, Logical, Assigns};

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
    TypeName(TypeName),
    TypeValue(TypeValue),
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
    Comma,
    DoubleColon,
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
    Statement(Statement),
    And,
    Break,
    If,
    Else,
    ElseIf,
    False,
    Function,
    Public,
    For,
    While,
    Print,
    Println,
    Return,
    True,
    Let,
    Equals,
    Import,

    // Operators
    Operator(Operator),
    Logical(Logical),
    Assigns(Assigns),
    
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
            Tokens::TypeName(_) => "type name",
            Tokens::TypeValue(_) => "type value",
            //Tokens::Operator(_) => self.display(),
            // Need to show only the Operator, not the whole enum
            Tokens::Operator(_) => 
            Tokens::Logical(_) => "logical",
            Tokens::Assigns(_) =>   "assigns",
            Tokens::Statement(_) => "statement",

            Tokens::Asterisk => "*",
            Tokens::At => "@",
            Tokens::Carat => "^",
            Tokens::Colon => ":",
            Tokens::Comma => ",",
            Tokens::DoubleColon => "::",
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
    pub fn is_identifier(&self) -> bool {
        match *self {
            Tokens::Identifier(_) => true,
            _ => false,
        }
    }
    pub fn operator_precedence(&self) -> i32 {
        match self {
            //Tokens::Multiply | Tokens::Divide | Tokens::Modulus => 3,
            Tokens::Operator(Operator::Add) | Tokens::Operator(Operator::Subtract) => 4,
            //Tokens::LeftShift | Tokens::RightShift => 5,
            //Tokens::LessThan | Tokens::LessThanOrEqual | Tokens::GreaterThan | Tokens::GreaterThanOrEqual => 6,
            Tokens::Logical(Logical::Equals) | Tokens::Logical(Logical::NotEquals) => 7,
            //Tokens::BitwiseAnd => 8,
            //Tokens::BitwiseXor => 9,
            //Tokens::BitwiseOr => 10,
            Tokens::Logical(Logical::And) => 13,
            Tokens::Logical(Logical::Or) => 14,
            //Tokens::Assign | Tokens::AddAssign | Tokens::SubAssign | Tokens::MulAssign | Tokens::DivAssign | Tokens::ModAssign | Tokens::LeftShiftAssign | Tokens::RightShiftAssign | Tokens::AndAssign | Tokens::XorAssign | Tokens::OrAssign => 16,
            Tokens::Comma => 18,
            Tokens::OpenParen | Tokens::CloseParen => 0,
            _ => -1,
        }
    }

}
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tokens::Integer(n) => write!(f, "Integer({})", n),
            //Tokens::Float(n) => write!(f, "Float({})", n),
            //Tokens::Decimal(n) => write!(f, "Decimal({})", n),
            Tokens::Identifier(ref s) => write!(f, "{}", s),
            Tokens::QuotedString(ref s) => write!(f, "QuotedString({})", s),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

/*
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    And,
    Or,
    Not,
}
impl Operator {
    /// Returns a string representation of the operator.
    pub fn as_str(&self) -> &str {
        match *self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Asterisk => "*",
            Operator::Slash => "/",
            Operator::Percent => "%",
            Operator::Equals => "==",
            Operator::NotEquals => "!=",
            Operator::LessThan => "<",
            Operator::LessThanEquals => "<=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanEquals => ">=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Not => "!",
        }
    }
}*/


#[cfg(test)]
mod test {
    use crate::core::lexer::{Tokens,Operator,Logical,Assigns};
    #[test]
    fn token_display() {
        let tokens = vec![
            Tokens::Identifier("let".to_string()),
            Tokens::Identifier("x".to_string()),
            Tokens::Assigns(Assigns::Assign),
            Tokens::Integer(3),
            Tokens::Operator(Operator::Add),
            Tokens::Integer(4),
            Tokens::Semicolon,
        ];

        let expected = "let, x, Assigns(Assign), Integer(3), Operator(Add), Integer(4), ;";
        let result: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
        let result = result.join(", ");
        assert_eq!(expected, result);
    }
}
