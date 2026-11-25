//! The Results workspace.
//!
//! A 41 px dataset-viewer tab strip, a distinct 31 px plot instrument or
//! purpose strip, a fill-height document well carrying the active viewer
//! (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics), and a content-fit readout strip.

pub(crate) mod axis_range;
mod bars;
mod bode;
mod box_violin;
pub(crate) mod browser_plot;
pub mod create_document;
pub mod document_edit;
pub(crate) use document_edit::show as show_document_dialog;
mod events;
mod eye;
mod fft;
mod floquet;
pub mod frame_work;
mod harmonic_balance;
mod hist;
pub mod import;
mod instrument;
pub mod manifest;
pub(crate) mod network_matrix;
mod noise_contrib;
pub mod notes;
mod nyquist;
pub(crate) use nyquist::loop_gain_source;
pub mod op_inspector;
pub mod operational_state;
mod optimization;
pub mod persistent_document;
mod phase_noise;
mod polar;
mod population;
mod pz;
mod readout_pins;
mod retained_memo;
mod s_parameters;
mod scatter;
mod sensitivity;
mod sheet_settings;
pub(crate) mod signal_sets;
pub mod smith;
mod soa;
mod sweep;
mod table;
pub(crate) mod trace_statistics;
mod trace_style;
mod transfer_function;
pub mod view_context;
pub(crate) mod view_edits;
mod view_history;
mod view_plans;
mod viewer_state;
pub mod working_view;

/// Lossless CSV projection of the exact evidence rendered by a Results sheet.
pub(crate) struct ResultSheetCsv {
    pub(crate) default_name: &'static str,
    pub(crate) contents: String,
    pub(crate) detail: String,
}

// The application retains the Events source-order cache; viewer selections
// belong to the lower-owned session.
pub(crate) use events::EventOrderCache;
pub(crate) use network_matrix::exact_values as network_matrix_exact_values;
pub(crate) use noise_contrib::export_csv as export_noise_contribution_csv;
pub(crate) use op_inspector::export_csv as export_operating_point_csv;
pub use optimization::apply_best_to_design;
pub(crate) use optimization::export_csv as export_optimization_csv;
pub use trace_style::{rename_trace, set_trace_style};
pub(crate) use view_history::{
    zoom_next, zoom_next_available, zoom_previous, zoom_previous_available,
};

pub(crate) fn harmonic_balance_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    harmonic_balance::analysis_is_renderable(analysis)
}

pub(crate) fn phase_noise_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    phase_noise::phase_noise_is_renderable(analysis)
}

/// Whether the tab strip offers a frequency-response sheet for one analysis,
/// counting the walk the distortion branch performs.
///
/// That walk is the reason per-frame callers ask this through
/// [`analysis_answers_structural_gate`] rather than directly.
pub(crate) fn bode_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::renderability::bode_analysis_is_renderable(analysis, frame_work::note_scan)
}

/// Open the dataset/manifest browser in its canonical Results frame.
///
/// The navigator and inspector are parts of that frame; this command exposes
/// them instead of creating a second dataset browser with independent state.
/// What a sheet with no results says, the same words wherever it is said:
/// the Run shortcut joins them when one is bound.
pub(crate) fn no_results_hint(run_shortcut: &str) -> String {
    if run_shortcut.is_empty() {
        "No results yet — run a simulation".to_owned()
    } else {
        format!("No results yet — run a simulation ({run_shortcut})")
    }
}

pub(crate) fn open_dataset_browser(state: &mut ResultsParts<'_>) {
    state.workbench.activate(Workspace::Results);
    state.workbench.navigator_visible = true;
    state.workbench.inspector_visible = true;
    state
        .workbench
        .outbox
        .push(crate::workbench::framework::requests::FocusNavigatorSearch);
}

/// Open a fresh immutable-dataset-bound result-document transaction.
pub(crate) fn open_create_document(state: &mut ResultsParts<'_>) {
    create_document::open(state);
}

/// Add the selected result as a pane: to the open result document after its
/// selected pane, or, over a quick view, to a new result document.
pub(crate) fn add_selected_result_as_pane(state: &mut ResultsParts<'_>) {
    if add_pane_available(state.results_reads()) {
        open_add_pane(state);
    } else {
        create_document::open(state);
    }
}

pub use document_edit::{
    add_pane_available, compare_availability, family_available, linked_cursors, open_add_pane,
    open_compare, open_family, open_measurement, open_properties, set_family_filter,
    toggle_linked_cursors,
};

#[cfg(any(test, feature = "test-support"))]
pub fn visualization_source_dataset(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
) -> Result<crate::results::visualization_document::SourceDataset, String> {
    create_document::source_reference(run, analysis, run.dataset_content_digest())
        .map_err(|error| error.to_string())
}

pub mod waves;
pub(crate) use waves::{
    copy_cursor_text, cursor_jump_availability, fit_y_to_visible_x, jump_cursor,
};

/// Place a point marker on the trace at cursor A, opening its dialog, as the
/// instrument bar's marker control does; says why on the status bar when
/// there is no trace under the cursor.
pub(crate) fn place_marker_at_cursor(state: &mut ResultsParts<'_>) {
    let t = Tokens::default();
    if waves::marker_at_cursor_a_available(state, &t) {
        waves::drop_marker_at_cursor_a(state, &t);
    } else {
        state.prompt("There is no visible trace under cursor A to mark.");
    }
}
pub use waves::{
    ActivePaneFacts, SharedXStatus, active_pane_facts, active_shared_x_status,
    analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};

pub use browser_plot::{
    ResultQuantityDrag, set_result_quantities_ticked, set_result_quantity_ticked,
};
pub use waves::{event_net_plotted, set_event_net_plotted, toggle_visibility, waveform_plotted};

use bars::show as show_sheet_bar;
use retained_memo::RetainedMemo;
use std::collections::{HashMap, HashSet};

use egui::{Ui, WidgetInfo, WidgetType};

#[cfg(any(test, feature = "test-support"))]
pub(crate) use crate::state::result_presentation::TracePresentationKey;
pub use crate::state::result_presentation::{AnalysisPresentationKey, WaveformPresentationKey};
pub use crate::state::result_presentation::{ExprTrace, MarkerKind, WavePanePresentationKey};

use crate::product::{AnalysisInstanceId, DatasetId};

use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{ActiveViewer, AnalysisResult, RetainedTrace, SimulationRun};

use crate::ui::tokens::Tokens;
use crate::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState;
use crate::workbench::framework::command_registry::{Registry, ids};
use crate::workbench::framework::ports::{CapturedViewport, ResultsPort};
use crate::workbench::state::{WorkbenchSession, Workspace, WorkspaceDocumentId};
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_app_workbench::workbench::app_state::session::shortcuts::runtime_command_platform;
use rspice_results::family_projection::SourceSampleSelection;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SourceWaveformPresentationKey,
};
use rspice_results_ui::chrome;
use rspice_results_ui::chrome::instrument::ResultPlotTool;
use rspice_results_ui::create_document::CreateResultDocumentDialogState;
use rspice_results_ui::document_dialogs::ResultDocumentDialog;
use rspice_results_ui::presentation::{PlotView, well_hint};
use rspice_results_ui::selection::{
    ResultBrowserSelectionKey, ResultExpressionPresentationKey, SelectedResultTrace,
};
use rspice_results_ui::session::{
    AxisExtent, DocumentMarker, MarkerSelector, OptimizationSelection, PaneAxis,
    PlotPresentationKey, ViewGesture,
};
use rspice_results_ui::soa::SoaRuleSelection;

pub(crate) type ExpressionSeriesResult = Result<Vec<ExpressionWaveform>, String>;

/// The input scope that produced an expression output. A family ordinal is
/// resolved only within the exact selection fingerprint held by its cache.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExpressionSource {
    Analysis,
    SelectedSamples,
    FamilyMember { ordinal: usize },
}

#[derive(Debug, Clone)]
pub struct ExpressionWaveform {
    pub(crate) source: ExpressionSource,
    pub(crate) waveform: crate::state::RetainedTrace,
    /// How a signal read from another run was brought onto this axis
    /// (`Run 2, linear`), when the expression reads one.
    pub(crate) reading: Option<String>,
}
/// Whether retained evidence belongs to the stable analysis authored in the
/// simulation plan. Deterministically expanded executions (for example PVT
/// points) have distinct execution identities and must still match their
/// common authored analysis.
pub(crate) fn analysis_matches_authored_source(
    analysis: &AnalysisResult,
    authored_source_id: AnalysisInstanceId,
) -> bool {
    analysis
        .provenance()
        .is_some_and(|provenance| provenance.authored_source_instance_id() == authored_source_id)
}

/// Resolve one typed Data Browser artifact to a durable, human-readable path.
///
/// The path is presentation text rather than a filesystem path. Every segment
/// comes from immutable dataset or producer identity, so copying it remains
/// meaningful after result vectors are reordered.
pub(crate) fn result_artifact_stable_path(
    key: &ResultArtifactPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (run_index, _, analysis) = key
        .resolve(runs)
        .ok_or_else(|| "The selected result no longer exists in its run.".to_owned())?;
    let analysis_id = analysis.provenance().map_or_else(
        || format!("legacy-{}", analysis.id),
        |provenance| provenance.source_instance_id().to_string(),
    );
    Ok(format!(
        "dataset/{}/analysis/{analysis_id}/artifact/{}",
        runs[run_index].dataset_id,
        key.canonical_name()
    ))
}

fn resolved_result_signal<'a>(
    key: &SourceWaveformPresentationKey,
    runs: &'a [SimulationRun],
) -> Result<(&'a SimulationRun, &'a AnalysisResult, &'a RetainedTrace), String> {
    let (run_index, analysis_index, _, waveform) = key.resolve(runs).ok_or_else(|| {
        "The selected signal is missing from its run or is no longer unique.".to_owned()
    })?;
    Ok((
        &runs[run_index],
        &runs[run_index].analyses[analysis_index],
        waveform,
    ))
}

pub(crate) fn result_signal_stable_path(
    key: &SourceWaveformPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (run, analysis, waveform) = resolved_result_signal(key, runs)?;
    Ok(signal_stable_path(run, analysis, waveform))
}

fn signal_stable_path(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    waveform: &RetainedTrace,
) -> String {
    format!("{}{}", signal_path_start(run, analysis), waveform.name)
}

/// What the stable path of each signal of `analysis` starts with: the path
/// is this and the signal's name.
fn signal_path_start(run: &SimulationRun, analysis: &AnalysisResult) -> String {
    let analysis_id = analysis.provenance().map_or_else(
        || format!("legacy-{}", analysis.id),
        |provenance| provenance.source_instance_id().to_string(),
    );
    format!(
        "dataset/{}/analysis/{analysis_id}/quantity/",
        run.dataset_id
    )
}

/// Two stable paths, each given as a start and the rest, in the order of
/// the strings they spell.
fn stable_path_order(left: (&str, &str), right: (&str, &str)) -> std::cmp::Ordering {
    if left.0 == right.0 {
        return left.1.cmp(right.1);
    }
    left.0
        .bytes()
        .chain(left.1.bytes())
        .cmp(right.0.bytes().chain(right.1.bytes()))
}

/// A browser selection's signal, as its run holds it.
pub(crate) type ResolvedSignal<'a> = (&'a SimulationRun, &'a AnalysisResult, &'a RetainedTrace);

/// The browser selection `keys` in the order of their stable paths, each with its signal where
/// it resolves to one, as [`result_browser_selection_stable_path`] and the key's own resolution
/// answer them. A key resolved on its own searches its whole analysis, so ordering a run's
/// every column by comparing keys in pairs cost the square of its columns: here each analysis
/// the selection names is indexed once, and a signal's path is read as its analysis's start and
/// its name, never written out.
pub(crate) fn resolve_result_browser_selection<'a>(
    keys: &std::collections::HashSet<ResultBrowserSelectionKey>,
    runs: &'a [SimulationRun],
    (indices, revision): (
        &rspice_results_ui::signal_index::SignalIndices,
        &crate::state::RunHistoryRevision,
    ),
) -> Vec<(ResultBrowserSelectionKey, Option<ResolvedSignal<'a>>)> {
    type ResolvedRun<'a> = (&'a SimulationRun, &'a AnalysisResult);
    /// An analysis's signals by name, from its index or else one walk; a name
    /// two signals share finds neither.
    enum Names<'a> {
        Indexed(std::sync::Arc<rspice_results_ui::signal_index::SignalIndex>),
        Walked(std::collections::HashMap<&'a str, Option<usize>>),
    }
    /// An analysis's run, the analysis, its signals by name and what their
    /// stable paths start with.
    type NameIndex<'a> = (
        &'a SimulationRun,
        &'a AnalysisResult,
        Names<'a>,
        std::sync::Arc<str>,
    );
    let mut analyses =
        std::collections::HashMap::<AnalysisPresentationKey, Option<NameIndex<'a>>>::new();
    let mut resolved = keys
        .iter()
        .map(|key| {
            let mut start = None;
            let signal = key.waveform().and_then(|signal| {
                let (run, analysis, names, path_start) = analyses
                    .entry(signal.analysis())
                    .or_insert_with(|| {
                        let run = runs
                            .iter()
                            .find(|run| run.dataset_id == signal.analysis().dataset_id())?;
                        let (_, analysis) = signal.analysis().resolve(&run.data)?;
                        let path_start = signal_path_start(run, analysis).into();
                        if let Some(index) =
                            indices.index(revision, signal.analysis(), &analysis.data)
                        {
                            return Some((run, analysis, Names::Indexed(index), path_start));
                        }
                        let mut names = std::collections::HashMap::new();
                        for (index, waveform) in analysis.waveforms.iter().enumerate() {
                            names
                                .entry(waveform.name.as_str())
                                .and_modify(|found: &mut Option<usize>| *found = None)
                                .or_insert(Some(index));
                        }
                        Some((run, analysis, Names::Walked(names), path_start))
                    })
                    .as_ref()?;
                let (run, analysis): ResolvedRun<'a> = (*run, *analysis);
                let index = match names {
                    Names::Indexed(index) => index.column(&analysis.data, signal.source_name())?,
                    Names::Walked(names) => (*names.get(signal.source_name())?)?,
                };
                start = Some(std::sync::Arc::clone(path_start));
                Some((run, analysis, &analysis.waveforms[index]))
            });
            // The key's stable path, as a start and the rest.
            let rest: std::borrow::Cow<'a, str> = match (key, signal) {
                (ResultBrowserSelectionKey::Waveform(_), Some((_, _, waveform))) => {
                    waveform.name.as_str().into()
                }
                (ResultBrowserSelectionKey::Waveform(_), None) => "".into(),
                (ResultBrowserSelectionKey::Artifact(artifact), _) => {
                    result_artifact_stable_path(artifact, runs)
                        .unwrap_or_default()
                        .into()
                }
            };
            (start, rest, key.clone(), signal)
        })
        .collect::<Vec<_>>();
    resolved.sort_by(|left, right| {
        stable_path_order(
            (left.0.as_deref().unwrap_or(""), &left.1),
            (right.0.as_deref().unwrap_or(""), &right.1),
        )
    });
    resolved
        .into_iter()
        .map(|(_, _, key, signal)| (key, signal))
        .collect()
}

