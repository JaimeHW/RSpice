//! Durable project execution context.
//!
//! The project format deliberately stores the authoritative simulation inputs
//! and model catalog separately from transient dialog/session state. Runtime
//! flags (open dialogs, palette filters, validation messages, browser expansion)
//! are reconstructed and never enter a project file.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::workbench::app_state::SimSetupState;
use crate::product::{ContentDigest, ProjectId};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::model_library::is_foreign_platform_absolute_path;
use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelDefinitionMetadata, ModelLibrary, ModelLibraryManager,
    ModelQualificationState, ModelSectionQualification, ModelSourceAuthority, ModelSourceContent,
    ModelSourceEdge, ModelSourceEvidenceBinding, ModelSourcePin, ParameterDataType, ParameterValue,
    ProcessCorner as LibraryProcessCorner, ProjectModelDefinition, ProjectModelRevisionDefinition,
    first_unreachable_source, is_portable_absolute_path,
};

pub const PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 9;
const LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 0;
const UNPINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 1;
const PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 2;
const SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION: u32 = 3;
const STABLE_ANALYSIS_PLAN_SCHEMA_VERSION: u32 = 4;
const PLAN_CATALOG_SCHEMA_VERSION: u32 = 5;
const RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION: u32 = 6;
const MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION: u32 = 7;
const MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION: u32 = 8;
const RETIRED_SINGLETON_ANALYSIS_FIELDS: &[&str] = &[
    "enabled",
    "analysis_order",
    "tran",
    "ac",
    "disto_f2_over_f1",
    "dc",
    "noise",
    "op",
    "pz",
    "sens",
    "mc",
    "pss",
    "stb",
    "temp",
    "hb",
    "sp",
    "pac",
    "pnoise",
    "pxf",
    "pstb",
    "xf",
    "corner",
    "envelope",
    "fourier",
    "reliability",
    "optimization",
    "soa",
    "listed",
];

fn legacy_execution_context_schema_version() -> u32 {
    LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION
}

/// Versioned, project-owned inputs required to reproduce a simulation plan.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectExecutionContext {
    pub schema_version: u32,
    pub simulation_plan: SimSetupState,
    pub model_libraries: Vec<ProjectModelLibrary>,
}

impl<'de> Deserialize<'de> for ProjectExecutionContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct PersistedExecutionContext {
            #[serde(default = "legacy_execution_context_schema_version")]
            schema_version: u32,
            simulation_plan: serde_json::Value,
            model_libraries: Vec<ProjectModelLibrary>,
        }

        let persisted = PersistedExecutionContext::deserialize(deserializer)?;
        if (STABLE_ANALYSIS_PLAN_SCHEMA_VERSION..=PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION)
            .contains(&persisted.schema_version)
        {
            let fields = persisted.simulation_plan.as_object().ok_or_else(|| {
                serde::de::Error::custom(format!(
                    "schema-{} simulation_plan must be a JSON object",
                    persisted.schema_version
                ))
            })?;
            if let Some(retired) = RETIRED_SINGLETON_ANALYSIS_FIELDS
                .iter()
                .find(|&&field| fields.contains_key(field))
            {
                return Err(serde::de::Error::custom(format!(
                    "schema-{} simulation_plan contains retired singleton field `{retired}`",
                    persisted.schema_version
                )));
            }
        }
        let simulation_plan =
            serde_json::from_value(persisted.simulation_plan).map_err(serde::de::Error::custom)?;
        Ok(Self {
            schema_version: persisted.schema_version,
            simulation_plan,
            model_libraries: persisted.model_libraries,
        })
    }
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
    #[serde(default)]
    pub source_authority: ModelSourceAuthority,
    /// Canonical root-plus-transitive-include source identities accepted by
    /// the last explicit load or refresh. Legacy external bindings may have
    /// an empty closure; they remain restorable but cannot participate in a
    /// simulation until the user explicitly refreshes or re-imports them.
    #[serde(default)]
    pub source_closure: Vec<ModelSourcePin>,
    /// Exact authenticated bytes retained for browser execution and
    /// self-contained project recovery.
    #[serde(default)]
    pub source_contents: Vec<ModelSourceContent>,
    /// Canonical resolution graph captured with the source closure. Schema 2
    /// projects may omit this field and remain repairable, but multi-file
    /// bindings stay blocked until explicit refresh.
    #[serde(default)]
    pub source_edges: Vec<ModelSourceEdge>,
    pub models: HashMap<String, DeviceModel>,
    #[serde(default)]
    pub model_definition_metadata: HashMap<String, ModelDefinitionMetadata>,
    #[serde(default)]
    pub model_qualification: HashMap<String, ModelQualificationState>,
    #[serde(default)]
    pub model_correlation: HashMap<String, ModelCorrelationState>,
    pub corners: HashMap<String, LibraryProcessCorner>,
    pub selected_corner: Option<String>,
    pub version: String,
}

