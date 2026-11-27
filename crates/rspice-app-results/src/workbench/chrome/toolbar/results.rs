//! The Results workspace's toolbar, and whether the document it shows is
//! still current.

use crate::product::{ContentDigest, ObjectRevision};
use crate::state::command_catalog::CommandKey;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::ResultsReads;
use crate::workbench::documents::result_document::persistent_document::current_result_source_digest;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::Workspace;

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

/// The run's specifications, which Verify judges and edits: the one place
/// they are, opened from here under the Verify workspace's own glyph.
const SPECIFICATIONS: ToolGroup = &[(ids::VERIFY_SPECIFICATIONS, WorkbenchIcon::Verify)];

/// The Results toolbar's groups, in the order it lays them out: the linked
/// cursors, the calculator, the dataset's commands, and the specifications.
/// They are the same whichever tab is in front, so the toolbar never moves
/// under the pointer; what a document says of itself is on its own bar.
pub fn offered_tool_groups() -> impl Iterator<Item = ToolGroup> {
    [LINKED_CURSORS, CALCULATOR, DATASET, SPECIFICATIONS].into_iter()
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
    let source_digest = current_result_source_digest(state);
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
