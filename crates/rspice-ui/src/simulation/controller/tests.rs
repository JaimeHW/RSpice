//! Tests for simulation orchestration.
//!
//! The cases cover the controller's transitions between requested, running,
//! and settled state, and assert that a run's results are only published once
//! the run that produced them has actually completed.

use super::*;
use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::simulation::plan::AnalysisKind;
use crate::state::{ComponentType, Point, PreparedSourceCheckReceipt, SimulationRunProvenance};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use std::cell::RefCell;
use std::path::Path;

#[test]
fn transient_specialized_views_never_outlive_their_retained_source() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    let x = (0..32)
        .map(|index| index as f64 * 1.0e-9)
        .collect::<Vec<_>>();
    let y = (0..32)
        .map(|index| ((index as f64) * 0.4).sin())
        .collect::<Vec<_>>();
    let retained = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::WaveformData::new("V(out)", x, y, "#ffffff"),
    ]);

    controller.populate_transient_post_views(&mut state, &retained);
    assert!(state.analysis.fft_state.has_data());

    let no_outputs = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    controller.populate_transient_post_views(&mut state, &no_outputs);
    assert!(!state.analysis.fft_state.has_data());
    assert!(state.analysis.cache_authority.fft.is_none());
}

#[test]
fn save_all_retains_engine_results_while_explicit_empty_retains_none() {
    let source = || {
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 0.5, 1.0], "#fff"),
            WaveformData::new("I(R1)", vec![0.0, 0.5, 1.0], vec![1.0, 0.5, 0.0], "#fff"),
        ])
    };
    let policy = |mode| crate::simulation::execution::SavePolicy::PlanOwned {
        output_selection_mode: mode,
        retained_dataset_limit: 10,
        maximum_storage_bytes: u64::MAX,
        live_streaming_enabled: true,
        retain_failure_diagnostics: true,
    };

    let mut explicit = SimulationController::new();
    explicit.current_save_policy = policy(crate::state::OutputSelectionMode::ExplicitOnly);
    let mut explicit_result = source();
    explicit.materialize_current_saved_outputs(&mut explicit_result);
    assert!(explicit_result.waveforms.is_empty());

    let mut all = SimulationController::new();
    all.current_save_policy = policy(crate::state::OutputSelectionMode::SaveAll);
    let selected = crate::state::SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        "V(out)",
        "V(out)",
        crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
        crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
        crate::state::SavedOutputPrecision::FullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
    )
    .expect("selected display output");
    all.current_saved_output_contracts =
        crate::simulation::output_contract::compile_saved_output_contracts(
            &selected,
            [(
                crate::product::AnalysisInstanceId::new(),
                &AnalysisSpec::Transient {
                    stop_time: 1.0,
                    step_time: 1.0,
                    start_time: 0.0,
                    max_timestep: None,
                    uic: false,
                },
            )],
        )
        .expect("display contract");
    let mut all_result = source();
    all.materialize_current_saved_outputs(&mut all_result);
    assert_eq!(all_result.waveforms.len(), 2);
    assert_eq!(all_result.waveforms[0].x.len(), 3);
    assert!(all_result.waveforms[0].visible);
    assert!(!all_result.waveforms[1].visible);
    assert_eq!(all_result.saved_output_receipts.len(), 1);
}

#[derive(Debug, Default)]
struct MockExportWorkflowIo {
    writes: RefCell<Vec<(PathBuf, String)>>,
    create_only_writes: RefCell<Vec<(PathBuf, String)>>,
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

fn state_with_current_drc_error() -> AppState {
    let mut state = AppState::default();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));

    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingGround,
        "missing ground",
        DrcLocation::Global,
    ));
    result.completed = true;
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    state
}

fn state_with_current_clean_drc() -> AppState {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    crate::workbench::examples::load_example("Voltage Divider", &mut state.schematic);
    let mut result = DrcResult::new();
    result.completed = true;
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    state
}

fn bind_test_run_running(
    state: &mut AppState,
    controller: &mut SimulationController,
    run_sequence: u64,
) {
    let run = state
        .simulation
        .run_by_sequence_mut(run_sequence)
        .expect("test execution has a retained run");
    run.mark_running().expect("test run enters running state");
    let identity = run.execution_identity();
    controller.current_run_id = Some(run_sequence);
    state.simulation.active_execution = identity;
}

fn bind_and_request_test_abort(state: &mut AppState, controller: &mut SimulationController) {
    let run_sequence = state
        .simulation
        .active_run()
        .expect("test abort has an active run")
        .id;
    bind_test_run_running(state, controller, run_sequence);
    state
        .simulation
        .request_abort_active_run()
        .expect("test abort binds to the active execution");
}

fn synthetic_sparameter_result() -> crate::simulation::SimulationResult {
    let frequencies = vec![1.0e6, 2.0e6];
    let mut waveforms = std::collections::HashMap::new();
    for (name, real, imag) in [
        ("S11", vec![0.1, 0.2], vec![0.0, 0.0]),
        ("S21", vec![0.3, 0.4], vec![0.01, 0.02]),
        ("S12", vec![0.5, 0.6], vec![0.03, 0.04]),
        ("S22", vec![0.7, 0.8], vec![0.05, 0.06]),
    ] {
        waveforms.insert(
            name.to_string(),
            crate::simulation::results::WaveformData::new_complex(
                name,
                frequencies.clone(),
                real,
                imag,
            ),
        );
    }

    crate::simulation::SimulationResult::Ac {
        frequencies,
        waveforms,
        measurements: Vec::new(),
    }
}

fn synthetic_dc_op_result() -> crate::simulation::SimulationResult {
    let mut result = crate::simulation::results::DcOpResult::default();
    result.node_voltages.insert("out".to_string(), 1.25);
    crate::simulation::SimulationResult::DcOp(Box::new(result))
}

fn synthetic_result_provenance() -> AnalysisResultProvenance {
    AnalysisResultProvenance::new(
        crate::product::AnalysisInstanceId::new(),
        crate::product::ObjectRevision::INITIAL,
        crate::product::ContentDigest::from_bytes([0x39; 32]),
        Vec::new(),
    )
    .expect("synthetic prepared-task provenance is valid")
}

#[test]
fn failed_result_retention_never_satisfies_prepared_dependencies() {
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let provenance = synthetic_result_provenance();
    let instance = provenance.source_instance_id();
    let failed = AnalysisResult::failed(
        1,
        AnalysisType::Reliability,
        "Reliability",
        "invalid retained evidence",
    );
    let mut controller = SimulationController::new();

    assert!(
        !controller
            .retain_completed_analysis(&mut state, Some(run_sequence), failed, provenance)
            .expect("failed result is retained as failure evidence")
    );
    assert!(!controller.successful_analysis_instances.contains(&instance));
    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("target run remains");
    assert!(!run.success);
    assert_eq!(run.analyses.len(), 1);
    assert!(!run.analyses[0].success);
}

