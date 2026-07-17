//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::common::AppState;
use crate::state::{
    AnalysisResult, AnalysisType, SharedWaveformValues, SimulationRun, SimulationState,
};
use crate::ui::plot::{
    self, Axis, CursorPair, DisplayDecimation, PlotSpec, SampleInterpolation, Trace, XScale,
    fmt_si_significant, fmt_significant, sample_at_with,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;
use crate::workbench::visualization_family::SourceSampleSelection;
use crate::workbench::{
    ComplexNumberDisplay, CursorInterpolation, LargeDatasetDisplay, ResultPresentationPolicy,
};

use super::strip::{LegendChip, StripHeader};
use super::{
    DerivedSeries, ExprEditor, ExprSeries, ExprTrace, ResultsState, WaveformSeriesResult,
    waveform_color, well_hint,
};

/// How a trace's Y values are interpreted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TraceKind {
    /// Plain values (V, A, sweep output).
    Value,
    /// dB-converted AC magnitude.
    MagnitudeDb,
    /// Phase in degrees (right axis, dashed).
    PhaseDeg,
    /// Phase in radians (right axis, dashed).
    PhaseRad,
    /// Original real component of a complex source quantity.
    Real,
    /// Original imaginary component of a complex source quantity.
    Imaginary,
}

impl TraceKind {
    const fn is_phase(self) -> bool {
        matches!(self, Self::PhaseDeg | Self::PhaseRad)
    }
}

/// One trace of a strip, with owned `Arc` handles into the run data.
struct StripTrace {
    waveform_index: usize,
    /// Name of the source waveform in the immutable dataset. Display names
    /// may be derived representations such as `re(V(out))`.
    source_waveform_name: String,
    name: String,
    color: egui::Color32,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
    kind: TraceKind,
    visible: bool,
    /// The run this trace belongs to (cache-key discriminator).
    run_id: u64,
    /// Overlay traces come from a non-active run: same signal hue, reduced
    /// weight, visibility slaved to the active run's matching signal.
    overlay: bool,
}

/// One strip (== one analysis of the active run).
pub(super) struct StripModel {
    analysis_index: usize,
    analysis_type: AnalysisType,
    kind_tag: String,
    subtitle: String,
    x_scale: XScale,
    x_unit: &'static str,
    y_unit: &'static str,
    /// Phase traces carry the unwrapped (continuous) series instead of the
    /// raw ±180°-wrapped samples. Folded into the cache keys.
    phase_continuous: bool,
    /// Number of active-run traces at the front of `traces`; everything
    /// after is overlay. The legend lists only this prefix (signal owns
    /// hue — one chip per signal, all runs).
    signal_trace_count: usize,
    traces: Vec<StripTrace>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct CursorDomain {
    analysis_type: AnalysisType,
    x_scale: XScale,
    x_unit: &'static str,
}

/// Frame cache for the strip models. Building them clones every trace name
/// and walks all overlay runs, and both the center view and the right panel
/// ask for them each frame — the fingerprint covers everything the models
/// read, so the rebuild only happens when an input actually changes.
#[derive(Default, Clone)]
pub(super) struct ModelsCache(Option<(u64, Arc<Vec<StripModel>>)>);

impl std::fmt::Debug for ModelsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelsCache(..)")
    }
}

/// Everything `build_models` reads: run data version, the display-run set,
/// per-trace visibility and stored color, phase mode, and the theme palette.
fn models_fingerprint(
    simulation: &SimulationState,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    t: &Tokens,
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    simulation.data_version.hash(&mut h);
    phase_continuous.hash(&mut h);
    complex_display.hash(&mut h);
    selection
        .map(SourceSampleSelection::fingerprint)
        .hash(&mut h);
    for color in &t.color.traces {
        color.to_array().hash(&mut h);
    }
    for run in simulation.display_runs() {
        run.id.hash(&mut h);
        for analysis in &run.analyses {
            analysis.analysis_type.hash(&mut h);
            analysis.label.hash(&mut h);
            analysis
                .provenance
                .as_ref()
                .map(|provenance| provenance.source_instance_id())
                .hash(&mut h);
            analysis.waveforms.len().hash(&mut h);
            for waveform in &analysis.waveforms {
                waveform.visible.hash(&mut h);
                waveform.color.hash(&mut h);
            }
        }
    }
    h.finish()
}

/// Fingerprint-cached [`build_models`]; the returned handle is cheap to
/// clone and stays valid across later state borrows.
pub(super) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    let fp = models_fingerprint(
        simulation,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        t,
    );
    if let Some((cached_fp, models)) = &results.models.0
        && *cached_fp == fp
    {
        return Arc::clone(models);
    }
    let models = Arc::new(build_models(
        simulation,
        &mut results.derived,
        t,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
    ));
    results.models.0 = Some((fp, Arc::clone(&models)));
    models
}

/// Fold a run identity into a cache key for overlay traces. Active-run
/// keys stay unchanged so existing envelopes/ranges remain warm.
fn run_mixed_key(base: u64, run_id: u64, overlay: bool) -> u64 {
    if overlay {
        base ^ run_id.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    } else {
        base
    }
}

fn unique_analysis<'a>(
    mut candidates: impl Iterator<Item = &'a AnalysisResult>,
) -> Option<&'a AnalysisResult> {
    let candidate = candidates.next()?;
    candidates.next().is_none().then_some(candidate)
}

/// Resolve the result in an overlay run that was produced by the same
/// prepared analysis instance. Kind/label inference is permitted only for
/// unambiguous legacy history, where neither side has prepared provenance.
fn matching_overlay_analysis<'a>(
    analysis: &AnalysisResult,
    overlay_run: &'a SimulationRun,
) -> Option<&'a AnalysisResult> {
    if let Some(source_instance_id) = analysis
        .provenance
        .as_ref()
        .map(|provenance| provenance.source_instance_id())
    {
        return overlay_run
            .find_analysis_by_source_instance(source_instance_id)
            .filter(|candidate| candidate.analysis_type == analysis.analysis_type);
    }

    let legacy_candidates = || {
        overlay_run.analyses.iter().filter(|candidate| {
            candidate.provenance.is_none() && candidate.analysis_type == analysis.analysis_type
        })
    };
    unique_analysis(legacy_candidates().filter(|candidate| candidate.label == analysis.label))
        .or_else(|| unique_analysis(legacy_candidates()))
}

impl StripModel {
    const fn cursor_domain(&self) -> CursorDomain {
        CursorDomain {
            analysis_type: self.analysis_type,
            x_scale: self.x_scale,
            x_unit: self.x_unit,
        }
    }

