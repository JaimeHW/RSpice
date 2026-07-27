//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use sha2::Digest as _;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::product::{
    AnalysisInstanceId, ContentDigest, DesignVariableId, ObjectRevision, ProjectId,
    ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationPlanId,
};
use crate::state::{
    AnalysisResultSourceDomain, Cell, ComponentType, Library, LibraryCellInstance, LibraryManager,
    SchematicState, View, ViewType,
};

/// Default editable design library created for new projects.
pub const DEFAULT_PROJECT_LIBRARY: &str = "user";
/// Default top-level cell created for new projects.
pub const DEFAULT_TOP_CELL: &str = "top";
/// Default schematic view name.
pub const DEFAULT_SCHEMATIC_VIEW: &str = "schematic";
/// Persisted schema for project identity metadata.
pub const PROJECT_DESCRIPTOR_SCHEMA_VERSION: u16 = 1;
/// Persisted schema for an exact project-owned technology binding.
pub const PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION: u16 = 1;

/// Maximum legal hierarchy depth. This is deliberately generous for real
/// designs while placing a deterministic bound on corrupt or hostile project
/// data before it reaches netlisting.
const MAX_HIERARCHY_RESOLUTION_DEPTH: usize = 128;
/// Maximum number of expanded instances accepted by the configuration
/// resolver. The table remains grouped by master, but the receipt count is an
/// exact expanded-instance count up to this defensive product limit.
const MAX_HIERARCHY_RESOLUTION_INSTANCES: usize = 1_000_000;

/// Versioned identity domain for legacy session descriptors that predate a
/// persisted [`ProjectId`]. Project-file migration derives its ID from the
/// complete source bytes before deserialization; this namespace is reserved
/// for standalone/session descriptor migration.
const LEGACY_PROJECT_DESCRIPTOR_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0xd59a_680f_c781_5f1a_a69f_9a67_64bb_32ac);

/// Validate one persisted library, cell, or view name.
///
/// The slash-delimited workspace key format is unambiguous only while every
/// segment follows the same contract enforced by the library dialogs: a
/// non-empty sequence of Unicode letters/numbers and underscores. Persisted
/// data is validated against this boundary before any generated key is used.
pub fn validate_cell_view_name_segment(value: &str) -> Result<(), CellViewNameError> {
    if value.is_empty() {
        return Err(CellViewNameError::Empty);
    }
    if let Some(character) = value
        .chars()
        .find(|character| !character.is_alphanumeric() && *character != '_')
    {
        return Err(CellViewNameError::UnsupportedCharacter(character));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CellViewNameError {
    #[error("must not be empty")]
    Empty,
    #[error(
        "contains unsupported character {0:?}; only letters, numbers, and underscores are allowed"
    )]
    UnsupportedCharacter(char),
}

fn default_project_name() -> String {
    "Untitled Project".to_owned()
}

/// Presence-aware project identity used only while decoding persisted data.
///
/// `Option<T>` intentionally maps both a missing field (through `default`) and
/// an explicit JSON `null` to `None`. Those states have different security
/// semantics for project identity: only a genuinely missing field from an
/// unversioned legacy descriptor may be migrated.
#[derive(Debug, Default)]
enum DeserializedProjectId {
    #[default]
    Missing,
    Null,
    Value(ProjectId),
}

#[derive(Debug, Default)]
enum DeserializedProjectSchemaVersion {
    #[default]
    Missing,
    Null,
    Value(u16),
}

impl<'de> Deserialize<'de> for DeserializedProjectSchemaVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            serde_json::from_value(value)
                .map(Self::Value)
                .map_err(D::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for DeserializedProjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            serde_json::from_value(value)
                .map(Self::Value)
                .map_err(D::Error::custom)
        }
    }
}

/// A stable reference to one Library/Cell/View document.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellViewRef {
    pub library: String,
    pub cell: String,
    pub view: String,
}

impl CellViewRef {
    pub fn new(
        library: impl Into<String>,
        cell: impl Into<String>,
        view: impl Into<String>,
    ) -> Self {
        Self {
            library: library.into(),
            cell: cell.into(),
            view: view.into(),
        }
    }

    pub fn default_top() -> Self {
        Self::new(
            DEFAULT_PROJECT_LIBRARY,
            DEFAULT_TOP_CELL,
            DEFAULT_SCHEMATIC_VIEW,
        )
    }

    pub fn key(&self) -> String {
        format!("{}/{}/{}", self.library, self.cell, self.view)
    }

    /// Validate every segment before this reference participates in a
    /// persisted slash-delimited key.
    pub fn validate_name_segments(&self) -> Result<(), CellViewNameError> {
        validate_cell_view_name_segment(&self.library)?;
        validate_cell_view_name_segment(&self.cell)?;
        validate_cell_view_name_segment(&self.view)
    }

    pub fn display_path(&self) -> String {
        self.key()
    }
}

/// Resolution state for one grouped library/cell/view binding in the complete
/// testbench hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyBindingStatus {
    Resolved,
    Modified,
    Unresolved,
    Recursive,
    DepthLimit,
    InstanceLimit,
}

impl HierarchyBindingStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Modified => "modified",
            Self::Unresolved => "unresolved",
            Self::Recursive => "recursive",
            Self::DepthLimit => "depth limit",
            Self::InstanceLimit => "instance limit",
        }
    }

    pub fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved | Self::Modified)
    }

    pub fn is_modified(self) -> bool {
        self == Self::Modified
    }

    fn severity(self) -> u8 {
        match self {
            Self::Resolved => 0,
            Self::Modified => 1,
            Self::Unresolved => 2,
            Self::Recursive => 3,
            Self::DepthLimit => 4,
            Self::InstanceLimit => 5,
        }
    }
}

/// One row in the resolved hierarchy-binding manifest. Repeated masters are
/// grouped while `instance_count` retains their exact expanded multiplicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHierarchyBinding {
    pub reference: CellViewRef,
    pub purpose: String,
    pub view_search_order: Vec<String>,
    pub stop_view: Option<String>,
    pub model_section: String,
    pub status: HierarchyBindingStatus,
    pub instance_count: usize,
    /// Exact expanded instance paths represented by this grouped semantic
    /// binding. Paths remain available for configuration review and exact
    /// override audit even when repeated masters are grouped in the table.
    pub instance_paths: Vec<String>,
    /// True when the active configuration explicitly permits and records a
    /// reviewed fallback outside its primary ordered view policy.
    pub used_review_fallback: bool,
    pub diagnostic: Option<String>,
}

/// Immutable resolution receipt for the project configuration surface and
/// preflight diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyResolution {
    pub bindings: Vec<ResolvedHierarchyBinding>,
    pub total_instances: usize,
    pub resolved_instances: usize,
    pub configuration_id: Option<crate::state::ConfigurationSetId>,
    pub configuration_revision: Option<u64>,
    pub configuration_digest: Option<ContentDigest>,
}

impl HierarchyResolution {
    pub fn unresolved_instances(&self) -> usize {
        self.total_instances.saturating_sub(self.resolved_instances)
    }

    pub fn is_valid(&self) -> bool {
        self.unresolved_instances() == 0
    }
}

/// One immutable, exact-path executable binding consumed by hierarchical
/// netlist generation.  The placed schematic binding is deliberately not
/// retained as execution authority: `materialized_binding` is rebuilt from
/// the resolved Library/Cell/View and its authoritative view metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionBinding {
    instance_path: String,
    resolved_reference: CellViewRef,
    resolved_view_type: ViewType,
    materialized_binding: Option<LibraryCellInstance>,
    model_section: Option<String>,
    stop_boundary: bool,
    project_veriloga: Option<ConfigurationVerilogABinding>,
}

/// Exact project-owned behavioral source selected for one configuration
/// binding. This is derived from the active configuration and source registry;
/// it is never accepted from placed-instance or filesystem metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationVerilogABinding {
    source_bundle_id: ProjectSourceId,
    source_closure_digest: ContentDigest,
    selected_module: String,
    source_key: String,
    netlist_alias: String,
}

impl ConfigurationVerilogABinding {
    pub const fn source_bundle_id(&self) -> ProjectSourceId {
        self.source_bundle_id
    }

    pub const fn source_closure_digest(&self) -> ContentDigest {
        self.source_closure_digest
    }

    pub fn selected_module(&self) -> &str {
        &self.selected_module
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }
}

impl ConfigurationExecutionBinding {
    pub fn instance_path(&self) -> &str {
        &self.instance_path
    }

    pub const fn resolved_reference(&self) -> &CellViewRef {
        &self.resolved_reference
    }

    pub const fn resolved_view_type(&self) -> ViewType {
        self.resolved_view_type
    }

    pub const fn materialized_binding(&self) -> Option<&LibraryCellInstance> {
        self.materialized_binding.as_ref()
    }

    pub fn model_section(&self) -> Option<&str> {
        self.model_section.as_deref()
    }

    pub const fn stop_boundary(&self) -> bool {
        self.stop_boundary
    }

    pub const fn project_veriloga(&self) -> Option<&ConfigurationVerilogABinding> {
        self.project_veriloga.as_ref()
    }
}

/// Frozen per-instance hierarchy authority for one active configuration-set
/// revision. Keys are canonicalized exact instance paths; values retain the
/// display spelling for receipts and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionPlan {
    root: CellViewRef,
    bindings: BTreeMap<String, ConfigurationExecutionBinding>,
    configuration_id: crate::state::ConfigurationSetId,
    configuration_revision: u64,
    configuration_digest: ContentDigest,
}

impl ConfigurationExecutionPlan {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn binding(&self, instance_path: &str) -> Option<&ConfigurationExecutionBinding> {
        self.bindings.get(&instance_path.to_ascii_lowercase())
    }

    pub fn bindings(&self) -> impl ExactSizeIterator<Item = &ConfigurationExecutionBinding> {
        self.bindings.values()
    }

    pub const fn configuration_id(&self) -> crate::state::ConfigurationSetId {
        self.configuration_id
    }

    pub const fn configuration_revision(&self) -> u64 {
        self.configuration_revision
    }

    pub const fn configuration_digest(&self) -> ContentDigest {
        self.configuration_digest
    }
}

/// Owned live-buffer projection paired with its frozen configuration plan.
/// Holding both in one value prevents a caller from resolving one hierarchy
/// and accidentally netlisting a different editor buffer.
#[derive(Debug, Clone)]
pub struct ConfigurationExecutionProjection {
    root: CellViewRef,
    schematic_buffers: HashMap<String, SchematicState>,
    plan: Option<ConfigurationExecutionPlan>,
    connectivity: crate::state::ConnectivityContract,
}

impl ConfigurationExecutionProjection {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn root_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&self.root.key()))
            .map(|(_, schematic)| schematic)
    }

    pub const fn schematic_buffers(&self) -> &HashMap<String, SchematicState> {
        &self.schematic_buffers
    }

    pub const fn plan(&self) -> Option<&ConfigurationExecutionPlan> {
        self.plan.as_ref()
    }

    pub const fn connectivity(&self) -> &crate::state::ConnectivityContract {
        &self.connectivity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigurationExecutionPlanError {
    #[error("configuration hierarchy is unresolved: {0}")]
    Unresolved(String),
    #[error("configuration root {0} has no materialized schematic buffer")]
    MissingRoot(String),
    #[error("design-management projection is invalid: {0}")]
    DesignManagement(String),
}

/// Exact project-owned attachment to a locally parsed and content-pinned model
/// technology. Physical layer decks, signed organization packages, and remote
/// entitlement receipts require provider records that are intentionally not
/// represented by this local binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTechnologyBinding {
    schema_version: u16,
    package_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    package_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    technology_node: Option<String>,
    model_library: String,
    root_source: PathBuf,
    source_closure: Vec<crate::state::model_library::ModelSourcePin>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    source_edges: Vec<crate::state::model_library::ModelSourceEdge>,
    model_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    process_sections: Vec<String>,
}

