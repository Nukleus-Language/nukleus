//pub mod compiler;
pub mod core;

use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let contents = read_file("input.nkl").unwrap();

    core::lexer::lexer(&contents);
    // Pass contents to the lexer here
}
