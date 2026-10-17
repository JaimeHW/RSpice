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

/// The session's log holds the run's start and its end, both anchored to it;
/// the run's own log opens with its header, has a line per analysis, and
/// closes with the same summary the session's end line states.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_run_writes_its_start_and_end_in_the_session_and_its_account_in_its_own_log() {
    let export_io = MockExportWorkflowIo::default();
    let (mut state, mut controller) = start_manual_batch(TWO_ANALYSES);
    let run_sequence = controller.current_run_id.expect("the batch has a run");
    while controller.has_active_batch() {
        test_execution::wait_until_finished_unpolled(&controller.runner);
        controller.update(&mut state, &export_io);
    }

    let session: Vec<_> = state.log_buffer.entries().collect();
    assert_eq!(session.len(), 2, "{session:?}");
    for entry in &session {
        assert_eq!(entry.source, LogSource::Run);
        assert_eq!(
            entry.anchor,
            Some(crate::diagnostics::LogAnchor::ResultRun { run_sequence })
        );
    }
    let analyses = [
        SimulationController::analysis_name_for_spec(&AnalysisSpec::dc_op()),
        "DC Sweep",
    ];
    assert_eq!(
        session[0].message,
        format!("Run {run_sequence} started: {}", analyses.join(", "))
    );

    let run = state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Completed);
    let lines = run.log.lines();
    let header = &lines[0].message;
    assert!(
        header.starts_with(&format!("Run {run_sequence} \u{b7} RSpice "))
            && header.ends_with(" \u{b7} desktop"),
        "{header}"
    );
    assert_eq!(run.analyses.len(), 2);
    for analysis in &run.analyses {
        assert!(
            lines.iter().any(|line| line.source == LogSource::Run
                && line.message.starts_with(&format!("{}: ", analysis.label))
                && line.message.ends_with(" s")),
            "a line with {}'s time: {lines:?}",
            analysis.label
        );
    }
    let summary = lines.last().expect("the log closes with its summary");
    assert_eq!(summary.severity, crate::diagnostics::LogSeverity::Info);
    assert!(
        summary
            .message
            .starts_with(&format!("Run {run_sequence} completed in ")),
        "{}",
        summary.message
    );
    assert_eq!(session[1].message, summary.message);
    assert_eq!(session[1].severity, summary.severity);
}

/// A typed deck's own parse warnings are advised in its run's log, as a
/// headless run of the same deck advises them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_typed_decks_parse_warnings_are_warnings_in_its_runs_log() {
    crate::diagnostics::engine_log::studio_logger_for_tests();
    let export_io = MockExportWorkflowIo::default();
    let (mut state, mut controller) = start_manual_batch(
        "* dropped control command\n\
         V1 in 0 DC 1\n\
         R1 in 0 1k\n\
         .op\n\
         .frobnicate\n\
         .control\n\
         print v(in)\n\
         .endc\n\
         .end\n",
    );
    let run_sequence = controller.current_run_id.expect("the deck has a run");
    while controller.has_active_batch() {
        test_execution::wait_until_finished_unpolled(&controller.runner);
        controller.update(&mut state, &export_io);
    }
    let run = state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    // The parser logs no line for a dropped control command, and logs one
    // for an unknown dot-command on every parse; each is stated once.
    for (warning, line) in [("control command 'print' ignored", 7), ("'.FROBNICATE'", 5)] {
        let stated: Vec<_> = run
            .log
            .lines()
            .iter()
            .filter(|row| row.message.contains(warning))
            .collect();
        assert_eq!(stated.len(), 1, "{warning}: {:?}", run.log.lines());
        assert_eq!(stated[0].severity, crate::diagnostics::LogSeverity::Warning);
        assert_eq!(stated[0].source, LogSource::Run);
        assert!(
            stated[0].message.starts_with(&format!("line {line}: ")),
            "{}",
            stated[0].message
        );
    }
}

/// A live preview that cannot be shown says why in the run's log, once,
/// though the preview is tried every frame.
#[test]
fn a_live_preview_that_cannot_be_shown_says_why_once_in_its_runs_log() {
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let mut controller = SimulationController::new();
    controller.current_run_id = Some(run_sequence);

    controller.publish_live_preview(&mut state);
    controller.publish_live_preview(&mut state);

    let run = state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    let lines: Vec<_> = run
        .log
        .lines()
        .iter()
        .map(|line| (line.severity, line.message.as_str()))
        .collect();
    assert_eq!(
        lines,
        [(
            crate::diagnostics::LogSeverity::Warning,
            "Live preview stopped: the running analysis could not be identified"
        )]
    );
    assert!(state.log_buffer.is_empty());

    controller.live_transient.clear();
    controller.publish_live_preview(&mut state);
    let run = state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("the run is retained");
    assert_eq!(run.log.lines().len(), 2, "the next task tries again");
}
