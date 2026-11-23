//! Checked generated-design and manual-deck preparation over borrowed inputs.

use super::run_memory;
pub use super::run_memory::MachineMemory;
use super::{
    ExecutionTargetCapabilities, ModelSourceIdentity, PreparationError, PreparationStage,
    PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy, SnapshotParts,
    TouchstoneExportPolicy,
};
use super::{attach_saved_output_contracts, prepare_manual_tasks, prepare_plan_tasks};
use crate::analysis_preparation::AnalysisInputs;
use crate::capture_ledger::{plan_capture_workload, validate_plan_saved_output_budget};
use crate::execution::SimulationRunIntent;
use crate::execution_identity::{drc_receipt_digest, manual_source_receipt_digest};
use crate::manual_deck;
use crate::model_sources::{
    prepared_project_model_sources_of, validate_projected_model_binding_authority,
};
use crate::netlist_gen::{CrossProbeSnapshot, NetlistSourceData};
use crate::netlist_preparation::dependencies::{
    expand_generated_dependencies_with_sealed_sources, expand_manual_dependencies,
};
use crate::netlist_preparation::{
    ExecutableHierarchy, contains_external_include_directive, deferred_external_source_reason,
    executable_logical_lines, measurements, parse_executable,
    reject_deferred_external_sources_with_project_runtimes, unparsed_executable_source,
    validated_executable_hierarchy_with_limits_and_abort,
};
use crate::output_contract::selection::{effective_plan_capture, projection_occurrence_nets};
use crate::preparation::{QueuedAnalysis, validate_prepared_periodic_sources};
use crate::project_technology::ProjectTechnologyInputs;
use crate::project_veriloga::preparation::{
    deck_delay_select, deck_external_cells, prepared_configuration_veriloga_runtimes,
    prepared_corner_veriloga_runtimes, prepared_model_library_veriloga_runtimes,
    prepared_pdk_veriloga_runtimes, project_veriloga_runtimes_referenced_by,
    rebind_project_veriloga_source_keys,
};
use crate::sealed_source::{generated_executable_source_digest, manual_executable_source_digest};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::saved_output::SavedOutputOrigin;
use rspice_vocabulary::canonical::content_digest;
use rspice_vocabulary::product::ProcessCorner;
use std::path::Path;

pub struct RunPreparationInputs<'a, R, A> {
    pub analysis: AnalysisInputs<'a, R, A>,
    pub technology: ProjectTechnologyInputs<'a>,
    pub libraries: &'a rspice_design::library::LibraryCatalog,
    pub project_sources: &'a rspice_design::project_sources::ProjectSourceRegistry,
    pub design_management: &'a rspice_design_model::design_management::DesignManagementCatalog,
    pub configuration_sets: &'a rspice_design::configuration_set::ConfigurationSetCatalog,
    pub source_data: NetlistSourceData<'a>,
    /// The file the open schematic was loaded from, which task preparation
    /// resolves the schematic's relative references against.
    pub schematic_file: Option<&'a Path>,
    /// The open schematic's topology revision, which a manual deck's run is
    /// recorded against.
    pub schematic_topology_revision: u64,
    pub imported_checkpoints:
        &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    pub imported_transient_checkpoints:
        &'a rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    /// Ceiling on concurrent solver workers for this run, device-local.
    /// `None` keeps the ceiling this build ships; the engine takes the
    /// smaller of whatever it ends up with and the parallelism the host
    /// reports.
    pub max_parallel_workers: Option<usize>,
    /// The memory of the machine the run is prepared on, which bounds the
    /// design it may run. `None` where the host cannot say, as in a
    /// browser: the run keeps the counts this build ships.
    pub machine_memory: Option<MachineMemory>,
    /// Folders this host searches for a deck's `.include` and `.lib` files
    /// after the project's own include search paths. Device-local like the
    /// worker ceiling: the reader's, not the project's.
    pub library_folders: &'a [std::path::PathBuf],
    pub compilation: Option<&'a crate::project_veriloga::receipt::ProjectCompileReceipt>,
    /// The configuration the project checks the configured root under — its
    /// rule deck and its connectivity policy — which the source-check gate
    /// judges the run by. The project's to decide, so the caller passes it.
    pub design_checks: rspice_design::drc::DrcConfig,
    /// The project's Added lines; the configured root's block ends every
    /// deck of the run.
    pub added_lines: &'a [rspice_design::added_lines::AddedLines],
}

impl<R, A> RunPreparationInputs<'_, R, A> {
    /// The resource ceilings this run executes under: the reviewed shipped
    /// limits, with the counts a run materializes sized to the machine and
    /// the worker ceiling the reader chose applied over them.
    fn execution_limits(&self) -> rspice_core::ResourceLimits {
        execution_limits_for(self.max_parallel_workers, self.machine_memory)
    }

    /// `executable_netlist` parsed and flattened once, under this run's
    /// limits, for every check the preparation makes of it and for the
    /// snapshot that seals it.
    fn executable_hierarchy(
        &self,
        executable_netlist: &str,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<ExecutableHierarchy, PreparationError> {
        let limits = self.execution_limits();
        let parsed = parse_executable(executable_netlist, limits, abort)
            .map_err(|error| unparsed_executable_source(error).located_in(executable_netlist))?;
        self.flattened(parsed, abort)
    }

    /// `parsed`, which [`parse_executable`] made under this run's limits,
    /// flattened; a design the machine cannot hold is refused in the
    /// reader's words, saying what it needs and what there is.
    fn flattened(
        &self,
        parsed: rspice_core::Netlist,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<ExecutableHierarchy, PreparationError> {
        let hierarchy = ExecutableHierarchy::of_parsed(parsed, self.execution_limits(), abort);
        let Some(memory) = self.machine_memory else {
            return hierarchy;
        };
        let hierarchy = hierarchy.map_err(|error| run_memory::too_big_for(error, memory))?;
        run_memory::check_fits(hierarchy.devices(), memory)?;
        Ok(hierarchy)
    }

    /// The chain this run's includes resolve through.
    fn include_search(&self) -> crate::netlist_preparation::IncludeSearchChain {
        crate::netlist_preparation::IncludeSearchChain::for_project(
            self.technology.project,
            self.library_folders,
        )
    }

    fn plan_data(
        &self,
        plan_id: rspice_vocabulary::product::SimulationPlanId,
    ) -> Option<&rspice_simulation_contract::plan_payload::SimulationPlanPayload> {
        self.analysis
            .plan_payloads
            .iter()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &record.payload)
    }
}