#[test]
fn plan_owned_runtime_retention_enforces_authenticated_storage_ceiling() {
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let provenance = synthetic_result_provenance();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new(
            "V(out)",
            (0..128).map(|index| index as f64).collect::<Vec<_>>(),
            vec![1.0; 128],
            "#fff",
        ),
    ]);
    let required = analysis.retained_storage_bytes();
    let mut controller = SimulationController::new();
    controller.current_save_policy = crate::simulation::execution::SavePolicy::PlanOwned {
        output_selection_mode: crate::state::OutputSelectionMode::Automatic,
        retained_dataset_limit: 10,
        maximum_storage_bytes: required - 1,
        live_streaming_enabled: false,
        retain_failure_diagnostics: true,
    };

    let error = controller
        .retain_completed_analysis(&mut state, Some(run_sequence), analysis, provenance)
        .expect_err("oversized evidence must not enter the retained run");

    assert!(error.contains("authenticated"));
    assert!(error.contains("storage ceiling"));
    assert!(
        state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("target run remains")
            .analyses
            .is_empty()
    );
}

#[test]
fn provisional_live_result_obeys_the_authenticated_storage_ceiling() {
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let partial = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![WaveformData::new(
            "V(out)",
            (0..64).map(|index| index as f64).collect::<Vec<_>>(),
            vec![1.0; 64],
            "#fff",
        )])
        .with_provenance(synthetic_result_provenance());
    let mut controller = SimulationController::new();
    controller.current_save_policy = crate::simulation::execution::SavePolicy::PlanOwned {
        output_selection_mode: crate::state::OutputSelectionMode::Automatic,
        retained_dataset_limit: 10,
        maximum_storage_bytes: partial.retained_storage_bytes() - 1,
        live_streaming_enabled: true,
        retain_failure_diagnostics: true,
    };

    let error = controller
        .validate_analysis_retention(
            state
                .simulation
                .run_by_sequence(run_sequence)
                .expect("target run"),
            &partial,
        )
        .expect_err("live evidence must obey the same immutable storage ceiling");

    assert!(error.contains("storage ceiling"));
    assert!(
        state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("target run")
            .analyses
            .is_empty()
    );
}

fn exact_vec(values: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    result.extend_from_slice(values);
    assert_eq!(
        result.capacity(),
        result.len(),
        "test vectors must be tightly allocated so pointer reuse is meaningful"
    );
    result
}

#[test]
fn direct_trigger_is_blocked_by_current_drc_errors() {
    let mut state = state_with_current_drc_error();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();

    state.simulation.request_simulate_run_set();
    controller.update(&mut state, &export_io);

    assert!(!state.simulation.trigger_simulation);
    assert!(!state.simulation.is_running);
    assert_eq!(state.simulation.status, "Run blocked");
}

#[test]
fn controller_plan_run_is_sealed_with_exact_prepared_receipt_before_results() {
    let mut state = state_with_current_clean_drc();
    let plan = state.sim_setup.analysis_plan.as_ref().expect("stable plan");
    let plan_id = plan.id();
    let plan_revision = plan.revision();
    let task_id = plan
        .instances()
        .iter()
        .find(|instance| instance.enabled())
        .expect("enabled plan task")
        .id();
    let project_revision = state.workspace.project.revision();
    let mut controller = SimulationController::new();
    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("clean plan preflight");

    state.simulation.request_simulate_run_set();
    controller.start_simulation(&mut state);

    let run = state.simulation.active_run().expect("prepared run starts");
    let receipt = run
        .prepared_receipt()
        .expect("run is sealed before results");
    assert_eq!(
        receipt.source_domain(),
        AnalysisResultSourceDomain::SimulationPlan
    );
    assert_eq!(receipt.simulation_plan_id(), Some(plan_id));
    assert_eq!(receipt.project_revision(), project_revision);
    assert_eq!(receipt.prepared_snapshot_digest(), metadata.snapshot_digest);
    assert_eq!(receipt.source_content_digest(), metadata.source_digest);
    assert_eq!(
        receipt.source_check_receipt(),
        PreparedSourceCheckReceipt::SchematicDrc(metadata.receipt_digest)
    );
    assert_eq!(receipt.tasks().len(), 1);
    let task = &receipt.tasks()[0];
    assert_eq!(task.instance_id(), task_id);
    assert_eq!(task.source_revision(), plan_revision);
    assert_eq!(task.analysis_kind_tag(), 5);
    assert!(task.dependencies().is_empty());
    assert_ne!(
        task.config_digest(),
        crate::product::ContentDigest::from_bytes([0; 32])
    );
    assert!(run.analyses.is_empty());
    assert!(run.validate_provenance().is_ok());

    controller.abort();
}

#[test]
fn controller_plan_run_receipt_survives_production_project_round_trip() {
    let mut state = state_with_current_clean_drc();
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("clean plan preflight");
    state.simulation.request_simulate_run_set();
    controller.start_simulation(&mut state);
    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("controller owns the first prepared plan task");
    let expected_source_id = task_provenance.source_instance_id();
    state
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
        .execution_context
        .as_ref()
        .expect("execution context retained")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan retained")
        .id();
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("controller plan history restores");
    let run = &restored.runs[0];
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
    state.simulation.start_run();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller
        .runner
        .store_pending_result(Ok(synthetic_dc_op_result()))
        .expect("seed old pending result");

    state.clear_design_execution_context();
    controller.update(&mut state, &export_io);

    assert!(
        !state.simulation.has_results(),
        "stale result from previous design should be discarded"
    );
    assert_eq!(state.log_buffer.len(), 0);
    assert_eq!(state.simulation.status, "");
    assert!(!controller.is_running());
}

#[test]
fn design_epoch_reset_terminalizes_executor_owned_history() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);

    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    controller.update(&mut state, &export_io);

    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("interrupted history remains retained");
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Interrupted);
    assert!(!run.success);
    assert!(state.simulation.active_execution.is_none());
    assert!(state.simulation.abort_request.is_none());
}

#[test]
fn stale_cancellation_request_never_mutates_run_lifecycle() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    state
        .simulation
        .request_abort_active_run()
        .expect("request is initially bound");
    state.simulation.active_execution = None;

    controller.update(&mut state, &export_io);

    assert_eq!(
        state
            .simulation
            .run_by_sequence(run_sequence)
            .unwrap()
            .lifecycle,
        SimulationRunLifecycle::Running
    );
    assert!(state.simulation.abort_request.is_none());
    assert!(
        state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("stale or unbound"))
    );
}

#[test]
fn abort_trigger_discards_worker_aborted_result_without_failed_analysis() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    state.simulation.start_run();
    state.simulation.status = "Running".to_string();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller
        .runner
        .store_pending_result(Err(crate::simulation::runner::SimulationError::Aborted))
        .expect("seed worker abort result");
    bind_and_request_test_abort(&mut state, &mut controller);

    controller.update(&mut state, &export_io);

    assert_eq!(state.simulation.status, "Aborted");
    let run = state.simulation.active_run().expect("active run remains");
    assert!(
        run.analyses.is_empty(),
        "aborted worker result must not be recorded as a failed analysis: {:?}",
        run.analyses
    );
    assert!(!run.success);
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
}

