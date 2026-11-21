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
use crate::workbench::documents::result_document::working_view::{self, source_of};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads, view_edits};
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

/// Why the view Results shows cannot be saved as a document, if it cannot.
pub(in crate::workbench) fn save_view_refusal(state: ResultsReads<'_>) -> Option<&'static str> {
    match active_result_document(state.model, state.workbench) {
        Some(WorkspaceDocumentId::VisualizationDocument(_)) => {
            Some("this is a saved document already")
        }
        Some(WorkspaceDocumentId::ResultDataset(dataset))
            if working_view::view_of_run(state, dataset).is_some() =>
        {
            None
        }
        _ => Some("Results shows no view to save"),
    }
}

/// `base`, or `base` and the first number after it that no document of the
/// project is titled, as titles are told apart regardless of case.
fn free_title(model: &Model, base: &str) -> String {
    let taken = |title: &str| {
        model
            .workspace
            .content
            .visualization_documents
            .iter()
            .any(|document| document.title().eq_ignore_ascii_case(title))
    };
    if !taken(base) {
        return base.to_owned();
    }
    (2..)
        .map(|number| format!("{base} {number}"))
        .find(|title| !taken(title))
        .unwrap_or_else(|| base.to_owned())
}

/// Keep the view Results shows as a result document of the project, titled
/// for its plan and the run it is on, and show it. Returns what the status
/// bar says.
pub(in crate::workbench) fn save_shown_view(
    state: &mut ResultsParts<'_>,
) -> Result<String, String> {
    let reads = state.results_reads();
    if let Some(reason) = save_view_refusal(reads) {
        return Err(reason.to_owned());
    }
    let Some(WorkspaceDocumentId::ResultDataset(dataset)) =
        active_result_document(reads.model, reads.workbench)
    else {
        return Err("Results shows no view to save".to_owned());
    };
    let view = working_view::view_of_run(reads, dataset).ok_or("Results shows no view to save")?;
    let run = reads
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .ok_or("the run is no longer in the run history")?;
    let source = source_of(run);
    let base = match source {
        WorkingViewSource::Plan { .. } => {
            format!(
                "{} \u{2014} {}",
                view_title(reads.model, source, run),
                run.name()
            )
        }
        WorkingViewSource::Imported { .. } => run.name().to_owned(),
    };
    let title = free_title(reads.model, &base);
    let saved = working_view::save_as_document(state, view, &title)?;
    show_visualization_document(state, &saved);
    Ok(format!("Saved the view as {title}."))
}

/// The saved result document Results shows, if it shows one.
fn shown_saved_document(state: ResultsReads<'_>) -> Option<crate::product::ResultDocumentId> {
    match active_result_document(state.model, state.workbench) {
        Some(WorkspaceDocumentId::VisualizationDocument(document)) => Some(document),
        _ => None,
    }
}

/// Why the saved document shown cannot be renamed, duplicated or deleted,
/// if it cannot.
pub(in crate::workbench) fn document_refusal(state: ResultsReads<'_>) -> Option<&'static str> {
    if state.model.safe_mode.project_read_only() {
        Some("the project is open read-only")
    } else if shown_saved_document(state).is_none() {
        Some("show a saved result document")
    } else {
        None
    }
}

/// Open the form that renames the saved document shown.
pub(in crate::workbench) fn open_rename(state: &mut ResultsParts<'_>) {
    let reads = state.results_reads();
    let Some(title) = shown_saved_document(reads)
        .and_then(|document| {
            reads
                .model
                .workspace
                .content
                .visualization_document(document)
        })
        .map(|document| document.title().to_owned())
    else {
        return;
    };
    *state.result_document_dialog = Some(
        rspice_results_ui::document_dialogs::ResultDocumentDialog::Rename(
            rspice_results_ui::document_dialogs::rename::RenameDraft::new(&title),
        ),
    );
}

/// Give the saved document shown the name `draft` holds, a step Undo takes
/// back. The form closes once the name lands; refused, it stays open and
/// the status bar says why.
pub(in crate::workbench) fn commit_rename(
    state: &mut ResultsParts<'_>,
    draft: &rspice_results_ui::document_dialogs::rename::RenameDraft,
) -> Result<Option<rspice_results_ui::document_dialogs::ResultDocumentDialog>, String> {
    let document = shown_saved_document(state.results_reads())
        .ok_or("Results no longer shows the document")?;
    let title = draft.title.trim();
    if title.is_empty() {
        return Err("Type a name for the document.".to_owned());
    }
    if title != draft.from {
        view_edits::rename_document(state, document, title)?;
        state.prompt(format!("Renamed {} to {title}.", draft.from));
    }
    Ok(None)
}

/// Copy the saved document shown as a document of its own, titled after it,
/// and show the copy: a step Undo takes back. Returns what the status bar
/// says.
pub(in crate::workbench) fn duplicate_shown_document(
    state: &mut ResultsParts<'_>,
) -> Result<String, String> {
    let reads = state.results_reads();
    let of = shown_saved_document(reads).ok_or("show a saved result document")?;
    let document = reads
        .model
        .workspace
        .content
        .visualization_document(of)
        .ok_or("the document is no longer in the project")?;
    let title = free_title(reads.model, &format!("{} copy", document.title()));
    let copy = document
        .duplicate(title.as_str())
        .map_err(|error| error.to_string())?;
    let from = document.title().to_owned();
    let shown = view_edits::add_duplicate(state, copy, of)?;
    show_visualization_document(state, &shown);
    Ok(format!("Duplicated {from} as {title}."))
}

/// Delete the saved document shown and show the tab beside it, as closing
/// it would: a step Undo takes back. Returns what the status bar says.
pub(in crate::workbench) fn delete_shown_document(
    state: &mut ResultsParts<'_>,
) -> Result<String, String> {
    let reads = state.results_reads();
    let document = shown_saved_document(reads).ok_or("show a saved result document")?;
    let title = reads
        .model
        .workspace
        .content
        .visualization_document(document)
        .map(|document| document.title().to_owned())
        .ok_or("the document is no longer in the project")?;
    let tabs = result_documents(reads.model);
    let at = tabs
        .iter()
        .position(|tab| tab.id == WorkspaceDocumentId::VisualizationDocument(document));
    let beside = at
        .and_then(|at| {
            tabs.get(at + 1)
                .or_else(|| at.checked_sub(1).and_then(|at| tabs.get(at)))
        })
        .map(|tab| tab.id.clone());
    view_edits::delete_document(state, document)?;
    match beside {
        Some(WorkspaceDocumentId::VisualizationDocument(next)) => {
            show_visualization_document(state, &next);
        }
        Some(WorkspaceDocumentId::ResultDataset(dataset)) => {
            show_result_dataset(state, &dataset);
        }
        _ => {}
    }
    Ok(format!("Deleted {title}. Undo puts it back."))
}
