//! New result document: a project-owned document over one analysis of a
//! retained run.
//!
//! The form opens on the run, analysis, sheet and signals Results shows.
//! Every choice is resolved again when Create is pressed; only a fully
//! validated [`VisualizationDocument`] enters the project-owned collection.
//! Solver samples and retained run history are never mutated.

use rspice_results::document_creation::ResultDocumentLayout;
use rspice_results_ui::create_document::{
    self, CreateDocumentChoices, CreateResultDocumentDialogState,
};
use rspice_ui_kit::widgets::DialogChoice;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::{
    AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId, ResultDocumentId,
};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerArt, ViewerDocumentDefinition, ViewerReleaseClass, viewer_document,
};
use crate::results::visualization_document::{
    AxisOrientation, AxisScale, DocumentEdit, EntityRef, LinkKind, NewPagePane, NewPane,
    PageUpdatePolicy, PaneDataBinding, PaneKind, PanePlacement, ResultDocumentTracking,
    ResultDocumentTrackingMode, RunSourceAnalysis, SourceDataset, VisualizationDocument,
    VisualizationError, WorkingViewSource,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle};
use crate::workbench::state::Workspace;

use super::{ResultViewer, ResultsParts, ResultsReads};

const MAX_DOCUMENT_NAME_BYTES: usize = 120;