#[test]
fn abort_trigger_discards_unpolled_success_result() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    state.simulation.start_run();
    state.simulation.status = "Running".to_string();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller
        .runner
        .store_pending_result(Ok(synthetic_dc_op_result()))
        .expect("seed unpolled success result");
    bind_and_request_test_abort(&mut state, &mut controller);

    controller.update(&mut state, &export_io);

    assert_eq!(state.simulation.status, "Aborted");
    let run = state.simulation.active_run().expect("active run remains");
    assert!(
        run.analyses.is_empty(),
        "success result that arrived before abort poll must not be recorded: {:?}",
        run.analyses
    );
    assert!(!run.success);
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
}

#[test]
fn completed_result_attaches_to_started_run_when_active_selection_changes() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let older_run_id = state.simulation.start_run().id;
    let started_run_id = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, started_run_id);
    assert!(
        state.simulation.select_run(1),
        "user can inspect an older run while a newer run is in flight"
    );
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_label =
        Some("DC Operating Point · point 2/3 · TT · param RLOAD=2k".to_owned());
    let provenance = synthetic_result_provenance();
    let expected_source_id = provenance.source_instance_id();
    controller.current_provenance = Some(provenance);
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller
        .runner
        .store_pending_result(Ok(synthetic_dc_op_result()))
        .expect("seed completed run result");

    controller.update(&mut state, &export_io);

    let older_run = state
        .simulation
        .run_by_sequence(older_run_id)
        .expect("older run remains");
    let started_run = state
        .simulation
        .run_by_sequence(started_run_id)
        .expect("started run remains");
    assert!(
        older_run.analyses.is_empty(),
        "completed analysis must not contaminate the selected historical run"
    );
    assert_eq!(started_run.analyses.len(), 1);
    assert_eq!(
        started_run.analyses[0].label,
        "DC Operating Point · point 2/3 · TT · param RLOAD=2k"
    );
    assert_eq!(
        started_run.analyses[0]
            .provenance
            .as_ref()
            .expect("completed result has prepared provenance")
            .source_instance_id(),
        expected_source_id
    );
    assert_eq!(
        state.simulation.active_run().map(|run| run.id),
        Some(started_run_id)
    );
    assert_eq!(started_run.lifecycle, SimulationRunLifecycle::Completed);
}

#[test]
fn completed_transient_result_reuses_owned_waveform_buffers_in_run_history() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    controller.current_spec = Some(AnalysisSpec::Transient {
        stop_time: 2.0e-9,
        step_time: 1.0e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    });
    controller.current_provenance = Some(synthetic_result_provenance());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;

    let time = exact_vec(&[0.0, 1.0e-9, 2.0e-9]);
    let time_ptr = time.as_ptr();
    let values = exact_vec(&[0.0, 0.5, 1.0]);
    let values_ptr = values.as_ptr();
    let mut waveforms = std::collections::HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        crate::simulation::results::WaveformData {
            name: "V(out)".to_string(),
            x_values: Vec::new(),
            y_values: values,
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        },
    );

    controller
        .runner
        .store_pending_result(Ok(crate::simulation::SimulationResult::Transient {
            time,
            waveforms,
            measurements: Vec::new(),
            periodic_state: None,
            convergence: Default::default(),
            events: Default::default(),
        }))
        .expect("seed completed transient result");

    controller.update(&mut state, &export_io);

    let analysis = state
        .simulation
        .active_analysis()
        .expect("completed transient analysis is selected");
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "V(out)")
        .expect("transient waveform is stored in run history");

    assert_eq!(
        waveform.x.iter().as_slice().as_ptr(),
        time_ptr,
        "time vector should move into run history instead of being copied"
    );
    assert_eq!(
        waveform.y.iter().as_slice().as_ptr(),
        values_ptr,
        "sample vector should move into run history instead of being copied"
    );
}

#[test]
fn completed_dc_sweep_result_reuses_owned_shared_axis_buffers_in_run_history() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    controller.current_spec = Some(AnalysisSpec::DcSweep {
        source_name: "V1".to_string(),
        start: 0.0,
        stop: 2.0,
        step: 1.0,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
    });
    controller.current_provenance = Some(synthetic_result_provenance());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;

    let sweep_values = exact_vec(&[0.0, 1.0, 2.0]);
    let sweep_ptr = sweep_values.as_ptr();
    let values = exact_vec(&[0.1, 0.2, 0.3]);
    let values_ptr = values.as_ptr();
    let mut waveforms = std::collections::HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        crate::simulation::results::WaveformData {
            name: "V(out)".to_string(),
            x_values: Vec::new(),
            y_values: values,
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        },
    );

    controller
        .runner
        .store_pending_result(Ok(crate::simulation::SimulationResult::DcSweep {
            sweep_var: "V1".to_string(),
            sweep_values,
            waveforms,
            measurements: Vec::new(),
        }))
        .expect("seed completed DC sweep result");

    controller.update(&mut state, &export_io);

    let analysis = state
        .simulation
        .active_analysis()
        .expect("completed DC sweep analysis is selected");
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "V(out)")
        .expect("DC sweep waveform is stored in run history");

    assert_eq!(
        waveform.x.iter().as_slice().as_ptr(),
        sweep_ptr,
        "shared sweep axis should move into run history instead of being copied"
    );
    assert_eq!(
        waveform.y.iter().as_slice().as_ptr(),
        values_ptr,
        "sweep samples should move into run history instead of being copied"
    );
}

#[test]
fn failed_completion_retains_exact_prepared_task_provenance() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    controller.current_spec = Some(AnalysisSpec::dc_op());
    let provenance = synthetic_result_provenance();
    let expected_source_id = provenance.source_instance_id();
    let expected_snapshot = provenance.prepared_snapshot_digest();
    controller.current_provenance = Some(provenance);
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller
        .runner
        .store_pending_result(Err(SimulationError::SolverError(
            "singular matrix".to_owned(),
        )))
        .expect("seed failed result");

    controller.update(&mut state, &export_io);

    let analysis = &state.simulation.active_run().expect("run remains").analyses[0];
    let restored = analysis
        .provenance
        .as_ref()
        .expect("failed result has prepared provenance");
    assert!(!analysis.success);
    assert_eq!(restored.source_instance_id(), expected_source_id);
    assert_eq!(restored.prepared_snapshot_digest(), expected_snapshot);
    assert_eq!(
        state.simulation.active_run().expect("failed run").lifecycle,
        SimulationRunLifecycle::Failed
    );
}

