//! Persisted drawing-sheet preset identities, authorities, and import receipts.
//!
//! This module separates governed preset lifecycle rules from physical sheet
//! geometry so callers cannot confuse ownership with presentation.

use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetPresetScope {
    #[default]
    Project,
    User,
    Organization,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetNewSheetPolicy {
    #[default]
    ProjectDefault,
    Ask,
    MatchCurrent,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetImportReference {
    pub preset_id: String,
    pub scope: DrawingSheetPresetScope,
}

impl DrawingSheetPresetImportReference {
    fn validate(&self) -> Result<(), DesignManagementError> {
        validate_name("drawing sheet preset import reference", &self.preset_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetPresetImportResolution {
    NewIdentity,
    MatchesByDigest,
    KeepBothRename,
    MapExisting,
    ReplaceManagedDependencies,
    RetainUnavailableDependency,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetPresetImportMappingKind {
    CreatedProjectPreset,
    ExistingPreset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetImportMapping {
    pub source: DrawingSheetPresetImportReference,
    pub target: DrawingSheetPresetImportReference,
    pub kind: DrawingSheetPresetImportMappingKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetImportConflict {
    pub source: DrawingSheetPresetImportReference,
    pub existing: Option<DrawingSheetPresetImportReference>,
    pub missing_managed_dependency: bool,
    pub resolution: DrawingSheetPresetImportResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetPresetImportSkipReason {
    NotSelected,
    ExplicitlySkipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetImportSkip {
    pub source: DrawingSheetPresetImportReference,
    pub reason: DrawingSheetPresetImportSkipReason,
}

/// Immutable audit evidence for one reviewed custom-sheet preset import.
///
/// Source candidates are partitioned into selected candidates and
/// `NotSelected` skips. Every selected candidate is then partitioned into one
/// mapping or one `ExplicitlySkipped` outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetImportReceipt {
    pub source_digest_sha256: String,
    pub source_schema: String,
    pub source_schema_version: u16,
    pub reviewed_candidate_count: usize,
    pub selected_candidates: Vec<DrawingSheetPresetImportReference>,
    pub mappings: Vec<DrawingSheetPresetImportMapping>,
    pub conflicts: Vec<DrawingSheetPresetImportConflict>,
    pub skipped_candidates: Vec<DrawingSheetPresetImportSkip>,
}

impl DrawingSheetPresetImportReceipt {
    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.source_digest_sha256.len() != 64
            || !self
                .source_digest_sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(DesignManagementError::InvalidText {
                field: "drawing sheet preset import source digest",
                value: self.source_digest_sha256.clone(),
            });
        }
        validate_name(
            "drawing sheet preset import source schema",
            &self.source_schema,
        )?;
        if self.source_schema_version == 0 {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet preset import source schema version",
            ));
        }
        require_limit(
            "drawing sheet preset import reviewed candidates",
            self.reviewed_candidate_count,
            MAX_DRAWING_SHEET_PRESET_IMPORT_CANDIDATES,
        )?;
        for (domain, actual) in [
            (
                "drawing sheet preset import selected candidates",
                self.selected_candidates.len(),
            ),
            ("drawing sheet preset import mappings", self.mappings.len()),
            (
                "drawing sheet preset import conflicts",
                self.conflicts.len(),
            ),
            (
                "drawing sheet preset import skipped candidates",
                self.skipped_candidates.len(),
            ),
        ] {
            require_limit(domain, actual, MAX_DRAWING_SHEET_PRESET_IMPORT_CANDIDATES)?;
        }

        let mut selected = BTreeSet::new();
        for source in &self.selected_candidates {
            source.validate()?;
            if !selected.insert(source.clone()) {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset import selected candidate",
                    value: source.preset_id.clone(),
                });
            }
        }

        let mut skipped = BTreeMap::new();
        let mut not_selected = 0_usize;
        for skip in &self.skipped_candidates {
            skip.source.validate()?;
            if skipped.insert(skip.source.clone(), skip.reason).is_some() {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset import skipped candidate",
                    value: skip.source.preset_id.clone(),
                });
            }
            match skip.reason {
                DrawingSheetPresetImportSkipReason::NotSelected => {
                    if selected.contains(&skip.source) {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import not-selected partition",
                        ));
                    }
                    not_selected += 1;
                }
                DrawingSheetPresetImportSkipReason::ExplicitlySkipped => {
                    if !selected.contains(&skip.source) {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import explicit-skip partition",
                        ));
                    }
                }
            }
        }
        if self.reviewed_candidate_count != selected.len() + not_selected {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet preset import reviewed-candidate partition",
            ));
        }

        let mut mapped = BTreeSet::new();
        for mapping in &self.mappings {
            mapping.source.validate()?;
            mapping.target.validate()?;
            if !selected.contains(&mapping.source) || skipped.contains_key(&mapping.source) {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import mapping partition",
                ));
            }
            if mapping.kind == DrawingSheetPresetImportMappingKind::CreatedProjectPreset
                && mapping.target.scope != DrawingSheetPresetScope::Project
            {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import created target ownership",
                ));
            }
            if !mapped.insert(mapping.source.clone()) {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset import mapped candidate",
                    value: mapping.source.preset_id.clone(),
                });
            }
        }
        for source in &selected {
            if !mapped.contains(source)
                && skipped.get(source)
                    != Some(&DrawingSheetPresetImportSkipReason::ExplicitlySkipped)
            {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import selected-candidate outcome",
                ));
            }
        }

        let reviewed = selected
            .iter()
            .cloned()
            .chain(
                self.skipped_candidates
                    .iter()
                    .filter(|skip| skip.reason == DrawingSheetPresetImportSkipReason::NotSelected)
                    .map(|skip| skip.source.clone()),
            )
            .collect::<BTreeSet<_>>();
        let mut conflicts = BTreeSet::new();
        for conflict in &self.conflicts {
            conflict.source.validate()?;
            if let Some(existing) = &conflict.existing {
                existing.validate()?;
            }
            if !reviewed.contains(&conflict.source)
                || (conflict.existing.is_none() && !conflict.missing_managed_dependency)
            {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import conflict evidence",
                ));
            }
            if !conflicts.insert(conflict.source.clone()) {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset import conflict",
                    value: conflict.source.preset_id.clone(),
                });
            }
            if !selected.contains(&conflict.source) {
                continue;
            }
            let mapping = self
                .mappings
                .iter()
                .find(|mapping| mapping.source == conflict.source);
            let explicitly_skipped = skipped.get(&conflict.source)
                == Some(&DrawingSheetPresetImportSkipReason::ExplicitlySkipped);
            match conflict.resolution {
                DrawingSheetPresetImportResolution::Skip => {
                    if !explicitly_skipped || mapping.is_some() {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import skip conflict outcome",
                        ));
                    }
                }
                DrawingSheetPresetImportResolution::MatchesByDigest
                | DrawingSheetPresetImportResolution::MapExisting => {
                    let Some(mapping) = mapping else {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import existing conflict mapping",
                        ));
                    };
                    if explicitly_skipped
                        || mapping.kind != DrawingSheetPresetImportMappingKind::ExistingPreset
                        || conflict.existing.as_ref() != Some(&mapping.target)
                    {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import existing conflict outcome",
                        ));
                    }
                }
                DrawingSheetPresetImportResolution::NewIdentity
                | DrawingSheetPresetImportResolution::KeepBothRename
                | DrawingSheetPresetImportResolution::ReplaceManagedDependencies
                | DrawingSheetPresetImportResolution::RetainUnavailableDependency => {
                    let Some(mapping) = mapping else {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import created conflict mapping",
                        ));
                    };
                    if explicitly_skipped
                        || mapping.kind != DrawingSheetPresetImportMappingKind::CreatedProjectPreset
                    {
                        return Err(DesignManagementError::NumericRange(
                            "drawing sheet preset import created conflict outcome",
                        ));
                    }
                }
            }
            if matches!(
                conflict.resolution,
                DrawingSheetPresetImportResolution::ReplaceManagedDependencies
                    | DrawingSheetPresetImportResolution::RetainUnavailableDependency
            ) && !conflict.missing_managed_dependency
            {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import managed-dependency resolution",
                ));
            }
            if conflict.missing_managed_dependency
                && !matches!(
                    conflict.resolution,
                    DrawingSheetPresetImportResolution::MatchesByDigest
                        | DrawingSheetPresetImportResolution::MapExisting
                        | DrawingSheetPresetImportResolution::ReplaceManagedDependencies
                        | DrawingSheetPresetImportResolution::RetainUnavailableDependency
                        | DrawingSheetPresetImportResolution::Skip
                )
            {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet preset import missing-managed outcome",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPreset {
    pub id: String,
    pub name: String,
    pub scope: DrawingSheetPresetScope,
    pub format: SchematicSheetFormat,
}

