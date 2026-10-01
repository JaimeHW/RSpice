//! App integration fixtures run through the same preparation and authorization as Run.

use super::{AppState, SimulationController, SimulationRunIntent, SimulationRunner};
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation::results::SimulationResult;
use std::time::{Duration, Instant};

pub(crate) mod monte_carlo;
pub(crate) mod pvt;
pub(crate) mod saved_outputs;

pub(crate) fn start_manual_deck(source: &str) -> SimulationRunner {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("fixture source passes complete preparation");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("authorize the unchanged fixture source");
    let mut tasks = dispatch.into_tasks();
    assert_eq!(tasks.len(), 1, "fixture requires one independent analysis");
    let task = tasks
        .pop_front()
        .unwrap()
        .resolve_dependency_artifacts(&Default::default())
        .expect("fixture has no unresolved dependencies");
    let mut runner = SimulationRunner::new();
    runner
        .start_prepared(task, false)
        .expect("start checked task");
    runner
}

pub(crate) fn wait_until_finished_unpolled(runner: &SimulationRunner) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while runner.is_running() {
        assert!(Instant::now() < deadline, "fixture execution timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        !runner.can_accept_prepared_task(),
        "completed result must remain pending until polled"
    );
}

pub(crate) fn run_manual_deck(source: &str) -> SimulationResult {
    let mut runner = start_manual_deck(source);
    wait_until_finished_unpolled(&runner);
    runner
        .poll_result()
        .expect("finished task publishes a terminal result")
        .expect("fixture simulation succeeds")
}

pub(crate) fn run_manual_batch(source: &str) -> crate::state::SimulationRun {
    run_manual_batch_with_lifecycle(source, SimulationRunLifecycle::Completed)
}

pub(crate) fn run_manual_batch_with_lifecycle(
    source: &str,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.content.netlist_source = Some(source.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("fixture source passes complete preparation");
    run_batch(state, controller, expected_lifecycle)
}

pub(crate) fn run_generated_batch(
    mut state: AppState,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    state.simulation.run_intent = SimulationRunIntent::SimulateRunSet;
    state.sync_active_schematic_to_workspace();
    let checks = state
        .run_active_design_checks()
        .expect("check the fixture design");
    assert!(!checks.has_errors(), "{checks:?}");
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("prepare the complete fixture plan");
    run_batch(state, controller, expected_lifecycle)
}

fn run_batch(
    mut state: AppState,
    mut controller: SimulationController,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    let export_io = super::tests::MockExportWorkflowIo::default();
    state.simulation.trigger_simulation = true;
    controller.update(&mut state, &export_io);
    let deadline = Instant::now() + Duration::from_secs(60);
    while controller.has_active_batch() {
        assert!(Instant::now() < deadline, "fixture batch timed out");
        std::thread::sleep(Duration::from_millis(1));
        controller.update(&mut state, &export_io);
    }
    assert_eq!(
        state.simulation.runs.len(),
        1,
        "{}",
        state.simulation.status
    );
    let run = state.simulation.runs[0].clone();
    assert_eq!(run.lifecycle, expected_lifecycle, "{:?}", run.analyses);
    assert!(!run.analyses.is_empty());
    if expected_lifecycle == SimulationRunLifecycle::Completed {
        for analysis in &run.analyses {
            assert!(analysis.success, "{:?}", analysis.error_message);
        }
    }
    crate::io::capture_simulation_results(&state.simulation)
        .validate()
        .expect("checked batch retains valid project history");
    run
}

pub(crate) fn run_resolved_task(
    task: rspice_simulation::execution::ResolvedTaskDispatch,
) -> Result<SimulationResult, rspice_simulation::error::SimulationError> {
    let mut runner = SimulationRunner::new();
    runner.start_prepared(task, false)?;
    wait_until_finished_unpolled(&runner);
    runner
        .poll_result()
        .expect("finished task publishes a terminal result")
}
