use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use rspice_core::netlist::{IncludeProcessor, parse_include_directive, parse_lib_directive};

use super::*;
use crate::simulation::execution::{
    AuthorizedRunDispatch, CrossProbeSnapshot, ExecutionPermit, ExecutionTargetCapabilities,
    ModelSourceIdentity, PreparationError, PreparationStage, PreparedRunMetadata,
    PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy, SnapshotParts,
    TouchstoneExportPolicy, analysis_kind_tag, content_digest, drc_receipt_digest,
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
            crate::common::menu_bar::run_design_rule_check(state);
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
        if state.schematic.components.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::DesignChecks,
                "Add a component before preparing a schematic simulation",
            ));
        }
        let drc = state.dialogs.drc_results.as_ref().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::DesignChecks,
                "Run schematic source checks before simulation",
            )
        })?;
        if state.dialogs.drc_checked_version != state.schematic.topology_version() {
            return Err(PreparationError::new(
                PreparationStage::DesignChecks,
                "Schematic source-check receipt is stale for the current topology",
            ));
        }
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
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
            &state.library_manager,
            &state.workspace.schematic_buffers,
        );
        let analysis_instances = plan
            .instances()
            .iter()
            .map(crate::simulation::plan::FrozenAnalysisInstance::id)
            .collect::<Vec<_>>();
        let generated =
            crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
                &state.schematic,
                &analysis_lines,
                &hierarchy,
                &plan_payload.design_variables,
                crate::simulation::netlist_gen::DesignVariableNetlistContext {
                    active_cell: &state.workspace.active_view,
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
        let mut netlist =
            Self::apply_reference_model_bindings_to_netlist(&generated.netlist, &model_cards);
        netlist = Self::apply_simulation_options_to_netlist(&netlist, &state.sim_setup.options);
        reject_deferred_external_sources(&netlist)?;

        let source_digest =
            content_digest("rspice.generated-executable-source/v1", netlist.as_bytes());
        let receipt = RunSourceReceipt::SchematicDrc(drc_receipt_digest(
            state.schematic.topology_version(),
            drc,
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
            state.schematic.current_file.as_deref(),
        )?;

        PreparedRunSnapshot::new(SnapshotParts {
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(plan.plan_id()),
            project_revision: state.workspace.project.revision().get(),
            topology_revision: state.schematic.topology_version(),
            source_digest,
            reference_process: state.sim_setup.reference_pvt.process,
            reference_temperature_celsius: state.sim_setup.reference_pvt.temperature_celsius,
            tasks,
            executable_netlist: netlist,
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities,
            target: ExecutionTargetCapabilities::current(),
            receipt,
            advisories,
            manual_source: None,
            cross_probe: Some(CrossProbeSnapshot::new(
                generated.point_to_net,
                generated.nets,
                generated.net_segments,
                state.schematic.topology_version(),
            )),
            touchstone_export,
            sealed_source_dependencies: Vec::new(),
        })
    }

    fn build_prepared_manual_deck(
        &self,
        state: &AppState,
    ) -> Result<PreparedRunSnapshot, PreparationError> {
        if state.ui.netlist.active_document_initialized
            && state.ui.netlist.active_document
                == crate::workbench::netlist_document::ActiveNetlistDocument::GeneratedDiff
        {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                "Generated comparison documents cannot be executed",
            ));
        }
        let owned_active = state.ui.netlist.active_document
            == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource
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
            crate::common::netlist_workflow::compose_owned_netlist_execution_source(state, source)
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
            expand_manual_dependencies(&composed, origin)?;
        reject_deferred_external_sources(&expanded)?;
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
        );
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
    ) -> Vec<PreparedTask> {
        let mut kind_occurrences = std::collections::HashMap::<u8, usize>::new();
        tasks
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
            .collect()
    }
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

fn absolute_source_identity(path: &Path) -> Result<PathBuf, PreparationError> {
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

fn reject_deferred_external_sources(netlist: &str) -> Result<(), PreparationError> {
    for (line_number, logical_line) in executable_logical_lines(netlist) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use crate::services::drc::DrcResult;
    use crate::state::{ComponentType, Point};

    static FIXTURE_NONCE: AtomicU64 = AtomicU64::new(0);

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
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        let mut drc = DrcResult::new();
        drc.completed = true;
        state.dialogs.drc_results = Some(drc);
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        state
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
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 2;
        controller.cached_netlist = Some("existing sealed batch".to_owned());

        controller.start_authorized_snapshot(&mut state);

        assert_eq!(controller.current_run_id, Some(active_run_id));
        assert_eq!(controller.current_spec, Some(AnalysisSpec::DcOp));
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
