//! Preparing a run.
//!
//! Resolves everything a run needs before it starts — the deck, its
//! includes, the sealed model set, and the export policy — so the run either
//! begins fully determined or is refused with a reason.

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
    /// Build the analysis-independent executable design deck used by
    /// inspection surfaces.
    ///
    /// This is not a preview database. It materializes the configured
    /// hierarchy, plan-owned design variables, reference-process model
    /// sources, simulation options, and include closure through the same
    /// binding helpers used by prepared execution. Consumers can therefore
    /// ask the engine to inspect the exact current design without inventing a
    /// second model-resolution path.
    pub(crate) fn prepare_design_netlist_for_inspection(
        state: &AppState,
    ) -> Result<String, PreparationError> {
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
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
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
            &state.library_manager,
            &projection,
        );
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = state.workspace.active_plan_data(plan.id()).ok_or_else(|| {
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
            crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
                root_schematic,
                &[],
                &hierarchy,
                &payload.design_variables,
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

        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state.model_library_manager.seal_execution_sources()
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = if has_project_technology {
            sealed_models
                .reference_process_model_cards(state.sim_setup.reference_pvt.process)
                .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
        } else {
            Vec::new()
        };
        let generated_source = state
            .workspace
            .bind_generated_netlist_provenance(generated.netlist);
        let mut source =
            Self::apply_reference_model_bindings_to_netlist(&generated_source, &model_cards);
        let pdk_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?;
        for runtime in pdk_veriloga_runtimes.iter() {
            crate::simulation::veriloga::append_project_veriloga_directive(
                &mut source,
                runtime.source_key(),
                runtime.netlist_alias(),
            );
        }
        let source = Self::apply_simulation_options_to_netlist(&source, &state.sim_setup.options);
        let (source, _) = expand_generated_dependencies_with_sealed_sources(
            &source,
            root_schematic.current_file.as_deref(),
            Some(&sealed_models),
        )?;
        Ok(source)
    }

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
        // A project need not have a technology; if it has one it must be
        // valid; if the plan needs one it must have one. A project that owes
        // nothing to a technology seals the plain model library instead.
        state
            .technology_gate_block_reason()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state.model_library_manager.seal_execution_sources()
        }
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
        let pdk_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?;
        let project_veriloga_runtimes = project_veriloga_runtimes
            .try_extend(pdk_veriloga_runtimes.iter().cloned())
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
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
        for runtime in pdk_veriloga_runtimes.iter() {
            crate::simulation::veriloga::append_project_veriloga_directive(
                &mut netlist,
                runtime.source_key(),
                runtime.netlist_alias(),
            );
        }
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
        reject_unresolved_device_models(&netlist, has_project_technology)?;
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
        append_signed_pdk_model_identity(&sealed_models, &mut model_identities);
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
            crate::workbench::workflows::netlist_workflow::compose_owned_netlist_execution_source(
                state, source,
            )
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?
        } else {
            source.to_owned()
        };
        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state.model_library_manager.seal_execution_sources()
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = if has_project_technology {
            sealed_models
                .reference_process_model_cards(state.sim_setup.reference_pvt.process)
                .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
        } else {
            Vec::new()
        };
        let composed = manual_deck::compose_manual_deck_source(&owned_materialized);
        let mut composed = Self::apply_reference_model_bindings_to_netlist(&composed, &model_cards);
        let pdk_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?;
        for runtime in pdk_veriloga_runtimes.iter() {
            crate::simulation::veriloga::append_project_veriloga_directive(
                &mut composed,
                runtime.source_key(),
                runtime.netlist_alias(),
            );
        }
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
            expand_manual_dependencies(&composed, origin, &sealed_models)?;
        reject_unresolved_device_models(&expanded, has_project_technology)?;
        let project_model_sources = prepared_project_model_sources(state, &expanded)?;
        let project_veriloga_runtimes = project_veriloga_runtimes_referenced_by(state, &expanded)?
            .try_extend(pdk_veriloga_runtimes.iter().cloned())
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
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
        append_signed_pdk_model_identity(&sealed_models, &mut model_identities);
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

/// Stable cache identity for analysis-independent design inspection.
///
/// Presentation-only selection state is intentionally excluded. Every source
/// input capable of changing hierarchy, variables, PVT expression context,
/// model cards, or option materialization participates.
pub(crate) fn design_inspection_input_digest(state: &AppState) -> crate::product::ContentDigest {
    let plan_identity = state.sim_setup.analysis_plan.as_ref().map_or_else(
        || "none".to_owned(),
        |plan| format!("{}:{}", plan.id(), plan.revision().get()),
    );
    let model_library_identity = state.model_library_manager.execution_catalog_digest();
    let material = format!(
        "{}\0{}\0{}\0{}\0{}\0{:?}\0{}\0{}\0{}",
        state.design_execution_epoch,
        state.workspace.project.revision().get(),
        state.workspace.simulation_root_reference().key(),
        state.workspace.active_view.key(),
        plan_identity,
        state.sim_setup.reference_pvt.process,
        state.sim_setup.reference_pvt.temperature_celsius,
        state.sim_setup.options.to_spice_options(),
        model_library_identity,
    );
    content_digest(
        "rspice.analysis-independent-design-inspection/v1",
        material.as_bytes(),
    )
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

/// Refuse a prepared source whose instantiated devices name models nothing
/// defines.
///
/// The builder rejects these at bind time, which surfaces them as an engine
/// failure after dispatch. Asking the same question here puts the answer in
/// preflight, next to the technology that would have supplied the missing
/// cards.
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
        return crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime])
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

fn prepared_signed_pdk_veriloga_runtimes(
    sealed_models: &crate::state::model_library::SealedModelExecutionSources,
) -> Result<crate::simulation::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some((package, archive_digest, artifacts, bindings)) =
        sealed_models.pdk_veriloga_authority()
    else {
        return Ok(Default::default());
    };
    let runtimes = bindings
        .iter()
        .map(|binding| {
            crate::simulation::veriloga::compile_signed_pdk_source_runtime(
                package,
                archive_digest,
                artifacts,
                binding,
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(runtimes)
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
    sealed_sources: &crate::state::model_library::SealedModelExecutionSources,
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
        let (expanded, dependencies) = sealed_sources
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
        let _ = sealed_sources;
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
        sealed_sources
            .expand_root_dependencies(&origin, source, &rspice_core::abort_signal::NoAbort)
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))
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

fn append_signed_pdk_model_identity(
    sealed_sources: &crate::state::model_library::SealedModelExecutionSources,
    identities: &mut Vec<ModelSourceIdentity>,
) {
    if let Some((label, archive_digest)) = sealed_sources.pdk_model_identity() {
        identities.push(ModelSourceIdentity::new(label, archive_digest));
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
mod tests;
