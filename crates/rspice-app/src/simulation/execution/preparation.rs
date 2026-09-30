//! Checked generated-design and manual-deck preparation over borrowed inputs.

use super::{
    ExecutionTargetCapabilities, ModelSourceIdentity, PreparationError, PreparationStage,
    PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy, SnapshotParts,
    TouchstoneExportPolicy,
};
use super::{attach_saved_output_contracts, prepare_manual_tasks, prepare_plan_tasks};
use crate::simulation::project_technology::ProjectTechnologyInputs;
use crate::state::SimulationRunIntent;
use rspice_app_types::canonical::content_digest;
use rspice_simulation::analysis_preparation::AnalysisInputs;
use rspice_simulation::capture_ledger::{plan_capture_workload, validate_plan_saved_output_budget};
use rspice_simulation::execution_identity::{drc_receipt_digest, manual_source_receipt_digest};
use rspice_simulation::manual_deck;
use rspice_simulation::model_sources::{
    prepared_project_model_sources, validate_projected_model_binding_authority,
};
use rspice_simulation::netlist_gen::{CrossProbeSnapshot, NetlistSourceData};
use rspice_simulation::netlist_preparation::dependencies::{
    expand_generated_dependencies_with_sealed_sources, expand_manual_dependencies,
};
use rspice_simulation::netlist_preparation::{
    contains_external_include_directive, deferred_external_source_reason, executable_logical_lines,
    measurements, owned_source, reject_deferred_external_sources_with_project_runtimes,
    validated_executable_hierarchy,
};
use rspice_simulation::output_contract::selection::{
    effective_plan_capture, projection_occurrence_nets,
};
use rspice_simulation::preparation::{QueuedAnalysis, validate_prepared_periodic_sources};
use rspice_simulation::project_veriloga::preparation::{
    prepared_configuration_veriloga_runtimes, prepared_model_library_veriloga_runtimes,
    prepared_signed_pdk_veriloga_runtimes, project_veriloga_runtimes_referenced_by,
};
use rspice_simulation::sealed_source::{
    generated_executable_source_digest, manual_executable_source_digest,
};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use std::path::Path;

pub(in crate::simulation) struct RunPreparationInputs<'a, R, A> {
    pub analysis: AnalysisInputs<'a, R, A>,
    pub technology: ProjectTechnologyInputs<'a>,
    pub libraries: &'a rspice_design::library::LibraryCatalog,
    pub project_sources: &'a rspice_design::project_sources::ProjectSourceRegistry,
    pub design_management: &'a rspice_design_model::design_management::DesignManagementCatalog,
    pub configuration_sets: &'a rspice_design::configuration_set::ConfigurationSetCatalog,
    pub source_data: NetlistSourceData<'a>,
    pub schematic_source: rspice_design::projection::SchematicSource<'a>,
    pub imported_checkpoints:
        &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    pub display_waveform_cache_samples: usize,
    pub compilation:
        Option<&'a rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
}

impl<R, A> RunPreparationInputs<'_, R, A> {
    fn plan_data(
        &self,
        plan_id: rspice_app_types::product::SimulationPlanId,
    ) -> Option<&rspice_simulation_contract::plan_payload::SimulationPlanPayload> {
        self.analysis
            .plan_payloads
            .iter()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &record.payload)
    }
}

pub(in crate::simulation) struct ManualSourceInputs<'a> {
    pub source: &'a str,
    pub descriptor: Option<&'a rspice_design::owned_netlist::OwnedNetlistDescriptor>,
    pub generated_artifact: Option<&'a rspice_design::netlist_document::GeneratedArtifact>,
    pub owned: bool,
    pub origin: Option<&'a Path>,
}

