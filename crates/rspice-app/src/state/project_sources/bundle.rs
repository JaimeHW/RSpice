//! What one project-owned source closure can be asked to do.
//!
//! A bundle is a graph, not a file list: files, their roles, and the
//! dependency edges between them move together or not at all. Every mutation
//! here re-derives the closure digest and invalidates validation, so a bundle
//! can never advertise a receipt for bytes it no longer holds.

use super::*;

impl ProjectSourceBundle {
    pub fn try_new(
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let roles = default_source_roles(language, &root_path)?;
        Self::try_new_with_id_and_roles(
            ProjectSourceId::new(),
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    /// Construct a source bundle with explicit persisted semantic roles.
    /// Callers creating Automation bundles should use this constructor so
    /// run-plan, environment, and permission documents never depend on path
    /// or content heuristics.
    pub fn try_new_with_roles(
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
        roles: impl IntoIterator<Item = ProjectSourceRoleBinding>,
    ) -> Result<Self, ProjectSourceError> {
        Self::try_new_with_id_and_roles(
            ProjectSourceId::new(),
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    pub fn try_new_with_id(
        id: ProjectSourceId,
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let roles = default_source_roles(language, &root_path)?;
        Self::try_new_with_id_and_roles(
            id,
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    pub fn try_new_with_id_and_roles(
        id: ProjectSourceId,
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
        roles: impl IntoIterator<Item = ProjectSourceRoleBinding>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let mut bundle = Self {
            id,
            owner,
            language,
            root: ProjectSourceDocument::try_new_with_id(
                ProjectSourceDocumentId::migrated(id, &root_path),
                root_path,
                language,
                root_content,
            )?,
            files: files.into_iter().collect(),
            dependencies: dependencies.into_iter().collect(),
            roles: roles.into_iter().collect(),
            revision: ObjectRevision::INITIAL,
            validated_identity: None,
            history: Vec::new(),
            qualifications: Vec::new(),
        };
        bundle.canonicalize();
        bundle.validate()?;
        Ok(bundle)
    }

    pub(super) fn migrated(
        mut document: ProjectSourceDocument,
    ) -> Result<Self, ProjectSourceError> {
        let id = ProjectSourceId::migrated(&document);
        if document.id.is_missing() {
            document.id = ProjectSourceDocumentId::migrated(id, document.logical_path());
        }
        document.validate()?;
        let revision = document.revision;
        let was_validated = document.validation_is_current();
        let mut bundle = Self {
            id,
            owner: ProjectSourceOwner::code_workspace(document.language),
            language: document.language,
            roles: default_source_roles(document.language, document.logical_path())?,
            root: document,
            files: Vec::new(),
            dependencies: Vec::new(),
            revision,
            validated_identity: None,
            history: Vec::new(),
            qualifications: Vec::new(),
        };
        if was_validated {
            bundle.validated_identity = Some(ProjectSourceValidationIdentity {
                revision,
                content_digest: bundle.closure_digest(),
            });
        }
        bundle.validate()?;
        Ok(bundle)
    }

    #[must_use]
    pub const fn id(&self) -> ProjectSourceId {
        self.id
    }

    #[must_use]
    pub fn owner(&self) -> &ProjectSourceOwner {
        &self.owner
    }

    #[must_use]
    pub const fn language(&self) -> ProjectSourceLanguage {
        self.language
    }

    #[must_use]
    pub fn root(&self) -> &ProjectSourceDocument {
        &self.root
    }

    #[must_use]
    pub fn files(&self) -> &[ProjectSourceFile] {
        &self.files
    }

    #[must_use]
    pub fn dependencies(&self) -> &[ProjectSourceDependency] {
        &self.dependencies
    }

    #[must_use]
    pub fn roles(&self) -> &[ProjectSourceRoleBinding] {
        &self.roles
    }

    #[must_use]
    pub fn qualifications(&self) -> &[ProjectSourceQualificationRecord] {
        &self.qualifications
    }

    /// Append immutable qualification evidence for the exact current source
    /// identity. Evidence never rewrites or deletes prior attempts.
    pub fn append_qualification(
        &mut self,
        mut record: ProjectSourceQualificationRecord,
    ) -> Result<u64, ProjectSourceError> {
        if self.qualifications.len() >= MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        if record.source_revision != self.revision.get()
            || record.source_closure_digest != self.closure_digest()
        {
            return Err(ProjectSourceError::QualificationIdentityMismatch);
        }
        let sequence = self
            .qualifications
            .last()
            .map_or(1, |previous| previous.sequence.saturating_add(1));
        if sequence == 0 {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        record.sequence = sequence;
        record.validate()?;
        self.qualifications.push(record);
        Ok(sequence)
    }

    pub fn paths_for_role(&self, role: ProjectSourceRole) -> impl Iterator<Item = &str> {
        self.roles
            .iter()
            .filter(move |binding| binding.role == role)
            .map(ProjectSourceRoleBinding::logical_path)
    }

    #[must_use]
    pub fn role_for_path(&self, logical_path: &str) -> Option<ProjectSourceRole> {
        let key = path_key(logical_path);
        self.roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
            .map(ProjectSourceRoleBinding::role)
    }

    /// Bind one retained document to a language-appropriate semantic role atomically.
    /// A document has at most one role, and singleton roles cannot be rebound
    /// without first unbinding their current document.
    pub fn bind_role(
        &mut self,
        logical_path: &str,
        role: ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        if !role.is_allowed_for(self.language) {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role,
                language: self.language,
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if self.roles.iter().any(|binding| {
            binding.role == role && path_key(binding.logical_path()) == path_key(&canonical_path)
        }) {
            return Ok(false);
        }
        if let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == path_key(&canonical_path))
        {
            return Err(ProjectSourceError::FileAlreadyHasRole {
                logical_path: canonical_path,
                role: binding.role,
            });
        }
        if !role.allows_multiple()
            && let Some(binding) = self.roles.iter().find(|binding| binding.role == role)
        {
            return Err(ProjectSourceError::DuplicateSingletonRole {
                role,
                first: binding.logical_path.clone(),
                second: canonical_path,
            });
        }
        if role == ProjectSourceRole::AutomationEntry
            && path_key(&canonical_path) != path_key(self.root.logical_path())
        {
            return Err(ProjectSourceError::AutomationEntryMustBeRoot {
                logical_path: canonical_path,
                root_path: self.root.logical_path().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate
            .roles
            .push(ProjectSourceRoleBinding::try_new(canonical_path, role)?);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Remove a non-entry Automation role atomically. The entry role is part
    /// of bundle identity and moves with root renames; it cannot be removed.
    pub fn unbind_role(&mut self, logical_path: &str) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
        else {
            return Ok(false);
        };
        if binding.role == ProjectSourceRole::AutomationEntry {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: binding.logical_path.clone(),
            });
        }
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate
            .roles
            .retain(|binding| path_key(binding.logical_path()) != key);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Replace the optional non-entry Automation role for one retained source
    /// as a single source-graph revision. Singleton roles move atomically from
    /// their prior document, so the project is never left in a transient state
    /// with two permission manifests or no environment lock merely because a
    /// UI operation was interrupted between unbind and bind calls.
    pub fn set_non_entry_role(
        &mut self,
        logical_path: &str,
        role: Option<ProjectSourceRole>,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        if self.language != ProjectSourceLanguage::RSpiceAutomation {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role: role.unwrap_or(ProjectSourceRole::AutomationEntry),
                language: self.language,
            });
        }
        if role == Some(ProjectSourceRole::AutomationEntry) {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: logical_path.to_owned(),
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if canonical_path.eq_ignore_ascii_case(self.root.logical_path()) {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: canonical_path,
            });
        }
        let current = self.role_for_path(&canonical_path);
        if current == role {
            return Ok(false);
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let requested_key = path_key(&canonical_path);
        candidate.roles.retain(|binding| {
            let same_document = path_key(binding.logical_path()) == requested_key;
            let displaced_singleton = role
                .is_some_and(|requested| !requested.allows_multiple() && binding.role == requested);
            binding.role == ProjectSourceRole::AutomationEntry
                || (!same_document && !displaced_singleton)
        });
        if let Some(role) = role {
            candidate
                .roles
                .push(ProjectSourceRoleBinding::try_new(&canonical_path, role)?);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Return the exact retained UTF-8 source for one portable logical path.
    /// Lookups are case-insensitive because bundle validation rejects paths
    /// that would collide on a supported desktop filesystem.
    #[must_use]
    pub fn file_content(&self, logical_path: &str) -> Option<&str> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.content());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::content)
    }

    #[must_use]
    pub fn document_id(&self, logical_path: &str) -> Option<ProjectSourceDocumentId> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.id());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::id)
    }

    #[must_use]
    pub fn document_revision(&self, logical_path: &str) -> Option<ObjectRevision> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.revision());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::revision)
    }

