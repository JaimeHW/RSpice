//! Controller checkpoint tests cover retained trials, budgets, and preflight evidence.

use super::*;
use crate::product::{ContentDigest, ObjectRevision, SimulationPlanId};
use crate::simulation::runner::{
    SimulationRequest, monte_carlo_checkpoint::tests::completed_checkpoint_fixture,
};
use crate::state::{CanonicalAnalysisKind, PreparedRunReceipt, PreparedRunTaskReceipt};

fn controller_fixture(request: SimulationRequest) -> (SimulationController, AppState) {
    let SimulationRequest::Spec { spec, options } = request else {
        panic!("MC spec")
    };
    let provenance = synthetic_result_provenance();
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        provenance.prepared_snapshot_digest(),
        ContentDigest::from_bytes([21; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([22; 32])),
        vec![
            PreparedRunTaskReceipt::new(
                provenance.source_instance_id(),
                provenance.source_revision(),
                vec![],
                CanonicalAnalysisKind::MonteCarlo.tag(),
                ContentDigest::from_bytes([23; 32]),
            )
            .unwrap(),
        ],
    )
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
    controller.current_analysis_label = Some("MC".into());
    controller.current_provenance = Some(provenance);
    controller.current_spec = Some(*spec);
    controller.current_spec_options = Some(*options);
    (controller, state)
}

#[test]
fn monte_carlo_checkpoint_retention_seals_abort_and_final_completion_drain() {
    let (request, bytes, completed) = completed_checkpoint_fixture();
    let (mut controller, mut state) = controller_fixture(request.clone());
    let before = state.simulation.data_version;
    controller.runner.store_checkpoint_for_test(bytes.clone());
    controller.publish_monte_carlo_checkpoint(&mut state);
    let live = state.simulation.active_analysis().unwrap();
    assert!(live.is_live_partial());
    assert!(!live.success);
    assert!(live.family_metadata.is_none());
    assert_eq!(
        live.monte_carlo_checkpoint.as_ref().unwrap().bytes(),
        &*bytes
    );
    assert_ne!(state.simulation.data_version, before);
    let mut without = live.clone();
    without.monte_carlo_checkpoint = None;
    assert_ne!(live.result_data_digest(), without.result_data_digest());
    assert!(live.retained_storage_bytes() >= without.retained_storage_bytes() + bytes.len() as u64);

    controller
        .runner
        .store_pending_result(Err(SimulationError::Aborted))
        .unwrap();
    controller.poll_completion(&mut state, &MockExportWorkflowIo::default());
    let run = state.simulation.active_run().unwrap();
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
    assert!(!run.success);
    assert_eq!(run.analyses.len(), 1);
    let retained = &run.analyses[0];
    assert!(!retained.is_live_partial());
    assert!(!retained.success);
    assert_eq!(
        retained.monte_carlo_checkpoint.as_ref().unwrap().bytes(),
        &*bytes
    );
    retained.validate_retained_evidence().unwrap();
    run.validate_provenance().unwrap();
    assert!(controller.successful_analysis_instances.is_empty());

    // No frame-level drain: completion must take the last queued checkpoint
    // before the task metadata is consumed and the next task can start.
    let (mut controller, mut state) = controller_fixture(request);
    controller.runner.store_checkpoint_for_test(bytes.clone());
    controller
        .runner
        .store_pending_result(Ok(completed))
        .unwrap();
    controller.poll_completion(&mut state, &MockExportWorkflowIo::default());
    let run = state.simulation.active_run().unwrap();
    assert_eq!(run.lifecycle, SimulationRunLifecycle::Completed);
    assert!(run.success);
    assert_eq!(run.analyses.len(), 1);
    let retained = &run.analyses[0];
    assert!(retained.success);
    assert!(!retained.is_live_partial());
    retained.validate_retained_evidence().unwrap();
    assert_eq!(
        retained.monte_carlo_checkpoint.as_ref().unwrap().bytes(),
        &*bytes
    );
    let mut wrong = retained.clone();
    let Some(AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements,
        ..
    }) = &mut wrong.family_metadata
    else {
        panic!("MC")
    };
    member_measurements[0].measurements[0].value = Some(7.0);
    assert!(
        wrong
            .validate_retained_evidence()
            .unwrap_err()
            .contains("disagree")
    );
}

