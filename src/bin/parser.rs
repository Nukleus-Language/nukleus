use pest::error::*;
use pest::iterators::Pair;
use pest::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::*;
use z3::ast::{Ast, Bool, BV};
use z3::{Config, Context, SatResult, Solver};
use crate::bin::ast::AstNode;
#[derive(Parser)]
#[grammar = "./cordium.pest"]
pub struct CdmParser<'a> {
    ctx: Context,
    pub funcs: Vec<Box<AstNode>>,
    variables: Vec<Vec<String>>,
    basis_t: Vec<Vec<(Box<Bool<'a>>, Vec<Box<BV<'a>>>)>>, // Supported arithmetic representation yet
    basis_f: Vec<Vec<Box<Bool<'a>>>>,
    stack: Vec<Box<AstNode>>,
}
pub fn numeric_eval(expr: AstNode, value: HashMap<String, i64>) -> i64 {
    match expr {
        AstNode::NumericTerm(lhs, rhs, op) => {
            let lhs_eval = numeric_eval(*lhs, value.clone());
            let rhs_eval = numeric_eval(*rhs, value.clone());
            match op {
                Ops::Add => lhs_eval + rhs_eval,
                Ops::Sub => lhs_eval - rhs_eval,
                Ops::Multply => lhs_eval * rhs_eval,
                Ops::Divide => lhs_eval / rhs_eval,
                Ops::And => lhs_eval & rhs_eval,
                Ops::Or => lhs_eval | rhs_eval,
                Ops::Xor => lhs_eval ^ rhs_eval,
                _ => {
                    // TODO
                    0
                }
            }
        }
        AstNode::Constant(c) => {
            match c {
                Constant::Number(x) => x,
                _ => {
                    // TODO
                    0
                }
            }
        }
        AstNode::Ident(x) => value[&x],
        _ => {
            // TODO
            0
        }
    }
}



pub fn parse_for(&mut self, expr: Pair<Rule>) -> AstNode {
       let pairs: Vec<Pair<Rule>> = expr.into_inner().collect();
       let expr = self.parse_boolean(pairs[0].clone());
       let block = self.parse_block(pairs[1].clone());
       AstNode::for(Box::new(expr), Box::new(block))
   }

   pub fn parse_expr(&mut self, expr: Pair<Rule>) -> AstNode {
       let pair = expr.into_inner().peek().unwrap();
       match pair.as_rule() {
           Rule::cdm_if => self.parse_if(pair),
           Rule::cdm_for => self.parse_for(pair),
           Rule::cdm_call => self.parse_call(pair),
           Rule::cdm_return => self.parse_return(pair),
           _ => AstNode::Todo,
       }
   }

   pub fn parse_if(&mut self, expr: Pair<Rule>) -> AstNode {
       let mut pair = expr.into_inner();
       let term = self.parse_boolean(pair.next().unwrap());
       let block = self.parse_block(pair.next().unwrap());
       AstNode::If(Box::new(term), Box::new(block))
   }

   pub fn parse_block(&mut self, block: Pair<Rule>) -> AstNode {
       let mut nodes = vec![];
       let pairs = block.into_inner();
       for pair in pairs {
           nodes.push(Box::new(self.parse_expr(pair)));
       }
       AstNode::Block(nodes)
   }

   pub fn parse_func_parm(&mut self, parm: Pair<Rule>) -> Variable {
       let mut pair = parm.into_inner();
       let var_name = pair.next().unwrap().as_str();
       let type_name = pair.next().unwrap().as_str();
       Variable::new(
           Box::new(AstNode::Ident(var_name.to_string())),
           VType::from_str(type_name),
       )
   }

   pub fn parse_parm(&mut self, parm: Pair<Rule>) -> AstNode {
       let mut pairs = parm.into_inner();
       let pair = pairs.next().unwrap();
       match pair.as_rule() {
           Rule::boolean_term => self.parse_boolean(pair),
           Rule::numeric_term => self.parse_numeric(pair),
           _ => AstNode::Todo,
       }
   }

   pub fn parse_call(&mut self, call: Pair<Rule>) -> AstNode {
       // TODO
       let mut call = call.into_inner();
       let name = AstNode::Ident(call.next().unwrap().as_str().to_string());
       let mut variables = vec![];
       while call.peek().is_some() && call.peek().unwrap().as_rule() == Rule::parm {
           variables.push(Box::new(self.parse_parm(call.next().unwrap())));
       }
       AstNode::Call(Box::new(name), variables)
   }

   pub fn parse_return(&mut self, _: Pair<Rule>) -> AstNode {
       // TODO
       AstNode::Return
   }

 #[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ops {
    Unknown,
    NotEqual,
    Equal,
    Leq,
    Geq,
    Less,
    Greater,
    And,
    Or,
    Xor,
    Add,
    Sub,
    Multply,
    Divide,
}

impl Ops {
    fn from_str(s: &str) -> Self {
        match s {
            "!=" => Self::NotEqual,
            "==" => Self::Equal,
            "<=" => Self::Leq,
            ">=" => Self::Geq,
            "<" => Self::Less,
            ">" => Self::Greater,
            "&&" | "&" => Self::And,
            "||" | "|" => Self::Or,
            "^" => Self::Xor,
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Divide,
            "*" => Self::Multply,
            _ => Self::Unknown,
        }
    }
}
