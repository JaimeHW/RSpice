//! Deterministic hardcopy scene rendering.
//!
//! The renderer is deliberately independent of egui, filesystems, browsers,
//! and printer APIs. Document adapters produce a validated semantic scene in
//! physical micrometres; this module applies the immutable [`HardcopyPlan`]
//! pagination and emits an authenticated artifact. PDF/A output is returned
//! only after Krilla's PDF/A-2b validator accepts the complete document.

use std::collections::BTreeMap;
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
mod compiler;
mod publication;

use compiler::*;

pub use publication::*;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use tiff::encoder::{Compression, DeflateLevel, Rational, TiffEncoder, colortype};
use tiff::tags::ResolutionUnit;

use super::sources::{
    HardcopySemanticDocument, ResolvedHardcopyDocument, SCHEMATIC_SHEET_ORIGIN_X_UNITS,
    SCHEMATIC_SHEET_ORIGIN_Y_UNITS, SCHEMATIC_UNIT_UM, SemanticAggregate, SemanticBounds,
    SemanticPlot, SemanticPoint, SemanticReport, SemanticReportFigure, SemanticResultSummary,
    SemanticSchematic, SemanticTable,
};
use crate::hardcopy::{
    BackgroundMode, Bleed, ColorMapping, ContentExtent, HardcopyArtifactIdentity, HardcopyPlan,
    Length, OutputFormat, OutsideSheetContentPolicy, PageRect, PreviewPage, PrintColor,
    PrintMappingTable, PrintObjectKind, PrintRedundancy, RenderTarget, ResolvedOrientation,
    ScaleRatio, SchematicHardcopyExtent, SchematicHardcopySetup, Watermark,
};
#[cfg(any(test, target_arch = "wasm32"))]
use crate::hardcopy::{HardcopyDocumentId, HardcopyPlanId};
use crate::product::ContentDigest;
#[cfg(any(test, target_arch = "wasm32"))]
use crate::product::ObjectRevision;
use crate::results::report_document::{FigureSizing, ReportBlockId, ReportBlockKind, TableCell};
use crate::schematic::SymbolLibrary;
use crate::schematic::symbols::PathCommand;
use crate::state::{
    Component, DocumentationShapeGeometry, DrawingSheetBorderTemplate, DrawingSheetRect,
    DrawingSheetTitleBlockRotation, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetZoneEdges, DrawingSheetZoneGrid, DrawingSheetZoneLabels, Point as SchematicPoint,
    SchematicSheetFormat, SymbolDocument, SymbolShape,
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
#[cfg(any(test, target_arch = "wasm32"))]
const PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION: u32 = 1;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const MAX_PREVIEW_WORKER_MANIFEST_BYTES: usize = 16 * 1_024;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) const MAX_PREVIEW_WORKER_RGBA_BYTES: usize = 48 * 1_048_576;
#[cfg(any(test, target_arch = "wasm32"))]
const MAX_PREVIEW_WORKER_TRANSFER_BYTES: usize = 48 * 1_048_576 + 16 * 1_024;
#[cfg(any(test, target_arch = "wasm32"))]
const PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION: u32 = 1;
#[cfg(any(test, target_arch = "wasm32"))]
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
    #[cfg(test)]
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SceneTextRotation {
    #[default]
    Upright,
    Clockwise90,
    CounterClockwise90,
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
        #[serde(default)]
        rotation: SceneTextRotation,
    },
    /// Preserve complete source primitives while presenting only one exact
    /// authenticated authored-sheet window. The renderer remaps
    /// `source_origin` to `destination_origin`; the physical page clip then
    /// clips circles, raster images, searchable text, and every future
    /// primitive without lossy geometry-specific rejection.
    ClippedGroup {
        source_origin: ScenePoint,
        destination_origin: ScenePoint,
        clip_extent: ContentExtent,
        source_extent: ContentExtent,
        primitives: Vec<ScenePrimitive>,
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
    #[cfg(test)]
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
    #[cfg(test)]
    pub fn primitives(&self) -> &[ScenePrimitive] {
        &self.primitives
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
    schematic_output: SchematicHardcopySetup,
    metadata: HardcopySceneMetadata,
) -> Result<HardcopyScene, HardcopyRenderError> {
    let drawing_sheet_has_outside_content = source
        .schematic_drawing_sheet_has_outside_content()
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    if drawing_sheet_has_outside_content == Some(true)
        && schematic_output.extent() == SchematicHardcopyExtent::AuthoredDrawingSheet
        && schematic_output.outside_content() == OutsideSheetContentPolicy::Ask
    {
        return Err(HardcopyRenderError::SchematicOutsideContentDecisionRequired);
    }
    if let Some(layout) = source
        .aggregate_layout_for_setup(schematic_output)
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?
        && schematic_output.extent() == SchematicHardcopyExtent::AuthoredDrawingSheet
        && schematic_output.outside_content() == OutsideSheetContentPolicy::ClipToAuthoredSheet
    {
        return scene_from_clipped_aggregate(source, mapping, schematic_output, metadata, layout);
    }
    let clipped_sheet_bounds = source
        .authored_sheet_bounds(schematic_output)
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    let output_extent = source
        .content_extent_for_setup(schematic_output)
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    let mut compiler = SemanticSceneCompiler::new(
        source.bounds(),
        source.content_extent(),
        mapping,
        schematic_output,
    );
    match source.semantic_document() {
        HardcopySemanticDocument::Schematic(schematic) => compiler.schematic(schematic)?,
        HardcopySemanticDocument::Symbol(symbol) => compiler.symbol_document(symbol, None)?,
        HardcopySemanticDocument::Plot(plot) => compiler.plot(plot)?,
        HardcopySemanticDocument::ResultSummary(summary) => compiler.result_summary(summary)?,
        HardcopySemanticDocument::Report(report) => compiler.report(report)?,
        HardcopySemanticDocument::Aggregate(aggregate) => compiler.aggregate(aggregate)?,
    }
    let primitives = if let Some(sheet_bounds) = clipped_sheet_bounds {
        clip_primitives_to_authored_sheet(&compiler.primitives, source.bounds(), sheet_bounds)?
    } else {
        compiler.primitives
    };
    let scene = HardcopyScene {
        extent: output_extent,
        metadata,
        primitives,
        legend: compiler.legend,
        aggregate_sections: compiler.aggregate_sections,
    };
    scene.validate()?;
    Ok(scene)
}

fn scene_from_clipped_aggregate(
    source: &ResolvedHardcopyDocument,
    mapping: &PrintMappingTable,
    schematic_output: SchematicHardcopySetup,
    metadata: HardcopySceneMetadata,
    layout: Vec<(
        &crate::workbench::hardcopy_adapters::sources::SemanticAggregateChild,
        ContentExtent,
        SemanticPoint,
    )>,
) -> Result<HardcopyScene, HardcopyRenderError> {
    let extent = source
        .content_extent_for_setup(schematic_output)
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    let mut primitives = Vec::new();
    let mut legend = Vec::new();
    let mut aggregate_sections = Vec::with_capacity(layout.len());
    for (child, child_extent, origin) in layout {
        let original_extent = child
            .local_bounds
            .content_extent()
            .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
        let mut compiler = SemanticSceneCompiler::new(
            child.local_bounds,
            original_extent,
            mapping,
            schematic_output,
        );
        compiler.mapping_ordinal = Some(child.ordinal);
        match child.document.as_ref() {
            HardcopySemanticDocument::Schematic(schematic) => compiler.schematic(schematic)?,
            HardcopySemanticDocument::Symbol(symbol) => compiler.symbol_document(symbol, None)?,
            HardcopySemanticDocument::Plot(plot) => compiler.plot(plot)?,
            HardcopySemanticDocument::ResultSummary(summary) => compiler.result_summary(summary)?,
            HardcopySemanticDocument::Report(report) => compiler.report(report)?,
            HardcopySemanticDocument::Aggregate(_) => {
                return Err(conversion_error("nested aggregate hardcopy source"));
            }
        }
        let child_primitives = match child.document.as_ref() {
            HardcopySemanticDocument::Schematic(schematic) => schematic
                .drawing_sheet
                .as_ref()
                .map(|format| {
                    let sheet =
                        crate::workbench::hardcopy_adapters::sources::authored_sheet_bounds(format)
                            .map_err(|error| {
                                HardcopyRenderError::SourceConversion(error.to_string())
                            })?;
                    clip_primitives_to_authored_sheet(
                        &compiler.primitives,
                        child.local_bounds,
                        sheet,
                    )
                })
                .transpose()?
                .unwrap_or(compiler.primitives),
            _ => compiler.primitives,
        };
        let origin = ScenePoint::new(
            Length::from_micrometres(
                u64::try_from(origin.x_um)
                    .map_err(|_| conversion_error("aggregate X origin is negative"))?,
            ),
            Length::from_micrometres(
                u64::try_from(origin.y_um)
                    .map_err(|_| conversion_error("aggregate Y origin is negative"))?,
            ),
        );
        let primitive_start = primitives.len();
        primitives.extend(translate_primitives(child_primitives, origin)?);
        let primitive_end = primitives.len();
        aggregate_sections.push(AggregateSection {
            ordinal: child.ordinal,
            source_key: child.source_key.clone(),
            display_name: child.display_name.clone(),
            content_digest: child.content_digest,
            origin,
            extent: child_extent,
            page_break_before: child.page_break_before,
            primitive_start,
            primitive_end,
        });
        for entry in compiler.legend {
            if !legend.iter().any(|existing| existing == &entry) {
                legend.push(entry);
            }
        }
    }
    let scene = HardcopyScene {
        extent,
        metadata,
        primitives,
        legend,
        aggregate_sections,
    };
    scene.validate()?;
    Ok(scene)
}

fn translate_primitives(
    primitives: Vec<ScenePrimitive>,
    origin: ScenePoint,
) -> Result<Vec<ScenePrimitive>, HardcopyRenderError> {
    let point = |point: ScenePoint| -> Result<ScenePoint, HardcopyRenderError> {
        Ok(ScenePoint::new(
            Length::from_micrometres(
                point
                    .x
                    .micrometres()
                    .checked_add(origin.x.micrometres())
                    .ok_or_else(|| conversion_error("aggregate X translation overflow"))?,
            ),
            Length::from_micrometres(
                point
                    .y
                    .micrometres()
                    .checked_add(origin.y.micrometres())
                    .ok_or_else(|| conversion_error("aggregate Y translation overflow"))?,
            ),
        ))
    };
    primitives
        .into_iter()
        .map(|primitive| {
            Ok(match primitive {
                ScenePrimitive::Line { from, to, stroke } => ScenePrimitive::Line {
                    from: point(from)?,
                    to: point(to)?,
                    stroke,
                },
                ScenePrimitive::Polyline {
                    points,
                    closed,
                    stroke,
                    fill,
                } => ScenePrimitive::Polyline {
                    points: points.into_iter().map(point).collect::<Result<_, _>>()?,
                    closed,
                    stroke,
                    fill,
                },
                ScenePrimitive::Rect { rect, stroke, fill } => ScenePrimitive::Rect {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(
                            rect.x
                                .micrometres()
                                .checked_add(origin.x.micrometres())
                                .ok_or_else(|| {
                                    conversion_error("aggregate rectangle X overflow")
                                })?,
                        ),
                        Length::from_micrometres(
                            rect.y
                                .micrometres()
                                .checked_add(origin.y.micrometres())
                                .ok_or_else(|| {
                                    conversion_error("aggregate rectangle Y overflow")
                                })?,
                        ),
                        rect.width,
                        rect.height,
                    )?,
                    stroke,
                    fill,
                },
                ScenePrimitive::Circle {
                    center,
                    radius,
                    stroke,
                    fill,
                } => ScenePrimitive::Circle {
                    center: point(center)?,
                    radius,
                    stroke,
                    fill,
                },
                ScenePrimitive::RasterImage {
                    rect,
                    png,
                    content_digest,
                    alternative_text,
                } => ScenePrimitive::RasterImage {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(
                            rect.x
                                .micrometres()
                                .checked_add(origin.x.micrometres())
                                .ok_or_else(|| conversion_error("aggregate image X overflow"))?,
                        ),
                        Length::from_micrometres(
                            rect.y
                                .micrometres()
                                .checked_add(origin.y.micrometres())
                                .ok_or_else(|| conversion_error("aggregate image Y overflow"))?,
                        ),
                        rect.width,
                        rect.height,
                    )?,
                    png,
                    content_digest,
                    alternative_text,
                },
                ScenePrimitive::Text {
                    origin: text_origin,
                    text,
                    font,
                    size,
                    color,
                    anchor,
                    rotation,
                } => ScenePrimitive::Text {
                    origin: point(text_origin)?,
                    text,
                    font,
                    size,
                    color,
                    anchor,
                    rotation,
                },
                ScenePrimitive::ClippedGroup {
                    source_origin,
                    destination_origin,
                    clip_extent,
                    source_extent,
                    primitives,
                } => ScenePrimitive::ClippedGroup {
                    source_origin,
                    destination_origin: point(destination_origin)?,
                    clip_extent,
                    source_extent,
                    primitives,
                },
            })
        })
        .collect()
}

