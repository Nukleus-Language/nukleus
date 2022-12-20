use crate::core::parser::expression::FuncArgument;
use std::collections::HashMap;
//statement ast part
#[derive(Clone, Debug)]
pub enum Statement {
    Import {
        path: String,
        name: String,
    },
    Class {
        public: bool,
        name: String,
        attributes: Vec<(String, String)>,
    },
    Function {
        public: bool,
        name: String,
        parameters: Option<Vec<Parameter>>,
        returns: Option<Vec<String>>,
        body: Vec<Expression>,
    },
    Error,
}

//expression ast part
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Assign {
        identifier: String,
        kind: Option<String>,
        value: Box<Expression>,
    },
    FuncCall {
        identifier: String,
        arguments: Option<Vec<FuncArgument>>,
    },
    Value {
        as_string: String,
    },
    Null,
}

//apparatus ast part
#[derive(Debug)]
pub struct Apparatus {
    pub name: String,
    pub environment: HashMap<String, Statement>,
}

//function ast part
#[derive(Clone, Debug)]
pub struct Parameter {
    pub label: String,
    pub name: String,
    pub kind: String,
}
