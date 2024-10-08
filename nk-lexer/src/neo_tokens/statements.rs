use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum Statement {

    Public,
    //Import,
    Let,
    Return,
    Print,
    Println,
    Scanln,
    Break,
    If,
    Else,
    ElseIf,
    For,
    While,
    Function,
    Inject,

}
impl Statement {
    // Returns a string representation of the statement.
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match *self {

            Statement::Public => "public",
            //Statement::Import => "import",
            Statement::Let => "let",
            Statement::Return => "return",
            Statement::Print => "print",
            Statement::Println => "println",
            Statement::Scanln => "scanln",
            Statement::Break => "break",
            Statement::If => "if",
            Statement::Else => "else",
            Statement::ElseIf => "else if",
            Statement::For => "for",
            Statement::While => "while",
            Statement::Function => "fn",
            Statement::Inject => "inject",
        }
    }
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