/// A signal's last retained sample as "Copy value" puts it on the clipboard:
/// its name, then x and y with their units in the format Copied values
/// chose, at full precision.
pub fn exact_result_signal_last_sample(
    key: &SourceWaveformPresentationKey,
    runs: &[SimulationRun],
    policy: crate::quantity::QuantityPresentationPolicy,
) -> Result<String, String> {
    let (_, analysis, waveform) = resolved_result_signal(key, runs)?;
    validate_exact_analysis_evidence(analysis)?;
    let y_unit = browser_signal_unit(
        &waveform.name,
        waveform.unit.as_deref(),
        analysis_default_unit(analysis.analysis_type),
    );
    rspice_formats::result_csv::encode_exact_sample(
        waveform.as_ref(),
        &sample_x_unit(analysis),
        y_unit,
        |value, unit| copied_sample_value(policy, value, unit),
    )
}

/// The unit of a signal's abscissa: an imported coordinate's own, the swept
/// source's for a DC sweep, otherwise the analysis's sweep axis.
fn sample_x_unit(analysis: &AnalysisResult) -> String {
    if let Some(coordinate) = analysis.imported_coordinate() {
        return coordinate.unit.clone().unwrap_or_default();
    }
    if let Some(crate::state::AnalysisResultPayload::DcSweep { evidence }) =
        &analysis.result_payload
    {
        // A SPICE source's leading letter is its element type, so it is
        // also the quantity swept; a swept parameter states no unit.
        let unit = match evidence.source.chars().next() {
            Some('V' | 'v') => "V",
            Some('I' | 'i') => "A",
            _ => "",
        };
        return unit.to_owned();
    }
    analysis.analysis_type.axis_info().1.to_owned()
}

/// One copied value with its unit, as Copied values writes it. A frequency
/// and an angle follow Frequency and Angle display; a unit that names its
/// own decade or carries a power takes no SI prefix; a value with no unit is
/// written as it stands, since a prefix with nothing after it reads as one.
fn copied_sample_value(
    policy: crate::quantity::QuantityPresentationPolicy,
    value: f64,
    unit: &str,
) -> String {
    match unit {
        "" => format!("{value:.17e}"),
        "Hz" => policy.copy_frequency(value),
        "°" => policy.copy_angle_degrees(value),
        "rad" => policy.copy_angle(value),
        unit if rspice_results_ui::waves::unit_carries_its_decade(unit) => {
            policy.copy_scaled_unit_value(value, unit)
        }
        unit if unit.contains(['^', '²']) => format!("{value:.17e} {unit}"),
        unit => policy.copy_si_value(value, unit),
    }
}

pub fn exact_result_signal_tsv(
    key: &SourceWaveformPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (run, analysis, waveform) = resolved_result_signal(key, runs)?;
    validate_exact_analysis_evidence(analysis)?;
    rspice_formats::result_csv::encode_signal_tsv(
        run.dataset_id,
        &analysis.label,
        waveform.as_ref(),
    )
}

fn validate_exact_analysis_evidence(analysis: &AnalysisResult) -> Result<(), String> {
    frame_work::note(frame_work::DatasetWalk::EvidenceValidation);
    analysis
        .validate_retained_evidence()
        .map_err(|error| exact_evidence_refusal(&error))
}

/// Why an analysis whose retained evidence failed validation gives no exact
/// values.
fn exact_evidence_refusal(error: &str) -> String {
    format!(
        "The selected analysis's saved results failed validation, so its values cannot be copied or exported: {error}"
    )
}

/// [`result_browser_selection_evidence`] for a key `signal` already resolves, when it is a
/// signal's.
pub(crate) fn resolved_selection_evidence(
    key: &ResultBrowserSelectionKey,
    signal: Option<ResolvedSignal<'_>>,
    results: &ResultsState,
    model: &Model,
) -> Result<(), String> {
    match (key, signal) {
        (ResultBrowserSelectionKey::Waveform(_), Some((run, analysis, _))) => {
            retained_evidence_verdict_over(
                results,
                model,
                AnalysisPresentationKey::new(run.dataset_id, analysis),
            )
            .map_err(|error| exact_evidence_refusal(&error))
        }
        (ResultBrowserSelectionKey::Waveform(_), None) => {
            Err("The selected signal is missing from its run or is no longer unique.".to_owned())
        }
        (ResultBrowserSelectionKey::Artifact(_), _) => {
            result_browser_selection_evidence(key, results, model)
        }
    }
}

/// Whether the browser selection `key` gives exact values, and why not,
/// answered from the workspace's memo: the navigator's row menus and its
/// band of batch actions ask on every frame, and the run is walked once per
/// retained revision, not once per row per frame.
pub(crate) fn result_browser_selection_evidence(
    key: &ResultBrowserSelectionKey,
    results: &ResultsState,
    model: &Model,
) -> Result<(), String> {
    let runs = &model.simulation.retained.runs;
    let (run, analysis) = match key {
        ResultBrowserSelectionKey::Waveform(key) => {
            let (run, analysis, _) = resolved_result_signal(key, runs)?;
            (run, analysis)
        }
        ResultBrowserSelectionKey::Artifact(key) => {
            let (run_index, _, analysis) = key
                .resolve(runs)
                .ok_or_else(|| "The selected result no longer exists in its run.".to_owned())?;
            (&runs[run_index], analysis)
        }
    };
    retained_evidence_verdict_over(
        results,
        model,
        AnalysisPresentationKey::new(run.dataset_id, analysis),
    )
    .map_err(|error| exact_evidence_refusal(&error))
}

pub(crate) fn result_browser_selection_stable_path(
    key: &ResultBrowserSelectionKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => result_signal_stable_path(key, runs),
        ResultBrowserSelectionKey::Artifact(key) => result_artifact_stable_path(key, runs),
    }
}

pub(crate) fn exact_result_browser_selection_text(
    key: &ResultBrowserSelectionKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => exact_result_signal_tsv(key, runs),
        ResultBrowserSelectionKey::Artifact(key) => exact_result_artifact_text(key, runs),
    }
}

pub(crate) fn result_browser_selection_canonical_name(
    key: &ResultBrowserSelectionKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => key
            .resolve(runs)
            .map(|(.., waveform)| waveform.name.clone())
            .ok_or_else(|| {
                "The selected signal is missing from its run or is no longer unique.".to_owned()
            }),
        ResultBrowserSelectionKey::Artifact(key) => Ok(key.canonical_name().to_owned()),
    }
}

/// Deterministic exact bundle used by batch clipboard and export actions.
pub fn exact_result_browser_selection_bundle(
    keys: &[ResultBrowserSelectionKey],
    runs: &[SimulationRun],
) -> Result<String, String> {
    rspice_formats::result_csv::encode_selection_text(keys.iter().map(|key| {
        let path = result_browser_selection_stable_path(key, runs)?;
        let exact = exact_result_browser_selection_text(key, runs)?;
        Ok((path, exact))
    }))
}

#[cfg(test)]
mod exact_evidence_quarantine_tests;

#[cfg(test)]
mod copied_value_tests;

#[cfg(any(test, feature = "test-support"))]
pub mod test_fixtures;
#[cfg(test)]
pub(crate) mod test_host;
#[cfg(test)]
pub(crate) use test_host::ResultsTestHost;

/// Lossless source-evidence projection for one typed Data Browser artifact.
///
/// This is the single adapter used by clipboard, exact-table, and file-export
/// workflows. Keeping those actions on one projection prevents a viewer from
/// displaying one retained entity while exporting another analysis payload.
pub fn exact_result_artifact_text(
    key: &ResultArtifactPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (run_index, _, analysis) = key
        .resolve(runs)
        .ok_or_else(|| "The selected result no longer exists in its run.".to_owned())?;
    validate_exact_analysis_evidence(analysis)?;
    rspice_formats::result_csv::encode_artifact_text(
        runs[run_index].dataset_id,
        &analysis.data,
        key.canonical_name(),
    )
}

pub use rspice_results::result_presentation::ResultViewer;

/// Whether a viewer draws through the shared unit-pane waveform stack.
///
/// Those sheets key their viewports by analysis; every other viewer is a
/// single canvas keyed by plot ordinal. A gesture that does not ask this
/// question first will write to a store the sheet never reads.
pub(crate) use rspice_results::result_presentation::viewer_uses_wave_stack;

/// Queue a viewport gesture for the active result sheet.
pub(crate) fn request_view_gesture(results: &mut ResultsState, gesture: ViewGesture) {
    results.session.pending_view_gesture = Some(gesture);
}

/// Whether the active sheet has a viewport that fitting can release.
///
/// Every plot sheet does; the structured documents (OP, table, XF, manifest)
/// have no viewport at all and must not offer the gesture.
pub(crate) fn fit_gesture_available(state: ResultsReads<'_>) -> bool {
    state.model.simulation.retained.has_results()
        && viewer_draws_a_pane(state.results.session.viewer)
}

/// Whether this sheet draws a plot pane at all.
///
/// The evidence tables (OP, sample table, event history) and the scalar
/// transfer-function readout draw no axes, so they have no viewport, no
/// fit, and no limit mask to report.
pub(crate) const fn viewer_draws_a_pane(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::Op
            | ResultViewer::Table
            | ResultViewer::Events
            | ResultViewer::TransferFunction
    )
}

/// Whether the active sheet can be magnified about its centre.
///
/// Only the unit-pane stack exposes the retained extents a zoom step has to
/// be computed against; the single-canvas viewers own their own gestures.
pub(crate) fn zoom_gesture_available(state: ResultsReads<'_>) -> bool {
    state.model.simulation.retained.has_results()
        && viewer_uses_wave_stack(state.results.session.viewer)
}

/// Apply any queued viewport gesture, now that the sheet's models and theme
/// tokens exist.
pub(crate) fn apply_pending_view_gesture(ui: &Ui, state: &mut ResultsParts<'_>) {
    let Some(gesture) = state.results.session.pending_view_gesture.take() else {
        return;
    };
    let viewer = state.results.session.viewer;
    if !viewer_uses_wave_stack(viewer) {
        // Single-canvas viewers keep their views under plot ordinals, and
        // only fit is expressible without their renderer's own extents.
        match gesture {
            ViewGesture::Fit => state.results.session.reset_viewer_plot_views(viewer),
            ViewGesture::ZoomIn | ViewGesture::ZoomOut => {}
        }
        return;
    }
    match gesture {
        ViewGesture::Fit => waves::fit_active_strip(state),
        ViewGesture::ZoomIn => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 0.5),
        ViewGesture::ZoomOut => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 2.0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerAvailability {
    pub available: bool,
    pub reason: &'static str,
}

impl ViewerAvailability {
    const fn available(reason: &'static str) -> Self {
        Self {
            available: true,
            reason,
        }
    }

    const fn unavailable(reason: &'static str) -> Self {
        Self {
            available: false,
            reason,
        }
    }
}

/// The session slices a custom-canvas sheet reads.
///
/// Sheets take the slices they use instead of the whole session aggregate.
/// `documents/result_document -> app_state` is an edge this crate is retiring
/// — a module that needs the retained runs, the requirements and its own
/// presentation state should say so — and a sheet that reached for `AppState`
/// would extend it by one more file for nothing.
pub(super) struct SheetContext<'a> {
    pub(super) simulation: &'a crate::state::SimulationState,
    pub(super) workspace: &'a crate::state::ProjectWorkspace,
    pub(super) results: &'a mut ResultsState,
    pub(super) policy: crate::quantity::QuantityPresentationPolicy,
    /// The reader's significant digits, which the sheet's values read at.
    pub(super) significant_digits: usize,
}

impl<'a> SheetContext<'a> {
    fn of(state: &'a mut ResultsParts<'_>) -> Self {
        let significant_digits = usize::from(
            state
                .results_reads()
                .presentation_policy()
                .displayed_significant_digits()
                .get(),
        );
        Self {
            significant_digits,
            policy: state.model.preferences.quantity_presentation_policy(),
            simulation: &state.model.simulation,
            workspace: &state.model.workspace,
            results: &mut *state.results,
        }
    }
}

/// Leave for Verify from a requirement line a result draws: on `page`, with
/// the specification measured as `specification` selected when the line
/// states one.
pub(super) fn open_in_verify(
    state: &mut ResultsParts<'_>,
    page: crate::state::VerificationPage,
    specification: Option<String>,
) {
    state
        .workbench
        .outbox
        .push(crate::workbench::framework::requests::OpenInVerify {
            page,
            specification,
            check: None,
        });
}

/// What the results workspace's predicates read: its viewer state and the
/// specialized viewers' state, the model whose retained runs they judge, and
/// the session whose active document they stand on.
///
/// Whether a viewer, a pane or a hop is available is a question about those
/// four parts alone, so the predicates take them rather than the application
/// state, and a feature's availability can answer them from what it reads.
#[derive(Clone, Copy)]
pub struct ResultsReads<'a> {
    pub(crate) results: &'a ResultsState,
    pub(crate) analysis: &'a AnalysisWorkspaceState,
    pub(crate) model: &'a Model,
    pub(crate) workbench: &'a WorkbenchSession,
}

/// What the results workspace's commands write: its viewer state, the
/// specialized viewers' state, the drafts of its forms, the result import
/// and the specification route, the model, the session, the platform
/// services' state, which says whether a project is open, and the run
/// preflight.
///
/// Opening a form, importing a dataset or linking cursors reads and writes
/// those parts alone, so the operations take them rather than the
/// application state, and a feature's command can run them in its context.
pub struct ResultsParts<'a> {
    pub(crate) results: &'a mut ResultsState,
    pub(crate) analysis: &'a mut AnalysisWorkspaceState,
    pub(crate) result_document_dialog: &'a mut Option<ResultDocumentDialog>,
    pub(crate) create_result_document: &'a mut CreateResultDocumentDialogState,
    pub(crate) result_import: &'a mut import::ResultImportDialogState,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) platform: &'a PlatformState,
    /// The run preflight, which an edit to the plan drops.
    pub(crate) preflight:
        &'a mut rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState,
    /// Every command the application registers: what a chord pressed over
    /// the plot is resolved against and a command's shortcut is labelled by.
    pub(crate) commands: &'static Registry,
    /// What Run plan answered when the parts were lent, for the Run again a
    /// state's card offers; hidden where the lender did not ask.
    pub(crate) run_plan: crate::workbench::framework::command_registry::CommandAvailability,
}

impl ResultsParts<'_> {
    /// These parts, holding what Run plan answers now: the card of a result
    /// the design moved on from, or of a cancelled run, offers Run again
    /// through it.
    #[must_use]
    pub fn with_run_plan(
        mut self,
        run_plan: crate::workbench::framework::command_registry::CommandAvailability,
    ) -> Self {
        self.run_plan = run_plan;
        self
    }
}

impl ResultsReads<'_> {
    /// The reader's result presentation, with the significant digits of the
    /// project result document the workspace is showing, if any.
    pub fn presentation_policy(
        self,
    ) -> rspice_app_state::workbench::preferences::ResultPresentationPolicy {
        let policy = self.model.preferences.result_presentation_policy();
        self.results
            .session
            .document_presentation
            .map_or(policy, |document| {
                policy.with_significant_digits(document.significant_digits)
            })
    }

    /// What the sheet shown prints beyond what its page reads for itself:
    /// a table sheet's tables in its words, or the figures its card shows.
    pub fn sheet_print(self) -> rspice_results_ui::table::SheetPrint {
        match self.results.session.viewer {
            ResultViewer::Table => table::printed(self),
            ResultViewer::NetworkMatrix => network_matrix::printed(self),
            ResultViewer::Bode => bode::printed_figures(self),
            ResultViewer::Nyquist => nyquist::printed_figures(self),
            ResultViewer::Fft => fft::printed_figures(self),
            _ => rspice_results_ui::table::SheetPrint::default(),
        }
    }
}

