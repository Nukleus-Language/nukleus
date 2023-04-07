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
