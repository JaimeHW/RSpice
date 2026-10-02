//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

#[cfg(test)]
use super::TracePresentationKey;
use rspice_results_ui::derived::DerivedSeries;
use rspice_results_ui::presentation::well_hint;
use rspice_results_ui::waves::{
    CursorDomain, NOISE_DENSITY_UNIT, StripTrace, TraceKind, anchor_key, family_color, fmt_in_unit,
    stable_hash, trace_key,
};
pub(super) use rspice_results_ui::waves::{FamilyTraceVisibilityKey, StripModel, UnitPane};
pub(crate) use rspice_results_ui::waves::{
    analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};
mod expression_evaluation;
mod expressions;
mod extent;
pub(super) use extent::{FamilyEnvelopeCache, FamilyEnvelopePlan};
pub(super) mod marker_dialog;
mod model_cache;
pub(super) use model_cache::{ModelsCache, cached_models};
mod readout;
mod viewport;

use expression_evaluation::{evaluate_expression, evaluate_expression_with_policy};
pub(crate) use expressions::*;
pub(crate) use readout::*;
pub(crate) use viewport::*;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::schematic::bus_notations;
use crate::state::{AnalysisResult, AnalysisType, SharedWaveformValues, SimulationState};
use crate::ui::icons::Icon;
use crate::ui::plot::sample::{
    BranchSample, SweepClass, SweepShape, XOrientation, nearest_sample, sample_at_with_shape,
    sample_branches_into,
};
use crate::ui::plot::{
    self, Axis, CursorPair, DisplayDecimation, MAX_AXIS_TICKS, PlotSpec, SampleInterpolation,
    Trace, XScale, fmt_si_significant, fmt_significant,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{IconButton, chip, section_header};
use crate::workbench::AppState;
use crate::workbench::{
    ComplexNumberDisplay, CursorInterpolation, LargeDatasetDisplay, ResultPresentationPolicy,
};
use rspice_results::family_projection::{FamilyTraceStyle, SourceSampleSelection};

use super::frame_work::{self, FrameSampleRead};
use super::{
    AnalysisPresentationKey, ExprEditor, ExprSeries, ExprTrace, ExpressionSeriesResult,
    ExpressionSource, ExpressionWaveform, HorizontalWaveCursor, MarkerEditDraft, MarkerKind,
    MarkerSelector, MarkerView, ResultsState, SelectedResultTrace, SourceWaveformPresentationKey,
    WavePanePresentationKey, WaveformPresentationKey,
};
use rspice_results_ui::strip::{LegendChip, StripHeader};

const WAVE_SHARED_X_HEIGHT: f32 = 57.0;

/// The shared-X strip's three bands, offset from the strip's top edge.
///
/// They are ordered by what each one describes. Tick labels and the A/B
/// flags both read the panes' zoomed viewport, so they sit directly against
/// the panes; the full-range overview lane is the outermost band, and
/// nothing viewport-scaled is ever drawn across it.
const SHARED_X_LABEL_TOP: f32 = 3.0;
const SHARED_X_FLAG_TOP: f32 = 17.0;
const SHARED_X_FLAG_HEIGHT: f32 = 13.0;
const SHARED_X_LANE_TOP: f32 = 35.0;
const SHARED_X_LANE_HEIGHT: f32 = 14.0;

/// The narrowest viewport an overview-handle drag can produce, as a fraction
/// of the full retained sweep. It is the reciprocal of the zoom ceiling, so
/// dragging a handle cannot reach a magnification the zoom controls refuse.
const SHARED_X_MIN_WINDOW: f64 = 1.0 / 200.0;
/// Points the overview lane's mini-trace is drawn from, whatever the run
/// retained. The lane is fourteen pixels tall and a plot wide, so a denser
/// curve would not reach the reader as anything but the same grey band.
const SHARED_X_OVERVIEW_POINTS: usize = 160;
// Mockup gutter geometry: a 64 px left gutter carries the Y ticks and the
// X-strip's band labels; the right edge keeps only a 14 px breathing strip
// now that no pane owns a secondary axis.
const WAVE_SHARED_LEFT_MARGIN: f32 = 64.0;
const WAVE_SHARED_RIGHT_MARGIN: f32 = 14.0;

/// The mockup's per-sheet left gutter: the noise sheet's nV/√Hz tick
/// labels need 88 px where the shared 64 px gutter suffices elsewhere.
fn wave_left_margin(results: &ResultsState) -> f32 {
    match results.viewer {
        super::ResultViewer::NoiseContrib => 88.0,
        _ => WAVE_SHARED_LEFT_MARGIN,
    }
}
const WAVE_PANE_HEADER_HEIGHT: f32 = 25.0;
const WAVE_MIN_PLOT_HEIGHT: f32 = 24.0;

/// Apply quick-view presentation overrides after constructing the immutable
/// dataset projection. Family visibility remains the more specific gate, so
/// revealing a source never accidentally reveals a hidden family member.
fn apply_waveform_visibility(
    models: &mut [StripModel],
    simulation: &SimulationState,
    overrides: &HashMap<SourceWaveformPresentationKey, bool>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
) {
    let Some(run) = simulation.active_run() else {
        return;
    };
    for model in models {
        let Some(analysis) = run.analyses.get(model.analysis_index) else {
            continue;
        };
        for trace in &mut model.traces {
            let Some(waveform) = analysis.waveforms.get(trace.waveform_index) else {
                trace.visible = false;
                continue;
            };
            let key = SourceWaveformPresentationKey::new(
                model.analysis_key,
                trace.source_waveform_name.clone(),
            );
            let source_visible = overrides.get(&key).copied().unwrap_or(waveform.visible);
            trace.visible = source_visible
                && trace
                    .family_visibility_key
                    .is_none_or(|key| !hidden_family_traces.contains(&key));
        }
    }
}

fn apply_family_trace_style<'a>(
    mut trace: Trace<'a>,
    style: Option<FamilyTraceStyle>,
) -> Trace<'a> {
    let Some(style) = style else {
        return trace;
    };
    trace = trace.show_single_point();
    if let Some(ordinal) = style.dash_ordinal {
        trace = trace.dash_style(ordinal);
    }
    if let Some(ordinal) = style.marker_ordinal {
        trace = trace.marker_style(ordinal);
    }
    if let Some(width) = style.width_points {
        trace = trace.width(width);
    }
    trace
}

/// Project the host-selected retained sources into the shared waveform model.
pub(super) fn build_models(
    simulation: &SimulationState,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
) -> Vec<StripModel> {
    let display_runs = simulation.display_runs();
    rspice_results_ui::waves::build_models(
        &display_runs,
        &simulation.executed_decks,
        derived,
        tokens,
        rspice_results_ui::waves::ProjectionOptions {
            phase_continuous,
            complex_display,
            selection,
            hidden_family_traces,
        },
        |index| super::incomplete_evidence_reason(&display_runs[0].analyses[index]),
    )
}

pub(super) fn source_waveform_anchor(
    state: &mut AppState,
    source_name: &str,
) -> Option<(AnalysisPresentationKey, WaveformPresentationKey)> {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let active_analysis_index = state.simulation.active_analysis_idx?;
    let model = models
        .iter()
        .find(|model| model.analysis_index == active_analysis_index)?;
    let trace = model
        .traces
        .iter()
        .filter(|trace| !trace.overlay && trace.source_waveform_name == source_name)
        .min_by_key(|trace| trace.family_group_ordinal.is_some())?;
    Some((model.analysis_key, anchor_key(model, trace)))
}

/// Y range of one pane's traces, padded 8 %. Per-trace extremes are cached
/// on the data version — never rescanned per frame.
///
/// The fit is per pane because each pane carries its own unit: fitting
/// volts and amps to one range would flatten whichever is smaller.
fn pane_y_range(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for trace in indices.iter().filter_map(|index| model.traces.get(*index)) {
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
    let pad = (max - min) * 0.08;
    // A span that cannot survive its own padding is flat as far as any axis
    // can draw it: exactly constant, or constant to within a few ulps. Both
    // widen the same way, because a range finer than the tick ladder can
    // subdivide leaves the pane with no labels at all.
    if pad <= 0.0 {
        return Some((min - 1.0, max + 1.0));
    }
    Some((min - pad, max + pad))
}

fn model_is_visible(model: &StripModel, models: &[StripModel], results: &ResultsState) -> bool {
    match results.maximized_strip {
        Some(maximized) if models.iter().any(|item| item.analysis_key == maximized) => {
            model.analysis_key == maximized
        }
        _ => !results.hidden_strips.contains(&model.analysis_key),
    }
}

fn active_pane<'a>(
    models: &'a [StripModel],
    results: &ResultsState,
) -> Option<(&'a StripModel, usize, UnitPane<'a>)> {
    let active = results.active_wave_pane.as_ref()?;
    let model = models
        .iter()
        .find(|model| model.analysis_key == active.analysis)
        .filter(|model| model_is_visible(model, models, results))?;
    model
        .unit_panes()
        .into_iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == active.unit)
        .map(|(ordinal, pane)| (model, ordinal, pane))
}

