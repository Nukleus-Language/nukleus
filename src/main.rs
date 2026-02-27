// #![no_std]
// pub mod compiler;
//pub mod compiler;
#![allow(clippy::cognitive_complexity, clippy::needless_borrow)]

#[cfg(feature = "legacy")]
pub mod cores;
mod errors;
#[cfg(feature = "legacy")]
pub mod interpreter;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command as ProcessCommand;

use clap::{Arg, ArgAction, Command};
#[cfg(feature = "jit")]
use codegen::cranelift_jit::{save_executable, JIT};
use codegen::lamina::LaminaBackend;
// use codegen::JIT;

// use inksac::types::*;

fn cli() -> Command {
    Command::new("nukleus")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Skuld Norniern. <skuldnorniern@gmail.com>")
        .about("Nukleus Language")
        .arg(Arg::new("input").default_value("repl"))
        .arg(
            Arg::new("backend")
                .long("backend")
                .short('b')
                .default_value("lamina")
                .value_parser(["cranelift", "lamina"]),
        )
        .arg(
            Arg::new("emit-asm")
                .long("emit-asm")
                .value_name("PATH")
                .help("When using --backend lamina, write assembly to this path"),
        )
        .arg(
            Arg::new("emit-ir")
                .long("emit-ir")
                .value_name("PATH")
                .help("When using --backend lamina, write Lamina IR to this path"),
        )
        .arg(
            Arg::new("lamina")
                .long("lamina")
                .action(ArgAction::SetTrue)
                .help("Use Lamina backend (shorthand for --backend lamina)"),
        )
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
    if let Err(e) = run_cli() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), String> {
    let matches = cli().get_matches();
    let input = matches
        .get_one::<String>("input")
        .ok_or("Input argument not found")?;
    let mut backend = matches
        .get_one::<String>("backend")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "lamina".to_string());
    let emit_asm_path = matches.get_one::<String>("emit-asm").map(String::as_str);
    let emit_ir_path = matches.get_one::<String>("emit-ir").map(String::as_str);

    if matches.get_flag("lamina") {
        backend = "lamina".to_string();
    }

    if backend == "cranelift" && !cfg!(target_arch = "x86_64") {
        println!(
            "cranelift backend is not supported on this architecture yet; falling back to lamina"
        );
        backend = "lamina".to_string();
    }

    if input == "repl" {
        #[cfg(feature = "legacy")]
        {
            let mut interpreter = interpreter::Interpreter::new();
            interpreter.run_repl();
            return Ok(());
        }
        #[cfg(not(feature = "legacy"))]
        {
            return Err("REPL requires legacy feature. Build with: cargo build --features legacy".to_string());
        }
    }

    let contents = read_file(input).map_err(|e| format!("Error reading file: {}", e))?;

    let input_path = Path::new(input);
    let lex_start = std::time::Instant::now();
    let mut lexer = lexer::frontend::Lexer::from_path(input_path, &contents);
    lexer.run().map_err(|e| format!("Lexer error: {}", e))?;
    let lex_duration = lex_start.elapsed();
    println!("Lexer Time: {:?}", lex_duration);
    let tokens = lexer.tokens().to_vec();

    #[cfg(debug_assertions)]
    {
        let chars_per_second = contents.len() as f64 / lex_duration.as_secs_f64();
        let chars_mb_per_second = chars_per_second * 4.0 / 1024.0 / 1024.0;
        println!("Lexer Chars Per Second: {}", chars_per_second);
        println!("Lexer Chars MB/s: {}", chars_mb_per_second);
    }

    // println!("Tokens: {:?}", tokens);
    // let ast = core::parser_new::parse::Parser::new(tokens).parse();
    // println!("{:?}", ast);
    // Pass contents to the lexer here
    // let start_time_parser_old = std::time::Instant::now();
    // let end_time_parser_old = std::time::Instant::now();
    // let duration_parser_old = end_time_parser_old.duration_since(start_time_parser_old);
    // println!("Old Parser Time: {:?}", duration_parser_old);
    let mut mid_ir = astgen::parser_new::Parser::new(&tokens, input_path.to_path_buf(), &contents);

    let start_time_parser_new = std::time::Instant::now();
    mid_ir.run().map_err(|e| e.to_diagnostic().to_string())?;
    let end_time_parser_new = std::time::Instant::now();
    let ast_new = mid_ir.get_asts();

    let duration_parser_new = end_time_parser_new.duration_since(start_time_parser_new);
    #[cfg(debug_assertions)]
    println!("New Parser Time: {:?}", duration_parser_new);

    // let speedup = duration_parser_old.as_nanos() as f64 / duration_parser_new.as_nanos() as f64;
    // println!("Speedup: {}x", speedup);

    // let old_tokens_per_second = tokens.len() as f64 / duration_parser_old.as_secs_f64();
    // println!("Old Tokens Per Second: {}", old_tokens_per_second);
    #[cfg(debug_assertions)]
    {
        let new_tokens_per_second = tokens.len() as f64 / duration_parser_new.as_secs_f64();
        println!("New Tokens Per Second: {}", new_tokens_per_second);
    }

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
    if backend == "lamina" {
        let compile_start = std::time::Instant::now();
        let mut lamina_backend = LaminaBackend::new();
        let ir = lamina_backend
            .compile_ast_to_ir(ast_new)
            .map_err(|e| format!("Lamina Compile Error: {}", e))?;
        let ir_path = output_ir_path(input, emit_ir_path);
        let asm_path = output_asm_path(input, emit_asm_path);
        ensure_parent_dir(&ir_path)
            .map_err(|e| format!("Failed to prepare IR output directory for '{}': {}", ir_path, e))?;
        fs::write(&ir_path, &ir)
            .map_err(|e| format!("Failed to write Lamina IR '{}': {}", ir_path, e))?;
        let assembly = lamina_backend
            .compile_ir_to_assembly(&ir, None)
            .map_err(|e| format!("Lamina Assembly Error: {}", e))?;
        let compile_duration = compile_start.elapsed();

        ensure_parent_dir(&asm_path).map_err(|e| {
            format!(
                "Failed to prepare assembly output directory for '{}': {}",
                asm_path, e
            )
        })?;
        fs::write(&asm_path, &assembly)
            .map_err(|e| format!("Failed to write assembly '{}': {}", asm_path, e))?;

        let bin_path = output_bin_path(input);
        ensure_parent_dir(&bin_path).map_err(|e| {
            format!(
                "Failed to prepare binary output directory for '{}': {}",
                bin_path, e
            )
        })?;
        let link_start = std::time::Instant::now();
        let link_status = ProcessCommand::new("cc")
            .arg(&asm_path)
            .arg("-o")
            .arg(&bin_path)
            .status();
        let link_duration = link_start.elapsed();

        match link_status {
            Ok(status) if status.success() => {}
            Ok(status) => {
                return Err(format!(
                    "Lamina link failed with status {}. Assembly was saved at {}",
                    status, asm_path
                ));
            }
            Err(e) => {
                return Err(format!(
                    "Failed to run system linker (cc): {}. Assembly was saved at {}",
                    e, asm_path
                ));
            }
        }

        let run_start = std::time::Instant::now();
        let run_target = executable_invocation_path(&bin_path);
        let run_status = ProcessCommand::new(&run_target).status();
        let run_duration = run_start.elapsed();
        let exit_code = run_status
            .map_err(|e| format!("Failed to run compiled binary '{}': {}", run_target, e))?
            .code()
            .unwrap_or(255);

        println!(
            "exit with code {} in {:?}",
            exit_code,
            lex_duration + duration_parser_new + compile_duration + link_duration + run_duration
        );
        println!("lamina ir: {}", ir_path);
        println!("lamina asm: {}", asm_path);
        println!("lamina bin: {}", bin_path);

        drop(contents);
        return Ok(());
    }

    #[cfg(feature = "jit")]
    {
        let start_time_jit = std::time::Instant::now();
        let mut jit = JIT::default();
        let raw_code_ptr = jit
            .compile(ast_new.clone(), input, false)
            .map_err(|e| e.to_diagnostic().to_string())?;
        let duration_jit = start_time_jit.elapsed();
        let pre_run_time = std::time::Instant::now();
        let result = run_jit(raw_code_ptr).map_err(|e| format!("Error during execution: {}", e))?;
        let duration = pre_run_time.elapsed();
        println!(
            "exit with code {} in {:?}",
            result,
            duration + duration_jit + lex_duration + duration_parser_new
        );
        if let Err(e) = save_executable(raw_code_ptr, "a") {
            eprintln!("Failed to save executable: {}", e);
        }
        drop(jit);
        drop(contents);
        Ok(())
    }

    #[cfg(not(feature = "jit"))]
    {
        Err("JIT backend not available. Build with --features jit or use --backend lamina (default).".to_string())
    }
}