#[test]
fn failed_prerequisite_skips_dependent_prepared_task_with_exact_provenance() {
    use crate::product::ProcessCorner;
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::simulation::execution::{
        ExecutionPermitIssuer, ExecutionTargetCapabilities, PreparedDependencyBinding,
        PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy, SnapshotParts,
    };

    let prerequisite_id = crate::product::AnalysisInstanceId::new();
    let dependent_id = crate::product::AnalysisInstanceId::new();
    let task = |spec, line: &str| QueuedAnalysis {
        spec,
        config: None,
        spec_options: SpecExecutionOptions::default(),
        analysis_line: line.to_owned(),
        numeric_override: None,
    };
    let prerequisite = PreparedTask::new(
        prerequisite_id,
        ObjectRevision::INITIAL,
        Vec::new(),
        "Prerequisite",
        task(
            AnalysisSpec::Transient {
                stop_time: 1.0,
                step_time: 0.005,
                start_time: 0.0,
                max_timestep: None,
                uic: false,
            },
            ".tran 0.005 1",
        ),
    );
    let mut dependent = PreparedTask::new(
        dependent_id,
        ObjectRevision::INITIAL,
        vec![prerequisite_id],
        "Dependent",
        task(
            AnalysisSpec::Fourier {
                fundamental_freq: 2.0,
                num_harmonics: 4,
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                start_time: 0.0,
                stop_time: 1.0,
                compute_thd: true,
                normalize: false,
            },
            ".four 2 V(out)",
        ),
    );
    dependent.set_dependency_bindings(vec![PreparedDependencyBinding::transient_trajectory(
        prerequisite_id,
        prerequisite.source_revision(),
        prerequisite.config_digest(),
    )]);
    let snapshot = PreparedRunSnapshot::new(SnapshotParts {
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
        project_revision: 3,
        topology_revision: 4,
        source_digest: ContentDigest::from_bytes([0x71; 32]),
        reference_process: ProcessCorner::TT,
        reference_temperature_celsius: 27.0,
        run_set: None,
        tasks: vec![prerequisite, dependent],
        executable_netlist: "deck\n.op\n.end\n".to_owned(),
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: crate::state::PreparedSpecificationPolicy::default(),
        project_veriloga_runtimes: Default::default(),
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([0x72; 32])),
        advisories: Vec::new(),
        manual_source: None,
        cross_probe: None,
        touchstone_export: TouchstoneExportPolicy::disabled(),
        sealed_source_dependencies: Vec::new(),
    })
    .expect("dependency-ordered snapshot validates");
    let digest = snapshot.digest();
    let issuer = ExecutionPermitIssuer::default();
    let proof = issuer
        .issue(digest)
        .expect("permit issues")
        .consume(digest, digest)
        .expect("permit consumes");
    let mut tasks = snapshot
        .authorize_dispatch(proof)
        .expect("snapshot authorizes")
        .into_tasks();
    let failed_task = tasks.pop_front().expect("prerequisite task");

    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let failed_provenance = AnalysisResultProvenance::new(
        failed_task.instance_id(),
        failed_task.source_revision(),
        failed_task.snapshot_digest(),
        failed_task.dependencies().to_vec(),
    )
    .expect("failed prerequisite provenance");
    state
        .simulation
        .run_by_sequence_mut(run_sequence)
        .expect("active run")
        .add_analysis(
            AnalysisResult::failed(1, AnalysisType::Transient, "Prerequisite", "solver failed")
                .with_provenance(failed_provenance),
        );

    let mut controller = SimulationController::new();
    controller.current_run_id = Some(run_sequence);
    controller.current_analysis_idx = 1;
    controller.total_analyses = 2;
    controller.pending_analyses = tasks;
    controller.start_next_analysis(&mut state);

    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("completed run remains");
    assert_eq!(run.analyses.len(), 2);
    let skipped = run
        .find_analysis_by_source_instance(dependent_id)
        .expect("dependent receives a retained skipped result");
    assert!(!skipped.success);
    assert!(
        skipped
            .error_message
            .as_deref()
            .is_some_and(|message| message.contains("prerequisite analysis result"))
    );
    assert_eq!(
        skipped
            .provenance
            .as_ref()
            .expect("skipped provenance")
            .dependency_ids(),
        &[prerequisite_id]
    );
    assert_eq!(state.simulation.status, "Completed with errors");
    assert!(!controller.is_running());
}

#[test]
fn touchstone_auto_export_uses_export_workflow_io() {
    let mut state = AppState::default();
    state.simulation.start_run();

    let mut controller = SimulationController::new();
    controller.touchstone_export_policy = TouchstoneExportPolicy::enabled(
        2,
        PathBuf::from("designs"),
        std::ffi::OsString::from("amp"),
    )
    .expect("valid prepared Touchstone policy");
    controller.current_spec = Some(AnalysisSpec::SParameter {
        start_freq: 1.0e6,
        stop_freq: 2.0e6,
        points_per_unit: 2,
        sweep: FrequencySweep::Linear,
        z0: 50.0,
        ports: vec![
            SpPort {
                node_pos: "IN".to_string(),
                node_neg: "0".to_string(),
                z0: None,
            },
            SpPort {
                node_pos: "OUT".to_string(),
                node_neg: "0".to_string(),
                z0: Some(75.0),
            },
        ],
    });
    controller.current_analysis_idx = 1;
    // Live editor mutations after dispatch must not redirect or reformat
    // the prepared automatic export.
    state.schematic.current_file = Some(PathBuf::from("changed").join("redirect.sch"));
    let mut changed = crate::simulation::dialog::SpConfig::default();
    changed.touchstone_export = false;
    changed.touchstone_version = 1;
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&changed);

    let export_io = MockExportWorkflowIo::default();
    let prepared = controller
        .prepare_touchstone_export(&synthetic_sparameter_result(), 1)
        .expect("valid Touchstone export")
        .expect("enabled Touchstone export");
    SimulationController::commit_touchstone_export(&mut state, &export_io, prepared);

    assert!(export_io.writes.borrow().is_empty());
    let writes = export_io.create_only_writes.borrow();
    assert_eq!(writes.len(), 1);
    assert_eq!(
        writes[0].0,
        PathBuf::from("designs").join("amp_run0001_sp01.s2p")
    );
    assert!(writes[0].1.contains("[Version] 2.0"));
    assert!(writes[0].1.contains("[Reference] 5e1 7.5e1"));
}

#[test]
fn touchstone_native_completion_message_confirms_file_export() {
    let message = SimulationController::touchstone_export_completed_message(Path::new("amp.s2p"));
    assert!(message.contains("Exported Touchstone"));
    assert!(message.contains("amp.s2p"));
}

#[test]
fn ac_result_conversion_retains_complex_components_for_export() {
    let controller = SimulationController::new();
    let result = synthetic_sparameter_result();

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        result,
        AnalysisType::SParameter,
        "SP",
    );
    let magnitude = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "|S11|")
        .expect("magnitude trace exists");
    let complex = magnitude
        .complex
        .as_ref()
        .expect("magnitude trace retains complex source data");

    assert_eq!(complex.source_name, "S11");
    assert_eq!(&*complex.real, &[0.1, 0.2]);
    assert_eq!(&*complex.imag, &[0.0, 0.0]);
}

