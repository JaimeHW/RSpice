//! Durable project execution context.
//!
//! The project format deliberately stores the authoritative simulation inputs
//! and model catalog separately from transient dialog/session state. Runtime
//! flags (open dialogs, palette filters, validation messages, browser expansion)
//! are reconstructed and never enter a project file.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::common::app::SimSetupState;
#[cfg(not(target_arch = "wasm32"))]
use crate::state::model_library::is_foreign_platform_absolute_path;
use crate::state::model_library::{
    DeviceModel, ModelLibrary, ModelLibraryManager, ModelSourceEdge, ModelSourcePin,
    ProcessCorner as LibraryProcessCorner, first_unreachable_source, is_portable_absolute_path,
};

pub const PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 3;
const LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 0;
const UNPINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 1;
const PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 2;

fn legacy_execution_context_schema_version() -> u32 {
    LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION
}

/// Versioned, project-owned inputs required to reproduce a simulation plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectExecutionContext {
    #[serde(default = "legacy_execution_context_schema_version")]
    pub schema_version: u32,
    pub simulation_plan: SimSetupState,
    pub model_libraries: Vec<ProjectModelLibrary>,
}

/// Serializable model-library data that affects model and section bindings.
/// UI-only manager selection/filter state and tree expansion are excluded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectModelLibrary {
    pub name: String,
    pub pdk_name: String,
    pub technology_node: String,
    pub root_path: Option<PathBuf>,
    /// Canonical root-plus-transitive-include source identities accepted by
    /// the last explicit load or refresh. Legacy external bindings may have
    /// an empty closure; they remain restorable but cannot participate in a
    /// simulation until the user explicitly refreshes or re-imports them.
    #[serde(default)]
    pub source_closure: Vec<ModelSourcePin>,
    /// Canonical resolution graph captured with the source closure. Schema 2
    /// projects may omit this field and remain repairable, but multi-file
    /// bindings stay blocked until explicit refresh.
    #[serde(default)]
    pub source_edges: Vec<ModelSourceEdge>,
    pub models: HashMap<String, DeviceModel>,
    pub corners: HashMap<String, LibraryProcessCorner>,
    pub selected_corner: Option<String>,
    pub version: String,
}

impl ProjectExecutionContext {
    pub fn from_state(
        simulation_plan: &SimSetupState,
        model_libraries: &ModelLibraryManager,
    ) -> Result<Self, String> {
        let mut simulation_plan = simulation_plan.clone();
        simulation_plan.ensure_initialized();
        simulation_plan.normalize_analysis_order();

        let context = Self {
            schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan,
            model_libraries: model_libraries
                .libraries_sorted()
                .into_iter()
                .map(ProjectModelLibrary::from)
                .collect(),
        };
        context.validate()?;
        Ok(context)
    }

    /// Apply explicit migrations for execution-context schemas that shipped
    /// before stable analysis ordering. No future schema is guessed.
    pub fn migrate_to_current(&mut self) -> Result<(), String> {
        match self.schema_version {
            LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION => {
                let mut order: Vec<_> = self.simulation_plan.enabled.iter().copied().collect();
                order.sort_unstable();
                self.simulation_plan.analysis_order = order;
                self.schema_version = UNPINNED_MODEL_SOURCE_SCHEMA_VERSION;
                self.migrate_to_current()
            }
            UNPINNED_MODEL_SOURCE_SCHEMA_VERSION => {
                // Schema 1 did not pin external model bytes. Never infer or
                // accept the file currently present on disk during migration:
                // retain the catalog and leave it unpinned so run binding
                // fails closed until an explicit refresh/re-import.
                for library in &mut self.model_libraries {
                    library.source_closure.clear();
                    library.source_edges.clear();
                }
                self.schema_version = PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION;
                self.migrate_to_current()
            }
            PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION => {
                // Schema 2 pinned canonical member bytes but did not retain
                // the filesystem resolution decisions needed to reproduce
                // aliases/search precedence without reopening paths. Keep the
                // pins/catalog for repair; multi-file execution fails closed
                // until refresh records the graph.
                for library in &mut self.model_libraries {
                    library.source_edges.clear();
                }
                self.schema_version = PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION;
                Ok(())
            }
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION => Ok(()),
            version => Err(format!(
                "unsupported execution-context schema version {version}; this build supports version {PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION}"
            )),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported execution-context schema version {}; this build supports version {}",
                self.schema_version, PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
            ));
        }
        validate_simulation_plan(&self.simulation_plan)?;
        validate_model_libraries(&self.model_libraries)
    }

    /// Restore authoritative state and report environment-dependent model
    /// source problems without substituting another source or section.
    pub fn into_state(
        mut self,
    ) -> Result<(SimSetupState, ModelLibraryManager, Vec<String>), String> {
        self.migrate_to_current()?;
        self.validate()?;
        let warnings = model_source_warnings(&self.model_libraries);
        let mut manager = ModelLibraryManager::new();
        for library in self.model_libraries {
            manager.add_library(library.into_model_library());
        }
        self.simulation_plan.prepare_after_restore();
        Ok((self.simulation_plan, manager, warnings))
    }
}

