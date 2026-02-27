use log::{LevelFilter, Log, Metadata, Record};
use std::io::{self, Write};

struct SimpleLogger {
    max_level: LevelFilter,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let _ = writeln!(io::stderr(), "[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {
        let _ = io::stderr().flush();
    }
}

pub fn init() {
    let max_level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| match s.to_lowercase().as_str() {
            "error" => Some(LevelFilter::Error),
            "warn" => Some(LevelFilter::Warn),
            "info" => Some(LevelFilter::Info),
            "debug" => Some(LevelFilter::Debug),
            "trace" => Some(LevelFilter::Trace),
            _ => None,
        })
        .unwrap_or(LevelFilter::Info);

    let logger = SimpleLogger { max_level };
    log::set_max_level(max_level);
    if log::set_logger(Box::leak(Box::new(logger))).is_err() {
        // Logger already set (e.g. in tests)
    }
}
