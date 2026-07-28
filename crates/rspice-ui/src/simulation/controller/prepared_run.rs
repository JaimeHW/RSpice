use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
use rspice_core::netlist::IncludeProcessor;
use rspice_core::netlist::{parse_include_directive, parse_lib_directive};

use super::*;
use crate::simulation::execution::{
    AuthorizedRunDispatch, CrossProbeSnapshot, ExecutionPermit, ExecutionTargetCapabilities,
    ModelSourceIdentity, PreparationError, PreparationStage, PreparedDependencyBinding,
    PreparedRunMetadata, PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy,
    SnapshotParts, TouchstoneExportPolicy, analysis_kind_tag, content_digest, drc_receipt_digest,
    manual_deck_analysis_instance_id, manual_source_receipt_digest,
};

pub(super) struct PendingPreparedRun {
    snapshot: PreparedRunSnapshot,
    permit: ExecutionPermit,
}

impl SimulationController {
    /// Validate the exact visible manual-deck document through the same
    /// dependency expansion, source checks, task construction, model binding,
    /// and execution-target contract used immediately before dispatch.
    ///
    /// The validated snapshot is retained behind a one-shot execution permit.
    /// Run rebuilds the complete source/dependency/target contract and must
    /// match this exact snapshot, so a dependency changed after validation
    /// cannot execute under a stale receipt.
    pub(crate) fn validate_manual_deck_document(
        &mut self,
        state: &AppState,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        self.clear_prepared_run();
        let snapshot = self.build_prepared_snapshot(state, SimulationRunIntent::ManualDeck)?;
        let metadata = snapshot.metadata();
        self.authorize_snapshot(snapshot)?;
        Ok(metadata)
    }

    /// Produce and retain the exact run-set tuple rendered by the mockup's
    /// preflight surface. The caller runs DRC immediately before this method.
    pub(crate) fn prepare_run_set_for_preflight(
        &mut self,
        state: &AppState,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        let snapshot = self.build_prepared_snapshot(state, SimulationRunIntent::SimulateRunSet)?;
        let metadata = snapshot.metadata();
        self.authorize_snapshot(snapshot)?;
        Ok(metadata)
    }

