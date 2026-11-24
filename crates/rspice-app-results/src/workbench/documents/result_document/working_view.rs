//! The working view each plan, and each imported run, has: what the quick
//! view draws, and the one place what the reader does to it is kept.
//!
//! The quick view draws the run a window shows as a stack of strips, one
//! per analysis, from the session's drawing state. That state is a
//! projection of the run's working view: before the quick view draws, each
//! analysis' strip takes what the view's pane for that analysis holds (the
//! signals plotted, the expressions and their units, the X scale, the strips
//! drawn against a signal, the log axes, a closed strip, the event lanes and
//! the markers), and what the reader changed is written back through
//! [`view_edits::apply`] once the quick view has drawn. A pane is found by
//! the authored analysis every run of a plan solves, so a window showing an
//! older run of the plan shows it through the same view.
//!
//! A plan's view follows the plan's newest run unless the reader chose a run
//! for it, and then until the plan runs again; an imported run's view reads
//! that run.

use std::collections::{HashMap, HashSet};

use rspice_results_ui::session::{DocumentXyPlot, PersistentPaneContext, XyStrip};
use rspice_results_ui::waves::lanes::EventLaneLayout;

use crate::product::{DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::visualization_document::{
    AxisOrientation, AxisScale, DocumentEdit, EntityRef, NewAxis, PaneId, ResultDocumentTracking,
    ResultDocumentTrackingMode, SourceDataset, Strip, TypedValue, VisualizationDocument,
    WorkingPane, WorkingViewSource,
};
use crate::state::SimulationRun;
use crate::ui::plot::XScale;

use super::persistent_document::lanes;
use super::readout_pins;
use super::sheet_settings;
use super::{
    AnalysisPresentationKey, DocumentMarker, ExprTrace, ResultsParts, ResultsReads,
    SourceWaveformPresentationKey, WavePanePresentationKey, view_edits,
};

/// The drawing state of one run's analyses besides the reader's ticks, as
/// the quick view last took it from the view or wrote it back. A difference
/// from it is the reader's; the ticks keep which signals the reader set.
#[derive(Debug, Clone, Default, PartialEq)]
struct ViewScratch {
    expressions: HashMap<AnalysisPresentationKey, Vec<ExprTrace>>,
    units: HashMap<(AnalysisPresentationKey, String), String>,
    x_scales: HashMap<AnalysisPresentationKey, XScale>,
    xy_strips: Vec<XyStrip>,
    log_y: HashSet<WavePanePresentationKey>,
    hidden: HashSet<AnalysisPresentationKey>,
    lanes: HashMap<AnalysisPresentationKey, EventLaneLayout>,
    pins: readout_pins::Pins,
    settings: sheet_settings::Settings,
    strips: HashMap<AnalysisPresentationKey, Vec<Strip>>,
    family: HashMap<
        AnalysisPresentationKey,
        crate::results::visualization_document::FamilyPresentationPolicy,
    >,
    maximized: Option<AnalysisPresentationKey>,
}

/// The view the quick view last drew, the run it drew it over, and the
/// drawing state it left.
#[derive(Debug, Clone)]
pub(crate) struct QuickViewBinding {
    view: ResultDocumentId,
    dataset: DatasetId,
    keys: Vec<(AnalysisPresentationKey, PaneId)>,
    scratch: ViewScratch,
    /// The content of the view, and the run history, it last drew from.
    generation: u64,
    runs: crate::state::RunHistoryRevision,
    /// The drawing state changed in ways the reader's ticks do not name:
    /// it is drawn whole from the view again.
    redraw: bool,
    /// The reader changed the drawing state since it was drawn, these
    /// signals among it: it is drawn from the view again, whether or not
    /// the view took the change.
    retake: Option<Vec<SourceWaveformPresentationKey>>,
}

/// Whose working view draws `run`.
pub(crate) fn source_of(run: &SimulationRun) -> WorkingViewSource {
    run.prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .map_or(
            WorkingViewSource::Imported {
                dataset_id: run.dataset_id,
            },
            |plan_id| WorkingViewSource::Plan { plan_id },
        )
}

/// The working view of `source`, when the project has one.
pub(crate) fn view_of(
    state: ResultsReads<'_>,
    source: WorkingViewSource,
) -> Option<ResultDocumentId> {
    state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .find(|document| document.working_view() == Some(source))
        .map(VisualizationDocument::id)
}

/// The working view `run` is drawn through, when it has one.
pub fn view_of_run(state: ResultsReads<'_>, dataset_id: DatasetId) -> Option<ResultDocumentId> {
    let run = state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)?;
    view_of(state, source_of(run))
}

/// The authored analysis `run`'s view keeps as the one the data browser
/// shows: the one the reader last picked in a run of its plan.
pub(crate) fn kept_analysis(
    state: ResultsReads<'_>,
    run: &SimulationRun,
) -> Option<crate::product::AnalysisInstanceId> {
    let view = view_of(state, source_of(run))?;
    state
        .model
        .workspace
        .content
        .visualization_document(view)?
        .shown_pane()?
        .binding
        .map(|binding| binding.analysis_id)
}

/// Keep in the view of run `dataset` that the data browser shows its
/// authored analysis `analysis`. A view not made yet, or with no pane for
/// the analysis, keeps nothing.
pub(crate) fn keep_analysis(
    state: &mut ResultsParts<'_>,
    dataset: DatasetId,
    analysis: crate::product::AnalysisInstanceId,
) {
    let Some(view) = view_of_run(state.results_reads(), dataset) else {
        return;
    };
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    let Some(pane) = document.pane_of_analysis(analysis) else {
        return;
    };
    if pane.shown {
        return;
    }
    let (pane_id, revision) = (pane.id, document.revision());
    if let Err(error) = view_edits::apply_following(
        state,
        view,
        revision,
        vec![DocumentEdit::ShowPaneAnalysis { pane_id }],
    ) {
        state.prompt(format!(
            "Could not keep the analysis shown in the view: {error}"
        ));
    }
}

/// Before the quick view draws: keep what the reader changed since it last
/// drew, make or follow the shown run's view, and draw from it.
pub fn sync(state: &mut ResultsParts<'_>) {
    capture(state);
    prune(state);
    let Some(shown) = state.model.simulation.active_run() else {
        state.results.session.working_view_panes.clear();
        return;
    };
    let (dataset, source) = (shown.dataset_id, source_of(shown));
    // Following the shown run reads all of it: a copy of the run, and walks
    // of its signals' names; after a million-device run, most of a frame. A
    // pass that changed nothing changes nothing again until what it reads
    // changes, so until then the view is drawn as it stands, or, for a run
    // that has none, a view is not sought again.
    let held = state
        .results
        .working_view_followed
        .as_ref()
        .filter(|at| {
            followed_at(state.results_reads(), at.view).as_ref() == Some(*at)
                && (at.view.is_some() || view_of(state.results_reads(), source).is_none())
        })
        .map(|at| at.view);
    let view = match held {
        Some(view) => view,
        None => follow_shown(state),
    };
    match view {
        Some(view) => project(state, view, dataset),
        None => state.results.session.working_view_panes.clear(),
    }
}

/// Make or find the shown run's view and follow the run with it, noting
/// what it was followed from when that changed nothing.
fn follow_shown(state: &mut ResultsParts<'_>) -> Option<ResultDocumentId> {
    let run = state.model.simulation.active_run().cloned()?;
    let view = ensure(state, &run);
    let before = followed_at(state.results_reads(), view);
    if let Some(view) = view {
        follow_a_new_run(state, view, &run);
        follow(state, view, &run);
        reread_grown(state, view, &run);
        plot_marked_outputs(state, view, &run);
    }
    let after = followed_at(state.results_reads(), view);
    state.results.working_view_followed = after.filter(|after| before.as_ref() == Some(after));
    view
}

