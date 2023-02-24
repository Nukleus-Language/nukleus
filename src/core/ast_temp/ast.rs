use crate::core::lexer::Tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        args: Vec<String>,
        statements: Vec<AST>,
        return_type: String,
    },

    Let {
        name: String,
        type_name: Option<String>,
        value: Box<AST>,
    },
    
    Return {
        value: Box<AST>,
    },
}
