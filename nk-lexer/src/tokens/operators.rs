use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl Iterator for Operator {
    type Item = Operator;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Operator::Add => Some(Operator::Add),
            Operator::Subtract => Some(Operator::Subtract),
            Operator::Multiply => Some(Operator::Multiply),
            Operator::Divide => Some(Operator::Divide),
            Operator::Remainder => Some(Operator::Remainder),
            Operator::ShiftLeft => Some(Operator::ShiftLeft),
            Operator::ShiftRight => Some(Operator::ShiftRight),
            Operator::BitAnd => Some(Operator::BitAnd),
            Operator::BitOr => Some(Operator::BitOr),
            Operator::BitXor => Some(Operator::BitXor),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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
impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Iterator for Logical {
    type Item = Logical;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Logical::Equals => Some(Logical::Equals),
            Logical::NotEquals => Some(Logical::NotEquals),
            Logical::LessThan => Some(Logical::LessThan),
            Logical::LessThanEquals => Some(Logical::LessThanEquals),
            Logical::GreaterThan => Some(Logical::GreaterThan),
            Logical::GreaterThanEquals => Some(Logical::GreaterThanEquals),
            Logical::And => Some(Logical::And),
            Logical::Or => Some(Logical::Or),
            Logical::Not => Some(Logical::Not),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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
    /// Returns a string representation of the operator.
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

impl Iterator for Assign {
    type Item = Assign;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Assign::Assign => Some(Assign::Assign),
            Assign::AddAssign => Some(Assign::AddAssign),
            Assign::SubAssign => Some(Assign::SubAssign),
            Assign::MulAssign => Some(Assign::MulAssign),
            Assign::DivAssign => Some(Assign::DivAssign),
            Assign::RemAssign => Some(Assign::RemAssign),
            Assign::BitAndAssign => Some(Assign::BitAndAssign),
            Assign::BitOrAssign => Some(Assign::BitOrAssign),
            Assign::BitXorAssign => Some(Assign::BitXorAssign),
            _ => None,
        }
    }
}
