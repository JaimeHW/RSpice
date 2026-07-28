//! Deterministic hardcopy scene rendering.
//!
//! The renderer is deliberately independent of egui, filesystems, browsers,
//! and printer APIs. Document adapters produce a validated semantic scene in
//! physical micrometres; this module applies the immutable [`HardcopyPlan`]
//! pagination and emits an authenticated artifact. PDF/A output is returned
//! only after Krilla's PDF/A-2b validator accepts the complete document.

use std::fmt::Write as _;
use std::io::Cursor;

use base64::Engine as _;
use krilla::color::rgb;
use krilla::configure::{Archival, ConfigurationBuilder};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::Image;
use krilla::metadata::{DateTime, Metadata, PageLayout};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, LineCap, LineJoin, Stroke, StrokeDash};
use krilla::text::{Font, TextDirection};
use krilla::{Document, SerializeSettings};
mod publication;

pub use publication::*;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use tiff::encoder::{Compression, DeflateLevel, Rational, TiffEncoder, colortype};
use tiff::tags::ResolutionUnit;

use super::sources::{
    HardcopySemanticDocument, ResolvedHardcopyDocument, SCHEMATIC_UNIT_UM, SemanticAggregate,
    SemanticBounds, SemanticPlot, SemanticPoint, SemanticReport, SemanticReportFigure,
    SemanticResultSummary, SemanticSchematic, SemanticTable,
};
use crate::hardcopy::{
    BackgroundMode, Bleed, ColorMapping, ContentExtent, HardcopyArtifactIdentity,
    HardcopyDocumentId, HardcopyPlan, HardcopyPlanId, Length, OutputFormat, PageRect, PreviewPage,
    PrintColor, PrintMappingTable, PrintObjectKind, PrintRedundancy, RenderTarget,
    ResolvedOrientation, ScaleRatio, Watermark,
};
use crate::product::{ContentDigest, ObjectRevision};
use crate::results::report_document::{FigureSizing, ReportBlockId, ReportBlockKind, TableCell};
use crate::schematic::SymbolLibrary;
use crate::schematic::symbols::PathCommand;
use crate::state::{
    Component, DocumentationShapeGeometry, Point as SchematicPoint, SymbolDocument, SymbolShape,
};

const MICROMETRES_PER_INCH: u64 = 25_400;
const POINTS_PER_INCH: f64 = 72.0;
const MAX_SCENE_PRIMITIVES: usize = 1_000_000;
#[cfg(not(target_arch = "wasm32"))]
const MAX_SCENE_VERTICES: usize = 4_000_000;
#[cfg(target_arch = "wasm32")]
const MAX_SCENE_VERTICES: usize = 1_000_000;
const MAX_SCENE_TEXT_BYTES: usize = 16 * 1_048_576;
const MAX_LEGEND_ENTRIES: usize = 4_096;
const MAX_METADATA_LINES: usize = 4_096;
const MAX_PREVIEW_BATCH_PAGES: usize = 2;
// Browser-worker transport is a cross-target contract. Desktop builds retain
// it for shared-source parity even though they use the in-process worker.
const PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION: u32 = 1;
pub(crate) const MAX_PREVIEW_WORKER_MANIFEST_BYTES: usize = 16 * 1_024;
pub(crate) const MAX_PREVIEW_WORKER_RGBA_BYTES: usize = 48 * 1_048_576;
const MAX_PREVIEW_WORKER_TRANSFER_BYTES: usize = 48 * 1_048_576 + 16 * 1_024;
const PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION: u32 = 1;
pub(crate) const MAX_PUBLICATION_WORKER_MANIFEST_BYTES: usize = 4 * 1_048_576;
const MAX_RASTER_PIXELS_PER_PAGE: u64 = 150_000_000;
const MAX_RASTER_PIXELS_TOTAL: u64 = 300_000_000;
pub(crate) const MAX_ARTIFACT_BYTES: usize = 512 * 1_048_576;
#[cfg(not(target_arch = "wasm32"))]
const MAX_EMBEDDED_FIGURE_BYTES: usize = 64 * 1_048_576;
#[cfg(target_arch = "wasm32")]
const MAX_EMBEDDED_FIGURE_BYTES: usize = 16 * 1_048_576;
pub(crate) const MAX_PUBLICATION_BYTES: u64 = 1_024 * 1_048_576;
#[cfg(not(target_arch = "wasm32"))]
const MAX_RENDER_WORK_UNITS: u64 = 32_000_000;
#[cfg(target_arch = "wasm32")]
const MAX_RENDER_WORK_UNITS: u64 = 8_000_000;
#[cfg(not(target_arch = "wasm32"))]
const MAX_ESTIMATED_VECTOR_BYTES: u64 = 512 * 1_048_576;
#[cfg(target_arch = "wasm32")]
const MAX_ESTIMATED_VECTOR_BYTES: u64 = 96 * 1_048_576;
#[cfg(not(target_arch = "wasm32"))]
const MAX_RASTER_WORKING_BYTES: u64 = 1_024 * 1_048_576;
#[cfg(target_arch = "wasm32")]
const MAX_RASTER_WORKING_BYTES: u64 = 256 * 1_048_576;
const DEFAULT_STROKE_UM: u64 = 250;
const DECORATION_TEXT_UM: u64 = 3_000;
const LEGEND_COLUMN_UM: u64 = 64_000;
const LEGEND_ROW_UM: u64 = 5_000;
const LEGEND_VERTICAL_PADDING_UM: u64 = 4_000;

// IBM Plex is licensed for embedding under SIL OFL 1.1. The retained license
// is assets/fonts/OFL-IBMPlex.txt.
const IBM_PLEX_SANS_REGULAR: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexSans-Regular.ttf");
const IBM_PLEX_SANS_SEMIBOLD: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexSans-SemiBold.ttf");
const IBM_PLEX_MONO_REGULAR: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexMono-Regular.ttf");

