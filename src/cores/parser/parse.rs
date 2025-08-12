use crate::cores::ast::{AST, AstParseError};
use lexer::{Assign, Statement, Symbol, Token, TypeName, TypeValue};

//use lexer::Lexer;

use std::collections::HashMap;
use std::iter::{Cloned, Peekable};

pub struct Parser<'a> {
    tokens: Peekable<Cloned<std::slice::Iter<'a, Token>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().cloned().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, AstParseError> {
        let mut ast = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Statement(Statement::Function) => {
                    ast.push(self.parse_function(false)?);
                }
                Token::Statement(Statement::Public) => {
                    ast.push(self.parse_function(true)?);
                }
                _ => {
                    return Err(AstParseError::UnknownToken {
                        token: token.to_string(),
                    });
                }
            }
        }

        Ok(ast)
    }

    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn expect(&mut self, expected: Token) -> Result<(), AstParseError> {
        if self.tokens.peek() == Some(&expected) {
            Ok(())
        } else {
            Err(AstParseError::ExpectedOther {
                token: expected.to_string(),
            })
        }
    }
    fn parse_function(&mut self, is_public: bool) -> Result<AST, AstParseError> {
        if is_public {
            self.consume(); // Consume Tokens::Public
        }
        self.consume(); // Consume Tokens::Function
        let name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Function Name".to_owned(),
            })?;
        match name.is_identifier() {
            true => (),
            false => {
                return Err(AstParseError::ExpectedOther {
                    token: "Function Name".to_owned(),
                });
            }
        }
        self.consume(); // Consume Function Name

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume OpenParen
        let mut done_flag = false;
        // Parse Arguments
        let mut arguments = Vec::new();
        while !done_flag {
            match self.tokens.peek() {
                Some(Token::Symbol(Symbol::CloseParen)) => {
                    self.consume(); // Consume CloseParen
                    done_flag = true;
                }

                _ => {
                    let arg_type =
                        self.tokens
                            .peek()
                            .cloned()
                            .ok_or(AstParseError::ExpectedOther {
                                token: "Argument Type".to_owned(),
                            })?;
                    self.consume(); // Consume Argument Type
                    self.expect(Token::Symbol(Symbol::Colon))?;
                    self.consume(); // Consume Colon
                    let arg_name =
                        self.tokens
                            .peek()
                            .cloned()
                            .ok_or(AstParseError::ExpectedOther {
                                token: "Argument Name".to_owned(),
                            })?;
                    match arg_name.is_identifier() {
                        true => (),
                        false => {
                            return Err(AstParseError::ExpectedOther {
                                token: "Proper Argumnet Name".to_owned(),
                            });
                        }
                    }
                    self.consume(); // Consume Argument Name
                    arguments.push((arg_type, arg_name));
                    if self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
                        self.consume(); // Consume Comma
                        if self.tokens.peek() == Some(&Token::Symbol(Symbol::CloseParen)) {
                            return Err(AstParseError::ExpectedOther {
                                token: "Argument Type".to_owned(),
                            });
                        }
                    }
                }
            }
        }

        //self.expect(Token::Symbol(Symbol::CloseParen))?;
        //self.consume(); // Consume CloseParen
        self.expect(Token::Symbol(Symbol::Arrow))?;
        self.consume(); // Consume Arrow

        let return_type = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Return Type".to_owned(),
            })?;
        self.consume(); // Consume Return Type

        self.expect(Token::Symbol(Symbol::OpenBrace))?;
        self.consume(); // Consume OpenBrace

        let statements = self.parse_statements(return_type.clone())?;
        let return_value = Token::TypeValue(TypeValue::None);
        /*if return_type != Token::TypeName(TypeName::Void) {
            self.expect(Token::Statement(Statement::Return))?;
            self.consume(); // Consume Return

            return_value = self
                .tokens
                .peek()
                .cloned()
                .ok_or(AstParseError::ExpectedOther {
                    token: "Return Value".to_owned(),
                })?;
            self.consume(); // Consume Value
            self.expect(Token::Symbol(Symbol::Semicolon))?;
            self.consume(); // Consume Semicolon
        } else if self.tokens.peek() == Some(&Token::Statement(Statement::Return)) {
            self.consume(); // Consume Return
            self.expect(Token::Symbol(Symbol::Semicolon))?;
            self.consume(); // Consume Semicolon
        }*/
        self.expect(Token::Symbol(Symbol::CloseBrace))?;
        self.consume(); // Consume Tokens::CloseBrace

        let function = AST::Function {
            public: is_public,
            name: name.to_string(),
            args: arguments,
            statements,
            variables: HashMap::new(),
            return_type,
            return_value,
        };

        Ok(function)
    }

    pub fn parse_statements(&mut self, _return_type: Token) -> Result<Vec<AST>, AstParseError> {
        let mut statements = Vec::new();
        //println!("{:?}", self.tokens.peek());
        while let Some(token) = self.tokens.peek() {
            //println!("{:?}", token);
            match token {
                Token::Statement(Statement::Let) => {
                    let let_statement = self.let_parser()?;
                    statements.push(let_statement);
                }
                Token::Statement(Statement::Print) => {
                    let print_statement = self.print_parser()?;
                    statements.push(print_statement);
                }
                Token::Statement(Statement::Println) => {
                    let println_statement = self.println_parser()?;
                    statements.push(println_statement);
                }
                Token::Statement(Statement::For) => {
                    let for_statement = self.for_parser()?;
                    statements.push(for_statement);
                }
                Token::Statement(Statement::If) => {
                    let if_statement = self.if_parser()?;
                    statements.push(if_statement);
                }
                Token::TypeValue(TypeValue::Identifier(_)) => {
                    //println!("PEEK 2 {:?}", self.tokens.peek());
                    let assign_statement = self.identifier_parser()?;
                    statements.push(assign_statement);
                }
                Token::Statement(Statement::Return) => {
                    let return_statement = self.return_parser()?;
                    statements.push(return_statement);
                }
                _ => break,
            }
        }
        Ok(statements)
    }

    fn let_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Let

        self.expect(Token::Symbol(Symbol::Colon))?;
        self.consume(); // Consume Colon
        let type_name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Type".to_owned(),
            })?;
        self.consume(); // Consume Type

        let variable_name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            })?;
        if !variable_name.is_identifier() {
            return Err(AstParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            });
        }
        self.consume(); // Consume Variable Name
        //
        self.expect(Token::Assign(Assign::Assign))?;
        self.consume(); // Consume Tokens::Assign

        //check if the value is a Identifier
        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value
        // check if the value is calling a function
        if self.tokens.peek() == Some(&Token::Symbol(Symbol::OpenParen)) {
            self.consume(); // Consume OpenParen
            let mut args: Vec<Token> = Vec::new();
            let mut is_after_comma = false;
            let mut is_after_open_paren = true;
            while let Some(token) = self.tokens.peek() {
                if token == &Token::Symbol(Symbol::CloseParen) {
                    if is_after_comma {
                        return Err(AstParseError::ExpectedOther {
                            token: "Argument".to_owned(),
                        });
                    }
                    break;
                } else if token == &Token::Symbol(Symbol::Comma) {
                    self.consume(); // Consume Comma
                    is_after_comma = true;
                } else if is_after_comma || is_after_open_paren {
                    args.push(token.clone());
                    self.consume(); // Consume Argument
                    is_after_comma = false;
                    is_after_open_paren = false;
                } else {
                    return Err(AstParseError::ExpectedOther {
                        token: "Argument".to_owned(),
                    });
                }
            }
            self.expect(Token::Symbol(Symbol::CloseParen))?;
            self.consume(); // Consume CloseParen
            let let_statement = AST::Let {
                name: variable_name.to_string(),
                type_name: Some(type_name.to_string()),
                value: Token::TypeValue(TypeValue::FunctionCall(value.to_string(), args)),
            };
            self.expect(Token::Symbol(Symbol::Semicolon))?;
            self.consume(); // Consume Tokens::Semicolon
            return Ok(let_statement);
        }

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Tokens::Semicolon
        //
        let let_statement = AST::Let {
            name: variable_name.to_string(),
            type_name: Some(type_name.to_string()),
            value,
        };
        Ok(let_statement)
    }
    /*fn assign_parser(&mut self, variable_name: Token) -> Result<AST, AstParseError> {
        println!("ASSIGN PARSER");
        println!("{:?}", self.tokens.peek());
        self.expect(Token::Assign(Assign::Assign))?;
        self.consume(); // Consume Assign

        let mut value: Vec<Token> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token == &Token::Symbol(Symbol::Semicolon) {
                break;
            }
            value.push(token.clone());
            self.consume();
        }
        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let assign_statement = AST::Assign {
            name: variable_name.to_string(),
            value,
        };
        Ok(assign_statement)
    }*/
    fn identifier_parser(&mut self) -> Result<AST, AstParseError> {
        let variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            })?;
        self.consume(); // Consume Identifier

        match self.tokens.peek() {
            Some(Token::Assign(Assign::Assign)) => {
                let assign_statement = self.assign_parser(variable)?;
                Ok(assign_statement)
            }

            /*Some(Token::Symbol(Symbol::OpenParen)) => {
                let function_call = self.function_call_parser()?;
                Ok(function_call)
            }*/
            Some(Token::Assign(Assign::AddAssign)) => {
                let add_assign_statement = self.add_assign_parser(variable)?;
                Ok(add_assign_statement)
            }
            Some(Token::Assign(Assign::SubAssign)) => {
                let sub_assign_statement = self.sub_assign_parser(variable)?;
                Ok(sub_assign_statement)
            }
            Some(Token::Assign(Assign::MulAssign)) => {
                let mul_assign_statement = self.mul_assign_parser(variable)?;
                Ok(mul_assign_statement)
            }
            Some(Token::Assign(Assign::DivAssign)) => {
                let div_assign_statement = self.div_assign_parser(variable)?;
                Ok(div_assign_statement)
            }
            Some(Token::Assign(Assign::RemAssign)) => {
                let rem_assign_statement = self.rem_assign_parser(variable)?;
                Ok(rem_assign_statement)
            }
            Some(Token::Symbol(Symbol::OpenParen)) => {
                let function_call = self.function_call_parser(variable)?;
                Ok(function_call)
            }
            _ => Err(AstParseError::Unknown),
        }
        //Ok()
    }
    fn assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::Assign))?;
        self.consume(); // Consume AddAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let assign_statement = AST::AddAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(assign_statement)
    }
    fn add_assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::AddAssign))?;
        self.consume(); // Consume AddAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let add_assign_statement = AST::AddAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(add_assign_statement)
    }
    fn sub_assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::SubAssign))?;
        self.consume(); // Consume SubAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let sub_assign_statement = AST::SubAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(sub_assign_statement)
    }
    fn mul_assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::MulAssign))?;
        self.consume(); // Consume MulAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let mul_assign_statement = AST::MulAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(mul_assign_statement)
    }
    fn div_assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::DivAssign))?;
        self.consume(); // Consume DivAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let div_assign_statement = AST::DivAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(div_assign_statement)
    }
    fn rem_assign_parser(&mut self, variable: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Assign(Assign::RemAssign))?;
        self.consume(); // Consume RemAssign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let rem_assign_statement = AST::RemAssign {
            l_var: variable,
            r_var: value,
        };
        Ok(rem_assign_statement)
    }
    fn function_call_parser(&mut self, name: Token) -> Result<AST, AstParseError> {
        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let mut done_flag = false;
        // Parse Arguments
        let mut arguments = Vec::new();
        //println!("Parsing Arguments...");
        while !done_flag {
            match self.tokens.peek() {
                Some(Token::Symbol(Symbol::CloseParen)) => {
                    self.consume(); // Consume CloseParen
                    done_flag = true;
                }
                Some(Token::Symbol(Symbol::Comma)) => {
                    self.consume(); // Consume Comma
                    // Check if there is another argument
                    if self.tokens.peek() == Some(&Token::Symbol(Symbol::CloseParen)) {
                        return Err(AstParseError::ExpectedOther {
                            token: "Argument".to_owned(),
                        });
                    }
                }
                _ => {
                    let arg_name =
                        self.tokens
                            .peek()
                            .cloned()
                            .ok_or(AstParseError::ExpectedOther {
                                token: "Argument".to_owned(),
                            })?;
                    self.consume(); // Consume Argument Name
                    //println!("Argument: {:?}", arg_name);
                    arguments.push(arg_name);
                }
            }
        }

        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume CloseParen

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let function_call_statement = AST::FunctionCall {
            name,
            args: arguments,
        };
        Ok(function_call_statement)
    }
    fn print_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Print

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume CloseParen

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let print_statement = AST::Print { value };
        Ok(print_statement)
    }
    fn if_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::If

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let l_value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        //self.expect(Token::Logical(_))?;
        let logic = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Logic".to_owned(),
            })?;
        self.consume(); // Consume Logic

        let r_value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume CloseParen

        self.expect(Token::Symbol(Symbol::OpenBrace))?;
        self.consume(); // Consume OpenBrace

        let statements = self.parse_statements(Token::TypeName(TypeName::Void))?;
        self.expect(Token::Symbol(Symbol::CloseBrace))?;
        self.consume(); // Consume CloseBrace

        let if_statement = AST::If {
            l_var: l_value,
            logic,
            r_var: r_value,
            statements,
        };
        Ok(if_statement)
    }
    /*fn if_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::If

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let mut condition: Vec<Token> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token == &Token::Symbol(Symbol::CloseParen) {
                break;
            }
            condition.push(token.clone());
            self.consume();
        }
        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume CloseParen

        self.expect(Token::Symbol(Symbol::OpenBrace))?;
        self.consume(); // Consume OpenBrace

        let statements = self.parse_statements(Token::TypeName(TypeName::Void))?;
        self.expect(Token::Symbol(Symbol::CloseBrace))?;
        self.consume(); // Consume CloseBrace

        let if_statement = AST::If {
            condition,
            statements,
        };
        Ok(if_statement)
    }*/

    fn println_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Println

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume Tokens::CloseParen

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Tokens::Semicolon

        let println_statement = AST::Println { value };
        Ok(println_statement)
    }

    fn for_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::For

        self.expect(Token::Symbol(Symbol::OpenParen))?;
        self.consume(); // Consume Tokens::OpenParen

        let start_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Token::Symbol(Symbol::Arrow))?;
        self.consume(); // Consume Tokens::Arrow

        let end_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Token::Symbol(Symbol::DoubleColon))?;
        self.consume(); // Consume Tokens::DoubleColon

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::CloseParen))?;
        self.consume(); // Consume Tokens::CloseParen

        self.expect(Token::Symbol(Symbol::OpenBrace))?;
        self.consume(); // Consume Tokens::OpenBrace

        let statements = self.parse_statements(Token::TypeName(TypeName::Void))?;

        self.expect(Token::Symbol(Symbol::CloseBrace))?;
        self.consume(); // Consume Tokens::CloseBrace
        let for_statement = AST::For {
            start: start_variable,
            end: end_variable,
            value,
            statements,
        };
        Ok(for_statement)
    }

    fn return_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Return
        if self.tokens.peek() == Some(&Token::Symbol(Symbol::Semicolon)) {
            self.consume(); // Consume Semicolon
            return Ok(AST::Return {
                value: Token::TypeValue(TypeValue::None),
            });
        }
        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Token::Symbol(Symbol::Semicolon))?;
        self.consume(); // Consume Semicolon

        let return_statement = AST::Return { value };
        Ok(return_statement)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cores::ast::AST;

    #[test]
    fn test_parse_function() {
        let tokens = vec![
            Token::Statement(Statement::Function),
            Token::TypeValue(TypeValue::Identifier("main".to_owned())),
            Token::Symbol(Symbol::OpenParen),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Arrow),
            Token::TypeName(TypeName::Void),
            Token::Symbol(Symbol::OpenBrace),
            Token::Symbol(Symbol::CloseBrace),
        ];
        let mut parser = Parser::new(&tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result,
            vec![AST::Function {
                public: false,
                name: "main".to_owned(),
                args: Vec::new(),
                statements: Vec::new(),
                variables: HashMap::new(),
                return_type: Token::TypeName(TypeName::Void),
                return_value: Token::TypeValue(TypeValue::None),
            }]
        );
    }

    /*#[test] //Not Implemented args yet
    fn test_parse_function_with_args() {
        let tokens = vec![
            Tokens::Function,
            Tokens::Identifier("my_function".to_owned()),
            Tokens::OpenParen,
            Tokens::Identifier("arg1".to_owned()),
            Tokens::Colon,
            Tokens::Int,
            Tokens::Comma,
            Tokens::Identifier("arg2".to_owned()),
            Tokens::Colon,
            Tokens::String,
            Tokens::CloseParen,
            Tokens::Arrow,
            Tokens::Void,
            Tokens::OpenBrace,
            Tokens::CloseBrace,

        ];
        let mut parser = Parser::new(&tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result,
            vec![AST::Function {
                public: false,
                name: "my_function".to_owned(),
                args: vec![
                    AST::FunctionArg {
                        name: "arg1".to_owned(),
                        type_name: "int".to_owned(),
                    },
                    AST::FunctionArg {
                        name: "arg2".to_owned(),
                        type_name: "string".to_owned(),
                    },
                ],
                statements: Vec::new(),
                return_type: "void".to_owned(),
            }]
        );
    }*/

    // Add more test functions for other statements here
}
