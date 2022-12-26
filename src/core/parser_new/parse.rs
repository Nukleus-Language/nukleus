use crate::core::ast_temp::ast::AST;
use crate::core::lexer::Tokens;

pub struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, &'static str> {
        let mut statements = Vec::new();

        while let Some(token) = self.tokens.get(self.pos) {
            match token {
                Tokens::Let => {
                    self.pos += 1;
                    if let Some(Tokens::OpenAngle) = self.tokens.get(self.pos) {
                        self.pos += 1;
                        if let Some(Tokens::Identifier(type_name)) = self.tokens.get(self.pos) {
                            self.pos += 1;
                            if let Some(Tokens::CloseAngle) = self.tokens.get(self.pos) {
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
                                                return Err("Expected semicolon after assignment");
                                            }
                                        } else {
                                            return Err("Expected integer value after assignment");
                                        }
                                    } else {
                                        return Err("Expected assignment operator (=) after type");
                                    }
                                } else {
                                    return Err("Expected varible name after type");
                                }
                            } else {
                                return Err("Expected closing angle bracket after type");
                            }
                        } else {
                            return Err("Expected type after opening angle bracket");
                        }
                    } else {
                        return Err("Expected opening angle bracket after identifier");
                    }
                }
                _ => return Err("Unexpected token"),
            }
        }
        Ok(statements)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::lexer::lexer;

    #[test]
    fn parser_let_statement() {
        let code = "let<int> x = 3;";
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
