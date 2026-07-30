//! Exact draft, validation, package, and conflict model for custom sizes.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::state::{
    AuthoredDrawingSheetSize, CustomDrawingSheetSnapshot, DesignManagementCatalog,
    DrawingSheetBorderTemplate, DrawingSheetDisplayUnit, DrawingSheetInheritance,
    DrawingSheetMargins, DrawingSheetPreset, DrawingSheetPresetScope,
    DrawingSheetTitleBlockTemplate, DrawingSheetZones, MAX_DRAWING_SHEET_PRESET_NAME_CHARS,
    SchematicPageOrientation, SchematicSheetFormat,
};
use crate::workbench::DrawingSheetPersonalPreferences;

pub(super) const PACKAGE_SCHEMA: &str = "rspice-sheet-formats";
pub(super) const PACKAGE_VERSION: u16 = 1;
pub(super) const MAX_PACKAGE_BYTES: usize = 4 * 1024 * 1024;

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
pub(super) enum StartingFrame {
    #[default]
    IsoA,
    AnsiA,
    Plain,
    None,
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
pub(super) enum PresetPackageFormat {
    #[default]
    CanonicalSchema1,
    HumanReviewJson,
}

impl PresetPackageFormat {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::CanonicalSchema1 => "RSpice sheet formats \u{00b7} schema 1",
            Self::HumanReviewJson => "JSON + schema \u{00b7} human review",
        }
    }
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DrawingSheetPresetPackage {
    pub(super) schema: String,
    pub(super) version: u16,
    pub(super) source_digest_sha256: String,
    pub(super) includes_builtin_frame_references: bool,
    pub(super) includes_source_metadata: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) builtin_frame_references: Vec<BuiltinFrameReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) source_metadata: Option<PresetPackageSourceMetadata>,
    pub(super) presets: Vec<PortableDrawingSheetPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BuiltinFrameReference {
    pub(super) kind: String,
    pub(super) identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PresetPackageSourceMetadata {
    pub(super) producer: String,
    pub(super) content: String,
    pub(super) exact_dimension_unit: String,
    pub(super) source_scopes: Vec<DrawingSheetPresetScope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PortableDrawingSheetPreset {
    pub(super) stable_id: String,
    pub(super) name: String,
    pub(super) source_scope: DrawingSheetPresetScope,
    pub(super) portrait_width_um: u64,
    pub(super) portrait_height_um: u64,
    pub(super) format: SchematicSheetFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewPackageDocument {
    document: String,
    schema: HumanReviewSchemaDescriptor,
    package: DrawingSheetPresetPackage,
    review: Vec<HumanReviewPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewSchemaDescriptor {
    identity: String,
    version: u16,
    representation: String,
    exact_dimension_unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewPreset {
    stable_id: String,
    name: String,
    source_scope: DrawingSheetPresetScope,
    portrait_dimensions: String,
    portrait_width_um: u64,
    portrait_height_um: u64,
    dependencies: Vec<String>,
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

pub(super) fn validate_dimensions(width_um: u64, height_um: u64) -> Result<(), String> {
    for (field, value) in [("Width", width_um), ("Height", height_um)] {
        if value < crate::state::DRAWING_SHEET_MIN_EDGE_UM {
            return Err(format!("{field} is below the 50.8 mm minimum."));
        }
        if value > crate::state::DRAWING_SHEET_MAX_EDGE_UM {
            return Err(format!("{field} exceeds the 2540 mm maximum."));
        }
    }
    let short = width_um.min(height_um);
    let long = width_um.max(height_um);
    if u128::from(long)
        > u128::from(short) * u128::from(crate::state::DRAWING_SHEET_MAX_ASPECT_RATIO)
    {
        return Err(format!(
            "Aspect ratio exceeds the {}:1 limit; a sheet this narrow cannot carry a border or title block.",
            crate::state::DRAWING_SHEET_MAX_ASPECT_RATIO
        ));
    }
    Ok(())
}

pub(super) fn custom_format(
    name: &str,
    width_um: u64,
    height_um: u64,
    frame: StartingFrame,
) -> Result<SchematicSheetFormat, String> {
    validate_dimensions(width_um, height_um)?;
    let mut format = SchematicSheetFormat::try_custom(
        name,
        width_um,
        height_um,
        SchematicPageOrientation::Portrait,
    )
    .map_err(|error| error.to_string())?;
    format = format
        .try_update(|draft| {
            draft.inheritance = DrawingSheetInheritance::Explicit;
            match frame {
                StartingFrame::IsoA => {
                    draft.margins = DrawingSheetMargins {
                        top_um: 10_000,
                        right_um: 10_000,
                        bottom_um: 10_000,
                        left_um: 20_000,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::AnsiA => {
                    draft.margins = DrawingSheetMargins {
                        top_um: 12_700,
                        right_um: 12_700,
                        bottom_um: 12_700,
                        left_um: 19_050,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Standard);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::Plain => {
                    draft.margins = DrawingSheetMargins {
                        top_um: 10_000,
                        right_um: 10_000,
                        bottom_um: 10_000,
                        left_um: 10_000,
                    };
                    draft.apply_border_template(DrawingSheetBorderTemplate::Plain);
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::Compact;
                }
                StartingFrame::None => {
                    draft.margins = DrawingSheetMargins::zero();
                    draft.apply_border_template(DrawingSheetBorderTemplate::None);
                    draft.zones = DrawingSheetZones::none();
                    draft.title_block.template = DrawingSheetTitleBlockTemplate::None;
                }
            }
        })
        .map_err(|error| error.to_string())?;
    Ok(format)
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

pub(super) fn portable_from_preset(
    preset: &DrawingSheetPreset,
) -> Result<PortableDrawingSheetPreset, String> {
    let preset = preset
        .clone()
        .normalized_for_storage()
        .map_err(|error| error.to_string())?;
    let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
        return Err(format!(
            "'{}' is not a custom physical sheet size.",
            preset.name
        ));
    };
    Ok(PortableDrawingSheetPreset {
        stable_id: preset.id.clone(),
        name: preset.name.clone(),
        source_scope: preset.scope,
        portrait_width_um: snapshot.portrait_width_um,
        portrait_height_um: snapshot.portrait_height_um,
        format: preset.format.as_reusable_drawing_sheet_preset(),
    })
}

#[cfg(test)]
pub(super) fn build_package(
    presets: impl IntoIterator<Item = DrawingSheetPreset>,
) -> Result<DrawingSheetPresetPackage, String> {
    build_package_with_options(presets, true, true)
}

pub(super) fn build_package_with_options(
    presets: impl IntoIterator<Item = DrawingSheetPreset>,
    include_builtin_frame_references: bool,
    include_source_metadata: bool,
) -> Result<DrawingSheetPresetPackage, String> {
    let mut presets = presets
        .into_iter()
        .map(|preset| portable_from_preset(&preset))
        .collect::<Result<Vec<_>, _>>()?;
    presets.sort_by(|left, right| {
        left.stable_id
            .cmp(&right.stable_id)
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| scope_rank(left.source_scope).cmp(&scope_rank(right.source_scope)))
    });
    let builtin_frame_references = if include_builtin_frame_references {
        builtin_frame_references(&presets)
    } else {
        Vec::new()
    };
    let source_metadata = include_source_metadata.then(|| source_metadata(&presets));
    let mut package = DrawingSheetPresetPackage {
        schema: PACKAGE_SCHEMA.to_owned(),
        version: PACKAGE_VERSION,
        source_digest_sha256: String::new(),
        includes_builtin_frame_references: include_builtin_frame_references,
        includes_source_metadata: include_source_metadata,
        builtin_frame_references,
        source_metadata,
        presets,
    };
    package.source_digest_sha256 = package_digest(&package)?;
    validate_package(&package)?;
    Ok(package)
}

#[cfg(test)]
pub(super) fn encode_package(package: &DrawingSheetPresetPackage) -> Result<String, String> {
    encode_package_with_format(package, PresetPackageFormat::CanonicalSchema1)
}

pub(super) fn encode_package_with_format(
    package: &DrawingSheetPresetPackage,
    format: PresetPackageFormat,
) -> Result<String, String> {
    validate_package(package)?;
    match format {
        PresetPackageFormat::CanonicalSchema1 => serde_json::to_string_pretty(package),
        PresetPackageFormat::HumanReviewJson => {
            let document = HumanReviewPackageDocument {
                document: "rspice-sheet-formats-human-review".to_owned(),
                schema: HumanReviewSchemaDescriptor {
                    identity: PACKAGE_SCHEMA.to_owned(),
                    version: PACKAGE_VERSION,
                    representation: "human-review".to_owned(),
                    exact_dimension_unit: "micrometre".to_owned(),
                },
                package: package.clone(),
                review: human_review_rows(&package.presets),
            };
            serde_json::to_string_pretty(&document)
        }
    }
    .map_err(|error| format!("Could not encode the sheet-format package: {error}"))
}

pub(super) fn parse_package(source: &str) -> Result<DrawingSheetPresetPackage, String> {
    if source.len() > MAX_PACKAGE_BYTES {
        return Err(format!(
            "The preset package exceeds the {} byte limit.",
            MAX_PACKAGE_BYTES
        ));
    }
    let value = serde_json::from_str::<serde_json::Value>(source)
        .map_err(|error| format!("The preset package is not valid JSON: {error}"))?;
    let package = if value
        .get("document")
        .and_then(serde_json::Value::as_str)
        .is_some()
    {
        let document =
            serde_json::from_value::<HumanReviewPackageDocument>(value).map_err(|error| {
                format!("The human-review package is not valid schema-1 JSON: {error}")
            })?;
        validate_human_review_document(&document)?;
        document.package
    } else {
        serde_json::from_value::<DrawingSheetPresetPackage>(value)
            .map_err(|error| format!("The preset package is not valid schema-1 JSON: {error}"))?
    };
    validate_package(&package)?;
    Ok(package)
}

fn validate_package(package: &DrawingSheetPresetPackage) -> Result<(), String> {
    if package.schema != PACKAGE_SCHEMA || package.version != PACKAGE_VERSION {
        return Err(format!(
            "Unsupported sheet-format package '{}', version {}.",
            package.schema, package.version
        ));
    }
    if package.presets.len() > crate::state::MAX_DRAWING_SHEET_PROJECT_PRESETS {
        return Err("The preset package contains too many definitions.".to_owned());
    }
    if !package.includes_builtin_frame_references && !package.builtin_frame_references.is_empty() {
        return Err(
            "The built-in frame-reference declaration does not match its manifest.".to_owned(),
        );
    }
    let expected_references = builtin_frame_references(&package.presets);
    if package.includes_builtin_frame_references
        && package.builtin_frame_references != expected_references
    {
        return Err(
            "The built-in frame-reference manifest does not match the preset definitions."
                .to_owned(),
        );
    }
    if !package.includes_source_metadata && package.source_metadata.is_some() {
        return Err("The source-metadata declaration does not match its payload.".to_owned());
    }
    if package.includes_source_metadata
        && package.source_metadata.as_ref() != Some(&source_metadata(&package.presets))
    {
        return Err("The source metadata does not match the preset definitions.".to_owned());
    }
    let digest = package_digest(package)?;
    if digest != package.source_digest_sha256 {
        return Err("The preset package digest does not match its contract.".to_owned());
    }
    let mut ids = BTreeSet::new();
    let mut names = BTreeSet::new();
    for preset in &package.presets {
        if !ids.insert(preset.stable_id.to_lowercase()) {
            return Err(format!(
                "Preset identity '{}' is duplicated in the package.",
                preset.stable_id
            ));
        }
        if !names.insert(preset.name.to_lowercase()) {
            return Err(format!(
                "Preset name '{}' is duplicated in the package.",
                preset.name
            ));
        }
        preset
            .format
            .validate()
            .map_err(|error| error.to_string())?;
        let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
            return Err(format!(
                "Preset '{}' is not a custom physical size.",
                preset.name
            ));
        };
        if snapshot.preset_id.as_deref() != Some(preset.stable_id.as_str())
            || snapshot.name != preset.name
        {
            return Err(format!(
                "Preset '{}' does not match its embedded custom-size identity.",
                preset.name
            ));
        }
        if snapshot.portrait_width_um != preset.portrait_width_um
            || snapshot.portrait_height_um != preset.portrait_height_um
        {
            return Err(format!(
                "Preset '{}' dimensions do not match its exact sheet snapshot.",
                preset.name
            ));
        }
        if preset
            .format
            .title_block
            .fields
            .values()
            .any(|field| !field.value.is_empty())
        {
            return Err(format!(
                "Preset '{}' contains authored title-block values; preset packages may retain visibility only.",
                preset.name
            ));
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct PackageDigestContract<'a> {
    schema: &'a str,
    version: u16,
    includes_builtin_frame_references: bool,
    includes_source_metadata: bool,
    builtin_frame_references: &'a [BuiltinFrameReference],
    source_metadata: &'a Option<PresetPackageSourceMetadata>,
    presets: &'a [PortableDrawingSheetPreset],
}

fn package_digest(package: &DrawingSheetPresetPackage) -> Result<String, String> {
    let contract = PackageDigestContract {
        schema: &package.schema,
        version: package.version,
        includes_builtin_frame_references: package.includes_builtin_frame_references,
        includes_source_metadata: package.includes_source_metadata,
        builtin_frame_references: &package.builtin_frame_references,
        source_metadata: &package.source_metadata,
        presets: &package.presets,
    };
    let canonical = serde_json::to_vec(&contract)
        .map_err(|error| format!("Could not canonicalize the sheet-format package: {error}"))?;
    Ok(Sha256::digest(canonical)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn scope_rank(scope: DrawingSheetPresetScope) -> u8 {
    match scope {
        DrawingSheetPresetScope::Project => 0,
        DrawingSheetPresetScope::User => 1,
        DrawingSheetPresetScope::Organization => 2,
    }
}

fn source_metadata(presets: &[PortableDrawingSheetPreset]) -> PresetPackageSourceMetadata {
    let mut scopes = presets
        .iter()
        .map(|preset| preset.source_scope)
        .collect::<Vec<_>>();
    scopes.sort_by_key(|scope| scope_rank(*scope));
    scopes.dedup();
    PresetPackageSourceMetadata {
        producer: "RSpice".to_owned(),
        content: "drawing-sheet-format-definitions".to_owned(),
        exact_dimension_unit: "micrometre".to_owned(),
        source_scopes: scopes,
    }
}

fn builtin_frame_references(presets: &[PortableDrawingSheetPreset]) -> Vec<BuiltinFrameReference> {
    let mut references = BTreeSet::new();
    for preset in presets {
        let border = match preset.format.border {
            DrawingSheetBorderTemplate::Standard => Some("standard"),
            DrawingSheetBorderTemplate::Plain => Some("plain"),
            DrawingSheetBorderTemplate::None | DrawingSheetBorderTemplate::OrganizationManaged => {
                None
            }
        };
        if let Some(identity) = border {
            references.insert(BuiltinFrameReference {
                kind: "border".to_owned(),
                identity: identity.to_owned(),
            });
        }
        let title_block = match preset.format.title_block.template {
            DrawingSheetTitleBlockTemplate::Compact => Some("compact"),
            DrawingSheetTitleBlockTemplate::Standard => Some("standard"),
            DrawingSheetTitleBlockTemplate::Wide => Some("wide"),
            DrawingSheetTitleBlockTemplate::None
            | DrawingSheetTitleBlockTemplate::OrganizationManaged => None,
        };
        if let Some(identity) = title_block {
            references.insert(BuiltinFrameReference {
                kind: "title-block".to_owned(),
                identity: identity.to_owned(),
            });
        }
    }
    references.into_iter().collect()
}

fn human_review_rows(presets: &[PortableDrawingSheetPreset]) -> Vec<HumanReviewPreset> {
    presets
        .iter()
        .map(|preset| {
            let mut dependencies = Vec::new();
            dependencies.push(format!("border:{}", border_identity(preset.format.border)));
            dependencies.push(format!(
                "title-block:{}",
                title_block_identity(preset.format.title_block.template)
            ));
            HumanReviewPreset {
                stable_id: preset.stable_id.clone(),
                name: preset.name.clone(),
                source_scope: preset.source_scope,
                portrait_dimensions: format!(
                    "{} \u{00d7} {} mm",
                    format_millimetres(preset.portrait_width_um),
                    format_millimetres(preset.portrait_height_um)
                ),
                portrait_width_um: preset.portrait_width_um,
                portrait_height_um: preset.portrait_height_um,
                dependencies,
            }
        })
        .collect()
}

fn validate_human_review_document(document: &HumanReviewPackageDocument) -> Result<(), String> {
    if document.document != "rspice-sheet-formats-human-review"
        || document.schema.identity != PACKAGE_SCHEMA
        || document.schema.version != PACKAGE_VERSION
        || document.schema.representation != "human-review"
        || document.schema.exact_dimension_unit != "micrometre"
    {
        return Err("Unsupported human-review sheet-format schema.".to_owned());
    }
    if document.review != human_review_rows(&document.package.presets) {
        return Err(
            "The human-review projection does not match the canonical preset definitions."
                .to_owned(),
        );
    }
    Ok(())
}

fn border_identity(template: DrawingSheetBorderTemplate) -> &'static str {
    match template {
        DrawingSheetBorderTemplate::Standard => "builtin/standard",
        DrawingSheetBorderTemplate::Plain => "builtin/plain",
        DrawingSheetBorderTemplate::None => "none",
        DrawingSheetBorderTemplate::OrganizationManaged => "organization-managed",
    }
}

fn title_block_identity(template: DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => "builtin/compact",
        DrawingSheetTitleBlockTemplate::Standard => "builtin/standard",
        DrawingSheetTitleBlockTemplate::Wide => "builtin/wide",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "organization-managed",
        DrawingSheetTitleBlockTemplate::None => "none",
    }
}

fn format_millimetres(value_um: u64) -> String {
    let whole = value_um / 1_000;
    let fraction = value_um % 1_000;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
    }
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
    fn packages_preserve_exact_snapshot_and_detect_tampering() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package = build_package([preset]).unwrap();
        let encoded = encode_package(&package).unwrap();
        let restored = parse_package(&encoded).unwrap();
        assert_eq!(restored.presets[0].portrait_width_um, 250_001);
        assert_eq!(restored.presets[0].portrait_height_um, 400_002);

        let tampered = encoded.replace("250001", "250002");
        assert!(parse_package(&tampered).is_err());
    }

    #[test]
    fn package_options_are_retained_and_covered_by_the_digest() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_000, 400_000, StartingFrame::IsoA).unwrap(),
        };
        let complete = build_package_with_options([preset.clone()], true, true).unwrap();
        assert!(complete.includes_builtin_frame_references);
        assert!(complete.includes_source_metadata);
        assert!(!complete.builtin_frame_references.is_empty());
        assert!(complete.source_metadata.is_some());

        let minimal = build_package_with_options([preset], false, false).unwrap();
        assert!(!minimal.includes_builtin_frame_references);
        assert!(!minimal.includes_source_metadata);
        assert!(minimal.builtin_frame_references.is_empty());
        assert!(minimal.source_metadata.is_none());
        assert_ne!(complete.source_digest_sha256, minimal.source_digest_sha256);
        let minimal_json = encode_package(&minimal).unwrap();
        assert!(!minimal_json.contains("\"builtin_frame_references\""));
        assert!(!minimal_json.contains("\"source_metadata\""));

        let mut reference_tamper = complete.clone();
        reference_tamper.builtin_frame_references[0].identity = "forged".to_owned();
        assert!(validate_package(&reference_tamper).is_err());
        let mut metadata_tamper = complete;
        metadata_tamper.source_metadata.as_mut().unwrap().producer = "Unknown".to_owned();
        assert!(validate_package(&metadata_tamper).is_err());

        let mut tampered = minimal;
        tampered.includes_source_metadata = true;
        assert!(validate_package(&tampered).is_err());
    }

    #[test]
    fn both_export_representations_are_deterministic_and_importable() {
        let first = DrawingSheetPreset {
            id: "z-strip".to_owned(),
            name: "Z strip".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: custom_format("Z strip", 210_000, 594_000, StartingFrame::Plain).unwrap(),
        };
        let second = DrawingSheetPreset {
            id: "a-panel".to_owned(),
            name: "A panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("A panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package =
            build_package_with_options([first.clone(), second.clone()], true, true).unwrap();
        let reordered = build_package_with_options([second, first], true, true).unwrap();
        assert_eq!(package, reordered);

        for format in [
            PresetPackageFormat::CanonicalSchema1,
            PresetPackageFormat::HumanReviewJson,
        ] {
            let encoded = encode_package_with_format(&package, format).unwrap();
            assert_eq!(
                encoded,
                encode_package_with_format(&package, format).unwrap()
            );
            assert_eq!(parse_package(&encoded).unwrap(), package);
        }
    }

    #[test]
    fn human_review_projection_is_validated_against_canonical_definitions() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package = build_package([preset]).unwrap();
        let encoded =
            encode_package_with_format(&package, PresetPackageFormat::HumanReviewJson).unwrap();
        let tampered = encoded.replace("250.001 \u{00d7} 400.002 mm", "250 \u{00d7} 400 mm");
        assert!(parse_package(&tampered).is_err());
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
