use crate::ast::*;
use lexer::tokens_new::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTmemoryspace {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTstatement {
    Import {
        //path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        public: bool,
        name: String,
        args: Vec<ASTtypecomp>,
        statements: Vec<AST>,
        return_type: ASTtypename,
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
        condition: Vec<AST>,
        statements: Vec<AST>,
    },
    ElseIf {
        condition: Vec<AST>,
        statements: Vec<AST>,
    },
    Else {
        statements: Vec<AST>,
    },

    For {
        start: ASTtypevalue,
        end: ASTtypevalue,
        value: ASTtypevalue,
        statements: Vec<AST>,
    },
    Print {
        value: Box<AST>,
    },
    Println {
        value: Box<AST>,
    },
    Return {
        value: Token,
    },
}
