//! The row every rectangular plot reads its abscissa from.
//!
//! One grammar, whatever the analysis, as ViVA and ADS draw every
//! rectangular graph in one axis style: the quantity and its unit in the
//! left gutter ("TIME · s", "FREQ · Hz"), bare SI-prefixed values under the
//! plot, and the values at the plot's edges held inside it. A waveform
//! stack's shared ruler and a plot standing on its own both paint their
//! tick row here, so the two read alike to the pixel.

use std::sync::Arc;

use egui::{Color32, Galley, Painter, Rangef, Rect, pos2};

use super::{Axis, XScale};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// The height of the tick row: the values' band under the plot. A ruler
/// that carries more under it (cursor boxes, an overview track) starts them
/// here.
pub const X_AXIS_ROW_HEIGHT: f32 = 17.0;
/// Where the tick values' tops stand below the row's top edge.
const LABEL_TOP: f32 = 3.0;
/// How far a value may run into a column it gives way to and still count
/// as touching it: a box placed a pad clear of a value, its pad taken back
/// off in another sum, lands within rounding of the value's edge.
const TOUCH: f32 = 0.25;
/// The least space between two tick values that are both drawn.
const LABEL_GAP: f32 = 6.0;
/// The gap between the gutter caption and the plot it stands under.
pub const X_CAPTION_GAP: f32 = 8.0;
/// The least space between the gutter caption and the row's left edge.
const CAPTION_EDGE: f32 = 2.0;

/// What the gutter says beside an abscissa's values: the quantity, then the
/// unit — "TIME · s", "FREQ · Hz", "VD · V", "TEMP · °C". Time and
/// frequency are named by their unit; an axis with a unit but no quantity
/// ("UI", "Re") states the unit alone.
///
/// The tick values themselves are SI-prefixed bare numbers, so the gutter is
/// where the axis states its unit, once, for the whole row.
pub fn x_axis_caption(quantity: &str, unit: &str) -> String {
    let name = match unit {
        "s" => "TIME".to_owned(),
        "Hz" => "FREQ".to_owned(),
        // ASCII only: an uppercase σ is Σ, the summation sign.
        _ => quantity.trim().to_ascii_uppercase(),
    };
    match (name.is_empty(), unit.is_empty()) {
        (true, _) => unit.to_owned(),
        (false, true) => name,
        (false, false) => format!("{name} · {unit}"),
    }
}

/// An abscissa as its row draws it.
pub struct XAxisRow<'a> {
    /// The ticks of the range the plot above shows.
    pub axis: &'a Axis,
    pub scale: XScale,
    /// The plot's left and right edges, which the values stand between.
    pub plot: Rangef,
    /// The words in the gutter: see [`x_axis_caption`].
    pub caption: &'a str,
    /// Columns a value gives way to, so it never reads as part of a box
    /// standing there (a cursor's value on a ruler).
    pub keep_clear: &'a [Rangef],
}

/// The band a tick row stands on: the panel's fill, with a rule on its top
/// edge where the plot above ends.
pub fn paint_x_axis_band(painter: &Painter, rect: Rect, t: &Tokens) {
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.top(),
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// Where the values of a row over `plot` stand, left to right, as
/// [`paint_x_axis_row`] sets them with nothing to keep clear: the stretches a
/// box sharing the row slides along to stay off.
pub fn x_axis_value_spans(
    painter: &Painter,
    axis: &Axis,
    scale: XScale,
    plot: Rangef,
    t: &Tokens,
) -> Vec<Rangef> {
    row_values(painter, axis, scale, plot, &[], t)
        .into_iter()
        .map(|(left, galley, _)| Rangef::new(left, left + galley.size().x))
        .collect()
}

/// The values a row sets, each with its left edge and ink: the offset anchor
/// first when the values are offsets from one, then every tick value with
/// room beside the last, held inside the plot's span and clear of
/// `keep_clear`.
fn row_values(
    painter: &Painter,
    axis: &Axis,
    scale: XScale,
    plot: Rangef,
    keep_clear: &[Rangef],
    t: &Tokens,
) -> Vec<(f32, Arc<Galley>, Color32)> {
    let c = t.color;
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut values = Vec::new();
    let mut last_right = f32::NEG_INFINITY;
    // Past a certain zoom the values stop being values and become
    // differences from one. The row owes the reader that value: a row
    // reading "−40n … 0 … +40n" with nothing to subtract them from states a
    // window near zero, which is not where the reader is. Stated once, at
    // the head of the row and brighter than the values.
    if let Some(anchor) = axis.offset_anchor() {
        let galley = painter.layout_no_wrap(anchor.to_owned(), font.clone(), c.text);
        last_right = plot.min + galley.size().x;
        values.push((plot.min, galley, c.text));
    }
    for (value, label) in &axis.ticks {
        let fraction = scale.normalize(*value, axis.min, axis.max);
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
            continue;
        }
        let x = plot.min + plot.span() * fraction as f32;
        let galley = painter.layout_no_wrap(label.clone(), font.clone(), c.text_dim);
        // No stub under the frame: the tick's gridline already runs to it and
        // marks the true position, and a stub read as that gridline drawn past
        // the plot. The value itself is held inside the plot's span, so an edge
        // value never reaches into the gutter and collides with the caption,
        // nor past the plot's right edge.
        let half = galley.size().x * 0.5 + 2.0;
        let centre = x.clamp(plot.min + half, (plot.max - half).max(plot.min + half));
        let left = centre - galley.size().x * 0.5;
        let span = Rangef::new(left, left + galley.size().x);
        // A value gives way only to a column it overlaps: one a box slid
        // off it stands clear of by exactly the pad, and keeps its place.
        let kept_clear = keep_clear
            .iter()
            .any(|column| column.min + TOUCH < span.max && span.min + TOUCH < column.max);
        if left >= last_right + LABEL_GAP && !kept_clear {
            last_right = left + galley.size().x;
            values.push((left, galley, c.text_dim));
        }
    }
    values
}

