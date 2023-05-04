use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
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
    Comment,
    Arrow,
    Semicolon,
}
impl Symbol {
    #[allow(dead_code)]
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
            Symbol::Comment => "//",
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

