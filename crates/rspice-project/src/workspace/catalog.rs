//! Validated project catalogs and persistence invariants.

use super::*;

pub struct PreparedPhysicalLayoutCatalog {
    documents: BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
}

pub(super) fn validate_hardcopy_source_set_catalog(
    source_sets: &[rspice_hardcopy_contract::sources::HardcopySourceSet],
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

type RenamedLayoutDocument = Result<
    rspice_design::physical_layout::PhysicalLayoutDocument,
    rspice_design::physical_layout::LayoutDocumentError,
>;

pub(super) fn layout_catalog_is_empty(
    documents: &Stamped<BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>>,
) -> bool {
    documents.is_empty()
}

fn validate_physical_layout_document_catalog(
    documents: &BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
) -> Result<(), rspice_design::physical_layout::LayoutDocumentError> {
    for (key, document) in documents {
        document.validate()?;
        if document.owner().key() != *key {
            return Err(
                rspice_design::physical_layout::LayoutDocumentError::Invalid {
                    path: format!("physical_layout_documents[{key}].owner"),
                    message: format!(
                        "document owner '{}' does not match its exact catalog key",
                        document.owner().key()
                    ),
                },
            );
        }
    }
    Ok(())
}

/// A cell a copy reads and the cell it makes, each as its library and
/// its name.
pub type CellCopyPair<'a> = ((&'a str, &'a str), (&'a str, &'a str));

impl ProjectWorkspace {
    pub fn physical_layout_documents(
        &self,
    ) -> &BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument> {
        &self.physical_layout_documents
    }

    /// The stamp of the layouts as they stand: equal stamps, equal layouts.
    /// See [`rspice_design::stamped`].
    pub const fn physical_layout_stamp(&self) -> u64 {
        self.physical_layout_documents.stamp()
    }

    pub fn physical_layout_document(
        &self,
        owner: &CellViewRef,
    ) -> Option<&rspice_design::physical_layout::PhysicalLayoutDocument> {
        self.physical_layout_documents.get(&owner.key())
    }

    pub fn commit_physical_layout_document(
        &mut self,
        document: rspice_design::physical_layout::PhysicalLayoutDocument,
    ) -> Result<
        Option<rspice_design::physical_layout::PhysicalLayoutDocument>,
        rspice_design::physical_layout::LayoutDocumentError,
    > {
        document.validate()?;
        let key = document.owner().key();
        let mut candidate = self.physical_layout_documents.clone();
        let previous = candidate.insert(key, document);
        validate_physical_layout_document_catalog(&candidate)?;
        self.physical_layout_documents = candidate;
        Ok(previous)
    }

    pub fn prepare_insert_physical_layout_document(
        &self,
        document: rspice_design::physical_layout::PhysicalLayoutDocument,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        document.validate()?;
        let key = document.owner().key();
        let mut candidate = (*self.physical_layout_documents).clone();
        if candidate.insert(key.clone(), document).is_some() {
            return Err(
                rspice_design::physical_layout::LayoutDocumentError::DuplicateObject {
                    kind: "physical-layout document",
                    id: key,
                },
            );
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub fn synchronize_physical_layout_document_from(
        &mut self,
        owner: &CellViewRef,
        source: &Self,
    ) -> Result<bool, rspice_design::physical_layout::LayoutDocumentError> {
        let key = owner.key();
        let incoming = source.physical_layout_documents.get(&key).cloned();
        if self.physical_layout_documents.get(&key) == incoming.as_ref() {
            return Ok(false);
        }
        let mut candidate = self.physical_layout_documents.clone();
        match incoming {
            Some(document) => {
                candidate.insert(key, document);
            }
            None => {
                candidate.remove(&key);
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        self.physical_layout_documents = candidate;
        Ok(true)
    }

    pub fn remove_physical_layout_document(&mut self, owner: &CellViewRef) -> bool {
        self.physical_layout_documents
            .remove(&owner.key())
            .is_some()
    }

    /// The catalog with a copy of every layout each cell of `copies` owns,
    /// owned by that cell's copy. With `rebind`, a copy places the copies of
    /// the cells `copies` names where its original placed those cells; else
    /// it places what its original placed.
    pub fn prepare_copy_physical_layout_cells(
        &self,
        copies: &[CellCopyPair<'_>],
        rebind: bool,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        let mut candidate = (*self.physical_layout_documents).clone();
        for ((source_library, source_cell), (target_library, target_cell)) in copies {
            let originals = self
                .physical_layout_documents
                .values()
                .filter(|document| {
                    document.owner().library == *source_library
                        && document.owner().cell == *source_cell
                })
                .collect::<Vec<_>>();
            for original in originals {
                let mut copy = original.copy_for_cell(
                    source_library,
                    source_cell,
                    target_library,
                    target_cell,
                )?;
                if rebind {
                    for (from, to) in copies {
                        copy = copy.rebind_cell_references(*from, *to)?;
                    }
                }
                let key = copy.owner().key();
                if candidate.insert(key.clone(), copy).is_some() {
                    return Err(
                        rspice_design::physical_layout::LayoutDocumentError::DuplicateObject {
                            kind: "physical-layout document",
                            id: key,
                        },
                    );
                }
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub fn prepare_rename_physical_layout_cell_documents(
        &self,
        library: &str,
        source_cell: &str,
        target_cell: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_cell_references(library, source_cell, target_cell)
        })
    }

    /// The catalog with every layout's references to `from` naming `to`
    /// instead, each re-keyed by its owner: a cell moved to another library.
    pub fn prepare_rebind_physical_layout_cell_documents(
        &self,
        from: (&str, &str),
        to: (&str, &str),
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        self.prepare_renamed_layout_catalog(|document| document.rebind_cell_references(from, to))
    }

    pub fn prepare_rename_physical_layout_library_documents(
        &self,
        source_library: &str,
        target_library: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_library_references(source_library, target_library)
        })
    }

    pub fn prepare_rename_physical_layout_view_documents(
        &self,
        library: &str,
        cell: &str,
        from_view: &str,
        to_view: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        self.prepare_renamed_layout_catalog(|document| {
            document.rename_view_references(library, cell, from_view, to_view)
        })
    }

    /// Rebuild the whole catalog through one rename, re-keyed by each owner.
    /// Every document is offered the rename, not only those the renamed scope
    /// owns: a layout elsewhere may place a master from it and must follow.
    fn prepare_renamed_layout_catalog(
        &self,
        rename: impl Fn(
            &rspice_design::physical_layout::PhysicalLayoutDocument,
        ) -> RenamedLayoutDocument,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        let mut candidate = BTreeMap::new();
        for document in self.physical_layout_documents.values() {
            let document = rename(document)?;
            let key = document.owner().key();
            if candidate.insert(key.clone(), document).is_some() {
                return Err(
                    rspice_design::physical_layout::LayoutDocumentError::DuplicateObject {
                        kind: "physical-layout document",
                        id: key,
                    },
                );
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    /// The catalog with each of `documents` in place of what its owner key
    /// holds now, `None` taking the key's document away, validated whole
    /// before anything changes.
    pub fn prepare_replaced_physical_layout_documents(
        &self,
        documents: &BTreeMap<
            String,
            Option<rspice_design::physical_layout::PhysicalLayoutDocument>,
        >,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        let mut candidate = (*self.physical_layout_documents).clone();
        for (key, document) in documents {
            match document {
                Some(document) => {
                    candidate.insert(key.clone(), document.clone());
                }
                None => {
                    candidate.remove(key);
                }
            }
        }
        validate_physical_layout_document_catalog(&candidate)?;
        Ok(PreparedPhysicalLayoutCatalog {
            documents: candidate,
        })
    }

    pub fn commit_prepared_physical_layout_catalog(
        &mut self,
        prepared: PreparedPhysicalLayoutCatalog,
    ) {
        self.physical_layout_documents = prepared.documents.into();
    }

    pub fn validate_physical_layout_documents(&self) -> Result<(), String> {
        validate_physical_layout_document_catalog(&self.physical_layout_documents)
            .map_err(|error| error.to_string())
    }

    /// Commit a new visualization document into the project authority.
    ///
    /// Validation and every duplicate check run before the vector changes, so
    /// a failed dialog commit cannot leave a partial document or dirty bit.
    pub fn insert_visualization_document(
        &mut self,
        document: rspice_results::visualization_document::VisualizationDocument,
    ) -> Result<ResultDocumentId, VisualizationDocumentPersistenceError> {
        let at = self.visualization_documents.len();
        self.insert_visualization_document_at(document, at)
    }

    /// [`Self::insert_visualization_document`] at place `at` among the
    /// project's documents, or last when `at` is past them: a deleted
    /// document put back where it stood.
    pub fn insert_visualization_document_at(
        &mut self,
        document: rspice_results::visualization_document::VisualizationDocument,
        at: usize,
    ) -> Result<ResultDocumentId, VisualizationDocumentPersistenceError> {
        if self.visualization_documents.len() >= MAX_PROJECT_VISUALIZATION_DOCUMENTS {
            return Err(VisualizationDocumentPersistenceError::CatalogFull);
        }
        document.content_digest().map_err(|error| {
            VisualizationDocumentPersistenceError::Invalid {
                message: error.to_string(),
            }
        })?;
        if self
            .visualization_documents
            .iter()
            .any(|candidate| candidate.id() == document.id())
        {
            return Err(VisualizationDocumentPersistenceError::DuplicateIdentity {
                document_id: document.id(),
            });
        }
        if self
            .visualization_documents
            .iter()
            .any(|candidate| candidate.title().eq_ignore_ascii_case(document.title()))
        {
            return Err(VisualizationDocumentPersistenceError::DuplicateTitle {
                title: document.title().to_owned(),
            });
        }
        let document_id = document.id();
        let at = at.min(self.visualization_documents.len());
        self.visualization_documents.insert(at, document);
        self.visualization_documents_dirty = true;
        self.visualization_documents_modified.insert(document_id);
        self.visualization_documents_unstamped.insert(document_id);
        Ok(document_id)
    }

    /// Give the result document `document_id` the title `title`, which no
    /// other document of the project holds, case aside.
    pub fn rename_visualization_document(
        &mut self,
        document_id: ResultDocumentId,
        title: &str,
    ) -> Result<(), VisualizationDocumentPersistenceError> {
        if self.visualization_documents.iter().any(|candidate| {
            candidate.id() != document_id && candidate.title().eq_ignore_ascii_case(title)
        }) {
            return Err(VisualizationDocumentPersistenceError::DuplicateTitle {
                title: title.to_owned(),
            });
        }
        let document = self
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or(VisualizationDocumentPersistenceError::NotFound { document_id })?;
        document.retitle(title).map_err(|error| {
            VisualizationDocumentPersistenceError::Invalid {
                message: error.to_string(),
            }
        })?;
        self.visualization_documents_dirty = true;
        self.visualization_documents_modified.insert(document_id);
        self.visualization_documents_unstamped.insert(document_id);
        Ok(())
    }

    /// Stamp every result document changed since the last stamp as
    /// changed at `unix_ms`, and forget the times of the documents the
    /// project no longer holds.
    pub fn stamp_visualization_document_changes(&mut self, unix_ms: u64) {
        for document_id in self.visualization_documents_unstamped.drain() {
            self.visualization_documents_changed_at
                .insert(document_id.as_uuid(), unix_ms);
        }
        let held = self
            .visualization_documents
            .iter()
            .map(|document| document.id().as_uuid())
            .collect::<HashSet<_>>();
        self.visualization_documents_changed_at
            .retain(|document_id, _| held.contains(document_id));
    }

    /// When the result document `document_id` last changed, in milliseconds
    /// since the Unix epoch, if its change was stamped.
    #[must_use]
    pub fn visualization_document_changed_at(&self, document_id: ResultDocumentId) -> Option<u64> {
        self.visualization_documents_changed_at
            .get(&document_id.as_uuid())
            .copied()
    }

    #[must_use]
    pub fn visualization_document(
        &self,
        document_id: ResultDocumentId,
    ) -> Option<&rspice_results::visualization_document::VisualizationDocument> {
        self.visualization_documents
            .iter()
            .find(|document| document.id() == document_id)
    }

    /// Apply a revision-checked transaction to one project-owned result
    /// document and mark the result-document catalog dirty only after the
    /// document commits successfully.
    pub fn transact_visualization_document(
        &mut self,
        document_id: ResultDocumentId,
        expected_revision: rspice_vocabulary::product::ObjectRevision,
        edits: Vec<rspice_results::visualization_document::DocumentEdit>,
    ) -> Result<
        rspice_results::visualization_document::VisualizationTransactionReceipt,
        VisualizationDocumentPersistenceError,
    > {
        let document = self
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or(VisualizationDocumentPersistenceError::NotFound { document_id })?;
        let receipt = document
            .transact(expected_revision, edits)
            .map_err(|error| VisualizationDocumentPersistenceError::Transaction {
                document_id,
                message: error.to_string(),
            })?;
        self.visualization_documents_dirty = true;
        self.visualization_documents_modified.insert(document_id);
        self.visualization_documents_unstamped.insert(document_id);
        Ok(receipt)
    }

    /// [`Self::transact_visualization_document`], returning with the receipt
    /// the step that undoes the transaction.
    pub fn transact_visualization_document_with_undo(
        &mut self,
        document_id: ResultDocumentId,
        expected_revision: rspice_vocabulary::product::ObjectRevision,
        edits: Vec<rspice_results::visualization_document::DocumentEdit>,
    ) -> Result<
        (
            rspice_results::visualization_document::VisualizationTransactionReceipt,
            rspice_results::visualization_document::ViewStep,
        ),
        VisualizationDocumentPersistenceError,
    > {
        let document = self
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or(VisualizationDocumentPersistenceError::NotFound { document_id })?;
        let committed = document
            .transact_with_undo(expected_revision, edits)
            .map_err(|error| VisualizationDocumentPersistenceError::Transaction {
                document_id,
                message: error.to_string(),
            })?;
        self.visualization_documents_dirty = true;
        self.visualization_documents_modified.insert(document_id);
        self.visualization_documents_unstamped.insert(document_id);
        Ok(committed)
    }

    /// Drop one result document from the project: a run's working view once
    /// nothing can draw it again. Whether there was one to drop.
    pub fn remove_visualization_document(&mut self, document_id: ResultDocumentId) -> bool {
        let before = self.visualization_documents.len();
        self.visualization_documents
            .retain(|document| document.id() != document_id);
        let removed = self.visualization_documents.len() != before;
        if removed {
            self.visualization_documents_dirty = true;
            self.visualization_documents_modified.remove(&document_id);
        }
        removed
    }

    /// Take one undo or redo step of a result document's view, as a
    /// revision-checked step like [`Self::transact_visualization_document`],
    /// returning the step that reverses it.
    pub fn take_visualization_document_step(
        &mut self,
        document_id: ResultDocumentId,
        expected_revision: rspice_vocabulary::product::ObjectRevision,
        step: rspice_results::visualization_document::ViewStep,
    ) -> Result<
        (
            rspice_results::visualization_document::VisualizationTransactionReceipt,
            rspice_results::visualization_document::ViewStep,
        ),
        VisualizationDocumentPersistenceError,
    > {
        let document = self
            .visualization_documents
            .iter_mut()
            .find(|document| document.id() == document_id)
            .ok_or(VisualizationDocumentPersistenceError::NotFound { document_id })?;
        let taken = document
            .take_step(expected_revision, step)
            .map_err(|error| VisualizationDocumentPersistenceError::Transaction {
                document_id,
                message: error.to_string(),
            })?;
        self.visualization_documents_dirty = true;
        self.visualization_documents_modified.insert(document_id);
        self.visualization_documents_unstamped.insert(document_id);
        Ok(taken)
    }

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
        validate_hardcopy_source_set_catalog(&self.hardcopy_source_sets).map_err(|error| {
            SimulationConfigurationError::InvalidHardcopySourceSetCatalog {
                message: error.to_string(),
            }
        })?;
        self.hardcopy_receipts.validate().map_err(|error| {
            SimulationConfigurationError::InvalidHardcopyReceiptLedger {
                message: error.to_string(),
            }
        })?;
        self.validate_physical_layout_documents()
            .map_err(
                |message| SimulationConfigurationError::InvalidPhysicalLayoutCatalog { message },
            )?;
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
        let mut visualization_document_ids = std::collections::HashSet::new();
        let mut visualization_document_titles = std::collections::HashMap::<String, usize>::new();
        for (index, document) in self.visualization_documents.iter().enumerate() {
            document.content_digest().map_err(|error| {
                SimulationConfigurationError::InvalidVisualizationDocument {
                    index,
                    message: error.to_string(),
                }
            })?;
            if !visualization_document_ids.insert(document.id()) {
                return Err(
                    SimulationConfigurationError::DuplicateVisualizationDocumentIdentity {
                        document_id: document.id(),
                    },
                );
            }
            let folded_title = document.title().to_lowercase();
            if let Some(first_index) = visualization_document_titles.insert(folded_title, index) {
                return Err(
                    SimulationConfigurationError::DuplicateVisualizationDocumentTitle {
                        title: document.title().to_owned(),
                        first_index,
                        index,
                    },
                );
            }
        }
        self.project_sources.validate().map_err(|error| {
            SimulationConfigurationError::InvalidProjectSourceRegistry {
                message: error.to_string(),
            }
        })?;
        let mut deck_ids = HashSet::new();
        if self.decks.iter().any(|deck| !deck_ids.insert(deck.id)) {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "two decks share one identity".to_owned(),
                },
            );
        }

        let mut plan_ids = HashMap::<SimulationPlanId, usize>::new();
        let mut variable_ids = HashMap::<DesignVariableId, SimulationPlanId>::new();
        let mut output_ids = HashMap::<SavedOutputId, SimulationPlanId>::new();
        let mut capture_group_ids = HashMap::<CaptureGroupId, SimulationPlanId>::new();
        let mut specification_ids = HashMap::<SpecificationId, SimulationPlanId>::new();
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

            capture_group::validate_plan_groups(
                plan_id,
                &record.payload.capture_groups,
                &mut capture_group_ids,
            )
            .map_err(|source| SimulationConfigurationError::CaptureGroup { plan_id, source })?;

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

            record
                .payload
                .specification_policy
                .validate()
                .map_err(
                    |message| SimulationConfigurationError::InvalidSpecificationPolicy {
                        plan_id,
                        message,
                    },
                )?;
            if !record.payload.specification_definitions.is_empty() {
                if record.payload.specification_definitions.len() != record.payload.specs.len() {
                    return Err(
                        SimulationConfigurationError::InvalidSpecificationDefinition {
                            plan_id,
                            index: record.payload.specification_definitions.len(),
                            message: format!(
                                "governed definition count {} does not match scalar projection count {}",
                                record.payload.specification_definitions.len(),
                                record.payload.specs.len()
                            ),
                        },
                    );
                }
                let mut requirement_keys = HashMap::<String, usize>::new();
                let mut governed_measurements = HashMap::<String, usize>::new();
                for (index, definition) in
                    record.payload.specification_definitions.iter().enumerate()
                {
                    definition.validate().map_err(|message| {
                        SimulationConfigurationError::InvalidSpecificationDefinition {
                            plan_id,
                            index,
                            message,
                        }
                    })?;
                    if let Some(first_plan_id) = specification_ids.insert(definition.id, plan_id) {
                        return Err(
                            SimulationConfigurationError::DuplicateSpecificationIdentity {
                                id: definition.id,
                                first_plan_id,
                                plan_id,
                            },
                        );
                    }
                    let requirement_key = definition.requirement_key.to_ascii_lowercase();
                    if let Some(first_index) = requirement_keys.insert(requirement_key, index) {
                        return Err(
                            SimulationConfigurationError::DuplicateSpecificationRequirementKey {
                                plan_id,
                                index,
                                first_index,
                            },
                        );
                    }
                    let measurement = definition.measurement.to_ascii_lowercase();
                    if let Some(first_index) = governed_measurements.insert(measurement, index) {
                        return Err(SimulationConfigurationError::DuplicateSpecification {
                            plan_id,
                            index,
                            first_index,
                        });
                    }
                    if definition.projected_entry() != record.payload.specs[index] {
                        return Err(
                            SimulationConfigurationError::InvalidSpecificationDefinition {
                                plan_id,
                                index,
                                message: "governed definition disagrees with its scalar execution projection"
                                    .to_owned(),
                            },
                        );
                    }
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
}