    fn x_label(&self) -> &'static str {
        match self.x_scale {
            XScale::Log10 => "f",
            XScale::Linear => {
                if self.x_unit == "s" {
                    "t"
                } else {
                    "x"
                }
            }
        }
    }

    fn format_x(
        &self,
        x: f64,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> String {
        if self.x_unit == "Hz" {
            return quantity_policy.format_frequency(x, significant_digits);
        }
        fmt_si_significant(
            x,
            if self.x_unit.is_empty() {
                ""
            } else {
                self.x_unit
            },
            significant_digits,
        )
    }

    fn format_trace_value(
        &self,
        trace: &StripTrace,
        value: f64,
        significant_digits: usize,
        quantity_policy: crate::quantity::QuantityPresentationPolicy,
    ) -> String {
        match trace.kind {
            TraceKind::Value => fmt_si_significant(value, self.y_unit, significant_digits),
            TraceKind::MagnitudeDb => fmt_significant(value, significant_digits, " dB"),
            TraceKind::PhaseDeg => {
                quantity_policy.format_angle(value.to_radians(), significant_digits)
            }
            TraceKind::PhaseRad => quantity_policy.format_angle(value, significant_digits),
            TraceKind::Real | TraceKind::Imaginary => {
                fmt_si_significant(value, self.y_unit, significant_digits)
            }
        }
    }
}

fn selected_series_pair(
    x: &SharedWaveformValues,
    y: &SharedWaveformValues,
    selection: Option<&SourceSampleSelection>,
) -> Option<(SharedWaveformValues, SharedWaveformValues)> {
    let Some(selection) = selection else {
        return Some((Arc::clone(x), Arc::clone(y)));
    };
    if x.len() != y.len()
        || selection
            .source_indices
            .last()
            .is_some_and(|index| *index >= x.len())
    {
        return None;
    }
    let selected_x = selection
        .source_indices
        .iter()
        .map(|index| x[*index])
        .collect();
    let selected_y = selection
        .source_indices
        .iter()
        .map(|index| y[*index])
        .collect();
    Some((Arc::new(selected_x), Arc::new(selected_y)))
}

/// Build strip models for every plottable analysis of the active run.
/// `phase_continuous` swaps phase traces to their unwrapped series.
pub(super) fn build_models(
    simulation: &SimulationState,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
) -> Vec<StripModel> {
    let display_runs = simulation.display_runs();
    let Some((&run, overlay_runs)) = display_runs.split_first() else {
        return Vec::new();
    };
    let mut models = Vec::new();

    for (analysis_index, analysis) in run.analyses.iter().enumerate() {
        if analysis.waveforms.is_empty() {
            continue;
        }
        let displays_cartesian_complex = analysis.analysis_type == AnalysisType::Ac
            && complex_display == ComplexNumberDisplay::RealImaginary
            && analysis
                .waveforms
                .iter()
                .any(|waveform| waveform.complex.is_some());
        let (x_scale, x_unit, y_unit) = match analysis.analysis_type {
            AnalysisType::Ac if displays_cartesian_complex => (XScale::Log10, "Hz", ""),
            AnalysisType::Ac => (XScale::Log10, "Hz", "dB"),
            AnalysisType::Noise | AnalysisType::Pnoise => (XScale::Log10, "Hz", "V^2/Hz"),
            AnalysisType::Transient => (XScale::Linear, "s", "V"),
            AnalysisType::DcSweep => (XScale::Linear, "V", "V"),
            _ => (XScale::Linear, "", "V"),
        };

        let mut traces = Vec::new();
        let sample_selection = selection.filter(|selection| {
            selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
        });
        let selection_key = sample_selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(17);
        for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
            let Some((source_x, source_y)) =
                selected_series_pair(&waveform.x, &waveform.y, sample_selection)
            else {
                continue;
            };
            let color = waveform_color(waveform, waveform_index, tokens);
            let is_phase = waveform.name.starts_with("phase(");
            let is_mag = waveform.name.starts_with('|');
            if displays_cartesian_complex {
                if let Some(complex) = &waveform.complex {
                    let Some((_, source_real)) =
                        selected_series_pair(&waveform.x, &complex.real, sample_selection)
                    else {
                        continue;
                    };
                    let Some((_, source_imaginary)) =
                        selected_series_pair(&waveform.x, &complex.imag, sample_selection)
                    else {
                        continue;
                    };
                    for (kind, name, color, y) in [
                        (
                            TraceKind::Real,
                            format!("re({})", complex.source_name),
                            color,
                            source_real,
                        ),
                        (
                            TraceKind::Imaginary,
                            format!("im({})", complex.source_name),
                            tokens.color.traces[(waveform_index + 1) % tokens.color.traces.len()],
                            source_imaginary,
                        ),
                    ] {
                        traces.push(StripTrace {
                            waveform_index,
                            source_waveform_name: waveform.name.clone(),
                            name,
                            color,
                            x: Arc::clone(&source_x),
                            y,
                            kind,
                            visible: waveform.visible,
                            run_id: run.id,
                            overlay: false,
                        });
                    }
                    continue;
                }
                if is_phase
                    && analysis.waveforms.iter().any(|candidate| {
                        candidate.complex.as_ref().is_some_and(|complex| {
                            waveform.name == format!("phase({})", complex.source_name)
                        })
                    })
                {
                    continue;
                }
            }
            let kind = if analysis.analysis_type == AnalysisType::Ac && is_phase {
                match complex_display {
                    ComplexNumberDisplay::MagnitudePhaseRadians => TraceKind::PhaseRad,
                    _ => TraceKind::PhaseDeg,
                }
            } else if analysis.analysis_type == AnalysisType::Ac && is_mag {
                TraceKind::MagnitudeDb
            } else {
                TraceKind::Value
            };
            let y = match kind {
                TraceKind::MagnitudeDb => derived.db(
                    ((analysis_index as u64) << 32 | waveform_index as u64) ^ selection_key,
                    &source_y,
                ),
                // Continuous phase display: cached unwrapped copy of the
                // wrapped samples, same key convention as `db`.
                TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                    derived,
                    ((analysis_index as u64) << 32 | waveform_index as u64) ^ selection_key,
                    &source_y,
                    phase_continuous,
                    kind == TraceKind::PhaseRad,
                ),
                _ => source_y,
            };
            traces.push(StripTrace {
                waveform_index,
                source_waveform_name: waveform.name.clone(),
                name: waveform.name.clone(),
                color,
                x: source_x,
                y,
                kind,
                visible: waveform.visible,
                run_id: run.id,
                overlay: false,
            });
        }
        let signal_trace_count = traces.len();

        // Overlay runs: match the exact prepared analysis instance and merge
        // traces by signal name. Signal owns hue —
        // overlay traces reuse the active trace's color and visibility —
        // run owns weight (applied at draw time).
        let mut overlaid_run_count = 0usize;
        for overlay_run in overlay_runs {
            let overlay_analysis = matching_overlay_analysis(analysis, overlay_run);
            let Some(overlay_analysis) = overlay_analysis else {
                continue;
            };

            let mut contributed = false;
            for signal_index in 0..signal_trace_count {
                let (source_name, signal_name, signal_color, signal_kind, signal_visible) = {
                    let signal = &traces[signal_index];
                    (
                        signal.source_waveform_name.clone(),
                        signal.name.clone(),
                        signal.color,
                        signal.kind,
                        signal.visible,
                    )
                };
                let Some((overlay_index, overlay_waveform)) = overlay_analysis
                    .waveforms
                    .iter()
                    .enumerate()
                    .find(|(_, waveform)| waveform.name == source_name)
                else {
                    continue;
                };

                let base_key = (analysis_index as u64) << 32 | overlay_index as u64;
                let derived_key = run_mixed_key(base_key, overlay_run.id, true);
                let y = match signal_kind {
                    TraceKind::MagnitudeDb => derived.db(derived_key, &overlay_waveform.y),
                    TraceKind::PhaseDeg | TraceKind::PhaseRad => displayed_phase_series(
                        derived,
                        derived_key,
                        &overlay_waveform.y,
                        phase_continuous,
                        signal_kind == TraceKind::PhaseRad,
                    ),
                    TraceKind::Real => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.real)
                    }
                    TraceKind::Imaginary => {
                        let Some(complex) = &overlay_waveform.complex else {
                            continue;
                        };
                        Arc::clone(&complex.imag)
                    }
                    _ => Arc::clone(&overlay_waveform.y),
                };
                traces.push(StripTrace {
                    waveform_index: overlay_index,
                    source_waveform_name: source_name,
                    name: signal_name,
                    color: signal_color,
                    x: Arc::clone(&overlay_waveform.x),
                    y,
                    kind: signal_kind,
                    visible: signal_visible,
                    run_id: overlay_run.id,
                    overlay: true,
                });
                contributed = true;
            }
            if contributed {
                overlaid_run_count += 1;
            }
        }

        let mut subtitle = analysis.label.clone();
        if overlaid_run_count > 0 {
            subtitle = format!(
                "{subtitle} · +{overlaid_run_count} run{} overlaid",
                if overlaid_run_count == 1 { "" } else { "s" }
            );
        }

        models.push(StripModel {
            analysis_index,
            analysis_type: analysis.analysis_type,
            kind_tag: analysis.analysis_type.short_label().to_uppercase(),
            subtitle,
            x_scale,
            x_unit,
            y_unit,
            phase_continuous,
            signal_trace_count,
            traces,
        });
    }
    models
}