/// What following the shown run reads: the retained runs, the one shown,
/// and its view, if it has one, at its revision.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FollowedAt {
    runs: crate::state::RunHistoryRevision,
    shown: Option<usize>,
    view: Option<ResultDocumentId>,
    revision: Option<rspice_vocabulary::product::ObjectRevision>,
}

fn followed_at(state: ResultsReads<'_>, view: Option<ResultDocumentId>) -> Option<FollowedAt> {
    let revision = match view {
        Some(view) => Some(
            state
                .model
                .workspace
                .content
                .visualization_document(view)?
                .revision(),
        ),
        None => None,
    };
    Some(FollowedAt {
        runs: state.model.simulation.retained.runs.revision(),
        shown: state.model.simulation.view.active_run_idx,
        view,
        revision,
    })
}

/// After the quick view drew, or before anything else draws over its state:
/// write what the reader changed back to the view it drew. The ticks name
/// the signals the reader set, and the rest of the drawing state is compared
/// where it stands, so a frame in which the reader changed nothing costs
/// next to nothing whatever the view draws.
///
/// More new traces than a frame makes itself are made away from it, and
/// the view takes them in, with the rest of the change, as one step once
/// they are made; until then the quick view draws the reader's ticks.
pub fn capture(state: &mut ResultsParts<'_>) {
    let set = state.results.session.waveform_visibility.take_set();
    let Some(mut binding) = state.results.working_view.take() else {
        take_in(state, None, false);
        return;
    };
    take_in(state, Some(&mut binding), false);
    let ticks: Option<Vec<SourceWaveformPresentationKey>> = set.map(|set| {
        set.into_iter()
            .filter(|tick| binding.keys.iter().any(|(key, _)| *key == tick.analysis()))
            .collect()
    });
    let held = scratch_holds(state.results_reads(), &binding);
    if ticks.as_ref().is_some_and(Vec::is_empty) && held {
        state.results.working_view = Some(binding);
        return;
    }
    // What the reader changed is kept after what an earlier change of
    // theirs is still making, so the steps stand in the order taken.
    take_in(state, Some(&mut binding), true);
    let Captured {
        mut edits,
        new,
        normalize,
    } = capture_edits(state, &binding, ticks.as_deref());
    let view = binding.view;
    // Only ticks the view takes as they stand leave the quick view drawing
    // what it draws: the view's change is the reader's, and nothing else is
    // drawn from it (a marker on a signal taken off loses its anchor).
    let normalize = normalize.filter(|_| {
        held && state
            .model
            .workspace
            .content
            .visualization_document(view)
            .is_some_and(|document| {
                edits.iter().all(|edit| match edit {
                    DocumentEdit::SetSignalsPlotted {
                        pane_id,
                        plotted: false,
                        ..
                    } => !document
                        .markers()
                        .iter()
                        .any(|marker| marker.pane_id == *pane_id),
                    DocumentEdit::SetSignalsPlotted { .. } => true,
                    _ => false,
                })
            })
    });
    let making: usize = new.iter().map(|(_, names)| names.len()).sum();
    let preparations = (making > FRAME_TRACES)
        .then(|| {
            let document = state.model.workspace.content.visualization_document(view)?;
            new.iter()
                .map(|(pane_id, _)| document.plot_preparation(*pane_id))
                .collect::<Option<Vec<_>>>()
        })
        .flatten();
    match preparations {
        Some(preparations) => {
            let work: Vec<_> = preparations
                .into_iter()
                .zip(new.into_iter().map(|(_, names)| names))
                .collect();
            let waker = state.results.working_view_plotting.waker.clone();
            let spawned = std::thread::Builder::new()
                .name("plot traces".to_owned())
                .spawn(move || {
                    let made = work
                        .into_iter()
                        .map(|(preparation, names)| preparation.prepare(names))
                        .collect();
                    if let Some(ctx) = waker {
                        ctx.request_repaint();
                    }
                    made
                });
            match spawned {
                Ok(making) => {
                    state.results.working_view_plotting.pending = Some(Box::new(Plotting {
                        view,
                        edits,
                        ticks,
                        normalize,
                        making,
                    }));
                }
                // A thread that cannot start makes nothing: the ticks are
                // drawn from the view again, as for a change it refused.
                Err(error) => keep(
                    state,
                    view,
                    Some(&mut binding),
                    Err(error.to_string()),
                    ticks,
                    None,
                ),
            }
        }
        None => {
            edits.extend(new.into_iter().map(|(pane_id, signals)| {
                DocumentEdit::SetSignalsPlotted {
                    pane_id,
                    signals,
                    plotted: true,
                }
            }));
            keep(state, view, Some(&mut binding), Ok(edits), ticks, normalize);
        }
    }
    binding.scratch = scratch_of(state.results_reads(), &binding.keys);
    state.results.working_view = Some(binding);
}

/// How many new traces a frame makes itself: each is a few small
/// allocations, a third of a microsecond, so about this many take a
/// millisecond. More are made away from the frame. The page has one
/// thread, and makes them all.
const FRAME_TRACES: usize = if cfg!(target_arch = "wasm32") {
    usize::MAX
} else {
    3_000
};

/// A tick the quick view draws other than the view draws it: the reader's
/// setting as it stood, and the view's.
type Normal = (SourceWaveformPresentationKey, Option<bool>, Option<bool>);

/// Traces being made away from the frame for one change of the reader's,
/// and the rest of that change, which the view takes in with them.
struct Plotting {
    view: ResultDocumentId,
    edits: Vec<DocumentEdit>,
    ticks: Option<Vec<SourceWaveformPresentationKey>>,
    normalize: Option<Vec<Normal>>,
    making: std::thread::JoinHandle<
        Result<
            Vec<crate::results::visualization_document::PreparedPlot>,
            crate::results::visualization_document::VisualizationError,
        >,
    >,
}

/// The traces a working view is being given away from the frame, if any,
/// and the window to wake once they are made.
#[derive(Default)]
pub(crate) struct PlottingSlot {
    pending: Option<Box<Plotting>>,
    waker: Option<egui::Context>,
}

impl Clone for PlottingSlot {
    /// A copy is not given the traces being made: they go to the view of
    /// the state they were made for. It wakes the same window.
    fn clone(&self) -> Self {
        Self {
            pending: None,
            waker: self.waker.clone(),
        }
    }
}

impl std::fmt::Debug for PlottingSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlottingSlot")
            .field(
                "pending",
                &self.pending.as_ref().map(|pending| pending.view),
            )
            .finish_non_exhaustive()
    }
}

impl PlottingSlot {
    /// The view traces are being made for, while they are.
    pub(crate) fn view(&self) -> Option<ResultDocumentId> {
        self.pending.as_ref().map(|pending| pending.view)
    }

    /// Make nothing more for `view`, which is gone.
    pub(crate) fn forget(&mut self, view: ResultDocumentId) {
        if self.view() == Some(view) {
            self.pending = None;
        }
    }
}

/// Wake `ctx` when traces made away from the frame are ready for the view.
pub(crate) fn wake_on_plotted(state: &mut ResultsParts<'_>, ctx: &egui::Context) {
    let slot = &mut state.results.working_view_plotting;
    if slot.waker.is_none() {
        slot.waker = Some(ctx.clone());
    }
}

/// Have the view take in the traces being made for it, waiting for them:
/// before anything else changes the view or draws from it.
pub fn settle(state: &mut ResultsParts<'_>) {
    if state.results.working_view_plotting.pending.is_none() {
        return;
    }
    let mut binding = state.results.working_view.take();
    take_in(state, binding.as_deref_mut(), true);
    if state.results.working_view.is_none() {
        state.results.working_view = binding;
    }
}

