use crate::core::parser::Rule;
use pest::iterators::Pair;

pub fn parse(import: Pair<Rule>) -> String {
    let mut string = String::new();
    for node in import.clone().into_inner() {
        match node.as_rule() {
            Rule::string => string = String::from(node.as_str()),
            _ => unreachable!(),
        }
    }

    string
}

#[cfg(test)]
mod tests {

    use crate::core::parser::{string, NklParser, Rule};
    use pest::Parser;

    #[test]
    fn string_literal() {
        let test_literal = "\"Hello world!\"";
        let test_ast = NklParser::parse(Rule::string_literal, test_literal)
            .unwrap_or_else(|e| panic!("{}", e));

        for node in test_ast {
            assert_eq!(string::parse(node), String::from("Hello world!"));
        }
    }
}
