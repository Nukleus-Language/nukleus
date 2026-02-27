mod error;
mod render;
mod span;

pub use error::{Diagnostic, Severity};
pub use render::format_diagnostic_with_source;
pub use span::{Position, Span};