/// Have the view take in the traces made away from the frame, once they are
/// made, or with `wait` as soon as they are.
fn take_in(state: &mut ResultsParts<'_>, binding: Option<&mut QuickViewBinding>, wait: bool) {
    let slot = &mut state.results.working_view_plotting;
    if !slot
        .pending
        .as_ref()
        .is_some_and(|pending| wait || pending.making.is_finished())
    {
        return;
    }
    let Some(pending) = slot.pending.take() else {
        return;
    };
    let Plotting {
        view,
        mut edits,
        ticks,
        normalize,
        making,
    } = *pending;
    let made = match making.join() {
        Ok(made) => made,
        // A preparation that failed fails the frame, as it would have had
        // the frame made the traces.
        Err(panic) => std::panic::resume_unwind(panic),
    };
    let edits = made
        .map(|prepared| {
            edits.extend(prepared.into_iter().map(DocumentEdit::PlotPrepared));
            edits
        })
        .map_err(|error| error.to_string());
    keep(state, view, binding, edits, ticks, normalize);
}

/// Write `edits`, what the reader changed since the quick view drew, to
/// `view`. With `normalize`, the edits are ticks alone: when the view takes
/// them as the quick view drew it, the quick view goes on drawing what it
/// draws, each tick as the view would draw it. Otherwise, whatever the view
/// took, the state is drawn from it again: what it refused or could not
/// hold goes back to what it holds.
fn keep(
    state: &mut ResultsParts<'_>,
    view: ResultDocumentId,
    mut binding: Option<&mut QuickViewBinding>,
    edits: Result<Vec<DocumentEdit>, String>,
    ticks: Option<Vec<SourceWaveformPresentationKey>>,
    normalize: Option<Vec<Normal>>,
) {
    let drew = binding
        .as_deref()
        .map(|binding| (binding.view, binding.generation));
    let stands = |state: &ResultsParts<'_>| {
        state
            .model
            .workspace
            .content
            .visualization_document(view)
            .map(|document| (document.revision(), document.generation()))
    };
    let mut drawn = false;
    match edits {
        Ok(edits) if edits.is_empty() => {
            drawn = drew.is_some() && drew == stands(state).map(|(_, at)| (view, at));
        }
        Ok(edits) => {
            if let Some((revision, generation)) = stands(state) {
                match view_edits::apply(state, view, revision, edits) {
                    Ok(receipt) => {
                        drawn = drew == Some((view, generation))
                            && receipt
                                .created
                                .iter()
                                .all(|entity| matches!(entity, EntityRef::Trace(_)));
                    }
                    Err(error) => {
                        state.prompt(format!("Could not keep this change to the view: {error}"));
                    }
                }
            }
        }
        Err(error) => state.prompt(format!("Could not keep this change to the view: {error}")),
    }
    let normalize = normalize.filter(|_| drawn);
    if let Some(normalize) = normalize {
        if let (Some(binding), Some((_, generation))) = (binding.as_deref_mut(), stands(state)) {
            binding.generation = generation;
        }
        for (key, set, visible) in normalize {
            // A tick the reader set again since is theirs, kept next.
            if state.results.session.waveform_visibility.get(&key).copied() == set {
                state.results.project_waveform_visible(key, visible);
            }
        }
        return;
    }
    if let Some(binding) = binding.filter(|binding| binding.view == view) {
        match ticks {
            Some(ticks) => binding.retake.get_or_insert_with(Vec::new).extend(ticks),
            None => binding.redraw = true,
        }
    }
}

/// Whether following the shown run, when it last changed nothing, would
/// change nothing still for `view` as it stands.
pub(crate) fn following_holds(state: ResultsReads<'_>, view: ResultDocumentId) -> bool {
    state
        .results
        .working_view_followed
        .as_ref()
        .is_some_and(|at| at.view == Some(view) && followed_at(state, at.view).as_ref() == Some(at))
}

/// After an edit of `view` that changes nothing following the run reads
/// (its tracking, the runs its panes read, the signals it drew), committed
/// at `revision`: a follow that changed nothing before, with `followed`,
/// still changes nothing.
pub(crate) fn still_followed(
    state: &mut ResultsParts<'_>,
    followed: bool,
    revision: rspice_vocabulary::product::ObjectRevision,
) {
    if followed && let Some(at) = &mut state.results.working_view_followed {
        at.revision = Some(revision);
    }
}

/// Forget the drawing state the quick view left: something else, a saved
/// document's pane, is about to draw over it.
pub(crate) fn release(state: &mut ResultsParts<'_>) {
    capture(state);
    settle(state);
    if state.results.working_view.take().is_some() {
        state.results.session.project_document_markers(Vec::new());
    }
    state.results.session.working_view_panes.clear();
}

/// Forget the working views of imported runs the history no longer keeps:
/// nothing can draw them again. A plan's view stays for its next run.
fn prune(state: &mut ResultsParts<'_>) {
    let runs = &state.model.simulation.retained.runs;
    let gone: Vec<ResultDocumentId> = state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .filter(|document| match document.working_view() {
            Some(WorkingViewSource::Imported { dataset_id }) => {
                !runs.iter().any(|run| run.dataset_id == dataset_id)
            }
            _ => false,
        })
        .map(VisualizationDocument::id)
        .collect();
    for view in gone {
        view_edits::forget(state, view);
    }
}

/// The working view `run` is drawn through, made the first time a run of
/// its plan, or the imported run itself, is shown.
fn ensure(state: &mut ResultsParts<'_>, run: &SimulationRun) -> Option<ResultDocumentId> {
    let source = source_of(run);
    if let Some(view) = view_of(state.results_reads(), source) {
        return Some(view);
    }
    let dataset = run_reference(state.results_reads(), run)?;
    let panes: Vec<WorkingPane> = run
        .analyses
        .iter()
        .filter_map(|analysis| {
            let viewer = super::create_document::view_pane_viewer(analysis)?;
            Some(WorkingPane {
                title: analysis.label.clone(),
                kind: super::create_document::pane_kind(viewer.art),
                viewer_id: viewer.id.to_owned(),
                analysis_id: super::create_document::source_analysis_identity(run, analysis),
                plotted: first_plotted(&state.results.session, run, analysis),
            })
        })
        .collect();
    if panes.is_empty() {
        return None;
    }
    let tracking = match source {
        WorkingViewSource::Plan { plan_id } => ResultDocumentTracking::for_plan(
            ResultDocumentTrackingMode::Latest,
            plan_id,
            panes[0].analysis_id,
        ),
        WorkingViewSource::Imported { .. } => ResultDocumentTracking::pinned(),
    };
    let document = match VisualizationDocument::new_working_view(source, dataset, tracking, panes) {
        Ok(document) => document,
        Err(error) => {
            state.prompt(format!(
                "Results could not make a view of this run: {error}"
            ));
            return None;
        }
    };
    match view_edits::create(state, document) {
        Ok(view) => Some(view),
        Err(error) => {
            state.prompt(format!(
                "Results could not keep a view of this run: {error}"
            ));
            None
        }
    }
}

/// Plot on `view` the signals `run` plots (a plan's outputs marked Plot)
/// that the view has never drawn: an output a re-run adds, the outputs of
/// an analysis it adds, and the rest of a run whose first signals were shown
/// while it was still running. A signal the reader took off keeps its
/// hidden trace and stays off. The view does this of itself: no step of the
/// reader's to undo.
fn plot_marked_outputs(state: &mut ResultsParts<'_>, view: ResultDocumentId, run: &SimulationRun) {
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    let mut edits = Vec::new();
    for (index, _, pane_id) in bound_panes(document, run) {
        let reads_run = document.panes().iter().any(|pane| {
            pane.id == pane_id
                && pane
                    .binding
                    .is_some_and(|binding| binding.dataset.dataset_id == run.dataset_id)
        });
        if !reads_run {
            continue;
        }
        let drawn: HashSet<&str> = document
            .traces()
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .map(trace_signal)
            .collect();
        let signals: Vec<String> = run.analyses[index]
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible && !drawn.contains(waveform.name.as_str()))
            .map(|waveform| waveform.name.clone())
            .collect();
        if !signals.is_empty() {
            edits.push(DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals,
                plotted: true,
            });
        }
    }
    if edits.is_empty() {
        return;
    }
    let revision = document.revision();
    if let Err(error) = view_edits::apply_following(state, view, revision, edits) {
        state.prompt(format!(
            "The view could not plot what {} plots: {error}",
            run.label
        ));
    }
}

