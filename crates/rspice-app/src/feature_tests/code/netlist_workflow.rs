//! Decks through the whole application: a deck opens from its file, in a
//! project of its own when none is open; Save writes it at once, in its own
//! encoding and line endings; the project keeps where the deck is, never its
//! text or its includes'; and the deck runs as the engine reads it, whatever
//! dialect it is written in.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_code::workbench::documents::deck_files::LineEnding;
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_state::state::TextEncoding;
use rspice_app_state::workbench::workspace::Workspace;

/// A folder of one test's files, removed with it.
pub(super) struct Folder(pub(super) PathBuf);

impl Folder {
    pub(super) fn new() -> Self {
        Self(rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir("decks"))
    }

    /// Write `bytes` to `name` in the folder; the file's path.
    pub(super) fn write(&self, name: &str, bytes: impl AsRef<[u8]>) -> PathBuf {
        let path = self.0.join(name);
        std::fs::create_dir_all(path.parent().expect("a folder")).expect("the folder");
        std::fs::write(&path, bytes).expect("the file is written");
        path
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A session with the deck at `path` open in a project of its own.
pub(super) fn opened(path: &Path) -> AppState {
    let mut state = AppState::default();
    assert!(
        state.open_deck_at(path, true),
        "the deck opens: {:?}",
        state.prompt_text()
    );
    state
}

/// The application with the deck at `path` open in its project.
pub(super) fn app_with_deck(path: &Path) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let new_project = !app.state.platform.project_lifecycle.is_open();
    assert!(
        app.state.open_deck_at(path, new_project),
        "the deck opens: {:?}",
        app.state.prompt_text()
    );
    app
}

/// The deck in front prepared as Run deck prepares it and handed to the
/// engine: the netlist the engine reads, or why it was refused.
pub(super) fn dispatched(state: &mut AppState) -> Result<String, String> {
    state.model.simulation.execution.run_intent =
        rspice_app_state::state::SimulationRunIntent::ManualDeck;
    let mut controller = SimulationController::new();
    controller
        .prepare_deck_run(state)
        .map_err(|error| error.to_string())?;
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
        .map_err(|error| error.to_string())?;
    Ok(dispatch.executable_netlist().to_owned())
}

/// Save writes a Latin-1 deck with CRLF line endings back as Latin-1 with
/// CRLF, the edit in it, at once: no dialog asks anything first.
#[test]
fn a_latin1_crlf_deck_saves_as_latin1_crlf_with_the_edit_and_no_dialog() {
    let folder = Folder::new();
    let path = folder.write(
        "legacy.cir",
        b"* r\xe9sistance\r\nV1 out 0 1\r\nR1 out 0 1k\r\n.op\r\n.end\r\n",
    );
    let mut app = app_with_deck(&path);
    let deck = app.state.code.netlist.open_deck.as_ref().expect("open");
    assert_eq!(deck.root.encoding, TextEncoding::Latin1);
    assert_eq!(deck.root.line_ending, LineEnding::CrLf);
    assert_eq!(
        app.state.model.simulation.source.netlist_content,
        "* r\u{e9}sistance\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
        "the editor's lines end as the editor ends them"
    );
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "* r\u{e9}sistance\nV1 out 0 1\nR1 out 0 2k\n.op\n.end\n".to_owned()
    ));

    app.execute_command_key(ProjectCommand::Save.key());

    assert_eq!(
        std::fs::read(&path).expect("the deck"),
        b"* r\xe9sistance\r\nV1 out 0 1\r\nR1 out 0 2k\r\n.op\r\n.end\r\n".to_vec()
    );
    assert!(!app.state.application_modal_open(), "Save opened a dialog");
    assert!(!app.state.workbench.confirmation_dialog.visible);
    assert!(
        !app.state
            .code
            .netlist
            .open_deck
            .as_ref()
            .unwrap()
            .root
            .dirty
    );
}

