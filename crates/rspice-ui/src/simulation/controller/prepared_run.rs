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

/// Let the deck resolve project-relative data-file references.
///
/// An unsaved project has no folder to resolve against, so its references stay
/// as written and the netlist generator reports the ones it cannot check.
fn bind_data_root<'a>(
    hierarchy: crate::simulation::netlist_gen::HierarchySource<'a>,
    state: &AppState,
) -> crate::simulation::netlist_gen::HierarchySource<'a> {
    match state.workspace.project.data_root() {
        Some(root) => hierarchy.with_data_root(root),
        None => hierarchy,
    }
}

/// Revalidate every model-bearing instance in the frozen hierarchy against
/// the project-global provider decision. Editor properties are not an
/// execution authority: restored projects and older symbol revisions must
/// pass this boundary immediately before their sources are sealed.
fn validate_projected_model_binding_authority(
    state: &AppState,
    projection: &crate::state::workspace::ConfigurationExecutionProjection,
) -> Result<(), PreparationError> {
    use crate::state::model_library::ModelConsumerScope;

    for (view, schematic) in projection.schematic_buffers() {
        for component in &schematic.components {
            let params = crate::state::parse_params_string(&component.params);
            let model_bound_cell = component.library_cell.as_ref().filter(|binding| {
                binding.netlist_template.is_some() && !binding.is_executable_builtin()
            });
            let (scope, definition) = if let Some(binding) = model_bound_cell {
                let definition = binding
                    .module_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|definition| !definition.is_empty())
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::ModelBindings,
                            format!(
                                "Model-bound instance '{}:{}' has no executable model or subcircuit name",
                                view, component.name
                            ),
                        )
                    })?;
                let is_subcircuit = binding
                    .effective_reference_prefix()
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("X"))
                    || binding
                        .netlist_template
                        .as_deref()
                        .is_some_and(|template| template.trim_start().starts_with("X{name}"));
                (
                    if is_subcircuit {
                        ModelConsumerScope::Subcircuit
                    } else {
                        ModelConsumerScope::PrimitiveModel
                    },
                    definition.to_owned(),
                )
            } else if let Some(definition) = params
                .get("model")
                .map(String::as_str)
                .map(str::trim)
                .filter(|definition| !definition.is_empty())
                .or_else(|| {
                    component_value_is_model_name(component.kind)
                        .then_some(component.value.as_str())
                        .map(str::trim)
                        .filter(|definition| !definition.is_empty())
                })
            {
                (ModelConsumerScope::PrimitiveModel, definition.to_owned())
            } else {
                continue;
            };

            let symbol_provider_library = model_bound_cell
                .map(|binding| bound_symbol_provider_library(state, binding, view, &component.name))
                .transpose()?
                .flatten();
            let selected_library = params
                .get("model_library")
                .map(String::as_str)
                .map(str::trim)
                .filter(|library| !library.is_empty())
                .map(str::to_owned)
                .or(symbol_provider_library);
            let providers = state
                .model_library_manager
                .definition_providers(scope, &definition);
            if providers.is_empty() {
                if let (Some(binding), Some(selected_library)) =
                    (model_bound_cell, selected_library.as_deref())
                    && selected_library.starts_with("signed-pdk:")
                    && signed_pdk_symbol_binding_matches(
                        state,
                        selected_library,
                        &definition,
                        binding.source_path.as_deref(),
                    )?
                {
                    continue;
                }
                if model_bound_cell.is_some() || selected_library.is_some() {
                    return Err(PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Instance '{}:{}' declares {} '{}' but its retained catalog provider is unavailable",
                            view,
                            component.name,
                            scope.label(),
                            definition
                        ),
                    ));
                }
                // Engine-native primitive models have no project catalog
                // provider. Their ordinary unresolved-model validation still
                // runs against the completed executable deck below.
                continue;
            }
            let effective = state
                .model_library_manager
                .effective_definition_provider(scope, &definition)
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Instance '{}:{}' cannot resolve: {error}",
                            view, component.name
                        ),
                    )
                })?
                .expect("a non-empty provider set has one effective provider");
            if let Some(selected_library) = selected_library.as_deref()
                && !effective.library.eq_ignore_ascii_case(selected_library)
            {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Instance '{}:{}' records model library '{}' but {} '{}' executes from project-global provider '{}'; review and rebind the instance",
                        view,
                        component.name,
                        selected_library,
                        scope.label(),
                        definition,
                        effective.library
                    ),
                ));
            }

            if let Some(binding) = model_bound_cell {
                let source_path = binding.source_path.as_deref().ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Model-bound instance '{}:{}' has no retained implementation source",
                            view, component.name
                        ),
                    )
                })?;
                let provider = state
                    .model_library_manager
                    .get_library(&effective.library)
                    .expect("the effective provider belongs to the live catalog");
                let source_matches = provider
                    .root_path
                    .as_deref()
                    .is_some_and(|path| model_source_paths_match(path, source_path))
                    || provider
                        .source_closure
                        .iter()
                        .any(|pin| model_source_paths_match(&pin.path, source_path));
                if !source_matches {
                    return Err(PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Model-bound instance '{}:{}' retains source '{}' but project-global provider '{}' authenticates a different source; recreate or rebind the symbol",
                            view,
                            component.name,
                            source_path.display(),
                            effective.library
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn signed_pdk_symbol_binding_matches(
    state: &AppState,
    provider_library: &str,
    definition: &str,
    source_path: Option<&Path>,
) -> Result<bool, PreparationError> {
    let Some(source_path) = source_path else {
        return Ok(false);
    };
    let package = state
        .project_signed_technology_package()
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!("Signed technology symbol authority is unavailable: {error}"),
            )
        })?
        .ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                "A signed-PDK symbol is bound but the project has no signed technology package",
            )
        })?;
    Ok(package.symbol_definitions().iter().any(|symbol| {
        symbol.netlist.model.as_ref().is_some_and(|model| {
            model.library.eq_ignore_ascii_case(provider_library)
                && model.model.eq_ignore_ascii_case(definition)
                && model.source_path.as_deref().is_some_and(|expected| {
                    model_source_paths_match(Path::new(expected), source_path)
                })
        })
    }))
}

