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

use crate::product::{ContentDigest, ProjectId};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::model_library::is_foreign_platform_absolute_path;
use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelDefinitionMetadata, ModelLibrary, ModelLibraryManager,
    ModelQualificationState, ModelResolutionRecord, ModelSectionQualification,
    ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourceEvidenceBinding,
    ModelSourcePin, ModelSubcircuitInterface, ModelValidationReceipt, ParameterDataType,
    ParameterValue, ProcessCorner as LibraryProcessCorner, ProjectModelDefinition,
    ProjectModelRevisionDefinition, first_unreachable_source, is_portable_absolute_path,
    subcircuit_interface_key,
};
use crate::workbench::app_state::SimSetupState;

pub const PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 18;
const LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION: u32 = 0;
const UNPINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 1;
const PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION: u32 = 2;
const SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION: u32 = 3;
const STABLE_ANALYSIS_PLAN_SCHEMA_VERSION: u32 = 4;
const PLAN_CATALOG_SCHEMA_VERSION: u32 = 5;
const RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION: u32 = 6;
const MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION: u32 = 7;
const MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION: u32 = 8;
const MODEL_MEASUREMENT_CORRELATION_SCHEMA_VERSION: u32 = 9;
const MODEL_BIN_AUDIT_SCHEMA_VERSION: u32 = 10;
const TYPED_CORNER_DOMAIN_SCHEMA_VERSION: u32 = 11;
const RETAINED_IMPORTED_SOURCE_AUTHORITY_SCHEMA_VERSION: u32 = 12;
const EXPLICIT_MODEL_DEFINITION_RESOLUTION_SCHEMA_VERSION: u32 = 13;
const RETAINED_SUBCIRCUIT_INTERFACE_SCHEMA_VERSION: u32 = 14;
const AUTHENTICATED_MODEL_SECTION_SCHEMA_VERSION: u32 = 15;
const SOURCE_QUALIFIED_MODEL_RESOLUTION_SCHEMA_VERSION: u32 = 16;
const EXPLICIT_SIMULATION_PLAN_MODEL_BINDINGS_SCHEMA_VERSION: u32 = 17;
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
    pub model_resolution_records: Vec<ModelResolutionRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_validation_receipt: Option<ModelValidationReceipt>,
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
            #[serde(default)]
            model_resolution_records: Vec<ModelResolutionRecord>,
            #[serde(default)]
            model_validation_receipt: Option<ModelValidationReceipt>,
            /// Retired with the local Models workspace. The field is still
            /// accepted, and discarded, so a project written by a build that
            /// carried the ledger still loads under `deny_unknown_fields`.
            /// Nothing writes it back.
            #[serde(default)]
            #[allow(dead_code)]
            model_bin_audit_receipts: serde::de::IgnoredAny,
            /// Retired alongside `model_bin_audit_receipts`.
            #[serde(default)]
            #[allow(dead_code)]
            model_definition_resolutions: serde::de::IgnoredAny,
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
            model_resolution_records: persisted.model_resolution_records,
            model_validation_receipt: persisted.model_validation_receipt,
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
    #[serde(default)]
    pub pack_id: Option<String>,
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
    pub top_level_models: HashMap<String, DeviceModel>,
    #[serde(default)]
    pub section_models: HashMap<String, HashMap<String, DeviceModel>>,
    #[serde(default)]
    pub subcircuits: HashMap<String, ModelSubcircuitInterface>,
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

        let mut context = Self {
            schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            simulation_plan,
            model_libraries: model_libraries
                .libraries_sorted()
                .into_iter()
                .map(ProjectModelLibrary::from)
                .collect(),
            model_resolution_records: model_libraries.owned_model_resolution_records(),
            model_validation_receipt: model_libraries.model_validation_receipt().cloned(),
        };
        // The authenticated source closure is the persistence authority for
        // the complete section catalog. Rebuild these derived maps at the save
        // boundary so callers that legitimately retarget a canonical member
        // path cannot leave inactive-section copies stale in memory. The
        // active `models` projection is still validated independently and is
        // never repaired here, so executable-card tampering remains fatal.
        for (index, library) in context.model_libraries.iter_mut().enumerate() {
            if !library.source_contents.is_empty() {
                migrate_complete_model_section_catalog(library).map_err(|error| {
                    format!(
                        "model_libraries[{index}] complete model-section catalog projection failed before save: {error}"
                    )
                })?;
            }
        }
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
                self.schema_version = MODEL_MEASUREMENT_CORRELATION_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            MODEL_MEASUREMENT_CORRELATION_SCHEMA_VERSION => {
                // Schema 9 persisted measurement-correlation evidence but
                // predated durable geometry-bin audit receipts. The empty
                // serde default is the only honest migration; no audit is
                // inferred from model cards or prior execution artifacts.
                self.schema_version = MODEL_BIN_AUDIT_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            MODEL_BIN_AUDIT_SCHEMA_VERSION => {
                // Schema 10 persisted model-bin audit evidence but predated
                // typed corner-domain composition. Serde defaults preserve
                // the historic file_path + corner-name meaning; execution
                // resolves that pair as one required composite section.
                self.schema_version = TYPED_CORNER_DOMAIN_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            TYPED_CORNER_DOMAIN_SCHEMA_VERSION => {
                // Schema 11 introduced typed corner-domain composition but
                // predated retained imported-source authority. Existing
                // project-owned and external bindings retain their exact
                // authority; no installed-pack origin is inferred from paths.
                self.schema_version = RETAINED_IMPORTED_SOURCE_AUTHORITY_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            RETAINED_IMPORTED_SOURCE_AUTHORITY_SCHEMA_VERSION => {
                // Schema 12 introduced retained imported-source authority but
                // predated explicit contested-definition provider contracts.
                // An empty migration is the only safe choice: any overlap
                // remains blocked until a user selects an exact provider.
                self.schema_version = EXPLICIT_MODEL_DEFINITION_RESOLUTION_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            EXPLICIT_MODEL_DEFINITION_RESOLUTION_SCHEMA_VERSION => {
                // Schema 13 retained executable subcircuit source bytes but no
                // durable interface catalog. Reconstruct only from the exact
                // authenticated closure already inside the project; never
                // inspect a live path or infer terminals from a symbol.
                for (index, library) in self.model_libraries.iter_mut().enumerate() {
                    library.subcircuits =
                        authenticated_subcircuit_projection(library).map_err(|error| {
                            format!(
                                "model_libraries[{index}] subcircuit metadata migration failed: {error}"
                            )
                        })?;
                }
                self.schema_version = RETAINED_SUBCIRCUIT_INTERFACE_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            RETAINED_SUBCIRCUIT_INTERFACE_SCHEMA_VERSION => {
                // Schema 14 retained exact model-card values and source lines,
                // but discarded whether an active card came from the selected
                // `.lib` section or the top level. Recover that provenance only
                // by reparsing the authenticated bytes already in the project.
                // Source-less built-ins and legacy repair-only external
                // bindings honestly remain top-level/unknown.
                for (index, library) in self.model_libraries.iter_mut().enumerate() {
                    migrate_authenticated_model_sections(library).map_err(|error| {
                        format!(
                            "model_libraries[{index}] model-section provenance migration failed: {error}"
                        )
                    })?;
                }
                self.schema_version = AUTHENTICATED_MODEL_SECTION_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            AUTHENTICATED_MODEL_SECTION_SCHEMA_VERSION => {
                // Schema 15 predated source-qualified provider decisions. An
                // empty ledger is the only safe migration: existing overlaps
                // stay fail-closed until a user publishes an exact decision.
                self.model_resolution_records.clear();
                self.schema_version = SOURCE_QUALIFIED_MODEL_RESOLUTION_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            SOURCE_QUALIFIED_MODEL_RESOLUTION_SCHEMA_VERSION => {
                // Schema 16 kept the nominal model section and library order
                // in the project-global manager. Preserve that exact former
                // behavior explicitly in every plan; future changes then
                // diverge independently.
                let mut manager = ModelLibraryManager::new();
                for library in self.model_libraries.clone() {
                    manager.add_library(library.into_model_library());
                }
                let bindings = manager.default_simulation_plan_bindings();
                self.simulation_plan
                    .migrate_legacy_model_bindings(&bindings);
                self.schema_version = EXPLICIT_SIMULATION_PLAN_MODEL_BINDINGS_SCHEMA_VERSION;
                self.migrate_to_current(project_id)
            }
            EXPLICIT_SIMULATION_PLAN_MODEL_BINDINGS_SCHEMA_VERSION => {
                // Schema 17 persisted only the active top-level-plus-section
                // projection. Reconstruct the complete section catalog from
                // authenticated retained bytes so changing corners cannot keep
                // stale parameters from the formerly active section.
                for (index, library) in self.model_libraries.iter_mut().enumerate() {
                    migrate_complete_model_section_catalog(library).map_err(|error| {
                        format!(
                            "model_libraries[{index}] complete model-section catalog migration failed: {error}"
                        )
                    })?;
                }
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
        validate_model_libraries(&self.model_libraries)?;
        let mut manager = ModelLibraryManager::new();
        for library in self.model_libraries.clone() {
            manager.add_library(library.into_model_library());
        }
        manager.restore_model_resolution_records(self.model_resolution_records.clone())?;
        manager.restore_model_validation_receipt(self.model_validation_receipt.clone())?;
        validate_simulation_plan_model_bindings(&self.simulation_plan, &manager)
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
        manager.restore_model_resolution_records(self.model_resolution_records)?;
        manager.restore_model_validation_receipt(self.model_validation_receipt)?;
        self.simulation_plan.prepare_after_restore();
        Ok((self.simulation_plan, manager, warnings))
    }
}

fn validate_simulation_plan_model_bindings(
    plan: &SimSetupState,
    manager: &ModelLibraryManager,
) -> Result<(), String> {
    manager
        .validate_simulation_plan_bindings(&plan.model_bindings)
        .map_err(|error| format!("active simulation-plan model bindings are invalid: {error}"))?;
    for stored in plan.inactive_plans() {
        manager
            .validate_simulation_plan_bindings(stored.model_bindings())
            .map_err(|error| {
                format!(
                    "simulation-plan '{}' model bindings are invalid: {error}",
                    stored.name()
                )
            })?;
    }
    Ok(())
}

impl From<&ModelLibrary> for ProjectModelLibrary {
    fn from(library: &ModelLibrary) -> Self {
        Self {
            name: library.name.clone(),
            pdk_name: library.pdk_name.clone(),
            technology_node: library.technology_node.clone(),
            pack_id: library.pack_id.clone(),
            root_path: library.root_path.clone(),
            source_authority: library.source_authority,
            source_closure: library.source_closure.clone(),
            source_contents: library.source_contents.clone(),
            source_edges: library.source_edges.clone(),
            models: library.models.clone(),
            top_level_models: library.top_level_models.clone(),
            section_models: library.section_models.clone(),
            subcircuits: library.subcircuits.clone(),
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
            for model in self.top_level_models.values_mut().chain(
                self.section_models
                    .values_mut()
                    .flat_map(|models| models.values_mut()),
            ) {
                if let Some(path) = model.file_path.as_mut() {
                    *path = path_map
                        .get(path)
                        .expect("validated section model source has a restored path identity")
                        .clone();
                }
            }
            for subcircuit in self.subcircuits.values_mut() {
                if let Some(path) = subcircuit.file_path.as_mut() {
                    *path = path_map
                        .get(path)
                        .expect("validated subcircuit source has a restored path identity")
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
        let mut library = ModelLibrary {
            name: self.name,
            pdk_name: self.pdk_name,
            technology_node: self.technology_node,
            pack_id: self.pack_id,
            root_path: self.root_path,
            source_authority: self.source_authority,
            source_closure: self.source_closure,
            source_contents: self.source_contents,
            source_edges: self.source_edges,
            models: self.models,
            top_level_models: self.top_level_models,
            section_models: self.section_models,
            subcircuits: self.subcircuits,
            model_definition_metadata: self.model_definition_metadata,
            model_qualification: self.model_qualification,
            model_correlation: self.model_correlation,
            corners: self.corners,
            selected_corner: self.selected_corner,
            version: self.version,
            expanded: false,
        };
        library.refresh_effective_model_projection();
        library
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
    ];
    if let Some((field, value)) = positive_fields.into_iter().find(|(_, value)| *value <= 0.0) {
        return Err(format!(
            "simulation_plan.options.{field} must be positive, got {value}"
        ));
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

fn authenticated_subcircuit_projection(
    library: &ProjectModelLibrary,
) -> Result<HashMap<String, ModelSubcircuitInterface>, String> {
    if library.source_contents.is_empty() {
        return Ok(HashMap::new());
    }
    let root = library
        .root_path
        .as_ref()
        .ok_or_else(|| "retained source bytes have no root identity".to_owned())?;
    validate_retained_model_source_dialects(library, "retained source closure")?;
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
        .map_err(|error| format!("retained source closure cannot be authenticated: {error}"))?;
    if !parsed.is_ok() {
        return Err(format!(
            "retained source closure does not parse: {}",
            parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    parsed_subcircuit_projection(library, &parsed, root)
}

fn migrate_authenticated_model_sections(library: &mut ProjectModelLibrary) -> Result<(), String> {
    if library.source_contents.is_empty() {
        for model in library.models.values_mut() {
            model.section = None;
        }
        return Ok(());
    }

    let root = library
        .root_path
        .as_ref()
        .ok_or_else(|| "retained source bytes have no root identity".to_owned())?;
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
        .map_err(|error| format!("retained source closure cannot be authenticated: {error}"))?;
    if !parsed.is_ok() {
        return Err(format!(
            "retained source closure does not parse: {}",
            parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }

    let projection = parsed_model_projection(library, &parsed, root)?;
    if projection.len() != library.models.len() {
        return Err(
            "persisted model catalog does not exactly cover the authenticated active cards"
                .to_owned(),
        );
    }
    for (name, persisted) in &mut library.models {
        let candidate = projection.get(name).ok_or_else(|| {
            format!("persisted model '{name}' is absent from the authenticated active cards")
        })?;
        if !parsed_model_projection_matches_without_section(persisted, candidate) {
            return Err(format!(
                "persisted model '{name}' is not an exact projection of its authenticated card"
            ));
        }
        persisted.section.clone_from(&candidate.section);
    }
    Ok(())
}

fn migrate_complete_model_section_catalog(library: &mut ProjectModelLibrary) -> Result<(), String> {
    if library.source_contents.is_empty() {
        library.top_level_models.clear();
        library.section_models.clear();
        for model in library.models.values() {
            if let Some(section) = model.section.as_deref() {
                library
                    .section_models
                    .entry(section.to_owned())
                    .or_default()
                    .insert(model.name.clone(), model.clone());
            } else {
                library
                    .top_level_models
                    .insert(model.name.clone(), model.clone());
            }
        }
        return Ok(());
    }

    let root = library
        .root_path
        .as_ref()
        .ok_or_else(|| "retained source bytes have no root identity".to_owned())?;
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
        .map_err(|error| format!("retained source closure cannot be authenticated: {error}"))?;
    if !parsed.is_ok() {
        return Err(format!(
            "retained source closure does not parse: {}",
            parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    let (top_level_models, section_models) = parsed_complete_model_catalog(&parsed, root);
    library.top_level_models = top_level_models;
    library.section_models = section_models;
    Ok(())
}

fn parsed_complete_model_catalog(
    parsed: &rspice_core::library::LibParseResult,
    root: &std::path::Path,
) -> (
    HashMap<String, DeviceModel>,
    HashMap<String, HashMap<String, DeviceModel>>,
) {
    let top_level_models = parsed
        .top_level_models
        .iter()
        .map(|model| {
            let model = ModelLibraryManager::convert_parsed_model(model, root);
            (model.name.clone(), model)
        })
        .collect();
    let section_models = parsed
        .sections
        .iter()
        .map(|section| {
            let models = section
                .models
                .iter()
                .map(|model| {
                    let model = ModelLibraryManager::convert_parsed_model_in_section(
                        model,
                        root,
                        Some(&section.name),
                    );
                    (model.name.clone(), model)
                })
                .collect();
            (section.name.clone(), models)
        })
        .collect();
    (top_level_models, section_models)
}

fn parsed_model_projection(
    library: &ProjectModelLibrary,
    parsed: &rspice_core::library::LibParseResult,
    root: &std::path::Path,
) -> Result<HashMap<String, DeviceModel>, String> {
    let mut projection = HashMap::new();
    for model in &parsed.top_level_models {
        let model = ModelLibraryManager::convert_parsed_model(model, root);
        projection.insert(model.name.clone(), model);
    }
    let section_names = persisted_active_model_section_names(library)?;
    let mut section_definitions = HashMap::<String, (String, String)>::new();
    for requested_section in section_names {
        let matching_sections = parsed
            .sections
            .iter()
            .filter(|section| section.name.eq_ignore_ascii_case(&requested_section))
            .collect::<Vec<_>>();
        if matching_sections.is_empty() {
            return Err(format!(
                "selected corner requires section '{requested_section}', which is absent from the authenticated closure"
            ));
        }
        for section in matching_sections {
            for parsed_model in &section.models {
                let canonical = parsed_model.name.to_ascii_lowercase();
                if let Some((first_section, first_name)) = section_definitions
                    .insert(canonical, (section.name.clone(), parsed_model.name.clone()))
                {
                    return Err(format!(
                        "active model '{}' resolves from both sections '{}' and '{}'",
                        first_name, first_section, section.name
                    ));
                }
                let model = ModelLibraryManager::convert_parsed_model_in_section(
                    parsed_model,
                    root,
                    Some(&section.name),
                );
                insert_case_insensitive_model_projection(&mut projection, model);
            }
        }
    }
    Ok(projection)
}

fn insert_case_insensitive_model_projection(
    projection: &mut HashMap<String, DeviceModel>,
    model: DeviceModel,
) {
    if let Some(existing) = projection
        .keys()
        .find(|name| name.eq_ignore_ascii_case(&model.name))
        .cloned()
    {
        projection.remove(&existing);
    }
    projection.insert(model.name.clone(), model);
}

fn persisted_active_model_section_names(
    library: &ProjectModelLibrary,
) -> Result<Vec<String>, String> {
    if matches!(
        library.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ) && !library.model_definition_metadata.is_empty()
    {
        return Ok(Vec::new());
    }
    let Some(selected_corner) = library.selected_corner.as_deref() else {
        return Ok(Vec::new());
    };
    let corner = library.corners.get(selected_corner).ok_or_else(|| {
        format!("selected corner '{selected_corner}' does not exist in the corner catalog")
    })?;
    let mut sections = BTreeMap::<String, String>::new();
    for binding in corner.effective_section_bindings() {
        sections
            .entry(binding.section.to_ascii_lowercase())
            .or_insert(binding.section);
    }
    if sections.is_empty() {
        if corner.file_path.is_none() && library.source_closure.is_empty() {
            return Ok(Vec::new());
        }
        return Err(format!(
            "selected corner '{selected_corner}' has no executable section bindings"
        ));
    }
    Ok(sections.into_values().collect())
}

fn parsed_subcircuit_projection(
    _library: &ProjectModelLibrary,
    parsed: &rspice_core::library::LibParseResult,
    root: &std::path::Path,
) -> Result<HashMap<String, ModelSubcircuitInterface>, String> {
    let mut projection = HashMap::new();
    let mut canonical_names = HashMap::<String, String>::new();
    let mut insert = |subcircuit: &rspice_core::library::ParsedSubcircuit,
                      section: Option<&str>|
     -> Result<(), String> {
        let interface = ModelLibraryManager::convert_parsed_subcircuit(subcircuit, root, section);
        let key = subcircuit_interface_key(interface.section.as_deref(), &interface.name);
        let canonical = key.to_ascii_lowercase();
        if let Some(first) = canonical_names.insert(canonical, key.clone()) {
            return Err(format!(
                "subcircuit identity '{key}' duplicates '{first}' case-insensitively"
            ));
        }
        projection.insert(key, interface);
        Ok(())
    };

    for subcircuit in &parsed.top_level_subcircuits {
        insert(subcircuit, None)?;
    }
    for section in &parsed.sections {
        for subcircuit in &section.subcircuits {
            insert(subcircuit, Some(&section.name))?;
        }
    }
    Ok(projection)
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
        let mut case_folded_models = HashMap::<String, &str>::new();
        for model_key in library.models.keys() {
            if let Some(first) =
                case_folded_models.insert(model_key.to_ascii_lowercase(), model_key)
            {
                return Err(format!(
                    "{context}.models contains case-insensitive duplicate names '{first}' and '{model_key}'"
                ));
            }
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
            ModelSourceAuthority::RetainedImport { digest, .. }
            | ModelSourceAuthority::ProjectOwned { digest, .. } => {
                let authority = if library.source_authority.is_project_owned() {
                    "project_owned"
                } else {
                    "retained_import"
                };
                let Some(root_path) = library.root_path.as_ref() else {
                    return Err(format!(
                        "{context}.source_authority {authority} requires a root identity"
                    ));
                };
                if library.source_closure.is_empty()
                    || library.source_closure.len() != library.source_contents.len()
                {
                    return Err(format!(
                        "{context}.source_authority {authority} requires exact retained bytes for its complete source closure"
                    ));
                }
                let root_pin = library
                    .source_closure
                    .iter()
                    .find(|source| source.path == *root_path)
                    .ok_or_else(|| {
                        format!(
                            "{context}.source_authority {authority} closure does not contain its root identity"
                        )
                    })?;
                if root_pin.digest != digest {
                    return Err(format!(
                        "{context}.source_authority {authority} root bytes do not match the authority digest"
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
            if (library.source_authority.uses_retained_bytes() || !library.source_edges.is_empty())
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
        let mut case_folded_corners = HashMap::<String, &str>::new();
        for (corner_key, corner) in &library.corners {
            if corner_key.trim().is_empty() {
                return Err(format!("{context}.corners contains an empty section name"));
            }
            if let Some(first) =
                case_folded_corners.insert(corner_key.to_ascii_lowercase(), corner_key)
            {
                return Err(format!(
                    "{context}.corners contains case-insensitive duplicate names '{first}' and '{corner_key}'"
                ));
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
            if let Err(errors) = corner.validate_draft_contract() {
                return Err(format!(
                    "{context}.corners['{corner_key}'] has an invalid authoring draft: {}",
                    errors.join("; ")
                ));
            }
        }
        let active_model_sections = persisted_active_model_section_names(library)?
            .into_iter()
            .map(|section| section.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
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
            if let Some(section) = model.section.as_deref() {
                if section.trim().is_empty()
                    || section.chars().any(|character| {
                        character.is_whitespace()
                            || character == '"'
                            || character == '\''
                            || character.is_control()
                    })
                {
                    return Err(format!(
                        "{context}.models['{model_key}'].section contains an unsupported section identity"
                    ));
                }
                if !library.corners.contains_key(section) {
                    return Err(format!(
                        "{context}.models['{model_key}'].section references unknown section '{section}'"
                    ));
                }
                if !active_model_sections.contains(&section.to_ascii_lowercase()) {
                    return Err(format!(
                        "{context}.models['{model_key}'].section '{section}' is not bound by the library's selected executable corner"
                    ));
                }
            }
            validate_model_numbers(&context, model_key, model)?;
        }
        let mut case_folded_subcircuits = HashMap::<String, &str>::new();
        for (subcircuit_key, subcircuit) in &library.subcircuits {
            if subcircuit_key.trim().is_empty() {
                return Err(format!(
                    "{context}.subcircuits contains an empty subcircuit name"
                ));
            }
            if let Some(first) =
                case_folded_subcircuits.insert(subcircuit_key.to_ascii_lowercase(), subcircuit_key)
            {
                return Err(format!(
                    "{context}.subcircuits contains case-insensitive duplicate names '{first}' and '{subcircuit_key}'"
                ));
            }
            let expected_key =
                subcircuit_interface_key(subcircuit.section.as_deref(), &subcircuit.name);
            if expected_key != *subcircuit_key {
                return Err(format!(
                    "{context}.subcircuits key '{subcircuit_key}' does not match embedded subcircuit identity '{expected_key}'"
                ));
            }
            let mut ports = HashSet::with_capacity(subcircuit.ports.len());
            for (port_index, port) in subcircuit.ports.iter().enumerate() {
                if port.trim().is_empty() {
                    return Err(format!(
                        "{context}.subcircuits['{subcircuit_key}'].ports[{port_index}] must not be empty"
                    ));
                }
                if !ports.insert(port.to_ascii_lowercase()) {
                    return Err(format!(
                        "{context}.subcircuits['{subcircuit_key}'].ports contains duplicate terminal '{port}'"
                    ));
                }
            }
            let mut parameters = HashSet::with_capacity(subcircuit.parameter_defaults.len());
            for (parameter, value) in &subcircuit.parameter_defaults {
                if parameter.trim().is_empty() || value.trim().is_empty() {
                    return Err(format!(
                        "{context}.subcircuits['{subcircuit_key}'].parameter_defaults contains an empty name or value"
                    ));
                }
                if !parameters.insert(parameter.to_ascii_lowercase()) {
                    return Err(format!(
                        "{context}.subcircuits['{subcircuit_key}'].parameter_defaults contains case-insensitive duplicate parameter '{parameter}'"
                    ));
                }
            }
            if !source_path_is_authorized(library, subcircuit.file_path.as_ref()) {
                return Err(format!(
                    "{context}.subcircuits['{subcircuit_key}'] source path is not a member of the authenticated library closure"
                ));
            }
            if subcircuit.source_line.is_some_and(|line| line == 0) {
                return Err(format!(
                    "{context}.subcircuits['{subcircuit_key}'].source_line must be one-based"
                ));
            }
            if let Some(section) = subcircuit.section.as_deref()
                && !library.corners.contains_key(section)
            {
                return Err(format!(
                    "{context}.subcircuits['{subcircuit_key}'] references unknown section '{section}'"
                ));
            }
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
            ModelSourceAuthority::RetainedImport { .. }
            | ModelSourceAuthority::ProjectOwned { .. } => false,
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
    // Authoring evidence is a narrower semantic contract than the derived
    // section-catalog maps. Diagnose an invented qualification first even if
    // the same tampered draft also changed the canonical source projection.
    validate_model_authoring_records(context, library)?;
    let root = library
        .root_path
        .as_ref()
        .expect("authenticated closure validation requires a root path");
    validate_retained_model_source_dialects(library, context)?;
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

    let parsed_subcircuits = parsed_subcircuit_projection(library, &parsed, root)
        .map_err(|error| format!("{context}.subcircuits cannot be projected: {error}"))?;
    if library.subcircuits != parsed_subcircuits {
        return Err(format!(
            "{context}.subcircuits is not the exact interface projection of the authenticated source closure"
        ));
    }

    let parsed_models = parsed_model_projection(library, &parsed, root)
        .map_err(|error| format!("{context}.models cannot be projected: {error}"))?;
    if library.models.len() != parsed_models.len() {
        return Err(format!(
            "{context}.models does not exactly cover the top-level and selected-section model cards in the authenticated source closure"
        ));
    }
    for (model_name, model) in &library.models {
        let Some(candidate) = parsed_models.get(model_name) else {
            return Err(format!(
                "{context}.models['{model_name}'] is absent from the authenticated active model cards"
            ));
        };
        if !parsed_model_projection_matches(model, candidate) {
            return Err(format!(
                "{context}.models['{model_name}'] is not an exact projection of its authenticated active model card"
            ));
        }
    }
    let (parsed_top_level, parsed_sections) = parsed_complete_model_catalog(&parsed, root);
    if !model_projection_maps_match(&library.top_level_models, &parsed_top_level) {
        return Err(format!(
            "{context}.top_level_models is not the exact complete top-level projection of the authenticated source closure"
        ));
    }
    if library.section_models.len() != parsed_sections.len()
        || library.section_models.iter().any(|(section, models)| {
            parsed_sections
                .get(section)
                .is_none_or(|parsed| !model_projection_maps_match(models, parsed))
        })
    {
        return Err(format!(
            "{context}.section_models is not the exact complete section projection of the authenticated source closure"
        ));
    }
    Ok(())
}

fn validate_retained_model_source_dialects(
    library: &ProjectModelLibrary,
    context: &str,
) -> Result<(), String> {
    for content in &library.source_contents {
        let source =
            rspice_core::netlist::decode_source_bytes(&content.bytes).map_err(|error| {
                format!(
                    "{context} member '{}' cannot be decoded for dialect validation: {error}",
                    content.path.display()
                )
            })?;
        ModelLibraryManager::validate_model_source_dialect(&content.path, &source)
            .map_err(|error| format!("{context} rejects source dialect: {error}"))?;
    }
    Ok(())
}

fn model_projection_maps_match(
    persisted: &HashMap<String, DeviceModel>,
    parsed: &HashMap<String, DeviceModel>,
) -> bool {
    persisted.len() == parsed.len()
        && persisted.iter().all(|(name, model)| {
            parsed
                .get(name)
                .is_some_and(|candidate| parsed_model_projection_matches(model, candidate))
        })
}

fn parsed_model_projection_matches(persisted: &DeviceModel, parsed: &DeviceModel) -> bool {
    persisted.section == parsed.section
        && parsed_model_projection_matches_without_section(persisted, parsed)
}

fn parsed_model_projection_matches_without_section(
    persisted: &DeviceModel,
    parsed: &DeviceModel,
) -> bool {
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
mod tests;