fn output_asm_path(input: &str, custom_path: Option<&str>) -> String {
    if let Some(path) = custom_path {
        return path.to_string();
    }

    let stem = input_stem(input, "out");
    format!("target/lamina/{}.s", stem)
}

fn output_bin_path(input: &str) -> String {
    let stem = input_stem(input, "a");

    if cfg!(target_os = "windows") {
        format!("target/lamina/{}.exe", stem)
    } else {
        format!("target/lamina/{}", stem)
    }
}

fn output_ir_path(input: &str, custom_path: Option<&str>) -> String {
    if let Some(path) = custom_path {
        return path.to_string();
    }

    let stem = input_stem(input, "out");
    format!("target/lamina/{}.lamina", stem)
}

fn input_stem(input: &str, fallback: &str) -> String {
    let input_path = Path::new(input);
    input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(fallback)
        .to_string()
}

fn ensure_parent_dir(path: &str) -> Result<(), std::io::Error> {
    let parent = Path::new(path).parent();
    if let Some(dir) = parent {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

fn executable_invocation_path(bin_path: &str) -> String {
    let path = Path::new(bin_path);
    if path.is_absolute() || bin_path.contains('/') || cfg!(target_os = "windows") {
        bin_path.to_string()
    } else {
        format!("./{}", bin_path)
    }
}

#[cfg(feature = "jit")]
fn run_jit(codeptr: *const u8) -> Result<isize, String> {
    unsafe { run_code(codeptr, ()) }
}

/// Invokes JIT-compiled code. Caller must ensure `codeptr` is a valid function
/// pointer produced by cranelift JIT for the correct signature.
///
/// # Safety
///
/// - `codeptr` must be non-null and point to executable code.
/// - The code must have been compiled for the signature `fn(I) -> O`.
/// - The memory must remain valid for the duration of the call.
#[cfg(feature = "jit")]
unsafe fn run_code<I, O>(codeptr: *const u8, input: I) -> Result<O, String> {
    if codeptr.is_null() {
        return Err("Null function pointer".to_string());
    }
    let code_fn = std::mem::transmute::<*const u8, fn(I) -> O>(codeptr);
    Ok(code_fn(input))
}
