//! Deterministic software rasterization for governed report figures.
//!
//! This module deliberately has no dependency on egui, `AppState`, WGPU, a
//! browser canvas, or platform font services. It resolves one exact
//! [`VisualizationDocument`] revision/page/pane into a bounded Cartesian line
//! scene and encodes that scene as an opaque RGB8 PNG. The same Rust code and
//! fixed PNG settings are therefore used by native and WebAssembly builds.
//!
//! The renderer is intentionally fail-closed. A document feature is rendered
//! only when this module has an explicit, deterministic interpretation for it;
//! other viewers, non-linear axes, family styling, non-numeric values, and
//! pane overlays return an error instead of being silently omitted.

use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::product::{ContentDigest, DatasetBinding, ObjectRevision, ResultDocumentId};

use super::report_document::{
    FrozenReportArtifact, MAX_FROZEN_ARTIFACT_BYTES, ReportReferenceSnapshot, ReportSourceId,
};
use super::visualization_document::{
    Axis, AxisOrientation, AxisRange, AxisScale, ColumnRole, PageId, PaneId, PaneKind,
    SourceDataset, TraceId, TypedValue, ValueType, VisualizationDocument,
};

/// Media type accepted by the governed report figure and PDF/A pipelines.
pub const VISUALIZATION_RASTER_MEDIA_TYPE: &str = "image/png";

/// Smallest useful report figure in either dimension.
pub const MIN_RASTER_DIMENSION: u32 = 128;
/// Per-dimension ceiling; the aggregate pixel ceiling is lower for rectangles.
pub const MAX_RASTER_DIMENSION: u32 = 4_096;
/// Maximum opaque RGB8 pixels allocated for one figure (24 MiB of samples).
pub const MAX_RASTER_PIXELS: usize = 8 * 1_048_576;
/// Maximum PNG bytes accepted by `FrozenReportArtifact`.
pub const MAX_RASTER_ARTIFACT_BYTES: usize = MAX_FROZEN_ARTIFACT_BYTES;
/// Maximum visible line traces in one pane.
pub const MAX_RASTER_TRACES: usize = 256;
/// Maximum source samples resolved across all visible traces.
pub const MAX_RASTER_POINTS: usize = 2_000_000;
/// Maximum Bresenham steps, bounding adversarial zig-zag source data.
pub const MAX_RASTER_STROKE_STEPS: usize = 64 * 1_048_576;
/// Maximum exact immutable bindings accepted for one pane figure.
pub const MAX_RASTER_DATASET_BINDINGS: usize = 256;

const SUPPORTED_VIEWER_ID: &str = "viewer-waveform";
const MAX_EXACT_F64_INTEGER: i64 = 9_007_199_254_740_992;
const PLOT_LEFT: i32 = 40;
const PLOT_RIGHT: i32 = 16;
const PLOT_TOP: i32 = 16;
const PLOT_BOTTOM: i32 = 28;
const VERTICAL_GRID_DIVISIONS: i32 = 8;
const HORIZONTAL_GRID_DIVISIONS: i32 = 6;

/// One opaque sRGB color. Alpha is absent by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    const fn channels(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

/// Explicit colors used by the software line-plot renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationRasterPalette {
    plot_background: Rgb8,
    grid: Rgb8,
    axes: Rgb8,
    traces: Vec<Rgb8>,
}

impl VisualizationRasterPalette {
    pub fn new(
        plot_background: Rgb8,
        grid: Rgb8,
        axes: Rgb8,
        traces: Vec<Rgb8>,
    ) -> Result<Self, VisualizationRasterError> {
        let palette = Self {
            plot_background,
            grid,
            axes,
            traces,
        };
        palette.validate()?;
        Ok(palette)
    }

    #[must_use]
    pub const fn plot_background(&self) -> Rgb8 {
        self.plot_background
    }

    #[must_use]
    pub const fn grid(&self) -> Rgb8 {
        self.grid
    }

    #[must_use]
    pub const fn axes(&self) -> Rgb8 {
        self.axes
    }

    #[must_use]
    pub fn traces(&self) -> &[Rgb8] {
        &self.traces
    }

    fn validate(&self) -> Result<(), VisualizationRasterError> {
        if self.traces.is_empty() || self.traces.len() > MAX_RASTER_TRACES {
            return Err(VisualizationRasterError::InvalidProfile {
                field: "palette.traces",
                message: format!(
                    "the trace palette requires 1 to {MAX_RASTER_TRACES} opaque RGB colors"
                ),
            });
        }
        Ok(())
    }
}

impl Default for VisualizationRasterPalette {
    fn default() -> Self {
        Self {
            plot_background: Rgb8::new(250, 251, 252),
            grid: Rgb8::new(220, 225, 229),
            axes: Rgb8::new(61, 72, 80),
            traces: vec![
                Rgb8::new(0, 103, 192),
                Rgb8::new(196, 78, 0),
                Rgb8::new(0, 132, 99),
                Rgb8::new(142, 68, 173),
                Rgb8::new(202, 138, 4),
                Rgb8::new(0, 149, 183),
                Rgb8::new(213, 94, 0),
                Rgb8::new(96, 105, 113),
            ],
        }
    }
}

/// Bounded, fully explicit output contract for one rasterized figure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationRasterProfile {
    pixel_width: u32,
    pixel_height: u32,
    background: Rgb8,
    palette: VisualizationRasterPalette,
}

impl VisualizationRasterProfile {
    pub fn new(
        pixel_width: u32,
        pixel_height: u32,
        background: Rgb8,
        palette: VisualizationRasterPalette,
    ) -> Result<Self, VisualizationRasterError> {
        let profile = Self {
            pixel_width,
            pixel_height,
            background,
            palette,
        };
        profile.validate()?;
        Ok(profile)
    }

    #[must_use]
    pub const fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    #[must_use]
    pub const fn pixel_height(&self) -> u32 {
        self.pixel_height
    }

    #[must_use]
    pub const fn background(&self) -> Rgb8 {
        self.background
    }

    #[must_use]
    pub const fn palette(&self) -> &VisualizationRasterPalette {
        &self.palette
    }

