use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
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
    QuotedString,
    Bool,
    Float,
    Number,
}
impl TypeName {
    // Returns a string representation of the type.
    #[allow(dead_code)]
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
            TypeName::QuotedString => "string",
            TypeName::Bool => "bool",
            TypeName::Float => "float",
            TypeName::Number => "number",
        }
    }
}
impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum TypeValue {
    NoneVoid,
    Number(String),
    QuotedString(String),
    Bool(bool),
    //Float(f64),
    Identifier(String),
}
impl TypeValue {
    // Returns a string representation of the type.
    #[allow(dead_code)]
    pub fn get_type(&self) -> TypeName {
        match *self {
            TypeValue::NoneVoid => TypeName::Void,
            TypeValue::QuotedString(_) => TypeName::QuotedString,
            TypeValue::Bool(_) => TypeName::Bool,
            TypeValue::Number(_) => TypeName::Number,
            _ => panic!("Type is not a valid type"),
        }
    }
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::NoneVoid => write!(f, "None"),
            TypeValue::QuotedString(ref s) => write!(f, "{}", s),
            //TypeValue::Float(n) => write!(f, "Float({})", n),
            TypeValue::Identifier(ref s) => write!(f, "{}", s),
            TypeValue::Bool(b) => write!(f, "{}", b),
            TypeValue::Number(ref s) => write!(f, "{}", s),
        }
    }
}
