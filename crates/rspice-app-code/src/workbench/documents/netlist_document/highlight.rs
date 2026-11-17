//! SPICE syntax highlighting for the netlist editor.
//!
//! Token classes follow the design's instrument-panel reading order:
//! element names neutral, nodes dim, values bright (trace blue),
//! dot-commands accent, comments faint, model names pink, `{expressions}`
//! green. What is a comment is the engine's to say: a stretch is faint
//! exactly when the parser reads no card from it. Lines carrying a parse
//! diagnostic are underlined in the error color — same source of truth as
//! the gutter pip and the bottom strip.

use std::ops::Range;
use std::sync::Arc;

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke};
use rspice_core::netlist::{SourceKind, comment_ranges, comment_start, is_spectre_source};

use crate::ui::palette::Palette;

use super::DiagnosticSeverity;
use super::dialect::ShownSource;
use super::param_scan::{expression_end, run_end};
use crate::state::netlist_diagnostics::Diagnostic;
use crate::workbench::documents::text_document_model::TextDocumentModel;
use crate::workbench::documents::virtual_text_editor::LayoutRow;

/// Mockup contract: 11-point editor text at a 1.55 line-height.
const EDITOR_LINE_HEIGHT: f32 = 17.05;

/// Where the comments of the shown source are. A SPICE line's are read from
/// the line alone; a source the Spectre front end reads is read whole, once
/// per revision of its text, since a block comment carries from line to line.
#[derive(Clone, Default)]
pub(crate) struct SourceComments {
    /// The document revision and length, and the source, the reading is of.
    read: Option<((u64, usize), ShownSource)>,
    kind: Option<SourceKind>,
    /// Each line's comments, for a source the Spectre front end reads.
    by_line: Option<Arc<[Vec<Range<usize>>]>>,
}

impl SourceComments {
    /// The comments on the stretch of a line `row` shows, as byte ranges of
    /// `row.text`, for the document read from `source`.
    pub(crate) fn on_row(
        &mut self,
        row: &LayoutRow<'_>,
        source: &ShownSource,
    ) -> Vec<Range<usize>> {
        let document = row.document;
        let key = (document.revision(), document.len_chars());
        if self
            .read
            .as_ref()
            .is_none_or(|(read, read_source)| *read != key || read_source != source)
        {
            self.read_whole(document, source);
            self.read = Some((key, source.clone()));
        }
        let line = document
            .line_content_char_range(row.logical_line)
            .map(|range| document.text_range(range))
            .unwrap_or_default();
        let on_line = self.on_line(row.logical_line, &line);
        let start = line
            .char_indices()
            .nth(row.first_column)
            .map_or(line.len(), |(index, _)| index);
        if !line[start..].starts_with(row.text) {
            // The row is not the line this frame read: read it alone rather
            // than mark the wrong text.
            return self
                .kind
                .and_then(|kind| comment_start(row.text, kind))
                .map(|at| std::iter::once(at..row.text.len()).collect())
                .unwrap_or_default();
        }
        let end = start + row.text.len();
        on_line
            .into_iter()
            .filter(|range| range.start < end && range.end > start)
            .map(|range| range.start.max(start) - start..range.end.min(end) - start)
            .collect()
    }

    fn read_whole(&mut self, document: &TextDocumentModel, source: &ShownSource) {
        let text = document.to_source();
        let kind = source.kind(&text);
        self.kind = Some(kind);
        self.by_line = is_spectre_source(&source.path, &text).then(|| {
            let mut by_line = vec![Vec::new(); text.split('\n').count()];
            let mut line_start = 0;
            let mut lines = by_line.iter_mut().zip(text.split('\n'));
            let mut current = lines.next();
            for range in comment_ranges(&text, kind) {
                while let Some((ranges, line)) = current.as_mut() {
                    if range.start <= line_start + line.len() {
                        ranges.push(range.start - line_start..range.end - line_start);
                        break;
                    }
                    line_start += line.len() + 1;
                    current = lines.next();
                }
            }
            by_line.into()
        });
    }

    /// The comments on line `index`, whose text is `line`, as byte ranges of
    /// the line.
    fn on_line(&self, index: usize, line: &str) -> Vec<Range<usize>> {
        if let Some(by_line) = &self.by_line {
            return by_line
                .get(index)
                .into_iter()
                .flatten()
                .filter(|range| range.end <= line.len())
                .cloned()
                .collect();
        }
        match self.kind {
            Some(SourceKind::DeckRoot) if index == 0 => std::iter::once(0..line.len()).collect(),
            Some(kind) => comment_start(line, kind)
                .map(|at| std::iter::once(at..line.len()).collect())
                .unwrap_or_default(),
            None => Vec::new(),
        }
    }
}

