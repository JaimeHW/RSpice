//! The project's own identity: its descriptor and its technology binding.
//!
//! A binding names the technology a project is authored against and nothing
//! more — physical layer decks, signed organization packages, and remote
//! entitlement receipts need provider records this local binding deliberately
//! does not stand in for. Keeping that boundary explicit is what stops a
//! locally-declared technology from being read as a licensed one.

use super::*;

/// Exact project-owned attachment to a locally parsed and content-pinned model
/// technology. Physical layer decks, signed organization packages, and remote
/// entitlement receipts require provider records that are intentionally not
/// represented by this local binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTechnologyBinding {
    pub(super) schema_version: u16,
    pub(super) package_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) package_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) technology_node: Option<String>,
    pub(super) model_library: String,
    pub(super) root_source: PathBuf,
    pub(super) source_closure: Vec<crate::state::model_library::ModelSourcePin>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) source_edges: Vec<crate::state::model_library::ModelSourceEdge>,
    pub(super) model_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) process_sections: Vec<String>,
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

pub(super) fn validate_retained_model_sources(
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

pub(super) fn nonempty_owned(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.to_owned())
}

pub(super) fn validate_technology_text(
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
    pub(super) id: ProjectId,
    /// Schema of this object, independent of the outer project container.
    pub(super) schema_version: u16,
    /// Monotonic logical revision. Runtime-only path changes do not alter it.
    #[serde(default)]
    pub(super) revision: ObjectRevision,
    #[serde(default = "default_project_name")]
    pub(super) name: String,
    pub path: Option<PathBuf>,
    pub root_library: String,
    pub top_cell: String,
    /// Legacy display-only attachment retained for schema-1 project and
    /// session compatibility. New commits always pair it with the exact
    /// structured binding below.
    pub technology: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) technology_binding: Option<ProjectTechnologyBinding>,
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
