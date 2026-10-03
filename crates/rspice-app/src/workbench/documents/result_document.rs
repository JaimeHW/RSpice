//! The Results workspace.
//!
//! A 41 px dataset-viewer tab strip, a distinct 31 px plot instrument or
//! purpose strip, a fill-height document well carrying the active viewer
//! (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics), and a content-fit readout strip.

mod bars;
mod bode;
mod box_violin;
mod create_document;
mod events;
mod eye;
mod fft;
pub(crate) mod frame_work;
mod harmonic_balance;
mod hist;
mod instrument;
pub(crate) mod manifest;
pub(crate) mod network_matrix;
mod noise_contrib;
mod nyquist;
mod op_inspector;
pub(crate) mod operational_state;
mod optimization;
mod persistent_document;
mod phase_noise;
mod polar;
mod population;
mod pz;
mod qpnoise;
mod retained_memo;
mod scatter;
mod sensitivity;
mod smith;
mod soa;
mod specs;
mod table;
mod transfer_function;
pub(crate) mod view_context;
mod view_plans;
mod viewer_state;

/// Lossless CSV projection of the exact evidence rendered by a Results sheet.
pub(crate) struct ResultSheetCsv {
    pub(crate) default_name: &'static str,
    pub(crate) contents: String,
    pub(crate) detail: String,
}

// The application retains the Events source-order cache; viewer selections
// belong to the lower-owned session.
pub(crate) use events::EventOrderCache;
pub(crate) use manifest::export_csv as export_manifest_csv;
pub(crate) use noise_contrib::export_csv as export_noise_contribution_csv;
pub(crate) use op_inspector::export_csv as export_operating_point_csv;
pub(crate) use optimization::export_csv as export_optimization_csv;
pub(crate) use specs::active_run_specifications as run_specifications;
pub(crate) use specs::export_csv as export_specs_csv;

/// Route to the one surface that authors specification limits.
///
/// The whole route, not just the document: the editor is a Results viewer, so
/// opening it without selecting that viewer and activating Results arms a
/// surface nobody is looking at. Three call sites had written those three
/// steps out identically, which is three places for the route to drift from
/// the one editor it is supposed to protect.
pub(crate) fn open_specification_editor(state: &mut AppState) {
    // Route transitions and persistent-document activation are reconciled at
    // frame boundaries. Carry a one-shot intent separately from `viewer` and
    // the draft rows so those projections cannot erase the requested editor
    // before the Results destination gets its first frame.
    state.workbench.specification_editor_route_pending = true;
    state.ui.results.session.viewer = crate::workbench::ResultViewer::Specs;
    specs::open_editor(state);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
}

/// Consume a specification-authoring route at the Results destination.
///
/// This deliberately reconstructs both projections after document/viewer
/// reconciliation. Returning `true` lets the surface give the editor
/// precedence over an active persistent result document in the same frame.
pub(crate) fn consume_pending_specification_editor(state: &mut AppState) -> bool {
    if !std::mem::take(&mut state.workbench.specification_editor_route_pending) {
        return false;
    }
    state.ui.results.session.viewer = crate::workbench::ResultViewer::Specs;
    specs::open_editor(state);
    true
}

pub(crate) fn harmonic_balance_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    harmonic_balance::analysis_is_renderable(analysis)
}

pub(crate) fn harmonic_balance_waveform_is_renderable(waveform: &WaveformData) -> bool {
    harmonic_balance::spectrum_trace_is_renderable(waveform)
}

pub(crate) fn phase_noise_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    phase_noise::phase_noise_is_renderable(analysis)
}

pub(crate) fn smith_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    smith::analysis_is_renderable(analysis)
}

/// Whether one analysis carries an ordinary-noise spectrum a reader may see.
///
/// Exposed to the crate because the printed page has to make the same
/// offering the sheet does — it used to keep its own copy of this predicate
/// and of the binding below, with a comment saying they mirrored these.
pub(crate) fn ordinary_noise_spectrum_is_renderable(analysis: &AnalysisResult) -> bool {
    bode::ordinary_noise_spectrum_is_renderable(analysis)
}

pub(crate) fn qpnoise_spectrum_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && qpnoise::is_renderable(analysis)
        && analysis.validate_retained_evidence().is_ok()
}

/// The one noise analysis a run's noise surfaces bind to.
pub(crate) fn selected_noise_analysis_index(
    globally_selected: Option<usize>,
    run: &SimulationRun,
) -> Option<usize> {
    if let Some(index) = globally_selected
        && let Some(analysis) = run.analyses.get(index)
        && analysis.analysis_type == crate::state::AnalysisType::Qpnoise
    {
        return qpnoise_spectrum_is_renderable(analysis).then_some(index);
    }
    bode::selected_noise_analysis_index_in(globally_selected, run).or_else(|| {
        // A failed explicitly selected noise analysis must remain selected.
        if globally_selected
            .and_then(|i| run.analyses.get(i))
            .is_some_and(|a| {
                matches!(
                    a.analysis_type,
                    crate::state::AnalysisType::Noise
                        | crate::state::AnalysisType::Hbnoise
                        | crate::state::AnalysisType::Pnoise
                )
            })
        {
            None
        } else {
            run.analyses.iter().position(qpnoise_spectrum_is_renderable)
        }
    })
}

pub(crate) fn phase_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    phase_noise::phase_noise_waveform_is_renderable(waveform)
}

/// Whether the tab strip offers a frequency-response sheet for one analysis.
///
/// The success gate belongs to both branches. A Bode summary resolves off the
/// retained magnitude and phase vectors alone, so a failed AC solve still
/// produces one — and the sheet behind this offering refuses a failed solve
/// outright, because margins read off vectors the engine emitted before
/// giving up are not measurements. Only the raw-curve branch was checking.
///
/// The question is one of shape — are the traces a Bode sheet needs present?
/// — so it is asked of [`crate::state::ac_bode_shape_for_analysis`] and not
/// of the summary. The summary answers the same shape question by measuring
/// the response first: a decibel conversion of the whole magnitude vector,
/// an unwrapped copy of the phase, and every crossing search over both. The
/// tab strip asks this about every analysis in the run, on every frame,
/// whichever sheet is open.
/// Only the magnitude/phase branch is a shape question. The distortion
/// family retains plain frequency curves with no `|…|`/`phase(…)` pairing to
/// resolve, so its branch has to prove the curve itself is well formed —
/// which is a walk of every retained sample, and the reason this whole
/// predicate is asked through [`analysis_answers_structural_gate`] rather
/// than directly.
pub(crate) fn bode_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && (crate::state::ac_bode_shape_for_analysis(analysis, 0).is_some()
            || (analysis.analysis_type.is_raw_frequency_curve()
                && analysis
                    .waveforms
                    .iter()
                    .any(raw_frequency_curve_is_renderable)))
}

/// Whether one retained distortion curve is drawable: paired coordinates, a
/// positive finite abscissa at every sample, and a strictly ascending sweep.
fn raw_frequency_curve_is_renderable(waveform: &WaveformData) -> bool {
    if !waveform.visible || waveform.x.len() != waveform.y.len() || waveform.x.len() < 2 {
        return false;
    }
    frame_work::note(frame_work::DatasetWalk::RawFrequencyCurveScan);
    waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .all(|(&x, &y)| x.is_finite() && x > 0.0 && y.is_finite())
        && waveform.x.windows(2).all(|pair| pair[0] < pair[1])
}

/// Open the dataset/manifest browser in its canonical Results frame.
///
/// The navigator and inspector are parts of that frame; this command exposes
/// them instead of creating a second dataset browser with independent state.
pub(crate) fn open_dataset_browser(app: &mut RSpiceApp) {
    app.state.workbench.activate(Workspace::Results);
    app.state.workbench.navigator_visible = true;
    app.state.workbench.inspector_visible = true;
    app.state.workbench.focus_navigator_search = true;
    app.state.ui.results.session.viewer = ResultViewer::Manifest;
}

/// Open a fresh immutable-dataset-bound result-document transaction.
pub(crate) fn open_create_document(app: &mut RSpiceApp) {
    create_document::open(app);
}

pub(crate) fn visualization_source_dataset(
    run: &crate::state::SimulationRun,
    analysis: &crate::state::AnalysisResult,
) -> Result<crate::results::visualization_document::SourceDataset, String> {
    create_document::source_dataset(run, analysis).map_err(|error| error.to_string())
}

mod waves;
pub(crate) use waves::copy_cursor_text;
pub(crate) use waves::{
    ActivePaneFacts, SharedXStatus, active_pane_facts, active_shared_x_status,
    analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};

pub(crate) use waves::toggle_visibility;

use bars::show as show_sheet_bar;
use retained_memo::RetainedMemo;
use std::collections::{HashMap, HashSet};

use egui::{Ui, WidgetInfo, WidgetType};