impl ProjectTechnologyBinding {
    pub fn from_model_library(
        library: &crate::state::model_library::ModelLibrary,
    ) -> Result<Self, TechnologyBindingError> {
        validate_retained_model_sources(library)?;
        let root_source = library
            .root_path
            .clone()
            .ok_or(TechnologyBindingError::MissingRootSource)?;
        let package_name = if library.pdk_name.trim().is_empty() {
            library.name.clone()
        } else {
            library.pdk_name.clone()
        };
        let package_version = nonempty_owned(&library.version);
        let technology_node = nonempty_owned(&library.technology_node);
        let mut process_sections = library.corners.keys().cloned().collect::<Vec<_>>();
        process_sections.sort();
        let binding = Self {
            schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            package_name,
            package_version,
            technology_node,
            model_library: library.name.clone(),
            root_source,
            source_closure: library.source_closure.clone(),
            source_edges: library.source_edges.clone(),
            model_count: library.models.len(),
            process_sections,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn validate(&self) -> Result<(), TechnologyBindingError> {
        if self.schema_version != PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION {
            return Err(TechnologyBindingError::UnsupportedSchema {
                found: self.schema_version,
                supported: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            });
        }
        validate_technology_text("package_name", &self.package_name)?;
        validate_technology_text("model_library", &self.model_library)?;
        if let Some(version) = &self.package_version {
            validate_technology_text("package_version", version)?;
        }
        if let Some(node) = &self.technology_node {
            validate_technology_text("technology_node", node)?;
        }
        if self.root_source.as_os_str().is_empty() {
            return Err(TechnologyBindingError::MissingRootSource);
        }
        if !crate::state::model_library::is_portable_absolute_path(&self.root_source) {
            return Err(TechnologyBindingError::NonAbsoluteSource(
                self.root_source.clone(),
            ));
        }
        if self.source_closure.is_empty() {
            return Err(TechnologyBindingError::EmptySourceClosure);
        }
        let mut paths = std::collections::HashSet::with_capacity(self.source_closure.len());
        for (index, source) in self.source_closure.iter().enumerate() {
            if !crate::state::model_library::is_portable_absolute_path(&source.path) {
                return Err(TechnologyBindingError::NonAbsoluteSource(
                    source.path.clone(),
                ));
            }
            if !paths.insert(source.path.clone()) {
                return Err(TechnologyBindingError::DuplicateSource(source.path.clone()));
            }
            if index > 0 && self.source_closure[index - 1].path >= source.path {
                return Err(TechnologyBindingError::UnsortedSourceClosure);
            }
        }
        if !paths.contains(&self.root_source) {
            return Err(TechnologyBindingError::RootAbsentFromClosure(
                self.root_source.clone(),
            ));
        }
        if self.source_closure.len() > 1 && self.source_edges.is_empty() {
            return Err(TechnologyBindingError::MissingSourceEdges);
        }
        for (index, edge) in self.source_edges.iter().enumerate() {
            if index > 0 && self.source_edges[index - 1] >= *edge {
                return Err(TechnologyBindingError::UnsortedSourceEdges);
            }
            if !paths.contains(&edge.owner) || !paths.contains(&edge.target) {
                return Err(TechnologyBindingError::SourceEdgeOutsideClosure);
            }
            rspice_core::netlist::normalize_source_path_literal(&edge.requested_path)
                .map_err(|_| TechnologyBindingError::InvalidSourceEdge)?;
        }
        if let Some(unreachable) = crate::state::model_library::first_unreachable_source(
            &self.root_source,
            &self.source_closure,
            &self.source_edges,
        ) {
            return Err(TechnologyBindingError::UnreachableSource(
                unreachable.to_path_buf(),
            ));
        }
        if self.model_count == 0 {
            return Err(TechnologyBindingError::NoModels);
        }
        for (index, section) in self.process_sections.iter().enumerate() {
            validate_technology_text("process_sections", section)?;
            if index > 0 && self.process_sections[index - 1] >= *section {
                return Err(TechnologyBindingError::UnsortedProcessSections);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        let mut label = self.package_name.clone();
        if let Some(version) = &self.package_version {
            label.push_str(" · ");
            label.push_str(version);
        }
        if let Some(node) = &self.technology_node {
            label.push_str(" · ");
            label.push_str(node);
        }
        label
    }

    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    #[must_use]
    pub fn package_version(&self) -> Option<&str> {
        self.package_version.as_deref()
    }

    #[must_use]
    pub fn technology_node(&self) -> Option<&str> {
        self.technology_node.as_deref()
    }

    #[must_use]
    pub fn model_library(&self) -> &str {
        &self.model_library
    }

    #[must_use]
    pub fn root_source(&self) -> &Path {
        &self.root_source
    }

    #[must_use]
    pub fn source_closure(&self) -> &[crate::state::model_library::ModelSourcePin] {
        &self.source_closure
    }

    #[must_use]
    pub fn source_edges(&self) -> &[crate::state::model_library::ModelSourceEdge] {
        &self.source_edges
    }

    /// Prove that the mutable execution catalog still contains exactly the
    /// technology contract accepted by the project. Re-parsing, refreshing,
    /// or replacing a library must therefore invalidate the attachment until
    /// the user explicitly accepts the new contract.
    pub(crate) fn validate_model_library(
        &self,
        library: &crate::state::model_library::ModelLibrary,
    ) -> Result<(), TechnologyBindingError> {
        let observed = Self::from_model_library(library)?;
        if &observed != self {
            return Err(TechnologyBindingError::CatalogDrift {
                library: self.model_library.clone(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub const fn model_count(&self) -> usize {
        self.model_count
    }

    #[must_use]
    pub fn process_sections(&self) -> &[String] {
        &self.process_sections
    }
}

fn validate_retained_model_sources(
    library: &crate::state::model_library::ModelLibrary,
) -> Result<(), TechnologyBindingError> {
    if library.source_contents.len() != library.source_closure.len() {
        return Err(TechnologyBindingError::MissingRetainedSourceBytes);
    }
    for (pin, content) in library.source_closure.iter().zip(&library.source_contents) {
        if pin.path != content.path {
            return Err(TechnologyBindingError::RetainedSourceIdentityMismatch);
        }
        let digest =
            crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&content.bytes).into());
        if digest != pin.digest {
            return Err(TechnologyBindingError::RetainedSourceDigestMismatch(
                pin.path.clone(),
            ));
        }
    }
    Ok(())
}

fn nonempty_owned(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.to_owned())
}

fn validate_technology_text(
    field: &'static str,
    value: &str,
) -> Result<(), TechnologyBindingError> {
    if value.is_empty() {
        return Err(TechnologyBindingError::RequiredField(field));
    }
    if value != value.trim() {
        return Err(TechnologyBindingError::SurroundingWhitespace(field));
    }
    if value.chars().count() > 240 {
        return Err(TechnologyBindingError::FieldTooLong(field));
    }
    if value.chars().any(char::is_control) {
        return Err(TechnologyBindingError::ControlCharacter(field));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TechnologyBindingError {
    #[error("technology binding schema {found} is unsupported; this build supports {supported}")]
    UnsupportedSchema { found: u16, supported: u16 },
    #[error("technology binding field {0} is required")]
    RequiredField(&'static str),
    #[error("technology binding field {0} must not begin or end with whitespace")]
    SurroundingWhitespace(&'static str),
    #[error("technology binding field {0} exceeds 240 Unicode scalar values")]
    FieldTooLong(&'static str),
    #[error("technology binding field {0} contains a control character")]
    ControlCharacter(&'static str),
    #[error("technology binding has no canonical root model source")]
    MissingRootSource,
    #[error("technology binding has no pinned model-source closure")]
    EmptySourceClosure,
    #[error("technology binding repeats pinned source {0}")]
    DuplicateSource(PathBuf),
    #[error("technology binding root {0} is absent from its pinned source closure")]
    RootAbsentFromClosure(PathBuf),
    #[error("technology binding source {0} is not an absolute portable path identity")]
    NonAbsoluteSource(PathBuf),
    #[error("technology binding source closure must be strictly sorted by canonical path")]
    UnsortedSourceClosure,
    #[error("multi-file technology binding has no authenticated dependency-resolution edges")]
    MissingSourceEdges,
    #[error("technology binding source edges must be strictly sorted and unique")]
    UnsortedSourceEdges,
    #[error("technology binding source edge references a source outside its pinned closure")]
    SourceEdgeOutsideClosure,
    #[error("technology binding contains an invalid source include path")]
    InvalidSourceEdge,
    #[error("technology binding source {0} is unreachable from its root")]
    UnreachableSource(PathBuf),
    #[error("technology binding process sections must be strictly sorted and unique")]
    UnsortedProcessSections,
    #[error("technology binding contains no parsed device models")]
    NoModels,
    #[error("technology binding does not retain exact bytes for every pinned model source")]
    MissingRetainedSourceBytes,
    #[error("technology binding retained source identities do not match their pinned closure")]
    RetainedSourceIdentityMismatch,
    #[error("technology binding retained bytes for {0} do not match their accepted digest")]
    RetainedSourceDigestMismatch(PathBuf),
    #[error(
        "attached model library '{library}' no longer matches the accepted technology contract"
    )]
    CatalogDrift { library: String },
}

/// Metadata for the active RSpice project.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectDescriptor {
    /// Stable product identity. Legacy project files receive an ID exactly
    /// once during deserialization and retain it on every later save.
    id: ProjectId,
    /// Schema of this object, independent of the outer project container.
    schema_version: u16,
    /// Monotonic logical revision. Runtime-only path changes do not alter it.
    #[serde(default)]
    revision: ObjectRevision,
    #[serde(default = "default_project_name")]
    name: String,
    pub path: Option<PathBuf>,
    pub root_library: String,
    pub top_cell: String,
    /// Legacy display-only attachment retained for schema-1 project and
    /// session compatibility. New commits always pair it with the exact
    /// structured binding below.
    pub technology: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    technology_binding: Option<ProjectTechnologyBinding>,
    pub description: String,
}

impl<'de> Deserialize<'de> for ProjectDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ProjectDescriptorDe {
            #[serde(default)]
            id: DeserializedProjectId,
            #[serde(default)]
            schema_version: DeserializedProjectSchemaVersion,
            #[serde(default)]
            revision: ObjectRevision,
            #[serde(default = "default_project_name")]
            name: String,
            path: Option<PathBuf>,
            root_library: String,
            top_cell: String,
            technology: Option<String>,
            #[serde(default)]
            technology_binding: Option<ProjectTechnologyBinding>,
            description: String,
        }

        let descriptor = ProjectDescriptorDe::deserialize(deserializer)?;
        if matches!(
            &descriptor.schema_version,
            DeserializedProjectSchemaVersion::Null
        ) {
            return Err(D::Error::custom(
                "project schema version must not be explicitly null",
            ));
        }
        let has_schema_version = matches!(
            &descriptor.schema_version,
            DeserializedProjectSchemaVersion::Value(_)
        );
        let id = match descriptor.id {
            DeserializedProjectId::Null => {
                return Err(D::Error::custom(
                    "project identity must not be explicitly null",
                ));
            }
            DeserializedProjectId::Value(id) => id,
            DeserializedProjectId::Missing if has_schema_version => {
                return Err(D::Error::custom(
                    "versioned project descriptor is missing its stable identity",
                ));
            }
            DeserializedProjectId::Missing => {
                // Serialize the complete identity-bearing descriptor state in
                // a fixed field order. Project/session containers inject an ID
                // scoped to their complete artifact before reaching this
                // fallback; this path covers standalone genuine-legacy
                // descriptors without inventing random identity.
                let material = serde_json::to_vec(&(
                    descriptor.revision,
                    &descriptor.name,
                    &descriptor.path,
                    &descriptor.root_library,
                    &descriptor.top_cell,
                    &descriptor.technology,
                    &descriptor.technology_binding,
                    &descriptor.description,
                ))
                .map_err(D::Error::custom)?;
                ProjectId::from_namespace(LEGACY_PROJECT_DESCRIPTOR_ID_NAMESPACE, &material)
            }
        };

        Ok(Self {
            id,
            schema_version: match descriptor.schema_version {
                DeserializedProjectSchemaVersion::Value(version) => version,
                DeserializedProjectSchemaVersion::Missing => PROJECT_DESCRIPTOR_SCHEMA_VERSION,
                DeserializedProjectSchemaVersion::Null => {
                    unreachable!("explicitly null project schema rejected above")
                }
            },
            revision: descriptor.revision,
            name: descriptor.name,
            path: descriptor.path,
            root_library: descriptor.root_library,
            top_cell: descriptor.top_cell,
            technology: descriptor.technology,
            technology_binding: descriptor.technology_binding,
            description: descriptor.description,
        })
    }
}

impl Default for ProjectDescriptor {
    fn default() -> Self {
        Self {
            id: ProjectId::new(),
            schema_version: PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            revision: ObjectRevision::INITIAL,
            name: default_project_name(),
            path: None,
            root_library: DEFAULT_PROJECT_LIBRARY.to_string(),
            top_cell: DEFAULT_TOP_CELL.to_string(),
            technology: None,
            technology_binding: None,
            description: String::new(),
        }
    }
}

impl ProjectDescriptor {
    #[must_use]
    pub const fn id(&self) -> ProjectId {
        self.id
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    /// Preflight the next logical project revision without mutating state.
    /// Multi-owner transactions use this before changing library or document
    /// state so exhausted revision authority fails before any partial commit.
    pub fn next_revision(&self) -> Result<ObjectRevision, ProjectDescriptorError> {
        self.revision.next().map_err(ProjectDescriptorError::from)
    }

    /// Advance only the logical project revision after every other part of a
    /// preflighted project transaction is ready to publish.
    pub fn advance_revision(&mut self) -> Result<ObjectRevision, ProjectDescriptorError> {
        let revision = self.next_revision()?;
        self.revision = revision;
        Ok(revision)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn display_name(&self) -> &str {
        if self.name.trim().is_empty() {
            "Untitled Project"
        } else {
            self.name.as_str()
        }
    }

    /// Validate the persisted identity boundary before saving or accepting a
    /// project file. Remote uniqueness and authority checks remain service
    /// responsibilities because they require the selected project location.
    pub fn validate(&self) -> Result<(), ProjectDescriptorError> {
        if self.schema_version != PROJECT_DESCRIPTOR_SCHEMA_VERSION {
            return Err(ProjectDescriptorError::UnsupportedSchema {
                found: self.schema_version,
                supported: PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            });
        }
        if self.id.as_uuid().is_nil() {
            return Err(ProjectDescriptorError::NilIdentity);
        }
        Self::validate_name(&self.name)?;
        if self.root_library.trim().is_empty() {
            return Err(ProjectDescriptorError::RequiredField("root_library"));
        }
        if self.top_cell.trim().is_empty() {
            return Err(ProjectDescriptorError::RequiredField("top_cell"));
        }
        if let Some(binding) = &self.technology_binding {
            binding.validate()?;
            let expected_label = binding.display_label();
            if self.technology.as_deref() != Some(expected_label.as_str()) {
                return Err(ProjectDescriptorError::TechnologyLabelMismatch);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn technology_binding(&self) -> Option<&ProjectTechnologyBinding> {
        self.technology_binding.as_ref()
    }

    /// Attach an exact, validated technology contract as one atomic project
    /// metadata revision. Reattaching the identical binding is a no-op.
    pub fn attach_technology(
        &mut self,
        binding: ProjectTechnologyBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        binding.validate()?;
        let label = binding.display_label();
        if self.technology_binding.as_ref() == Some(&binding)
            && self.technology.as_deref() == Some(label.as_str())
        {
            return Ok(self.revision);
        }
        let next_revision = self.revision.next()?;
        self.technology = Some(label);
        self.technology_binding = Some(binding);
        self.revision = next_revision;
        Ok(next_revision)
    }

    /// Validate the local portion of `project.name` from the frozen field
    /// contract. The exact Unicode scalar sequence is retained; no case or
    /// normalization folding is performed.
    pub fn validate_name(value: &str) -> Result<(), ProjectDescriptorError> {
        if value != value.trim() {
            return Err(ProjectDescriptorError::SurroundingWhitespace);
        }
        let grapheme_count = value.graphemes(true).count();
        if grapheme_count == 0 {
            return Err(ProjectDescriptorError::EmptyName);
        }
        if grapheme_count > 120 {
            return Err(ProjectDescriptorError::NameTooLong { grapheme_count });
        }
        if let Some(character) = value.chars().find(|character| character.is_control()) {
            return Err(ProjectDescriptorError::ControlCharacter(character));
        }
        if let Some(separator) = value
            .chars()
            .find(|character| matches!(character, '/' | '\\'))
        {
            return Err(ProjectDescriptorError::PathSeparator(separator));
        }
        Ok(())
    }

    /// Rename as one fail-closed logical transaction. A rejected name leaves
    /// both the value and revision unchanged.
    pub fn rename(
        &mut self,
        name: impl Into<String>,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        let name = name.into();
        Self::validate_name(&name)?;
        if self.name == name {
            return Ok(self.revision);
        }
        let next_revision = self.revision.next()?;
        self.name = name;
        self.revision = next_revision;
        Ok(next_revision)
    }

    pub fn set_path(&mut self, path: PathBuf) {
        // The first save may supply a useful name for an otherwise untitled
        // project. Reopening or moving a named project must not silently
        // rename it or change its logical revision.
        if self.path.is_none()
            && self.name == default_project_name()
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && Self::validate_name(stem).is_ok()
        {
            // Validation above and an available next revision make this
            // infallible for every practical project lifetime. If the
            // revision space is exhausted, retaining the old name is safer.
            let _ = self.rename(stem.to_owned());
        }
        self.path = Some(path);
    }

    /// Create the descriptor for an independent project copy.
    ///
    /// A project copy is not a rename or move of the source project: it owns
    /// a fresh stable identity and starts a new revision history at the
    /// selected location.  The receiver is borrowed, so this operation cannot
    /// accidentally rebind or otherwise mutate the source project.
    #[must_use]
    pub fn fork_copy_at(&self, path: PathBuf) -> Self {
        let mut copy = self.clone();
        copy.id = ProjectId::new();
        copy.revision = ObjectRevision::INITIAL;
        copy.path = Some(path);
        copy
    }

    pub fn directory(&self) -> Option<&Path> {
        self.path.as_deref().and_then(Path::parent)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectDescriptorError {
    #[error("project schema version {found} is unsupported; this build supports {supported}")]
    UnsupportedSchema { found: u16, supported: u16 },
    #[error("project identity must not be the nil UUID")]
    NilIdentity,
    #[error("project name must contain at least one grapheme cluster")]
    EmptyName,
    #[error("project name contains {grapheme_count} grapheme clusters; the maximum is 120")]
    NameTooLong { grapheme_count: usize },
    #[error("project name must not begin or end with whitespace")]
    SurroundingWhitespace,
    #[error("project name contains control character {0:?}")]
    ControlCharacter(char),
    #[error("project name contains path separator {0:?}")]
    PathSeparator(char),
    #[error("project field {0} is required")]
    RequiredField(&'static str),
    #[error("legacy technology label does not match the exact project technology binding")]
    TechnologyLabelMismatch,
    #[error(transparent)]
    Technology(#[from] TechnologyBindingError),
    #[error(transparent)]
    Revision(#[from] RevisionError),
}

/// One open view tab in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCellView {
    pub reference: CellViewRef,
    pub view_type: ViewType,
    pub dirty: bool,
}

impl OpenCellView {
    pub fn new(reference: CellViewRef, view_type: ViewType) -> Self {
        Self {
            reference,
            view_type,
            dirty: false,
        }
    }
}

fn is_schematic_like(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

fn library_view_type(libraries: &LibraryManager, reference: &CellViewRef) -> Option<ViewType> {
    libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
}

/// One specification bound for a `.MEAS` result — a row of the specs
/// matrix. At least one of `min`/`max` is normally set; a spec with
/// neither still pins the measurement as a tracked row (value-only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecEntry {
    /// `.MEAS` result name this spec bounds (case-insensitive match).
    pub measurement: String,
    /// Lower bound (pass when value ≥ min).
    pub min: Option<f64>,
    /// Upper bound (pass when value ≤ max).
    pub max: Option<f64>,
    /// Display unit, purely cosmetic (e.g. "V", "s", "dB").
    pub unit: String,
}

impl SpecEntry {
    pub fn validate(&self) -> Result<(), String> {
        validate_parameter_name(&self.measurement)
            .map_err(|error| format!("measurement name is invalid: {error}"))?;
        if self.min.is_some_and(|value| !value.is_finite())
            || self.max.is_some_and(|value| !value.is_finite())
        {
            return Err("specification bounds must be finite".to_owned());
        }
        if self
            .min
            .zip(self.max)
            .is_some_and(|(minimum, maximum)| minimum > maximum)
        {
            return Err("specification minimum exceeds its maximum".to_owned());
        }
        validate_bounded_text("unit", &self.unit, 64, true)
    }

    /// Spec verdict for one measured value.
    pub fn passes(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }

    /// Violation magnitude (how far outside the bounds), 0 when passing.
    pub fn violation(&self, value: f64) -> f64 {
        let below = self.min.map_or(0.0, |min| (min - value).max(0.0));
        let above = self.max.map_or(0.0, |max| (value - max).max(0.0));
        below.max(above)
    }
}

/// Physical quantity carried by a design variable. The quantity is retained
/// independently from the expression so editors can validate units without
/// coercing the user's exact engineering input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableQuantity {
    Resistance,
    Capacitance,
    Voltage,
    Current,
    Temperature,
    Dimensionless,
}

impl DesignVariableQuantity {
    pub const ALL: [Self; 6] = [
        Self::Resistance,
        Self::Capacitance,
        Self::Voltage,
        Self::Current,
        Self::Temperature,
        Self::Dimensionless,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Resistance => "Resistance",
            Self::Capacitance => "Capacitance",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Temperature => "Temperature",
            Self::Dimensionless => "Dimensionless",
        }
    }
}

/// Exact ownership boundary for a design variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DesignVariableScope {
    Testbench,
    Project,
    SelectedCell { cell: CellViewRef },
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl DesignVariableScope {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Testbench => "Lab characterization · testbench",
            Self::Project => "Project",
            Self::SelectedCell { .. } => "Selected cell",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignVariableScopeKind {
    Testbench,
    Project,
    SelectedCell,
    SelectedAnalysis,
}

impl DesignVariableScopeKind {
    pub const ALL: [Self; 4] = [
        Self::Testbench,
        Self::Project,
        Self::SelectedCell,
        Self::SelectedAnalysis,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Testbench => "Lab characterization · testbench",
            Self::Project => "Project",
            Self::SelectedCell => "Selected cell",
            Self::SelectedAnalysis => "Selected analysis only",
        }
    }
}

impl From<&DesignVariableScope> for DesignVariableScopeKind {
    fn from(value: &DesignVariableScope) -> Self {
        match value {
            DesignVariableScope::Testbench => Self::Testbench,
            DesignVariableScope::Project => Self::Project,
            DesignVariableScope::SelectedCell { .. } => Self::SelectedCell,
            DesignVariableScope::SelectedAnalysis { .. } => Self::SelectedAnalysis,
        }
    }
}

/// Inclusive engineering bounds for a variable. Bounds remain expressions so
/// suffixes and owner variables survive a lossless project round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariableRange {
    pub minimum: String,
    pub maximum: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableSweepEligibility {
    NestedSweepAndOptimization,
    OptimizationOnly,
    FixedParameter,
}

impl DesignVariableSweepEligibility {
    pub const ALL: [Self; 3] = [
        Self::NestedSweepAndOptimization,
        Self::OptimizationOnly,
        Self::FixedParameter,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::NestedSweepAndOptimization => "Nested sweep + optimization",
            Self::OptimizationOnly => "Optimization only",
            Self::FixedParameter => "Fixed parameter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableOverridePolicy {
    ExplicitTestLocalOverride,
    InheritOwnerOnly,
}

impl DesignVariableOverridePolicy {
    pub const ALL: [Self; 2] = [Self::ExplicitTestLocalOverride, Self::InheritOwnerOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExplicitTestLocalOverride => "Explicit test-local override",
            Self::InheritOwnerOnly => "Inherit owner only",
        }
    }
}

/// Persisted, typed simulation-plan parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariable {
    pub id: DesignVariableId,
    pub revision: ObjectRevision,
    pub name: String,
    pub expression: String,
    pub quantity: DesignVariableQuantity,
    pub scope: DesignVariableScope,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub allowed_range: Option<DesignVariableRange>,
    pub sweep_eligibility: DesignVariableSweepEligibility,
    pub override_policy: DesignVariableOverridePolicy,
}

impl DesignVariable {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        expression: impl Into<String>,
        quantity: DesignVariableQuantity,
        scope: DesignVariableScope,
        description: impl Into<String>,
        allowed_range: Option<DesignVariableRange>,
        sweep_eligibility: DesignVariableSweepEligibility,
        override_policy: DesignVariableOverridePolicy,
    ) -> Result<Self, String> {
        let variable = Self {
            id: DesignVariableId::new(),
            revision: ObjectRevision::INITIAL,
            name: name.into(),
            expression: expression.into(),
            quantity,
            scope,
            description: description.into(),
            allowed_range,
            sweep_eligibility,
            override_policy,
        };
        variable.validate()?;
        Ok(variable)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_parameter_name(&self.name)?;
        validate_single_line_expression("expression", &self.expression)?;
        let value = self.resolved_value_si()?;
        if let Some(range) = &self.allowed_range {
            validate_single_line_expression("allowed-range minimum", &range.minimum)?;
            validate_single_line_expression("allowed-range maximum", &range.maximum)?;
            let minimum =
                parse_design_quantity(&range.minimum, self.quantity).map_err(|error| {
                    format!(
                        "allowed-range minimum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })?;
            let maximum =
                parse_design_quantity(&range.maximum, self.quantity).map_err(|error| {
                    format!(
                        "allowed-range maximum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })?;
            if minimum > maximum {
                return Err("allowed-range minimum exceeds its maximum".to_owned());
            }
            if value < minimum || value > maximum {
                return Err(format!(
                    "resolved value {value} is outside the inclusive allowed range {minimum}..={maximum}"
                ));
            }
        }
        validate_bounded_text("description", &self.description, 1_024, true)?;
        if let DesignVariableScope::SelectedCell { cell } = &self.scope {
            cell.validate_name_segments()
                .map_err(|error| format!("selected cell is invalid: {error}"))?;
        }
        Ok(())
    }

    pub fn resolved_value_si(&self) -> Result<f64, String> {
        parse_design_quantity(&self.expression, self.quantity).map_err(|error| {
            format!(
                "expression is invalid for {}: {error}",
                self.quantity.label()
            )
        })
    }

    /// Canonical top-level SPICE statement. Validation is intentionally kept
    /// separate so callers can aggregate every project diagnostic at once.
    pub fn netlist_statement(&self) -> String {
        let value = self
            .resolved_value_si()
            .expect("validated design variables always resolve to finite SI values");
        format!(".param {}={value:.17e}", self.name)
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = DesignVariableId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let DesignVariableScope::SelectedAnalysis { analysis_id } = &mut cloned.scope {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

const LEGACY_DESIGN_VARIABLE_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x3c56_6c65_03dc_5e65_b66c_a4d4_86dc_8d53);

impl<'de> Deserialize<'de> for DesignVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            #[serde(default = "missing_identity_sentinel")]
            id: serde_json::Value,
            #[serde(default)]
            revision: ObjectRevision,
            name: String,
            expression: String,
            quantity: DesignVariableQuantity,
            scope: DesignVariableScope,
            #[serde(default)]
            description: String,
            #[serde(default)]
            allowed_range: Option<DesignVariableRange>,
            sweep_eligibility: DesignVariableSweepEligibility,
            override_policy: DesignVariableOverridePolicy,
        }

        let wire = Wire::deserialize(deserializer)?;
        let identity = serde_json::to_vec(&(
            &wire.name,
            &wire.expression,
            wire.quantity,
            &wire.scope,
            &wire.description,
            &wire.allowed_range,
            wire.sweep_eligibility,
            wire.override_policy,
        ))
        .map_err(D::Error::custom)?;
        let id = deserialize_or_migrate_identity::<DesignVariableId, D::Error>(
            wire.id,
            LEGACY_DESIGN_VARIABLE_ID_NAMESPACE,
            &identity,
            DesignVariableId::from_namespace,
        )?;
        Ok(Self {
            id,
            revision: wire.revision,
            name: wire.name,
            expression: wire.expression,
            quantity: wire.quantity,
            scope: wire.scope,
            description: wire.description,
            allowed_range: wire.allowed_range,
            sweep_eligibility: wire.sweep_eligibility,
            override_policy: wire.override_policy,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputKind {
    RawVoltageOrCurrent,
    DerivedExpression,
    DeviceOperatingPointQuantity,
    NoiseContributor,
    RfPortQuantity,
}

impl SavedOutputKind {
    pub const ALL: [Self; 5] = [
        Self::RawVoltageOrCurrent,
        Self::DerivedExpression,
        Self::DeviceOperatingPointQuantity,
        Self::NoiseContributor,
        Self::RfPortQuantity,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RawVoltageOrCurrent => "Raw voltage / current",
            Self::DerivedExpression => "Derived expression",
            Self::DeviceOperatingPointQuantity => "Device operating-point quantity",
            Self::NoiseContributor => "Noise contributor",
            Self::RfPortQuantity => "RF port quantity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedOutputCompatibility {
    OpTranAc,
    AllCompatibleAnalyses,
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl SavedOutputCompatibility {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::OpTranAc => "OP + TRAN + AC",
            Self::AllCompatibleAnalyses => "All compatible analyses",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedOutputCompatibilityKind {
    OpTranAc,
    AllCompatibleAnalyses,
    SelectedAnalysis,
}

impl SavedOutputCompatibilityKind {
    pub const ALL: [Self; 3] = [
        Self::OpTranAc,
        Self::AllCompatibleAnalyses,
        Self::SelectedAnalysis,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::OpTranAc => "OP + TRAN + AC",
            Self::AllCompatibleAnalyses => "All compatible analyses",
            Self::SelectedAnalysis => "Selected analysis only",
        }
    }
}

impl From<&SavedOutputCompatibility> for SavedOutputCompatibilityKind {
    fn from(value: &SavedOutputCompatibility) -> Self {
        match value {
            SavedOutputCompatibility::OpTranAc => Self::OpTranAc,
            SavedOutputCompatibility::AllCompatibleAnalyses => Self::AllCompatibleAnalyses,
            SavedOutputCompatibility::SelectedAnalysis { .. } => Self::SelectedAnalysis,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPolicy {
    EveryAcceptedPoint,
    SelectedAndFinalPoints,
    OnDemandFromRetainedState,
    FailureDiagnosticsOnly,
}

impl SavedOutputPolicy {
    pub const ALL: [Self; 4] = [
        Self::EveryAcceptedPoint,
        Self::SelectedAndFinalPoints,
        Self::OnDemandFromRetainedState,
        Self::FailureDiagnosticsOnly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EveryAcceptedPoint => "Every accepted point",
            Self::SelectedAndFinalPoints => "Selected + final points",
            Self::OnDemandFromRetainedState => "On demand from retained state",
            Self::FailureDiagnosticsOnly => "Failure diagnostics only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPrecision {
    FullSourcePrecision,
    DisplayCacheWithFullSourcePrecision,
}

impl SavedOutputPrecision {
    pub const ALL: [Self; 2] = [
        Self::FullSourcePrecision,
        Self::DisplayCacheWithFullSourcePrecision,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSourcePrecision => "f64 / complex128",
            Self::DisplayCacheWithFullSourcePrecision => {
                "f32 display cache + full source precision"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputStreaming {
    LivePlotAdaptiveDisplayDecimation,
    StoreOnly,
}

impl SavedOutputStreaming {
    pub const ALL: [Self; 2] = [Self::LivePlotAdaptiveDisplayDecimation, Self::StoreOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::LivePlotAdaptiveDisplayDecimation => "Live plot · adaptive display decimation",
            Self::StoreOnly => "Store only",
        }
    }
}

/// Persisted waveform/data contract owned by the simulation plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutput {
    pub id: SavedOutputId,
    pub revision: ObjectRevision,
    pub kind: SavedOutputKind,
    pub name: String,
    pub source_expression: String,
    pub compatible_analyses: SavedOutputCompatibility,
    pub save_policy: SavedOutputPolicy,
    pub stored_precision: SavedOutputPrecision,
    pub streaming: SavedOutputStreaming,
}

impl SavedOutput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        kind: SavedOutputKind,
        name: impl Into<String>,
        source_expression: impl Into<String>,
        compatible_analyses: SavedOutputCompatibility,
        save_policy: SavedOutputPolicy,
        stored_precision: SavedOutputPrecision,
        streaming: SavedOutputStreaming,
    ) -> Result<Self, String> {
        let output = Self {
            id: SavedOutputId::new(),
            revision: ObjectRevision::INITIAL,
            kind,
            name: name.into(),
            source_expression: source_expression.into(),
            compatible_analyses,
            save_policy,
            stored_precision,
            streaming,
        };
        output.validate()?;
        Ok(output)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_bounded_text("name", &self.name, 256, false)?;
        validate_bounded_text(
            "source or expression",
            &self.source_expression,
            8_192,
            false,
        )?;
        validate_saved_output_expression(self.kind, &self.source_expression)
    }

    /// Unit preview derived from the output schema without inspecting a
    /// mutable result dataset. Derived expressions remain explicit until the
    /// calculator's dimensional resolver evaluates their dependencies.
    pub fn inferred_unit(&self) -> &'static str {
        match self.kind {
            SavedOutputKind::RawVoltageOrCurrent
                if self
                    .source_expression
                    .trim()
                    .get(..2)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V(")) =>
            {
                "volts"
            }
            SavedOutputKind::RawVoltageOrCurrent => "amperes",
            SavedOutputKind::DerivedExpression => "resolved from expression",
            SavedOutputKind::DeviceOperatingPointQuantity => "from device quantity",
            SavedOutputKind::NoiseContributor => "V²/Hz or A²/Hz",
            SavedOutputKind::RfPortQuantity => "dimensionless",
        }
    }

    pub const fn status_label(&self) -> &'static str {
        match self.save_policy {
            SavedOutputPolicy::EveryAcceptedPoint => "full capture",
            SavedOutputPolicy::SelectedAndFinalPoints => "selected + final",
            SavedOutputPolicy::OnDemandFromRetainedState => "retained-state derivation",
            SavedOutputPolicy::FailureDiagnosticsOnly => "failure diagnostics",
        }
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = SavedOutputId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let SavedOutputCompatibility::SelectedAnalysis { analysis_id } =
            &mut cloned.compatible_analyses
        {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

const LEGACY_SAVED_OUTPUT_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x75b5_6a7e_614a_5b71_b2dc_32cb_7624_1038);

impl<'de> Deserialize<'de> for SavedOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            #[serde(default = "missing_identity_sentinel")]
            id: serde_json::Value,
            #[serde(default)]
            revision: ObjectRevision,
            kind: SavedOutputKind,
            name: String,
            source_expression: String,
            compatible_analyses: SavedOutputCompatibility,
            save_policy: SavedOutputPolicy,
            stored_precision: SavedOutputPrecision,
            streaming: SavedOutputStreaming,
        }

        let wire = Wire::deserialize(deserializer)?;
        let identity = serde_json::to_vec(&(
            wire.kind,
            &wire.name,
            &wire.source_expression,
            &wire.compatible_analyses,
            wire.save_policy,
            wire.stored_precision,
            wire.streaming,
        ))
        .map_err(D::Error::custom)?;
        let id = deserialize_or_migrate_identity::<SavedOutputId, D::Error>(
            wire.id,
            LEGACY_SAVED_OUTPUT_ID_NAMESPACE,
            &identity,
            SavedOutputId::from_namespace,
        )?;
        Ok(Self {
            id,
            revision: wire.revision,
            kind: wire.kind,
            name: wire.name,
            source_expression: wire.source_expression,
            compatible_analyses: wire.compatible_analyses,
            save_policy: wire.save_policy,
            stored_precision: wire.stored_precision,
            streaming: wire.streaming,
        })
    }
}

const MISSING_IDENTITY_SENTINEL: &str = "__rspice_missing_stable_identity__";

fn missing_identity_sentinel() -> serde_json::Value {
    serde_json::Value::String(MISSING_IDENTITY_SENTINEL.to_owned())
}

fn deserialize_or_migrate_identity<I, E>(
    value: serde_json::Value,
    namespace: Uuid,
    identity: &[u8],
    migrate: fn(Uuid, &[u8]) -> I,
) -> Result<I, E>
where
    I: serde::de::DeserializeOwned,
    E: serde::de::Error,
{
    if value == missing_identity_sentinel() {
        Ok(migrate(namespace, identity))
    } else if value.is_null() {
        Err(E::custom("stable identity must not be null"))
    } else {
        serde_json::from_value(value).map_err(E::custom)
    }
}

fn parse_design_quantity(
    expression: &str,
    quantity: DesignVariableQuantity,
) -> Result<f64, String> {
    use crate::quantity::{
        QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
    };

    let text = expression.trim();
    let (numeric, kind) = match quantity {
        DesignVariableQuantity::Resistance => (
            strip_required_unit(text, &["ohm", "Ω"])
                .ok_or_else(|| "explicit resistance unit required (ohm or Ω)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Capacitance => (
            text.strip_suffix('F')
                .ok_or_else(|| "explicit capacitance unit required (F)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Voltage => (
            text.strip_suffix('V')
                .ok_or_else(|| "explicit voltage unit required (V)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Current => (
            text.strip_suffix('A')
                .ok_or_else(|| "explicit current unit required (A)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Temperature => (text, QuantityInputKind::Temperature),
        DesignVariableQuantity::Dimensionless => (text, QuantityInputKind::EngineeringScalar),
    };
    let numeric = if quantity == DesignVariableQuantity::Dimensionless
        || quantity == DesignVariableQuantity::Temperature
    {
        numeric.trim().to_owned()
    } else {
        normalize_explicit_unit_scalar(numeric.trim())
    };
    parse_ui_quantity(
        &numeric,
        kind,
        QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    )
    .map_err(|error| error.to_string())
}

fn normalize_explicit_unit_scalar(value: &str) -> String {
    if let Some(prefix) = value.strip_suffix('M') {
        format!("{}Meg", prefix.trim_end())
    } else {
        value.to_owned()
    }
}

fn strip_required_unit<'a>(value: &'a str, units: &[&str]) -> Option<&'a str> {
    units.iter().find_map(|unit| {
        if unit.is_ascii() {
            value
                .get(value.len().saturating_sub(unit.len())..)
                .filter(|suffix| suffix.eq_ignore_ascii_case(unit))
                .map(|_| &value[..value.len() - unit.len()])
        } else {
            value.strip_suffix(unit)
        }
    })
}

fn validate_saved_output_expression(kind: SavedOutputKind, expression: &str) -> Result<(), String> {
    let expression = expression.trim();
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => validate_raw_probe(expression),
        SavedOutputKind::DerivedExpression => parse_calculator_expression(expression),
        SavedOutputKind::DeviceOperatingPointQuantity => validate_device_op_probe(expression),
        SavedOutputKind::NoiseContributor => {
            if validate_hierarchical_token(expression).is_ok() {
                Ok(())
            } else {
                parse_calculator_expression(expression)
            }
        }
        SavedOutputKind::RfPortQuantity => {
            if !expression
                .get(..2)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
            {
                return Err("RF port quantity must use S(port, port) syntax".to_owned());
            }
            parse_calculator_expression(expression)
        }
    }
}

fn parse_calculator_expression(expression: &str) -> Result<(), String> {
    crate::analysis::calculator::parser::Parser::new(expression)
        .try_parse()
        .map(|_| ())
        .map_err(|error| format!("expression is invalid: {error}"))
}

pub(crate) fn validate_raw_probe(expression: &str) -> Result<(), String> {
    let open = expression
        .find('(')
        .ok_or_else(|| "raw output must use V(node), V(node+, node-), or I(source)".to_owned())?;
    if !expression.ends_with(')') {
        return Err("raw output has an unterminated probe".to_owned());
    }
    let function = &expression[..open];
    let arguments = &expression[open + 1..expression.len() - 1];
    let arguments = arguments.split(',').map(str::trim).collect::<Vec<_>>();
    if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2)
        || function.eq_ignore_ascii_case("I") && arguments.len() == 1
    {
        for argument in arguments {
            validate_hierarchical_token(argument).map_err(|error| {
                format!("raw output must use V(node), V(node+, node-), or I(source): {error}")
            })?;
        }
        Ok(())
    } else {
        Err("raw output must use V(node), V(node+, node-), or I(source)".to_owned())
    }
}

fn validate_device_op_probe(expression: &str) -> Result<(), String> {
    let Some(body) = expression.strip_prefix('@') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    let Some(open) = body.find('[') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    if !body.ends_with(']') {
        return Err("device operating-point quantity has an unterminated quantity".to_owned());
    }
    validate_hierarchical_token(&body[..open])?;
    validate_parameter_name(&body[open + 1..body.len() - 1])
        .map_err(|error| format!("device quantity is invalid: {error}"))
}

fn validate_hierarchical_token(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("probe target is required".to_owned());
    }
    if let Some(character) = value.chars().find(|character| {
        !character.is_ascii_alphanumeric()
            && !matches!(character, '_' | '.' | ':' | '/' | '$' | '+' | '-')
    }) {
        return Err(format!(
            "probe target contains unsupported character {character:?}"
        ));
    }
    Ok(())
}

fn validate_parameter_name(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("name is required".to_owned());
    }
    if value.len() > 128 {
        return Err("name exceeds 128 bytes".to_owned());
    }
    let mut characters = value.chars();
    let first = characters.next().expect("empty handled above");
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err("name must begin with an ASCII letter or underscore".to_owned());
    }
    if let Some(character) =
        characters.find(|character| !character.is_ascii_alphanumeric() && *character != '_')
    {
        return Err(format!(
            "name contains unsupported character {character:?}; use ASCII letters, digits, and underscores"
        ));
    }
    Ok(())
}

fn validate_single_line_expression(label: &str, value: &str) -> Result<(), String> {
    validate_bounded_text(label, value, 8_192, false)?;
    if value
        .chars()
        .any(|character| matches!(character, '\r' | '\n'))
    {
        return Err(format!("{label} must be a single line"));
    }
    Ok(())
}

fn validate_bounded_text(
    label: &str,
    value: &str,
    maximum_bytes: usize,
    allow_empty: bool,
) -> Result<(), String> {
    if !allow_empty && value.trim().is_empty() {
        return Err(format!("{label} is required"));
    }
    if value.len() > maximum_bytes {
        return Err(format!("{label} exceeds {maximum_bytes} bytes"));
    }
    if let Some(character) = value
        .chars()
        .find(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    {
        return Err(format!("{label} contains control character {character:?}"));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SimulationConfigurationError {
    #[error("project configuration-set catalog is invalid: {message}")]
    InvalidConfigurationSetCatalog { message: String },
    #[error("project design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("project connectivity contract is invalid: {message}")]
    InvalidConnectivityContract { message: String },
    #[error("project-owned netlist document is invalid: {message}")]
    InvalidNetlistDocumentProjection { message: String },
    #[error("project-owned Code source registry is invalid: {message}")]
    InvalidProjectSourceRegistry { message: String },
    #[error("project plot export preset catalog has invalid ownership: {message}")]
    InvalidPlotExportPresetOwnership { message: String },
    #[error("project hardcopy source-set catalog is invalid: {message}")]
    InvalidHardcopySourceSetCatalog { message: String },
    #[error("report_documents[{index}] is invalid: {message}")]
    InvalidReportDocument { index: usize, message: String },
    #[error("report document identity {document_id} is duplicated")]
    DuplicateReportDocumentIdentity { document_id: ResultDocumentId },
    #[error("simulation_plan_payloads contains duplicate owner {plan_id}")]
    DuplicatePlanPayload { plan_id: SimulationPlanId },
    #[error("simulation_plan_payloads[{plan_id}].design_variables[{index}] is invalid: {message}")]
    InvalidDesignVariable {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].design_variables[{index}] duplicates the case-insensitive name of design_variables[{first_index}]"
    )]
    DuplicateDesignVariableName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("design variable identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateDesignVariableIdentity {
        id: DesignVariableId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error("simulation_plan_payloads[{plan_id}].saved_outputs[{index}] is invalid: {message}")]
    InvalidSavedOutput {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].saved_outputs[{index}] duplicates the case-insensitive name of saved_outputs[{first_index}]"
    )]
    DuplicateSavedOutputName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("saved output identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateSavedOutputIdentity {
        id: SavedOutputId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error("simulation_plan_payloads[{plan_id}].specs[{index}] is invalid: {message}")]
    InvalidSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specs[{index}] duplicates the case-insensitive measurement of specs[{first_index}]"
    )]
    DuplicateSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] is invalid: {message}"
    )]
    InvalidRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] duplicates target owned by entry {first_index}"
    )]
    DuplicateRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("simulation plan {plan_id} already owns a design variable named '{name}'")]
    DesignVariableNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no design variable with identity {variable_id}")]
    DesignVariableNotFound {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error(
        "design variable {variable_id} in simulation plan {plan_id} could not advance its revision: {source}"
    )]
    DesignVariableRevision {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
        #[source]
        source: RevisionError,
    },
    #[error(
        "design variable {variable_id} is repeated in one update transaction for simulation plan {plan_id}"
    )]
    DuplicateDesignVariableUpdate {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error("simulation plan {plan_id} already owns a saved output named '{name}'")]
    SavedOutputNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no configuration payload")]
    PlanPayloadMissing { plan_id: SimulationPlanId },
    #[error("simulation plan {plan_id} already has a configuration payload")]
    PlanPayloadAlreadyExists { plan_id: SimulationPlanId },
    #[error("cloned plan payload has no destination mapping for source analysis {analysis_id}")]
    MissingClonedAnalysisMapping { analysis_id: AnalysisInstanceId },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectConfigurationMutationError {
    #[error("configuration-set catalog is invalid: {0}")]
    InvalidCatalog(#[from] crate::state::ConfigurationSetError),
    #[error("design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("configuration '{configuration}' root {root} is not a schematic or testbench view")]
    UnsupportedRootView { configuration: String, root: String },
    #[error("configuration '{configuration}' root {root} has no authoritative schematic buffer")]
    MissingRootBuffer { configuration: String, root: String },
    #[error("project revision could not advance: {0}")]
    ProjectRevision(#[from] RevisionError),
    #[error("configuration-set transaction has no semantic changes")]
    NoChanges,
}

pub const MAX_PROJECT_HARDCOPY_SOURCE_SETS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HardcopySourceSetPersistenceError {
    #[error("hardcopy source set is invalid: {message}")]
    Invalid { message: String },
    #[error(
        "project hardcopy source-set catalog is full ({MAX_PROJECT_HARDCOPY_SOURCE_SETS} sets)"
    )]
    CatalogFull,
    #[error("hardcopy source-set name '{name}' is already owned by another retained set")]
    DuplicateName { name: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionComparisonMethod {
    AbsoluteRelativeEnvelope,
    PointwiseRelative,
}

impl RegressionComparisonMethod {
    pub const ALL: [Self; 2] = [Self::AbsoluteRelativeEnvelope, Self::PointwiseRelative];

    pub const fn label(self) -> &'static str {
        match self {
            Self::AbsoluteRelativeEnvelope => "Absolute + relative envelope",
            Self::PointwiseRelative => "Pointwise relative",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionTargetKind {
    Measurement,
    Waveform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionTargetSelector {
    pub source_domain: AnalysisResultSourceDomain,
    pub source_instance_id: AnalysisInstanceId,
    pub kind: RegressionTargetKind,
    pub name: String,
    pub occurrence: u32,
}

impl RegressionTargetSelector {
    fn validate(&self) -> Result<(), String> {
        if self.source_domain == AnalysisResultSourceDomain::LegacyUnclassified {
            return Err(
                "legacy-unclassified result sources cannot own regression policy".to_owned(),
            );
        }
        if self.name.trim().is_empty() {
            return Err("target name must not be empty".to_owned());
        }
        if self.name != self.name.trim() {
            return Err("target name must not have surrounding whitespace".to_owned());
        }
        if self.name.chars().any(char::is_control) {
            return Err("target name must not contain control characters".to_owned());
        }
        if self.name.graphemes(true).count() > 256 {
            return Err("target name exceeds 256 grapheme clusters".to_owned());
        }
        Ok(())
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        if self.source_domain == AnalysisResultSourceDomain::SimulationPlan {
            cloned.source_instance_id = analysis_identity_map
                .get(&self.source_instance_id)
                .copied()
                .ok_or(self.source_instance_id)?;
        }
        Ok(cloned)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionComparisonWindow {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionToleranceRule {
    pub target: RegressionTargetSelector,
    pub method: RegressionComparisonMethod,
    /// Absolute value-domain tolerance in the target's retained base unit.
    pub absolute_tolerance: f64,
    /// Relative tolerance as a fraction (`0.005` = `0.5%`).
    pub relative_tolerance: f64,
    /// Maximum horizontal displacement in the waveform X-axis base unit.
    pub time_skew_allowance: f64,
    /// Optional inclusive X-axis comparison window. Measurements use `None`.
    pub comparison_window: Option<RegressionComparisonWindow>,
}

impl RegressionToleranceRule {
    pub fn validate(&self) -> Result<(), String> {
        self.target.validate()?;
        for (label, value) in [
            ("absolute tolerance", self.absolute_tolerance),
            ("relative tolerance", self.relative_tolerance),
            ("time-skew allowance", self.time_skew_allowance),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("{label} must be finite and nonnegative"));
            }
        }
        if self.target.kind == RegressionTargetKind::Measurement
            && (self.time_skew_allowance != 0.0 || self.comparison_window.is_some())
        {
            return Err(
                "measurement targets cannot define time skew or a comparison window".to_owned(),
            );
        }
        if let Some(window) = self.comparison_window {
            if !window.start.is_finite() || !window.end.is_finite() {
                return Err("comparison-window bounds must be finite".to_owned());
            }
            if window.start > window.end {
                return Err("comparison-window start must not exceed its end".to_owned());
            }
        }
        Ok(())
    }

    fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.target = self.target.cloned_for_new_plan(analysis_identity_map)?;
        Ok(cloned)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayload {
    #[serde(default)]
    pub design_variables: Vec<DesignVariable>,
    #[serde(default)]
    pub saved_outputs: Vec<SavedOutput>,
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    #[serde(default)]
    pub regression_baseline_run: Option<RunId>,
    #[serde(default)]
    pub regression_tolerances: Vec<RegressionToleranceRule>,
}

/// Vec-backed because product UUID wrappers intentionally do not define an
/// ordering. Validation guarantees unique owners; lifecycle hashing sorts a
/// canonical projection by UUID bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayloadRecord {
    pub plan_id: SimulationPlanId,
    pub payload: SimulationPlanPayload,
}

/// Mockup-specified ownership strategy for a project-owned SPICE artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum OwnedNetlistEditStrategy {
    #[default]
    OwnedSource,
    ParameterOptionOverride,
    IncludeOrderOverride,
    AnalysisOnlyDeck,
}

impl OwnedNetlistEditStrategy {
    pub const ALL: [Self; 4] = [
        Self::OwnedSource,
        Self::ParameterOptionOverride,
        Self::IncludeOrderOverride,
        Self::AnalysisOnlyDeck,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::OwnedSource => "Owned source derived from generated output",
            Self::ParameterOptionOverride => "Parameter and option override",
            Self::IncludeOrderOverride => "Include-order override",
            Self::AnalysisOnlyDeck => "Analysis-only deck",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistSaveRecord {
    pub document_revision: u64,
    pub content_digest: crate::product::ContentDigest,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistDescriptor {
    pub artifact_name: String,
    pub strategy: OwnedNetlistEditStrategy,
    #[serde(default)]
    pub save_history: Vec<OwnedNetlistSaveRecord>,
}

// Re-export the source bundle API from its historical workspace path so
// downstream integrations keep compiling while the implementation remains an
// independently testable state subsystem.
pub use super::project_sources::{
    MAX_PROJECT_CODE_SOURCE_BYTES, MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    MAX_PROJECT_SOURCE_DEPENDENCIES, MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceDocument, ProjectSourceError,
    ProjectSourceFile, ProjectSourceId, ProjectSourceIdError, ProjectSourceIdParseError,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    ProjectSourceValidationIdentity, project_veriloga_bundle_alias,
    project_veriloga_bundle_source_key,
};

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
    /// Project-owned hierarchy/view-resolution authority. Empty catalogs
    /// preserve legacy deterministic resolution; once populated, the active
    /// configuration is the exact authority used by preflight and netlisting.
    #[serde(default)]
    pub configuration_sets: crate::state::ConfigurationSetCatalog,
    /// Project-owned schematic sheet, assembly variant, annotation, and
    /// hierarchy-audit authority. The catalog is deliberately separate from
    /// simulation configuration sets: it describes design identity, while a
    /// configuration set describes how that identity is executed.
    #[serde(default)]
    pub design_management: crate::state::DesignManagementCatalog,
    /// Project-owned bundle mapping and global-net policy. Older projects
    /// migrate to strict fail-closed defaults instead of inheriting UI state.
    #[serde(default)]
    pub connectivity: crate::state::ConnectivityContract,
    pub active_view: CellViewRef,
    pub open_views: Vec<OpenCellView>,
    pub hierarchy_stack: Vec<CellViewRef>,
    /// Instance names descended through, aligned with
    /// `hierarchy_stack[1..]`: entry N-1 is the instance whose master is
    /// `hierarchy_stack[N]`. Older saves default to empty; rendering
    /// falls back to cell names per entry.
    #[serde(default)]
    pub hierarchy_instances: Vec<String>,
    pub schematic_buffers: HashMap<String, SchematicState>,
    /// Measurement specifications for the results specs matrix. Project
    /// design intent, so it persists with the workspace.
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Plan-owned variables, output contracts, and specifications. Projects
    /// predating this feature migrate the active legacy `specs` projection
    /// into one record after execution-context migration.
    #[serde(default)]
    pub simulation_plan_payloads: Vec<SimulationPlanPayloadRecord>,
    /// Project-owned, versioned publication profiles for result plots.
    /// Personal profiles are owned by serialized user preferences; an
    /// organization profile requires a connected organization authority.
    #[serde(default)]
    pub plot_export_presets: crate::results::plot_export_preset::PlotExportPresetCatalog,
    /// Versioned, per-document Page Setup contracts used by schematic,
    /// symbol, result, and report hardcopy workflows. Publication artifacts
    /// and transient preview state are intentionally not persisted here.
    #[serde(default)]
    pub hardcopy_setups: crate::hardcopy::HardcopySetupStore,
    /// Reusable print-mapping sets owned by this project. Personal portable
    /// presets are persisted by `UserPreferences`; document mappings remain
    /// embedded in `hardcopy_setups` for reproducible publication.
    #[serde(default)]
    pub project_print_mappings: crate::hardcopy::PrintMappingPresetCatalog,
    /// Project-owned named engineering-table views. Working and personal
    /// views are device preferences; only explicitly project-scoped views
    /// participate in project revisioning and collaboration.
    #[serde(default)]
    pub engineering_table_views: crate::state::EngineeringTableViewStore,
    /// Ordered, exact source aggregates used by all-sheets/all-panes and
    /// named print-set publication. Every member pins its document revision
    /// and content digest; stale members fail closed when resolved.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    hardcopy_source_sets: Vec<crate::hardcopy::sources::HardcopySourceSet>,
    /// Project-owned, versioned engineering report sources. Rendered review
    /// artifacts are derived from these documents and are never represented
    /// here unless a publication writer has produced and verified them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_documents: Vec<crate::results::report_document::ReportDocument>,
    /// Manually edited netlist source. When set, simulations run this
    /// deck instead of regenerating from the schematic (text-first mode);
    /// `None` means the netlist view shows the generated artifact.
    #[serde(default)]
    pub netlist_source: Option<String>,
    /// Canonical owned-source identity, provenance, generated base, sealed
    /// dependency metadata, revision, and validation evidence. The legacy
    /// `netlist_source` projection remains for backwards compatibility and
    /// must exactly match this document when both are present.
    #[serde(default)]
    pub netlist_document: Option<crate::state::NetlistDocument>,
    /// Ownership-dialog selection for the project-owned source artifact.
    #[serde(default)]
    pub netlist_descriptor: Option<OwnedNetlistDescriptor>,
    /// Project-owned source documents shown by the Verilog-A and Automation
    /// pages of the Code workspace. Older projects intentionally restore an
    /// empty registry rather than receiving demonstration content.
    #[serde(default, skip_serializing_if = "ProjectSourceRegistry::is_empty")]
    pub project_sources: ProjectSourceRegistry,
    /// Native filesystem origin for `netlist_source`, used to resolve relative
    /// `.include`/`.lib` paths for imported decks. Edits retain this origin:
    /// changing document bytes does not change the directory against which its
    /// authored relative dependencies resolve. Browser imports have no native
    /// path authority and therefore leave it absent.
    #[serde(default)]
    pub netlist_source_path: Option<PathBuf>,
    /// Runtime dirty bit for `netlist_source`; skipped because dirty state is
    /// session-local while the source itself is persisted with the project.
    #[serde(default, skip)]
    pub netlist_source_dirty: bool,
    /// Runtime dirty state for `project_sources`. Source bytes and validation
    /// identities persist; dirty state is derived against the accepted project
    /// and remains session-local.
    #[serde(default, skip)]
    pub project_sources_dirty: bool,
    /// Runtime dirty state for project-owned metadata such as an exact
    /// technology attachment. The binding itself persists in `project`.
    #[serde(default, skip)]
    #[doc(hidden)]
    pub project_metadata_dirty: bool,
    /// Runtime dirty projection for project-owned report sources. The report
    /// documents themselves persist; accepted-baseline comparison remains the
    /// canonical save/revert authority.
    #[serde(default, skip)]
    pub report_documents_dirty: bool,
    /// Runtime dirty projection for committed per-document page setups.
    #[serde(default, skip)]
    pub hardcopy_setups_dirty: bool,
    /// Runtime dirty projection for reusable project-owned print mappings.
    #[serde(default, skip)]
    pub project_print_mappings_dirty: bool,
    /// Runtime dirty projection for project-owned hardcopy source sets.
    #[serde(default, skip)]
    hardcopy_source_sets_dirty: bool,
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let active_view = CellViewRef::default_top();
        let mut schematic_buffers = HashMap::new();
        schematic_buffers.insert(active_view.key(), SchematicState::default());

        Self {
            project: ProjectDescriptor::default(),
            configuration_sets: crate::state::ConfigurationSetCatalog::default(),
            design_management: crate::state::DesignManagementCatalog::default(),
            connectivity: crate::state::ConnectivityContract::default(),
            active_view: active_view.clone(),
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            hierarchy_stack: vec![active_view],
            hierarchy_instances: Vec::new(),
            schematic_buffers,
            specs: Vec::new(),
            simulation_plan_payloads: Vec::new(),
            plot_export_presets:
                crate::results::plot_export_preset::PlotExportPresetCatalog::default(),
            hardcopy_setups: crate::hardcopy::HardcopySetupStore::default(),
            project_print_mappings: crate::hardcopy::PrintMappingPresetCatalog::new(
                crate::hardcopy::PrintMappingCatalogOwner::Project,
            ),
            engineering_table_views: crate::state::EngineeringTableViewStore::default(),
            hardcopy_source_sets: Vec::new(),
            report_documents: Vec::new(),
            netlist_source: None,
            netlist_document: None,
            netlist_descriptor: None,
            project_sources: ProjectSourceRegistry::default(),
            netlist_source_path: None,
            netlist_source_dirty: false,
            project_sources_dirty: false,
            project_metadata_dirty: false,
            report_documents_dirty: false,
            hardcopy_setups_dirty: false,
            project_print_mappings_dirty: false,
            hardcopy_source_sets_dirty: false,
        }
    }
}

fn validate_hardcopy_source_set_catalog(
    source_sets: &[crate::hardcopy::sources::HardcopySourceSet],
) -> Result<(), HardcopySourceSetPersistenceError> {
    if source_sets.len() > MAX_PROJECT_HARDCOPY_SOURCE_SETS {
        return Err(HardcopySourceSetPersistenceError::CatalogFull);
    }
    let mut source_keys = std::collections::HashSet::with_capacity(source_sets.len());
    let mut folded_names = std::collections::HashSet::with_capacity(source_sets.len());
    for source_set in source_sets {
        source_set
            .validate()
            .map_err(|error| HardcopySourceSetPersistenceError::Invalid {
                message: error.to_string(),
            })?;
        if !source_keys.insert(source_set.source_key()) {
            return Err(HardcopySourceSetPersistenceError::Invalid {
                message: format!("source identity {} is duplicated", source_set.source_key()),
            });
        }
        if !folded_names.insert(source_set.name().to_lowercase()) {
            return Err(HardcopySourceSetPersistenceError::DuplicateName {
                name: source_set.name().to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_connectivity_contract_references(
    workspace: &ProjectWorkspace,
) -> Result<(), SimulationConfigurationError> {
    let mut nets_by_view = HashMap::<&str, HashSet<String>>::new();
    for (view_key, schematic) in &workspace.schematic_buffers {
        nets_by_view.insert(
            view_key.as_str(),
            crate::simulation::netlist_gen::design_nets(schematic)
                .into_iter()
                .map(|net| net.name)
                .collect(),
        );
    }
    for (bundle_index, bundle) in workspace.connectivity.named_bundles.iter().enumerate() {
        for (member_index, member) in bundle.members.iter().enumerate() {
            let Some(nets) = nets_by_view.get(member.target.view_key.as_str()) else {
                return Err(SimulationConfigurationError::InvalidConnectivityContract {
                    message: format!(
                        "named_bundles[{bundle_index}].members[{member_index}] references missing schematic view '{}'",
                        member.target.view_key
                    ),
                });
            };
            if !nets.contains(&member.target.net_name) {
                return Err(SimulationConfigurationError::InvalidConnectivityContract {
                    message: format!(
                        "named_bundles[{bundle_index}].members[{member_index}] references missing exact net '{}::{}'",
                        member.target.view_key, member.target.net_name
                    ),
                });
            }
        }
    }
    Ok(())
}

impl ProjectWorkspace {
    /// Validate the persisted simulation configuration without requiring any
    /// runtime editor state. Cross-document targets are validated by project
    /// I/O once the library tree and simulation plan are available.
    pub fn validate_simulation_configuration(&self) -> Result<(), SimulationConfigurationError> {
        self.configuration_sets.validate().map_err(|error| {
            SimulationConfigurationError::InvalidConfigurationSetCatalog {
                message: error.to_string(),
            }
        })?;
        self.design_management.validate().map_err(|error| {
            SimulationConfigurationError::InvalidDesignManagementCatalog {
                message: error.to_string(),
            }
        })?;
        self.connectivity.validate().map_err(|message| {
            SimulationConfigurationError::InvalidConnectivityContract { message }
        })?;
        validate_connectivity_contract_references(self)?;
        self.plot_export_presets
            .validate_ownership_scope(
                crate::results::plot_export_preset::PlotExportPresetScope::Project,
            )
            .map_err(
                |error| SimulationConfigurationError::InvalidPlotExportPresetOwnership {
                    message: error.to_string(),
                },
            )?;
        validate_hardcopy_source_set_catalog(&self.hardcopy_source_sets).map_err(|error| {
            SimulationConfigurationError::InvalidHardcopySourceSetCatalog {
                message: error.to_string(),
            }
        })?;
        let mut report_document_ids = std::collections::HashSet::new();
        for (index, document) in self.report_documents.iter().enumerate() {
            document.validate().map_err(|error| {
                SimulationConfigurationError::InvalidReportDocument {
                    index,
                    message: error.to_string(),
                }
            })?;
            if !report_document_ids.insert(document.id()) {
                return Err(
                    SimulationConfigurationError::DuplicateReportDocumentIdentity {
                        document_id: document.id(),
                    },
                );
            }
        }
        self.project_sources.validate().map_err(|error| {
            SimulationConfigurationError::InvalidProjectSourceRegistry {
                message: error.to_string(),
            }
        })?;
        if let Some(document) = &self.netlist_document {
            if document.ownership()
                == crate::state::DocumentOwnership::Generated
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "project-owned netlist document cannot have generated ownership"
                            .to_owned(),
                    },
                );
            }
            if self.netlist_source.as_deref() != Some(document.source()) {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "canonical document bytes differ from netlist_source".to_owned(),
                    },
                );
            }
            let descriptor = self.netlist_descriptor.as_ref().ok_or_else(|| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "canonical document has no owned-artifact descriptor".to_owned(),
                }
            })?;
            let name = descriptor.artifact_name.trim();
            if name.is_empty()
                || name != descriptor.artifact_name
                || name.chars().any(char::is_control)
                || name.contains('/')
                || name.contains('\\')
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned artifact name must be one trimmed file name".to_owned(),
                    },
                );
            }
            let mut previous_revision = 0_u64;
            for record in &descriptor.save_history {
                if record.document_revision == 0
                    || record.document_revision <= previous_revision
                    || record.document_revision > document.revision().get()
                    || record.message.trim().is_empty()
                    || record.message != record.message.trim()
                    || record.message.chars().any(char::is_control)
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned source save history is not strictly revision ordered or has an invalid message".to_owned(),
                        },
                    );
                }
                previous_revision = record.document_revision;
            }
        } else if self.netlist_descriptor.is_some() {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "owned-artifact descriptor has no canonical document".to_owned(),
                },
            );
        }

        let mut plan_ids = HashMap::<SimulationPlanId, usize>::new();
        let mut variable_ids = HashMap::<DesignVariableId, SimulationPlanId>::new();
        let mut output_ids = HashMap::<SavedOutputId, SimulationPlanId>::new();
        for (record_index, record) in self.simulation_plan_payloads.iter().enumerate() {
            let plan_id = record.plan_id;
            if plan_ids.insert(plan_id, record_index).is_some() {
                return Err(SimulationConfigurationError::DuplicatePlanPayload { plan_id });
            }

            let mut variable_names = HashMap::<String, usize>::new();
            for (index, variable) in record.payload.design_variables.iter().enumerate() {
                variable.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidDesignVariable {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_plan_id) = variable_ids.insert(variable.id, plan_id) {
                    return Err(
                        SimulationConfigurationError::DuplicateDesignVariableIdentity {
                            id: variable.id,
                            first_plan_id,
                            plan_id,
                        },
                    );
                }
                let canonical = variable.name.to_ascii_lowercase();
                if let Some(first_index) = variable_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateDesignVariableName {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut output_names = HashMap::<String, usize>::new();
            for (index, output) in record.payload.saved_outputs.iter().enumerate() {
                output.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidSavedOutput {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_plan_id) = output_ids.insert(output.id, plan_id) {
                    return Err(SimulationConfigurationError::DuplicateSavedOutputIdentity {
                        id: output.id,
                        first_plan_id,
                        plan_id,
                    });
                }
                let canonical = output.name.to_lowercase();
                if let Some(first_index) = output_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateSavedOutputName {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut specification_names = HashMap::<String, usize>::new();
            for (index, specification) in record.payload.specs.iter().enumerate() {
                specification.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidSpecification {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                let canonical = specification.measurement.to_ascii_lowercase();
                if let Some(first_index) = specification_names.insert(canonical, index) {
                    return Err(SimulationConfigurationError::DuplicateSpecification {
                        plan_id,
                        index,
                        first_index,
                    });
                }
            }

            let mut regression_targets = Vec::<&RegressionTargetSelector>::new();
            for (index, tolerance) in record.payload.regression_tolerances.iter().enumerate() {
                tolerance.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidRegressionTolerance {
                        plan_id,
                        index,
                        message,
                    }
                })?;
                if let Some(first_index) = regression_targets
                    .iter()
                    .position(|target| **target == tolerance.target)
                {
                    return Err(SimulationConfigurationError::DuplicateRegressionTolerance {
                        plan_id,
                        index,
                        first_index,
                    });
                }
                regression_targets.push(&tolerance.target);
            }
        }
        Ok(())
    }

    pub fn active_plan_data(&self, plan_id: SimulationPlanId) -> Option<&SimulationPlanPayload> {
        self.simulation_plan_payloads
            .iter()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &record.payload)
    }

    pub fn active_plan_data_mut(
        &mut self,
        plan_id: SimulationPlanId,
    ) -> Option<&mut SimulationPlanPayload> {
        self.simulation_plan_payloads
            .iter_mut()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &mut record.payload)
    }

    /// Deterministically seed a plan payload from the legacy active specs
    /// projection. Existing plan-owned data always wins.
    pub fn migrate_active_plan_data(&mut self, plan_id: SimulationPlanId) {
        if self.active_plan_data(plan_id).is_none() {
            self.simulation_plan_payloads
                .push(SimulationPlanPayloadRecord {
                    plan_id,
                    payload: SimulationPlanPayload {
                        specs: self.specs.clone(),
                        ..SimulationPlanPayload::default()
                    },
                });
        }
        self.sync_legacy_specs_projection(plan_id);
    }

    /// Seed a retained inactive plan without copying the active plan's legacy
    /// specification projection into a different ownership boundary.
    pub fn migrate_inactive_plan_data(&mut self, plan_id: SimulationPlanId) {
        if self.active_plan_data(plan_id).is_none() {
            self.simulation_plan_payloads
                .push(SimulationPlanPayloadRecord {
                    plan_id,
                    payload: SimulationPlanPayload::default(),
                });
        }
    }

    pub fn ensure_active_plan_data(
        &mut self,
        plan_id: SimulationPlanId,
    ) -> &mut SimulationPlanPayload {
        self.migrate_active_plan_data(plan_id);
        self.active_plan_data_mut(plan_id)
            .expect("migration inserts the requested plan payload")
    }

    pub fn active_specs(&self, plan_id: SimulationPlanId) -> &[SpecEntry] {
        self.active_plan_data(plan_id)
            .map_or(self.specs.as_slice(), |payload| payload.specs.as_slice())
    }

    pub fn replace_active_specs(&mut self, plan_id: SimulationPlanId, specs: Vec<SpecEntry>) {
        self.ensure_active_plan_data(plan_id).specs = specs.clone();
        self.specs = specs;
    }

    pub fn sync_legacy_specs_projection(&mut self, plan_id: SimulationPlanId) {
        if let Some(specs) = self
            .active_plan_data(plan_id)
            .map(|payload| payload.specs.clone())
        {
            self.specs = specs;
        }
    }

    pub fn add_design_variable(
        &mut self,
        plan_id: SimulationPlanId,
        variable: DesignVariable,
    ) -> Result<(), SimulationConfigurationError> {
        variable.validate().map_err(|message| {
            SimulationConfigurationError::InvalidDesignVariable {
                plan_id,
                index: self
                    .active_plan_data(plan_id)
                    .map_or(0, |payload| payload.design_variables.len()),
                message,
            }
        })?;
        let payload = self.ensure_active_plan_data(plan_id);
        if payload
            .design_variables
            .iter()
            .any(|existing| existing.name.eq_ignore_ascii_case(&variable.name))
        {
            return Err(SimulationConfigurationError::DesignVariableNameConflict {
                plan_id,
                name: variable.name,
            });
        }
        payload.design_variables.push(variable);
        Ok(())
    }

    /// Replace the expression of one plan-owned design variable as a single
    /// validated workspace transaction.
    ///
    /// The stable identity and all engineering metadata are retained. The
    /// variable revision advances exactly once for every committed update.
    /// Validation runs against a complete cloned workspace before assignment,
    /// so a malformed expression, range violation, revision exhaustion, or
    /// unrelated configuration invariant leaves the source workspace intact.
    pub fn update_design_variable_expression(
        &mut self,
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
        expression: impl Into<String>,
    ) -> Result<ObjectRevision, SimulationConfigurationError> {
        let revisions =
            self.update_design_variable_expressions(plan_id, &[(variable_id, expression.into())])?;
        Ok(revisions[0].1)
    }

    /// Atomically replace multiple expressions in one active-plan payload.
    /// Every target is resolved and validated in the same candidate workspace,
    /// which avoids partial commits and scales independently of update count.
    pub fn update_design_variable_expressions(
        &mut self,
        plan_id: SimulationPlanId,
        updates: &[(DesignVariableId, String)],
    ) -> Result<Vec<(DesignVariableId, ObjectRevision)>, SimulationConfigurationError> {
        let mut candidate = self.clone();
        let payload = candidate
            .active_plan_data_mut(plan_id)
            .ok_or(SimulationConfigurationError::PlanPayloadMissing { plan_id })?;
        let mut seen = std::collections::HashSet::with_capacity(updates.len());
        let mut revisions = Vec::with_capacity(updates.len());
        for (variable_id, expression) in updates {
            if !seen.insert(*variable_id) {
                return Err(
                    SimulationConfigurationError::DuplicateDesignVariableUpdate {
                        plan_id,
                        variable_id: *variable_id,
                    },
                );
            }
            let index = payload
                .design_variables
                .iter()
                .position(|variable| variable.id == *variable_id)
                .ok_or(SimulationConfigurationError::DesignVariableNotFound {
                    plan_id,
                    variable_id: *variable_id,
                })?;
            let variable = &mut payload.design_variables[index];
            let next_revision = variable.revision.next().map_err(|source| {
                SimulationConfigurationError::DesignVariableRevision {
                    plan_id,
                    variable_id: *variable_id,
                    source,
                }
            })?;
            variable.expression.clone_from(expression);
            variable.revision = next_revision;
            variable.validate().map_err(|message| {
                SimulationConfigurationError::InvalidDesignVariable {
                    plan_id,
                    index,
                    message,
                }
            })?;
            revisions.push((*variable_id, next_revision));
        }
        candidate.validate_simulation_configuration()?;

        *self = candidate;
        Ok(revisions)
    }

    pub fn add_saved_output(
        &mut self,
        plan_id: SimulationPlanId,
        output: SavedOutput,
    ) -> Result<(), SimulationConfigurationError> {
        output
            .validate()
            .map_err(|message| SimulationConfigurationError::InvalidSavedOutput {
                plan_id,
                index: self
                    .active_plan_data(plan_id)
                    .map_or(0, |payload| payload.saved_outputs.len()),
                message,
            })?;
        let payload = self.ensure_active_plan_data(plan_id);
        if payload
            .saved_outputs
            .iter()
            .any(|existing| existing.name.eq_ignore_ascii_case(&output.name))
        {
            return Err(SimulationConfigurationError::SavedOutputNameConflict {
                plan_id,
                name: output.name,
            });
        }
        payload.saved_outputs.push(output);
        Ok(())
    }

    /// Copy or initialize the payload for a newly cloned plan. The insertion
    /// is atomic with respect to validation: no target record is created on
    /// any error.
    pub fn clone_plan_data(
        &mut self,
        source_plan_id: SimulationPlanId,
        cloned_plan_id: SimulationPlanId,
        copy_variables_outputs_specs: bool,
        copy_regression_baseline: bool,
        analysis_identity_map: &[(AnalysisInstanceId, AnalysisInstanceId)],
    ) -> Result<(), SimulationConfigurationError> {
        if self.active_plan_data(cloned_plan_id).is_some() {
            return Err(SimulationConfigurationError::PlanPayloadAlreadyExists {
                plan_id: cloned_plan_id,
            });
        }
        let source = (copy_variables_outputs_specs || copy_regression_baseline)
            .then(|| {
                self.active_plan_data(source_plan_id).cloned().ok_or(
                    SimulationConfigurationError::PlanPayloadMissing {
                        plan_id: source_plan_id,
                    },
                )
            })
            .transpose()?;
        let remap = analysis_identity_map
            .iter()
            .copied()
            .collect::<HashMap<_, _>>();
        let (design_variables, saved_outputs, specs) = if copy_variables_outputs_specs {
            let source = source.as_ref().expect("copy request resolves source above");
            let design_variables = source
                .design_variables
                .iter()
                .map(|variable| variable.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            let saved_outputs = source
                .saved_outputs
                .iter()
                .map(|output| output.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            (design_variables, saved_outputs, source.specs.clone())
        } else {
            (Vec::new(), Vec::new(), Vec::new())
        };
        let (regression_baseline_run, regression_tolerances) = if copy_regression_baseline {
            let source = source.as_ref().expect("copy request resolves source above");
            let tolerances = source
                .regression_tolerances
                .iter()
                .map(|rule| rule.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            (source.regression_baseline_run, tolerances)
        } else {
            (None, Vec::new())
        };
        let payload = SimulationPlanPayload {
            design_variables,
            saved_outputs,
            specs,
            regression_baseline_run,
            regression_tolerances,
        };
        self.simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id: cloned_plan_id,
                payload,
            });
        self.sync_legacy_specs_projection(cloned_plan_id);
        Ok(())
    }
}

struct HierarchyResolver<'a> {
    workspace: &'a ProjectWorkspace,
    libraries: &'a LibraryManager,
    active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    rows: Vec<ResolvedHierarchyBinding>,
    row_indices: HashMap<String, usize>,
    total_instances: usize,
    resolved_instances: usize,
    encountered_instance_paths: HashSet<String>,
    execution_bindings: BTreeMap<String, ConfigurationExecutionBinding>,
}

#[derive(Clone)]
struct HierarchyMaster<'a> {
    schematic: Option<&'a SchematicState>,
    view_type: Option<ViewType>,
    view_modified: bool,
    library_read_only: bool,
    library_has_technology: bool,
    materialized_binding: Option<LibraryCellInstance>,
}

impl<'a> HierarchyResolver<'a> {
    fn new(
        workspace: &'a ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    ) -> Self {
        Self {
            workspace,
            libraries,
            active_overlay,
            rows: Vec::new(),
            row_indices: HashMap::new(),
            total_instances: 0,
            resolved_instances: 0,
            encountered_instance_paths: HashSet::new(),
            execution_bindings: BTreeMap::new(),
        }
    }

    fn resolve(self) -> HierarchyResolution {
        self.resolve_all().0
    }

    fn resolve_all(mut self) -> (HierarchyResolution, Option<ConfigurationExecutionPlan>) {
        let active_configuration = self.workspace.configuration_sets.active();
        let root = active_configuration.map_or_else(
            || {
                CellViewRef::new(
                    &self.workspace.project.root_library,
                    &self.workspace.project.top_cell,
                    DEFAULT_SCHEMATIC_VIEW,
                )
            },
            |configuration| configuration.root().clone(),
        );
        let required_paths = active_configuration
            .map(|configuration| {
                let mut paths = vec![(
                    configuration.dut_path().to_owned(),
                    "configured DUT path".to_owned(),
                )];
                paths.extend(configuration.overrides().iter().map(|scoped| {
                    (
                        scoped.instance_path.clone(),
                        "scoped configuration override".to_owned(),
                    )
                }));
                paths
            })
            .unwrap_or_default();
        let mut ancestors = Vec::new();
        if let Some(error) = active_configuration
            .and_then(|configuration| validate_override_pattern_authority(configuration).err())
        {
            self.total_instances = 1;
            let row = self.binding_row(
                root.clone(),
                None,
                "/top",
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row);
        } else {
            self.resolve_reference(root.clone(), None, "/top", 0, true, &mut ancestors);
        }
        for (path, purpose) in required_paths {
            let matched = if path.contains('*') {
                self.encountered_instance_paths
                    .iter()
                    .any(|candidate| instance_path_pattern_matches(&path, candidate))
            } else {
                self.encountered_instance_paths
                    .contains(&path.to_ascii_lowercase())
            };
            if !matched {
                self.encountered_instance_paths
                    .insert(path.to_ascii_lowercase());
                self.total_instances = self.total_instances.saturating_add(1);
                let row = self.binding_row(
                    root.clone(),
                    None,
                    &path,
                    1,
                    false,
                    (
                        HierarchyBindingStatus::Unresolved,
                        Some(format!(
                            "{purpose} {path} does not exist in the expanded hierarchy"
                        )),
                    ),
                );
                self.upsert(row);
            }
        }
        if let Some(error) = self.execution_model_section_conflict() {
            self.total_instances = self.total_instances.saturating_add(1);
            let row = self.binding_row(
                root.clone(),
                None,
                "/top",
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row);
        }
        let active_configuration = self.workspace.configuration_sets.active();
        let resolution = HierarchyResolution {
            bindings: self.rows,
            total_instances: self.total_instances,
            resolved_instances: self.resolved_instances,
            configuration_id: active_configuration.map(|configuration| configuration.id()),
            configuration_revision: active_configuration
                .map(|configuration| configuration.revision()),
            configuration_digest: active_configuration
                .map(|configuration| configuration.semantic_digest()),
        };
        let plan = active_configuration.map(|configuration| ConfigurationExecutionPlan {
            root,
            bindings: self.execution_bindings,
            configuration_id: configuration.id(),
            configuration_revision: configuration.revision(),
            configuration_digest: configuration.semantic_digest(),
        });
        (resolution, plan)
    }

    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<CellViewRef>,
    ) {
        self.encountered_instance_paths
            .insert(instance_path.to_ascii_lowercase());
        if self.total_instances >= MAX_HIERARCHY_RESOLUTION_INSTANCES {
            let mut row = self.binding_row(
                requested,
                binding,
                instance_path,
                depth,
                is_root,
                (
                    HierarchyBindingStatus::InstanceLimit,
                    Some(format!(
                        "hierarchy exceeds the supported limit of {MAX_HIERARCHY_RESOLUTION_INSTANCES} expanded instances"
                    )),
                ),
            );
            row.instance_count = 1;
            self.total_instances = self.total_instances.saturating_add(1);
            self.upsert(row);
            return;
        }
        self.total_instances += 1;

        if depth > MAX_HIERARCHY_RESOLUTION_DEPTH {
            let row = self.binding_row(
                requested,
                binding,
                instance_path,
                depth,
                is_root,
                (
                    HierarchyBindingStatus::DepthLimit,
                    Some(format!(
                        "hierarchy exceeds the supported depth of {MAX_HIERARCHY_RESOLUTION_DEPTH}"
                    )),
                ),
            );
            self.upsert(row);
            return;
        }

        let search_order = self.view_search_order(&requested.view, is_root, instance_path);
        let (master, resolution_error) =
            match self.resolve_master(&requested, binding, &search_order) {
                Ok(master) => (master, None),
                Err(error) => (None, Some(error)),
            };
        let resolved_reference = master
            .as_ref()
            .and_then(|(_, reference)| reference.clone())
            .unwrap_or_else(|| requested.clone());
        let used_review_fallback = master.is_some()
            && self.used_review_fallback(
                &requested.view,
                &resolved_reference.view,
                is_root,
                instance_path,
            );
        let identity = hierarchy_identity(&resolved_reference);

        if let Some(cycle_start) = ancestors
            .iter()
            .position(|ancestor| hierarchy_identity(ancestor) == identity)
        {
            let chain = ancestors
                .iter()
                .skip(cycle_start)
                .chain(std::iter::once(&resolved_reference))
                .map(hierarchy_display_path)
                .collect::<Vec<_>>()
                .join(" → ");
            let row = self.binding_row_with_master(
                resolved_reference,
                binding,
                instance_path,
                depth,
                is_root,
                master.map(|(master, _)| master),
                HierarchyBindingStatus::Recursive,
                false,
                Some(format!("recursive hierarchy: {chain}")),
            );
            self.upsert(row);
            return;
        }

        let (master, mut status, mut diagnostic) = match master {
            Some((master, _)) => {
                let modified = master.view_modified
                    || master.schematic.is_some_and(|schematic| schematic.is_dirty)
                    || used_review_fallback;
                (
                    Some(master),
                    if modified {
                        HierarchyBindingStatus::Modified
                    } else {
                        HierarchyBindingStatus::Resolved
                    },
                    None,
                )
            }
            None => (
                None,
                HierarchyBindingStatus::Unresolved,
                resolution_error.or_else(|| {
                    Some(format!(
                        "no executable master resolved for {} using {}",
                        hierarchy_display_path(&requested),
                        search_order.join(" → ")
                    ))
                }),
            ),
        };

        let current_platform = crate::state::ConfigurationPlatform::current();
        if !self.configured_platform_eligible(instance_path, current_platform) {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} is not supported by this execution target ({})",
                current_platform.label()
            ));
        }
        if status.is_resolved()
            && self.configured_platform_declared(
                instance_path,
                crate::state::ConfigurationPlatform::Browser,
            )
            && master
                .as_ref()
                .and_then(|value| value.materialized_binding.as_ref())
                .and_then(|binding| binding.source_path.as_ref())
                .is_some_and(|path| !is_project_virtual_source_path(path))
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} declares Browser eligibility, but its filesystem-backed source is unavailable in this browser session"
            ));
        }
        let configured_model_section = self.configured_model_section(instance_path);
        if let Some(section) = configured_model_section.as_deref()
            && status.is_resolved()
            && master
                .as_ref()
                .and_then(|value| value.view_type)
                .is_some_and(|view_type| {
                    !matches!(view_type, ViewType::Spice | ViewType::Extracted)
                })
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} requires a source-backed SPICE or extracted view"
            ));
        }
        if let Some(section) = configured_model_section.as_deref()
            && status.is_resolved()
            && let Some(source_path) = master
                .as_ref()
                .and_then(|value| value.materialized_binding.as_ref())
                .and_then(|binding| binding.source_path.as_deref())
            && let Err(error) = validate_configured_model_section(source_path, section)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} is unavailable: {error}"
            ));
        }

        let project_veriloga = if status.is_resolved()
            && master
                .as_ref()
                .and_then(|value| value.view_type)
                .is_some_and(|view_type| view_type == ViewType::VerilogA)
        {
            match project_veriloga_binding_for_view(
                self.workspace,
                self.libraries,
                &resolved_reference,
            ) {
                Ok(binding) => Some(binding),
                Err(error) => {
                    status = HierarchyBindingStatus::Unresolved;
                    diagnostic = Some(error);
                    None
                }
            }
        } else {
            None
        };

        let stop_boundary = master
            .as_ref()
            .and_then(|value| value.view_type)
            .is_some_and(|view_type| self.stops_at(instance_path, Some(view_type)));
        if status.is_resolved()
            && self.workspace.configuration_sets.active().is_some()
            && let Some(master) = master.as_ref()
            && let Some(view_type) = master.view_type
        {
            let mut materialized_binding = if is_root {
                None
            } else {
                master.materialized_binding.clone()
            };
            if let Some(materialized) = materialized_binding.as_mut()
                && matches!(view_type, ViewType::Schematic | ViewType::Testbench)
            {
                materialized.module_name = Some(configured_subcircuit_name(
                    &resolved_reference,
                    instance_path,
                ));
            }
            self.execution_bindings.insert(
                instance_path.to_ascii_lowercase(),
                ConfigurationExecutionBinding {
                    instance_path: instance_path.to_owned(),
                    resolved_reference: resolved_reference.clone(),
                    resolved_view_type: view_type,
                    materialized_binding,
                    model_section: configured_model_section,
                    stop_boundary,
                    project_veriloga,
                },
            );
        }

        let row = self.binding_row_with_master(
            resolved_reference.clone(),
            binding,
            instance_path,
            depth,
            is_root,
            master.clone(),
            status,
            used_review_fallback,
            diagnostic,
        );
        self.upsert(row);
        if status.is_resolved() {
            self.resolved_instances += 1;
        }

        if stop_boundary {
            return;
        }

        let Some(schematic) = master.as_ref().and_then(|master| master.schematic) else {
            return;
        };
        let children = schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| {
                component
                    .library_cell
                    .clone()
                    .map(|binding| (component.name.clone(), binding))
            })
            .collect::<Vec<_>>();
        if children.is_empty() {
            return;
        }

        ancestors.push(resolved_reference.clone());
        for (instance_name, child) in &children {
            let requested_view = if child.view.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW
            } else {
                child.view.as_str()
            };
            let child_path = format!("{instance_path}/{instance_name}");
            self.resolve_reference(
                CellViewRef::new(&child.library, &child.cell, requested_view),
                Some(child),
                &child_path,
                depth + 1,
                false,
                ancestors,
            );
        }
        ancestors.pop();
    }

    fn binding_row(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
        depth: usize,
        is_root: bool,
        outcome: (HierarchyBindingStatus, Option<String>),
    ) -> ResolvedHierarchyBinding {
        let (status, diagnostic) = outcome;
        self.binding_row_with_master(
            reference,
            binding,
            instance_path,
            depth,
            is_root,
            None,
            status,
            false,
            diagnostic,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn binding_row_with_master(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
        depth: usize,
        is_root: bool,
        master: Option<HierarchyMaster<'_>>,
        status: HierarchyBindingStatus,
        used_review_fallback: bool,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        let search_order = self.view_search_order(
            binding.map_or(reference.view.as_str(), |value| value.view.as_str()),
            is_root,
            instance_path,
        );
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();
        let terminal_view = master
            .as_ref()
            .and_then(|value| value.view_type)
            .filter(|view_type| hierarchy_stop_view(*view_type));
        let purpose = if is_root {
            "testbench root"
        } else if source_bound || terminal_view.is_some() {
            "macro-model"
        } else if master
            .is_some_and(|value| value.library_read_only && value.library_has_technology)
        {
            "foundry devices"
        } else if depth == 1 {
            "design under test"
        } else {
            "hierarchical cell"
        };
        let stop_view = if self.workspace.configuration_sets.active().is_some() {
            self.configured_stop_views(instance_path)
                .into_iter()
                .find(|stop| {
                    terminal_view.is_some_and(|view_type| {
                        view_type.display_name().eq_ignore_ascii_case(stop)
                    }) || search_order
                        .iter()
                        .any(|candidate| candidate.eq_ignore_ascii_case(stop))
                })
        } else if is_root {
            None
        } else {
            terminal_view
                .map(|view_type| view_type.display_name().to_owned())
                .or_else(|| {
                    search_order
                        .iter()
                        .rev()
                        .find(|view| hierarchy_stop_view(ViewType::from_name(view)))
                        .cloned()
                })
        };
        ResolvedHierarchyBinding {
            model_section: self.model_section(&reference, binding, instance_path),
            reference,
            purpose: purpose.to_owned(),
            view_search_order: search_order,
            stop_view,
            status,
            instance_count: 1,
            instance_paths: vec![instance_path.to_owned()],
            used_review_fallback,
            diagnostic,
        }
    }

    fn configured_primary_views(
        &self,
        requested: &str,
        is_root: bool,
        instance_path: &str,
    ) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return hierarchy_view_search_order(requested, is_root);
        };
        let mut order = Vec::new();
        if is_root {
            order.push(if requested.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW.to_owned()
            } else {
                requested.to_ascii_lowercase()
            });
        }
        let configured = selected_configuration_override(configuration.overrides(), instance_path)
            .map_or(configuration.executable_view_policy(), |scoped| {
                scoped.executable_views.as_slice()
            });
        order.extend(configured.iter().cloned());
        deduplicate_view_order(&mut order);
        order
    }

    fn view_search_order(
        &self,
        requested: &str,
        is_root: bool,
        instance_path: &str,
    ) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return hierarchy_view_search_order(requested, is_root);
        };
        let mut order = self.configured_primary_views(requested, is_root, instance_path);
        if configuration.definition().unresolved_policy
            == crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview
        {
            order.extend(hierarchy_view_search_order(requested, is_root));
        }
        deduplicate_view_order(&mut order);
        order
    }

    fn used_review_fallback(
        &self,
        requested: &str,
        resolved: &str,
        is_root: bool,
        instance_path: &str,
    ) -> bool {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return false;
        };
        configuration.definition().unresolved_policy
            == crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview
            && !self
                .configured_primary_views(requested, is_root, instance_path)
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(resolved))
    }

    fn configured_stop_views(&self, instance_path: &str) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return Vec::new();
        };
        if let Some(scoped) =
            selected_configuration_override(configuration.overrides(), instance_path)
            && let Some(stop_view) = &scoped.stop_view
        {
            return vec![stop_view.clone()];
        }
        configuration.stop_views().to_vec()
    }

    fn stops_at(&self, instance_path: &str, resolved_view: Option<ViewType>) -> bool {
        let Some(resolved_view) = resolved_view else {
            return false;
        };
        // A stop is executable only when the selected view is itself a
        // materialized terminal implementation.  Treating a schematic as a
        // black box would emit an X-instance without any defining source.
        if !hierarchy_stop_view(resolved_view) {
            return false;
        }
        self.configured_stop_views(instance_path)
            .iter()
            .any(|stop| resolved_view.display_name().eq_ignore_ascii_case(stop))
    }

    fn model_section(
        &self,
        reference: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
    ) -> String {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
                    .and_then(|scoped| scoped.model_section.clone())
            })
            .unwrap_or_else(|| hierarchy_model_section(self.libraries, reference, binding))
    }

    fn configured_model_section(&self, instance_path: &str) -> Option<String> {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .and_then(|scoped| scoped.model_section.clone())
    }

    fn configured_platform_eligible(
        &self,
        instance_path: &str,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .is_none_or(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn configured_platform_declared(
        &self,
        instance_path: &str,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .is_some_and(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn execution_model_section_conflict(&self) -> Option<String> {
        let mut sources = HashMap::<String, (Option<&str>, &str)>::new();
        for binding in self.execution_bindings.values() {
            let Some(materialized) = binding.materialized_binding.as_ref() else {
                continue;
            };
            let Some(source_path) = materialized.source_path.as_deref() else {
                continue;
            };
            let key = configured_source_identity(source_path);
            let section = binding.model_section.as_deref();
            if let Some((existing_section, existing_path)) = sources.get(&key).copied() {
                if existing_section != section {
                    return Some(format!(
                        "source '{}' has conflicting model-section bindings '{}' at {} and '{}' at {}",
                        source_path.display(),
                        existing_section.unwrap_or("<entire source>"),
                        existing_path,
                        section.unwrap_or("<entire source>"),
                        binding.instance_path
                    ));
                }
            } else {
                sources.insert(key, (section, binding.instance_path()));
            }
        }
        None
    }

    fn resolve_master(
        &self,
        requested: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        search_order: &[String],
    ) -> Result<Option<(HierarchyMaster<'a>, Option<CellViewRef>)>, String> {
        let library = find_library(self.libraries, &requested.library);
        let cell = library.and_then(|library| find_cell(library, &requested.cell));
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();

        // Compatibility mode retains the historical placed-binding authority.
        // Configuration mode below instead materializes each selected L/C/V
        // from the authoritative library view.
        if self.workspace.configuration_sets.active().is_none() && source_bound {
            if !search_order
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(&requested.view))
            {
                return Ok(None);
            }
            let Some(view) = cell.and_then(|cell| find_view(cell, &requested.view)) else {
                return Ok(None);
            };
            let binding = binding.expect("source-bound branch has a placed binding");
            self.validate_source_binding(binding)?;
            return Ok(Some((
                HierarchyMaster {
                    schematic: None,
                    view_type: Some(view.view_type),
                    view_modified: view.modified,
                    library_read_only: library.is_some_and(|library| library.read_only),
                    library_has_technology: library
                        .is_some_and(|library| !library.technology.trim().is_empty()),
                    materialized_binding: Some(binding.clone()),
                },
                Some(requested.clone()),
            )));
        }

        for candidate in search_order {
            // A buffer without an authoritative library/cell/view identity is
            // an orphan, not an executable master. Corrupt or partially
            // restored workspaces must fail closed.
            let Some(view) = cell.and_then(|cell| find_view(cell, candidate)) else {
                continue;
            };
            let reference = CellViewRef::new(
                &library.expect("view implies library").name,
                &cell.expect("view implies cell").name,
                &view.name,
            );
            let view_type = view.view_type;
            if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
                if let Some(schematic) = self.find_schematic(&reference) {
                    let materialized_binding = binding
                        .map(|placed| materialize_schematic_binding(placed, &reference, schematic))
                        .transpose()?;
                    return Ok(Some((
                        HierarchyMaster {
                            schematic: Some(schematic),
                            view_type: Some(view_type),
                            view_modified: view.modified,
                            library_read_only: library.is_some_and(|library| library.read_only),
                            library_has_technology: library
                                .is_some_and(|library| !library.technology.trim().is_empty()),
                            materialized_binding,
                        },
                        Some(reference),
                    )));
                }
                continue;
            }
            if self.workspace.configuration_sets.active().is_some()
                && hierarchy_stop_view(view_type)
            {
                let Some(placed) = binding else {
                    return Err(format!(
                        "configuration root {} cannot materialize source view '{}' without an instance interface",
                        requested.display_path(),
                        candidate
                    ));
                };
                let materialized = materialize_authoritative_source_binding(
                    placed,
                    library.expect("view implies library"),
                    cell.expect("view implies cell"),
                    view,
                    self.workspace,
                    self.libraries,
                )?;
                self.validate_source_binding(&materialized)?;
                return Ok(Some((
                    HierarchyMaster {
                        schematic: None,
                        view_type: Some(view_type),
                        view_modified: view.modified,
                        library_read_only: library.is_some_and(|library| library.read_only),
                        library_has_technology: library
                            .is_some_and(|library| !library.technology.trim().is_empty()),
                        materialized_binding: Some(materialized),
                    },
                    Some(reference),
                )));
            }
        }
        Ok(None)
    }

    fn find_schematic(&self, reference: &CellViewRef) -> Option<&'a SchematicState> {
        if let Some((overlay_reference, schematic)) = self.active_overlay
            && overlay_reference
                .key()
                .eq_ignore_ascii_case(&reference.key())
        {
            return Some(schematic);
        }
        find_schematic(self.workspace, reference)
    }

    fn validate_source_binding(&self, binding: &LibraryCellInstance) -> Result<(), String> {
        let Some(library) = find_library(self.libraries, &binding.library) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative library",
                binding.library, binding.cell
            ));
        };
        let Some(cell) = find_cell(library, &binding.cell) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative cell",
                binding.library, binding.cell
            ));
        };
        let Some(view) = find_view(cell, &binding.view) else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative view",
                binding.library, binding.cell, binding.view
            ));
        };
        if !matches!(
            view.view_type,
            ViewType::Spice | ViewType::VerilogA | ViewType::Verilog | ViewType::Extracted
        ) {
            return Err(format!(
                "source-backed binding {}/{}/{} is not an executable source view",
                binding.library, binding.cell, binding.view
            ));
        }
        if binding.terminal_order.is_empty() {
            return Err(format!(
                "source-backed binding {}/{}/{} has no validated terminal contract",
                binding.library, binding.cell, binding.view
            ));
        }
        let source_path = binding
            .source_path
            .as_deref()
            .expect("validated only for source-backed bindings");
        if view.view_type == ViewType::VerilogA
            && self.workspace.configuration_sets.active().is_some()
        {
            let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
            let project_binding =
                project_veriloga_binding_for_view(self.workspace, self.libraries, &reference)?;
            if source_path != Path::new(project_binding.source_key())
                || binding.module_name.as_deref().is_none_or(|module| {
                    !module.eq_ignore_ascii_case(project_binding.netlist_alias())
                })
            {
                return Err(format!(
                    "source-backed binding {} does not match its exact project-owned Verilog-A bundle",
                    reference.display_path()
                ));
            }
            return Ok(());
        }
        if !source_path.is_absolute() {
            return Err(format!(
                "source-backed binding {}/{}/{} does not have an absolute source identity",
                binding.library, binding.cell, binding.view
            ));
        }
        let authoritative_path = view
            .file_path
            .as_deref()
            .or_else(|| metadata_source_path(&view.metadata))
            .or_else(|| metadata_source_path(&cell.metadata));
        let Some(authoritative_path) = authoritative_path else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative source identity",
                binding.library, binding.cell, binding.view
            ));
        };
        if !source_paths_match(source_path, authoritative_path) {
            return Err(format!(
                "source-backed binding {}/{}/{} conflicts with the authoritative source path",
                binding.library, binding.cell, binding.view
            ));
        }
        validate_source_file(source_path, view.view_type, binding)
    }

    fn upsert(&mut self, row: ResolvedHierarchyBinding) {
        // Rows are grouped by the executable binding contract, not by their
        // current outcome. Repeated instances of one master must remain one
        // review row while `instance_paths` preserves every exact occurrence;
        // a recursive or unresolved occurrence then promotes the aggregate to
        // the most severe observed status. Configuration variants still split
        // naturally through their ordered views, stop, model, and fallback
        // fields below.
        let key = format!(
            "{}|{}|{}|{}|{}",
            row.reference.key().to_ascii_lowercase(),
            row.view_search_order.join(",").to_ascii_lowercase(),
            row.stop_view
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase(),
            row.model_section.to_ascii_lowercase(),
            row.used_review_fallback,
        );
        if let Some(index) = self.row_indices.get(&key).copied() {
            let existing = &mut self.rows[index];
            existing.instance_count = existing.instance_count.saturating_add(row.instance_count);
            existing.instance_paths.extend(row.instance_paths);
            existing
                .instance_paths
                .sort_by_key(|path| path.to_ascii_lowercase());
            existing
                .instance_paths
                .dedup_by(|left, right| left.eq_ignore_ascii_case(right));
            if row.status.severity() > existing.status.severity() {
                existing.status = row.status;
                existing.diagnostic = row.diagnostic;
            }
            return;
        }
        self.row_indices.insert(key, self.rows.len());
        self.rows.push(row);
    }
}

