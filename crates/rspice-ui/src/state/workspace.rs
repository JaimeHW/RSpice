//! Project workspace state.
//!
//! This module is the product-level design spine for RSpice Studio. It keeps
//! project identity, open Library/Cell/View documents, active hierarchy
//! breadcrumbs, and per-view schematic buffers together instead of letting the
//! workbench, library browser, and single schematic buffer drift apart.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use sha2::Digest as _;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::product::{
    AnalysisInstanceId, DesignVariableId, ObjectRevision, ProjectId, RevisionError, RunId,
    SavedOutputId, SimulationPlanId,
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
    pub diagnostic: Option<String>,
}

/// Immutable resolution receipt for the project configuration surface and
/// preflight diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyResolution {
    pub bindings: Vec<ResolvedHierarchyBinding>,
    pub total_instances: usize,
    pub resolved_instances: usize,
}

impl HierarchyResolution {
    pub fn unresolved_instances(&self) -> usize {
        self.total_instances.saturating_sub(self.resolved_instances)
    }

    pub fn is_valid(&self) -> bool {
        self.unresolved_instances() == 0
    }
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

fn validate_raw_probe(expression: &str) -> Result<(), String> {
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
    #[error("project-owned netlist document is invalid: {message}")]
    InvalidNetlistDocumentProjection { message: String },
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

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
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
    pub netlist_document: Option<crate::workbench::code_workspace::NetlistDocument>,
    /// Ownership-dialog selection for the project-owned source artifact.
    #[serde(default)]
    pub netlist_descriptor: Option<OwnedNetlistDescriptor>,
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
    /// Runtime dirty state for project-owned metadata such as an exact
    /// technology attachment. The binding itself persists in `project`.
    #[serde(default, skip)]
    #[doc(hidden)]
    pub project_metadata_dirty: bool,
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let active_view = CellViewRef::default_top();
        let mut schematic_buffers = HashMap::new();
        schematic_buffers.insert(active_view.key(), SchematicState::default());

        Self {
            project: ProjectDescriptor::default(),
            active_view: active_view.clone(),
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            hierarchy_stack: vec![active_view],
            hierarchy_instances: Vec::new(),
            schematic_buffers,
            specs: Vec::new(),
            simulation_plan_payloads: Vec::new(),
            netlist_source: None,
            netlist_document: None,
            netlist_descriptor: None,
            netlist_source_path: None,
            netlist_source_dirty: false,
            project_metadata_dirty: false,
        }
    }
}

