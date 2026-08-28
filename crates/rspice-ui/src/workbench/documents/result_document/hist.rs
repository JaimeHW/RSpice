//! HIST — Monte-Carlo distribution: accent-tinted bins, a normal-fit
//! overlay, the ±1σ band, and µ / spec-limit markers; distribution stats and
//! yield in the right panel.

use egui::Ui;

use crate::services::yield_manager::{SpecLimitType, YieldResult};
use crate::state::{AnalysisResultFamilyMetadata, MonteCarloVariableMetadata, SimulationState};
use crate::ui::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::frame_work::{self, DatasetWalk};
use super::strip::{self, LegendChip};
use super::well_hint;

/// Return the one yield result authorized by both the active immutable
/// dataset and the selected Monte-Carlo measurement. Yield results are stored
/// as a flat collection, so selecting the first entry can silently apply the
/// limits and verdict for another variable. Duplicate target evidence is also
/// rejected: the UI must not choose an arbitrary result when retained state is
/// malformed or came from an older, less constrained schema.
///
/// This half is cheap — name matching over the retained result list. Whether
/// the result is internally consistent walks the whole retained population,
/// and that verdict comes from the plan instead.
fn matching_yield_result<'a>(
    simulation: &'a SimulationState,
    histogram_name: &str,
) -> Option<&'a YieldResult> {
    let results = simulation.yield_results_for_active_dataset()?;
    let mut matches = results
        .iter()
        .filter(|result| yield_target_measurement(&result.spec.target) == histogram_name);
    let result = matches.next()?;
    matches.next().is_none().then_some(result)
}

fn selected_yield_result<'a>(
    simulation: &'a SimulationState,
    histogram_name: &str,
    consistent: bool,
) -> Option<&'a YieldResult> {
    consistent
        .then(|| matching_yield_result(simulation, histogram_name))
        .flatten()
}

/// `mean(name)` is an accepted Monte-Carlo yield target spelling and refers
/// to the complete retained sample population named `name`. Keep this
/// canonicalization byte-for-byte compatible with the yield engine so display
/// authority never broadens beyond evidence the engine actually evaluated.
fn yield_target_measurement(target: &str) -> &str {
    let target = target.trim();
    const PREFIX: &str = "mean";
    if target.len() > PREFIX.len() + 2
        && target
            .get(..PREFIX.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(PREFIX))
        && target
            .get(PREFIX.len()..)
            .is_some_and(|tail| tail.starts_with('('))
        && target.ends_with(')')
    {
        &target[PREFIX.len() + 1..target.len() - 1]
    } else {
        target
    }
}

#[derive(Debug, Clone, Copy)]
struct ExactMoments {
    count: usize,
    mean: f64,
    std_dev: f64,
    min: f64,
    max: f64,
}

fn nearly_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 128.0 * f64::EPSILON * scale
}

fn moments_from_samples(samples: &[f64]) -> Option<ExactMoments> {
    frame_work::note(DatasetWalk::HistMoments);
    if samples.is_empty() || samples.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let count = samples.len();
    let mean = samples.iter().sum::<f64>() / count as f64;
    if !mean.is_finite() {
        return None;
    }
    let variance = samples
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (count.saturating_sub(1).max(1) as f64);
    let std_dev = variance.sqrt();
    let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (std_dev.is_finite() && min.is_finite() && max.is_finite()).then_some(ExactMoments {
        count,
        mean,
        std_dev,
        min,
        max,
    })
}

fn moments_match(
    moments: ExactMoments,
    count: usize,
    mean: f64,
    std_dev: f64,
    min: f64,
    max: f64,
) -> bool {
    moments.count == count
        && nearly_equal(moments.mean, mean)
        && nearly_equal(moments.std_dev, std_dev)
        && nearly_equal(moments.min, min)
        && nearly_equal(moments.max, max)
}

