//! Tests for simulation orchestration.
//!
//! The cases cover the controller's transitions between requested, running,
//! and settled state, and assert that a run's results are only published once
//! the run that produced them has actually completed.

use super::*;
use crate::simulation::plan::AnalysisKind;
use crate::state::{ComponentType, Point, PreparedSourceCheckReceipt, SimulationRunProvenance};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

// The application the controller's tests build. The controller itself never
// names it; its test modules import it from here.
pub(in crate::simulation::controller) use crate::workbench::app_state::AppState;

use super::test_execution::state_with_current_clean_drc;
use super::test_host::RunTestHost;
use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use std::cell::RefCell;
use std::path::Path;

#[test]
fn save_all_retains_engine_results_while_explicit_empty_retains_none() {
    let source = || {
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 0.5, 1.0], "#fff"),
            RetainedTrace::new("I(R1)", vec![0.0, 0.5, 1.0], vec![1.0, 0.5, 0.0], "#fff"),
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
        rspice_simulation::output_contract::compile_saved_output_contracts(
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
pub(crate) struct MockExportWorkflowIo {
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

fn state_with_current_drc_error() -> RunTestHost {
    let mut state = RunTestHost::default();
    state
        .model
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
    state.model.design_checks.drc_results = Some(result);
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();
    state
}

pub(crate) fn bind_test_run_running(
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

fn bind_and_request_test_abort(state: &mut RunTestHost, controller: &mut SimulationController) {
    let run_sequence = state
        .model
        .simulation
        .active_run()
        .expect("test abort has an active run")
        .id;
    bind_test_run_running(state, controller, run_sequence);
    state
        .model
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
            rspice_simulation::results::SampledWaveform::new_complex(
                name,
                frequencies.clone(),
                real,
                imag,
            ),
        );
    }

    crate::simulation::SimulationResult::Ac {
        convergence: None,
        noise_reference_temperature_kelvin: None,
        reference_impedances_ohm: None,
        frequencies,
        waveforms,
        measurements: Vec::new(),
    }
}

fn synthetic_dc_op_result() -> crate::simulation::SimulationResult {
    let mut result = rspice_simulation::results::DcOpResult::default();
    result.node_voltages.insert("out".to_string(), 1.25);
    crate::simulation::SimulationResult::DcOp(Box::new(result))
}

pub(crate) fn synthetic_result_provenance() -> AnalysisResultProvenance {
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
    let mut state = RunTestHost::default();
    let run_sequence = state.model.simulation.start_run().id;
    let provenance = synthetic_result_provenance();
    let instance = provenance.source_instance_id();
    let failed = AnalysisResult::failed(1, AnalysisType::Soa, "SOA", "invalid retained evidence");
    let mut controller = SimulationController::new();

    assert!(
        !controller
            .retain_completed_analysis(&mut state.model, Some(run_sequence), failed, provenance)
            .expect("failed result is retained as failure evidence")
    );
    assert!(!controller.successful_analysis_instances.contains(&instance));
    let run = state
        .model
        .simulation
        .retained
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
    let mut state = RunTestHost::default();
    let run_sequence = state.model.simulation.start_run().id;
    let controller = SimulationController::new();

    let before = state.model.simulation.view.data_version;
    let failed = AnalysisResult::failed(1, AnalysisType::DcOp, "OP", "solver gave up")
        .with_provenance(synthetic_result_provenance());
    assert_eq!(
        controller.seal_failed_run(&mut state.model, Some(run_sequence), Some(failed), None),
        Vec::<String>::new(),
        "the default save policy admits a failure record"
    );

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("target run remains");
    assert!(!run.success, "the run's verdict must flip to failed");
    assert_eq!(run.analyses.len(), 1);
    assert_ne!(
        state.model.simulation.view.data_version, before,
        "the retained run changed under memos that key on the data version"
    );

    // A terminal lifecycle is part of the same event, and the generation is
    // declared after it rather than between the two mutations. Nothing to
    // retain is not nothing to declare: the verdict alone moved the run.
    let before = state.model.simulation.view.data_version;
    let terminal = Some(SimulationRunLifecycle::Aborted);
    assert!(
        controller
            .seal_failed_run(&mut state.model, Some(run_sequence), None, terminal)
            .is_empty()
    );
    assert_ne!(state.model.simulation.view.data_version, before);
    assert_eq!(
        state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("target run remains")
            .lifecycle,
        SimulationRunLifecycle::Aborted
    );

    // A target run that no longer exists has nothing to seal and nothing to
    // declare.
    let before = state.model.simulation.view.data_version;
    for absent in [Some(run_sequence + 4_096), None] {
        assert!(
            controller
                .seal_failed_run(&mut state.model, absent, None, None)
                .is_empty()
        );
        assert_eq!(state.model.simulation.view.data_version, before);
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
    let production = [
        crate::source_guard::production_half(include_str!("../controller.rs")),
        include_str!("completion.rs"),
    ]
    .join("\n");
    let offenders = production
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
    let mut state = RunTestHost::default();
    let run_sequence = state.model.simulation.start_run().id;
    let provenance = synthetic_result_provenance();
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new(
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
        .retain_completed_analysis(&mut state.model, Some(run_sequence), analysis, provenance)
        .expect_err("oversized evidence must not enter the retained run");

    assert!(error.contains("storage limit"));
    assert!(
        state
            .model
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .expect("target run remains")
            .analyses
            .is_empty()
    );
}

#[test]
fn provisional_live_result_obeys_the_authenticated_storage_ceiling() {
    let mut state = RunTestHost::default();
    let run_sequence = state.model.simulation.start_run().id;
    let partial = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![RetainedTrace::new(
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
                .model
                .simulation
                .retained
                .run_by_sequence(run_sequence)
                .expect("target run"),
            &partial,
        )
        .expect_err("live evidence must obey the same immutable storage ceiling");

    assert!(error.contains("storage limit"));
    assert!(
        state
            .model
            .simulation
            .retained
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

    state.model.simulation.execution.request_simulate_run_set();
    controller.update(&mut state, &export_io);

    assert!(!state.model.simulation.execution.trigger_simulation);
    assert!(!state.model.simulation.execution.is_running);
    assert_eq!(state.model.simulation.execution.status, "Run blocked");
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
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(&mut state);
    let run_id = state
        .model
        .simulation
        .active_run()
        .expect("a run starts")
        .id;
    controller.abort();

    let deck = state
        .model
        .simulation
        .retained
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
        state.runs.manual_run.last_run_buffer.is_none(),
        "and nobody typed it, so there is no manual baseline to be current with"
    );
}

#[test]
fn controller_plan_run_is_sealed_with_exact_prepared_receipt_before_results() {
    let mut state = state_with_current_clean_drc();
    let plan = state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("stable plan");
    let plan_id = plan.id();
    let plan_revision = plan.revision();
    let task_id = plan
        .instances()
        .iter()
        .find(|instance| instance.enabled())
        .expect("enabled plan task")
        .id();
    let project_revision = state.model.workspace.content.project.revision();
    let mut controller = SimulationController::new();
    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("clean plan preflight");

    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(&mut state);

    let run = state
        .model
        .simulation
        .active_run()
        .expect("prepared run starts");
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
fn design_epoch_reset_terminalizes_executor_owned_history() {
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.model.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);

    state.model.design_execution_epoch = state.model.design_execution_epoch.wrapping_add(1);
    controller.update(&mut state, &export_io);

    let run = state
        .model
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .expect("interrupted history remains retained");
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Interrupted);
    assert!(!run.success);
    assert!(state.model.simulation.execution.active_execution.is_none());
    assert!(state.model.simulation.execution.abort_request.is_none());
}

#[test]
fn abort_trigger_discards_worker_aborted_result_without_failed_analysis() {
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    state.model.simulation.start_run();
    state.model.simulation.execution.status = "Running".to_string();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller.runner = super::test_execution::start_manual_deck(
        "Cancelled task\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
    );
    controller.runner.abort();
    super::test_execution::wait_until_finished_unpolled(&controller.runner);
    bind_and_request_test_abort(&mut state, &mut controller);

    controller.update(&mut state, &export_io);

    assert_eq!(state.model.simulation.execution.status, "Aborted");
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active run remains");
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
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    state.model.simulation.start_run();
    state.model.simulation.execution.status = "Running".to_string();
    controller.current_spec = Some(AnalysisSpec::dc_op());
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller.runner = super::test_execution::start_manual_deck(
        "Completed task\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
    );
    super::test_execution::wait_until_finished_unpolled(&controller.runner);
    bind_and_request_test_abort(&mut state, &mut controller);

    controller.update(&mut state, &export_io);

    assert_eq!(state.model.simulation.execution.status, "Aborted");
    let run = state
        .model
        .simulation
        .active_run()
        .expect("active run remains");
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
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let older_run_id = state.model.simulation.start_run().id;
    let started_run_id = state.model.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, started_run_id);
    assert!(
        state.model.simulation.select_run(1),
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
    controller.accept_completion(&mut state, &export_io, Ok(synthetic_dc_op_result()));

    let older_run = state
        .model
        .simulation
        .retained
        .run_by_sequence(older_run_id)
        .expect("older run remains");
    let started_run = state
        .model
        .simulation
        .retained
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
        state.model.simulation.active_run().map(|run| run.id),
        Some(started_run_id)
    );
    assert_eq!(started_run.lifecycle, SimulationRunLifecycle::Completed);
}

#[test]
fn completed_transient_result_reuses_owned_waveform_buffers_in_run_history() {
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.model.simulation.start_run().id;
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
        rspice_simulation::results::SampledWaveform {
            name: "V(out)".to_string(),
            x_values: Vec::new(),
            y_values: values,
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        },
    );

    controller.accept_completion(
        &mut state,
        &export_io,
        Ok(crate::simulation::SimulationResult::Transient {
            spectra: Vec::new(),
            hdl_output: Vec::new(),
            checks: Default::default(),
            time,
            waveforms,
            measurements: Vec::new(),
            periodic_state: None,
            convergence: Default::default(),
            events: Default::default(),
        }),
    );

    let analysis = state
        .model
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
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.model.simulation.start_run().id;
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
        modes: Default::default(),
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
        rspice_simulation::results::SampledWaveform {
            name: "V(out)".to_string(),
            x_values: Vec::new(),
            y_values: values,
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        },
    );

    controller.accept_completion(
        &mut state,
        &export_io,
        Ok(crate::simulation::SimulationResult::DcSweep {
            evidence: None,
            sweep_var: "V1".to_string(),
            sweep_values,
            waveforms,
            measurements: Vec::new(),
            checks: Default::default(),
        }),
    );

    let analysis = state
        .model
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
    let mut state = RunTestHost::default();
    let mut controller = SimulationController::new();
    let export_io = MockExportWorkflowIo::default();
    let run_sequence = state.model.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_sequence);
    controller.current_spec = Some(AnalysisSpec::dc_op());
    let provenance = synthetic_result_provenance();
    let expected_source_id = provenance.source_instance_id();
    let expected_snapshot = provenance.prepared_snapshot_digest();
    controller.current_provenance = Some(provenance);
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller.accept_completion(
        &mut state,
        &export_io,
        Err(SimulationError::SolverError("singular matrix".to_owned())),
    );

    let analysis = &state
        .model
        .simulation
        .active_run()
        .expect("run remains")
        .analyses[0];
    let restored = analysis
        .provenance
        .as_ref()
        .expect("failed result has prepared provenance");
    assert!(!analysis.success);
    assert_eq!(restored.source_instance_id(), expected_source_id);
    assert_eq!(restored.prepared_snapshot_digest(), expected_snapshot);
    assert_eq!(
        state
            .model
            .simulation
            .active_run()
            .expect("failed run")
            .lifecycle,
        SimulationRunLifecycle::Failed
    );
}

