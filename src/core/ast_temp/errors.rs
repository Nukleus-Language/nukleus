use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AstParseError{
    #[snafu(display("Unknown token `{token}`"))]
    UnknownToken { token: String },
    #[snafu(display("Expected `{token}`"))]
    ExpectedOther { token: String },
    #[snafu(display("End of file"))]
    EndofFile 
}
