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
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use tiff::encoder::{Compression, DeflateLevel, Rational, TiffEncoder, colortype};
use tiff::tags::ResolutionUnit;

use crate::hardcopy::{
    BackgroundMode, Bleed, ColorMapping, ContentExtent, HardcopyArtifactIdentity,
    HardcopyDocumentId, HardcopyPlan, HardcopyPlanId, Length, OutputFormat, PageRect, PreviewPage,
    PrintColor, PrintMappingTable, PrintObjectKind, PrintRedundancy, RenderTarget,
    ResolvedOrientation, ScaleRatio, Watermark,
};
use super::hardcopy_sources::{
    HardcopySemanticDocument, ResolvedHardcopyDocument, SCHEMATIC_UNIT_UM, SemanticAggregate,
    SemanticBounds, SemanticPlot, SemanticPoint, SemanticReport, SemanticReportFigure,
    SemanticResultSummary, SemanticSchematic, SemanticTable,
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
#[allow(dead_code)]
const PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION: u32 = 1;
#[allow(dead_code)]
pub(crate) const MAX_PREVIEW_WORKER_MANIFEST_BYTES: usize = 16 * 1_024;
#[allow(dead_code)]
pub(crate) const MAX_PREVIEW_WORKER_RGBA_BYTES: usize = 48 * 1_048_576;
#[allow(dead_code)]
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
const IBM_PLEX_SANS_REGULAR: &[u8] = include_bytes!("../../assets/fonts/IBMPlexSans-Regular.ttf");
const IBM_PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/IBMPlexSans-SemiBold.ttf");
const IBM_PLEX_MONO_REGULAR: &[u8] = include_bytes!("../../assets/fonts/IBMPlexMono-Regular.ttf");

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PdfConformance {
    StandardPdf,
    PdfA2bValidated,
}

/// One renderer-owned dialog preview. The dimensions are the exact physical
/// page at the requested preview DPI; no viewport scaling or screen capture is
/// involved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardcopyPreviewPage {
    page_number: u32,
    coordinate: String,
    width: u32,
    height: u32,
    dpi: u16,
    rgba: Vec<u8>,
    soft_proof_applied: bool,
    digest: ContentDigest,
}

/// Transferable browser-worker envelope for a single preview page. Pixels
/// remain a distinct byte buffer so the WASM boundary can transfer the
/// backing `ArrayBuffer` without base64 expansion or a JSON pixel copy.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct PreviewWorkerTransfer {
    manifest_json: Vec<u8>,
    rgba: Vec<u8>,
}

#[allow(dead_code)]
impl PreviewWorkerTransfer {
    pub(crate) fn into_parts(self) -> (Vec<u8>, Vec<u8>) {
        (self.manifest_json, self.rgba)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreviewWorkerManifest {
    schema_version: u32,
    plan_id: HardcopyPlanId,
    plan_digest: ContentDigest,
    source_document_id: HardcopyDocumentId,
    source_revision: ObjectRevision,
    source_digest: ContentDigest,
    zero_based_page: u32,
    page_number: u32,
    coordinate: String,
    width: u32,
    height: u32,
    dpi: u16,
    soft_proof_applied: bool,
    rgba_byte_length: u64,
    rgba_digest: ContentDigest,
    preview_digest: ContentDigest,
    transport_digest: ContentDigest,
}

#[derive(Serialize)]
struct PreviewWorkerManifestMaterial<'a> {
    schema_version: u32,
    plan_id: HardcopyPlanId,
    plan_digest: ContentDigest,
    source_document_id: HardcopyDocumentId,
    source_revision: ObjectRevision,
    source_digest: ContentDigest,
    zero_based_page: u32,
    page_number: u32,
    coordinate: &'a str,
    width: u32,
    height: u32,
    dpi: u16,
    soft_proof_applied: bool,
    rgba_byte_length: u64,
    rgba_digest: ContentDigest,
    preview_digest: ContentDigest,
}

impl HardcopyPreviewPage {
    #[must_use]
    pub const fn page_number(&self) -> u32 {
        self.page_number
    }

    #[must_use]
    pub fn coordinate(&self) -> &str {
        &self.coordinate
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn dpi(&self) -> u16 {
        self.dpi
    }

    #[must_use]
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    /// True when this non-authoritative preview has the explicitly requested
    /// print-safe gamut simulation applied. Publication and printer bytes are
    /// never transformed by this preview-only option.
    #[must_use]
    pub const fn soft_proof_applied(&self) -> bool {
        self.soft_proof_applied
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// Consume this page into a small authenticated manifest and a raw RGBA
    /// payload suitable for a transferable browser `ArrayBuffer`.
    ///
    /// The immutable plan, authenticated source, planned page, exact physical
    /// dimensions, DPI, pixel bytes, and renderer preview digest are all
    /// checked before either buffer crosses the worker boundary.
    #[allow(dead_code)]
    pub(crate) fn into_worker_transfer(
        self,
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        zero_based_page: usize,
    ) -> Result<PreviewWorkerTransfer, HardcopyRenderError> {
        validate_preview_worker_contract(&self, plan, source, zero_based_page)?;
        let rgba_digest = ContentDigest::from_bytes(Sha256::digest(&self.rgba).into());
        let rgba_byte_length = u64::try_from(self.rgba.len())
            .map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
        let zero_based_page = u32::try_from(zero_based_page).map_err(|_| {
            HardcopyRenderError::WorkerSnapshot("preview page index exceeds u32".to_owned())
        })?;
        let material = PreviewWorkerManifestMaterial {
            schema_version: PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION,
            plan_id: plan.id(),
            plan_digest: plan.content_digest(),
            source_document_id: source.authority().document_id(),
            source_revision: source.authority().revision(),
            source_digest: source.authority().content_digest(),
            zero_based_page,
            page_number: self.page_number,
            coordinate: &self.coordinate,
            width: self.width,
            height: self.height,
            dpi: self.dpi,
            soft_proof_applied: self.soft_proof_applied,
            rgba_byte_length,
            rgba_digest,
            preview_digest: self.digest,
        };
        let transport_digest = preview_worker_material_digest(&material)?;
        let manifest = PreviewWorkerManifest {
            schema_version: material.schema_version,
            plan_id: material.plan_id,
            plan_digest: material.plan_digest,
            source_document_id: material.source_document_id,
            source_revision: material.source_revision,
            source_digest: material.source_digest,
            zero_based_page: material.zero_based_page,
            page_number: material.page_number,
            coordinate: material.coordinate.to_owned(),
            width: material.width,
            height: material.height,
            dpi: material.dpi,
            soft_proof_applied: material.soft_proof_applied,
            rgba_byte_length: material.rgba_byte_length,
            rgba_digest,
            preview_digest: material.preview_digest,
            transport_digest,
        };
        let manifest_json = serde_json::to_vec(&manifest)
            .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
        validate_preview_worker_transfer_budget(manifest_json.len(), self.rgba.len())?;
        Ok(PreviewWorkerTransfer {
            manifest_json,
            rgba: self.rgba,
        })
    }

    /// Reconstruct a renderer-owned preview only after validating the
    /// metadata-only manifest and independently transferred RGBA bytes against
    /// caller-retained plan, source, page, and DPI authority.
    #[allow(dead_code)]
    pub(crate) fn from_worker_transfer(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        expected_zero_based_page: usize,
        expected_dpi: u16,
        manifest_json: &[u8],
        rgba: Vec<u8>,
    ) -> Result<Self, HardcopyRenderError> {
        validate_preview_worker_transfer_budget(manifest_json.len(), rgba.len())?;
        if manifest_json.is_empty() {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest is empty".to_owned(),
            ));
        }
        let manifest: PreviewWorkerManifest = serde_json::from_slice(manifest_json)
            .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
        if manifest.schema_version != PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "unsupported preview worker manifest schema".to_owned(),
            ));
        }
        if !(36..=1_200).contains(&expected_dpi) {
            return Err(HardcopyRenderError::InvalidPreviewDpi(expected_dpi));
        }
        let expected_zero_based_page_u32 =
            u32::try_from(expected_zero_based_page).map_err(|_| {
                HardcopyRenderError::WorkerSnapshot("preview page index exceeds u32".to_owned())
            })?;
        let observed_rgba_byte_length =
            u64::try_from(rgba.len()).map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
        validate_worker_authority(plan, source)?;
        let page = plan
            .pagination()
            .pages()
            .get(expected_zero_based_page)
            .ok_or(HardcopyRenderError::PreviewPageOutOfRange {
                index: expected_zero_based_page,
                page_count: plan.pagination().pages().len(),
            })?;
        let (expected_width, expected_height, expected_pixels) =
            raster_dimensions(page, expected_dpi)?;
        let expected_rgba_bytes = expected_pixels
            .checked_mul(4)
            .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
        if manifest.plan_id != plan.id()
            || manifest.plan_digest != plan.content_digest()
            || manifest.source_document_id != source.authority().document_id()
            || manifest.source_revision != source.authority().revision()
            || manifest.source_digest != source.authority().content_digest()
            || manifest.zero_based_page != expected_zero_based_page_u32
            || manifest.page_number != page.number()
            || manifest.coordinate != page.coordinate()
            || manifest.width != expected_width
            || manifest.height != expected_height
            || manifest.dpi != expected_dpi
            || manifest.soft_proof_applied != plan.setup().render().soft_proof_print_safe_colors()
            || manifest.rgba_byte_length != expected_rgba_bytes
            || manifest.rgba_byte_length != observed_rgba_byte_length
        {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest does not match the immutable plan, source, page, or DPI"
                    .to_owned(),
            ));
        }
        let observed_rgba_digest = ContentDigest::from_bytes(Sha256::digest(&rgba).into());
        if observed_rgba_digest != manifest.rgba_digest {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker RGBA digest mismatch".to_owned(),
            ));
        }
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
        if preview_worker_material_digest(&material)? != manifest.transport_digest {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest digest mismatch".to_owned(),
            ));
        }
        let preview = Self {
            page_number: manifest.page_number,
            coordinate: manifest.coordinate,
            width: manifest.width,
            height: manifest.height,
            dpi: manifest.dpi,
            rgba,
            soft_proof_applied: manifest.soft_proof_applied,
            digest: manifest.preview_digest,
        };
        validate_preview_worker_contract(&preview, plan, source, expected_zero_based_page)?;
        Ok(preview)
    }
}

#[allow(dead_code)]
fn validate_preview_worker_transfer_budget(
    manifest_bytes: usize,
    rgba_bytes: usize,
) -> Result<(), HardcopyRenderError> {
    let transfer_bytes = manifest_bytes
        .checked_add(rgba_bytes)
        .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
    if manifest_bytes > MAX_PREVIEW_WORKER_MANIFEST_BYTES
        || rgba_bytes > MAX_PREVIEW_WORKER_RGBA_BYTES
        || transfer_bytes > MAX_PREVIEW_WORKER_TRANSFER_BYTES
    {
        return Err(HardcopyRenderError::WorkerSnapshotTooLarge);
    }
    Ok(())
}

#[allow(dead_code)]
fn preview_worker_material_digest(
    material: &PreviewWorkerManifestMaterial<'_>,
) -> Result<ContentDigest, HardcopyRenderError> {
    let payload = serde_json::to_vec(material)
        .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-preview-worker-manifest-v1");
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[allow(dead_code)]
fn validate_preview_worker_contract(
    preview: &HardcopyPreviewPage,
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    zero_based_page: usize,
) -> Result<(), HardcopyRenderError> {
    validate_worker_authority(plan, source)?;
    validate_text("preview coordinate", &preview.coordinate, 128)?;
    if preview.page_number == 0 || !(36..=1_200).contains(&preview.dpi) {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview page identity or DPI is invalid".to_owned(),
        ));
    }
    let page = plan.pagination().pages().get(zero_based_page).ok_or(
        HardcopyRenderError::PreviewPageOutOfRange {
            index: zero_based_page,
            page_count: plan.pagination().pages().len(),
        },
    )?;
    let (expected_width, expected_height, expected_pixels) = raster_dimensions(page, preview.dpi)?;
    let rgba_bytes = expected_pixels
        .checked_mul(4)
        .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
    let observed_rgba_bytes = u64::try_from(preview.rgba.len())
        .map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
    if preview.page_number != page.number()
        || preview.coordinate != page.coordinate()
        || preview.width != expected_width
        || preview.height != expected_height
        || rgba_bytes != observed_rgba_bytes
        || rgba_bytes > MAX_PREVIEW_WORKER_RGBA_BYTES as u64
        || preview.soft_proof_applied != plan.setup().render().soft_proof_print_safe_colors()
    {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview does not match the planned page geometry or bounded RGBA payload".to_owned(),
        ));
    }
    let expected_digest = preview_content_digest(
        plan,
        source,
        page,
        preview.dpi,
        preview.width,
        preview.height,
        &preview.rgba,
    );
    if preview.digest != expected_digest {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview renderer digest mismatch".to_owned(),
        ));
    }
    Ok(())
}

fn preview_content_digest(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    page: &PreviewPage,
    dpi: u16,
    width: u32,
    height: u32,
    rgba: &[u8],
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-preview-v1");
    digest.update(plan.content_digest().as_bytes());
    digest.update(source.authority().content_digest().as_bytes());
    digest.update(page.number().to_le_bytes());
    digest.update(dpi.to_le_bytes());
    digest.update(width.to_le_bytes());
    digest.update(height.to_le_bytes());
    digest.update(rgba);
    ContentDigest::from_bytes(digest.finalize().into())
}

/// One fully rendered physical printer page in straight-alpha RGBA8 sRGB.
/// Native adapters may discard the alpha channel after validation; it is
/// always 255 because printer plans require an opaque background.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrinterRasterPage {
    page_number: u32,
    coordinate: String,
    width: u32,
    height: u32,
    dpi: u16,
    rgba: Vec<u8>,
    digest: ContentDigest,
}

impl PrinterRasterPage {
    #[must_use]
    pub const fn page_number(&self) -> u32 {
        self.page_number
    }

    #[must_use]
    pub fn coordinate(&self) -> &str {
        &self.coordinate
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn dpi(&self) -> u16 {
        self.dpi
    }

    #[must_use]
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedPrinterPages {
    pages: Vec<PrinterRasterPage>,
    digest: ContentDigest,
}

impl RenderedPrinterPages {
    #[must_use]
    pub fn pages(&self) -> &[PrinterRasterPage] {
        &self.pages
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }
}

/// One independently publishable file. Single-file formats have one part;
/// SVG and PNG tiling produce one numbered part per planned physical page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedHardcopyPart {
    bytes: Vec<u8>,
    digest: ContentDigest,
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
}

impl RenderedHardcopyPart {
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub const fn media_type(&self) -> &'static str {
        self.media_type
    }

    #[must_use]
    pub const fn filename_extension(&self) -> &'static str {
        self.filename_extension
    }

    #[must_use]
    pub fn suggested_filename(&self) -> &str {
        &self.suggested_filename
    }

    #[must_use]
    pub const fn first_page(&self) -> u32 {
        self.first_page
    }

    #[must_use]
    pub const fn page_count(&self) -> u32 {
        self.page_count
    }
}

/// Complete typed render publication. `digest` binds the ordered part names,
/// media types, page ranges, and exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedHardcopyPublication {
    parts: Vec<RenderedHardcopyPart>,
    digest: ContentDigest,
    format: OutputFormat,
    page_count: u32,
    pdf_conformance: Option<PdfConformance>,
}

/// Transferable browser-worker envelope. Artifact payloads remain distinct
/// byte buffers so the WASM boundary can transfer their backing ArrayBuffers
/// without JSON/base64 expansion or a second aggregate artifact allocation.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct PublicationWorkerTransfer {
    manifest_json: Vec<u8>,
    payloads: Vec<Vec<u8>>,
}

