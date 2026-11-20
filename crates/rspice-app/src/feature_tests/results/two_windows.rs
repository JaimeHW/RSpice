//! Two windows on two runs, each drawn in turn on every frame.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::prepare_viewer_state;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::{
    WorkspaceDocumentId, WorkspaceLayoutState,
};

fn transient(id: u64) -> AnalysisResult {
    AnalysisResult::new(id, AnalysisType::Transient, "Transient").with_waveforms(vec![
        RetainedTrace::new(
            "V(out)",
            vec![0.0, 0.5, 1.0],
            vec![0.0, 1.0, 0.0],
            "#55aaff",
        ),
    ])
}

fn run(id: u64) -> SimulationRun {
    let mut run = SimulationRun::new(id);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses = vec![transient(1), transient(2)];
    run
}

/// Each window put its run back on screen every frame, which reset the run
/// to its first analysis and cleared the cursors twice a frame. Now neither
/// window changes the other's analysis, and the cursors hold.
#[test]
fn two_windows_on_two_runs_keep_the_cursors_and_each_runs_analysis() {
    let mut app = RSpiceApp::test_instance();
    let (first, second) = (run(1), run(2));
    let datasets = [first.dataset_id, second.dataset_id];
    app.state.model.simulation.retained.runs = vec![second, first].into();
    app.state.workbench.activate(Workspace::Results);
    let session = &mut app.state.workbench.window_session;
    let windows = datasets.map(|dataset| {
        let window = session.create_window(
            "Results",
            Workspace::Results,
            WorkspaceLayoutState::default(),
            false,
        );
        session.state_mut(window).unwrap().active_document =
            Some(WorkspaceDocumentId::ResultDataset(dataset));
        window
    });
    let draw = |app: &mut RSpiceApp, window| {
        assert!(app.project_application_window(window, WorkspaceLayoutState::default()));
        prepare_viewer_state(&mut app.state.results_parts());
    };
    for window in windows {
        draw(&mut app, window);
        assert!(app.state.model.simulation.select_analysis(1));
        prepare_viewer_state(&mut app.state.results_parts());
    }
    app.state.results.results.session.cursors.a = Some(0.25);
    app.state.results.results.session.cursors.b = Some(0.75);
    let placed = app.state.results.results.session.cursors;

    for _ in 0..100 {
        for (window, dataset) in windows.into_iter().zip(datasets) {
            draw(&mut app, window);
            let simulation = &app.state.model.simulation;
            assert_eq!(
                simulation.active_run().map(|run| run.dataset_id),
                Some(dataset)
            );
            assert_eq!(simulation.view.active_analysis_idx, Some(1));
        }
    }
    assert_eq!(app.state.results.results.session.cursors, placed);
}
