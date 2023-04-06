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
    QuotedString,
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
            TypeName::QuotedString => "string",
            TypeName::Bool => "bool",
            TypeName::Float => "float",
        }
    }
}
impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
    QuotedString(String),
    Bool(bool),
    //Float(f64),
    Identifier(String),
}
impl TypeValue {
    pub fn as_i8(&self) -> i8 {
        match *self {
            TypeValue::I8(n) => n,
            _ => panic!("Type is not an i8"),
        }
    }
    pub fn as_i16(&self) -> i16 {
        match *self {
            TypeValue::I16(n) => n,
            _ => panic!("Type is not an i16"),
        }
    }
    pub fn as_i32(&self) -> i32 {
        match *self {
            TypeValue::I32(n) => n,
            _ => panic!("Type is not an i32"),
        }
    }
    pub fn as_i64(&self) -> i64 {
        match *self {
            TypeValue::I64(n) => n,
            _ => panic!("Type is not an i64"),
        }
    }
    pub fn as_u8(&self) -> u8 {
        match *self {
            TypeValue::U8(n) => n,
            _ => panic!("Type is not an u8"),
        }
    }
    pub fn as_u16(&self) -> u16 {
        match *self {
            TypeValue::U16(n) => n,
            _ => panic!("Type is not an u16"),
        }
    }
    pub fn as_u32(&self) -> u32 {
        match *self {
            TypeValue::U32(n) => n,
            _ => panic!("Type is not an u32"),
        }
    }
    pub fn as_u64(&self) -> u64 {
        match *self {
            TypeValue::U64(n) => n,
            _ => panic!("Type is not an u64"),
        }
    }
    pub fn as_string(&self) -> String {
        match *self {
            TypeValue::QuotedString(ref s) => s.clone(),
            _ => panic!("Type is not a string"),
        }
    }
    pub fn as_bool(&self) -> bool {
        match *self {
            TypeValue::Bool(n) => n,
            _ => panic!("Type is not a bool"),
        }
    }
    pub fn as_identifier(&self) -> String {
        match *self {
            TypeValue::Identifier(ref s) => s.clone(),
            _ => panic!("Type is not an identifier"),
        }
    }
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::None => write!(f, "None"),
            TypeValue::I8(n) => write!(f, "{}", n),
            TypeValue::I16(n) => write!(f, "{}", n),
            TypeValue::I32(n) => write!(f, "{}", n),
            TypeValue::I64(n) => write!(f, "{}", n),
            TypeValue::U8(n) => write!(f, "{}", n),
            TypeValue::U16(n) => write!(f, "{}", n),
            TypeValue::U32(n) => write!(f, "{}", n),
            TypeValue::U64(n) => write!(f, "{}", n),
            TypeValue::QuotedString(ref s) => write!(f, "{}", s),
            TypeValue::Bool(n) => write!(f, "{}", n),
            //TypeValue::Float(n) => write!(f, "Float({})", n),
            TypeValue::Identifier(ref s) => write!(f, "{}", s),
            //_ => write!(f, "{}", self.as_str()),
        }
    }
}