/// Build the highlighted layout job for `text`, whose comments are at
/// `comments` (byte ranges of `text`, in order). The job's text must equal
/// `text` exactly — `TextEdit` maps cursor positions onto it.
pub fn layout_job(
    text: &str,
    comments: &[Range<usize>],
    font: FontId,
    c: &Palette,
    diagnostics: &[Diagnostic],
) -> LayoutJob {
    let mut job = LayoutJob {
        break_on_newline: true,
        ..Default::default()
    };
    job.wrap.max_width = f32::INFINITY;

    let segments: Vec<&str> = text.split('\n').collect();
    let last = segments.len().saturating_sub(1);
    let mut line_start = 0usize;
    let mut comments = comments.iter().peekable();
    for (line_idx, line) in segments.iter().enumerate() {
        let line_end = line_start + line.len();
        let mut line_comments = Vec::new();
        while let Some(range) = comments.next_if(|range| range.start <= line_end) {
            line_comments.push(
                range.start.clamp(line_start, line_end) - line_start
                    ..range.end.clamp(line_start, line_end) - line_start,
            );
        }
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
        highlight_line(&mut job, line, &line_comments, c, &mut push);
        if line_idx != last {
            // The newline itself carries no visible style.
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: font.clone(),
                    color: c.text,
                    line_height: Some(EDITOR_LINE_HEIGHT),
                    ..Default::default()
                },
            );
            line_start += line.len() + 1;
        }
    }
    job
}

/// Unified-diff highlighting for immutable generated-revision comparisons.
/// The complete input is appended byte-for-byte so TextEdit selection and
/// copy semantics remain exact.
pub fn diff_layout_job(text: &str, font: FontId, c: &Palette) -> LayoutJob {
    let mut job = LayoutJob {
        break_on_newline: true,
        ..Default::default()
    };
    job.wrap.max_width = f32::INFINITY;
    for line in text.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        let color = if content.starts_with("@@") {
            c.traces[1]
        } else if content.starts_with("+++") || content.starts_with("---") {
            c.text_dim
        } else if content.starts_with('+') {
            c.ok
        } else if content.starts_with('-') {
            c.err
        } else {
            c.text_faint
        };
        job.append(
            line,
            0.0,
            TextFormat {
                font_id: font.clone(),
                color,
                line_height: Some(EDITOR_LINE_HEIGHT),
                ..Default::default()
            },
        );
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
        if !diagnostic.is_current() {
            continue;
        }
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
                line_height: Some(EDITOR_LINE_HEIGHT),
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
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.span.is_none()
                && diagnostic.line == Some(line_idx)
        })
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
            if !diagnostic.is_current() {
                return None;
            }
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
        DiagnosticSeverity::Hint => c.text_dim,
        DiagnosticSeverity::Error => c.err,
        DiagnosticSeverity::Warning => c.warn,
        DiagnosticSeverity::Info => c.text_dim,
    };
    Stroke::new(1.0, color)
}

/// Append one line's tokens to the job, its `comments` (byte ranges of the
/// line, in order) faint.
fn highlight_line(
    job: &mut LayoutJob,
    line: &str,
    comments: &[Range<usize>],
    c: &Palette,
    push: &mut impl FnMut(&mut LayoutJob, &str, Color32),
) {
    let mut at = 0;
    for comment in comments {
        highlight_code(job, &line[at..comment.start], at == 0, c, push);
        push(job, &line[comment.clone()], c.text_faint);
        at = comment.end;
    }
    highlight_code(job, &line[at..], at == 0, c, push);
}

