//! Versioned plot-export preset contracts.
//!
//! This module owns configuration and persistence only. Rendering and file
//! writing deliberately remain outside this domain boundary.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, RevisionError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct PlotExportPresetId(Uuid);

impl PlotExportPresetId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, PlotExportPresetError> {
        (!value.is_nil())
            .then_some(Self(value))
            .ok_or(PlotExportPresetError::NilIdentity)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for PlotExportPresetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PlotExportPresetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for PlotExportPresetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from_uuid(Uuid::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamingToken {
    Document,
    Revision,
    Page,
    Preset,
    Format,
}

impl NamingToken {
    const fn parse(value: &str) -> Option<Self> {
        match value.as_bytes() {
            b"document" => Some(Self::Document),
            b"revision" => Some(Self::Revision),
            b"page" => Some(Self::Page),
            b"preset" => Some(Self::Preset),
            b"format" => Some(Self::Format),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicNamingTemplate(String);

impl DeterministicNamingTemplate {
    pub fn new(value: impl Into<String>) -> Result<Self, PlotExportPresetError> {
        let value = value.into();
        validate_naming_template(&value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for DeterministicNamingTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for DeterministicNamingTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

fn validate_naming_template(value: &str) -> Result<(), PlotExportPresetError> {
    if value.is_empty() || value.len() > 128 {
        return Err(PlotExportPresetError::InvalidValue {
            field: "preset.naming-template",
            message: "naming template must contain between 1 and 128 bytes".to_owned(),
        });
    }
    let bytes = value.as_bytes();
    let mut tokens = HashSet::new();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] == b'{' {
            let remainder = &value[cursor + 1..];
            let end = remainder
                .find('}')
                .ok_or_else(|| PlotExportPresetError::InvalidValue {
                    field: "preset.naming-template",
                    message: "naming template contains an unterminated token".to_owned(),
                })?;
            let token_name = &remainder[..end];
            let token = NamingToken::parse(token_name).ok_or_else(|| {
                PlotExportPresetError::InvalidValue {
                    field: "preset.naming-template",
                    message: format!("unsupported deterministic naming token '{{{token_name}}}'"),
                }
            })?;
            tokens.insert(token);
            cursor += end + 2;
        } else {
            let byte = bytes[cursor];
            if byte == b'}'
                || (!byte.is_ascii_alphanumeric() && byte != b'-' && byte != b'_' && byte != b'.')
            {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "preset.naming-template",
                    message:
                        "literal naming characters must be ASCII letters, digits, '.', '-', or '_'"
                            .to_owned(),
                });
            }
            cursor += 1;
        }
    }
    if value.contains("..") {
        return Err(PlotExportPresetError::InvalidValue {
            field: "preset.naming-template",
            message: "naming template must not contain a parent-path sequence".to_owned(),
        });
    }
    if !tokens.contains(&NamingToken::Document) || !tokens.contains(&NamingToken::Format) {
        return Err(PlotExportPresetError::InvalidValue {
            field: "preset.naming-template",
            message: "naming template must include {document} and {format}".to_owned(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdfAConformance {
    PdfA1b,
    PdfA2b,
    PdfA3b,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "format")]
pub enum PlotExportFormat {
    PdfA { conformance: PdfAConformance },
    Svg,
    RasterPng { scale_percent: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlotExportFormatKind {
    PdfA,
    Svg,
    RasterPng,
}

impl PlotExportFormat {
    const fn kind(self) -> PlotExportFormatKind {
        match self {
            Self::PdfA { .. } => PlotExportFormatKind::PdfA,
            Self::Svg => PlotExportFormatKind::Svg,
            Self::RasterPng { .. } => PlotExportFormatKind::RasterPng,
        }
    }

    fn validate(self) -> Result<(), PlotExportPresetError> {
        if let Self::RasterPng { scale_percent } = self
            && !(25..=800).contains(&scale_percent)
        {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.formats.png.scale-percent",
                message: "PNG scale must be between 25% and 800%".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "size")]
pub enum ExportPageSize {
    AutoCrop,
    A4,
    A3,
    UsLetter,
    UsLedger,
    Custom {
        width_micrometers: u32,
        height_micrometers: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageMargins {
    pub top_micrometers: u32,
    pub right_micrometers: u32,
    pub bottom_micrometers: u32,
    pub left_micrometers: u32,
}

impl PageMargins {
    pub const NONE: Self = Self {
        top_micrometers: 0,
        right_micrometers: 0,
        bottom_micrometers: 0,
        left_micrometers: 0,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageGeometry {
    pub size: ExportPageSize,
    pub orientation: PageOrientation,
    pub margins: PageMargins,
}

impl PageGeometry {
    fn dimensions_micrometers(self) -> Option<(u32, u32)> {
        let dimensions = match self.size {
            ExportPageSize::AutoCrop => return None,
            ExportPageSize::A4 => (210_000, 297_000),
            ExportPageSize::A3 => (297_000, 420_000),
            ExportPageSize::UsLetter => (215_900, 279_400),
            ExportPageSize::UsLedger => (279_400, 431_800),
            ExportPageSize::Custom {
                width_micrometers,
                height_micrometers,
            } => (width_micrometers, height_micrometers),
        };
        Some(match self.orientation {
            PageOrientation::Portrait => dimensions,
            PageOrientation::Landscape => (dimensions.1, dimensions.0),
        })
    }

    fn validate(self) -> Result<(), PlotExportPresetError> {
        if let ExportPageSize::Custom {
            width_micrometers,
            height_micrometers,
        } = self.size
            && (!(10_000..=5_000_000).contains(&width_micrometers)
                || !(10_000..=5_000_000).contains(&height_micrometers))
        {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.page.size",
                message: "custom page dimensions must be between 10 mm and 5 m".to_owned(),
            });
        }
        if let Some((width, height)) = self.dimensions_micrometers() {
            let horizontal = self
                .margins
                .left_micrometers
                .checked_add(self.margins.right_micrometers);
            let vertical = self
                .margins
                .top_micrometers
                .checked_add(self.margins.bottom_micrometers);
            if horizontal.is_none_or(|margin| margin >= width)
                || vertical.is_none_or(|margin| margin >= height)
            {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "preset.page.margins",
                    message: "opposing margins must leave a positive page content area".to_owned(),
                });
            }
        } else if [
            self.margins.top_micrometers,
            self.margins.right_micrometers,
            self.margins.bottom_micrometers,
            self.margins.left_micrometers,
        ]
        .into_iter()
        .any(|margin| margin > 100_000)
        {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.page.margins",
                message: "auto-crop margins must not exceed 100 mm per edge".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FontEmbeddingPolicy {
    EmbedSubset,
    EmbedFull,
    ConvertToOutlines,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontPolicy {
    pub primary_family: String,
    pub fallback_families: Vec<String>,
    pub embedding: FontEmbeddingPolicy,
}

impl FontPolicy {
    fn validate(&self) -> Result<(), PlotExportPresetError> {
        validate_human_label("preset.fonts.primary-family", &self.primary_family, 96)?;
        if self.fallback_families.len() > 8 {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.fonts.fallback-families",
                message: "at most eight fallback font families may be configured".to_owned(),
            });
        }
        let mut families = HashSet::new();
        families.insert(self.primary_family.trim().to_ascii_lowercase());
        for family in &self.fallback_families {
            validate_human_label("preset.fonts.fallback-family", family, 96)?;
            if !families.insert(family.trim().to_ascii_lowercase()) {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "preset.fonts.fallback-families",
                    message: "font family names must be unique ignoring case".to_owned(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "profile")]
pub enum ColorProfile {
    Srgb,
    DisplayP3,
    CustomIcc {
        name: String,
        content_digest: ContentDigest,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "background")]
pub enum ExportBackground {
    ApplicationTheme,
    White,
    Transparent,
    SolidRgba {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
}

impl ExportBackground {
    const fn has_transparency(self) -> bool {
        matches!(
            self,
            Self::Transparent | Self::SolidRgba { alpha: 0..=254, .. }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VectorHandling {
    PreserveNative,
    OutlineUnsupported,
    RasterizeUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RasterResampling {
    Nearest,
    Bilinear,
    Bicubic,
    Lanczos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorRasterPolicy {
    pub vector_handling: VectorHandling,
    pub raster_dpi: u16,
    pub raster_resampling: RasterResampling,
    pub antialias: bool,
}

impl VectorRasterPolicy {
    fn validate(self) -> Result<(), PlotExportPresetError> {
        if !(72..=2400).contains(&self.raster_dpi) {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.rendering.raster-dpi",
                message: "raster DPI must be between 72 and 2400".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataProvenancePolicy {
    pub include_document_metadata: bool,
    pub include_dataset_manifest: bool,
    pub include_source_digests: bool,
    pub include_revision_receipts: bool,
    pub include_export_timestamp: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlotExportPresetScope {
    Personal,
    Project,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlotExportPresetDefinition {
    pub name: String,
    pub formats: Vec<PlotExportFormat>,
    pub page: PageGeometry,
    pub fonts: FontPolicy,
    pub color_profile: ColorProfile,
    pub background: ExportBackground,
    pub rendering: VectorRasterPolicy,
    pub metadata: MetadataProvenancePolicy,
    pub naming_template: DeterministicNamingTemplate,
    pub scope: PlotExportPresetScope,
}

#[derive(Deserialize)]
struct PlotExportPresetDefinitionWire {
    name: String,
    formats: Vec<PlotExportFormat>,
    page: PageGeometry,
    fonts: FontPolicy,
    color_profile: ColorProfile,
    background: ExportBackground,
    rendering: VectorRasterPolicy,
    metadata: MetadataProvenancePolicy,
    naming_template: DeterministicNamingTemplate,
    scope: PlotExportPresetScope,
}

impl<'de> Deserialize<'de> for PlotExportPresetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = PlotExportPresetDefinitionWire::deserialize(deserializer)?;
        let definition = Self {
            name: wire.name,
            formats: wire.formats,
            page: wire.page,
            fonts: wire.fonts,
            color_profile: wire.color_profile,
            background: wire.background,
            rendering: wire.rendering,
            metadata: wire.metadata,
            naming_template: wire.naming_template,
            scope: wire.scope,
        };
        definition.validate().map_err(serde::de::Error::custom)?;
        Ok(definition)
    }
}

impl PlotExportPresetDefinition {
    pub fn validate(&self) -> Result<(), PlotExportPresetError> {
        validate_human_label("preset.name", &self.name, 96)?;
        if self.formats.is_empty() {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.formats",
                message: "at least one export format is required".to_owned(),
            });
        }
        let mut kinds = HashSet::new();
        for format in &self.formats {
            format.validate()?;
            if !kinds.insert(format.kind()) {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "preset.formats",
                    message: "each export format may appear only once".to_owned(),
                });
            }
        }
        self.page.validate()?;
        self.fonts.validate()?;
        self.rendering.validate()?;
        if let ColorProfile::CustomIcc { name, .. } = &self.color_profile {
            validate_human_label("preset.color-profile.name", name, 96)?;
        }
        if self.background.has_transparency()
            && self.formats.iter().any(|format| {
                matches!(
                    format,
                    PlotExportFormat::PdfA {
                        conformance: PdfAConformance::PdfA1b
                    }
                )
            })
        {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.background",
                message: "PDF/A-1b does not permit transparent output backgrounds".to_owned(),
            });
        }
        Ok(())
    }
}

fn validate_human_label(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), PlotExportPresetError> {
    if value.trim().is_empty()
        || value != value.trim()
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(PlotExportPresetError::InvalidValue {
            field,
            message: format!(
                "value must be trimmed, non-blank, contain no control characters, and not exceed {maximum_bytes} bytes"
            ),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotExportPresetRevision {
    pub preset_id: PlotExportPresetId,
    pub revision: ObjectRevision,
    pub definition: PlotExportPresetDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PlotExportPresetHistory {
    preset_id: PlotExportPresetId,
    revisions: Vec<PlotExportPresetRevision>,
    tombstone: Option<PlotExportPresetTombstone>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotExportPresetTombstone {
    pub preset_id: PlotExportPresetId,
    pub last_active_revision: ObjectRevision,
    pub deleted_preset_revision: ObjectRevision,
    pub deleted_at_catalog_revision: ObjectRevision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlotExportPresetMutationKind {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotExportPresetMutationReceipt {
    pub receipt_id: Uuid,
    pub kind: PlotExportPresetMutationKind,
    pub preset_id: PlotExportPresetId,
    pub previous_preset_revision: Option<ObjectRevision>,
    pub committed_preset_revision: ObjectRevision,
    pub previous_catalog_revision: ObjectRevision,
    pub committed_catalog_revision: ObjectRevision,
    pub timestamp_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlotExportPresetCatalog {
    schema_version: u16,
    revision: ObjectRevision,
    histories: Vec<PlotExportPresetHistory>,
    receipts: Vec<PlotExportPresetMutationReceipt>,
}

#[derive(Deserialize)]
struct PlotExportPresetCatalogWire {
    schema_version: u16,
    revision: ObjectRevision,
    histories: Vec<PlotExportPresetHistory>,
    receipts: Vec<PlotExportPresetMutationReceipt>,
}

impl<'de> Deserialize<'de> for PlotExportPresetCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = PlotExportPresetCatalogWire::deserialize(deserializer)?;
        let catalog = Self {
            schema_version: wire.schema_version,
            revision: wire.revision,
            histories: wire.histories,
            receipts: wire.receipts,
        };
        catalog.validate().map_err(serde::de::Error::custom)?;
        Ok(catalog)
    }
}

impl Default for PlotExportPresetCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl PlotExportPresetCatalog {
    pub const SCHEMA_VERSION: u16 = 1;

    #[must_use]
    pub const fn new() -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            revision: ObjectRevision::INITIAL,
            histories: Vec::new(),
            receipts: Vec::new(),
        }
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn active_presets(&self) -> Vec<&PlotExportPresetRevision> {
        self.histories
            .iter()
            .filter(|history| history.tombstone.is_none())
            .filter_map(|history| history.revisions.last())
            .collect()
    }

    #[must_use]
    pub fn active(&self, preset_id: PlotExportPresetId) -> Option<&PlotExportPresetRevision> {
        self.histories
            .iter()
            .find(|history| history.preset_id == preset_id && history.tombstone.is_none())
            .and_then(|history| history.revisions.last())
    }

    #[must_use]
    pub fn historical_revision(
        &self,
        preset_id: PlotExportPresetId,
        revision: ObjectRevision,
    ) -> Option<&PlotExportPresetRevision> {
        self.histories
            .iter()
            .find(|history| history.preset_id == preset_id)
            .and_then(|history| {
                history
                    .revisions
                    .iter()
                    .find(|stored| stored.revision == revision)
            })
    }

    #[must_use]
    pub fn tombstone(&self, preset_id: PlotExportPresetId) -> Option<&PlotExportPresetTombstone> {
        self.histories
            .iter()
            .find(|history| history.preset_id == preset_id)
            .and_then(|history| history.tombstone.as_ref())
    }

    #[must_use]
    pub fn receipts(&self) -> &[PlotExportPresetMutationReceipt] {
        &self.receipts
    }

    /// Verify that every immutable revision in this catalog belongs to the
    /// authority that owns the catalog. Deleted histories are included: a
    /// tombstone must never become a path for moving personal or organization
    /// policy into a project document (or vice versa).
    pub fn validate_ownership_scope(
        &self,
        expected: PlotExportPresetScope,
    ) -> Result<(), PlotExportPresetError> {
        for history in &self.histories {
            for revision in &history.revisions {
                if revision.definition.scope != expected {
                    return Err(PlotExportPresetError::InvalidValue {
                        field: "catalog.ownership-scope",
                        message: format!(
                            "preset {} revision {} declares {:?}, but this catalog is owned by {:?}",
                            history.preset_id,
                            revision.revision.get(),
                            revision.definition.scope,
                            expected
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn create(
        &mut self,
        expected_catalog_revision: ObjectRevision,
        definition: PlotExportPresetDefinition,
        timestamp_unix_ms: u64,
    ) -> Result<PlotExportPresetMutationReceipt, PlotExportPresetError> {
        self.require_catalog_revision(expected_catalog_revision)?;
        definition.validate()?;
        self.require_unique_active_name(&definition.name, None)?;
        let committed_catalog_revision = self.revision.next()?;
        let preset_id = PlotExportPresetId::new();
        let mut candidate = self.clone();
        candidate.histories.push(PlotExportPresetHistory {
            preset_id,
            revisions: vec![PlotExportPresetRevision {
                preset_id,
                revision: ObjectRevision::INITIAL,
                definition,
            }],
            tombstone: None,
        });
        let receipt = PlotExportPresetMutationReceipt {
            receipt_id: Uuid::new_v4(),
            kind: PlotExportPresetMutationKind::Created,
            preset_id,
            previous_preset_revision: None,
            committed_preset_revision: ObjectRevision::INITIAL,
            previous_catalog_revision: self.revision,
            committed_catalog_revision,
            timestamp_unix_ms,
        };
        candidate.revision = committed_catalog_revision;
        candidate.receipts.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    /// Create through an owning authority. The complete retained history is
    /// checked before the mutation and the new definition must declare the
    /// same owner, so the operation either preserves ownership end-to-end or
    /// leaves the catalog byte-for-byte unchanged.
    pub fn create_owned(
        &mut self,
        expected_catalog_revision: ObjectRevision,
        expected_scope: PlotExportPresetScope,
        definition: PlotExportPresetDefinition,
        timestamp_unix_ms: u64,
    ) -> Result<PlotExportPresetMutationReceipt, PlotExportPresetError> {
        self.validate_ownership_scope(expected_scope)?;
        if definition.scope != expected_scope {
            return Err(PlotExportPresetError::InvalidValue {
                field: "preset.scope",
                message: format!(
                    "preset declares {:?}, but the owning catalog requires {:?}",
                    definition.scope, expected_scope
                ),
            });
        }
        self.create(expected_catalog_revision, definition, timestamp_unix_ms)
    }

    pub fn update(
        &mut self,
        expected_catalog_revision: ObjectRevision,
        preset_id: PlotExportPresetId,
        expected_preset_revision: ObjectRevision,
        definition: PlotExportPresetDefinition,
        timestamp_unix_ms: u64,
    ) -> Result<PlotExportPresetMutationReceipt, PlotExportPresetError> {
        self.require_catalog_revision(expected_catalog_revision)?;
        definition.validate()?;
        let current = self.require_active(preset_id)?;
        self.require_unique_active_name(&definition.name, Some(preset_id))?;
        if current.revision != expected_preset_revision {
            return Err(PlotExportPresetError::PresetRevisionConflict {
                preset_id,
                expected: expected_preset_revision,
                actual: current.revision,
            });
        }
        if current.definition == definition {
            return Err(PlotExportPresetError::NoChanges(preset_id));
        }
        let committed_preset_revision = current.revision.next()?;
        let committed_catalog_revision = self.revision.next()?;
        let mut candidate = self.clone();
        let history = candidate
            .histories
            .iter_mut()
            .find(|history| history.preset_id == preset_id)
            .ok_or(PlotExportPresetError::PresetNotFound(preset_id))?;
        history.revisions.push(PlotExportPresetRevision {
            preset_id,
            revision: committed_preset_revision,
            definition,
        });
        let receipt = PlotExportPresetMutationReceipt {
            receipt_id: Uuid::new_v4(),
            kind: PlotExportPresetMutationKind::Updated,
            preset_id,
            previous_preset_revision: Some(current.revision),
            committed_preset_revision,
            previous_catalog_revision: self.revision,
            committed_catalog_revision,
            timestamp_unix_ms,
        };
        candidate.revision = committed_catalog_revision;
        candidate.receipts.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    pub fn delete(
        &mut self,
        expected_catalog_revision: ObjectRevision,
        preset_id: PlotExportPresetId,
        expected_preset_revision: ObjectRevision,
        timestamp_unix_ms: u64,
    ) -> Result<PlotExportPresetMutationReceipt, PlotExportPresetError> {
        self.require_catalog_revision(expected_catalog_revision)?;
        let current = self.require_active(preset_id)?;
        if current.revision != expected_preset_revision {
            return Err(PlotExportPresetError::PresetRevisionConflict {
                preset_id,
                expected: expected_preset_revision,
                actual: current.revision,
            });
        }
        let deleted_preset_revision = current.revision.next()?;
        let committed_catalog_revision = self.revision.next()?;
        let mut candidate = self.clone();
        let history = candidate
            .histories
            .iter_mut()
            .find(|history| history.preset_id == preset_id)
            .ok_or(PlotExportPresetError::PresetNotFound(preset_id))?;
        history.tombstone = Some(PlotExportPresetTombstone {
            preset_id,
            last_active_revision: current.revision,
            deleted_preset_revision,
            deleted_at_catalog_revision: committed_catalog_revision,
        });
        let receipt = PlotExportPresetMutationReceipt {
            receipt_id: Uuid::new_v4(),
            kind: PlotExportPresetMutationKind::Deleted,
            preset_id,
            previous_preset_revision: Some(current.revision),
            committed_preset_revision: deleted_preset_revision,
            previous_catalog_revision: self.revision,
            committed_catalog_revision,
            timestamp_unix_ms,
        };
        candidate.revision = committed_catalog_revision;
        candidate.receipts.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    fn require_catalog_revision(
        &self,
        expected: ObjectRevision,
    ) -> Result<(), PlotExportPresetError> {
        if expected != self.revision {
            return Err(PlotExportPresetError::CatalogRevisionConflict {
                expected,
                actual: self.revision,
            });
        }
        Ok(())
    }

    fn require_active(
        &self,
        preset_id: PlotExportPresetId,
    ) -> Result<&PlotExportPresetRevision, PlotExportPresetError> {
        let history = self
            .histories
            .iter()
            .find(|history| history.preset_id == preset_id)
            .ok_or(PlotExportPresetError::PresetNotFound(preset_id))?;
        if history.tombstone.is_some() {
            return Err(PlotExportPresetError::PresetDeleted(preset_id));
        }
        history
            .revisions
            .last()
            .ok_or(PlotExportPresetError::PresetNotFound(preset_id))
    }

    fn require_unique_active_name(
        &self,
        name: &str,
        except: Option<PlotExportPresetId>,
    ) -> Result<(), PlotExportPresetError> {
        let canonical = name.trim().to_ascii_lowercase();
        if self.active_presets().into_iter().any(|preset| {
            Some(preset.preset_id) != except
                && preset.definition.name.trim().to_ascii_lowercase() == canonical
        }) {
            return Err(PlotExportPresetError::DuplicateActiveName(name.to_owned()));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), PlotExportPresetError> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(PlotExportPresetError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }
        let expected_receipts =
            self.revision
                .get()
                .checked_sub(1)
                .ok_or(PlotExportPresetError::InvalidValue {
                    field: "catalog.revision",
                    message: "catalog revision cannot precede the initial revision".to_owned(),
                })?;
        if usize::try_from(expected_receipts).ok() != Some(self.receipts.len()) {
            return Err(PlotExportPresetError::InvalidValue {
                field: "catalog.receipts",
                message: "catalog revision must be closed over a complete mutation receipt chain"
                    .to_owned(),
            });
        }
        let mut preset_ids = HashSet::new();
        let mut active_names = HashSet::new();
        for history in &self.histories {
            if !preset_ids.insert(history.preset_id) || history.revisions.is_empty() {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "catalog.histories",
                    message:
                        "preset histories must have unique identities and at least one revision"
                            .to_owned(),
                });
            }
            for (index, revision) in history.revisions.iter().enumerate() {
                if revision.preset_id != history.preset_id
                    || revision.revision.get() != u64::try_from(index).unwrap_or(u64::MAX) + 1
                {
                    return Err(PlotExportPresetError::InvalidValue {
                        field: "catalog.histories.revisions",
                        message:
                            "preset revisions must preserve identity and be contiguous from one"
                                .to_owned(),
                    });
                }
                revision.definition.validate()?;
            }
            let latest = history
                .revisions
                .last()
                .ok_or(PlotExportPresetError::PresetNotFound(history.preset_id))?;
            if let Some(tombstone) = &history.tombstone {
                if tombstone.preset_id != history.preset_id
                    || tombstone.last_active_revision != latest.revision
                    || tombstone.deleted_preset_revision != latest.revision.next()?
                    || tombstone.deleted_at_catalog_revision > self.revision
                {
                    return Err(PlotExportPresetError::InvalidValue {
                        field: "catalog.histories.tombstone",
                        message: "preset tombstone does not close over its final active revision"
                            .to_owned(),
                    });
                }
            } else if !active_names.insert(latest.definition.name.trim().to_ascii_lowercase()) {
                return Err(PlotExportPresetError::DuplicateActiveName(
                    latest.definition.name.clone(),
                ));
            }
        }
        self.validate_receipts()
    }

    fn validate_receipts(&self) -> Result<(), PlotExportPresetError> {
        let mut receipt_ids = HashSet::new();
        for (index, receipt) in self.receipts.iter().enumerate() {
            let previous_catalog_revision = ObjectRevision::new(
                u64::try_from(index).map_err(|_| PlotExportPresetError::RevisionSpaceExhausted)?
                    + 1,
            )?;
            if receipt.receipt_id.is_nil()
                || !receipt_ids.insert(receipt.receipt_id)
                || receipt.previous_catalog_revision != previous_catalog_revision
                || receipt.committed_catalog_revision != previous_catalog_revision.next()?
            {
                return Err(PlotExportPresetError::InvalidValue {
                    field: "catalog.receipts",
                    message: "receipt identities must be unique and catalog revisions contiguous"
                        .to_owned(),
                });
            }
            let history = self
                .histories
                .iter()
                .find(|history| history.preset_id == receipt.preset_id)
                .ok_or(PlotExportPresetError::PresetNotFound(receipt.preset_id))?;
            match receipt.kind {
                PlotExportPresetMutationKind::Created => {
                    if receipt.previous_preset_revision.is_some()
                        || receipt.committed_preset_revision != ObjectRevision::INITIAL
                    {
                        return Err(invalid_receipt());
                    }
                }
                PlotExportPresetMutationKind::Updated => {
                    let previous = receipt
                        .previous_preset_revision
                        .ok_or_else(invalid_receipt)?;
                    if receipt.committed_preset_revision != previous.next()?
                        || history
                            .revisions
                            .iter()
                            .all(|revision| revision.revision != receipt.committed_preset_revision)
                    {
                        return Err(invalid_receipt());
                    }
                }
                PlotExportPresetMutationKind::Deleted => {
                    let previous = receipt
                        .previous_preset_revision
                        .ok_or_else(invalid_receipt)?;
                    if receipt.committed_preset_revision != previous.next()?
                        || history.tombstone.as_ref().is_none_or(|tombstone| {
                            tombstone.deleted_preset_revision != receipt.committed_preset_revision
                                || tombstone.deleted_at_catalog_revision
                                    != receipt.committed_catalog_revision
                        })
                    {
                        return Err(invalid_receipt());
                    }
                }
            }
        }
        for history in &self.histories {
            let related: Vec<_> = self
                .receipts
                .iter()
                .filter(|receipt| receipt.preset_id == history.preset_id)
                .collect();
            let expected_count = history.revisions.len() + usize::from(history.tombstone.is_some());
            if related.len() != expected_count
                || related
                    .first()
                    .is_none_or(|receipt| receipt.kind != PlotExportPresetMutationKind::Created)
            {
                return Err(invalid_receipt());
            }
            for (index, revision) in history.revisions.iter().enumerate().skip(1) {
                let receipt = related.get(index).ok_or_else(invalid_receipt)?;
                let previous_revision = history.revisions[index - 1].revision;
                if receipt.kind != PlotExportPresetMutationKind::Updated
                    || receipt.previous_preset_revision != Some(previous_revision)
                    || receipt.committed_preset_revision != revision.revision
                {
                    return Err(invalid_receipt());
                }
            }
            if let Some(tombstone) = &history.tombstone {
                let receipt = related.last().ok_or_else(invalid_receipt)?;
                if receipt.kind != PlotExportPresetMutationKind::Deleted
                    || receipt.previous_preset_revision != Some(tombstone.last_active_revision)
                    || receipt.committed_preset_revision != tombstone.deleted_preset_revision
                {
                    return Err(invalid_receipt());
                }
            }
        }
        Ok(())
    }
}

fn invalid_receipt() -> PlotExportPresetError {
    PlotExportPresetError::InvalidValue {
        field: "catalog.receipts",
        message: "preset receipt does not agree with immutable revision history".to_owned(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PlotExportPresetError {
    #[error("plot export preset identity must not be the nil UUID")]
    NilIdentity,
    #[error("invalid {field}: {message}")]
    InvalidValue {
        field: &'static str,
        message: String,
    },
    #[error("unsupported plot export preset catalog schema version {0}")]
    UnsupportedSchemaVersion(u16),
    #[error("catalog expected revision {expected:?}, current revision is {actual:?}")]
    CatalogRevisionConflict {
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("preset {preset_id} expected revision {expected:?}, current revision is {actual:?}")]
    PresetRevisionConflict {
        preset_id: PlotExportPresetId,
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("plot export preset {0} was not found")]
    PresetNotFound(PlotExportPresetId),
    #[error("plot export preset {0} was deleted")]
    PresetDeleted(PlotExportPresetId),
    #[error("an active plot export preset already uses name {0:?}")]
    DuplicateActiveName(String),
    #[error("plot export preset {0} update contains no changes")]
    NoChanges(PlotExportPresetId),
    #[error("plot export preset revision space is exhausted")]
    RevisionSpaceExhausted,
    #[error(transparent)]
    Revision(#[from] RevisionError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(name: &str) -> PlotExportPresetDefinition {
        PlotExportPresetDefinition {
            name: name.to_owned(),
            formats: vec![
                PlotExportFormat::PdfA {
                    conformance: PdfAConformance::PdfA2b,
                },
                PlotExportFormat::Svg,
                PlotExportFormat::RasterPng { scale_percent: 200 },
            ],
            page: PageGeometry {
                size: ExportPageSize::A4,
                orientation: PageOrientation::Landscape,
                margins: PageMargins {
                    top_micrometers: 10_000,
                    right_micrometers: 12_000,
                    bottom_micrometers: 10_000,
                    left_micrometers: 12_000,
                },
            },
            fonts: FontPolicy {
                primary_family: "Inter".to_owned(),
                fallback_families: vec!["Noto Sans".to_owned(), "DejaVu Sans".to_owned()],
                embedding: FontEmbeddingPolicy::EmbedSubset,
            },
            color_profile: ColorProfile::CustomIcc {
                name: "Qualified RGB".to_owned(),
                content_digest: ContentDigest::from_bytes([7; 32]),
            },
            background: ExportBackground::White,
            rendering: VectorRasterPolicy {
                vector_handling: VectorHandling::PreserveNative,
                raster_dpi: 600,
                raster_resampling: RasterResampling::Lanczos,
                antialias: true,
            },
            metadata: MetadataProvenancePolicy {
                include_document_metadata: true,
                include_dataset_manifest: true,
                include_source_digests: true,
                include_revision_receipts: true,
                include_export_timestamp: false,
            },
            naming_template: DeterministicNamingTemplate::new(
                "{document}_r{revision}_{page}_{format}",
            )
            .unwrap(),
            scope: PlotExportPresetScope::Project,
        }
    }

    #[test]
    fn deterministic_naming_template_is_path_safe_and_requires_collision_tokens() {
        let valid = DeterministicNamingTemplate::new("{document}_{preset}_{format}").unwrap();
        assert_eq!(valid.as_str(), "{document}_{preset}_{format}");
        for invalid in [
            "{document}",
            "{format}",
            "{document}_{timestamp}_{format}",
            "../{document}_{format}",
            "{document}/{format}",
            "{document}_{format",
            "{document}_{format}}",
        ] {
            assert!(
                DeterministicNamingTemplate::new(invalid).is_err(),
                "{invalid}"
            );
        }
    }

    #[test]
    fn complete_definition_validates_and_round_trips_without_precision_loss() {
        let definition = definition("Publication package");
        definition.validate().unwrap();
        let encoded = serde_json::to_string(&definition).unwrap();
        let decoded: PlotExportPresetDefinition = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, definition);
    }

    #[test]
    fn definition_validation_rejects_ambiguous_or_unrenderable_policies() {
        let mut candidate = definition("Invalid");
        candidate.formats.push(PlotExportFormat::Svg);
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.formats",
                ..
            })
        ));

        let mut candidate = definition("Invalid");
        candidate.formats = vec![PlotExportFormat::RasterPng { scale_percent: 801 }];
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.formats.png.scale-percent",
                ..
            })
        ));

        let mut candidate = definition("Invalid");
        candidate.page.margins.left_micrometers = 200_000;
        candidate.page.margins.right_micrometers = 200_000;
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.page.margins",
                ..
            })
        ));

        let mut candidate = definition("Invalid");
        candidate.fonts.fallback_families.push("inter".to_owned());
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.fonts.fallback-families",
                ..
            })
        ));

        let mut candidate = definition("Invalid");
        candidate.rendering.raster_dpi = 71;
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.rendering.raster-dpi",
                ..
            })
        ));

        let mut candidate = definition("Invalid");
        candidate.formats = vec![PlotExportFormat::PdfA {
            conformance: PdfAConformance::PdfA1b,
        }];
        candidate.background = ExportBackground::Transparent;
        assert!(matches!(
            candidate.validate(),
            Err(PlotExportPresetError::InvalidValue {
                field: "preset.background",
                ..
            })
        ));
    }

    #[test]
    fn create_update_delete_preserve_immutable_history_and_receipts() {
        let mut catalog = PlotExportPresetCatalog::new();
        let created = catalog
            .create(catalog.revision(), definition("Engineering"), 1_000)
            .unwrap();
        let preset_id = created.preset_id;
        assert_eq!(created.kind, PlotExportPresetMutationKind::Created);
        assert_eq!(created.committed_preset_revision, ObjectRevision::INITIAL);
        let original = catalog.active(preset_id).unwrap().clone();

        let mut updated_definition = definition("Release verification");
        updated_definition.scope = PlotExportPresetScope::Organization;
        let updated = catalog
            .update(
                catalog.revision(),
                preset_id,
                ObjectRevision::INITIAL,
                updated_definition.clone(),
                2_000,
            )
            .unwrap();
        assert_eq!(updated.kind, PlotExportPresetMutationKind::Updated);
        assert_eq!(
            updated.previous_preset_revision,
            Some(ObjectRevision::INITIAL)
        );
        assert_eq!(
            catalog.active(preset_id).unwrap().definition,
            updated_definition
        );
        assert_eq!(
            catalog
                .historical_revision(preset_id, ObjectRevision::INITIAL)
                .unwrap(),
            &original
        );

        let deleted = catalog
            .delete(
                catalog.revision(),
                preset_id,
                updated.committed_preset_revision,
                3_000,
            )
            .unwrap();
        assert_eq!(deleted.kind, PlotExportPresetMutationKind::Deleted);
        assert!(catalog.active(preset_id).is_none());
        let tombstone = catalog.tombstone(preset_id).unwrap();
        assert_eq!(
            tombstone.last_active_revision,
            updated.committed_preset_revision
        );
        assert_eq!(
            tombstone.deleted_preset_revision,
            deleted.committed_preset_revision
        );
        assert_eq!(catalog.receipts(), &[created, updated, deleted]);

        let encoded = serde_json::to_string(&catalog).unwrap();
        let decoded: PlotExportPresetCatalog = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, catalog);
    }

    #[test]
    fn catalog_ownership_validation_includes_immutable_history() {
        let mut project_catalog = PlotExportPresetCatalog::new();
        let mut wrong_owner = definition("Wrong owner");
        wrong_owner.scope = PlotExportPresetScope::Personal;
        let before = project_catalog.clone();
        assert!(
            project_catalog
                .create_owned(
                    project_catalog.revision(),
                    PlotExportPresetScope::Project,
                    wrong_owner,
                    500,
                )
                .is_err()
        );
        assert_eq!(project_catalog, before);

        let mut catalog = PlotExportPresetCatalog::new();
        let mut personal = definition("Personal publication");
        personal.scope = PlotExportPresetScope::Personal;
        let created = catalog.create(catalog.revision(), personal, 1_000).unwrap();
        catalog
            .validate_ownership_scope(PlotExportPresetScope::Personal)
            .unwrap();
        assert!(
            catalog
                .validate_ownership_scope(PlotExportPresetScope::Project)
                .is_err()
        );

        catalog
            .delete(
                catalog.revision(),
                created.preset_id,
                created.committed_preset_revision,
                2_000,
            )
            .unwrap();
        assert!(
            catalog
                .validate_ownership_scope(PlotExportPresetScope::Project)
                .is_err(),
            "deleting a wrongly owned preset must not erase its ownership history"
        );
    }

    #[test]
    fn conflicts_duplicates_and_noop_updates_are_atomic() {
        let mut catalog = PlotExportPresetCatalog::new();
        let first = catalog
            .create(catalog.revision(), definition("Datasheet"), 1)
            .unwrap();
        let before = catalog.clone();
        assert!(matches!(
            catalog.create(ObjectRevision::INITIAL, definition("Other"), 2),
            Err(PlotExportPresetError::CatalogRevisionConflict { .. })
        ));
        assert_eq!(catalog, before);
        assert!(matches!(
            catalog.create(catalog.revision(), definition("DATASHEET"), 2),
            Err(PlotExportPresetError::DuplicateActiveName(_))
        ));
        assert_eq!(catalog, before);
        assert!(matches!(
            catalog.update(
                catalog.revision(),
                first.preset_id,
                ObjectRevision::INITIAL,
                definition("Datasheet"),
                2,
            ),
            Err(PlotExportPresetError::NoChanges(_))
        ));
        assert_eq!(catalog, before);
        assert!(matches!(
            catalog.delete(
                catalog.revision(),
                first.preset_id,
                ObjectRevision::new(2).unwrap(),
                2,
            ),
            Err(PlotExportPresetError::PresetRevisionConflict { .. })
        ));
        assert_eq!(catalog, before);
    }

    #[test]
    fn deserialization_rejects_broken_receipt_chains_and_nil_preset_ids() {
        let mut catalog = PlotExportPresetCatalog::new();
        catalog
            .create(catalog.revision(), definition("Audit"), 1)
            .unwrap();
        let mut broken = serde_json::to_value(&catalog).unwrap();
        broken["receipts"][0]["committed_catalog_revision"] = serde_json::json!(9);
        assert!(serde_json::from_value::<PlotExportPresetCatalog>(broken).is_err());

        let nil = serde_json::json!(Uuid::nil());
        assert!(serde_json::from_value::<PlotExportPresetId>(nil).is_err());
    }
}