/// A deck a manual run executes: its text as written, and the file it is
/// read from, which its relative includes resolve against.
pub struct ManualSourceInputs<'a> {
    pub source: &'a str,
    pub origin: Option<&'a Path>,
    /// A deck the project keeps as text, which has no file: its include
    /// lines read the files kept with it.
    pub stored: Option<StoredDeckInputs<'a>>,
}

/// A deck the project keeps as text: its name, and the files kept with it by
/// their paths from it.
#[derive(Debug, Clone, Copy)]
pub struct StoredDeckInputs<'a> {
    pub name: &'a str,
    pub files: &'a [rspice_design::deck::StoredDeckFile],
}

/// The reviewed shipped ceilings, with the counts a run materializes sized
/// to `machine_memory` and the worker ceiling the reader chose applied.
///
/// The shipped counts guard a service against input it did not write. A run
/// of the reader's own design on their own machine is bounded by that
/// machine instead, never below what ships. With neither a machine nor a
/// ceiling the limits are exactly as this build ships them.
///
/// A free function so the arithmetic can be read and tested without
/// assembling a run's worth of borrowed catalogs. What a run reads from
/// files and how deep its hierarchy nests stay at the shipped values.
fn execution_limits_for(
    max_parallel_workers: Option<usize>,
    machine_memory: Option<MachineMemory>,
) -> rspice_core::ResourceLimits {
    let mut limits = rspice_core::ResourceLimits::default();
    if let Some(workers) = max_parallel_workers {
        limits.max_parallel_workers = workers;
    }
    match machine_memory {
        Some(memory) => run_memory::sized_to(limits, memory.installed_bytes),
        None => limits,
    }
}

pub fn build_analysis_plan(
    state: &rspice_simulation_contract::setup_state::SimulationSetup,
) -> Result<rspice_simulation_contract::plan_model::FrozenSimulationPlan, Vec<String>> {
    let plan = state.analysis_plan.as_ref().ok_or_else(|| {
        vec!["The simulation plan has not been migrated to stable analysis instances".to_owned()]
    })?;
    plan.freeze().map_err(|error| vec![error.to_string()])
}

pub fn build_prepared_run_set<'a, R, A, W>(
    inputs: &RunPreparationInputs<'a, R, A>,
    execution_projection: rspice_design::projection::ConfigurationExecutionProjection,
) -> Result<PreparedRunSnapshot, PreparationError>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    build_prepared_run_set_reporting(inputs, execution_projection, &rspice_core::NoAbort, &|_| {})
}

