use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Symbol {
    Colon,
    Comma,
    DoubleColon,
    Dot,
    At,
    OpenParen,
    OpenBrace,
    //OpenAngle,
    OpenSquare,
    CloseParen,
    CloseBrace,
    //CloseAngle,
    CloseSquare,
    Arrow,
    Semicolon,
}
impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::Colon => ":",
            Symbol::Comma => ",",
            Symbol::DoubleColon => "::",
            Symbol::Dot => ".",
            Symbol::At => "@",
            Symbol::OpenParen => "(",
            Symbol::OpenBrace => "{",
            //Symbol::OpenAngle => "<",
            Symbol::OpenSquare => "[",
            Symbol::CloseParen => ")",
            Symbol::CloseBrace => "}",
            //Symbol::CloseAngle => ">",
            Symbol::CloseSquare => "]",
            Symbol::Arrow => "->",
            Symbol::Semicolon => ";",
        }
    }
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
