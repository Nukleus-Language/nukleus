use crate::core::lexer::Tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    // A leaf node representing a single token
    Token(Tokens),

    Import {
        //path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        public: bool,
        name: String,
        args: Vec<AST>,
        statements: Vec<AST>,
        return_type: Tokens,
        return_value: Tokens,
    },

    Let {
        name: String,
        type_name: Option<String>,
        value: Tokens,
    },
    Assign {
        name: String,
        value: Vec<Tokens>,
    },
    If {
        condition: Vec<Tokens>,
        statements: Vec<AST>,
        //else_if: Vec<AST>,
        //else_: Option<Box<AST>>,
    },
    ElseIf {
        condition: Vec<Tokens>,
        statements: Vec<AST>,
    },
    Else {
        statements: Vec<AST>,
    },

    For {
        start: Tokens,
        end: Tokens,
        value: Tokens,
        statements: Vec<AST>,
    },
    Print {
        value: Tokens,
    },
    Println {
        value: Tokens,
    },

    Return {
        value: Tokens,
    },
}
impl AST {
    pub fn is_function(&self) -> bool {
        match self {
            AST::Function { .. } => true,
            _ => false,
        }
    }
    pub fn function_get_statements(&self) -> Vec<AST> {
        match self {
            AST::Function { statements, .. } => statements.clone(),
            _ => panic!("Not a function"),
        }
    }
    pub fn function_get_name(&self) -> String {
        match self {
            AST::Function { name, .. } => name.clone(),
            _ => panic!("Not a function"),
        }
    }
}
