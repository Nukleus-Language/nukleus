use super::Args;

#[derive(Debug)]
pub enum Command {
    Repl,
    Lsp,
    Compile(Args),
}

impl Command {
    pub fn from_args(args: Args) -> Self {
        if args.input == "repl" {
            Command::Repl
        } else if args.input == "lsp" {
            Command::Lsp
        } else {
            Command::Compile(args)
        }
    }
}