#[cfg(test)]
pub(crate) use crate::state::result_presentation::TracePresentationKey;
pub(crate) use crate::state::result_presentation::{
    AnalysisPresentationKey, WaveformPresentationKey,
};
pub use crate::state::result_presentation::{
    ExprTrace, MarkerKind, ResultMarker, WavePanePresentationKey,
};
use crate::state::result_presentation::{ResultExpressionGroup, ResultPresentation};

use crate::product::{AnalysisInstanceId, DatasetId};

use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{AnalysisResult, SimulationRun, WaveformData};

use crate::ui::tokens::Tokens;
use crate::workbench::app_state::ActiveViewer;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};
use crate::workbench::{AppState, RSpiceApp};
use rspice_results::family_projection::SourceSampleSelection;
use rspice_results_ui::chrome;
use rspice_results_ui::chrome::bars::viewer_has_sheet_bar;
#[cfg(test)]
use rspice_results_ui::chrome::bars::viewer_has_structured_strip;
use rspice_results_ui::chrome::instrument::ResultPlotTool;
use rspice_results_ui::presentation::{PlotView, well_hint};
use rspice_results_ui::selection::{
    ResultArtifactPresentationKey, ResultBrowserSelectionKey, ResultExpressionPresentationKey,
    SelectedResultTrace, SourceWaveformPresentationKey,
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
pub(crate) struct ExpressionWaveform {
    pub(crate) source: ExpressionSource,
    pub(crate) waveform: crate::state::WaveformData,
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
    let (run_index, _, analysis) = key.resolve(runs).ok_or_else(|| {
        "The selected typed result no longer resolves in its immutable dataset.".to_owned()
    })?;
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
) -> Result<(&'a SimulationRun, &'a AnalysisResult, &'a WaveformData), String> {
    let (run_index, analysis_index, _, waveform) = key.resolve(runs).ok_or_else(|| {
        "The selected quantity no longer resolves uniquely in its immutable dataset.".to_owned()
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
    let analysis_id = analysis.provenance().map_or_else(
        || format!("legacy-{}", analysis.id),
        |provenance| provenance.source_instance_id().to_string(),
    );
    Ok(format!(
        "dataset/{}/analysis/{analysis_id}/quantity/{}",
        run.dataset_id, waveform.name
    ))
}

pub(crate) fn exact_result_signal_last_sample(
    key: &SourceWaveformPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (_, analysis, waveform) = resolved_result_signal(key, runs)?;
    validate_exact_analysis_evidence(analysis)?;
    rspice_formats::result_csv::encode_exact_sample(waveform.as_ref())
}

pub(crate) fn exact_result_signal_tsv(
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
    analysis.validate_retained_evidence().map_err(|error| {
        format!(
            "The selected analysis failed retained-evidence verification; exact numeric access is quarantined: {error}"
        )
    })
}

pub(crate) fn validate_result_browser_selection_evidence(
    key: &ResultBrowserSelectionKey,
    runs: &[SimulationRun],
) -> Result<(), String> {
    let analysis = match key {
        ResultBrowserSelectionKey::Waveform(key) => resolved_result_signal(key, runs)?.1,
        ResultBrowserSelectionKey::Artifact(key) => {
            let (_, _, analysis) = key.resolve(runs).ok_or_else(|| {
                "The selected typed result no longer resolves in its immutable dataset.".to_owned()
            })?;
            analysis
        }
    };
    validate_exact_analysis_evidence(analysis)
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
                "The selected quantity no longer resolves uniquely in its immutable dataset."
                    .to_owned()
            }),
        ResultBrowserSelectionKey::Artifact(key) => Ok(key.canonical_name().to_owned()),
    }
}

/// Deterministic exact bundle used by batch clipboard and export actions.
pub(crate) fn exact_result_browser_selection_bundle(
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

/// Lossless source-evidence projection for one typed Data Browser artifact.
///
/// This is the single adapter used by clipboard, exact-table, and file-export
/// workflows. Keeping those actions on one projection prevents a viewer from
/// displaying one retained entity while exporting another analysis payload.
pub(crate) fn exact_result_artifact_text(
    key: &ResultArtifactPresentationKey,
    runs: &[SimulationRun],
) -> Result<String, String> {
    let (run_index, _, analysis) = key.resolve(runs).ok_or_else(|| {
        "The selected typed result no longer resolves in its immutable dataset.".to_owned()
    })?;
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
pub(crate) fn request_view_gesture(state: &mut AppState, gesture: ViewGesture) {
    state.ui.results.session.pending_view_gesture = Some(gesture);
}

/// Whether the active sheet has a viewport that fitting can release.
///
/// Every plot sheet does; the structured documents (OP, specs, table, XF,
/// manifest) have no viewport at all and must not offer the gesture.
pub(crate) fn fit_gesture_available(state: &AppState) -> bool {
    state.simulation.has_results() && viewer_draws_a_pane(state.ui.results.session.viewer)
}

/// Whether this sheet draws a plot pane at all.
///
/// The evidence tables (OP, specs, sample table, event history, manifest) and
/// the scalar transfer-function readout draw no axes, so they have no
/// viewport, no fit, and no limit mask to report.
pub(crate) const fn viewer_draws_a_pane(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::Op
            | ResultViewer::Specs
            | ResultViewer::Table
            | ResultViewer::Events
            | ResultViewer::TransferFunction
            | ResultViewer::Manifest
    )
}

/// Whether the active sheet can be magnified about its centre.
///
/// Only the unit-pane stack exposes the retained extents a zoom step has to
/// be computed against; the single-canvas viewers own their own gestures.
pub(crate) fn zoom_gesture_available(state: &AppState) -> bool {
    state.simulation.has_results() && viewer_uses_wave_stack(state.ui.results.session.viewer)
}

/// Apply any queued viewport gesture, now that the sheet's models and theme
/// tokens exist.
pub(crate) fn apply_pending_view_gesture(ui: &Ui, state: &mut AppState) {
    let Some(gesture) = state.ui.results.session.pending_view_gesture.take() else {
        return;
    };
    let viewer = state.ui.results.session.viewer;
    if !viewer_uses_wave_stack(viewer) {
        // Single-canvas viewers keep their views under plot ordinals, and
        // only fit is expressible without their renderer's own extents.
        match gesture {
            ViewGesture::Fit => state.ui.results.session.reset_viewer_plot_views(viewer),
            ViewGesture::SetRanges { x, y } => {
                let view = state.ui.results.session.plot_view_pane_mut_for(
                    viewer,
                    PlotPresentationKey::Global(0),
                    0,
                );
                if let Some(x) = x {
                    view.x = Some(x);
                }
                if let Some(y) = y {
                    view.y = Some(y);
                }
            }
            ViewGesture::ZoomIn | ViewGesture::ZoomOut => {}
        }
        return;
    }
    match gesture {
        ViewGesture::Fit => waves::fit_active_strip(state),
        ViewGesture::ZoomIn => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 0.5),
        ViewGesture::ZoomOut => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 2.0),
        ViewGesture::SetRanges { x, y } => {
            let tokens = Tokens::get(ui.ctx());
            if let Some(x) = x {
                waves::set_active_pane_axis_range(&tokens, state, PaneAxis::X, Some(x));
            }
            if let Some(y) = y {
                waves::set_active_pane_axis_range(&tokens, state, PaneAxis::Y, Some(y));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ViewerAvailability {
    available: bool,
    reason: &'static str,
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
}

impl<'a> SheetContext<'a> {
    fn of(state: &'a mut AppState) -> Self {
        Self {
            policy: state.ui.preferences.quantity_presentation_policy(),
            simulation: &state.simulation,
            workspace: &state.workspace,
            results: &mut state.ui.results,
        }
    }
}

/// How a retained plot-marker kind reads as a Results marker kind.
pub(crate) const fn marker_kind_of_retained(
    kind: crate::results::visualization_document::PlotMarkerKind,
) -> MarkerKind {
    use crate::results::visualization_document::PlotMarkerKind;
    match kind {
        PlotMarkerKind::PointNote | PlotMarkerKind::MeasurementAnchor => MarkerKind::Note,
        PlotMarkerKind::Peak => MarkerKind::Peak,
        PlotMarkerKind::SpecificationLine => MarkerKind::Spec,
    }
}

/// How a Results marker kind is retained in a visualization document.
pub(crate) const fn retained_kind_of_marker(
    kind: MarkerKind,
) -> crate::results::visualization_document::PlotMarkerKind {
    use crate::results::visualization_document::PlotMarkerKind;
    match kind {
        MarkerKind::Note => PlotMarkerKind::PointNote,
        MarkerKind::Peak => PlotMarkerKind::Peak,
        MarkerKind::Spec => PlotMarkerKind::SpecificationLine,
    }
}

/// One requested marker placement, as the drawn pane knows it.
pub(super) struct MarkerPlacement<'a> {
    pub analysis: AnalysisPresentationKey,
    pub anchor: WaveformPresentationKey,
    pub trace_name: String,
    /// Where the reader asked for it, in the strip's X data space.
    pub x: f64,
    /// The anchoring trace's X samples, as drawn.
    ///
    /// A retained document marker names an exact retained sample — the
    /// document declines to interpolate one into existence — so a document
    /// placement is snapped onto the nearest of these. A quick marker keeps
    /// the requested position, which is what the quick view has always meant.
    pub samples: &'a [f64],
}

impl MarkerPlacement<'_> {
    /// The nearest retained sample to the requested position.
    fn retained_x(&self) -> f64 {
        self.samples
            .iter()
            .copied()
            .filter(|sample| sample.is_finite())
            .min_by(|left, right| (left - self.x).abs().total_cmp(&(right - self.x).abs()))
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

/// Place a marker on whichever store owns the pane being drawn.
///
/// On a persistent pane the retained document is the owner, so the marker is
/// committed to it immediately — one interaction, one transaction, no frame
/// diff to reconcile. A document that cannot hold the marker — it does not
/// retain the clicked trace (an expression trace, or one family member of a
/// swept trace), or it declines the placement outright — falls back to a quick
/// marker and says why, rather than dropping the reader's click.
pub(super) fn place_marker(
    state: &mut AppState,
    placement: MarkerPlacement<'_>,
) -> Option<MarkerSelector> {
    if let Err(error) =
        ResultMarker::validate_placement(placement.analysis, &placement.anchor, placement.x)
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Could not place marker: {error}"
        )));
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
    let retained = state
        .ui
        .results
        .session
        .persistent_pane_context
        .and_then(|context| {
            let document = state
                .workspace
                .content
                .visualization_document(context.document_id)?;
            let trace = document
                .traces()
                .iter()
                .find(|trace| trace.pane_id == context.pane_id && trace.label == trace_name)?;
            Some((context, document.revision(), trace.id))
        });
    let quick_fallback = |state: &mut AppState, reason: Option<String>| match state
        .ui
        .results
        .session
        .add_marker(analysis, anchor.clone(), trace_name.clone(), x)
    {
        Ok(id) => {
            if let Some(reason) = reason {
                state.push_user_message(crate::diagnostics::ConsoleMessage::info(reason));
            }
            Some(MarkerSelector::Quick(id))
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Could not place marker: {error}"
            )));
            None
        }
    };

    let Some((context, revision, trace_id)) = retained else {
        let reason = state.ui.results.session.persistent_pane_context.is_some().then(|| {
            format!(
                "'{trace_name}' is not a retained trace of this result document, so the marker was placed on the dataset instead of the document."
            )
        });
        return quick_fallback(state, reason);
    };

    let receipt = match state.workspace.content.transact_visualization_document(
        context.document_id,
        revision,
        vec![
            crate::results::visualization_document::DocumentEdit::AddTypedMarker {
                pane_id: context.pane_id,
                trace_id,
                coordinate: crate::results::visualization_document::TypedValue::Real(retained_x),
                label: trace_name.clone(),
                kind: retained_kind_of_marker(MarkerKind::default()),
                scope: crate::results::visualization_document::PlotMarkerScope::Pane,
                source_specification: None,
            },
        ],
    ) {
        Ok(receipt) => receipt,
        Err(error) => {
            return quick_fallback(
                state,
                Some(format!(
                    "This result document declined to retain the marker ({error}), so it was placed on the dataset instead."
                )),
            );
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
    state
        .ui
        .results
        .session
        .document_markers
        .push(DocumentMarker {
            document_id: context.document_id,
            pane_id: context.pane_id,
            retained_id: marker_id,
            analysis,
            anchor,
            trace_name: trace_name.clone(),
            x: retained_x,
            kind: MarkerKind::default(),
            note: trace_name,
        });
    Some(MarkerSelector::Document {
        document_id: context.document_id,
        pane_id: context.pane_id,
        marker_id,
    })
}

/// Remove one marker from whichever store owns it.
pub(super) fn remove_marker(state: &mut AppState, selector: MarkerSelector) {
    match selector {
        MarkerSelector::Quick(id) => state.ui.results.session.remove_marker(id),
        MarkerSelector::Document {
            document_id,
            marker_id,
            ..
        } => {
            let Some(revision) = state
                .workspace
                .content
                .visualization_document(document_id)
                .map(|document| document.revision())
            else {
                return;
            };
            if let Err(error) = state.workspace.content.transact_visualization_document(
                document_id,
                revision,
                vec![
                    crate::results::visualization_document::DocumentEdit::Remove(
                        crate::results::visualization_document::EntityRef::Marker(marker_id),
                    ),
                ],
            ) {
                state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                    "Could not remove the retained marker: {error}"
                )));
                return;
            }
            state
                .ui
                .results
                .session
                .document_markers
                .retain(|marker| marker.retained_id != marker_id);
            if state
                .ui
                .results
                .session
                .marker_edit
                .as_ref()
                .is_some_and(|draft| draft.selector == selector)
            {
                state.ui.results.session.marker_edit = None;
            }
        }
    }
}

