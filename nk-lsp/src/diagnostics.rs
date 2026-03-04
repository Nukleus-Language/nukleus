//! Maps nk_diagnostics::Diagnostic to lsp_types::Diagnostic.
//! LSP uses 0-based line/character; nk-diagnostics uses 1-based line/column.

use nk_diagnostics::{Diagnostic as NkDiagnostic, Severity, Span};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn to_lsp_diagnostic(nk: &NkDiagnostic) -> Diagnostic {
    let range = nk
        .primary_span
        .map(span_to_range)
        .unwrap_or_else(|| Range::new(Position::new(0, 0), Position::new(0, 0)));

    let severity = match nk.severity {
        Severity::Error => Some(DiagnosticSeverity::ERROR),
        Severity::Warning => Some(DiagnosticSeverity::WARNING),
        Severity::Info => Some(DiagnosticSeverity::INFORMATION),
    };

    let mut message = nk.message.clone();
    if let Some(s) = &nk.suggestion {
        message.push_str("\nSuggestion: ");
        message.push_str(s);
    }
    for note in &nk.notes {
        message.push_str("\nNote: ");
        message.push_str(note);
    }

    Diagnostic {
        range,
        severity,
        code: Some(tower_lsp::lsp_types::NumberOrString::String(
            nk.code.clone(),
        )),
        code_description: None,
        source: Some("nk".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn span_to_range(span: Span) -> Range {
    let start = position_to_lsp(span.start.line, span.start.column);
    let end = position_to_lsp(span.end.line, span.end.column);
    if start == end {
        Range::new(start, Position::new(end.line, end.character + 1))
    } else {
        Range::new(start, end)
    }
}

fn position_to_lsp(line: usize, column: usize) -> Position {
    Position::new(
        (line.saturating_sub(1)) as u32,
        (column.saturating_sub(1)) as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_nk_diagnostic_to_lsp_with_span() {
        let nk = NkDiagnostic::error("NK-LEX-001", "unclosed string", Some(Span::point(2, 5)));
        let lsp = to_lsp_diagnostic(&nk);
        assert_eq!(lsp.range.start.line, 1);
        assert_eq!(lsp.range.start.character, 4);
        assert_eq!(lsp.message, "unclosed string");
        assert_eq!(
            lsp.code,
            Some(tower_lsp::lsp_types::NumberOrString::String(
                "NK-LEX-001".into()
            ))
        );
    }
}
