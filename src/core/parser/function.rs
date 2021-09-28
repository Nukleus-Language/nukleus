use pest::iterators::Pair;
use crate::parser::{
    Rule,
    statement::Statement,
    expression::{ Expression, parse_expression }
};

#[derive(Clone, Debug)]
pub struct Parameter {
    pub label: String,
    pub name: String,
    pub kind: String
}

fn parse_param(param: Pair<Rule>) -> Parameter {
    let mut label = String::new();
    let mut name = String::new();
    let mut kind = String::new();

    for node in param.into_inner() {
        match node.as_rule() {
            Rule::label => label = String::from(node.as_str()),
            Rule::identifier => name = String::from(node.as_str()),
            Rule::kind => kind = String::from(node.as_str()),
            _ => println!("UNCHECKED RULE IN PARSE_PARAM: {:?}", node.as_rule())
        }
    }

    Parameter { label, name, kind }
}

fn parse_params(param_list: Pair<Rule>) -> Option<Vec<Parameter>> {
    let mut params: Vec<Parameter> = Vec::new();

    for node in param_list.into_inner() {
        match node.as_rule() {
            Rule::function_parameter => params.push(parse_param(node)),
            _ => println!("UNCHECKED RULE IN PARSE_PARAMS: {:?}", node.as_rule())
        }
    }

    Some(params)
}

fn parse_returns(return_list: Pair<Rule>) -> Option<Vec<String>> {
    let mut returns: Vec<String> = Vec::new();

    for node in return_list.into_inner() {
        match node.as_rule() {
            Rule::kind => returns.push(String::from(node.as_str())),
            _ => println!("UNCHECKED RULE IN PARSE_RETURNS: {:?}", node.as_rule())
        }
    }

    Some(returns)
}

pub fn parse(function: Pair<Rule>) -> Statement {
    let mut name = String::from("");
    let mut parameters = None;
    let mut returns = None;
    let mut body: Vec<Expression> = Vec::new();
    let mut public = false;

    for node in function.into_inner() {
        match node.as_rule() {
            Rule::public => public = true,
            Rule::identifier => name = String::from(node.as_str()),
            Rule::function_parameter_list => parameters = parse_params(node),
            Rule::returns => returns = parse_returns(node),
            Rule::expression => body.push(parse_expression(node)),
            _ => println!("UNCHECKED RULE IN PARSE: {:?}", node.as_rule())
        }
    }

    Statement::Function { public, name, parameters, returns, body }
}