/// Keep the instrument strip bound to an exact, currently visible waveform
/// pane. Stable analysis identity and engineering unit are used instead of
/// transient pane ordinals, so hiding or reordering a strip cannot redirect an
/// action to unrelated data.
pub(super) fn reconcile_active_pane(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    if active_pane(&models, &state.ui.results).is_some() {
        return;
    }

    let preferred_analysis = state
        .ui
        .results
        .valid_selected_trace(&state.simulation)
        .map(SelectedResultTrace::analysis_key)
        .or_else(|| {
            state.ui.results.cursor_strip.and_then(|index| {
                models
                    .iter()
                    .find(|model| model.analysis_index == index)
                    .map(|model| model.analysis_key)
            })
        });
    let next = preferred_analysis
        .and_then(|analysis| {
            models.iter().find(|model| {
                model.analysis_key == analysis
                    && model_is_visible(model, &models, &state.ui.results)
            })
        })
        .or_else(|| {
            models
                .iter()
                .find(|model| model_is_visible(model, &models, &state.ui.results))
        })
        .and_then(|model| {
            model
                .unit_panes()
                .first()
                .map(|pane| WavePanePresentationKey {
                    analysis: model.analysis_key,
                    unit: pane.unit.to_owned(),
                })
        });
    state.ui.results.active_wave_pane = next;
}

fn matching_spec_limits(
    state: &AppState,
    model: &StripModel,
    pane: &UnitPane,
    t: &Tokens,
) -> Vec<plot::LimitLine> {
    // The bounds a run was judged against are the ones its receipt froze, not
    // the ones the workspace is authoring now. Drawn as overlays on a
    // completed run's waveforms, a limit edited after the run put a line the
    // run had never been measured against straight across its curve — beside
    // the verdict the run had actually earned. This is the same resolver the
    // specifications sheet reads.
    let specifications = super::run_specifications(state);
    let mut seen = HashSet::new();
    let mut limits = Vec::new();
    for trace in pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay)
    {
        for specification in specifications.iter().filter(|specification| {
            specification
                .measurement
                .eq_ignore_ascii_case(&trace.source_waveform_name)
        }) {
            if let Some(minimum) = specification.min.filter(|value| value.is_finite())
                && seen.insert((
                    specification.measurement.to_ascii_lowercase(),
                    0_u8,
                    minimum.to_bits(),
                ))
            {
                limits.push(plot::LimitLine {
                    y: minimum,
                    color: t.color.warn,
                    label: format!(
                        "{} \u{2265} {}",
                        specification.measurement,
                        fmt_si_significant(minimum, &specification.unit, 6)
                    ),
                });
            }
            if let Some(maximum) = specification.max.filter(|value| value.is_finite())
                && seen.insert((
                    specification.measurement.to_ascii_lowercase(),
                    1_u8,
                    maximum.to_bits(),
                ))
            {
                limits.push(plot::LimitLine {
                    y: maximum,
                    color: t.color.warn,
                    label: format!(
                        "{} \u{2264} {}",
                        specification.measurement,
                        fmt_si_significant(maximum, &specification.unit, 6)
                    ),
                });
            }
        }
    }
    limits
}

pub(super) fn spec_limits_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    active_pane(&models, &state.ui.results)
        .is_some_and(|(model, _, pane)| !matching_spec_limits(state, model, &pane, t).is_empty())
}

pub(super) fn family_envelope_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let generation = state.ui.results.models.generation();
    let Some((model, _, pane)) = active_pane(&models, &state.ui.results) else {
        return false;
    };
    !extent::family_envelopes(&mut state.ui.results, generation, model, &pane)
        .series()
        .is_empty()
}

/// Everything dropping a marker at cursor A needs to name where it landed:
/// the analysis and the trace anchor that identify it across a re-run, the
/// trace's display name, the abscissa the cursor sits on, and that trace's
/// retained X column — kept so the marker can be re-sampled without going
/// back through the strip model.
type CursorMarkerTarget = (
    AnalysisPresentationKey,
    WaveformPresentationKey,
    String,
    f64,
    std::sync::Arc<Vec<f64>>,
);

fn cursor_marker_target(state: &AppState, models: &[StripModel]) -> Option<CursorMarkerTarget> {
    let cursor_x = state
        .ui
        .results
        .cursors
        .a
        .filter(|value| value.is_finite())?;
    let (model, _, pane) = active_pane(models, &state.ui.results)?;
    let pane_traces = pane
        .traces
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .filter(|trace| trace.visible && !trace.overlay)
        .collect::<Vec<_>>();
    let selected_source = state
        .ui
        .results
        .valid_selected_trace(&state.simulation)
        .filter(|selected| selected.analysis_key() == model.analysis_key)
        .map(SelectedResultTrace::source_name);
    let trace = state
        .ui
        .results
        .cursor_a_anchor
        .as_ref()
        .filter(|anchor| anchor.analysis == model.analysis_key)
        .and_then(|anchor| {
            pane_traces
                .iter()
                .copied()
                .find(|trace| anchor_key(model, trace) == *anchor)
        })
        .or_else(|| {
            selected_source.and_then(|source| {
                pane_traces
                    .iter()
                    .copied()
                    .find(|trace| trace.source_waveform_name == source)
            })
        })
        .or_else(|| pane_traces.first().copied())?;
    // Only a finiteness gate — the marker carries the X, not this value — but
    // it is gated on the reading the strip will make, so it is taken the same
    // way the strip takes it.
    let sampled = sample_at_with_shape(
        &trace.x,
        &trace.y,
        &trace.shape,
        cursor_x,
        cursor_interpolation(
            state
                .ui
                .preferences
                .result_presentation_policy()
                .cursor_interpolation(),
        ),
    );
    sampled.is_finite().then(|| {
        (
            model.analysis_key,
            anchor_key(model, trace),
            trace.name.clone(),
            cursor_x,
            trace.x.clone(),
        )
    })
}

pub(super) fn marker_at_cursor_a_available(state: &mut AppState, t: &Tokens) -> bool {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    cursor_marker_target(state, &models).is_some()
}

pub(super) fn drop_marker_at_cursor_a(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((analysis, anchor, trace_name, x, samples)) = cursor_marker_target(state, &models)
    else {
        return;
    };
    // Same ownership rule as a plot click, and the same dialog: the pane the
    // cursor is on decides which store retains the marker.
    let placement = super::MarkerPlacement {
        analysis,
        anchor,
        trace_name,
        x,
        samples: samples.as_slice(),
    };
    if let Some(selector) = super::place_marker(state, placement) {
        marker_dialog::open(state, selector);
    }
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
    show_with_pane_chrome(ui, state, true);
}

/// Render the same retained waveform strips inside the compact split-results
/// pane. Trace selection and swatch visibility remain fully interactive, while
/// the pane-only maximize/close/fit and expression controls are omitted.
pub fn show_compact(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, false);
}

/// The Bode sheet: the run's AC response through the same pane-stack
/// instrument, which [`StripModel::unit_panes`] reads as the mockup's two
/// weighted panes — magnitude over phase on one shared X domain. The AC
/// scope rides the viewer-aware model cache; stability margins stay in the
/// right panel's inspector card.
pub fn show_bode(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, true);
}

/// The Noise sheet: retained noise PSDs as an nV/√Hz pane through the same
/// instrument, scoped to Noise analyses by the viewer-aware model cache.
/// The spectrum summary card stays in the right panel.
pub fn show_noise(ui: &mut Ui, state: &mut AppState) {
    show_with_pane_chrome(ui, state, true);
}