#[derive(Debug, thiserror::Error)]
pub enum CreateResultDocumentError {
    #[error("{0}")]
    InvalidDraft(String),
    #[error(transparent)]
    Visualization(#[from] VisualizationError),
    #[error(transparent)]
    Persistence(#[from] VisualizationDocumentPersistenceError),
}

#[derive(Debug, Clone)]
pub struct ResolvedDraft<'a> {
    name: &'a str,
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
    pub viewer: &'static ViewerDocumentDefinition,
    layout: ResultDocumentLayout,
    /// What each pane plots, by signal name.
    plotted: Vec<String>,
}

/// Open the form on what Results shows: its run, the analysis the data
/// browser picked, the sheet on the canvas and the signals it plots.
pub fn open(state: &mut ResultsParts<'_>) {
    let reads = state.results_reads();
    let simulation = &reads.model.simulation;
    let shown = simulation.active_run().filter(|run| finished(run));
    // The history lists its newest run first.
    let run = shown.or_else(|| simulation.retained.runs.iter().find(|run| finished(run)));
    let analysis = run.and_then(|run| {
        let picked = shown
            .filter(|shown| shown.dataset_id == run.dataset_id)
            .and(simulation.view.active_analysis_idx)
            .filter(|index| *index < run.analyses.len());
        picked.or_else(|| (!run.analyses.is_empty()).then_some(0))
    });
    let mut draft = CreateResultDocumentDialogState {
        open: true,
        dataset_id: run.map(|run| run.dataset_id),
        analysis,
        ..CreateResultDocumentDialogState::default()
    };
    let shown_sheet = reads.results.session.viewer.viewer_document_id();
    settle(reads, &mut draft, Some(shown_sheet));
    *state.create_result_document = draft;
}

/// Draw the form and carry out what the reader asked of it.
pub(crate) fn show(ctx: &egui::Context, state: &mut ResultsParts<'_>) {
    if !state.create_result_document.open {
        return;
    }
    let mut draft = state.create_result_document.clone();
    let before = draft.clone();
    let offered = choices(state.results_reads(), &draft);
    let choice = create_document::show(ctx, &mut draft, &offered);
    reconcile(state.results_reads(), &before, &mut draft);
    *state.create_result_document = draft;
    match choice {
        DialogChoice::Primary => {
            let name = state.create_result_document.name.trim().to_owned();
            match commit(state) {
                Ok(_) => {
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!("Created result document {name}."),
                    );
                }
                Err(error) => {
                    state.create_result_document.validation_error = Some(error.to_string());
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            *state.create_result_document = CreateResultDocumentDialogState::default();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// What the form offers for `draft`, and why Create is refused, if it is.
pub fn choices(
    state: ResultsReads<'_>,
    draft: &CreateResultDocumentDialogState,
) -> CreateDocumentChoices {
    let runs = state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| finished(run))
        .map(|run| (run.dataset_id, run.name().to_owned()))
        .collect();
    let run = draft.dataset_id.and_then(|id| retained_run(state, id));
    let analyses = run.map_or_else(Vec::new, |run| {
        run.analyses
            .iter()
            .enumerate()
            .map(|(index, analysis)| (index, analysis_name(analysis)))
            .collect()
    });
    let analysis = run
        .zip(draft.analysis)
        .and_then(|(run, index)| run.analyses.get(index));
    let viewers = analysis.map_or_else(Vec::new, |analysis| {
        sheets_of(analysis)
            .map(|viewer| (viewer.id, viewer.title.to_owned()))
            .collect()
    });
    let signals = analysis
        .zip(viewer_document(&draft.viewer_id))
        .filter(|(_, viewer)| plots_signals(viewer))
        .map_or_else(Vec::new, |(analysis, _)| signal_names(analysis));
    CreateDocumentChoices {
        runs,
        analyses,
        viewers,
        signals,
        refusal: resolve_draft(state, draft)
            .err()
            .map(|error| error.to_string()),
    }
}

/// Follow a change the reader made: another run opens on its analysis of the
/// same kind, another analysis on a sheet that draws it, and either on the
/// signals it plots. A name the reader did not type follows them.
pub fn reconcile(
    state: ResultsReads<'_>,
    before: &CreateResultDocumentDialogState,
    draft: &mut CreateResultDocumentDialogState,
) {
    if draft.dataset_id != before.dataset_id {
        let kind = before
            .dataset_id
            .and_then(|id| retained_run(state, id))
            .zip(before.analysis)
            .and_then(|(run, index)| run.analyses.get(index))
            .map(|analysis| analysis.analysis_type);
        draft.analysis = draft
            .dataset_id
            .and_then(|id| retained_run(state, id))
            .and_then(|run| {
                kind.and_then(|kind| {
                    run.analyses
                        .iter()
                        .position(|analysis| analysis.analysis_type == kind)
                })
                .or_else(|| (!run.analyses.is_empty()).then_some(0))
            });
    }
    if draft.dataset_id != before.dataset_id || draft.analysis != before.analysis {
        let kept = before.viewer_id.clone();
        draft.signals.clear();
        settle(state, draft, viewer_document(&kept).map(|viewer| viewer.id));
    } else if draft.viewer_id != before.viewer_id && draft.signals.is_empty() {
        draft.signals = default_signals(state, draft);
    }
}

/// Give `draft` the sheet `preferred` when it draws the chosen analysis, else
/// the analysis' own, its default signals, and a name if the reader typed
/// none.
fn settle(
    state: ResultsReads<'_>,
    draft: &mut CreateResultDocumentDialogState,
    preferred: Option<&str>,
) {
    let analysis = chosen_analysis(state, draft);
    draft.viewer_id = analysis
        .and_then(|(_, analysis)| {
            preferred
                .and_then(|id| sheets_of(analysis).find(|viewer| viewer.id == id))
                .or_else(|| view_pane_viewer(analysis))
        })
        .map_or_else(String::new, |viewer| viewer.id.to_owned());
    draft.signals = default_signals(state, draft);
    if !draft.name_touched {
        draft.name = analysis.map_or_else(String::new, |(run, analysis)| {
            default_name(state, run, analysis)
        });
    }
    draft.validation_error = None;
}

fn chosen_analysis<'a>(
    state: ResultsReads<'a>,
    draft: &CreateResultDocumentDialogState,
) -> Option<(&'a SimulationRun, &'a AnalysisResult)> {
    let run = retained_run(state, draft.dataset_id?)?;
    Some((run, run.analyses.get(draft.analysis?)?))
}

/// What a new pane over the chosen analysis plots: what Results plots of it
/// now, else what its run first plots.
fn default_signals(
    state: ResultsReads<'_>,
    draft: &CreateResultDocumentDialogState,
) -> Vec<String> {
    let Some((run, analysis)) = chosen_analysis(state, draft) else {
        return Vec::new();
    };
    if !viewer_document(&draft.viewer_id).is_some_and(plots_signals) {
        return Vec::new();
    }
    super::working_view::shown_signals(state, run, analysis)
}

/// "<plan or run> — <analysis>", or that and the first number after it that
/// no document of the project is titled.
fn default_name(state: ResultsReads<'_>, run: &SimulationRun, analysis: &AnalysisResult) -> String {
    let source = super::working_view::source_of(run);
    let owner = match source {
        WorkingViewSource::Plan { .. } => {
            crate::workbench::chrome::document_bar::results::view_title(state.model, source, run)
        }
        WorkingViewSource::Imported { .. } => run.name().to_owned(),
    };
    let base = format!("{owner} \u{2014} {}", analysis_name(analysis));
    let taken = |title: &str| title_taken(state, title);
    if !taken(&base) {
        return base;
    }
    (2..)
        .map(|number| format!("{base} {number}"))
        .find(|title| !taken(title))
        .unwrap_or(base)
}

fn title_taken(state: ResultsReads<'_>, title: &str) -> bool {
    state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .any(|document| document.title().eq_ignore_ascii_case(title))
}

/// What the data browser calls `analysis`.
fn analysis_name(analysis: &AnalysisResult) -> String {
    crate::workbench::docks::navigator::results::analysis_label(analysis)
}

/// A run a document can read: one that has stopped and kept an analysis.
fn finished(run: &SimulationRun) -> bool {
    !matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    ) && !run.analyses.is_empty()
}

/// The sheets this build draws `analysis` on, in catalogue order.
fn sheets_of(analysis: &AnalysisResult) -> impl Iterator<Item = &'static ViewerDocumentDefinition> {
    VIEWER_DOCUMENTS
        .iter()
        .filter(|viewer| draws(viewer, analysis))
}

fn draws(viewer: &ViewerDocumentDefinition, analysis: &AnalysisResult) -> bool {
    viewer.release == ViewerReleaseClass::ReleaseTarget
        && analysis_matches_viewer(viewer, analysis.analysis_type)
        && super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
}

/// Whether a pane of `viewer` draws each signal the reader ticks as a trace
/// of its own. A table or chart of the analysis as a whole draws no ticked
/// signal, and the spectrum and eye sheets transform the one source their
/// own bar picks.
fn plots_signals(viewer: &ViewerDocumentDefinition) -> bool {
    matches!(
        viewer.art,
        ViewerArt::Wave | ViewerArt::Bode | ViewerArt::Phase | ViewerArt::Smith | ViewerArt::Polar
    )
}

fn signal_names(analysis: &AnalysisResult) -> Vec<String> {
    analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.clone())
        .collect()
}

