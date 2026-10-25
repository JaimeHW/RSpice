//! App integration fixtures run through the same preparation and authorization as Run.

use super::test_host::RunTestHost;
use super::{SimulationController, SimulationRunIntent, SimulationRunner};
use crate::simulation::RunHost;
use crate::state::AnalysisResultProvenance;
use rspice_app_services::workbench::workflows::export_workflow::{
    ExportWorkflowIo, SaveDialogConfig,
};
use rspice_design::drc::DrcResult;
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation::results::SimulationResult;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub mod dc_history;
pub mod monte_carlo;
pub mod pvt;
pub mod saved_outputs;

/// The divider on the signed test technology, its design check current and
/// clean.
pub fn state_with_current_clean_drc() -> RunTestHost {
    let mut state = RunTestHost::default();
    state.provision_test_project_technology_contract();
    rspice_app_state::workbench::examples::load_example(
        "Voltage Divider",
        &mut state.model.schematic,
    );
    let mut result = DrcResult::new();
    result.completed = true;
    state.model.design_checks.drc_results = Some(result);
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();
    state
}

pub fn start_manual_deck(source: &str) -> SimulationRunner {
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(source.to_owned());
    start_manual_deck_of(&state)
}

/// Validate and start the manual deck of `state`, as Run does.
fn start_manual_deck_of(state: &impl RunHost) -> SimulationRunner {
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(state)
        .expect("fixture source passes complete preparation");
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
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

pub fn wait_until_finished_unpolled(runner: &SimulationRunner) {
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

pub fn run_manual_deck(source: &str) -> SimulationResult {
    finish(start_manual_deck(source))
}

/// Validate and run the manual deck of `state`, the project it belongs to
/// and its editor's compile among what it reads.
pub fn run_manual_deck_of(state: &impl RunHost) -> SimulationResult {
    finish(start_manual_deck_of(state))
}

fn finish(mut runner: SimulationRunner) -> SimulationResult {
    wait_until_finished_unpolled(&runner);
    runner
        .poll_result()
        .expect("finished task publishes a terminal result")
        .expect("fixture simulation succeeds")
}

pub fn run_manual_batch(source: &str) -> crate::state::SimulationRun {
    run_manual_batch_with_lifecycle(source, SimulationRunLifecycle::Completed)
}

pub fn run_manual_batch_with_lifecycle(
    source: &str,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    let mut state = RunTestHost::default();
    state.model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    state.model.workspace.content.netlist_source = Some(source.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("fixture source passes complete preparation");
    run_batch(state, controller, expected_lifecycle)
}

pub fn run_generated_batch(
    mut state: RunTestHost,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    state.model.simulation.execution.run_intent = SimulationRunIntent::SimulateRunSet;
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

/// Run `source` as a manual deck through the real controller and hand back
/// the host it ran in, a new one of `H`, for a test that reads what the run
/// published.
pub fn run_manual_batch_session<H: RunHost + Default>(source: &str) -> H {
    let mut state = H::default();
    let model = state.model_mut();
    model.simulation.execution.run_intent = SimulationRunIntent::ManualDeck;
    model.workspace.content.netlist_source = Some(source.to_owned());
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("fixture source passes complete preparation");
    run_batch_session(&mut state, controller, SimulationRunLifecycle::Completed);
    state
}

fn run_batch(
    mut state: RunTestHost,
    controller: SimulationController,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    run_batch_session(&mut state, controller, expected_lifecycle)
}

/// Run the batch `controller` prepared to its end on `state`, whichever
/// host it is, and check what it retained.
fn run_batch_session(
    state: &mut impl RunHost,
    mut controller: SimulationController,
    expected_lifecycle: SimulationRunLifecycle,
) -> crate::state::SimulationRun {
    let export_io = MockExportWorkflowIo::default();
    state.model_mut().simulation.execution.trigger_simulation = true;
    controller.update(state, &export_io);
    let deadline = Instant::now() + Duration::from_secs(60);
    while controller.has_active_batch() {
        assert!(Instant::now() < deadline, "fixture batch timed out");
        std::thread::sleep(Duration::from_millis(1));
        controller.update(state, &export_io);
    }
    let model = state.model();
    assert_eq!(
        model.simulation.retained.runs.len(),
        1,
        "{}",
        model.simulation.execution.status
    );
    let run = model.simulation.retained.runs[0].clone();
    assert_eq!(run.lifecycle, expected_lifecycle, "{:?}", run.analyses);
    assert!(!run.analyses.is_empty());
    if expected_lifecycle == SimulationRunLifecycle::Completed {
        for analysis in &run.analyses {
            assert!(analysis.success, "{:?}", analysis.error_message);
        }
    }
    crate::io::capture_simulation_results(&model.simulation)
        .validate()
        .expect("checked batch retains valid project history");
    run
}

pub fn run_resolved_task(
    task: rspice_simulation::execution::ResolvedTaskDispatch,
) -> Result<SimulationResult, Box<rspice_simulation::error::SimulationError>> {
    let mut runner = SimulationRunner::new();
    runner.start_prepared(task, false)?;
    wait_until_finished_unpolled(&runner);
    runner
        .poll_result()
        .expect("finished task publishes a terminal result")
        .map_err(Box::new)
}

#[derive(Debug, Default)]
pub struct MockExportWorkflowIo {
    pub writes: RefCell<Vec<(PathBuf, String)>>,
    pub create_only_writes: RefCell<Vec<(PathBuf, String)>>,
}

impl ExportWorkflowIo for MockExportWorkflowIo {
    fn show_save_dialog(&self, _config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        Ok(None)
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.writes
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_string()));
        Ok(())
    }

    fn write_new_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.create_only_writes
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_string()));
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

pub fn bind_test_run_running(
    state: &mut impl RunHost,
    controller: &mut SimulationController,
    run_sequence: u64,
) {
    let model = state.model_mut();
    let run = model
        .simulation
        .retained
        .run_by_sequence_mut(run_sequence)
        .expect("test execution has a retained run");
    run.mark_running().expect("test run enters running state");
    let identity = run.execution_identity();
    controller.current_run_id = Some(run_sequence);
    model.simulation.execution.active_execution = identity;
}

pub fn synthetic_result_provenance() -> AnalysisResultProvenance {
    AnalysisResultProvenance::new(
        crate::product::AnalysisInstanceId::new(),
        crate::product::ObjectRevision::INITIAL,
        crate::product::ContentDigest::from_bytes([0x39; 32]),
        Vec::new(),
    )
    .expect("synthetic prepared-task provenance is valid")
}

/// Trapezoidal 1010 clock at 1 ns, 20-80 % edge of 50 ps.
pub fn clock(bits: usize, t_start: f64) -> (Vec<f64>, Vec<f64>) {
    const UI: f64 = 1e-9;
    const DT: f64 = 10e-12;
    const RAMP: f64 = 50e-12 / 0.6;

    let events: Vec<(f64, f64)> = (0..bits)
        .map(|n| {
            (
                t_start + n as f64 * UI,
                if n.is_multiple_of(2) { 1.0 } else { 0.0 },
            )
        })
        .collect();
    let t_end = t_start + (bits as f64 + 1.0) * UI;
    let count = (t_end / DT) as usize;
    let mut time = Vec::with_capacity(count);
    let mut signal = Vec::with_capacity(count);
    let mut cursor = 0usize;
    let mut held = 0.0;
    for index in 0..count {
        let t = index as f64 * DT;
        while cursor < events.len() && t >= events[cursor].0 + 0.5 * RAMP {
            held = events[cursor].1;
            cursor += 1;
        }
        let value = match events.get(cursor) {
            Some(&(edge, next)) if t > edge - 0.5 * RAMP => {
                let alpha = ((t - (edge - 0.5 * RAMP)) / RAMP).clamp(0.0, 1.0);
                held + alpha * (next - held)
            }
            _ => held,
        };
        time.push(t);
        signal.push(value);
    }
    (time, signal)
}
