mod error;
mod lex;
pub mod lex_new;
pub mod lex_new_new;
pub mod neo_tokens;
mod tokens;
pub mod trie_lex;
pub mod trie_tokens;

pub mod tokens_new;
pub use lex::lexer;
pub use tokens::*;
pub use error::LexerError;

// benchmark between the two lexers