/// Commit one marker-purpose edit to whichever store owns the marker.
///
/// The retained coordinate, scope and source specification are read back from
/// the live document rather than from the projection, so an edit of the label
/// or kind can never restate a stale position as if the reader had moved it.
pub(super) fn commit_marker_edit(
    state: &mut AppState,
    selector: MarkerSelector,
    note: &str,
    kind: MarkerKind,
) -> Result<(), String> {
    match selector {
        MarkerSelector::Quick(id) => {
            let marker =
                state.ui.results.session.marker_mut(id).ok_or_else(|| {
                    "The quick marker is no longer part of this project.".to_owned()
                })?;
            marker.note = note.to_owned();
            marker.kind = kind;
            Ok(())
        }
        MarkerSelector::Document {
            document_id,
            marker_id,
            ..
        } => {
            let trace_name = state
                .ui
                .results
                .session
                .document_marker(marker_id)
                .map(|marker| marker.trace_name.clone())
                .unwrap_or_default();
            let label = retained_marker_label(note, &trace_name).to_owned();
            let retained = state
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
                return Err(
                    "The retained marker is no longer part of this result document.".to_owned(),
                );
            };
            state
                .workspace
                .content
                .transact_visualization_document(
                    document_id,
                    revision,
                    vec![
                        crate::results::visualization_document::DocumentEdit::SetMarker {
                            marker_id,
                            coordinate,
                            label: label.clone(),
                            kind: retained_kind_of_marker(kind),
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
                .ui
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
    }
}

/// A marker anchor naming one retained signal's plain value projection.
///
/// Production anchors come from the drawn trace, which carries the family
/// discriminator this cannot know; this exists so persistence tests can name
/// a signal without standing up a whole strip.
#[cfg(test)]
pub(crate) fn marker_anchor_for(
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

/// Restore authored presentation only after its datasets and visualization
/// documents have been loaded. Exhaustive ownership covers every durable field.
pub(crate) fn restore_presentation(state: &mut AppState, presentation: ResultPresentation) {
    let ResultPresentation {
        markers,
        marker_id_high_water,
        log_y_panes,
        expression_groups,
    } = presentation;
    restore_markers(state, markers, marker_id_high_water);
    restore_log_y_panes(state, log_y_panes);
    restore_expression_groups(state, expression_groups);
}

/// Restore the markers a project retained, dropping any whose analysis is not
/// in the reopened datasets, and any that is only a saved projection of a
/// marker the workspace's own visualization documents already own.
///
/// Projects saved before markers had one owner captured every retained
/// document marker into the quick-view list as well. Reopening such a project
/// would draw both copies and offer two rows for one annotation, so an
/// incoming quick marker that matches a retained one in content — same trace,
/// position, label and kind — is dropped in favour of the document's. The
/// match is content-shaped, not id-shaped: the two stores never shared an id
/// space that could be compared.
pub(crate) fn restore_markers(
    state: &mut AppState,
    markers: Vec<ResultMarker>,
    high_water: Option<u32>,
) {
    let legacy = high_water.is_none();
    let mut highest = high_water.unwrap_or(0);
    let retained: Vec<ResultMarker> = markers
        .into_iter()
        .filter(|marker| {
            // Current writers explicitly own quick markers. A genuine quick
            // annotation can match a document marker without being its old
            // projection; migrate this ambiguity only in legacy files.
            if legacy && is_document_marker_projection(state, marker) {
                return false;
            }
            // A discarded dataset does not make its allocated labels reusable.
            highest = highest.max(marker.id);
            state
                .simulation
                .runs
                .iter()
                .any(|run| marker.analysis.resolve(run).is_some())
        })
        .collect();
    state.ui.results.session.adopt_markers(retained, highest);
}

/// Whether one loaded quick marker restates a marker a project-owned
/// visualization document already retains.
fn is_document_marker_projection(state: &AppState, marker: &ResultMarker) -> bool {
    use crate::results::visualization_document::TypedValue;
    if marker.anchor.analysis != marker.analysis {
        return false;
    }
    state
        .workspace
        .content
        .visualization_documents
        .iter()
        .any(|document| {
            document.markers().iter().any(|retained| {
                let Some(trace) = document
                    .traces()
                    .iter()
                    .find(|trace| trace.id == retained.trace_id)
                else {
                    return false;
                };
                let Some(binding) = document
                    .panes()
                    .iter()
                    .find(|pane| pane.id == trace.pane_id)
                    .and_then(|pane| pane.binding)
                else {
                    return false;
                };
                // Equal display text is not evidence of equal ownership.
                // Preserve annotations on another dataset, analysis, or signal.
                binding.dataset == trace.binding
                    && binding.dataset.dataset_id == marker.analysis.dataset_id()
                    && binding.analysis_id == marker.analysis.retained_instance_id()
                    && trace.source_signal() == Some(marker.anchor.trace.source_name.as_str())
                    && trace.label == marker.trace_name
                    && retained.coordinate == TypedValue::Real(marker.x)
                    && retained.label == marker.note
                    && marker_kind_of_retained(retained.kind) == marker.kind
            })
        })
}

/// Restore the reader's logarithmic-axis pane choices, dropping any whose
/// analysis this project no longer retains.
///
/// Same rule as the markers above: a presentation decision that cannot find
/// the dataset it was made about is not a decision about anything.
pub(crate) fn restore_log_y_panes(state: &mut AppState, panes: Vec<WavePanePresentationKey>) {
    state.ui.results.session.log_y_panes = panes
        .into_iter()
        .filter(|pane| {
            state
                .simulation
                .runs
                .iter()
                .any(|run| pane.analysis.resolve(run).is_some())
        })
        .collect();
}

/// Restore project-owned expression definitions after their immutable result
/// datasets have been loaded. Stale groups fail closed instead of attaching
/// their text to whichever analysis happens to occupy an old ordinal.
pub(crate) fn restore_expression_groups(state: &mut AppState, groups: Vec<ResultExpressionGroup>) {
    state.ui.results.session.analysis_exprs.clear();
    state.ui.results.session.exprs.clear();
    state.ui.results.session.expr_projection_keys.clear();
    state.ui.results.analysis_expr_cache.clear();
    for group in groups {
        let retained = state
            .simulation
            .runs
            .iter()
            .any(|run| group.analysis.resolve(run).is_some());
        if retained && !group.traces.is_empty() {
            state
                .ui
                .results
                .session
                .analysis_exprs
                .insert(group.analysis, group.traces);
        }
    }
    state
        .ui
        .results
        .reconcile_expression_projection(&state.simulation);
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
    state: &AppState,
    analysis: AnalysisPresentationKey,
) -> bool {
    state
        .ui
        .results
        .retained_evidence_validity
        .get_or_insert_with(&state.simulation, analysis, || {
            state
                .simulation
                .runs
                .iter()
                .find_map(|run| analysis.resolve(run))
                .is_some_and(|(_, resolved)| {
                    frame_work::note(frame_work::DatasetWalk::EvidenceValidation);
                    resolved.validate_retained_evidence().is_ok()
                })
        })
}

/// A structural question a tab strip asks about a retained analysis whose
/// answer costs a walk of every retained sample.
///
/// Every one of these is a *shape* question — can this sheet draw this
/// evidence? — that the retained vectors alone can answer, and every one of
/// them was asked directly by the availability gates, about every analysis in
/// the run, on every frame. The magnitude/phase half of the Bode question is
/// genuinely cheap and is answered without coming here; the rest are not, so
/// they come here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum StructuralGate {
    /// A frequency response or a raw distortion curve the Bode sheet can
    /// draw; see [`bode_analysis_is_renderable`].
    BodeResponse,
    /// An ordinary-noise spectrum; see
    /// [`bode::ordinary_noise_spectrum_is_renderable`].
    OrdinaryNoiseSpectrum,
    /// A discrete complex coefficient spectrum; see
    /// [`harmonic_balance::analysis_is_renderable`].
    HarmonicSpectrum,
    /// An explicitly-labelled phase-noise trace with a retained carrier; see
    /// [`phase_noise::phase_noise_is_renderable`].
    PhaseNoiseSpectrum,
    /// S-parameter traces with per-port reference impedances; see
    /// [`smith::structure_is_renderable`].
    SParameterStructure,
    NetworkMatrix,
    /// Exact event histories or validated legacy event projections.
    EventHistory,
}

/// One structural gate answered by walking the evidence, for callers that
/// hold a run without a session to memoize against — the printed page, the
/// visualization document's own compatibility check, and the tests.
pub(crate) fn structural_gate_is_answered_directly(
    gate: StructuralGate,
    analysis: &AnalysisResult,
) -> bool {
    match gate {
        StructuralGate::BodeResponse => bode_analysis_is_renderable(analysis),
        StructuralGate::OrdinaryNoiseSpectrum => {
            bode::ordinary_noise_spectrum_is_renderable(analysis)
        }
        StructuralGate::HarmonicSpectrum => harmonic_balance::analysis_is_renderable(analysis),
        StructuralGate::PhaseNoiseSpectrum => phase_noise::phase_noise_is_renderable(analysis),
        StructuralGate::SParameterStructure => smith::structure_is_renderable(analysis),
        StructuralGate::NetworkMatrix => network_matrix::structure_is_renderable(analysis),
        StructuralGate::EventHistory => events::analysis_is_renderable(analysis),
    }
}

/// Whether one retained analysis answers a structural gate, resolved once per
/// dataset generation.
///
/// Like [`retained_evidence_is_valid`], this reader checks the retained source
/// revision itself. Tab availability cannot reuse an older structural answer
/// while waiting for a painted frame to invalidate the cache.
pub(crate) fn analysis_answers_structural_gate(
    state: &AppState,
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
        .ui
        .results
        .structural_gates
        .get_or_insert_with(&state.simulation, key, || {
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
    state: &AppState,
    run: &SimulationRun,
) -> crate::product::ContentDigest {
    state
        .ui
        .results
        .dataset_digests
        .get_or_insert_with(&state.simulation, run.dataset_id, || {
            frame_work::note(frame_work::DatasetWalk::DatasetDigest);
            run.dataset_content_digest()
        })
}

/// The two halves of a pinned view: the abscissa window, then the ordinate
/// window. Either may be absent — the reader can pin one axis and leave the
/// other to the page — which is why the pair is not one `Option`.
pub(crate) type CapturedViewport = (Option<(f64, f64)>, Option<(f64, f64)>);

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
    state: &AppState,
    analysis: AnalysisPresentationKey,
) -> Option<CapturedViewport> {
    let viewer = state.ui.results.session.viewer;
    let analysis_pane = state
        .ui
        .results
        .session
        .analysis_plot_view_pane(viewer, analysis, 0);
    let view = if analysis_pane.is_zoomed() {
        analysis_pane
    } else {
        state.ui.results.session.plot_view(viewer, 0)
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
/// Crate-visible because the memo is the owner of that verdict for the whole
/// shell rather than a Results-local convenience. The Verify workspace's
/// evidence gates ask the same question of the same immutable datasets, and a
/// second copy of the walk there would be a second answer to keep in step as
/// well as a second million-sample scan.
pub(crate) fn analysis_evidence_is_valid(
    state: &AppState,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> bool {
    retained_evidence_is_valid(state, AnalysisPresentationKey::new(dataset_id, analysis))
}

/// Current retained distributions, including freshly restored runs.
pub(crate) fn histogram_is_available(state: &AppState) -> bool {
    hist::histogram_is_available(state)
}

/// Shared bins for the sheet and exact CSV export.
pub(crate) fn active_histogram(
    state: &AppState,
) -> Option<std::sync::Arc<rspice_results::histogram::Histogram>> {
    hist::active_histogram(state)
}

pub(crate) fn active_histogram_display(
    state: &AppState,
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
    state: &AppState,
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
            .unwrap_or_else(|| "retained evidence failed validation".to_owned()),
    )
}

/// Application composition of viewer state and source-bound caches.
#[derive(Debug, Clone, Default)]
pub struct ResultsState {
    pub session: rspice_results_ui::session::ResultViewerState,
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
    network_matrix: network_matrix::NetworkMatrixState,
    /// Exact retained history and display version behind waveform caches.
    wave_cache_source: Option<(crate::state::RunHistoryRevision, u64)>,
    /// Fingerprint-keyed strip-model cache for the waves viewer.
    models: rspice_results_ui::waves::cache::ModelsCache,
    /// Cached Bode margins + extremes for the active data version.
    pub bode: Option<BodeDerived>,
    /// Cached Nyquist stability numbers for the active data version.
    pub nyquist: Option<nyquist::NyquistDerived>,
    /// `simulation.data_version` last seen by the workspace; when it
    /// advances, cursors are cleared so they never report stale data.
    seen_version: u64,
    /// Evaluated expression series, keyed by (stable analysis, expression);
    /// refreshed when the simulation data version advances.
    pub(crate) analysis_expr_cache:
        std::collections::HashMap<(AnalysisPresentationKey, String), ExprSeries>,
    /// The run whose attributed failure sites are currently marked on the
    /// drawing, if any.
    ///
    /// Held so the control that marked them can offer to take them back, and
    /// keyed by run so selecting a different dataset does not leave a stale
    /// "Clear" offering to unmark objects another run named. Transient.
    pub(crate) marked_failure_run: Option<u64>,
    /// Merged event order for the analysis the EVENTS sheet last drew.
    pub(super) event_order_cache: Option<EventOrderCache>,
    /// Source-aware retained-evidence verdict per analysis; see
    /// [`retained_evidence_is_valid`].
    retained_evidence_validity: RetainedMemo<AnalysisPresentationKey, bool>,
    /// Source-aware content digest per dataset; see
    /// [`retained_dataset_digest`].
    dataset_digests: RetainedMemo<DatasetId, crate::product::ContentDigest>,
    /// Source-aware ordinary-noise structure per analysis;
    /// see [`bode::noise_spectrum_shape`].
    noise_spectrum_shapes: RetainedMemo<AnalysisPresentationKey, Option<bode::NoiseSpectrumShape>>,
    /// Source-aware structural verdicts per analysis and question;
    /// see [`analysis_answers_structural_gate`].
    structural_gates: RetainedMemo<(AnalysisPresentationKey, StructuralGate), bool>,
    /// Memoized viewer projections; see [`view_plans::ViewPlans`].
    plans: view_plans::ViewPlans,
}

/// One evaluated expression series (owned arrays, cheap to clone).
#[derive(Debug, Clone)]
pub(crate) struct ExprSeries {
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
pub(super) fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    frame_work::note_samples(frame_work::FrameSampleRead::TraceExtremes, values.len());
    rspice_results::measurements::finite_extremes(values)
}

// ---------------------------------------------------------------------------
// shared right-panel furniture
// ---------------------------------------------------------------------------

pub use rspice_results_ui::waveform::waveform_color;

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Results workspace center view (docbar + active viewer).
pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    results_keymap(ui, app);
    show_with_chrome(ui, app, ResultChrome::Full);
    waves::marker_dialog::show(ui.ctx(), &mut app.state);
    create_document::show(ui.ctx(), app);
}

/// The workspace's own single-key gestures, as the mockup's results keymap
/// defines them: fit, zoom, grid, cursor tool, drop a trace, nudge a cursor.
///
/// Two kinds of key meet here. Fit, zoom and grid are commands, and they are
/// claimed through the chord the user's own profile shows for them — their
/// registry context is the engineering canvas, which never becomes active on
/// this workspace, so the resolver would never fire them here. The rest are
/// view gestures no menu carries and no command names.
///
/// Everything is a bare key, so the map stands down whenever a widget wants
/// the keyboard or a modifier is held: nothing here may fire while the reader
/// is typing into a filter, an expression, or a marker label.
fn results_keymap(ui: &Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx();
    // A focused widget owns the keyboard: a filter, an expression, or a
    // marker label must never have a letter stolen from it by this map.
    if ctx.memory(|memory| memory.focused().is_some()) {
        return;
    }
    // Nor may the map fire underneath a dialog.
    if app.state.ui.results.session.marker_edit.is_some() {
        return;
    }
    if !app.state.simulation.has_results() {
        return;
    }
    let plain = ctx
        .input(|input| !input.modifiers.command && !input.modifiers.alt && !input.modifiers.ctrl);
    if !plain {
        return;
    }

    // Actions that already exist as commands keep the chord the rest of the
    // product shows for them, read from the user's own profile, so rebinding
    // one in preferences rebinds it here. Their registry context is the
    // engineering canvas and never becomes active on this workspace, which is
    // why the keystroke has to be claimed here rather than left to the
    // resolver.
    for (command, gesture) in [
        (
            crate::workbench::commands::vocabulary::Command::ZoomFit,
            ViewGesture::Fit,
        ),
        (
            crate::workbench::commands::vocabulary::Command::ZoomIn,
            ViewGesture::ZoomIn,
        ),
        (
            crate::workbench::commands::vocabulary::Command::ZoomOut,
            ViewGesture::ZoomOut,
        ),
    ] {
        let permitted = match gesture {
            ViewGesture::Fit => fit_gesture_available(&app.state),
            ViewGesture::ZoomIn | ViewGesture::ZoomOut => zoom_gesture_available(&app.state),
            ViewGesture::SetRanges { .. } => false,
        };
        if permitted && consume_command_key(ctx, &app.state, command) {
            request_view_gesture(&mut app.state, gesture);
        }
    }
    if viewer_uses_wave_stack(app.state.ui.results.session.viewer)
        && consume_command_key(
            ctx,
            &app.state,
            crate::workbench::commands::vocabulary::Command::CycleGrid,
        )
    {
        app.state.ui.results.session.show_minor_grid =
            !app.state.ui.results.session.show_minor_grid;
    }

    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::C)) {
        app.state.ui.results.session.toggle_cursor_tool();
    }
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Delete)) {
        hide_selected_trace(&mut app.state);
    }
    // Arrow nudging is a cursor gesture, so it only exists while cursors do.
    if app.state.ui.results.session.cursor_readout_active() {
        let shift = ctx.input(|input| input.modifiers.shift);
        let modifiers = if shift {
            egui::Modifiers::SHIFT
        } else {
            egui::Modifiers::NONE
        };
        let steps = if ctx.input_mut(|input| input.consume_key(modifiers, egui::Key::ArrowRight)) {
            1.0
        } else if ctx.input_mut(|input| input.consume_key(modifiers, egui::Key::ArrowLeft)) {
            -1.0
        } else {
            0.0
        };
        if steps != 0.0 {
            let t = Tokens::get(ctx);
            waves::nudge_cursor(&mut app.state, &t, shift, steps);
        }
    }
}

/// Claim the modifier-free keystroke the profile binds to `command`.
///
/// Only bare or shift-only chords are claimed: these are viewport gestures
/// pressed with one hand over the plot. A profile that moves the command onto
/// a modifier chord simply leaves this workspace without a key for it — the
/// menu entry is still the way in, and no key is silently invented.
fn consume_command_key(
    ctx: &egui::Context,
    state: &AppState,
    command: crate::workbench::commands::vocabulary::Command,
) -> bool {
    let platform = crate::workbench::app_state::runtime_command_platform(ctx);
    state
        .ui
        .preferences
        .shortcuts()
        .effective_bindings(command)
        .iter()
        .filter(|binding| binding.supports(platform))
        .filter_map(|binding| {
            let strokes = binding.sequence().strokes();
            let [stroke] = strokes else { return None };
            (!stroke.primary() && !stroke.alt()).then_some((stroke.key(), stroke.shift()))
        })
        .any(|(key, shift)| {
            let modifiers = if shift {
                egui::Modifiers::SHIFT
            } else {
                egui::Modifiers::NONE
            };
            ctx.input_mut(|input| input.consume_key(modifiers, key))
        })
}

/// Take the selected trace off the sheet, the way the mockup's Delete does.
///
/// Hiding is the reversible form of removal here: the retained dataset is
/// immutable, so a trace leaves the plot by losing visibility and comes back
/// from the pane's add-signal menu.
fn hide_selected_trace(state: &mut AppState) {
    let Some(selected) = state.ui.results.valid_selected_trace(&state.simulation) else {
        return;
    };
    let analysis_key = selected.analysis_key();
    let source_name = selected.source_name().to_owned();
    let Some(run) = state.simulation.active_run() else {
        return;
    };
    let located = run
        .analyses
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
        });
    if let Some((analysis_index, waveform_index)) = located {
        waves::toggle_visibility(state, analysis_index, waveform_index);
        state.ui.results.session.selected_trace = None;
    }
}

