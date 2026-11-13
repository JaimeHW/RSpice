//! The drawing-sheet preset package: the portable file a project's custom
//! sheet sizes are exported to and imported from.
//!
//! A package records the SHA-256 digest of the projection built by
//! `canonical_package_contract`. Import recomputes it, so a package edited by
//! hand without its digest, or damaged in transit, is refused before any of
//! its definitions is reviewed.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::design_management::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetPreset,
    DrawingSheetPresetScope, DrawingSheetTitleBlockTemplate, MAX_DRAWING_SHEET_PROJECT_PRESETS,
    SchematicSheetFormat,
};

pub const PACKAGE_SCHEMA: &str = "rspice-sheet-formats";
pub const PACKAGE_VERSION: u16 = 1;
pub const DRAWING_SHEET_PACKAGE_MAX_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawingSheetPackageEncoding {
    #[default]
    CanonicalSchema1,
    HumanReviewJson,
}

impl DrawingSheetPackageEncoding {
    pub const fn label(self) -> &'static str {
        match self {
            Self::CanonicalSchema1 => "RSpice sheet formats \u{00b7} schema 1",
            Self::HumanReviewJson => "JSON + schema \u{00b7} human review",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetPackage {
    pub schema: String,
    pub version: u16,
    pub source_digest_sha256: String,
    pub includes_builtin_frame_references: bool,
    pub includes_source_metadata: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub builtin_frame_references: Vec<BuiltinFrameReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<PresetPackageSourceMetadata>,
    pub presets: Vec<PortableDrawingSheetPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinFrameReference {
    pub kind: String,
    pub identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresetPackageSourceMetadata {
    pub producer: String,
    pub content: String,
    pub exact_dimension_unit: String,
    pub source_scopes: Vec<DrawingSheetPresetScope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortableDrawingSheetPreset {
    pub stable_id: String,
    pub name: String,
    pub source_scope: DrawingSheetPresetScope,
    pub portrait_width_um: u64,
    pub portrait_height_um: u64,
    pub format: SchematicSheetFormat,
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

pub fn portable_from_preset(
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

pub fn build_package_with_options(
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

pub fn encode_package_with_format(
    package: &DrawingSheetPresetPackage,
    format: DrawingSheetPackageEncoding,
) -> Result<String, String> {
    validate_package(package)?;
    match format {
        DrawingSheetPackageEncoding::CanonicalSchema1 => serde_json::to_string_pretty(package),
        DrawingSheetPackageEncoding::HumanReviewJson => {
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

pub fn parse_package(source: &str) -> Result<DrawingSheetPresetPackage, String> {
    if source.len() > DRAWING_SHEET_PACKAGE_MAX_BYTES {
        return Err(format!(
            "The preset package exceeds the {} byte limit.",
            DRAWING_SHEET_PACKAGE_MAX_BYTES
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
    if package.presets.len() > MAX_DRAWING_SHEET_PROJECT_PRESETS {
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

fn canonical_package_contract(package: &DrawingSheetPresetPackage) -> Result<Vec<u8>, String> {
    let contract = PackageDigestContract {
        schema: &package.schema,
        version: package.version,
        includes_builtin_frame_references: package.includes_builtin_frame_references,
        includes_source_metadata: package.includes_source_metadata,
        builtin_frame_references: &package.builtin_frame_references,
        source_metadata: &package.source_metadata,
        presets: &package.presets,
    };
    serde_json::to_vec(&contract)
        .map_err(|error| format!("Could not canonicalize the sheet-format package: {error}"))
}

fn package_digest(package: &DrawingSheetPresetPackage) -> Result<String, String> {
    Ok(Sha256::digest(canonical_package_contract(package)?)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn scope_rank(scope: DrawingSheetPresetScope) -> u8 {
    match scope {
        DrawingSheetPresetScope::Project => 0,
        DrawingSheetPresetScope::User => 1,
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
        if preset.format.border == DrawingSheetBorderTemplate::Standard {
            references.insert(BuiltinFrameReference {
                kind: "border".to_owned(),
                identity: "standard".to_owned(),
            });
        }
        let title_block = match preset.format.title_block.template {
            DrawingSheetTitleBlockTemplate::Standard => Some("standard"),
            DrawingSheetTitleBlockTemplate::AsmeY141 => Some("asme-y14-1"),
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

fn border_identity(border: DrawingSheetBorderTemplate) -> &'static str {
    match border {
        DrawingSheetBorderTemplate::Standard => "builtin/standard",
        DrawingSheetBorderTemplate::None => "none",
    }
}

fn title_block_identity(template: DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Standard => "builtin/standard",
        DrawingSheetTitleBlockTemplate::AsmeY141 => "builtin/asme-y14-1",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "organization-managed",
        DrawingSheetTitleBlockTemplate::None => "none",
    }
}

fn human_review_rows(presets: &[PortableDrawingSheetPreset]) -> Vec<HumanReviewPreset> {
    presets
        .iter()
        .map(|preset| {
            let dependencies = vec![
                format!("border:{}", border_identity(preset.format.border)),
                format!(
                    "title-block:{}",
                    title_block_identity(preset.format.title_block.template)
                ),
            ];
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

#[cfg(test)]
mod tests {
    use crate::sheet_authoring::{StartingFrame, custom_format};

    use super::*;

    fn build_package(
        presets: impl IntoIterator<Item = DrawingSheetPreset>,
    ) -> Result<DrawingSheetPresetPackage, String> {
        build_package_with_options(presets, true, true)
    }

    fn encode_package(package: &DrawingSheetPresetPackage) -> Result<String, String> {
        encode_package_with_format(package, DrawingSheetPackageEncoding::CanonicalSchema1)
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
    fn package_schema_downgrades_and_unknown_fields_are_rejected() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_000, 400_000, StartingFrame::IsoA).unwrap(),
        };
        let package = build_package([preset]).unwrap();
        let mut downgraded = package.clone();
        downgraded.version = 0;
        downgraded.source_digest_sha256 = package_digest(&downgraded).unwrap();
        assert!(parse_package(&serde_json::to_string(&downgraded).unwrap()).is_err());

        let mut value = serde_json::to_value(package).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("unknown_override".to_owned(), serde_json::Value::Bool(true));
        assert!(parse_package(&serde_json::to_string(&value).unwrap()).is_err());
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
            format: custom_format("Z strip", 210_000, 594_000, StartingFrame::None).unwrap(),
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
            DrawingSheetPackageEncoding::CanonicalSchema1,
            DrawingSheetPackageEncoding::HumanReviewJson,
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
            encode_package_with_format(&package, DrawingSheetPackageEncoding::HumanReviewJson)
                .unwrap();
        let tampered = encoded.replace("250.001 \u{00d7} 400.002 mm", "250 \u{00d7} 400 mm");
        assert!(parse_package(&tampered).is_err());
    }
}
