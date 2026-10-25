//! Preparing a run.
//!
//! Resolves everything a run needs before it starts — the deck, its
//! includes, the sealed model set, and the export policy — so the run either
//! begins fully determined or is refused with a reason.

use std::collections::HashSet;
#[cfg(test)]
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

use super::*;
use crate::simulation::execution::AuthorizedRunDispatch;
use crate::simulation::execution::PreparationError;
use crate::simulation::execution::PreparationStage;
use crate::simulation::execution::PreparedRunMetadata;
use crate::simulation::execution::PreparedRunSnapshot;
#[cfg(test)]
use crate::simulation::execution::TouchstoneExportPolicy;
#[cfg(test)]
use crate::simulation::execution::preparation::reject_deferred_corner_model_sources;
use crate::state::CampaignMemberRefusal;
use rspice_vocabulary::canonical::content_digest;

#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
mod ground_expression_tests;
#[cfg(test)]
pub(crate) mod measurement_tests;

use rspice_simulation::model_sources::validate_projected_model_binding_authority;
#[cfg(test)]
use rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies;
use rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies_with_sealed_sources;
use rspice_simulation::netlist_preparation::measurements;
#[cfg(test)]
use rspice_simulation::netlist_preparation::{
    contains_external_include_directive, reject_deferred_external_sources_with_project_runtimes,
    validated_executable_hierarchy,
};
use rspice_simulation::output_contract::selection::{
    effective_plan_capture, projection_occurrence_nets,
};
#[cfg(test)]
use rspice_simulation::project_veriloga::preparation::prepared_configuration_veriloga_runtimes;
use rspice_simulation::project_veriloga::preparation::{
    prepared_model_library_veriloga_runtimes, prepared_signed_pdk_veriloga_runtimes,
};

/// Let the deck find the data files its sources name.
///
/// Project-relative references resolve against the project's folder; an
/// unsaved project has none, so its references stay as written and the netlist
/// generator reports the ones it cannot check. The stimulus library goes with
/// it, so a file-backed source whose named table is not reachable runs from the
/// copy its definition retains instead of being refused.
fn project_netlist_source_data(
    model: &Model,
) -> rspice_simulation::netlist_gen::NetlistSourceData<'_> {
    rspice_simulation::netlist_gen::NetlistSourceData {
        files: &crate::simulation::table_route::SourceFiles,
        data_root: model.workspace.content.project.data_root(),
        stimulus_library: Some(&model.workspace.content.stimulus_library),
    }
}

type RunPreparationInputs<'a> = crate::simulation::execution::preparation::RunPreparationInputs<
    'a,
    crate::state::SimulationRun,
    crate::state::AnalysisResult,
>;

pub fn run_preparation_inputs(state: &impl RunHost) -> RunPreparationInputs<'_> {
    run_preparation_inputs_reading(state, &crate::simulation::table_route::SourceFiles)
}

/// [`run_preparation_inputs`], with the data files read through `files`.
fn run_preparation_inputs_reading<'a>(
    state: &'a impl RunHost,
    files: &'a dyn rspice_simulation::netlist_gen::NetlistSourceHost,
) -> RunPreparationInputs<'a> {
    let model = state.model();
    let schematic =
        rspice_design::projection::ProjectionSource::projection_source(&model.schematic);
    RunPreparationInputs {
        analysis: analysis_inputs(model),
        technology: model.technology_inputs(),
        libraries: model.library_manager.catalog(),
        project_sources: &model.workspace.content.project_sources,
        design_management: &model.workspace.content.design_management,
        configuration_sets: &model.workspace.content.configuration_sets,
        source_data: rspice_simulation::netlist_gen::NetlistSourceData {
            files,
            ..project_netlist_source_data(model)
        },
        schematic_file: schematic.current_file,
        schematic_topology_revision: schematic.schematic.topology_version(),
        imported_checkpoints: &model.simulation.retained.imported_monte_carlo_checkpoints,
        imported_transient_checkpoints: &model.simulation.retained.imported_transient_checkpoints,
        display_waveform_cache_samples: crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
        max_parallel_workers: model.preferences.solver_worker_limit(),
        library_folders: model.library_folders(),
        compilation: state.veriloga_compilation(),
        design_checks: model.configured_root_design_check_config(),
    }
}

/// What preparing the run set reads from the session, frozen at the press.
///
/// Owned, so the preparation can run on a worker while the reader goes on
/// editing. Each field is a copy of what [`run_preparation_inputs`] borrows,
/// and [`FrozenRunInputs::view`] lends it back in the same shape, so the run
/// prepared from it is the run the synchronous path prepares from the state it
/// was copied from. Freezing copies the design and setup, not the work: the
/// retained runs' samples are shared rather than copied, and no model file is
/// read, no deck generated and nothing parsed until the worker prepares.
pub(crate) struct FrozenRunPreparation {
    projection: rspice_design::projection::ConfigurationExecutionProjection,
    inputs: FrozenRunInputs,
}

