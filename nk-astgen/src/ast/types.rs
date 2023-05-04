/*pub enum ASTtype{
    TypeVoid
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16)
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    QuotedString(String),
    Array(Vec<ASTtype>),
    Identifier(String),
    Argument{
        type_name: ASTtypename,
        identifier: ASTtype,
    }
    FunctionCall{
        name: String,
        args: Vec<ASTtype>,
    },
}*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTtypename {
    TypeVoid,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    QuotedString,
    Array,
    Identifier,
    Argument,
    FunctionCall,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTtypevalue {
    TypeVoid,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    //F32(f32),
    //F64(f64),
    Bool(bool),
    QuotedString(String),
    Identifier(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTtypecomp {
    Array(Vec<ASTtypevalue>),
    Argument {
        type_name: ASTtypename,
        identifier: ASTtypevalue,
    },
    FunctionCall {
        name: String,
        args: Vec<ASTtypevalue>,
    },
}
