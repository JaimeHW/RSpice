//! Typing into text that reads right to left.
//!
//! egui's editor takes the glyphs of the galley it shows, in order, for the
//! characters of its text: its caret, its selection and a click are all
//! placed by glyph. Here a field holding Arabic or Hebrew is laid out in
//! display order instead, one row for each row of the logical text, holding
//! the same characters and line breaks. Typing, deleting and the arrow keys,
//! which move in logical order, act on indices into the text and still land
//! on the right characters. What the display order moves is where things
//! are drawn, so once egui's editor has run, [`ReadingOrderEdit::show`]
//! stands the caret at the leading edge of the character after it, paints
//! the selection over each stretch of selected glyphs, and moves a press or
//! a drag to the caret position nearest the pointer.
//!
//! Two things still follow the glyph order: a field narrower than its text
//! scrolls to keep egui's caret in view, and Up and Down keep the caret's
//! column counted in glyphs.

use std::cell::RefCell;
use std::ops::Range;
use std::sync::Arc;

use egui::epaint::text::{FontsView, Row};
use egui::layers::ShapeIdx;
use egui::text::{CCursor, CCursorRange, CharIndex, TextFormat, TextWrapping};
use egui::text_edit::TextEditOutput;
use egui::text_selection::text_cursor_state::{ccursor_next_word, ccursor_previous_word};
use egui::{
    Color32, FontSelection, Galley, Pos2, Rect, Shape, TextBuffer, Ui, Vec2, Visuals, pos2, vec2,
};
use rspice_fonts::Role;
use rspice_fonts::bidi::{BidiText, is_logical_order};
use unicode_segmentation::UnicodeSegmentation;

use super::{display_job, job, row_ranges};
use crate::fonts::role_of;

/// An editor's layout for text holding right-to-left characters, and what
/// is drawn for it once egui's editor has run.
///
/// Make one for the text before handing the text to the editor, give the
/// editor [`Self::layouter`], and show the editor through [`Self::show`].
pub struct ReadingOrderEdit {
    format: TextFormat,
    multiline: bool,
    laid: RefCell<Option<Laid>>,
}

impl ReadingOrderEdit {
    /// For an editor over `text` in `font` and `color`, as given to the
    /// editor (`None` for its own default ink). `None` when `text` has no
    /// right-to-left character, which egui's editor shows as it is read.
    #[must_use]
    pub fn new(
        ui: &Ui,
        text: &str,
        font: impl Into<FontSelection>,
        color: Option<Color32>,
        multiline: bool,
    ) -> Option<Self> {
        if is_logical_order(text) {
            return None;
        }
        let visuals = ui.visuals();
        let color = color
            .or(visuals.override_text_color)
            .unwrap_or_else(|| visuals.widgets.inactive.text_color());
        Some(Self {
            format: TextFormat::simple(font.into().resolve(ui.style()), color),
            multiline,
            laid: RefCell::new(None),
        })
    }

    /// The layout to give the editor ([`egui::TextEdit::layouter`]).
    pub fn layouter(&self) -> impl FnMut(&Ui, &dyn TextBuffer, f32) -> Arc<Galley> + '_ {
        move |ui: &Ui, text: &dyn TextBuffer, wrap_width: f32| {
            let wrap = if self.multiline {
                TextWrapping::wrap_at_width(wrap_width)
            } else {
                TextWrapping::no_max_width()
            };
            let (galley, laid) =
                ui.fonts_mut(|fonts| edit_layout(fonts, text.as_str(), self.format.clone(), wrap));
            *self.laid.borrow_mut() = laid;
            galley
        }
    }

    /// Runs `show`, which shows the editor given [`Self::layouter`], then
    /// draws its caret and selection where their characters are displayed,
    /// and moves a press or a drag to the character nearest the pointer.
    pub fn show(
        &self,
        ui: &mut Ui,
        show: impl FnOnce(&mut Ui) -> TextEditOutput,
    ) -> TextEditOutput {
        let layer = ui.layer_id();
        let first = ui
            .ctx()
            .graphics_mut(|graphics| graphics.entry(layer).next_idx());
        let mut output = show(ui);
        let Some(laid) = self.laid.borrow_mut().take() else {
            return output;
        };
        let origin = output.galley_pos - vec2(output.galley.rect.left(), 0.0);
        if let Some(pointer) = output.response.interact_pointer_pos()
            && let Some(range) = pointed(ui, &output, &laid, pointer - origin)
        {
            output.state.cursor.set_char_range(Some(range));
            output.state.clone().store(ui.ctx(), output.response.id);
            output.cursor_range = Some(range);
        }
        let range = output.cursor_range.filter(|_| output.response.has_focus());
        let shown = selected(&laid, range, ui.visuals());
        let caret = range.map(|range| laid.caret(range.primary).map(|end| origin + end.to_vec2()));
        let caret_stroke = ui.visuals().text_cursor.stroke;
        ui.ctx().graphics_mut(|graphics| {
            let shapes = graphics.entry(layer);
            for index in first.0..shapes.next_idx().0 {
                shapes.mutate_shape(ShapeIdx(index), |clipped| match &mut clipped.shape {
                    Shape::Text(text) if Arc::ptr_eq(&text.galley, &output.galley) => {
                        text.galley = Arc::clone(&shown);
                    }
                    Shape::LineSegment { points, stroke }
                        if *stroke == caret_stroke && points[0].x == points[1].x =>
                    {
                        if let Some(caret) = caret {
                            *points = caret;
                        }
                    }
                    _ => {}
                });
            }
        });
        output.galley = shown;
        output
    }
}

