//! Dataset-driven Create Result Document workflow.
//!
//! The modal is a transaction editor, not a navigation shortcut. Every
//! dataset, family, viewer, layout, and title is re-resolved when Create is
//! pressed; only a fully validated [`VisualizationDocument`] enters the
//! project-owned collection. Solver samples and retained run history are
//! never mutated.

use rspice_results::document_creation::{ResultDocumentFamily, ResultDocumentLayout};
use rspice_results_ui::create_document::{
    self, CreateDocumentHost, CreateResultDocumentDialogState, ViewerChoice,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerArt, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerReleaseClass, viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AxisOrientation, DocumentEdit, EntityRef, LinkKind, NewPagePane, NewPane, PageUpdatePolicy,
    PaneDataBinding, PaneKind, PanePlacement, ResultDocumentTracking, ResultDocumentTrackingMode,
    RunSignal, RunSourceAnalysis, SourceDataset, VisualizationDocument, VisualizationError,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle};
use crate::workbench::state::Workspace;

use super::{ResultsParts, ResultsReads};

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

#[derive(Debug, Clone, Copy)]
pub struct ResolvedDraft<'a> {
    name: &'a str,
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
    pub viewer: &'static ViewerDocumentDefinition,
    family: ResultDocumentFamily,
    layout: ResultDocumentLayout,
}

/// Open a fresh, dataset-aware transaction draft.
pub fn open(state: &mut ResultsParts<'_>) {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .or_else(|| state.model.simulation.retained.runs.last())
        .map(|run| run.dataset_id);
    let family = ResultDocumentFamily::WaveformWorksheet;
    let viewer_id = first_compatible_viewer(state.results_reads(), dataset_id, family)
        .unwrap_or("viewer-waveform");
    let name = next_document_name(state.results_reads(), family);
    *state.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name,
        name_touched: false,
        dataset_id,
        family_id: family.id().to_owned(),
        viewer_id: viewer_id.to_owned(),
        layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
        validation_error: None,
    };
}

/// Render the modal transaction editor.
pub(crate) fn show(ctx: &egui::Context, state: &mut ResultsParts<'_>) {
    if !state.create_result_document.open {
        return;
    }
    let mut draft = state.create_result_document.clone();
    let submit = create_document::show(
        ctx,
        &mut draft,
        &CreateDocumentSources(state.results_reads()),
    );
    *state.create_result_document = draft;
    if submit {
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
}

pub struct CreateDocumentSources<'a>(pub ResultsReads<'a>);

impl CreateDocumentHost for CreateDocumentSources<'_> {
    fn validation_message(&self, draft: &CreateResultDocumentDialogState) -> Option<String> {
        resolve_draft(self.0, draft)
            .as_ref()
            .err()
            .map(ToString::to_string)
    }

    fn can_create(&self, draft: &CreateResultDocumentDialogState) -> bool {
        resolve_draft(self.0, draft).is_ok()
    }

    fn select_family(
        &self,
        draft: &mut CreateResultDocumentDialogState,
        family: ResultDocumentFamily,
    ) {
        draft.family_id = family.id().to_owned();
        if !draft.name_touched {
            draft.name = next_document_name(self.0, family);
        }
        if !viewer_is_creatable(self.0, draft.dataset_id, family, &draft.viewer_id) {
            draft.viewer_id = first_compatible_viewer(self.0, draft.dataset_id, family)
                .unwrap_or_default()
                .to_owned();
        }
        draft.validation_error = None;
    }

    fn select_dataset(&self, draft: &mut CreateResultDocumentDialogState, dataset_id: DatasetId) {
        draft.dataset_id = Some(dataset_id);
        let family = ResultDocumentFamily::from_id(&draft.family_id)
            .unwrap_or(ResultDocumentFamily::WaveformWorksheet);
        if !viewer_is_creatable(self.0, draft.dataset_id, family, &draft.viewer_id) {
            draft.viewer_id = first_compatible_viewer(self.0, draft.dataset_id, family)
                .unwrap_or_default()
                .to_owned();
        }
        draft.validation_error = None;
    }

    fn dataset_label(&self, dataset_id: Option<DatasetId>) -> String {
        dataset_id
            .and_then(|id| retained_run(self.0, id))
            .map_or_else(|| "Select retained dataset".to_owned(), dataset_label)
    }

    fn datasets(&self) -> impl Iterator<Item = (DatasetId, String)> {
        self.0
            .model
            .simulation
            .retained
            .runs
            .iter()
            .map(|run| (run.dataset_id, dataset_label(run)))
    }

    fn viewer_catalog(&self, dataset_id: Option<DatasetId>) -> impl Iterator<Item = ViewerChoice> {
        let capabilities = dataset_id
            .and_then(|id| retained_run(self.0, id))
            .map(run_analysis_ids)
            .unwrap_or_default();
        VIEWER_DOCUMENTS.iter().map(move |viewer| {
            let compatibility = viewer_compatibility(
                viewer.id,
                ViewerCapabilities {
                    analysis_ids: &capabilities,
                    external_capabilities: &[],
                },
            );
            let renderer_available = dataset_id
                .and_then(|id| retained_run(self.0, id))
                .is_some_and(|run| {
                    run.analyses.iter().any(|analysis| {
                        super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
                    })
                });
            ViewerChoice {
                viewer,
                compatibility,
                renderer_available,
            }
        })
    }
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