    /// Rebuild the complete live run-set contract and require it to match the
    /// exact snapshot authorized by a governed caller. Unlike the one-shot
    /// execution permit, this check remains usable while a run is executing,
    /// allowing evidence publication to reject any in-flight change to PVT,
    /// solver options, sources, model bindings, outputs, target capability, or
    /// analysis configuration.
    pub(crate) fn ensure_run_set_snapshot_current(
        &self,
        state: &AppState,
        expected_snapshot_digest: crate::product::ContentDigest,
        expected_source_digest: crate::product::ContentDigest,
    ) -> Result<(), PreparationError> {
        let current = self.build_prepared_snapshot(state, SimulationRunIntent::SimulateRunSet)?;
        let metadata = current.metadata();
        if metadata.snapshot_digest != expected_snapshot_digest
            || metadata.source_digest != expected_source_digest
        {
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                "The live simulation contract no longer matches the Automation dispatch snapshot",
            ));
        }
        Ok(())
    }

    pub(crate) fn clear_prepared_run(&mut self) {
        self.pending_prepared_run = None;
        if let Err(error) = self.execution_permits.invalidate() {
            log::error!("Failed to invalidate prepared execution permit: {error}");
        }
    }

    pub(crate) fn has_retained_manual_authorization(
        &self,
        expected_snapshot_digest: crate::product::ContentDigest,
    ) -> bool {
        self.pending_prepared_run.as_ref().is_some_and(|pending| {
            pending.snapshot.intent() == SimulationRunIntent::ManualDeck
                && pending.snapshot.digest() == expected_snapshot_digest
        })
    }

    /// Rebuild the complete manual-deck contract and require both the retained
    /// one-shot permit and the current source/dependency/environment snapshot
    /// to identify the validation receipt exactly. Used before publishing an
    /// owned source revision so Save cannot rely on stale source-only evidence.
    pub(crate) fn ensure_retained_manual_authorization_current(
        &self,
        state: &AppState,
        expected_snapshot_digest: crate::product::ContentDigest,
    ) -> Result<(), PreparationError> {
        if !self.has_retained_manual_authorization(expected_snapshot_digest) {
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                "The retained netlist validation authorization is no longer available",
            ));
        }
        let current = self.build_prepared_snapshot(state, SimulationRunIntent::ManualDeck)?;
        if current.digest() != expected_snapshot_digest {
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                "A source dependency, PVT setting, execution capability, or project input changed after validation",
            ));
        }
        Ok(())
    }

    /// Resolve a fresh internal preflight when Run was invoked directly, or
    /// validate an explicitly retained preflight snapshot. Only the retained
    /// immutable snapshot is returned for execution.
    pub(super) fn consume_snapshot_for_dispatch(
        &mut self,
        state: &mut AppState,
    ) -> Result<AuthorizedRunDispatch, PreparationError> {
        let intent = state.simulation.run_intent;
        if self
            .pending_prepared_run
            .as_ref()
            .is_some_and(|pending| pending.snapshot.intent() != intent)
        {
            self.clear_prepared_run();
        }

        if self.pending_prepared_run.is_none() {
            if intent == SimulationRunIntent::ManualDeck {
                return Err(PreparationError::new(
                    PreparationStage::Authorization,
                    "Validate the exact current netlist before running; manual decks are never auto-authorized",
                ));
            }
            crate::workbench::menu_bar::run_design_rule_check(state);
            let snapshot = self.build_prepared_snapshot(state, intent)?;
            self.authorize_snapshot(snapshot)?;
        }

        let pending = self.pending_prepared_run.take().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::Authorization,
                "No authorized prepared run is available",
            )
        })?;

        let current = match self.build_prepared_snapshot(state, intent) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                let _ = self.execution_permits.invalidate();
                return Err(error);
            }
        };
        let retained_digest = pending.snapshot.digest();
        let current_digest = current.digest();
        let proof = match pending.permit.consume(retained_digest, current_digest) {
            Ok(proof) => proof,
            Err(error) => {
                let _ = self.execution_permits.invalidate();
                return Err(PreparationError::new(
                    PreparationStage::Authorization,
                    format!(
                        "Prepared run expired because a bound input, capability, or check receipt changed ({error})"
                    ),
                ));
            }
        };
        pending.snapshot.authorize_dispatch(proof)
    }

    fn authorize_snapshot(
        &mut self,
        snapshot: PreparedRunSnapshot,
    ) -> Result<(), PreparationError> {
        let permit = self
            .execution_permits
            .issue(snapshot.digest())
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::Authorization,
                    format!("Could not authorize prepared run: {error}"),
                )
            })?;
        self.pending_prepared_run = Some(PendingPreparedRun { snapshot, permit });
        Ok(())
    }

    fn build_prepared_snapshot(
        &self,
        state: &AppState,
        intent: SimulationRunIntent,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        match intent {
            SimulationRunIntent::SimulateRunSet => self.build_prepared_run_set(state),
            SimulationRunIntent::ManualDeck => self.build_prepared_manual_deck(state),
        }
    }

    fn build_prepared_run_set(
        &self,
        state: &AppState,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        let execution_projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
        let root_reference = execution_projection.root().clone();
        let root_schematic = execution_projection
            .root_schematic()
            .expect("a successful execution projection has a materialized root");
        if root_schematic.components.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::DesignChecks,
                format!(
                    "Add a component to configured simulation root '{}' before preparing a run",
                    root_reference.display_path()
                ),
            ));
        }
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
            &state.library_manager,
            &execution_projection,
        );
        let drc = crate::services::drc::run_drc_check_with_hierarchy_and_config(
            root_schematic,
            &hierarchy,
            crate::services::drc::DrcConfig {
                check_missing_ground: true,
                ..crate::services::drc::DrcConfig::default()
            },
        );
        if !drc.completed {
            return Err(PreparationError::new(
                PreparationStage::DesignChecks,
                "Schematic source checks did not complete",
            ));
        }
        if drc.has_errors() {
            let summary = drc.summary();
            return Err(PreparationError::new(
                PreparationStage::DesignChecks,
                format!(
                    "Fix schematic source-check errors before simulation ({} critical, {} error{})",
                    summary.critical,
                    summary.errors,
                    if summary.errors == 1 { "" } else { "s" }
                ),
            ));
        }

        let plan = self.build_analysis_plan(state).map_err(|errors| {
            PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
        })?;
        let plan_payload = state.workspace.active_plan_data(plan.plan_id()).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Simulation plan {} has no plan-owned variables, outputs, and specifications payload",
                    plan.plan_id()
                ),
            )
        })?;
        state
            .model_library_manager
            .validate_attached_technology(state.workspace.project.technology_binding())
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let sealed_models = state
            .model_library_manager
            .seal_execution_sources()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let tasks = self
            .build_queue_from_plan(state, &plan, &sealed_models)
            .map_err(|errors| {
                PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
            })?;
        let tasks = attach_saved_output_contracts(tasks, &plan_payload.saved_outputs)?;
        if tasks.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "No runnable analyses were selected",
            ));
        }
        reject_deferred_corner_model_sources(tasks.iter().map(PreparedTask::queued_analysis))?;

        let analysis_lines = tasks
            .iter()
            .map(|task| task.queued_analysis().analysis_line.clone())
            .collect::<Vec<_>>();
        let analysis_instances = plan
            .instances()
            .iter()
            .map(crate::simulation::plan::FrozenAnalysisInstance::id)
            .collect::<Vec<_>>();
        let project_veriloga_runtimes =
            prepared_configuration_veriloga_runtimes(state, &execution_projection)?;
        let generated =
            crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
                root_schematic,
                &analysis_lines,
                &hierarchy,
                &plan_payload.design_variables,
                crate::simulation::netlist_gen::DesignVariableNetlistContext {
                    active_cell: &root_reference,
                    analysis_instances: &analysis_instances,
                },
            );
        if !generated.errors.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::Netlist,
                generated.errors.join("; "),
            ));
        }

        let model_cards = sealed_models
            .reference_process_model_cards(state.sim_setup.reference_pvt.process)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let generated_source = state
            .workspace
            .bind_generated_netlist_provenance(generated.netlist);
        let mut netlist =
            Self::apply_reference_model_bindings_to_netlist(&generated_source, &model_cards);
        netlist = Self::apply_simulation_options_to_netlist(&netlist, &state.sim_setup.options);
        let (expanded_netlist, sealed_source_dependencies) =
            expand_generated_dependencies_with_sealed_sources(
                &netlist,
                root_schematic.current_file.as_deref(),
                Some(&sealed_models),
            )?;
        netlist = expanded_netlist;
        reject_deferred_external_sources_with_project_runtimes(
            &netlist,
            &project_veriloga_runtimes,
        )?;
        validate_prepared_periodic_sources(&tasks, &netlist)?;
        let project_model_sources = prepared_project_model_sources(state, &netlist)?;

        let source_digest =
            content_digest("rspice.generated-executable-source/v1", netlist.as_bytes());
        let receipt = RunSourceReceipt::SchematicDrc(drc_receipt_digest(
            root_schematic.topology_version(),
            &drc,
        ));
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
        append_corner_model_identities(
            tasks.iter().map(PreparedTask::queued_analysis),
            &mut model_identities,
        );

        let mut advisories = generated.warnings;
        advisories.extend(
            drc.warnings().into_iter().map(|violation| {
                format!("{} · {}", violation.message, violation.location.display())
            }),
        );
        let touchstone_export = touchstone_export_policy(
            state,
            tasks.iter().map(PreparedTask::queued_analysis),
            root_schematic.current_file.as_deref(),
        )?;

        PreparedRunSnapshot::new(SnapshotParts {
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(plan.plan_id()),
            project_revision: state.workspace.project.revision().get(),
            topology_revision: root_schematic.topology_version(),
            source_digest,
            reference_process: state.sim_setup.reference_pvt.process,
            reference_temperature_celsius: state.sim_setup.reference_pvt.temperature_celsius,
            tasks,
            executable_netlist: netlist,
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities,
            project_model_sources,
            project_veriloga_runtimes,
            target: ExecutionTargetCapabilities::current(),
            receipt,
            advisories,
            manual_source: None,
            cross_probe: Some(CrossProbeSnapshot::new(
                root_reference,
                generated.point_to_net,
                generated.nets,
                generated.net_segments,
                root_schematic.topology_version(),
            )),
            touchstone_export,
            sealed_source_dependencies,
        })
    }

    fn build_prepared_manual_deck(
        &self,
        state: &AppState,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        if state.ui.netlist.active_document_initialized
            && state.ui.netlist.active_document
                == crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                "Generated comparison documents cannot be executed",
            ));
        }
        let owned_active = state.ui.netlist.active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
            || (!state.ui.netlist.active_document_initialized
                && state.simulation.netlist_content.is_empty()
                && state.workspace.netlist_source.is_some());
        let source = if owned_active {
            state
                .workspace
                .netlist_source
                .as_deref()
                .unwrap_or(state.simulation.netlist_content.as_str())
        } else {
            state.simulation.netlist_content.as_str()
        };
        if source.trim().is_empty() {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                "Enter a netlist before running",
            ));
        }

        let owned_materialized = if owned_active {
            crate::workbench::netlist_workflow::compose_owned_netlist_execution_source(state, source)
                .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?
        } else {
            source.to_owned()
        };
        let composed = manual_deck::compose_manual_deck_source(&owned_materialized);
        let origin = if owned_active {
            state.workspace.netlist_source_path.as_deref()
        } else {
            state.schematic.current_file.as_deref()
        };
        if origin.is_none() && contains_external_include_directive(&composed) {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                "Relative .include/.inc/.lib sources require an imported deck origin before they can be sealed",
            ));
        }
        let (expanded, canonical_origin, sealed_source_dependencies) =
            expand_manual_dependencies(&composed, origin, &state.model_library_manager)?;
        let project_model_sources = prepared_project_model_sources(state, &expanded)?;
        let project_veriloga_runtimes = project_veriloga_runtimes_referenced_by(state, &expanded)?;
        reject_deferred_external_sources_with_project_runtimes(
            &expanded,
            &project_veriloga_runtimes,
        )?;
        let queued_tasks =
            manual_deck::build_manual_deck_queue(state, &expanded).map_err(|errors| {
                PreparationError::new(PreparationStage::SourceChecks, errors.join("; "))
            })?;
        let source_digest =
            content_digest("rspice.manual-executable-source/v1", expanded.as_bytes());
        let tasks = self.prepare_manual_tasks(
            source_digest,
            state.workspace.project.revision(),
            queued_tasks,
        )?;
        reject_deferred_corner_model_sources(tasks.iter().map(PreparedTask::queued_analysis))?;
        let analysis_config_digests = tasks
            .iter()
            .map(PreparedTask::config_digest)
            .collect::<Vec<_>>();
        let dependency_closure_digest =
            crate::simulation::execution::sealed_dependency_closure_digest(
                &sealed_source_dependencies,
            );
        let receipt_digest = manual_source_receipt_digest(
            source,
            &expanded,
            canonical_origin.as_deref(),
            dependency_closure_digest,
            &analysis_config_digests,
        );
        let mut model_identities = Vec::new();
        append_corner_model_identities(
            tasks.iter().map(PreparedTask::queued_analysis),
            &mut model_identities,
        );
        let touchstone_export = touchstone_export_policy(
            state,
            tasks.iter().map(PreparedTask::queued_analysis),
            origin,
        )?;

        PreparedRunSnapshot::new(SnapshotParts {
            intent: SimulationRunIntent::ManualDeck,
            simulation_plan_id: None,
            project_revision: state.workspace.project.revision().get(),
            topology_revision: state.schematic.topology_version(),
            source_digest,
            reference_process: state.sim_setup.reference_pvt.process,
            reference_temperature_celsius: state.sim_setup.reference_pvt.temperature_celsius,
            tasks,
            executable_netlist: expanded,
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities,
            project_model_sources,
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

    fn prepare_manual_tasks(
        &self,
        expanded_source_identity: crate::product::ContentDigest,
        source_revision: crate::product::ObjectRevision,
        tasks: Vec<QueuedAnalysis>,
    ) -> Result<Vec<PreparedTask>, PreparationError> {
        let mut kind_occurrences = std::collections::HashMap::<u8, usize>::new();
        let mut prepared = tasks
            .into_iter()
            .map(|task| {
                let occurrence = kind_occurrences
                    .entry(analysis_kind_tag(&task.spec))
                    .or_default();
                let current_occurrence = *occurrence;
                *occurrence += 1;
                let instance_id = manual_deck_analysis_instance_id(
                    expanded_source_identity,
                    &task.spec,
                    current_occurrence,
                );
                let label = self.analysis_name_for_spec(&task.spec);
                PreparedTask::new(instance_id, source_revision, Vec::new(), label, task)
            })
            .collect::<Vec<_>>();

        let fourier_count = prepared
            .iter()
            .filter(|task| matches!(&task.queued_analysis().spec, AnalysisSpec::Fourier { .. }))
            .count();
        if fourier_count == 0 {
            return Ok(prepared);
        }
        let transient_producers = prepared
            .iter()
            .filter(|task| matches!(&task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
            .map(|task| {
                (
                    task.instance_id(),
                    task.source_revision(),
                    task.config_digest(),
                )
            })
            .collect::<Vec<_>>();
        let [(producer_id, producer_revision, producer_config_digest)] =
            transient_producers.as_slice()
        else {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Manual-deck Fourier tasks require exactly one prepared Transient producer; found {}",
                    transient_producers.len()
                ),
            ));
        };
        for task in &mut prepared {
            if matches!(&task.queued_analysis().spec, AnalysisSpec::Fourier { .. }) {
                task.set_dependencies(vec![*producer_id]);
                task.set_dependency_bindings(vec![
                    PreparedDependencyBinding::transient_trajectory(
                        *producer_id,
                        *producer_revision,
                        *producer_config_digest,
                    ),
                ]);
            }
        }

        // Analysis directives are declarative, so source order cannot make a
        // .FOUR consumer precede its .TRAN producer. Preserve authored order
        // among every currently-ready task while applying the exact graph.
        let mut ordered = Vec::with_capacity(prepared.len());
        let mut completed = HashSet::with_capacity(prepared.len());
        while !prepared.is_empty() {
            let Some(ready_index) = prepared.iter().position(|task| {
                task.dependencies()
                    .iter()
                    .all(|dependency| completed.contains(dependency))
            }) else {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    "Manual-deck analysis dependencies contain a cycle",
                ));
            };
            let task = prepared.remove(ready_index);
            completed.insert(task.instance_id());
            ordered.push(task);
        }
        Ok(ordered)
    }
}

