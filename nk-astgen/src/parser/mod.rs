use lexer::tokens_new::*;

mod error;
use crate::ast::*;
use error::{AstError, AstGenError};

use std::collections::HashMap;
use std::path::PathBuf;
use std::iter::{Cloned, Peekable};

#[derive(Debug, Clone, PartialEq)]
enum State {
    EmptyState,
    DefaultState,
    PublicFunction,
    Function,
    Inject,
    GlobalLet,
}

#[derive(Debug, Clone, PartialEq)]
enum ArgumentParseState {
    WaitForType,
    WaitForColon,
    WaitForIdentifier,
    WaitForCommaOrCloseParen,
}
pub struct Parser<'a> {
    tokens: Peekable<Cloned<std::slice::Iter<'a, Token>>>,
    state: State,
    brace_inner: usize,
    asts: Vec<AST>,
    buffer: Vec<Token>,
    file_path: PathBuf,
    source: &'a str,
}

impl<'a> Parser<'a> {
    #[allow(dead_code)]
    pub fn new(tokens: &'a [Token],file_path: PathBuf, code: &'a str) -> Self {
        let peeked = tokens.iter().cloned().peekable();
        // println!("{:?}", tokens);
        Parser {
            tokens: peeked,
            state: State::EmptyState,
            brace_inner: 0,
            asts: Vec::new(),
            buffer: Vec::new(),
            file_path,
            source: code,
        }
    }
    #[allow(dead_code)]
    fn next_token(&mut self) -> Token {
        let token = self.tokens.next();
        // println!("{} Next Token: {:?}{}", "\x1b[36m", token, "\x1b[0m");
        match token {
            Some(t) => t,
            None => Token::new(TokenType::EOF, TokenMetadata::default()),
        }
    }
    #[allow(dead_code)]
    fn peek_token(&mut self) -> Token {
        let peek = self.tokens.peek();
        // println!("{} Peek Token: {:?}{}", "\x1b[38m", peek, "\x1b[0m");
        match peek {
            Some(t) => t.clone(),
            None => Token::new(TokenType::EOF, TokenMetadata::default()),
        }
    }
    #[allow(dead_code)]
    fn expect(&mut self, _current: Token, expected: Token) -> Result<(), AstGenError> {
        // println!("{} Current Token: {:?}{}", "\x1b[37m", current, "\x1b[0m");
        match expected.token_type {
            _cur_token => Ok(()),
            TokenType::EOF => Err(AstGenError {
                message: AstError::UnexpectedEOF(),
            }),
            _ => Err(AstGenError {
                message: AstError::ExpectedToken(expected),
            }),
        }
    }
    fn parse_statement(&mut self) -> Vec<AST> {
        let mut statements: Vec<AST> = Vec::new();
        // parse statements
        while let token = self.next_token() {
            match token.token_type {
                TokenType::Statement(Statement::Let) => {
                    statements.push(self.parse_let());
                }
                TokenType::Statement(Statement::For) => {
                    statements.push(self.parse_for());
                }
                TokenType::Statement(Statement::Print) => {
                    statements.push(self.parse_print());
                }
                TokenType::Statement(Statement::Println) => {
                    statements.push(self.parse_println());
                }
                TokenType::Statement(Statement::If) => {
                    statements.push(self.parse_if());
                }
                TokenType::Statement(Statement::Return) => {
                    statements.push(self.parse_return());
                }
                TokenType::TypeValue(TypeValue::Identifier(ident)) => {
                    if let TokenType::Assign(op) = self.peek_token().token_type {
                        match op {
                            Assign::Assign
                            | Assign::AddAssign
                            | Assign::SubAssign
                            | Assign::MulAssign
                            | Assign::DivAssign
                            | Assign::RemAssign
                            | Assign::BitAndAssign
                            | Assign::BitOrAssign
                            | Assign::BitXorAssign => {
                                statements.push(self.parse_assignment(ident));
                            }
                        }
                    }
                }

                TokenType::Symbol(Symbol::OpenBrace) => {
                    continue;
                }
                TokenType::Symbol(Symbol::CloseBrace) => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }

        statements
    }
    fn report_error(&self, error: AstGenError) {
        panic!("{}", error);
    }
    pub fn run(&mut self) {
        //println!("{:?}", self.tokens.peek());
        while let token = self.next_token() {
            //println!("{:?}", token);
            let peeked = self.peek_token();
            if self.state == State::EmptyState {
                //println!("{:?}", token);
                match token.token_type {
                    TokenType::Statement(Statement::Function) => {
                        self.parse_function(false);
                    }
                    TokenType::Statement(Statement::Public) => {
                        self.parse_function(true);
                    }
                    TokenType::Statement(Statement::Let) => {
                        self.state = State::GlobalLet;
                        //println!("Founded Global Let");
                    }
                    TokenType::Statement(Statement::Inject) => {
                        self.state = State::Inject;
                        //println!("Founded Import");
                    }

                    TokenType::EOF => {
                        break;
                    }
                    _ => {
                        panic!(
                            "{}",
                            AstGenError {
                                message: AstError::ExpectedStatement(),
                            }
                        );
                    }
                }
            }
            match self.state {
                State::Inject => {
                    //self.buffer.push(token);
                    if peeked.token_type == TokenType::Symbol(Symbol::Semicolon) {
                        self.state = State::EmptyState;
                        //self.buffer.push(peeked);
                        //parse_import
                        //println!("{} Inject: {:?} {}","\x1b[32m" , self.buffer,"\x1b[0m");
                        //self.buffer.clear();
                        self.next_token();
                    }
                }
                State::GlobalLet => {
                    //self.buffer.push(token);
                    if peeked.token_type == TokenType::Symbol(Symbol::Semicolon) {
                        self.state = State::EmptyState;
                        //self.buffer.push(peeked);
                        //parse_let
                        //println!("{} Let: {:?} {}","\x1b[31m" , self.buffer,"\x1b[0m");
                        //self.buffer.clear();
                        self.next_token();
                    }
                }

                _ => {
                    continue;
                }
            }
        }
    }
    fn parse_function(&mut self, is_public: bool) {
        //println!("Brace: {}", self.brace_inner);
        let type_map: HashMap<TypeName, ASTtypename> = [
            (TypeName::Void, ASTtypename::TypeVoid),
            (TypeName::I8, ASTtypename::I8),
            (TypeName::I16, ASTtypename::I16),
            (TypeName::I32, ASTtypename::I32),
            (TypeName::I64, ASTtypename::I64),
            (TypeName::U8, ASTtypename::U8),
            (TypeName::U16, ASTtypename::U16),
            (TypeName::U32, ASTtypename::U32),
            (TypeName::U64, ASTtypename::U64),
            (TypeName::Bool, ASTtypename::Bool),
            (TypeName::QuotedString, ASTtypename::QuotedString),
        ]
        .iter()
        .cloned()
        .collect();

        let mut cur_token = self.next_token();
        if is_public {
            cur_token = self.next_token();
        }
        // let args: Vec<ASTtypecomp> = Vec::new();

        // parse arguments and function header
        //println!("{} Start of Function: {:?} {}", "\x1b[34m", cur_token,"\x1b[0m");
        let function_name = cur_token.to_string();

        //println!("{} Function name: {:?} {}", "\x1b[34m", function_name,"\x1b[0m");

        //println!("cur: {:?}", cur_token);
        // Parse parameters of the function
        let arguments = self.parse_arguments();
        //println!("{} Arguments: {:?} {}", "\x1b[34m", arguments,"\x1b[0m");

        // Parse function return type
        // -> <type>
        let mut return_type = ASTtypename::TypeVoid;
        if self.next_token().token_type == TokenType::Symbol(Symbol::Arrow) {
            //self.next_token();
            match self.next_token().token_type {
                TokenType::TypeName(type_name) => {
                    if let Some(ast_type) = type_map.get(&type_name) {
                        return_type = *ast_type;
                    }
                }
                _ => {
                    panic!(
                        "{} Require type to construct function! {:?}{}",
                        "\x1b[31m", cur_token, "\x1b[0m"
                    );
                }
            }
        }

        // parse statements
        let statements = self.parse_statement();

        self.asts.push(AST::Statement(ASTstatement::Function {
            public: is_public,
            name: function_name,
            args: arguments,
            statements,
            return_type,
        }));
    }
    fn parse_return(&mut self) -> AST {
        let return_value = match self.peek_token().token_type {
            TokenType::Symbol(Symbol::Semicolon) => AST::TypeValue(ASTtypevalue::TypeVoid),
            _ => {
                let value = self.parse_expression();
                let next = self.next_token();
                if next.token_type != TokenType::Symbol(Symbol::Semicolon) {
                    self.report_error(AstGenError {
                        message: AstError::ExpectedToken(Token::new(
                            TokenType::Symbol(Symbol::Semicolon),
                            next.metadata,
                        )),
                    })
                }
                value
            }
        };

        AST::Statement(ASTstatement::Return {
            value: Box::new(return_value),
        })
    }
    fn parse_assignment(&mut self, ident: String) -> AST {
        let op = match self.next_token().token_type {
            TokenType::Assign(op) => match op {
                Assign::Assign => ASTOperator::Assign,
                Assign::AddAssign => ASTOperator::AddAssign,
                Assign::SubAssign => ASTOperator::SubAssign,
                Assign::MulAssign => ASTOperator::MulAssign,
                Assign::DivAssign => ASTOperator::DivAssign,
                Assign::RemAssign => ASTOperator::RemAssign,
                Assign::BitAndAssign => ASTOperator::BitAndAssign,
                Assign::BitOrAssign => ASTOperator::BitOrAssign,
                Assign::BitXorAssign => ASTOperator::BitXorAssign,
            },
            _ => unreachable!(),
        };
        // println!("{} Op: {:?} {}", "\x1b[34m", op, "\x1b[0m");
        let right_expr = self.parse_expression();
        // println!("{} Right expr: {:?} {}", "\x1b[34m", right_expr, "\x1b[0m");
        // println!("{} peek: {:?} {}", "\x1b[34m", self.peek_token(), "\x1b[0m");
        let peeked = self.peek_token();
        if peeked.token_type == TokenType::Symbol(Symbol::Semicolon) {
            self.next_token();
        } else {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::Semicolon),
                    peeked.metadata,
                )),
            })
        }
        AST::Statement(ASTstatement::Assignment {
            left: Box::new(AST::TypeValue(ASTtypevalue::Identifier(ident))),
            op,
            right: Box::new(right_expr),
        })
    }
    fn parse_expression(&mut self) -> AST {
        self.parse_level1()
    }

    fn parse_level1(&mut self) -> AST {
        let mut node = self.parse_level2();
        while let TokenType::Logical(op) = self.peek_token().token_type {
            match op {
                Logical::Or => {
                    self.next_token();
                    let right_node = self.parse_level2();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: ASTOperator::Or,
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_level2(&mut self) -> AST {
        let mut node = self.parse_level3();
        while let TokenType::Logical(op) = self.peek_token().token_type {
            match op {
                Logical::And => {
                    self.next_token();
                    let right_node = self.parse_level3();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: ASTOperator::And,
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_level3(&mut self) -> AST {
        let mut node = self.parse_level4();
        while let TokenType::Logical(op) = self.peek_token().token_type {
            match op {
                Logical::Equals | Logical::NotEquals => {
                    self.next_token();
                    let right_node = self.parse_level4();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: match op {
                            Logical::Equals => ASTOperator::Equals,
                            Logical::NotEquals => ASTOperator::NotEquals,
                            _ => unreachable!(),
                        },
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_level4(&mut self) -> AST {
        let mut node = self.parse_level5();
        while let TokenType::Logical(op) = self.peek_token().token_type {
            match op {
                Logical::LessThan
                | Logical::LessThanEquals
                | Logical::GreaterThan
                | Logical::GreaterThanEquals => {
                    self.next_token();
                    let right_node = self.parse_level5();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: match op {
                            Logical::LessThan => ASTOperator::Less,
                            Logical::LessThanEquals => ASTOperator::LessEquals,
                            Logical::GreaterThan => ASTOperator::Greater,
                            Logical::GreaterThanEquals => ASTOperator::GreaterEquals,
                            _ => unreachable!(),
                        },
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_level5(&mut self) -> AST {
        let mut node = self.parse_level6();
        while let TokenType::Operator(op) = self.peek_token().token_type {
            match op {
                Operator::Add | Operator::Subtract => {
                    self.next_token();
                    let right_node = self.parse_level6();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: match op {
                            Operator::Add => ASTOperator::Add,
                            Operator::Subtract => ASTOperator::Subtract,
                            _ => unreachable!(),
                        },
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_level6(&mut self) -> AST {
        let mut node = self.parse_primary();
        while let TokenType::Operator(op) = self.peek_token().token_type {
            match op {
                Operator::Multiply | Operator::Divide | Operator::Remainder => {
                    self.next_token();
                    let right_node = self.parse_primary();
                    node = AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(node),
                        op: match op {
                            Operator::Multiply => ASTOperator::Multiply,
                            Operator::Divide => ASTOperator::Divide,
                            Operator::Remainder => ASTOperator::Remainder,
                            _ => unreachable!(),
                        },
                        right: Box::new(right_node),
                    });
                }
                _ => break,
            }
        }
        node
    }

    fn parse_primary(&mut self) -> AST {
        // let cur_token = self.next_token();
        let next_token = self.peek_token();
        // println!("WAI {} {} {} ", "\x1b[31m", next_token, "\x1b[0m");
        if TokenType::Symbol(Symbol::OpenParen) == next_token.token_type {
            // Consume the opening parenthesis
            // println!("WAI {} {} {} ", "\x1b[31m", cur_token, "\x1b[0m");
            self.next_token();
            let node = self.parse_expression();
            let peek_token = self.peek_token();
            let test = match peek_token.token_type {
                // TokenType::Logical(_)| Token::Operator(_) => self.parse_expression(),
                TokenType::Symbol(Symbol::CloseParen) => {
                    self.next_token();
                    return node;
                }
                _ => {
                    self.next_token();
                    self.parse_expression()
                }
            };
            // println!("YES IS ME ");

            return test;
        }
        // Handle literals and identifiers
        match next_token.token_type {
            // TokenType::Symbol(Symbol::OpenParen) => panic!("Open Parenthesis"),
            TokenType::TypeValue(TypeValue::Number(num)) => match self.peek_token().token_type {
                TokenType::Logical(_) => self.parse_expression(),
                _ => {
                    self.next_token();
                    AST::TypeValue(ASTtypevalue::I64(num.parse::<i64>().unwrap()))
                }
            },
            TokenType::TypeValue(TypeValue::Identifier(ident)) => {
                match self.peek_token().token_type {
                    TokenType::Logical(_) => self.parse_expression(),
                    _ => {
                        self.next_token();
                        let _status = 1;
                        if self.peek_token().token_type == TokenType::Symbol(Symbol::OpenParen) {
                            //FuncCall
                            self.next_token(); // Consume the opening parenthesis
                            let mut arguments = Vec::new();
                            // println!("FuncCall");
                            while let token = self.peek_token() {
                                // println!("Token FC: {:?}", token);

                                match token.token_type {
                                    TokenType::Symbol(Symbol::CloseParen) => {
                                        self.next_token();
                                        // println!("Close Paren");
                                        break;
                                    }
                                    TokenType::Symbol(Symbol::Comma) => {
                                        self.next_token();
                                        // println!("Comma");
                                    }
                                    _ => {
                                        // println!("Argument {:?}",token);
                                        arguments.push(self.parse_expression());
                                        // self.next_token();
                                    }
                                }
                            }
                            // println!("Arguments: {:?}", arguments);
                            return AST::TypeValue(ASTtypevalue::FunctionCall {
                                name: ident.to_string(),
                                args: arguments,
                            });
                        }
                        AST::TypeValue(ASTtypevalue::Identifier(ident.to_string()))
                    }
                }
            }
            TokenType::TypeValue(TypeValue::QuotedString(s)) => {
                match self.peek_token().token_type {
                    TokenType::Logical(_) => {
                        panic!("Logical operator is not allowed in quoted string!")
                    }
                    _ => {
                        self.next_token();
                        AST::TypeValue(ASTtypevalue::QuotedString(s.to_string()))
                    }
                }
            }
            TokenType::Logical(_) | TokenType::Operator(_) => self.parse_expression(),
            _ => {
                // println!(
                // "{} Current Token: {:?}{}",
                // "\x1b[36m", next_token, "\x1b[0m"
                // );
                self.report_error(AstGenError {
                    message: AstError::ExpectedExpression(),
                });
                AST::TypeValue(ASTtypevalue::TypeVoid) // Placeholder
            }
        }
    }

    fn parse_print(&mut self) -> AST {
        // Consume the opening parenthesis
        let next = self.next_token();
        if next.token_type != TokenType::Symbol(Symbol::OpenParen) {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::OpenParen),
                    next.metadata,
                )),
            });
        }

        let value = self.parse_expression();
        let cur_token = self.next_token();
        // Consume the closing parenthesis
        if cur_token.token_type != TokenType::Symbol(Symbol::CloseParen) {
            // println!("Consume the closing parenthesis");
            // println!("cur token: {:?}", cur_token);
            // println!("next token: {:?}", self.peek_token());
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::CloseParen),
                    cur_token.metadata,
                )),
            });
        }
        // cunsume the semicolon
        let cur_token = self.next_token();
        if cur_token.token_type != TokenType::Symbol(Symbol::Semicolon) {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::Semicolon),
                    cur_token.metadata,
                )),
            });
        }

        AST::Statement(ASTstatement::Print {
            value: Box::new(value),
        })
    }
    fn parse_println(&mut self) -> AST {
        // Consume the opening parenthesis
        let next = self.next_token();
        if next.token_type != TokenType::Symbol(Symbol::OpenParen) {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::OpenParen),
                    next.metadata,
                )),
            });
        }

        let value = self.parse_expression();

        // Consume the closing parenthesis
        let cur_token = self.next_token();
        if cur_token.token_type != TokenType::Symbol(Symbol::CloseParen) {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::CloseParen),
                    cur_token.metadata,
                )),
            });
        }
        // cunsume the semicolon
        let cur_token = self.next_token();
        if cur_token.token_type != TokenType::Symbol(Symbol::Semicolon) {
            self.report_error(AstGenError {
                message: AstError::ExpectedToken(Token::new(
                    TokenType::Symbol(Symbol::Semicolon),
                    cur_token.metadata,
                )),
            });
        }

        AST::Statement(ASTstatement::Println {
            value: Box::new(value),
        })
    }
    fn parse_if(&mut self) -> AST {
        self.next_token();
        // Parse the condition
        let condition = self.parse_expression();
        // Parse the statements
        let statements = self.parse_statement();
        let mut elif = Option::None;
        let mut else_statements = Option::None;
        // Create the If AST node
        // Check for else or else if
        if let TokenType::Statement(Statement::Else) = self.peek_token().token_type {
            self.next_token(); // consume the else token
            match self.peek_token().token_type {
                TokenType::Statement(Statement::If) => {
                    self.next_token(); // consume the if token
                    let else_if_node = self.parse_if();

                    elif = Option::Some(Box::new(else_if_node));
                }
                _ => {
                    else_statements = Option::Some(self.parse_statement());
                }
            }
        }
        AST::Statement(ASTstatement::If {
            condition: Box::new(condition),
            statements,
            elif,
            else_statements,
        })
    }
    fn parse_let(&mut self) -> AST {
        // Let Statement Example
        // let:i32 a = 5;

        let mut status = 1;
        let mut name: String = String::new();
        let mut type_name: Option<ASTtypename> = None;
        let mut value: Option<Box<AST>> = None;
        let type_map: HashMap<TypeName, ASTtypename> = [
            (TypeName::I8, ASTtypename::I8),
            (TypeName::I16, ASTtypename::I16),
            (TypeName::I32, ASTtypename::I32),
            (TypeName::I64, ASTtypename::I64),
            (TypeName::U8, ASTtypename::U8),
            (TypeName::U16, ASTtypename::U16),
            (TypeName::U32, ASTtypename::U32),
            (TypeName::U64, ASTtypename::U64),
            (TypeName::Bool, ASTtypename::Bool),
            (TypeName::QuotedString, ASTtypename::QuotedString),
        ]
        .iter()
        .cloned()
        .collect();
        match self.next_token().token_type {
            TokenType::Symbol(Symbol::Colon) => {
                status = 2;
            }
            TokenType::TypeValue(TypeValue::Identifier(ident)) => {
                name = ident.to_string();
                status = 4;
            }
            _ => {
                println!("Invalid `let` statement Contruction Detected");
                std::process::exit(0);
            }
        }

        while let token = self.peek_token() {
            // println!(
            // "\x1b[34m Token: {:?}, Status:{} \x1b[0m", token, status
            // );
            match (token.clone().token_type, &status) {
                (TokenType::TypeName(typename), 2) => {
                    if let Some(ast_type) = type_map.get(&typename) {
                        type_name = Some(*ast_type);
                        self.next_token();
                        status = 3;
                        continue;
                    }
                    println!("Missing Type Announcement for `let` statement After `:`");
                }
                (TokenType::TypeValue(TypeValue::Identifier(ident)), 3) => {
                    name = ident.to_string();
                    self.next_token();
                    status = 4;
                    continue;
                }
                (TokenType::Assign(Assign::Assign), 4) => {
                    self.next_token();
                    status = 5;
                    continue;
                }
                (_, 5) => {
                    value = Some(Box::new(self.parse_expression()));
                    // println!("Value: {:?}", value);
                    status = 6;
                    break;
                }
                (TokenType::Symbol(Symbol::Semicolon), 6) => {
                    self.next_token();
                    println!("End of `let` statement");
                    break;
                }
                _ => {
                    panic!("Unexpected token in `let` : {:?}", token);
                }
            }
        }

        AST::Statement(ASTstatement::Let {
            name,
            type_name,
            value,
        })
    }

    fn parse_for(&mut self) -> AST {
        //let mut statements: Vec<ASTstatement> = Vec::new();
        // parse arguments and for header
        //println!("{} Start of For: {:?} {}", "\x1b[34m", self.next_token(), "\x1b[0m");

        let mut status = 1;
        let mut start_val: ASTtypevalue = ASTtypevalue::TypeVoid;
        let mut end_val: ASTtypevalue = ASTtypevalue::TypeVoid;
        let mut val: ASTtypevalue = ASTtypevalue::TypeVoid;

        while let token = self.next_token() {
            match (token.clone().token_type, &status) {
                (TokenType::TypeValue(TypeValue::Identifier(ident)), 2) => {
                    start_val = ASTtypevalue::Identifier(ident);
                    status = 3;
                    continue;
                }
                (TokenType::Symbol(Symbol::Arrow), 3) => {
                    status = 4;
                    continue;
                }
                (TokenType::TypeValue(TypeValue::Identifier(ident)), 4) => {
                    end_val = ASTtypevalue::Identifier(ident);
                    status = 5;
                    continue;
                }
                (TokenType::TypeValue(TypeValue::Number(num)), 4) => {
                    end_val = ASTtypevalue::I64(num.parse::<i64>().unwrap());
                    status = 5;
                    continue;
                }
                (TokenType::Symbol(Symbol::DoubleColon), 5) => {
                    status = 6;
                    continue;
                }
                (TokenType::TypeValue(TypeValue::Number(num)), 6) => {
                    val = ASTtypevalue::I64(num.parse::<i64>().unwrap());
                    status = 7;
                    continue;
                }
                (TokenType::Symbol(Symbol::CloseParen), 7) => {
                    break;
                }
                (TokenType::Symbol(Symbol::OpenParen), 1) => {
                    status = 2;
                    continue;
                }
                _ => {
                    //println!("{} cur statement token: {:?} {}", "\x1b[31m", token, "\x1b[0m");
                    //println!("{} cur statement status: {:?} {}", "\x1b[31m", status, "\x1b[0m");
                    panic!("{} Invalid for statement! {}", "\x1b[31m", "\x1b[0m");
                }
            }
        }
        // parse statements
        let statements = self.parse_statement();

        AST::Statement(ASTstatement::For {
            start: start_val,
            end: end_val,
            value: val,
            statements,
        })
    }
    /*fn parse_if(&mut self) {
    //let mut statements: Vec<ASTstatement> = Vec::new();
    // parse arguments and if header
    //println!("{} Start of If: {:?} {}", "\x1b[34m", self.next_token(), "\x1b[0m");
    let mut condition = Vec::new();

    while let token = self.next_token() {
    match token {
    TokenType::Symbol(Symbol::OpenParen) => {
    continue;
    }
    TokenType::Symbol(Symbol::CloseParen) => {
    break;
    }
    _ => {
    condition.push(token);
    }
    }
    }
    // parse statements
    self.parse_statement();
    }*/
    fn parse_arguments(&mut self) -> Vec<ASTtypecomp> {
        let mut args: Vec<ASTtypecomp> = Vec::new();
        let mut state: ArgumentParseState = ArgumentParseState::WaitForType;
        let mut cur_type = ASTtypename::TypeVoid;
        let type_map: HashMap<TypeName, ASTtypename> = [
            (TypeName::I8, ASTtypename::I8),
            (TypeName::I16, ASTtypename::I16),
            (TypeName::I32, ASTtypename::I32),
            (TypeName::I64, ASTtypename::I64),
            (TypeName::U8, ASTtypename::U8),
            (TypeName::U16, ASTtypename::U16),
            (TypeName::U32, ASTtypename::U32),
            (TypeName::U64, ASTtypename::U64),
            (TypeName::Bool, ASTtypename::Bool),
            (TypeName::QuotedString, ASTtypename::QuotedString),
        ]
        .iter()
        .cloned()
        .collect();
        while let token = self.next_token() {
            let _peeked = self.peek_token();
            //println!("{}cur arg: {:?}{}", "\x1b[38m", token, "\x1b[0m");
            //println!("{}cur State: {:?}{}", "\x1b[38m", state, "\x1b[0m");
            match (token.clone().token_type, &state) {
                (
                    TokenType::Symbol(Symbol::CloseParen),
                    ArgumentParseState::WaitForCommaOrCloseParen,
                ) => {
                    break;
                }
                (TokenType::Symbol(Symbol::CloseParen), ArgumentParseState::WaitForType) => {
                    break;
                }
                (TokenType::Symbol(Symbol::OpenParen), ArgumentParseState::WaitForType) => {
                    continue;
                }
                (TokenType::TypeName(type_name), ArgumentParseState::WaitForType) => {
                    if let Some(ast_type) = type_map.get(&type_name) {
                        cur_type = *ast_type;
                        state = ArgumentParseState::WaitForColon;
                    }
                }
                (TokenType::Symbol(Symbol::Colon), ArgumentParseState::WaitForColon) => {
                    state = ArgumentParseState::WaitForIdentifier;
                }
                (
                    TokenType::TypeValue(TypeValue::Identifier(ident)),
                    ArgumentParseState::WaitForIdentifier,
                ) => {
                    let ident_name = ASTtypevalue::Identifier(ident.to_string());
                    args.push(ASTtypecomp::Argument {
                        identifier: ident_name,
                        type_name: cur_type,
                    });
                    state = ArgumentParseState::WaitForCommaOrCloseParen;
                    cur_type = ASTtypename::TypeVoid;
                }
                (
                    TokenType::Symbol(Symbol::Comma),
                    ArgumentParseState::WaitForCommaOrCloseParen,
                ) => {
                    state = ArgumentParseState::WaitForType;
                }

                _ => {
                    let error_msg = match state {
                        ArgumentParseState::WaitForType => {
                            "Require a type to construct an argument!"
                        }
                        ArgumentParseState::WaitForColon => {
                            "Require a colon to construct an argument!"
                        }
                        ArgumentParseState::WaitForIdentifier => {
                            "Require an identifier to construct an argument!"
                        }
                        ArgumentParseState::WaitForCommaOrCloseParen => {
                            "Require a comma or close paren to construct an argument!"
                        }
                    };
                    //println!("{} {} {}", "\x1b[33m", token, "\x1b[0m");
                    panic!("{} {} {}", "\x1b[31m", error_msg, "\x1b[0m");
                }
            }
        }

        args
    }
    #[allow(dead_code)]
    pub fn get_asts(&self) -> Vec<AST> {
        self.asts.clone()
    }
}
