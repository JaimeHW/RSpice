//! Run, and Check and netlist, on a deck prepared away from the frame, as
//! the application prepares one: the press answers at once with the deck
//! preparing, Stop drops it before anything runs, an edit made while it
//! prepares is not the run's, and the deck's edits are saved away from the
//! press, a save that fails holding the run.

#![cfg(not(target_arch = "wasm32"))]

use super::netlist_workflow::{Folder, app_with_deck};
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_code::workbench::documents::deck_files;
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_runs::simulation::controller::{DeckAction, DeckPreparationHold};
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use std::path::{Path, PathBuf};

const DECK: &str = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

/// A session on the Netlist workspace with `DECK` open as `amp_tb.cir`,
/// the schematic's components gone, and the next deck preparation held
/// before it prepares anything.
fn held_session() -> (RSpiceApp, DeckPreparationHold) {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(
        app.state
            .open_test_deck(DECK.to_owned(), None, "amp_tb.cir")
    );
    app.state.workbench.workspace = Workspace::Netlist;
    let hold = app.simulation_controller.hold_next_deck_run_preparation();
    (app, hold)
}

/// Let the held preparation prepare, and wait until its worker has ended.
fn release(hold: DeckPreparationHold) {
    let DeckPreparationHold { release, ended } = hold;
    drop(release);
    // Disconnects once the worker has ended.
    let _ = ended.recv();
}

/// Whether a frame's accessibility tree says `text`.
fn says(output: &egui::FullOutput, text: &str) -> bool {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update
                .nodes
                .iter()
                .any(|(_, node)| node.label() == Some(text) || node.value() == Some(text))
        })
}

/// Whether a run of the deck has started: asked for, or dispatched.
fn deck_run_started(app: &RSpiceApp) -> bool {
    let simulation = &app.state.model.simulation;
    (simulation.execution.trigger_simulation
        && simulation.execution.run_intent == SimulationRunIntent::ManualDeck)
        || simulation.retained.executed_decks.iter().count() > 0
}

/// The text the deck's run ran.
fn text_run(app: &RSpiceApp) -> Option<String> {
    let handoff = &app.state.runs.manual_run;
    handoff
        .pending_run_buffer
        .as_ref()
        .or(handoff.last_run_buffer.as_ref())
        .map(|text| text.as_str().to_owned())
}

/// The press returns with the run preparing and nothing run: the status
/// bar says so and the run control is Stop. The run starts on the first
/// frame after it is prepared.
#[test]
fn the_press_answers_with_the_run_preparing() {
    let (mut app, hold) = held_session();
    let ctx = themed_context();

    app.execute_command_key(ids::RUN_SIMULATION);

    assert!(
        app.state.runs.preparing_deck.is_some(),
        "the run is preparing"
    );
    assert!(
        !deck_run_started(&app),
        "nothing runs before it is prepared"
    );
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(says(&output, "Preparing"), "the status bar says so");
    assert!(
        app.command_key_enabled(ids::STOP_SIMULATION),
        "Stop stops it"
    );
    assert!(!app.command_key_enabled(ids::RUN_SIMULATION));

    release(hold);
    frame(&ctx, &mut app, DESKTOP);
    assert!(app.state.runs.preparing_deck.is_none());
    assert!(deck_run_started(&app), "the prepared run starts");
}

/// Stop while the run prepares drops it: nothing is dispatched, then or
/// once the worker ends, and the notice says the run stopped.
#[test]
fn stop_while_preparing_runs_nothing_and_says_so() {
    let (mut app, hold) = held_session();
    let ctx = themed_context();

    app.execute_command_key(ids::RUN_SIMULATION);
    app.execute_command_key(ids::STOP_SIMULATION);

    assert!(app.state.runs.preparing_deck.is_none());
    assert_eq!(
        app.state.prompt_text(),
        Some("Stopped amp_tb.cir before it ran.")
    );
    release(hold);
    for _ in 0..2 {
        frame(&ctx, &mut app, DESKTOP);
    }
    assert!(!deck_run_started(&app), "a stopped run never starts");
    assert!(
        app.command_key_enabled(ids::RUN_SIMULATION),
        "Run is Run again"
    );
}

/// An edit made while the run prepares is not the run's: it runs the text
/// the press took, and is not refused for the edit.
#[test]
fn an_edit_while_preparing_does_not_reach_the_run() {
    let (mut app, hold) = held_session();
    let ctx = themed_context();

    app.execute_command_key(ids::RUN_SIMULATION);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        DECK.replace("1k", "2k"),
    ));
    release(hold);
    frame(&ctx, &mut app, DESKTOP);

    assert!(
        deck_run_started(&app),
        "the run is not refused for the edit"
    );
    assert_eq!(
        text_run(&app).as_deref(),
        Some(DECK),
        "it ran the text pressed"
    );
}