/// The signals a plan's first run plots: the outputs it marks for plotting,
/// and any the reader plotted or took off before the run was first shown
/// (from a deck's Plot command, say).
fn first_plotted(
    session: &rspice_results_ui::session::ResultViewerState,
    run: &SimulationRun,
    analysis: &crate::state::AnalysisResult,
) -> Vec<String> {
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            session.waveform_visibility(
                &SourceWaveformPresentationKey::new(key, waveform.name.clone()),
                waveform.visible,
            )
        })
        .map(|waveform| waveform.name.clone())
        .collect()
}

/// `run` as a view reads it: by identity and digest, and every analysis it
/// solved. The view lists none of its signals, so a run of any size has one.
fn run_reference(state: ResultsReads<'_>, run: &SimulationRun) -> Option<SourceDataset> {
    let analyses = run
        .analyses
        .iter()
        .map(|analysis| super::create_document::run_source_analysis(run, analysis))
        .collect();
    SourceDataset::run_reference(
        DatasetBinding::new(run.dataset_id, super::retained_dataset_digest(state, run)),
        super::create_document::reference_run_label(run),
        analyses,
    )
    .ok()
}

/// The newest retained run of `plan`.
fn newest_run_of_plan(
    state: ResultsReads<'_>,
    source: WorkingViewSource,
) -> Option<&SimulationRun> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| source_of(run) == source)
        .max_by_key(|run| run.id)
}

/// A plan that runs again shows its new run, as Assembler's Results tab
/// turns to the history a run makes: a view kept on an earlier run follows
/// the plan's newest again once the newest is the run shown. The view
/// changes itself here, so the reader has no step to undo.
fn follow_a_new_run(state: &mut ResultsParts<'_>, view: ResultDocumentId, shown: &SimulationRun) {
    let source = source_of(shown);
    let WorkingViewSource::Plan { .. } = source else {
        return;
    };
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    let mut tracking = document.tracking();
    let reads_shown = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding)
        .any(|binding| binding.dataset.dataset_id == shown.dataset_id);
    if tracking.mode != ResultDocumentTrackingMode::Pinned
        || reads_shown
        || newest_run_of_plan(state.results_reads(), source).map(|run| run.dataset_id)
            != Some(shown.dataset_id)
    {
        return;
    }
    tracking.mode = ResultDocumentTrackingMode::Latest;
    let revision = document.revision();
    if let Err(error) = view_edits::apply_following(
        state,
        view,
        revision,
        vec![DocumentEdit::SetTracking(tracking)],
    ) {
        state.prompt(format!(
            "The view could not follow {}: {error}",
            shown.label
        ));
    }
}

/// Move a view that follows its plan onto the plan's newest run.
fn follow(state: &mut ResultsParts<'_>, view: ResultDocumentId, shown: &SimulationRun) {
    let source = source_of(shown);
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    if document.tracking().mode != ResultDocumentTrackingMode::Latest {
        return;
    }
    let Some(newest) = newest_run_of_plan(state.results_reads(), source).cloned() else {
        return;
    };
    show_run(state, view, &newest);
}

/// Read `run` again through `view` when the view reads it and it grew since
/// (it was shown while still running), whether the view follows its plan or
/// stays on this run.
fn reread_grown(state: &mut ResultsParts<'_>, view: ResultDocumentId, run: &SimulationRun) {
    let reads_run = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .is_some_and(|document| {
            document
                .panes()
                .iter()
                .filter_map(|pane| pane.binding)
                .any(|binding| binding.dataset.dataset_id == run.dataset_id)
        });
    if reads_run {
        show_run(state, view, run);
    }
}

/// Read `run` through `view`: each pane onto the same analysis of `run`, and
/// a pane for each analysis of `run` the view does not draw yet.
fn show_run(state: &mut ResultsParts<'_>, view: ResultDocumentId, run: &SimulationRun) {
    let finished = if run.lifecycle.is_terminal() {
        state.results.read_while_running.remove(&run.dataset_id)
    } else {
        state.results.read_while_running.insert(run.dataset_id);
        false
    };
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    let reads_run = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding)
        .any(|binding| binding.dataset.dataset_id == run.dataset_id);
    // The run the view reads grew since the view last read it: it was shown
    // while still running, and has analyses now that it had not then, or it
    // finished with samples it had not then. A signal it gained needs no
    // reading: the view names the signals it plots, not the run's list.
    let grown = reads_run
        && document
            .datasets()
            .iter()
            .find(|dataset| dataset.binding().dataset_id == run.dataset_id)
            .is_some_and(|held| {
                let analyses_changed = held.run_source().is_none_or(|source| {
                    run.analyses.iter().any(|analysis| {
                        source
                            .analysis(super::create_document::source_analysis_identity(
                                run, analysis,
                            ))
                            .is_none()
                    })
                });
                analyses_changed
                    || (finished
                        && held.binding().content_digest
                            != super::retained_dataset_digest(state.results_reads(), run))
            });
    let missing: Vec<_> = run
        .analyses
        .iter()
        .filter(|analysis| {
            document
                .pane_of_analysis(super::create_document::source_analysis_identity(
                    run, analysis,
                ))
                .is_none()
        })
        .collect();
    if reads_run && missing.is_empty() && !grown {
        return;
    }
    let revision = document.revision();
    let page_id = document.pages()[0].id;
    let last_pane = document
        .panes()
        .iter()
        .max_by_key(|pane| pane.order)
        .map(|pane| pane.id);
    let Some(next) = run_reference(state.results_reads(), run) else {
        return;
    };
    let binding = next.binding();
    let mut edits = vec![DocumentEdit::FollowRun { next }];
    let mut anchor = last_pane;
    for analysis in missing {
        let Some(viewer) = super::create_document::view_pane_viewer(analysis) else {
            continue;
        };
        edits.push(DocumentEdit::AddViewPane(
            crate::results::visualization_document::NewPane {
                page_id,
                title: analysis.label.clone(),
                kind: super::create_document::pane_kind(viewer.art),
                viewer_id: viewer.id.to_owned(),
                binding: Some(crate::results::visualization_document::PaneDataBinding {
                    analysis_id: super::create_document::source_analysis_identity(run, analysis),
                    dataset: binding,
                }),
                placement: anchor.map_or(
                    crate::results::visualization_document::PanePlacement::Primary,
                    |anchor_pane_id| crate::results::visualization_document::PanePlacement::Below {
                        anchor_pane_id,
                    },
                ),
                plotted: Vec::new(),
            },
        ));
        // A pane added in this transaction has no identity to anchor the
        // next one to yet; each new pane goes after the view's last.
        anchor = last_pane;
    }
    if let Err(error) = view_edits::apply(state, view, revision, edits) {
        state.prompt(format!(
            "The view could not move onto {}: {error}",
            run.label
        ));
    }
}

/// Choose the run `view` reads and show it: `Some` keeps the view on that
/// run, `None` makes it follow its plan's newest run again.
pub fn choose_run(
    state: &mut ResultsParts<'_>,
    view: ResultDocumentId,
    run: Option<DatasetId>,
) -> Result<(), String> {
    let document = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .ok_or_else(|| "The view is no longer in the project.".to_owned())?;
    let source = document
        .working_view()
        .ok_or_else(|| "Only a working view chooses its run.".to_owned())?;
    let mut tracking = document.tracking();
    let target = match run {
        Some(dataset_id) => state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .find(|run| run.dataset_id == dataset_id && source_of(run) == source)
            .cloned()
            .ok_or_else(|| "That run is not a run of this view's plan.".to_owned())?,
        None => newest_run_of_plan(state.results_reads(), source)
            .cloned()
            .ok_or_else(|| "The plan has no retained run to follow.".to_owned())?,
    };
    if let WorkingViewSource::Plan { .. } = source {
        tracking.mode = if run.is_some() {
            ResultDocumentTrackingMode::Pinned
        } else {
            ResultDocumentTrackingMode::Latest
        };
        if tracking != document.tracking() {
            let revision = document.revision();
            view_edits::apply(
                state,
                view,
                revision,
                vec![DocumentEdit::SetTracking(tracking)],
            )
            .map_err(|error| error.to_string())?;
        }
    }
    show_run(state, view, &target);
    let simulation = &mut state.model.simulation;
    if let Some(index) = simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == target.dataset_id)
    {
        simulation.select_run(index);
        super::open_shown_run(state);
    }
    Ok(())
}

