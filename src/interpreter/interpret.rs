use std::collections::HashMap;
use std::convert::TryInto;

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
                AST::For {
                    start,
                    end,
                    value,
                    statements,
                } => {
                    let start_value = self.eval_expr(&start).parse::<i32>().unwrap();
                    let end_value = self.eval_expr(&end).parse::<i32>().unwrap();
                    let by_value = self.eval_expr(&value).parse::<usize>().unwrap();
                    for i in (start_value..end_value).step_by(by_value) {
                        self.variables.insert(
                            start.clone().to_string(),
                            Tokens::Integer(i.try_into().unwrap()),
                        );
                        self.run_function(statements.clone());
                    }
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
