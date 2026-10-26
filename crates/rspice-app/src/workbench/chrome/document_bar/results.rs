//! The document bar's Results tabs: one per retained dataset, and the
//! activation of a dataset or a saved result document.

use crate::state::SimulationState;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// One tab per retained result dataset.
pub(in crate::workbench) fn result_documents(
    simulation: &SimulationState,
) -> Vec<WorkspaceDocument> {
    simulation
        .retained
        .runs
        .iter()
        .map(|run| WorkspaceDocument {
            id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
            label: run.label.clone(),
            icon: WorkbenchIcon::Results,
            dirty: false,
        })
        .collect()
}

/// Show a saved result document.
pub(in crate::workbench) fn activate_visualization_document(
    state: &mut ResultsParts<'_>,
    document_id: &crate::product::ResultDocumentId,
) -> bool {
    crate::workbench::documents::result_document::activate_persistent_document(state, *document_id)
}

/// Show a retained result dataset.
pub(in crate::workbench) fn activate_result_dataset(
    simulation: &mut SimulationState,
    dataset_id: &crate::product::DatasetId,
) -> bool {
    // Re-activating the dataset that is already active must not re-select
    // its run. `select_run` resynchronizes the displayed waveforms, which
    // advances the simulation data version, and every version change
    // retires cursors, the selected trace, the active pane, and pinned
    // readouts. This path runs on every frame the document bar draws, so
    // an unguarded re-selection made those states impossible to hold.
    simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == *dataset_id)
        .is_some_and(|index| {
            simulation.view.active_run_idx == Some(index) || simulation.select_run(index)
        })
}
