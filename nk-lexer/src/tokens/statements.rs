use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Statement {
    Public,
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
            Statement::Public => "public",
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
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