/// Check and netlist prepares the deck as Run does: the press returns with
/// the deck preparing and the status bar saying so, Stop drops it, and
/// nothing is dispatched, then or once the worker ends.
#[test]
fn check_while_preparing_behaves_as_run_does() {
    let (mut app, hold) = held_session();
    let ctx = themed_context();

    app.execute_command_key(ids::CHECK_AND_NETLIST);

    assert!(
        app.state
            .runs
            .preparing_deck
            .as_ref()
            .is_some_and(|deck| deck.action == DeckAction::Check),
        "the check is preparing"
    );
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(says(&output, "Preparing"), "the status bar says so");
    assert!(
        app.command_key_enabled(ids::STOP_SIMULATION),
        "Stop stops it"
    );
    assert!(!app.command_key_enabled(ids::CHECK_AND_NETLIST));
    assert!(!app.command_key_enabled(ids::RUN_SIMULATION));

    app.execute_command_key(ids::STOP_SIMULATION);
    assert!(app.state.runs.preparing_deck.is_none());
    assert_eq!(
        app.state.prompt_text(),
        Some("Stopped checking amp_tb.cir.")
    );
    release(hold);
    for _ in 0..2 {
        frame(&ctx, &mut app, DESKTOP);
    }
    assert!(!deck_run_started(&app), "a check dispatches nothing");
    assert_eq!(
        app.state.prompt_text(),
        Some("Stopped checking amp_tb.cir."),
        "a stopped check says nothing more"
    );
    assert!(app.command_key_enabled(ids::CHECK_AND_NETLIST));
}

/// A check prepared on a worker says what it found once it ends, and runs
/// nothing.
#[test]
fn a_check_on_a_worker_says_what_it_found() {
    let (mut app, hold) = held_session();
    let ctx = themed_context();

    app.execute_command_key(ids::CHECK_AND_NETLIST);
    release(hold);
    frame(&ctx, &mut app, DESKTOP);

    assert!(app.state.runs.preparing_deck.is_none());
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|notice| notice.starts_with("amp_tb.cir checked")),
        "{:?}",
        app.state.prompt_text()
    );
    assert!(!deck_run_started(&app), "a check runs nothing");
}

const EDITED: &str = "standalone\nV1 out 0 1\nR1 out 0 2k\n.op\n.end\n";

/// `DECK` as the file `amp.cir` in `folder`, open on the Netlist workspace
/// with `EDITED` typed over it.
fn edited_file_session(folder: &Folder) -> (RSpiceApp, PathBuf) {
    let path = folder.write("amp.cir", DECK);
    let mut app = app_with_deck(&path);
    app.state.workbench.activate(Workspace::Netlist);
    assert!(edit_deck_file(
        &mut app.state.model,
        &mut app.state.code.netlist,
        EDITED.to_owned()
    ));
    (app, path)
}

/// Make saving `amp.cir` in `folder` fail: a folder stands where the save
/// writes its temporary file. What a save that fails there says.
fn block_the_save(folder: &Folder) -> String {
    let blocker = folder.0.join(".amp.cir.saving");
    std::fs::create_dir(&blocker).expect("the blocker");
    let error = std::fs::write(&blocker, b"").expect_err("a folder is not written as a file");
    format!("amp.cir was not saved: {error}")
}

/// Run saves the deck's edits away from the press: the file is as it was
/// while the preparation is held, and once it ends the file holds the
/// edits, the tab is clean and the run runs them.
#[test]
fn run_saves_the_deck_away_from_the_press() {
    let folder = Folder::new();
    let (mut app, path) = edited_file_session(&folder);
    let hold = app.simulation_controller.hold_next_deck_run_preparation();
    let ctx = themed_context();

    app.execute_command_key(ids::RUN_SIMULATION);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the deck"),
        DECK,
        "the press writes nothing"
    );

    release(hold);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(std::fs::read_to_string(&path).expect("the deck"), EDITED);
    assert_eq!(
        deck_files::unsaved_before_run(&app.state.code.netlist),
        0,
        "the tab is clean"
    );
    assert!(deck_run_started(&app));
    assert_eq!(text_run(&app).as_deref(), Some(EDITED));
}

/// What a press of Run on a deck whose save fails comes to: the notice says
/// why as a save says it, nothing runs, the file is as it was and the tab
/// keeps its edits.
fn assert_the_failed_save_held_the_run(app: &RSpiceApp, path: &Path, message: &str) {
    assert_eq!(app.state.prompt_text(), Some(message));
    assert!(app.state.runs.preparing_deck.is_none());
    assert!(!deck_run_started(app), "nothing runs");
    assert_eq!(std::fs::read_to_string(path).expect("the deck"), DECK);
    assert_eq!(
        deck_files::unsaved_before_run(&app.state.code.netlist),
        1,
        "the tab keeps its edits"
    );
    assert!(
        app.command_key_enabled(ids::RUN_SIMULATION),
        "Run is Run again"
    );
}

/// A save that fails before the run holds it, where the press prepares the
/// run.
#[test]
fn a_failed_save_stops_the_run_with_the_saves_message() {
    let folder = Folder::new();
    let (mut app, path) = edited_file_session(&folder);
    let message = block_the_save(&folder);

    app.execute_command_key(ids::RUN_SIMULATION);

    assert_the_failed_save_held_the_run(&app, &path, &message);
}

/// A save that fails before the run holds it, where a worker prepares the
/// run.
#[test]
fn a_failed_save_on_a_worker_stops_the_run_with_the_saves_message() {
    let folder = Folder::new();
    let (mut app, path) = edited_file_session(&folder);
    let message = block_the_save(&folder);
    let hold = app.simulation_controller.hold_next_deck_run_preparation();
    let ctx = themed_context();

    app.execute_command_key(ids::RUN_SIMULATION);
    release(hold);
    frame(&ctx, &mut app, DESKTOP);

    assert_the_failed_save_held_the_run(&app, &path, &message);
}