/// A field laid out in display order, and where each of its glyphs came
/// from.
struct Laid {
    /// What the editor shows, before a selection is painted into it.
    galley: Arc<Galley>,
    rows: Vec<RowMap>,
}

/// One row of a field: the logical character each glyph letters, and where
/// each character is drawn.
struct RowMap {
    /// The row's first character, counted from the start of the text.
    start: usize,
    /// The character each glyph letters, from the left.
    glyph_chars: Vec<usize>,
    /// The row's characters from its first, in logical order.
    edges: Vec<Edges>,
}

/// Where one character is drawn: the left and right edges of its grapheme
/// cluster, and which way it reads.
#[derive(Debug, Clone, Copy, Default)]
struct Edges {
    left: f32,
    right: f32,
    right_to_left: bool,
}

impl Edges {
    /// The edge a caret before the character stands at.
    fn leading(self) -> f32 {
        if self.right_to_left {
            self.right
        } else {
            self.left
        }
    }

    /// The edge a caret after the character stands at.
    fn trailing(self) -> f32 {
        if self.right_to_left {
            self.left
        } else {
            self.right
        }
    }
}

impl RowMap {
    /// Where, from the row's left, a caret before the row's `column`-th
    /// character stands, or after its last character for the column past it.
    fn caret_x(&self, column: usize) -> f32 {
        match self.edges.get(column) {
            Some(edges) => edges.leading(),
            None => self.edges.last().map_or(0.0, |edges| edges.trailing()),
        }
    }

    /// The stretches of the row, from its left, drawn for the characters in
    /// `chars`: one for each run of neighbouring glyphs, and one for the line
    /// break that ends the row when it is selected.
    fn spans(&self, row: &Row, chars: &Range<usize>, ends_with_newline: bool) -> Vec<Range<f32>> {
        let mut spans: Vec<Range<f32>> = Vec::new();
        let mut follows_selected = false;
        for (glyph, character) in row.glyphs.iter().zip(&self.glyph_chars) {
            let selected = chars.contains(character);
            if selected {
                let (left, right) = (glyph.pos.x, glyph.pos.x + glyph.advance_width);
                match spans.last_mut() {
                    Some(span) if follows_selected => {
                        span.start = span.start.min(left);
                        span.end = span.end.max(right);
                    }
                    _ => spans.push(left..right),
                }
            }
            follows_selected = selected;
        }
        if ends_with_newline && chars.contains(&(self.start + self.glyph_chars.len())) {
            spans.push(row.size.x..row.size.x + row.height() / 2.0);
        }
        spans
    }
}

impl Laid {
    /// The top and bottom of a caret before `cursor`, in the galley's frame.
    fn caret(&self, cursor: CCursor) -> [Pos2; 2] {
        let at = self.galley.layout_from_cursor(cursor);
        let placed = &self.galley.rows[at.row];
        let x = placed.pos.x + self.rows[at.row].caret_x(at.column.0);
        [pos2(x, placed.min_y() - 1.5), pos2(x, placed.max_y() + 1.5)]
    }