struct FrozenRunInputs {
    sim_setup: SimulationSetup,
    schematic: rspice_design::schematic::document::SchematicDocument,
    schematic_file: Option<std::path::PathBuf>,
    schematic_topology_revision: u64,
    selected_components: HashSet<u64>,
    selection_occurrence: Option<rspice_design::occurrence::DocumentOccurrence>,
    runs: Vec<crate::state::SimulationRun>,
    /// Which of `runs` the session had active.
    active_run: Option<usize>,
    plan_payloads: Vec<rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord>,
    project: crate::state::ProjectDescriptor,
    models: rspice_model_library::ModelCatalog,
    resolutions: rspice_model_library::ModelResolutionRecords,
    registry: rspice_simulation::pdk::PdkTechnologyRegistry,
    layouts:
        std::collections::BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    libraries: rspice_design::library::LibraryCatalog,
    project_sources: rspice_design::project_sources::ProjectSourceRegistry,
    design_management: rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog,
    stimulus_library: rspice_design::stimulus_library::library::StimulusLibrary,
    imported_checkpoints: rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    max_parallel_workers: Option<usize>,
    library_folders: Vec<std::path::PathBuf>,
    compilation: Option<rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
    design_checks: rspice_design::drc::DrcConfig,
    /// The data-file host, which the worker reads as it is when it reads,
    /// recording what it answered so dispatch can ask again.
    source_files: super::run_inputs::RecordingSourceFiles,
}

impl FrozenRunPreparation {
    /// Copy what preparing the run set reads. Fails as the synchronous path
    /// does when the configured root cannot be projected.
    pub(crate) fn freeze(state: &impl RunHost) -> Result<Self, PreparationError> {
        let model = state.model();
        let projection = model
            .workspace
            .configuration_execution_projection(
                &model.library_manager,
                &model.workspace.content.active_view,
                &model.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
        let schematic =
            rspice_design::projection::ProjectionSource::projection_source(&model.schematic);
        let runs = model.simulation.retained.runs.to_vec();
        let active_run = model.simulation.active_run().and_then(|active| {
            model
                .simulation
                .retained
                .runs
                .iter()
                .position(|run| std::ptr::eq(run, active))
        });
        Ok(Self {
            projection,
            inputs: FrozenRunInputs {
                sim_setup: SimulationSetup::clone(&model.sim_setup),
                schematic: model.schematic.document().clone(),
                schematic_file: schematic.current_file.map(std::path::Path::to_path_buf),
                schematic_topology_revision: schematic.schematic.topology_version(),
                selected_components: model.schematic.session.editor.selection.components.clone(),
                selection_occurrence: model.workspace.content.active_occurrence().cloned(),
                runs,
                active_run,
                plan_payloads: model.workspace.content.simulation_plan_payloads.clone(),
                project: model.workspace.content.project.clone(),
                models: model.model_library_manager.catalog().clone(),
                resolutions: model.model_library_manager.resolution_records().clone(),
                registry: model.pdk_config.technology_registry.clone(),
                layouts: model.workspace.content.physical_layout_documents().clone(),
                libraries: model.library_manager.catalog().clone(),
                project_sources: model.workspace.content.project_sources.clone(),
                design_management: (*model.workspace.content.design_management).clone(),
                configuration_sets: (*model.workspace.content.configuration_sets).clone(),
                stimulus_library: model.workspace.content.stimulus_library.clone(),
                imported_checkpoints: model
                    .simulation
                    .retained
                    .imported_monte_carlo_checkpoints
                    .clone(),
                imported_transient_checkpoints: model
                    .simulation
                    .retained
                    .imported_transient_checkpoints
                    .clone(),
                max_parallel_workers: model.preferences.solver_worker_limit(),
                library_folders: model.library_folders().to_vec(),
                compilation: state.veriloga_compilation().cloned(),
                design_checks: model.configured_root_design_check_config(),
                source_files: Default::default(),
            },
        })
    }

    /// Prepare the run set from what was frozen: the work
    /// [`SimulationController::prepare_run_set_for_preflight`] does, stopping
    /// between stages once `abort` is raised. Returns, beside the run, what
    /// the data-file host answered while it was prepared.
    pub(super) fn prepare(
        self,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
        progress: &dyn Fn(crate::simulation::execution::PreparationProgress),
    ) -> Result<(PreparedRunSnapshot, super::run_inputs::SourceFileAnswers), PreparationError> {
        let Self { projection, inputs } = self;
        let snapshot = crate::simulation::execution::preparation::build_prepared_run_set_reporting(
            &inputs.view(),
            projection,
            abort,
            progress,
        )?;
        Ok((snapshot, inputs.source_files.into_answers()))
    }
}

impl FrozenRunInputs {
    /// The frozen copy, lent in the shape [`run_preparation_inputs`] lends
    /// the live state.
    fn view(&self) -> RunPreparationInputs<'_> {
        RunPreparationInputs {
            analysis: rspice_simulation::analysis_preparation::AnalysisInputs {
                sim_setup: &self.sim_setup,
                schematic: &self.schematic,
                selected_components: &self.selected_components,
                selection_occurrence: self.selection_occurrence.as_ref(),
                runs: &self.runs,
                active_run: self
                    .active_run
                    .and_then(|index| self.runs.get(index))
                    .map(|run| &run.data),
                project_revision: self.project.revision(),
                plan_payloads: &self.plan_payloads,
            },
            technology: rspice_simulation::project_technology::ProjectTechnologyInputs {
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
            source_data: rspice_simulation::netlist_gen::NetlistSourceData {
                files: &self.source_files,
                data_root: self.project.data_root(),
                stimulus_library: Some(&self.stimulus_library),
            },
            schematic_file: self.schematic_file.as_deref(),
            schematic_topology_revision: self.schematic_topology_revision,
            imported_checkpoints: &self.imported_checkpoints,
            imported_transient_checkpoints: &self.imported_transient_checkpoints,
            display_waveform_cache_samples: crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
            max_parallel_workers: self.max_parallel_workers,
            library_folders: &self.library_folders,
            compilation: self.compilation.as_ref(),
            design_checks: self.design_checks.clone(),
        }
    }
}

/// A campaign outcome for the shell to announce.
///
/// A member that never became a run, and the end of the campaign itself,
/// are not runs a frame can watch reach their end, so the controller hands
/// the shell the fact rather than leaving it to a Console line.
#[derive(Debug, Clone, PartialEq)]
pub enum CampaignOutcome {
    /// A member that could not be started, with the Console line that says
    /// why.
    MemberRefused {
        campaign: String,
        plan: String,
        reason: String,
        log_id: u64,
    },
    /// A campaign that ran out of members without being cancelled, with the
    /// Console line that sums it up. `completed` and `failed` count each
    /// member once.
    Finished {
        name: String,
        completed: u32,
        failed: u32,
        elapsed_seconds: f64,
        log_id: u64,
    },
}

/// What the shell has seen of runs and campaigns, so that each one's end is
/// announced exactly once.
///
/// Every run seen before it ended is watched, not just the newest: a
/// campaign starts its next member in the same update that finishes the
/// last, so by the time a frame looks the newest run has already moved on.
/// Transient by construction.
#[derive(Debug, Clone, Default)]
pub struct RunWatch {
    /// Runs seen before they ended, by stable identity. A run that was over
    /// when it was first seen, such as history opened with a project, is
    /// never in here, so it is never announced as having just finished.
    running: std::collections::HashSet<crate::product::RunId>,
    /// Campaign outcomes the controller reported that the shell has not
    /// announced yet, oldest first.
    campaign_outcomes: Vec<CampaignOutcome>,
}

impl RunWatch {
    /// Watch a run from the moment it opens, before any frame has seen it.
    /// A run that ends inside the update that started it is still one that
    /// was seen running.
    pub fn run_started(&mut self, run_id: crate::product::RunId) {
        self.running.insert(run_id);
    }

