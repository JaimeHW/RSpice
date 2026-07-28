//! Physical page setup: paper, margins, orientation, tiling, printer job.
//!
//! Every dimension here is an exact integer [`Length`] in micrometres, never a
//! float, so page arithmetic is reproducible and a margin can never round a
//! tile off the sheet. Tiling derives its grid from the printable area rather
//! than the paper size, which is what keeps a tiled plot's overlap honest on
//! printers with different unprintable borders.

use super::*;

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
    pub(super) name: String,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) display_unit: LengthUnit,
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

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
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
    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
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
    pub(super) paper: PaperSize,
    pub(super) margins: PageMargins,
    pub(super) bleed: Bleed,
    pub(super) orientation: Orientation,
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

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
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
    pub(super) fn validate(self) -> Result<(), HardcopyError> {
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
    pub(super) mode: TilingMode,
    pub(super) overlap: Length,
    pub(super) registration_marks_and_coordinates: bool,
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

    pub(super) fn validate(self) -> Result<(), HardcopyError> {
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
    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
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
    pub(super) physical_width_px: u32,
    pub(super) physical_height_px: u32,
    pub(super) printable_x_px: u32,
    pub(super) printable_y_px: u32,
    pub(super) printable_width_px: u32,
    pub(super) printable_height_px: u32,
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

    pub(super) fn validate(self) -> Result<(), HardcopyError> {
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
    pub(super) capabilities_digest: ContentDigest,
    pub(super) selected_paper_id: String,
    pub(super) raster_geometry: PrinterRasterGeometry,
    pub(super) media_source: PrinterMediaSource,
    pub(super) resolution_dpi: u16,
    pub(super) duplex: DuplexMode,
    pub(super) copies: u16,
    pub(super) collate: bool,
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

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
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