fn yield_result_is_consistent(result: &YieldResult) -> bool {
    frame_work::note(DatasetWalk::HistMoments);
    let spec = &result.spec;
    let limits_valid = match spec.limit_type {
        SpecLimitType::Lower => spec.min.is_some_and(f64::is_finite) && spec.max.is_none(),
        SpecLimitType::Upper => spec.min.is_none() && spec.max.is_some_and(f64::is_finite),
        SpecLimitType::Range => matches!(
            (spec.min, spec.max),
            (Some(min), Some(max)) if min.is_finite() && max.is_finite() && min < max
        ),
    };
    if spec.target.trim().is_empty()
        || !limits_valid
        || spec.target_val.is_some_and(|value| !value.is_finite())
        || !spec.weight.is_finite()
        || !(0.0..=1.0).contains(&spec.weight)
        || result.total_runs == 0
        || result.pass_count.saturating_add(result.fail_count) != result.total_runs
        || result.trail.len() != result.total_runs
        || result.trail.iter().filter(|passed| **passed).count() != result.pass_count
        || result.samples.len() != result.total_runs
        || result
            .samples
            .iter()
            .filter(|value| spec.evaluates(**value))
            .count()
            != result.pass_count
        || !result.yield_percent.is_finite()
        || !(0.0..=100.0).contains(&result.yield_percent)
        || !result.stats.median.is_finite()
        || !result.stats.skewness.is_finite()
        || !result.stats.kurtosis.is_finite()
        || result.stats.cp.is_some_and(|value| !value.is_finite())
        || result.stats.cpk.is_some_and(|value| !value.is_finite())
    {
        return false;
    }

    let expected_yield = result.pass_count as f64 * 100.0 / result.total_runs as f64;
    let Some(moments) = moments_from_samples(&result.samples) else {
        return false;
    };
    nearly_equal(result.yield_percent, expected_yield)
        && moments_match(
            moments,
            result.stats.count,
            result.stats.mean,
            result.stats.std_dev,
            result.stats.min,
            result.stats.max,
        )
}

#[derive(Debug, Clone, Copy)]
struct MonteCarloAuthority<'a> {
    seed: u64,
    runs_requested: usize,
    runs_completed: usize,
    failures: usize,
    variable: &'a MonteCarloVariableMetadata,
}

fn active_monte_carlo_authority<'a>(
    state: &'a AppState,
    histogram_name: &str,
) -> Option<MonteCarloAuthority<'a>> {
    let analysis = state.simulation.active_analysis()?;
    if !analysis.success {
        return None;
    }
    let metadata = analysis.family_metadata.as_ref()?;
    if metadata.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    let AnalysisResultFamilyMetadata::MonteCarlo {
        seed,
        runs_requested,
        runs_completed,
        failures,
        variables,
        ..
    } = metadata
    else {
        return None;
    };
    let mut matches = variables
        .iter()
        .filter(|variable| variable.name == histogram_name);
    let variable = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(MonteCarloAuthority {
        seed: *seed,
        runs_requested: *runs_requested,
        runs_completed: *runs_completed,
        failures: *failures,
        variable,
    })
}

fn exact_moments(state: &AppState, histogram_name: &str) -> Option<ExactMoments> {
    if state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.family_metadata.as_ref())
        .is_some()
    {
        let authority = active_monte_carlo_authority(state, histogram_name)?;
        let variable = authority.variable;
        return moments_from_samples(&variable.samples).filter(|moments| {
            moments_match(
                *moments,
                variable.samples.len(),
                variable.mean,
                variable.std_dev,
                variable.min,
                variable.max,
            )
        });
    }

    let result = matching_yield_result(&state.simulation, histogram_name)
        .filter(|result| yield_result_is_consistent(result))?;
    moments_from_samples(&result.samples)
}

/// The descriptive moments and yield verdict behind one distribution.
///
/// Both walk the retained Monte-Carlo population several times over — the
/// moments are four passes and the yield verdict counts, filters and then
/// recomputes the moments to check the retained summary against them. The
/// sheet and its panel each asked for both on every frame.
#[derive(Debug, Clone)]
pub(super) struct HistPlan {
    version: u64,
    histogram: String,
    moments: Option<ExactMoments>,
    yield_is_consistent: bool,
}

/// Resolve the distribution's statistics once per (generation, histogram).
fn hist_plan(state: &mut AppState, histogram: &str) -> std::sync::Arc<HistPlan> {
    let version = state.simulation.data_version;
    if let Some(plan) = state.ui.results.plans.hist.as_ref()
        && plan.version == version
        && plan.histogram == histogram
    {
        return std::sync::Arc::clone(plan);
    }
    let built = std::sync::Arc::new(HistPlan {
        version,
        histogram: histogram.to_owned(),
        moments: exact_moments(state, histogram),
        yield_is_consistent: matching_yield_result(&state.simulation, histogram)
            .is_some_and(yield_result_is_consistent),
    });
    state.ui.results.plans.hist = Some(std::sync::Arc::clone(&built));
    built
}

