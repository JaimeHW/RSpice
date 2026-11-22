//! The source document dialog through the whole application: the lifecycle
//! transactions it opens, the creation form a project with no deck opens,
//! the properties and rows the frame draws, and the include folders a deck's
//! dialog adds, reorders and removes.

use crate::workbench::AppState;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{
    add_veriloga_cell, open_in_design, open_project_app,
};
use crate::workbench::app::veriloga_flow_tests::User;
use rspice_app_code::workbench::app::dialogs::source_document::fixtures::facts_of;
use rspice_app_code::workbench::app::dialogs::source_document::{
    apply_lifecycle_action, netlist_lifecycle_facts, open_source_document_dialog_in,
    source_document_dialog_is_available,
};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::code::commands::LifecycleAction;
use rspice_app_code::workbench::documents::code_workspace::CodeSourceFileAction;
use rspice_app_state::state::ProjectSourceOwner;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::localization::MessageId;
use std::path::PathBuf;

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

/// A deck page whose project searches `folders` for includes, its Deck and
/// include folders dialog open.
fn deck_dialog(folders: &[&str]) -> User {
    let mut app = open_project_app();
    app.state.seed_test_deck("* deck\nR1 a 0 1k\n.end\n");
    app.state.workbench.activate(Workspace::Netlist);
    app.state
        .model
        .workspace
        .content
        .project
        .set_include_search_paths(folders.iter().map(PathBuf::from).collect())
        .expect("the folders are a valid chain");
    let mut user = User::new(app);
    user.app
        .execute_command_key(CodeCommand::ManageSourceDocument.key());
    user.settle();
    let title = user
        .app
        .state
        .workbench
        .messages()
        .text(MessageId::NetlistManageDeckTitle);
    user.expect(&title);
    user
}

/// The project's include folders, in the order they are searched.
fn folders(user: &User) -> Vec<String> {
    user.app
        .state
        .model
        .workspace
        .content
        .project
        .include_search_paths()
        .iter()
        .map(|folder| folder.display().to_string())
        .collect()
}

/// Press `name`, then take the pointer off the dialog, so the next frame
/// shows the row actions of the selected folder and of no hovered one.
fn press(user: &mut User, name: &str) {
    user.click(name);
    user.frame(vec![egui::Event::PointerGone]);
    user.settle();
}

/// Add takes what the field holds, and only once it holds something: an
/// empty field adds nothing.
#[test]
fn add_takes_the_typed_include_folder_and_nothing_before_it() {
    let mut user = deck_dialog(&[]);
    let messages = user.app.state.workbench.messages();
    let add = messages.text(MessageId::NetlistIncludeSearchPathAddNamed);

    press(&mut user, &add);
    assert!(folders(&user).is_empty(), "an empty field adds nothing");

    user.focus_on(&messages.text(MessageId::NetlistIncludeSearchPathField));
    user.frame(vec![egui::Event::Text("models".to_owned())]);
    user.settle();
    press(&mut user, &add);
    assert_eq!(folders(&user), ["models"]);
    user.expect("1. models");
}

/// Move up, Move down and Remove are on the folder's own row: none shows
/// on a list at rest, and the selected folder's move it and remove it.
#[test]
fn a_selected_include_folder_moves_and_goes_by_its_own_row_actions() {
    let mut user = deck_dialog(&["models", "pdk", "vendor"]);
    let messages = user.app.state.workbench.messages();
    let up = messages.text(MessageId::NetlistIncludeSearchPathMoveUp);
    let down = messages.text(MessageId::NetlistIncludeSearchPathMoveDown);
    let remove = messages.text(MessageId::NetlistIncludeSearchPathRemove);
    assert!(
        user.find(&up).is_none() && user.find(&remove).is_none(),
        "a list at rest shows its folders and no actions"
    );

    press(&mut user, "2. pdk");
    press(&mut user, &up);
    assert_eq!(folders(&user), ["pdk", "models", "vendor"]);

    press(&mut user, &down);
    assert_eq!(folders(&user), ["models", "pdk", "vendor"]);

    press(&mut user, &remove);
    assert_eq!(folders(&user), ["models", "vendor"]);
    assert!(
        user.find(&remove).is_none(),
        "with its folder gone, no row is selected"
    );
}