/// Project-domain evidence for a committed Page Setup or Sheet Format Manager
/// operation. It is deliberately independent from transient console messages
/// and undo history so a saved project can explain every multi-sheet format
/// mutation after reopening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTransactionKind {
    PageSetup,
    SheetFormatManager,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetTransactionSkip {
    pub sheet_id: SheetId,
    pub sheet_name: String,
    pub reason: String,
}

impl DrawingSheetTransactionSkip {
    fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.sheet_id.as_uuid(), "drawing sheet transaction skip")?;
        validate_name("drawing sheet transaction skipped sheet", &self.sheet_name)?;
        validate_value("drawing sheet transaction skip reason", &self.reason, false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetTransactionReceipt {
    pub catalog_revision: u64,
    pub kind: DrawingSheetTransactionKind,
    pub owner_cell_view_key: String,
    pub source_format_digest: ContentDigest,
    pub selected_sheet_ids: Vec<SheetId>,
    pub applied_sheet_ids: Vec<SheetId>,
    pub unchanged_sheet_ids: Vec<SheetId>,
    pub skipped: Vec<DrawingSheetTransactionSkip>,
    #[serde(default)]
    pub project_default_changed: bool,
    #[serde(default)]
    pub project_preset_saved: bool,
    #[serde(default)]
    pub project_settings_changed: bool,
}

impl DrawingSheetTransactionReceipt {
    pub fn validate(&self) -> Result<(), DesignManagementError> {
        require_nonzero_revision(
            self.catalog_revision,
            "drawing sheet transaction receipt",
            self.owner_cell_view_key.clone(),
        )?;
        canonical_cell_view_key(&self.owner_cell_view_key)?;
        require_limit(
            "drawing sheet transaction selected sheets",
            self.selected_sheet_ids.len(),
            MAX_DESIGN_SHEETS,
        )?;
        require_limit(
            "drawing sheet transaction applied sheets",
            self.applied_sheet_ids.len(),
            MAX_DESIGN_SHEETS,
        )?;
        require_limit(
            "drawing sheet transaction unchanged sheets",
            self.unchanged_sheet_ids.len(),
            MAX_DESIGN_SHEETS,
        )?;
        require_limit(
            "drawing sheet transaction skipped sheets",
            self.skipped.len(),
            MAX_DESIGN_SHEETS,
        )?;
        if self.selected_sheet_ids.is_empty() {
            return Err(DesignManagementError::ActiveSelectionRequired(
                "drawing sheet transaction sheet",
            ));
        }
        let mut selected = BTreeSet::new();
        for id in &self.selected_sheet_ids {
            require_non_nil(id.as_uuid(), "drawing sheet transaction selected sheet")?;
            if !selected.insert(*id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "drawing sheet transaction selected sheet",
                    identity: id.to_string(),
                });
            }
        }
        let mut dispositions = BTreeSet::new();
        for id in &self.applied_sheet_ids {
            require_non_nil(id.as_uuid(), "drawing sheet transaction applied sheet")?;
            if !selected.contains(id) {
                return Err(DesignManagementError::MissingReference {
                    domain: "drawing sheet transaction selected sheet",
                    identity: id.to_string(),
                });
            }
            if !dispositions.insert(*id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "drawing sheet transaction disposition",
                    identity: id.to_string(),
                });
            }
        }
        for id in &self.unchanged_sheet_ids {
            require_non_nil(id.as_uuid(), "drawing sheet transaction unchanged sheet")?;
            if !selected.contains(id) {
                return Err(DesignManagementError::MissingReference {
                    domain: "drawing sheet transaction selected sheet",
                    identity: id.to_string(),
                });
            }
            if !dispositions.insert(*id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "drawing sheet transaction disposition",
                    identity: id.to_string(),
                });
            }
        }
        for skip in &self.skipped {
            skip.validate()?;
            if !selected.contains(&skip.sheet_id) {
                return Err(DesignManagementError::MissingReference {
                    domain: "drawing sheet transaction selected sheet",
                    identity: skip.sheet_id.to_string(),
                });
            }
            if !dispositions.insert(skip.sheet_id) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "drawing sheet transaction disposition",
                    identity: skip.sheet_id.to_string(),
                });
            }
        }
        if dispositions.len() != selected.len() {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet transaction disposition coverage",
            ));
        }
        if self.applied_sheet_ids.is_empty()
            && self.skipped.is_empty()
            && !self.project_default_changed
            && !self.project_preset_saved
            && !self.project_settings_changed
        {
            return Err(DesignManagementError::NoChanges(
                "drawing sheet transaction receipt",
            ));
        }
        Ok(())
    }
}

