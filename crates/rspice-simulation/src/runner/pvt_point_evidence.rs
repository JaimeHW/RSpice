//! Driving a PVT declaration's per-point expansion for tests that judge its
//! results.
//!
//! The prepared snapshot and the spec runner are both private to
//! `crate::simulation`, and a specification's verdict is only worth asserting
//! against measurements the executor really produced. So the expansion, the
//! authorization and the solve all happen here, and the retained results are
//! handed out whole to the surfaces that read them.

use std::collections::HashMap;

use rspice_core::NoAbort;

use crate::execution::SimulationRunIntent;
use crate::execution::{
    ExecutionTargetCapabilities, PreparedRunAuthorization, PreparedRunSnapshot, PreparedTask,
    RunSourceReceipt, SavePolicy, SnapshotParts, TouchstoneExportPolicy,
};
use crate::preparation::QueuedAnalysis;
use rspice_app_types::product::ProcessCorner;
use rspice_app_types::product::{ContentDigest, ObjectRevision, SimulationPlanId};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::provenance::AnalysisResultProvenance;
use rspice_results::provenance::AnalysisResultSourceDomain;
use rspice_results::run::SimulationRun;
use rspice_results::run_receipt::SimulationRunProvenance;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

use crate::error::SimulationError;

const TEST_NAMESPACE: uuid::Uuid = uuid::Uuid::from_u128(0x0f22_9f3a_51b8_4cd7_9e21_7c60_5d18_a4b3);

pub(crate) fn run_standalone_spec(deck: &str, spec: AnalysisSpec) -> super::SimulationResult {
    super::spec::run_spec_request(
        &crate::engine_bridge::EngineBridge::new(),
        spec,
        Default::default(),
        deck,
        None,
        &crate::execution_artifact::ResolvedExecutionDependencies::default(),
        &NoAbort,
    )
    .unwrap()
}

/// Real OP-to-HB execution for output/transport fixtures. Keep the mandatory
/// dependency contract even when a test does not need a complete plan graph.
pub(crate) fn run_hb_spec_with_op(deck: &str, spec: AnalysisSpec) -> super::SimulationResult {
    use crate::execution_artifact::ExecutionArtifactEnvelope;
    use crate::execution_artifact::PreparedDependencyBinding;
    use crate::execution_artifact::ResolvedExecutionDependencies;
    let bridge = crate::engine_bridge::EngineBridge::new();
    let config = rspice_simulation_contract::config::OpConfig::default();
    let result = bridge
        .run(
            &rspice_simulation_contract::config::AnalysisConfig::DcOp(config.clone()),
            deck,
        )
        .unwrap();
    let snapshot = ContentDigest::from_bytes([91; 32]);
    let binding = PreparedDependencyBinding::dc_operating_point_seed(
        rspice_app_types::product::AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([92; 32]),
    );
    let source = rspice_design::netlist_document::content_digest(deck);
    let artifact = ExecutionArtifactEnvelope::from_dc_operating_point_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        source,
        &config,
        &result,
    )
    .unwrap()
    .unwrap();
    let mut dependencies = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    dependencies.bind_source(deck, source);
    super::spec::run_spec_request(
        &bridge,
        spec,
        Default::default(),
        deck,
        None,
        &dependencies,
        &NoAbort,
    )
    .unwrap()
}

/// Prepare, authorize and run a corner declaration, retaining every result the
/// expansion produced.
pub(crate) fn run_corner_declaration(
    deck: &str,
    contract: crate::sweeps::CornerRunConfig,
    reference_temperature_celsius: f64,
) -> Result<SimulationRun, String> {
    run_declaration(
        deck,
        "Corner",
        QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Corner,
            config: None,
            spec_options: crate::execution_options::SpecExecutionOptions {
                corner: Some(contract),
                ..crate::execution_options::SpecExecutionOptions::default()
            },
            analysis_line: ".corner".to_owned(),
        },
        reference_temperature_celsius,
        SavePolicy::RetainEngineProducedResults,
        &[],
    )
}

/// Prepare, authorize and run a temperature step, retaining every result the
/// expansion produced.
pub(crate) fn run_temperature_declaration(
    deck: &str,
    contract: crate::sweeps::TempRunConfig,
    reference_temperature_celsius: f64,
) -> Result<SimulationRun, String> {
    run_declaration(
        deck,
        "Temperature",
        QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options: crate::execution_options::SpecExecutionOptions {
                temp: Some(contract),
                ..crate::execution_options::SpecExecutionOptions::default()
            },
            analysis_line: ".step temp".to_owned(),
        },
        reference_temperature_celsius,
        SavePolicy::RetainEngineProducedResults,
        &[],
    )
}