/// [`build_prepared_run_set`], for a caller that runs it off the UI thread:
/// it stops between stages once `abort` is raised, and tells `progress` how
/// many of the run's points have their deck ready. What it prepares is the
/// same, byte for byte.
pub fn build_prepared_run_set_reporting<'a, R, A, W>(
    inputs: &RunPreparationInputs<'a, R, A>,
    execution_projection: rspice_design::projection::ConfigurationExecutionProjection,
    abort: &dyn rspice_core::abort_signal::AbortSignal,
    progress: &dyn Fn(super::PreparationProgress),
) -> Result<PreparedRunSnapshot, PreparationError>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    let root_reference = execution_projection.root().clone();
    let configuration = run_configuration(&execution_projection, inputs.configuration_sets);
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
    let drc = crate::preparation::check_generated_design(
        root_schematic,
        &root_reference,
        &hierarchy,
        inputs.design_checks.clone(),
    )?;
    validate_projected_model_binding_authority(
        inputs.technology.models,
        inputs.technology.resolutions,
        &inputs.technology.sim_setup.model_bindings,
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
    PreparationError::check_abort(abort)?;
    let sealed_models = if has_project_technology {
        inputs.technology.seal_project_execution_model_sources()
    } else {
        crate::model_sources::seal_plan_execution_sources(
            inputs.technology.models,
            inputs.technology.resolutions,
            &inputs.analysis.sim_setup.model_bindings,
        )
    }
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    PreparationError::check_abort(abort)?;
    let tasks = prepare_plan_tasks(
        inputs.analysis,
        &plan,
        &sealed_models,
        inputs.imported_checkpoints,
        inputs.imported_transient_checkpoints,
        inputs.schematic_file,
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
    crate::netlist_preparation::startup_nets::check_startup_nets(
        &inputs.analysis.sim_setup.options,
        &occurrences,
        &plan_payload.design_variables,
    )?;
    let (mut effective_saved_outputs, used_automatic_outputs, capture_membership) =
        effective_plan_capture(
            inputs.analysis.sim_setup.save_policy.output_selection_mode,
            &plan_payload.saved_outputs,
            &plan_payload.capture_groups,
            &root_schematic.document().probes,
            &occurrences,
            plan.plan_id(),
        )?;
    effective_saved_outputs
        .iter_mut()
        .for_each(crate::analysis_preparation::spell_output_as_the_deck);
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
    )?;
    let tasks = attach_saved_output_contracts(tasks, &effective_saved_outputs)?;
    if tasks.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "No runnable analyses were selected",
        ));
    }

    PreparationError::check_abort(abort)?;
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
    let run_set_contract = crate::analysis_preparation::corner_run_config_from_dialog(
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
    // Every process a deck of this run may be materialized for: the Run Set's
    // and each corner run's own.
    let corner_processes = run_set_contract
        .process_corners
        .iter()
        .chain(tasks.iter().flat_map(|task| {
            task.queued_analysis()
                .spec_options
                .corner
                .iter()
                .flat_map(|corner| corner.process_corners.iter())
        }))
        .copied()
        .collect::<Vec<_>>();
    let prepared_run_set =
        super::PreparedRunSet::new(inputs.analysis.sim_setup.run_set.clone(), run_set_contract);
    PreparationError::check_abort(abort)?;

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
    let reference_process = inputs.analysis.sim_setup.reference_pvt.process;
    let model_library_veriloga_runtimes =
        prepared_model_library_veriloga_runtimes(&sealed_models, reference_process)?;
    let corner_veriloga_runtimes = prepared_corner_veriloga_runtimes(
        &sealed_models,
        reference_process,
        &model_library_veriloga_runtimes,
        corner_processes,
    )?;
    let external_veriloga_runtimes = prepared_pdk_veriloga_runtimes(&sealed_models)?
        .try_merge(model_library_veriloga_runtimes)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let generated = crate::netlist_gen::generate_netlist_hierarchical_with_variables(
        root_schematic,
        &analysis_lines,
        &hierarchy,
        &plan_payload.design_variables,
        crate::netlist_gen::DesignVariableNetlistContext {
            active_cell: &root_reference,
            analysis_instances: &analysis_instances,
        },
        &source_data,
    );
    let sim_setup = inputs.analysis.sim_setup;
    let mut defects = generated.defects.clone();
    defects.extend(crate::netlist_gen::plan_clashes(
        root_schematic.document(),
        &root_reference,
        crate::netlist_gen::PlanStatements {
            design_variables: &plan_payload.design_variables,
            options_block: &sim_setup.options.to_spice_options(),
            temperature: sim_setup.plan_states_temperature(),
        },
    ));
    if let Some(error) = crate::netlist_gen::refusal_of(&generated.errors, &defects) {
        return Err(error);
    }
    let generated_netlist = crate::netlist_gen::append_added_lines(
        generated.netlist,
        &root_reference,
        inputs.added_lines,
    )
    .map_err(|refused| PreparationError::new(PreparationStage::Netlist, refused.to_string()))?;

    PreparationError::check_abort(abort)?;
    let model_execution_plan = sealed_models
        .reference_model_execution_plan(inputs.analysis.sim_setup.reference_pvt.process)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let model_cards = model_execution_plan.model_cards();
    let generated_source = rspice_design::netlist_document::bind_generated_netlist_provenance(
        inputs.design_management,
        inputs.configuration_sets,
        generated_netlist,
    );
    let mut netlist = crate::analysis_preparation::apply_reference_model_bindings_to_netlist(
        &generated_source,
        &model_cards,
    );
    for runtime in external_veriloga_runtimes.sources() {
        crate::netlist_preparation::append_project_veriloga_directive(
            &mut netlist,
            runtime.source_key(),
            runtime.netlist_alias(),
        );
    }
    netlist = crate::analysis_preparation::apply_simulation_options_to_netlist(
        &netlist,
        &inputs.analysis.sim_setup.options,
    );
    let (expanded_netlist, sealed_source_dependencies) =
        expand_generated_dependencies_with_sealed_sources(
            &netlist,
            root_schematic.current_file(),
            &inputs.include_search(),
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
    // A configured design is compiled against the cells of the deck it runs
    // in, which a design may instantiate as leaves, and under its min:typ:max
    // selection.
    let project_veriloga_runtimes = prepared_configuration_veriloga_runtimes(
        inputs.technology.project.id(),
        inputs.project_sources,
        &execution_projection,
        &deck_external_cells(&netlist),
        deck_delay_select(&netlist),
    )?
    .try_merge(external_veriloga_runtimes.clone())
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let measurement_references =
        crate::measurement_references::PreparedMeasurementReferences::capture(
            &netlist,
            &plan_payload.specification_definitions,
        )
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    reject_deferred_external_sources_with_project_runtimes(
        &netlist,
        &project_veriloga_runtimes,
        &measurement_references,
    )?;
    // The files the design's code models read, from beside its root sheet.
    let data_files = crate::data_files::PreparedDataFiles::capture(
        &netlist,
        root_schematic.current_file().and_then(Path::parent),
        rspice_core::ResourceLimits::default(),
        abort,
    )?;
    validate_prepared_periodic_sources(
        tasks
            .iter()
            .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
        &netlist,
        &project_veriloga_runtimes,
        &data_files,
        inputs.execution_limits(),
    )?;
    PreparationError::check_abort(abort)?;
    // One parse and one flattening of the deck serve every check below and
    // the snapshot that seals it: a million devices take seconds to flatten.
    let hierarchy = inputs.executable_hierarchy(&netlist, abort)?;
    reject_unresolved_models_in(
        &hierarchy.parsed,
        &hierarchy.flattened,
        has_project_technology,
    )?;
    PreparationError::check_abort(abort)?;
    reject_deferred_corner_model_sources_under(
        tasks.iter().map(PreparedTask::queued_analysis),
        &netlist,
        hierarchy.limits(),
    )?;
    PreparationError::check_abort(abort)?;
    let project_model_sources =
        prepared_project_model_sources_of(inputs.technology.models, &hierarchy)?;

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
    append_pdk_model_identity(&sealed_models, &mut model_identities);
    append_corner_model_identities(
        tasks.iter().map(PreparedTask::queued_analysis),
        &mut model_identities,
    );

    let mut advisories = generated.warnings;
    if used_automatic_outputs {
        // The plan's own outputs and its probes are saved beside the automatic
        // set, so only the automatic ones are counted as its.
        let automatic_outputs = || {
            effective_saved_outputs
                .iter()
                .filter(|output| output.origin == SavedOutputOrigin::Automatic)
        };
        let currents = automatic_outputs()
            .filter(|output| output.source_expression.starts_with("I("))
            .count();
        let automatic = automatic_outputs().count() - currents;
        let asked = effective_saved_outputs.len() - automatic - currents;
        advisories.push(if effective_saved_outputs.is_empty() {
            "Automatic output selection found no eligible node voltage; analyses without selected outputs retain their native results.".to_owned()
        } else {
            format!(
                "Automatic output selection retained {automatic} bounded node voltage{}{}{} for circuit waveforms. Other analyses retain their native results.",
                if automatic == 1 { "" } else { "s" },
                match currents {
                    0 => String::new(),
                    1 => " and 1 device current".to_owned(),
                    currents => format!(" and {currents} device currents"),
                },
                match asked {
                    0 => String::new(),
                    1 => " beside the plan's 1 output and probe".to_owned(),
                    asked => format!(" beside the plan's {asked} outputs and probes"),
                }
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

    PreparedRunSnapshot::new_with_hierarchy(
        SnapshotParts {
            task_source_policy: super::snapshot::TaskSourcePolicy::PreparedObservations,
            execution_limits: inputs.execution_limits(),
            measurement_references,
            data_files,
            hdl_deck_directory: source_directory(root_schematic.current_file()),
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(plan.plan_id()),
            project_revision: inputs.technology.project.revision().get(),
            topology_revision: root_schematic.topology_version(),
            source_digest,
            reference_process: inputs.analysis.sim_setup.reference_pvt.process,
            reference_temperature_celsius: inputs
                .analysis
                .sim_setup
                .reference_pvt
                .temperature_celsius,
            run_set: Some(prepared_run_set),
            tasks,
            executable_netlist: netlist,
            save_policy: SavePolicy::PlanOwned {
                output_selection_mode: inputs.analysis.sim_setup.save_policy.output_selection_mode,
                retained_dataset_limit: inputs
                    .analysis
                    .sim_setup
                    .save_policy
                    .retained_dataset_limit,
                maximum_storage_bytes: inputs.analysis.sim_setup.save_policy.maximum_storage_bytes,
                live_streaming_enabled: inputs
                    .analysis
                    .sim_setup
                    .save_policy
                    .live_streaming_enabled,
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
            corner_veriloga_runtimes,
            target: ExecutionTargetCapabilities::current(),
            receipt,
            advisories,
            manual_source: None,
            cross_probe: Some(CrossProbeSnapshot {
                // The generator read the projected document; every point
                // goes home to the sheet that draws it before the canvas
                // compares it with anything it draws.
                sheets: crate::netlist_gen::net_geometry_by_sheet(
                    inputs
                        .design_management
                        .sheet_catalog(&root_reference.key()),
                    generated.point_to_net,
                    generated.nets,
                    generated.net_segments,
                ),
                source_reference: root_reference,
                topology_version: root_schematic.topology_version(),
                emission_map: generated.emission_map,
                // Each master's nets on its own drawing, with the revision
                // of the buffer the run read, so a descended block probes
                // and annotates against its own geometry.
                masters: generated
                    .master_geometry
                    .into_iter()
                    .filter_map(|master| {
                        let key = master.reference.key();
                        let topology_version = execution_projection
                            .schematic_buffers()
                            .iter()
                            .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))?
                            .1
                            .topology_version();
                        Some(crate::netlist_gen::MasterCrossProbe {
                            sheets: crate::netlist_gen::net_geometry_by_sheet(
                                inputs.design_management.sheet_catalog(&key),
                                master.point_to_net,
                                master.net_to_points,
                                master.net_segments,
                            ),
                            reference: master.reference,
                            topology_version,
                        })
                    })
                    .collect(),
            }),
            touchstone_export,
            sealed_source_dependencies,
            configuration: Some(configuration),
        },
        hierarchy,
        abort,
        progress,
    )
}

/// The configuration `projection` was elaborated under, named as `catalog`
/// names it now: the run records it by identity, its name and its revision.
fn run_configuration(
    projection: &rspice_design::projection::DesignProjection,
    catalog: &rspice_design::configuration_set::ConfigurationSetCatalog,
) -> rspice_results::run_receipt::RunConfiguration {
    let plan = projection.plan();
    let id = plan.configuration_id();
    rspice_results::run_receipt::RunConfiguration {
        id,
        name: id.and_then(|id| catalog.find(id)).map_or_else(
            || "Default".to_owned(),
            |configuration| configuration.name().to_owned(),
        ),
        revision: plan.configuration_revision(),
    }
}

/// A manual deck composed and expanded as its run executes it, before the
/// project Verilog-A designs it names are prepared.
struct ExpandedManualDeck {
    expanded: String,
    canonical_origin: Option<String>,
    sealed_source_dependencies: Vec<rspice_core::netlist::ResolvedIncludeDependency>,
    sealed_models: crate::model_sources::SealedModelExecutionSources,
    model_execution_plan: Option<rspice_model_library::ModelExecutionPlan>,
    model_cards: Vec<String>,
    external_veriloga_runtimes: crate::veriloga::PreparedVerilogARuntimeSet,
    /// The runtimes sealed from the Verilog-A sources the deck's own lines
    /// name, and those sources as they were read.
    deck_hdl_runtimes: crate::veriloga::PreparedVerilogARuntimeSet,
    deck_hdl_dependencies: Vec<rspice_core::netlist::ResolvedIncludeDependency>,
    has_project_technology: bool,
    /// The include expansion's line map and the text it maps, when the deck
    /// included a file.
    source_lines: Option<(
        std::sync::Arc<rspice_core::netlist::ExpandedLineMap>,
        String,
    )>,
}

/// The cells the manual deck `manual` offers its project's HDL designs as
/// leaves, and the min:typ:max selection it keeps in them: those of the deck
/// as its run expands it, which the run compiles each design it names
/// against and under. The editor compiles a design against and under them.
pub fn manual_deck_hdl<R, A>(
    inputs: &RunPreparationInputs<'_, R, A>,
    manual: ManualSourceInputs<'_>,
) -> Result<
    (
        Vec<rspice_veriloga::ExternalCell>,
        rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    ),
    PreparationError,
> {
    expand_manual_deck(inputs, &manual, false).map(|deck| {
        (
            deck_external_cells(&deck.expanded),
            deck_delay_select(&deck.expanded),
        )
    })
}

/// Expand the manual deck `manual` as its run executes it. With
/// `seal_hdl`, the Verilog-A sources its own lines name are read and sealed
/// first, and their lines name the sealed runtimes; without, they are left
/// as written, for a caller that only reads the deck's cells.
fn expand_manual_deck<R, A>(
    inputs: &RunPreparationInputs<'_, R, A>,
    manual: &ManualSourceInputs<'_>,
    seal_hdl: bool,
) -> Result<ExpandedManualDeck, PreparationError> {
    let source = manual.source;
    if source.trim().is_empty() {
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            "the deck is empty",
        ));
    }

    let has_project_technology = inputs.technology.project_technology_in_effect();
    let sealed_models = if has_project_technology {
        inputs.technology.seal_project_execution_model_sources()
    } else {
        crate::model_sources::seal_plan_execution_sources(
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
    let composed = manual_deck::compose_manual_deck_source(source);
    let mut composed = crate::analysis_preparation::apply_reference_model_bindings_to_netlist(
        &composed,
        &model_cards,
    );
    let external_veriloga_runtimes = prepared_pdk_veriloga_runtimes(&sealed_models)?
        .try_merge(prepared_model_library_veriloga_runtimes(
            &sealed_models,
            inputs.analysis.sim_setup.reference_pvt.process,
        )?)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    for runtime in external_veriloga_runtimes.sources() {
        crate::netlist_preparation::append_project_veriloga_directive(
            &mut composed,
            runtime.source_key(),
            runtime.netlist_alias(),
        );
    }
    let origin = manual.origin;
    // An include resolves against the deck's folder; with no file there is
    // no folder, and the process's working directory is never read instead.
    // A deck the project keeps as text reads the files kept with it.
    if origin.is_none() && manual.stored.is_none() && contains_external_include_directive(&composed)
    {
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            "The deck has no file, so its .include and .lib lines have no folder to be read from.",
        ));
    }
    let sealed_hdl = if seal_hdl {
        crate::netlist_preparation::seal_deck_hdl_sources(
            &composed,
            origin,
            &inputs.include_search(),
        )?
    } else {
        crate::netlist_preparation::SealedDeckHdl {
            source: composed,
            ..Default::default()
        }
    };
    // A source line an included file writes is sealed where the expansion
    // puts it, by the file and line that wrote it.
    let crate::netlist_preparation::dependencies::ExpandedManualDependencies {
        expanded,
        canonical_origin,
        dependencies: sealed_source_dependencies,
        source_lines,
    } = expand_manual_dependencies(
        &sealed_hdl.source,
        origin,
        manual.stored,
        &inputs.include_search(),
        &sealed_models,
        &|location| sealed_hdl.included_line(location),
    )?;
    let source_lines = source_lines.map(|lines| (lines, expanded.clone()));
    let crate::netlist_preparation::SealedDeckHdl {
        runtimes: deck_hdl_runtimes,
        dependencies: deck_hdl_dependencies,
        ..
    } = sealed_hdl;
    // A `d_process` program starts from the deck's directory, as the CLI
    // starts it, though the run solves with no source path.
    let expanded = crate::netlist_preparation::resolve_process_programs(
        expanded,
        origin.and_then(Path::parent),
    );
    let expanded = rebind_project_veriloga_source_keys(
        inputs.technology.project.id(),
        inputs.project_sources,
        expanded,
    );
    Ok(ExpandedManualDeck {
        expanded,
        canonical_origin,
        sealed_source_dependencies,
        sealed_models,
        model_execution_plan,
        model_cards,
        external_veriloga_runtimes,
        deck_hdl_runtimes,
        deck_hdl_dependencies,
        has_project_technology,
        source_lines,
    })
}

pub fn build_prepared_manual_deck<R, A>(
    inputs: &RunPreparationInputs<'_, R, A>,
    manual: ManualSourceInputs<'_>,
) -> Result<PreparedRunSnapshot, PreparationError> {
    let source = manual.source;
    let origin = manual.origin;
    let ExpandedManualDeck {
        expanded,
        canonical_origin,
        mut sealed_source_dependencies,
        sealed_models,
        model_execution_plan,
        model_cards,
        external_veriloga_runtimes,
        deck_hdl_runtimes,
        deck_hdl_dependencies,
        has_project_technology,
        source_lines,
    } = expand_manual_deck(inputs, &manual, true)?;
    // A sealed Verilog-A source is a dependency like an include: the run
    // refuses to dispatch once the host no longer gives the bytes it sealed.
    sealed_source_dependencies.extend(deck_hdl_dependencies);
    // One parse and one flattening serve the model checks, the deck's own
    // warnings (a dropped control command, a shadowed definition), which the
    // run advises as a headless run of the same deck does, and the snapshot
    // that seals the run. Each warning is worded as the engine words the
    // parse warnings it also logs, "line 5: …", so a run can tell the engine
    // repeating one from news.
    let parsed = match parse_executable(&expanded, inputs.execution_limits(), &rspice_core::NoAbort)
    {
        Ok(parsed) => parsed,
        Err(error) => {
            // A deck that included files is refused where the card that
            // failed can still be named by the file and line that wrote it.
            if let rspice_core::netlist::ParseWithAbortError::Parse(failure) = &error
                && let Some(lines) = source_lines.as_ref().and_then(|(lines, expansion)| {
                    super::source_lines::aligned_line_map(lines, expansion, &expanded)
                })
            {
                return Err(PreparationError::new(
                    PreparationStage::SourceChecks,
                    super::source_lines::located_parse_message(failure.to_string(), &lines),
                ));
            }
            return Err(unparsed_executable_source(error));
        }
    };
    let advisories = parsed
        .diagnostics
        .iter()
        .map(|diagnostic| match diagnostic.line {
            0 => diagnostic.message.clone(),
            line => format!("line {line}: {}", diagnostic.message),
        })
        .collect();
    let hierarchy = inputs.flattened(parsed, &rspice_core::NoAbort)?;
    let deck_outputs = super::DeckOutputs::of(&hierarchy.parsed);
    reject_unresolved_models_in(
        &hierarchy.parsed,
        &hierarchy.flattened,
        has_project_technology,
    )?;
    let project_model_sources =
        prepared_project_model_sources_of(inputs.technology.models, &hierarchy)?;
    let project_veriloga_runtimes = project_veriloga_runtimes_referenced_by(
        inputs.technology.project.id(),
        inputs.project_sources,
        inputs.compilation,
        &expanded,
    )?
    .try_merge(external_veriloga_runtimes)
    .and_then(|runtimes| runtimes.try_merge(deck_hdl_runtimes))
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
        crate::measurement_references::PreparedMeasurementReferences::capture(
            &expanded,
            definitions,
        )
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    if let Some(refusal) = crate::netlist_preparation::included_hdl_source_refusal(
        &expanded,
        &sealed_source_dependencies,
    ) {
        return Err(refusal);
    }
    reject_deferred_external_sources_with_project_runtimes(
        &expanded,
        &project_veriloga_runtimes,
        &measurement_references,
    )?;
    // The files the deck's code models read, from beside the deck, held
    // only while reading a `.PSS` elaborates it.
    let data_files = crate::data_files::PreparedDataFiles::capture(
        &expanded,
        origin.and_then(Path::parent),
        rspice_core::ResourceLimits::default(),
        &rspice_core::NoAbort,
    )?;
    let _data_files = manual_deck::has_pss_card(&expanded)
        .then(|| data_files.hold(inputs.execution_limits()))
        .transpose()
        .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
    let queued_tasks = manual_deck::build_manual_deck_queue_with_runtimes(
        inputs.analysis.sim_setup.reference_pvt.temperature_celsius,
        &expanded,
        &project_veriloga_runtimes,
    )
    .map_err(|error| {
        // A card an included file wrote is named by that file and line, as
        // a parse of the files names it.
        let lines = source_lines.as_ref().and_then(|(lines, expansion)| {
            super::source_lines::aligned_line_map(lines, expansion, &expanded)
        });
        let messages = error
            .into_messages()
            .into_iter()
            .map(|message| match &lines {
                Some(lines) => super::source_lines::located_parse_message(message, lines),
                None => message,
            });
        PreparationError::new(
            PreparationStage::SourceChecks,
            messages.collect::<Vec<_>>().join("; "),
        )
    })?;
    let source_digest = manual_executable_source_digest(&expanded);
    let mut tasks = prepare_manual_tasks(
        source_digest,
        inputs.technology.project.revision(),
        queued_tasks,
    )?;
    attach_family_members(
        &mut tasks,
        &expanded,
        inputs.analysis.sim_setup.reference_pvt.process,
        inputs.analysis.sim_setup.reference_pvt.temperature_celsius,
    )?;
    reject_deferred_corner_model_sources_under(
        tasks.iter().map(PreparedTask::queued_analysis),
        &expanded,
        hierarchy.limits(),
    )?;
    let analysis_config_digests = tasks
        .iter()
        .map(PreparedTask::config_digest)
        .collect::<Vec<_>>();
    let dependency_closure_digest =
        crate::execution_identity::sealed_dependency_closure_digest(&sealed_source_dependencies);
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
    append_pdk_model_identity(&sealed_models, &mut model_identities);
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

    PreparedRunSnapshot::new_with_hierarchy(
        SnapshotParts {
            task_source_policy: super::snapshot::TaskSourcePolicy::AuthoredDeck,
            execution_limits: inputs.execution_limits(),
            measurement_references,
            data_files,
            hdl_deck_directory: source_directory(origin),
            intent: SimulationRunIntent::ManualDeck,
            simulation_plan_id: None,
            project_revision: inputs.technology.project.revision().get(),
            topology_revision: inputs.schematic_topology_revision,
            source_digest,
            reference_process: inputs.analysis.sim_setup.reference_pvt.process,
            reference_temperature_celsius: inputs
                .analysis
                .sim_setup
                .reference_pvt
                .temperature_celsius,
            run_set: None,
            tasks,
            executable_netlist: expanded,
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities,
            project_model_sources,
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            project_veriloga_runtimes,
            // A manual deck selects its own sections and carries its own corner
            // directives; no deck of its run is materialized for a process.
            corner_veriloga_runtimes: Default::default(),
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::ManualSourceCheck(receipt_digest),
            advisories,
            manual_source: Some(source.to_owned()),
            cross_probe: None,
            touchstone_export,
            sealed_source_dependencies,
            configuration: None,
        },
        hierarchy,
        &rspice_core::NoAbort,
        &|_| {},
    )
    .map(|snapshot| {
        let snapshot = snapshot.with_deck_outputs(deck_outputs);
        match source_lines {
            Some((lines, expansion)) => snapshot.with_source_lines(&expansion, Some(lines)),
            None => snapshot,
        }
    })
}

/// Record on each analysis that solves one trial or point of the deck's
/// `.MC` study or `.STEP` sweep which member it solves, with the value each
/// swept axis takes at a point, and name the task after it.
///
/// A point that varies only the temperature is also a PVT point, attributed
/// against the run's reference process and temperature, so a specification
/// scoped to the nominal point or to a corner reads it as it reads a
/// temperature step's point. A point that varies anything else is attributed
/// to no PVT point: its condition is not one a PVT point can name.
fn attach_family_members(
    tasks: &mut [PreparedTask],
    expanded: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<(), PreparationError> {
    use crate::execution_options::FamilyMember;
    use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};
    let member_of = |task: &PreparedTask| task.queued_analysis().spec_options.member;
    if !tasks.iter().any(|task| member_of(task).is_some()) {
        return Ok(());
    }
    let refused = |error: String| {
        PreparationError::new(PreparationStage::AnalysisPlan, format!(".step: {error}"))
    };
    let points = if tasks
        .iter()
        .any(|task| matches!(member_of(task), Some(FamilyMember::StepPoint { .. })))
    {
        let netlist =
            rspice_core::Netlist::parse(&manual_deck::compose_manual_deck_source(expanded))
                .map_err(|error| refused(error.to_string()))?;
        crate::deck_family::step_members(&netlist, &Default::default()).map_err(refused)?
    } else {
        Vec::new()
    };
    for task in tasks {
        let mut pvt_temperature_celsius = None;
        let record = match member_of(task) {
            None => continue,
            Some(FamilyMember::MonteCarloTrial { index, .. }) => {
                AnalysisResultFamilyMember::MonteCarloTrial { index }
            }
            Some(FamilyMember::StepPoint { index }) => {
                let point = points
                    .get(index)
                    .ok_or_else(|| refused(format!("the sweep has no point {index}")))?;
                pvt_temperature_celsius = point.pvt_temperature_celsius;
                AnalysisResultFamilyMember::StepPoint {
                    index,
                    coordinate: point
                        .coordinate
                        .iter()
                        .map(|(name, value)| AnalysisResultStepCoordinate {
                            name: name.clone(),
                            value: *value,
                        })
                        .collect(),
                }
            }
        };
        task.set_family_member(record);
        if let Some(temperature_celsius) = pvt_temperature_celsius {
            task.attribute_temperature_member(
                temperature_celsius,
                reference_process,
                reference_temperature_celsius,
            )?;
        }
    }
    Ok(())
}

/// The directory of a run's source, where its relative HDL file paths
/// resolve, as `rspice run` resolves them beside its deck and as the run's
/// Touchstone export is written: `None` for a source not saved to a file,
/// whose paths resolve against the working directory.
pub(crate) fn source_directory(source: Option<&Path>) -> Option<std::path::PathBuf> {
    let directory = source?
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())?;
    Some(
        directory
            .canonicalize()
            .unwrap_or_else(|_| directory.to_path_buf()),
    )
}

pub fn touchstone_export_policy<'a>(
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

    crate::preparation::touchstone::touchstone_export_policy_for_dialog(
        dialog,
        schematic,
        source_path,
    )
}

