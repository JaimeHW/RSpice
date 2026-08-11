//! Harmonic-balance spectrum viewer.
//!
//! HB produces a sparse set of complex Fourier coefficients.  The converted
//! retained result deliberately keeps the magnitude and its original complex
//! components together, so this viewer accepts only those magnitude traces:
//! a phase display, an ordinary AC curve, or a guessed frequency grid must
//! never be presented as harmonic-balance evidence.

use std::sync::Arc;

use egui::Ui;

use crate::state::{AnalysisResult, AnalysisType, SharedWaveformValues, WaveformData};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si, sample_at};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{self, LegendChip};
use super::well_hint;

/// One retained complex coefficient sequence.  `magnitude` is the exact
/// result-conversion magnitude, not a display-derived dB or RMS estimate.
struct HarmonicTrace {
    name: String,
    frequency: SharedWaveformValues,
    magnitude: SharedWaveformValues,
    color: egui::Color32,
    cache_key: u64,
}

/// All evidence needed by both the plot and the inspector.
struct HarmonicBalanceModel {
    label: String,
    traces: Vec<HarmonicTrace>,
    frequency_max: f64,
    magnitude_min: f64,
    magnitude_max: f64,
    retained_frequency_count: usize,
}

/// A waveform is an HB magnitude only when result conversion retained the
/// corresponding complex coefficients.  Name matching is intentionally not
/// sufficient: users may name an unrelated waveform with vertical bars.
pub(super) fn spectrum_trace_is_renderable(waveform: &WaveformData) -> bool {
    waveform.complex.is_some()
        && waveform.x.len() == waveform.y.len()
        && !waveform.x.is_empty()
        && waveform
            .x
            .iter()
            .zip(waveform.y.iter())
            .all(|(&frequency, &magnitude)| {
                frequency.is_finite() && frequency >= 0.0 && magnitude.is_finite()
            })
        && waveform.x.windows(2).all(|window| window[0] <= window[1])
}

/// Whether this exact retained analysis can drive the HB viewer.
pub(super) fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && analysis.analysis_type == AnalysisType::HarmonicBalance
        && analysis.waveforms.iter().any(spectrum_trace_is_renderable)
}

/// Availability helper for the dataset quick view. Persistent documents bind
/// an exact analysis before rendering; the quick-view tab first honors the
/// selected analysis and otherwise resolves the first exact retained HB
/// result in the active immutable dataset.
pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    state
        .simulation
        .active_run()
        .is_some_and(|run| run.analyses.iter().any(analysis_is_renderable))
}

fn selected_hb_analysis(state: &AppState) -> Option<&AnalysisResult> {
    let run = state.simulation.active_run()?;
    state
        .simulation
        .active_analysis_idx
        .and_then(|index| run.analyses.get(index))
        .filter(|analysis| analysis_is_renderable(analysis))
        .or_else(|| {
            run.analyses
                .iter()
                .find(|analysis| analysis_is_renderable(analysis))
        })
}

fn build_model(state: &AppState, tokens: &Tokens) -> Option<HarmonicBalanceModel> {
    let analysis = selected_hb_analysis(state)?;

    let palette = &tokens.color.traces;
    let mut traces = Vec::new();
    let mut frequency_min = f64::INFINITY;
    let mut frequency_max = f64::NEG_INFINITY;
    let mut magnitude_min = f64::INFINITY;
    let mut magnitude_max = f64::NEG_INFINITY;

    for waveform in analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform.visible && spectrum_trace_is_renderable(waveform))
    {
        let (waveform_index, waveform) = waveform;
        for (&frequency, &magnitude) in waveform.x.iter().zip(waveform.y.iter()) {
            frequency_min = frequency_min.min(frequency);
            frequency_max = frequency_max.max(frequency);
            magnitude_min = magnitude_min.min(magnitude);
            magnitude_max = magnitude_max.max(magnitude);
        }
        traces.push(HarmonicTrace {
            name: waveform.complex.as_ref().map_or_else(
                || waveform.name.clone(),
                |complex| complex.source_name.clone(),
            ),
            frequency: Arc::clone(&waveform.x),
            magnitude: Arc::clone(&waveform.y),
            color: palette[traces.len() % palette.len()],
            cache_key: harmonic_trace_cache_key(analysis.id, waveform_index),
        });
    }

    if traces.is_empty()
        || !frequency_min.is_finite()
        || !frequency_max.is_finite()
        || !magnitude_min.is_finite()
        || !magnitude_max.is_finite()
    {
        return None;
    }

    let retained_frequency_count = traces.first().map_or(0, |trace| trace.frequency.len());
    Some(HarmonicBalanceModel {
        label: analysis.label.clone(),
        traces,
        frequency_max,
        magnitude_min,
        magnitude_max,
        retained_frequency_count,
    })
}