impl DrawingSheetPreset {
    /// Canonicalize a preset at an authority boundary.
    ///
    /// This is also the bounded migration path for legacy presets that stored
    /// a standard-size enum or repeated sheet title values. The resulting
    /// preset always owns an explicit custom snapshot whose identity exactly
    /// matches the containing preset.
    pub fn normalized_for_storage(mut self) -> Result<Self, DesignManagementError> {
        let (portrait_width_um, portrait_height_um) =
            self.format.authored_size.portrait_dimensions_um();
        let source_preset_unavailable = matches!(
            &self.format.authored_size,
            AuthoredDrawingSheetSize::Custom {
                snapshot: CustomDrawingSheetSnapshot {
                    source_preset_unavailable: true,
                    ..
                }
            }
        );
        let id = self.id.clone();
        let name = self.name.clone();
        self.format = self.format.try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            draft.authored_size = AuthoredDrawingSheetSize::Custom {
                snapshot: CustomDrawingSheetSnapshot {
                    preset_id: Some(id),
                    name,
                    portrait_width_um,
                    portrait_height_um,
                    source_preset_unavailable,
                },
            };
            for field in draft.title_block.fields.values_mut() {
                field.value.clear();
            }
        })?;
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        validate_name("drawing sheet preset id", &self.id)?;
        validate_name("drawing sheet preset name", &self.name)?;
        validate_drawing_sheet_preset_label(&self.name)?;
        self.format.validate()?;
        if self.format.inheritance != DrawingSheetInheritance::Explicit {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet preset inheritance",
            ));
        }
        let AuthoredDrawingSheetSize::Custom { snapshot } = &self.format.authored_size else {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet preset custom snapshot",
            ));
        };
        if snapshot.preset_id.as_deref() != Some(self.id.as_str()) || snapshot.name != self.name {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet preset snapshot identity",
            ));
        }
        validate_no_title_values(&self.format)
    }
}