fn displayed_phase_series(
    derived: &mut DerivedSeries,
    key: u64,
    phase_degrees: &SharedWaveformValues,
    continuous: bool,
    radians: bool,
) -> SharedWaveformValues {
    let degrees = if continuous {
        derived.unwrapped(key, phase_degrees)
    } else {
        Arc::clone(phase_degrees)
    };
    if !radians {
        return degrees;
    }
    // Radian conversion is a cached presentation series. Stored samples stay
    // in their original degree representation for reproducibility/export.
    const RADIANS_KEY_BIT: u64 = 1 << 61;
    const CONTINUOUS_KEY_BIT: u64 = 1 << 60;
    derived.get_or(
        key ^ RADIANS_KEY_BIT ^ if continuous { CONTINUOUS_KEY_BIT } else { 0 },
        || Arc::new(degrees.iter().map(|value| value.to_radians()).collect()),
    )
}

/// Stable per-trace identity shared by the decimation, range, and
/// measurement caches. Phase traces fold in the wrapped/continuous choice
/// so a toggle never serves stale envelopes, ranges, or stats.
fn trace_key(model: &StripModel, trace: &StripTrace) -> u64 {
    let continuous = (trace.kind.is_phase() && model.phase_continuous) as u64;
    let base = (model.analysis_index as u64) << 44
        | continuous << 43
        | (trace.waveform_index as u64) << 3
        | trace.kind as u64;
    run_mixed_key(base, trace.run_id, trace.overlay)
}

/// Y range of the visible traces on one axis side, padded 8 %. Per-trace
/// extremes are cached on the data version — never rescanned per frame.
fn y_range(derived: &mut DerivedSeries, model: &StripModel, phase: bool) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for trace in &model.traces {
        let is_phase = trace.kind.is_phase();
        if is_phase != phase || !trace.visible {
            continue;
        }
        let extremes =
            derived.range_or(trace_key(model, trace), || super::finite_extremes(&trace.y));
        if let Some((lo, hi)) = extremes {
            min = min.min(lo);
            max = max.max(hi);
        }
    }
    if !min.is_finite() {
        return None;
    }
    if min == max {
        return Some((min - 1.0, max + 1.0));
    }
    let pad = (max - min) * 0.08;
    Some((min - pad, max + pad))
}

/// X range of a strip (shared X across its traces).
fn x_range(model: &StripModel) -> Option<(f64, f64)> {
    let x = &model.traces.first()?.x;
    if x.is_empty() {
        return None;
    }
    let (mut x0, x1) = (x[0], x[x.len() - 1]);
    if model.x_scale == XScale::Log10 {
        if x1 <= 0.0 {
            return None;
        }
        if x0 <= 0.0 {
            x0 = x.iter().copied().find(|&v| v > 0.0)?;
        }
    }
    (x1 > x0).then_some((x0, x1))
}

const fn cursor_interpolation(policy: CursorInterpolation) -> SampleInterpolation {
    match policy {
        CursorInterpolation::MonotoneCubicWhereValid => SampleInterpolation::MonotoneCubic,
        CursorInterpolation::Linear => SampleInterpolation::Linear,
        CursorInterpolation::NearestAcceptedPoint => SampleInterpolation::Nearest,
    }
}

