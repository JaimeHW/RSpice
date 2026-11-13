//! The scale a Verify comparison plot is read against.
//!
//! The tuning response and the regression waveform are the same picture — a
//! baseline curve and a candidate over it — so they carry the same axes: a
//! gutter of value ticks on the left, a band of sweep ticks underneath, the
//! 1-2-5 ladder and the engineering labels the histogram's axis already uses.
//! A curve drawn without them is a shape; with them it is a measurement.

use egui::{Rect, Stroke, Ui, pos2};

use crate::state::AnalysisType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::histogram::{nice_step, tick_text};

/// Room left of the plot for the value ticks, and under it for the sweep.
pub(super) const GUTTER_W: f32 = 56.0;
pub(super) const BAND_H: f32 = 16.0;
/// Least plot left after both; under it the axes are not drawn at all.
const LEAST_PLOT: f32 = 40.0;

/// What a plot's two axes carry: the span of each, and its unit.
pub(super) struct Axes<'a> {
    pub(super) x: (f64, f64),
    pub(super) y: (f64, f64),
    pub(super) x_unit: &'a str,
    pub(super) y_unit: &'a str,
}

/// The unit a run's sweep axis is in.
///
/// Only where the analysis alone settles it: a transient sweeps time and the
/// small-signal families sweep frequency. Everything else sweeps a quantity
/// its own plan names, and guessing one would label the axis wrongly.
pub(super) fn sweep_unit(analysis: AnalysisType) -> &'static str {
    match analysis {
        AnalysisType::Transient => "s",
        AnalysisType::Ac | AnalysisType::Noise => "Hz",
        _ => "",
    }
}

/// The ticks across `span`, on the 1-2-5 ladder, and the step they sit on.
fn ticks(span: (f64, f64), room: f32) -> (Vec<f64>, f64) {
    let (low, high) = span;
    let divisions = (room / 90.0).clamp(2.0, 8.0) as f64;
    let step = nice_step((high - low).abs().max(f64::MIN_POSITIVE) / divisions);
    let mut ticks = Vec::new();
    let mut tick = (low / step).ceil() * step;
    while tick <= high + step * 1.0e-9 {
        ticks.push(if tick.abs() < step * 1.0e-9 {
            0.0
        } else {
            tick
        });
        tick += step;
    }
    (ticks, step)
}

/// Paint the axes around `rect` and return the plot area left inside it.
///
/// The returned rect is what a caller maps its samples onto; when `rect` is
/// too small to carry a scale, it is handed back whole and nothing is drawn,
/// because a tick band that overlaps the curve is worse than none.
pub(super) fn paint(ui: &Ui, rect: Rect, axes: &Axes<'_>) -> Rect {
    let t = Tokens::get(ui.ctx());
    if rect.width() - GUTTER_W < LEAST_PLOT || rect.height() - BAND_H < LEAST_PLOT {
        ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
        return rect;
    }
    let plot = Rect::from_min_max(
        pos2(rect.left() + GUTTER_W, rect.top()),
        pos2(rect.right(), rect.bottom() - BAND_H),
    );
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(plot, 0.0, t.color.canvas_bg);
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let grid = Stroke::new(1.0, t.color.canvas_grid);

    let (value_ticks, value_step) = ticks(axes.y, plot.height());
    for value in &value_ticks {
        let y = egui::remap(
            *value,
            axes.y.0..=axes.y.1,
            plot.bottom() as f64..=plot.top() as f64,
        ) as f32;
        painter.hline(plot.x_range(), y, grid);
        painter.text(
            pos2(plot.left() - 6.0, y),
            egui::Align2::RIGHT_CENTER,
            tick_text(*value, value_step, axes.y_unit),
            font.clone(),
            t.color.text_faint,
        );
    }

    let (sweep_ticks, sweep_step) = ticks(axes.x, plot.width());
    let mut last_right = f32::NEG_INFINITY;
    for value in &sweep_ticks {
        let x = egui::remap(
            *value,
            axes.x.0..=axes.x.1,
            plot.left() as f64..=plot.right() as f64,
        ) as f32;
        painter.vline(x, plot.y_range(), grid);
        let galley = painter.layout_no_wrap(
            tick_text(*value, sweep_step, axes.x_unit),
            font.clone(),
            t.color.text_faint,
        );
        // A label is centred on its tick, kept inside the plot's own edges,
        // and dropped when it would run into the one before it.
        let left = (x - galley.size().x * 0.5)
            .clamp(rect.left(), rect.right() - galley.size().x)
            .max(last_right + 8.0);
        if left + galley.size().x > rect.right() + 0.5 {
            continue;
        }
        last_right = left + galley.size().x;
        painter.galley(pos2(left, plot.bottom() + 2.0), galley, t.color.text_faint);
    }
    painter.hline(
        plot.x_range(),
        plot.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    painter.vline(
        plot.left(),
        plot.y_range(),
        Stroke::new(1.0, t.color.border_strong),
    );
    plot
}

/// Map `(x, y)` pairs onto `plot` under the spans `axes` states.
pub(super) fn points(
    plot: Rect,
    axes: &Axes<'_>,
    x_values: &[f64],
    y_values: &[f64],
) -> Vec<egui::Pos2> {
    let span = |low: f64, high: f64, value: f64| {
        if (high - low).abs() <= f64::EPSILON {
            0.5
        } else {
            ((value - low) / (high - low)) as f32
        }
    };
    x_values
        .iter()
        .zip(y_values)
        .filter(|(x, y)| x.is_finite() && y.is_finite())
        .map(|(x, y)| {
            pos2(
                egui::lerp(plot.left()..=plot.right(), span(axes.x.0, axes.x.1, *x)),
                egui::lerp(plot.bottom()..=plot.top(), span(axes.y.0, axes.y.1, *y)),
            )
        })
        .collect()
}

/// A legend chip row, right-aligned in `row`: one short rule per entry.
pub(super) fn legend(ui: &Ui, row: Rect, entries: &[(&str, Stroke, bool)]) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let painter = ui.painter().with_clip_rect(row);
    let mut x = row.right();
    for (label, stroke, dashed) in entries {
        let galley = painter.layout_no_wrap((*label).to_owned(), font.clone(), t.color.text_dim);
        x -= galley.size().x;
        painter.galley(
            pos2(x, row.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
        let line = [
            pos2(x - 20.0, row.center().y),
            pos2(x - 6.0, row.center().y),
        ];
        if *dashed {
            painter.extend(egui::Shape::dashed_line(&line, *stroke, 3.0, 2.0));
        } else {
            painter.line_segment(line, *stroke);
        }
        x -= 20.0 + 14.0;
    }
}