impl ProjectWorkspace {
    /// Validate the persisted simulation configuration without requiring any
    /// runtime editor state. Cross-document targets are validated by project
    /// I/O once the library tree and simulation plan are available.
    pub fn validate_simulation_configuration(&self) -> Result<(), SimulationConfigurationError> {
        if let Some(document) = &self.netlist_document {
            if document.ownership()
                == crate::workbench::code_workspace::DocumentOwnership::Generated
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
}

#[derive(Clone, Copy)]
struct HierarchyMaster<'a> {
    schematic: Option<&'a SchematicState>,
    view_type: Option<ViewType>,
    view_modified: bool,
    library_read_only: bool,
    library_has_technology: bool,
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
        }
    }

    fn resolve(mut self) -> HierarchyResolution {
        let root = CellViewRef::new(
            &self.workspace.project.root_library,
            &self.workspace.project.top_cell,
            DEFAULT_SCHEMATIC_VIEW,
        );
        let mut ancestors = Vec::new();
        self.resolve_reference(root, None, 0, true, &mut ancestors);
        HierarchyResolution {
            bindings: self.rows,
            total_instances: self.total_instances,
            resolved_instances: self.resolved_instances,
        }
    }

    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<String>,
    ) {
        if self.total_instances >= MAX_HIERARCHY_RESOLUTION_INSTANCES {
            let mut row = self.binding_row(
                requested,
                binding,
                depth,
                is_root,
                HierarchyBindingStatus::InstanceLimit,
                Some(format!(
                    "hierarchy exceeds the supported limit of {MAX_HIERARCHY_RESOLUTION_INSTANCES} expanded instances"
                )),
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
                depth,
                is_root,
                HierarchyBindingStatus::DepthLimit,
                Some(format!(
                    "hierarchy exceeds the supported depth of {MAX_HIERARCHY_RESOLUTION_DEPTH}"
                )),
            );
            self.upsert(row);
            return;
        }

        let search_order = hierarchy_view_search_order(&requested.view, is_root);
        let source_binding_error = binding.and_then(|binding| {
            binding
                .source_path
                .as_ref()
                .and_then(|_| self.validate_source_binding(binding).err())
        });
        let master = if source_binding_error.is_none() {
            self.resolve_master(&requested, binding, &search_order)
        } else {
            None
        };
        let resolved_reference = master
            .as_ref()
            .and_then(|(_, reference)| reference.clone())
            .unwrap_or_else(|| requested.clone());
        let identity = hierarchy_identity(&resolved_reference);

        if ancestors.iter().any(|ancestor| ancestor == &identity) {
            let chain = ancestors
                .iter()
                .chain(std::iter::once(&identity))
                .cloned()
                .collect::<Vec<_>>()
                .join(" → ");
            let row = self.binding_row_with_master(
                resolved_reference,
                binding,
                depth,
                is_root,
                master.map(|(master, _)| master),
                HierarchyBindingStatus::Recursive,
                Some(format!("recursive hierarchy: {chain}")),
            );
            self.upsert(row);
            return;
        }

        let (master, status, diagnostic) = match master {
            Some((master, _)) => {
                let modified = master.view_modified
                    || master.schematic.is_some_and(|schematic| schematic.is_dirty);
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
                source_binding_error.or_else(|| {
                    Some(format!(
                        "no executable master resolved for {} using {}",
                        hierarchy_display_path(&requested),
                        search_order.join(" → ")
                    ))
                }),
            ),
        };

        let row = self.binding_row_with_master(
            resolved_reference,
            binding,
            depth,
            is_root,
            master,
            status,
            diagnostic,
        );
        self.upsert(row);
        if status.is_resolved() {
            self.resolved_instances += 1;
        }

        let Some(schematic) = master.and_then(|master| master.schematic) else {
            return;
        };
        let children = schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| component.library_cell.clone())
            .collect::<Vec<_>>();
        if children.is_empty() {
            return;
        }

        ancestors.push(identity);
        for child in &children {
            let requested_view = if child.view.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW
            } else {
                child.view.as_str()
            };
            self.resolve_reference(
                CellViewRef::new(&child.library, &child.cell, requested_view),
                Some(child),
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
        depth: usize,
        is_root: bool,
        status: HierarchyBindingStatus,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        self.binding_row_with_master(reference, binding, depth, is_root, None, status, diagnostic)
    }

    #[allow(clippy::too_many_arguments)]
    fn binding_row_with_master(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        depth: usize,
        is_root: bool,
        master: Option<HierarchyMaster<'_>>,
        status: HierarchyBindingStatus,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        let search_order = hierarchy_view_search_order(&reference.view, is_root);
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();
        let terminal_view = master
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
        let stop_view = if is_root {
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
            model_section: hierarchy_model_section(self.libraries, &reference, binding),
            reference,
            purpose: purpose.to_owned(),
            view_search_order: search_order,
            stop_view,
            status,
            instance_count: 1,
            diagnostic,
        }
    }

    fn resolve_master(
        &self,
        requested: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        search_order: &[String],
    ) -> Option<(HierarchyMaster<'a>, Option<CellViewRef>)> {
        let library = find_library(self.libraries, &requested.library);
        let cell = library.and_then(|library| find_cell(library, &requested.cell));
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();

        if source_bound {
            let view = cell.and_then(|cell| find_view(cell, &requested.view))?;
            return Some((
                HierarchyMaster {
                    schematic: None,
                    view_type: Some(view.view_type),
                    view_modified: view.modified,
                    library_read_only: library.is_some_and(|library| library.read_only),
                    library_has_technology: library
                        .is_some_and(|library| !library.technology.trim().is_empty()),
                },
                Some(requested.clone()),
            ));
        }

        for candidate in search_order {
            let reference = CellViewRef::new(&requested.library, &requested.cell, candidate);
            // A buffer without an authoritative library/cell/view identity is
            // an orphan, not an executable master. Corrupt or partially
            // restored workspaces must fail closed.
            let Some(view) = cell.and_then(|cell| find_view(cell, candidate)) else {
                continue;
            };
            let view_type = view.view_type;
            if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
                if let Some(schematic) = self.find_schematic(&reference) {
                    return Some((
                        HierarchyMaster {
                            schematic: Some(schematic),
                            view_type: Some(view_type),
                            view_modified: view.modified,
                            library_read_only: library.is_some_and(|library| library.read_only),
                            library_has_technology: library
                                .is_some_and(|library| !library.technology.trim().is_empty()),
                        },
                        Some(reference),
                    ));
                }
                continue;
            }
            // External executable views are owned by the placed binding's
            // source path. A library view alone is not enough: accepting it
            // here would claim a closure the netlist generator cannot emit.
        }
        None
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
        let key = row.reference.key().to_ascii_lowercase();
        if let Some(index) = self.row_indices.get(&key).copied() {
            let existing = &mut self.rows[index];
            existing.instance_count = existing.instance_count.saturating_add(row.instance_count);
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

#[cfg(not(target_arch = "wasm32"))]
fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = std::fs::read_to_string(source_path).map_err(|error| {
        format!(
            "source-backed binding {}/{}/{} cannot read {}: {error}",
            binding.library,
            binding.cell,
            binding.view,
            source_path.display()
        )
    })?;
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
            tokens
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && tokens
                    .next()
                    .is_some_and(|token| token.eq_ignore_ascii_case(master))
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
    format!(
        "{}/{}",
        reference.library.to_ascii_lowercase(),
        reference.cell.to_ascii_lowercase()
    )
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
    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut workspace = Self::default();
        workspace.ensure_library_model(libraries);
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
        self.project_metadata_dirty = false;
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self
                .schematic_buffers
                .values()
                .any(|schematic| schematic.is_dirty)
            || self.netlist_source_dirty
            || self.project_metadata_dirty
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
}