/// How one distribution is laid out on its abscissa.
#[derive(Debug, Clone, Copy, PartialEq)]
struct HistAxis {
    /// The window the axis is ruled over.
    x0: f64,
    x1: f64,
    /// The one value every retained sample landed on, when the distribution
    /// has no width at all. The sheet draws a bar for it rather than the
    /// zero-width rectangle its bin edges describe.
    degenerate_at: Option<f64>,
}

/// A distribution's abscissa window, and whether it is a single point.
///
/// A Monte Carlo whose measurement did not move retains exactly one bin whose
/// two edges are the same number — that is what the engine's histogram builder
/// emits when the sample range is zero. Padding a zero span by six percent
/// leaves a window narrower than the value's own floating-point resolution:
/// every axis label prints the same number, and the one populated bin is a
/// one-pixel line in the middle of an empty frame. A degenerate distribution
/// is ruled around its value instead, so the reader sees a bar standing at a
/// number rather than an empty plot.
fn hist_axis(histogram: &crate::analysis::histogram::data::Histogram) -> HistAxis {
    let span = histogram.data_max - histogram.data_min;
    if span.is_finite() && span > 0.0 {
        let pad = span * 0.06;
        return HistAxis {
            x0: histogram.data_min - pad,
            x1: histogram.data_max + pad,
            degenerate_at: None,
        };
    }
    let value = histogram.data_min;
    if !value.is_finite() {
        return HistAxis {
            x0: -1.0,
            x1: 1.0,
            degenerate_at: None,
        };
    }
    // A window a float can actually distinguish: one part in a thousand of
    // the value, and a unit window when the value is zero.
    let pad = if value == 0.0 {
        1.0
    } else {
        value.abs() * 1.0e-3
    };
    HistAxis {
        x0: value - pad,
        x1: value + pad,
        degenerate_at: Some(value),
    }
}

/// The share of the frame the single bar of a degenerate distribution covers.
const DEGENERATE_BAR_FRACTION: f32 = 0.18;

/// The yield figure, with the population it was measured over.
///
/// A percentage alone reads as a property of the run, and it is not: the
/// denominator is the trials the yield engine had evidence for, and a Monte
/// Carlo that requested a hundred and completed ninety reports a yield over
/// ninety. Both halves are stated here rather than left for the reader to
/// reconstruct from the "Failures" row and the method panel — the count that
/// makes the percentage mean something belongs beside it.
fn yield_label(result: &YieldResult, authority: Option<MonteCarloAuthority<'_>>) -> String {
    let mut label = format!(
        "{:.1} % · {} of {} evaluated",
        result.yield_percent, result.pass_count, result.total_runs
    );
    if let Some(authority) = authority
        && authority.failures > 0
    {
        label.push_str(&format!(" · {} diverged excluded", authority.failures));
    }
    label
}

/// What the method panel says about the retained binning.
///
/// A collapsed distribution states that it is one: "1 retained bins" reads as
/// a count that happens to be small, when what the reader needs to know is
/// that the bin has no width because the measurement never moved.
fn binning_label(histogram: &crate::analysis::histogram::data::Histogram) -> String {
    if hist_axis(histogram).degenerate_at.is_some() {
        return "1 retained bin · zero width, every sample at one value".to_owned();
    }
    let count = histogram.bins.len();
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} retained bin{plural} · selection rule unavailable")
}