#[test]
fn failed_prerequisite_skips_dependent_prepared_task_with_exact_provenance() {
    use crate::simulation::plan::AnalysisDraft;
    use rspice_simulation_contract::fourier_draft::{FourierConfig, FourierDialogState};

    let mut state = super::prepared_run::tests::runnable_state();
    state.model.sim_setup.run_set =
        rspice_simulation_contract::run_set::RunSetState::reference_only();
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    let (prerequisite_id, _) = plan.insert(AnalysisKind::Transient).unwrap();
    let (dependent_id, _) = plan.insert(AnalysisKind::Fourier).unwrap();
    plan.edit(prerequisite_id, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            unreachable!()
        };
        draft.stop = "1".into();
        draft.step = ".005".into();
        draft.start = "0".into();
    })
    .unwrap();
    plan.edit(dependent_id, |draft| {
        *draft = AnalysisDraft::Fourier(FourierDialogState::from_config(&FourierConfig {
            fundamental_freq: 2.0,
            num_harmonics: 4,
            num_periods: 1,
            output_node: "out".into(),
            output_ref: "0".into(),
            additional_outputs: Vec::new(),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        }));
    })
    .unwrap();
    plan.bind_dependency(dependent_id, AnalysisKind::Transient, prerequisite_id)
        .unwrap();
    state.sync_active_schematic_to_workspace();
    assert!(!state.run_active_design_checks().unwrap().has_errors());
    let mut preparer = SimulationController::new();
    preparer
        .prepare_run_set_for_preflight(&state)
        .expect("prepare the authored dependency graph");
    let dispatch = preparer
        .consume_snapshot_for_dispatch(&state)
        .expect("authorize the unchanged generated plan");
    let receipt = dispatch
        .prepared_run_receipt(AnalysisResultSourceDomain::SimulationPlan)
        .unwrap();
    let mut tasks = dispatch.into_tasks();
    assert_eq!(tasks.len(), 2);
    let failed_task = tasks.pop_front().expect("prerequisite task");

    let run = state.model.simulation.start_run();
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .unwrap();
    let run_sequence = run.id;
    let failed_provenance = AnalysisResultProvenance::new(
        failed_task.instance_id(),
        failed_task.source_revision(),
        failed_task.snapshot_digest(),
        failed_task.dependencies().to_vec(),
    )
    .expect("failed prerequisite provenance");
    state
        .model
        .simulation
        .retained
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
        .model
        .simulation
        .retained
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
            .is_some_and(|message| message.contains("which did not complete"))
    );
    assert_eq!(
        skipped
            .provenance
            .as_ref()
            .expect("skipped provenance")
            .dependency_ids(),
        &[prerequisite_id]
    );
    assert_eq!(
        state.model.simulation.execution.status,
        "Completed with errors"
    );
    assert!(!controller.is_running());
}

mod completion_paths;
mod monte_carlo_checkpoint;
mod transient_checkpoint;

mod autonomous_periodic;
mod periodic_port_noise;
mod run_checks;
mod run_log;

mod pvt_family;