    pub fn validate(&self) -> Result<(), VisualizationRasterError> {
        for (field, value) in [
            ("profile.pixel-width", self.pixel_width),
            ("profile.pixel-height", self.pixel_height),
        ] {
            if !(MIN_RASTER_DIMENSION..=MAX_RASTER_DIMENSION).contains(&value) {
                return Err(VisualizationRasterError::InvalidProfile {
                    field,
                    message: format!(
                        "dimensions must be between {MIN_RASTER_DIMENSION} and {MAX_RASTER_DIMENSION} pixels"
                    ),
                });
            }
        }
        let pixels = usize::try_from(self.pixel_width)
            .ok()
            .and_then(|width| {
                usize::try_from(self.pixel_height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .ok_or(VisualizationRasterError::ArithmeticOverflow(
                "profile pixel count",
            ))?;
        if pixels > MAX_RASTER_PIXELS {
            return Err(VisualizationRasterError::ResourceLimit {
                resource: "raster pixels",
                maximum: MAX_RASTER_PIXELS,
                actual: pixels,
            });
        }
        self.palette.validate()
    }
}

impl Default for VisualizationRasterProfile {
    fn default() -> Self {
        Self {
            pixel_width: 1_200,
            pixel_height: 675,
            background: Rgb8::new(255, 255, 255),
            palette: VisualizationRasterPalette::default(),
        }
    }
}

/// One exact, numeric source sample after document resolution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedRasterPoint {
    x: f64,
    y: f64,
}

impl ResolvedRasterPoint {
    #[must_use]
    pub const fn x(self) -> f64 {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> f64 {
        self.y
    }
}

/// A visible trace in immutable document order.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedRasterTrace {
    trace_id: TraceId,
    label: String,
    binding: DatasetBinding,
    points: Vec<ResolvedRasterPoint>,
}

impl ResolvedRasterTrace {
    #[must_use]
    pub const fn trace_id(&self) -> TraceId {
        self.trace_id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn binding(&self) -> DatasetBinding {
        self.binding
    }

    #[must_use]
    pub fn points(&self) -> &[ResolvedRasterPoint] {
        &self.points
    }
}

/// Validated Cartesian line scene detached from all mutable application state.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedCartesianLineScene {
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    source_digest: ContentDigest,
    page_id: PageId,
    pane_id: PaneId,
    dataset_bindings: Vec<DatasetBinding>,
    x_range: AxisRange,
    y_range: AxisRange,
    traces: Vec<ResolvedRasterTrace>,
}

impl ResolvedCartesianLineScene {
    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn source_digest(&self) -> ContentDigest {
        self.source_digest
    }

    #[must_use]
    pub const fn page_id(&self) -> PageId {
        self.page_id
    }

    #[must_use]
    pub const fn pane_id(&self) -> PaneId {
        self.pane_id
    }

    #[must_use]
    pub fn dataset_bindings(&self) -> &[DatasetBinding] {
        &self.dataset_bindings
    }

    #[must_use]
    pub const fn x_range(&self) -> AxisRange {
        self.x_range
    }

    #[must_use]
    pub const fn y_range(&self) -> AxisRange {
        self.y_range
    }

    #[must_use]
    pub fn traces(&self) -> &[ResolvedRasterTrace] {
        &self.traces
    }
}

/// Provenance and byte identity retained beside the frozen PNG artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationRasterMetadata {
    pub document_id: ResultDocumentId,
    pub source_revision: ObjectRevision,
    pub source_content_digest: ContentDigest,
    pub page_id: PageId,
    pub pane_id: PaneId,
    pub dataset_bindings: Vec<DatasetBinding>,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub media_type: String,
    pub artifact_content_digest: ContentDigest,
    pub artifact_bytes: usize,
}

/// Report-ready PNG plus its exact source and payload metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualizationRasterArtifact {
    metadata: VisualizationRasterMetadata,
    artifact: FrozenReportArtifact,
}

impl VisualizationRasterArtifact {
    #[must_use]
    pub const fn metadata(&self) -> &VisualizationRasterMetadata {
        &self.metadata
    }

    #[must_use]
    pub const fn artifact(&self) -> &FrozenReportArtifact {
        &self.artifact
    }

