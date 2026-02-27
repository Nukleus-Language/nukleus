use super::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub primary_span: Option<Span>,
    pub suggestion: Option<String>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn error(
        code: impl std::fmt::Display,
        message: impl Into<String>,
        primary_span: Option<Span>,
    ) -> Self {
        Self {
            code: code.to_string(),
            severity: Severity::Error,
            message: message.into(),
            primary_span,
            suggestion: None,
            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    pub fn format_with_source(&self, source: &str) -> String {
        crate::render::format_diagnostic_with_source(self, source, None)
    }

    pub fn format_with_source_and_path(&self, source: &str, path: impl AsRef<str>) -> String {
        crate::render::format_diagnostic_with_source(self, source, Some(path.as_ref()))
    }
}

#[cfg(test)]
mod test {
    use super::super::Span;
    use super::*;

    #[test]
    fn format_with_source_shows_context_and_caret() {
        let diag = Diagnostic::error("TEST-001", "test error", Some(Span::point(2, 5)))
            .with_suggestion("Fix it");

        let source = "line one\nline two\nline three";
        let out = diag.format_with_source(source);

        assert!(out.contains("test error"));
        assert!(out.contains("TEST-001"));
        assert!(out.contains("line two"));
        assert!(out.contains("^"));
        assert!(out.contains("Suggestion: Fix it"));
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loc = self
            .primary_span
            .map(|s| format!(" at {}", s))
            .unwrap_or_default();
        writeln!(
            f,
            "{}: {} ({}){}",
            self.severity.label(),
            self.message,
            self.code,
            loc
        )?;
        for note in &self.notes {
            writeln!(f, "  note: {}", note)?;
        }
        if let Some(s) = &self.suggestion {
            writeln!(f, "  suggestion: {}", s)?;
        }
        Ok(())
    }
}
