//! Creating and revising models the project itself owns.
//!
//! Every mutation renders, parses, and checks the candidate completely before
//! the manager is touched, so a rejected edit leaves the library exactly as it
//! was — there is no partially-created model and no half-applied revision.
//! Each revision is built as a whole library rather than patched in place,
//! which is what makes the round-trip verification at the end meaningful.

use super::*;

impl ModelLibraryManager {
    /// Create a new single-card model whose exact source is owned by the
    /// project. The candidate is rendered, parsed, and checked completely
    /// before the manager is mutated.
    pub fn create_project_model(
        &mut self,
        library_name: &str,
        definition: &ProjectModelDefinition,
    ) -> Result<ProjectModelCommit, String> {
        validate_project_library_name(library_name)?;
        if let Some(existing) = self
            .libraries
            .keys()
            .find(|existing| existing.eq_ignore_ascii_case(library_name))
        {
            return Err(format!(
                "Model library '{library_name}' conflicts with existing library '{existing}'"
            ));
        }

        let source_id = ModelSourceId::new();
        let revision = ObjectRevision::INITIAL;
        let root = super::super::project_owned_source_path(source_id);
        let after = Self::build_project_model_library(
            library_name,
            None,
            source_id,
            revision,
            root,
            definition,
        )?;
        let model_name = definition.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: None,
            after,
            affects_execution: true,
        })
    }

    /// Create one complete project-owned model revision. The base card,
    /// process sections, typed schema, statistical definition, and temperature
    /// laws are validated and published with one source identity or not at
    /// all.
    pub fn create_project_model_revision(
        &mut self,
        library_name: &str,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        validate_project_library_name(library_name)?;
        if let Some(existing) = self
            .libraries
            .keys()
            .find(|existing| existing.eq_ignore_ascii_case(library_name))
        {
            return Err(format!(
                "Model library '{library_name}' conflicts with existing library '{existing}'"
            ));
        }

        let source_id = ModelSourceId::new();
        let revision = ObjectRevision::INITIAL;
        let root = super::super::project_owned_source_path(source_id);
        let after = Self::build_project_model_revision_library(
            library_name,
            None,
            source_id,
            revision,
            root,
            definition,
            qualification,
        )?;
        let model_name = definition.base.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: None,
            after,
            affects_execution: true,
        })
    }

    /// Replace one editable project model using optimistic source-revision
    /// guards. External, built-in, multi-card, and stale sources fail closed.
    pub fn replace_project_model(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_revision: ObjectRevision,
        definition: &ProjectModelDefinition,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            ..
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        if before.models.len() != 1
            || before.source_closure.len() != 1
            || before.source_contents.len() != 1
            || !before.source_edges.is_empty()
            || !before.corners.is_empty()
        {
            return Err(format!(
                "Model library '{library_name}' is not an editable single-card definition"
            ));
        }
        let next_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let root = before.root_path.clone().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no source identity")
        })?;
        let after = Self::build_project_model_library(
            library_name,
            Some(&before),
            source_id,
            next_revision,
            root,
            definition,
        )?;
        if before.source_contents[0].bytes == after.source_contents[0].bytes {
            return Err("Model candidate has no source changes to save".to_owned());
        }
        let model_name = definition.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace one complete project-owned model revision using optimistic
    /// source-revision guards. Validation and canonical source parsing finish
    /// before the live manager is mutated.
    pub fn replace_project_model_revision(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_revision: ObjectRevision,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            ..
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        if before.source_closure.len() != 1
            || before.source_contents.len() != 1
            || !before.source_edges.is_empty()
        {
            return Err(format!(
                "Model library '{library_name}' is not a complete editable project-model revision"
            ));
        }
        let previous_model_name = before.models.keys().next().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no model projection")
        })?;
        if before.models.len() != 1
            || !before
                .model_definition_metadata
                .contains_key(previous_model_name)
        {
            return Err(format!(
                "Model library '{library_name}' does not have one coherent editable definition"
            ));
        }
        let display_name = before
            .model_definition_metadata
            .get(previous_model_name)
            .and_then(|metadata| metadata.sections.first())
            .and_then(|section| section.model_files.first())
            .map_or_else(
                || format!("{library_name}.model"),
                |identity| identity.display_name.clone(),
            );
        let current_identity_candidate = definition
            .clone()
            .bind_project_source_identity(source_id, revision, display_name)
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let current_identity_source = current_identity_candidate
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        if before.source_contents[0].bytes == current_identity_source.into_bytes()
            && before.model_definition_metadata.get(previous_model_name)
                == Some(&current_identity_candidate.metadata)
            && before
                .model_qualification
                .get(previous_model_name)
                .cloned()
                .unwrap_or_default()
                == *qualification
        {
            return Err("Model candidate has no semantic changes to save".to_owned());
        }

        let next_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let root = before.root_path.clone().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no source identity")
        })?;
        let after = Self::build_project_model_revision_library(
            library_name,
            Some(&before),
            source_id,
            next_revision,
            root,
            definition,
            qualification,
        )?;
        let model_name = definition.base.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace one exact canonical model revision inside a project-owned
    /// multi-model/include source closure. Every untouched byte and graph edge
    /// is retained verbatim. The transaction fails unless the selected old
    /// revision occurs exactly once in the source member recorded by its model
    /// projection, so editing one card can never rewrite an adjacent model.
    pub fn replace_project_model_revision_in_library(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_name: &str,
        expected_model_digest: ContentDigest,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let old_model = before.models.get(expected_model_name).ok_or_else(|| {
            format!("Model '{expected_model_name}' no longer exists in library '{library_name}'")
        })?;
        let old_metadata = before
            .model_definition_metadata
            .get(expected_model_name)
            .ok_or_else(|| {
                format!(
                    "Model '{expected_model_name}' has no typed project-owned definition metadata"
                )
            })?;
        let old_definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(old_model),
            old_metadata.clone(),
        );
        let old_source = old_definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let actual_model_digest =
            ContentDigest::from_bytes(Sha256::digest(old_source.as_bytes()).into());
        let old_identity = old_definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let actual_model_revision = old_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if old_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || actual_model_revision != expected_model_revision
            || actual_model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{expected_model_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        let source_path = old_model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{expected_model_name}' has no retained source-file projection")
        })?;
        let content_index = before
            .source_contents
            .iter()
            .position(|content| content.path == *source_path)
            .ok_or_else(|| {
                format!(
                    "Model '{expected_model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let pin_index = before
            .source_closure
            .iter()
            .position(|pin| pin.path == *source_path)
            .ok_or_else(|| {
                format!(
                    "Retained model source '{}' has no authenticated pin",
                    source_path.display()
                )
            })?;
        let old_bytes = &before.source_contents[content_index].bytes;
        let offsets = exact_subslice_offsets(old_bytes, old_source.as_bytes());
        let [offset] = offsets.as_slice() else {
            return Err(format!(
                "Model '{expected_model_name}' canonical revision must occur exactly once in retained source '{}' (found {})",
                source_path.display(),
                offsets.len()
            ));
        };

        let next_library_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let next_model_revision = actual_model_revision
            .next()
            .map_err(|error| format!("Cannot revise model '{expected_model_name}': {error}"))?;
        let display_name = old_metadata
            .sections
            .first()
            .and_then(|section| section.model_files.first())
            .map(|identity| identity.display_name.clone())
            .or_else(|| {
                source_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| format!("{library_name}.model"));
        let mut bound = definition
            .clone()
            .bind_project_source_identity(source_id, next_model_revision, display_name)
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let new_source = bound
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        bound
            .verify_source_round_trip(&new_source)
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        let identity = bound
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?
            .ok_or_else(|| "Project model source identity was not bound".to_owned())?;
        for section in &mut bound.metadata.sections {
            if !matches!(
                section.qualification,
                ModelSectionQualification::Unqualified
            ) {
                section.qualification = ModelSectionQualification::Unqualified;
            }
        }
        qualification
            .validate_for_model(&bound.base.name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            &bound.base.name,
            source_id,
            identity.content_digest,
            next_model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let retained_qualification = qualification
            .reconcile_after_source_revision(&current_source)
            .map_err(|error| {
                format!("Project model qualification migration is invalid: {error}")
            })?;
        validate_section_qualification_evidence(
            &bound.metadata,
            &retained_qualification,
            &current_source,
        )?;
        if bound.base.name != expected_model_name && before.models.contains_key(&bound.base.name) {
            return Err(format!(
                "Model '{}' already exists in library '{library_name}'",
                bound.base.name
            ));
        }
        if bound.base.name != expected_model_name
            && before
                .model_qualification
                .get(expected_model_name)
                .is_some_and(|retained| *retained != ModelQualificationState::default())
        {
            return Err(
                "A qualified model cannot be renamed without an explicit release-lineage migration"
                    .to_owned(),
            );
        }
        if bound.base.name != expected_model_name
            && before
                .model_correlation
                .get(expected_model_name)
                .is_some_and(|retained| *retained != ModelCorrelationState::default())
        {
            return Err(
                "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                    .to_owned(),
            );
        }

        let mut parser = rspice_core::library::LibParser::new(
            source_path.parent().unwrap_or_else(|| Path::new("/")),
        );
        let parsed = parser.parse_string(&new_source);
        if !parsed.is_ok() || parsed.top_level_models.len() != 1 {
            return Err(format!(
                "Project model source could not be projected: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        let mut device_model = Self::convert_parsed_model(&parsed.top_level_models[0], source_path);
        device_model.spice_type = Some(bound.base.spice_type.to_ascii_uppercase());
        device_model.description = bound.base.description.clone();
        device_model.file_path = Some(source_path.clone());
        device_model.source_line = parsed.top_level_models[0].source_line.map(|relative_line| {
            old_bytes[..*offset]
                .iter()
                .filter(|byte| **byte == b'\n')
                .count()
                + relative_line
        });

        let mut after = before.clone();
        let content = &mut after.source_contents[content_index].bytes;
        content.splice(*offset..(*offset + old_source.len()), new_source.bytes());
        let changed_member_digest =
            ContentDigest::from_bytes(Sha256::digest(content.as_slice()).into());
        after.source_closure[pin_index].digest = changed_member_digest;
        let next_root_digest = if after.root_path.as_ref() == Some(source_path) {
            changed_member_digest
        } else {
            root_digest
        };
        after.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: next_library_revision,
            digest: next_root_digest,
        };
        after.models.remove(expected_model_name);
        after.models.insert(bound.base.name.clone(), device_model);
        after.model_definition_metadata.remove(expected_model_name);
        after
            .model_definition_metadata
            .insert(bound.base.name.clone(), bound.metadata.clone());
        after.model_qualification.remove(expected_model_name);
        if retained_qualification != ModelQualificationState::default() {
            after
                .model_qualification
                .insert(bound.base.name.clone(), retained_qualification);
        }
        if bound.base.name != expected_model_name {
            after.model_correlation.remove(expected_model_name);
        }
        after.version = next_library_revision.get().to_string();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: bound.base.name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace only the qualification/release aggregate for an exact
    /// project-owned model source. The source identity, bytes, digest, and
    /// revision remain unchanged so newly produced evidence does not become
    /// stale as a side effect of persisting it.
    pub fn replace_project_model_qualification(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after qualification began; rerun against the current source revision"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let Some(model) = before.models.get(model_name) else {
            return Err(format!(
                "Model library '{library_name}' does not contain model '{model_name}'"
            ));
        };
        let metadata = before
            .model_definition_metadata
            .get(model_name)
            .ok_or_else(|| {
                format!("Model '{model_name}' has no typed project-owned definition metadata")
            })?;
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata.clone(),
        );
        let canonical = definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_identity = definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let model_revision = model_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if model_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || model_revision != expected_model_revision
            || model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{model_name}' changed after qualification began; rerun against the current source revision"
            ));
        }
        let source_path = model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{model_name}' has no retained source-file projection")
        })?;
        let source_bytes = before
            .source_contents
            .iter()
            .find(|content| content.path == *source_path)
            .map(|content| content.bytes.as_slice())
            .ok_or_else(|| {
                format!(
                    "Model '{model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let occurrences = exact_subslice_offsets(source_bytes, canonical.as_bytes()).len();
        if occurrences != 1 {
            return Err(format!(
                "Model '{model_name}' canonical revision must occur exactly once in retained source '{}' (found {occurrences})",
                source_path.display()
            ));
        }
        qualification
            .validate_for_model(model_name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            model_name,
            source_id,
            model_digest,
            model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        validate_section_qualification_evidence(metadata, qualification, &current_source)?;
        let retained = before
            .model_qualification
            .get(model_name)
            .cloned()
            .unwrap_or_default();
        if retained == *qualification {
            return Err("Model qualification has no semantic changes to save".to_owned());
        }

        let mut after = before.clone();
        if *qualification == ModelQualificationState::default() {
            after.model_qualification.remove(model_name);
        } else {
            after
                .model_qualification
                .insert(model_name.to_owned(), qualification.clone());
        }
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: model_name.to_owned(),
            before: Some(before),
            after,
            affects_execution: false,
        })
    }

    /// Replace only the measurement-correlation aggregate for an exact
    /// project-owned model source. Source bytes and revisions remain
    /// unchanged; historical suites may remain retained while every new suite
    /// binds the exact source revision selected by its author.
    pub fn replace_project_model_correlation(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        correlation: &ModelCorrelationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after correlation review began; reload the current source revision"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let model = before.models.get(model_name).ok_or_else(|| {
            format!("Model library '{library_name}' does not contain model '{model_name}'")
        })?;
        let metadata = before
            .model_definition_metadata
            .get(model_name)
            .ok_or_else(|| {
                format!("Model '{model_name}' has no typed project-owned definition metadata")
            })?;
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata.clone(),
        );
        let canonical = definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_identity = definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let model_revision = model_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if model_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || model_revision != expected_model_revision
            || model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{model_name}' changed after correlation review began; reload the current source revision"
            ));
        }
        let source_path = model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{model_name}' has no retained source-file projection")
        })?;
        let source_bytes = before
            .source_contents
            .iter()
            .find(|content| content.path == *source_path)
            .map(|content| content.bytes.as_slice())
            .ok_or_else(|| {
                format!(
                    "Model '{model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let occurrences = exact_subslice_offsets(source_bytes, canonical.as_bytes()).len();
        if occurrences != 1 {
            return Err(format!(
                "Model '{model_name}' canonical revision must occur exactly once in retained source '{}' (found {occurrences})",
                source_path.display()
            ));
        }
        correlation
            .validate_for_model(model_name)
            .map_err(|error| format!("Project model correlation state is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            model_name,
            source_id,
            model_digest,
            model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let retained = before
            .model_correlation
            .get(model_name)
            .cloned()
            .unwrap_or_default();
        if retained == *correlation {
            return Err("Model correlation has no semantic changes to save".to_owned());
        }
        for existing in &retained.suites {
            let replacement = correlation
                .suites
                .iter()
                .find(|candidate| {
                    candidate.id.eq_ignore_ascii_case(&existing.id)
                        && candidate.revision == existing.revision
                })
                .ok_or_else(|| {
                    format!(
                        "Correlation suite '{}@{}' is immutable and cannot be removed",
                        existing.id,
                        existing.revision.get()
                    )
                })?;
            if replacement != existing {
                return Err(format!(
                    "Correlation suite '{}@{}' is immutable and cannot be replaced",
                    existing.id,
                    existing.revision.get()
                ));
            }
        }
        for existing in &retained.evidence {
            let replacement = correlation
                .evidence
                .iter()
                .find(|candidate| candidate.id.eq_ignore_ascii_case(&existing.id))
                .ok_or_else(|| {
                    format!(
                        "Correlation evidence '{}' is immutable and cannot be removed",
                        existing.id
                    )
                })?;
            if replacement != existing {
                return Err(format!(
                    "Correlation evidence '{}' is immutable and cannot be replaced",
                    existing.id
                ));
            }
        }
        for suite in correlation.suites.iter().filter(|candidate| {
            !retained.suites.iter().any(|existing| {
                existing.id.eq_ignore_ascii_case(&candidate.id)
                    && existing.revision == candidate.revision
            })
        }) {
            if suite.source != current_source {
                return Err(format!(
                    "New correlation suite '{}@{}' must bind the exact current model source revision",
                    suite.id,
                    suite.revision.get()
                ));
            }
            if retained
                .suites
                .iter()
                .filter(|existing| existing.id.eq_ignore_ascii_case(&suite.id))
                .any(|existing| existing.revision >= suite.revision)
            {
                return Err(format!(
                    "New correlation suite revision '{}@{}' does not advance retained history",
                    suite.id,
                    suite.revision.get()
                ));
            }
        }
        for evidence in correlation.evidence.iter().filter(|candidate| {
            !retained
                .evidence
                .iter()
                .any(|existing| existing.id.eq_ignore_ascii_case(&candidate.id))
        }) {
            if evidence.source != current_source {
                return Err(format!(
                    "New correlation evidence '{}' must bind the exact current model source revision",
                    evidence.id
                ));
            }
        }

        let mut after = before.clone();
        if *correlation == ModelCorrelationState::default() {
            after.model_correlation.remove(model_name);
        } else {
            after
                .model_correlation
                .insert(model_name.to_owned(), correlation.clone());
        }
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: model_name.to_owned(),
            before: Some(before),
            after,
            affects_execution: false,
        })
    }

    fn build_project_model_library(
        library_name: &str,
        previous: Option<&ModelLibrary>,
        source_id: ModelSourceId,
        revision: ObjectRevision,
        root: PathBuf,
        definition: &ProjectModelDefinition,
    ) -> Result<ModelLibrary, String> {
        let source = definition.canonical_source()?;
        let bytes = source.into_bytes();
        let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        let mut parser =
            rspice_core::library::LibParser::new(root.parent().unwrap_or_else(|| Path::new("/")));
        let parsed = parser.parse_string(
            rspice_core::netlist::decode_source_bytes(&bytes)
                .map_err(|error| format!("Project model source cannot be decoded: {error}"))?
                .as_str(),
        );
        if !parsed.is_ok() {
            return Err(format!(
                "Project model source is invalid: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        if parsed.top_level_models.len() != 1
            || parsed.model_count() != 1
            || parsed.subcircuit_count() != 0
            || !parsed.sections.is_empty()
        {
            return Err(
                "Project model source must contain exactly one top-level .model card and no sections or subcircuits"
                    .to_owned(),
            );
        }
        let parsed_model = &parsed.top_level_models[0];
        if parsed_model.name != definition.name {
            return Err(format!(
                "Parsed model identity '{}' does not match candidate '{}'",
                parsed_model.name, definition.name
            ));
        }
        verify_project_model_round_trip(definition, parsed_model)?;

        let mut device_model = Self::convert_parsed_model(parsed_model, &root);
        device_model.spice_type = Some(definition.spice_type.to_ascii_uppercase());
        device_model.description = definition.description.clone();
        device_model.source_line = Some(
            definition
                .description
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count()
                + 1,
        );

        let mut library = ModelLibrary::new(library_name);
        let previous_model_name = previous
            .and_then(|library| library.models.keys().next())
            .cloned();
        if let Some(previous) = previous {
            library.pdk_name = previous.pdk_name.clone();
            library.technology_node = previous.technology_node.clone();
            library.expanded = previous.expanded;
        }
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest,
        };
        library.source_closure = vec![ModelSourcePin {
            path: root.clone(),
            digest,
        }];
        library.source_contents = vec![ModelSourceContent { path: root, bytes }];
        library.source_edges.clear();
        library.models.clear();
        library
            .models
            .insert(device_model.name.clone(), device_model);
        let previous_metadata = previous_model_name.as_deref().and_then(|model_name| {
            previous.and_then(|library| library.model_definition_metadata.get(model_name))
        });
        let mut metadata = reconcile_project_model_metadata(definition, previous_metadata)?;
        metadata.source_identity = Some(ModelFileIdentity {
            source_id: source_id.to_string(),
            revision: revision.get(),
            content_digest: digest.to_string(),
            display_name: format!("{library_name}.model"),
        });
        library
            .model_definition_metadata
            .insert(definition.name.clone(), metadata);
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(qualification) = previous.model_qualification.get(previous_model_name)
        {
            if previous_model_name != definition.name && *qualification != Default::default() {
                return Err(
                    "A qualified model cannot be renamed without an explicit release-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == definition.name {
                library
                    .model_qualification
                    .insert(definition.name.clone(), qualification.clone());
            }
        }
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(correlation) = previous.model_correlation.get(previous_model_name)
        {
            if previous_model_name != definition.name
                && *correlation != ModelCorrelationState::default()
            {
                return Err(
                    "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == definition.name {
                library
                    .model_correlation
                    .insert(definition.name.clone(), correlation.clone());
            }
        }
        library.corners.clear();
        library.selected_corner = None;
        library.version = revision.get().to_string();
        Ok(library)
    }

    fn build_project_model_revision_library(
        library_name: &str,
        previous: Option<&ModelLibrary>,
        source_id: ModelSourceId,
        revision: ObjectRevision,
        root: PathBuf,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ModelLibrary, String> {
        qualification
            .validate_for_model(&definition.base.name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let mut bound = definition
            .clone()
            .bind_project_source_identity(source_id, revision, format!("{library_name}.model"))
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let source = bound
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        bound
            .verify_source_round_trip(&source)
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        let identity = bound
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?
            .ok_or_else(|| "Project model source identity was not bound".to_owned())?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            &bound.base.name,
            source_id,
            identity.content_digest,
            revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let source_changed = previous.is_some_and(|library| {
            !matches!(
                library.source_authority,
                ModelSourceAuthority::ProjectOwned {
                    source_id: previous_source_id,
                    revision: previous_revision,
                    digest: previous_digest,
                } if previous_source_id == source_id
                    && previous_revision == revision
                    && previous_digest == identity.content_digest
            )
        });
        if source_changed {
            for section in &mut bound.metadata.sections {
                if !matches!(
                    section.qualification,
                    ModelSectionQualification::Unqualified
                ) {
                    section.qualification = ModelSectionQualification::Unqualified;
                }
            }
        }
        let retained_qualification = qualification
            .reconcile_after_source_revision(&current_source)
            .map_err(|error| {
                format!("Project model qualification migration is invalid: {error}")
            })?;
        validate_section_qualification_evidence(
            &bound.metadata,
            &retained_qualification,
            &current_source,
        )?;
        let bytes = source.into_bytes();

        let mut parser =
            rspice_core::library::LibParser::new(root.parent().unwrap_or_else(|| Path::new("/")));
        let parsed = parser.parse_string(
            rspice_core::netlist::decode_source_bytes(&bytes)
                .map_err(|error| format!("Project model source cannot be decoded: {error}"))?
                .as_str(),
        );
        if !parsed.is_ok() || parsed.top_level_models.len() != 1 {
            let details = parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(if details.is_empty() {
                "Project model source did not produce one top-level model".to_owned()
            } else {
                format!("Project model source could not be projected: {details}")
            });
        }

        let mut device_model = Self::convert_parsed_model(&parsed.top_level_models[0], &root);
        device_model.spice_type = Some(bound.base.spice_type.to_ascii_uppercase());
        device_model.description = bound.base.description.clone();
        device_model.source_line = Some(
            bound
                .base
                .description
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count()
                + 1,
        );

        let previous_model_name = previous
            .and_then(|library| library.models.keys().next())
            .cloned();
        let mut library = ModelLibrary::new(library_name);
        if let Some(previous) = previous {
            library.pdk_name = previous.pdk_name.clone();
            library.technology_node = previous.technology_node.clone();
            library.expanded = previous.expanded;
        }
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: identity.content_digest,
        };
        library.source_closure = vec![ModelSourcePin {
            path: root.clone(),
            digest: identity.content_digest,
        }];
        library.source_contents = vec![ModelSourceContent {
            path: root.clone(),
            bytes,
        }];
        library.source_edges.clear();
        library.models.clear();
        library.models.insert(bound.base.name.clone(), device_model);
        library.model_definition_metadata.clear();
        library
            .model_definition_metadata
            .insert(bound.base.name.clone(), bound.metadata.clone());
        library.model_qualification.clear();
        if let Some(previous_model_name) = previous_model_name.as_deref() {
            if previous_model_name != bound.base.name && *qualification != Default::default() {
                return Err(
                    "A qualified model cannot be renamed without an explicit release-lineage migration"
                        .to_owned(),
                );
            }
        }
        if retained_qualification != Default::default() {
            library
                .model_qualification
                .insert(bound.base.name.clone(), retained_qualification);
        }
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(correlation) = previous.model_correlation.get(previous_model_name)
        {
            if previous_model_name != bound.base.name
                && *correlation != ModelCorrelationState::default()
            {
                return Err(
                    "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == bound.base.name {
                library
                    .model_correlation
                    .insert(bound.base.name.clone(), correlation.clone());
            }
        }

        library.corners.clear();
        let selected_corner = bound
            .metadata
            .sections
            .iter()
            .find(|section| section.name.eq_ignore_ascii_case("tt"))
            .or_else(|| bound.metadata.sections.first())
            .map(|section| section.name.clone());
        for section in &bound.metadata.sections {
            let mut corner = ProcessCorner::new(&section.name);
            corner.description = format!("Project model section {}", section.name);
            corner.nmos_corner = section.name.to_ascii_lowercase();
            corner.pmos_corner = section.name.to_ascii_lowercase();
            corner.file_path = Some(root.clone());
            corner.is_default = selected_corner.as_deref() == Some(section.name.as_str());
            library.corners.insert(section.name.clone(), corner);
        }
        library.selected_corner = selected_corner;
        library.version = revision.get().to_string();
        Ok(library)
    }
}