fn bound_symbol_provider_library(
    state: &AppState,
    binding: &crate::state::LibraryCellInstance,
    view: &str,
    instance: &str,
) -> Result<Option<String>, PreparationError> {
    let Some(cell) = state
        .library_manager
        .get_library(&binding.library)
        .and_then(|library| library.get_cell(&binding.cell))
    else {
        return Ok(None);
    };
    let preferred = cell.get_view(&binding.view).into_iter();
    let remaining = cell
        .views_sorted()
        .into_iter()
        .filter(|candidate| !candidate.name.eq_ignore_ascii_case(&binding.view));
    for candidate in preferred.chain(remaining) {
        let definition = crate::state::ModelBoundSymbolDefinition::load_from_view(candidate)
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Model-bound instance '{}:{}' has an invalid retained symbol contract: {error}",
                        view, instance
                    ),
                )
            })?;
        if let Some(definition) = definition {
            return Ok(definition
                .netlist
                .model
                .as_ref()
                .map(|model| model.library.clone()));
        }
    }
    Ok(None)
}

fn model_source_paths_match(left: &Path, right: &Path) -> bool {
    if cfg!(windows) {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    } else {
        left == right
    }
}

fn component_value_is_model_name(kind: crate::state::ComponentType) -> bool {
    use crate::state::ComponentType;

    matches!(
        kind,
        ComponentType::Diode
            | ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi
            | ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::Nmesfet
            | ComponentType::Pmesfet
    )
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
        state.workspace.migrate_active_plan_data(current_id);
        state.workspace.migrate_inactive_plan_data(plan_id);
        state
            .sim_setup
            .activate_plan(plan_id)
            .map_err(|error| error.to_string())?;
        state.workspace.sync_legacy_specs_projection(plan_id);
        state
            .workspace
            .validate_simulation_configuration()
            .map_err(|error| error.to_string())?;
    }
    Ok(plan_name)
}

