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

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId};
use rspice_core::netlist::{SourceKind, comment_ranges, comment_start, is_spectre_source};

use super::index_worker::BackgroundIndex;
use crate::state::TextSnapshot;
use crate::ui::palette::Palette;

use super::dialect::ShownSource;
use super::param_scan::{expression_end, run_end};
use crate::workbench::documents::virtual_text_editor::LayoutRow;

/// Mockup contract: 11-point editor text at a 1.55 line-height.
const EDITOR_LINE_HEIGHT: f32 = 17.05;

/// Where the comments of the shown source are. A SPICE line's are read from
/// the line alone; a source the Spectre front end reads is read whole, since
/// a block comment carries from line to line. The whole text is read away
/// from the editor's thread, once per revision; until the reading of the
/// text as it stands arrives, each line is read alone, in the reading the
/// last one found, so what is marked is always the text on screen.
#[derive(Clone, Default)]
pub(crate) struct SourceComments {
    reading: BackgroundIndex<ReadingKey, SourceReading>,
}

/// The text a reading is of: its revision, and where it is read from.
type ReadingKey = (u64, ShownSource);

/// How the engine reads one revision of a source, and for a source the
/// Spectre front end reads, each line's comments.
struct SourceReading {
    key: ReadingKey,
    kind: SourceKind,
    by_line: Option<Box<[Vec<Range<usize>>]>>,
}

/// A large Spectre source's comments are a list for every line: whoever lets
/// go of a reading last, a newer one replacing it, they are freed away from
/// that thread.
impl Drop for SourceReading {
    fn drop(&mut self) {
        rspice_design::freed_away::free_away(&mut self.by_line);
    }
}