impl ResultsState {
    /// Whether the table sheet shown holds a table its page can print,
    /// without reading its words; `true` for every other sheet, whose page
    /// reads its own evidence.
    pub fn sheet_table_is_printable(&self, simulation: &crate::state::SimulationState) -> bool {
        match self.session.viewer {
            ResultViewer::Table => table::is_printable(self, simulation),
            ResultViewer::NetworkMatrix => network_matrix::is_printable(self, simulation),
            _ => true,
        }
    }
}

impl ResultsParts<'_> {
    /// What the results workspace's predicates read of these parts.
    pub(crate) fn results_reads(&self) -> ResultsReads<'_> {
        ResultsReads {
            results: &*self.results,
            analysis: &*self.analysis,
            model: &*self.model,
            workbench: &*self.workbench,
        }
    }

    /// Answer the gesture just made, on the status bar, as
    /// [`AppState::prompt`](crate::workbench::AppState::prompt) does.
    pub(crate) fn prompt(&mut self, text: impl Into<String>) {
        self.workbench.prompt(text);
    }

    /// Record an event in the Console, as
    /// [`AppState::record`](crate::workbench::AppState::record) does.
    pub(crate) fn record(
        &mut self,
        severity: crate::diagnostics::LogSeverity,
        source: crate::diagnostics::LogSource,
        text: impl Into<String>,
    ) -> u64 {
        self.model.record(severity, source, text)
    }

    /// Report a failure of something the reader asked for, as
    /// [`AppState::report_failure`](crate::workbench::AppState::report_failure) does.
    pub(crate) fn report_failure(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        self.model.report_failure(title, detail);
    }

    /// Show a part of the open drawing in Design, selected and centred on the
    /// sheet that draws it, by the route every "show me this" takes.
    pub(crate) fn show_component_on_schematic(
        &mut self,
        component_id: u64,
        position: crate::state::Point,
    ) {
        let mut selection = rspice_design::schematic::selection::Selection::default();
        selection.select_component(component_id);
        self.workbench
            .outbox
            .push(crate::workbench::framework::requests::ShowOnSchematic {
                cell_view: self.model.workspace.content.active_schematic_reference(),
                sheet: self
                    .model
                    .multi_sheet_catalog()
                    .and_then(|catalog| catalog.sheet_for_object(component_id)),
                point: position,
                selection,
            });
    }
}

/// How a retained plot-marker kind reads as a Results marker kind.
pub const fn marker_kind_of_retained(
    kind: crate::results::visualization_document::PlotMarkerKind,
) -> MarkerKind {
    use crate::results::visualization_document::PlotMarkerKind;
    match kind {
        PlotMarkerKind::PointNote | PlotMarkerKind::MeasurementAnchor => MarkerKind::Note,
        PlotMarkerKind::Peak { size } => MarkerKind::Peak(size),
        PlotMarkerKind::Valley { size } => MarkerKind::Valley(size),
        PlotMarkerKind::Crossing { level } => MarkerKind::Crossing(level),
        PlotMarkerKind::SpecificationLine => MarkerKind::Spec,
        PlotMarkerKind::VerticalLine => MarkerKind::Vertical,
        PlotMarkerKind::Offset { from, by } => {
            MarkerKind::Offset(rspice_results::result_presentation::MarkerOffset { from, by })
        }
    }
}

/// How a Results marker kind is retained in a visualization document.
pub(crate) const fn retained_kind_of_marker(
    kind: MarkerKind,
) -> crate::results::visualization_document::PlotMarkerKind {
    use crate::results::visualization_document::PlotMarkerKind;
    match kind {
        MarkerKind::Note => PlotMarkerKind::PointNote,
        MarkerKind::Peak(size) => PlotMarkerKind::Peak { size },
        MarkerKind::Valley(size) => PlotMarkerKind::Valley { size },
        MarkerKind::Crossing(level) => PlotMarkerKind::Crossing { level },
        MarkerKind::Spec => PlotMarkerKind::SpecificationLine,
        MarkerKind::Vertical => PlotMarkerKind::VerticalLine,
        MarkerKind::Offset(rspice_results::result_presentation::MarkerOffset { from, by }) => {
            PlotMarkerKind::Offset { from, by }
        }
    }
}

/// One requested marker placement, as the drawn pane knows it.
pub struct MarkerPlacement<'a> {
    pub analysis: AnalysisPresentationKey,
    pub anchor: WaveformPresentationKey,
    pub trace_name: String,
    /// Where the reader asked for it, in the strip's X data space.
    pub x: f64,
    /// The anchoring trace's X samples, as drawn.
    ///
    /// A retained document marker names an exact retained sample — the
    /// document declines to interpolate one into existence — so a placement
    /// on a saved document is snapped onto the nearest of these. A working
    /// view keeps the requested position.
    pub samples: &'a rspice_results::waveform::SharedWaveformValues,
}

impl MarkerPlacement<'_> {
    /// The nearest retained sample to the requested position: on an axis its run checked
    /// ascends, one of the two samples either side of it, found without reading the column;
    /// on any other, the nearest of every sample, read a block at a time.
    fn retained_x(&self) -> f64 {
        let distance = |left: &f64, right: &f64| -> std::cmp::Ordering {
            (left - self.x).abs().total_cmp(&(right - self.x).abs())
        };
        if self.samples.is_ascending_axis() {
            let at = self.samples.partition_point(|sample| sample < self.x);
            let beside = [at.checked_sub(1), Some(at)]
                .into_iter()
                .flatten()
                .filter_map(|index| self.samples.sample(index))
                .filter(|sample: &f64| sample.is_finite())
                .min_by(distance);
            if let Some(nearest) = beside {
                return nearest;
            }
        }
        self.samples
            .samples(..)
            .filter(|sample: &f64| sample.is_finite())
            .min_by(distance)
            .unwrap_or(self.x)
    }
}

/// What a document retains as one marker's label.
///
/// A retained marker is a named entity: the document refuses a blank label,
/// because an anonymous annotation in a hardcopy is not an annotation. Until
/// the reader names it in the purpose dialog it is labelled with the signal it
/// rides, which is the most useful thing that is already true about it.
fn retained_marker_label<'a>(note: &'a str, trace_name: &'a str) -> &'a str {
    if note.trim().is_empty() {
        trace_name
    } else {
        note
    }
}

/// The note a retained marker label reads as: none while it is only the
/// name of the signal it rides, which its row states already.
pub(crate) fn marker_note(label: &str, trace_name: &str) -> String {
    if label == trace_name {
        String::new()
    } else {
        label.to_owned()
    }
}

/// Keep an expression the calculator plotted on `analysis` in the view in
/// front: the saved document whose tab is active, or else the shown run's
/// working view, which the quick view draws.
pub fn keep_plotted_expression(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    text: String,
) -> Result<bool, String> {
    let in_front = crate::workbench::chrome::document_bar::results::active_result_document(
        state.model,
        state.workbench,
    );
    if let Some(crate::workbench::state::WorkspaceDocumentId::VisualizationDocument(document)) =
        in_front
    {
        return persistent_document::keep_plotted_expression(state, document, &text);
    }
    // The run's view is made, or followed, and drawn into the quick view's
    // state first, so the expression is kept by it rather than drawn over.
    working_view::sync(state);
    let added = state
        .results
        .add_expression_trace(&state.model.simulation, analysis, text)?;
    working_view::capture(state);
    Ok(added)
}

/// Keep `text`, the difference a two-net probe named while no run was
/// shown, for the view of `plan`'s next run, which plots it once it reads
/// that run, as a single net's probe plots once a run saves its net.
/// `false` when it already waits.
pub fn keep_for_next_run(
    state: &mut ResultsParts<'_>,
    plan: crate::product::SimulationPlanId,
    text: String,
) -> bool {
    let waiting = &mut state.results.next_run_expressions;
    if waiting
        .iter()
        .any(|(held, held_text)| *held == plan && *held_text == text)
    {
        return false;
    }
    waiting.push((plan, text));
    true
}

/// Give the trace of `signal` on `analysis`'s strip the colour `color`, or
/// with `None` its signal's own colour again, on the view that keeps the
/// strip: the saved document's pane, or the run's working view. The run is
/// left as it was solved.
pub fn set_trace_color(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    signal: &str,
    color: Option<crate::results::visualization_document::TraceColor>,
) -> Result<(), String> {
    let context = state
        .results
        .session
        .pane_for(analysis)
        .ok_or_else(|| format!("No view keeps this strip, so '{signal}' keeps its colour."))?;
    let document = state
        .model
        .workspace
        .content
        .visualization_document(context.document_id)
        .ok_or_else(|| "The view is no longer in the project.".to_owned())?;
    let held: Vec<_> = document
        .traces()
        .iter()
        .filter(|trace| {
            trace.pane_id == context.pane_id
                && trace.source_signal().unwrap_or(&trace.label) == signal
        })
        .collect();
    if held.is_empty() {
        return Err(format!(
            "'{signal}' is not a trace this view keeps, so its colour was not changed."
        ));
    }
    let edits: Vec<_> = held
        .iter()
        .filter(|trace| trace.color != color)
        .map(
            |trace| crate::results::visualization_document::DocumentEdit::SetTraceColor {
                trace_id: trace.id,
                color,
            },
        )
        .collect();
    if !edits.is_empty() {
        let revision = document.revision();
        view_edits::apply(state, context.document_id, revision, edits)
            .map_err(|error| error.to_string())?;
    }
    state
        .results
        .draw_waveform_color(&SourceWaveformPresentationKey::new(analysis, signal), color);
    Ok(())
}

