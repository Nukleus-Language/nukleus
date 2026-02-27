use std::env;

#[derive(Debug, Default)]
pub struct Args {
    pub input: String,
    pub backend: String,
    pub emit_asm: Option<String>,
    pub emit_ir: Option<String>,
    pub lamina: bool,
}

const HELP: &str = r#"Nukleus Language

USAGE:
    nukleus [OPTIONS] [input]

OPTIONS:
    -b, --backend <BACKEND>    Backend: lamina (default) or cranelift
    --emit-asm <PATH>         When using lamina, write assembly to PATH
    --emit-ir <PATH>          When using lamina, write IR to PATH
    --lamina                  Use lamina backend (shorthand)
    -h, --help                Print this help
    -V, --version             Print version

ARGS:
    <input>                   Input file (.nk) or "repl" for interactive [default: repl]
"#;

pub fn parse_args() -> Result<Args, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut result = Args {
        input: "repl".to_string(),
        backend: "lamina".to_string(),
        emit_asm: None,
        emit_ir: None,
        lamina: false,
    };

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            print!("{}", HELP);
            std::process::exit(0);
        }
        if arg == "-V" || arg == "--version" {
            println!("nukleus {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        if arg == "--lamina" {
            result.lamina = true;
            i += 1;
            continue;
        }
        if arg == "-b" || arg == "--backend" {
            i += 1;
            let val = args.get(i).ok_or("--backend requires a value")?;
            if val != "lamina" && val != "cranelift" {
                return Err(format!("--backend must be lamina or cranelift, got: {}", val));
            }
            result.backend = val.clone();
            i += 1;
            continue;
        }
        if arg == "--emit-asm" {
            i += 1;
            let val = args.get(i).ok_or("--emit-asm requires a value")?;
            result.emit_asm = Some(val.clone());
            i += 1;
            continue;
        }
        if arg == "--emit-ir" {
            i += 1;
            let val = args.get(i).ok_or("--emit-ir requires a value")?;
            result.emit_ir = Some(val.clone());
            i += 1;
            continue;
        }
        if arg.starts_with('-') {
            return Err(format!("Unknown option: {}", arg));
        }
        result.input = arg.clone();
        i += 1;
    }

    if result.lamina {
        result.backend = "lamina".to_string();
    }

    Ok(result)
}
