use std::path::Path;

use astgen::ast::AST;

pub fn compile(
    input_path: &Path,
    contents: &str,
) -> Result<(Vec<AST>, std::time::Duration, std::time::Duration), String> {
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

    let mut parser = astgen::parser_new::Parser::new(&tokens, input_path.to_path_buf(), contents);

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

    Ok((ast_new.clone(), lex_duration, parse_duration))
}