/// Refuse a deck whose devices name a model nothing defines, from the
/// flattening its preparation already made.
pub(crate) fn reject_unresolved_models_in(
    parsed: &rspice_core::Netlist,
    flattened: &rspice_core::netlist::FlattenedNetlist,
    technology_in_effect: bool,
) -> Result<(), PreparationError> {
    let unresolved =
        rspice_core::netlist::unresolved_flattened_device_model_references(parsed, flattened);
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

pub fn reject_deferred_corner_model_sources<'a>(
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    reject_deferred_corner_model_sources_under(
        tasks,
        executable_netlist,
        rspice_core::ResourceLimits::default(),
    )
}

/// [`reject_deferred_corner_model_sources`], flattening each corner's deck
/// under `limits`, the limits the run executes under.
pub(crate) fn reject_deferred_corner_model_sources_under<'a>(
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    executable_netlist: &str,
    limits: rspice_core::ResourceLimits,
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
                            "The corner's model file '{}' reads a file that is not kept with the run ({reason}) at line {line_number}: {logical_line}",
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
            let source = crate::netlist_preparation::materialize_corner_process_source(
                executable_netlist,
                corner,
                process,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::ModelBindings, error.to_string())
            })?;
            validated_executable_hierarchy_with_limits_and_abort(
                &source,
                limits,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| {
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

fn append_pdk_model_identity(
    sealed_sources: &crate::model_sources::SealedModelExecutionSources,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_worker_ceiling_is_the_only_limit_a_preference_moves() {
        let shipped = rspice_core::ResourceLimits::default();
        let limits = execution_limits_for(Some(6), None);

        assert_eq!(limits.max_parallel_workers, 6);
        // Everything a run is allowed to materialize stays where the build
        // put it. A reader asking for fewer threads is not asking for a
        // larger netlist, more result values, or a deeper hierarchy.
        assert_eq!(limits.max_netlist_bytes, shipped.max_netlist_bytes);
        assert_eq!(limits.max_result_values, shipped.max_result_values);
        assert_eq!(limits.max_hierarchy_depth, shipped.max_hierarchy_depth);
        assert_eq!(limits.max_batch_runs, shipped.max_batch_runs);
    }

    /// Choosing nothing on a host that cannot say its memory produces the
    /// shipped limits *exactly*.
    #[test]
    fn choosing_no_ceiling_leaves_the_shipped_limits_untouched() {
        assert_eq!(
            execution_limits_for(None, None),
            rspice_core::ResourceLimits::default()
        );
    }

    /// A desktop run's counts follow the machine's memory, and the run's
    /// worker ceiling still applies over them.
    #[test]
    fn a_machines_memory_sizes_the_counts_a_run_may_materialize() {
        let gigabytes = 1024 * 1024 * 1024;
        let limits = execution_limits_for(
            Some(3),
            Some(MachineMemory {
                installed_bytes: 32 * gigabytes,
                free_bytes: 4 * gigabytes,
            }),
        );
        let shipped = rspice_core::ResourceLimits::default();
        assert_eq!(limits.max_parallel_workers, 3);
        assert!(limits.max_flattened_elements > 4 * shipped.max_flattened_elements);
        assert_eq!(limits.max_netlist_bytes, shipped.max_netlist_bytes);
        assert_eq!(limits.max_hierarchy_depth, shipped.max_hierarchy_depth);
    }

    /// Everything a manual deck's preparation borrows, empty: no project, no
    /// technology, no libraries, as a deck opened on its own has.
    #[derive(Default)]
    struct ManualDeckFixture {
        sim_setup: rspice_simulation_contract::setup_state::SimulationSetup,
        schematic: rspice_design::schematic::document::SchematicDocument,
        selected: std::collections::HashSet<u64>,
        project: rspice_project_contract::ProjectDescriptor,
        models: rspice_model_library::ModelCatalog,
        resolutions: rspice_model_library::ModelResolutionRecords,
        registry: crate::pdk::PdkTechnologyRegistry,
        layouts: std::collections::BTreeMap<
            String,
            rspice_design::physical_layout::PhysicalLayoutDocument,
        >,
        libraries: rspice_design::library::LibraryCatalog,
        project_sources: rspice_design::project_sources::ProjectSourceRegistry,
        design_management: rspice_design_model::design_management::DesignManagementCatalog,
        configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog,
        monte_carlo: rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
        transient: rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    }

    impl ManualDeckFixture {
        fn inputs(&self) -> RunPreparationInputs<'_, (), ()> {
            RunPreparationInputs {
                analysis: AnalysisInputs {
                    sim_setup: &self.sim_setup,
                    schematic: &self.schematic,
                    selected_components: &self.selected,
                    selection_occurrence: None,
                    runs: &[],
                    active_run: None,
                    project_revision: self.project.revision(),
                    plan_payloads: &[],
                },
                technology: ProjectTechnologyInputs {
                    project: &self.project,
                    sim_setup: &self.sim_setup,
                    models: &self.models,
                    resolutions: &self.resolutions,
                    registry: &self.registry,
                    layouts: &self.layouts,
                },
                libraries: &self.libraries,
                project_sources: &self.project_sources,
                design_management: &self.design_management,
                configuration_sets: &self.configuration_sets,
                source_data: NetlistSourceData::new(&crate::netlist_gen::FixtureSourceFiles),
                schematic_file: None,
                schematic_topology_revision: 0,
                imported_checkpoints: &self.monte_carlo,
                imported_transient_checkpoints: &self.transient,
                max_parallel_workers: None,
                machine_memory: None,
                library_folders: &[],
                compilation: None,
                design_checks: rspice_design::drc::DrcConfig::default(),
                added_lines: &[],
            }
        }
    }

    fn manual<'a>(source: &'a str, origin: &'a Path) -> ManualSourceInputs<'a> {
        ManualSourceInputs {
            source,
            origin: Some(origin),
            stored: None,
        }
    }

    /// A manual deck's `.hdl` line used to be refused on the prepared route
    /// as an unsealed external source. Its file is now read from beside the
    /// deck, sealed, and named by key in the deck the run executes; the read
    /// is a dependency of the run, and a file that cannot be read is refused
    /// with its line before anything is solved. A source line of an included
    /// file is sealed too, read relative to that file.
    #[test]
    fn a_manual_decks_verilog_a_source_is_sealed_on_the_prepared_route() {
        let dir = std::env::temp_dir().join(format!("rspice-manual-hdl-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create the deck directory");
        std::fs::write(
            dir.join("vres.va"),
            "`include \"disciplines.vams\"\nmodule vres(p, n);\n  inout p, n;\n  electrical p, n;\n  \
             parameter real r = 1k;\n  analog I(p, n) <+ V(p, n) / r;\nendmodule\n",
        )
        .expect("write the model");
        let deck_path = dir.join("divider.cir");
        let deck =
            "divider\n.hdl \"vres.va\" vres\nV1 in 0 1\nR1 in out 1k\nX1 out 0 vres\n.op\n.end\n";
        let fixture = ManualDeckFixture::default();
        let inputs = fixture.inputs();

        let snapshot = build_prepared_manual_deck(&inputs, manual(deck, &deck_path))
            .unwrap_or_else(|error| panic!("the deck prepares: {error}"));
        let executable = snapshot.executable_netlist();
        assert!(
            executable.contains(".veriloga \"__rspice_project__/deck/")
                && executable.contains("/vres/vres/vres.va\" vres"),
            "{executable}"
        );
        assert!(!executable.contains(".hdl"), "{executable}");
        let [dependency] = snapshot.sealed_source_dependencies() else {
            panic!(
                "one dependency, saw {:?}",
                snapshot.sealed_source_dependencies()
            );
        };
        assert_eq!(dependency.requested_path(), "vres.va");
        assert_eq!(dependency.directive_line(), 2);

        let missing = deck.replace("vres.va", "gone.va");
        let error = build_prepared_manual_deck(&inputs, manual(&missing, &deck_path))
            .expect_err("a file that cannot be read is refused");
        assert_eq!(error.line(), Some(2), "{error}");
        assert!(error.to_string().contains("gone.va"), "{error}");

        // A source line an included file holds is read relative to that file
        // and sealed where the expansion puts it; the include and the source
        // are both dependencies of the run.
        std::fs::create_dir_all(dir.join("models")).expect("create the model directory");
        std::fs::rename(dir.join("vres.va"), dir.join("models").join("vres.va"))
            .expect("move the model beside its include");
        std::fs::write(
            dir.join("models").join("sub.inc"),
            "* models\n.hdl \"vres.va\" vres\n",
        )
        .expect("write the include");
        let included = "divider\n.include \"models/sub.inc\"\nV1 in 0 1\nR1 in out 1k\n\
                        X1 out 0 vres\n.op\n.end\n";
        let snapshot = build_prepared_manual_deck(&inputs, manual(included, &deck_path))
            .unwrap_or_else(|error| panic!("the included source line is sealed: {error}"));
        let executable = snapshot.executable_netlist();
        assert!(
            executable.contains("/vres/vres/vres.va\" vres") && !executable.contains(".hdl"),
            "{executable}"
        );
        let read: Vec<&str> = snapshot
            .sealed_source_dependencies()
            .iter()
            .map(|dependency| dependency.requested_path())
            .collect();
        assert!(
            read.contains(&"models/sub.inc") && read.iter().any(|path| path.ends_with("vres.va")),
            "{read:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A deck of `fanout`^`levels` resistors placed through `levels`
    /// subcircuits, written beside the other test decks as `name`.
    fn nested_deck(name: &str, fanout: usize, levels: usize) -> (String, std::path::PathBuf) {
        let mut deck = format!("{name}\n.subckt l{levels} a b\nR1 a b 1k\n.ends\n");
        for level in (1..levels).rev() {
            deck.push_str(&format!(".subckt l{level} a b\n"));
            for at in 0..fanout {
                deck.push_str(&format!("X{at} a b l{}\n", level + 1));
            }
            deck.push_str(".ends\n");
        }
        for at in 0..fanout {
            deck.push_str(&format!("X{at} in 0 l1\n"));
        }
        deck.push_str("V1 in 0 1\n.op\n.end\n");
        let dir = std::env::temp_dir().join(format!("rspice-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create the deck directory");
        let path = dir.join(format!("{name}.cir"));
        std::fs::write(&path, &deck).expect("write the deck");
        (deck, path)
    }

    /// Flattening is what a large design costs to prepare. Every check of
    /// the devices a deck holds, and the snapshot that seals its run, read
    /// the one flattening its preparation makes.
    #[test]
    fn preparing_a_deck_flattens_its_hierarchy_once() {
        let (deck, path) = nested_deck("flattened-once", 4, 3);
        let fixture = ManualDeckFixture::default();
        let inputs = fixture.inputs();
        let before = rspice_core::netlist::flattenings_on_this_thread();
        build_prepared_manual_deck(&inputs, manual(&deck, &path))
            .unwrap_or_else(|error| panic!("the deck prepares: {error}"));
        assert_eq!(
            rspice_core::netlist::flattenings_on_this_thread() - before,
            1
        );
    }

    /// A run of the reader's own design is bounded by their machine, not by
    /// the count the engine ships for input it did not write: a design past
    /// that count prepares on a machine with memory for it, and a design
    /// the machine has no memory free for is refused before it runs, saying
    /// what it needs and what there is.
    #[test]
    fn a_design_is_bounded_by_the_machine_it_runs_on() {
        let gigabytes = 1024 * 1024 * 1024;
        let shipped = rspice_core::ResourceLimits::default().max_flattened_elements;
        let (deck, path) = nested_deck("machine-sized", 64, 3);
        assert!(64 * 64 * 64 > shipped);
        let fixture = ManualDeckFixture::default();
        let roomy = RunPreparationInputs {
            machine_memory: Some(MachineMemory {
                installed_bytes: 16 * gigabytes,
                free_bytes: 8 * gigabytes,
            }),
            ..fixture.inputs()
        };
        let snapshot = build_prepared_manual_deck(&roomy, manual(&deck, &path))
            .unwrap_or_else(|error| panic!("the machine holds the design: {error}"));
        assert!(snapshot.execution_limits().max_flattened_elements > shipped);

        let busy = RunPreparationInputs {
            machine_memory: Some(MachineMemory {
                installed_bytes: 16 * gigabytes,
                free_bytes: gigabytes / 2,
            }),
            ..fixture.inputs()
        };
        let refused = build_prepared_manual_deck(&busy, manual(&deck, &path))
            .expect_err("half a gigabyte does not hold the design");
        assert_eq!(
            refused.message(),
            "This design has 262,145 devices. Running it needs about 1.3 GB of memory, and this \
             computer has 512 MB free. Close other programs to free memory, or run it on a \
             computer with more."
        );

        // Without a machine to size them by, the shipped count still holds.
        let refused = build_prepared_manual_deck(&fixture.inputs(), manual(&deck, &path))
            .expect_err("the shipped count does not hold the design");
        assert!(refused.resource_limit().is_some(), "{refused}");
    }
}