pub(in crate::simulation) fn build_analysis_plan(
    state: &rspice_simulation_contract::setup_state::SimulationSetup,
) -> Result<rspice_simulation_contract::plan_model::FrozenSimulationPlan, Vec<String>> {
    let plan = state.analysis_plan.as_ref().ok_or_else(|| {
        vec!["The simulation plan has not been migrated to stable analysis instances".to_owned()]
    })?;
    plan.freeze().map_err(|error| vec![error.to_string()])
}

pub(in crate::simulation) fn build_prepared_run_set<'a, R, A, W>(
    inputs: &RunPreparationInputs<'a, R, A>,
    execution_projection: rspice_design::projection::ConfigurationExecutionProjection,
) -> Result<PreparedRunSnapshot, PreparationError>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    let root_reference = execution_projection.root().clone();
    let root_schematic = execution_projection
        .root_schematic()
        .expect("a successful execution projection has a materialized root");
    if root_schematic.document().components.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::DesignChecks,
            format!(
                "Add a component to configured simulation root '{}' before preparing a run",
                root_reference.display_path()
            ),
        ));
    }
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        inputs.libraries,
        &execution_projection,
    );
    let source_data = inputs.source_data;
    let drc = rspice_simulation::preparation::check_generated_design(root_schematic, &hierarchy)?;
    validate_projected_model_binding_authority(
        inputs.technology.models,
        inputs.technology.resolutions,
        inputs.libraries,
        inputs.technology.project.technology_binding(),
        inputs.technology.registry.validated_packages(),
        &execution_projection,
    )?;

    let plan = build_analysis_plan(inputs.analysis.sim_setup).map_err(|errors| {
        PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
    })?;
    let plan_payload = inputs.plan_data(plan.plan_id()).ok_or_else(|| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Simulation plan {} has no plan-owned variables, outputs, and specifications payload",
                plan.plan_id()
            ),
        )
    })?;
    let specifications = if plan_payload.specification_definitions.is_empty() {
        plan_payload
            .specs
            .iter()
            .cloned()
            .map(rspice_results::specification::PreparedSpecification::new)
            .collect::<Result<Vec<_>, _>>()
    } else {
        plan_payload
            .specification_definitions
            .iter()
            .cloned()
            .map(rspice_results::specification::PreparedSpecification::from_definition)
            .collect::<Result<Vec<_>, _>>()
    }
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Simulation-plan specification is invalid: {error}"),
        )
    })?;
    let specification_policy = rspice_results::specification::PreparedSpecificationPolicy::new(
        plan_payload.specification_policy.clone(),
    )
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Simulation-plan specification policy is invalid: {error}"),
        )
    })?;
    // A project need not have a technology; if it has one it must be
    // valid; if the plan needs one it must have one. A project that owes
    // nothing to a technology seals the plain model library instead.
    inputs
        .technology
        .technology_gate_block_reason()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let has_project_technology = inputs.technology.project_technology_in_effect();
    let sealed_models = if has_project_technology {
        inputs.technology.seal_project_execution_model_sources()
    } else {
        rspice_simulation::model_sources::seal_plan_execution_sources(
            inputs.technology.models,
            inputs.technology.resolutions,
            &inputs.analysis.sim_setup.model_bindings,
        )
    }
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let tasks = prepare_plan_tasks(
        inputs.analysis,
        &plan,
        &sealed_models,
        inputs.imported_checkpoints,
        inputs.schematic_source.current_file,
    )
    .map_err(|errors| PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; ")))?;
    let design_nets = std::sync::Arc::new(
        rspice_design::connectivity::summary::design_nets_with_hierarchy(
            root_schematic,
            &hierarchy,
        ),
    );
    let occurrences =
        projection_occurrence_nets(inputs.libraries, &execution_projection, design_nets);
    let (effective_saved_outputs, used_automatic_outputs, capture_membership) =
        effective_plan_capture(
            inputs.analysis.sim_setup.save_policy.output_selection_mode,
            &plan_payload.saved_outputs,
            &plan_payload.capture_groups,
            &root_schematic.document().probes,
            &occurrences,
            plan.plan_id(),
        )?;
    validate_plan_saved_output_budget(
        &plan_payload.capture_groups,
        &effective_saved_outputs,
        &capture_membership,
        tasks
            .iter()
            .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
        &plan_capture_workload(
            &inputs.analysis.sim_setup.run_set,
            inputs.analysis.sim_setup.reference_pvt,
            tasks.iter().map(|task| (task.instance_id(), task.run_at())),
        ),
        &inputs.analysis.sim_setup.save_policy,
        inputs.display_waveform_cache_samples,
    )?;
    let tasks = attach_saved_output_contracts(tasks, &effective_saved_outputs)?;
    if tasks.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "No runnable analyses were selected",
        ));
    }

    let run_set_config = inputs
        .analysis
        .sim_setup
        .run_set
        .to_corner_config(
            rspice_simulation_contract::corner_config::CornerBaseAnalysis::Op,
            inputs.analysis.sim_setup.reference_pvt,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Run Set is invalid: {error}"),
            )
        })?;
    let run_set_contract = rspice_simulation::analysis_preparation::corner_run_config_from_dialog(
        inputs.analysis.sim_setup,
        &run_set_config,
        &sealed_models,
    )
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!("Run Set model binding failed: {error}"),
        )
    })?;
    let prepared_run_set =
        super::PreparedRunSet::new(inputs.analysis.sim_setup.run_set.clone(), run_set_contract);

    // An FFT card is excluded from the run-level deck on purpose: it is
    // spliced into the deck of the transient it is bound to, and into no
    // other, because it changes the solve it rides on.
    // Authored AC tables likewise belong only to their own task; repeated
    // default table names must not mix the rows of independent analyses.
    let analysis_lines = tasks
        .iter()
        .filter(|task| !matches!(task.queued_analysis().spec, AnalysisSpec::Fft { .. }))
        .filter(|task| task.authored_ac_data_cards().is_none())
        .map(|task| task.queued_analysis().analysis_line.clone())
        .collect::<Vec<_>>();
    let analysis_instances = plan
        .instances()
        .iter()
        .map(rspice_simulation_contract::plan_model::FrozenAnalysisInstance::id)
        .collect::<Vec<_>>();
    let project_veriloga_runtimes = prepared_configuration_veriloga_runtimes(
        inputs.technology.project.id(),
        inputs.project_sources,
        &execution_projection,
    )?;
    let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
        .try_merge(prepared_model_library_veriloga_runtimes(&sealed_models)?)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let project_veriloga_runtimes = project_veriloga_runtimes
        .try_merge(external_veriloga_runtimes.clone())
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
        root_schematic,
        &analysis_lines,
        &hierarchy,
        &plan_payload.design_variables,
        rspice_simulation::netlist_gen::DesignVariableNetlistContext {
            active_cell: &root_reference,
            analysis_instances: &analysis_instances,
        },
        &source_data,
    );
    if !generated.errors.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::Netlist,
            generated.errors.join("; "),
        ));
    }

    let model_execution_plan = sealed_models
        .reference_model_execution_plan(inputs.analysis.sim_setup.reference_pvt.process)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let model_cards = model_execution_plan.model_cards();
    let generated_source = rspice_project::bind_generated_netlist_provenance(
        inputs.design_management,
        inputs.configuration_sets,
        generated.netlist,
    );
    let mut netlist =
        rspice_simulation::analysis_preparation::apply_reference_model_bindings_to_netlist(
            &generated_source,
            &model_cards,
        );
    for runtime in external_veriloga_runtimes.sources() {
        rspice_simulation::netlist_preparation::append_project_veriloga_directive(
            &mut netlist,
            runtime.source_key(),
            runtime.netlist_alias(),
        );
    }
    netlist = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
        &netlist,
        &inputs.analysis.sim_setup.options,
    );
    let (expanded_netlist, sealed_source_dependencies) =
        expand_generated_dependencies_with_sealed_sources(
            &netlist,
            root_schematic.current_file(),
            &rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
                inputs.technology.project.include_search_paths(),
                inputs.technology.project.data_root(),
            ),
            Some(&sealed_models),
        )?;
    netlist = measurements::materialize(
        &expanded_netlist,
        &plan_payload.specification_definitions,
        &plan
            .instances()
            .iter()
            .map(|instance| (instance.id(), instance.draft()))
            .collect(),
    )?;
    let measurement_references =
        rspice_simulation::measurement_references::PreparedMeasurementReferences::capture(
            &netlist,
            &plan_payload.specification_definitions,
        )
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    reject_deferred_external_sources_with_project_runtimes(
        &netlist,
        &project_veriloga_runtimes,
        &measurement_references,
    )?;
    validate_prepared_periodic_sources(
        tasks
            .iter()
            .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
        &netlist,
    )?;
    reject_unresolved_device_models(&netlist, has_project_technology)?;
    reject_deferred_corner_model_sources(
        tasks.iter().map(PreparedTask::queued_analysis),
        &netlist,
    )?;
    let project_model_sources = prepared_project_model_sources(inputs.technology.models, &netlist)?;

    let source_digest = generated_executable_source_digest(&netlist);
    let receipt =
        RunSourceReceipt::SchematicDrc(drc_receipt_digest(root_schematic.topology_version(), &drc));
    let mut model_identities = model_cards
        .iter()
        .enumerate()
        .map(|(index, cards)| {
            ModelSourceIdentity::new(
                format!("reference-model-source-{index}"),
                content_digest("rspice.materialized-model-cards/v1", cards.as_bytes()),
            )
        })
        .collect::<Vec<_>>();
    append_model_execution_plan_identity(&model_execution_plan, &mut model_identities);
    append_signed_pdk_model_identity(&sealed_models, &mut model_identities);
    append_corner_model_identities(
        tasks.iter().map(PreparedTask::queued_analysis),
        &mut model_identities,
    );

    let mut advisories = generated.warnings;
    if used_automatic_outputs {
        advisories.push(if effective_saved_outputs.is_empty() {
        "Automatic output selection found no eligible node voltage; analyses without selected outputs retain their native results.".to_owned()
    } else {
        format!(
            "Automatic output selection retained {} bounded node voltage{} for circuit waveforms. Other analyses retain their native results.",
            effective_saved_outputs.len(),
            if effective_saved_outputs.len() == 1 { "" } else { "s" }
        )
    });
    }
    advisories.extend(
        drc.warnings()
            .into_iter()
            .map(|violation| format!("{} · {}", violation.message, violation.location.display())),
    );
    let touchstone_export = touchstone_export_policy(
        &inputs.analysis.sim_setup.sp,
        inputs.analysis.schematic,
        tasks.iter().map(PreparedTask::queued_analysis),
        root_schematic.current_file(),
    )?;

    PreparedRunSnapshot::new(SnapshotParts {
        measurement_references,
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(plan.plan_id()),
        project_revision: inputs.technology.project.revision().get(),
        topology_revision: root_schematic.topology_version(),
        source_digest,
        reference_process: inputs.analysis.sim_setup.reference_pvt.process,
        reference_temperature_celsius: inputs.analysis.sim_setup.reference_pvt.temperature_celsius,
        run_set: Some(prepared_run_set),
        tasks,
        executable_netlist: netlist,
        save_policy: SavePolicy::PlanOwned {
            output_selection_mode: inputs.analysis.sim_setup.save_policy.output_selection_mode,
            retained_dataset_limit: inputs.analysis.sim_setup.save_policy.retained_dataset_limit,
            maximum_storage_bytes: inputs.analysis.sim_setup.save_policy.maximum_storage_bytes,
            live_streaming_enabled: inputs.analysis.sim_setup.save_policy.live_streaming_enabled,
            retain_failure_diagnostics: inputs
                .analysis
                .sim_setup
                .save_policy
                .retain_failure_diagnostics,
        },
        model_identities,
        project_model_sources,
        specifications,
        specification_policy,
        project_veriloga_runtimes,
        target: ExecutionTargetCapabilities::current(),
        receipt,
        advisories,
        manual_source: None,
        cross_probe: Some(CrossProbeSnapshot {
            source_reference: root_reference,
            point_to_net: generated.point_to_net,
            nets: generated.nets,
            net_segments: generated.net_segments,
            topology_version: root_schematic.topology_version(),
            emission_map: generated.emission_map,
        }),
        touchstone_export,
        sealed_source_dependencies,
    })
}