/// A marker sits at a finite X on a trace of its own strip.
fn validate_marker_placement(placement: &MarkerPlacement<'_>) -> Result<(), &'static str> {
    if !placement.x.is_finite() {
        return Err("marker position must be finite");
    }
    if placement.anchor.analysis != placement.analysis {
        return Err("marker anchor belongs to a different analysis or dataset");
    }
    Ok(())
}

/// Place a marker on the view that keeps the strip being drawn: the saved
/// document's pane, or the working view's pane for the strip's analysis.
///
/// The marker is committed to it immediately: one interaction, one
/// transaction, no frame diff to reconcile. It rides the view's trace of the
/// clicked signal; a click on something the view does not plot as a trace of
/// its own (an expression, or a trace another document draws) places nothing
/// and says why, rather than keeping the marker somewhere the view does not.
pub fn place_marker(
    state: &mut ResultsParts<'_>,
    placement: MarkerPlacement<'_>,
) -> Option<MarkerSelector> {
    if let Err(error) = validate_marker_placement(&placement) {
        state.prompt(format!("Could not place marker: {error}"));
        return None;
    }
    let retained_x = placement.retained_x();
    let MarkerPlacement {
        analysis,
        anchor,
        trace_name,
        x,
        ..
    } = placement;
    let Some(context) = state.results.session.pane_for(analysis) else {
        state.prompt(format!(
            "No view keeps this strip, so the marker on '{trace_name}' was not placed."
        ));
        return None;
    };
    let retained = state
        .model
        .workspace
        .content
        .visualization_document(context.document_id)
        .and_then(|document| {
            let trace = document.traces().iter().find(|trace| {
                trace.pane_id == context.pane_id
                    && (trace.label == trace_name
                        || trace.source_signal() == Some(anchor.trace.source_name.as_str()))
            })?;
            // A working view reads the run itself, so its marker stays where
            // it was put, as the quick view's always has; a saved document
            // names a retained sample.
            let at = if document.working_view().is_some() {
                x
            } else {
                retained_x
            };
            Some((document.revision(), trace.id, at))
        });
    let Some((revision, trace_id, at)) = retained else {
        state.prompt(format!(
            "'{trace_name}' is not a trace this view keeps, so the marker was not placed."
        ));
        return None;
    };

    let receipt = match view_edits::apply(
        state,
        context.document_id,
        revision,
        vec![
            crate::results::visualization_document::DocumentEdit::AddTypedMarker {
                pane_id: context.pane_id,
                trace_id,
                coordinate: crate::results::visualization_document::TypedValue::Real(at),
                label: trace_name.clone(),
                kind: crate::results::visualization_document::PlotMarkerKind::PointNote,
                scope: crate::results::visualization_document::PlotMarkerScope::Pane,
                source_specification: None,
            },
        ],
    ) {
        Ok(receipt) => receipt,
        Err(error) => {
            state.prompt(format!("Could not place marker: {error}"));
            return None;
        }
    };
    let marker_id = receipt
        .created
        .into_iter()
        .find_map(|entity| match entity {
            crate::results::visualization_document::EntityRef::Marker(id) => Some(id),
            _ => None,
        })?;
    // The overlay is rebuilt by the next projection, but the marker has to be
    // on screen and addressable in the frame the reader placed it.
    state.results.session.document_markers.push(DocumentMarker {
        document_id: context.document_id,
        pane_id: context.pane_id,
        retained_id: marker_id,
        analysis,
        anchor,
        trace_name,
        x: at,
        kind: MarkerKind::default(),
        note: String::new(),
    });
    Some(MarkerSelector {
        document_id: context.document_id,
        pane_id: context.pane_id,
        marker_id,
    })
}

/// Remove one marker from the view that keeps it.
pub fn remove_marker(state: &mut ResultsParts<'_>, selector: MarkerSelector) {
    let MarkerSelector {
        document_id,
        marker_id,
        ..
    } = selector;
    let Some(revision) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(|document| document.revision())
    else {
        return;
    };
    if let Err(error) = view_edits::apply(
        state,
        document_id,
        revision,
        vec![
            crate::results::visualization_document::DocumentEdit::Remove(
                crate::results::visualization_document::EntityRef::Marker(marker_id),
            ),
        ],
    ) {
        state.prompt(format!("Could not remove the marker: {error}"));
        return;
    }
    state
        .results
        .session
        .document_markers
        .retain(|marker| marker.retained_id != marker_id);
    if state
        .results
        .session
        .marker_edit
        .as_ref()
        .is_some_and(|draft| draft.selector == selector)
    {
        state.results.session.marker_edit = None;
    }
    if state.results.session.readout_base
        == Some(rspice_results_ui::session::ReadingColumnId::Marker(
            selector,
        ))
    {
        state.results.session.readout_base = None;
    }
}

/// Move one marker on the view that keeps it: a working view to where it
/// was asked, a saved document to the retained sample nearest.
pub fn move_marker(
    state: &mut ResultsParts<'_>,
    moved: rspice_results_ui::waves::dock::MarkerMove,
) {
    let rspice_results_ui::waves::dock::MarkerMove {
        selector,
        x,
        nearest_sample,
    } = moved;
    let MarkerSelector {
        document_id,
        marker_id,
        ..
    } = selector;
    let Some((revision, at)) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(|document| {
            let at = if document.working_view().is_some() {
                x
            } else {
                nearest_sample.unwrap_or(x)
            };
            (document.revision(), at)
        })
    else {
        return;
    };
    if let Err(error) = view_edits::apply(
        state,
        document_id,
        revision,
        vec![
            crate::results::visualization_document::DocumentEdit::MoveMarker {
                marker_id,
                coordinate: crate::results::visualization_document::TypedValue::Real(at),
            },
        ],
    ) {
        state.prompt(format!("Could not move the marker: {error}"));
        return;
    }
    // The overlay follows in the frame the reader moved it.
    if let Some(marker) = state
        .results
        .session
        .document_markers
        .iter_mut()
        .find(|marker| marker.retained_id == marker_id)
    {
        marker.x = at;
    }
}

/// Commit one marker-purpose edit to the view that keeps the marker.
///
/// The retained coordinate, scope and source specification are read back from
/// the live document rather than from the projection, so an edit of the label
/// or kind can never restate a stale position as if the reader had moved it.
pub fn commit_marker_edit(
    state: &mut ResultsParts<'_>,
    selector: MarkerSelector,
    note: &str,
    kind: MarkerKind,
) -> Result<(), String> {
    let MarkerSelector {
        document_id,
        marker_id,
        ..
    } = selector;
    let retained_kind = retained_kind_of_marker(kind);
    let trace_name = state
        .results
        .session
        .document_marker(marker_id)
        .map(|marker| marker.trace_name.clone())
        .unwrap_or_default();
    let label = retained_marker_label(note, &trace_name).to_owned();
    let retained = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .and_then(|document| {
            let marker = document
                .markers()
                .iter()
                .find(|marker| marker.id == marker_id)?;
            Some((
                document.revision(),
                marker.coordinate.clone(),
                marker.scope,
                marker.source_specification.clone(),
            ))
        });
    let Some((revision, coordinate, scope, source_specification)) = retained else {
        return Err("The retained marker is no longer part of this result document.".to_owned());
    };
    view_edits::apply(
        state,
        document_id,
        revision,
        vec![
            crate::results::visualization_document::DocumentEdit::SetMarker {
                marker_id,
                coordinate,
                label: label.clone(),
                kind: retained_kind,
                scope,
                source_specification,
            },
        ],
    )
    .map_err(|error| error.to_string())?;
    // The overlay states what the document now holds, not what was
    // typed: those are the same thing only when the typed label was
    // one the document would accept.
    if let Some(marker) = state
        .results
        .session
        .document_markers
        .iter_mut()
        .find(|marker| marker.retained_id == marker_id)
    {
        marker.note = label;
        marker.kind = kind;
    }
    Ok(())
}

/// A marker anchor naming one retained signal's plain value projection.
///
/// Production anchors come from the drawn trace, which carries the family
/// discriminator this cannot know; this exists so persistence tests can name
/// a signal without standing up a whole strip.
#[cfg(any(test, feature = "test-support"))]
pub fn marker_anchor_for(
    analysis: AnalysisPresentationKey,
    source_name: &str,
) -> WaveformPresentationKey {
    WaveformPresentationKey {
        analysis,
        trace: TracePresentationKey {
            source_name: source_name.to_owned(),
            kind: 0,
            family_group: 0,
        },
    }
}

/// Forget what Results showed of a project another project replaced: the
/// view or run it had open, and every view's undo steps. The opened
/// project then opens on its own Results, not on the view of the project
/// it replaced.
pub fn forget_replaced_project(state: &mut ResultsParts<'_>) {
    state
        .workbench
        .documents
        .retain_available(Workspace::Results, std::iter::empty());
    state.results.view_edits = view_edits::ViewHistories::default();
}

/// Restore the signal sets a project kept, repaired as the browser keeps
/// them.
pub fn restore_signal_sets(
    state: &mut ResultsParts<'_>,
    signal_sets: Vec<rspice_results::result_presentation::SignalSet>,
) {
    state.results.signal_sets = signal_sets::sanitize_signal_sets(signal_sets);
}

/// Whether one analysis' retained evidence passes its own validator.
///
/// This is the workspace's single owner of that question. The validator walks
/// every retained sample of every waveform, and the sheet bar, the tab strip's
/// availability gates and the typed-evidence viewers all asked it on every
/// frame — so the same million-sample walk happened several times a frame for
/// a reader who was not touching anything.
///
/// The memo checks the retained history revision on every read, including
/// queries before frame preparation. Both successful and failed verdicts are
/// invalidated when the source changes, even if its display version repeats.
pub(crate) fn retained_evidence_is_valid(
    state: ResultsReads<'_>,
    analysis: AnalysisPresentationKey,
) -> bool {
    retained_evidence_is_valid_over(state.results, state.model, analysis)
}

/// The same verdict over the two things it reads: the workspace's memo and
/// the model's retained runs.
pub(crate) fn retained_evidence_is_valid_over(
    results: &ResultsState,
    model: &Model,
    analysis: AnalysisPresentationKey,
) -> bool {
    retained_evidence_verdict_over(results, model, analysis).is_ok()
}

/// The validator's verdict on `analysis`, with its reason, from the
/// workspace's memo of it.
pub(crate) fn retained_evidence_verdict_over(
    results: &ResultsState,
    model: &Model,
    analysis: AnalysisPresentationKey,
) -> Result<(), std::sync::Arc<str>> {
    retained_evidence_verdict_in(
        &results.retained_evidence_validity,
        &model.simulation,
        analysis,
    )
}

/// [`incomplete_evidence_reason`] answered from the workspace's memo of the
/// evidence, `validity`: a build of the strips asks it of every analysis of
/// the shown run, and the validator reads every column of one.
fn remembered_incomplete_reason(
    validity: &RetainedMemo<AnalysisPresentationKey, Result<(), std::sync::Arc<str>>>,
    simulation: &crate::state::SimulationState,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> Option<&'static str> {
    if !analysis.success {
        return Some("the run did not complete — these samples stop where it failed");
    }
    // A stored trace still being read has not failed.
    retained_evidence_verdict_in(
        validity,
        simulation,
        AnalysisPresentationKey::new(dataset_id, analysis),
    )
    .is_err_and(|cause| !rspice_results::waveform::is_pending_read(&cause))
    .then_some("the saved results failed validation")
}

/// The validator's verdict on `analysis`, with its reason, from `validity`,
/// the workspace's memo of it.
fn retained_evidence_verdict_in(
    validity: &RetainedMemo<AnalysisPresentationKey, Result<(), std::sync::Arc<str>>>,
    simulation: &crate::state::SimulationState,
    analysis: AnalysisPresentationKey,
) -> Result<(), std::sync::Arc<str>> {
    // A verdict that waits on the reader service, for a stored trace still being read, is
    // asked again on a later frame rather than kept as a failure.
    validity.get_or_insert_lasting(
        simulation,
        analysis,
        || {
            let Some((_, resolved)) = simulation
                .retained
                .runs
                .iter()
                .find_map(|run| analysis.resolve(run))
            else {
                return Err("The selected result no longer exists in its run.".into());
            };
            frame_work::note(frame_work::DatasetWalk::EvidenceValidation);
            resolved.validate_retained_evidence().map_err(Into::into)
        },
        |verdict| {
            !verdict
                .as_ref()
                .is_err_and(|cause| rspice_results::waveform::is_pending_read(cause))
        },
    )
}

impl ResultsState {
    /// Hold what looking `analysis` over away from the frame found, as the
    /// retained runs stand now: the index of its signals and, when there is
    /// one of each, the validator's verdict on its evidence and its run's
    /// digest, taken of the digest of its data. None is worked out again
    /// while the runs stand.
    pub(crate) fn adopt_looked_over(
        &mut self,
        simulation: &crate::state::SimulationState,
        analysis: AnalysisPresentationKey,
        index: std::sync::Arc<rspice_results_ui::signal_index::SignalIndex>,
        (verdict, digest): (
            Option<Result<(), std::sync::Arc<str>>>,
            Option<crate::product::ContentDigest>,
        ),
    ) {
        let source = simulation.retained.runs.revision();
        self.session
            .signal_indices
            .adopt(&source, analysis, index);
        if let Some(verdict) = verdict {
            self.retained_evidence_validity
                .insert(simulation, &source, analysis, verdict);
        }
        let dataset = analysis.dataset_id();
        if let Some(digest) = digest
            && let Some(run) = simulation
                .retained
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset)
            && let Some(place) = run
                .analyses
                .iter()
                .position(|kept| AnalysisPresentationKey::new(dataset, kept) == analysis)
        {
            // The run's other analyses, which were not looked over, are
            // digested here.
            if run.analyses.len() > 1 {
                frame_work::note(frame_work::DatasetWalk::DatasetDigest);
            }
            let digest =
                crate::state::run_store::dataset_digest_knowing(&run.data, (place, digest));
            self.dataset_digests
                .insert(simulation, &source, dataset, digest);
        }
    }
}

pub(crate) use rspice_results::renderability::StructuralGate;

/// One structural gate answered by walking the evidence, for callers that
/// hold a run without a session to memoize against — the printed page, the
/// visualization document's own compatibility check, and the tests.
pub(crate) fn structural_gate_is_answered_directly(
    gate: StructuralGate,
    analysis: &AnalysisResult,
) -> bool {
    rspice_results::renderability::structural_gate_is_answered(
        gate,
        analysis,
        frame_work::note_scan,
    )
}

/// Whether one retained analysis answers a structural gate, resolved once per
/// dataset generation.
///
/// Like [`retained_evidence_is_valid`], this reader checks the retained source
/// revision itself. Tab availability cannot reuse an older structural answer
/// while waiting for a painted frame to invalidate the cache.
pub(crate) fn analysis_answers_structural_gate(
    state: ResultsReads<'_>,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
    gate: StructuralGate,
) -> bool {
    // The ordinary-noise spectrum has a memo of its own, because the noise
    // sheet needs the resolved shape and not only the verdict. Storing the
    // verdict here as well would walk the same samples a second time to
    // learn something already known.
    if gate == StructuralGate::OrdinaryNoiseSpectrum {
        return bode::ordinary_noise_spectrum_is_renderable_in(state, dataset_id, analysis);
    }
    let key = (AnalysisPresentationKey::new(dataset_id, analysis), gate);
    state
        .results
        .structural_gates
        .get_or_insert_with(&state.model.simulation, key, || {
            structural_gate_is_answered_directly(gate, analysis)
        })
}

/// The content digest of one retained dataset.
///
/// Hashing a dataset encodes every retained sample of it. The resolved
/// displayed view carries the digest so it can prove the run behind a
/// document is still the run it resolved — and everything that reads that
/// view calls `run()` to re-check it, several times per frame, for a reader
/// who is not touching anything.
///
/// The retained history revision participates in every memo lookup. Restoring
/// changed content with the same dataset identity and display version cannot
/// make an older view's immutable binding appear valid.
pub(crate) fn retained_dataset_digest(
    reads: ResultsReads<'_>,
    run: &SimulationRun,
) -> crate::product::ContentDigest {
    reads.results.dataset_digests.get_or_insert_with(
        &reads.model.simulation,
        run.dataset_id,
        || {
            frame_work::note(frame_work::DatasetWalk::DatasetDigest);
            run.dataset_content_digest()
        },
    )
}

/// The digest of the netlist text `content`, digested once per edit: the
/// text it was taken of is kept, and only a different text is digested
/// again.
pub(crate) fn netlist_digest(
    results: &ResultsState,
    content: &str,
) -> crate::product::ContentDigest {
    let mut memo = results.netlist_digest.borrow_mut();
    if let Some((text, digest)) = memo.as_ref()
        && text == content
    {
        return *digest;
    }
    frame_work::note(frame_work::DatasetWalk::NetlistDigest);
    let digest = crate::state::content_digest(content);
    *memo = Some((content.to_owned(), digest));
    digest
}

/// The window the reader has pinned on the active sheet, if any.
///
/// A sheet's zoom lives in one of two places: keyed to the analysis, which is
/// how the waveform strips remember a pane per result, or globally per viewer
/// for the single-pane instruments. Export/Print has to carry whichever one
/// the active sheet is actually using — a page taken from a zoomed sheet that
/// reverts to the data extents is not the view that was captured.
///
/// `None` means the reader pinned nothing and the page rules its own extents,
/// which is what it has always done.
///
/// The abscissa always travels. The ordinate travels only from a sheet whose
/// page is the same plot the reader was reading — see [`captured_ordinate`].
pub(crate) fn captured_viewport(
    results: &dyn ResultsPort,
    analysis: AnalysisPresentationKey,
) -> Option<CapturedViewport> {
    let viewer = results.result_viewer();
    let analysis_pane = results.analysis_plot_view_pane(viewer, analysis, 0);
    let view = if analysis_pane.is_zoomed() {
        analysis_pane
    } else {
        results.plot_view(viewer, 0)
    };
    let y = captured_ordinate(viewer, view);
    (view.x.is_some() || y.is_some()).then_some((view.x, y))
}

/// The ordinate half of a capture, which not every sheet can hand over.
///
/// The waveform strip is a stack of panes, split by unit, and its Y zoom is
/// recorded per pane for that reason: one factor across volts and amps would
/// mean nothing. Its printed page is not that stack — `quick_waveform_plot`
/// merges every visible waveform of the analysis into one plot on one linear
/// ordinate — so pane 0's window arrives stated in pane 0's unit and bounds
/// traces drawn in another. On a volts-and-amps strip the page's own clipper
/// then dropped the amps trace outright.
///
/// Resolving how many panes the strip actually has would mean re-deriving
/// `waves::build_models`' unit grouping here, which is the one rule that
/// decides it; a second copy of it would drift. The abscissa is shared across
/// every pane of a strip — `set_shared_x_view` writes it to all of them — so
/// it carries, and the page rules its own ordinate from the data, exactly as
/// it did before any window was carried at all.
fn captured_ordinate(viewer: ResultViewer, view: PlotView) -> Option<(f64, f64)> {
    match viewer {
        ResultViewer::Waves => None,
        _ => view.y,
    }
}

/// The same question for an analysis already in hand, resolved to its key.
///
/// The memo is the owner of that verdict for the whole shell rather than a
/// Results-local convenience. The Verify workspace's evidence gates ask the
/// same question of the same immutable datasets through the results port,
/// which answers from this memo: a second copy of the walk there would be a
/// second answer to keep in step as well as a second million-sample scan.
pub(crate) fn analysis_evidence_is_valid(
    state: ResultsReads<'_>,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> bool {
    retained_evidence_is_valid(state, AnalysisPresentationKey::new(dataset_id, analysis))
}

/// Current retained distributions, including freshly restored runs.
pub(crate) fn histogram_is_available(state: ResultsReads<'_>) -> bool {
    hist::histogram_is_available(state)
}

/// Shared bins for the sheet and exact CSV export.
pub fn active_histogram(
    state: ResultsReads<'_>,
) -> Option<std::sync::Arc<rspice_results::histogram::Histogram>> {
    hist::active_histogram(state)
}

pub fn active_histogram_display(
    state: ResultsReads<'_>,
) -> Option<std::sync::Arc<rspice_results_ui::histogram::display::HistogramDisplay>> {
    hist::active_histogram_display(state)
}

/// Why one analysis' retained evidence is invalid, when it is.
///
/// The verdict comes from the memo, so a sound dataset costs a map lookup; the
/// validator's own reason is produced only on the rare frame there is one to
/// state. Readers that show the reason go through here rather than calling the
/// validator and discarding the answer on every good frame.
pub(crate) fn analysis_evidence_failure(
    state: ResultsReads<'_>,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> Option<String> {
    if analysis_evidence_is_valid(state, dataset_id, analysis) {
        return None;
    }
    Some(
        analysis
            .validate_retained_evidence()
            .err()
            .unwrap_or_else(|| "the saved results failed validation".to_owned()),
    )
}

