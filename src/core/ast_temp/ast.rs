use crate::core::lexer::Tokens;

#[derive(Debug)]
pub enum AST {
    // A leaf node representing a single token
    Token(Tokens),

    Import {
        path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        public: bool,
        name: String,
        parameters: Vec<String>,
        body: Box<AST>,
    },
    
    Let {
        name: String,
        type_name: Option<String>,
        value: Box<AST>,
    },
}
impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AST::Token(t1), AST::Token(t2)) => t1 == t2,
            (AST::Import { path: p1, name: n1 }, AST::Import { path: p2, name: n2 }) => p1 == p2 && n1 == n2,
            (AST::Function { public: p1, name: n1, parameters: par1, body: b1 }, AST::Function { public: p2, name: n2, parameters: par2, body: b2 }) => p1 == p2 && n1 == n2 && par1 == par2 && b1 == b2,
            (AST::Let { name: n1, type_name: tn1, value: v1 }, AST::Let { name: n2, type_name: tn2, value: v2 }) => n1 == n2 && tn1 == tn2 && v1 == v2,
            _ => false,
        }
    }
}