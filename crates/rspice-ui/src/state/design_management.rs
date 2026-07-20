//! Durable design-management authority for schematic sheets, assembly
//! variants, reference annotation, and hierarchy preflight evidence.
//!
//! The UI only edits drafts. Every mutation in this module is applied to a
//! cloned candidate and committed after complete validation, so malformed or
//! stale dialog input cannot partially change the project. Stable identities,
//! revisions, semantic digests, and immutable receipts are owned here rather
//! than inferred from labels rendered by the workbench.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

use crate::product::ContentDigest;

use super::{ConfigurationSetId, Point};

pub const DESIGN_MANAGEMENT_SCHEMA_VERSION: u16 = 1;
pub const SHEET_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const VARIANT_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const ANNOTATION_STATE_SCHEMA_VERSION: u16 = 1;
pub const MAX_DESIGN_SHEETS: usize = 1_024;
pub const MAX_SHEET_PORTS: usize = 65_536;
pub const MAX_SHEET_OBJECT_ASSIGNMENTS: usize = 1_000_000;
pub const MAX_ASSEMBLY_VARIANTS: usize = 1_024;
pub const MAX_VARIANT_OVERRIDES: usize = 250_000;
pub const MAX_ANNOTATION_RANGES: usize = 16_384;
pub const MAX_ANNOTATION_JOURNAL_ENTRIES: usize = 16_384;
pub const MAX_ANNOTATION_MAPPINGS_PER_ENTRY: usize = 1_000_000;
pub const MAX_ANNOTATION_OBJECT_AUTHORITIES: usize = 1_000_000;
pub const MAX_HIERARCHY_AUDIT_RECEIPTS: usize = 4_096;
pub const MAX_HIERARCHY_AUDIT_SUBJECTS: usize = 250_000;
pub const MAX_HIERARCHY_AUDIT_FINDINGS: usize = 250_000;
pub const MAX_DESIGN_NAME_BYTES: usize = 128;
pub const MAX_DESIGN_PATH_BYTES: usize = 4_096;
pub const MAX_DESIGN_VALUE_BYTES: usize = 8_192;
pub const MAX_PREFIX_BYTES: usize = 16;

macro_rules! stable_uuid_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
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

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

stable_uuid_id!(SheetId);
stable_uuid_id!(CrossSheetPortId);
stable_uuid_id!(AssemblyVariantId);
stable_uuid_id!(AnnotationJournalId);
stable_uuid_id!(HierarchyAuditReceiptId);

/// Project-unique schematic-object identity. Raw object numbers are stable
/// only inside one cell/view buffer, so persisted project policy always pairs
/// them with the canonical owning cell/view key. The transparent string form
/// keeps this type usable as a deterministic JSON map key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SchematicObjectKey(String);

impl SchematicObjectKey {
    pub fn new(cell_view_key: &str, object_id: u64) -> Result<Self, DesignManagementError> {
        require_object_id(object_id)?;
        let cell_view_key = canonical_cell_view_key(cell_view_key)?;
        Ok(Self(format!("{cell_view_key}#{object_id}")))
    }

    #[must_use]
    pub fn cell_view_key(&self) -> &str {
        self.parts()
            .map(|(cell_view_key, _)| cell_view_key)
            .expect("validated schematic object key")
    }

    #[must_use]
    pub fn object_id(&self) -> u64 {
        self.parts()
            .map(|(_, object_id)| object_id)
            .expect("validated schematic object key")
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        let (cell_view_key, object_id) = self
            .parts()
            .ok_or_else(|| DesignManagementError::InvalidSchematicObjectKey(self.0.clone()))?;
        if canonical_cell_view_key(cell_view_key)? != cell_view_key {
            return Err(DesignManagementError::InvalidSchematicObjectKey(
                self.0.clone(),
            ));
        }
        require_object_id(object_id)
    }

    fn parts(&self) -> Option<(&str, u64)> {
        let (cell_view_key, object_id) = self.0.rsplit_once('#')?;
        let object_id = object_id.parse::<u64>().ok()?;
        Some((cell_view_key, object_id))
    }

    fn remap_cell_owner(
        &self,
        source_library: &str,
        source_cell: &str,
        destination_library: &str,
        destination_cell: &str,
    ) -> Result<Option<Self>, DesignManagementError> {
        let [library, cell, view] = cell_view_key_segments(self.cell_view_key())?;
        if library != source_library || cell != source_cell {
            return Ok(None);
        }
        Self::new(
            &format!("{destination_library}/{destination_cell}/{view}"),
            self.object_id(),
        )
        .map(Some)
    }
}

