//! Histogram controls, plot and inspector over qualified retained evidence:
//! the distribution of one measurement in its own unit, the normal it is
//! fitted with, its probability plot, and the yield and capability against
//! the specification that bounds it.

use super::HistogramState;
use crate::presentation::{PlotView, well_hint};
use crate::requirement_lines::{self, RequirementLine};
use crate::strip::{self, LegendChip};
use egui::Ui;
use rspice_results::histogram::display::{HistogramDisplay, hist_axis};
use rspice_results::histogram::{Histogram, HistogramDisplayMode, SampleMoments};
use rspice_results::monte_carlo::{MonteCarloMeanConfidence, MonteCarloMeanInterval};
use rspice_results::population::{self, ProbabilityPlot};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::panels::{
    panel_body, panel_inset, property_row_control_columns, property_row_height,
};
use rspice_ui_kit::plot::{
    self, Axis, PlotSpec, XScale, fmt_column_reading, fmt_si_significant, padded_fit,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::source_revision::SourceRevision;

/// Why the host cannot supply a qualified selected distribution.
#[derive(Clone, Copy)]
pub enum HistogramUnavailable {
    NoSelection { has_measurements: bool },
    Invalid,
}

/// The shape of a population beyond its moments: the median, the sample
/// skewness and excess kurtosis (G1 and G2), and its normal probability plot.
/// Each walks or sorts the whole population, so the host resolves it once
/// per population and lends it to every frame.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleShape {
    pub median: f64,
    pub skewness: Option<f64>,
    pub kurtosis: Option<f64>,
    pub probability: Option<ProbabilityPlot>,
}

impl SampleShape {
    /// The shape of finite `samples`; `None` for an empty set.
    pub fn of(samples: &[f64]) -> Option<Self> {
        let sorted = population::sorted(samples);
        Some(Self {
            median: population::quantile(&sorted, 0.5)?,
            skewness: population::skewness(&sorted),
            kurtosis: population::excess_kurtosis(&sorted),
            probability: population::normal_probability_plot(&sorted),
        })
    }
}

/// Where the specification limits a distribution is judged against came
/// from. The run's own yield and the project's requirement are the run's;
/// the reader's are the reader's to change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitSource {
    /// The yield the run judged.
    Yield,
    /// The project's requirement for the measurement.
    Requirement,
    /// A family measurement on the reading pane.
    Pane,
    /// The reader set them on this sheet.
    Reader,
}

/// The lower and upper specification limits, either optional.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecLimits {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub source: LimitSource,
}

/// How many of the population pass its limits, out of how many trials.
/// Trials with no value count against the yield, as the yield engine counts
/// a diverged trial.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YieldCount {
    pub passing: usize,
    pub total: usize,
    /// Trials the run requested that never completed, named beside the figure.
    pub excluded: usize,
}

impl YieldCount {
    pub fn percent(self) -> Option<f64> {
        (self.total > 0).then(|| 100.0 * self.passing as f64 / self.total as f64)
    }
}

/// One source-qualified population and its cached display projection.
pub struct HistogramPlot<'a> {
    pub histogram: &'a Histogram,
    pub display: Result<&'a HistogramDisplay, &'static str>,
    pub moments: Option<SampleMoments>,
    pub shape: Option<&'a SampleShape>,
    /// The binned quantity's unit; empty when the run states none.
    pub unit: &'a str,
    pub limits: Option<SpecLimits>,
    pub source: &'a SourceRevision,
    /// The values of the members the reader picked, drawn over the bars.
    pub selected: &'a [f64],
    /// The reader's significant digits, which every value on the plot reads at.
    pub significant_digits: usize,
    /// Whether the fitted normal is drawn.
    pub normal_fit: bool,
    /// The samples against their normal quantiles instead of the bars.
    pub normal_quantile: bool,
}

/// Retained method facts for the population selected by the host.
#[derive(Debug, Clone, Copy)]
pub struct MonteCarloMethod {
    pub seed: u64,
    pub runs_requested: usize,
    pub runs_completed: usize,
    pub failures: usize,
    pub mean_confidence: Option<MonteCarloMeanConfidence>,
}

/// Descriptive statistics and qualified specification evidence for one population.
pub struct HistogramInspector<'a> {
    pub histogram: &'a Histogram,
    pub moments: Option<SampleMoments>,
    pub shape: Option<&'a SampleShape>,
    pub unit: &'a str,
    pub method: Option<MonteCarloMethod>,
    pub limits: Option<SpecLimits>,
    pub yield_count: Option<YieldCount>,
    /// The reader's own limits, offered for editing where the run states none.
    pub reader_limits: (Option<f64>, Option<f64>),
    /// The reader's significant digits, which every statistic reads at.
    pub significant_digits: usize,
}

/// What the reader changed in the inspector.
#[derive(Debug, Default, PartialEq)]
pub struct HistogramInspectorResponse {
    /// The reader's own lower and upper limits, when they set either.
    pub reader_limits: Option<(Option<f64>, Option<f64>)>,
}

/// What a click on the histogram asked of the member selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistogramPick {
    /// Pick the members counted in this bin.
    Bin(usize),
    /// Pick no member.
    Clear,
}

/// Plot outcomes applied by the host to its document view.
#[derive(Default)]
pub struct HistogramViewResponse {
    pub plot: Option<plot::PlotResponse>,
    pub fit_requested: bool,
    pub picked: Option<HistogramPick>,
    /// The reader followed a spec limit to the specification it states.
    pub open_specification: bool,
}