/// Application composition of viewer state and source-bound caches.
#[derive(Debug, Clone, Default)]
pub struct ResultsState {
    pub session: rspice_results_ui::session::ResultViewerState,
    /// Whether a widget of the result area held the keyboard as the last
    /// frame began: a plot or the ruler reached by Tab, a lane or a marker
    /// strip clicked. Each takes the bare arrows itself. Transient.
    pub(crate) sheet_focus: bool,
    /// The HDL or deck line the reader asked to open from a waveform strip's
    /// event markers, until the application takes it and opens that file at
    /// that line. One-shot and transient.
    pub source_request: Option<rspice_results_ui::waves::logic::SourceLine>,
    /// The reader's named signal sets. They belong to the project, not the
    /// session: saved with it and restored when it opens.
    pub signal_sets: Vec<rspice_results::result_presentation::SignalSet>,
    /// Explicit runtime boundary reported by the operation that owns it.
    /// Source-derived states such as stale, partial, and corrupted are never
    /// stored here; the operational classifier recomputes those from exact
    /// retained evidence and provenance.
    operational_condition: Option<operational_state::ResultRuntimeCondition>,
    /// The retained dataset history this presentation state was last
    /// reconciled against. Transient.
    retained_datasets: HashSet<DatasetId>,
    retained_history_revision: Option<crate::state::RunHistoryRevision>,
    /// Why Latest tracking could not advance one document onto one candidate
    /// dataset.
    ///
    /// Retargeting rebuilds the retained source dataset from the run, so a
    /// refusal that is re-attempted every frame pays for that rebuild every
    /// frame. Keyed by document and held against the exact candidate that was
    /// refused, so a genuinely new run is still tried once. Transient.
    latest_retarget_failures:
        std::collections::HashMap<crate::product::ResultDocumentId, (DatasetId, String)>,
    /// Why an archived run could not be brought back from the document that
    /// holds it, keyed by the exact binding. Decoding an archive reads the
    /// whole run, so a refusal is not re-attempted every frame. Transient.
    archive_restore_failures: std::collections::HashMap<crate::product::DatasetBinding, String>,
    network_matrix: network_matrix::NetworkMatrixState,
    /// Exact retained history and display version behind waveform caches.
    wave_cache_source: Option<(crate::state::RunHistoryRevision, u64)>,
    /// Fingerprint-keyed strip-model cache for the waves viewer.
    models: rspice_results_ui::waves::cache::ModelsCache,
    /// The `digital()` expressions the cached strip models carry as logic
    /// lanes, fingerprinted, so a change to them rebuilds the models.
    logic_lanes_source: u64,
    /// Cached Bode margins + extremes for the active data version.
    pub bode: Option<BodeDerived>,
    /// Each stability analysis's locus and its stability numbers, read off
    /// the run once per data version.
    pub(crate) nyquist: std::collections::HashMap<nyquist::LoopGainSource, nyquist::NyquistDerived>,
    /// `simulation.data_version` last seen by the workspace; when it
    /// advances, the merged event order is read again.
    seen_version: u64,
    /// The run and analysis the workspace last prepared for. New samples, an
    /// overlay or a hidden trace leave the reader's cursors and selection be;
    /// another run or analysis retires what named the one before.
    seen_selection: Option<ShownSelection>,
    /// The runs the quick view has settled on an analysis to show; see
    /// [`settling_analysis`]. Transient.
    pub(crate) settled_runs: HashSet<DatasetId>,
    /// The analysis kind the A/B cursors were last read against. Their X is
    /// kept across runs and analyses of one kind, and cleared for another.
    cursor_kind: Option<crate::state::AnalysisType>,
    /// Which retained samples the decimation and derived caches hold. It
    /// moves when the run history changes, not when the selection does: both
    /// caches key every entry by its run, analysis and trace.
    samples_generation: u64,
    /// The quick view's viewer while a saved document's pane shows its own,
    /// put back when the quick view is shown again.
    pub(crate) quick_view_viewer: Option<ResultViewer>,
    /// The working view the quick view last drew and the drawing state it
    /// left, so what the reader changed is written back to it. Boxed to keep
    /// the results state inside its stack budget. Transient.
    pub(crate) working_view: Option<Box<working_view::QuickViewBinding>>,
    /// What the working view last followed the shown run from, when that
    /// changed nothing. Transient.
    pub(crate) working_view_followed: Option<working_view::FollowedAt>,
    /// The traces the working view is being given away from the frame, and
    /// the rest of the reader's change they belong to. Transient.
    pub(crate) working_view_plotting: working_view::PlottingSlot,
    /// The run the working view waits on to be digested away from the
    /// frame, while it is. Transient.
    pub(crate) working_view_digesting: working_view::DigestSlot,
    /// The runs a working view read while they were still running, so each
    /// is read again once it finishes. A run opened finished is read as it
    /// is: its samples may be kept another way after a save, not grown.
    /// Transient.
    pub(crate) read_while_running: HashSet<DatasetId>,
    /// The differences a two-net probe named while no run was shown, by the
    /// plan whose next run's view takes them in. Transient.
    pub(crate) next_run_expressions: Vec<(crate::product::SimulationPlanId, String)>,
    /// Evaluated expression series, keyed by (stable analysis, expression);
    /// refreshed when the simulation data version advances.
    pub analysis_expr_cache:
        std::collections::HashMap<(AnalysisPresentationKey, String), ExprSeries>,
    /// The run whose attributed failure sites are currently marked on the
    /// drawing, if any.
    ///
    /// Held so the control that marked them can offer to take them back, and
    /// keyed by run so selecting a different dataset does not leave a stale
    /// "Clear" offering to unmark objects another run named. Transient.
    pub marked_failure_run: Option<u64>,
    /// Merged event order for the analysis the EVENTS sheet last drew.
    pub(super) event_order_cache: Option<EventOrderCache>,
    /// Source-aware retained-evidence verdict per analysis; see
    /// [`retained_evidence_is_valid`].
    retained_evidence_validity:
        RetainedMemo<AnalysisPresentationKey, Result<(), std::sync::Arc<str>>>,
    /// Source-aware content digest per dataset; see
    /// [`retained_dataset_digest`].
    dataset_digests: RetainedMemo<DatasetId, crate::product::ContentDigest>,
    /// Source-aware ordinary-noise structure per analysis;
    /// see [`bode::noise_spectrum_shape`].
    noise_spectrum_shapes: RetainedMemo<AnalysisPresentationKey, Option<bode::NoiseSpectrumShape>>,
    /// Source-aware structural verdicts per analysis and question;
    /// see [`analysis_answers_structural_gate`].
    structural_gates: RetainedMemo<(AnalysisPresentationKey, StructuralGate), bool>,
    /// The netlist last digested and its digest; see [`netlist_digest`].
    netlist_digest: std::cell::RefCell<Option<(String, crate::product::ContentDigest)>>,
    /// Whether Compare datasets has a baseline for a candidate analysis; see
    /// [`document_edit::compare::compare_available`].
    compare_availability: RetainedMemo<AnalysisPresentationKey, bool>,
    /// The runs a candidate analysis can be compared with, and the signals
    /// each shares with it; see [`document_edit::compare::choices`].
    compare_baselines:
        RetainedMemo<AnalysisPresentationKey, std::sync::Arc<[document_edit::compare::Baseline]>>,
    /// The comparison running on its worker, while its form is open.
    /// Transient.
    compare_run: Option<std::sync::Arc<document_edit::compare::Running>>,
    /// The cursor jump searching a stored trace, until it answers. Transient.
    cursor_jump: Option<waves::PendingJump>,
    /// The comparison that last answered, until another does. Transient.
    last_comparison: Option<std::sync::Arc<document_edit::compare::Finished>>,
    /// Source-aware statistics per selected trace; see
    /// [`trace_statistics::trace_statistics`].
    trace_statistics:
        RetainedMemo<(AnalysisPresentationKey, usize), trace_statistics::PendingStatistics>,
    /// What the measurement form's expression last measured; see
    /// [`document_edit::measurement::preview`].
    measurement_preview: document_edit::measurement::PreviewMemo,
    /// Memoized viewer projections; see [`view_plans::ViewPlans`].
    plans: view_plans::ViewPlans,
    /// The views each waveform sheet showed, for Zoom previous. Transient.
    view_histories: view_history::WaveViewHistories,
    /// Each view's undo and redo steps. Transient.
    pub(crate) view_edits: view_edits::ViewHistories,
}

/// One evaluated expression series (owned arrays, cheap to clone).
#[derive(Debug, Clone)]
pub struct ExprSeries {
    /// `simulation.data_version` the series was computed against.
    pub version: u64,
    /// Evaluation result, or the error to show on the strip.
    pub series: ExpressionSeriesResult,
}

/// Memoized stability metrics for the selected AC waveform.
#[derive(Debug, Clone, Copy)]
pub struct BodeDerived {
    pub(crate) version: u64,
    pub(crate) analysis_index: usize,
    pub(crate) mag_index: usize,
    pub(crate) metrics: rspice_results::bode::AcBodeMetrics,
}

/// Finite (min, max) of a slice, if any finite values exist.
/// The least and greatest finite sample of `values`, read a block at a time.
pub(super) fn finite_extremes(
    values: &rspice_results::waveform::SharedWaveformValues,
) -> Option<(f64, f64)> {
    frame_work::note_samples(frame_work::FrameSampleRead::TraceExtremes, values.len());
    values
        .samples(..)
        .filter(|value: &f64| value.is_finite())
        .fold(None, |held: Option<(f64, f64)>, value| {
            Some(held.map_or((value, value), |(low, high): (f64, f64)| {
                (low.min(value), high.max(value))
            }))
        })
}

// ---------------------------------------------------------------------------
// shared right-panel furniture
// ---------------------------------------------------------------------------

pub use rspice_results_ui::waveform::waveform_color;

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Results workspace center view (docbar + active viewer).
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>, run_controller: &mut SimulationController) {
    if waves::poll_cursor_jump(state, &Tokens::get(ui.ctx())) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(30));
    }
    record_key_area(ui, state.results);
    show_with_chrome(ui, state, run_controller);
    waves::marker_dialog::show(ui.ctx(), state);
    view_history::observe(ui, state, view_history::WaveSheet::QuickView);
}

/// Where the result area lay this frame, for whether the Results keys stand
/// down: the area a focused widget must lie in to keep them.
fn key_area_id() -> egui::Id {
    egui::Id::new("workbench.results.key-area")
}

/// Note `ui`'s area as the one the Results keys act over, and whether a
/// widget in it holds the keyboard.
fn record_key_area(ui: &Ui, results: &mut ResultsState) {
    let area = ui.max_rect();
    ui.ctx()
        .data_mut(|data| data.insert_temp(key_area_id(), area));
    results.sheet_focus = focus_in_key_area(ui.ctx()) == Some(true);
}

/// Whether the widget that holds the keyboard lies in the result area, or
/// `None` when none holds it.
fn focus_in_key_area(ctx: &egui::Context) -> Option<bool> {
    let focused = ctx.memory(|memory| memory.focused())?;
    let area = ctx.data(|data| data.get_temp::<egui::Rect>(key_area_id()));
    Some(match (area, ctx.read_response(focused)) {
        (Some(area), Some(response)) => area.contains(response.rect.center()),
        _ => false,
    })
}

/// Whether the Results keys stand down this frame, as the shortcut resolver
/// asks before it reads them in Results.
///
/// They stand down while a text field has the keyboard, so no letter is
/// stolen from a filter, an expression or a label; while a marker's dialog
/// is open over the sheet; and while a widget outside the result area holds
/// focus, as a data browser row does, whose arrows fold and unfold it. A
/// widget focused inside the result area, a plot, a lane or a legend chip
/// reached by Tab, keeps them, the bare arrows aside, which it takes
/// itself: focusing a pane turned every key off before.
pub fn keys_stand_down(ctx: &egui::Context, results: &ResultsState) -> bool {
    ctx.text_edit_focused()
        || results.session.marker_edit.is_some()
        || focus_in_key_area(ctx) == Some(false)
}

/// Where the selected trace is among the shown run's analyses and their
/// waveforms, when one is selected on the sheet.
pub(crate) fn selected_trace_on_sheet(state: ResultsReads<'_>) -> Option<(usize, usize)> {
    if !viewer_uses_wave_stack(state.results.session.viewer) {
        return None;
    }
    let selected = state
        .results
        .valid_selected_trace(&state.model.simulation)?;
    let analysis_key = selected.analysis_key();
    let source_name = selected.source_name();
    let run = state.model.simulation.active_run()?;
    run.analyses
        .iter()
        .enumerate()
        .find_map(|(index, analysis)| {
            (AnalysisPresentationKey::new(run.dataset_id, analysis) == analysis_key).then(|| {
                analysis
                    .waveforms
                    .iter()
                    .position(|waveform| waveform.name == source_name)
                    .map(|waveform_index| (index, waveform_index))
            })?
        })
}

/// Take the selected trace off the sheet, as Delete does.
///
/// Hiding is the reversible form of removal here: the retained dataset is
/// immutable, so a trace leaves the plot by losing visibility, comes back
/// from the pane's add-signal menu, and Undo puts it back.
pub(crate) fn hide_selected_trace(state: &mut ResultsParts<'_>) {
    if let Some((analysis_index, waveform_index)) = selected_trace_on_sheet(state.results_reads()) {
        waves::toggle_visibility(state, analysis_index, waveform_index);
        state.results.session.selected_trace = None;
    }
}

/// Render one project-owned result document selected by its stable identity.
///
/// Unlike the dataset quick-view, this projection is driven by the exact
/// pages, panes, viewer identities, and immutable bindings retained in the
/// project document.
pub fn show_persistent_document(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
    document_id: crate::product::ResultDocumentId,
) {
    // A project-owned document is the same workspace with a retained page
    // layout, so it answers the same keys and owns the same marker dialog.
    record_key_area(ui, state.results);
    persistent_document::show(ui, state, run_controller, document_id);
    waves::marker_dialog::show(ui.ctx(), state);
    view_history::observe(ui, state, view_history::WaveSheet::Document(document_id));
}

/// Resolve and select the primary binding of a project-owned result document.
///
/// Document-tab activation and post-create activation share this path so a
/// document can never select one retained result when created and a different
/// one when reopened.
pub fn activate_persistent_document(
    state: &mut ResultsParts<'_>,
    document_id: crate::product::ResultDocumentId,
) -> bool {
    persistent_document::activate(state, document_id)
}

/// The dock's Plot page, on the pages that plot in the dock: its toolbar
/// band, then the plot, flush to the dock's sides and foot. This is the same
/// canonical retained result state Results draws, not a second result
/// document, so Open in Results shows the same view.
pub fn show_plot_page(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    // The page plots; the run's other sheets (its table, spectrum, eye and
    // manifest) are in Results, a press of the band's Open in Results away.
    let sheets = pane_sheets(state.results_reads());
    if let Some(&first) = sheets.first()
        && !sheets.contains(&state.results.session.viewer)
    {
        state.results.session.viewer = first;
    }
    prepare_quick_view(ui, state);
    let open_in_results = rspice_app_workbench::workbench::docks::console::toolbar(ui, |ui| {
        plot_page_band(ui, state, &sheets)
    });
    crate::ui::plot::set_interaction_mode(
        ui.ctx(),
        state.results.session.plot_tool.interaction_mode(),
    );
    let well = ui.available_rect_before_wrap();
    ui.allocate_ui(well.size(), |ui| {
        ui.set_min_height(well.height());
        if sheets.is_empty() && state.model.simulation.retained.has_results() {
            let t = Tokens::get(ui.ctx());
            ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
            // An analysis that failed has nothing to plot for that reason:
            // its card takes the well, as on its sheet in Results.
            let viewer = state.results.session.viewer;
            let status = operational_state::classify_viewer(state, viewer);
            if !(status.blocks_visuals
                && operational_state::show_result_operational_status(ui, state, &status))
            {
                well_hint(
                    ui,
                    "Nothing to plot from this run: its values are on the drawing, and Open in Results shows every sheet",
                );
            }
        } else if viewer_well_is_open(ui, state) {
            waves::show_compact(ui, state);
        }
    });
    working_view::capture_at_frame_end(state);
    waves::marker_dialog::show(ui.ctx(), state);
    if open_in_results {
        state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    }
}