#[allow(dead_code)]
impl PublicationWorkerTransfer {
    pub(crate) fn into_parts(self) -> (Vec<u8>, Vec<Vec<u8>>) {
        (self.manifest_json, self.payloads)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicationWorkerManifest {
    schema_version: u32,
    plan_digest: ContentDigest,
    source_digest: ContentDigest,
    publication_digest: ContentDigest,
    format: OutputFormat,
    page_count: u32,
    pdf_conformance: Option<PdfConformance>,
    parts: Vec<PublicationWorkerPartManifest>,
    transport_digest: ContentDigest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicationWorkerPartManifest {
    ordinal: u32,
    byte_length: u64,
    digest: ContentDigest,
    media_type: String,
    filename_extension: String,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
}

#[derive(Serialize)]
struct PublicationWorkerManifestMaterial<'a> {
    schema_version: u32,
    plan_digest: ContentDigest,
    source_digest: ContentDigest,
    publication_digest: ContentDigest,
    format: OutputFormat,
    page_count: u32,
    pdf_conformance: Option<PdfConformance>,
    parts: &'a [PublicationWorkerPartManifest],
}

impl RenderedHardcopyPublication {
    #[must_use]
    pub fn parts(&self) -> &[RenderedHardcopyPart] {
        &self.parts
    }

    #[must_use]
    pub fn single_part(&self) -> Option<&RenderedHardcopyPart> {
        (self.parts.len() == 1).then(|| &self.parts[0])
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    #[must_use]
    pub const fn page_count(&self) -> u32 {
        self.page_count
    }

    #[must_use]
    pub const fn pdf_conformance(&self) -> Option<PdfConformance> {
        self.pdf_conformance
    }

    pub fn identity(&self) -> Result<HardcopyArtifactIdentity, HardcopyRenderError> {
        let byte_length = self.parts.iter().try_fold(0_u64, |sum, part| {
            sum.checked_add(part.bytes.len() as u64)
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "aggregate artifact bytes",
                    maximum: u64::MAX,
                })
        })?;
        HardcopyArtifactIdentity::try_new(self.digest, byte_length, self.page_count, self.format)
            .map_err(|error| HardcopyRenderError::ArtifactIdentity(error.to_string()))
    }

    /// Consume a validated publication into a small digest-bound manifest and
    /// independent payload buffers suitable for transferable ArrayBuffers.
    #[allow(dead_code)]
    pub(crate) fn into_worker_transfer(
        self,
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
    ) -> Result<PublicationWorkerTransfer, HardcopyRenderError> {
        validate_publication_worker_contract(&self, plan, source)?;
        let parts = self
            .parts
            .iter()
            .enumerate()
            .map(|(index, part)| PublicationWorkerPartManifest {
                ordinal: index as u32,
                byte_length: part.bytes.len() as u64,
                digest: part.digest,
                media_type: part.media_type.to_owned(),
                filename_extension: part.filename_extension.to_owned(),
                suggested_filename: part.suggested_filename.clone(),
                first_page: part.first_page,
                page_count: part.page_count,
            })
            .collect::<Vec<_>>();
        let material = PublicationWorkerManifestMaterial {
            schema_version: PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION,
            plan_digest: plan.content_digest(),
            source_digest: source.authority().content_digest(),
            publication_digest: self.digest,
            format: self.format,
            page_count: self.page_count,
            pdf_conformance: self.pdf_conformance,
            parts: &parts,
        };
        let transport_digest = publication_worker_manifest_digest(&material)?;
        let manifest = PublicationWorkerManifest {
            schema_version: material.schema_version,
            plan_digest: material.plan_digest,
            source_digest: material.source_digest,
            publication_digest: material.publication_digest,
            format: material.format,
            page_count: material.page_count,
            pdf_conformance: material.pdf_conformance,
            parts,
            transport_digest,
        };
        let manifest_json = serde_json::to_vec(&manifest)
            .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
        if manifest_json.len() > MAX_PUBLICATION_WORKER_MANIFEST_BYTES {
            return Err(HardcopyRenderError::PublicationWorkerManifestTooLarge);
        }
        let payloads = self.parts.into_iter().map(|part| part.bytes).collect();
        Ok(PublicationWorkerTransfer {
            manifest_json,
            payloads,
        })
    }

    /// Reconstruct a renderer-owned publication only after validating the
    /// complete manifest and every independently transferred payload against
    /// the immutable plan and authenticated source.
    #[allow(dead_code)]
    pub(crate) fn from_worker_transfer(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        manifest_json: &[u8],
        payloads: Vec<Vec<u8>>,
    ) -> Result<Self, HardcopyRenderError> {
        if manifest_json.is_empty() {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication worker manifest is empty".to_owned(),
            ));
        }
        if manifest_json.len() > MAX_PUBLICATION_WORKER_MANIFEST_BYTES {
            return Err(HardcopyRenderError::PublicationWorkerManifestTooLarge);
        }
        let manifest: PublicationWorkerManifest = serde_json::from_slice(manifest_json)
            .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
        if manifest.schema_version != PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "unsupported publication worker manifest schema".to_owned(),
            ));
        }
        validate_worker_authority(plan, source)?;
        let expected_part_count = publication_worker_part_count(plan);
        if manifest.plan_digest != plan.content_digest()
            || manifest.source_digest != source.authority().content_digest()
            || manifest.format != plan.setup().render().format()
            || manifest.page_count != plan.pagination().pages().len() as u32
            || manifest.pdf_conformance != expected_pdf_conformance(manifest.format)
            || manifest.parts.len() != expected_part_count
            || manifest.parts.len() != payloads.len()
        {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication manifest does not match the immutable plan or source".to_owned(),
            ));
        }
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
        if publication_worker_manifest_digest(&material)? != manifest.transport_digest {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication worker manifest digest mismatch".to_owned(),
            ));
        }
        let mut aggregate_bytes = 0_u64;
        let mut parts = Vec::with_capacity(payloads.len());
        for (index, (part_manifest, bytes)) in manifest.parts.iter().zip(payloads).enumerate() {
            let expected = expected_worker_part(plan, index)?;
            if part_manifest.ordinal != index as u32
                || part_manifest.byte_length != bytes.len() as u64
                || part_manifest.media_type != expected.media_type
                || part_manifest.filename_extension != expected.filename_extension
                || part_manifest.suggested_filename != expected.suggested_filename
                || part_manifest.first_page != expected.first_page
                || part_manifest.page_count != expected.page_count
            {
                return Err(HardcopyRenderError::PublicationWorkerTransfer(
                    "publication part metadata is not canonical".to_owned(),
                ));
            }
            if bytes.is_empty() || bytes.len() > MAX_ARTIFACT_BYTES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "artifact part bytes",
                    maximum: MAX_ARTIFACT_BYTES as u64,
                });
            }
            aggregate_bytes = aggregate_bytes.checked_add(bytes.len() as u64).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "aggregate publication bytes",
                    maximum: MAX_PUBLICATION_BYTES,
                },
            )?;
            let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
            if digest != part_manifest.digest {
                return Err(HardcopyRenderError::PublicationWorkerTransfer(
                    "publication part digest mismatch".to_owned(),
                ));
            }
            parts.push(RenderedHardcopyPart {
                bytes,
                digest,
                media_type: expected.media_type,
                filename_extension: expected.filename_extension,
                suggested_filename: expected.suggested_filename,
                first_page: expected.first_page,
                page_count: expected.page_count,
            });
        }
        if aggregate_bytes > MAX_PUBLICATION_BYTES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            });
        }
        let digest = publication_digest(manifest.format, manifest.page_count, &parts);
        if digest != manifest.publication_digest {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication aggregate digest mismatch".to_owned(),
            ));
        }
        let publication = Self {
            parts,
            digest,
            format: manifest.format,
            page_count: manifest.page_count,
            pdf_conformance: manifest.pdf_conformance,
        };
        validate_publication_worker_contract(&publication, plan, source)?;
        Ok(publication)
    }
}

struct ExpectedWorkerPart {
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
}

fn validate_worker_authority(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
) -> Result<(), HardcopyRenderError> {
    if plan.source() != source.authority() {
        return Err(HardcopyRenderError::SourceAuthorityMismatch);
    }
    if plan.content_extent() != source.content_extent() {
        return Err(HardcopyRenderError::ExtentMismatch);
    }
    Ok(())
}

fn expected_pdf_conformance(format: OutputFormat) -> Option<PdfConformance> {
    match format {
        OutputFormat::PdfA => Some(PdfConformance::PdfA2bValidated),
        OutputFormat::PdfVector | OutputFormat::NativePrinter => Some(PdfConformance::StandardPdf),
        OutputFormat::BrowserPrintDocument
        | OutputFormat::SvgVector
        | OutputFormat::Png { .. }
        | OutputFormat::Tiff { .. } => None,
    }
}

fn publication_worker_part_count(plan: &HardcopyPlan) -> usize {
    match plan.setup().render().format() {
        OutputFormat::SvgVector | OutputFormat::Png { .. } => plan.pagination().pages().len(),
        _ => 1,
    }
}

fn expected_worker_part(
    plan: &HardcopyPlan,
    index: usize,
) -> Result<ExpectedWorkerPart, HardcopyRenderError> {
    let page_count = plan.pagination().pages().len() as u32;
    let format = plan.setup().render().format();
    match format {
        OutputFormat::SvgVector | OutputFormat::Png { .. } => {
            let page = plan.pagination().pages().get(index).ok_or_else(|| {
                HardcopyRenderError::PublicationWorkerTransfer(
                    "publication has more page parts than the immutable plan".to_owned(),
                )
            })?;
            let (media_type, filename_extension) = match format {
                OutputFormat::SvgVector => ("image/svg+xml", "svg"),
                OutputFormat::Png { .. } => ("image/png", "png"),
                _ => unreachable!("matched above"),
            };
            Ok(ExpectedWorkerPart {
                media_type,
                filename_extension,
                suggested_filename: format!(
                    "page-{:04}-{}.{}",
                    page.number(),
                    page.coordinate(),
                    filename_extension
                ),
                first_page: page.number(),
                page_count: 1,
            })
        }
        _ if index != 0 => Err(HardcopyRenderError::PublicationWorkerTransfer(
            "single-artifact format contains multiple worker payloads".to_owned(),
        )),
        OutputFormat::BrowserPrintDocument => Ok(ExpectedWorkerPart {
            media_type: "text/html",
            filename_extension: "html",
            suggested_filename: "hardcopy-print.html".to_owned(),
            first_page: 1,
            page_count,
        }),
        OutputFormat::Tiff { .. } => Ok(ExpectedWorkerPart {
            media_type: "image/tiff",
            filename_extension: "tiff",
            suggested_filename: "hardcopy.tiff".to_owned(),
            first_page: 1,
            page_count,
        }),
        OutputFormat::PdfA | OutputFormat::PdfVector | OutputFormat::NativePrinter => {
            Ok(ExpectedWorkerPart {
                media_type: "application/pdf",
                filename_extension: "pdf",
                suggested_filename: "hardcopy.pdf".to_owned(),
                first_page: 1,
                page_count,
            })
        }
    }
}

fn validate_publication_worker_contract(
    publication: &RenderedHardcopyPublication,
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
) -> Result<(), HardcopyRenderError> {
    validate_worker_authority(plan, source)?;
    let expected_page_count = plan.pagination().pages().len() as u32;
    let expected_parts = publication_worker_part_count(plan);
    if publication.format != plan.setup().render().format()
        || publication.page_count != expected_page_count
        || publication.pdf_conformance != expected_pdf_conformance(publication.format)
        || publication.parts.len() != expected_parts
    {
        return Err(HardcopyRenderError::PublicationWorkerTransfer(
            "publication does not match the immutable format and pagination".to_owned(),
        ));
    }
    let mut aggregate_bytes = 0_u64;
    for (index, part) in publication.parts.iter().enumerate() {
        let expected = expected_worker_part(plan, index)?;
        if part.media_type != expected.media_type
            || part.filename_extension != expected.filename_extension
            || part.suggested_filename != expected.suggested_filename
            || part.first_page != expected.first_page
            || part.page_count != expected.page_count
            || part.bytes.is_empty()
            || part.bytes.len() > MAX_ARTIFACT_BYTES
            || ContentDigest::from_bytes(Sha256::digest(&part.bytes).into()) != part.digest
        {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication part failed canonical metadata or digest validation".to_owned(),
            ));
        }
        aggregate_bytes = aggregate_bytes.checked_add(part.bytes.len() as u64).ok_or(
            HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            },
        )?;
    }
    if aggregate_bytes > MAX_PUBLICATION_BYTES
        || publication_digest(
            publication.format,
            publication.page_count,
            &publication.parts,
        ) != publication.digest
    {
        return Err(HardcopyRenderError::PublicationWorkerTransfer(
            "publication aggregate size or digest is invalid".to_owned(),
        ));
    }
    Ok(())
}

