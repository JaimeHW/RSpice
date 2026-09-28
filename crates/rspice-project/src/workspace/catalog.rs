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

impl ProjectWorkspace {
    /// Assign deterministic stable identities to legacy top decks that predate
    /// the multi-deck catalog. Migration uses only already-persisted project,
    /// document, and logical-path identities, so loading identical bytes on a
    /// second machine produces the same project state.
    pub fn migrate_owned_netlist_deck_ids(&mut self) {
        let namespace = self.project.id().as_uuid();
        if let (Some(descriptor), Some(document)) =
            (&mut self.netlist_descriptor, &self.netlist_document)
            && descriptor.deck_id.is_nil()
        {
            let name = format!(
                "rspice/top-deck/v1/{}/{}",
                document.id().as_uuid(),
                descriptor.artifact_name
            );
            descriptor.deck_id = Uuid::new_v5(&namespace, name.as_bytes());
        }
        for (index, deck) in self.retained_netlist_decks.iter_mut().enumerate() {
            if deck.descriptor.deck_id.is_nil() {
                let name = format!(
                    "rspice/retained-top-deck/v1/{index}/{}/{}",
                    deck.document.id().as_uuid(),
                    deck.descriptor.artifact_name
                );
                deck.descriptor.deck_id = Uuid::new_v5(&namespace, name.as_bytes());
            }
        }
    }