fn validate_prepared_periodic_sources(
    tasks: &[PreparedTask],
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    if !tasks
        .iter()
        .any(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Pss { .. }))
    {
        return Ok(());
    }

    let parsed = rspice_core::Netlist::parse(executable_netlist).map_err(|error| {
        PreparationError::new(
            PreparationStage::Netlist,
            format!("Could not authenticate periodic sources in the executable netlist: {error}"),
        )
    })?;
    let engine = rspice_core::Engine::new(rspice_core::SimulationConfig::default());

    for task in tasks {
        let AnalysisSpec::Pss {
            fundamental_freq,
            tone_sources,
            ..
        } = &task.queued_analysis().spec
        else {
            continue;
        };
        engine
            .validate_periodic_source_contract(&parsed, tone_sources, *fundamental_freq)
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "PSS instance {} does not match the prepared circuit: {error}",
                        task.instance_id()
                    ),
                )
            })?;
    }
    Ok(())
}

fn prepared_configuration_veriloga_runtimes(
    state: &AppState,
    projection: &crate::state::workspace::ConfigurationExecutionProjection,
) -> Result<crate::simulation::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some(plan) = projection.plan() else {
        return Ok(Default::default());
    };
    let mut prepared =
        HashMap::<String, crate::simulation::veriloga::PreparedVerilogARuntime>::new();
    for execution in plan.bindings() {
        let Some(binding) = execution.project_veriloga() else {
            continue;
        };
        let bundle = state
            .workspace
            .project_sources
            .get_bundle(binding.source_bundle_id())
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Configured Verilog-A source bundle {} at {} no longer exists",
                        binding.source_bundle_id(),
                        execution.instance_path()
                    ),
                )
            })?;
        if bundle.closure_digest() != binding.source_closure_digest() {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A source bundle {} changed after hierarchy resolution",
                    binding.source_bundle_id()
                ),
            ));
        }
        let runtime = crate::simulation::veriloga::compile_project_source_bundle_runtime(
            state.workspace.project.id(),
            bundle,
            binding.selected_module(),
        )
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        if runtime.source_key() != binding.source_key()
            || !runtime
                .netlist_alias()
                .eq_ignore_ascii_case(binding.netlist_alias())
        {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A binding at {} changed while compiling its sealed source",
                    execution.instance_path()
                ),
            ));
        }
        let materialized = execution.materialized_binding().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Configured Verilog-A binding at {} has no materialized interface",
                    execution.instance_path()
                ),
            )
        })?;
        let compiled_terminals = runtime
            .terminal_names()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        if compiled_terminals.len() != materialized.terminal_order.len()
            || !compiled_terminals
                .iter()
                .zip(&materialized.terminal_order)
                .all(|(compiled, declared)| compiled.eq_ignore_ascii_case(declared))
        {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Compiled Verilog-A module '{}' at {} does not match the exact declared terminal order [{}]",
                    binding.selected_module(),
                    execution.instance_path(),
                    materialized.terminal_order.join(", ")
                ),
            ));
        }
        if let Some(existing) = prepared.get(runtime.source_key()) {
            if existing != &runtime {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Configured Verilog-A source key '{}' resolves to conflicting artifacts",
                        runtime.source_key()
                    ),
                ));
            }
        } else {
            prepared.insert(runtime.source_key().to_owned(), runtime);
        }
    }
    crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(
        prepared.into_values().collect(),
    )
    .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

fn prepared_project_veriloga_runtimes(
    state: &AppState,
) -> Result<crate::simulation::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some(bundle) = state.workspace.project_sources.bundle_for_owner(
        &crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ),
    ) else {
        return Ok(Default::default());
    };
    let document = bundle.root();
    let retained = state.ui.code_workspace.veriloga.receipt.as_ref();
    if let Some(receipt) = retained
        && receipt.token.project_id == state.workspace.project.id()
        && receipt.token.bundle_id == bundle.id()
        && receipt.token.revision == bundle.revision().get()
        && receipt.token.closure_digest == bundle.closure_digest()
    {
        let runtime = crate::simulation::veriloga::PreparedVerilogARuntime::try_new(
            state.workspace.project.id(),
            bundle,
            &receipt.token,
            &receipt.module_name,
            &receipt.report,
            receipt.module_name.clone(),
        )
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        return crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![
            runtime,
        ])
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error));
    }
    if !document.validation_is_current() {
        return Err(PreparationError::new(
            PreparationStage::ModelBindings,
            format!(
                "Compile the exact current project Verilog-A source '{}' before preparing execution",
                document.file_name()
            ),
        ));
    }
    // Persisted validation authenticates only the exact source bytes. Rebuild
    // transient executable artifacts rather than trusting serialized code or
    // requiring a redundant manual compile after project/session restore.
    let receipt = crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
        state.workspace.project.id(),
        bundle,
        None,
    )
    .map_err(|diagnostics| {
        let detail = diagnostics
            .first()
            .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
            .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!(
                "Could not rebuild validated Verilog-A source '{}': {detail}",
                document.file_name()
            ),
        )
    })?;
    let runtime =
        crate::simulation::veriloga::PreparedVerilogARuntime::try_from_current_bundle_receipt(
            state.workspace.project.id(),
            bundle,
            &receipt,
        )
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime])
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
}

fn project_veriloga_runtimes_referenced_by(
    state: &AppState,
    source: &str,
) -> Result<crate::simulation::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some(bundle) = state.workspace.project_sources.bundle_for_owner(
        &crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ),
    ) else {
        return Ok(Default::default());
    };
    let key_prefix = format!(
        "__rspice_project__/{}/{}/{}/",
        state.workspace.project.id(),
        bundle.id(),
        bundle.closure_digest()
    );
    let key_suffix = format!("/{}", bundle.root().logical_path());
    let references_project_key = executable_logical_lines(source)
        .iter()
        .filter_map(|(_, line)| parse_veriloga_directive_identity(line))
        .any(|(path, _)| path.starts_with(&key_prefix) && path.ends_with(&key_suffix));
    if !references_project_key {
        return Ok(Default::default());
    }
    let runtimes = prepared_project_veriloga_runtimes(state)?;
    let exact_reference = runtimes.iter().any(|runtime| {
        executable_logical_lines(source).iter().any(|(_, line)| {
            project_veriloga_directive_matches_exact_identity(
                line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        })
    });
    if !exact_reference {
        return Ok(Default::default());
    }
    Ok(runtimes)
}

/// Parse the identity-bearing fields from the one project Verilog-A directive
/// shape emitted by RSpice. SPICE command and model identifiers are
/// case-insensitive, but the project virtual path is an authenticated key and
/// must remain byte-for-byte exact.
fn parse_veriloga_directive_identity(line: &str) -> Option<(&str, Option<&str>)> {
    let (command, remainder) = take_spice_token(line)?;
    if !command.eq_ignore_ascii_case(".veriloga") {
        return None;
    }
    let (path, remainder) = take_spice_token(remainder)?;
    let remainder = remainder.trim();
    if remainder.is_empty() {
        return Some((path, None));
    }
    let (model_name, trailing) = take_spice_token(remainder)?;
    trailing
        .trim()
        .is_empty()
        .then_some((path, Some(model_name)))
}

fn take_spice_token(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    let first = input.chars().next()?;
    if matches!(first, '\'' | '"') {
        let quoted = &input[first.len_utf8()..];
        let mut escaped = false;
        for (index, character) in quoted.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character == first {
                return Some((&quoted[..index], &quoted[index + character.len_utf8()..]));
            }
        }
        return None;
    }

    let end = input.find(char::is_whitespace).unwrap_or(input.len());
    (end > 0).then_some((&input[..end], &input[end..]))
}

fn project_veriloga_directive_matches_exact_identity(
    line: &str,
    source_key: &str,
    module_name: &str,
) -> bool {
    parse_veriloga_directive_identity(line).is_some_and(|(path, model_name)| {
        path == source_key
            && model_name.is_some_and(|model| model.eq_ignore_ascii_case(module_name))
    })
}

fn attach_saved_output_contracts(
    tasks: Vec<PreparedTask>,
    outputs: &[crate::state::SavedOutput],
) -> Result<Vec<PreparedTask>, PreparationError> {
    if outputs.is_empty() {
        return Ok(tasks);
    }
    let analyses = tasks
        .iter()
        .map(|task| (task.instance_id(), &task.queued_analysis().spec))
        .collect::<Vec<_>>();
    let mut by_analysis = HashMap::with_capacity(tasks.len());
    for output in outputs {
        let contracts = crate::simulation::output_contract::compile_saved_output_contracts(
            output,
            analyses.iter().copied(),
        )
        .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        for contract in contracts {
            by_analysis
                .entry(contract.analysis_id())
                .or_insert_with(Vec::new)
                .push(contract);
        }
    }
    Ok(tasks
        .into_iter()
        .map(|task| {
            let contracts = by_analysis.remove(&task.instance_id()).unwrap_or_default();
            task.with_saved_output_contracts(contracts)
        })
        .collect())
}

fn expand_manual_dependencies(
    source: &str,
    origin: Option<&Path>,
    model_libraries: &crate::state::ModelLibraryManager,
) -> Result<
    (
        String,
        Option<String>,
        Vec<rspice_core::netlist::ResolvedIncludeDependency>,
    ),
    PreparationError,