fn show_with_pane_chrome(ui: &mut Ui, state: &mut AppState, pane_chrome: bool) {
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
                crate::workbench::commands::vocabulary::Command::RunSimulation,
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            if shortcut.is_empty() {
                "No results yet — use the Run button to simulate".to_owned()
            } else {
                format!("No results yet — press {shortcut} or use the Run button to simulate")
            }
        } else {
            "No waveform is selected for display. Show a retained signal in the Results browser, or place a schematic probe and run again."
                .to_owned()
        };
        well_hint(ui, &hint);
        return;
    }

    // Apply hide/maximize strip state.
    let results = &state.ui.results;
    let visible: Vec<&StripModel> = match results.maximized_strip {
        Some(max_key) if models.iter().any(|m| m.analysis_key == max_key) => models
            .iter()
            .filter(|m| m.analysis_key == max_key)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_key))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(ui, "All strips hidden — restore them from the document bar");
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_maximize: Option<AnalysisPresentationKey> = None;
    let mut close_strip: Option<AnalysisPresentationKey> = None;
    let mut fit_strip: Option<AnalysisPresentationKey> = None;
    let mut toggle_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut remove_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut open_editor: Option<AnalysisPresentationKey> = None;
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
                        // Analysis identity remains in the strip header.
                        // Unit-owned signal legends live in the pane headers;
                        // only unitless expressions remain alongside the
                        // analysis identity here.
                        let strip_exprs: Vec<ExprTrace> = state
                            .ui
                            .results
                            .analysis_exprs
                            .get(&model.analysis_key)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> = strip_exprs
                            .iter()
                            .map(|expr| {
                                let complex = state
                                    .ui
                                    .results
                                    .analysis_expr_cache
                                    .get(&(model.analysis_key, expr.text.clone()))
                                    .is_some_and(|cached| {
                                        cached.series.as_ref().is_ok_and(|outputs| {
                                            outputs
                                                .iter()
                                                .any(|output| output.waveform.complex.is_some())
                                        })
                                    });
                                expression_label(expr, complex)
                            })
                            .collect();
                        let legend: Vec<LegendChip<'_>> = strip_exprs
                            .iter()
                            .enumerate()
                            .map(|(i, expr)| LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, expr_palette_slot(model, i)),
                                on: expr.visible,
                            })
                            .collect();

                        let zoomed = state.ui.results.analysis_strip_is_zoomed(
                            super::ResultViewer::Waves,
                            model.analysis_key,
                        );
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .incomplete(model.incomplete)
                            .maximized(maximized)
                            .closable(pane_chrome && !maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(0)
                            .pane_actions(pane_chrome)
                            .show(ui);
                        if let Some(chip_index) = header.legend_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_visibility_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_removed {
                            remove_expr = Some((model.analysis_key, chip_index));
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_key);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_key);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_key);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_key);
                        }

                        expr_editor_row(ui, state, model.analysis_key, model.analysis_index);

                        // Strips scrolled out of view skip the plot body
                        // entirely (range lookups, envelope mapping, shape
                        // building) — only the space is reserved.
                        let plot_rect = ui.available_rect_before_wrap();
                        if ui.is_rect_visible(plot_rect) {
                            show_strip_plot(ui, state, model, linked_cursor_domain.as_ref());
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    let results = &mut state.ui.results;
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if models
            .iter()
            .find(|model| model.analysis_key == idx)
            .is_some_and(|model| results.cursor_strip == Some(model.analysis_index))
        {
            results.clear_cursors();
        }
    }
    if let Some(key) = fit_strip {
        results.reset_analysis_plot_view(super::ResultViewer::Waves, key);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .analysis_exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = results.analysis_exprs.get_mut(&analysis)
    {
        if index < list.len() {
            let removed = list.remove(index);
            results
                .analysis_expr_cache
                .remove(&(analysis, removed.text));
        }
        if list.is_empty() {
            results.analysis_exprs.remove(&analysis);
        }
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some(analysis) = open_editor {
        results.expr_editor = Some(ExprEditor {
            analysis,
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
        policy.copy_si_value(x, &model.x_unit)
    };
    let _ = writeln!(
        target,
        "{cursor} {} = {}",
        model.x_label(),
        copied_x.trim_end()
    );
    for trace in model.traces.iter().filter(|trace| trace.visible).take(6) {
        // The copy has to say what the table says. Read unshaped, a loop's
        // line pasted a value off the far side of its turnaround while the
        // register on screen reported each branch.
        let value = sample_at_with_shape(&trace.x, &trace.y, &trace.shape, x, interpolation);
        let copied = match trace.kind {
            TraceKind::PhaseDeg => policy.copy_angle(value.to_radians()),
            TraceKind::PhaseRad => policy.copy_angle(value),
            TraceKind::MagnitudeDb => policy.copy_si_value(value, model.trace_unit(trace)),
            TraceKind::NoiseDensity => policy.copy_scaled_unit_value(value, NOISE_DENSITY_UNIT),
            TraceKind::Value | TraceKind::Real | TraceKind::Imaginary => {
                // The trace's own unit, not the strip's: copying a supply
                // current off a sheet it shares with node voltages must not
                // paste milliamps as millivolts.
                policy.copy_si_value(value, model.trace_unit(trace))
            }
        };
        let _ = writeln!(target, "{} = {}", trace.name, copied.trim_end());
    }
    while target.ends_with('\n') {
        target.pop();
    }
}

fn model_x_axis(
    model: &StripModel,
    x0: f64,
    x1: f64,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Axis {
    let axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, &model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, &model.x_unit),
    }
    .with_label(&model.x_label);
    if model.x_unit == "Hz" {
        let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
        axis.with_display_transform(scale, offset, unit)
    } else {
        axis
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WaveStackGeometry {
    /// Height of one weight unit, before a pane's own weight is applied.
    pane_unit_height: f32,
    total_weight: f32,
    shared_x_height: f32,
    seam_height: f32,
}

/// The mockup's pane weights: the sheet's primary quantity takes three parts
/// and every companion pane two, so a supply-current or phase pane reads as
/// the secondary evidence it is instead of splitting the stack evenly.
fn pane_weight(ordinal: usize, pane_count: usize) -> f32 {
    if pane_count <= 1 {
        1.0
    } else if ordinal == 0 {
        3.0
    } else {
        2.0
    }
}

impl WaveStackGeometry {
    fn pane_height(&self, ordinal: usize, pane_count: usize) -> f32 {
        (self.pane_unit_height * pane_weight(ordinal, pane_count)).max(0.0)
    }
}

fn wave_stack_geometry(available_height: f32, pane_count: usize) -> WaveStackGeometry {
    if pane_count == 0 || !available_height.is_finite() {
        return WaveStackGeometry {
            pane_unit_height: 0.0,
            total_weight: 1.0,
            shared_x_height: 0.0,
            seam_height: 0.0,
        };
    }
    let available = available_height.max(0.0);
    let seam_count = pane_count.saturating_sub(1) as f32;
    let seam_height = if seam_count > 0.0 {
        (available / seam_count).min(1.0)
    } else {
        0.0
    };
    let content = (available - seam_height * seam_count).max(0.0);
    // The normal 50 px navigator is retained when space permits and shrinks
    // proportionally in constrained multi-pane/multi-strip arrangements.
    // The sum is exact: this function never asks the parent to grow.
    let shared_x_height = (content * 0.28).min(WAVE_SHARED_X_HEIGHT);
    let total_weight: f32 = (0..pane_count)
        .map(|ordinal| pane_weight(ordinal, pane_count))
        .sum();
    let pane_unit_height = ((content - shared_x_height) / total_weight).max(0.0);
    WaveStackGeometry {
        pane_unit_height,
        total_weight,
        shared_x_height,
        seam_height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SharedXDrag {
    Viewport,
    ResizeStart,
    ResizeEnd,
    CursorA,
    CursorB,
}

fn shared_x_drag_id(model: &StripModel) -> egui::Id {
    egui::Id::new(("rspice.results.shared-x-drag", model.analysis_key))
}

/// What the inspector's Active pane section reports about the pane the
/// instrument is acting on.
///
/// The pane's identity, scale and limit binding live behind this module's
/// privacy, so the inspector asks for them rather than reaching in — and the
/// scale reader sits beside the header toggle that writes it.
pub(crate) struct ActivePaneFacts {
    /// The unit that names the pane in a unit-scoped stack.
    pub unit: Option<String>,
    /// The analysis the pane belongs to. A sheet that is not the waveform
    /// stack still draws exactly one analysis, and it is not necessarily the
    /// one the run's analysis selector points at.
    pub analysis: Option<String>,
    /// Visible and bound trace counts on this pane alone.
    pub traces: Option<(usize, usize)>,
    /// How many runs the pane draws at once. The mockup's corner families are
    /// a Visualization Studio presentation; on this workspace the same
    /// question — one curve, or several realizations of it? — is answered by
    /// the overlaid runs the pane is actually carrying.
    pub runs: Option<usize>,
    pub scale: Option<&'static str>,
    pub limit_mask: &'static str,
    pub x_viewport: Option<String>,
    pub y_viewport: Option<String>,
    /// The same two intervals as numbers, so an axis-limit editor opens on
    /// what the reader is actually looking at rather than on an empty field.
    pub x_extent: Option<(f64, f64)>,
    pub y_extent: Option<(f64, f64)>,
    /// Whether any pane of the active strip is showing a pinned viewport.
    /// `None` when this sheet has no unit-pane stack to report on.
    pub pinned: Option<bool>,
}

pub(crate) fn active_pane_facts(tokens: &Tokens, state: &mut AppState) -> ActivePaneFacts {
    // The viewport the pane is actually showing, whether the user pinned it
    // or it is fitting the retained data. The mockup states the interval
    // either way: "automatic" alone does not tell a reader what they see.
    let (x_viewport, y_viewport) = active_pane_viewports(tokens, state);
    let (x_extent, y_extent) = active_pane_extents(tokens, state);
    let (analysis, traces, runs) = active_pane_identity(tokens, state);
    let pinned = state
        .ui
        .results
        .active_wave_pane
        .is_some()
        .then(|| active_pane_is_pinned(tokens, state));
    let key = state.ui.results.active_wave_pane.as_ref();
    let scale = key.map(|key| {
        // One owner: the pane’s own log-Y flag on ResultsState. This read
        // used to reach into egui’s persisted memory with a hand-built id,
        // which is a second copy of a fact and drifts the moment either side
        // changes its key.
        if state.ui.results.log_y_panes.contains(key) {
            "logarithmic"
        } else {
            "linear"
        }
    });
    ActivePaneFacts {
        unit: key.map(|key| key.unit.clone()),
        analysis,
        traces,
        runs,
        scale,
        limit_mask: if state.ui.results.show_spec_limits {
            "project specification limits"
        } else {
            "none bound"
        },
        x_viewport,
        y_viewport,
        x_extent,
        y_extent,
        pinned,
    }
}

/// What the status bar reports about the sheet on the Results workspace.
pub(crate) struct SharedXStatus {
    /// The X interval the panes are showing.
    pub span: String,
    /// How far that interval is magnified from the full retained sweep.
    pub zoom: f64,
}

/// The visible X interval and its magnification.
///
/// The instrument bar owns the A/B cursor readout, so the status bar's
/// coordinate segment states the view rather than triplicating the cursors —
/// and its zoom chip reports this magnification instead of a canvas scale no
/// waveform sheet has.
pub(crate) fn active_shared_x_status(
    tokens: &Tokens,
    state: &mut AppState,
) -> Option<SharedXStatus> {
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let active = state
        .ui
        .results
        .active_wave_pane
        .as_ref()
        .map(|key| key.analysis);
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let model = match active {
        Some(key) => models.iter().find(|model| model.analysis_key == key),
        None => models.first(),
    }?;
    let full = model.x_range?;
    let panes = model.unit_panes().len();
    let view = shared_x_view(&state.ui.results, model.analysis_key, panes).unwrap_or(full);
    let (start, end) = shared_axis_viewport_fraction(model.x_scale, full, view);
    let width = end - start;
    Some(SharedXStatus {
        span: format!(
            "{} … {}",
            model.format_x(view.0, digits, quantity_policy),
            model.format_x(view.1, digits, quantity_policy)
        ),
        zoom: if width > 0.0 { 1.0 / width } else { 1.0 },
    })
}

/// The analysis the active pane belongs to and how many of its traces the
/// pane carries.
///
/// A sheet that is not the waveform stack picks its own analysis, so the
/// run's analysis selector is not the authority here: the pane is.
fn active_pane_identity(
    tokens: &Tokens,
    state: &mut AppState,
) -> (Option<String>, Option<(usize, usize)>, Option<usize>) {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return (None, None, None);
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return (None, None, None);
    };
    let bound = model
        .traces
        .iter()
        .filter(|trace| model.trace_unit(trace) == key.unit)
        .count();
    let visible = model
        .traces
        .iter()
        .filter(|trace| trace.visible && model.trace_unit(trace) == key.unit)
        .count();
    let runs = model
        .traces
        .iter()
        .filter(|trace| model.trace_unit(trace) == key.unit)
        .map(|trace| trace.run_id)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let label = state
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(model.analysis_index))
        .map(|analysis| analysis.label.clone());
    (label, Some((visible, bound)), Some(runs))
}

/// The active pane's X and Y intervals, formatted through the strip's own
/// formatter so they read like every other number on the sheet.
/// The active pane's X and Y intervals, as numbers.
///
/// One derivation, shared with the formatted rows below, so a typed axis
/// limit can never open on a different interval than the one displayed.
fn active_pane_extents(
    tokens: &Tokens,
    state: &mut AppState,
) -> (Option<super::AxisExtent>, Option<super::AxisExtent>) {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return (None, None);
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return (None, None);
    };
    let panes = model.unit_panes();
    let Some((ordinal, pane)) = panes
        .iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == key.unit)
    else {
        return (None, None);
    };
    let x = model
        .x_range
        .map(|full| shared_x_view(&state.ui.results, key.analysis, panes.len()).unwrap_or(full));
    let pinned = state
        .ui
        .results
        .analysis_plot_view_pane(super::ResultViewer::Waves, key.analysis, ordinal)
        .y;
    let y = match pinned {
        Some(range) => Some(range),
        None => displayed_pane_auto_y(state, model, pane, ordinal, tokens),
    };
    (x, y)
}

/// Whether the active pane's axis carries an explicit interval.
pub(crate) fn active_pane_axis_is_pinned(results: &ResultsState, axis: super::PaneAxis) -> bool {
    let Some(key) = results.active_wave_pane.as_ref() else {
        return false;
    };
    // Resolving the pane ordinal would need the built models, which this
    // accessor deliberately does not take. It does not need them: X is shared
    // by the whole strip, and a pinned Y on any pane of the strip is what the
    // "manual range" state means to a reader either way.
    results.analysis_strip_axis_is_pinned(super::ResultViewer::Waves, key.analysis, axis)
}

/// Pin the active pane's axis to an explicit interval, or clear it.
///
/// X belongs to the strip, not the pane: panes of one analysis always share
/// one abscissa, so an explicit X writes through every pane the way a drag
/// on the shared strip does. Y is the pane's own, because each pane carries
/// its own unit and one interval across volts and amps would mean nothing.
pub(crate) fn set_active_pane_axis_range(
    tokens: &Tokens,
    state: &mut AppState,
    axis: super::PaneAxis,
    range: Option<(f64, f64)>,
) -> bool {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return false;
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return false;
    };
    let panes = model.unit_panes();
    let Some(ordinal) = panes.iter().position(|pane| pane.unit == key.unit) else {
        return false;
    };
    match axis {
        super::PaneAxis::X => {
            set_shared_x_view(&mut state.ui.results, key.analysis, panes.len(), range);
        }
        super::PaneAxis::Y => {
            state
                .ui
                .results
                .analysis_plot_view_pane_mut(super::ResultViewer::Waves, key.analysis, ordinal)
                .y = range;
        }
    }
    true
}

fn active_pane_viewports(
    tokens: &Tokens,
    state: &mut AppState,
) -> (Option<String>, Option<String>) {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return (None, None);
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return (None, None);
    };
    let panes = model.unit_panes();
    let Some((ordinal, pane)) = panes
        .iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == key.unit)
    else {
        return (None, None);
    };
    let x = model.x_range.map(|full| {
        let (x0, x1) = shared_x_view(&state.ui.results, key.analysis, panes.len()).unwrap_or(full);
        format!(
            "{} … {}",
            model.format_x(x0, digits, quantity_policy),
            model.format_x(x1, digits, quantity_policy)
        )
    });
    let pinned = state
        .ui
        .results
        .analysis_plot_view_pane(super::ResultViewer::Waves, key.analysis, ordinal)
        .y;
    let unit = pane.unit;
    let y = match pinned {
        Some(range) => Some(range),
        None => displayed_pane_auto_y(state, model, pane, ordinal, tokens),
    }
    .map(|(y0, y1)| {
        format!(
            "{} … {}",
            fmt_in_unit(y0, unit, digits),
            fmt_in_unit(y1, unit, digits)
        )
    });
    (x, y)
}