fn publication_worker_manifest_digest(
    material: &PublicationWorkerManifestMaterial<'_>,
) -> Result<ContentDigest, HardcopyRenderError> {
    let payload = serde_json::to_vec(material)
        .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-publication-worker-v1");
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[derive(Debug, thiserror::Error)]
pub enum HardcopyRenderError {
    #[error("hardcopy publication timestamp is not a valid UTC Gregorian date and time")]
    InvalidTimestamp,
    #[error("{field} must be trimmed printable text no longer than {maximum} bytes")]
    InvalidText { field: &'static str, maximum: usize },
    #[error("hardcopy scene exceeds the {scope} limit of {maximum}")]
    ResourceLimit { scope: &'static str, maximum: u64 },
    #[error("scene primitive has empty geometry")]
    EmptyPrimitiveGeometry,
    #[error("scene coordinate lies outside the declared content extent")]
    PrimitiveOutsideExtent,
    #[error("polyline requires at least two points, or three points when closed")]
    InvalidPolyline,
    #[error("stroke width {0:?} is outside the supported physical range")]
    InvalidStrokeWidth(Length),
    #[error("cross-hatch line width and spacing must be ordered physical values within 25 mm")]
    InvalidCrossHatch,
    #[error("text size must be between 1 micrometre and 100 millimetres")]
    InvalidTextSize,
    #[error("embedded hardcopy font has no glyph for U+{codepoint:04X} in {context}")]
    UnsupportedGlyph {
        codepoint: u32,
        context: &'static str,
    },
    #[error("hardcopy scene extent does not match the immutable render plan")]
    ExtentMismatch,
    #[error("aggregate scene sections do not match the immutable pagination plan")]
    AggregatePaginationMismatch,
    #[error("resolved hardcopy source authority does not match the immutable render plan")]
    SourceAuthorityMismatch,
    #[error("could not convert the resolved semantic hardcopy source: {0}")]
    SourceConversion(String),
    #[error(
        "report {0} lacks authenticated renderable content; publication is blocked instead of flattening evidence"
    )]
    UnsupportedAuthenticatedReportBlock(&'static str),
    #[error("report block reference is absent from or differs from the authenticated snapshot")]
    UnauthenticatedReportReference,
    #[error("report Markdown contains unsupported or unbalanced authored constructs")]
    UnsupportedReportMarkdown,
    #[error("embedded report figure digest does not match its authenticated artifact")]
    EmbeddedFigureDigestMismatch,
    #[error("embedded report figure is not a supported bounded PNG: {0}")]
    InvalidEmbeddedFigure(String),
    #[error("PDF/A export requires an explicit deterministic publication timestamp")]
    PdfARequiresPublicationTimestamp,
    #[error(
        "{decoration} contains {actual} entries but the planned band can render at most {maximum}"
    )]
    DecorationOverflow {
        decoration: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("SVG export cannot outline text; enable searchable text or remove text from the scene")]
    SvgTextOutliningUnsupported,
    #[error("deterministic browser printing with text requires embedded fonts")]
    BrowserTextRequiresEmbeddedFonts,
    #[error("native printer output must use an opaque page background")]
    TransparentPrinterPage,
    #[error("printer raster pages require a NativePrinter plan targeted at a system printer")]
    PrinterRasterRequiresNativePlan,
    #[error("printer device resolution {0} DPI is outside the supported 72–9600 DPI range")]
    InvalidPrinterDpi(u16),
    #[error("printer render DPI differs from the immutable native job resolution")]
    PrinterRasterDpiMismatch,
    #[error("planned printable area lies outside the sealed native driver printable geometry")]
    PrinterPrintableGeometryMismatch,
    #[error("the selected native printer job cannot mix portrait and landscape page geometry")]
    PrinterMixedPageGeometryUnsupported,
    #[error("preview resolution {0} DPI is outside the supported 36–1200 DPI range")]
    InvalidPreviewDpi(u16),
    #[error("preview page index {index} is outside the planned {page_count} pages")]
    PreviewPageOutOfRange { index: usize, page_count: usize },
    #[error("preview requests must contain one or two distinct page indices")]
    InvalidPreviewPageBatch,
    #[error("hardcopy preview worker transfer exceeds its bounded transport budget")]
    WorkerSnapshotTooLarge,
    #[error("hardcopy preview worker transfer is invalid: {0}")]
    WorkerSnapshot(String),
    #[error("hardcopy publication worker manifest exceeds its bounded transport budget")]
    PublicationWorkerManifestTooLarge,
    #[error("hardcopy publication worker transfer is invalid: {0}")]
    PublicationWorkerTransfer(String),
    #[error("raster dimensions overflow the supported integer range")]
    RasterDimensionOverflow,
    #[error("could not parse canonical SVG for rasterization: {0}")]
    SvgParse(String),
    #[error("could not allocate the requested raster page")]
    RasterAllocation,
    #[error("could not encode {format}: {message}")]
    Encoding {
        format: &'static str,
        message: String,
    },
    #[error("could not load embedded font {0}")]
    InvalidEmbeddedFont(&'static str),
    #[error("could not configure validated PDF/A-2b output: {0}")]
    PdfAConfiguration(String),
    #[error("PDF serialization or validation failed: {0}")]
    PdfSerialization(String),
    #[error("hardcopy artifact identity is invalid: {0}")]
    ArtifactIdentity(String),
}

/// Deterministic, platform-independent hardcopy renderer.
pub struct HardcopyRenderer;

impl HardcopyRenderer {
    /// Convert and render an authenticated semantic source in one fail-closed
    /// operation. The plan must bind the exact same source authority and
    /// physical extent; a stale tab or replaced revision can never be printed
    /// through a newer plan.
    pub fn render_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent() != source.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        let scene = scene_from_resolved(source, plan.setup().print_mapping(), metadata)?;
        Self::render_scene(plan, &scene)
    }

    /// Resolve and rasterize a single planned page for the hardcopy dialog.
    /// The selected export/print format is intentionally ignored: preview is
    /// a non-authoritative view of the same sealed semantic plan.
    pub fn render_preview_page_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        zero_based_page: usize,
        dpi: u16,
    ) -> Result<HardcopyPreviewPage, HardcopyRenderError> {
        let mut pages = Self::render_preview_pages_resolved(
            plan,
            source,
            metadata,
            &[zero_based_page],
            dpi,
            || false,
        )?;
        pages
            .pop()
            .ok_or(HardcopyRenderError::InvalidPreviewPageBatch)
    }

    /// Resolve and validate one semantic scene, then rasterize a bounded set
    /// of preview pages. `cancelled` is checked between pages so a worker can
    /// retain the selected page while skipping speculative adjacent work.
    pub fn render_preview_pages_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        zero_based_pages: &[usize],
        dpi: u16,
        cancelled: impl Fn() -> bool,
    ) -> Result<Vec<HardcopyPreviewPage>, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent() != source.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        if !(36..=1_200).contains(&dpi) {
            return Err(HardcopyRenderError::InvalidPreviewDpi(dpi));
        }
        if zero_based_pages.is_empty()
            || zero_based_pages.len() > MAX_PREVIEW_BATCH_PAGES
            || (zero_based_pages.len() == 2 && zero_based_pages[0] == zero_based_pages[1])
        {
            return Err(HardcopyRenderError::InvalidPreviewPageBatch);
        }
        let pages = zero_based_pages
            .iter()
            .copied()
            .map(|index| {
                plan.pagination().pages().get(index).ok_or(
                    HardcopyRenderError::PreviewPageOutOfRange {
                        index,
                        page_count: plan.pagination().pages().len(),
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let scene = scene_from_resolved(source, plan.setup().print_mapping(), metadata)?;
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        validate_aggregate_pagination(plan, &scene)?;
        validate_decoration_capacity(plan, &scene)?;
        validate_render_budget(plan, &scene, pages.len())?;
        let mut largest_preview = 0_u64;
        for page in &pages {
            let (_, _, preview_pixels) = raster_dimensions(page, dpi)?;
            largest_preview = largest_preview.max(preview_pixels);
        }
        validate_raster_working_set(largest_preview, 1, 8)?;

        let mut rendered = Vec::with_capacity(pages.len());
        for (position, page) in pages.into_iter().enumerate() {
            if position != 0 && cancelled() {
                break;
            }
            let mut raster = rasterize_page(plan, &scene, page, dpi)?;
            let soft_proof_applied = plan.setup().render().soft_proof_print_safe_colors();
            if soft_proof_applied {
                apply_soft_proof_preview(&mut raster.rgba);
            }
            let digest = preview_content_digest(
                plan,
                source,
                page,
                dpi,
                raster.width,
                raster.height,
                &raster.rgba,
            );
            rendered.push(HardcopyPreviewPage {
                page_number: page.number(),
                coordinate: page.coordinate().to_owned(),
                width: raster.width,
                height: raster.height,
                dpi,
                rgba: raster.rgba,
                soft_proof_applied,
                digest,
            });
        }
        if rendered.is_empty() {
            return Err(HardcopyRenderError::PreviewPageOutOfRange {
                index: zero_based_pages[0],
                page_count: plan.pagination().pages().len(),
            });
        }
        Ok(rendered)
    }

    /// Render exact native-printer pages directly from the authenticated
    /// semantic source, closing the adapter gap without exposing a mutable
    /// intermediate scene.
    pub fn render_printer_pages_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent() != source.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        let scene = scene_from_resolved(source, plan.setup().print_mapping(), metadata)?;
        Self::render_printer_scene(plan, &scene, device_dpi)
    }

    /// Raw scene rendering exists only for module and adapter contract tests.
    /// Production callers must use [`Self::render_resolved`] so the semantic
    /// document authority is checked against the immutable plan.
    #[cfg(test)]
    pub(crate) fn render(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        Self::render_scene(plan, scene)
    }

    fn render_scene(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        if scene.extent != plan.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        validate_aggregate_pagination(plan, scene)?;
        validate_decoration_capacity(plan, scene)?;
        if matches!(
            plan.setup().render().background(),
            BackgroundMode::Transparent
        ) && plan.setup().render().format() == OutputFormat::NativePrinter
        {
            return Err(HardcopyRenderError::TransparentPrinterPage);
        }
        let format = plan.setup().render().format();
        let page_count = u32::try_from(plan.pagination().pages().len()).map_err(|_| {
            HardcopyRenderError::ResourceLimit {
                scope: "pages",
                maximum: u32::MAX as u64,
            }
        })?;
        validate_render_budget(plan, scene, page_count as usize)?;
        let (parts, conformance) = match format {
            OutputFormat::BrowserPrintDocument => {
                if scene_contains_text(scene) && !plan.setup().render().fonts().embed_fonts() {
                    return Err(HardcopyRenderError::BrowserTextRequiresEmbeddedFonts);
                }
                (
                    vec![make_part(
                        render_browser_print_html(plan, scene)?.into_bytes(),
                        "text/html",
                        "html",
                        "hardcopy-print.html".to_owned(),
                        1,
                        page_count,
                    )?],
                    None,
                )
            }
            OutputFormat::SvgVector => {
                if !plan.setup().render().fonts().preserve_searchable_text()
                    && scene_contains_text(scene)
                {
                    return Err(HardcopyRenderError::SvgTextOutliningUnsupported);
                }
                let mut parts = Vec::with_capacity(plan.pagination().pages().len());
                for page in plan.pagination().pages() {
                    parts.push(make_part(
                        render_page_svg(plan, scene, page)?.into_bytes(),
                        "image/svg+xml",
                        "svg",
                        format!("page-{:04}-{}.svg", page.number(), page.coordinate()),
                        page.number(),
                        1,
                    )?);
                }
                (parts, None)
            }
            OutputFormat::Png { dpi } => {
                let (total, largest_page) = aggregate_raster_pixels(plan, dpi)?;
                if total > MAX_RASTER_PIXELS_TOTAL {
                    return Err(HardcopyRenderError::ResourceLimit {
                        scope: "aggregate raster pixels",
                        maximum: MAX_RASTER_PIXELS_TOTAL,
                    });
                }
                validate_raster_working_set(largest_page, 1, 8)?;
                let mut parts = Vec::with_capacity(plan.pagination().pages().len());
                for page in plan.pagination().pages() {
                    let pixels = rasterize_page(plan, scene, page, dpi)?;
                    parts.push(make_part(
                        encode_png(&pixels, dpi)?,
                        "image/png",
                        "png",
                        format!("page-{:04}-{}.png", page.number(), page.coordinate()),
                        page.number(),
                        1,
                    )?);
                }
                (parts, None)
            }
            OutputFormat::Tiff { dpi } => (
                vec![make_part(
                    render_tiff(plan, scene, dpi)?,
                    "image/tiff",
                    "tiff",
                    "hardcopy.tiff".to_owned(),
                    1,
                    page_count,
                )?],
                None,
            ),
            OutputFormat::PdfA => (
                vec![make_part(
                    render_pdf(plan, scene, true)?,
                    "application/pdf",
                    "pdf",
                    "hardcopy.pdf".to_owned(),
                    1,
                    page_count,
                )?],
                Some(PdfConformance::PdfA2bValidated),
            ),
            OutputFormat::PdfVector | OutputFormat::NativePrinter => (
                vec![make_part(
                    render_pdf(plan, scene, false)?,
                    "application/pdf",
                    "pdf",
                    "hardcopy.pdf".to_owned(),
                    1,
                    page_count,
                )?],
                Some(PdfConformance::StandardPdf),
            ),
        };
        let publication_bytes = parts.iter().try_fold(0_u64, |sum, part| {
            sum.checked_add(part.bytes.len() as u64)
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "aggregate publication bytes",
                    maximum: MAX_PUBLICATION_BYTES,
                })
        })?;
        if publication_bytes > MAX_PUBLICATION_BYTES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            });
        }
        let digest = publication_digest(format, page_count, &parts);
        Ok(RenderedHardcopyPublication {
            parts,
            digest,
            format,
            page_count,
            pdf_conformance: conformance,
        })
    }

    /// Render exact physical pages for a native printer adapter at the
    /// selected device resolution. This uses the same canonical SVG scene and
    /// pagination as file publication; platform code never has to parse PDF or
    /// duplicate layout logic.
    #[cfg(test)]
    pub(crate) fn render_printer_pages(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        Self::render_printer_scene(plan, scene, device_dpi)
    }

    fn render_printer_scene(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        let RenderTarget::SystemPrinter { job, .. } = plan.setup().render().target() else {
            return Err(HardcopyRenderError::PrinterRasterRequiresNativePlan);
        };
        if plan.setup().render().format() != OutputFormat::NativePrinter {
            return Err(HardcopyRenderError::PrinterRasterRequiresNativePlan);
        }
        if !(72..=9_600).contains(&device_dpi) {
            return Err(HardcopyRenderError::InvalidPrinterDpi(device_dpi));
        }
        if device_dpi != job.resolution_dpi() {
            return Err(HardcopyRenderError::PrinterRasterDpiMismatch);
        }
        if plan.setup().render().background() == BackgroundMode::Transparent {
            return Err(HardcopyRenderError::TransparentPrinterPage);
        }
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        if scene.extent != plan.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        validate_aggregate_pagination(plan, scene)?;
        validate_decoration_capacity(plan, scene)?;
        validate_render_budget(plan, scene, plan.pagination().pages().len())?;
        let first_page = plan
            .pagination()
            .pages()
            .first()
            .ok_or(HardcopyRenderError::AggregatePaginationMismatch)?;
        if plan
            .pagination()
            .pages()
            .iter()
            .any(|page| page.geometry() != first_page.geometry())
        {
            return Err(HardcopyRenderError::PrinterMixedPageGeometryUnsupported);
        }
        let (raster_width, raster_height) = job.raster_geometry().physical_size_px();
        validate_driver_printable_geometry(
            first_page,
            raster_width,
            raster_height,
            job.raster_geometry().printable_rect_px(),
        )?;
        let per_page_pixels = u64::from(raster_width)
            .checked_mul(u64::from(raster_height))
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        if per_page_pixels > MAX_RASTER_PIXELS_PER_PAGE {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "printer raster pixels per page",
                maximum: MAX_RASTER_PIXELS_PER_PAGE,
            });
        }
        let total_pixels = per_page_pixels
            .checked_mul(plan.pagination().pages().len() as u64)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        if total_pixels > MAX_RASTER_PIXELS_TOTAL {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate printer raster pixels",
                maximum: MAX_RASTER_PIXELS_TOTAL,
            });
        }
        validate_printer_raster_working_set(
            per_page_pixels,
            plan.pagination().pages().len() as u64,
        )?;
        let mut pages = Vec::with_capacity(plan.pagination().pages().len());
        let mut publication_digest = Sha256::new();
        publication_digest.update(b"rspice-hardcopy-printer-pages-v1");
        publication_digest.update(device_dpi.to_le_bytes());
        for page in plan.pagination().pages() {
            let raster = rasterize_page_at_dimensions(
                plan,
                scene,
                page,
                device_dpi,
                raster_width,
                raster_height,
            )?;
            if raster.rgba.chunks_exact(4).any(|pixel| pixel[3] != u8::MAX) {
                return Err(HardcopyRenderError::TransparentPrinterPage);
            }
            let digest = ContentDigest::from_bytes(Sha256::digest(&raster.rgba).into());
            publication_digest.update(page.number().to_le_bytes());
            publication_digest.update(raster.width.to_le_bytes());
            publication_digest.update(raster.height.to_le_bytes());
            publication_digest.update(&raster.rgba);
            pages.push(PrinterRasterPage {
                page_number: page.number(),
                coordinate: page.coordinate().to_owned(),
                width: raster.width,
                height: raster.height,
                dpi: device_dpi,
                rgba: raster.rgba,
                digest,
            });
        }
        Ok(RenderedPrinterPages {
            pages,
            digest: ContentDigest::from_bytes(publication_digest.finalize().into()),
        })
    }
}