/// A point that fails to solve is retained as a failed result rather than
/// dropped, because a point that did not converge is the answer to a
/// specification asked about that point.
///
/// The run is sealed with the dispatch's own receipt, the way the controller
/// seals one. Without it a test could not tell whether the results it is
/// judging are an authentic ordered prefix of the authorized task graph — which
/// is the property a project save and reload depends on.
pub(crate) fn run_declaration(
    deck: &str,
    label: &str,
    declaration: QueuedAnalysis,
    reference_temperature_celsius: f64,
    save_policy: SavePolicy,
    outputs: &[rspice_simulation_contract::saved_output::SavedOutput],
) -> Result<SimulationRun, String> {
    let instance = rspice_app_types::product::AnalysisInstanceId::from_namespace(
        TEST_NAMESPACE,
        label.as_bytes(),
    );
    let mut contracts = Vec::new();
    for output in outputs {
        contracts.extend(crate::output_contract::compile_saved_output_contracts(
            output,
            [(instance, &declaration.spec)],
        )?);
    }
    let parts = SnapshotParts {
        task_source_policy: crate::execution::TaskSourcePolicy::PreparedObservations,
        measurement_references: Default::default(),
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
        run_set: None,
        tasks: vec![
            PreparedTask::new(
                instance,
                ObjectRevision::INITIAL,
                Vec::new(),
                label,
                declaration,
            )
            .with_saved_output_contracts(contracts),
        ],
        executable_netlist: deck.to_owned(),
        save_policy,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
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
    let dispatch = PreparedRunAuthorization::default().authorize_campaign_member(snapshot)?;

    // Points are retained through the canonical runtime conversion. A hand-built
    // result would decide for itself what evidence a point keeps, and the
    // family is assembled from exactly that evidence.
    let mut families = crate::point_family::PointFamilyRegistry::default();
    for task in dispatch.tasks() {
        families.register(task);
    }
    let receipt = dispatch
        .prepared_run_receipt(AnalysisResultSourceDomain::SimulationPlan)
        .map_err(|error| error.to_string())?;
    let mut run = SimulationRun::new(1, 0.0, rspice_results::run::ExecutionTarget::current());
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))?;
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
        let outputs = task.saved_output_contracts().to_vec();

        // The declaration's own turn assembles the family from the points that
        // already ran, exactly as the controller does. It costs no engine call,
        // which is the whole point of it still being a task.
        if families.declares(task.instance_id()) {
            let mut analysis = match families.family_for(task.instance_id(), &run) {
                Ok(result) => {
                    crate::result_conversion::convert(result, analysis_type, &label, || 0.0)
                }
                Err(error) => AnalysisResult::failed(id, analysis_type, label, error, 0.0),
            };
            crate::output_contract::apply_saved_output_policy(&mut analysis, save_policy, &outputs);
            run.add_analysis(analysis.with_provenance(provenance));
            continue;
        }

        let resolved = task
            .resolve_dependency_artifacts(&HashMap::new())
            .map_err(|error| error.to_string())?;
        let (queued, netlist, _runtimes, references, dependencies, environment) =
            resolved.into_runner_parts();
        let bridge =
            crate::engine_bridge::EngineBridge::new().with_measurement_references(references);

        let outcome = match queued.config {
            Some(config) => bridge.run_with_abort_and_source_path_and_environment(
                &config,
                &netlist,
                None,
                environment,
                &NoAbort,
            ),
            None => super::spec::run_spec_request_in_context(
                &bridge,
                queued.spec,
                queued.spec_options,
                super::spec::SpecExecutionContext {
                    netlist: &netlist,
                    source_path: None,
                    dependencies: &dependencies,
                    environment,
                    abort_flag: &NoAbort,
                    checkpoint_observer: None,
                },
            ),
        };

        let mut analysis = match outcome {
            Ok(result) => crate::result_conversion::convert(result, analysis_type, &label, || 0.0),
            Err(SimulationError::Aborted) => {
                return Err("the PVT evidence run was aborted".to_owned());
            }
            Err(error) => AnalysisResult::failed(id, analysis_type, label, error.to_string(), 0.0),
        };
        families.capture_result(provenance.source_instance_id(), &analysis);
        crate::output_contract::apply_saved_output_policy(&mut analysis, save_policy, &outputs);
        run.add_analysis(analysis.with_provenance(provenance));
    }
    // Sealed the way the controller seals a finished batch, so the run a test
    // receives is one a project could actually persist.
    run.mark_running()?;
    run.finish_lifecycle(
        if run.success {
            rspice_results::run::SimulationRunLifecycle::Completed
        } else {
            rspice_results::run::SimulationRunLifecycle::Failed
        },
        || Ok(std::time::Duration::ZERO),
    )?;
    Ok(run)
}

fn analysis_type_for(spec: &AnalysisSpec) -> AnalysisType {
    crate::execution_identity::canonical_analysis_kind(spec).result_analysis_type()
}

/// Real OP artifact with exact source binding and worker transport validation.
pub(crate) fn op_dependencies(
    basis: &str,
    op_deck: &str,
    consumer_deck: &str,
    config: rspice_simulation_contract::config::OpConfig,
) -> crate::execution_artifact::ResolvedExecutionDependencies {
    use crate::engine_bridge::EngineBridge;
    use crate::execution_artifact::ExecutionArtifactEnvelope;
    use crate::execution_artifact::PreparedDependencyBinding;
    use crate::execution_artifact::ResolvedExecutionDependencies;
    use rspice_app_types::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    let snapshot = ContentDigest::from_bytes([91; 32]);
    let binding = PreparedDependencyBinding::dc_operating_point_seed(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([92; 32]),
    );
    let result = EngineBridge::new()
        .run(
            &rspice_simulation_contract::config::AnalysisConfig::DcOp(config.clone()),
            op_deck,
        )
        .unwrap();
    let source = rspice_design::netlist_document::content_digest(basis);
    let artifact = ExecutionArtifactEnvelope::from_dc_operating_point_result(
        snapshot,
        binding.producer_instance_id(),
        binding.producer_source_revision(),
        binding.producer_config_digest(),
        source,
        &config,
        &result,
    )
    .unwrap()
    .unwrap();
    let mut dependencies = ResolvedExecutionDependencies::resolve(
        snapshot,
        vec![binding.clone()],
        &HashMap::from([(binding.producer_instance_id(), artifact)]),
    )
    .unwrap();
    dependencies.bind_source(consumer_deck, source);
    let (metadata, buffers) =
        crate::runner::worker_contract::copy_dependency_transfer(&dependencies).unwrap();
    ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap()
}