    #[must_use]
    pub fn into_artifact(self) -> FrozenReportArtifact {
        self.artifact
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VisualizationRasterError {
    #[error("invalid {field}: {message}")]
    InvalidProfile {
        field: &'static str,
        message: String,
    },
    #[error("report reference is not a visualization-document source")]
    InvalidReferenceSource,
    #[error("report reference targets document {expected}, but received {actual}")]
    DocumentIdentityMismatch {
        expected: ResultDocumentId,
        actual: ResultDocumentId,
    },
    #[error("visualization report references require an exact source revision")]
    MissingSourceRevision,
    #[error("report reference expects revision {expected:?}, current revision is {actual:?}")]
    RevisionMismatch {
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("source digest mismatch: expected {expected}, computed {actual}")]
    SourceDigestMismatch {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("report dataset manifest does not exactly match the selected pane sources")]
    DatasetManifestMismatch {
        expected: Vec<DatasetBinding>,
        actual: Vec<DatasetBinding>,
    },
    #[error("duplicate immutable binding for dataset {0}")]
    DuplicateDatasetBinding(crate::product::DatasetId),
    #[error("page {0} is not present in the selected document revision")]
    PageNotFound(u64),
    #[error("pane {0} is not present in the selected document revision")]
    PaneNotFound(u64),
    #[error("pane {pane_id} belongs to page {actual_page_id}, not requested page {page_id}")]
    PanePageMismatch {
        pane_id: u64,
        page_id: u64,
        actual_page_id: u64,
    },
    #[error("pane kind {0:?} is not supported by the Cartesian line renderer")]
    UnsupportedPaneKind(PaneKind),
    #[error("viewer `{0}` is not supported by the deterministic line renderer")]
    UnsupportedViewer(String),
    #[error("unsupported pane style mode: {0}")]
    UnsupportedStyleMode(&'static str),
    #[error("unsupported pane overlay: {0}")]
    UnsupportedOverlay(&'static str),
    #[error("unsupported axis configuration: {0}")]
    UnsupportedAxisConfiguration(String),
    #[error("axis {axis_id} uses unsupported scale {scale:?}")]
    UnsupportedAxisScale { axis_id: u64, scale: AxisScale },
    #[error("trace {trace_id} column `{column}` uses unsupported value type {value_type:?}")]
    UnsupportedValueType {
        trace_id: u64,
        column: String,
        value_type: ValueType,
    },
    #[error(
        "trace {trace_id} column `{column}` integer {value} is not exactly representable as f64"
    )]
    InexactInteger {
        trace_id: u64,
        column: String,
        value: i64,
    },
    #[error("dataset {0} is not attached to the selected document revision")]
    DatasetNotFound(crate::product::DatasetId),
    #[error(
        "dataset {dataset_id} digest mismatch: trace binds {requested}, document contains {actual}"
    )]
    DatasetDigestMismatch {
        dataset_id: crate::product::DatasetId,
        requested: ContentDigest,
        actual: ContentDigest,
    },
    #[error("trace {trace_id} references missing column `{column}`")]
    ColumnNotFound { trace_id: u64, column: String },
    #[error("trace {0} has no immutable source samples")]
    EmptyTrace(u64),
    #[error("the selected pane has no visible line traces")]
    NoVisibleTraces,
    #[error("invalid numeric range for {0}")]
    InvalidNumericRange(&'static str),
    #[error("{resource} exceeds the limit of {maximum}; received {actual}")]
    ResourceLimit {
        resource: &'static str,
        maximum: usize,
        actual: usize,
    },
    #[error("arithmetic overflow while calculating {0}")]
    ArithmeticOverflow(&'static str),
    #[error("memory allocation failed for {0}")]
    Allocation(&'static str),
    #[error("canonical visualization serialization failed: {0}")]
    SourceSerialization(String),
    #[error("PNG encoding failed: {0}")]
    PngEncoding(String),
    #[error("frozen report artifact validation failed: {0}")]
    ReportArtifact(String),
}

/// Returns the visualization domain's canonical digest for the exact document.
///
/// This adapter deliberately delegates serialization and digest ownership to
/// [`VisualizationDocument`]. Report rendering must never define a second
/// canonicalization algorithm that could disagree with the reference issuer.
pub fn visualization_document_content_digest(
    document: &VisualizationDocument,
) -> Result<ContentDigest, VisualizationRasterError> {
    document
        .content_digest()
        .map_err(|error| VisualizationRasterError::SourceSerialization(error.to_string()))
}

/// Resolve one exact report reference/page/pane into a platform-independent
/// line scene. Every report source identity and immutable binding is checked
/// before any pixels are allocated.
pub fn resolve_cartesian_line_scene(
    document: &VisualizationDocument,
    reference: &ReportReferenceSnapshot,
    page_id: PageId,
    pane_id: PaneId,
) -> Result<ResolvedCartesianLineScene, VisualizationRasterError> {
    let ReportSourceId::VisualizationDocument {
        document_id: expected_document_id,
    } = &reference.source
    else {
        return Err(VisualizationRasterError::InvalidReferenceSource);
    };
    if *expected_document_id != document.id() {
        return Err(VisualizationRasterError::DocumentIdentityMismatch {
            expected: *expected_document_id,
            actual: document.id(),
        });
    }
    let expected_revision = reference
        .source_revision
        .ok_or(VisualizationRasterError::MissingSourceRevision)?;
    if expected_revision != document.revision() {
        return Err(VisualizationRasterError::RevisionMismatch {
            expected: expected_revision,
            actual: document.revision(),
        });
    }
    let source_digest = visualization_document_content_digest(document)?;
    if source_digest != reference.content_digest {
        return Err(VisualizationRasterError::SourceDigestMismatch {
            expected: reference.content_digest,
            actual: source_digest,
        });
    }

    document
        .pages()
        .iter()
        .find(|page| page.id == page_id)
        .ok_or(VisualizationRasterError::PageNotFound(page_id.get()))?;
    let pane = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .ok_or(VisualizationRasterError::PaneNotFound(pane_id.get()))?;
    if pane.page_id != page_id {
        return Err(VisualizationRasterError::PanePageMismatch {
            pane_id: pane_id.get(),
            page_id: page_id.get(),
            actual_page_id: pane.page_id.get(),
        });
    }
    if pane.kind != PaneKind::Cartesian {
        return Err(VisualizationRasterError::UnsupportedPaneKind(pane.kind));
    }
    if pane.viewer_id != SUPPORTED_VIEWER_ID {
        return Err(VisualizationRasterError::UnsupportedViewer(
            pane.viewer_id.clone(),
        ));
    }
    if pane.family_policy.is_some() {
        return Err(VisualizationRasterError::UnsupportedStyleMode(
            "family presentation",
        ));
    }
    reject_unsupported_overlays(document, pane_id)?;

    let pane_axes = document
        .axes()
        .iter()
        .filter(|axis| axis.pane_id == pane_id)
        .collect::<Vec<_>>();
    let x_axis = unique_axis(&pane_axes, AxisOrientation::Horizontal, "horizontal")?;
    let y_axis = unique_axis(&pane_axes, AxisOrientation::VerticalLeft, "vertical-left")?;
    if pane_axes.len() != 2 {
        return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
            "exactly one horizontal and one vertical-left axis are required".to_owned(),
        ));
    }
    require_linear_axis(x_axis)?;
    require_linear_axis(y_axis)?;

    let visible = document
        .traces()
        .iter()
        .filter(|trace| trace.pane_id == pane_id && trace.visible)
        .collect::<Vec<_>>();
    if visible.is_empty() {
        return Err(VisualizationRasterError::NoVisibleTraces);
    }
    if visible.len() > MAX_RASTER_TRACES {
        return Err(VisualizationRasterError::ResourceLimit {
            resource: "visible raster traces",
            maximum: MAX_RASTER_TRACES,
            actual: visible.len(),
        });
    }

    let mut actual_bindings = Vec::new();
    if let Some(binding) = pane.binding {
        // Pane-level bindings participate in provenance even when every
        // visible trace happens to use another immutable dataset.
        dataset_for_binding(document, binding.dataset)?;
        insert_actual_binding(&mut actual_bindings, binding.dataset)?;
    }
    let mut traces = Vec::with_capacity(visible.len());
    let mut total_points = 0_usize;
    for trace in visible {
        if trace.x_axis_id != x_axis.id || trace.y_axis_id != y_axis.id {
            return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
                format!(
                    "trace {} must use the pane's sole horizontal and vertical-left axes",
                    trace.id.get()
                ),
            ));
        }
        let dataset = dataset_for_binding(document, trace.binding)?;
        insert_actual_binding(&mut actual_bindings, trace.binding)?;
        let coordinate_columns = dataset
            .columns()
            .iter()
            .filter(|column| column.role() == ColumnRole::Coordinate)
            .count();
        if coordinate_columns != 1 {
            return Err(VisualizationRasterError::UnsupportedStyleMode(
                "multi-dimensional family source",
            ));
        }
        let x_index = column_index(dataset, trace.id, &trace.coordinate_key)?;
        let y_index = column_index(dataset, trace.id, &trace.signal_key)?;
        let x_column = &dataset.columns()[x_index];
        let y_column = &dataset.columns()[y_index];
        if x_column.role() != ColumnRole::Coordinate || y_column.role() != ColumnRole::Signal {
            return Err(VisualizationRasterError::UnsupportedStyleMode(
                "trace source-column role",
            ));
        }
        require_numeric_column(trace.id, x_column.key(), x_column.value_type())?;
        require_numeric_column(trace.id, y_column.key(), y_column.value_type())?;
        if dataset.rows().is_empty() {
            return Err(VisualizationRasterError::EmptyTrace(trace.id.get()));
        }
        total_points = total_points.checked_add(dataset.rows().len()).ok_or(
            VisualizationRasterError::ArithmeticOverflow("resolved raster point count"),
        )?;
        if total_points > MAX_RASTER_POINTS {
            return Err(VisualizationRasterError::ResourceLimit {
                resource: "resolved raster points",
                maximum: MAX_RASTER_POINTS,
                actual: total_points,
            });
        }
        let mut points = Vec::new();
        points
            .try_reserve_exact(dataset.rows().len())
            .map_err(|_| VisualizationRasterError::Allocation("resolved raster points"))?;
        for row in dataset.rows() {
            let x = numeric_value(&row.values()[x_index], trace.id, x_column.key())?;
            let y = numeric_value(&row.values()[y_index], trace.id, y_column.key())?;
            points.push(ResolvedRasterPoint { x, y });
        }
        traces.push(ResolvedRasterTrace {
            trace_id: trace.id,
            label: trace.label.clone(),
            binding: trace.binding,
            points,
        });
    }

    canonicalize_actual_bindings(&mut actual_bindings);
    let expected_bindings = canonicalize_expected_bindings(&reference.dataset_bindings)?;
    if expected_bindings != actual_bindings {
        return Err(VisualizationRasterError::DatasetManifestMismatch {
            expected: expected_bindings,
            actual: actual_bindings,
        });
    }

    let x_range = resolve_axis_range(
        x_axis.range,
        traces
            .iter()
            .flat_map(|trace| trace.points.iter().map(|point| point.x)),
        "horizontal axis",
    )?;
    let y_range = resolve_axis_range(
        y_axis.range,
        traces
            .iter()
            .flat_map(|trace| trace.points.iter().map(|point| point.y)),
        "vertical axis",
    )?;

    Ok(ResolvedCartesianLineScene {
        document_id: document.id(),
        revision: document.revision(),
        source_digest,
        page_id,
        pane_id,
        dataset_bindings: actual_bindings,
        x_range,
        y_range,
        traces,
    })
}

