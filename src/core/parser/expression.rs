use pest::iterators::Pair;
use crate::parser::Rule;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Assign {
        identifier: String,
        kind: Option<String>,
        value: Box<Expression>
    },
    FuncCall {
        identifier: String,
        arguments: Option<Vec<FuncArgument>>
    },
    Value { as_string: String },
    Null
}

#[derive(Clone, Debug)]
pub struct FuncArgument {
    label: Option<String>,
    value: Expression
}

fn parse_fn_arg(arg: Pair<Rule>) -> FuncArgument {
    let mut label = None;
    let mut value = Expression::Null;

    for node in arg.into_inner() {
        match node.as_rule() {
            Rule::label => label = Some(String::from(node.as_str())),
            Rule::expression => value = parse_expression(node),
            _ => println!("UNCHECKED RULE: {:?}", node.as_rule())
        }
    }

    FuncArgument { label, value }
}

fn parse_fn_args(arg_list: Pair<Rule>) -> Option<Vec<FuncArgument>> {
    let mut args: Vec<FuncArgument> = Vec::new();

    for node in arg_list.into_inner() {
        match node.as_rule() {
            Rule::function_arg => args.push(parse_fn_arg(node)),
            _ => unreachable!()
        }
    }

    Some(args)
}

fn parse_function_call(fn_call: Pair<Rule>) -> Expression {
    let mut identifier = String::new();
    let mut arguments = None;

    for node in fn_call.into_inner() {
        match node.as_rule() {
            Rule::identifier => identifier = String::from(node.as_str()),
            Rule::function_arg_list => arguments = parse_fn_args(node),
            _ => println!("UNCHECKED RULE: {:?}", node.as_rule())
        }
    }

    Expression::FuncCall { identifier, arguments }
}

fn parse_assignment(expr: Pair<Rule>) -> Expression {
    let mut identifier = String::new();
    let mut value = Box::new(Expression::Null);
    let mut kind = None;

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::identifier => identifier = String::from(node.as_str()),
            Rule::kind => kind = Some(String::from(node.as_str())),
            Rule::expression => {
                let parsed_value = parse_expression(node);
                value = Box::new(parsed_value);
            },
            _ => unreachable!()
        }
    }

    Expression::Assign { identifier, value, kind }
}

pub fn parse_expression(expr: Pair<Rule>) -> Expression {
    let mut new_expr = Expression::Null;

    for node in expr.into_inner() {
        match node.as_rule() {
            Rule::function_call => new_expr = parse_function_call(node),
            Rule::assignment => new_expr = parse_assignment(node),
            Rule::value => {
                new_expr = Expression::Value {
                    as_string: String::from(node.as_str())
                }
            },
            _ => println!("UNCHECKED RULE: {:?}", node.as_rule())
        }
    }

    new_expr
}