/// The share of the frame the single bar of a degenerate distribution covers.
const DEGENERATE_BAR_FRACTION: f32 = 0.18;
/// How opaque a bar's fill is over the well: enough that a bar reads as a
/// filled bar in the trace hue, light enough that the fit and the ±1σ band
/// read through it.
const BAR_FILL: f32 = 0.38;
/// Samples along the fitted normal curve.
const FIT_SAMPLES: usize = 241;
/// The bar's measurement select: the width of a long measurement name.
const MEASURE_WIDTH: f32 = 150.0;
/// The bar's display select: its longest choice.
const DISPLAY_WIDTH: f32 = 132.0;
const DISPLAY_CHOICES: [&str; 5] = [
    "Count",
    "Density",
    "Percent",
    "Cumulative",
    "Normal quantile",
];

/// A value of the binned quantity at the reader's digits, in its unit,
/// zeros kept, as a readout row states it.
fn quantity(value: f64, unit: &str, digits: usize) -> String {
    fmt_column_reading(value, unit, digits)
}

/// A value of the binned quantity as a label or a limit field states it:
/// at most the reader's digits, the zeros that only pad it dropped, so a
/// limit reads as it was typed.
fn said(value: f64, unit: &str, digits: usize) -> String {
    fmt_si_significant(value, unit, digits)
}

/// A dimensionless statistic at the reader's digits, never prefixed.
fn plain(value: f64, digits: usize) -> String {
    fmt_column_reading(value, "", digits)
}

/// A percentage at the reader's digits.
fn percent(value: f64, digits: usize) -> String {
    fmt_column_reading(value, "%", digits)
}

/// The yield figure, over the trials the population holds.
///
/// A percentage alone reads as a property of the run, and it is not: the
/// denominator is the trials the population holds, and a Monte Carlo that
/// requested a hundred and completed ninety reports a yield over ninety.
fn yield_label(count: YieldCount, digits: usize) -> String {
    count
        .percent()
        .map_or_else(|| "\u{2014}".to_owned(), |share| percent(share, digits))
}

/// The yield's 95 % Wilson interval, which says how far the figure can be
/// trusted at the population's size.
fn interval_label(count: YieldCount, digits: usize) -> Option<String> {
    let (low, high) = population::wilson_interval(count.passing, count.total)?;
    Some(format!(
        "{} to {}",
        plain(low, digits),
        percent(high, digits)
    ))
}

/// The trials the yield was counted over.
fn evaluated_label(count: YieldCount) -> String {
    let mut label = format!("{} of {}", count.passing, count.total);
    if count.excluded > 0 {
        label.push_str(&format!(" \u{b7} {} diverged excluded", count.excluded));
    }
    label
}

/// What the method panel says about display binning.
///
/// A collapsed distribution states that it is one: "1 retained bins" reads as
/// a count that happens to be small, when what the reader needs to know is
/// that the bin has no width because the measurement never moved.
fn binning_label(histogram: &rspice_results::histogram::Histogram) -> String {
    if hist_axis(histogram).degenerate_at.is_some() {
        return "1 display bin · zero width, every sample at one value".to_owned();
    }
    let count = histogram.bins.len();
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} display bin{plural}")
}

/// The display select's choice for `settings`.
fn display_choice(settings: &HistogramState) -> usize {
    if settings.normal_quantile {
        return 4;
    }
    match settings.mode {
        HistogramDisplayMode::Count => 0,
        HistogramDisplayMode::Pdf => 1,
        HistogramDisplayMode::Percent => 2,
        HistogramDisplayMode::Cdf => 3,
    }
}

/// The sheet's own controls in its bar: the measurement, how it is drawn,
/// how finely it is binned, and whether the normal fit is drawn. True when a
/// change asks for a fitted view.
pub fn domain_bar(ui: &mut Ui, settings: &mut HistogramState, names: &[String]) -> bool {
    if names.is_empty() {
        return false;
    }
    if settings.selected.is_none() {
        settings.selected = names.first().cloned();
    }
    let before = settings.clone();
    crate::chrome::bars::bar_label(ui, "Measure");
    let index = names
        .iter()
        .position(|name| Some(name) == settings.selected.as_ref())
        .unwrap_or(0);
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.hist.measurement",
        "Measurement",
        names.get(index).map_or("", String::as_str),
        names,
        MEASURE_WIDTH,
    ) {
        settings.selected = names.get(picked).cloned();
    }
    ui.add_space(8.0);
    crate::chrome::bars::bar_label(ui, "Display");
    let choices = DISPLAY_CHOICES.map(str::to_owned).to_vec();
    let choice = display_choice(settings);
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.hist.display",
        "Display",
        DISPLAY_CHOICES[choice],
        &choices,
        DISPLAY_WIDTH,
    ) {
        settings.normal_quantile = picked == 4;
        settings.mode = match picked {
            1 => HistogramDisplayMode::Pdf,
            2 => HistogramDisplayMode::Percent,
            3 => HistogramDisplayMode::Cdf,
            4 => settings.mode,
            _ => HistogramDisplayMode::Count,
        };
    }
    ui.add_space(8.0);
    crate::chrome::bars::bar_label(ui, "Bins");
    let binned = !settings.normal_quantile && settings.mode != HistogramDisplayMode::Cdf;
    ui.add_enabled(
        binned,
        egui::DragValue::new(&mut settings.bin_count)
            .range(1..=1000)
            .speed(0.25),
    )
    .on_hover_text("Bins across the binned range")
    .on_disabled_hover_text("A cumulative or quantile plot reads every sample, unbinned");
    ui.add_space(8.0);
    let fit = settings.normal_fit;
    let response = rspice_ui_kit::widgets::chip(ui, "Fit", fit).on_hover_text(if fit {
        "The normal distribution with the sample mean and standard deviation drawn"
    } else {
        "The fitted normal distribution hidden"
    });
    if response.clicked() {
        settings.normal_fit = !fit;
    }
    // A new measurement, a new reading or new bins are a new range to fit;
    // the normal fit drawn or hidden is not.
    settings.selected != before.selected
        || settings.mode != before.mode
        || settings.normal_quantile != before.normal_quantile
        || settings.bin_count != before.bin_count
}

