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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignSheet {
    pub(super) id: SheetId,
    pub(super) revision: u64,
    pub(super) semantic_digest: ContentDigest,
    pub(super) definition: SheetDefinition,
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

    pub(super) fn validate(&self) -> Result<(), DesignManagementError> {
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
