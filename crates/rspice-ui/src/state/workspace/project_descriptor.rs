//! The project's own identity: its descriptor and its technology binding.
//!
//! A binding pins the model-source closure and, when a signed PDK is attached,
//! the exact trusted package revision and content digests. Remote entitlement
//! receipts remain a provider concern and are never inferred from local state.

use super::*;

pub const PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION: u16 = 1;
pub const PROJECT_TECHNOLOGY_CHANGE_RECEIPT_SCHEMA_VERSION: u16 = 1;
pub const MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS: usize = 4_096;
pub const PROJECT_LIBRARY_MUTATION_RECEIPT_SCHEMA_VERSION: u16 = 1;
pub const MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS: usize = 16_384;
pub const PROJECT_CLOUD_PUBLICATION_BINDING_SCHEMA_VERSION: u16 = 1;

/// Persisted, content-addressed project pin to one signed PDK package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSignedTechnologyPin {
    schema_version: u16,
    package_id: String,
    revision: String,
    manifest_digest: crate::product::ContentDigest,
    archive_digest: crate::product::ContentDigest,
    technology_name: String,
    publisher_id: String,
    signing_key_id: String,
    process_node_nm: u32,
    stack_name: String,
    execution_targets: Vec<crate::state::pdk_config::PdkExecutionTarget>,
}

impl ProjectSignedTechnologyPin {
    pub(crate) fn from_validated_package(
        package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage,
    ) -> Result<Self, TechnologyBindingError> {
        let manifest = package.manifest();
        let mut execution_targets = manifest.compatibility.targets.clone();
        execution_targets.sort();
        let pin = Self {
            schema_version: PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
            package_id: manifest.package_id.clone(),
            revision: manifest.revision.clone(),
            manifest_digest: package.manifest_digest(),
            archive_digest: package.archive_digest(),
            technology_name: manifest.technology_name.clone(),
            publisher_id: manifest.publisher_id.clone(),
            signing_key_id: manifest.signing_key_id.clone(),
            process_node_nm: manifest.process_node_nm,
            stack_name: manifest.stack_name.clone(),
            execution_targets,
        };
        pin.validate()?;
        Ok(pin)
    }

