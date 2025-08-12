// #![no_std]
// pub mod compiler;
//pub mod compiler;

pub mod cores;
mod errors;
pub mod interpreter;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, Command};
use codegen::cranelift_jit::save_executable;
use codegen::cranelift_jit::JIT;
// use codegen::JIT;

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

    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            if ext_str != "nk" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Provided file is not a nukleus file",
                ));
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not convert file extension to string",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File has no extension",
        ));
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
    let input_option = matches.get_one::<String>("input");
    let input = match input_option {
        Some(input_str) => input_str,
        None => {
            eprintln!("Input argument not found");
            return;
        }
    };

    let mut interpreter = interpreter::Interpreter::new();
    if input == "repl" {
        interpreter.run_repl();
        return;
    }

    let contents = match read_file(input) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut new_lexer = lexer::lex_new::Lexer::new(&contents);
    // check the time between the two lexers
    let start_time_new = std::time::Instant::now();
    new_lexer.run();
    let end_time_new = std::time::Instant::now();
    let duration_new = end_time_new.duration_since(start_time_new);
    let new_tokens = new_lexer.get_tokens();

    let mut new_new_lexer =
        lexer::lex_new_new::Lexer::new(Path::new(input).to_path_buf(), &contents);
    let start_time_new_new = std::time::Instant::now();
    let lex_result = new_new_lexer.run();
    if let Err(e) = lex_result {
        println!("new_new_lexer Error: {}", e);
        return;
    }
    let end_time_new_new = std::time::Instant::now();
    let duration_new_new = end_time_new_new.duration_since(start_time_new_new);
    let new_new_tokens = new_new_lexer.get_tokens();

    // #[cfg(debug_assertions)]
    println!("Neo Lexer Time: {:?}", duration_new_new);

    let mut trie_lexer = lexer::trie_lex::Lexer::new(Path::new(input).to_path_buf(), &contents);
    let start_time_trie = std::time::Instant::now();
    let trie_result = trie_lexer.run();
    if let Err(e) = trie_result {
        println!("Trie Error: {}", e);
        return;
    }
    let end_time_trie = std::time::Instant::now();
    let duration_trie = end_time_trie.duration_since(start_time_trie);

    println!("Trie Lexer Time: {:?}", duration_trie);
    //println!("New Tokens: {:?}", new_tokens);
    // let start_time_old = std::time::Instant::now();
    // let tokens = lexer(&contents);
    // let end_time_old = std::time::Instant::now();
    // let duration_old = end_time_old.duration_since(start_time_old);
    #[cfg(debug_assertions)]
    println!("New Lexer Time: {:?}", duration_new);
    // println!("Old Lexer Time: {:?}", duration_old);
    #[cfg(debug_assertions)]
    {
        for token in &new_tokens {
            // println!("{}", token);
        }
    }
    // calulate how times faster the new lexer is
    // let speedup = duration_old.as_nanos() as f64 / duration_new.as_nanos() as f64;
    // println!("Speedup: {}x", speedup);
    // calculate how much characters the old lexer can lex per second
    // let old_chars_per_second = contents.len() as f64 / duration_old.as_secs_f64();
    // println!("Old Chars Per Second: {}", old_chars_per_second);
    // calculate how much characters the new lexer can lex per second
    let new_chars_per_second = contents.len() as f64 / duration_new.as_secs_f64();
    let new_chars_mb_per_second = new_chars_per_second * 4.0 / 1024.0 / 1024.0;
    let new_new_chars_per_second = contents.len() as f64 / duration_new_new.as_secs_f64();
    let new_new_chars_mb_per_second = new_new_chars_per_second * 4.0 / 1024.0 / 1024.0;
    #[cfg(debug_assertions)]
    {
        println!("New Chars Per Second: {}", new_chars_per_second);
        println!("New Chars MB/s: {}", new_chars_mb_per_second);
        println!("New New Chars Per Second: {}", new_new_chars_per_second);
        println!("New New Chars MB/s: {}", new_new_chars_mb_per_second);
    }
    // println!("New New Lexer Contents: {:?}", new_new_tokens);
    // println!("New Lexer Contents: {:?}", new_tokens);

    // println!("Tokens: {:?}", tokens);
    // let ast = core::parser_new::parse::Parser::new(tokens).parse();
    // println!("{:?}", ast);
    // Pass contents to the lexer here
    // let start_time_parser_old = std::time::Instant::now();
    // let end_time_parser_old = std::time::Instant::now();
    // let duration_parser_old = end_time_parser_old.duration_since(start_time_parser_old);
    // println!("Old Parser Time: {:?}", duration_parser_old);
    let mut mid_ir =
        astgen::parser_new::Parser::new(new_new_tokens, Path::new(input).to_path_buf(), &contents);

    let start_time_parser_new = std::time::Instant::now();
    let ast_result = mid_ir.run();
    if let Err(e) = ast_result {
        println!("Error: {}", e);
        return;
    }
    let end_time_parser_new = std::time::Instant::now();
    let ast_new = mid_ir.get_asts();

    let duration_parser_new = end_time_parser_new.duration_since(start_time_parser_new);
    #[cfg(debug_assertions)]
    println!("New Parser Time: {:?}", duration_parser_new);

    // let speedup = duration_parser_old.as_nanos() as f64 / duration_parser_new.as_nanos() as f64;
    // println!("Speedup: {}x", speedup);

    // let old_tokens_per_second = tokens.len() as f64 / duration_parser_old.as_secs_f64();
    // println!("Old Tokens Per Second: {}", old_tokens_per_second);
    let new_tokens_per_second = new_tokens.len() as f64 / duration_parser_new.as_secs_f64();
    #[cfg(debug_assertions)]
    println!("New Tokens Per Second: {}", new_tokens_per_second);

    //let old_tokens_mb_per_second = old_tokens_per_second / 1024.0 / 1024.0;
    //let new_tokens_mb_per_second = new_tokens_per_second / 1024.0 / 1024.0;
    //println!("Old Tokens MB/s: {}", old_tokens_mb_per_second);
    //println!("New Tokens MB/s: {}", new_tokens_mb_per_second);
    // println!("{:?}", ast);
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

    // println!("Mid IR code: ");
    // for ast in ast_new.clone() {
    // println!("{}", ast);
    // }
    // println!();

    // println!("{:?}",ast_new);
    //

    // println!("{}", generate_ir(ast_new));
    // generate_ir(ast_new);
    let start_time_jit = std::time::Instant::now();
    let mut jit = JIT::default();
    #[cfg(debug_assertions)]
    println!("JIT: ");
    let code_ptr_result = jit.compile(ast_new.clone(), input, false);
    if let Err(e) = code_ptr_result {
        println!("JIT Compile Error: {}", e);
        return;
    }
    let raw_code_ptr = code_ptr_result.unwrap();
    let end_time_jit = std::time::Instant::now();
    let duration_jit = end_time_jit.duration_since(start_time_jit);
    #[cfg(debug_assertions)]
    println!("JIT Compile Time: {:?}", duration_jit);
    println!();
    let pre_run_time = std::time::Instant::now();
    let result = match run(raw_code_ptr) {
        Ok(res) => res,
        Err(e) => {
            println!("Error during execution: {}", e);
            return;
        }
    };
    let duration = std::time::Instant::now().duration_since(pre_run_time);
    println!(
        "exit with code {} in {:?}",
        result,
        duration + duration_jit + duration_new_new + duration_parser_new
    );
    if let Err(e) = save_executable(raw_code_ptr, "a") {
        println!("Failed to save executable: {}", e);
    }
    drop(jit);
    drop(new_new_lexer);
    drop(new_lexer);
    drop(contents);

    // println!("Run TIme: {:?}", duration);
}

fn run(codeptr: *const u8) -> Result<isize, String> {
    unsafe { run_code(codeptr, ()) }
}

unsafe fn run_code<I, O>(codeptr: *const u8, input: I) -> Result<O, String> {
    if codeptr.is_null() {
        return Err("Null function pointer".to_string());
    }
    let code_fn = std::mem::transmute::<*const u8, fn(I) -> O>(codeptr);
    Ok(code_fn(input))
}