    pub fn physical_layout_documents(
        &self,
    ) -> &BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument> {
        &self.physical_layout_documents
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
        let mut candidate = self.physical_layout_documents.clone();
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

    pub fn prepare_copy_physical_layout_cell_documents(
        &self,
        source_library: &str,
        source_cell: &str,
        target_library: &str,
        target_cell: &str,
    ) -> Result<PreparedPhysicalLayoutCatalog, rspice_design::physical_layout::LayoutDocumentError>
    {
        let copies = self
            .physical_layout_documents
            .values()
            .filter(|document| {
                document.owner().library == source_library && document.owner().cell == source_cell
            })
            .map(|document| {
                document.copy_for_cell(source_library, source_cell, target_library, target_cell)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut candidate = self.physical_layout_documents.clone();
        for document in &copies {
            let key = document.owner().key();
            if candidate.contains_key(&key) {
                return Err(
                    rspice_design::physical_layout::LayoutDocumentError::DuplicateObject {
                        kind: "physical-layout document",
                        id: key,
                    },
                );
            }
            candidate.insert(key, document.clone());
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

    pub fn commit_prepared_physical_layout_catalog(
        &mut self,
        prepared: PreparedPhysicalLayoutCatalog,
    ) {
        self.physical_layout_documents = prepared.documents;
    }

    pub fn validate_physical_layout_documents(&self) -> Result<(), String> {
        validate_physical_layout_document_catalog(&self.physical_layout_documents)
            .map_err(|error| error.to_string())
    }

    #[must_use]
    pub fn pdk_callback_receipts(
        &self,
    ) -> &[rspice_model_library::pdk::callback::ProjectPdkCallbackReceipt] {
        &self.pdk_callback_receipts
    }

    /// Replace the project-owned callback ledger while applying an already
    /// validated lifecycle snapshot. The candidate is validated against this
    /// workspace's project identity and revision before any live state changes.
    pub fn replace_pdk_callback_receipts_for_lifecycle(
        &mut self,
        receipts: Vec<rspice_model_library::pdk::callback::ProjectPdkCallbackReceipt>,
    ) -> Result<(), String> {
        let mut candidate = self.clone();
        candidate.pdk_callback_receipts = receipts;
        candidate.validate_pdk_callback_receipts()?;
        self.pdk_callback_receipts = candidate.pdk_callback_receipts;
        Ok(())
    }

    pub fn validate_pdk_callback_receipts(&self) -> Result<(), String> {
        use rspice_model_library::pdk::callback::MAX_PROJECT_PDK_CALLBACK_RECEIPTS;

        if self.pdk_callback_receipts.len() > MAX_PROJECT_PDK_CALLBACK_RECEIPTS {
            return Err(format!(
                "receipt count exceeds {MAX_PROJECT_PDK_CALLBACK_RECEIPTS}"
            ));
        }
        let mut previous_digest = None;
        let mut previous_to_revision = None;
        for (index, receipt) in self.pdk_callback_receipts.iter().enumerate() {
            receipt
                .validate()
                .map_err(|error| format!("receipt[{index}] is invalid: {error}"))?;
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| "receipt sequence overflow".to_owned())?;
            if receipt.sequence != expected_sequence {
                return Err(format!(
                    "receipt[{index}] has sequence {}, expected {expected_sequence}",
                    receipt.sequence
                ));
            }
            if receipt.project_id != self.project.id() {
                return Err(format!(
                    "receipt[{index}] belongs to project {}, not {}",
                    receipt.project_id,
                    self.project.id()
                ));
            }
            if receipt.previous_receipt_digest != previous_digest {
                return Err(format!(
                    "receipt[{index}] does not continue the callback receipt hash chain"
                ));
            }
            if previous_to_revision.is_some_and(|previous: ObjectRevision| {
                receipt.from_project_revision.get() < previous.get()
            }) {
                return Err(format!(
                    "receipt[{index}] predates the previous callback project revision"
                ));
            }
            if receipt.to_project_revision.get() > self.project.revision().get() {
                return Err(format!(
                    "receipt[{index}] claims future project revision {}",
                    receipt.to_project_revision.get()
                ));
            }
            previous_digest = Some(receipt.receipt_digest);
            previous_to_revision = Some(receipt.to_project_revision);
        }
        Ok(())
    }

    pub fn commit_pdk_callback_execution(
        &mut self,
        plan_id: SimulationPlanId,
        plan_revision: ObjectRevision,
        authority: &rspice_model_library::pdk::PdkAdministrativeAuthority,
        reason: &str,
        input: rspice_model_library::pdk::callback::PdkCallbackExecutionInput,
        execution: rspice_model_library::pdk::callback::PdkCallbackExecutionReceipt,
    ) -> Result<
        rspice_model_library::pdk::callback::ProjectPdkCallbackReceipt,
        rspice_model_library::pdk::callback::PdkCallbackError,
    > {
        use rspice_model_library::pdk::callback::{
            MAX_PROJECT_PDK_CALLBACK_RECEIPTS, PdkCallbackError, ProjectPdkCallbackReceipt,
        };

        self.validate_pdk_callback_receipts()
            .map_err(PdkCallbackError::ProjectTransaction)?;
        if self.pdk_callback_receipts.len() >= MAX_PROJECT_PDK_CALLBACK_RECEIPTS {
            return Err(PdkCallbackError::ProjectTransaction(format!(
                "callback receipt ledger is limited to {MAX_PROJECT_PDK_CALLBACK_RECEIPTS} entries"
            )));
        }
        let from_project_revision = self.project.revision();
        let to_project_revision = self
            .project
            .next_revision()
            .map_err(|error| PdkCallbackError::ProjectTransaction(error.to_string()))?;
        let sequence = u64::try_from(self.pdk_callback_receipts.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                PdkCallbackError::ProjectTransaction(
                    "callback receipt sequence is exhausted".to_owned(),
                )
            })?;
        let receipt = ProjectPdkCallbackReceipt {
            schema_version:
                rspice_model_library::pdk::callback::PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION,
            sequence,
            project_id: self.project.id(),
            from_project_revision,
            to_project_revision,
            plan_id,
            plan_revision,
            actor_id: authority.actor_id.trim().to_owned(),
            authority_id: authority.authority_id.trim().to_owned(),
            reason: reason.trim().to_owned(),
            input,
            execution,
            previous_receipt_digest: self
                .pdk_callback_receipts
                .last()
                .map(|receipt| receipt.receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        }
        .with_validated_digest()?;

        let mut candidate = self.clone();
        let committed_revision = candidate
            .project
            .advance_revision()
            .map_err(|error| PdkCallbackError::ProjectTransaction(error.to_string()))?;
        if committed_revision != to_project_revision {
            return Err(PdkCallbackError::ProjectTransaction(
                "preflighted project revision changed before callback receipt commit".to_owned(),
            ));
        }
        candidate.pdk_callback_receipts.push(receipt.clone());
        candidate
            .validate_pdk_callback_receipts()
            .map_err(PdkCallbackError::ProjectTransaction)?;
        *self = candidate;
        Ok(receipt)
    }

    /// Commit a new visualization document into the project authority.
    ///
    /// Validation and every duplicate check run before the vector changes, so
    /// a failed dialog commit cannot leave a partial document or dirty bit.
    pub fn insert_visualization_document(
        &mut self,
        document: rspice_results::visualization_document::VisualizationDocument,
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
        self.visualization_documents.push(document);
        self.visualization_documents_dirty = true;
        Ok(document_id)
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
        expected_revision: rspice_app_types::product::ObjectRevision,
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
        Ok(receipt)
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
        self.validate_pdk_callback_receipts().map_err(|message| {
            SimulationConfigurationError::InvalidPdkCallbackReceiptLedger { message }
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
        if let Some(document) = &self.netlist_document {
            if document.ownership() == rspice_design::netlist_document::DocumentOwnership::Generated
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
            if descriptor.deck_id.is_nil() {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned top-deck identity cannot be nil".to_owned(),
                    },
                );
            }
            validate_owned_netlist_artifact_path(&descriptor.artifact_name).map_err(|message| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
            })?;
            let declared_dialect = descriptor
                .imported_dialect
                .unwrap_or(NetlistSourceDialect::RSpice);
            let expected_profile = declared_dialect.execution_profile();
            if descriptor.execution_profile.is_some()
                && descriptor.execution_profile != expected_profile
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: format!(
                            "owned netlist dialect {} does not match its recorded execution profile",
                            declared_dialect.label()
                        ),
                    },
                );
            }
            if declared_dialect.requires_compatibility_review() {
                let reviewed = descriptor.compatibility_reviewed
                    && descriptor.execution_profile == expected_profile
                    && expected_profile.is_some();
                let quarantined =
                    !descriptor.compatibility_reviewed && descriptor.execution_profile.is_none();
                if !reviewed && !quarantined {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned non-canonical netlist dialect {} has neither an exact reviewed executable profile nor a fail-closed quarantine",
                                declared_dialect.label()
                            ),
                        },
                    );
                }
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
            if descriptor.revision_history.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned source revision history exceeds its bounded entry limit"
                            .to_owned(),
                    },
                );
            }
            let mut previous_revision = 0_u64;
            let mut retained_bytes = 0_usize;
            for snapshot in &descriptor.revision_history {
                snapshot.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
                })?;
                if snapshot.document_revision <= previous_revision
                    || snapshot.document_revision > document.revision().get()
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message:
                                "owned source revision history is not strictly revision ordered"
                                    .to_owned(),
                        },
                    );
                }
                retained_bytes = retained_bytes
                    .checked_add(snapshot.retained_bytes())
                    .ok_or_else(|| {
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned source revision history size overflowed".to_owned(),
                        }
                    })?;
                previous_revision = snapshot.document_revision;
            }
            if retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned source revision history exceeds its bounded byte limit"
                            .to_owned(),
                    },
                );
            }
            if descriptor.owned_includes.len()
                > rspice_design::project_sources::MAX_PROJECT_SOURCE_FILES
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "owned include catalog exceeds the project file limit".to_owned(),
                    },
                );
            }
            let mut include_ids = HashSet::new();
            let mut include_identities = HashSet::new();
            for include in &descriptor.owned_includes {
                include.validate().map_err(|message| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
                })?;
                if !include_ids.insert(include.document_id)
                    || !include_identities.insert(include.logical_identity.as_str())
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: "owned include identities must be unique".to_owned(),
                        },
                    );
                }
                let dependency = document
                    .dependencies()
                    .iter()
                    .find(|dependency| {
                        dependency.locator().logical_identity() == include.logical_identity
                    })
                    .ok_or_else(|| {
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned include '{}' is absent from the canonical dependency closure",
                                include.logical_identity
                            ),
                        }
                    })?;
                let source = dependency.source().ok_or_else(|| {
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: format!(
                            "owned include '{}' has no retained source bytes",
                            include.logical_identity
                        ),
                    }
                })?;
                if include.content_digest != rspice_design::netlist_document::content_digest(source)
                {
                    return Err(
                        SimulationConfigurationError::InvalidNetlistDocumentProjection {
                            message: format!(
                                "owned include '{}' digest does not identify its retained bytes",
                                include.logical_identity
                            ),
                        },
                    );
                }
            }
        } else if self.netlist_descriptor.is_some() {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "owned-artifact descriptor has no canonical document".to_owned(),
                },
            );
        }

        if self.retained_netlist_decks.len()
            > rspice_design::project_sources::MAX_PROJECT_SOURCE_FILES
        {
            return Err(
                SimulationConfigurationError::InvalidNetlistDocumentProjection {
                    message: "retained top-deck catalog exceeds the project file limit".to_owned(),
                },
            );
        }
        let mut deck_ids = HashSet::new();
        let mut deck_paths = HashSet::new();
        if let Some(descriptor) = &self.netlist_descriptor {
            deck_ids.insert(descriptor.deck_id);
            deck_paths.insert(descriptor.artifact_name.to_ascii_lowercase());
        }
        for deck in &self.retained_netlist_decks {
            validate_owned_netlist_projection(
                &deck.document,
                &deck.descriptor,
                deck.document.source(),
            )
            .map_err(|message| {
                SimulationConfigurationError::InvalidNetlistDocumentProjection { message }
            })?;
            if !deck_ids.insert(deck.descriptor.deck_id)
                || !deck_paths.insert(deck.descriptor.artifact_name.to_ascii_lowercase())
            {
                return Err(
                    SimulationConfigurationError::InvalidNetlistDocumentProjection {
                        message: "top-deck identities and logical paths must be unique".to_owned(),
                    },
                );
            }
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
