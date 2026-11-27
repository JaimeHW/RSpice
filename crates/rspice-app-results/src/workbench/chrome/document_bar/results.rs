//! The document bar's Results tabs: the browser's view, then one per open
//! saved result document, and the activation of either.
//!
//! The data browser picks the run Results shows, and the run is drawn
//! through its plan's view, or the imported run's. The first tab is that
//! view's, named for the run: it is never closed, and it is the way back to
//! the browser's pick from a saved document. Runs and plans take no tab of
//! their own, and the data browser lists data only: a saved document is
//! opened from its tab, New result document, or Open result document.

use crate::product::DatasetId;
use crate::results::visualization_document::{ResultDocumentTrackingMode, WorkingViewSource};
use crate::state::SimulationRun;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::result_document::working_view::{self, source_of};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads, view_edits};
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// The name a plan's view goes by: its plan's, or the imported run's label.
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

/// The newest retained run drawn through `source`'s view.
fn newest_run_of(model: &Model, source: WorkingViewSource) -> Option<&SimulationRun> {
    model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| source_of(run) == source)
        .max_by_key(|run| run.id)
}

/// Whether `source`'s view follows its plan's newest run: it does until the
/// reader picks an older run, and before it is made.
fn follows_newest(state: ResultsReads<'_>, source: WorkingViewSource) -> bool {
    working_view::view_of(state, source)
        .and_then(|view| state.model.workspace.content.visualization_document(view))
        .is_none_or(|view| view.tracking().mode == ResultDocumentTrackingMode::Latest)
}

/// The run the browser's view shows, in the window drawn.
///
/// It is the run the window's Results shows, or, while a saved document is
/// in front, the run it showed before. A run the history starts after it
/// takes the view only when it is the newest of the same plan and the view
/// follows its plan's newest run: a run the reader picked in the browser
/// holds the view, and another plan's run is listed in the browser, not
/// shown over the one being read.
pub fn browser_run(state: ResultsReads<'_>) -> Option<&SimulationRun> {
    let model = state.model;
    let simulation = &model.simulation;
    let retained = |dataset: DatasetId| simulation.retained.run_by_dataset_id(dataset);
    let shown = simulation.active_run();
    let window = state.workbench.window_session.current();
    match state
        .workbench
        .window_session
        .active_document(window, Workspace::Results)
    {
        Some(WorkspaceDocumentId::ResultDataset(dataset)) => {
            let Some(held) = retained(*dataset) else {
                return shown;
            };
            let source = source_of(held);
            let newer = shown.filter(|run| {
                run.id > held.id
                    && source_of(run) == source
                    && newest_run_of(model, source).map(|newest| newest.dataset_id)
                        == Some(run.dataset_id)
                    && follows_newest(state, source)
            });
            Some(newer.unwrap_or(held))
        }
        Some(WorkspaceDocumentId::VisualizationDocument(_)) => {
            state.results.quick_view_run.and_then(retained).or(shown)
        }
        _ => shown,
    }
}

/// Keep the window's record of what Results shows on the run the browser's
/// view shows, while that view is in front: a run that takes the view, and
/// the run shown when Results opens, are what it goes on showing, whatever
/// run the history starts next.
pub fn keep_browser_view(state: &mut ResultsParts<'_>) {
    if state.workbench.workspace != Workspace::Results {
        return;
    }
    let window = state.workbench.window_session.current();
    let held = state
        .workbench
        .window_session
        .active_document(window, Workspace::Results)
        .cloned();
    if matches!(held, Some(WorkspaceDocumentId::VisualizationDocument(_))) {
        return;
    }
    let Some(run) = browser_run(state.results_reads()).map(|run| run.dataset_id) else {
        return;
    };
    let shown = WorkspaceDocumentId::ResultDataset(run);
    if held.as_ref() != Some(&shown) {
        super::record_active_document(state.workbench, &shown);
    }
}

/// The tab of the browser's view, named for the run it shows.
fn browser_view_tab(run: &SimulationRun) -> WorkspaceDocument {
    WorkspaceDocument {
        id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
        label: run.name().to_owned(),
        icon: WorkbenchIcon::Results,
        dirty: false,
    }
}

/// A saved result document as Results lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListedDocument {
    pub id: crate::product::ResultDocumentId,
    pub title: String,
    /// Whether it holds edits the project has not saved. Only its own edits
    /// count: what the reader does in the quick view edits its run's working
    /// view, which has no tab to mark; the project's unsaved state shows it.
    pub dirty: bool,
}

/// The saved result documents, in the order the project made them.
pub fn saved_documents(model: &Model) -> Vec<ListedDocument> {
    let content = &model.workspace.content;
    content
        .visualization_documents
        .iter()
        .filter(|document| document.working_view().is_none())
        .map(|document| ListedDocument {
            id: document.id(),
            title: document.title().to_owned(),
            dirty: content
                .visualization_documents_modified
                .contains(&document.id()),
        })
        .collect()
}

