//! The document bar's Results tabs: one per retained dataset, and the
//! activation of a dataset or a saved result document.

use crate::workbench::AppState;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use super::WorkspaceDocument;

/// One tab per retained result dataset.
pub(super) fn result_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    state
        .model
        .simulation
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
pub(super) fn activate_visualization_document(
    state: &mut AppState,
    document_id: &crate::product::ResultDocumentId,
) -> bool {
    crate::workbench::documents::result_document::activate_persistent_document(state, *document_id)
}

/// Show a retained result dataset.
pub(super) fn activate_result_dataset(
    state: &mut AppState,
    dataset_id: &crate::product::DatasetId,
) -> bool {
    // Re-activating the dataset that is already active must not re-select
    // its run. `select_run` resynchronizes the displayed waveforms, which
    // advances the simulation data version, and every version change
    // retires cursors, the selected trace, the active pane, and pinned
    // readouts. This path runs on every frame the document bar draws, so
    // an unguarded re-selection made those states impossible to hold.
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == *dataset_id)
        .is_some_and(|index| {
            state.model.simulation.view.active_run_idx == Some(index)
                || state.model.simulation.select_run(index)
        })
}
