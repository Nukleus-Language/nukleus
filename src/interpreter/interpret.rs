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
pub struct FunctionId {
    function_address: u32,
    mem_address: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId {
    function_address: u32,
    statement_address: u32,
    var_address: u32,
    mem_address: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    var_type: TypeName,
    var_value: TypeValue,
}

pub struct Interpreter {
    function_map: HashMap<String, FunctionId>,
    functions: HashMap<FunctionId, AST>,
    functions_address: u32,
    variables: HashMap<String, Token>,
    variables_address: u32,
    mem_address: u32,
    main_address: u32,
    main_mem_address: u32,
    cur_function: FunctionId,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            function_map: HashMap::new(),
            functions: HashMap::new(),
            functions_address: 0,
            variables: HashMap::new(),
            variables_address: 0,
            mem_address: 0,
            main_address: 0,
            main_mem_address: 0,
            cur_function: FunctionId {
                function_address: 0,
                mem_address: 0,
            },
        }
    }
    pub fn pre_run(&mut self, program: Vec<AST>) {
        let mut is_main = false;
        for func in program {
            if func.is_function() {
                self.function_map.insert(
                    func.function_get_name().to_string(),
                    FunctionId {
                        function_address: self.functions_address,
                        mem_address: self.mem_address,
                    },
                );
                //func.function.variables = HashMap::new();
                self.functions.insert(
                    FunctionId {
                        function_address: self.functions_address,
                        mem_address: self.mem_address,
                    },
                    func.clone(),
                );
                if func.function_get_name() == "main" {
                    is_main = true;
                    self.main_address = self.functions_address;
                    self.main_mem_address = self.mem_address;
                }
                self.functions_address += 1;
            }
        }
        if !is_main {
            panic!("No main function found");
        }
    }
    pub fn run(&mut self, program: Vec<AST>) {
        self.pre_run(program);
        let main_addr = self
            .function_map
            .get("main")
            .expect("No main function found");
        let main = self
            .functions
            .get(main_addr)
            .expect("No main function found");
        self.cur_function = main_addr.clone();
        self.run_function(main.function_get_statements(), vec![]);
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
                    self.run_function(ast, vec![]);
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

    fn run_function(&mut self, statements: Vec<AST>, _arguments: Vec<Token>) {
        for stmt in statements {
            match stmt {
                AST::Let { name, value, .. } => {
                    //self.variables.insert(name.clone(), lexer::Token::TypeValue(self.eval_expr(&value)));
                    let getten_value = self.eval_expr(&value.clone());

                    // check if the variable name is taken by a function
                    if self.function_map.contains_key(&name) {
                        panic!("Variable name '{}' is taken", name);
                    }

                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        name.clone(),
                        lexer::Token::TypeValue(getten_value),
                    );
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
                        let func = self
                            .functions
                            .get_mut(&self.cur_function)
                            .expect("Function not found");
                        func.function_insert_variable(
                            start.clone().to_string(),
                            Token::TypeValue(TypeValue::I32(i.try_into().unwrap())),
                        );
                        self.run_function(statements.clone(), vec![]);
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
                        Token::Logical(Logical::GreaterThanEquals) => {
                            left.as_i32() >= right.as_i32()
                        }
                        Token::Logical(Logical::LessThanEquals) => left.as_i32() <= right.as_i32(),
                        _ => panic!("Invalid logic operator"),
                    };
                    if condition {
                        self.run_function(statements, vec![]);
                    }
                }
                AST::Assign { l_var, r_var } => {
                    //let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((right.as_i32()).try_into().unwrap())),
                    );
                }
                AST::AddAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() + right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::SubAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() - right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::MulAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() * right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::DivAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() / right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::RemAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() % right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AST::FunctionCall { name, args } => {
                    let func_name = name.clone().to_string();
                    let target_func_addr = self
                        .function_map
                        .get(&func_name)
                        .expect("Function not found")
                        .clone();

                    let (target_statements, this_function, target_args) = {
                        let target_func = self
                            .functions
                            .get_mut(&target_func_addr)
                            .expect("Function not found");

                        let target_statements = target_func.function_get_statements().clone();
                        let this_function = self.cur_function.clone();
                        let target_args = target_func.function_get_args_format().clone();
                        (target_statements, this_function, target_args)
                    };
                    if target_args.len() != args.len() {
                        panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.eval_expr(arg).get_type();
                        let arg_type = Token::TypeName(arg_type);
                        if arg_type != target_args[i].0 {
                            panic!("Argument type mismatch");
                        }
                    }
                    self.cur_function = target_func_addr.clone();
                    self.run_function(target_statements, args.clone());
                    self.cur_function = this_function;
                    /*let return_value = {
                        let target_func = self
                        .functions
                        .get(&target_func_addr)
                        .expect("Function not found");
                        target_func.function_get_return_value().clone()
                    };
                    self.eval_expr(&return_value)
                                        // Check for argument count, and type
                    let target_args = target_func.function_get_args_format();
                    if target_args.len() != args.len() {
                        panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.eval_expr(arg).get_type();
                        let arg_type = Token::TypeName(arg_type);
                        if arg_type != target_args[i].0 {
                            panic!("Argument type mismatch");
                        }
                    }*/
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
                AST::Return { value } => {
                    let return_value = Token::TypeValue(self.eval_expr(&value));
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_set_return_value(return_value);
                    break;
                }
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&mut self, expr: &Token) -> TypeValue {
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
                    let func = self
                        .functions
                        .get(&self.cur_function)
                        .expect("Function not found");
                    if let value = func.function_get_variable(id.to_string()) {
                        if let Token::TypeValue(inner_value) = value {
                            match inner_value {
                                TypeValue::None => TypeValue::None,
                                TypeValue::I8(i) => TypeValue::I8(i),
                                TypeValue::I16(i) => TypeValue::I16(i),
                                TypeValue::I32(i) => TypeValue::I32(i),
                                TypeValue::I64(i) => TypeValue::I64(i),
                                TypeValue::U8(u) => TypeValue::U8(u),
                                TypeValue::U16(u) => TypeValue::U16(u),
                                TypeValue::U32(u) => TypeValue::U32(u),
                                TypeValue::U64(u) => TypeValue::U64(u),
                                TypeValue::QuotedString(s) => TypeValue::QuotedString(s),
                                TypeValue::Bool(b) => TypeValue::Bool(b),
                                TypeValue::Identifier(_) => {
                                    panic!("Invalid identifier reference")
                                }
                                TypeValue::FunctionCall(_, _) => {
                                    panic!("Invalid function call reference")
                                }
                            }
                        } else {
                            panic!("Invalid value type");
                        }
                    } else {
                        panic!("Undefined variable '{}'", id);
                    }
                }
                TypeValue::FunctionCall(id, args) => {
                    let func_name = id.clone();
                    let target_func_addr = self
                        .function_map
                        .get(&func_name)
                        .expect("Function not found")
                        .clone();

                    let (target_statements, this_function, target_args) = {
                        let target_func = self
                            .functions
                            .get_mut(&target_func_addr)
                            .expect("Function not found");

                        let target_statements = target_func.function_get_statements().clone();
                        let this_function = self.cur_function.clone();
                        let target_args = target_func.function_get_args_format().clone();
                        (target_statements, this_function, target_args)
                    };
                    if target_args.len() != args.len() {
                        panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.eval_expr(arg).get_type();
                        let arg_type = Token::TypeName(arg_type);
                        if arg_type != target_args[i].0 {
                            panic!("Argument type mismatch");
                        }
                    }

                    self.cur_function = target_func_addr.clone();
                    self.run_function(target_statements, args.clone());
                    self.cur_function = this_function;
                    let return_value = {
                        let target_func = self
                            .functions
                            .get(&target_func_addr)
                            .expect("Function not found");
                        target_func.function_get_return_value().clone()
                    };
                    self.eval_expr(&return_value)
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
