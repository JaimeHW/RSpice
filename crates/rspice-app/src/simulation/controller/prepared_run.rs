//! Preparing a run.
//!
//! Resolves everything a run needs before it starts — the deck, its
//! includes, the sealed model set, and the export policy — so the run either
//! begins fully determined or is refused with a reason.

use std::collections::HashSet;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

use super::*;
use crate::simulation::execution::AuthorizedRunDispatch;
use crate::simulation::execution::ExecutionTargetCapabilities;
use crate::simulation::execution::ModelSourceIdentity;
use crate::simulation::execution::PreparationError;
use crate::simulation::execution::PreparationStage;
use crate::simulation::execution::PreparedRunMetadata;
use crate::simulation::execution::PreparedRunSnapshot;
use crate::simulation::execution::PreparedTask;
use crate::simulation::execution::RunSourceReceipt;
use crate::simulation::execution::SavePolicy;
use crate::simulation::execution::SnapshotParts;
use crate::simulation::execution::TouchstoneExportPolicy;
use crate::simulation::execution::{attach_saved_output_contracts, prepare_manual_tasks};
use rspice_app_types::canonical::content_digest;
use rspice_simulation::execution_identity::drc_receipt_digest;
use rspice_simulation::execution_identity::manual_source_receipt_digest;
use rspice_simulation::netlist_gen::CrossProbeSnapshot;
use rspice_simulation::sealed_source::{
    generated_executable_source_digest, manual_executable_source_digest,
};

#[cfg(test)]
mod measurement_tests;

use rspice_simulation::capture_ledger::{plan_capture_workload, validate_plan_saved_output_budget};
use rspice_simulation::model_sources::prepared_project_model_sources;
use rspice_simulation::model_sources::validate_projected_model_binding_authority;
#[cfg(test)]
use rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies;
use rspice_simulation::netlist_preparation::dependencies::{
    expand_generated_dependencies_with_sealed_sources, expand_manual_dependencies,
};
use rspice_simulation::netlist_preparation::{
    contains_external_include_directive, deferred_external_source_reason, executable_logical_lines,
    reject_deferred_external_sources_with_project_runtimes, validated_executable_hierarchy,
};
use rspice_simulation::netlist_preparation::{measurements, owned_source};
use rspice_simulation::output_contract::selection::{
    effective_plan_capture, projection_occurrence_nets,
};
use rspice_simulation::preparation::validate_prepared_periodic_sources;
use rspice_simulation::project_veriloga::preparation::{
    prepared_configuration_veriloga_runtimes, prepared_model_library_veriloga_runtimes,
    prepared_signed_pdk_veriloga_runtimes, project_veriloga_runtimes_referenced_by,
};

/// Let the deck find the data files its sources name.
///
/// Project-relative references resolve against the project's folder; an
/// unsaved project has none, so its references stay as written and the netlist
/// generator reports the ones it cannot check. The stimulus library goes with
/// it, so a file-backed source whose named table is not reachable runs from the
/// copy its definition retains instead of being refused.
fn project_netlist_source_data(
    state: &AppState,
) -> rspice_simulation::netlist_gen::NetlistSourceData<'_> {
    rspice_simulation::netlist_gen::NetlistSourceData {
        files: &crate::simulation::table_route::SourceFiles,
        data_root: state.workspace.content.project.data_root(),
        stimulus_library: Some(&state.workspace.content.stimulus_library),
    }
}

