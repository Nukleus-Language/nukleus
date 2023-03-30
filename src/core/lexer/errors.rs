use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum LexerError {
    #[snafu(display("a variable name only can contain letters, numbers and underscores"))]
    InvalidIdentifierChar,
    #[snafu(display("a variable name cannot start with a number"))]
    InvalidIdentifierNum,
    #[snafu(display("Unknown Character `{character}`"))]
    UnknownCharacter { character: String },
}