    /// Bring the watch up to date with the runs as they stand, newest first,
    /// and return the runs that ended since it last looked, oldest first.
    pub fn observe(
        &mut self,
        runs: impl IntoIterator<Item = (crate::product::RunId, crate::state::SimulationRunLifecycle)>,
    ) -> Vec<crate::product::RunId> {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        let mut running = std::collections::HashSet::new();
        let mut ended = Vec::new();
        for (run_id, lifecycle) in runs {
            if matches!(
                lifecycle,
                Lifecycle::Preparing | Lifecycle::Running | Lifecycle::Cancelling
            ) {
                running.insert(run_id);
            } else if lifecycle.is_terminal() && self.running.contains(&run_id) {
                ended.push(run_id);
            }
        }
        // A run that left the history while running is not announced, and
        // is not kept waiting for an end it can no longer reach.
        self.running = running;
        ended.reverse();
        ended
    }

    /// Hold a campaign outcome until the shell announces it.
    pub fn campaign_outcome(&mut self, outcome: CampaignOutcome) {
        self.campaign_outcomes.push(outcome);
    }

    /// The campaign outcomes not yet announced, oldest first.
    pub fn take_campaign_outcomes(&mut self) -> Vec<CampaignOutcome> {
        std::mem::take(&mut self.campaign_outcomes)
    }
}

fn activate_campaign_plan(
    model: &mut Model,
    plan_id: crate::product::SimulationPlanId,
) -> Result<String, String> {
    let current_id = model.sim_setup.stable_analysis_plan()?.id();
    let plan_name = if current_id == plan_id {
        model.sim_setup.active_plan_name().to_string()
    } else {
        let stored = model
            .sim_setup
            .inactive_plans()
            .iter()
            .find(|plan| plan.id() == plan_id)
            .ok_or_else(|| format!("Simulation plan {plan_id} does not exist"))?;
        if stored.archived() {
            return Err(format!(
                "Archived simulation plan '{}' cannot be queued in a campaign",
                stored.name()
            ));
        }
        stored.name().to_string()
    };
    if current_id != plan_id {
        model.workspace.content.migrate_active_plan_data(current_id);
        model.workspace.content.migrate_inactive_plan_data(plan_id);
        model
            .sim_setup
            .activate_plan(plan_id)
            .map_err(|error| error.to_string())?;
        model
            .workspace
            .content
            .sync_legacy_specs_projection(plan_id);
        model
            .workspace
            .content
            .validate_simulation_configuration()
            .map_err(|error| error.to_string())?;
    }
    Ok(plan_name)
}

impl SimulationController {
    /// Resolve the output set shown by Simulation Studio through the same
    /// configured-root and hierarchy projection used by run preparation.
    /// This keeps Automatic mode's forecast honest when the editor is showing
    /// a child sheet or a different library view than the simulation root.
    pub fn effective_saved_outputs_preflight(
        &self,
        model: &Model,
        explicit: &[crate::state::SavedOutput],
        groups: &[crate::state::CaptureGroup],
    ) -> Result<
        (
            Vec<crate::state::SavedOutput>,
            Vec<rspice_simulation::output_contract::SavedOutputPreflightReport>,
            bool,
            crate::state::CaptureGroupMembership,
        ),
        PreparationError,
    > {
        let selection_mode = model.sim_setup.save_policy.output_selection_mode;
        let projection = model
            .workspace
            .configuration_execution_projection(
                &model.library_manager,
                &model.workspace.content.active_view,
                &model.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
        let root_schematic = projection.root_schematic().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::DesignChecks,
                "The configured simulation root is not materialized",
            )
        })?;
        let plan = model
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        // Net names carry no data-file references, so the projection's own
        // extraction is the same answer as a data-root-bound generator.
        let nets = rspice_design::connectivity::summary::projection_nets(
            model.library_manager.catalog(),
            &projection,
            &projection.root().key(),
        );
        let occurrences =
            projection_occurrence_nets(model.library_manager.catalog(), &projection, nets);
        let (outputs, automatic_fallback, membership) = effective_plan_capture(
            selection_mode,
            explicit,
            groups,
            &root_schematic.document().probes,
            &occurrences,
            plan.id(),
        )?;
        let reports = self.saved_outputs_preflight(model, &outputs);
        Ok((outputs, reports, automatic_fallback, membership))
    }

    /// Build the analysis-independent executable design deck used by
    /// inspection surfaces.
    ///
    /// This is not a preview database. It materializes the configured
    /// hierarchy, plan-owned design variables, reference-process model
    /// sources, simulation options, and include closure through the same
    /// binding helpers used by prepared execution. Consumers can therefore
    /// ask the engine to inspect the exact current design without inventing a
    /// second model-resolution path.
    pub fn prepare_design_netlist_for_inspection(
        model: &Model,
    ) -> Result<String, PreparationError> {
        let projection = model
            .workspace
            .configuration_execution_projection(
                &model.library_manager,
                &model.workspace.content.active_view,
                &model.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
        let root_reference = projection.root().clone();
        let root_schematic = projection.root_schematic().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::DesignChecks,
                "The configured simulation root is not materialized",
            )
        })?;
        validate_projected_model_binding_authority(
            model.model_library_manager.catalog(),
            model.model_library_manager.resolution_records(),
            model.library_manager.catalog(),
            model.workspace.content.project.technology_binding(),
            model.pdk_config.technology_registry.validated_packages(),
            &projection,
        )?;
        let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
            model.library_manager.catalog(),
            &projection,
        );
        let source_data = project_netlist_source_data(model);
        let plan = model
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = model.workspace.content.plan_data(plan.id()).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Simulation plan {} has no plan-owned variables, outputs, and specifications payload",
                    plan.id()
                ),
            )
        })?;
        let analysis_instances = plan
            .instances()
            .iter()
            .filter(|instance| instance.enabled())
            .map(crate::simulation::plan::AnalysisInstance::id)
            .collect::<Vec<_>>();
        let generated =
            rspice_simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
                root_schematic,
                &[],
                &hierarchy,
                &payload.design_variables,
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

        let has_project_technology = model.technology_inputs().project_technology_in_effect();
        let sealed_models = if has_project_technology {
            model
                .technology_inputs()
                .seal_project_execution_model_sources()
        } else {
            model
                .model_library_manager
                .seal_execution_sources_for_plan(&model.sim_setup.model_bindings)
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = if has_project_technology {
            sealed_models
                .reference_model_execution_plan(model.sim_setup.reference_pvt.process)
                .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
                .model_cards()
        } else {
            Vec::new()
        };
        let generated_source = model
            .workspace
            .content
            .bind_generated_netlist_provenance(generated.netlist);
        let mut source =
            rspice_simulation::analysis_preparation::apply_reference_model_bindings_to_netlist(
                &generated_source,
                &model_cards,
            );
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_merge(prepared_model_library_veriloga_runtimes(
                &sealed_models,
                model.sim_setup.reference_pvt.process,
            )?)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        for runtime in external_veriloga_runtimes.sources() {
            rspice_simulation::netlist_preparation::append_project_veriloga_directive(
                &mut source,
                runtime.source_key(),
                runtime.netlist_alias(),
            );
        }
        let source = rspice_simulation::analysis_preparation::apply_simulation_options_to_netlist(
            &source,
            &model.sim_setup.options,
        );
        let (source, _) = expand_generated_dependencies_with_sealed_sources(
            &source,
            root_schematic.current_file(),
            &model.include_search_chain(),
            Some(&sealed_models),
        )?;
        Ok(source)
    }

    /// Include explicitly authored measurements in the generated primary and
    /// its exports. Operand validation waits for preparation, where includes
    /// and model-owned parameters have been resolved into the execution deck.
    pub fn append_plan_measurements_to_generated_netlist(
        model: &Model,
        source: &str,
    ) -> Result<String, PreparationError> {
        let plan = model
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = model
            .workspace
            .content
            .plan_data(plan.id())
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    "Missing active plan payload",
                )
            })?;
        measurements::append_to_generated_source(
            source,
            &payload.specification_definitions,
            &plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .map(|instance| (instance.id(), instance.draft()))
                .collect(),
        )
    }

    /// Validate the exact visible manual-deck document through the same
    /// dependency expansion, source checks, task construction, model binding,
    /// and execution-target contract used immediately before dispatch.
    ///
    /// The validated snapshot is retained behind a one-shot execution permit,
    /// with the identity of everything it was prepared from. Run dispatches
    /// this very snapshot once none of it has moved, the deck's text, its
    /// file, its includes and its model files among it, so a dependency
    /// changed after validation cannot execute under a stale receipt; nothing
    /// is prepared again in the frame that starts it.
    pub fn validate_manual_deck_document(
        &mut self,
        state: &impl RunHost,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        self.clear_prepared_run();
        let snapshot = Self::build_prepared_snapshot(state, SimulationRunIntent::ManualDeck)?;
        let identity = super::run_inputs::RunInputIdentity::of_manual_deck(state, &snapshot)?;
        let metadata = snapshot.metadata();
        self.retain_prepared_run(
            snapshot,
            super::run_inputs::RetainedRunInputs {
                identity,
                source_files: Default::default(),
            },
        )?;
        Ok(metadata)
    }

    /// Produce and retain the exact run-set tuple the preflight report
    /// authorizes, in the caller's frame. The caller runs DRC immediately
    /// before this method. Preflight itself prepares off the UI thread; see
    /// [`Self::start_preflight`].
    ///
    /// Automation and tuning prepare their runs here. The identity of the
    /// state the run is prepared from is retained with it, and the frame that
    /// dispatches it checks that identity, as it checks a Run press's.
    /// Automation prepares through [`Self::prepare_run_set_for_evidence`].
    pub fn prepare_run_set_for_preflight(
        &mut self,
        state: &impl RunHost,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        let identity = super::run_inputs::RunInputIdentity::of_run_set(state);
        let files = super::run_inputs::RecordingSourceFiles::default();
        let snapshot = Self::build_prepared_run_set_reading(state, &files)?;
        let metadata = snapshot.metadata();
        self.retain_prepared_run(
            snapshot,
            super::run_inputs::RetainedRunInputs {
                identity,
                source_files: files.into_answers(),
            },
        )?;
        Ok(metadata)
    }

    /// [`Self::prepare_run_set_for_preflight`], for a workflow that publishes
    /// the results of the run it dispatches: beside the run's metadata, what
    /// the run was prepared from, shared with the retained run. The dispatch
    /// spends the retained run; the workflow keeps these inputs past it, and
    /// publishes only while [`super::PreparedRunSetInputs::still_current`]
    /// holds.
    pub fn prepare_run_set_for_evidence(
        &mut self,
        state: &impl RunHost,
    ) -> Result<(PreparedRunMetadata, super::PreparedRunSetInputs), PreparationError> {
        let metadata = self.prepare_run_set_for_preflight(state)?;
        let inputs = self
            .retained_run_inputs
            .as_ref()
            .zip(self.run_authorization.retained_snapshot())
            .and_then(|(inputs, snapshot)| super::PreparedRunSetInputs::of(inputs, snapshot))
            .ok_or_else(super::run_inputs::unaccounted_run)?;
        Ok((metadata, inputs))
    }

    /// Retain `snapshot` for dispatch, replacing whatever was retained, with
    /// what it was prepared from, which its dispatch checks.
    pub(super) fn retain_prepared_run(
        &mut self,
        snapshot: PreparedRunSnapshot,
        inputs: super::run_inputs::RetainedRunInputs,
    ) -> Result<(), PreparationError> {
        self.retained_run_generation = self.retained_run_generation.wrapping_add(1);
        self.retained_run_inputs = None;
        self.run_authorization.retain(snapshot)?;
        self.retained_run_inputs = Some(std::sync::Arc::new(inputs));
        Ok(())
    }

    /// Freeze and start a declared-order campaign of independent simulation
    /// plans. Every member is prepared in a cloned project view before any
    /// engine work begins, so a later member never observes edits made while
    /// an earlier member is executing.
    pub fn prepare_and_start_campaign(
        &mut self,
        state: &mut impl RunHost,
        name: &str,
        member_ids: &[crate::product::SimulationPlanId],
    ) -> Result<super::SimulationCampaignDispatchReceipt, String> {
        const MAX_CAMPAIGN_MEMBERS: usize = 64;

        if self.has_active_batch() || self.active_campaign.is_some() {
            return Err(
                "A simulation run or campaign is already active; stop it before queuing another campaign"
                    .to_owned(),
            );
        }
        let name = name.trim();
        if name.is_empty() {
            return Err("Campaign name must not be empty".to_owned());
        }
        if name.chars().count() > 160 {
            return Err("Campaign name must not exceed 160 characters".to_owned());
        }
        let mut seen = HashSet::with_capacity(member_ids.len());
        let member_ids = member_ids
            .iter()
            .copied()
            .filter(|id| seen.insert(*id))
            .collect::<Vec<_>>();
        if member_ids.len() < 2 {
            return Err("A simulation campaign requires at least two distinct plans".to_owned());
        }
        if member_ids.len() > MAX_CAMPAIGN_MEMBERS {
            return Err(format!(
                "A simulation campaign may contain at most {MAX_CAMPAIGN_MEMBERS} plans"
            ));
        }

        let mut frozen_state = state.clone();
        let mut members = VecDeque::with_capacity(member_ids.len());
        let mut task_count = 0_usize;
        for plan_id in member_ids {
            let plan_name = activate_campaign_plan(frozen_state.model_mut(), plan_id)?;
            let snapshot =
                Self::build_prepared_snapshot(&frozen_state, SimulationRunIntent::SimulateRunSet)
                    .map_err(|error| {
                    format!("Campaign member '{plan_name}' is not runnable: {error}")
                })?;
            if snapshot.simulation_plan_id() != Some(plan_id) {
                return Err(format!(
                    "Campaign member '{plan_name}' prepared under the wrong simulation-plan identity"
                ));
            }
            task_count = task_count.saturating_add(snapshot.metadata().task_count);
            members.push_back(super::PreparedCampaignMember {
                plan_name,
                snapshot,
            });
        }

        let campaign_id = crate::product::SimulationCampaignId::new();
        let member_count = members.len();
        self.clear_prepared_run();
        state.runs_mut().preflight.invalidate();
        self.design_execution_epoch = state.model().design_execution_epoch;
        self.active_campaign = Some(super::ActiveSimulationCampaign {
            id: campaign_id,
            name: name.to_owned(),
            member_count: member_count as u32,
            dispatched_count: 0,
            completed_count: 0,
            failed_count: 0,
            cancelled: false,
            pending: members,
            started: crate::time_compat::Instant::now(),
        });
        state.model_mut().record(
            LogSeverity::Info,
            crate::diagnostics::LogSource::Run,
            format!("Campaign '{name}' queued: {member_count} plans, {task_count} analyses"),
        );
        self.dispatch_next_campaign_member(state)?;
        Ok(super::SimulationCampaignDispatchReceipt {
            campaign_id,
            member_count,
            task_count,
        })
    }

    pub(super) fn dispatch_next_campaign_member(
        &mut self,
        state: &mut impl RunHost,
    ) -> Result<(), String> {
        loop {
            let Some(mut campaign) = self.active_campaign.take() else {
                return Ok(());
            };
            let Some(member) = campaign.pending.pop_front() else {
                let (outcome, severity) = if campaign.cancelled {
                    ("cancelled", LogSeverity::Warning)
                } else if campaign.failed_count == 0 {
                    ("completed", LogSeverity::Info)
                } else {
                    ("completed with errors", LogSeverity::Error)
                };
                // Every member that finished is counted in `completed_count`,
                // the failed ones too; the summary counts each member once.
                let completed = campaign
                    .completed_count
                    .saturating_sub(campaign.failed_count);
                let summary = state.model_mut().record(
                    severity,
                    crate::diagnostics::LogSource::Run,
                    format!(
                        "Campaign '{}' {outcome}: {completed} completed, {} failed",
                        campaign.name, campaign.failed_count
                    ),
                );
                state.model_mut().simulation.execution.status = format!("Campaign {outcome}");
                // A campaign the reader cancelled is not news to them.
                if !campaign.cancelled {
                    state
                        .runs_mut()
                        .run_watch
                        .campaign_outcome(CampaignOutcome::Finished {
                            name: campaign.name,
                            completed,
                            failed: campaign.failed_count,
                            elapsed_seconds: campaign.started.elapsed().as_secs_f64(),
                            log_id: summary,
                        });
                }
                return Ok(());
            };
            campaign.dispatched_count = campaign.dispatched_count.saturating_add(1);
            let membership = match crate::state::SimulationCampaignMembership::new(
                campaign.id,
                campaign.name.clone(),
                campaign.dispatched_count,
                campaign.member_count,
            ) {
                Ok(membership) => membership,
                Err(error) => {
                    let member_index = campaign.dispatched_count;
                    self.active_campaign = Some(campaign);
                    let line = state.model_mut().record(
                        LogSeverity::Error,
                        crate::diagnostics::LogSource::Run,
                        format!(
                            "Campaign member '{}' could not start: {error}",
                            member.plan_name
                        ),
                    );
                    self.refuse_campaign_member(
                        state,
                        member_index,
                        &member.plan_name,
                        CampaignMemberRefusal::Start,
                        &error,
                        line,
                    );
                    continue;
                }
            };
            let member_name = member.plan_name;
            let dispatch = self
                .run_authorization
                .authorize_campaign_member(member.snapshot);
            self.active_campaign = Some(campaign);
            let dispatch = match dispatch {
                Ok(dispatch) => dispatch,
                Err(error) => {
                    let line = state.model_mut().record(
                        LogSeverity::Error,
                        crate::diagnostics::LogSource::Run,
                        format!("Campaign member '{member_name}' could not be authorized: {error}"),
                    );
                    self.refuse_campaign_member(
                        state,
                        membership.member_index(),
                        &member_name,
                        CampaignMemberRefusal::Authorization,
                        &error,
                        line,
                    );
                    continue;
                }
            };
            let member_index = membership.member_index();
            match self.start_authorized_dispatch(state, dispatch, Some((membership, &member_name)))
            {
                Ok(()) => return Ok(()),
                Err(error) => {
                    let line = state.model_mut().record(
                        LogSeverity::Error,
                        crate::diagnostics::LogSource::Run,
                        format!("Campaign member '{member_name}' could not start: {error}"),
                    );
                    self.refuse_campaign_member(
                        state,
                        member_index,
                        &member_name,
                        CampaignMemberRefusal::Start,
                        &error.to_string(),
                        line,
                    );
                }
            }
        }
    }

    /// Count a member that never became a run among the campaign's failures,
    /// and file it where its failures would have been filed had it run.
    ///
    /// The Console line is the caller's, which knows what refused it, and it
    /// is the event. This is the condition the event leaves: the campaign's
    /// outcome is short one member, the Problems page names that member
    /// beside the ones that ran until a later run supersedes them all, and
    /// the shell announces it as a member that failed, since no run of it
    /// will ever end for the shell to see.
    fn refuse_campaign_member(
        &mut self,
        state: &mut impl RunHost,
        member_index: u32,
        member_name: &str,
        refusal: CampaignMemberRefusal,
        error: &str,
        line: u64,
    ) {
        let campaign = self
            .active_campaign
            .as_mut()
            .expect("a member is refused while its campaign is installed");
        campaign.completed_count = campaign.completed_count.saturating_add(1);
        campaign.failed_count = campaign.failed_count.saturating_add(1);
        let facts = state.slot_facts();
        state.model_mut().publish_campaign_member_refusal(
            campaign.id,
            member_index,
            member_name,
            refusal,
            error,
            facts,
        );
        state
            .runs_mut()
            .run_watch
            .campaign_outcome(CampaignOutcome::MemberRefused {
                campaign: campaign.name.clone(),
                plan: member_name.to_owned(),
                reason: format!("It could not be started: {error}"),
                log_id: line,
            });
    }

    pub(super) fn complete_campaign_member(&mut self, state: &mut impl RunHost, succeeded: bool) {
        let Some(campaign) = self.active_campaign.as_mut() else {
            return;
        };
        campaign.completed_count = campaign.completed_count.saturating_add(1);
        if !succeeded {
            campaign.failed_count = campaign.failed_count.saturating_add(1);
        }
        if let Err(error) = self.dispatch_next_campaign_member(state) {
            state.model_mut().record(
                LogSeverity::Error,
                crate::diagnostics::LogSource::Run,
                format!("Campaign stopped: {error}"),
            );
            self.active_campaign = None;
            state.model_mut().simulation.execution.status = "Campaign stopped".to_owned();
        }
    }

    pub fn clear_prepared_run(&mut self) {
        self.retained_run_generation = self.retained_run_generation.wrapping_add(1);
        self.retained_run_inputs = None;
        self.run_authorization.clear();
    }

    /// Whether the run retained as checked would be dispatched as it stands:
    /// nothing it was prepared from has moved, judged as its dispatch judges
    /// it. `false` where no run is retained.
    pub(crate) fn retained_run_is_current(&self, state: &impl RunHost) -> bool {
        let (Some(inputs), Some(snapshot)) = (
            self.retained_run_inputs.as_ref(),
            self.run_authorization.retained_snapshot(),
        ) else {
            return false;
        };
        inputs.still_current(state, snapshot).is_ok()
    }

    pub fn has_retained_manual_authorization(
        &self,
        expected_snapshot_digest: crate::product::ContentDigest,
    ) -> bool {
        self.run_authorization
            .retained_snapshot()
            .is_some_and(|snapshot| {
                snapshot.intent() == SimulationRunIntent::ManualDeck
                    && snapshot.digest() == expected_snapshot_digest
            })
    }

    /// Validate and consume an explicitly retained preflight snapshot. Run,
    /// collaborative approval, Automation, and tuning all prepare through an
    /// owning workflow before they request dispatch; the controller never
    /// manufactures hidden authorization at this final boundary.
    ///
    /// A run set or a manual deck dispatches the very snapshot its
    /// preparation authorized, once nothing it was prepared from has moved;
    /// nothing is prepared here.
    pub fn consume_snapshot_for_dispatch(
        &mut self,
        state: &impl RunHost,
    ) -> Result<AuthorizedRunDispatch, PreparationError> {
        let intent = state.model().simulation.execution.run_intent;
        let inputs = self.retained_run_inputs.take();
        self.run_authorization.consume(intent, |snapshot| {
            inputs
                .ok_or_else(super::run_inputs::unaccounted_run)?
                .still_current(state, snapshot)
        })
    }

    pub fn build_prepared_snapshot(
        state: &impl RunHost,
        intent: SimulationRunIntent,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        match intent {
            SimulationRunIntent::SimulateRunSet => Self::build_prepared_run_set(state),
            SimulationRunIntent::ManualDeck => Self::build_prepared_manual_deck(state),
        }
    }

    fn build_prepared_run_set(
        state: &impl RunHost,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        Self::build_prepared_run_set_reading(state, &crate::simulation::table_route::SourceFiles)
    }

    /// [`Self::build_prepared_run_set`], with the data files read through
    /// `files`.
    fn build_prepared_run_set_reading(
        state: &impl RunHost,
        files: &dyn rspice_simulation::netlist_gen::NetlistSourceHost,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        #[cfg(any(test, feature = "test-support"))]
        thread_preparations::record(SimulationRunIntent::SimulateRunSet);
        let model = state.model();
        let execution_projection = model
            .workspace
            .configuration_execution_projection(
                &model.library_manager,
                &model.workspace.content.active_view,
                &model.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
        crate::simulation::execution::preparation::build_prepared_run_set(
            &run_preparation_inputs_reading(state, files),
            execution_projection,
        )
    }

    fn build_prepared_manual_deck(
        state: &impl RunHost,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        #[cfg(any(test, feature = "test-support"))]
        thread_preparations::record(SimulationRunIntent::ManualDeck);
        crate::simulation::execution::preparation::build_prepared_manual_deck(
            &run_preparation_inputs(state),
            manual_source_inputs(state)?,
        )
    }
}

