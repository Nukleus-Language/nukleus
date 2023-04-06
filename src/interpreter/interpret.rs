use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};

use crate::core::ast_temp::AST;
use crate::core::parser_new::parse::Parser;

use lexer::Token;
use lexer::TypeName;
use lexer::TypeValue;
use lexer::Logical;

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
        println!("Nukleus 0.1.0 Nightly 2023-04-B1");
        loop {
            print!("> ");
            io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let tokens = lexer::lexer(&input);
            let ast = Parser::new(&tokens).parse_statements(Token::TypeName(TypeName::Void));      
            match ast {
                Ok(ast) => {
                    //println!("AST Tree: {:?}", ast);
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
                    let start_value = self.eval_expr(&start).as_i32();
                    let end_value = self.eval_expr(&end).as_i32();
                    let by_value = self.eval_expr(&value).as_i32().try_into().unwrap();
                    for i in (start_value..end_value).step_by(by_value) {
                        self.variables
                            .insert(start.clone().to_string(), Token::TypeValue(TypeValue::I32(i.try_into().unwrap())));
                        self.run_function(statements.clone());
                    }
                }
                AST::If {
                    l_var,
                    logic,
                    r_var,
                    statements,
                } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    match logic {
                        Token::Logical(Logical::And) => {
                            if left.as_bool() && right.as_bool() {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::Or) => {
                            if left.as_bool() || right.as_bool() {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::Equals) => {
                            if left == right{
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::NotEquals) => {
                            if left != right {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::GreaterThan) => {
                            if left.as_i32() > right.as_i32() {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::LessThan) => {
                            if left.as_i32() < right.as_i32() {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::GreaterThanEquals) => {
                            if left.as_i32() >= right.as_i32() {
                                self.run_function(statements);
                            }
                        }
                        Token::Logical(Logical::LessThanEquals) => {
                            if left.as_i32() <= right.as_i32() {
                                self.run_function(statements);
                            }
                        }
                        _ => panic!("Invalid logic operator"),
                    }
                }
                AST::Assign { name, value } => {
                    println!("Not implemented yet");
                }
                AST::AddAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((left.as_i32() + right.as_i32()).try_into().unwrap())),
                    );
                }
                AST::SubAssign {l_var, r_var} => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((left.as_i32() - right.as_i32()).try_into().unwrap())),
                    );
                }
                AST::MulAssign {l_var, r_var} => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((left.as_i32() * right.as_i32()).try_into().unwrap())),
                    );
                }
                AST::DivAssign {l_var, r_var} => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((left.as_i32() / right.as_i32()).try_into().unwrap())),
                    );
                }
                AST::RemAssign {l_var, r_var} => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((left.as_i32() % right.as_i32()).try_into().unwrap())),
                    );
                }
                
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&self, expr: &Token) -> TypeValue {
        match expr {
            Token::TypeValue(TypeValue::I32(i)) => TypeValue::I32(*i),
            Token::TypeValue(TypeValue::QuotedString(s)) => TypeValue::QuotedString(s.clone()),
            Token::TypeValue(TypeValue::Bool(b)) => TypeValue::Bool(*b),
            Token::TypeValue(TypeValue::Identifier(id)) => {
                if let Some(value) = self.variables.get(id) {
                    match value {
                        Token::TypeValue(TypeValue::I32(i)) => TypeValue::I32(*i),
                        Token::TypeValue(TypeValue::QuotedString(s)) => TypeValue::QuotedString(s.clone()),
                        Token::TypeValue(TypeValue::Bool(b)) => TypeValue::Bool(*b),
                        Token::TypeValue(TypeValue::Identifier(_)) => panic!("Invalid identifier reference"),
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
