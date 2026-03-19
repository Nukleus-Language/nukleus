use std::fs;
use std::io::Read;
use std::path::Path;

use astgen::ast::AST;

use super::compile;
use codegen::lamina::LaminaBackend;

use std::process::{Command as ProcessCommand, Stdio};

use crate::aot;
use crate::cli::Command;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let file_path = Path::new(filename);

    if let Some(ext) = file_path.extension() {
        if ext != "nk" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Provided file is not a nukleus file",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File has no extension",
        ));
    }

    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(clippy::needless_return)]
pub fn run(command: &Command) -> Result<(), String> {
    if let Command::Lsp = command {
        let status = ProcessCommand::new("nk-lsp")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| format!("Failed to run nk-lsp: {}. Ensure nk-lsp is in PATH (cargo install --path nk-lsp)", e))?;
        std::process::exit(status.code().unwrap_or(1));
    }

    let args = match command {
        Command::Lsp => unreachable!("Lsp handled above"),
        Command::Repl => {
            #[cfg(feature = "legacy")]
            {
                let mut interpreter = crate::legacy::interpreter::Interpreter::new();
                interpreter.run_repl();
                return Ok(());
            }
            #[cfg(not(feature = "legacy"))]
            {
                return Err(
                    "REPL requires legacy feature. Build with: cargo build --features legacy"
                        .to_string(),
                );
            }
        }
        Command::Run(a) => a,
    };

    let input = &args.input;
    let mut backend = args.backend.clone();
    let emit_asm_path = args.emit_asm.as_deref();
    let emit_ir_path = args.emit_ir.as_deref();

    if backend == "cranelift" && !cfg!(target_arch = "x86_64") {
        log::warn!(
            "cranelift backend is not supported on this architecture yet; falling back to lamina"
        );
        backend = "lamina".to_string();
    }

    let contents = read_file(input).map_err(|e| format!("Error reading file: {}", e))?;
    let input_path = Path::new(input);

    let (ast_new, lex_duration, parse_duration) = compile::compile(input_path, &contents)?;

    if backend == "lamina" {
        run_lamina(
            input,
            ast_new,
            emit_ir_path,
            emit_asm_path,
            lex_duration,
            parse_duration,
        )?;
        return Ok(());
    }

    #[cfg(feature = "jit")]
    {
        run_jit_backend(input, ast_new, lex_duration, parse_duration)?;
        return Ok(());
    }

    #[cfg(not(feature = "jit"))]
    Err(
        "JIT backend not available. Build with --features jit or use --backend lamina (default)."
            .to_string(),
    )
}

fn run_lamina(
    input: &str,
    ast_new: Vec<AST>,
    emit_ir_path: Option<&str>,
    emit_asm_path: Option<&str>,
    lex_duration: std::time::Duration,
    parse_duration: std::time::Duration,
) -> Result<(), String> {
    use std::process::Command as ProcessCommand;

    eprintln!("Compiling...");
    let compile_start = std::time::Instant::now();
    let mut lamina_backend = LaminaBackend::new();
    eprintln!("  Generating IR...");
    let ir = lamina_backend
        .compile_ast_to_ir(&ast_new)
        .map_err(|e| format!("Lamina Compile Error: {}", e))?;

    let ir_path = aot::output_ir_path(input, emit_ir_path);
    let asm_path = aot::output_asm_path(input, emit_asm_path);

    eprintln!("  Writing IR to {}...", ir_path);
    aot::ensure_parent_dir(&ir_path).map_err(|e| {
        format!(
            "Failed to prepare IR output directory for '{}': {}",
            ir_path, e
        )
    })?;
    fs::write(&ir_path, &ir)
        .map_err(|e| format!("Failed to write Lamina IR '{}': {}", ir_path, e))?;

    eprintln!("  Generating assembly...");
    let assembly = lamina_backend
        .compile_ir_to_assembly(&ir, None)
        .map_err(|e| format!("Lamina Assembly Error: {}", e))?;
    let compile_duration = compile_start.elapsed();

    eprintln!("  Writing assembly to {}...", asm_path);
    aot::ensure_parent_dir(&asm_path).map_err(|e| {
        format!(
            "Failed to prepare assembly output directory for '{}': {}",
            asm_path, e
        )
    })?;
    fs::write(&asm_path, &assembly)
        .map_err(|e| format!("Failed to write assembly '{}': {}", asm_path, e))?;

    let bin_path = aot::output_bin_path(input);
    aot::ensure_parent_dir(&bin_path).map_err(|e| {
        format!(
            "Failed to prepare binary output directory for '{}': {}",
            bin_path, e
        )
    })?;

    let link_start = std::time::Instant::now();
    eprintln!("  Linking {} -> {}...", asm_path, bin_path);
    aot::link_assembly(&asm_path, &bin_path)?;
    let link_duration = link_start.elapsed();

    let compile_total = lex_duration + parse_duration + compile_duration + link_duration;
    eprintln!(
        "Compile: {:.3}s (lex {:.0}ms, parse {:.0}ms, codegen {:.0}ms, link {:.0}ms)",
        compile_total.as_secs_f64(),
        lex_duration.as_secs_f64() * 1000.0,
        parse_duration.as_secs_f64() * 1000.0,
        compile_duration.as_secs_f64() * 1000.0,
        link_duration.as_secs_f64() * 1000.0
    );

    eprintln!("Running...");
    let run_start = std::time::Instant::now();
    let run_target = aot::executable_invocation_path(&bin_path);
    let run_status = ProcessCommand::new(&run_target).status();
    let run_duration = run_start.elapsed();

    let exit_code = run_status
        .map_err(|e| format!("Failed to run compiled binary '{}': {}", run_target, e))?
        .code()
        .unwrap_or(255);

    eprintln!(
        "Run: {:.3}s (exit code {})",
        run_duration.as_secs_f64(),
        exit_code
    );
    log::info!("lamina ir: {}", ir_path);
    log::info!("lamina asm: {}", asm_path);
    log::info!("lamina bin: {}", bin_path);

    Ok(())
}

#[cfg(feature = "jit")]
fn run_jit_backend(
    input: &str,
    ast_new: Vec<AST>,
    lex_duration: std::time::Duration,
    parse_duration: std::time::Duration,
) -> Result<(), String> {
    use codegen::cranelift_jit::{JIT, save_executable};

    let start_time_jit = std::time::Instant::now();
    let mut jit = JIT::default();
    let raw_code_ptr = jit
        .compile(ast_new, input, false)
        .map_err(|e| e.to_diagnostic().to_string())?;
    let duration_jit = start_time_jit.elapsed();

    let pre_run_time = std::time::Instant::now();
    let result = run_jit(raw_code_ptr).map_err(|e| format!("Error during execution: {}", e))?;
    let duration = pre_run_time.elapsed();

    log::info!(
        "exit with code {} in {:?}",
        result,
        duration + duration_jit + lex_duration + parse_duration
    );
    if let Err(e) = save_executable(raw_code_ptr, "a") {
        log::error!("Failed to save executable: {}", e);
    }

    Ok(())
}

#[cfg(feature = "jit")]
fn run_jit(codeptr: *const u8) -> Result<isize, String> {
    unsafe { run_code(codeptr, ()) }
}

#[cfg(feature = "jit")]
unsafe fn run_code<I, O>(codeptr: *const u8, input: I) -> Result<O, String> {
    if codeptr.is_null() {
        return Err("Null function pointer".to_string());
    }
    let code_fn = unsafe { std::mem::transmute::<*const u8, fn(I) -> O>(codeptr) };
    Ok(code_fn(input))
}