fn find_library<'a>(libraries: &'a LibraryManager, name: &str) -> Option<&'a Library> {
    libraries
        .libraries_by_key()
        .find(|(key, library)| {
            key.eq_ignore_ascii_case(name) || library.name.eq_ignore_ascii_case(name)
        })
        .map(|(_, library)| library)
}

fn find_cell<'a>(library: &'a Library, name: &str) -> Option<&'a Cell> {
    library
        .cells
        .iter()
        .find(|(key, cell)| key.eq_ignore_ascii_case(name) || cell.name.eq_ignore_ascii_case(name))
        .map(|(_, cell)| cell)
}

fn find_view<'a>(cell: &'a Cell, name: &str) -> Option<&'a View> {
    cell.views
        .iter()
        .find(|(key, view)| key.eq_ignore_ascii_case(name) || view.name.eq_ignore_ascii_case(name))
        .map(|(_, view)| view)
}

fn find_schematic<'a>(
    workspace: &'a ProjectWorkspace,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    workspace
        .schematic_buffers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
        .map(|(_, schematic)| schematic)
}

fn is_project_virtual_source_path(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|value| value.starts_with("__rspice_project__/"))
}

fn translated_point(
    point: crate::state::Point,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    Ok(crate::state::Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet x coordinate",
            ))?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet y coordinate",
            ))?,
    ))
}

