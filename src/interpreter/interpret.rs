use std::collections::HashMap;

use crate::core::ast_temp::AST;
use crate::core::lexer::Tokens;

pub struct Interpreter {
    variables: HashMap<String, Tokens>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Vec<AST>) {
        for func in program {
            if func.is_function() && func.function_get_name() == "main" {
                self.run_function(func.function_get_statements());
            }
        }
    }

    fn run_function(&mut self, statements: Vec<AST>) {
        for stmt in statements {
            match stmt {
                AST::Let { name, value, .. } => {
                    self.variables.insert(name.clone(), value.clone());
                }
                AST::Print { value } => {
                    print!("{}", self.eval_expr(&value));
                }
                AST::Println { value } => {
                    println!("{}", self.eval_expr(&value));
                }
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&self, expr: &Tokens) -> String {
        match expr {
            Tokens::Integer(i) => i.to_string(),
            Tokens::QuotedString(s) => s.clone(),
            Tokens::Identifier(id) => {
                if let Some(value) = self.variables.get(id) {
                    match value {
                        Tokens::Integer(i) => i.to_string(),
                        Tokens::QuotedString(s) => s.clone(),
                        Tokens::Identifier(_) => panic!("Invalid identifier reference"),
                        _ => panic!("Invalid value type"),
                    }
                } else {
                    panic!("Undefined variable '{}'", id);
                }
            }
            _ => panic!("Invalid expression"),
        }
    }
}