#[test]
fn ac_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
    let controller = SimulationController::new();
    let frequencies = vec![1.0, 10.0, 100.0];
    let mut waveforms = std::collections::HashMap::new();
    waveforms.insert(
        "V(bad_real)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(bad_real)",
            frequencies.clone(),
            vec![1.0, 2.0],
            vec![0.0, 0.0, 0.0],
        ),
    );
    waveforms.insert(
        "V(bad_imag)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(bad_imag)",
            frequencies.clone(),
            vec![1.0, 2.0, 3.0],
            vec![0.0, 0.0],
        ),
    );
    waveforms.insert(
        "V(good)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(good)",
            frequencies.clone(),
            vec![3.0, 4.0, 5.0],
            vec![4.0, 3.0, 0.0],
        ),
    );

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements: Vec::new(),
        },
        AnalysisType::Ac,
        "AC",
    );

    let names: Vec<_> = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.as_str())
        .collect();
    assert_eq!(names, vec!["|V(good)|", "phase(V(good))"]);
    assert!(
        analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.x.len() == waveform.y.len()),
        "converted AC traces must never pair mismatched x/y arrays"
    );
}

#[test]
fn noise_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
    let controller = SimulationController::new();
    let mut contributors = std::collections::HashMap::new();
    contributors.insert("good".to_string(), vec![1.0e-18, 2.0e-18, 3.0e-18]);
    contributors.insert("bad".to_string(), vec![1.0e-18, 2.0e-18]);

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Noise {
            frequencies: vec![1.0, 10.0, 100.0],
            output_noise: vec![2.0e-18, 3.0e-18],
            input_noise: Some(vec![1.0e-18, 1.5e-18, 2.0e-18]),
            contributors,
            summary: None,
            measurements: Vec::new(),
        },
        AnalysisType::Noise,
        "Noise",
    );

    let names: Vec<_> = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.as_str())
        .collect();
    assert_eq!(names, vec!["inoise", "noise(good)"]);
    assert!(
        analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.x.len() == waveform.y.len()),
        "converted noise traces must never pair mismatched x/y arrays"
    );
}

#[test]
fn advanced_result_conversion_retains_exact_family_metadata() {
    use crate::state::{AnalysisResultFamilyMetadata, MonteCarloVariableMetadata};

    let controller = SimulationController::new();
    let empty_waveforms = || std::collections::HashMap::new();

    let monte_carlo = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::MonteCarlo {
            seed: 0x5eed,
            runs_requested: 4,
            runs_completed: 3,
            num_failures: 1,
            all_converged: false,
            variables: vec![crate::simulation::results::MonteCarloVariableResult {
                name: "V(out)".to_owned(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
                histogram: vec![1, 2],
                bin_edges: vec![0.85, 1.0, 1.15],
            }],
        },
        AnalysisType::MonteCarlo,
        "MC",
    );
    assert_eq!(
        monte_carlo.family_metadata,
        Some(AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 0x5eed,
            runs_requested: 4,
            runs_completed: 3,
            failures: 1,
            all_converged: false,
            variables: vec![MonteCarloVariableMetadata {
                name: "V(out)".to_owned(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
            }],
        })
    );
    assert_eq!(monte_carlo.waveforms[0].name, "hist(V(out))");

    let parametric = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Parametric {
            target: "PARAM rload".to_owned(),
            sweep_values: vec![1_000.0, 2_000.0],
            waveforms: empty_waveforms(),
            num_failures: 1,
        },
        AnalysisType::Parametric,
        "STEP",
    );
    assert_eq!(
        parametric.family_metadata,
        Some(AnalysisResultFamilyMetadata::Parametric {
            target: "PARAM rload".to_owned(),
            sweep_values: vec![1_000.0, 2_000.0],
            failed_points: 1,
        })
    );

    let corner = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Corner {
            x_values: vec![0.0, 1.0],
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![-40.0, 125.0],
            corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
            waveforms: empty_waveforms(),
            num_failures: 0,
        },
        AnalysisType::Corner,
        "Corner",
    );
    assert_eq!(
        corner.family_metadata,
        Some(AnalysisResultFamilyMetadata::Corner {
            x_values: vec![0.0, 1.0],
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![-40.0, 125.0],
            corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
            failed_corners: 0,
        })
    );

    let reliability = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Reliability {
            years: vec![1.0, 5.0, 10.0],
            waveforms: empty_waveforms(),
            device_results: vec![crate::simulation::ReliabilityResult {
                device_id: "M1".to_owned(),
                stress: crate::simulation::StressMetrics {
                    avg_vgs_stress: 1.2,
                    avg_vds_stress: 1.8,
                    avg_temp: 358.15,
                    duration: 3_600.0,
                },
                shifts: std::collections::HashMap::from([
                    ("1y".to_owned(), crate::simulation::ParamShift::default()),
                    ("5y".to_owned(), crate::simulation::ParamShift::default()),
                    ("10y".to_owned(), crate::simulation::ParamShift::default()),
                ]),
            }],
        },
        AnalysisType::Reliability,
        "Reliability",
    );
    assert_eq!(
        reliability.family_metadata,
        Some(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0, 10.0],
        })
    );
    assert!(matches!(
        reliability.result_payload,
        Some(AnalysisResultPayload::Reliability { ref devices }) if devices.len() == 1
    ));

    let optimization = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            waveforms: empty_waveforms(),
            best_cost: 0.125,
            best_variables: std::collections::HashMap::from([
                ("w".to_owned(), 2.0e-6),
                ("l".to_owned(), 180.0e-9),
            ]),
            converged: true,
        },
        AnalysisType::Optimization,
        "Optimization",
    );
    assert_eq!(
        optimization.family_metadata,
        Some(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            best_cost: 0.125,
            best_variables: std::collections::BTreeMap::from([
                ("l".to_owned(), 180.0e-9),
                ("w".to_owned(), 2.0e-6),
            ]),
            converged: true,
        })
    );

    let soa = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Soa {
            time: vec![0.0, 1.0e-9],
            waveforms: empty_waveforms(),
            violations: Vec::new(),
            evaluations: vec![crate::services::safety::SoAEvaluation {
                device_id: "M1".to_owned(),
                parameter: crate::services::safety::SoAParameter::Vgs,
                limit_value: 1.8,
                worst_actual_value: 1.0,
                worst_time: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum gate-source voltage".to_owned(),
                verdict: crate::services::safety::SoARuleVerdict::Pass,
            }],
        },
        AnalysisType::Soa,
        "SOA",
    );
    assert_eq!(
        soa.family_metadata,
        Some(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0e-9],
        })
    );
    assert!(matches!(
        soa.result_payload,
        Some(AnalysisResultPayload::Soa {
            ref evaluations,
            ref violations,
        }) if evaluations.len() == 1 && violations.is_empty()
    ));
}

