use std::fs;
use pest::{self, Parser};
use std::path::Path;

pub mod statement;
pub mod import;
pub mod function;
pub mod expression;
pub mod class;
pub mod apparatus;
pub mod string;

use crate::core::ast::Apparatus;
#[derive(pest_derive::Parser)]
#[grammar = "./nukleus-test.pest"]
pub struct NklParser;

pub fn parse(path: &str) -> Option<Apparatus> {
    let apparatus_name = Path::new(path)
        .file_stem()
        .expect("File not found")
        .to_str().expect("Error getting file name");

    let data = fs::read_to_string(path).expect("Unable to read file");
    let ast = NklParser::parse(Rule::apparatus, data.as_str())
        .unwrap_or_else(|e| panic!("{}", e));

    let mut parser_result: Option<Apparatus> = None;
    for node in ast {
        match node.as_rule() {
            Rule::apparatus => {
                parser_result = Some(apparatus::parse(apparatus_name, node));
            },
            _ => unreachable!()
        }
    }

    parser_result
}
