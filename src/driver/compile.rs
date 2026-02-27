use std::collections::HashSet;
use std::path::{Path, PathBuf};

use astgen::ast::{AST, ASTstatement};

fn resolve_import_path(name: &str, from_path: &Path) -> PathBuf {
    let file_name = format!("{}.nk", name);
    let from_dir = from_path.parent().unwrap_or(Path::new("."));
    from_dir.join(&file_name)
}

fn expand_imports(
    ast: &[AST],
    input_path: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Vec<AST>, String> {
    let mut merged: Vec<AST> = Vec::new();

    for node in ast {
        if let AST::Statement(ASTstatement::Import { name }) = &node {
            let resolved = resolve_import_path(name, input_path);
            let canonical = resolved.canonicalize().map_err(|e| {
                format!(
                    "Cannot resolve import '{}': {} (tried {})",
                    name,
                    e,
                    resolved.display()
                )
            })?;

            if visited.contains(&canonical) {
                continue;
            }
            visited.insert(canonical.clone());

            eprintln!("  Importing {}...", resolved.display());
            let contents = std::fs::read_to_string(&canonical)
                .map_err(|e| format!("Cannot read import '{}': {}", name, e))?;

            eprintln!("    Lexing {}...", resolved.display());
            let mut lexer = lexer::frontend::Lexer::from_path(resolved.as_path(), &contents);
            lexer.run().map_err(|e| {
                e.format_with_source_and_path(&contents, canonical.display().to_string())
            })?;

            let tokens = lexer.tokens().to_vec();
            eprintln!("    Parsing {}...", resolved.display());
            let mut parser = astgen::parser::Parser::new(&tokens, canonical.clone(), &contents);
            parser.run().map_err(|e| {
                let diag = e.to_diagnostic();
                diag.format_with_source_and_path(&contents, canonical.display().to_string())
            })?;

            let imported_ast = expand_imports(parser.get_asts(), canonical.as_path(), visited)?;
            merged.extend(imported_ast);
        } else if !matches!(node, AST::Statement(ASTstatement::Import { .. })) {
            merged.push(node.clone());
        }
    }

    Ok(merged)
}

pub fn compile(
    input_path: &Path,
    contents: &str,
) -> Result<(Vec<AST>, std::time::Duration, std::time::Duration), String> {
    eprintln!("Lexing {}...", input_path.display());
    let lex_start = std::time::Instant::now();
    let mut lexer = lexer::frontend::Lexer::from_path(input_path, contents);
    lexer
        .run()
        .map_err(|e| e.format_with_source_and_path(contents, input_path.display().to_string()))?;
    let lex_duration = lex_start.elapsed();

    let tokens = lexer.tokens().to_vec();

    #[cfg(debug_assertions)]
    {
        let chars_per_second = contents.len() as f64 / lex_duration.as_secs_f64();
        let chars_mb_per_second = chars_per_second * 4.0 / 1024.0 / 1024.0;
        log::debug!(
            "Lexer chars/s: {} MB/s: {}",
            chars_per_second,
            chars_mb_per_second
        );
    }

    eprintln!("Parsing {}...", input_path.display());
    let mut parser = astgen::parser::Parser::new(&tokens, input_path.to_path_buf(), contents);

    let start_time = std::time::Instant::now();
    parser.run().map_err(|e| {
        let diag = e.to_diagnostic();
        diag.format_with_source_and_path(contents, input_path.display().to_string())
    })?;
    let ast_new = parser.get_asts();
    let parse_duration = start_time.elapsed();

    #[cfg(debug_assertions)]
    {
        log::debug!("Parser time: {:?}", parse_duration);
        let tokens_per_second = tokens.len() as f64 / parse_duration.as_secs_f64();
        log::debug!("Parser tokens/s: {}", tokens_per_second);
    }

    let mut visited = HashSet::new();
    let input_canonical = input_path
        .canonicalize()
        .map_err(|e| format!("Cannot canonicalize input path: {}", e))?;
    visited.insert(input_canonical);

    let ast_expanded = expand_imports(&ast_new, input_path, &mut visited)?;

    Ok((ast_expanded, lex_duration, parse_duration))
}
