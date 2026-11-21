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
    AxisOrientation, AxisScale, DocumentEdit, NewAxis, PaneId, ResultDocumentTracking,
    ResultDocumentTrackingMode, SourceDataset, TypedValue, VisualizationDocument, WorkingPane,
    WorkingViewSource,
};
use crate::state::SimulationRun;
use crate::ui::plot::XScale;

use super::persistent_document::lanes;
use super::{
    AnalysisPresentationKey, DocumentMarker, ExprTrace, ResultsParts, ResultsReads,
    SourceWaveformPresentationKey, WavePanePresentationKey, view_edits,
};

/// The drawing state of one run's analyses, as the quick view last took it
/// from the view or wrote it back. A difference from it is the reader's.
#[derive(Debug, Clone, Default, PartialEq)]
struct ViewScratch {
    visibility: HashMap<SourceWaveformPresentationKey, bool>,
    expressions: HashMap<AnalysisPresentationKey, Vec<ExprTrace>>,
    units: HashMap<(AnalysisPresentationKey, String), String>,
    x_scales: HashMap<AnalysisPresentationKey, XScale>,
    xy_strips: Vec<XyStrip>,
    log_y: HashSet<WavePanePresentationKey>,
    hidden: HashSet<AnalysisPresentationKey>,
    lanes: HashMap<AnalysisPresentationKey, EventLaneLayout>,
}