fn pane_log_y_key(model: &StripModel, pane: &UnitPane) -> WavePanePresentationKey {
    WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    }
}

pub(super) fn pane_log_y(results: &ResultsState, model: &StripModel, pane: &UnitPane) -> bool {
    results.log_y_panes.contains(&pane_log_y_key(model, pane))
}

fn set_pane_log_y(results: &mut ResultsState, model: &StripModel, pane: &UnitPane, enabled: bool) {
    let key = pane_log_y_key(model, pane);
    if enabled {
        results.log_y_panes.insert(key);
    } else {
        results.log_y_panes.remove(&key);
    }
}

fn trace_belongs_to_pane(
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    trace: &StripTrace,
) -> bool {
    if !trace.kind.is_phase() {
        return model.trace_unit(trace) == pane.unit;
    }
    let has_magnitude = model
        .traces
        .iter()
        .any(|candidate| !candidate.kind.is_phase() && model.trace_unit(candidate) == "dB");
    if has_magnitude {
        pane.unit == "dB"
    } else {
        ordinal == 0
    }
}

#[derive(Default)]
struct UnitPaneHeaderResponse {
    autoscale_y: bool,
    toggle_log_y: bool,
}

fn show_unit_pane_header(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    log_y: bool,
    log_y_available: bool,
    height: f32,
) -> UnitPaneHeaderResponse {
    // A legend chip names the conductor the design drew; the trace keeps the
    // engine's own name as its identity and as the key it is exported under.
    let notations = bus_notations(&state.workspace, &state.schematic);
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = height.clamp(0.0, WAVE_PANE_HEADER_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    if height < 18.0 {
        return UnitPaneHeaderResponse::default();
    }

    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let active = state.ui.results.active_wave_pane.as_ref() == Some(&pane_key);
    let action_width = 58.0;
    // Fit the pane's actual unit tag: nV/√Hz is wider than the quantity tags
    // the old fixed 46 px assumed, and a clipped unit misreads as a new unit.
    let unit_label_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                pane.unit.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text,
            )
            .size()
            .x
    });
    let unit_width = (unit_label_width + 28.0).max(46.0);
    let unit_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 5.0, rect.top() + 1.5),
        egui::pos2(
            (rect.left() + unit_width).min(rect.right()),
            rect.bottom() - 1.5,
        ),
    );
    let actions_rect = egui::Rect::from_min_max(
        egui::pos2(
            (rect.right() - action_width).max(unit_rect.right()),
            rect.top(),
        ),
        rect.right_bottom(),
    );
    let legend_rect = egui::Rect::from_min_max(
        egui::pos2(unit_rect.right() + 3.0, rect.top()),
        egui::pos2(
            (actions_rect.left() - 3.0).max(unit_rect.right() + 3.0),
            rect.bottom(),
        ),
    );

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(unit_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(unit_rect);
            let response = chip(ui, pane.unit, active)
                .on_hover_text(format!("{} unit-scoped Y axis", pane.unit));
            if response.clicked() {
                state.ui.results.active_wave_pane = Some(pane_key.clone());
            }
        },
    );

    // Cursor A only reads out on the strip it was placed on; another strip's
    // chips must not imply a value at an X they never sampled.
    let cursor_a_value = (state.ui.results.cursor_readout_active()
        && state.ui.results.cursor_strip == Some(model.analysis_index))
    .then(|| {
        state.ui.results.cursors.a.map(|x| {
            (
                x,
                state.ui.preferences.result_presentation_policy(),
                state.ui.preferences.quantity_presentation_policy(),
            )
        })
    })
    .flatten();

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(legend_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(legend_rect);
            egui::ScrollArea::horizontal()
                .id_salt(("rspice.results.pane-legend", model.analysis_key, pane.unit))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    let traces = model
                        .traces
                        .iter()
                        .take(model.signal_trace_count)
                        .enumerate()
                        .filter(|(_, trace)| trace_belongs_to_pane(model, pane, ordinal, trace));
                    for (_, trace) in traces {
                        let selected = state
                            .ui
                            .results
                            .valid_selected_trace(&state.simulation)
                            .is_some_and(|selected| {
                                selected.analysis_key() == model.analysis_key
                                    && selected.source_name() == trace.source_waveform_name
                            });
                        let (swatch, swatch_response) =
                            ui.allocate_exact_size(egui::vec2(13.0, 19.0), egui::Sense::click());
                        ui.painter().hline(
                            egui::Rangef::new(swatch.left() + 1.0, swatch.right() - 1.0),
                            swatch.center().y,
                            egui::Stroke::new(2.0, trace.color),
                        );
                        swatch_response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                true,
                                trace.visible,
                                format!("Toggle {} visibility", notations.display(&trace.name)),
                            )
                        });
                        theme::paint_focus_ring(ui, &swatch_response, swatch);
                        if swatch_response.clicked() {
                            if let Some(key) = trace.family_visibility_key {
                                state.ui.results.toggle_family_trace_visibility(key);
                            } else {
                                toggle_visibility(
                                    state,
                                    model.analysis_index,
                                    trace.waveform_index,
                                );
                            }
                        }
                        // The instrument idiom: a trace states its own value
                        // at cursor A right where its name is, so reading one
                        // curve never costs a trip to the readout register.
                        // A corner family draws one chip for the whole group,
                        // so the chip states how many traces it stands for.
                        let family = trace.family_group_ordinal.map(|_| {
                            model
                                .traces
                                .iter()
                                .filter(|candidate| {
                                    candidate.presentation_key == trace.presentation_key
                                        && candidate.family_group_ordinal.is_some()
                                })
                                .count()
                        });
                        let shown = notations.display(&trace.name);
                        let label = match &cursor_a_value {
                            Some((x, presentation, policy)) if trace.visible => {
                                let digits =
                                    usize::from(presentation.displayed_significant_digits().get());
                                let value = sample_at_with_shape(
                                    &trace.x,
                                    &trace.y,
                                    &trace.shape,
                                    *x,
                                    cursor_interpolation(presentation.cursor_interpolation()),
                                );
                                format!(
                                    "{}  {}",
                                    elide(&shown, 16),
                                    model.format_trace_value(trace, value, digits, *policy)
                                )
                            }
                            _ => elide(&shown, 20),
                        };
                        let label = match family {
                            Some(count) if count > 1 => format!("{label}  ×{count}"),
                            _ => label,
                        };
                        // A hidden trace keeps its chip so it can be brought
                        // back, but must not read as a curve on the canvas.
                        let label = if trace.visible {
                            egui::RichText::new(label)
                        } else {
                            egui::RichText::new(label)
                                .strikethrough()
                                .color(c.text_faint)
                        };
                        if ui
                            .selectable_label(selected, label)
                            .on_hover_text(&*shown)
                            .clicked()
                        {
                            state.ui.results.selected_trace =
                                Some(SelectedResultTrace::from_identity(
                                    model.analysis_key,
                                    trace.source_waveform_name.clone(),
                                ));
                            state.ui.results.active_wave_pane = Some(pane_key.clone());
                        }
                    }
                    ui.menu_button("+", |ui| {
                        let mut any = false;
                        for trace in
                            model
                                .traces
                                .iter()
                                .take(model.signal_trace_count)
                                .filter(|trace| {
                                    !trace.visible
                                        && trace_belongs_to_pane(model, pane, ordinal, trace)
                                })
                        {
                            any = true;
                            if ui.button(&*notations.display(&trace.name)).clicked() {
                                if let Some(key) = trace.family_visibility_key {
                                    state.ui.results.toggle_family_trace_visibility(key);
                                } else {
                                    toggle_visibility(
                                        state,
                                        model.analysis_index,
                                        trace.waveform_index,
                                    );
                                }
                                ui.close();
                            }
                        }
                        if !any {
                            ui.label("All compatible signals are already shown");
                        }
                    })
                    .response
                    .on_hover_text(format!("Add a signal to the {} pane", pane.unit));
                });
        },
    );

    let mut output = UnitPaneHeaderResponse::default();
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(actions_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(actions_rect);
            let log = ui
                .add_enabled_ui(log_y_available, |ui| chip(ui, "log", log_y))
                .inner
                .on_hover_text(if log_y_available {
                    "Toggle logarithmic Y axis"
                } else {
                    "Logarithmic Y requires strictly positive visible values"
                });
            if log.clicked() {
                output.toggle_log_y = true;
            }
            if IconButton::new(Icon::ZoomFit)
                .side(19.0)
                .tooltip("Autoscale Y to visible traces")
                .show(ui)
                .clicked()
            {
                output.autoscale_y = true;
            }
        },
    );
    output
}

