//! The document bar's Results tabs: one per retained dataset and one per
//! saved result document, and the activation of either.

use crate::state::SimulationState;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::state::{WorkbenchSession, WorkspaceDocumentId};

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// One tab per retained result dataset, then one per saved result document
/// in the order the project made them.
pub fn result_documents(model: &Model) -> Vec<WorkspaceDocument> {
    let content = &model.workspace.content;
    // An edit to a result document is committed to the project's catalog as
    // it is made, and the catalog is saved and reverted whole, with the result
    // history: its one unsaved flag is every document's.
    let unsaved = content.visualization_documents_dirty;
    let datasets = model
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| WorkspaceDocument {
            id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
            label: run.label.clone(),
            icon: WorkbenchIcon::Results,
            dirty: false,
        });
    let documents = content
        .visualization_documents
        .iter()
        .map(|document| WorkspaceDocument {
            id: WorkspaceDocumentId::VisualizationDocument(document.id()),
            label: document.title().to_owned(),
            icon: WorkbenchIcon::Results,
            dirty: unsaved,
        });
    datasets.chain(documents).collect()
}

/// The document Results shows, as the document bar resolves it while
/// Results is the workspace showing: a retained dataset or a saved result
/// document, and no page document stands in for them.
pub(in crate::workbench) fn active_result_document(
    model: &Model,
    workbench: &WorkbenchSession,
) -> Option<WorkspaceDocumentId> {
    super::active_document_among(workbench, model, result_documents(model), None)
}

/// Show a saved result document.
pub fn activate_visualization_document(
    state: &mut ResultsParts<'_>,
    document_id: &crate::product::ResultDocumentId,
) -> bool {
    crate::workbench::documents::result_document::activate_persistent_document(state, *document_id)
}

/// Show a saved result document as the document Results reads, as choosing
/// its tab does: only while Results is the workspace showing.
pub(in crate::workbench) fn show_visualization_document(
    state: &mut ResultsParts<'_>,
    document_id: &crate::product::ResultDocumentId,
) -> bool {
    let document = WorkspaceDocumentId::VisualizationDocument(*document_id);
    if document.workspace() != state.workbench.workspace
        || !activate_visualization_document(state, document_id)
    {
        return false;
    }
    super::record_active_document(state.workbench, &document);
    true
}

/// Show a retained result dataset.
pub fn activate_result_dataset(
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

/// Show a retained result dataset as the document Results reads, as
/// choosing its tab does: only while Results is the workspace showing.
pub(in crate::workbench) fn show_result_dataset(
    simulation: &mut SimulationState,
    workbench: &mut WorkbenchSession,
    dataset_id: &crate::product::DatasetId,
) -> bool {
    let document = WorkspaceDocumentId::ResultDataset(*dataset_id);
    if document.workspace() != workbench.workspace
        || !activate_result_dataset(simulation, dataset_id)
    {
        return false;
    }
    super::record_active_document(workbench, &document);
    true
}