#[test]
fn scalar_and_complex_analysis_conversion_retains_exact_typed_payloads() {
    use crate::state::{
        AnalysisResultPayload, ComplexResultValue, SensitivityResultMode, SensitivityResultRow,
    };

    let controller = SimulationController::new();
    let pole_zero = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::PoleZero {
            poles: vec![(-1.0, 2.0), (-1.0, -2.0)],
            zeros: vec![(-3.0, 0.0)],
            gain: 4.0,
        },
        AnalysisType::PoleZero,
        "PZ",
    );
    assert_eq!(
        pole_zero.result_payload,
        Some(AnalysisResultPayload::PoleZero {
            poles: vec![
                ComplexResultValue {
                    real: -1.0,
                    imaginary: 2.0,
                },
                ComplexResultValue {
                    real: -1.0,
                    imaginary: -2.0,
                },
            ],
            zeros: vec![ComplexResultValue {
                real: -3.0,
                imaginary: 0.0,
            }],
            gain: 4.0,
        })
    );
    assert!(pole_zero.has_data());

    let sensitivity = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Sensitivity {
            output: "V(out)".to_owned(),
            ac_mode: true,
            frequency_hz: Some(10_000.0),
            sensitivities: std::collections::HashMap::from([
                ("width".to_owned(), 2.0),
                ("length".to_owned(), -1.0),
            ]),
            normalized: std::collections::HashMap::from([
                ("width".to_owned(), 0.5),
                ("length".to_owned(), -0.25),
            ]),
        },
        AnalysisType::Sensitivity,
        "SENS",
    );
    assert_eq!(
        sensitivity.result_payload,
        Some(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 10_000.0,
            },
            rows: vec![
                SensitivityResultRow {
                    parameter: "length".to_owned(),
                    raw: -1.0,
                    normalized: -0.25,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                },
            ],
        })
    );

    let scalar = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::MeasurementsOnly {
            measurements: std::collections::HashMap::from([
                ("zeta".to_owned(), 0.7),
                ("gain".to_owned(), 10.0),
            ]),
        },
        AnalysisType::DcMismatch,
        "DC mismatch",
    );
    assert_eq!(
        scalar.result_payload,
        Some(AnalysisResultPayload::ScalarMeasurements {
            values: std::collections::BTreeMap::from([
                ("gain".to_owned(), 10.0),
                ("zeta".to_owned(), 0.7),
            ]),
        })
    );
}

#[test]
fn incomplete_reliability_and_soa_results_fail_closed_without_retained_payloads() {
    let controller = SimulationController::new();
    let reliability = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Reliability {
            years: vec![1.0, 10.0],
            waveforms: std::collections::HashMap::new(),
            device_results: Vec::new(),
        },
        AnalysisType::Reliability,
        "Reliability",
    );
    assert!(!reliability.success);
    assert!(reliability.result_payload.is_none());
    assert!(
        reliability
            .error_message
            .as_deref()
            .is_some_and(|message| message.contains("no device evidence"))
    );

    let soa = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Soa {
            time: vec![0.0, 1.0],
            waveforms: std::collections::HashMap::new(),
            violations: Vec::new(),
            evaluations: Vec::new(),
        },
        AnalysisType::Soa,
        "SOA",
    );
    assert!(!soa.success);
    assert!(soa.result_payload.is_none());
    assert!(
        soa.error_message
            .as_deref()
            .is_some_and(|message| message.contains("no evaluated-rule evidence"))
    );
}

#[test]
fn invalid_sensitivity_result_contract_fails_closed() {
    let controller = SimulationController::new();
    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Sensitivity {
            output: "V(out)".to_owned(),
            ac_mode: false,
            frequency_hz: None,
            sensitivities: std::collections::HashMap::from([("width".to_owned(), 2.0)]),
            normalized: std::collections::HashMap::new(),
        },
        AnalysisType::Sensitivity,
        "SENS",
    );

    assert!(!analysis.success);
    assert!(analysis.result_payload.is_none());
    assert!(
        analysis
            .error_message
            .as_deref()
            .is_some_and(|message| message.contains("misaligned"))
    );
}

#[test]
fn manual_deck_trigger_runs_deck_analysis_without_enabled_run_set() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("the sole run-set analysis disables");
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);
    let total_analyses = controller.total_analyses;
    let current_spec = controller.current_spec.clone();
    let source_domain = controller
        .current_provenance
        .as_ref()
        .map(AnalysisResultProvenance::source_domain);
    let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
    let run_count = state.simulation.runs.len();
    let status = state.simulation.status.clone();
    controller.abort();

    assert_eq!(total_analyses, 1);
    assert!(matches!(current_spec, Some(AnalysisSpec::DcOp { .. })));
    assert_eq!(source_domain, Some(AnalysisResultSourceDomain::ManualDeck));
    assert!(cached_netlist.contains(".op\n.end"));
    assert_eq!(run_count, 1);
    assert_eq!(status, "DC Operating Point");
}

#[test]
fn controller_manual_run_receipt_remains_authoritative_if_result_provenance_is_stripped() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("run-set analysis disables");
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let project_revision = state.workspace.project.revision();
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);

    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("manual task owns exact provenance");
    let expanded = controller
        .cached_netlist
        .as_deref()
        .expect("sealed manual deck");
    let expected_source_digest = crate::simulation::execution::content_digest(
        "rspice.manual-executable-source/v1",
        expanded.as_bytes(),
    );
    {
        let run = state.simulation.active_run().expect("manual run starts");
        let receipt = run.prepared_receipt().expect("manual run is sealed");
        assert_eq!(
            receipt.source_domain(),
            AnalysisResultSourceDomain::ManualDeck
        );
        assert_eq!(receipt.simulation_plan_id(), None);
        assert_eq!(receipt.project_revision(), project_revision);
        assert_eq!(
            receipt.prepared_snapshot_digest(),
            task_provenance.prepared_snapshot_digest()
        );
        assert_eq!(receipt.source_content_digest(), expected_source_digest);
        assert!(receipt.source_check_receipt().is_manual_source_check());
        assert_eq!(receipt.tasks().len(), 1);
        let task = &receipt.tasks()[0];
        assert_eq!(task.instance_id(), task_provenance.source_instance_id());
        assert_eq!(task.source_revision(), project_revision);
        assert_eq!(task.analysis_kind_tag(), 0);
        assert!(task.dependencies().is_empty());
    }

    let run = state
        .simulation
        .active_run_mut()
        .expect("manual run remains active");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
    );
    assert!(run.validate_provenance().is_ok());
    run.analyses[0].provenance = None;
    assert!(matches!(
        run.provenance(),
        Some(SimulationRunProvenance::Prepared(_))
    ));
    assert!(run.validate_provenance().is_err());

    controller.abort();
}

#[test]
fn controller_manual_run_receipt_survives_production_project_round_trip() {
    let mut state = AppState::default();
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    state.simulation.request_manual_deck_run();
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
        loaded.execution_context.is_some(),
        "manual receipt must remain independent of the unrelated retained plan"
    );
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("controller manual history restores");
    let run = &restored.runs[0];
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
    assert!(receipt.source_check_receipt().is_manual_source_check());
    assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
    assert_eq!(result_provenance.source_instance_id(), expected_source_id);
    assert_eq!(
        result_provenance.prepared_snapshot_digest(),
        receipt.prepared_snapshot_digest()
    );
}

#[test]
fn manual_deck_run_preserves_editor_source_without_ui_option_injection() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("the sole run-set analysis disables");
    state.sim_setup.options.reltol = 1.0e-4;
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);
    let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
    controller.abort();

    assert_eq!(cached_netlist, "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
    assert!(!cached_netlist.contains(".OPTIONS"));
    assert!(!cached_netlist.contains("RELTOL"));
}

