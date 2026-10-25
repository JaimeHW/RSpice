//! The run controller observed through a whole session.

use super::session_of;
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::test_execution::state_with_current_clean_drc;
use crate::simulation::controller::tests::{MockExportWorkflowIo, bind_test_run_running};
use crate::simulation::multi_run::AnalysisSpec;
use crate::state::{
    AnalysisResult, AnalysisResultSourceDomain, AnalysisType, SimulationRunLifecycle,
};
use crate::workbench::app_state::AppState;

#[test]
fn controller_plan_run_receipt_survives_production_project_round_trip() {
    let mut state = session_of(state_with_current_clean_drc());
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("clean plan preflight");
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(&mut state);
    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("controller owns the first prepared plan task");
    let expected_source_id = task_provenance.source_instance_id();
    state
        .model
        .simulation
        .active_run_mut()
        .expect("prepared plan run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_provenance(task_provenance),
        );
    controller.abort();

    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("production snapshot accepts controller run");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("controller plan run serializes");
    let loaded =
        crate::io::project_io::load_project_text(&json, None).expect("controller plan run reloads");
    let loaded_plan_id = loaded
        .file
        .execution_context
        .as_ref()
        .expect("execution context retained")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan retained")
        .id();
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("controller plan history restores");
    let run = &restored.retained.runs[0];
    let receipt = run.prepared_receipt().expect("prepared receipt retained");
    let result_provenance = run.analyses[0]
        .provenance
        .as_ref()
        .expect("result provenance retained");

    assert_eq!(
        receipt.source_domain(),
        AnalysisResultSourceDomain::SimulationPlan
    );
    assert_eq!(receipt.simulation_plan_id(), Some(loaded_plan_id));
    assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
    assert_eq!(result_provenance.source_instance_id(), expected_source_id);
    assert_eq!(
        result_provenance.prepared_snapshot_digest(),
        receipt.prepared_snapshot_digest()
    );
}

#[test]
fn design_context_reset_discards_pending_controller_result() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    state.model.simulation.start_run();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller.runner = crate::simulation::controller::test_execution::start_manual_deck(
        "Old design\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
    );
    crate::simulation::controller::test_execution::wait_until_finished_unpolled(&controller.runner);

    state.clear_design_execution_context();
    controller.update(&mut state, &export_io);

    assert!(
        !state.model.simulation.retained.has_results(),
        "stale result from previous design should be discarded"
    );
    assert_eq!(state.model.log_buffer.len(), 0);
    assert_eq!(state.model.simulation.execution.status, "");
    assert!(!controller.is_running());
}

#[test]
fn stale_cancellation_request_never_mutates_run_lifecycle() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.model.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    state
        .model
        .simulation
        .request_abort_active_run()
        .expect("request is initially bound");
    state.model.simulation.execution.active_execution = None;

    controller.update(&mut state, &export_io);

    assert_eq!(
        state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .unwrap()
            .lifecycle,
        SimulationRunLifecycle::Running
    );
    assert!(state.model.simulation.execution.abort_request.is_none());
    assert!(
        state
            .workbench
            .prompt
            .as_ref()
            .is_some_and(|prompt| prompt.text.starts_with("Nothing to cancel")),
        "a stale request is answered on the status bar, not logged"
    );
}