/// The deck a manual run is prepared from: the netlist document the session
/// shows, the file it came from, and what composing it reads.
pub fn manual_source_inputs(
    state: &impl RunHost,
) -> Result<crate::simulation::execution::preparation::ManualSourceInputs<'_>, PreparationError> {
    let shown = state.netlist_shown();
    let model = state.model();
    if shown.settled && shown.comparison {
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            "Generated comparison documents cannot be executed",
        ));
    }
    let owned_active = shown.owned_source
        || (!shown.settled
            && model.simulation.source.netlist_content.is_empty()
            && model.workspace.content.netlist_source.is_some());
    let source = if owned_active {
        model
            .workspace
            .content
            .netlist_source
            .as_deref()
            .unwrap_or(model.simulation.source.netlist_content.as_str())
    } else {
        model.simulation.source.netlist_content.as_str()
    };
    let origin = if owned_active {
        model.workspace.content.netlist_source_path.as_deref()
    } else {
        model.schematic.session.current_file.as_deref()
    };
    Ok(
        crate::simulation::execution::preparation::ManualSourceInputs {
            source,
            descriptor: model.workspace.content.netlist_descriptor.as_ref(),
            generated_artifact: model
                .workspace
                .content
                .netlist_document
                .as_ref()
                .and_then(|document| document.generated_artifact()),
            owned: owned_active,
            origin,
        },
    )
}