/// Render one project-owned result document selected by its stable identity.
///
/// Unlike the dataset quick-view, this projection is driven by the exact
/// pages, panes, viewer identities, and immutable bindings retained in the
/// project document.
pub(crate) fn show_persistent_document(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document_id: crate::product::ResultDocumentId,
) {
    // A project-owned document is the same workspace with a retained page
    // layout, so it answers the same keys and owns the same marker dialog.
    results_keymap(ui, app);
    persistent_document::show(ui, app, document_id);
    waves::marker_dialog::show(ui.ctx(), &mut app.state);
    create_document::show(ui.ctx(), app);
}

/// Resolve and select the primary binding of a project-owned result document.
///
/// Document-tab activation and post-create activation share this path so a
/// document can never select one retained result when created and a different
/// one when reopened.
pub(crate) fn activate_persistent_document(
    state: &mut AppState,
    document_id: crate::product::ResultDocumentId,
) -> bool {
    persistent_document::activate(state, document_id)
}

/// Render the compact upgraded-mockup projection used beside another
/// engineering document. This is the same canonical retained result state,
/// not a second result document.
pub fn show_compact_split(ui: &mut Ui, app: &mut RSpiceApp) {
    show_with_chrome(ui, app, ResultChrome::CompactSplit);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResultChrome {
    Full,
    CompactSplit,
}

fn show_with_chrome(ui: &mut Ui, app: &mut RSpiceApp, chrome: ResultChrome) {
    // Every quick-style entry into the workspace leaves the persistent
    // projection here, not just the full one: the compact split renders the
    // same strips, and a pane context left over from a project-owned document
    // would scope its plot views, cursors and marker edits onto them.
    app.state.ui.results.session.leave_persistent_document();
    app.state.ui.results.set_sample_selection(None);
    prepare_viewer_state(app);
    let plan_before_docbar = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    match chrome {
        ResultChrome::Full => {
            show_docbar(ui, app);
            if result_stage_bar_visible(&app.state) {
                show_sheet_bar(ui, &mut app.state);
            }
        }
        ResultChrome::CompactSplit => show_compact_docbar(ui, &mut app.state),
    }
    crate::ui::plot::set_interaction_mode(
        ui.ctx(),
        app.state.ui.results.session.plot_tool.interaction_mode(),
    );
    let plan_after_docbar = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    if plan_after_docbar != plan_before_docbar {
        app.invalidate_simulation_preflight();
    }

    // The stage bottom holds one bounded cursor/marker dock. Its header and
    // owning scroll body are content-fit, so a stage with nothing to report
    // gives the whole area back to the document.
    let strip_height = match chrome {
        ResultChrome::Full => readout_strip_height(&mut app.state),
        ResultChrome::CompactSplit => 0.0,
    };
    let available = ui.available_rect_before_wrap();
    let well_height = (available.height() - strip_height).max(0.0);
    ui.allocate_ui(egui::vec2(available.width(), well_height), |ui| {
        ui.set_min_height(well_height);
        show_viewer_well(ui, app, chrome);
    });
    if strip_height > 0.0 {
        waves::readout_strip(ui, &mut app.state, strip_height);
    }
}

/// Whether the active viewer owns a stage-local bar below the viewer tabs.
///
/// Plot and custom-canvas sheets use the 31 px instrument/purpose bar. OP,
/// Specs, and Table use the mockup's 40 px structured-document strip. XF and
/// Manifest own no controls there and therefore collapse the row completely.
fn result_stage_bar_visible(state: &AppState) -> bool {
    (state.ui.results.session.viewer == ResultViewer::Specs
        && state.ui.results.session.spec_drafts.is_some())
        || (state
            .simulation
            .active_run()
            .is_some_and(|run| !run.analyses.is_empty())
            && viewer_has_sheet_bar(state.ui.results.session.viewer))
}

/// Height of the stage's readout strip for the active viewer, or zero.
///
/// Only the cursor-bearing waveform viewers carry a readout; structured
/// documents (OP, specs, tables) have no cursor to report.
fn readout_strip_height(state: &mut AppState) -> f32 {
    match state.ui.results.session.viewer {
        ResultViewer::Waves
        | ResultViewer::DcSweep
        | ResultViewer::Bode
        | ResultViewer::NoiseContrib => waves::readout_strip_height(state),
        _ => 0.0,
    }
}

/// The sheet a bound pane actually draws, once the analysis behind it is known.
///
/// Several viewer documents cover a pair of sheets that differ only by what the
/// analysis turned out to be — one waveform renderer serves transient and DC
/// sweeps, one spectrum renderer serves FFT and harmonic balance, one AC
/// renderer serves Bode and ordinary noise. Which of the pair to draw is one
/// question, so it is answered once: the persistent-document layer and the
/// Studio each answered it separately and the Studio's copy was missing the
/// noise rule, so the same pane drew the noise spectrum in a result document
/// and reported an unsatisfiable Bode contract in the Studio.
pub(crate) fn project_viewer_for_analysis(
    viewer: ResultViewer,
    analysis: &crate::state::AnalysisResult,
) -> ResultViewer {
    use crate::state::AnalysisType;

    match viewer {
        ResultViewer::Waves if analysis.analysis_type == AnalysisType::DcSweep => {
            ResultViewer::DcSweep
        }
        ResultViewer::Fft | ResultViewer::HarmonicBalance
            if harmonic_balance_analysis_is_renderable(analysis) =>
        {
            ResultViewer::HarmonicBalance
        }
        ResultViewer::Bode
            if qpnoise::is_renderable(analysis)
                || bode::ordinary_noise_spectrum_is_renderable(analysis) =>
        {
            ResultViewer::NoiseContrib
        }
        _ => viewer,
    }
}

pub(crate) fn viewer_is_available(state: &AppState, viewer: ResultViewer) -> bool {
    viewer_availability(state, viewer).available
}

pub(crate) fn viewer_unavailability_reason(
    state: &AppState,
    viewer: ResultViewer,
) -> Option<&'static str> {
    let availability = viewer_availability(state, viewer);
    (!availability.available).then_some(availability.reason)
}

