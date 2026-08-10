//! Reference-annotation policy, previews, immutable journals, and object authority.

use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
// The dialog offers these as "By device family", "By sheet", "By hierarchy",
// and the saved policy spells them the same way. The shared word is the
// product's, not an accident of naming.
#[allow(clippy::enum_variant_names)]
pub enum AnnotationPrefixAllocation {
    #[default]
    ByDeviceFamily,
    BySheet,
    ByHierarchy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ImportedReferencePolicy {
    #[default]
    PreserveWithSourceMap,
    NormalizeAfterReview,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnnotationCollisionPolicy {
    #[default]
    PreviewAndBlock,
    AllocateNextFreeRange,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceDesignatorBehavior {
    #[default]
    StableAcrossVariants,
    RenumberSelectedScope,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DefaultAnnotationScope {
    #[default]
    WholeProject,
    CurrentHierarchy,
    CurrentSheet,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackannotationPolicy {
    #[default]
    GenerateReviewedMapping,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AnnotationRangeScope {
    Project,
    Sheet { sheet_id: SheetId },
    Hierarchy { path: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationReservedRange {
    pub scope: AnnotationRangeScope,
    pub prefixes: Vec<String>,
    pub first: u32,
    pub last: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationPolicyDefinition {
    #[serde(default)]
    pub reference_designators: ReferenceDesignatorBehavior,
    #[serde(default)]
    pub default_scope: DefaultAnnotationScope,
    pub prefix_allocation: AnnotationPrefixAllocation,
    pub reserved_ranges: Vec<AnnotationReservedRange>,
    pub imported_ids: ImportedReferencePolicy,
    pub collision_policy: AnnotationCollisionPolicy,
    #[serde(default)]
    pub backannotation: BackannotationPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationPolicy {
    pub(super) revision: u64,
    pub(super) semantic_digest: ContentDigest,
    pub(super) definition: AnnotationPolicyDefinition,
}

impl Default for AnnotationPolicy {
    fn default() -> Self {
        let definition = AnnotationPolicyDefinition::default();
        Self {
            revision: 1,
            semantic_digest: digest_infallible("rspice-annotation-policy-semantic/v1", &definition),
            definition,
        }
    }
}

impl AnnotationPolicy {
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    #[must_use]
    pub const fn definition(&self) -> &AnnotationPolicyDefinition {
        &self.definition
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_nonzero_revision(
            self.revision,
            "annotation policy",
            "project policy".to_owned(),
        )?;
        validate_annotation_policy_definition(&self.definition)?;
        require_digest(
            self.semantic_digest,
            digest("rspice-annotation-policy-semantic/v1", &self.definition)?,
            "annotation policy",
            "project policy".to_owned(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenumberOrder {
    HierarchyThenCoordinates,
    SheetThenCoordinates,
    ConnectivityOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectedReferencePolicy {
    RetainLockedAndExternalIds,
    IncludeAfterReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum RenumberScope {
    WholeProject,
    CurrentHierarchy { path: String },
    CurrentSheet { sheet_id: SheetId },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationPosition {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationObject {
    pub object: SchematicObjectKey,
    pub current_reference: String,
    pub device_family: String,
    pub sheet_id: Option<SheetId>,
    pub hierarchy_path: String,
    pub position: AnnotationPosition,
    pub connectivity_order: Option<u64>,
    pub locked: bool,
    pub external: bool,
    pub imported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenumberRequest {
    pub scope: RenumberScope,
    pub order: RenumberOrder,
    pub protected_references: ProtectedReferencePolicy,
    pub protected_reviewed: bool,
    pub objects: Vec<AnnotationObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationMapping {
    pub old_reference: String,
    pub new_reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenumberPreview {
    pub policy_revision: u64,
    pub policy_digest: ContentDigest,
    pub request_digest: ContentDigest,
    pub mappings: BTreeMap<SchematicObjectKey, AnnotationMapping>,
    pub semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationJournalEntry {
    id: AnnotationJournalId,
    sequence: u64,
    policy_revision: u64,
    policy_digest: ContentDigest,
    request_digest: ContentDigest,
    mappings: BTreeMap<SchematicObjectKey, AnnotationMapping>,
    semantic_digest: ContentDigest,
}

impl AnnotationJournalEntry {
    #[must_use]
    pub const fn id(&self) -> AnnotationJournalId {
        self.id
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn policy_revision(&self) -> u64 {
        self.policy_revision
    }

    #[must_use]
    pub const fn policy_digest(&self) -> ContentDigest {
        self.policy_digest
    }

    #[must_use]
    pub fn mappings(&self) -> &BTreeMap<SchematicObjectKey, AnnotationMapping> {
        &self.mappings
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "annotation journal entry")?;
        require_nonzero_revision(
            self.sequence,
            "annotation journal entry",
            self.id.to_string(),
        )?;
        require_nonzero_revision(
            self.policy_revision,
            "annotation policy",
            self.id.to_string(),
        )?;
        require_limit(
            "annotation mappings",
            self.mappings.len(),
            MAX_ANNOTATION_MAPPINGS_PER_ENTRY,
        )?;
        validate_annotation_mappings(&self.mappings)?;
        require_digest(
            self.semantic_digest,
            digest(
                "rspice-annotation-journal-entry-semantic/v1",
                &AnnotationJournalMaterial::from(self),
            )?,
            "annotation journal entry",
            self.id.to_string(),
        )
    }
}

#[derive(Serialize)]
struct AnnotationJournalMaterial<'a> {
    id: AnnotationJournalId,
    sequence: u64,
    policy_revision: u64,
    policy_digest: ContentDigest,
    request_digest: ContentDigest,
    mappings: &'a BTreeMap<SchematicObjectKey, AnnotationMapping>,
}

/// Mutable ownership authority layered over the immutable annotation journal.
///
/// Journal keys retain the identity that was reviewed at commit time. Cell
/// lifecycle operations update this separate authority so a rename can point
/// that evidence at the renamed object and a deletion can make it explicitly
/// non-effective without rewriting historical receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AnnotationObjectAuthority {
    Redirect { target: SchematicObjectKey },
    Tombstone,
}

impl<'a> From<&'a AnnotationJournalEntry> for AnnotationJournalMaterial<'a> {
    fn from(entry: &'a AnnotationJournalEntry) -> Self {
        Self {
            id: entry.id,
            sequence: entry.sequence,
            policy_revision: entry.policy_revision,
            policy_digest: entry.policy_digest,
            request_digest: entry.request_digest,
            mappings: &entry.mappings,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnnotationState {
    pub(super) schema_version: u16,
    pub(super) policy: AnnotationPolicy,
    pub(super) journal: Vec<AnnotationJournalEntry>,
    pub(super) object_authorities: BTreeMap<SchematicObjectKey, AnnotationObjectAuthority>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnnotationStateWire {
    schema_version: u16,
    policy: AnnotationPolicy,
    #[serde(default)]
    journal: Vec<AnnotationJournalEntry>,
    #[serde(default)]
    object_authorities: BTreeMap<SchematicObjectKey, AnnotationObjectAuthority>,
}

impl<'de> Deserialize<'de> for AnnotationState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AnnotationStateWire::deserialize(deserializer)?;
        let value = Self {
            schema_version: wire.schema_version,
            policy: wire.policy,
            journal: wire.journal,
            object_authorities: wire.object_authorities,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Default for AnnotationState {
    fn default() -> Self {
        Self {
            schema_version: ANNOTATION_STATE_SCHEMA_VERSION,
            policy: AnnotationPolicy::default(),
            journal: Vec::new(),
            object_authorities: BTreeMap::new(),
        }
    }
}

impl AnnotationState {
    #[must_use]
    pub const fn policy(&self) -> &AnnotationPolicy {
        &self.policy
    }

    #[must_use]
    pub fn journal(&self) -> &[AnnotationJournalEntry] {
        &self.journal
    }

    #[must_use]
    pub const fn object_authorities(
        &self,
    ) -> &BTreeMap<SchematicObjectKey, AnnotationObjectAuthority> {
        &self.object_authorities
    }

    /// Fold immutable journal entries into the current effective annotation
    /// map. Later reviewed transactions supersede only the scoped objects they
    /// contain; mappings for every other sheet/cell remain effective.
    #[must_use]
    pub fn effective_mappings(&self) -> BTreeMap<SchematicObjectKey, AnnotationMapping> {
        let authorities = self
            .resolved_object_authorities()
            .expect("validated annotation authority is acyclic");
        let mut effective = BTreeMap::new();
        for entry in &self.journal {
            for (object, mapping) in &entry.mappings {
                if let Some(object) = resolved_authority_for(&authorities, object) {
                    effective.insert(object, mapping.clone());
                }
            }
        }
        effective
    }

    pub fn effective_mapping_for(
        &self,
        cell_view_key: &str,
        object_id: u64,
    ) -> Result<Option<&AnnotationMapping>, DesignManagementError> {
        let key = SchematicObjectKey::new(cell_view_key, object_id)?;
        let authorities = self.resolved_object_authorities()?;
        for entry in self.journal.iter().rev() {
            for (journal_key, mapping) in entry.mappings.iter().rev() {
                if resolved_authority_for(&authorities, journal_key).as_ref() == Some(&key) {
                    return Ok(Some(mapping));
                }
            }
        }
        Ok(None)
    }

    pub(super) fn remap_object_owners(
        &mut self,
        source_library: &str,
        source_cell: &str,
        destination_library: &str,
        destination_cell: &str,
    ) -> Result<usize, DesignManagementError> {
        let effective = self.effective_mappings();
        let mut redirects = BTreeMap::new();
        let mut redirect_targets = BTreeSet::new();
        for object in effective.keys() {
            let Some(new_object) = object.remap_cell_owner(
                source_library,
                source_cell,
                destination_library,
                destination_cell,
            )?
            else {
                continue;
            };
            if effective.contains_key(&new_object) || !redirect_targets.insert(new_object.clone()) {
                return Err(DesignManagementError::DuplicateScopedSchematicObject(
                    new_object,
                ));
            }
            redirects.insert(
                object.clone(),
                AnnotationObjectAuthority::Redirect { target: new_object },
            );
        }
        if redirects.is_empty() {
            return Ok(0);
        }
        require_limit(
            "annotation object authorities",
            self.object_authorities.len() + redirects.len(),
            MAX_ANNOTATION_OBJECT_AUTHORITIES,
        )?;
        let mut candidate = self.clone();
        let count = redirects.len();
        candidate.object_authorities.extend(redirects);
        candidate.validate()?;
        *self = candidate;
        Ok(count)
    }

    pub(super) fn tombstone_objects(
        &mut self,
        predicate: impl Fn(&SchematicObjectKey) -> bool,
    ) -> Result<usize, DesignManagementError> {
        let objects = self
            .effective_mappings()
            .into_keys()
            .filter(predicate)
            .collect::<Vec<_>>();
        if objects.is_empty() {
            return Ok(0);
        }
        require_limit(
            "annotation object authorities",
            self.object_authorities.len() + objects.len(),
            MAX_ANNOTATION_OBJECT_AUTHORITIES,
        )?;
        let mut candidate = self.clone();
        for object in &objects {
            candidate
                .object_authorities
                .insert(object.clone(), AnnotationObjectAuthority::Tombstone);
        }
        candidate.validate()?;
        *self = candidate;
        Ok(objects.len())
    }

    fn resolved_object_authorities(
        &self,
    ) -> Result<BTreeMap<SchematicObjectKey, Option<SchematicObjectKey>>, DesignManagementError>
    {
        let mut resolved = BTreeMap::<SchematicObjectKey, Option<SchematicObjectKey>>::new();
        for start in self.object_authorities.keys() {
            if resolved.contains_key(start) {
                continue;
            }
            let mut current = start.clone();
            let mut path = Vec::new();
            let mut visited = HashSet::new();
            let outcome = loop {
                if let Some(cached) = resolved.get(&current) {
                    break cached.clone();
                }
                if !visited.insert(current.clone()) {
                    return Err(DesignManagementError::AnnotationAuthorityCycle(current));
                }
                path.push(current.clone());
                match self.object_authorities.get(&current) {
                    Some(AnnotationObjectAuthority::Redirect { target }) => {
                        current = target.clone();
                    }
                    Some(AnnotationObjectAuthority::Tombstone) => break None,
                    None => break Some(current),
                }
            };
            for object in path {
                resolved.insert(object, outcome.clone());
            }
        }
        Ok(resolved)
    }

    fn authority_reaches(&self, source: &SchematicObjectKey, target: &SchematicObjectKey) -> bool {
        let mut current = source;
        let mut visited = HashSet::new();
        loop {
            if current == target {
                return true;
            }
            if !visited.insert(current) {
                return false;
            }
            match self.object_authorities.get(current) {
                Some(AnnotationObjectAuthority::Redirect { target }) => current = target,
                Some(AnnotationObjectAuthority::Tombstone) | None => return false,
            }
        }
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.schema_version != ANNOTATION_STATE_SCHEMA_VERSION {
            return Err(DesignManagementError::UnsupportedSchema {
                domain: "annotation state",
                actual: self.schema_version,
            });
        }
        self.policy.validate()?;
        require_limit(
            "annotation journal entries",
            self.journal.len(),
            MAX_ANNOTATION_JOURNAL_ENTRIES,
        )?;
        let mut ids = HashSet::with_capacity(self.journal.len());
        for (index, entry) in self.journal.iter().enumerate() {
            entry.validate()?;
            if !ids.insert(entry.id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "annotation journal entry",
                    identity: entry.id.to_string(),
                });
            }
            let expected = u64::try_from(index + 1)
                .map_err(|_| DesignManagementError::NumericRange("annotation sequence"))?;
            if entry.sequence != expected {
                return Err(DesignManagementError::InvalidAnnotationSequence {
                    expected,
                    actual: entry.sequence,
                });
            }
        }
        require_limit(
            "annotation object authorities",
            self.object_authorities.len(),
            MAX_ANNOTATION_OBJECT_AUTHORITIES,
        )?;
        for (object, authority) in &self.object_authorities {
            object.validate()?;
            if let AnnotationObjectAuthority::Redirect { target } = authority {
                target.validate()?;
                if object == target {
                    return Err(DesignManagementError::AnnotationAuthorityCycle(
                        object.clone(),
                    ));
                }
            }
        }
        let resolved_authorities = self.resolved_object_authorities()?;
        let journal_objects = self
            .journal
            .iter()
            .flat_map(|entry| entry.mappings.keys())
            .collect::<BTreeSet<_>>();
        let mut resolved_sources = BTreeMap::<SchematicObjectKey, &SchematicObjectKey>::new();
        for source in journal_objects {
            let Some(target) = resolved_authority_for(&resolved_authorities, source) else {
                continue;
            };
            if let Some(previous) = resolved_sources.insert(target.clone(), source)
                && previous != source
                && !self.authority_reaches(previous, source)
                && !self.authority_reaches(source, previous)
            {
                return Err(DesignManagementError::AnnotationAuthorityConflation {
                    target,
                    first: previous.clone(),
                    second: source.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn update_policy(
        &mut self,
        expected_revision: u64,
        definition: AnnotationPolicyDefinition,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_revision,
            self.policy.revision,
            "annotation policy",
            "project policy".to_owned(),
        )?;
        let definition = normalize_annotation_policy_definition(definition);
        validate_annotation_policy_definition(&definition)?;
        if definition == self.policy.definition {
            return Err(DesignManagementError::NoChanges("annotation policy"));
        }
        let revision = next_revision(
            self.policy.revision,
            "annotation policy",
            "project policy".to_owned(),
        )?;
        let mut candidate = self.clone();
        candidate.policy = AnnotationPolicy {
            revision,
            semantic_digest: digest("rspice-annotation-policy-semantic/v1", &definition)?,
            definition,
        };
        candidate.validate()?;
        *self = candidate;
        Ok(revision)
    }

    pub fn preview_renumbering(
        &self,
        request: &RenumberRequest,
    ) -> Result<RenumberPreview, DesignManagementError> {
        self.validate()?;
        validate_renumber_request(request)?;
        if let Some(object) = request
            .objects
            .iter()
            .map(|entry| &entry.object)
            .find(|object| self.object_authorities.contains_key(*object))
        {
            return Err(DesignManagementError::InactiveAnnotationObjectAuthority(
                object.clone(),
            ));
        }
        let request_digest = digest("rspice-renumber-request-semantic/v1", request)?;
        let mut selected = request
            .objects
            .iter()
            .filter(|object| object_in_scope(object, &request.scope))
            .cloned()
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Err(DesignManagementError::EmptyRenumberScope);
        }
        sort_annotation_objects(&mut selected, request.order);

        let selected_ids = selected
            .iter()
            .map(|object| object.object.clone())
            .collect::<HashSet<_>>();
        let mut occupied = request
            .objects
            .iter()
            .filter(|object| !selected_ids.contains(&object.object))
            .map(|object| case_fold(&object.current_reference))
            .collect::<HashSet<_>>();
        let mut mappings = BTreeMap::new();
        for object in selected {
            let protected = object.locked || object.external;
            if protected
                && request.protected_references
                    == ProtectedReferencePolicy::RetainLockedAndExternalIds
            {
                occupied.insert(case_fold(&object.current_reference));
                continue;
            }
            if protected
                && request.protected_references == ProtectedReferencePolicy::IncludeAfterReview
                && !request.protected_reviewed
            {
                return Err(DesignManagementError::ProtectedReferenceReviewRequired(
                    object.object.clone(),
                ));
            }
            if object.imported
                && self.policy.definition.imported_ids
                    == ImportedReferencePolicy::PreserveWithSourceMap
            {
                occupied.insert(case_fold(&object.current_reference));
                continue;
            }
            let prefix = annotation_prefix(&object, self.policy.definition.prefix_allocation)?;
            let ranges = matching_annotation_ranges(
                &self.policy.definition.reserved_ranges,
                &object,
                &prefix,
            );
            let new_reference = allocate_reference(&prefix, &ranges, &occupied)?;
            occupied.insert(case_fold(&new_reference));
            mappings.insert(
                object.object,
                AnnotationMapping {
                    old_reference: object.current_reference,
                    new_reference,
                },
            );
        }
        if mappings.is_empty() {
            return Err(DesignManagementError::NoChanges("reference annotation"));
        }
        #[derive(Serialize)]
        struct Material<'a> {
            policy_revision: u64,
            policy_digest: ContentDigest,
            request_digest: ContentDigest,
            mappings: &'a BTreeMap<SchematicObjectKey, AnnotationMapping>,
        }
        let semantic_digest = digest(
            "rspice-renumber-preview-semantic/v1",
            &Material {
                policy_revision: self.policy.revision,
                policy_digest: self.policy.semantic_digest,
                request_digest,
                mappings: &mappings,
            },
        )?;
        Ok(RenumberPreview {
            policy_revision: self.policy.revision,
            policy_digest: self.policy.semantic_digest,
            request_digest,
            mappings,
            semantic_digest,
        })
    }

    pub fn commit_renumbering(
        &mut self,
        preview: &RenumberPreview,
        current_request: &RenumberRequest,
    ) -> Result<AnnotationJournalId, DesignManagementError> {
        self.validate()?;
        if preview.policy_revision != self.policy.revision
            || preview.policy_digest != self.policy.semantic_digest
        {
            return Err(DesignManagementError::StaleRenumberPreview);
        }
        let current = self.preview_renumbering(current_request)?;
        if &current != preview {
            return Err(DesignManagementError::StaleRenumberPreview);
        }
        require_limit(
            "annotation journal entries",
            self.journal.len() + 1,
            MAX_ANNOTATION_JOURNAL_ENTRIES,
        )?;
        let sequence = u64::try_from(self.journal.len() + 1)
            .map_err(|_| DesignManagementError::NumericRange("annotation sequence"))?;
        let id = AnnotationJournalId::new();
        let mut entry = AnnotationJournalEntry {
            id,
            sequence,
            policy_revision: preview.policy_revision,
            policy_digest: preview.policy_digest,
            request_digest: preview.request_digest,
            mappings: preview.mappings.clone(),
            semantic_digest: empty_digest(),
        };
        entry.semantic_digest = digest(
            "rspice-annotation-journal-entry-semantic/v1",
            &AnnotationJournalMaterial::from(&entry),
        )?;
        let mut candidate = self.clone();
        candidate.journal.push(entry);
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }
}

fn resolved_authority_for(
    authorities: &BTreeMap<SchematicObjectKey, Option<SchematicObjectKey>>,
    object: &SchematicObjectKey,
) -> Option<SchematicObjectKey> {
    authorities
        .get(object)
        .cloned()
        .unwrap_or_else(|| Some(object.clone()))
}
