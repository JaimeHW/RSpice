//! Exact draft, validation, and conflict model for custom sizes.
//!
//! The signed package contract this dialog imports and exports lives in
//! `rspice-design-model::sheet_package`, shared byte-for-byte with the offline
//! publisher. Only the editing surfaces are here.

use std::collections::BTreeSet;

use uuid::Uuid;

// The editor and its renderer both name these; they are the model's, not the
// dialog's, so they are imported rather than defined and passed straight on.
pub(super) use rspice_design_model::sheet_authoring::{StartingFrame, custom_format};
use rspice_design_model::sheet_package::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPresetPackage,
    PortableDrawingSheetPreset, portable_from_preset,
};

use crate::state::{
    AuthoredDrawingSheetSize, CustomDrawingSheetSnapshot, DesignManagementCatalog,
    DrawingSheetBorderTemplate, DrawingSheetDisplayUnit, DrawingSheetInheritance,
    DrawingSheetPreset, DrawingSheetPresetScope, DrawingSheetTitleBlockTemplate,
    MAX_DRAWING_SHEET_PRESET_NAME_CHARS, SchematicSheetFormat,
};
use crate::workbench::DrawingSheetPersonalPreferences;

pub(super) const MAX_PACKAGE_BYTES: usize = DRAWING_SHEET_PACKAGE_MAX_BYTES;

pub(super) type PresetPackageFormat = DrawingSheetPackageEncoding;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum PresetEditorUnit {
    #[default]
    Millimetres,
    Centimetres,
    Inches,
}