/// Project a union-bounded schematic scene onto the authored sheet.  This is
/// deliberately done before pagination: all backends receive sheet-local
/// coordinates and therefore share the same exact physical clipping edge.
fn clip_primitives_to_authored_sheet(
    primitives: &[ScenePrimitive],
    source_bounds: SemanticBounds,
    sheet_bounds: SemanticBounds,
) -> Result<Vec<ScenePrimitive>, HardcopyRenderError> {
    let x = u64::try_from(sheet_bounds.minimum.x_um - source_bounds.minimum.x_um)
        .map_err(|_| conversion_error("authored sheet precedes source bounds"))?;
    let y = u64::try_from(sheet_bounds.minimum.y_um - source_bounds.minimum.y_um)
        .map_err(|_| conversion_error("authored sheet precedes source bounds"))?;
    let width = u64::try_from(sheet_bounds.maximum.x_um - sheet_bounds.minimum.x_um)
        .map_err(|_| conversion_error("invalid authored sheet width"))?;
    let height = u64::try_from(sheet_bounds.maximum.y_um - sheet_bounds.minimum.y_um)
        .map_err(|_| conversion_error("invalid authored sheet height"))?;
    let source_extent = source_bounds
        .content_extent()
        .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    let clip_extent = ContentExtent::try_new(
        Length::from_micrometres(width),
        Length::from_micrometres(height),
    )
    .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?;
    Ok(vec![ScenePrimitive::ClippedGroup {
        source_origin: ScenePoint::new(Length::from_micrometres(x), Length::from_micrometres(y)),
        destination_origin: ScenePoint::new(Length::ZERO, Length::ZERO),
        clip_extent,
        source_extent,
        primitives: primitives.to_vec(),
    }])
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct ClipRect {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

#[cfg(test)]
impl ClipRect {
    fn right(self) -> u64 {
        self.x.saturating_add(self.width)
    }
    fn bottom(self) -> u64 {
        self.y.saturating_add(self.height)
    }
    fn contains(self, point: ScenePoint) -> bool {
        let x = point.x.micrometres();
        let y = point.y.micrometres();
        x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
    }
    fn local(self, point: ScenePoint) -> ScenePoint {
        ScenePoint::new(
            Length::from_micrometres(point.x.micrometres() - self.x),
            Length::from_micrometres(point.y.micrometres() - self.y),
        )
    }
}

#[cfg(test)]
fn clip_scene_primitive(
    primitive: &ScenePrimitive,
    clip: ClipRect,
) -> Result<Vec<ScenePrimitive>, HardcopyRenderError> {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => Ok(clip_line(*from, *to, clip)
            .map(|(from, to)| {
                vec![ScenePrimitive::Line {
                    from: clip.local(from),
                    to: clip.local(to),
                    stroke: *stroke,
                }]
            })
            .unwrap_or_default()),
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } if !closed && fill.is_none() => Ok(points
            .windows(2)
            .filter_map(|edge| clip_line(edge[0], edge[1], clip))
            .map(|(from, to)| ScenePrimitive::Line {
                from: clip.local(from),
                to: clip.local(to),
                stroke: *stroke,
            })
            .collect()),
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            let points = clip_polygon(points, clip)
                .into_iter()
                .map(|point| clip.local(point))
                .collect::<Vec<_>>();
            if points.len() < if *closed { 3 } else { 2 } {
                Ok(Vec::new())
            } else {
                Ok(vec![ScenePrimitive::Polyline {
                    points,
                    closed: *closed,
                    stroke: *stroke,
                    fill: *fill,
                }])
            }
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            let left = rect.x.micrometres().max(clip.x);
            let top = rect.y.micrometres().max(clip.y);
            let right = rect
                .x
                .micrometres()
                .saturating_add(rect.width.micrometres())
                .min(clip.right());
            let bottom = rect
                .y
                .micrometres()
                .saturating_add(rect.height.micrometres())
                .min(clip.bottom());
            if right <= left || bottom <= top {
                return Ok(Vec::new());
            }
            Ok(vec![ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(left - clip.x),
                    Length::from_micrometres(top - clip.y),
                    Length::from_micrometres(right - left),
                    Length::from_micrometres(bottom - top),
                )?,
                stroke: *stroke,
                fill: *fill,
            }])
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let radius_um = radius.micrometres();
            if center.x.micrometres().saturating_sub(radius_um) >= clip.x
                && center.y.micrometres().saturating_sub(radius_um) >= clip.y
                && center.x.micrometres().saturating_add(radius_um) <= clip.right()
                && center.y.micrometres().saturating_add(radius_um) <= clip.bottom()
            {
                Ok(vec![ScenePrimitive::Circle {
                    center: clip.local(*center),
                    radius: *radius,
                    stroke: *stroke,
                    fill: *fill,
                }])
            } else {
                Ok(Vec::new())
            }
        }
        ScenePrimitive::RasterImage {
            rect,
            png,
            content_digest,
            alternative_text,
        } => {
            let right = rect
                .x
                .micrometres()
                .saturating_add(rect.width.micrometres());
            let bottom = rect
                .y
                .micrometres()
                .saturating_add(rect.height.micrometres());
            if rect.x.micrometres() >= clip.x
                && rect.y.micrometres() >= clip.y
                && right <= clip.right()
                && bottom <= clip.bottom()
            {
                Ok(vec![ScenePrimitive::RasterImage {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(rect.x.micrometres() - clip.x),
                        Length::from_micrometres(rect.y.micrometres() - clip.y),
                        rect.width,
                        rect.height,
                    )?,
                    png: png.clone(),
                    content_digest: *content_digest,
                    alternative_text: alternative_text.clone(),
                }])
            } else {
                Ok(Vec::new())
            }
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            color,
            anchor,
            rotation,
        } => {
            if clip.contains(*origin) {
                Ok(vec![ScenePrimitive::Text {
                    origin: clip.local(*origin),
                    text: text.clone(),
                    font: *font,
                    size: *size,
                    color: *color,
                    anchor: *anchor,
                    rotation: *rotation,
                }])
            } else {
                Ok(Vec::new())
            }
        }
        ScenePrimitive::ClippedGroup { .. } => Err(HardcopyRenderError::InvalidClippedScene),
    }
}

