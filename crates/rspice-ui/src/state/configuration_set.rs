//! Project-owned hierarchy and executable-view configuration sets.
//!
//! A configuration set is the durable authority that decides which design
//! root, DUT instance, executable views, stop views, scoped overrides, and
//! model profile participate in netlisting. Mutations use clone-and-validate
//! transactions so invalid input or a stale revision cannot partially alter
//! the catalog.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use thiserror::Error;
use uuid::Uuid;

use crate::product::ContentDigest;

use super::workspace::CellViewRef;

pub const CONFIGURATION_SET_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const MAX_CONFIGURATION_SETS: usize = 256;
pub const MAX_CONFIGURATION_NAME_BYTES: usize = 128;
pub const MAX_CONFIGURATION_OWNER_BYTES: usize = 256;
pub const MAX_CONFIGURATION_PATH_BYTES: usize = 1_024;
pub const MAX_CONFIGURATION_PATH_DEPTH: usize = 128;
pub const MAX_CONFIGURATION_VIEW_POLICY_ENTRIES: usize = 32;
pub const MAX_CONFIGURATION_OVERRIDES: usize = 4_096;
pub const MAX_CONFIGURATION_MODEL_SECTION_BYTES: usize = 256;

/// Canonical executable view types understood by the hierarchy resolver and
/// source-backed netlist generator. Persisted policies use these exact names;
/// aliases are normalized only at the mutation boundary.
pub const ALLOWED_EXECUTABLE_VIEW_TYPES: [&str; 5] =
    ["schematic", "testbench", "extracted", "spice", "veriloga"];

/// Stable project identity for one configuration set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConfigurationSetId(Uuid);

impl ConfigurationSetId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }

    #[must_use]
    pub fn is_nil(self) -> bool {
        self.0.is_nil()
    }
}

impl Default for ConfigurationSetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConfigurationSetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Exact-path view and model override inside a configuration's expanded DUT
/// hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationSetOverride {
    pub instance_path: String,
    pub executable_views: Vec<String>,
    pub stop_view: Option<String>,
    pub model_section: Option<String>,
    #[serde(default = "default_configuration_platforms")]
    pub eligible_platforms: Vec<ConfigurationPlatform>,
}

/// Editable semantic definition. Stable identity, revision, lineage, and
/// digest are catalog-owned and therefore cannot be supplied by callers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationSetDefinition {
    pub name: String,
    pub root: CellViewRef,
    pub dut_path: String,
    pub executable_view_policy: Vec<String>,
    pub stop_views: Vec<String>,
    #[serde(default)]
    pub unresolved_policy: UnresolvedBindingPolicy,
    #[serde(default)]
    pub black_box_policy: ConfigurationBlackBoxPolicy,
    pub overrides: Vec<ConfigurationSetOverride>,
    #[serde(default)]
    pub model_profile: ConfigurationModelProfile,
    pub owner: String,
}

/// A black-box boundary is never inferred from a missing implementation. The
/// only release-qualified policy requires an explicit, materialized source
/// view selected as a stop boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigurationBlackBoxPolicy {
    #[default]
    MaterializedSourceBoundariesOnly,
}

impl ConfigurationBlackBoxPolicy {
    pub const fn label(self) -> &'static str {
        match self {
            Self::MaterializedSourceBoundariesOnly => "Materialized source boundaries only",
        }
    }
}

/// Execution platforms that a scoped hierarchy binding is authorized to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigurationPlatform {
    Desktop,
    Browser,
}

impl ConfigurationPlatform {
    pub const ALL: [Self; 2] = [Self::Desktop, Self::Browser];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Browser => "Browser",
        }
    }

    pub const fn current() -> Self {
        if cfg!(target_arch = "wasm32") {
            Self::Browser
        } else {
            Self::Desktop
        }
    }
}

#[cfg(test)]
fn all_configuration_platforms() -> Vec<ConfigurationPlatform> {
    ConfigurationPlatform::ALL.to_vec()
}

fn default_configuration_platforms() -> Vec<ConfigurationPlatform> {
    vec![ConfigurationPlatform::Desktop]
}

/// Stable authority for the model environment used by a configuration. Exact
/// process sections remain owned by the active simulation run set and are
/// materialized from its sealed model-source snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigurationModelProfile {
    #[default]
    ProjectRunSetSections,
}

impl ConfigurationModelProfile {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProjectRunSetSections => "Project technology · run-set sections",
        }
    }
}

/// Fail-closed policy for a cell that cannot resolve within the configured
/// ordered views. Reviewed fallback appends the legacy requested-view order
/// and records every use in the immutable hierarchy receipt.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnresolvedBindingPolicy {
    #[default]
    BlockNetlist,
    ExplicitFallbackWithReview,
}

impl UnresolvedBindingPolicy {
    pub const ALL: [Self; 2] = [Self::BlockNetlist, Self::ExplicitFallbackWithReview];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BlockNetlist => "Block netlist",
            Self::ExplicitFallbackWithReview => "Use explicit fallback with review",
        }
    }
}

/// Exact semantic subset copied by the mockup's clone workflow.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfigurationCloneScope {
    #[default]
    AllBindings,
    BindingsOnly,
    EnvironmentOnly,
}

impl ConfigurationCloneScope {
    pub const ALL: [Self; 3] = [Self::AllBindings, Self::BindingsOnly, Self::EnvironmentOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::AllBindings => "All bindings",
            Self::BindingsOnly => "Bindings only",
            Self::EnvironmentOnly => "Environment only",
        }
    }
}

