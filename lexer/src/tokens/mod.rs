mod operators;
mod statements;
mod types;
mod symbols;

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