/// Resolve one typed cross-sheet endpoint against the authored topology and
/// then project it into the endpoint sheet's execution namespace. A wire
/// point must still lie on its retained conductor; a component terminal must
/// still own a canonical wire connection. Stale contracts fail before DRC or
/// netlisting rather than silently connecting a label to a component origin.
fn projected_cross_sheet_anchor(
    source: &SchematicState,
    projected: &SchematicState,
    endpoint: &crate::state::CrossSheetPortEndpoint,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    let authored_point = match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, point } => {
            let wire = source
                .wires
                .iter()
                .find(|wire| wire.id == *wire_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                })?;
            if !wire.contains_point(*point) {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor point",
                    identity: format!("{}@{},{}", wire_id, point.x, point.y),
                });
            }
            *point
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal {
            component_id,
            terminal_name,
        } => {
            if !source
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
            let connection = source
                .connections
                .iter()
                .find(|connection| {
                    connection.component_id == *component_id
                        && connection.terminal_name.eq_ignore_ascii_case(terminal_name)
                })
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal connection",
                    identity: format!("{}:{}", component_id, terminal_name),
                })?;
            source
                .wires
                .iter()
                .find(|wire| wire.id == connection.wire_id)
                .and_then(|wire| wire.points.get(connection.point_index))
                .copied()
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal wire point",
                    identity: format!("{}:{}", connection.wire_id, connection.point_index),
                })?
        }
    };
    let anchor = translated_point(authored_point, delta)?;
    match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, .. } => {
            if !projected
                .wires
                .iter()
                .any(|wire| wire.id == *wire_id && wire.contains_point(anchor))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                });
            }
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal { component_id, .. } => {
            if !projected
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
        }
    }
    Ok(anchor)
}