#[cfg(test)]
fn clip_line(from: ScenePoint, to: ScenePoint, clip: ClipRect) -> Option<(ScenePoint, ScenePoint)> {
    let (mut x0, mut y0) = (from.x.micrometres() as f64, from.y.micrometres() as f64);
    let (mut x1, mut y1) = (to.x.micrometres() as f64, to.y.micrometres() as f64);
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut t0 = 0.0_f64;
    let mut t1 = 1.0_f64;
    for (p, q) in [
        (-dx, x0 - clip.x as f64),
        (dx, clip.right() as f64 - x0),
        (-dy, y0 - clip.y as f64),
        (dy, clip.bottom() as f64 - y0),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                if r > t1 {
                    return None;
                }
                t0 = t0.max(r);
            } else {
                if r < t0 {
                    return None;
                }
                t1 = t1.min(r);
            }
        }
    }
    x1 = x0 + t1 * dx;
    y1 = y0 + t1 * dy;
    x0 += t0 * dx;
    y0 += t0 * dy;
    Some((
        ScenePoint::new(
            Length::from_micrometres(x0.round() as u64),
            Length::from_micrometres(y0.round() as u64),
        ),
        ScenePoint::new(
            Length::from_micrometres(x1.round() as u64),
            Length::from_micrometres(y1.round() as u64),
        ),
    ))
}