/// The Plot page's band: the result's state when it is not complete, the
/// sheet when the run has more than one to plot, the cursor tools, zoom and
/// fit, and at its right end what the cursors read and Open in Results.
/// Returns whether Open in Results was pressed.
fn plot_page_band(ui: &mut Ui, state: &mut ResultsParts<'_>, sheets: &[ResultViewer]) -> bool {
    let mut open_in_results = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = PLOT_BAND_GAP;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            open_in_results = crate::ui::widgets::Button::new("Open in Results")
                .compact()
                .show(ui)
                .on_hover_text("Open this plot in Results: measurements, cursors and every sheet")
                .clicked();
            let t = Tokens::get(ui.ctx());
            if let Some(readout) = waves::inline_cursor_readout(state, &t) {
                ui.label(
                    egui::RichText::new(readout)
                        .font(crate::ui::theme::mono(
                            crate::ui::tokens::FS_0,
                            crate::ui::theme::FontWeight::Regular,
                        ))
                        .color(t.color.text_dim),
                );
            }
            let rest = ui.available_size();
            ui.allocate_ui_with_layout(
                rest,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_clip_rect(ui.max_rect().intersect(ui.clip_rect()));
                    if operational_state::show_status_mark(ui, state) {
                        instrument::separator(ui);
                    }
                    if sheets.len() > 1 {
                        let labels: Vec<&str> =
                            sheets.iter().map(|sheet| sheet.tab_label()).collect();
                        let mut chosen = sheets
                            .iter()
                            .position(|&sheet| sheet == state.results.session.viewer)
                            .unwrap_or(0);
                        if rspice_ui_kit::widgets::segmented(
                            ui,
                            "results.plot-page.sheet",
                            &labels,
                            &mut chosen,
                            rspice_ui_kit::widgets::SegmentedWidth::Natural,
                        ) {
                            state.results.session.viewer = sheets[chosen];
                        }
                        instrument::separator(ui);
                    }
                    instrument::show_plot_tools(ui, state);
                },
            );
        });
    });
    open_in_results
}

/// The gap between two items of the Plot page's band.
const PLOT_BAND_GAP: f32 = 6.0;

/// The sheets the dock's Plot page can show for the active run: the ones
/// that plot it against time, a sweep or frequency.
pub(crate) fn pane_sheets(state: ResultsReads<'_>) -> Vec<ResultViewer> {
    ResultViewer::all()
        .filter(|&viewer| pane_sheet(viewer) && viewer_availability(state, viewer).available)
        .collect()
}

/// Whether the dock's Plot page draws `viewer`: the sheets that plot a run
/// against time, a sweep or frequency.
const fn pane_sheet(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// The state of the result the dock's Plot page draws, when it is one the
/// reader should notice, for the page's tab to carry.
pub fn plot_page_status(
    state: &mut ResultsParts<'_>,
) -> Option<rspice_results_ui::operational::ResultOperationalStatus> {
    let viewer = plot_page_viewer(state.results_reads())?;
    operational_state::notable_status(state, viewer)
}

/// The sheet the Plot page draws: the one shown when the page can draw it,
/// else the first the run can plot.
pub(crate) fn plot_page_viewer(state: ResultsReads<'_>) -> Option<ResultViewer> {
    let viewer = state.results.session.viewer;
    let sheets = pane_sheets(state);
    if sheets.contains(&viewer) {
        Some(viewer)
    } else {
        sheets.first().copied()
    }
}

/// Settle the quick view's selection and working view before it draws, in
/// Results and on the dock's Plot page alike.
fn prepare_quick_view(ui: &Ui, state: &mut ResultsParts<'_>) {
    // Every quick-style entry leaves the persistent projection here: the
    // Plot page renders the same strips, and a pane context left over from a
    // project-owned document would scope its plot views, cursors and marker
    // edits onto them.
    state.results.session.leave_persistent_document();
    let family = document_edit::family::quick_view_selection(state.results_reads());
    state.results.set_sample_selection(family);
    prepare_viewer_state(state);
    working_view::wake_on_plotted(state, ui.ctx());
    working_view::sync(state);
}

fn show_with_chrome(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
) {
    prepare_quick_view(ui, state);
    let plan_before_docbar = state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    show_docbar(ui, state);
    if result_stage_bar_visible(state.results_reads()) {
        show_sheet_bar(ui, state);
    }
    crate::ui::plot::set_interaction_mode(
        ui.ctx(),
        state.results.session.plot_tool.interaction_mode(),
    );
    let plan_after_docbar = state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    if plan_after_docbar != plan_before_docbar {
        state.preflight.invalidate();
        run_controller.clear_prepared_run();
    }

    // What the cursors and markers read is the dock's Cursors page, so the
    // viewer well takes the whole stage.
    let available = ui.available_rect_before_wrap();
    ui.allocate_ui(available.size(), |ui| {
        ui.set_min_height(available.height());
        show_viewer_well(ui, state, run_controller);
    });
    working_view::capture_at_frame_end(state);
}

/// Whether the stage shows its bar below the viewer tabs: whenever the run
/// has results. Every sheet's bar is the one 31 px bar, plot and
/// structured document alike, and carries its copy and export outputs in the
/// same place, so no sheet goes without one.
fn result_stage_bar_visible(state: ResultsReads<'_>) -> bool {
    state
        .model
        .simulation
        .active_run()
        .is_some_and(|run| !run.analyses.is_empty())
}

/// Whether a sheet reads cursors and markers: the waveform sheets do;
/// structured documents (OP, tables) have no cursor to report.
pub(crate) fn sheet_has_cursors(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// The sheet a bound pane actually draws, once the analysis behind it is known.
///
/// Several viewer documents cover a pair of sheets that differ only by what the
/// analysis turned out to be — one waveform renderer serves transient and DC
/// sweeps, one spectrum renderer serves FFT and harmonic balance, one AC
/// renderer serves Bode and ordinary noise. Which of the pair to draw is one
/// question, so it is answered once, here, for every pane that draws.
pub(crate) fn project_viewer_for_analysis(
    viewer: ResultViewer,
    analysis: &crate::state::AnalysisResult,
) -> ResultViewer {
    use crate::state::AnalysisType;

    match viewer {
        ResultViewer::Waves if analysis.analysis_type == AnalysisType::DcSweep => {
            ResultViewer::DcSweep
        }
        ResultViewer::Waves
            if matches!(
                analysis.analysis_type,
                AnalysisType::Parametric | AnalysisType::Corner
            ) =>
        {
            ResultViewer::Sweep
        }
        ResultViewer::Fft | ResultViewer::HarmonicBalance
            if harmonic_balance_analysis_is_renderable(analysis) =>
        {
            ResultViewer::HarmonicBalance
        }
        ResultViewer::Bode
            if rspice_results::noise_spectrum::qpnoise_is_renderable(analysis.as_ref())
                || bode::ordinary_noise_spectrum_is_renderable(analysis) =>
        {
            ResultViewer::NoiseContrib
        }
        _ => viewer,
    }
}

pub(crate) fn viewer_is_available(state: ResultsReads<'_>, viewer: ResultViewer) -> bool {
    viewer_availability(state, viewer).available
}

pub(crate) fn viewer_unavailability_reason(
    state: ResultsReads<'_>,
    viewer: ResultViewer,
) -> Option<&'static str> {
    let availability = viewer_availability(state, viewer);
    (!availability.available).then_some(availability.reason)
}

/// Render one already-resolved persistent pane through an exact existing
/// viewer. The caller owns binding and compatibility checks; this helper only
/// refreshes renderer caches and deliberately restores the requested viewer
/// after quick-view reconciliation so no fallback viewer can be substituted.
///
/// `family` is the slice the pane's kept family policy shows. Panes of one
/// page draw in turn, so the selection changes between them every frame; a
/// change of selection clears the session's cursors and hidden family
/// members, which here were projected from this pane's own document state
/// and have to survive it, or capturing the pane would delete them.
pub fn show_persistent_pane_viewer(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
    viewer: ResultViewer,
    family: Option<SourceSampleSelection>,
) {
    let session = &state.results.session;
    let cursors = (
        session.cursors,
        session.cursor_strip,
        session.cursor_a_anchor.clone(),
    );
    let hidden = session.hidden_family_traces.clone();
    let results = &mut state.results;
    results.set_sample_selection(family);
    (
        results.session.cursors,
        results.session.cursor_strip,
        results.session.cursor_a_anchor,
    ) = cursors;
    results.session.hidden_family_traces = hidden;
    prepare_viewer_state(state);
    show_document_viewer(state, viewer);
    show_viewer_well(ui, state, run_controller);
}

/// The run and analysis on screen, by identity, and the kind of analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShownSelection {
    dataset: crate::product::DatasetId,
    analysis: Option<u64>,
    kind: Option<crate::state::AnalysisType>,
}

/// The sheet a run that has written nothing yet draws its first results
/// on: the one its plan's first plotted analysis streams into, as its
/// prepared tasks say, so the run opens where its plot arrives. A run with
/// no prepared tasks (a deck run) says nothing.
fn starting_sheet(run: &SimulationRun) -> Option<ResultViewer> {
    run.prepared_receipt()?
        .tasks()
        .iter()
        .find_map(|task| ResultViewer::live_sheet(task.result_analysis_type()))
}

/// The sheet the shown analysis opens on, from the one table of default
/// sheets and what the analysis can fill.
pub(crate) fn opening_sheet(state: ResultsReads<'_>) -> Option<ResultViewer> {
    let shown = shown_analysis(&state.model.simulation).map(|analysis| analysis.analysis_type);
    rspice_results::result_presentation::opening_sheet(shown, |viewer| {
        viewer_is_available(state, viewer)
    })
}

/// The analysis the data browser shows: the one picked, or, before any is,
/// the run's first, which is the one the browser marks.
fn shown_analysis(simulation: &crate::state::SimulationState) -> Option<&AnalysisResult> {
    simulation
        .active_analysis()
        .or_else(|| simulation.active_run()?.analyses.first())
}

/// The analysis the active run opens on when nothing has picked one: its
/// first curves, else its first analysis.
fn opening_analysis(state: ResultsReads<'_>) -> Option<usize> {
    let run = state.model.simulation.active_run()?;
    let kinds = run
        .analyses
        .iter()
        .map(|analysis| analysis.analysis_type)
        .collect::<Vec<_>>();
    rspice_results::result_presentation::opening_analysis(&kinds, |index, sheet| {
        let analysis = &run.analyses[index];
        rspice_results::renderability::viewer_can_render(sheet, analysis, |gate| {
            analysis_answers_structural_gate(state, run.dataset_id, analysis, gate)
        })
    })
}

/// How the active run comes to be shown, which decides what it opens on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShowingRun {
    /// The quick view draws the run while it is written.
    Writing,
    /// The reader opened the run: a run finished, a run imported, a run
    /// picked in the data browser.
    Opened,
}

/// The analysis the active run is to show, the first time it is shown: the
/// one its view keeps, which the reader last picked in the data browser in
/// a run of the plan, when the run solved it. A run still writing waits for
/// that analysis and shows what it writes meanwhile; an opened run that did
/// not solve it opens on its curves, else on its first analysis.
///
/// `None` once the run is settled, while it waits, and for a finished run
/// the quick view merely draws: whatever showed that run chose its analysis.
pub(crate) fn settling_analysis(state: ResultsReads<'_>, showing: ShowingRun) -> Option<usize> {
    let run = state.model.simulation.active_run()?;
    if run.analyses.is_empty() || state.results.settled_runs.contains(&run.dataset_id) {
        return None;
    }
    let writing = !run.lifecycle.is_terminal();
    if showing == ShowingRun::Writing && !writing {
        return None;
    }
    let kept = working_view::kept_analysis(state, run);
    let solved = kept.and_then(|kept| {
        run.analyses
            .iter()
            .position(|analysis| create_document::source_analysis_identity(run, analysis) == kept)
    });
    match (solved, showing) {
        (Some(index), _) => Some(index),
        (None, ShowingRun::Writing) => None,
        (None, ShowingRun::Opened) if writing && kept.is_some() => None,
        (None, ShowingRun::Opened) => opening_analysis(state),
    }
}

/// Settle the active run on what it opens on when the reader opens it.
pub(crate) fn open_shown_run(state: &mut ResultsParts<'_>) {
    if let Some(index) = settling_analysis(state.results_reads(), ShowingRun::Opened) {
        settle_shown_analysis(state.results, &mut state.model.simulation, index);
    }
}

/// Show analysis `index` of the active run and count the run settled: from
/// now on it shows whatever the reader picks in it.
pub(crate) fn settle_shown_analysis(
    results: &mut ResultsState,
    simulation: &mut crate::state::SimulationState,
    index: usize,
) {
    let Some(dataset) = simulation.active_run().map(|run| run.dataset_id) else {
        return;
    };
    if simulation.view.active_analysis_idx != Some(index) {
        simulation.select_analysis(index);
    }
    let runs = &simulation.retained.runs;
    results
        .settled_runs
        .retain(|held| runs.iter().any(|run| run.dataset_id == *held));
    results.settled_runs.insert(dataset);
}

/// Show the analysis the quick view's run, still writing, waits for once it
/// has written it. A persistent pane shows the analysis it binds.
fn settle_writing_run(state: &mut ResultsParts<'_>) {
    if state.results.session.persistent_pane_context.is_none()
        && let Some(index) = settling_analysis(state.results_reads(), ShowingRun::Writing)
    {
        settle_shown_analysis(state.results, &mut state.model.simulation, index);
    }
}

/// The reader picked the shown analysis in the data browser: the run shows
/// it from now on, and the run's view keeps it, so the plan's next run and
/// the project reopened show it too.
pub(crate) fn keep_picked_analysis(state: &mut ResultsParts<'_>) {
    let simulation = &state.model.simulation;
    let (Some(run), Some(analysis)) = (simulation.active_run(), simulation.active_analysis())
    else {
        return;
    };
    let dataset = run.dataset_id;
    let kept = create_document::source_analysis_identity(run, analysis);
    state.results.settled_runs.insert(dataset);
    working_view::keep_analysis(state, dataset, kept);
}

fn shown_selection(simulation: &crate::state::SimulationState) -> Option<ShownSelection> {
    let run = simulation.active_run()?;
    let analysis = simulation.active_analysis();
    Some(ShownSelection {
        dataset: run.dataset_id,
        analysis: analysis.map(|analysis| analysis.id),
        kind: analysis.map(|analysis| analysis.analysis_type),
    })
}

pub fn prepare_viewer_state(state: &mut ResultsParts<'_>) {
    synchronize_quick_view_dataset_authority(state);
    settle_writing_run(state);
    state
        .results
        .reconcile_retained_datasets(&state.model.simulation);
    let data_version = state.model.simulation.view.data_version;
    let shown = shown_selection(&state.model.simulation);
    let results = &mut *state.results;
    results.synchronize_wave_caches(&state.model.simulation);
    if results.seen_version != data_version {
        results.seen_version = data_version;
        // The merged event order belongs to the retained dataset version.
        results.event_order_cache = None;
    }
    // New samples, an added overlay or a hidden trace change the data
    // version and nothing the reader placed. Another run or analysis
    // retires what named the one before; the cursors go only when it reads
    // X in another quantity, or when nothing is shown.
    if results.seen_selection != shown {
        results.seen_selection = shown;
        let kind = shown.and_then(|selection| selection.kind);
        let other_quantity = match (results.cursor_kind, kind) {
            (Some(placed), Some(now)) => placed != now,
            _ => shown.is_none(),
        };
        if other_quantity {
            results.session.clear_cursors();
            results.session.horizontal_cursors.clear();
        }
        if shown.is_none() {
            results.cursor_kind = None;
        } else if kind.is_some() {
            results.cursor_kind = kind;
        }
        results.session.active_wave_pane = None;
        results.session.selected_trace = None;
        results.session.selected_result_artifact = None;
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.session.rf_pin.clear();
        // Runtime operation failures and recovery notices belong to the
        // dataset generation that reported them.
        results.operational_condition = None;
    }
    results
        .session
        .cache
        .ensure_version(results.samples_generation);
    results
        .session
        .derived
        .ensure_version(results.samples_generation);

    smith::synchronize_active_analysis(state);
    reconcile_active_viewer(state);
}

