mod operators;
mod statements;
mod types;
mod symbols;

use std::fmt;

pub use operators::Operator;
pub use operators::Logical;
pub use operators::Assign;

pub use statements::Statement;

pub use types::TypeName;
//pub use types::TypeValue;

pub use symbols::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Operator(Operator),
    Logical(Logical),
    Assign(Assign),
    Statement(Statement),
    TypeName(TypeName),
    //TypeValue(TypeValue),
    Symbol(Symbol),
    None,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    QuotedString(String),
    Bool(bool),
    Identifier(String),
    
    EOF,
}
impl Token {
    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            _ => false,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(op) => write!(f, "{}", op),
            Token::Logical(log) => write!(f, "{}", log),
            Token::Assign(asn) => write!(f, "{}", asn),
            Token::Statement(st) => write!(f, "{}", st),
            Token::TypeName(tn) => write!(f, "{}", tn),
            //Token::TypeValue(tv) => write!(f, "{}", tv),
            Token::Symbol(sym) => write!(f, "{}", sym),
            Token::None => write!(f, "None"),
            Token::I8(i) => write!(f, "{}", i),
            Token::I16(i) => write!(f, "{}", i),
            Token::I32(i) => write!(f, "{}", i),
            Token::I64(i) => write!(f, "{}", i),
            Token::U8(u) => write!(f, "{}", u),
            Token::U16(u) => write!(f, "{}", u),
            Token::U32(u) => write!(f, "{}", u),
            Token::U64(u) => write!(f, "{}", u),
            Token::QuotedString(s) => write!(f, "{}", s),
            Token::Bool(b) => write!(f, "{}", b),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