const fn display_decimation(policy: LargeDatasetDisplay) -> DisplayDecimation {
    match policy {
        LargeDatasetDisplay::EnvelopeExtrema => DisplayDecimation::EnvelopeExtrema,
        LargeDatasetDisplay::UniformDisplaySampling => DisplayDecimation::Uniform,
        LargeDatasetDisplay::NoDisplayDecimation => DisplayDecimation::FullResolution,
    }
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the strip stack.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    if models.is_empty() {
        let hint = if state.simulation.active_run().is_none() {
            let shortcut = state.ui.preferences.shortcuts().resolved_label(
                crate::workbench::commands::Command::RunSimulation,
                crate::common::app::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            if shortcut.is_empty() {
                "No results yet — use the Run button to simulate".to_owned()
            } else {
                format!("No results yet — press {shortcut} or use the Run button to simulate")
            }
        } else {
            "The active run has no plottable analyses".to_owned()
        };
        well_hint(ui, &hint);
        return;
    }

    // Apply hide/maximize strip state.
    let results = &state.ui.results;
    let visible: Vec<&StripModel> = match results.maximized_strip {
        Some(max_idx) if models.iter().any(|m| m.analysis_index == max_idx) => models
            .iter()
            .filter(|m| m.analysis_index == max_idx)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_index))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(ui, "All strips hidden — restore them from the document bar");
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_trace: Option<(usize, usize)> = None;
    let mut toggle_maximize: Option<usize> = None;
    let mut close_strip: Option<usize> = None;
    let mut fit_strip: Option<usize> = None;
    let mut toggle_expr: Option<(usize, usize)> = None;
    let mut remove_expr: Option<(usize, usize)> = None;
    let mut open_editor: Option<usize> = None;

    let avail = ui.available_rect_before_wrap();
    let n = visible.len();
    let separators = (n.saturating_sub(1)) as f32;
    let strip_height = ((avail.height() - separators) / n as f32).max(140.0);
    let maximized = state.ui.results.maximized_strip.is_some();
    let linked_cursor_domain = state
        .ui
        .results
        .cursor_strip
        .and_then(|owner| models.iter().find(|model| model.analysis_index == owner))
        .map(|model| model.cursor_domain());

    egui::ScrollArea::vertical()
        .id_salt("rspice.results.strips")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            for (position, model) in visible.iter().enumerate() {
                if position > 0 {
                    // 1 px border seam between strips.
                    let (seam, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 1.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(seam, 0.0, t.color.border);
                }
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), strip_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(strip_height);
                        // Legend chips list signals only (the active-run
                        // prefix): one chip per signal toggles it across
                        // every overlaid run.
                        let legend: Vec<LegendChip<'_>> = model
                            .traces
                            .iter()
                            .take(model.signal_trace_count)
                            .map(|trace| LegendChip {
                                name: &trace.name,
                                color: trace.color,
                                on: trace.visible,
                            })
                            .collect();
                        // Expression chips follow the waveform chips; long
                        // expressions are elided so a chip never eats the row.
                        let strip_exprs: Vec<ExprTrace> = state
                            .ui
                            .results
                            .exprs
                            .get(&model.analysis_index)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> =
                            strip_exprs.iter().map(|e| elide(&e.text, 24)).collect();
                        let mut legend = legend;
                        for (i, expr) in strip_exprs.iter().enumerate() {
                            legend.push(LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, model.signal_trace_count + i),
                                on: expr.visible,
                            });
                        }

                        let zoomed = state
                            .ui
                            .results
                            .plot_view(super::ResultViewer::Waves, model.analysis_index)
                            .is_zoomed();
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .maximized(maximized)
                            .closable(!maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(true)
                            .removable_from(model.signal_trace_count)
                            .show(ui);
                        if let Some(chip_index) = header.legend_clicked {
                            if chip_index < model.signal_trace_count {
                                if let Some(trace) = model.traces.get(chip_index) {
                                    toggle_trace =
                                        Some((model.analysis_index, trace.waveform_index));
                                }
                            } else {
                                toggle_expr = Some((
                                    model.analysis_index,
                                    chip_index - model.signal_trace_count,
                                ));
                            }
                        }
                        if let Some(chip_index) = header.legend_removed
                            && chip_index >= model.signal_trace_count
                        {
                            remove_expr =
                                Some((model.analysis_index, chip_index - model.signal_trace_count));
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_index);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_index);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_index);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_index);
                        }

                        expr_editor_row(ui, state, model.analysis_index);

                        // Strips scrolled out of view skip the plot body
                        // entirely (range lookups, envelope mapping, shape
                        // building) — only the space is reserved.
                        let plot_rect = ui.available_rect_before_wrap();
                        if ui.is_rect_visible(plot_rect) {
                            show_strip_plot(ui, state, model, linked_cursor_domain);
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    if let Some((analysis_index, waveform_index)) = toggle_trace {
        toggle_visibility(state, analysis_index, waveform_index);
    }
    let results = &mut state.ui.results;
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if results.cursor_strip == Some(idx) {
            results.clear_cursors();
        }
    }
    if let Some(idx) = fit_strip {
        results.reset_plot_view(super::ResultViewer::Waves, idx);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = results.exprs.get_mut(&analysis)
    {
        if index < list.len() {
            let removed = list.remove(index);
            results.expr_cache.remove(&(analysis, removed.text));
        }
        if list.is_empty() {
            results.exprs.remove(&analysis);
        }
    }
    if let Some(analysis) = open_editor {
        results.expr_editor = Some(ExprEditor {
            analysis_index: analysis,
            text: String::new(),
            error: None,
            want_focus: true,
        });
    }
}

