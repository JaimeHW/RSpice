//! The Netlist workspace a launch leaves is the one the next launch opens:
//! the same tabs, the same one in front, and each document as it was left,
//! read back through the store the application keeps its session in.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rspice_app_code::workbench::documents::netlist_document::{NetlistSession, NetlistTab};
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
        state.workbench.messages(),
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

/// What one frame told assistive technology: each node's label, whether it
/// is a toggled row or tab, whether it is disabled, and where it is.
fn told(output: &egui::FullOutput) -> Vec<(String, bool, bool, egui::Rect)> {
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter_map(|(_, node)| {
            let bounds = node.bounds()?;
            Some((
                node.label()?.to_owned(),
                node.toggled().is_some(),
                node.is_disabled(),
                egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                ),
            ))
        })
        .collect()
}

/// The deck the launch reopens is missing: the workspace stands on its tab,
/// in its missing state alone. The deck's verbs are there and unavailable,
/// Run names the deck and cannot run it, nothing of the generated netlist's
/// view is shown, the outline holds just the missing file, and the two ways
/// out are offered once, in the page's body, where Remove asks first.
#[test]
fn a_deck_whose_file_went_between_launches_stands_on_its_tab_in_its_missing_state() {
    let (deck, include) = deck_on_disk();
    let mut app = working_app(&deck, &include);
    let id = app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .id;
    std::fs::remove_file(&deck).expect("the deck's file goes");

    let mut reopened = relaunch(&mut app);
    let ctx = themed_context();
    frame(&ctx, &mut reopened, DESKTOP);
    let output = frame(&ctx, &mut reopened, DESKTOP);

    assert_eq!(reopened.state.code.netlist.front, NetlistTab::Deck(id));
    let nodes = told(&output);
    let disabled = |wanted: &str| {
        nodes
            .iter()
            .find(|(label, ..)| label == wanted || label.starts_with(&format!("{wanted} (")))
            .map(|(_, _, disabled, _)| *disabled)
    };
    for verb in ["Save deck", "Export deck"] {
        assert_eq!(disabled(verb), Some(true), "{verb} is there, unavailable");
    }
    assert!(
        nodes
            .iter()
            .any(|(label, _, disabled, _)| label.starts_with("Run deck long.cir") && *disabled),
        "Run names the missing deck and cannot run it: {nodes:?}"
    );
    for gone in ["Save as deck", "Export netlist", "New deck…", "Open deck…"] {
        assert_eq!(disabled(gone), None, "{gone} belongs to another view");
    }
    for way_out in ["Locate…", "Remove from project"] {
        let offered = nodes.iter().filter(|(label, ..)| label == way_out).count();
        assert_eq!(offered, 1, "{way_out} is offered once: {nodes:?}");
    }
    // Remove from project asks first, in the body: its row turns into the
    // question.
    let at = nodes
        .iter()
        .find(|(label, ..)| label == "Remove from project")
        .map(|(.., rect)| rect.center())
        .expect("Remove from project is offered");
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame_with(
        &ctx,
        &mut reopened,
        DESKTOP,
        vec![egui::Event::PointerMoved(at), button(true)],
    );
    frame_with(&ctx, &mut reopened, DESKTOP, vec![button(false)]);
    let output = frame(&ctx, &mut reopened, DESKTOP);
    let asking = told(&output);
    let offered = |wanted: &str| asking.iter().filter(|(label, ..)| label == wanted).count();
    // Words are published as a label node's value, controls by their label.
    let question = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Label
                && node.value() == Some("Remove long.cir from the project? The file stays on disk.")
        })
        .count();
    assert_eq!(
        [
            question,
            offered("Remove"),
            offered("Cancel"),
            offered("Locate…"),
            offered("Remove from project"),
        ],
        [1, 1, 1, 0, 0],
        "the body asks before it removes: {asking:?}"
    );
    // The outline is the left dock's column of rows, right of the rail.
    let outline = nodes
        .iter()
        .filter(|(_, row, _, rect)| *row && rect.min.x > 50.0 && rect.max.x < 320.0)
        .map(|(label, ..)| label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        outline,
        ["long.cir"],
        "the outline holds just the missing file"
    );
}
