//! What a run writes into its own log.

use super::*;
use crate::diagnostics::LogSource;
use crate::simulation::controller::test_execution;

/// Two analyses, so the first task's completion dispatches the second in the
/// same call.
const TWO_ANALYSES: &str = "* run log fixture\n\
    V1 in 0 DC 1\n\
    R1 in out 1k\n\
    R2 out 0 1k\n\
    .op\n\
    .dc V1 0 1 0.5\n\
    .end\n";

fn start_manual_batch(source: &str) -> (AppState, SimulationController) {
    let mut state = AppState::default();
    state.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the fixture deck prepares");
    state.simulation.execution.trigger_simulation = true;
    controller.update(&mut state, &MockExportWorkflowIo::default());
    (state, controller)
}

fn inventory_lines(state: &AppState, run_sequence: u64) -> usize {
    state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained")
        .log
        .lines()
        .iter()
        .filter(|line| line.source == LogSource::Engine && line.message.starts_with("Circuit:"))
        .count()
}

/// A task that finishes between the frame's drain and its poll still has its
/// last lines in the engine queue. Dispatching the next task clears that
/// queue, so the poll takes them into the run first.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tasks_last_engine_lines_land_in_its_run_before_the_next_task_dispatches() {
    crate::diagnostics::engine_log::studio_logger_for_tests();
    let export_io = MockExportWorkflowIo::default();
    let (mut state, mut controller) = start_manual_batch(TWO_ANALYSES);
    let run_sequence = controller.current_run_id.expect("the batch has a run");
    assert_eq!(controller.total_analyses, 2);

    test_execution::wait_until_finished_unpolled(&controller.runner);
    controller.poll_completion(&mut state, &export_io);
    assert_eq!(
        controller.current_analysis_idx, 2,
        "the second task was dispatched"
    );
    assert_eq!(
        inventory_lines(&state, run_sequence),
        1,
        "the first task's circuit line reached its run"
    );

    test_execution::wait_until_finished_unpolled(&controller.runner);
    controller.poll_completion(&mut state, &export_io);
    assert!(!controller.has_active_batch());
    assert_eq!(inventory_lines(&state, run_sequence), 2);
}