fn materialize_schematic_binding(
    placed: &LibraryCellInstance,
    reference: &CellViewRef,
    schematic: &SchematicState,
) -> Result<LibraryCellInstance, String> {
    let ports = schematic.interface_ports();
    let authoritative = ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(&placed.terminal_order, &authoritative)
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative schematic view '{}'",
            placed.library, placed.cell, reference.view
        ));
    }
    let mut materialized =
        LibraryCellInstance::new(&reference.library, &reference.cell, &reference.view);
    materialized.bind_interface(&ports);
    Ok(materialized)
}

fn materialize_authoritative_source_binding(
    placed: &LibraryCellInstance,
    library: &Library,
    cell: &Cell,
    view: &View,
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
) -> Result<LibraryCellInstance, String> {
    let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
    let project_veriloga = (view.view_type == ViewType::VerilogA)
        .then(|| project_veriloga_binding_for_view(workspace, libraries, &reference))
        .transpose()?;
    let source_path = if let Some(binding) = project_veriloga.as_ref() {
        PathBuf::from(binding.source_key())
    } else {
        view.file_path
            .clone()
            .or_else(|| metadata_source_path(&view.metadata).map(Path::to_path_buf))
            .or_else(|| metadata_source_path(&cell.metadata).map(Path::to_path_buf))
            .ok_or_else(|| {
                format!(
                    "authoritative source view {}/{}/{} has no source identity",
                    library.name, cell.name, view.name
                )
            })?
    };
    let terminal_order = metadata_terminal_names(&view.metadata)
        .or_else(|| metadata_terminal_names(&cell.metadata))
        .ok_or_else(|| {
            format!(
                "authoritative source view {}/{}/{} has no terminal contract",
                library.name, cell.name, view.name
            )
        })?;
    if !placed.terminal_order.is_empty()
        && !same_terminal_contract(
            &placed.terminal_order,
            &terminal_order
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
    {
        return Err(format!(
            "placed interface for {}/{} is incompatible with authoritative source view '{}'",
            placed.library, placed.cell, view.name
        ));
    }
    let mut materialized = LibraryCellInstance::new(&library.name, &cell.name, &view.name);
    materialized.source_path = Some(source_path);
    materialized.module_name = project_veriloga
        .as_ref()
        .map(|binding| binding.netlist_alias().to_owned())
        .or_else(|| {
            view.metadata
                .get("veriloga.module")
                .or_else(|| view.metadata.get("netlist.module"))
                .or_else(|| cell.metadata.get("veriloga.module"))
                .or_else(|| cell.metadata.get("netlist.module"))
                .cloned()
        });
    materialized.netlist_template = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.template", "netlist_template"],
    );
    materialized.model_section = metadata_value(
        [&view.metadata, &cell.metadata],
        &["netlist.section", "model.section"],
    );
    materialized.reference_prefix = metadata_value(
        [&view.metadata, &cell.metadata],
        &["reference.prefix", "reference_prefix"],
    );
    materialized.parameter_order = metadata_terminal_names_for_keys(
        [&view.metadata, &cell.metadata],
        &["netlist.parameter_order"],
    )
    .unwrap_or_default();
    let ports = terminal_order
        .into_iter()
        .map(|name| crate::state::PortSpec {
            name,
            direction: crate::state::PortDirection::InOut,
        })
        .collect::<Vec<_>>();
    materialized.bind_interface(&ports);
    Ok(materialized)
}

pub(crate) fn project_veriloga_binding_for_view(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Result<ConfigurationVerilogABinding, String> {
    let library = find_library(libraries, &reference.library).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative library",
            reference.display_path()
        )
    })?;
    let cell = find_cell(library, &reference.cell).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative cell",
            reference.display_path()
        )
    })?;
    let view = find_view(cell, &reference.view).ok_or_else(|| {
        format!(
            "project Verilog-A source owner {} has no authoritative view",
            reference.display_path()
        )
    })?;
    if view.view_type != ViewType::VerilogA {
        return Err(format!(
            "project source owner {} is not a Verilog-A view",
            reference.display_path()
        ));
    }
    let owner = ProjectSourceOwner::cell_view(reference.clone());
    let bundle = workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| {
            format!(
                "Verilog-A view {} has no project-owned source bundle",
                reference.display_path()
            )
        })?;
    let selected_module = view
        .metadata
        .get("veriloga.module")
        .or_else(|| view.metadata.get("netlist.module"))
        .or_else(|| cell.metadata.get("veriloga.module"))
        .or_else(|| cell.metadata.get("netlist.module"))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "Verilog-A view {} has no explicit module binding",
                reference.display_path()
            )
        })?
        .to_owned();
    let source_key = super::project_sources::project_veriloga_bundle_source_key(
        workspace.project.id(),
        bundle,
        &selected_module,
    )
    .map_err(|error| error.to_string())?;
    let netlist_alias =
        super::project_sources::project_veriloga_bundle_alias(bundle, &selected_module)
            .map_err(|error| error.to_string())?;
    Ok(ConfigurationVerilogABinding {
        source_bundle_id: bundle.id(),
        source_closure_digest: bundle.closure_digest(),
        selected_module,
        source_key,
        netlist_alias,
    })
}

fn metadata_terminal_names(metadata: &HashMap<String, String>) -> Option<Vec<String>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    });
    (!names.is_empty()).then_some(names)
}

fn metadata_value<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<String> {
    maps.into_iter()
        .find_map(|metadata| {
            keys.iter()
                .find_map(|key| metadata.get(*key))
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
        })
        .map(str::to_owned)
}

fn metadata_terminal_names_for_keys<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<Vec<String>> {
    let encoded = maps
        .into_iter()
        .find_map(|metadata| keys.iter().find_map(|key| metadata.get(*key)))?;
    let values = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|value| {
                let value = value.trim();
                (!value.is_empty()).then(|| value.to_owned())
            })
            .collect()
    });
    (!values.is_empty()).then_some(values)
}

fn same_terminal_contract(placed: &[String], authoritative: &[&str]) -> bool {
    placed.len() == authoritative.len()
        && placed
            .iter()
            .zip(authoritative)
            .all(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn configured_subcircuit_name(reference: &CellViewRef, instance_path: &str) -> String {
    let stem = reference
        .cell
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    let digest = sha2::Sha256::digest(
        format!(
            "{}|{}",
            reference.key().to_ascii_lowercase(),
            instance_path.to_ascii_lowercase()
        )
        .as_bytes(),
    );
    let suffix = digest[..6]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{stem}__cfg_{suffix}")
}

fn metadata_source_path(metadata: &HashMap<String, String>) -> Option<&Path> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(Path::new)
}

fn source_paths_match(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
            (Ok(left), Ok(right)) => left == right,
            _ => false,
        }
    }
    #[cfg(target_arch = "wasm32")]
    false
}