fn retained_run(state: ResultsReads<'_>, dataset_id: DatasetId) -> Option<&SimulationRun> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
}

pub(super) fn analysis_matches_viewer(
    viewer: &ViewerDocumentDefinition,
    analysis: AnalysisType,
) -> bool {
    viewer.analysis_ids.is_empty()
        || viewer.analysis_ids.contains(&analysis_manifest_id(analysis))
        // TEMP is the canonical tabular temperature-sweep authority. RSpice's
        // current typed producer retains it as a Corner family analysis, so it
        // carries both identities instead of silently making the release-target
        // table impossible to create.
        || (analysis == AnalysisType::Corner && viewer.analysis_ids.contains(&"temp"))
}

pub(super) const fn analysis_manifest_id(analysis: AnalysisType) -> &'static str {
    match analysis {
        AnalysisType::DcOp => "op",
        AnalysisType::DcSweep | AnalysisType::Parametric => "dc",
        AnalysisType::Ac => "ac",
        AnalysisType::Disto => "disto",
        AnalysisType::Transient => "tran",
        AnalysisType::Noise => "noise",
        AnalysisType::PoleZero => "pz",
        AnalysisType::Tf => "xf",
        AnalysisType::Sensitivity => "sens",
        AnalysisType::Pac => "pac",
        AnalysisType::Pnoise => "pnoise",
        AnalysisType::Pxf => "pxf",
        AnalysisType::Pstb => "pstb",
        AnalysisType::Stb => "stb",
        AnalysisType::MonteCarlo => "mc",
        AnalysisType::Corner => "corner",
        AnalysisType::Optimization => "opt",
        AnalysisType::Soa => "soa",
        AnalysisType::SParameter => "sp",
        AnalysisType::Envelope => "envelope",
        AnalysisType::Fourier => "fourier",
        AnalysisType::HarmonicBalance => "hb",
        AnalysisType::Pss => "pss",
        AnalysisType::Qpss => "qpss",
        AnalysisType::Hbsp => "hbsp",
        AnalysisType::Hbnoise => "hbnoise",
        AnalysisType::Psp => "psp",
        AnalysisType::Qpac => "qpac",
        AnalysisType::Qpnoise => "qpnoise",
        AnalysisType::Qpxf => "qpxf",
        AnalysisType::TransientNoise => "tnoise",
        AnalysisType::DcMismatch => "dcmatch",
    }
}

