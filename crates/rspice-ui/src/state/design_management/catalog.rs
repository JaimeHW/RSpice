//! Project-level design-management aggregate and transactional lifecycle operations.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DesignManagementCatalog {
    schema_version: u16,
    revision: u64,
    sheet_catalogs: BTreeMap<String, SheetCatalog>,
    variants: AssemblyVariantCatalog,
    annotation: AnnotationState,
    hierarchy_settings: HierarchyManagementSettings,
    hierarchy_audits: Vec<HierarchyAuditReceipt>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DesignManagementCatalogWire {
    schema_version: u16,
    revision: u64,
    #[serde(default)]
    sheet_catalogs: BTreeMap<String, SheetCatalog>,
    #[serde(default)]
    variants: AssemblyVariantCatalog,
    #[serde(default)]
    annotation: AnnotationState,
    #[serde(default)]
    hierarchy_settings: HierarchyManagementSettings,
    #[serde(default)]
    hierarchy_audits: Vec<HierarchyAuditReceipt>,
}

impl<'de> Deserialize<'de> for DesignManagementCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DesignManagementCatalogWire::deserialize(deserializer)?;
        let value = Self {
            schema_version: wire.schema_version,
            revision: wire.revision,
            sheet_catalogs: wire.sheet_catalogs,
            variants: wire.variants,
            annotation: wire.annotation,
            hierarchy_settings: wire.hierarchy_settings,
            hierarchy_audits: wire.hierarchy_audits,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Default for DesignManagementCatalog {
    fn default() -> Self {
        Self {
            schema_version: DESIGN_MANAGEMENT_SCHEMA_VERSION,
            revision: 1,
            sheet_catalogs: BTreeMap::new(),
            variants: AssemblyVariantCatalog::default(),
            annotation: AnnotationState::default(),
            hierarchy_settings: HierarchyManagementSettings::default(),
            hierarchy_audits: Vec::new(),
        }
    }
}

impl DesignManagementCatalog {
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sheet_catalogs.is_empty()
            && self.variants.variants.is_empty()
            && self.annotation.journal.is_empty()
            && self.annotation.object_authorities.is_empty()
            && self.hierarchy_audits.is_empty()
    }

    #[must_use]
    pub fn sheet_catalogs(&self) -> &BTreeMap<String, SheetCatalog> {
        &self.sheet_catalogs
    }

    #[must_use]
    pub fn sheet_catalog(&self, cell_view_key: &str) -> Option<&SheetCatalog> {
        canonical_cell_view_key(cell_view_key)
            .ok()
            .and_then(|key| self.sheet_catalogs.get(&key))
    }

    pub fn sheet_catalog_mut(&mut self, cell_view_key: &str) -> Option<&mut SheetCatalog> {
        canonical_cell_view_key(cell_view_key)
            .ok()
            .and_then(|key| self.sheet_catalogs.get_mut(&key))
    }

    pub fn ensure_sheet_catalog(
        &mut self,
        cell_view_key: &str,
    ) -> Result<&mut SheetCatalog, DesignManagementError> {
        let key = canonical_cell_view_key(cell_view_key)?;
        Ok(self.sheet_catalogs.entry(key).or_default())
    }

    /// Bootstrap one real sheet for a legacy schematic buffer. An empty
    /// catalog remains the canonical representation until a sheet workflow is
    /// first used, so loading an older project never mutates it implicitly.
    pub fn bootstrap_for_cell_view(
        &mut self,
        cell_view_key: &str,
        sheet_name: impl Into<String>,
        object_ids: impl IntoIterator<Item = u64>,
    ) -> Result<SheetId, DesignManagementError> {
        let key = canonical_cell_view_key(cell_view_key)?;
        if self.sheet_catalogs.contains_key(&key) {
            return Err(DesignManagementError::AlreadyBootstrapped(key));
        }
        let ids = unique_object_ids(object_ids)?;
        let mut catalog = SheetCatalog::default();
        let id = catalog.create_sheet(
            SheetDefinition {
                name: sheet_name.into(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(1),
            },
            None,
        )?;
        if !ids.is_empty() {
            catalog.assign_objects(catalog.revision(), id, ids)?;
        }
        let mut candidate = self.clone();
        candidate.sheet_catalogs.insert(key, catalog);
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    /// Copy all sheet catalogs owned by one cell into a newly copied cell.
    /// Sheet and port identities are regenerated project-wide, while
    /// document-local object numbers remain valid because ownership moves to
    /// the new canonical cell/view key. Sheet-scoped annotation ranges are
    /// cloned onto the new sheet identities; variant and annotation object
    /// mappings remain bound to their original scoped object keys.
    pub fn copy_cell_sheet_catalogs(
        &mut self,
        source_library: &str,
        source_cell: &str,
        destination_library: &str,
        destination_cell: &str,
    ) -> Result<DesignManagementCopyReceipt, DesignManagementError> {
        self.validate()?;
        let source_library = canonical_cell_view_segment("library", source_library)?;
        let source_cell = canonical_cell_view_segment("cell", source_cell)?;
        let destination_library = canonical_cell_view_segment("library", destination_library)?;
        let destination_cell = canonical_cell_view_segment("cell", destination_cell)?;
        if source_library == destination_library && source_cell == destination_cell {
            return Err(DesignManagementError::NoChanges(
                "design management cell copy",
            ));
        }

        let sources = self
            .sheet_catalogs
            .iter()
            .filter_map(|(key, catalog)| {
                let [key_library, key_cell, key_view] = cell_view_key_segments(key).ok()?;
                (key_library == source_library && key_cell == source_cell).then(|| {
                    (
                        format!("{destination_library}/{destination_cell}/{key_view}"),
                        catalog.clone(),
                    )
                })
            })
            .collect::<Vec<_>>();
        if sources.is_empty() {
            return Ok(DesignManagementCopyReceipt {
                catalog_revision: self.revision,
                copied_sheet_catalogs: 0,
                sheet_identity_map: BTreeMap::new(),
                port_identity_map: BTreeMap::new(),
            });
        }
        for (target, _) in &sources {
            if self.sheet_catalogs.contains_key(target) {
                return Err(DesignManagementError::SheetCatalogOwnershipCollision(
                    target.clone(),
                ));
            }
        }

        let mut sheet_identity_map = BTreeMap::new();
        let mut port_identity_map = BTreeMap::new();
        for (_, catalog) in &sources {
            for sheet in &catalog.sheets {
                sheet_identity_map.insert(sheet.id, SheetId::new());
            }
            for port in &catalog.cross_sheet_ports {
                port_identity_map.insert(port.id, CrossSheetPortId::new());
            }
        }

        let mut copied_catalogs = Vec::with_capacity(sources.len());
        for (target, source) in &sources {
            let sheets = source
                .sheets
                .iter()
                .map(|sheet| {
                    let id = sheet_identity_map[&sheet.id];
                    Ok(DesignSheet {
                        id,
                        revision: 1,
                        semantic_digest: digest(
                            "rspice-design-sheet-semantic/v1",
                            &sheet.definition,
                        )?,
                        definition: sheet.definition.clone(),
                    })
                })
                .collect::<Result<Vec<_>, DesignManagementError>>()?;
            let object_assignments = source
                .object_assignments
                .iter()
                .map(|(&object_id, sheet_id)| (object_id, sheet_identity_map[sheet_id]))
                .collect::<BTreeMap<_, _>>();
            let cross_sheet_ports = source
                .cross_sheet_ports
                .iter()
                .map(|port| {
                    let mut definition = port.definition.clone();
                    definition.first.sheet_id = sheet_identity_map[&definition.first.sheet_id];
                    definition.second.sheet_id = sheet_identity_map[&definition.second.sheet_id];
                    Ok(CrossSheetPortContract {
                        id: port_identity_map[&port.id],
                        revision: 1,
                        semantic_digest: digest(
                            "rspice-cross-sheet-port-semantic/v1",
                            &definition,
                        )?,
                        definition,
                    })
                })
                .collect::<Result<Vec<_>, DesignManagementError>>()?;
            let copied = SheetCatalog {
                schema_version: SHEET_CATALOG_SCHEMA_VERSION,
                revision: 1,
                settings: source.settings.clone(),
                sheets,
                active_sheet_id: source.active_sheet_id.map(|id| sheet_identity_map[&id]),
                object_assignments,
                cross_sheet_ports,
            };
            copied.validate()?;
            copied_catalogs.push((target.clone(), copied));
        }

        let mut candidate = self.clone();
        for (target, catalog) in copied_catalogs {
            if candidate
                .sheet_catalogs
                .insert(target.clone(), catalog)
                .is_some()
            {
                return Err(DesignManagementError::SheetCatalogOwnershipCollision(
                    target,
                ));
            }
        }
        let copied_ranges = self
            .annotation
            .policy
            .definition
            .reserved_ranges
            .iter()
            .filter_map(|range| match range.scope {
                AnnotationRangeScope::Sheet { sheet_id } => {
                    sheet_identity_map.get(&sheet_id).map(|new_sheet_id| {
                        let mut copied = range.clone();
                        copied.scope = AnnotationRangeScope::Sheet {
                            sheet_id: *new_sheet_id,
                        };
                        copied
                    })
                }
                AnnotationRangeScope::Project | AnnotationRangeScope::Hierarchy { .. } => None,
            })
            .collect::<Vec<_>>();
        if !copied_ranges.is_empty() {
            let mut policy = candidate.annotation.policy.definition.clone();
            policy.reserved_ranges.extend(copied_ranges);
            let policy_revision = candidate.annotation.policy.revision;
            candidate
                .annotation
                .update_policy(policy_revision, policy)?;
        }
        candidate.revision = next_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        let receipt = DesignManagementCopyReceipt {
            catalog_revision: candidate.revision,
            copied_sheet_catalogs: sources.len(),
            sheet_identity_map,
            port_identity_map,
        };
        *self = candidate;
        Ok(receipt)
    }

    /// Atomically remap every owned view of one renamed cell. Stable sheet,
    /// port, object, annotation, and result identities remain unchanged; only
    /// the canonical owning key changes.
    pub fn rename_cell_sheet_catalogs(
        &mut self,
        library: &str,
        old_cell: &str,
        new_cell: &str,
    ) -> Result<DesignManagementOwnershipReceipt, DesignManagementError> {
        self.validate()?;
        let library = canonical_cell_view_segment("library", library)?;
        let old_cell = canonical_cell_view_segment("cell", old_cell)?;
        let new_cell = canonical_cell_view_segment("cell", new_cell)?;
        if old_cell == new_cell {
            return Err(DesignManagementError::NoChanges(
                "design management cell ownership",
            ));
        }
        let matching = self
            .sheet_catalogs
            .keys()
            .filter_map(|key| {
                let [key_library, key_cell, key_view] = cell_view_key_segments(key).ok()?;
                (key_library == library && key_cell == old_cell)
                    .then(|| (key.clone(), format!("{library}/{new_cell}/{key_view}")))
            })
            .collect::<Vec<_>>();
        let old_keys = matching
            .iter()
            .map(|(old, _)| old.clone())
            .collect::<HashSet<_>>();
        for (_, target) in &matching {
            if self.sheet_catalogs.contains_key(target) && !old_keys.contains(target) {
                return Err(DesignManagementError::SheetCatalogOwnershipCollision(
                    target.clone(),
                ));
            }
        }
        let mut candidate = self.clone();
        let mut moved = Vec::with_capacity(matching.len());
        for (old, new) in matching {
            let catalog = candidate
                .sheet_catalogs
                .remove(&old)
                .expect("matching ownership was validated");
            moved.push((new, catalog));
        }
        for (new, catalog) in moved {
            if candidate
                .sheet_catalogs
                .insert(new.clone(), catalog)
                .is_some()
            {
                return Err(DesignManagementError::SheetCatalogOwnershipCollision(new));
            }
        }
        let remapped_variant_objects = remap_variant_object_owners(
            &mut candidate.variants,
            &library,
            &old_cell,
            &library,
            &new_cell,
        )?;
        let remapped_annotation_objects = candidate
            .annotation
            .remap_object_owners(&library, &old_cell, &library, &new_cell)?;
        if old_keys.is_empty() && remapped_variant_objects == 0 && remapped_annotation_objects == 0
        {
            return Ok(DesignManagementOwnershipReceipt {
                catalog_revision: self.revision,
                affected_sheet_catalogs: 0,
                remapped_variant_objects: 0,
                remapped_annotation_objects: 0,
            });
        }
        candidate.revision = next_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        let receipt = DesignManagementOwnershipReceipt {
            catalog_revision: candidate.revision,
            affected_sheet_catalogs: old_keys.len(),
            remapped_variant_objects,
            remapped_annotation_objects,
        };
        *self = candidate;
        Ok(receipt)
    }

    /// Remove the sheet catalog owned by one exact deleted cell/view. The
    /// operation blocks while a live variant override still owns an object or
    /// annotation policy reserves a sheet range. Historical annotation
    /// receipts remain immutable; their effective object authority is
    /// tombstoned as part of the same transaction.
    pub fn remove_sheet_catalog_for_view(
        &mut self,
        cell_view_key: &str,
    ) -> Result<DesignManagementOwnershipReceipt, DesignManagementError> {
        self.validate()?;
        let key = canonical_cell_view_key(cell_view_key)?;
        self.require_no_variant_objects(|object| object.cell_view_key() == key)?;
        if let Some(catalog) = self.sheet_catalogs.get(&key) {
            self.require_sheets_unreferenced(catalog.sheets.iter().map(|sheet| sheet.id))?;
        }
        let mut candidate = self.clone();
        let affected_sheet_catalogs = usize::from(candidate.sheet_catalogs.remove(&key).is_some());
        let remapped_annotation_objects = candidate
            .annotation
            .tombstone_objects(|object| object.cell_view_key() == key)?;
        if affected_sheet_catalogs == 0 && remapped_annotation_objects == 0 {
            return Ok(DesignManagementOwnershipReceipt {
                catalog_revision: self.revision,
                affected_sheet_catalogs: 0,
                remapped_variant_objects: 0,
                remapped_annotation_objects: 0,
            });
        }
        candidate.revision = next_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        let receipt = DesignManagementOwnershipReceipt {
            catalog_revision: candidate.revision,
            affected_sheet_catalogs,
            remapped_variant_objects: 0,
            remapped_annotation_objects,
        };
        *self = candidate;
        Ok(receipt)
    }

    /// Remove every view catalog owned by a deleted cell and tombstone its
    /// effective annotation authority as one transaction. Live scoped variant
    /// overrides block deletion until the user resolves them explicitly.
    pub fn remove_sheet_catalogs_for_cell(
        &mut self,
        library: &str,
        cell: &str,
    ) -> Result<DesignManagementOwnershipReceipt, DesignManagementError> {
        self.validate()?;
        let library = canonical_cell_view_segment("library", library)?;
        let cell = canonical_cell_view_segment("cell", cell)?;
        self.require_no_variant_objects(|object| {
            cell_view_key_segments(object.cell_view_key())
                .is_ok_and(|[key_library, key_cell, _]| key_library == library && key_cell == cell)
        })?;
        let keys = self
            .sheet_catalogs
            .keys()
            .filter(|key| {
                cell_view_key_segments(key).is_ok_and(|[key_library, key_cell, _]| {
                    key_library == library && key_cell == cell
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        let sheet_ids = keys
            .iter()
            .filter_map(|key| self.sheet_catalogs.get(key))
            .flat_map(|catalog| catalog.sheets.iter().map(|sheet| sheet.id));
        self.require_sheets_unreferenced(sheet_ids)?;
        let mut candidate = self.clone();
        for key in &keys {
            candidate.sheet_catalogs.remove(key);
        }
        let remapped_annotation_objects = candidate.annotation.tombstone_objects(|object| {
            cell_view_key_segments(object.cell_view_key())
                .is_ok_and(|[key_library, key_cell, _]| key_library == library && key_cell == cell)
        })?;
        if keys.is_empty() && remapped_annotation_objects == 0 {
            return Ok(DesignManagementOwnershipReceipt {
                catalog_revision: self.revision,
                affected_sheet_catalogs: 0,
                remapped_variant_objects: 0,
                remapped_annotation_objects: 0,
            });
        }
        candidate.revision = next_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        let receipt = DesignManagementOwnershipReceipt {
            catalog_revision: candidate.revision,
            affected_sheet_catalogs: keys.len(),
            remapped_variant_objects: 0,
            remapped_annotation_objects,
        };
        *self = candidate;
        Ok(receipt)
    }

    #[must_use]
    pub fn sheet_for_object_or_active(
        &self,
        cell_view_key: &str,
        object_id: u64,
    ) -> Option<SheetId> {
        let catalog = self.sheet_catalog(cell_view_key)?;
        catalog
            .sheet_for_object(object_id)
            .or(catalog.active_sheet_id)
            .or_else(|| catalog.sheets.first().map(|sheet| sheet.id))
    }

    #[must_use]
    pub const fn variants(&self) -> &AssemblyVariantCatalog {
        &self.variants
    }

    pub fn variants_mut(&mut self) -> &mut AssemblyVariantCatalog {
        &mut self.variants
    }

    #[must_use]
    pub const fn annotation(&self) -> &AnnotationState {
        &self.annotation
    }

    pub fn annotation_mut(&mut self) -> &mut AnnotationState {
        &mut self.annotation
    }

    #[must_use]
    pub const fn hierarchy_settings(&self) -> &HierarchyManagementSettings {
        &self.hierarchy_settings
    }

    #[must_use]
    pub fn hierarchy_audits(&self) -> &[HierarchyAuditReceipt] {
        &self.hierarchy_audits
    }

    pub fn set_hierarchy_settings(
        &mut self,
        settings: HierarchyManagementSettings,
    ) -> Result<(), DesignManagementError> {
        self.validate()?;
        if self.hierarchy_settings == settings {
            return Err(DesignManagementError::NoChanges(
                "hierarchy management settings",
            ));
        }
        let mut candidate = self.clone();
        candidate.hierarchy_settings = settings;
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn run_and_record_hierarchy_audit(
        &mut self,
        request: &HierarchyAuditRequest,
    ) -> Result<HierarchyAuditReceiptId, DesignManagementError> {
        self.validate()?;
        validate_hierarchy_audit_request(request)?;
        require_limit(
            "hierarchy audit receipts",
            self.hierarchy_audits.len() + 1,
            MAX_HIERARCHY_AUDIT_RECEIPTS,
        )?;
        let request_digest = digest("rspice-hierarchy-audit-request-semantic/v1", request)?;
        let findings = evaluate_hierarchy_audit(request)?;
        let sequence = u64::try_from(self.hierarchy_audits.len() + 1)
            .map_err(|_| DesignManagementError::NumericRange("hierarchy audit sequence"))?;
        let id = HierarchyAuditReceiptId::new();
        let mut receipt = HierarchyAuditReceipt {
            id,
            sequence,
            request_digest,
            resolved_subjects: request
                .subjects
                .iter()
                .filter(|subject| subject.resolved_simulation_view.is_some())
                .count(),
            findings,
            semantic_digest: empty_digest(),
        };
        receipt.semantic_digest = digest(
            "rspice-hierarchy-audit-receipt-semantic/v1",
            &HierarchyAuditReceiptMaterial::from(&receipt),
        )?;
        let mut candidate = self.clone();
        candidate.hierarchy_audits.push(receipt);
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    pub fn publish_reviewed_candidate(
        &mut self,
        expected_revision: u64,
        mut candidate: Self,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_revision,
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        if semantic_material(self)? == semantic_material(&candidate)? {
            return Err(DesignManagementError::NoChanges(
                "design management catalog",
            ));
        }
        candidate.revision = next_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        candidate.validate()?;
        let revision = candidate.revision;
        *self = candidate;
        Ok(revision)
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.schema_version != DESIGN_MANAGEMENT_SCHEMA_VERSION {
            return Err(DesignManagementError::UnsupportedSchema {
                domain: "design management catalog",
                actual: self.schema_version,
            });
        }
        require_nonzero_revision(
            self.revision,
            "design management catalog",
            "catalog".to_owned(),
        )?;
        let mut project_sheet_ids = HashSet::new();
        let mut project_port_ids = HashSet::new();
        for (key, catalog) in &self.sheet_catalogs {
            if canonical_cell_view_key(key)? != *key {
                return Err(DesignManagementError::NonCanonicalCellViewKey(key.clone()));
            }
            catalog.validate()?;
            for sheet in &catalog.sheets {
                if !project_sheet_ids.insert(sheet.id) {
                    return Err(DesignManagementError::DuplicateIdentity {
                        domain: "project sheet",
                        identity: sheet.id.to_string(),
                    });
                }
            }
            for port in &catalog.cross_sheet_ports {
                if !project_port_ids.insert(port.id) {
                    return Err(DesignManagementError::DuplicateIdentity {
                        domain: "project cross-sheet port",
                        identity: port.id.to_string(),
                    });
                }
            }
        }
        self.variants.validate()?;
        self.annotation.validate()?;
        for range in &self.annotation.policy.definition.reserved_ranges {
            if let AnnotationRangeScope::Sheet { sheet_id } = range.scope
                && !project_sheet_ids.contains(&sheet_id)
            {
                return Err(DesignManagementError::MissingReference {
                    domain: "annotation range sheet",
                    identity: sheet_id.to_string(),
                });
            }
        }
        require_limit(
            "hierarchy audit receipts",
            self.hierarchy_audits.len(),
            MAX_HIERARCHY_AUDIT_RECEIPTS,
        )?;
        let mut ids = HashSet::with_capacity(self.hierarchy_audits.len());
        for (index, receipt) in self.hierarchy_audits.iter().enumerate() {
            receipt.validate()?;
            if !ids.insert(receipt.id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "hierarchy audit receipt",
                    identity: receipt.id.to_string(),
                });
            }
            let expected = u64::try_from(index + 1)
                .map_err(|_| DesignManagementError::NumericRange("hierarchy audit sequence"))?;
            if receipt.sequence != expected {
                return Err(DesignManagementError::InvalidHierarchyAuditSequence {
                    expected,
                    actual: receipt.sequence,
                });
            }
        }
        Ok(())
    }

    pub fn semantic_digest(&self) -> Result<ContentDigest, DesignManagementError> {
        self.validate()?;
        digest(
            "rspice-design-management-catalog-semantic/v1",
            &semantic_material(self)?,
        )
    }

    fn require_sheets_unreferenced(
        &self,
        sheet_ids: impl IntoIterator<Item = SheetId>,
    ) -> Result<(), DesignManagementError> {
        let sheet_ids = sheet_ids.into_iter().collect::<HashSet<_>>();
        for range in &self.annotation.policy.definition.reserved_ranges {
            if let AnnotationRangeScope::Sheet { sheet_id } = range.scope
                && sheet_ids.contains(&sheet_id)
            {
                return Err(DesignManagementError::SheetCatalogReferenced(sheet_id));
            }
        }
        Ok(())
    }

    fn require_no_variant_objects(
        &self,
        matches: impl Fn(&SchematicObjectKey) -> bool,
    ) -> Result<(), DesignManagementError> {
        for variant in &self.variants.variants {
            if let Some(object) = variant
                .definition
                .overrides
                .keys()
                .find(|object| matches(object))
            {
                return Err(DesignManagementError::LiveVariantObjectReference {
                    variant: variant.id,
                    object: object.clone(),
                });
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Serialize)]
struct DesignManagementSemanticMaterial<'a> {
    sheet_catalogs: &'a BTreeMap<String, SheetCatalog>,
    variants: &'a AssemblyVariantCatalog,
    annotation: &'a AnnotationState,
    hierarchy_settings: &'a HierarchyManagementSettings,
    hierarchy_audits: &'a [HierarchyAuditReceipt],
}

fn semantic_material(
    catalog: &DesignManagementCatalog,
) -> Result<DesignManagementSemanticMaterial<'_>, DesignManagementError> {
    catalog.validate()?;
    Ok(DesignManagementSemanticMaterial {
        sheet_catalogs: &catalog.sheet_catalogs,
        variants: &catalog.variants,
        annotation: &catalog.annotation,
        hierarchy_settings: &catalog.hierarchy_settings,
        hierarchy_audits: &catalog.hierarchy_audits,
    })
}