/// The fitted normal over `x0..x1`, in the ordinate of `mode`: the expected
/// count or percentage of a bin `width` wide, the density, or the cumulative
/// probability.
fn normal_fit_curve(
    moments: SampleMoments,
    mode: HistogramDisplayMode,
    width: f64,
    (x0, x1): (f64, f64),
) -> Option<(Vec<f64>, Vec<f64>)> {
    let sigma = moments.std_dev;
    if !(sigma.is_finite() && sigma > 0.0 && x1 > x0) {
        return None;
    }
    let scale = match mode {
        HistogramDisplayMode::Count => moments.count as f64 * width,
        HistogramDisplayMode::Percent => 100.0 * width,
        HistogramDisplayMode::Pdf | HistogramDisplayMode::Cdf => 1.0,
    };
    let step = (x1 - x0) / (FIT_SAMPLES - 1) as f64;
    let xs: Vec<f64> = (0..FIT_SAMPLES).map(|i| x0 + step * i as f64).collect();
    let ys = xs
        .iter()
        .map(|x| {
            let z = (x - moments.mean) / sigma;
            if mode == HistogramDisplayMode::Cdf {
                population::normal_cdf(z)
            } else {
                scale * population::normal_pdf(z) / sigma
            }
        })
        .collect();
    Some((xs, ys))
}

/// The width of one bin of the drawn histogram.
fn bin_width(histogram: &Histogram) -> f64 {
    match (histogram.bins.first(), histogram.bins.last()) {
        (Some(first), Some(last)) if !histogram.bins.is_empty() => {
            (last.upper - first.lower) / histogram.bins.len() as f64
        }
        _ => 0.0,
    }
}

/// Draw the qualified distribution without changing host source or selection state.
pub fn show(
    ui: &mut Ui,
    model: Result<HistogramPlot<'_>, HistogramUnavailable>,
    view: PlotView,
    cache: &mut plot::DecimationCache,
) -> HistogramViewResponse {
    let out = HistogramViewResponse::default();
    let model = match model {
        Ok(model) => model,
        Err(HistogramUnavailable::NoSelection { has_measurements }) => {
            well_hint(
                ui,
                if has_measurements {
                    "The selected measurement is unavailable. Choose a measurement in the bar."
                } else {
                    "No distribution yet — run a Monte Carlo analysis"
                },
            );
            return out;
        }
        Err(HistogramUnavailable::Invalid) => {
            well_hint(
                ui,
                "The selected samples, retained statistics, or display range are invalid or unavailable",
            );
            return out;
        }
    };
    let histogram = model.histogram;
    if histogram.bins.is_empty() || histogram.total_count == 0 {
        well_hint(ui, "The selected distribution is empty");
        return out;
    }
    if model.normal_quantile {
        return show_probability(ui, &model, view, cache);
    }
    let display = match model.display {
        Ok(display) => display,
        Err(reason) => {
            well_hint(ui, reason);
            return out;
        }
    };
    show_bars(ui, &model, display, view, cache)
}