impl fmt::Display for SchematicObjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for SchematicObjectKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetTemplate {
    #[default]
    AnalogSchematic,
    MixedSignalSchematic,
    BlankGovernedSheet,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetPortPolicy {
    #[default]
    TypedOffSheetPorts,
    NoAutomaticPorts,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OffSheetConnectorPolicy {
    #[default]
    TypedPortsWithExplicitDirection,
    NamedConnectorsCompatibility,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetPageNumbering {
    #[default]
    StableProjectOrder,
    PerPrintSet,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetDeleteBehavior {
    #[default]
    BlockWhileReferenced,
    MoveReferencesToReviewedReplacement,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReorderPageNumbering {
    #[default]
    UpdatePrintPageNumbers,
    RetainExplicitPageNumbers,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReorderCrossReferences {
    #[default]
    UpdateDisplayOnlyStableIdsRetained,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CrossSheetPortDirection {
    Input,
    Output,
    #[default]
    InOut,
    Supply,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CrossSheetSignalType {
    Logic,
    #[default]
    Analog,
    Power,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CrossSheetDiscipline {
    #[default]
    Electrical,
    Logic,
    Wreal,
    Thermal,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetCatalogSettings {
    pub connector_policy: OffSheetConnectorPolicy,
    pub page_numbering: SheetPageNumbering,
    pub delete_behavior: SheetDeleteBehavior,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetDefinition {
    pub name: String,
    pub template: SheetTemplate,
    pub port_policy: SheetPortPolicy,
    pub explicit_page_number: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignSheet {
    id: SheetId,
    revision: u64,
    semantic_digest: ContentDigest,
    definition: SheetDefinition,
}

impl DesignSheet {
    #[must_use]
    pub const fn id(&self) -> SheetId {
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
    pub const fn definition(&self) -> &SheetDefinition {
        &self.definition
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.definition.name
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "sheet")?;
        require_nonzero_revision(self.revision, "sheet", self.id.to_string())?;
        validate_sheet_definition(&self.definition)?;
        require_digest(
            self.semantic_digest,
            digest("rspice-design-sheet-semantic/v1", &self.definition)?,
            "sheet",
            self.id.to_string(),
        )
    }
}

/// Durable attachment point for one side of a cross-sheet port.
///
/// An object identity alone is insufficient: a component may expose several
/// terminals, and a wire may contain several geometrically distinct points.
/// Keeping that information in the contract prevents materializers from
/// guessing a component origin or an arbitrary wire vertex.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum CrossSheetPortAnchor {
    WirePoint {
        wire_id: u64,
        point: Point,
    },
    ComponentTerminal {
        component_id: u64,
        terminal_name: String,
    },
}

impl CrossSheetPortAnchor {
    #[must_use]
    pub const fn object_id(&self) -> u64 {
        match self {
            Self::WirePoint { wire_id, .. } => *wire_id,
            Self::ComponentTerminal { component_id, .. } => *component_id,
        }
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_object_id(self.object_id())?;
        if let Self::ComponentTerminal { terminal_name, .. } = self {
            validate_name("component terminal", terminal_name)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossSheetPortEndpoint {
    pub sheet_id: SheetId,
    pub anchor: CrossSheetPortAnchor,
}

impl CrossSheetPortEndpoint {
    #[must_use]
    pub const fn object_id(&self) -> u64 {
        self.anchor.object_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossSheetPortDefinition {
    pub net_name: String,
    pub first: CrossSheetPortEndpoint,
    pub second: CrossSheetPortEndpoint,
    pub direction: CrossSheetPortDirection,
    pub signal_type: CrossSheetSignalType,
    pub discipline: CrossSheetDiscipline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossSheetPortContract {
    id: CrossSheetPortId,
    revision: u64,
    semantic_digest: ContentDigest,
    definition: CrossSheetPortDefinition,
}

impl CrossSheetPortContract {
    #[must_use]
    pub const fn id(&self) -> CrossSheetPortId {
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
    pub const fn definition(&self) -> &CrossSheetPortDefinition {
        &self.definition
    }

    fn validate(
        &self,
        sheet_ids: &HashSet<SheetId>,
        object_assignments: &BTreeMap<u64, SheetId>,
    ) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "cross-sheet port")?;
        require_nonzero_revision(self.revision, "cross-sheet port", self.id.to_string())?;
        validate_cross_sheet_port_definition(&self.definition, sheet_ids)?;
        for endpoint in [&self.definition.first, &self.definition.second] {
            let actual = object_assignments.get(&endpoint.object_id()).copied();
            if actual != Some(endpoint.sheet_id) {
                return Err(DesignManagementError::CrossSheetPortAnchorSheetMismatch {
                    object_id: endpoint.object_id(),
                    expected: endpoint.sheet_id,
                    actual,
                });
            }
        }
        require_digest(
            self.semantic_digest,
            digest("rspice-cross-sheet-port-semantic/v1", &self.definition)?,
            "cross-sheet port",
            self.id.to_string(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveSelectionRequest {
    pub expected_catalog_revision: u64,
    pub object_ids: Vec<u64>,
    pub destination_sheet_id: SheetId,
    pub boundary_resolution: MoveBoundaryResolution,
}

/// Reviewed result of the canonical connectivity scan performed before a
/// sheet move. An empty vector cannot ambiguously mean either "not scanned"
/// or "no crossings": the latter must be stated explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum MoveBoundaryResolution {
    VerifiedNoBoundaryNets,
    ExplicitPorts {
        ports: Vec<CrossSheetPortDefinition>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveSelectionReceipt {
    pub catalog_revision: u64,
    pub source_sheet_id: SheetId,
    pub destination_sheet_id: SheetId,
    pub object_ids: Vec<u64>,
    pub created_port_ids: Vec<CrossSheetPortId>,
    pub boundary_resolution_digest: ContentDigest,
    pub semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SheetReconcileReceipt {
    pub catalog_revision: u64,
    pub added_assignments: usize,
    pub removed_assignments: usize,
    pub removed_cross_sheet_ports: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SheetCatalog {
    schema_version: u16,
    revision: u64,
    settings: SheetCatalogSettings,
    sheets: Vec<DesignSheet>,
    active_sheet_id: Option<SheetId>,
    object_assignments: BTreeMap<u64, SheetId>,
    cross_sheet_ports: Vec<CrossSheetPortContract>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SheetCatalogWire {
    schema_version: u16,
    revision: u64,
    settings: SheetCatalogSettings,
    #[serde(default)]
    sheets: Vec<DesignSheet>,
    #[serde(default)]
    active_sheet_id: Option<SheetId>,
    #[serde(default)]
    object_assignments: BTreeMap<u64, SheetId>,
    #[serde(default)]
    cross_sheet_ports: Vec<CrossSheetPortContract>,
}

impl<'de> Deserialize<'de> for SheetCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SheetCatalogWire::deserialize(deserializer)?;
        let value = Self {
            schema_version: wire.schema_version,
            revision: wire.revision,
            settings: wire.settings,
            sheets: wire.sheets,
            active_sheet_id: wire.active_sheet_id,
            object_assignments: wire.object_assignments,
            cross_sheet_ports: wire.cross_sheet_ports,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Default for SheetCatalog {
    fn default() -> Self {
        Self {
            schema_version: SHEET_CATALOG_SCHEMA_VERSION,
            revision: 1,
            settings: SheetCatalogSettings::default(),
            sheets: Vec::new(),
            active_sheet_id: None,
            object_assignments: BTreeMap::new(),
            cross_sheet_ports: Vec::new(),
        }
    }
}

impl SheetCatalog {
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn settings(&self) -> &SheetCatalogSettings {
        &self.settings
    }

    #[must_use]
    pub fn sheets(&self) -> &[DesignSheet] {
        &self.sheets
    }

    #[must_use]
    pub const fn active_sheet_id(&self) -> Option<SheetId> {
        self.active_sheet_id
    }

    #[must_use]
    pub fn object_assignments(&self) -> &BTreeMap<u64, SheetId> {
        &self.object_assignments
    }

    #[must_use]
    pub fn cross_sheet_ports(&self) -> &[CrossSheetPortContract] {
        &self.cross_sheet_ports
    }

    #[must_use]
    pub fn find(&self, id: SheetId) -> Option<&DesignSheet> {
        self.sheets.iter().find(|sheet| sheet.id == id)
    }

    #[must_use]
    pub fn active(&self) -> Option<&DesignSheet> {
        self.active_sheet_id.and_then(|id| self.find(id))
    }

    #[must_use]
    pub fn sheet_for_object(&self, object_id: u64) -> Option<SheetId> {
        self.object_assignments.get(&object_id).copied()
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.schema_version != SHEET_CATALOG_SCHEMA_VERSION {
            return Err(DesignManagementError::UnsupportedSchema {
                domain: "sheet catalog",
                actual: self.schema_version,
            });
        }
        require_nonzero_revision(self.revision, "sheet catalog", "catalog".to_owned())?;
        require_limit("sheets", self.sheets.len(), MAX_DESIGN_SHEETS)?;
        require_limit(
            "sheet object assignments",
            self.object_assignments.len(),
            MAX_SHEET_OBJECT_ASSIGNMENTS,
        )?;
        require_limit(
            "cross-sheet ports",
            self.cross_sheet_ports.len(),
            MAX_SHEET_PORTS,
        )?;

        let mut sheet_ids = HashSet::with_capacity(self.sheets.len());
        let mut names = HashSet::with_capacity(self.sheets.len());
        let mut page_numbers = HashSet::new();
        for sheet in &self.sheets {
            sheet.validate()?;
            if !sheet_ids.insert(sheet.id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "sheet",
                    identity: sheet.id.to_string(),
                });
            }
            if !names.insert(case_fold(sheet.name())) {
                return Err(DesignManagementError::DuplicateName {
                    domain: "sheet",
                    name: sheet.name().to_owned(),
                });
            }
            if let Some(page) = sheet.definition.explicit_page_number
                && !page_numbers.insert(page)
            {
                return Err(DesignManagementError::DuplicateSheetPage(page));
            }
        }

        match self.active_sheet_id {
            Some(id) if !sheet_ids.contains(&id) => {
                return Err(DesignManagementError::MissingReference {
                    domain: "active sheet",
                    identity: id.to_string(),
                });
            }
            None if !self.sheets.is_empty() => {
                return Err(DesignManagementError::ActiveSelectionRequired("sheet"));
            }
            _ => {}
        }
        for (&object_id, sheet_id) in &self.object_assignments {
            require_object_id(object_id)?;
            if !sheet_ids.contains(sheet_id) {
                return Err(DesignManagementError::MissingReference {
                    domain: "object sheet",
                    identity: sheet_id.to_string(),
                });
            }
        }
        let mut port_ids = HashSet::with_capacity(self.cross_sheet_ports.len());
        let mut endpoint_pairs = HashSet::with_capacity(self.cross_sheet_ports.len());
        for port in &self.cross_sheet_ports {
            port.validate(&sheet_ids, &self.object_assignments)?;
            if !port_ids.insert(port.id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "cross-sheet port",
                    identity: port.id.to_string(),
                });
            }
            let key = canonical_port_key(&port.definition)?;
            if !endpoint_pairs.insert(key) {
                return Err(DesignManagementError::DuplicateCrossSheetPort);
            }
        }
        Ok(())
    }

    pub fn create_sheet(
        &mut self,
        definition: SheetDefinition,
        insert_after: Option<SheetId>,
    ) -> Result<SheetId, DesignManagementError> {
        self.validate()?;
        require_limit("sheets", self.sheets.len() + 1, MAX_DESIGN_SHEETS)?;
        let definition = normalize_sheet_definition(definition);
        validate_sheet_definition(&definition)?;
        if self
            .sheets
            .iter()
            .any(|sheet| case_fold(sheet.name()) == case_fold(&definition.name))
        {
            return Err(DesignManagementError::DuplicateName {
                domain: "sheet",
                name: definition.name,
            });
        }
        let insertion = match insert_after {
            Some(id) => {
                self.sheets
                    .iter()
                    .position(|sheet| sheet.id == id)
                    .ok_or_else(|| DesignManagementError::MissingReference {
                        domain: "insert-after sheet",
                        identity: id.to_string(),
                    })?
                    + 1
            }
            None => self.sheets.len(),
        };
        let id = SheetId::new();
        let sheet = DesignSheet {
            id,
            revision: 1,
            semantic_digest: digest("rspice-design-sheet-semantic/v1", &definition)?,
            definition,
        };
        let mut candidate = self.clone();
        candidate.sheets.insert(insertion, sheet);
        candidate.active_sheet_id.get_or_insert(id);
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        *self = candidate;
        Ok(id)
    }

    pub fn update_sheet(
        &mut self,
        id: SheetId,
        expected_sheet_revision: u64,
        definition: SheetDefinition,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        let definition = normalize_sheet_definition(definition);
        validate_sheet_definition(&definition)?;
        if self
            .sheets
            .iter()
            .any(|sheet| sheet.id != id && case_fold(sheet.name()) == case_fold(&definition.name))
        {
            return Err(DesignManagementError::DuplicateName {
                domain: "sheet",
                name: definition.name,
            });
        }
        let mut candidate = self.clone();
        let sheet = candidate
            .sheets
            .iter_mut()
            .find(|sheet| sheet.id == id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "sheet",
                identity: id.to_string(),
            })?;
        require_revision(
            expected_sheet_revision,
            sheet.revision,
            "sheet",
            id.to_string(),
        )?;
        if sheet.definition == definition {
            return Err(DesignManagementError::NoChanges("sheet"));
        }
        sheet.revision = next_revision(sheet.revision, "sheet", id.to_string())?;
        sheet.semantic_digest = digest("rspice-design-sheet-semantic/v1", &definition)?;
        sheet.definition = definition;
        let committed_revision = sheet.revision;
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        *self = candidate;
        Ok(committed_revision)
    }

    pub fn set_settings(
        &mut self,
        expected_catalog_revision: u64,
        settings: SheetCatalogSettings,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        if self.settings == settings {
            return Err(DesignManagementError::NoChanges("sheet catalog settings"));
        }
        let mut candidate = self.clone();
        candidate.settings = settings;
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        let revision = candidate.revision;
        *self = candidate;
        Ok(revision)
    }

    pub fn set_active(&mut self, id: SheetId) -> Result<(), DesignManagementError> {
        self.validate()?;
        if self.find(id).is_none() {
            return Err(DesignManagementError::MissingReference {
                domain: "sheet",
                identity: id.to_string(),
            });
        }
        self.active_sheet_id = Some(id);
        Ok(())
    }

    pub fn reorder(
        &mut self,
        expected_catalog_revision: u64,
        ordered_ids: Vec<SheetId>,
        page_numbering: ReorderPageNumbering,
        _cross_references: ReorderCrossReferences,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        if ordered_ids.len() != self.sheets.len()
            || ordered_ids.iter().copied().collect::<HashSet<_>>().len() != self.sheets.len()
            || ordered_ids.iter().any(|id| self.find(*id).is_none())
        {
            return Err(DesignManagementError::InvalidSheetOrder);
        }
        if self
            .sheets
            .iter()
            .map(|sheet| sheet.id)
            .eq(ordered_ids.iter().copied())
            && page_numbering == ReorderPageNumbering::RetainExplicitPageNumbers
        {
            return Err(DesignManagementError::NoChanges("sheet order"));
        }
        let by_id = self
            .sheets
            .iter()
            .cloned()
            .map(|sheet| (sheet.id, sheet))
            .collect::<BTreeMap<_, _>>();
        let mut candidate = self.clone();
        candidate.sheets = ordered_ids
            .iter()
            .map(|id| by_id.get(id).cloned().expect("order was validated"))
            .collect();
        if page_numbering == ReorderPageNumbering::UpdatePrintPageNumbers {
            for (index, sheet) in candidate.sheets.iter_mut().enumerate() {
                let page = u32::try_from(index + 1)
                    .map_err(|_| DesignManagementError::NumericRange("sheet page number"))?;
                if sheet.definition.explicit_page_number != Some(page) {
                    sheet.definition.explicit_page_number = Some(page);
                    sheet.revision = next_revision(sheet.revision, "sheet", sheet.id.to_string())?;
                    sheet.semantic_digest =
                        digest("rspice-design-sheet-semantic/v1", &sheet.definition)?;
                }
            }
        }
        if candidate.sheets == self.sheets {
            return Err(DesignManagementError::NoChanges("sheet order"));
        }
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        let revision = candidate.revision;
        *self = candidate;
        Ok(revision)
    }

    pub fn assign_objects(
        &mut self,
        expected_catalog_revision: u64,
        sheet_id: SheetId,
        object_ids: impl IntoIterator<Item = u64>,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        if self.find(sheet_id).is_none() {
            return Err(DesignManagementError::MissingReference {
                domain: "sheet",
                identity: sheet_id.to_string(),
            });
        }
        let ids = unique_object_ids(object_ids)?;
        if ids.is_empty() {
            return Err(DesignManagementError::EmptySelection);
        }
        let mut candidate = self.clone();
        let mut changed = false;
        for id in ids {
            changed |= candidate.object_assignments.insert(id, sheet_id) != Some(sheet_id);
        }
        if !changed {
            return Err(DesignManagementError::NoChanges("sheet object assignments"));
        }
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        let revision = candidate.revision;
        *self = candidate;
        Ok(revision)
    }

    /// Reconcile persisted membership against the owning schematic's live
    /// stable object identities. This is the only cleanup path used after
    /// undo, delete, or legacy migration: missing live objects are assigned
    /// deterministically, deleted objects are removed, and a port whose
    /// physical anchor disappeared is removed in the same atomic commit.
    pub fn reconcile_object_assignments(
        &mut self,
        expected_catalog_revision: u64,
        live_object_ids: impl IntoIterator<Item = u64>,
        default_sheet_id: Option<SheetId>,
    ) -> Result<SheetReconcileReceipt, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        let live = unique_object_ids(live_object_ids)?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let destination = default_sheet_id
            .or(self.active_sheet_id)
            .or_else(|| self.sheets.first().map(|sheet| sheet.id));
        if !live.is_empty() && destination.is_none() {
            return Err(DesignManagementError::ActiveSelectionRequired("sheet"));
        }
        if let Some(id) = destination
            && self.find(id).is_none()
        {
            return Err(DesignManagementError::MissingReference {
                domain: "default sheet",
                identity: id.to_string(),
            });
        }

        let mut candidate = self.clone();
        let assignments_before = candidate.object_assignments.len();
        candidate
            .object_assignments
            .retain(|object_id, _| live.contains(object_id));
        let removed_assignments = assignments_before - candidate.object_assignments.len();
        let mut added_assignments = 0;
        if let Some(destination) = destination {
            for object_id in &live {
                if !candidate.object_assignments.contains_key(object_id) {
                    candidate.object_assignments.insert(*object_id, destination);
                    added_assignments += 1;
                }
            }
        }
        let ports_before = candidate.cross_sheet_ports.len();
        candidate.cross_sheet_ports.retain(|port| {
            live.contains(&port.definition.first.object_id())
                && live.contains(&port.definition.second.object_id())
        });
        let removed_cross_sheet_ports = ports_before - candidate.cross_sheet_ports.len();
        if added_assignments + removed_assignments + removed_cross_sheet_ports > 0 {
            candidate.bump_revision("sheet catalog")?;
            candidate.validate()?;
            *self = candidate;
        }
        Ok(SheetReconcileReceipt {
            catalog_revision: self.revision,
            added_assignments,
            removed_assignments,
            removed_cross_sheet_ports,
        })
    }

    pub fn move_selection(
        &mut self,
        request: MoveSelectionRequest,
    ) -> Result<MoveSelectionReceipt, DesignManagementError> {
        self.validate()?;
        require_revision(
            request.expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        if self.find(request.destination_sheet_id).is_none() {
            return Err(DesignManagementError::MissingReference {
                domain: "destination sheet",
                identity: request.destination_sheet_id.to_string(),
            });
        }
        let object_ids = unique_object_ids(request.object_ids)?;
        if object_ids.is_empty() {
            return Err(DesignManagementError::EmptySelection);
        }
        let source_ids = object_ids
            .iter()
            .map(|id| {
                self.object_assignments
                    .get(id)
                    .copied()
                    .ok_or(DesignManagementError::UnassignedSchematicObject(*id))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if source_ids.len() != 1 {
            return Err(DesignManagementError::MixedSourceSheets);
        }
        let source_sheet_id = *source_ids.iter().next().expect("one source was checked");
        if source_sheet_id == request.destination_sheet_id {
            return Err(DesignManagementError::NoChanges("sheet move"));
        }
        let boundary_ports = match &request.boundary_resolution {
            MoveBoundaryResolution::VerifiedNoBoundaryNets => &[][..],
            MoveBoundaryResolution::ExplicitPorts { ports } if ports.is_empty() => {
                return Err(DesignManagementError::EmptyExplicitBoundaryPorts);
            }
            MoveBoundaryResolution::ExplicitPorts { ports } => ports.as_slice(),
        };
        require_limit(
            "cross-sheet ports",
            self.cross_sheet_ports.len() + boundary_ports.len(),
            MAX_SHEET_PORTS,
        )?;
        let sheet_ids = self
            .sheets
            .iter()
            .map(|sheet| sheet.id)
            .collect::<HashSet<_>>();
        for definition in boundary_ports {
            validate_cross_sheet_port_definition(definition, &sheet_ids)?;
            let endpoints = [definition.first.sheet_id, definition.second.sheet_id];
            if !endpoints.contains(&source_sheet_id)
                || !endpoints.contains(&request.destination_sheet_id)
            {
                return Err(DesignManagementError::BoundaryPortOutsideMove);
            }
        }
        let boundary_resolution_digest = digest(
            "rspice-sheet-move-boundary-resolution-semantic/v1",
            &request.boundary_resolution,
        )?;

        let mut candidate = self.clone();
        for object_id in &object_ids {
            candidate
                .object_assignments
                .insert(*object_id, request.destination_sheet_id);
        }
        let boundary_ports = match request.boundary_resolution {
            MoveBoundaryResolution::VerifiedNoBoundaryNets => Vec::new(),
            MoveBoundaryResolution::ExplicitPorts { ports } => ports,
        };
        let mut created_port_ids = Vec::with_capacity(boundary_ports.len());
        for definition in boundary_ports {
            let id = CrossSheetPortId::new();
            candidate.cross_sheet_ports.push(CrossSheetPortContract {
                id,
                revision: 1,
                semantic_digest: digest("rspice-cross-sheet-port-semantic/v1", &definition)?,
                definition,
            });
            created_port_ids.push(id);
        }
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        let mut receipt = MoveSelectionReceipt {
            catalog_revision: candidate.revision,
            source_sheet_id,
            destination_sheet_id: request.destination_sheet_id,
            object_ids,
            created_port_ids,
            boundary_resolution_digest,
            semantic_digest: empty_digest(),
        };
        receipt.semantic_digest = digest(
            "rspice-sheet-move-receipt-semantic/v1",
            &MoveSelectionReceiptMaterial::from(&receipt),
        )?;
        *self = candidate;
        Ok(receipt)
    }

    fn bump_revision(&mut self, domain: &'static str) -> Result<(), DesignManagementError> {
        self.revision = next_revision(self.revision, domain, "catalog".to_owned())?;
        Ok(())
    }
}

#[derive(Serialize)]
struct MoveSelectionReceiptMaterial<'a> {
    catalog_revision: u64,
    source_sheet_id: SheetId,
    destination_sheet_id: SheetId,
    object_ids: &'a [u64],
    created_port_ids: &'a [CrossSheetPortId],
    boundary_resolution_digest: ContentDigest,
}

impl<'a> From<&'a MoveSelectionReceipt> for MoveSelectionReceiptMaterial<'a> {
    fn from(receipt: &'a MoveSelectionReceipt) -> Self {
        Self {
            catalog_revision: receipt.catalog_revision,
            source_sheet_id: receipt.source_sheet_id,
            destination_sheet_id: receipt.destination_sheet_id,
            object_ids: &receipt.object_ids,
            created_port_ids: &receipt.created_port_ids,
            boundary_resolution_digest: receipt.boundary_resolution_digest,
        }
    }
}

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
    id: AssemblyVariantId,
    revision: u64,
    semantic_digest: ContentDigest,
    definition: AssemblyVariantDefinition,
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
    AddedOverride,
    RemovedOverride,
    ChangedOverride,
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
    schema_version: u16,
    #[serde(default)]
    settings: AssemblyVariantSettings,
    variants: Vec<AssemblyVariant>,
    active_variant_id: Option<AssemblyVariantId>,
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
                    (None, Some(_)) => VariantDifferenceKind::AddedOverride,
                    (Some(_), None) => VariantDifferenceKind::RemovedOverride,
                    (Some(_), Some(_)) => VariantDifferenceKind::ChangedOverride,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
    revision: u64,
    semantic_digest: ContentDigest,
    definition: AnnotationPolicyDefinition,
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
    schema_version: u16,
    policy: AnnotationPolicy,
    journal: Vec<AnnotationJournalEntry>,
    object_authorities: BTreeMap<SchematicObjectKey, AnnotationObjectAuthority>,
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

    fn remap_object_owners(
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

    fn tombstone_objects(
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EditInPlaceDepth {
    #[default]
    CurrentAndParentContext,
    CurrentOnly,
    TwoParentLevels,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissingHierarchyViewPolicy {
    #[default]
    BlockNetlist,
    UseDeclaredFallbackOrder,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyBlackBoxPolicy {
    #[default]
    RequireSignedBoundaryContract,
    AllowProjectAbstract,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyCyclePolicy {
    #[default]
    BlockSaveAndIdentifyPath,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyManagementSettings {
    pub edit_in_place_depth: EditInPlaceDepth,
    pub missing_view: MissingHierarchyViewPolicy,
    pub black_box: HierarchyBlackBoxPolicy,
    pub cycle_detection: HierarchyCyclePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HierarchyAuditConfiguration {
    ActiveProject,
    ConfigurationSet {
        id: ConfigurationSetId,
        revision: u64,
        semantic_digest: ContentDigest,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyViewChecks {
    #[default]
    AllDeclaredFallbacks,
    SelectedHierarchy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectedBoundaryChecks {
    #[default]
    ValidateSignaturesAndPins,
    PinsOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditSubject {
    pub instance_path: String,
    pub cell_name: String,
    pub design_view: String,
    pub declared_fallbacks: Vec<String>,
    pub resolved_simulation_view: Option<String>,
    pub fallback_used: Option<String>,
    pub child_instance_paths: Vec<String>,
    pub protected_boundary_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedBoundaryEvidence {
    pub boundary_id: String,
    pub signature_valid: bool,
    pub pins_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditRequest {
    pub configuration: HierarchyAuditConfiguration,
    pub view_checks: HierarchyViewChecks,
    pub protected_boundaries: ProtectedBoundaryChecks,
    pub subjects: Vec<HierarchyAuditSubject>,
    pub boundary_evidence: Vec<ProtectedBoundaryEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyAuditFindingKind {
    UnresolvedView,
    UndeclaredFallback,
    MissingChild,
    HierarchyCycle,
    MissingProtectedBoundaryEvidence,
    InvalidProtectedBoundarySignature,
    ProtectedBoundaryPinMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditFinding {
    pub kind: HierarchyAuditFindingKind,
    pub instance_path: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditReceipt {
    id: HierarchyAuditReceiptId,
    sequence: u64,
    request_digest: ContentDigest,
    resolved_subjects: usize,
    findings: Vec<HierarchyAuditFinding>,
    semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignManagementOwnershipReceipt {
    pub catalog_revision: u64,
    pub affected_sheet_catalogs: usize,
    pub remapped_variant_objects: usize,
    pub remapped_annotation_objects: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignManagementCopyReceipt {
    pub catalog_revision: u64,
    pub copied_sheet_catalogs: usize,
    pub sheet_identity_map: BTreeMap<SheetId, SheetId>,
    pub port_identity_map: BTreeMap<CrossSheetPortId, CrossSheetPortId>,
}

impl HierarchyAuditReceipt {
    #[must_use]
    pub const fn id(&self) -> HierarchyAuditReceiptId {
        self.id
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn request_digest(&self) -> ContentDigest {
        self.request_digest
    }

    #[must_use]
    pub const fn resolved_subjects(&self) -> usize {
        self.resolved_subjects
    }

    #[must_use]
    pub fn findings(&self) -> &[HierarchyAuditFinding] {
        &self.findings
    }

    #[must_use]
    pub fn passed(&self) -> bool {
        self.findings.is_empty()
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "hierarchy audit receipt")?;
        require_nonzero_revision(
            self.sequence,
            "hierarchy audit receipt",
            self.id.to_string(),
        )?;
        require_limit(
            "hierarchy audit findings",
            self.findings.len(),
            MAX_HIERARCHY_AUDIT_FINDINGS,
        )?;
        for finding in &self.findings {
            validate_path("hierarchy audit finding path", &finding.instance_path)?;
            validate_value("hierarchy audit finding detail", &finding.detail, false)?;
        }
        require_digest(
            self.semantic_digest,
            digest(
                "rspice-hierarchy-audit-receipt-semantic/v1",
                &HierarchyAuditReceiptMaterial::from(self),
            )?,
            "hierarchy audit receipt",
            self.id.to_string(),
        )
    }
}

#[derive(Serialize)]
struct HierarchyAuditReceiptMaterial<'a> {
    id: HierarchyAuditReceiptId,
    sequence: u64,
    request_digest: ContentDigest,
    resolved_subjects: usize,
    findings: &'a [HierarchyAuditFinding],
}

impl<'a> From<&'a HierarchyAuditReceipt> for HierarchyAuditReceiptMaterial<'a> {
    fn from(receipt: &'a HierarchyAuditReceipt) -> Self {
        Self {
            id: receipt.id,
            sequence: receipt.sequence,
            request_digest: receipt.request_digest,
            resolved_subjects: receipt.resolved_subjects,
            findings: &receipt.findings,
        }
    }
}

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

fn normalize_sheet_definition(mut definition: SheetDefinition) -> SheetDefinition {
    definition.name = normalize_text(&definition.name);
    definition
}

fn validate_sheet_definition(definition: &SheetDefinition) -> Result<(), DesignManagementError> {
    validate_name("sheet name", &definition.name)?;
    if definition.explicit_page_number == Some(0) {
        return Err(DesignManagementError::NumericRange("sheet page number"));
    }
    Ok(())
}

fn validate_cross_sheet_port_definition(
    definition: &CrossSheetPortDefinition,
    sheet_ids: &HashSet<SheetId>,
) -> Result<(), DesignManagementError> {
    validate_name("cross-sheet net name", &definition.net_name)?;
    definition.first.anchor.validate()?;
    definition.second.anchor.validate()?;
    if definition.first.sheet_id == definition.second.sheet_id {
        return Err(DesignManagementError::CrossSheetPortSameSheet);
    }
    for endpoint in [&definition.first, &definition.second] {
        if !sheet_ids.contains(&endpoint.sheet_id) {
            return Err(DesignManagementError::MissingReference {
                domain: "cross-sheet port endpoint sheet",
                identity: endpoint.sheet_id.to_string(),
            });
        }
    }
    match (definition.signal_type, definition.discipline) {
        (CrossSheetSignalType::Logic, CrossSheetDiscipline::Logic)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Electrical)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Wreal)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Thermal)
        | (CrossSheetSignalType::Power, CrossSheetDiscipline::Electrical)
        | (CrossSheetSignalType::Power, CrossSheetDiscipline::Thermal) => Ok(()),
        _ => Err(DesignManagementError::IncompatiblePortContract),
    }
}

fn canonical_port_key(
    definition: &CrossSheetPortDefinition,
) -> Result<(SheetId, ContentDigest, SheetId, ContentDigest, String), DesignManagementError> {
    let first = (
        definition.first.sheet_id,
        digest(
            "rspice-cross-sheet-port-anchor-semantic/v1",
            &definition.first.anchor,
        )?,
    );
    let second = (
        definition.second.sheet_id,
        digest(
            "rspice-cross-sheet-port-anchor-semantic/v1",
            &definition.second.anchor,
        )?,
    );
    let (first, second) = if first <= second {
        (first, second)
    } else {
        (second, first)
    };
    Ok((
        first.0,
        first.1,
        second.0,
        second.1,
        case_fold(&definition.net_name),
    ))
}

fn normalize_variant_definition(
    mut definition: AssemblyVariantDefinition,
) -> AssemblyVariantDefinition {
    definition.name = normalize_text(&definition.name);
    for value in definition.overrides.values_mut() {
        match value {
            VariantObjectOverride::Substitute { replacement } => {
                replacement.library = normalize_text(&replacement.library);
                replacement.cell = normalize_text(&replacement.cell);
                replacement.view = normalize_text(&replacement.view);
                replacement.value_override = replacement
                    .value_override
                    .take()
                    .map(|value| normalize_text(&value));
                replacement.model_section = replacement
                    .model_section
                    .take()
                    .map(|value| normalize_text(&value));
            }
            VariantObjectOverride::DoNotPopulate { approval_reference } => {
                *approval_reference = normalize_text(approval_reference);
            }
        }
    }
    definition
}

fn remap_variant_object_owners(
    catalog: &mut AssemblyVariantCatalog,
    source_library: &str,
    source_cell: &str,
    destination_library: &str,
    destination_cell: &str,
) -> Result<usize, DesignManagementError> {
    catalog.validate()?;
    let original = catalog.clone();
    let mut ids = original
        .variants
        .iter()
        .map(|variant| {
            let mut depth = 0usize;
            let mut cursor = variant.definition.parent.as_ref().map(|parent| parent.id);
            while let Some(id) = cursor {
                depth = depth
                    .checked_add(1)
                    .ok_or(DesignManagementError::NumericRange("variant parent depth"))?;
                cursor = original
                    .find(id)
                    .and_then(|parent| parent.definition.parent.as_ref().map(|entry| entry.id));
            }
            Ok((depth, variant.id))
        })
        .collect::<Result<Vec<_>, DesignManagementError>>()?;
    ids.sort_by_key(|(depth, id)| (*depth, *id));
    let mut rebuilt: BTreeMap<AssemblyVariantId, AssemblyVariant> = BTreeMap::new();
    let mut remapped_objects = 0usize;
    for (_, id) in ids {
        let source = original.find(id).expect("variant catalog was validated");
        let mut definition = source.definition.clone();
        let mut overrides = BTreeMap::new();
        for (object, value) in &definition.overrides {
            let target = object
                .remap_cell_owner(
                    source_library,
                    source_cell,
                    destination_library,
                    destination_cell,
                )?
                .unwrap_or_else(|| object.clone());
            if &target != object {
                remapped_objects += 1;
            }
            if overrides.insert(target.clone(), value.clone()).is_some() {
                return Err(DesignManagementError::DuplicateScopedSchematicObject(
                    target,
                ));
            }
        }
        definition.overrides = overrides;
        if let Some(parent) = &mut definition.parent {
            let rebuilt_parent = rebuilt
                .get(&parent.id)
                .expect("parent variants are rebuilt before their children");
            parent.revision = rebuilt_parent.revision;
            parent.semantic_digest = rebuilt_parent.semantic_digest;
        }
        let mut target = source.clone();
        if definition != source.definition {
            target.revision =
                next_revision(source.revision, "assembly variant", source.id.to_string())?;
            target.semantic_digest = digest("rspice-assembly-variant-semantic/v1", &definition)?;
            target.definition = definition;
        }
        rebuilt.insert(id, target);
    }
    if remapped_objects == 0 {
        return Ok(0);
    }
    catalog.variants = original
        .variants
        .iter()
        .map(|variant| {
            rebuilt
                .remove(&variant.id)
                .expect("every variant was rebuilt")
        })
        .collect();
    catalog.validate()?;
    Ok(remapped_objects)
}

fn validate_variant_definition(
    definition: &AssemblyVariantDefinition,
) -> Result<(), DesignManagementError> {
    validate_name("assembly variant name", &definition.name)?;
    require_limit(
        "assembly variant overrides",
        definition.overrides.len(),
        MAX_VARIANT_OVERRIDES,
    )?;
    if let Some(parent) = &definition.parent {
        require_non_nil(parent.id.as_uuid(), "parent assembly variant")?;
        require_nonzero_revision(
            parent.revision,
            "parent assembly variant",
            parent.id.to_string(),
        )?;
    }
    for (object, value) in &definition.overrides {
        object.validate()?;
        match value {
            VariantObjectOverride::Substitute { replacement } => {
                validate_substitution(replacement)?;
            }
            VariantObjectOverride::DoNotPopulate { approval_reference } => {
                validate_value("DNP approval reference", approval_reference, false)?;
            }
        }
    }
    Ok(())
}

fn validate_substitution(replacement: &ComponentSubstitution) -> Result<(), DesignManagementError> {
    validate_name("replacement library", &replacement.library)?;
    validate_name("replacement cell", &replacement.cell)?;
    validate_name("replacement view", &replacement.view)?;
    if let Some(value) = &replacement.value_override {
        validate_value("replacement value", value, true)?;
    }
    if let Some(section) = &replacement.model_section {
        validate_name("replacement model section", section)?;
    }
    Ok(())
}

fn normalize_annotation_policy_definition(
    mut definition: AnnotationPolicyDefinition,
) -> AnnotationPolicyDefinition {
    for range in &mut definition.reserved_ranges {
        if let AnnotationRangeScope::Hierarchy { path } = &mut range.scope {
            *path = normalize_text(path);
        }
        for prefix in &mut range.prefixes {
            *prefix = normalize_prefix(prefix);
        }
        range.prefixes.sort();
        range.prefixes.dedup();
    }
    definition.reserved_ranges.sort_by(|left, right| {
        left.scope
            .cmp(&right.scope)
            .then_with(|| left.first.cmp(&right.first))
            .then_with(|| left.last.cmp(&right.last))
            .then_with(|| left.prefixes.cmp(&right.prefixes))
    });
    definition
}

fn validate_annotation_policy_definition(
    definition: &AnnotationPolicyDefinition,
) -> Result<(), DesignManagementError> {
    require_limit(
        "annotation reserved ranges",
        definition.reserved_ranges.len(),
        MAX_ANNOTATION_RANGES,
    )?;
    for range in &definition.reserved_ranges {
        if range.first == 0 || range.first > range.last {
            return Err(DesignManagementError::InvalidAnnotationRange {
                first: range.first,
                last: range.last,
            });
        }
        if range.prefixes.is_empty() {
            return Err(DesignManagementError::EmptyAnnotationPrefixes);
        }
        for prefix in &range.prefixes {
            validate_prefix(prefix)?;
        }
        if let AnnotationRangeScope::Hierarchy { path } = &range.scope {
            validate_path("annotation hierarchy range", path)?;
        }
    }
    for (index, left) in definition.reserved_ranges.iter().enumerate() {
        for right in definition.reserved_ranges.iter().skip(index + 1) {
            if left.scope == right.scope
                && left.first <= right.last
                && right.first <= left.last
                && left
                    .prefixes
                    .iter()
                    .any(|prefix| right.prefixes.contains(prefix))
            {
                return Err(DesignManagementError::OverlappingAnnotationRanges);
            }
        }
    }
    Ok(())
}

fn validate_renumber_request(request: &RenumberRequest) -> Result<(), DesignManagementError> {
    require_limit(
        "renumber objects",
        request.objects.len(),
        MAX_ANNOTATION_MAPPINGS_PER_ENTRY,
    )?;
    match &request.scope {
        RenumberScope::CurrentHierarchy { path } => {
            validate_path("renumber hierarchy path", path)?;
        }
        RenumberScope::CurrentSheet { sheet_id } => {
            require_non_nil(sheet_id.as_uuid(), "renumber sheet")?;
        }
        RenumberScope::WholeProject => {}
    }
    let mut ids = HashSet::with_capacity(request.objects.len());
    let mut references = HashSet::with_capacity(request.objects.len());
    for object in &request.objects {
        object.object.validate()?;
        if !ids.insert(object.object.clone()) {
            return Err(DesignManagementError::DuplicateScopedSchematicObject(
                object.object.clone(),
            ));
        }
        validate_reference_designator(&object.current_reference)?;
        validate_name("annotation device family", &object.device_family)?;
        validate_path("annotation hierarchy path", &object.hierarchy_path)?;
        if let Some(sheet_id) = object.sheet_id {
            require_non_nil(sheet_id.as_uuid(), "annotation sheet")?;
        }
        if !references.insert(case_fold(&object.current_reference)) {
            return Err(DesignManagementError::DuplicateReferenceDesignator(
                object.current_reference.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_annotation_mappings(
    mappings: &BTreeMap<SchematicObjectKey, AnnotationMapping>,
) -> Result<(), DesignManagementError> {
    let mut new_refs = HashSet::with_capacity(mappings.len());
    for (object, mapping) in mappings {
        object.validate()?;
        validate_reference_designator(&mapping.old_reference)?;
        validate_reference_designator(&mapping.new_reference)?;
        if !new_refs.insert(case_fold(&mapping.new_reference)) {
            return Err(DesignManagementError::DuplicateReferenceDesignator(
                mapping.new_reference.clone(),
            ));
        }
    }
    Ok(())
}

fn object_in_scope(object: &AnnotationObject, scope: &RenumberScope) -> bool {
    match scope {
        RenumberScope::WholeProject => true,
        RenumberScope::CurrentHierarchy { path } => {
            object.hierarchy_path == *path
                || object
                    .hierarchy_path
                    .strip_prefix(path)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        }
        RenumberScope::CurrentSheet { sheet_id } => object.sheet_id == Some(*sheet_id),
    }
}

fn sort_annotation_objects(objects: &mut [AnnotationObject], order: RenumberOrder) {
    objects.sort_by(|left, right| match order {
        RenumberOrder::HierarchyThenCoordinates => left
            .hierarchy_path
            .cmp(&right.hierarchy_path)
            .then_with(|| left.position.y.cmp(&right.position.y))
            .then_with(|| left.position.x.cmp(&right.position.x))
            .then_with(|| left.object.cmp(&right.object)),
        RenumberOrder::SheetThenCoordinates => left
            .sheet_id
            .cmp(&right.sheet_id)
            .then_with(|| left.position.y.cmp(&right.position.y))
            .then_with(|| left.position.x.cmp(&right.position.x))
            .then_with(|| left.object.cmp(&right.object)),
        RenumberOrder::ConnectivityOrder => left
            .connectivity_order
            .unwrap_or(u64::MAX)
            .cmp(&right.connectivity_order.unwrap_or(u64::MAX))
            .then_with(|| left.object.cmp(&right.object)),
    });
}

fn annotation_prefix(
    object: &AnnotationObject,
    allocation: AnnotationPrefixAllocation,
) -> Result<String, DesignManagementError> {
    let from_reference = object
        .current_reference
        .chars()
        .take_while(|character| character.is_ascii_alphabetic())
        .collect::<String>();
    let candidate = match allocation {
        AnnotationPrefixAllocation::ByDeviceFamily => {
            let family = object
                .device_family
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<String>();
            if family.is_empty() {
                from_reference
            } else {
                family
            }
        }
        AnnotationPrefixAllocation::BySheet | AnnotationPrefixAllocation::ByHierarchy => {
            from_reference
        }
    };
    let prefix = normalize_prefix(&candidate);
    validate_prefix(&prefix)?;
    Ok(prefix)
}

fn matching_annotation_ranges<'a>(
    ranges: &'a [AnnotationReservedRange],
    object: &AnnotationObject,
    prefix: &str,
) -> Vec<&'a AnnotationReservedRange> {
    let mut matches = ranges
        .iter()
        .filter(|range| {
            range.prefixes.iter().any(|entry| entry == prefix)
                && match &range.scope {
                    AnnotationRangeScope::Project => true,
                    AnnotationRangeScope::Sheet { sheet_id } => object.sheet_id == Some(*sheet_id),
                    AnnotationRangeScope::Hierarchy { path } => {
                        object.hierarchy_path == *path
                            || object
                                .hierarchy_path
                                .strip_prefix(path)
                                .is_some_and(|suffix| suffix.starts_with('/'))
                    }
                }
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        annotation_scope_specificity(&right.scope)
            .cmp(&annotation_scope_specificity(&left.scope))
            .then_with(|| left.first.cmp(&right.first))
    });
    matches
}

fn annotation_scope_specificity(scope: &AnnotationRangeScope) -> u8 {
    match scope {
        AnnotationRangeScope::Project => 0,
        AnnotationRangeScope::Sheet { .. } => 1,
        AnnotationRangeScope::Hierarchy { .. } => 2,
    }
}

fn allocate_reference(
    prefix: &str,
    ranges: &[&AnnotationReservedRange],
    occupied: &HashSet<String>,
) -> Result<String, DesignManagementError> {
    if ranges.is_empty() {
        for number in 1..=u32::MAX {
            let candidate = format!("{prefix}{number}");
            if !occupied.contains(&case_fold(&candidate)) {
                return Ok(candidate);
            }
        }
    } else {
        for range in ranges {
            for number in range.first..=range.last {
                let candidate = format!("{prefix}{number}");
                if !occupied.contains(&case_fold(&candidate)) {
                    return Ok(candidate);
                }
            }
        }
    }
    Err(DesignManagementError::AnnotationRangeExhausted(
        prefix.to_owned(),
    ))
}

fn validate_hierarchy_audit_request(
    request: &HierarchyAuditRequest,
) -> Result<(), DesignManagementError> {
    if let HierarchyAuditConfiguration::ConfigurationSet { id, revision, .. } =
        &request.configuration
    {
        if id.is_nil() {
            return Err(DesignManagementError::NilIdentity(
                "hierarchy audit configuration",
            ));
        }
        require_nonzero_revision(*revision, "hierarchy audit configuration", id.to_string())?;
    }
    require_limit(
        "hierarchy audit subjects",
        request.subjects.len(),
        MAX_HIERARCHY_AUDIT_SUBJECTS,
    )?;
    if request.subjects.is_empty() {
        return Err(DesignManagementError::EmptyHierarchyAudit);
    }
    let mut paths = HashSet::with_capacity(request.subjects.len());
    for subject in &request.subjects {
        validate_path("hierarchy instance path", &subject.instance_path)?;
        validate_name("hierarchy cell name", &subject.cell_name)?;
        validate_name("hierarchy design view", &subject.design_view)?;
        validate_string_list("declared hierarchy fallback", &subject.declared_fallbacks)?;
        if let Some(view) = &subject.resolved_simulation_view {
            validate_name("resolved simulation view", view)?;
        }
        if let Some(view) = &subject.fallback_used {
            validate_name("used hierarchy fallback", view)?;
        }
        for child in &subject.child_instance_paths {
            validate_path("hierarchy child path", child)?;
        }
        if let Some(boundary) = &subject.protected_boundary_id {
            validate_name("protected boundary identity", boundary)?;
        }
        if !paths.insert(subject.instance_path.clone()) {
            return Err(DesignManagementError::DuplicateHierarchyPath(
                subject.instance_path.clone(),
            ));
        }
    }
    let mut boundaries = HashSet::with_capacity(request.boundary_evidence.len());
    for evidence in &request.boundary_evidence {
        validate_name("protected boundary identity", &evidence.boundary_id)?;
        if !boundaries.insert(evidence.boundary_id.clone()) {
            return Err(DesignManagementError::DuplicateProtectedBoundary(
                evidence.boundary_id.clone(),
            ));
        }
    }
    Ok(())
}

fn evaluate_hierarchy_audit(
    request: &HierarchyAuditRequest,
) -> Result<Vec<HierarchyAuditFinding>, DesignManagementError> {
    let by_path = request
        .subjects
        .iter()
        .map(|subject| (subject.instance_path.as_str(), subject))
        .collect::<BTreeMap<_, _>>();
    let boundaries = request
        .boundary_evidence
        .iter()
        .map(|evidence| (evidence.boundary_id.as_str(), evidence))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for subject in &request.subjects {
        if subject.resolved_simulation_view.is_none() {
            findings.push(HierarchyAuditFinding {
                kind: HierarchyAuditFindingKind::UnresolvedView,
                instance_path: subject.instance_path.clone(),
                detail: format!("{} has no resolved simulation view", subject.cell_name),
            });
        }
        if let Some(fallback) = &subject.fallback_used
            && !subject
                .declared_fallbacks
                .iter()
                .any(|view| view == fallback)
        {
            findings.push(HierarchyAuditFinding {
                kind: HierarchyAuditFindingKind::UndeclaredFallback,
                instance_path: subject.instance_path.clone(),
                detail: format!("fallback {fallback:?} is not declared"),
            });
        }
        for child in &subject.child_instance_paths {
            if !by_path.contains_key(child.as_str()) {
                findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::MissingChild,
                    instance_path: subject.instance_path.clone(),
                    detail: format!("declared child {child:?} is absent"),
                });
            }
        }
        if let Some(boundary_id) = &subject.protected_boundary_id {
            match boundaries.get(boundary_id.as_str()) {
                None => findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::MissingProtectedBoundaryEvidence,
                    instance_path: subject.instance_path.clone(),
                    detail: format!("protected boundary {boundary_id:?} has no evidence"),
                }),
                Some(evidence) => {
                    if request.protected_boundaries
                        == ProtectedBoundaryChecks::ValidateSignaturesAndPins
                        && !evidence.signature_valid
                    {
                        findings.push(HierarchyAuditFinding {
                            kind: HierarchyAuditFindingKind::InvalidProtectedBoundarySignature,
                            instance_path: subject.instance_path.clone(),
                            detail: format!(
                                "protected boundary {boundary_id:?} signature is invalid"
                            ),
                        });
                    }
                    if !evidence.pins_match {
                        findings.push(HierarchyAuditFinding {
                            kind: HierarchyAuditFindingKind::ProtectedBoundaryPinMismatch,
                            instance_path: subject.instance_path.clone(),
                            detail: format!("protected boundary {boundary_id:?} pins do not match"),
                        });
                    }
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut active = HashSet::new();
    for subject in &request.subjects {
        detect_hierarchy_cycles(
            subject.instance_path.as_str(),
            &by_path,
            &mut visited,
            &mut active,
            &mut findings,
        );
    }
    findings.sort_by(|left, right| {
        left.instance_path
            .cmp(&right.instance_path)
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.detail.cmp(&right.detail))
    });
    require_limit(
        "hierarchy audit findings",
        findings.len(),
        MAX_HIERARCHY_AUDIT_FINDINGS,
    )?;
    Ok(findings)
}

fn detect_hierarchy_cycles(
    path: &str,
    by_path: &BTreeMap<&str, &HierarchyAuditSubject>,
    visited: &mut HashSet<String>,
    active: &mut HashSet<String>,
    findings: &mut Vec<HierarchyAuditFinding>,
) {
    if visited.contains(path) {
        return;
    }
    if !active.insert(path.to_owned()) {
        findings.push(HierarchyAuditFinding {
            kind: HierarchyAuditFindingKind::HierarchyCycle,
            instance_path: path.to_owned(),
            detail: format!("hierarchy cycle reaches {path:?}"),
        });
        return;
    }
    if let Some(subject) = by_path.get(path) {
        for child in &subject.child_instance_paths {
            if active.contains(child) {
                findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::HierarchyCycle,
                    instance_path: path.to_owned(),
                    detail: format!("hierarchy cycle reaches {child:?}"),
                });
            } else if by_path.contains_key(child.as_str()) {
                detect_hierarchy_cycles(child, by_path, visited, active, findings);
            }
        }
    }
    active.remove(path);
    visited.insert(path.to_owned());
}

fn canonical_cell_view_key(value: &str) -> Result<String, DesignManagementError> {
    let normalized = normalize_text(value);
    validate_value("cell-view key", &normalized, false)?;
    if normalized.len() > MAX_DESIGN_PATH_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field: "cell-view key",
            actual: normalized.len(),
            maximum: MAX_DESIGN_PATH_BYTES,
        });
    }
    if normalized.contains('\\') {
        return Err(DesignManagementError::InvalidCellViewKey(normalized));
    }
    let segments = normalized.split('/').collect::<Vec<_>>();
    if segments.len() != 3 {
        return Err(DesignManagementError::InvalidCellViewKey(normalized));
    }
    let library = canonical_cell_view_segment("library", segments[0])?;
    let cell = canonical_cell_view_segment("cell", segments[1])?;
    let view = canonical_cell_view_segment("view", segments[2])?;
    Ok(format!("{library}/{cell}/{view}"))
}

fn canonical_cell_view_segment(
    field: &'static str,
    value: &str,
) -> Result<String, DesignManagementError> {
    validate_name(field, value)?;
    if value
        .chars()
        .any(|character| !character.is_alphanumeric() && character != '_')
    {
        return Err(DesignManagementError::InvalidCellViewSegment {
            field,
            value: value.to_owned(),
        });
    }
    Ok(value.to_lowercase())
}

fn cell_view_key_segments(value: &str) -> Result<[&str; 3], DesignManagementError> {
    let mut segments = value.split('/');
    let library = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    let cell = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    let view = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    if segments.next().is_some() {
        return Err(DesignManagementError::InvalidCellViewKey(value.to_owned()));
    }
    Ok([library, cell, view])
}

fn validate_name(field: &'static str, value: &str) -> Result<(), DesignManagementError> {
    validate_value(field, value, false)?;
    if value.len() > MAX_DESIGN_NAME_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_NAME_BYTES,
        });
    }
    Ok(())
}

fn validate_path(field: &'static str, value: &str) -> Result<(), DesignManagementError> {
    validate_value(field, value, false)?;
    if value.len() > MAX_DESIGN_PATH_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_PATH_BYTES,
        });
    }
    if !value.starts_with('/') || value.contains("//") || value.split('/').any(|part| part == "..")
    {
        return Err(DesignManagementError::InvalidPath {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_value(
    field: &'static str,
    value: &str,
    allow_empty: bool,
) -> Result<(), DesignManagementError> {
    let normalized = value.nfc().collect::<String>();
    if value.trim() != value || normalized != value || (!allow_empty && value.is_empty()) {
        return Err(DesignManagementError::InvalidText {
            field,
            value: value.to_owned(),
        });
    }
    if value.len() > MAX_DESIGN_VALUE_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_VALUE_BYTES,
        });
    }
    if value.chars().any(char::is_control) {
        return Err(DesignManagementError::InvalidText {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_string_list(
    field: &'static str,
    values: &[String],
) -> Result<(), DesignManagementError> {
    let mut unique = HashSet::with_capacity(values.len());
    for value in values {
        validate_name(field, value)?;
        if !unique.insert(case_fold(value)) {
            return Err(DesignManagementError::DuplicateListEntry {
                field,
                value: value.clone(),
            });
        }
    }
    Ok(())
}

fn validate_prefix(prefix: &str) -> Result<(), DesignManagementError> {
    if prefix.is_empty()
        || prefix.len() > MAX_PREFIX_BYTES
        || !prefix
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        || normalize_prefix(prefix) != prefix
    {
        return Err(DesignManagementError::InvalidAnnotationPrefix(
            prefix.to_owned(),
        ));
    }
    Ok(())
}

fn validate_reference_designator(value: &str) -> Result<(), DesignManagementError> {
    validate_name("reference designator", value)?;
    let prefix_len = value
        .chars()
        .take_while(|character| character.is_ascii_alphabetic())
        .count();
    let (prefix, number) = value.split_at(prefix_len);
    validate_prefix(&normalize_prefix(prefix))?;
    if number.is_empty()
        || !number.chars().all(|character| character.is_ascii_digit())
        || number
            .parse::<u32>()
            .ok()
            .filter(|number| *number > 0)
            .is_none()
    {
        return Err(DesignManagementError::InvalidReferenceDesignator(
            value.to_owned(),
        ));
    }
    Ok(())
}

fn normalize_text(value: &str) -> String {
    value.trim().nfc().collect()
}

fn normalize_prefix(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

fn case_fold(value: &str) -> String {
    value.to_lowercase()
}

fn unique_object_ids(
    ids: impl IntoIterator<Item = u64>,
) -> Result<Vec<u64>, DesignManagementError> {
    let mut seen = HashSet::new();
    let mut values = Vec::new();
    for id in ids {
        require_object_id(id)?;
        if !seen.insert(id) {
            return Err(DesignManagementError::DuplicateSchematicObject(id));
        }
        values.push(id);
    }
    values.sort_unstable();
    Ok(values)
}

fn require_object_id(id: u64) -> Result<(), DesignManagementError> {
    if id == 0 {
        Err(DesignManagementError::ZeroSchematicObject)
    } else {
        Ok(())
    }
}

fn require_limit(
    domain: &'static str,
    actual: usize,
    maximum: usize,
) -> Result<(), DesignManagementError> {
    if actual > maximum {
        Err(DesignManagementError::LimitExceeded {
            domain,
            actual,
            maximum,
        })
    } else {
        Ok(())
    }
}

fn require_non_nil(value: Uuid, domain: &'static str) -> Result<(), DesignManagementError> {
    if value.is_nil() {
        Err(DesignManagementError::NilIdentity(domain))
    } else {
        Ok(())
    }
}

fn require_nonzero_revision(
    revision: u64,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if revision == 0 {
        Err(DesignManagementError::ZeroRevision { domain, identity })
    } else {
        Ok(())
    }
}

fn require_revision(
    expected: u64,
    actual: u64,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if expected == actual {
        Ok(())
    } else {
        Err(DesignManagementError::RevisionConflict {
            domain,
            identity,
            expected,
            actual,
        })
    }
}

fn next_revision(
    revision: u64,
    domain: &'static str,
    identity: String,
) -> Result<u64, DesignManagementError> {
    revision
        .checked_add(1)
        .ok_or(DesignManagementError::RevisionExhausted { domain, identity })
}

fn require_digest(
    actual: ContentDigest,
    expected: ContentDigest,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if actual == expected {
        Ok(())
    } else {
        Err(DesignManagementError::SemanticDigestMismatch { domain, identity })
    }
}

fn digest<T: Serialize>(
    schema: &'static str,
    value: &T,
) -> Result<ContentDigest, DesignManagementError> {
    #[derive(Serialize)]
    struct Material<'a, T> {
        schema: &'static str,
        value: &'a T,
    }
    let bytes = serde_json::to_vec(&Material { schema, value })
        .map_err(|error| DesignManagementError::Serialization(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

fn digest_infallible<T: Serialize>(schema: &'static str, value: &T) -> ContentDigest {
    digest(schema, value).expect("serializing a compile-time domain structure cannot fail")
}

const fn empty_digest() -> ContentDigest {
    ContentDigest::from_bytes([0; 32])
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DesignManagementError {
    #[error("{domain} schema {actual} is unsupported")]
    UnsupportedSchema { domain: &'static str, actual: u16 },
    #[error("{domain} contains {actual} entries; maximum is {maximum}")]
    LimitExceeded {
        domain: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("{0} identity must not be nil")]
    NilIdentity(&'static str),
    #[error("{domain} identity {identity} is duplicated")]
    DuplicateIdentity {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} has revision zero")]
    ZeroRevision {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} revision conflict: expected {expected}, current is {actual}")]
    RevisionConflict {
        domain: &'static str,
        identity: String,
        expected: u64,
        actual: u64,
    },
    #[error("{domain} {identity} revision space is exhausted")]
    RevisionExhausted {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} semantic digest does not match its content")]
    SemanticDigestMismatch {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} name {name:?} is already in use")]
    DuplicateName { domain: &'static str, name: String },
    #[error("{domain} references missing identity {identity}")]
    MissingReference {
        domain: &'static str,
        identity: String,
    },
    #[error("a non-empty {0} catalog requires an active selection")]
    ActiveSelectionRequired(&'static str),
    #[error("the active {0} cannot be removed")]
    ActiveRemoval(&'static str),
    #[error("{0} has no semantic changes")]
    NoChanges(&'static str),
    #[error("{field} value {value:?} is empty, padded, or contains control characters")]
    InvalidText { field: &'static str, value: String },
    #[error("{field} contains {actual} bytes; maximum is {maximum}")]
    TextTooLong {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("{field} path {value:?} is not canonical")]
    InvalidPath { field: &'static str, value: String },
    #[error("{field} contains duplicate entry {value:?}")]
    DuplicateListEntry { field: &'static str, value: String },
    #[error("{0} is outside the supported numeric range")]
    NumericRange(&'static str),
    #[error("explicit sheet page number {0} is duplicated")]
    DuplicateSheetPage(u32),
    #[error("sheet reorder must contain every current sheet exactly once")]
    InvalidSheetOrder,
    #[error("schematic object identity must be greater than zero")]
    ZeroSchematicObject,
    #[error("schematic object {0} is duplicated")]
    DuplicateSchematicObject(u64),
    #[error("project schematic object {0} is duplicated")]
    DuplicateScopedSchematicObject(SchematicObjectKey),
    #[error("schematic object {0} has no explicit sheet assignment")]
    UnassignedSchematicObject(u64),
    #[error("the selection spans more than one source sheet")]
    MixedSourceSheets,
    #[error("the operation requires at least one selected object")]
    EmptySelection,
    #[error("explicit boundary-port resolution must contain at least one port")]
    EmptyExplicitBoundaryPorts,
    #[error("a boundary port does not connect the source and destination sheets")]
    BoundaryPortOutsideMove,
    #[error("a cross-sheet port must connect two distinct sheets")]
    CrossSheetPortSameSheet,
    #[error("the cross-sheet port signal type and discipline are incompatible")]
    IncompatiblePortContract,
    #[error("a cross-sheet port already owns the same net and endpoints")]
    DuplicateCrossSheetPort,
    #[error(
        "cross-sheet port anchor object {object_id} belongs to {actual:?}; expected sheet {expected}"
    )]
    CrossSheetPortAnchorSheetMismatch {
        object_id: u64,
        expected: SheetId,
        actual: Option<SheetId>,
    },
    #[error("assembly variant {0} has dependent child variants and is immutable")]
    VariantHasDependents(AssemblyVariantId),
    #[error("assembly variant parent chain contains a cycle at {0}")]
    VariantParentCycle(AssemblyVariantId),
    #[error("assembly variant {child} retains a stale parent snapshot {parent}")]
    StaleVariantParent {
        child: AssemblyVariantId,
        parent: AssemblyVariantId,
    },
    #[error("a variant cannot be compared with itself")]
    SameVariantComparison,
    #[error("variant matrix cell ({variant}, object {object}) is duplicated")]
    DuplicateVariantMatrixCell {
        variant: AssemblyVariantId,
        object: SchematicObjectKey,
    },
    #[error("object {0} has no replacement and the matrix policy blocks missing replacements")]
    MissingReplacement(SchematicObjectKey),
    #[error("object {0} replacement is not qualified")]
    UnqualifiedReplacement(SchematicObjectKey),
    #[error("annotation reserved range {first}..{last} is invalid")]
    InvalidAnnotationRange { first: u32, last: u32 },
    #[error("annotation reserved range must name at least one prefix")]
    EmptyAnnotationPrefixes,
    #[error("annotation reserved ranges overlap for a shared scope and prefix")]
    OverlappingAnnotationRanges,
    #[error("annotation prefix {0:?} must contain 1..={MAX_PREFIX_BYTES} ASCII letters")]
    InvalidAnnotationPrefix(String),
    #[error("reference designator {0:?} is invalid")]
    InvalidReferenceDesignator(String),
    #[error("reference designator {0:?} is duplicated")]
    DuplicateReferenceDesignator(String),
    #[error("annotation range for prefix {0:?} is exhausted")]
    AnnotationRangeExhausted(String),
    #[error("renumber scope contains no eligible objects")]
    EmptyRenumberScope,
    #[error("protected reference on object {0} requires explicit review")]
    ProtectedReferenceReviewRequired(SchematicObjectKey),
    #[error("renumber preview is stale relative to policy or schematic objects")]
    StaleRenumberPreview,
    #[error("annotation journal sequence is {actual}; expected {expected}")]
    InvalidAnnotationSequence { expected: u64, actual: u64 },
    #[error("annotation object authority contains a redirect cycle at {0}")]
    AnnotationAuthorityCycle(SchematicObjectKey),
    #[error("annotation authority merges unrelated objects {first} and {second} into {target}")]
    AnnotationAuthorityConflation {
        target: SchematicObjectKey,
        first: SchematicObjectKey,
        second: SchematicObjectKey,
    },
    #[error("annotation request refers to renamed or deleted object authority {0}")]
    InactiveAnnotationObjectAuthority(SchematicObjectKey),
    #[error("hierarchy audit requires at least one subject")]
    EmptyHierarchyAudit,
    #[error("hierarchy audit instance path {0:?} is duplicated")]
    DuplicateHierarchyPath(String),
    #[error("protected boundary evidence {0:?} is duplicated")]
    DuplicateProtectedBoundary(String),
    #[error("hierarchy audit sequence is {actual}; expected {expected}")]
    InvalidHierarchyAuditSequence { expected: u64, actual: u64 },
    #[error("cell-view key {0:?} is invalid")]
    InvalidCellViewKey(String),
    #[error("schematic object key {0:?} is invalid")]
    InvalidSchematicObjectKey(String),
    #[error("cell-view {field} segment {value:?} is invalid")]
    InvalidCellViewSegment { field: &'static str, value: String },
    #[error("cell-view key {0:?} is not stored canonically")]
    NonCanonicalCellViewKey(String),
    #[error("sheet-catalog ownership key {0:?} already exists")]
    SheetCatalogOwnershipCollision(String),
    #[error("sheet {0} remains referenced by the annotation policy")]
    SheetCatalogReferenced(SheetId),
    #[error("assembly variant {variant} still owns deleted schematic object {object}")]
    LiveVariantObjectReference {
        variant: AssemblyVariantId,
        object: SchematicObjectKey,
    },
    #[error("cell-view key {0:?} already has a sheet catalog")]
    AlreadyBootstrapped(String),
    #[error("design-management data could not be serialized: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet(name: &str, page: u32) -> SheetDefinition {
        SheetDefinition {
            name: name.to_owned(),
            template: SheetTemplate::AnalogSchematic,
            port_policy: SheetPortPolicy::TypedOffSheetPorts,
            explicit_page_number: Some(page),
        }
    }

    fn substitution(cell: &str, qualification: VariantQualificationState) -> ComponentSubstitution {
        ComponentSubstitution {
            library: "project".to_owned(),
            cell: cell.to_owned(),
            view: "schematic".to_owned(),
            value_override: None,
            model_section: None,
            port_equivalence_digest: Some(ContentDigest::from_bytes([7; 32])),
            qualification,
        }
    }

    fn variant_draft(
        name: &str,
        parent_id: Option<AssemblyVariantId>,
        overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
    ) -> AssemblyVariantDraft {
        AssemblyVariantDraft {
            name: name.to_owned(),
            parent_id,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides,
        }
    }

    fn annotation_object(id: u64, reference: &str, x: i64) -> AnnotationObject {
        AnnotationObject {
            object: object_key(id),
            current_reference: reference.to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition { x, y: 0 },
            connectivity_order: Some(id),
            locked: false,
            external: false,
            imported: false,
        }
    }

    fn object_key(id: u64) -> SchematicObjectKey {
        SchematicObjectKey::new("work/top/schematic", id).unwrap()
    }

    #[test]
    fn empty_catalog_round_trips_and_rejects_unknown_fields() {
        let catalog = DesignManagementCatalog::default();
        assert!(catalog.is_empty());
        catalog.validate().expect("default catalog validates");
        let encoded = serde_json::to_string(&catalog).expect("serialize catalog");
        let decoded: DesignManagementCatalog =
            serde_json::from_str(&encoded).expect("deserialize catalog");
        assert_eq!(decoded, catalog);
        assert_eq!(
            decoded.semantic_digest().unwrap(),
            catalog.semantic_digest().unwrap()
        );

        let mut value = serde_json::to_value(catalog).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("invented".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<DesignManagementCatalog>(value).is_err());
    }

    #[test]
    fn sheet_order_changes_presentation_without_changing_stable_identity() {
        let mut catalog = SheetCatalog::default();
        let afe = catalog.create_sheet(sheet("AFE core", 1), None).unwrap();
        let bias = catalog
            .create_sheet(sheet("Bias and reference", 2), Some(afe))
            .unwrap();
        let afe_digest = catalog.find(afe).unwrap().semantic_digest();
        let revision = catalog.revision();

        catalog
            .reorder(
                revision,
                vec![bias, afe],
                ReorderPageNumbering::RetainExplicitPageNumbers,
                ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
            )
            .unwrap();

        assert_eq!(
            catalog
                .sheets()
                .iter()
                .map(DesignSheet::id)
                .collect::<Vec<_>>(),
            vec![bias, afe]
        );
        assert_eq!(catalog.find(afe).unwrap().semantic_digest(), afe_digest);
        assert!(matches!(
            catalog.reorder(
                revision,
                vec![afe, bias],
                ReorderPageNumbering::RetainExplicitPageNumbers,
                ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
            ),
            Err(DesignManagementError::RevisionConflict { .. })
        ));
    }

    #[test]
    fn sheet_move_and_reconciliation_are_atomic_and_remove_dead_ports() {
        let mut catalog = SheetCatalog::default();
        let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
        let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
        catalog
            .assign_objects(catalog.revision(), source, [1, 2])
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), destination, [3])
            .unwrap();
        let receipt = catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![1],
                destination_sheet_id: destination,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "VREF".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: source,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: 2,
                                point: Point::new(14, 9),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: 3,
                                terminal_name: "VREF".to_owned(),
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .unwrap();
        assert_eq!(catalog.sheet_for_object(1), Some(destination));
        assert_eq!(receipt.created_port_ids.len(), 1);
        let retained = catalog.cross_sheet_ports()[0].definition();
        assert_eq!(
            retained.first.anchor,
            CrossSheetPortAnchor::WirePoint {
                wire_id: 2,
                point: Point::new(14, 9),
            }
        );
        assert_eq!(retained.second.object_id(), 3);

        let reconciled = catalog
            .reconcile_object_assignments(catalog.revision(), [1, 4], Some(destination))
            .unwrap();
        assert_eq!(reconciled.added_assignments, 1);
        assert_eq!(reconciled.removed_assignments, 2);
        assert_eq!(reconciled.removed_cross_sheet_ports, 1);
        assert_eq!(catalog.sheet_for_object(4), Some(destination));
        assert!(catalog.cross_sheet_ports().is_empty());
    }

    #[test]
    fn typed_cross_sheet_anchor_rejects_ambiguous_terminal_without_mutation() {
        let mut catalog = SheetCatalog::default();
        let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
        let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
        catalog
            .assign_objects(catalog.revision(), source, [1, 2])
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), destination, [3])
            .unwrap();
        let before = catalog.clone();
        let result = catalog.move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: destination,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "VREF".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: source,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: 2,
                            terminal_name: String::new(),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: destination,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: 3,
                            point: Point::new(0, 0),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        });
        assert!(matches!(
            result,
            Err(DesignManagementError::InvalidText {
                field: "component terminal",
                ..
            })
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn verified_empty_boundary_move_is_distinct_from_empty_explicit_ports() {
        let mut catalog = SheetCatalog::default();
        let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
        let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
        catalog
            .assign_objects(catalog.revision(), source, [1])
            .unwrap();
        let receipt = catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![1],
                destination_sheet_id: destination,
                boundary_resolution: MoveBoundaryResolution::VerifiedNoBoundaryNets,
            })
            .unwrap();
        assert!(receipt.created_port_ids.is_empty());
        assert_eq!(catalog.sheet_for_object(1), Some(destination));

        let before = catalog.clone();
        assert!(matches!(
            catalog.move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![1],
                destination_sheet_id: source,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts { ports: Vec::new() },
            }),
            Err(DesignManagementError::EmptyExplicitBoundaryPorts)
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn variant_resolution_is_immutable_and_comparison_does_not_mutate() {
        let mut catalog = AssemblyVariantCatalog::default();
        let base = catalog
            .create(variant_draft(
                "Industrial",
                None,
                BTreeMap::from([(
                    object_key(10),
                    VariantObjectOverride::Substitute {
                        replacement: substitution(
                            "resistor_industrial",
                            VariantQualificationState::Current,
                        ),
                    },
                )]),
            ))
            .unwrap();
        let child = catalog
            .create(variant_draft(
                "Automotive",
                Some(base),
                BTreeMap::from([(
                    object_key(11),
                    VariantObjectOverride::DoNotPopulate {
                        approval_reference: "ECO-42".to_owned(),
                    },
                )]),
            ))
            .unwrap();
        let before = catalog.clone();
        let resolved = catalog.resolve(child).unwrap();
        assert_eq!(resolved.lineage.len(), 2);
        assert_eq!(resolved.overrides.len(), 2);
        let comparison = catalog.compare(base, child).unwrap();
        assert_eq!(comparison.differences.len(), 1);
        assert_eq!(catalog, before, "comparison must be read-only");

        let base_revision = catalog.find(base).unwrap().revision();
        assert!(matches!(
            catalog.update(
                base,
                base_revision,
                variant_draft("Industrial revised", None, BTreeMap::new()),
            ),
            Err(DesignManagementError::VariantHasDependents(id)) if id == base
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn substitution_matrix_enforces_qualification_before_any_commit() {
        let mut catalog = AssemblyVariantCatalog::default();
        let id = catalog
            .create(variant_draft("Industrial", None, BTreeMap::new()))
            .unwrap();
        let revision = catalog.find(id).unwrap().revision();
        let before = catalog.clone();
        let result = catalog.apply_substitution_matrix(
            vec![VariantMatrixEdit {
                variant_id: id,
                expected_revision: revision,
                object: object_key(44),
                replacement: Some(substitution(
                    "candidate",
                    VariantQualificationState::ReviewRequired,
                )),
            }],
            MissingReplacementPolicy::Block,
            ModelEquivalencePolicy::RequireQualifiedReplacement,
        );
        assert!(matches!(
            result,
            Err(DesignManagementError::UnqualifiedReplacement(object)) if object == object_key(44)
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn renumber_preview_is_deterministic_and_commit_retains_immutable_mapping() {
        let mut state = AnnotationState::default();
        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![
                annotation_object(2, "R20", 20),
                annotation_object(1, "R10", 10),
            ],
        };
        let first = state.preview_renumbering(&request).unwrap();
        let second = state.preview_renumbering(&request).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first.mappings.get(&object_key(1)).unwrap().new_reference,
            "R1"
        );
        assert_eq!(
            first.mappings.get(&object_key(2)).unwrap().new_reference,
            "R2"
        );

        let id = state.commit_renumbering(&first, &request).unwrap();
        let retained = state.journal().last().unwrap();
        assert_eq!(retained.id(), id);
        assert_eq!(retained.mappings(), &first.mappings);
        let encoded = serde_json::to_string(&state).unwrap();
        let restored: AnnotationState = serde_json::from_str(&encoded).unwrap();
        assert_eq!(restored, state);
    }

    #[test]
    fn invalid_annotation_policy_is_rejected_without_partial_mutation() {
        let mut state = AnnotationState::default();
        let before = state.clone();
        let definition = AnnotationPolicyDefinition {
            reserved_ranges: vec![
                AnnotationReservedRange {
                    scope: AnnotationRangeScope::Project,
                    prefixes: vec!["R".to_owned()],
                    first: 1,
                    last: 10,
                },
                AnnotationReservedRange {
                    scope: AnnotationRangeScope::Project,
                    prefixes: vec!["R".to_owned()],
                    first: 5,
                    last: 15,
                },
            ],
            ..AnnotationPolicyDefinition::default()
        };
        let revision = state.policy().revision();
        assert!(matches!(
            state.update_policy(revision, definition),
            Err(DesignManagementError::OverlappingAnnotationRanges)
        ));
        assert_eq!(state, before);
    }

    #[test]
    fn effective_annotation_folds_partial_journal_entries_in_sequence_order() {
        let mut state = AnnotationState::default();
        let first_sheet = SheetId::new();
        let second_sheet = SheetId::new();
        let mut first_object = annotation_object(1, "R10", 10);
        first_object.sheet_id = Some(first_sheet);
        let mut second_object = annotation_object(2, "R20", 20);
        second_object.sheet_id = Some(second_sheet);
        let first_request = RenumberRequest {
            scope: RenumberScope::CurrentSheet {
                sheet_id: first_sheet,
            },
            order: RenumberOrder::SheetThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![first_object.clone(), second_object.clone()],
        };
        let first_preview = state.preview_renumbering(&first_request).unwrap();
        state
            .commit_renumbering(&first_preview, &first_request)
            .unwrap();

        first_object.current_reference =
            first_preview.mappings[&object_key(1)].new_reference.clone();
        let second_request = RenumberRequest {
            scope: RenumberScope::CurrentSheet {
                sheet_id: second_sheet,
            },
            order: RenumberOrder::SheetThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![first_object, second_object],
        };
        let second_preview = state.preview_renumbering(&second_request).unwrap();
        state
            .commit_renumbering(&second_preview, &second_request)
            .unwrap();

        let effective = state.effective_mappings();
        assert_eq!(effective.len(), 2);
        assert_eq!(effective[&object_key(1)].new_reference, "R1");
        assert_eq!(effective[&object_key(2)].new_reference, "R2");
    }

    #[test]
    fn annotation_authority_rejects_cycles_and_unrelated_object_conflation() {
        let mut state = AnnotationState::default();
        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![
                annotation_object(1, "R10", 10),
                annotation_object(2, "R20", 20),
            ],
        };
        let preview = state.preview_renumbering(&request).unwrap();
        state.commit_renumbering(&preview, &request).unwrap();
        let target = object_key(3);
        state.object_authorities.insert(
            object_key(1),
            AnnotationObjectAuthority::Redirect {
                target: target.clone(),
            },
        );
        state.object_authorities.insert(
            object_key(2),
            AnnotationObjectAuthority::Redirect {
                target: target.clone(),
            },
        );
        assert!(matches!(
            state.validate(),
            Err(DesignManagementError::AnnotationAuthorityConflation {
                target: ref actual,
                ..
            }) if actual == &target
        ));

        let mut cycle = AnnotationState::default();
        cycle.object_authorities.insert(
            object_key(1),
            AnnotationObjectAuthority::Redirect {
                target: object_key(2),
            },
        );
        cycle.object_authorities.insert(
            object_key(2),
            AnnotationObjectAuthority::Redirect {
                target: object_key(1),
            },
        );
        assert!(matches!(
            cycle.validate(),
            Err(DesignManagementError::AnnotationAuthorityCycle(_))
        ));
    }

    #[test]
    fn hierarchy_audit_records_cycle_and_protected_boundary_failures() {
        let mut catalog = DesignManagementCatalog::default();
        let request = HierarchyAuditRequest {
            configuration: HierarchyAuditConfiguration::ActiveProject,
            view_checks: HierarchyViewChecks::AllDeclaredFallbacks,
            protected_boundaries: ProtectedBoundaryChecks::ValidateSignaturesAndPins,
            subjects: vec![
                HierarchyAuditSubject {
                    instance_path: "/top".to_owned(),
                    cell_name: "top".to_owned(),
                    design_view: "schematic".to_owned(),
                    declared_fallbacks: vec!["schematic".to_owned()],
                    resolved_simulation_view: Some("schematic".to_owned()),
                    fallback_used: Some("schematic".to_owned()),
                    child_instance_paths: vec!["/top/X1".to_owned()],
                    protected_boundary_id: None,
                },
                HierarchyAuditSubject {
                    instance_path: "/top/X1".to_owned(),
                    cell_name: "vendor".to_owned(),
                    design_view: "symbol".to_owned(),
                    declared_fallbacks: vec!["protected-spice".to_owned()],
                    resolved_simulation_view: Some("protected-spice".to_owned()),
                    fallback_used: Some("protected-spice".to_owned()),
                    child_instance_paths: vec!["/top".to_owned()],
                    protected_boundary_id: Some("vendor-boundary".to_owned()),
                },
            ],
            boundary_evidence: vec![ProtectedBoundaryEvidence {
                boundary_id: "vendor-boundary".to_owned(),
                signature_valid: false,
                pins_match: false,
            }],
        };
        let id = catalog.run_and_record_hierarchy_audit(&request).unwrap();
        let receipt = catalog.hierarchy_audits().last().unwrap();
        assert_eq!(receipt.id(), id);
        assert!(!receipt.passed());
        let kinds = receipt
            .findings()
            .iter()
            .map(|finding| finding.kind)
            .collect::<BTreeSet<_>>();
        assert!(kinds.contains(&HierarchyAuditFindingKind::HierarchyCycle));
        assert!(kinds.contains(&HierarchyAuditFindingKind::InvalidProtectedBoundarySignature));
        assert!(kinds.contains(&HierarchyAuditFindingKind::ProtectedBoundaryPinMismatch));
    }

    #[test]
    fn cell_view_sheet_ownership_and_reviewed_publish_are_deterministic() {
        let mut live = DesignManagementCatalog::default();
        let sheet_id = live
            .bootstrap_for_cell_view(" Project/Top/Schematic ", "Main", [8, 9])
            .unwrap();
        assert_eq!(
            live.sheet_for_object_or_active("project/top/schematic", 8),
            Some(sheet_id)
        );
        assert_eq!(
            live.sheet_for_object_or_active("PROJECT/TOP/SCHEMATIC", 500),
            Some(sheet_id),
            "legacy unassigned objects inherit active/first sheet without mutation"
        );
        let original_revision = live.revision();
        let mut candidate = live.clone();
        candidate
            .sheet_catalog_mut("project/top/schematic")
            .unwrap()
            .create_sheet(sheet("Power", 2), Some(sheet_id))
            .unwrap();
        let new_revision = live
            .publish_reviewed_candidate(original_revision, candidate)
            .unwrap();
        assert_eq!(new_revision, original_revision + 1);
        assert_eq!(
            live.sheet_catalog("project/top/schematic")
                .unwrap()
                .sheets()
                .len(),
            2
        );
    }

    #[test]
    fn cell_rename_and_delete_remap_sheet_catalog_ownership_atomically() {
        let mut catalog = DesignManagementCatalog::default();
        catalog
            .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
            .unwrap();
        catalog
            .bootstrap_for_cell_view("work/amp/testbench", "Bench", [2])
            .unwrap();
        let original_revision = catalog.revision();
        let renamed = catalog
            .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
            .unwrap();
        assert_eq!(renamed.affected_sheet_catalogs, 2);
        assert_eq!(renamed.catalog_revision, original_revision + 1);
        assert!(catalog.sheet_catalog("work/amp/schematic").is_none());
        assert!(catalog.sheet_catalog("work/amp_rev_b/schematic").is_some());

        let removed = catalog
            .remove_sheet_catalog_for_view("work/amp_rev_b/testbench")
            .unwrap();
        assert_eq!(removed.affected_sheet_catalogs, 1);
        let removed = catalog
            .remove_sheet_catalogs_for_cell("work", "amp_rev_b")
            .unwrap();
        assert_eq!(removed.affected_sheet_catalogs, 1);
        assert!(catalog.sheet_catalogs().is_empty());
        let revision = catalog.revision();
        let copy = catalog
            .copy_cell_sheet_catalogs("work", "missing", "work", "copy")
            .unwrap();
        assert_eq!(copy.copied_sheet_catalogs, 0);
        assert_eq!(copy.catalog_revision, revision);
        assert_eq!(catalog.revision(), revision);
    }

    #[test]
    fn deleting_sheet_ownership_blocks_while_annotation_range_references_it() {
        let mut catalog = DesignManagementCatalog::default();
        let sheet_id = catalog
            .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
            .unwrap();
        let mut policy = catalog.annotation().policy().definition().clone();
        policy.reserved_ranges.push(AnnotationReservedRange {
            scope: AnnotationRangeScope::Sheet { sheet_id },
            prefixes: vec!["R".to_owned()],
            first: 1,
            last: 399,
        });
        let revision = catalog.annotation().policy().revision();
        catalog
            .annotation_mut()
            .update_policy(revision, policy)
            .unwrap();
        let before = catalog.clone();
        assert!(matches!(
            catalog.remove_sheet_catalog_for_view("work/amp/schematic"),
            Err(DesignManagementError::SheetCatalogReferenced(id)) if id == sheet_id
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn cell_rename_remaps_live_variant_and_annotation_object_owners() {
        let mut catalog = DesignManagementCatalog::default();
        catalog
            .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
            .unwrap();
        let old_object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
        let new_object = SchematicObjectKey::new("work/amp_rev_b/schematic", 1).unwrap();
        let variant = catalog
            .variants_mut()
            .create(variant_draft(
                "Industrial",
                None,
                BTreeMap::from([(
                    old_object.clone(),
                    VariantObjectOverride::DoNotPopulate {
                        approval_reference: "ECO-9".to_owned(),
                    },
                )]),
            ))
            .unwrap();
        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: old_object.clone(),
                current_reference: "R10".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: catalog
                    .sheet_catalog("work/amp/schematic")
                    .unwrap()
                    .active_sheet_id(),
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition::default(),
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let preview = catalog.annotation().preview_renumbering(&request).unwrap();
        catalog
            .annotation_mut()
            .commit_renumbering(&preview, &request)
            .unwrap();

        let receipt = catalog
            .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
            .unwrap();
        assert_eq!(receipt.remapped_variant_objects, 1);
        assert_eq!(receipt.remapped_annotation_objects, 1);
        let resolved = catalog.variants().resolve(variant).unwrap();
        assert!(!resolved.overrides.contains_key(&old_object));
        assert!(resolved.overrides.contains_key(&new_object));
        assert!(
            catalog
                .annotation()
                .effective_mapping_for("work/amp_rev_b/schematic", 1)
                .unwrap()
                .is_some()
        );
        assert!(
            catalog
                .annotation()
                .effective_mapping_for("work/amp/schematic", 1)
                .unwrap()
                .is_none(),
            "renamed ownership must not remain effective under the old key"
        );
        assert_eq!(catalog.annotation().journal().len(), 1);
        assert!(matches!(
            catalog.annotation().object_authorities().get(&old_object),
            Some(AnnotationObjectAuthority::Redirect { target }) if target == &new_object
        ));
        let encoded = serde_json::to_string(&catalog).unwrap();
        let restored: DesignManagementCatalog = serde_json::from_str(&encoded).unwrap();
        assert_eq!(restored, catalog);
    }

    #[test]
    fn cell_delete_blocks_live_variant_then_tombstones_annotation_without_rewriting_history() {
        let mut catalog = DesignManagementCatalog::default();
        catalog
            .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
            .unwrap();
        let object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
        let variant = catalog
            .variants_mut()
            .create(variant_draft(
                "Industrial",
                None,
                BTreeMap::from([(
                    object.clone(),
                    VariantObjectOverride::DoNotPopulate {
                        approval_reference: "ECO-11".to_owned(),
                    },
                )]),
            ))
            .unwrap();
        let request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: object.clone(),
                current_reference: "R8".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: None,
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition::default(),
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let preview = catalog.annotation().preview_renumbering(&request).unwrap();
        catalog
            .annotation_mut()
            .commit_renumbering(&preview, &request)
            .unwrap();

        let before = catalog.clone();
        assert!(matches!(
            catalog.remove_sheet_catalogs_for_cell("work", "amp"),
            Err(DesignManagementError::LiveVariantObjectReference { variant: id, object: ref key })
                if id == variant && key == &object
        ));
        assert_eq!(catalog, before, "blocked deletion must be atomic");

        let revision = catalog.variants().find(variant).unwrap().revision();
        catalog
            .variants_mut()
            .update(
                variant,
                revision,
                variant_draft("Industrial", None, BTreeMap::new()),
            )
            .unwrap();
        let removed = catalog
            .remove_sheet_catalogs_for_cell("work", "amp")
            .unwrap();
        assert_eq!(removed.affected_sheet_catalogs, 1);
        assert_eq!(removed.remapped_annotation_objects, 1);
        assert_eq!(catalog.annotation().journal().len(), 1);
        assert!(catalog.annotation().effective_mappings().is_empty());
        assert!(matches!(
            catalog.annotation().object_authorities().get(&object),
            Some(AnnotationObjectAuthority::Tombstone)
        ));
        assert!(matches!(
            catalog.annotation().preview_renumbering(&request),
            Err(DesignManagementError::InactiveAnnotationObjectAuthority(ref key)) if key == &object
        ));
    }

    #[test]
    fn cell_copy_regenerates_sheet_port_identity_and_clones_sheet_annotation_policy() {
        let mut catalog = DesignManagementCatalog::default();
        let main = catalog
            .bootstrap_for_cell_view("work/amp/schematic", "Main", [1, 2, 3])
            .unwrap();
        let sheets = catalog.sheet_catalog_mut("work/amp/schematic").unwrap();
        let auxiliary = sheets
            .create_sheet(sheet("Auxiliary", 2), Some(main))
            .unwrap();
        sheets
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: sheets.revision(),
                object_ids: vec![3],
                destination_sheet_id: auxiliary,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: main,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: 1,
                                terminal_name: "BIAS_OUT".to_owned(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: auxiliary,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: 3,
                                point: Point::new(21, -4),
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .unwrap();
        let source_port = sheets.cross_sheet_ports()[0].id();
        let mut policy = catalog.annotation().policy().definition().clone();
        policy.reserved_ranges.push(AnnotationReservedRange {
            scope: AnnotationRangeScope::Sheet { sheet_id: main },
            prefixes: vec!["R".to_owned()],
            first: 1,
            last: 399,
        });
        let policy_revision = catalog.annotation().policy().revision();
        catalog
            .annotation_mut()
            .update_policy(policy_revision, policy)
            .unwrap();
        let annotation_request = RenumberRequest {
            scope: RenumberScope::WholeProject,
            order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            protected_reviewed: false,
            objects: vec![AnnotationObject {
                object: SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
                current_reference: "R12".to_owned(),
                device_family: "R".to_owned(),
                sheet_id: Some(main),
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition::default(),
                connectivity_order: Some(1),
                locked: false,
                external: false,
                imported: false,
            }],
        };
        let annotation_preview = catalog
            .annotation()
            .preview_renumbering(&annotation_request)
            .unwrap();
        catalog
            .annotation_mut()
            .commit_renumbering(&annotation_preview, &annotation_request)
            .unwrap();
        let variant = catalog
            .variants_mut()
            .create(variant_draft(
                "Industrial",
                None,
                BTreeMap::from([(
                    SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
                    VariantObjectOverride::DoNotPopulate {
                        approval_reference: "ECO-1".to_owned(),
                    },
                )]),
            ))
            .unwrap();

        let receipt = catalog
            .copy_cell_sheet_catalogs("work", "amp", "work", "amp_copy")
            .unwrap();
        assert_eq!(receipt.copied_sheet_catalogs, 1);
        assert_ne!(receipt.sheet_identity_map[&main], main);
        assert_ne!(receipt.port_identity_map[&source_port], source_port);
        let copied = catalog.sheet_catalog("work/amp_copy/schematic").unwrap();
        assert_eq!(
            copied.sheet_for_object(1),
            Some(receipt.sheet_identity_map[&main])
        );
        assert_eq!(
            copied.cross_sheet_ports()[0].definition().first.sheet_id,
            receipt.sheet_identity_map[&main]
        );
        assert_eq!(
            copied.cross_sheet_ports()[0].definition().first.anchor,
            CrossSheetPortAnchor::ComponentTerminal {
                component_id: 1,
                terminal_name: "BIAS_OUT".to_owned(),
            }
        );
        assert!(
            catalog
                .annotation()
                .policy()
                .definition()
                .reserved_ranges
                .iter()
                .any(|range| range.scope
                    == AnnotationRangeScope::Sheet {
                        sheet_id: receipt.sheet_identity_map[&main],
                    })
        );
        let resolved = catalog.variants().resolve(variant).unwrap();
        assert!(
            resolved
                .overrides
                .contains_key(&SchematicObjectKey::new("work/amp/schematic", 1).unwrap())
        );
        assert!(
            !resolved
                .overrides
                .contains_key(&SchematicObjectKey::new("work/amp_copy/schematic", 1).unwrap())
        );
        assert!(
            catalog
                .annotation()
                .effective_mapping_for("work/amp/schematic", 1)
                .unwrap()
                .is_some()
        );
        assert!(
            catalog
                .annotation()
                .effective_mapping_for("work/amp_copy/schematic", 1)
                .unwrap()
                .is_none()
        );
    }
}
