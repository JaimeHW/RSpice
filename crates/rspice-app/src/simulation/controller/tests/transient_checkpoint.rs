//! Accepted core images cross retention, project recovery, and receiving limits.
use super::*;
use crate::product::{ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};
use crate::state::{CanonicalAnalysisKind, PreparedRunReceipt, PreparedRunTaskReceipt};
use rspice_core::engine::{Engine, TransientCheckpointEncoding, TransientStartupMode};
use rspice_core::{NoAbort, ResourceLimits};
use rspice_formats::project_results::{PersistedField, ProjectSimulationResults};
use rspice_results::transient_checkpoint::{
    TransientCheckpointEvidence, TransientCheckpointLibrary,
};
use rspice_simulation::transient_checkpoint::TransientCheckpointRequest;
use std::sync::Arc;

const DECK: &str =
    "Checkpoint retention\nV1 in 0 sin(0 1 1meg)\nR1 in out 1k\nC1 out 0 1p\n.tran 1n 20n\n.end\n";

fn images() -> Vec<Arc<[u8]>> {
    let (_, images) = Engine::default()
        .run_tran_checkpoint_schedule_with_startup_mode(
            &rspice_core::Netlist::parse(DECK).unwrap(),
            20e-9,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &[7e-9, 14e-9],
        )
        .unwrap();
    images
        .iter()
        .map(|image| {
            image
                .checkpoint
                .to_bytes(TransientCheckpointEncoding::Packed)
                .unwrap()
                .into()
        })
        .collect()
}

fn fixture() -> (SimulationController, AppState) {
    let provenance = synthetic_result_provenance();
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: provenance.prepared_snapshot_digest(),
        source_content_digest: ContentDigest::from_bytes([21; 32]),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [22; 32],
        )),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: Default::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                provenance.source_instance_id(),
                provenance.source_revision(),
                vec![],
                CanonicalAnalysisKind::Transient.tag(),
                ContentDigest::from_bytes([23; 32]),
            )
            .unwrap(),
        ],
    })
    .unwrap();
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .unwrap();
    run.mark_running().unwrap();
    let mut controller = SimulationController::new();
    controller.current_run_id = Some(run.id);
    controller.current_analysis_idx = 1;
    controller.total_analyses = 1;
    controller.current_analysis_label = Some("TRAN".into());
    controller.current_provenance = Some(provenance);
    controller.current_execution_limits = Some(ResourceLimits::default());
    controller.current_spec = Some(AnalysisSpec::Transient {
        stop_time: 20e-9,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    });
    controller.current_spec_options = Some(SpecExecutionOptions {
        tran_checkpoint: Some(TransientCheckpointRequest {
            times: vec![7e-9, 14e-9],
            resume: None,
        }),
        ..Default::default()
    });
    (controller, state)
}

#[test]
fn transient_checkpoint_survives_live_updates_project_recovery_and_terminal_results() {
    let bytes = images();
    for success in [false, true] {
        let (controller, mut state) = fixture();
        controller
            .retain_transient_checkpoint(&mut state, bytes[0].clone())
            .unwrap();
        let before = state.simulation.view.data_version;
        let partial = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_provenance(controller.current_provenance.clone().unwrap())
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![0.0, 1e-9],
                vec![0.0, 0.01],
                "#fff",
            )]);
        controller
            .retain_live_transient_analysis(&mut state, controller.current_run_id.unwrap(), partial)
            .unwrap();
        assert_ne!(before, state.simulation.view.data_version);
        let retained = state.simulation.active_analysis().unwrap();
        assert_eq!(
            retained.transient_checkpoint.as_ref().unwrap().bytes(),
            &*bytes[0]
        );
        let mut without = retained.clone();
        without.transient_checkpoint = None;
        assert_ne!(retained.result_data_digest(), without.result_data_digest());
        assert!(
            retained.retained_storage_bytes()
                >= without.retained_storage_bytes() + bytes[0].len() as u64
        );

        // A later checkpoint keeps the existing live output; a stale one cannot replace it.
        controller
            .retain_transient_checkpoint(&mut state, bytes[1].clone())
            .unwrap();
        assert_eq!(
            state.simulation.active_analysis().unwrap().waveforms.len(),
            1
        );
        assert!(
            controller
                .retain_transient_checkpoint(&mut state, bytes[0].clone())
                .is_err()
        );
        let persisted = crate::io::capture_simulation_results(&state.simulation);
        persisted.validate().unwrap();
        let loaded: ProjectSimulationResults =
            serde_json::from_str(&serde_json::to_string(&persisted).unwrap()).unwrap();
        let recovered = crate::io::simulation_state_from_results(loaded).unwrap();
        assert_eq!(
            recovered.active_run().unwrap().lifecycle,
            SimulationRunLifecycle::Interrupted
        );
        let recovered_image = recovered
            .active_analysis()
            .unwrap()
            .transient_checkpoint
            .as_ref()
            .unwrap();
        assert_eq!(recovered_image.bytes(), &*bytes[1]);
        let checkpoint = rspice_results::transient_checkpoint::decode_bytes(
            recovered_image.bytes(),
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        let original = rspice_core::engine::TransientCheckpoint::from_bytes(&bytes[1]).unwrap();
        let netlist = rspice_core::Netlist::parse(DECK).unwrap();
        let engine = Engine::default();
        let (expected, _) = engine
            .run_tran_resume(&netlist, &original, 20e-9, 1e-9)
            .unwrap();
        let (actual, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 20e-9, 1e-9)
            .unwrap();
        assert_eq!(actual.time, expected.time);
        assert_eq!(actual.voltages, expected.voltages);
        assert_eq!(actual.branch_currents, expected.branch_currents);

        let terminal = if success {
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        } else {
            AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "interrupted")
        }
        .with_provenance(controller.current_provenance.clone().unwrap());
        let run = state
            .simulation
            .retained
            .run_by_sequence_mut(controller.current_run_id.unwrap())
            .unwrap();
        controller
            .retain_analysis_under_current_policy(run, terminal)
            .unwrap();
        assert_eq!(run.analyses.len(), 1);
        assert!(!run.analyses[0].is_live_partial());
        assert_eq!(run.analyses[0].success, success);
        assert_eq!(
            run.analyses[0]
                .transient_checkpoint
                .as_ref()
                .unwrap()
                .bytes(),
            &*bytes[1]
        );
        run.analyses[0].validate_retained_evidence().unwrap();
        assert!(
            controller
                .retain_transient_checkpoint(&mut state, bytes[1].clone())
                .is_err()
        );
    }
}