/// Render only the canonical result viewer well. Visualization Studio owns
/// its document toolbar, library, exact-data dock, and entity inspector, so
/// embedding the same retained renderer must not duplicate the quick-view
/// docbar or create a second result document.
pub(crate) fn show_embedded_with_sample_selection(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selection: Option<SourceSampleSelection>,
) {
    // The Studio stage is a quick-style surface too: it embeds the renderer
    // against the global projection, never against a retained document pane.
    app.state.ui.results.session.leave_persistent_document();
    app.state.ui.results.set_sample_selection(selection);
    prepare_viewer_state(app);
    show_viewer_well(ui, app, ResultChrome::Full);
}

/// Render one already-resolved persistent pane through an exact existing
/// viewer. The caller owns binding and compatibility checks; this helper only
/// refreshes renderer caches and deliberately restores the requested viewer
/// after quick-view reconciliation so no fallback viewer can be substituted.
pub(super) fn show_persistent_pane_viewer(ui: &mut Ui, app: &mut RSpiceApp, viewer: ResultViewer) {
    app.state.ui.results.set_sample_selection(None);
    prepare_viewer_state(app);
    app.state.ui.results.session.viewer = viewer;
    show_viewer_well(ui, app, ResultChrome::Full);
}

pub(crate) fn prepare_viewer_state(app: &mut RSpiceApp) {
    synchronize_quick_view_dataset_authority(&mut app.state);
    app.state
        .ui
        .results
        .reconcile_retained_datasets(&app.state.simulation);
    let data_version = app.state.simulation.data_version;
    // The user's display-cache budget is session state, and the cache is not,
    // so applying it only where the setting is edited left a restored session
    // running on the default until the reader happened to reopen that panel.
    // Reasserting it each frame keeps the two in step by construction; it
    // costs a comparison unless the budget actually shrank.
    let budget = app.state.workbench.visualization_studio.tile_memory_mib;
    let results = &mut app.state.ui.results;
    results.session.cache.set_memory_budget_mib(budget);
    results.synchronize_wave_caches(&app.state.simulation);
    if results.seen_version != data_version {
        results.seen_version = data_version;
        results.session.clear_cursors();
        results.session.horizontal_cursor = None;
        results.session.active_wave_pane = None;
        results.session.selected_trace = None;
        results.session.selected_result_artifact = None;
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.session.rf_pin.clear();
        // The merged event order belongs to the retained dataset version.
        results.event_order_cache = None;
        // Runtime operation failures and recovery notices belong to the
        // dataset generation that reported them.
        results.operational_condition = None;
    }
    results.session.cache.ensure_version(data_version);
    results.session.derived.ensure_version(data_version);

    smith::synchronize_active_analysis(&mut app.state);
    reconcile_active_viewer(&mut app.state);
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
fn synchronize_quick_view_dataset_authority(state: &mut AppState) -> bool {
    let Some(WorkspaceDocumentId::ResultDataset(dataset_id)) =
        state.workbench.documents.active(Workspace::Results)
    else {
        return false;
    };
    let Some(run_index) = state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == *dataset_id)
    else {
        return false;
    };
    if state.simulation.active_run_idx == Some(run_index) {
        return false;
    }
    state.simulation.select_run(run_index)
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
}

