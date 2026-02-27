#![allow(clippy::cognitive_complexity, clippy::needless_borrow)]

mod aot;
mod cli;
mod diagnostics;
mod driver;
mod errors;
#[cfg(feature = "legacy")]
mod legacy;
mod logger;

fn main() {
    logger::init();
    if let Err(e) = run() {
        log::error!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = cli::parse_args()?;
    let command = cli::Command::from_args(args);
    driver::run(&command)
}
