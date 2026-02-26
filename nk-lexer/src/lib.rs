pub mod diagnostics;
mod error;
pub mod frontend;
mod lex;
pub mod neo_tokens;
mod tokens;

#[deprecated(since = "0.1.0", note = "Use neo_tokens instead. This alias will be removed.")]
pub use neo_tokens as tokens_new;

pub use error::LexerError;
pub use frontend::Lexer;
pub use lex::lexer;
pub use tokens::*;

// benchmark between the two lexers