#[test]
fn monte_carlo_checkpoint_retention_obeys_budget_and_rejects_unrequested_capture() {
    let (request, bytes, completed) = completed_checkpoint_fixture();
    let (mut controller, mut state) = controller_fixture(request);
    controller.current_save_policy = crate::simulation::execution::SavePolicy::PlanOwned {
        output_selection_mode: crate::state::OutputSelectionMode::Automatic,
        retained_dataset_limit: 10,
        maximum_storage_bytes: 1,
        live_streaming_enabled: false,
        retain_failure_diagnostics: false,
    };
    controller.runner.store_checkpoint_for_test(bytes.clone());
    controller.publish_monte_carlo_checkpoint(&mut state);
    assert!(state.simulation.active_run().unwrap().analyses.is_empty());
    controller.current_save_policy =
        crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
    controller
        .current_spec_options
        .as_mut()
        .unwrap()
        .mc_checkpoint = None;
    controller.runner.store_checkpoint_for_test(bytes.clone());
    controller.publish_monte_carlo_checkpoint(&mut state);
    assert!(state.simulation.active_run().unwrap().analyses.is_empty());
    controller
        .current_spec_options
        .as_mut()
        .unwrap()
        .mc_checkpoint = Some(
        crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest {
            publish_every: 1.try_into().unwrap(),
            trial_range: None,
            resume: None,
        },
    );
    let without_checkpoint = controller.convert_to_analysis_result_with_metadata_owned(
        completed,
        AnalysisType::MonteCarlo,
        "MC",
    );
    let error = controller
        .retain_completed_analysis(
            &mut state,
            controller.current_run_id,
            without_checkpoint,
            controller.current_provenance.clone().unwrap(),
        )
        .unwrap_err();
    assert!(error.contains("without its requested retained checkpoint"));
    assert!(state.simulation.active_run().unwrap().analyses.is_empty());
    controller.runner.store_checkpoint_for_test(bytes);
    controller.publish_monte_carlo_checkpoint(&mut state);
    let retained = state.simulation.active_analysis().unwrap();
    assert!(retained.is_live_partial());
    assert_eq!(
        retained
            .monte_carlo_checkpoint
            .as_ref()
            .unwrap()
            .completed_trials(),
        1
    );
}

#[test]
fn monte_carlo_checkpoint_controls_resolve_evidence_before_preparation() {
    use crate::simulation::dialog::{
        McDialogState,
        mc::{McConfig, checkpoint::McCheckpointConfig},
    };
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
    let (request, bytes, _) = completed_checkpoint_fixture();
    let (mut controller, mut state) = controller_fixture(request);
    controller.runner.store_checkpoint_for_test(bytes.clone());
    controller.publish_monte_carlo_checkpoint(&mut state);
    let digest = state
        .simulation
        .active_analysis()
        .unwrap()
        .monte_carlo_checkpoint
        .as_ref()
        .unwrap()
        .digest();
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let (mc, _) = plan.insert(AnalysisKind::MonteCarlo).unwrap();
    plan.edit(mc, |draft| {
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&McConfig {
            base_analysis: Some(op),
            measurements: vec!["scalar:V(out)".into()],
            checkpoint: Some(McCheckpointConfig {
                publish_every: 4.try_into().unwrap(),
                resume: vec![digest],
            }),
            ..Default::default()
        }))
    })
    .unwrap();
    let frozen = plan.freeze().unwrap();
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .unwrap();
    let queue = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let task = queue.iter().find(|task| task.instance_id() == mc).unwrap();
    let options = &task.queued_analysis().spec_options;
    assert_eq!(options.study_base.as_ref().unwrap().instance_id, op);
    let policy = options.mc_checkpoint.as_ref().unwrap();
    assert_eq!(policy.publish_every.get(), 4);
    assert!(
        policy.trial_range.is_none(),
        "authored run range stays on the MC card"
    );
    assert!(
        policy.resume.is_none(),
        "checkpoint is selected only after the point is materialized"
    );
    let input = task.monte_carlo_resumes()[0].input();
    assert_eq!(input.digest(), digest);
    state.simulation.runs.clear();
    assert_eq!(
        input.decode().unwrap().completed_trials(),
        1,
        "prepared bytes outlive retained history"
    );
    assert!(
        controller
            .build_queue_from_plan(&state, &frozen, &sealed)
            .unwrap_err()
            .iter()
            .any(|error| error.contains("no longer retained"))
    );
    let imported = crate::state::MonteCarloCheckpointEvidence::from_bytes(bytes).unwrap();
    state
        .simulation
        .imported_monte_carlo_checkpoints
        .insert("import.rspice-mc".into(), imported)
        .unwrap();
    let imported = controller
        .build_queue_from_plan(&state, &frozen, &sealed)
        .unwrap();
    let imported = imported
        .iter()
        .find(|task| task.instance_id() == mc)
        .unwrap();
    assert_eq!(imported.monte_carlo_resumes()[0].input().digest(), digest);
}