#[test]
fn transient_checkpoint_retention_rejects_unrequested_overbudget_and_incompatible_evidence() {
    let bytes = images();
    let (mut controller, mut state) = fixture();
    controller
        .current_execution_limits
        .as_mut()
        .unwrap()
        .max_external_data_bytes = 1;
    assert!(
        controller
            .retain_transient_checkpoint(&mut state, bytes[0].clone())
            .is_err()
    );
    controller.current_execution_limits = Some(ResourceLimits::default());
    controller.current_save_policy = crate::simulation::execution::SavePolicy::PlanOwned {
        output_selection_mode: crate::state::OutputSelectionMode::Automatic,
        retained_dataset_limit: 10,
        maximum_storage_bytes: 1,
        live_streaming_enabled: true,
        retain_failure_diagnostics: true,
    };
    assert!(
        controller
            .retain_transient_checkpoint(&mut state, bytes[0].clone())
            .is_err()
    );
    assert!(state.simulation.active_run().unwrap().analyses.is_empty());
    controller.current_save_policy =
        crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
    let terminal = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_provenance(controller.current_provenance.clone().unwrap());
    let run = state
        .simulation
        .retained
        .run_by_sequence_mut(controller.current_run_id.unwrap())
        .unwrap();
    assert!(
        controller
            .retain_analysis_under_current_policy(run, terminal)
            .unwrap_err()
            .contains("without its requested")
    );
    controller
        .current_spec_options
        .as_mut()
        .unwrap()
        .tran_checkpoint
        .as_mut()
        .unwrap()
        .times
        .clear();
    assert!(
        controller
            .retain_transient_checkpoint(&mut state, bytes[0].clone())
            .is_err()
    );
    controller
        .current_spec_options
        .as_mut()
        .unwrap()
        .tran_checkpoint
        .as_mut()
        .unwrap()
        .times
        .push(7e-9);
    controller
        .retain_transient_checkpoint(&mut state, bytes[0].clone())
        .unwrap();
    let retained = state.simulation.active_analysis().unwrap();
    for case in 0..3 {
        let mut invalid = retained.clone();
        match case {
            0 => invalid.analysis_type = AnalysisType::Ac,
            1 => invalid.provenance = None,
            _ => {
                invalid.import_source = Some(rspice_results::result_import::ResultImportSource {
                    source_name: "foreign.raw".into(),
                    format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
                    coordinate: None,
                })
            }
        }
        assert!(invalid.validate_retained_evidence().is_err());
    }
    let errors = controller.seal_failed_run(
        &mut state,
        controller.current_run_id,
        None,
        Some(SimulationRunLifecycle::Aborted),
    );
    assert!(errors.is_empty(), "{errors:?}");
    let retained = state.simulation.active_analysis().unwrap();
    assert!(!retained.is_live_partial());
    assert!(retained.transient_checkpoint.is_some());

    // A destination failure can race a successful, still-unpolled solver.
    // Keep the prior image, but fail the task instead of reporting success or
    // misattributing the retention failure to a user cancellation.
    let (mut controller, mut state) = fixture();
    controller.runner = super::super::test_execution::start_manual_deck(DECK);
    super::super::test_execution::wait_until_finished_unpolled(&controller.runner);
    controller
        .retain_transient_checkpoint(&mut state, bytes[0].clone())
        .unwrap();
    controller.accept_transient_checkpoint(&mut state, Arc::from(&b"corrupt"[..]));
    controller.poll_completion(&mut state, &MockExportWorkflowIo::default());
    let run = state.simulation.active_run().unwrap();
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Failed);
    assert!(!run.success);
    assert_eq!(
        run.analyses[0]
            .transient_checkpoint
            .as_ref()
            .unwrap()
            .bytes(),
        &*bytes[0]
    );
    assert!(
        run.analyses[0]
            .error_message
            .as_deref()
            .unwrap()
            .contains("Could not retain transient checkpoint")
    );
}

