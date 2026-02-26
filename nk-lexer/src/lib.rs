pub mod diagnostics;
mod error;
pub mod frontend;
mod lex;
pub mod neo_tokens;
mod tokens;
pub use error::LexerError;
pub use frontend::Lexer;
pub use lex::lexer;
pub use tokens::*;

// benchmark between the two lexers
