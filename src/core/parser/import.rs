use std::path::Path;
use pest::iterators::Pair;
use crate::parser::{
    Rule,
    string,
    statement::Statement
};

pub fn parse(import: Pair<Rule>) -> Statement {
    let mut path: String = String::new();
    let mut name: String = String::new();
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string_literal => path = string::parse(node),
            Rule::identifier => name = String::from(node.as_str()),
            _ => print!("")
        }
    }

    // files name is the module name (e.g. "types/dfs.nkl" becomes dfs)
    if name.is_empty() {
        let name_str = Path::new(path.as_str())
            .file_stem().expect("Error getting module name")
            .to_str().expect("Error parsing module name");
        name = String::from(name_str);
    }

    Statement::Import { path, name }
}