fn validate_driver_printable_geometry(
    page: &PreviewPage,
    raster_width: u32,
    raster_height: u32,
    driver_printable: (u32, u32, u32, u32),
) -> Result<(), HardcopyRenderError> {
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let (physical_width, physical_height) = geometry.physical_size();
    let map_floor =
        |position: u64, pixels: u32, physical: u64| -> Result<u64, HardcopyRenderError> {
            u64::try_from(
                u128::from(position)
                    .checked_mul(u128::from(pixels))
                    .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
                    / u128::from(physical),
            )
            .map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
        };
    let map_ceil =
        |position: u64, pixels: u32, physical: u64| -> Result<u64, HardcopyRenderError> {
            u64::try_from(
                u128::from(position)
                    .checked_mul(u128::from(pixels))
                    .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
                    .div_ceil(u128::from(physical)),
            )
            .map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
        };
    let right_um = printable
        .x
        .micrometres()
        .checked_add(printable.width.micrometres())
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let bottom_um = printable
        .y
        .micrometres()
        .checked_add(printable.height.micrometres())
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let planned_left = map_floor(
        printable.x.micrometres(),
        raster_width,
        physical_width.micrometres(),
    )?;
    let planned_top = map_floor(
        printable.y.micrometres(),
        raster_height,
        physical_height.micrometres(),
    )?;
    let planned_right = map_ceil(right_um, raster_width, physical_width.micrometres())?;
    let planned_bottom = map_ceil(bottom_um, raster_height, physical_height.micrometres())?;
    let (driver_x, driver_y, driver_width, driver_height) = driver_printable;
    let driver_right = u64::from(driver_x)
        .checked_add(u64::from(driver_width))
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let driver_bottom = u64::from(driver_y)
        .checked_add(u64::from(driver_height))
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    if planned_left < u64::from(driver_x)
        || planned_top < u64::from(driver_y)
        || planned_right > driver_right
        || planned_bottom > driver_bottom
    {
        return Err(HardcopyRenderError::PrinterPrintableGeometryMismatch);
    }
    Ok(())
}

fn render_browser_print_html(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<String, HardcopyRenderError> {
    let mm = |length: Length| length.micrometres() as f64 / 1_000.0;
    let first_orientation = plan
        .pagination()
        .pages()
        .first()
        .map(|page| page.geometry().orientation());
    let orientation = if plan
        .pagination()
        .pages()
        .iter()
        .all(|page| Some(page.geometry().orientation()) == first_orientation)
    {
        match first_orientation {
            Some(ResolvedOrientation::Landscape) => "landscape",
            Some(ResolvedOrientation::Portrait) => "portrait",
            None => "unknown",
        }
    } else {
        "mixed"
    };
    let capacity = checked_vector_capacity(
        4_096,
        plan.pagination().pages().len(),
        32_768,
        scene.primitives.len(),
        96,
    )?;
    let mut html = String::with_capacity(capacity);
    html.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; style-src 'unsafe-inline'; font-src data:; img-src data:; base-uri 'none'; form-action 'none'\">");
    write!(
        html,
        "<meta name=\"rspice-plan-digest\" content=\"{}\"><meta name=\"rspice-source-digest\" content=\"{}\"><meta name=\"rspice-page-count\" content=\"{}\"><title>",
        plan.content_digest(),
        plan.source().content_digest(),
        plan.pagination().pages().len()
    )
    .expect("write to string");
    escape_xml_into(scene.metadata.title(), &mut html);
    html.push_str("</title><style>");
    html.push_str("*{box-sizing:border-box}html,body{margin:0;padding:0;background:#fff}body{-webkit-print-color-adjust:exact;print-color-adjust:exact}");
    html.push_str(".rspice-print-page{position:relative;break-after:page;page-break-after:always;overflow:hidden}.rspice-print-page:last-child{break-after:auto;page-break-after:auto}.rspice-print-page>svg{position:absolute;inset:0;max-width:none;max-height:none;display:block}");
    for page in plan.pagination().pages() {
        let (width, height) = page.geometry().physical_size();
        let width_mm = mm(width);
        let height_mm = mm(height);
        write!(
            html,
            "@page rspice-page-{}{{size:{width_mm:.3}mm {height_mm:.3}mm;margin:0;}}.rspice-print-page[data-page=\"{}\"]{{page:rspice-page-{};width:{width_mm:.3}mm;height:{height_mm:.3}mm}}.rspice-print-page[data-page=\"{}\"]>svg{{width:{width_mm:.3}mm;height:{height_mm:.3}mm}}",
            page.number(),
            page.number(),
            page.number(),
            page.number(),
        )
        .expect("write to string");
    }
    html.push_str("@media screen{body{padding:12mm;background:#c7cbd0}.rspice-print-page{margin:0 auto 12mm;box-shadow:0 2mm 6mm rgba(0,0,0,.28);background:#fff}}@media print{body{background:transparent}.rspice-print-page{margin:0}}</style></head>");
    write!(
        html,
        "<body data-rspice-plan=\"{}\" data-rspice-source=\"{}\" data-rspice-orientation=\"{orientation}\">",
        plan.content_digest(),
        plan.source().content_digest()
    )
    .expect("write to string");
    for page in plan.pagination().pages() {
        write!(
            html,
            "<section class=\"rspice-print-page\" data-page=\"{}\" data-coordinate=\"{}\" aria-label=\"Page {} of {}\">",
            page.number(),
            page.coordinate(),
            page.number(),
            plan.pagination().pages().len()
        )
        .expect("write to string");
        html.push_str(&render_page_svg(plan, scene, page)?);
        html.push_str("</section>");
    }
    // Deliberately script-free. The authenticated browser-print adapter owns
    // the popup load listener and invokes `print()` only after this complete,
    // self-contained document has loaded.
    html.push_str("</body></html>");
    Ok(html)
}

fn validate_plan_font_coverage(plan: &HardcopyPlan) -> Result<(), HardcopyRenderError> {
    let coverage = FontCoverage::load()?;
    if let Watermark::Custom(text) = plan.setup().decorations().watermark() {
        coverage.validate_text(SceneFont::SansSemibold, text, "custom watermark")?;
    }
    // Validate renderer-authored punctuation up front. Authored engineering
    // symbols are checked against their selected face as each scene text node
    // is validated; unsupported symbols fail precisely instead of making all
    // unrelated publications unavailable.
    const FIXED_PUBLICATION_GLYPHS: &str = "rev · page / source plan";
    coverage.validate_text(
        SceneFont::Sans,
        FIXED_PUBLICATION_GLYPHS,
        "fixed publication text",
    )?;
    coverage.validate_text(
        SceneFont::Monospace,
        FIXED_PUBLICATION_GLYPHS,
        "fixed publication text",
    )
}

fn validate_decoration_capacity(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<(), HardcopyRenderError> {
    let decorations = plan.setup().decorations();
    if decorations.includes_header() && scene.metadata.header_lines.len() > 1 {
        return Err(HardcopyRenderError::DecorationOverflow {
            decoration: "header",
            actual: scene.metadata.header_lines.len(),
            maximum: 1,
        });
    }
    if decorations.includes_provenance() && scene.metadata.provenance_lines.len() > 1 {
        return Err(HardcopyRenderError::DecorationOverflow {
            decoration: "provenance",
            actual: scene.metadata.provenance_lines.len(),
            maximum: 1,
        });
    }
    if decorations.includes_legends() {
        for page in plan.pagination().pages() {
            let geometry = page.geometry();
            let columns = geometry.printable_rect().width.micrometres() / LEGEND_COLUMN_UM;
            let usable_height = geometry
                .legend_band()
                .micrometres()
                .saturating_sub(LEGEND_VERTICAL_PADDING_UM);
            let rows = usable_height / LEGEND_ROW_UM;
            let maximum = usize::try_from(columns.saturating_mul(rows)).unwrap_or(usize::MAX);
            if scene.legend.len() > maximum {
                return Err(HardcopyRenderError::DecorationOverflow {
                    decoration: "legend",
                    actual: scene.legend.len(),
                    maximum,
                });
            }
        }
    }
    Ok(())
}

fn make_part(
    bytes: Vec<u8>,
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
) -> Result<RenderedHardcopyPart, HardcopyRenderError> {
    if bytes.is_empty() || bytes.len() > MAX_ARTIFACT_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "artifact part bytes",
            maximum: MAX_ARTIFACT_BYTES as u64,
        });
    }
    let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
    Ok(RenderedHardcopyPart {
        bytes,
        digest,
        media_type,
        filename_extension,
        suggested_filename,
        first_page,
        page_count,
    })
}

fn publication_digest(
    format: OutputFormat,
    page_count: u32,
    parts: &[RenderedHardcopyPart],
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-publication-v1");
    digest.update(serde_json::to_vec(&format).expect("OutputFormat serialization cannot fail"));
    digest.update(page_count.to_le_bytes());
    digest.update((parts.len() as u64).to_le_bytes());
    for part in parts {
        digest.update(part.first_page.to_le_bytes());
        digest.update(part.page_count.to_le_bytes());
        digest.update((part.media_type.len() as u64).to_le_bytes());
        digest.update(part.media_type.as_bytes());
        digest.update((part.suggested_filename.len() as u64).to_le_bytes());
        digest.update(part.suggested_filename.as_bytes());
        digest.update((part.bytes.len() as u64).to_le_bytes());
        digest.update(&part.bytes);
    }
    ContentDigest::from_bytes(digest.finalize().into())
}

fn validate_lines(
    field: &'static str,
    lines: &[String],
    maximum_lines: usize,
    maximum_line_bytes: usize,
) -> Result<(), HardcopyRenderError> {
    if lines.len() > maximum_lines {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: field,
            maximum: maximum_lines as u64,
        });
    }
    for line in lines {
        validate_text(field, line, maximum_line_bytes)?;
    }
    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HardcopyRenderError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        Err(HardcopyRenderError::InvalidText { field, maximum })
    } else {
        Ok(())
    }
}

fn png_dimensions(payload: &[u8]) -> Result<(u32, u32), HardcopyRenderError> {
    if payload.is_empty() || payload.len() > MAX_EMBEDDED_FIGURE_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure bytes",
            maximum: MAX_EMBEDDED_FIGURE_BYTES as u64,
        });
    }
    let reader = png::Decoder::new(Cursor::new(payload))
        .read_info()
        .map_err(|error| HardcopyRenderError::InvalidEmbeddedFigure(error.to_string()))?;
    let info = reader.info();
    let pixels = u64::from(info.width)
        .checked_mul(u64::from(info.height))
        .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
    if info.width == 0 || info.height == 0 || pixels > MAX_RASTER_PIXELS_PER_PAGE {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure pixels",
            maximum: MAX_RASTER_PIXELS_PER_PAGE,
        });
    }
    Ok((info.width, info.height))
}

fn validate_point(point: ScenePoint, extent: ContentExtent) -> Result<(), HardcopyRenderError> {
    if point.x > extent.width() || point.y > extent.height() {
        Err(HardcopyRenderError::PrimitiveOutsideExtent)
    } else {
        Ok(())
    }
}

fn validate_rect(rect: SceneRect, extent: ContentExtent) -> Result<(), HardcopyRenderError> {
    let right = rect
        .x
        .micrometres()
        .checked_add(rect.width.micrometres())
        .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
    let bottom = rect
        .y
        .micrometres()
        .checked_add(rect.height.micrometres())
        .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
    if right > extent.width().micrometres() || bottom > extent.height().micrometres() {
        Err(HardcopyRenderError::PrimitiveOutsideExtent)
    } else {
        Ok(())
    }
}

fn validate_primitive(
    primitive: &ScenePrimitive,
    extent: ContentExtent,
    text_bytes: &mut usize,
    coverage: &FontCoverage,
) -> Result<(), HardcopyRenderError> {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            validate_point(*from, extent)?;
            validate_point(*to, extent)?;
            validate_stroke(*stroke)
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            if points.len() < if *closed { 3 } else { 2 } {
                return Err(HardcopyRenderError::InvalidPolyline);
            }
            for point in points {
                validate_point(*point, extent)?;
            }
            validate_stroke(*stroke)?;
            validate_fill(*fill)
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            validate_rect(*rect, extent)?;
            if let Some(stroke) = stroke {
                validate_stroke(*stroke)?;
            }
            validate_fill(*fill)
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let right = center
                .x
                .micrometres()
                .checked_add(radius.micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            let bottom = center
                .y
                .micrometres()
                .checked_add(radius.micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            if *radius == Length::ZERO
                || center.x.micrometres() < radius.micrometres()
                || center.y.micrometres() < radius.micrometres()
                || right > extent.width().micrometres()
                || bottom > extent.height().micrometres()
            {
                return Err(HardcopyRenderError::PrimitiveOutsideExtent);
            }
            if let Some(stroke) = stroke {
                validate_stroke(*stroke)?;
            }
            validate_fill(*fill)
        }
        ScenePrimitive::RasterImage {
            rect,
            png,
            content_digest,
            alternative_text,
        } => {
            validate_rect(*rect, extent)?;
            validate_text("figure alternative text", alternative_text, 16_384)?;
            coverage.validate_text(SceneFont::Sans, alternative_text, "figure alternative text")?;
            if png.is_empty() || png.len() > MAX_EMBEDDED_FIGURE_BYTES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "embedded figure bytes",
                    maximum: MAX_EMBEDDED_FIGURE_BYTES as u64,
                });
            }
            let observed = ContentDigest::from_bytes(Sha256::digest(png).into());
            if observed != *content_digest {
                return Err(HardcopyRenderError::EmbeddedFigureDigestMismatch);
            }
            let _ = png_dimensions(png)?;
            *text_bytes = text_bytes.checked_add(alternative_text.len()).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "scene text bytes",
                    maximum: MAX_SCENE_TEXT_BYTES as u64,
                },
            )?;
            Ok(())
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            ..
        } => {
            validate_point(*origin, extent)?;
            validate_text("scene text", text, 65_536)?;
            if *size == Length::ZERO || size.micrometres() > 100_000 {
                return Err(HardcopyRenderError::InvalidTextSize);
            }
            coverage.validate_text(*font, text, "scene text")?;
            *text_bytes =
                text_bytes
                    .checked_add(text.len())
                    .ok_or(HardcopyRenderError::ResourceLimit {
                        scope: "scene text bytes",
                        maximum: MAX_SCENE_TEXT_BYTES as u64,
                    })?;
            Ok(())
        }
    }
}

fn validate_fill(fill: Option<SceneFill>) -> Result<(), HardcopyRenderError> {
    if let Some(SceneFill::CrossHatch {
        line_width,
        spacing,
        ..
    }) = fill
    {
        if line_width == Length::ZERO
            || spacing == Length::ZERO
            || line_width.micrometres() > 25_000
            || spacing.micrometres() > 25_000
            || line_width > spacing
        {
            return Err(HardcopyRenderError::InvalidCrossHatch);
        }
    }
    Ok(())
}

fn primitive_vertex_count(primitive: &ScenePrimitive) -> usize {
    match primitive {
        ScenePrimitive::Line { .. } => 2,
        ScenePrimitive::Polyline { points, .. } => points.len(),
        ScenePrimitive::Rect { .. } => 4,
        ScenePrimitive::Circle { .. } => 4,
        ScenePrimitive::RasterImage { .. } => 4,
        ScenePrimitive::Text { .. } => 1,
    }
}

fn primitive_hatch_line_count(primitive: &ScenePrimitive) -> u64 {
    let Some(SceneFill::CrossHatch { spacing, .. }) = primitive_fill(primitive) else {
        return 0;
    };
    let span = match primitive {
        ScenePrimitive::Polyline { points, .. } => {
            let minimum_x = points.iter().map(|point| point.x.micrometres()).min();
            let maximum_x = points.iter().map(|point| point.x.micrometres()).max();
            let minimum_y = points.iter().map(|point| point.y.micrometres()).min();
            let maximum_y = points.iter().map(|point| point.y.micrometres()).max();
            match (minimum_x, maximum_x, minimum_y, maximum_y) {
                (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
                    max_x.saturating_sub(min_x) + max_y.saturating_sub(min_y)
                }
                _ => 0,
            }
        }
        ScenePrimitive::Rect { rect, .. } => rect.width.micrometres() + rect.height.micrometres(),
        ScenePrimitive::Circle { radius, .. } => radius.micrometres().saturating_mul(4),
        ScenePrimitive::Line { .. }
        | ScenePrimitive::RasterImage { .. }
        | ScenePrimitive::Text { .. } => 0,
    };
    span.div_ceil(spacing.micrometres()).saturating_mul(2)
}