const fn harmonic_trace_cache_key(analysis_id: u64, waveform_index: usize) -> u64 {
    0x48B0_0000_0000_0000_u64 ^ analysis_id.rotate_left(23) ^ (waveform_index as u64).rotate_left(7)
}

fn padded_bounds(minimum: f64, maximum: f64) -> Option<(f64, f64)> {
    if !minimum.is_finite() || !maximum.is_finite() || minimum > maximum {
        return None;
    }
    if minimum < maximum {
        let pad = ((maximum - minimum) * 0.08).max(f64::EPSILON);
        return Some((minimum - pad, maximum + pad));
    }
    let pad = (minimum.abs() * 0.08).max(1.0);
    Some((minimum - pad, maximum + pad))
}

fn automatic_x_range(model: &HarmonicBalanceModel) -> Option<(f64, f64)> {
    if !model.frequency_max.is_finite() || model.frequency_max < 0.0 {
        return None;
    }
    if model.frequency_max == 0.0 {
        return Some((0.0, 1.0));
    }
    Some((0.0, model.frequency_max * 1.08))
}

fn automatic_y_range(model: &HarmonicBalanceModel) -> Option<(f64, f64)> {
    // A zero reference is part of a coefficient/stem plot, not a claim that
    // every coefficient is positive.  Negative values can occur in legacy
    // converted data, so keep their exact extent too.
    let lower = model.magnitude_min.min(0.0);
    let upper = model.magnitude_max.max(0.0);
    if lower >= 0.0 {
        return (upper > 0.0).then_some((0.0, upper * 1.08));
    }
    padded_bounds(lower, upper)
}

