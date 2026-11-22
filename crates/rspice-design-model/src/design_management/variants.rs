//! Assembly-variant definitions, resolution, comparison, and catalog authority.

use super::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariantInheritance {
    #[default]
    OverrideChangedObjectsOnly,
    IndependentReviewedCopy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariantQualificationPlan {
    #[default]
    InvalidateAffectedTests,
    CreateEmptyQualificationPlan,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariantQualificationState {
    #[default]
    RequiresQualification,
    Current,
    ReviewRequired,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelEquivalencePolicy {
    #[default]
    RequireQualifiedReplacement,
    AllowReviewCandidate,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariantResultCompatibility {
    #[default]
    ExactVariantIdentityRequired,
    AllowReviewedOverlay,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblyVariantSettings {
    pub inheritance: VariantInheritance,
    pub model_equivalence: ModelEquivalencePolicy,
    pub result_compatibility: VariantResultCompatibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantParentRef {
    pub id: AssemblyVariantId,
    pub revision: u64,
    pub semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentSubstitution {
    pub library: String,
    pub cell: String,
    pub view: String,
    pub value_override: Option<String>,
    pub model_section: Option<String>,
    pub port_equivalence_digest: Option<ContentDigest>,
    pub qualification: VariantQualificationState,
}

impl ComponentSubstitution {
    /// The digest a substitute records for the terminal mapping it was
    /// chosen with: each terminal of the part and the replacement terminal
    /// it lands on, in the part's order.
    pub fn port_equivalence_of(
        mapping: &[(String, String)],
    ) -> Result<ContentDigest, DesignManagementError> {
        digest("rspice-variant-port-equivalence/v1", &mapping)
    }
}

/// What one variant does to one part of the base design.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum VariantObjectOverride {
    /// The part is replaced by another library cell.
    Substitute { replacement: ComponentSubstitution },
    /// The part is left off the board and out of the netlist. The reason is
    /// optional and may be empty.
    DoNotPopulate { approval_reference: String },
    /// The authored part, fitted. With no value it restores a part an
    /// ancestor variant leaves off or replaces; with a value it changes the
    /// part's value and nothing else, so its binding, its pins and the cell
    /// the base design names for it all stay as authored.
    Fitted { value_override: Option<String> },
}

impl VariantObjectOverride {
    /// Whether this override leaves the part exactly as the base design has
    /// it: an explicit Fitted with no value, which only matters for undoing
    /// what an ancestor variant does.
    #[must_use]
    pub const fn is_plain_fitted(&self) -> bool {
        matches!(
            self,
            Self::Fitted {
                value_override: None
            }
        )
    }

    /// The value this override gives the part, where it gives one.
    #[must_use]
    pub fn value_override(&self) -> Option<&str> {
        match self {
            Self::Substitute { replacement } => replacement.value_override.as_deref(),
            Self::Fitted { value_override } => value_override.as_deref(),
            Self::DoNotPopulate { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblyVariantDefinition {
    pub name: String,
    pub parent: Option<VariantParentRef>,
    pub inheritance: VariantInheritance,
    pub qualification_plan: VariantQualificationPlan,
    pub overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyVariantDraft {
    pub name: String,
    pub parent_id: Option<AssemblyVariantId>,
    pub inheritance: VariantInheritance,
    pub qualification_plan: VariantQualificationPlan,
    pub overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblyVariant {
    pub(super) id: AssemblyVariantId,
    pub(super) revision: u64,
    pub(super) semantic_digest: ContentDigest,
    pub(super) definition: AssemblyVariantDefinition,
}

impl AssemblyVariant {
    #[must_use]
    pub const fn id(&self) -> AssemblyVariantId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    #[must_use]
    pub const fn definition(&self) -> &AssemblyVariantDefinition {
        &self.definition
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.definition.name
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "assembly variant")?;
        require_nonzero_revision(self.revision, "assembly variant", self.id.to_string())?;
        validate_variant_definition(&self.definition)?;
        require_digest(
            self.semantic_digest,
            digest("rspice-assembly-variant-semantic/v1", &self.definition)?,
            "assembly variant",
            self.id.to_string(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedAssemblyVariant {
    pub variant_id: AssemblyVariantId,
    pub variant_revision: u64,
    pub lineage: Vec<VariantParentRef>,
    pub overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
    pub semantic_digest: ContentDigest,
}

impl ResolvedAssemblyVariant {
    pub fn override_for(
        &self,
        cell_view_key: &str,
        object_id: u64,
    ) -> Result<Option<&VariantObjectOverride>, DesignManagementError> {
        let key = SchematicObjectKey::new(cell_view_key, object_id)?;
        Ok(self.overrides.get(&key))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VariantDifferenceKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantDifference {
    pub object: SchematicObjectKey,
    pub kind: VariantDifferenceKind,
    pub reference: Option<VariantObjectOverride>,
    pub comparison: Option<VariantObjectOverride>,
}

/// Two choices compared part by part. A side with no id is the base design,
/// which overrides nothing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantComparison {
    pub reference_id: Option<AssemblyVariantId>,
    pub comparison_id: Option<AssemblyVariantId>,
    pub reference_digest: Option<ContentDigest>,
    pub comparison_digest: Option<ContentDigest>,
    pub differences: Vec<VariantDifference>,
    pub semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblyVariantCatalog {
    pub(super) schema_version: u16,
    #[serde(default)]
    pub(super) settings: AssemblyVariantSettings,
    pub(super) variants: Vec<AssemblyVariant>,
    pub(super) active_variant_id: Option<AssemblyVariantId>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AssemblyVariantCatalogWire {
    schema_version: u16,
    #[serde(default)]
    settings: AssemblyVariantSettings,
    #[serde(default)]
    variants: Vec<AssemblyVariant>,
    #[serde(default)]
    active_variant_id: Option<AssemblyVariantId>,
}

impl<'de> Deserialize<'de> for AssemblyVariantCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = AssemblyVariantCatalogWire::deserialize(deserializer)?;
        let value = Self {
            schema_version: wire.schema_version,
            settings: wire.settings,
            variants: wire.variants,
            active_variant_id: wire.active_variant_id,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Default for AssemblyVariantCatalog {
    fn default() -> Self {
        Self {
            schema_version: VARIANT_CATALOG_SCHEMA_VERSION,
            settings: AssemblyVariantSettings::default(),
            variants: Vec::new(),
            active_variant_id: None,
        }
    }
}

impl AssemblyVariantCatalog {
    #[must_use]
    pub const fn settings(&self) -> &AssemblyVariantSettings {
        &self.settings
    }

    #[must_use]
    pub fn variants(&self) -> &[AssemblyVariant] {
        &self.variants
    }

    #[must_use]
    pub const fn active_variant_id(&self) -> Option<AssemblyVariantId> {
        self.active_variant_id
    }

    #[must_use]
    pub fn find(&self, id: AssemblyVariantId) -> Option<&AssemblyVariant> {
        self.variants.iter().find(|variant| variant.id == id)
    }

    #[must_use]
    pub fn active(&self) -> Option<&AssemblyVariant> {
        self.active_variant_id.and_then(|id| self.find(id))
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.schema_version != VARIANT_CATALOG_SCHEMA_VERSION {
            return Err(DesignManagementError::UnsupportedSchema {
                domain: "assembly variant catalog",
                actual: self.schema_version,
            });
        }
        require_limit(
            "assembly variants",
            self.variants.len(),
            MAX_ASSEMBLY_VARIANTS,
        )?;
        let mut ids = HashSet::with_capacity(self.variants.len());
        let mut names = HashSet::with_capacity(self.variants.len());
        for variant in &self.variants {
            variant.validate()?;
            if !ids.insert(variant.id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "assembly variant",
                    identity: variant.id.to_string(),
                });
            }
            if !names.insert(case_fold(variant.name())) {
                return Err(DesignManagementError::DuplicateName {
                    domain: "assembly variant",
                    name: variant.name().to_owned(),
                });
            }
        }
        // No active variant is the base design, a choice like any other.
        if let Some(id) = self.active_variant_id
            && !ids.contains(&id)
        {
            return Err(DesignManagementError::MissingReference {
                domain: "active assembly variant",
                identity: id.to_string(),
            });
        }
        for variant in &self.variants {
            if let Some(parent) = &variant.definition.parent {
                let current = self.find(parent.id).ok_or_else(|| {
                    DesignManagementError::MissingReference {
                        domain: "parent assembly variant",
                        identity: parent.id.to_string(),
                    }
                })?;
                if current.id == variant.id {
                    return Err(DesignManagementError::VariantParentCycle(variant.id));
                }
                if current.revision != parent.revision
                    || current.semantic_digest != parent.semantic_digest
                {
                    return Err(DesignManagementError::StaleVariantParent {
                        child: variant.id,
                        parent: parent.id,
                    });
                }
            }
            self.validate_parent_chain(variant.id)?;
        }
        Ok(())
    }

    pub fn set_settings(
        &mut self,
        settings: AssemblyVariantSettings,
    ) -> Result<(), DesignManagementError> {
        self.validate()?;
        if self.settings == settings {
            return Err(DesignManagementError::NoChanges(
                "assembly variant settings",
            ));
        }
        let mut candidate = self.clone();
        candidate.settings = settings;
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn create(
        &mut self,
        draft: AssemblyVariantDraft,
    ) -> Result<AssemblyVariantId, DesignManagementError> {
        self.validate()?;
        require_limit(
            "assembly variants",
            self.variants.len() + 1,
            MAX_ASSEMBLY_VARIANTS,
        )?;
        let definition = self.materialize_draft(draft)?;
        validate_variant_definition(&definition)?;
        if self
            .variants
            .iter()
            .any(|variant| case_fold(variant.name()) == case_fold(&definition.name))
        {
            return Err(DesignManagementError::DuplicateName {
                domain: "assembly variant",
                name: definition.name,
            });
        }
        let id = AssemblyVariantId::new();
        let variant = AssemblyVariant {
            id,
            revision: 1,
            semantic_digest: digest("rspice-assembly-variant-semantic/v1", &definition)?,
            definition,
        };
        let mut candidate = self.clone();
        candidate.variants.push(variant);
        candidate.active_variant_id.get_or_insert(id);
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    pub fn update(
        &mut self,
        id: AssemblyVariantId,
        expected_revision: u64,
        draft: AssemblyVariantDraft,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        let current = self
            .find(id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            })?;
        require_revision(
            expected_revision,
            current.revision,
            "assembly variant",
            id.to_string(),
        )?;
        let definition = self.materialize_draft(draft)?;
        validate_variant_definition(&definition)?;
        if self.variants.iter().any(|variant| {
            variant.id != id && case_fold(variant.name()) == case_fold(&definition.name)
        }) {
            return Err(DesignManagementError::DuplicateName {
                domain: "assembly variant",
                name: definition.name,
            });
        }
        if current.definition == definition {
            return Err(DesignManagementError::NoChanges("assembly variant"));
        }
        let mut candidate = self.clone();
        let revision = candidate.publish_definition(id, definition)?;
        candidate.validate()?;
        *self = candidate;
        Ok(revision)
    }

    /// Change what variant `id` does to some parts, as one revision of it.
    ///
    /// Each edit names a part and its new override; `None` removes the
    /// variant's own override, so the part reads as the parent variant, or
    /// the base design, has it. Variants derived from this one are re-pinned
    /// to its new revision in the same step (see
    /// [`Self::publish_definition`]).
    pub fn set_overrides(
        &mut self,
        id: AssemblyVariantId,
        expected_revision: u64,
        edits: BTreeMap<SchematicObjectKey, Option<VariantObjectOverride>>,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        if edits.is_empty() {
            return Err(DesignManagementError::EmptySelection);
        }
        let current = self
            .find(id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            })?;
        require_revision(
            expected_revision,
            current.revision,
            "assembly variant",
            id.to_string(),
        )?;
        let mut definition = current.definition.clone();
        for (object, change) in edits {
            object.validate()?;
            match change {
                Some(change) => {
                    definition.overrides.insert(object, change);
                }
                None => {
                    definition.overrides.remove(&object);
                }
            }
        }
        let definition = normalize_variant_definition(definition);
        validate_variant_definition(&definition)?;
        if current.definition == definition {
            return Err(DesignManagementError::NoChanges("assembly variant"));
        }
        let mut candidate = self.clone();
        let revision = candidate.publish_definition(id, definition)?;
        candidate.validate()?;
        *self = candidate;
        Ok(revision)
    }

    /// Give variant `id` its new `definition` at the next revision, then
    /// re-pin every variant derived from it, parents before children.
    ///
    /// A child keeps its own overrides. Where it substitutes a part whose
    /// state in its parent this step changed, its substitute turns
    /// [`VariantQualificationState::ReviewRequired`]: it was chosen against
    /// what the parent did before. Returns the new revision of `id`. The
    /// caller validates the catalog afterwards.
    fn publish_definition(
        &mut self,
        id: AssemblyVariantId,
        definition: AssemblyVariantDefinition,
    ) -> Result<u64, DesignManagementError> {
        let before = self
            .variants
            .iter()
            .map(|variant| (variant.id, self.layered_overrides(variant.id)))
            .collect::<BTreeMap<_, _>>();
        let revision = self.republish(id, definition)?;
        let mut seen = HashSet::from([id]);
        let mut parents = VecDeque::from([id]);
        while let Some(parent_id) = parents.pop_front() {
            let parent = self.find(parent_id).expect("variant was validated");
            let pin = VariantParentRef {
                id: parent_id,
                revision: parent.revision,
                semantic_digest: parent.semantic_digest,
            };
            let after = self.layered_overrides(parent_id);
            let empty = BTreeMap::new();
            let prior = before.get(&parent_id).unwrap_or(&empty);
            let touched = prior
                .keys()
                .chain(after.keys())
                .filter(|object| prior.get(*object) != after.get(*object))
                .cloned()
                .collect::<BTreeSet<_>>();
            let children = self
                .variants
                .iter()
                .filter(|variant| {
                    variant
                        .definition
                        .parent
                        .as_ref()
                        .is_some_and(|parent| parent.id == parent_id)
                })
                .map(|variant| variant.id)
                .collect::<Vec<_>>();
            for child in children {
                // A cycle is refused by the validation that follows; here it
                // must only not loop.
                if !seen.insert(child) {
                    continue;
                }
                let mut definition = self
                    .find(child)
                    .expect("child variant was listed")
                    .definition
                    .clone();
                definition.parent = Some(pin.clone());
                for (object, change) in &mut definition.overrides {
                    if let VariantObjectOverride::Substitute { replacement } = change
                        && touched.contains(object)
                    {
                        replacement.qualification = VariantQualificationState::ReviewRequired;
                    }
                }
                self.republish(child, definition)?;
                parents.push_back(child);
            }
        }
        Ok(revision)
    }

    /// Store `definition` as variant `id`'s next revision.
    fn republish(
        &mut self,
        id: AssemblyVariantId,
        definition: AssemblyVariantDefinition,
    ) -> Result<u64, DesignManagementError> {
        let target = self
            .variants
            .iter_mut()
            .find(|variant| variant.id == id)
            .expect("variant was validated");
        target.revision = next_revision(target.revision, "assembly variant", id.to_string())?;
        target.semantic_digest = digest("rspice-assembly-variant-semantic/v1", &definition)?;
        target.definition = definition;
        Ok(target.revision)
    }

    /// What variant `id` does to each part once its ancestors are layered
    /// beneath it, leaving out the parts it leaves as the base design has
    /// them. Reads the chain as stored, without validating it first.
    fn layered_overrides(
        &self,
        id: AssemblyVariantId,
    ) -> BTreeMap<SchematicObjectKey, VariantObjectOverride> {
        let mut chain = Vec::new();
        let mut seen = HashSet::new();
        let mut cursor = self.find(id);
        while let Some(variant) = cursor {
            if !seen.insert(variant.id) {
                break;
            }
            chain.push(variant);
            cursor = variant
                .definition
                .parent
                .as_ref()
                .and_then(|parent| self.find(parent.id));
        }
        let mut overrides = BTreeMap::new();
        for variant in chain.into_iter().rev() {
            if variant.definition.inheritance == VariantInheritance::IndependentReviewedCopy {
                overrides.clear();
            }
            overrides.extend(variant.definition.overrides.clone());
        }
        // A plain Fitted has done its work once it has hidden what an
        // ancestor did: the part is as the base design has it.
        overrides.retain(|_, change: &mut VariantObjectOverride| !change.is_plain_fitted());
        overrides
    }

    pub fn remove(
        &mut self,
        id: AssemblyVariantId,
        expected_revision: u64,
    ) -> Result<(), DesignManagementError> {
        self.validate()?;
        let current = self
            .find(id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            })?;
        require_revision(
            expected_revision,
            current.revision,
            "assembly variant",
            id.to_string(),
        )?;
        if self.variants.iter().any(|variant| {
            variant
                .definition
                .parent
                .as_ref()
                .is_some_and(|parent| parent.id == id)
        }) {
            return Err(DesignManagementError::VariantHasDependents(id));
        }
        if self.active_variant_id == Some(id) {
            return Err(DesignManagementError::ActiveRemoval("assembly variant"));
        }
        let mut candidate = self.clone();
        candidate.variants.retain(|variant| variant.id != id);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn set_active(&mut self, id: AssemblyVariantId) -> Result<(), DesignManagementError> {
        self.choose(Some(id))
    }

    /// Make `id` the variant the design is netlisted, drawn and inspected
    /// as, or with `None` the base design.
    pub fn choose(&mut self, id: Option<AssemblyVariantId>) -> Result<(), DesignManagementError> {
        self.validate()?;
        if let Some(id) = id
            && self.find(id).is_none()
        {
            return Err(DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            });
        }
        self.active_variant_id = id;
        Ok(())
    }

    pub fn resolve(
        &self,
        id: AssemblyVariantId,
    ) -> Result<ResolvedAssemblyVariant, DesignManagementError> {
        self.validate()?;
        let leaf = self
            .find(id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            })?;
        let mut chain = Vec::new();
        let mut cursor = Some(leaf);
        while let Some(variant) = cursor {
            chain.push(variant);
            cursor = variant
                .definition
                .parent
                .as_ref()
                .map(|parent| self.find(parent.id).expect("catalog was validated"));
        }
        chain.reverse();
        let lineage = chain
            .iter()
            .map(|variant| VariantParentRef {
                id: variant.id,
                revision: variant.revision,
                semantic_digest: variant.semantic_digest,
            })
            .collect::<Vec<_>>();
        let overrides = self.layered_overrides(id);
        #[derive(Serialize)]
        struct Material<'a> {
            variant_id: AssemblyVariantId,
            variant_revision: u64,
            lineage: &'a [VariantParentRef],
            overrides: &'a BTreeMap<SchematicObjectKey, VariantObjectOverride>,
        }
        let semantic_digest = digest(
            "rspice-resolved-assembly-variant-semantic/v1",
            &Material {
                variant_id: leaf.id,
                variant_revision: leaf.revision,
                lineage: &lineage,
                overrides: &overrides,
            },
        )?;
        Ok(ResolvedAssemblyVariant {
            variant_id: leaf.id,
            variant_revision: leaf.revision,
            lineage,
            overrides,
            semantic_digest,
        })
    }

    /// Compare two choices part by part; `None` on either side is the base
    /// design, which overrides nothing.
    pub fn compare(
        &self,
        reference_id: Option<AssemblyVariantId>,
        comparison_id: Option<AssemblyVariantId>,
    ) -> Result<VariantComparison, DesignManagementError> {
        if reference_id == comparison_id {
            return Err(DesignManagementError::SameVariantComparison);
        }
        self.validate()?;
        let reference = reference_id.map(|id| self.resolve(id)).transpose()?;
        let comparison = comparison_id.map(|id| self.resolve(id)).transpose()?;
        let empty = BTreeMap::new();
        let reference_overrides = reference.as_ref().map_or(&empty, |side| &side.overrides);
        let comparison_overrides = comparison.as_ref().map_or(&empty, |side| &side.overrides);
        let objects = reference_overrides
            .keys()
            .chain(comparison_overrides.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        let differences = objects
            .into_iter()
            .filter_map(|object| {
                let left = reference_overrides.get(&object);
                let right = comparison_overrides.get(&object);
                if left == right {
                    return None;
                }
                let kind = match (left, right) {
                    (None, Some(_)) => VariantDifferenceKind::Added,
                    (Some(_), None) => VariantDifferenceKind::Removed,
                    (Some(_), Some(_)) => VariantDifferenceKind::Changed,
                    (None, None) => unreachable!(),
                };
                Some(VariantDifference {
                    object,
                    kind,
                    reference: left.cloned(),
                    comparison: right.cloned(),
                })
            })
            .collect::<Vec<_>>();
        #[derive(Serialize)]
        struct Material<'a> {
            reference_id: Option<AssemblyVariantId>,
            comparison_id: Option<AssemblyVariantId>,
            reference_digest: Option<ContentDigest>,
            comparison_digest: Option<ContentDigest>,
            differences: &'a [VariantDifference],
        }
        let reference_digest = reference.as_ref().map(|side| side.semantic_digest);
        let comparison_digest = comparison.as_ref().map(|side| side.semantic_digest);
        let semantic_digest = digest(
            "rspice-assembly-variant-comparison-semantic/v1",
            &Material {
                reference_id,
                comparison_id,
                reference_digest,
                comparison_digest,
                differences: &differences,
            },
        )?;
        Ok(VariantComparison {
            reference_id,
            comparison_id,
            reference_digest,
            comparison_digest,
            differences,
            semantic_digest,
        })
    }

    fn materialize_draft(
        &self,
        draft: AssemblyVariantDraft,
    ) -> Result<AssemblyVariantDefinition, DesignManagementError> {
        let mut overrides = if draft.inheritance == VariantInheritance::IndependentReviewedCopy {
            match draft.parent_id {
                Some(id) => self.resolve(id)?.overrides,
                None => BTreeMap::new(),
            }
        } else {
            BTreeMap::new()
        };
        overrides.extend(draft.overrides);
        let parent = match draft.parent_id {
            Some(id) => {
                let parent =
                    self.find(id)
                        .ok_or_else(|| DesignManagementError::MissingReference {
                            domain: "parent assembly variant",
                            identity: id.to_string(),
                        })?;
                Some(VariantParentRef {
                    id,
                    revision: parent.revision,
                    semantic_digest: parent.semantic_digest,
                })
            }
            None => None,
        };
        Ok(normalize_variant_definition(AssemblyVariantDefinition {
            name: draft.name,
            parent,
            inheritance: draft.inheritance,
            qualification_plan: draft.qualification_plan,
            overrides,
        }))
    }

    fn validate_parent_chain(&self, start: AssemblyVariantId) -> Result<(), DesignManagementError> {
        let mut seen = HashSet::new();
        let mut cursor = Some(start);
        while let Some(id) = cursor {
            if !seen.insert(id) {
                return Err(DesignManagementError::VariantParentCycle(id));
            }
            cursor = self
                .find(id)
                .and_then(|variant| variant.definition.parent.as_ref().map(|parent| parent.id));
        }
        Ok(())
    }
}
