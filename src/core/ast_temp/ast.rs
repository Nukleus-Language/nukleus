use lexer::Token;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    // A leaf node representing a single token
    Token(Token),

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
        return_type: Token,
        return_value: Token,
    },

    Let {
        name: String,
        type_name: Option<String>,
        value: Token,
    },
    Assign {
        name: String,
        value: Vec<Token>,
    },
    If {
        condition: Vec<Token>,
        statements: Vec<AST>,
        //else_if: Vec<AST>,
        //else_: Option<Box<AST>>,
    },
    ElseIf {
        condition: Vec<Token>,
        statements: Vec<AST>,
    },
    Else {
        statements: Vec<AST>,
    },

    For {
        start: Token,
        end: Token,
        value: Token,
        statements: Vec<AST>,
    },
    Print {
        value: Token,
    },
    Println {
        value: Token,
    },

    Return {
        value: Token,
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
