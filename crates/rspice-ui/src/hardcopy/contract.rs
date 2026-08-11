//! Typed hardcopy and page-setup contracts.
//!
//! This module deliberately contains no egui, printer, filesystem, or browser
//! integration. It turns an exact document snapshot and a validated page setup
//! into an immutable render plan, deterministic preview pages, and a sealed
//! execution receipt. Physical geometry is represented in integer micrometres;
//! neither pagination nor content identities depend on host floating-point
//! behavior.

mod mapping;
mod page_setup;

pub use mapping::*;
pub use page_setup::*;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, RevisionError};

const HARDCOPY_SCHEMA_VERSION: u32 = 1;
const SETUP_STORE_SCHEMA_VERSION: u32 = 1;
const MAX_TEXT_BYTES: usize = 256;
const MAX_OUTCOME_MESSAGE_BYTES: usize = 2_048;
const MAX_PAGE_DIMENSION_UM: u64 = 5_000_000;
const MIN_PAGE_DIMENSION_UM: u64 = 10_000;
pub(crate) const MAX_PREVIEW_PAGES: u32 = 10_000;
const MAX_MANUAL_AXIS_PAGES: u16 = 512;
const HEADER_BAND_UM: u64 = 8_000;
const PROVENANCE_BAND_UM: u64 = 8_000;
const LEGEND_COLUMN_UM: u64 = 64_000;
const LEGEND_ROW_UM: u64 = 5_000;
const LEGEND_VERTICAL_PADDING_UM: u64 = 4_000;

/// Stable identity of the source document whose page setup follows it across
/// ordinary content revisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct HardcopyDocumentId(Uuid);

