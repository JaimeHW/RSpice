//! Typed hardcopy and page-setup contracts.
//!
//! This module deliberately contains no egui, printer, filesystem, or browser
//! integration. It turns an exact document snapshot and a validated page setup
//! into an immutable render plan, deterministic preview pages, and a sealed
//! execution receipt. Physical geometry is represented in integer micrometres;
//! neither pagination nor content identities depend on host floating-point
//! behavior.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StandardPaper {
    Letter,
    Legal,
    Tabloid,
    A4,
    A3,
    A2,
    A1,
    A0,
}

impl StandardPaper {
    #[must_use]
    pub const fn portrait_dimensions(self) -> (Length, Length) {
        let (width, height) = match self {
            Self::Letter => (215_900, 279_400),
            Self::Legal => (215_900, 355_600),
            Self::Tabloid => (279_400, 431_800),
            Self::A4 => (210_000, 297_000),
            Self::A3 => (297_000, 420_000),
            Self::A2 => (420_000, 594_000),
            Self::A1 => (594_000, 841_000),
            Self::A0 => (841_000, 1_189_000),
        };
        (Length(width), Length(height))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomPaper {
    name: String,
    width: Length,
    height: Length,
    display_unit: LengthUnit,
}

impl CustomPaper {
    pub fn try_new(
        name: impl Into<String>,
        width: Length,
        height: Length,
        display_unit: LengthUnit,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            name: name.into(),
            width,
            height,
            display_unit,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        validate_text("custom paper name", &self.name, 128)?;
        validate_page_dimension("custom paper width", self.width)?;
        validate_page_dimension("custom paper height", self.height)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn dimensions(&self) -> (Length, Length) {
        (self.width, self.height)
    }

    #[must_use]
    pub const fn display_unit(&self) -> LengthUnit {
        self.display_unit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PaperSize {
    Standard(StandardPaper),
    Custom(CustomPaper),
}

impl PaperSize {
    fn validate(&self) -> Result<(), HardcopyError> {
        match self {
            Self::Standard(_) => Ok(()),
            Self::Custom(custom) => custom.validate(),
        }
    }

    #[must_use]
    pub fn portrait_dimensions(&self) -> (Length, Length) {
        let (width, height) = match self {
            Self::Standard(paper) => paper.portrait_dimensions(),
            Self::Custom(custom) => custom.dimensions(),
        };
        if width <= height {
            (width, height)
        } else {
            (height, width)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageMargins {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

impl PageMargins {
    #[must_use]
    pub const fn uniform(value: Length) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum Bleed {
    None,
    Uniform(Length),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Orientation {
    Portrait,
    Landscape,
    AutomaticPerPage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalPageSetup {
    paper: PaperSize,
    margins: PageMargins,
    bleed: Bleed,
    orientation: Orientation,
}

impl PhysicalPageSetup {
    pub fn try_new(
        paper: PaperSize,
        margins: PageMargins,
        bleed: Bleed,
        orientation: Orientation,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            paper,
            margins,
            bleed,
            orientation,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.paper.validate()?;
        let (width, height) = self.paper.portrait_dimensions();
        validate_margins(width, height, self.margins)?;
        if let Bleed::Uniform(bleed) = self.bleed {
            if bleed == Length::ZERO
                || [
                    self.margins.top,
                    self.margins.right,
                    self.margins.bottom,
                    self.margins.left,
                ]
                .into_iter()
                .any(|margin| bleed > margin)
            {
                return Err(HardcopyError::BleedExceedsMargins);
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn paper(&self) -> &PaperSize {
        &self.paper
    }

    #[must_use]
    pub const fn margins(&self) -> PageMargins {
        self.margins
    }

    #[must_use]
    pub const fn bleed(&self) -> Bleed {
        self.bleed
    }

    #[must_use]
    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum ScaleMode {
    FitPrintableArea,
    EngineeringOneToOne,
    CustomPercent { hundredths_percent: u32 },
    FitWidth,
}

impl ScaleMode {
    fn validate(self) -> Result<(), HardcopyError> {
        if let Self::CustomPercent { hundredths_percent } = self
            && !(1..=100_000).contains(&hundredths_percent)
        {
            return Err(HardcopyError::InvalidScale(hundredths_percent));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum TilingMode {
    Automatic,
    SinglePage,
    Manual { columns: u16, rows: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TilingSetup {
    mode: TilingMode,
    overlap: Length,
    registration_marks_and_coordinates: bool,
}

impl TilingSetup {
    pub fn try_new(
        mode: TilingMode,
        overlap: Length,
        registration_marks_and_coordinates: bool,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            mode,
            overlap,
            registration_marks_and_coordinates,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(self) -> Result<(), HardcopyError> {
        if let TilingMode::Manual { columns, rows } = self.mode
            && (columns == 0
                || rows == 0
                || columns > MAX_MANUAL_AXIS_PAGES
                || rows > MAX_MANUAL_AXIS_PAGES
                || u32::from(columns) * u32::from(rows) > MAX_PREVIEW_PAGES)
        {
            return Err(HardcopyError::InvalidManualTiling { columns, rows });
        }
        Ok(())
    }

    #[must_use]
    pub const fn mode(self) -> TilingMode {
        self.mode
    }

    #[must_use]
    pub const fn overlap(self) -> Length {
        self.overlap
    }

    #[must_use]
    pub const fn registration_marks_and_coordinates(self) -> bool {
        self.registration_marks_and_coordinates
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PrinterMediaSource {
    AutomaticCompatibleTray,
    NamedTray(String),
    Roll { width: Length },
    ManualFeed,
}

impl PrinterMediaSource {
    fn validate(&self) -> Result<(), HardcopyError> {
        match self {
            Self::AutomaticCompatibleTray | Self::ManualFeed => Ok(()),
            Self::NamedTray(name) => {
                validate_text("printer media source identity", name, MAX_TEXT_BYTES)
            }
            Self::Roll { width } => {
                if (50_000..=2_000_000).contains(&width.0) {
                    Ok(())
                } else {
                    Err(HardcopyError::InvalidPrinterRollWidth)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DuplexMode {
    Off,
    LongEdge,
    ShortEdge,
}

/// Exact raster and printable geometry reported by the selected native driver
/// for one paper/orientation/resolution mode. Binding this into the immutable
/// plan prevents platform adapters from silently rounding physical dimensions
/// or clipping planned ink against an unrecorded device margin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterRasterGeometry {
    physical_width_px: u32,
    physical_height_px: u32,
    printable_x_px: u32,
    printable_y_px: u32,
    printable_width_px: u32,
    printable_height_px: u32,
}

impl PrinterRasterGeometry {
    pub fn try_new(
        physical_width_px: u32,
        physical_height_px: u32,
        printable_x_px: u32,
        printable_y_px: u32,
        printable_width_px: u32,
        printable_height_px: u32,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            physical_width_px,
            physical_height_px,
            printable_x_px,
            printable_y_px,
            printable_width_px,
            printable_height_px,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(self) -> Result<(), HardcopyError> {
        let printable_right = self
            .printable_x_px
            .checked_add(self.printable_width_px)
            .ok_or(HardcopyError::InvalidPrinterRasterGeometry)?;
        let printable_bottom = self
            .printable_y_px
            .checked_add(self.printable_height_px)
            .ok_or(HardcopyError::InvalidPrinterRasterGeometry)?;
        if self.physical_width_px == 0
            || self.physical_height_px == 0
            || self.printable_width_px == 0
            || self.printable_height_px == 0
            || printable_right > self.physical_width_px
            || printable_bottom > self.physical_height_px
        {
            return Err(HardcopyError::InvalidPrinterRasterGeometry);
        }
        Ok(())
    }

    #[must_use]
    pub const fn physical_size_px(self) -> (u32, u32) {
        (self.physical_width_px, self.physical_height_px)
    }

    #[must_use]
    pub const fn printable_rect_px(self) -> (u32, u32, u32, u32) {
        (
            self.printable_x_px,
            self.printable_y_px,
            self.printable_width_px,
            self.printable_height_px,
        )
    }
}

/// Device-capability-selected native spool settings. The capabilities digest
/// authenticates the exact driver/device capability snapshot against which
/// these values were selected; the adapter must not silently substitute them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterJobSettings {
    capabilities_digest: ContentDigest,
    selected_paper_id: String,
    raster_geometry: PrinterRasterGeometry,
    media_source: PrinterMediaSource,
    resolution_dpi: u16,
    duplex: DuplexMode,
    copies: u16,
    collate: bool,
}

impl PrinterJobSettings {
    pub fn try_new(
        capabilities_digest: ContentDigest,
        selected_paper_id: impl Into<String>,
        raster_geometry: PrinterRasterGeometry,
        media_source: PrinterMediaSource,
        resolution_dpi: u16,
        duplex: DuplexMode,
        copies: u16,
        collate: bool,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            capabilities_digest,
            selected_paper_id: selected_paper_id.into(),
            raster_geometry,
            media_source,
            resolution_dpi,
            duplex,
            copies,
            collate,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        validate_text(
            "selected printer paper identity",
            &self.selected_paper_id,
            MAX_TEXT_BYTES,
        )?;
        self.raster_geometry.validate()?;
        self.media_source.validate()?;
        if !(72..=9_600).contains(&self.resolution_dpi) {
            return Err(HardcopyError::InvalidPrinterResolution(self.resolution_dpi));
        }
        if !(1..=999).contains(&self.copies) {
            return Err(HardcopyError::InvalidCopyCount(self.copies));
        }
        if self.copies == 1 && self.collate {
            return Err(HardcopyError::CollationRequiresMultipleCopies);
        }
        Ok(())
    }

    #[must_use]
    pub const fn capabilities_digest(&self) -> ContentDigest {
        self.capabilities_digest
    }

    #[must_use]
    pub fn selected_paper_id(&self) -> &str {
        &self.selected_paper_id
    }

    #[must_use]
    pub const fn raster_geometry(&self) -> PrinterRasterGeometry {
        self.raster_geometry
    }

    #[must_use]
    pub const fn media_source(&self) -> &PrinterMediaSource {
        &self.media_source
    }

    #[must_use]
    pub const fn resolution_dpi(&self) -> u16 {
        self.resolution_dpi
    }

    #[must_use]
    pub const fn duplex(&self) -> DuplexMode {
        self.duplex
    }

    #[must_use]
    pub const fn copies(&self) -> u16 {
        self.copies
    }

    #[must_use]
    pub const fn collate(&self) -> bool {
        self.collate
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

impl OutputFormat {
    #[must_use]
    pub const fn is_vector(self) -> bool {
        matches!(
            self,
            Self::BrowserPrintDocument | Self::PdfVector | Self::PdfA | Self::SvgVector
        )
    }

    fn validate(self) -> Result<(), HardcopyError> {
        if let Self::Png { dpi } | Self::Tiff { dpi } = self
            && !(72..=9_600).contains(&dpi)
        {
            return Err(HardcopyError::InvalidRasterResolution(dpi));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorMapping {
    PrintSafeEngineeringPalette,
    ScreenColors,
    GrayscaleWithDashMarkerRedundancy,
    Monochrome,
}

/// Semantic category of an engineering object participating in the exact
/// layer/trace/marker print-mapping table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrintObjectKind {
    Trace,
    Layer,
    Net,
    Marker,
    DrcMarker,
    ReviewAnnotation,
    Other,
}

/// Stable semantic identity and the source-owned screen style shown beside a
/// print mapping. The screen style is descriptive input, never the authority
/// for the resolved print style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintObjectIdentity {
    kind: PrintObjectKind,
    stable_id: String,
    display_name: String,
    screen_style: String,
}

impl PrintObjectIdentity {
    pub fn try_new(
        kind: PrintObjectKind,
        stable_id: impl Into<String>,
        display_name: impl Into<String>,
        screen_style: impl Into<String>,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            kind,
            stable_id: stable_id.into(),
            display_name: display_name.into(),
            screen_style: screen_style.into(),
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        validate_text(
            "print object stable identity",
            &self.stable_id,
            MAX_TEXT_BYTES,
        )?;
        validate_text(
            "print object display name",
            &self.display_name,
            MAX_TEXT_BYTES,
        )?;
        validate_text(
            "print object screen style",
            &self.screen_style,
            MAX_TEXT_BYTES,
        )
    }

    #[must_use]
    pub const fn kind(&self) -> PrintObjectKind {
        self.kind
    }

    #[must_use]
    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn screen_style(&self) -> &str {
        &self.screen_style
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PrintColor {
    Black,
    /// Percentage of black ink coverage. For example, `70` renders as a dark
    /// gray with 30% reflected sRGB intensity.
    GrayPercent(u8),
    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },
}

impl PrintColor {
    fn validate(self) -> Result<(), HardcopyError> {
        if let Self::GrayPercent(percent) = self
            && !(1..=99).contains(&percent)
        {
            return Err(HardcopyError::InvalidPrintGrayPercent(percent));
        }
        Ok(())
    }
}

/// Non-color print encoding. These physical dimensions make redundancy exact
/// in PDF, raster export, and printer backends rather than a display-pixel
/// approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PrintRedundancy {
    SolidLine {
        width: Length,
    },
    DashedLine {
        width: Length,
        dash: Length,
        gap: Length,
    },
    DottedLeader {
        width: Length,
        spacing: Length,
    },
    SolidFill,
    CrossHatch {
        line_width: Length,
        spacing: Length,
    },
    TriangleWithId {
        size: Length,
    },
    SourceStyle,
}

impl PrintRedundancy {
    fn validate(self) -> Result<(), HardcopyError> {
        match self {
            Self::SolidLine { width } => validate_print_feature("line width", width),
            Self::DashedLine { width, dash, gap } => {
                validate_print_feature("line width", width)?;
                validate_print_feature("dash length", dash)?;
                validate_print_feature("dash gap", gap)
            }
            Self::DottedLeader { width, spacing } => {
                validate_print_feature("leader width", width)?;
                validate_print_feature("dot spacing", spacing)
            }
            Self::SolidFill | Self::SourceStyle => Ok(()),
            Self::CrossHatch {
                line_width,
                spacing,
            } => {
                validate_print_feature("hatch line width", line_width)?;
                validate_print_feature("hatch spacing", spacing)
            }
            Self::TriangleWithId { size } => validate_print_feature("marker size", size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintMappingEntry {
    object: PrintObjectIdentity,
    print_color: PrintColor,
    redundancy: PrintRedundancy,
    include_in_legend: bool,
}

impl PrintMappingEntry {
    pub fn try_new(
        object: PrintObjectIdentity,
        print_color: PrintColor,
        redundancy: PrintRedundancy,
        include_in_legend: bool,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            object,
            print_color,
            redundancy,
            include_in_legend,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.object.validate()?;
        self.print_color.validate()?;
        self.redundancy.validate()
    }

    #[must_use]
    pub const fn object(&self) -> &PrintObjectIdentity {
        &self.object
    }

    #[must_use]
    pub const fn print_color(&self) -> PrintColor {
        self.print_color
    }

    #[must_use]
    pub const fn redundancy(&self) -> PrintRedundancy {
        self.redundancy
    }

    #[must_use]
    pub const fn include_in_legend(&self) -> bool {
        self.include_in_legend
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PrintMappingSaveScope {
    Document,
    ProjectPrintSet(String),
    PortablePersonalPreset(String),
}

impl PrintMappingSaveScope {
    fn validate(&self) -> Result<(), HardcopyError> {
        match self {
            Self::Document => Ok(()),
            Self::ProjectPrintSet(name) => {
                validate_text("project print set mapping name", name, MAX_TEXT_BYTES)
            }
            Self::PortablePersonalPreset(name) => {
                validate_text("personal print mapping preset name", name, MAX_TEXT_BYTES)
            }
        }
    }
}

/// Exact, ordered mapping table for layer, trace, net, marker, and annotation
/// output. Entries remain in source display order while duplicate semantic
/// identities are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintMappingTable {
    save_scope: PrintMappingSaveScope,
    entries: Vec<PrintMappingEntry>,
}

impl PrintMappingTable {
    pub fn try_new(
        save_scope: PrintMappingSaveScope,
        entries: Vec<PrintMappingEntry>,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            save_scope,
            entries,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), HardcopyError> {
        self.save_scope.validate()?;
        if self.entries.len() > 4_096 {
            return Err(HardcopyError::TooManyPrintMappings(self.entries.len()));
        }
        let mut identities = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            let identity = (entry.object.kind, entry.object.stable_id.clone());
            if !identities.insert(identity) {
                return Err(HardcopyError::DuplicatePrintObjectIdentity {
                    kind: entry.object.kind,
                    stable_id: entry.object.stable_id.clone(),
                });
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn save_scope(&self) -> &PrintMappingSaveScope {
        &self.save_scope
    }

    #[must_use]
    pub fn entries(&self) -> &[PrintMappingEntry] {
        &self.entries
    }
}

impl Default for PrintMappingTable {
    fn default() -> Self {
        Self {
            save_scope: PrintMappingSaveScope::Document,
            entries: Vec::new(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopySetup {
    physical_page: PhysicalPageSetup,
    scale: ScaleMode,
    tiling: TilingSetup,
    render: RenderSetup,
    decorations: DecorationSetup,
    print_mapping: PrintMappingTable,
}

#[derive(Deserialize)]
struct HardcopySetupWire {
    physical_page: PhysicalPageSetup,
    scale: ScaleMode,
    tiling: TilingSetup,
    render: RenderSetup,
    decorations: DecorationSetup,
    print_mapping: PrintMappingTable,
}

impl<'de> Deserialize<'de> for HardcopySetup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HardcopySetupWire::deserialize(deserializer)?;
        Self::try_new(
            wire.physical_page,
            wire.scale,
            wire.tiling,
            wire.render,
            wire.decorations,
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
        let value = Self {
            physical_page,
            scale,
            tiling,
            render,
            decorations,
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
    fn resolve(setup: &HardcopySetup, content: ContentExtent) -> Result<Self, HardcopyError> {
        let orientation = match setup.physical_page.orientation {
            Orientation::Portrait => ResolvedOrientation::Portrait,
            Orientation::Landscape => ResolvedOrientation::Landscape,
            Orientation::AutomaticPerPage => {
                if let RenderTarget::SystemPrinter { job, .. } = setup.render.target() {
                    let (width_pixels, height_pixels) = job.raster_geometry().physical_size_px();
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
        };
        let (portrait_width, portrait_height) = setup.physical_page.paper.portrait_dimensions();
        let (physical_width, physical_height) = match orientation {
            ResolvedOrientation::Portrait => (portrait_width, portrait_height),
            ResolvedOrientation::Landscape => (portrait_height, portrait_width),
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
    pub const fn row(&self) -> u16 {
        self.row
    }

    #[must_use]
    pub const fn column(&self) -> u16 {
        self.column
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
        let geometry = ValidatedPageGeometry::resolve(setup, content)?;
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
            let child = Self::build(setup, section.extent)?;
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
    pub const fn scale(&self) -> ScaleRatio {
        self.scale
    }

    #[must_use]
    pub const fn scaled_content(&self) -> ContentExtent {
        self.scaled_content
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

fn validate_text(field: &'static str, value: &str, maximum: usize) -> Result<(), HardcopyError> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        Err(HardcopyError::InvalidText { field, maximum })
    } else {
        Ok(())
    }
}

fn validate_page_dimension(field: &'static str, value: Length) -> Result<(), HardcopyError> {
    if (MIN_PAGE_DIMENSION_UM..=MAX_PAGE_DIMENSION_UM).contains(&value.0) {
        Ok(())
    } else {
        Err(HardcopyError::InvalidPageDimension { field })
    }
}

fn validate_print_feature(field: &'static str, value: Length) -> Result<(), HardcopyError> {
    if (1..=25_000).contains(&value.0) {
        Ok(())
    } else {
        Err(HardcopyError::InvalidPrintFeatureSize { field })
    }
}

fn validate_margins(
    width: Length,
    height: Length,
    margins: PageMargins,
) -> Result<(), HardcopyError> {
    let horizontal = margins.left.0.checked_add(margins.right.0);
    let vertical = margins.top.0.checked_add(margins.bottom.0);
    if horizontal.is_none_or(|value| value >= width.0)
        || vertical.is_none_or(|value| value >= height.0)
    {
        Err(HardcopyError::MarginsExhaustPage)
    } else {
        Ok(())
    }
}

fn checked_sub(value: u64, consumed: u64, label: &'static str) -> Result<u64, HardcopyError> {
    value
        .checked_sub(consumed)
        .ok_or(HardcopyError::GeometryUnderflow(label))
}

fn validate_accepted_pages(observed: u32, expected: u32) -> Result<(), HardcopyError> {
    if observed == expected {
        Ok(())
    } else {
        Err(HardcopyError::AcceptedPageCountMismatch { expected, observed })
    }
}

fn greatest_common_divisor(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn ratio_less_than(left: ScaleRatio, right: ScaleRatio) -> bool {
    u128::from(left.numerator) * u128::from(right.denominator)
        < u128::from(right.numerator) * u128::from(left.denominator)
}

fn scale_ceil(value: u64, scale: ScaleRatio) -> Result<u64, HardcopyError> {
    let scaled = u128::from(value)
        .checked_mul(u128::from(scale.numerator))
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    let denominator = u128::from(scale.denominator);
    let rounded = scaled
        .checked_add(denominator - 1)
        .ok_or(HardcopyError::ScaledGeometryOverflow)?
        / denominator;
    u64::try_from(rounded).map_err(|_| HardcopyError::ScaledGeometryOverflow)
}

fn scale_floor(value: u64, scale: ScaleRatio) -> Result<u64, HardcopyError> {
    let product = u128::from(value)
        .checked_mul(u128::from(scale.numerator))
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    u64::try_from(product / u128::from(scale.denominator))
        .map_err(|_| HardcopyError::ScaledGeometryOverflow)
}

fn minimum_tiles(content: u64, viewport: u64, overlap: u64) -> Result<u16, HardcopyError> {
    if content <= viewport {
        return Ok(1);
    }
    let step = viewport
        .checked_sub(overlap)
        .ok_or(HardcopyError::OverlapExhaustsPage)?;
    if step == 0 {
        return Err(HardcopyError::OverlapExhaustsPage);
    }
    let remainder = content - viewport;
    let additional = remainder.div_ceil(step);
    let count = 1_u64
        .checked_add(additional)
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    let count = u16::try_from(count).map_err(|_| HardcopyError::TooManyPages(u32::MAX))?;
    Ok(count)
}

fn spreadsheet_column(zero_based: u16) -> String {
    let mut value = u32::from(zero_based) + 1;
    let mut reversed = Vec::new();
    while value != 0 {
        value -= 1;
        reversed.push((b'A' + (value % 26) as u8) as char);
        value /= 26;
    }
    reversed.into_iter().rev().collect()
}

fn canonical_digest(domain: &[u8], value: &impl Serialize) -> Result<ContentDigest, HardcopyError> {
    let payload = serde_json::to_vec(value)
        .map_err(|error| HardcopyError::CanonicalSerialization(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[derive(Serialize)]
struct SavedSetupMaterial<'a> {
    schema_version: u32,
    document_kind: HardcopyDocumentKind,
    scope: &'a HardcopyScope,
    setup: &'a HardcopySetup,
}

fn saved_setup_digest(
    document_kind: HardcopyDocumentKind,
    scope: &HardcopyScope,
    setup: &HardcopySetup,
) -> Result<ContentDigest, HardcopyError> {
    canonical_digest(
        b"rspice-hardcopy-document-setup-v1",
        &SavedSetupMaterial {
            schema_version: SETUP_STORE_SCHEMA_VERSION,
            document_kind,
            scope,
            setup,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn printer_geometry() -> PrinterRasterGeometry {
        PrinterRasterGeometry::try_new(10_200, 13_200, 0, 0, 10_200, 13_200).unwrap()
    }

    fn id(value: u128) -> HardcopyDocumentId {
        HardcopyDocumentId::try_from_uuid(Uuid::from_u128(value)).expect("document id")
    }

    fn source_with(
        document_id: HardcopyDocumentId,
        kind: HardcopyDocumentKind,
        scope: HardcopyScope,
    ) -> ActiveHardcopySource {
        ActiveHardcopySource::try_new(
            document_id,
            ObjectRevision::INITIAL,
            digest(0x42),
            "top · schematic",
            kind,
            scope,
        )
        .expect("source")
    }

    fn source() -> ActiveHardcopySource {
        source_with(
            id(1),
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
    }

    fn one_to_one_setup(tiling: TilingMode, overlap: Length) -> HardcopySetup {
        HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins::uniform(Length::from_micrometres(10_000)),
                Bleed::None,
                Orientation::Landscape,
            )
            .unwrap(),
            ScaleMode::EngineeringOneToOne,
            TilingSetup::try_new(tiling, overlap, true).unwrap(),
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::PdfVector,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
            PrintMappingTable::default(),
        )
        .unwrap()
    }

    fn printer_job() -> PrinterJobSettings {
        PrinterJobSettings::try_new(
            digest(0x35),
            "paper-1",
            printer_geometry(),
            PrinterMediaSource::AutomaticCompatibleTray,
            1_200,
            DuplexMode::Off,
            1,
            false,
        )
        .unwrap()
    }

    #[test]
    fn decimal_lengths_are_exact_and_unit_typed() {
        assert_eq!(
            Length::parse_decimal("0.25", LengthUnit::Inches).unwrap(),
            Length::from_micrometres(6_350)
        );
        assert_eq!(
            Length::parse_decimal("210", LengthUnit::Millimetres).unwrap(),
            Length::from_micrometres(210_000)
        );
        assert!(Length::parse_decimal("1 in", LengthUnit::Inches).is_err());
        assert!(Length::parse_decimal("-1", LengthUnit::Inches).is_err());
    }

    #[test]
    fn scope_is_checked_against_document_kind() {
        assert!(
            ActiveHardcopySource::try_new(
                id(2),
                ObjectRevision::INITIAL,
                digest(1),
                "waveform",
                HardcopyDocumentKind::PlotOrWorksheet,
                HardcopyScope::ActivePlotDocument,
            )
            .is_ok()
        );
        assert!(matches!(
            ActiveHardcopySource::try_new(
                id(3),
                ObjectRevision::INITIAL,
                digest(1),
                "waveform",
                HardcopyDocumentKind::PlotOrWorksheet,
                HardcopyScope::CurrentSheet,
            ),
            Err(HardcopyError::IncompatibleScope { .. })
        ));
    }

    #[test]
    fn custom_paper_and_bleed_fail_closed() {
        let paper = CustomPaper::try_new(
            "Engineering custom 11×17",
            Length::parse_decimal("17", LengthUnit::Inches).unwrap(),
            Length::parse_decimal("11", LengthUnit::Inches).unwrap(),
            LengthUnit::Inches,
        )
        .unwrap();
        assert_eq!(paper.display_unit(), LengthUnit::Inches);
        assert!(
            PhysicalPageSetup::try_new(
                PaperSize::Custom(paper),
                PageMargins::uniform(Length::from_micrometres(5_000)),
                Bleed::Uniform(Length::from_micrometres(6_000)),
                Orientation::Landscape,
            )
            .is_err()
        );
    }

    #[test]
    fn decoration_bands_are_reserved_from_printable_geometry() {
        let plan = HardcopyPlan::compile(
            source(),
            HardcopySetup::default(),
            ContentExtent::try_new(
                Length::from_micrometres(400_000),
                Length::from_micrometres(200_000),
            )
            .unwrap(),
        )
        .unwrap();
        let geometry = plan.pagination().geometry();
        assert_eq!(geometry.orientation(), ResolvedOrientation::Landscape);
        assert_eq!(
            geometry.header_band(),
            Length::from_micrometres(HEADER_BAND_UM)
        );
        assert_eq!(
            geometry.provenance_band(),
            Length::from_micrometres(PROVENANCE_BAND_UM)
        );
        assert!(geometry.content_rect().height < geometry.printable_rect().height);
    }

    #[test]
    fn automatic_pagination_is_row_major_with_engineering_coordinates() {
        let setup = one_to_one_setup(TilingMode::Automatic, Length::from_micrometres(10_000));
        let plan = HardcopyPlan::compile(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(400_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(plan.pagination().columns(), 2);
        assert_eq!(plan.pagination().rows(), 1);
        assert_eq!(plan.pagination().pages()[0].coordinate(), "A1");
        assert_eq!(plan.pagination().pages()[1].coordinate(), "B1");
        assert_eq!(
            plan.pagination().pages()[1].scaled_content_window().x,
            Length::from_micrometres(249_400)
        );
    }

    #[test]
    fn single_page_and_manual_tiling_reject_uncovered_content() {
        let content = ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        assert!(matches!(
            HardcopyPlan::compile(
                source(),
                one_to_one_setup(TilingMode::SinglePage, Length::ZERO),
                content,
            ),
            Err(HardcopyError::SinglePageOverflow {
                required_columns: 2,
                ..
            })
        ));
        assert!(matches!(
            HardcopyPlan::compile(
                source(),
                one_to_one_setup(
                    TilingMode::Manual {
                        columns: 3,
                        rows: 1
                    },
                    Length::from_micrometres(10_000),
                ),
                content,
            ),
            Err(HardcopyError::ManualTilingDoesNotCover {
                columns: 3,
                required_columns: 2,
                ..
            })
        ));
    }

    #[test]
    fn automatic_orientation_follows_selected_content_aspect() {
        let mut setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
        setup.physical_page.orientation = Orientation::AutomaticPerPage;
        let portrait = HardcopyPlan::compile(
            source(),
            setup.clone(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(200_000),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            portrait.pagination().geometry().orientation(),
            ResolvedOrientation::Portrait
        );
        let landscape = HardcopyPlan::compile(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(200_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            landscape.pagination().geometry().orientation(),
            ResolvedOrientation::Landscape
        );
    }

    #[test]
    fn aggregate_sections_preserve_mixed_automatic_page_orientations() {
        let mut setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
        setup.physical_page.orientation = Orientation::AutomaticPerPage;
        let portrait_extent = ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(200_000),
        )
        .unwrap();
        let landscape_extent = ContentExtent::try_new(
            Length::from_micrometres(200_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let sections = vec![
            HardcopyContentSection::try_new(
                0,
                digest(0x10),
                Length::ZERO,
                Length::ZERO,
                portrait_extent,
                false,
            )
            .unwrap(),
            HardcopyContentSection::try_new(
                1,
                digest(0x11),
                Length::ZERO,
                Length::from_micrometres(205_000),
                landscape_extent,
                true,
            )
            .unwrap(),
        ];
        let plan = HardcopyPlan::compile_with_sections(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(200_000),
                Length::from_micrometres(305_000),
            )
            .unwrap(),
            sections,
        )
        .unwrap();
        assert_eq!(plan.pagination().pages().len(), 2);
        assert_eq!(
            plan.pagination().pages()[0].geometry().orientation(),
            ResolvedOrientation::Portrait
        );
        assert_eq!(
            plan.pagination().pages()[1].geometry().orientation(),
            ResolvedOrientation::Landscape
        );
        assert_eq!(plan.pagination().pages()[0].coordinate(), "S1-A1");
        assert_eq!(plan.pagination().pages()[1].coordinate(), "S2-A1");
        assert_eq!(
            plan.pagination().pages()[1].scaled_content_window().y,
            Length::from_micrometres(205_000)
        );
    }

    #[test]
    fn heterogeneous_aggregate_grids_publish_a_truthful_linear_page_summary() {
        let setup = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
        let wide_extent = ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let compact_extent = ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let sections = vec![
            HardcopyContentSection::try_new(
                0,
                digest(0x20),
                Length::ZERO,
                Length::ZERO,
                wide_extent,
                false,
            )
            .unwrap(),
            HardcopyContentSection::try_new(
                1,
                digest(0x21),
                Length::ZERO,
                Length::from_micrometres(105_000),
                compact_extent,
                true,
            )
            .unwrap(),
        ];
        let plan = HardcopyPlan::compile_with_sections(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(400_000),
                Length::from_micrometres(205_000),
            )
            .unwrap(),
            sections,
        )
        .unwrap();
        assert_eq!(plan.pagination().pages().len(), 3);
        assert_eq!(plan.pagination().columns(), 1);
        assert_eq!(plan.pagination().rows(), 3);
        assert_eq!(
            usize::from(plan.pagination().columns()) * usize::from(plan.pagination().rows()),
            plan.pagination().pages().len()
        );
        assert_eq!(
            plan.pagination()
                .pages()
                .iter()
                .map(PreviewPage::coordinate)
                .collect::<Vec<_>>(),
            vec!["S1-A1", "S1-B1", "S2-A1"]
        );
        assert_eq!(
            plan.pagination()
                .pages()
                .iter()
                .map(PreviewPage::section_ordinal)
                .collect::<Vec<_>>(),
            vec![0, 0, 1]
        );
    }

    #[test]
    fn native_automatic_orientation_resolves_from_the_sealed_driver_geometry() {
        let setup = HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins::uniform(Length::from_micrometres(10_000)),
                Bleed::None,
                Orientation::AutomaticPerPage,
            )
            .unwrap(),
            ScaleMode::EngineeringOneToOne,
            TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
            RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: "landscape-driver".to_owned(),
                    job: PrinterJobSettings::try_new(
                        digest(0x31),
                        "letter",
                        PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612).unwrap(),
                        PrinterMediaSource::AutomaticCompatibleTray,
                        72,
                        DuplexMode::Off,
                        1,
                        false,
                    )
                    .unwrap(),
                },
                OutputFormat::NativePrinter,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, false),
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
            PrintMappingTable::default(),
        )
        .unwrap();
        let sections = vec![
            HardcopyContentSection::try_new(
                0,
                digest(0x32),
                Length::ZERO,
                Length::ZERO,
                ContentExtent::try_new(
                    Length::from_micrometres(100_000),
                    Length::from_micrometres(200_000),
                )
                .unwrap(),
                false,
            )
            .unwrap(),
            HardcopyContentSection::try_new(
                1,
                digest(0x33),
                Length::ZERO,
                Length::from_micrometres(205_000),
                ContentExtent::try_new(
                    Length::from_micrometres(200_000),
                    Length::from_micrometres(100_000),
                )
                .unwrap(),
                true,
            )
            .unwrap(),
        ];
        let plan = HardcopyPlan::compile_with_sections(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(200_000),
                Length::from_micrometres(305_000),
            )
            .unwrap(),
            sections,
        )
        .unwrap();
        assert!(
            plan.pagination()
                .pages()
                .iter()
                .all(|page| { page.geometry().orientation() == ResolvedOrientation::Landscape })
        );
    }

    #[test]
    fn render_target_background_and_font_combinations_are_validated() {
        assert!(matches!(
            RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: "office-a".to_owned(),
                    job: printer_job(),
                },
                OutputFormat::PdfVector,
                ColorMapping::Monochrome,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            ),
            Err(HardcopyError::IncompatibleRenderTarget)
        ));
        assert!(matches!(
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::Png { dpi: 600 },
                ColorMapping::ScreenColors,
                BackgroundMode::Transparent,
                FontPolicy::new(true, false),
                false,
            ),
            Err(HardcopyError::TransparentBackgroundRequiresVectorExport)
        ));
        assert!(matches!(
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::Png { dpi: 600 },
                ColorMapping::ScreenColors,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                false,
            ),
            Err(HardcopyError::SearchableTextRequiresVectorOutput)
        ));
    }

    #[test]
    fn printer_capability_selection_is_validated_and_plan_authenticated() {
        assert!(matches!(
            PrinterJobSettings::try_new(
                digest(0x35),
                "paper-1",
                printer_geometry(),
                PrinterMediaSource::ManualFeed,
                1_200,
                DuplexMode::LongEdge,
                0,
                false,
            ),
            Err(HardcopyError::InvalidCopyCount(0))
        ));
        assert!(matches!(
            PrinterJobSettings::try_new(
                digest(0x35),
                "paper-1",
                printer_geometry(),
                PrinterMediaSource::Roll {
                    width: Length::from_micrometres(25_000),
                },
                1_200,
                DuplexMode::Off,
                1,
                false,
            ),
            Err(HardcopyError::InvalidPrinterRollWidth)
        ));

        let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(61)).unwrap();
        let extent = ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let native_setup = |resolution_dpi| {
            let mut setup = HardcopySetup::default();
            setup.render = RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: "engineering-printer-04".to_owned(),
                    job: PrinterJobSettings::try_new(
                        digest(0x35),
                        "paper-1",
                        printer_geometry(),
                        PrinterMediaSource::NamedTray("Letter / A4 tray".to_owned()),
                        resolution_dpi,
                        DuplexMode::ShortEdge,
                        2,
                        true,
                    )
                    .unwrap(),
                },
                OutputFormat::NativePrinter,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, false),
                true,
            )
            .unwrap();
            setup
        };
        let high_resolution =
            HardcopyPlan::compile_with_id(plan_id, source(), native_setup(1_200), extent).unwrap();
        let standard_resolution =
            HardcopyPlan::compile_with_id(plan_id, source(), native_setup(600), extent).unwrap();
        assert_ne!(
            high_resolution.content_digest(),
            standard_resolution.content_digest()
        );
    }

    #[test]
    fn per_object_print_mapping_covers_mockup_style_and_save_scope() {
        let trace = PrintMappingEntry::try_new(
            PrintObjectIdentity::try_new(
                PrintObjectKind::Trace,
                "trace:v(afe_out)",
                "V(afe_out)",
                "cyan solid · 2 px",
            )
            .unwrap(),
            PrintColor::Black,
            PrintRedundancy::SolidLine {
                width: Length::from_micrometres(350),
            },
            true,
        )
        .unwrap();
        let layer = PrintMappingEntry::try_new(
            PrintObjectIdentity::try_new(
                PrintObjectKind::Layer,
                "layer:m4:drawing",
                "M4 drawing",
                "yellow fill",
            )
            .unwrap(),
            PrintColor::GrayPercent(40),
            PrintRedundancy::CrossHatch {
                line_width: Length::from_micrometres(200),
                spacing: Length::from_micrometres(1_500),
            },
            true,
        )
        .unwrap();
        let table = PrintMappingTable::try_new(
            PrintMappingSaveScope::ProjectPrintSet("Release review".to_owned()),
            vec![trace, layer],
        )
        .unwrap();
        assert_eq!(table.entries().len(), 2);
        assert!(matches!(
            table.save_scope(),
            PrintMappingSaveScope::ProjectPrintSet(name) if name == "Release review"
        ));
        assert!(
            table
                .entries()
                .iter()
                .all(PrintMappingEntry::include_in_legend)
        );
    }

    #[test]
    fn print_mapping_rejects_duplicate_semantic_objects_and_invalid_physical_styles() {
        let object = PrintObjectIdentity::try_new(
            PrintObjectKind::Trace,
            "trace:v(sensor_p)",
            "V(sensor_p)",
            "green solid · 2 px",
        )
        .unwrap();
        let entry = PrintMappingEntry::try_new(
            object,
            PrintColor::GrayPercent(70),
            PrintRedundancy::DashedLine {
                width: Length::from_micrometres(300),
                dash: Length::from_micrometres(2_000),
                gap: Length::from_micrometres(1_000),
            },
            true,
        )
        .unwrap();
        assert!(matches!(
            PrintMappingTable::try_new(
                PrintMappingSaveScope::PortablePersonalPreset("Lab printer".to_owned()),
                vec![entry.clone(), entry],
            ),
            Err(HardcopyError::DuplicatePrintObjectIdentity { .. })
        ));
        assert!(matches!(
            PrintMappingEntry::try_new(
                PrintObjectIdentity::try_new(
                    PrintObjectKind::DrcMarker,
                    "drc:blocking",
                    "DRC blocking",
                    "red marker",
                )
                .unwrap(),
                PrintColor::Black,
                PrintRedundancy::TriangleWithId { size: Length::ZERO },
                true,
            ),
            Err(HardcopyError::InvalidPrintFeatureSize { .. })
        ));
    }

    #[test]
    fn fixed_identity_produces_deterministic_plan_digest() {
        let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(50)).unwrap();
        let extent = ContentExtent::try_new(
            Length::from_micrometres(400_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let first = HardcopyPlan::compile_with_id(
            plan_id,
            source(),
            one_to_one_setup(TilingMode::Automatic, Length::ZERO),
            extent,
        )
        .unwrap();
        let second = HardcopyPlan::compile_with_id(
            plan_id,
            source(),
            one_to_one_setup(TilingMode::Automatic, Length::ZERO),
            extent,
        )
        .unwrap();
        assert_eq!(first.content_digest(), second.content_digest());
        assert_eq!(first.pagination(), second.pagination());
    }

    #[test]
    fn source_content_identity_is_bound_into_plan_digest() {
        let plan_id = HardcopyPlanId::try_from_uuid(Uuid::from_u128(51)).unwrap();
        let extent = ContentExtent::try_new(
            Length::from_micrometres(100_000),
            Length::from_micrometres(100_000),
        )
        .unwrap();
        let first =
            HardcopyPlan::compile_with_id(plan_id, source(), HardcopySetup::default(), extent)
                .unwrap();
        let mut changed = source();
        changed.content_digest = digest(0x99);
        let second =
            HardcopyPlan::compile_with_id(plan_id, changed, HardcopySetup::default(), extent)
                .unwrap();
        assert_ne!(first.content_digest(), second.content_digest());
    }

    #[test]
    fn setup_store_is_per_document_and_revisioned_only_on_change() {
        let mut store = HardcopySetupStore::default();
        let source = source();
        let inserted = store.save(&source, HardcopySetup::default()).unwrap();
        assert_eq!(inserted.disposition(), SetupSaveDisposition::Inserted);
        assert_eq!(inserted.saved().revision(), ObjectRevision::INITIAL);

        let unchanged = store.save(&source, HardcopySetup::default()).unwrap();
        assert_eq!(unchanged.disposition(), SetupSaveDisposition::Unchanged);
        assert_eq!(unchanged.saved().revision(), ObjectRevision::INITIAL);

        let changed = one_to_one_setup(TilingMode::Automatic, Length::ZERO);
        let updated = store.save(&source, changed).unwrap();
        assert_eq!(updated.disposition(), SetupSaveDisposition::Updated);
        assert_eq!(updated.saved().revision().get(), 2);
        assert_eq!(store.len(), 1);
        assert_eq!(store.setup_for(&source).unwrap(), Some(updated.saved()));
    }

    #[test]
    fn setup_store_round_trips_and_rejects_digest_tampering() {
        let mut store = HardcopySetupStore::default();
        store.save(&source(), HardcopySetup::default()).unwrap();
        let encoded = serde_json::to_value(&store).unwrap();
        let restored: HardcopySetupStore = serde_json::from_value(encoded.clone()).unwrap();
        assert_eq!(store, restored);

        let mut tampered = encoded;
        let key = source().document_id().to_string();
        tampered["documents"][&key]["content_digest"] =
            serde_json::Value::String(digest(0xee).to_string());
        assert!(serde_json::from_value::<HardcopySetupStore>(tampered).is_err());
    }

    #[test]
    fn setup_store_rejects_document_kind_reuse() {
        let mut store = HardcopySetupStore::default();
        let source = source();
        store.save(&source, HardcopySetup::default()).unwrap();
        let changed_kind = source_with(
            source.document_id(),
            HardcopyDocumentKind::PlotOrWorksheet,
            HardcopyScope::ActivePlotDocument,
        );
        assert!(matches!(
            store.save(&changed_kind, HardcopySetup::default()),
            Err(HardcopyError::PersistedDocumentKindChanged { .. })
        ));
    }

    #[test]
    fn success_receipt_binds_exact_plan_artifact_and_digest() {
        let plan = HardcopyPlan::compile(
            source(),
            HardcopySetup::default(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        let artifact = HardcopyArtifactIdentity::try_new(
            digest(0x77),
            12_345,
            plan.pagination().pages().len() as u32,
            OutputFormat::PdfVector,
        )
        .unwrap();
        let receipt_id = HardcopyReceiptId::try_from_uuid(Uuid::from_u128(70)).unwrap();
        let first = HardcopyReceipt::record_with_id(
            receipt_id,
            &plan,
            HardcopyOutcome::ArtifactExported {
                artifact: artifact.clone(),
            },
        )
        .unwrap();
        let second = HardcopyReceipt::record_with_id(
            receipt_id,
            &plan,
            HardcopyOutcome::ArtifactExported { artifact },
        )
        .unwrap();
        assert_eq!(first.content_digest(), second.content_digest());
        assert_eq!(first.plan_id(), plan.id());
        assert_eq!(first.plan_content_digest(), plan.content_digest());
        assert_eq!(
            first.source_content_digest(),
            plan.source().content_digest()
        );
    }

    #[test]
    fn success_receipt_rejects_wrong_page_count_or_format() {
        let plan = HardcopyPlan::compile(
            source(),
            HardcopySetup::default(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        let wrong_pages =
            HardcopyArtifactIdentity::try_new(digest(1), 100, 99, OutputFormat::PdfVector).unwrap();
        assert!(matches!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::ArtifactExported {
                    artifact: wrong_pages
                }
            ),
            Err(HardcopyError::ArtifactPageCountMismatch { .. })
        ));
        let wrong_format = HardcopyArtifactIdentity::try_new(
            digest(1),
            100,
            plan.pagination().pages().len() as u32,
            OutputFormat::SvgVector,
        )
        .unwrap();
        assert!(matches!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::ArtifactExported {
                    artifact: wrong_format
                }
            ),
            Err(HardcopyError::ArtifactFormatMismatch)
        ));
    }

    #[test]
    fn native_spool_acceptance_is_truthful_and_device_bound() {
        let mut setup = HardcopySetup::default();
        setup.render = RenderSetup::try_new(
            RenderTarget::SystemPrinter {
                printer_id: "engineering-printer-04".to_owned(),
                job: printer_job(),
            },
            OutputFormat::NativePrinter,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, false),
            true,
        )
        .unwrap();
        let plan = HardcopyPlan::compile(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        let pages = plan.pagination().pages().len() as u32;
        let receipt = HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::SpoolAccepted {
                device_id: "engineering-printer-04".to_owned(),
                job_id: "spool-job-813".to_owned(),
                pages_accepted: pages,
                source_artifact_digest: digest(0x81),
            },
        )
        .unwrap();
        assert!(matches!(
            receipt.outcome(),
            HardcopyOutcome::SpoolAccepted { job_id, .. } if job_id == "spool-job-813"
        ));
        assert!(matches!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::SpoolAccepted {
                    device_id: "different-printer".to_owned(),
                    job_id: "spool-job-814".to_owned(),
                    pages_accepted: pages,
                    source_artifact_digest: digest(0x82),
                },
            ),
            Err(HardcopyError::SpoolDeviceMismatch)
        ));
    }

    #[test]
    fn browser_print_handoff_is_distinct_from_artifact_export() {
        let mut setup = HardcopySetup::default();
        setup.render = RenderSetup::try_new(
            RenderTarget::BrowserPrintDialog,
            OutputFormat::BrowserPrintDocument,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        )
        .unwrap();
        let plan = HardcopyPlan::compile(
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        let pages = plan.pagination().pages().len() as u32;
        assert!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::BrowserPrintNavigationAccepted {
                    navigation_id: "browser-print-22".to_owned(),
                    pages_accepted: pages,
                    source_artifact_digest: digest(0x90),
                },
            )
            .is_ok()
        );
        let artifact = HardcopyArtifactIdentity::try_new(
            digest(0x91),
            1_000,
            pages,
            OutputFormat::BrowserPrintDocument,
        )
        .unwrap();
        assert!(matches!(
            HardcopyReceipt::record(&plan, HardcopyOutcome::ArtifactExported { artifact },),
            Err(HardcopyError::OutcomeRenderTargetMismatch)
        ));
        assert!(matches!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::BrowserPrintNavigationAccepted {
                    navigation_id: "browser-print-23".to_owned(),
                    pages_accepted: pages + 1,
                    source_artifact_digest: digest(0x92),
                },
            ),
            Err(HardcopyError::AcceptedPageCountMismatch { .. })
        ));
    }

    #[test]
    fn cancellation_and_failure_are_explicit_validated_outcomes() {
        let plan = HardcopyPlan::compile(
            source(),
            HardcopySetup::default(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::Cancelled {
                    phase: CancellationPhase::Rendering,
                    pages_completed: 0,
                    reason: Some("Cancelled by user".to_owned()),
                },
            )
            .is_ok()
        );
        assert!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::Failed {
                    code: HardcopyFailureCode::DeviceUnavailable,
                    message: "Selected printer is offline".to_owned(),
                    pages_completed: 0,
                    retryable: true,
                },
            )
            .is_ok()
        );
        assert!(matches!(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::Failed {
                    code: HardcopyFailureCode::RenderFailure,
                    message: "".to_owned(),
                    pages_completed: 0,
                    retryable: false,
                },
            ),
            Err(HardcopyError::InvalidText { .. })
        ));
    }
}