fn active_hb_failure(state: &AppState) -> Option<&str> {
    let analysis = state.simulation.active_analysis()?;
    (analysis.analysis_type == AnalysisType::HarmonicBalance && !analysis.success).then(|| {
        analysis
            .error_message
            .as_deref()
            .unwrap_or("HB execution failed")
    })
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the selected HB spectrum as coefficient stems plus exact retained
/// traces for cursor/readout interaction.  Plot gestures reuse the Results
/// plot state and therefore support pan, zoom, and the global FIT action.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let tokens = Tokens::get(ui.ctx());
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state, &tokens) else {
        if let Some(error) = active_hb_failure(state) {
            well_hint(ui, &format!("Harmonic-balance execution failed: {error}"));
        } else {
            well_hint(
                ui,
                "No retained harmonic-balance spectrum for the selected analysis",
            );
        }
        return;
    };

    let legend = model
        .traces
        .iter()
        .map(|trace| LegendChip {
            name: &trace.name,
            color: trace.color,
            on: true,
        })
        .collect::<Vec<_>>();
    let view = state
        .ui
        .results
        .plot_view(super::ResultViewer::HarmonicBalance, 0);
    let header = strip::StripHeader::new(
        "HB",
        &format!(
            "{} · {} retained spectral samples",
            model.label, model.retained_frequency_count
        ),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::HarmonicBalance, 0);
    }

    let Some((auto_x0, auto_x1)) = automatic_x_range(&model) else {
        well_hint(ui, "Retained HB frequency axis is degenerate");
        return;
    };
    let Some((auto_y0, auto_y1)) = automatic_y_range(&model) else {
        well_hint(ui, "Retained HB magnitudes are degenerate");
        return;
    };
    let (x0, x1) = view.x.unwrap_or((auto_x0, auto_x1));
    let (y0, y1) = view.y.unwrap_or((auto_y0, auto_y1));
    if !(x0 < x1 && y0 < y1) {
        well_hint(ui, "Retained HB plot range is invalid");
        return;
    }

    let (frequency_scale, frequency_offset, frequency_unit) =
        quantity_policy.frequency_axis_transform();
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Hz").with_display_transform(
            frequency_scale,
            frequency_offset,
            frequency_unit,
        ),
        XScale::Linear,
        Axis::linear_with(y0, y1, "", 7).with_label("magnitude"),
    )
    .accessible_name("Harmonic-balance coefficient spectrum")
    .accessible_detail(
        "Exact retained harmonic-balance magnitude coefficients. Solver tone configuration, harmonic order, convergence iterations, fundamental, and THD are shown only when retained.",
    );
    spec.left_margin = 64.0;
    spec.ref_lines.push(plot::RefLine { y: 0.0 });

    // Retained coefficients are discrete.  The stem underlay makes that
    // fact clear while the thin trace preserves shared cursor/readout and
    // keyboard accessibility behaviour from the Results plot primitive.
    let stems = model
        .traces
        .iter()
        .map(|trace| {
            (
                Arc::clone(&trace.frequency),
                Arc::clone(&trace.magnitude),
                trace.color,
            )
        })
        .collect::<Vec<_>>();
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let baseline = mapper.y(0.0);
        for (frequency, magnitude, color) in &stems {
            for (&x, &y) in frequency.iter().zip(magnitude.iter()) {
                painter.line_segment(
                    [
                        egui::pos2(mapper.x(x), baseline),
                        egui::pos2(mapper.x(x), mapper.y(y)),
                    ],
                    egui::Stroke::new(1.0, *color),
                );
            }
        }
    }));
    for (index, trace) in model.traces.iter().enumerate() {
        spec.traces.push(
            Trace::new(&trace.frequency, &trace.magnitude, trace.color)
                .thin()
                .marker_style(index)
                .cache_key(trace.cache_key),
        );
    }

    let readout = |frequency: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            "f".to_owned(),
            quantity_policy.format_frequency(frequency, 3),
        )];
        for trace in model.traces.iter().take(3) {
            rows.push((
                trace.name.clone(),
                fmt_si(
                    sample_at(&trace.frequency, &trace.magnitude, frequency),
                    "",
                    4,
                ),
            ));
        }
        if model.traces.len() > 3 {
            rows.push((
                "signals".to_owned(),
                format!("+{} more", model.traces.len() - 3),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, &mut state.ui.results.cache, None, Some(&readout));
    super::record_drawn_axes(
        &mut state.ui.results,
        super::ResultViewer::HarmonicBalance,
        &response,
    );
    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::HarmonicBalance, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Retained-HB inspector.  The execution result presently persists spectrum
/// values but not the authored tone plan or iterative solver telemetry; the
/// panel states those absences explicitly instead of reverse-engineering
/// plausible-looking values from the plotted grid.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Harmonic balance", None);
    let tokens = Tokens::get(ui.ctx());
    let Some(model) = build_model(state, &tokens) else {
        if let Some(error) = active_hb_failure(state) {
            super::panel_note(ui, &format!("Harmonic-balance execution failed: {error}"));
        } else {
            super::panel_note(
                ui,
                "Select a completed harmonic-balance result with retained complex coefficients.",
            );
        }
        return;
    };
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let lowest_retained = model
        .traces
        .iter()
        .flat_map(|trace| trace.frequency.iter().copied())
        .filter(|frequency| *frequency > 0.0)
        .min_by(f64::total_cmp);
    let highest_retained = model
        .traces
        .iter()
        .flat_map(|trace| trace.frequency.iter().copied())
        .max_by(f64::total_cmp);

    let retained_samples = format!(
        "{} spectral sample{} · f₀ not retained",
        model.retained_frequency_count,
        if model.retained_frequency_count == 1 {
            ""
        } else {
            "s"
        }
    );
    let rows = [
        ("Tones / f₀", retained_samples, true),
        ("Harmonic order", "Not retained".to_owned(), false),
        (
            "Convergence",
            "Completed · solver iterations not retained".to_owned(),
            false,
        ),
        ("Fundamental", "Not retained".to_owned(), false),
        ("THD", "Not retained".to_owned(), true),
    ];
    super::stat_table(ui, &rows);

    section_header(ui, "Retained spectrum", None);
    let rows = [
        ("Signals", model.traces.len().to_string(), false),
        (
            "Lowest frequency",
            lowest_retained.map_or_else(
                || "DC only".to_owned(),
                |frequency| quantity_policy.format_frequency(frequency, 3),
            ),
            false,
        ),
        (
            "Highest frequency",
            highest_retained.map_or("—".to_owned(), |frequency| {
                quantity_policy.format_frequency(frequency, 3)
            }),
            false,
        ),
        (
            "Magnitude range",
            format!(
                "{} … {}",
                fmt_si(model.magnitude_min, "", 4),
                fmt_si(model.magnitude_max, "", 4)
            ),
            false,
        ),
    ];
    super::stat_table(ui, &rows);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn spectrum_waveform() -> WaveformData {
        WaveformData::new(
            "|V(out)|",
            Arc::new(vec![0.0, 1.0e6, 2.0e6]),
            Arc::new(vec![0.1, 1.0, 0.2]),
            "#ffffff",
        )
        .with_complex_components(
            "V(out)",
            Arc::new(vec![0.1, 1.0, 0.2]),
            Arc::new(vec![0.0, 0.0, 0.0]),
        )
    }

    #[test]
    fn hb_viewer_accepts_only_complex_retained_coefficients() {
        let result = AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB")
            .with_waveforms(vec![spectrum_waveform()]);
        assert!(analysis_is_renderable(&result));

        let phase_only = AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB")
            .with_waveforms(vec![WaveformData::new(
                "phase(V(out))",
                Arc::new(vec![0.0, 1.0e6]),
                Arc::new(vec![0.0, 45.0]),
                "#ffffff",
            )]);
        assert!(!analysis_is_renderable(&phase_only));
    }

    #[test]
    fn hb_viewer_rejects_malformed_or_wrong_family_data() {
        let malformed = WaveformData::new(
            "|V(out)|",
            Arc::new(vec![1.0e6, 0.5e6]),
            Arc::new(vec![1.0, 0.5]),
            "#ffffff",
        )
        .with_complex_components(
            "V(out)",
            Arc::new(vec![1.0, 0.5]),
            Arc::new(vec![0.0, 0.0]),
        );
        assert!(!spectrum_trace_is_renderable(&malformed));

        let wrong_family = AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![spectrum_waveform()]);
        assert!(!analysis_is_renderable(&wrong_family));
    }

    #[test]
    fn padded_bounds_keep_single_retained_frequency_plotable() {
        let (minimum, maximum) = padded_bounds(1.0e6, 1.0e6).expect("finite singleton");
        assert!(minimum < 1.0e6 && maximum > 1.0e6);
        assert!(padded_bounds(f64::NAN, 1.0).is_none());
    }

    #[test]
    fn trace_cache_identity_survives_visibility_filtering() {
        assert_ne!(
            harmonic_trace_cache_key(7, 0),
            harmonic_trace_cache_key(7, 1)
        );
        assert_ne!(
            harmonic_trace_cache_key(7, 0),
            harmonic_trace_cache_key(8, 0)
        );
    }
}