/// Stable cache identity for analysis-independent design inspection.
///
/// Presentation-only selection state is intentionally excluded. Every source
/// input capable of changing hierarchy, variables, PVT expression context,
/// model cards, or option materialization participates.
///
/// The catalogue contributes
/// [`ModelLibraryManager::design_inspection_catalog_key`], not its execution
/// digest. Both are content, so both are equally sound against a catalogue
/// replaced wholesale, but the execution digest serializes every library in the
/// corpus and this key is asked for on every frame the Bins & geometry page or
/// the analysis editor paints. See that method for what the key covers and why
/// the coverage is complete.
pub fn design_inspection_input_digest(model: &Model) -> crate::product::ContentDigest {
    let plan_identity = model.sim_setup.analysis_plan.as_ref().map_or_else(
        || "none".to_owned(),
        |plan| format!("{}:{}", plan.id(), plan.revision().get()),
    );
    let model_library_identity = model.model_library_manager.design_inspection_catalog_key();
    let material = format!(
        "{}\0{}\0{}\0{}\0{}\0{:?}\0{}\0{}\0{}",
        model.design_execution_epoch,
        model.workspace.content.project.revision().get(),
        model.workspace.content.simulation_root_reference().key(),
        model.workspace.content.active_view.key(),
        plan_identity,
        model.sim_setup.reference_pvt.process,
        model.sim_setup.reference_pvt.temperature_celsius,
        model.sim_setup.options.to_spice_options(),
        model_library_identity,
    );
    content_digest(
        "rspice.analysis-independent-design-inspection/v2",
        material.as_bytes(),
    )
}