fn configured_source_identity(path: &Path) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    #[cfg(target_arch = "wasm32")]
    let path = path.to_path_buf();
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = if let Some(section) = binding.model_section.as_deref() {
        let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
        processor
            .process_lib(&source_path.to_string_lossy(), Some(section))
            .map_err(|error| {
                format!(
                    "source-backed binding {}/{}/{} cannot resolve model section '{}' from {}: {error}",
                    binding.library,
                    binding.cell,
                    binding.view,
                    section,
                    source_path.display()
                )
            })?
    } else {
        std::fs::read_to_string(source_path).map_err(|error| {
            format!(
                "source-backed binding {}/{}/{} cannot read {}: {error}",
                binding.library,
                binding.cell,
                binding.view,
                source_path.display()
            )
        })?
    };
    let master = binding
        .module_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(&binding.cell);
    let declaration_found = match view_type {
        ViewType::Verilog | ViewType::VerilogA => source.lines().any(|line| {
            let code = line.split("//").next().unwrap_or_default();
            let mut tokens = code
                .split(|character: char| !character.is_alphanumeric() && character != '_')
                .filter(|token| !token.is_empty());
            tokens.any(|token| token.eq_ignore_ascii_case("module"))
                && tokens.any(|token| token.eq_ignore_ascii_case(master))
        }),
        ViewType::Spice | ViewType::Extracted => source.lines().any(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let directive = tokens.next();
            let declared_name = tokens.next();
            let subcircuit_matches = directive
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            let model_matches = binding.netlist_template.is_some()
                && directive.is_some_and(|token| token.eq_ignore_ascii_case(".model"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            subcircuit_matches || model_matches
        }),
        _ => false,
    };
    if declaration_found {
        Ok(())
    } else {
        Err(format!(
            "source-backed binding {}/{}/{} does not declare executable master {master}",
            binding.library, binding.cell, binding.view
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_configured_model_section(source_path: &Path, section: &str) -> Result<(), String> {
    let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
    processor
        .process_lib(&source_path.to_string_lossy(), Some(section))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
fn validate_configured_model_section(_source_path: &Path, _section: &str) -> Result<(), String> {
    Err("filesystem-backed model sections are unavailable in this browser session".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn validate_source_file(
    _source_path: &Path,
    _view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    Err(format!(
        "source-backed binding {}/{}/{} references a desktop path unavailable in this browser session",
        binding.library, binding.cell, binding.view
    ))
}

fn hierarchy_identity(reference: &CellViewRef) -> String {
    reference.key().to_ascii_lowercase()
}

fn hierarchy_display_path(reference: &CellViewRef) -> String {
    format!("{}/{}", reference.library, reference.cell)
}

fn hierarchy_view_search_order(requested: &str, is_root: bool) -> Vec<String> {
    let requested = if requested.eq_ignore_ascii_case("symbol") {
        DEFAULT_SCHEMATIC_VIEW
    } else {
        requested
    };
    let mut order = vec![requested.to_owned()];
    match ViewType::from_name(requested) {
        ViewType::Schematic | ViewType::Testbench if !is_root => {
            order.push("extracted".to_owned());
            order.push("spice".to_owned());
        }
        ViewType::Schematic | ViewType::Testbench => order.push("spice".to_owned()),
        ViewType::Extracted | ViewType::Verilog | ViewType::VerilogA => {
            order.push("spice".to_owned());
        }
        ViewType::Spice => {}
        _ => order.push("spice".to_owned()),
    }
    order.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    order
}

fn deduplicate_view_order(order: &mut Vec<String>) {
    let mut seen = HashSet::with_capacity(order.len());
    order.retain(|view| seen.insert(view.to_ascii_lowercase()));
}

fn selected_configuration_override<'a>(
    overrides: &'a [crate::state::ConfigurationSetOverride],
    instance_path: &str,
) -> Option<&'a crate::state::ConfigurationSetOverride> {
    overrides
        .iter()
        .filter(|scoped| instance_path_pattern_matches(&scoped.instance_path, instance_path))
        .max_by_key(|scoped| instance_path_pattern_specificity(&scoped.instance_path))
}

fn instance_path_pattern_matches(pattern: &str, instance_path: &str) -> bool {
    let pattern = pattern.trim_start_matches('/').split('/');
    let instance = instance_path.trim_start_matches('/').split('/');
    let pattern = pattern.collect::<Vec<_>>();
    let instance = instance.collect::<Vec<_>>();
    pattern.len() == instance.len()
        && pattern
            .iter()
            .zip(instance)
            .all(|(expected, actual)| *expected == "*" || expected.eq_ignore_ascii_case(actual))
}

fn instance_path_pattern_specificity(pattern: &str) -> usize {
    pattern
        .trim_start_matches('/')
        .split('/')
        .filter(|segment| *segment != "*")
        .count()
}

fn instance_path_patterns_overlap(left: &str, right: &str) -> bool {
    let left = left.trim_start_matches('/').split('/').collect::<Vec<_>>();
    let right = right.trim_start_matches('/').split('/').collect::<Vec<_>>();
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| *left == "*" || right == "*" || left.eq_ignore_ascii_case(right))
}

fn validate_override_pattern_authority(
    configuration: &crate::state::ConfigurationSet,
) -> Result<(), String> {
    for (index, left) in configuration.overrides().iter().enumerate() {
        for right in configuration.overrides().iter().skip(index + 1) {
            if instance_path_pattern_specificity(&left.instance_path)
                == instance_path_pattern_specificity(&right.instance_path)
                && instance_path_patterns_overlap(&left.instance_path, &right.instance_path)
            {
                return Err(format!(
                    "configuration overrides '{}' and '{}' overlap with equal specificity",
                    left.instance_path, right.instance_path
                ));
            }
        }
    }
    Ok(())
}

fn hierarchy_stop_view(view_type: ViewType) -> bool {
    matches!(
        view_type,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA | ViewType::Extracted
    )
}

fn hierarchy_model_section(
    libraries: &LibraryManager,
    reference: &CellViewRef,
    binding: Option<&LibraryCellInstance>,
) -> String {
    let library = find_library(libraries, &reference.library);
    let cell = library.and_then(|library| find_cell(library, &reference.cell));
    let view = cell.and_then(|cell| find_view(cell, &reference.view));
    for metadata in [
        view.map(|view| &view.metadata),
        cell.map(|cell| &cell.metadata),
        library.map(|library| &library.metadata),
    ]
    .into_iter()
    .flatten()
    {
        for key in ["model_sections", "model_section", "sections", "section"] {
            if let Some(value) = metadata.get(key).filter(|value| !value.trim().is_empty()) {
                return value.clone();
            }
        }
    }
    if binding
        .and_then(|value| value.source_path.as_ref())
        .is_some()
    {
        "source-defined".to_owned()
    } else {
        "inherit PVT".to_owned()
    }
}

impl ProjectWorkspace {
    /// Exact testbench root selected for simulation. Legacy projects without
    /// configuration sets retain their project descriptor root.
    pub fn simulation_root_reference(&self) -> CellViewRef {
        self.configuration_sets.active().map_or_else(
            || {
                CellViewRef::new(
                    &self.project.root_library,
                    &self.project.top_cell,
                    DEFAULT_SCHEMATIC_VIEW,
                )
            },
            |configuration| configuration.root().clone(),
        )
    }

    /// Resolve the exact root schematic while projecting the live editor only
    /// when it is the selected root. A different open tab can never silently
    /// replace the configuration's simulation source.
    pub fn simulation_root_schematic<'a>(
        &'a self,
        active_reference: &CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Option<&'a SchematicState> {
        let root = self.simulation_root_reference();
        if root.key().eq_ignore_ascii_case(&active_reference.key()) {
            Some(active_schematic)
        } else {
            find_schematic(self, &root)
        }
    }

    /// Bind the exact active hierarchy configuration into generated source.
    /// The SPICE comment is part of the executable bytes and therefore flows
    /// into source, snapshot, and retained-run digests without relying on
    /// mutable UI state or a side-channel receipt.
    pub fn bind_generated_netlist_provenance(&self, mut source: String) -> String {
        let insertion = source.find('\n').map_or(0, |index| index + 1);
        let mut provenance = self
            .design_management
            .semantic_digest()
            .map(|digest| format!("* RSpice design-management digest {digest}\n"))
            .unwrap_or_else(|error| format!("* RSpice design-management INVALID ({error})\n"));
        if let Some(configuration) = self.configuration_sets.active() {
            provenance.push_str(&format!(
                "* RSpice configuration-set {} revision {} digest {}\n",
                configuration.id(),
                configuration.revision(),
                configuration.semantic_digest()
            ));
        }
        source.insert_str(insertion, &provenance);
        source
    }

    /// Publish an independently mutated catalog and the owning project
    /// revision as one fail-closed transaction. Runtime invalidation uses the
    /// dirty flag while persistent lifecycle hashing authenticates the exact
    /// catalog bytes.
    pub fn replace_configuration_sets(
        &mut self,
        candidate: crate::state::ConfigurationSetCatalog,
    ) -> Result<ObjectRevision, ProjectConfigurationMutationError> {
        candidate.validate()?;
        for configuration in candidate.configurations() {
            let root = configuration.root();
            if !matches!(
                ViewType::from_name(&root.view),
                ViewType::Schematic | ViewType::Testbench
            ) {
                return Err(ProjectConfigurationMutationError::UnsupportedRootView {
                    configuration: configuration.name().to_owned(),
                    root: root.display_path(),
                });
            }
            if !self
                .schematic_buffers
                .keys()
                .any(|key| key.eq_ignore_ascii_case(&root.key()))
            {
                return Err(ProjectConfigurationMutationError::MissingRootBuffer {
                    configuration: configuration.name().to_owned(),
                    root: root.display_path(),
                });
            }
        }
        if candidate == self.configuration_sets {
            return Err(ProjectConfigurationMutationError::NoChanges);
        }
        let next_revision = self.project.revision.next()?;
        self.configuration_sets = candidate;
        self.project.revision = next_revision;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Publish a complete design-management candidate and its owning project
    /// revision atomically. Validation happens before any live state changes;
    /// failed candidates therefore cannot partially alter sheet, variant,
    /// annotation, or hierarchy-audit authority.
    pub fn replace_design_management(
        &mut self,
        candidate: crate::state::DesignManagementCatalog,
    ) -> Result<ObjectRevision, ProjectConfigurationMutationError> {
        candidate.validate().map_err(|source| {
            ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                message: source.to_string(),
            }
        })?;
        let mut published = self.design_management.clone();
        published
            .publish_reviewed_candidate(self.design_management.revision(), candidate)
            .map_err(|source| {
                ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                    message: source.to_string(),
                }
            })?;
        let next_revision = self.project.revision.next()?;
        self.design_management = published;
        self.project.revision = next_revision;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Bind newly authored schematic objects to the currently active sheet.
    /// Legacy projects with no sheet catalog remain untouched; once the
    /// user enters multi-sheet authoring, every later object receives durable
    /// membership at the same save/sync boundary as its schematic edit.
    pub fn assign_unowned_objects_to_active_sheet(
        &mut self,
        reference: &CellViewRef,
        schematic: &SchematicState,
    ) -> Result<bool, ProjectConfigurationMutationError> {
        let key = reference.key();
        let Some(catalog) = self.design_management.sheet_catalog(&key) else {
            return Ok(false);
        };
        let Some(active_sheet_id) = catalog.active_sheet_id() else {
            return Ok(false);
        };
        let live_object_ids = schematic
            .components
            .iter()
            .map(|object| object.id)
            .chain(schematic.wires.iter().map(|object| object.id))
            .chain(schematic.buses.iter().map(|object| object.id))
            .chain(schematic.bus_taps.iter().map(|object| object.id))
            .chain(schematic.junctions.iter().map(|object| object.id))
            .chain(schematic.net_labels.iter().map(|object| object.id))
            .chain(schematic.design_notes.iter().map(|object| object.id))
            .chain(
                schematic
                    .documentation_shapes
                    .iter()
                    .map(|object| object.id),
            )
            .chain(schematic.probes.iter().map(|object| object.id))
            .collect::<Vec<_>>();

        let mut candidate = self.design_management.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .expect("the cloned catalog retains the validated cell/view key");
        let receipt = catalog
            .reconcile_object_assignments(
                catalog.revision(),
                live_object_ids,
                Some(active_sheet_id),
            )
            .map_err(|source| {
                ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                    message: source.to_string(),
                }
            })?;
        if receipt.added_assignments == 0
            && receipt.removed_assignments == 0
            && receipt.removed_cross_sheet_ports == 0
        {
            return Ok(false);
        }
        self.replace_design_management(candidate)?;
        Ok(true)
    }

    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut project_sources = ProjectSourceRegistry::try_from_documents([
            ProjectSourceDocument::try_new(
                "sensor_bridge.va",
                ProjectSourceLanguage::VerilogA,
                "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule",
            )
            .expect("the built-in Verilog-A example is valid"),
            ProjectSourceDocument::try_new(
                "characterize.rspice",
                ProjectSourceLanguage::RSpiceAutomation,
                "plan = project.plan(\"Lab characterization\")\nrun = plan.with_corners(\"all\").execute(target=\"local\")\nrun.require(specs=\"release\")\nrun.compare(baseline=\"main\", waveforms=True)\nrun.export([\"junit\", \"summary.json\", \"report.pdf\"])",
            )
            .expect("the built-in Automation example is valid"),
        ])
        .expect("the bootstrapped Code source registry is valid");
        // The canonical demonstration project opens with both exact examples
        // already validated. File > New uses `new_empty_bootstrapped`, and any
        // subsequent byte edit invalidates this identity immediately.
        project_sources
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .expect("the built-in Verilog-A identity is valid");
        project_sources
            .mark_validated(ProjectSourceLanguage::RSpiceAutomation)
            .expect("the built-in Automation identity is valid");
        let mut workspace = Self {
            project_sources,
            ..Self::default()
        };
        workspace.ensure_library_model(libraries);
        workspace
    }

    /// Create a genuinely empty user project. The canonical startup fixture
    /// keeps the mockup's example sources, while File > New must not force an
    /// unrelated circuit to compile or execute demonstration code.
    pub fn new_empty_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut workspace = Self::new_bootstrapped(libraries);
        workspace.project_sources = ProjectSourceRegistry::default();
        workspace.project_sources_dirty = false;
        workspace
    }

    /// Resolve the complete executable library/cell/view closure rooted at the
    /// project testbench. Open tabs are intentionally irrelevant: this receipt
    /// follows placed hierarchical instances and the same schematic/source
    /// ownership used by netlisting.
    pub fn resolve_hierarchy(&self, libraries: &LibraryManager) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, None).resolve()
    }

    /// Resolve the hierarchy while projecting the live editor buffer over its
    /// persisted workspace copy. Rendering and validation use this form so a
    /// just-placed instance cannot disappear from the receipt until save or a
    /// view switch.
    pub fn resolve_hierarchy_with_active<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
            .resolve()
    }

    /// Materialize the exact multi-sheet, active-variant, and annotation
    /// projection consumed by DRC and netlisting. Authored canvas coordinates
    /// stay local to each sheet; the execution clone namespaces them by sheet
    /// order so coincident coordinates on different pages cannot create an
    /// accidental electrical connection. Explicit cross-sheet port contracts
    /// are then materialized as identically named labels at both endpoints.
    fn materialize_design_management_schematic(
        &self,
        cell_view_key: &str,
        source: &SchematicState,
    ) -> Result<SchematicState, crate::state::DesignManagementError> {
        self.design_management.validate()?;
        let mut projected = source.clone();

        if let Some(catalog) = self.design_management.sheet_catalog(cell_view_key) {
            let offsets = catalog
                .sheets()
                .iter()
                .enumerate()
                .map(|(index, sheet)| {
                    let ordinal = i32::try_from(index).unwrap_or(i32::MAX);
                    (
                        sheet.id(),
                        crate::state::Point::new(ordinal.saturating_mul(1_000_000), 0),
                    )
                })
                .collect::<HashMap<_, _>>();
            let offset_for = |object_id: u64| {
                self.design_management
                    .sheet_for_object_or_active(cell_view_key, object_id)
                    .and_then(|sheet_id| offsets.get(&sheet_id).copied())
                    .unwrap_or_else(crate::state::Point::origin)
            };

            for component in &mut projected.components {
                component.pos = translated_point(component.pos, offset_for(component.id))?;
            }
            for wire in &mut projected.wires {
                let delta = offset_for(wire.id);
                for point in &mut wire.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for bus in &mut projected.buses {
                let delta = offset_for(bus.id);
                for point in &mut bus.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for tap in &mut projected.bus_taps {
                let delta = offset_for(tap.id);
                tap.bus_point = translated_point(tap.bus_point, delta)?;
                tap.connection_point = translated_point(tap.connection_point, delta)?;
            }
            for junction in &mut projected.junctions {
                junction.pos = translated_point(junction.pos, offset_for(junction.id))?;
            }
            for label in &mut projected.net_labels {
                label.pos = translated_point(label.pos, offset_for(label.id))?;
            }
            for note in &mut projected.design_notes {
                note.pos = translated_point(note.pos, offset_for(note.id))?;
            }
            for shape in &mut projected.documentation_shapes {
                let delta = offset_for(shape.id);
                let (minimum, maximum) = shape.bounds();
                let _ = translated_point(minimum, delta)?;
                let _ = translated_point(maximum, delta)?;
                shape.translate(delta);
            }

            for contract in catalog.cross_sheet_ports() {
                for endpoint in [&contract.definition().first, &contract.definition().second] {
                    if catalog.sheet_for_object(endpoint.object_id()) != Some(endpoint.sheet_id) {
                        return Err(crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet anchor sheet assignment",
                            identity: endpoint.object_id().to_string(),
                        });
                    }
                    let delta = offsets.get(&endpoint.sheet_id).copied().ok_or_else(|| {
                        crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet port sheet",
                            identity: endpoint.sheet_id.to_string(),
                        }
                    })?;
                    let anchor = projected_cross_sheet_anchor(source, &projected, endpoint, delta)?;
                    let next_id = projected.next_id();
                    projected.net_labels.push(crate::state::NetLabel::new(
                        next_id,
                        anchor,
                        contract.definition().net_name.clone(),
                    ));
                }
            }
        }

        if let Some(active_variant) = self.design_management.variants().active() {
            let resolved = self
                .design_management
                .variants()
                .resolve(active_variant.id())?;
            let mut do_not_populate = HashSet::new();
            for component in &mut projected.components {
                let Some(override_value) = resolved.override_for(cell_view_key, component.id)?
                else {
                    continue;
                };
                match override_value {
                    crate::state::VariantObjectOverride::DoNotPopulate { .. } => {
                        do_not_populate.insert(component.id);
                    }
                    crate::state::VariantObjectOverride::Substitute { replacement } => {
                        let prior = component.library_cell.take();
                        let mut binding = crate::state::LibraryCellInstance::new(
                            replacement.library.clone(),
                            replacement.cell.clone(),
                            replacement.view.clone(),
                        );
                        if let Some(prior) = prior {
                            binding.terminal_order = prior.terminal_order;
                            binding.terminal_dirs = prior.terminal_dirs;
                            binding.interface_bound = prior.interface_bound;
                        }
                        component.kind = crate::state::ComponentType::CellInstance;
                        component.library_cell = Some(binding);
                        if let Some(value) = &replacement.value_override {
                            component.value.clone_from(value);
                        }
                        if let Some(section) = &replacement.model_section {
                            let mut params =
                                crate::properties::parse_params_string(&component.params);
                            params.insert("model_section".to_owned(), section.clone());
                            component.params =
                                crate::properties::property_bridge::format_params_string(&params);
                        }
                    }
                }
            }
            if !do_not_populate.is_empty() {
                projected
                    .components
                    .retain(|component| !do_not_populate.contains(&component.id));
                projected
                    .connections
                    .retain(|connection| !do_not_populate.contains(&connection.component_id));
            }
        }

        for component in &mut projected.components {
            if let Some(mapping) = self
                .design_management
                .annotation()
                .effective_mapping_for(cell_view_key, component.id)?
            {
                component.name.clone_from(&mapping.new_reference);
            }
        }
        projected.recalculate_runtime_state();
        Ok(projected)
    }

    /// Freeze the live editor projection and the active configuration's
    /// exact-path execution plan as one immutable value.  Legacy projects
    /// return the same projected buffers with no plan, preserving the
    /// historical generator path.
    pub fn configuration_execution_projection<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        let root = self.simulation_root_reference();
        let (resolution, plan) = if self.configuration_sets.active().is_some() {
            HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
                .resolve_all()
        } else {
            (
                HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
                    .resolve(),
                None,
            )
        };
        if self.configuration_sets.active().is_some() && !resolution.is_valid() {
            let diagnostics = resolution
                .bindings
                .iter()
                .filter(|binding| !binding.status.is_resolved())
                .map(|binding| {
                    binding.diagnostic.clone().unwrap_or_else(|| {
                        format!(
                            "{} is {}",
                            binding.reference.display_path(),
                            binding.status.label()
                        )
                    })
                })
                .collect::<Vec<_>>()
                .join("; ");
            return Err(ConfigurationExecutionPlanError::Unresolved(diagnostics));
        }

        let mut schematic_buffers = self.schematic_buffers.clone();
        if let Some(existing_key) = schematic_buffers
            .keys()
            .find(|key| key.eq_ignore_ascii_case(&active_reference.key()))
            .cloned()
        {
            schematic_buffers.insert(existing_key, active_schematic.clone());
        } else {
            schematic_buffers.insert(active_reference.key(), active_schematic.clone());
        }
        for (key, schematic) in &mut schematic_buffers {
            *schematic = self
                .materialize_design_management_schematic(key, schematic)
                .map_err(|error| {
                    ConfigurationExecutionPlanError::DesignManagement(error.to_string())
                })?;
        }
        let projection = ConfigurationExecutionProjection {
            root,
            schematic_buffers,
            plan,
            connectivity: self.connectivity.clone(),
        };
        if projection.root_schematic().is_none() {
            return Err(ConfigurationExecutionPlanError::MissingRoot(
                projection.root.display_path(),
            ));
        }
        Ok(projection)
    }

    /// Ensure the workspace's top library/cell/view exists in the library tree.
    pub fn ensure_library_model(&mut self, libraries: &mut LibraryManager) {
        ensure_project_library(libraries);

        if self.active_view.library.is_empty() {
            self.active_view.library = self.project.root_library.clone();
        }
        if self.active_view.cell.is_empty() {
            self.active_view.cell = self.project.top_cell.clone();
        }
        if self.active_view.view.is_empty() {
            self.active_view.view = DEFAULT_SCHEMATIC_VIEW.to_string();
        }

        let active_view_type = self
            .open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .or_else(|| library_view_type(libraries, &self.active_view))
            .unwrap_or(ViewType::Schematic);

        ensure_cell_view(
            libraries,
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
            active_view_type,
        );

        if self.open_views.is_empty() {
            self.open_views.push(OpenCellView::new(
                self.active_view.clone(),
                active_view_type,
            ));
        }
        if self.hierarchy_stack.is_empty() {
            self.hierarchy_stack.push(self.active_view.clone());
        }

        if is_schematic_like(active_view_type) {
            self.ensure_active_buffer();
        }
        libraries.select_view(
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
        );
    }

    pub fn active_key(&self) -> String {
        self.active_view.key()
    }

    pub fn active_display_path(&self) -> String {
        self.active_view.display_path()
    }

    pub fn active_view_type(&self) -> ViewType {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .unwrap_or(ViewType::Schematic)
    }

    pub fn ensure_active_buffer(&mut self) {
        let key = self.active_key();
        self.schematic_buffers.entry(key).or_default();
    }

    pub fn active_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers.get(&self.active_key())
    }

    pub fn active_schematic_reference(&self) -> CellViewRef {
        if self.active_view_type() == ViewType::Symbol {
            return CellViewRef::new(
                &self.active_view.library,
                &self.active_view.cell,
                DEFAULT_SCHEMATIC_VIEW,
            );
        }
        self.active_view.clone()
    }

    pub fn active_context_schematic(&self) -> Option<&SchematicState> {
        let reference = self.active_schematic_reference();
        self.schematic_buffers.get(&reference.key())
    }

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
        if !is_schematic_like(self.active_view_type()) {
            return;
        }
        let key = self.active_key();
        self.schematic_buffers.insert(key, schematic.clone());
        self.set_active_dirty(schematic.is_dirty);
    }

    pub fn mark_all_clean(&mut self) {
        for view in &mut self.open_views {
            view.dirty = false;
        }
        for schematic in self.schematic_buffers.values_mut() {
            schematic.is_dirty = false;
        }
        self.netlist_source_dirty = false;
        self.project_sources_dirty = false;
        self.project_metadata_dirty = false;
        self.report_documents_dirty = false;
        self.hardcopy_setups_dirty = false;
        self.project_print_mappings_dirty = false;
        self.hardcopy_source_sets_dirty = false;
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self
                .schematic_buffers
                .values()
                .any(|schematic| schematic.is_dirty)
            || self.netlist_source_dirty
            || self.project_sources_dirty
            || self.project_metadata_dirty
            || self.report_documents_dirty
            || self.hardcopy_setups_dirty
            || self.project_print_mappings_dirty
            || self.hardcopy_source_sets_dirty
    }

    /// Commit a validated page setup through the project dirty lifecycle.
    /// Re-saving byte-identical settings is a no-op and does not manufacture
    /// an unsaved project change.
    pub fn save_hardcopy_setup(
        &mut self,
        source: &crate::hardcopy::ActiveHardcopySource,
        setup: crate::hardcopy::HardcopySetup,
    ) -> Result<
        crate::hardcopy::SetupSaveOutcome,
        crate::hardcopy::HardcopyError,
    > {
        let outcome = self.hardcopy_setups.save(source, setup)?;
        if outcome.disposition() != crate::hardcopy::SetupSaveDisposition::Unchanged {
            self.hardcopy_setups_dirty = true;
        }
        Ok(outcome)
    }

    /// Persist a reusable project print-set mapping through the same project
    /// dirty lifecycle as document page setups.
    pub fn save_project_print_mapping(
        &mut self,
        table: crate::hardcopy::PrintMappingTable,
    ) -> Result<
        crate::hardcopy::PrintMappingSaveReceipt,
        crate::hardcopy::PrintMappingPersistenceError,
    > {
        let outcome = self.project_print_mappings.save(table)?;
        if outcome.disposition() != crate::hardcopy::PrintMappingSaveDisposition::Unchanged {
            self.project_print_mappings_dirty = true;
        }
        Ok(outcome)
    }

    #[must_use]
    pub fn hardcopy_source_sets(&self) -> &[crate::hardcopy::sources::HardcopySourceSet] {
        &self.hardcopy_source_sets
    }

    #[must_use]
    pub fn hardcopy_source_set(
        &self,
        source_key: &str,
    ) -> Option<&crate::hardcopy::sources::HardcopySourceSet> {
        self.hardcopy_source_sets
            .iter()
            .find(|source_set| source_set.source_key() == source_key)
    }

    /// Insert or replace one exact source-set definition as a small,
    /// validated transaction. This never clones the rest of the project.
    pub fn save_hardcopy_source_set(
        &mut self,
        source_set: crate::hardcopy::sources::HardcopySourceSet,
    ) -> Result<bool, HardcopySourceSetPersistenceError> {
        source_set
            .validate()
            .map_err(|error| HardcopySourceSetPersistenceError::Invalid {
                message: error.to_string(),
            })?;
        let source_key = source_set.source_key();
        if let Some(existing) = self
            .hardcopy_source_sets
            .iter()
            .find(|existing| existing.source_key() == source_key)
            && existing == &source_set
        {
            return Ok(false);
        }
        let mut candidate = self.hardcopy_source_sets.clone();
        if let Some(index) = candidate
            .iter()
            .position(|existing| existing.source_key() == source_key)
        {
            candidate[index] = source_set;
        } else {
            candidate.push(source_set);
        }
        validate_hardcopy_source_set_catalog(&candidate)?;
        self.hardcopy_source_sets = candidate;
        self.hardcopy_source_sets_dirty = true;
        Ok(true)
    }

    /// Remove one retained aggregate by its stable source identity.
    pub fn remove_hardcopy_source_set(&mut self, source_key: &str) -> bool {
        let before = self.hardcopy_source_sets.len();
        self.hardcopy_source_sets
            .retain(|source_set| source_set.source_key() != source_key);
        let removed = self.hardcopy_source_sets.len() != before;
        self.hardcopy_source_sets_dirty |= removed;
        removed
    }

    pub fn attach_technology(
        &mut self,
        binding: ProjectTechnologyBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        let before = self.project.revision();
        let revision = self.project.attach_technology(binding)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok(revision)
    }

    pub fn set_netlist_source_dirty(&mut self, dirty: bool) {
        self.netlist_source_dirty = dirty;
    }

    /// Add a source document to a legacy/empty project and enter the ordinary
    /// project dirty lifecycle. Duplicate language identities are rejected.
    pub fn add_project_source(
        &mut self,
        document: ProjectSourceDocument,
    ) -> Result<(), ProjectSourceError> {
        self.project_sources.insert(document)?;
        self.project_sources_dirty = true;
        Ok(())
    }

    /// Replace exact source bytes and enter the ordinary project dirty
    /// lifecycle. An unchanged write is a no-op and retains validation.
    pub fn replace_project_source(
        &mut self,
        language: ProjectSourceLanguage,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self.project_sources.replace_content(language, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Replace one language slot from an explicitly imported UTF-8 file while
    /// preserving monotonic slot revision and invalidating old validation.
    pub fn replace_imported_project_source(
        &mut self,
        language: ProjectSourceLanguage,
        file_name: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .replace_imported(language, file_name, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    pub fn remove_project_source(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Option<ProjectSourceDocument> {
        let removed = self.project_sources.remove(language);
        if removed.is_some() {
            self.project_sources_dirty = true;
        }
        removed
    }

    /// Record successful validation for the document's exact current identity.
    /// This evidence is persisted and therefore marks the project dirty only
    /// when it changes.
    pub fn mark_project_source_validated(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        let before = self
            .project_sources
            .get(language)
            .and_then(ProjectSourceDocument::validated_identity);
        let identity = self.project_sources.mark_validated(language)?;
        if before != Some(identity) {
            self.project_sources_dirty = true;
        }
        Ok(identity)
    }

    pub fn mark_project_sources_clean(&mut self) {
        self.project_sources_dirty = false;
    }

    /// Whether the Netlist workspace owns an editable source deck.
    ///
    /// A missing source is intentional: in that state the editor is showing a
    /// generated schematic artifact and must never promote edits implicitly.
    pub fn has_editable_netlist_source(&self) -> bool {
        self.netlist_source.is_some()
    }

    /// Create a project-owned source deck from the current generated artifact.
    ///
    /// This is the one ownership transition used by the explicit Netlist
    /// workspace "Make editable copy" action. Creating the source changes the
    /// persisted project, so it participates in the ordinary project dirty and
    /// save lifecycle on both native and browser targets.
    pub fn make_netlist_editable_copy(&mut self, generated: &str) -> bool {
        if self.netlist_source.is_some() {
            return false;
        }

        self.netlist_source = Some(generated.to_owned());
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    /// Replace an existing project-owned source deck.
    ///
    /// Returns `false` for generated artifacts instead of silently creating an
    /// editable source. That guard makes editor, completion, and tuner writes
    /// safe even if a caller accidentally reaches a mutation path while the
    /// generated document is active.
    pub fn replace_editable_netlist_source(&mut self, source: String) -> bool {
        let Some(owned_source) = self.netlist_source.as_mut() else {
            return false;
        };

        if *owned_source == source {
            return false;
        }

        *owned_source = source;
        self.netlist_source_dirty = true;
        true
    }

    /// Remove the project-owned source and return to schematic-generated output.
    ///
    /// Removing persisted source ownership is itself a project modification;
    /// the dirty bit remains set until an actual project save succeeds.
    pub fn return_to_generated_netlist(&mut self) -> bool {
        if self.netlist_source.take().is_none() {
            return false;
        }

        self.netlist_document = None;
        self.netlist_descriptor = None;
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    pub fn open_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.active_view = reference.clone();
        if !self
            .open_views
            .iter()
            .any(|open| open.reference == reference)
        {
            self.open_views
                .push(OpenCellView::new(reference.clone(), view_type));
        }
        if is_schematic_like(view_type) {
            self.schematic_buffers.entry(reference.key()).or_default();
        }
    }

    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.hierarchy_stack.clear();
        self.hierarchy_instances.clear();
        self.hierarchy_stack.push(reference);
    }

    pub fn enter_hierarchy(&mut self, reference: CellViewRef, view_type: ViewType) {
        // No instance context (menu/browser entry): the cell name is the
        // best available occurrence label.
        let label = reference.cell.clone();
        self.descend_into(label, reference, view_type);
    }

    /// Descend into `instance`, opening its master `reference`. The
    /// occurrence path (TOP, X1, XB, ...) records the instance name.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        if self.hierarchy_stack.last() != Some(&reference) {
            self.hierarchy_stack.push(reference);
            self.hierarchy_instances.push(instance);
            self.align_occurrences();
        }
    }

    /// Keep `hierarchy_instances` exactly one shorter than the stack —
    /// older saves and external truncation re-label from cell names.
    fn align_occurrences(&mut self) {
        let want = self.hierarchy_stack.len().saturating_sub(1);
        self.hierarchy_instances.truncate(want);
        while self.hierarchy_instances.len() < want {
            let index = self.hierarchy_instances.len() + 1;
            self.hierarchy_instances
                .push(self.hierarchy_stack[index].cell.clone());
        }
    }

    /// Display labels for the occurrence path: the root cell, then the
    /// instance descended through at each level.
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.hierarchy_stack
            .iter()
            .enumerate()
            .map(|(index, reference)| {
                if index == 0 {
                    reference.cell.clone()
                } else {
                    self.hierarchy_instances
                        .get(index - 1)
                        .cloned()
                        .unwrap_or_else(|| reference.cell.clone())
                }
            })
            .collect()
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let len = self.hierarchy_stack.len();
        if len < 2 {
            return None;
        }
        self.focus_breadcrumb(len - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        if index >= self.hierarchy_stack.len() {
            return None;
        }

        self.hierarchy_stack.truncate(index + 1);
        self.align_occurrences();
        let reference = self.hierarchy_stack[index].clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        Some(reference)
    }

    pub fn close_view(&mut self, reference: &CellViewRef) {
        if self.open_views.len() <= 1 {
            return;
        }

        self.open_views.retain(|open| &open.reference != reference);
        if &self.active_view == reference
            && let Some(next) = self.open_views.last().cloned()
        {
            self.active_view = next.reference;
        }
    }

    pub fn set_active_dirty(&mut self, dirty: bool) {
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.active_view)
        {
            open.dirty = dirty;
        }
    }
}

/// Ensure the default editable project library exists.
pub fn ensure_project_library(libraries: &mut LibraryManager) {
    if libraries.get_library(DEFAULT_PROJECT_LIBRARY).is_none() {
        let mut library = Library::new(DEFAULT_PROJECT_LIBRARY);
        library
            .metadata
            .insert("role".to_string(), "project".to_string());
        library.metadata.insert(
            "description".to_string(),
            "Project design library".to_string(),
        );
        libraries.add_library(library);
    }
}

/// Ensure a cell view exists in the library manager.
pub fn ensure_cell_view(
    libraries: &mut LibraryManager,
    library_name: &str,
    cell_name: &str,
    view_name: &str,
    view_type: ViewType,
) {
    if libraries.get_library(library_name).is_none() {
        libraries.add_library(Library::new(library_name));
    }

    if let Some(library) = libraries.get_library_mut(library_name) {
        if library.get_cell(cell_name).is_none() {
            let mut cell = Cell::new(cell_name);
            cell.description = "Top-level design cell".to_string();
            cell.add_view(View::new(view_name, view_type));
            library.add_cell(cell);
            return;
        }

        if let Some(cell) = library.get_cell_mut(cell_name)
            && cell.get_view(view_name).is_none()
        {
            cell.add_view(View::new(view_name, view_type));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    fn reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    fn symbol_reference(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "symbol")
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn model_bound_source_validation_resolves_the_selected_lib_section() {
        let path = std::env::temp_dir().join(format!(
            "rspice-model-bound-section-{}.lib",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(
            &path,
            ".lib TT\n.model nmos_18 nmos level=1\n.endl TT\n.lib FF\n.model nmos_18_fast nmos level=1\n.endl FF\n",
        )
        .expect("write sectioned model fixture");
        let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.module_name = Some("nmos_18".to_owned());
        binding.netlist_template = Some("M{name} {nodes} {model} {params}".to_owned());
        binding.model_section = Some("TT".to_owned());

        validate_source_file(&path, ViewType::Spice, &binding)
            .expect("selected section declares the executable model");
        binding.model_section = Some("FF".to_owned());
        assert!(validate_source_file(&path, ViewType::Spice, &binding).is_err());

        std::fs::remove_file(path).expect("remove sectioned model fixture");
    }

    #[test]
    fn configuration_override_patterns_use_most_specific_segment_match() {
        let overrides = vec![
            crate::state::ConfigurationSetOverride {
                instance_path: "/top/*".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            },
            crate::state::ConfigurationSetOverride {
                instance_path: "/top/Xcritical".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            },
        ];
        let selected = selected_configuration_override(&overrides, "/top/xCRITICAL")
            .expect("specific override matches");
        assert_eq!(selected.instance_path, "/top/Xcritical");
        let wildcard = selected_configuration_override(&overrides, "/top/Xother")
            .expect("wildcard override matches");
        assert_eq!(wildcard.instance_path, "/top/*");
    }

    #[test]
    fn equal_specificity_pattern_overlap_is_detectable() {
        assert!(instance_path_patterns_overlap("/top/*/X1", "/top/I0/*"));
        assert_eq!(
            instance_path_pattern_specificity("/top/*/X1"),
            instance_path_pattern_specificity("/top/I0/*")
        );
        assert!(!instance_path_patterns_overlap("/top/I0/X1", "/top/I1/X1"));
    }

    fn resistance_variable(
        name: &str,
        expression: &str,
        scope: DesignVariableScope,
    ) -> DesignVariable {
        DesignVariable::new(
            name,
            expression,
            DesignVariableQuantity::Resistance,
            scope,
            "fixture",
            Some(DesignVariableRange {
                minimum: "1 kohm".to_owned(),
                maximum: "1 Mohm".to_owned(),
            }),
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .expect("fixture variable is valid")
    }

    fn raw_output(
        name: &str,
        expression: &str,
        compatibility: SavedOutputCompatibility,
    ) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            compatibility,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
        )
        .expect("fixture output is valid")
    }

    #[test]
    fn typed_design_variable_enforces_units_range_and_canonical_netlist_value() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        assert_eq!(variable.resolved_value_si().unwrap(), 10_000.0);
        assert_eq!(
            variable.netlist_statement(),
            ".param RLOAD=1.00000000000000000e4"
        );

        let mut wrong_unit = variable.clone();
        wrong_unit.expression = "10 V".to_owned();
        assert!(wrong_unit.validate().unwrap_err().contains("resistance"));

        let mut outside = variable;
        outside.expression = "2 Mohm".to_owned();
        assert!(outside.validate().unwrap_err().contains("outside"));
    }

    #[test]
    fn design_variable_expression_update_preserves_identity_and_metadata() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let original = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = original.id;
        workspace
            .add_design_variable(plan_id, original.clone())
            .expect("fixture variable is accepted");

        workspace
            .update_design_variable_expression(plan_id, variable_id, "22 kohm")
            .expect("valid expression update commits");

        let updated = &workspace
            .active_plan_data(plan_id)
            .expect("plan payload remains present")
            .design_variables[0];
        assert_eq!(updated.id, original.id);
        assert_eq!(updated.name, original.name);
        assert_eq!(updated.expression, "22 kohm");
        assert_eq!(updated.quantity, original.quantity);
        assert_eq!(updated.scope, original.scope);
        assert_eq!(updated.description, original.description);
        assert_eq!(updated.allowed_range, original.allowed_range);
        assert_eq!(updated.sweep_eligibility, original.sweep_eligibility);
        assert_eq!(updated.override_policy, original.override_policy);
    }

    #[test]
    fn out_of_range_design_variable_update_is_rejected_atomically() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        let error = workspace
            .update_design_variable_expression(plan_id, variable_id, "2 Mohm")
            .expect_err("out-of-range expression must be rejected");

        assert!(matches!(
            error,
            SimulationConfigurationError::InvalidDesignVariable { message, .. }
                if message.contains("outside the inclusive allowed range")
        ));
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn design_variable_update_rejects_a_missing_stable_identity() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let missing_id = DesignVariableId::new();
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        assert_eq!(
            workspace.update_design_variable_expression(plan_id, missing_id, "22 kohm"),
            Err(SimulationConfigurationError::DesignVariableNotFound {
                plan_id,
                variable_id: missing_id,
            })
        );
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn committed_design_variable_update_advances_revision_once() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        let initial_revision = variable.revision;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");

        let committed_revision = workspace
            .update_design_variable_expression(plan_id, variable_id, "22 kohm")
            .expect("valid expression update commits");

        assert_eq!(committed_revision.get(), initial_revision.get() + 1);
        assert_eq!(
            workspace
                .active_plan_data(plan_id)
                .expect("plan payload remains present")
                .design_variables[0]
                .revision,
            committed_revision
        );
    }

    #[test]
    fn bulk_design_variable_update_is_all_or_nothing() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let first = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let second = resistance_variable("RBIAS", "15 kohm", DesignVariableScope::Project);
        let updates = vec![
            (first.id, "22 kohm".to_owned()),
            (second.id, "2 Mohm".to_owned()),
        ];
        workspace
            .add_design_variable(plan_id, first)
            .expect("first fixture variable is accepted");
        workspace
            .add_design_variable(plan_id, second)
            .expect("second fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");

        assert!(matches!(
            workspace.update_design_variable_expressions(plan_id, &updates),
            Err(SimulationConfigurationError::InvalidDesignVariable { index: 1, .. })
        ));
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn bulk_design_variable_update_rejects_duplicate_identities_atomically() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let variable_id = variable.id;
        workspace
            .add_design_variable(plan_id, variable)
            .expect("fixture variable is accepted");
        let before = serde_json::to_value(&workspace).expect("workspace serializes");
        let updates = vec![
            (variable_id, "22 kohm".to_owned()),
            (variable_id, "47 kohm".to_owned()),
        ];

        assert_eq!(
            workspace.update_design_variable_expressions(plan_id, &updates),
            Err(
                SimulationConfigurationError::DuplicateDesignVariableUpdate {
                    plan_id,
                    variable_id,
                }
            )
        );
        assert_eq!(
            serde_json::to_value(&workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn saved_output_validation_is_kind_specific() {
        assert!(
            raw_output(
                "VOUT",
                "V(out)",
                SavedOutputCompatibility::AllCompatibleAnalyses
            )
            .validate()
            .is_ok()
        );
        let invalid = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "gain",
            "V(out) / V(in)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        );
        assert!(invalid.unwrap_err().contains("raw output"));
        let derived = SavedOutput::new(
            SavedOutputKind::DerivedExpression,
            "gain",
            "V(out) / V(in)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::OnDemandFromRetainedState,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("calculator expression is valid");
        assert_eq!(derived.inferred_unit(), "resolved from expression");
    }

    #[test]
    fn missing_row_identity_migrates_deterministically_and_null_is_rejected() {
        let variable = resistance_variable("RLOAD", "10 kohm", DesignVariableScope::Project);
        let mut value = serde_json::to_value(variable).unwrap();
        value.as_object_mut().unwrap().remove("id");
        let first: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        let second: DesignVariable = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(first.id, second.id);

        value
            .as_object_mut()
            .unwrap()
            .insert("id".to_owned(), serde_json::Value::Null);
        assert!(
            serde_json::from_value::<DesignVariable>(value)
                .unwrap_err()
                .to_string()
                .contains("must not be null")
        );
    }

    #[test]
    fn plan_payload_clone_refreshes_row_ids_and_analysis_references() {
        let source_plan_id = SimulationPlanId::new();
        let cloned_plan_id = SimulationPlanId::new();
        let source_analysis = AnalysisInstanceId::new();
        let cloned_analysis = AnalysisInstanceId::new();
        let mut workspace = ProjectWorkspace::default();
        let variable = resistance_variable(
            "RLOAD",
            "10 kohm",
            DesignVariableScope::SelectedAnalysis {
                analysis_id: source_analysis,
            },
        );
        let output = raw_output(
            "VOUT",
            "V(out)",
            SavedOutputCompatibility::SelectedAnalysis {
                analysis_id: source_analysis,
            },
        );
        let variable_id = variable.id;
        let output_id = output.id;
        let regression_rule = RegressionToleranceRule {
            target: RegressionTargetSelector {
                source_domain: AnalysisResultSourceDomain::SimulationPlan,
                source_instance_id: source_analysis,
                kind: RegressionTargetKind::Waveform,
                name: "v(out)".to_owned(),
                occurrence: 0,
            },
            method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            absolute_tolerance: 0.01,
            relative_tolerance: 0.005,
            time_skew_allowance: 20e-6,
            comparison_window: Some(RegressionComparisonWindow {
                start: 0.0,
                end: 20e-3,
            }),
        };
        workspace
            .simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id: source_plan_id,
                payload: SimulationPlanPayload {
                    design_variables: vec![variable],
                    saved_outputs: vec![output],
                    regression_baseline_run: Some(RunId::new()),
                    regression_tolerances: vec![regression_rule],
                    ..SimulationPlanPayload::default()
                },
            });

        workspace
            .clone_plan_data(
                source_plan_id,
                cloned_plan_id,
                true,
                true,
                &[(source_analysis, cloned_analysis)],
            )
            .unwrap();
        let cloned = workspace.active_plan_data(cloned_plan_id).unwrap();
        assert_ne!(cloned.design_variables[0].id, variable_id);
        assert_ne!(cloned.saved_outputs[0].id, output_id);
        assert!(matches!(
            cloned.design_variables[0].scope,
            DesignVariableScope::SelectedAnalysis { analysis_id }
                if analysis_id == cloned_analysis
        ));
        assert_eq!(cloned.regression_tolerances.len(), 1);
        assert_eq!(
            cloned.regression_tolerances[0].target.source_instance_id,
            cloned_analysis
        );
        assert_eq!(
            cloned.regression_tolerances[0].comparison_window,
            Some(RegressionComparisonWindow {
                start: 0.0,
                end: 20e-3,
            })
        );
        assert!(matches!(
            cloned.saved_outputs[0].compatible_analyses,
            SavedOutputCompatibility::SelectedAnalysis { analysis_id }
                if analysis_id == cloned_analysis
        ));

        workspace
            .active_plan_data_mut(cloned_plan_id)
            .unwrap()
            .design_variables[0]
            .expression = "20 kohm".to_owned();
        assert_eq!(
            workspace
                .active_plan_data(source_plan_id)
                .unwrap()
                .design_variables[0]
                .expression,
            "10 kohm"
        );
        workspace.validate_simulation_configuration().unwrap();
    }

    #[test]
    fn regression_tolerance_contract_round_trips_and_rejects_invalid_windows() {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        let rule = RegressionToleranceRule {
            target: RegressionTargetSelector {
                source_domain: AnalysisResultSourceDomain::ManualDeck,
                source_instance_id: AnalysisInstanceId::new(),
                kind: RegressionTargetKind::Waveform,
                name: "v(out)".to_owned(),
                occurrence: 0,
            },
            method: RegressionComparisonMethod::PointwiseRelative,
            absolute_tolerance: 1e-3,
            relative_tolerance: 0.02,
            time_skew_allowance: 1e-6,
            comparison_window: Some(RegressionComparisonWindow {
                start: 0.0,
                end: 1e-3,
            }),
        };
        workspace
            .ensure_active_plan_data(plan_id)
            .regression_tolerances = vec![rule.clone()];
        workspace.validate_simulation_configuration().unwrap();

        let json = serde_json::to_string(&workspace).unwrap();
        let restored: ProjectWorkspace = serde_json::from_str(&json).unwrap();
        assert_eq!(
            restored
                .active_plan_data(plan_id)
                .unwrap()
                .regression_tolerances,
            vec![rule]
        );

        let mut invalid = restored;
        invalid
            .active_plan_data_mut(plan_id)
            .unwrap()
            .regression_tolerances[0]
            .comparison_window = Some(RegressionComparisonWindow {
            start: 2.0,
            end: 1.0,
        });
        assert!(matches!(
            invalid.validate_simulation_configuration(),
            Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
        ));

        let mut invalid_name = workspace;
        invalid_name
            .active_plan_data_mut(plan_id)
            .unwrap()
            .regression_tolerances[0]
            .target
            .name = "v(out)\u{1}".to_owned();
        assert!(matches!(
            invalid_name.validate_simulation_configuration(),
            Err(SimulationConfigurationError::InvalidRegressionTolerance { .. })
        ));
    }

    fn add_schematic_master(
        libraries: &mut LibraryManager,
        workspace: &mut ProjectWorkspace,
        library_name: &str,
        cell_name: &str,
        schematic: SchematicState,
    ) {
        if libraries.get_library(library_name).is_none() {
            libraries.add_library(Library::new(library_name));
        }
        let library = libraries
            .get_library_mut(library_name)
            .expect("library exists");
        let cell = library.get_or_create_cell(cell_name);
        if cell.get_view("schematic").is_none() {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        workspace.schematic_buffers.insert(
            CellViewRef::new(library_name, cell_name, "schematic").key(),
            schematic,
        );
    }

    fn instance(library: &str, cell: &str) -> LibraryCellInstance {
        LibraryCellInstance::new(library, cell, "schematic")
    }

    #[test]
    fn hierarchy_resolution_follows_instances_not_open_tabs() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_views.push(OpenCellView::new(
            CellViewRef::new("unrelated", "open_tab", "schematic"),
            ViewType::Schematic,
        ));
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.total_instances, 1);
        assert_eq!(resolution.resolved_instances, 1);
        assert_eq!(resolution.bindings.len(), 1);
        assert_eq!(resolution.bindings[0].purpose, "testbench root");
        assert_eq!(resolution.bindings[0].reference.cell, "top");
    }

    #[test]
    fn hierarchy_resolution_counts_transitive_repeated_instances() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);

        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));

        let mut amp = SchematicState::default();
        amp.add_library_cell_component(Point::new(40, 40), instance("work", "bias"));
        add_schematic_master(&mut libraries, &mut workspace, "work", "amp", amp);
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "bias",
            SchematicState::default(),
        );

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert!(resolution.is_valid());
        assert_eq!(resolution.total_instances, 5);
        assert_eq!(resolution.resolved_instances, 5);
        assert_eq!(resolution.bindings.len(), 3);
        let amp = resolution
            .bindings
            .iter()
            .find(|row| row.reference.cell == "amp")
            .expect("amp row");
        assert_eq!(amp.instance_count, 2);
        assert_eq!(amp.purpose, "design under test");
        assert_eq!(
            amp.view_search_order
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["schematic", "extracted", "spice"]
        );
        assert_eq!(amp.stop_view.as_deref(), Some("spice"));
        let bias = resolution
            .bindings
            .iter()
            .find(|row| row.reference.cell == "bias")
            .expect("bias row");
        assert_eq!(bias.instance_count, 2);
        assert_eq!(bias.purpose, "hierarchical cell");
    }

    #[test]
    fn active_configuration_drives_exact_path_resolution_and_receipt_identity() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "amp",
            SchematicState::default(),
        );

        let id = workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Lab characterization".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: vec![crate::state::ConfigurationSetOverride {
                    instance_path: "/top/X2".to_owned(),
                    executable_views: vec!["spice".to_owned()],
                    stop_view: Some("spice".to_owned()),
                    model_section: Some("tt".to_owned()),
                    eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
                }],
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Analog design".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.configuration_id, Some(id));
        assert_eq!(resolution.configuration_revision, Some(1));
        assert_eq!(
            resolution.configuration_digest,
            workspace
                .configuration_sets
                .find(id)
                .map(|configuration| configuration.semantic_digest())
        );
        assert_eq!(resolution.total_instances, 3);
        assert_eq!(resolution.resolved_instances, 2);
        assert_eq!(resolution.unresolved_instances(), 1);
        let configured = resolution
            .bindings
            .iter()
            .find(|binding| binding.instance_paths == ["/top/X2"])
            .expect("exact overridden instance row");
        assert_eq!(configured.view_search_order, ["spice"]);
        assert_eq!(configured.model_section, "tt");
        assert_eq!(configured.status, HierarchyBindingStatus::Unresolved);
        assert!(resolution.bindings.iter().any(|binding| {
            binding.instance_paths.iter().any(|path| path == "/top/X1")
                && binding.status.is_resolved()
        }));
    }

    #[test]
    fn active_configuration_rejects_missing_dut_and_override_paths() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Missing bindings".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/XMISSING".to_owned(),
                executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: vec![crate::state::ConfigurationSetOverride {
                    instance_path: "/top/XOTHER".to_owned(),
                    executable_views: vec!["schematic".to_owned()],
                    stop_view: None,
                    model_section: None,
                    eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
                }],
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.total_instances, 3);
        assert_eq!(resolution.resolved_instances, 1);
        assert_eq!(resolution.unresolved_instances(), 2);
        assert!(resolution.bindings.iter().any(|binding| {
            binding.diagnostic.as_deref().is_some_and(|diagnostic| {
                diagnostic.contains("configured DUT path /top/XMISSING does not exist")
            })
        }));
        assert!(resolution.bindings.iter().any(|binding| {
            binding.diagnostic.as_deref().is_some_and(|diagnostic| {
                diagnostic.contains("scoped configuration override /top/XOTHER does not exist")
            })
        }));
    }

    #[test]
    fn reviewed_fallback_is_resolved_and_retained_in_the_hierarchy_receipt() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
        add_schematic_master(
            &mut libraries,
            &mut workspace,
            "work",
            "amp",
            SchematicState::default(),
        );
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Reviewed fallback".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["spice".to_owned()],
                stop_views: vec!["spice".to_owned()],
                unresolved_policy:
                    crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("create configuration");

        let resolution = workspace.resolve_hierarchy(&libraries);
        let fallback = resolution
            .bindings
            .iter()
            .find(|binding| binding.instance_paths == ["/top/X1"])
            .expect("child binding");

        assert!(fallback.status.is_resolved());
        assert!(fallback.used_review_fallback);
        assert_eq!(fallback.reference.view, "schematic");
        assert_eq!(
            fallback.view_search_order,
            ["spice", "schematic", "extracted"]
        );
    }

    #[test]
    fn configuration_catalog_replacement_advances_project_revision_atomically() {
        let mut workspace = ProjectWorkspace::default();
        let original_revision = workspace.project.revision;
        let mut candidate = workspace.configuration_sets.clone();
        candidate
            .create(crate::state::ConfigurationSetDefinition {
                name: "Release".to_owned(),
                root: CellViewRef::default_top(),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("candidate configuration");

        let committed_revision = workspace
            .replace_configuration_sets(candidate.clone())
            .expect("publish configuration catalog");
        assert_eq!(workspace.project.revision, committed_revision);
        assert_ne!(workspace.project.revision, original_revision);
        assert_eq!(workspace.configuration_sets, candidate);
        assert!(workspace.project_metadata_dirty);

        let committed = workspace.clone();
        assert_eq!(
            workspace.replace_configuration_sets(candidate),
            Err(ProjectConfigurationMutationError::NoChanges)
        );
        assert_eq!(workspace.project.revision, committed.project.revision);
        assert_eq!(workspace.configuration_sets, committed.configuration_sets);
    }

    #[test]
    fn configuration_catalog_replacement_rejects_unmaterialized_roots_atomically() {
        let mut workspace = ProjectWorkspace::default();
        let before = workspace.clone();
        let mut candidate = crate::state::ConfigurationSetCatalog::default();
        candidate
            .create(crate::state::ConfigurationSetDefinition {
                name: "Missing root".to_owned(),
                root: CellViewRef::new("user", "missing", "schematic"),
                dut_path: "/top/X1".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("structurally valid candidate");

        assert!(matches!(
            workspace.replace_configuration_sets(candidate),
            Err(ProjectConfigurationMutationError::MissingRootBuffer { .. })
        ));
        assert_eq!(workspace.project.revision, before.project.revision);
        assert_eq!(workspace.configuration_sets, before.configuration_sets);
        assert_eq!(
            workspace.project_metadata_dirty,
            before.project_metadata_dirty
        );
    }

    #[test]
    fn design_management_projection_namespaces_sheets_and_materializes_explicit_ports() {
        use crate::state::{
            CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
            CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
            MoveBoundaryResolution, MoveSelectionRequest, SheetDefinition, SheetPortPolicy,
            SheetTemplate,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let first = schematic
            .add_wire(vec![Point::origin(), Point::new(10, 0)])
            .expect("first wire");
        let second = schematic
            .add_wire(vec![Point::origin(), Point::new(0, 10)])
            .expect("second wire");
        let component = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
        let terminal_name = schematic
            .components
            .iter()
            .find(|candidate| candidate.id == component)
            .expect("component")
            .terminal_positions_resolved(None)
            .into_iter()
            .find(|(_, point)| *point == Point::origin())
            .map(|(name, _)| name)
            .expect("terminal at the second-wire anchor");
        schematic
            .connections
            .push(crate::state::WireConnection::new(
                second,
                0,
                component,
                terminal_name.clone(),
            ));
        let source_sheet = workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Input", [first, second, component])
            .expect("bootstrap sheet ownership");
        let catalog = workspace
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let destination_sheet = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(source_sheet),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![second, component],
                destination_sheet_id: destination_sheet,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: source_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: first,
                                point: Point::origin(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination_sheet,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: component,
                                terminal_name,
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("move with explicit boundary contract");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize governed design");
        let first_position = projected
            .wires
            .iter()
            .find(|wire| wire.id == first)
            .and_then(|wire| wire.points.first())
            .copied()
            .expect("first wire");
        let second_position = projected
            .wires
            .iter()
            .find(|wire| wire.id == second)
            .and_then(|wire| wire.points.first())
            .copied()
            .expect("second wire");
        assert_ne!(first_position, second_position);
        assert_eq!(first_position, Point::origin());
        assert_eq!(second_position, Point::new(1_000_000, 0));

        let mut port_positions = projected
            .net_labels
            .iter()
            .filter(|label| label.name == "BIAS")
            .map(|label| label.pos)
            .collect::<Vec<_>>();
        port_positions.sort_by_key(|point| point.x);
        assert_eq!(port_positions, [first_position, second_position]);
    }

    #[test]
    fn design_management_projection_applies_active_variant_and_annotation() {
        use std::collections::BTreeMap;

        use crate::state::{
            AnnotationObject, AnnotationPosition, AssemblyVariantDraft, ComponentSubstitution,
            ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope,
            SchematicObjectKey, VariantInheritance, VariantObjectOverride,
            VariantQualificationPlan, VariantQualificationState,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let substituted = schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        let omitted = schematic.add_component(ComponentType::Capacitor, Point::new(20, 10));
        let variant = workspace
            .design_management
            .variants_mut()
            .create(AssemblyVariantDraft {
                name: "Automotive".to_owned(),
                parent_id: None,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: BTreeMap::from([
                    (
                        SchematicObjectKey::new(&key, substituted)
                            .expect("scoped substituted identity"),
                        VariantObjectOverride::Substitute {
                            replacement: ComponentSubstitution {
                                library: "qualified".to_owned(),
                                cell: "resistor_aecq".to_owned(),
                                view: "schematic".to_owned(),
                                value_override: Some("2 kohm".to_owned()),
                                model_section: Some("automotive".to_owned()),
                                port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
                                qualification: VariantQualificationState::Current,
                            },
                        },
                    ),
                    (
                        SchematicObjectKey::new(&key, omitted).expect("scoped omitted identity"),
                        VariantObjectOverride::DoNotPopulate {
                            approval_reference: "ECO-104".to_owned(),
                        },
                    ),
                ]),
            })
            .expect("create governed variant");
        workspace
            .design_management
            .variants_mut()
            .set_active(variant)
            .expect("activate variant");

        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: SchematicObjectKey::new(&key, substituted)
                    .expect("scoped annotation identity"),
                current_reference: "R42".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: None,
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition { x: 10, y: 10 },
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let preview = workspace
            .design_management
            .annotation()
            .preview_renumbering(&request)
            .expect("preview annotation");
        workspace
            .design_management
            .annotation_mut()
            .commit_renumbering(&preview, &request)
            .expect("commit annotation receipt");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize variant and annotation");
        assert!(
            projected
                .components
                .iter()
                .all(|component| component.id != omitted)
        );
        assert!(
            projected
                .connections
                .iter()
                .all(|connection| connection.component_id != omitted)
        );
        let component = projected
            .components
            .iter()
            .find(|component| component.id == substituted)
            .expect("substituted component");
        let binding = component
            .library_cell
            .as_ref()
            .expect("qualified cell binding");
        assert_eq!(binding.library, "qualified");
        assert_eq!(binding.cell, "resistor_aecq");
        assert_eq!(component.value, "2 kohm");
        assert!(component.params.contains("model_section=automotive"));
        assert_eq!(component.name, "R1");
    }

    #[test]
    fn hierarchy_resolution_reports_unbound_and_recursive_masters() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), instance("missing", "unbound"));

        let unresolved = workspace.resolve_hierarchy(&libraries);
        assert_eq!(unresolved.total_instances, 2);
        assert_eq!(unresolved.resolved_instances, 1);
        assert_eq!(unresolved.unresolved_instances(), 1);
        assert_eq!(
            unresolved.bindings[1].status,
            HierarchyBindingStatus::Unresolved
        );
        assert!(unresolved.bindings[1].diagnostic.is_some());

        let top = workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer");
        top.components.clear();
        top.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
        let mut loop_master = SchematicState::default();
        loop_master.add_library_cell_component(Point::new(20, 20), instance("work", "loop"));
        add_schematic_master(&mut libraries, &mut workspace, "work", "loop", loop_master);

        let recursive = workspace.resolve_hierarchy(&libraries);
        assert_eq!(recursive.total_instances, 3);
        assert_eq!(recursive.resolved_instances, 2);
        let loop_row = recursive
            .bindings
            .iter()
            .find(|row| row.reference.cell == "loop")
            .expect("loop row");
        assert_eq!(loop_row.instance_count, 2);
        assert_eq!(loop_row.status, HierarchyBindingStatus::Recursive);
        assert!(
            loop_row
                .diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("work/loop → work/loop"))
        );
    }

    #[test]
    fn hierarchy_resolution_projects_unsaved_active_topology() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let mut live = workspace
            .schematic_buffers
            .get(&CellViewRef::default_top().key())
            .expect("top buffer")
            .clone();
        live.add_library_cell_component(Point::new(20, 20), instance("missing", "live_child"));

        let persisted = workspace.resolve_hierarchy(&libraries);
        let projected =
            workspace.resolve_hierarchy_with_active(&libraries, &workspace.active_view, &live);

        assert_eq!(persisted.total_instances, 1);
        assert_eq!(projected.total_instances, 2);
        assert_eq!(projected.unresolved_instances(), 1);
        assert!(
            projected
                .bindings
                .iter()
                .any(|binding| binding.reference.cell == "live_child"
                    && binding.status == HierarchyBindingStatus::Unresolved)
        );
    }

    #[test]
    fn hierarchy_resolution_rejects_orphan_schematic_buffers() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), instance("orphan", "amp"));
        workspace.schematic_buffers.insert(
            CellViewRef::new("orphan", "amp", "schematic").key(),
            SchematicState::default(),
        );

        let resolution = workspace.resolve_hierarchy(&libraries);

        assert_eq!(resolution.unresolved_instances(), 1);
        assert!(
            resolution
                .bindings
                .iter()
                .any(|binding| binding.reference.cell == "amp"
                    && binding.status == HierarchyBindingStatus::Unresolved)
        );
    }

    #[test]
    fn configuration_veriloga_binding_uses_exact_project_bundle_on_all_targets() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let reference = CellViewRef::new("models", "amp", "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "project_amp".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
        let mut cell = Cell::new("amp");
        cell.add_view(view);
        let mut library = Library::new("models");
        library.add_cell(cell);
        libraries.add_library(library);

        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            "models/amp.va",
            "module project_amp(input in, output out); electrical in, out; analog V(out) <+ V(in); endmodule\n",
            [],
            [],
        )
        .expect("valid project source bundle");
        let bundle_id = bundle.id();
        workspace
            .project_sources
            .insert_bundle(bundle)
            .expect("attach project source bundle");

        let mut placed = LibraryCellInstance::new("models", "amp", "schematic");
        placed.terminal_order = vec!["in".to_owned(), "out".to_owned()];
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), placed);
        workspace
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: CellViewRef::default_top(),
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
            .expect("create mixed-signal configuration");

        let active = workspace
            .active_schematic()
            .expect("active schematic")
            .clone();
        let projection = workspace
            .configuration_execution_projection(&libraries, &CellViewRef::default_top(), &active)
            .expect("resolve project-owned Verilog-A binding");
        let execution = projection
            .plan()
            .and_then(|plan| plan.binding("/top/X1"))
            .expect("exact execution binding");
        let behavioral = execution
            .project_veriloga()
            .expect("project Verilog-A contract");
        assert_eq!(behavioral.source_bundle_id(), bundle_id);
        assert_eq!(behavioral.selected_module(), "project_amp");
        assert!(behavioral.source_key().starts_with("__rspice_project__/"));
        assert_eq!(
            execution
                .materialized_binding()
                .and_then(|binding| binding.source_path.as_deref()),
            Some(Path::new(behavioral.source_key()))
        );
        assert_eq!(
            execution
                .materialized_binding()
                .and_then(|binding| binding.module_name.as_deref()),
            Some(behavioral.netlist_alias())
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn hierarchy_resolution_rejects_missing_and_conflicting_source_bindings() {
        let mut workspace = ProjectWorkspace::default();
        let mut libraries = LibraryManager::default();
        workspace.ensure_library_model(&mut libraries);
        let base = std::env::temp_dir().join(format!("rspice-hierarchy-{}", Uuid::new_v4()));
        let authoritative = base.join("amp.cir");
        let conflicting = base.join("other.cir");
        std::fs::create_dir_all(&base).expect("create source fixture directory");
        std::fs::write(&authoritative, ".subckt amp in out\n.ends amp\n")
            .expect("write authoritative source");
        std::fs::write(&conflicting, ".subckt amp in out\n.ends amp\n")
            .expect("write conflicting source");

        let missing_path = base.join("missing.cir");
        let mut library = Library::new("models");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("spice", ViewType::Spice).with_path(missing_path.clone()));
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("models", "amp", "spice");
        binding.terminal_order = vec!["in".to_owned(), "out".to_owned()];
        binding.source_path = Some(missing_path);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .add_library_cell_component(Point::new(20, 20), binding.clone());

        let missing = workspace.resolve_hierarchy(&libraries);
        assert_eq!(missing.unresolved_instances(), 1);
        assert!(missing.bindings.iter().any(|row| {
            row.diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("cannot read"))
        }));

        libraries
            .get_library_mut("models")
            .and_then(|library| library.get_cell_mut("amp"))
            .and_then(|cell| cell.get_view_mut("spice"))
            .expect("authoritative source view")
            .file_path = Some(authoritative);
        binding.source_path = Some(conflicting);
        workspace
            .schematic_buffers
            .get_mut(&CellViewRef::default_top().key())
            .expect("top buffer")
            .components
            .last_mut()
            .expect("source-backed instance")
            .library_cell = Some(binding);
        let conflicting = workspace.resolve_hierarchy(&libraries);
        assert_eq!(conflicting.unresolved_instances(), 1);
        assert!(conflicting.bindings.iter().any(|row| {
            row.diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("conflicts"))
        }));

        std::fs::remove_dir_all(base).expect("remove source fixture directory");
    }

    #[test]
    fn descend_records_the_instance_names() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
        workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1", "XB"]);
        assert_eq!(workspace.active_view.cell, "bias_2t");
    }

    #[test]
    fn breadcrumb_focus_truncates_the_occurrence_path() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        workspace.descend_into("X1".into(), reference("ota_5t"), ViewType::Schematic);
        workspace.descend_into("XB".into(), reference("bias_2t"), ViewType::Schematic);

        workspace.focus_breadcrumb(1);
        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "X1"]);
        assert_eq!(workspace.active_view.cell, "ota_5t");

        workspace.ascend_one();
        assert_eq!(workspace.occurrence_labels(), ["tb_ota"]);
        assert_eq!(workspace.active_view.cell, "tb_ota");
        // At the root, ascending is a no-op.
        assert!(workspace.ascend_one().is_none());
    }

    #[test]
    fn legacy_stacks_fall_back_to_cell_names() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(reference("tb_ota"), ViewType::Schematic);
        // Simulate an older save: stack grew without instance labels.
        workspace.hierarchy_stack.push(reference("ota_5t"));
        assert_eq!(workspace.occurrence_labels(), ["tb_ota", "ota_5t"]);
    }

    #[test]
    fn symbol_active_view_does_not_allocate_schematic_buffer() {
        let reference = symbol_reference("ota_5t");
        let mut workspace = ProjectWorkspace {
            active_view: reference.clone(),
            open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
            hierarchy_stack: vec![reference.clone()],
            schematic_buffers: HashMap::new(),
            ..ProjectWorkspace::default()
        };
        let mut libraries = LibraryManager::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("ota_5t");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        libraries.add_library(library);

        workspace.ensure_library_model(&mut libraries);

        assert_eq!(workspace.active_view_type(), ViewType::Symbol);
        assert!(
            !workspace.schematic_buffers.contains_key(&reference.key()),
            "symbol views must not be backed by stale schematic buffers"
        );
        let symbol_view = libraries
            .get_library("work")
            .and_then(|library| library.get_cell("ota_5t"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view still exists");
        assert_eq!(symbol_view.view_type, ViewType::Symbol);
    }

    #[test]
    fn saving_while_symbol_active_does_not_create_symbol_schematic_buffer() {
        let reference = symbol_reference("ota_5t");
        let mut workspace = ProjectWorkspace {
            active_view: reference.clone(),
            open_views: vec![OpenCellView::new(reference.clone(), ViewType::Symbol)],
            hierarchy_stack: vec![reference.clone()],
            schematic_buffers: HashMap::new(),
            ..ProjectWorkspace::default()
        };

        workspace.save_active_schematic(&SchematicState::default());

        assert!(
            !workspace.schematic_buffers.contains_key(&reference.key()),
            "session restore/save paths must not persist default schematics under symbol views"
        );
    }

    #[test]
    fn project_identity_is_stable_and_rename_is_atomic() {
        let mut project = ProjectDescriptor::default();
        let id = project.id();
        let initial_revision = project.revision();

        let renamed_revision = project
            .rename("Precision ΔΣ ADC")
            .expect("valid Unicode name");

        assert_eq!(project.id(), id);
        assert_eq!(project.name(), "Precision ΔΣ ADC");
        assert_eq!(renamed_revision.get(), initial_revision.get() + 1);
        assert_eq!(
            project.rename("Precision ΔΣ ADC").expect("no-op rename"),
            renamed_revision
        );

        let rejected = project.rename("bad/name");
        assert!(matches!(
            rejected,
            Err(ProjectDescriptorError::PathSeparator('/'))
        ));
        assert_eq!(project.name(), "Precision ΔΣ ADC");
        assert_eq!(project.revision(), renamed_revision);
        assert_eq!(project.id(), id);
    }

    #[test]
    fn legacy_project_descriptor_identity_migration_is_deterministic() {
        let original = ProjectDescriptor::default();
        let mut legacy = serde_json::to_value(&original).expect("descriptor serializes");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("id");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("schema_version");
        legacy
            .as_object_mut()
            .expect("descriptor is an object")
            .remove("revision");
        let legacy_json = serde_json::to_string(&legacy).expect("legacy descriptor serializes");

        let first: ProjectDescriptor =
            serde_json::from_str(&legacy_json).expect("legacy descriptor restores");
        let second: ProjectDescriptor =
            serde_json::from_str(&legacy_json).expect("legacy descriptor restores again");

        assert_eq!(first.id(), second.id());
        assert!(!first.id().as_uuid().is_nil());
        assert_ne!(first.id(), original.id());

        let persisted = serde_json::to_value(&first).expect("migrated descriptor serializes");
        assert_eq!(
            persisted.get("id"),
            Some(&serde_json::to_value(first.id()).expect("identity serializes"))
        );
    }

    #[test]
    fn versioned_or_explicitly_null_project_identity_and_schema_are_rejected() {
        let project = ProjectDescriptor::default();
        let mut missing = serde_json::to_value(&project).expect("descriptor serializes");
        missing
            .as_object_mut()
            .expect("descriptor object")
            .remove("id");
        let missing_error = serde_json::from_value::<ProjectDescriptor>(missing)
            .expect_err("versioned descriptor must retain identity");
        assert!(
            missing_error
                .to_string()
                .contains("missing its stable identity")
        );

        let mut null = serde_json::to_value(&project).expect("descriptor serializes");
        null["id"] = serde_json::Value::Null;
        let null_error = serde_json::from_value::<ProjectDescriptor>(null)
            .expect_err("explicit null identity is not legacy absence");
        assert!(
            null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut unversioned_null = serde_json::to_value(&project).expect("descriptor serializes");
        unversioned_null
            .as_object_mut()
            .expect("descriptor object")
            .remove("schema_version");
        unversioned_null["id"] = serde_json::Value::Null;
        let unversioned_null_error = serde_json::from_value::<ProjectDescriptor>(unversioned_null)
            .expect_err("unversioned explicit null is not genuine legacy absence");
        assert!(
            unversioned_null_error
                .to_string()
                .contains("must not be explicitly null")
        );

        let mut null_schema = serde_json::to_value(&project).expect("descriptor serializes");
        null_schema["schema_version"] = serde_json::Value::Null;
        let null_schema_error = serde_json::from_value::<ProjectDescriptor>(null_schema)
            .expect_err("explicit null schema is not an unversioned descriptor");
        assert!(
            null_schema_error
                .to_string()
                .contains("schema version must not be explicitly null")
        );
    }

    #[test]
    fn project_name_contract_counts_graphemes_and_rejects_unsafe_text() {
        let family = "👨‍👩‍👧‍👦";
        assert!(ProjectDescriptor::validate_name(&family.repeat(120)).is_ok());
        assert!(matches!(
            ProjectDescriptor::validate_name(&family.repeat(121)),
            Err(ProjectDescriptorError::NameTooLong {
                grapheme_count: 121
            })
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name(" leading"),
            Err(ProjectDescriptorError::SurroundingWhitespace)
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name("line\nfeed"),
            Err(ProjectDescriptorError::ControlCharacter('\n'))
        ));
        assert!(matches!(
            ProjectDescriptor::validate_name("path\\name"),
            Err(ProjectDescriptorError::PathSeparator('\\'))
        ));
    }

    #[test]
    fn cell_view_name_contract_keeps_slash_delimited_keys_injective() {
        for valid in ["user", "bandgap_2", "ΔΣ"] {
            assert!(validate_cell_view_name_segment(valid).is_ok(), "{valid}");
        }
        assert_eq!(
            validate_cell_view_name_segment(""),
            Err(CellViewNameError::Empty)
        );
        assert_eq!(
            validate_cell_view_name_segment("bad/name"),
            Err(CellViewNameError::UnsupportedCharacter('/'))
        );
        assert_eq!(
            validate_cell_view_name_segment("has space"),
            Err(CellViewNameError::UnsupportedCharacter(' '))
        );
    }

    #[test]
    fn changing_source_path_does_not_rename_an_existing_project() {
        let mut project = ProjectDescriptor::default();
        project.set_path(PathBuf::from("first-save.rspiceproj"));
        let revision = project.revision();

        assert_eq!(project.name(), "first-save");
        project.set_path(PathBuf::from("moved-copy.rspiceproj"));

        assert_eq!(project.name(), "first-save");
        assert_eq!(project.revision(), revision);
        assert_eq!(
            project.path.as_deref(),
            Some(Path::new("moved-copy.rspiceproj"))
        );
    }

    #[test]
    fn project_copy_has_independent_identity_without_rebinding_source() {
        let mut source = ProjectDescriptor::default();
        source
            .rename("Precision reference")
            .expect("source name is valid");
        source.set_path(PathBuf::from("source.rspiceproj"));
        let source_id = source.id();
        let source_revision = source.revision();
        let source_path = source.path.clone();

        let copy = source.fork_copy_at(PathBuf::from("copy.rspiceproj"));

        assert_ne!(copy.id(), source_id);
        assert_eq!(copy.revision(), ObjectRevision::INITIAL);
        assert_eq!(copy.name(), source.name());
        assert_eq!(copy.path.as_deref(), Some(Path::new("copy.rspiceproj")));
        assert_eq!(source.id(), source_id);
        assert_eq!(source.revision(), source_revision);
        assert_eq!(source.path, source_path);
    }

    #[test]
    fn generated_netlist_cannot_be_promoted_by_an_editor_write() {
        let mut workspace = ProjectWorkspace::default();

        assert!(!workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
        assert!(workspace.netlist_source.is_none());
        assert!(!workspace.netlist_source_dirty);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn explicit_editable_copy_enters_project_dirty_lifecycle() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source_path = Some(PathBuf::from("generated.sp"));

        assert!(workspace.make_netlist_editable_copy("generated\n.op\n.end\n"));
        assert_eq!(
            workspace.netlist_source.as_deref(),
            Some("generated\n.op\n.end\n")
        );
        assert!(workspace.netlist_source_path.is_none());
        assert!(workspace.netlist_source_dirty);
        assert!(workspace.any_dirty());

        workspace.mark_all_clean();
        assert!(workspace.has_editable_netlist_source());
        assert!(!workspace.netlist_source_dirty);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn editable_copy_does_not_overwrite_existing_owned_source() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

        assert!(!workspace.make_netlist_editable_copy("generated\n.end\n"));
        assert_eq!(workspace.netlist_source.as_deref(), Some("owned\n.end\n"));
        assert_eq!(
            workspace.netlist_source_path.as_deref(),
            Some(Path::new("owned.cir"))
        );
        assert!(!workspace.netlist_source_dirty);
    }

    #[test]
    fn editing_imported_source_preserves_its_dependency_origin() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("decks/owned.cir"));

        assert!(workspace.replace_editable_netlist_source("edited\n.end\n".to_owned()));
        assert_eq!(
            workspace.netlist_source_path.as_deref(),
            Some(Path::new("decks/owned.cir"))
        );
        assert!(workspace.netlist_source_dirty);
    }

    #[test]
    fn returning_to_generated_output_is_saved_as_a_project_change() {
        let mut workspace = ProjectWorkspace::default();
        workspace.netlist_source = Some("owned\n.end\n".to_owned());
        workspace.netlist_source_path = Some(PathBuf::from("owned.cir"));

        assert!(workspace.return_to_generated_netlist());
        assert!(workspace.netlist_source.is_none());
        assert!(workspace.netlist_source_path.is_none());
        assert!(workspace.netlist_source_dirty);
        assert!(workspace.any_dirty());
        assert!(!workspace.return_to_generated_netlist());
    }

    fn technology_binding_fixture() -> ProjectTechnologyBinding {
        let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
        ProjectTechnologyBinding {
            schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            package_name: "Qualified analog models".to_owned(),
            package_version: Some("2026.07".to_owned()),
            technology_node: Some("180 nm".to_owned()),
            model_library: "qualified_analog".to_owned(),
            root_source: root.clone(),
            source_closure: vec![crate::state::model_library::ModelSourcePin {
                path: root,
                digest: crate::product::ContentDigest::from_bytes([0x4a; 32]),
            }],
            source_edges: Vec::new(),
            model_count: 14,
            process_sections: vec!["ff".to_owned(), "ss".to_owned(), "tt".to_owned()],
        }
    }

    #[test]
    fn technology_attachment_is_atomic_revisioned_and_idempotent() {
        let mut project = ProjectDescriptor::default();
        let initial_revision = project.revision();
        let binding = technology_binding_fixture();

        let committed = project
            .attach_technology(binding.clone())
            .expect("valid binding commits");
        assert_eq!(committed.get(), initial_revision.get() + 1);
        assert_eq!(project.technology_binding(), Some(&binding));
        assert_eq!(
            project.technology.as_deref(),
            Some(binding.display_label().as_str())
        );
        assert_eq!(
            project
                .attach_technology(binding)
                .expect("identical binding is a no-op"),
            committed
        );

        let mut rejected = technology_binding_fixture();
        rejected.model_count = 0;
        let before = project.clone();
        assert!(matches!(
            project.attach_technology(rejected),
            Err(ProjectDescriptorError::Technology(
                TechnologyBindingError::NoModels
            ))
        ));
        assert_eq!(project.revision(), before.revision());
        assert_eq!(project.technology, before.technology);
        assert_eq!(project.technology_binding(), before.technology_binding());
    }

    #[test]
    fn attached_technology_detects_exact_catalog_drift() {
        let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
        let bytes = b".model nch nmos level=1\n".to_vec();
        let digest = crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into());
        let mut library = crate::state::model_library::ModelLibrary::new("qualified_analog")
            .with_technology("Qualified analog models", "180 nm");
        library.version = "2026.07".to_owned();
        library.root_path = Some(root.clone());
        library.source_closure = vec![crate::state::model_library::ModelSourcePin {
            path: root.clone(),
            digest,
        }];
        library.source_contents =
            vec![crate::state::model_library::ModelSourceContent { path: root, bytes }];
        library.add_model(crate::state::model_library::DeviceModel::new(
            "nch",
            crate::state::model_library::ModelType::Nmos,
        ));
        let binding = ProjectTechnologyBinding::from_model_library(&library)
            .expect("exact retained source is attachable");
        binding
            .validate_model_library(&library)
            .expect("unchanged catalog matches");

        library.version = "2026.08".to_owned();
        assert!(matches!(
            binding.validate_model_library(&library),
            Err(TechnologyBindingError::CatalogDrift { .. })
        ));
    }

    #[test]
    fn technology_binding_persists_while_runtime_dirty_state_resets() {
        let mut workspace = ProjectWorkspace::default();
        let binding = technology_binding_fixture();
        workspace
            .attach_technology(binding.clone())
            .expect("valid binding commits");
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
        let restored: ProjectWorkspace =
            serde_json::from_slice(&bytes).expect("workspace restores");

        assert_eq!(restored.project.technology_binding(), Some(&binding));
        restored
            .project
            .validate()
            .expect("restored binding validates");
        assert!(!restored.any_dirty());
    }

    #[test]
    fn hardcopy_page_setup_persists_and_uses_project_dirty_lifecycle() {
        use crate::hardcopy::{
            ActiveHardcopySource, HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope,
            HardcopySetup, SetupSaveDisposition,
        };

        let source = ActiveHardcopySource::try_new(
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4852_4450_5901))
                .expect("stable fixture identity"),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x48; 32]),
            "top / schematic",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .expect("valid hardcopy source");
        let mut workspace = ProjectWorkspace::default();

        let first = workspace
            .save_hardcopy_setup(&source, HardcopySetup::default())
            .expect("page setup commits");
        assert_eq!(first.disposition(), SetupSaveDisposition::Inserted);
        assert!(workspace.hardcopy_setups_dirty);
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).expect("workspace serializes");
        let mut restored: ProjectWorkspace =
            serde_json::from_slice(&bytes).expect("workspace restores");
        assert_eq!(restored.hardcopy_setups.len(), 1);
        assert!(!restored.hardcopy_setups_dirty);
        assert!(!restored.any_dirty());

        let unchanged = restored
            .save_hardcopy_setup(&source, HardcopySetup::default())
            .expect("identical setup is accepted");
        assert_eq!(unchanged.disposition(), SetupSaveDisposition::Unchanged);
        assert!(!restored.hardcopy_setups_dirty);
        assert!(!restored.any_dirty());
    }

    #[test]
    fn project_print_mapping_routes_through_project_dirty_lifecycle() {
        let mapping = crate::hardcopy::PrintMappingTable::try_new(
            crate::hardcopy::PrintMappingSaveScope::ProjectPrintSet(
                "documentation".to_owned(),
            ),
            Vec::new(),
        )
        .unwrap();
        let mut workspace = ProjectWorkspace::default();
        let receipt = workspace
            .save_project_print_mapping(mapping.clone())
            .unwrap();
        assert_eq!(
            receipt.disposition(),
            crate::hardcopy::PrintMappingSaveDisposition::Created
        );
        assert!(workspace.project_print_mappings_dirty);
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).unwrap();
        let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
        assert!(
            restored
                .project_print_mappings
                .get("documentation")
                .is_some()
        );
        assert!(!restored.any_dirty());

        let unchanged = restored.save_project_print_mapping(mapping).unwrap();
        assert_eq!(
            unchanged.disposition(),
            crate::hardcopy::PrintMappingSaveDisposition::Unchanged
        );
        assert!(!restored.any_dirty());
    }

    #[test]
    fn hardcopy_source_sets_persist_validate_and_use_project_dirty_lifecycle() {
        use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
        use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};

        let member_id =
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_4d45_4d42_4552))
                .unwrap();
        let set_id =
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4853_5345_5449_4431))
                .unwrap();
        let member = HardcopySourceSetMember::try_new(
            "project:test:sheet:1",
            "Sheet 1",
            member_id,
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x51; 32]),
            HardcopyScope::CurrentSheet,
        )
        .unwrap();
        let source_set = HardcopySourceSet::try_new(
            set_id,
            crate::product::ObjectRevision::INITIAL,
            "Review set",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet("Review set".to_owned()),
            vec![member],
        )
        .unwrap();
        let source_key = source_set.source_key();
        let mut workspace = ProjectWorkspace::default();

        assert!(workspace.save_hardcopy_source_set(source_set).unwrap());
        assert!(!workspace.hardcopy_source_sets().is_empty());
        assert!(workspace.hardcopy_source_set(&source_key).is_some());
        assert!(workspace.any_dirty());

        let bytes = serde_json::to_vec(&workspace).unwrap();
        let mut restored: ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
        restored.validate_simulation_configuration().unwrap();
        assert_eq!(restored.hardcopy_source_sets().len(), 1);
        assert!(!restored.any_dirty());
        assert!(restored.remove_hardcopy_source_set(&source_key));
        assert!(restored.hardcopy_source_sets().is_empty());
        assert!(restored.any_dirty());
    }

    #[test]
    fn hardcopy_source_set_catalog_rejects_case_folded_duplicate_names() {
        use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
        use crate::hardcopy::sources::{HardcopySourceSet, HardcopySourceSetMember};

        let build_set = |seed: u128, name: &str| {
            let member_id = HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed)).unwrap();
            let set_id =
                HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(seed + 0x1000)).unwrap();
            let member = HardcopySourceSetMember::try_new(
                format!("project:test:sheet:{seed}"),
                format!("Sheet {seed}"),
                member_id,
                crate::product::ObjectRevision::INITIAL,
                crate::product::ContentDigest::from_bytes([(seed & 0xff) as u8; 32]),
                HardcopyScope::CurrentSheet,
            )
            .unwrap();
            HardcopySourceSet::try_new(
                set_id,
                crate::product::ObjectRevision::INITIAL,
                name,
                HardcopyDocumentKind::SchematicOrSymbol,
                HardcopyScope::NamedPrintSet(name.to_owned()),
                vec![member],
            )
            .unwrap()
        };
        let mut workspace = ProjectWorkspace::default();
        workspace
            .save_hardcopy_source_set(build_set(0x5100, "Tapeout"))
            .unwrap();
        let error = workspace
            .save_hardcopy_source_set(build_set(0x5200, "tapeout"))
            .unwrap_err();
        assert!(matches!(
            error,
            HardcopySourceSetPersistenceError::DuplicateName { .. }
        ));
        assert_eq!(workspace.hardcopy_source_sets().len(), 1);
    }

    #[test]
    fn corrupted_persisted_technology_contract_fails_project_validation() {
        let mut project = ProjectDescriptor::default();
        project
            .attach_technology(technology_binding_fixture())
            .expect("fixture binding commits");
        let mut encoded = serde_json::to_value(&project).expect("descriptor serializes");
        encoded["technology_binding"]["root_source"] =
            serde_json::Value::String("relative/models.lib".to_owned());
        let restored: ProjectDescriptor =
            serde_json::from_value(encoded).expect("descriptor shape restores");

        assert!(matches!(
            restored.validate(),
            Err(ProjectDescriptorError::Technology(
                TechnologyBindingError::NonAbsoluteSource(_)
            ))
        ));
    }

    #[test]
    fn legacy_workspaces_restore_with_no_project_source_examples() {
        let mut value = serde_json::to_value(ProjectWorkspace::default()).unwrap();
        value.as_object_mut().unwrap().remove("project_sources");

        let restored: ProjectWorkspace = serde_json::from_value(value).unwrap();

        assert!(restored.project_sources.is_empty());
        assert!(!restored.project_sources_dirty);
    }

    #[test]
    fn only_bootstrapped_projects_receive_exact_mockup_sources() {
        let mut libraries = LibraryManager::default();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let verilog_a = workspace
            .project_sources
            .get(ProjectSourceLanguage::VerilogA)
            .unwrap();
        let automation = workspace
            .project_sources
            .get(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();

        assert_eq!(verilog_a.file_name(), "sensor_bridge.va");
        assert_eq!(
            verilog_a.content(),
            "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule"
        );
        assert_eq!(automation.file_name(), "characterize.rspice");
        assert_eq!(
            automation.content(),
            "plan = project.plan(\"Lab characterization\")\nrun = plan.with_corners(\"all\").execute(target=\"local\")\nrun.require(specs=\"release\")\nrun.compare(baseline=\"main\", waveforms=True)\nrun.export([\"junit\", \"summary.json\", \"report.pdf\"])",
        );
        assert!(!workspace.any_dirty());
        assert!(ProjectWorkspace::default().project_sources.is_empty());
    }

    #[test]
    fn file_new_bootstrap_is_empty_but_keeps_a_valid_project_hierarchy() {
        let mut libraries = LibraryManager::default();
        let workspace = ProjectWorkspace::new_empty_bootstrapped(&mut libraries);

        assert!(workspace.project_sources.is_empty());
        assert!(!workspace.project_sources_dirty);
        assert!(
            libraries
                .get_library(&workspace.active_view.library)
                .and_then(|library| library.get_cell(&workspace.active_view.cell))
                .and_then(|cell| cell.get_view(&workspace.active_view.view))
                .is_some()
        );
    }

    #[test]
    fn project_source_names_are_portable_and_extensions_are_case_insensitive() {
        assert!(
            ProjectSourceDocument::try_new(
                "MODEL.VA",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            )
            .is_ok()
        );
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "bad\"name.va",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            ),
            Err(ProjectSourceError::InvalidFileNameCharacters { .. })
        ));
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "COM1.va",
                ProjectSourceLanguage::VerilogA,
                "module model; endmodule",
            ),
            Err(ProjectSourceError::ReservedFileName { .. })
        ));
    }

    #[test]
    fn project_source_payload_limit_is_enforced_before_compilation() {
        let oversized = "x".repeat(MAX_PROJECT_CODE_SOURCE_BYTES + 1);
        assert!(matches!(
            ProjectSourceDocument::try_new(
                "oversized.va",
                ProjectSourceLanguage::VerilogA,
                oversized,
            ),
            Err(ProjectSourceError::SourceTooLarge {
                bytes,
                limit: MAX_PROJECT_CODE_SOURCE_BYTES,
                ..
            }) if bytes == MAX_PROJECT_CODE_SOURCE_BYTES + 1
        ));
    }

    #[test]
    fn source_edits_preserve_exact_utf8_and_invalidate_validation_identity() {
        let mut registry =
            ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
                "sensor_bridge.va",
                ProjectSourceLanguage::VerilogA,
                "module sensor_bridge; endmodule\r\n",
            )
            .unwrap()])
            .unwrap();
        let first_identity = registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();
        assert!(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .unwrap()
                .validation_is_current()
        );

        let source = "module sensor_bridge; // Δ温度\nendmodule\n".to_owned();
        assert!(
            registry
                .replace_content(ProjectSourceLanguage::VerilogA, source.clone())
                .unwrap()
        );
        let edited = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
        assert_eq!(edited.content(), source);
        assert_eq!(edited.revision().get(), 2);
        assert!(edited.validated_identity().is_none());
        assert_ne!(edited.content_digest(), first_identity.content_digest());
        let edited_revision = edited.revision();
        assert!(
            !registry
                .replace_content(ProjectSourceLanguage::VerilogA, source)
                .unwrap()
        );
        assert_eq!(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .unwrap()
                .revision(),
            edited_revision
        );
    }

    #[test]
    fn imported_source_replacement_is_monotonic_validated_and_atomic() {
        let mut registry =
            ProjectSourceRegistry::try_from_documents([ProjectSourceDocument::try_new(
                "first.va",
                ProjectSourceLanguage::VerilogA,
                "module first; endmodule\n",
            )
            .unwrap()])
            .unwrap();
        registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();

        assert!(
            registry
                .replace_imported(
                    ProjectSourceLanguage::VerilogA,
                    "second.va".to_owned(),
                    "module second; endmodule\r\n".to_owned(),
                )
                .unwrap()
        );
        let imported = registry.get(ProjectSourceLanguage::VerilogA).unwrap();
        assert_eq!(imported.file_name(), "second.va");
        assert_eq!(imported.content(), "module second; endmodule\r\n");
        assert_eq!(imported.revision().get(), 2);
        assert!(imported.validated_identity().is_none());

        let before = registry.clone();
        assert!(matches!(
            registry.replace_imported(
                ProjectSourceLanguage::VerilogA,
                "wrong.txt".to_owned(),
                "module wrong; endmodule\n".to_owned(),
            ),
            Err(ProjectSourceError::InvalidFileNameExtension { .. })
        ));
        assert_eq!(registry, before);
    }

    #[test]
    fn workspace_source_dirty_state_tracks_edits_validation_and_cleaning() {
        let mut libraries = LibraryManager::default();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);

        workspace
            .replace_project_source(
                ProjectSourceLanguage::RSpiceAutomation,
                "plan = project.plan(\"Unicode Δ\")".to_owned(),
            )
            .unwrap();
        assert!(workspace.project_sources_dirty);
        assert!(workspace.any_dirty());
        workspace.mark_project_sources_clean();
        assert!(!workspace.any_dirty());

        let identity = workspace
            .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();
        assert!(workspace.project_sources_dirty);
        assert_eq!(
            workspace
                .project_sources
                .get(ProjectSourceLanguage::RSpiceAutomation)
                .unwrap()
                .validated_identity(),
            Some(identity)
        );
        workspace.mark_all_clean();
        assert!(!workspace.any_dirty());

        let repeated = workspace
            .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
            .unwrap();
        assert_eq!(repeated, identity);
        assert!(!workspace.any_dirty());
    }

    #[test]
    fn project_source_validation_rejects_mismatched_slots_and_stale_evidence() {
        let document = ProjectSourceDocument::try_new(
            "sensor_bridge.va",
            ProjectSourceLanguage::VerilogA,
            "module sensor_bridge; endmodule",
        )
        .unwrap();
        let mut registry = ProjectSourceRegistry::try_from_documents([document]).unwrap();
        registry
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .unwrap();
        let mut value = serde_json::to_value(&registry).unwrap();
        value["bundles"][0]["root"]["content"] = serde_json::Value::String("changed".to_owned());
        assert!(serde_json::from_value::<ProjectSourceRegistry>(value).is_err());

        let root = serde_json::to_value(
            registry
                .get(ProjectSourceLanguage::VerilogA)
                .expect("fixture root exists"),
        )
        .unwrap();
        let mut legacy = serde_json::json!({ "verilog_a": root });
        legacy["verilog_a"]["language"] = serde_json::Value::String("rspice-automation".to_owned());
        assert!(serde_json::from_value::<ProjectSourceRegistry>(legacy).is_err());
    }
}