/// One shared X-axis and viewport navigator for every unit pane in an
/// analysis strip. The plots retain X grid/interactions but never repeat the
/// labeled axis, so vertical grids remain aligned and readable.
fn show_shared_x_axis(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    full_domain: (f64, f64),
    pane_count: usize,
    height: f32,
    linked_cursor_domain: Option<&CursorDomain>,
) {
    if height <= 1.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let current =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or(full_domain);
    let axis = model_x_axis(
        model,
        current.0,
        current.1,
        state.ui.preferences.quantity_presentation_policy(),
    );
    let cursor_owner = state.ui.results.cursor_strip == Some(model.analysis_index);
    let linked_cursor =
        state.ui.results.linked_cursors && linked_cursor_domain == Some(&model.cursor_domain());
    let cursor_values = (cursor_owner || linked_cursor).then_some(state.ui.results.cursors);
    let cursor_summary = cursor_values.map_or_else(
        || "No A/B cursors on this axis".to_owned(),
        |cursors| {
            let a = cursors.a.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            let b = cursors.b.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            format!("Cursor A {a}; cursor B {b}")
        },
    );
    let accessibility_label = format!(
        "Shared {} axis. Current range {} to {}. Full retained range {} to {}. {}. Drag the viewport window to pan or either of its edge handles to resize it, click the overview to recenter, use the wheel to zoom at the pointer, drag A or B to move a cursor, and press F to fit.",
        model.x_label(),
        axis.format_display_value(current.0),
        axis.format_display_value(current.1),
        axis.format_display_value(full_domain.0),
        axis.format_display_value(full_domain.1),
        cursor_summary,
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click_and_drag(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, true, accessibility_label.clone())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GraphicsDocument);
        node.set_label(accessibility_label.clone());
    });
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));

    let plot_left = (rect.left() + wave_left_margin(&state.ui.results)).min(rect.right());
    let plot_right = (rect.right() - WAVE_SHARED_RIGHT_MARGIN).max(plot_left);
    let label_top = rect.top() + SHARED_X_LABEL_TOP;
    let flag_top = rect.top() + SHARED_X_FLAG_TOP;
    let flag_bottom = flag_top + SHARED_X_FLAG_HEIGHT;
    // The lane spans exactly the panes' plot area, so the overview sits
    // under the traces it mirrors and its viewport window maps 1:1 to what
    // the panes show. Both rows name themselves into the panes' own left
    // gutter, which states the thing two stacked X scales otherwise hide:
    // the ticks are the zoomed viewport, the bar below is the full sweep.
    let track = egui::Rect::from_min_max(
        egui::pos2(plot_left, rect.top() + SHARED_X_LANE_TOP),
        egui::pos2(
            plot_right,
            (rect.top() + SHARED_X_LANE_TOP + SHARED_X_LANE_HEIGHT).min(rect.bottom()),
        ),
    );
    let mut viewport = egui::Rect::NOTHING;
    if track.width() > 1.0 {
        painter.rect_filled(track, 0.0, c.canvas_bg);
        painter.rect_stroke(
            track,
            0.0,
            egui::Stroke::new(1.0, c.border),
            egui::StrokeKind::Inside,
        );
        if let Some(trace) = model
            .traces
            .iter()
            .find(|trace| trace.visible && !trace.overlay)
        {
            // The lane's vertical fit is the trace's own finite extremes,
            // which the pane axis above it already holds under this very key.
            // Folding them again here was a second full pass over the
            // retained samples on every frame, for a curve fourteen pixels
            // tall.
            let (minimum, maximum) = state
                .ui
                .results
                .derived
                .range_or(trace_key(model, trace), || super::finite_extremes(&trace.y))
                .unwrap_or((0.0, 1.0));
            let span = (maximum - minimum).max(f64::EPSILON);
            // Index the stride instead of stepping an iterator over it:
            // `step_by` has only `Iterator::nth` to skip with, and a `Zip`
            // has no `nth` of its own, so reaching every 1 562nd sample of a
            // quarter-million-sample sweep visited all 250 000 of them.
            let paired = trace.x.len().min(trace.y.len());
            let stride = (trace.x.len() / SHARED_X_OVERVIEW_POINTS).max(1);
            frame_work::note_samples(FrameSampleRead::StripOverview, paired.div_ceil(stride));
            let points = (0..paired)
                .step_by(stride)
                .filter_map(|index| {
                    let (x, y) = (trace.x[index], trace.y[index]);
                    let fraction = model.x_scale.normalize(x, full_domain.0, full_domain.1);
                    (fraction.is_finite() && y.is_finite()).then(|| {
                        egui::pos2(
                            track.left() + track.width() * fraction as f32,
                            track.bottom()
                                - 2.0
                                - ((y - minimum) / span) as f32 * (track.height() - 4.0),
                        )
                    })
                })
                .collect::<Vec<_>>();
            if points.len() >= 2 {
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(1.0, trace.color.gamma_multiply(0.75)),
                ));
            }
        }

        let (start, end) = shared_axis_viewport_fraction(model.x_scale, full_domain, current);
        let window_left = track.left() + track.width() * start as f32;
        viewport = egui::Rect::from_min_max(
            egui::pos2(window_left, track.top()),
            egui::pos2(
                (track.left() + track.width() * end as f32).max(window_left + 6.0),
                track.bottom(),
            ),
        );
        painter.rect_filled(viewport, 0.0, c.accent.gamma_multiply(0.14));
        painter.rect_stroke(
            viewport,
            0.0,
            egui::Stroke::new(1.0, c.accent),
            egui::StrokeKind::Inside,
        );
        // Grab handles at both edges are the always-visible affordance for
        // resizing the visible span in place: drag an edge in from the full
        // range to zoom, and the opposite edge holds.
        for edge_x in [viewport.left(), viewport.right()] {
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(edge_x - 3.0, track.top() - 1.0),
                    egui::pos2(edge_x + 3.0, track.bottom() + 1.0),
                ),
                2.0,
                c.accent,
            );
            painter.vline(
                edge_x,
                egui::Rangef::new(track.top() + 3.0, track.bottom() - 3.0),
                egui::Stroke::new(1.0, c.canvas_bg),
            );
        }
    }

    // Tick values for the zoomed viewport sit at the top of the strip, right
    // against the panes they describe, and their stubs point up toward the
    // plot.
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut last_right = f32::NEG_INFINITY;
    // Past a certain zoom the tick labels stop being values and become
    // differences from one. This strip draws its own tick row instead of
    // letting a plot draw one, so it also owes the reader that value: a row
    // reading "−40n … 0 … +40n" with nothing to subtract them from states a
    // window somewhere near zero, which is not where the reader is. Stated
    // once, at the head of the row and brighter than the ticks, exactly as
    // the plot's own axis states it.
    if let Some(anchor) = axis.offset_anchor() {
        let galley = painter.layout_no_wrap(anchor.to_owned(), font.clone(), c.text);
        last_right = plot_left + galley.size().x;
        painter.galley(egui::pos2(plot_left, label_top), galley, c.text);
    }
    for (value, label) in &axis.ticks {
        let fraction = model.x_scale.normalize(*value, axis.min, axis.max);
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
            continue;
        }
        let x = plot_left + (plot_right - plot_left) * fraction as f32;
        painter.vline(
            x,
            egui::Rangef::new(rect.top(), rect.top() + 3.0),
            egui::Stroke::new(1.0, c.border_strong),
        );
        let galley = painter.layout_no_wrap(label.clone(), font.clone(), c.text_dim);
        // The stub marks the true position; the label itself is held inside
        // the plot area so an edge value never reaches into the gutter
        // column and collides with the row's own name.
        let half = galley.size().x * 0.5 + 2.0;
        let centre = x.clamp(plot_left + half, (plot_right - half).max(plot_left + half));
        let left = centre - galley.size().x * 0.5;
        if left >= last_right + 6.0 {
            last_right = left + galley.size().x;
            painter.galley(egui::pos2(left, label_top), galley, c.text_dim);
        }
    }
    let gutter_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    painter.text(
        egui::pos2(plot_left - 8.0, label_top + font.size * 0.5),
        egui::Align2::RIGHT_CENTER,
        shared_x_gutter_label(&model.x_unit),
        gutter_font.clone(),
        c.text_faint,
    );
    painter.text(
        egui::pos2(plot_left - 8.0, track.center().y),
        egui::Align2::RIGHT_CENTER,
        "FULL",
        gutter_font,
        c.text_faint,
    );

    let flag_centre = |value: f64| {
        // Flags track the viewport like the pane cursor lines above them,
        // so a cursor zoomed past simply leaves the strip.
        let fraction = model.x_scale.normalize(value, current.0, current.1);
        (fraction.is_finite() && (-0.001..=1.001).contains(&fraction))
            .then(|| plot_left + (plot_right - plot_left) * fraction.clamp(0.0, 1.0) as f32)
    };
    if let Some(cursors) = cursor_values {
        for (label, value, color) in [("A", cursors.a, c.traces[1]), ("B", cursors.b, c.accent)] {
            let Some(value) = value else { continue };
            if let Some(px) = flag_centre(value) {
                let flag = egui::Rect::from_min_max(
                    egui::pos2(px - 8.0, flag_top),
                    egui::pos2(px + 8.0, flag_bottom),
                );
                painter.vline(
                    px,
                    egui::Rangef::new(rect.top(), flag_top),
                    egui::Stroke::new(1.0, color),
                );
                painter.rect_filled(flag, 2.0, color);
                painter.text(
                    flag.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::SemiBold),
                    c.canvas_bg,
                );
            }
            // The overview keeps its own notch at the cursor's absolute
            // source position, so it still locates a cursor the viewport has
            // left behind.
            if track.width() > 1.0 {
                let fraction = model.x_scale.normalize(value, full_domain.0, full_domain.1);
                if fraction.is_finite() {
                    let notch = track.left() + track.width() * fraction.clamp(0.0, 1.0) as f32;
                    painter.vline(
                        notch,
                        egui::Rangef::new(track.top() + 1.0, track.top() + 5.0),
                        egui::Stroke::new(2.0, color),
                    );
                }
            }
        }
    }

    if response.clicked() {
        response.request_focus();
    }
    // The lane band owns the overview gestures and everything above it
    // belongs to the flags, so a flag grab never fights a resize handle.
    let lane_band = egui::Rangef::new(track.top() - 4.0, track.bottom() + 4.0);
    let handle_at = |pointer: egui::Pos2| {
        if track.width() <= 1.0 || !lane_band.contains(pointer.y) {
            return None;
        }
        if (pointer.x - viewport.left()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeStart)
        } else if (pointer.x - viewport.right()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeEnd)
        } else {
            None
        }
    };
    let flag_at = |pointer: egui::Pos2| {
        if pointer.y >= track.top() - 4.0 {
            return None;
        }
        let cursors = cursor_values?;
        [
            (SharedXDrag::CursorA, cursors.a),
            (SharedXDrag::CursorB, cursors.b),
        ]
        .into_iter()
        .find_map(|(drag, value)| {
            let px = flag_centre(value?)?;
            ((pointer.x - px).abs() <= 9.0).then_some(drag)
        })
    };
    let drag_id = shared_x_drag_id(model);
    if response.drag_started()
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let drag = handle_at(pointer)
            .or_else(|| flag_at(pointer))
            .unwrap_or(SharedXDrag::Viewport);
        ui.memory_mut(|memory| memory.data.insert_temp(drag_id, drag));
    }
    let drag = ui.memory(|memory| memory.data.get_temp::<SharedXDrag>(drag_id));
    if response.dragged()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.width() > 1.0
        && let Some(drag) = drag
    {
        let fraction = f64::from(((pointer.x - track.left()) / track.width()).clamp(0.0, 1.0));
        match drag {
            SharedXDrag::Viewport => {
                let delta = ui.ctx().input(|input| input.pointer.delta().x);
                if let Some(range) = panned_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    f64::from(delta / track.width()),
                ) {
                    set_shared_x_view(
                        &mut state.ui.results,
                        model.analysis_key,
                        pane_count,
                        Some(range),
                    );
                }
            }
            SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {
                if let Some(range) = resized_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    drag == SharedXDrag::ResizeStart,
                    fraction,
                ) {
                    set_shared_x_view(
                        &mut state.ui.results,
                        model.analysis_key,
                        pane_count,
                        Some(range),
                    );
                }
            }
            SharedXDrag::CursorA | SharedXDrag::CursorB => {
                // A flag is anchored to what the panes show, so its drag
                // converts through the viewport rather than the full sweep.
                let view_fraction = f64::from(
                    ((pointer.x - plot_left) / (plot_right - plot_left).max(1.0)).clamp(0.0, 1.0),
                );
                let x = model
                    .x_scale
                    .denormalize(view_fraction, current.0, current.1);
                if !cursor_owner && !linked_cursor {
                    state.ui.results.clear_cursors();
                    state.ui.results.cursor_strip = Some(model.analysis_index);
                }
                match drag {
                    SharedXDrag::CursorA => {
                        state.ui.results.cursors.a = Some(x);
                        state.ui.results.cursor_a_anchor = None;
                    }
                    SharedXDrag::CursorB => state.ui.results.cursors.b = Some(x),
                    SharedXDrag::Viewport | SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {}
                }
            }
        }
    }
    if response.drag_stopped() {
        ui.memory_mut(|memory| memory.data.remove::<SharedXDrag>(drag_id));
    }

    if response.hovered()
        && let Some(pointer) = response.hover_pos()
    {
        ui.ctx().set_cursor_icon(
            if handle_at(pointer).is_some() || flag_at(pointer).is_some() {
                egui::CursorIcon::ResizeHorizontal
            } else if lane_band.contains(pointer.y) {
                egui::CursorIcon::Grab
            } else {
                egui::CursorIcon::Default
            },
        );
    }

    if response.clicked()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.contains(pointer)
        && !viewport.contains(pointer)
        && let Some(range) = recentered_shared_x_view(
            model.x_scale,
            full_domain,
            current,
            f64::from((pointer.x - track.left()) / track.width()),
        )
    {
        set_shared_x_view(
            &mut state.ui.results,
            model.analysis_key,
            pane_count,
            Some(range),
        );
    }

    if response.hovered() && track.width() > 1.0 {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll != 0.0
            && let Some(pointer) = response.hover_pos()
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                f64::from((pointer.x - track.left()) / track.width()),
                (f64::from(-scroll) * 0.002).exp(),
            )
        {
            ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
    }

    if response.has_focus() {
        let fit_key =
            ui.input(|input| input.key_pressed(egui::Key::F) || input.key_pressed(egui::Key::Home));
        if fit_key {
            set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
        }
        let zoom_in = ui.input(|input| input.key_pressed(egui::Key::Plus));
        let zoom_out = ui.input(|input| input.key_pressed(egui::Key::Minus));
        if (zoom_in || zoom_out)
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                0.5,
                if zoom_in { 0.8 } else { 1.25 },
            )
        {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
        let direction = ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowLeft) {
                -1.0
            } else if input.key_pressed(egui::Key::ArrowRight) {
                1.0
            } else {
                0.0
            }
        });
        if direction != 0.0
            && let Some(range) =
                panned_shared_x_view(model.x_scale, full_domain, current, direction * 0.05)
        {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(range),
            );
        }
    }

    if response.double_clicked() {
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(
        "Shared X overview — drag the window or its edges, click to recenter, wheel to zoom, drag A/B, F to fit",
    );
}

