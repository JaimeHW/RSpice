//! Histogram controls, plot and inspector over qualified retained evidence.

use super::HistogramState;
use crate::presentation::{PlotView, well_hint};
use crate::strip::{self, LegendChip};
use egui::Ui;
use rspice_app_types::source_revision::SourceRevision;
use rspice_results::histogram::display::{HistogramDisplay, hist_axis};
use rspice_results::histogram::{Histogram, HistogramDisplayMode, SampleMoments};
use rspice_results::monte_carlo::{MonteCarloMeanConfidence, MonteCarloMeanInterval};
use rspice_results::yield_analysis::YieldResult;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::section_header;

/// Why the host cannot supply a qualified selected distribution.
#[derive(Clone, Copy)]
pub enum HistogramUnavailable {
    NoSelection { has_measurements: bool },
    Invalid,
}

/// One source-qualified population and its cached display projection.
pub struct HistogramPlot<'a> {
    pub histogram: &'a Histogram,
    pub display: Result<&'a HistogramDisplay, &'static str>,
    pub moments: Option<SampleMoments>,
    pub spec_limits: Option<(Option<f64>, Option<f64>)>,
    pub source: &'a SourceRevision,
}

/// Retained method facts for the population selected by the host.
#[derive(Clone, Copy)]
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
    pub method: Option<MonteCarloMethod>,
    pub yield_result: Option<&'a YieldResult>,
}

