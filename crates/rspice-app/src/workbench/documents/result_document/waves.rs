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
#[cfg(test)]
use crate::ui::plot::{CursorPair, SampleInterpolation};
use rspice_results_ui::derived::DerivedSeries;
use rspice_results_ui::presentation::well_hint;
#[cfg(test)]
use rspice_results_ui::session::{ExprEditor, MarkerView};
use rspice_results_ui::waves::cursor_interpolation;
use rspice_results_ui::waves::header as pane_header;
use rspice_results_ui::waves::navigation::shared_axis_viewport_fraction;
use rspice_results_ui::waves::pane::pane_auto_y;
#[cfg(test)]
use rspice_results_ui::waves::pane::{apply_family_trace_style, nearest_drawn_trace};
use rspice_results_ui::waves::readout::append_copied_cursor;
pub(super) use rspice_results_ui::waves::{FamilyTraceVisibilityKey, StripModel, UnitPane};
use rspice_results_ui::waves::{StripTrace, anchor_key, family_color, fmt_in_unit};
#[cfg(test)]
use rspice_results_ui::waves::{TraceKind, trace_key};
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
use crate::ui::plot::{self, DisplayDecimation, XScale, fmt_si_significant};
use crate::ui::tokens::Tokens;
use crate::workbench::AppState;
use crate::workbench::{ComplexNumberDisplay, LargeDatasetDisplay};
use rspice_results::family_projection::SourceSampleSelection;
#[cfg(test)]
use rspice_results_ui::waves::navigation::model_x_axis;

use super::frame_work::{self, FrameSampleRead};
use super::{
    AnalysisPresentationKey, ExprSeries, ExprTrace, ExpressionSeriesResult, ExpressionSource,
    ExpressionWaveform, MarkerSelector, ResultsState, SelectedResultTrace,
    SourceWaveformPresentationKey, WavePanePresentationKey, WaveformPresentationKey,
};
#[cfg(test)]
use rspice_results_ui::waves::stack::set_pane_log_y;
use rspice_results_ui::waves::stack::{
    active_pane, model_is_visible, pane_log_y, set_shared_x_view, shared_x_view,
};
mod stack_host;

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
        &simulation.retained.executed_decks,
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
    let active_analysis_index = state.simulation.view.active_analysis_idx?;
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
    if active_pane(&models, &state.ui.results.session).is_some() {
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
                    && model_is_visible(model, &models, &state.ui.results.session)
            })
        })
        .or_else(|| {
            models
                .iter()
                .find(|model| model_is_visible(model, &models, &state.ui.results.session))
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
    active_pane(&models, &state.ui.results.session)
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
    let Some((model, _, pane)) = active_pane(&models, &state.ui.results.session) else {
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
    let (model, _, pane) = active_pane(models, &state.ui.results.session)?;
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

    let quantity = state.ui.preferences.quantity_presentation_policy();
    rspice_results_ui::waves::stack::show(
        ui,
        &mut stack_host::WaveStackHost(state),
        &models,
        rspice_results_ui::waves::stack::StackOptions {
            pane_chrome,
            readout: presentation.readout(),
            quantity,
            display_decimation: display_decimation(presentation.large_dataset_display()),
        },
    );
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
    let view = shared_x_view(&state.ui.results.session, model.analysis_key, panes).unwrap_or(full);
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
    let x = model.x_range.map(|full| {
        shared_x_view(&state.ui.results.session, key.analysis, panes.len()).unwrap_or(full)
    });
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
            set_shared_x_view(
                &mut state.ui.results.session,
                key.analysis,
                panes.len(),
                range,
            );
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
        let (x0, x1) =
            shared_x_view(&state.ui.results.session, key.analysis, panes.len()).unwrap_or(full);
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

#[cfg(test)]
mod tests;

fn pane_y_range(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
) -> Option<(f64, f64)> {
    rspice_results_ui::waves::stack::pane_y_range(derived, model, indices, |count| {
        frame_work::note_samples(FrameSampleRead::TraceExtremes, count)
    })
}