/// Save the working view `view` as a document named `title`, copied whole:
/// its hidden traces, markers, axes and expressions, and whether it follows
/// its plan's newest run or stays on the run it shows. What the reader did in
/// the quick view since it last drew is kept in the view first.
pub fn save_as_document(
    state: &mut ResultsParts<'_>,
    view: ResultDocumentId,
    title: &str,
) -> Result<ResultDocumentId, String> {
    capture(state);
    let saved = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .ok_or_else(|| "The view is no longer in the project.".to_owned())?
        .saved_copy(title.trim())
        .map_err(|error| error.to_string())?;
    view_edits::create(state, saved).map_err(|error| error.to_string())
}

/// The analyses of `run` and the panes of `view` that draw them.
fn bound_panes(
    document: &VisualizationDocument,
    run: &SimulationRun,
) -> Vec<(usize, AnalysisPresentationKey, PaneId)> {
    run.analyses
        .iter()
        .enumerate()
        .filter_map(|(index, analysis)| {
            let pane = document.pane_of_analysis(
                super::create_document::source_analysis_identity(run, analysis),
            )?;
            Some((
                index,
                AnalysisPresentationKey::new(run.dataset_id, analysis),
                pane.id,
            ))
        })
        .collect()
}

/// The signal a trace reads, by the name its run gives it.
fn trace_signal(trace: &crate::results::visualization_document::Trace) -> &str {
    trace.source_signal().unwrap_or(&trace.label)
}

/// The retained run `dataset`, borrowed where it is kept.
fn retained_run(
    model: &rspice_app_state::workbench::app_state::model::Model,
    dataset: DatasetId,
) -> Option<&SimulationRun> {
    model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset)
}

/// Draw the retained run `dataset` from `view`. The first time, or once the
/// run history changed, each pane is drawn whole; after that only what the
/// view changed since the quick view last drew it, and what the reader
/// changed that the view may not have taken. A frame in which neither
/// changed draws nothing.
fn project(state: &mut ResultsParts<'_>, view: ResultDocumentId, dataset: DatasetId) {
    // What is drawn from the view again waits for the traces being made
    // for it, which the quick view draws already.
    if state.results.working_view_plotting.view().is_some()
        && !drawn_as_it_stands(state.results_reads(), view, dataset)
    {
        settle(state);
    }
    let runs = state.model.simulation.retained.runs.revision();
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return;
    };
    let generation = document.generation();
    let drawn = state.results.working_view.as_deref().filter(|binding| {
        binding.view == view
            && binding.dataset == dataset
            && binding.runs == runs
            && !binding.redraw
    });
    if drawn.is_some_and(|binding| binding.generation == generation && binding.retake.is_none()) {
        let results = &mut *state.results;
        if let Some(binding) = results.working_view.as_deref() {
            let session = &mut results.session;
            let panes = binding
                .keys
                .iter()
                .map(|&(analysis, pane_id)| PersistentPaneContext {
                    document_id: view,
                    pane_id,
                    analysis,
                });
            if !session.working_view_panes.iter().copied().eq(panes.clone()) {
                session.working_view_panes = panes.collect();
            }
        }
        return;
    }
    // The signals each pane drawn before draws again; a pane it does not
    // name is drawn whole.
    let again: Option<HashMap<PaneId, HashSet<String>>> = drawn.and_then(|binding| {
        let changed = document.signals_changed_since(binding.generation)?;
        let mut again: HashMap<PaneId, HashSet<String>> = binding
            .keys
            .iter()
            .map(|&(_, pane_id)| (pane_id, HashSet::new()))
            .collect();
        for (pane_id, signal) in changed {
            if let Some(signals) = again.get_mut(&pane_id) {
                signals.insert(signal.to_owned());
            }
        }
        for tick in binding.retake.iter().flatten() {
            if let Some((_, pane_id)) = binding.keys.iter().find(|(key, _)| *key == tick.analysis())
                && let Some(signals) = again.get_mut(pane_id)
            {
                signals.insert(tick.source_name().to_owned());
            }
        }
        Some(again)
    });
    // A pane of event lanes draws its lanes again where the view changed
    // them, or an event net they list, or the reader's change is retaken.
    let lanes_again = drawn.is_none_or(|binding| {
        document.event_lanes_changed_since(binding.generation) != Some(false)
    });
    let retaken: Vec<AnalysisPresentationKey> = drawn
        .and_then(|binding| binding.retake.as_ref())
        .map(|ticks| ticks.iter().map(SourceWaveformPresentationKey::analysis).collect())
        .unwrap_or_default();
    // The run is borrowed where it is kept, not copied: a copy is every
    // signal's name.
    let Some(run) = retained_run(state.model, dataset) else {
        return;
    };
    let panes = bound_panes(document, run);
    let mut markers = Vec::new();
    for &(index, key, pane_id) in &panes {
        let analysis = &run.analyses[index];
        let Some(pane) = document.panes().iter().find(|pane| pane.id == pane_id) else {
            continue;
        };
        let events = lanes::has_event_lanes(analysis);
        let event_names = || {
            let index = state
                .results
                .session
                .signal_indices
                .index(&runs, key, &analysis.data);
            lanes::EventNames::of(analysis, index.as_deref())
        };
        let names = match again.as_ref().and_then(|again| again.get(&pane_id)) {
            Some(signals) => {
                let names = (events
                    && (lanes_again
                        || retaken.contains(&key)
                        || signals.iter().any(|signal| is_event_net(signal))))
                .then(event_names)
                .flatten();
                project_signals(
                    state.results,
                    &runs,
                    document,
                    key,
                    pane_id,
                    analysis,
                    signals,
                );
                if let Some(names) = &names {
                    project_history_nets(state.results, document, key, pane_id, names);
                }
                names
            }
            None => {
                let names = events.then(event_names).flatten();
                project_traces(
                    state.results,
                    &runs,
                    document,
                    key,
                    pane_id,
                    analysis,
                    names.as_ref(),
                );
                names
            }
        };

        let held: Vec<_> = document
            .expression_traces()
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .collect();
        state.results.session.project_document_expressions(
            key,
            index,
            held.iter()
                .map(|trace| ExprTrace {
                    text: trace.text.clone(),
                    complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
                    visible: trace.visible,
                })
                .collect(),
            held.iter()
                .filter_map(|trace| Some((trace.text.clone(), trace.unit.clone()?)))
                .collect(),
        );
        if let Some(names) = names {
            let session = &mut state.results.session;
            let layout =
                lanes::projected_layout(document, pane_id, &names, session.event_lanes.get(&key));
            if session.event_lanes.get(&key) != Some(&layout) {
                session.event_lanes.insert(key, layout);
            }
        }
        let axes: Vec<_> = document
            .axes()
            .iter()
            .filter(|axis| axis.pane_id == pane_id)
            .collect();
        if let Some(axis) = axes
            .iter()
            .find(|axis| axis.orientation == AxisOrientation::Horizontal)
        {
            state.results.session.project_document_x_scale(
                key,
                if axis.scale == AxisScale::Logarithmic {
                    XScale::Log10
                } else {
                    XScale::Linear
                },
            );
        }
        state.results.session.project_document_xy_strips(
            key,
            document
                .xy_plots()
                .iter()
                .filter(|plot| plot.pane_id == pane_id)
                .map(|plot| DocumentXyPlot {
                    unit: plot.unit.clone(),
                    abscissa: plot.abscissa.clone(),
                    sweep_range: plot.sweep_range.map(|range| (range.minimum, range.maximum)),
                    follow_view: plot.follow_sweep_view,
                    log_x: plot.log_abscissa,
                })
                .collect(),
        );
        let session = &mut state.results.session;
        session.log_y_panes.retain(|pane| pane.analysis != key);
        session.log_y_panes.extend(
            axes.iter()
                .filter(|axis| {
                    axis.orientation == AxisOrientation::VerticalLeft
                        && axis.scale == AxisScale::Logarithmic
                })
                .filter_map(|axis| {
                    Some(WavePanePresentationKey {
                        analysis: key,
                        unit: pane_unit(axis.unit.as_deref()?).to_owned(),
                    })
                }),
        );
        if pane.hidden {
            session.hidden_strips.insert(key);
        } else {
            session.hidden_strips.remove(&key);
        }
        readout_pins::project(&mut session.readout_pins, document, key, pane_id);
        sheet_settings::project(&mut session.sheet_settings, document, key, pane_id);
        if pane.strips.is_empty() {
            session.strip_layouts.remove(&key);
        } else if session.strip_layouts.get(&key) != Some(&pane.strips) {
            session.strip_layouts.insert(key, pane.strips.clone());
        }
        // The family slice and the maximized strip are the view's too.
        match &pane.family_policy {
            Some(policy) if session.family_policies.get(&key) != Some(policy) => {
                session.family_policies.insert(key, policy.clone());
            }
            Some(_) => {}
            None => {
                session.family_policies.remove(&key);
            }
        }
        if pane.maximized {
            session.maximized_strip = Some(key);
        } else if session.maximized_strip == Some(key) {
            session.maximized_strip = None;
        }

        for marker in document
            .markers()
            .iter()
            .filter(|marker| marker.pane_id == pane_id)
        {
            let (Some(trace), TypedValue::Real(x)) =
                (document.trace(marker.trace_id), &marker.coordinate)
            else {
                continue;
            };
            if trace.pane_id != pane_id {
                continue;
            }
            markers.push(MarkerAt {
                index,
                pane_id,
                retained_id: marker.id,
                signal: trace_signal(trace).to_owned(),
                trace_name: trace.label.clone(),
                x: *x,
                kind: marker.kind,
                note: super::marker_note(&marker.label, &trace.label),
            });
        }
    }
    // A marker's anchor is read through the strip models, which may be
    // built as it is read, once every pane is drawn. A marker on a trace
    // that is not drawn has nothing to sit on.
    let mut anchored = Vec::with_capacity(markers.len());
    for marker in markers {
        let Some((analysis, anchor)) =
            super::waves::source_waveform_anchor_in(state, marker.index, &marker.signal)
        else {
            continue;
        };
        anchored.push(DocumentMarker {
            document_id: view,
            pane_id: marker.pane_id,
            retained_id: marker.retained_id,
            analysis,
            anchor,
            trace_name: marker.trace_name,
            x: marker.x,
            kind: super::marker_kind_of_retained(marker.kind),
            note: marker.note,
        });
    }
    let session = &mut state.results.session;
    session.project_document_markers(anchored);
    session.working_view_panes = panes
        .iter()
        .map(|&(_, analysis, pane_id)| PersistentPaneContext {
            document_id: view,
            pane_id,
            analysis,
        })
        .collect();
    let keys: Vec<_> = panes.iter().map(|&(_, key, pane)| (key, pane)).collect();
    let scratch = scratch_of(state.results_reads(), &keys);
    state.results.working_view = Some(Box::new(QuickViewBinding {
        view,
        dataset,
        keys,
        scratch,
        generation,
        runs,
        redraw: false,
        retake: None,
    }));
}