#[cfg(test)]
fn touchstone_export_policy<'a>(
    model: &Model,
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    crate::simulation::execution::preparation::touchstone_export_policy(
        &model.sim_setup.sp,
        model.schematic.document(),
        tasks,
        source_path,
    )
}

#[cfg(test)]
fn reject_deferred_external_sources(netlist: &str) -> Result<(), PreparationError> {
    reject_deferred_external_sources_with_project_runtimes(
        netlist,
        &Default::default(),
        &Default::default(),
    )?;
    validated_executable_hierarchy(netlist).map(|_| ())
}

#[cfg(any(test, feature = "test-support"))]
impl SimulationController {
    /// Retain a snapshot a test prepared from `state` as a preparation does:
    /// with the identity of `state`'s inputs, which its dispatch checks. The
    /// test prepared it with the plain data-file host, so no data-file answer
    /// is held for dispatch to ask again.
    pub fn retain_prepared_from(
        &mut self,
        state: &impl RunHost,
        snapshot: PreparedRunSnapshot,
    ) -> Result<(), PreparationError> {
        let identity = match snapshot.intent() {
            SimulationRunIntent::SimulateRunSet => {
                super::run_inputs::RunInputIdentity::of_run_set(state)
            }
            SimulationRunIntent::ManualDeck => {
                super::run_inputs::RunInputIdentity::of_manual_deck(state, &snapshot)?
            }
        };
        self.retain_prepared_run(
            snapshot,
            super::run_inputs::RetainedRunInputs {
                identity,
                source_files: Default::default(),
            },
        )
    }
}

/// Test-only record of every preparation run on the calling thread, so a test
/// can state that a frame prepared nothing.
#[cfg(any(test, feature = "test-support"))]
pub mod thread_preparations {
    use std::cell::RefCell;

    use crate::simulation::execution::SimulationRunIntent;

    thread_local! {
        static PREPARED: RefCell<Vec<SimulationRunIntent>> = const { RefCell::new(Vec::new()) };
    }

    pub(super) fn record(intent: SimulationRunIntent) {
        PREPARED.with(|prepared| prepared.borrow_mut().push(intent));
    }

    /// The preparations this thread ran since the last call, which forgets
    /// them.
    pub fn take() -> Vec<SimulationRunIntent> {
        PREPARED.with(|prepared| std::mem::take(&mut *prepared.borrow_mut()))
    }
}

#[cfg(test)]
mod binned_family_tests;
#[cfg(test)]
mod dispatch_parity_tests;
#[cfg(test)]
mod recorded_fft_tests;
// Visible to the crate rather than private, because `runnable_state` is the
// one fixture in this layer that produces a preparable project: the
// projection ratchet prepares one on the run test host, and the shell's
// feature tests build their whole sessions from it.
#[cfg(test)]
pub(crate) mod tests;
