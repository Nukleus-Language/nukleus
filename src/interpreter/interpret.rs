use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};

use crate::core::ast_temp::AST;
use crate::core::parser_new::parse::Parser;
use lexer::Token;
use lexer::TypeName;

pub struct Interpreter {
    variables: HashMap<String, Token>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Vec<AST>) {
        let mut is_main = false;
        for func in program {
            if func.is_function() && func.function_get_name() == "main" {
                self.run_function(func.function_get_statements());
                is_main = true;
            }
        }
        if !is_main {
            panic!("No main function found");
        }
    }
    pub fn run_repl(&mut self){
        println!("Nukleus 0.1.0 Nightly 2023-04");
        loop {
            print!("> ");
            io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let tokens = lexer::lexer(&input);
            let ast = Parser::new(&tokens).parse_statements(Token::TypeName(TypeName::Void));      
            match ast {
                Ok(ast) => {
                    println!("AST Tree: {:?}", ast);
                    self.run_function(ast);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
                _ => println!("Unknown Error occured"),
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
                        self.variables
                            .insert(start.clone().to_string(), Token::I32(i.try_into().unwrap()));
                        self.run_function(statements.clone());
                    }
                }
                AST::If {
                    condition,
                    statements,
                } => {
                    println!("Condition: {:?}", condition);
                    println!("Statements: {:?}", statements);
                    println!("Not implemented yet");
                }
                AST::Assign { name: _, value: _ } => {
                    println!("Not implemented yet");
                }
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&self, expr: &Token) -> String {
        match expr {
            Token::I32(i) => i.to_string(),
            Token::QuotedString(s) => s.clone(),
            Token::Identifier(id) => {
                if let Some(value) = self.variables.get(id) {
                    match value {
                        Token::I32(i) => i.to_string(),
                        Token::QuotedString(s) => s.clone(),
                        Token::Identifier(_) => panic!("Invalid identifier reference"),
                        _ => panic!("Invalid value type"),
                    }
                } else {
                    panic!("Undefined variable '{}'", id);
                }
            }
            _ => panic!("Invalid expression"),
        }
    }
    // can evaluate conditions like i <10 && i > j
    //fn eval_cond(&self, cond: Vec<Tokens>) -> bool {}
    //fn eval_operater
}
