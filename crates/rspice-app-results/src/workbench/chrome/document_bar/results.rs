//! The document bar's Results tabs: one per view and one per saved result
//! document, and the activation of either.
//!
//! A view is how one plan's results are drawn, or one imported run's. Its
//! tab shows the run the view is on; which run that is, the newest or one
//! the reader chose, is picked on the toolbar's run selector, so a plan's
//! runs never each take a tab.

use crate::results::visualization_document::WorkingViewSource;
use crate::state::SimulationRun;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::working_view::source_of;
use crate::workbench::state::{WorkbenchSession, WorkspaceDocumentId};

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// The name a view's tab carries: its plan's, or the imported run's label.
pub fn view_title(model: &Model, source: WorkingViewSource, run: &SimulationRun) -> String {
    let WorkingViewSource::Plan { plan_id } = source else {
        return run.label.clone();
    };
    let setup = &model.sim_setup;
    setup
        .analysis_plan
        .as_ref()
        .filter(|plan| plan.id() == plan_id)
        .map(|_| setup.active_plan_name().to_string())
        .or_else(|| {
            setup
                .inactive_plans()
                .iter()
                .find(|plan| plan.id() == plan_id)
                .map(|plan| plan.name().to_string())
        })
        .unwrap_or_else(|| "Deleted plan".to_owned())
}

/// The run each view's tab is on, in the order the history first names
/// them: the run shown when it is one of the view's, else the run the view
/// reads, else the view's newest run.
pub fn view_runs(model: &Model) -> Vec<(WorkingViewSource, &SimulationRun)> {
    let content = &model.workspace.content;
    let runs = &model.simulation.retained.runs;
    let shown = model.simulation.active_run().map(|run| run.dataset_id);
    let mut views: Vec<(WorkingViewSource, &SimulationRun)> = Vec::new();
    for run in runs.iter() {
        let source = source_of(run);
        if views.iter().any(|(seen, _)| *seen == source) {
            continue;
        }
        let of_view = || {
            runs.iter()
                .filter(move |candidate| source_of(candidate) == source)
        };
        let read = content
            .visualization_documents
            .iter()
            .find(|document| document.working_view() == Some(source))
            .and_then(|view| view.panes().iter().find_map(|pane| pane.binding))
            .map(|binding| binding.dataset.dataset_id);
        let on = of_view()
            .find(|candidate| Some(candidate.dataset_id) == shown)
            .or_else(|| of_view().find(|candidate| Some(candidate.dataset_id) == read))
            .or_else(|| of_view().max_by_key(|candidate| candidate.id))
            .unwrap_or(run);
        views.push((source, on));
    }
    views
}

/// One tab per view, then one per saved result document in the order the
/// project made them.
pub fn result_documents(model: &Model) -> Vec<WorkspaceDocument> {
    let content = &model.workspace.content;
    let datasets = view_runs(model)
        .into_iter()
        .map(|(source, run)| WorkspaceDocument {
            id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
            label: view_title(model, source, run),
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
