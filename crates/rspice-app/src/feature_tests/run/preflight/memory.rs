//! Run on the plan weighed against the memory of the machine it runs on, a
//! machine the tests pretend: a run the check authorizes that needs more
//! than is free waits, for Run anyway or Stop, with its numbers on the
//! status line and in the band above the plan, as a deck's run waits.

#![cfg(not(target_arch = "wasm32"))]

use egui::accesskit::Role;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_simulation::execution::run_memory::{
    MachineMemory, forget_machine_memory_here, pretend_machine_memory_here,
};

use super::engine::{binned_library, inverter_on_binned_pdk};
use super::*;
use crate::feature_tests::replay::Replay;

const GB: u64 = 1024 * 1024 * 1024;

/// A machine with memory installed for any design and none free.
const BUSY: MachineMemory = MachineMemory {
    installed_bytes: 16 * GB,
    free_bytes: 0,
};

/// Whether a run of the plan was asked for or has started.
fn run_started(app: &RSpiceApp) -> bool {
    let simulation = &app.state.model.simulation;
    simulation.execution.trigger_simulation
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
}

/// Whether a frame's accessibility tree has a label that starts `text`.
fn says_first(output: &egui::FullOutput, text: &str) -> bool {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update
                .nodes
                .iter()
                .any(|(_, node)| node.label().is_some_and(|label| label.starts_with(text)))
        })
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

/// The inverter on its PDK, its plan's Run pressed on the Design workspace
/// of a machine with none of its memory free, the check settled.
fn held_plan() -> RSpiceApp {
    pretend_machine_memory_here(BUSY);
    let mut app = inverter_on_binned_pdk(binned_library(None));
    app.state.workbench.workspace = Workspace::Design;
    start(&mut app.state, &mut app.simulation_controller, true);
    app.settle_preflight();
    app
}

/// Run on the plan prepares and checks its run, and the run, authorized,
/// waits: nothing dispatches, the Simulate workspace comes to the front, the
/// status line says what the run needs and what is free, and the band above
/// the plan says it. Run anyway there starts the run authorized.
#[test]
fn a_plan_run_needing_more_than_is_free_waits_for_run_anyway() {
    let app = held_plan();
    let warning = app
        .state
        .runs
        .preflight
        .held_for_memory
        .expect("the run waits for an answer");
    assert_eq!(warning.memory, BUSY);
    assert!(
        app.state
            .runs
            .preflight
            .report
            .as_ref()
            .is_some_and(|report| report.blockers.is_empty() && report.prepared.is_some()),
        "the check authorized the run"
    );
    assert!(!run_started(&app), "nothing dispatches");
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert!(app.command_key_enabled(ids::STOP_SIMULATION));
    assert!(!app.command_key_enabled(SimulateCommand::Run.key()));

    let mut replay = Replay::of(app);
    let output = replay.frame(Vec::new());
    let status = format!("Warning: {}", warning.status());
    assert!(says(&output, &status), "the status line says {status:?}");
    assert!(says(&output, &warning.numbers()), "the band says it");
    assert!(
        says_first(&output, "Stop the run waiting for memory"),
        "the run control is Stop"
    );
    for _ in 0..3 {
        replay.frame(Vec::new());
    }
    assert!(!run_started(&replay.app), "still nothing dispatches");

    replay.click("Run anyway", Role::Button);
    assert!(replay.app.state.runs.preflight.held_for_memory.is_none());
    assert!(run_started(&replay.app), "Run anyway starts the run");
    assert!(
        replay
            .app
            .state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry
                .message
                .ends_with("though it needs more memory than is free.")),
        "the log says it ran anyway"
    );
    forget_machine_memory_here();
}

/// Stop drops the run held for memory: nothing of it runs, the log says so,
/// and Run asks again.
#[test]
fn stop_drops_a_plan_run_held_for_memory() {
    let mut app = held_plan();
    assert!(app.state.runs.preflight.held_for_memory.is_some());

    app.execute_command_key(ids::STOP_SIMULATION);

    assert!(app.state.runs.preflight.held_for_memory.is_none());
    assert!(!run_started(&app), "nothing runs");
    assert!(
        app.state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.ends_with("before it ran.")),
        "the log says it stopped"
    );
    assert!(app.command_key_enabled(SimulateCommand::Run.key()));
    forget_machine_memory_here();
}