/// Shorten a label to `max` characters with a typographic ellipsis.
fn elide(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// Palette color for the i-th trace slot of a strip (waveforms, then
/// expressions).
fn expr_color(tokens: &Tokens, slot: usize) -> egui::Color32 {
    tokens.color.traces[slot % tokens.color.traces.len()]
}

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, error inline.
fn expr_editor_row(ui: &mut Ui, state: &mut AppState, analysis_index: usize) {
    let Some(editor) = state
        .ui
        .results
        .expr_editor
        .as_mut()
        .filter(|editor| editor.analysis_index == analysis_index)
    else {
        return;
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    enum Action {
        None,
        Commit,
        Cancel,
    }
    let mut action = Action::None;

    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(egui::vec2(10.0, 5.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let row = &mut child;
    row.spacing_mut().item_spacing.x = 8.0;

    row.label(
        egui::RichText::new("expr")
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(c.text_dim),
    );

    let response = row.add(
        egui::TextEdit::singleline(&mut editor.text)
            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
            .hint_text("V(out)/V(in) · dB(V(out)) · deriv(V(out))")
            .desired_width(340.0),
    );
    if editor.want_focus {
        response.request_focus();
        editor.want_focus = false;
    }
    if response.lost_focus() && row.input(|i| i.key_pressed(egui::Key::Enter)) {
        action = Action::Commit;
    }
    if row.input(|i| i.key_pressed(egui::Key::Escape)) {
        action = Action::Cancel;
    }
    if crate::ui::widgets::Button::new("Add").show(row).clicked() {
        action = Action::Commit;
    }
    if let Some(error) = &editor.error {
        row.label(
            egui::RichText::new(elide(error, 64))
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }

    match action {
        Action::None => {}
        Action::Cancel => state.ui.results.expr_editor = None,
        Action::Commit => {
            let text = state
                .ui
                .results
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.ui.results.expr_editor = None;
                return;
            }
            let sample_selection = state.ui.results.sample_selection.clone();
            let (series, extremes) = evaluate_expression(
                &state.simulation,
                analysis_index,
                &text,
                sample_selection.as_ref(),
            );
            match series {
                Ok(series) => {
                    state.ui.results.expr_cache.insert(
                        (analysis_index, text.clone()),
                        ExprSeries {
                            version: expression_version(
                                state.simulation.data_version,
                                sample_selection.as_ref(),
                            ),
                            series: Ok(series),
                            y_extremes: extremes,
                        },
                    );
                    state
                        .ui
                        .results
                        .exprs
                        .entry(analysis_index)
                        .or_default()
                        .push(ExprTrace {
                            text,
                            visible: true,
                        });
                    state.ui.results.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.ui.results.expr_editor.as_mut() {
                        editor.error = Some(error);
                        editor.want_focus = true;
                    }
                }
            }
        }
    }
}

type ExpressionEvaluation = (WaveformSeriesResult, Option<(f64, f64)>);

/// Evaluate one expression against an analysis' waveforms. Scalars become a
/// constant trace across the analysis' x span.
fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionEvaluation {
    let Some(run) = simulation.active_run() else {
        return (Err("analysis no longer exists".to_owned()), None);
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return (Err("analysis no longer exists".to_owned()), None);
    };
    let selection = selection.filter(|selection| {
        selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
    });

    let ctx = calculator::WaveformsContext::new(&analysis.waveforms);
    let expr = match calculator::parser::try_parse(text) {
        Ok(expr) => expr,
        Err(error) => return (Err(format!("parse error: {error}")), None),
    };
    match calculator::evaluator::evaluate(&expr, &ctx) {
        Ok(calculator::CalcValue::Waveform(x, y)) if !x.is_empty() => {
            let (x, y) =
                match selection {
                    None => (x, y),
                    Some(selection)
                        if x.len() == y.len()
                            && selection
                                .source_indices
                                .last()
                                .is_none_or(|index| *index < x.len()) =>
                    {
                        (
                            selection
                                .source_indices
                                .iter()
                                .map(|index| x[*index])
                                .collect(),
                            selection
                                .source_indices
                                .iter()
                                .map(|index| y[*index])
                                .collect(),
                        )
                    }
                    Some(_) => {
                        return (
                        Err("expression sample count does not match the retained family manifest"
                            .to_owned()),
                        None,
                    );
                    }
                };
            let extremes = super::finite_extremes(&y);
            (Ok((x.into(), y.into())), extremes)
        }
        Ok(calculator::CalcValue::Waveform(..)) => {
            (Err("expression produced no samples".to_owned()), None)
        }
        Ok(calculator::CalcValue::Scalar(value)) => {
            let span = analysis.waveforms.first().and_then(|waveform| {
                let (x, _) = selected_series_pair(&waveform.x, &waveform.y, selection)?;
                (x.len() >= 2).then(|| (x[0], x[x.len() - 1]))
            });
            match span {
                Some((x0, x1)) => (
                    Ok((vec![x0, x1].into(), vec![value, value].into())),
                    Some((value, value)),
                ),
                None => (Err("scalar result with no x span".to_owned()), None),
            }
        }
        Err(error) => (Err(error.to_string()), None),
    }
}

fn expression_version(data_version: u64, selection: Option<&SourceSampleSelection>) -> u64 {
    data_version
        ^ selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(23)
}

/// One expression trace resolved for plotting.
struct ResolvedExpr {
    x: SharedWaveformValues,
    y: SharedWaveformValues,
    color: egui::Color32,
    cache_key: u64,
    label: String,
    y_extremes: Option<(f64, f64)>,
}

/// Refresh the expression cache for a strip at the current data version and
/// hand back plottable series (visible expressions, successful evaluations).
fn resolve_strip_exprs(
    state: &mut AppState,
    model: &StripModel,
    tokens: &Tokens,
) -> Vec<ResolvedExpr> {
    let exprs: Vec<(usize, ExprTrace)> = state
        .ui
        .results
        .exprs
        .get(&model.analysis_index)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let sample_selection = state.ui.results.sample_selection.clone();
    let version = expression_version(state.simulation.data_version, sample_selection.as_ref());
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let key = (model.analysis_index, expr.text.clone());
        let fresh = state
            .ui
            .results
            .expr_cache
            .get(&key)
            .is_some_and(|s| s.version == version);
        if !fresh {
            let (series, extremes) = evaluate_expression(
                &state.simulation,
                model.analysis_index,
                &expr.text,
                sample_selection.as_ref(),
            );
            if let Err(error) = &series {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                    "expression `{}`: {}",
                    expr.text, error
                )));
            }
            state.ui.results.expr_cache.insert(
                key.clone(),
                ExprSeries {
                    version,
                    series,
                    y_extremes: extremes,
                },
            );
        }
        if !expr.visible {
            continue;
        }
        if let Some(ExprSeries {
            series: Ok((x, y)),
            y_extremes,
            ..
        }) = state.ui.results.expr_cache.get(&key)
        {
            resolved.push(ResolvedExpr {
                x: Arc::clone(x),
                y: Arc::clone(y),
                color: expr_color(tokens, model.traces.len() + slot),
                cache_key: expr_cache_key(model.analysis_index, &expr.text),
                label: elide(&expr.text, 24),
                y_extremes: *y_extremes,
            });
        }
    }
    resolved
}

/// Stable decimation-cache identity for an expression trace. The high bit
/// keeps it out of the waveform trace_key space.
fn expr_cache_key(analysis_index: usize, text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (analysis_index, text).hash(&mut hasher);
    hasher.finish() | (1 << 63)
}

/// Flip a waveform's visibility on the run, keeping the live copy in sync.
pub(crate) fn toggle_visibility(
    state: &mut AppState,
    analysis_index: usize,
    waveform_index: usize,
) {
    let Some(run_idx) = state.simulation.active_run_idx else {
        return;
    };
    let mut name: Option<String> = None;
    if let Some(waveform) = state
        .simulation
        .runs
        .get_mut(run_idx)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
    {
        waveform.visible = !waveform.visible;
        name = Some(waveform.name.clone());
    }
    // Mirror into the live waveform list when this is the active analysis.
    if state.simulation.active_analysis_idx == Some(analysis_index)
        && let Some(name) = name
        && let Some(live) = state
            .simulation
            .waveforms
            .iter_mut()
            .find(|w| w.name == name)
    {
        live.visible = !live.visible;
    }
}