/// Resolve the draft against the retained runs, or say why it cannot be
/// created.
pub fn resolve_draft<'a>(
    state: ResultsReads<'a>,
    draft: &'a CreateResultDocumentDialogState,
) -> Result<ResolvedDraft<'a>, CreateResultDocumentError> {
    let invalid = |message: String| Err(CreateResultDocumentError::InvalidDraft(message));
    let name = draft.name.as_str();
    if name.trim().is_empty() {
        return invalid("Enter a name.".to_owned());
    }
    if name != name.trim() {
        return invalid("A name cannot begin or end with a space.".to_owned());
    }
    if name.len() > MAX_DOCUMENT_NAME_BYTES {
        return invalid(format!(
            "A name is at most {MAX_DOCUMENT_NAME_BYTES} bytes long."
        ));
    }
    if name.chars().any(char::is_control) {
        return invalid("A name cannot hold a control character.".to_owned());
    }
    if title_taken(state, name) {
        return invalid(format!(
            "A document named \u{201c}{name}\u{201d} already exists."
        ));
    }
    let Some(run) = draft.dataset_id.and_then(|id| retained_run(state, id)) else {
        return invalid("Choose a run.".to_owned());
    };
    if !finished(run) {
        return invalid(format!("{} has not finished.", run.name()));
    }
    let Some(analysis) = draft.analysis.and_then(|index| run.analyses.get(index)) else {
        return invalid("Choose an analysis.".to_owned());
    };
    let Some(viewer) = viewer_document(&draft.viewer_id).filter(|viewer| draws(viewer, analysis))
    else {
        return invalid(if sheets_of(analysis).next().is_none() {
            format!("No sheet draws {}.", analysis_name(analysis))
        } else {
            "Choose a sheet.".to_owned()
        });
    };
    // No tick gives an empty pane the reader plots into, as Add pane does.
    let plotted = if plots_signals(viewer) {
        let names = signal_names(analysis);
        draft
            .signals
            .iter()
            .filter(|signal| names.contains(signal))
            .cloned()
            .collect()
    } else {
        deck_plotted(analysis)
    };
    Ok(ResolvedDraft {
        name,
        run,
        analysis,
        viewer,
        layout: draft.layout,
        plotted,
    })
}

/// The form over the first analysis of the run `dataset_id` names that the
/// sheet `viewer_id` draws, plotting what Results plots of it, under a name
/// the reader typed.
#[cfg(any(test, feature = "test-support"))]
pub fn draft_over(
    state: ResultsReads<'_>,
    name: String,
    dataset_id: Option<DatasetId>,
    viewer_id: String,
    layout: ResultDocumentLayout,
) -> CreateResultDocumentDialogState {
    let run = dataset_id.and_then(|id| retained_run(state, id));
    let analysis = run.and_then(|run| {
        let viewer = viewer_document(&viewer_id)?;
        run.analyses
            .iter()
            .position(|analysis| draws(viewer, analysis))
    });
    let mut draft = CreateResultDocumentDialogState {
        open: true,
        name,
        name_touched: true,
        dataset_id,
        analysis,
        viewer_id,
        layout,
        ..CreateResultDocumentDialogState::default()
    };
    draft.signals = default_signals(state, &draft);
    draft
}

pub fn source_analysis_identity(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> AnalysisInstanceId {
    analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
        },
        |provenance| provenance.source_instance_id(),
    )
}

/// How a run reference names its run, for the reader once the run is gone.
pub(super) fn reference_run_label(run: &SimulationRun) -> String {
    let label = run.label.trim();
    if label.is_empty() || label.chars().any(char::is_control) {
        format!("Run {}", run.id)
    } else {
        label.to_owned()
    }
}

/// The dataset a document reads one analysis of a retained run through: the
/// run by identity and content digest, and the analysis. The samples and the
/// list of signals stay in the run, so a document is no larger for a larger
/// run. `digest` is the run's content digest as
/// [`super::retained_dataset_digest`] keeps it: digesting a run reads every
/// sample it holds, so a document or pane made over it never digests it
/// again.
pub(super) fn source_reference(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    digest: ContentDigest,
) -> Result<SourceDataset, VisualizationError> {
    SourceDataset::run_reference(
        DatasetBinding::new(run.dataset_id, digest),
        reference_run_label(run),
        vec![run_source_analysis(run, analysis)],
    )
}