/// Rasterize a previously validated immutable scene with fixed software rules.
pub fn rasterize_cartesian_line_scene(
    scene: &ResolvedCartesianLineScene,
    profile: &VisualizationRasterProfile,
) -> Result<VisualizationRasterArtifact, VisualizationRasterError> {
    profile.validate()?;
    validate_range(scene.x_range, "horizontal axis")?;
    validate_range(scene.y_range, "vertical axis")?;

    let mut canvas = Canvas::new(profile)?;
    let plot = PixelRect {
        left: PLOT_LEFT,
        top: PLOT_TOP,
        right: i32::try_from(profile.pixel_width)
            .map_err(|_| VisualizationRasterError::ArithmeticOverflow("plot width"))?
            - 1
            - PLOT_RIGHT,
        bottom: i32::try_from(profile.pixel_height)
            .map_err(|_| VisualizationRasterError::ArithmeticOverflow("plot height"))?
            - 1
            - PLOT_BOTTOM,
    };
    if plot.width() < 2 || plot.height() < 2 {
        return Err(VisualizationRasterError::InvalidProfile {
            field: "profile.dimensions",
            message: "dimensions leave no positive Cartesian plot area".to_owned(),
        });
    }
    canvas.fill_rect(plot, profile.palette.plot_background);
    draw_grid(&mut canvas, plot, &profile.palette);

    let mut stroke_steps = 0_usize;
    for (trace_index, trace) in scene.traces.iter().enumerate() {
        let color = profile.palette.traces[trace_index % profile.palette.traces.len()];
        if trace.points.len() == 1 {
            let point = trace.points[0];
            if point_in_range(point, scene.x_range, scene.y_range) {
                let (x, y) = map_point(point, scene.x_range, scene.y_range, plot)?;
                canvas.fill_rect(
                    PixelRect {
                        left: x - 1,
                        right: x + 1,
                        top: y - 1,
                        bottom: y + 1,
                    },
                    color,
                );
            }
            continue;
        }
        for pair in trace.points.windows(2) {
            let Some((start, end)) = clip_segment(pair[0], pair[1], scene.x_range, scene.y_range)
            else {
                continue;
            };
            let (x0, y0) = map_point(start, scene.x_range, scene.y_range, plot)?;
            let (x1, y1) = map_point(end, scene.x_range, scene.y_range, plot)?;
            let segment_steps = usize::try_from((x1 - x0).abs().max((y1 - y0).abs()))
                .unwrap_or(usize::MAX)
                .saturating_add(1);
            stroke_steps = stroke_steps.checked_add(segment_steps).ok_or(
                VisualizationRasterError::ArithmeticOverflow("raster stroke steps"),
            )?;
            if stroke_steps > MAX_RASTER_STROKE_STEPS {
                return Err(VisualizationRasterError::ResourceLimit {
                    resource: "raster stroke steps",
                    maximum: MAX_RASTER_STROKE_STEPS,
                    actual: stroke_steps,
                });
            }
            canvas.line(x0, y0, x1, y1, color);
        }
    }

    let png = encode_rgb8_png(profile.pixel_width, profile.pixel_height, canvas.pixels())?;
    let artifact = FrozenReportArtifact::new(VISUALIZATION_RASTER_MEDIA_TYPE, png)
        .map_err(|error| VisualizationRasterError::ReportArtifact(error.to_string()))?;
    let metadata = VisualizationRasterMetadata {
        document_id: scene.document_id,
        source_revision: scene.revision,
        source_content_digest: scene.source_digest,
        page_id: scene.page_id,
        pane_id: scene.pane_id,
        dataset_bindings: scene.dataset_bindings.clone(),
        pixel_width: profile.pixel_width,
        pixel_height: profile.pixel_height,
        media_type: VISUALIZATION_RASTER_MEDIA_TYPE.to_owned(),
        artifact_content_digest: artifact.content_digest(),
        artifact_bytes: artifact.payload().len(),
    };
    Ok(VisualizationRasterArtifact { metadata, artifact })
}