/// A deck something else changed since it was opened is not overwritten:
/// Save says so and keeps the edit.
#[test]
fn a_deck_changed_on_disk_is_never_overwritten() {
    let folder = Folder::new();
    let path = folder.write("amp.cir", "* amp\nR1 out 0 1k\n.op\n.end\n");
    let mut app = app_with_deck(&path);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        "* amp\nR1 out 0 2k\n.op\n.end\n".to_owned()
    ));
    std::fs::write(&path, "* amp\nR1 out 0 10k\n.op\n.end\n").expect("another writer");

    app.execute_command_key(ProjectCommand::Save.key());

    assert_eq!(
        std::fs::read_to_string(&path).expect("the deck"),
        "* amp\nR1 out 0 10k\n.op\n.end\n"
    );
    assert_eq!(
        app.state.prompt_text(),
        Some(
            "amp.cir changed on disk since you opened it, so it was not saved: reload it, \
             keep yours or compare the two."
        )
    );
    assert!(
        app.state
            .code
            .netlist
            .open_deck
            .as_ref()
            .unwrap()
            .root
            .dirty
    );
}

/// The project keeps where its deck is, relative to the project's folder
/// when the deck is under it.
#[test]
fn the_project_file_keeps_a_deck_under_its_folder_by_its_relative_path() {
    let folder = Folder::new();
    let project = folder.0.join("amp.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );
    let deck = folder.write("decks/amp.cir", "* amp\nR1 out 0 1k\n.op\n.end\n");
    assert!(app.state.open_deck_at(&deck, false));
    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );

    let stored: String = std::fs::read_to_string(&project)
        .expect("the project file")
        .split_whitespace()
        .collect();
    assert!(
        stored.contains(r#"{"kind":"file","path":"decks/amp.cir"}"#),
        "the deck is stored by its relative path: {stored}"
    );
    assert!(
        !stored.contains("R1out01k"),
        "the deck's text is not in the project"
    );
}

/// A deck that reads one library twice, for two of its sections, runs
/// (NL-3).
#[test]
fn a_deck_reading_two_sections_of_one_library_runs() {
    let folder = Folder::new();
    folder.write(
        "m.lib",
        "* corners\n.lib tt\n.model dmod d is=1e-14\n.endl tt\n\
         .lib res\n.model dbig d is=1e-12\n.endl res\n",
    );
    let deck = folder.write(
        "two.cir",
        "* two sections\n.lib m.lib tt\n.lib m.lib res\nV1 in 0 1\nR1 in a 1k\n\
         D1 a 0 dmod\nD2 a 0 dbig\n.op\n.end\n",
    );
    let mut state = opened(&deck);
    let netlist = dispatched(&mut state).expect("the deck runs");
    let netlist = netlist.to_lowercase();
    assert!(
        netlist.contains("dmod") && netlist.contains("dbig"),
        "both sections are read: {netlist}"
    );
}

/// A library whose section selects another section of itself is what the
/// engine reads, not a cycle (NL-6).
#[test]
fn a_library_selecting_its_own_section_runs() {
    let folder = Folder::new();
    folder.write(
        "m.lib",
        "* corners\n.lib tt\n.lib m.lib common\n.model dmod d is=1e-14\n.endl tt\n\
         .lib common\n.model dcom d is=2e-14\n.endl common\n",
    );
    let deck = folder.write(
        "own.cir",
        "* own section\n.lib m.lib tt\nV1 in 0 1\nR1 in a 1k\nD1 a 0 dmod\nD2 a 0 dcom\n.op\n.end\n",
    );
    let mut state = opened(&deck);
    let netlist = dispatched(&mut state).expect("the deck runs");
    assert!(netlist.to_lowercase().contains("dcom"), "{netlist}");
}

/// A 20 MiB include runs, and the project keeps none of its text (NL-4).
#[test]
fn a_large_include_runs_and_stays_out_of_the_project_file() {
    const MARKER: &str = "* padding line of the large include\n";
    let folder = Folder::new();
    let mut include = MARKER.repeat(20 * 1024 * 1024 / MARKER.len() + 1);
    include.push_str(".model dbig d is=1e-14\n");
    folder.write("big.inc", include);
    let deck = folder.write(
        "big.cir",
        "* big include\n.include big.inc\nV1 in 0 1\nR1 in a 1k\nD1 a 0 dbig\n.op\n.end\n",
    );
    let project = folder.0.join("big.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );
    assert!(app.state.open_deck_at(&deck, false));

    dispatched(&mut app.state).expect("the deck runs");
    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );
    let stored = std::fs::read_to_string(&project).expect("the project file");
    assert!(
        !stored.contains("padding line of the large include"),
        "the include's text is not in the project"
    );
    assert!(stored.len() < 1024 * 1024, "{} bytes", stored.len());
}

/// An ngspice deck with a `.control` block and an HSPICE deck with
/// `.option post=2` reach the engine as written: no profile stands between
/// a deck and the engine (NL-5).
#[test]
fn ngspice_and_hspice_decks_reach_the_engine_as_written() {
    for (name, deck) in [
        (
            "ngspice.cir",
            "* ngspice\nV1 out 0 1\nR1 out 0 1k\n.op\n.control\nrun\nprint v(out)\n.endc\n.end\n",
        ),
        (
            "hspice.sp",
            "* hspice\n.option post=2\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
        ),
    ] {
        let folder = Folder::new();
        let mut state = opened(&folder.write(name, deck));
        let netlist = dispatched(&mut state).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert!(
            !netlist.to_lowercase().contains("profile"),
            "{name}: {netlist}"
        );
    }
}

/// Opening a deck adds it to the open project and puts it in front; the
/// project's runs stay.
#[test]
fn opening_a_deck_adds_it_to_the_open_project_and_keeps_its_runs() {
    const DECK: &str = "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let folder = Folder::new();
    let path = folder.write("bias.cir", DECK);
    let mut state = AppState::default();
    state.model.simulation.start_run();

    assert!(state.open_deck_at(&path, false));

    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    assert_eq!(state.model.simulation.source.netlist_content, DECK);
    assert_eq!(state.model.workspace.content.decks.len(), 1);
    assert!(state.model.workspace.content.decks_dirty);
    assert!(state.model.workspace.any_dirty());
    state
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .expect("a project with a deck is a valid project");
    assert_eq!(state.model.simulation.retained.runs.len(), 1);
}

/// With no project open, a deck opens in a new project named after it.
#[test]
fn opening_a_deck_with_no_project_makes_a_project_around_it() {
    let folder = Folder::new();
    let path = folder.write("bias.cir", "new\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
    let mut state = AppState::default();
    let original_project_id = state.model.workspace.content.project.id();
    state.model.simulation.start_run();

    assert!(state.open_deck_at(&path, true));

    assert_ne!(
        state.model.workspace.content.project.id(),
        original_project_id
    );
    assert_eq!(state.model.workspace.content.project.name(), "bias");
    assert!(!state.model.simulation.retained.has_results());
    assert_eq!(state.workbench.workspace, Workspace::Netlist);
}

/// A deck that cannot be read says why and leaves the project as it was.
#[test]
fn a_deck_that_cannot_be_read_says_why_and_changes_nothing() {
    let folder = Folder::new();
    let mut state = AppState::default();
    let original_project_id = state.model.workspace.content.project.id();

    assert!(!state.open_deck_at(&folder.0.join("missing.cir"), true));

    assert_eq!(
        state.model.workspace.content.project.id(),
        original_project_id
    );
    let prompt = state.prompt_text().expect("the reason is shown");
    assert!(prompt.contains("missing.cir cannot be read"), "{prompt}");
}

/// A project open read-only takes no deck.
#[test]
fn opening_a_deck_refuses_a_read_only_project_without_change() {
    let folder = Folder::new();
    let path = folder.write("new.cir", "new\n.op\n.end\n");
    let mut state = AppState::default();
    state.model.safe_mode.active = true;
    state.model.safe_mode.applied = rspice_app_state::state::write_policy::LocalSafeModeOptions {
        open_project_read_only: true,
        ..Default::default()
    };

    assert!(!state.open_deck_at(&path, false));
    assert!(state.model.workspace.content.decks.is_empty());
    assert!(state.code.netlist.open_deck.is_none());
}

/// The open deck's unsaved edits are never dropped by opening another.
#[test]
fn another_deck_waits_for_the_open_decks_edits_to_be_saved() {
    let folder = Folder::new();
    let first = folder.write("first.cir", "first\n.op\n.end\n");
    let second = folder.write("second.cir", "second\n.op\n.end\n");
    let mut state = opened(&first);
    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        "first edited\n.op\n.end\n".to_owned()
    ));

    assert!(!state.open_deck_at(&second, false));

    assert_eq!(
        state.prompt_text(),
        Some("Save first.cir before opening another deck.")
    );
    assert_eq!(
        state.code.netlist.open_deck.as_ref().unwrap().root.text,
        "first edited\n.op\n.end\n"
    );
}