impl SourceReading {
    fn of(text: &TextSnapshot, source: ShownSource) -> Self {
        let revision = text.revision();
        let text = text.as_cow();
        let kind = source.kind(&text);
        let by_line = is_spectre_source(&source.path, &text).then(|| {
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
        Self {
            key: (revision, source),
            kind,
            by_line,
        }
    }
}

impl SourceComments {
    /// The whole line `row` shows part of, and the comments on it as byte
    /// ranges of the line, for the document read from `source`. A reading of
    /// a newer text is started here; `ctx` is woken when it is done.
    pub(crate) fn on_line(
        &mut self,
        ctx: &egui::Context,
        row: &LayoutRow<'_>,
        source: &ShownSource,
    ) -> (String, Vec<Range<usize>>) {
        let document = row.document;
        let key = (document.revision(), source.clone());
        if self.reading.wants(|read| *read == key) {
            let text = document.snapshot();
            let read_from = source.clone();
            let ctx = ctx.clone();
            self.reading.start(
                key.clone(),
                move || SourceReading::of(&text, read_from),
                move || ctx.request_repaint(),
            );
        }
        let reading = self
            .reading
            .last_built()
            .filter(|reading| reading.key.1 == *source);
        let kind = reading
            .as_ref()
            .map_or_else(|| source.kind(""), |reading| reading.kind);
        let current = reading.filter(|reading| reading.key == key);
        let line = row.line();
        let on_line = match current
            .as_ref()
            .and_then(|reading| reading.by_line.as_ref())
        {
            Some(by_line) => by_line
                .get(row.logical_line)
                .into_iter()
                .flatten()
                .filter(|range| range.end <= line.len())
                .cloned()
                .collect(),
            None => on_line_alone(row.logical_line, &line, kind),
        };
        (line, on_line)
    }

    /// Read the text `document` holds now, here, as the frame after the
    /// reading arrives would.
    #[cfg(test)]
    pub(crate) fn read_now(
        &mut self,
        document: &crate::workbench::documents::text_document_model::TextDocumentModel,
        source: &ShownSource,
    ) {
        let reading = SourceReading::of(&document.snapshot(), source.clone());
        self.reading.store(reading.key.clone(), reading);
    }
}

/// The comments on line `index`, whose text is `line`, read from the line
/// alone in the reading `kind`.
fn on_line_alone(index: usize, line: &str, kind: SourceKind) -> Vec<Range<usize>> {
    match kind {
        SourceKind::DeckRoot if index == 0 => std::iter::once(0..line.len()).collect(),
        kind => comment_start(line, kind)
            .map(|at| std::iter::once(at..line.len()).collect())
            .unwrap_or_default(),
    }
}

/// Build the highlighted layout job for `text`, whose comments are at
/// `comments` (byte ranges of `text`, in order). The job's text must equal
/// `text` exactly — `TextEdit` maps cursor positions onto it.
pub fn layout_job(text: &str, comments: &[Range<usize>], font: FontId, c: &Palette) -> LayoutJob {
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
        let mut push = |job: &mut LayoutJob, span: &str, color: Color32| {
            if span.is_empty() {
                return;
            }
            job.append(
                span,
                0.0,
                TextFormat {
                    font_id: font.clone(),
                    color,
                    line_height: Some(EDITOR_LINE_HEIGHT),
                    ..Default::default()
                },
            );
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
            // Expression: braces and body in the number hue, to the matching brace.
            let start = i;
            i = expression_end(text, i);
            push(job, &text[start..i], c.traces[2]);
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
            c.traces[2]
        } else if was_preceded_by_assign(text, start) {
            // Bare-word parameter values remain distinct from circuit nodes.
            c.net_label
        } else {
            c.syntax.port_name
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
    use egui::FontId;

    use super::*;
    use crate::ui::palette::INSTRUMENT_DARK;
    use crate::workbench::documents::text_document_model::TextDocumentModel;

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
                ("in", c.syntax.port_name),
                ("\u{a0}", c.text),
                ("out", c.syntax.port_name),
                ("\u{85}", c.text),
                ("Å", c.syntax.port_name),
                (" ", c.text),
                ("1k", c.traces[2]),
            ]
        );

        let job = highlight("Gain\u{a0}stage 入😀");
        assert_eq!(
            tokens(&job),
            vec![
                ("Gain", c.traces[0]),
                ("\u{a0}", c.text),
                ("stage", c.syntax.port_name),
                (" ", c.text),
                ("入😀", c.syntax.port_name),
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
                ("2", c.traces[2]),
                ("\u{a0}", c.text),
                ("vbias", c.net_label),
                ("=", c.text_faint),
                ("{gain*à}", c.traces[2]),
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
        let mut comments = SourceComments::default();
        comments.read_now(&document, &source);
        let (whole, comments) = comments.on_line(&egui::Context::default(), &row, &source);
        let job = row.shown_part(
            &whole,
            layout_job(&whole, &comments, FontId::monospace(12.0), &INSTRUMENT_DARK),
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
            card.contains(&("$N_0001".to_owned(), c.syntax.port_name)),
            "{card:?}"
        );
        assert!(
            card.contains(&("$G_DPWR".to_owned(), c.syntax.port_name)),
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

    /// Scrolled sideways, a row keeps the colours its whole line gives it: a
    /// model name stays a model name and a node stays a node, rather than
    /// being read as the first word of a card.
    #[test]
    fn a_row_scrolled_right_keeps_the_colours_its_line_gives_it() {
        let deck = "title\n.model nch nmos level=1\nR1 a b 1k\n+ tc1=0.1\n";
        for (line, from) in [(1, 7), (2, 3), (3, 2)] {
            let whole = row_tokens(deck, "amp.cir", true, line, 0);
            let mut at = 0;
            let tail = whole
                .into_iter()
                .filter(|(token, _)| {
                    let shown = at >= from;
                    at += token.chars().count();
                    shown
                })
                .collect::<Vec<_>>();
            assert_eq!(
                row_tokens(deck, "amp.cir", true, line, from),
                tail,
                "line {line} from column {from}"
            );
        }
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
            closing.contains(&("c1".to_owned(), c.syntax.port_name)),
            "{closing:?}"
        );
        assert_eq!(faint(&row_tokens(deck, "amp.scs", true, 4, 0)), ["* star"]);
    }

    /// While the reading of the text as it stands is still being made, a
    /// row is read alone: the reading of an earlier text, whose block
    /// comment the edit took out, never marks the text on screen.
    #[test]
    fn a_reading_of_an_earlier_text_never_marks_the_text_on_screen() {
        let source = ShownSource {
            path: "amp.scs".into(),
            root: true,
        };
        let mut document =
            TextDocumentModel::from_source("// tb\n/* bias\nr1 (a b) resistor r=1k\n*/\n");
        let mut comments = SourceComments::default();
        comments.read_now(&document, &source);
        let row = |document: &TextDocumentModel| {
            document
                .line_content_char_range(2)
                .map(|range| document.text_range(range))
                .expect("the line exists")
        };
        let inside = row(&document);
        let (_, marked) = comments.on_line(
            &egui::Context::default(),
            &LayoutRow {
                text: &inside,
                logical_line: 2,
                first_column: 0,
                document: &document,
            },
            &source,
        );
        assert_eq!(
            marked,
            std::iter::once(0..inside.len()).collect::<Vec<_>>(),
            "inside the block comment"
        );

        // The edit takes out the line that opened the block comment.
        assert!(document.synchronize_source("// tb\nbias\nr1 (a b) resistor r=1k\n*/\n"));
        let card = row(&document);
        let (_, marked) = comments.on_line(
            &egui::Context::default(),
            &LayoutRow {
                text: &card,
                logical_line: 2,
                first_column: 0,
                document: &document,
            },
            &source,
        );
        assert!(marked.is_empty(), "{marked:?}");
    }
}