pub(in crate::simulation) fn build_prepared_manual_deck<R, A>(
    inputs: &RunPreparationInputs<'_, R, A>,
    manual: ManualSourceInputs<'_>,
) -> Result<PreparedRunSnapshot, PreparationError> {
    let source = manual.source;
    let owned_active = manual.owned;
    if source.trim().is_empty() {
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            "Enter a netlist before running",
        ));
    }

    let owned_materialized = if owned_active {
        owned_source::compose_owned_netlist_execution_source(
            manual.descriptor,
            manual.generated_artifact,
            source,
        )
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?
    } else {
        source.to_owned()
    };
    let descriptor = owned_active.then_some(manual.descriptor).flatten();
    let owned_materialized =
        owned_source::adapt_owned_execution_profile(descriptor, &owned_materialized)
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    let has_project_technology = inputs.technology.project_technology_in_effect();
    let sealed_models = if has_project_technology {
        inputs.technology.seal_project_execution_model_sources()
    } else {
        rspice_simulation::model_sources::seal_plan_execution_sources(
            inputs.technology.models,
            inputs.technology.resolutions,
            &inputs.analysis.sim_setup.model_bindings,
        )
    }
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let model_execution_plan = if has_project_technology {
        Some(
            sealed_models
                .reference_model_execution_plan(inputs.analysis.sim_setup.reference_pvt.process)
                .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?,
        )
    } else {
        None
    };
    let model_cards = model_execution_plan.as_ref().map_or_else(
        Vec::new,
        rspice_model_library::ModelExecutionPlan::model_cards,
    );
    let composed = manual_deck::compose_manual_deck_source(&owned_materialized);
    let mut composed =
        rspice_simulation::analysis_preparation::apply_reference_model_bindings_to_netlist(
            &composed,
            &model_cards,
        );
    let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
        .try_merge(prepared_model_library_veriloga_runtimes(&sealed_models)?)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    for runtime in external_veriloga_runtimes.sources() {
        rspice_simulation::netlist_preparation::append_project_veriloga_directive(
            &mut composed,
            runtime.source_key(),
            runtime.netlist_alias(),
        );
    }
    let origin = manual.origin;
    if origin.is_none() && contains_external_include_directive(&composed) {
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            "Relative .include/.inc/.lib sources require an imported deck origin before they can be sealed",
        ));
    }
    let (expanded, canonical_origin, sealed_source_dependencies) = expand_manual_dependencies(
        &composed,
        origin,
        &rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
            inputs.technology.project.include_search_paths(),
            inputs.technology.project.data_root(),
        ),
        &sealed_models,
    )?;
    let expanded = owned_source::bind_execution_profile(
        descriptor.and_then(|descriptor| descriptor.execution_profile),
        expanded,
    )
    .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    reject_unresolved_device_models(&expanded, has_project_technology)?;
    let project_model_sources =
        prepared_project_model_sources(inputs.technology.models, &expanded)?;
    let project_veriloga_runtimes = project_veriloga_runtimes_referenced_by(
        inputs.technology.project.id(),
        inputs.project_sources,
        inputs.compilation,
        &expanded,
    )?
    .try_merge(external_veriloga_runtimes)
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let definitions = inputs
        .analysis
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| inputs.plan_data(plan.id()))
        .map_or(&[][..], |payload| {
            payload.specification_definitions.as_slice()
        });
    let measurement_references =
        rspice_simulation::measurement_references::PreparedMeasurementReferences::capture(
            &expanded,
            definitions,
        )
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    reject_deferred_external_sources_with_project_runtimes(
        &expanded,
        &project_veriloga_runtimes,
        &measurement_references,
    )?;
    let queued_tasks = manual_deck::build_manual_deck_queue(
        inputs.analysis.sim_setup.reference_pvt.temperature_celsius,
        &expanded,
    )
    .map_err(|errors| PreparationError::new(PreparationStage::SourceChecks, errors.join("; ")))?;
    let source_digest = manual_executable_source_digest(&expanded);
    let tasks = prepare_manual_tasks(
        source_digest,
        inputs.technology.project.revision(),
        queued_tasks,
    )?;
    reject_deferred_corner_model_sources(
        tasks.iter().map(PreparedTask::queued_analysis),
        &expanded,
    )?;
    let analysis_config_digests = tasks
        .iter()
        .map(PreparedTask::config_digest)
        .collect::<Vec<_>>();
    let dependency_closure_digest =
        rspice_simulation::execution_identity::sealed_dependency_closure_digest(
            &sealed_source_dependencies,
        );
    let receipt_digest = manual_source_receipt_digest(
        source,
        &expanded,
        canonical_origin.as_deref(),
        dependency_closure_digest,
        &analysis_config_digests,
    );
    let mut model_identities = model_cards
        .iter()
        .enumerate()
        .map(|(index, cards)| {
            ModelSourceIdentity::new(
                format!("reference-model-source-{index}"),
                content_digest("rspice.materialized-model-cards/v1", cards.as_bytes()),
            )
        })
        .collect::<Vec<_>>();
    if let Some(plan) = model_execution_plan.as_ref() {
        append_model_execution_plan_identity(plan, &mut model_identities);
    }
    append_signed_pdk_model_identity(&sealed_models, &mut model_identities);
    append_corner_model_identities(
        tasks.iter().map(PreparedTask::queued_analysis),
        &mut model_identities,
    );
    let touchstone_export = touchstone_export_policy(
        &inputs.analysis.sim_setup.sp,
        inputs.analysis.schematic,
        tasks.iter().map(PreparedTask::queued_analysis),
        origin,
    )?;

    PreparedRunSnapshot::new(SnapshotParts {
        measurement_references,
        intent: SimulationRunIntent::ManualDeck,
        simulation_plan_id: None,
        project_revision: inputs.technology.project.revision().get(),
        topology_revision: inputs.schematic_source.schematic.topology_version(),
        source_digest,
        reference_process: inputs.analysis.sim_setup.reference_pvt.process,
        reference_temperature_celsius: inputs.analysis.sim_setup.reference_pvt.temperature_celsius,
        run_set: None,
        tasks,
        executable_netlist: expanded,
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities,
        project_model_sources,
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        project_veriloga_runtimes,
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::ManualSourceCheck(receipt_digest),
        advisories: Vec::new(),
        manual_source: Some(source.to_owned()),
        cross_probe: None,
        touchstone_export,
        sealed_source_dependencies,
    })
}