/// Append the tokens of code on a line, which opens the card when
/// `opens_line`.
fn highlight_code(
    job: &mut LayoutJob,
    code: &str,
    opens_line: bool,
    c: &Palette,
    push: &mut impl FnMut(&mut LayoutJob, &str, Color32),
) {
    if !opens_line {
        highlight_fields(job, code, c, push, false);
        return;
    }
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
    push(job, cmd, c.traces[1]);

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
        push(job, name, c.traces[0]);
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
    // `i` only ever lands on a character boundary: every step is a whole
    // character, so a slice never cuts a multi-byte character in two.
    let mut i = 0;
    let mut field_index = 0usize;

    while let Some(ch) = text[i..].chars().next() {
        if ch.is_whitespace() {
            let start = i;
            i = run_end(text, i, char::is_whitespace);
            push(job, &text[start..i], c.text);
            continue;
        }

        if ch == '{' {
            // Expression: color braces and body green, to the matching brace.
            let start = i;
            i = expression_end(text, i);
            push(job, &text[start..i], c.traces[3]);
            field_index += 1;
            continue;
        }

        if ch == '"' || ch == '\'' {
            let start = i;
            i = quoted_end(text, i, ch);
            push(job, &text[start..i], c.warn);
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
        i = run_end(text, i, |t| !(t.is_whitespace() || t == '=' || t == '{'));
        let token = &text[start..i];
        let next_is_assign = text[i..].starts_with('=');

        let color = if field_index == 0 && first_is_element {
            c.traces[0]
        } else if next_is_assign {
            c.net_label
        } else if looks_numeric(token) {
            c.traces[3]
        } else if was_preceded_by_assign(text, start) {
            // Bare-word parameter values remain distinct from circuit nodes.
            c.net_label
        } else {
            c.traces[2]
        };
        push(job, token, color);
        field_index += 1;
    }
}

/// End of the quoted string that opens with `quote` at byte `start`: just
/// past its closing quote, or the end of the text when it never closes. A
/// backslash escapes the character after it.
fn quoted_end(text: &str, start: usize, quote: char) -> usize {
    let mut escaped = false;
    for (offset, ch) in text[start..].char_indices().skip(1) {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            return start + offset + ch.len_utf8();
        }
    }
    text.len()
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
            .map(|section| section.byte_range.start.0..section.byte_range.end.0)
            .collect()
    }

    fn error_diagnostic(span: Option<Range<usize>>, line: Option<usize>) -> Diagnostic {
        let mut diagnostic = Diagnostic::current(
            "rspice.test",
            "TEST-BAD-TOKEN",
            DiagnosticSeverity::Error,
            "bad token",
        );
        diagnostic.source_line = line;
        diagnostic.span = span;
        diagnostic.line = line;
        diagnostic.refresh_canonical_location();
        diagnostic
    }

    #[test]
    fn span_diagnostic_underlines_only_the_offending_range() {
        let text = "R1 in out bad\n";
        let diagnostic = error_diagnostic(Some(10..13), Some(0));

        let job = layout_job(
            text,
            &[],
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
            &[],
            FontId::monospace(12.0),
            &INSTRUMENT_DARK,
            &[diagnostic],
        );

        assert_eq!(underlined_ranges(&job), vec![13..16]);
    }

    /// Characters whose UTF-8 bytes include 0x85 or 0xA0 (both whitespace
    /// when a byte is read as a Latin-1 character), a CJK character and an
    /// emoji.
    const AWKWARD: [&str; 8] = ["\u{a0}", "\u{85}", "à", "Å", "Р", "入", "😀", "\u{2003}"];

    fn tokens(job: &LayoutJob) -> Vec<(&str, Color32)> {
        job.sections
            .iter()
            .map(|section| {
                (
                    &job.text[section.byte_range.start.0..section.byte_range.end.0],
                    section.format.color,
                )
            })
            .collect()
    }

    /// `text` laid out as an include's lines, with no title.
    fn highlight(text: &str) -> LayoutJob {
        layout_job(
            text,
            &comment_ranges(text, SourceKind::Included),
            FontId::monospace(12.0),
            &INSTRUMENT_DARK,
            &[],
        )
    }

    #[test]
    fn every_row_with_non_ascii_text_lays_out_whole() {
        let rows = [
            "Gain stage title line",
            "R1 in out 1k",
            "+ w=1u l={l0*2}",
            ".param gain=2 vbias={gain*1.5}",
            ".model nch nmos level=1",
            "X1 a b {w*2} cell",
            "V1 in 0 'pulse' 1",
            "C1 out 0 1p ; trailing note",
        ];
        for row in rows {
            let boundaries = row.char_indices().map(|(at, _)| at).chain([row.len()]);
            for at in boundaries {
                for character in AWKWARD {
                    let line = format!("{}{character}{}", &row[..at], &row[at..]);
                    let text = format!("{line}\r\n{line}");
                    let job = highlight(&text);
                    assert_eq!(job.text, text, "the layout must hold the text exactly");
                }
            }
        }
    }

    #[test]
    fn non_ascii_tokens_keep_their_colours_and_boundaries() {
        let c = &INSTRUMENT_DARK;
        let job = highlight("Rà in\u{a0}out\u{85}Å 1k");
        assert_eq!(
            tokens(&job),
            vec![
                ("Rà", c.traces[0]),
                (" ", c.text),
                ("in", c.traces[2]),
                ("\u{a0}", c.text),
                ("out", c.traces[2]),
                ("\u{85}", c.text),
                ("Å", c.traces[2]),
                (" ", c.text),
                ("1k", c.traces[3]),
            ]
        );

        let job = highlight("Gain\u{a0}stage 入😀");
        assert_eq!(
            tokens(&job),
            vec![
                ("Gain", c.traces[0]),
                ("\u{a0}", c.text),
                ("stage", c.traces[2]),
                (" ", c.text),
                ("入😀", c.traces[2]),
            ]
        );

        let job = highlight(".param gain=2\u{a0}vbias={gain*à} note='Å'");
        assert_eq!(
            tokens(&job),
            vec![
                (".param", c.traces[1]),
                (" ", c.text),
                ("gain", c.net_label),
                ("=", c.text_faint),
                ("2", c.traces[3]),
                ("\u{a0}", c.text),
                ("vbias", c.net_label),
                ("=", c.text_faint),
                ("{gain*à}", c.traces[3]),
                (" ", c.text),
                ("note", c.net_label),
                ("=", c.text_faint),
                ("'Å'", c.warn),
            ]
        );
    }

    /// The colour of each token of the row the editor draws for `line` of
    /// `text`, from `first_column`, read from `path` as the root deck when
    /// `root`.
    fn row_tokens(
        text: &str,
        path: &str,
        root: bool,
        line: usize,
        first_column: usize,
    ) -> Vec<(String, Color32)> {
        let document = TextDocumentModel::from_source(text);
        let whole = document
            .line_content_char_range(line)
            .map(|range| document.text_range(range))
            .expect("the line exists");
        let start = whole
            .char_indices()
            .nth(first_column)
            .map_or(whole.len(), |(index, _)| index);
        let row = LayoutRow {
            text: &whole[start..],
            logical_line: line,
            first_column,
            document: &document,
        };
        let source = ShownSource {
            path: path.into(),
            root,
        };
        let comments = SourceComments::default().on_row(&row, &source);
        let job = layout_job(
            row.text,
            &comments,
            FontId::monospace(12.0),
            &INSTRUMENT_DARK,
            &[],
        );
        tokens(&job)
            .into_iter()
            .map(|(token, colour)| (token.to_owned(), colour))
            .collect()
    }

    /// The text of the row drawn faint as a comment, or as the title.
    fn faint(tokens: &[(String, Color32)]) -> Vec<&str> {
        tokens
            .iter()
            .filter(|(token, colour)| *colour == INSTRUMENT_DARK.text_faint && token != "=")
            .map(|(token, _)| token.as_str())
            .collect()
    }

    #[test]
    fn a_spice_deck_is_faint_exactly_where_the_engine_reads_no_card() {
        let c = &INSTRUMENT_DARK;
        let deck = "PSpice export\n\
                    R1 $N_0001 $G_DPWR 1k $load\n\
                    V1 1 0 'a;b' ; note\n\
                    C1 1 0 1p // x\n\
                    # hash\n";
        assert_eq!(
            faint(&row_tokens(deck, "amp.cir", true, 0, 0)),
            ["PSpice export"]
        );
        let card = row_tokens(deck, "amp.cir", true, 1, 0);
        assert!(
            card.contains(&("$N_0001".to_owned(), c.traces[2])),
            "{card:?}"
        );
        assert!(
            card.contains(&("$G_DPWR".to_owned(), c.traces[2])),
            "{card:?}"
        );
        assert_eq!(faint(&card), ["$load"]);
        let quoted = row_tokens(deck, "amp.cir", true, 2, 0);
        assert!(quoted.contains(&("'a;b'".to_owned(), c.warn)), "{quoted:?}");
        assert_eq!(faint(&quoted), ["; note"]);
        assert_eq!(faint(&row_tokens(deck, "amp.cir", true, 3, 0)), ["// x"]);
        assert_eq!(faint(&row_tokens(deck, "amp.cir", true, 4, 0)), ["# hash"]);
        // An include has no title: its first line is a card.
        assert!(faint(&row_tokens(deck, "amp.cir", false, 0, 0)).is_empty());
    }

    #[test]
    fn a_row_scrolled_past_a_comments_start_stays_faint() {
        let deck = "title\nR1 a b 1k ; a long note\n";
        assert_eq!(
            faint(&row_tokens(deck, "amp.cir", true, 1, 14)),
            ["long note"]
        );
    }

    #[test]
    fn a_spectre_deck_colours_its_block_and_line_comments() {
        let c = &INSTRUMENT_DARK;
        let deck = "// opamp testbench\n\
                    /* bias\n   r1 (a b) resistor r=1k\n\
                    */ c1 (a 0) capacitor c=1p // load\n\
                    * star\n";
        assert_eq!(
            faint(&row_tokens(deck, "amp.scs", true, 0, 0)),
            ["// opamp testbench"]
        );
        assert_eq!(faint(&row_tokens(deck, "amp.scs", true, 1, 0)), ["/* bias"]);
        assert_eq!(
            faint(&row_tokens(deck, "amp.scs", true, 2, 0)),
            ["   r1 (a b) resistor r=1k"]
        );
        let closing = row_tokens(deck, "amp.scs", true, 3, 0);
        assert_eq!(faint(&closing), ["*/", "// load"]);
        assert!(
            closing.contains(&("c1".to_owned(), c.traces[2])),
            "{closing:?}"
        );
        assert_eq!(faint(&row_tokens(deck, "amp.scs", true, 4, 0)), ["* star"]);
    }
}
