//! SPICE syntax highlighting for the netlist editor.
//!
//! Token classes follow the design's instrument-panel reading order:
//! element names neutral, nodes dim, values bright (trace blue),
//! dot-commands accent, comments faint, model names pink, `{expressions}`
//! green. Lines carrying a parse diagnostic are underlined in the error
//! color — same source of truth as the gutter pip and the bottom strip.

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke};

use crate::ui::palette::Palette;

use super::{Diagnostic, DiagnosticSeverity};

/// Build the highlighted layout job for the whole buffer. The job's text
/// must equal `text` exactly — `TextEdit` maps cursor positions onto it.
pub fn layout_job(text: &str, font: FontId, c: &Palette, diagnostics: &[Diagnostic]) -> LayoutJob {
    let mut job = LayoutJob {
        break_on_newline: true,
        ..Default::default()
    };
    job.wrap.max_width = f32::INFINITY;

    let segments: Vec<&str> = text.split('\n').collect();
    let last = segments.len().saturating_sub(1);
    let mut line_start = 0usize;
    for (line_idx, line) in segments.iter().enumerate() {
        let line_fallback = line_only_severity(diagnostics, line_idx);
        let mut next_offset = line_start;
        let style = DiagnosticStyle {
            font: &font,
            palette: c,
            diagnostics,
            line_fallback,
        };
        let mut push = |job: &mut LayoutJob, span: &str, color: Color32| {
            if span.is_empty() {
                return;
            }
            let span_start = next_offset;
            next_offset += span.len();
            append_diagnostic_span(job, span, span_start, color, &style);
        };
        highlight_line(&mut job, line, c, &mut push);
        if line_idx != last {
            // The newline itself carries no visible style.
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: font.clone(),
                    color: c.text,
                    ..Default::default()
                },
            );
            line_start += line.len() + 1;
        }
    }
    job
}

struct DiagnosticStyle<'a> {
    font: &'a FontId,
    palette: &'a Palette,
    diagnostics: &'a [Diagnostic],
    line_fallback: Option<DiagnosticSeverity>,
}

fn append_diagnostic_span(
    job: &mut LayoutJob,
    span: &str,
    global_start: usize,
    color: Color32,
    style: &DiagnosticStyle<'_>,
) {
    let global_end = global_start + span.len();
    let mut split_points = vec![global_start, global_end];
    for diagnostic in style.diagnostics {
        let Some(range) = diagnostic.span.as_ref() else {
            continue;
        };
        for point in [range.start, range.end] {
            if point > global_start
                && point < global_end
                && span.is_char_boundary(point - global_start)
            {
                split_points.push(point);
            }
        }
    }
    split_points.sort_unstable();
    split_points.dedup();

    for window in split_points.windows(2) {
        let part_start = window[0];
        let part_end = window[1];
        if part_start == part_end {
            continue;
        }
        let local_start = part_start - global_start;
        let local_end = part_end - global_start;
        let severity =
            span_severity(style.diagnostics, part_start..part_end).or(style.line_fallback);
        job.append(
            &span[local_start..local_end],
            0.0,
            TextFormat {
                font_id: style.font.clone(),
                color,
                underline: severity
                    .map(|severity| severity_stroke(severity, style.palette))
                    .unwrap_or(Stroke::NONE),
                ..Default::default()
            },
        );
    }
}

fn line_only_severity(diagnostics: &[Diagnostic], line_idx: usize) -> Option<DiagnosticSeverity> {
    diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.span.is_none() && diagnostic.line == Some(line_idx))
        .map(|diagnostic| diagnostic.severity)
        .max()
}

fn span_severity(
    diagnostics: &[Diagnostic],
    range: std::ops::Range<usize>,
) -> Option<DiagnosticSeverity> {
    diagnostics
        .iter()
        .filter_map(|diagnostic| {
            let span = diagnostic.span.as_ref()?;
            ranges_overlap(span, &range).then_some(diagnostic.severity)
        })
        .max()
}

