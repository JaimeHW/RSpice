//! Decks the project keeps as text, through the whole application: where
//! there is no file system, New deck makes one and puts it in front; what is
//! typed is the project's at once; the deck runs as written; and a project
//! file holding one opens it again with its text.

use crate::feature_tests::replay::Replay;
use crate::workbench::app_state::AppState;
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_state::workbench::workspace::Workspace;

const DECK: &str = "* divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n";

/// The deck in front prepared as Run deck prepares it and handed to the
/// engine: the netlist the engine reads, or why it was refused.
fn dispatched(state: &mut AppState) -> Result<String, String> {
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

/// New deck makes `deck1.cir` in a project of its own, in front of the
/// Netlist workspace; an edit is the project's text at once and leaves the
/// project with unsaved changes; and the deck runs as typed.
#[test]
fn new_deck_keeps_its_text_in_the_project_and_runs_it() {
    let mut state = AppState::default();
    assert!(state.new_stored_deck(), "{:?}", state.prompt_text());
    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        state.model.simulation.source.netlist_content,
        "* deck1\n\n.end\n"
    );
    let decks = &state.model.workspace.content.decks;
    assert_eq!(decks.len(), 1);
    assert_eq!(decks[0].name(), "deck1.cir");

    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        DECK.to_owned()
    ));
    let content = &state.model.workspace.content;
    assert_eq!(
        content.decks[0]
            .stored_deck()
            .map(|deck| deck.text.as_str()),
        Some(DECK)
    );
    assert!(content.decks_dirty, "the project has unsaved changes");
    assert!(
        !state.code.netlist.open_deck.as_ref().unwrap().root.dirty,
        "nothing waits for a file to be saved"
    );

    let netlist = dispatched(&mut state).expect("the deck runs");
    assert!(
        netlist.contains("R2 out 0 1k"),
        "the engine reads the deck as typed: {netlist}"
    );
}

/// A second New deck takes the next free name and leaves the first deck in
/// the project.
#[test]
fn a_second_new_deck_takes_the_next_name() {
    let mut state = AppState::default();
    assert!(state.new_stored_deck());
    assert!(state.new_stored_deck());
    let names = state
        .model
        .workspace
        .content
        .decks
        .iter()
        .map(rspice_app_state::state::ProjectDeck::name)
        .collect::<Vec<_>>();
    assert_eq!(names, ["deck1.cir", "deck2.cir"]);
    assert_eq!(
        state.model.simulation.source.netlist_content,
        "* deck2\n\n.end\n"
    );
}

/// The project file a save or a checkpoint writes holds the deck's text, and
/// a session that opens that file finds the deck in front of the Netlist
/// workspace with what was typed.
#[test]
fn a_stored_deck_opens_again_from_the_project_file() {
    let mut replay = Replay::new();
    assert!(replay.app.state.new_stored_deck());
    let state = &mut replay.app.state;
    assert!(edit_deck_file(
        &mut state.model,
        &mut state.code.netlist,
        DECK.to_owned()
    ));
    let snapshot = rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(state)
        .expect("the project snapshots");
    let text = rspice_app_services::io::project_io::serialize_project_file(&snapshot)
        .expect("the project file is written");
    let compact: String = text.split_whitespace().collect();
    assert!(
        compact.contains(r#""kind":"stored","name":"deck1.cir""#),
        "the project file holds the deck: {compact}"
    );

    let loaded = rspice_app_services::io::project_io::load_project_text(&text, None)
        .expect("the project file loads");
    let mut reopened = Replay::new();
    let state = &mut reopened.app.state;
    state.model.library_manager = loaded.file.libraries;
    state.model.workspace = rspice_app_state::state::ProjectWorkspace::from_parts(
        loaded.file.workspace,
        loaded.workspace_session,
    );
    state.workbench.activate(Workspace::Netlist);
    reopened.frames(Vec::new());
    let state = &reopened.app.state;
    assert_eq!(
        state
            .code
            .netlist
            .open_deck
            .as_ref()
            .map(|deck| deck.root.text.as_str()),
        Some(DECK),
        "the deck opens with what was typed"
    );
    assert_eq!(state.model.simulation.source.netlist_content, DECK);
}