pub(in crate::design_management) fn validate_drawing_sheet_preset_label(
    name: &str,
) -> Result<(), DesignManagementError> {
    if name.chars().count() > MAX_DRAWING_SHEET_PRESET_NAME_CHARS
        || name.chars().any(|character| {
            matches!(
                character,
                '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'
            )
        })
    {
        return Err(DesignManagementError::InvalidText {
            field: "drawing sheet preset name",
            value: name.to_owned(),
        });
    }
    Ok(())
}

#[must_use]
pub fn drawing_sheet_preset_key(value: &str) -> String {
    case_fold(value)
}

/// Lifecycle state of the controlled schematic document represented by its
/// drawing sheets. This is intentionally independent of catalog and sheet
/// object revisions: those revisions are concurrency authorities, not
/// engineering release identifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetReleaseStatus {
    #[default]
    Draft,
    InReview,
    Released,
    Obsolete,
}

impl DrawingSheetReleaseStatus {
    pub const ALL: [Self; 4] = [Self::Draft, Self::InReview, Self::Released, Self::Obsolete];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::InReview => "In review",
            Self::Released => "Released",
            Self::Obsolete => "Obsolete",
        }
    }
}

/// Project-owned document-control authority projected into every drawing
/// sheet title block and frozen into hardcopy publication identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DrawingSheetDocumentControl {
    pub revision: String,
    /// Exact UTC calendar date in `YYYY-MM-DD` form. Empty means the document
    /// has not been dated for review or release.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub revision_date_utc: String,
    pub status: DrawingSheetReleaseStatus,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub change_reference: String,
}