impl ProjectExecutionContext {
    pub fn from_state(
        _project_id: ProjectId,
        simulation_plan: &SimSetupState,
        model_libraries: &ModelLibraryManager,
    ) -> Result<Self, String> {
        let mut simulation_plan = simulation_plan.clone();
        simulation_plan.analysis_plan.as_ref().ok_or_else(|| {
            "current simulation state has no stable analysis plan; legacy singleton migration is load-only"
                .to_owned()
        })?;
        simulation_plan.prepare_after_restore();

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
    pub fn migrate_to_current(&mut self, project_id: ProjectId) -> Result<(), String> {
        match self.schema_version {
            LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION => {
                let mut order: Vec<_> = self.simulation_plan.enabled.iter().copied().collect();
                order.sort_unstable();
                self.simulation_plan.analysis_order = order;
                self.schema_version = UNPINNED_MODEL_SOURCE_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
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
                self.migrate_to_current(project_id)
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
                self.schema_version = SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION => {
                self.simulation_plan
                    .migrate_legacy_analysis_plan(project_id)?;
                self.schema_version = STABLE_ANALYSIS_PLAN_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            STABLE_ANALYSIS_PLAN_SCHEMA_VERSION => {
                // Schema 4 persisted one stable analysis plan but had no
                // named-plan catalog. Serde defaults deterministically promote
                // that plan to the canonical initial name and root lineage.
                self.simulation_plan
                    .validate_plan_catalog()
                    .map_err(|error| {
                        format!("simulation plan catalog migration failed: {error}")
                    })?;
                self.schema_version = PLAN_CATALOG_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            PLAN_CATALOG_SCHEMA_VERSION => {
                // Schema 5 pinned source identities and resolution edges but
                // did not retain the authenticated bytes needed by browser
                // execution. Leave legacy bindings repairable and fail closed
                // until an explicit refresh captures their exact bytes.
                for library in &mut self.model_libraries {
                    library.source_contents.clear();
                }
                self.schema_version = RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION => {
                // Schema 6 retained exact bytes for browser execution but did
                // not distinguish their ownership. Never infer edit authority
                // from byte availability: a legacy path remains external, and
                // a source-less catalog remains built-in.
                for library in &mut self.model_libraries {
                    library.source_authority = if library.root_path.is_some() {
                        ModelSourceAuthority::External
                    } else {
                        ModelSourceAuthority::BuiltIn
                    };
                }
                self.schema_version = MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION => {
                // Schema 7 introduced explicit source authority but predated
                // typed model-authoring and qualification records. Serde
                // defaults preserve those projects as honest empty metadata;
                // no schema, test, or release facts are inferred.
                self.schema_version = MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION => {
                // Schema 8 introduced typed authoring and qualification but
                // predated persisted measurement-correlation records. Serde
                // defaults preserve those projects with an honest empty
                // correlation ledger; no datasets, metrics, or review
                // evidence are inferred during migration.
                self.schema_version = PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION;
                Ok(())
            }
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION => {
                self.simulation_plan.prepare_after_restore();
                Ok(())
            }
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

    /// Bind project technology metadata to the exact execution library it
    /// governs. Descriptor and execution-context validation are intentionally
    /// joined here so a project can never advertise one accepted technology
    /// while executing a refreshed, removed, or substituted catalog.
    pub(crate) fn validate_technology_binding(
        &self,
        binding: &crate::state::ProjectTechnologyBinding,
    ) -> Result<(), String> {
        let mut matches = self
            .model_libraries
            .iter()
            .filter(|library| library.name == binding.model_library());
        let library = matches.next().ok_or_else(|| {
            format!(
                "attached technology references missing model library '{}'",
                binding.model_library()
            )
        })?;
        if matches.next().is_some() {
            return Err(format!(
                "attached technology model library '{}' is ambiguous",
                binding.model_library()
            ));
        }
        binding
            .validate_model_library(&library.clone().into_model_library())
            .map_err(|error| error.to_string())
    }

    /// Restore authoritative state and report environment-dependent model
    /// source problems without substituting another source or section.
    pub fn into_state(
        mut self,
        project_id: ProjectId,
    ) -> Result<(SimSetupState, ModelLibraryManager, Vec<String>), String> {
        self.migrate_to_current(project_id)?;
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
            source_authority: library.source_authority,
            source_closure: library.source_closure.clone(),
            source_contents: library.source_contents.clone(),
            source_edges: library.source_edges.clone(),
            models: library.models.clone(),
            model_definition_metadata: library.model_definition_metadata.clone(),
            model_qualification: library.model_qualification.clone(),
            model_correlation: library.model_correlation.clone(),
            corners: library.corners.clone(),
            selected_corner: library.selected_corner.clone(),
            version: library.version.clone(),
        }
    }
}

impl ProjectModelLibrary {
    fn into_model_library(mut self) -> ModelLibrary {
        if let ModelSourceAuthority::ProjectOwned { source_id, .. } = self.source_authority {
            let persisted_root = self
                .root_path
                .clone()
                .expect("validated project-owned library has a root identity");
            let host_root = crate::state::model_library::project_owned_source_path(source_id);
            let mut path_map = BTreeMap::new();
            path_map.insert(persisted_root.clone(), host_root.clone());
            let host_parent = host_root
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."));
            for (index, source) in self
                .source_closure
                .iter()
                .filter(|source| source.path != persisted_root)
                .enumerate()
            {
                path_map.insert(
                    source.path.clone(),
                    host_parent.join(format!("member-{:04}.model-source", index + 1)),
                );
            }
            self.root_path = Some(host_root.clone());
            for source in &mut self.source_closure {
                source.path = path_map
                    .get(&source.path)
                    .expect("validated closure member has a restored path identity")
                    .clone();
            }
            for content in &mut self.source_contents {
                content.path = path_map
                    .get(&content.path)
                    .expect("validated retained source has a restored path identity")
                    .clone();
            }
            for edge in &mut self.source_edges {
                edge.owner = path_map
                    .get(&edge.owner)
                    .expect("validated dependency owner has a restored path identity")
                    .clone();
                edge.target = path_map
                    .get(&edge.target)
                    .expect("validated dependency target has a restored path identity")
                    .clone();
            }
            for model in self.models.values_mut() {
                if let Some(path) = model.file_path.as_mut() {
                    *path = path_map
                        .get(path)
                        .expect("validated model source has a restored path identity")
                        .clone();
                }
            }
            for corner in self.corners.values_mut() {
                if let Some(path) = corner.file_path.as_mut() {
                    *path = path_map
                        .get(path)
                        .expect("validated corner source has a restored path identity")
                        .clone();
                }
            }
            self.source_closure
                .sort_by(|left, right| left.path.cmp(&right.path));
            self.source_contents
                .sort_by(|left, right| left.path.cmp(&right.path));
            self.source_edges.sort();
        }
        ModelLibrary {
            name: self.name,
            pdk_name: self.pdk_name,
            technology_node: self.technology_node,
            root_path: self.root_path,
            source_authority: self.source_authority,
            source_closure: self.source_closure,
            source_contents: self.source_contents,
            source_edges: self.source_edges,
            models: self.models,
            model_definition_metadata: self.model_definition_metadata,
            model_qualification: self.model_qualification,
            model_correlation: self.model_correlation,
            corners: self.corners,
            selected_corner: self.selected_corner,
            version: self.version,
            expanded: false,
        }
    }
}

fn validate_simulation_plan(plan: &SimSetupState) -> Result<(), String> {
    plan.validate_plan_catalog()
        .map_err(|error| format!("simulation_plan catalog is invalid: {error}"))?;
    validate_active_simulation_plan(plan)?;

    let inactive_ids = plan
        .inactive_plans()
        .iter()
        .map(crate::workbench::app_state::StoredSimulationPlan::id)
        .collect::<Vec<_>>();
    for id in inactive_ids {
        let mut projection = plan.clone();
        projection.prepare_after_restore();
        projection.activate_plan(id).map_err(|error| {
            format!("simulation_plan catalog entry {id} could not be activated: {error}")
        })?;
        validate_active_simulation_plan(&projection)?;
    }
    Ok(())
}

fn validate_active_simulation_plan(plan: &SimSetupState) -> Result<(), String> {
    let stable_plan = plan.stable_analysis_plan()?;
    stable_plan
        .validate_structure()
        .map_err(|error| format!("simulation_plan.analysis_plan is invalid: {error}"))?;

    validate_reference_pvt(plan)?;
    validate_solver_options(plan)?;
    for instance in stable_plan.instances() {
        let mut projection = plan.clone();
        projection.apply_analysis_draft_projection(instance.draft());
        validate_choice_indices(&projection).map_err(|error| {
            format!(
                "simulation_plan.analysis_plan instance {} ({}) is invalid: {error}",
                instance.id(),
                instance.kind().stable_id()
            )
        })?;
    }
    Ok(())
}

fn validate_reference_pvt(plan: &SimSetupState) -> Result<(), String> {
    let temperature = plan.reference_pvt.temperature_celsius;
    if !temperature.is_finite() || temperature <= -273.15 {
        return Err(format!(
            "simulation_plan.reference_pvt.temperature_celsius must be finite and above -273.15 C, got {temperature}"
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
        (
            "envelope.initial_periodic_solve_idx",
            plan.envelope.initial_periodic_solve_idx,
            2,
        ),
        (
            "envelope.adaptive_mode_idx",
            plan.envelope.adaptive_mode_idx,
            2,
        ),
        (
            "envelope.extraction_path_idx",
            plan.envelope.extraction_path_idx,
            0,
        ),
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
        match library.source_authority {
            ModelSourceAuthority::BuiltIn => {
                if library.root_path.is_some()
                    || !library.source_closure.is_empty()
                    || !library.source_contents.is_empty()
                    || !library.source_edges.is_empty()
                {
                    return Err(format!(
                        "{context}.source_authority built_in cannot own a root path or source closure"
                    ));
                }
            }
            ModelSourceAuthority::External => {
                if library.root_path.is_none() {
                    return Err(format!(
                        "{context}.source_authority external requires a root_path"
                    ));
                }
            }
            ModelSourceAuthority::ProjectOwned { digest, .. } => {
                let Some(root_path) = library.root_path.as_ref() else {
                    return Err(format!(
                        "{context}.source_authority project_owned requires a root identity"
                    ));
                };
                if library.source_closure.is_empty()
                    || library.source_closure.len() != library.source_contents.len()
                {
                    return Err(format!(
                        "{context}.source_authority project_owned requires exact retained bytes for its complete source closure"
                    ));
                }
                let root_pin = library
                    .source_closure
                    .iter()
                    .find(|source| source.path == *root_path)
                    .ok_or_else(|| {
                        format!(
                            "{context}.source_authority project_owned closure does not contain its root identity"
                        )
                    })?;
                if root_pin.digest != digest {
                    return Err(format!(
                        "{context}.source_authority project_owned root bytes do not match the authority digest"
                    ));
                }
            }
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
        if library.source_closure.is_empty() && !library.source_contents.is_empty() {
            return Err(format!(
                "{context}.source_contents cannot exist without a pinned source_closure"
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
            if !library.source_contents.is_empty() {
                if library.source_contents.len() != library.source_closure.len() {
                    return Err(format!(
                        "{context}.source_contents must contain exact bytes for every pinned source"
                    ));
                }
                for (source_index, (pin, content)) in library
                    .source_closure
                    .iter()
                    .zip(&library.source_contents)
                    .enumerate()
                {
                    if pin.path != content.path {
                        return Err(format!(
                            "{context}.source_contents[{source_index}] does not match source_closure path '{}'",
                            pin.path.display()
                        ));
                    }
                    let digest = crate::product::ContentDigest::from_bytes(
                        Sha256::digest(&content.bytes).into(),
                    );
                    if digest != pin.digest {
                        return Err(format!(
                            "{context}.source_contents[{source_index}] bytes do not match the pinned SHA-256"
                        ));
                    }
                }
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
            if (matches!(
                library.source_authority,
                ModelSourceAuthority::ProjectOwned { .. }
            ) || !library.source_edges.is_empty())
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
            if !library.source_contents.is_empty() {
                validate_authenticated_source_projection(&context, library, &source_paths)?;
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
                || corner.temperature <= -273.15
                || !corner.vdd_factor.is_finite()
                || corner.vdd_factor <= 0.0
            {
                return Err(format!(
                    "{context}.corners['{corner_key}'] contains an invalid temperature or supply scaling"
                ));
            }
            if !source_path_is_authorized(library, corner.file_path.as_ref()) {
                return Err(format!(
                    "{context}.corners['{corner_key}'] source path is not a member of the authenticated library closure"
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
            if !source_path_is_authorized(library, model.file_path.as_ref()) {
                return Err(format!(
                    "{context}.models['{model_key}'] source path is not a member of the authenticated library closure"
                ));
            }
            validate_model_numbers(&context, model_key, model)?;
        }
        validate_model_authoring_records(&context, library)?;
    }
    Ok(())
}

fn source_path_is_authorized(library: &ProjectModelLibrary, source_path: Option<&PathBuf>) -> bool {
    if library.source_closure.is_empty() {
        match library.source_authority {
            // Legacy external projects may retain parsed models from include
            // members while lacking the authenticated closure introduced by
            // later schemas. Keep those absolute identities diagnosable; the
            // empty closure still blocks every execution path until refresh.
            ModelSourceAuthority::External => {
                source_path.is_some_and(|path| is_portable_absolute_path(path))
            }
            ModelSourceAuthority::BuiltIn => source_path.is_none(),
            ModelSourceAuthority::ProjectOwned { .. } => false,
        }
    } else {
        source_path.is_some_and(|path| {
            library
                .source_closure
                .iter()
                .any(|source| source.path == *path)
        })
    }
}

fn validate_authenticated_source_projection(
    context: &str,
    library: &ProjectModelLibrary,
    source_paths: &HashSet<&PathBuf>,
) -> Result<(), String> {
    let root = library
        .root_path
        .as_ref()
        .expect("authenticated closure validation requires a root path");
    let sources = library
        .source_contents
        .iter()
        .map(|content| (content.path.clone(), content.bytes.clone()));
    let dependencies =
        library
            .source_edges
            .iter()
            .map(|edge| rspice_core::library::ResolvedLibDependency {
                owner: edge.owner.clone(),
                requested_path: edge.requested_path.clone(),
                target: edge.target.clone(),
            });
    let mut parser = rspice_core::library::LibParser::new(
        root.parent().unwrap_or_else(|| std::path::Path::new(".")),
    );
    let parsed = parser
        .parse_authenticated_closure(root.clone(), sources, dependencies)
        .map_err(|error| format!("{context}.source_contents cannot be authenticated: {error}"))?;
    if !parsed.is_ok() {
        return Err(format!(
            "{context}.source_contents do not form a valid authenticated library closure: {}",
            parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }

    let observed_sources = parsed
        .resolved_sources
        .iter()
        .map(|source| &source.path)
        .collect::<BTreeSet<_>>();
    let expected_sources = source_paths.iter().copied().collect::<BTreeSet<_>>();
    if observed_sources != expected_sources {
        return Err(format!(
            "{context}.source_closure contains retained members that are not consumed by the authenticated parse"
        ));
    }
    let observed_dependencies = parsed
        .resolved_dependencies
        .iter()
        .map(|edge| (&edge.owner, edge.requested_path.as_str(), &edge.target))
        .collect::<BTreeSet<_>>();
    let expected_dependencies = library
        .source_edges
        .iter()
        .map(|edge| (&edge.owner, edge.requested_path.as_str(), &edge.target))
        .collect::<BTreeSet<_>>();
    if observed_dependencies != expected_dependencies {
        return Err(format!(
            "{context}.source_edges do not exactly describe the dependencies consumed by the authenticated parse"
        ));
    }

    if matches!(
        library.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ) {
        let mut executable_models = parsed.top_level_models.iter().collect::<Vec<_>>();
        if let Some(selected_corner) = library.selected_corner.as_deref()
            && let Some(section) = parsed.get_section(selected_corner)
        {
            executable_models.extend(&section.models);
        }
        let parsed_models = executable_models
            .into_iter()
            .map(|model| ModelLibraryManager::convert_parsed_model(model, root))
            .collect::<Vec<_>>();
        for (model_name, model) in &library.models {
            if !parsed_models
                .iter()
                .any(|candidate| parsed_model_projection_matches(model, candidate))
            {
                return Err(format!(
                    "{context}.models['{model_name}'] is not an exact projection of any model card in the authenticated source closure"
                ));
            }
        }
        let expected_model_names = parsed_models
            .iter()
            .map(|model| model.name.as_str())
            .collect::<BTreeSet<_>>();
        if library.models.len() != expected_model_names.len()
            || expected_model_names
                .iter()
                .any(|model_name| !library.models.contains_key(*model_name))
        {
            return Err(format!(
                "{context}.models does not exactly cover the top-level and selected-section model cards in the authenticated source closure"
            ));
        }
    }
    Ok(())
}

fn parsed_model_projection_matches(persisted: &DeviceModel, parsed: &DeviceModel) -> bool {
    persisted.name == parsed.name
        && persisted.model_type == parsed.model_type
        && persisted.spice_type == parsed.spice_type
        && persisted.level == parsed.level
        && persisted.spice_level == parsed.spice_level
        && persisted.model_version.map(f64::to_bits) == parsed.model_version.map(f64::to_bits)
        && persisted.l_min.map(f64::to_bits) == parsed.l_min.map(f64::to_bits)
        && persisted.l_max.map(f64::to_bits) == parsed.l_max.map(f64::to_bits)
        && persisted.w_min.map(f64::to_bits) == parsed.w_min.map(f64::to_bits)
        && persisted.w_max.map(f64::to_bits) == parsed.w_max.map(f64::to_bits)
        && persisted.file_path == parsed.file_path
        && persisted.parameters.len() == parsed.parameters.len()
        && persisted.parameters.iter().all(|(name, value)| {
            parsed
                .parameters
                .get(name)
                .is_some_and(|candidate| candidate.to_bits() == value.to_bits())
        })
        && persisted.string_parameters == parsed.string_parameters
        && persisted.source_line == parsed.source_line
}

fn validate_model_authoring_records(
    context: &str,
    library: &ProjectModelLibrary,
) -> Result<(), String> {
    let mut metadata_names = HashSet::with_capacity(library.model_definition_metadata.len());
    for (model_name, metadata) in &library.model_definition_metadata {
        let canonical = model_name.to_ascii_lowercase();
        if !metadata_names.insert(canonical) {
            return Err(format!(
                "{context}.model_definition_metadata contains duplicate case-insensitive model names"
            ));
        }
        let model = library.models.get(model_name).ok_or_else(|| {
            format!(
                "{context}.model_definition_metadata['{model_name}'] has no exact model projection"
            )
        })?;
        metadata.validate().map_err(|error| {
            format!("{context}.model_definition_metadata['{model_name}'] is invalid: {error}")
        })?;
        if metadata.parameters.len() != model.parameters.len() + model.string_parameters.len() {
            return Err(format!(
                "{context}.model_definition_metadata['{model_name}'].parameters does not exactly cover the model source parameters"
            ));
        }
        for parameter in &metadata.parameters {
            match (&parameter.data_type, &parameter.value) {
                (ParameterDataType::Numeric, ParameterValue::Numeric(value)) => {
                    let observed = model
                        .parameters
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(&parameter.name))
                        .map(|(_, value)| *value);
                    if observed.is_none_or(|observed| observed.to_bits() != value.get().to_bits()) {
                        return Err(format!(
                            "{context}.model_definition_metadata['{model_name}'].parameters['{}'] does not match the numeric model source value",
                            parameter.name
                        ));
                    }
                }
                (ParameterDataType::String, ParameterValue::String(value)) => {
                    let observed = model
                        .string_parameters
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(&parameter.name))
                        .map(|(_, value)| value);
                    if observed != Some(value) {
                        return Err(format!(
                            "{context}.model_definition_metadata['{model_name}'].parameters['{}'] does not match the string model source value",
                            parameter.name
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "{context}.model_definition_metadata['{model_name}'].parameters['{}'] has inconsistent type and value",
                        parameter.name
                    ));
                }
            }
        }
    }

    let mut qualification_names = HashSet::with_capacity(library.model_qualification.len());
    for (model_name, qualification) in &library.model_qualification {
        if !qualification_names.insert(model_name.to_ascii_lowercase()) {
            return Err(format!(
                "{context}.model_qualification contains duplicate case-insensitive model names"
            ));
        }
        if !library.models.contains_key(model_name) {
            return Err(format!(
                "{context}.model_qualification['{model_name}'] has no exact model projection"
            ));
        }
        qualification
            .validate_for_model(model_name)
            .map_err(|error| {
                format!("{context}.model_qualification['{model_name}'] is invalid: {error}")
            })?;
    }

    let mut correlation_names = HashSet::with_capacity(library.model_correlation.len());
    for (model_name, correlation) in &library.model_correlation {
        if !correlation_names.insert(model_name.to_ascii_lowercase()) {
            return Err(format!(
                "{context}.model_correlation contains duplicate case-insensitive model names"
            ));
        }
        if !library.models.contains_key(model_name) {
            return Err(format!(
                "{context}.model_correlation['{model_name}'] has no exact model projection"
            ));
        }
        correlation
            .validate_for_model(model_name)
            .map_err(|error| {
                format!("{context}.model_correlation['{model_name}'] is invalid: {error}")
            })?;
        let ModelSourceAuthority::ProjectOwned { source_id, .. } = library.source_authority else {
            return Err(format!(
                "{context}.model_correlation['{model_name}'] requires a project-owned source authority"
            ));
        };
        if correlation
            .suites
            .iter()
            .any(|suite| suite.source.source_id != Some(source_id))
            || correlation
                .evidence
                .iter()
                .any(|evidence| evidence.source.source_id != Some(source_id))
        {
            return Err(format!(
                "{context}.model_correlation['{model_name}'] contains evidence from a different project source identity"
            ));
        }
    }

    for (model_name, metadata) in &library.model_definition_metadata {
        if metadata.source_identity.is_none() && metadata.sections.is_empty() {
            continue;
        }
        let model = &library.models[model_name];
        let ModelSourceAuthority::ProjectOwned { source_id, .. } = library.source_authority else {
            return Err(format!(
                "{context}.model_definition_metadata['{model_name}'] source identity requires a project-owned source authority"
            ));
        };
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata.clone(),
        );
        let expected_digest = definition.expected_source_digest().map_err(|error| {
            format!(
                "{context}.model_definition_metadata['{model_name}'] cannot produce an authoritative model revision: {error}"
            )
        })?;
        let identity = definition
            .project_source_identity()
            .map_err(|error| {
                format!(
                    "{context}.model_definition_metadata['{model_name}'] has an invalid model source identity: {error}"
                )
            })?
            .ok_or_else(|| {
                format!(
                    "{context}.model_definition_metadata['{model_name}'] has no canonical source identity"
                )
            })?;
        if identity.source_id != source_id {
            return Err(format!(
                "{context}.model_definition_metadata['{model_name}'] is bound to a different project source identity"
            ));
        }
        if identity.content_digest != expected_digest {
            return Err(format!(
                "{context}.model_definition_metadata['{model_name}'] is not bound to the exact canonical model digest"
            ));
        }
        let canonical_source = definition.canonical_source().map_err(|error| {
            format!(
                "{context}.model_definition_metadata['{model_name}'] canonical source is invalid: {error}"
            )
        })?;
        let model_path = model.file_path.as_ref().ok_or_else(|| {
            format!(
                "{context}.models['{model_name}'] has no authenticated source member for its canonical definition"
            )
        })?;
        let retained_source = library
            .source_contents
            .iter()
            .find(|content| content.path == *model_path)
            .ok_or_else(|| {
                format!(
                    "{context}.models['{model_name}'] canonical definition points outside the retained source closure"
                )
            })?;
        let occurrences = retained_source
            .bytes
            .windows(canonical_source.len())
            .filter(|bytes| *bytes == canonical_source.as_bytes())
            .count();
        if occurrences != 1 {
            return Err(format!(
                "{context}.models['{model_name}'] canonical definition must occur exactly once in authenticated source '{}' (found {occurrences})",
                model_path.display()
            ));
        }

        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            model_name,
            identity.source_id,
            identity.content_digest,
            identity.revision,
        )
        .map_err(|error| {
            format!(
                "{context}.model_definition_metadata['{model_name}'] has an invalid evidence source binding: {error}"
            )
        })?;
        for section in &metadata.sections {
            let ModelSectionQualification::Qualified { evidence_digest } = &section.qualification
            else {
                continue;
            };
            let qualification = library.model_qualification.get(model_name).ok_or_else(|| {
                format!(
                    "{context}.model_definition_metadata['{model_name}'].sections['{}'] claims qualified evidence without a retained qualification record",
                    section.name
                )
            })?;
            let evidence_digest = evidence_digest
                .as_deref()
                .ok_or_else(|| {
                    format!(
                        "{context}.model_definition_metadata['{model_name}'].sections['{}'] is qualified without an evidence digest",
                        section.name
                    )
                })?
                .parse::<ContentDigest>()
                .map_err(|error| {
                    format!(
                        "{context}.model_definition_metadata['{model_name}'].sections['{}'] has an invalid evidence digest: {error}",
                        section.name
                    )
                })?;
            qualification
                .validate_exact_section_evidence_digest(
                    &source,
                    &section.name,
                    evidence_digest,
                )
                .map_err(|error| {
                    format!(
                        "{context}.model_definition_metadata['{model_name}'].sections['{}'] does not resolve to passing evidence for the exact canonical model source: {error}",
                        section.name
                    )
                })?;
        }
    }
    Ok(())
}

fn validate_model_numbers(
    context: &str,
    model_key: &str,
    model: &DeviceModel,
) -> Result<(), String> {
    if let Some(spice_type) = &model.spice_type
        && (spice_type.trim().is_empty() || spice_type.chars().any(char::is_control))
    {
        return Err(format!(
            "{context}.models['{model_key}'].spice_type must be a non-empty source token"
        ));
    }
    if model.model_version.is_some_and(|value| !value.is_finite()) {
        return Err(format!(
            "{context}.models['{model_key}'].model_version must be finite"
        ));
    }
    if model.source_line == Some(0) {
        return Err(format!(
            "{context}.models['{model_key}'].source_line must be one-based"
        ));
    }
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
    let mut parameter_names = HashSet::new();
    for parameter in model.parameters.keys() {
        if parameter.trim().is_empty() || !parameter_names.insert(parameter.to_ascii_lowercase()) {
            return Err(format!(
                "{context}.models['{model_key}'] contains an empty or case-duplicate parameter name '{parameter}'"
            ));
        }
    }
    for (parameter, value) in &model.string_parameters {
        if parameter.trim().is_empty()
            || !parameter_names.insert(parameter.to_ascii_lowercase())
            || value.is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(format!(
                "{context}.models['{model_key}'].string_parameters['{parameter}'] is empty, duplicates another parameter, or contains control characters"
            ));
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn model_source_warnings(libraries: &[ProjectModelLibrary]) -> Vec<String> {
    let mut warnings = Vec::new();
    for library in libraries {
        if library.source_authority != ModelSourceAuthority::External {
            continue;
        }
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
        .filter(|library| library.source_authority == ModelSourceAuthority::External)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind, AnalysisLifecycleState};
    use crate::state::model_library::{
        CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSuite,
    };

    fn project_id() -> ProjectId {
        ProjectId::from_namespace(
            uuid::Uuid::from_u128(0xe707_36ed_7eef_5205_b51e_9608_f55e_bd35),
            b"project-execution-tests",
        )
    }

    fn context_from_state(
        plan: &SimSetupState,
        manager: &ModelLibraryManager,
    ) -> Result<ProjectExecutionContext, String> {
        ProjectExecutionContext::from_state(project_id(), plan, manager)
    }

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
        let context =
            context_from_state(&plan, &ModelLibraryManager::new()).expect("valid context");

        let value = serde_json::to_value(context).expect("serialize context");
        let plan = &value["simulation_plan"];
        assert!(plan.get("options_open").is_none());
        assert!(plan.get("options_errors").is_none());
        assert!(plan.get("options_draft").is_none());
        assert!(plan.get("palette_open").is_none());
        assert!(plan.get("palette_query").is_none());
        for retired in [
            "enabled",
            "analysis_order",
            "listed",
            "tran",
            "ac",
            "dc",
            "noise",
            "op",
            "pss",
            "disto_f2_over_f1",
        ] {
            assert!(
                plan.get(retired).is_none(),
                "current schema must omit retired singleton field {retired}"
            );
        }
        assert!(plan.get("analysis_plan").is_some());
    }

    #[test]
    fn current_schema_rejects_every_retired_singleton_analysis_field() {
        let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
            .expect("baseline context validates");
        let baseline = serde_json::to_value(context).expect("context serializes");

        for field in RETIRED_SINGLETON_ANALYSIS_FIELDS {
            let mut value = baseline.clone();
            value["simulation_plan"]
                .as_object_mut()
                .expect("simulation plan is an object")
                .insert((*field).to_owned(), serde_json::Value::Null);
            let error = serde_json::from_value::<ProjectExecutionContext>(value)
                .expect_err("current schema must reject retired singleton input")
                .to_string();
            assert!(
                error.contains(&format!("retired singleton field `{field}`")),
                "{error}"
            );
        }
    }

    #[test]
    fn schema_three_still_accepts_singletons_only_for_load_time_migration() {
        let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
            .expect("baseline context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        value["schema_version"] = serde_json::json!(SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION);
        let persisted_plan = value["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan is an object");
        persisted_plan.remove("analysis_plan");
        persisted_plan.insert("enabled".to_owned(), serde_json::json!([1]));
        persisted_plan.insert("analysis_order".to_owned(), serde_json::json!([1]));
        persisted_plan.insert("listed".to_owned(), serde_json::json!([1]));
        persisted_plan.insert(
            "tran".to_owned(),
            serde_json::to_value(crate::workbench::app_state::TranSetup::default())
                .expect("legacy draft serializes"),
        );

        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("schema 3 accepts its legacy fields");
        restored
            .migrate_to_current(project_id())
            .expect("schema 3 migrates at the load boundary");

        assert_eq!(
            restored.schema_version,
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
        );
        assert!(restored.simulation_plan.stable_analysis_plan().is_ok());
        restored.validate().expect("migrated context validates");
    }

    #[test]
    fn schema_four_promotes_the_single_stable_plan_into_the_named_catalog() {
        let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
            .expect("baseline context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        value["schema_version"] = serde_json::json!(STABLE_ANALYSIS_PLAN_SCHEMA_VERSION);
        let persisted_plan = value["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan is an object");
        persisted_plan.remove("active_plan_name");
        persisted_plan.remove("active_plan_lineage");
        persisted_plan.remove("inactive_plans");
        persisted_plan["analysis_plan"]
            .as_object_mut()
            .expect("stable analysis plan is an object")
            .remove("configuration_receipts");

        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("schema 4 remains readable");
        restored
            .migrate_to_current(project_id())
            .expect("schema 4 promotes deterministically");

        assert_eq!(
            restored.schema_version,
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
        );
        assert_eq!(
            restored.simulation_plan.active_plan_name().as_str(),
            "Lab characterization"
        );
        assert_eq!(restored.simulation_plan.plan_count(), 1);
        assert_eq!(
            restored.simulation_plan.active_plan_lineage(),
            crate::workbench::app_state::SimulationPlanLineage::root()
        );
        restored.validate().expect("promoted context validates");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn schema_six_classifies_legacy_sources_without_inventing_edit_authority() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load external source");
        manager.add_library(ModelLibrary::new("built-in-catalog"));
        let context =
            context_from_state(&SimSetupState::new(), &manager).expect("context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        value["schema_version"] = serde_json::json!(RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION);
        for library in value["model_libraries"]
            .as_array_mut()
            .expect("libraries array")
        {
            library
                .as_object_mut()
                .expect("library object")
                .remove("source_authority");
        }

        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("schema six remains readable");
        restored
            .migrate_to_current(project_id())
            .expect("schema six migrates");
        restored.validate().expect("migrated context validates");
        assert!(restored.model_libraries.iter().any(|library| {
            library.name == "foundry" && library.source_authority == ModelSourceAuthority::External
        }));
        assert!(restored.model_libraries.iter().any(|library| {
            library.name == "built-in-catalog"
                && library.source_authority == ModelSourceAuthority::BuiltIn
        }));

        std::fs::remove_dir_all(directory).expect("remove model fixture");
    }

    #[test]
    fn schema_seven_migrates_without_inventing_model_authoring_records() {
        let mut manager = ModelLibraryManager::new();
        manager.add_library(ModelLibrary::new("legacy-catalog"));
        let context = context_from_state(&SimSetupState::new(), &manager)
            .expect("baseline context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        value["schema_version"] = serde_json::json!(MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION);
        for library in value["model_libraries"]
            .as_array_mut()
            .expect("libraries are an array")
        {
            library
                .as_object_mut()
                .expect("library is an object")
                .remove("model_definition_metadata");
            library
                .as_object_mut()
                .expect("library is an object")
                .remove("model_qualification");
            library
                .as_object_mut()
                .expect("library is an object")
                .remove("model_correlation");
        }

        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("schema seven remains readable");
        restored
            .migrate_to_current(project_id())
            .expect("schema seven migrates");

        assert_eq!(
            restored.schema_version,
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
        );
        assert!(restored.model_libraries.iter().all(|library| {
            library.model_definition_metadata.is_empty()
                && library.model_qualification.is_empty()
                && library.model_correlation.is_empty()
        }));
        restored.validate().expect("migrated context validates");
    }

    #[test]
    fn schema_eight_migrates_without_inventing_correlation_records() {
        let mut manager = ModelLibraryManager::new();
        manager.add_library(ModelLibrary::new("legacy-qualified-catalog"));
        let context = context_from_state(&SimSetupState::new(), &manager)
            .expect("baseline context validates");
        let mut value = serde_json::to_value(context).expect("context serializes");
        value["schema_version"] = serde_json::json!(MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION);
        for library in value["model_libraries"]
            .as_array_mut()
            .expect("libraries are an array")
        {
            library
                .as_object_mut()
                .expect("library is an object")
                .remove("model_correlation");
        }

        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("schema eight remains readable");
        restored
            .migrate_to_current(project_id())
            .expect("schema eight migrates");

        assert_eq!(
            restored.schema_version,
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
        );
        assert!(
            restored
                .model_libraries
                .iter()
                .all(|library| library.model_correlation.is_empty())
        );
        restored.validate().expect("migrated context validates");
    }

    #[test]
    fn project_owned_model_round_trip_preserves_authority_bytes_and_revision() {
        let definition = crate::state::model_library::ProjectModelDefinition {
            name: "owned_nch".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Persisted project model".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("kp".to_owned(), 0.001),
            ]),
            string_parameters: std::collections::BTreeMap::from([(
                "revision_tag".to_owned(),
                "r1".to_owned(),
            )]),
        };
        let mut manager = ModelLibraryManager::new();
        let committed = manager
            .create_project_model("owned-models", &definition)
            .expect("create project model");
        let expected_bytes = committed.after.source_contents[0].bytes.clone();
        let expected_authority = committed.after.source_authority;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: _,
            ..
        } = expected_authority
        else {
            panic!("project model must be source-bound");
        };
        let model = &committed.after.models["owned_nch"];
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            committed.after.model_definition_metadata["owned_nch"].clone(),
        );
        let canonical = definition.canonical_source().unwrap();
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_revision = definition
            .project_source_identity()
            .unwrap()
            .expect("project source identity")
            .revision;
        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            "owned_nch",
            source_id,
            model_digest,
            model_revision,
        )
        .unwrap();
        let reference = CorrelationDatasetRevision::try_from_csv(
            "bench-reference",
            crate::product::ObjectRevision::INITIAL,
            "Bench reference",
            CorrelationDatasetClass::BenchMeasurement,
            "test lab",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "bench.csv",
            b"id,quantity,value,unit\nr1,gain,1,V\n".to_vec(),
            None,
        )
        .unwrap();
        let suite = CorrelationSuite::try_new(
            "owned-nch-correlation",
            crate::product::ObjectRevision::INITIAL,
            "Owned NCH correlation",
            "model-owner",
            source,
            vec![reference],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let expected_correlation = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
        manager
            .get_library_mut("owned-models")
            .unwrap()
            .model_correlation
            .insert("owned_nch".to_owned(), expected_correlation.clone());
        let context =
            context_from_state(&SimSetupState::new(), &manager).expect("context validates");
        let mut foreign_evidence = context.clone();
        foreign_evidence.model_libraries[0]
            .model_correlation
            .get_mut("owned_nch")
            .unwrap()
            .suites[0]
            .source
            .source_id = Some(crate::product::ModelSourceId::new());
        let error = foreign_evidence
            .validate()
            .expect_err("foreign project source identity must be rejected");
        assert!(
            error.contains("different project source identity"),
            "{error}"
        );
        let json = serde_json::to_string(&context).expect("context serializes");
        let restored: ProjectExecutionContext =
            serde_json::from_str(&json).expect("context deserializes");
        let (_, restored_manager, warnings) = restored
            .into_state(project_id())
            .expect("project-owned model restores");

        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(
            restored_manager
                .get_library("owned-models")
                .unwrap()
                .model_correlation
                .get("owned_nch"),
            Some(&expected_correlation)
        );
        let library = restored_manager
            .get_library("owned-models")
            .expect("library restored");
        assert_eq!(library.source_authority, expected_authority);
        assert_eq!(library.source_contents[0].bytes, expected_bytes);
        let metadata = library
            .model_definition_metadata
            .get("owned_nch")
            .expect("typed model metadata restored");
        assert_eq!(metadata.parameters.len(), 3);
        metadata.validate().expect("restored metadata validates");
        restored_manager
            .seal_execution_sources()
            .expect("desktop execution consumes retained project bytes");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_owned_multifile_closure_restores_distinct_member_identities() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load authenticated multi-file fixture");
        let source_id = crate::product::ModelSourceId::new();
        let library = manager
            .get_library_mut("foundry")
            .expect("fixture library exists");
        let root = library.root_path.clone().expect("fixture has a root");
        let root_digest = library
            .source_closure
            .iter()
            .find(|source| source.path == root)
            .expect("root is pinned")
            .digest;
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: crate::product::ObjectRevision::INITIAL,
            digest: root_digest,
        };
        let context = context_from_state(&SimSetupState::new(), &manager)
            .expect("multi-file project-owned closure validates");
        std::fs::remove_dir_all(directory).expect("retained restore must not need fixture files");

        let (_, restored, warnings) = context
            .into_state(project_id())
            .expect("multi-file project-owned closure restores from retained bytes");
        assert!(warnings.is_empty(), "{warnings:?}");
        let library = restored.get_library("foundry").expect("library restores");
        assert_eq!(library.source_closure.len(), 2);
        assert_eq!(library.source_contents.len(), 2);
        assert_eq!(library.source_edges.len(), 1);
        let restored_root = library.root_path.as_ref().expect("root restores");
        assert_eq!(&library.source_edges[0].owner, restored_root);
        assert_ne!(library.source_edges[0].target, *restored_root);
        assert_eq!(
            library.models["helper"].file_path.as_ref(),
            Some(&library.source_edges[0].target)
        );
        assert_eq!(library.models["helper"].source_line, Some(1));
        restored
            .seal_execution_sources()
            .expect("execution seals the complete retained project closure");
    }

    #[test]
    fn project_model_identity_is_bound_to_its_member_digest_and_independent_revision() {
        let definition = ProjectModelDefinition {
            name: "member_nch".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Included canonical model".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("kp".to_owned(), 0.001),
            ]),
            string_parameters: std::collections::BTreeMap::new(),
        };
        let mut manager = ModelLibraryManager::new();
        let base = manager
            .create_project_model("base-model", &definition)
            .expect("base metadata is synthesized");
        let mut metadata = base.after.model_definition_metadata["member_nch"].clone();
        metadata
            .sections
            .push(crate::state::model_library::ModelSectionDefinition {
                name: "TT".to_owned(),
                parent: None,
                overrides: std::collections::BTreeMap::new(),
                model_files: Vec::new(),
                qualification: ModelSectionQualification::Unqualified,
            });
        let revision = ProjectModelRevisionDefinition::new(definition, metadata);
        manager
            .create_project_model_revision(
                "sectioned-model",
                &revision,
                &ModelQualificationState::default(),
            )
            .expect("sectioned source is published");
        let library = manager
            .get_library_mut("sectioned-model")
            .expect("sectioned library exists");
        let root = library
            .root_path
            .clone()
            .expect("project source has a root");
        let member = root.with_file_name("model-member.lib");
        let member_bytes = library.source_contents[0].bytes.clone();
        let member_digest = ContentDigest::from_bytes(Sha256::digest(&member_bytes).into());
        let root_bytes = b".include \"model-member.lib\"\n".to_vec();
        let root_digest = ContentDigest::from_bytes(Sha256::digest(&root_bytes).into());
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: library_revision,
            ..
        } = library.source_authority
        else {
            panic!("fixture is project-owned");
        };
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: library_revision,
            digest: root_digest,
        };
        library.source_closure = vec![
            ModelSourcePin {
                path: root.clone(),
                digest: root_digest,
            },
            ModelSourcePin {
                path: member.clone(),
                digest: member_digest,
            },
        ];
        library
            .source_closure
            .sort_by(|left, right| left.path.cmp(&right.path));
        library.source_contents = vec![
            ModelSourceContent {
                path: root.clone(),
                bytes: root_bytes,
            },
            ModelSourceContent {
                path: member.clone(),
                bytes: member_bytes,
            },
        ];
        library
            .source_contents
            .sort_by(|left, right| left.path.cmp(&right.path));
        library.source_edges = vec![ModelSourceEdge {
            owner: root,
            requested_path: "model-member.lib".to_owned(),
            target: member.clone(),
        }];
        library
            .models
            .get_mut("member_nch")
            .expect("model projection exists")
            .file_path = Some(member.clone());
        for corner in library.corners.values_mut() {
            corner.file_path = Some(member.clone());
        }
        let model_revision = crate::product::ObjectRevision::new(7).expect("fixture revision");
        let metadata = library
            .model_definition_metadata
            .get_mut("member_nch")
            .expect("typed metadata exists");
        metadata
            .source_identity
            .as_mut()
            .expect("base model identity is retained")
            .revision = model_revision.get();
        for section in &mut metadata.sections {
            section.model_files[0].revision = model_revision.get();
        }

        let context = context_from_state(&SimSetupState::new(), &manager).expect(
            "model identity may use its canonical member digest and revision independently of the library root",
        );
        let persisted = context
            .model_libraries
            .iter()
            .find(|library| library.name == "sectioned-model")
            .expect("sectioned model persists");
        let ModelSourceAuthority::ProjectOwned {
            revision: persisted_library_revision,
            digest: persisted_root_digest,
            ..
        } = persisted.source_authority
        else {
            panic!("persisted fixture is project-owned");
        };
        let identity = persisted.model_definition_metadata["member_nch"]
            .source_identity
            .as_ref()
            .expect("model identity persists");
        assert_eq!(identity.revision, model_revision.get());
        assert_ne!(identity.revision, persisted_library_revision.get());
        assert_eq!(identity.content_digest, member_digest.to_string());
        assert_ne!(member_digest, persisted_root_digest);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_owned_load_rejects_a_tampered_serialized_model_projection() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load authenticated multi-file fixture");
        let source_id = crate::product::ModelSourceId::new();
        let library = manager
            .get_library_mut("foundry")
            .expect("fixture library exists");
        let root = library.root_path.clone().expect("fixture has a root");
        let root_digest = library
            .source_closure
            .iter()
            .find(|source| source.path == root)
            .expect("root is pinned")
            .digest;
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: crate::product::ObjectRevision::INITIAL,
            digest: root_digest,
        };
        let mut context = context_from_state(&SimSetupState::new(), &manager)
            .expect("untampered projection validates");
        std::fs::remove_dir_all(directory).expect("remove model fixture");
        context.model_libraries[0]
            .models
            .get_mut("helper")
            .expect("helper projection exists")
            .parameters
            .insert("kp".to_owned(), 0.75);

        let error = context
            .validate()
            .expect_err("serialized projection cannot diverge from retained model cards");
        assert!(error.contains("not an exact projection"), "{error}");
    }

    #[test]
    fn project_load_rejects_qualified_section_without_exact_retained_evidence() {
        let definition = crate::state::model_library::ProjectModelDefinition {
            name: "owned_nch".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Persisted project model".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("kp".to_owned(), 0.001),
            ]),
            string_parameters: std::collections::BTreeMap::new(),
        };
        let mut manager = ModelLibraryManager::new();
        manager
            .create_project_model("owned-models", &definition)
            .expect("create project model");
        let mut context =
            context_from_state(&SimSetupState::new(), &manager).expect("base context validates");
        let library = context
            .model_libraries
            .iter_mut()
            .find(|library| library.name == "owned-models")
            .expect("project model library persists");
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest,
        } = library.source_authority
        else {
            panic!("fixture source is project-owned");
        };
        library
            .model_definition_metadata
            .get_mut("owned_nch")
            .expect("fixture metadata")
            .sections
            .push(crate::state::model_library::ModelSectionDefinition {
                name: "TT".to_owned(),
                parent: None,
                overrides: std::collections::BTreeMap::new(),
                model_files: vec![crate::state::model_library::ModelFileIdentity {
                    source_id: source_id.to_string(),
                    revision: revision.get(),
                    content_digest: digest.to_string(),
                    display_name: "definition.model".to_owned(),
                }],
                qualification: ModelSectionQualification::Qualified {
                    evidence_digest: Some("0".repeat(64)),
                },
            });
        let bound = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(&library.models["owned_nch"]),
            library.model_definition_metadata["owned_nch"].clone(),
        )
        .bind_project_source_identity(source_id, revision, "definition.model")
        .expect("fixture section identity binds to its canonical model digest");
        let canonical_source = bound
            .canonical_source()
            .expect("fixture canonical source renders")
            .into_bytes();
        let identity = bound
            .project_source_identity()
            .expect("fixture source identity validates")
            .expect("sectioned fixture has a source identity");
        library
            .model_definition_metadata
            .insert("owned_nch".to_owned(), bound.metadata);
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: identity.content_digest,
        };
        library.source_closure[0].digest = identity.content_digest;
        library.source_contents[0].bytes = canonical_source;

        let error = context
            .validate()
            .expect_err("a qualified section cannot invent its evidence digest");
        assert!(
            error.contains("claims qualified evidence without a retained qualification record"),
            "{error}"
        );
    }

    #[test]
    fn current_state_save_fails_closed_without_a_stable_plan() {
        let mut plan = SimSetupState::new();
        plan.analysis_plan = None;

        let error = context_from_state(&plan, &ModelLibraryManager::new())
            .expect_err("current-state persistence must never invoke legacy migration");

        assert!(error.contains("legacy singleton migration is load-only"));
        assert!(plan.analysis_plan.is_none());
    }

    #[test]
    fn restored_execution_lifecycle_never_retains_runner_authority() {
        let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
            .expect("baseline context validates");
        let baseline = serde_json::to_value(context).expect("context serializes");

        for lifecycle in ["queued", "running", "paused"] {
            let mut value = baseline.clone();
            value["simulation_plan"]["analysis_plan"]["instances"][0]["lifecycle"] =
                serde_json::json!(lifecycle);
            let mut restored: ProjectExecutionContext =
                serde_json::from_value(value).expect("persisted lifecycle deserializes");
            restored
                .migrate_to_current(project_id())
                .expect("current context restores");
            let instance = &restored
                .simulation_plan
                .stable_analysis_plan()
                .expect("stable plan restored")
                .instances()[0];
            let id = instance.id();
            assert_eq!(instance.lifecycle(), AnalysisLifecycleState::Draft);
            restored.validate().expect("normalized context validates");

            let (mut setup, _, _) = restored
                .into_state(project_id())
                .expect("normalized context enters application state");
            setup
                .stable_analysis_plan_mut()
                .expect("stable plan remains present")
                .edit(id, |_| ())
                .expect("stale runner lifecycle cannot lock the restored draft");
        }
    }

    #[test]
    fn incomplete_disabled_and_enabled_analysis_drafts_round_trip_losslessly() {
        let mut plan = SimSetupState::new();
        let stable = plan
            .analysis_plan
            .as_mut()
            .expect("current setup owns a stable plan");
        let transient_id = stable.instances()[0].id();
        stable
            .edit(transient_id, |draft| {
                let AnalysisDraft::Transient(transient) = draft else {
                    panic!("default instance must be transient");
                };
                transient.stop = "also unfinished".to_owned();
            })
            .expect("transient draft edit commits");
        let (pss_id, _) = stable.insert(AnalysisKind::Pss).expect("PSS inserts");
        stable
            .edit(pss_id, |draft| {
                let AnalysisDraft::Pss(pss) = draft else {
                    panic!("inserted instance must be PSS");
                };
                pss.fund_freq = "unfinished-expression(".to_owned();
            })
            .expect("PSS draft edit commits");
        // PSS proves a disabled draft is retained; Transient proves an enabled
        // invalid draft is persistable but remains blocked by run validation.
        stable
            .set_enabled(pss_id, false)
            .expect("PSS disables without losing its position");

        let context = context_from_state(&plan, &ModelLibraryManager::new())
            .expect("draft validity is a run concern, not a persistence concern");
        let serialized = serde_json::to_string(&context).expect("context serializes");
        let restored: ProjectExecutionContext =
            serde_json::from_str(&serialized).expect("context deserializes");
        let (restored, _, _) = restored.into_state(project_id()).expect("context restores");
        let restored = restored
            .stable_analysis_plan()
            .expect("v4 restores a stable plan");
        assert_eq!(
            restored
                .instances()
                .iter()
                .map(|instance| instance.id())
                .collect::<Vec<_>>(),
            vec![transient_id, pss_id]
        );
        let transient = restored.instance(transient_id).expect("transient retained");
        let AnalysisDraft::Transient(transient_draft) = transient.draft() else {
            panic!("transient identity must retain its kind");
        };
        assert_eq!(transient_draft.stop, "also unfinished");
        assert!(transient.enabled());
        let pss = restored.instance(pss_id).expect("PSS retained");
        let AnalysisDraft::Pss(pss_draft) = pss.draft() else {
            panic!("PSS identity must retain its kind");
        };
        assert_eq!(pss_draft.fund_freq, "unfinished-expression(");
        assert!(!pss.enabled());
        assert!(pss.dependencies().is_empty());
    }

    #[test]
    fn duplicate_and_unknown_stable_analysis_data_fail_precisely() {
        let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
            .expect("baseline context validates");
        let mut duplicate_value = serde_json::to_value(&context).expect("context serializes");
        let instances = duplicate_value["simulation_plan"]["analysis_plan"]["instances"]
            .as_array_mut()
            .expect("v4 instances are an array");
        let duplicate_instance = instances[0].clone();
        instances.push(duplicate_instance);
        let duplicate: ProjectExecutionContext =
            serde_json::from_value(duplicate_value).expect("shape deserializes before validation");
        let error = duplicate
            .validate()
            .expect_err("duplicate identity must fail");
        assert!(error.contains("appears more than once"));

        let mut unknown_value = serde_json::to_value(context).expect("context serializes");
        unknown_value["simulation_plan"]["analysis_plan"]["instances"][0]["kind"] =
            serde_json::Value::String("future-analysis".to_owned());
        let error = serde_json::from_value::<ProjectExecutionContext>(unknown_value)
            .expect_err("unknown stable analysis kind must fail closed")
            .to_string();
        assert!(error.contains("unknown variant `future-analysis`"));
    }

    #[test]
    fn legacy_context_migrates_to_sorted_execution_order() {
        let mut plan = SimSetupState::new();
        plan.ensure_initialized();
        plan.analysis_plan = None;
        plan.enabled.extend([4, 0]);
        plan.analysis_order.clear();
        let mut context = ProjectExecutionContext {
            schema_version: LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan: plan,
            model_libraries: Vec::new(),
        };

        context
            .migrate_to_current(project_id())
            .expect("legacy migration");

        let migrated = context
            .simulation_plan
            .stable_analysis_plan()
            .expect("legacy context migrates to stable identity");
        assert_eq!(migrated.instances().len(), AnalysisKind::ALL.len());
        assert_eq!(
            migrated
                .instances()
                .iter()
                .take(3)
                .map(|instance| (instance.kind(), instance.enabled()))
                .collect::<Vec<_>>(),
            vec![
                (AnalysisKind::OperatingPoint, true),
                (AnalysisKind::Transient, true),
                (AnalysisKind::Noise, true),
            ]
        );
        let noise = migrated
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Noise)
            .expect("noise migrated");
        let op = migrated
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::OperatingPoint)
            .expect("OP migrated");
        assert_eq!(noise.dependencies().len(), 1);
        assert_eq!(noise.dependencies()[0].target(), op.id());
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
        let context =
            context_from_state(&SimSetupState::new(), &manager).expect("context validates");
        let canonical_root = context.model_libraries[0]
            .root_path
            .clone()
            .expect("external library keeps canonical root");
        let json = serde_json::to_string(&context).expect("context serializes");
        let restored_context: ProjectExecutionContext =
            serde_json::from_str(&json).expect("context deserializes");

        let (_, restored_manager, warnings) = restored_context
            .into_state(project_id())
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
        let mut context =
            context_from_state(&SimSetupState::new(), &manager).expect("current context validates");
        context.schema_version = UNPINNED_MODEL_SOURCE_SCHEMA_VERSION;
        context.model_libraries[0].source_closure.clear();

        let (_, restored, warnings) = context
            .into_state(project_id())
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
        let mut context =
            context_from_state(&SimSetupState::new(), &manager).expect("current context validates");
        context.schema_version = PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION;
        context.model_libraries[0].source_edges.clear();

        let (_, restored, warnings) = context
            .into_state(project_id())
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
        let context =
            context_from_state(&SimSetupState::new(), &manager).expect("context validates");
        let canonical_root = context.model_libraries[0]
            .root_path
            .clone()
            .expect("external library keeps canonical root");

        std::fs::remove_file(&path).expect("remove source");
        let (_, retained, warnings) = context
            .clone()
            .into_state(project_id())
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
            .into_state(project_id())
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
                source_authority: ModelSourceAuthority::External,
                source_closure: vec![ModelSourcePin {
                    path: root.clone(),
                    digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
                }],
                source_contents: Vec::new(),
                source_edges: Vec::new(),
                models: HashMap::new(),
                model_definition_metadata: HashMap::new(),
                model_qualification: HashMap::new(),
                model_correlation: HashMap::new(),
                corners: HashMap::new(),
                selected_corner: None,
                version: String::new(),
            }],
        };

        context
            .validate()
            .expect("foreign desktop syntax remains valid project metadata");
        let (_, manager, warnings) = context
            .into_state(project_id())
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
                source_authority: ModelSourceAuthority::External,
                source_closure,
                source_contents: Vec::new(),
                source_edges,
                models: HashMap::new(),
                model_definition_metadata: HashMap::new(),
                model_qualification: HashMap::new(),
                model_correlation: HashMap::new(),
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
