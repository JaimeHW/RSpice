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

use publication::auto_trace_pattern;
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
#[cfg(not(target_arch = "wasm32"))]
use crate::hardcopy::RenderTarget;
use crate::hardcopy::{
    BackgroundMode, Bleed, ColorMapping, ContentExtent, HardcopyArtifactIdentity, HardcopyPlan,
    Length, OutputFormat, OutsideSheetContentPolicy, PageRect, PreviewPage, PrintColor,
    PrintMappingTable, PrintObjectKind, PrintRedundancy, ResolvedOrientation, ScaleRatio,
    SchematicHardcopyExtent, SchematicHardcopySetup, Watermark,
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

    /// The flat painter's-order primitive list. The publication snapshot
    /// builder converts this directly; render backends keep using the
    /// page-sliced views.
    #[must_use]
    pub(crate) fn primitives(&self) -> &[ScenePrimitive] {
        &self.primitives
    }

    /// The authenticated physical extent of the compiled scene.
    #[must_use]
    pub(crate) fn extent(&self) -> ContentExtent {
        self.extent
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
            HardcopySemanticDocument::Schematic(schematic) => compiler
                .schematic_with_page_label(schematic, child.publication_page_label.as_deref())?,
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
mod tests;
