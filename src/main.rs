//pub mod compiler;
//pub mod compiler;

pub mod cores;
mod errors;
pub mod interpreter;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use astgen::{Parser, AST};
use clap::{Arg, Command};
use codegen::cranelift_JIT;
use codegen::JIT;
use core::mem;
use lexer::lexer;
// use inksac::types::*;

fn cli() -> Command {
    Command::new("nukleus")
        .version(env!("CARGO_PKG_VERSION"))
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

// fn run_interpreter_environment() {
// let _interpreter = interpreter::Interpreter::new();
//interpreter.run_repl();
// }

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

    // println!("Tokens: {:?}", tokens);
    // let ast = core::parser_new::parse::Parser::new(tokens).parse();
    // println!("{:?}", ast);
    // Pass contents to the lexer here
    let start_time_parser_old = std::time::Instant::now();
    let end_time_parser_old = std::time::Instant::now();
    let duration_parser_old = end_time_parser_old.duration_since(start_time_parser_old);
    println!("Old Parser Time: {:?}", duration_parser_old);
    let mut mid_ir = Parser::new(&new_tokens);

    let start_time_parser_new = std::time::Instant::now();
    mid_ir.run();
    let end_time_parser_new = std::time::Instant::now();
    let ast_new = mid_ir.get_asts();
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
    // let start_time_interpreter = std::time::Instant::now();
    // let mut interpreter = interpreter::Interpreter::new();
    // interpreter.run(ast.unwrap());
    // let end_time_interpreter = std::time::Instant::now();
    // let duration_interpreter = end_time_interpreter.duration_since(start_time_interpreter);
    // println!("Interpreter Time: {:?}", duration_interpreter);

    println!("NEW AST");
    for ast in ast_new.clone() {
        println!("{}", ast);
    }
    println!();

    //println!("{:?}",ast);
    //

    // println!("{}", generate_ir(ast_new));
    // generate_ir(ast_new);
    let start_time_jit = std::time::Instant::now();
    let mut jit = cranelift_JIT::JIT::default();
    let code_ptr = jit.compile(ast_new);
    let end_time_jit = std::time::Instant::now();
    let duration_jit = end_time_jit.duration_since(start_time_jit);
    println!("JIT Compile Time: {:?}", duration_jit);
    let pre_run_time = std::time::Instant::now();
    let result = run(&mut jit, code_ptr.unwrap()).unwrap();
    let duration = std::time::Instant::now().duration_since(pre_run_time);
    println!("result {} ", result);
    println!("JIT Run TIme: {:?}", duration);
}

fn run(jit: &mut cranelift_JIT::JIT, codeptr: *const u8) -> Result<isize, String> {
    unsafe { run_code(codeptr, ()) }
}
unsafe fn run_code<I, O>(codeptr: *const u8, input: I) -> Result<O, String> {
    // Pass the string to the JIT, and it returns a raw pointer to machine code.

    let code_fn = mem::transmute::<_, fn(I) -> O>(codeptr);
    Ok(code_fn(input))
}
