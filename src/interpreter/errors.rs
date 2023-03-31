use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum InterpretingError {
    #[snafu(display("Syntax error: {}", source))]
    SyntaxError { source: String },
    #[snafu(display("Runtime error: {}", source))]
    RuntimeError { source: String },
}       

