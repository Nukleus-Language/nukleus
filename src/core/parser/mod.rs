use std::fs;
use pest::Parser;
use std::path::Path;

pub mod statement;
pub mod import;
pub mod function;


#[derive(Parser)]
#[grammar = "./nukleus-test.pest"]
pub struct NklParser;