#[test]
fn manual_deck_runs_use_imported_netlist_origin_for_relative_includes() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
    state.workspace.netlist_source =
        Some("deck\n.include models.lib\nV1 out 0 1\n.op\n.end\n".to_string());
    state.workspace.netlist_source_path = Some(PathBuf::from("decks").join("bias.cir"));

    assert_eq!(
        SimulationController::analysis_source_path(&state).as_deref(),
        Some(std::path::Path::new("decks").join("bias.cir").as_path())
    );
}

#[test]
fn manual_deck_runs_do_not_fall_back_to_schematic_path() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\n.op\n.end\n".to_string());

    assert!(
        SimulationController::analysis_source_path(&state).is_none(),
        "manual netlist text without an origin must not resolve includes from the schematic file"
    );
}

#[test]
fn simulate_run_set_does_not_run_manual_deck_source() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("default transient disables");
    let (op_id, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts as the sole enabled analysis");
    assert_eq!(plan.instances()[0].id(), op_id);
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_simulate_run_set();
    let mut controller = SimulationController::new();

    controller.start_simulation(&mut state);
    let status = state.simulation.status.clone();
    let run_count = state.simulation.runs.len();
    let total_analyses = controller.total_analyses;
    controller.abort();

    assert_eq!(status, "Run blocked");
    assert_eq!(run_count, 0);
    assert_eq!(total_analyses, 0);
}

#[test]
fn successful_manual_deck_run_promotes_pending_baseline() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.simulation.netlist_content =
        "deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string();
    let run_id = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer =
        Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string());
    state.ui.netlist.edited_lines.insert(0);

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n")
    );
    assert!((state.ui.netlist.last_run_params["r"] - 1e3).abs() < 1e-9);
    assert!((state.ui.netlist.last_run_params["cl"] - 2e-12).abs() < 1e-21);
    assert!(!state.ui.netlist.last_run_params.contains_key("expr"));
    assert!(state.ui.netlist.pending_manual_run_id.is_none());
    assert!(state.ui.netlist.pending_run_buffer.is_none());
    assert!(state.ui.netlist.edited_lines.is_empty());
}

#[test]
fn failed_manual_deck_run_keeps_previous_baseline() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.ui.netlist.last_run_buffer = Some("old\n.op\n.end\n".to_string());
    let run = state.simulation.start_run();
    let run_id = run.id;
    run.success = false;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer = Some("new\n.op\n.end\n".to_string());

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("old\n.op\n.end\n")
    );
    assert!(state.ui.netlist.pending_manual_run_id.is_none());
    assert!(state.ui.netlist.pending_run_buffer.is_none());
}

#[test]
fn batch_without_exact_run_ownership_does_not_modify_selected_history() {
    let mut state = AppState::default();
    let historical_id = state.simulation.start_run().id;
    let historical_lifecycle = state
        .simulation
        .run_by_sequence(historical_id)
        .expect("historical run")
        .lifecycle;
    let mut controller = SimulationController::new();
    controller.total_analyses = 1;

    controller.finish_simulation_batch(&mut state);

    let historical = state
        .simulation
        .run_by_sequence(historical_id)
        .expect("historical run remains");
    assert!(historical.success);
    assert_eq!(historical.lifecycle, historical_lifecycle);
    assert_eq!(state.simulation.status, "Completed with errors");
}

#[test]
fn disappeared_batch_target_does_not_reselect_the_active_historical_analysis() {
    let mut state = AppState::default();
    let historical_id = state.simulation.start_run().id;
    let historical = state
        .simulation
        .run_by_sequence_mut(historical_id)
        .expect("historical run");
    historical.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "first"));
    historical.add_analysis(AnalysisResult::new(2, AnalysisType::Ac, "second"));
    assert!(state.simulation.select_latest_analysis());
    assert_eq!(
        state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.label.as_str()),
        Some("second")
    );

    let mut controller = SimulationController::new();
    controller.current_run_id = Some(historical_id + 10_000);
    controller.total_analyses = 1;

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.label.as_str()),
        Some("second"),
        "a missing completion target must not call complete_run on the selected history row"
    );
}

#[test]
fn live_transient_accumulator_rejects_partial_or_schema_changing_points() {
    let sample = |time, waveforms: &[(&str, f64)]| TransientSampleDelta {
        time,
        waveforms: waveforms
            .iter()
            .map(
                |(name, value)| crate::simulation::runner::TransientWaveformSample {
                    name: (*name).to_owned(),
                    value: *value,
                    y_unit: "V".to_owned(),
                },
            )
            .collect(),
    };
    let mut accumulator = LiveTransientAccumulator::default();

    accumulator.ingest(vec![
        sample(0.0, &[("out", 0.0), ("ref", 1.0)]),
        sample(1.0, &[("out", 1.0)]),
        sample(2.0, &[("out", 2.0), ("other", 2.0)]),
        sample(3.0, &[("out", 3.0), ("ref", 4.0)]),
    ]);

    assert_eq!(accumulator.waveforms.len(), 2);
    for waveform in &accumulator.waveforms {
        assert_eq!(waveform.x, vec![0.0, 3.0]);
        assert_eq!(waveform.x.len(), waveform.y.len());
    }
}

#[test]
fn live_transient_accumulator_compacts_aligned_source_traces() {
    let mut accumulator = LiveTransientAccumulator::default();
    let deltas = (0..LiveTransientAccumulator::MAX_SOURCE_SAMPLES + 1)
        .map(|index| TransientSampleDelta {
            time: index as f64,
            waveforms: vec![
                crate::simulation::runner::TransientWaveformSample {
                    name: "out".to_owned(),
                    value: (index as f64 / 10.0).sin(),
                    y_unit: "V".to_owned(),
                },
                crate::simulation::runner::TransientWaveformSample {
                    name: "ref".to_owned(),
                    value: (index as f64 / 17.0).cos(),
                    y_unit: "V".to_owned(),
                },
            ],
        })
        .collect();

    accumulator.ingest(deltas);

    assert_eq!(accumulator.waveforms.len(), 2);
    assert!(accumulator.waveforms[0].x.len() <= LiveTransientAccumulator::COMPACTED_SOURCE_SAMPLES);
    assert_eq!(
        accumulator.waveforms[0].x, accumulator.waveforms[1].x,
        "derived expressions require one aligned provisional axis"
    );
    assert_eq!(accumulator.waveforms[0].x.first(), Some(&0.0));
    assert_eq!(
        accumulator.waveforms[0].x.last(),
        Some(&(LiveTransientAccumulator::MAX_SOURCE_SAMPLES as f64))
    );
}

#[test]
fn successful_manual_deck_run_preserves_post_launch_diff_pips() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.simulation.netlist_content = "deck\n.op\nR1 out 0 2k\n.end\n".to_string();
    let run_id = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer = Some("deck\n.op\nR1 out 0 1k\n.end\n".to_string());

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("deck\n.op\nR1 out 0 1k\n.end\n")
    );
    assert!(state.ui.netlist.edited_lines.contains(&2));
    assert_eq!(state.ui.netlist.edited_lines.len(), 1);
}

