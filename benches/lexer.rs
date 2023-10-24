use divan::{
    counter::{BytesCount, ItemsCount},
    Bencher,
};
const TEST_CODE: &str = include_str!("../test.nk");

fn main() {
    divan::main()
}

#[divan::bench(counters = [
    BytesCount::of_slice(TEST_CODE),
    ItemsCount::new(TEST_CODE.len()),
],threads = [0, 1, 4, 8, 16])]
fn new_lexer() {
    fn compute(code: &str) {
        let mut lexer = lexer::lex_new::Lexer::new(code);
        let tokens = lexer.run();
    }
    compute(divan::black_box(TEST_CODE))
}

#[divan::bench(counters = [
    BytesCount::of_slice(TEST_CODE),
    ItemsCount::new(TEST_CODE.len()),
],threads = [0, 1, 4, 8, 16])]
fn old_lexer() {
    fn compute(code: &str) {
        let tokens = lexer::lexer(code);
    }
    compute(divan::black_box(TEST_CODE))
}
