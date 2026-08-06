//! Hydrated plot pane: axes, grid, series, and cursor readout over the
//! sealed datasets.
//!
//! Axis arithmetic lives in pure functions so tick placement and
//! engineering-notation formatting are unit-tested natively; the widget
//! itself only maps data through those functions and paints.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke as EguiStroke, Vec2};
use rspice_publication_contract::AxisScale;

use crate::payload::HydratedPlot;
use crate::theme::Palette;

/// Format a value in engineering notation with an SI prefix, the notation
/// every commercial simulator's cursor readout uses.
#[must_use]
pub fn engineering(value: f64) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    if !value.is_finite() {
        return if value.is_nan() {
            "NaN".to_owned()
        } else if value > 0.0 {
            "∞".to_owned()
        } else {
            "-∞".to_owned()
        };
    }
    const PREFIXES: [(i32, &str); 17] = [
        (-24, "y"),
        (-21, "z"),
        (-18, "a"),
        (-15, "f"),
        (-12, "p"),
        (-9, "n"),
        (-6, "µ"),
        (-3, "m"),
        (0, ""),
        (3, "k"),
        (6, "M"),
        (9, "G"),
        (12, "T"),
        (15, "P"),
        (18, "E"),
        (21, "Z"),
        (24, "Y"),
    ];
    let exponent = value.abs().log10().floor() as i32;
    let engineering_exponent = (exponent.div_euclid(3) * 3).clamp(-24, 24);
    let mantissa = value / 10f64.powi(engineering_exponent);
    let prefix = PREFIXES
        .iter()
        .find(|(e, _)| *e == engineering_exponent)
        .map_or("", |(_, p)| p);
    // Three significant digits after rounding the mantissa's magnitude.
    let magnitude = mantissa.abs();
    let decimals = if magnitude >= 100.0 {
        0
    } else if magnitude >= 10.0 {
        1
    } else {
        2
    };
    let formatted = format!("{mantissa:.decimals$}");
    // Rounding can carry 999.6 → 1000; keep the display within three digits.
    if formatted.trim_start_matches('-').starts_with("1000") {
        let carried = mantissa / 1000.0;
        let carried_exponent = (engineering_exponent + 3).clamp(-24, 24);
        if carried_exponent != engineering_exponent {
            let prefix = PREFIXES
                .iter()
                .find(|(e, _)| *e == carried_exponent)
                .map_or("", |(_, p)| p);
            return format!("{carried:.2}{prefix}");
        }
    }
    format!("{formatted}{prefix}")
}

/// Map a data value onto the axis' plotting coordinate.
#[must_use]
pub fn axis_coordinate(scale: AxisScale, value: f64) -> Option<f64> {
    match scale {
        AxisScale::Linear => value.is_finite().then_some(value),
        AxisScale::Logarithmic => (value.is_finite() && value > 0.0).then(|| value.log10()),
    }
}

/// Inclusive plotting range over every finite coordinate of every series.
#[must_use]
pub fn coordinate_range(coordinates: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut range: Option<(f64, f64)> = None;
    for value in coordinates {
        if !value.is_finite() {
            continue;
        }
        range = Some(match range {
            None => (value, value),
            Some((low, high)) => (low.min(value), high.max(value)),
        });
    }
    // A flat range still needs height to plot.
    range.map(|(low, high)| {
        if low == high {
            let pad = if low == 0.0 { 1.0 } else { low.abs() * 0.1 };
            (low - pad, high + pad)
        } else {
            (low, high)
        }
    })
}

/// Tick positions for one axis in plotting coordinates.
///
/// Linear axes step by the classic 1/2/5 ladder; logarithmic axes tick each
/// decade (the plotting coordinate is already log10).
#[must_use]
pub fn ticks(scale: AxisScale, low: f64, high: f64, target: usize) -> Vec<f64> {
    if !(low.is_finite() && high.is_finite()) || high <= low || target == 0 {
        return Vec::new();
    }
    match scale {
        AxisScale::Linear => {
            let raw_step = (high - low) / target as f64;
            let magnitude = 10f64.powf(raw_step.log10().floor());
            let residual = raw_step / magnitude;
            let step = magnitude
                * if residual > 5.0 {
                    10.0
                } else if residual > 2.0 {
                    5.0
                } else if residual > 1.0 {
                    2.0
                } else {
                    1.0
                };
            let first = (low / step).ceil();
            let mut ticks = Vec::new();
            let mut index = first;
            while index * step <= high + step * 1e-9 {
                ticks.push(index * step);
                index += 1.0;
            }
            ticks
        }
        AxisScale::Logarithmic => {
            let mut ticks = Vec::new();
            let mut decade = low.ceil();
            while decade <= high + 1e-9 {
                ticks.push(decade);
                decade += 1.0;
            }
            ticks
        }
    }
}

