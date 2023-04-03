use std::fmt;

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
    QuoatedString,
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
            TypeName::QuoatedString => "string",
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
    QuoatedString(String),
    Bool(bool),
    //Float(f64),
    Identifier(String),
}
impl TypeValue{
    fn as_str(&self) -> &str {
        match *self {
            TypeValue::None => "None",
            TypeValue::I8(n) => "I8",
            TypeValue::I16(n) => "I16",
            TypeValue::I32(n) => "I32",
            TypeValue::I64(n) => "I64",
            TypeValue::U8(n) => "U8",
            TypeValue::U16(n) => "U16",
            TypeValue::U32(n) => "U32",
            TypeValue::U64(n) => "U64",
            TypeValue::QuoatedString(ref s) => "QuotedString",
            TypeValue::Bool(n) => "Bool",
            //TypeValue::Float(n) => "Float",
            TypeValue::Identifier(ref s) => "Identifier",
        }
    }
}
impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::I8(n) => write!(f, "I8({})", n),
            TypeValue::I16(n) => write!(f, "I16({})", n),
            TypeValue::I32(n) => write!(f, "I32({})", n),
            TypeValue::I64(n) => write!(f, "I64({})", n),
            TypeValue::U8(n) => write!(f, "U8({})", n),
            TypeValue::U16(n) => write!(f, "U16({})", n),
            TypeValue::U32(n) => write!(f, "U32({})", n),
            TypeValue::U64(n) => write!(f, "U64({})", n),
            TypeValue::QuoatedString(ref s) => write!(f, "QuotedString({})", s),
            TypeValue::Bool(n) => write!(f, "Bool({})", n),
            //TypeValue::Float(n) => write!(f, "Float({})", n),
            TypeValue::Identifier(ref s) => write!(f, "Identifier({})", s),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}