fn dataset_label(run: &SimulationRun) -> String {
    let lifecycle = match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy retained",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "completed",
        SimulationRunLifecycle::Failed => "failed · partial",
        SimulationRunLifecycle::Aborted => "aborted · partial",
        SimulationRunLifecycle::Interrupted => "interrupted · partial",
    };
    format!(
        "Run {} · {} · {} analyses · {lifecycle}",
        run.id,
        run.label,
        run.analyses.len()
    )
}

fn run_analysis_ids(run: &SimulationRun) -> Vec<&'static str> {
    let mut ids = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            let analysis_type = analysis.analysis_type;
            std::iter::once(analysis_manifest_id(analysis_type))
                .chain((analysis_type == AnalysisType::Corner).then_some("temp"))
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    ids
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

pub fn first_compatible_viewer(
    state: ResultsReads<'_>,
    dataset_id: Option<DatasetId>,
    family: ResultDocumentFamily,
) -> Option<&'static str> {
    let run = dataset_id.and_then(|id| retained_run(state, id))?;
    let analysis_ids = run_analysis_ids(run);
    VIEWER_DOCUMENTS
        .iter()
        .find(|viewer| viewer_is_creatable_for_run(run, family, viewer, &analysis_ids))
        .map(|viewer| viewer.id)
}

pub fn viewer_is_creatable(
    state: ResultsReads<'_>,
    dataset_id: Option<DatasetId>,
    family: ResultDocumentFamily,
    viewer_id: &str,
) -> bool {
    let Some(run) = dataset_id.and_then(|id| retained_run(state, id)) else {
        return false;
    };
    let Some(viewer) = viewer_document(viewer_id) else {
        return false;
    };
    let analysis_ids = run_analysis_ids(run);
    viewer_is_creatable_for_run(run, family, viewer, &analysis_ids)
}

fn viewer_is_creatable_for_run(
    run: &SimulationRun,
    family: ResultDocumentFamily,
    viewer: &ViewerDocumentDefinition,
    analysis_ids: &[&str],
) -> bool {
    family.includes(viewer)
        && viewer.release == ViewerReleaseClass::ReleaseTarget
        && viewer_compatibility(
            viewer.id,
            ViewerCapabilities {
                analysis_ids,
                external_capabilities: &[],
            },
        )
        .is_compatible()
        && run.analyses.iter().any(|analysis| {
            super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
        })
}