fn activate_campaign_plan(
    state: &mut AppState,
    plan_id: crate::product::SimulationPlanId,
) -> Result<String, String> {
    let current_id = state.sim_setup.stable_analysis_plan()?.id();
    let plan_name = if current_id == plan_id {
        state.sim_setup.active_plan_name().to_string()
    } else {
        let stored = state
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
        state.workspace.content.migrate_active_plan_data(current_id);
        state.workspace.content.migrate_inactive_plan_data(plan_id);
        state
            .sim_setup
            .activate_plan(plan_id)
            .map_err(|error| error.to_string())?;
        state
            .workspace
            .content
            .sync_legacy_specs_projection(plan_id);
        state
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
    pub(crate) fn effective_saved_outputs_preflight(
        &self,
        state: &AppState,
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
        let selection_mode = state.sim_setup.save_policy.output_selection_mode;
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.content.active_view,
                &state.schematic,
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
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        // Net names carry no data-file references, so the projection's own
        // extraction is the same answer as a data-root-bound generator.
        let nets = rspice_design::connectivity::summary::projection_nets(
            state.library_manager.catalog(),
            &projection,
            &projection.root().key(),
        );
        let occurrences =
            projection_occurrence_nets(state.library_manager.catalog(), &projection, nets);
        let (outputs, automatic_fallback, membership) = effective_plan_capture(
            selection_mode,
            explicit,
            groups,
            &root_schematic.document().probes,
            &occurrences,
            plan.id(),
        )?;
        let reports = self.saved_outputs_preflight(state, &outputs);
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
    pub(crate) fn prepare_design_netlist_for_inspection(
        state: &AppState,
    ) -> Result<String, PreparationError> {
        let projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.content.active_view,
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
        validate_projected_model_binding_authority(
            state.model_library_manager.catalog(),
            state.model_library_manager.resolution_records(),
            state.library_manager.catalog(),
            state.workspace.content.project.technology_binding(),
            state.pdk_config.technology_registry.validated_packages(),
            &projection,
        )?;
        let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
            state.library_manager.catalog(),
            &projection,
        );
        let source_data = project_netlist_source_data(state);
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = state.workspace.content.plan_data(plan.id()).ok_or_else(|| {
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

        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state
                .model_library_manager
                .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = if has_project_technology {
            sealed_models
                .reference_model_execution_plan(state.sim_setup.reference_pvt.process)
                .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
                .model_cards()
        } else {
            Vec::new()
        };
        let generated_source = state
            .workspace
            .content
            .bind_generated_netlist_provenance(generated.netlist);
        let mut source =
            rspice_simulation::analysis_preparation::apply_reference_model_bindings_to_netlist(
                &generated_source,
                &model_cards,
            );
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_merge(prepared_model_library_veriloga_runtimes(&sealed_models)?)
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
            &state.sim_setup.options,
        );
        let (source, _) = expand_generated_dependencies_with_sealed_sources(
            &source,
            root_schematic.current_file(),
            &rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
                state.workspace.content.project.include_search_paths(),
                state.workspace.content.project.data_root(),
            ),
            Some(&sealed_models),
        )?;
        Ok(source)
    }

    /// Include explicitly authored measurements in the generated primary and
    /// its exports. Operand validation waits for preparation, where includes
    /// and model-owned parameters have been resolved into the execution deck.
    pub(crate) fn append_plan_measurements_to_generated_netlist(
        state: &AppState,
        source: &str,
    ) -> Result<String, PreparationError> {
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = state
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
    /// The validated snapshot is retained behind a one-shot execution permit.
    /// Run rebuilds the complete source/dependency/target contract and must
    /// match this exact snapshot, so a dependency changed after validation
    /// cannot execute under a stale receipt.
    pub(crate) fn validate_manual_deck_document(
        &mut self,
        state: &AppState,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        self.clear_prepared_run();
        let snapshot = Self::build_prepared_snapshot(state, SimulationRunIntent::ManualDeck)?;
        let metadata = snapshot.metadata();
        self.run_authorization.retain(snapshot)?;
        Ok(metadata)
    }

    /// Produce and retain the exact run-set tuple rendered by the mockup's
    /// preflight surface. The caller runs DRC immediately before this method.
    pub(crate) fn prepare_run_set_for_preflight(
        &mut self,
        state: &AppState,
    ) -> Result<PreparedRunMetadata, PreparationError> {
        let snapshot = Self::build_prepared_snapshot(state, SimulationRunIntent::SimulateRunSet)?;
        let metadata = snapshot.metadata();
        self.run_authorization.retain(snapshot)?;
        Ok(metadata)
    }

    /// Freeze and start a declared-order campaign of independent simulation
    /// plans. Every member is prepared in a cloned project view before any
    /// engine work begins, so a later member never observes edits made while
    /// an earlier member is executing.
    pub(crate) fn prepare_and_start_campaign(
        &mut self,
        state: &mut AppState,
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
            let plan_name = activate_campaign_plan(&mut frozen_state, plan_id)?;
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
        state.workbench.preflight.invalidate();
        self.design_execution_epoch = state.design_execution_epoch;
        self.active_campaign = Some(super::ActiveSimulationCampaign {
            id: campaign_id,
            name: name.to_owned(),
            member_count: member_count as u32,
            dispatched_count: 0,
            completed_count: 0,
            failed_count: 0,
            cancelled: false,
            pending: members,
        });
        state.push_sim_message(ConsoleMessage::info(format!(
            "Queued campaign '{name}' ({member_count} plans, {task_count} authenticated tasks)"
        )));
        self.dispatch_next_campaign_member(state)?;
        Ok(super::SimulationCampaignDispatchReceipt {
            campaign_id,
            member_count,
            task_count,
        })
    }

    pub(super) fn dispatch_next_campaign_member(
        &mut self,
        state: &mut AppState,
    ) -> Result<(), String> {
        loop {
            let Some(mut campaign) = self.active_campaign.take() else {
                return Ok(());
            };
            let Some(member) = campaign.pending.pop_front() else {
                let outcome = if campaign.cancelled {
                    "cancelled"
                } else if campaign.failed_count == 0 {
                    "completed"
                } else {
                    "completed with errors"
                };
                state.push_sim_message(ConsoleMessage::info(format!(
                    "Campaign '{}' {outcome}: {} completed, {} failed",
                    campaign.name, campaign.completed_count, campaign.failed_count
                )));
                state.simulation.status = format!("Campaign {outcome}");
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
                    campaign.completed_count = campaign.completed_count.saturating_add(1);
                    campaign.failed_count = campaign.failed_count.saturating_add(1);
                    self.active_campaign = Some(campaign);
                    state.push_sim_message(ConsoleMessage::error(format!(
                        "Campaign member '{}' has invalid membership metadata: {error}",
                        member.plan_name
                    )));
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
                    let campaign = self
                        .active_campaign
                        .as_mut()
                        .expect("campaign is reinstalled before authorization is handled");
                    campaign.completed_count = campaign.completed_count.saturating_add(1);
                    campaign.failed_count = campaign.failed_count.saturating_add(1);
                    state.push_sim_message(ConsoleMessage::error(format!(
                        "Campaign member '{member_name}' could not be authorized: {error}"
                    )));
                    continue;
                }
            };
            state.push_sim_message(ConsoleMessage::info(format!(
                "Dispatching campaign member {} of {}: '{member_name}'",
                membership.member_index(),
                membership.member_count()
            )));
            match self.start_authorized_dispatch(state, dispatch, Some(membership)) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    let campaign = self
                        .active_campaign
                        .as_mut()
                        .expect("campaign remains installed during member dispatch");
                    campaign.completed_count = campaign.completed_count.saturating_add(1);
                    campaign.failed_count = campaign.failed_count.saturating_add(1);
                    state.push_sim_message(ConsoleMessage::error(format!(
                        "Campaign member '{member_name}' could not start: {error}"
                    )));
                }
            }
        }
    }

    pub(super) fn complete_campaign_member(&mut self, state: &mut AppState, succeeded: bool) {
        let Some(campaign) = self.active_campaign.as_mut() else {
            return;
        };
        campaign.completed_count = campaign.completed_count.saturating_add(1);
        if !succeeded {
            campaign.failed_count = campaign.failed_count.saturating_add(1);
        }
        if let Err(error) = self.dispatch_next_campaign_member(state) {
            state.push_sim_message(ConsoleMessage::error(format!(
                "Campaign scheduling stopped safely: {error}"
            )));
            self.active_campaign = None;
            state.simulation.status = "Campaign stopped".to_owned();
        }
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
        let current = Self::build_prepared_snapshot(state, SimulationRunIntent::SimulateRunSet)?;
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
        self.run_authorization.clear();
    }

    pub(crate) fn has_retained_manual_authorization(
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
    pub(super) fn consume_snapshot_for_dispatch(
        &mut self,
        state: &AppState,
    ) -> Result<AuthorizedRunDispatch, PreparationError> {
        let intent = state.simulation.run_intent;
        self.run_authorization
            .consume(intent, || Self::build_prepared_snapshot(state, intent))
    }

    fn build_prepared_snapshot(
        state: &AppState,
        intent: SimulationRunIntent,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        match intent {
            SimulationRunIntent::SimulateRunSet => Self::build_prepared_run_set(state),
            SimulationRunIntent::ManualDeck => Self::build_prepared_manual_deck(state),
        }
    }

    fn build_prepared_run_set(state: &AppState) -> Result<PreparedRunSnapshot, PreparationError> {
        let execution_projection = state
            .workspace
            .configuration_execution_projection(
                &state.library_manager,
                &state.workspace.content.active_view,
                &state.schematic,
            )
            .map_err(|error| {
                PreparationError::new(PreparationStage::DesignChecks, error.to_string())
            })?;
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
            state.library_manager.catalog(),
            &execution_projection,
        );
        let source_data = project_netlist_source_data(state);
        let drc =
            rspice_simulation::preparation::check_generated_design(root_schematic, &hierarchy)?;
        validate_projected_model_binding_authority(
            state.model_library_manager.catalog(),
            state.model_library_manager.resolution_records(),
            state.library_manager.catalog(),
            state.workspace.content.project.technology_binding(),
            state.pdk_config.technology_registry.validated_packages(),
            &execution_projection,
        )?;

        let plan = Self::build_analysis_plan(&state.sim_setup).map_err(|errors| {
            PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
        })?;
        let plan_payload = state.workspace.content.plan_data(plan.plan_id()).ok_or_else(|| {
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
                .map(crate::state::PreparedSpecification::new)
                .collect::<Result<Vec<_>, _>>()
        } else {
            plan_payload
                .specification_definitions
                .iter()
                .cloned()
                .map(crate::state::PreparedSpecification::from_definition)
                .collect::<Result<Vec<_>, _>>()
        }
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Simulation-plan specification is invalid: {error}"),
            )
        })?;
        let specification_policy = crate::state::PreparedSpecificationPolicy::new(
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
        state
            .technology_gate_block_reason()
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state
                .model_library_manager
                .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let tasks =
            Self::build_queue_from_plan(state, &plan, &sealed_models).map_err(|errors| {
                PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
            })?;
        let design_nets = std::sync::Arc::new(
            rspice_design::connectivity::summary::design_nets_with_hierarchy(
                root_schematic,
                &hierarchy,
            ),
        );
        let occurrences = projection_occurrence_nets(
            state.library_manager.catalog(),
            &execution_projection,
            design_nets,
        );
        let (effective_saved_outputs, used_automatic_outputs, capture_membership) =
            effective_plan_capture(
                state.sim_setup.save_policy.output_selection_mode,
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
                &state.sim_setup.run_set,
                state.sim_setup.reference_pvt,
                tasks.iter().map(|task| (task.instance_id(), task.run_at())),
            ),
            &state.sim_setup.save_policy,
            crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
        )?;
        let tasks = attach_saved_output_contracts(tasks, &effective_saved_outputs)?;
        if tasks.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "No runnable analyses were selected",
            ));
        }

        let run_set_config = state
            .sim_setup
            .run_set
            .to_corner_config(
                crate::simulation::dialog::corner::CornerBaseAnalysis::Op,
                state.sim_setup.reference_pvt,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Run Set is invalid: {error}"),
                )
            })?;
        let run_set_contract =
            rspice_simulation::analysis_preparation::corner_run_config_from_dialog(
                &state.sim_setup,
                &run_set_config,
                &sealed_models,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Run Set model binding failed: {error}"),
                )
            })?;
        let prepared_run_set = crate::simulation::execution::PreparedRunSet::new(
            state.sim_setup.run_set.clone(),
            run_set_contract,
        );

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
            .map(crate::simulation::plan::FrozenAnalysisInstance::id)
            .collect::<Vec<_>>();
        let project_veriloga_runtimes = prepared_configuration_veriloga_runtimes(
            state.workspace.content.project.id(),
            &state.workspace.content.project_sources,
            &execution_projection,
        )?;
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_merge(prepared_model_library_veriloga_runtimes(&sealed_models)?)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let project_veriloga_runtimes = project_veriloga_runtimes
            .try_merge(external_veriloga_runtimes.clone())
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let generated =
            rspice_simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
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
            .reference_model_execution_plan(state.sim_setup.reference_pvt.process)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = model_execution_plan.model_cards();
        let generated_source = state
            .workspace
            .content
            .bind_generated_netlist_provenance(generated.netlist);
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
            &state.sim_setup.options,
        );
        let (expanded_netlist, sealed_source_dependencies) =
            expand_generated_dependencies_with_sealed_sources(
                &netlist,
                root_schematic.current_file(),
                &rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
                    state.workspace.content.project.include_search_paths(),
                    state.workspace.content.project.data_root(),
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
        let project_model_sources =
            prepared_project_model_sources(state.model_library_manager.catalog(), &netlist)?;

        let source_digest = generated_executable_source_digest(&netlist);
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
            drc.warnings().into_iter().map(|violation| {
                format!("{} · {}", violation.message, violation.location.display())
            }),
        );
        let touchstone_export = touchstone_export_policy(
            state,
            tasks.iter().map(PreparedTask::queued_analysis),
            root_schematic.current_file(),
        )?;

        PreparedRunSnapshot::new(SnapshotParts {
            measurement_references,
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(plan.plan_id()),
            project_revision: state.workspace.content.project.revision().get(),
            topology_revision: root_schematic.topology_version(),
            source_digest,
            reference_process: state.sim_setup.reference_pvt.process,
            reference_temperature_celsius: state.sim_setup.reference_pvt.temperature_celsius,
            run_set: Some(prepared_run_set),
            tasks,
            executable_netlist: netlist,
            save_policy: SavePolicy::PlanOwned {
                output_selection_mode: state.sim_setup.save_policy.output_selection_mode,
                retained_dataset_limit: state.sim_setup.save_policy.retained_dataset_limit,
                maximum_storage_bytes: state.sim_setup.save_policy.maximum_storage_bytes,
                live_streaming_enabled: state.sim_setup.save_policy.live_streaming_enabled,
                retain_failure_diagnostics: state.sim_setup.save_policy.retain_failure_diagnostics,
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

    fn build_prepared_manual_deck(
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
                && state.workspace.content.netlist_source.is_some());
        let source = if owned_active {
            state
                .workspace
                .content
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
            owned_source::compose_owned_netlist_execution_source(
                state.workspace.content.netlist_descriptor.as_ref(),
                state
                    .workspace
                    .content
                    .netlist_document
                    .as_ref()
                    .and_then(|document| document.generated_artifact()),
                source,
            )
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?
        } else {
            source.to_owned()
        };
        let descriptor = owned_active
            .then_some(state.workspace.content.netlist_descriptor.as_ref())
            .flatten();
        let owned_materialized =
            owned_source::adapt_owned_execution_profile(descriptor, &owned_materialized)
                .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
        let has_project_technology = state.project_technology_in_effect();
        let sealed_models = if has_project_technology {
            state.seal_project_execution_model_sources()
        } else {
            state
                .model_library_manager
                .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        }
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_execution_plan = if has_project_technology {
            Some(
                sealed_models
                    .reference_model_execution_plan(state.sim_setup.reference_pvt.process)
                    .map_err(|error| {
                        PreparationError::new(PreparationStage::ModelBindings, error)
                    })?,
            )
        } else {
            None
        };
        let model_cards = model_execution_plan.as_ref().map_or_else(
            Vec::new,
            crate::state::model_library::ModelExecutionPlan::model_cards,
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
        let origin = if owned_active {
            state.workspace.content.netlist_source_path.as_deref()
        } else {
            state.schematic.session.current_file.as_deref()
        };
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
                state.workspace.content.project.include_search_paths(),
                state.workspace.content.project.data_root(),
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
            prepared_project_model_sources(state.model_library_manager.catalog(), &expanded)?;
        let project_veriloga_runtimes = project_veriloga_runtimes_referenced_by(
            state.workspace.content.project.id(),
            &state.workspace.content.project_sources,
            state
                .ui
                .code_workspace
                .veriloga
                .receipt
                .as_ref()
                .map(|receipt| &receipt.compilation),
            &expanded,
        )?
        .try_merge(external_veriloga_runtimes)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let definitions = state
            .sim_setup
            .analysis_plan
            .as_ref()
            .and_then(|plan| state.workspace.content.plan_data(plan.id()))
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
            state.sim_setup.reference_pvt.temperature_celsius,
            &expanded,
        )
        .map_err(|errors| {
            PreparationError::new(PreparationStage::SourceChecks, errors.join("; "))
        })?;
        let source_digest = manual_executable_source_digest(&expanded);
        let tasks = prepare_manual_tasks(
            source_digest,
            state.workspace.content.project.revision(),
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
            state,
            tasks.iter().map(PreparedTask::queued_analysis),
            origin,
        )?;

        PreparedRunSnapshot::new(SnapshotParts {
            measurement_references,
            intent: SimulationRunIntent::ManualDeck,
            simulation_plan_id: None,
            project_revision: state.workspace.content.project.revision().get(),
            topology_revision: state.schematic.topology_version(),
            source_digest,
            reference_process: state.sim_setup.reference_pvt.process,
            reference_temperature_celsius: state.sim_setup.reference_pvt.temperature_celsius,
            run_set: None,
            tasks,
            executable_netlist: expanded,
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities,
            project_model_sources,
            specifications: Vec::new(),
            specification_policy: crate::state::PreparedSpecificationPolicy::default(),
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
pub(crate) fn design_inspection_input_digest(state: &AppState) -> crate::product::ContentDigest {
    let plan_identity = state.sim_setup.analysis_plan.as_ref().map_or_else(
        || "none".to_owned(),
        |plan| format!("{}:{}", plan.id(), plan.revision().get()),
    );
    let model_library_identity = state.model_library_manager.design_inspection_catalog_key();
    let material = format!(
        "{}\0{}\0{}\0{}\0{}\0{:?}\0{}\0{}\0{}",
        state.design_execution_epoch,
        state.workspace.content.project.revision().get(),
        state.workspace.content.simulation_root_reference().key(),
        state.workspace.content.active_view.key(),
        plan_identity,
        state.sim_setup.reference_pvt.process,
        state.sim_setup.reference_pvt.temperature_celsius,
        state.sim_setup.options.to_spice_options(),
        model_library_identity,
    );
    content_digest(
        "rspice.analysis-independent-design-inspection/v2",
        material.as_bytes(),
    )
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

    rspice_simulation::preparation::touchstone::touchstone_export_policy_for_dialog(
        &state.sim_setup.sp,
        state.schematic.document(),
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

fn reject_deferred_corner_model_sources<'a>(
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
    sealed_sources: &crate::state::model_library::SealedModelExecutionSources,
    identities: &mut Vec<ModelSourceIdentity>,
) {
    if let Some((label, archive_digest)) = sealed_sources.pdk_model_identity() {
        identities.push(ModelSourceIdentity::new(label, archive_digest));
    }
}

fn append_model_execution_plan_identity(
    plan: &crate::state::model_library::ModelExecutionPlan,
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
mod dispatch_parity_tests;
#[cfg(test)]
mod recorded_fft_tests;
// Visible to the controller rather than private, because `runnable_state` is
// the one fixture in this layer that produces a preparable project — and the
// projection ratchet has to prepare one without reaching up to the shell for
// a whole application.
#[cfg(test)]
pub(in crate::simulation::controller) mod tests;
