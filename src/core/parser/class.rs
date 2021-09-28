use pest::iterators::Pair;
use crate::parser::{ Rule, statement::Statement };

fn parse_attribute(attr: Pair<Rule>) -> (String, String) {
    let mut name = String::new();
    let mut kind = String::new();

    for node in attr.into_inner() {
        match node.as_rule() {
            Rule::colon => (),
            Rule::identifier => name = String::from(node.as_str()),
            Rule::kind => kind = String::from(node.as_str()),
            _ => unreachable!()
        }
    }

    (name, kind)
}

fn parse_attr_list(attrs: Pair<Rule>) -> Vec<(String, String)> {
    let mut results: Vec<(String, String)> = Vec::new();

    for node in attrs.into_inner() {
        match node.as_rule() {
            Rule::comma => (),
            Rule::attribute => results.push(parse_attribute(node)),
            _ => unreachable!()
        }
    }

    results
}

pub fn parse(custom_type: Pair<Rule>) -> Statement {
    let mut name = String::new();
    let mut attributes: Vec<(String, String)> = Vec::new();
    let mut public: bool = false;

    for node in custom_type.into_inner() {
        match node.as_rule() {
            Rule::identifier => name = String::from(node.as_str()),
            Rule::attribute_list => attributes = parse_attr_list(node),
            Rule::public => public = true,
            _ => print!("")
        }
    }

    Statement::Type { name, attributes, public }
}