fn validate_stroke(stroke: StrokeStyle) -> Result<(), HardcopyRenderError> {
    if stroke.width == Length::ZERO || stroke.width.micrometres() > 100_000 {
        Err(HardcopyRenderError::InvalidStrokeWidth(stroke.width))
    } else {
        Ok(())
    }
}

fn scene_contains_text(scene: &HardcopyScene) -> bool {
    scene
        .primitives
        .iter()
        .any(|primitive| matches!(primitive, ScenePrimitive::Text { .. }))
        || !scene.legend.is_empty()
        || !scene.metadata.header_lines.is_empty()
        || !scene.metadata.provenance_lines.is_empty()
}

fn validate_render_budget(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page_count: usize,
) -> Result<(), HardcopyRenderError> {
    let mut vertices = 0_u64;
    let mut hatch_lines = 0_u64;
    let mut text_bytes = 0_u64;
    let mut image_bytes = 0_u64;
    let mut image_pixels = 0_u64;
    for primitive in &scene.primitives {
        vertices = vertices
            .checked_add(primitive_vertex_count(primitive) as u64)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
        hatch_lines = hatch_lines
            .checked_add(primitive_hatch_line_count(primitive))
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
        if let ScenePrimitive::Text { text, .. } = primitive {
            text_bytes = text_bytes.checked_add(text.len() as u64).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "render work units",
                    maximum: MAX_RENDER_WORK_UNITS,
                },
            )?;
        } else if let ScenePrimitive::RasterImage { png, .. } = primitive {
            image_bytes = image_bytes.checked_add(png.len() as u64).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "render work units",
                    maximum: MAX_RENDER_WORK_UNITS,
                },
            )?;
            let (width, height) = png_dimensions(png)?;
            image_pixels = image_pixels
                .checked_add(u64::from(width).saturating_mul(u64::from(height)))
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "render work units",
                    maximum: MAX_RENDER_WORK_UNITS,
                })?;
        }
    }
    for entry in &scene.legend {
        text_bytes = text_bytes.checked_add(entry.label.len() as u64).ok_or(
            HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            },
        )?;
    }
    let scene_units = (scene.primitives.len() as u64)
        .checked_add(vertices)
        .and_then(|value| value.checked_add(hatch_lines.saturating_mul(2)))
        .and_then(|value| value.checked_add(text_bytes.div_ceil(16)))
        .and_then(|value| value.checked_add(image_pixels.div_ceil(64)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "render work units",
            maximum: MAX_RENDER_WORK_UNITS,
        })?;
    let total_work =
        scene_units
            .checked_mul(page_count as u64)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
    if total_work > MAX_RENDER_WORK_UNITS {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "render work units",
            maximum: MAX_RENDER_WORK_UNITS,
        });
    }
    let decoded_image_bytes =
        image_pixels
            .checked_mul(4)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "embedded figure decoded bytes",
                maximum: MAX_RASTER_WORKING_BYTES,
            })?;
    if decoded_image_bytes > MAX_RASTER_WORKING_BYTES / 2 {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure decoded bytes",
            maximum: MAX_RASTER_WORKING_BYTES / 2,
        });
    }

    let per_page = 32_768_u64
        .checked_add((scene.primitives.len() as u64).saturating_mul(192))
        .and_then(|value| value.checked_add(vertices.saturating_mul(40)))
        .and_then(|value| value.checked_add(text_bytes.saturating_mul(6)))
        .and_then(|value| value.checked_add(image_bytes.saturating_mul(2)))
        .and_then(|value| value.checked_add((scene.legend.len() as u64).saturating_mul(320)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    let embedded_font_bytes = if plan.setup().render().fonts().embed_fonts() {
        (IBM_PLEX_SANS_REGULAR.len() + IBM_PLEX_SANS_SEMIBOLD.len() + IBM_PLEX_MONO_REGULAR.len())
            as u64
            * 2
    } else {
        0
    };
    let format = plan.setup().render().format();
    let font_instances = if matches!(
        format,
        OutputFormat::SvgVector | OutputFormat::BrowserPrintDocument
    ) {
        page_count as u64
    } else {
        u64::from(page_count != 0)
    };
    let estimated_vector_bytes = per_page
        .checked_mul(page_count as u64)
        .and_then(|value| value.checked_add(embedded_font_bytes.saturating_mul(font_instances)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    if estimated_vector_bytes > MAX_ESTIMATED_VECTOR_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        });
    }
    Ok(())
}

fn validate_raster_working_set(
    pixels_per_page: u64,
    retained_pages: u64,
    bytes_per_pixel_budget: u64,
) -> Result<(), HardcopyRenderError> {
    let bytes = pixels_per_page
        .checked_mul(retained_pages)
        .and_then(|pixels| pixels.checked_mul(bytes_per_pixel_budget))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        })?;
    if bytes > MAX_RASTER_WORKING_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        });
    }
    Ok(())
}

fn validate_printer_raster_working_set(
    pixels_per_page: u64,
    page_count: u64,
) -> Result<(), HardcopyRenderError> {
    // Every returned page remains retained as RGBA (4 B/px), while the
    // current SVG parse/raster and the native adapter's BGRX spool conversion
    // require another conservative 12 B/px at peak.
    let bytes_per_pixel = page_count
        .checked_mul(4)
        .and_then(|retained| retained.checked_add(12))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        })?;
    validate_raster_working_set(pixels_per_page, 1, bytes_per_pixel)
}

fn checked_vector_capacity(
    base: usize,
    page_count: usize,
    per_page: usize,
    primitive_count: usize,
    per_primitive: usize,
) -> Result<usize, HardcopyRenderError> {
    let page_bytes = primitive_count
        .checked_mul(per_primitive)
        .and_then(|value| value.checked_add(per_page))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    let capacity = page_count
        .checked_mul(page_bytes)
        .and_then(|value| value.checked_add(base))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    if capacity as u64 > MAX_ESTIMATED_VECTOR_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        });
    }
    Ok(capacity)
}

#[derive(Debug, Clone, Copy)]
struct PageTransform {
    content_rect: PageRect,
    window: PageRect,
    scale: ScaleRatio,
}

fn validate_aggregate_pagination(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<(), HardcopyRenderError> {
    let planned = plan.pagination().sections();
    let rendered = &scene.aggregate_sections;
    if planned.len() != rendered.len() {
        return Err(HardcopyRenderError::AggregatePaginationMismatch);
    }
    for (planned, rendered) in planned.iter().copied().zip(rendered) {
        let (origin_x, origin_y) = planned.origin();
        if planned.ordinal() != rendered.ordinal
            || planned.content_digest() != rendered.content_digest
            || origin_x != rendered.origin.x
            || origin_y != rendered.origin.y
            || planned.extent() != rendered.extent
            || planned.page_break_before() != rendered.page_break_before
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
    }
    if rendered.is_empty() {
        if plan
            .pagination()
            .pages()
            .iter()
            .any(|page| page.section_ordinal() != 0)
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
        return Ok(());
    }
    for page in plan.pagination().pages() {
        if !rendered
            .iter()
            .any(|section| section.ordinal == page.section_ordinal())
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
    }
    Ok(())
}

fn page_primitives<'a>(
    scene: &'a HardcopyScene,
    page: &PreviewPage,
) -> Result<&'a [ScenePrimitive], HardcopyRenderError> {
    if scene.aggregate_sections.is_empty() {
        return Ok(&scene.primitives);
    }
    let section = scene
        .aggregate_sections
        .iter()
        .find(|section| section.ordinal == page.section_ordinal())
        .ok_or(HardcopyRenderError::AggregatePaginationMismatch)?;
    scene
        .primitives
        .get(section.primitive_start..section.primitive_end)
        .ok_or(HardcopyRenderError::AggregatePaginationMismatch)
}

impl PageTransform {
    fn point(self, point: ScenePoint) -> (f64, f64) {
        (
            self.axis(
                self.content_rect.x.micrometres(),
                point.x.micrometres(),
                self.window.x.micrometres(),
            ),
            self.axis(
                self.content_rect.y.micrometres(),
                point.y.micrometres(),
                self.window.y.micrometres(),
            ),
        )
    }

    fn axis(self, page_origin: u64, source: u64, window_origin: u64) -> f64 {
        page_origin as f64
            + source as f64 * self.scale.numerator() as f64 / self.scale.denominator() as f64
            - window_origin as f64
    }

    fn length(self, value: Length) -> f64 {
        value.micrometres() as f64 * self.scale.numerator() as f64 / self.scale.denominator() as f64
    }

    fn scale_factor(self) -> f64 {
        self.scale.numerator() as f64 / self.scale.denominator() as f64
    }
}

#[derive(Debug, Clone, Copy)]
struct ResolvedStroke {
    color: Rgb8,
    width_um: f64,
    pattern: StrokePattern,
    exact_dash_um: Option<(f64, f64)>,
    exact_dot_spacing_um: Option<f64>,
}

fn page_transform(page: &PreviewPage) -> PageTransform {
    PageTransform {
        content_rect: page.geometry().content_rect(),
        window: page.scaled_content_window(),
        scale: page.scale(),
    }
}

fn resolve_stroke(
    plan: &HardcopyPlan,
    transform: PageTransform,
    style: StrokeStyle,
) -> ResolvedStroke {
    let mapping = plan.setup().render().color_mapping();
    let pattern = if mapping == ColorMapping::GrayscaleWithDashMarkerRedundancy
        && style.pattern == StrokePattern::Solid
    {
        style
            .series_index
            .map(auto_trace_pattern)
            .unwrap_or(StrokePattern::Solid)
    } else {
        style.pattern
    };
    ResolvedStroke {
        color: resolve_color(plan, style.color),
        width_um: transform.length(style.width).max(80.0),
        pattern,
        exact_dash_um: style
            .exact_dash
            .map(|(dash, gap)| (transform.length(dash), transform.length(gap))),
        exact_dot_spacing_um: style
            .exact_dot_spacing
            .map(|spacing| transform.length(spacing)),
    }
}

fn auto_trace_pattern(index: u16) -> StrokePattern {
    match index % 4 {
        0 => StrokePattern::Solid,
        1 => StrokePattern::Dashed,
        2 => StrokePattern::Dotted,
        _ => StrokePattern::DashDot,
    }
}

fn base_semantic_color(color: SemanticColor) -> Rgb8 {
    const TRACES: [Rgb8; 10] = [
        Rgb8::new(0, 113, 188),
        Rgb8::new(213, 94, 0),
        Rgb8::new(0, 158, 115),
        Rgb8::new(204, 121, 167),
        Rgb8::new(230, 159, 0),
        Rgb8::new(86, 180, 233),
        Rgb8::new(111, 78, 161),
        Rgb8::new(0, 0, 0),
        Rgb8::new(56, 131, 74),
        Rgb8::new(180, 65, 65),
    ];
    match color {
        SemanticColor::Foreground => Rgb8::new(25, 32, 38),
        SemanticColor::Secondary => Rgb8::new(91, 102, 112),
        SemanticColor::Grid => Rgb8::new(181, 188, 194),
        SemanticColor::Accent => Rgb8::new(196, 139, 0),
        SemanticColor::Warning => Rgb8::new(190, 67, 54),
        SemanticColor::Success => Rgb8::new(0, 125, 82),
        SemanticColor::Trace(index) => TRACES[usize::from(index) % TRACES.len()],
        SemanticColor::Exact(value) => value,
    }
}

fn resolve_color(plan: &HardcopyPlan, color: SemanticColor) -> Rgb8 {
    let source = if plan.setup().render().background() == BackgroundMode::WorkspaceBackground {
        match color {
            SemanticColor::Foreground => Rgb8::new(224, 230, 234),
            SemanticColor::Secondary => Rgb8::new(166, 176, 184),
            SemanticColor::Grid => Rgb8::new(66, 78, 86),
            _ => base_semantic_color(color),
        }
    } else {
        base_semantic_color(color)
    };
    match plan.setup().render().color_mapping() {
        ColorMapping::ScreenColors => source,
        ColorMapping::PrintSafeEngineeringPalette => print_safe(source),
        ColorMapping::GrayscaleWithDashMarkerRedundancy => {
            let luminance = ((u32::from(source.red) * 54
                + u32::from(source.green) * 183
                + u32::from(source.blue) * 19)
                / 256) as u8;
            Rgb8::new(luminance, luminance, luminance)
        }
        ColorMapping::Monochrome => Rgb8::new(0, 0, 0),
    }
}

fn print_safe(color: Rgb8) -> Rgb8 {
    // Constrain saturated screen colors to a reproducible, contrast-safe sRGB
    // print palette. Exact black stays black; near-white ink is darkened.
    if color == Rgb8::new(0, 0, 0) {
        return color;
    }
    let clamp = |value: u8| value.clamp(18, 220);
    let mut resolved = Rgb8::new(clamp(color.red), clamp(color.green), clamp(color.blue));
    let luminance = (u32::from(resolved.red) * 54
        + u32::from(resolved.green) * 183
        + u32::from(resolved.blue) * 19)
        / 256;
    if luminance > 196 {
        resolved.red = resolved.red.saturating_sub(42);
        resolved.green = resolved.green.saturating_sub(42);
        resolved.blue = resolved.blue.saturating_sub(42);
    }
    resolved
}

fn background_color(plan: &HardcopyPlan) -> Option<Rgb8> {
    match plan.setup().render().background() {
        BackgroundMode::White => Some(Rgb8::new(255, 255, 255)),
        BackgroundMode::WorkspaceBackground => Some(Rgb8::new(17, 24, 28)),
        BackgroundMode::Transparent => None,
    }
}

fn svg_color(color: Rgb8) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}

fn render_page_svg(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) -> Result<String, HardcopyRenderError> {
    let geometry = page.geometry();
    let (page_width, page_height) = geometry.physical_size();
    let width_um = page_width.micrometres();
    let height_um = page_height.micrometres();
    let width_mm = width_um as f64 / 1_000.0;
    let height_mm = height_um as f64 / 1_000.0;
    let clip = geometry.content_rect();
    let mut output = String::with_capacity(checked_vector_capacity(
        32_768,
        1,
        0,
        scene.primitives.len(),
        96,
    )?);
    let bleed_um = match plan.setup().physical_page().bleed() {
        Bleed::None => 0,
        Bleed::Uniform(value) => value.micrometres(),
    };
    write!(
        output,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width_mm:.3}mm\" height=\"{height_mm:.3}mm\" viewBox=\"0 0 {width_um} {height_um}\" data-rspice-bleed-um=\"{bleed_um}\" data-rspice-trim=\"{bleed_um} {bleed_um} {} {}\">",
        width_um.saturating_sub(bleed_um.saturating_mul(2)),
        height_um.saturating_sub(bleed_um.saturating_mul(2)),
    )
    .expect("write to string");
    output.push_str("<title>");
    escape_xml_into(scene.metadata.title(), &mut output);
    output.push_str("</title>");
    output.push_str("<metadata>");
    escape_xml_into(
        &format!(
            "RSpice hardcopy plan {}; source digest {}; plan digest {}",
            plan.id(),
            plan.source().content_digest(),
            plan.content_digest()
        ),
        &mut output,
    );
    output.push_str("</metadata>");
    if plan.setup().render().fonts().embed_fonts() {
        write_embedded_svg_fonts(&mut output);
    }
    write!(
        output,
        "<defs><clipPath id=\"content-clip\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",
        clip.x.micrometres(),
        clip.y.micrometres(),
        clip.width.micrometres(),
        clip.height.micrometres()
    )
    .expect("write to string");
    let primitives = page_primitives(scene, page)?;
    write_svg_hatch_defs(&mut output, plan, primitives, &scene.legend);
    if let Some(background) = background_color(plan) {
        write!(
            output,
            "<rect x=\"0\" y=\"0\" width=\"{width_um}\" height=\"{height_um}\" fill=\"{}\"/>",
            svg_color(background)
        )
        .expect("write to string");
    }
    output.push_str("<g clip-path=\"url(#content-clip)\">");
    let transform = page_transform(page);
    for primitive in primitives {
        write_svg_primitive(&mut output, plan, transform, primitive);
    }
    output.push_str("</g>");
    write_svg_decorations(&mut output, plan, scene, page);
    output.push_str("</svg>");
    Ok(output)
}

