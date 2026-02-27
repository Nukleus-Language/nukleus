//! Nukleus lexer. Primary API: `frontend::Lexer` with tokens from `neo_tokens`.
//!
//! Canonical token types: `neo_tokens::{Token, TokenMetadata, TokenType, ...}`.
//! The `tokens` module is legacy; use `neo_tokens` for new code.

pub mod diagnostics;
mod error;
pub mod frontend;
mod lex;
/// Canonical token types. Use this module for all new code.
pub mod neo_tokens;
/// Legacy token model. Prefer `neo_tokens` for new code.
pub mod tokens;

#[deprecated(
    since = "0.1.0",
    note = "Use neo_tokens instead. This alias will be removed."
)]
pub use neo_tokens as tokens_new;

pub use error::LexerError;
pub use frontend::Lexer;
pub use lex::lexer;
pub use tokens::*;
