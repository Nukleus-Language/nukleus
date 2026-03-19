use std::env;

use super::error::CliError;
use super::Command;
use super::RunOpts;

const HELP: &str = r#"Nukleus Language

USAGE:
    nukleus [OPTIONS] [COMMAND] [INPUT]

COMMANDS:
    run <file>     Compile and execute a .nk file (default when file given)
    repl           Start interactive REPL
    lsp            Run language server
    help           Print this help

OPTIONS (for run):
    -b, --backend <NAME>   Backend: lamina (default) or cranelift
    --emit-asm <PATH>      Write assembly to PATH
    --emit-ir <PATH>       Write IR to PATH

EXAMPLES:
    nukleus run main.nk
    nukleus main.nk
    nukleus repl
    nukleus lsp
    nukleus --emit-ir out.lamina main.nk
"#;

pub fn parse() -> Result<Command, CliError> {
    let args: Vec<String> = env::args().skip(1).collect();
    parse_from(&args)
}

pub fn parse_from(args: &[String]) -> Result<Command, CliError> {
    let mut input = None;
    let mut backend = "lamina".to_string();
    let mut emit_asm = None;
    let mut emit_ir = None;
    let mut explicit_command = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" || arg == "help" {
            print!("{HELP}");
            std::process::exit(0);
        }
        if arg == "-V" || arg == "--version" {
            println!("nukleus {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        if arg == "-b" || arg == "--backend" {
            i += 1;
            let val = args
                .get(i)
                .ok_or(CliError::MissingValue("--backend"))?;
            if val != "lamina" && val != "cranelift" {
                return Err(CliError::InvalidBackend(val.clone()));
            }
            backend = val.clone();
            i += 1;
            continue;
        }
        if arg == "--emit-asm" {
            i += 1;
            let val = args
                .get(i)
                .ok_or(CliError::MissingValue("--emit-asm"))?;
            emit_asm = Some(val.clone());
            i += 1;
            continue;
        }
        if arg == "--emit-ir" {
            i += 1;
            let val = args
                .get(i)
                .ok_or(CliError::MissingValue("--emit-ir"))?;
            emit_ir = Some(val.clone());
            i += 1;
            continue;
        }
        if arg.starts_with('-') {
            return Err(CliError::UnknownOption(arg.clone()));
        }
        match arg.as_str() {
            "run" | "repl" | "lsp" if explicit_command.is_none() => {
                explicit_command = Some(arg.clone());
            }
            _ => {
                if input.is_none() {
                    input = Some(arg.clone());
                } else {
                    return Err(CliError::UnknownOption(arg.clone()));
                }
            }
        }
        i += 1;
    }

    match (explicit_command.as_deref(), input.as_deref()) {
        (Some("repl"), _) | (None, None) => Ok(Command::Repl),
        (Some("lsp"), _) | (Some("run"), Some("lsp")) => Ok(Command::Lsp),
        (Some("run"), Some("repl")) => Ok(Command::Repl),
        (Some("run"), Some(path)) | (None, Some(path)) => {
            if path == "repl" {
                Ok(Command::Repl)
            } else if path == "lsp" {
                Ok(Command::Lsp)
            } else {
                Ok(Command::Run(RunOpts {
                    input: path.to_string(),
                    backend,
                    emit_asm,
                    emit_ir,
                }))
            }
        }
        (Some("run"), None) => Err(CliError::MissingInput),
        _ => Ok(Command::Repl),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_args(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn parse_default_is_repl() {
        let args = to_args(&[]);
        let cmd = parse_from(&args).unwrap();
        assert!(matches!(cmd, Command::Repl));
    }

    #[test]
    fn parse_repl_explicit() {
        let args = to_args(&["repl"]);
        let cmd = parse_from(&args).unwrap();
        assert!(matches!(cmd, Command::Repl));
    }

    #[test]
    fn parse_lsp() {
        let args = to_args(&["lsp"]);
        let cmd = parse_from(&args).unwrap();
        assert!(matches!(cmd, Command::Lsp));
    }

    #[test]
    fn parse_run_file_shorthand() {
        let args = to_args(&["main.nk"]);
        let cmd = parse_from(&args).unwrap();
        match &cmd {
            Command::Run(opts) => {
                assert_eq!(opts.input, "main.nk");
                assert_eq!(opts.backend, "lamina");
            }
            _ => panic!("expected Run"),
        }
    }

    #[test]
    fn parse_run_file_explicit() {
        let args = to_args(&["run", "main.nk"]);
        let cmd = parse_from(&args).unwrap();
        match &cmd {
            Command::Run(opts) => {
                assert_eq!(opts.input, "main.nk");
            }
            _ => panic!("expected Run"),
        }
    }

    #[test]
    fn parse_run_with_backend() {
        let args = to_args(&["-b", "cranelift", "main.nk"]);
        let cmd = parse_from(&args).unwrap();
        match &cmd {
            Command::Run(opts) => {
                assert_eq!(opts.backend, "cranelift");
            }
            _ => panic!("expected Run"),
        }
    }

    #[test]
    fn parse_run_with_emit_opts() {
        let args = to_args(&["--emit-ir", "out.lamina", "--emit-asm", "out.s", "x.nk"]);
        let cmd = parse_from(&args).unwrap();
        match &cmd {
            Command::Run(opts) => {
                assert_eq!(opts.emit_ir.as_deref(), Some("out.lamina"));
                assert_eq!(opts.emit_asm.as_deref(), Some("out.s"));
            }
            _ => panic!("expected Run"),
        }
    }

    #[test]
    fn parse_invalid_backend() {
        let args = to_args(&["-b", "foo", "x.nk"]);
        let err = parse_from(&args).unwrap_err();
        assert!(matches!(err, CliError::InvalidBackend(_)));
    }

    #[test]
    fn parse_run_missing_input() {
        let args = to_args(&["run"]);
        let err = parse_from(&args).unwrap_err();
        assert!(matches!(err, CliError::MissingInput));
    }

    #[test]
    fn parse_legacy_repl_as_input() {
        let args = to_args(&["repl"]);
        let cmd = parse_from(&args).unwrap();
        assert!(matches!(cmd, Command::Repl));
    }

    #[test]
    fn parse_legacy_lsp_as_input() {
        let args = to_args(&["lsp"]);
        let cmd = parse_from(&args).unwrap();
        assert!(matches!(cmd, Command::Lsp));
    }

    #[test]
    fn parse_unknown_option() {
        let args = to_args(&["--unknown", "x.nk"]);
        let err = parse_from(&args).unwrap_err();
        assert!(matches!(err, CliError::UnknownOption(_)));
    }
}
