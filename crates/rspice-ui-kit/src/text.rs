//! Text a person typed, laid out in the order it is read.
//!
//! egui shapes each run of a [`LayoutJob`] in the direction its own script
//! implies and places the runs in logical order. That suffices for Latin,
//! Greek, CJK and Thai, but not for a line holding Arabic or Hebrew: a
//! multi-word phrase shows its words left to right, and a number inside it
//! lands on the wrong side. Here a line is cut from the logical text first,
//! then handed to egui as one section per piece in display order
//! ([`rspice_fonts::bidi`]), so each piece is shaped alone and in the right
//! direction, and Arabic still joins inside it.
//!
//! Text with no right-to-left character takes egui's own path untouched,
//! after one scan of its characters (of its bytes, for ASCII).

use std::ops::Range;
use std::sync::Arc;

use egui::epaint::text::FontsView;
use egui::text::{LayoutJob, TextFormat, TextWrapping};
use egui::{Align2, Color32, FontId, Galley, Painter, Pos2, Rect, Response, Ui};
use rspice_fonts::Role;
use rspice_fonts::bidi::{BidiText, is_logical_order};

use crate::fonts::role_of;

mod edit;

pub use edit::ReadingOrderEdit;

/// `text` laid out in `format`, wrapped as `wrap` says, in the order it is
/// read.
///
/// A wrapped paragraph is broken into rows on its logical text, and each
/// row is then reordered on its own. A row cut short by `wrap.max_rows`
/// loses its logical end, and the overflow character takes the place the
/// lost text would have been read from.
pub fn layout(
    fonts: &mut FontsView<'_>,
    text: &str,
    format: TextFormat,
    wrap: TextWrapping,
) -> Arc<Galley> {
    let Some(bidi) = BidiText::new(text) else {
        return fonts.layout_job(job(text, format, wrap));
    };
    if wrap.max_width.is_infinite() && wrap.max_rows == usize::MAX {
        return fonts.layout_job(display_job(&bidi, &paragraph_lines(text), format));
    }
    let overflow = wrap.overflow_character.unwrap_or('…');
    let logical = fonts.layout_job(job(text, format.clone(), wrap));
    let rows = row_ranges(&logical, text);
    if !logical.elided {
        return fonts.layout_job(display_job(&bidi, &rows, format));
    }
    // The last row ends in the overflow character: keep what it shows of
    // the logical text, and read the character at the logical end.
    let mut shown: String = text[..rows.last().map_or(0, |row| row.end)].to_owned();
    shown.pop();
    shown.push(overflow);
    let Some(shown_bidi) = BidiText::new(&shown) else {
        return logical;
    };
    let shown_rows = row_ranges_within(&rows, &shown);
    fonts.layout_job(display_job(&shown_bidi, &shown_rows, format))
}

/// `text` on one line per paragraph, in the order it is read.
pub fn layout_no_wrap(
    fonts: &mut FontsView<'_>,
    text: &str,
    font: FontId,
    color: Color32,
) -> Arc<Galley> {
    layout(
        fonts,
        text,
        TextFormat::simple(font, color),
        TextWrapping::no_max_width(),
    )
}

/// [`Painter::text`] for text a person typed: lay it out in the order it is
/// read and paint it anchored at `pos`. Returns where it ended up.
pub fn paint_text(
    painter: &Painter,
    pos: Pos2,
    anchor: Align2,
    text: impl AsRef<str>,
    font: FontId,
    color: Color32,
) -> Rect {
    let text = text.as_ref();
    if is_logical_order(text) {
        return painter.text(pos, anchor, text, font, color);
    }
    let galley = painter.fonts_mut(|fonts| layout_no_wrap(fonts, text, font, color));
    let rect = anchor.anchor_size(pos, galley.size());
    painter.galley(rect.min, galley, color);
    rect
}

/// A label of text a person typed, in `font` and `color`, wrapped at the
/// width left in the [`Ui`] and in the order it is read. Returns the label
/// and the galley it shows.
pub fn user_text(ui: &mut Ui, text: &str, font: FontId, color: Color32) -> (Response, Arc<Galley>) {
    let wrap = TextWrapping::wrap_at_width(ui.available_width());
    let galley = ui.fonts_mut(|fonts| layout(fonts, text, TextFormat::simple(font, color), wrap));
    (ui.label(Arc::clone(&galley)), galley)
}

fn job(text: &str, format: TextFormat, wrap: TextWrapping) -> LayoutJob {
    LayoutJob {
        wrap,
        ..LayoutJob::single_section(text.to_owned(), format)
    }
}

/// Each paragraph of `text` as one line: the byte ranges between its line
/// breaks.
fn paragraph_lines(text: &str) -> Vec<Range<usize>> {
    let mut start = 0;
    let mut lines = Vec::new();
    for (at, _) in text.match_indices('\n') {
        lines.push(start..at);
        start = at + 1;
    }
    lines.push(start..text.len());
    lines
}

/// The byte range of `text` each row of `galley` holds, without the line
/// break that ends a paragraph. `galley` lays `text` out with one glyph per
/// character.
fn row_ranges(galley: &Galley, text: &str) -> Vec<Range<usize>> {
    let mut boundaries = text.char_indices().map(|(at, _)| at).chain([text.len()]);
    let mut rows = Vec::with_capacity(galley.rows.len());
    let mut start = boundaries.next().unwrap_or(0);
    for row in &galley.rows {
        let mut end = start;
        for _ in 0..row.row.glyphs.len() {
            end = boundaries.next().unwrap_or(text.len());
        }
        rows.push(start..end);
        if row.ends_with_newline {
            start = boundaries.next().unwrap_or(text.len());
        } else {
            start = end;
        }
    }
    rows
}

/// `rows` of a text, carried over to `shown`, the same text cut after the
/// last row and ended in the overflow character.
fn row_ranges_within(rows: &[Range<usize>], shown: &str) -> Vec<Range<usize>> {
    let mut rows = rows.to_vec();
    if let Some(last) = rows.last_mut() {
        last.end = shown.len();
    }
    rows
}

/// A job holding each of `lines` of the text in display order, one section
/// per piece, with a line break between lines.
fn display_job(bidi: &BidiText<'_>, lines: &[Range<usize>], format: TextFormat) -> LayoutJob {
    let role: Role = role_of(&format.font_id.family);
    let mut job = LayoutJob::default();
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            job.append("\n", 0.0, format.clone());
        }
        if line.is_empty() {
            continue;
        }
        for piece in bidi.line(role, line.clone()) {
            job.append(&piece.shaped, 0.0, format.clone());
        }
    }
    job
}

#[cfg(test)]
mod tests;