/// The interval the active sheet's axis is showing, pinned or fitted.
pub(crate) fn active_axis_range(
    state: &AppState,
    facts: &waves::ActivePaneFacts,
    axis: PaneAxis,
) -> Option<(f64, f64)> {
    let viewer = state.ui.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return match axis {
            PaneAxis::X => facts.x_extent,
            PaneAxis::Y => facts.y_extent,
        };
    }
    let drawn = state.ui.results.session.drawn_axes.get(&viewer).copied();
    let pinned = state.ui.results.session.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.or_else(|| drawn.map(|(x, _)| x)),
        PaneAxis::Y => pinned.y.or_else(|| drawn.map(|(_, y)| y)),
    }
}

/// Resolve the interval currently consumed by the active renderer.
///
/// Callers outside the result-document drawing code must use this adapter
/// rather than reading the legacy ordinal plot store. The waveform stack is
/// keyed by stable analysis and unit-pane identity, while single-canvas
/// viewers remain keyed by their global plot slot.
pub(crate) fn active_renderer_axis_range(
    ctx: &egui::Context,
    state: &mut AppState,
    axis: PaneAxis,
) -> Option<(f64, f64)> {
    if viewer_uses_wave_stack(state.ui.results.session.viewer) {
        let facts = active_pane_facts(&Tokens::get(ctx), state);
        return active_axis_range(state, &facts, axis);
    }
    let viewer = state.ui.results.session.viewer;
    let drawn = state.ui.results.session.drawn_axes.get(&viewer).copied();
    let pinned = state.ui.results.session.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.or_else(|| drawn.map(|(x, _)| x)),
        PaneAxis::Y => pinned.y.or_else(|| drawn.map(|(_, y)| y)),
    }
}

