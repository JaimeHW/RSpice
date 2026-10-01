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
use rspice_app_types::canonical::content_digest;

#[cfg(test)]
mod measurement_tests;

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
use rspice_simulation::project_veriloga::preparation::{
    prepared_configuration_veriloga_runtimes, project_veriloga_runtimes_referenced_by,
};
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
    state: &AppState,
) -> rspice_simulation::netlist_gen::NetlistSourceData<'_> {
    rspice_simulation::netlist_gen::NetlistSourceData {
        files: &crate::simulation::table_route::SourceFiles,
        data_root: state.workspace.content.project.data_root(),
        stimulus_library: Some(&state.workspace.content.stimulus_library),
    }
}

fn run_preparation_inputs(
    state: &AppState,
) -> crate::simulation::execution::preparation::RunPreparationInputs<
    '_,
    crate::state::SimulationRun,
    crate::state::AnalysisResult,
> {
    crate::simulation::execution::preparation::RunPreparationInputs {
        analysis: analysis_inputs(state),
        technology: state.technology_inputs(),
        libraries: state.library_manager.catalog(),
        project_sources: &state.workspace.content.project_sources,
        design_management: &state.workspace.content.design_management,
        configuration_sets: &state.workspace.content.configuration_sets,
        source_data: project_netlist_source_data(state),
        schematic_source: rspice_design::projection::ProjectionSource::projection_source(
            &state.schematic,
        ),
        imported_checkpoints: &state.simulation.imported_monte_carlo_checkpoints,
        display_waveform_cache_samples: crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
        compilation: state
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation),
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
        crate::simulation::execution::preparation::build_prepared_run_set(
            &run_preparation_inputs(state),
            execution_projection,
        )
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
        let origin = if owned_active {
            state.workspace.content.netlist_source_path.as_deref()
        } else {
            state.schematic.session.current_file.as_deref()
        };
        crate::simulation::execution::preparation::build_prepared_manual_deck(
            &run_preparation_inputs(state),
            crate::simulation::execution::preparation::ManualSourceInputs {
                source,
                descriptor: state.workspace.content.netlist_descriptor.as_ref(),
                generated_artifact: state
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

#[cfg(test)]
fn touchstone_export_policy<'a>(
    state: &AppState,
    tasks: impl IntoIterator<Item = &'a QueuedAnalysis>,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    crate::simulation::execution::preparation::touchstone_export_policy(
        &state.sim_setup.sp,
        state.schematic.document(),
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