/// Keep the ordinary Results quick-view's global simulation projection bound
/// to the stable dataset document the reader has open.
///
/// Most viewer implementations intentionally consume `SimulationState`'s
/// selected run. Document activation establishes that projection, but an
/// asynchronous completion or another workflow can subsequently move the
/// selector. Reasserting it at the Results frame boundary prevents every
/// downstream viewer, inspector, cursor, and measurement from reading a
/// background run while the document bar still names another dataset.
fn synchronize_quick_view_dataset_authority(state: &mut ResultsParts<'_>) -> bool {
    let Some(WorkspaceDocumentId::ResultDataset(dataset_id)) =
        state.workbench.documents.active(Workspace::Results)
    else {
        return false;
    };
    let Some(run_index) = state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == *dataset_id)
    else {
        return false;
    };
    if state.model.simulation.view.active_run_idx == Some(run_index) {
        return false;
    }
    state.model.simulation.select_run(run_index)
}

/// Show a saved document's viewer, keeping the quick view's to put back when
/// the quick view is shown again. A document wrote its viewer over the quick
/// view's, so a Bode pane in one window turned the quick view in another
/// into a Bode plot.
pub(crate) fn show_document_viewer(state: &mut ResultsParts<'_>, viewer: ResultViewer) {
    let results = &mut *state.results;
    if results.session.viewer == viewer {
        return;
    }
    if results.quick_view_viewer.is_none() {
        results.quick_view_viewer = Some(results.session.viewer);
    }
    results.session.viewer = viewer;
}

/// Put the quick view's viewer back, if a saved document's pane took it.
pub(crate) fn restore_quick_view_viewer(results: &mut ResultsState) {
    if let Some(viewer) = results.quick_view_viewer.take() {
        results.session.viewer = viewer;
    }
}

/// Record what a single-canvas sheet's axes just spanned.
///
/// Called by the sheet that drew, because only it knows which plot on the
/// sheet is the one the inspector speaks for.
pub(super) fn record_drawn_axes(
    results: &mut ResultsState,
    viewer: ResultViewer,
    response: &crate::ui::plot::PlotResponse,
) {
    results.session.drawn_axes.insert(viewer, response.axes);
    results
        .session
        .drawn_units
        .insert(viewer, response.units.clone());
}

/// The interval the active sheet's axis is showing, pinned or fitted.
pub fn active_axis_range(
    state: ResultsReads<'_>,
    facts: &waves::ActivePaneFacts,
    axis: PaneAxis,
) -> Option<(f64, f64)> {
    let viewer = state.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return match axis {
            PaneAxis::X => facts.x_extent,
            PaneAxis::Y => facts.y_extent,
        };
    }
    let drawn = state.results.session.drawn_axes.get(&viewer).copied();
    let pinned = state.results.session.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.or_else(|| drawn.map(|(x, _)| x)),
        PaneAxis::Y => pinned.y.or_else(|| drawn.map(|(_, y)| y)),
    }
}

/// Whether the active sheet's axis is pinned to an explicit interval rather
/// than fitting its data.
pub fn active_axis_is_pinned(state: ResultsReads<'_>, axis: PaneAxis) -> bool {
    let viewer = state.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::active_pane_axis_is_pinned(state.results, axis);
    }
    let pinned = state.results.session.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.is_some(),
        PaneAxis::Y => pinned.y.is_some(),
    }
}

/// Pin the active sheet's axis to an explicit interval, or clear it back to
/// automatic fit.
///
/// Routes on [`viewer_uses_wave_stack`] for the same reason every other
/// viewport gesture does: the stack keys its viewports by analysis, and a
/// write to the single-canvas store would land where the sheet never reads.
pub fn set_active_axis_range(
    tokens: &Tokens,
    state: &mut ResultsParts<'_>,
    axis: PaneAxis,
    range: Option<(f64, f64)>,
) -> bool {
    let viewer = state.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::set_active_pane_axis_range(tokens, state, axis, range);
    }
    let view =
        state
            .results
            .session
            .plot_view_pane_mut_for(viewer, PlotPresentationKey::Global(0), 0);
    match axis {
        PaneAxis::X => view.x = range,
        PaneAxis::Y => view.y = range,
    }
    true
}

fn show_viewer_well(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
) {
    if !viewer_well_is_open(ui, state) {
        return;
    }
    let viewer = state.results.session.viewer;
    match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => waves::show(ui, state),
        ResultViewer::Bode => waves::show_bode(ui, state),
        ResultViewer::SParameters => s_parameters::show(ui, state),
        ResultViewer::Fft => {
            if ensure_derived(ui, state, run_controller, ActiveViewer::Fft) {
                fft::show(ui, state);
            }
        }
        ResultViewer::HarmonicBalance => harmonic_balance::show(ui, state),
        ResultViewer::PhaseNoise => phase_noise::show(ui, state),
        ResultViewer::Eye => {
            if ensure_derived(ui, state, run_controller, ActiveViewer::EyeDiagram) {
                eye::show(ui, state);
            }
        }
        ResultViewer::Hist => hist::show(ui, state),
        ResultViewer::Op => op_inspector::show(ui, state),
        ResultViewer::NoiseContrib => waves::show_noise(ui, state),
        ResultViewer::Contribution => sensitivity::show(ui, state),
        ResultViewer::TransferFunction => transfer_function::show(ui, state),
        ResultViewer::Table => table::show(ui, state),
        ResultViewer::Nyquist => nyquist::show(ui, state),
        ResultViewer::Smith => smith::show(ui, state),
        ResultViewer::Polar => polar::show(ui, &mut SheetContext::of(state)),
        ResultViewer::NetworkMatrix => network_matrix::show(ui, &mut SheetContext::of(state)),
        ResultViewer::PoleZero => pz::show(ui, state),
        ResultViewer::Floquet => floquet::show(ui, state),
        ResultViewer::Scatter => {
            let followed = scatter::show(ui, &mut SheetContext::of(state));
            if followed.is_some() {
                open_in_verify(
                    state,
                    crate::state::VerificationPage::Specifications,
                    followed,
                );
            }
        }
        ResultViewer::BoxViolin => {
            let followed = box_violin::show(ui, &mut SheetContext::of(state));
            if followed.is_some() {
                open_in_verify(
                    state,
                    crate::state::VerificationPage::Specifications,
                    followed,
                );
            }
        }
        ResultViewer::Events => events::show(ui, state),
        ResultViewer::Soa => soa::show(ui, state),
        ResultViewer::Optimization => optimization::show(ui, state),
        ResultViewer::Sweep => sweep::show(ui, &mut SheetContext::of(state)),
    }
}

/// Paint the viewer well's backdrop and name it, then say whether a viewer
/// draws in it: not while the result's state takes the well over, nor while
/// there are no results at all, when the well says so instead.
fn viewer_well_is_open(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    apply_pending_view_gesture(ui, state);
    let t = Tokens::get(ui.ctx());
    // The document well backdrop; viewers paint on top.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
    let viewer = state.results.session.viewer;
    let panel = ui.interact(
        well,
        ui.id().with(("result-viewer-panel", viewer)),
        egui::Sense::hover(),
    );
    let panel_label = format!("{} result viewer", viewer.tab_label());
    panel.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, &panel_label));
    ui.ctx().accesskit_node_builder(panel.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(panel_label);
    });

    let operational = operational_state::classify_viewer(state, viewer);
    if operational_state::show_result_operational_status(ui, state, &operational) {
        return false;
    }

    if !state.model.simulation.retained.has_results() {
        let shortcut = state.model.preferences.shortcuts().resolved_label(
            state.commands,
            ids::RUN_SIMULATION,
            runtime_command_platform(ui.ctx()),
            ui.ctx().os(),
        );
        well_hint(ui, &no_results_hint(&shortcut));
        return false;
    }
    true
}

fn show_docbar(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    show_docbar_for_family(ui, state, None, chrome::bars::DocumentBarSource::QuickView);
}

/// The docbar over a project result document. Returns the index of the page
/// the reader picked, when they picked one.
pub(super) fn show_persistent_docbar(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    family_label: &str,
    pages: &[String],
    page: usize,
) -> Option<usize> {
    show_docbar_for_family(
        ui,
        state,
        Some(family_label),
        chrome::bars::DocumentBarSource::Document { pages, page },
    )
}

fn show_docbar_for_family(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    family_label: Option<&str>,
    source: chrome::bars::DocumentBarSource<'_>,
) -> Option<usize> {
    let available = ResultViewer::all().filter(|&viewer| {
        family_label.is_none_or(|family| family_allows_viewer(family, viewer))
            && viewer_availability(state.results_reads(), viewer).available
    });
    let actions = chrome::bars::document_bar(ui, state.results.session.viewer, available, source);
    if let Some(viewer) = actions.viewer {
        state.results.session.viewer = viewer;
    }
    if actions.create_document {
        create_document::open(state);
    } else if actions.add_pane {
        document_edit::open_add_pane(state);
    } else if actions.open_properties {
        document_edit::open_properties(state);
    }
    actions.page
}

/// The domain controls a custom-canvas sheet owns, drawn left-aligned in its
/// bar. `false` means the sheet owns none and the bar states its purpose.
///
/// The controls live with the sheet that reads them rather than in one
/// growing match here: each is a statement about that sheet's own evidence —
/// which network term, which two columns, which normalization — and the sheet
/// is the only place that knows what the retained result can offer.
fn sheet_domain_controls(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let viewer = state.results.session.viewer;
    match viewer {
        ResultViewer::Smith => return smith::domain_bar(ui, state),
        ResultViewer::Fft => return fft::domain_bar(ui, state),
        ResultViewer::Eye => return eye::domain_bar(ui, state),
        ResultViewer::HarmonicBalance => return harmonic_balance::domain_bar(ui, state),
        ResultViewer::Soa => return soa::domain_bar(ui, state),
        ResultViewer::Events => return events::domain_bar(ui, state),
        ResultViewer::Nyquist => return nyquist::domain_bar(ui, state),
        ResultViewer::PoleZero => return pz::domain_bar(ui, state),
        ResultViewer::Hist => return hist::domain_bar(ui, state),
        ResultViewer::Op => return op_inspector::domain_bar(ui, state),
        ResultViewer::PhaseNoise => return phase_noise::domain_bar(ui, state),
        ResultViewer::Optimization => return optimization::domain_bar(ui, state),
        _ => {}
    }
    let mut context = SheetContext::of(state);
    match viewer {
        ResultViewer::Polar => polar::domain_bar(ui, &mut context),
        ResultViewer::Contribution => sensitivity::domain_bar(ui, &mut context),
        ResultViewer::Scatter => scatter::domain_bar(ui, &mut context),
        ResultViewer::BoxViolin => box_violin::domain_bar(ui, &mut context),
        ResultViewer::NetworkMatrix => network_matrix::domain_bar(ui, &mut context),
        ResultViewer::Sweep => sweep::domain_bar(ui, &mut context),
        _ => false,
    }
}

fn hidden_wave_strip_count(state: ResultsReads<'_>) -> usize {
    let Some(run) = state.model.simulation.active_run() else {
        return 0;
    };
    state
        .results
        .session
        .hidden_strips
        .iter()
        .filter(|key| key.dataset_id() == run.dataset_id && key.resolve(run).is_some())
        .count()
}

/// Why one analysis' retained evidence is less than a complete run, if it is.
///
/// The typed-evidence sheets all refuse to draw an unsuccessful analysis, so
/// their reader is never misled. The waveform sheets deliberately do draw
/// one — where a transient stopped converging is exactly what an engineer
/// opens the plot to find out — and for a long time they drew it identically
/// to a run that finished. A curve that ends early is not visibly different
/// from a sweep that was specified to end there.
pub(crate) fn incomplete_evidence_reason(analysis: &AnalysisResult) -> Option<&'static str> {
    if !analysis.success {
        return Some("the run did not complete — these samples stop where it failed");
    }
    frame_work::note(frame_work::DatasetWalk::EvidenceValidation);
    // A stored trace still being read has not failed.
    if analysis
        .validate_retained_evidence()
        .is_err_and(|cause| !rspice_results::waveform::is_pending_read(&cause))
    {
        return Some("the saved results failed validation");
    }
    None
}