/// Whether the quick view draws `view` over the retained run `dataset` as
/// the view stands, with nothing of the reader's to draw from it again.
fn drawn_as_it_stands(state: ResultsReads<'_>, view: ResultDocumentId, dataset: DatasetId) -> bool {
    let Some(document) = state.model.workspace.content.visualization_document(view) else {
        return false;
    };
    state
        .results
        .working_view
        .as_deref()
        .is_some_and(|binding| {
            binding.view == view
                && binding.dataset == dataset
                && binding.runs == state.model.simulation.retained.runs.revision()
                && !binding.redraw
                && binding.generation == document.generation()
                && binding.retake.is_none()
        })
}

/// A marker of the view, read off it before its anchor is.
struct MarkerAt {
    index: usize,
    pane_id: PaneId,
    retained_id: crate::results::visualization_document::MarkerId,
    signal: String,
    trace_name: String,
    x: f64,
    kind: crate::results::visualization_document::PlotMarkerKind,
    note: String,
}

/// Draw each signal pane `pane_id` draws as the view holds it: whether it
/// is shown, its colour and its look.
fn project_traces(
    results: &mut super::ResultsState,
    runs: &crate::state::RunHistoryRevision,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
    analysis: &crate::state::AnalysisResult,
    names: Option<&lanes::EventNames>,
) {
    let traces: Vec<_> = document
        .traces()
        .iter()
        .filter(|trace| trace.pane_id == pane_id)
        .collect();
    // Each signal the pane draws, and whether every trace of it is shown.
    let mut drawn = HashMap::<&str, bool>::new();
    for trace in &traces {
        *drawn.entry(trace_signal(trace)).or_insert(true) &= trace.visible;
    }
    let entry = |waveform: &crate::state::RetainedTrace| {
        let plotted = drawn.get(waveform.name.as_str()).copied().unwrap_or(false);
        (waveform.name.clone(), waveform.visible, plotted)
    };
    // Only a signal the pane draws or the dataset plots can be plotted
    // other than where its dataset put it, so with the analysis indexed
    // only those columns are read, not a million.
    let mut visibility: Vec<(String, bool, bool)> =
        match results
            .session
            .signal_indices
            .index(runs, key, &analysis.data)
        {
            Some(index) => {
                let mut columns = index.plotted().to_vec();
                for name in drawn.keys() {
                    columns.extend(index.columns_named(&analysis.data, name));
                }
                columns.sort_unstable();
                columns.dedup();
                columns
                    .into_iter()
                    .map(|column| entry(&analysis.waveforms[column]))
                    .collect()
            }
            None => analysis.waveforms.iter().map(entry).collect(),
        };
    if let Some(names) = names {
        let listed = lanes::plotted_history_nets(document, pane_id, names);
        for name in names.history_only() {
            let plotted = listed.as_ref().map_or_else(
                || {
                    results.session.waveform_visibility(
                        &SourceWaveformPresentationKey::new(key, name.clone()),
                        false,
                    )
                },
                |listed| listed.contains(name),
            );
            visibility.push((name.clone(), false, plotted));
        }
    }
    results.project_waveform_visibility(key, visibility);
    results.project_waveform_colors(
        key,
        traces
            .iter()
            .filter_map(|trace| Some((trace_signal(trace).to_owned(), trace.color?))),
    );
    results.project_waveform_looks(
        key,
        traces
            .iter()
            .map(|trace| super::trace_style::look_of(trace)),
    );
}