/// One analysis of a run as a run reference names it: its identity and its
/// kind.
pub(super) fn run_source_analysis(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> RunSourceAnalysis {
    RunSourceAnalysis::new(
        source_analysis_identity(run, analysis),
        analysis.analysis_type,
    )
}

/// The signals of `analysis` its deck or plan names to plot, by name: what a
/// new document's pane over it plots, as a run's own view does. A run that
/// names none gives a pane that plots nothing until the reader ticks a
/// signal; a pane never plots every signal of a run it was not told to.
pub(super) fn deck_plotted(analysis: &AnalysisResult) -> Vec<String> {
    analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .map(|waveform| waveform.name.clone())
        .collect()
}

pub(super) fn pane_kind(art: ViewerArt) -> PaneKind {
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        ViewerArt::Wave
        | ViewerArt::Bode
        | ViewerArt::Spectrum
        | ViewerArt::Phase
        | ViewerArt::Field
        | ViewerArt::Contour
        | ViewerArt::Wireless
        | ViewerArt::Scatter
        | ViewerArt::Eye
        | ViewerArt::Bathtub
        | ViewerArt::Margin
        | ViewerArt::DigitalEvents
        | ViewerArt::Soa
        | ViewerArt::Optimization
        | ViewerArt::PoleZero
        | ViewerArt::Thermal
        | ViewerArt::Mesh => PaneKind::Cartesian,
    }
}

fn build_document(
    resolved: ResolvedDraft<'_>,
    digest: ContentDigest,
    tracking: ResultDocumentTracking,
) -> Result<VisualizationDocument, VisualizationError> {
    let source = source_reference(resolved.run, resolved.analysis, digest)?;
    let binding = PaneDataBinding {
        analysis_id: source_analysis_identity(resolved.run, resolved.analysis),
        dataset: source.binding(),
    };
    let mut document = VisualizationDocument::new_with_initial_pane(
        resolved.name,
        vec![source],
        NewPagePane {
            title: resolved.viewer.title.to_owned(),
            kind: pane_kind(resolved.viewer.art),
            viewer_id: resolved.viewer.id.to_owned(),
            binding: Some(binding),
            plotted: resolved.plotted.clone(),
        },
    )?;
    let page_id = document.pages()[0].id;
    let template_id = document.pages()[0].template_id.clone();
    let first_pane_id = document.panes()[0].id;
    document.transact(
        document.revision(),
        vec![
            DocumentEdit::SetTracking(tracking),
            DocumentEdit::Rename {
                entity: EntityRef::Page(page_id),
                value: analysis_name(resolved.analysis),
            },
            DocumentEdit::SetPageComposition {
                page_id,
                layout: resolved.layout.page_layout(),
                template_id,
                update_policy: PageUpdatePolicy::RefreshLinkedFigures,
            },
            DocumentEdit::Rename {
                entity: EntityRef::Pane(first_pane_id),
                value: resolved.viewer.title.to_owned(),
            },
        ],
    )?;

    let mut pane_ids = vec![first_pane_id];
    for pane_index in 1..resolved.layout.pane_count() {
        let anchor = if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index == 2
        {
            first_pane_id
        } else {
            *pane_ids.last().expect("the initial pane always exists")
        };
        let placement =
            if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index % 2 == 1 {
                PanePlacement::RightOf {
                    anchor_pane_id: anchor,
                }
            } else {
                PanePlacement::Below {
                    anchor_pane_id: anchor,
                }
            };
        let receipt = document.transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: format!("{} {}", resolved.viewer.title, pane_index + 1),
                kind: pane_kind(resolved.viewer.art),
                viewer_id: resolved.viewer.id.to_owned(),
                binding: Some(binding),
                placement,
                plotted: resolved.plotted.clone(),
            })],
        )?;
        let pane_id = receipt
            .created
            .into_iter()
            .find_map(|entity| match entity {
                EntityRef::Pane(id) => Some(id),
                _ => None,
            })
            .expect("AddBoundPane transaction returns its stable pane identity");
        pane_ids.push(pane_id);
    }

    let horizontal_axes = pane_ids
        .iter()
        .filter_map(|pane_id| {
            document
                .axes()
                .iter()
                .find(|axis| {
                    axis.pane_id == *pane_id && axis.orientation == AxisOrientation::Horizontal
                })
                .map(|axis| EntityRef::Axis(axis.id))
        })
        .collect::<Vec<_>>();
    if resolved.layout == ResultDocumentLayout::TwoLinkedPanes {
        document.transact(
            document.revision(),
            vec![DocumentEdit::AddLinkGroup {
                label: "Linked horizontal viewport".to_owned(),
                kind: LinkKind::HorizontalViewport,
                members: horizontal_axes,
            }],
        )?;
    }
    Ok(document)
}

