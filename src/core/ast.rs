#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VType {
    Unknown,
    Int,
    String,
    Set,
    Boolean,
    Queue,
    Stack,
    Array,
    LinkedList,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ops {
    Unknown,
    NotEqual,
    Equal,
    Leq,
    Geq,
    Less,
    Greater,
    And,
    Or,
    Xor,
    Add,
    Sub,
    Multply,
    Divide,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    Todo,
    Constant(Constant),
    NumericTerm(Box<Node>, Box<Node>, Ops),
    BooleanTerm(Box<Node>, Box<Node>, Ops),
    Block(Vec<Box<Node>>),
    Class(Box<Node>, Vec<Variable>, Box<Node>),
    If(Box<Node>, Box<Node>),
    Loop(Box<Node>, Box<Node>),
    Ident(String),
    Call(Box<Node>, Vec<Box<Node>>),
    Assign(Box<Node>, Box<Node>),
    Int(i64),
    Return,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Variable {
    pub name: Box<Node>,
    pub vtype: VType,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Constant {
    Unknown,
    Boolean(bool),
    Int(i64),
}
impl Ops {
    pub fn from_str(s: &str) -> Self {
        match s {
            "!=" => Self::NotEqual,
            "==" => Self::Equal,
            "<=" => Self::Leq,
            ">=" => Self::Geq,
            "<" => Self::Less,
            ">" => Self::Greater,
            "&&" | "&" => Self::And,
            "||" | "|" => Self::Or,
            "^" => Self::Xor,
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Divide,
            "*" => Self::Multply,
            _ => Self::Unknown,
        }
    }
}
impl Constant {
    pub fn parse_bool(source: &str) -> Self {
        match source {
            "true" => Self::Boolean(true),
            "false" => Self::Boolean(false),
            _ => Self::parse_numeric(source), // Failed
        }
    }

    pub fn parse_numeric(source: &str) -> Self {
        Self::Int(source.parse::<i64>().unwrap())
    }

    pub fn is_numeric(&self) -> bool {
        match *self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }
}
impl VType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "int" => VType::Int,
            "string" => VType::String,
            "boolean" => VType::Boolean,
            _ => VType::Unknown,
        }
    }
}
impl Variable {
    pub fn new(name: Box<Node>, vtype: VType) -> Self {
        Self { name, vtype }
    }
}