    pub fn validate(&self) -> Result<(), TechnologyBindingError> {
        if self.schema_version != PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION {
            return Err(TechnologyBindingError::UnsupportedSignedPackageSchema {
                found: self.schema_version,
                supported: PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
            });
        }
        for (field, value) in [
            ("signed_package.package_id", self.package_id.as_str()),
            ("signed_package.revision", self.revision.as_str()),
            (
                "signed_package.technology_name",
                self.technology_name.as_str(),
            ),
            ("signed_package.publisher_id", self.publisher_id.as_str()),
            (
                "signed_package.signing_key_id",
                self.signing_key_id.as_str(),
            ),
            ("signed_package.stack_name", self.stack_name.as_str()),
        ] {
            validate_technology_text(field, value)?;
        }
        if self.process_node_nm == 0 {
            return Err(TechnologyBindingError::InvalidSignedPackage(
                "process node must be greater than zero".to_owned(),
            ));
        }
        if self.execution_targets.is_empty() {
            return Err(TechnologyBindingError::InvalidSignedPackage(
                "at least one execution target is required".to_owned(),
            ));
        }
        for index in 1..self.execution_targets.len() {
            if self.execution_targets[index - 1] >= self.execution_targets[index] {
                return Err(TechnologyBindingError::InvalidSignedPackage(
                    "execution targets must be strictly sorted and unique".to_owned(),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn validate_registry(
        &self,
        registry: &crate::state::pdk_config::PdkTechnologyRegistry,
    ) -> Result<(), TechnologyBindingError> {
        self.validate()?;
        let package = registry
            .validated_packages()
            .iter()
            .find(|package| {
                package
                    .manifest()
                    .package_id
                    .eq_ignore_ascii_case(&self.package_id)
                    && package.manifest().revision == self.revision
                    && package.manifest_digest() == self.manifest_digest
                    && package.archive_digest() == self.archive_digest
            })
            .ok_or_else(|| TechnologyBindingError::SignedPackageUnavailable {
                package_id: self.package_id.clone(),
                revision: self.revision.clone(),
            })?;
        let observed = Self::from_validated_package(package)?;
        if &observed != self {
            return Err(TechnologyBindingError::SignedPackageMetadataDrift {
                package_id: self.package_id.clone(),
                revision: self.revision.clone(),
            });
        }
        package
            .runtime_compatibility()
            .map_err(TechnologyBindingError::SignedPackageRuntime)
    }

    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> crate::product::ContentDigest {
        self.manifest_digest
    }

    #[must_use]
    pub const fn archive_digest(&self) -> crate::product::ContentDigest {
        self.archive_digest
    }

    #[must_use]
    pub fn technology_name(&self) -> &str {
        &self.technology_name
    }

    #[must_use]
    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    #[must_use]
    pub fn signing_key_id(&self) -> &str {
        &self.signing_key_id
    }

    #[must_use]
    pub const fn process_node_nm(&self) -> u32 {
        self.process_node_nm
    }

    #[must_use]
    pub fn stack_name(&self) -> &str {
        &self.stack_name
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        format!("{} {}", self.package_id, self.revision)
    }
}

/// Exact project-owned attachment to a content-pinned model technology and an
/// optional immutable signed PDK package revision. Runtime use requires the
/// signed pin to resolve against the currently trusted registry.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) signed_package: Option<ProjectSignedTechnologyPin>,
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
            signed_package: None,
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
        if let Some(pin) = &self.signed_package {
            pin.validate()?;
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
        if let Some(pin) = &self.signed_package {
            label.push_str(" · signed ");
            label.push_str(&pin.display_label());
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
        let mut observed = Self::from_model_library(library)?;
        // The live model catalog can prove only its own authenticated source
        // closure. The signed PDK pin is independently resolved against the
        // trusted technology registry, so carry the accepted pin across this
        // comparison instead of treating its deliberate absence from
        // `from_model_library` as catalog drift.
        observed.signed_package = self.signed_package.clone();
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

    #[must_use]
    pub fn signed_package(&self) -> Option<&ProjectSignedTechnologyPin> {
        self.signed_package.as_ref()
    }

    pub(crate) fn with_signed_package(
        mut self,
        package: &crate::state::pdk_config::ValidatedPdkTechnologyPackage,
    ) -> Result<Self, TechnologyBindingError> {
        self.signed_package = Some(ProjectSignedTechnologyPin::from_validated_package(package)?);
        self.validate()?;
        Ok(self)
    }

    pub(crate) fn validate_signed_package(
        &self,
        registry: &crate::state::pdk_config::PdkTechnologyRegistry,
    ) -> Result<(), TechnologyBindingError> {
        self.signed_package
            .as_ref()
            .ok_or(TechnologyBindingError::MissingSignedPackage)?
            .validate_registry(registry)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectTechnologyChangeAction {
    Attach,
    Replace,
}

/// Human and authorization context captured before a project technology
/// transaction starts. This is intentionally separate from the recovery
/// checkpoint evidence, which is produced only after the exact pre-change
/// project snapshot has been durably written and read-back verified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTechnologyChangeAuthority {
    actor_id: String,
    authority_id: String,
    reason: String,
}

impl ProjectTechnologyChangeAuthority {
    pub fn new(
        actor_id: impl Into<String>,
        authority_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, ProjectDescriptorError> {
        let authority = Self {
            actor_id: actor_id.into(),
            authority_id: authority_id.into(),
            reason: reason.into(),
        };
        authority.validate()?;
        Ok(authority)
    }

    fn validate(&self) -> Result<(), ProjectDescriptorError> {
        validate_project_audit_text("technology_change.actor_id", &self.actor_id, 240)?;
        validate_project_audit_text("technology_change.authority_id", &self.authority_id, 240)?;
        validate_project_audit_text("technology_change.reason", &self.reason, 1_024)
    }
}

/// Exact checkpoint proof required to authorize one project technology
/// mutation. Callers cannot claim a checkpoint for a different project
/// revision because the descriptor validates this evidence at commit time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectTechnologyChangeContext {
    authority: ProjectTechnologyChangeAuthority,
    checkpoint_id: Uuid,
    checkpoint_project_revision: ObjectRevision,
    checkpoint_created_unix_ms: u64,
    checkpoint_snapshot_digest: ContentDigest,
    checkpoint_snapshot_byte_len: u64,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
}

impl ProjectTechnologyChangeContext {
    pub fn new(
        authority: ProjectTechnologyChangeAuthority,
        checkpoint_id: Uuid,
        checkpoint_project_revision: ObjectRevision,
        checkpoint_created_unix_ms: u64,
        checkpoint_snapshot_digest: ContentDigest,
        checkpoint_snapshot_byte_len: u64,
    ) -> Result<Self, ProjectDescriptorError> {
        let context = Self {
            authority,
            checkpoint_id,
            checkpoint_project_revision,
            checkpoint_created_unix_ms,
            checkpoint_snapshot_digest,
            checkpoint_snapshot_byte_len,
            migration_evidence: None,
        };
        context.validate()?;
        Ok(context)
    }

    pub fn with_migration_evidence(
        mut self,
        evidence: crate::state::pdk_config::PdkTechnologyMigrationEvidence,
    ) -> Result<Self, ProjectDescriptorError> {
        evidence.validate().map_err(|error| {
            ProjectDescriptorError::InvalidTechnologyMigrationEvidence(error.to_string())
        })?;
        self.migration_evidence = Some(evidence);
        self.validate()?;
        Ok(self)
    }

    fn validate(&self) -> Result<(), ProjectDescriptorError> {
        self.authority.validate()?;
        if self.checkpoint_id.is_nil() {
            return Err(ProjectDescriptorError::InvalidTechnologyCheckpoint(
                "checkpoint identity must not be nil".to_owned(),
            ));
        }
        if self.checkpoint_created_unix_ms == 0 {
            return Err(ProjectDescriptorError::InvalidTechnologyCheckpoint(
                "checkpoint creation time must be greater than zero".to_owned(),
            ));
        }
        if self.checkpoint_snapshot_byte_len == 0 {
            return Err(ProjectDescriptorError::InvalidTechnologyCheckpoint(
                "checkpoint snapshot must not be empty".to_owned(),
            ));
        }
        if self.checkpoint_snapshot_digest == ContentDigest::from_bytes([0; 32]) {
            return Err(ProjectDescriptorError::InvalidTechnologyCheckpoint(
                "checkpoint snapshot digest must not be the zero sentinel".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTechnologyChangeReceipt {
    schema_version: u16,
    sequence: u64,
    project_id: ProjectId,
    action: ProjectTechnologyChangeAction,
    actor_id: String,
    authority_id: String,
    reason: String,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    before_binding_digest: Option<ContentDigest>,
    after_binding_digest: ContentDigest,
    before_binding_label: Option<String>,
    after_binding_label: String,
    checkpoint_id: Uuid,
    checkpoint_project_revision: ObjectRevision,
    checkpoint_created_unix_ms: u64,
    checkpoint_snapshot_digest: ContentDigest,
    checkpoint_snapshot_byte_len: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    previous_receipt_digest: Option<ContentDigest>,
    receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectTechnologyChangeReceiptPayload<'a> {
    schema_version: u16,
    sequence: u64,
    project_id: ProjectId,
    action: ProjectTechnologyChangeAction,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    before_binding_digest: Option<ContentDigest>,
    after_binding_digest: ContentDigest,
    before_binding_label: &'a Option<String>,
    after_binding_label: &'a str,
    checkpoint_id: Uuid,
    checkpoint_project_revision: ObjectRevision,
    checkpoint_created_unix_ms: u64,
    checkpoint_snapshot_digest: ContentDigest,
    checkpoint_snapshot_byte_len: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    migration_evidence: &'a Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    previous_receipt_digest: Option<ContentDigest>,
}

impl ProjectTechnologyChangeReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, ProjectDescriptorError> {
        let payload = ProjectTechnologyChangeReceiptPayload {
            schema_version: self.schema_version,
            sequence: self.sequence,
            project_id: self.project_id,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            from_project_revision: self.from_project_revision,
            to_project_revision: self.to_project_revision,
            before_binding_digest: self.before_binding_digest,
            after_binding_digest: self.after_binding_digest,
            before_binding_label: &self.before_binding_label,
            after_binding_label: &self.after_binding_label,
            checkpoint_id: self.checkpoint_id,
            checkpoint_project_revision: self.checkpoint_project_revision,
            checkpoint_created_unix_ms: self.checkpoint_created_unix_ms,
            checkpoint_snapshot_digest: self.checkpoint_snapshot_digest,
            checkpoint_snapshot_byte_len: self.checkpoint_snapshot_byte_len,
            migration_evidence: &self.migration_evidence,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| ProjectDescriptorError::Serialization(error.to_string()))?;
        Ok(ContentDigest::from_bytes(
            sha2::Sha256::digest(bytes).into(),
        ))
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn action(&self) -> ProjectTechnologyChangeAction {
        self.action
    }

    #[must_use]
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }

    #[must_use]
    pub fn authority_id(&self) -> &str {
        &self.authority_id
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    #[must_use]
    pub const fn source_project_revision(&self) -> ObjectRevision {
        self.from_project_revision
    }

    #[must_use]
    pub const fn target_project_revision(&self) -> ObjectRevision {
        self.to_project_revision
    }

    #[must_use]
    pub fn after_binding_label(&self) -> &str {
        &self.after_binding_label
    }

    #[must_use]
    pub const fn checkpoint_id(&self) -> Uuid {
        self.checkpoint_id
    }

    #[must_use]
    pub const fn checkpoint_snapshot_digest(&self) -> ContentDigest {
        self.checkpoint_snapshot_digest
    }

    #[must_use]
    pub const fn migration_evidence(
        &self,
    ) -> Option<&crate::state::pdk_config::PdkTechnologyMigrationEvidence> {
        self.migration_evidence.as_ref()
    }

    #[must_use]
    pub const fn receipt_digest(&self) -> ContentDigest {
        self.receipt_digest
    }
}

/// Exact semantic operation retained by the project library audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProjectLibraryMutation {
    CreateCell {
        library: String,
        cell: String,
    },
    CreateView {
        library: String,
        cell: String,
        view: String,
    },
    CopyCell {
        source_library: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    RenameCell {
        library: String,
        from_cell: String,
        to_cell: String,
    },
    DeleteCell {
        library: String,
        cell: String,
    },
    DeleteView {
        library: String,
        cell: String,
        view: String,
    },
    RollbackPublication {
        publication_id: Uuid,
        publication_label: String,
        snapshot_digest: ContentDigest,
        actor_id: String,
        authority_id: String,
        reason: String,
    },
}

impl ProjectLibraryMutation {
    #[must_use]
    pub const fn operation_label(&self) -> &'static str {
        match self {
            Self::CreateCell { .. } => "cell creation",
            Self::CreateView { .. } => "view creation",
            Self::CopyCell { .. } => "cell copy",
            Self::RenameCell { .. } => "cell rename",
            Self::DeleteCell { .. } => "cell deletion",
            Self::DeleteView { .. } => "view deletion",
            Self::RollbackPublication { .. } => "library publication rollback",
        }
    }

    fn validate(&self) -> Result<(), ProjectDescriptorError> {
        let fields: &[(&str, &str)] = match self {
            Self::CreateCell { library, cell } | Self::DeleteCell { library, cell } => {
                &[("library", library), ("cell", cell)]
            }
            Self::CreateView {
                library,
                cell,
                view,
            }
            | Self::DeleteView {
                library,
                cell,
                view,
            } => &[("library", library), ("cell", cell), ("view", view)],
            Self::CopyCell {
                source_library,
                source_cell,
                target_library,
                target_cell,
            } => &[
                ("source_library", source_library),
                ("source_cell", source_cell),
                ("target_library", target_library),
                ("target_cell", target_cell),
            ],
            Self::RenameCell {
                library,
                from_cell,
                to_cell,
            } => &[
                ("library", library),
                ("from_cell", from_cell),
                ("to_cell", to_cell),
            ],
            Self::RollbackPublication {
                publication_label,
                actor_id,
                authority_id,
                reason,
                ..
            } => &[
                ("publication_label", publication_label),
                ("actor_id", actor_id),
                ("authority_id", authority_id),
                ("reason", reason),
            ],
        };
        for (field, value) in fields {
            validate_library_audit_text(field, value)?;
        }

        match self {
            Self::CopyCell {
                source_library,
                source_cell,
                target_library,
                target_cell,
            } if crate::state::canonical_cell_view_owner_key(source_library, source_cell, "")
                == crate::state::canonical_cell_view_owner_key(target_library, target_cell, "") =>
            {
                Err(ProjectDescriptorError::LibraryAuditCorrupted(
                    "cell copy source and target identities are equal".to_owned(),
                ))
            }
            Self::RenameCell {
                library,
                from_cell,
                to_cell,
            } if crate::state::canonical_cell_view_owner_key(library, from_cell, "")
                == crate::state::canonical_cell_view_owner_key(library, to_cell, "") =>
            {
                Err(ProjectDescriptorError::LibraryAuditCorrupted(
                    "cell rename source and target identities are equal".to_owned(),
                ))
            }
            Self::RollbackPublication { publication_id, .. } if publication_id.is_nil() => {
                Err(ProjectDescriptorError::LibraryAuditCorrupted(
                    "library publication rollback identity must not be nil".to_owned(),
                ))
            }
            _ => Ok(()),
        }
    }
}

/// Immutable hash-chained record of one project library membership change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLibraryMutationReceipt {
    schema_version: u16,
    sequence: u64,
    project_id: ProjectId,
    mutation: ProjectLibraryMutation,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    from_library_revision: u64,
    to_library_revision: u64,
    previous_receipt_digest: Option<ContentDigest>,
    receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectLibraryMutationReceiptPayload<'a> {
    schema_version: u16,
    sequence: u64,
    project_id: ProjectId,
    mutation: &'a ProjectLibraryMutation,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    from_library_revision: u64,
    to_library_revision: u64,
    previous_receipt_digest: Option<ContentDigest>,
}

impl ProjectLibraryMutationReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, ProjectDescriptorError> {
        let bytes = serde_json::to_vec(&ProjectLibraryMutationReceiptPayload {
            schema_version: self.schema_version,
            sequence: self.sequence,
            project_id: self.project_id,
            mutation: &self.mutation,
            from_project_revision: self.from_project_revision,
            to_project_revision: self.to_project_revision,
            from_library_revision: self.from_library_revision,
            to_library_revision: self.to_library_revision,
            previous_receipt_digest: self.previous_receipt_digest,
        })
        .map_err(|error| ProjectDescriptorError::Serialization(error.to_string()))?;
        Ok(ContentDigest::from_bytes(
            sha2::Sha256::digest(bytes).into(),
        ))
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn mutation(&self) -> &ProjectLibraryMutation {
        &self.mutation
    }

    #[must_use]
    pub const fn source_project_revision(&self) -> ObjectRevision {
        self.from_project_revision
    }

    #[must_use]
    pub const fn target_project_revision(&self) -> ObjectRevision {
        self.to_project_revision
    }

    #[must_use]
    pub const fn source_library_revision(&self) -> u64 {
        self.from_library_revision
    }

    #[must_use]
    pub const fn target_library_revision(&self) -> u64 {
        self.to_library_revision
    }

    #[must_use]
    pub const fn previous_receipt_digest(&self) -> Option<ContentDigest> {
        self.previous_receipt_digest
    }

    #[must_use]
    pub const fn receipt_digest(&self) -> ContentDigest {
        self.receipt_digest
    }
}

/// Fully preflighted descriptor-side portion of a library transaction.
#[derive(Debug)]
pub(crate) struct PreparedProjectLibraryMutation {
    receipt: ProjectLibraryMutationReceipt,
}

impl PreparedProjectLibraryMutation {
    #[must_use]
    pub(crate) const fn minimum_library_revision(&self) -> u64 {
        self.receipt.to_library_revision
    }
}

pub(super) fn validate_library_audit_text(
    field: &str,
    value: &str,
) -> Result<(), ProjectDescriptorError> {
    if value.is_empty() {
        return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
            "{field} is required"
        )));
    }
    if value != value.trim() {
        return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
            "{field} must not begin or end with whitespace"
        )));
    }
    if value.chars().count() > 240 {
        return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
            "{field} exceeds 240 Unicode scalar values"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
            "{field} contains a control character"
        )));
    }
    Ok(())
}

fn technology_binding_digest(
    binding: Option<&ProjectTechnologyBinding>,
) -> Result<Option<ContentDigest>, ProjectDescriptorError> {
    binding
        .map(|binding| {
            serde_json::to_vec(binding)
                .map(|bytes| ContentDigest::from_bytes(sha2::Sha256::digest(bytes).into()))
                .map_err(|error| ProjectDescriptorError::Serialization(error.to_string()))
        })
        .transpose()
}

fn migration_endpoint_matches_project_pin(
    endpoint: &crate::state::pdk_config::PdkTechnologyBinding,
    archive_digest: ContentDigest,
    pin: &ProjectSignedTechnologyPin,
) -> bool {
    endpoint.package_id.eq_ignore_ascii_case(pin.package_id())
        && endpoint.revision == pin.revision()
        && endpoint.manifest_digest == pin.manifest_digest()
        && archive_digest == pin.archive_digest()
}

fn validate_project_audit_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ProjectDescriptorError> {
    if value.is_empty() {
        return Err(ProjectDescriptorError::RequiredField(field));
    }
    if value != value.trim() {
        return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
            "{field} must not begin or end with whitespace"
        )));
    }
    if value.chars().count() > maximum {
        return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
            "{field} exceeds {maximum} Unicode scalar values"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
            "{field} contains a control character"
        )));
    }
    Ok(())
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
    #[error("signed technology pin schema {found} is unsupported; this build supports {supported}")]
    UnsupportedSignedPackageSchema { found: u16, supported: u16 },
    #[error("signed technology package pin is invalid: {0}")]
    InvalidSignedPackage(String),
    #[error("project technology binding has no signed PDK package pin")]
    MissingSignedPackage,
    #[error("signed PDK package '{package_id}' revision '{revision}' is not currently trusted")]
    SignedPackageUnavailable {
        package_id: String,
        revision: String,
    },
    #[error("signed PDK package '{package_id}' revision '{revision}' metadata has drifted")]
    SignedPackageMetadataDrift {
        package_id: String,
        revision: String,
    },
    #[error("signed PDK package is incompatible with this runtime: {0}")]
    SignedPackageRuntime(String),
}

