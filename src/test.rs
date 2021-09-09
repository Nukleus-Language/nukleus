use crate::core::parser::parser;

#[test]
fn parse_numeric_with_boolean() {
    let mut ast: parser = parser::new();
    ast.run(
        "
        func fib(n: number) {
            if ((n == 1) || (n == 0)) { return; }
            call fib ((n-1));
            call fib ((n-2));
        }
    ",
    );
    dbg!(ast.eval(50, 5000, 0));
    //dbg!(ast.funcs.clone());
    panic!("tt");
}

#[test]
fn z3_test() {}