/// Immutable clone provenance. The source may later be removed; its stable ID
/// and exact revision remain sufficient to retain lineage without fabricating
/// a live dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationSetLineage {
    pub source_id: ConfigurationSetId,
    pub source_revision: u64,
}

/// Current revision of one project configuration set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationSet {
    id: ConfigurationSetId,
    revision: u64,
    lineage: Option<ConfigurationSetLineage>,
    semantic_digest: ContentDigest,
    definition: ConfigurationSetDefinition,
}

impl ConfigurationSet {
    #[must_use]
    pub const fn id(&self) -> ConfigurationSetId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn lineage(&self) -> Option<ConfigurationSetLineage> {
        self.lineage
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    #[must_use]
    pub const fn definition(&self) -> &ConfigurationSetDefinition {
        &self.definition
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.definition.name
    }

    #[must_use]
    pub const fn root(&self) -> &CellViewRef {
        &self.definition.root
    }

    #[must_use]
    pub fn dut_path(&self) -> &str {
        &self.definition.dut_path
    }

    #[must_use]
    pub fn executable_view_policy(&self) -> &[String] {
        &self.definition.executable_view_policy
    }

    #[must_use]
    pub fn stop_views(&self) -> &[String] {
        &self.definition.stop_views
    }

    #[must_use]
    pub fn overrides(&self) -> &[ConfigurationSetOverride] {
        &self.definition.overrides
    }

    #[must_use]
    pub const fn model_profile(&self) -> ConfigurationModelProfile {
        self.definition.model_profile
    }

    #[must_use]
    pub fn owner(&self) -> &str {
        &self.definition.owner
    }

    fn validate(&self) -> Result<(), ConfigurationSetError> {
        if self.id.is_nil() {
            return Err(ConfigurationSetError::NilIdentity);
        }
        if self.revision == 0 {
            return Err(ConfigurationSetError::ZeroRevision(self.id));
        }
        if let Some(lineage) = self.lineage {
            if lineage.source_id.is_nil() {
                return Err(ConfigurationSetError::NilLineageIdentity(self.id));
            }
            if lineage.source_id == self.id {
                return Err(ConfigurationSetError::SelfLineage(self.id));
            }
            if lineage.source_revision == 0 {
                return Err(ConfigurationSetError::ZeroLineageRevision(self.id));
            }
        }
        validate_definition(&self.definition)?;
        let expected = semantic_digest(&self.definition)?;
        if self.semantic_digest != expected {
            return Err(ConfigurationSetError::SemanticDigestMismatch(self.id));
        }
        Ok(())
    }
}

/// Schema-versioned project catalog. A missing catalog at the owning project
/// field migrates through `Default`; once a catalog object exists, its schema
/// and all retained content validate fail-closed during deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationSetCatalog {
    schema_version: u16,
    configurations: Vec<ConfigurationSet>,
    active_configuration_id: Option<ConfigurationSetId>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigurationSetCatalogWire {
    schema_version: u16,
    #[serde(default)]
    configurations: Vec<ConfigurationSet>,
    #[serde(default)]
    active_configuration_id: Option<ConfigurationSetId>,
}

impl<'de> Deserialize<'de> for ConfigurationSetCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ConfigurationSetCatalogWire::deserialize(deserializer)?;
        let catalog = Self {
            schema_version: wire.schema_version,
            configurations: wire.configurations,
            active_configuration_id: wire.active_configuration_id,
        };
        catalog.validate().map_err(serde::de::Error::custom)?;
        Ok(catalog)
    }
}

impl Default for ConfigurationSetCatalog {
    fn default() -> Self {
        Self {
            schema_version: CONFIGURATION_SET_CATALOG_SCHEMA_VERSION,
            configurations: Vec::new(),
            active_configuration_id: None,
        }
    }
}

impl ConfigurationSetCatalog {
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub fn configurations(&self) -> &[ConfigurationSet] {
        &self.configurations
    }

    #[must_use]
    pub const fn active_configuration_id(&self) -> Option<ConfigurationSetId> {
        self.active_configuration_id
    }

    #[must_use]
    pub fn find(&self, id: ConfigurationSetId) -> Option<&ConfigurationSet> {
        self.configurations.iter().find(|entry| entry.id == id)
    }

    #[must_use]
    pub fn active(&self) -> Option<&ConfigurationSet> {
        self.active_configuration_id.and_then(|id| self.find(id))
    }