fn write_embedded_svg_fonts(output: &mut String) {
    let encoded = |bytes: &[u8]| base64::engine::general_purpose::STANDARD.encode(bytes);
    output.push_str("<style>");
    for (family, weight, bytes) in [
        ("RSpice Plex Sans", "400", IBM_PLEX_SANS_REGULAR),
        ("RSpice Plex Sans", "600", IBM_PLEX_SANS_SEMIBOLD),
        ("RSpice Plex Mono", "400", IBM_PLEX_MONO_REGULAR),
    ] {
        write!(
            output,
            "@font-face{{font-family:'{family}';font-style:normal;font-weight:{weight};src:url(data:font/ttf;base64,{}) format('truetype');}}",
            encoded(bytes)
        )
        .expect("write to string");
    }
    output.push_str("</style>");
}

fn write_svg_primitive(
    output: &mut String,
    plan: &HardcopyPlan,
    transform: PageTransform,
    primitive: &ScenePrimitive,
) {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            let (x1, y1) = transform.point(*from);
            let (x2, y2) = transform.point(*to);
            let stroke = resolve_stroke(plan, transform, *stroke);
            write!(
                output,
                "<line x1=\"{x1:.3}\" y1=\"{y1:.3}\" x2=\"{x2:.3}\" y2=\"{y2:.3}\""
            )
            .expect("write to string");
            write_svg_stroke(output, stroke);
            output.push_str(" fill=\"none\"/>");
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            output.push_str(if *closed {
                "<polygon points=\""
            } else {
                "<polyline points=\""
            });
            for (index, point) in points.iter().enumerate() {
                let (x, y) = transform.point(*point);
                if index != 0 {
                    output.push(' ');
                }
                write!(output, "{x:.3},{y:.3}").expect("write to string");
            }
            output.push('"');
            write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let width = transform.length(rect.width);
            let height = transform.length(rect.height);
            write!(
                output,
                "<rect x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\""
            )
            .expect("write to string");
            if let Some(stroke) = stroke {
                write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            } else {
                output.push_str(" stroke=\"none\"");
            }
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let (cx, cy) = transform.point(*center);
            let radius = transform.length(*radius);
            write!(
                output,
                "<circle cx=\"{cx:.3}\" cy=\"{cy:.3}\" r=\"{radius:.3}\""
            )
            .expect("write to string");
            if let Some(stroke) = stroke {
                write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            } else {
                output.push_str(" stroke=\"none\"");
            }
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::RasterImage {
            rect,
            png,
            content_digest,
            alternative_text,
        } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let width = transform.length(rect.width);
            let height = transform.length(rect.height);
            write!(
                output,
                "<image x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\" preserveAspectRatio=\"xMidYMid meet\" href=\"data:image/png;base64,{}\" data-content-digest=\"{}\" aria-label=\"",
                base64::engine::general_purpose::STANDARD.encode(png),
                content_digest,
            )
            .expect("write to string");
            escape_xml_into(alternative_text, output);
            output.push_str("\"/>");
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            color,
            anchor,
        } => {
            let (x, y) = transform.point(*origin);
            let size = transform.length(*size);
            let (family, weight) = svg_font(*font);
            let anchor = match anchor {
                TextAnchor::Start => "start",
                TextAnchor::Middle => "middle",
                TextAnchor::End => "end",
            };
            write!(
                output,
                "<text x=\"{x:.3}\" y=\"{y:.3}\" fill=\"{}\" font-family=\"{family}\" font-weight=\"{weight}\" font-size=\"{size:.3}\" text-anchor=\"{anchor}\">",
                svg_color(resolve_color(plan, *color))
            )
            .expect("write to string");
            escape_xml_into(text, output);
            output.push_str("</text>");
        }
    }
}

fn write_svg_stroke(output: &mut String, stroke: ResolvedStroke) {
    write!(
        output,
        " stroke=\"{}\" stroke-width=\"{:.3}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"",
        svg_color(stroke.color),
        stroke.width_um
    )
    .expect("write to string");
    let dash = if let Some((dash, gap)) = stroke.exact_dash_um {
        Some(format!("{dash:.3} {gap:.3}"))
    } else if let Some(spacing) = stroke.exact_dot_spacing_um {
        Some(format!("{:.3} {spacing:.3}", stroke.width_um))
    } else {
        svg_dash_pattern(stroke.pattern, stroke.width_um)
    };
    if let Some(dash) = dash {
        write!(output, " stroke-dasharray=\"{dash}\"").expect("write to string");
    }
}

fn svg_dash_pattern(pattern: StrokePattern, width: f64) -> Option<String> {
    match pattern {
        StrokePattern::Solid => None,
        StrokePattern::Dashed => Some(format!("{:.3} {:.3}", width * 6.0, width * 3.0)),
        StrokePattern::Dotted => Some(format!("{:.3} {:.3}", width, width * 2.5)),
        StrokePattern::DashDot => Some(format!(
            "{:.3} {:.3} {:.3} {:.3}",
            width * 6.0,
            width * 2.5,
            width,
            width * 2.5
        )),
    }
}

fn write_svg_hatch_defs(
    output: &mut String,
    plan: &HardcopyPlan,
    primitives: &[ScenePrimitive],
    legend: &[LegendEntry],
) {
    let mut seen = Vec::<String>::new();
    for fill in primitives
        .iter()
        .filter_map(primitive_fill)
        .chain(legend.iter().filter_map(|entry| entry.fill))
    {
        let SceneFill::CrossHatch {
            color,
            line_width,
            spacing,
        } = fill
        else {
            continue;
        };
        let id = hatch_pattern_id(fill);
        if seen.iter().any(|existing| existing == &id) {
            continue;
        }
        seen.push(id.clone());
        let tile = spacing.micrometres() as f64 * std::f64::consts::SQRT_2;
        let color = svg_color(resolve_color(plan, color));
        write!(
            output,
            "<defs><pattern id=\"{id}\" patternUnits=\"userSpaceOnUse\" width=\"{tile:.6}\" height=\"{tile:.6}\"><path d=\"M 0 0 L {tile:.6} {tile:.6} M 0 {tile:.6} L {tile:.6} 0\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{}\"/></pattern></defs>",
            line_width.micrometres(),
        )
        .expect("write to string");
    }
}

fn primitive_fill(primitive: &ScenePrimitive) -> Option<SceneFill> {
    match primitive {
        ScenePrimitive::Polyline { fill, .. }
        | ScenePrimitive::Rect { fill, .. }
        | ScenePrimitive::Circle { fill, .. } => *fill,
        ScenePrimitive::Line { .. }
        | ScenePrimitive::RasterImage { .. }
        | ScenePrimitive::Text { .. } => None,
    }
}

fn hatch_pattern_id(fill: SceneFill) -> String {
    match fill {
        SceneFill::CrossHatch {
            color,
            line_width,
            spacing,
        } => {
            let encoded = serde_json::to_vec(&color).expect("semantic color serialization");
            let digest = Sha256::digest(encoded);
            format!(
                "hatch-{:02x}{:02x}{:02x}{:02x}-{}-{}",
                digest[0],
                digest[1],
                digest[2],
                digest[3],
                line_width.micrometres(),
                spacing.micrometres()
            )
        }
        SceneFill::Solid { .. } => "solid".to_owned(),
    }
}

fn write_svg_fill(output: &mut String, plan: &HardcopyPlan, fill: Option<SceneFill>) {
    match fill {
        Some(SceneFill::Solid { color }) => write!(
            output,
            " fill=\"{}\"",
            svg_color(resolve_color(plan, color))
        )
        .expect("write to string"),
        Some(fill @ SceneFill::CrossHatch { .. }) => {
            write!(output, " fill=\"url(#{})\"", hatch_pattern_id(fill)).expect("write to string")
        }
        None => output.push_str(" fill=\"none\""),
    }
}

fn svg_font(font: SceneFont) -> (&'static str, &'static str) {
    match font {
        SceneFont::Sans => ("RSpice Plex Sans", "400"),
        SceneFont::SansSemibold => ("RSpice Plex Sans", "600"),
        SceneFont::Monospace => ("RSpice Plex Mono", "400"),
    }
}

fn escape_xml_into(value: &str, output: &mut String) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            character => output.push(character),
        }
    }
}

fn write_svg_decorations(
    output: &mut String,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let ink = svg_color(resolve_color(plan, SemanticColor::Foreground));
    let secondary = svg_color(resolve_color(plan, SemanticColor::Secondary));
    if plan.setup().decorations().includes_header() {
        let baseline = printable.y.micrometres() + DECORATION_TEXT_UM;
        let page_right = printable.x.micrometres() + printable.width.micrometres();
        write!(
            output,
            "<text x=\"{}\" y=\"{baseline}\" fill=\"{ink}\" font-family=\"RSpice Plex Sans\" font-weight=\"600\" font-size=\"{DECORATION_TEXT_UM}\">",
            printable.x.micrometres()
        )
        .expect("write to string");
        escape_xml_into(scene.metadata.title(), output);
        output.push_str("</text>");
        write!(
            output,
            "<text x=\"{page_right}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"RSpice Plex Mono\" font-size=\"{DECORATION_TEXT_UM}\" text-anchor=\"end\">"
        )
        .expect("write to string");
        escape_xml_into(
            &format!(
                "rev {} · page {} / {} · {}",
                plan.source().revision().get(),
                page.number(),
                plan.pagination().pages().len(),
                page.coordinate()
            ),
            output,
        );
        output.push_str("</text>");
        if let Some(line) = scene.metadata.header_lines.first() {
            let center = printable.x.micrometres() + printable.width.micrometres() / 2;
            write!(
                output,
                "<text x=\"{center}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"RSpice Plex Sans\" font-size=\"{DECORATION_TEXT_UM}\" text-anchor=\"middle\">"
            )
            .expect("write to string");
            escape_xml_into(line, output);
            output.push_str("</text>");
        }
    }
    if plan.setup().decorations().includes_provenance() {
        let baseline = printable.y.micrometres() + printable.height.micrometres() - 2_000;
        write!(
            output,
            "<text x=\"{}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"RSpice Plex Mono\" font-size=\"2200\">",
            printable.x.micrometres()
        )
        .expect("write to string");
        let text = scene
            .metadata
            .provenance_lines
            .first()
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "source {} · plan {}",
                    plan.source().content_digest(),
                    plan.content_digest()
                )
            });
        escape_xml_into(&text, output);
        output.push_str("</text>");
    }
    if plan.setup().decorations().includes_legends() && !scene.legend.is_empty() {
        let legend_band = geometry.legend_band().micrometres();
        let columns = printable.width.micrometres() / LEGEND_COLUMN_UM;
        let rows = legend_band.saturating_sub(LEGEND_VERTICAL_PADDING_UM) / LEGEND_ROW_UM;
        let legend_top = content
            .y
            .micrometres()
            .saturating_add(content.height.micrometres());
        let box_height = rows
            .saturating_mul(LEGEND_ROW_UM)
            .saturating_add(LEGEND_VERTICAL_PADDING_UM);
        write!(
            output,
            "<g data-rspice-decoration=\"legend\"><rect x=\"{}\" y=\"{legend_top}\" width=\"{}\" height=\"{box_height}\" fill=\"#ffffff\" fill-opacity=\"0.92\" stroke=\"{secondary}\" stroke-width=\"150\"/>",
            printable.x.micrometres(),
            printable.width.micrometres(),
        )
        .expect("write to string");
        for (index, entry) in scene.legend.iter().enumerate() {
            let index = index as u64;
            let column = index / rows.max(1);
            let row = index % rows.max(1);
            if column >= columns {
                break;
            }
            let x = printable
                .x
                .micrometres()
                .saturating_add(column.saturating_mul(LEGEND_COLUMN_UM))
                .saturating_add(2_000);
            let y = legend_top
                .saturating_add(3_500)
                .saturating_add(row.saturating_mul(LEGEND_ROW_UM));
            let stroke = resolve_stroke(plan, page_transform(page), entry.stroke);
            if entry.fill.is_some() {
                write!(
                    output,
                    "<rect x=\"{x}\" y=\"{}\" width=\"12000\" height=\"3000\"",
                    y.saturating_sub(2_000)
                )
                .expect("write to string");
                write_svg_stroke(output, stroke);
                write_svg_fill(output, plan, entry.fill);
                output.push_str("/>");
            } else {
                write!(
                    output,
                    "<line x1=\"{x}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\"",
                    x + 12_000
                )
                .expect("write to string");
                write_svg_stroke(output, stroke);
                output.push_str("/>");
            }
            write!(
                output,
                "<text x=\"{}\" y=\"{}\" fill=\"{ink}\" font-family=\"RSpice Plex Sans\" font-size=\"2600\">",
                x + 15_000,
                y + 900
            )
            .expect("write to string");
            escape_xml_into(&entry.label, output);
            output.push_str("</text>");
        }
        output.push_str("</g>");
    }
    write_svg_watermark(output, plan, scene, page);
    write_svg_trim_marks(output, plan, page);
    write_svg_registration_marks(output, plan, page);
}

fn write_svg_trim_marks(output: &mut String, plan: &HardcopyPlan, page: &PreviewPage) {
    let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() else {
        return;
    };
    let geometry = page.geometry();
    let (width, height) = geometry.physical_size();
    let inset = bleed.micrometres();
    let right = width.micrometres().saturating_sub(inset);
    let bottom = height.micrometres().saturating_sub(inset);
    let gap = (inset / 4).max(100);
    let length = (inset.saturating_mul(3) / 4).min(5_000).max(500);
    let color = svg_color(resolve_color(plan, SemanticColor::Foreground));
    write!(
        output,
        "<path data-rspice-decoration=\"trim-marks\" d=\"M {} {inset} H {} M {inset} {} V {} M {} {inset} H {} M {right} {} V {} M {} {bottom} H {} M {inset} {} V {} M {} {bottom} H {} M {right} {} V {}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"150\"/>",
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        right.saturating_add(gap),
        right.saturating_add(gap).saturating_add(length),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        bottom.saturating_add(gap),
        bottom.saturating_add(gap).saturating_add(length),
        right.saturating_add(gap),
        right.saturating_add(gap).saturating_add(length),
        bottom.saturating_add(gap),
        bottom.saturating_add(gap).saturating_add(length),
    )
    .expect("write to string");
}

fn write_svg_watermark(
    output: &mut String,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let text = match plan.setup().decorations().watermark() {
        Watermark::None => return,
        Watermark::Draft => "DRAFT",
        Watermark::Confidential => "CONFIDENTIAL",
        Watermark::Custom(text) => text,
    };
    let (width, height) = page.geometry().physical_size();
    let cx = width.micrometres() / 2;
    let cy = height.micrometres() / 2;
    let ink = svg_color(resolve_color(plan, SemanticColor::Secondary));
    let size = width.micrometres().min(height.micrometres()) / 10;
    write!(
        output,
        "<text x=\"{cx}\" y=\"{cy}\" fill=\"{ink}\" fill-opacity=\"0.16\" font-family=\"RSpice Plex Sans\" font-weight=\"600\" font-size=\"{size}\" text-anchor=\"middle\" transform=\"rotate(-35 {cx} {cy})\">"
    )
    .expect("write to string");
    escape_xml_into(text, output);
    output.push_str("</text>");
    let _ = scene;
}

