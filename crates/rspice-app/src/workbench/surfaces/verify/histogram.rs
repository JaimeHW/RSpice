//! The distribution of one Monte Carlo output, drawn as a histogram.
//!
//! Bars are neutral; a bar whose samples break the limit is a muted error
//! tone. The limit is a solid vertical rule, the mean a thin one, and the mean
//! plus and minus three standard deviations thin dashed ones. The bins are laid
//! so a limit falls on a bin edge — a bar is either wholly inside the limit or
//! wholly outside it, never a bar of mixed samples painted one colour — and the
//! axis ticks are engineering notation with the output's unit.
//!
//! Everything is computed from the exact retained samples. A run that kept no
//! samples has no histogram.

use egui::{Rect, Sense, Shape, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::table::TABLE_EDGE_INSET;

/// Height of the histogram's header row.
const HEADER_H: f32 = 24.0;
/// Room under the plot for the axis ticks.
const TICK_BAND_H: f32 = 20.0;
/// Room left of the plot for the count labels.
const COUNT_GUTTER_W: f32 = 28.0;

/// One output's distribution, binned and measured.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct HistogramModel {
    /// Bin edges, one more than there are bins.
    pub(super) edges: Vec<f64>,
    pub(super) counts: Vec<usize>,
    /// Whether each bin's samples break the limit.
    pub(super) failing: Vec<bool>,
    /// The limits that fall inside the axis.
    pub(super) limits: Vec<f64>,
    pub(super) mean: f64,
    pub(super) std_dev: f64,
    /// The axis: the samples, the limits and mean ± 3 standard deviations.
    pub(super) axis: (f64, f64),
    pub(super) tick_step: f64,
    pub(super) ticks: Vec<f64>,
}

/// A 1-2-5 step near `raw`.
fn nice_step(raw: f64) -> f64 {
    if !(raw.is_finite() && raw > 0.0) {
        return 1.0;
    }
    let magnitude = 10f64.powf(raw.log10().floor());
    let fraction = raw / magnitude;
    let nice = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 2.5 {
        2.5
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * magnitude
}

/// Bin `samples` against `min`/`max` limits.
///
/// Returns `None` when there is no finite sample to bin.
pub(super) fn histogram_model(
    samples: &[f64],
    min: Option<f64>,
    max: Option<f64>,
) -> Option<HistogramModel> {
    let finite = samples
        .iter()
        .copied()
        .filter(|sample| sample.is_finite())
        .collect::<Vec<_>>();
    if finite.is_empty() {
        return None;
    }
    let count = finite.len() as f64;
    let mean = finite.iter().sum::<f64>() / count;
    let std_dev = if finite.len() > 1 {
        (finite
            .iter()
            .map(|sample| (sample - mean).powi(2))
            .sum::<f64>()
            / (count - 1.0))
            .sqrt()
    } else {
        0.0
    };
    let low = finite.iter().copied().fold(f64::INFINITY, f64::min);
    let high = finite.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let limits = [min, max].into_iter().flatten().collect::<Vec<_>>();
    let mut axis_low = low.min(mean - 3.0 * std_dev);
    let mut axis_high = high.max(mean + 3.0 * std_dev);
    for limit in &limits {
        axis_low = axis_low.min(*limit);
        axis_high = axis_high.max(*limit);
    }
    if axis_high - axis_low <= f64::EPSILON * axis_low.abs().max(1.0) {
        let half = axis_low.abs().max(1.0) * 0.05;
        axis_low -= half;
        axis_high += half;
    }

    // Bin width: about 1.5·√n bins across the samples' own spread, so forty
    // samples get nine or ten bars and ten thousand get the cap.
    let target_bins = (count.sqrt() * 1.5).clamp(8.0, 48.0);
    let spread = (high - low).max((axis_high - axis_low) * 0.25);
    let width = nice_step(spread / target_bins);
    // A limit sits on a bin edge; without one, zero does.
    let anchor = limits.first().copied().unwrap_or(0.0);
    let first = anchor - ((anchor - axis_low) / width).ceil() * width;
    let bins = (((axis_high - first) / width).ceil() as usize).max(1);
    let edges = (0..=bins)
        .map(|index| first + index as f64 * width)
        .collect::<Vec<_>>();
    let mut counts = vec![0; bins];
    for sample in &finite {
        let index = (((sample - first) / width).floor().max(0.0) as usize).min(bins - 1);
        counts[index] += 1;
    }
    let passes =
        |value: f64| min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max);
    let failing = edges
        .windows(2)
        .map(|edge| !passes((edge[0] + edge[1]) * 0.5))
        .collect::<Vec<_>>();

    let axis = (edges[0], edges[bins]);
    let tick_step = nice_step((axis.1 - axis.0) / 8.0);
    let mut ticks = Vec::new();
    let mut tick = (axis.0 / tick_step).ceil() * tick_step;
    while tick <= axis.1 + tick_step * 1.0e-9 {
        ticks.push(if tick.abs() < tick_step * 1.0e-9 {
            0.0
        } else {
            tick
        });
        tick += tick_step;
    }
    Some(HistogramModel {
        edges,
        counts,
        failing,
        limits,
        mean,
        std_dev,
        axis,
        tick_step,
        ticks,
    })
}

/// A tick's label: engineering notation with the output's unit, with as many
/// places as the step needs for two neighbours to differ.
pub(super) fn tick_text(value: f64, step: f64, unit: &str) -> String {
    rspice_results::number_format::tick_with_unit(
        &rspice_results::number_format::tick_label_with_step(value, step),
        unit,
    )
}