/// Whether the active sheet's axis is pinned to an explicit interval rather
/// than fitting its data.
pub(crate) fn active_axis_is_pinned(state: &AppState, axis: PaneAxis) -> bool {
    let viewer = state.ui.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::active_pane_axis_is_pinned(&state.ui.results, axis);
    }
    let pinned = state.ui.results.session.plot_view(viewer, 0);
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
pub(crate) fn set_active_axis_range(
    tokens: &Tokens,
    state: &mut AppState,
    axis: PaneAxis,
    range: Option<(f64, f64)>,
) -> bool {
    let viewer = state.ui.results.session.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::set_active_pane_axis_range(tokens, state, axis, range);
    }
    let view =
        state
            .ui
            .results
            .session
            .plot_view_pane_mut_for(viewer, PlotPresentationKey::Global(0), 0);
    match axis {
        PaneAxis::X => view.x = range,
        PaneAxis::Y => view.y = range,
    }
    true
}

fn show_viewer_well(ui: &mut Ui, app: &mut RSpiceApp, chrome: ResultChrome) {
    apply_pending_view_gesture(ui, &mut app.state);
    let t = Tokens::get(ui.ctx());
    // The document well backdrop; viewers paint on top. The rect doubles
    // as the crop window for viewer PNG export.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
    app.state.ui.results.session.well_rect = Some(well);
    let viewer = app.state.ui.results.session.viewer;
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

    let operational = operational_state::classify_viewer(&mut app.state, viewer);
    if operational_state::show_result_operational_status(ui, &mut app.state, &operational) {
        return;
    }

    if viewer_requires_retained_results(viewer) && !app.state.simulation.has_results() {
        let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
            crate::workbench::commands::vocabulary::Command::RunSimulation,
            crate::workbench::app_state::runtime_command_platform(ui.ctx()),
            ui.ctx().os(),
        );
        let hint = if shortcut.is_empty() {
            "No retained dataset — run a simulation".to_owned()
        } else {
            format!("No retained dataset — run a simulation ({shortcut})")
        };
        well_hint(ui, &hint);
        return;
    }

    match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => match chrome {
            ResultChrome::Full => waves::show(ui, &mut app.state),
            ResultChrome::CompactSplit => waves::show_compact(ui, &mut app.state),
        },
        ResultViewer::Bode => waves::show_bode(ui, &mut app.state),
        ResultViewer::Fft => {
            if ensure_derived(ui, app, ActiveViewer::Fft) {
                fft::show(ui, &mut app.state);
            }
        }
        ResultViewer::HarmonicBalance => harmonic_balance::show(ui, &mut app.state),
        ResultViewer::PhaseNoise => phase_noise::show(ui, &mut app.state),
        ResultViewer::Eye => {
            if ensure_derived(ui, app, ActiveViewer::EyeDiagram) {
                eye::show(ui, &mut app.state);
            }
        }
        ResultViewer::Hist => hist::show(ui, &mut app.state),
        ResultViewer::Op => op_inspector::show(ui, &mut app.state),
        ResultViewer::NoiseContrib => waves::show_noise(ui, &mut app.state),
        ResultViewer::Contribution => sensitivity::show(ui, &mut app.state),
        ResultViewer::TransferFunction => transfer_function::show(ui, &mut app.state),
        ResultViewer::Specs => specs::show(ui, &mut app.state),
        ResultViewer::Table => table::show(ui, &mut app.state),
        ResultViewer::Nyquist => nyquist::show(ui, &mut app.state),
        ResultViewer::Smith => smith::show(ui, &mut app.state),
        ResultViewer::Polar => polar::show(ui, &mut SheetContext::of(&mut app.state)),
        ResultViewer::NetworkMatrix => {
            network_matrix::show(ui, &mut SheetContext::of(&mut app.state))
        }
        ResultViewer::PoleZero => pz::show(ui, &mut app.state),
        ResultViewer::Scatter => scatter::show(ui, &mut SheetContext::of(&mut app.state)),
        ResultViewer::BoxViolin => box_violin::show(ui, &mut SheetContext::of(&mut app.state)),
        ResultViewer::Events => events::show(ui, &mut app.state),
        ResultViewer::Soa => soa::show(ui, &mut app.state),
        ResultViewer::Optimization => optimization::show(ui, &mut app.state),
        ResultViewer::Manifest => manifest::show(ui, &mut app.state),
    }
}

/// Whether this viewer is meaningless without a retained result dataset.
///
/// Specifications is deliberately excluded: it owns the plan's requirement
/// editor and must remain usable before the first run. Treating every Results
/// viewer except Manifest as dataset-only made the Simulation Studio's
/// authoring handoff open an invisible editor behind the empty-result hint.
const fn viewer_requires_retained_results(viewer: ResultViewer) -> bool {
    !matches!(viewer, ResultViewer::Manifest | ResultViewer::Specs)
}

/// Split projection of the mockup's viewer-tab row. Run binding, output
/// controls, result-document creation and properties remain owned by the full
/// Results workspace; every compatible existing viewer stays reachable here.
fn show_compact_docbar(ui: &mut Ui, state: &mut AppState) {
    let available =
        ResultViewer::all().filter(|&viewer| viewer_availability(state, viewer).available);
    if let Some(viewer) =
        chrome::bars::compact_document_bar(ui, state.ui.results.session.viewer, available)
    {
        state.ui.results.session.viewer = viewer;
    }
}

fn show_docbar(ui: &mut Ui, app: &mut RSpiceApp) {
    show_docbar_for_family(ui, app, None);
}

pub(super) fn show_persistent_docbar(ui: &mut Ui, app: &mut RSpiceApp, family_label: &str) {
    show_docbar_for_family(ui, app, Some(family_label));
}

fn show_docbar_for_family(ui: &mut Ui, app: &mut RSpiceApp, family_label: Option<&str>) {
    let available = ResultViewer::all().filter(|&viewer| {
        family_label.is_none_or(|family| family_allows_viewer(family, viewer))
            && viewer_availability(&app.state, viewer).available
    });
    let actions = chrome::bars::document_bar(ui, app.state.ui.results.session.viewer, available);
    if let Some(viewer) = actions.viewer {
        app.state.ui.results.session.viewer = viewer;
    }
    if actions.create_document {
        create_document::open(app);
    } else if actions.open_properties {
        crate::workbench::documents::visualization_studio::open(app);
        crate::workbench::documents::visualization_studio::open_document_properties(app);
    }
}

/// The domain controls a custom-canvas sheet owns, drawn left-aligned in its
/// bar. `false` means the sheet owns none and the bar states its purpose.
///
/// The controls live with the sheet that reads them rather than in one
/// growing match here: each is a statement about that sheet's own evidence —
/// which network term, which two columns, which normalization — and the sheet
/// is the only place that knows what the retained result can offer.
fn sheet_domain_controls(ui: &mut Ui, state: &mut AppState) -> bool {
    let viewer = state.ui.results.session.viewer;
    let mut context = SheetContext::of(state);
    match viewer {
        ResultViewer::Polar => polar::domain_bar(ui, &mut context),
        ResultViewer::Contribution => sensitivity::domain_bar(ui, &mut context),
        ResultViewer::Scatter => scatter::domain_bar(ui, &mut context),
        ResultViewer::BoxViolin => box_violin::domain_bar(ui, &mut context),
        ResultViewer::Events => events::domain_bar(ui, &mut context),
        _ => false,
    }
}

fn hidden_wave_strip_count(state: &AppState) -> usize {
    let Some(run) = state.simulation.active_run() else {
        return 0;
    };
    state
        .ui
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
    if analysis.validate_retained_evidence().is_err() {
        return Some("the retained evidence failed validation");
    }
    None
}

