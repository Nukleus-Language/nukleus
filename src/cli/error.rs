#[derive(Debug)]
pub enum CliError {
    MissingValue(&'static str),
    InvalidBackend(String),
    UnknownOption(String),
    MissingInput,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingValue(opt) => write!(f, "{opt} requires a value"),
            CliError::InvalidBackend(val) => {
                write!(f, "backend must be 'lamina' or 'cranelift', got: {val}")
            }
            CliError::UnknownOption(opt) => write!(f, "unknown option: {opt}"),
            CliError::MissingInput => write!(f, "input file required"),
        }
    }
}

impl std::error::Error for CliError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_error_display() {
        assert!(CliError::MissingValue("--x").to_string().contains("requires a value"));
        assert!(CliError::InvalidBackend("foo".into()).to_string().contains("foo"));
        assert!(CliError::UnknownOption("--bad".into()).to_string().contains("unknown"));
    }
}
