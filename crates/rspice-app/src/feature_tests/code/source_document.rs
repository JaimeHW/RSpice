//! The source document dialog through the whole application: the lifecycle
//! transactions it opens, the creation form a project with no deck opens,
//! and the properties and rows the frame draws.

use crate::workbench::AppState;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{add_veriloga_cell, open_in_design};
use rspice_app_code::workbench::app::dialogs::source_document::fixtures::facts_of;
use rspice_app_code::workbench::app::dialogs::source_document::{
    apply_lifecycle_action, netlist_lifecycle_facts, open_source_document_dialog_in,
    source_document_dialog_is_available,
};
use rspice_app_code::workbench::code::commands::LifecycleAction;
use rspice_app_code::workbench::documents::code_workspace::CodeSourceFileAction;
use rspice_app_state::state::ProjectSourceOwner;

/// Bring the lifecycle dialog up for the page that is showing, as Manage
/// source document does.
fn open_source_document_dialog(state: &mut AppState) -> Result<(), String> {
    open_source_document_dialog_in(
        state.workbench.workspace,
        &state.model,
        &mut state.code.code_workspace,
        &state.code.netlist,
        &state.platform,
        state.workbench.messages(),
    )
}

/// A Verilog-A cell view whose source is `gain.va`, open in Design, and the
/// owner of that source.
fn veriloga_view_app() -> (RSpiceApp, ProjectSourceOwner) {
    let mut app = RSpiceApp::test_instance();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "gain.va", "gain");
    open_in_design(&mut app.state, &reference);
    (app, ProjectSourceOwner::cell_view(reference))
}

/// The defect this dialog exists for. Every file operation was reachable
/// only from a test, so the whole lifecycle shipped unreachable. Assert a
/// real transaction can be opened, edited, and committed through the same
/// entry points the body uses.
#[test]
fn a_rename_transaction_runs_end_to_end_through_the_dialog_entry_points() {
    let (mut app, owner) = veriloga_view_app();
    let original = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .map(|bundle| bundle.root().logical_path().to_owned())
        .unwrap_or_default();
    assert!(!original.is_empty(), "the fixture project owns a bundle");

    open_source_document_dialog(&mut app.state).expect("the Verilog-A view owns a source bundle");
    let facts = facts_of(
        app.state.workbench.workspace,
        &app.state.model,
        &app.state.code.code_workspace,
        &app.state.platform,
    )
    .expect("the page states a document");
    assert_eq!(facts.logical_path, original);
    assert!(facts.is_root, "the bundle root is the default document");

    apply_lifecycle_action(
        &mut app.code_workspace_parts(),
        LifecycleAction::Begin(CodeSourceFileAction::Rename),
        &facts.logical_path,
    );
    let draft = app
        .state
        .code
        .code_workspace
        .source_file_dialog
        .as_mut()
        .expect("Rename opens a transaction");
    draft.proposed_path = "models/renamed_gain.va".to_owned();

    apply_lifecycle_action(
        &mut app.code_workspace_parts(),
        LifecycleAction::Commit,
        &facts.logical_path,
    );
    assert!(
        app.state.code.code_workspace.source_file_dialog.is_none(),
        "a committed transaction closes"
    );
    let renamed = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .map(|bundle| bundle.root().logical_path().to_owned())
        .unwrap_or_default();
    assert_eq!(renamed, "models/renamed_gain.va");
}

/// The generated netlist is read-only, so the dialog has nothing to manage
/// until a deck is open.
#[test]
fn a_generated_netlist_page_refuses_the_dialog_until_a_deck_is_open() {
    let mut app = RSpiceApp::test_instance();
    // The generated deck is read-only and the project owns no other:
    // there is nothing to manage, and the refusal says so.
    assert!(!source_document_dialog_is_available(
        app.state.workbench.workspace,
        &app.state.model,
        &app.state.code.netlist,
        &app.state.platform,
    ));
    let refusal = open_source_document_dialog(&mut app.state).unwrap_err();
    assert!(refusal.contains("read-only"), "{refusal}");
    assert!(!app.state.code.code_workspace.source_document_dialog);

    app.state.seed_test_deck("* deck\nR1 a 0 1k\n.end\n");
    assert!(source_document_dialog_is_available(
        app.state.workbench.workspace,
        &app.state.model,
        &app.state.code.netlist,
        &app.state.platform,
    ));
    open_source_document_dialog(&mut app.state).unwrap();
    assert!(app.state.code.code_workspace.source_document_dialog);
    assert!(netlist_lifecycle_facts(&app.state.model, &app.state.code.netlist).is_some());
}
