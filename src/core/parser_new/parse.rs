use crate::core::ast_temp::{AST,AstParseError};
use crate::core::lexer::Tokens;

pub struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, AstParseError> {
        let mut ast = Vec::new();
        loop {
            if let Some(token) = self.tokens.get(self.pos) {
                match token {
                    Tokens::Function => {
                        ast.push(self.parse_function()?);
                    }
                    _ => {
                        return Err(AstParseError::UnknownToken {
                            token: token.to_string()
                        });
                    }
                }
            } else {
                return Err(AstParseError::EndofFile);
            }
        }
    }
    fn parse_statements(&mut self,return_type:&Tokens) -> Result<Vec<AST>,AstParseError>{
        let mut statements = Vec::new();

        while let Some(token) = self.tokens.get(self.pos) {
            match token {
                Tokens::Let => {
                    self.pos += 1;
                    if let Some(Tokens::Colon) = self.tokens.get(self.pos) {
                        self.pos += 1;
                        if let Some(Tokens::Identifier(type_name)) = self.tokens.get(self.pos) {
                            self.pos += 1;
                            if let Some(Tokens::Identifier(variable_name)) =
                                self.tokens.get(self.pos)
                            {
                                self.pos += 1;
                                if let Some(Tokens::Assign) = self.tokens.get(self.pos) {
                                    self.pos += 1;
                                    if let Some(Tokens::Integer(value)) =
                                        self.tokens.get(self.pos)
                                    {
                                        self.pos += 1;
                                        if let Some(Tokens::Semicolon) =
                                            self.tokens.get(self.pos)
                                        {
                                            self.pos += 1;
                                            let type_name = Some(type_name.to_string());
                                            let let_statement = AST::Let {
                                                name: variable_name.to_string(),
                                                type_name,
                                                value: Box::new(AST::Token(Tokens::Integer(
                                                    *value,
                                                ))),
                                            };
                                            statements.push(let_statement);
                                        } else {
                                            return Err(AstParseError::ExpectedOther { token: "Semicolon".to_owned() });
                                        }
                                    } else {
                                        return Err(AstParseError::ExpectedOther { token: "Value".to_owned() });
                                    }
                                } else {
                                    return Err(AstParseError::ExpectedOther { token: "Assignment Operator (=)".to_owned() });
                                }
                            } else {
                                return Err(AstParseError::ExpectedOther { token: "Varible Name".to_owned() });
                            }
                        } else {
                            return Err(AstParseError::ExpectedOther { token: "Type".to_owned() });
                        }
                    } else {
                        return Err(AstParseError::ExpectedOther { token: "Colon".to_owned() });
                    }
                }
                
                
                _ => return Err(AstParseError::UnknownToken {  token:token.to_string() }),
            }
        }
        Ok(statements)
    }
    fn parse_function(&mut self) -> Result<AST, AstParseError> {
        
        while let Some(token) = self.tokens.get(self.pos){
            match token {
                Tokens::Function => {
                    self.pos += 1;
                    if let Some(Tokens::Identifier(name)) = self.tokens.get(self.pos).clone() {
                        self.pos += 1;
                        if let Some(Tokens::OpenParen) = self.tokens.get(self.pos) {
                            self.pos += 1;
                            if let Some(Tokens::CloseParen) = self.tokens.get(self.pos) {
                                self.pos += 1;
                                if let Some(Tokens::OpenBrace) = self.tokens.get(self.pos) {
                                    self.pos += 1;
                                    if let Some(Tokens::Arrow) = self.tokens.get(self.pos){
                                        self.pos += 1;
                                        // return type : Void, Int, String, Bool, Float
                                        let return_type = self.tokens.get(self.pos).unwrap();
                                        if return_type == &Tokens::Void || return_type == &Tokens::Int || return_type == &Tokens::String || return_type == &Tokens::Bool || return_type == &Tokens::Float {
                                            if let Some(Tokens::CloseBrace) = self.tokens.get(self.pos) {
                                                self.pos += 1;
                                                let statements = self.parse_statements(return_type)?;
                                                let function = AST::Function {
                                                    public: false,
                                                    name: name.to_string(),
                                                    args: Vec::new(),
                                                    statements,
                                                    return_type: return_type.to_string(),
                                                };
                                                return Ok(function);
                                            } else {
                                                return Err(AstParseError::ExpectedOther { token: "CloseBrace".to_owned() });
                                            }
                                        }
                                    } else {
                                        return Err(AstParseError::ExpectedOther { token: "Arrow".to_owned() });
                                    }
                                } else {
                                    return Err(AstParseError::ExpectedOther { token: "OpenBrace".to_owned() });
                                }
                            } else {
                                return Err(AstParseError::ExpectedOther { token: "CloseParen".to_owned() });
                            }
                        } else {
                            return Err(AstParseError::ExpectedOther { token: "OpenParen".to_owned() });
                        }
                    } else {
                        return Err(AstParseError::ExpectedOther { token: "Function Name".to_owned() });
                    }
                }
                _ => return Err(AstParseError::UnknownToken {  token:token.to_string() }),
            }
        }

        return Err(AstParseError::EndofFile);     
    }

    
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::lexer::lexer;

    #[test]
    fn parser_let_statement() {
        let code = "let:int x = 3;";
        let tokens = lexer(code);
        let mut parser = Parser::new(tokens);
        let expected = vec![AST::Let {
            name: "x".to_string(),
            type_name: Some("int".to_string()),
            value: Box::new(AST::Token(Tokens::Integer(3))),
        }];

        // Compare each element in the vector to the expected value
        for (i, actual) in parser.parse().unwrap().iter().enumerate() {
            assert_eq!(actual, &expected[i]);
        }
    }
}
