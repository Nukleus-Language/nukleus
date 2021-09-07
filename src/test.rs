use crate::bin::parser::CdmParser;

#[test]
fn parse_numeric_with_boolean() {
    let mut ast: CdmParser = CdmParser::new();
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