/// The same question, answered from the memo instead of the validator.
///
/// Identical verdict to [`incomplete_evidence_reason`]; the sheet bar asks it
/// every frame, and on the wave stack it asks for every analysis of the run,
/// so this is the caller that must not walk the samples.
fn memoized_incomplete_evidence_reason(
    state: &AppState,
    dataset_id: DatasetId,
    analysis: &AnalysisResult,
) -> Option<&'static str> {
    if !analysis.success {
        return Some("the run did not complete — these samples stop where it failed");
    }
    (!analysis_evidence_is_valid(state, dataset_id, analysis))
        .then_some("the retained evidence failed validation")
}

/// The same question for whichever analysis the active sheet is speaking for.
fn active_incomplete_evidence_reason(state: &AppState) -> Option<&'static str> {
    let run = state.simulation.active_run()?;
    if viewer_uses_wave_stack(state.ui.results.session.viewer) {
        // The stack draws every analysis of the run at once, so the bar
        // speaks for the run: one failed strip makes the sheet's evidence
        // incomplete even when the strip beside it converged.
        return run.analyses.iter().find_map(|analysis| {
            memoized_incomplete_evidence_reason(state, run.dataset_id, analysis)
        });
    }
    memoized_incomplete_evidence_reason(state, run.dataset_id, state.simulation.active_analysis()?)
}

fn sheet_purpose(state: &AppState) -> String {
    let viewer = state.ui.results.session.viewer;
    let detail = viewer_availability(state, viewer).reason;
    match active_incomplete_evidence_reason(state) {
        Some(caution) => format!("{} · {detail} · {caution}", viewer.tab_label()),
        None => format!("{} · {detail}", viewer.tab_label()),
    }
}

#[cfg(test)]
fn viewer_tabs(ui: &mut Ui, state: &mut AppState) {
    let available =
        ResultViewer::all().filter(|&viewer| viewer_availability(state, viewer).available);
    if let Some(viewer) = chrome::bars::viewer_tabs(ui, state.ui.results.session.viewer, available)
    {
        state.ui.results.session.viewer = viewer;
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
    create_document::ResultDocumentFamily::from_label(family_label)
        .is_none_or(|family| family.offers_sheet(viewer))
}

fn reconcile_active_viewer(state: &mut AppState) {
    if viewer_availability(state, state.ui.results.session.viewer).available {
        return;
    }
    if state.simulation.active_run().is_none() {
        return;
    }
    if let Some(viewer) =
        ResultViewer::all().find(|viewer| viewer_availability(state, *viewer).available)
    {
        state.ui.results.session.viewer = viewer;
    }
}

fn viewer_availability(state: &AppState, viewer: ResultViewer) -> ViewerAvailability {
    let active_run = state.simulation.active_run();
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
                    "Requires HB or Fourier to retain exact complex coefficients",
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
            if state.simulation.active_analysis().is_some_and(|analysis| {
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
            }) {
                ViewerAvailability::available("Operating-point evidence is available")
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
                    "Retained transfer-function evidence is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained transfer-function payload",
                )
            }
        }
        ResultViewer::Specs => {
            // The active run, not the history. The sheet reads `active_run()`
            // and nothing else, so scanning every retained run offered a tab
            // whose sheet then reported "no measured results" — the measured
            // results were in a run the reader is not looking at.
            let has_measurements = active_run.is_some_and(|run| {
                run.analyses
                    .iter()
                    .any(|analysis| !analysis.measurements.is_empty())
            });
            if !state.workspace.content.specs.is_empty() || has_measurements {
                ViewerAvailability::available("Specification or measurement data is available")
            } else {
                ViewerAvailability::unavailable("Requires specifications or measured results")
            }
        }
        ResultViewer::Table => {
            // The table reads whatever the WAVES stage can plot: if a strip
            // exists, its retained samples can be listed.
            if state
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
        ResultViewer::Smith => smith::availability(state),
        ResultViewer::Polar => {
            if active_run.is_some_and(|run| {
                state.simulation.active_analysis().is_some_and(|analysis| {
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
        ResultViewer::NetworkMatrix => network_matrix::availability(state),
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            if active_run.is_some_and(|run| {
                state.simulation.active_analysis().is_some_and(|analysis| {
                    rspice_results::population::is_a_population(analysis)
                        && analysis_evidence_is_valid(state, run.dataset_id, analysis)
                })
            }) {
                ViewerAvailability::available(
                    "A retained Monte Carlo population is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a Monte Carlo with retained per-trial evidence",
                )
            }
        }
        ResultViewer::Events => {
            if events::active_analysis_is_renderable(state) {
                ViewerAvailability::available(
                    "A committed XSPICE event history is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a transient analysis with retained XSPICE event nodes",
                )
            }
        }
        ResultViewer::Soa => {
            if soa::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained safe-operating-area evidence is available for the active analysis",
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
        ResultViewer::Manifest => {
            if active_run.is_some() {
                ViewerAvailability::available("The immutable active-dataset inventory is available")
            } else {
                ViewerAvailability::unavailable("Requires a selected retained dataset")
            }
        }
    }
}

fn specialized_availability(state: &AppState, viewer: ActiveViewer) -> ViewerAvailability {
    let capability = state.viewer_capability(viewer);
    ViewerAvailability {
        available: capability.available,
        reason: capability.reason,
    }
}

/// Gate FFT/eye rendering on the controller's derived-data loader. Returns
/// `true` when the viewer can render.
fn ensure_derived(ui: &mut Ui, app: &mut RSpiceApp, viewer: ActiveViewer) -> bool {
    let Some(analysis_type) = app
        .state
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
    match app
        .simulation_controller
        .ensure_transient_viewer_data(&mut app.state, viewer)
    {
        DerivedViewerLoadState::Ready => {
            app.state.ui.results.clear_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
            );
            true
        }
        DerivedViewerLoadState::Loading => {
            let data_version = app.state.simulation.data_version;
            app.state.ui.results.record_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
                "Preparing and validating derived data from the active transient.",
                data_version,
            );
            well_hint(ui, "Preparing derived data from the active transient…");
            ui.ctx().request_repaint();
            false
        }
        DerivedViewerLoadState::Unavailable => {
            app.state.ui.results.clear_runtime_condition(
                operational_state::ResultRuntimeConditionKind::IntegrityVerifying,
            );
            // Derived viewers can be unavailable for reasons the reader can
            // act on. Preserve the FFT builder's typed diagnostic and the
            // eye's timebase guidance at this outer gate: the individual
            // viewer is not rendered when loading reports `Unavailable`.
            let hint = match viewer {
                ActiveViewer::Fft => app
                    .state
                    .analysis
                    .fft_state
                    .last_error
                    .as_ref()
                    .map(|error| format!("Spectrum unavailable — {error}")),
                ActiveViewer::EyeDiagram => eye::unavailable_hint(&app.state),
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
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    if let Ok(view) = view_context::resolve_displayed_result_view(state)
        && let Some(analysis) = view.primary_analysis(state)
        && let Some(crate::state::AnalysisResultPayload::Qpnoise { response }) =
            &analysis.result_payload
    {
        qpnoise::summary(ui, response);
        if view.viewer == ResultViewer::NoiseContrib {
            waves::right_panel(ui, state);
            return;
        }
    }
    match state.ui.results.session.viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => waves::right_panel(ui, state),
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::HarmonicBalance => harmonic_balance::right_panel(ui, state),
        ResultViewer::PhaseNoise => phase_noise::right_panel(ui, state),
        ResultViewer::Eye => eye::right_panel(ui, state),
        ResultViewer::Hist => hist::right_panel(ui, state),
        ResultViewer::Op => op_inspector::right_panel(ui, state),
        ResultViewer::NoiseContrib => noise_contrib::right_panel(ui, state),
        ResultViewer::Contribution => sensitivity::right_panel(ui, state),
        ResultViewer::TransferFunction => transfer_function::right_panel(ui, state),
        ResultViewer::Specs => specs::right_panel(ui, state),
        ResultViewer::Table => table::right_panel(ui, state),
        ResultViewer::Nyquist => nyquist::right_panel(ui, state),
        ResultViewer::Smith => smith::right_panel(ui, state),
        ResultViewer::Polar => polar::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::NetworkMatrix => network_matrix::right_panel(ui),
        ResultViewer::PoleZero => pz::right_panel(ui, state),
        ResultViewer::Scatter => scatter::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::BoxViolin => box_violin::right_panel(ui, &mut SheetContext::of(state)),
        ResultViewer::Events => events::right_panel(ui, state),
        ResultViewer::Soa => soa::right_panel(ui, state),
        ResultViewer::Optimization => optimization::right_panel(ui, state),
        ResultViewer::Manifest => manifest::right_panel(ui, state),
    }
}

#[cfg(test)]
mod availability_tests;