/// The browser's view first, then each saved result document in the order
/// the project made them, then the browser's views other windows show.
pub fn result_documents(state: ResultsReads<'_>) -> Vec<WorkspaceDocument> {
    let model = state.model;
    let view = browser_run(state).map(browser_view_tab);
    let shown = view.as_ref().map(|tab| tab.id.clone());
    let sessions = &state.workbench.window_session;
    let current = sessions.current();
    let elsewhere: Vec<WorkspaceDocument> = sessions
        .windows()
        .filter(|(window, _)| *window != current)
        .flat_map(|(window, held)| {
            held.documents
                .iter()
                .filter_map(move |document| match document {
                    WorkspaceDocumentId::ResultDataset(dataset)
                        if sessions.owner(document) == window =>
                    {
                        Some(*dataset)
                    }
                    _ => None,
                })
        })
        .filter_map(|dataset| model.simulation.retained.run_by_dataset_id(dataset))
        .map(browser_view_tab)
        .filter(|tab| Some(&tab.id) != shown.as_ref())
        .collect();
    let documents = saved_documents(model)
        .into_iter()
        .map(|document| WorkspaceDocument {
            id: WorkspaceDocumentId::VisualizationDocument(document.id),
            label: document.title,
            icon: WorkbenchIcon::Results,
            dirty: document.dirty,
        });
    view.into_iter().chain(documents).chain(elsewhere).collect()
}

/// The document Results shows, as the document bar resolves it while
/// Results is the workspace showing: the browser's view or a saved result
/// document, and no page document stands in for them.
pub(in crate::workbench) fn active_result_document(
    state: ResultsReads<'_>,
) -> Option<WorkspaceDocumentId> {
    super::active_document_among(state.workbench, state.model, result_documents(state), None)
}