/// The view the quick view last drew, the run it drew it over, and the
/// drawing state it left.
#[derive(Debug, Clone)]
pub(crate) struct QuickViewBinding {
    view: ResultDocumentId,
    dataset: DatasetId,
    keys: Vec<(AnalysisPresentationKey, PaneId)>,
    scratch: ViewScratch,
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

/// Before the quick view draws: keep what the reader changed since it last
/// drew, make or follow the shown run's view, and draw from it.
pub fn sync(state: &mut ResultsParts<'_>) {
    capture(state);
    prune(state);
    let Some(run) = state.model.simulation.active_run().cloned() else {
        state.results.session.working_view_panes.clear();
        return;
    };
    let Some(view) = ensure(state, &run) else {
        state.results.session.working_view_panes.clear();
        return;
    };
    follow_a_new_run(state, view, &run);
    follow(state, view, &run);
    reread_grown(state, view, &run);
    plot_marked_outputs(state, view, &run);
    project(state, view, &run);
}

/// After the quick view drew, or before anything else draws over its state:
/// write what the reader changed back to the view it drew.
pub fn capture(state: &mut ResultsParts<'_>) {
    let Some(mut binding) = state.results.working_view.take() else {
        return;
    };
    let now = scratch_of(state.results_reads(), &binding.keys);
    if now != binding.scratch {
        let edits = capture_edits(state, &binding);
        if !edits.is_empty()
            && let Some(revision) = state
                .model
                .workspace
                .content
                .visualization_document(binding.view)
                .map(VisualizationDocument::revision)
            && let Err(error) = view_edits::apply(state, binding.view, revision, edits)
        {
            state.prompt(format!("Could not keep this change to the view: {error}"));
        }
        binding.scratch = scratch_of(state.results_reads(), &binding.keys);
    }
    state.results.working_view = Some(binding);
}

/// Forget the drawing state the quick view left: something else, a saved
/// document's pane, is about to draw over it.
pub(crate) fn release(state: &mut ResultsParts<'_>) {
    capture(state);
    state.results.working_view = None;
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
        for waveform in run.analyses[index]
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible)
        {
            let drawn = document
                .traces()
                .iter()
                .any(|trace| trace.pane_id == pane_id && trace_signal(trace) == waveform.name);
            if !drawn {
                edits.push(DocumentEdit::PlotSignal {
                    pane_id,
                    signal: waveform.name.clone(),
                });
            }
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

/// `run` as a view reads it: by identity and digest, with the signals of
/// every analysis it solved.
fn run_reference(state: ResultsReads<'_>, run: &SimulationRun) -> Option<SourceDataset> {
    let analyses = run
        .analyses
        .iter()
        .map(|analysis| super::create_document::run_source_analysis(run, analysis))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
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
    // while still running, and has signals now that it had not then, or it
    // finished with samples it had not then.
    let grown = reads_run
        && document
            .datasets()
            .iter()
            .find(|dataset| dataset.binding().dataset_id == run.dataset_id)
            .is_some_and(|held| {
                let signals_changed = held.run_source().is_none_or(|source| {
                    run.analyses.iter().any(|analysis| {
                        source
                            .analysis(super::create_document::source_analysis_identity(
                                run, analysis,
                            ))
                            .is_some_and(|read| {
                                !read
                                    .signals()
                                    .iter()
                                    .map(|signal| signal.name())
                                    .eq(analysis.waveforms.iter().map(|w| w.name.as_str()))
                            })
                    })
                });
                signals_changed
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

/// Draw `run` from `view`.
fn project(state: &mut ResultsParts<'_>, view: ResultDocumentId, run: &SimulationRun) {
    let Some(document) = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .cloned()
    else {
        return;
    };
    let panes = bound_panes(&document, run);
    let mut markers = Vec::new();
    for &(index, key, pane_id) in &panes {
        let analysis = &run.analyses[index];
        let Some(pane) = document.panes().iter().find(|pane| pane.id == pane_id) else {
            continue;
        };
        let traces: Vec<_> = document
            .traces()
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .collect();
        let names = lanes::EventNames::of(analysis);
        let mut visibility: Vec<(String, bool, bool)> = analysis
            .waveforms
            .iter()
            .map(|waveform| {
                let mut held = traces
                    .iter()
                    .filter(|trace| trace_signal(trace) == waveform.name)
                    .peekable();
                let plotted = held.peek().is_some() && held.all(|trace| trace.visible);
                (waveform.name.clone(), waveform.visible, plotted)
            })
            .collect();
        if let Some(names) = names.as_ref() {
            let listed = lanes::plotted_history_nets(&document, pane_id, names);
            for name in names.history_only() {
                let plotted = listed.as_ref().map_or_else(
                    || {
                        state.results.session.waveform_visibility(
                            &SourceWaveformPresentationKey::new(key, name.clone()),
                            false,
                        )
                    },
                    |listed| listed.contains(name),
                );
                visibility.push((name.clone(), false, plotted));
            }
        }
        state.results.project_waveform_visibility(key, visibility);
        state.results.project_waveform_colors(
            key,
            traces
                .iter()
                .filter_map(|trace| Some((trace_signal(trace).to_owned(), trace.color?))),
        );

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
                lanes::projected_layout(&document, pane_id, &names, session.event_lanes.get(&key));
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

        for marker in document
            .markers()
            .iter()
            .filter(|marker| marker.pane_id == pane_id)
        {
            let (Some(trace), TypedValue::Real(x)) = (
                traces.iter().find(|trace| trace.id == marker.trace_id),
                &marker.coordinate,
            ) else {
                continue;
            };
            // A marker on a trace that is not drawn has nothing to sit on.
            let Some((analysis, anchor)) =
                super::waves::source_waveform_anchor_in(state, index, trace_signal(trace))
            else {
                continue;
            };
            markers.push(DocumentMarker {
                document_id: view,
                pane_id,
                retained_id: marker.id,
                analysis,
                anchor,
                trace_name: trace.label.clone(),
                x: *x,
                kind: super::marker_kind_of_retained(marker.kind),
                note: super::marker_note(&marker.label, &trace.label),
            });
        }
    }
    let session = &mut state.results.session;
    session.project_document_markers(markers);
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
        dataset: run.dataset_id,
        keys,
        scratch,
    }));
}

/// The drawing state of the strips `keys` name.
fn scratch_of(state: ResultsReads<'_>, keys: &[(AnalysisPresentationKey, PaneId)]) -> ViewScratch {
    let session = &state.results.session;
    let shown = |analysis: &AnalysisPresentationKey| keys.iter().any(|(key, _)| key == analysis);
    ViewScratch {
        visibility: session
            .waveform_visibility
            .iter()
            .filter(|(key, _)| shown(&key.analysis()))
            .map(|(key, visible)| (key.clone(), *visible))
            .collect(),
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
    }
}

/// The edits that make `binding`'s view hold what the quick view drew.
fn capture_edits(state: &ResultsParts<'_>, binding: &QuickViewBinding) -> Vec<DocumentEdit> {
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
        return Vec::new();
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
        let traces: Vec<_> = document
            .traces()
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .collect();
        for waveform in &analysis.waveforms {
            let wanted = session.waveform_visibility(
                &SourceWaveformPresentationKey::new(key, waveform.name.clone()),
                waveform.visible,
            );
            let held: Vec<_> = traces
                .iter()
                .filter(|trace| trace_signal(trace) == waveform.name)
                .collect();
            let plotted = !held.is_empty() && held.iter().all(|trace| trace.visible);
            if wanted == plotted {
                continue;
            }
            if wanted {
                edits.push(DocumentEdit::PlotSignal {
                    pane_id,
                    signal: waveform.name.clone(),
                });
            } else {
                edits.extend(held.iter().filter(|trace| trace.visible).map(|trace| {
                    DocumentEdit::SetTraceVisibility {
                        trace_id: trace.id,
                        visible: false,
                    }
                }));
            }
        }
        if let Some(names) = lanes::EventNames::of(analysis)
            && let Some(layout) = session.event_lanes.get(&key)
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
    }
    edits
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
