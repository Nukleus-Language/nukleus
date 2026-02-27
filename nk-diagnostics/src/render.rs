use inksac::{Color, Style, Styleable};

use super::{Diagnostic, Severity, Span};

const CONTEXT_LINES: usize = 3;

fn error_style() -> Style {
    Style::builder()
        .foreground(Color::Red)
        .bold()
        .build()
}

fn warning_style() -> Style {
    Style::builder()
        .foreground(Color::Yellow)
        .bold()
        .build()
}

fn info_style() -> Style {
    Style::builder()
        .foreground(Color::Cyan)
        .bold()
        .build()
}

fn accent_style() -> Style {
    Style::builder()
        .foreground(Color::Red)
        .build()
}

fn suggestion_style() -> Style {
    Style::builder()
        .foreground(Color::Cyan)
        .build()
}

fn severity_style(severity: Severity) -> Style {
    match severity {
        Severity::Error => error_style(),
        Severity::Warning => warning_style(),
        Severity::Info => info_style(),
    }
}

pub fn format_diagnostic_with_source(
    diag: &Diagnostic,
    source: &str,
    path: Option<&str>,
) -> String {
    let mut out = String::new();

    let label = diag.severity.label();
    let style = severity_style(diag.severity);
    let header = format!(
        "{}: {} ({})",
        label.style(style),
        diag.message,
        diag.code
    );
    out.push_str(&header);
    if let Some(p) = path
        && !p.is_empty()
    {
        out.push_str(&format!(" in {}", p));
    }
    out.push('\n');

    if let Some(span) = &diag.primary_span {
        let snippet = build_snippet(source, span);
        out.push_str(&snippet);
    }

    if let Some(suggestion) = &diag.suggestion {
        out.push('\n');
        out.push_str(&"Suggestion: ".style(suggestion_style()).to_string());
        out.push_str(suggestion);
        out.push('\n');
    }

    for note in &diag.notes {
        out.push_str("\n  note: ");
        out.push_str(note);
        out.push('\n');
    }

    out
}

fn build_snippet(source: &str, span: &Span) -> String {
    let lines: Vec<&str> = source.split('\n').collect();
    if lines.is_empty() {
        return String::new();
    }

    let line_one = span.start.line.max(1);
    let col_one = span.start.column.max(1);
    let line_idx = line_one.saturating_sub(1);
    if line_idx >= lines.len() {
        return format!("\n  at line {}, column {}", line_one, col_one);
    }

    let start_line = line_idx.saturating_sub(CONTEXT_LINES);
    let end_line = (line_idx + CONTEXT_LINES + 1).min(lines.len());

    let mut out = String::new();
    out.push_str("\n  note: Context around Line ");
    out.push_str(&line_one.to_string());
    out.push_str(":\n");

    for (i, line_content) in lines[start_line..end_line].iter().enumerate() {
        let display_line = start_line + i + 1;
        let marker = if display_line == line_one {
            ">".style(accent_style()).to_string()
        } else {
            " ".to_string()
        };
        out.push_str(&format!("{} {} | {}\n", marker, display_line, line_content));

        if display_line == line_one {
            let caret = build_caret_line(line_content, col_one, span);
            out.push_str(&format!("    | {}\n", caret.style(accent_style())));
        }
    }

    let error_loc = format!("  --> Error at Line: {}, Column: {}", line_one, col_one);
    out.push_str(&error_loc.style(accent_style()).to_string());

    out
}

fn build_caret_line(line: &str, col_one: usize, span: &Span) -> String {
    let col_zero = col_one.saturating_sub(1);
    let before_len: usize = line
        .chars()
        .take(col_zero)
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum();
    let span_len = if span.start == span.end {
        1
    } else {
        let end_col = span.end.column.saturating_sub(1);
        let take_count = end_col.saturating_sub(col_zero).max(1);
        line.chars()
            .skip(col_zero)
            .take(take_count)
            .map(|c| if c == '\t' { 4 } else { 1 })
            .sum::<usize>()
    };
    let spaces = " ".repeat(before_len);
    let carets = "^".repeat(span_len.max(1));
    format!("{}{}", spaces, carets)
}
