use crate::core::parser;

#[test]
fn parse_numeric_with_boolean() {
    //let mut ast: parser = parser::new();
    parser::parse(
        "
        class fib(n: int) {
            if ((n == 1) || (n == 0)) { return; }
            call fib ((n-1));
            call fib ((n-2));
        }
    ",
    );

    //dbg!(ast.funcs.clone());
}