fn document_tracking(
    runs: &[SimulationRun],
    selected_run: &SimulationRun,
    selected_analysis: &AnalysisResult,
) -> ResultDocumentTracking {
    let identities = selected_run
        .prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .zip(
            selected_analysis
                .provenance()
                .map(|provenance| provenance.authored_source_instance_id()),
        );
    let Some((simulation_plan_id, authored_analysis_id)) = identities else {
        return ResultDocumentTracking::pinned();
    };
    let newest_for_plan = runs
        .iter()
        .find(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(simulation_plan_id)
        })
        .is_some_and(|run| run.dataset_id == selected_run.dataset_id);
    ResultDocumentTracking::for_plan(
        if newest_for_plan {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        },
        simulation_plan_id,
        authored_analysis_id,
    )
}

/// Revalidate and commit the current draft into the project authority.
pub fn commit(state: &mut ResultsParts<'_>) -> Result<ResultDocumentId, CreateResultDocumentError> {
    let draft = state.create_result_document.clone();
    let document_id = insert(state, &draft)?;
    *state.create_result_document = CreateResultDocumentDialogState::default();
    Ok(document_id)
}

/// The viewer a working view's pane for `analysis` names: the first sheet
/// released for its kind that draws it.
pub(super) fn view_pane_viewer(
    analysis: &AnalysisResult,
) -> Option<&'static ViewerDocumentDefinition> {
    let draws = |viewer: &&'static ViewerDocumentDefinition| {
        viewer.release == ViewerReleaseClass::ReleaseTarget
            && analysis_matches_viewer(viewer, analysis.analysis_type)
            && super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
    };
    // The analysis' default sheets first, as the quick view opens on them.
    ResultViewer::default_sheets(analysis.analysis_type)
        .iter()
        .map(|sheet| sheet.viewer_document_id())
        .filter_map(|id| VIEWER_DOCUMENTS.iter().find(|viewer| viewer.id == id))
        .find(draws)
        .or_else(|| VIEWER_DOCUMENTS.iter().find(draws))
}

