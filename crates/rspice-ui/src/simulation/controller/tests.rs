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
    let mut controller = SimulationController::new();
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
fn malformed_eager_fft_input_clears_the_prior_spectrum_and_keeps_the_typed_failure() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    controller.current_run_id = Some(run_sequence);
    controller.current_provenance = Some(synthetic_result_provenance());
    let expected_owner = controller
        .in_flight_specialized_viewer_provenance(&state)
        .expect("the active prepared analysis owns specialized-viewer caches");
    let valid_time = (0..17).map(|index| index as f64).collect::<Vec<_>>();
    let valid_values = (0..17)
        .map(|index| (index as f64 * 0.25).sin())
        .collect::<Vec<_>>();
    let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::WaveformData::new("V(out)", valid_time.clone(), valid_values, "#ffffff"),
    ]);
    controller.populate_transient_post_views(&mut state, &valid);
    assert!(state.analysis.fft_state.has_data());
    assert_eq!(state.analysis.cache_authority.fft, Some(expected_owner));
    assert!(controller.transient_post.fft_loaded.is_some_and(|loaded| {
        loaded.analysis == expected_owner
            && matches!(
                loaded.availability,
                super::transient_post::DerivedViewAvailability::Ready
            )
    }));

    let malformed = AnalysisResult::new(2, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::WaveformData::new("V(out)", valid_time, vec![0.0; 16], "#ffffff"),
    ]);
    controller.populate_transient_post_views(&mut state, &malformed);

    assert!(!state.analysis.fft_state.has_data());
    assert!(state.analysis.fft_state.source_cache.is_none());
    assert!(state.analysis.cache_authority.fft.is_none());
    assert!(controller.transient_post.fft_loaded.is_some_and(|loaded| {
        loaded.analysis == expected_owner
            && matches!(
                loaded.availability,
                super::transient_post::DerivedViewAvailability::Unavailable
            )
    }));
    assert!(matches!(
        state.analysis.fft_state.last_error,
        Some(crate::analysis::fft::FftFailure::Input(
            crate::analysis::fft::FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16
            }
        ))
    ));
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

/// Sealing a failure into a retained run is a new generation of that run.
///
/// Every Results memo over a run is keyed on `data_version` — the dataset
/// content digest the inspector's tamper check compares against, the
/// operating-point row plan, the retained-evidence verdict. A failure sealed
/// at a constant version leaves all of them describing the run as it stood
/// before it failed, and the tamper check in particular then reads a digest
/// memo that was never re-taken.
#[test]
fn sealing_a_failure_declares_a_new_dataset_generation() {
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    let controller = SimulationController::new();

    let before = state.simulation.data_version;
    let failed = AnalysisResult::failed(1, AnalysisType::DcOp, "OP", "solver gave up")
        .with_provenance(synthetic_result_provenance());
    assert_eq!(
        controller.seal_failed_run(&mut state, Some(run_sequence), Some(failed), None),
        Vec::<String>::new(),
        "the default save policy admits a failure record"
    );

    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("target run remains");
    assert!(!run.success, "the run's verdict must flip to failed");
    assert_eq!(run.analyses.len(), 1);
    assert_ne!(
        state.simulation.data_version, before,
        "the retained run changed under memos that key on the data version"
    );

    // A terminal lifecycle is part of the same event, and the generation is
    // declared after it rather than between the two mutations. Nothing to
    // retain is not nothing to declare: the verdict alone moved the run.
    let before = state.simulation.data_version;
    let terminal = Some(SimulationRunLifecycle::Aborted);
    assert!(
        controller
            .seal_failed_run(&mut state, Some(run_sequence), None, terminal)
            .is_empty()
    );
    assert_ne!(state.simulation.data_version, before);
    assert_eq!(
        state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("target run remains")
            .lifecycle,
        SimulationRunLifecycle::Aborted
    );

    // A target run that no longer exists has nothing to seal and nothing to
    // declare.
    let before = state.simulation.data_version;
    for absent in [Some(run_sequence + 4_096), None] {
        assert!(
            controller
                .seal_failed_run(&mut state, absent, None, None)
                .is_empty()
        );
        assert_eq!(state.simulation.data_version, before);
    }
}

/// The verdict flip, the lifecycle seal and the generation bump are one
/// event, and one function performs it.
///
/// Ten shipped paths used to flip `run.success` on their own, at a constant
/// data version, under memos keyed on that version. A guard on the source is
/// what keeps the eleventh from being written: nothing about
/// `run.success = false` at a call site fails a test by itself.
#[test]
fn no_shipped_path_fails_a_run_outside_the_sealing_helper() {
    let offenders = crate::source_guard::production_half(include_str!("../controller.rs"))
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("run.success = false"))
        .map(|(index, line)| format!("{}: {}", index + 1, line.trim()))
        .collect::<Vec<_>>();
    assert_eq!(
        offenders.len(),
        1,
        "every shipped path that fails a run must go through `seal_failed_run`, which is the \
         one place that declares the new dataset generation the flip creates:\n{}",
        offenders.join("\n")
    );
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

/// A schematic run seals the deck its engine read.
///
/// Not a manual deck: this is exactly the run that used to leave nothing
/// behind. The editor-buffer half of the netlist state stays empty, because
/// nobody typed this deck — what is retained is the source the engine was
/// handed, which is a different artifact and the only one that can answer
/// "what did this actually solve". What the viewer then does with it is
/// proved next door, in `netlist_document::tests`.
#[test]
fn a_schematic_run_retains_the_deck_it_executed() {
    let mut state = state_with_current_clean_drc();
    let mut controller = SimulationController::new();
    controller
        .prepare_run_set_for_preflight(&state)
        .expect("clean plan preflight");
    state.simulation.request_simulate_run_set();
    controller.start_simulation(&mut state);
    let run_id = state.simulation.active_run().expect("a run starts").id;
    controller.abort();

    let deck = state
        .simulation
        .executed_decks
        .get(run_id)
        .expect("a schematic run seals its executed deck too");
    let executed = deck.point(0).expect("the run has a first point");
    assert!(
        executed.deck.contains(".end"),
        "what is retained is a deck: {}",
        executed.deck
    );
    assert!(
        state.ui.netlist.last_run_buffer.is_none(),
        "and nobody typed it, so there is no manual baseline to be current with"
    );
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
        hysteresis: false,
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

mod completion_paths;
