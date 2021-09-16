use crate::{Compile, Node, Ops, Result};

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<i64>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0i64;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }
}
struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }
    // ANCHOR: interpreter_eval
    pub fn eval(&self, node: &Node) -> i64 {
        match node {
            Node::Int(n) => *n,
            Node::NumericTerm(lhs, rhs, op) => {
                let lhs_eval = self.eval(lhs);
                let rhs_eval = self.eval(rhs);
                match op {
                    Ops::Add => lhs_eval + rhs_eval,
                    Ops::Sub => lhs_eval - rhs_eval,
                    Ops::Multply => lhs_eval * rhs_eval,
                    Ops::Divide => lhs_eval / rhs_eval,
                    Ops::And => lhs_eval & rhs_eval,
                    Ops::Or => lhs_eval | rhs_eval,
                    Ops::Xor => lhs_eval ^ rhs_eval,
                    _ => {
                        // TODO
                        0
                    }
                }
            }
            _ => {
                // TODO
                0
            }
        }
    }
}
#[cfg(test)]
//mod test;
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            Interpreter::from_source(
                "
                class fib(n: int) {
                    n=3;
                    return;
                }
                ",
            )
            .unwrap() as i64,
            0
        );
        assert_eq!(
            Interpreter::from_source("n=2;").unwrap() as i64,2);
    }
}
