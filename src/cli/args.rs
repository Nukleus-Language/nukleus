#[derive(Debug)]
pub struct RunOpts {
    pub input: String,
    pub backend: String,
    pub emit_asm: Option<String>,
    pub emit_ir: Option<String>,
}

#[derive(Debug)]
pub enum Command {
    Repl,
    Lsp,
    Run(RunOpts),
}