/// Paint `row` into `rect`, whose top edge is the plot's bottom edge and
/// whose left edge is the plot's gutter's, over its band
/// ([`paint_x_axis_band`]): the tick values, the offset anchor
/// when the values are offsets from one, and the caption in the gutter,
/// which breaks at its unit and wraps within `rect` when the gutter is too
/// narrow for one line. Returns where the caption stands.
pub fn paint_x_axis_row(painter: &Painter, rect: Rect, row: XAxisRow<'_>, t: &Tokens) -> Rect {
    // The row is measured from the pixel row its rule lands on, so its
    // values stand the same distance under the rule wherever a layout puts
    // the row's edge between two pixels.
    let ppp = painter.pixels_per_point();
    let top = painter.round_to_pixel_center(rect.top()) - 0.5 / ppp;
    let rect = Rect::from_min_max(pos2(rect.left(), top), rect.max);
    let c = t.color;
    let XAxisRow {
        axis,
        scale,
        plot,
        caption,
        keep_clear,
    } = row;
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let label_top = rect.top() + LABEL_TOP;
    for (left, galley, ink) in row_values(painter, axis, scale, plot, keep_clear, t) {
        painter.galley(pos2(left, label_top), galley, ink);
    }
    // The gutter keeps its width whatever the caption, so every plot keeps
    // one left edge. A caption too long for it breaks at its unit, the
    // quantity over the unit, and a quantity too long for a line wraps
    // within it.
    let gutter_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let right = plot.min - X_CAPTION_GAP;
    let column = (right - rect.left() - CAPTION_EDGE).max(1.0);
    let one_line = painter.layout_no_wrap(caption.to_owned(), gutter_font.clone(), c.text_faint);
    let caption_top = label_top + font.size * 0.5 - one_line.size().y * 0.5;
    let galley = if one_line.size().x <= column {
        one_line
    } else {
        let line = one_line.size().y.max(1.0);
        let text = caption.rsplit_once(" \u{b7} ").map_or_else(
            || caption.to_owned(),
            |(quantity, unit)| format!("{quantity}\n{unit}"),
        );
        let mut job = egui::text::LayoutJob::simple(text, gutter_font, c.text_faint, column);
        job.halign = egui::Align::RIGHT;
        job.wrap.break_anywhere = true;
        job.wrap.max_rows = (((rect.bottom() - caption_top) / line).floor() as usize).max(1);
        painter.layout_job(job)
    };
    // A left-aligned galley spans 0..width, a right-aligned one -width..0:
    // either way its right end lands on the caption's edge.
    let at = pos2(right - galley.rect.max.x, caption_top);
    let placed = galley.rect.translate(at.to_vec2());
    painter.galley(at, galley, c.text_faint);
    placed
}

#[cfg(test)]
mod tests {
    use super::x_axis_caption;

    #[test]
    fn the_gutter_names_the_quantity_with_its_unit() {
        assert_eq!(x_axis_caption("t", "s"), "TIME \u{b7} s");
        assert_eq!(x_axis_caption("", "Hz"), "FREQ \u{b7} Hz");
        assert_eq!(x_axis_caption("VD", "V"), "VD \u{b7} V");
        assert_eq!(x_axis_caption("Ibias", "A"), "IBIAS \u{b7} A");
        assert_eq!(x_axis_caption("temp", "\u{b0}C"), "TEMP \u{b7} \u{b0}C");
        assert_eq!(x_axis_caption("iteration", ""), "ITERATION");
        assert_eq!(x_axis_caption("σ", "rad/s"), "σ · rad/s");
        // An axis with a unit but no quantity states the unit alone, and an
        // axis with neither says nothing.
        assert_eq!(x_axis_caption("", "UI"), "UI");
        assert_eq!(x_axis_caption("", ""), "");
    }
}
