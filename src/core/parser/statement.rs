use crate::core::ast::Statement;
use crate::core::parser::{class, function, import, Rule};
use pest::iterators::Pair;

pub fn parse(statement: Pair<Rule>) -> Statement {
    let mut s: Statement = Statement::Error;
    let mut _public: bool = false;
    for node in statement.into_inner() {
        match node.as_rule() {
            Rule::import => s = import::parse(node),
            Rule::func => s = function::parse(node),
            Rule::class => s = class::parse(node),
            _ => unreachable!(),
        }
    }

    s
}