/// Resolve and rasterize an exact report figure in one fail-closed operation.
pub fn render_visualization_report_figure(
    document: &VisualizationDocument,
    reference: &ReportReferenceSnapshot,
    page_id: PageId,
    pane_id: PaneId,
    profile: &VisualizationRasterProfile,
) -> Result<VisualizationRasterArtifact, VisualizationRasterError> {
    let scene = resolve_cartesian_line_scene(document, reference, page_id, pane_id)?;
    rasterize_cartesian_line_scene(&scene, profile)
}

fn reject_unsupported_overlays(
    document: &VisualizationDocument,
    pane_id: PaneId,
) -> Result<(), VisualizationRasterError> {
    for (kind, present) in [
        (
            "cursor",
            document
                .cursors()
                .iter()
                .any(|item| item.pane_id == pane_id),
        ),
        (
            "marker",
            document
                .markers()
                .iter()
                .any(|item| item.pane_id == pane_id),
        ),
        (
            "measurement",
            document
                .measurements()
                .iter()
                .any(|item| item.pane_id == pane_id),
        ),
        (
            "annotation",
            document
                .annotations()
                .iter()
                .any(|item| item.pane_id == pane_id),
        ),
    ] {
        if present {
            return Err(VisualizationRasterError::UnsupportedOverlay(kind));
        }
    }
    Ok(())
}

fn unique_axis<'a>(
    axes: &[&'a Axis],
    orientation: AxisOrientation,
    label: &'static str,
) -> Result<&'a Axis, VisualizationRasterError> {
    let mut matching = axes
        .iter()
        .copied()
        .filter(|axis| axis.orientation == orientation);
    let axis = matching.next().ok_or_else(|| {
        VisualizationRasterError::UnsupportedAxisConfiguration(format!(
            "one {label} axis is required"
        ))
    })?;
    if matching.next().is_some() {
        return Err(VisualizationRasterError::UnsupportedAxisConfiguration(
            format!("only one {label} axis is supported"),
        ));
    }
    Ok(axis)
}

fn require_linear_axis(axis: &Axis) -> Result<(), VisualizationRasterError> {
    if axis.scale != AxisScale::Linear {
        return Err(VisualizationRasterError::UnsupportedAxisScale {
            axis_id: axis.id.get(),
            scale: axis.scale,
        });
    }
    Ok(())
}

fn dataset_for_binding(
    document: &VisualizationDocument,
    binding: DatasetBinding,
) -> Result<&SourceDataset, VisualizationRasterError> {
    let dataset = document
        .datasets()
        .iter()
        .find(|dataset| dataset.binding().dataset_id == binding.dataset_id)
        .ok_or(VisualizationRasterError::DatasetNotFound(
            binding.dataset_id,
        ))?;
    if dataset.binding().content_digest != binding.content_digest {
        return Err(VisualizationRasterError::DatasetDigestMismatch {
            dataset_id: binding.dataset_id,
            requested: binding.content_digest,
            actual: dataset.binding().content_digest,
        });
    }
    Ok(dataset)
}

fn column_index(
    dataset: &SourceDataset,
    trace_id: TraceId,
    key: &str,
) -> Result<usize, VisualizationRasterError> {
    dataset
        .columns()
        .iter()
        .position(|column| column.key() == key)
        .ok_or_else(|| VisualizationRasterError::ColumnNotFound {
            trace_id: trace_id.get(),
            column: key.to_owned(),
        })
}

fn require_numeric_column(
    trace_id: TraceId,
    column: &str,
    value_type: ValueType,
) -> Result<(), VisualizationRasterError> {
    if !matches!(value_type, ValueType::Real | ValueType::Integer) {
        return Err(VisualizationRasterError::UnsupportedValueType {
            trace_id: trace_id.get(),
            column: column.to_owned(),
            value_type,
        });
    }
    Ok(())
}

fn numeric_value(
    value: &TypedValue,
    trace_id: TraceId,
    column: &str,
) -> Result<f64, VisualizationRasterError> {
    match value {
        TypedValue::Real(value) if value.is_finite() => Ok(*value),
        TypedValue::Integer(value)
            if (-MAX_EXACT_F64_INTEGER..=MAX_EXACT_F64_INTEGER).contains(value) =>
        {
            Ok(*value as f64)
        }
        TypedValue::Integer(value) => Err(VisualizationRasterError::InexactInteger {
            trace_id: trace_id.get(),
            column: column.to_owned(),
            value: *value,
        }),
        other => Err(VisualizationRasterError::UnsupportedValueType {
            trace_id: trace_id.get(),
            column: column.to_owned(),
            value_type: other.value_type(),
        }),
    }
}

fn insert_actual_binding(
    bindings: &mut Vec<DatasetBinding>,
    binding: DatasetBinding,
) -> Result<(), VisualizationRasterError> {
    if let Some(existing) = bindings
        .iter()
        .find(|existing| existing.dataset_id == binding.dataset_id)
    {
        if existing.content_digest != binding.content_digest {
            return Err(VisualizationRasterError::DatasetDigestMismatch {
                dataset_id: binding.dataset_id,
                requested: binding.content_digest,
                actual: existing.content_digest,
            });
        }
        return Ok(());
    }
    if bindings.len() >= MAX_RASTER_DATASET_BINDINGS {
        return Err(VisualizationRasterError::ResourceLimit {
            resource: "pane dataset bindings",
            maximum: MAX_RASTER_DATASET_BINDINGS,
            actual: bindings.len().saturating_add(1),
        });
    }
    bindings.push(binding);
    Ok(())
}

fn canonicalize_actual_bindings(bindings: &mut [DatasetBinding]) {
    bindings.sort_by(binding_order);
}