impl From<&ModelLibrary> for ProjectModelLibrary {
    fn from(library: &ModelLibrary) -> Self {
        Self {
            name: library.name.clone(),
            pdk_name: library.pdk_name.clone(),
            technology_node: library.technology_node.clone(),
            root_path: library.root_path.clone(),
            source_closure: library.source_closure.clone(),
            source_edges: library.source_edges.clone(),
            models: library.models.clone(),
            corners: library.corners.clone(),
            selected_corner: library.selected_corner.clone(),
            version: library.version.clone(),
        }
    }
}

impl ProjectModelLibrary {
    fn into_model_library(self) -> ModelLibrary {
        ModelLibrary {
            name: self.name,
            pdk_name: self.pdk_name,
            technology_node: self.technology_node,
            root_path: self.root_path,
            source_closure: self.source_closure,
            source_edges: self.source_edges,
            models: self.models,
            corners: self.corners,
            selected_corner: self.selected_corner,
            version: self.version,
            expanded: false,
        }
    }
}

fn validate_simulation_plan(plan: &SimSetupState) -> Result<(), String> {
    let analysis_count = crate::common::simulation_analysis_tabs::ANALYSIS_COUNT;
    validate_analysis_set("simulation_plan.enabled", &plan.enabled, analysis_count)?;
    validate_analysis_set("simulation_plan.listed", &plan.listed, analysis_count)?;

    let mut order_seen = HashSet::with_capacity(plan.analysis_order.len());
    for (position, index) in plan.analysis_order.iter().copied().enumerate() {
        if index >= analysis_count {
            return Err(format!(
                "simulation_plan.analysis_order[{position}] contains unsupported analysis index {index}; supported indices are 0..{}",
                analysis_count - 1
            ));
        }
        if !order_seen.insert(index) {
            return Err(format!(
                "simulation_plan.analysis_order contains duplicate analysis index {index}"
            ));
        }
        if !plan.enabled.contains(&index) {
            return Err(format!(
                "simulation_plan.analysis_order contains disabled analysis index {index}"
            ));
        }
    }
    if order_seen != plan.enabled {
        let mut missing: Vec<_> = plan.enabled.difference(&order_seen).copied().collect();
        missing.sort_unstable();
        return Err(format!(
            "simulation_plan.analysis_order is missing enabled analysis indices {}",
            format_indices(&missing)
        ));
    }

    validate_reference_pvt(plan)?;
    validate_solver_options(plan)?;
    validate_choice_indices(plan)?;
    Ok(())
}

fn validate_analysis_set(
    field: &str,
    indices: &HashSet<usize>,
    analysis_count: usize,
) -> Result<(), String> {
    if let Some(index) = indices
        .iter()
        .copied()
        .find(|index| *index >= analysis_count)
    {
        return Err(format!(
            "{field} contains unsupported analysis index {index}; supported indices are 0..{}",
            analysis_count - 1
        ));
    }
    Ok(())
}

fn validate_reference_pvt(plan: &SimSetupState) -> Result<(), String> {
    let temperature = plan.reference_pvt.temperature_celsius;
    if !temperature.is_finite() || temperature < -273.15 {
        return Err(format!(
            "simulation_plan.reference_pvt.temperature_celsius must be finite and at or above -273.15 C, got {temperature}"
        ));
    }
    if plan.options.temp != temperature {
        return Err(format!(
            "simulation_plan reference temperature ({temperature} C) disagrees with solver option temp ({} C)",
            plan.options.temp
        ));
    }
    Ok(())
}

