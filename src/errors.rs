use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum RuntimeError {
    #[snafu(display("Unknown file extention `{format}`"))]
    Unknownformat { format: String },
}
