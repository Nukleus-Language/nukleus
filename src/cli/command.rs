use super::Args;

#[derive(Debug)]
pub enum Command {
    Repl,
    Compile(Args),
}

impl Command {
    pub fn from_args(args: Args) -> Self {
        if args.input == "repl" {
            Command::Repl
        } else {
            Command::Compile(args)
        }
    }
}
