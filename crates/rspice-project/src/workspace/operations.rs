//! Project source, hardcopy and configuration transactions.

use super::catalog::validate_hardcopy_source_set_catalog;
use super::*;

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
        candidate: rspice_design::configuration_set::ConfigurationSetCatalog,
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
        let next_revision = self.project.advance_revision()?;
        self.configuration_sets = candidate;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Publish a complete design-management candidate and its owning project
    /// revision atomically. Validation happens before any live state changes;
    /// failed candidates therefore cannot partially alter sheet, variant,
    /// annotation, or hierarchy-audit authority.
    pub fn replace_design_management(
        &mut self,
        candidate: rspice_design_model::design_management::DesignManagementCatalog,
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
        let next_revision = self.project.advance_revision()?;
        self.design_management = published;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Republish one cell view's sheets into this workspace's design
    /// management, leaving every other authority inside it alone.
    ///
    /// A document-scoped save writes one cell view into the accepted baseline.
    /// Its sheets are part of that document and travel with it, while the
    /// variants, the annotation journal, the drawing-sheet defaults and every
    /// other cell view's catalog belong to the project document and must stay
    /// exactly as the last project save left them.
    ///
    /// The catalog travels as one whole value: a catalog rebuilt from its
    /// sheets alone loses the revision and the semantic digests its own
    /// validation demands. The design-management revision travels with it
    /// because it is one coordinate over all of those authorities — a merged
    /// baseline left at the older number would report the project
    /// configuration unsaved for as long as the session lasts.
    pub fn overlay_sheet_catalog_from(
        &mut self,
        reference: &CellViewRef,
        source: &Self,
    ) -> Result<(), ProjectConfigurationMutationError> {
        let key = reference.key();
        let mut merged = self.design_management.clone();
        match source.design_management.sheet_catalog(&key) {
            Some(catalog) => {
                *merged
                    .ensure_sheet_catalog(&key)
                    .map_err(invalid_design_management)? = catalog.clone();
            }
            None if merged.sheet_catalog(&key).is_some() => {
                merged
                    .remove_sheet_catalog_for_view(&key)
                    .map_err(invalid_design_management)?;
            }
            None => {}
        }
        self.design_management =
            design_management_at_revision(&merged, source.design_management.revision())?;
        Ok(())
    }

    /// Commit a validated page setup through the project dirty lifecycle.
    /// Re-saving byte-identical settings is a no-op and does not manufacture
    /// an unsaved project change.
    pub fn save_hardcopy_setup(
        &mut self,
        source: &rspice_hardcopy_contract::ActiveHardcopySource,
        setup: rspice_hardcopy_contract::HardcopySetup,
    ) -> Result<rspice_hardcopy_contract::SetupSaveOutcome, rspice_hardcopy_contract::HardcopyError>
    {
        let outcome = self.hardcopy_setups.save(source, setup)?;
        if outcome.disposition() != rspice_hardcopy_contract::SetupSaveDisposition::Unchanged {
            self.hardcopy_setups_dirty = true;
        }
        Ok(outcome)
    }

    /// Append one sealed publication outcome to the bounded project ledger.
    /// This is the only path from runtime hardcopy execution into durable
    /// project evidence.
    pub fn record_hardcopy_receipt(
        &mut self,
        receipt: rspice_hardcopy_contract::HardcopyReceipt,
    ) -> Result<(), rspice_hardcopy_contract::HardcopyError> {
        self.hardcopy_receipts.append(receipt)?;
        self.hardcopy_receipts_dirty = true;
        Ok(())
    }

    /// Persist a reusable project print-set mapping through the same project
    /// dirty lifecycle as document page setups.
    pub fn save_project_print_mapping(
        &mut self,
        table: rspice_hardcopy_contract::PrintMappingTable,
    ) -> Result<
        rspice_hardcopy_contract::PrintMappingSaveReceipt,
        rspice_hardcopy_contract::PrintMappingPersistenceError,
    > {
        let outcome = self.project_print_mappings.save(table)?;
        if outcome.disposition() != rspice_hardcopy_contract::PrintMappingSaveDisposition::Unchanged
        {
            self.project_print_mappings_dirty = true;
        }
        Ok(outcome)
    }

    #[must_use]
    pub fn hardcopy_source_sets(&self) -> &[rspice_hardcopy_contract::sources::HardcopySourceSet] {
        &self.hardcopy_source_sets
    }

    #[must_use]
    pub fn hardcopy_source_set(
        &self,
        source_key: &str,
    ) -> Option<&rspice_hardcopy_contract::sources::HardcopySourceSet> {
        self.hardcopy_source_sets
            .iter()
            .find(|source_set| source_set.source_key() == source_key)
    }

    /// Insert or replace one exact source-set definition as a small,
    /// validated transaction. This never clones the rest of the project.
    pub fn save_hardcopy_source_set(
        &mut self,
        source_set: rspice_hardcopy_contract::sources::HardcopySourceSet,
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
        self.validate_physical_layout_technology_change(&binding)?;
        let before = self.project.revision();
        let revision = self.project.attach_technology(binding)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok(revision)
    }

    pub fn attach_technology_audited(
        &mut self,
        binding: ProjectTechnologyBinding,
        context: ProjectTechnologyChangeContext,
    ) -> Result<(ObjectRevision, ProjectTechnologyChangeReceipt), ProjectDescriptorError> {
        self.validate_physical_layout_technology_change(&binding)?;
        let before = self.project.revision();
        let (revision, receipt) = self.project.attach_technology_audited(binding, context)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok((revision, receipt))
    }

    /// Record the cloud circuit this project publishes to.
    pub fn bind_cloud_publication(
        &mut self,
        binding: ProjectCloudPublicationBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        let before = self.project.revision();
        let revision = self.project.bind_cloud_publication(binding)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok(revision)
    }

    fn validate_physical_layout_technology_change(
        &self,
        binding: &ProjectTechnologyBinding,
    ) -> Result<(), ProjectDescriptorError> {
        if self.physical_layout_documents().is_empty() {
            return Ok(());
        }
        let pin = binding.signed_package();
        for document in self.physical_layout_documents().values() {
            let technology = document.technology();
            let matches = pin.is_some_and(|pin| {
                technology.package_id() == pin.package_id()
                    && technology.revision() == pin.revision()
                    && technology.manifest_digest() == pin.manifest_digest()
                    && technology.archive_digest() == pin.archive_digest()
                    && technology.stack_id() == pin.stack_name()
            });
            if !matches {
                return Err(
                    ProjectDescriptorError::TechnologyConflictsWithPhysicalLayout {
                        owner: document.owner().display_path(),
                    },
                );
            }
        }
        Ok(())
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

    /// Replace one exact document in a project-owned source closure and enter
    /// the same persisted dirty lifecycle as root-document edits.
    pub fn replace_project_source_bundle_file(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        logical_path: &str,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .replace_bundle_file_content(bundle_id, logical_path, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Commit a workspace-wide replacement as one persisted source-graph
    /// transaction. Partial replacement is never observable.
    pub fn replace_project_source_bundle_files_transactionally(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        replacements: impl IntoIterator<Item = (String, String)>,
    ) -> Result<usize, ProjectSourceError> {
        let changed = self
            .project_sources
            .replace_bundle_files_transactionally(bundle_id, replacements)?;
        if changed > 0 {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Add one project-owned source document and its authenticated dependency
    /// edge as a single dirty-state transaction.
    pub fn add_project_source_bundle_file(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        importer_path: &str,
        file: rspice_design::project_sources::ProjectSourceFile,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .add_bundle_file(bundle_id, importer_path, file)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Persist a source document and its semantic role as one authenticated
    /// source-graph revision.
    pub fn add_project_source_bundle_file_with_role(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        importer_path: &str,
        file: rspice_design::project_sources::ProjectSourceFile,
        role: rspice_design::project_sources::ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .add_bundle_file_with_role(bundle_id, importer_path, file, role)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    pub fn append_project_source_qualification(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        record: rspice_design::project_sources::ProjectSourceQualificationAttempt,
    ) -> Result<u64, ProjectSourceError> {
        let sequence = self
            .project_sources
            .append_bundle_qualification(bundle_id, record)?;
        self.project_sources_dirty = true;
        Ok(sequence)
    }

    /// Rename a project-owned source while atomically migrating its roles,
    /// dependency edges, and language-specific include references.
    pub fn rename_project_source_bundle_file(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        current_path: &str,
        new_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .rename_bundle_file(bundle_id, current_path, new_path)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Remove a non-root project-owned source only after the bundle's role and
    /// dependency invariants accept the transaction.
    pub fn remove_project_source_bundle_file(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        logical_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .remove_bundle_file(bundle_id, logical_path)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Assign or clear one persisted non-entry Automation role in a single
    /// dirty source-graph transaction.
    pub fn set_project_source_bundle_non_entry_role(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        logical_path: &str,
        role: Option<rspice_design::project_sources::ProjectSourceRole>,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .set_bundle_non_entry_role(bundle_id, logical_path, role)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Insert a new project-owned source bundle and participate in the
    /// ordinary project dirty/save/recovery lifecycle.
    pub fn insert_project_source_bundle(
        &mut self,
        bundle: rspice_design::project_sources::ProjectSourceBundle,
    ) -> Result<(), ProjectSourceError> {
        self.project_sources.insert_bundle(bundle)?;
        self.project_sources_dirty = true;
        Ok(())
    }

    /// Restore a retained project-source revision as a new monotonic dirty
    /// revision. The registry owns the complete graph transaction; the
    /// workspace owns only the ordinary persisted dirty lifecycle.
    pub fn restore_project_source_bundle_revision(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
        expected_current: rspice_app_types::product::ObjectRevision,
        retained_revision: rspice_app_types::product::ObjectRevision,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self.project_sources.restore_bundle_revision(
            bundle_id,
            expected_current,
            retained_revision,
        )?;
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

    /// Retain validation for an exact source bundle, including every
    /// dependency file in its authenticated closure.
    pub fn mark_project_source_bundle_validated(
        &mut self,
        bundle_id: rspice_design::project_sources::ProjectSourceId,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        let before = self
            .project_sources
            .get_bundle(bundle_id)
            .and_then(rspice_design::project_sources::ProjectSourceBundle::validated_identity);
        let identity = self.project_sources.mark_bundle_validated(bundle_id)?;
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

    pub fn mark_all_clean(&mut self) {
        for view in &mut self.open_views {
            view.dirty = false;
        }
        self.netlist_source_dirty = false;
        self.project_sources_dirty = false;
        self.project_metadata_dirty = false;
        self.report_documents_dirty = false;
        self.visualization_documents_dirty = false;
        self.hardcopy_setups_dirty = false;
        self.hardcopy_receipts_dirty = false;
        self.project_print_mappings_dirty = false;
        self.hardcopy_source_sets_dirty = false;
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self.netlist_source_dirty
            || self.project_sources_dirty
            || self.project_metadata_dirty
            || self.report_documents_dirty
            || self.visualization_documents_dirty
            || self.hardcopy_setups_dirty
            || self.hardcopy_receipts_dirty
            || self.project_print_mappings_dirty
            || self.hardcopy_source_sets_dirty
    }
    /// Bind newly authored schematic objects to the currently active sheet,
    /// returning what the reconciliation actually changed.
    ///
    /// Legacy projects with no sheet catalog remain untouched; once the
    /// user enters multi-sheet authoring, every later object receives durable
    /// membership at the same save/sync boundary as its schematic edit.
    ///
    /// `recorded` is where the caller last saw each object. An object that a
    /// schematic undo has just brought back returns to its own sheet through
    /// it, instead of piling onto whichever sheet happens to be active.
    pub fn assign_unowned_objects_to_active_sheet(
        &mut self,
        reference: &CellViewRef,
        schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
        recorded: &BTreeMap<u64, rspice_design_model::design_management::SheetId>,
    ) -> Result<
        Option<rspice_design_model::design_management::SheetReconcileReceipt>,
        ProjectConfigurationMutationError,
    > {
        let key = reference.key();
        let Some(catalog) = self.design_management.sheet_catalog(&key) else {
            return Ok(None);
        };
        let Some(active_sheet_id) = catalog.active_sheet_id() else {
            return Ok(None);
        };
        let live_object_ids = schematic
            .as_ref()
            .components
            .iter()
            .map(|object| object.id)
            .chain(schematic.as_ref().wires.iter().map(|object| object.id))
            .chain(schematic.as_ref().buses.iter().map(|object| object.id))
            .chain(schematic.as_ref().bus_taps.iter().map(|object| object.id))
            .chain(schematic.as_ref().junctions.iter().map(|object| object.id))
            .chain(schematic.as_ref().net_labels.iter().map(|object| object.id))
            .chain(
                schematic
                    .as_ref()
                    .design_notes
                    .iter()
                    .map(|object| object.id),
            )
            .chain(
                schematic
                    .as_ref()
                    .documentation_shapes
                    .iter()
                    .map(|object| object.id),
            )
            .chain(schematic.as_ref().probes.iter().map(|object| object.id))
            .collect::<Vec<_>>();

        let mut candidate = self.design_management.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .expect("the cloned catalog retains the validated cell/view key");
        let receipt = catalog
            .reconcile_object_assignments_with(
                catalog.revision(),
                live_object_ids,
                recorded,
                Some(active_sheet_id),
            )
            .map_err(invalid_design_management)?;
        if receipt.added_assignments == 0
            && receipt.removed_assignments == 0
            && receipt.removed_cross_sheet_ports == 0
        {
            return Ok(None);
        }
        self.replace_design_management(candidate)?;
        Ok(Some(receipt))
    }
}

fn invalid_design_management(
    source: rspice_design_model::design_management::DesignManagementError,
) -> ProjectConfigurationMutationError {
    ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
        message: source.to_string(),
    }
}

/// Restate a design-management catalog at an exact project revision.
///
/// The revision is the catalog's own transaction coordinate: every operation
/// on it derives the next one, and nothing may set it. A merged save is not a
/// new transaction, though — it republishes content the session already
/// committed — so it has to land on the revision the session is at. The
/// catalog is restated through the same wire form the project file is written
/// in, which revalidates the whole aggregate on the way back.
fn design_management_at_revision(
    catalog: &rspice_design_model::design_management::DesignManagementCatalog,
    revision: u64,
) -> Result<
    rspice_design_model::design_management::DesignManagementCatalog,
    ProjectConfigurationMutationError,
> {
    let mut wire = serde_json::to_value(catalog).map_err(|source| {
        ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
            message: source.to_string(),
        }
    })?;
    wire["revision"] = serde_json::Value::from(revision);
    serde_json::from_value(wire).map_err(|source| {
        ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
            message: source.to_string(),
        }
    })
}