#[cfg(test)]
fn clip_polygon(points: &[ScenePoint], clip: ClipRect) -> Vec<ScenePoint> {
    let mut output = points.to_vec();
    for (axis, bound, keep_greater) in [
        (0, clip.x, true),
        (0, clip.right(), false),
        (1, clip.y, true),
        (1, clip.bottom(), false),
    ] {
        let input = std::mem::take(&mut output);
        if input.is_empty() {
            break;
        }
        let inside = |point: ScenePoint| {
            let value = if axis == 0 {
                point.x.micrometres()
            } else {
                point.y.micrometres()
            };
            if keep_greater {
                value >= bound
            } else {
                value <= bound
            }
        };
        let mut previous = *input.last().expect("nonempty polygon");
        let mut previous_inside = inside(previous);
        for current in input {
            let current_inside = inside(current);
            if current_inside != previous_inside
                && let Some((a, b)) = if axis == 0 {
                    clip_line(
                        previous,
                        current,
                        ClipRect {
                            x: bound,
                            y: 0,
                            width: 0,
                            height: u64::MAX,
                        },
                    )
                } else {
                    clip_line(
                        previous,
                        current,
                        ClipRect {
                            x: 0,
                            y: bound,
                            width: u64::MAX,
                            height: 0,
                        },
                    )
                }
            {
                output.push(if axis == 0 {
                    if a.x.micrometres() == bound { a } else { b }
                } else if a.y.micrometres() == bound {
                    a
                } else {
                    b
                });
            }
            if current_inside {
                output.push(current);
            }
            previous = current;
            previous_inside = current_inside;
        }
    }
    output
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
    use crate::hardcopy::sources::{
        HardcopySourceIdentity, HardcopySourceSet, HardcopySourceSetMember,
    };
    use crate::hardcopy::{
        ActiveHardcopySource, DecorationSetup, DuplexMode, FontPolicy, HardcopyContentSection,
        HardcopyDocumentKind, HardcopyScope, HardcopySetup, Orientation, OutsideSheetContentPolicy,
        PageMargins, PaperSize, PhysicalPageSetup, PrintMappingEntry, PrintMappingSaveScope,
        PrintMappingTable, PrintObjectIdentity, PrinterJobSettings, PrinterMediaSource,
        RenderSetup, ScaleMode, SchematicHardcopyExtent, SchematicHardcopySetup, StandardPaper,
        TilingMode, TilingSetup,
    };
    use crate::state::{
        DrawingSheetTitleBlockRotation, DrawingSheetTitleFieldId, DrawingSheetZoneEdges,
        DrawingSheetZoneLabels, SchematicSheetFormat,
    };
    use crate::workbench::hardcopy_adapters::sources::{
        SymbolHardcopySource, resolve_blank_schematic_sheet_with_format,
        resolve_hardcopy_source_set_with, resolve_symbol_source,
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
            rotation: SceneTextRotation::Upright,
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
                    rotation: SceneTextRotation::Upright,
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

    fn resolved_blank_schematic(format: &SchematicSheetFormat) -> ResolvedHardcopyDocument {
        resolved_blank_schematic_with_identity(
            format,
            "test-schematic",
            0x5343_4845_4d41,
            "Test schematic",
        )
    }

    fn resolved_blank_schematic_with_identity(
        format: &SchematicSheetFormat,
        source_key: &str,
        document_id: u128,
        display_name: &str,
    ) -> ResolvedHardcopyDocument {
        resolve_blank_schematic_sheet_with_format(
            HardcopySourceIdentity::try_new(
                source_key,
                HardcopyDocumentId::try_from_uuid(Uuid::from_u128(document_id)).unwrap(),
                ObjectRevision::INITIAL,
                display_name,
            )
            .unwrap(),
            HardcopyScope::CurrentSheet,
            Some(format),
        )
        .unwrap()
    }

    fn setup_with_schematic_output(
        format: OutputFormat,
        schematic: SchematicHardcopySetup,
    ) -> HardcopySetup {
        let base = setup(format, false);
        HardcopySetup::try_new_with_schematic(
            base.physical_page().clone(),
            base.scale(),
            base.tiling(),
            base.render().clone(),
            base.decorations().clone(),
            schematic,
            base.print_mapping().clone(),
        )
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
        let mut first_identity =
            SemanticSceneCompiler::new(bounds, content, &empty, SchematicHardcopySetup::default());
        first_identity.mapping_ordinal = Some(0);
        let mut second_identity =
            SemanticSceneCompiler::new(bounds, content, &empty, SchematicHardcopySetup::default());
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
        let mut first = SemanticSceneCompiler::new(
            bounds,
            content,
            &mapping,
            SchematicHardcopySetup::default(),
        );
        first.mapping_ordinal = Some(0);
        let mut second = SemanticSceneCompiler::new(
            bounds,
            content,
            &mapping,
            SchematicHardcopySetup::default(),
        );
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
        let scene = scene_from_resolved(
            &source,
            plan.setup().print_mapping(),
            plan.setup().schematic(),
            resolved_metadata(),
        )
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
    fn schematic_inclusion_contract_gates_real_scene_primitives() {
        let source = resolved_blank_schematic(&SchematicSheetFormat::default());
        let excluded = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        let excluded_scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            excluded,
            resolved_metadata(),
        )
        .unwrap();
        assert!(excluded_scene.primitives().is_empty());

        let grid_only = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            false,
            false,
            true,
        );
        let grid_scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            grid_only,
            resolved_metadata(),
        )
        .unwrap();
        assert!(!grid_scene.primitives().is_empty());
        assert!(
            grid_scene
                .primitives()
                .iter()
                .all(|primitive| matches!(primitive, ScenePrimitive::Line { .. }))
        );
    }

    #[test]
    fn schematic_clipping_policy_compiles_an_authored_sheet() {
        let source = resolved_blank_schematic(&SchematicSheetFormat::default());
        let clipping = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::ClipToAuthoredSheet,
            true,
            true,
            true,
            true,
            true,
            false,
        );

        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            clipping,
            resolved_metadata(),
        )
        .unwrap();

        assert_eq!(scene.extent, source.content_extent());
    }

    #[test]
    fn authored_sheet_clip_projects_crossing_geometry_to_the_exact_sheet_edge() {
        let clip = ClipRect {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
        };
        let stroke = StrokeStyle::default();
        let primitives = clip_scene_primitive(
            &ScenePrimitive::Line {
                from: ScenePoint::new(Length::from_micrometres(0), Length::from_micrometres(100)),
                to: ScenePoint::new(Length::from_micrometres(200), Length::from_micrometres(100)),
                stroke,
            },
            clip,
        )
        .unwrap();
        assert!(matches!(
            primitives.as_slice(),
            [ScenePrimitive::Line { from, to, .. }]
                if from.x.micrometres() == 0
                    && from.y.micrometres() == 50
                    && to.x.micrometres() == 100
                    && to.y.micrometres() == 50
        ));
    }

    #[test]
    fn authored_sheet_clip_preserves_partially_intersecting_non_linear_primitives() {
        let source_bounds =
            SemanticBounds::try_new(SemanticPoint::new(0, 0), SemanticPoint::new(200, 200))
                .unwrap();
        let sheet_bounds =
            SemanticBounds::try_new(SemanticPoint::new(50, 50), SemanticPoint::new(150, 150))
                .unwrap();
        let crossing = vec![
            ScenePrimitive::Circle {
                center: ScenePoint::new(Length::from_micrometres(40), Length::from_micrometres(90)),
                radius: Length::from_micrometres(20),
                stroke: Some(StrokeStyle::default()),
                fill: None,
            },
            ScenePrimitive::RasterImage {
                rect: SceneRect::try_new(
                    Length::from_micrometres(40),
                    Length::from_micrometres(60),
                    Length::from_micrometres(30),
                    Length::from_micrometres(30),
                )
                .unwrap(),
                png: vec![1, 2, 3],
                content_digest: ContentDigest::from_bytes([7; 32]),
                alternative_text: "crossing image".to_owned(),
            },
            ScenePrimitive::Text {
                origin: ScenePoint::new(Length::from_micrometres(40), Length::from_micrometres(80)),
                text: "crossing text".to_owned(),
                font: SceneFont::Sans,
                size: Length::from_micrometres(20),
                color: SemanticColor::Foreground,
                anchor: TextAnchor::Start,
                rotation: SceneTextRotation::Upright,
            },
        ];
        let clipped =
            clip_primitives_to_authored_sheet(&crossing, source_bounds, sheet_bounds).unwrap();
        let [
            ScenePrimitive::ClippedGroup {
                source_origin,
                destination_origin,
                clip_extent,
                source_extent,
                primitives,
            },
        ] = clipped.as_slice()
        else {
            panic!("authored clipping must retain one exact renderer-owned clip group");
        };
        assert_eq!(source_origin.x.micrometres(), 50);
        assert_eq!(source_origin.y.micrometres(), 50);
        assert_eq!(
            *destination_origin,
            ScenePoint::new(Length::ZERO, Length::ZERO)
        );
        assert_eq!(clip_extent.width().micrometres(), 100);
        assert_eq!(clip_extent.height().micrometres(), 100);
        assert_eq!(source_extent.width().micrometres(), 200);
        assert_eq!(source_extent.height().micrometres(), 200);
        assert_eq!(primitives, &crossing);
    }

    #[test]
    fn coordinate_zone_mode_keeps_rules_but_suppresses_every_edge_label() {
        let mut format = SchematicSheetFormat::default();
        format.zones.labels = DrawingSheetZoneLabels::Coordinates;
        let geometry = format.geometry().unwrap();
        let source = resolved_blank_schematic(&format);
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            false,
            true,
            false,
        );
        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            schematic,
            resolved_metadata(),
        )
        .unwrap();
        assert!(
            scene
                .primitives()
                .iter()
                .all(|primitive| !matches!(primitive, ScenePrimitive::Text { .. }))
        );
        assert!(
            scene
                .primitives()
                .iter()
                .all(|primitive| matches!(primitive, ScenePrimitive::Line { .. })),
            "zones-only output must own its ruled band without borrowing paper, border, or title primitives"
        );
        assert!(
            scene
                .primitives()
                .iter()
                .any(|primitive| matches!(primitive, ScenePrimitive::Line { .. }))
        );
        assert!(
            scene.primitives().iter().any(|primitive| {
                let ScenePrimitive::Line { from, to, .. } = primitive else {
                    return false;
                };
                from.x.micrometres().abs_diff(to.x.micrometres()) == geometry.border_band_um
                    || from.y.micrometres().abs_diff(to.y.micrometres()) == geometry.border_band_um
            }),
            "the zones-only layer must span the complete authored band even without the border layer"
        );
    }

    #[test]
    fn hardcopy_zone_labels_follow_the_selected_output_edges() {
        let bottom_right = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.zones.edges = DrawingSheetZoneEdges::BottomAndRight;
            })
            .unwrap();
        let geometry = bottom_right.geometry().unwrap();
        let source = resolved_blank_schematic(&bottom_right);
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            false,
            true,
            false,
        );
        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            schematic,
            resolved_metadata(),
        )
        .unwrap();
        let labels = scene
            .primitives()
            .iter()
            .filter_map(|primitive| {
                let ScenePrimitive::Text { origin, .. } = primitive else {
                    return None;
                };
                Some(origin)
            })
            .collect::<Vec<_>>();
        let zones = geometry.zones.unwrap();
        assert_eq!(labels.len(), usize::from(zones.columns + zones.rows));
        let drawing_right =
            u64::try_from(geometry.drawing_area.x_um + geometry.drawing_area.width_um as i64)
                .unwrap();
        let drawing_bottom =
            u64::try_from(geometry.drawing_area.y_um + geometry.drawing_area.height_um as i64)
                .unwrap();
        assert_eq!(
            labels
                .iter()
                .filter(|origin| origin.y.micrometres() > drawing_bottom)
                .count(),
            usize::from(zones.columns),
            "one column label must be emitted on the selected bottom edge"
        );
        assert_eq!(
            labels
                .iter()
                .filter(|origin| origin.x.micrometres() > drawing_right)
                .count(),
            usize::from(zones.rows),
            "one row label must be emitted on the selected right edge"
        );

        let all_edges = bottom_right
            .try_update(|draft| draft.zones.edges = DrawingSheetZoneEdges::All)
            .unwrap();
        let source = resolved_blank_schematic(&all_edges);
        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            schematic,
            resolved_metadata(),
        )
        .unwrap();
        assert_eq!(
            scene
                .primitives()
                .iter()
                .filter(|primitive| matches!(primitive, ScenePrimitive::Text { .. }))
                .count(),
            usize::from((zones.columns + zones.rows) * 2),
            "all-edge output must publish the label set on both opposing edges"
        );
    }

    #[test]
    fn structured_scale_authority_projects_even_when_legacy_field_storage_is_empty() {
        let format = SchematicSheetFormat::default()
            .try_update(|draft| {
                draft
                    .title_block
                    .fields
                    .get_mut(&DrawingSheetTitleFieldId::Scale)
                    .unwrap()
                    .value
                    .clear();
            })
            .unwrap();
        let source = resolved_blank_schematic(&format);
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            true,
            false,
            false,
        );
        let scene = scene_from_resolved(
            &source,
            source.default_print_mapping(),
            schematic,
            resolved_metadata(),
        )
        .unwrap();
        let text = scene
            .primitives()
            .iter()
            .filter_map(|primitive| {
                let ScenePrimitive::Text { text, .. } = primitive else {
                    return None;
                };
                Some(text.as_str())
            })
            .collect::<Vec<_>>();
        assert!(text.contains(&"\u{2022} Scale: 1:1"));
        assert!(
            !text.iter().any(|line| line.starts_with("SCALE: NTS")),
            "the structured ratio must not be replaced by an unrelated NTS value"
        );
    }

    #[test]
    fn hardcopy_zone_alphabet_matches_engineering_drawing_overflow_rules() {
        assert_eq!(compiler::zone_alpha_label(0), "A");
        assert_eq!(compiler::zone_alpha_label(8), "J");
        assert_eq!(compiler::zone_alpha_label(21), "Y");
        assert_eq!(compiler::zone_alpha_label(22), "23");
    }

    #[test]
    fn clipped_sheet_set_reflows_each_child_and_compiles_with_matching_sections() {
        let first = resolved_blank_schematic(&SchematicSheetFormat::default());
        let mut second_format = SchematicSheetFormat::default();
        second_format.orientation = crate::state::SchematicPageOrientation::Landscape;
        let second = resolved_blank_schematic_with_identity(
            &second_format,
            "test-schematic-second-sheet",
            0x5343_4845_4d42,
            "Test schematic · second sheet",
        );
        let members = [&first, &second]
            .into_iter()
            .map(|document| {
                HardcopySourceSetMember::try_new(
                    document.source_key(),
                    document.authority().display_name(),
                    document.authority().document_id(),
                    document.authority().revision(),
                    document.authority().content_digest(),
                    HardcopyScope::CurrentSheet,
                )
                .unwrap()
            })
            .collect();
        let set = HardcopySourceSet::try_new(
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(0x5151)).unwrap(),
            ObjectRevision::INITIAL,
            "Test sheet set",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::AllSheetsOrPanes,
            members,
        )
        .unwrap();
        let mut retained = vec![first, second].into_iter();
        let source =
            resolve_hardcopy_source_set_with(&set, |_| Ok(retained.next().unwrap())).unwrap();
        let clipping = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::ClipToAuthoredSheet,
            false,
            true,
            true,
            true,
            true,
            false,
        );
        let extent = source.content_extent_for_setup(clipping).unwrap();
        let sections = source.hardcopy_sections_for_setup(clipping).unwrap();
        assert_eq!(sections.len(), 2);
        let setup = setup_with_schematic_output(OutputFormat::SvgVector, clipping);
        let plan = HardcopyPlan::compile_with_sections(
            source.authority().clone(),
            setup,
            extent,
            sections,
        )
        .unwrap();
        let scene = scene_from_resolved(
            &source,
            plan.setup().print_mapping(),
            clipping,
            resolved_metadata(),
        )
        .unwrap();
        assert_eq!(scene.extent, plan.content_extent());
        HardcopyRenderer::render(&plan, &scene).unwrap();
    }

    #[test]
    fn rotated_title_block_rotates_grid_text_and_published_svg() {
        let mut format = SchematicSheetFormat::default();
        format.title_block.rotation = DrawingSheetTitleBlockRotation::Clockwise90;
        let source = resolved_blank_schematic(&format);
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            false,
            true,
            false,
            false,
        );
        let setup = setup_with_schematic_output(OutputFormat::SvgVector, schematic);
        let plan =
            HardcopyPlan::compile(source.authority().clone(), setup, source.content_extent())
                .unwrap();
        let scene = scene_from_resolved(
            &source,
            plan.setup().print_mapping(),
            plan.setup().schematic(),
            resolved_metadata(),
        )
        .unwrap();
        let rotated_text = scene
            .primitives()
            .iter()
            .filter_map(|primitive| match primitive {
                ScenePrimitive::Text { rotation, .. } => Some(*rotation),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!rotated_text.is_empty());
        assert!(
            rotated_text
                .iter()
                .all(|rotation| *rotation == SceneTextRotation::Clockwise90)
        );

        let publication =
            HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
        let svg = String::from_utf8_lossy(publication.single_part().unwrap().bytes());
        assert!(svg.contains("transform=\"rotate(90 "));
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
