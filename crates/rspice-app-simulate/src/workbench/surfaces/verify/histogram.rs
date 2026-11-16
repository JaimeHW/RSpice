//! The distribution of one Monte Carlo output, drawn as a histogram.
//!
//! It is the Results histogram's plot: the same renderer, axes, engineering
//! ticks with the unit stated once at the axis end, outlined bars, tagged
//! markers and washed fail zone. It is drawn in the page's inks on the page's
//! ground, since it shares one frame with the failing-trial list beside it.
//!
//! What is Verify's own: the trials the verification sheet fails, guard band
//! and all, are stacked at each bar's foot in the error tone; the histogram
//! only counts them. A limit is the strong rule, tagged with its value at its
//! top; the mean is a dot with a drop line, tagged `µ`; mean ± 3 standard
//! deviations are thin dashed rules. The bins are laid so a limit falls on a
//! bin edge.
//!
//! Everything is computed from the exact retained samples. A run that kept no
//! samples has no histogram.

use egui::{Context, Rect, Stroke, Ui, pos2};

use crate::ui::plot::{self, Axis, InteractionMode, Marker, MarkerShape, PlotSpec, XScale};
use crate::ui::tokens::Tokens;

/// The count axis's headroom over the tallest bar, as the Results histogram
/// leaves it, so the tags at the top stand clear of the bars.
const HEADROOM: f64 = 1.18;
/// Room left of the plot for the count ticks, as the Results histogram has.
const COUNT_GUTTER_W: f32 = 48.0;
/// How far below the limits' tags the ±3σ tags sit, so the two rows never
/// collide.
const SIGMA_TAG_DROP: f32 = 20.0;
/// Width of a limit's rule: the strongest line on the plot.
const LIMIT_RULE_W: f32 = 2.0;

/// Which side of a limit fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Side {
    /// A floor: a value under it fails.
    Floor,
    /// A ceiling: a value over it fails.
    Ceiling,
}

/// One bound of a limit, drawn over the bars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Bound {
    pub(super) value: f64,
    pub(super) side: Side,
}

impl Bound {
    /// The bound's tag: `≥ 40 dB` for a floor, `≤ 1.2 mA` for a ceiling.
    pub(super) fn tag(self, unit: &str) -> String {
        let sign = match self.side {
            Side::Floor => '\u{2265}',
            Side::Ceiling => '\u{2264}',
        };
        format!("{sign} {}", super::format_bound(self.value, unit))
    }
}

/// One output's distribution, binned and measured.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct HistogramModel {
    /// Bin edges, one more than there are bins.
    pub(super) edges: Vec<f64>,
    pub(super) counts: Vec<usize>,
    /// Of each bin's samples, how many are failing trials.
    pub(super) failing: Vec<usize>,
    /// The limit's bounds, each inside the axis.
    pub(super) limits: Vec<Bound>,
    pub(super) mean: f64,
    pub(super) std_dev: f64,
    /// The axis: the samples, the limits and mean ± 3 standard deviations,
    /// out to the bin edges.
    pub(super) axis: (f64, f64),
}

impl HistogramModel {
    /// The count axis's top: the tallest bar and its headroom, never under
    /// four, so a count axis is ruled in whole trials.
    pub(super) fn count_top(&self) -> f64 {
        let peak = self.counts.iter().copied().max().unwrap_or(0) as f64;
        (peak * HEADROOM).ceil().max(4.0)
    }

    /// The bin `value` falls in.
    fn bin_at(&self, value: f64) -> Option<usize> {
        self.edges
            .windows(2)
            .position(|edge| edge[0] <= value && value < edge[1])
            .or_else(|| {
                (self.edges.last() == Some(&value)).then_some(self.counts.len().saturating_sub(1))
            })
    }
}