fn validate_plan_saved_output_budget(
    outputs: &[crate::state::SavedOutput],
    tasks: &[PreparedTask],
    run_set_point_count: usize,
    maximum_storage_bytes: u64,
    selection_mode: crate::state::OutputSelectionMode,
) -> Result<(), PreparationError> {
    let mut bounded_bytes = if selection_mode == crate::state::OutputSelectionMode::SaveAll {
        crate::simulation::output_contract::retained_engine_source_upper_bound_bytes(tasks.len())
    } else {
        0
    };
    let mut retained_engine_source_analyses = std::collections::HashSet::new();
    for output in outputs {
        let report = crate::simulation::output_contract::preflight_saved_output(
            output,
            tasks
                .iter()
                .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
        );
        match report.storage_estimate() {
            crate::simulation::SavedOutputStorageEstimate::ExactBytes(bytes) => {
                bounded_bytes = bounded_bytes.saturating_add(*bytes);
            }
            crate::simulation::SavedOutputStorageEstimate::Indeterminate { reason } => {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Saved-output storage budget cannot be proven for '{}': {reason}",
                        output.name
                    ),
                ));
            }
        }
        retained_engine_source_analyses
            .extend(report.retained_engine_source_analysis_ids().iter().copied());
    }
    if selection_mode != crate::state::OutputSelectionMode::SaveAll {
        bounded_bytes = bounded_bytes.saturating_add(
            crate::simulation::output_contract::retained_engine_source_upper_bound_bytes(
                retained_engine_source_analyses.len(),
            ),
        );
    }
    let forecast = bounded_bytes.saturating_mul(run_set_point_count as u64);
    if forecast > maximum_storage_bytes {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Saved-output forecast {} exceeds this plan's {} storage budget",
                crate::simulation::run_set::format_bytes(forecast),
                crate::simulation::run_set::format_bytes(maximum_storage_bytes)
            ),
        ));
    }
    Ok(())
}

const AUTOMATIC_OUTPUT_SMALL_DESIGN_LIMIT: usize = 16;
const AUTOMATIC_OUTPUT_HARD_LIMIT: usize = 32;

