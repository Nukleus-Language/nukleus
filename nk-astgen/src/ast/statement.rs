
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#![allow(missing_docs)]
#![allow(dead_code)]
pub enum ASTstatement{
    Import {
        //path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        public: bool,
        name: String,
        args: Vec<(Token, Token)>,
        statements: Vec<AST>,
        return_type: Token,
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
impl ASTstatement{
    pub get_mem_template
}
