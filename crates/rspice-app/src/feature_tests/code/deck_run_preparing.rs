//! Run on a deck whose run is prepared away from the frame, as the
//! application prepares one: the press answers at once with the run
//! preparing, Stop drops it before anything runs, and an edit made while it
//! prepares is not the run's.

#![cfg(not(target_arch = "wasm32"))]

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_code::workbench::documents::netlist_document::edit_deck_file;
use rspice_app_runs::simulation::controller::DeckPreparationHold;
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

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