/// Serialize the active Waves cursor readout for the platform clipboard.
/// This is the Edit → Copy consumer for the Units copied-value policy.
pub(crate) fn copy_cursor_text(state: &mut AppState) -> Option<String> {
    let x = state.ui.results.cursors.a?;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let sample_selection = state.ui.results.sample_selection.clone();
    let models = build_models(
        &state.simulation,
        &mut state.ui.results.derived,
        &Tokens::default(),
        state.ui.results.phase_continuous,
        presentation.complex_number_display(),
        sample_selection.as_ref(),
    );
    let model = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;

    let mut text = String::new();
    append_copied_cursor(&mut text, "A", x, model, interpolation, quantity_policy);
    if let Some(b) = state.ui.results.cursors.b {
        text.push('\n');
        append_copied_cursor(&mut text, "B", b, model, interpolation, quantity_policy);
    }
    Some(text)
}

fn append_copied_cursor(
    target: &mut String,
    cursor: &str,
    x: f64,
    model: &StripModel,
    interpolation: SampleInterpolation,
    policy: crate::quantity::QuantityPresentationPolicy,
) {
    use std::fmt::Write as _;

    let copied_x = if model.x_unit == "Hz" {
        policy.copy_frequency(x)
    } else {
        policy.copy_si_value(x, model.x_unit)
    };
    let _ = writeln!(
        target,
        "{cursor} {} = {}",
        model.x_label(),
        copied_x.trim_end()
    );
    for trace in model.traces.iter().filter(|trace| trace.visible).take(6) {
        let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
        let copied = match trace.kind {
            TraceKind::PhaseDeg => policy.copy_angle(value.to_radians()),
            TraceKind::PhaseRad => policy.copy_angle(value),
            TraceKind::MagnitudeDb => policy.copy_si_value(value, "dB"),
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                policy.copy_si_value(value, model.y_unit)
            }
        };
        let _ = writeln!(target, "{} = {}", trace.name, copied.trim_end());
    }
    while target.ends_with('\n') {
        target.pop();
    }
}

fn show_strip_plot(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    linked_cursor_domain: Option<CursorDomain>,
) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let Some((x0, x1)) = x_range(model) else {
        well_hint(ui, "No data");
        return;
    };

    // Expression traces participate in the automatic fit alongside the
    // run's visible traces.
    let exprs = resolve_strip_exprs(state, model, &t);
    let model_range = y_range(&mut state.ui.results.derived, model, false);
    let auto_y = {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        if let Some((a, b)) = model_range {
            lo = a;
            hi = b;
        }
        for expr in &exprs {
            if let Some((a, b)) = expr.y_extremes {
                lo = lo.min(a);
                hi = hi.max(b);
            }
        }
        if !lo.is_finite() || !hi.is_finite() {
            None
        } else if lo == hi {
            Some((lo - 1.0, hi + 1.0))
        } else {
            Some((lo, hi))
        }
    };
    let Some((y0, y1)) = auto_y else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };

    // User zoom/pan overrides the automatic fit per axis.
    let view = state
        .ui
        .results
        .plot_view(super::ResultViewer::Waves, model.analysis_index);
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let (y0, y1) = view.y.unwrap_or((y0, y1));

    let mut x_axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, model.x_unit),
    };
    if model.x_unit == "Hz" {
        let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
        x_axis = x_axis.with_display_transform(scale, offset, unit);
    }
    let mut spec = PlotSpec::new(x_axis, model.x_scale, Axis::linear(y0, y1, model.y_unit))
        .accessible_name("Waveform plot");
    spec.display_decimation = display_decimation(presentation.large_dataset_display());

    // Right (phase) axis when phase traces are visible.
    let has_phase = model
        .traces
        .iter()
        .any(|trace| trace.kind.is_phase() && trace.visible);
    if has_phase && let Some((p0, p1)) = y_range(&mut state.ui.results.derived, model, true) {
        let displays_radians = model
            .traces
            .iter()
            .any(|trace| trace.kind == TraceKind::PhaseRad && trace.visible);
        let axis = match (view.y_right, displays_radians) {
            (Some((z0, z1)), true) => Axis::linear_with(z0, z1, "rad", 5),
            (None, true) => Axis::linear_with(p0, p1, "rad", 5),
            // Zoomed degree axes use plain linear ticks; the 45° lattice
            // would be too dense at arbitrary zoom depths.
            (Some((z0, z1)), false) => Axis::linear_with(z0, z1, "°", 5),
            (None, false) => {
                let p0 = (p0 / 45.0).floor() * 45.0;
                let p1 = (p1 / 45.0).ceil() * 45.0;
                let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
                    .map(|i| p0 + i as f64 * 45.0)
                    .collect();
                Axis::with_ticks(p0, p1, "°", &ticks)
            }
        };
        let axis = if displays_radians {
            match quantity_policy.angle_display {
                crate::quantity::AngleDisplay::Degrees => {
                    axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
                }
                crate::quantity::AngleDisplay::Radians => axis,
            }
        } else {
            let (scale, offset, unit) = quantity_policy.degree_axis_transform();
            axis.with_display_transform(scale, offset, unit)
        };
        spec.y_right = Some((axis, t.color.traces[2]));
    }
    // 0 dB reference on log-magnitude strips.
    if model.y_unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let draw_order = model
        .traces
        .iter()
        .filter(|trace| trace.overlay)
        .chain(model.traces.iter().filter(|trace| !trace.overlay));
    for trace in draw_order {
        if !trace.visible {
            continue;
        }
        let color = if trace.overlay {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        let mut plot_trace =
            Trace::new(&trace.x, &trace.y, color).cache_key(trace_key(model, trace));
        if trace.overlay {
            plot_trace = plot_trace.thin();
        }
        if trace.kind.is_phase() {
            plot_trace = plot_trace.right().dashed();
        }
        spec.traces.push(plot_trace);
    }
    for expr in &exprs {
        spec.traces.push(
            Trace::new(&expr.x, &expr.y, expr.color)
                .thin()
                .cache_key(expr.cache_key),
        );
    }

    let cursor_domain_matches = linked_cursor_domain == Some(model.cursor_domain());
    let cursors = (state.ui.results.cursor_strip == Some(model.analysis_index)
        || (state.ui.results.linked_cursors && cursor_domain_matches))
        .then_some(state.ui.results.cursors);

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        for trace in model.traces.iter().filter(|t| t.visible).take(6) {
            let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
            rows.push((
                trace.name.clone(),
                model.format_trace_value(trace, value, significant_digits, quantity_policy),
            ));
        }
        for expr in exprs.iter().take(3) {
            let value = sample_at_with(&expr.x, &expr.y, x, interpolation);
            rows.push((
                expr.label.clone(),
                fmt_si_significant(value, "", significant_digits),
            ));
        }
        rows
    };

    let response = plot::show(
        ui,
        &spec,
        &mut state.ui.results.cache,
        cursors.as_ref(),
        Some(&readout),
    );

    if let Some(clicked_x) = response.clicked_x {
        let results = &mut state.ui.results;
        if results.cursor_strip != Some(model.analysis_index)
            && (!results.linked_cursors || !cursor_domain_matches)
        {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        results.cursors.place(clicked_x);
    }

    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Waves, model.analysis_index)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Cursors + measurements readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(action) = section_header(ui, "Cursors", Some("Clear"))
        && action.clicked()
    {
        state.ui.results.clear_cursors();
    }

    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let cursor_model = state
        .ui
        .results
        .cursor_strip
        .and_then(|idx| models.iter().find(|m| m.analysis_index == idx));

    let cursors = state.ui.results.cursors;
    match (cursor_model, cursors.a) {
        (Some(model), Some(a)) => {
            cursor_block(
                ui,
                "A",
                c.accent,
                &model.format_x(a, significant_digits, quantity_policy),
                &value_rows(model, a, presentation, quantity_policy),
            );
            if let Some(b) = cursors.b {
                cursor_block(
                    ui,
                    "B",
                    c.traces[4],
                    &model.format_x(b, significant_digits, quantity_policy),
                    &value_rows(model, b, presentation, quantity_policy),
                );
                cursor_block(
                    ui,
                    "B − A",
                    c.text_faint,
                    &model.format_x(b - a, significant_digits, quantity_policy),
                    &delta_rows(model, a, b, presentation, quantity_policy),
                );
            }
        }
        _ => {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Click a plot to place cursor A, again for B. Esc clears.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            });
        }
    }

    // Measurements over the cursor window (or the full range).
    let measured_model = cursor_model.or_else(|| models.first());
    if let Some(model) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        let title = if window.is_some() {
            "Measurements · A–B"
        } else {
            "Measurements"
        };
        section_header(ui, title, None);
        measurement_rows(
            ui,
            &mut state.ui.results.derived,
            model,
            window,
            significant_digits,
            quantity_policy,
        );
    }
}