pub(in crate::simulation) fn touchstone_export_policy<'a>(
    dialog: &rspice_simulation_contract::sp_draft::SpDialogState,
    schematic: &rspice_design::schematic::document::SchematicDocument,
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    if !tasks
        .into_iter()
        .any(|task| matches!(&task.spec, AnalysisSpec::SParameter { .. }))
    {
        return Ok(TouchstoneExportPolicy::disabled());
    }

    rspice_simulation::preparation::touchstone::touchstone_export_policy_for_dialog(
        dialog,
        schematic,
        source_path,
    )
}

fn reject_unresolved_device_models(
    executable_netlist: &str,
    technology_in_effect: bool,
) -> Result<(), PreparationError> {
    // Parseability and hierarchy resolution belong to earlier stages, which
    // report them in their own words; this check contributes nothing when
    // either fails.
    let Ok(parsed) = rspice_core::netlist::parse_netlist(executable_netlist) else {
        return Ok(());
    };
    let Ok(unresolved) = rspice_core::netlist::unresolved_device_model_references(&parsed) else {
        return Ok(());
    };
    if unresolved.is_empty() {
        return Ok(());
    }

    const LISTED_REFERENCES: usize = 5;
    let listed = unresolved
        .iter()
        .take(LISTED_REFERENCES)
        .map(|reference| {
            format!(
                "{} ({}) references unknown model '{}'",
                reference.element, reference.device_kind, reference.model
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let remaining = unresolved.len().saturating_sub(LISTED_REFERENCES);
    let truncation = if remaining == 0 {
        String::new()
    } else {
        format!("; … and {remaining} more")
    };
    let remedy = if technology_in_effect {
        "The attached technology does not define these models."
    } else {
        "No project technology is attached; attach one that defines these models, or add .MODEL/.subckt definitions to the design."
    };
    Err(PreparationError::new(
        PreparationStage::ModelBindings,
        format!("{listed}{truncation}. {remedy}"),
    ))
}

pub(in crate::simulation) fn reject_deferred_corner_model_sources<'a>(
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    for task in tasks {
        let Some(corner) = task.spec_options.corner.as_ref() else {
            continue;
        };
        for binding in &corner.model_bindings {
            for (line_number, logical_line) in
                executable_logical_lines(&binding.materialized_model_cards)
            {
                if let Some(reason) = deferred_external_source_reason(&logical_line) {
                    return Err(PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Materialized corner model source '{}' contains an unsealed external dependency ({reason}) at line {line_number}: {logical_line}",
                            binding.source_label
                        ),
                    ));
                }
            }
        }
        if corner.model_bindings.is_empty() {
            continue;
        }
        for &process in &corner.process_corners {
            // Use the runner's own composition so root parameters and active
            // subcircuit instances resolve against this corner's actual cards.
            let source = rspice_simulation::netlist_preparation::materialize_corner_process_source(
                executable_netlist,
                corner,
                process,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::ModelBindings, error.to_string())
            })?;
            validated_executable_hierarchy(&source).map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Materialized {process:?} corner source failed validation: {error}"),
                )
            })?;
        }
    }
    Ok(())
}

