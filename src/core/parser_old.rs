use pest::{self, Parser};

use crate::core::ast::{Constant, Node, Ops, VType, Variable};

#[derive(pest_derive::Parser)]
#[grammar = "nukleus.pest"]
struct Nksparser {
    pub class: Vec<Box<Node>>,
    variables: Vec<Vec<String>>,
}

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let ast = vec![];
    let pairs = Nksparser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::top_cmnd => {
                let mut pair = pair.into_inner();
                parse_top_cmnd(pair.next().unwrap());
            }
            Rule::block=> {
                let mut pair = pair.into_inner();
                parse_block(pair.next().unwrap());
            }
            _ => {
                // TODO: unexpected
            }
        }
    }
    Ok(ast)
}
fn parse_expr(expr: pest::iterators::Pair<Rule>) -> Node {
    let pair = expr.into_inner().peek().unwrap();
    match pair.as_rule() {
        Rule::nks_if => parse_if(pair),
        Rule::nks_for => parse_for(pair),
        Rule::nks_call => parse_call(pair),
        Rule::nks_return => parse_return(pair),
        _ => Node::Todo,
    }
}
fn parse_call(call: pest::iterators::Pair<Rule>) -> Node {
    let mut call = call.into_inner();
    let name = Node::Ident(call.next().unwrap().as_str().to_string());
    let mut variables = vec![];
    while call.peek().is_some() && call.peek().unwrap().as_rule() == Rule::parm {
        variables.push(Box::new(parse_parm(call.next().unwrap())));
    }
    Node::Call(Box::new(name), variables)
}
fn parse_for(expr: pest::iterators::Pair<Rule>) -> Node {
    let pairs: Vec<pest::iterators::Pair<Rule>> = expr.into_inner().collect();
    let expr = parse_boolean(pairs[0].clone());
    let block = parse_block(pairs[1].clone());
    Node::Loop(Box::new(expr), Box::new(block))
}
fn parse_if(expr: pest::iterators::Pair<Rule>) -> Node {
    let mut pair = expr.into_inner();
    let term = parse_boolean(pair.next().unwrap());
    let block = parse_block(pair.next().unwrap());
    Node::If(Box::new(term), Box::new(block))
}
pub fn parse_block(block: pest::iterators::Pair<Rule>) -> Node {
    let mut nodes = vec![];
    let pairs = block.into_inner();
    for pair in pairs {
        nodes.push(Box::new(parse_expr(pair)));
    }
    Node::Block(nodes)
}
pub fn parse_func_parm(parm: pest::iterators::Pair<Rule>) -> Variable {
    let mut pair = parm.into_inner();
    let var_name = pair.next().unwrap().as_str();
    let type_name = pair.next().unwrap().as_str();
    Variable::new(
        Box::new(Node::Ident(var_name.to_string())),
        VType::from_str(type_name),
    )
}
pub fn parse_return(_: pest::iterators::Pair<Rule>) -> Node {
    // TODO
    Node::Return
}
pub fn parse_numeric(expr: pest::iterators::Pair<Rule>) -> Node {
    let pairs: Vec<pest::iterators::Pair<Rule>> = expr.into_inner().collect();
    if pairs.len() == 1 {
        match pairs[0].as_rule() {
            Rule::ident => Node::Ident(pairs[0].as_str().to_string()),
            Rule::constants => Node::Constant(Constant::parse_numeric(pairs[0].as_str())),
            Rule::numeric_term => parse_numeric(pairs[0].clone()),
            _ => Node::Todo,
        }
    } else {
        match pairs[0].as_rule() {
            Rule::numeric_term => {
                assert_eq!(pairs[2].as_rule(), Rule::numeric_term);
                let op = Ops::from_str(pairs[1].as_str());
                let lhs = parse_numeric(pairs[0].clone());
                let rhs = parse_numeric(pairs[2].clone());
                Node::NumericTerm(Box::new(lhs), Box::new(rhs), op)
            }
            _ => Node::Todo,
        }
    }
}
pub fn parse_boolean(expr: pest::iterators::Pair<Rule>) -> Node {
    let pairs: Vec<pest::iterators::Pair<Rule>> = expr.into_inner().collect();
    if pairs.len() == 1 {
        match pairs[0].as_rule() {
            Rule::ident => Node::Ident(pairs[0].as_str().to_string()),
            Rule::constants => Node::Constant(Constant::parse_bool(pairs[0].as_str())),
            Rule::boolean_term => parse_boolean(pairs[0].clone()),
            _ => Node::Todo,
        }
    } else {
        let mut i = 0;
        let mut res = Node::Todo;
        while pairs.len() > i {
            if i == 0 {
                match pairs[i].as_rule() {
                    Rule::boolean_term => {
                        // TODO
                        res = parse_boolean(pairs[i].clone());
                    }
                    Rule::numeric_term => {
                        res = parse_numeric(pairs[i].clone());
                    }
                    _ => {}
                }
            } else {
                match pairs[i].as_rule() {
                    Rule::boolean_term => {
                        // TODO
                        let op = Ops::from_str(pairs[i - 1].as_str());
                        let rhs = parse_boolean(pairs[i].clone());
                        res = Node::BooleanTerm(Box::new(res.clone()), Box::new(rhs), op);
                    }
                    Rule::numeric_term => {
                        let op = Ops::from_str(pairs[i - 1].as_str());
                        let rhs = parse_numeric(pairs[i].clone());
                        res = Node::BooleanTerm(Box::new(res.clone()), Box::new(rhs), op);
                    }
                    _ => {}
                }
            }
            i += 2;
        }
        res
    }
}
impl Nksparser {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            class: vec![],
        }
    }
}
pub fn register_class(func: pest::iterators::Pair<Rule>) {
    // TODO
    let mut sause = Nksparser::new();
    let mut func = func.into_inner();
    let name = Node::Ident(func.next().unwrap().as_str().to_string());
    let mut variables = vec![];
    while func.peek().unwrap().as_rule() == Rule::parm_with_type {
        variables.push(parse_func_parm(func.next().unwrap()));
    }
    let block = parse_block(func.next().unwrap());
    sause.variables.push(vec![]);
    for var in variables.clone() {
        if let Node::Ident(name) = *var.name {
            sause.variables[sause.class.len()].push(name);
        }
    }
    sause.class.push(Box::new(Node::Class(
        Box::new(name),
        variables,
        Box::new(block),
    )));
}
pub fn parse_parm(parm: pest::iterators::Pair<Rule>) -> Node {
    let mut pairs = parm.into_inner();
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::boolean_term => parse_boolean(pair),
        Rule::numeric_term => parse_numeric(pair),
        _ => Node::Todo,
    }
}
pub fn parse_top_cmnd(expr: pest::iterators::Pair<Rule>) {
    match expr.as_rule() {
        Rule::class => {
            register_class(expr);
        }
        _ => {}
    }
}