/// The same question, answered from the memo instead of the validator.
///
/// Identical verdict to [`incomplete_evidence_reason`]; the sheet bar asks it
/// every frame, and on the wave stack it asks for every analysis of the run,
/// so this is the caller that must not walk the samples.
fn memoized_incomplete_evidence_reason(
    state: ResultsReads<'_>,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> Option<&'static str> {
    if !analysis.success {
        return Some("the run did not complete — these samples stop where it failed");
    }
    (!analysis_evidence_is_valid(state, dataset_id, analysis))
        .then_some("the saved results failed validation")
}

/// The same question for whichever analysis the active sheet is speaking for.
pub fn active_incomplete_evidence_reason(state: ResultsReads<'_>) -> Option<&'static str> {
    let run = state.model.simulation.active_run()?;
    if viewer_uses_wave_stack(state.results.session.viewer) {
        // The stack draws every analysis of the run at once, so the bar
        // speaks for the run: one failed strip makes the sheet's evidence
        // incomplete even when the strip beside it converged.
        return run.analyses.iter().find_map(|analysis| {
            memoized_incomplete_evidence_reason(state, run.dataset_id, analysis)
        });
    }
    memoized_incomplete_evidence_reason(
        state,
        run.dataset_id,
        state.model.simulation.active_analysis()?,
    )
}

/// What the sheet bar says: the sheet and what it reads (the run for the
/// wave stack, the analysis otherwise), why it cannot draw it when it
/// cannot, a caution when the evidence is incomplete, and, when a finished
/// run's analysis has none of its own sheets to show it, one sentence
/// naming what is missing. The gate's own wording for an available sheet
/// ("... is available") says nothing the drawn sheet does not.
pub fn sheet_purpose(state: ResultsReads<'_>) -> String {
    let viewer = state.results.session.viewer;
    let simulation = &state.model.simulation;
    let subject = if viewer_uses_wave_stack(viewer) {
        simulation.active_run().map(|run| run.label.as_str())
    } else {
        simulation
            .active_analysis()
            .map(|analysis| analysis.label.as_str())
    };
    let availability = viewer_availability(state, viewer);
    let mut parts = vec![viewer.tab_label().to_owned()];
    parts.extend(
        subject
            .filter(|subject| !subject.is_empty())
            .map(str::to_owned),
    );
    if !availability.available {
        parts.push(availability.reason.to_owned());
    }
    parts.extend(active_incomplete_evidence_reason(state).map(str::to_owned));
    parts.extend(
        unfilled_sheet(state)
            .filter(|(sheet, _)| *sheet != viewer)
            .map(|(sheet, reason)| format!("{}: {reason}", sheet.tab_label())),
    );
    parts.join(" · ")
}

/// The sheet the active analysis of a finished run opens on, with what it
/// needs, when none of that analysis' own sheets (the table aside, which
/// reads anything) can draw it.
fn unfilled_sheet(state: ResultsReads<'_>) -> Option<(ResultViewer, &'static str)> {
    let simulation = &state.model.simulation;
    // A run still solving has not said what it cannot fill yet.
    simulation.active_run().filter(|run| {
        !matches!(
            run.lifecycle,
            crate::state::SimulationRunLifecycle::Preparing
                | crate::state::SimulationRunLifecycle::Running
                | crate::state::SimulationRunLifecycle::Cancelling
        )
    })?;
    let sheets = ResultViewer::default_sheets(simulation.active_analysis()?.analysis_type);
    let first = *sheets
        .first()
        .filter(|first| **first != ResultViewer::Table)?;
    let drawn = sheets
        .iter()
        .filter(|sheet| **sheet != ResultViewer::Table)
        .any(|sheet| viewer_is_available(state, *sheet));
    (!drawn).then(|| (first, viewer_availability(state, first).reason))
}

#[cfg(test)]
fn viewer_tabs(ui: &mut Ui, state: &mut ResultsTestHost) {
    let available = ResultViewer::all()
        .filter(|&viewer| viewer_availability(state.results_reads(), viewer).available);
    if let Some(viewer) =
        chrome::bars::viewer_tabs(ui, state.results.results.session.viewer, available)
    {
        state.results.results.session.viewer = viewer;
    }
}

/// Which sheets a persistent result document's docbar offers.
///
/// The family owns the answer, next to the `includes` list the Create dialog
/// composes documents from, so the two cannot drift: this restated the mapping
/// once and drifted immediately — the digital family bound its first pane to
/// the waveform renderer and then refused every waveform sheet, and the
/// phase-noise and table sheets were unreachable in families the dialog binds
/// them for.
///
/// Imported or renamed pages resolve to no family and keep every sheet their
/// dataset can feed reachable.
fn family_allows_viewer(family_label: &str, viewer: ResultViewer) -> bool {
    rspice_results::document_creation::ResultDocumentFamily::from_label(family_label)
        .is_none_or(|family| family.offers_sheet(viewer))
}

/// Keep the canvas on a view of the shown analysis: the sheet it is on while
/// the shown analysis offers it (Table stays Table when the reader picks
/// another analysis that has a table), else the shown analysis' opening
/// sheet. Choosing a tab never comes back here to change the analysis.
fn reconcile_active_viewer(state: &mut ResultsParts<'_>) {
    // A run that has written nothing yet opens on the sheet its plot
    // arrives on, where the sheet says the run is starting.
    if let Some(run) = state.model.simulation.active_run()
        && run.analyses.is_empty()
        && !run.lifecycle.is_terminal()
        && let Some(sheet) = starting_sheet(run)
    {
        state.results.session.viewer = sheet;
        return;
    }
    if viewer_availability(state.results_reads(), state.results.session.viewer).available {
        return;
    }
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    // A run that has written nothing yet keeps the reader's sheet: its first results arrive
    // there, and until they do the sheet says what is coming.
    if run.analyses.is_empty()
        && matches!(
            run.lifecycle,
            crate::state::SimulationRunLifecycle::Preparing
                | crate::state::SimulationRunLifecycle::Running
        )
    {
        return;
    }
    if let Some(viewer) = opening_sheet(state.results_reads()) {
        state.results.session.viewer = viewer;
    }
}

/// Whether `viewer` is offered: the run's data can fill it, and it is a view
/// of the analysis the data browser shows. The tabs offer the shown
/// analysis' views and nothing of another: picking an analysis is the
/// browser's alone.
pub fn viewer_availability(state: ResultsReads<'_>, viewer: ResultViewer) -> ViewerAvailability {
    let availability = run_viewer_availability(state, viewer);
    if availability.available
        && let Some(run) = state.model.simulation.active_run()
        && let Some(shown) = shown_analysis(&state.model.simulation)
        && !rspice_results::renderability::viewer_can_render(viewer, shown, |gate| {
            analysis_answers_structural_gate(state, run.dataset_id, shown, gate)
        })
    {
        return ViewerAvailability::unavailable(
            "Not a view of the analysis shown; pick an analysis it draws in the data browser",
        );
    }
    availability
}

/// Whether the active run's data can fill `viewer`, and if not, what it
/// needs.
fn run_viewer_availability(state: ResultsReads<'_>, viewer: ResultViewer) -> ViewerAvailability {
    let active_run = state.model.simulation.active_run();
    // The sheet a starting run's plot arrives on is offered before its first
    // results, so the reader is shown where the run is starting.
    if let Some(run) = active_run
        && run.analyses.is_empty()
        && !run.lifecycle.is_terminal()
        && starting_sheet(run) == Some(viewer)
    {
        return ViewerAvailability::available(
            "The run is starting; its first results are drawn here",
        );
    }
    match viewer {
        ResultViewer::Waves => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty()
                })
            }) {
                ViewerAvailability::available(
                    "Time-domain waveforms are present in the active dataset",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires transient, PSS, envelope, or other time-domain waveform data",
                )
            }
        }
        ResultViewer::DcSweep => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::DcSweep
                        && !analysis.waveforms.is_empty()
                })
            }) {
                ViewerAvailability::available(
                    "A retained swept-source or swept-parameter DC transfer is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires DC sweep waveform data in the active dataset",
                )
            }
        }
        ResultViewer::Bode => {
            let has_frequency_response = active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis_answers_structural_gate(
                        state,
                        run.dataset_id,
                        analysis,
                        StructuralGate::BodeResponse,
                    )
                })
            });
            if has_frequency_response {
                ViewerAvailability::available("A usable frequency response is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable AC, PAC, PXF, STB, or distortion response",
                )
            }
        }
        ResultViewer::Fft => specialized_availability(state, ActiveViewer::Fft),
        ResultViewer::HarmonicBalance => {
            if harmonic_balance::active_analysis_is_renderable(state) {
                ViewerAvailability::available(
                    "A retained complex HB or Fourier coefficient spectrum is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires HB or Fourier results that keep complex coefficients",
                )
            }
        }
        ResultViewer::PhaseNoise => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis_answers_structural_gate(
                        state,
                        run.dataset_id,
                        analysis,
                        StructuralGate::PhaseNoiseSpectrum,
                    )
                })
            }) {
                ViewerAvailability::available(
                    "A retained periodic phase-noise spectrum is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires PNOISE/QPNOISE data explicitly retained as phase noise",
                )
            }
        }
        ResultViewer::Eye => specialized_availability(state, ActiveViewer::EyeDiagram),
        ResultViewer::Hist => specialized_availability(state, ActiveViewer::Histogram),
        ResultViewer::Op => {
            // The analysis kind belongs in the gate because it is already in
            // the sheet: `op_inspector::selected_op_evidence` resolves
            // nothing for an analysis that is not a DC operating point, so a
            // transient that retained its bias solution lit the tab and then
            // showed "The selected analysis is not a DC operating-point
            // result." The bias solution behind a transient is that
            // transient's initial condition, not an operating point measured
            // under an operating-point solve — the sheet's solve-facts card,
            // its detail policy and its export all describe the latter, so
            // the coherent answer is to offer the tab only where the sheet
            // has something to say.
            if state
                .model
                .simulation
                .active_analysis()
                .is_some_and(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::DcOp
                        && (analysis.dc_op.is_some()
                            || analysis
                                .device_op
                                .as_ref()
                                .is_some_and(|report| !report.is_empty())
                            || matches!(
                                analysis.result_payload,
                                Some(crate::state::AnalysisResultPayload::OperatingPoint { .. })
                            ))
                })
            {
                ViewerAvailability::available("Operating-point results are available")
            } else {
                // The gate reads the *active analysis*, and accepts any of
                // three kinds of evidence. Naming only the device report, and
                // naming the dataset as its scope, sent a reader who had a
                // node DC solution under a different analysis looking for a
                // report they did not need and could not have produced.
                ViewerAvailability::unavailable(
                    "Requires the active analysis to be a DC operating-point result carrying a \
                     node DC solution, a device operating-point report, or a retained \
                     operating-point payload",
                )
            }
        }
        ResultViewer::NoiseContrib => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    view_context::analysis_supports_viewer_memoized(
                        state,
                        run.dataset_id,
                        ResultViewer::NoiseContrib,
                        analysis,
                    )
                })
            }) {
                ViewerAvailability::available("A retained noise spectrum is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable noise spectrum in the active dataset",
                )
            }
        }
        ResultViewer::Contribution => {
            if sensitivity::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained contributions are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a valid retained sensitivity or DC mismatch payload",
                )
            }
        }
        ResultViewer::TransferFunction => {
            if transfer_function::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Transfer-function results are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained transfer-function payload",
                )
            }
        }
        ResultViewer::Table => {
            // The table reads whatever the WAVES stage can plot: if a strip
            // exists, its retained samples can be listed.
            if state
                .model
                .simulation
                .active_run()
                .is_some_and(|run| run.analyses.iter().any(|a| !a.waveforms.is_empty()))
            {
                ViewerAvailability::available("Retained samples are available to list")
            } else {
                ViewerAvailability::unavailable("Requires an analysis with retained samples")
            }
        }
        ResultViewer::Nyquist => specialized_availability(state, ActiveViewer::Nyquist),
        ResultViewer::SParameters => s_parameters::availability(state),
        ResultViewer::Smith => smith::availability(state),
        ResultViewer::Polar => {
            if active_run.is_some_and(|run| {
                state
                    .model
                    .simulation
                    .active_analysis()
                    .is_some_and(|analysis| {
                        !polar::quantities(analysis).is_empty()
                            && analysis_evidence_is_valid(state, run.dataset_id, analysis)
                    })
            }) {
                ViewerAvailability::available(
                    "Retained complex responses are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires AC, SP, or HB with retained complex responses",
                )
            }
        }
        ResultViewer::PoleZero => specialized_availability(state, ActiveViewer::PoleZero),
        ResultViewer::Floquet => floquet::availability(state),
        ResultViewer::NetworkMatrix => network_matrix::availability(state),
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            if active_run.is_some_and(|run| {
                state
                    .model
                    .simulation
                    .active_analysis()
                    .is_some_and(|analysis| {
                        rspice_results::population::is_a_population(analysis)
                            && analysis_evidence_is_valid(state, run.dataset_id, analysis)
                    })
            }) {
                ViewerAvailability::available(
                    "A retained Monte Carlo population is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a Monte Carlo analysis with saved per-trial results",
                )
            }
        }
        ResultViewer::Events => {
            if events::active_analysis_is_renderable(state) {
                ViewerAvailability::available(
                    "A committed event history or the run's check messages are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a transient analysis with retained event nodes or check messages",
                )
            }
        }
        ResultViewer::Soa => {
            if soa::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Safe-operating-area results are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained SOA payload",
                )
            }
        }
        ResultViewer::Optimization => {
            if optimization::active_metadata_is_valid(state) {
                ViewerAvailability::available(
                    "A retained optimizer cost history is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to carry a retained optimization history",
                )
            }
        }
        ResultViewer::Sweep => {
            if sweep::shown_is_sweep(state) {
                ViewerAvailability::available(
                    "The shown analysis is a point of a sweep, corner run or temperature step",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a .STEP, corner or temperature sweep; pick one of its points in the data browser",
                )
            }
        }
    }
}

fn specialized_availability(state: ResultsReads<'_>, viewer: ActiveViewer) -> ViewerAvailability {
    let capability = state.viewer_capability(viewer);
    ViewerAvailability {
        available: capability.available,
        reason: capability.reason,
    }
}

/// Gate FFT/eye rendering on the controller's derived-data loader. Returns
/// `true` when the viewer can render.
fn ensure_derived(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_controller: &mut SimulationController,
    viewer: ActiveViewer,
) -> bool {
    let Some(analysis_type) = state
        .model
        .simulation
        .active_analysis()
        .map(|analysis| analysis.analysis_type)
    else {
        well_hint(ui, "Select an analysis with transient data");
        return false;
    };
    if !SimulationController::analysis_supports_transient_derivation(analysis_type) {
        well_hint(ui, "The active analysis has no usable time-domain source");
        return false;
    }
    match run_controller.ensure_transient_viewer_data(state, viewer) {
        DerivedViewerLoadState::Ready => {
            state.results.clear_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
            );
            true
        }
        DerivedViewerLoadState::Loading => {
            let data_version = state.model.simulation.view.data_version;
            state.results.record_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
                "Preparing and validating derived data from the active transient.",
                data_version,
            );
            well_hint(ui, "Preparing derived data from the active transient…");
            ui.ctx().request_repaint();
            false
        }
        DerivedViewerLoadState::Unavailable => {
            state.results.clear_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
            );
            // Derived viewers can be unavailable for reasons the reader can
            // act on. Preserve the FFT builder's typed diagnostic and the
            // eye's timebase guidance at this outer gate: the individual
            // viewer is not rendered when loading reports `Unavailable`.
            let hint = match viewer {
                ActiveViewer::Fft => state
                    .analysis
                    .fft_state
                    .last_error
                    .as_ref()
                    .map(|error| format!("Spectrum unavailable — {error}")),
                ActiveViewer::EyeDiagram => eye::unavailable_hint(state.results_reads()),
                _ => None,
            };
            well_hint(
                ui,
                hint.as_deref()
                    .unwrap_or("The active analysis does not contain a usable source"),
            );
            false
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Render the Results context right panel for the active viewer.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    if let Ok(view) = view_context::resolve_displayed_result_view(state.results_reads())
        && let Some(analysis) = view.primary_analysis(state.results_reads())
        && let Some(crate::state::AnalysisResultPayload::Qpnoise { response }) =
            &analysis.result_payload
    {
        let quantity_policy = state.model.preferences.quantity_presentation_policy();
        let digits = usize::from(
            state
                .results_reads()
                .presentation_policy()
                .displayed_significant_digits()
                .get(),
        );
        let shown = rspice_results_ui::qpnoise::summary(ui, response, &quantity_policy, digits);
        let contributors = view.viewer == ResultViewer::NoiseContrib;
        if let Some(device) = shown {
            noise_contrib::show_on_schematic(state, &device);
        }
        if contributors {
            waves::right_panel(ui, state);
            return;
        }
    }
    match state.results.session.viewer {
        ResultViewer::Waves | ResultViewer::DcSweep | ResultViewer::SParameters => {
            waves::right_panel(ui, state)
        }
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::HarmonicBalance => harmonic_balance::right_panel(ui, state),
        ResultViewer::PhaseNoise => phase_noise::right_panel(ui, state),
        ResultViewer::Eye => eye::right_panel(ui, state),
        ResultViewer::Hist => hist::right_panel(ui, state),
        ResultViewer::Op => op_inspector::right_panel(ui, state),
        ResultViewer::NoiseContrib => noise_contrib::right_panel(ui, state),
        ResultViewer::Contribution => sensitivity::right_panel(ui, state),
        ResultViewer::Nyquist => nyquist::right_panel(ui, state),
        ResultViewer::Smith => smith::right_panel(ui, state),
        ResultViewer::Polar => polar::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::PoleZero => pz::right_panel(ui, state),
        ResultViewer::Floquet => floquet::right_panel(ui, state),
        ResultViewer::TransferFunction => transfer_function::right_panel(ui, state),
        // The table and network-matrix panels held nothing but what their
        // own sheet already states.
        ResultViewer::Table | ResultViewer::NetworkMatrix => {}
        ResultViewer::Scatter => scatter::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::BoxViolin => box_violin::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::Events => events::right_panel(ui, state),
        ResultViewer::Soa => soa::right_panel(ui, state),
        ResultViewer::Optimization => optimization::right_panel(ui, state),
        // The points' table on the sheet states every value the panel could.
        ResultViewer::Sweep => {}
    }
}

#[cfg(test)]
mod availability_tests;
