//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

#[cfg(test)]
use super::{MarkerKind, TracePresentationKey};
#[cfg(test)]
use crate::ui::plot::fmt_significant;
use rspice_results_ui::derived::DerivedSeries;
use rspice_results_ui::presentation::well_hint;
#[cfg(test)]
use rspice_results_ui::session::MarkerView;
use rspice_results_ui::waves::cursor_interpolation;
use rspice_results_ui::waves::header::{self as pane_header, WAVE_PANE_HEADER_HEIGHT, elide};
use rspice_results_ui::waves::navigation::{
    self, WAVE_SHARED_X_HEIGHT, shared_axis_viewport_fraction,
};
use rspice_results_ui::waves::pane::{self as pane_view, pane_auto_y};
#[cfg(test)]
use rspice_results_ui::waves::pane::{apply_family_trace_style, nearest_drawn_trace};
use rspice_results_ui::waves::{
    CursorDomain, NOISE_DENSITY_UNIT, StripTrace, TraceKind, anchor_key, family_color, fmt_in_unit,
    trace_key,
};
pub(super) use rspice_results_ui::waves::{FamilyTraceVisibilityKey, StripModel, UnitPane};
pub(crate) use rspice_results_ui::waves::{
    analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};
mod expression_evaluation;
mod expressions;
use rspice_results_ui::waves::extent::{self, FamilyEnvelopePlan};
pub(super) mod marker_dialog;
mod model_cache;
pub(super) use model_cache::cached_models;
mod readout;
mod viewport;

use expression_evaluation::{evaluate_expression, evaluate_expression_with_policy};
pub(crate) use expressions::*;
pub(crate) use readout::*;
pub(crate) use viewport::*;

use std::collections::HashSet;
#[cfg(test)]
use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::schematic::bus_notations;
#[cfg(test)]
use crate::state::AnalysisType;
use crate::state::{AnalysisResult, SimulationState};
#[cfg(test)]
use crate::ui::plot::Trace;
use crate::ui::plot::sample::{SweepShape, sample_at_with_shape};
use crate::ui::plot::{
    self, CursorPair, DisplayDecimation, SampleInterpolation, XScale, fmt_si_significant,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::AppState;
use crate::workbench::{ComplexNumberDisplay, LargeDatasetDisplay};
use rspice_results::family_projection::SourceSampleSelection;
#[cfg(test)]
use rspice_results_ui::waves::navigation::model_x_axis;

use super::frame_work::{self, FrameSampleRead};
use super::{
    AnalysisPresentationKey, ExprEditor, ExprSeries, ExprTrace, ExpressionSeriesResult,
    ExpressionSource, ExpressionWaveform, HorizontalWaveCursor, MarkerSelector, ResultsState,
    SelectedResultTrace, SourceWaveformPresentationKey, WavePanePresentationKey,
    WaveformPresentationKey,
};
use rspice_results_ui::strip::{LegendChip, StripHeader};

// Mockup gutter geometry: a 64 px left gutter carries the Y ticks and the
// X-strip's band labels; the right edge keeps only a 14 px breathing strip
// now that no pane owns a secondary axis.
const WAVE_SHARED_LEFT_MARGIN: f32 = 64.0;

/// The mockup's per-sheet left gutter: the noise sheet's nV/√Hz tick
/// labels need 88 px where the shared 64 px gutter suffices elsewhere.
fn wave_left_margin(results: &ResultsState) -> f32 {
    match results.session.viewer {
        super::ResultViewer::NoiseContrib => 88.0,
        _ => WAVE_SHARED_LEFT_MARGIN,
    }
}
const WAVE_MIN_PLOT_HEIGHT: f32 = 24.0;

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
    match results.session.maximized_strip {
        Some(maximized) if models.iter().any(|item| item.analysis_key == maximized) => {
            model.analysis_key == maximized
        }
        _ => !results.session.hidden_strips.contains(&model.analysis_key),
    }
}

