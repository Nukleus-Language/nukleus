use crate::core::ast_temp::{AstParseError, AST};
use crate::core::lexer::{Tokens, Operator, Assigns, TypeName, TypeValue};
use std::iter::{Cloned, Peekable};

pub struct Parser<'a> {
    tokens: Peekable<Cloned<std::slice::Iter<'a, Tokens>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Tokens]) -> Self {
        Parser {
            tokens: tokens.iter().cloned().peekable(),
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

    fn consume(&mut self) -> Option<Tokens> {
        self.tokens.next()
    }

    fn expect(&mut self, expected: Tokens) -> Result<(), AstParseError> {
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

        let statements = self.parse_statements(return_type.clone())?;
        let mut return_value = Tokens::TypeValue(TypeValue::None);
        if return_type != Tokens::TypeName(TypeName::Void) {
            self.expect(Tokens::Return)?;
            self.consume(); // Consume Tokens::Return

            return_value = self
                .tokens
                .peek()
                .cloned()
                .ok_or(AstParseError::ExpectedOther {
                    token: "Return Value".to_owned(),
                })?;
            self.consume(); // Consume Value
            self.expect(Tokens::Semicolon)?;
            self.consume(); // Consume Tokens::Semicolon
        } else if self.tokens.peek() == Some(&Tokens::Return) {
            self.consume(); // Consume Tokens::Return
            self.expect(Tokens::Semicolon)?;
            self.consume(); // Consume Tokens::Semicolon
        }
        self.expect(Tokens::CloseBrace)?;
        self.consume(); // Consume Tokens::CloseBrace

        let function = AST::Function {
            public: is_public,
            name: name.to_string(),
            args: Vec::new(),
            statements,
            return_type,
            return_value,
        };

        Ok(function)
    }

    fn parse_statements(&mut self, _return_type: Tokens) -> Result<Vec<AST>, AstParseError> {
        let mut statements = Vec::new();
        //println!("{:?}", self.tokens.peek());
        while let Some(token) = self.tokens.peek() {
            match token {
                Tokens::Let => {
                    let let_statement = self.let_parser()?;
                    statements.push(let_statement);
                }
                Tokens::Print => {
                    let print_statement = self.print_parser()?;
                    statements.push(print_statement);
                }
                Tokens::Println => {
                    let println_statement = self.println_parser()?;
                    statements.push(println_statement);
                }
                Tokens::For => {
                    let for_statement = self.for_parser()?;
                    statements.push(for_statement);
                }
                Tokens::If => {
                    let if_statement = self.if_parser()?;
                    statements.push(if_statement);
                }
                Tokens::Identifier(_) => {
                    let assign_statement = self.assign_parser()?;
                    statements.push(assign_statement);
                }
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
        match variable_name.is_identifier() {
            false => {
                return Err(AstParseError::ExpectedOther {
                    token: "Variable Name".to_owned(),
                })
            }
            true => (),
        }
        self.consume(); // Consume Variable Name
                        //
        self.expect(Tokens::Assigns(Assigns::Assign))?;
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

        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon
                        //
        let let_statement = AST::Let {
            name: variable_name.to_string(),
            type_name: Some(type_name.to_string()),
            value,
        };
        Ok(let_statement)
    }
    fn assign_parser(&mut self) -> Result<AST, AstParseError> {
        let variable_name = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable Name".to_owned(),
            })?;
        self.consume(); // Consume Variable Name

        self.expect(Tokens::Assigns(Assigns::Assign))?;
        self.consume(); // Consume Tokens::Assign

        let mut value: Vec<Tokens> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token == &Tokens::Semicolon {
                break;
            }
            value.push(token.clone());
            self.consume();
        }
        self.expect(Tokens::Semicolon)?;
        self.consume(); // Consume Tokens::Semicolon

        let assign_statement = AST::Assign {
            name: variable_name.to_string(),
            value,
        };
        Ok(assign_statement)
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

        let print_statement = AST::Print { value };
        Ok(print_statement)
    }
    fn if_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::If

        self.expect(Tokens::OpenParen)?;
        self.consume(); // Consume Tokens::OpenParen

        let mut condition: Vec<Tokens> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token == &Tokens::CloseParen {
                break;
            }
            condition.push(token.clone());
            self.consume();
        }
        self.expect(Tokens::CloseParen)?;
        self.consume(); // Consume Tokens::CloseParen

        self.expect(Tokens::OpenBrace)?;
        self.consume(); // Consume Tokens::OpenBrace

        let statements = self.parse_statements(Tokens::Void)?;
        self.expect(Tokens::CloseBrace)?;
        self.consume(); // Consume Tokens::CloseBrace

        let if_statement = AST::If {
            condition,
            statements,
        };
        Ok(if_statement)
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

        let println_statement = AST::Println { value };
        Ok(println_statement)
    }

    fn for_parser(&mut self) -> Result<AST, AstParseError> {
        self.consume(); // Consume Tokens::For

        self.expect(Tokens::OpenParen)?;
        self.consume(); // Consume Tokens::OpenParen

        let start_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Tokens::Arrow)?;
        self.consume(); // Consume Tokens::Arrow

        let end_variable = self
            .tokens
            .peek()
            .cloned()
            .ok_or(AstParseError::ExpectedOther {
                token: "Variable".to_owned(),
            })?;
        self.consume(); // Consume Variable

        self.expect(Tokens::DoubleColon)?;
        self.consume(); // Consume Tokens::DoubleColon

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

        self.expect(Tokens::OpenBrace)?;
        self.consume(); // Consume Tokens::OpenBrace

        let statements = self.parse_statements(Tokens::Void)?;

        self.expect(Tokens::CloseBrace)?;
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

        let return_statement = AST::Return { value };
        Ok(return_statement)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::ast_temp::AST;
    use crate::core::lexer::{Tokens, TypeValue, TypeName};

    #[test]
    fn test_parse_function() {
        let tokens = vec![
            Tokens::Function,
            Tokens::Identifier("main".to_owned()),
            Tokens::OpenParen,
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
                name: "main".to_owned(),
                args: Vec::new(),
                statements: Vec::new(),
                return_type: Tokens::TypeName(TypeName::Void),
                return_value: Tokens::TypeValue(TypeValue::None),
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
