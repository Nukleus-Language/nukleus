use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
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
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum Logical {
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
    // Returns a string representation of the operator.
    #[allow(dead_code)]
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
impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum Assign {
    // Default
    Assign,

    // Normal Operator Assigns
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,

    // Bitwise Operator Assigns
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}
impl Assign {
    // Returns a string representation of the operator.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match *self {
            Assign::Assign => "=",
            Assign::AddAssign => "+=",
            Assign::SubAssign => "-=",
            Assign::MulAssign => "*=",
            Assign::DivAssign => "/=",
            Assign::RemAssign => "%=",
            Assign::BitAndAssign => "&=",
            Assign::BitOrAssign => "|=",
            Assign::BitXorAssign => "^=",
        }
    }
}
impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