fn show_bars(
    ui: &mut Ui,
    model: &HistogramPlot<'_>,
    display: &HistogramDisplay,
    view: PlotView,
    cache: &mut plot::DecimationCache,
) -> HistogramViewResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut out = HistogramViewResponse::default();
    let histogram = model.histogram;
    let digits = model.significant_digits;
    let unit = model.unit;
    let mode = display.mode;
    let moments = model.moments;
    let limits = model.limits;
    let spec_limits = limits.map(|limits| (limits.lower, limits.upper));

    let axis = hist_axis(histogram);
    let (x0, x1) = view.x.unwrap_or((axis.x0, axis.x1));
    if !(x1 - x0).is_finite() || x1 <= x0 {
        well_hint(
            ui,
            "The sample range exceeds the linear display range. Set a narrower range in the Distribution panel.",
        );
        return out;
    }
    let fit = moments
        .filter(|_| model.normal_fit && axis.degenerate_at.is_none())
        .and_then(|moments| normal_fit_curve(moments, mode, bin_width(histogram), (x0, x1)));
    let fit_peak = fit
        .as_ref()
        .map_or(0.0, |(_, ys)| ys.iter().copied().fold(0.0, f64::max));
    let y_top = if display.cdf.is_some() {
        display.y_max()
    } else {
        display.y_max().max(fit_peak * 1.08)
    };
    let (y0, y1) = view.y.unwrap_or((0.0, y_top));

    let subtitle = format!("{} samples", histogram.total_count);
    let mut legend = vec![LegendChip {
        name: mode.label(),
        color: c.traces[0],
        on: true,
        glyph: strip::LegendGlyph::Line,
    }];
    if fit.is_some() {
        legend.push(LegendChip {
            name: "normal fit",
            color: c.text,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    if moments.is_some_and(|moments| moments.std_dev > 0.0) {
        legend.push(LegendChip {
            name: "\u{b1}1\u{3c3}",
            color: c.text_dim,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    if spec_limits.is_some() {
        legend.push(LegendChip {
            name: "spec limit",
            color: c.err,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    let selected_count = model
        .selected
        .iter()
        .filter(|value| histogram.bin_of(**value).is_some())
        .count();
    let selected_name = format!("{selected_count} picked");
    if selected_count > 0 {
        legend.push(LegendChip {
            name: &selected_name,
            color: c.warn,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    let header = strip::StripHeader::new("MC", &subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    out.fit_requested = header.fit_clicked;

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, unit).with_label(&histogram.name),
        XScale::Linear,
        Axis::linear_with(y0, y1, mode.ordinate_unit(unit), 5),
    )
    .with_significant_digits(digits)
    .accessible_name("Statistical histogram");

    // The mean and one standard deviation either side, from the exact
    // retained moments. The mean's value is read once, in the panel.
    if let Some(moments) = moments {
        if moments.std_dev > 0.0 {
            let band_start = (moments.mean - moments.std_dev).max(x0);
            let band_end = (moments.mean + moments.std_dev).min(x1);
            if band_start < band_end {
                spec.bands.push(plot::Band {
                    x0: band_start,
                    x1: band_end,
                });
            }
        }
        spec.markers.push(plot::Marker {
            x: moments.mean,
            y: y1 * 0.86,
            color: c.accent,
            label: "\u{3bc}".to_owned(),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }

    if let Some((lsl, usl)) = spec_limits {
        for (limit, name) in [(lsl, "LSL"), (usl, "USL")] {
            if let Some(limit) = limit
                && limit > x0
                && limit < x1
            {
                spec.markers.push(plot::Marker {
                    x: limit,
                    y: y1 * 0.72,
                    color: c.err,
                    label: format!("{name} {}", said(limit, unit, digits)),
                    drop_line: true,
                    label_dy: 0.0,
                    shape: plot::MarkerShape::Point,
                });
            }
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
                said(value, unit, digits)
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
    let ordinates = &display.ordinates;
    // The bars in the first trace hue, filled so they read as bars, not outlines.
    let accent = c.traces[0];
    let accent_dim = accent.gamma_multiply(BAR_FILL);
    let err = c.err;
    let picked_fill = c.warn.gamma_multiply(0.55);
    let picked_edge = c.warn;
    // Each bin's share of the picked members, drawn as the matching part of
    // its bar: in a count, a density or a percentage alike, the picked part
    // stands in the bin's own ordinate scale.
    let mut picked_per_bin = vec![0usize; bins.len()];
    for value in model.selected {
        if let Some(bin) = histogram.bin_of(*value) {
            picked_per_bin[bin] += 1;
        }
    }
    cache.ensure_source(model.source);
    if let Some(cdf) = &display.cdf {
        spec.traces
            .push(plot::Trace::new(&cdf.x, &cdf.y, accent).cache_key(0x4849_5354_4344_4600));
    }
    if let Some((xs, ys)) = &fit {
        spec.traces.push(plot::Trace::new(xs, ys, c.text).thin());
    }
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
        if let Some(cdf) = &display.cdf {
            let first = cdf.x[0];
            let last = *cdf.x.last().unwrap();
            for (left, right, value) in [(x0, first.min(x1), 0.0), (last.max(x0), x1, 1.0)] {
                if left < right {
                    painter.line_segment(
                        [
                            egui::pos2(mapper.x(left), mapper.y(value)),
                            egui::pos2(mapper.x(right), mapper.y(value)),
                        ],
                        egui::Stroke::new(1.8, accent),
                    );
                }
            }
            return;
        }
        // One bar for a distribution whose bin edges coincide: the retained
        // rectangle has no width, so the frame supplies one.
        if let Some(value) = degenerate_at {
            if let Some(&ordinate) = ordinates.first().filter(|value| **value > 0.0) {
                let half = mapper.rect.width() * DEGENERATE_BAR_FRACTION * 0.5;
                let centre = mapper.x(value);
                let rect = egui::Rect::from_min_max(
                    egui::pos2(centre - half, mapper.y(ordinate)),
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
        for (bin, &ordinate) in bins.iter().zip(ordinates) {
            if ordinate == 0.0 {
                continue;
            }
            let left = mapper.x(bin.lower) + 1.0;
            let right = (mapper.x(bin.upper) - 1.0).max(left + 1.0);
            let top = mapper.y(ordinate);
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
        for ((bin, &ordinate), &picked) in bins.iter().zip(ordinates).zip(&picked_per_bin) {
            if picked == 0 || bin.count == 0 {
                continue;
            }
            let share = ordinate * picked as f64 / bin.count as f64;
            let left = mapper.x(bin.lower) + 1.0;
            let right = (mapper.x(bin.upper) - 1.0).max(left + 1.0);
            let rect = egui::Rect::from_min_max(
                egui::pos2(left, mapper.y(share)),
                egui::pos2(right, mapper.y(0.0)),
            );
            painter.rect(
                rect,
                0.0,
                picked_fill,
                egui::Stroke::new(1.5, picked_edge),
                egui::StrokeKind::Inside,
            );
        }
    }));

    let ordinate_unit = mode.ordinate_unit(unit);
    let readout = |x: f64| -> Vec<(String, String)> {
        let value = display.value_at(histogram, x);
        vec![
            (histogram.name.clone(), quantity(x, unit, digits)),
            (
                mode.label().to_owned(),
                if mode == HistogramDisplayMode::Count {
                    format!("{value:.0}")
                } else {
                    fmt_column_reading(value, &ordinate_unit, digits)
                },
            ),
        ]
    };

    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    if let Some((lsl, usl)) = spec_limits {
        let action = requirement_lines::specification_action(&histogram.name);
        let lines = [lsl, usl]
            .into_iter()
            .flatten()
            .filter(|x| *x > x0 && *x < x1)
            .map(|x| (RequirementLine::Vertical { x }, action.clone()))
            .collect::<Vec<_>>();
        out.open_specification =
            requirement_lines::follow(ui, &response, XScale::Linear, &lines).is_some();
    }
    // A click on a bar picks the members it counts; a click beside the
    // bars, or Escape, picks none. A cumulative curve has no bars to pick.
    if display.cdf.is_none() {
        if let Some(x) = response.clicked_x {
            out.picked = Some(
                histogram
                    .bin_of(x)
                    .filter(|bin| histogram.bins[*bin].count > 0)
                    .map_or(HistogramPick::Clear, HistogramPick::Bin),
            );
        } else if response.response.hovered()
            && ui.input(|input| input.key_pressed(egui::Key::Escape))
        {
            out.picked = Some(HistogramPick::Clear);
        }
    }
    out.plot = Some(response);
    out
}

/// The normal probability plot: each sorted sample against the standard
/// normal quantile of its plotting position, the fitted normal as the line
/// they lie on when the population is normal, and the limits across it.
fn show_probability(
    ui: &mut Ui,
    model: &HistogramPlot<'_>,
    view: PlotView,
    cache: &mut plot::DecimationCache,
) -> HistogramViewResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut out = HistogramViewResponse::default();
    let digits = model.significant_digits;
    let unit = model.unit;
    let Some(probability) = model.shape.and_then(|shape| shape.probability.as_ref()) else {
        well_hint(ui, "The selected distribution has no probability plot");
        return out;
    };
    let quantiles: Vec<f64> = probability.points.iter().map(|(z, _)| *z).collect();
    let samples: Vec<f64> = probability.points.iter().map(|(_, x)| *x).collect();
    let z_range = (
        quantiles.first().copied().unwrap_or(-1.0),
        quantiles.last().copied().unwrap_or(1.0),
    );
    let (z0, z1) = view.x.unwrap_or_else(|| padded_fit(z_range));
    let limits = model.limits;
    let mut y_values: Vec<f64> = samples.clone();
    if let Some(limits) = limits {
        y_values.extend(limits.lower);
        y_values.extend(limits.upper);
    }
    let y_range = y_values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), value| {
            (lo.min(*value), hi.max(*value))
        });
    let (y0, y1) = view.y.unwrap_or_else(|| padded_fit(y_range));

    let fit = model
        .moments
        .filter(|moments| model.normal_fit && moments.std_dev > 0.0)
        .map(|moments| {
            let xs = vec![z0, z1];
            let ys = vec![
                moments.mean + moments.std_dev * z0,
                moments.mean + moments.std_dev * z1,
            ];
            (xs, ys)
        });

    let subtitle = match probability.correlation {
        Some(r) => format!("{} samples \u{b7} r {}", samples.len(), said(r, "", digits)),
        None => format!("{} samples", samples.len()),
    };
    let mut legend = vec![LegendChip {
        name: "samples",
        color: c.traces[0],
        on: true,
        glyph: strip::LegendGlyph::Line,
    }];
    if fit.is_some() {
        legend.push(LegendChip {
            name: "normal fit",
            color: c.text,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    if limits.is_some() {
        legend.push(LegendChip {
            name: "spec limit",
            color: c.err,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    let header = strip::StripHeader::new("MC", &subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    out.fit_requested = header.fit_clicked;

    let mut spec = PlotSpec::new(
        Axis::linear(z0, z1, "").with_label("z"),
        XScale::Linear,
        Axis::linear_with(y0, y1, unit, 5),
    )
    .with_significant_digits(digits)
    .accessible_name("Normal probability plot");
    cache.ensure_source(model.source);
    if let Some((xs, ys)) = &fit {
        spec.traces.push(plot::Trace::new(xs, ys, c.text).thin());
    }
    spec.traces.push(
        plot::Trace::new(&quantiles, &samples, c.traces[0])
            .draw(plot::DrawMode::Points)
            .cache_key(0x4849_5354_5150_5000),
    );
    if let Some(limits) = limits {
        for (limit, name) in [(limits.lower, "LSL"), (limits.upper, "USL")] {
            if let Some(limit) = limit {
                spec.limit_lines.push(plot::LimitLine {
                    y: limit,
                    color: c.err,
                    label: format!("{name} {}", said(limit, unit, digits)),
                });
            }
        }
    }
    let moments = model.moments;
    let name = model.histogram.name.clone();
    let readout = |z: f64| -> Vec<(String, String)> {
        let mut rows = vec![("Normal quantile".to_owned(), plain(z, digits))];
        if let Some(moments) = moments.filter(|moments| moments.std_dev > 0.0) {
            rows.push((
                format!("{name} fit"),
                quantity(moments.mean + moments.std_dev * z, unit, digits),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    out.plot = Some(response);
    out
}

/// Draw local range controls; true requests a fitted view before rebuilding.
pub fn range_controls(
    ui: &mut Ui,
    settings: &mut HistogramState,
    current_range: Option<(f64, f64)>,
) -> bool {
    section_header(ui, "Display range", None);
    let mut range_changed = panel_body(ui, |ui| {
        ui.checkbox(&mut settings.custom_range, "Custom range")
            .changed()
    });
    if range_changed
        && settings.custom_range
        && let Some((min, max)) = current_range
        && min < max
    {
        settings.custom_min = min;
        settings.custom_max = max;
    }
    if settings.custom_range {
        range_changed |= bound_row(ui, "Minimum", &mut settings.custom_min);
        range_changed |= bound_row(ui, "Maximum", &mut settings.custom_max);
    }
    range_changed
}

/// One bound of the custom range as a property row: its name in the label
/// column and the drag field that sets it in the value column.
fn bound_row(ui: &mut Ui, label: &str, value: &mut f64) -> bool {
    property_field(ui, label, |ui| ui.add(egui::DragValue::new(value)))
}

/// A property row: `label` in the label column and the field `add` draws in
/// the value column.
fn property_field(ui: &mut Ui, label: &str, add: impl FnOnce(&mut Ui) -> egui::Response) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let trailing = panel_inset(ui);
    let (label_left, value_left) = property_row_control_columns(ui, width);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width, property_row_height(ui)),
        egui::Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + label_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let value_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + value_left, rect.top()),
        egui::pos2(rect.right() - trailing, rect.bottom()),
    );
    let field = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(value_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            add,
        )
        .inner;
    ui.ctx()
        .accesskit_node_builder(field.id, |node| node.set_label(label));
    field.changed()
}

/// A limit the reader types in engineering notation ("1.2m", "950 mV"),
/// taken when the field loses focus; empty clears it. True when it changed.
fn limit_field(
    ui: &mut Ui,
    label: &str,
    limit: &mut Option<f64>,
    unit: &str,
    digits: usize,
) -> bool {
    let id = ui.id().with(("rspice.results.hist.limit", label));
    let mut changed = false;
    property_field(ui, label, |ui| {
        let shown = limit.map_or_else(String::new, |value| said(value, unit, digits));
        let mut text = ui
            .data(|data| data.get_temp::<String>(id))
            .unwrap_or_else(|| shown.clone());
        let response = ui.add(
            egui::TextEdit::singleline(&mut text)
                .id(id.with("field"))
                .font(theme::field_mono())
                .hint_text("none")
                .desired_width(f32::INFINITY),
        );
        if response.has_focus() {
            ui.data_mut(|data| data.insert_temp(id, text.clone()));
        } else if response.lost_focus() {
            ui.data_mut(|data| data.remove::<String>(id));
            let trimmed = text.trim();
            let typed = trimmed.strip_suffix(unit).unwrap_or(trimmed).trim();
            let parsed = if typed.is_empty() {
                Some(None)
            } else {
                rspice_vocabulary::quantity::parse_engineering_value(typed)
                    .ok()
                    .filter(|value| value.is_finite())
                    .map(Some)
            };
            if let Some(parsed) = parsed
                && parsed != *limit
            {
                *limit = parsed;
                changed = true;
            }
        }
        response
    });
    changed
}

/// Render descriptive and specification evidence already qualified by the
/// host, and the reader's own limits where the run states none.
pub fn right_panel(
    ui: &mut Ui,
    model: Result<HistogramInspector<'_>, HistogramUnavailable>,
) -> HistogramInspectorResponse {
    let mut out = HistogramInspectorResponse::default();
    let model = match model {
        Ok(model) => model,
        Err(reason) => {
            section_header(ui, "Distribution", None);
            crate::presentation::panel_note(
                ui,
                match reason {
                    HistogramUnavailable::NoSelection { .. } => {
                        "Stats appear once a Monte Carlo run is loaded."
                    }
                    HistogramUnavailable::Invalid => "Invalid display range or sample data.",
                },
            );
            return out;
        }
    };
    let histogram = model.histogram;
    let digits = model.significant_digits;
    let unit = model.unit;
    let value = |value: f64| quantity(value, unit, digits);
    let absent = |reason: &str| format!("\u{2014} {reason}");
    section_header(ui, "Distribution", None);
    if let Some(moments) = model.moments {
        let mut rows = vec![
            ("N", moments.count.to_string(), false),
            (
                "Mean",
                value(rspice_results::number_format::snapped_to_spread(
                    moments.mean,
                    moments.max - moments.min,
                )),
                true,
            ),
        ];
        if let Some(shape) = model.shape {
            rows.push(("Median", value(shape.median), false));
        }
        rows.push(("Std dev", value(moments.std_dev), true));
        if let Some(shape) = model.shape {
            rows.push((
                "Skewness",
                shape
                    .skewness
                    .map_or_else(|| absent("needs 3 samples"), |g1| plain(g1, digits)),
                false,
            ));
            rows.push((
                "Excess kurtosis",
                shape
                    .kurtosis
                    .map_or_else(|| absent("needs 4 samples"), |g2| plain(g2, digits)),
                false,
            ));
        }
        rows.push(("Min", value(moments.min), false));
        rows.push(("Max", value(moments.max), false));
        if let Some(r) = model
            .shape
            .and_then(|shape| shape.probability.as_ref())
            .and_then(|plot| plot.correlation)
        {
            rows.push(("Normal plot r", plain(r, digits), false));
        }
        let outside = histogram.underflow + histogram.overflow;
        if outside > 0 {
            rows.push(("Outside range", outside.to_string(), false));
        }
        crate::presentation::stat_table(ui, &rows);
    } else {
        crate::presentation::panel_note(ui, "Statistics unavailable: sample data inconsistent.");
    }

    let mc = model.method;
    let mean_confidence = mc.and_then(|authority| authority.mean_confidence);
    // The interval and the level it was taken at are one statement.
    let confidence_row = mean_confidence.map(|confidence| {
        (
            format!("Mean {}% CI", confidence.level_pct),
            match confidence.interval {
                MonteCarloMeanInterval::Available { lower, upper } => {
                    format!("{} to {}", value(lower), value(upper))
                }
                MonteCarloMeanInterval::InsufficientSamples => {
                    "Unavailable — fewer than two successful trials".into()
                }
                MonteCarloMeanInterval::Unrepresentable => {
                    "Unavailable — limits exceed the finite range".into()
                }
            },
        )
    });
    if let Some((label, limits)) = &confidence_row {
        crate::presentation::stat_table(ui, &[(label.as_str(), limits.clone(), true)]);
    }

    section_header(ui, "Specification", None);
    let editable = model
        .limits
        .is_none_or(|limits| limits.source == LimitSource::Reader);
    if editable {
        let (mut lower, mut upper) = model.reader_limits;
        let mut changed = limit_field(ui, "LSL", &mut lower, unit, digits);
        changed |= limit_field(ui, "USL", &mut upper, unit, digits);
        if changed {
            out.reader_limits = Some((lower, upper));
        }
    } else if let Some(limits) = model.limits {
        let from = match limits.source {
            LimitSource::Yield => "the run's yield",
            LimitSource::Requirement => "the project's requirement",
            LimitSource::Pane => "the reading pane's measurement",
            LimitSource::Reader => "this sheet",
        };
        crate::presentation::stat_table(
            ui,
            &[
                (
                    "LSL",
                    limits.lower.map_or_else(|| "none".to_owned(), value),
                    false,
                ),
                (
                    "USL",
                    limits.upper.map_or_else(|| "none".to_owned(), value),
                    false,
                ),
                ("From", from.to_owned(), false),
            ],
        );
    }
    if let (Some(limits), Some(count)) = (model.limits, model.yield_count) {
        let capability = model
            .moments
            .map(|moments| {
                population::capability(moments.mean, moments.std_dev, limits.lower, limits.upper)
            })
            .unwrap_or_default();
        let interval = interval_label(count, digits).unwrap_or_else(|| "\u{2014}".to_owned());
        let rows = [
            ("Yield", yield_label(count, digits), true),
            ("95 % CI (Wilson)", interval, false),
            ("Passing", evaluated_label(count), false),
            (
                "Cp",
                capability
                    .cp
                    .map_or_else(|| absent("two limits"), |cp| plain(cp, digits)),
                false,
            ),
            (
                "Cpk",
                capability
                    .cpk
                    .map_or_else(|| absent("no spread"), |cpk| plain(cpk, digits)),
                true,
            ),
        ];
        crate::presentation::stat_table(ui, &rows);
    }

    section_header(ui, "Run", None);
    let mut run_rows = Vec::new();
    if let Some(authority) = mc {
        run_rows.push((
            "Run completion",
            format!(
                "{} / {} · {} failed",
                authority.runs_completed, authority.runs_requested, authority.failures
            ),
            false,
        ));
        // In decimal, as the Monte Carlo setup takes it.
        run_rows.push(("Seed", authority.seed.to_string(), false));
    }
    run_rows.push(("Binning", binning_label(histogram), false));
    crate::presentation::stat_table(ui, &run_rows);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The histogram a zero-variation Monte Carlo produces.
    ///
    /// `VariableStatistics::compute_histogram` returns `(vec![n], vec![v, v])`
    /// whenever the sample range is not positive, and
    /// `populate_monte_carlo_histograms` maps that to one bin whose two edges
    /// are the same number. This is that shape, byte for byte.
    fn zero_variation_histogram(
        value: f64,
        samples: usize,
    ) -> rspice_results::histogram::Histogram {
        rspice_results::histogram::Histogram {
            name: "V(out)".to_owned(),
            bins: vec![rspice_results::histogram::HistogramBin {
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
            fmt_column_reading(axis.x0, "", 4),
            fmt_column_reading(axis.x1, "", 4),
            "both ends of the axis print the same number"
        );
    }

    /// The percentage states its 95 % Wilson interval and the population
    /// it was measured over, at the reader's digits.
    ///
    /// "97.0 %" alone reads as a property of the run. It is a property of the
    /// trials the population holds, which is not the number requested when
    /// trials diverged, and at a hundred trials it is uncertain by several
    /// points either way.
    #[test]
    fn the_yield_figure_states_its_interval_and_population() {
        let count = YieldCount {
            passing: 97,
            total: 100,
            excluded: 0,
        };
        // Wilson at 97 of 100: 91.548 % to 98.975 %.
        assert_eq!(yield_label(count, 4), "97.00 %");
        assert_eq!(
            interval_label(count, 4).as_deref(),
            Some("91.55 to 98.97 %")
        );
        assert_eq!(interval_label(count, 3).as_deref(), Some("91.5 to 99.0 %"));
        assert_eq!(evaluated_label(count), "97 of 100");
        assert_eq!(
            evaluated_label(YieldCount {
                excluded: 10,
                ..count
            }),
            "97 of 100 \u{b7} 10 diverged excluded"
        );
    }

    #[test]
    fn a_degenerate_distribution_says_its_bin_has_no_width() {
        assert_eq!(
            binning_label(&zero_variation_histogram(1.5, 40)),
            "1 display bin · zero width, every sample at one value"
        );
    }

    #[test]
    fn an_ordinary_distribution_keeps_its_padded_data_window() {
        let mut histogram = zero_variation_histogram(0.0, 0);
        histogram.bins = vec![
            rspice_results::histogram::HistogramBin {
                lower: 1.0,
                upper: 2.0,
                count: 3,
                weight: 3.0,
            },
            rspice_results::histogram::HistogramBin {
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
        assert_eq!(binning_label(&histogram), "2 display bins");
    }

    /// A click on a bar picks the bin it stands in; a click beside the
    /// bars picks no member; the picked members are counted in the legend.
    /// A click on a spec limit follows it to its specification instead.
    #[test]
    fn a_click_on_a_bar_picks_its_bin() {
        use rspice_results::histogram::HistogramBuilder;
        let samples: Vec<f64> = (0..40).map(|trial| f64::from(trial % 10)).collect();
        let histogram = HistogramBuilder::new()
            .name("V(out)")
            .bin_count(10)
            .range(-0.5, 9.5)
            .build(&samples);
        let display =
            HistogramDisplay::new(&histogram, &samples, HistogramDisplayMode::Count).unwrap();
        let source = SourceRevision::default();
        let selected = [3.0, 3.0, 4.0];
        let pick_at = |value: Option<f64>, spec_limits: Option<(Option<f64>, Option<f64>)>| {
            let ctx = egui::Context::default();
            rspice_ui_kit::Theme::default().apply(&ctx);
            let mut cache = plot::DecimationCache::default();
            let mut target = None;
            let mut picked = None;
            let mut opened = false;
            let mut text = String::new();
            for frame in 0..4 {
                let mut input = rspice_ui_kit::raster::display_input(egui::vec2(900.0, 500.0), 1.0);
                if let (1 | 2, Some(at)) = (frame, target) {
                    input.events = vec![
                        egui::Event::PointerMoved(at),
                        egui::Event::PointerButton {
                            pos: at,
                            button: egui::PointerButton::Primary,
                            pressed: frame == 1,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ];
                }
                let output = ctx.run_ui(input, |root| {
                    egui::CentralPanel::default().show(root, |ui| {
                        let out = show(
                            ui,
                            Ok(HistogramPlot {
                                histogram: &histogram,
                                display: Ok(&display),
                                moments: None,
                                shape: None,
                                unit: "",
                                limits: spec_limits.map(|(lower, upper)| SpecLimits {
                                    lower,
                                    upper,
                                    source: LimitSource::Requirement,
                                }),
                                source: &source,
                                selected: &selected,
                                significant_digits: 4,
                                normal_fit: false,
                                normal_quantile: false,
                            }),
                            PlotView::default(),
                            &mut cache,
                        );
                        opened |= out.open_specification;
                        let plot = out.plot.expect("the histogram is drawn");
                        if frame == 0 {
                            let ((x0, x1), _) = plot.axes;
                            let rect = plot.plot_rect;
                            // An infinite value is the plot's right edge,
                            // beside the last bar.
                            target = value.map(|value| {
                                let x = if value.is_finite() {
                                    rect.left() + ((value - x0) / (x1 - x0)) as f32 * rect.width()
                                } else {
                                    rect.right() - 2.0
                                };
                                egui::pos2(x, rect.bottom() - 4.0)
                            });
                        }
                        picked = picked.or(out.picked);
                    });
                });
                if frame == 0 {
                    for shape in &output.shapes {
                        if let egui::Shape::Text(value) = &shape.shape {
                            text.push_str(value.galley.text());
                        }
                    }
                }
            }
            (picked, text, opened)
        };
        let (picked, text, _) = pick_at(Some(7.0), None);
        assert_eq!(picked, Some(HistogramPick::Bin(7)));
        assert!(text.contains("3 picked"), "{text}");
        let (beside, ..) = pick_at(Some(f64::INFINITY), None);
        assert_eq!(beside, Some(HistogramPick::Clear));
        assert_eq!(pick_at(None, None).0, None);

        // A spec limit is the way to its specification, not a bar.
        let limits = Some((Some(2.5), Some(7.5)));
        let (picked, _, opened) = pick_at(Some(7.5), limits);
        assert!(opened);
        assert_eq!(picked, None, "the limit took the click");
        let (picked, _, opened) = pick_at(Some(5.0), limits);
        assert!(!opened);
        assert_eq!(picked, Some(HistogramPick::Bin(5)));
    }
}
