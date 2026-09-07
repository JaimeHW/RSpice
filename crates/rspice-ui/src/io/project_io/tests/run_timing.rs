//! A restored run's duration is retained evidence, never a new stopwatch.

use super::*;

fn timed_results() -> ProjectSimulationResults {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffffff"),
        ]),
    );
    seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run.timestamp = 1234.5;
    run.elapsed_time = 12.5;
    let mut state = SimulationState::default();
    state.runs.push(run);
    ProjectSimulationResults::from_state(&state)
}

#[test]
fn restored_terminal_run_timing_is_unchanged_by_repeated_sealing() {
    for terminal in [
        SimulationRunLifecycle::Completed,
        SimulationRunLifecycle::Failed,
        SimulationRunLifecycle::Aborted,
        SimulationRunLifecycle::Interrupted,
    ] {
        let mut results = timed_results();
        results.runs[0].lifecycle = Some(terminal);
        results.runs[0].success = terminal == SimulationRunLifecycle::Completed;
        let mut restored = results.into_simulation_state().unwrap();
        let before = serde_json::to_value(ProjectSimulationResults::from_state(&restored)).unwrap();
        restored.runs[0].finish_lifecycle(terminal).unwrap();
        assert_eq!(restored.runs[0].elapsed_time, 12.5);
        assert_eq!(
            serde_json::to_value(ProjectSimulationResults::from_state(&restored)).unwrap(),
            before,
            "re-sealing {terminal:?} changed persisted history"
        );
    }
}

#[test]
fn restored_interrupted_run_timing_does_not_count_time_since_the_old_timestamp() {
    for active in [
        SimulationRunLifecycle::Preparing,
        SimulationRunLifecycle::Running,
        SimulationRunLifecycle::Cancelling,
    ] {
        let mut results = timed_results();
        results.runs[0].lifecycle = Some(active);
        results.runs[0].success = false;
        let mut restored = results.into_simulation_state().unwrap();
        let run = &mut restored.runs[0];
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Interrupted);
        assert!(!run.success);
        assert_eq!(run.elapsed_time, 12.5);
        run.finish_lifecycle(SimulationRunLifecycle::Interrupted)
            .unwrap();
        assert_eq!(run.elapsed_time, 12.5);
        assert!(run.mark_running().is_err());
    }
}

#[test]
fn negative_run_timing_is_rejected_before_project_publication() {
    let mut libraries = LibraryManager::with_primitives();
    let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let mut project =
        ProjectFile::new_with_simulation_results(workspace, libraries, timed_results());
    serialize_project_file(&project).unwrap();
    project.simulation_results.runs[0].elapsed_time = -0.125;
    assert!(
        serialize_project_file(&project).is_err(),
        "a negative run duration cannot be published as valid evidence"
    );
}