fn append_corner_model_identities<'a>(
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    identities: &mut Vec<ModelSourceIdentity>,
) {
    for task in tasks {
        let Some(corner) = task.spec_options.corner.as_ref() else {
            continue;
        };
        for binding in &corner.model_bindings {
            identities.push(ModelSourceIdentity::new(
                binding.source_label.clone(),
                content_digest(
                    "rspice.materialized-corner-model-cards/v1",
                    binding.materialized_model_cards.as_bytes(),
                ),
            ));
        }
    }
}

fn append_signed_pdk_model_identity(
    sealed_sources: &rspice_simulation::model_sources::SealedModelExecutionSources,
    identities: &mut Vec<ModelSourceIdentity>,
) {
    if let Some((label, archive_digest)) = sealed_sources.pdk_model_identity() {
        identities.push(ModelSourceIdentity::new(label, archive_digest));
    }
}

fn append_model_execution_plan_identity(
    plan: &rspice_model_library::ModelExecutionPlan,
    identities: &mut Vec<ModelSourceIdentity>,
) {
    let selections = plan
        .selected_library_corners()
        .iter()
        .map(|(library, corner)| format!("{library}={}", corner.as_deref().unwrap_or("top-level")))
        .collect::<Vec<_>>()
        .join(",");
    identities.push(ModelSourceIdentity::new(
        format!(
            "model-execution-plan/{}/{}-bindings/{selections}",
            plan.reference_process().short_name(),
            plan.bindings().len()
        ),
        plan.digest(),
    ));
}