/// Paint `model` across the rest of `ui`'s height, under a header naming
/// `output`.
pub(super) fn paint_histogram(
    ui: &mut Ui,
    output: &str,
    unit: &str,
    model: &HistogramModel,
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    #[cfg(test)]
    super::raster::probe(ui, "histogram", rect);
    let total = model.counts.iter().sum::<usize>();
    let failing = model
        .counts
        .iter()
        .zip(&model.failing)
        .filter(|(_, failing)| **failing)
        .map(|(count, _)| count)
        .sum::<usize>();
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!(
                "{output} histogram: {total} samples, {failing} outside the limit; bin counts {}",
                model
                    .counts
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(rect, 0.0, t.color.bg_panel);

    // Header: the output's name, and what each rule is.
    let header = Rect::from_min_size(rect.min, vec2(width, HEADER_H));
    painter.text(
        pos2(header.left() + TABLE_EDGE_INSET, header.center().y),
        egui::Align2::LEFT_CENTER,
        format!("{output} distribution"),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let legend_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let mut x = header.right() - TABLE_EDGE_INSET;
    for (label, stroke, dashed) in [
        (
            "\u{00b1}3 std dev",
            Stroke::new(1.0, t.color.text_faint),
            true,
        ),
        ("mean", Stroke::new(1.0, t.color.text_dim), false),
        ("limit", Stroke::new(1.0, t.color.text), false),
    ] {
        let galley =
            painter.layout_no_wrap(label.to_owned(), legend_font.clone(), t.color.text_dim);
        x -= galley.size().x;
        let text_left = x;
        painter.galley(
            pos2(text_left, header.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
        let line = [
            pos2(text_left - 20.0, header.center().y),
            pos2(text_left - 6.0, header.center().y),
        ];
        if dashed {
            painter.extend(Shape::dashed_line(&line, stroke, 3.0, 2.0));
        } else {
            painter.line_segment(line, stroke);
        }
        x = text_left - 20.0 - 14.0;
    }

    let plot = Rect::from_min_max(
        pos2(
            rect.left() + TABLE_EDGE_INSET + COUNT_GUTTER_W,
            header.bottom() + 8.0,
        ),
        pos2(rect.right() - TABLE_EDGE_INSET, rect.bottom() - TICK_BAND_H),
    );
    if plot.width() <= 4.0 || plot.height() <= 4.0 {
        return;
    }
    let (axis_low, axis_high) = model.axis;
    let to_x = |value: f64| {
        egui::remap(
            value,
            axis_low..=axis_high,
            plot.left() as f64..=plot.right() as f64,
        ) as f32
    };

    // Count gridlines and their labels.
    let peak = model.counts.iter().copied().max().unwrap_or(1).max(1);
    let count_step = nice_step(peak as f64 / 4.0).max(1.0).round() as usize;
    let count_step = count_step.max(1);
    let count_top = peak.div_ceil(count_step) * count_step;
    let to_y = |count: usize| {
        egui::remap(
            count as f32,
            0.0..=count_top as f32,
            plot.bottom()..=plot.top(),
        )
    };
    let mut level = 0;
    while level <= count_top {
        let y = to_y(level);
        painter.hline(plot.x_range(), y, Stroke::new(1.0, t.color.canvas_grid));
        painter.text(
            pos2(plot.left() - 6.0, y),
            egui::Align2::RIGHT_CENTER,
            level.to_string(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
        level += count_step;
    }

    // Bars.
    for ((edge, count), failing) in model
        .edges
        .windows(2)
        .zip(&model.counts)
        .zip(&model.failing)
    {
        if *count == 0 {
            continue;
        }
        let bar = Rect::from_min_max(
            pos2(to_x(edge[0]) + 0.5, to_y(*count)),
            pos2(to_x(edge[1]) - 0.5, plot.bottom()),
        );
        painter.rect_filled(
            bar,
            0.0,
            if *failing {
                t.color.err.gamma_multiply(0.45)
            } else {
                t.color.text_dim.gamma_multiply(0.38)
            },
        );
    }
    painter.hline(
        plot.x_range(),
        plot.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );

    // Mean, mean ± 3 standard deviations, and the limits.
    let rule = |value: f64, stroke: Stroke, dashed: bool| {
        if !(axis_low..=axis_high).contains(&value) {
            return;
        }
        let x = to_x(value).round() + 0.5;
        let line = [pos2(x, plot.top()), pos2(x, plot.bottom())];
        if dashed {
            painter.extend(Shape::dashed_line(&line, stroke, 4.0, 3.0));
        } else {
            painter.line_segment(line, stroke);
        }
    };
    if model.std_dev > 0.0 {
        for side in [-3.0, 3.0] {
            rule(
                model.mean + side * model.std_dev,
                Stroke::new(1.0, t.color.text_faint),
                true,
            );
        }
    }
    rule(model.mean, Stroke::new(1.0, t.color.text_dim), false);
    for limit in &model.limits {
        rule(*limit, Stroke::new(1.0, t.color.text), false);
    }

    // Axis ticks.
    let tick_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut last_right = f32::NEG_INFINITY;
    for tick in &model.ticks {
        let x = to_x(*tick);
        painter.vline(
            x,
            plot.bottom()..=plot.bottom() + 3.0,
            Stroke::new(1.0, t.color.border_strong),
        );
        let galley = painter.layout_no_wrap(
            tick_text(*tick, model.tick_step, unit),
            tick_font.clone(),
            t.color.text_faint,
        );
        let left = (x - galley.size().x * 0.5).clamp(
            plot.left() - COUNT_GUTTER_W,
            rect.right() - TABLE_EDGE_INSET - galley.size().x,
        );
        if left < last_right + 8.0 {
            continue;
        }
        last_right = left + galley.size().x;
        painter.galley(pos2(left, plot.bottom() + 5.0), galley, t.color.text_faint);
    }
}
