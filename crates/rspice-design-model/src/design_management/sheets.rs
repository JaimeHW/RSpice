//! Governed sheet catalogs, assignments, and cross-sheet port contracts.

use super::*;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DesignSheet {
    pub(super) id: SheetId,
    pub(super) revision: u64,
    pub(super) semantic_digest: ContentDigest,
    pub(super) definition: SheetDefinition,
    #[serde(default)]
    pub(super) page_format: SchematicSheetFormat,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DesignSheetWire {
    id: SheetId,
    revision: u64,
    semantic_digest: ContentDigest,
    definition: SheetDefinition,
    #[serde(default)]
    page_format: SchematicSheetFormat,
}

impl<'de> Deserialize<'de> for DesignSheet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DesignSheetWire::deserialize(deserializer)?;
        let mut page_format = wire.page_format;
        let title = page_format
            .title_block
            .fields
            .entry(DrawingSheetTitleFieldId::SheetTitle)
            .or_default();
        // Legacy sheets predate the required authored Sheet title. Their
        // stable sheet name is the only non-invented, deterministic migration
        // value and keeps old projects loadable without weakening validation.
        if title.value.trim().is_empty() {
            title.value.clone_from(&wire.definition.name);
        }
        title.visible = true;
        let value = Self {
            id: wire.id,
            revision: wire.revision,
            semantic_digest: wire.semantic_digest,
            definition: wire.definition,
            page_format,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
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
    pub const fn page_format(&self) -> &SchematicSheetFormat {
        &self.page_format
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.definition.name
    }

    pub(super) fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "sheet")?;
        require_nonzero_revision(self.revision, "sheet", self.id.to_string())?;
        validate_sheet_definition(&self.definition)?;
        self.page_format.validate()?;
        self.page_format.validate_authored_sheet_title()?;
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

    pub(super) fn validate(&self) -> Result<(), DesignManagementError> {
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
    pub(super) id: CrossSheetPortId,
    pub(super) revision: u64,
    pub(super) semantic_digest: ContentDigest,
    pub(super) definition: CrossSheetPortDefinition,
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

/// What a sheet deletion does with the schematic objects the sheet still
/// owns. The catalog cannot delete schematic geometry, so the destructive
/// resolution states the objects back to the caller instead of guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetDeleteResolution {
    BlockIfNotEmpty,
    MoveObjectsTo(SheetId),
    DeleteObjects,
}

/// Immutable outcome of one sheet deletion.
///
/// `deleted_object_ids` is a work order for the owning schematic: the catalog
/// has already dropped its own assignments, and exactly those objects must be
/// removed from the drawing to keep the two authorities in agreement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetDeleteReceipt {
    pub catalog_revision: u64,
    pub removed_sheet_id: SheetId,
    pub moved_object_ids: Vec<u64>,
    pub deleted_object_ids: Vec<u64>,
    pub removed_cross_sheet_ports: Vec<CrossSheetPortId>,
    pub next_active_sheet_id: Option<SheetId>,
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
    pub(super) schema_version: u16,
    pub(super) revision: u64,
    pub(super) settings: SheetCatalogSettings,
    pub(super) sheets: Vec<DesignSheet>,
    pub(super) active_sheet_id: Option<SheetId>,
    pub(super) object_assignments: BTreeMap<u64, SheetId>,
    pub(super) cross_sheet_ports: Vec<CrossSheetPortContract>,
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

    /// Resolve the page number printed for a sheet under the catalog's
    /// numbering authority. Stable-project numbering honors the sheet's
    /// explicit number; per-print-set numbering uses catalog order until a
    /// publication supplies a narrower ordered print set.
    #[must_use]
    pub fn page_number_and_count(&self, id: SheetId) -> Option<(u32, u32)> {
        let index = self.sheets.iter().position(|sheet| sheet.id == id)?;
        let ordinal = u32::try_from(index + 1).ok()?;
        let count = u32::try_from(self.sheets.len()).ok()?;
        let number = match self.settings.page_numbering {
            SheetPageNumbering::StableProjectOrder => self.sheets[index]
                .definition
                .explicit_page_number
                .unwrap_or(ordinal),
            SheetPageNumbering::PerPrintSet => ordinal,
        };
        Some((number, count))
    }

    #[must_use]
    pub fn sheet_for_object(&self, object_id: u64) -> Option<SheetId> {
        self.object_assignments.get(&object_id).copied()
    }

    /// The sheet after `id` in catalog order, or `None` at the last sheet.
    /// Navigation stops at the ends; wrapping is a presentation choice and
    /// belongs to the surface that offers it, not to the durable order.
    #[must_use]
    pub fn next_sheet(&self, id: SheetId) -> Option<SheetId> {
        let index = self.sheets.iter().position(|sheet| sheet.id == id)?;
        self.sheets.get(index + 1).map(|sheet| sheet.id)
    }

    /// The sheet before `id` in catalog order, or `None` at the first sheet.
    #[must_use]
    pub fn previous_sheet(&self, id: SheetId) -> Option<SheetId> {
        let index = self.sheets.iter().position(|sheet| sheet.id == id)?;
        self.sheets.get(index.checked_sub(1)?).map(|sheet| sheet.id)
    }

    /// Every schematic object assigned to a sheet, in ascending identity
    /// order.
    pub fn objects_on_sheet(&self, id: SheetId) -> impl Iterator<Item = u64> + '_ {
        self.object_assignments
            .iter()
            .filter(move |(_, sheet_id)| **sheet_id == id)
            .map(|(object_id, _)| *object_id)
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
        self.create_sheet_with_page_format(
            definition,
            insert_after,
            SchematicSheetFormat::default(),
        )
    }

    /// Create a sheet with an already-resolved authored drawing-sheet
    /// format. Callers that own project default policy resolve it before this
    /// catalog-local transaction; the sheet receives an exact snapshot and a
    /// sheet-owned title.
    pub fn create_sheet_with_page_format(
        &mut self,
        definition: SheetDefinition,
        insert_after: Option<SheetId>,
        page_format: SchematicSheetFormat,
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
        let page_format = page_format.try_update(|draft| {
            draft
                .title_block
                .fields
                .entry(DrawingSheetTitleFieldId::SheetTitle)
                .or_default()
                .value
                .clone_from(&definition.name);
        })?;
        let id = SheetId::new();
        let sheet = DesignSheet {
            id,
            revision: 1,
            semantic_digest: digest("rspice-design-sheet-semantic/v1", &definition)?,
            definition,
            page_format,
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

    /// Rename one sheet.
    ///
    /// The sheet name and the sheet-owned title-block value are one authored
    /// fact printed in two places, so a rename moves both in the same
    /// candidate. Splitting them would let the drawing print a name the
    /// navigator no longer shows.
    pub fn rename_sheet(
        &mut self,
        id: SheetId,
        expected_sheet_revision: u64,
        name: String,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        let name = normalize_text(&name);
        validate_name("sheet name", &name)?;
        if self
            .sheets
            .iter()
            .any(|sheet| sheet.id != id && case_fold(sheet.name()) == case_fold(&name))
        {
            return Err(DesignManagementError::DuplicateName {
                domain: "sheet",
                name,
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
        let page_format = sheet.page_format.try_update(|draft| {
            let title = draft
                .title_block
                .fields
                .entry(DrawingSheetTitleFieldId::SheetTitle)
                .or_default();
            title.visible = true;
            title.value.clone_from(&name);
        })?;
        if sheet.definition.name == name && sheet.page_format == page_format {
            return Err(DesignManagementError::NoChanges("sheet name"));
        }
        let mut definition = sheet.definition.clone();
        definition.name = name;
        sheet.semantic_digest = digest("rspice-design-sheet-semantic/v1", &definition)?;
        sheet.definition = definition;
        sheet.page_format = page_format;
        sheet.revision = next_revision(sheet.revision, "sheet", id.to_string())?;
        let committed_revision = sheet.revision;
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        *self = candidate;
        Ok(committed_revision)
    }

    /// Remove one sheet and resolve everything it still owns in a single
    /// transaction.
    ///
    /// A catalog is never left without a sheet, so removing the final one is
    /// refused instead of silently emptying the design. Deleting the active
    /// sheet moves activation to the following sheet, or to the preceding one
    /// when the removed sheet was last. The survivors are renumbered into a
    /// contiguous print order under the same authority `reorder` applies.
    pub fn delete_sheet(
        &mut self,
        expected_catalog_revision: u64,
        id: SheetId,
        resolution: SheetDeleteResolution,
    ) -> Result<SheetDeleteReceipt, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        let index = self
            .sheets
            .iter()
            .position(|sheet| sheet.id == id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "sheet",
                identity: id.to_string(),
            })?;
        if self.sheets.len() == 1 {
            return Err(DesignManagementError::LastSheetRemoval);
        }
        let owned = self.objects_on_sheet(id).collect::<Vec<_>>();
        let next_active_sheet_id = match self.active_sheet_id {
            Some(active) if active == id => self.next_sheet(id).or_else(|| self.previous_sheet(id)),
            retained => retained,
        };

        let mut candidate = self.clone();
        let mut moved_object_ids = Vec::new();
        let mut deleted_object_ids = Vec::new();
        let mut removed_cross_sheet_ports = Vec::new();
        match resolution {
            SheetDeleteResolution::BlockIfNotEmpty if !owned.is_empty() => {
                return Err(DesignManagementError::SheetNotEmpty {
                    sheet: id,
                    objects: owned.len(),
                });
            }
            SheetDeleteResolution::BlockIfNotEmpty => {}
            SheetDeleteResolution::MoveObjectsTo(destination) => {
                // A sheet on its way out cannot receive its own objects.
                if destination == id || self.find(destination).is_none() {
                    return Err(DesignManagementError::MissingReference {
                        domain: "destination sheet",
                        identity: destination.to_string(),
                    });
                }
                for object_id in &owned {
                    candidate.object_assignments.insert(*object_id, destination);
                }
                removed_cross_sheet_ports =
                    candidate.reanchor_cross_sheet_ports(id, destination)?;
                moved_object_ids = owned;
            }
            SheetDeleteResolution::DeleteObjects => {
                candidate
                    .object_assignments
                    .retain(|_, sheet_id| *sheet_id != id);
                let dropped = owned.iter().copied().collect::<BTreeSet<_>>();
                candidate.cross_sheet_ports.retain(|port| {
                    let anchored = dropped.contains(&port.definition.first.object_id())
                        || dropped.contains(&port.definition.second.object_id());
                    if anchored {
                        removed_cross_sheet_ports.push(port.id);
                    }
                    !anchored
                });
                deleted_object_ids = owned;
            }
        }
        candidate.sheets.remove(index);
        candidate.active_sheet_id = next_active_sheet_id;
        candidate.renumber_print_pages()?;
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        let mut receipt = SheetDeleteReceipt {
            catalog_revision: candidate.revision,
            removed_sheet_id: id,
            moved_object_ids,
            deleted_object_ids,
            removed_cross_sheet_ports,
            next_active_sheet_id: candidate.active_sheet_id,
            semantic_digest: empty_digest(),
        };
        receipt.semantic_digest = digest(
            "rspice-sheet-delete-receipt-semantic/v1",
            &SheetDeleteReceiptMaterial::from(&receipt),
        )?;
        *self = candidate;
        Ok(receipt)
    }

    /// Repoint every cross-sheet port endpoint that lived on `removed` at
    /// `destination`, and report the ports that stopped crossing a boundary.
    ///
    /// When both endpoints land on one sheet the connection is ordinary
    /// on-sheet connectivity: the contract is dropped while the physical
    /// anchors it named survive untouched.
    fn reanchor_cross_sheet_ports(
        &mut self,
        removed: SheetId,
        destination: SheetId,
    ) -> Result<Vec<CrossSheetPortId>, DesignManagementError> {
        let mut dropped = Vec::new();
        for port in &mut self.cross_sheet_ports {
            let mut definition = port.definition.clone();
            for endpoint in [&mut definition.first, &mut definition.second] {
                if endpoint.sheet_id == removed {
                    endpoint.sheet_id = destination;
                }
            }
            if definition == port.definition {
                continue;
            }
            if definition.first.sheet_id == definition.second.sheet_id {
                dropped.push(port.id);
                continue;
            }
            port.revision = next_revision(port.revision, "cross-sheet port", port.id.to_string())?;
            port.semantic_digest = digest("rspice-cross-sheet-port-semantic/v1", &definition)?;
            port.definition = definition;
        }
        let doomed = dropped.iter().copied().collect::<HashSet<_>>();
        self.cross_sheet_ports
            .retain(|port| !doomed.contains(&port.id));
        Ok(dropped)
    }

    /// Restate every printed page number as the sheet's position in catalog
    /// order, advancing the revision of each sheet whose number changed. A
    /// structural change must not leave a hole in the print set.
    fn renumber_print_pages(&mut self) -> Result<(), DesignManagementError> {
        for (index, sheet) in self.sheets.iter_mut().enumerate() {
            let page = u32::try_from(index + 1)
                .map_err(|_| DesignManagementError::NumericRange("sheet page number"))?;
            if sheet.definition.explicit_page_number != Some(page) {
                sheet.definition.explicit_page_number = Some(page);
                sheet.revision = next_revision(sheet.revision, "sheet", sheet.id.to_string())?;
                sheet.semantic_digest =
                    digest("rspice-design-sheet-semantic/v1", &sheet.definition)?;
            }
        }
        Ok(())
    }

    /// Atomically replace one sheet's authored physical page format.
    ///
    /// Electrical semantics are unchanged, but both the sheet and catalog
    /// revisions advance so asynchronous Page Setup commits fail closed.
    pub fn update_sheet_page_format(
        &mut self,
        id: SheetId,
        expected_sheet_revision: u64,
        page_format: SchematicSheetFormat,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        page_format.validate()?;
        page_format.validate_authored_sheet_title()?;
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
        if sheet.page_format == page_format {
            return Err(DesignManagementError::NoChanges("sheet page format"));
        }
        sheet.page_format = page_format;
        sheet.revision = next_revision(sheet.revision, "sheet", id.to_string())?;
        let committed_revision = sheet.revision;
        candidate.bump_revision("sheet catalog")?;
        candidate.validate()?;
        *self = candidate;
        Ok(committed_revision)
    }

    /// Refresh fallback snapshots for sheets that follow the project
    /// drawing-sheet default.
    ///
    /// Keeping the snapshot current makes every consumer—including hardcopy,
    /// retained-source descriptors, and background jobs that do not own the
    /// full project aggregate—resolve the same physical page as the canvas.
    /// Sheet-owned title values and visibility remain attached to each target.
    ///
    /// The snapshot is a transport fallback, not an authored sheet override.
    /// Updating it therefore does not advance sheet or sheet-catalog
    /// revisions. The project settings transaction already advances the
    /// design-management revision and invalidates every authority that
    /// depends on the resolved default.
    pub(super) fn refresh_inherited_page_formats(
        &mut self,
        project_default: &SchematicSheetFormat,
    ) -> Result<usize, DesignManagementError> {
        self.validate()?;
        project_default.validate()?;
        let mut candidate = self.clone();
        let mut changed = 0;
        for sheet in &mut candidate.sheets {
            if sheet.page_format.inheritance != DrawingSheetInheritance::ProjectDefault {
                continue;
            }
            let replacement = project_default.with_target_sheet_title_fields(&sheet.page_format);
            if replacement == sheet.page_format {
                continue;
            }
            sheet.page_format = replacement;
            changed += 1;
        }
        if changed == 0 {
            return Ok(0);
        }
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
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
            candidate.renumber_print_pages()?;
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

    /// Move one sheet to an absolute position in catalog order.
    ///
    /// The durable authority is the complete order, so this states the whole
    /// sequence to [`SheetCatalog::reorder`] rather than mutating a slice in
    /// place: a drag in the navigator and a scripted reorder commit through
    /// exactly one transaction.
    pub fn move_sheet(
        &mut self,
        expected_catalog_revision: u64,
        id: SheetId,
        to_index: usize,
        page_numbering: ReorderPageNumbering,
    ) -> Result<u64, DesignManagementError> {
        self.validate()?;
        require_revision(
            expected_catalog_revision,
            self.revision,
            "sheet catalog",
            "catalog".to_owned(),
        )?;
        let from_index = self
            .sheets
            .iter()
            .position(|sheet| sheet.id == id)
            .ok_or_else(|| DesignManagementError::MissingReference {
                domain: "sheet",
                identity: id.to_string(),
            })?;
        if to_index >= self.sheets.len() {
            return Err(DesignManagementError::InvalidSheetOrder);
        }
        let mut ordered_ids = self.sheets.iter().map(|sheet| sheet.id).collect::<Vec<_>>();
        let moved = ordered_ids.remove(from_index);
        ordered_ids.insert(to_index, moved);
        self.reorder(
            expected_catalog_revision,
            ordered_ids,
            page_numbering,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        )
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
    /// stable object identities. This is the cleanup path used after undo,
    /// delete, or legacy migration: missing live objects are assigned
    /// deterministically, deleted objects are removed, and a port whose
    /// physical anchor disappeared is removed in the same atomic commit.
    ///
    /// A caller that still knows where an object used to live states that
    /// through [`SheetCatalog::reconcile_object_assignments_with`] instead of
    /// letting every restored object land on one default sheet.
    pub fn reconcile_object_assignments(
        &mut self,
        expected_catalog_revision: u64,
        live_object_ids: impl IntoIterator<Item = u64>,
        default_sheet_id: Option<SheetId>,
    ) -> Result<SheetReconcileReceipt, DesignManagementError> {
        self.reconcile_object_assignments_with(
            expected_catalog_revision,
            live_object_ids,
            &BTreeMap::new(),
            default_sheet_id,
        )
    }

    /// Reconcile membership while honoring a sheet recorded per object.
    ///
    /// `recorded` is the schematic's own memory of where an object lived, as
    /// an undo, a paste, or a legacy migration restores it. It outranks the
    /// default sheet so a restored object returns to its own sheet instead of
    /// piling onto whichever sheet happens to be active. A recorded sheet that
    /// this catalog no longer contains is a stale hint, not a fault: those
    /// objects fall back to the default.
    pub fn reconcile_object_assignments_with(
        &mut self,
        expected_catalog_revision: u64,
        live_object_ids: impl IntoIterator<Item = u64>,
        recorded: &BTreeMap<u64, SheetId>,
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
                if candidate.object_assignments.contains_key(object_id) {
                    continue;
                }
                let sheet_id = recorded
                    .get(object_id)
                    .copied()
                    .filter(|recorded| self.find(*recorded).is_some())
                    .unwrap_or(destination);
                candidate.object_assignments.insert(*object_id, sheet_id);
                added_assignments += 1;
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
struct SheetDeleteReceiptMaterial<'a> {
    catalog_revision: u64,
    removed_sheet_id: SheetId,
    moved_object_ids: &'a [u64],
    deleted_object_ids: &'a [u64],
    removed_cross_sheet_ports: &'a [CrossSheetPortId],
    next_active_sheet_id: Option<SheetId>,
}

impl<'a> From<&'a SheetDeleteReceipt> for SheetDeleteReceiptMaterial<'a> {
    fn from(receipt: &'a SheetDeleteReceipt) -> Self {
        Self {
            catalog_revision: receipt.catalog_revision,
            removed_sheet_id: receipt.removed_sheet_id,
            moved_object_ids: &receipt.moved_object_ids,
            deleted_object_ids: &receipt.deleted_object_ids,
            removed_cross_sheet_ports: &receipt.removed_cross_sheet_ports,
            next_active_sheet_id: receipt.next_active_sheet_id,
        }
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