/// A 1-2-5 step near `raw`.
pub(super) fn nice_step(raw: f64) -> f64 {
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

/// Bin `samples`, each a value and whether its trial passed, with `limits`
/// drawn over them.
///
/// Returns `None` when there is no finite sample to bin.
pub(super) fn histogram_model(samples: &[(f64, bool)], limits: &[Bound]) -> Option<HistogramModel> {
    let judged = samples
        .iter()
        .copied()
        .filter(|(sample, _)| sample.is_finite())
        .collect::<Vec<_>>();
    let finite = judged.iter().map(|(sample, _)| *sample).collect::<Vec<_>>();
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
    let limits = limits
        .iter()
        .copied()
        .filter(|limit| limit.value.is_finite())
        .collect::<Vec<_>>();
    let mut axis_low = low.min(mean - 3.0 * std_dev);
    let mut axis_high = high.max(mean + 3.0 * std_dev);
    for limit in &limits {
        axis_low = axis_low.min(limit.value);
        axis_high = axis_high.max(limit.value);
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
    let anchor = limits.first().map_or(0.0, |limit| limit.value);
    let first = anchor - ((anchor - axis_low) / width).ceil() * width;
    let bins = (((axis_high - first) / width).ceil() as usize).max(1);
    let edges = (0..=bins)
        .map(|index| first + index as f64 * width)
        .collect::<Vec<_>>();
    let mut counts = vec![0; bins];
    let mut failing = vec![0; bins];
    for (sample, passing) in &judged {
        let index = (((sample - first) / width).floor().max(0.0) as usize).min(bins - 1);
        counts[index] += 1;
        if !passing {
            failing[index] += 1;
        }
    }

    Some(HistogramModel {
        axis: (edges[0], edges[bins]),
        edges,
        counts,
        failing,
        limits,
        mean,
        std_dev,
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

/// The plot's description of `model`: where its rules stand, what its tags
/// say and how it reads under the pointer. Bars and limit rules are painted
/// by the caller's underlay.
pub(super) fn histogram_spec<'a>(
    unit: &str,
    model: &HistogramModel,
    accessible: (&'a str, &'a str),
    tokens: &Tokens,
) -> PlotSpec<'a> {
    let c = tokens.color;
    let (low, high) = model.axis;
    let mut spec = PlotSpec::new(
        Axis::linear(low, high, unit),
        XScale::Linear,
        Axis::linear_with(0.0, model.count_top(), "trials", 5),
    )
    .accessible_name(accessible.0)
    .accessible_detail(accessible.1);
    spec.left_margin = COUNT_GUTTER_W;
    if model.std_dev > 0.0 {
        for (side, tag) in [(-3.0, "\u{2212}3\u{03c3}"), (3.0, "+3\u{03c3}")] {
            spec.markers.push(Marker {
                label_dy: SIGMA_TAG_DROP,
                ..Marker::limit_line(model.mean + side * model.std_dev, c.text_dim, tag)
            });
        }
    }
    spec.markers.push(Marker {
        x: model.mean,
        y: model.count_top() * 0.86,
        color: c.accent,
        label: format!("\u{00b5} {}", super::format_value(model.mean, unit)),
        drop_line: true,
        label_dy: 0.0,
        shape: MarkerShape::Point,
    });
    for bound in &model.limits {
        spec.markers
            .push(Marker::limit_line(bound.value, c.err, bound.tag(unit)));
    }
    spec
}

/// Paint `model` across the rest of `ui`, as the Results histogram paints a
/// distribution, with the failing trials stacked at each bar's foot.
pub(super) fn paint_histogram(ui: &mut Ui, output: &str, unit: &str, model: &HistogramModel) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    #[cfg(any(test, feature = "test-support"))]
    super::raster::probe(ui, "histogram", ui.available_rect_before_wrap());
    let total = model.counts.iter().sum::<usize>();
    let failing = model.failing.iter().sum::<usize>();
    let name = format!("{output} histogram");
    let detail = format!(
        "{total} trials, {failing} failing; bin counts {}",
        model
            .counts
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut spec = histogram_spec(unit, model, (&name, &detail), &t);
    let (low, high) = model.axis;
    // The failing part of a bar takes the passing part's wash in the error
    // tone, and its outline in the error ink.
    let fail_fill = c
        .err
        .gamma_multiply(f32::from(c.accent_dim.a()) / f32::from(u8::MAX));
    spec.underlay = Some(Box::new(move |painter, mapper| {
        // The fail zones washed, as the Results histogram washes them.
        let wash = c.err.gamma_multiply(0.09);
        for bound in &model.limits {
            let x = mapper.x(bound.value.clamp(low, high));
            let zone = match bound.side {
                Side::Floor => mapper.rect.with_max_x(x),
                Side::Ceiling => mapper.rect.with_min_x(x),
            };
            painter.rect_filled(zone, 0.0, wash);
        }
        let bar = |from: usize, to: usize, edge: &[f64]| {
            let left = mapper.x(edge[0]) + 1.0;
            let right = (mapper.x(edge[1]) - 1.0).max(left + 1.0);
            Rect::from_min_max(
                pos2(left, mapper.y(to as f64)),
                pos2(right, mapper.y(from as f64)),
            )
        };
        for ((edge, &count), &failing) in model
            .edges
            .windows(2)
            .zip(&model.counts)
            .zip(&model.failing)
        {
            if count > failing {
                painter.rect(
                    bar(failing, count, edge),
                    0.0,
                    c.accent_dim,
                    Stroke::new(1.0, c.accent),
                    egui::StrokeKind::Inside,
                );
            }
            if failing > 0 {
                painter.rect(
                    bar(0, failing, edge),
                    0.0,
                    fail_fill,
                    Stroke::new(1.0, c.err),
                    egui::StrokeKind::Inside,
                );
            }
        }
        for bound in &model.limits {
            let x = mapper.x(bound.value).round();
            painter.vline(x, mapper.rect.y_range(), Stroke::new(LIMIT_RULE_W, c.err));
        }
    }));
    let readout = |value: f64| {
        let mut rows = vec![
            (output.to_owned(), super::format_value(value, unit)),
            (
                "Trials".to_owned(),
                model
                    .bin_at(value)
                    .map_or(0, |bin| model.counts[bin])
                    .to_string(),
            ),
        ];
        if let Some(failing) = model
            .bin_at(value)
            .map(|bin| model.failing[bin])
            .filter(|failing| *failing > 0)
        {
            rows.push(("Failing".to_owned(), failing.to_string()));
        }
        rows
    };
    // The bins are the sheet's own, so the reader reads them rather than
    // pans or zooms them.
    plot::set_interaction_mode(ui.ctx(), InteractionMode::Select);
    let _inks = PageInks::wear(ui.ctx());
    let _ = plot::show(
        ui,
        &spec,
        &mut plot::DecimationCache::default(),
        None,
        Some(&readout),
    );
}

/// The page's inks in place of the canvas theme's while a plot that sits on
/// the page draws, and the canvas theme's again once it has: the plot's
/// ground is the page's, so its ticks, grid and frame take the page's tones
/// in either mode.
struct PageInks {
    ctx: Context,
    worn: Tokens,
}

impl PageInks {
    fn wear(ctx: &Context) -> Self {
        let worn = (*Tokens::get(ctx)).clone();
        let mut page = worn.clone();
        let c = &mut page.color;
        c.canvas_bg = c.bg_app;
        c.canvas_grid = c.border;
        c.canvas_frame = c.border_strong;
        c.canvas_text = c.text;
        c.canvas_text_dim = c.text_dim;
        c.canvas_text_faint = c.text_faint;
        c.canvas_cursor_a = c.accent;
        c.canvas_cursor_b = c.text_dim;
        page.install(ctx);
        Self {
            ctx: ctx.clone(),
            worn,
        }
    }
}

impl Drop for PageInks {
    fn drop(&mut self) {
        std::mem::take(&mut self.worn).install(&self.ctx);
    }
}