/// Draw `signals` of pane `pane_id` as the view holds them, as
/// [`project_traces`] draws every signal of the pane: whether each is
/// shown, its colour and its look, read from its own traces alone.
fn project_signals(
    results: &mut super::ResultsState,
    runs: &crate::state::RunHistoryRevision,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
    analysis: &crate::state::AnalysisResult,
    signals: &HashSet<String>,
) {
    if signals.is_empty() {
        return;
    }
    let index = results
        .session
        .signal_indices
        .index(runs, key, &analysis.data);
    for signal in signals {
        let traces = document.traces_reading(pane_id, signal);
        let column = match &index {
            Some(index) => index.columns_named(&analysis.data, signal).next(),
            None => analysis
                .waveforms
                .iter()
                .position(|waveform| waveform.name == *signal),
        };
        let presentation = SourceWaveformPresentationKey::new(key, signal.clone());
        if let Some(column) = column {
            let dataset_default = analysis.waveforms[column].visible;
            let plotted = !traces.is_empty() && traces.iter().all(|trace| trace.visible);
            results.project_waveform_visible(
                presentation.clone(),
                (plotted != dataset_default).then_some(plotted),
            );
        }
        let color = traces.iter().rev().find_map(|trace| trace.color);
        results.draw_waveform_color(presentation.clone(), color);
        let look = traces
            .iter()
            .rev()
            .map(|trace| super::trace_style::look_of(trace).1)
            .find(|look| *look != rspice_results_ui::session::WaveformLook::default())
            .unwrap_or_default();
        results.draw_waveform_look(presentation, look);
    }
}

/// Whether `signal` names an event net, the `D()` or `E()` column or
/// history of one, which a pane's event lanes may list.
fn is_event_net(signal: &str) -> bool {
    signal.starts_with("D(") || signal.starts_with("E(")
}

/// Draw the nets `names` keeps only as history on pane `pane_id` as the
/// view lists them; while the pane lists no lanes, the reader's choice
/// stands.
fn project_history_nets(
    results: &mut super::ResultsState,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
    names: &lanes::EventNames,
) {
    let Some(listed) = lanes::plotted_history_nets(document, pane_id, names) else {
        return;
    };
    for name in names.history_only() {
        results.project_waveform_visible(
            SourceWaveformPresentationKey::new(key, name.clone()),
            listed.contains(name).then_some(true),
        );
    }
}

/// The drawing state of the strips `keys` name, besides the ticks.
fn scratch_of(state: ResultsReads<'_>, keys: &[(AnalysisPresentationKey, PaneId)]) -> ViewScratch {
    let session = &state.results.session;
    let shown = |analysis: &AnalysisPresentationKey| keys.iter().any(|(key, _)| key == analysis);
    ViewScratch {
        expressions: session
            .analysis_exprs
            .iter()
            .filter(|(key, _)| shown(key))
            .map(|(key, traces)| (*key, traces.clone()))
            .collect(),
        units: session
            .expression_units
            .iter()
            .filter(|((key, _), _)| shown(key))
            .map(|(key, unit)| (key.clone(), unit.clone()))
            .collect(),
        x_scales: session
            .x_scales
            .iter()
            .filter(|(key, _)| shown(key))
            .map(|(key, scale)| (*key, *scale))
            .collect(),
        xy_strips: session
            .xy_strips
            .iter()
            .filter(|strip| shown(&strip.analysis))
            .cloned()
            .collect(),
        log_y: session
            .log_y_panes
            .iter()
            .filter(|pane| shown(&pane.analysis))
            .cloned()
            .collect(),
        hidden: session
            .hidden_strips
            .iter()
            .copied()
            .filter(|key| shown(key))
            .collect(),
        lanes: session
            .event_lanes
            .iter()
            .filter(|(key, _)| shown(key))
            .map(|(key, layout)| (*key, layout.clone()))
            .collect(),
        pins: readout_pins::scratch(&session.readout_pins, keys),
        settings: sheet_settings::scratch(&session.sheet_settings, keys),
        strips: session
            .strip_layouts
            .iter()
            .filter(|(key, _)| shown(key))
            .map(|(key, strips)| (*key, strips.clone()))
            .collect(),
        family: session
            .family_policies
            .iter()
            .filter(|(key, _)| shown(key))
            .map(|(key, policy)| (*key, policy.clone()))
            .collect(),
        maximized: session.maximized_strip.filter(|key| shown(key)),
    }
}

/// Whether the drawing state of `binding`'s strips, besides the ticks, is
/// still what the quick view left: compared where it stands, copying none.
fn scratch_holds(state: ResultsReads<'_>, binding: &QuickViewBinding) -> bool {
    let session = &state.results.session;
    let scratch = &binding.scratch;
    let shown =
        |analysis: &AnalysisPresentationKey| binding.keys.iter().any(|(key, _)| key == analysis);
    same_entries(&session.analysis_exprs, &scratch.expressions, shown)
        && same_entries(&session.expression_units, &scratch.units, |(key, _)| {
            shown(key)
        })
        && same_entries(&session.x_scales, &scratch.x_scales, shown)
        && session
            .xy_strips
            .iter()
            .filter(|strip| shown(&strip.analysis))
            .eq(scratch.xy_strips.iter())
        && same_members(&session.log_y_panes, &scratch.log_y, |pane| {
            shown(&pane.analysis)
        })
        && same_members(&session.hidden_strips, &scratch.hidden, shown)
        && same_entries(&session.event_lanes, &scratch.lanes, shown)
        && readout_pins::scratch(&session.readout_pins, &binding.keys) == scratch.pins
        && sheet_settings::scratch(&session.sheet_settings, &binding.keys) == scratch.settings
        && same_entries(&session.strip_layouts, &scratch.strips, shown)
        && same_entries(&session.family_policies, &scratch.family, shown)
        && session.maximized_strip.filter(|key| shown(key)) == scratch.maximized
}

/// Whether the entries of `now` that `shown` picks are exactly `then`.
fn same_entries<K: Eq + std::hash::Hash, V: PartialEq>(
    now: &HashMap<K, V>,
    then: &HashMap<K, V>,
    shown: impl Fn(&K) -> bool,
) -> bool {
    let mut picked = 0;
    for (key, value) in now {
        if shown(key) {
            if then.get(key) != Some(value) {
                return false;
            }
            picked += 1;
        }
    }
    picked == then.len()
}

/// Whether the members of `now` that `shown` picks are exactly `then`.
fn same_members<K: Eq + std::hash::Hash>(
    now: &HashSet<K>,
    then: &HashSet<K>,
    shown: impl Fn(&K) -> bool,
) -> bool {
    let mut picked = 0;
    for key in now {
        if shown(key) {
            if !then.contains(key) {
                return false;
            }
            picked += 1;
        }
    }
    picked == then.len()
}

/// What the quick view drew that its view does not hold yet.
struct Captured {
    /// The edits that make the view hold it, but for its new traces.
    edits: Vec<DocumentEdit>,
    /// The signals each pane plots now that it has no trace of, in the
    /// order the run lists them.
    new: Vec<(PaneId, Vec<String>)>,
    /// With the reader's ticks known, each a signal of its analysis: those
    /// the quick view draws other than the view draws them.
    normalize: Option<Vec<Normal>>,
}

