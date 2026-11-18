//! The Netlist workspace a launch leaves is the one the next launch opens:
//! the same tabs, the same one in front, and each document as it was left,
//! read back through the store the application keeps its session in.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rspice_app_code::workbench::documents::netlist_document::NetlistTab;
use rspice_app_code::workbench::documents::netlist_document::session::NetlistSession;
use rspice_app_state::workbench::preferences::TogglePreference;
use rspice_app_state::workbench::workspace::Workspace;

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, frame_with, themed_context};
use crate::workbench::app_state::AppState;

#[derive(Default)]
struct MemoryStorage(HashMap<String, String>);

impl eframe::Storage for MemoryStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }

    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }

    fn flush(&mut self) {}
}

/// A deck long enough to scroll, beside the file it includes, in a folder
/// of its own: the deck's path and the include's.
fn deck_on_disk() -> (PathBuf, PathBuf) {
    let folder =
        std::env::temp_dir().join(format!("rspice_netlist_session_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&folder).expect("deck folder");
    let include = folder.join("models.inc");
    std::fs::write(&include, ".model nch nmos\n").expect("include written");
    let mut deck = String::from("long deck\n.include models.inc\nV1 n1 0 1\n");
    for n in 1..=150 {
        deck.push_str(&format!("R{n} n{n} n{} 1k\n", n + 1));
    }
    deck.push_str(".op\n.end\n");
    let path = folder.join("long.cir");
    std::fs::write(&path, &deck).expect("deck written");
    (path, include)
}

/// The application with the deck open in front on the Netlist workspace,
/// its include open on a tab beside it, the editor gone to line 140 and
/// searching for "R14", and word wrap on.
fn working_app(deck: &Path, include: &Path) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let source = std::fs::read_to_string(deck).expect("deck read");
    assert!(
        app.state
            .open_test_deck(source, Some(deck.to_path_buf()), "long.cir")
    );
    app.state.workbench.activate(Workspace::Netlist);
    app.state
        .model
        .preferences
        .set_toggle(TogglePreference::EditorWordWrap, true);
    let state = &mut app.state;
    rspice_app_code::workbench::documents::deck_files::show_include(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        include,
    )
    .expect("the include opens");
    assert!(
        rspice_app_code::workbench::documents::deck_files::show_deck(
            &mut state.model,
            &mut state.code.netlist,
        )
    );
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    app.state.code.netlist.requested_line = Some(140);
    frame(&ctx, &mut app, DESKTOP);
    app.state.code.netlist.find.open = true;
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![egui::Event::Text("R14".to_owned())],
    );
    frame(&ctx, &mut app, DESKTOP);
    app
}

/// What the application keeps of its Netlist workspace.
fn netlist_session(state: &AppState) -> NetlistSession {
    state.code.netlist_session(
        state
            .model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id()),
    )
}

/// Store the application's session, and launch again from it, drawn twice.
fn relaunch(app: &mut RSpiceApp) -> RSpiceApp {
    let mut storage = MemoryStorage::default();
    app.state.store_session(&mut storage);
    let mut restored = AppState::restore_eframe_session(&storage).expect("the session restores");
    restored.restore_active_schematic_from_workspace();
    crate::workbench::app_state::services_access::initialize_from_session(&mut restored);
    let mut reopened = RSpiceApp::test_instance();
    reopened.state = restored;
    let ctx = themed_context();
    frame(&ctx, &mut reopened, DESKTOP);
    frame(&ctx, &mut reopened, DESKTOP);
    reopened
}

#[test]
fn a_launch_reopens_the_netlist_workspace_as_the_last_one_left_it() {
    let (deck, include) = deck_on_disk();
    let mut app = working_app(&deck, &include);
    let left = netlist_session(&app.state);
    let deck_id = app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .id;
    let front = NetlistTab::Deck(deck_id);
    assert_eq!(left.front(), Some(&front));
    let view = left.view(&front).expect("the deck's view is kept");
    assert!(view.caret().is_some_and(|caret| caret > 0), "{view:?}");
    assert!(view.scrolled_down() > 0.0, "{view:?}");
    assert_eq!(view.searching_for(), Some("R14"));

    let reopened = relaunch(&mut app);

    let state = &reopened.state;
    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    assert_eq!(state.code.netlist.front, front);
    assert_eq!(state.code.netlist.cursor_line, 139);
    assert_eq!(
        state.code.netlist.open_deck.as_ref().map(|deck| deck.id),
        Some(deck_id)
    );
    assert!(
        state.code.netlist_open_documents.iter().any(|document| matches!(
            document,
            rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId::NetlistDependency { logical_identity, .. }
                if Path::new(logical_identity) == include
        )),
        "the include's tab reopens"
    );
    assert!(
        state
            .model
            .preferences
            .toggle(TogglePreference::EditorWordWrap)
    );
    assert_eq!(
        netlist_session(state),
        left,
        "every tab, the one in front and the deck's caret, selection, scroll and search come back"
    );
}

#[test]
fn a_deck_whose_file_went_between_launches_reopens_as_missing() {
    let (deck, include) = deck_on_disk();
    let mut app = working_app(&deck, &include);
    std::fs::remove_file(&deck).expect("the deck's file goes");

    let reopened = relaunch(&mut app);

    let missing = rspice_app_code::workbench::documents::deck_files::missing_deck(
        &reopened.state.model,
        &reopened.state.code.netlist,
    )
    .expect("the deck reopens as missing");
    assert_eq!(missing.path.file_name(), deck.file_name());
    assert_eq!(reopened.state.workbench.workspace, Workspace::Netlist);
}
