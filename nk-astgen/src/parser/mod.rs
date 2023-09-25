use lexer::tokens_new::*;

mod error;
use crate::ast::*;
use error::{AstError, AstGenError};

use std::collections::HashMap;
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
}

impl<'a> Parser<'a> {
    #[allow(dead_code)]
    pub fn new(tokens: &'a [Token]) -> Self {
        let peeked = tokens.iter().cloned().peekable();
        Parser {
            tokens: peeked,
            state: State::EmptyState,
            brace_inner: 0,
            asts: Vec::new(),
            buffer: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn next_token(&mut self) -> Token {
        let token = self.tokens.next();

        match token {
            Some(t) => t,
            None => Token::EOF,
        }
    }
    #[allow(dead_code)]
    fn peek_token(&mut self) -> Token {
        let peek = self.tokens.peek().clone();

        match peek {
            Some(t) => t.clone(),
            None => Token::EOF,
        }
    }
    #[allow(dead_code)]
    fn expect(&mut self, current: Token, expected: Token) -> Result<(), AstGenError> {
        println!("{} Current Token: {:?}{}", "\x1b[37m", current, "\x1b[0m");
        match expected {
            cur_token => Ok(()),
            Token::EOF => Err(AstGenError {
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
            match token {
                Token::Statement(Statement::If) => {
                    self.parse_if();
                }
                Token::Statement(Statement::For) => {
                    statements.push(self.parse_for());
                }

                Token::Symbol(Symbol::OpenBrace) => {
                    continue;
                }
                Token::Symbol(Symbol::CloseBrace) => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }

        statements
    }
    #[allow(dead_code)]
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
                match token {
                    Token::Statement(Statement::Function) => {
                        self.parse_function(false);
                    }
                    Token::Statement(Statement::Public) => {
                        self.parse_function(true);
                    }
                    Token::Statement(Statement::Let) => {
                        self.state = State::GlobalLet;
                        //println!("Founded Global Let");
                    }
                    Token::Statement(Statement::Inject) => {
                        self.state = State::Inject;
                        //println!("Founded Import");
                    }
                    Token::EOF => {
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
                    if peeked == Token::Symbol(Symbol::Semicolon) {
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
                    if peeked == Token::Symbol(Symbol::Semicolon) {
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
         let mut args: Vec<ASTtypecomp> = Vec::new();

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
        if self.next_token() == Token::Symbol(Symbol::Arrow){
            //self.next_token();
            match self.next_token() {
                Token::TypeName(type_name) => {
                    if let Some(ast_type) = type_map.get(&type_name) {
                        return_type = *ast_type;
                    }

                }
                _ => {
                    panic!("{} Require type to construct function! {}", "\x1b[31m", "\x1b[0m");
                }
            }
                    }

        // parse statements
        let statements = self.parse_statement();

        self.asts.push(AST::Statement(ASTstatement::Function{
            public: is_public,
            name: function_name,
            args: arguments,
            statements,
            return_type

        }));
    }
    /*fn parse_print(&mut self) -> AST {
        // Get the token inside the parentheses
        let mut inside_parentheses = Vec::new();

        while let token = self.next_token() {
            match token {
                Token::Symbol(Symbol::OpenParen) => {
                    continue;
                }
                Token::Symbol(Symbol::CloseParen) => {
                    break;
                }
                _ => {
        
                }
            }
        }

        // Construct the AST
        AST::Statement(ASTstatement::Print {
            value: inside_parentheses,
        })
    }*/
    fn parse_println(&mut self) {
        
    }
    fn parse_for(&mut self) -> AST {
        //let mut statements: Vec<ASTstatement> = Vec::new();
        // parse arguments and for header
        //println!("{} Start of For: {:?} {}", "\x1b[34m", self.next_token(), "\x1b[0m");
        
        let mut status = 1;
        let mut start_val:ASTtypevalue =ASTtypevalue::TypeVoid;
        let mut end_val:ASTtypevalue =ASTtypevalue::TypeVoid;
        let mut val:ASTtypevalue =ASTtypevalue::TypeVoid;

        while let token = self.next_token() {
            match (token.clone(), &status) {
                (Token::TypeValue(TypeValue::Identifier(ident)), 2) => {
                    start_val = ASTtypevalue::Identifier(ident);
                    status = 3;
                    continue;
                }
                (Token::Symbol(Symbol::Arrow), 3) => {
                    status = 4;
                    continue;
                }
                (Token::TypeValue(TypeValue::Identifier(ident)), 4) => {
                    end_val = ASTtypevalue::Identifier(ident);
                    status = 5;
                    continue;
                }
                (Token::TypeValue(TypeValue::Number(num)), 4) => {
                    end_val =  ASTtypevalue::I64(num.parse::<i64>().unwrap());
                    status = 5;
                    continue;
                }
                (Token::Symbol(Symbol::DoubleColon), 5) => {
                    status = 6;
                    continue;
                }
                (Token::TypeValue(TypeValue::Number(num)), 6) => {
                    val = ASTtypevalue::I64(num.parse::<i64>().unwrap());
                    status = 7;
                    continue;
                }
                (Token::Symbol(Symbol::CloseParen), 7) => {
                    break;
                }
                (Token::Symbol(Symbol::OpenParen), 1) => {
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
            statements: statements,
            
        })
    }
    fn parse_if(&mut self) {
        //let mut statements: Vec<ASTstatement> = Vec::new();
        // parse arguments and if header
        //println!("{} Start of If: {:?} {}", "\x1b[34m", self.next_token(), "\x1b[0m");
        let mut condition = Vec::new();

        while let token = self.next_token() {
            match token {
                Token::Symbol(Symbol::OpenParen) => {
                    continue;
                }
                Token::Symbol(Symbol::CloseParen) => {
                    break;
                }
                _ => {
                    condition.push(token);
                }
            }
        }
        // parse statements
        self.parse_statement();
    }
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
            let peeked = self.peek_token();
            //println!("{}cur arg: {:?}{}", "\x1b[38m", token, "\x1b[0m");
            //println!("{}cur State: {:?}{}", "\x1b[38m", state, "\x1b[0m");
            match (token.clone(), &state) {
                (
                    Token::Symbol(Symbol::CloseParen),
                    (ArgumentParseState::WaitForCommaOrCloseParen),
                ) => {
                    break;
                }
                (Token::Symbol(Symbol::CloseParen), ArgumentParseState::WaitForType) => {
                    break;
                }
                (Token::Symbol(Symbol::OpenParen), ArgumentParseState::WaitForType) => {
                    continue;
                }
                (Token::TypeName(type_name), ArgumentParseState::WaitForType) => {
                    if let Some(ast_type) = type_map.get(&type_name) {
                        cur_type = *ast_type;
                        state = ArgumentParseState::WaitForColon;
                    }
                }
                (Token::Symbol(Symbol::Colon), ArgumentParseState::WaitForColon) => {
                    state = ArgumentParseState::WaitForIdentifier;
                }
                (
                    Token::TypeValue(TypeValue::Identifier(ident)),
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
                (Token::Symbol(Symbol::Comma), ArgumentParseState::WaitForCommaOrCloseParen) => {
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