fn validate_solver_options(plan: &SimSetupState) -> Result<(), String> {
    let options = &plan.options;
    let finite_fields = [
        ("reltol", options.reltol),
        ("residual_reltol", options.residual_reltol),
        ("vntol", options.vntol),
        ("abstol", options.abstol),
        ("iabstol", options.iabstol),
        ("chgtol", options.chgtol),
        ("pivrel", options.pivrel),
        ("pivtol", options.pivtol),
        ("gmin", options.gmin),
        ("bypass_reltol", options.bypass_reltol),
        ("bypass_abstol", options.bypass_abstol),
        ("min_timestep", options.min_timestep),
        ("max_timestep", options.max_timestep),
        ("timestep_factor", options.timestep_factor),
        ("temp", options.temp),
        ("tnom", options.tnom),
    ];
    if let Some((field, value)) = finite_fields
        .into_iter()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(format!(
            "simulation_plan.options.{field} must be finite, got {value}"
        ));
    }
    options.validate().map_err(|errors| {
        format!(
            "simulation_plan.options is invalid: {}",
            errors
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        )
    })?;
    let positive_fields = [
        ("pivrel", options.pivrel),
        ("pivtol", options.pivtol),
        ("bypass_reltol", options.bypass_reltol),
        ("bypass_abstol", options.bypass_abstol),
        ("timestep_factor", options.timestep_factor),
    ];
    if let Some((field, value)) = positive_fields.into_iter().find(|(_, value)| *value <= 0.0) {
        return Err(format!(
            "simulation_plan.options.{field} must be positive, got {value}"
        ));
    }
    if options.itl2 == 0 {
        return Err("simulation_plan.options.itl2 must be greater than zero".to_owned());
    }
    if options.gmin < 0.0 {
        return Err(format!(
            "simulation_plan.options.gmin must be non-negative, got {}",
            options.gmin
        ));
    }
    Ok(())
}

fn validate_choice_indices(plan: &SimSetupState) -> Result<(), String> {
    let choices = [
        ("ac.sweep", plan.ac.sweep, 2),
        ("pz.transfer_idx", plan.pz.transfer_idx, 1),
        ("pz.analysis_idx", plan.pz.analysis_idx, 2),
        ("sens.sens_type_idx", plan.sens.sens_type_idx, 1),
        ("mc.distribution_idx", plan.mc.distribution_idx, 2),
        ("mc.base_idx", plan.mc.base_idx, 3),
        ("pss.method_idx", plan.pss.method_idx, 1),
        ("temp.base_idx", plan.temp.base_idx, 3),
        ("hb.solver_idx", plan.hb.solver_idx, 1),
        ("sp.sweep_type_idx", plan.sp.sweep_type_idx, 2),
        ("pac.sweep_type_idx", plan.pac.sweep_type_idx, 2),
        ("pnoise.sweep_type_idx", plan.pnoise.sweep_type_idx, 2),
        ("pnoise.noise_ref_idx", plan.pnoise.noise_ref_idx, 2),
        ("pxf.sweep_type_idx", plan.pxf.sweep_type_idx, 2),
        ("xf.sweep_type_idx", plan.xf.sweep_type_idx, 2),
        ("corner.base_analysis_idx", plan.corner.base_analysis_idx, 3),
        ("envelope.modulation_idx", plan.envelope.modulation_idx, 3),
        ("optimization.goal_mode", plan.optimization.goal_mode, 2),
        ("optimization.algorithm", plan.optimization.algorithm, 2),
    ];
    if let Some((field, value, max)) = choices.into_iter().find(|(_, value, max)| value > max) {
        return Err(format!(
            "simulation_plan.{field} has invalid choice index {value}; maximum is {max}"
        ));
    }
    Ok(())
}

