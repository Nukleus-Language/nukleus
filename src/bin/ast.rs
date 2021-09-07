#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AstNode {
    Todo,
    Constant(Constant),
    NumericTerm(Box<AstNode>, Box<AstNode>, Ops),
    BooleanTerm(Box<AstNode>, Box<AstNode>, Ops),
    Block(Vec<Box<AstNode>>),
    Func(Box<AstNode>, Vec<Variable>, Box<AstNode>),
    If(Box<AstNode>, Box<AstNode>),
    Loop(Box<AstNode>, Box<AstNode>),
    Ident(String),
    Call(Box<AstNode>, Vec<Box<AstNode>>),
    Assign(Box<AstNode>, Box<AstNode>),
    Return,
}