#[test]
fn transient_checkpoint_project_schema_and_import_library_are_content_checked() {
    let bytes = images();
    let (controller, mut state) = fixture();
    controller
        .retain_transient_checkpoint(&mut state, bytes[0].clone())
        .unwrap();
    let evidence = state
        .simulation
        .active_analysis()
        .unwrap()
        .transient_checkpoint
        .clone()
        .unwrap();
    let mut encoded = serde_json::to_value(&evidence).unwrap();
    encoded["digest"] = serde_json::to_value(ContentDigest::from_bytes([0; 32])).unwrap();
    assert!(serde_json::from_value::<TransientCheckpointEvidence>(encoded).is_err());
    let mut corrupted = bytes[0].to_vec();
    *corrupted.last_mut().unwrap() ^= 1;
    assert!(TransientCheckpointEvidence::from_bytes(corrupted.into()).is_err());
    let mut limited = ResourceLimits::default();
    limited.max_result_values = 0;
    assert!(
        TransientCheckpointEvidence::from_bytes_with_limits(bytes[0].clone(), limited, &NoAbort)
            .is_err()
    );

    let original = crate::io::capture_simulation_results(&state.simulation);
    for version in [1, 15, 27, 40, 43] {
        let mut legacy = original.clone();
        legacy.schema_version = version;
        assert!(
            legacy
                .migrate_to_current(ProjectId::new())
                .unwrap_err()
                .contains("transient checkpoints")
        );
        assert_eq!(legacy.schema_version, version);
    }
    let mut null = original.clone();
    null.runs[0].analyses[0].transient_checkpoint = PersistedField::Null;
    assert!(
        null.validate()
            .unwrap_err()
            .contains("null transient checkpoint")
    );
    state.simulation.active_run_mut().unwrap().analyses[0].transient_checkpoint = None;
    let mut legacy = crate::io::capture_simulation_results(&state.simulation);
    let digest = legacy.runs[0].dataset_content_digest.clone();
    legacy.schema_version = 43;
    legacy.migrate_to_current(ProjectId::new()).unwrap();
    assert_eq!(legacy.schema_version, 44);
    assert_eq!(legacy.runs[0].dataset_content_digest, digest);

    let mut library = TransientCheckpointLibrary::default();
    assert!(
        library
            .insert_bounded("resume".into(), evidence.clone(), 1)
            .is_err()
    );
    assert!(library.is_empty());
    assert!(library.insert("resume".into(), evidence.clone()).unwrap());
    assert!(
        !library
            .insert("same image".into(), evidence.clone())
            .unwrap()
    );
    let before = library.clone();
    assert!(library.shares_content_with(&before));
    assert!(library.remove(evidence.digest()));
    assert!(!library.shares_content_with(&before));
    // Imported images survive clearing native history and restoring a project.
    let mut state = AppState::default();
    state.simulation.retained.imported_transient_checkpoints = before;
    let persisted = crate::io::capture_simulation_results(&state.simulation);
    assert!(!persisted.is_empty());
    let decoded: ProjectSimulationResults =
        serde_json::from_str(&serde_json::to_string(&persisted).unwrap()).unwrap();
    let restored = crate::io::simulation_state_from_results(decoded).unwrap();
    assert!(restored.retained.runs.is_empty());
    assert_eq!(
        restored
            .retained
            .imported_transient_checkpoints
            .get(evidence.digest()),
        Some(&evidence)
    );
    let mut invalid = persisted;
    invalid.schema_version = 43;
    assert!(
        invalid
            .migrate_to_current(ProjectId::new())
            .unwrap_err()
            .contains("imported transient checkpoints")
    );
}
