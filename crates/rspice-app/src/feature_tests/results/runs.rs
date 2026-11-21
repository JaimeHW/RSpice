//! A retained run as the reader handles it in Results: its identity across a
//! save and reopen.

use crate::workbench::app_state::AppState;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::{
    run_generated_batches, state_with_current_clean_drc,
};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// Run the session's plan once, its transient answering `out`, and finish
/// the run, its samples held in memory.
fn run_plan(state: &mut AppState, out: [f64; 5]) {
    let mut controller = SimulationController::new();
    controller
        .preflight_run_set(state)
        .expect("clean plan preflight");
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(state);
    let provenance = controller
        .current_provenance
        .clone()
        .expect("the plan's prepared task");
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_provenance(provenance)
                .with_waveforms(vec![
                    RetainedTrace::new("V(in)", TIME.to_vec(), vec![1.0; 5], "#ffffff"),
                    RetainedTrace::new("V(out)", TIME.to_vec(), out.to_vec(), "#ffffff"),
                ]),
        );
    controller.abort();
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the run finishes");
}

/// A session with one finished run of its plan, Results in front.
fn session_with_a_run() -> AppState {
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    run_plan(&mut state, [0.0, 0.4, 1.0, 0.4, 0.0]);
    state
}

/// Save `state`'s project to a scratch folder and open it again in a new
/// session. The reopened runs read their samples from the results folder
/// beside the project file, so the folder stays while the test holds it.
fn save_and_reopen(mut state: AppState) -> (AppState, ScratchFolder) {
    let root = std::env::temp_dir().join(format!("rspice-runs-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&root).expect("a scratch folder");
    let path = root.join("runs.rspiceproj");
    assert!(state.save_project_to_path(&path), "the project saves");
    drop(state);
    let mut state = AppState::default();
    let reopened = state.load_project_from_path(&path);
    assert!(reopened, "the project reopens");
    (state, ScratchFolder(root))
}

/// A scratch folder removed when the test lets go of it.
struct ScratchFolder(std::path::PathBuf);

impl Drop for ScratchFolder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_run_saved_from_memory_reopens_with_the_digest_it_had() {
    let state = session_with_a_run();
    let run = &state.model.simulation.retained.runs[0];
    let (dataset, before) = (run.dataset_id, run.dataset_content_digest());

    let (state, _folder) = save_and_reopen(state);
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .expect("the run reopens");
    assert_eq!(
        run.dataset_content_digest(),
        before,
        "the same samples have the same digest wherever they are held"
    );
}

/// A run the engine solved keeps its samples in the store from the start,
/// and the save copies its segments beside the project file.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_solved_run_reopens_with_the_digest_it_had() {
    let host = run_generated_batches(state_with_current_clean_drc(), 1);
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    let run = &state.model.simulation.retained.runs[0];
    assert!(
        !run.holds_samples_in_memory(),
        "a solved run is in the store"
    );
    let (dataset, before) = (run.dataset_id, run.dataset_content_digest());

    let (state, _folder) = save_and_reopen(state);
    let run = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(dataset)
        .expect("the run reopens");
    assert_eq!(
        run.dataset_content_digest(),
        before,
        "the same samples have the same digest wherever they are held"
    );
}
