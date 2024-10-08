use std::fmt;

use crate::ast::*;

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
        type_name: Option<ASTtypename>,
        value: Option<Box<AST>>,
    },
    Assignment {
        left: Box<AST>,
        op: ASTOperator,
        right: Box<AST>,
    },
    If {
        condition: Box<AST>,
        statements: Vec<AST>,
        elif: Option<Box<AST>>,
        else_statements: Option<Vec<AST>>,
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
        args: Vec<AST>,
    },
    Println {
        value: Box<AST>,
        args: Vec<AST>,
    },
    Return {
        value: Box<AST>,
    },
}
impl fmt::Display for ASTstatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTstatement::Import { name } => write!(f, "INJECT {}", name),
            ASTstatement::Function {
                public,
                name,
                args,
                statements,
                return_type,
            } => {
                let args_string = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let statements_string = statements
                    .iter()
                    .map(|statement| statement.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                let pub_eval = if *public { "public " } else { "" };
                write!(
                    f,
                    "{}function {}({})  {}\n{{\n{}\n}}",
                    pub_eval, name, args_string, return_type, statements_string
                )
            }
            ASTstatement::Let {
                name,
                type_name,
                value,
            } => {
                write!(
                    f,
                    "let {} : {} = {}",
                    name,
                    (*type_name).unwrap(),
                    value.clone().unwrap()
                )
            }
            ASTstatement::Assignment { left, op, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
            ASTstatement::If {
                condition,
                statements,
                elif,
                else_statements,
            } => {
                write!(
                    f,
                    "if {} {{\n{}\n}}else{}\n else {{\n{:?}\n}}",
                    condition,
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                    elif.clone()
                        .unwrap_or_else(|| Box::new(AST::TypeValue(ASTtypevalue::TypeVoid))),
                    else_statements
                        .clone()
                        .unwrap_or_else(std::vec::Vec::new)
                        .iter()
                )
            }
            ASTstatement::ElseIf {
                condition,
                statements,
            } => {
                write!(
                    f,
                    "else if {} {{\n{}\n}}",
                    condition
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::Else { statements } => {
                write!(
                    f,
                    "else {{\n{}\n}}",
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::For {
                start,
                end,
                value,
                statements,
            } => {
                write!(
                    f,
                    "for {} {} {} {{\n{}\n}}",
                    start,
                    end,
                    value,
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::Print { value, args } => {
                write!(f, "print {}", value);
                for arg in args {
                    write!(f, " {}", arg);
                }
                Ok(())
            }
            ASTstatement::Println { value, args } => {
                write!(f, "println {}", value);
                for arg in args {
                    write!(f, " {}", arg);
                }
                write!(f, "\n");
                Ok(())
            }
            ASTstatement::Return { value } => write!(f, "return {}", value),
        }
    }
}