/// Show a saved result document. The browser's view keeps the run it showed,
/// which a saved document's pane takes from it, to show it again when its
/// tab is chosen.
pub fn activate_visualization_document(
    state: &mut ResultsParts<'_>,
    document_id: &crate::product::ResultDocumentId,
) -> bool {
    let browser = browser_run(state.results_reads()).map(|run| run.dataset_id);
    let shown = crate::workbench::documents::result_document::activate_persistent_document(
        state,
        *document_id,
    );
    if shown {
        state.results.quick_view_run = browser;
    }
    shown
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
        state.results.quick_view_run = None;
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

/// Show the run of `dataset` through the browser's view, as picking it in
/// the data browser does, whatever Results showed: a plan's run through the
/// plan's view, which follows the plan's newest run when `dataset` is that
/// run and holds on `dataset` when it is an older one; an imported run
/// through its own.
pub fn show_run(state: &mut ResultsParts<'_>, dataset: DatasetId) -> Result<(), String> {
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .ok_or("the run is no longer in the run history")?;
    let source = source_of(run);
    let newest = newest_run_of(state.model, source).map(|run| run.dataset_id) == Some(dataset);
    if !show_result_dataset(state, &dataset) {
        return Err("Results is not the workspace showing".to_owned());
    }
    let WorkingViewSource::Plan { .. } = source else {
        return Ok(());
    };
    if working_view::view_of(state.results_reads(), source).is_none() {
        // The plan's first run shown makes its view.
        working_view::sync(state);
    }
    // A run still being digested has no view yet; it is shown all the same.
    if let Some(view) = working_view::view_of(state.results_reads(), source)
        && let Err(error) = working_view::choose_run(state, view, (!newest).then_some(dataset))
    {
        state.prompt(format!("The view could not keep to the run: {error}"));
    }
    Ok(())
}

/// The run after or before the browser's in the data browser's run list,
/// which lists the retained runs newest first: the newer run when `newer`.
pub fn stepped_run(state: ResultsReads<'_>, newer: bool) -> Option<DatasetId> {
    let runs = &state.model.simulation.retained.runs;
    let at = browser_run(state).and_then(|shown| {
        runs.iter()
            .position(|run| run.dataset_id == shown.dataset_id)
    })?;
    let next = if newer { at.checked_sub(1)? } else { at + 1 };
    runs.get(next).map(|run| run.dataset_id)
}

/// Show the run newer or older than the one the browser's view shows, as
/// stepping through the data browser's run list does. Says why when there
/// is no run to step to.
pub fn step_run(state: &mut ResultsParts<'_>, newer: bool) -> Result<(), String> {
    let dataset = stepped_run(state.results_reads(), newer).ok_or(if newer {
        "the run shown is the newest"
    } else {
        "the run shown is the oldest"
    })?;
    show_run(state, dataset)
}

/// Why the view Results shows cannot be saved as a document, if it cannot.
pub(in crate::workbench) fn save_view_refusal(state: ResultsReads<'_>) -> Option<&'static str> {
    match active_result_document(state) {
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
pub(in crate::workbench) fn free_title(model: &Model, base: &str) -> String {
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
    let Some(WorkspaceDocumentId::ResultDataset(dataset)) = active_result_document(reads) else {
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
pub fn shown_document(state: ResultsReads<'_>) -> Option<crate::product::ResultDocumentId> {
    match active_result_document(state) {
        Some(WorkspaceDocumentId::VisualizationDocument(document)) => Some(document),
        _ => None,
    }
}

/// Why the saved document shown cannot be renamed, duplicated or deleted,
/// if it cannot.
pub(in crate::workbench) fn document_refusal(state: ResultsReads<'_>) -> Option<&'static str> {
    if state.model.safe_mode.project_read_only() {
        Some("the project is open read-only")
    } else if shown_document(state).is_none() {
        Some("show a saved result document")
    } else {
        None
    }
}

/// Open the form that renames the saved document shown.
pub(in crate::workbench) fn open_rename(state: &mut ResultsParts<'_>) {
    let reads = state.results_reads();
    let Some(title) = shown_document(reads)
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
    let document =
        shown_document(state.results_reads()).ok_or("Results no longer shows the document")?;
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
    let of = shown_document(reads).ok_or("show a saved result document")?;
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

/// Every saved result document as Open result document lists it, the most
/// recently made first, each with the run and analysis its first pane
/// reads, beside the document it is.
pub(in crate::workbench) fn open_rows(
    state: ResultsReads<'_>,
) -> Vec<(
    crate::product::ResultDocumentId,
    rspice_results_ui::document_dialogs::open::OpenRow,
)> {
    use crate::workbench::documents::result_document::persistent_document;
    let content = &state.model.workspace.content;
    let runs = &state.model.simulation.retained.runs;
    saved_documents(state.model)
        .into_iter()
        .rev()
        .map(|listed| {
            let read = content
                .visualization_document(listed.id)
                .and_then(|document| {
                    let page = document.pages().first()?;
                    persistent_document::ordered_page_panes(document.panes(), page)
                        .into_iter()
                        .find_map(|pane| pane.binding)
                })
                .and_then(|binding| persistent_document::resolve_binding(state, binding).ok())
                .and_then(|(run, analysis)| {
                    let run = runs.get(run)?;
                    Some((run.label.clone(), run.analyses.get(analysis)?.label.clone()))
                });
            let (run, analysis) =
                read.unwrap_or_else(|| ("not in the run history".to_owned(), String::new()));
            (
                listed.id,
                rspice_results_ui::document_dialogs::open::OpenRow {
                    key: listed.id.as_uuid().to_string(),
                    title: listed.title,
                    run,
                    analysis,
                },
            )
        })
        .collect()
}

/// Open the form that brings a saved result document forward.
pub(in crate::workbench) fn open_document_form(state: &mut ResultsParts<'_>) {
    *state.result_document_dialog =
        Some(rspice_results_ui::document_dialogs::ResultDocumentDialog::Open(Default::default()));
}

/// Show the document chosen in Open result document: its tab is added, or
/// brought forward if it is open, and the form closes.
pub(in crate::workbench) fn open_chosen(
    state: &mut ResultsParts<'_>,
    draft: &rspice_results_ui::document_dialogs::open::OpenDraft,
) -> Result<Option<rspice_results_ui::document_dialogs::ResultDocumentDialog>, String> {
    let rows = open_rows(state.results_reads());
    let (document, _) = draft
        .chosen()
        .and_then(|row| rows.get(row))
        .ok_or("Choose a document to open.")?;
    if !show_visualization_document(state, document) {
        return Err("Results is not the workspace showing".to_owned());
    }
    Ok(None)
}

/// Delete the saved document shown and show the tab beside it, as closing
/// it would: a step Undo takes back, so nothing asks first, unsaved edits or
/// not. Returns what the status bar says.
pub(in crate::workbench) fn delete_shown_document(
    state: &mut ResultsParts<'_>,
) -> Result<String, String> {
    let reads = state.results_reads();
    let document = shown_document(reads).ok_or("show a saved result document")?;
    let title = reads
        .model
        .workspace
        .content
        .visualization_document(document)
        .map(|document| document.title().to_owned())
        .ok_or("the document is no longer in the project")?;
    let shown = WorkspaceDocumentId::VisualizationDocument(document);
    let tabs = result_documents(reads)
        .into_iter()
        .filter(|tab| tab.id == shown || !reads.workbench.documents.is_closed(&tab.id))
        .collect::<Vec<_>>();
    let at = tabs.iter().position(|tab| tab.id == shown);
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
