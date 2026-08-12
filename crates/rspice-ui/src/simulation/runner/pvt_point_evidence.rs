//! Driving a corner run's per-point expansion for tests that judge its results.
//!
//! The prepared snapshot and the spec runner are both private to
//! `crate::simulation`, and a specification's verdict is only worth asserting
//! against measurements the executor really produced. So the expansion, the
//! authorization and the solve all happen here, and the retained results are
//! handed out whole to the surfaces that read them.

use std::collections::HashMap;

use rspice_core::NoAbort;

use crate::product::{ContentDigest, ObjectRevision, SimulationPlanId};
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::execution::{
    ExecutionPermitIssuer, ExecutionTargetCapabilities, PreparedRunSnapshot, PreparedTask,
    RunSourceReceipt, SavePolicy, SnapshotParts, TouchstoneExportPolicy,
};
use crate::simulation::multi_run::{AnalysisRunType, AnalysisSpec};
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
    SimulationRun, SimulationRunIntent, SimulationRunProvenance,
};

use super::SimulationError;

/// Prepare, authorize and run a corner declaration, retaining every result the
/// expansion produced.
///
/// A point that fails to solve is retained as a failed result rather than
/// dropped, because a corner that did not converge is the answer to a
/// specification asked about that corner.
///
/// The run is sealed with the dispatch's own receipt, the way the controller
/// seals one. Without it a test could not tell whether the results it is
/// judging are an authentic ordered prefix of the authorized task graph — which
/// is the property a project save and reload depends on.
pub(crate) fn run_corner_declaration(
    deck: &str,
    contract: crate::services::simulation_runner::CornerRunConfig,
    reference_temperature_celsius: f64,
) -> Result<SimulationRun, String> {
    const TEST_NAMESPACE: uuid::Uuid =
        uuid::Uuid::from_u128(0x0f22_9f3a_51b8_4cd7_9e21_7c60_5d18_a4b3);

    let corner = QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: super::SpecExecutionOptions {
            corner: Some(contract),
            ..super::SpecExecutionOptions::default()
        },
        analysis_line: ".corner".to_owned(),
    };
    let parts = SnapshotParts {
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(SimulationPlanId::from_namespace(
            TEST_NAMESPACE,
            b"pvt-point-evidence-plan",
        )),
        project_revision: 1,
        topology_revision: 1,
        // Nothing in the expansion reads this beyond snapshot identity, and
        // the fixture prepares one deck, so a constant keeps the run stable.
        source_digest: ContentDigest::from_bytes([9; 32]),
        reference_process: ProcessCorner::TT,
        reference_temperature_celsius,
        tasks: vec![PreparedTask::new(
            crate::product::AnalysisInstanceId::from_namespace(TEST_NAMESPACE, b"corner"),
            ObjectRevision::INITIAL,
            Vec::new(),
            "Corner",
            corner,
        )],
        executable_netlist: deck.to_owned(),
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        project_veriloga_runtimes: Default::default(),
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([7; 32])),
        advisories: Vec::new(),
        manual_source: None,
        cross_probe: None,
        touchstone_export: TouchstoneExportPolicy::disabled(),
        sealed_source_dependencies: Vec::new(),
    };

    let snapshot = PreparedRunSnapshot::new(parts).map_err(|error| error.to_string())?;
    let digest = snapshot.digest();
    let permit = ExecutionPermitIssuer::default()
        .issue(digest)
        .map_err(|error| error.to_string())?;
    let proof = permit
        .consume(digest, digest)
        .map_err(|error| error.to_string())?;
    let dispatch = snapshot
        .authorize_dispatch(proof)
        .map_err(|error| error.to_string())?;

    let bridge = crate::simulation::EngineBridge::new();
    // Points are retained through the controller's own conversion. A hand-built
    // result would decide for itself what evidence a point keeps, and the
    // corner family is assembled from exactly that evidence.
    let retainer = crate::simulation::SimulationController::new();
    let mut corner_families = crate::simulation::corner_family::CornerFamilyRegistry::default();
    for task in dispatch.tasks() {
        corner_families.register(task);
    }
    let receipt = dispatch
        .prepared_run_receipt(AnalysisResultSourceDomain::SimulationPlan)
        .map_err(|error| error.to_string())?;
    let mut run = SimulationRun::new(1);
    run.restore_provenance(SimulationRunProvenance::Prepared(receipt))?;
    for (index, task) in dispatch.into_tasks().into_iter().enumerate() {
        let provenance = AnalysisResultProvenance::new_with_authored_source_domain(
            AnalysisResultSourceDomain::SimulationPlan,
            task.instance_id(),
            task.authored_instance_id(),
            task.source_revision(),
            task.snapshot_digest(),
            task.dependencies().to_vec(),
        )
        .map_err(|error| error.to_string())?
        .with_pvt_point(task.pvt_point().cloned());
        let analysis_type = analysis_type_for(task.spec());
        let label = task.label().to_owned();
        let id = u64::try_from(index).unwrap_or(u64::MAX).saturating_add(1);

        // The declaration's own turn assembles the family from the points that
        // already ran, exactly as the controller does. It costs no engine call,
        // which is the whole point of it still being a task.
        if matches!(task.spec(), AnalysisSpec::Corner) {
            let analysis = match corner_families.family_for(task.instance_id(), &run) {
                Ok(result) => retainer.convert_to_analysis_result_with_metadata_owned(
                    result,
                    AnalysisType::Corner,
                    &label,
                ),
                Err(error) => AnalysisResult::failed(id, AnalysisType::Corner, label, error),
            };
            run.add_analysis(analysis.with_provenance(provenance));
            continue;
        }

        let resolved = task
            .resolve_dependency_artifacts(&HashMap::new())
            .map_err(|error| error.to_string())?;
        let (queued, netlist, _runtimes, dependencies) = resolved.into_runner_parts();

        let outcome = match queued.config {
            Some(config) => {
                bridge.run_with_abort_and_source_path(&config, &netlist, None, None, &NoAbort)
            }
            None => super::spec::run_spec_request(
                &bridge,
                queued.spec,
                queued.spec_options,
                &netlist,
                None,
                &dependencies,
                &NoAbort,
            ),
        };

        let analysis = match outcome {
            Ok(result) => retainer.convert_to_analysis_result_with_metadata_owned(
                result,
                analysis_type,
                &label,
            ),
            Err(SimulationError::Aborted) => {
                return Err("corner evidence run was aborted".to_owned());
            }
            Err(error) => AnalysisResult::failed(id, analysis_type, label, error.to_string()),
        };
        run.add_analysis(analysis.with_provenance(provenance));
    }
    // Sealed the way the controller seals a finished batch, so the run a test
    // receives is one a project could actually persist.
    run.mark_running()?;
    run.finish_lifecycle(if run.success {
        crate::state::SimulationRunLifecycle::Completed
    } else {
        crate::state::SimulationRunLifecycle::Failed
    })?;
    Ok(run)
}

fn analysis_type_for(spec: &AnalysisSpec) -> AnalysisType {
    match spec.run_type() {
        AnalysisRunType::DcSweep => AnalysisType::DcSweep,
        AnalysisRunType::Transient => AnalysisType::Transient,
        AnalysisRunType::Ac => AnalysisType::Ac,
        AnalysisRunType::Corner => AnalysisType::Corner,
        _ => AnalysisType::DcOp,
    }
}
