mod errors;
mod lex;
pub mod lex_new;
mod tokens;
mod tokens_new;

pub use lex::lexer;
pub use tokens::*;

// benchmark between the two lexers
