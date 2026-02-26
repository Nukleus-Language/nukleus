use std::path::{Path, PathBuf};

use super::token_type::Token;
use nk_diagnostics::Diagnostic;

pub struct Lexer<'a> {
    inner: crate::frontend::scanner::Lexer<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_path: PathBuf, source: &'a str) -> Self {
        Self {
            inner: crate::frontend::scanner::Lexer::new(file_path, source),
        }
    }

    pub fn from_path(file_path: &Path, source: &'a str) -> Self {
        Self::new(file_path.to_path_buf(), source)
    }

    pub fn run(&mut self) -> Result<(), Diagnostic> {
        self.inner.run().map_err(|err| err.to_diagnostic())
    }

    pub fn tokens(&self) -> &[Token] {
        self.inner.get_tokens()
    }
}