fn write_svg_registration_marks(output: &mut String, plan: &HardcopyPlan, page: &PreviewPage) {
    if !plan.setup().tiling().registration_marks_and_coordinates() {
        return;
    }
    let rect = page.geometry().content_rect();
    let color = svg_color(resolve_color(plan, SemanticColor::Secondary));
    let left = rect.x.micrometres();
    let right = left + rect.width.micrometres();
    let top = rect.y.micrometres();
    let bottom = top + rect.height.micrometres();
    for (x, y) in [(left, top), (right, top), (left, bottom), (right, bottom)] {
        write!(
            output,
            "<path d=\"M {} {y} H {} M {x} {} V {}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"150\"/>",
            x.saturating_sub(2_000),
            x + 2_000,
            y.saturating_sub(2_000),
            y + 2_000
        )
        .expect("write to string");
    }
    write!(
        output,
        "<text x=\"{}\" y=\"{}\" fill=\"{color}\" font-family=\"RSpice Plex Mono\" font-size=\"2200\">{}</text>",
        left + 2_500,
        top + 3_000,
        page.coordinate()
    )
    .expect("write to string");
}

#[derive(Debug)]
struct RasterPage {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
}

fn raster_dimensions(page: &PreviewPage, dpi: u16) -> Result<(u32, u32, u64), HardcopyRenderError> {
    let (width, height) = page.geometry().physical_size();
    let pixels = |length: Length| -> Result<u32, HardcopyRenderError> {
        let numerator = u128::from(length.micrometres()) * u128::from(dpi);
        let value = numerator.div_ceil(u128::from(MICROMETRES_PER_INCH));
        u32::try_from(value).map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
    };
    let width = pixels(width)?;
    let height = pixels(height)?;
    let count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
    if width == 0 || height == 0 || count > MAX_RASTER_PIXELS_PER_PAGE {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "raster pixels per page",
            maximum: MAX_RASTER_PIXELS_PER_PAGE,
        });
    }
    Ok((width, height, count))
}

fn aggregate_raster_pixels(
    plan: &HardcopyPlan,
    dpi: u16,
) -> Result<(u64, u64), HardcopyRenderError> {
    let mut total = 0_u64;
    let mut largest = 0_u64;
    for page in plan.pagination().pages() {
        let (_, _, pixels) = raster_dimensions(page, dpi)?;
        total = total
            .checked_add(pixels)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        largest = largest.max(pixels);
    }
    Ok((total, largest))
}

fn rasterize_page(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
    dpi: u16,
) -> Result<RasterPage, HardcopyRenderError> {
    let (width, height, _) = raster_dimensions(page, dpi)?;
    rasterize_page_at_dimensions(plan, scene, page, dpi, width, height)
}

fn rasterize_page_at_dimensions(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
    dpi: u16,
    width: u32,
    height: u32,
) -> Result<RasterPage, HardcopyRenderError> {
    let svg = render_page_svg(plan, scene, page)?;
    let mut options = usvg::Options {
        dpi: f32::from(dpi),
        font_family: "RSpice Plex Sans".to_owned(),
        ..usvg::Options::default()
    };
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_SANS_REGULAR.to_vec());
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_SANS_SEMIBOLD.to_vec());
    options
        .fontdb_mut()
        .load_font_data(IBM_PLEX_MONO_REGULAR.to_vec());
    let tree = usvg::Tree::from_str(&svg, &options)
        .map_err(|error| HardcopyRenderError::SvgParse(error.to_string()))?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or(HardcopyRenderError::RasterAllocation)?;
    let transform = resvg::tiny_skia::Transform::from_scale(
        width as f32 / tree.size().width(),
        height as f32 / tree.size().height(),
    );
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Ok(RasterPage {
        width,
        height,
        rgba: pixmap.take(),
    })
}

fn apply_soft_proof_preview(rgba: &mut [u8]) {
    // Deterministic, deliberately conservative uncoated-paper proof. This is
    // a dialog aid, not a color-management claim: it compresses chroma and
    // dynamic range against a warm paper white while retaining alpha.
    for pixel in rgba.chunks_exact_mut(4) {
        let red = u16::from(pixel[0]);
        let green = u16::from(pixel[1]);
        let blue = u16::from(pixel[2]);
        let luminance = (red * 54 + green * 183 + blue * 19) / 256;
        let proof = |channel: u16, paper: u16| -> u8 {
            let desaturated = (channel * 3 + luminance) / 4;
            let compressed = 18 + desaturated * 217 / 255;
            u8::try_from(compressed * paper / 255).unwrap_or(u8::MAX)
        };
        pixel[0] = proof(red, 250);
        pixel[1] = proof(green, 247);
        pixel[2] = proof(blue, 238);
    }
}

fn encode_png(page: &RasterPage, dpi: u16) -> Result<Vec<u8>, HardcopyRenderError> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, page.width, page.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Balanced);
        encoder.set_filter(png::Filter::Adaptive);
        let pixels_per_metre = u32::from(dpi)
            .checked_mul(10_000)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
            .div_ceil(254);
        encoder.set_pixel_dims(Some(png::PixelDimensions {
            xppu: pixels_per_metre,
            yppu: pixels_per_metre,
            unit: png::Unit::Meter,
        }));
        let mut writer = encoder
            .write_header()
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
        writer
            .write_image_data(&page.rgba)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
        writer
            .finish()
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "PNG",
                message: error.to_string(),
            })?;
    }
    Ok(bytes)
}

fn render_tiff(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    dpi: u16,
) -> Result<Vec<u8>, HardcopyRenderError> {
    let (total_pixels, largest_page) = aggregate_raster_pixels(plan, dpi)?;
    if total_pixels > MAX_RASTER_PIXELS_TOTAL {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "aggregate raster pixels",
            maximum: MAX_RASTER_PIXELS_TOTAL,
        });
    }
    validate_raster_working_set(largest_page, 1, 12)?;
    let mut bytes = Vec::new();
    let cursor = Cursor::new(&mut bytes);
    let mut encoder = TiffEncoder::new(cursor)
        .map_err(|error| HardcopyRenderError::Encoding {
            format: "TIFF",
            message: error.to_string(),
        })?
        .with_compression(Compression::Deflate(DeflateLevel::Balanced));
    for page in plan.pagination().pages() {
        let raster = rasterize_page(plan, scene, page, dpi)?;
        let rgb_capacity = raster
            .rgba
            .len()
            .checked_div(4)
            .and_then(|pixels| pixels.checked_mul(3))
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let mut rgb = Vec::with_capacity(rgb_capacity);
        for pixel in raster.rgba.chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
        }
        let mut image = encoder
            .new_image::<colortype::RGB8>(raster.width, raster.height)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "TIFF",
                message: error.to_string(),
            })?;
        image.resolution(
            ResolutionUnit::Inch,
            Rational {
                n: u32::from(dpi),
                d: 1,
            },
        );
        image
            .write_data(&rgb)
            .map_err(|error| HardcopyRenderError::Encoding {
                format: "TIFF",
                message: error.to_string(),
            })?;
    }
    drop(encoder);
    Ok(bytes)
}

struct PdfFonts {
    sans: Font,
    semibold: Font,
    mono: Font,
}

impl PdfFonts {
    fn load() -> Result<Self, HardcopyRenderError> {
        Ok(Self {
            sans: load_pdf_font(IBM_PLEX_SANS_REGULAR, "IBM Plex Sans Regular")?,
            semibold: load_pdf_font(IBM_PLEX_SANS_SEMIBOLD, "IBM Plex Sans Semibold")?,
            mono: load_pdf_font(IBM_PLEX_MONO_REGULAR, "IBM Plex Mono Regular")?,
        })
    }

    fn get(&self, font: SceneFont) -> &Font {
        match font {
            SceneFont::Sans => &self.sans,
            SceneFont::SansSemibold => &self.semibold,
            SceneFont::Monospace => &self.mono,
        }
    }
}

fn load_pdf_font(bytes: &'static [u8], name: &'static str) -> Result<Font, HardcopyRenderError> {
    Font::new(bytes.into(), 0).ok_or(HardcopyRenderError::InvalidEmbeddedFont(name))
}

fn render_pdf(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    archival: bool,
) -> Result<Vec<u8>, HardcopyRenderError> {
    if archival && scene.metadata.publication_timestamp.is_none() {
        return Err(HardcopyRenderError::PdfARequiresPublicationTimestamp);
    }
    let configuration = if archival {
        ConfigurationBuilder::new()
            .with_archival_validator(Archival::A2_B)
            .finish()
            .map_err(|error| HardcopyRenderError::PdfAConfiguration(format!("{error:?}")))?
    } else {
        ConfigurationBuilder::new()
            .finish()
            .map_err(|error| HardcopyRenderError::PdfSerialization(format!("{error:?}")))?
    };
    let settings = SerializeSettings {
        pretty: true,
        compress_content_streams: false,
        no_device_cs: archival,
        xmp_metadata: archival,
        configuration,
        enable_tagging: false,
        ..SerializeSettings::default()
    };
    let fonts = PdfFonts::load()?;
    let mut document = Document::new_with(settings);
    let mut metadata = Metadata::new()
        .title(scene.metadata.title.clone())
        .description(format!(
            "RSpice hardcopy; source {}; plan {}",
            plan.source().content_digest(),
            plan.content_digest()
        ))
        .creator(scene.metadata.creator.clone())
        .producer(if archival {
            "RSpice validated PDF/A-2b hardcopy renderer".to_owned()
        } else {
            "RSpice vector hardcopy renderer".to_owned()
        })
        .document_id(format!("rspice-hardcopy-{}", plan.id()))
        .language("en-US".to_owned())
        .page_layout(PageLayout::OneColumn);
    if !scene.metadata.authors.is_empty() {
        metadata = metadata.authors(scene.metadata.authors.clone());
    }
    if let Some(timestamp) = scene.metadata.publication_timestamp {
        metadata = metadata.creation_date(timestamp.as_krilla());
    }
    document.set_metadata(metadata);

    for preview_page in plan.pagination().pages() {
        let geometry = preview_page.geometry();
        let (physical_width, physical_height) = geometry.physical_size();
        let page_width = um_to_points(physical_width.micrometres());
        let page_height = um_to_points(physical_height.micrometres());
        let mut page_settings = PageSettings::from_wh(page_width, page_height)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let full = Rect::from_xywh(0.0, 0.0, page_width, page_height)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let content = geometry.content_rect();
        let art_box = pdf_rect(content)?;
        page_settings = page_settings.with_art_box(Some(art_box));
        if let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() {
            let inset = um_to_points(bleed.micrometres());
            let trim = Rect::from_xywh(
                inset,
                inset,
                page_width - 2.0 * inset,
                page_height - 2.0 * inset,
            )
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
            page_settings = page_settings
                .with_bleed_box(Some(full))
                .with_trim_box(Some(trim));
        }
        let mut page = document.start_page_with(page_settings);
        let mut surface = page.surface();
        if let Some(background) = background_color(plan) {
            set_pdf_fill(&mut surface, background, NormalizedF32::ONE);
            surface.set_stroke(None);
            let mut builder = PathBuilder::new();
            builder.push_rect(full);
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        let mut clip_builder = PathBuilder::new();
        clip_builder.push_rect(pdf_rect(content)?);
        let clip = clip_builder
            .finish()
            .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)?;
        surface.push_clip_path(&clip, &FillRule::NonZero);
        let transform = page_transform(preview_page);
        for primitive in page_primitives(scene, preview_page)? {
            draw_pdf_primitive(&mut surface, &fonts, plan, transform, primitive)?;
        }
        surface.pop();
        draw_pdf_decorations(&mut surface, &fonts, plan, scene, preview_page);
        draw_pdf_trim_marks(&mut surface, plan, preview_page);
        surface.finish();
        page.finish();
    }
    document
        .finish()
        .map_err(|error| HardcopyRenderError::PdfSerialization(error.to_string()))
}

fn um_to_points(value: u64) -> f32 {
    (value as f64 * POINTS_PER_INCH / MICROMETRES_PER_INCH as f64) as f32
}

fn coordinate_to_points(value: f64) -> f32 {
    (value * POINTS_PER_INCH / MICROMETRES_PER_INCH as f64) as f32
}

fn draw_pdf_trim_marks(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() else {
        return;
    };
    let (width, height) = page.geometry().physical_size();
    let inset = bleed.micrometres();
    let right = width.micrometres().saturating_sub(inset);
    let bottom = height.micrometres().saturating_sub(inset);
    let gap = (inset / 4).max(100);
    let length = (inset.saturating_mul(3) / 4).min(5_000).max(500);
    let segments = [
        (
            inset.saturating_sub(gap.saturating_add(length)),
            inset,
            inset.saturating_sub(gap),
            inset,
        ),
        (
            inset,
            inset.saturating_sub(gap.saturating_add(length)),
            inset,
            inset.saturating_sub(gap),
        ),
        (
            right.saturating_add(gap),
            inset,
            right.saturating_add(gap).saturating_add(length),
            inset,
        ),
        (
            right,
            inset.saturating_sub(gap.saturating_add(length)),
            right,
            inset.saturating_sub(gap),
        ),
        (
            inset.saturating_sub(gap.saturating_add(length)),
            bottom,
            inset.saturating_sub(gap),
            bottom,
        ),
        (
            inset,
            bottom.saturating_add(gap),
            inset,
            bottom.saturating_add(gap).saturating_add(length),
        ),
        (
            right.saturating_add(gap),
            bottom,
            right.saturating_add(gap).saturating_add(length),
            bottom,
        ),
        (
            right,
            bottom.saturating_add(gap),
            right,
            bottom.saturating_add(gap).saturating_add(length),
        ),
    ];
    let mut builder = PathBuilder::new();
    for (x1, y1, x2, y2) in segments {
        builder.move_to(um_to_points(x1), um_to_points(y1));
        builder.line_to(um_to_points(x2), um_to_points(y2));
    }
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, SemanticColor::Foreground),
            width_um: 150.0,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    if let Some(path) = builder.finish() {
        surface.draw_path(&path);
    }
}

fn pdf_rect(rect: PageRect) -> Result<Rect, HardcopyRenderError> {
    Rect::from_xywh(
        um_to_points(rect.x.micrometres()),
        um_to_points(rect.y.micrometres()),
        um_to_points(rect.width.micrometres()),
        um_to_points(rect.height.micrometres()),
    )
    .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)
}

fn set_pdf_fill(surface: &mut krilla::surface::Surface<'_>, color: Rgb8, opacity: NormalizedF32) {
    surface.set_fill(Some(Fill {
        paint: rgb::Color::new(color.red, color.green, color.blue).into(),
        opacity,
        rule: FillRule::NonZero,
    }));
}

fn set_pdf_scene_fill(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    fill: Option<SceneFill>,
) {
    match fill {
        Some(SceneFill::Solid { color }) => {
            set_pdf_fill(surface, resolve_color(plan, color), NormalizedF32::ONE)
        }
        Some(SceneFill::CrossHatch { .. }) | None => surface.set_fill(None),
    }
}

fn draw_pdf_cross_hatch(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    scale: f64,
    clip: &krilla::geom::Path,
    bounds: (f32, f32, f32, f32),
    fill: Option<SceneFill>,
) {
    let Some(SceneFill::CrossHatch {
        color,
        line_width,
        spacing,
    }) = fill
    else {
        return;
    };
    let (x, y, width, height) = bounds;
    let tile =
        coordinate_to_points(spacing.micrometres() as f64 * scale) * std::f32::consts::SQRT_2;
    if tile <= 0.0 || width <= 0.0 || height <= 0.0 {
        return;
    }
    let mut builder = PathBuilder::new();
    let mut offset = -height;
    let mut lines = 0_u64;
    while offset <= width {
        builder.move_to(x + offset, y);
        builder.line_to(x + offset + height, y + height);
        builder.move_to(x + offset, y + height);
        builder.line_to(x + offset + height, y);
        offset += tile;
        lines += 2;
        if lines > MAX_RENDER_WORK_UNITS {
            return;
        }
    }
    let Some(path) = builder.finish() else {
        return;
    };
    surface.push_clip_path(clip, &FillRule::NonZero);
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, color),
            width_um: line_width.micrometres() as f64 * scale,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    surface.draw_path(&path);
    surface.pop();
}