#[test]
fn ui_progress_fraction_uses_runner_fraction_or_running_floor() {
    assert!((SimulationController::ui_progress_fraction(Some(0.42), true) - 0.42).abs() < 1e-6);
    assert_eq!(SimulationController::ui_progress_fraction(None, true), 0.08);
    assert_eq!(SimulationController::ui_progress_fraction(None, false), 0.0);
    assert_eq!(
        SimulationController::ui_progress_fraction(Some(1.2), true),
        1.0
    );
}

/// A corner declaration keeps a task so the run's receipt has an entry for it,
/// but its turn must not reach the runner: the points are the solve. If it were
/// dispatched, the declared space would be solved a second time.
#[test]
fn a_corner_declarations_turn_assembles_its_family_without_reaching_the_runner() {
    use crate::product::ProcessCorner;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::services::simulation_runner::{CornerBaseMode, CornerProcess, CornerRunConfig};
    use crate::simulation::execution::{
        ExecutionPermitIssuer, ExecutionTargetCapabilities, PreparedRunSnapshot, PreparedTask,
        RunSourceReceipt, SavePolicy, SnapshotParts,
    };

    let corner = QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: SpecExecutionOptions {
            corner: Some(CornerRunConfig {
                process_corners: vec![CornerProcess::TT],
                voltages: vec![1.8, 1.62],
                supply_source_names: vec!["VDD".to_owned()],
                temperatures_c: vec![27.0],
                full_matrix: true,
                nominal_voltage: Some(1.8),
                base_mode: CornerBaseMode::Op,
                model_bindings: Vec::new(),
                points: Vec::new(),
            }),
            ..SpecExecutionOptions::default()
        },
        analysis_line: ".corner".to_owned(),
    };
    let snapshot = PreparedRunSnapshot::new(SnapshotParts {
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: 3,
        topology_revision: 4,
        source_digest: ContentDigest::from_bytes([0x81; 32]),
        reference_process: ProcessCorner::TT,
        reference_temperature_celsius: 27.0,
        run_set: None,
        tasks: vec![PreparedTask::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            "Corner",
            corner,
        )],
        executable_netlist: "corner\nVDD vdd 0 DC 1.8\nR1 vdd out 1k\nR2 out 0 1k\n.op\n.end\n"
            .to_owned(),
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: crate::state::PreparedSpecificationPolicy::default(),
        project_veriloga_runtimes: Default::default(),
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([0x82; 32])),
        advisories: Vec::new(),
        manual_source: None,
        cross_probe: None,
        touchstone_export: TouchstoneExportPolicy::disabled(),
        sealed_source_dependencies: Vec::new(),
    })
    .expect("corner snapshot validates");
    let digest = snapshot.digest();
    let proof = ExecutionPermitIssuer::default()
        .issue(digest)
        .expect("permit issues")
        .consume(digest, digest)
        .expect("permit consumes");
    let dispatch = snapshot
        .authorize_dispatch(proof)
        .expect("snapshot authorizes");

    let mut controller = SimulationController::new();
    for task in dispatch.tasks() {
        controller.point_families.register(task);
    }
    let mut tasks = dispatch.into_tasks();
    assert_eq!(tasks.len(), 3, "two declared points and the assembly");

    // Both points ran and neither converged. That is still evidence about those
    // corners, and the declaration must answer for it in its own position
    // rather than being dispatched to find out.
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    for _ in 0..2 {
        let point = tasks.pop_front().expect("a declared point");
        let provenance = AnalysisResultProvenance::new(
            point.instance_id(),
            point.source_revision(),
            point.snapshot_digest(),
            point.dependencies().to_vec(),
        )
        .expect("point provenance")
        .with_pvt_point(point.pvt_point().cloned());
        state
            .simulation
            .run_by_sequence_mut(run_sequence)
            .expect("active run")
            .add_analysis(
                AnalysisResult::failed(1, AnalysisType::DcOp, point.label(), "solver failed")
                    .with_provenance(provenance),
            );
    }
    let declaration = tasks.front().expect("the assembly is last").instance_id();

    controller.current_run_id = Some(run_sequence);
    controller.current_analysis_idx = 2;
    controller.total_analyses = 3;
    controller.pending_analyses = tasks;
    controller.start_next_analysis(&mut state);

    assert!(
        !controller.is_running(),
        "the assembly must never be handed to the runner"
    );
    assert_eq!(
        controller.total_analyses, 0,
        "the batch finished in the same call, so nothing is awaiting an engine"
    );
    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("completed run remains");
    assert_eq!(run.analyses.len(), 3);
    let family = run
        .find_analysis_by_source_instance(declaration)
        .expect("the declaration retained a result in its own position");
    assert_eq!(family.analysis_type, AnalysisType::Corner);
    assert!(!family.success, "no corner converged, so the family failed");
    assert_eq!(run.analyses[2].id, family.id, "and it is retained last");
}

#[test]
fn a_wholesale_catalogue_replacement_cannot_reuse_an_inspection_key_it_did_not_earn() {
    // The inspection digest is a repaint cache key, so the tempting cheap
    // version of it is a mutation counter on the model catalogue. That is
    // unsound here: `ModelLibraryManager` is replaced whole when a project is
    // opened, when a recovery comparison is accepted, when design history
    // restores a candidate, and when a model-hub operation publishes a rebuilt
    // one — and a replacement arrives carrying whatever counter it was
    // serialized with, which may be one this session has already cached an
    // answer against. This builds that collision deliberately: every other
    // input to the digest is held identical, including the two counters it does
    // fold in, and the catalogue is replaced by a *deserialized* one rather
    // than mutated in place, which is how a project open does it.
    use crate::state::model_library::ModelLibraryManager;

    let mut state = AppState::default();
    let epoch = state.design_execution_epoch;
    let project_revision = state.workspace.project.revision();

    let mut first = ModelLibraryManager::new();
    first
        .load_library_bytes(
            "opened-project.lib",
            b".model nch NMOS (LEVEL=1 VTO=0.4)\n".to_vec(),
            None,
        )
        .expect("the first project's catalogue imports");
    let mut second = ModelLibraryManager::new();
    second
        .load_library_bytes(
            "opened-project.lib",
            b".model nch NMOS (LEVEL=1 VTO=0.9)\n.model pch PMOS (LEVEL=1)\n".to_vec(),
            None,
        )
        .expect("the second project's catalogue imports");

    state.model_library_manager = first;
    let before = prepared_run::design_inspection_input_digest(&state);

    state.model_library_manager = serde_json::from_str(
        &serde_json::to_string(&second).expect("a catalogue serializes into a project file"),
    )
    .expect("a catalogue restores out of a project file");

    assert_eq!(
        state.design_execution_epoch, epoch,
        "the collision is only real while every counter the digest folds in is unmoved"
    );
    assert_eq!(state.workspace.project.revision(), project_revision);
    assert_ne!(
        prepared_run::design_inspection_input_digest(&state),
        before,
        "a catalogue replaced wholesale presented a key this session had already \
         cached an answer against; the Bins & geometry page would go on stating the \
         previous project's model cards"
    );
}