    #[must_use]
    pub fn contains_file(&self, logical_path: &str) -> bool {
        self.file_content(logical_path).is_some()
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn revision_history(&self) -> &[ProjectSourceRevisionSnapshot] {
        &self.history
    }

    #[must_use]
    pub fn revision_snapshot(
        &self,
        revision: ObjectRevision,
    ) -> Option<&ProjectSourceRevisionSnapshot> {
        self.history
            .iter()
            .find(|snapshot| snapshot.revision == revision)
    }

    /// Restore a retained source-graph revision as a new monotonic revision.
    /// The expected-current check prevents a comparison dialog from applying
    /// to a newer bundle, and current ownership is intentionally preserved.
    pub fn restore_revision(
        &mut self,
        expected_current: ObjectRevision,
        retained_revision: ObjectRevision,
    ) -> Result<bool, ProjectSourceError> {
        if self.revision != expected_current {
            return Err(ProjectSourceError::StaleRevisionRestore {
                expected: expected_current.get(),
                found: self.revision.get(),
            });
        }
        let snapshot = self.revision_snapshot(retained_revision).cloned().ok_or(
            ProjectSourceError::MissingRetainedRevision {
                revision: retained_revision.get(),
            },
        )?;
        if self.root == snapshot.root
            && self.files == snapshot.files
            && self.dependencies == snapshot.dependencies
            && self.roles == snapshot.roles
        {
            return Ok(false);
        }
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate.root = snapshot.root;
        candidate.files = snapshot.files;
        candidate.dependencies = snapshot.dependencies;
        candidate.roles = snapshot.roles;
        candidate.advance_revision()?;
        candidate.root.revision = candidate.revision;
        for file in &mut candidate.files {
            file.revision = candidate.revision;
        }
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    #[must_use]
    pub const fn validated_identity(&self) -> Option<ProjectSourceValidationIdentity> {
        self.validated_identity
    }

    #[must_use]
    pub fn validation_is_current(&self) -> bool {
        self.validated_identity.is_some_and(|identity| {
            identity.revision == self.revision && identity.content_digest == self.closure_digest()
        })
    }

    /// SHA-256 over an explicitly versioned sequence of length-framed owner,
    /// language, path, edge, and exact UTF-8 content fields. Framing prevents
    /// concatenation ambiguity; canonical ordering makes the result independent
    /// of insertion order.
    #[must_use]
    pub fn closure_digest(&self) -> ContentDigest {
        let mut hasher = Sha256::new();
        hash_frame(&mut hasher, CLOSURE_DIGEST_DOMAIN);
        match &self.owner {
            ProjectSourceOwner::CodeWorkspace { language } => {
                hash_frame(&mut hasher, b"code-workspace");
                hash_frame(&mut hasher, language.stable_name().as_bytes());
            }
            ProjectSourceOwner::CellView { reference } => {
                hash_frame(&mut hasher, b"cell-view");
                hash_frame(&mut hasher, reference.library.as_bytes());
                hash_frame(&mut hasher, reference.cell.as_bytes());
                hash_frame(&mut hasher, reference.view.as_bytes());
            }
        }
        hash_frame(&mut hasher, self.language.stable_name().as_bytes());
        hash_frame(&mut hasher, self.root.id.as_uuid().as_bytes());
        hash_frame(&mut hasher, self.root.file_name.as_bytes());
        hash_frame(&mut hasher, self.root.content.as_bytes());
        hash_u64(&mut hasher, self.files.len() as u64);
        for file in &self.files {
            hash_frame(&mut hasher, file.id.as_uuid().as_bytes());
            hash_frame(&mut hasher, file.logical_path.as_bytes());
            hash_frame(&mut hasher, file.content.as_bytes());
        }
        hash_u64(&mut hasher, self.dependencies.len() as u64);
        for dependency in &self.dependencies {
            hash_frame(&mut hasher, dependency.importer.as_bytes());
            hash_frame(&mut hasher, dependency.imported.as_bytes());
        }
        hash_u64(&mut hasher, self.roles.len() as u64);
        for binding in &self.roles {
            hash_frame(
                &mut hasher,
                source_role_stable_name(binding.role).as_bytes(),
            );
            hash_frame(&mut hasher, binding.logical_path.as_bytes());
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    /// Replace exact bytes for one retained logical path atomically.
    pub fn replace_file_content(
        &mut self,
        logical_path: &str,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let changed = if path_key(&candidate.root.file_name) == key {
            candidate.root.replace_content(content)?
        } else if let Some(file) = candidate
            .files
            .iter_mut()
            .find(|file| path_key(&file.logical_path) == key)
        {
            if file.content == content {
                false
            } else {
                validate_source_content(&file.logical_path, &content)?;
                file.revision = next_revision(file.revision, &file.logical_path)?;
                file.content = content;
                true
            }
        } else {
            return Err(ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            });
        };
        if !changed {
            return Ok(false);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Replace any number of documents as one journaled source-graph
    /// revision. Per-document revisions advance for changed documents, while
    /// the owning bundle advances exactly once and retains exactly one
    /// predecessor snapshot.
    pub fn replace_files_transactionally(
        &mut self,
        replacements: impl IntoIterator<Item = (String, String)>,
    ) -> Result<usize, ProjectSourceError> {
        let replacements = replacements.into_iter().collect::<Vec<_>>();
        let mut paths = BTreeSet::new();
        for (logical_path, _) in &replacements {
            validate_logical_path(logical_path)?;
            if !paths.insert(path_key(logical_path)) {
                return Err(ProjectSourceError::DuplicateLogicalPath {
                    logical_path: logical_path.clone(),
                });
            }
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let mut changed = 0_usize;
        for (logical_path, content) in replacements {
            let key = path_key(&logical_path);
            let file_changed = if path_key(candidate.root.logical_path()) == key {
                candidate.root.replace_content(content)?
            } else if let Some(file) = candidate
                .files
                .iter_mut()
                .find(|file| path_key(file.logical_path()) == key)
            {
                if file.content == content {
                    false
                } else {
                    validate_source_content(file.logical_path(), &content)?;
                    file.revision = next_revision(file.revision, file.logical_path())?;
                    file.content = content;
                    true
                }
            } else {
                return Err(ProjectSourceError::MissingFile { logical_path });
            };
            changed += usize::from(file_changed);
        }
        if changed == 0 {
            return Ok(0);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
    }

    /// Add one dependency document to the sealed closure and attach it to an
    /// existing importer. The corresponding include is inserted at the start
    /// of the importer, so the authenticated graph and executable source can
    /// never diverge as a side effect of the authoring command.
    pub fn add_file(
        &mut self,
        importer: &str,
        file: ProjectSourceFile,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(importer)?;
        file.validate()?;
        if self.contains_file(file.logical_path()) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: file.logical_path().to_owned(),
            });
        }
        if !self.contains_file(importer) {
            return Err(ProjectSourceError::MissingFile {
                logical_path: importer.to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let imported = file.logical_path().to_owned();
        candidate.files.push(file);
        candidate
            .dependencies
            .push(ProjectSourceDependency::try_new(
                importer.to_owned(),
                imported.clone(),
            )?);
        if candidate.language == ProjectSourceLanguage::VerilogA {
            candidate.prepend_include(importer, &imported)?;
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Add one project document, dependency edge, and semantic role as one
    /// source-graph revision. Unlike ordinary Verilog-A dependencies, a build
    /// profile is configuration and is therefore never injected as an
    /// `` `include `` directive into source text.
    pub fn add_file_with_role(
        &mut self,
        importer: &str,
        file: ProjectSourceFile,
        role: ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(importer)?;
        file.validate()?;
        if !role.is_allowed_for(self.language) {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role,
                language: self.language,
            });
        }
        if self.contains_file(file.logical_path()) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: file.logical_path().to_owned(),
            });
        }
        if !self.contains_file(importer) {
            return Err(ProjectSourceError::MissingFile {
                logical_path: importer.to_owned(),
            });
        }
        if !role.allows_multiple()
            && let Some(binding) = self.roles.iter().find(|binding| binding.role == role)
        {
            return Err(ProjectSourceError::DuplicateSingletonRole {
                role,
                first: binding.logical_path.clone(),
                second: file.logical_path().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let imported = file.logical_path().to_owned();
        candidate.files.push(file);
        candidate
            .dependencies
            .push(ProjectSourceDependency::try_new(importer, &imported)?);
        candidate
            .roles
            .push(ProjectSourceRoleBinding::try_new(&imported, role)?);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Rename one source document atomically. Every authenticated dependency
    /// endpoint and every include that realizes an incoming dependency is
    /// rewritten in the same transaction. The root remains the explicit root;
    /// only its portable logical path changes.
    pub fn rename_file(
        &mut self,
        current_path: &str,
        new_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(current_path)?;
        validate_logical_path(new_path)?;
        let current_key = path_key(current_path);
        let new_key = path_key(new_path);
        let current = self
            .logical_path_with_original_case(current_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: current_path.to_owned(),
            })?;
        if current == new_path {
            return Ok(false);
        }
        if current_key != new_key && self.contains_file(new_path) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: new_path.to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let canonical_current = candidate
            .logical_path_with_original_case(current_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: current_path.to_owned(),
            })?
            .to_owned();
        if candidate.language == ProjectSourceLanguage::VerilogA {
            let incoming = candidate
                .dependencies
                .iter()
                .filter(|edge| path_key(edge.imported()) == current_key)
                .map(|edge| edge.importer().to_owned())
                .collect::<Vec<_>>();
            for importer in incoming {
                candidate.rewrite_include(&importer, &canonical_current, Some(new_path))?;
            }
        }

        if path_key(candidate.root.logical_path()) == current_key {
            candidate
                .root
                .replace_imported(new_path.to_owned(), candidate.root.content.clone())?;
        } else {
            let file = candidate
                .files
                .iter_mut()
                .find(|file| path_key(file.logical_path()) == current_key)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: current_path.to_owned(),
                })?;
            let mut renamed =
                ProjectSourceFile::try_new_with_id(file.id, new_path, file.content.clone())?;
            renamed.revision = next_revision(file.revision, file.logical_path())?;
            *file = renamed;
        }
        for edge in &mut candidate.dependencies {
            if path_key(edge.importer()) == current_key {
                edge.importer = new_path.to_owned();
            }
            if path_key(edge.imported()) == current_key {
                edge.imported = new_path.to_owned();
            }
        }
        for binding in &mut candidate.roles {
            if path_key(binding.logical_path()) == current_key {
                binding.logical_path = new_path.to_owned();
            }
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Delete a non-root leaf document. Includes in every importer are removed
    /// atomically. A document with its own dependencies must be emptied from
    /// the leaves upward, preventing an accidental cascade that hides source
    /// loss from the user.
    pub fn remove_file(&mut self, logical_path: &str) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Err(ProjectSourceError::CannotRemoveBundleRoot {
                logical_path: self.root.logical_path().to_owned(),
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
        {
            return Err(ProjectSourceError::RoleBoundFile {
                logical_path: canonical_path,
                role: binding.role,
            });
        }
        if let Some(edge) = self
            .dependencies
            .iter()
            .find(|edge| path_key(edge.importer()) == key)
        {
            return Err(ProjectSourceError::FileHasDependencies {
                logical_path: canonical_path,
                dependency: edge.imported().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        if candidate.language == ProjectSourceLanguage::VerilogA {
            let importers = candidate
                .dependencies
                .iter()
                .filter(|edge| path_key(edge.imported()) == key)
                .map(|edge| edge.importer().to_owned())
                .collect::<Vec<_>>();
            for importer in importers {
                candidate.rewrite_include(&importer, &canonical_path, None)?;
            }
        }
        candidate
            .files
            .retain(|file| path_key(file.logical_path()) != key);
        candidate
            .dependencies
            .retain(|edge| path_key(edge.importer()) != key && path_key(edge.imported()) != key);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    fn logical_path_with_original_case(&self, logical_path: &str) -> Option<&str> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.logical_path());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::logical_path)
    }

    /// Return the retained spelling of a case-insensitively matched logical
    /// path. Callers use this to keep command/search identity canonical.
    pub fn retained_logical_path(&self, logical_path: &str) -> Option<&str> {
        self.logical_path_with_original_case(logical_path)
    }

    fn prepend_include(
        &mut self,
        importer: &str,
        imported: &str,
    ) -> Result<(), ProjectSourceError> {
        let content =
            self.file_content(importer)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: importer.to_owned(),
                })?;
        let updated = format!("`include \"{imported}\"\n{content}");
        self.replace_file_content_without_bundle_revision(importer, updated)
    }

    fn rewrite_include(
        &mut self,
        importer: &str,
        imported: &str,
        replacement: Option<&str>,
    ) -> Result<(), ProjectSourceError> {
        let source =
            self.file_content(importer)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: importer.to_owned(),
                })?;
        let updated = rewrite_matching_include_lines(source, importer, imported, replacement);
        if updated != source {
            self.replace_file_content_without_bundle_revision(importer, updated)?;
        }
        Ok(())
    }

    fn replace_file_content_without_bundle_revision(
        &mut self,
        logical_path: &str,
        content: String,
    ) -> Result<(), ProjectSourceError> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            self.root.replace_content(content)?;
            return Ok(());
        }
        let file = self
            .files
            .iter_mut()
            .find(|file| path_key(file.logical_path()) == key)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?;
        validate_source_content(file.logical_path(), &content)?;
        file.revision = next_revision(file.revision, file.logical_path())?;
        file.content = content;
        Ok(())
    }

    pub(super) fn replace_root_imported(
        &mut self,
        root_path: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let retained_profile_path = (self.language == ProjectSourceLanguage::VerilogA)
            .then(|| {
                self.paths_for_role(ProjectSourceRole::VerilogABuildProfile)
                    .next()
                    .map(str::to_owned)
            })
            .flatten();
        let discarded_closure = self.files.iter().any(|file| {
            retained_profile_path
                .as_deref()
                .is_none_or(|path| !file.logical_path().eq_ignore_ascii_case(path))
        }) || self.dependencies.iter().any(|edge| {
            retained_profile_path.as_deref().is_none_or(|path| {
                !edge.imported().eq_ignore_ascii_case(path)
                    || !edge
                        .importer()
                        .eq_ignore_ascii_case(self.root.logical_path())
            })
        });
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let root_changed = candidate.root.replace_imported(root_path, content)?;
        if !root_changed && !discarded_closure {
            return Ok(false);
        }
        if let Some(profile_path) = retained_profile_path {
            candidate
                .files
                .retain(|file| file.logical_path().eq_ignore_ascii_case(&profile_path));
            candidate.dependencies.clear();
            candidate
                .dependencies
                .push(ProjectSourceDependency::try_new(
                    candidate.root.logical_path(),
                    &profile_path,
                )?);
            candidate.roles.retain(|binding| {
                binding.role == ProjectSourceRole::VerilogABuildProfile
                    && binding.logical_path().eq_ignore_ascii_case(&profile_path)
            });
        } else {
            candidate.files.clear();
            candidate.dependencies.clear();
            candidate.roles =
                default_source_roles(candidate.language, candidate.root.logical_path())?;
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    pub fn mark_validated(
        &mut self,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        self.validate()?;
        self.root.mark_validated();
        let identity = ProjectSourceValidationIdentity {
            revision: self.revision,
            content_digest: self.closure_digest(),
        };
        self.validated_identity = Some(identity);
        Ok(identity)
    }

    pub fn validate(&self) -> Result<(), ProjectSourceError> {
        self.owner.validate(self.language)?;
        if self.root.language != self.language {
            return Err(ProjectSourceError::BundleLanguageMismatch {
                bundle: self.language,
                root: self.root.language,
            });
        }
        self.root.validate()?;
        if self.files.len().saturating_add(1) > MAX_PROJECT_SOURCE_FILES {
            return Err(ProjectSourceError::TooManyFiles {
                files: self.files.len().saturating_add(1),
                limit: MAX_PROJECT_SOURCE_FILES,
            });
        }
        if self.dependencies.len() > MAX_PROJECT_SOURCE_DEPENDENCIES {
            return Err(ProjectSourceError::TooManyDependencies {
                dependencies: self.dependencies.len(),
                limit: MAX_PROJECT_SOURCE_DEPENDENCIES,
            });
        }

        let root_key = path_key(&self.root.file_name);
        let mut paths = BTreeMap::new();
        paths.insert(root_key.clone(), self.root.file_name.as_str());
        let mut document_ids = BTreeSet::from([self.root.id]);
        let mut total_bytes = self.root.content.len();
        let mut previous_file_key: Option<String> = None;
        for file in &self.files {
            file.validate()?;
            if !document_ids.insert(file.id) {
                return Err(ProjectSourceError::DuplicateDocumentIdentity {
                    id: file.id,
                    logical_path: file.logical_path.clone(),
                });
            }
            total_bytes = total_bytes.checked_add(file.content.len()).ok_or(
                ProjectSourceError::BundleTooLarge {
                    bytes: usize::MAX,
                    limit: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
                },
            )?;
            let key = path_key(&file.logical_path);
            if paths.insert(key.clone(), &file.logical_path).is_some() {
                return Err(ProjectSourceError::DuplicateLogicalPath {
                    logical_path: file.logical_path.clone(),
                });
            }
            if previous_file_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedFiles);
            }
            previous_file_key = Some(key);
        }
        if total_bytes > MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(ProjectSourceError::BundleTooLarge {
                bytes: total_bytes,
                limit: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
            });
        }

        let mut graph: BTreeMap<String, Vec<String>> = paths
            .keys()
            .map(|path| (path.clone(), Vec::new()))
            .collect();
        let mut edge_keys = BTreeSet::new();
        let mut previous_edge_key: Option<(String, String)> = None;
        for dependency in &self.dependencies {
            validate_logical_path(&dependency.importer)?;
            validate_logical_path(&dependency.imported)?;
            let key = dependency.canonical_key();
            if !paths.contains_key(&key.0) {
                return Err(ProjectSourceError::MissingDependencyEndpoint {
                    logical_path: dependency.importer.clone(),
                });
            }
            if !paths.contains_key(&key.1) {
                return Err(ProjectSourceError::MissingDependencyEndpoint {
                    logical_path: dependency.imported.clone(),
                });
            }
            if !edge_keys.insert(key.clone()) {
                return Err(ProjectSourceError::DuplicateDependency {
                    importer: dependency.importer.clone(),
                    imported: dependency.imported.clone(),
                });
            }
            if previous_edge_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedDependencies);
            }
            previous_edge_key = Some(key.clone());
            graph.entry(key.0).or_default().push(key.1);
        }