/// Plot outcomes applied by the host to its document view.
#[derive(Default)]
pub struct HistogramViewResponse {
    pub plot: Option<plot::PlotResponse>,
    pub fit_requested: bool,
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
fn yield_label(result: &YieldResult, authority: Option<MonteCarloMethod>) -> String {
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
    format!("{count} display bin{plural} · rebuilt from exact samples")
}

/// Draw measurement, display mode and bin controls; true requests a fitted view.
pub fn controls(ui: &mut Ui, settings: &mut HistogramState, names: &[String]) -> bool {
    if !names.is_empty() {
        if settings.selected.is_none() {
            settings.selected = names.first().cloned();
        }
        let selected_text = settings.selected.as_deref().unwrap_or_default().to_owned();
        let changed = ui
            .horizontal_wrapped(|ui| {
                let mut changed = false;
                ui.label("Measure");
                egui::ComboBox::from_id_salt("hist_measurement")
                    .selected_text(&selected_text)
                    .show_ui(ui, |ui| {
                        for name in names {
                            changed |= ui
                                .selectable_value(&mut settings.selected, Some(name.clone()), name)
                                .changed();
                        }
                    });
                ui.label("Display");
                egui::ComboBox::from_id_salt("hist_mode")
                    .selected_text(settings.mode.label())
                    .show_ui(ui, |ui| {
                        for mode in HistogramDisplayMode::ALL {
                            changed |= ui
                                .selectable_value(&mut settings.mode, mode, mode.label())
                                .changed();
                        }
                    });
                ui.label("Bins");
                changed |= ui
                    .add_enabled(
                        settings.mode != HistogramDisplayMode::Cdf,
                        egui::DragValue::new(&mut settings.bin_count).range(1..=1000),
                    )
                    .changed();
                changed
            })
            .inner;
        return changed;
    }
    false
}

/// Draw the qualified distribution without changing host source or selection state.
pub fn show(
    ui: &mut Ui,
    model: Result<HistogramPlot<'_>, HistogramUnavailable>,
    view: PlotView,
    cache: &mut plot::DecimationCache,
) -> HistogramViewResponse {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut out = HistogramViewResponse::default();
    let model = match model {
        Ok(model) => model,
        Err(HistogramUnavailable::NoSelection { has_measurements }) => {
            well_hint(
                ui,
                if has_measurements {
                    "The selected measurement is unavailable. Choose a measurement above."
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
    let display = match model.display {
        Ok(display) => display,
        Err(reason) => {
            well_hint(ui, reason);
            return out;
        }
    };
    let mode = display.mode;
    let moments = model.moments;
    let spec_limits = model.spec_limits;

    let subtitle = format!("{} · {} samples", histogram.name, histogram.total_count);
    let mut legend = vec![LegendChip {
        name: mode.label(),
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
    let header = strip::StripHeader::new("MC", &subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    out.fit_requested = header.fit_clicked;

    let axis = hist_axis(histogram);
    let (x0, x1) = view.x.unwrap_or((axis.x0, axis.x1));
    if !(x1 - x0).is_finite() || x1 <= x0 {
        well_hint(
            ui,
            "The sample range exceeds the linear display range. Set a narrower range in the Distribution panel.",
        );
        return out;
    }
    let y1 = display.y_max();
    let (y0, y1) = view.y.unwrap_or((0.0, y1));

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, ""),
        XScale::Linear,
        Axis::linear_with(y0, y1, mode.unit(), 5),
    )
    .accessible_name("Statistical histogram");
    spec.left_margin = 48.0;

    // Descriptive ±1σ band and mean marker from exact retained sample
    // moments. This is deliberately not called a distribution fit: no fit
    // family or goodness-of-fit evidence is retained by the result schema.
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
    let ordinates = &display.ordinates;
    let accent = c.accent;
    let accent_dim = c.accent_dim;
    let err = c.err;
    cache.ensure_source(model.source);
    if let Some(cdf) = &display.cdf {
        spec.traces
            .push(plot::Trace::new(&cdf.x, &cdf.y, accent).cache_key(0x4849_5354_4344_4600));
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
    }));

    let readout = |x: f64| -> Vec<(String, String)> {
        let value = display.value_at(histogram, x);
        vec![
            ("x".to_owned(), fmt_si(x, "", 3)),
            (
                mode.label().to_owned(),
                if mode == HistogramDisplayMode::Count {
                    format!("{value:.0}")
                } else {
                    fmt_si(value, mode.unit(), 4)
                },
            ),
        ]
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
    let mut range_changed = ui
        .checkbox(&mut settings.custom_range, "Custom range")
        .changed();
    if range_changed
        && settings.custom_range
        && let Some((min, max)) = current_range
        && min < max
    {
        settings.custom_min = min;
        settings.custom_max = max;
    }
    if settings.custom_range {
        range_changed |= ui
            .horizontal_wrapped(|ui| {
                ui.label("Min");
                let min_changed = ui
                    .add(egui::DragValue::new(&mut settings.custom_min))
                    .changed();
                ui.label("Max");
                ui.add(egui::DragValue::new(&mut settings.custom_max))
                    .changed()
                    || min_changed
            })
            .inner;
    }
    range_changed
}

/// Render descriptive and specification evidence already qualified by the host.
pub fn right_panel(ui: &mut Ui, model: Result<HistogramInspector<'_>, HistogramUnavailable>) {
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
                    HistogramUnavailable::Invalid => {
                        "The selected samples, retained statistics, or display range are invalid or unavailable."
                    }
                },
            );
            return;
        }
    };
    let histogram = model.histogram;
    section_header(ui, "Distribution", None);
    if let Some(moments) = model.moments {
        let rows = [
            ("Measure", histogram.name.clone(), false),
            ("Exact samples", moments.count.to_string(), false),
            ("Mean", fmt_si(moments.mean, "", 3), true),
            ("Std dev", fmt_si(moments.std_dev, "", 3), true),
            ("Min", fmt_si(moments.min, "", 3), false),
            ("Max", fmt_si(moments.max, "", 3), false),
            ("Below range", histogram.underflow.to_string(), false),
            ("Above range", histogram.overflow.to_string(), false),
        ];
        crate::presentation::stat_table(ui, &rows);
    } else {
        crate::presentation::stat_table(
            ui,
            &[
                ("Measure", histogram.name.clone(), false),
                (
                    "Exact moments",
                    "Unavailable — retained summary disagrees with samples or moments exceed the finite range".to_owned(),
                    false,
                ),
            ],
        );
    }

    let mc = model.method;
    let mean_confidence = mc.and_then(|authority| authority.mean_confidence);
    if let Some(confidence) = mean_confidence {
        section_header(ui, "Confidence in mean", None);
        let limits = match confidence.interval {
            MonteCarloMeanInterval::Available { lower, upper } => {
                format!("{} to {}", fmt_si(lower, "", 4), fmt_si(upper, "", 4))
            }
            MonteCarloMeanInterval::InsufficientSamples => {
                "Unavailable — fewer than two successful trials".into()
            }
            MonteCarloMeanInterval::Unrepresentable => {
                "Unavailable — limits exceed the finite range".into()
            }
        };
        crate::presentation::stat_table(
            ui,
            &[
                ("Level", format!("{}%", confidence.level_pct), false),
                ("Mean interval", limits, true),
            ],
        );
        crate::presentation::panel_note(
            ui,
            if confidence.conditional_on_successful_trials {
                "Describes successful trials only; failed trials censor the original population. This is not a yield interval."
            } else {
                "Uncertainty in the population mean for independent trials. Student t is exact for normal observations; bootstrap coverage depends on sample size and resampling."
            },
        );
    }

    section_header(ui, "Method authority", None);
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
    crate::presentation::stat_table(
        ui,
        &[
            ("Run completion", completion, false),
            ("Seed", seed, false),
            (
                if mean_confidence.is_some() {
                    "Mean estimator"
                } else {
                    "Estimator"
                },
                mean_confidence.map_or_else(
                    || "Unavailable — method not retained".to_owned(),
                    |confidence| confidence.estimator_label(),
                ),
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

    if let Some(yield_result) = model.yield_result {
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
        crate::presentation::stat_table(ui, &rows);
    } else {
        section_header(ui, "Spec", None);
        crate::presentation::panel_note(
            ui,
            "No unambiguous specification or yield evidence is retained for this measurement.",
        );
    }
    crate::presentation::panel_note(
        ui,
        "The shaded band is descriptive ±1σ only when exact retained moments are available. No distribution fit is inferred.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::yield_analysis::{DistributionStats, YieldSpec};
    fn result(target: &str, yield_percent: f64) -> YieldResult {
        let total_runs = 100;
        let pass_count = yield_percent.round() as usize;
        let mut samples = vec![1.0; pass_count];
        samples.resize(total_runs, 0.0);
        let moments = SampleMoments::from_samples(&samples).expect("finite test samples");
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

        let authority = MonteCarloMethod {
            seed: 7,
            runs_requested: 110,
            runs_completed: 100,
            failures: 10,
            mean_confidence: None,
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
        assert_eq!(
            binning_label(&histogram),
            "2 display bins · rebuilt from exact samples"
        );
    }
}