fn validate_model_libraries(libraries: &[ProjectModelLibrary]) -> Result<(), String> {
    let mut names = HashSet::with_capacity(libraries.len());
    for (library_index, library) in libraries.iter().enumerate() {
        let context = format!("model_libraries[{library_index}]");
        if library.name.trim().is_empty() {
            return Err(format!("{context}.name must not be empty"));
        }
        if !names.insert(library.name.as_str()) {
            return Err(format!(
                "model_libraries contains duplicate library name '{}'",
                library.name
            ));
        }
        if let Some(path) = &library.root_path
            && path.as_os_str().is_empty()
        {
            return Err(format!("{context}.root_path must not be empty"));
        }
        if library.root_path.is_none() && !library.source_closure.is_empty() {
            return Err(format!(
                "{context}.source_closure cannot exist without an external root_path"
            ));
        }
        if library.root_path.is_none() && !library.source_edges.is_empty() {
            return Err(format!(
                "{context}.source_edges cannot exist without an external root_path"
            ));
        }
        if library.source_closure.is_empty() && !library.source_edges.is_empty() {
            return Err(format!(
                "{context}.source_edges cannot exist without a pinned source_closure"
            ));
        }
        if let Some(root_path) = &library.root_path
            && !library.source_closure.is_empty()
        {
            if !is_portable_absolute_path(root_path) {
                return Err(format!(
                    "{context}.root_path must be an absolute desktop path when a source closure is pinned"
                ));
            }
            let mut source_paths = HashSet::with_capacity(library.source_closure.len());
            for (source_index, source) in library.source_closure.iter().enumerate() {
                if source.path.as_os_str().is_empty() || !is_portable_absolute_path(&source.path) {
                    return Err(format!(
                        "{context}.source_closure[{source_index}].path must be a non-empty absolute desktop path"
                    ));
                }
                if !source_paths.insert(&source.path) {
                    return Err(format!(
                        "{context}.source_closure contains duplicate path '{}'",
                        source.path.display()
                    ));
                }
            }
            if !source_paths.contains(root_path) {
                return Err(format!(
                    "{context}.source_closure does not contain root_path '{}'",
                    root_path.display()
                ));
            }
            if library
                .source_closure
                .iter()
                .all(|source| source.path.is_absolute())
                && library
                    .source_closure
                    .windows(2)
                    .any(|pair| pair[0].path >= pair[1].path)
            {
                return Err(format!(
                    "{context}.source_closure must be strictly sorted by canonical path"
                ));
            }
            let mut edge_keys = HashSet::with_capacity(library.source_edges.len());
            for (edge_index, edge) in library.source_edges.iter().enumerate() {
                if !is_portable_absolute_path(&edge.owner)
                    || !is_portable_absolute_path(&edge.target)
                {
                    return Err(format!(
                        "{context}.source_edges[{edge_index}] owner and target must be absolute desktop paths"
                    ));
                }
                if !source_paths.contains(&edge.owner) || !source_paths.contains(&edge.target) {
                    return Err(format!(
                        "{context}.source_edges[{edge_index}] references a source outside source_closure"
                    ));
                }
                let normalized = rspice_core::netlist::normalize_source_path_literal(
                    &edge.requested_path,
                )
                .map_err(|error| {
                    format!(
                        "{context}.source_edges[{edge_index}].requested_path is invalid: {error}"
                    )
                })?;
                if normalized != edge.requested_path {
                    return Err(format!(
                        "{context}.source_edges[{edge_index}].requested_path is not normalized"
                    ));
                }
                if !edge_keys.insert((&edge.owner, &edge.requested_path)) {
                    return Err(format!(
                        "{context}.source_edges contains duplicate owner/path identity '{}': '{}'",
                        edge.owner.display(),
                        edge.requested_path
                    ));
                }
            }
            if library
                .source_edges
                .iter()
                .all(|edge| edge.owner.is_absolute() && edge.target.is_absolute())
                && library
                    .source_edges
                    .windows(2)
                    .any(|pair| pair[0] >= pair[1])
            {
                return Err(format!(
                    "{context}.source_edges must be strictly sorted by owner, requested path, and target"
                ));
            }
            if !library.source_edges.is_empty()
                && let Some(unreachable) = first_unreachable_source(
                    root_path,
                    &library.source_closure,
                    &library.source_edges,
                )
            {
                return Err(format!(
                    "{context}.source_closure member '{}' is not reachable from root_path by authenticated resolution edges",
                    unreachable.display()
                ));
            }
        }
        if let Some(selected) = &library.selected_corner
            && !library.corners.contains_key(selected)
        {
            return Err(format!(
                "{context}.selected_corner '{}' does not exist in the library corner map",
                selected
            ));
        }
        for (corner_key, corner) in &library.corners {
            if corner_key.trim().is_empty() {
                return Err(format!("{context}.corners contains an empty section name"));
            }
            if corner.name != *corner_key {
                return Err(format!(
                    "{context}.corners key '{corner_key}' does not match embedded corner name '{}'",
                    corner.name
                ));
            }
            if !corner.temperature.is_finite()
                || corner.temperature < -273.15
                || !corner.vdd_factor.is_finite()
                || corner.vdd_factor <= 0.0
            {
                return Err(format!(
                    "{context}.corners['{corner_key}'] contains an invalid temperature or supply scaling"
                ));
            }
            if corner.file_path != library.root_path {
                return Err(format!(
                    "{context}.corners['{corner_key}'] source path does not match the library root path"
                ));
            }
        }
        for (model_key, model) in &library.models {
            if model_key.trim().is_empty() {
                return Err(format!("{context}.models contains an empty model name"));
            }
            if model.name != *model_key {
                return Err(format!(
                    "{context}.models key '{model_key}' does not match embedded model name '{}'",
                    model.name
                ));
            }
            if model.file_path != library.root_path {
                return Err(format!(
                    "{context}.models['{model_key}'] source path does not match the library root path"
                ));
            }
            validate_model_numbers(&context, model_key, model)?;
        }
    }
    Ok(())
}