    /// Return every configuration whose executable root is owned by the
    /// requested cell or exact view. Callers use this before destructive
    /// library operations so a valid catalog can never be made dangling.
    #[must_use]
    pub fn roots_in_scope(
        &self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<&ConfigurationSet> {
        self.configurations
            .iter()
            .filter(|configuration| {
                let root = configuration.root();
                root.library.eq_ignore_ascii_case(library)
                    && root.cell.eq_ignore_ascii_case(cell)
                    && view.is_none_or(|view| root.view.eq_ignore_ascii_case(view))
            })
            .collect()
    }

    /// Remap configuration roots as part of a cell rename. Every affected
    /// configuration receives a new revision and semantic digest, while the
    /// catalog is committed only after the complete candidate validates.
    pub fn rename_cell_roots(
        &mut self,
        library: &str,
        old_cell: &str,
        new_cell: &str,
    ) -> Result<usize, ConfigurationSetError> {
        self.validate()?;
        let mut candidate = self.clone();
        let mut changed = 0usize;
        for configuration in &mut candidate.configurations {
            if configuration
                .definition
                .root
                .library
                .eq_ignore_ascii_case(library)
                && configuration
                    .definition
                    .root
                    .cell
                    .eq_ignore_ascii_case(old_cell)
            {
                configuration.definition.root.cell = new_cell.to_owned();
                configuration.revision = configuration
                    .revision
                    .checked_add(1)
                    .ok_or(ConfigurationSetError::RevisionExhausted(configuration.id))?;
                configuration.semantic_digest = semantic_digest(&configuration.definition)?;
                changed += 1;
            }
        }
        if changed == 0 {
            return Ok(0);
        }
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
    }

    pub fn validate(&self) -> Result<(), ConfigurationSetError> {
        if self.schema_version != CONFIGURATION_SET_CATALOG_SCHEMA_VERSION {
            return Err(ConfigurationSetError::UnsupportedSchema(
                self.schema_version,
            ));
        }
        if self.configurations.len() > MAX_CONFIGURATION_SETS {
            return Err(ConfigurationSetError::CatalogLimitExceeded {
                actual: self.configurations.len(),
                maximum: MAX_CONFIGURATION_SETS,
            });
        }

        let mut ids = HashSet::with_capacity(self.configurations.len());
        let mut names = HashSet::with_capacity(self.configurations.len());
        for configuration in &self.configurations {
            configuration.validate()?;
            if !ids.insert(configuration.id) {
                return Err(ConfigurationSetError::DuplicateIdentity(configuration.id));
            }
            if !names.insert(case_fold(configuration.name())) {
                return Err(ConfigurationSetError::DuplicateName(
                    configuration.name().to_owned(),
                ));
            }
        }

        match self.active_configuration_id {
            Some(active) if !ids.contains(&active) => {
                Err(ConfigurationSetError::ActiveConfigurationMissing(active))
            }
            None if !self.configurations.is_empty() => {
                Err(ConfigurationSetError::ActiveConfigurationRequired)
            }
            _ => Ok(()),
        }
    }

    /// Create a configuration. The first configuration becomes active; later
    /// creates preserve the user's exact active selection.
    pub fn create(
        &mut self,
        definition: ConfigurationSetDefinition,
    ) -> Result<ConfigurationSetId, ConfigurationSetError> {
        self.validate()?;
        if self.configurations.len() >= MAX_CONFIGURATION_SETS {
            return Err(ConfigurationSetError::CatalogLimitExceeded {
                actual: self.configurations.len().saturating_add(1),
                maximum: MAX_CONFIGURATION_SETS,
            });
        }
        let definition = normalize_definition(definition);
        validate_definition(&definition)?;
        self.require_unique_name(&definition.name, None)?;

        let id = ConfigurationSetId::new();
        let configuration = ConfigurationSet {
            id,
            revision: 1,
            lineage: None,
            semantic_digest: semantic_digest(&definition)?,
            definition,
        };
        let mut candidate = self.clone();
        candidate.configurations.push(configuration);
        if candidate.active_configuration_id.is_none() {
            candidate.active_configuration_id = Some(id);
        }
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    /// Clone an exact source revision under a new unique name. The clone owns
    /// a new stable identity and starts at revision one while retaining source
    /// identity and revision as immutable lineage.
    pub fn clone_configuration(
        &mut self,
        source_id: ConfigurationSetId,
        expected_source_revision: u64,
        new_name: impl Into<String>,
    ) -> Result<ConfigurationSetId, ConfigurationSetError> {
        let defaults = self
            .find(source_id)
            .ok_or(ConfigurationSetError::NotFound(source_id))?
            .definition
            .clone();
        self.clone_configuration_scoped(
            source_id,
            expected_source_revision,
            new_name,
            ConfigurationCloneScope::AllBindings,
            defaults,
        )
    }

    /// Clone the exact mockup-selected semantic subset into caller-provided
    /// destination defaults. Defaults are independently validated, so a
    /// partial copy can never retain an invalid implicit root or policy.
    pub fn clone_configuration_scoped(
        &mut self,
        source_id: ConfigurationSetId,
        expected_source_revision: u64,
        new_name: impl Into<String>,
        scope: ConfigurationCloneScope,
        destination_defaults: ConfigurationSetDefinition,
    ) -> Result<ConfigurationSetId, ConfigurationSetError> {
        self.validate()?;
        if self.configurations.len() >= MAX_CONFIGURATION_SETS {
            return Err(ConfigurationSetError::CatalogLimitExceeded {
                actual: self.configurations.len().saturating_add(1),
                maximum: MAX_CONFIGURATION_SETS,
            });
        }
        let source = self
            .find(source_id)
            .ok_or(ConfigurationSetError::NotFound(source_id))?;
        require_revision(source, expected_source_revision)?;
        let mut definition = match scope {
            ConfigurationCloneScope::AllBindings => source.definition.clone(),
            ConfigurationCloneScope::BindingsOnly => {
                let mut definition = destination_defaults;
                definition.root = source.definition.root.clone();
                definition.dut_path = source.definition.dut_path.clone();
                definition.executable_view_policy =
                    source.definition.executable_view_policy.clone();
                definition.stop_views = source.definition.stop_views.clone();
                definition.unresolved_policy = source.definition.unresolved_policy;
                definition.black_box_policy = source.definition.black_box_policy;
                definition.overrides = source.definition.overrides.clone();
                definition
            }
            ConfigurationCloneScope::EnvironmentOnly => {
                let mut definition = destination_defaults;
                definition.model_profile = source.definition.model_profile;
                definition
            }
        };
        definition.name = new_name.into();
        let definition = normalize_definition(definition);
        validate_definition(&definition)?;
        self.require_unique_name(&definition.name, None)?;

        let id = ConfigurationSetId::new();
        let clone = ConfigurationSet {
            id,
            revision: 1,
            lineage: Some(ConfigurationSetLineage {
                source_id,
                source_revision: expected_source_revision,
            }),
            semantic_digest: semantic_digest(&definition)?,
            definition,
        };
        let mut candidate = self.clone();
        candidate.configurations.push(clone);
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    /// Replace one complete semantic definition using optimistic revision
    /// authority. No-op updates are rejected and revision exhaustion fails
    /// before the live catalog changes.
    pub fn update(
        &mut self,
        id: ConfigurationSetId,
        expected_revision: u64,
        definition: ConfigurationSetDefinition,
    ) -> Result<u64, ConfigurationSetError> {
        self.validate()?;
        let current = self.find(id).ok_or(ConfigurationSetError::NotFound(id))?;
        require_revision(current, expected_revision)?;
        let definition = normalize_definition(definition);
        validate_definition(&definition)?;
        self.require_unique_name(&definition.name, Some(id))?;
        if current.definition == definition {
            return Err(ConfigurationSetError::NoChanges(id));
        }
        let committed_revision = current
            .revision
            .checked_add(1)
            .ok_or(ConfigurationSetError::RevisionExhausted(id))?;
        let digest = semantic_digest(&definition)?;

        let mut candidate = self.clone();
        let target = candidate
            .configurations
            .iter_mut()
            .find(|entry| entry.id == id)
            .ok_or(ConfigurationSetError::NotFound(id))?;
        target.revision = committed_revision;
        target.semantic_digest = digest;
        target.definition = definition;
        candidate.validate()?;
        *self = candidate;
        Ok(committed_revision)
    }

    /// Remove a non-active configuration under exact revision authority. An
    /// active set must first be replaced explicitly so deletion can never
    /// silently change the design used for netlisting.
    pub fn remove(
        &mut self,
        id: ConfigurationSetId,
        expected_revision: u64,
    ) -> Result<(), ConfigurationSetError> {
        self.validate()?;
        let current = self.find(id).ok_or(ConfigurationSetError::NotFound(id))?;
        require_revision(current, expected_revision)?;
        if self.active_configuration_id == Some(id) {
            return Err(ConfigurationSetError::ActiveConfigurationRemoval(id));
        }

        let mut candidate = self.clone();
        candidate.configurations.retain(|entry| entry.id != id);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Activate an existing configuration. Activation does not revise the
    /// configuration because it changes catalog selection, not its semantics.
    pub fn activate(&mut self, id: ConfigurationSetId) -> Result<(), ConfigurationSetError> {
        self.validate()?;
        if self.find(id).is_none() {
            return Err(ConfigurationSetError::NotFound(id));
        }
        if self.active_configuration_id == Some(id) {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.active_configuration_id = Some(id);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    fn require_unique_name(
        &self,
        name: &str,
        except: Option<ConfigurationSetId>,
    ) -> Result<(), ConfigurationSetError> {
        let folded = case_fold(name);
        if self
            .configurations
            .iter()
            .any(|entry| Some(entry.id) != except && case_fold(entry.name()) == folded)
        {
            return Err(ConfigurationSetError::DuplicateName(name.to_owned()));
        }
        Ok(())
    }
}

fn require_revision(
    configuration: &ConfigurationSet,
    expected: u64,
) -> Result<(), ConfigurationSetError> {
    if configuration.revision != expected {
        return Err(ConfigurationSetError::RevisionConflict {
            id: configuration.id,
            expected,
            actual: configuration.revision,
        });
    }
    Ok(())
}

fn normalize_definition(mut definition: ConfigurationSetDefinition) -> ConfigurationSetDefinition {
    definition.name = definition.name.trim().to_owned();
    definition.dut_path = definition.dut_path.trim().to_owned();
    definition.owner = definition.owner.trim().to_owned();
    definition.executable_view_policy = definition
        .executable_view_policy
        .into_iter()
        .map(|view| view.trim().to_ascii_lowercase())
        .collect();
    definition.stop_views = definition
        .stop_views
        .into_iter()
        .map(|view| view.trim().to_ascii_lowercase())
        .collect();
    for scoped in &mut definition.overrides {
        scoped.instance_path = scoped.instance_path.trim().to_owned();
        scoped.executable_views = scoped
            .executable_views
            .drain(..)
            .map(|view| view.trim().to_ascii_lowercase())
            .collect();
        scoped.stop_view = scoped
            .stop_view
            .take()
            .map(|view| view.trim().to_ascii_lowercase());
        scoped.model_section = scoped
            .model_section
            .take()
            .map(|section| section.trim().to_owned());
        scoped.eligible_platforms.sort();
        scoped.eligible_platforms.dedup();
    }
    definition.overrides.sort_by(|left, right| {
        case_fold(&left.instance_path)
            .cmp(&case_fold(&right.instance_path))
            .then_with(|| left.instance_path.cmp(&right.instance_path))
    });
    definition
}

fn validate_definition(
    definition: &ConfigurationSetDefinition,
) -> Result<(), ConfigurationSetError> {
    validate_text(
        "configuration.name",
        &definition.name,
        MAX_CONFIGURATION_NAME_BYTES,
    )?;
    definition
        .root
        .validate_name_segments()
        .map_err(|error| ConfigurationSetError::InvalidCellViewReference(error.to_string()))?;
    validate_absolute_instance_path("configuration.dut-path", &definition.dut_path)?;
    validate_view_policy(
        "configuration.executable-view-policy",
        &definition.executable_view_policy,
        true,
    )?;
    validate_view_policy("configuration.stop-views", &definition.stop_views, false)?;
    require_views_are_members(
        "configuration.stop-views",
        &definition.stop_views,
        &definition.executable_view_policy,
    )?;
    validate_text(
        "configuration.owner",
        &definition.owner,
        MAX_CONFIGURATION_OWNER_BYTES,
    )?;
    if definition.overrides.len() > MAX_CONFIGURATION_OVERRIDES {
        return Err(ConfigurationSetError::OverrideLimitExceeded {
            actual: definition.overrides.len(),
            maximum: MAX_CONFIGURATION_OVERRIDES,
        });
    }

    if definition
        .overrides
        .windows(2)
        .any(|pair| case_fold(&pair[0].instance_path) > case_fold(&pair[1].instance_path))
    {
        return Err(ConfigurationSetError::UnsortedOverridePaths);
    }

    let mut paths = HashSet::with_capacity(definition.overrides.len());
    for scoped in &definition.overrides {
        validate_hierarchy_path_pattern("configuration.override.path", &scoped.instance_path)?;
        if !paths.insert(case_fold(&scoped.instance_path)) {
            return Err(ConfigurationSetError::DuplicateOverridePath(
                scoped.instance_path.clone(),
            ));
        }
        validate_view_policy(
            "configuration.override.executable-views",
            &scoped.executable_views,
            true,
        )?;
        if let Some(stop_view) = scoped.stop_view.as_deref() {
            validate_executable_view("configuration.override.stop-view", stop_view)?;
            require_view_is_member(
                "configuration.override.stop-view",
                stop_view,
                &scoped.executable_views,
            )?;
        }
        if let Some(model_section) = scoped.model_section.as_deref() {
            validate_text(
                "configuration.override.model-section",
                model_section,
                MAX_CONFIGURATION_MODEL_SECTION_BYTES,
            )?;
            validate_model_section_identifier(model_section)?;
        }
        if scoped.eligible_platforms.is_empty() {
            return Err(ConfigurationSetError::NoEligiblePlatforms(
                scoped.instance_path.clone(),
            ));
        }
    }
    for (index, left) in definition.overrides.iter().enumerate() {
        for right in definition.overrides.iter().skip(index + 1) {
            if patterns_overlap(&left.instance_path, &right.instance_path)
                && pattern_specificity(&left.instance_path)
                    == pattern_specificity(&right.instance_path)
            {
                return Err(ConfigurationSetError::AmbiguousOverridePatterns {
                    left: left.instance_path.clone(),
                    right: right.instance_path.clone(),
                });
            }
        }
    }
    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ConfigurationSetError> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(ConfigurationSetError::InvalidText {
            field,
            maximum_bytes,
        });
    }
    Ok(())
}

fn validate_model_section_identifier(section: &str) -> Result<(), ConfigurationSetError> {
    let mut characters = section.chars();
    let valid_start = characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_');
    let valid_tail = characters.all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '$' | '-')
    });
    if valid_start && valid_tail {
        Ok(())
    } else {
        Err(ConfigurationSetError::InvalidModelSection(
            section.to_owned(),
        ))
    }
}

fn validate_absolute_instance_path(
    field: &'static str,
    path: &str,
) -> Result<(), ConfigurationSetError> {
    if path.len() > MAX_CONFIGURATION_PATH_BYTES
        || !path.starts_with('/')
        || path.ends_with('/')
        || path.chars().any(char::is_control)
    {
        return Err(ConfigurationSetError::InvalidInstancePath {
            field,
            path: path.to_owned(),
        });
    }
    let segments = path[1..].split('/').collect::<Vec<_>>();
    if segments.is_empty()
        || segments.len() > MAX_CONFIGURATION_PATH_DEPTH
        || segments.iter().any(|segment| {
            segment.is_empty()
                || segment
                    .chars()
                    .any(|character| !character.is_alphanumeric() && character != '_')
        })
    {
        return Err(ConfigurationSetError::InvalidInstancePath {
            field,
            path: path.to_owned(),
        });
    }
    Ok(())
}

fn validate_hierarchy_path_pattern(
    field: &'static str,
    pattern: &str,
) -> Result<(), ConfigurationSetError> {
    if pattern.len() > MAX_CONFIGURATION_PATH_BYTES
        || !pattern.starts_with('/')
        || pattern.ends_with('/')
        || pattern.chars().any(char::is_control)
    {
        return Err(ConfigurationSetError::InvalidHierarchyPattern {
            field,
            pattern: pattern.to_owned(),
        });
    }
    let segments = pattern[1..].split('/').collect::<Vec<_>>();
    if segments.is_empty()
        || segments.len() > MAX_CONFIGURATION_PATH_DEPTH
        || segments.iter().any(|segment| {
            segment.is_empty()
                || (*segment != "*"
                    && segment
                        .chars()
                        .any(|character| !character.is_alphanumeric() && character != '_'))
        })
    {
        return Err(ConfigurationSetError::InvalidHierarchyPattern {
            field,
            pattern: pattern.to_owned(),
        });
    }
    Ok(())
}

fn pattern_specificity(pattern: &str) -> usize {
    pattern[1..]
        .split('/')
        .filter(|segment| *segment != "*")
        .count()
}

fn patterns_overlap(left: &str, right: &str) -> bool {
    let left = left[1..].split('/').collect::<Vec<_>>();
    let right = right[1..].split('/').collect::<Vec<_>>();
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| *left == "*" || right == "*" || left.eq_ignore_ascii_case(right))
}

fn validate_view_policy(
    field: &'static str,
    views: &[String],
    require_nonempty: bool,
) -> Result<(), ConfigurationSetError> {
    if (require_nonempty && views.is_empty()) || views.len() > MAX_CONFIGURATION_VIEW_POLICY_ENTRIES
    {
        return Err(ConfigurationSetError::InvalidViewPolicyLength {
            field,
            actual: views.len(),
            maximum: MAX_CONFIGURATION_VIEW_POLICY_ENTRIES,
            require_nonempty,
        });
    }
    let mut unique = HashSet::with_capacity(views.len());
    for view in views {
        validate_executable_view(field, view)?;
        if !unique.insert(view.to_ascii_lowercase()) {
            return Err(ConfigurationSetError::DuplicateView {
                field,
                view: view.clone(),
            });
        }
    }
    Ok(())
}

fn validate_executable_view(field: &'static str, view: &str) -> Result<(), ConfigurationSetError> {
    if view.trim() != view || !ALLOWED_EXECUTABLE_VIEW_TYPES.contains(&view) {
        return Err(ConfigurationSetError::UnsupportedExecutableView {
            field,
            view: view.to_owned(),
        });
    }
    Ok(())
}

fn require_views_are_members(
    field: &'static str,
    members: &[String],
    policy: &[String],
) -> Result<(), ConfigurationSetError> {
    for member in members {
        require_view_is_member(field, member, policy)?;
    }
    Ok(())
}

fn require_view_is_member(
    field: &'static str,
    member: &str,
    policy: &[String],
) -> Result<(), ConfigurationSetError> {
    if !policy.iter().any(|candidate| candidate == member) {
        return Err(ConfigurationSetError::StopViewOutsidePolicy {
            field,
            view: member.to_owned(),
        });
    }
    Ok(())
}

fn case_fold(value: &str) -> String {
    value.to_lowercase()
}

fn semantic_digest(
    definition: &ConfigurationSetDefinition,
) -> Result<ContentDigest, ConfigurationSetError> {
    #[derive(Serialize)]
    struct SemanticMaterial<'a> {
        schema: &'static str,
        definition: &'a ConfigurationSetDefinition,
    }
    let bytes = serde_json::to_vec(&SemanticMaterial {
        schema: "rspice-project-configuration-set-semantic/v1",
        definition,
    })
    .map_err(|error| ConfigurationSetError::Serialization(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigurationSetError {
    #[error("configuration-set catalog schema {0} is unsupported")]
    UnsupportedSchema(u16),
    #[error("configuration-set catalog contains {actual} entries; maximum is {maximum}")]
    CatalogLimitExceeded { actual: usize, maximum: usize },
    #[error("configuration-set identity must not be nil")]
    NilIdentity,
    #[error("configuration-set identity {0} is duplicated")]
    DuplicateIdentity(ConfigurationSetId),
    #[error("configuration-set {0} has revision zero")]
    ZeroRevision(ConfigurationSetId),
    #[error("configuration-set {0} has a nil lineage source")]
    NilLineageIdentity(ConfigurationSetId),
    #[error("configuration-set {0} cannot derive from itself")]
    SelfLineage(ConfigurationSetId),
    #[error("configuration-set {0} has lineage revision zero")]
    ZeroLineageRevision(ConfigurationSetId),
    #[error("configuration-set name {0:?} is already in use (names are case-insensitive)")]
    DuplicateName(String),
    #[error("configuration-set {0} was not found")]
    NotFound(ConfigurationSetId),
    #[error(
        "configuration-set {id} revision conflict: expected {expected}, current revision is {actual}"
    )]
    RevisionConflict {
        id: ConfigurationSetId,
        expected: u64,
        actual: u64,
    },
    #[error("configuration-set {0} revision space is exhausted")]
    RevisionExhausted(ConfigurationSetId),
    #[error("configuration-set {0} has no semantic changes")]
    NoChanges(ConfigurationSetId),
    #[error("active configuration-set {0} must be replaced explicitly before removal")]
    ActiveConfigurationRemoval(ConfigurationSetId),
    #[error("active configuration-set {0} does not exist in the catalog")]
    ActiveConfigurationMissing(ConfigurationSetId),
    #[error("a non-empty configuration-set catalog requires an active configuration")]
    ActiveConfigurationRequired,
    #[error("configuration-set {0} has an invalid semantic digest")]
    SemanticDigestMismatch(ConfigurationSetId),
    #[error("configuration root has an invalid library/cell/view segment: {0}")]
    InvalidCellViewReference(String),
    #[error(
        "{field} must be trimmed, non-empty, contain no control characters, and not exceed {maximum_bytes} bytes"
    )]
    InvalidText {
        field: &'static str,
        maximum_bytes: usize,
    },
    #[error("{field} contains invalid absolute instance path {path:?}")]
    InvalidInstancePath { field: &'static str, path: String },
    #[error(
        "{field} contains {actual} entries; maximum is {maximum} and non-empty is {require_nonempty}"
    )]
    InvalidViewPolicyLength {
        field: &'static str,
        actual: usize,
        maximum: usize,
        require_nonempty: bool,
    },
    #[error("{field} contains unsupported executable view {view:?}")]
    UnsupportedExecutableView { field: &'static str, view: String },
    #[error("{field} contains duplicate executable view {view:?}")]
    DuplicateView { field: &'static str, view: String },
    #[error("{field} stop view {view:?} does not occur in its ordered executable policy")]
    StopViewOutsidePolicy { field: &'static str, view: String },
    #[error("configuration contains {actual} scoped overrides; maximum is {maximum}")]
    OverrideLimitExceeded { actual: usize, maximum: usize },
    #[error("scoped override path {0:?} is duplicated (paths are case-insensitive)")]
    DuplicateOverridePath(String),
    #[error("scoped override patterns {left:?} and {right:?} overlap at equal specificity")]
    AmbiguousOverridePatterns { left: String, right: String },
    #[error("{field} contains invalid bounded hierarchy pattern {pattern:?}")]
    InvalidHierarchyPattern {
        field: &'static str,
        pattern: String,
    },
    #[error("scoped override paths must be stored in canonical case-insensitive order")]
    UnsortedOverridePaths,
    #[error("scoped override {0:?} must authorize at least one execution platform")]
    NoEligiblePlatforms(String),
    #[error("model section {0:?} is not a valid SPICE library-section identifier")]
    InvalidModelSection(String),
    #[error("configuration-set digest serialization failed: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(name: &str) -> ConfigurationSetDefinition {
        ConfigurationSetDefinition {
            name: name.to_owned(),
            root: CellViewRef::new("user", "top_tb", "testbench"),
            dut_path: "/top/XAFE".to_owned(),
            executable_view_policy: vec![
                "schematic".to_owned(),
                "extracted".to_owned(),
                "spice".to_owned(),
            ],
            stop_views: vec!["extracted".to_owned(), "spice".to_owned()],
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![ConfigurationSetOverride {
                instance_path: "/top/XAFE/XADC".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: Some("tt".to_owned()),
                eligible_platforms: all_configuration_platforms(),
            }],
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Analog design".to_owned(),
        }
    }

    #[test]
    fn semantic_digest_is_deterministic_and_round_trips() {
        let mut first = ConfigurationSetCatalog::default();
        let mut second = ConfigurationSetCatalog::default();
        let first_id = first.create(definition("Release")).expect("create first");
        let second_id = second.create(definition("Release")).expect("create second");

        assert_ne!(first_id, second_id);
        assert_eq!(
            first.find(first_id).unwrap().semantic_digest(),
            second.find(second_id).unwrap().semantic_digest()
        );
        let encoded = serde_json::to_string(&first).expect("serialize catalog");
        let restored: ConfigurationSetCatalog =
            serde_json::from_str(&encoded).expect("deserialize catalog");
        assert_eq!(restored, first);
        restored.validate().expect("restored catalog validates");
    }

    #[test]
    fn unsupported_and_unknown_catalog_schemas_fail_closed() {
        let mut value = serde_json::to_value(ConfigurationSetCatalog::default()).unwrap();
        value["schema_version"] = serde_json::json!(2);
        assert!(serde_json::from_value::<ConfigurationSetCatalog>(value).is_err());

        let mut unknown = serde_json::to_value(ConfigurationSetCatalog::default()).unwrap();
        unknown["future_policy"] = serde_json::json!(true);
        assert!(serde_json::from_value::<ConfigurationSetCatalog>(unknown).is_err());
    }

    #[test]
    fn clone_retains_exact_source_lineage_and_new_identity() {
        let mut catalog = ConfigurationSetCatalog::default();
        let source_id = catalog.create(definition("Release")).expect("source");
        let clone_id = catalog
            .clone_configuration(source_id, 1, "Post-layout")
            .expect("clone");
        let cloned = catalog.find(clone_id).expect("stored clone");

        assert_ne!(source_id, clone_id);
        assert_eq!(cloned.revision(), 1);
        assert_eq!(
            cloned.lineage(),
            Some(ConfigurationSetLineage {
                source_id,
                source_revision: 1,
            })
        );
        assert_eq!(catalog.active_configuration_id(), Some(source_id));
    }

    #[test]
    fn scoped_clone_copies_only_the_reviewed_semantic_subset() {
        let mut bindings_catalog = ConfigurationSetCatalog::default();
        let source_id = bindings_catalog
            .create(definition("Release"))
            .expect("source");
        let mut defaults = definition("Destination defaults");
        defaults.root = CellViewRef::new("user", "other_tb", "schematic");
        defaults.dut_path = "/top/XOTHER".to_owned();
        defaults.executable_view_policy = vec!["schematic".to_owned()];
        defaults.stop_views.clear();
        defaults.unresolved_policy = UnresolvedBindingPolicy::ExplicitFallbackWithReview;
        defaults.overrides.clear();
        defaults.owner = "Destination owner".to_owned();

        let bindings_id = bindings_catalog
            .clone_configuration_scoped(
                source_id,
                1,
                "Bindings clone",
                ConfigurationCloneScope::BindingsOnly,
                defaults.clone(),
            )
            .expect("bindings clone");
        let source = bindings_catalog
            .find(source_id)
            .expect("source remains")
            .clone();
        let bindings = bindings_catalog
            .find(bindings_id)
            .expect("bindings clone retained");
        assert_eq!(bindings.root(), source.root());
        assert_eq!(bindings.dut_path(), source.dut_path());
        assert_eq!(
            bindings.executable_view_policy(),
            source.executable_view_policy()
        );
        assert_eq!(bindings.stop_views(), source.stop_views());
        assert_eq!(bindings.overrides(), source.overrides());
        assert_eq!(
            bindings.model_profile(),
            ConfigurationModelProfile::ProjectRunSetSections
        );
        assert_eq!(bindings.owner(), "Destination owner");

        let environment_id = bindings_catalog
            .clone_configuration_scoped(
                source_id,
                1,
                "Environment clone",
                ConfigurationCloneScope::EnvironmentOnly,
                defaults.clone(),
            )
            .expect("environment clone");
        let environment = bindings_catalog
            .find(environment_id)
            .expect("environment clone retained");
        assert_eq!(environment.root(), &defaults.root);
        assert_eq!(environment.dut_path(), defaults.dut_path);
        assert_eq!(
            environment.executable_view_policy(),
            defaults.executable_view_policy
        );
        assert_eq!(environment.stop_views(), defaults.stop_views);
        assert_eq!(environment.overrides(), defaults.overrides);
        assert_eq!(environment.model_profile(), source.model_profile());
        assert_eq!(environment.owner(), "Destination owner");
    }

    #[test]
    fn active_configuration_must_be_replaced_before_removal() {
        let mut catalog = ConfigurationSetCatalog::default();
        let source_id = catalog.create(definition("Release")).expect("source");
        let clone_id = catalog
            .clone_configuration(source_id, 1, "Post-layout")
            .expect("clone");
        let before = catalog.clone();
        assert_eq!(
            catalog.remove(source_id, 1),
            Err(ConfigurationSetError::ActiveConfigurationRemoval(source_id))
        );
        assert_eq!(catalog, before, "failed removal is transactional");

        catalog.activate(clone_id).expect("activate replacement");
        catalog
            .remove(source_id, 1)
            .expect("remove inactive source");
        assert!(catalog.find(source_id).is_none());
        assert_eq!(catalog.active_configuration_id(), Some(clone_id));
    }

    #[test]
    fn cell_root_rename_revises_all_affected_configurations_atomically() {
        let mut catalog = ConfigurationSetCatalog::default();
        let first = catalog.create(definition("Release")).expect("first");
        let second = catalog
            .clone_configuration(first, 1, "Characterization")
            .expect("second");
        let old_first_digest = catalog.find(first).unwrap().semantic_digest();
        let old_second_digest = catalog.find(second).unwrap().semantic_digest();

        assert_eq!(
            catalog.rename_cell_roots("USER", "TOP_TB", "renamed_tb"),
            Ok(2)
        );
        for id in [first, second] {
            let configuration = catalog.find(id).expect("configuration remains");
            assert_eq!(configuration.root().cell, "renamed_tb");
            assert_eq!(configuration.revision(), 2);
        }
        assert_ne!(
            catalog.find(first).unwrap().semantic_digest(),
            old_first_digest
        );
        assert_ne!(
            catalog.find(second).unwrap().semantic_digest(),
            old_second_digest
        );
        assert_eq!(
            catalog
                .roots_in_scope("user", "renamed_tb", Some("testbench"))
                .len(),
            2
        );
        catalog.validate().expect("remapped catalog validates");
    }

    #[test]
    fn duplicate_names_and_override_paths_are_rejected_atomically() {
        let mut catalog = ConfigurationSetCatalog::default();
        catalog.create(definition("Release")).expect("source");
        let before = catalog.clone();
        assert_eq!(
            catalog.create(definition("release")),
            Err(ConfigurationSetError::DuplicateName("release".to_owned()))
        );
        assert_eq!(catalog, before);

        let mut invalid = definition("Characterization");
        let mut duplicate = invalid.overrides[0].clone();
        duplicate.instance_path = "/TOP/xafe/xadc".to_owned();
        invalid.overrides.push(duplicate);
        assert!(matches!(
            catalog.create(invalid),
            Err(ConfigurationSetError::DuplicateOverridePath(_))
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn hierarchy_patterns_are_bounded_and_have_deterministic_precedence() {
        let mut catalog = ConfigurationSetCatalog::default();
        let mut valid = definition("Pattern policy");
        valid.overrides = vec![
            ConfigurationSetOverride {
                instance_path: "/top/*".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: all_configuration_platforms(),
            },
            ConfigurationSetOverride {
                instance_path: "/top/XAFE".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: all_configuration_platforms(),
            },
        ];
        catalog
            .create(valid)
            .expect("specific path wins over wildcard");

        let before = catalog.clone();
        let mut ambiguous = definition("Ambiguous patterns");
        ambiguous.overrides = vec![
            ConfigurationSetOverride {
                instance_path: "/top/*/X1".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: all_configuration_platforms(),
            },
            ConfigurationSetOverride {
                instance_path: "/top/XAFE/*".to_owned(),
                executable_views: vec!["spice".to_owned()],
                stop_view: Some("spice".to_owned()),
                model_section: None,
                eligible_platforms: all_configuration_platforms(),
            },
        ];
        assert!(matches!(
            catalog.create(ambiguous),
            Err(ConfigurationSetError::AmbiguousOverridePatterns { .. })
        ));
        assert_eq!(catalog, before);

        let mut invalid = definition("Unbounded pattern");
        invalid.overrides[0].instance_path = "/top/**".to_owned();
        assert!(matches!(
            catalog.create(invalid),
            Err(ConfigurationSetError::InvalidHierarchyPattern { .. })
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn scoped_binding_requires_an_explicit_execution_platform() {
        let mut catalog = ConfigurationSetCatalog::default();
        let mut invalid = definition("No execution target");
        invalid.overrides[0].eligible_platforms.clear();

        assert!(matches!(
            catalog.create(invalid),
            Err(ConfigurationSetError::NoEligiblePlatforms(_))
        ));
        assert!(catalog.configurations().is_empty());
    }

    #[test]
    fn stale_or_invalid_updates_preserve_the_complete_catalog() {
        let mut catalog = ConfigurationSetCatalog::default();
        let id = catalog.create(definition("Release")).expect("source");
        let before = catalog.clone();
        let mut changed = definition("Release updated");
        changed.executable_view_policy = vec!["layout".to_owned()];
        assert!(matches!(
            catalog.update(id, 1, changed),
            Err(ConfigurationSetError::UnsupportedExecutableView { .. })
        ));
        assert_eq!(catalog, before);

        assert!(matches!(
            catalog.update(id, 2, definition("Release updated")),
            Err(ConfigurationSetError::RevisionConflict { .. })
        ));
        assert_eq!(catalog, before);
    }
}