fn canonicalize_expected_bindings(
    bindings: &[DatasetBinding],
) -> Result<Vec<DatasetBinding>, VisualizationRasterError> {
    if bindings.len() > MAX_RASTER_DATASET_BINDINGS {
        return Err(VisualizationRasterError::ResourceLimit {
            resource: "report dataset bindings",
            maximum: MAX_RASTER_DATASET_BINDINGS,
            actual: bindings.len(),
        });
    }
    let mut canonical = bindings.to_vec();
    canonical.sort_by(binding_order);
    for pair in canonical.windows(2) {
        if pair[0].dataset_id == pair[1].dataset_id {
            return Err(VisualizationRasterError::DuplicateDatasetBinding(
                pair[0].dataset_id,
            ));
        }
    }
    Ok(canonical)
}

fn binding_order(left: &DatasetBinding, right: &DatasetBinding) -> std::cmp::Ordering {
    left.dataset_id
        .as_uuid()
        .as_u128()
        .cmp(&right.dataset_id.as_uuid().as_u128())
        .then_with(|| left.content_digest.cmp(&right.content_digest))
}

fn resolve_axis_range(
    explicit: Option<AxisRange>,
    values: impl Iterator<Item = f64>,
    label: &'static str,
) -> Result<AxisRange, VisualizationRasterError> {
    if let Some(range) = explicit {
        validate_range(range, label)?;
        return Ok(range);
    }
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    for value in values {
        if !value.is_finite() {
            return Err(VisualizationRasterError::InvalidNumericRange(label));
        }
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    }
    if !minimum.is_finite() || !maximum.is_finite() {
        return Err(VisualizationRasterError::InvalidNumericRange(label));
    }
    if minimum == maximum {
        let value = minimum;
        if value == 0.0 {
            minimum = -1.0;
            maximum = 1.0;
        } else {
            let delta = (value.abs() * 0.05).max(f64::MIN_POSITIVE);
            let lower = value - delta;
            let upper = value + delta;
            if lower.is_finite() && lower < value {
                minimum = lower;
            }
            if upper.is_finite() && upper > value {
                maximum = upper;
            }
            if minimum == maximum {
                return Err(VisualizationRasterError::InvalidNumericRange(label));
            }
        }
    }
    let range = AxisRange { minimum, maximum };
    validate_range(range, label)?;
    Ok(range)
}

fn validate_range(range: AxisRange, label: &'static str) -> Result<(), VisualizationRasterError> {
    if !range.minimum.is_finite()
        || !range.maximum.is_finite()
        || range.minimum >= range.maximum
        || !(range.maximum - range.minimum).is_finite()
    {
        return Err(VisualizationRasterError::InvalidNumericRange(label));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct PixelRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl PixelRect {
    const fn width(self) -> i32 {
        self.right - self.left + 1
    }

    const fn height(self) -> i32 {
        self.bottom - self.top + 1
    }
}

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Canvas {
    fn new(profile: &VisualizationRasterProfile) -> Result<Self, VisualizationRasterError> {
        let width = usize::try_from(profile.pixel_width)
            .map_err(|_| VisualizationRasterError::ArithmeticOverflow("raster width"))?;
        let height = usize::try_from(profile.pixel_height)
            .map_err(|_| VisualizationRasterError::ArithmeticOverflow("raster height"))?;
        let sample_bytes = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(3))
            .ok_or(VisualizationRasterError::ArithmeticOverflow(
                "RGB8 sample bytes",
            ))?;
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(sample_bytes)
            .map_err(|_| VisualizationRasterError::Allocation("RGB8 sample buffer"))?;
        pixels.resize(sample_bytes, 0);
        for pixel in pixels.chunks_exact_mut(3) {
            pixel.copy_from_slice(&profile.background.channels());
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    fn set(&mut self, x: i32, y: i32, color: Rgb8) {
        let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
            return;
        };
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.width + x) * 3;
        self.pixels[offset..offset + 3].copy_from_slice(&color.channels());
    }

    fn fill_rect(&mut self, rect: PixelRect, color: Rgb8) {
        let left = rect.left.max(0);
        let top = rect.top.max(0);
        let right = rect.right.min(self.width.saturating_sub(1) as i32);
        let bottom = rect.bottom.min(self.height.saturating_sub(1) as i32);
        if left > right || top > bottom {
            return;
        }
        for y in top..=bottom {
            for x in left..=right {
                self.set(x, y, color);
            }
        }
    }

    fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Rgb8) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            self.set(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let doubled = error * 2;
            if doubled >= dy {
                error += dy;
                x0 += sx;
            }
            if doubled <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}

fn draw_grid(canvas: &mut Canvas, plot: PixelRect, palette: &VisualizationRasterPalette) {
    for division in 1..VERTICAL_GRID_DIVISIONS {
        let x = plot.left + (plot.width() - 1) * division / VERTICAL_GRID_DIVISIONS;
        canvas.line(x, plot.top, x, plot.bottom, palette.grid);
    }
    for division in 1..HORIZONTAL_GRID_DIVISIONS {
        let y = plot.top + (plot.height() - 1) * division / HORIZONTAL_GRID_DIVISIONS;
        canvas.line(plot.left, y, plot.right, y, palette.grid);
    }
    canvas.line(plot.left, plot.top, plot.right, plot.top, palette.axes);
    canvas.line(plot.right, plot.top, plot.right, plot.bottom, palette.axes);
    canvas.line(
        plot.right,
        plot.bottom,
        plot.left,
        plot.bottom,
        palette.axes,
    );
    canvas.line(plot.left, plot.bottom, plot.left, plot.top, palette.axes);
}

fn point_in_range(point: ResolvedRasterPoint, x: AxisRange, y: AxisRange) -> bool {
    (x.minimum..=x.maximum).contains(&point.x) && (y.minimum..=y.maximum).contains(&point.y)
}

fn map_point(
    point: ResolvedRasterPoint,
    x_range: AxisRange,
    y_range: AxisRange,
    plot: PixelRect,
) -> Result<(i32, i32), VisualizationRasterError> {
    let x_unit = normalize(point.x, x_range, "horizontal axis")?;
    let y_unit = normalize(point.y, y_range, "vertical axis")?;
    let x = plot.left + (x_unit * f64::from(plot.width() - 1)).round() as i32;
    let y = plot.bottom - (y_unit * f64::from(plot.height() - 1)).round() as i32;
    Ok((
        x.clamp(plot.left, plot.right),
        y.clamp(plot.top, plot.bottom),
    ))
}

fn normalize(
    value: f64,
    range: AxisRange,
    label: &'static str,
) -> Result<f64, VisualizationRasterError> {
    let span = range.maximum - range.minimum;
    let normalized = (value - range.minimum) / span;
    if !normalized.is_finite() {
        return Err(VisualizationRasterError::InvalidNumericRange(label));
    }
    Ok(normalized.clamp(0.0, 1.0))
}

/// Liang-Barsky clipping in source coordinates prevents an off-screen segment
/// from causing unbounded integer raster work.
fn clip_segment(
    start: ResolvedRasterPoint,
    end: ResolvedRasterPoint,
    x: AxisRange,
    y: AxisRange,
) -> Option<(ResolvedRasterPoint, ResolvedRasterPoint)> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    if !dx.is_finite() || !dy.is_finite() {
        return None;
    }
    let mut lower: f64 = 0.0;
    let mut upper: f64 = 1.0;
    for (p, q) in [
        (-dx, start.x - x.minimum),
        (dx, x.maximum - start.x),
        (-dy, start.y - y.minimum),
        (dy, y.maximum - start.y),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            if ratio > upper {
                return None;
            }
            lower = lower.max(ratio);
        } else {
            if ratio < lower {
                return None;
            }
            upper = upper.min(ratio);
        }
    }
    Some((
        ResolvedRasterPoint {
            x: start.x + lower * dx,
            y: start.y + lower * dy,
        },
        ResolvedRasterPoint {
            x: start.x + upper * dx,
            y: start.y + upper * dy,
        },
    ))
}

