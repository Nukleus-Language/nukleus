use pest::error::*;
use pest::iterators::Pair;
use pest::*;
use std::ops::*;
use z3::ast::{Ast, Bool, BV};
use z3::{Config, Context, Solver, SatResult};

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