/// Resolve `draft` and insert the document it describes into the project,
/// shown in Results.
fn insert(
    state: &mut ResultsParts<'_>,
    draft: &CreateResultDocumentDialogState,
) -> Result<ResultDocumentId, CreateResultDocumentError> {
    let resolved = resolve_draft(state.results_reads(), draft)?;
    let tracking = document_tracking(
        &state.model.simulation.retained.runs,
        resolved.run,
        resolved.analysis,
    );
    // The strip's expressions, with the units declared for them, are part of
    // the view being saved; each pane reading the analysis keeps them.
    let analysis_key =
        super::AnalysisPresentationKey::new(resolved.run.dataset_id, resolved.analysis);
    let session = &state.results.session;
    let expressions: Vec<(String, Option<String>, bool)> = session
        .analysis_exprs
        .get(&analysis_key)
        .into_iter()
        .flatten()
        .map(|expr| {
            (
                expr.text.clone(),
                session
                    .expression_units
                    .get(&(analysis_key, expr.text.clone()))
                    .cloned(),
                expr.visible,
            )
        })
        .collect();
    // So is the X scale the reader chose for the strip.
    let x_scale = session.x_scales.get(&analysis_key).map(|scale| {
        if *scale == crate::ui::plot::XScale::Log10 {
            AxisScale::Logarithmic
        } else {
            AxisScale::Linear
        }
    });
    // And the strips that plot its panes against a signal, in their order.
    let xy_strips: Vec<rspice_results_ui::session::XyStrip> = session
        .xy_strips
        .iter()
        .filter(|strip| strip.analysis == analysis_key)
        .cloned()
        .collect();
    let digest = super::retained_dataset_digest(state.results_reads(), resolved.run);
    let mut document = build_document(resolved, digest, tracking)?;
    let bound: Vec<_> = document
        .panes()
        .iter()
        .filter(|pane| pane.binding.is_some())
        .map(|pane| pane.id)
        .collect();
    let scale_edits: Vec<DocumentEdit> = match x_scale {
        Some(scale) => document
            .axes()
            .iter()
            .filter(|axis| {
                axis.orientation == AxisOrientation::Horizontal
                    && axis.scale != scale
                    && bound.contains(&axis.pane_id)
            })
            .map(|axis| DocumentEdit::SetAxisScale {
                axis_id: axis.id,
                scale,
            })
            .collect(),
        None => Vec::new(),
    };
    let edits: Vec<DocumentEdit> = document
        .panes()
        .iter()
        .filter_map(|pane| Some((pane.id, pane.binding?.dataset)))
        .flat_map(|(pane_id, dataset)| {
            expressions.iter().map(move |(text, unit, visible)| {
                DocumentEdit::AddExpressionTrace(
                    rspice_results::visualization_document::ExpressionTrace {
                        pane_id,
                        text: text.clone(),
                        datasets: vec![dataset],
                        unit: unit.clone(),
                        visible: *visible,
                    },
                )
            })
        })
        .chain(scale_edits)
        .chain(
            bound
                .iter()
                .filter(|_| !xy_strips.is_empty())
                .map(|pane_id| DocumentEdit::SetXyPlots {
                    pane_id: *pane_id,
                    plots: super::persistent_document::xy_plots_of(*pane_id, &xy_strips),
                }),
        )
        .collect();
    if !edits.is_empty() {
        document.transact(document.revision(), edits)?;
    }
    let previous_documents_dirty = state.model.workspace.content.visualization_documents_dirty;
    let previous_workspace = state.workbench.workspace;
    let document_id = state
        .model
        .workspace
        .content
        .insert_visualization_document(document)?;

    if state.workbench.current_route().surface_id() != crate::workbench::SurfaceId::Results {
        state.workbench.activate(Workspace::Results);
    }
    if !crate::workbench::chrome::document_bar::results::show_visualization_document(
        state,
        &document_id,
    ) {
        if let Some(index) = state
            .model
            .workspace
            .content
            .visualization_documents
            .iter()
            .position(|document| document.id() == document_id)
        {
            state
                .model
                .workspace
                .content
                .visualization_documents
                .remove(index);
        }
        state.model.workspace.content.visualization_documents_dirty = previous_documents_dirty;
        state
            .model
            .workspace
            .content
            .visualization_documents_modified
            .remove(&document_id);
        state.workbench.activate(previous_workspace);
        return Err(CreateResultDocumentError::InvalidDraft(
            "The created result document could not resolve its retained dataset binding."
                .to_owned(),
        ));
    }
    Ok(document_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::RetainedTrace;

    /// Whatever the analysis, every sheet the form offers is a released one
    /// that a sheet of this build draws: a designed view no sheet draws is
    /// not a choice.
    #[test]
    fn the_form_offers_only_sheets_this_build_draws() {
        use AnalysisType as A;
        for analysis_type in [
            A::DcOp,
            A::DcSweep,
            A::Parametric,
            A::Ac,
            A::Disto,
            A::Transient,
            A::Noise,
            A::PoleZero,
            A::Tf,
            A::Sensitivity,
            A::Pac,
            A::Pnoise,
            A::Pxf,
            A::Pstb,
            A::Stb,
            A::MonteCarlo,
            A::Corner,
            A::Optimization,
            A::Soa,
            A::SParameter,
            A::Envelope,
            A::Fourier,
            A::HarmonicBalance,
            A::Pss,
            A::Qpss,
            A::Hbsp,
            A::Hbnoise,
            A::Psp,
            A::Qpac,
            A::Qpnoise,
            A::Qpxf,
            A::TransientNoise,
            A::DcMismatch,
        ] {
            let analysis = AnalysisResult::new(1, analysis_type, "analysis").with_waveforms(vec![
                RetainedTrace::new("V(out)", vec![1.0, 2.0], vec![0.0, 1.0], "#55aaff"),
            ]);
            for viewer in sheets_of(&analysis) {
                assert_eq!(viewer.release, ViewerReleaseClass::ReleaseTarget);
                assert!(
                    ResultViewer::from_viewer_document_id(viewer.id).is_some(),
                    "{} is offered for {analysis_type:?} with no sheet to draw it",
                    viewer.id
                );
            }
        }
    }
}