fn set_pdf_stroke(surface: &mut krilla::surface::Surface<'_>, stroke: ResolvedStroke) {
    let width = coordinate_to_points(stroke.width_um);
    let dash = if let Some((dash, gap)) = stroke.exact_dash_um {
        Some(vec![coordinate_to_points(dash), coordinate_to_points(gap)])
    } else if let Some(spacing) = stroke.exact_dot_spacing_um {
        Some(vec![width, coordinate_to_points(spacing)])
    } else {
        match stroke.pattern {
            StrokePattern::Solid => None,
            StrokePattern::Dashed => Some(vec![width * 6.0, width * 3.0]),
            StrokePattern::Dotted => Some(vec![width, width * 2.5]),
            StrokePattern::DashDot => Some(vec![width * 6.0, width * 2.5, width, width * 2.5]),
        }
    };
    surface.set_stroke(Some(Stroke {
        paint: rgb::Color::new(stroke.color.red, stroke.color.green, stroke.color.blue).into(),
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        dash: dash.map(|array| StrokeDash { array, offset: 0.0 }),
        ..Stroke::default()
    }));
}

fn draw_pdf_primitive(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    transform: PageTransform,
    primitive: &ScenePrimitive,
) -> Result<(), HardcopyRenderError> {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            let (x1, y1) = transform.point(*from);
            let (x2, y2) = transform.point(*to);
            let mut builder = PathBuilder::new();
            builder.move_to(coordinate_to_points(x1), coordinate_to_points(y1));
            builder.line_to(coordinate_to_points(x2), coordinate_to_points(y2));
            surface.set_fill(None);
            set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke));
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            let mut builder = PathBuilder::new();
            let mut minimum_x = f32::INFINITY;
            let mut minimum_y = f32::INFINITY;
            let mut maximum_x = f32::NEG_INFINITY;
            let mut maximum_y = f32::NEG_INFINITY;
            for (index, point) in points.iter().enumerate() {
                let (x, y) = transform.point(*point);
                let x = coordinate_to_points(x);
                let y = coordinate_to_points(y);
                minimum_x = minimum_x.min(x);
                minimum_y = minimum_y.min(y);
                maximum_x = maximum_x.max(x);
                maximum_y = maximum_y.max(y);
                if index == 0 {
                    builder.move_to(x, y);
                } else {
                    builder.line_to(x, y);
                }
            }
            if *closed {
                builder.close();
            }
            set_pdf_scene_fill(surface, plan, *fill);
            set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke));
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
                draw_pdf_cross_hatch(
                    surface,
                    plan,
                    transform.scale_factor(),
                    &path,
                    (
                        minimum_x,
                        minimum_y,
                        maximum_x - minimum_x,
                        maximum_y - minimum_y,
                    ),
                    *fill,
                );
            }
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let pdf_rect = Rect::from_xywh(
                coordinate_to_points(x),
                coordinate_to_points(y),
                coordinate_to_points(transform.length(rect.width)),
                coordinate_to_points(transform.length(rect.height)),
            );
            set_pdf_scene_fill(surface, plan, *fill);
            match stroke {
                Some(stroke) => set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke)),
                None => surface.set_stroke(None),
            }
            if let Some(pdf_rect_value) = pdf_rect {
                let mut builder = PathBuilder::new();
                builder.push_rect(pdf_rect_value);
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                    draw_pdf_cross_hatch(
                        surface,
                        plan,
                        transform.scale_factor(),
                        &path,
                        (
                            coordinate_to_points(x),
                            coordinate_to_points(y),
                            coordinate_to_points(transform.length(rect.width)),
                            coordinate_to_points(transform.length(rect.height)),
                        ),
                        *fill,
                    );
                }
            }
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let (cx, cy) = transform.point(*center);
            let cx = coordinate_to_points(cx);
            let cy = coordinate_to_points(cy);
            let radius = coordinate_to_points(transform.length(*radius));
            let k = radius * 0.552_284_8;
            let mut builder = PathBuilder::new();
            builder.move_to(cx + radius, cy);
            builder.cubic_to(cx + radius, cy + k, cx + k, cy + radius, cx, cy + radius);
            builder.cubic_to(cx - k, cy + radius, cx - radius, cy + k, cx - radius, cy);
            builder.cubic_to(cx - radius, cy - k, cx - k, cy - radius, cx, cy - radius);
            builder.cubic_to(cx + k, cy - radius, cx + radius, cy - k, cx + radius, cy);
            builder.close();
            set_pdf_scene_fill(surface, plan, *fill);
            match stroke {
                Some(stroke) => set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke)),
                None => surface.set_stroke(None),
            }
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
                draw_pdf_cross_hatch(
                    surface,
                    plan,
                    transform.scale_factor(),
                    &path,
                    (cx - radius, cy - radius, radius * 2.0, radius * 2.0),
                    *fill,
                );
            }
        }
        ScenePrimitive::RasterImage { rect, png, .. } => {
            let image = Image::from_png(png.clone().into(), false)
                .map_err(HardcopyRenderError::InvalidEmbeddedFigure)?;
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let size = Size::from_wh(
                coordinate_to_points(transform.length(rect.width)),
                coordinate_to_points(transform.length(rect.height)),
            )
            .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)?;
            surface.push_transform(&Transform::from_translate(
                coordinate_to_points(x),
                coordinate_to_points(y),
            ));
            surface.draw_image(image, size);
            surface.pop();
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            color,
            anchor,
        } => {
            let (x, y) = transform.point(*origin);
            draw_pdf_text(
                surface,
                fonts.get(*font),
                coordinate_to_points(x),
                coordinate_to_points(y),
                coordinate_to_points(transform.length(*size)),
                text,
                resolve_color(plan, *color),
                *anchor,
                !plan.setup().render().fonts().preserve_searchable_text(),
                NormalizedF32::ONE,
            );
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_pdf_text(
    surface: &mut krilla::surface::Surface<'_>,
    font: &Font,
    mut x: f32,
    y: f32,
    size: f32,
    text: &str,
    color: Rgb8,
    anchor: TextAnchor,
    outlined: bool,
    opacity: NormalizedF32,
) {
    let estimated_width = text.chars().count() as f32 * size * 0.55;
    match anchor {
        TextAnchor::Start => {}
        TextAnchor::Middle => x -= estimated_width / 2.0,
        TextAnchor::End => x -= estimated_width,
    }
    set_pdf_fill(surface, color, opacity);
    surface.set_stroke(None);
    surface.draw_text(
        Point::from_xy(x, y),
        font.clone(),
        size,
        text,
        outlined,
        TextDirection::LeftToRight,
    );
}

fn draw_pdf_decorations(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let outlined = !plan.setup().render().fonts().preserve_searchable_text();
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let ink = resolve_color(plan, SemanticColor::Foreground);
    let secondary = resolve_color(plan, SemanticColor::Secondary);
    if plan.setup().decorations().includes_header() {
        let baseline = um_to_points(printable.y.micrometres() + DECORATION_TEXT_UM);
        let left = um_to_points(printable.x.micrometres());
        let right = um_to_points(printable.x.micrometres() + printable.width.micrometres());
        let font_size = um_to_points(DECORATION_TEXT_UM);
        draw_pdf_text(
            surface,
            &fonts.semibold,
            left,
            baseline,
            font_size,
            scene.metadata.title(),
            ink,
            TextAnchor::Start,
            outlined,
            NormalizedF32::ONE,
        );
        draw_pdf_text(
            surface,
            &fonts.mono,
            right,
            baseline,
            font_size,
            &format!(
                "rev {} · page {} / {} · {}",
                plan.source().revision().get(),
                page.number(),
                plan.pagination().pages().len(),
                page.coordinate()
            ),
            secondary,
            TextAnchor::End,
            outlined,
            NormalizedF32::ONE,
        );
        if let Some(line) = scene.metadata.header_lines.first() {
            draw_pdf_text(
                surface,
                &fonts.sans,
                (left + right) / 2.0,
                baseline,
                font_size,
                line,
                secondary,
                TextAnchor::Middle,
                outlined,
                NormalizedF32::ONE,
            );
        }
    }
    if plan.setup().decorations().includes_provenance() {
        let text = scene
            .metadata
            .provenance_lines
            .first()
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "source {} · plan {}",
                    plan.source().content_digest(),
                    plan.content_digest()
                )
            });
        draw_pdf_text(
            surface,
            &fonts.mono,
            um_to_points(printable.x.micrometres()),
            um_to_points(printable.y.micrometres() + printable.height.micrometres() - 2_000),
            um_to_points(2_200),
            &text,
            secondary,
            TextAnchor::Start,
            outlined,
            NormalizedF32::ONE,
        );
    }
    if plan.setup().decorations().includes_legends() && !scene.legend.is_empty() {
        let legend_band = geometry.legend_band().micrometres();
        let columns = printable.width.micrometres() / LEGEND_COLUMN_UM;
        let rows = legend_band.saturating_sub(LEGEND_VERTICAL_PADDING_UM) / LEGEND_ROW_UM;
        let legend_top = content
            .y
            .micrometres()
            .saturating_add(content.height.micrometres());
        let background = Rect::from_xywh(
            um_to_points(printable.x.micrometres()),
            um_to_points(legend_top),
            um_to_points(printable.width.micrometres()),
            um_to_points(legend_band),
        );
        if let Some(background) = background {
            set_pdf_fill(
                surface,
                Rgb8::new(255, 255, 255),
                NormalizedF32::new(0.92).expect("valid opacity"),
            );
            set_pdf_stroke(
                surface,
                ResolvedStroke {
                    color: secondary,
                    width_um: 150.0,
                    pattern: StrokePattern::Solid,
                    exact_dash_um: None,
                    exact_dot_spacing_um: None,
                },
            );
            let mut builder = PathBuilder::new();
            builder.push_rect(background);
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        for (index, entry) in scene.legend.iter().enumerate() {
            let index = index as u64;
            let column = index / rows.max(1);
            let row = index % rows.max(1);
            if column >= columns {
                break;
            }
            let x_um = printable
                .x
                .micrometres()
                .saturating_add(column.saturating_mul(LEGEND_COLUMN_UM))
                .saturating_add(2_000);
            let y_um = legend_top
                .saturating_add(3_500)
                .saturating_add(row.saturating_mul(LEGEND_ROW_UM));
            let mut builder = PathBuilder::new();
            let transform = page_transform(page);
            if entry.fill.is_some() {
                let swatch_y = y_um.saturating_sub(2_000);
                let Some(swatch) = Rect::from_xywh(
                    um_to_points(x_um),
                    um_to_points(swatch_y),
                    um_to_points(12_000),
                    um_to_points(3_000),
                ) else {
                    continue;
                };
                builder.push_rect(swatch);
                set_pdf_scene_fill(surface, plan, entry.fill);
                set_pdf_stroke(surface, resolve_stroke(plan, transform, entry.stroke));
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                    draw_pdf_cross_hatch(
                        surface,
                        plan,
                        1.0,
                        &path,
                        (
                            um_to_points(x_um),
                            um_to_points(swatch_y),
                            um_to_points(12_000),
                            um_to_points(3_000),
                        ),
                        entry.fill,
                    );
                }
            } else {
                builder.move_to(um_to_points(x_um), um_to_points(y_um));
                builder.line_to(um_to_points(x_um + 12_000), um_to_points(y_um));
                surface.set_fill(None);
                set_pdf_stroke(surface, resolve_stroke(plan, transform, entry.stroke));
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                }
            }
            draw_pdf_text(
                surface,
                &fonts.sans,
                um_to_points(x_um + 15_000),
                um_to_points(y_um + 900),
                um_to_points(2_600),
                &entry.label,
                ink,
                TextAnchor::Start,
                outlined,
                NormalizedF32::ONE,
            );
        }
    }
    draw_pdf_watermark(surface, fonts, plan, page);
    draw_pdf_registration_marks(surface, fonts, plan, page);
}

fn draw_pdf_watermark(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    let text = match plan.setup().decorations().watermark() {
        Watermark::None => return,
        Watermark::Draft => "DRAFT",
        Watermark::Confidential => "CONFIDENTIAL",
        Watermark::Custom(text) => text,
    };
    let (width, height) = page.geometry().physical_size();
    let cx = um_to_points(width.micrometres()) / 2.0;
    let cy = um_to_points(height.micrometres()) / 2.0;
    let size = um_to_points(width.micrometres().min(height.micrometres()) / 10);
    surface.push_transform(&Transform::from_rotate_at(-35.0, cx, cy));
    draw_pdf_text(
        surface,
        &fonts.semibold,
        cx,
        cy,
        size,
        text,
        resolve_color(plan, SemanticColor::Secondary),
        TextAnchor::Middle,
        !plan.setup().render().fonts().preserve_searchable_text(),
        NormalizedF32::new(0.16).expect("valid opacity"),
    );
    surface.pop();
}

fn draw_pdf_registration_marks(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    if !plan.setup().tiling().registration_marks_and_coordinates() {
        return;
    }
    let rect = page.geometry().content_rect();
    let left = rect.x.micrometres();
    let right = left + rect.width.micrometres();
    let top = rect.y.micrometres();
    let bottom = top + rect.height.micrometres();
    let mut builder = PathBuilder::new();
    for (x, y) in [(left, top), (right, top), (left, bottom), (right, bottom)] {
        builder.move_to(um_to_points(x.saturating_sub(2_000)), um_to_points(y));
        builder.line_to(um_to_points(x + 2_000), um_to_points(y));
        builder.move_to(um_to_points(x), um_to_points(y.saturating_sub(2_000)));
        builder.line_to(um_to_points(x), um_to_points(y + 2_000));
    }
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, SemanticColor::Secondary),
            width_um: 150.0,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    if let Some(path) = builder.finish() {
        surface.draw_path(&path);
    }
    draw_pdf_text(
        surface,
        &fonts.mono,
        um_to_points(left + 2_500),
        um_to_points(top + 3_000),
        um_to_points(2_200),
        page.coordinate(),
        resolve_color(plan, SemanticColor::Secondary),
        TextAnchor::Start,
        !plan.setup().render().fonts().preserve_searchable_text(),
        NormalizedF32::ONE,
    );
}

#[cfg(test)]
mod tests {
    use lopdf::Document as ParsedPdf;
    use uuid::Uuid;

    use super::*;
    use crate::hardcopy::{
        ActiveHardcopySource, DecorationSetup, DuplexMode, FontPolicy, HardcopyContentSection,
        HardcopyDocumentKind, HardcopyScope, HardcopySetup, Orientation, PageMargins, PaperSize,
        PhysicalPageSetup, PrintMappingEntry, PrintMappingSaveScope, PrintMappingTable,
        PrintObjectIdentity, PrinterJobSettings, PrinterMediaSource, RenderSetup, ScaleMode,
        StandardPaper, TilingMode, TilingSetup,
    };
    use crate::hardcopy::sources::HardcopySourceIdentity;
    use crate::workbench::hardcopy_sources::{SymbolHardcopySource, resolve_symbol_source};

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
                    crate::hardcopy::PrinterRasterGeometry::try_new(
                        792, 612, 0, 0, 792, 612,
                    )
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
                    crate::hardcopy::PrinterRasterGeometry::try_new(
                        792, 612, 0, 0, 792, 612,
                    )
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