    /// The caret position nearest `at`, in the galley's frame: on the row
    /// under it, before whichever character's caret stands closest.
    fn cursor_at(&self, at: Vec2) -> CCursor {
        let last = self.galley.rows.len().saturating_sub(1);
        let row = self
            .galley
            .rows
            .iter()
            .position(|placed| at.y < placed.max_y())
            .unwrap_or(last);
        let x = at.x - self.galley.rows[row].pos.x;
        let map = &self.rows[row];
        let column = (0..=map.edges.len())
            .min_by(|a, b| {
                (map.caret_x(*a) - x)
                    .abs()
                    .total_cmp(&(map.caret_x(*b) - x).abs())
            })
            .unwrap_or(0);
        CCursor {
            index: CharIndex(map.start + column),
            prefer_next_row: column == 0,
        }
    }
}

/// The selection a press, drag or double click on the editor makes, with
/// the pointer at `at` in the galley's frame, as egui's editor makes it
/// from the glyph under the pointer. `None` leaves egui's: a triple click
/// selects the paragraph egui found, which is the one under the pointer.
fn pointed(ui: &Ui, output: &TextEditOutput, laid: &Laid, at: Vec2) -> Option<CCursorRange> {
    let response = &output.response;
    let hit = laid.cursor_at(at);
    if response.double_clicked() {
        let text = laid.galley.text();
        let end = ccursor_next_word(text, hit);
        return Some(CCursorRange::two(ccursor_previous_word(text, end), end));
    }
    if response.triple_clicked() {
        return None;
    }
    if !response.sense.senses_drag() {
        return (response.hovered() && response.clicked()).then(|| CCursorRange::one(hit));
    }
    let pressed = response.hovered() && ui.input(|input| input.pointer.any_pressed());
    if pressed && !ui.input(|input| input.modifiers.shift) {
        return Some(CCursorRange::one(hit));
    }
    if !pressed && !ui.ctx().is_being_dragged(response.id) {
        return None;
    }
    // A press with Shift held, or a drag: the far end follows the pointer.
    let mut range = output
        .state
        .cursor
        .char_range()
        .unwrap_or_else(|| CCursorRange::one(hit));
    range.primary = hit;
    Some(range)
}

/// The galley `laid` shows, with `range` painted as egui paints a selection:
/// a band behind each stretch of selected glyphs, and the glyphs in the
/// selection's ink.
fn selected(laid: &Laid, range: Option<CCursorRange>, visuals: &Visuals) -> Arc<Galley> {
    let Some(range) = range.filter(|range| !range.is_empty()) else {
        return Arc::clone(&laid.galley);
    };
    let [min, max] = range.sorted_cursors();
    let chars = min.index.0..max.index.0;
    let mut galley = Galley::clone(&laid.galley);
    for (placed, map) in galley.rows.iter_mut().zip(&laid.rows) {
        let spans = map.spans(&placed.row, &chars, placed.ends_with_newline);
        if spans.is_empty() {
            continue;
        }
        let row = Arc::make_mut(&mut placed.row);
        let ends = row
            .glyphs
            .iter()
            .skip(1)
            .map(|glyph| glyph.first_vertex as usize)
            .chain([row.visuals.glyph_vertex_range.end]);
        let selected_vertices: Vec<Range<usize>> = row
            .glyphs
            .iter()
            .zip(ends)
            .zip(&map.glyph_chars)
            .filter(|(_, character)| chars.contains(character))
            .map(|((glyph, end), _)| glyph.first_vertex as usize..end)
            .collect();
        for vertices in selected_vertices {
            for vertex in &mut row.visuals.mesh.vertices[vertices] {
                vertex.color = visuals.selection.stroke.color;
            }
        }
        let height = row.size.y;
        for span in spans {
            band_behind_glyphs(
                row,
                Rect::from_min_max(pos2(span.start, 0.0), pos2(span.end, height)),
                visuals.selection.bg_fill,
            );
        }
        row.visuals.mesh_bounds = row.visuals.mesh.calc_bounds();
    }
    Arc::new(galley)
}

/// Paints `rect` into `row` above its backgrounds and below its glyphs.
fn band_behind_glyphs(row: &mut Row, rect: Rect, color: Color32) {
    let mesh = &mut row.visuals.mesh;
    let before = mesh.indices.len();
    mesh.add_colored_rect(rect, color);
    let band: Vec<u32> = mesh.indices.drain(before..).collect();
    let at = row.visuals.glyph_index_start;
    mesh.indices.splice(at..at, band.iter().copied());
    row.visuals.glyph_index_start += band.len();
}

