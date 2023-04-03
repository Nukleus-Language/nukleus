use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    ShiftLeft,
    ShiftRight,
    
    BitAnd,
    BitOr,
    BitXor,
}
impl Operator {
    /// Returns a string representation of the operator.
    pub fn as_str(&self) -> &str {
        match *self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Remainder => "%",
            Operator::ShiftLeft => "<<",
            Operator::ShiftRight => ">>",
            Operator::BitAnd => "&",
            Operator::BitOr => "|",
            Operator::BitXor => "^",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Logical{
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    And,
    Or,
    Not,
}
impl Logical {
    /// Returns a string representation of the operator.
    pub fn as_str(&self) -> &str {
        match *self {
            Logical::Equals => "==",
            Logical::NotEquals => "!=",
            Logical::LessThan => "<",
            Logical::LessThanEquals => "<=",
            Logical::GreaterThan => ">",
            Logical::GreaterThanEquals => ">=",
            Logical::And => "&&",
            Logical::Or => "||",
            Logical::Not => "!",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Assign{
    // Default
    Assign,
    
    // Normal Operator Assigns
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    RemainderAssign,

    // Bitwise Operator Assigns
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}
impl Assign {
    /// Returns a string representation of the operator.
    pub fn as_str(&self) -> &str {
        match *self {
            Assigns::Assign => "=",
            Assigns::PlusAssign => "+=",
            Assigns::MinusAssign => "-=",
            Assigns::MultiplyAssign => "*=",
            Assigns::DivideAssign => "/=",
            Assigns::RemainderAssign => "%=",
            Assigns::BitAndAssign => "&=",
            Assigns::BitOrAssign => "|=",
            Assigns::BitXorAssign => "^=",
        }
    }
}
impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
