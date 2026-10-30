//! The Results workspace's toolbar, and whether the document it shows is
//! still current.

use crate::product::{ContentDigest, ObjectRevision};
use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, labeled_icon_button_sized};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::Workspace;

use rspice_app_workbench::workbench::chrome::toolbar::{
    explicit_label_width, take_projected_tool_slot,
};

/// A group of the Results toolbar's commands, drawn as icons and named by
/// the host, with a rule between one group and the next.
type ToolGroup = &'static [(CommandKey, WorkbenchIcon)];

/// The linked A/B cursors.
const LINKED_CURSORS: ToolGroup = &[(
    ResultsCommand::ToggleLinkedCursors.key(),
    WorkbenchIcon::Target,
)];

/// The waveform calculator.
const CALCULATOR: ToolGroup = &[(
    ResultsCommand::WaveformCalculator.key(),
    WorkbenchIcon::Sliders,
)];

/// The dataset's comparison, import, export and print.
const DATASET: ToolGroup = &[
    (
        ResultsCommand::CompareResultDatasets.key(),
        WorkbenchIcon::Compare,
    ),
    (
        ResultsCommand::ImportResultDataset.key(),
        WorkbenchIcon::Folder,
    ),
    (
        ResultsCommand::ExportWaveformsCsv.key(),
        WorkbenchIcon::Export,
    ),
    (ids::PRINT_HARDCOPY, WorkbenchIcon::File),
];

/// The Results toolbar's groups, in the order it lays them out: the linked
/// cursors, the calculator, which a historical document is not offered, and
/// the dataset's commands.
pub fn offered_tool_groups(state: ResultsReads<'_>) -> impl Iterator<Item = ToolGroup> + use<> {
    let historical = results_document_is_historical(state);
    [
        Some(LINKED_CURSORS),
        (!historical).then_some(CALCULATOR),
        Some(DATASET),
    ]
    .into_iter()
    .flatten()
}

/// The tracking control of the saved result document Results shows: Historical
/// when no completed run matches the current design, otherwise the Latest /
/// Pinned toggle. Whether it drew anything, for the rule after it.
pub fn results_tracking_button(
    ui: &mut egui::Ui,
    state: &mut ResultsParts<'_>,
    layout: LayoutSpec,
) -> bool {
    use crate::results::visualization_document::{
        DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
    };
    use crate::workbench::state::WorkspaceDocumentId;

    let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) = state
        .workbench
        .documents
        .active(Workspace::Results)
        .cloned()
    else {
        return false;
    };
    if results_document_is_historical(state.results_reads()) {
        if take_projected_tool_slot(ui, layout) {
            labeled_icon_button_sized(
                ui,
                WorkbenchIcon::History,
                "Historical",
                false,
                explicit_label_width("Historical"),
                layout.toolbar_control_height,
            )
            .on_hover_text(
                "This document still shows the results it was made from, but no completed run matches the current design and its netlist.",
            );
        }
        return true;
    }
    let Some((tracking, revision)) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(|document| (document.tracking(), document.revision()))
    else {
        return false;
    };
    let (Some(plan_id), Some(analysis_id)) =
        (tracking.simulation_plan_id, tracking.authored_analysis_id)
    else {
        // A migrated or externally imported historical document cannot
        // truthfully claim Latest until exact plan and authored-analysis
        // authority exists.
        return false;
    };
    if !take_projected_tool_slot(ui, layout) {
        return true;
    }

    let pinned = tracking.mode == ResultDocumentTrackingMode::Pinned;
    let label = if pinned { "Pinned" } else { "Latest" };
    let tooltip = if pinned {
        "Pinned to this run's results; a rerun does not replace them."
    } else {
        "Tracking latest: this document follows the newest run of its plan and analysis."
    };
    let response = labeled_icon_button_sized(
        ui,
        WorkbenchIcon::Pin,
        label,
        pinned,
        explicit_label_width(label),
        layout.toolbar_control_height,
    );
    let response = response.on_hover_text(tooltip);
    if response.clicked() {
        let next_mode = if pinned {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        };
        let next = ResultDocumentTracking::for_plan(next_mode, plan_id, analysis_id);
        let result = state
            .model
            .workspace
            .content
            .transact_visualization_document(
                document_id,
                revision,
                vec![DocumentEdit::SetTracking(next)],
            )
            .map(|_| ())
            .map_err(|error| error.to_string());
        match result {
            Ok(()) => state.workbench.prompt(if pinned {
                "Result document now follows the latest plan run."
            } else {
                "Result document is pinned to its current dataset."
            }),
            Err(error) => state.workbench.prompt(error),
        }
    }
    true
}

fn current_result_source_digest(model: &Model) -> Option<ContentDigest> {
    let netlist = &model.netlist_artifacts;
    (netlist.generation_error.is_none()
        && netlist.generated_input_digest.is_some()
        && netlist.generated_input_digest == netlist.current_generation_input_digest
        && !model.simulation.source.netlist_content.trim().is_empty())
    .then(|| crate::state::content_digest(&model.simulation.source.netlist_content))
}

pub fn run_has_current_success_authority(
    run: &crate::state::SimulationRun,
    project_revision: ObjectRevision,
    source_digest: Option<ContentDigest>,
) -> bool {
    run.lifecycle == crate::state::SimulationRunLifecycle::Completed
        && run.success
        && run.prepared_receipt().is_some_and(|receipt| {
            receipt.project_revision() == project_revision
                && source_digest.is_some_and(|digest| receipt.source_content_digest() == digest)
        })
}

pub fn results_document_is_historical(state: ResultsReads<'_>) -> bool {
    use crate::workbench::state::WorkspaceDocumentId;

    let project_revision = state.model.workspace.content.project.revision();
    let source_digest = current_result_source_digest(state.model);
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .filter(|run| run_has_current_success_authority(run, project_revision, source_digest))
            .max_by_key(|run| run.id)
            .is_none_or(|latest| latest.dataset_id != *dataset_id),
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
            let Some(document) = state
                .model
                .workspace
                .content
                .visualization_document(*document_id)
            else {
                return true;
            };
            let Some(binding) = document
                .panes()
                .iter()
                .filter_map(|pane| pane.binding)
                .next()
            else {
                return true;
            };
            let tracking = document.tracking();
            let (Some(plan_id), Some(analysis_id)) =
                (tracking.simulation_plan_id, tracking.authored_analysis_id)
            else {
                return true;
            };
            state
                .model.simulation
                .retained.runs
                .iter()
                .filter(|run| {
                    run_has_current_success_authority(run, project_revision, source_digest)
                        && run
                            .prepared_receipt()
                            .and_then(|receipt| receipt.simulation_plan_id())
                            == Some(plan_id)
                        && run.analyses.iter().any(|analysis| {
                            analysis.success
                                && crate::workbench::documents::result_document::analysis_matches_authored_source(
                                    analysis,
                                    analysis_id,
                                )
                        })
                })
                .max_by_key(|run| run.id)
                .is_none_or(|latest| latest.dataset_id != binding.dataset.dataset_id)
        }
        _ => false,
    }
}
