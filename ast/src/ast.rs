use lexer::Token;

use std::collections::HashMap;
use std::convert::AsMut;

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
        args: Vec<(Token, Token)>,
        statements: Vec<AST>,
        variables: HashMap<String, Token>,
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
/*impl<T> AsMut<T> for AST {
    fn as_mut(&mut self) -> &mut T {
        match self {
            AST::Function {
                public,
                name,
                args,
                statements,
                variables,
                return_type,
                return_value,
            } => {
                // Implement the conversion from AST to the generic type T
            }
            _ => panic!("Invalid AST variant"),
        }
    }
}*/
impl AsMut<HashMap<String, Token>> for AST {
    fn as_mut(&mut self) -> &mut HashMap<String, Token> {
        match self {
            AST::Function {
                public: _,
                name: _,
                args: _,
                statements: _,
                variables,
                return_type: _,
                return_value: _,
            } => variables,
            _ => panic!("Invalid AST variant"),
        }
    }
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
    pub fn function_get_args_format(&self) -> Vec<(Token, Token)> {
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
    pub fn function_set_return_value(&mut self, value: Token) {
        match self {
            AST::Function {
                public: _,
                name: _,
                args: _,
                statements: _,
                variables: _,
                return_type: _,
                return_value,
            } => {
                *return_value = value;
            }
            _ => panic!("Invalid Return Value"),
        }
    }
    pub fn function_get_return_value(&self) -> Token {
        match self {
            AST::Function {
                public: _,
                name: _,
                args: _,
                statements: _,
                variables: _,
                return_type: _,
                return_value,
            } => return_value.clone(),
            _ => panic!("Invalid Return Value"),
        }
    }
    pub fn function_insert_variable(&mut self, var_name: String, value: Token) {
        match self {
            AST::Function {
                public: _,
                name: _,
                args: _,
                statements: _,
                variables,
                return_type: _,
                return_value: _,
            } => {
                variables.insert(var_name, value);
                //.nsert(var_name, value);
            }
            _ => panic!("Invalid Variable Insertion"),
        }
    }
    pub fn function_get_variable(&self, var_name: String) -> Token {
        match self {
            AST::Function {
                public: _,
                name: _,
                args: _,
                statements: _,
                variables,
                return_type: _,
                return_value: _,
            } => {
                if variables.contains_key(&var_name) {
                    variables.get(&var_name).unwrap().clone()
                } else {
                    panic!("Variable not found");
                }
            }
            _ => panic!("Invalid Variable Reference"),
        }
    }
}

pub enum ASTtype{
    Void
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16)
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    QuotedString(String),
    Array(Vec<ASTtype>),
    Identifier(String),
}
pub enum ASTstatement{

}
pub enum ASTexpression{

}
