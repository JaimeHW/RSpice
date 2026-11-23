//! Deck and include folders through the whole application: which pages
//! have it, the tail line that says where the search goes past the rows,
//! and the include folders it adds, reorders and removes.

use crate::workbench::AppState;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{
    add_veriloga_cell, open_in_design, open_project_app,
};
use crate::workbench::app::veriloga_flow_tests::User;
use rspice_app_code::workbench::app::dialogs::source_document::{
    netlist_lifecycle_facts, open_source_document_dialog_in, source_document_dialog_is_available,
};
use rspice_app_code::workbench::code::commands::SourceCommand;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::localization::MessageId;
use std::path::PathBuf;

/// Bring the dialog up for the page that is showing, as Deck and include
/// folders… does.
fn open_source_document_dialog(state: &mut AppState) -> Result<(), String> {
    open_source_document_dialog_in(
        state.workbench.workspace,
        &state.model,
        &mut state.code.source_editing,
        &state.code.netlist,
        &state.platform,
        state.workbench.messages(),
    )
}

/// Whether the page that is showing has the dialog.
fn available(state: &AppState) -> bool {
    source_document_dialog_is_available(
        state.workbench.workspace,
        &state.model,
        &state.code.netlist,
        &state.platform,
    )
}

/// The generated netlist is read-only, so the dialog has nothing to manage
/// until a deck is open.
#[test]
fn a_generated_netlist_page_refuses_the_dialog_until_a_deck_is_open() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Netlist);
    // The generated deck is read-only and the project owns no other:
    // there is nothing to manage, and the refusal says so.
    assert!(!available(&app.state));
    let refusal = open_source_document_dialog(&mut app.state).unwrap_err();
    assert!(refusal.contains("read-only"), "{refusal}");
    assert!(!app.state.code.source_editing.source_document_dialog);

    app.state.seed_test_deck("* deck\nR1 a 0 1k\n.end\n");
    assert!(available(&app.state));
    open_source_document_dialog(&mut app.state).unwrap();
    assert!(app.state.code.source_editing.source_document_dialog);
    assert!(netlist_lifecycle_facts(&app.state.model, &app.state.code.netlist).is_some());
}

/// Only a deck has the dialog. A Verilog-A file's operations are its row's
/// on the Files rail, so its page offers none and the command says why.
#[test]
fn a_veriloga_page_has_no_source_dialog() {
    let mut app = RSpiceApp::test_instance();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "gain.va", "gain");
    open_in_design(&mut app.state, &reference);
    assert!(!available(&app.state));
    let refusal = open_source_document_dialog(&mut app.state).unwrap_err();
    assert_eq!(
        refusal,
        app.state
            .workbench
            .messages()
            .text(MessageId::NetlistManageNeedsDeck)
    );
    assert!(!app.state.code.source_editing.source_document_dialog);
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
        .execute_command_key(SourceCommand::ManageSourceDocument.key());
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

/// The search goes on past the rows, so a list that ended at its last row
/// would misstate where a name is found: one line under the rows says where,
/// in the words the empty list uses.
#[test]
fn the_folder_list_says_where_the_search_goes_past_its_rows() {
    let user = deck_dialog(&[]);
    let messages = user.app.state.workbench.messages();
    user.expect(&messages.text(MessageId::NetlistIncludeSearchPathEmpty));

    let user = deck_dialog(&["models", "pdk"]);
    user.expect(&messages.text(MessageId::NetlistIncludeSearchPathThen));
    assert!(
        user.find(&messages.text(MessageId::NetlistIncludeSearchPathEmpty))
            .is_none(),
        "a list with rows does not call itself empty"
    );
}