        let mut previous_role_key: Option<(ProjectSourceRole, String)> = None;
        let mut role_paths = BTreeSet::new();
        let mut singleton_roles = BTreeMap::new();
        for binding in &self.roles {
            if !binding.role.is_allowed_for(self.language) {
                return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                    role: binding.role,
                    language: self.language,
                });
            }
            validate_logical_path(binding.logical_path())?;
            let path = path_key(binding.logical_path());
            if !paths.contains_key(&path) {
                return Err(ProjectSourceError::MissingRoleEndpoint {
                    logical_path: binding.logical_path.clone(),
                    role: binding.role,
                });
            }
            if !role_paths.insert(path) {
                return Err(ProjectSourceError::FileHasMultipleRoles {
                    logical_path: binding.logical_path.clone(),
                });
            }
            if !binding.role.allows_multiple()
                && let Some(first) = singleton_roles.insert(binding.role, binding.logical_path())
            {
                return Err(ProjectSourceError::DuplicateSingletonRole {
                    role: binding.role,
                    first: first.to_owned(),
                    second: binding.logical_path.clone(),
                });
            }
            let key = binding.canonical_key();
            if previous_role_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedRoles);
            }
            previous_role_key = Some(key);
        }
        if self.language == ProjectSourceLanguage::RSpiceAutomation {
            let entries = self
                .roles
                .iter()
                .filter(|binding| binding.role == ProjectSourceRole::AutomationEntry)
                .collect::<Vec<_>>();
            if entries.len() != 1 {
                return Err(ProjectSourceError::MissingAutomationEntryRole);
            }
            if path_key(entries[0].logical_path()) != root_key {
                return Err(ProjectSourceError::AutomationEntryMustBeRoot {
                    logical_path: entries[0].logical_path.clone(),
                    root_path: self.root.logical_path().to_owned(),
                });
            }
        }

        validate_dependency_graph(&root_key, &graph)?;
        if self.qualifications.len() > MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        let mut previous_qualification_sequence = 0_u64;
        for record in &self.qualifications {
            record.validate()?;
            if record.sequence <= previous_qualification_sequence
                || record.source_revision > self.revision.get()
            {
                return Err(ProjectSourceError::InvalidQualificationRecord);
            }
            previous_qualification_sequence = record.sequence;
        }
        if self.history.len() > MAX_SOURCE_HISTORY_REVISIONS {
            return Err(ProjectSourceError::RetainedHistoryLimitExceeded);
        }
        let retained_bytes = self
            .history
            .iter()
            .map(ProjectSourceRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        if retained_bytes > MAX_SOURCE_HISTORY_BYTES && self.history.len() > 1 {
            return Err(ProjectSourceError::RetainedHistoryLimitExceeded);
        }
        let mut previous_revision = None;
        for snapshot in &self.history {
            if snapshot.revision >= self.revision
                || previous_revision.is_some_and(|revision| revision >= snapshot.revision)
            {
                return Err(ProjectSourceError::CorruptRetainedRevision {
                    revision: snapshot.revision.get(),
                });
            }
            let historical = ProjectSourceBundle {
                id: self.id,
                owner: snapshot.owner.clone(),
                language: self.language,
                root: snapshot.root.clone(),
                files: snapshot.files.clone(),
                dependencies: snapshot.dependencies.clone(),
                roles: snapshot.roles.clone(),
                revision: snapshot.revision,
                validated_identity: None,
                history: Vec::new(),
                qualifications: Vec::new(),
            };
            historical.validate()?;
            if historical.closure_digest() != snapshot.closure_digest {
                return Err(ProjectSourceError::CorruptRetainedRevision {
                    revision: snapshot.revision.get(),
                });
            }
            previous_revision = Some(snapshot.revision);
        }
        if self.validated_identity.is_some() && !self.validation_is_current() {
            return Err(ProjectSourceError::StaleBundleValidationIdentity { id: self.id });
        }
        Ok(())
    }

    pub(super) fn canonicalize(&mut self) {
        self.files.sort_by(|left, right| {
            path_key(&left.logical_path)
                .cmp(&path_key(&right.logical_path))
                .then_with(|| left.logical_path.cmp(&right.logical_path))
        });
        self.dependencies.sort_by(|left, right| {
            left.canonical_key()
                .cmp(&right.canonical_key())
                .then_with(|| left.cmp(right))
        });
        self.roles.sort_by(|left, right| {
            left.canonical_key()
                .cmp(&right.canonical_key())
                .then_with(|| left.cmp(right))
        });
    }

    pub(super) fn advance_revision(&mut self) -> Result<(), ProjectSourceError> {
        self.revision = next_revision(self.revision, self.root.file_name())?;
        Ok(())
    }

    pub(super) fn record_revision_snapshot(&mut self, source: &ProjectSourceBundle) {
        let digest = source.closure_digest();
        if self.history.last().is_some_and(|snapshot| {
            snapshot.revision == source.revision && snapshot.closure_digest == digest
        }) {
            return;
        }
        self.history.push(ProjectSourceRevisionSnapshot {
            revision: source.revision,
            closure_digest: digest,
            owner: source.owner.clone(),
            root: source.root.clone(),
            files: source.files.clone(),
            dependencies: source.dependencies.clone(),
            roles: source.roles.clone(),
        });
        let mut retained_bytes = self
            .history
            .iter()
            .map(ProjectSourceRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        while self.history.len() > MAX_SOURCE_HISTORY_REVISIONS
            || (retained_bytes > MAX_SOURCE_HISTORY_BYTES && self.history.len() > 1)
        {
            retained_bytes = retained_bytes.saturating_sub(self.history[0].retained_bytes());
            self.history.remove(0);
        }
    }

    pub(super) fn invalidate_validation(&mut self) {
        self.validated_identity = None;
        self.root.invalidate_validation();
    }

    /// One-time migration from the pre-role schema. Runtime execution never
    /// calls this heuristic. It exists only while decoding schema v1, after
    /// which the resolved bindings are persisted explicitly in schema v2.
    pub(super) fn migrate_schema_v1(&mut self) -> Result<(), ProjectSourceError> {
        if self.root.id.is_missing() {
            self.root.id = ProjectSourceDocumentId::migrated(self.id, self.root.logical_path());
        }
        for file in &mut self.files {
            file.revision = self.revision;
            if file.id.is_missing() {
                file.id = ProjectSourceDocumentId::migrated(self.id, file.logical_path());
            }
        }
        if self.language != ProjectSourceLanguage::RSpiceAutomation {
            self.invalidate_validation();
            return Ok(());
        }
        self.roles = default_source_roles(self.language, self.root.logical_path())?;
        if self
            .root
            .logical_path()
            .to_ascii_lowercase()
            .ends_with(".rspice")
        {
            self.invalidate_validation();
            self.canonicalize();
            return Ok(());
        }

        let run_plans = self
            .files
            .iter()
            .filter(|file| {
                let path = file.logical_path().to_ascii_lowercase();
                path.ends_with(".yaml") || path.ends_with(".yml")
            })
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        for path in run_plans {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                path,
                ProjectSourceRole::AutomationRunPlan,
            )?);
        }
        let locks = self
            .files
            .iter()
            .filter(|file| file.content().contains("rspice-python-lock/"))
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        if let [path] = locks.as_slice() {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                *path,
                ProjectSourceRole::AutomationEnvironmentLock,
            )?);
        }
        let permission_manifests = self
            .files
            .iter()
            .filter(|file| {
                file.content().lines().any(|line| {
                    let key = line.split('=').next().unwrap_or_default().trim();
                    matches!(
                        key,
                        "project_files" | "artifact_directory" | "network" | "process_spawn"
                    )
                })
            })
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        if let [path] = permission_manifests.as_slice() {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                *path,
                ProjectSourceRole::AutomationPermissionManifest,
            )?);
        }
        self.invalidate_validation();
        self.canonicalize();
        Ok(())
    }
}