impl HardcopyDocumentId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, HardcopyError> {
        if value.is_nil() {
            Err(HardcopyError::NilIdentity("hardcopy document"))
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for HardcopyDocumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HardcopyDocumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Uuid::deserialize(deserializer)?;
        Self::try_from_uuid(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for HardcopyDocumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

macro_rules! immutable_uuid_identity {
    ($name:ident, $label:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn try_from_uuid(value: Uuid) -> Result<Self, HardcopyError> {
                if value.is_nil() {
                    Err(HardcopyError::NilIdentity($label))
                } else {
                    Ok(Self(value))
                }
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = Uuid::deserialize(deserializer)?;
                Self::try_from_uuid(value).map_err(serde::de::Error::custom)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

immutable_uuid_identity!(HardcopyPlanId, "hardcopy plan");
immutable_uuid_identity!(HardcopyReceiptId, "hardcopy receipt");

/// Physical length used throughout the hardcopy contract.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Length(u64);

impl Length {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn from_micrometres(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn micrometres(self) -> u64 {
        self.0
    }

    /// Parse an unsigned decimal quantity with deterministic half-up rounding
    /// to the nearest micrometre. Unit suffixes are intentionally rejected;
    /// the selected UI unit is an independent typed field.
    pub fn parse_decimal(value: &str, unit: LengthUnit) -> Result<Self, HardcopyError> {
        let value = value.trim();
        if value.is_empty() || value.starts_with('-') || value.starts_with('+') {
            return Err(HardcopyError::InvalidLength(value.to_owned()));
        }
        let mut components = value.split('.');
        let whole = components
            .next()
            .filter(|part| !part.is_empty())
            .ok_or_else(|| HardcopyError::InvalidLength(value.to_owned()))?;
        let fraction = components.next().unwrap_or_default();
        if components.next().is_some()
            || !whole.bytes().all(|byte| byte.is_ascii_digit())
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || fraction.len() > 9
        {
            return Err(HardcopyError::InvalidLength(value.to_owned()));
        }
        let denominator = 10_u128
            .checked_pow(fraction.len() as u32)
            .ok_or_else(|| HardcopyError::InvalidLength(value.to_owned()))?;
        let whole = whole
            .parse::<u128>()
            .map_err(|_| HardcopyError::InvalidLength(value.to_owned()))?;
        let fraction = if fraction.is_empty() {
            0
        } else {
            fraction
                .parse::<u128>()
                .map_err(|_| HardcopyError::InvalidLength(value.to_owned()))?
        };
        let scaled = whole
            .checked_mul(denominator)
            .and_then(|number| number.checked_add(fraction))
            .and_then(|number| number.checked_mul(unit.micrometres_per_unit() as u128))
            .ok_or_else(|| HardcopyError::InvalidLength(value.to_owned()))?;
        let rounded = scaled
            .checked_add(denominator / 2)
            .ok_or_else(|| HardcopyError::InvalidLength(value.to_owned()))?
            / denominator;
        let rounded =
            u64::try_from(rounded).map_err(|_| HardcopyError::InvalidLength(value.to_owned()))?;
        Ok(Self(rounded))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LengthUnit {
    Inches,
    Millimetres,
}

impl LengthUnit {
    const fn micrometres_per_unit(self) -> u64 {
        match self {
            Self::Inches => 25_400,
            Self::Millimetres => 1_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardcopyDocumentKind {
    SchematicOrSymbol,
    LayoutWithLayerLegend,
    PlotOrWorksheet,
    Report,
    EngineeringDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum HardcopyScope {
    Selection,
    CurrentSheet,
    VisibleHierarchy,
    ActivePlotDocument,
    CompleteReport,
    ActiveDocument,
    AllSheetsOrPanes,
    NamedPrintSet(String),
}

impl HardcopyScope {
    fn validate_for(&self, kind: HardcopyDocumentKind) -> Result<(), HardcopyError> {
        if let Self::NamedPrintSet(name) = self {
            validate_text("named print set", name, MAX_TEXT_BYTES)?;
        }
        let compatible = match self {
            Self::Selection | Self::ActiveDocument | Self::NamedPrintSet(_) => true,
            Self::CurrentSheet | Self::VisibleHierarchy => matches!(
                kind,
                HardcopyDocumentKind::SchematicOrSymbol
                    | HardcopyDocumentKind::LayoutWithLayerLegend
                    | HardcopyDocumentKind::EngineeringDocument
            ),
            Self::ActivePlotDocument => matches!(kind, HardcopyDocumentKind::PlotOrWorksheet),
            Self::CompleteReport => matches!(kind, HardcopyDocumentKind::Report),
            Self::AllSheetsOrPanes => !matches!(kind, HardcopyDocumentKind::EngineeringDocument),
        };
        if compatible {
            Ok(())
        } else {
            Err(HardcopyError::IncompatibleScope {
                kind,
                scope: self.clone(),
            })
        }
    }
}

/// Exact active-document snapshot used as the source authority for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActiveHardcopySource {
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    content_digest: ContentDigest,
    display_name: String,
    document_kind: HardcopyDocumentKind,
    scope: HardcopyScope,
}

impl ActiveHardcopySource {
    pub fn try_new(
        document_id: HardcopyDocumentId,
        revision: ObjectRevision,
        content_digest: ContentDigest,
        display_name: impl Into<String>,
        document_kind: HardcopyDocumentKind,
        scope: HardcopyScope,
    ) -> Result<Self, HardcopyError> {
        let display_name = display_name.into();
        validate_text(
            "hardcopy source display name",
            &display_name,
            MAX_TEXT_BYTES,
        )?;
        scope.validate_for(document_kind)?;
        Ok(Self {
            document_id,
            revision,
            content_digest,
            display_name,
            document_kind,
            scope,
        })
    }

    #[must_use]
    pub const fn document_id(&self) -> HardcopyDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub const fn document_kind(&self) -> HardcopyDocumentKind {
        self.document_kind
    }

    #[must_use]
    pub const fn scope(&self) -> &HardcopyScope {
        &self.scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RenderTarget {
    SystemPrinter {
        printer_id: String,
        job: PrinterJobSettings,
    },
    BrowserPrintDialog,
    ExportArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum OutputFormat {
    NativePrinter,
    BrowserPrintDocument,
    PdfVector,
    PdfA,
    SvgVector,
    Png { dpi: u16 },
    Tiff { dpi: u16 },
}

/// Absolute bounds on a raster output resolution. These are the contract's
/// own limits, independent of any target: what a given host can actually
/// afford is narrower and depends on the page, so the dialog derives its
/// ceiling from the renderer instead of from these.
pub const MIN_RASTER_DPI: u16 = 72;
/// See [`MIN_RASTER_DPI`].
pub const MAX_RASTER_DPI: u16 = 9_600;

impl OutputFormat {
    #[must_use]
    pub const fn is_vector(self) -> bool {
        matches!(
            self,
            Self::BrowserPrintDocument | Self::PdfVector | Self::PdfA | Self::SvgVector
        )
    }

    /// The resolution a raster format carries, if this is one.
    #[must_use]
    pub const fn raster_dpi(self) -> Option<u16> {
        match self {
            Self::Png { dpi } | Self::Tiff { dpi } => Some(dpi),
            _ => None,
        }
    }

    /// The same format at another resolution. Vector and printer formats have
    /// no resolution to change and are returned unaltered.
    #[must_use]
    pub const fn with_raster_dpi(self, dpi: u16) -> Self {
        match self {
            Self::Png { .. } => Self::Png { dpi },
            Self::Tiff { .. } => Self::Tiff { dpi },
            other => other,
        }
    }

    fn validate(self) -> Result<(), HardcopyError> {
        if let Self::Png { dpi } | Self::Tiff { dpi } = self
            && !(MIN_RASTER_DPI..=MAX_RASTER_DPI).contains(&dpi)
        {
            return Err(HardcopyError::InvalidRasterResolution(dpi));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackgroundMode {
    White,
    Transparent,
    WorkspaceBackground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontPolicy {
    embed_fonts: bool,
    preserve_searchable_text: bool,
}

impl FontPolicy {
    #[must_use]
    pub const fn new(embed_fonts: bool, preserve_searchable_text: bool) -> Self {
        Self {
            embed_fonts,
            preserve_searchable_text,
        }
    }

    #[must_use]
    pub const fn embed_fonts(self) -> bool {
        self.embed_fonts
    }

    #[must_use]
    pub const fn preserve_searchable_text(self) -> bool {
        self.preserve_searchable_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderSetup {
    target: RenderTarget,
    format: OutputFormat,
    color_mapping: ColorMapping,
    background: BackgroundMode,
    fonts: FontPolicy,
    soft_proof_print_safe_colors: bool,
}

impl RenderSetup {
    pub fn try_new(
        target: RenderTarget,
        format: OutputFormat,
        color_mapping: ColorMapping,
        background: BackgroundMode,
        fonts: FontPolicy,
        soft_proof_print_safe_colors: bool,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            target,
            format,
            color_mapping,
            background,
            fonts,
            soft_proof_print_safe_colors,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.format.validate()?;
        match (&self.target, self.format) {
            (RenderTarget::SystemPrinter { printer_id, job }, OutputFormat::NativePrinter) => {
                validate_text("printer identity", printer_id, MAX_TEXT_BYTES)?;
                job.validate()?;
            }
            (RenderTarget::BrowserPrintDialog, OutputFormat::BrowserPrintDocument) => {}
            (RenderTarget::ExportArtifact, OutputFormat::NativePrinter)
            | (RenderTarget::ExportArtifact, OutputFormat::BrowserPrintDocument)
            | (RenderTarget::BrowserPrintDialog, _)
            | (RenderTarget::SystemPrinter { .. }, _) => {
                return Err(HardcopyError::IncompatibleRenderTarget);
            }
            (RenderTarget::ExportArtifact, _) => {}
        }
        if self.background == BackgroundMode::Transparent
            && (!self.format.is_vector() || !matches!(self.target, RenderTarget::ExportArtifact))
        {
            return Err(HardcopyError::TransparentBackgroundRequiresVectorExport);
        }
        if self.fonts.preserve_searchable_text && !self.format.is_vector() {
            return Err(HardcopyError::SearchableTextRequiresVectorOutput);
        }
        if self.fonts.preserve_searchable_text && !self.fonts.embed_fonts {
            return Err(HardcopyError::SearchableTextRequiresEmbeddedFonts);
        }
        if self.format == OutputFormat::PdfA && !self.fonts.embed_fonts {
            return Err(HardcopyError::PdfARequiresEmbeddedFonts);
        }
        Ok(())
    }

    #[must_use]
    pub const fn target(&self) -> &RenderTarget {
        &self.target
    }

    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    #[must_use]
    pub const fn color_mapping(&self) -> ColorMapping {
        self.color_mapping
    }

    #[must_use]
    pub const fn background(&self) -> BackgroundMode {
        self.background
    }

    #[must_use]
    pub const fn fonts(&self) -> FontPolicy {
        self.fonts
    }

    #[must_use]
    pub const fn soft_proof_print_safe_colors(&self) -> bool {
        self.soft_proof_print_safe_colors
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum Watermark {
    None,
    Draft,
    Confidential,
    Custom(String),
}

impl Watermark {
    fn validate(&self) -> Result<(), HardcopyError> {
        if let Self::Custom(text) = self {
            validate_text("custom watermark", text, 128)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecorationSetup {
    include_trace_layer_net_marker_legends: bool,
    include_project_revision_sheet_date_page_header: bool,
    include_result_manifest_model_digest_run_provenance: bool,
    watermark: Watermark,
}

impl DecorationSetup {
    pub fn try_new(
        include_trace_layer_net_marker_legends: bool,
        include_project_revision_sheet_date_page_header: bool,
        include_result_manifest_model_digest_run_provenance: bool,
        watermark: Watermark,
    ) -> Result<Self, HardcopyError> {
        watermark.validate()?;
        Ok(Self {
            include_trace_layer_net_marker_legends,
            include_project_revision_sheet_date_page_header,
            include_result_manifest_model_digest_run_provenance,
            watermark,
        })
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.watermark.validate()
    }

    #[must_use]
    pub const fn includes_legends(&self) -> bool {
        self.include_trace_layer_net_marker_legends
    }

    #[must_use]
    pub const fn includes_header(&self) -> bool {
        self.include_project_revision_sheet_date_page_header
    }

    #[must_use]
    pub const fn includes_provenance(&self) -> bool {
        self.include_result_manifest_model_digest_run_provenance
    }

    #[must_use]
    pub const fn watermark(&self) -> &Watermark {
        &self.watermark
    }
}

/// Schematic-only output extent. This remains independent of the authored
/// drawing-sheet format: changing it never rewrites the schematic document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicHardcopyExtent {
    /// Use the authored drawing sheet as the intended output boundary.
    AuthoredDrawingSheet,
    /// Include the complete resolved schematic extent, including content
    /// parked outside the authored drawing sheet.
    CompleteSchematicContent,
}

/// Required decision when resolved schematic content crosses the authored
/// drawing-sheet boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutsideSheetContentPolicy {
    /// Refuse publication until the operator chooses clipping or extension.
    Ask,
    /// Clip schematic content at the physical authored drawing-sheet edge.
    ClipToAuthoredSheet,
    /// Extend the output extent to retain every resolved schematic object.
    ExtendOutput,
}

/// Independent print/export inclusion contract for an authored schematic
/// drawing sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SchematicHardcopySetup {
    extent: SchematicHardcopyExtent,
    outside_content: OutsideSheetContentPolicy,
    crop_marks: bool,
    include_paper: bool,
    include_border: bool,
    include_title_block: bool,
    include_zones: bool,
    include_grid: bool,
}

impl SchematicHardcopySetup {
    #[must_use]
    pub const fn new(
        extent: SchematicHardcopyExtent,
        outside_content: OutsideSheetContentPolicy,
        crop_marks: bool,
        include_paper: bool,
        include_border: bool,
        include_title_block: bool,
        include_zones: bool,
        include_grid: bool,
    ) -> Self {
        Self {
            extent,
            outside_content,
            crop_marks,
            include_paper,
            include_border,
            include_title_block,
            include_zones,
            include_grid,
        }
    }

    #[must_use]
    pub const fn extent(self) -> SchematicHardcopyExtent {
        self.extent
    }

    #[must_use]
    pub const fn outside_content(self) -> OutsideSheetContentPolicy {
        self.outside_content
    }

    #[must_use]
    pub const fn crop_marks(self) -> bool {
        self.crop_marks
    }

    #[must_use]
    pub const fn includes_paper(self) -> bool {
        self.include_paper
    }

    #[must_use]
    pub const fn includes_border(self) -> bool {
        self.include_border
    }

    #[must_use]
    pub const fn includes_title_block(self) -> bool {
        self.include_title_block
    }

    #[must_use]
    pub const fn includes_zones(self) -> bool {
        self.include_zones
    }

    #[must_use]
    pub const fn includes_grid(self) -> bool {
        self.include_grid
    }
}

impl Default for SchematicHardcopySetup {
    fn default() -> Self {
        Self::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            true,
            true,
            true,
            true,
            true,
            false,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopySetup {
    physical_page: PhysicalPageSetup,
    scale: ScaleMode,
    tiling: TilingSetup,
    render: RenderSetup,
    decorations: DecorationSetup,
    schematic: SchematicHardcopySetup,
    print_mapping: PrintMappingTable,
}

#[derive(Deserialize)]
struct HardcopySetupWire {
    physical_page: PhysicalPageSetup,
    scale: ScaleMode,
    tiling: TilingSetup,
    render: RenderSetup,
    decorations: DecorationSetup,
    #[serde(default)]
    schematic: SchematicHardcopySetup,
    print_mapping: PrintMappingTable,
}

impl<'de> Deserialize<'de> for HardcopySetup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HardcopySetupWire::deserialize(deserializer)?;
        Self::try_new_with_schematic(
            wire.physical_page,
            wire.scale,
            wire.tiling,
            wire.render,
            wire.decorations,
            wire.schematic,
            wire.print_mapping,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl HardcopySetup {
    pub fn try_new(
        physical_page: PhysicalPageSetup,
        scale: ScaleMode,
        tiling: TilingSetup,
        render: RenderSetup,
        decorations: DecorationSetup,
        print_mapping: PrintMappingTable,
    ) -> Result<Self, HardcopyError> {
        Self::try_new_with_schematic(
            physical_page,
            scale,
            tiling,
            render,
            decorations,
            SchematicHardcopySetup::default(),
            print_mapping,
        )
    }

    pub fn try_new_with_schematic(
        physical_page: PhysicalPageSetup,
        scale: ScaleMode,
        tiling: TilingSetup,
        render: RenderSetup,
        decorations: DecorationSetup,
        schematic: SchematicHardcopySetup,
        print_mapping: PrintMappingTable,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            physical_page,
            scale,
            tiling,
            render,
            decorations,
            schematic,
            print_mapping,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.physical_page.validate()?;
        self.scale.validate()?;
        self.tiling.validate()?;
        self.render.validate()?;
        self.decorations.validate()?;
        self.print_mapping.validate()
    }

    #[must_use]
    pub const fn physical_page(&self) -> &PhysicalPageSetup {
        &self.physical_page
    }

    #[must_use]
    pub const fn scale(&self) -> ScaleMode {
        self.scale
    }

    #[must_use]
    pub const fn tiling(&self) -> TilingSetup {
        self.tiling
    }

    #[must_use]
    pub const fn render(&self) -> &RenderSetup {
        &self.render
    }

    #[must_use]
    pub const fn decorations(&self) -> &DecorationSetup {
        &self.decorations
    }

    #[must_use]
    pub const fn schematic(&self) -> SchematicHardcopySetup {
        self.schematic
    }

    #[must_use]
    pub const fn print_mapping(&self) -> &PrintMappingTable {
        &self.print_mapping
    }
}

impl Default for HardcopySetup {
    fn default() -> Self {
        let inch = |value| Length::parse_decimal(value, LengthUnit::Inches).expect("constant");
        Self::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins {
                    top: inch("0.350"),
                    right: inch("0.350"),
                    bottom: inch("0.450"),
                    left: inch("0.350"),
                },
                Bleed::None,
                Orientation::Landscape,
            )
            .expect("default physical page"),
            ScaleMode::FitPrintableArea,
            TilingSetup::try_new(TilingMode::Automatic, inch("0.25"), true)
                .expect("default tiling"),
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::PdfVector,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            )
            .expect("default rendering"),
            DecorationSetup::try_new(true, true, true, Watermark::None)
                .expect("default decorations"),
            PrintMappingTable::default(),
        )
        .expect("default hardcopy setup")
    }
}

/// Exact physical extent of the selected source content at 1:1 scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentExtent {
    width: Length,
    height: Length,
}

/// Authenticated independently paginated portion of a semantic aggregate.
/// The origin maps local `(0, 0)` into the aggregate source coordinate space;
/// every section is paginated independently so a required break is never
/// inferred from a geometric gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardcopyContentSection {
    ordinal: u32,
    content_digest: ContentDigest,
    origin_x: Length,
    origin_y: Length,
    extent: ContentExtent,
    page_break_before: bool,
}

impl HardcopyContentSection {
    pub fn try_new(
        ordinal: u32,
        content_digest: ContentDigest,
        origin_x: Length,
        origin_y: Length,
        extent: ContentExtent,
        page_break_before: bool,
    ) -> Result<Self, HardcopyError> {
        if ordinal == 0 && page_break_before {
            return Err(HardcopyError::InvalidContentSections(
                "the first content section cannot require a preceding break",
            ));
        }
        Ok(Self {
            ordinal,
            content_digest,
            origin_x,
            origin_y,
            extent,
            page_break_before,
        })
    }

    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }

    #[must_use]
    pub const fn content_digest(self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn origin(self) -> (Length, Length) {
        (self.origin_x, self.origin_y)
    }

    #[must_use]
    pub const fn extent(self) -> ContentExtent {
        self.extent
    }

    #[must_use]
    pub const fn page_break_before(self) -> bool {
        self.page_break_before
    }
}

impl ContentExtent {
    pub fn try_new(width: Length, height: Length) -> Result<Self, HardcopyError> {
        if width == Length::ZERO || height == Length::ZERO {
            return Err(HardcopyError::EmptyContentExtent);
        }
        Ok(Self { width, height })
    }

    #[must_use]
    pub const fn width(self) -> Length {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> Length {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResolvedOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageRect {
    pub x: Length,
    pub y: Length,
    pub width: Length,
    pub height: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedPageGeometry {
    orientation: ResolvedOrientation,
    physical_width: Length,
    physical_height: Length,
    printable_rect: PageRect,
    content_rect: PageRect,
    header_band: Length,
    legend_band: Length,
    provenance_band: Length,
}

impl ValidatedPageGeometry {
    fn resolve_for_section(
        setup: &HardcopySetup,
        content: ContentExtent,
        section_ordinal: u32,
    ) -> Result<Self, HardcopyError> {
        let authored_media = setup.physical_page.paper.authored_media(section_ordinal);
        let orientation = authored_media.map_or_else(
            || match setup.physical_page.orientation {
                Orientation::Portrait => ResolvedOrientation::Portrait,
                Orientation::Landscape => ResolvedOrientation::Landscape,
                Orientation::AutomaticPerPage => {
                    if let RenderTarget::SystemPrinter { job, .. } = setup.render.target() {
                        let (width_pixels, height_pixels) =
                            job.raster_geometry().physical_size_px();
                        if width_pixels > height_pixels {
                            ResolvedOrientation::Landscape
                        } else if height_pixels > width_pixels {
                            ResolvedOrientation::Portrait
                        } else if content.width > content.height {
                            ResolvedOrientation::Landscape
                        } else {
                            ResolvedOrientation::Portrait
                        }
                    } else if content.width > content.height {
                        ResolvedOrientation::Landscape
                    } else {
                        ResolvedOrientation::Portrait
                    }
                }
            },
            |media| {
                let (width, height) = media.dimensions();
                if width > height {
                    ResolvedOrientation::Landscape
                } else {
                    ResolvedOrientation::Portrait
                }
            },
        );
        let (physical_width, physical_height) = if let Some(media) = authored_media {
            media.dimensions()
        } else {
            let (portrait_width, portrait_height) = setup.physical_page.paper.portrait_dimensions();
            match orientation {
                ResolvedOrientation::Portrait => (portrait_width, portrait_height),
                ResolvedOrientation::Landscape => (portrait_height, portrait_width),
            }
        };
        let margins = setup.physical_page.margins;
        validate_margins(physical_width, physical_height, margins)?;
        let printable_width = checked_sub(
            physical_width.0,
            margins.left.0 + margins.right.0,
            "horizontal margins",
        )?;
        let printable_height = checked_sub(
            physical_height.0,
            margins.top.0 + margins.bottom.0,
            "vertical margins",
        )?;
        let header_band = if setup.decorations.includes_header() {
            HEADER_BAND_UM
        } else {
            0
        };
        let provenance_band = if setup.decorations.includes_provenance() {
            PROVENANCE_BAND_UM
        } else {
            0
        };
        let legend_entries = if setup.decorations.includes_legends() {
            setup
                .print_mapping
                .entries()
                .iter()
                .filter(|entry| entry.include_in_legend())
                .count()
        } else {
            0
        };
        let legend_band = if legend_entries == 0 {
            0
        } else {
            let columns = printable_width / LEGEND_COLUMN_UM;
            if columns == 0 {
                return Err(HardcopyError::NoPrintableContentArea);
            }
            let entries =
                u64::try_from(legend_entries).map_err(|_| HardcopyError::ScaledGeometryOverflow)?;
            let rows = entries.div_ceil(columns);
            rows.checked_mul(LEGEND_ROW_UM)
                .and_then(|height| height.checked_add(LEGEND_VERTICAL_PADDING_UM))
                .ok_or(HardcopyError::ScaledGeometryOverflow)?
        };
        let decoration_height = header_band
            .checked_add(legend_band)
            .and_then(|height| height.checked_add(provenance_band))
            .ok_or(HardcopyError::ScaledGeometryOverflow)?;
        let content_height = checked_sub(
            printable_height,
            decoration_height,
            "header, legend, and provenance bands",
        )?;
        if printable_width == 0 || content_height == 0 {
            return Err(HardcopyError::NoPrintableContentArea);
        }
        let printable_rect = PageRect {
            x: margins.left,
            y: margins.top,
            width: Length(printable_width),
            height: Length(printable_height),
        };
        let content_rect = PageRect {
            x: margins.left,
            y: Length(margins.top.0 + header_band),
            width: Length(printable_width),
            height: Length(content_height),
        };
        Ok(Self {
            orientation,
            physical_width,
            physical_height,
            printable_rect,
            content_rect,
            header_band: Length(header_band),
            legend_band: Length(legend_band),
            provenance_band: Length(provenance_band),
        })
    }

    #[must_use]
    pub const fn orientation(self) -> ResolvedOrientation {
        self.orientation
    }

    #[must_use]
    pub const fn physical_size(self) -> (Length, Length) {
        (self.physical_width, self.physical_height)
    }

    #[must_use]
    pub const fn printable_rect(self) -> PageRect {
        self.printable_rect
    }

    #[must_use]
    pub const fn content_rect(self) -> PageRect {
        self.content_rect
    }

    #[must_use]
    pub const fn header_band(self) -> Length {
        self.header_band
    }

    #[must_use]
    pub const fn legend_band(self) -> Length {
        self.legend_band
    }

    #[must_use]
    pub const fn provenance_band(self) -> Length {
        self.provenance_band
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaleRatio {
    numerator: u64,
    denominator: u64,
}

impl ScaleRatio {
    fn new(numerator: u64, denominator: u64) -> Result<Self, HardcopyError> {
        if numerator == 0 || denominator == 0 {
            return Err(HardcopyError::ZeroScaleRatio);
        }
        let divisor = greatest_common_divisor(numerator, denominator);
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewPage {
    number: u32,
    row: u16,
    column: u16,
    coordinate: String,
    geometry: ValidatedPageGeometry,
    scaled_content_window: PageRect,
    scale: ScaleRatio,
    section_ordinal: u32,
}

impl PreviewPage {
    #[must_use]
    pub const fn number(&self) -> u32 {
        self.number
    }

    #[must_use]
    pub fn coordinate(&self) -> &str {
        &self.coordinate
    }

    #[must_use]
    pub const fn geometry(&self) -> ValidatedPageGeometry {
        self.geometry
    }

    #[must_use]
    pub const fn scaled_content_window(&self) -> PageRect {
        self.scaled_content_window
    }

    #[must_use]
    pub const fn scale(&self) -> ScaleRatio {
        self.scale
    }

    #[must_use]
    pub const fn section_ordinal(&self) -> u32 {
        self.section_ordinal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewPagination {
    geometry: ValidatedPageGeometry,
    scale: ScaleRatio,
    scaled_content: ContentExtent,
    columns: u16,
    rows: u16,
    pages: Vec<PreviewPage>,
    sections: Vec<HardcopyContentSection>,
}

impl PreviewPagination {
    fn build(setup: &HardcopySetup, content: ContentExtent) -> Result<Self, HardcopyError> {
        Self::build_for_section(setup, content, 0)
    }

    fn build_for_section(
        setup: &HardcopySetup,
        content: ContentExtent,
        section_ordinal: u32,
    ) -> Result<Self, HardcopyError> {
        let geometry = ValidatedPageGeometry::resolve_for_section(setup, content, section_ordinal)?;
        let viewport = geometry.content_rect;
        let scale = match setup.scale {
            ScaleMode::FitPrintableArea => {
                let width_ratio = ScaleRatio::new(viewport.width.0, content.width.0)?;
                let height_ratio = ScaleRatio::new(viewport.height.0, content.height.0)?;
                if ratio_less_than(width_ratio, height_ratio) {
                    width_ratio
                } else {
                    height_ratio
                }
            }
            ScaleMode::EngineeringOneToOne => ScaleRatio::new(1, 1)?,
            ScaleMode::CustomPercent { hundredths_percent } => {
                ScaleRatio::new(u64::from(hundredths_percent), 10_000)?
            }
            ScaleMode::FitWidth => ScaleRatio::new(viewport.width.0, content.width.0)?,
        };
        let scaled_content = ContentExtent::try_new(
            Length(scale_ceil(content.width.0, scale)?),
            Length(scale_ceil(content.height.0, scale)?),
        )?;
        let overlap = setup.tiling.overlap.0;
        if overlap >= viewport.width.0 || overlap >= viewport.height.0 {
            return Err(HardcopyError::OverlapExhaustsPage);
        }
        let required_columns = minimum_tiles(scaled_content.width.0, viewport.width.0, overlap)?;
        let required_rows = minimum_tiles(scaled_content.height.0, viewport.height.0, overlap)?;
        let (columns, rows) = match setup.tiling.mode {
            TilingMode::Automatic => (required_columns, required_rows),
            TilingMode::SinglePage => {
                if required_columns != 1 || required_rows != 1 {
                    return Err(HardcopyError::SinglePageOverflow {
                        required_columns,
                        required_rows,
                    });
                }
                (1, 1)
            }
            TilingMode::Manual { columns, rows } => {
                if columns != required_columns || rows != required_rows {
                    return Err(HardcopyError::ManualTilingDoesNotCover {
                        columns,
                        rows,
                        required_columns,
                        required_rows,
                    });
                }
                (columns, rows)
            }
        };
        let page_count = u32::from(columns) * u32::from(rows);
        if page_count == 0 || page_count > MAX_PREVIEW_PAGES {
            return Err(HardcopyError::TooManyPages(page_count));
        }
        let step_x = viewport.width.0 - overlap;
        let step_y = viewport.height.0 - overlap;
        let mut pages = Vec::with_capacity(page_count as usize);
        for row in 0..rows {
            for column in 0..columns {
                let x = u64::from(column) * step_x;
                let y = u64::from(row) * step_y;
                let width = viewport.width.0.min(scaled_content.width.0 - x);
                let height = viewport.height.0.min(scaled_content.height.0 - y);
                let number = u32::from(row) * u32::from(columns) + u32::from(column) + 1;
                pages.push(PreviewPage {
                    number,
                    row: row + 1,
                    column: column + 1,
                    coordinate: format!("{}{}", spreadsheet_column(column), row + 1),
                    geometry,
                    scaled_content_window: PageRect {
                        x: Length(x),
                        y: Length(y),
                        width: Length(width),
                        height: Length(height),
                    },
                    scale,
                    section_ordinal: 0,
                });
            }
        }
        Ok(Self {
            geometry,
            scale,
            scaled_content,
            columns,
            rows,
            pages,
            sections: Vec::new(),
        })
    }

    fn build_sections(
        setup: &HardcopySetup,
        aggregate_extent: ContentExtent,
        sections: &[HardcopyContentSection],
    ) -> Result<Self, HardcopyError> {
        if sections.is_empty() || sections.len() > 4_096 {
            return Err(HardcopyError::InvalidContentSections(
                "section count must be within 1..=4096",
            ));
        }
        let mut pages = Vec::new();
        let mut first_geometry = None;
        let mut first_scale = None;
        let mut scaled_right = 0_u64;
        let mut scaled_bottom = 0_u64;
        for (index, section) in sections.iter().copied().enumerate() {
            if section.ordinal != index as u32 || section.page_break_before != (index != 0) {
                return Err(HardcopyError::InvalidContentSections(
                    "ordinals and required breaks must match canonical order",
                ));
            }
            let (origin_x, origin_y) = section.origin();
            let right = origin_x
                .0
                .checked_add(section.extent.width.0)
                .ok_or(HardcopyError::ScaledGeometryOverflow)?;
            let bottom = origin_y
                .0
                .checked_add(section.extent.height.0)
                .ok_or(HardcopyError::ScaledGeometryOverflow)?;
            if right > aggregate_extent.width.0 || bottom > aggregate_extent.height.0 {
                return Err(HardcopyError::InvalidContentSections(
                    "section geometry exceeds aggregate extent",
                ));
            }
            let child = Self::build_for_section(setup, section.extent, section.ordinal)?;
            if first_geometry.is_none() {
                first_geometry = Some(child.geometry);
                first_scale = Some(child.scale);
            }
            let scaled_origin_x = scale_floor(origin_x.0, child.scale)?;
            let scaled_origin_y = scale_floor(origin_y.0, child.scale)?;
            let section_right = scaled_origin_x
                .checked_add(child.scaled_content.width.0)
                .ok_or(HardcopyError::ScaledGeometryOverflow)?;
            let section_bottom = scaled_origin_y
                .checked_add(child.scaled_content.height.0)
                .ok_or(HardcopyError::ScaledGeometryOverflow)?;
            scaled_right = scaled_right.max(section_right);
            scaled_bottom = scaled_bottom.max(section_bottom);
            for mut page in child.pages {
                let number = u32::try_from(pages.len())
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .ok_or(HardcopyError::TooManyPages(u32::MAX))?;
                if number > MAX_PREVIEW_PAGES {
                    return Err(HardcopyError::TooManyPages(number));
                }
                page.number = number;
                page.coordinate = format!("S{}-{}", section.ordinal + 1, page.coordinate);
                page.scaled_content_window.x = Length(
                    page.scaled_content_window
                        .x
                        .0
                        .checked_add(scaled_origin_x)
                        .ok_or(HardcopyError::ScaledGeometryOverflow)?,
                );
                page.scaled_content_window.y = Length(
                    page.scaled_content_window
                        .y
                        .0
                        .checked_add(scaled_origin_y)
                        .ok_or(HardcopyError::ScaledGeometryOverflow)?,
                );
                page.section_ordinal = section.ordinal;
                pages.push(page);
            }
        }
        let aggregate_rows = u16::try_from(pages.len())
            .map_err(|_| HardcopyError::TooManyPages(pages.len() as u32))?;
        Ok(Self {
            geometry: first_geometry.ok_or(HardcopyError::InvalidContentSections(
                "aggregate has no page geometry",
            ))?,
            scale: first_scale.ok_or(HardcopyError::InvalidContentSections(
                "aggregate has no scale",
            ))?,
            scaled_content: ContentExtent::try_new(Length(scaled_right), Length(scaled_bottom))?,
            // Each section owns an independent local grid. A synthetic
            // rectangular aggregate grid would imply nonexistent pages when
            // section grids have different widths, so publish the canonical
            // ordered page sequence as one column.
            columns: 1,
            rows: aggregate_rows,
            pages,
            sections: sections.to_vec(),
        })
    }

    #[must_use]
    pub const fn geometry(&self) -> ValidatedPageGeometry {
        self.geometry
    }

    #[must_use]
    pub const fn columns(&self) -> u16 {
        self.columns
    }

    #[must_use]
    pub const fn rows(&self) -> u16 {
        self.rows
    }

    #[must_use]
    pub fn pages(&self) -> &[PreviewPage] {
        &self.pages
    }

    #[must_use]
    pub fn sections(&self) -> &[HardcopyContentSection] {
        &self.sections
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopyPlan {
    schema_version: u32,
    id: HardcopyPlanId,
    source: ActiveHardcopySource,
    setup: HardcopySetup,
    content_extent: ContentExtent,
    pagination: PreviewPagination,
    content_digest: ContentDigest,
}

#[derive(Serialize)]
struct HardcopyPlanMaterial<'a> {
    schema_version: u32,
    id: HardcopyPlanId,
    source: &'a ActiveHardcopySource,
    setup: &'a HardcopySetup,
    content_extent: ContentExtent,
    pagination: &'a PreviewPagination,
}

impl HardcopyPlan {
    pub fn compile(
        source: ActiveHardcopySource,
        setup: HardcopySetup,
        content_extent: ContentExtent,
    ) -> Result<Self, HardcopyError> {
        Self::compile_with_id(HardcopyPlanId::new(), source, setup, content_extent)
    }

    pub fn compile_with_id(
        id: HardcopyPlanId,
        source: ActiveHardcopySource,
        setup: HardcopySetup,
        content_extent: ContentExtent,
    ) -> Result<Self, HardcopyError> {
        source.scope.validate_for(source.document_kind)?;
        setup.validate()?;
        if let PaperSize::MatchAuthoredSheets(media) = setup.physical_page.paper() {
            if source.document_kind != HardcopyDocumentKind::SchematicOrSymbol {
                return Err(HardcopyError::AuthoredMediaRequiresSchematicSource);
            }
            if media.len() != 1 {
                return Err(HardcopyError::InvalidAuthoredSheetMedia(
                    "a non-aggregate plan requires exactly one authored medium",
                ));
            }
        }
        let pagination = PreviewPagination::build(&setup, content_extent)?;
        let material = HardcopyPlanMaterial {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            source: &source,
            setup: &setup,
            content_extent,
            pagination: &pagination,
        };
        let content_digest = canonical_digest(b"rspice-hardcopy-plan-v1", &material)?;
        Ok(Self {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            source,
            setup,
            content_extent,
            pagination,
            content_digest,
        })
    }

    pub fn compile_with_sections(
        source: ActiveHardcopySource,
        setup: HardcopySetup,
        content_extent: ContentExtent,
        sections: Vec<HardcopyContentSection>,
    ) -> Result<Self, HardcopyError> {
        Self::compile_with_id_and_sections(
            HardcopyPlanId::new(),
            source,
            setup,
            content_extent,
            sections,
        )
    }

    pub fn compile_with_id_and_sections(
        id: HardcopyPlanId,
        source: ActiveHardcopySource,
        setup: HardcopySetup,
        content_extent: ContentExtent,
        sections: Vec<HardcopyContentSection>,
    ) -> Result<Self, HardcopyError> {
        source.scope.validate_for(source.document_kind)?;
        setup.validate()?;
        if let PaperSize::MatchAuthoredSheets(media) = setup.physical_page.paper() {
            if source.document_kind != HardcopyDocumentKind::SchematicOrSymbol {
                return Err(HardcopyError::AuthoredMediaRequiresSchematicSource);
            }
            if media.len() != sections.len() {
                return Err(HardcopyError::InvalidAuthoredSheetMedia(
                    "authored media and content sections must have identical cardinality",
                ));
            }
        }
        let pagination = PreviewPagination::build_sections(&setup, content_extent, &sections)?;
        let material = HardcopyPlanMaterial {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            source: &source,
            setup: &setup,
            content_extent,
            pagination: &pagination,
        };
        let content_digest = canonical_digest(b"rspice-hardcopy-plan-v1", &material)?;
        Ok(Self {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            source,
            setup,
            content_extent,
            pagination,
            content_digest,
        })
    }

    #[must_use]
    pub const fn id(&self) -> HardcopyPlanId {
        self.id
    }

    #[must_use]
    pub const fn source(&self) -> &ActiveHardcopySource {
        &self.source
    }

    #[must_use]
    pub const fn setup(&self) -> &HardcopySetup {
        &self.setup
    }

    #[must_use]
    pub const fn content_extent(&self) -> ContentExtent {
        self.content_extent
    }

    #[must_use]
    pub const fn pagination(&self) -> &PreviewPagination {
        &self.pagination
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardcopyArtifactIdentity {
    content_digest: ContentDigest,
    byte_length: u64,
    page_count: u32,
    format: OutputFormat,
}

impl HardcopyArtifactIdentity {
    pub fn try_new(
        content_digest: ContentDigest,
        byte_length: u64,
        page_count: u32,
        format: OutputFormat,
    ) -> Result<Self, HardcopyError> {
        if byte_length == 0 {
            return Err(HardcopyError::EmptyArtifact);
        }
        format.validate()?;
        Ok(Self {
            content_digest,
            byte_length,
            page_count,
            format,
        })
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    #[must_use]
    pub const fn page_count(&self) -> u32 {
        self.page_count
    }

    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CancellationPhase {
    Queued,
    Preparing,
    Rendering,
    Spooling,
    CommittingArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardcopyFailureCode {
    DeviceUnavailable,
    UnsupportedFormat,
    InvalidPrinterConfiguration,
    PermissionDenied,
    RenderFailure,
    InsufficientMemory,
    DestinationWriteFailed,
    SourceChanged,
    InternalFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum HardcopyOutcome {
    ArtifactExported {
        artifact: HardcopyArtifactIdentity,
    },
    /// The native spooler accepted the complete render job. This is not a
    /// claim that paper was produced or the device completed the job.
    SpoolAccepted {
        device_id: String,
        job_id: String,
        pages_accepted: u32,
        source_artifact_digest: ContentDigest,
    },
    /// The browser accepted navigation of a user-initiated window to the
    /// complete print document. This does not claim that the document loaded,
    /// that the print dialog opened, or that a physical print completed.
    BrowserPrintNavigationAccepted {
        navigation_id: String,
        pages_accepted: u32,
        source_artifact_digest: ContentDigest,
    },
    /// A native desktop launcher accepted a request to open the complete,
    /// authenticated print-ready document. This is deliberately distinct from
    /// direct spooling and does not claim that a print dialog opened or paper
    /// was produced.
    DesktopPrintHandoffAccepted {
        handoff_id: String,
        pages_accepted: u32,
        source_artifact_digest: ContentDigest,
    },
    Cancelled {
        phase: CancellationPhase,
        pages_completed: u32,
        reason: Option<String>,
    },
    Failed {
        code: HardcopyFailureCode,
        message: String,
        pages_completed: u32,
        retryable: bool,
    },
}

impl HardcopyOutcome {
    fn validate(&self, plan: &HardcopyPlan) -> Result<(), HardcopyError> {
        let expected_pages = plan.pagination.pages.len() as u32;
        match self {
            Self::ArtifactExported { artifact } => {
                if !matches!(&plan.setup.render.target, RenderTarget::ExportArtifact) {
                    return Err(HardcopyError::OutcomeRenderTargetMismatch);
                }
                if artifact.page_count != expected_pages {
                    return Err(HardcopyError::ArtifactPageCountMismatch {
                        expected: expected_pages,
                        observed: artifact.page_count,
                    });
                }
                if artifact.format != plan.setup.render.format {
                    return Err(HardcopyError::ArtifactFormatMismatch);
                }
            }
            Self::SpoolAccepted {
                device_id,
                job_id,
                pages_accepted,
                ..
            } => {
                let RenderTarget::SystemPrinter { printer_id, .. } = &plan.setup.render.target
                else {
                    return Err(HardcopyError::OutcomeRenderTargetMismatch);
                };
                if plan.setup.render.format != OutputFormat::NativePrinter {
                    return Err(HardcopyError::OutcomeRenderTargetMismatch);
                }
                validate_text("spool device identity", device_id, MAX_TEXT_BYTES)?;
                validate_text("spool job identity", job_id, MAX_TEXT_BYTES)?;
                if device_id != printer_id {
                    return Err(HardcopyError::SpoolDeviceMismatch);
                }
                validate_accepted_pages(*pages_accepted, expected_pages)?;
            }
            Self::BrowserPrintNavigationAccepted {
                navigation_id,
                pages_accepted,
                ..
            } => {
                if !matches!(&plan.setup.render.target, RenderTarget::BrowserPrintDialog)
                    || plan.setup.render.format != OutputFormat::BrowserPrintDocument
                {
                    return Err(HardcopyError::OutcomeRenderTargetMismatch);
                }
                validate_text(
                    "browser print navigation identity",
                    navigation_id,
                    MAX_TEXT_BYTES,
                )?;
                validate_accepted_pages(*pages_accepted, expected_pages)?;
            }
            Self::DesktopPrintHandoffAccepted {
                handoff_id,
                pages_accepted,
                ..
            } => {
                if !matches!(&plan.setup.render.target, RenderTarget::BrowserPrintDialog)
                    || plan.setup.render.format != OutputFormat::BrowserPrintDocument
                {
                    return Err(HardcopyError::OutcomeRenderTargetMismatch);
                }
                validate_text("desktop print handoff identity", handoff_id, MAX_TEXT_BYTES)?;
                validate_accepted_pages(*pages_accepted, expected_pages)?;
            }
            Self::Cancelled {
                pages_completed,
                reason,
                ..
            } => {
                if *pages_completed > expected_pages {
                    return Err(HardcopyError::InvalidCompletedPageCount {
                        completed: *pages_completed,
                        total: expected_pages,
                    });
                }
                if let Some(reason) = reason {
                    validate_text("cancellation reason", reason, MAX_OUTCOME_MESSAGE_BYTES)?;
                }
            }
            Self::Failed {
                message,
                pages_completed,
                ..
            } => {
                validate_text(
                    "hardcopy failure message",
                    message,
                    MAX_OUTCOME_MESSAGE_BYTES,
                )?;
                if *pages_completed > expected_pages {
                    return Err(HardcopyError::InvalidCompletedPageCount {
                        completed: *pages_completed,
                        total: expected_pages,
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopyReceipt {
    schema_version: u32,
    id: HardcopyReceiptId,
    plan_id: HardcopyPlanId,
    plan_content_digest: ContentDigest,
    source_content_digest: ContentDigest,
    outcome: HardcopyOutcome,
    content_digest: ContentDigest,
}

#[derive(Deserialize)]
struct HardcopyReceiptWire {
    schema_version: u32,
    id: HardcopyReceiptId,
    plan_id: HardcopyPlanId,
    plan_content_digest: ContentDigest,
    source_content_digest: ContentDigest,
    outcome: HardcopyOutcome,
    content_digest: ContentDigest,
}

#[derive(Serialize)]
struct HardcopyReceiptMaterial<'a> {
    schema_version: u32,
    id: HardcopyReceiptId,
    plan_id: HardcopyPlanId,
    plan_content_digest: ContentDigest,
    source_content_digest: ContentDigest,
    outcome: &'a HardcopyOutcome,
}

impl HardcopyReceipt {
    pub fn record(plan: &HardcopyPlan, outcome: HardcopyOutcome) -> Result<Self, HardcopyError> {
        Self::record_with_id(HardcopyReceiptId::new(), plan, outcome)
    }

    pub fn record_with_id(
        id: HardcopyReceiptId,
        plan: &HardcopyPlan,
        outcome: HardcopyOutcome,
    ) -> Result<Self, HardcopyError> {
        outcome.validate(plan)?;
        let material = HardcopyReceiptMaterial {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            plan_id: plan.id,
            plan_content_digest: plan.content_digest,
            source_content_digest: plan.source.content_digest,
            outcome: &outcome,
        };
        let content_digest = canonical_digest(b"rspice-hardcopy-receipt-v1", &material)?;
        Ok(Self {
            schema_version: HARDCOPY_SCHEMA_VERSION,
            id,
            plan_id: plan.id,
            plan_content_digest: plan.content_digest,
            source_content_digest: plan.source.content_digest,
            outcome,
            content_digest,
        })
    }

    fn validate_integrity(&self) -> Result<(), HardcopyError> {
        if self.schema_version != HARDCOPY_SCHEMA_VERSION {
            return Err(HardcopyError::UnsupportedReceiptSchema(self.schema_version));
        }
        let material = HardcopyReceiptMaterial {
            schema_version: self.schema_version,
            id: self.id,
            plan_id: self.plan_id,
            plan_content_digest: self.plan_content_digest,
            source_content_digest: self.source_content_digest,
            outcome: &self.outcome,
        };
        let expected = canonical_digest(b"rspice-hardcopy-receipt-v1", &material)?;
        if expected != self.content_digest {
            return Err(HardcopyError::PersistedReceiptDigestMismatch(self.id));
        }
        Ok(())
    }

    #[must_use]
    pub const fn id(&self) -> HardcopyReceiptId {
        self.id
    }

    #[must_use]
    pub const fn plan_id(&self) -> HardcopyPlanId {
        self.plan_id
    }

    #[must_use]
    pub const fn plan_content_digest(&self) -> ContentDigest {
        self.plan_content_digest
    }

    #[must_use]
    pub const fn source_content_digest(&self) -> ContentDigest {
        self.source_content_digest
    }

    #[must_use]
    pub const fn outcome(&self) -> &HardcopyOutcome {
        &self.outcome
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

impl<'de> Deserialize<'de> for HardcopyReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HardcopyReceiptWire::deserialize(deserializer)?;
        let receipt = Self {
            schema_version: wire.schema_version,
            id: wire.id,
            plan_id: wire.plan_id,
            plan_content_digest: wire.plan_content_digest,
            source_content_digest: wire.source_content_digest,
            outcome: wire.outcome,
            content_digest: wire.content_digest,
        };
        receipt
            .validate_integrity()
            .map_err(serde::de::Error::custom)?;
        Ok(receipt)
    }
}

const MAX_RETAINED_HARDCOPY_RECEIPTS: usize = 512;

/// Bounded project-owned history of every observed hardcopy outcome,
/// including cancellation and failure. Entries are individually sealed and
/// revalidated when a project is loaded.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct HardcopyReceiptLedger {
    receipts: Vec<HardcopyReceipt>,
}

impl HardcopyReceiptLedger {
    pub fn append(&mut self, receipt: HardcopyReceipt) -> Result<(), HardcopyError> {
        receipt.validate_integrity()?;
        if self
            .receipts
            .iter()
            .any(|existing| existing.id == receipt.id)
        {
            return Err(HardcopyError::DuplicateReceiptIdentity(receipt.id));
        }
        if self.receipts.len() == MAX_RETAINED_HARDCOPY_RECEIPTS {
            self.receipts.remove(0);
        }
        self.receipts.push(receipt);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), HardcopyError> {
        if self.receipts.len() > MAX_RETAINED_HARDCOPY_RECEIPTS {
            return Err(HardcopyError::TooManyRetainedReceipts(self.receipts.len()));
        }
        let mut ids = std::collections::HashSet::with_capacity(self.receipts.len());
        for receipt in &self.receipts {
            receipt.validate_integrity()?;
            if !ids.insert(receipt.id) {
                return Err(HardcopyError::DuplicateReceiptIdentity(receipt.id));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn receipts(&self) -> &[HardcopyReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn latest(&self) -> Option<&HardcopyReceipt> {
        self.receipts.last()
    }
}

impl<'de> Deserialize<'de> for HardcopyReceiptLedger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(default)]
            receipts: Vec<HardcopyReceipt>,
        }
        let ledger = Self {
            receipts: Wire::deserialize(deserializer)?.receipts,
        };
        ledger.validate().map_err(serde::de::Error::custom)?;
        Ok(ledger)
    }
}

/// One validated per-document setup revision retained in project settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedHardcopySetup {
    document_id: HardcopyDocumentId,
    document_kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    revision: ObjectRevision,
    setup: HardcopySetup,
    content_digest: ContentDigest,
}

impl SavedHardcopySetup {
    fn validate(&self) -> Result<(), HardcopyError> {
        self.scope.validate_for(self.document_kind)?;
        self.setup.validate()?;
        let expected = saved_setup_digest(self.document_kind, &self.scope, &self.setup)?;
        if self.content_digest != expected {
            return Err(HardcopyError::PersistedSetupDigestMismatch(
                self.document_id,
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn document_id(&self) -> HardcopyDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn document_kind(&self) -> HardcopyDocumentKind {
        self.document_kind
    }

    #[must_use]
    pub const fn scope(&self) -> &HardcopyScope {
        &self.scope
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn setup(&self) -> &HardcopySetup {
        &self.setup
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupSaveDisposition {
    Inserted,
    Updated,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupSaveOutcome {
    disposition: SetupSaveDisposition,
    saved: SavedHardcopySetup,
}

impl SetupSaveOutcome {
    #[must_use]
    pub const fn disposition(&self) -> SetupSaveDisposition {
        self.disposition
    }

    #[must_use]
    pub const fn saved(&self) -> &SavedHardcopySetup {
        &self.saved
    }
}

/// Versioned per-document page-setup store. Deserialization is fail-closed:
/// schema, map identity, setup validation, and content digests are all checked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopySetupStore {
    schema_version: u32,
    documents: BTreeMap<HardcopyDocumentId, SavedHardcopySetup>,
}

#[derive(Deserialize)]
struct HardcopySetupStoreWire {
    schema_version: u32,
    documents: BTreeMap<HardcopyDocumentId, SavedHardcopySetup>,
}

impl<'de> Deserialize<'de> for HardcopySetupStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HardcopySetupStoreWire::deserialize(deserializer)?;
        if wire.schema_version != SETUP_STORE_SCHEMA_VERSION {
            return Err(serde::de::Error::custom(
                HardcopyError::UnsupportedSetupStoreSchema(wire.schema_version),
            ));
        }
        for (document_id, saved) in &wire.documents {
            if *document_id != saved.document_id {
                return Err(serde::de::Error::custom(
                    HardcopyError::PersistedSetupKeyMismatch {
                        key: *document_id,
                        entry: saved.document_id,
                    },
                ));
            }
            saved.validate().map_err(serde::de::Error::custom)?;
        }
        Ok(Self {
            schema_version: wire.schema_version,
            documents: wire.documents,
        })
    }
}

impl Default for HardcopySetupStore {
    fn default() -> Self {
        Self {
            schema_version: SETUP_STORE_SCHEMA_VERSION,
            documents: BTreeMap::new(),
        }
    }
}

impl HardcopySetupStore {
    pub fn save(
        &mut self,
        source: &ActiveHardcopySource,
        setup: HardcopySetup,
    ) -> Result<SetupSaveOutcome, HardcopyError> {
        source.scope.validate_for(source.document_kind)?;
        setup.validate()?;
        let digest = saved_setup_digest(source.document_kind, &source.scope, &setup)?;
        let prior = self.documents.get(&source.document_id);
        if let Some(prior) = prior {
            if prior.document_kind != source.document_kind {
                return Err(HardcopyError::PersistedDocumentKindChanged {
                    document_id: source.document_id,
                    retained: prior.document_kind,
                    observed: source.document_kind,
                });
            }
            if prior.content_digest == digest {
                return Ok(SetupSaveOutcome {
                    disposition: SetupSaveDisposition::Unchanged,
                    saved: prior.clone(),
                });
            }
        }
        let (revision, disposition) = if let Some(prior) = prior {
            (prior.revision.next()?, SetupSaveDisposition::Updated)
        } else {
            (ObjectRevision::INITIAL, SetupSaveDisposition::Inserted)
        };
        let saved = SavedHardcopySetup {
            document_id: source.document_id,
            document_kind: source.document_kind,
            scope: source.scope.clone(),
            revision,
            setup,
            content_digest: digest,
        };
        self.documents.insert(source.document_id, saved.clone());
        Ok(SetupSaveOutcome { disposition, saved })
    }

    #[must_use]
    pub fn get(&self, document_id: HardcopyDocumentId) -> Option<&SavedHardcopySetup> {
        self.documents.get(&document_id)
    }

    pub fn setup_for(
        &self,
        source: &ActiveHardcopySource,
    ) -> Result<Option<&SavedHardcopySetup>, HardcopyError> {
        let Some(saved) = self.documents.get(&source.document_id) else {
            return Ok(None);
        };
        if saved.document_kind != source.document_kind {
            return Err(HardcopyError::PersistedDocumentKindChanged {
                document_id: source.document_id,
                retained: saved.document_kind,
                observed: source.document_kind,
            });
        }
        Ok(Some(saved))
    }

    pub fn remove(&mut self, document_id: HardcopyDocumentId) -> Option<SavedHardcopySetup> {
        self.documents.remove(&document_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HardcopyError {
    #[error("{0} identity must not be nil")]
    NilIdentity(&'static str),
    #[error("{field} must be trimmed, printable, non-empty text of at most {maximum} bytes")]
    InvalidText { field: &'static str, maximum: usize },
    #[error("invalid unsigned decimal physical length {0:?}")]
    InvalidLength(String),
    #[error("{field} must be between 10 mm and 5 m")]
    InvalidPageDimension { field: &'static str },
    #[error("the selected scope {scope:?} is not valid for {kind:?}")]
    IncompatibleScope {
        kind: HardcopyDocumentKind,
        scope: HardcopyScope,
    },
    #[error("page margins leave no printable area")]
    MarginsExhaustPage,
    #[error("bleed must be nonzero and no larger than every page margin")]
    BleedExceedsMargins,
    #[error("custom scale {0} hundredths of a percent is outside 0.01%..=1000%")]
    InvalidScale(u32),
    #[error("manual tiling {columns} by {rows} is outside the supported page grid")]
    InvalidManualTiling { columns: u16, rows: u16 },
    #[error("raster resolution {0} dpi is outside 72..=9600 dpi")]
    InvalidRasterResolution(u16),
    #[error("printer roll width must be between 50 mm and 2 m")]
    InvalidPrinterRollWidth,
    #[error("printer resolution {0} dpi is outside 72..=9600 dpi")]
    InvalidPrinterResolution(u16),
    #[error("native printer raster geometry is empty or lies outside the physical page")]
    InvalidPrinterRasterGeometry,
    #[error("copy count {0} is outside 1..=999")]
    InvalidCopyCount(u16),
    #[error("collation requires at least two copies")]
    CollationRequiresMultipleCopies,
    #[error("print gray percentage {0} must be between 1 and 99")]
    InvalidPrintGrayPercent(u8),
    #[error("{field} must be between 1 µm and 25 mm")]
    InvalidPrintFeatureSize { field: &'static str },
    #[error("print mapping contains {0} entries, above the hard limit")]
    TooManyPrintMappings(usize),
    #[error("duplicate {kind:?} print mapping for semantic identity {stable_id:?}")]
    DuplicatePrintObjectIdentity {
        kind: PrintObjectKind,
        stable_id: String,
    },
    #[error("render target and output format are incompatible")]
    IncompatibleRenderTarget,
    #[error("transparent backgrounds require a vector artifact export")]
    TransparentBackgroundRequiresVectorExport,
    #[error("searchable text requires a vector output format")]
    SearchableTextRequiresVectorOutput,
    #[error("searchable text requires embedded fonts")]
    SearchableTextRequiresEmbeddedFonts,
    #[error("PDF/A output requires embedded fonts")]
    PdfARequiresEmbeddedFonts,
    #[error("source content extent must be nonzero")]
    EmptyContentExtent,
    #[error("authenticated content sections are invalid: {0}")]
    InvalidContentSections(&'static str),
    #[error("authored drawing-sheet output media are invalid: {0}")]
    InvalidAuthoredSheetMedia(&'static str),
    #[error("matching authored drawing-sheet media requires a governed schematic source")]
    AuthoredMediaRequiresSchematicSource,
    #[error("{0} leave no printable content area")]
    GeometryUnderflow(&'static str),
    #[error("page decorations leave no printable content area")]
    NoPrintableContentArea,
    #[error("tile overlap must be smaller than both printable content dimensions")]
    OverlapExhaustsPage,
    #[error("scale ratio must be nonzero")]
    ZeroScaleRatio,
    #[error("scaled physical geometry overflowed")]
    ScaledGeometryOverflow,
    #[error("pagination requires {0} pages, above the hard limit")]
    TooManyPages(u32),
    #[error("single-page mode needs a {required_columns} by {required_rows} page grid")]
    SinglePageOverflow {
        required_columns: u16,
        required_rows: u16,
    },
    #[error(
        "manual {columns} by {rows} tiling does not equal the required {required_columns} by {required_rows} grid"
    )]
    ManualTilingDoesNotCover {
        columns: u16,
        rows: u16,
        required_columns: u16,
        required_rows: u16,
    },
    #[error("could not serialize canonical hardcopy content: {0}")]
    CanonicalSerialization(String),
    #[error("rendered artifact must contain at least one byte")]
    EmptyArtifact,
    #[error("artifact page count is {observed}; the immutable plan requires {expected}")]
    ArtifactPageCountMismatch { expected: u32, observed: u32 },
    #[error("artifact format differs from the immutable plan")]
    ArtifactFormatMismatch,
    #[error("success outcome does not match the immutable plan render target")]
    OutcomeRenderTargetMismatch,
    #[error("spool acceptance device differs from the immutable plan printer identity")]
    SpoolDeviceMismatch,
    #[error("accepted page count is {observed}; the immutable plan requires {expected}")]
    AcceptedPageCountMismatch { expected: u32, observed: u32 },
    #[error("{completed} completed pages exceeds the plan total of {total}")]
    InvalidCompletedPageCount { completed: u32, total: u32 },
    #[error("unsupported persisted hardcopy setup schema {0}")]
    UnsupportedSetupStoreSchema(u32),
    #[error("unsupported persisted hardcopy receipt schema {0}")]
    UnsupportedReceiptSchema(u32),
    #[error("persisted hardcopy receipt {0} has a mismatched digest")]
    PersistedReceiptDigestMismatch(HardcopyReceiptId),
    #[error("hardcopy receipt history contains duplicate identity {0}")]
    DuplicateReceiptIdentity(HardcopyReceiptId),
    #[error("hardcopy receipt history contains {0} entries, above the hard limit")]
    TooManyRetainedReceipts(usize),
    #[error("persisted setup map key {key} does not match entry identity {entry}")]
    PersistedSetupKeyMismatch {
        key: HardcopyDocumentId,
        entry: HardcopyDocumentId,
    },
    #[error("persisted hardcopy setup digest does not match document {0}")]
    PersistedSetupDigestMismatch(HardcopyDocumentId),
    #[error(
        "document {document_id} changed kind from retained {retained:?} to observed {observed:?}"
    )]
    PersistedDocumentKindChanged {
        document_id: HardcopyDocumentId,
        retained: HardcopyDocumentKind,
        observed: HardcopyDocumentKind,
    },
    #[error(transparent)]
    Revision(#[from] RevisionError),
}

mod validation;
use validation::*;
#[cfg(test)]
mod tests;