/// One strip, drawn as one pane per unit.
///
/// Signals route to the pane that owns their unit; the panes stack and
/// share the strip's X domain, so a strip stays one measurement read
/// against as many scales as it genuinely needs.
fn show_strip_plot(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    linked_cursor_domain: Option<&CursorDomain>,
) {
    let t = Tokens::get(ui.ctx());
    let Some(x_domain) = model.x_range else {
        well_hint(ui, "No data");
        return;
    };
    let panes = model.unit_panes();
    if panes.is_empty() {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    }

    // Expression traces participate in the first pane's automatic fit.
    let exprs = resolve_strip_exprs(state, model, &t);
    let available = ui.available_rect_before_wrap();
    let count = panes.len();
    let geometry = wave_stack_geometry(available.height(), count);

    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), geometry.seam_height),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
        }
        let pane_height = geometry.pane_height(ordinal, count);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_height(pane_height);
                show_unit_pane(
                    ui,
                    state,
                    model,
                    pane,
                    ordinal,
                    x_domain,
                    // Only the first pane carries the strip's expressions:
                    // an expression has no declared unit, so it cannot be
                    // routed to a unit-owning pane on the evidence available.
                    if ordinal == 0 { &exprs } else { &[] },
                    linked_cursor_domain,
                    count,
                );
            },
        );
    }
    show_shared_x_axis(
        ui,
        state,
        model,
        x_domain,
        count,
        geometry.shared_x_height,
        linked_cursor_domain,
    );
}