fn validate_model_numbers(
    context: &str,
    model_key: &str,
    model: &DeviceModel,
) -> Result<(), String> {
    let optional_values = [
        ("l_min", model.l_min),
        ("l_max", model.l_max),
        ("w_min", model.w_min),
        ("w_max", model.w_max),
        ("vdd", model.vdd),
        ("vth0", model.vth0),
    ];
    if let Some((field, value)) = optional_values.into_iter().find_map(|(field, value)| {
        value
            .filter(|value| !value.is_finite())
            .map(|value| (field, value))
    }) {
        return Err(format!(
            "{context}.models['{model_key}'].{field} must be finite, got {value}"
        ));
    }
    if let (Some(min), Some(max)) = (model.l_min, model.l_max)
        && min > max
    {
        return Err(format!(
            "{context}.models['{model_key}'] has l_min greater than l_max"
        ));
    }
    if let (Some(min), Some(max)) = (model.w_min, model.w_max)
        && min > max
    {
        return Err(format!(
            "{context}.models['{model_key}'] has w_min greater than w_max"
        ));
    }
    if let Some((parameter, value)) = model
        .parameters
        .iter()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(format!(
            "{context}.models['{model_key}'].parameters['{parameter}'] must be finite, got {value}"
        ));
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn model_source_warnings(libraries: &[ProjectModelLibrary]) -> Vec<String> {
    let mut warnings = Vec::new();
    for library in libraries {
        let Some(path) = library.root_path.as_deref() else {
            continue;
        };
        if library.source_closure.is_empty() {
            warnings.push(format!(
                "Model library '{}' was restored with its persisted catalog and source binding, but the legacy binding is not content-pinned; refresh or re-import '{}' before simulation",
                library.name,
                path.display()
            ));
            continue;
        }
        if is_foreign_platform_absolute_path(path)
            || library
                .source_closure
                .iter()
                .any(|source| is_foreign_platform_absolute_path(&source.path))
        {
            warnings.push(format!(
                "Model library '{}' retains a foreign-platform source binding rooted at '{}'; it is unavailable on this host and simulations that require it remain blocked until it is re-imported or repaired",
                library.name,
                path.display()
            ));
            continue;
        }
        if library.source_closure.len() > 1 && library.source_edges.is_empty() {
            warnings.push(format!(
                "Model library '{}' was restored with schema-2 content pins but no authenticated dependency-resolution graph; refresh or re-import '{}' before simulation",
                library.name,
                path.display()
            ));
            continue;
        }
        if !library.source_edges.is_empty()
            && let Some(unreachable) =
                first_unreachable_source(path, &library.source_closure, &library.source_edges)
        {
            warnings.push(format!(
                "Model library '{}' retains source '{}' that is not reachable from its authenticated root; simulations remain blocked until the library is refreshed or re-imported",
                library.name,
                unreachable.display()
            ));
            continue;
        }
        for source in &library.source_closure {
            if !source.path.is_file() {
                warnings.push(format!(
                    "Model library '{}' was restored with its exact persisted source closure, but dependency '{}' is unavailable; simulations that require it remain blocked",
                    library.name,
                    source.path.display()
                ));
                break;
            }
            match ModelLibraryManager::calculate_source_digest(&source.path) {
                Ok(actual_digest) if actual_digest == source.digest => {}
                Ok(_) => {
                    warnings.push(format!(
                        "Model library '{}' was restored with its persisted catalog, but dependency '{}' differs from the explicitly accepted SHA-256 identity; simulation remains blocked until an explicit refresh or re-import accepts the new source closure",
                        library.name,
                        source.path.display()
                    ));
                    break;
                }
                Err(error) => {
                    warnings.push(format!(
                        "Model library '{}' was restored with its persisted catalog, but dependency '{}' could not be verified ({error}); simulation remains blocked",
                        library.name,
                        source.path.display()
                    ));
                    break;
                }
            }
        }
    }
    warnings
}

#[cfg(target_arch = "wasm32")]
fn model_source_warnings(libraries: &[ProjectModelLibrary]) -> Vec<String> {
    libraries
        .iter()
        .filter_map(|library| {
            library.root_path.as_ref().map(|path| {
                format!(
                    "Model library '{}' keeps its persisted source closure rooted at '{}', but browser builds cannot verify desktop file paths; simulations that require it remain blocked until the source is re-imported through an available browser workflow",
                    library.name,
                    path.display()
                )
            })
        })
        .collect()
}

fn format_indices(indices: &[usize]) -> String {
    indices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    fn model_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-project-execution-models-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create model fixture directory");
        let path = directory.join("foundry.lib");
        std::fs::write(
            directory.join("shared.inc"),
            ".model helper NMOS (LEVEL=1 KP=5e-4)\n",
        )
        .expect("write transitive model fixture");
        std::fs::write(
            &path,
            ".include \"shared.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n",
        )
        .expect("write model fixture");
        (directory, path)
    }

    #[test]
    fn transient_runtime_state_is_not_serialized() {
        let mut plan = SimSetupState::new();
        plan.options_open = true;
        plan.options_errors.push("not project data".to_owned());
        plan.palette_open = true;
        plan.palette_query = "noise".to_owned();
        let context = ProjectExecutionContext::from_state(&plan, &ModelLibraryManager::new())
            .expect("valid context");

        let value = serde_json::to_value(context).expect("serialize context");
        let plan = &value["simulation_plan"];
        assert!(plan.get("options_open").is_none());
        assert!(plan.get("options_errors").is_none());
        assert!(plan.get("options_draft").is_none());
        assert!(plan.get("palette_open").is_none());
        assert!(plan.get("palette_query").is_none());
        assert!(plan["op"].get("initialized").is_none());
    }

    #[test]
    fn incomplete_disabled_and_enabled_analysis_drafts_round_trip_losslessly() {
        use crate::common::simulation_analysis_tabs::{TAB_PSS, TAB_TRANSIENT};

        let mut plan = SimSetupState::new();
        plan.ensure_initialized();
        plan.pss.fund_freq = "unfinished-expression(".to_owned();
        plan.tran.stop = "also unfinished".to_owned();
        plan.enabled.insert(TAB_PSS);
        plan.analysis_order.push(TAB_PSS);
        // PSS proves a disabled draft is retained; Transient proves an enabled
        // invalid draft is persistable but remains blocked by run validation.
        plan.enabled.remove(&TAB_PSS);
        plan.analysis_order.retain(|index| *index != TAB_PSS);
        assert!(plan.enabled.contains(&TAB_TRANSIENT));

        let context = ProjectExecutionContext::from_state(&plan, &ModelLibraryManager::new())
            .expect("draft validity is a run concern, not a persistence concern");
        let serialized = serde_json::to_string(&context).expect("context serializes");
        let restored: ProjectExecutionContext =
            serde_json::from_str(&serialized).expect("context deserializes");
        let (restored, _, _) = restored.into_state().expect("context restores");

        assert_eq!(restored.pss.fund_freq, "unfinished-expression(");
        assert_eq!(restored.tran.stop, "also unfinished");
        assert!(restored.validation_error(TAB_PSS).is_some());
        assert!(restored.validation_error(TAB_TRANSIENT).is_some());
    }

    #[test]
    fn duplicate_and_unsupported_order_entries_fail_precisely() {
        let manager = ModelLibraryManager::new();
        let mut plan = SimSetupState::new();
        plan.analysis_order = vec![1, 1];
        let duplicate = ProjectExecutionContext {
            schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan: plan.clone(),
            model_libraries: Vec::new(),
        }
        .validate()
        .expect_err("duplicate must fail");
        assert!(duplicate.contains("duplicate analysis index 1"));

        plan.enabled.insert(99);
        plan.analysis_order = vec![1, 99];
        let unsupported = ProjectExecutionContext::from_state(&plan, &manager)
            .expect_err("unsupported analysis must fail");
        assert!(unsupported.contains("unsupported analysis index 99"));
    }

    #[test]
    fn legacy_context_migrates_to_sorted_execution_order() {
        let mut plan = SimSetupState::new();
        plan.ensure_initialized();
        plan.enabled.extend([4, 0]);
        plan.analysis_order.clear();
        let mut context = ProjectExecutionContext {
            schema_version: LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan: plan,
            model_libraries: Vec::new(),
        };

        context.migrate_to_current().expect("legacy migration");

        assert_eq!(context.simulation_plan.analysis_order, vec![0, 1, 4]);
        context.validate().expect("migrated context validates");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn model_source_and_section_bindings_round_trip_without_substitution() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("FF"))
            .expect("load FF section");
        let expected = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::FF)
            .expect("source FF binding");
        let context = ProjectExecutionContext::from_state(&SimSetupState::new(), &manager)
            .expect("context validates");
        let canonical_root = context.model_libraries[0]
            .root_path
            .clone()
            .expect("external library keeps canonical root");
        let json = serde_json::to_string(&context).expect("context serializes");
        let restored_context: ProjectExecutionContext =
            serde_json::from_str(&json).expect("context deserializes");

        let (_, restored_manager, warnings) = restored_context
            .into_state()
            .expect("available source restores");

        assert!(warnings.is_empty());
        assert_eq!(
            restored_manager
                .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::FF,)
                .expect("restored FF binding"),
            expected
        );
        assert_eq!(
            restored_manager
                .get_library("foundry")
                .expect("library restored")
                .selected_corner
                .as_deref(),
            Some("FF")
        );
        assert_eq!(
            restored_manager
                .get_library("foundry")
                .expect("library restored")
                .source_closure
                .iter()
                .find(|source| source.path == canonical_root)
                .expect("root pin restored")
                .digest,
            ModelLibraryManager::calculate_source_digest(&path).expect("fixture digest computes")
        );
        assert_eq!(
            restored_manager
                .get_library("foundry")
                .expect("library restored")
                .source_closure
                .len(),
            2
        );
        let restored_library = restored_manager
            .get_library("foundry")
            .expect("library restored");
        assert_eq!(restored_library.source_edges.len(), 1);
        assert_eq!(restored_library.source_edges[0].owner, canonical_root);
        assert_eq!(
            restored_library.source_edges[0].requested_path,
            "shared.inc"
        );

        std::fs::remove_dir_all(directory).expect("remove model fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn schema_one_external_source_migrates_unpinned_and_stays_blocked_until_refresh() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load source");
        let mut context = ProjectExecutionContext::from_state(&SimSetupState::new(), &manager)
            .expect("current context validates");
        context.schema_version = UNPINNED_MODEL_SOURCE_SCHEMA_VERSION;
        context.model_libraries[0].source_closure.clear();

        let (_, restored, warnings) = context
            .into_state()
            .expect("legacy unpinned catalog remains recoverable");

        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("legacy binding is not content-pinned"));
        let blocked = restored
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("unpinned legacy source must not run");
        assert!(blocked.contains("is not content-pinned"));

        std::fs::remove_dir_all(directory).expect("remove model fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn schema_two_multifile_source_migrates_without_inventing_resolution_edges() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load multifile source");
        let mut context = ProjectExecutionContext::from_state(&SimSetupState::new(), &manager)
            .expect("current context validates");
        context.schema_version = PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION;
        context.model_libraries[0].source_edges.clear();

        let (_, restored, warnings) = context
            .into_state()
            .expect("schema-two catalog remains repairable");
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("no authenticated dependency-resolution graph"));
        let blocked = restored
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("missing legacy graph must block a multifile source");
        assert!(
            blocked.contains("no authenticated resolution edge"),
            "{blocked}"
        );

        std::fs::remove_dir_all(directory).expect("remove model fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn unavailable_or_changed_model_source_is_retained_warned_and_run_blocked() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load model source");
        let context = ProjectExecutionContext::from_state(&SimSetupState::new(), &manager)
            .expect("context validates");
        let canonical_root = context.model_libraries[0]
            .root_path
            .clone()
            .expect("external library keeps canonical root");

        std::fs::remove_file(&path).expect("remove source");
        let (_, retained, warnings) = context
            .clone()
            .into_state()
            .expect("missing source is retained for repair");
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("is unavailable"));
        assert_eq!(
            retained
                .get_library("foundry")
                .expect("binding retained")
                .root_path
                .as_deref(),
            Some(canonical_root.as_path())
        );
        let unavailable = retained
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("missing source must block binding");
        assert!(unavailable.contains("is unavailable"));

        std::fs::write(
            &path,
            ".include \"shared.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=9e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=8e-3)\n.endl FF\n",
        )
        .expect("write changed source");
        let (_, retained, warnings) = context
            .into_state()
            .expect("changed source must not discard persisted catalog");
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("differs from the explicitly accepted SHA-256"));
        let changed = retained
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("changed source must block binding");
        assert!(changed.contains("dependency changed at"));

        std::fs::remove_dir_all(directory).expect("remove model fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn foreign_platform_source_binding_is_retained_without_filesystem_probe() {
        #[cfg(windows)]
        let root = PathBuf::from("/opt/foundry/models/device.lib");
        #[cfg(not(windows))]
        let root = PathBuf::from(r"C:\Foundry\Models\device.lib");

        assert!(is_portable_absolute_path(&root));
        assert!(is_foreign_platform_absolute_path(&root));

        let context = ProjectExecutionContext {
            schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan: SimSetupState::new(),
            model_libraries: vec![ProjectModelLibrary {
                name: "foreign-foundry".to_owned(),
                pdk_name: String::new(),
                technology_node: String::new(),
                root_path: Some(root.clone()),
                source_closure: vec![ModelSourcePin {
                    path: root.clone(),
                    digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
                }],
                source_edges: Vec::new(),
                models: HashMap::new(),
                corners: HashMap::new(),
                selected_corner: None,
                version: String::new(),
            }],
        };

        context
            .validate()
            .expect("foreign desktop syntax remains valid project metadata");
        let (_, manager, warnings) = context
            .into_state()
            .expect("foreign binding remains retained for repair");
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("foreign-platform"), "{:?}", warnings);

        let blocked = manager
            .seal_execution_sources()
            .expect_err("execution must fail before probing a foreign path");
        assert!(
            blocked.contains("foreign-platform") || blocked.contains("non-canonical"),
            "{blocked}"
        );
    }

    #[test]
    fn disconnected_source_subgraph_is_rejected_even_when_every_member_has_an_edge() {
        let directory = std::env::temp_dir().join("rspice-disconnected-persisted-graph");
        let root = directory.join("root.lib");
        let reachable = directory.join("reachable.inc");
        let orphan = directory.join("orphan.inc");
        let digest = crate::product::ContentDigest::from_bytes([0x33; 32]);
        let mut source_closure = vec![
            ModelSourcePin {
                path: root.clone(),
                digest,
            },
            ModelSourcePin {
                path: reachable.clone(),
                digest,
            },
            ModelSourcePin {
                path: orphan.clone(),
                digest,
            },
        ];
        source_closure.sort_by(|left, right| left.path.cmp(&right.path));
        let mut source_edges = vec![
            ModelSourceEdge {
                owner: root.clone(),
                requested_path: "reachable.inc".to_owned(),
                target: reachable,
            },
            ModelSourceEdge {
                owner: orphan.clone(),
                requested_path: "orphan.inc".to_owned(),
                target: orphan.clone(),
            },
        ];
        source_edges.sort();
        let context = ProjectExecutionContext {
            schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan: SimSetupState::new(),
            model_libraries: vec![ProjectModelLibrary {
                name: "disconnected".to_owned(),
                pdk_name: String::new(),
                technology_node: String::new(),
                root_path: Some(root),
                source_closure,
                source_edges,
                models: HashMap::new(),
                corners: HashMap::new(),
                selected_corner: None,
                version: String::new(),
            }],
        };

        let error = context
            .validate()
            .expect_err("all closure members must be root-reachable");
        assert!(error.contains("not reachable from root_path"), "{error}");
    }
}