fn prepared_output_expression_key(expression: &str) -> String {
    expression
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Resolve the plan's effective saved-output set without mutating project
/// data. Automatic outputs belong to the prepared snapshot, use stable IDs,
/// and therefore remain deterministic for an unchanged plan and topology.
pub(super) fn effective_plan_saved_outputs(
    selection_mode: crate::state::OutputSelectionMode,
    explicit: &[crate::state::SavedOutput],
    probes: &[crate::state::SchematicProbe],
    nets: &[crate::simulation::netlist_gen::DesignNet],
    plan_id: crate::product::SimulationPlanId,
) -> Result<(Vec<crate::state::SavedOutput>, bool), PreparationError> {
    let mut enabled_probe_outputs =
        std::collections::HashMap::<crate::product::SavedOutputId, HashSet<String>>::new();
    let mut enabled_probe_expressions = std::collections::BTreeMap::new();
    for probe in probes.iter().filter(|probe| probe.enabled) {
        let Some(expression) = probe.source_expression.as_deref().map(str::trim) else {
            continue;
        };
        let expression_key = prepared_output_expression_key(expression);
        if expression_key == "v(0)" {
            continue;
        }
        enabled_probe_expressions
            .entry(expression_key.clone())
            .and_modify(|(_, plot): &mut (String, bool)| *plot |= probe.plot_on_materialization)
            .or_insert_with(|| (expression.to_owned(), probe.plot_on_materialization));
        if probe.plan_id == Some(plan_id)
            && let Some(output_id) = probe.saved_output_id
        {
            enabled_probe_outputs
                .entry(output_id)
                .or_default()
                .insert(expression_key);
        }
    }
    let mut explicit = explicit
        .iter()
        .filter(|output| {
            output.origin != crate::state::SavedOutputOrigin::SchematicProbe
                || enabled_probe_outputs
                    .get(&output.id)
                    .is_some_and(|expressions| {
                        expressions
                            .contains(&prepared_output_expression_key(&output.source_expression))
                    })
        })
        .cloned()
        .map(|mut output| {
            let expression_plot = enabled_probe_expressions
                .get(&prepared_output_expression_key(&output.source_expression))
                .map(|(_, plot)| *plot);
            if let Some(plot) = expression_plot {
                output.display_intent = if plot {
                    crate::state::SavedOutputDisplayIntent::Plot
                } else {
                    crate::state::SavedOutputDisplayIntent::DataBrowserOnly
                };
            }
            output
        })
        .collect::<Vec<_>>();
    let mut present_expressions = explicit
        .iter()
        .map(|output| prepared_output_expression_key(&output.source_expression))
        .collect::<HashSet<_>>();
    for (expression_key, (expression, plot)) in enabled_probe_expressions {
        if !present_expressions.insert(expression_key.clone()) {
            continue;
        }
        let mut output_name = expression.clone();
        if explicit
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(&output_name))
        {
            let mut ordinal = explicit.len().saturating_add(1);
            loop {
                let candidate = format!("Schematic probe {ordinal}");
                if !explicit
                    .iter()
                    .any(|output| output.name.eq_ignore_ascii_case(&candidate))
                {
                    output_name = candidate;
                    break;
                }
                ordinal = ordinal.saturating_add(1);
            }
        }
        let mut output = crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            output_name,
            expression,
            crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
            crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
            crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Schematic probe output is invalid: {error}"),
            )
        })?
        .with_origin(crate::state::SavedOutputOrigin::SchematicProbe)
        .with_display_intent(if plot {
            crate::state::SavedOutputDisplayIntent::Plot
        } else {
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        });
        let identity = format!("rspice.schematic-probe/v1/{expression_key}");
        output.id =
            crate::product::SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        explicit.push(output);
    }
    if selection_mode == crate::state::OutputSelectionMode::SaveAll {
        return Ok((explicit, false));
    }
    if !explicit.is_empty() {
        return Ok((explicit, false));
    }
    if selection_mode == crate::state::OutputSelectionMode::ExplicitOnly {
        return Ok((Vec::new(), false));
    }

    let non_ground_count = nets
        .iter()
        .filter(|net| net.class != crate::simulation::netlist_gen::NetClass::Ground)
        .count();
    let include_unnamed = non_ground_count <= AUTOMATIC_OUTPUT_SMALL_DESIGN_LIMIT;
    let mut candidates = nets
        .iter()
        .filter(|net| net.class != crate::simulation::netlist_gen::NetClass::Ground)
        .filter_map(|net| {
            let priority = match net.port {
                Some(crate::state::PortDirection::Out) => 0,
                Some(crate::state::PortDirection::InOut) => 1,
                _ if net.authored_name => 2,
                _ if include_unnamed => 3,
                _ => return None,
            };
            Some((priority, net.name.to_ascii_lowercase(), net))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        (left.0, left.1.as_str(), left.2.name.as_str()).cmp(&(
            right.0,
            right.1.as_str(),
            right.2.name.as_str(),
        ))
    });
    candidates.dedup_by(|left, right| left.1 == right.1);

    let mut outputs = Vec::with_capacity(candidates.len().min(AUTOMATIC_OUTPUT_HARD_LIMIT));
    for (priority, canonical_name, net) in candidates.into_iter().take(AUTOMATIC_OUTPUT_HARD_LIMIT)
    {
        let expression = format!("V({})", net.name);
        let mut output = crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            expression.clone(),
            expression,
            crate::state::SavedOutputCompatibility::OpTranAc,
            crate::state::SavedOutputPolicy::SelectedAndFinalPoints,
            crate::state::SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
            crate::state::SavedOutputStreaming::StoreOnly,
        )
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Automatic output for net '{}' is invalid: {error}",
                    net.name
                ),
            )
        })?
        .with_display_intent(if priority <= 1 {
            crate::state::SavedOutputDisplayIntent::Plot
        } else {
            crate::state::SavedOutputDisplayIntent::DataBrowserOnly
        });
        let identity = format!("rspice.automatic-node-voltage/v1/{canonical_name}");
        output.id =
            crate::product::SavedOutputId::from_namespace(plan_id.as_uuid(), identity.as_bytes());
        outputs.push(output);
    }
    Ok((outputs, true))
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
    ) -> Result<
        (
            Vec<crate::state::SavedOutput>,
            Vec<crate::simulation::SavedOutputPreflightReport>,
            bool,
        ),
        PreparationError,
    > {
        let selection_mode = state.sim_setup.save_policy.output_selection_mode;
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
        let root_schematic = projection.root_schematic().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::DesignChecks,
                "The configured simulation root is not materialized",
            )
        })?;
        let hierarchy = bind_data_root(
            crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
                &state.library_manager,
                &projection,
            ),
            state,
        );
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let nets =
            crate::simulation::netlist_gen::design_nets_with_hierarchy(root_schematic, &hierarchy);
        let (outputs, automatic_fallback) = effective_plan_saved_outputs(
            selection_mode,
            explicit,
            &root_schematic.probes,
            &nets,
            plan.id(),
        )?;
        let reports = self.saved_outputs_preflight(state, &outputs);
        Ok((outputs, reports, automatic_fallback))
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
        validate_projected_model_binding_authority(state, &projection)?;
        let hierarchy = bind_data_root(
            crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
                &state.library_manager,
                &projection,
            ),
            state,
        );
        let plan = state
            .sim_setup
            .stable_analysis_plan()
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        let payload = state.workspace.plan_data(plan.id()).ok_or_else(|| {
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
            .bind_generated_netlist_provenance(generated.netlist);
        let mut source =
            Self::apply_reference_model_bindings_to_netlist(&generated_source, &model_cards);
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_extend(
                prepared_model_library_veriloga_runtimes(&sealed_models)?
                    .iter()
                    .cloned(),
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        for runtime in external_veriloga_runtimes.iter() {
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
            let snapshot = self
                .build_prepared_snapshot(&frozen_state, SimulationRunIntent::SimulateRunSet)
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
            let digest = member.snapshot.digest();
            let dispatch = (|| {
                let permit = self
                    .execution_permits
                    .issue(digest)
                    .map_err(|error| format!("could not authorize member: {error}"))?;
                let proof = permit
                    .consume(digest, digest)
                    .map_err(|error| format!("could not consume member authorization: {error}"))?;
                member
                    .snapshot
                    .authorize_dispatch(proof)
                    .map_err(|error| error.to_string())
            })();
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

    /// Validate and consume an explicitly retained preflight snapshot. Run,
    /// collaborative approval, Automation, and tuning all prepare through an
    /// owning workflow before they request dispatch; the controller never
    /// manufactures hidden authorization at this final boundary.
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
            let message = match intent {
                SimulationRunIntent::ManualDeck => {
                    "Validate the exact current netlist before running; manual decks are never auto-authorized"
                }
                SimulationRunIntent::SimulateRunSet => {
                    "Run Simulation preflight before dispatch; Studio runs are never auto-authorized"
                }
            };
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                message,
            ));
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
        let hierarchy = bind_data_root(
            crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
                &state.library_manager,
                &execution_projection,
            ),
            state,
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
        validate_projected_model_binding_authority(state, &execution_projection)?;

        let plan = self.build_analysis_plan(state).map_err(|errors| {
            PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
        })?;
        let plan_payload = state.workspace.plan_data(plan.plan_id()).ok_or_else(|| {
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
        let tasks = self
            .build_queue_from_plan(state, &plan, &sealed_models)
            .map_err(|errors| {
                PreparationError::new(PreparationStage::AnalysisPlan, errors.join("; "))
            })?;
        let design_nets =
            crate::simulation::netlist_gen::design_nets_with_hierarchy(root_schematic, &hierarchy);
        let (effective_saved_outputs, used_automatic_outputs) = effective_plan_saved_outputs(
            state.sim_setup.save_policy.output_selection_mode,
            &plan_payload.saved_outputs,
            &root_schematic.probes,
            &design_nets,
            plan.plan_id(),
        )?;
        validate_plan_saved_output_budget(
            &effective_saved_outputs,
            &tasks,
            state.sim_setup.run_set.point_count(),
            state.sim_setup.save_policy.maximum_storage_bytes,
            state.sim_setup.save_policy.output_selection_mode,
        )?;
        let tasks = attach_saved_output_contracts(tasks, &effective_saved_outputs)?;
        if tasks.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "No runnable analyses were selected",
            ));
        }
        reject_deferred_corner_model_sources(tasks.iter().map(PreparedTask::queued_analysis))?;

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
            Self::corner_run_config_from_dialog(state, &run_set_config, &sealed_models).map_err(
                |error| {
                    PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!("Run Set model binding failed: {error}"),
                    )
                },
            )?;
        let prepared_run_set = crate::simulation::execution::PreparedRunSet::new(
            state.sim_setup.run_set.clone(),
            run_set_contract,
        );

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
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_extend(
                prepared_model_library_veriloga_runtimes(&sealed_models)?
                    .iter()
                    .cloned(),
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let project_veriloga_runtimes = project_veriloga_runtimes
            .try_extend(external_veriloga_runtimes.iter().cloned())
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

        let model_execution_plan = sealed_models
            .reference_model_execution_plan(state.sim_setup.reference_pvt.process)
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        let model_cards = model_execution_plan.model_cards();
        let generated_source = state
            .workspace
            .bind_generated_netlist_provenance(generated.netlist);
        let mut netlist =
            Self::apply_reference_model_bindings_to_netlist(&generated_source, &model_cards);
        for runtime in external_veriloga_runtimes.iter() {
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
        append_model_execution_plan_identity(&model_execution_plan, &mut model_identities);
        append_signed_pdk_model_identity(&sealed_models, &mut model_identities);
        append_corner_model_identities(
            tasks.iter().map(PreparedTask::queued_analysis),
            &mut model_identities,
        );

        let mut advisories = generated.warnings;
        if used_automatic_outputs {
            advisories.push(if effective_saved_outputs.is_empty() {
                "Automatic output selection found no eligible top-level voltage; the run will publish a guided empty waveform result.".to_owned()
            } else {
                format!(
                    "Automatic output selection retained {} bounded top-level voltage{} because the plan has no explicit outputs.",
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
        let mut composed = Self::apply_reference_model_bindings_to_netlist(&composed, &model_cards);
        let external_veriloga_runtimes = prepared_signed_pdk_veriloga_runtimes(&sealed_models)?
            .try_extend(
                prepared_model_library_veriloga_runtimes(&sealed_models)?
                    .iter()
                    .cloned(),
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
        for runtime in external_veriloga_runtimes.iter() {
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
            .try_extend(external_veriloga_runtimes.iter().cloned())
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
            intent: SimulationRunIntent::ManualDeck,
            simulation_plan_id: None,
            project_revision: state.workspace.project.revision().get(),
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
        let periodic_producers = prepared
            .iter()
            .filter(|task| matches!(&task.queued_analysis().spec, AnalysisSpec::Pss { .. }))
            .map(|task| {
                (
                    task.instance_id(),
                    task.source_revision(),
                    task.config_digest(),
                )
            })
            .collect::<Vec<_>>();
        let operating_point_producers = prepared
            .iter()
            .filter(|task| {
                matches!(
                    &task.queued_analysis().spec,
                    AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
                )
            })
            .map(|task| {
                (
                    task.instance_id(),
                    task.source_revision(),
                    task.config_digest(),
                )
            })
            .collect::<Vec<_>>();
        for task in &mut prepared {
            type ProducerIdentity = (
                crate::product::AnalysisInstanceId,
                crate::product::ObjectRevision,
                crate::product::ContentDigest,
            );
            type BindingConstructor = fn(
                crate::product::AnalysisInstanceId,
                crate::product::ObjectRevision,
                crate::product::ContentDigest,
            ) -> PreparedDependencyBinding;
            let (producers, artifact_label, binding): (
                &[ProducerIdentity],
                &str,
                BindingConstructor,
            ) = match &task.queued_analysis().spec {
                AnalysisSpec::Fourier { .. } => (
                    &transient_producers,
                    "Transient trajectory",
                    PreparedDependencyBinding::transient_trajectory,
                ),
                AnalysisSpec::Pss {
                    method: PssMethod::Shooting,
                    ..
                } => (
                    &operating_point_producers,
                    "operating-point seed",
                    PreparedDependencyBinding::dc_operating_point_seed,
                ),
                AnalysisSpec::PssSpectrum { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. } => (
                    &periodic_producers,
                    "shooting-PSS state",
                    PreparedDependencyBinding::periodic_state,
                ),
                _ => continue,
            };
            let [(producer_id, producer_revision, producer_config_digest)] = producers else {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Manual-deck {} requires exactly one prepared {artifact_label} producer; found {}",
                        task.queued_analysis().spec.run_type().display_name(),
                        producers.len()
                    ),
                ));
            };
            task.set_dependencies(vec![*producer_id]);
            task.set_dependency_bindings(vec![binding(
                *producer_id,
                *producer_revision,
                *producer_config_digest,
            )]);
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

fn prepared_model_library_veriloga_runtimes(
    sealed_models: &crate::state::model_library::SealedModelExecutionSources,
) -> Result<crate::simulation::veriloga::PreparedVerilogARuntimeSet, PreparationError> {
    let Some(authority) = sealed_models
        .model_library_veriloga_authority()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
    else {
        return Ok(Default::default());
    };
    crate::simulation::veriloga::compile_model_library_source_runtimes(&authority)
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