/// The Y interval a pane fits itself to when the reader has not pinned one.
///
/// Everything the pane draws widens it: the traces, whatever expressions the
/// strip carries, and the specification limits — a bound drawn off the top of
/// the axis is a bound the reader cannot check against.
fn pane_auto_y(
    pane_range: Option<(f64, f64)>,
    exprs: &[ResolvedExpr],
    limits: &[plot::LimitLine],
) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    if let Some((a, b)) = pane_range {
        lo = a;
        hi = b;
    }
    for expr in exprs {
        if let Some((a, b)) = expr.y_extremes {
            lo = lo.min(a);
            hi = hi.max(b);
        }
    }
    for limit in limits {
        lo = lo.min(limit.y);
        hi = hi.max(limit.y);
    }
    if !lo.is_finite() || !hi.is_finite() {
        None
    } else if lo == hi && lo > 0.0 {
        Some((lo / 1.1, hi * 1.1))
    } else if lo == hi {
        Some((lo - 1.0, hi + 1.0))
    } else {
        Some((lo, hi))
    }
}

/// The Y interval one pane of a strip is showing right now, automatic fit
/// included.
///
/// The inspector's extents, the axis-limit editor and the zoom control all
/// read this rather than the traces' own extremes: an expression or a
/// specification limit widens the drawn axis, and reporting the narrower
/// interval meant the typed limits opened on numbers that were not on screen.
pub(super) fn displayed_pane_auto_y(
    state: &mut AppState,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    t: &Tokens,
) -> Option<(f64, f64)> {
    let pane_range = pane_y_range(&mut state.ui.results.derived, model, &pane.traces);
    let limits = if state.ui.results.show_spec_limits {
        matching_spec_limits(state, model, pane, t)
    } else {
        Vec::new()
    };
    // Only the strip's first pane draws its expressions; the others must not
    // widen themselves against a curve they do not carry.
    let exprs = if ordinal == 0 {
        resolve_strip_exprs(state, model, t)
    } else {
        Vec::new()
    };
    pane_auto_y(pane_range, &exprs, &limits)
}

/// The active-run trace whose drawn curve passes closest to the pointer.
///
/// Two things make this the trace the reader is pointing at rather than an
/// approximation of it. The value is mapped to the screen through the pane's
/// own scale, which is the mapping the painter used — a linear guess on a
/// decade pane picks a curve the pointer is nowhere near. And a loop is
/// measured on every branch that reaches this abscissa, so clicking the return
/// leg of a hysteresis curve anchors to that curve rather than to whichever
/// neighbour happened to sit near its forward leg.
fn nearest_drawn_trace<'a>(
    pane_traces: &[(usize, &'a StripTrace)],
    x: f64,
    pointer_y: Option<f32>,
    plot_rect: egui::Rect,
    y_scale: XScale,
    (y0, y1): (f64, f64),
    interpolation: SampleInterpolation,
) -> Option<&'a StripTrace> {
    let screen_y = |value: f64| -> Option<f32> {
        let fraction = y_scale.normalize(value, y0, y1);
        (value.is_finite() && fraction.is_finite())
            .then(|| plot_rect.bottom() - fraction as f32 * plot_rect.height())
    };
    let mut samples: Vec<BranchSample> = Vec::new();
    let mut best: Option<(&StripTrace, f32)> = None;
    for (_, trace) in pane_traces.iter().filter(|(_, trace)| !trace.overlay) {
        // The ordinary sweep has one answer here and is spared the branch
        // walk; anything else is measured on every leg that reaches this
        // abscissa, so clicking the return leg of a loop finds that curve.
        let mut values: Vec<f64> = if trace.shape.is_single_ascending() {
            Vec::new()
        } else {
            sample_branches_into(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
                &mut samples,
            );
            samples.iter().map(|sample| sample.value).collect()
        };
        if values.is_empty() {
            values.push(sample_at_with_shape(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
            ));
        }
        for value in values {
            let Some(y) = screen_y(value) else { continue };
            let distance = pointer_y.map_or(0.0, |pointer| (pointer - y).abs());
            if best.is_none_or(|(_, closest)| distance < closest) {
                best = Some((trace, distance));
            }
        }
    }
    best.map(|(trace, _)| trace)
}