/// Exact attachment of this project to its RSpice Cloud publication lineage:
/// the circuit whose `/c/` page this project publishes to. Recorded on the
/// first successful publish so every later publish seals a new version of the
/// same page instead of minting a parallel one. Identifiers are the service's
/// own; the binding asserts lineage, never entitlement or reachability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectCloudPublicationBinding {
    pub(super) schema_version: u16,
    pub(super) workspace_id: String,
    pub(super) circuit_id: String,
}

impl ProjectCloudPublicationBinding {
    pub fn new(workspace_id: String, circuit_id: String) -> Result<Self, ProjectDescriptorError> {
        let binding = Self {
            schema_version: PROJECT_CLOUD_PUBLICATION_BINDING_SCHEMA_VERSION,
            workspace_id,
            circuit_id,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn validate(&self) -> Result<(), ProjectDescriptorError> {
        if self.schema_version != PROJECT_CLOUD_PUBLICATION_BINDING_SCHEMA_VERSION {
            return Err(ProjectDescriptorError::UnsupportedCloudPublicationSchema {
                found: self.schema_version,
                supported: PROJECT_CLOUD_PUBLICATION_BINDING_SCHEMA_VERSION,
            });
        }
        for (field, value) in [
            ("workspace_id", &self.workspace_id),
            ("circuit_id", &self.circuit_id),
        ] {
            if value.is_empty()
                || value.len() > 128
                || !value.chars().all(|ch| ch.is_ascii_graphic())
            {
                return Err(ProjectDescriptorError::InvalidCloudPublicationBinding(
                    field,
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    #[must_use]
    pub fn circuit_id(&self) -> &str {
        &self.circuit_id
    }
}

/// Metadata for the active RSpice project.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectDescriptor {
    /// Stable product identity. Legacy project files receive an ID exactly
    /// once during deserialization and retain it on every later save.
    pub(super) id: ProjectId,
    /// Schema of this object, independent of the outer project container.
    pub(super) schema_version: u16,
    /// True only while a genuinely unversioned descriptor is being restored.
    /// Session containers replace the descriptor-local migration identity with
    /// one derived from the complete recoverable session before exposing it.
    #[serde(skip)]
    pub(super) migrated_legacy_identity: bool,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) technology_change_audit: Vec<ProjectTechnologyChangeReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) library_mutation_audit: Vec<ProjectLibraryMutationReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) library_publications: Vec<ProjectLibraryPublicationReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cloud_publication: Option<ProjectCloudPublicationBinding>,
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
            #[serde(default)]
            technology_change_audit: Vec<ProjectTechnologyChangeReceipt>,
            #[serde(default)]
            library_mutation_audit: Vec<ProjectLibraryMutationReceipt>,
            #[serde(default)]
            library_publications: Vec<ProjectLibraryPublicationReceipt>,
            #[serde(default)]
            cloud_publication: Option<ProjectCloudPublicationBinding>,
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
        let (id, migrated_legacy_identity) = match descriptor.id {
            DeserializedProjectId::Null => {
                return Err(D::Error::custom(
                    "project identity must not be explicitly null",
                ));
            }
            DeserializedProjectId::Value(id) => (id, false),
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
                    &descriptor.technology_change_audit,
                    &descriptor.description,
                ))
                .map_err(D::Error::custom)?;
                (
                    ProjectId::from_namespace(LEGACY_PROJECT_DESCRIPTOR_ID_NAMESPACE, &material),
                    true,
                )
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
            migrated_legacy_identity,
            revision: descriptor.revision,
            name: descriptor.name,
            path: descriptor.path,
            root_library: descriptor.root_library,
            top_cell: descriptor.top_cell,
            technology: descriptor.technology,
            technology_binding: descriptor.technology_binding,
            technology_change_audit: descriptor.technology_change_audit,
            library_mutation_audit: descriptor.library_mutation_audit,
            library_publications: descriptor.library_publications,
            cloud_publication: descriptor.cloud_publication,
            description: descriptor.description,
        })
    }
}

