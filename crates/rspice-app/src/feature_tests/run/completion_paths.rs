//! Completed runs observed through a whole session: a manual run saved with
//! its project, and failures filed in an open project's Problems.

use crate::simulation::controller::SimulationController;
use crate::state::{AnalysisResult, AnalysisResultSourceDomain, AnalysisType};
use crate::workbench::app_state::AppState;

#[test]
fn controller_manual_run_receipt_survives_production_project_round_trip() {
    let mut state = AppState::default();
    state.model.workspace.content.netlist_source =
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    state.model.simulation.execution.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");
    controller.start_simulation(&mut state);
    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("controller owns the prepared manual task");
    let expected_source_id = task_provenance.source_instance_id();
    state
        .model
        .simulation
        .active_run_mut()
        .expect("prepared manual run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
        );
    controller.abort();

    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("production snapshot accepts controller manual run");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("controller manual run serializes");
    let loaded = crate::io::project_io::load_project_text(&json, None)
        .expect("controller manual run reloads");
    assert!(
        loaded.file.execution_context.is_some(),
        "manual receipt must remain independent of the unrelated retained plan"
    );
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("controller manual history restores");
    let run = &restored.retained.runs[0];
    let receipt = run.prepared_receipt().expect("manual receipt retained");
    let result_provenance = run.analyses[0]
        .provenance
        .as_ref()
        .expect("manual result provenance retained");

    assert_eq!(
        receipt.source_domain(),
        AnalysisResultSourceDomain::ManualDeck
    );
    assert_eq!(receipt.simulation_plan_id(), None);
    assert!(matches!(
        receipt.source_check_receipt(),
        crate::state::PreparedSourceCheckReceipt::ManualSourceCheck(_)
    ));
    assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
    assert_eq!(result_provenance.source_instance_id(), expected_source_id);
    assert_eq!(
        result_provenance.prepared_snapshot_digest(),
        receipt.prepared_snapshot_digest()
    );
}

#[test]
fn a_failed_analysis_is_written_once_in_its_runs_log_and_filed_in_problems() {
    let mut state = AppState::default();
    state.open_project_session();
    let attribution = rspice_results::convergence_attribution::from_core(
        &rspice_core::diagnostics::ConvergenceDiagnostic {
            class: rspice_core::diagnostics::ConvergenceFailureClass::NewtonNonConvergence,
            sites: vec![
                rspice_core::diagnostics::ConvergenceSite {
                    name: "OUT".to_owned(),
                    kind: rspice_core::diagnostics::ConvergenceSiteKind::Node,
                    residual: Some(4.5),
                },
                rspice_core::diagnostics::ConvergenceSite {
                    name: "V1".to_owned(),
                    kind: rspice_core::diagnostics::ConvergenceSiteKind::Branch,
                    residual: None,
                },
            ],
            elided_sites: 0,
            failure_message: "Convergence failed after 100 iterations".to_owned(),
        },
    );

    let run_sequence = state.model.simulation.start_run().id;
    SimulationController::report_failed_analysis(
        &mut state,
        "Transient",
        "Transient failed: Convergence failed after 100 iterations",
        &Some(attribution),
        Some(run_sequence),
    );

    assert_eq!(
        run_log_messages(&state, run_sequence),
        vec![(
            crate::diagnostics::LogSeverity::Error,
            "Transient failed: Convergence failed after 100 iterations".to_owned()
        )],
        "the run's log records the event once, in the engine's own prose"
    );
    assert!(
        state.model.log_buffer.is_empty(),
        "a run's failure is its own log's, not the session's"
    );
    // The objects the engine named are the Problems record's route, which
    // `app_state::diagnostic_store` covers; the log's job here is the event.
    assert_eq!(
        state
            .model
            .diagnostics
            .published_run_failures(run_sequence)
            .len(),
        1
    );
}

/// The lines a run's own log holds, as (severity, text).
fn run_log_messages(
    state: &AppState,
    run_sequence: u64,
) -> Vec<(crate::diagnostics::LogSeverity, String)> {
    state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained")
        .log
        .lines()
        .iter()
        .map(|line| (line.severity, line.message.clone()))
        .collect()
}

#[test]
fn a_real_operating_point_refusal_is_filed_with_the_conductor_the_engine_named() {
    // Nothing here is injected: the deck is refused by the solver, the
    // attribution is the one `rspice-core` recorded (`engine/core.rs:1309`),
    // and the failure is written by the same call the run loop makes.
    let mut runner = crate::simulation::controller::test_execution::start_manual_deck(
        "current-driven floating node\ni1 0 out dc 1m\nc1 out 0 1u\n.op\n.end\n",
    );
    crate::simulation::controller::test_execution::wait_until_finished_unpolled(&runner);
    let error = runner
        .poll_result()
        .unwrap()
        .expect_err("a current-driven floating node has no operating point");
    let attribution = error
        .attribution()
        .cloned()
        .expect("the operating point is the analysis that names its refusals");

    assert!(
        attribution
            .nets()
            .any(|net| net.eq_ignore_ascii_case("out")),
        "the engine names the floating node"
    );

    let mut state = AppState::default();
    state.open_project_session();
    let run_sequence = state.model.simulation.start_run().id;
    let message = format!("Operating point failed: {error}");
    SimulationController::report_failed_analysis(
        &mut state,
        "Operating point",
        &message,
        &Some(attribution),
        Some(run_sequence),
    );

    assert_eq!(
        run_log_messages(&state, run_sequence),
        vec![(crate::diagnostics::LogSeverity::Error, message)],
        "an attributed failure writes the engine's prose once; the attribution is the record's"
    );
    assert_eq!(
        state
            .model
            .diagnostics
            .published_run_failures(run_sequence)
            .len(),
        1
    );
}

#[test]
fn a_failure_that_named_nothing_writes_one_line() {
    let mut state = AppState::default();
    state.open_project_session();
    let run_sequence = state.model.simulation.start_run().id;

    SimulationController::report_failed_analysis(
        &mut state,
        "Transient",
        "Transient failed: Parse error: unterminated .subckt",
        &None,
        Some(run_sequence),
    );

    assert_eq!(
        run_log_messages(&state, run_sequence).len(),
        1,
        "no attribution, no second line"
    );
}
