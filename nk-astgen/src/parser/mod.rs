use lexer::tokens_new::*;

mod error;
use crate::ast::*;
use error::{AstError, AstGenError};

use std::iter::Cloned;
use std::iter::Peekable;
use std::collections::HashMap;

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
    fn parse_statement(&mut self) {
        let mut statements: Vec<AST> = Vec::new();
        // parse statements
        while let token = self.next_token() {
            match token {
                Token::Statement(Statement::If) => {
                    self.parse_if();
                }
                Token::Statement(Statement::For) => {
                    self.parse_for();
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
                        self.state = State::Function;
                        //println!("Founded Function");
                    }
                    Token::Statement(Statement::Public) => {
                        self.state = State::PublicFunction;
                        // parse the public function in order of
                        // public -> fn -> function_name -> Arguments -> Arrow -> return type -> { ->
                        // function_body -> }
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
                State::PublicFunction => {
                    //if token == Token::Symbol(Symbol::OpenBrace) {
                    //    self.brace_inner += 1;
                    //} else if token == Token::Symbol(Symbol::CloseBrace) {
                    //    self.brace_inner -= 1;
                    //}
                    self.parse_function(true);
                    //self.buffer.push(token);

                    self.state = State::EmptyState;
                }
                State::Function => {
                    if token == Token::Symbol(Symbol::OpenBrace) {
                        self.brace_inner += 1;
                    }
                    self.parse_function(false);
                    self.state = State::EmptyState;

                    //self.buffer.clear();
                }
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
        let mut cur_token = self.next_token();
        if is_public {
            cur_token = self.next_token();
        }
        let mut statements: Vec<ASTstatement> = Vec::new();
        let mut args: Vec<ASTtypecomp> = Vec::new();

        // parse arguments and function header
        //println!("{} Start of Function: {:?} {}", "\x1b[34m", cur_token,"\x1b[0m");
        let function_name = cur_token.to_string();

        //println!("{} Function name: {:?} {}", "\x1b[34m", function_name,"\x1b[0m");

        let arguments = self.parse_arguments();
        // parse statements
        self.parse_statement();
    }

    fn parse_for(&mut self) {
        //let mut statements: Vec<ASTstatement> = Vec::new();
        // parse arguments and for header
        //println!("{} Start of For: {:?} {}", "\x1b[34m", self.next_token(), "\x1b[0m");
        let mut condition = Vec::new();
        let mut status = 1;
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

        // parse statements
        //while let token = self.peek_token() {

        //}
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
            (TypeName::QuotedString, ASTtypename::QuotedString)
        ].iter().cloned().collect();
        while let token = self.next_token() {
            let peeked = self.peek_token();
            //println!("{}cur arg: {:?}{}", "\x1b[38m", token, "\x1b[0m");
            match (token.clone(), &state) {
                (Token::Symbol(Symbol::CloseParen), (ArgumentParseState::WaitForCommaOrCloseParen | ArgumentParseState::WaitForType)) => {
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
                (Token::TypeValue(TypeValue::Identifier(ident)), ArgumentParseState::WaitForIdentifier) => {
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
                    println!("{} Unexpected token: {:?} {}", "\x1b[31m", &token, "\x1b[0m");
                    println!("{} state: {:?} {}", "\x1b[31m", state, "\x1b[0m");
                    let error_msg = match state {
                        ArgumentParseState::WaitForType => "Require a type to construct an argument!",
                        ArgumentParseState::WaitForColon => "Require a colon to construct an argument!",
                        ArgumentParseState::WaitForIdentifier => "Require an identifier to construct an argument!",
                        ArgumentParseState::WaitForCommaOrCloseParen => "Require a comma or close paren to construct an argument!",
                    };
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
