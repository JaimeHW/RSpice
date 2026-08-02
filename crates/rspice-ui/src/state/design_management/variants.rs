//! Assembly-variant definitions, resolution, comparison, and catalog authority.

use super::*;

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
pub enum MissingReplacementPolicy {
    #[default]
    Block,
    ExplicitDoNotPopulate,
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
    pub missing_replacement: MissingReplacementPolicy,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum VariantObjectOverride {
    Substitute { replacement: ComponentSubstitution },
    DoNotPopulate { approval_reference: String },
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantComparison {
    pub reference_id: AssemblyVariantId,
    pub comparison_id: AssemblyVariantId,
    pub reference_digest: ContentDigest,
    pub comparison_digest: ContentDigest,
    pub differences: Vec<VariantDifference>,
    pub semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantMatrixEdit {
    pub variant_id: AssemblyVariantId,
    pub expected_revision: u64,
    pub object: SchematicObjectKey,
    pub replacement: Option<ComponentSubstitution>,
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
        match self.active_variant_id {
            Some(id) if !ids.contains(&id) => {
                return Err(DesignManagementError::MissingReference {
                    domain: "active assembly variant",
                    identity: id.to_string(),
                });
            }
            None if !self.variants.is_empty() => {
                return Err(DesignManagementError::ActiveSelectionRequired(
                    "assembly variant",
                ));
            }
            _ => {}
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
        if self.variants.iter().any(|variant| {
            variant
                .definition
                .parent
                .as_ref()
                .is_some_and(|parent| parent.id == id)
        }) {
            return Err(DesignManagementError::VariantHasDependents(id));
        }
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
        let revision = next_revision(current.revision, "assembly variant", id.to_string())?;
        let mut candidate = self.clone();
        let target = candidate
            .variants
            .iter_mut()
            .find(|variant| variant.id == id)
            .expect("variant was validated");
        target.revision = revision;
        target.semantic_digest = digest("rspice-assembly-variant-semantic/v1", &definition)?;
        target.definition = definition;
        candidate.validate()?;
        *self = candidate;
        Ok(revision)
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
        self.validate()?;
        if self.find(id).is_none() {
            return Err(DesignManagementError::MissingReference {
                domain: "assembly variant",
                identity: id.to_string(),
            });
        }
        self.active_variant_id = Some(id);
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
        let mut overrides = BTreeMap::new();
        for variant in chain {
            if variant.definition.inheritance == VariantInheritance::IndependentReviewedCopy {
                overrides.clear();
            }
            overrides.extend(variant.definition.overrides.clone());
        }
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

    pub fn compare(
        &self,
        reference_id: AssemblyVariantId,
        comparison_id: AssemblyVariantId,
    ) -> Result<VariantComparison, DesignManagementError> {
        if reference_id == comparison_id {
            return Err(DesignManagementError::SameVariantComparison);
        }
        let reference = self.resolve(reference_id)?;
        let comparison = self.resolve(comparison_id)?;
        let objects = reference
            .overrides
            .keys()
            .chain(comparison.overrides.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        let differences = objects
            .into_iter()
            .filter_map(|object| {
                let left = reference.overrides.get(&object);
                let right = comparison.overrides.get(&object);
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
            reference_id: AssemblyVariantId,
            comparison_id: AssemblyVariantId,
            reference_digest: ContentDigest,
            comparison_digest: ContentDigest,
            differences: &'a [VariantDifference],
        }
        let semantic_digest = digest(
            "rspice-assembly-variant-comparison-semantic/v1",
            &Material {
                reference_id,
                comparison_id,
                reference_digest: reference.semantic_digest,
                comparison_digest: comparison.semantic_digest,
                differences: &differences,
            },
        )?;
        Ok(VariantComparison {
            reference_id,
            comparison_id,
            reference_digest: reference.semantic_digest,
            comparison_digest: comparison.semantic_digest,
            differences,
            semantic_digest,
        })
    }

    pub fn apply_substitution_matrix(
        &mut self,
        edits: Vec<VariantMatrixEdit>,
        missing_policy: MissingReplacementPolicy,
        equivalence_policy: ModelEquivalencePolicy,
    ) -> Result<Vec<(AssemblyVariantId, u64)>, DesignManagementError> {
        self.validate()?;
        if edits.is_empty() {
            return Err(DesignManagementError::EmptySelection);
        }
        let mut unique = HashSet::with_capacity(edits.len());
        for edit in &edits {
            edit.object.validate()?;
            if !unique.insert((edit.variant_id, edit.object.clone())) {
                return Err(DesignManagementError::DuplicateVariantMatrixCell {
                    variant: edit.variant_id,
                    object: edit.object.clone(),
                });
            }
            let current = self.find(edit.variant_id).ok_or_else(|| {
                DesignManagementError::MissingReference {
                    domain: "assembly variant",
                    identity: edit.variant_id.to_string(),
                }
            })?;
            require_revision(
                edit.expected_revision,
                current.revision,
                "assembly variant",
                edit.variant_id.to_string(),
            )?;
            if self.variants.iter().any(|variant| {
                variant
                    .definition
                    .parent
                    .as_ref()
                    .is_some_and(|parent| parent.id == edit.variant_id)
            }) {
                return Err(DesignManagementError::VariantHasDependents(edit.variant_id));
            }
            if let Some(replacement) = &edit.replacement {
                validate_substitution(replacement)?;
                if equivalence_policy == ModelEquivalencePolicy::RequireQualifiedReplacement
                    && replacement.qualification != VariantQualificationState::Current
                {
                    return Err(DesignManagementError::UnqualifiedReplacement(
                        edit.object.clone(),
                    ));
                }
            } else if missing_policy == MissingReplacementPolicy::Block {
                return Err(DesignManagementError::MissingReplacement(
                    edit.object.clone(),
                ));
            }
        }

        let mut candidate = self.clone();
        let mut touched = BTreeSet::new();
        for edit in edits {
            let target = candidate
                .variants
                .iter_mut()
                .find(|variant| variant.id == edit.variant_id)
                .expect("variant was validated");
            let new_override = match edit.replacement {
                Some(replacement) => VariantObjectOverride::Substitute { replacement },
                None => VariantObjectOverride::DoNotPopulate {
                    approval_reference: "reviewed substitution matrix".to_owned(),
                },
            };
            if target.definition.overrides.get(&edit.object) == Some(&new_override) {
                continue;
            }
            target
                .definition
                .overrides
                .insert(edit.object, new_override);
            touched.insert(edit.variant_id);
        }
        if touched.is_empty() {
            return Err(DesignManagementError::NoChanges(
                "variant substitution matrix",
            ));
        }
        let mut revisions = Vec::with_capacity(touched.len());
        for id in touched {
            let target = candidate
                .variants
                .iter_mut()
                .find(|variant| variant.id == id)
                .expect("variant was validated");
            target.revision =
                next_revision(target.revision, "assembly variant", target.id.to_string())?;
            target.semantic_digest =
                digest("rspice-assembly-variant-semantic/v1", &target.definition)?;
            revisions.push((id, target.revision));
        }
        candidate.validate()?;
        *self = candidate;
        Ok(revisions)
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
