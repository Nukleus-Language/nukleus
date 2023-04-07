use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};

use crate::core::ast::AST;
use crate::core::parser::parse::Parser;

use lexer::Logical;
use lexer::Token;
use lexer::TypeName;
use lexer::TypeValue;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId {
    function_address: u32,
    statement_address: u32,
    var_address: u32,
    mem_address: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    var_type: TypeName,
    var_value: TypeValue,
}


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
    pub fn run_repl(&mut self) {
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
    /*fn update_variable<T: FnOnce(&TypeValue, &TypeValue) -> TypeValue>(
        &mut self,
        l_var: &Tokens,
        r_var: &Tokens,
        operation: T,
    ) {
        let right = self.eval_expr(r_var);
        let left = self.variables
            .get(l_var)
            .expect(&format!("Undefined variable '{}'", l_var));

        let new_value = operation(left, &right);
        self.variables.insert(l_var.to_string(), Token::TypeValue(new_value));
    }*/

    fn run_function(&mut self, statements: Vec<AST>) {
        for stmt in statements {
            match stmt {
                AST::Let { name, value, .. } => {
                    self.variables.insert(name.clone(), lexer::Token::TypeValue(self.eval_expr(&value)));
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
                        self.variables.insert(
                            start.clone().to_string(),
                            Token::TypeValue(TypeValue::I32(i.try_into().unwrap())),
                        );
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
                    let condition = match logic {
                        Token::Logical(Logical::And) => left.as_bool() && right.as_bool(),
                        Token::Logical(Logical::Or) => left.as_bool() || right.as_bool(),
                        Token::Logical(Logical::Equals) => left == right,
                        Token::Logical(Logical::NotEquals) => left != right,
                        Token::Logical(Logical::GreaterThan) => left.as_i32() > right.as_i32(),
                        Token::Logical(Logical::LessThan) => left.as_i32() < right.as_i32(),
                        Token::Logical(Logical::GreaterThanEquals) => left.as_i32() >= right.as_i32(),
                        Token::Logical(Logical::LessThanEquals) => left.as_i32() <= right.as_i32(),
                        _ => panic!("Invalid logic operator"),
                    };
                    if condition {
                        self.run_function(statements);
                    }   
                }
                AST::Assign { l_var, r_var } => {
                    //let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::AddAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() + right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::SubAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() - right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::MulAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() * right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::DivAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() / right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::RemAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    self.variables.insert(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() % right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                /*
                AST::Assign { l_var, r_var } => {
                    self.update_variable(l_var, r_var, |_, right| right.clone());
                },
                AST::AddAssign { l_var, r_var } => {
                    self.update_variable(l_var, r_var, |left, right| left.add(right));
                },
                AST::SubAssign { l_var, r_var } => {
                   self.update_variable(l_var, r_var, |left, right| left.sub(right));
                },
                AST::MulAssign { l_var, r_var } => {
                    self.update_variable(l_var, r_var, |left, right| left.mul(right));
                },
                AST::DivAssign { l_var, r_var } => {
                    self.update_variable(l_var, r_var, |left, right| left.div(right));
                },
                AST::RemAssign { l_var, r_var } => {
                    self.update_variable(l_var, r_var, |left, right| left.rem(right));
                },
                */
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&self, expr: &Token) -> TypeValue {
        if let Token::TypeValue(inner_expr) = expr {
            match inner_expr {
                TypeValue::None => TypeValue::None,
                TypeValue::I8(i) => TypeValue::I8(*i),
                TypeValue::I16(i) => TypeValue::I16(*i),
                TypeValue::I32(i) => TypeValue::I32(*i),
                TypeValue::I64(i) => TypeValue::I64(*i),
                TypeValue::U8(u) => TypeValue::U8(*u),
                TypeValue::U16(u) => TypeValue::U16(*u),
                TypeValue::U32(u) => TypeValue::U32(*u),
                TypeValue::U64(u) => TypeValue::U64(*u),
                TypeValue::QuotedString(s) => TypeValue::QuotedString(s.clone()),
                TypeValue::Bool(b) => TypeValue::Bool(*b),
                TypeValue::Identifier(id) => {
                    if let Some(value) = self.variables.get(id) {
                        if let Token::TypeValue(inner_value) = value {
                            match inner_value {
                                TypeValue::None => TypeValue::None,
                                TypeValue::I8(i) => TypeValue::I8(*i),
                                TypeValue::I16(i) => TypeValue::I16(*i),
                                TypeValue::I32(i) => TypeValue::I32(*i),
                                TypeValue::I64(i) => TypeValue::I64(*i),
                                TypeValue::U8(u) => TypeValue::U8(*u),
                                TypeValue::U16(u) => TypeValue::U16(*u),
                                TypeValue::U32(u) => TypeValue::U32(*u),
                                TypeValue::U64(u) => TypeValue::U64(*u),
                                TypeValue::QuotedString(s) => TypeValue::QuotedString(s.clone()),
                                TypeValue::Bool(b) => TypeValue::Bool(*b),
                                TypeValue::Identifier(_) => {
                                    panic!("Invalid identifier reference")
                                }
                            }
                        } else {
                            panic!("Invalid value type");
                        }
                    } else {
                        panic!("Undefined variable '{}'", id);
                    }
                }
            }
        } else {
            panic!("Invalid expression");
        }
    }

    // can evaluate conditions like i <10 && i > j
    //fn eval_cond(&self, cond: Vec<Tokens>) -> bool {}
    //fn eval_operater
}