fn ranges_overlap(a: &std::ops::Range<usize>, b: &std::ops::Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn severity_stroke(severity: DiagnosticSeverity, c: &Palette) -> Stroke {
    let color = match severity {
        DiagnosticSeverity::Error => c.err,
        DiagnosticSeverity::Warning => c.warn,
        DiagnosticSeverity::Info => c.text_dim,
    };
    Stroke::new(1.0, color)
}

/// Append one line's tokens to the job.
fn highlight_line(
    job: &mut LayoutJob,
    line: &str,
    c: &Palette,
    push: &mut impl FnMut(&mut LayoutJob, &str, Color32),
) {
    let trimmed = line.trim_start();

    // Whole-line comment.
    if trimmed.starts_with('*') {
        push(job, line, c.text_faint);
        return;
    }

    // Inline comment split (`;` or ` $`).
    let comment_at = find_inline_comment(line);
    let (code, comment) = line.split_at(comment_at.unwrap_or(line.len()));

    let code_trimmed = code.trim_start();
    let lead_len = code.len() - code_trimmed.len();
    push(job, &code[..lead_len], c.text);

    if code_trimmed.starts_with('.') {
        highlight_dot_line(job, code_trimmed, c, push);
    } else if let Some(stripped) = code_trimmed.strip_prefix('+') {
        push(job, &code_trimmed[..1], c.text_faint);
        highlight_fields(job, stripped, c, push, false);
    } else {
        // Element line: refdes neutral-bright, then nodes/values.
        highlight_fields(job, code_trimmed, c, push, true);
    }

    push(job, comment, c.text_faint);
}

/// Dot-command line: the command accent, `.model` names pink, the rest
/// per the generic field rules.
fn highlight_dot_line(
    job: &mut LayoutJob,
    code: &str,
    c: &Palette,
    push: &mut impl FnMut(&mut LayoutJob, &str, Color32),
) {
    let cmd_end = code
        .char_indices()
        .find(|(_, ch)| ch.is_whitespace())
        .map(|(i, _)| i)
        .unwrap_or(code.len());
    let (cmd, rest) = code.split_at(cmd_end);
    push(job, cmd, c.accent);

    if cmd.eq_ignore_ascii_case(".model") || cmd.eq_ignore_ascii_case(".subckt") {
        // The defined name reads in the model hue.
        let ws_end = rest
            .char_indices()
            .find(|(_, ch)| !ch.is_whitespace())
            .map(|(i, _)| i)
            .unwrap_or(rest.len());
        let (ws, after) = rest.split_at(ws_end);
        push(job, ws, c.text);
        let name_end = after
            .char_indices()
            .find(|(_, ch)| ch.is_whitespace())
            .map(|(i, _)| i)
            .unwrap_or(after.len());
        let (name, tail) = after.split_at(name_end);
        push(job, name, c.traces[2]);
        highlight_fields(job, tail, c, push, false);
    } else {
        highlight_fields(job, rest, c, push, false);
    }
}

/// Generic SPICE fields: identifiers dim (nodes), numbers bright,
/// `name=value` pairs, `{expressions}` green. When `first_is_element`,
/// the leading identifier reads as the element name (neutral-bright).
fn highlight_fields(
    job: &mut LayoutJob,
    text: &str,
    c: &Palette,
    push: &mut impl FnMut(&mut LayoutJob, &str, Color32),
    first_is_element: bool,
) {
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut field_index = 0usize;

    while i < bytes.len() {
        let ch = bytes[i] as char;

        if ch.is_whitespace() {
            let start = i;
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            push(job, &text[start..i], c.text);
            continue;
        }

        if ch == '{' {
            // Expression: color braces and body green, to the matching brace.
            let start = i;
            let mut depth = 0i32;
            while i < bytes.len() {
                match bytes[i] as char {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            push(job, &text[start..i], c.traces[3]);
            field_index += 1;
            continue;
        }

        if ch == '=' {
            push(job, "=", c.text_faint);
            i += 1;
            continue;
        }

        // A token: read to whitespace, '=', or '{'.
        let start = i;
        while i < bytes.len() {
            let t = bytes[i] as char;
            if t.is_whitespace() || t == '=' || t == '{' {
                break;
            }
            i += 1;
        }
        let token = &text[start..i];
        let next_is_assign = bytes.get(i).copied() == Some(b'=');

        let color = if field_index == 0 && first_is_element {
            c.text
        } else if next_is_assign {
            c.text_dim
        } else if looks_numeric(token) {
            c.traces[1]
        } else if was_preceded_by_assign(text, start) {
            // Bare-word parameter values (e.g. `model=nch`) read as values.
            c.traces[1]
        } else {
            c.text_dim
        };
        push(job, token, color);
        field_index += 1;
    }
}

/// Engineering-notation literal: starts with a digit, sign, or decimal
/// point ("4.7k", "-2.5", ".5u", "1e-9").
fn looks_numeric(token: &str) -> bool {
    token
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '.')
}

/// Whether the non-space character before `start` is `=`.
fn was_preceded_by_assign(text: &str, start: usize) -> bool {
    text[..start]
        .chars()
        .rev()
        .find(|ch| !ch.is_whitespace())
        .is_some_and(|ch| ch == '=')
}

/// Position of an inline comment (`;` anywhere, or ` $` after whitespace).
fn find_inline_comment(line: &str) -> Option<usize> {
    let semicolon = line.find(';');
    let dollar = line
        .char_indices()
        .find(|(i, ch)| {
            *ch == '$'
                && *i > 0
                && line[..*i]
                    .chars()
                    .next_back()
                    .is_some_and(|prev| prev.is_whitespace())
        })
        .map(|(i, _)| i);
    match (semicolon, dollar) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use egui::FontId;

    use super::super::DiagnosticSeverity;
    use super::*;
    use crate::ui::palette::INSTRUMENT_DARK;

    fn underlined_ranges(job: &LayoutJob) -> Vec<Range<usize>> {
        job.sections
            .iter()
            .filter(|section| section.format.underline.width > 0.0)
            .map(|section| section.byte_range.clone())
            .collect()
    }

    fn error_diagnostic(span: Option<Range<usize>>, line: Option<usize>) -> Diagnostic {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            span,
            line,
            column: None,
            message: "bad token".to_owned(),
            fix: None,
        }
    }

    #[test]
    fn span_diagnostic_underlines_only_the_offending_range() {
        let text = "R1 in out bad\n";
        let diagnostic = error_diagnostic(Some(10..13), Some(0));

        let job = layout_job(
            text,
            FontId::monospace(12.0),
            &INSTRUMENT_DARK,
            &[diagnostic],
        );

        assert_eq!(underlined_ranges(&job), vec![10..13]);
    }

    #[test]
    fn line_only_diagnostic_underlines_the_reported_line() {
        let text = "R1 in out 1k\nBAD\nC1 out 0 1p";
        let diagnostic = error_diagnostic(None, Some(1));

        let job = layout_job(
            text,
            FontId::monospace(12.0),
            &INSTRUMENT_DARK,
            &[diagnostic],
        );

        assert_eq!(underlined_ranges(&job), vec![13..16]);
    }
}