/// The distribution the reader has selected, by name.
fn selected_histogram_name(state: &AppState) -> Option<String> {
    let hist_state = &state.analysis.histogram_state;
    hist_state
        .histograms
        .get(hist_state.selected)
        .map(|histogram| histogram.name.clone())
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the histogram.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(name) = selected_histogram_name(state) else {
        well_hint(ui, "No distribution yet — run a Monte Carlo analysis");
        return;
    };
    // Resolved before the distribution is borrowed, so the population is
    // walked once per dataset generation rather than once per frame.
    let plan = hist_plan(state, &name);
    let hist_state = &state.analysis.histogram_state;
    let Some(histogram) = hist_state.histograms.get(hist_state.selected) else {
        well_hint(ui, "No distribution yet — run a Monte Carlo analysis");
        return;
    };
    if histogram.bins.is_empty() || histogram.total_count == 0 {
        well_hint(ui, "The selected distribution is empty");
        return;
    }
    let moments = plan.moments;
    let spec_limits =
        selected_yield_result(&state.simulation, &histogram.name, plan.yield_is_consistent)
            .map(|result| (result.spec.min, result.spec.max));

    let subtitle = format!("{} · {} samples", histogram.name, histogram.total_count);
    let mut legend = vec![LegendChip {
        name: "count",
        color: c.accent,
        on: true,
    }];
    if moments.is_some() {
        legend.push(LegendChip {
            name: "descriptive ±1σ",
            color: c.text_dim,
            on: true,
        });
    }
    if spec_limits.is_some() {
        legend.push(LegendChip {
            name: "retained spec limit",
            color: c.err,
            on: true,
        });
    }
    let view = state.ui.results.plot_view(super::ResultViewer::Hist, 0);
    let header = strip::StripHeader::new("MC", &subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Hist, 0);
    }

    let axis = hist_axis(histogram);
    let (x0, x1) = view.x.unwrap_or((axis.x0, axis.x1));
    let max_count = histogram.bins.iter().map(|b| b.count).max().unwrap_or(1) as f64;
    let y1 = (max_count * 1.18).ceil().max(4.0);
    let (y0, y1) = view.y.unwrap_or((0.0, y1));

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, ""),
        XScale::Linear,
        Axis::linear_with(y0, y1, "n", 5),
    )
    .accessible_name("Statistical histogram");
    spec.left_margin = 48.0;

    // Descriptive ±1σ band and mean marker from exact retained sample
    // moments. This is deliberately not called a distribution fit: no fit
    // family or goodness-of-fit evidence is retained by the result schema.
    if let Some(moments) = moments {
        if moments.std_dev > 0.0 {
            spec.bands.push(plot::Band {
                x0: moments.mean - moments.std_dev,
                x1: moments.mean + moments.std_dev,
            });
        }
        spec.markers.push(plot::Marker {
            x: moments.mean,
            y: y1 * 0.86,
            color: c.accent,
            label: format!("µ {}", fmt_si(moments.mean, "", 2)),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }

    // Spec limits from the yield manager, when present.
    if let Some((lsl, usl)) = spec_limits {
        if let Some(lsl) = lsl
            && lsl > x0
            && lsl < x1
        {
            spec.markers.push(plot::Marker {
                x: lsl,
                y: y1 * 0.72,
                color: c.err,
                label: format!("LSL {}", fmt_si(lsl, "", 2)),
                drop_line: true,
                label_dy: 0.0,
                shape: plot::MarkerShape::Point,
            });
        }
        if let Some(usl) = usl
            && usl > x0
            && usl < x1
        {
            spec.markers.push(plot::Marker {
                x: usl,
                y: y1 * 0.72,
                color: c.err,
                label: format!("USL {}", fmt_si(usl, "", 2)),
                drop_line: true,
                label_dy: 0.0,
                shape: plot::MarkerShape::Point,
            });
        }
    }

    // A distribution with no width has nothing for the ±1σ band or the bin
    // rectangles to say, so it names the value it collapsed onto instead.
    if let Some(value) = axis.degenerate_at {
        spec.markers.push(plot::Marker {
            x: value,
            y: y1 * 0.55,
            color: c.accent,
            label: format!(
                "all {} samples at {}",
                histogram.total_count,
                fmt_si(value, "", 3)
            ),
            drop_line: false,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }

    // Bars under everything else, with the out-of-spec regions washed in
    // the error tint — the fail zone itself, not the data envelope.
    let bins = &histogram.bins;
    let degenerate_at = axis.degenerate_at;
    let total_count = histogram.total_count;
    let accent = c.accent;
    let accent_dim = c.accent_dim;
    let err = c.err;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        if let Some((lsl, usl)) = spec_limits {
            let wash = err.gamma_multiply(0.09);
            if let Some(lsl) = lsl
                && lsl > x0
            {
                let rect = egui::Rect::from_min_max(
                    egui::pos2(mapper.rect.left(), mapper.rect.top()),
                    egui::pos2(mapper.x(lsl.min(x1)), mapper.rect.bottom()),
                );
                painter.rect_filled(rect, 0.0, wash);
            }
            if let Some(usl) = usl
                && usl < x1
            {
                let rect = egui::Rect::from_min_max(
                    egui::pos2(mapper.x(usl.max(x0)), mapper.rect.top()),
                    egui::pos2(mapper.rect.right(), mapper.rect.bottom()),
                );
                painter.rect_filled(rect, 0.0, wash);
            }
        }
        // One bar for a distribution whose bin edges coincide: the retained
        // rectangle has no width, so the frame supplies one.
        if let Some(value) = degenerate_at {
            if total_count > 0 {
                let half = mapper.rect.width() * DEGENERATE_BAR_FRACTION * 0.5;
                let centre = mapper.x(value);
                let rect = egui::Rect::from_min_max(
                    egui::pos2(centre - half, mapper.y(total_count as f64)),
                    egui::pos2(centre + half, mapper.y(0.0)),
                );
                painter.rect(
                    rect,
                    0.0,
                    accent_dim,
                    egui::Stroke::new(1.0, accent),
                    egui::StrokeKind::Inside,
                );
            }
            return;
        }
        for bin in bins {
            if bin.count == 0 {
                continue;
            }
            let left = mapper.x(bin.lower) + 1.0;
            let right = (mapper.x(bin.upper) - 1.0).max(left + 1.0);
            let top = mapper.y(bin.count as f64);
            let bottom = mapper.y(0.0);
            let rect = egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, bottom));
            painter.rect(
                rect,
                0.0,
                accent_dim,
                egui::Stroke::new(1.0, accent),
                egui::StrokeKind::Inside,
            );
        }
    }));

    let readout = |x: f64| -> Vec<(String, String)> {
        // A zero-width bin can never contain the pointer, so a degenerate
        // distribution reads out the population it collapsed onto instead of
        // reporting an empty frame.
        let count = degenerate_at.map_or_else(
            || {
                bins.iter()
                    .find(|b| x >= b.lower && x < b.upper)
                    .map_or(0, |b| b.count)
            },
            |_| total_count,
        );
        vec![
            ("x".to_owned(), fmt_si(x, "", 2)),
            ("count".to_owned(), count.to_string()),
        ]
    };

    let response = plot::show(ui, &spec, &mut state.ui.results.cache, None, Some(&readout));
    super::record_drawn_axes(&mut state.ui.results, super::ResultViewer::Hist, &response);
    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Hist, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Distribution stats + spec/yield verdict.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(name) = selected_histogram_name(state) else {
        section_header(ui, "Distribution", None);
        super::panel_note(ui, "Stats appear once a Monte Carlo run is loaded.");
        return;
    };
    let plan = hist_plan(state, &name);
    let hist_state = &state.analysis.histogram_state;
    let Some(histogram) = hist_state.histograms.get(hist_state.selected) else {
        section_header(ui, "Distribution", None);
        super::panel_note(ui, "Stats appear once a Monte Carlo run is loaded.");
        return;
    };

    section_header(ui, "Distribution", None);
    if let Some(moments) = plan.moments {
        let rows = [
            ("Measure", histogram.name.clone(), false),
            ("Exact samples", moments.count.to_string(), false),
            ("Mean", fmt_si(moments.mean, "", 3), true),
            ("Std dev", fmt_si(moments.std_dev, "", 3), true),
            ("Min", fmt_si(moments.min, "", 3), false),
            ("Max", fmt_si(moments.max, "", 3), false),
        ];
        super::stat_table(ui, &rows);
    } else {
        super::stat_table(
            ui,
            &[
                ("Measure", histogram.name.clone(), false),
                (
                    "Exact moments",
                    "Unavailable — source samples not retained".to_owned(),
                    false,
                ),
            ],
        );
    }

    section_header(ui, "Method authority", None);
    let mc = active_monte_carlo_authority(state, &histogram.name);
    let seed = mc.map_or_else(
        || "Unavailable — not retained".to_owned(),
        |authority| format!("{} · 0x{:X}", authority.seed, authority.seed),
    );
    let completion = mc.map_or_else(
        || "Unavailable — not retained".to_owned(),
        |authority| {
            format!(
                "{} / {} · {} failed",
                authority.runs_completed, authority.runs_requested, authority.failures
            )
        },
    );
    let binning = binning_label(histogram);
    super::stat_table(
        ui,
        &[
            ("Run completion", completion, false),
            ("Seed", seed, false),
            (
                "Estimator",
                "Unavailable — method not retained".to_owned(),
                false,
            ),
            (
                "Distribution fit",
                "Unavailable — no fit evidence retained".to_owned(),
                false,
            ),
            ("Binning", binning, false),
        ],
    );

    if let Some(yield_result) =
        selected_yield_result(&state.simulation, &histogram.name, plan.yield_is_consistent)
    {
        section_header(ui, "Spec", None);
        let cpk = yield_result
            .stats
            .cpk
            .map_or("—".to_owned(), |v| format!("{v:.2}"));
        let rows = [
            ("Yield", yield_label(yield_result, mc), true),
            ("Cpk", cpk, false),
            (
                "Failures",
                format!("{} / {}", yield_result.fail_count, yield_result.total_runs),
                false,
            ),
            (
                "Confidence interval",
                "Unavailable — not retained".to_owned(),
                false,
            ),
        ];
        super::stat_table(ui, &rows);
    } else {
        section_header(ui, "Spec", None);
        super::panel_note(
            ui,
            "No unambiguous specification or yield evidence is retained for this measurement.",
        );
    }
    super::panel_note(
        ui,
        "The shaded band is descriptive ±1σ only when exact retained moments are available. No distribution fit is inferred.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::yield_manager::{
        DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldSpec,
    };
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata,
        SimulationRun,
    };

    fn result(target: &str, yield_percent: f64) -> YieldResult {
        let total_runs = 100;
        let pass_count = yield_percent.round() as usize;
        let mut samples = vec![1.0; pass_count];
        samples.resize(total_runs, 0.0);
        let moments = moments_from_samples(&samples).expect("finite test samples");
        YieldResult {
            spec: YieldSpec::lower(target, 0.9, "V"),
            total_runs,
            pass_count,
            fail_count: total_runs - pass_count,
            yield_percent,
            stats: DistributionStats {
                count: moments.count,
                mean: moments.mean,
                std_dev: moments.std_dev,
                min: moments.min,
                max: moments.max,
                median: 0.0,
                skewness: 0.0,
                kurtosis: 0.0,
                cp: None,
                cpk: None,
            },
            trail: (0..total_runs).map(|index| index < pass_count).collect(),
            samples,
        }
    }

    fn provenance(run: &SimulationRun) -> YieldAnalysisProvenance {
        YieldAnalysisProvenance {
            source_run_id: run.run_id,
            source_dataset_id: run.dataset_id,
            seed: 7,
            runs_requested: 100,
            runs_completed: 100,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }
    }

    #[test]
    fn selected_measurement_never_uses_first_result_for_another_measurement() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.runs.push(run.clone());
        simulation.active_run_idx = Some(0);
        simulation.replace_yield_evidence(
            vec![result("gain", 12.0), result("V(out)", 97.0)],
            Some(provenance(&run)),
        );

        let selected = selected_yield_result(&simulation, "V(out)", true)
            .expect("the exact selected measurement is authoritative");
        assert_eq!(selected.spec.target, "V(out)");
        assert_eq!(selected.yield_percent, 97.0);
    }

    #[test]
    fn stale_dataset_yield_evidence_is_rejected() {
        let stale_run = SimulationRun::new(1);
        let active_run = SimulationRun::new(2);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![stale_run.clone(), active_run];
        simulation.active_run_idx = Some(1);
        simulation
            .replace_yield_evidence(vec![result("V(out)", 23.0)], Some(provenance(&stale_run)));

        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());
    }

    #[test]
    fn absent_or_ambiguous_measurement_evidence_fails_closed() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.runs.push(run.clone());
        simulation.active_run_idx = Some(0);
        simulation.replace_yield_evidence(vec![result("gain", 90.0)], Some(provenance(&run)));
        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());

        simulation.replace_yield_evidence(
            vec![result("V(out)", 90.0), result("mean(V(out))", 10.0)],
            Some(provenance(&run)),
        );
        assert!(selected_yield_result(&simulation, "V(out)", true).is_none());
    }

    #[test]
    fn mean_wrapper_resolves_only_its_exact_sample_population() {
        let run = SimulationRun::new(1);
        let mut simulation = SimulationState::default();
        simulation.runs.push(run.clone());
        simulation.active_run_idx = Some(0);
        simulation
            .replace_yield_evidence(vec![result("mean(V(out))", 88.0)], Some(provenance(&run)));

        assert_eq!(
            selected_yield_result(&simulation, "V(out)", true).map(|result| result.yield_percent),
            Some(88.0)
        );
        assert!(selected_yield_result(&simulation, "v(out)", true).is_none());
    }

    /// The histogram a zero-variation Monte Carlo produces.
    ///
    /// `VariableStatistics::compute_histogram` returns `(vec![n], vec![v, v])`
    /// whenever the sample range is not positive, and
    /// `populate_monte_carlo_histograms` maps that to one bin whose two edges
    /// are the same number. This is that shape, byte for byte.
    fn zero_variation_histogram(
        value: f64,
        samples: usize,
    ) -> crate::analysis::histogram::data::Histogram {
        crate::analysis::histogram::data::Histogram {
            name: "V(out)".to_owned(),
            bins: vec![crate::analysis::histogram::data::HistogramBin {
                lower: value,
                upper: value,
                count: samples,
                weight: samples as f64,
            }],
            total_count: samples,
            total_weight: samples as f64,
            underflow: 0,
            overflow: 0,
            data_min: value,
            data_max: value,
        }
    }

    /// The plan's gate: a Monte Carlo whose measurement never moved must
    /// still draw as a distribution.
    #[test]
    fn a_zero_variation_monte_carlo_is_ruled_around_the_value_it_collapsed_onto() {
        let histogram = zero_variation_histogram(1.5, 40);
        let axis = hist_axis(&histogram);

        assert_eq!(
            axis.degenerate_at,
            Some(1.5),
            "the sheet did not recognize a single zero-width bin"
        );
        assert!(
            axis.x0 < 1.5 && 1.5 < axis.x1,
            "the value is not inside its own window: {axis:?}"
        );
        // Wide enough that the axis labels differ from one another, which the
        // 6 % padding of a zero span never achieves.
        assert!(
            axis.x1 - axis.x0 >= 1.5 * 1.0e-3,
            "the window is narrower than the value's own resolution: {axis:?}"
        );
        assert_ne!(
            fmt_si(axis.x0, "", 3),
            fmt_si(axis.x1, "", 3),
            "both ends of the axis print the same number"
        );
    }

    /// The percentage states the population it was measured over.
    ///
    /// "97.0 %" alone reads as a property of the run. It is a property of the
    /// trials the yield engine had evidence for, which is not the number
    /// requested when trials diverged, and the reader had to reconstruct the
    /// difference from two other rows and a second panel.
    #[test]
    fn the_yield_figure_states_the_population_it_was_measured_over() {
        let result = result("V(out)", 97.0);
        assert_eq!(
            yield_label(&result, None),
            "97.0 % · 97 of 100 evaluated",
            "the yield percentage stands on its own with no denominator"
        );

        let variable = mc_variable("V(out)");
        let authority = MonteCarloAuthority {
            seed: 7,
            runs_requested: 110,
            runs_completed: 100,
            failures: 10,
            variable: &variable,
        };
        assert_eq!(
            yield_label(&result, Some(authority)),
            "97.0 % · 97 of 100 evaluated · 10 diverged excluded"
        );
    }

    #[test]
    fn a_degenerate_distribution_says_its_bin_has_no_width() {
        assert_eq!(
            binning_label(&zero_variation_histogram(1.5, 40)),
            "1 retained bin · zero width, every sample at one value"
        );
    }

    #[test]
    fn an_ordinary_distribution_keeps_its_padded_data_window() {
        let mut histogram = zero_variation_histogram(0.0, 0);
        histogram.bins = vec![
            crate::analysis::histogram::data::HistogramBin {
                lower: 1.0,
                upper: 2.0,
                count: 3,
                weight: 3.0,
            },
            crate::analysis::histogram::data::HistogramBin {
                lower: 2.0,
                upper: 3.0,
                count: 1,
                weight: 1.0,
            },
        ];
        histogram.total_count = 4;
        histogram.data_min = 1.0;
        histogram.data_max = 3.0;

        let axis = hist_axis(&histogram);
        assert_eq!(axis.degenerate_at, None);
        assert!((axis.x0 - 0.88).abs() < 1.0e-12, "{axis:?}");
        assert!((axis.x1 - 3.12).abs() < 1.0e-12, "{axis:?}");
        assert_eq!(
            binning_label(&histogram),
            "2 retained bins · selection rule unavailable"
        );
    }

    fn mc_variable(name: &str) -> MonteCarloVariableMetadata {
        MonteCarloVariableMetadata {
            name: name.to_owned(),
            samples: vec![1.0, 2.0, 3.0],
            mean: 2.0,
            std_dev: 1.0,
            min: 1.0,
            max: 3.0,
        }
    }

    #[test]
    fn monte_carlo_method_authority_is_bound_to_the_selected_measurement() {
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 0x73a4,
                runs_requested: 4,
                runs_completed: 3,
                failures: 1,
                all_converged: false,
                variables: vec![mc_variable("gain"), mc_variable("offset")],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        let authority = active_monte_carlo_authority(&state, "offset")
            .expect("exact selected variable authority");
        assert_eq!(authority.seed, 0x73a4);
        assert_eq!(authority.runs_completed, 3);
        assert_eq!(authority.failures, 1);
        assert_eq!(exact_moments(&state, "offset").unwrap().mean, 2.0);
        assert!(active_monte_carlo_authority(&state, "missing").is_none());
    }

    #[test]
    fn duplicate_monte_carlo_variable_authority_fails_closed() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC");
        analysis.family_metadata = Some(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 9,
            runs_requested: 3,
            runs_completed: 3,
            failures: 0,
            all_converged: true,
            variables: vec![mc_variable("gain"), mc_variable("gain")],
        });
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert!(active_monte_carlo_authority(&state, "gain").is_none());
    }

    #[test]
    fn exact_moments_fail_closed_when_retained_summary_disagrees_with_samples() {
        let mut variable = mc_variable("gain");
        variable.mean = 200.0;
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 9,
                runs_requested: 3,
                runs_completed: 3,
                failures: 0,
                all_converged: true,
                variables: vec![variable],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert!(exact_moments(&state, "gain").is_none());
    }

    #[test]
    fn inconsistent_yield_counts_and_statistics_are_rejected() {
        let valid = result("V(out)", 90.0);
        assert!(yield_result_is_consistent(&valid));

        let mut bad_counts = valid.clone();
        bad_counts.fail_count += 1;
        assert!(!yield_result_is_consistent(&bad_counts));

        let mut bad_stats = valid;
        bad_stats.stats.mean += 0.25;
        assert!(!yield_result_is_consistent(&bad_stats));
    }

    /// The population is walked once per (generation, distribution), and the
    /// answer must move when either does.
    #[test]
    fn the_distribution_statistics_are_resolved_once_per_generation() {
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 3,
                runs_requested: 3,
                runs_completed: 3,
                failures: 0,
                all_converged: true,
                variables: vec![mc_variable("gain"), mc_variable("offset")],
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        let first = hist_plan(&mut state, "gain");
        assert!(std::sync::Arc::ptr_eq(
            &first,
            &hist_plan(&mut state, "gain")
        ));
        assert_eq!(first.moments.expect("retained moments").mean, 2.0);
        // The memo is the projection it replaced.
        assert_eq!(
            first.moments.expect("retained moments").std_dev,
            exact_moments(&state, "gain")
                .expect("retained moments")
                .std_dev
        );

        // Another distribution is another answer, at the same generation.
        let other = hist_plan(&mut state, "offset");
        assert!(!std::sync::Arc::ptr_eq(&first, &other));

        // A new generation of the population is a new answer.
        let AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } = state.simulation.runs[0]
            .analyses[0]
            .family_metadata
            .as_mut()
            .expect("Monte Carlo metadata")
        else {
            panic!("the fixture retains Monte Carlo metadata");
        };
        variables[0].samples = vec![10.0, 20.0, 30.0];
        variables[0].mean = 20.0;
        variables[0].std_dev = 10.0;
        variables[0].min = 10.0;
        variables[0].max = 30.0;
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = hist_plan(&mut state, "gain");
        assert_eq!(
            after.moments.expect("retained moments").mean,
            20.0,
            "the sheet reported the previous population's mean"
        );
    }
}
