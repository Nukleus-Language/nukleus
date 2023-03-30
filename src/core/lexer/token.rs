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

    Operator(Operator),
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
            Tokens::Operator(_) => "operator",
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
    pub fn is_identifier(&self) -> bool {
        match *self {
            Tokens::Identifier(_) => true,
            _ => false,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Statement {
    Let,
    Return,
    Print,
    Println,
    Break,
    If,
    Else,
    ElseIf,
    For,
    While,
    Function,
    Import,
}
impl Statement {
    /// Returns a string representation of the statement.
    pub fn as_str(&self) -> &str {
        match *self {
            Statement::Let => "let",
            Statement::Return => "return",
            Statement::Print => "print",
            Statement::Println => "println",
            Statement::Break => "break",
            Statement::If => "if",
            Statement::Else => "else",
            Statement::ElseIf => "else if",
            Statement::For => "for",
            Statement::While => "while",
            Statement::Function => "fn",
            Statement::Import => "import",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TypeName {
    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    String,
    Bool,
    Float,
}
impl TypeName {
    /// Returns a string representation of the type.
    pub fn as_str(&self) -> &str {
        match *self {
            TypeName::Void => "void",
            TypeName::I8 => "i8",
            TypeName::I16 => "i16",
            TypeName::I32 => "i32",
            TypeName::I64 => "i64",
            TypeName::U8 => "u8",
            TypeName::U16 => "u16",
            TypeName::U32 => "u32",
            TypeName::U64 => "u64",
            TypeName::String => "string",
            TypeName::Bool => "bool",
            TypeName::Float => "float",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TypeValue {
    None,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    String(String),
    Bool(bool),
    //Float(f64),
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

        let expected = "let, x, =, Integer(3), +, Integer(4), ;";
        let result: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
        let result = result.join(", ");
        assert_eq!(expected, result);
    }
}