fn active_pane<'a>(
    models: &'a [StripModel],
    results: &ResultsState,
) -> Option<(&'a StripModel, usize, UnitPane<'a>)> {
    let active = results.session.active_wave_pane.as_ref()?;
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
            state.ui.results.session.cursor_strip.and_then(|index| {
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
    state.ui.results.session.active_wave_pane = next;
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
    !extent::family_envelopes(
        &mut state.ui.results.plans.envelopes,
        generation,
        model,
        &pane,
    )
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
        .session
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
        .session
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
    let visible: Vec<&StripModel> = match results.session.maximized_strip {
        Some(max_key) if models.iter().any(|m| m.analysis_key == max_key) => models
            .iter()
            .filter(|m| m.analysis_key == max_key)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.session.hidden_strips.contains(&m.analysis_key))
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
    let maximized = state.ui.results.session.maximized_strip.is_some();
    let linked_cursor_domain = state
        .ui
        .results
        .session
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
                            .session
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

                        let zoomed = state.ui.results.session.analysis_strip_is_zoomed(
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
        results.session.maximized_strip =
            (results.session.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.session.hidden_strips.insert(idx);
        if models
            .iter()
            .find(|model| model.analysis_key == idx)
            .is_some_and(|model| results.session.cursor_strip == Some(model.analysis_index))
        {
            results.session.clear_cursors();
        }
    }
    if let Some(key) = fit_strip {
        results
            .session
            .reset_analysis_plot_view(super::ResultViewer::Waves, key);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .session
            .analysis_exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results
                .session
                .sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = results.session.analysis_exprs.get_mut(&analysis)
    {
        if index < list.len() {
            let removed = list.remove(index);
            results
                .analysis_expr_cache
                .remove(&(analysis, removed.text));
        }
        if list.is_empty() {
            results.session.analysis_exprs.remove(&analysis);
        }
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results
                .session
                .sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some(analysis) = open_editor {
        results.session.expr_editor = Some(ExprEditor {
            analysis,
            text: String::new(),
            error: None,
            want_focus: true,
        });
    }
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
        .session
        .active_wave_pane
        .is_some()
        .then(|| active_pane_is_pinned(tokens, state));
    let key = state.ui.results.session.active_wave_pane.as_ref();
    let scale = key.map(|key| {
        // One owner: the pane’s own log-Y flag on ResultsState. This read
        // used to reach into egui’s persisted memory with a hand-built id,
        // which is a second copy of a fact and drifts the moment either side
        // changes its key.
        if state.ui.results.session.log_y_panes.contains(key) {
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
        limit_mask: if state.ui.results.session.show_spec_limits {
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
        .session
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
    let Some(key) = state.ui.results.session.active_wave_pane.clone() else {
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
    let Some(key) = state.ui.results.session.active_wave_pane.clone() else {
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
        .session
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
    let Some(key) = results.session.active_wave_pane.as_ref() else {
        return false;
    };
    // Resolving the pane ordinal would need the built models, which this
    // accessor deliberately does not take. It does not need them: X is shared
    // by the whole strip, and a pinned Y on any pane of the strip is what the
    // "manual range" state means to a reader either way.
    results
        .session
        .analysis_strip_axis_is_pinned(super::ResultViewer::Waves, key.analysis, axis)
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
    let Some(key) = state.ui.results.session.active_wave_pane.clone() else {
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
                .session
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
    let Some(key) = state.ui.results.session.active_wave_pane.clone() else {
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
        .session
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
    results
        .session
        .log_y_panes
        .contains(&pane_log_y_key(model, pane))
}

fn set_pane_log_y(results: &mut ResultsState, model: &StripModel, pane: &UnitPane, enabled: bool) {
    let key = pane_log_y_key(model, pane);
    if enabled {
        results.session.log_y_panes.insert(key);
    } else {
        results.session.log_y_panes.remove(&key);
    }
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
) -> pane_header::UnitPaneHeaderResponse {
    // Design notation changes labels only; retained source names remain identities.
    let notations = bus_notations(&state.workspace, &state.schematic);
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    // Cursor A only reads out on the strip it was placed on.
    let cursor_a = (state.ui.results.session.cursor_readout_active()
        && state.ui.results.session.cursor_strip == Some(model.analysis_index))
    .then(|| {
        state.ui.results.session.cursors.a.map(|x| {
            (
                x,
                state.ui.preferences.result_presentation_policy().readout(),
                state.ui.preferences.quantity_presentation_policy(),
            )
        })
    })
    .flatten();
    let input = pane_header::PaneHeader {
        height,
        active: state.ui.results.session.active_wave_pane.as_ref() == Some(&pane_key),
        log_y,
        log_y_available,
        cursor_a,
    };
    struct HeaderHost<'a> {
        state: &'a mut AppState,
        model: &'a StripModel,
        pane_key: WavePanePresentationKey,
    }
    impl pane_header::PaneHeaderHost for HeaderHost<'_> {
        fn trace_selected(&self, trace: &StripTrace) -> bool {
            self.state
                .ui
                .results
                .valid_selected_trace(&self.state.simulation)
                .is_some_and(|selected| {
                    selected.analysis_key() == self.model.analysis_key
                        && selected.source_name() == trace.source_waveform_name
                })
        }

        fn toggle_trace_visibility(&mut self, trace: &StripTrace) {
            if let Some(key) = trace.family_visibility_key {
                self.state.ui.results.toggle_family_trace_visibility(key);
            } else {
                toggle_visibility(self.state, self.model.analysis_index, trace.waveform_index);
            }
        }

        fn select_trace(&mut self, trace: &StripTrace) {
            self.state.ui.results.session.selected_trace =
                Some(SelectedResultTrace::from_identity(
                    self.model.analysis_key,
                    trace.source_waveform_name.clone(),
                ));
            self.activate_pane();
        }

        fn activate_pane(&mut self) {
            self.state.ui.results.session.active_wave_pane = Some(self.pane_key.clone());
        }
    }
    pane_header::show_header(
        ui,
        model,
        pane,
        ordinal,
        input,
        |name| notations.display(name),
        &mut HeaderHost {
            state,
            model,
            pane_key,
        },
    )
}

/// Resolve source ownership, then apply the navigator's actions to the app.
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
    let current =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or(full_domain);
    let cursor_owner = state.ui.results.session.cursor_strip == Some(model.analysis_index);
    let linked_cursor = state.ui.results.session.linked_cursors
        && linked_cursor_domain == Some(&model.cursor_domain());
    let input = navigation::SharedXAxis {
        full_domain,
        current,
        height,
        left_margin: wave_left_margin(&state.ui.results),
        quantity_policy: state.ui.preferences.quantity_presentation_policy(),
        cursors: (cursor_owner || linked_cursor).then_some(state.ui.results.session.cursors),
    };
    let output =
        navigation::show_shared_x_axis(ui, &mut state.ui.results.session.derived, model, input);
    frame_work::note_samples(FrameSampleRead::StripOverview, output.overview_samples_read);
    frame_work::note_samples(FrameSampleRead::TraceExtremes, output.extrema_samples_read);
    if let Some(viewport) = output.viewport {
        let range = match viewport {
            navigation::SharedXViewChange::Fit => None,
            navigation::SharedXViewChange::Range(range) => Some(range),
        };
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, range);
    }
    if let Some(cursor) = output.cursor {
        if !cursor_owner && !linked_cursor {
            state.ui.results.session.clear_cursors();
            state.ui.results.session.cursor_strip = Some(model.analysis_index);
        }
        match cursor {
            navigation::CursorMove::A(x) => {
                state.ui.results.session.cursors.a = Some(x);
                state.ui.results.session.cursor_a_anchor = None;
            }
            navigation::CursorMove::B(x) => state.ui.results.session.cursors.b = Some(x),
        }
    }
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
    let pane_range = pane_y_range(&mut state.ui.results.session.derived, model, &pane.traces);
    let limits = if state.ui.results.session.show_spec_limits {
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
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let (x0, x1) = x_domain;
    // The pane's own top edge, kept so the active rail can span header and
    // canvas together once the pane's full height is known.
    let pane_top = ui.available_rect_before_wrap().top();

    let pane_range = pane_y_range(&mut state.ui.results.session.derived, model, &pane.traces);
    let specification_limits = if state.ui.results.session.show_spec_limits {
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
        let view = state.ui.results.session.analysis_plot_view_pane_mut(
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
    let pane_view = state.ui.results.session.analysis_plot_view_pane(
        super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let (x0, x1) =
        shared_x_view(&state.ui.results, model.analysis_key, pane_count).unwrap_or((x0, x1));
    let family_envelopes = state.ui.results.session.show_family_envelope.then(|| {
        let generation = state.ui.results.models.generation();
        extent::family_envelopes(
            &mut state.ui.results.plans.envelopes,
            generation,
            model,
            pane,
        )
    });
    let family_envelopes: &[extent::FamilyEnvelopeSeries] = family_envelopes
        .as_deref()
        .map_or(&[], FamilyEnvelopePlan::series);
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let model_cursor_domain = model.cursor_domain();
    let cursor_domain_matches = linked_cursor_domain == Some(&model_cursor_domain);
    let cursors = (state.ui.results.session.cursor_strip == Some(model.analysis_index)
        || (state.ui.results.session.linked_cursors && cursor_domain_matches))
        .then_some(state.ui.results.session.cursors);
    let markers = pane_view::plot_markers(
        model,
        pane,
        interpolation,
        state
            .ui
            .results
            .session
            .strip_markers(model.analysis_key)
            .into_iter()
            .map(|marker| pane_view::MarkerPresentation {
                anchor: marker.anchor(),
                x: marker.x(),
                kind: marker.kind(),
                color: marker_color(marker.kind(), &t),
                label: marker_label(marker),
            }),
    );
    let input = pane_view::PanePlot {
        x_range: (x0, x1),
        auto_y: (auto_y0, auto_y1),
        y_view: pane_view.y,
        log_y,
        left_margin: wave_left_margin(&state.ui.results),
        readout: presentation.readout(),
        quantity: quantity_policy,
        display_decimation: display_decimation(presentation.large_dataset_display()),
        minor_grid: state.ui.results.session.show_minor_grid,
        horizontal_cursor: state
            .ui
            .results
            .session
            .horizontal_cursor
            .as_ref()
            .filter(|cursor| cursor.pane == pane_key)
            .map(|cursor| cursor.y),
        horizontal_cursor_interactive: state
            .ui
            .results
            .session
            .horizontal_cursor_placement_enabled(),
        cursors,
        specification_limits,
        markers,
        expressions: exprs,
        family_envelopes,
        find_nearest: state.ui.results.session.marker_tool.is_armed()
            || (state.ui.results.session.cursor_placement_enabled()
                && state.ui.results.session.cursor_a_is_next()),
    };
    let drawn = pane_view::show_plot(ui, &mut state.ui.results.session.cache, model, pane, input);
    let response = drawn.response;
    if response.response.hovered()
        || response.response.dragged()
        || response.clicked_x.is_some()
        || response.horizontal_cursor_y.is_some()
    {
        state.ui.results.session.active_wave_pane = Some(pane_key.clone());
    }
    if let Some(y) = response.horizontal_cursor_y {
        state.ui.results.session.horizontal_cursor =
            Some(HorizontalWaveCursor { pane: pane_key, y });
    }

    // The marker tool takes the click when armed: one click cannot both
    // annotate and move a cursor, and the armed chip says which it will do.
    if let Some(clicked_x) = response.clicked_x
        && state.ui.results.session.marker_tool.is_armed()
    {
        let nearest = drawn.nearest_trace;
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
        && state.ui.results.session.cursor_placement_enabled()
    {
        let placing_cursor_a = state.ui.results.session.cursor_a_is_next();
        let nearest_anchor =
            placing_cursor_a.then(|| drawn.nearest_trace.map(|trace| anchor_key(model, trace)));
        let results = &mut state.ui.results;
        if results.session.cursor_strip != Some(model.analysis_index)
            && (!results.session.linked_cursors || !cursor_domain_matches)
        {
            results.session.cursors = CursorPair::default();
        }
        results.session.cursor_strip = Some(model.analysis_index);
        if placing_cursor_a {
            results.session.cursor_a_anchor = nearest_anchor.flatten();
        }
        results.session.cursors.place(clicked_x);
    }

    if response.view.reset {
        set_shared_x_view(&mut state.ui.results, model.analysis_key, pane_count, None);
        let view = state.ui.results.session.analysis_plot_view_pane_mut(
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
        let view = state.ui.results.session.analysis_plot_view_pane_mut(
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
    let pane_active = state.ui.results.session.active_wave_pane.as_ref()
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