> {
    let Some(origin) = origin else {
        return Ok((source.to_owned(), None, Vec::new()));
    };
    let absolute_origin = absolute_source_identity(origin)?;

    #[cfg(target_arch = "wasm32")]
    {
        let sealed = model_libraries
            .seal_execution_sources()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let (expanded, dependencies) = sealed
            .expand_root_dependencies(
                &absolute_origin,
                source,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
        Ok((
            expanded,
            Some(path_identity(&absolute_origin)),
            dependencies,
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = model_libraries;
        let mut processor = IncludeProcessor::new(&absolute_origin);
        let expanded = processor
            .expand_content(source, &absolute_origin)
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!("Could not seal manual deck dependencies: {error}"),
                )
            })?;
        Ok((
            expanded,
            Some(path_identity(&absolute_origin)),
            processor.resolved_dependencies().to_vec(),
        ))
    }
}

pub(crate) fn expand_generated_dependencies(
    source: &str,
    origin: Option<&Path>,
    model_libraries: &crate::state::ModelLibraryManager,
) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), PreparationError> {
    #[cfg(target_arch = "wasm32")]
    let sealed = model_libraries
        .seal_execution_sources()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    #[cfg(not(target_arch = "wasm32"))]
    let _ = model_libraries;

    expand_generated_dependencies_with_sealed_sources(source, origin, {
        #[cfg(target_arch = "wasm32")]
        {
            Some(&sealed)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    })
}

fn expand_generated_dependencies_with_sealed_sources(
    source: &str,
    origin: Option<&Path>,
    sealed_sources: Option<&crate::state::model_library::SealedModelExecutionSources>,
) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), PreparationError> {
    if !contains_external_include_directive(source) {
        return Ok((source.to_owned(), Vec::new()));
    }

    #[cfg(target_arch = "wasm32")]
    {
        let origin = origin.ok_or_else(|| {
            PreparationError::new(
                PreparationStage::SourceChecks,
                "Configured external SPICE sources require an imported root identity before browser execution",
            )
        })?;
        let origin = absolute_source_identity(origin)?;
        let sealed_sources = sealed_sources.ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                "Configured external SPICE sources have no authenticated browser source bundle",
            )
        })?;
        return sealed_sources
            .expand_root_dependencies(&origin, source, &rspice_core::abort_signal::NoAbort)
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = sealed_sources;
        let owner = match origin {
            Some(path) => absolute_source_identity(path)?,
            None => execution_current_directory()?.join("__rspice_generated_source__.cir"),
        };
        let mut processor = IncludeProcessor::new(&owner);
        let expanded = processor.expand_content(source, &owner).map_err(|error| {
            PreparationError::new(
                PreparationStage::SourceChecks,
                format!("Could not seal configured source dependencies: {error}"),
            )
        })?;
        Ok((expanded, processor.resolved_dependencies().to_vec()))
    }
}

fn absolute_source_identity(path: &Path) -> Result<PathBuf, PreparationError> {
    #[cfg(target_arch = "wasm32")]
    if crate::state::model_library::is_portable_absolute_path(path) {
        return Ok(path.to_path_buf());
    }
    if path.is_absolute() {
        return Ok(path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
    }
    let current = std::env::current_dir().map_err(|error| {
        PreparationError::new(
            PreparationStage::SourceChecks,
            format!("Could not resolve manual deck origin: {error}"),
        )
    })?;
    let joined = current.join(path);
    Ok(joined.canonicalize().unwrap_or(joined))
}

fn path_identity(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn touchstone_export_policy<'a>(
    state: &AppState,
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    if !tasks
        .into_iter()
        .any(|task| matches!(&task.spec, AnalysisSpec::SParameter { .. }))
    {
        return Ok(TouchstoneExportPolicy::disabled());
    }

    touchstone_export_policy_for_dialog(&state.sim_setup.sp, source_path)
}

pub(super) fn touchstone_export_policy_for_dialog(
    dialog: &crate::simulation::dialog::SpDialogState,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    let mut dialog = dialog.clone();
    dialog.ensure_initialized();
    let config = dialog.to_config().map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Invalid Touchstone export settings: {error}"),
        )
    })?;
    if !config.touchstone_export {
        return Ok(TouchstoneExportPolicy::disabled());
    }

    let (directory, stem) = touchstone_output_prefix(source_path)?;
    TouchstoneExportPolicy::enabled(config.touchstone_version, directory, stem)
}

fn touchstone_output_prefix(
    source_path: Option<&Path>,
) -> Result<(PathBuf, OsString), PreparationError> {
    let current = execution_current_directory()?;
    let Some(source) = source_path else {
        return Ok((current, OsString::from("untitled")));
    };

    let absolute = if source.is_absolute() {
        source.to_path_buf()
    } else {
        current.join(source)
    };
    let directory = absolute
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| current.clone());
    let directory = directory.canonicalize().unwrap_or(directory);
    let stem = absolute
        .file_stem()
        .filter(|stem| !stem.is_empty())
        .map(OsString::from)
        .unwrap_or_else(|| OsString::from("untitled"));
    Ok((directory, stem))
}

fn execution_current_directory() -> Result<PathBuf, PreparationError> {
    match std::env::current_dir() {
        Ok(path) => Ok(path),
        #[cfg(target_arch = "wasm32")]
        Err(_) => Ok(PathBuf::from(".")),
        #[cfg(not(target_arch = "wasm32"))]
        Err(error) => Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Could not resolve the automatic export directory: {error}"),
        )),
    }
}

fn contains_external_include_directive(source: &str) -> bool {
    source
        .lines()
        .any(|line| parse_include_directive(line).is_some() || parse_lib_directive(line).is_some())
}

#[cfg(test)]
fn reject_deferred_external_sources(netlist: &str) -> Result<(), PreparationError> {
    reject_deferred_external_sources_with_project_runtimes(netlist, &Default::default())
}

fn reject_deferred_external_sources_with_project_runtimes(
    netlist: &str,
    project_runtimes: &crate::simulation::veriloga::PreparedVerilogARuntimeSet,
) -> Result<(), PreparationError> {
    for (line_number, logical_line) in executable_logical_lines(netlist) {
        if project_runtimes.iter().any(|runtime| {
            project_veriloga_directive_matches_exact_identity(
                &logical_line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        }) {
            continue;
        }
        if let Some(reason) = deferred_external_source_reason(&logical_line) {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                format!(
                    "Executable netlist contains an unsealed external dependency ({reason}) at line {}: {}",
                    line_number, logical_line
                ),
            ));
        }
    }
    Ok(())
}

/// Fold physical SPICE continuation records exactly as the core parser does
/// for executable lines: comment removal and trimming happen per physical
/// line, then a leading `+` appends to the preceding logical record. Auditing
/// the folded form prevents an external path or parameter name from being
/// split across continuation boundaries after authorization.
fn executable_logical_lines(source: &str) -> Vec<(usize, String)> {
    let mut logical_lines = Vec::new();
    let mut pending: Option<(usize, String)> = None;

    for (index, physical_line) in source.lines().enumerate() {
        let trimmed = executable_source_portion(physical_line).trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix('+') {
            let (_, logical) = pending.get_or_insert_with(|| (index + 1, String::new()));
            logical.push(' ');
            logical.push_str(rest);
            continue;
        }

        if let Some(previous) = pending.replace((index + 1, trimmed.to_owned())) {
            logical_lines.push(previous);
        }
    }

    if let Some(previous) = pending {
        logical_lines.push(previous);
    }
    logical_lines
}

fn reject_deferred_corner_model_sources<'a>(
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
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
    }
    Ok(())
}

fn deferred_external_source_reason(line: &str) -> Option<&'static str> {
    let line = executable_source_portion(line);
    if line.is_empty() {
        return None;
    }
    if parse_include_directive(line).is_some() || parse_lib_directive(line).is_some() {
        return Some("include/library directive");
    }
    let lower = line.to_ascii_lowercase();
    let directive = lower.split_whitespace().next().unwrap_or_default();
    if matches!(
        directive,
        ".spef_include" | ".veriloga" | ".va" | ".ahdl_include" | ".hdl" | ".verilog" | ".load"
    ) {
        return Some("external source directive");
    }

    let without_whitespace = lower
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    if ["file", "input_file", "state_file", "process_file"]
        .iter()
        .any(|name| contains_parameter_assignment(&lower, name))
    {
        return Some("file-backed element or code-model parameter");
    }
    let tokens = lower
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if lower.contains("pwl") && tokens.contains(&"file") {
        return Some("file-backed PWL source");
    }
    if matches!(directive, ".measure" | ".meas") && tokens.contains(&"file") {
        return Some("file-backed measurement reference");
    }
    // `simulation` is the d_cosim shared-library/provider selector and may be
    // supplied either on its model or as an instance override.
    if contains_parameter_assignment(&lower, "simulation") {
        return Some("external co-simulation runtime");
    }
    const FILE_LOOKUPS: [&str; 16] = [
        "table",
        "tablefile",
        "fasttable",
        "fasttablefile",
        "cubic",
        "cubicfile",
        "akima",
        "akimafile",
        "spline",
        "splinefile",
        "wodicka",
        "wodickafile",
        "bli",
        "blifile",
        "barycentric",
        "barycentricfile",
    ];
    if FILE_LOOKUPS.iter().any(|function| {
        [format!("{function}(\""), format!("{function}('")]
            .iter()
            .any(|needle| without_whitespace.contains(needle))
    }) {
        return Some("file-backed behavioral lookup");
    }
    None
}