impl Default for DrawingSheetDocumentControl {
    fn default() -> Self {
        Self {
            revision: "DRAFT".to_owned(),
            revision_date_utc: String::new(),
            status: DrawingSheetReleaseStatus::Draft,
            change_reference: String::new(),
        }
    }
}

impl DrawingSheetDocumentControl {
    pub fn validate(&self) -> Result<(), DesignManagementError> {
        validate_value("drawing sheet document revision", &self.revision, false)?;
        validate_bounded_document_control_text(
            "drawing sheet document revision",
            &self.revision,
            MAX_DRAWING_SHEET_REVISION_BYTES,
        )?;
        validate_value(
            "drawing sheet document revision date",
            &self.revision_date_utc,
            true,
        )?;
        if !self.revision_date_utc.is_empty()
            && !is_exact_iso_calendar_date(&self.revision_date_utc)
        {
            return Err(DesignManagementError::InvalidText {
                field: "drawing sheet document revision date",
                value: self.revision_date_utc.clone(),
            });
        }
        validate_value(
            "drawing sheet document change reference",
            &self.change_reference,
            true,
        )?;
        validate_bounded_document_control_text(
            "drawing sheet document change reference",
            &self.change_reference,
            MAX_DRAWING_SHEET_CHANGE_REFERENCE_BYTES,
        )?;
        if matches!(
            self.status,
            DrawingSheetReleaseStatus::Released | DrawingSheetReleaseStatus::Obsolete
        ) {
            if self.revision.eq_ignore_ascii_case("draft") {
                return Err(DesignManagementError::InvalidText {
                    field: "released drawing sheet document revision",
                    value: self.revision.clone(),
                });
            }
            if self.revision_date_utc.is_empty() {
                return Err(DesignManagementError::InvalidText {
                    field: "released drawing sheet document revision date",
                    value: self.revision_date_utc.clone(),
                });
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn display_date(&self) -> &str {
        if self.revision_date_utc.is_empty() {
            "UNRELEASED"
        } else {
            &self.revision_date_utc
        }
    }
}

fn validate_bounded_document_control_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), DesignManagementError> {
    if value.len() > maximum {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum,
        });
    }
    Ok(())
}

fn is_exact_iso_calendar_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit()))
    {
        return false;
    }
    let Ok(year) = value[0..4].parse::<u16>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u8>() else {
        return false;
    };
    let leap_year =
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let maximum_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    year != 0 && (1..=maximum_day).contains(&day)
}

/// Project-owned drawing-sheet policy.
///
/// Only `Project` presets are accepted here. User and organization presets
/// belong to their respective preference/policy authorities; a project may
/// use one only after capturing it as an explicitly owned project snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetProjectSettings {
    pub default_format: SchematicSheetFormat,
    pub presets: Vec<DrawingSheetPreset>,
    #[serde(default)]
    pub document_control: DrawingSheetDocumentControl,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preset_import_receipts: Vec<DrawingSheetPresetImportReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transaction_receipts: Vec<DrawingSheetTransactionReceipt>,
    /// Canonical project-owned title-block values. Sheets own visibility for
    /// these fields, but never duplicate their values.
    #[serde(default = "default_project_title_block_field_values")]
    pub title_block_field_values: BTreeMap<DrawingSheetTitleFieldId, String>,
    #[serde(default)]
    pub new_sheet_policy: DrawingSheetNewSheetPolicy,
    #[serde(default)]
    pub remember_last_explicit_format: bool,
    #[serde(default)]
    pub last_explicit_format: Option<SchematicSheetFormat>,
}