impl PresetEditorUnit {
    pub(super) const fn suffix(self) -> &'static str {
        match self {
            Self::Millimetres => "mm",
            Self::Centimetres => "cm",
            Self::Inches => "in",
        }
    }

    const fn micrometres_per_unit(self) -> u64 {
        match self {
            Self::Millimetres => 1_000,
            Self::Centimetres => 10_000,
            Self::Inches => 25_400,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum PresetEditorMode {
    #[default]
    Create,
    Edit,
}

#[derive(Debug, Clone)]
pub(super) struct PresetEditorDraft {
    pub(super) mode: PresetEditorMode,
    pub(super) source_id: Option<String>,
    pub(super) name: String,
    pub(super) scope: DrawingSheetPresetScope,
    pub(super) width: String,
    pub(super) height: String,
    pub(super) unit: PresetEditorUnit,
    pub(super) frame: StartingFrame,
    pub(super) unavailable: bool,
    pub(super) baseline: Option<DrawingSheetPreset>,
    pub(super) last_valid_preview: SchematicSheetFormat,
    pub(super) error: Option<String>,
}

impl Default for PresetEditorDraft {
    fn default() -> Self {
        let format = custom_format(
            "Lab documentation panel",
            250_000,
            400_000,
            StartingFrame::IsoA,
        )
        .expect("the mockup's canonical editor seed is valid");
        Self {
            mode: PresetEditorMode::Create,
            source_id: None,
            name: "Lab documentation panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            width: "250".to_owned(),
            height: "400".to_owned(),
            unit: PresetEditorUnit::Millimetres,
            frame: StartingFrame::IsoA,
            unavailable: false,
            baseline: None,
            last_valid_preview: format,
            error: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum TransferMode {
    #[default]
    Import,
    Export,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum ImportResolution {
    #[default]
    NewIdentity,
    MatchesByDigest,
    KeepBothRename,
    MapExisting,
    ReplaceManagedDependencies,
    RetainUnavailableDependency,
    Skip,
}

#[derive(Debug, Clone)]
pub(super) struct ImportCandidate {
    pub(super) portable: PortableDrawingSheetPreset,
    pub(super) selected: bool,
    pub(super) resolution: ImportResolution,
    pub(super) existing_id: Option<String>,
    pub(super) missing_managed_dependency: bool,
}

#[derive(Debug, Clone)]
pub(super) struct PresetTransferState {
    pub(super) open: bool,
    pub(super) mode: TransferMode,
    pub(super) package_name: String,
    pub(super) json: String,
    pub(super) export_ids: BTreeSet<String>,
    pub(super) import_candidates: Vec<ImportCandidate>,
    pub(super) reviewed_digest: Option<String>,
    pub(super) package_format: PresetPackageFormat,
    pub(super) include_builtin_frame_references: bool,
    pub(super) include_source_metadata: bool,
    pub(super) error: Option<String>,
    /// What an export in flight will have to report once its destination
    /// picker comes back. The count and digest are known when the picker
    /// opens and the destination only when it closes, so neither end can
    /// compose the sentence alone.
    pub(super) pending_export: Option<PendingExport>,
}

/// The half of an export receipt that is known before a destination is.
#[derive(Debug, Clone)]
pub(super) struct PendingExport {
    pub(super) preset_count: usize,
    pub(super) digest: String,
}

impl Default for PresetTransferState {
    fn default() -> Self {
        Self {
            open: false,
            mode: TransferMode::Import,
            package_name: String::new(),
            json: String::new(),
            export_ids: BTreeSet::new(),
            import_candidates: Vec::new(),
            reviewed_digest: None,
            package_format: PresetPackageFormat::CanonicalSchema1,
            include_builtin_frame_references: true,
            include_source_metadata: true,
            error: None,
            pending_export: None,
        }
    }
}
pub(super) fn parse_dimension_um(
    text: &str,
    unit: PresetEditorUnit,
    field: &str,
) -> Result<u64, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err(format!("{field} must be a number."));
    }
    if text.starts_with('-') {
        return Err(format!("{field} must be greater than zero."));
    }
    let text = text.strip_prefix('+').unwrap_or(text);
    let mut parts = text.split('.');
    let whole = parts
        .next()
        .filter(|part| !part.is_empty())
        .ok_or_else(|| format!("{field} must be a number."))?;
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some()
        || !whole.chars().all(|character| character.is_ascii_digit())
        || !fraction.chars().all(|character| character.is_ascii_digit())
    {
        return Err(format!("{field} must be a number."));
    }
    let multiplier = unit.micrometres_per_unit();
    let whole = whole
        .parse::<u64>()
        .map_err(|_| format!("{field} is outside the supported range."))?;
    let mut value = whole
        .checked_mul(multiplier)
        .ok_or_else(|| format!("{field} is outside the supported range."))?;
    if !fraction.is_empty() {
        let numerator = fraction
            .parse::<u128>()
            .map_err(|_| format!("{field} must be a number."))?;
        let denominator = 10_u128
            .checked_pow(
                u32::try_from(fraction.len())
                    .map_err(|_| format!("{field} has too many decimal places."))?,
            )
            .ok_or_else(|| format!("{field} has too many decimal places."))?;
        let scaled = numerator
            .checked_mul(u128::from(multiplier))
            .ok_or_else(|| format!("{field} is outside the supported range."))?;
        let rounded = scaled
            .checked_add(denominator / 2)
            .ok_or_else(|| format!("{field} is outside the supported range."))?
            / denominator;
        value = value
            .checked_add(
                u64::try_from(rounded)
                    .map_err(|_| format!("{field} is outside the supported range."))?,
            )
            .ok_or_else(|| format!("{field} is outside the supported range."))?;
    }
    if value == 0 {
        return Err(format!("{field} must be greater than zero."));
    }
    Ok(value)
}

pub(super) fn format_dimension_um(value_um: u64, unit: PresetEditorUnit) -> String {
    let display = match unit {
        PresetEditorUnit::Millimetres => DrawingSheetDisplayUnit::Millimetres,
        PresetEditorUnit::Centimetres => DrawingSheetDisplayUnit::Centimetres,
        PresetEditorUnit::Inches => DrawingSheetDisplayUnit::Inches,
    };
    display.format_um(value_um)
}

pub(super) fn validate_preset_name(
    name: &str,
    presets: &[DrawingSheetPreset],
    ignore_id: Option<&str>,
) -> Result<String, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Name the custom size before saving it as a preset.".to_owned());
    }
    if name.chars().count() > MAX_DRAWING_SHEET_PRESET_NAME_CHARS {
        return Err(format!(
            "Preset names are limited to {MAX_DRAWING_SHEET_PRESET_NAME_CHARS} characters."
        ));
    }
    if name.chars().any(|character| {
        matches!(
            character,
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'
        )
    }) {
        return Err("A preset name cannot contain \\ / : * ? \" < > or |.".to_owned());
    }
    if let Some(clash) = presets.iter().find(|preset| {
        !ignore_id.is_some_and(|id| preset.id.eq_ignore_ascii_case(id))
            && preset.name.eq_ignore_ascii_case(name)
    }) {
        return Err(format!(
            "A {} preset is already named '{}'. Choose another name or edit that preset.",
            match clash.scope {
                DrawingSheetPresetScope::Project => "project",
                DrawingSheetPresetScope::User => "personal",
                DrawingSheetPresetScope::Organization => "organization",
            },
            clash.name
        ));
    }
    Ok(name.to_owned())
}

pub(super) fn preset_from_editor(
    draft: &PresetEditorDraft,
    visible: &[DrawingSheetPreset],
) -> Result<DrawingSheetPreset, String> {
    let name = validate_preset_name(&draft.name, visible, draft.source_id.as_deref())?;
    let width_um = parse_dimension_um(&draft.width, draft.unit, "Width")?;
    let height_um = parse_dimension_um(&draft.height, draft.unit, "Height")?;
    let mut format = custom_format(&name, width_um, height_um, draft.frame)?;
    let id = draft
        .source_id
        .clone()
        .unwrap_or_else(|| format!("custom-{}", Uuid::new_v4().simple()));
    format = format
        .try_update(|format| {
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut format.authored_size {
                snapshot.preset_id = Some(id.clone());
            }
        })
        .map_err(|error| error.to_string())?;
    Ok(DrawingSheetPreset {
        id,
        name,
        scope: draft.scope,
        format: format.as_reusable_drawing_sheet_preset(),
    })
}
pub(super) fn prepare_import_candidates(
    package: &DrawingSheetPresetPackage,
    existing: &[DrawingSheetPreset],
) -> Result<Vec<ImportCandidate>, String> {
    let mut candidates = Vec::with_capacity(package.presets.len());
    for portable in &package.presets {
        let matching_id = existing
            .iter()
            .find(|preset| preset.id.eq_ignore_ascii_case(&portable.stable_id));
        let matching_name = existing
            .iter()
            .find(|preset| preset.name.eq_ignore_ascii_case(&portable.name));
        let exact_match = matching_id
            .or(matching_name)
            .is_some_and(|preset| portable_from_preset(preset).ok().as_ref() == Some(portable));
        let missing_managed_dependency = matches!(
            portable.format.border,
            DrawingSheetBorderTemplate::OrganizationManaged
        ) || matches!(
            portable.format.title_block.template,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
        let (selected, resolution, existing_id) = if exact_match {
            (
                true,
                ImportResolution::MatchesByDigest,
                matching_id
                    .or(matching_name)
                    .map(|preset| preset.id.clone()),
            )
        } else if matching_id.is_some() || matching_name.is_some() {
            (
                false,
                ImportResolution::KeepBothRename,
                matching_id
                    .or(matching_name)
                    .map(|preset| preset.id.clone()),
            )
        } else if missing_managed_dependency {
            (false, ImportResolution::ReplaceManagedDependencies, None)
        } else {
            (true, ImportResolution::NewIdentity, None)
        };
        candidates.push(ImportCandidate {
            portable: portable.clone(),
            selected,
            resolution,
            existing_id,
            missing_managed_dependency,
        });
    }
    Ok(candidates)
}

pub(super) fn imported_project_preset(
    candidate: &ImportCandidate,
    visible: &[DrawingSheetPreset],
) -> Result<Option<DrawingSheetPreset>, String> {
    if !candidate.selected
        || matches!(
            candidate.resolution,
            ImportResolution::MatchesByDigest
                | ImportResolution::MapExisting
                | ImportResolution::Skip
        )
    {
        return Ok(None);
    }
    if candidate.missing_managed_dependency
        && !matches!(
            candidate.resolution,
            ImportResolution::ReplaceManagedDependencies
                | ImportResolution::RetainUnavailableDependency
        )
    {
        return Err(format!(
            "Preset '{}' has unavailable managed dependencies. Choose an explicit replacement, retain it as unavailable, map it to an existing preset, or skip it.",
            candidate.portable.name
        ));
    }
    let mut id = candidate.portable.stable_id.clone();
    let mut name = candidate.portable.name.clone();
    if candidate.resolution == ImportResolution::KeepBothRename
        || visible
            .iter()
            .any(|preset| preset.id.eq_ignore_ascii_case(&id))
        || visible
            .iter()
            .any(|preset| preset.name.eq_ignore_ascii_case(&name))
    {
        id = format!("custom-{}", Uuid::new_v4().simple());
        name = unique_copy_name(&format!("{name} imported"), visible);
    }
    let mut format = candidate.portable.format.clone();
    format = format
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            if candidate.resolution == ImportResolution::ReplaceManagedDependencies {
                if draft.border == DrawingSheetBorderTemplate::OrganizationManaged {
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                }
                if draft.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged
                {
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
            }
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                snapshot.preset_id = Some(id.clone());
                snapshot.name.clone_from(&name);
                snapshot.source_preset_unavailable =
                    candidate.resolution == ImportResolution::RetainUnavailableDependency;
            }
        })
        .map_err(|error| error.to_string())?;
    Ok(Some(DrawingSheetPreset {
        id,
        name,
        scope: DrawingSheetPresetScope::Project,
        format: format.as_reusable_drawing_sheet_preset(),
    }))
}

pub(super) fn unique_copy_name(base: &str, visible: &[DrawingSheetPreset]) -> String {
    if !visible
        .iter()
        .any(|preset| preset.name.eq_ignore_ascii_case(base))
    {
        return base.to_owned();
    }
    (2_u32..)
        .map(|suffix| format!("{base} {suffix}"))
        .find(|candidate| {
            !visible
                .iter()
                .any(|preset| preset.name.eq_ignore_ascii_case(candidate))
        })
        .expect("a bounded preset catalog always has a free display suffix")
}

pub(crate) fn capture_personal_preset_into_project(
    candidate: &mut DesignManagementCatalog,
    personal: &DrawingSheetPreset,
) -> Result<DrawingSheetPreset, String> {
    if personal.scope != DrawingSheetPresetScope::User {
        return Err("Only a personal preset requires project capture.".to_owned());
    }
    if let Some(existing) = candidate
        .drawing_sheet_settings()
        .presets
        .iter()
        .find(|preset| same_preset_content(&preset.format, &personal.format))
    {
        return Ok(existing.clone());
    }
    let visible = candidate.drawing_sheet_settings().presets.clone();
    let id = if visible
        .iter()
        .any(|preset| preset.id.eq_ignore_ascii_case(&personal.id))
    {
        format!("custom-{}", Uuid::new_v4().simple())
    } else {
        personal.id.clone()
    };
    let name = unique_copy_name(&personal.name, &visible);
    let mut format = personal
        .format
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                snapshot.preset_id = Some(id.clone());
                snapshot.name.clone_from(&name);
                snapshot.source_preset_unavailable = false;
            }
        })
        .map_err(|error| error.to_string())?;
    format.inheritance = DrawingSheetInheritance::Explicit;
    let captured = DrawingSheetPreset {
        id,
        name,
        scope: DrawingSheetPresetScope::Project,
        format: format.as_reusable_drawing_sheet_preset(),
    };
    candidate
        .publish_drawing_sheet_preset(candidate.revision(), captured.clone())
        .map_err(|error| error.to_string())?;
    Ok(captured)
}

