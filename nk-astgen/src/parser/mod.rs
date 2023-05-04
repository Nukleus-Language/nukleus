use lexer::tokens_new::*;

mod error;
use crate::ast::AST;
use error::{AstError, AstGenError};

use std::iter::Cloned;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
enum State {
    EmptyState,
    DefaultState,
    PublicFunction,
    Function,
    Inject,
    GlobalLet,
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
    fn expect(&mut self, expected: Token, cur_token: Token) -> Result<(), AstGenError> {
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
    #[allow(dead_code)]
    fn report_error(&self, error: AstGenError) {
        println!("{}", error);
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
                        //println!("Founded Public Function");
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
                    if token == Token::Symbol(Symbol::OpenBrace) {
                        self.brace_inner += 1;
                    } else if token == Token::Symbol(Symbol::CloseBrace) {
                        self.brace_inner -= 1;
                    }
                    //self.buffer.push(token);
                    if peeked == Token::Symbol(Symbol::CloseBrace) && self.brace_inner == 1 {
                        self.state = State::EmptyState;
                        //self.buffer.push(peeked);
                        //parse_function(self.buffer.clone(), true);
                        //println!("{} Function: {:?} {}","\x1b[34m" , self.buffer,"\x1b[0m");
                        //self.buffer.clear();
                        self.brace_inner = 0;
                        //self.buffer.clear();
                        self.next_token();
                    }
                }
                State::Function => {
                    if token == Token::Symbol(Symbol::OpenBrace) {
                        self.brace_inner += 1;
                    }
                    self.parse_function(false);
                    self.state = State::EmptyState;
                    self.brace_inner = 0;
                    //self.buffer.clear();
                    self.next_token();
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
        while let token = self.peek_token() {
            //println!("cur: {:?}", token);
            match token {
                Token::Symbol(Symbol::OpenBrace) => {
                    //self.buffer.push(token);
                    self.brace_inner += 1;
                    //println!("Brace: {}", self.brace_inner);
                    self.next_token();
                }
                Token::Symbol(Symbol::CloseBrace) => {
                    self.brace_inner -= 1;
                    //println!("Brace: {}", self.brace_inner);
                    if self.brace_inner == 0 {
                        //self.buffer.push(token);

                        //println!("{} Function: {:?} {}", "\x1b[34m", self.buffer,"\x1b[0m");
                        return;
                    }
                    self.next_token();
                }
                _ => {
                    //self.buffer.push(token);
                    self.next_token();
                }
            }
        }
    }
    /*fn parse_statement(&mut self) -> Result<AST, AstGenError> {

    }*/

    #[allow(dead_code)]
    pub fn get_asts(&self) -> Vec<AST> {
        self.asts.clone()
    }
}
