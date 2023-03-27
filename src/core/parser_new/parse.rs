use crate::core::ast_temp::{AstParseError, AST};
use crate::core::lexer::Tokens;
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Tokens>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Tokens]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, AstParseError> {
        let mut ast = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Tokens::Function => {
                    ast.push(self.parse_function(false)?);
                }
                Tokens::Public => {
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

    fn consume(&mut self) -> Option<&Tokens> {
        self.tokens.next()
    }

    fn expect(&mut self, expected: Tokens) -> Result<(), AstParseError> {
        if self.tokens.peek() == Some(&&expected) {
            Ok(())
        } else {
            println!("{:?}", self.tokens.peek());
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
        self.consume(); // Consume Function Name

        self.expect(Tokens::OpenParen)?;
        self.consume(); // Consume Tokens::OpenParen
        self.expect(Tokens::CloseParen)?;
        self.consume(); // Consume Tokens::CloseParen
        self.expect(Tokens::Arrow)?;
        self.consume(); // Consume Tokens::Arrow

        let return_type = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Return Type".to_owned(),
            })?;
        self.consume(); // Consume Return Type

        self.expect(Tokens::OpenBrace)?;
        self.consume(); // Consume Tokens::OpenBrace
       
        let statements = self.parse_statements(return_type)?;
        let mut return_value;
        if return_type != &Tokens::Void {
            self.expect(Tokens::Return)?;
            self.consume(); // Consume Tokens::Return
            return_value = self
                .tokens
                .peek()
                .cloned()
                .ok_or(AstParseError::ExpectedOther {
                    token: "Value".to_owned(),
                })?;
            self.consume(); // Consume Value
            self.expect(Tokens::Semicolon)?;
            self.consume(); // Consume Tokens::Semicolon
        }
        else if self.tokens.peek() == Some(&&Tokens::Return) {
            self.consume(); // Consume Tokens::Return
            self.expect(Tokens::Semicolon)?;
            self.consume(); // Consume Tokens::Semicolon
        }
        if let Some(Tokens::CloseBrace) = self.consume() {
            let function = AST::Function {
                public: is_public,
                name: name.to_string(),
                args: Vec::new(),
                statements,
                return_type: return_type.to_string(),
            };

            Ok(function)
        } else {
            Err(AstParseError::ExpectedOther {
                token: "CloseBrace".to_owned(),
            })
        }
    }

    fn parse_statements(&mut self, _return_type: &Tokens) -> Result<Vec<AST>, AstParseError> {
        let mut statements = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Tokens::Let => {
                    let let_statement = self.let_parser()?;        
                    statements.push(let_statement);
                },
                Tokens::Print => {
                    let print_statement = self.print_parser()?;
                    statements.push(print_statement);
                },
                Tokens::Println => {
                    let println_statement = self.println_parser()?;
                    statements.push(println_statement);
                },
                _ => break,
            }
        }
        Ok(statements)
    }
    
    fn let_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Let

        self.expect(Tokens::Colon)?;
        self.consume(); // Consume Tokens::Colon
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
        self.consume(); // Consume Variable Name
                        //
        self.expect(Tokens::Assign)?;
        self.consume(); // Consume Tokens::Assign

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon
                        //
        let let_statement = AST::Let {
            name: variable_name.to_string(),
            type_name: Some(type_name.to_string()),
            value: Box::new(value.clone()),
        };
        Ok(let_statement)
    }

    fn print_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Print

        self.expect(Tokens::OpenParen)?;
        self.consume(); // Consume Tokens::OpenParen

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Tokens::CloseParen)?;
        self.consume(); // Consume Tokens::CloseParen

        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon

        let print_statement = AST::Print {
            value: Box::new(value.clone()),
        };
        Ok(print_statement)
    }

    fn println_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Println

        self.expect(Tokens::OpenParen)?;
        self.consume(); // Consume Tokens::OpenParen

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Tokens::CloseParen)?;
        self.consume(); // Consume Tokens::CloseParen

        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon

        let println_statement = AST::Println {
            value: Box::new(value.clone()),
        };
        Ok(println_statement)
    }

    fn return_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::Return

        let value = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Value".to_owned(),
            })?;
        self.consume(); // Consume Value

        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon

        let return_statement = AST::Return {
            value: Box::new(value.clone()),
        };
        Ok(return_statement)
    }
}