impl Default for DrawingSheetProjectSettings {
    fn default() -> Self {
        Self {
            default_format: SchematicSheetFormat {
                inheritance: DrawingSheetInheritance::ProjectDefault,
                ..SchematicSheetFormat::default()
            },
            presets: Vec::new(),
            document_control: DrawingSheetDocumentControl::default(),
            preset_import_receipts: Vec::new(),
            transaction_receipts: Vec::new(),
            title_block_field_values: default_project_title_block_field_values(),
            new_sheet_policy: DrawingSheetNewSheetPolicy::ProjectDefault,
            remember_last_explicit_format: false,
            last_explicit_format: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct DrawingSheetProjectSettingsWire {
    default_format: SchematicSheetFormat,
    presets: Vec<DrawingSheetPreset>,
    document_control: DrawingSheetDocumentControl,
    preset_import_receipts: Vec<DrawingSheetPresetImportReceipt>,
    transaction_receipts: Vec<DrawingSheetTransactionReceipt>,
    title_block_field_values: PresentField<BTreeMap<DrawingSheetTitleFieldId, String>>,
    new_sheet_policy: DrawingSheetNewSheetPolicy,
    remember_last_explicit_format: bool,
    last_explicit_format: Option<SchematicSheetFormat>,
}

impl Default for DrawingSheetProjectSettingsWire {
    fn default() -> Self {
        let settings = DrawingSheetProjectSettings::default();
        Self {
            default_format: settings.default_format,
            presets: settings.presets,
            document_control: settings.document_control,
            preset_import_receipts: settings.preset_import_receipts,
            transaction_receipts: settings.transaction_receipts,
            // The wrapper is the schema-presence sentinel. A missing field
            // belongs to the legacy format and must migrate project-owned
            // values from `default_format`; a present map retains its direct
            // serialized shape in both RON and self-describing formats.
            title_block_field_values: PresentField::default(),
            new_sheet_policy: settings.new_sheet_policy,
            remember_last_explicit_format: settings.remember_last_explicit_format,
            last_explicit_format: settings.last_explicit_format,
        }
    }
}

impl<'de> Deserialize<'de> for DrawingSheetProjectSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DrawingSheetProjectSettingsWire::deserialize(deserializer)?;
        let mut default_format = wire.default_format;
        let presets = wire
            .presets
            .into_iter()
            .map(DrawingSheetPreset::normalized_for_storage)
            .collect::<Result<Vec<_>, _>>()
            .map_err(D::Error::custom)?;
        let title_block_field_values = wire.title_block_field_values.0.unwrap_or_else(|| {
            let mut migrated = default_project_title_block_field_values();
            for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
                if let Some(field) = default_format.title_block.fields.get(&id) {
                    migrated.insert(id, field.value.clone());
                }
            }
            migrated
        });
        clear_project_owned_title_values(&mut default_format);
        let mut last_explicit_format = wire.last_explicit_format;
        if let Some(format) = &mut last_explicit_format {
            clear_project_owned_title_values(format);
        }
        let mut settings = Self {
            default_format,
            presets,
            document_control: wire.document_control,
            preset_import_receipts: wire.preset_import_receipts,
            transaction_receipts: wire.transaction_receipts,
            title_block_field_values,
            new_sheet_policy: wire.new_sheet_policy,
            remember_last_explicit_format: wire.remember_last_explicit_format,
            last_explicit_format,
        };
        settings.normalize_preset_references();
        settings.validate().map_err(D::Error::custom)?;
        Ok(settings)
    }
}