fn show_unit_pane(
    ui: &mut Ui,
    state: &mut AppState,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    x_domain: (f64, f64),
    exprs: &[ResolvedExpr],
    linked_cursor_domain: Option<&CursorDomain>,
    pane_count: usize,
) {
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let (x0, x1) = x_domain;
    // The pane's own top edge, kept so the active rail can span header and
    // canvas together once the pane's full height is known.
    let pane_top = ui.available_rect_before_wrap().top();

    let pane_range = pane_y_range(&mut state.ui.results.derived, model, &pane.traces);
    let specification_limits = if state.ui.results.show_spec_limits {
        matching_spec_limits(state, model, pane, &t)
    } else {
        Vec::new()
    };
    let auto_y = pane_auto_y(pane_range, exprs, &specification_limits);
    let log_y_available = auto_y.is_some_and(|(minimum, maximum)| minimum > 0.0 && maximum > 0.0);
    let mut log_y = pane_log_y(&state.ui.results, model, pane) && log_y_available;
    if !log_y_available && log_y {
        set_pane_log_y(&mut state.ui.results, model, pane, false);
    }
    let header = show_unit_pane_header(
        ui,
        state,
        model,
        pane,
        ordinal,
        log_y,
        log_y_available,
        WAVE_PANE_HEADER_HEIGHT.min(ui.available_height()),
    );
    if header.autoscale_y || header.toggle_log_y {
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    }
    if header.toggle_log_y {
        log_y = !log_y;
        set_pane_log_y(&mut state.ui.results, model, pane, log_y);
    }

    let Some((auto_y0, auto_y1)) = auto_y else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };
    if ui.available_height() < WAVE_MIN_PLOT_HEIGHT {
        return;
    }

    // User zoom/pan overrides the automatic fit per axis, per pane.
    let pane_view = state.ui.results.analysis_plot_view_pane(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let (x0, x1) =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or((x0, x1));
    let (mut y0, mut y1) = pane_view
        .y
        .filter(|(minimum, maximum)| !log_y || (*minimum > 0.0 && *maximum > 0.0))
        .unwrap_or((auto_y0, auto_y1));

    let x_axis = model_x_axis(model, x0, x1, quantity_policy);
    let family_envelopes = state.ui.results.show_family_envelope.then(|| {
        let generation = state.ui.results.models.generation();
        extent::family_envelopes(&mut state.ui.results, generation, model, pane)
    });
    let family_envelopes: &[extent::FamilyEnvelopeSeries] = family_envelopes
        .as_deref()
        .map_or(&[], FamilyEnvelopePlan::series);
    let y_axis = if log_y {
        Axis::log_decades(y0, y1, pane.unit)
    } else if pane.unit == "°" && pane_view.y.is_none() {
        // An unzoomed degree pane keeps the 45° lattice a Bode phase
        // reading expects; arbitrary zoom depths fall back to plain linear
        // ticks, which stay legible where the lattice would crowd.
        y0 = (y0 / 45.0).floor() * 45.0;
        y1 = (y1 / 45.0).ceil() * 45.0;
        // Continuous phase does not stay inside one turn: an unwrapped loop
        // response walks thousands of degrees, and 45° steps across it are
        // thousands of labels stacked into an unreadable band — and thousands
        // of galleys laid out every frame. The lattice thins by whole 45°
        // multiples so what is left still falls on the values a phase reading
        // is taken at.
        let steps = ((y1 - y0) / 45.0).round().max(0.0) as usize;
        let stride = (steps + 1).div_ceil(MAX_AXIS_TICKS).max(1);
        let ticks: Vec<f64> = (0..=steps)
            .step_by(stride)
            .map(|index| 45.0f64.mul_add(index as f64, y0))
            .collect();
        Axis::with_ticks(y0, y1, "°", &ticks)
    } else {
        Axis::linear(y0, y1, pane.unit)
    };
    let y_axis = if pane.unit == "rad" {
        match quantity_policy.angle_display {
            crate::quantity::AngleDisplay::Degrees => {
                y_axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
            }
            crate::quantity::AngleDisplay::Radians => y_axis,
        }
    } else if pane.unit == "°" {
        let (scale, offset, unit) = quantity_policy.degree_axis_transform();
        y_axis.with_display_transform(scale, offset, unit)
    } else {
        y_axis
    };
    // The scale the pane is actually drawn on. Every hit test below maps
    // through it rather than assuming a linear ordinate, because "nearest"
    // has to mean nearest on screen — and on a decade pane a linear guess is
    // wrong by most of the window.
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    // The plot's own description is where the caution has to live as words.
    // The kind tag carries it as colour and a glyph, and neither of those
    // reaches a reader who cannot see the strip.
    let mut spec = PlotSpec::new(x_axis, model.x_scale, y_axis)
        .accessible_name("Waveform plot")
        .without_x_axis_chrome()
        .with_right_margin(WAVE_SHARED_RIGHT_MARGIN);
    if let Some(reason) = model.incomplete {
        spec = spec.accessible_detail(reason);
    }
    spec.left_margin = wave_left_margin(&state.ui.results);
    if log_y {
        spec = spec.with_log_y();
    }
    spec.display_decimation = display_decimation(presentation.large_dataset_display());
    spec.limit_lines = specification_limits;
    spec.minor_grid = state.ui.results.show_minor_grid;
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    spec.horizontal_cursor = state
        .ui
        .results
        .horizontal_cursor
        .as_ref()
        .filter(|cursor| cursor.pane == pane_key)
        .map(|cursor| cursor.y);
    spec.horizontal_cursor_interactive = state.ui.results.horizontal_cursor_placement_enabled();

    // 0 dB reference on a log-magnitude pane.
    if pane.unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Family envelopes are derived only from exact shared X coordinates.
    // They draw behind source curves and never interpolate missing family
    // samples into evidence that was not retained.
    for envelope in family_envelopes {
        let mut minimum = Trace::new(&envelope.x, &envelope.minimum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.minimum_cache_key);
        let mut maximum = Trace::new(&envelope.x, &envelope.maximum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.maximum_cache_key);
        if envelope.x.len() == 1 {
            minimum = minimum.show_single_point();
            maximum = maximum.show_single_point();
        }
        spec.traces.push(minimum);
        spec.traces.push(maximum);
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let pane_traces: Vec<(usize, &StripTrace)> = pane
        .traces
        .iter()
        .copied()
        .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
        .collect();
    let draw_order = pane_traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(pane_traces.iter().filter(|(_, trace)| !trace.overlay));
    for (_, trace) in draw_order {
        let color = if trace.overlay {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        // The reduction has to be told what the abscissa is. Without it a
        // reverse sweep vanished the moment it was zoomed — every window it
        // was asked for came back empty — and a hysteresis loop lost whichever
        // branch fell outside one contiguous index window.
        let mut plot_trace = apply_family_trace_style(
            Trace::new(&trace.x, &trace.y, color)
                .cache_key(trace_key(model, trace))
                .shape(&trace.shape),
            trace.family_style,
        );
        if trace.overlay {
            plot_trace = plot_trace.thin();
        }
        spec.traces.push(plot_trace);
    }
    for expr in exprs {
        spec.traces.push(apply_family_trace_style(
            Trace::new(&expr.x, &expr.y, expr.color)
                .thin()
                .cache_key(expr.cache_key)
                .shape(&expr.shape),
            expr.family_style,
        ));
    }

    // Markers ride their anchored trace: Y is resampled here rather than
    // stored, so zoom, pan, and a re-run all leave the tag on the curve.
    for marker in state.ui.results.strip_markers(model.analysis_key) {
        let color = marker_color(marker.kind(), &t);
        let label = marker_label(marker);
        if marker.kind() == MarkerKind::Spec {
            // A spec constrains the X position, which every pane of the
            // strip shares — so it draws on all of them.
            spec.markers
                .push(plot::Marker::limit_line(marker.x(), color, label));
            continue;
        }
        // A marker belongs to the pane that owns its trace's unit; the
        // other panes are a different scale and would misplace it.
        let anchored = pane_traces
            .iter()
            .find(|(_, trace)| !trace.overlay && anchor_key(model, trace) == *marker.anchor());
        let Some((_, trace)) = anchored else {
            continue;
        };
        // A loop has a value on each branch that reaches this X, and a tag on
        // only one of them points at half the evidence.
        let mut samples = Vec::new();
        sample_branches_into(
            &trace.x,
            &trace.y,
            &trace.shape,
            marker.x(),
            interpolation,
            &mut samples,
        );
        if trace.shape.branch_count() <= 1 || samples.is_empty() {
            let y =
                sample_at_with_shape(&trace.x, &trace.y, &trace.shape, marker.x(), interpolation);
            if y.is_finite() {
                spec.markers
                    .push(plot::Marker::point(marker.x(), y, color, label));
            }
            continue;
        }
        for sample in &samples {
            if !sample.value.is_finite() {
                continue;
            }
            spec.markers.push(plot::Marker::point(
                marker.x(),
                sample.value,
                color,
                format!("{label} {}", branch_tag(&trace.shape, sample.run)),
            ));
        }
    }

    let model_cursor_domain = model.cursor_domain();
    let cursor_domain_matches = linked_cursor_domain == Some(&model_cursor_domain);
    let cursors = (state.ui.results.cursor_strip == Some(model.analysis_index)
        || (state.ui.results.linked_cursors && cursor_domain_matches))
        .then_some(state.ui.results.cursors);

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        let mut samples: Vec<BranchSample> = Vec::new();
        for (_, trace) in pane_traces.iter().take(6) {
            sample_branches_into(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
                &mut samples,
            );
            // A sweep with one answer at this X keeps its single unlabelled
            // row. A loop reports each branch that reaches here, because one
            // of the two numbers is not the reading.
            if trace.shape.branch_count() <= 1
                || samples.is_empty()
                || samples.len() > MAX_READOUT_BRANCHES
            {
                let value =
                    sample_at_with_shape(&trace.x, &trace.y, &trace.shape, x, interpolation);
                rows.push((
                    trace.name.clone(),
                    model.format_trace_value(trace, value, significant_digits, quantity_policy),
                ));
                continue;
            }
            for sample in &samples {
                rows.push((
                    format!("{} {}", trace.name, branch_tag(&trace.shape, sample.run)),
                    model.format_trace_value(
                        trace,
                        sample.value,
                        significant_digits,
                        quantity_policy,
                    ),
                ));
            }
        }
        for expr in exprs.iter().take(3) {
            let value = sample_at_with_shape(&expr.x, &expr.y, &expr.shape, x, interpolation);
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
    if response.response.hovered()
        || response.response.dragged()
        || response.clicked_x.is_some()
        || response.horizontal_cursor_y.is_some()
    {
        state.ui.results.active_wave_pane = Some(pane_key.clone());
    }
    if let Some(y) = response.horizontal_cursor_y {
        state.ui.results.horizontal_cursor = Some(HorizontalWaveCursor { pane: pane_key, y });
    }

    // The marker tool takes the click when armed: one click cannot both
    // annotate and move a cursor, and the armed chip says which it will do.
    if let Some(clicked_x) = response.clicked_x
        && state.ui.results.marker_tool.is_armed()
    {
        let pointer_y = response.response.interact_pointer_pos().map(|pos| pos.y);
        let plot_rect = response.plot_rect;
        let nearest = nearest_drawn_trace(
            &pane_traces,
            clicked_x,
            pointer_y,
            plot_rect,
            y_scale,
            (y0, y1),
            interpolation,
        );
        if let Some(trace) = nearest {
            // The pane being drawn owns the marker, and placing one is the
            // first half of saying what it means, so the dialog opens on it.
            let placement = super::MarkerPlacement {
                analysis: model.analysis_key,
                anchor: anchor_key(model, trace),
                trace_name: trace.name.clone(),
                x: clicked_x,
                samples: trace.x.as_slice(),
            };
            if let Some(selector) = super::place_marker(state, placement) {
                marker_dialog::open(state, selector);
            }
        }
    } else if let Some(clicked_x) = response.clicked_x
        && state.ui.results.cursor_placement_enabled()
    {
        let placing_cursor_a = state.ui.results.cursor_a_is_next();
        let pointer_y = response
            .response
            .interact_pointer_pos()
            .map(|position| position.y);
        let nearest_anchor = placing_cursor_a.then(|| {
            nearest_drawn_trace(
                &pane_traces,
                clicked_x,
                pointer_y,
                response.plot_rect,
                y_scale,
                (y0, y1),
                interpolation,
            )
            .map(|trace| anchor_key(model, trace))
        });
        let results = &mut state.ui.results;
        if results.cursor_strip != Some(model.analysis_index)
            && (!results.linked_cursors || !cursor_domain_matches)
        {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        if placing_cursor_a {
            results.cursor_a_anchor = nearest_anchor.flatten();
        }
        results.cursors.place(clicked_x);
    }

    if response.view.reset {
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    } else if response.view.any() {
        if let Some(x) = response.view.x {
            set_shared_x_view(
                &mut state.ui.results,
                model.analysis_key,
                pane_count,
                Some(x),
            );
        }
        let view = state.ui.results.analysis_plot_view_pane_mut(
            super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        if let Some(y) = response.view.y {
            view.y = Some(y);
        }
    }

    // The mockup's `.plot-pane.active::before`: a 2 px rail down the pane
    // that received the instrument's actions. Painted last so it reads over
    // the header fill and the canvas alike.
    let pane_active = state.ui.results.active_wave_pane.as_ref()
        == Some(&WavePanePresentationKey {
            analysis: model.analysis_key,
            unit: pane.unit.to_owned(),
        });
    if pane_active {
        let bottom = ui.min_rect().bottom().max(pane_top);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(ui.min_rect().left(), pane_top),
                egui::pos2(ui.min_rect().left() + 2.0, bottom),
            ),
            0.0,
            t.color.accent,
        );
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// readout strip
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