/// Stable publication timestamp. PDF/A requires this explicit value; the
/// renderer never substitutes wall-clock time because that would make output
/// non-reproducible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardcopyPublicationTimestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl HardcopyPublicationTimestamp {
    /// Convert Unix seconds to a deterministic UTC Gregorian timestamp.
    /// Values after `9999-12-31T23:59:59Z` are rejected instead of wrapping
    /// the PDF metadata year or silently losing precision.
    pub fn from_unix_seconds(value: u64) -> Result<Self, HardcopyRenderError> {
        const MAX_UNIX_SECONDS: u64 = 253_402_300_799;
        if value > MAX_UNIX_SECONDS {
            return Err(HardcopyRenderError::InvalidTimestamp);
        }
        let days = (value / 86_400) as i64;
        let seconds = value % 86_400;
        // Howard Hinnant's checked civil-from-days transform. `days` is
        // already bounded to year 9999, so every intermediate fits i64.
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let mut year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_prime = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
        let month = month_prime + if month_prime < 10 { 3 } else { -9 };
        year += i64::from(month <= 2);
        Self::try_new(
            u16::try_from(year).map_err(|_| HardcopyRenderError::InvalidTimestamp)?,
            u8::try_from(month).map_err(|_| HardcopyRenderError::InvalidTimestamp)?,
            u8::try_from(day).map_err(|_| HardcopyRenderError::InvalidTimestamp)?,
            (seconds / 3_600) as u8,
            ((seconds % 3_600) / 60) as u8,
            (seconds % 60) as u8,
        )
    }

    pub fn try_new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, HardcopyRenderError> {
        let maximum_day =
            days_in_month(year, month).ok_or(HardcopyRenderError::InvalidTimestamp)?;
        if year == 0
            || year > 9_999
            || day == 0
            || day > maximum_day
            || hour > 23
            || minute > 59
            || second > 59
        {
            return Err(HardcopyRenderError::InvalidTimestamp);
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    fn as_krilla(self) -> DateTime {
        DateTime::new(self.year)
            .month(self.month)
            .day(self.day)
            .hour(self.hour)
            .minute(self.minute)
            .second(self.second)
            .utc_offset_hour(0)
            .utc_offset_minute(0)
    }

    #[must_use]
    pub fn to_utc_display(self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

const fn days_in_month(year: u16, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            Some(29)
        }
        2 => Some(28),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardcopySceneMetadata {
    title: String,
    creator: String,
    authors: Vec<String>,
    publication_timestamp: Option<HardcopyPublicationTimestamp>,
    header_lines: Vec<String>,
    provenance_lines: Vec<String>,
}

impl HardcopySceneMetadata {
    pub fn try_new(
        title: impl Into<String>,
        creator: impl Into<String>,
    ) -> Result<Self, HardcopyRenderError> {
        let value = Self {
            title: title.into(),
            creator: creator.into(),
            authors: Vec::new(),
            publication_timestamp: None,
            header_lines: Vec::new(),
            provenance_lines: Vec::new(),
        };
        value.validate()?;
        Ok(value)
    }

    /// Build the required title and revision provenance directly from the
    /// authenticated source. Callers may then add authors and a publication
    /// timestamp without recreating source-identity formatting.
    pub fn for_resolved_source(
        source: &ResolvedHardcopyDocument,
        creator: impl Into<String>,
    ) -> Result<Self, HardcopyRenderError> {
        let mut metadata = Self::try_new(source.authority().display_name(), creator)?;
        metadata.set_provenance_lines(vec![format!(
            "source {} · document {} · revision {} · digest {}",
            source.source_key(),
            source.authority().document_id(),
            source.authority().revision().get(),
            source.authority().content_digest()
        )])?;
        Ok(metadata)
    }

    pub fn set_publication_timestamp(&mut self, value: HardcopyPublicationTimestamp) {
        self.publication_timestamp = Some(value);
    }

    pub fn set_authors(&mut self, authors: Vec<String>) -> Result<(), HardcopyRenderError> {
        let previous = std::mem::replace(&mut self.authors, authors);
        if let Err(error) = self.validate() {
            self.authors = previous;
            return Err(error);
        }
        Ok(())
    }

    pub fn set_header_lines(&mut self, lines: Vec<String>) -> Result<(), HardcopyRenderError> {
        let previous = std::mem::replace(&mut self.header_lines, lines);
        if let Err(error) = self.validate() {
            self.header_lines = previous;
            return Err(error);
        }
        Ok(())
    }

    pub fn set_provenance_lines(&mut self, lines: Vec<String>) -> Result<(), HardcopyRenderError> {
        let previous = std::mem::replace(&mut self.provenance_lines, lines);
        if let Err(error) = self.validate() {
            self.provenance_lines = previous;
            return Err(error);
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), HardcopyRenderError> {
        validate_text("scene title", &self.title, 1_024)?;
        validate_text("scene creator", &self.creator, 512)?;
        validate_lines("authors", &self.authors, 64, 512)?;
        validate_lines(
            "header lines",
            &self.header_lines,
            MAX_METADATA_LINES,
            2_048,
        )?;
        validate_lines(
            "provenance lines",
            &self.provenance_lines,
            MAX_METADATA_LINES,
            4_096,
        )
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn publication_timestamp(&self) -> Option<HardcopyPublicationTimestamp> {
        self.publication_timestamp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenePoint {
    pub x: Length,
    pub y: Length,
}

impl ScenePoint {
    #[must_use]
    pub const fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneRect {
    pub x: Length,
    pub y: Length,
    pub width: Length,
    pub height: Length,
}

impl SceneRect {
    pub fn try_new(
        x: Length,
        y: Length,
        width: Length,
        height: Length,
    ) -> Result<Self, HardcopyRenderError> {
        if width == Length::ZERO || height == Length::ZERO {
            return Err(HardcopyRenderError::EmptyPrimitiveGeometry);
        }
        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb8 {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum SemanticColor {
    Foreground,
    Secondary,
    Grid,
    Accent,
    Warning,
    Success,
    Trace(u16),
    Exact(Rgb8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrokePattern {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrokeStyle {
    pub color: SemanticColor,
    pub width: Length,
    pub pattern: StrokePattern,
    pub series_index: Option<u16>,
    exact_dash: Option<(Length, Length)>,
    exact_dot_spacing: Option<Length>,
}

impl StrokeStyle {
    pub fn try_new(
        color: SemanticColor,
        width: Length,
        pattern: StrokePattern,
        series_index: Option<u16>,
    ) -> Result<Self, HardcopyRenderError> {
        if width == Length::ZERO || width.micrometres() > 100_000 {
            return Err(HardcopyRenderError::InvalidStrokeWidth(width));
        }
        Ok(Self {
            color,
            width,
            pattern,
            series_index,
            exact_dash: None,
            exact_dot_spacing: None,
        })
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            color: SemanticColor::Foreground,
            width: Length::from_micrometres(DEFAULT_STROKE_UM),
            pattern: StrokePattern::Solid,
            series_index: None,
            exact_dash: None,
            exact_dot_spacing: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SceneFont {
    Sans,
    SansSemibold,
    Monospace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SceneFill {
    Solid {
        color: SemanticColor,
    },
    CrossHatch {
        color: SemanticColor,
        line_width: Length,
        spacing: Length,
    },
}

impl SceneFill {
    const fn solid(color: SemanticColor) -> Self {
        Self::Solid { color }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ScenePrimitive {
    Line {
        from: ScenePoint,
        to: ScenePoint,
        stroke: StrokeStyle,
    },
    Polyline {
        points: Vec<ScenePoint>,
        closed: bool,
        stroke: StrokeStyle,
        fill: Option<SceneFill>,
    },
    Rect {
        rect: SceneRect,
        stroke: Option<StrokeStyle>,
        fill: Option<SceneFill>,
    },
    Circle {
        center: ScenePoint,
        radius: Length,
        stroke: Option<StrokeStyle>,
        fill: Option<SceneFill>,
    },
    RasterImage {
        rect: SceneRect,
        png: Vec<u8>,
        content_digest: ContentDigest,
        alternative_text: String,
    },
    Text {
        origin: ScenePoint,
        text: String,
        font: SceneFont,
        size: Length,
        color: SemanticColor,
        anchor: TextAnchor,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegendEntry {
    label: String,
    stroke: StrokeStyle,
    fill: Option<SceneFill>,
}

impl LegendEntry {
    pub fn try_new(
        label: impl Into<String>,
        stroke: StrokeStyle,
    ) -> Result<Self, HardcopyRenderError> {
        let label = label.into();
        validate_text("legend label", &label, 512)?;
        Ok(Self {
            label,
            stroke,
            fill: None,
        })
    }

    fn try_new_with_fill(
        label: impl Into<String>,
        stroke: StrokeStyle,
        fill: Option<SceneFill>,
    ) -> Result<Self, HardcopyRenderError> {
        let mut entry = Self::try_new(label, stroke)?;
        validate_fill(fill)?;
        entry.fill = fill;
        Ok(entry)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AggregateSection {
    ordinal: u32,
    source_key: String,
    display_name: String,
    content_digest: ContentDigest,
    origin: ScenePoint,
    extent: ContentExtent,
    page_break_before: bool,
    primitive_start: usize,
    primitive_end: usize,
}

/// Fully validated source scene. Coordinates are in the same one-to-one
/// micrometre space as the plan's [`ContentExtent`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardcopyScene {
    extent: ContentExtent,
    metadata: HardcopySceneMetadata,
    primitives: Vec<ScenePrimitive>,
    legend: Vec<LegendEntry>,
    aggregate_sections: Vec<AggregateSection>,
}

impl HardcopyScene {
    pub fn try_new(
        extent: ContentExtent,
        metadata: HardcopySceneMetadata,
        primitives: Vec<ScenePrimitive>,
        legend: Vec<LegendEntry>,
    ) -> Result<Self, HardcopyRenderError> {
        let scene = Self {
            extent,
            metadata,
            primitives,
            legend,
            aggregate_sections: Vec::new(),
        };
        scene.validate()?;
        Ok(scene)
    }

    fn validate(&self) -> Result<(), HardcopyRenderError> {
        self.metadata.validate()?;
        let coverage = FontCoverage::load()?;
        coverage.validate_text(
            SceneFont::SansSemibold,
            &self.metadata.title,
            "document title",
        )?;
        for line in &self.metadata.header_lines {
            coverage.validate_text(SceneFont::Sans, line, "page header")?;
        }
        for line in &self.metadata.provenance_lines {
            coverage.validate_text(SceneFont::Monospace, line, "page provenance")?;
        }
        if self.primitives.len() > MAX_SCENE_PRIMITIVES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "scene primitives",
                maximum: MAX_SCENE_PRIMITIVES as u64,
            });
        }
        if self.legend.len() > MAX_LEGEND_ENTRIES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "legend entries",
                maximum: MAX_LEGEND_ENTRIES as u64,
            });
        }
        if self.aggregate_sections.len() > 4_096 {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate sections",
                maximum: 4_096,
            });
        }
        let mut expected_primitive_start = 0_usize;
        for (index, section) in self.aggregate_sections.iter().enumerate() {
            if section.ordinal != index as u32 {
                return Err(conversion_error(
                    "aggregate section identities are not in canonical order",
                ));
            }
            validate_text("aggregate source key", &section.source_key, 1_024)?;
            validate_text("aggregate display name", &section.display_name, 1_024)?;
            coverage.validate_text(
                SceneFont::Sans,
                &section.display_name,
                "aggregate display name",
            )?;
            validate_point(section.origin, self.extent)?;
            let section_right = section
                .origin
                .x
                .micrometres()
                .checked_add(section.extent.width().micrometres())
                .ok_or_else(|| conversion_error("aggregate section X extent overflow"))?;
            let section_bottom = section
                .origin
                .y
                .micrometres()
                .checked_add(section.extent.height().micrometres())
                .ok_or_else(|| conversion_error("aggregate section Y extent overflow"))?;
            if section_right > self.extent.width().micrometres()
                || section_bottom > self.extent.height().micrometres()
            {
                return Err(conversion_error(
                    "aggregate section exceeds the authenticated aggregate extent",
                ));
            }
            if section.primitive_start != expected_primitive_start
                || section.primitive_end < section.primitive_start
                || section.primitive_end > self.primitives.len()
            {
                return Err(conversion_error(
                    "aggregate primitive ownership ranges are not canonical",
                ));
            }
            expected_primitive_start = section.primitive_end;
        }
        if !self.aggregate_sections.is_empty() && expected_primitive_start != self.primitives.len()
        {
            return Err(conversion_error(
                "aggregate primitive ownership does not cover the scene",
            ));
        }
        let mut text_bytes = 0_usize;
        let mut vertices = 0_usize;
        for primitive in &self.primitives {
            validate_primitive(primitive, self.extent, &mut text_bytes, &coverage)?;
            vertices = vertices
                .checked_add(primitive_vertex_count(primitive))
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "scene vertices",
                    maximum: MAX_SCENE_VERTICES as u64,
                })?;
            if vertices > MAX_SCENE_VERTICES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "scene vertices",
                    maximum: MAX_SCENE_VERTICES as u64,
                });
            }
        }
        for entry in &self.legend {
            validate_text("legend label", &entry.label, 512)?;
            coverage.validate_text(SceneFont::Sans, &entry.label, "print legend")?;
            text_bytes = text_bytes.checked_add(entry.label.len()).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "scene text bytes",
                    maximum: MAX_SCENE_TEXT_BYTES as u64,
                },
            )?;
        }
        if text_bytes > MAX_SCENE_TEXT_BYTES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "scene text bytes",
                maximum: MAX_SCENE_TEXT_BYTES as u64,
            });
        }
        Ok(())
    }

    #[must_use]
    pub const fn extent(&self) -> ContentExtent {
        self.extent
    }

    #[must_use]
    pub const fn metadata(&self) -> &HardcopySceneMetadata {
        &self.metadata
    }

    #[must_use]
    pub fn primitives(&self) -> &[ScenePrimitive] {
        &self.primitives
    }

    #[must_use]
    pub fn legend(&self) -> &[LegendEntry] {
        &self.legend
    }
}

struct FontCoverage {
    sans: ttf_parser::Face<'static>,
    semibold: ttf_parser::Face<'static>,
    mono: ttf_parser::Face<'static>,
}

impl FontCoverage {
    fn load() -> Result<Self, HardcopyRenderError> {
        Ok(Self {
            sans: ttf_parser::Face::parse(IBM_PLEX_SANS_REGULAR, 0)
                .map_err(|_| HardcopyRenderError::InvalidEmbeddedFont("IBM Plex Sans Regular"))?,
            semibold: ttf_parser::Face::parse(IBM_PLEX_SANS_SEMIBOLD, 0)
                .map_err(|_| HardcopyRenderError::InvalidEmbeddedFont("IBM Plex Sans Semibold"))?,
            mono: ttf_parser::Face::parse(IBM_PLEX_MONO_REGULAR, 0)
                .map_err(|_| HardcopyRenderError::InvalidEmbeddedFont("IBM Plex Mono Regular"))?,
        })
    }

    fn validate_text(
        &self,
        font: SceneFont,
        text: &str,
        context: &'static str,
    ) -> Result<(), HardcopyRenderError> {
        let face = match font {
            SceneFont::Sans => &self.sans,
            SceneFont::SansSemibold => &self.semibold,
            SceneFont::Monospace => &self.mono,
        };
        for character in text.chars() {
            if face.glyph_index(character).is_none() {
                return Err(HardcopyRenderError::UnsupportedGlyph {
                    codepoint: character as u32,
                    context,
                });
            }
        }
        Ok(())
    }
}

/// Compile one authenticated renderer-neutral document into the canonical
/// physical scene used by every output backend. Signed source coordinates are
/// translated by the source's frozen bounds; they are never rounded through
/// screen pixels or current zoom state.
pub fn scene_from_resolved(
    source: &ResolvedHardcopyDocument,
    mapping: &PrintMappingTable,
    metadata: HardcopySceneMetadata,
) -> Result<HardcopyScene, HardcopyRenderError> {
    let mut compiler =
        SemanticSceneCompiler::new(source.bounds(), source.content_extent(), mapping);
    match source.semantic_document() {
        HardcopySemanticDocument::Schematic(schematic) => compiler.schematic(schematic)?,
        HardcopySemanticDocument::Symbol(symbol) => compiler.symbol_document(symbol, None)?,
        HardcopySemanticDocument::Plot(plot) => compiler.plot(plot)?,
        HardcopySemanticDocument::ResultSummary(summary) => compiler.result_summary(summary)?,
        HardcopySemanticDocument::Report(report) => compiler.report(report)?,
        HardcopySemanticDocument::Aggregate(aggregate) => compiler.aggregate(aggregate)?,
    }
    let scene = HardcopyScene {
        extent: source.content_extent(),
        metadata,
        primitives: compiler.primitives,
        legend: compiler.legend,
        aggregate_sections: compiler.aggregate_sections,
    };
    scene.validate()?;
    Ok(scene)
}

/// Resolve the authored print color to device-independent sRGB. `GrayPercent`
/// is an ink-coverage contract: 70 means 70% black ink (30% reflected gray),
/// not an RGB channel value of 70%.
fn print_color_rgb(color: PrintColor) -> Rgb8 {
    match color {
        PrintColor::Black => Rgb8::new(0, 0, 0),
        PrintColor::GrayPercent(black_percent) => {
            let reflected_percent = 100u16.saturating_sub(u16::from(black_percent));
            let channel = ((reflected_percent * 255) + 50) / 100;
            Rgb8::new(channel as u8, channel as u8, channel as u8)
        }
        PrintColor::Rgb { red, green, blue } => Rgb8::new(red, green, blue),
    }
}

struct SemanticSceneCompiler<'a> {
    bounds: SemanticBounds,
    extent: ContentExtent,
    mapping: &'a PrintMappingTable,
    primitives: Vec<ScenePrimitive>,
    legend: Vec<LegendEntry>,
    coordinate_offset: ScenePoint,
    aggregate_sections: Vec<AggregateSection>,
    mapping_ordinal: Option<u32>,
}

impl<'a> SemanticSceneCompiler<'a> {
    fn new(bounds: SemanticBounds, extent: ContentExtent, mapping: &'a PrintMappingTable) -> Self {
        Self {
            bounds,
            extent,
            mapping,
            primitives: Vec::new(),
            legend: Vec::new(),
            coordinate_offset: ScenePoint::new(Length::ZERO, Length::ZERO),
            aggregate_sections: Vec::new(),
            mapping_ordinal: None,
        }
    }

    fn semantic_point(&self, point: SemanticPoint) -> Result<ScenePoint, HardcopyRenderError> {
        self.signed_micrometre_point(point.x_um, point.y_um)
    }

    fn schematic_point(&self, point: SchematicPoint) -> Result<ScenePoint, HardcopyRenderError> {
        let x = i64::from(point.x)
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("schematic X coordinate overflow"))?;
        let y = i64::from(point.y)
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("schematic Y coordinate overflow"))?;
        self.signed_micrometre_point(x, y)
    }

    fn signed_micrometre_point(&self, x: i64, y: i64) -> Result<ScenePoint, HardcopyRenderError> {
        let x = x
            .checked_sub(self.bounds.minimum.x_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| conversion_error("source X coordinate precedes frozen bounds"))?;
        let y = y
            .checked_sub(self.bounds.minimum.y_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| conversion_error("source Y coordinate precedes frozen bounds"))?;
        let x = x
            .checked_add(self.coordinate_offset.x.micrometres())
            .ok_or_else(|| conversion_error("aggregate X placement overflow"))?;
        let y = y
            .checked_add(self.coordinate_offset.y.micrometres())
            .ok_or_else(|| conversion_error("aggregate Y placement overflow"))?;
        if x > self.extent.width().micrometres() || y > self.extent.height().micrometres() {
            return Err(conversion_error(
                "source coordinate exceeds its authenticated physical bounds",
            ));
        }
        Ok(ScenePoint::new(
            Length::from_micrometres(x),
            Length::from_micrometres(y),
        ))
    }

    fn compile_document(
        &mut self,
        document: &HardcopySemanticDocument,
    ) -> Result<(), HardcopyRenderError> {
        match document {
            HardcopySemanticDocument::Schematic(schematic) => self.schematic(schematic),
            HardcopySemanticDocument::Symbol(symbol) => self.symbol_document(symbol, None),
            HardcopySemanticDocument::Plot(plot) => self.plot(plot),
            HardcopySemanticDocument::ResultSummary(summary) => self.result_summary(summary),
            HardcopySemanticDocument::Report(report) => self.report(report),
            HardcopySemanticDocument::Aggregate(_) => Err(conversion_error(
                "nested semantic aggregates are not supported",
            )),
        }
    }

    fn aggregate(&mut self, aggregate: &SemanticAggregate) -> Result<(), HardcopyRenderError> {
        if aggregate.children.is_empty() || aggregate.children.len() > 4_096 {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate children",
                maximum: 4_096,
            });
        }
        for (index, child) in aggregate.children.iter().enumerate() {
            let expected_ordinal =
                u32::try_from(index).map_err(|_| conversion_error("aggregate ordinal overflow"))?;
            if child.ordinal != expected_ordinal {
                return Err(conversion_error(
                    "aggregate children are not in canonical ordinal order",
                ));
            }
            let offset_x = child
                .placement_origin
                .x_um
                .checked_sub(self.bounds.minimum.x_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child X placement precedes bounds"))?;
            let offset_y = child
                .placement_origin
                .y_um
                .checked_sub(self.bounds.minimum.y_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child Y placement precedes bounds"))?;
            let mut compiler = Self {
                bounds: child.local_bounds,
                extent: self.extent,
                mapping: self.mapping,
                primitives: Vec::new(),
                legend: Vec::new(),
                coordinate_offset: ScenePoint::new(
                    Length::from_micrometres(offset_x),
                    Length::from_micrometres(offset_y),
                ),
                aggregate_sections: Vec::new(),
                mapping_ordinal: Some(child.ordinal),
            };
            compiler.compile_document(&child.document)?;
            let primitive_start = self.primitives.len();
            let primitive_end = primitive_start
                .checked_add(compiler.primitives.len())
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "scene primitives",
                    maximum: MAX_SCENE_PRIMITIVES as u64,
                })?;
            let width = child
                .local_bounds
                .maximum
                .x_um
                .checked_sub(child.local_bounds.minimum.x_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child width is invalid"))?;
            let height = child
                .local_bounds
                .maximum
                .y_um
                .checked_sub(child.local_bounds.minimum.y_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child height is invalid"))?;
            let extent = ContentExtent::try_new(
                Length::from_micrometres(width),
                Length::from_micrometres(height),
            )
            .map_err(|error| conversion_error(error.to_string()))?;
            self.aggregate_sections.push(AggregateSection {
                ordinal: child.ordinal,
                source_key: child.source_key.clone(),
                display_name: child.display_name.clone(),
                content_digest: child.content_digest,
                origin: compiler.coordinate_offset,
                extent,
                page_break_before: child.page_break_before,
                primitive_start,
                primitive_end,
            });
            self.primitives.extend(compiler.primitives);
            for legend in compiler.legend {
                if !self.legend.iter().any(|existing| existing == &legend) {
                    self.legend.push(legend);
                }
            }
        }
        Ok(())
    }

    fn schematic_polyline(
        &self,
        points: &[SchematicPoint],
    ) -> Result<Vec<ScenePoint>, HardcopyRenderError> {
        points
            .iter()
            .copied()
            .map(|point| self.schematic_point(point))
            .collect()
    }

    fn mapped_stroke(
        &self,
        kind: PrintObjectKind,
        stable_id: &str,
        fallback: StrokeStyle,
    ) -> StrokeStyle {
        let mapped_id = self.mapping_stable_id(stable_id);
        let Some(entry) =
            self.mapping.entries().iter().find(|entry| {
                entry.object().kind() == kind && entry.object().stable_id() == mapped_id
            })
        else {
            return fallback;
        };
        let color = SemanticColor::Exact(print_color_rgb(entry.print_color()));
        let (width, pattern, exact_dash, exact_dot_spacing) = match entry.redundancy() {
            PrintRedundancy::SolidLine { width } => (width, StrokePattern::Solid, None, None),
            PrintRedundancy::DashedLine { width, dash, gap } => {
                (width, StrokePattern::Dashed, Some((dash, gap)), None)
            }
            PrintRedundancy::DottedLeader { width, spacing } => {
                (width, StrokePattern::Dotted, None, Some(spacing))
            }
            PrintRedundancy::CrossHatch { line_width, .. } => {
                (line_width, StrokePattern::Solid, None, None)
            }
            PrintRedundancy::TriangleWithId { .. }
            | PrintRedundancy::SolidFill
            | PrintRedundancy::SourceStyle => (
                fallback.width,
                fallback.pattern,
                fallback.exact_dash,
                fallback.exact_dot_spacing,
            ),
        };
        StrokeStyle {
            color,
            width,
            pattern,
            series_index: fallback.series_index,
            exact_dash,
            exact_dot_spacing,
        }
    }

    fn mapped_redundancy(&self, kind: PrintObjectKind, stable_id: &str) -> Option<PrintRedundancy> {
        let mapped_id = self.mapping_stable_id(stable_id);
        self.mapping
            .entries()
            .iter()
            .find(|entry| entry.object().kind() == kind && entry.object().stable_id() == mapped_id)
            .map(|entry| entry.redundancy())
    }

    fn mapped_fill(
        &self,
        kind: PrintObjectKind,
        stable_id: &str,
        fallback: Option<SceneFill>,
    ) -> Option<SceneFill> {
        let mapped_id = self.mapping_stable_id(stable_id);
        let entry = self.mapping.entries().iter().find(|entry| {
            entry.object().kind() == kind && entry.object().stable_id() == mapped_id
        })?;
        let color = SemanticColor::Exact(print_color_rgb(entry.print_color()));
        match entry.redundancy() {
            PrintRedundancy::SolidFill => Some(SceneFill::solid(color)),
            PrintRedundancy::CrossHatch {
                line_width,
                spacing,
            } => Some(SceneFill::CrossHatch {
                color,
                line_width,
                spacing,
            }),
            _ => fallback,
        }
    }

    fn add_mapping_legend(
        &mut self,
        kind: PrintObjectKind,
        stable_id: &str,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let mapped_id = self.mapping_stable_id(stable_id);
        let Some(entry) = self.mapping.entries().iter().find(|entry| {
            entry.object().kind() == kind
                && entry.object().stable_id() == mapped_id
                && entry.include_in_legend()
        }) else {
            return Ok(());
        };
        if self
            .legend
            .iter()
            .any(|existing| existing.label == entry.object().display_name())
        {
            return Ok(());
        }
        let display_name = entry.object().display_name().to_owned();
        let fill = self.mapped_fill(kind, stable_id, None);
        self.legend
            .push(LegendEntry::try_new_with_fill(display_name, stroke, fill)?);
        Ok(())
    }

    fn mapping_stable_id(&self, stable_id: &str) -> String {
        let Some(ordinal) = self.mapping_ordinal else {
            return stable_id.to_owned();
        };
        let stable_digest = Sha256::digest(
            [
                b"rspice-aggregate-print-object-v1:".as_slice(),
                &ordinal.to_be_bytes(),
                stable_id.as_bytes(),
            ]
            .concat(),
        );
        let stable_suffix = stable_digest[..12]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        format!("aggregate:{ordinal}:{stable_suffix}")
    }

    fn schematic(&mut self, schematic: &SemanticSchematic) -> Result<(), HardcopyRenderError> {
        let component_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            StrokeStyle::default(),
        );
        let component_fill = self.mapped_fill(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            Some(SceneFill::solid(component_stroke.color)),
        );
        let wire_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-wiring",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(250),
                StrokePattern::Solid,
                None,
            )?,
        );
        let bus_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-buses",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(600),
                StrokePattern::Solid,
                None,
            )?,
        );
        for wire in &schematic.wires {
            if wire.points.len() >= 2 {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&wire.points)?,
                    closed: false,
                    stroke: wire_stroke,
                    fill: None,
                });
            }
        }
        for bus in &schematic.buses {
            self.primitives.push(ScenePrimitive::Polyline {
                points: self.schematic_polyline(&bus.points)?,
                closed: false,
                stroke: bus_stroke,
                fill: None,
            });
        }
        for tap in &schematic.bus_taps {
            self.primitives.push(ScenePrimitive::Line {
                from: self.schematic_point(tap.bus_point)?,
                to: self.schematic_point(tap.connection_point)?,
                stroke: bus_stroke,
            });
        }
        for junction in &schematic.junctions {
            self.primitives.push(ScenePrimitive::Circle {
                center: self.schematic_point(junction.pos)?,
                radius: Length::from_micrometres(900),
                stroke: None,
                fill: self.mapped_fill(
                    PrintObjectKind::Layer,
                    "layer:schematic-wiring",
                    Some(SceneFill::solid(wire_stroke.color)),
                ),
            });
        }
        for label in &schematic.net_labels {
            self.add_text(
                self.schematic_point(label.pos)?,
                &label.name,
                SceneFont::SansSemibold,
                2_800,
                wire_stroke.color,
            )?;
        }

        let library = SymbolLibrary::load_embedded()
            .map_err(|error| conversion_error(format!("embedded symbol library: {error}")))?;
        for semantic in &schematic.components {
            if let Some(symbol) = &semantic.resolved_symbol {
                self.symbol_document(symbol, Some(&semantic.component))?;
            } else {
                self.library_component(
                    &library,
                    &semantic.component,
                    component_stroke,
                    component_fill,
                )?;
            }
            let anchor = self.schematic_point(semantic.component.pos)?;
            self.add_text(
                anchor,
                &semantic.component.name,
                SceneFont::SansSemibold,
                2_700,
                component_stroke.color,
            )?;
            if !semantic.component.value.is_empty() {
                let value_anchor = self.offset_scene_point(anchor, 0, 3_500)?;
                self.add_text(
                    value_anchor,
                    &semantic.component.value,
                    SceneFont::Sans,
                    2_500,
                    component_stroke.color,
                )?;
            }
        }

        let annotation_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-annotation",
            StrokeStyle::try_new(
                SemanticColor::Secondary,
                Length::from_micrometres(200),
                StrokePattern::Dotted,
                None,
            )?,
        );
        for note in &schematic.design_notes {
            let mut origin = self.schematic_point(note.pos)?;
            for line in note.text.lines() {
                self.add_text(
                    origin,
                    line,
                    SceneFont::Sans,
                    2_800,
                    annotation_stroke.color,
                )?;
                origin = self.offset_scene_point(origin, 0, 3_400)?;
            }
        }
        let documentation_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-documentation",
            StrokeStyle::try_new(
                SemanticColor::Secondary,
                Length::from_micrometres(220),
                StrokePattern::Solid,
                None,
            )?,
        );
        for shape in &schematic.documentation_shapes {
            self.documentation_shape(&shape.geometry, documentation_stroke)?;
        }
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            component_stroke,
        )?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:schematic-wiring",
            wire_stroke,
        )?;
        self.add_mapping_legend(PrintObjectKind::Layer, "layer:schematic-buses", bus_stroke)?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:drawing-annotation",
            annotation_stroke,
        )?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:drawing-documentation",
            documentation_stroke,
        )?;
        Ok(())
    }

    fn offset_scene_point(
        &self,
        point: ScenePoint,
        dx: i64,
        dy: i64,
    ) -> Result<ScenePoint, HardcopyRenderError> {
        let axis = |value: u64, delta: i64, maximum: u64| {
            let adjusted = i128::from(value) + i128::from(delta);
            if adjusted < 0 || adjusted > i128::from(maximum) {
                None
            } else {
                Some(adjusted as u64)
            }
        };
        Ok(ScenePoint::new(
            Length::from_micrometres(
                axis(point.x.micrometres(), dx, self.extent.width().micrometres())
                    .ok_or_else(|| conversion_error("text anchor exceeds source bounds"))?,
            ),
            Length::from_micrometres(
                axis(
                    point.y.micrometres(),
                    dy,
                    self.extent.height().micrometres(),
                )
                .ok_or_else(|| conversion_error("text anchor exceeds source bounds"))?,
            ),
        ))
    }

    fn add_text(
        &mut self,
        origin: ScenePoint,
        text: &str,
        font: SceneFont,
        size_um: u64,
        color: SemanticColor,
    ) -> Result<(), HardcopyRenderError> {
        let normalized = text.replace(['\r', '\n', '\t'], " ");
        if normalized.trim().is_empty() {
            return Ok(());
        }
        validate_text("semantic source text", &normalized, 65_536)?;
        self.primitives.push(ScenePrimitive::Text {
            origin,
            text: normalized,
            font,
            size: Length::from_micrometres(size_um),
            color,
            anchor: TextAnchor::Start,
        });
        Ok(())
    }

    fn symbol_document(
        &mut self,
        symbol: &SymbolDocument,
        component: Option<&Component>,
    ) -> Result<(), HardcopyRenderError> {
        let body_stable_id = if component.is_some() {
            "layer:schematic-components"
        } else {
            "layer:symbol-body"
        };
        let pin_stable_id = if component.is_some() {
            "layer:schematic-components"
        } else {
            "layer:symbol-pins"
        };
        let body_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            body_stable_id,
            StrokeStyle::default(),
        );
        let pin_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            pin_stable_id,
            StrokeStyle::try_new(
                SemanticColor::Accent,
                Length::from_micrometres(220),
                StrokePattern::Solid,
                None,
            )?,
        );
        let convert = |this: &Self, point: SchematicPoint| {
            let world = if let Some(component) = component {
                let local = component.transform_point(point);
                SchematicPoint::new(
                    component.pos.x.saturating_add(local.x),
                    component.pos.y.saturating_add(local.y),
                )
            } else {
                point
            };
            this.schematic_point(world)
        };
        for shape in &symbol.body {
            match shape {
                SymbolShape::Polyline { points, closed } => {
                    if points.len() >= if *closed { 3 } else { 2 } {
                        self.primitives.push(ScenePrimitive::Polyline {
                            points: points
                                .iter()
                                .copied()
                                .map(|point| convert(self, point))
                                .collect::<Result<_, _>>()?,
                            closed: *closed,
                            stroke: body_stroke,
                            fill: None,
                        });
                    }
                }
                SymbolShape::Circle { center, radius } => {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: convert(self, *center)?,
                        radius: Length::from_micrometres(
                            u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        ),
                        stroke: Some(body_stroke),
                        fill: None,
                    });
                }
                SymbolShape::Dot { center, radius } => {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: convert(self, *center)?,
                        radius: Length::from_micrometres(
                            u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        ),
                        stroke: None,
                        fill: self.mapped_fill(
                            PrintObjectKind::Layer,
                            body_stable_id,
                            Some(SceneFill::solid(body_stroke.color)),
                        ),
                    });
                }
                SymbolShape::Arc {
                    center,
                    radius,
                    start_degrees,
                    sweep_degrees,
                } => {
                    let count = (sweep_degrees.unsigned_abs() / 5).clamp(8, 144) as usize;
                    let mut points = Vec::with_capacity(count + 1);
                    for index in 0..=count {
                        let angle = f64::from(*start_degrees)
                            + f64::from(*sweep_degrees) * index as f64 / count as f64;
                        let radians = angle.to_radians();
                        points.push(
                            SchematicPoint::new(
                                center.x.saturating_add(
                                    (f64::from(*radius) * radians.cos()).round() as i32,
                                ),
                                center.y.saturating_add(
                                    (f64::from(*radius) * radians.sin()).round() as i32,
                                ),
                            ),
                        );
                    }
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: points
                            .into_iter()
                            .map(|point| convert(self, point))
                            .collect::<Result<_, _>>()?,
                        closed: false,
                        stroke: body_stroke,
                        fill: None,
                    });
                }
                SymbolShape::Arrow {
                    tip,
                    rotation_quarters,
                } => {
                    let direction = rotation_quarters.rem_euclid(4);
                    let mut local = [
                        SchematicPoint::new(0, 0),
                        SchematicPoint::new(-8, -4),
                        SchematicPoint::new(-8, 4),
                    ];
                    for point in &mut local {
                        for _ in 0..direction {
                            *point = SchematicPoint::new(-point.y, point.x);
                        }
                        point.x = point.x.saturating_add(tip.x);
                        point.y = point.y.saturating_add(tip.y);
                    }
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: local
                            .into_iter()
                            .map(|point| convert(self, point))
                            .collect::<Result<_, _>>()?,
                        closed: true,
                        stroke: body_stroke,
                        fill: self.mapped_fill(
                            PrintObjectKind::Layer,
                            body_stable_id,
                            Some(SceneFill::solid(body_stroke.color)),
                        ),
                    });
                }
            }
        }
        for pin in &symbol.pins {
            let Some(position) = pin.position else {
                continue;
            };
            let position = convert(self, position)?;
            self.primitives.push(ScenePrimitive::Circle {
                center: position,
                radius: Length::from_micrometres(500),
                stroke: Some(pin_stroke),
                fill: None,
            });
            if component.is_none() {
                let label = self.offset_scene_point(position, 1_000, -800)?;
                self.add_text(label, &pin.name, SceneFont::Sans, 2_300, pin_stroke.color)?;
            }
        }
        self.add_mapping_legend(PrintObjectKind::Layer, body_stable_id, body_stroke)?;
        if pin_stable_id != body_stable_id {
            self.add_mapping_legend(PrintObjectKind::Layer, pin_stable_id, pin_stroke)?;
        }
        Ok(())
    }

    fn library_component(
        &mut self,
        library: &SymbolLibrary,
        component: &Component,
        stroke: StrokeStyle,
        mapped_fill: Option<SceneFill>,
    ) -> Result<(), HardcopyRenderError> {
        let (symbol, rotation) = library
            .get_with_rotation_variant(
                component.kind,
                component.rotation.degrees(),
                component.symbol_variant.as_deref(),
            )
            .ok_or_else(|| {
                conversion_error(format!(
                    "no production symbol is registered for {:?}",
                    component.kind
                ))
            })?;
        let (cx, cy) = symbol.center();
        let scale_x = f64::from(symbol.target_width / symbol.width().max(0.001));
        let scale_y = f64::from(symbol.target_height / symbol.height().max(0.001));
        let radians = f64::from(rotation).to_radians();
        let (cosine, sine) = (radians.cos(), radians.sin());
        let transform = |x: f32, y: f32| {
            let mut x = (f64::from(x) - f64::from(cx)) * scale_x;
            let mut y = (f64::from(y) - f64::from(cy)) * scale_y;
            if component.mirror_h {
                x = -x;
            }
            if component.mirror_v {
                y = -y;
            }
            let rotated_x = x * cosine - y * sine;
            let rotated_y = x * sine + y * cosine;
            SchematicPoint::new(
                component.pos.x.saturating_add(rotated_x.round() as i32),
                component.pos.y.saturating_add(rotated_y.round() as i32),
            )
        };
        for path in &symbol.paths {
            let mut points = Vec::<SchematicPoint>::new();
            let mut current = (0.0_f32, 0.0_f32);
            let flush = |this: &mut Self,
                         points: &mut Vec<SchematicPoint>|
             -> Result<(), HardcopyRenderError> {
                if points.len() >= 2 {
                    let closed = points.first() == points.last() && points.len() >= 4;
                    if closed {
                        points.pop();
                    }
                    this.primitives.push(ScenePrimitive::Polyline {
                        points: points
                            .drain(..)
                            .map(|point| this.schematic_point(point))
                            .collect::<Result<_, _>>()?,
                        closed,
                        stroke,
                        fill: if path.filled { mapped_fill } else { None },
                    });
                } else {
                    points.clear();
                }
                Ok(())
            };
            for command in &path.commands {
                match command {
                    PathCommand::MoveTo(x, y) => {
                        flush(self, &mut points)?;
                        points.push(transform(*x, *y));
                        current = (*x, *y);
                    }
                    PathCommand::LineTo(x, y) => {
                        points.push(transform(*x, *y));
                        current = (*x, *y);
                    }
                    PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                        for index in 1..=16 {
                            let t = index as f32 / 16.0;
                            let one_minus_t = 1.0 - t;
                            let x = one_minus_t.powi(3) * current.0
                                + 3.0 * one_minus_t.powi(2) * t * ctrl1.0
                                + 3.0 * one_minus_t * t.powi(2) * ctrl2.0
                                + t.powi(3) * end.0;
                            let y = one_minus_t.powi(3) * current.1
                                + 3.0 * one_minus_t.powi(2) * t * ctrl1.1
                                + 3.0 * one_minus_t * t.powi(2) * ctrl2.1
                                + t.powi(3) * end.1;
                            points.push(transform(x, y));
                        }
                        current = *end;
                    }
                    PathCommand::Close => {
                        if let Some(first) = points.first().copied() {
                            points.push(first);
                        }
                    }
                }
            }
            flush(self, &mut points)?;
        }
        Ok(())
    }

    fn documentation_shape(
        &mut self,
        geometry: &DocumentationShapeGeometry,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        match geometry {
            DocumentationShapeGeometry::Rectangle { first, opposite } => {
                let first = self.schematic_point(*first)?;
                let opposite = self.schematic_point(*opposite)?;
                let x = first.x.min(opposite.x);
                let y = first.y.min(opposite.y);
                let width = Length::from_micrometres(
                    first.x.micrometres().abs_diff(opposite.x.micrometres()),
                );
                let height = Length::from_micrometres(
                    first.y.micrometres().abs_diff(opposite.y.micrometres()),
                );
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(x, y, width, height)?,
                    stroke: Some(stroke),
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Line { start, end } => {
                self.primitives.push(ScenePrimitive::Line {
                    from: self.schematic_point(*start)?,
                    to: self.schematic_point(*end)?,
                    stroke,
                });
            }
            DocumentationShapeGeometry::Polygon { points } => {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(points)?,
                    closed: true,
                    stroke,
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Arc {
                start,
                through,
                end,
            } => {
                let points = circular_arc_points(*start, *through, *end)?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&points)?,
                    closed: false,
                    stroke,
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Callout {
                tip,
                elbow,
                box_corner,
            } => {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&[*tip, *elbow])?,
                    closed: false,
                    stroke,
                    fill: None,
                });
                let first = self.schematic_point(*elbow)?;
                let opposite = self.schematic_point(*box_corner)?;
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(
                        first.x.min(opposite.x),
                        first.y.min(opposite.y),
                        Length::from_micrometres(
                            first.x.micrometres().abs_diff(opposite.x.micrometres()),
                        ),
                        Length::from_micrometres(
                            first.y.micrometres().abs_diff(opposite.y.micrometres()),
                        ),
                    )?,
                    stroke: Some(stroke),
                    fill: None,
                });
            }
        }
        Ok(())
    }

    fn plot(&mut self, plot: &SemanticPlot) -> Result<(), HardcopyRenderError> {
        let frame_stroke = StrokeStyle::try_new(
            SemanticColor::Secondary,
            Length::from_micrometres(220),
            StrokePattern::Solid,
            None,
        )?;
        let frame = SceneRect::try_new(
            Length::from_micrometres(12_700),
            Length::from_micrometres(12_700),
            Length::from_micrometres(self.extent.width().micrometres().saturating_sub(25_400)),
            Length::from_micrometres(self.extent.height().micrometres().saturating_sub(25_400)),
        )?;
        self.primitives.push(ScenePrimitive::Rect {
            rect: frame,
            stroke: Some(frame_stroke),
            fill: None,
        });
        for division in 1..10_u64 {
            let x = self.extent.width().micrometres() * division / 10;
            self.primitives.push(ScenePrimitive::Line {
                from: ScenePoint::new(Length::from_micrometres(x), frame.y),
                to: ScenePoint::new(
                    Length::from_micrometres(x),
                    Length::from_micrometres(frame.y.micrometres() + frame.height.micrometres()),
                ),
                stroke: StrokeStyle::try_new(
                    SemanticColor::Grid,
                    Length::from_micrometres(120),
                    StrokePattern::Dotted,
                    None,
                )?,
            });
        }
        for (index, trace) in plot.traces.iter().enumerate() {
            let stable_id = format!("trace:{}", trace.trace_id);
            let stroke = self.mapped_stroke(
                PrintObjectKind::Trace,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Trace(index as u16),
                    Length::from_micrometres(300),
                    auto_trace_pattern(index as u16),
                    Some(index as u16),
                )?,
            );
            for path in &trace.paths {
                if path.len() >= 2 {
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: path
                            .iter()
                            .copied()
                            .map(|point| self.semantic_point(point))
                            .collect::<Result<_, _>>()?,
                        closed: false,
                        stroke,
                        fill: None,
                    });
                }
            }
            self.add_mapping_legend(PrintObjectKind::Trace, &stable_id, stroke)?;
            let mapped_id = self.mapping_stable_id(&stable_id);
            if !self.mapping.entries().iter().any(|entry| {
                entry.object().kind() == PrintObjectKind::Trace
                    && entry.object().stable_id() == mapped_id
            }) {
                self.legend
                    .push(LegendEntry::try_new(&trace.label, stroke)?);
            }
        }
        for marker in &plot.markers {
            let position = marker.position.ok_or_else(|| {
                conversion_error(format!(
                    "plot marker {} has no authenticated physical position",
                    marker.marker_id
                ))
            })?;
            let center = self.semantic_point(position)?;
            let stable_id = format!("marker:{}", marker.marker_id);
            let size = match self.mapped_redundancy(PrintObjectKind::Marker, &stable_id) {
                Some(PrintRedundancy::TriangleWithId { size }) => i64::try_from(size.micrometres())
                    .map_err(|_| conversion_error("mapped marker size exceeds signed geometry"))?,
                _ => 2_500_i64,
            };
            let points = [
                self.offset_scene_point(center, 0, -size)?,
                self.offset_scene_point(center, -size, size)?,
                self.offset_scene_point(center, size, size)?,
            ];
            let stroke = self.mapped_stroke(
                PrintObjectKind::Marker,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Accent,
                    Length::from_micrometres(250),
                    StrokePattern::Solid,
                    None,
                )?,
            );
            self.primitives.push(ScenePrimitive::Polyline {
                points: points.to_vec(),
                closed: true,
                stroke,
                fill: self.mapped_fill(
                    PrintObjectKind::Marker,
                    &stable_id,
                    Some(SceneFill::solid(stroke.color)),
                ),
            });
            let label = self.offset_scene_point(center, size + 800, -size)?;
            let mapped_marker_id = marker.marker_id.to_string();
            self.add_text(
                label,
                if matches!(
                    self.mapped_redundancy(PrintObjectKind::Marker, &stable_id),
                    Some(PrintRedundancy::TriangleWithId { .. })
                ) {
                    &mapped_marker_id
                } else {
                    &marker.label
                },
                SceneFont::Monospace,
                2_300,
                stroke.color,
            )?;
            self.add_mapping_legend(PrintObjectKind::Marker, &stable_id, stroke)?;
        }
        for annotation in &plot.annotations {
            let position = annotation.position.ok_or_else(|| {
                conversion_error(format!(
                    "plot annotation {} has no authenticated physical position",
                    annotation.annotation_id
                ))
            })?;
            let origin = self.semantic_point(position)?;
            let label = self.offset_scene_point(origin, 4_000, -4_000)?;
            let stable_id = format!("annotation:{}", annotation.annotation_id);
            let stroke = self.mapped_stroke(
                PrintObjectKind::ReviewAnnotation,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Secondary,
                    Length::from_micrometres(200),
                    StrokePattern::Dotted,
                    None,
                )?,
            );
            self.primitives.push(ScenePrimitive::Line {
                from: origin,
                to: label,
                stroke,
            });
            self.add_text(
                label,
                &annotation.text,
                SceneFont::Sans,
                2_400,
                stroke.color,
            )?;
        }
        Ok(())
    }

    fn result_summary(
        &mut self,
        summary: &SemanticResultSummary,
    ) -> Result<(), HardcopyRenderError> {
        let stable_id = format!(
            "layer:result-summary:{}",
            summary.viewer.label().to_ascii_lowercase()
        );
        let stroke = self.mapped_stroke(PrintObjectKind::Layer, &stable_id, StrokeStyle::default());
        let mut y = 12_000_u64;
        self.add_text(
            ScenePoint::new(
                Length::from_micrometres(12_000),
                Length::from_micrometres(y),
            ),
            &summary.title,
            SceneFont::SansSemibold,
            4_500,
            stroke.color,
        )?;
        y += 9_000;
        for table in &summary.tables {
            y = self.semantic_table(table, y, stroke)?;
        }
        self.add_mapping_legend(PrintObjectKind::Layer, &stable_id, stroke)?;
        Ok(())
    }

    fn semantic_table(
        &mut self,
        table: &SemanticTable,
        mut y: u64,
        stroke: StrokeStyle,
    ) -> Result<u64, HardcopyRenderError> {
        let left = 12_000_u64;
        let right = self.extent.width().micrometres().saturating_sub(12_000);
        let columns = table.columns.len().max(1) as u64;
        let column_width = right.saturating_sub(left) / columns;
        self.add_text(
            ScenePoint::new(Length::from_micrometres(left), Length::from_micrometres(y)),
            &table.title,
            SceneFont::SansSemibold,
            3_200,
            stroke.color,
        )?;
        y += 5_000;
        for (column, heading) in table.columns.iter().enumerate() {
            self.primitives.push(ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(left + column as u64 * column_width),
                    Length::from_micrometres(y),
                    Length::from_micrometres(column_width),
                    Length::from_micrometres(4_500),
                )?,
                stroke: Some(stroke),
                fill: None,
            });
            self.add_text(
                ScenePoint::new(
                    Length::from_micrometres(left + column as u64 * column_width + 800),
                    Length::from_micrometres(y + 3_000),
                ),
                heading,
                SceneFont::SansSemibold,
                2_300,
                stroke.color,
            )?;
        }
        y += 4_500;
        for row in &table.rows {
            if y + 4_500 > self.extent.height().micrometres().saturating_sub(8_000) {
                return Err(conversion_error(format!(
                    "table '{}' does not fit its authenticated result page",
                    table.title
                )));
            }
            for (column, value) in row.iter().enumerate() {
                if column >= columns as usize {
                    break;
                }
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(left + column as u64 * column_width),
                        Length::from_micrometres(y),
                        Length::from_micrometres(column_width),
                        Length::from_micrometres(4_500),
                    )?,
                    stroke: Some(stroke),
                    fill: None,
                });
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(left + column as u64 * column_width + 800),
                        Length::from_micrometres(y + 3_000),
                    ),
                    value,
                    SceneFont::Monospace,
                    2_100,
                    stroke.color,
                )?;
            }
            y += 4_500;
        }
        Ok(y + 5_000)
    }

    fn report(&mut self, report: &SemanticReport) -> Result<(), HardcopyRenderError> {
        const PAGE_HEIGHT: u64 = 279_400;
        const PAGE_GAP: u64 = 5_000;
        let stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:report-content",
            StrokeStyle::default(),
        );
        for (page_index, page) in report.pages.iter().enumerate() {
            let page_top = page_index as u64 * (PAGE_HEIGHT + PAGE_GAP);
            let page_bottom = page_top + PAGE_HEIGHT;
            self.primitives.push(ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(0),
                    Length::from_micrometres(page_top),
                    Length::from_micrometres(self.extent.width().micrometres()),
                    Length::from_micrometres(PAGE_HEIGHT),
                )?,
                stroke: Some(stroke),
                fill: None,
            });
            let mut y = page_top + 16_000;
            self.add_text(
                ScenePoint::new(
                    Length::from_micrometres(16_000),
                    Length::from_micrometres(y),
                ),
                page.title(),
                SceneFont::SansSemibold,
                5_000,
                stroke.color,
            )?;
            y += 9_000;
            for section in page.sections() {
                self.ensure_report_room(y, 7_000, page_bottom, page.title())?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(16_000),
                        Length::from_micrometres(y),
                    ),
                    section.title(),
                    SceneFont::SansSemibold,
                    3_800,
                    stroke.color,
                )?;
                y += 7_000;
                for block in section.blocks() {
                    if let Some(reference) = block.kind().reference() {
                        let authenticated = report
                            .authenticated_references
                            .iter()
                            .find(|candidate| candidate.block_id == block.id())
                            .ok_or(HardcopyRenderError::UnauthenticatedReportReference)?;
                        if &authenticated.reference != reference {
                            return Err(HardcopyRenderError::UnauthenticatedReportReference);
                        }
                    }
                    y = self.report_block(
                        block.id(),
                        block.kind(),
                        report
                            .figures
                            .iter()
                            .find(|figure| figure.block_id == block.id()),
                        y,
                        page_bottom,
                        stroke,
                        page.title(),
                    )?;
                }
            }
        }
        self.add_mapping_legend(PrintObjectKind::Layer, "layer:report-content", stroke)?;
        Ok(())
    }

    fn ensure_report_room(
        &self,
        y: u64,
        required: u64,
        page_bottom: u64,
        title: &str,
    ) -> Result<(), HardcopyRenderError> {
        if y.saturating_add(required) > page_bottom.saturating_sub(12_000) {
            Err(conversion_error(format!(
                "authored report page '{title}' overflows its physical page"
            )))
        } else {
            Ok(())
        }
    }

    fn report_block(
        &mut self,
        block_id: ReportBlockId,
        block: &ReportBlockKind,
        figure: Option<&SemanticReportFigure>,
        mut y: u64,
        page_bottom: u64,
        stroke: StrokeStyle,
        page_title: &str,
    ) -> Result<u64, HardcopyRenderError> {
        let mut lines = Vec::<String>::new();
        match block {
            ReportBlockKind::PlotFigure(value) => {
                let figure =
                    figure.ok_or(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                        "unresolved plot figure",
                    ))?;
                if figure.media_type != "image/png"
                    || figure.caption != value.caption
                    || figure.alternative_text != value.alternative_text
                    || figure.sizing != value.sizing
                {
                    return Err(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                        "mismatched plot figure",
                    ));
                }
                let (pixel_width, pixel_height) = png_dimensions(&figure.payload)?;
                if pixel_width != figure.width_pixels || pixel_height != figure.height_pixels {
                    return Err(HardcopyRenderError::InvalidEmbeddedFigure(
                        "authenticated PNG dimensions changed after source resolution".to_owned(),
                    ));
                }
                self.ensure_report_room(y, 5_500, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &value.caption,
                    SceneFont::SansSemibold,
                    2_900,
                    stroke.color,
                )?;
                y += 5_500;
                let maximum_width = self
                    .extent
                    .width()
                    .micrometres()
                    .checked_sub(40_000)
                    .ok_or_else(|| conversion_error("report figure page is too narrow"))?;
                let maximum_height = page_bottom.saturating_sub(18_000).saturating_sub(y);
                let natural_width = u64::from(pixel_width)
                    .checked_mul(MICROMETRES_PER_INCH)
                    .ok_or_else(|| conversion_error("report figure width overflow"))?
                    .div_ceil(96);
                let natural_height = u64::from(pixel_height)
                    .checked_mul(MICROMETRES_PER_INCH)
                    .ok_or_else(|| conversion_error("report figure height overflow"))?
                    .div_ceil(96);
                let scale = match value.sizing {
                    FigureSizing::Natural => 1.0_f64
                        .min(maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64),
                    FigureSizing::FitWidth => (maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64),
                    FigureSizing::FitPage => (maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64)
                        .min(1.0),
                };
                if !scale.is_finite() || scale <= 0.0 {
                    return Err(conversion_error(format!(
                        "authenticated plot figure {block_id} cannot fit its authored page"
                    )));
                }
                let width = (natural_width as f64 * scale).floor() as u64;
                let height = (natural_height as f64 * scale).floor() as u64;
                self.ensure_report_room(y, height.saturating_add(7_000), page_bottom, page_title)?;
                self.primitives.push(ScenePrimitive::RasterImage {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                        Length::from_micrometres(width),
                        Length::from_micrometres(height),
                    )?,
                    png: figure.payload.clone(),
                    content_digest: figure.artifact_digest,
                    alternative_text: value.alternative_text.clone(),
                });
                y = y.saturating_add(height).saturating_add(4_000);
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &value.alternative_text,
                    SceneFont::Sans,
                    2_200,
                    stroke.color,
                )?;
                return Ok(y + 5_000);
            }
            ReportBlockKind::DataTable(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: value
                        .columns
                        .iter()
                        .map(|column| column.heading.clone())
                        .collect(),
                    rows: value
                        .rows
                        .iter()
                        .map(|row| row.iter().map(format_table_cell).collect())
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report table '{}' overflows authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Datasheet(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec!["Field".to_owned(), "Value".to_owned()],
                    rows: value
                        .fields
                        .iter()
                        .map(|field| {
                            vec![
                                field.label.clone(),
                                format!(
                                    "{}{}",
                                    field.value,
                                    field
                                        .unit
                                        .as_deref()
                                        .map(|unit| format!(" {unit}"))
                                        .unwrap_or_default()
                                ),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report datasheet '{}' overflows authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Requirements(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec![
                        "Requirement".to_owned(),
                        "Statement".to_owned(),
                        "Disposition".to_owned(),
                        "Evidence".to_owned(),
                    ],
                    rows: value
                        .entries
                        .iter()
                        .map(|entry| {
                            vec![
                                entry.requirement_id.clone(),
                                entry.statement.clone(),
                                format!("{:?}", entry.disposition),
                                entry.evidence_label.clone().unwrap_or_default(),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report requirements '{}' overflow authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Specifications(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec![
                        "Expression".to_owned(),
                        "Limit".to_owned(),
                        "Measured".to_owned(),
                        "Disposition".to_owned(),
                    ],
                    rows: value
                        .entries
                        .iter()
                        .map(|entry| {
                            vec![
                                entry.expression.clone(),
                                entry.limit.clone(),
                                entry.measured.clone().unwrap_or_else(|| "—".to_owned()),
                                format!("{:?}", entry.disposition),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report specifications '{}' overflow authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Prose(value) => {
                return self.report_markdown(&value.markdown, y, page_bottom, stroke, page_title);
            }
            ReportBlockKind::ReviewNote(value) => {
                lines.push(format!("Review — {} [{:?}]", value.author, value.status));
                lines.extend(value.message.lines().map(str::to_owned));
            }
            ReportBlockKind::Evidence(_) => {
                return Err(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                    "evidence",
                ));
            }
        }
        for line in lines {
            for wrapped in wrap_text(&line, 112) {
                self.ensure_report_room(y, 4_200, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &wrapped,
                    SceneFont::Sans,
                    2_500,
                    stroke.color,
                )?;
                y += 4_200;
            }
        }
        Ok(y + 3_000)
    }

    fn report_markdown(
        &mut self,
        markdown: &str,
        mut y: u64,
        page_bottom: u64,
        stroke: StrokeStyle,
        page_title: &str,
    ) -> Result<u64, HardcopyRenderError> {
        let mut in_code_block = false;
        let mut paragraph = String::new();
        let flush_paragraph = |this: &mut Self,
                               paragraph: &mut String,
                               y: &mut u64|
         -> Result<(), HardcopyRenderError> {
            if paragraph.is_empty() {
                return Ok(());
            }
            let (text, font) = parse_supported_inline_markdown(paragraph)?;
            for wrapped in wrap_text(&text, 112) {
                this.ensure_report_room(*y, 4_200, page_bottom, page_title)?;
                this.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(*y),
                    ),
                    &wrapped,
                    font,
                    2_500,
                    stroke.color,
                )?;
                *y += 4_200;
            }
            *y += 1_500;
            paragraph.clear();
            Ok(())
        };
        for raw in markdown.lines() {
            let line = raw.trim_end();
            if line.trim() == "```" {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                self.ensure_report_room(y, 4_000, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(22_000),
                        Length::from_micrometres(y),
                    ),
                    if line.is_empty() { " " } else { line },
                    SceneFont::Monospace,
                    2_300,
                    stroke.color,
                )?;
                y += 4_000;
                continue;
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                continue;
            }
            if trimmed.starts_with('>')
                || trimmed.starts_with("![")
                || trimmed.contains("](")
                || trimmed.starts_with('<')
                || trimmed.matches('|').count() >= 2
            {
                return Err(HardcopyRenderError::UnsupportedReportMarkdown);
            }
            let heading =
                ["#### ", "### ", "## ", "# "]
                    .iter()
                    .enumerate()
                    .find_map(|(index, prefix)| {
                        trimmed
                            .strip_prefix(prefix)
                            .map(|text| (text, 3_000 + (3 - index as u64) * 350))
                    });
            if let Some((text, size)) = heading {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                let (text, _) = parse_supported_inline_markdown(text)?;
                self.ensure_report_room(y, 5_500, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &text,
                    SceneFont::SansSemibold,
                    size,
                    stroke.color,
                )?;
                y += 5_500;
                continue;
            }
            let unordered = trimmed
                .strip_prefix("- ")
                .or_else(|| trimmed.strip_prefix("* "));
            let ordered = trimmed
                .find(". ")
                .filter(|index| *index > 0 && trimmed[..*index].chars().all(|c| c.is_ascii_digit()))
                .map(|index| (&trimmed[..index + 1], &trimmed[index + 2..]));
            if unordered.is_some() || ordered.is_some() {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                let (prefix, content) = match (unordered, ordered) {
                    (Some(content), _) => ("•".to_owned(), content),
                    (_, Some((number, content))) => (number.to_owned(), content),
                    _ => unreachable!(),
                };
                let (content, font) = parse_supported_inline_markdown(content)?;
                for wrapped in wrap_text(&format!("{prefix} {content}"), 106) {
                    self.ensure_report_room(y, 4_200, page_bottom, page_title)?;
                    self.add_text(
                        ScenePoint::new(
                            Length::from_micrometres(24_000),
                            Length::from_micrometres(y),
                        ),
                        &wrapped,
                        font,
                        2_500,
                        stroke.color,
                    )?;
                    y += 4_200;
                }
                continue;
            }
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(trimmed);
        }
        if in_code_block {
            return Err(HardcopyRenderError::UnsupportedReportMarkdown);
        }
        flush_paragraph(self, &mut paragraph, &mut y)?;
        Ok(y + 3_000)
    }
}

fn parse_supported_inline_markdown(
    value: &str,
) -> Result<(String, SceneFont), HardcopyRenderError> {
    let bold_markers = value.matches("**").count();
    let code_markers = value.matches('`').count();
    if !bold_markers.is_multiple_of(2)
        || !code_markers.is_multiple_of(2)
        || value.contains("~~")
        || value.contains("__")
    {
        return Err(HardcopyRenderError::UnsupportedReportMarkdown);
    }
    let font = if code_markers > 0 {
        SceneFont::Monospace
    } else if bold_markers > 0 {
        SceneFont::SansSemibold
    } else {
        SceneFont::Sans
    };
    Ok((value.replace("**", "").replace('`', ""), font))
}

fn conversion_error(message: impl Into<String>) -> HardcopyRenderError {
    HardcopyRenderError::SourceConversion(message.into())
}

fn format_table_cell(cell: &TableCell) -> String {
    match cell {
        TableCell::Empty => String::new(),
        TableCell::Text(value) => value.clone(),
        TableCell::Number { value, unit } => match unit {
            Some(unit) => format!("{value:.12} {unit}"),
            None => format!("{value:.12}"),
        },
        TableCell::Integer(value) => value.to_string(),
        TableCell::Boolean(value) => value.to_string(),
    }
}

fn wrap_text(text: &str, maximum_chars: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if word.chars().count() > maximum_chars {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
            let mut chunk = String::new();
            for character in word.chars() {
                chunk.push(character);
                if chunk.chars().count() == maximum_chars {
                    lines.push(std::mem::take(&mut chunk));
                }
            }
            current = chunk;
            continue;
        }
        if !current.is_empty() && current.chars().count() + 1 + word.chars().count() > maximum_chars
        {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn circular_arc_points(
    start: SchematicPoint,
    through: SchematicPoint,
    end: SchematicPoint,
) -> Result<Vec<SchematicPoint>, HardcopyRenderError> {
    let (x1, y1) = (f64::from(start.x), f64::from(start.y));
    let (x2, y2) = (f64::from(through.x), f64::from(through.y));
    let (x3, y3) = (f64::from(end.x), f64::from(end.y));
    let determinant = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    if determinant.abs() < f64::EPSILON {
        return Err(conversion_error("documentation arc is collinear"));
    }
    let cx = ((x1 * x1 + y1 * y1) * (y2 - y3)
        + (x2 * x2 + y2 * y2) * (y3 - y1)
        + (x3 * x3 + y3 * y3) * (y1 - y2))
        / determinant;
    let cy = ((x1 * x1 + y1 * y1) * (x3 - x2)
        + (x2 * x2 + y2 * y2) * (x1 - x3)
        + (x3 * x3 + y3 * y3) * (x2 - x1))
        / determinant;
    let radius = (x1 - cx).hypot(y1 - cy);
    let start_angle = (y1 - cy).atan2(x1 - cx);
    let through_angle = (y2 - cy).atan2(x2 - cx);
    let end_angle = (y3 - cy).atan2(x3 - cx);
    let normalize = |angle: f64| angle.rem_euclid(std::f64::consts::TAU);
    let ccw_sweep = normalize(end_angle - start_angle);
    let through_ccw = normalize(through_angle - start_angle);
    let sweep = if through_ccw <= ccw_sweep {
        ccw_sweep
    } else {
        ccw_sweep - std::f64::consts::TAU
    };
    let segments = ((sweep.abs() * radius / 2.0).ceil() as usize).clamp(8, 256);
    Ok((0..=segments)
        .map(|index| {
            let angle = start_angle + sweep * index as f64 / segments as f64;
            SchematicPoint::new(
                (cx + radius * angle.cos()).round() as i32,
                (cy + radius * angle.sin()).round() as i32,
            )
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use lopdf::Document as ParsedPdf;
    use uuid::Uuid;

    use super::*;
    use crate::hardcopy::sources::HardcopySourceIdentity;
    use crate::hardcopy::{
        ActiveHardcopySource, DecorationSetup, DuplexMode, FontPolicy, HardcopyContentSection,
        HardcopyDocumentKind, HardcopyScope, HardcopySetup, Orientation, PageMargins, PaperSize,
        PhysicalPageSetup, PrintMappingEntry, PrintMappingSaveScope, PrintMappingTable,
        PrintObjectIdentity, PrinterJobSettings, PrinterMediaSource, RenderSetup, ScaleMode,
        StandardPaper, TilingMode, TilingSetup,
    };
    use crate::workbench::hardcopy_adapters::sources::{
        SymbolHardcopySource, resolve_symbol_source,
    };

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn reseal_preview_worker_manifest(manifest: &mut PreviewWorkerManifest) -> Vec<u8> {
        let material = PreviewWorkerManifestMaterial {
            schema_version: manifest.schema_version,
            plan_id: manifest.plan_id,
            plan_digest: manifest.plan_digest,
            source_document_id: manifest.source_document_id,
            source_revision: manifest.source_revision,
            source_digest: manifest.source_digest,
            zero_based_page: manifest.zero_based_page,
            page_number: manifest.page_number,
            coordinate: &manifest.coordinate,
            width: manifest.width,
            height: manifest.height,
            dpi: manifest.dpi,
            soft_proof_applied: manifest.soft_proof_applied,
            rgba_byte_length: manifest.rgba_byte_length,
            rgba_digest: manifest.rgba_digest,
            preview_digest: manifest.preview_digest,
        };
        manifest.transport_digest = preview_worker_material_digest(&material).unwrap();
        serde_json::to_vec(manifest).unwrap()
    }

    fn reseal_publication_worker_manifest(manifest: &mut PublicationWorkerManifest) -> Vec<u8> {
        let material = PublicationWorkerManifestMaterial {
            schema_version: manifest.schema_version,
            plan_digest: manifest.plan_digest,
            source_digest: manifest.source_digest,
            publication_digest: manifest.publication_digest,
            format: manifest.format,
            page_count: manifest.page_count,
            pdf_conformance: manifest.pdf_conformance,
            parts: &manifest.parts,
        };
        manifest.transport_digest = publication_worker_manifest_digest(&material).unwrap();
        serde_json::to_vec(manifest).unwrap()
    }

    fn extent(width_um: u64, height_um: u64) -> ContentExtent {
        ContentExtent::try_new(
            Length::from_micrometres(width_um),
            Length::from_micrometres(height_um),
        )
        .unwrap()
    }

    fn source() -> ActiveHardcopySource {
        ActiveHardcopySource::try_new(
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(1)).unwrap(),
            ObjectRevision::INITIAL,
            digest(0x33),
            "top · schematic",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .unwrap()
    }

    fn setup(format: OutputFormat, tiled: bool) -> HardcopySetup {
        let target = match format {
            OutputFormat::NativePrinter => RenderTarget::SystemPrinter {
                printer_id: "test-printer".to_owned(),
                job: PrinterJobSettings::try_new(
                    digest(0x44),
                    "paper-letter",
                    crate::hardcopy::PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612)
                        .unwrap(),
                    PrinterMediaSource::AutomaticCompatibleTray,
                    72,
                    DuplexMode::Off,
                    1,
                    false,
                )
                .unwrap(),
            },
            OutputFormat::BrowserPrintDocument => RenderTarget::BrowserPrintDialog,
            _ => RenderTarget::ExportArtifact,
        };
        let fonts = FontPolicy::new(format.is_vector(), format.is_vector());
        HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins::uniform(Length::from_micrometres(10_000)),
                Bleed::None,
                Orientation::Landscape,
            )
            .unwrap(),
            if tiled {
                ScaleMode::EngineeringOneToOne
            } else {
                ScaleMode::FitPrintableArea
            },
            TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
            RenderSetup::try_new(
                target,
                format,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                fonts,
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(true, true, true, Watermark::Draft).unwrap(),
            PrintMappingTable::try_new(
                PrintMappingSaveScope::Document,
                vec![
                    PrintMappingEntry::try_new(
                        PrintObjectIdentity::try_new(
                            PrintObjectKind::Trace,
                            "trace:test",
                            "V(out)",
                            "blue solid",
                        )
                        .unwrap(),
                        PrintColor::Black,
                        PrintRedundancy::SolidLine {
                            width: Length::from_micrometres(300),
                        },
                        true,
                    )
                    .unwrap(),
                ],
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn plan(format: OutputFormat, content: ContentExtent, tiled: bool) -> HardcopyPlan {
        HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(2)).unwrap(),
            source(),
            setup(format, tiled),
            content,
        )
        .unwrap()
    }

    fn aggregate_plan_and_scene(format: OutputFormat) -> (HardcopyPlan, HardcopyScene) {
        let target = match format {
            OutputFormat::NativePrinter => RenderTarget::SystemPrinter {
                printer_id: "test-printer".to_owned(),
                job: PrinterJobSettings::try_new(
                    digest(0x44),
                    "paper-letter",
                    crate::hardcopy::PrinterRasterGeometry::try_new(792, 612, 0, 0, 792, 612)
                        .unwrap(),
                    PrinterMediaSource::AutomaticCompatibleTray,
                    72,
                    DuplexMode::Off,
                    1,
                    false,
                )
                .unwrap(),
            },
            _ => RenderTarget::ExportArtifact,
        };
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
                target,
                format,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(format.is_vector(), format.is_vector()),
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
            PrintMappingTable::default(),
        )
        .unwrap();
        let portrait_extent = extent(100_000, 200_000);
        let landscape_extent = extent(200_000, 100_000);
        let aggregate_extent = extent(200_000, 305_000);
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
        let plan = HardcopyPlan::compile_with_id_and_sections(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(0x29)).unwrap(),
            source(),
            setup,
            aggregate_extent,
            sections,
        )
        .unwrap();
        let text = |origin, value: &str| ScenePrimitive::Text {
            origin,
            text: value.to_owned(),
            font: SceneFont::Sans,
            size: Length::from_micrometres(4_000),
            color: SemanticColor::Foreground,
            anchor: TextAnchor::Start,
        };
        let scene = HardcopyScene {
            extent: aggregate_extent,
            metadata: HardcopySceneMetadata::try_new("Mixed pages", "RSpice tests").unwrap(),
            primitives: vec![
                text(
                    ScenePoint::new(
                        Length::from_micrometres(5_000),
                        Length::from_micrometres(10_000),
                    ),
                    "PORTRAIT_ONLY",
                ),
                text(
                    ScenePoint::new(
                        Length::from_micrometres(5_000),
                        Length::from_micrometres(215_000),
                    ),
                    "LANDSCAPE_ONLY",
                ),
            ],
            legend: Vec::new(),
            aggregate_sections: vec![
                AggregateSection {
                    ordinal: 0,
                    source_key: "child:portrait".to_owned(),
                    display_name: "Portrait child".to_owned(),
                    content_digest: digest(0x10),
                    origin: ScenePoint::new(Length::ZERO, Length::ZERO),
                    extent: portrait_extent,
                    page_break_before: false,
                    primitive_start: 0,
                    primitive_end: 1,
                },
                AggregateSection {
                    ordinal: 1,
                    source_key: "child:landscape".to_owned(),
                    display_name: "Landscape child".to_owned(),
                    content_digest: digest(0x11),
                    origin: ScenePoint::new(Length::ZERO, Length::from_micrometres(205_000)),
                    extent: landscape_extent,
                    page_break_before: true,
                    primitive_start: 1,
                    primitive_end: 2,
                },
            ],
        };
        scene.validate().unwrap();
        (plan, scene)
    }

    fn scene(content: ContentExtent) -> HardcopyScene {
        let mut metadata = HardcopySceneMetadata::try_new(
            "Precision sensor & verification",
            "RSpice hardcopy tests",
        )
        .unwrap();
        metadata.set_publication_timestamp(
            HardcopyPublicationTimestamp::try_new(2026, 7, 22, 12, 30, 0).unwrap(),
        );
        metadata
            .set_provenance_lines(vec!["retained run 17 · model digest 9f00".to_owned()])
            .unwrap();
        let trace = StrokeStyle::try_new(
            SemanticColor::Trace(1),
            Length::from_micrometres(350),
            StrokePattern::Solid,
            Some(1),
        )
        .unwrap();
        HardcopyScene::try_new(
            content,
            metadata,
            vec![
                ScenePrimitive::Line {
                    from: ScenePoint::new(
                        Length::from_micrometres(5_000),
                        Length::from_micrometres(5_000),
                    ),
                    to: ScenePoint::new(
                        Length::from_micrometres(content.width().micrometres() - 5_000),
                        Length::from_micrometres(content.height().micrometres() - 5_000),
                    ),
                    stroke: trace,
                },
                ScenePrimitive::Text {
                    origin: ScenePoint::new(
                        Length::from_micrometres(10_000),
                        Length::from_micrometres(20_000),
                    ),
                    text: "V(out) < 1.2 V".to_owned(),
                    font: SceneFont::Monospace,
                    size: Length::from_micrometres(4_000),
                    color: SemanticColor::Foreground,
                    anchor: TextAnchor::Start,
                },
            ],
            vec![LegendEntry::try_new("V(out)", trace).unwrap()],
        )
        .unwrap()
    }

    fn resolved_symbol() -> ResolvedHardcopyDocument {
        let document = SymbolDocument {
            pins: Vec::new(),
            body: vec![SymbolShape::Polyline {
                points: vec![
                    SchematicPoint::new(-20, -15),
                    SchematicPoint::new(20, -15),
                    SchematicPoint::new(20, 15),
                    SchematicPoint::new(-20, 15),
                ],
                closed: true,
            }],
            origin: SchematicPoint::origin(),
            name_anchor: SchematicPoint::new(-20, -25),
            value_anchor: SchematicPoint::new(-20, 25),
        };
        resolve_symbol_source(SymbolHardcopySource {
            identity: HardcopySourceIdentity::try_new(
                "test-symbol",
                HardcopyDocumentId::try_from_uuid(Uuid::from_u128(77)).unwrap(),
                ObjectRevision::INITIAL,
                "Comparator symbol",
            )
            .unwrap(),
            document: &document,
            selection: None,
            scope: HardcopyScope::ActiveDocument,
        })
        .unwrap()
    }

    fn resolved_wide_symbol() -> ResolvedHardcopyDocument {
        let document = SymbolDocument {
            pins: Vec::new(),
            body: vec![SymbolShape::Polyline {
                points: vec![
                    SchematicPoint::new(0, 0),
                    SchematicPoint::new(2_000, 0),
                    SchematicPoint::new(2_000, 100),
                    SchematicPoint::new(0, 100),
                ],
                closed: true,
            }],
            origin: SchematicPoint::origin(),
            name_anchor: SchematicPoint::origin(),
            value_anchor: SchematicPoint::origin(),
        };
        resolve_symbol_source(SymbolHardcopySource {
            identity: HardcopySourceIdentity::try_new(
                "test-wide-symbol",
                HardcopyDocumentId::try_from_uuid(Uuid::from_u128(79)).unwrap(),
                ObjectRevision::INITIAL,
                "Wide comparator symbol",
            )
            .unwrap(),
            document: &document,
            selection: None,
            scope: HardcopyScope::ActiveDocument,
        })
        .unwrap()
    }

    fn plan_for_resolved(source: &ResolvedHardcopyDocument, format: OutputFormat) -> HardcopyPlan {
        HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(78)).unwrap(),
            source.authority().clone(),
            setup(format, false),
            source.content_extent(),
        )
        .unwrap()
    }

    fn resolved_metadata() -> HardcopySceneMetadata {
        HardcopySceneMetadata::try_new("Comparator symbol", "RSpice hardcopy tests").unwrap()
    }

    #[test]
    fn scene_validation_rejects_geometry_outside_declared_extent() {
        let content = extent(100_000, 60_000);
        let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
        assert!(matches!(
            HardcopyScene::try_new(
                content,
                metadata,
                vec![ScenePrimitive::Line {
                    from: ScenePoint::new(Length::ZERO, Length::ZERO),
                    to: ScenePoint::new(
                        Length::from_micrometres(100_001),
                        Length::from_micrometres(1)
                    ),
                    stroke: StrokeStyle::default(),
                }],
                Vec::new(),
            ),
            Err(HardcopyRenderError::PrimitiveOutsideExtent)
        ));
    }

    #[test]
    fn embedded_fonts_cover_engineering_symbols_and_reject_notdef() {
        let coverage = FontCoverage::load().unwrap();
        let glyphs = "Ω µ μ Δ ≥ ≤ ± ° × · √";
        coverage
            .validate_text(SceneFont::Sans, glyphs, "engineering glyph")
            .unwrap();
        for font in [SceneFont::Monospace] {
            for glyph in glyphs
                .chars()
                .filter(|character| !character.is_whitespace())
            {
                match coverage.validate_text(font, &glyph.to_string(), "engineering glyph") {
                    Ok(()) => {}
                    Err(HardcopyRenderError::UnsupportedGlyph { codepoint, context }) => {
                        assert_eq!(codepoint, glyph as u32);
                        assert_eq!(context, "engineering glyph");
                    }
                    Err(error) => panic!("unexpected font validation failure: {error}"),
                }
            }
        }
        for font in [SceneFont::Sans, SceneFont::Monospace] {
            assert!(matches!(
                coverage.validate_text(font, "∠", "engineering angle"),
                Err(HardcopyRenderError::UnsupportedGlyph {
                    codepoint: 0x2220,
                    context: "engineering angle"
                })
            ));
        }
        assert!(matches!(
            coverage.validate_text(SceneFont::Sans, "\u{10ffff}", "test"),
            Err(HardcopyRenderError::UnsupportedGlyph {
                codepoint: 0x10ffff,
                context: "test"
            })
        ));
    }

    #[test]
    fn exact_cross_hatch_is_shared_by_svg_pdf_and_raster_paths() {
        let content = extent(100_000, 60_000);
        let metadata = HardcopySceneMetadata::try_new("hatch test", "RSpice").unwrap();
        let fill = SceneFill::CrossHatch {
            color: SemanticColor::Foreground,
            line_width: Length::from_micrometres(275),
            spacing: Length::from_micrometres(2_750),
        };
        let scene = HardcopyScene::try_new(
            content,
            metadata,
            vec![ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(5_000),
                    Length::from_micrometres(5_000),
                    Length::from_micrometres(30_000),
                    Length::from_micrometres(20_000),
                )
                .unwrap(),
                stroke: Some(StrokeStyle::default()),
                fill: Some(fill),
            }],
            Vec::new(),
        )
        .unwrap();
        let svg_plan = plan(OutputFormat::SvgVector, content, false);
        let svg_publication = HardcopyRenderer::render(&svg_plan, &scene).unwrap();
        let svg = std::str::from_utf8(svg_publication.single_part().unwrap().bytes()).unwrap();
        assert!(svg.contains("stroke-width=\"275\""));
        assert!(svg.contains("-275-2750"));
        assert!(svg.contains("fill=\"url(#hatch-"));

        let pdf_plan = plan(OutputFormat::PdfVector, content, false);
        let pdf = HardcopyRenderer::render(&pdf_plan, &scene).unwrap();
        ParsedPdf::load_mem(pdf.single_part().unwrap().bytes()).unwrap();

        let png_plan = plan(OutputFormat::Png { dpi: 72 }, content, false);
        let png = HardcopyRenderer::render(&png_plan, &scene).unwrap();
        let decoder = png::Decoder::new(Cursor::new(png.single_part().unwrap().bytes()));
        decoder.read_info().unwrap();
    }

    #[test]
    fn printer_working_set_accounts_for_retained_rgba_and_spool_conversion() {
        let letter_1200_dpi_pixels = 13_200_u64 * 10_200;
        assert!(matches!(
            validate_printer_raster_working_set(letter_1200_dpi_pixels, 1),
            Err(HardcopyRenderError::ResourceLimit {
                scope: "raster working-set bytes",
                ..
            })
        ));
    }

    #[test]
    fn aggregate_mapping_namespaces_colliding_child_object_identities() {
        let bounds = SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(100_000, 100_000),
        )
        .unwrap();
        let content = extent(100_000, 100_000);
        let empty = PrintMappingTable::default();
        let mut first_identity = SemanticSceneCompiler::new(bounds, content, &empty);
        first_identity.mapping_ordinal = Some(0);
        let mut second_identity = SemanticSceneCompiler::new(bounds, content, &empty);
        second_identity.mapping_ordinal = Some(1);
        let first_id = first_identity.mapping_stable_id("trace:shared");
        let second_id = second_identity.mapping_stable_id("trace:shared");
        assert_ne!(first_id, second_id);

        let mapping = PrintMappingTable::try_new(
            PrintMappingSaveScope::Document,
            vec![
                PrintMappingEntry::try_new(
                    PrintObjectIdentity::try_new(
                        PrintObjectKind::Trace,
                        first_id,
                        "Child 1 shared trace",
                        "red",
                    )
                    .unwrap(),
                    PrintColor::Rgb {
                        red: 220,
                        green: 20,
                        blue: 20,
                    },
                    PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(300),
                    },
                    true,
                )
                .unwrap(),
                PrintMappingEntry::try_new(
                    PrintObjectIdentity::try_new(
                        PrintObjectKind::Trace,
                        second_id,
                        "Child 2 shared trace",
                        "blue",
                    )
                    .unwrap(),
                    PrintColor::Rgb {
                        red: 20,
                        green: 20,
                        blue: 220,
                    },
                    PrintRedundancy::SolidLine {
                        width: Length::from_micrometres(300),
                    },
                    true,
                )
                .unwrap(),
            ],
        )
        .unwrap();
        let fallback = StrokeStyle::default();
        let mut first = SemanticSceneCompiler::new(bounds, content, &mapping);
        first.mapping_ordinal = Some(0);
        let mut second = SemanticSceneCompiler::new(bounds, content, &mapping);
        second.mapping_ordinal = Some(1);
        assert_eq!(
            first
                .mapped_stroke(PrintObjectKind::Trace, "trace:shared", fallback)
                .color,
            SemanticColor::Exact(Rgb8::new(220, 20, 20))
        );
        assert_eq!(
            second
                .mapped_stroke(PrintObjectKind::Trace, "trace:shared", fallback)
                .color,
            SemanticColor::Exact(Rgb8::new(20, 20, 220))
        );
    }

    #[test]
    fn unix_timestamp_conversion_is_checked_and_gregorian() {
        assert_eq!(
            HardcopyPublicationTimestamp::from_unix_seconds(0).unwrap(),
            HardcopyPublicationTimestamp::try_new(1970, 1, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(
            HardcopyPublicationTimestamp::from_unix_seconds(951_782_400).unwrap(),
            HardcopyPublicationTimestamp::try_new(2000, 2, 29, 0, 0, 0).unwrap()
        );
        assert_eq!(
            HardcopyPublicationTimestamp::from_unix_seconds(253_402_300_799).unwrap(),
            HardcopyPublicationTimestamp::try_new(9999, 12, 31, 23, 59, 59).unwrap()
        );
        assert!(matches!(
            HardcopyPublicationTimestamp::from_unix_seconds(253_402_300_800),
            Err(HardcopyRenderError::InvalidTimestamp)
        ));
    }

    #[test]
    fn svg_is_deterministic_searchable_embedded_and_xml_escaped() {
        let content = extent(100_000, 60_000);
        let plan = plan(OutputFormat::SvgVector, content, false);
        let scene = scene(content);
        let first = HardcopyRenderer::render(&plan, &scene).unwrap();
        let second = HardcopyRenderer::render(&plan, &scene).unwrap();
        assert_eq!(first, second);
        let part = first.single_part().unwrap();
        let svg = std::str::from_utf8(part.bytes()).unwrap();
        assert!(svg.contains("@font-face"));
        assert!(svg.contains("Precision sensor &amp; verification"));
        assert!(svg.contains("V(out) &lt; 1.2 V"));
        assert_eq!(part.media_type(), "image/svg+xml");
        assert_eq!(first.identity().unwrap().page_count(), 1);
    }

    #[test]
    fn tiled_svg_is_a_complete_numbered_multi_part_publication() {
        let content = extent(500_000, 100_000);
        let plan = plan(OutputFormat::SvgVector, content, true);
        assert!(plan.pagination().pages().len() > 1);
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        assert_eq!(publication.parts().len(), plan.pagination().pages().len());
        assert_eq!(publication.page_count(), publication.parts().len() as u32);
        for (index, part) in publication.parts().iter().enumerate() {
            assert_eq!(part.first_page(), index as u32 + 1);
            assert!(
                part.suggested_filename()
                    .starts_with(&format!("page-{:04}", index + 1))
            );
            assert_eq!(part.page_count(), 1);
        }
    }

    #[test]
    fn browser_print_document_is_self_contained_paginated_and_digest_bound() {
        let content = extent(500_000, 100_000);
        let plan = plan(OutputFormat::BrowserPrintDocument, content, true);
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        let part = publication.single_part().unwrap();
        assert_eq!(part.media_type(), "text/html");
        assert_eq!(part.filename_extension(), "html");
        let html = std::str::from_utf8(part.bytes()).unwrap();
        assert!(html.contains("@page rspice-page-1{size:279.400mm 215.900mm;margin:0;}"));
        assert!(html.contains(
            ".rspice-print-page[data-page=\"1\"]{page:rspice-page-1;width:279.400mm;height:215.900mm}"
        ));
        assert!(html.contains(&format!(
            "name=\"rspice-plan-digest\" content=\"{}\"",
            plan.content_digest()
        )));
        assert!(html.contains(&format!(
            "name=\"rspice-source-digest\" content=\"{}\"",
            plan.source().content_digest()
        )));
        assert_eq!(
            html.matches("class=\"rspice-print-page\"").count(),
            plan.pagination().pages().len()
        );
        assert!(html.contains("@font-face"));
        assert!(html.contains("V(out) &lt; 1.2 V"));
        assert!(!html.contains("script-src"));
        assert!(!html.contains("<script"));
    }

    #[test]
    fn aggregate_pages_render_independently_with_mixed_orientations() {
        let (plan, scene) = aggregate_plan_and_scene(OutputFormat::SvgVector);
        let publication = HardcopyRenderer::render(&plan, &scene).unwrap();
        assert_eq!(publication.parts().len(), 2);
        let portrait = std::str::from_utf8(publication.parts()[0].bytes()).unwrap();
        let landscape = std::str::from_utf8(publication.parts()[1].bytes()).unwrap();
        assert!(portrait.contains("width=\"215.900mm\" height=\"279.400mm\""));
        assert!(portrait.contains("PORTRAIT_ONLY"));
        assert!(!portrait.contains("LANDSCAPE_ONLY"));
        assert!(landscape.contains("width=\"279.400mm\" height=\"215.900mm\""));
        assert!(landscape.contains("LANDSCAPE_ONLY"));
        assert!(!landscape.contains("PORTRAIT_ONLY"));

        let (pdf_plan, pdf_scene) = aggregate_plan_and_scene(OutputFormat::PdfVector);
        let pdf = HardcopyRenderer::render(&pdf_plan, &pdf_scene).unwrap();
        assert_eq!(
            ParsedPdf::load_mem(pdf.single_part().unwrap().bytes())
                .unwrap()
                .get_pages()
                .len(),
            2
        );
    }

    #[test]
    fn native_printer_resolves_automatic_orientation_before_rendering() {
        let (plan, scene) = aggregate_plan_and_scene(OutputFormat::NativePrinter);
        assert!(
            plan.pagination()
                .pages()
                .iter()
                .all(|page| { page.geometry().orientation() == ResolvedOrientation::Landscape })
        );
        let publication = HardcopyRenderer::render_printer_pages(&plan, &scene, 72).unwrap();
        assert_eq!(publication.pages().len(), plan.pagination().pages().len());
    }

    #[test]
    fn png_uses_planned_physical_geometry_and_records_dpi() {
        let content = extent(100_000, 60_000);
        let plan = plan(OutputFormat::Png { dpi: 72 }, content, false);
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        let decoder = png::Decoder::new(Cursor::new(publication.single_part().unwrap().bytes()));
        let reader = decoder.read_info().unwrap();
        let info = reader.info();
        assert_eq!((info.width, info.height), (792, 612));
        let dimensions = info.pixel_dims.unwrap();
        assert_eq!(dimensions.unit, png::Unit::Meter);
        assert!((2_830..=2_836).contains(&dimensions.xppu));
        assert_eq!(dimensions.xppu, dimensions.yppu);
    }

    #[test]
    fn vector_pdf_is_deterministic_multi_page_and_searchable() {
        let content = extent(500_000, 100_000);
        let plan = plan(OutputFormat::PdfVector, content, true);
        let scene = scene(content);
        let first = HardcopyRenderer::render(&plan, &scene).unwrap();
        let second = HardcopyRenderer::render(&plan, &scene).unwrap();
        assert_eq!(first, second);
        let part = first.single_part().unwrap();
        let parsed = ParsedPdf::load_mem(part.bytes()).unwrap();
        assert_eq!(parsed.get_pages().len(), plan.pagination().pages().len());
        let pages = parsed.get_pages().keys().copied().collect::<Vec<_>>();
        let text = parsed.extract_text(&pages).unwrap();
        assert!(text.contains("V(out) < 1.2 V"));
        assert!(String::from_utf8_lossy(part.bytes()).contains("/FontFile2"));
    }

    #[test]
    fn tiff_contains_every_planned_page_at_the_requested_resolution() {
        let content = extent(500_000, 100_000);
        let plan = plan(OutputFormat::Tiff { dpi: 72 }, content, true);
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        let mut decoder =
            tiff::decoder::Decoder::new(Cursor::new(publication.single_part().unwrap().bytes()))
                .unwrap();
        let mut image_count = 1_usize;
        assert_eq!(decoder.dimensions().unwrap(), (792, 612));
        while decoder.more_images() {
            decoder.next_image().unwrap();
            assert_eq!(decoder.dimensions().unwrap(), (792, 612));
            image_count += 1;
        }
        assert_eq!(image_count, plan.pagination().pages().len());
    }

    #[test]
    fn pdfa_is_validator_accepted_and_contains_archival_identification() {
        let content = extent(100_000, 60_000);
        let plan = plan(OutputFormat::PdfA, content, false);
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        assert_eq!(
            publication.pdf_conformance(),
            Some(PdfConformance::PdfA2bValidated)
        );
        let raw = String::from_utf8_lossy(publication.single_part().unwrap().bytes());
        assert!(raw.contains("<pdfaid:part>2</pdfaid:part>"));
        assert!(raw.contains("<pdfaid:conformance>B</pdfaid:conformance>"));
        assert!(raw.contains("2026-07-22T12:30:00+00:00"));
    }

    #[test]
    fn pdfa_fails_closed_without_publication_timestamp() {
        let content = extent(100_000, 60_000);
        let plan = plan(OutputFormat::PdfA, content, false);
        let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
        let scene = HardcopyScene::try_new(content, metadata, Vec::new(), Vec::new()).unwrap();
        assert!(matches!(
            HardcopyRenderer::render(&plan, &scene),
            Err(HardcopyRenderError::PdfARequiresPublicationTimestamp)
        ));
    }

    #[test]
    fn native_printer_pages_are_opaque_and_share_canonical_pagination() {
        let content = extent(100_000, 60_000);
        let plan = plan(OutputFormat::NativePrinter, content, false);
        let rendered = HardcopyRenderer::render_printer_pages(&plan, &scene(content), 72).unwrap();
        assert_eq!(rendered.pages().len(), plan.pagination().pages().len());
        let page = &rendered.pages()[0];
        assert_eq!((page.width(), page.height()), (792, 612));
        assert_eq!(page.page_number(), 1);
        assert_eq!(page.dpi(), 72);
        assert!(page.rgba().chunks_exact(4).all(|pixel| pixel[3] == 255));
    }

    #[test]
    fn resolved_source_compiles_to_semantic_scene_and_deterministic_preview() {
        let source = resolved_symbol();
        let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
        let scene = scene_from_resolved(&source, plan.setup().print_mapping(), resolved_metadata())
            .unwrap();
        assert!(!scene.primitives().is_empty());
        let first = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        let second = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        assert_eq!(first, second);
        assert_eq!((first.width(), first.height()), (792, 612));
        assert_eq!(first.page_number(), 1);
        assert_eq!(first.dpi(), 72);
        assert_eq!(first.rgba().len(), 792 * 612 * 4);
    }

    #[test]
    fn preview_worker_transfer_round_trips_without_pixel_json() {
        let source = resolved_symbol();
        let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
        let preview = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        let transfer = preview
            .clone()
            .into_worker_transfer(&plan, &source, 0)
            .unwrap();
        let (manifest_json, rgba) = transfer.into_parts();
        assert_eq!(rgba, preview.rgba());
        assert!(manifest_json.len() < 4_096);
        let manifest_value: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
        assert!(manifest_value.get("rgba").is_none());
        assert!(manifest_value.get("rgba_base64").is_none());
        assert_eq!(
            HardcopyPreviewPage::from_worker_transfer(&plan, &source, 0, 72, &manifest_json, rgba,)
                .unwrap(),
            preview
        );
    }

    #[test]
    fn preview_worker_transfer_rejects_manifest_payload_and_preview_tampering() {
        let source = resolved_symbol();
        let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
        let preview = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        let (manifest_json, rgba) = preview
            .into_worker_transfer(&plan, &source, 0)
            .unwrap()
            .into_parts();

        let mut unknown_field: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
        unknown_field["unexpected"] = serde_json::Value::Bool(true);
        assert!(matches!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                72,
                &serde_json::to_vec(&unknown_field).unwrap(),
                rgba.clone(),
            ),
            Err(HardcopyRenderError::WorkerSnapshot(_))
        ));

        let mut changed_pixels = rgba.clone();
        changed_pixels[0] ^= 0xff;
        let mut pixel_manifest: PreviewWorkerManifest =
            serde_json::from_slice(&manifest_json).unwrap();
        pixel_manifest.rgba_digest =
            ContentDigest::from_bytes(Sha256::digest(&changed_pixels).into());
        let pixel_manifest_json = reseal_preview_worker_manifest(&mut pixel_manifest);
        assert!(matches!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                72,
                &pixel_manifest_json,
                changed_pixels,
            ),
            Err(HardcopyRenderError::WorkerSnapshot(_))
        ));

        let mut digest_manifest: PreviewWorkerManifest =
            serde_json::from_slice(&manifest_json).unwrap();
        digest_manifest.preview_digest = digest(0xa5);
        let digest_manifest_json = reseal_preview_worker_manifest(&mut digest_manifest);
        assert!(matches!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                72,
                &digest_manifest_json,
                rgba,
            ),
            Err(HardcopyRenderError::WorkerSnapshot(_))
        ));
    }

    #[test]
    fn preview_worker_transfer_strictly_checks_caller_authority_geometry_and_budgets() {
        let source = resolved_symbol();
        let plan = plan_for_resolved(&source, OutputFormat::SvgVector);
        let preview = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        let (manifest_json, rgba) = preview
            .into_worker_transfer(&plan, &source, 0)
            .unwrap()
            .into_parts();
        let original_manifest: PreviewWorkerManifest =
            serde_json::from_slice(&manifest_json).unwrap();

        let mutations: [fn(&mut PreviewWorkerManifest); 15] = [
            |manifest| manifest.schema_version += 1,
            |manifest: &mut PreviewWorkerManifest| manifest.plan_id = HardcopyPlanId::new(),
            |manifest| manifest.plan_digest = digest(0x91),
            |manifest| manifest.source_document_id = HardcopyDocumentId::new(),
            |manifest| manifest.source_revision = ObjectRevision::new(2).unwrap(),
            |manifest| manifest.source_digest = digest(0x92),
            |manifest: &mut PreviewWorkerManifest| manifest.zero_based_page = 1,
            |manifest: &mut PreviewWorkerManifest| manifest.page_number = 2,
            |manifest| manifest.coordinate.push('x'),
            |manifest: &mut PreviewWorkerManifest| manifest.width += 1,
            |manifest| manifest.height += 1,
            |manifest: &mut PreviewWorkerManifest| manifest.dpi = 73,
            |manifest| manifest.soft_proof_applied = !manifest.soft_proof_applied,
            |manifest| manifest.rgba_byte_length += 4,
            |manifest| manifest.rgba_digest = digest(0x93),
        ];
        for mutate in mutations {
            let mut candidate = original_manifest.clone();
            mutate(&mut candidate);
            let candidate_json = reseal_preview_worker_manifest(&mut candidate);
            assert!(
                HardcopyPreviewPage::from_worker_transfer(
                    &plan,
                    &source,
                    0,
                    72,
                    &candidate_json,
                    rgba.clone(),
                )
                .is_err()
            );
        }
        let mut transport_tamper = original_manifest;
        transport_tamper.transport_digest = digest(0x94);
        assert!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                72,
                &serde_json::to_vec(&transport_tamper).unwrap(),
                rgba.clone(),
            )
            .is_err()
        );
        assert!(
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &source,
                0,
                73,
                &manifest_json,
                rgba.clone(),
            )
            .is_err()
        );
        assert!(matches!(
            HardcopyPreviewPage::from_worker_transfer(&plan, &source, 0, 72, &[], rgba),
            Err(HardcopyRenderError::WorkerSnapshot(_))
        ));
        assert!(matches!(
            validate_preview_worker_transfer_budget(MAX_PREVIEW_WORKER_MANIFEST_BYTES + 1, 0),
            Err(HardcopyRenderError::WorkerSnapshotTooLarge)
        ));
        assert!(matches!(
            validate_preview_worker_transfer_budget(0, MAX_PREVIEW_WORKER_RGBA_BYTES + 1),
            Err(HardcopyRenderError::WorkerSnapshotTooLarge)
        ));
    }

    #[test]
    fn publication_worker_transfer_round_trips_without_payload_json() {
        let source = resolved_wide_symbol();
        let plan = HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(81)).unwrap(),
            source.authority().clone(),
            setup(OutputFormat::SvgVector, true),
            source.content_extent(),
        )
        .unwrap();
        let publication = HardcopyRenderer::render_resolved(
            &plan,
            &source,
            HardcopySceneMetadata::try_new("Wide comparator symbol", "RSpice tests").unwrap(),
        )
        .unwrap();
        assert!(publication.parts().len() >= 2);
        let transfer = publication
            .clone()
            .into_worker_transfer(&plan, &source)
            .unwrap();
        let (manifest, payloads) = transfer.into_parts();
        assert!(manifest.len() < payloads.iter().map(Vec::len).sum::<usize>());
        for payload in &payloads {
            assert!(payload.len() >= 16);
            assert!(!manifest.windows(16).any(|window| window == &payload[..16]));
        }
        let restored =
            RenderedHardcopyPublication::from_worker_transfer(&plan, &source, &manifest, payloads)
                .unwrap();
        assert_eq!(restored, publication);
    }

    #[test]
    fn publication_worker_transfer_rejects_payload_manifest_and_plan_tampering() {
        let source = resolved_symbol();
        let resolved_plan = plan_for_resolved(&source, OutputFormat::PdfVector);
        let publication =
            HardcopyRenderer::render_resolved(&resolved_plan, &source, resolved_metadata())
                .unwrap();
        let (manifest, mut payloads) = publication
            .clone()
            .into_worker_transfer(&resolved_plan, &source)
            .unwrap()
            .into_parts();
        payloads[0][0] ^= 0x01;
        assert!(matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &resolved_plan,
                &source,
                &manifest,
                payloads
            ),
            Err(HardcopyRenderError::PublicationWorkerTransfer(_))
        ));

        let (mut manifest, payloads) = publication
            .clone()
            .into_worker_transfer(&resolved_plan, &source)
            .unwrap()
            .into_parts();
        let marker = manifest
            .windows(b"publication_digest".len())
            .position(|window| window == b"publication_digest")
            .unwrap();
        manifest[marker] = b'P';
        assert!(matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &resolved_plan,
                &source,
                &manifest,
                payloads
            ),
            Err(HardcopyRenderError::PublicationWorkerTransfer(_))
        ));

        let unrelated = plan(OutputFormat::PdfVector, source.content_extent(), false);
        let (manifest, payloads) = publication
            .into_worker_transfer(&resolved_plan, &source)
            .unwrap()
            .into_parts();
        assert!(matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &unrelated, &source, &manifest, payloads
            ),
            Err(HardcopyRenderError::SourceAuthorityMismatch)
        ));
    }

    #[test]
    fn publication_worker_transfer_strictly_validates_every_authority_and_part_field() {
        let source = resolved_symbol();
        let plan = plan_for_resolved(&source, OutputFormat::PdfVector);
        let publication =
            HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let (manifest_json, payloads) = publication
            .into_worker_transfer(&plan, &source)
            .unwrap()
            .into_parts();
        let manifest: PublicationWorkerManifest = serde_json::from_slice(&manifest_json).unwrap();
        assert!(matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &plan,
                &source,
                &[],
                payloads.clone()
            ),
            Err(HardcopyRenderError::PublicationWorkerTransfer(_))
        ));
        let rejects = |mut candidate: PublicationWorkerManifest| {
            let encoded = reseal_publication_worker_manifest(&mut candidate);
            matches!(
                RenderedHardcopyPublication::from_worker_transfer(
                    &plan,
                    &source,
                    &encoded,
                    payloads.clone()
                ),
                Err(HardcopyRenderError::PublicationWorkerTransfer(_))
            )
        };

        let mut changed = manifest.clone();
        changed.plan_digest = digest(0x90);
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.source_digest = digest(0x91);
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.publication_digest = digest(0x92);
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.format = OutputFormat::PdfA;
        changed.pdf_conformance = Some(PdfConformance::PdfA2bValidated);
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.page_count += 1;
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.parts[0].byte_length += 1;
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.parts[0].digest = digest(0x93);
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.parts[0].first_page += 1;
        assert!(rejects(changed));

        let mut changed = manifest.clone();
        changed.parts[0].page_count += 1;
        assert!(rejects(changed));

        let mut unknown_field: serde_json::Value = serde_json::from_slice(&manifest_json).unwrap();
        unknown_field["unexpected"] = serde_json::Value::Bool(true);
        assert!(matches!(
            RenderedHardcopyPublication::from_worker_transfer(
                &plan,
                &source,
                &serde_json::to_vec(&unknown_field).unwrap(),
                payloads
            ),
            Err(HardcopyRenderError::PublicationWorkerTransfer(_))
        ));
    }

    #[test]
    fn bounded_preview_batch_resolves_once_and_cancels_between_pages() {
        let source = resolved_wide_symbol();
        let plan = HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(80)).unwrap(),
            source.authority().clone(),
            setup(OutputFormat::SvgVector, true),
            source.content_extent(),
        )
        .unwrap();
        assert!(plan.pagination().pages().len() >= 2);
        let batch = HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            &source,
            resolved_metadata(),
            &[0, 1],
            72,
            || false,
        )
        .unwrap();
        assert_eq!(batch.len(), 2);
        assert_ne!(batch[0].digest(), batch[1].digest());
        let selected = HardcopyRenderer::render_preview_page_resolved(
            &plan,
            &source,
            resolved_metadata(),
            0,
            72,
        )
        .unwrap();
        assert_eq!(batch[0], selected);

        let cancelled = HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            &source,
            resolved_metadata(),
            &[0, 1],
            72,
            || true,
        )
        .unwrap();
        assert_eq!(cancelled, vec![selected]);
        assert!(matches!(
            HardcopyRenderer::render_preview_pages_resolved(
                &plan,
                &source,
                resolved_metadata(),
                &[0, 0],
                72,
                || false
            ),
            Err(HardcopyRenderError::InvalidPreviewPageBatch)
        ));
    }

    #[test]
    fn resolved_preview_rejects_authority_index_and_resolution_mismatch() {
        let source = resolved_symbol();
        let resolved_plan = plan_for_resolved(&source, OutputFormat::PdfVector);
        let unrelated = plan(OutputFormat::PdfVector, source.content_extent(), false);
        assert!(matches!(
            HardcopyRenderer::render_preview_page_resolved(
                &unrelated,
                &source,
                resolved_metadata(),
                0,
                72
            ),
            Err(HardcopyRenderError::SourceAuthorityMismatch)
        ));
        assert!(matches!(
            HardcopyRenderer::render_preview_page_resolved(
                &resolved_plan,
                &source,
                resolved_metadata(),
                1,
                72
            ),
            Err(HardcopyRenderError::PreviewPageOutOfRange { .. })
        ));
        assert!(matches!(
            HardcopyRenderer::render_preview_page_resolved(
                &resolved_plan,
                &source,
                resolved_metadata(),
                0,
                12
            ),
            Err(HardcopyRenderError::InvalidPreviewDpi(12))
        ));
    }

    #[test]
    fn resolved_native_printer_pages_succeed_and_reject_stale_authority() {
        let source = resolved_symbol();
        let resolved_plan = plan_for_resolved(&source, OutputFormat::NativePrinter);
        let pages = HardcopyRenderer::render_printer_pages_resolved(
            &resolved_plan,
            &source,
            resolved_metadata(),
            72,
        )
        .unwrap();
        assert_eq!(
            pages.pages().len(),
            resolved_plan.pagination().pages().len()
        );
        assert!(
            pages.pages()[0]
                .rgba()
                .chunks_exact(4)
                .all(|pixel| pixel[3] == 255)
        );
        let unrelated = plan(OutputFormat::NativePrinter, source.content_extent(), false);
        assert!(matches!(
            HardcopyRenderer::render_printer_pages_resolved(
                &unrelated,
                &source,
                resolved_metadata(),
                72
            ),
            Err(HardcopyRenderError::SourceAuthorityMismatch)
        ));
    }

    #[test]
    fn grayscale_trace_policy_adds_redundant_dash_encoding() {
        let content = extent(100_000, 60_000);
        let base = setup(OutputFormat::SvgVector, false);
        let setup = HardcopySetup::try_new(
            base.physical_page().clone(),
            base.scale(),
            base.tiling(),
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::SvgVector,
                ColorMapping::GrayscaleWithDashMarkerRedundancy,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            )
            .unwrap(),
            base.decorations().clone(),
            base.print_mapping().clone(),
        )
        .unwrap();
        let plan = HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(3)).unwrap(),
            source(),
            setup,
            content,
        )
        .unwrap();
        let publication = HardcopyRenderer::render(&plan, &scene(content)).unwrap();
        let svg = std::str::from_utf8(publication.single_part().unwrap().bytes()).unwrap();
        assert!(svg.contains("stroke-dasharray"));
    }

    #[test]
    fn gray_percent_is_black_ink_coverage_not_reflected_channel_value() {
        assert_eq!(
            print_color_rgb(PrintColor::GrayPercent(70)),
            Rgb8::new(77, 77, 77)
        );
        assert_eq!(
            print_color_rgb(PrintColor::GrayPercent(40)),
            Rgb8::new(153, 153, 153)
        );
        assert_eq!(
            print_color_rgb(PrintColor::GrayPercent(60)),
            Rgb8::new(102, 102, 102)
        );
    }
}