impl Default for ProjectDescriptor {
    fn default() -> Self {
        Self {
            id: ProjectId::new(),
            schema_version: PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            migrated_legacy_identity: false,
            revision: ObjectRevision::INITIAL,
            name: default_project_name(),
            path: None,
            root_library: DEFAULT_PROJECT_LIBRARY.to_string(),
            top_cell: DEFAULT_TOP_CELL.to_string(),
            technology: None,
            technology_binding: None,
            technology_change_audit: Vec::new(),
            library_mutation_audit: Vec::new(),
            library_publications: Vec::new(),
            cloud_publication: None,
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
    pub(crate) const fn has_descriptor_local_legacy_identity(&self) -> bool {
        self.migrated_legacy_identity
    }

    /// Replace a temporary descriptor-local migration identity with the
    /// identity derived by its owning legacy session artifact.
    pub(crate) fn bind_legacy_session_identity(&mut self, id: ProjectId) {
        self.id = id;
        self.schema_version = PROJECT_DESCRIPTOR_SCHEMA_VERSION;
        self.migrated_legacy_identity = false;
    }

    /// Folder that project-relative file references resolve against.
    ///
    /// This is the directory holding the `.rspiceproj`, so a design that
    /// references a data file beside it keeps working when the whole folder is
    /// moved or handed to someone else. An unsaved project has no such folder
    /// and therefore no relative references to resolve.
    #[must_use]
    pub fn data_root(&self) -> Option<&std::path::Path> {
        self.path.as_deref().and_then(std::path::Path::parent)
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
        self.validate_technology_change_audit()?;
        self.validate_library_mutation_audit()?;
        self.validate_library_publications()?;
        if let Some(binding) = &self.cloud_publication {
            binding.validate()?;
        }
        Ok(())
    }

    #[must_use]
    pub fn technology_binding(&self) -> Option<&ProjectTechnologyBinding> {
        self.technology_binding.as_ref()
    }

    #[must_use]
    pub fn cloud_publication(&self) -> Option<&ProjectCloudPublicationBinding> {
        self.cloud_publication.as_ref()
    }

    /// Record the cloud circuit this project publishes to as one atomic
    /// project metadata revision. Re-recording the identical binding is a
    /// no-op; a different circuit replaces the lineage (the service keeps the
    /// old page's history — this binding only selects future publish targets).
    pub(crate) fn bind_cloud_publication(
        &mut self,
        binding: ProjectCloudPublicationBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        binding.validate()?;
        if self.cloud_publication.as_ref() == Some(&binding) {
            return Ok(self.revision);
        }
        let next_revision = self.revision.next()?;
        self.cloud_publication = Some(binding);
        self.revision = next_revision;
        Ok(next_revision)
    }

    #[must_use]
    pub fn technology_change_audit(&self) -> &[ProjectTechnologyChangeReceipt] {
        &self.technology_change_audit
    }

    #[must_use]
    pub fn library_mutation_audit(&self) -> &[ProjectLibraryMutationReceipt] {
        &self.library_mutation_audit
    }

    /// Preflight an exact library membership mutation, including the complete
    /// receipt and both next revisions, before any live catalog state changes.
    pub(crate) fn prepare_library_mutation(
        &self,
        mutation: ProjectLibraryMutation,
        library_revision: u64,
    ) -> Result<PreparedProjectLibraryMutation, ProjectDescriptorError> {
        self.validate_library_mutation_audit()?;
        mutation.validate()?;
        if self.library_mutation_audit.len() >= MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS {
            return Err(ProjectDescriptorError::LibraryAuditLimit {
                maximum: MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS,
            });
        }

        let to_project_revision = self.revision.next()?;
        let to_library_revision = library_revision
            .checked_add(1)
            .ok_or(ProjectDescriptorError::LibraryRevisionExhausted)?;
        let sequence = u64::try_from(self.library_mutation_audit.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(ProjectDescriptorError::LibraryAuditSequenceExhausted)?;
        let mut receipt = ProjectLibraryMutationReceipt {
            schema_version: PROJECT_LIBRARY_MUTATION_RECEIPT_SCHEMA_VERSION,
            sequence,
            project_id: self.id,
            mutation,
            from_project_revision: self.revision,
            to_project_revision,
            from_library_revision: library_revision,
            to_library_revision,
            previous_receipt_digest: self
                .library_mutation_audit
                .last()
                .map(ProjectLibraryMutationReceipt::receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;

        let mut candidate = self.clone();
        candidate.revision = to_project_revision;
        candidate.library_mutation_audit.push(receipt.clone());
        candidate.validate()?;
        Ok(PreparedProjectLibraryMutation { receipt })
    }

    /// Publish a receipt that was fully constructed and validated before the
    /// catalog mutation. The caller owns the matching catalog revision check.
    pub(crate) fn publish_library_mutation(
        &mut self,
        prepared: PreparedProjectLibraryMutation,
        observed_library_revision: u64,
    ) -> Result<ProjectLibraryMutationReceipt, ProjectDescriptorError> {
        let mut receipt = prepared.receipt;
        assert_eq!(
            self.id, receipt.project_id,
            "library mutation receipt belongs to a different project"
        );
        assert_eq!(
            self.revision, receipt.from_project_revision,
            "project changed after the library mutation was preflighted"
        );
        assert_eq!(
            self.library_mutation_audit
                .last()
                .map(ProjectLibraryMutationReceipt::receipt_digest),
            receipt.previous_receipt_digest,
            "library mutation audit changed after preflight"
        );
        if observed_library_revision < receipt.to_library_revision {
            return Err(ProjectDescriptorError::LibraryAuditCorrupted(
                "library revision did not advance during the prepared mutation".to_owned(),
            ));
        }
        receipt.to_library_revision = observed_library_revision;
        receipt.receipt_digest = receipt.calculate_digest()?;

        let mut candidate = self.clone();
        candidate.revision = receipt.to_project_revision;
        candidate.library_mutation_audit.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    fn validate_library_mutation_audit(&self) -> Result<(), ProjectDescriptorError> {
        if self.library_mutation_audit.len() > MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS {
            return Err(ProjectDescriptorError::LibraryAuditLimit {
                maximum: MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS,
            });
        }

        let mut previous_receipt_digest = None;
        let mut previous_project_revision = None;
        let mut previous_library_revision = None;
        for (index, receipt) in self.library_mutation_audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(ProjectDescriptorError::LibraryAuditSequenceExhausted)?;
            if receipt.schema_version != PROJECT_LIBRARY_MUTATION_RECEIPT_SCHEMA_VERSION {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] schema {} is unsupported",
                    receipt.schema_version
                )));
            }
            if receipt.sequence != expected_sequence {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] sequence is {}, expected {expected_sequence}",
                    receipt.sequence
                )));
            }
            if receipt.project_id != self.id {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] belongs to a different project"
                )));
            }
            if receipt.previous_receipt_digest != previous_receipt_digest
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] has invalid content or chain linkage"
                )));
            }
            receipt.mutation.validate()?;
            if receipt.from_project_revision.next()? != receipt.to_project_revision {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] does not advance exactly one project revision"
                )));
            }
            if receipt.to_project_revision > self.revision {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] claims a future project revision"
                )));
            }
            if receipt.to_library_revision <= receipt.from_library_revision {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] does not advance the library revision"
                )));
            }
            if previous_project_revision
                .is_some_and(|previous| receipt.from_project_revision < previous)
            {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] project revision regresses"
                )));
            }
            if previous_library_revision
                .is_some_and(|previous| receipt.from_library_revision < previous)
            {
                return Err(ProjectDescriptorError::LibraryAuditCorrupted(format!(
                    "receipt[{index}] library revision regresses"
                )));
            }
            previous_receipt_digest = Some(receipt.receipt_digest);
            previous_project_revision = Some(receipt.to_project_revision);
            previous_library_revision = Some(receipt.to_library_revision);
        }
        Ok(())
    }

    /// Attach an exact, validated technology contract as one atomic project
    /// metadata revision. Reattaching the identical binding is a no-op.
    pub(crate) fn attach_technology(
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
        if !self.technology_change_audit.is_empty() {
            return Err(ProjectDescriptorError::TechnologyAuditRequired);
        }
        let next_revision = self.revision.next()?;
        self.technology = Some(label);
        self.technology_binding = Some(binding);
        self.revision = next_revision;
        Ok(next_revision)
    }

    /// Commit an exact technology change and its checkpoint-backed authority
    /// receipt as one atomic project descriptor revision.
    pub fn attach_technology_audited(
        &mut self,
        binding: ProjectTechnologyBinding,
        context: ProjectTechnologyChangeContext,
    ) -> Result<(ObjectRevision, ProjectTechnologyChangeReceipt), ProjectDescriptorError> {
        binding.validate()?;
        context.validate()?;
        self.validate_technology_change_audit()?;
        let label = binding.display_label();
        if self.technology_binding.as_ref() == Some(&binding)
            && self.technology.as_deref() == Some(label.as_str())
        {
            return Err(ProjectDescriptorError::NoOpTechnologyChange);
        }
        if context.checkpoint_project_revision != self.revision {
            return Err(ProjectDescriptorError::TechnologyCheckpointRevision {
                checkpoint: context.checkpoint_project_revision.get(),
                current: self.revision.get(),
            });
        }
        if self.technology_change_audit.len() >= MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS {
            return Err(ProjectDescriptorError::TechnologyAuditLimit {
                maximum: MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS,
            });
        }

        let before_pin = self
            .technology_binding
            .as_ref()
            .and_then(ProjectTechnologyBinding::signed_package);
        let after_pin = binding.signed_package();
        let signed_revision_changed = before_pin != after_pin;
        match (
            before_pin,
            after_pin,
            signed_revision_changed,
            context.migration_evidence.as_ref(),
        ) {
            (Some(before), Some(after), true, Some(evidence)) => {
                evidence.validate().map_err(|error| {
                    ProjectDescriptorError::InvalidTechnologyMigrationEvidence(error.to_string())
                })?;
                if !migration_endpoint_matches_project_pin(
                    evidence.baseline(),
                    evidence.baseline_archive_digest(),
                    before,
                ) || !migration_endpoint_matches_project_pin(
                    evidence.candidate(),
                    evidence.candidate_archive_digest(),
                    after,
                ) {
                    return Err(ProjectDescriptorError::InvalidTechnologyMigrationEvidence(
                        "reviewed diff endpoints do not match the exact project transition"
                            .to_owned(),
                    ));
                }
            }
            (Some(_), Some(_), true, None) => {
                return Err(ProjectDescriptorError::MissingTechnologyMigrationEvidence);
            }
            (_, _, false, Some(_)) | (None, _, _, Some(_)) => {
                return Err(ProjectDescriptorError::InvalidTechnologyMigrationEvidence(
                    "migration evidence was supplied for a transition without a prior differing signed revision"
                        .to_owned(),
                ));
            }
            _ => {}
        }

        let next_revision = self.revision.next()?;
        let before_binding_digest = technology_binding_digest(self.technology_binding.as_ref())?;
        let after_binding_digest = technology_binding_digest(Some(&binding))?.ok_or_else(|| {
            ProjectDescriptorError::Serialization(
                "technology binding digest was not produced".to_owned(),
            )
        })?;
        let action = if before_binding_digest.is_some() {
            ProjectTechnologyChangeAction::Replace
        } else {
            ProjectTechnologyChangeAction::Attach
        };
        let mut receipt = ProjectTechnologyChangeReceipt {
            schema_version: PROJECT_TECHNOLOGY_CHANGE_RECEIPT_SCHEMA_VERSION,
            sequence: u64::try_from(self.technology_change_audit.len())
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(ProjectDescriptorError::TechnologyAuditSequenceExhausted)?,
            project_id: self.id,
            action,
            actor_id: context.authority.actor_id,
            authority_id: context.authority.authority_id,
            reason: context.authority.reason,
            from_project_revision: self.revision,
            to_project_revision: next_revision,
            before_binding_digest,
            after_binding_digest,
            before_binding_label: self.technology.clone(),
            after_binding_label: label.clone(),
            checkpoint_id: context.checkpoint_id,
            checkpoint_project_revision: context.checkpoint_project_revision,
            checkpoint_created_unix_ms: context.checkpoint_created_unix_ms,
            checkpoint_snapshot_digest: context.checkpoint_snapshot_digest,
            checkpoint_snapshot_byte_len: context.checkpoint_snapshot_byte_len,
            migration_evidence: context.migration_evidence,
            previous_receipt_digest: self
                .technology_change_audit
                .last()
                .map(ProjectTechnologyChangeReceipt::receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;

        let mut candidate = self.clone();
        candidate.technology = Some(label);
        candidate.technology_binding = Some(binding);
        candidate.revision = next_revision;
        candidate.technology_change_audit.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok((next_revision, receipt))
    }

    fn validate_technology_change_audit(&self) -> Result<(), ProjectDescriptorError> {
        if self.technology_change_audit.len() > MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS {
            return Err(ProjectDescriptorError::TechnologyAuditLimit {
                maximum: MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS,
            });
        }
        if self.technology_change_audit.is_empty() {
            return Ok(());
        }

        let mut previous_receipt_digest = None;
        let mut previous_to_revision = None;
        let mut previous_after_binding_digest = None;
        let mut previous_after_binding_label: Option<&str> = None;
        for (index, receipt) in self.technology_change_audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(ProjectDescriptorError::TechnologyAuditSequenceExhausted)?;
            if receipt.schema_version != PROJECT_TECHNOLOGY_CHANGE_RECEIPT_SCHEMA_VERSION {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] schema {} is unsupported",
                    receipt.schema_version
                )));
            }
            if receipt.sequence != expected_sequence {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] sequence is {}, expected {expected_sequence}",
                    receipt.sequence
                )));
            }
            if receipt.project_id != self.id {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] belongs to a different project"
                )));
            }
            if receipt.previous_receipt_digest != previous_receipt_digest
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] has invalid content or chain linkage"
                )));
            }
            validate_project_audit_text("technology_change.actor_id", &receipt.actor_id, 240)?;
            validate_project_audit_text(
                "technology_change.authority_id",
                &receipt.authority_id,
                240,
            )?;
            validate_project_audit_text("technology_change.reason", &receipt.reason, 1_024)?;
            validate_project_audit_text(
                "technology_change.after_binding_label",
                &receipt.after_binding_label,
                1_024,
            )?;
            if receipt
                .before_binding_label
                .as_deref()
                .is_some_and(|label| {
                    validate_project_audit_text(
                        "technology_change.before_binding_label",
                        label,
                        1_024,
                    )
                    .is_err()
                })
            {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] has an invalid preceding binding label"
                )));
            }
            if receipt.checkpoint_id.is_nil()
                || receipt.checkpoint_created_unix_ms == 0
                || receipt.checkpoint_snapshot_byte_len == 0
                || receipt.checkpoint_snapshot_digest == ContentDigest::from_bytes([0; 32])
            {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] has invalid checkpoint evidence"
                )));
            }
            if receipt.checkpoint_project_revision != receipt.from_project_revision {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] checkpoint does not capture its pre-change project revision"
                )));
            }
            if let Some(evidence) = &receipt.migration_evidence {
                evidence.validate().map_err(|error| {
                    ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                        "receipt[{index}] has invalid migration evidence: {error}"
                    ))
                })?;
                if receipt.action != ProjectTechnologyChangeAction::Replace {
                    return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                        "receipt[{index}] attaches an initial technology but claims migration evidence"
                    )));
                }
            }
            if receipt.from_project_revision.next()? != receipt.to_project_revision {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] does not advance exactly one project revision"
                )));
            }
            if receipt.to_project_revision > self.revision {
                return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                    "receipt[{index}] claims a future project revision"
                )));
            }
            if let Some(previous) = previous_to_revision {
                if receipt.from_project_revision < previous {
                    return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                        "receipt[{index}] project revision regresses"
                    )));
                }
                if receipt.before_binding_digest != previous_after_binding_digest
                    || receipt.before_binding_label.as_deref() != previous_after_binding_label
                {
                    return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                        "receipt[{index}] does not continue the preceding technology state"
                    )));
                }
            }
            match receipt.action {
                ProjectTechnologyChangeAction::Attach => {
                    if receipt.before_binding_digest.is_some()
                        || receipt.before_binding_label.is_some()
                    {
                        return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                            "receipt[{index}] attach transition has a preceding binding"
                        )));
                    }
                }
                ProjectTechnologyChangeAction::Replace => {
                    if receipt.before_binding_digest.is_none()
                        || receipt.before_binding_label.is_none()
                        || receipt.before_binding_digest == Some(receipt.after_binding_digest)
                    {
                        return Err(ProjectDescriptorError::TechnologyAuditCorrupted(format!(
                            "receipt[{index}] replacement transition is invalid or a no-op"
                        )));
                    }
                }
            }
            previous_receipt_digest = Some(receipt.receipt_digest);
            previous_to_revision = Some(receipt.to_project_revision);
            previous_after_binding_digest = Some(receipt.after_binding_digest);
            previous_after_binding_label = Some(receipt.after_binding_label.as_str());
        }

        let final_receipt = self.technology_change_audit.last().ok_or_else(|| {
            ProjectDescriptorError::TechnologyAuditCorrupted(
                "technology audit unexpectedly has no final receipt".to_owned(),
            )
        })?;
        let current_digest = technology_binding_digest(self.technology_binding.as_ref())?;
        if current_digest != Some(final_receipt.after_binding_digest)
            || self.technology.as_deref() != Some(final_receipt.after_binding_label.as_str())
        {
            return Err(ProjectDescriptorError::TechnologyAuditCorrupted(
                "current project technology does not match the final receipt".to_owned(),
            ));
        }
        Ok(())
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
        copy.technology_change_audit.clear();
        copy.library_mutation_audit.clear();
        copy.library_publications.clear();
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
    #[error(
        "cloud publication binding schema {found} is unsupported; this build supports {supported}"
    )]
    UnsupportedCloudPublicationSchema { found: u16, supported: u16 },
    #[error("cloud publication binding field {0} is not a valid service identifier")]
    InvalidCloudPublicationBinding(&'static str),
    #[error("technology changes require an audited checkpoint-backed transaction")]
    TechnologyAuditRequired,
    #[error(
        "technology change conflicts with physical layout {owner}; migrate or remove that layout before changing its exact signed PDK pin"
    )]
    TechnologyConflictsWithPhysicalLayout { owner: String },
    #[error("technology change is identical to the current project binding")]
    NoOpTechnologyChange,
    #[error(
        "technology checkpoint captured project revision {checkpoint}, but the current revision is {current}"
    )]
    TechnologyCheckpointRevision { checkpoint: u64, current: u64 },
    #[error("technology checkpoint evidence is invalid: {0}")]
    InvalidTechnologyCheckpoint(String),
    #[error("replacing a signed PDK revision requires exact reviewed migration evidence")]
    MissingTechnologyMigrationEvidence,
    #[error("technology migration evidence is invalid: {0}")]
    InvalidTechnologyMigrationEvidence(String),
    #[error("technology change audit is limited to {maximum} receipts")]
    TechnologyAuditLimit { maximum: usize },
    #[error("technology change audit sequence is exhausted")]
    TechnologyAuditSequenceExhausted,
    #[error("technology change audit is corrupted: {0}")]
    TechnologyAuditCorrupted(String),
    #[error("project library mutation audit is limited to {maximum} receipts")]
    LibraryAuditLimit { maximum: usize },
    #[error("project library mutation audit sequence is exhausted")]
    LibraryAuditSequenceExhausted,
    #[error("project library content revision is exhausted")]
    LibraryRevisionExhausted,
    #[error("project library mutation audit is corrupted: {0}")]
    LibraryAuditCorrupted(String),
    #[error("project library publication audit is limited to {maximum} receipts")]
    LibraryPublicationLimit { maximum: usize },
    #[error("project library publication sequence is exhausted")]
    LibraryPublicationSequenceExhausted,
    #[error("project library publication audit is corrupted: {0}")]
    LibraryPublicationCorrupted(String),
    #[error("project descriptor serialization failed: {0}")]
    Serialization(String),
    #[error(transparent)]
    Technology(#[from] TechnologyBindingError),
    #[error(transparent)]
    Revision(#[from] RevisionError),
}