/// The edits that make `binding`'s view hold what the quick view drew.
///
/// With `ticks`, the reader set only those signals' ticks since the quick
/// view drew; without, any of them.
fn capture_edits(
    state: &ResultsParts<'_>,
    binding: &QuickViewBinding,
    ticks: Option<&[SourceWaveformPresentationKey]>,
) -> Captured {
    let mut normalize = ticks.map(|_| Vec::new());
    let mut all_new = Vec::new();
    let (Some(document), Some(run)) = (
        state
            .model
            .workspace
            .content
            .visualization_document(binding.view),
        state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .find(|run| run.dataset_id == binding.dataset),
    ) else {
        return Captured {
            edits: Vec::new(),
            new: all_new,
            normalize: None,
        };
    };
    let session = &state.results.session;
    let mut edits = Vec::new();
    for &(key, pane_id) in &binding.keys {
        let Some((_, analysis)) = key.resolve(run) else {
            continue;
        };
        let Some(pane) = document.panes().iter().find(|pane| pane.id == pane_id) else {
            continue;
        };
        // However many signals one gesture ticks or unticks, the view takes
        // them as one edit each way, in the order the run lists them. With
        // the signals the reader set known, only those are read.
        // A signal is drawn when every trace of it is shown; one with no
        // trace yet is new.
        let (mut ticked, mut unticked, mut new) = (Vec::new(), Vec::new(), Vec::new());
        // Whether the reader set any signal of the strip, or may have.
        let mut touched = ticks.is_none();
        let mut differ = |column: usize, name: &str, wanted: bool, drawn: Option<bool>| {
            let to = match (wanted, drawn) {
                (true, None) => &mut new,
                (true, Some(false)) => &mut ticked,
                (false, Some(true)) => &mut unticked,
                _ => return,
            };
            to.push((column, name.to_owned()));
        };
        match ticks {
            Some(ticks) => {
                let ticks: Vec<&SourceWaveformPresentationKey> =
                    ticks.iter().filter(|tick| tick.analysis() == key).collect();
                touched = !ticks.is_empty();
                let names: Vec<&str> = ticks.iter().map(|tick| tick.source_name()).collect();
                let columns = match session.signal_indices.index(
                    &state.model.simulation.retained.runs.revision(),
                    key,
                    &analysis.data,
                ) {
                    Some(index) => index.first_columns(&analysis.data, &names),
                    None => names
                        .iter()
                        .map(|name| {
                            analysis
                                .waveforms
                                .iter()
                                .position(|waveform| waveform.name == *name)
                        })
                        .collect(),
                };
                for ((tick, name), column) in ticks.into_iter().zip(names).zip(columns) {
                    let Some(column) = column else {
                        normalize = None;
                        continue;
                    };
                    let dataset_default = analysis.waveforms[column].visible;
                    let set = session.waveform_visibility.get(tick).copied();
                    let wanted = set.unwrap_or(dataset_default);
                    if let Some(normalize) = &mut normalize {
                        let drawn = (wanted != dataset_default).then_some(wanted);
                        if set != drawn {
                            normalize.push((tick.clone(), set, drawn));
                        }
                    }
                    differ(column, name, wanted, document.signal_drawn(pane_id, name));
                }
            }
            None => {
                // Each signal the pane draws, and whether every trace of it
                // is shown.
                let mut drawn = HashMap::<&str, bool>::new();
                for trace in document
                    .traces()
                    .iter()
                    .filter(|trace| trace.pane_id == pane_id)
                {
                    *drawn.entry(trace_signal(trace)).or_insert(true) &= trace.visible;
                }
                for (column, waveform) in analysis.waveforms.iter().enumerate() {
                    let wanted = session.waveform_visibility(
                        &SourceWaveformPresentationKey::new(key, waveform.name.clone()),
                        waveform.visible,
                    );
                    let drawn = drawn.get(waveform.name.as_str()).copied();
                    differ(column, &waveform.name, wanted, drawn);
                }
            }
        }
        for (mut signals, plotted) in [(ticked, true), (unticked, false)] {
            if !signals.is_empty() {
                signals.sort_unstable_by_key(|(column, _)| *column);
                signals.dedup_by_key(|(column, _)| *column);
                edits.push(DocumentEdit::SetSignalsPlotted {
                    pane_id,
                    signals: signals.into_iter().map(|(_, name)| name).collect(),
                    plotted,
                });
            }
        }
        if !new.is_empty() {
            new.sort_unstable_by_key(|(column, _)| *column);
            new.dedup_by_key(|(column, _)| *column);
            all_new.push((pane_id, new.into_iter().map(|(_, name)| name).collect()));
        }
        // The lanes are taken where the reader changed them, or set a
        // signal they may list.
        let layout = session.event_lanes.get(&key);
        if (touched || layout != binding.scratch.lanes.get(&key))
            && lanes::has_event_lanes(analysis)
            && let Some(layout) = layout
            && let Some(names) = lanes::EventNames::of(
                analysis,
                session
                    .signal_indices
                    .index(
                        &state.model.simulation.retained.runs.revision(),
                        key,
                        &analysis.data,
                    )
                    .as_deref(),
            )
        {
            let plotted: Vec<&str> = names
                .history_only()
                .iter()
                .filter(|name| {
                    session.waveform_visibility(
                        &SourceWaveformPresentationKey::new(key, (*name).clone()),
                        false,
                    )
                })
                .map(String::as_str)
                .collect();
            edits.extend(lanes::lane_edits(
                document, pane_id, &names, layout, &plotted,
            ));
        }
        if let Some(dataset) = pane.binding.map(|binding| binding.dataset) {
            edits.extend(super::persistent_document::expression_edits_for(
                state, document, pane_id, dataset, run, key,
            ));
        }
        edits.extend(super::persistent_document::x_scale_edit_for(
            state, document, pane_id, key,
        ));
        edits.extend(super::persistent_document::xy_plot_edit_for(
            state, document, pane_id, key,
        ));
        edits.extend(log_axis_edits(state, document, pane_id, key));
        let hidden = session.hidden_strips.contains(&key);
        if hidden != pane.hidden {
            edits.push(DocumentEdit::SetPaneHidden { pane_id, hidden });
        }
        edits.extend(readout_pins::edits_for(
            &session.readout_pins,
            document,
            key,
            pane_id,
        ));
        edits.extend(sheet_settings::edits_for(
            &session.sheet_settings,
            document,
            key,
            pane_id,
        ));
        let strips = session
            .strip_layouts
            .get(&key)
            .map_or(&[][..], Vec::as_slice);
        if strips != pane.strips.as_slice() {
            edits.push(DocumentEdit::SetPaneStrips {
                pane_id,
                strips: strips.to_vec(),
            });
        }
        let policy = session.family_policies.get(&key);
        if policy != pane.family_policy.as_ref() {
            edits.push(DocumentEdit::SetPaneFamilyPresentation {
                pane_id,
                policy: policy.cloned(),
            });
        }
        let maximized = session.maximized_strip == Some(key);
        if maximized != pane.maximized {
            edits.push(DocumentEdit::SetPaneMaximized { pane_id, maximized });
        }
    }
    Captured {
        edits,
        new: all_new,
        normalize,
    }
}

/// The unit a pane's axis keeps for a unit pane of the waveform stack: a
/// plain number's pane, which has no unit, keeps the dimensionless `1`.
fn axis_unit(pane_unit: &str) -> &str {
    if pane_unit.is_empty() { "1" } else { pane_unit }
}

/// The unit pane of the waveform stack an axis unit names.
fn pane_unit(axis_unit: &str) -> &str {
    if axis_unit == "1" { "" } else { axis_unit }
}

/// The edits that put exactly the unit panes the reader set logarithmic on
/// log axes of the pane.
fn log_axis_edits(
    state: &ResultsParts<'_>,
    document: &VisualizationDocument,
    pane_id: PaneId,
    key: AnalysisPresentationKey,
) -> Vec<DocumentEdit> {
    let wanted: HashSet<&str> = state
        .results
        .session
        .log_y_panes
        .iter()
        .filter(|pane| pane.analysis == key)
        .map(|pane| axis_unit(&pane.unit))
        .collect();
    let unit_axes: Vec<_> = document
        .axes()
        .iter()
        .filter(|axis| {
            axis.pane_id == pane_id
                && axis.orientation == AxisOrientation::VerticalLeft
                && axis.unit.is_some()
        })
        .collect();
    let mut edits = Vec::new();
    for axis in &unit_axes {
        let log = wanted.contains(axis.unit.as_deref().unwrap_or_default());
        let scale = if log {
            AxisScale::Logarithmic
        } else {
            AxisScale::Linear
        };
        if axis.scale != scale {
            edits.push(DocumentEdit::SetAxisScale {
                axis_id: axis.id,
                scale,
            });
        }
    }
    for unit in wanted {
        if !unit_axes
            .iter()
            .any(|axis| axis.unit.as_deref() == Some(unit))
        {
            edits.push(DocumentEdit::AddAxis(NewAxis {
                pane_id,
                label: unit.to_owned(),
                orientation: AxisOrientation::VerticalLeft,
                scale: AxisScale::Logarithmic,
                unit: Some(unit.to_owned()),
                range: None,
            }));
        }
    }
    edits
}