fn value_rows(
    model: &StripModel,
    x: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(6)
        .map(|trace| {
            let value = sample_at_with(&trace.x, &trace.y, x, interpolation);
            (
                trace.name.clone(),
                model.format_trace_value(trace, value, significant_digits, quantity_policy),
            )
        })
        .collect()
}

fn delta_rows(
    model: &StripModel,
    a: f64,
    b: f64,
    presentation: ResultPresentationPolicy,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let dx = b - a;
    match model.x_scale {
        XScale::Linear if model.x_unit == "s" => {
            rows.push((
                "Δt".to_owned(),
                fmt_si_significant(dx, "s", significant_digits),
            ));
            if dx != 0.0 {
                rows.push((
                    "1/Δt".to_owned(),
                    quantity_policy.format_frequency(1.0 / dx.abs(), significant_digits),
                ));
            }
        }
        XScale::Log10 => {
            rows.push((
                "Δf".to_owned(),
                quantity_policy.format_frequency(dx, significant_digits),
            ));
        }
        _ => rows.push((
            "Δx".to_owned(),
            fmt_si_significant(dx, model.x_unit, significant_digits),
        )),
    }
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let dv = sample_at_with(&trace.x, &trace.y, b, interpolation)
            - sample_at_with(&trace.x, &trace.y, a, interpolation);
        rows.push((
            format!("Δ{}", trace.name),
            model.format_trace_value(trace, dv, significant_digits, quantity_policy),
        ));
    }
    // dB/decade slope between cursors on log strips.
    if model.x_scale == XScale::Log10 && a > 0.0 && b > 0.0 {
        let dlog = (b.log10() - a.log10()).abs();
        if dlog > 1e-12
            && let Some(mag) = model
                .traces
                .iter()
                .find(|t| t.kind == TraceKind::MagnitudeDb && t.visible)
        {
            let ddb = sample_at_with(&mag.x, &mag.y, b, interpolation)
                - sample_at_with(&mag.x, &mag.y, a, interpolation);
            rows.push((
                "slope".to_owned(),
                fmt_significant(ddb / dlog, significant_digits, " dB/dec"),
            ));
        }
    }
    rows
}

/// The design's `.cursor-block`: inset bordered block with a letter head and
/// mono key/value rows.
fn cursor_block(
    ui: &mut Ui,
    letter: &str,
    dot: egui::Color32,
    x_label: &str,
    rows: &[(String, String)],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::NONE
        .fill(c.bg_inset)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .outer_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 2,
            bottom: 6,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let width = ui.available_width();
            // Head row.
            let (head, _) = ui.allocate_exact_size(egui::vec2(width, 24.0), egui::Sense::hover());
            let painter = ui.painter();
            painter.rect_filled(
                egui::Rect::from_center_size(
                    egui::pos2(head.left() + 13.0, head.center().y),
                    egui::vec2(8.0, 8.0),
                ),
                2.0,
                dot,
            );
            painter.text(
                egui::pos2(head.left() + 24.0, head.center().y),
                egui::Align2::LEFT_CENTER,
                letter,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                c.text,
            );
            painter.text(
                egui::pos2(head.right() - 9.0, head.center().y),
                egui::Align2::RIGHT_CENTER,
                x_label,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_dim,
            );
            painter.hline(
                head.x_range(),
                head.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            // Value rows.
            for (key, value) in rows {
                let (row, _) =
                    ui.allocate_exact_size(egui::vec2(width, 18.0), egui::Sense::hover());
                let painter = ui.painter();
                painter.text(
                    egui::pos2(row.left() + 9.0, row.center().y),
                    egui::Align2::LEFT_CENTER,
                    key,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_dim,
                );
                painter.text(
                    egui::pos2(row.right() - 9.0, row.center().y),
                    egui::Align2::RIGHT_CENTER,
                    value,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text,
                );
            }
            ui.add_space(5.0);
        });
}

