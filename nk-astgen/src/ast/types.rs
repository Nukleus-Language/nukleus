use std::fmt;
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
impl fmt::Display for ASTtypename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTtypename::TypeVoid => write!(f, "TypeVoid"),
            ASTtypename::I8 => write!(f, "I8"),
            ASTtypename::I16 => write!(f, "I16"),
            ASTtypename::I32 => write!(f, "I32"),
            ASTtypename::I64 => write!(f, "I64"),
            ASTtypename::U8 => write!(f, "U8"),
            ASTtypename::U16 => write!(f, "U16"),
            ASTtypename::U32 => write!(f, "U32"),
            ASTtypename::U64 => write!(f, "U64"),
            ASTtypename::F32 => write!(f, "F32"),
            ASTtypename::F64 => write!(f, "F64"),
            ASTtypename::Bool => write!(f, "Bool"),
            ASTtypename::QuotedString => write!(f, "QuotedString"),
            ASTtypename::Array => write!(f, "Array"),
            ASTtypename::Identifier => write!(f, "Identifier"),
            ASTtypename::Argument => write!(f, "Argument"),
            ASTtypename::FunctionCall => write!(f, "FunctionCall"),
        }
    }
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
impl fmt::Display for ASTtypevalue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTtypevalue::TypeVoid => write!(f, "TypeVoid"),
            ASTtypevalue::I8(val) => write!(f, "{}", val),
            ASTtypevalue::I16(val) => write!(f, "{}", val),
            ASTtypevalue::I32(val) => write!(f, "{}", val),
            ASTtypevalue::I64(val) => write!(f, "{}", val),
            ASTtypevalue::U8(val) => write!(f, "{}", val),
            ASTtypevalue::U16(val) => write!(f, "{}", val),
            ASTtypevalue::U32(val) => write!(f, "{}", val),
            ASTtypevalue::U64(val) => write!(f, "{}", val),
            ASTtypevalue::Bool(val) => write!(f, "{}", val),
            ASTtypevalue::QuotedString(val) => write!(f, "{}", val),
            ASTtypevalue::Identifier(val) => write!(f, "{}", val),
        }
    }
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
impl fmt::Display for ASTtypecomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTtypecomp::Array(val) => write!(
                f,
                "[{}]",
                val.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ASTtypecomp::Argument {
                type_name,
                identifier,
            } => write!(f, "{} {}", type_name, identifier),
            ASTtypecomp::FunctionCall { name, args } => write!(
                f,
                "{}({})",
                name,
                args.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