impl DrawingSheetProjectSettings {
    pub(in crate::design_management) fn normalize_preset_references(&mut self) {
        let available = self
            .presets
            .iter()
            .map(|preset| {
                (
                    case_fold(&preset.id),
                    (
                        preset.name.clone(),
                        matches!(
                            &preset.format.authored_size,
                            AuthoredDrawingSheetSize::Custom {
                                snapshot: CustomDrawingSheetSnapshot {
                                    source_preset_unavailable: true,
                                    ..
                                }
                            }
                        ),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>();
        normalize_format_preset_reference(&mut self.default_format, &available);
        if let Some(format) = &mut self.last_explicit_format {
            normalize_format_preset_reference(format, &available);
        }
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        if self.default_format.inheritance != DrawingSheetInheritance::ProjectDefault {
            return Err(DesignManagementError::NumericRange(
                "project drawing sheet default inheritance",
            ));
        }
        self.default_format.validate()?;
        self.document_control.validate()?;
        validate_no_project_owned_title_values(&self.default_format)?;
        validate_project_drawing_sheet_title_field_values(&self.title_block_field_values)?;
        if let Some(format) = &self.last_explicit_format {
            format.validate()?;
            validate_no_project_owned_title_values(format)?;
            if format.inheritance != DrawingSheetInheritance::Explicit {
                return Err(DesignManagementError::NumericRange(
                    "last explicit drawing sheet format inheritance",
                ));
            }
        }
        require_limit(
            "drawing sheet project presets",
            self.presets.len(),
            MAX_DRAWING_SHEET_PROJECT_PRESETS,
        )?;
        require_limit(
            "drawing sheet preset import receipts",
            self.preset_import_receipts.len(),
            MAX_DRAWING_SHEET_PRESET_IMPORT_RECEIPTS,
        )?;
        for receipt in &self.preset_import_receipts {
            receipt.validate()?;
        }
        require_limit(
            "drawing sheet transaction receipts",
            self.transaction_receipts.len(),
            MAX_DRAWING_SHEET_TRANSACTION_RECEIPTS,
        )?;
        let mut receipt_revisions = BTreeSet::new();
        for receipt in &self.transaction_receipts {
            receipt.validate()?;
            if !receipt_revisions.insert(receipt.catalog_revision) {
                return Err(DesignManagementError::DuplicateIdentity {
                    domain: "drawing sheet transaction receipt revision",
                    identity: receipt.catalog_revision.to_string(),
                });
            }
        }
        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();
        for preset in &self.presets {
            preset.validate()?;
            if preset.scope != DrawingSheetPresetScope::Project
                || preset.format.inheritance != DrawingSheetInheritance::Explicit
            {
                return Err(DesignManagementError::NumericRange(
                    "project drawing sheet preset ownership",
                ));
            }
            if !ids.insert(case_fold(&preset.id)) {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset id",
                    value: preset.id.clone(),
                });
            }
            if !names.insert(case_fold(&preset.name)) {
                return Err(DesignManagementError::DuplicateListEntry {
                    field: "drawing sheet preset name",
                    value: preset.name.clone(),
                });
            }
        }
        let available = self
            .presets
            .iter()
            .map(|preset| {
                (
                    case_fold(&preset.id),
                    (
                        preset.name.clone(),
                        matches!(
                            &preset.format.authored_size,
                            AuthoredDrawingSheetSize::Custom {
                                snapshot: CustomDrawingSheetSnapshot {
                                    source_preset_unavailable: true,
                                    ..
                                }
                            }
                        ),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>();
        validate_format_preset_reference(&self.default_format, &available)?;
        if let Some(format) = &self.last_explicit_format {
            validate_format_preset_reference(format, &available)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn find_preset(&self, id: &str) -> Option<&DrawingSheetPreset> {
        let id = case_fold(id);
        self.presets
            .iter()
            .find(|preset| case_fold(&preset.id) == id)
    }
}

pub fn validate_project_drawing_sheet_title_field_values(
    values: &BTreeMap<DrawingSheetTitleFieldId, String>,
) -> Result<(), DesignManagementError> {
    if values.len() != DrawingSheetTitleFieldId::PROJECT_OWNED.len()
        || DrawingSheetTitleFieldId::PROJECT_OWNED
            .into_iter()
            .any(|field| !values.contains_key(&field))
    {
        return Err(DesignManagementError::NumericRange(
            "project drawing sheet title field set",
        ));
    }
    for (field, value) in values {
        if !field.is_project_owned() {
            return Err(DesignManagementError::NumericRange(
                "project drawing sheet title field ownership",
            ));
        }
        validate_value("project drawing sheet title field", value, true)?;
    }
    Ok(())
}

fn default_project_title_block_field_values() -> BTreeMap<DrawingSheetTitleFieldId, String> {
    DrawingSheetTitleFieldId::PROJECT_OWNED
        .into_iter()
        .map(|field| (field, String::new()))
        .collect()
}

pub(super) fn clear_project_owned_title_values(format: &mut SchematicSheetFormat) {
    for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
        format
            .title_block
            .fields
            .entry(id)
            .or_default()
            .value
            .clear();
    }
}

pub(super) fn clear_all_title_values(format: &mut SchematicSheetFormat) {
    for field in format.title_block.fields.values_mut() {
        field.value.clear();
    }
}

fn validate_no_title_values(format: &SchematicSheetFormat) -> Result<(), DesignManagementError> {
    if format
        .title_block
        .fields
        .values()
        .any(|field| !field.value.is_empty())
    {
        return Err(DesignManagementError::InvalidText {
            field: "authored drawing sheet title value in preset",
            value: "preset formats retain visibility but never authored values".to_owned(),
        });
    }
    Ok(())
}

fn validate_no_project_owned_title_values(
    format: &SchematicSheetFormat,
) -> Result<(), DesignManagementError> {
    if DrawingSheetTitleFieldId::PROJECT_OWNED
        .into_iter()
        .any(|id| {
            format
                .title_block
                .fields
                .get(&id)
                .is_some_and(|field| !field.value.is_empty())
        })
    {
        return Err(DesignManagementError::InvalidText {
            field: "project-owned drawing sheet title value in format",
            value: "value must be stored by project authority".to_owned(),
        });
    }
    Ok(())
}

pub(in crate::design_management) fn normalize_format_preset_reference(
    format: &mut SchematicSheetFormat,
    available: &BTreeMap<String, (String, bool)>,
) {
    let AuthoredDrawingSheetSize::Custom { snapshot } = &mut format.authored_size else {
        return;
    };
    let Some(id) = snapshot.preset_id.as_ref() else {
        snapshot.source_preset_unavailable = false;
        return;
    };
    if let Some((name, unavailable)) = available.get(&case_fold(id)) {
        snapshot.name.clone_from(name);
        snapshot.source_preset_unavailable = *unavailable;
    } else {
        snapshot.source_preset_unavailable = true;
    }
}

pub fn validate_format_preset_reference(
    format: &SchematicSheetFormat,
    available: &BTreeMap<String, (String, bool)>,
) -> Result<(), DesignManagementError> {
    let AuthoredDrawingSheetSize::Custom { snapshot } = &format.authored_size else {
        return Ok(());
    };
    let Some(id) = snapshot.preset_id.as_ref() else {
        if snapshot.source_preset_unavailable {
            return Err(DesignManagementError::NumericRange(
                "unavailable drawing sheet preset without identity",
            ));
        }
        return Ok(());
    };
    let expected_unavailable = available
        .get(&case_fold(id))
        .is_none_or(|(_, unavailable)| *unavailable);
    if let Some((name, _)) = available.get(&case_fold(id))
        && snapshot.name != *name
    {
        return Err(DesignManagementError::InvalidText {
            field: "drawing sheet preset reference name",
            value: snapshot.name.clone(),
        });
    }
    if snapshot.source_preset_unavailable != expected_unavailable {
        return Err(DesignManagementError::MissingReference {
            domain: "drawing sheet preset",
            identity: id.clone(),
        });
    }
    Ok(())
}