/// min / max / rms rows per visible trace, optionally windowed to [a, b].
/// The single-pass stats are cached per (trace, window, data version), so
/// no samples are rescanned until the cursors or the data move.
fn measurement_rows(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
    significant_digits: usize,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) {
    use crate::waveform::measurements as basic;

    // Window identity for the cache key; u64::MAX is a NaN bit pattern no
    // finite cursor can produce, marking the full-range case.
    let (a_bits, b_bits) = match window {
        Some((a, b)) => (a.to_bits(), b.to_bits()),
        None => (u64::MAX, u64::MAX),
    };

    let mut rows: Vec<(String, String)> = Vec::new();
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let key = (trace_key(model, trace), a_bits, b_bits);
        let stats = derived.stats_or(key, || {
            let (start, end) = match window {
                Some((a, b)) => {
                    let start = trace.x.partition_point(|&v| v < a);
                    let end = trace.x.partition_point(|&v| v <= b);
                    (start, end.max(start))
                }
                None => (0, trace.y.len()),
            };
            basic::calculate_min_max_rms(&trace.y[start..end])
        });
        let Some((min, max, rms)) = stats else {
            continue;
        };
        let fmt = |v: f64| -> String {
            match trace.kind {
                TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                    fmt_si_significant(v, model.y_unit, significant_digits)
                }
                TraceKind::MagnitudeDb => fmt_significant(v, significant_digits, " dB"),
                TraceKind::PhaseDeg => {
                    quantity_policy.format_angle(v.to_radians(), significant_digits)
                }
                TraceKind::PhaseRad => quantity_policy.format_angle(v, significant_digits),
            }
        };
        rows.push((format!("{} min", trace.name), fmt(min)));
        rows.push((format!("{} max", trace.name), fmt(max)));
        if matches!(
            trace.kind,
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
        ) {
            rows.push((format!("{} rms", trace.name), fmt(rms)));
        }
    }
    let refs: Vec<(&str, &str)> = rows.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    crate::ui::widgets::measurement_table(ui, &refs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::state::{AnalysisResult, AnalysisResultProvenance, SimulationRun, WaveformData};
    use crate::workbench::ChoicePreference;

    #[test]
    fn noise_strip_uses_spectral_density_unit_without_db_conversion() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 2.0e-18], "#fff"),
            ]),
        );
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
        );

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].y_unit, "V^2/Hz");
        assert!(matches!(models[0].traces[0].kind, TraceKind::Value));
        assert_eq!(models[0].traces[0].y.as_slice(), &[1.0e-18, 2.0e-18]);
    }

    #[test]
    fn family_selection_projects_exact_source_rows_without_mutating_the_run() {
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(41, AnalysisType::Corner, "PVT").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![1.0, 2.0, 3.0, 4.0],
                    vec![10.0, 20.0, 30.0, 40.0],
                    "#fff",
                ),
            ]),
        );
        let run = simulation.active_run().expect("active run");
        let selection = SourceSampleSelection::new(run.dataset_id, 41, vec![1, 3])
            .expect("ordered exact selection");
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            Some(&selection),
        );

        assert_eq!(models[0].traces[0].x.as_slice(), &[2.0, 4.0]);
        assert_eq!(models[0].traces[0].y.as_slice(), &[20.0, 40.0]);
        let original = &simulation.active_run().unwrap().analyses[0].waveforms[0];
        assert_eq!(original.x.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(original.y.as_slice(), &[10.0, 20.0, 30.0, 40.0]);
    }

    fn ac_result(
        source_id: AnalysisInstanceId,
        values: [f64; 2],
        snapshot_byte: u8,
    ) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![1.0, 10.0],
                values.to_vec(),
                "#fff",
            )])
            .with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([snapshot_byte; 32]),
                    Vec::new(),
                )
                .expect("valid AC provenance"),
            )
    }

    #[test]
    fn overlays_pair_two_same_kind_results_by_exact_source_instance() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();

        let mut active = SimulationRun::new(2);
        active.add_analysis(ac_result(first_id, [1.0, 2.0], 0x11));
        active.add_analysis(ac_result(second_id, [3.0, 4.0], 0x11));

        let mut overlay = SimulationRun::new(1);
        // Reverse the same-kind result order: kind/label matching would alias
        // the first overlay result onto both active strips.
        overlay.add_analysis(ac_result(second_id, [201.0, 202.0], 0x22));
        overlay.add_analysis(ac_result(first_id, [101.0, 102.0], 0x22));
        let overlay_dataset_id = overlay.dataset_id;

        let mut simulation = SimulationState {
            runs: vec![active, overlay],
            active_run_idx: Some(0),
            overlay_dataset_ids: vec![overlay_dataset_id],
            ..SimulationState::default()
        };
        assert!(simulation.select_analysis(0));
        let mut derived = DerivedSeries::default();

        let models = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseDegrees,
            None,
        );

        assert_eq!(models.len(), 2);
        assert_eq!(models[0].signal_trace_count, 1);
        assert_eq!(models[0].traces.len(), 2);
        assert!(models[0].traces[1].overlay);
        assert_eq!(models[0].traces[1].y.as_slice(), &[101.0, 102.0]);
        assert_eq!(models[1].signal_trace_count, 1);
        assert_eq!(models[1].traces.len(), 2);
        assert!(models[1].traces[1].overlay);
        assert_eq!(models[1].traces[1].y.as_slice(), &[201.0, 202.0]);
    }

    #[test]
    fn complex_display_policy_uses_original_components_or_radian_phase() {
        let magnitude = WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 10.0], "#fff")
            .with_complex_components("V(out)", vec![0.8, 6.0], vec![0.6, 8.0]);
        let phase = WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![180.0, 90.0], "#aaa");
        let mut simulation = SimulationState::default();
        simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![magnitude, phase]),
        );

        let mut derived = DerivedSeries::default();
        let cartesian = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::RealImaginary,
            None,
        );
        assert_eq!(cartesian[0].signal_trace_count, 2);
        assert_eq!(cartesian[0].y_unit, "");
        assert_eq!(cartesian[0].traces[0].name, "re(V(out))");
        assert_eq!(cartesian[0].traces[0].y.as_slice(), &[0.8, 6.0]);
        assert_eq!(cartesian[0].traces[1].name, "im(V(out))");
        assert_eq!(cartesian[0].traces[1].y.as_slice(), &[0.6, 8.0]);

        let radians = build_models(
            &simulation,
            &mut derived,
            &Tokens::default(),
            false,
            ComplexNumberDisplay::MagnitudePhaseRadians,
            None,
        );
        assert!(matches!(radians[0].traces[0].kind, TraceKind::MagnitudeDb));
        assert_eq!(radians[0].traces[0].y.as_slice(), &[0.0, 20.0]);
        assert!(matches!(radians[0].traces[1].kind, TraceKind::PhaseRad));
        assert!((radians[0].traces[1].y[0] - std::f64::consts::PI).abs() < 1e-12);
        assert!((radians[0].traces[1].y[1] - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn cursor_copy_uses_explicit_scientific_si_policy() {
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![2.0e-18, 4.0e-18], "#fff"),
            ]),
        );
        state.ui.results.cursor_strip = Some(0);
        state.ui.results.cursors.a = Some(10.0);
        state
            .ui
            .preferences
            .set_choice(ChoicePreference::CopiedValueFormat, 1)
            .unwrap();

        let copied = copy_cursor_text(&mut state).expect("active cursor has copy data");

        assert!(copied.contains("A f = 1.00000000000000000e1 Hz"));
        assert!(copied.contains("onoise = 4."));
        assert!(copied.contains("e-18 V^2/Hz"));
    }
}
