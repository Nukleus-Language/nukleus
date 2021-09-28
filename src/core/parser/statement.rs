use pest::iterators::Pair;
use crate::parser::{ import, func, expression, class, Rule };



pub fn parse(statement: Pair<Rule>) -> Statement {
    let mut s: Statement = ast::Statement::Error;
    let mut _public: bool = false;
    for node in statement.into_inner() {
        match node.as_rule() {
            Rule::import => { s = import::parse(node)},
            Rule::func => { s = func::parse(node)},
            Rule::class => { s = class::parse(node)},
            _ => unreachable!()
        }
    }

    return s;
}