fn encode_rgb8_png(
    width: u32,
    height: u32,
    pixels: &[u8],
) -> Result<Vec<u8>, VisualizationRasterError> {
    let mut sink = BoundedBytesWriter::new(MAX_RASTER_ARTIFACT_BYTES);
    let result = (|| {
        let mut encoder = png::Encoder::new(&mut sink, width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::NoCompression);
        encoder.set_filter(png::Filter::NoFilter);
        encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(pixels)?;
        writer.finish()
    })();
    if let Err(error) = result {
        if let Some(actual) = sink.exceeded {
            return Err(VisualizationRasterError::ResourceLimit {
                resource: "encoded PNG bytes",
                maximum: MAX_RASTER_ARTIFACT_BYTES,
                actual,
            });
        }
        return Err(VisualizationRasterError::PngEncoding(error.to_string()));
    }
    Ok(sink.bytes)
}

struct BoundedBytesWriter {
    bytes: Vec<u8>,
    maximum: usize,
    exceeded: Option<usize>,
}

impl BoundedBytesWriter {
    fn new(maximum: usize) -> Self {
        Self {
            bytes: Vec::new(),
            maximum,
            exceeded: None,
        }
    }
}

impl Write for BoundedBytesWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let next = self
            .bytes
            .len()
            .checked_add(buffer.len())
            .ok_or_else(|| io::Error::other("PNG byte count overflow"))?;
        if next > self.maximum {
            self.exceeded = Some(next);
            return Err(io::Error::other("PNG artifact limit exceeded"));
        }
        self.bytes
            .try_reserve(buffer.len())
            .map_err(|_| io::Error::other("PNG output allocation failed"))?;
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use sha2::{Digest as _, Sha256};

    use super::*;
    use crate::product::{DatasetBinding, DatasetId};
    use crate::results::report_document::ReportReferenceSnapshot;
    use crate::results::visualization_document::{
        DocumentEdit, EntityRef, NewAxis, NewTrace, SourceColumn, SourceRow,
    };

    struct Fixture {
        document: VisualizationDocument,
        page_id: PageId,
        pane_id: PaneId,
        binding: DatasetBinding,
    }

    fn fixture(signal_type: ValueType, y_scale: AxisScale) -> Fixture {
        let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x5a; 32]));
        let columns = vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "out",
                "Output",
                signal_type,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ];
        let signal = |value: f64| match signal_type {
            ValueType::Real => TypedValue::Real(value),
            ValueType::Integer => TypedValue::Integer(value as i64),
            ValueType::Boolean => TypedValue::Boolean(value > 0.0),
            ValueType::Text => TypedValue::Text(format!("{value}")),
        };
        let rows = vec![
            SourceRow::new(vec![TypedValue::Real(0.0), signal(0.0)]),
            SourceRow::new(vec![TypedValue::Real(1.0), signal(1.0)]),
            SourceRow::new(vec![TypedValue::Real(2.0), signal(-0.5)]),
            SourceRow::new(vec![TypedValue::Real(3.0), signal(0.75)]),
        ];
        let dataset = SourceDataset::new(binding, columns, rows).unwrap();
        let mut document =
            VisualizationDocument::new("Deterministic figure", vec![dataset]).unwrap();
        let page_id = document.pages()[0].id;
        let pane_id = document.panes()[0].id;
        let axes = document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id,
                        label: "Time".to_owned(),
                        orientation: AxisOrientation::Horizontal,
                        scale: AxisScale::Linear,
                        unit: Some("s".to_owned()),
                        range: None,
                    }),
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id,
                        label: "Output".to_owned(),
                        orientation: AxisOrientation::VerticalLeft,
                        scale: y_scale,
                        unit: Some("V".to_owned()),
                        range: None,
                    }),
                ],
            )
            .unwrap();
        let x_axis_id = match axes.created[0] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        let y_axis_id = match axes.created[1] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddTrace(NewTrace {
                    pane_id,
                    binding,
                    signal_key: "out".to_owned(),
                    coordinate_key: "time".to_owned(),
                    x_axis_id,
                    y_axis_id,
                    label: "V(out)".to_owned(),
                })],
            )
            .unwrap();
        Fixture {
            document,
            page_id,
            pane_id,
            binding,
        }
    }

    fn reference(fixture: &Fixture) -> ReportReferenceSnapshot {
        ReportReferenceSnapshot::new(
            ReportSourceId::VisualizationDocument {
                document_id: fixture.document.id(),
            },
            Some(fixture.document.revision()),
            visualization_document_content_digest(&fixture.document).unwrap(),
            vec![fixture.binding],
        )
        .unwrap()
    }

    #[test]
    fn repeated_rendering_is_byte_for_byte_deterministic_and_authenticated() {
        let fixture = fixture(ValueType::Real, AxisScale::Linear);
        let reference = reference(&fixture);
        let profile = VisualizationRasterProfile::default();
        let first = render_visualization_report_figure(
            &fixture.document,
            &reference,
            fixture.page_id,
            fixture.pane_id,
            &profile,
        )
        .unwrap();
        let second = render_visualization_report_figure(
            &fixture.document,
            &reference,
            fixture.page_id,
            fixture.pane_id,
            &profile,
        )
        .unwrap();

        assert_eq!(first.artifact().payload(), second.artifact().payload());
        assert_eq!(first.metadata(), second.metadata());
        assert_eq!(
            first.metadata().artifact_content_digest,
            ContentDigest::from_bytes(Sha256::digest(first.artifact().payload()).into())
        );
        assert_eq!(
            first.metadata().source_content_digest,
            reference.content_digest
        );
        assert_eq!(first.metadata().dataset_bindings, vec![fixture.binding]);
    }

    #[test]
    fn exact_revision_source_digest_and_dataset_digest_are_required() {
        let fixture = fixture(ValueType::Real, AxisScale::Linear);
        let valid = reference(&fixture);

        let wrong_revision = ReportReferenceSnapshot::new(
            valid.source.clone(),
            Some(fixture.document.revision().next().unwrap()),
            valid.content_digest,
            valid.dataset_bindings.clone(),
        )
        .unwrap();
        assert!(matches!(
            resolve_cartesian_line_scene(
                &fixture.document,
                &wrong_revision,
                fixture.page_id,
                fixture.pane_id
            ),
            Err(VisualizationRasterError::RevisionMismatch { .. })
        ));

        let wrong_source_digest = ReportReferenceSnapshot::new(
            valid.source.clone(),
            valid.source_revision,
            ContentDigest::from_bytes([0xa5; 32]),
            valid.dataset_bindings.clone(),
        )
        .unwrap();
        assert!(matches!(
            resolve_cartesian_line_scene(
                &fixture.document,
                &wrong_source_digest,
                fixture.page_id,
                fixture.pane_id
            ),
            Err(VisualizationRasterError::SourceDigestMismatch { .. })
        ));

        let wrong_binding = DatasetBinding::new(
            fixture.binding.dataset_id,
            ContentDigest::from_bytes([0xbb; 32]),
        );
        let wrong_dataset_digest = ReportReferenceSnapshot::new(
            valid.source,
            valid.source_revision,
            valid.content_digest,
            vec![wrong_binding],
        )
        .unwrap();
        assert!(matches!(
            resolve_cartesian_line_scene(
                &fixture.document,
                &wrong_dataset_digest,
                fixture.page_id,
                fixture.pane_id
            ),
            Err(VisualizationRasterError::DatasetManifestMismatch { .. })
        ));
    }

    #[test]
    fn encoded_png_is_exactly_opaque_rgb8_without_alpha() {
        let fixture = fixture(ValueType::Real, AxisScale::Linear);
        let artifact = render_visualization_report_figure(
            &fixture.document,
            &reference(&fixture),
            fixture.page_id,
            fixture.pane_id,
            &VisualizationRasterProfile::default(),
        )
        .unwrap();
        let decoder = png::Decoder::new(Cursor::new(artifact.artifact().payload()));
        let mut reader = decoder.read_info().unwrap();
        assert_eq!(reader.info().color_type, png::ColorType::Rgb);
        assert_eq!(reader.info().bit_depth, png::BitDepth::Eight);
        assert!(reader.info().trns.is_none());
        let mut decoded = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut decoded).unwrap();
        assert_eq!(info.width, 1_200);
        assert_eq!(info.height, 675);
        assert_eq!(info.color_type, png::ColorType::Rgb);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        assert_eq!(info.buffer_size(), 1_200 * 675 * 3);
    }

    #[test]
    fn profile_dimensions_and_aggregate_pixels_are_bounded() {
        let palette = VisualizationRasterPalette::default();
        for (width, height) in [
            (MIN_RASTER_DIMENSION - 1, 256),
            (MAX_RASTER_DIMENSION + 1, 256),
            (MAX_RASTER_DIMENSION, 2_049),
        ] {
            assert!(
                VisualizationRasterProfile::new(
                    width,
                    height,
                    Rgb8::new(255, 255, 255),
                    palette.clone()
                )
                .is_err()
            );
        }
        assert!(
            VisualizationRasterProfile::new(
                MIN_RASTER_DIMENSION,
                MIN_RASTER_DIMENSION,
                Rgb8::new(255, 255, 255),
                palette
            )
            .is_ok()
        );
        assert!(
            VisualizationRasterPalette::new(
                Rgb8::new(255, 255, 255),
                Rgb8::new(220, 220, 220),
                Rgb8::new(0, 0, 0),
                Vec::new()
            )
            .is_err()
        );
    }

    #[test]
    fn unsupported_viewer_value_and_axis_style_modes_fail_closed() {
        let mut viewer = fixture(ValueType::Real, AxisScale::Linear);
        viewer
            .document
            .transact(
                viewer.document.revision(),
                vec![DocumentEdit::SetPaneSource {
                    pane_id: viewer.pane_id,
                    viewer_id: "viewer-bode".to_owned(),
                    binding: None,
                }],
            )
            .unwrap();
        assert!(matches!(
            resolve_cartesian_line_scene(
                &viewer.document,
                &reference(&viewer),
                viewer.page_id,
                viewer.pane_id
            ),
            Err(VisualizationRasterError::UnsupportedViewer(_))
        ));

        let values = fixture(ValueType::Boolean, AxisScale::Linear);
        assert!(matches!(
            resolve_cartesian_line_scene(
                &values.document,
                &reference(&values),
                values.page_id,
                values.pane_id
            ),
            Err(VisualizationRasterError::UnsupportedValueType { .. })
        ));

        let style = fixture(ValueType::Real, AxisScale::Logarithmic);
        assert!(matches!(
            resolve_cartesian_line_scene(
                &style.document,
                &reference(&style),
                style.page_id,
                style.pane_id
            ),
            Err(VisualizationRasterError::UnsupportedAxisScale { .. })
        ));
    }
}
