//pub mod compiler;
//pub mod compiler;
pub mod core;
mod errors;
pub mod interpreter;

use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, Command};
use lexer::lexer;

fn cli() -> Command {
    Command::new("nukleus")
        .version("0.1.0 Nightly 2023-04")
        .author("Skuld Norniern. <skuldnorniern@gmail.com>")
        .about("Nukleus Language")
        .arg(Arg::new("input").default_value("repl"))
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    // Get the file
    let file_path = std::path::Path::new(filename);

    let file_extension = file_path.extension().unwrap().to_str().unwrap();
    if file_extension != "nk" {
        panic!("Provided file is not a .nk file");
    }
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run_interpreter_environment() {
    let _interpreter = interpreter::Interpreter::new();
    //interpreter.run_repl();
}

fn main() {
    let matches = cli().get_matches();
    let input = matches.get_one::<String>("input").unwrap();
    let mut interpreter = interpreter::Interpreter::new();
    if input == "repl" {
        interpreter.run_repl();
        return;
    }
    let contents = read_file(input).unwrap();
    //let contents = read_file("input.nk").unwrap();
    //let contents = "public fn main() -> void\n{\nlet:int a = 3;}";
    //println!("Input: {}", contents);

    let tokens = lexer(&contents);
    //println!("Tokens: {:?}", tokens);
    //let ast = core::parser_new::parse::Parser::new(tokens).parse();
    //println!("{:?}", ast);
    // Pass contents to the lexer here
    let ast = core::parser::parse::Parser::new(&tokens).parse();
    //println!("{:?}", ast);
    /*match ast.clone() {
        Ok(ast) => {
            println!("AST Tree: {:?}", ast;
            //let mut interpreter = interpreter::Interpreter::new();
            //interpreter.run(ast);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }*/
    //println!("{:?}", ast);

    //let compiled = compiler::compile::compile_and_run(ast.unwrap());
    //let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(ast.unwrap());

    //println!("{:?}",ast);
}
