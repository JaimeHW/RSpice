//! The document bar's Results tabs: one per retained dataset and one per
//! saved result document, and the activation of either.

use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::state::{WorkbenchSession, WorkspaceDocumentId};

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// One tab per retained result dataset, then one per saved result document
/// in the order the project made them. A run's working view is drawn
/// through its dataset's tab and has none of its own.
pub fn result_documents(model: &Model) -> Vec<WorkspaceDocument> {
    let content = &model.workspace.content;
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
        .filter(|document| document.working_view().is_none())
        .map(|document| WorkspaceDocument {
            id: WorkspaceDocumentId::VisualizationDocument(document.id()),
            label: document.title().to_owned(),
            icon: WorkbenchIcon::Results,
            // Only this document's own edits mark its tab. What the reader
            // does in the quick view edits its run's working view, which has
            // no tab to mark; the project's unsaved state shows it.
            dirty: content
                .visualization_documents_modified
                .contains(&document.id()),
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

/// Show a retained result dataset in the quick view, with the quick view's
/// own viewer if a saved document's pane took it.
///
/// This path runs on every frame the document bar or a window draws.
/// Selecting the run already shown changes nothing, and another run comes
/// back on the analysis it was last shown on.
pub fn activate_result_dataset(
    state: &mut ResultsParts<'_>,
    dataset_id: &crate::product::DatasetId,
) -> bool {
    let simulation = &mut state.model.simulation;
    let shown = simulation
        .retained
        .runs
        .iter()
        .position(|run| run.dataset_id == *dataset_id)
        .is_some_and(|index| simulation.select_run(index));
    if shown {
        crate::workbench::documents::result_document::restore_quick_view_viewer(state.results);
    }
    shown
}

/// Show a retained result dataset as the document Results reads, as
/// choosing its tab does: only while Results is the workspace showing.
pub(in crate::workbench) fn show_result_dataset(
    state: &mut ResultsParts<'_>,
    dataset_id: &crate::product::DatasetId,
) -> bool {
    let document = WorkspaceDocumentId::ResultDataset(*dataset_id);
    if document.workspace() != state.workbench.workspace
        || !activate_result_dataset(state, dataset_id)
    {
        return false;
    }
    super::record_active_document(state.workbench, &document);
    true
}