fn same_preset_content(left: &SchematicSheetFormat, right: &SchematicSheetFormat) -> bool {
    let normalize = |format: &SchematicSheetFormat| {
        format
            .try_update(|draft| {
                draft.inheritance = DrawingSheetInheritance::Explicit;
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = None;
                    snapshot.name = "content comparison".to_owned();
                    snapshot.source_preset_unavailable = false;
                }
            })
            .ok()
    };
    normalize(left) == normalize(right)
}

pub(super) fn all_visible_presets(
    project: &DesignManagementCatalog,
    personal: &DrawingSheetPersonalPreferences,
) -> Vec<DrawingSheetPreset> {
    project
        .drawing_sheet_settings()
        .presets
        .iter()
        .chain(&personal.presets)
        .filter(|preset| {
            matches!(
                preset.format.authored_size,
                AuthoredDrawingSheetSize::Custom { .. }
            )
        })
        .cloned()
        .collect()
}

pub(super) fn unavailable(preset: &DrawingSheetPreset) -> bool {
    matches!(
        &preset.format.authored_size,
        AuthoredDrawingSheetSize::Custom {
            snapshot: CustomDrawingSheetSnapshot {
                source_preset_unavailable: true,
                ..
            }
        }
    )
}

/// Organization definitions may be redistributed only by publisher tooling
/// that can produce a trusted signature. The interactive editor deliberately
/// has no access to private publisher keys.
pub(super) fn unsigned_exportable(preset: &DrawingSheetPreset) -> bool {
    !unavailable(preset) && preset.scope != DrawingSheetPresetScope::Organization
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_dimensions_round_once_to_exact_micrometres() {
        assert_eq!(
            parse_dimension_um("8.5", PresetEditorUnit::Inches, "Width").unwrap(),
            215_900
        );
        assert_eq!(
            parse_dimension_um("215.9004", PresetEditorUnit::Millimetres, "Width").unwrap(),
            215_900
        );
        assert_eq!(
            parse_dimension_um("215.9006", PresetEditorUnit::Millimetres, "Width").unwrap(),
            215_901
        );
    }

    #[test]
    fn personal_use_captures_a_project_owned_snapshot() {
        let personal = DrawingSheetPreset {
            id: "review-strip".to_owned(),
            name: "Review strip".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: custom_format("Review strip", 210_000, 594_000, StartingFrame::Plain).unwrap(),
        };
        let mut project = DesignManagementCatalog::default();
        let captured = capture_personal_preset_into_project(&mut project, &personal).unwrap();
        assert_eq!(captured.scope, DrawingSheetPresetScope::Project);
        assert_eq!(
            project.drawing_sheet_settings().find_preset("review-strip"),
            Some(&captured)
        );
    }

    #[test]
    fn missing_managed_dependency_requires_an_explicit_resolution() {
        let managed_format = custom_format("Managed panel", 250_000, 400_000, StartingFrame::Plain)
            .unwrap()
            .try_update(|draft| {
                draft.border = DrawingSheetBorderTemplate::OrganizationManaged;
                draft.marks = DrawingSheetBorderTemplate::OrganizationManaged.default_marks();
            })
            .unwrap();
        let portable = portable_from_preset(&DrawingSheetPreset {
            id: "managed-panel".to_owned(),
            name: "Managed panel".to_owned(),
            scope: DrawingSheetPresetScope::Organization,
            format: managed_format,
        })
        .unwrap();
        let mut candidate = ImportCandidate {
            portable,
            selected: true,
            resolution: ImportResolution::NewIdentity,
            existing_id: None,
            missing_managed_dependency: true,
        };

        assert!(imported_project_preset(&candidate, &[]).is_err());
        candidate.resolution = ImportResolution::ReplaceManagedDependencies;
        let resolved = imported_project_preset(&candidate, &[])
            .unwrap()
            .expect("explicit replacement produces a project preset");
        assert_eq!(resolved.format.border, DrawingSheetBorderTemplate::Standard);
        assert!(!unavailable(&resolved));
    }
}
