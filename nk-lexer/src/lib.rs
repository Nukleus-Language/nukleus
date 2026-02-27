//! Nukleus lexer. Primary API: `frontend::Lexer` with tokens from `tokens`.
//!
//! Canonical token types: `tokens::{Token, TokenMetadata, TokenType, ...}`.

pub mod diagnostics;
mod error;
pub mod frontend;
mod lex;
/// Canonical token types.
pub mod tokens;
/// Legacy token model. Use `tokens` for new code.
pub mod tokens_legacy;

pub use error::LexerError;
pub use frontend::Lexer;
pub use lex::lexer;
pub use tokens::*;
