//pub mod compiler;
//pub mod compiler;
pub mod core;
mod errors;
pub mod interpreter;

use std::fs::File;
use std::io::prelude::*;

use astgen::Parser;
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
        panic!("Provided file is not a nukleus file");
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
    let mut new_lexer = lexer::lex_new::Lexer::new(&contents);
    // check the time between the two lexers
    let start_time_new = std::time::Instant::now();
    new_lexer.run();
    let end_time_new = std::time::Instant::now();
    let duration_new = end_time_new.duration_since(start_time_new);
    let new_tokens = new_lexer.get_tokens();
    //println!("New Tokens: {:?}", new_tokens);
    let start_time_old = std::time::Instant::now();
    let tokens = lexer(&contents);
    let end_time_old = std::time::Instant::now();
    let duration_old = end_time_old.duration_since(start_time_old);

    println!("New Lexer Time: {:?}", duration_new);
    println!("Old Lexer Time: {:?}", duration_old);

    // calulate how times faster the new lexer is
    let speedup = duration_old.as_nanos() as f64 / duration_new.as_nanos() as f64;
    println!("Speedup: {}x", speedup);
    // calculate how much characters the old lexer can lex per second
    let old_chars_per_second = contents.len() as f64 / duration_old.as_secs_f64();
    println!("Old Chars Per Second: {}", old_chars_per_second);
    // calculate how much characters the new lexer can lex per second
    let new_chars_per_second = contents.len() as f64 / duration_new.as_secs_f64();
    println!("New Chars Per Second: {}", new_chars_per_second);
    // measure speed in mb/s
    let old_chars_mb_per_second = old_chars_per_second * 4.0 / 1024.0 / 1024.0;
    let new_chars_mb_per_second = new_chars_per_second * 4.0 / 1024.0 / 1024.0;
    println!("Old Chars MB/s: {}", old_chars_mb_per_second);
    println!("New Chars MB/s: {}", new_chars_mb_per_second);

    ///println!("Tokens: {:?}", tokens);
    //let ast = core::parser_new::parse::Parser::new(tokens).parse();
    //println!("{:?}", ast);
    // Pass contents to the lexer here
    let start_time_parser_old = std::time::Instant::now();
    let ast = core::parser::parse::Parser::new(&tokens).parse();
    let end_time_parser_old = std::time::Instant::now();
    let duration_parser_old = end_time_parser_old.duration_since(start_time_parser_old);
    println!("Old Parser Time: {:?}", duration_parser_old);

    let start_time_parser_new = std::time::Instant::now();
    Parser::new(&new_tokens).run();
    let end_time_parser_new = std::time::Instant::now();
    let duration_parser_new = end_time_parser_new.duration_since(start_time_parser_new);
    println!("New Parser Time: {:?}", duration_parser_new);

    let speedup = duration_parser_old.as_nanos() as f64 / duration_parser_new.as_nanos() as f64;
    println!("Speedup: {}x", speedup);
   

    let old_tokens_per_second = tokens.len() as f64 / duration_parser_old.as_secs_f64();
    println!("Old Tokens Per Second: {}", old_tokens_per_second);
    let new_tokens_per_second = tokens.len() as f64 / duration_parser_new.as_secs_f64();
    println!("New Tokens Per Second: {}", new_tokens_per_second);

    //let old_tokens_mb_per_second = old_tokens_per_second / 1024.0 / 1024.0;
    //let new_tokens_mb_per_second = new_tokens_per_second / 1024.0 / 1024.0;
    //println!("Old Tokens MB/s: {}", old_tokens_mb_per_second);
    //println!("New Tokens MB/s: {}", new_tokens_mb_per_second);
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
