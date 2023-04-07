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
        args: Vec<Token>,
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
        l_var: Token,
        r_var: Token,
    },
    AddAssign {
        l_var: Token,
        r_var: Token,
    },
    SubAssign {
        l_var: Token,
        r_var: Token,
    },
    MulAssign {
        l_var: Token,
        r_var: Token,
    },
    DivAssign {
        l_var: Token,
        r_var: Token,
    },
    RemAssign {
        l_var: Token,
        r_var: Token,
    },
    If {
        l_var: Token,
        logic: Token,
        r_var: Token,
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
    FunctionCall {
        name: Token,
        args: Vec<Token>,
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
    pub fn function_get_args(&self) -> Vec<Token> {
        match self {
            AST::Function { args, .. } => args.clone(),
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
