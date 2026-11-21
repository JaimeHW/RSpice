//! A saved result document kept from the view Results shows, then renamed,
//! duplicated and deleted from the Results menu, each step undone and
//! redone in Results itself.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::document_edit::commit_open_dialog;
use rspice_app_results::workbench::documents::result_document::working_view;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_results_ui::document_dialogs::ResultDocumentDialog;

/// Results showing the view of one finished transient run.
fn app_on_a_view() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffffff"),
        ]),
    );
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    app.state.workbench.activate(Workspace::Results);
    let mut parts = app.state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
    app
}

/// The titles of the saved documents, in the project's order.
fn saved(app: &RSpiceApp) -> Vec<String> {
    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .filter(|document| document.working_view().is_none())
        .map(|document| document.title().to_owned())
        .collect()
}

/// The title of the saved document Results shows, if it shows one.
fn shown(app: &RSpiceApp) -> Option<String> {
    match app.state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::VisualizationDocument(document)) => app
            .state
            .model
            .workspace
            .content
            .visualization_document(*document)
            .map(|document| document.title().to_owned()),
        _ => None,
    }
}

/// Raise `command`, which has to be available.
fn run(app: &mut RSpiceApp, command: ResultsCommand) {
    let key = command.key();
    assert!(
        app.command_key_available(key),
        "{command:?}: {:?}",
        app.command_availability(key)
    );
    app.execute_command_key(key);
}

fn undo(app: &mut RSpiceApp) {
    assert!(app.command_key_available(WorkbenchCommand::Undo.key()));
    app.execute_command_key(WorkbenchCommand::Undo.key());
}

fn redo(app: &mut RSpiceApp) {
    assert!(app.command_key_available(WorkbenchCommand::Redo.key()));
    app.execute_command_key(WorkbenchCommand::Redo.key());
}

/// Save view keeps the view as a document named for its run and shows it;
/// Rename, Duplicate and Delete then act on it, and Undo takes each back in
/// Results, latest first, with Redo taking it again.
#[test]
fn rename_duplicate_and_delete_of_a_saved_document_each_undo() {
    let mut app = app_on_a_view();
    run(&mut app, ResultsCommand::SaveViewAsDocument);
    let first = saved(&app);
    assert_eq!(first.len(), 1, "the view is kept as one document");
    assert_eq!(shown(&app), Some(first[0].clone()), "and shown");

    run(&mut app, ResultsCommand::RenameDocument);
    let Some(ResultDocumentDialog::Rename(mut draft)) =
        app.state.results.result_document_dialog.clone()
    else {
        panic!("Rename opens its form");
    };
    assert_eq!(draft.title, first[0], "the form starts from the name");
    draft.title = "Gain sweep".to_owned();
    app.state.results.result_document_dialog = Some(ResultDocumentDialog::Rename(draft));
    commit_open_dialog(&mut app.state.results_parts());
    assert_eq!(app.state.results.result_document_dialog, None);
    assert_eq!(saved(&app), ["Gain sweep"]);
    undo(&mut app);
    assert_eq!(saved(&app), first, "Undo takes the name back");
    redo(&mut app);
    assert_eq!(saved(&app), ["Gain sweep"], "and Redo gives it again");

    run(&mut app, ResultsCommand::DuplicateDocument);
    assert_eq!(saved(&app), ["Gain sweep", "Gain sweep copy"]);
    assert_eq!(shown(&app).as_deref(), Some("Gain sweep copy"));
    undo(&mut app);
    assert_eq!(saved(&app), ["Gain sweep"], "Undo takes the copy away");
    assert_eq!(shown(&app).as_deref(), Some("Gain sweep"));
    redo(&mut app);
    assert_eq!(saved(&app), ["Gain sweep", "Gain sweep copy"]);
    assert_eq!(shown(&app).as_deref(), Some("Gain sweep copy"));
    undo(&mut app);

    run(&mut app, ResultsCommand::DeleteDocument);
    assert!(saved(&app).is_empty(), "Delete takes the document away");
    assert_eq!(shown(&app), None, "and shows the view beside it");
    undo(&mut app);
    assert_eq!(saved(&app), ["Gain sweep"], "Undo puts it back");
    assert_eq!(shown(&app).as_deref(), Some("Gain sweep"));
    undo(&mut app);
    assert_eq!(saved(&app), first, "and the step before it is next");
}

/// A name another document holds is refused and the form stays open, as is
/// a blank one; Rename, Duplicate and Delete are off with no saved document
/// shown.
#[test]
fn a_taken_name_is_refused_and_the_operations_need_a_saved_document() {
    let mut app = app_on_a_view();
    for command in [
        ResultsCommand::RenameDocument,
        ResultsCommand::DuplicateDocument,
        ResultsCommand::DeleteDocument,
    ] {
        assert!(
            !app.command_key_available(command.key()),
            "{command:?} with a view shown"
        );
    }
    run(&mut app, ResultsCommand::SaveViewAsDocument);
    run(&mut app, ResultsCommand::DuplicateDocument);
    let titles = saved(&app);
    run(&mut app, ResultsCommand::RenameDocument);
    for typed in [titles[0].to_uppercase(), "  ".to_owned()] {
        let Some(ResultDocumentDialog::Rename(mut draft)) =
            app.state.results.result_document_dialog.clone()
        else {
            panic!("Rename opens its form");
        };
        draft.title = typed;
        app.state.results.result_document_dialog = Some(ResultDocumentDialog::Rename(draft));
        commit_open_dialog(&mut app.state.results_parts());
        assert!(
            app.state.results.result_document_dialog.is_some(),
            "the form stays open"
        );
        assert_eq!(saved(&app), titles, "nothing is renamed");
    }
}