fn next_document_name(state: ResultsReads<'_>, family: ResultDocumentFamily) -> String {
    for sequence in 1..=u32::MAX {
        let candidate = format!("{} · {sequence:02}", family.label());
        if !state
            .model
            .workspace
            .content
            .visualization_documents
            .iter()
            .any(|document| document.title().eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    family.label().to_owned()
}

pub fn resolve_draft<'a>(
    state: ResultsReads<'a>,
    draft: &'a CreateResultDocumentDialogState,
) -> Result<ResolvedDraft<'a>, CreateResultDocumentError> {
    let name = draft.name.as_str();
    if name.trim().is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Enter a document name.".to_owned(),
        ));
    }
    if name != name.trim() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot begin or end with whitespace.".to_owned(),
        ));
    }
    if name.len() > MAX_DOCUMENT_NAME_BYTES {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "Document names cannot exceed {MAX_DOCUMENT_NAME_BYTES} UTF-8 bytes."
        )));
    }
    if name.chars().any(char::is_control) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot contain control characters.".to_owned(),
        ));
    }
    if state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .any(|document| document.title().eq_ignore_ascii_case(name))
    {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "A result document named '{name}' already exists."
        )));
    }

    let family = ResultDocumentFamily::from_id(&draft.family_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document family.".to_owned(),
        )
    })?;
    let layout = ResultDocumentLayout::from_id(&draft.layout_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document layout.".to_owned(),
        )
    })?;
    let dataset_id = draft.dataset_id.ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a retained result dataset.".to_owned())
    })?;
    let run = retained_run(state, dataset_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "The selected result dataset is no longer retained.".to_owned(),
        )
    })?;
    if matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    ) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "A result document requires a retained, non-running dataset.".to_owned(),
        ));
    }
    if run.analyses.is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "The selected dataset contains no retained analyses.".to_owned(),
        ));
    }

    let viewer = viewer_document(&draft.viewer_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a viewer type.".to_owned())
    })?;
    if !family.includes(viewer) {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "{} is not part of the {} document family.",
            viewer.title,
            family.label()
        )));
    }
    let analysis_ids = run_analysis_ids(run);
    match viewer_compatibility(
        viewer.id,
        ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        },
    ) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires {} analysis data.",
                viewer.title,
                accepted_analysis_ids.join(" / ").to_ascii_uppercase()
            )));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires a qualified {capability_id} specialist dataset.",
                viewer.title
            )));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err(CreateResultDocumentError::InvalidDraft(
                "The selected viewer is not registered.".to_owned(),
            ));
        }
    }
    let analysis = run.analyses.iter().find(|analysis| {
        analysis_matches_viewer(viewer, analysis.analysis_type)
            && super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
    })
    .ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(format!(
            "{} cannot render any retained analysis in the selected dataset without substituting a different result contract.",
            viewer.title
        ))
    })?;

    Ok(ResolvedDraft {
        name,
        run,
        analysis,
        viewer,
        family,
        layout,
    })
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
/// run by identity and content digest, and the signals of the analysis. The
/// samples stay in the run, so a document is no larger for a larger run.
pub(super) fn source_reference(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<SourceDataset, VisualizationError> {
    SourceDataset::run_reference(
        DatasetBinding::new(run.dataset_id, run.dataset_content_digest()),
        reference_run_label(run),
        vec![run_source_analysis(run, analysis)?],
    )
}

/// One analysis of a run as a run reference catalogs it: its identity, its
/// kind and its signals.
pub(super) fn run_source_analysis(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<RunSourceAnalysis, VisualizationError> {
    let signals = analysis
        .waveforms
        .iter()
        .enumerate()
        .map(|(index, waveform)| {
            let index = u32::try_from(index).map_err(|_| VisualizationError::InvalidValue {
                field: "run-source.signals",
                message: "the analysis holds more signals than a run reference indexes".to_owned(),
            })?;
            Ok(RunSignal::new(
                index,
                waveform.name.clone(),
                waveform.complex.is_some(),
            ))
        })
        .collect::<Result<Vec<_>, VisualizationError>>()?;
    Ok(RunSourceAnalysis::new(
        source_analysis_identity(run, analysis),
        analysis.analysis_type,
        signals,
    ))
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
    tracking: ResultDocumentTracking,
) -> Result<VisualizationDocument, VisualizationError> {
    let source = source_reference(resolved.run, resolved.analysis)?;
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
        },
    )?;
    let page_id = document.pages()[0].id;
    let first_pane_id = document.panes()[0].id;
    document.transact(
        document.revision(),
        vec![
            DocumentEdit::SetTracking(tracking),
            DocumentEdit::Rename {
                entity: EntityRef::Page(page_id),
                value: resolved.family.label().to_owned(),
            },
            DocumentEdit::SetPageComposition {
                page_id,
                layout: resolved.layout.page_layout(),
                template_id: resolved.layout.template_id().to_owned(),
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
    let resolved = resolve_draft(state.results_reads(), state.create_result_document)?;
    let tracking = document_tracking(
        &state.model.simulation.retained.runs,
        resolved.run,
        resolved.analysis,
    );
    let document = build_document(resolved, tracking)?;
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
    state.create_result_document.open = false;
    state.create_result_document.validation_error = None;
    Ok(document_id)
}