fn executable_source_portion(line: &str) -> &str {
    let line = line.trim();
    if line.starts_with('*') {
        return "";
    }

    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;
    let mut previous = None;
    let mut characters = line.char_indices().peekable();

    while let Some((index, character)) = characters.next() {
        if escaped {
            escaped = false;
            previous = Some(character);
            continue;
        }

        match character {
            '\\' if in_single_quote || in_double_quote => escaped = true,
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ';' if !in_single_quote && !in_double_quote => {
                return line[..index].trim_end();
            }
            '$' if !in_single_quote && !in_double_quote => {
                if characters
                    .peek()
                    .is_none_or(|(_, next)| next.is_whitespace())
                {
                    return line[..index].trim_end();
                }
            }
            '/' if !in_single_quote && !in_double_quote => {
                if matches!(characters.peek(), Some((_, '/')))
                    && previous.is_none_or(char::is_whitespace)
                {
                    return line[..index].trim_end();
                }
            }
            _ => {}
        }
        previous = Some(character);
    }
    line
}

fn contains_parameter_assignment(line: &str, parameter: &str) -> bool {
    line.match_indices(parameter).any(|(index, _)| {
        let has_identifier_boundary = index == 0
            || line[..index]
                .chars()
                .next_back()
                .is_some_and(|character| !(character.is_ascii_alphanumeric() || character == '_'));
        has_identifier_boundary
            && line[index + parameter.len()..]
                .trim_start()
                .starts_with('=')
    })
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

fn prepared_project_model_sources(
    state: &AppState,
    executable_netlist: &str,
) -> Result<Vec<crate::state::PreparedModelSourceIdentity>, PreparationError> {
    let parsed = rspice_core::netlist::parse_netlist(executable_netlist).map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!("Executable source cannot authenticate project model use: {error}"),
        )
    })?;
    let flattened =
        rspice_core::netlist::flatten_netlist_with_models(&parsed).map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!("Executable hierarchy cannot authenticate project model use: {error}"),
            )
        })?;
    let referenced_names = flattened
        .elements
        .iter()
        .filter_map(element_model_name)
        .map(str::to_ascii_lowercase)
        .collect::<HashSet<_>>();
    let executable_models = parsed
        .models
        .iter()
        .chain(flattened.scoped_models.iter())
        .collect::<Vec<_>>();
    let identities = state
        .model_library_manager
        .project_model_definition_identities()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
        .into_iter()
        .filter(|(_, model_name, _, _)| referenced_names.contains(&model_name.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let canonical_models = canonical_project_model_definitions(state);
    let mut name_counts = HashMap::<String, usize>::new();
    for (_, model_name, _, _) in &identities {
        *name_counts
            .entry(model_name.to_ascii_lowercase())
            .or_default() += 1;
    }
    identities
        .into_iter()
        // A repeated semantic name cannot be traced back to one exact project
        // source from the executable SPICE instance. Omit every ambiguous
        // candidate so simulation remains available while correlation
        // evidence fails closed.
        .filter(|(_, model_name, _, _)| {
            name_counts.get(&model_name.to_ascii_lowercase()).copied() == Some(1)
        })
        .filter(|(source_id, model_name, _, _)| {
            let mut executable_matches = executable_models
                .iter()
                .copied()
                .filter(|model| model.name.eq_ignore_ascii_case(model_name));
            let Some(executable_model) = executable_matches.next() else {
                return false;
            };
            if executable_matches.next().is_some() {
                return false;
            }

            let mut canonical_matches =
                canonical_models
                    .iter()
                    .filter(|(candidate_source_id, candidate_name, _)| {
                        candidate_source_id == source_id
                            && candidate_name.eq_ignore_ascii_case(model_name)
                    });
            let Some((_, _, canonical_model)) = canonical_matches.next() else {
                return false;
            };
            canonical_matches.next().is_none()
                && model_definitions_match(canonical_model, executable_model)
        })
        .map(|(source_id, model_name, revision, content_digest)| {
            crate::state::PreparedModelSourceIdentity::new(
                source_id,
                model_name,
                revision,
                content_digest,
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
        })
        .collect()
}

fn canonical_project_model_definitions(
    state: &AppState,
) -> Vec<(
    crate::product::ModelSourceId,
    String,
    rspice_core::netlist::ModelDef,
)> {
    let mut models = Vec::new();
    for library in state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.source_authority.is_project_owned())
    {
        let crate::state::model_library::ModelSourceAuthority::ProjectOwned { source_id, .. } =
            library.source_authority
        else {
            continue;
        };
        for (model_name, model) in &library.models {
            let Some(metadata) = library.model_definition_metadata.get(model_name) else {
                continue;
            };
            let definition = crate::state::model_library::ProjectModelRevisionDefinition::new(
                crate::state::model_library::ProjectModelDefinition::from_device_model(model),
                metadata.clone(),
            );
            let Ok(source) = definition.qualification_model_source(None) else {
                continue;
            };
            let deck = format!("Authenticated project model\n{source}.end\n");
            let Ok(parsed) = rspice_core::netlist::parse_netlist(&deck) else {
                continue;
            };
            let mut matching = parsed
                .models
                .into_iter()
                .filter(|candidate| candidate.name.eq_ignore_ascii_case(model_name));
            let Some(canonical_model) = matching.next() else {
                continue;
            };
            if matching.next().is_none() {
                models.push((source_id, model_name.clone(), canonical_model));
            }
        }
    }
    models
}

fn model_definitions_match(
    canonical: &rspice_core::netlist::ModelDef,
    executable: &rspice_core::netlist::ModelDef,
) -> bool {
    canonical.name.eq_ignore_ascii_case(&executable.name)
        && canonical
            .model_type
            .eq_ignore_ascii_case(&executable.model_type)
        && named_model_fields_match(&canonical.params, &executable.params)
        && named_model_fields_match(&canonical.expr_params, &executable.expr_params)
        && named_model_fields_match(&canonical.string_params, &executable.string_params)
        && named_model_fields_match(
            &canonical.string_vector_params,
            &executable.string_vector_params,
        )
        && named_model_fields_match(
            &canonical.real_vector_params,
            &executable.real_vector_params,
        )
        && named_model_fields_match(
            &canonical.real_vector_expr_params,
            &executable.real_vector_expr_params,
        )
        && named_model_fields_match(
            &canonical.integer_vector_params,
            &executable.integer_vector_params,
        )
}

fn named_model_fields_match<T: PartialEq>(
    canonical: &[(String, T)],
    executable: &[(String, T)],
) -> bool {
    fn normalized<T>(fields: &[(String, T)]) -> Option<HashMap<String, &T>> {
        let mut normalized = HashMap::with_capacity(fields.len());
        for (name, value) in fields {
            if normalized
                .insert(name.to_ascii_lowercase(), value)
                .is_some()
            {
                return None;
            }
        }
        Some(normalized)
    }

    normalized(canonical)
        .zip(normalized(executable))
        .is_some_and(|(canonical, executable)| canonical == executable)
}

fn element_model_name(element: &rspice_core::netlist::Element) -> Option<&str> {
    use rspice_core::netlist::ElementKind as Kind;
    match &element.kind {
        Kind::Resistor { model, .. }
        | Kind::Capacitor { model, .. }
        | Kind::Inductor { model, .. }
        | Kind::TransmissionLine { model, .. } => model.as_deref(),
        Kind::JilesAthertonInductor { model, .. }
        | Kind::Diode { model, .. }
        | Kind::Bjt { model, .. }
        | Kind::Mosfet { model, .. }
        | Kind::Jfet { model, .. }
        | Kind::Mesfet { model, .. }
        | Kind::XyceMemristor { model, .. }
        | Kind::VSwitch { model, .. }
        | Kind::ISwitch { model, .. }
        | Kind::GenericSwitch { model, .. }
        | Kind::Xspice { model, .. } => Some(model),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::services::drc::DrcResult;
    static FIXTURE_NONCE: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn prepared_model_receipts_authenticate_exact_executable_model_definition() {
        let mut state = AppState::default();
        state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = prepared_model_receipt_definition();
        state
            .model_library_manager
            .create_project_model("models-a", &definition)
            .unwrap();
        let source = definition.canonical_source().unwrap();
        let used = prepared_project_model_sources(
            &state,
            &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
        )
        .unwrap();
        assert_eq!(used.len(), 1);
        assert_eq!(used[0].model_name(), "nch_receipt");
    }

    #[test]
    fn prepared_model_receipts_reject_modified_same_name_manual_card() {
        let mut state = AppState::default();
        state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = prepared_model_receipt_definition();
        state
            .model_library_manager
            .create_project_model("models-a", &definition)
            .unwrap();

        let modified = prepared_project_model_sources(
            &state,
            "receipt fixture\n.model nch_receipt NMOS (level=1 vth0=0.51)\nM1 d g 0 0 nch_receipt\n.op\n.end\n",
        )
        .unwrap();
        assert!(
            modified.is_empty(),
            "a same-name card with different executable parameters must not inherit project provenance"
        );
    }

    #[test]
    fn prepared_model_receipts_ignore_exact_but_unused_model() {
        let mut state = AppState::default();
        state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = prepared_model_receipt_definition();
        state
            .model_library_manager
            .create_project_model("models-a", &definition)
            .unwrap();
        let source = definition.canonical_source().unwrap();
        let unused = prepared_project_model_sources(
            &state,
            &format!("receipt fixture\n{source}V1 out 0 1\nR1 out 0 1k\n.op\n.end\n"),
        )
        .unwrap();
        assert!(unused.is_empty());
    }

    #[test]
    fn prepared_model_receipts_reject_duplicate_project_model_name() {
        let mut state = AppState::default();
        state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = prepared_model_receipt_definition();
        state
            .model_library_manager
            .create_project_model("models-a", &definition)
            .unwrap();
        state
            .model_library_manager
            .create_project_model("models-b", &definition)
            .unwrap();
        let source = definition.canonical_source().unwrap();
        let ambiguous = prepared_project_model_sources(
            &state,
            &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
        )
        .unwrap();
        assert!(
            ambiguous.is_empty(),
            "an executable model name shared by multiple project sources cannot authenticate either source"
        );
    }

    fn prepared_model_receipt_definition() -> crate::state::model_library::ProjectModelDefinition {
        crate::state::model_library::ProjectModelDefinition {
            name: "nch_receipt".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Prepared receipt fixture".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.48),
            ]),
            string_parameters: std::collections::BTreeMap::new(),
        }
    }

    fn fixture_dir(label: &str) -> PathBuf {
        let nonce = FIXTURE_NONCE.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "rspice-prepared-run-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create prepared-run fixture");
        directory
    }

    fn runnable_state() -> AppState {
        let mut state = AppState::default();
        crate::workbench::examples::load_example("Voltage Divider", &mut state.schematic);
        let mut drc = DrcResult::new();
        drc.completed = true;
        state.dialogs.drc_results = Some(drc);
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        state
    }

    fn prepared_pss_task(
        tone_sources: impl IntoIterator<Item = &'static str>,
        oscillator_mode: bool,
    ) -> PreparedTask {
        PreparedTask::new(
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            Vec::new(),
            "PSS",
            QueuedAnalysis {
                spec: AnalysisSpec::Pss {
                    method: PssMethod::Shooting,
                    fundamental_freq: 1.0e3,
                    tone_sources: tone_sources.into_iter().map(str::to_owned).collect(),
                    tstab_periods: 20,
                    points_per_period: 512,
                    tolerance: 1.0e-7,
                    oscillator_mode,
                    oscillator_node: oscillator_mode.then(|| "out".to_owned()),
                    num_harmonics: 20,
                },
                config: None,
                spec_options: SpecExecutionOptions::default(),
                analysis_line: ".pss 1k".to_owned(),
            },
        )
    }

    #[test]
    fn prepared_pss_authenticates_the_complete_executable_source_set() {
        let deck = "periodic sources\nVLO lo 0 SIN(0 1 1k)\nVCLK clk 0 PULSE(0 1 0 1u 1u 200u 500u)\nR1 lo 0 1k\nR2 clk 0 1k\n.end\n";
        validate_prepared_periodic_sources(&[prepared_pss_task(["vclk", "VLO"], false)], deck)
            .expect("the complete commensurate source set is accepted");

        let error = validate_prepared_periodic_sources(&[prepared_pss_task(["VLO"], false)], deck)
            .expect_err("an omitted periodic source fails preflight");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("omitted: VCLK"));
    }

    #[test]
    fn prepared_autonomous_pss_accepts_an_exact_empty_driven_source_set() {
        let deck = "autonomous oscillator\nR1 out 0 1k\nC1 out 0 1n\n.end\n";
        validate_prepared_periodic_sources(&[prepared_pss_task([], true)], deck)
            .expect("a source-free autonomous circuit has an exact empty source set");
    }

    fn edit_frozen_transient_stop(state: &mut AppState, stop: &str) {
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("test state owns a stable plan");
        let transient = plan
            .instances()
            .iter()
            .find(|instance| {
                instance.kind() == crate::simulation::plan::AnalysisKind::Transient
                    && instance.enabled()
            })
            .expect("enabled transient instance")
            .id();
        plan.edit(transient, |draft| {
            let crate::simulation::plan::AnalysisDraft::Transient(draft) = draft else {
                panic!("expected transient draft");
            };
            draft.stop = stop.to_owned();
        })
        .expect("transient edit commits");
    }

    #[test]
    fn prepared_snapshot_detects_analysis_mutation_without_revision_change() {
        let mut state = runnable_state();
        let controller = SimulationController::new();
        let prepared = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare first snapshot");
        let revision = state.workspace.project.revision().get();
        edit_frozen_transient_stop(&mut state, "2m");
        let changed = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare changed snapshot");
        assert_eq!(state.workspace.project.revision().get(), revision);
        assert_ne!(prepared.digest(), changed.digest());
    }

    #[test]
    fn prepared_snapshot_detects_non_topology_source_mutation() {
        let mut state = runnable_state();
        let controller = SimulationController::new();
        let prepared = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare first snapshot");
        let topology = state.schematic.topology_version();
        state.schematic.components[0].value = "2k".to_owned();
        let changed = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare changed snapshot");
        assert_eq!(state.schematic.topology_version(), topology);
        assert_ne!(prepared.digest(), changed.digest());
    }

    #[test]
    fn prepared_snapshot_authenticates_plan_owned_saved_outputs() {
        let mut state = runnable_state();
        let controller = SimulationController::new();
        let without_output = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare baseline snapshot");
        assert_eq!(without_output.metadata().saved_output_contract_count, 0);

        let plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let output = crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(1)",
            crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
            crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
            crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            crate::state::SavedOutputStreaming::StoreOnly,
        )
        .expect("valid output");
        state
            .workspace
            .add_saved_output(plan_id, output)
            .expect("plan owns output");

        let with_output = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("prepare output snapshot");
        assert_eq!(with_output.metadata().saved_output_contract_count, 1);
        assert_ne!(without_output.digest(), with_output.digest());
    }

    #[test]
    fn automatic_touchstone_export_policy_captures_live_dialog_and_path_once() {
        let mut state = AppState::default();
        state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
            &crate::simulation::dialog::SpConfig::default(),
        );
        state.schematic.current_file = Some(PathBuf::from("designs").join("amp.rsch"));
        let tasks = vec![QueuedAnalysis {
            spec: AnalysisSpec::SParameter {
                start_freq: 1.0e6,
                stop_freq: 1.0e9,
                points_per_unit: 20,
                sweep: FrequencySweep::Decade,
                z0: 50.0,
                ports: vec![SpPort {
                    node_pos: "in".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                }],
            },
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".sp dec 20 1Meg 1Gig".to_owned(),
        }];

        let prepared =
            touchstone_export_policy(&state, &tasks, state.schematic.current_file.as_deref())
                .expect("capture enabled policy");
        let prepared_path = prepared.output_path(7, 1, 2).expect("enabled export path");

        state.schematic.current_file = Some(PathBuf::from("redirect").join("changed.rsch"));
        let mut disabled = crate::simulation::dialog::SpConfig::default();
        disabled.touchstone_export = false;
        state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&disabled);
        let current =
            touchstone_export_policy(&state, &tasks, state.schematic.current_file.as_deref())
                .expect("capture disabled policy");

        assert!(current.output_path(7, 1, 2).is_none());
        assert!(prepared_path.ends_with("designs/amp_run0007_sp01.s2p"));
    }

    #[test]
    fn manual_touchstone_export_uses_imported_deck_origin_not_stale_schematic_path() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.schematic.current_file = Some(PathBuf::from("stale").join("schematic.rsch"));
        state.workspace.netlist_source_path =
            Some(PathBuf::from("imported").join("rf_fixture.cir"));
        state.workspace.netlist_source = Some(
            "deck\nV2 out 0 dc 0 ac 1 portnum 2 z0 75\nV1 in 0 dc 0 ac 1 portnum 1 z0 50\nR1 in out 100\n.sp lin 3 1Meg 3Meg\n.end\n"
                .to_owned(),
        );
        state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
            &crate::simulation::dialog::SpConfig::default(),
        );

        let mut controller = SimulationController::new();
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare imported RF deck");
        controller
            .authorize_snapshot(snapshot)
            .expect("authorize imported RF deck");
        let dispatch = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect("dispatch imported RF deck");
        let policy = dispatch
            .tasks()
            .next()
            .expect("manual S-parameter task")
            .touchstone_export_policy();

        let path = policy.output_path(4, 1, 2).expect("export is enabled");
        assert!(path.ends_with("imported/rf_fixture_run0004_sp01.s2p"));
        assert!(!path.to_string_lossy().contains("schematic"));
    }

    #[test]
    fn manual_fourier_is_topologically_bound_to_its_exact_transient_task() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source = Some(
            "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.tran 10u 5m\n.end\n"
                .to_owned(),
        );

        let mut controller = SimulationController::new();
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare manual Fourier dependency graph");
        controller
            .authorize_snapshot(snapshot)
            .expect("authorize manual Fourier dependency graph");
        let dispatch = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect("dispatch manual Fourier dependency graph");
        let tasks = dispatch.tasks().collect::<Vec<_>>();

        assert_eq!(tasks.len(), 2);
        assert!(matches!(tasks[0].spec(), AnalysisSpec::Transient { .. }));
        assert!(matches!(tasks[1].spec(), AnalysisSpec::Fourier { .. }));
        assert_eq!(tasks[1].dependencies(), &[tasks[0].instance_id()]);
    }

    #[test]
    fn dispatch_rejects_mutation_after_explicit_preflight() {
        let mut state = runnable_state();
        let mut controller = SimulationController::new();
        controller
            .prepare_run_set_for_preflight(&state)
            .expect("preflight");
        edit_frozen_transient_stop(&mut state, "3m");
        let error = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect_err("changed input must fail closed");
        assert_eq!(error.stage(), PreparationStage::Authorization);
        assert!(error.message().contains("expired"));
    }

    #[test]
    fn governed_snapshot_check_rejects_in_flight_pvt_change() {
        let mut state = runnable_state();
        let mut controller = SimulationController::new();
        let metadata = controller
            .prepare_run_set_for_preflight(&state)
            .expect("preflight");

        controller
            .ensure_run_set_snapshot_current(
                &state,
                metadata.snapshot_digest,
                metadata.source_digest,
            )
            .expect("unchanged contract remains current");

        state
            .sim_setup
            .set_reference_pvt(crate::simulation::dialog::corner::ProcessCorner::TT, 125.0)
            .expect("physical temperature");
        let error = controller
            .ensure_run_set_snapshot_current(
                &state,
                metadata.snapshot_digest,
                metadata.source_digest,
            )
            .expect_err("PVT mutation must invalidate evidence authority");
        assert_eq!(error.stage(), PreparationStage::Authorization);
        assert!(error.message().contains("no longer matches"));
    }

    #[test]
    fn manual_include_without_origin_fails_closed() {
        let mut state = AppState::default();
        state.workspace.netlist_source =
            Some("deck\n.include models.lib\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        let controller = SimulationController::new();
        let error = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect_err("unbound include must fail");
        assert_eq!(error.stage(), PreparationStage::SourceChecks);
        assert!(error.message().contains("origin"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn generated_model_section_is_expanded_and_retained_before_dispatch() {
        let directory = fixture_dir("configured-model-section");
        let model = directory.join("device.lib");
        fs::write(
            &model,
            ".lib tt\n.subckt owned in out\nRsrc in out 9k\n.ends owned\n.endl tt\n.lib ff\n.subckt owned in out\nRsrc in out 4k\n.ends owned\n.endl ff\n",
        )
        .expect("write model-section fixture");
        let source = format!(
            "configured deck\n.lib \"{}\" tt\nX1 in out owned\n.end\n",
            model.display()
        );

        let (expanded, dependencies) = expand_generated_dependencies(
            &source,
            Some(&directory.join("generated.cir")),
            &crate::state::ModelLibraryManager::default(),
        )
        .expect("configured dependency seals");

        assert!(expanded.contains("Rsrc in out 9k"));
        assert!(!expanded.contains("Rsrc in out 4k"));
        reject_deferred_external_sources(&expanded).expect("expanded deck has no deferred source");
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].selected_section(), Some("tt"));
        assert_eq!(
            dependencies[0].source(),
            fs::read_to_string(&model).unwrap()
        );

        fs::remove_dir_all(directory).expect("remove model-section fixture");
    }

    #[test]
    fn every_runtime_external_input_category_fails_closed() {
        let cases = [
            ".include model.lib",
            ".lib model.lib TT",
            ".spef_include parasitics.spef",
            ".VERILOGA compact_model.va",
            ".VA compact_model.va",
            ".ahdl_include compact_model.va",
            ".hdl compact_model.va",
            ".verilog compact_model.va",
            ".load prior.raw",
            "Vstim in 0 PWL FILE \"wave.csv\"",
            ".measure tran fit ERROR V(out) FILE reference.prn DEPVARCOL 2",
            ".model touchstone transfer (file = \"network.s2p\")",
            ".model source d_source (input_file = \"stimulus.txt\")",
            ".model state d_state (state_file = \"state.tbl\")",
            ".model process d_process (process_file = worker)",
            ".model cosim d_cosim (simulation = \"payload.dll\")",
            "Aco [in] [out] null cosim simulation = provider",
        ];

        for line in cases {
            let Err(error) = reject_deferred_external_sources(line) else {
                panic!("unsealed runtime input must be rejected: {line}");
            };
            assert_eq!(error.stage(), PreparationStage::SourceChecks, "{line}");
            assert!(
                error.message().contains("unsealed external dependency"),
                "{line}"
            );
        }
    }

    #[test]
    fn case_altered_project_veriloga_key_is_rejected_before_dispatch() {
        let mut state = AppState::default();
        state
            .workspace
            .replace_imported_project_source(
            crate::state::ProjectSourceLanguage::VerilogA,
                "model.va".to_owned(),
            "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                .to_owned(),
        )
            .expect("replace bootstrapped project Verilog-A source");
        let bundle = state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ))
            .expect("installed project source bundle");
        let receipt = crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
            state.workspace.project.id(),
            bundle,
            None,
        )
        .expect("compile project Verilog-A source");
        state.ui.code_workspace.veriloga.receipt = Some(receipt);

        let bundle = state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ))
            .expect("installed project source bundle");
        let source_key = crate::state::project_veriloga_bundle_source_key(
            state.workspace.project.id(),
            bundle,
            "owned",
        )
        .expect("derive exact project source key");
        let exact_directive =
            crate::simulation::veriloga::project_veriloga_directive(&source_key, "owned");
        let exact_runtimes = project_veriloga_runtimes_referenced_by(&state, &exact_directive)
            .expect("inspect exact project directive");
        assert_eq!(exact_runtimes.len(), 1);
        reject_deferred_external_sources_with_project_runtimes(&exact_directive, &exact_runtimes)
            .expect("exact project identity is permitted");

        let altered_key = source_key.replacen("__rspice_project__", "__RSPICE_PROJECT__", 1);
        let altered_directive =
            crate::simulation::veriloga::project_veriloga_directive(&altered_key, "owned");
        let altered_runtimes = project_veriloga_runtimes_referenced_by(&state, &altered_directive)
            .expect("inspect altered project directive");
        assert!(
            altered_runtimes.is_empty(),
            "case-altered virtual paths must not acquire the exact project runtime"
        );
        let error = reject_deferred_external_sources_with_project_runtimes(
            &altered_directive,
            &altered_runtimes,
        )
        .expect_err("case-altered project key must remain an external dependency");
        assert_eq!(error.stage(), PreparationStage::SourceChecks);
        assert!(error.message().contains("unsealed external dependency"));
    }

    #[test]
    fn project_veriloga_path_is_exact_while_spice_identifiers_ignore_case() {
        let source_key = "__rspice_project__/project/digest/Model.va";
        assert!(project_veriloga_directive_matches_exact_identity(
            ".VERILOGA \"__rspice_project__/project/digest/Model.va\" OWNED",
            source_key,
            "owned",
        ));
        assert!(!project_veriloga_directive_matches_exact_identity(
            ".VERILOGA \"__rspice_project__/project/digest/model.va\" OWNED",
            source_key,
            "owned",
        ));
    }

    #[test]
    fn configured_cell_view_compiles_the_exact_sealed_veriloga_bundle() {
        let mut state = AppState::default();
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");

        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            crate::state::ProjectSourceLanguage::VerilogA,
            "behavioral/gain.va",
            "`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [crate::state::ProjectSourceFile::try_new(
                "behavioral/gain_constants.va",
                "`define RSPICE_GAIN 1.0\n",
            )
            .expect("valid included source")],
            [crate::state::ProjectSourceDependency::try_new(
                "behavioral/gain.va",
                "behavioral/gain_constants.va",
            )
            .expect("valid dependency edge")],
        )
        .expect("valid sealed Verilog-A bundle");
        let expected_digest = bundle.closure_digest();
        state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach cell-view source");

        let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
        placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
        state
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), placed);
        state
            .workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: crate::state::CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create executable mixed-signal configuration");

        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .expect("resolve configured behavioral view");
        let runtimes = prepared_configuration_veriloga_runtimes(&state, &projection)
            .expect("compile exact configured source closure");

        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
            &state.library_manager,
            &projection,
        );
        let generated = crate::simulation::netlist_gen::generate_netlist_hierarchical(
            projection.root_schematic().expect("materialized root"),
            &[],
            &hierarchy,
        );
        assert!(generated.errors.is_empty(), "{:?}", generated.errors);

        assert_eq!(runtimes.len(), 1);
        let runtime = runtimes.iter().next().expect("prepared runtime");
        assert_eq!(runtime.source_digest(), expected_digest);
        assert_eq!(runtime.module_name(), "sealed_gain");
        assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
        assert!(runtime.source_key().starts_with("__rspice_project__/"));
        assert_eq!(
            generated
                .netlist
                .lines()
                .filter(|line| line.trim().eq_ignore_ascii_case(
                    &crate::simulation::veriloga::project_veriloga_directive(
                        runtime.source_key(),
                        runtime.netlist_alias(),
                    )
                ))
                .count(),
            1,
            "configured netlist must reference the exact prepared runtime once"
        );
        runtime
            .install()
            .expect("sealed configured runtime installs in the session cache");
    }

    #[test]
    fn continuation_folding_cannot_hide_external_dependencies() {
        for source in [
            "deck\nBlookup out 0 V=table\n+ (\"C:/curves/transfer.tbl\")\n.end\n",
            "deck\nVstim in 0 PWL(\n+ FILE = \"C:/stimulus/wave.csv\"\n+ )\n.end\n",
            "deck\n.model cosim d_cosim (simulation\n* comment between continued records\n+ = \"C:/plugins/payload.dll\")\n.end\n",
            "deck\n.model source d_source (input_file\n+ = 'C:/stimulus/input.txt')\n.end\n",
        ] {
            let error = reject_deferred_external_sources(source)
                .expect_err("continued external dependency must fail closed");
            assert_eq!(error.stage(), PreparationStage::SourceChecks, "{source}");
            assert!(
                error.message().contains("unsealed external dependency"),
                "{source}: {error}"
            );
        }
    }

    #[test]
    fn benign_continuations_remain_accepted() {
        for source in [
            "deck\nBinline out 0 V=table(\n+ V(in), 0, 0, 1, 1)\n.end\n",
            "deck\n.model diode D(\n+ IS=1e-12\n+ N=1.1)\n.end\n",
        ] {
            reject_deferred_external_sources(source).unwrap_or_else(|error| {
                panic!("benign continuation was rejected: {source}: {error}")
            });
        }
    }

    #[test]
    fn every_materialized_corner_binding_is_audited_before_dispatch() {
        use crate::services::simulation_runner::{
            CornerModelBinding, CornerProcess, CornerRunConfig,
        };

        let task = QueuedAnalysis {
            spec: AnalysisSpec::Corner,
            config: None,
            spec_options: SpecExecutionOptions {
                corner: Some(CornerRunConfig {
                    process_corners: vec![CornerProcess::FF],
                    model_bindings: vec![CornerModelBinding {
                        process: CornerProcess::FF,
                        source_label: "foundry.lib [FF]".to_owned(),
                        section: Some("FF".to_owned()),
                        materialized_model_cards:
                            ".model external d_source (input_file\n+ = \"C:/late/stimulus.txt\")"
                                .to_owned(),
                    }],
                    ..CornerRunConfig::default()
                }),
                ..SpecExecutionOptions::default()
            },
            analysis_line: ".corner".to_owned(),
        };

        let error = reject_deferred_corner_model_sources(&[task])
            .expect_err("non-reference corner source must be sealed");
        assert_eq!(error.stage(), PreparationStage::ModelBindings);
        assert!(error.message().contains("foundry.lib [FF]"));
        assert!(error.message().contains("unsealed external dependency"));
    }

    #[test]
    fn active_batch_reentry_preserves_prepared_authorization_and_batch_metadata() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        let mut controller = SimulationController::new();
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare replacement request");
        let prepared_digest = snapshot.digest();
        controller
            .authorize_snapshot(snapshot)
            .expect("authorize replacement request");

        let active_run_id = state.simulation.start_run().id;
        controller.current_run_id = Some(active_run_id);
        controller.current_spec = Some(AnalysisSpec::dc_op());
        controller.current_analysis_idx = 1;
        controller.total_analyses = 2;
        controller.cached_netlist = Some("existing sealed batch".to_owned());

        controller.start_authorized_snapshot(&mut state);

        assert_eq!(controller.current_run_id, Some(active_run_id));
        assert_eq!(controller.current_spec, Some(AnalysisSpec::dc_op()));
        assert_eq!(controller.current_analysis_idx, 1);
        assert_eq!(controller.total_analyses, 2);
        assert_eq!(
            controller.cached_netlist.as_deref(),
            Some("existing sealed batch")
        );
        assert_eq!(state.simulation.runs.len(), 1);
        assert_eq!(
            controller
                .pending_prepared_run
                .as_ref()
                .map(|pending| pending.snapshot.digest()),
            Some(prepared_digest)
        );
    }

    #[test]
    fn unpolled_completion_reentry_does_not_consume_or_replace_authorization() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        let mut controller = SimulationController::new();
        let snapshot = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare replacement request");
        let prepared_digest = snapshot.digest();
        controller
            .authorize_snapshot(snapshot)
            .expect("authorize replacement request");
        controller
            .runner
            .store_pending_result(Err(SimulationError::Aborted))
            .expect("seed finished, unpolled worker result");

        controller.start_authorized_snapshot(&mut state);

        assert!(state.simulation.runs.is_empty());
        assert_eq!(controller.total_analyses, 0);
        assert_eq!(
            controller
                .pending_prepared_run
                .as_ref()
                .map(|pending| pending.snapshot.digest()),
            Some(prepared_digest)
        );
        assert!(!controller.runner.can_accept_prepared_task());
    }

    #[test]
    fn every_behavioral_file_lookup_alias_fails_closed() {
        let functions = [
            "table",
            "tablefile",
            "fasttable",
            "fasttablefile",
            "cubic",
            "cubicfile",
            "akima",
            "akimafile",
            "spline",
            "splinefile",
            "wodicka",
            "wodickafile",
            "bli",
            "blifile",
            "barycentric",
            "barycentricfile",
        ];

        for function in functions {
            let line = format!("Blookup out 0 V={function}(\"curve.dat\")");
            assert_eq!(
                deferred_external_source_reason(&line),
                Some("file-backed behavioral lookup"),
                "{function}"
            );
        }
    }

    #[test]
    fn external_input_audit_ignores_comments_and_inline_data() {
        for line in [
            "* .include ignored.lib",
            "// .VERILOGA ignored.va",
            "R1 out 0 1k $ file=ignored.tbl",
            "R2 out 0 2k ; file=ignored.tbl",
            "R3 out 0 3k // file=ignored.tbl",
            ".data sweep_values",
            "+ 0 1 2 3",
            ".enddata",
            "Binline out 0 V=table(V(in), 0, 0, 1, 1)",
            ".param profile=1",
        ] {
            assert_eq!(deferred_external_source_reason(line), None, "{line}");
        }

        for line in [
            "Aco $G_DPWR [in] [out] null cosim simulation=provider",
            ".model source d_source (input_file='stimulus;production.txt')",
            ".model source d_source (input_file=\"stimulus\\\"production.txt\")",
        ] {
            assert!(
                deferred_external_source_reason(line).is_some(),
                "executable dependency must survive comment scanning: {line}"
            );
        }
    }

    #[test]
    fn direct_manual_run_cannot_bypass_internal_prepare_and_permit_consumption() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        let mut controller = SimulationController::new();

        controller.start_simulation(&mut state);

        assert!(controller.pending_prepared_run.is_none());
        assert_eq!(controller.total_analyses, 0);
        assert!(controller.cached_netlist.is_none());

        controller
            .validate_manual_deck_document(&state)
            .expect("explicit validation authorizes the exact manual deck");
        controller.start_simulation(&mut state);

        assert!(controller.pending_prepared_run.is_none());
        assert_eq!(controller.total_analyses, 1);
        assert!(
            controller
                .cached_netlist
                .as_deref()
                .is_some_and(|netlist| netlist.contains(".op"))
        );
        controller.abort();
    }

    #[test]
    fn included_source_mutation_after_prepare_is_rejected() {
        let directory = fixture_dir("include-mutation");
        let origin = directory.join("deck.cir");
        let include = directory.join("device.inc");
        fs::write(&include, "R1 out 0 1k\n").expect("write include");
        let source = "deck\n.include device.inc\nV1 out 0 1\n.op\n.end\n";
        fs::write(&origin, source).expect("write deck origin");

        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source = Some(source.to_owned());
        state.workspace.netlist_source_path = Some(origin);
        let mut controller = SimulationController::new();
        let metadata = controller
            .validate_manual_deck_document(&state)
            .expect("validate and retain first include closure");

        fs::write(&include, "R1 out 0 2k\n").expect("mutate include");
        let changed = controller
            .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
            .expect("prepare changed include closure")
            .metadata();
        assert_ne!(metadata.source_digest, changed.source_digest);
        let error = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect_err("changed include must invalidate authorization");
        assert_eq!(error.stage(), PreparationStage::Authorization);

        fs::remove_dir_all(directory).expect("remove include fixture");
    }

    #[test]
    fn dispatched_include_closure_never_reopens_mutated_source_files() {
        let directory = fixture_dir("dispatched-include-closure");
        let origin = directory.join("deck.cir");
        let include = directory.join("device.inc");
        fs::write(&include, "R1 out 0 1k\n").expect("write include");
        let source = "deck\n.include device.inc\nV1 out 0 1\n.op\n.end\n";
        fs::write(&origin, source).expect("write deck origin");

        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.workspace.netlist_source = Some(source.to_owned());
        state.workspace.netlist_source_path = Some(origin);
        let mut controller = SimulationController::new();
        controller
            .validate_manual_deck_document(&state)
            .expect("validate the exact include closure");
        let dispatch = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect("freeze the authorized dispatch");

        fs::write(&include, "R1 out 0 2k\n").expect("mutate source after dispatch");

        assert!(dispatch.executable_netlist().contains("R1 out 0 1k"));
        assert!(!dispatch.executable_netlist().contains("R1 out 0 2k"));
        assert!(!contains_external_include_directive(
            dispatch.executable_netlist()
        ));
        for task in dispatch.tasks() {
            assert_eq!(task.executable_netlist(), dispatch.executable_netlist());
        }

        fs::remove_dir_all(directory).expect("remove include fixture");
    }

    #[test]
    fn accepted_model_file_mutation_after_prepare_fails_closed() {
        let directory = fixture_dir("model-mutation");
        let model = directory.join("foundry.lib");
        fs::write(
            &model,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n",
        )
        .expect("write model");
        let mut state = runnable_state();
        state
            .model_library_manager
            .load_library_file(&model, None)
            .expect("load model library");
        let mut controller = SimulationController::new();
        controller
            .prepare_run_set_for_preflight(&state)
            .expect("prepare model-bound run");

        fs::write(
            &model,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
        )
        .expect("mutate model");
        let error = controller
            .consume_snapshot_for_dispatch(&mut state)
            .expect_err("changed accepted model bytes must block dispatch");
        assert_eq!(error.stage(), PreparationStage::ModelBindings);
        assert!(error.message().contains("changed"));

        fs::remove_dir_all(directory).expect("remove model fixture");
    }
}
