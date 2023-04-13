use super::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TypeValue {
    NoneVoid,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Number(String),
    QuotedString(String),
    Bool(bool),
    //Float(f64),
    Identifier(String),
    FunctionCall(String, Vec<Token>),
}
impl TypeValue {
    pub fn get_type(&self) -> TypeName {
        match *self {
            TypeValue::NoneVoid => TypeName::Void,
            TypeValue::I8(_) => TypeName::I8,
            TypeValue::I16(_) => TypeName::I16,
            TypeValue::I32(_) => TypeName::I32,
            TypeValue::I64(_) => TypeName::I64,
            TypeValue::U8(_) => TypeName::U8,
            TypeValue::U16(_) => TypeName::U16,
            TypeValue::U32(_) => TypeName::U32,
            TypeValue::U64(_) => TypeName::U64,
            TypeValue::QuotedString(_) => TypeName::QuotedString,
            TypeValue::Bool(_) => TypeName::Bool,
            //TypeValue::Number(_) => println!("Number"),
            _ => panic!("Type is not a valid type"),
        }
    }
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

    pub fn add(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::I8(a + b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::I16(a + b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::I32(a + b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::I64(a + b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::U8(a + b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::U16(a + b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::U32(a + b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::U64(a + b),
            _ => panic!("Invalid types for addition"),
        }
    }
    pub fn sub(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::I8(a - b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::I16(a - b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::I32(a - b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::I64(a - b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::U8(a - b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::U16(a - b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::U32(a - b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::U64(a - b),
            _ => panic!("Invalid types for subtraction"),
        }
    }
    pub fn mul(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::I8(a * b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::I16(a * b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::I32(a * b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::I64(a * b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::U8(a * b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::U16(a * b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::U32(a * b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::U64(a * b),
            _ => panic!("Invalid types for multiplication"),
        }
    }
    pub fn div(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::I8(a / b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::I16(a / b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::I32(a / b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::I64(a / b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::U8(a / b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::U16(a / b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::U32(a / b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::U64(a / b),
            _ => panic!("Invalid types for division"),
        }
    }
    pub fn rem(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::I8(a % b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::I16(a % b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::I32(a % b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::I64(a % b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::U8(a % b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::U16(a % b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::U32(a % b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::U64(a % b),
            _ => panic!("Invalid types for remainder"),
        }
    }
    pub fn eq(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a == b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a == b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a == b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a == b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a == b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a == b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a == b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a == b),
            (TypeValue::QuotedString(ref a), TypeValue::QuotedString(ref b)) => {
                TypeValue::Bool(a == b)
            }
            (TypeValue::Bool(a), TypeValue::Bool(b)) => TypeValue::Bool(a == b),
            _ => panic!("Invalid types for equality"),
        }
    }
    pub fn ne(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a != b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a != b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a != b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a != b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a != b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a != b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a != b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a != b),
            (TypeValue::QuotedString(ref a), TypeValue::QuotedString(ref b)) => {
                TypeValue::Bool(a != b)
            }
            (TypeValue::Bool(a), TypeValue::Bool(b)) => TypeValue::Bool(a != b),
            _ => panic!("Invalid types for inequality"),
        }
    }
    pub fn lt(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a < b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a < b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a < b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a < b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a < b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a < b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a < b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a < b),
            _ => panic!("Invalid types for less than"),
        }
    }
    pub fn le(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a <= b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a <= b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a <= b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a <= b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a <= b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a <= b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a <= b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a <= b),
            _ => panic!("Invalid types for less than or equal"),
        }
    }
    pub fn gt(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a > b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a > b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a > b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a > b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a > b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a > b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a > b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a > b),
            _ => panic!("Invalid types for greater than"),
        }
    }
    pub fn ge(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::I8(a), TypeValue::I8(b)) => TypeValue::Bool(a >= b),
            (TypeValue::I16(a), TypeValue::I16(b)) => TypeValue::Bool(a >= b),
            (TypeValue::I32(a), TypeValue::I32(b)) => TypeValue::Bool(a >= b),
            (TypeValue::I64(a), TypeValue::I64(b)) => TypeValue::Bool(a >= b),
            (TypeValue::U8(a), TypeValue::U8(b)) => TypeValue::Bool(a >= b),
            (TypeValue::U16(a), TypeValue::U16(b)) => TypeValue::Bool(a >= b),
            (TypeValue::U32(a), TypeValue::U32(b)) => TypeValue::Bool(a >= b),
            (TypeValue::U64(a), TypeValue::U64(b)) => TypeValue::Bool(a >= b),
            _ => panic!("Invalid types for greater than or equal"),
        }
    }
    pub fn and(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::Bool(a), TypeValue::Bool(b)) => TypeValue::Bool(*a && *b),
            _ => panic!("Invalid types for and"),
        }
    }
    pub fn or(&self, other: &TypeValue) -> TypeValue {
        match (self, other) {
            (TypeValue::Bool(a), TypeValue::Bool(b)) => TypeValue::Bool(*a || *b),
            _ => panic!("Invalid types for or"),
        }
    }
    pub fn not(&self) -> TypeValue {
        match *self {
            TypeValue::Bool(a) => TypeValue::Bool(!a),
            _ => panic!("Invalid types for not"),
        }
    }
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeValue::NoneVoid => write!(f, "None"),
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
            TypeValue::Number(ref s) => write!(f, "{}", s),
            TypeValue::FunctionCall(ref s, ref args) => {
                write!(f, "{}(", s)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            } //_ => write!(f, "{}", self.as_str()),
        }
    }
}