/// Display label for a tick, undoing the plotting-coordinate mapping.
#[must_use]
pub fn tick_label(scale: AxisScale, coordinate: f64) -> String {
    match scale {
        AxisScale::Linear => engineering(coordinate),
        AxisScale::Logarithmic => engineering(10f64.powf(coordinate)),
    }
}

/// Nearest sample index to a target abscissa coordinate.
#[must_use]
pub fn nearest_sample(coordinates: &[f64], target: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (index, &value) in coordinates.iter().enumerate() {
        if !value.is_finite() {
            continue;
        }
        let distance = (value - target).abs();
        if best.is_none_or(|(_, best_distance)| distance < best_distance) {
            best = Some((index, distance));
        }
    }
    best.map(|(index, _)| index)
}

/// Per-series plotting coordinates: abscissas and ordinates, `None` where a
/// value cannot appear on the axis (non-finite, or non-positive on log).
type SeriesCoordinates = (Vec<Option<f64>>, Vec<Option<f64>>);

const MARGIN_LEFT: f32 = 64.0;
const MARGIN_BOTTOM: f32 = 28.0;
const MARGIN_TOP: f32 = 10.0;
const MARGIN_RIGHT: f32 = 12.0;

/// The hydrated plot pane.
pub fn plot_pane(ui: &mut egui::Ui, plot: &HydratedPlot) {
    let palette = Palette::for_dark_mode(ui.visuals().dark_mode);
    let available = ui.available_size();
    let size = Vec2::new(
        available.x,
        (available.x * 0.55).clamp(180.0, available.y.max(180.0)),
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter_at(rect);

    let frame = Rect::from_min_max(
        Pos2::new(rect.min.x + MARGIN_LEFT, rect.min.y + MARGIN_TOP),
        Pos2::new(rect.max.x - MARGIN_RIGHT, rect.max.y - MARGIN_BOTTOM),
    );

    // Plotting coordinates for every series, with per-point validity.
    let series_coordinates: Vec<SeriesCoordinates> = plot
        .series
        .iter()
        .map(|series| {
            (
                series
                    .x
                    .iter()
                    .map(|&value| axis_coordinate(plot.x_scale, value))
                    .collect(),
                series
                    .y
                    .iter()
                    .map(|&value| axis_coordinate(plot.y_scale, value))
                    .collect(),
            )
        })
        .collect();

    let x_range = coordinate_range(
        series_coordinates
            .iter()
            .flat_map(|(x, _)| x.iter().copied().flatten()),
    );
    let y_range = coordinate_range(
        series_coordinates
            .iter()
            .flat_map(|(_, y)| y.iter().copied().flatten()),
    );
    let (Some((x_low, x_high)), Some((y_low, y_high))) = (x_range, y_range) else {
        painter.text(
            frame.center(),
            Align2::CENTER_CENTER,
            "no plottable samples",
            FontId::proportional(13.0),
            palette.secondary,
        );
        return;
    };

    let to_screen = |x: f64, y: f64| -> Pos2 {
        Pos2::new(
            frame.min.x + ((x - x_low) / (x_high - x_low)) as f32 * frame.width(),
            frame.max.y - ((y - y_low) / (y_high - y_low)) as f32 * frame.height(),
        )
    };

    // Grid and tick labels.
    let grid_stroke = EguiStroke::new(0.5, palette.grid);
    let label_font = FontId::proportional(10.0);
    for tick in ticks(plot.x_scale, x_low, x_high, 6) {
        let screen = to_screen(tick, y_low);
        painter.add(Shape::line_segment(
            [
                Pos2::new(screen.x, frame.min.y),
                Pos2::new(screen.x, frame.max.y),
            ],
            grid_stroke,
        ));
        painter.text(
            Pos2::new(screen.x, frame.max.y + 4.0),
            Align2::CENTER_TOP,
            tick_label(plot.x_scale, tick),
            label_font.clone(),
            palette.secondary,
        );
    }
    for tick in ticks(plot.y_scale, y_low, y_high, 5) {
        let screen = to_screen(x_low, tick);
        painter.add(Shape::line_segment(
            [
                Pos2::new(frame.min.x, screen.y),
                Pos2::new(frame.max.x, screen.y),
            ],
            grid_stroke,
        ));
        painter.text(
            Pos2::new(frame.min.x - 6.0, screen.y),
            Align2::RIGHT_CENTER,
            tick_label(plot.y_scale, tick),
            label_font.clone(),
            palette.secondary,
        );
    }
    painter.rect_stroke(
        frame,
        0.0,
        EguiStroke::new(1.0, palette.secondary),
        egui::StrokeKind::Inside,
    );
    painter.text(
        Pos2::new(frame.center().x, rect.max.y - 2.0),
        Align2::CENTER_BOTTOM,
        &plot.x_label,
        label_font.clone(),
        palette.secondary,
    );

    // Series polylines, broken at unplottable points.
    for (series, (x_coords, y_coords)) in plot.series.iter().zip(&series_coordinates) {
        let color = palette.traces[usize::from(series.series_index)];
        let stroke = EguiStroke::new(1.5, color);
        let mut run: Vec<Pos2> = Vec::new();
        for (x, y) in x_coords.iter().zip(y_coords) {
            match (x, y) {
                (Some(x), Some(y)) => run.push(to_screen(*x, *y)),
                _ => {
                    if run.len() >= 2 {
                        painter.add(Shape::line(core::mem::take(&mut run), stroke));
                    }
                    run.clear();
                }
            }
        }
        if run.len() >= 2 {
            painter.add(Shape::line(run, stroke));
        }
    }

    // Cursor readout: nearest abscissa sample per series.
    if let Some(pointer) = response.hover_pos()
        && frame.contains(pointer)
    {
        let target_x =
            x_low + f64::from((pointer.x - frame.min.x) / frame.width()) * (x_high - x_low);
        painter.add(Shape::line_segment(
            [
                Pos2::new(pointer.x, frame.min.y),
                Pos2::new(pointer.x, frame.max.y),
            ],
            EguiStroke::new(0.75, palette.accent),
        ));
        let mut lines: Vec<(Color32, String)> = Vec::new();
        for (series, (x_coords, y_coords)) in plot.series.iter().zip(&series_coordinates) {
            let flattened: Vec<f64> = x_coords
                .iter()
                .map(|value| value.unwrap_or(f64::NAN))
                .collect();
            let Some(index) = nearest_sample(&flattened, target_x) else {
                continue;
            };
            if y_coords.get(index).copied().flatten().is_some() {
                let color = palette.traces[usize::from(series.series_index)];
                let value = engineering(series.y[index]);
                let unit = &series.unit;
                let x_value = engineering(series.x[index]);
                lines.push((
                    color,
                    format!("{}: {value}{unit} @ {x_value}", series.label),
                ));
            }
        }
        if !lines.is_empty() {
            let line_height = 14.0;
            let box_height = 8.0 + line_height * lines.len() as f32;
            let box_width = 8.0
                + lines
                    .iter()
                    .map(|(_, text)| text.len() as f32 * 6.2)
                    .fold(0.0, f32::max);
            let anchor = Pos2::new(
                (pointer.x + 12.0).min(frame.max.x - box_width),
                (pointer.y - box_height - 8.0).max(frame.min.y),
            );
            let readout = Rect::from_min_size(anchor, Vec2::new(box_width, box_height));
            painter.rect_filled(
                readout,
                3.0,
                if ui.visuals().dark_mode {
                    Color32::from_black_alpha(200)
                } else {
                    Color32::from_white_alpha(230)
                },
            );
            for (row, (color, text)) in lines.iter().enumerate() {
                painter.text(
                    Pos2::new(
                        readout.min.x + 4.0,
                        readout.min.y + 4.0 + line_height * row as f32,
                    ),
                    Align2::LEFT_TOP,
                    text,
                    FontId::monospace(10.0),
                    *color,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_notation_uses_si_prefixes() {
        assert_eq!(engineering(0.0), "0");
        assert_eq!(engineering(1.0), "1.00");
        assert_eq!(engineering(3.162e-6), "3.16µ");
        assert_eq!(engineering(-2.5e3), "-2.50k");
        assert_eq!(engineering(4.7e-10), "470p");
        assert_eq!(engineering(1.5e9), "1.50G");
    }

    #[test]
    fn engineering_notation_carries_rounding_across_the_prefix_boundary() {
        assert_eq!(engineering(999.9999), "1.00k");
    }

    #[test]
    fn linear_ticks_step_the_one_two_five_ladder() {
        let ticks = ticks(AxisScale::Linear, 0.0, 1.0, 6);
        assert_eq!(ticks, vec![0.0, 0.2, 0.4, 0.6000000000000001, 0.8, 1.0]);
    }

    #[test]
    fn log_ticks_mark_each_decade() {
        let ticks = ticks(AxisScale::Logarithmic, 0.0, 3.0, 6);
        assert_eq!(ticks, vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(tick_label(AxisScale::Logarithmic, 3.0), "1.00k");
    }

    #[test]
    fn log_axis_drops_non_positive_values_for_line_breaks() {
        assert_eq!(axis_coordinate(AxisScale::Logarithmic, 100.0), Some(2.0));
        assert_eq!(axis_coordinate(AxisScale::Logarithmic, 0.0), None);
        assert_eq!(axis_coordinate(AxisScale::Logarithmic, -1.0), None);
        assert_eq!(axis_coordinate(AxisScale::Linear, f64::NEG_INFINITY), None);
    }

    #[test]
    fn a_flat_series_still_gets_a_plottable_range() {
        let range = coordinate_range([5.0, 5.0, 5.0].into_iter()).expect("range");
        assert!(range.0 < 5.0 && range.1 > 5.0);
    }

    #[test]
    fn nearest_sample_skips_unplottable_points() {
        let index = nearest_sample(&[f64::NAN, 1.0, 2.0], 1.9).expect("nearest");
        assert_eq!(index, 2);
    }
}