/// `text` laid out for an editor in `format`, wrapped as `wrap` says: in
/// display order, one row for each row of the logical text with its
/// characters and line breaks, the text itself in logical order, and with
/// where each glyph came from. As egui lays it out where it has no
/// right-to-left character, or where a displayed row fails to hold one glyph
/// for each character of its logical row.
fn edit_layout(
    fonts: &mut FontsView<'_>,
    text: &str,
    format: TextFormat,
    wrap: TextWrapping,
) -> (Arc<Galley>, Option<Laid>) {
    let mut logical_job = job(text, format.clone(), wrap);
    logical_job.keep_trailing_whitespace = true;
    let logical = fonts.layout_job(logical_job);
    let Some(bidi) = BidiText::new(text) else {
        return (logical, None);
    };
    let rows = row_ranges(&logical, text);
    let role = role_of(&format.font_id.family);
    let mut display_job = display_job(&bidi, &rows, format);
    display_job.keep_trailing_whitespace = true;
    let display = fonts.layout_job(display_job);
    if display.rows.len() != logical.rows.len() {
        return (logical, None);
    }
    let Some(maps) = row_maps(&bidi, role, text, &rows, &display) else {
        return (logical, None);
    };
    let mut galley = Galley::clone(&display);
    galley.job = Arc::clone(&logical.job);
    for (shown, read) in galley.rows.iter_mut().zip(&logical.rows) {
        shown.ends_with_newline = read.ends_with_newline;
    }
    let galley = Arc::new(galley);
    let laid = Laid {
        galley: Arc::clone(&galley),
        rows: maps,
    };
    (galley, Some(laid))
}

/// For each row of `display`, laid out from `rows` of `text`: the character
/// each glyph letters, and where each character is drawn. A piece's glyphs
/// letter its grapheme clusters in order, or in reverse order where it reads
/// right to left, one glyph for each character of a cluster. `None` where a
/// row holds a different number of glyphs.
fn row_maps(
    bidi: &BidiText<'_>,
    role: Role,
    text: &str,
    rows: &[Range<usize>],
    display: &Galley,
) -> Option<Vec<RowMap>> {
    let mut char_at = vec![0; text.len() + 1];
    for (index, (at, character)) in text.char_indices().enumerate() {
        char_at[at..at + character.len_utf8()].fill(index);
    }
    char_at[text.len()] = text.chars().count();
    let mut maps = Vec::with_capacity(rows.len());
    for (row, placed) in rows.iter().zip(&display.rows) {
        let start = char_at[row.start];
        let mut glyph_chars = Vec::new();
        let mut clusters = Vec::new();
        for piece in bidi.line(role, row.clone()) {
            let mut graphemes: Vec<(usize, &str)> =
                text[piece.logical.clone()].grapheme_indices(true).collect();
            if piece.right_to_left {
                graphemes.reverse();
            }
            for (offset, cluster) in graphemes {
                let first = char_at[piece.logical.start + offset];
                let count = cluster.chars().count();
                clusters.push((
                    glyph_chars.len()..glyph_chars.len() + count,
                    piece.right_to_left,
                ));
                glyph_chars.extend(first..first + count);
            }
        }
        let glyphs = &placed.row.glyphs;
        if glyphs.len() != glyph_chars.len() || char_at[row.end] - start != glyph_chars.len() {
            return None;
        }
        let mut edges = vec![Edges::default(); glyph_chars.len()];
        for (cluster, right_to_left) in clusters {
            let left = cluster
                .clone()
                .map(|glyph| glyphs[glyph].pos.x)
                .fold(f32::INFINITY, f32::min);
            let right = cluster
                .clone()
                .map(|glyph| glyphs[glyph].pos.x + glyphs[glyph].advance_width)
                .fold(f32::NEG_INFINITY, f32::max);
            for glyph in cluster {
                edges[glyph_chars[glyph] - start] = Edges {
                    left,
                    right,
                    right_to_left,
                };
            }
        }
        maps.push(RowMap {
            start,
            glyph_chars,
            edges,
        });
    }
    Some(maps)
}

#[cfg(test)]
mod tests;
