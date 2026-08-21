//! Persistent Visualization Studio surface.
//!
//! This is the authoring projection of the same immutable result datasets
//! rendered by the Results quick view. It owns no solver samples and never
//! copies a result into a parallel data store. Viewer compatibility is
//! fail-closed: a catalog entry is selectable only when both its data contract
//! and a real Rust renderer are available.

mod actions;
mod chrome;
mod dock;

use actions::*;
use chrome::*;
mod sections;
mod stage;

use sections::*;
use stage::*;

use dock::{
    active_family_sample_selection, concept_banner, dock_action, dock_body, empty_note,
    labeled_combo, numeric_policy, paint_bottom_rule, paint_top_rule, panel_heading, policy_row,
    property_row, separator, table_header,
};

use std::collections::{BTreeMap, BTreeSet, HashSet};

use egui::{
    Align, Align2, Color32, Frame, Grid, Id, Layout, Margin, Rect, RichText, ScrollArea, Sense,
    Stroke, Ui, Vec2, vec2,
};
use serde::{Deserialize, Serialize};

use crate::analysis::calculator;
use crate::diagnostics::ConsoleMessage;
use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerArt, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerGroup, viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AccessibleColorPalette, AxisOrientation, ColumnRole, ComparisonAlignmentMethod,
    ComparisonExecutionContract, ComparisonExtrapolationPolicy, ComparisonInterpolationPolicy,
    ComparisonPolicy, ComparisonPrecisionPolicy, ComparisonReceipt, ComparisonRequest,
    ComparisonResamplingPolicy, CursorId, DocumentEdit, EntityRef, FamilyAggregationMethod,
    FamilyAggregationPolicy, FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap,
    FamilyPresentationPolicy, FamilyXDimension, FamilyXOrdering, LinkKind, MissingPointPolicy,
    NumericTolerance, PageUpdatePolicy, PaneId, RowAlignmentPolicy, SourceColumn, SourceDataset,
    SourceRow, TypedValue, ValueType, VisualizationDocument, VisualizationTransactionReceipt,
    compare_source_datasets,
};
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, SensitivityResultMode,
    SensitivityResultRow, SimulationRun,
};
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};
use crate::workbench::{AppState, RSpiceApp};

use crate::workbench::{
    ChoicePreference, ResultViewer, RouteTransitionSource, ScalarPreference, SurfaceId,
    SurfaceRoute,
    design_system::WorkbenchIcon,
    state::{Workspace, WorkspaceDocumentId},
};

use super::result_document;
use super::visualization_family::{FamilyManifest, FamilyValueKind, SourceSampleSelection};

const SUMMARY: &str = "Compose waveform, tabular, statistical, RF, eye, field, and report-page views with family slicing, exact axes, annotations, measurements, and large-data policies.";
const EVIDENCE: &str = "Implemented visualization entities retain dataset and analysis identities, viewer type, pane/page placement, links, exact markers, scalar measurements, and annotations without mutating source samples.";
const OWNERSHIP: &str =
    "Owns result-document presentation entities, not solver data or release decisions.";
const COMPACT_BREAKPOINT: f32 = 820.0;
const NARROW_VIEWER_BREAKPOINT: f32 = 1_100.0;
const TOUCH_DOCK_HEIGHT: f32 = 52.0;
const WORKSPACE_HEADER_HEIGHT: f32 = 58.0;
const WORKSPACE_HEADER_VERTICAL_MARGIN: f32 = 7.0;
const SECTION_NAVIGATION_HEIGHT: f32 = 36.0;
const VIEWER_TOOLBAR_HEIGHT: f32 = 36.0;
const VIEWER_TOOLBAR_VERTICAL_MARGIN: f32 = 5.0;
const VIEWER_STAGE_HEADER_HEIGHT: f32 = 44.0;
const VIEWER_STAGE_HEADER_VERTICAL_MARGIN: f32 = 6.0;
const VIEWER_STAGE_STATUS_HEIGHT: f32 = 27.0;
const VIEWER_STAGE_STATUS_VERTICAL_MARGIN: f32 = 4.0;
const PANEL_HEADING_HEIGHT: f32 = 29.0;
const EXACT_DATA_CARD_PADDING: f32 = 12.0;
const EXACT_DATA_TABLE_HEIGHT: f32 = 102.0;
const EXACT_DATA_DOCK_HEIGHT: f32 =
    EXACT_DATA_CARD_PADDING * 2.0 + PANEL_HEADING_HEIGHT + EXACT_DATA_TABLE_HEIGHT;
const NATIVE_VIEWERS: [ResultViewer; 12] = [
    ResultViewer::Waves,
    ResultViewer::DcSweep,
    ResultViewer::Bode,
    ResultViewer::Fft,
    ResultViewer::HarmonicBalance,
    ResultViewer::PhaseNoise,
    ResultViewer::Eye,
    ResultViewer::Hist,
    ResultViewer::Contribution,
    ResultViewer::Specs,
    ResultViewer::Smith,
    ResultViewer::PoleZero,
];

const fn document_pane_kind(art: ViewerArt) -> crate::results::visualization_document::PaneKind {
    use crate::results::visualization_document::PaneKind;
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        _ => PaneKind::Cartesian,
    }
}

const fn bar_content_height(target_height: f32, vertical_margin: f32) -> f32 {
    target_height - vertical_margin * 2.0
}

const fn uses_horizontal_kpi_strip(width: f32, coarse_pointer: bool, touch_screen: bool) -> bool {
    width <= COMPACT_BREAKPOINT || coarse_pointer || touch_screen
}

fn viewer_column_rects(rect: Rect, library_width: f32, inspector_width: f32) -> [Rect; 3] {
    let library = Rect::from_min_size(rect.min, vec2(library_width, rect.height()));
    let inspector = Rect::from_min_max(
        egui::pos2(rect.right() - inspector_width, rect.top()),
        rect.max,
    );
    let stage = Rect::from_min_max(
        egui::pos2(library.right() + 1.0, rect.top()),
        egui::pos2(inspector.left() - 1.0, rect.bottom()),
    );
    [library, stage, inspector]
}

fn visible_available_width(available: f32, cursor_left: f32, clip_right: f32) -> f32 {
    available.min((clip_right - cursor_left).max(1.0)).max(1.0)
}

fn compact_dock_geometry(viewport_width: f32) -> (f32, f32) {
    let window_width = (viewport_width - 18.0).clamp(180.0, 520.0);
    let body_max_width = (window_width - 24.0).max(156.0);
    (window_width, body_max_width)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationSection {
    Document,
    #[default]
    Viewers,
    Axes,
    Families,
    Measurements,
    LargeData,
    ExportReport,
}

impl VisualizationSection {
    const ALL: [Self; 7] = [
        Self::Document,
        Self::Viewers,
        Self::Axes,
        Self::Families,
        Self::Measurements,
        Self::LargeData,
        Self::ExportReport,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Document => "Document",
            Self::Viewers => "Viewers",
            Self::Axes => "Axes",
            Self::Families => "Families",
            Self::Measurements => "Measurements",
            Self::LargeData => "Large data",
            Self::ExportReport => "Export & report",
        }
    }

    pub(super) const fn title(self) -> &'static str {
        match self {
            Self::Document => "Worksheet, panes, pages, and link groups",
            Self::Viewers => "Interactive engineering viewer document",
            Self::Axes => "Axes, transforms, scaling, grids, and units",
            Self::Families => "N-dimensional slicing, pivoting, grouping, and visual encoding",
            Self::Measurements => "Expressions, measurements, cursors, markers, and annotations",
            Self::LargeData => "Streaming, level-of-detail, memory, and exact-value access",
            Self::ExportReport => "Publication, data export, datasheets, and review packages",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::Document => {
                "Arrange real result viewers into one versioned engineering worksheet."
            }
            Self::Viewers => {
                "Inspect exact samples, complex data, markers, cursors, axes, and measurements without leaving the persistent result document."
            }
            Self::Axes => {
                "Each axis declares its physical quantity, transform, range, tick policy, and compatible traces."
            }
            Self::Families => {
                "Use stable dataset and analysis identities for comparisons instead of generated trace indices."
            }
            Self::Measurements => {
                "Derived data remains dependency-tracked and inspectable beside its immutable source samples."
            }
            Self::LargeData => {
                "Display reduction never changes stored precision, measurements, exports, or exact cursor queries."
            }
            Self::ExportReport => {
                "Export exact engineering data or the active rendered viewer with retained provenance."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ViewerTool {
    #[default]
    Select,
    Pan,
    Zoom,
}

impl ViewerTool {
    const ALL: [Self; 3] = [Self::Select, Self::Pan, Self::Zoom];

    const fn label(self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Pan => "Pan",
            Self::Zoom => "Zoom",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationTouchPane {
    #[default]
    Stage,
    Sections,
    Inspector,
    Actions,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationPanePlacement {
    #[default]
    BelowSelected,
    RightOfSelected,
    NewWorksheetPage,
}

impl VisualizationPanePlacement {
    const ALL: [Self; 3] = [
        Self::BelowSelected,
        Self::RightOfSelected,
        Self::NewWorksheetPage,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::BelowSelected => "Below selected pane",
            Self::RightOfSelected => "Right of selected pane",
            Self::NewWorksheetPage => "New worksheet page",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisualizationDock {
    AddPane,
    TraceManager,
    CursorManager,
    DocumentProperties,
    ReorderPanes,
    LinkGroups,
    PageEditor,
    Measurement,
    Annotation,
    FamilySlice,
    FamilyEncoding,
    FamilyFilter,
    Comparison,
    Export,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationPane {
    pub id: u64,
    pub viewer: ResultViewer,
    pub viewer_document_id: String,
    pub dataset_id: DatasetId,
    /// Stable run-local analysis sequence bound to this pane.
    #[serde(default)]
    pub analysis_sequence: u64,
    pub x_link: Option<u64>,
    pub cursor_group: Option<u64>,
    pub page: String,
    #[serde(default)]
    pub placement: VisualizationPanePlacement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationAnnotation {
    pub id: u64,
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub x: f64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationReportPagePolicy {
    pub template: String,
    pub update_policy: PageUpdatePolicy,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationMarker {
    pub id: u64,
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub waveform_name: String,
    pub sample_index: usize,
    pub x: f64,
    pub y: f64,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationMeasurement {
    pub id: u64,
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub expression: String,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ComparisonAlignmentDraft {
    #[default]
    FirstThresholdCrossing,
    AbsoluteXAxis,
    CrossCorrelation,
}

impl ComparisonAlignmentDraft {
    const ALL: [Self; 3] = [
        Self::FirstThresholdCrossing,
        Self::AbsoluteXAxis,
        Self::CrossCorrelation,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::FirstThresholdCrossing => "First threshold crossing",
            Self::AbsoluteXAxis => "Absolute X axis",
            Self::CrossCorrelation => "Cross-correlation alignment",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationDifferenceKind {
    Absolute,
    Relative,
    Normalized,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationDifferenceSeries {
    pub id: u64,
    pub kind: VisualizationDifferenceKind,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationDifferenceTraceSet {
    pub id: u64,
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    pub signal_key: String,
    pub signal_label: String,
    pub coordinate_unit: Option<String>,
    pub coordinates: Vec<f64>,
    pub absolute: VisualizationDifferenceSeries,
    pub relative: VisualizationDifferenceSeries,
    pub normalized: VisualizationDifferenceSeries,
    pub execution: ComparisonExecutionContract,
    /// Normalized difference is `|candidate - baseline| /
    /// (absolute + relative * |baseline|)`.
    pub tolerance: NumericTolerance,
}

impl VisualizationDifferenceTraceSet {
    fn retained_numeric_values(&self) -> Result<usize, String> {
        self.coordinates
            .len()
            .checked_mul(4)
            .ok_or_else(|| "Difference-trace retained-value count overflowed".to_owned())
    }

    fn validate(&self) -> Result<(), String> {
        if self.id == 0
            || self.absolute.id == 0
            || self.relative.id == 0
            || self.normalized.id == 0
            || self.baseline.dataset_id == self.candidate.dataset_id
        {
            return Err(
                "Difference traces require non-zero stable identities and distinct immutable sources"
                    .to_owned(),
            );
        }
        if self.absolute.kind != VisualizationDifferenceKind::Absolute
            || self.relative.kind != VisualizationDifferenceKind::Relative
            || self.normalized.kind != VisualizationDifferenceKind::Normalized
        {
            return Err(
                "Difference-trace series identities do not match their quantities".to_owned(),
            );
        }
        if !self.tolerance.absolute.is_finite()
            || self.tolerance.absolute < 0.0
            || !self.tolerance.relative.is_finite()
            || self.tolerance.relative < 0.0
        {
            return Err("Difference-trace tolerance must be finite and non-negative".to_owned());
        }
        if self.signal_key.trim().is_empty()
            || self.signal_key != self.signal_key.trim()
            || self.signal_key.len() > 256
            || self.signal_key.chars().any(char::is_control)
            || self.signal_label.trim().is_empty()
            || self.signal_label.len() > 1_024
            || self.signal_label.chars().any(char::is_control)
            || self.coordinate_unit.as_ref().is_some_and(|unit| {
                unit.trim().is_empty()
                    || unit != unit.trim()
                    || unit.len() > 64
                    || unit.chars().any(char::is_control)
            })
        {
            return Err("Difference traces require bounded signal and unit metadata".to_owned());
        }
        let row_count = self.coordinates.len();
        if row_count == 0
            || self.absolute.values.len() != row_count
            || self.relative.values.len() != row_count
            || self.normalized.values.len() != row_count
        {
            return Err(
                "Difference-trace coordinate and quantity series must have identical non-zero lengths"
                    .to_owned(),
            );
        }
        if self
            .coordinates
            .iter()
            .any(|coordinate| !coordinate.is_finite())
            || self.coordinates.windows(2).any(|pair| pair[0] >= pair[1])
            || self
                .absolute
                .values
                .iter()
                .chain(&self.relative.values)
                .chain(&self.normalized.values)
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(
                "Difference-trace coordinates must increase and every retained quantity must be finite and non-negative"
                    .to_owned(),
            );
        }
        self.execution.validate().map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationAutoscale {
    #[default]
    RobustVisible,
    ExactExtrema,
    SpecificationBounds,
}

impl VisualizationAutoscale {
    const ALL: [Self; 3] = [
        Self::RobustVisible,
        Self::ExactExtrema,
        Self::SpecificationBounds,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::RobustVisible => "Robust visible data + 5% margin",
            Self::ExactExtrema => "Exact extrema",
            Self::SpecificationBounds => "Specification bounds",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplexProjection {
    #[default]
    MagnitudePhase,
    RealImaginary,
}

impl ComplexProjection {
    const ALL: [Self; 2] = [Self::MagnitudePhase, Self::RealImaginary];

    const fn label(self) -> &'static str {
        match self {
            Self::MagnitudePhase => "Magnitude / phase",
            Self::RealImaginary => "Real / imaginary",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisplayLodPolicy {
    #[default]
    EnvelopePreserving,
    UniformSampling,
    ExactVisibleSamples,
}

impl DisplayLodPolicy {
    const ALL: [Self; 3] = [
        Self::EnvelopePreserving,
        Self::UniformSampling,
        Self::ExactVisibleSamples,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::EnvelopePreserving => "Envelope-preserving multiresolution",
            Self::UniformSampling => "Uniform display sampling",
            Self::ExactVisibleSamples => "Exact visible samples",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OperationState {
    #[default]
    NotStarted,
    Running,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationStudioState {
    #[serde(default)]
    pub section: VisualizationSection,
    #[serde(default)]
    pub tool: ViewerTool,
    #[serde(default = "default_zoom")]
    pub zoom: f32,
    #[serde(default = "default_viewer_document")]
    pub selected_viewer_document: String,
    #[serde(default)]
    pub panes: Vec<VisualizationPane>,
    #[serde(default)]
    pub active_pane: Option<u64>,
    #[serde(default = "default_next_identity")]
    pub next_identity: u64,
    #[serde(default = "default_revision")]
    pub revision: u64,
    #[serde(default)]
    pub annotations: Vec<VisualizationAnnotation>,
    #[serde(default)]
    pub markers: Vec<VisualizationMarker>,
    #[serde(default)]
    pub measurements: Vec<VisualizationMeasurement>,
    #[serde(default)]
    pub linked_x_ranges: BTreeMap<u64, (f64, f64)>,
    #[serde(default)]
    pub linked_cursor_positions: BTreeMap<u64, (Option<f64>, Option<f64>)>,
    #[serde(default)]
    pub pane_x_ranges: BTreeMap<u64, (f64, f64)>,
    #[serde(default)]
    pub pane_cursor_positions: BTreeMap<u64, (Option<f64>, Option<f64>)>,
    #[serde(default)]
    pub family_policies: BTreeMap<u64, FamilyPresentationPolicy>,
    #[serde(default)]
    pub report_page_policies: BTreeMap<String, VisualizationReportPagePolicy>,
    #[serde(default)]
    pub comparison_receipts: Vec<ComparisonReceipt>,
    #[serde(default)]
    pub difference_trace_sets: Vec<VisualizationDifferenceTraceSet>,
    #[serde(default)]
    pub autoscale: VisualizationAutoscale,
    #[serde(default)]
    pub complex_projection: ComplexProjection,
    #[serde(default)]
    pub display_lod: DisplayLodPolicy,
    #[serde(default = "default_tile_memory_mib")]
    pub tile_memory_mib: u32,
    #[serde(default = "default_significant_digits")]
    pub significant_digits: u8,
    #[serde(default)]
    pub touch_pane: VisualizationTouchPane,
    #[serde(skip)]
    viewer_query: String,
    #[serde(skip)]
    dock: Option<VisualizationDock>,
    #[serde(skip)]
    draft_viewer: ResultViewer,
    #[serde(skip)]
    draft_dataset_id: Option<DatasetId>,
    #[serde(skip)]
    draft_analysis_sequence: Option<u64>,
    #[serde(skip)]
    draft_pane_placement: VisualizationPanePlacement,
    #[serde(skip)]
    draft_page_title: String,
    #[serde(skip)]
    draft_annotation: String,
    #[serde(skip)]
    draft_measurement: String,
    #[serde(skip)]
    draft_page_pane: Option<u64>,
    #[serde(skip)]
    draft_page: String,
    #[serde(skip)]
    draft_report_template: String,
    #[serde(skip)]
    draft_report_freeze: bool,
    #[serde(skip)]
    draft_link_pane: Option<u64>,
    #[serde(skip)]
    draft_x_link: u64,
    #[serde(skip)]
    draft_cursor_group: u64,
    #[serde(skip)]
    draft_pane_order: Vec<u64>,
    #[serde(skip)]
    draft_trace_dataset: Option<DatasetId>,
    #[serde(skip)]
    draft_trace_analysis: Option<u64>,
    #[serde(skip)]
    draft_trace_visibility: Vec<(String, bool)>,
    #[serde(skip)]
    draft_significant_digits: Option<u8>,
    #[serde(skip)]
    draft_phase_continuous: Option<bool>,
    #[serde(skip)]
    applied_link_pane: Option<u64>,
    #[serde(skip)]
    pub(crate) family_query: String,
    #[serde(skip)]
    draft_family_x_dimension: String,
    #[serde(skip)]
    draft_family_dimension: String,
    #[serde(skip)]
    draft_family_color_dimension: String,
    #[serde(skip)]
    draft_family_dash_dimension: String,
    #[serde(skip)]
    draft_family_marker_dimension: String,
    #[serde(skip)]
    draft_family_exclude_missing: bool,
    #[serde(skip)]
    draft_comparison_dataset: Option<DatasetId>,
    #[serde(skip)]
    draft_comparison_candidates: Vec<DatasetId>,
    #[serde(skip)]
    draft_comparison_data_version: u64,
    #[serde(skip)]
    draft_comparison_absolute_tolerance: f64,
    #[serde(skip)]
    draft_comparison_relative_tolerance: f64,
    #[serde(skip)]
    draft_comparison_alignment: ComparisonAlignmentDraft,
    #[serde(skip)]
    draft_comparison_alignment_signal: String,
    #[serde(skip)]
    draft_comparison_threshold: f64,
    #[serde(skip)]
    draft_comparison_maximum_lag_samples: u32,
    #[serde(skip)]
    draft_comparison_difference_trace: bool,
    #[serde(skip)]
    operation_state: OperationState,
    #[serde(skip)]
    operation_dataset_id: Option<DatasetId>,
    #[serde(skip)]
    operation_analysis_sequence: Option<u64>,
    #[serde(skip)]
    operation_processed: usize,
    #[serde(skip)]
    operation_total: usize,
    #[serde(skip)]
    operation_checksum: u64,
}

impl Default for VisualizationStudioState {
    fn default() -> Self {
        Self {
            section: VisualizationSection::Viewers,
            tool: ViewerTool::Select,
            zoom: default_zoom(),
            selected_viewer_document: default_viewer_document(),
            panes: Vec::new(),
            active_pane: None,
            next_identity: default_next_identity(),
            revision: default_revision(),
            annotations: Vec::new(),
            markers: Vec::new(),
            measurements: Vec::new(),
            linked_x_ranges: BTreeMap::new(),
            linked_cursor_positions: BTreeMap::new(),
            pane_x_ranges: BTreeMap::new(),
            pane_cursor_positions: BTreeMap::new(),
            family_policies: BTreeMap::new(),
            report_page_policies: BTreeMap::new(),
            comparison_receipts: Vec::new(),
            difference_trace_sets: Vec::new(),
            autoscale: VisualizationAutoscale::default(),
            complex_projection: ComplexProjection::default(),
            display_lod: DisplayLodPolicy::default(),
            tile_memory_mib: default_tile_memory_mib(),
            significant_digits: default_significant_digits(),
            touch_pane: VisualizationTouchPane::Stage,
            viewer_query: String::new(),
            dock: None,
            draft_viewer: ResultViewer::Waves,
            draft_dataset_id: None,
            draft_analysis_sequence: None,
            draft_pane_placement: VisualizationPanePlacement::BelowSelected,
            draft_page_title: String::new(),
            draft_annotation: String::new(),
            draft_measurement: String::new(),
            draft_page_pane: None,
            draft_page: String::new(),
            draft_report_template: "Release verification 4.2".to_owned(),
            draft_report_freeze: false,
            draft_link_pane: None,
            draft_x_link: 0,
            draft_cursor_group: 0,
            draft_pane_order: Vec::new(),
            draft_trace_dataset: None,
            draft_trace_analysis: None,
            draft_trace_visibility: Vec::new(),
            draft_significant_digits: None,
            draft_phase_continuous: None,
            applied_link_pane: None,
            family_query: String::new(),
            draft_family_x_dimension: String::new(),
            draft_family_dimension: String::new(),
            draft_family_color_dimension: String::new(),
            draft_family_dash_dimension: String::new(),
            draft_family_marker_dimension: String::new(),
            draft_family_exclude_missing: false,
            draft_comparison_dataset: None,
            draft_comparison_candidates: Vec::new(),
            draft_comparison_data_version: 0,
            draft_comparison_absolute_tolerance: 0.0,
            draft_comparison_relative_tolerance: 0.0,
            draft_comparison_alignment: ComparisonAlignmentDraft::default(),
            draft_comparison_alignment_signal: String::new(),
            draft_comparison_threshold: 0.0,
            draft_comparison_maximum_lag_samples: 128,
            draft_comparison_difference_trace: true,
            operation_state: OperationState::NotStarted,
            operation_dataset_id: None,
            operation_analysis_sequence: None,
            operation_processed: 0,
            operation_total: 0,
            operation_checksum: 0,
        }
    }
}

impl VisualizationStudioState {
    /// Restore only transient viewer navigation and filtering.
    ///
    /// Pane composition, annotations, measurements, report policies, and
    /// comparison receipts are durable visualization-document content and
    /// deliberately survive View > Reset active view.
    pub(crate) fn reset_transient_view(&mut self) {
        self.tool = ViewerTool::Select;
        self.zoom = default_zoom();
        self.viewer_query.clear();
        self.family_query.clear();
        self.dock = None;
        self.linked_x_ranges.clear();
        self.linked_cursor_positions.clear();
        self.pane_x_ranges.clear();
        self.pane_cursor_positions.clear();
    }
}

const fn default_zoom() -> f32 {
    1.0
}

fn default_viewer_document() -> String {
    "viewer-waveform".to_owned()
}

const fn default_next_identity() -> u64 {
    1
}

const fn default_revision() -> u64 {
    1
}

const fn default_tile_memory_mib() -> u32 {
    crate::ui::plot::DEFAULT_DISPLAY_CACHE_MIB
}

const fn default_significant_digits() -> u8 {
    7
}

const REPORT_PAGE_TEMPLATES: [&str; 3] = [
    "Release verification 4.2",
    "Design review",
    "Model qualification",
];
const MAX_REPORT_PAGE_TITLE_BYTES: usize = 120;
const MAX_COMPARISON_RECEIPTS: usize = 512;
const MAX_DIFFERENCE_TRACE_SETS: usize = 4_096;
const MAX_DIFFERENCE_TRACE_NUMERIC_VALUES: usize = 8_000_000;

impl VisualizationStudioState {
    fn allocate_identity(&mut self) -> Option<u64> {
        let id = self.next_identity;
        self.next_identity = self.next_identity.checked_add(1)?;
        Some(id)
    }

    fn active_pane_mut(&mut self) -> Option<&mut VisualizationPane> {
        let active = self.active_pane?;
        self.panes.iter_mut().find(|pane| pane.id == active)
    }

    fn active_pane(&self) -> Option<&VisualizationPane> {
        let active = self.active_pane?;
        self.panes.iter().find(|pane| pane.id == active)
    }

    fn transact<T>(
        &mut self,
        edit: impl FnOnce(&mut Self) -> Result<T, String>,
    ) -> Result<T, String> {
        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| "Visualization document revision space is exhausted".to_owned())?;
        let snapshot = self.clone();
        match edit(self).and_then(|output| {
            self.validate_presentation()?;
            Ok(output)
        }) {
            Ok(output) => {
                self.revision = next_revision;
                Ok(output)
            }
            Err(error) => {
                *self = snapshot;
                Err(error)
            }
        }
    }

    fn commit_revision(&mut self) -> Result<(), String> {
        self.transact(|_| Ok(()))
    }

    fn validate_presentation(&self) -> Result<(), String> {
        if !self.zoom.is_finite() || !(0.25..=8.0).contains(&self.zoom) {
            return Err("Visualization zoom must be finite and between 25% and 800%".to_owned());
        }
        if !(64..=16_384).contains(&self.tile_memory_mib)
            || !(3..=17).contains(&self.significant_digits)
        {
            return Err("Visualization presentation policy is outside supported bounds".to_owned());
        }
        let mut identities = HashSet::new();
        for pane in &self.panes {
            if pane.id == 0 || !identities.insert(pane.id) {
                return Err("Visualization pane identities must be unique and non-zero".to_owned());
            }
            let Some(canonical_document_id) = pane.viewer.viewer_document_id() else {
                return Err(format!(
                    "Pane {} uses a dataset-native projection that cannot be retained by Visualization Studio",
                    pane.id
                ));
            };
            if pane.viewer_document_id != canonical_document_id {
                return Err(format!(
                    "Pane {} viewer identity does not match its registered document",
                    pane.id
                ));
            }
            if pane.page.trim().is_empty() {
                return Err(format!("Pane {} must belong to a named page", pane.id));
            }
        }
        for (pane_id, policy) in &self.family_policies {
            if !self.panes.iter().any(|pane| pane.id == *pane_id) {
                return Err(format!(
                    "Family presentation policy references missing pane {pane_id}"
                ));
            }
            policy.validate().map_err(|error| error.to_string())?;
        }
        for (page, policy) in &self.report_page_policies {
            if page.trim().is_empty()
                || page != page.trim()
                || page.len() > MAX_REPORT_PAGE_TITLE_BYTES
                || page.chars().any(char::is_control)
                || policy.revision == 0
                || !REPORT_PAGE_TEMPLATES.contains(&policy.template.as_str())
            {
                return Err("Report page policies require a named page, supported template, and non-zero revision".to_owned());
            }
        }
        if self.comparison_receipts.len() > MAX_COMPARISON_RECEIPTS {
            return Err(format!(
                "Visualization comparison history exceeds the supported limit of {MAX_COMPARISON_RECEIPTS} receipts"
            ));
        }
        for receipt in &self.comparison_receipts {
            receipt
                .validate_structure()
                .map_err(|error| error.to_string())?;
        }
        if self.difference_trace_sets.len() > MAX_DIFFERENCE_TRACE_SETS {
            return Err(format!(
                "Visualization difference traces exceed the supported limit of {MAX_DIFFERENCE_TRACE_SETS} signal sets"
            ));
        }
        let mut retained_difference_values = 0_usize;
        for trace_set in &self.difference_trace_sets {
            trace_set.validate()?;
            for identity in [
                trace_set.id,
                trace_set.absolute.id,
                trace_set.relative.id,
                trace_set.normalized.id,
            ] {
                if !identities.insert(identity) {
                    return Err(
                        "Visualization difference-trace identities must be globally unique"
                            .to_owned(),
                    );
                }
            }
            retained_difference_values = retained_difference_values
                .checked_add(trace_set.retained_numeric_values()?)
                .ok_or_else(|| "Difference-trace retained-value count overflowed".to_owned())?;
            if retained_difference_values > MAX_DIFFERENCE_TRACE_NUMERIC_VALUES {
                return Err(format!(
                    "Visualization difference traces exceed the supported limit of {MAX_DIFFERENCE_TRACE_NUMERIC_VALUES} retained numeric values"
                ));
            }
        }
        for annotation in &self.annotations {
            if annotation.id == 0 || !identities.insert(annotation.id) || !annotation.x.is_finite()
            {
                return Err(
                    "Visualization annotations require unique identities and finite anchors"
                        .to_owned(),
                );
            }
            if annotation.text.trim().is_empty() {
                return Err(format!(
                    "Annotation {} must contain review text",
                    annotation.id
                ));
            }
        }
        for marker in &self.markers {
            if marker.id == 0
                || !identities.insert(marker.id)
                || !marker.x.is_finite()
                || !marker.y.is_finite()
            {
                return Err(
                    "Visualization markers require unique identities and finite source values"
                        .to_owned(),
                );
            }
        }
        for measurement in &self.measurements {
            if measurement.id == 0
                || !identities.insert(measurement.id)
                || !measurement.value.is_finite()
                || measurement.expression.trim().is_empty()
            {
                return Err(
                    "Visualization measurements require unique identities, a definition, and a finite value"
                        .to_owned(),
                );
            }
        }
        for range in self
            .linked_x_ranges
            .values()
            .chain(self.pane_x_ranges.values())
        {
            if !range.0.is_finite() || !range.1.is_finite() || range.0 >= range.1 {
                return Err("Visualization X-link ranges must be finite and increasing".to_owned());
            }
        }
        for (a, b) in self
            .linked_cursor_positions
            .values()
            .chain(self.pane_cursor_positions.values())
        {
            if a.is_some_and(|value| !value.is_finite())
                || b.is_some_and(|value| !value.is_finite())
            {
                return Err("Visualization linked cursor positions must be finite".to_owned());
            }
        }
        if self
            .active_pane
            .is_some_and(|active| !self.panes.iter().any(|pane| pane.id == active))
        {
            return Err("Active visualization pane does not exist".to_owned());
        }
        let greatest_identity = identities.into_iter().max().unwrap_or_default();
        if self.next_identity <= greatest_identity {
            return Err("Next visualization identity must exceed every retained entity".to_owned());
        }
        Ok(())
    }

    fn normalize(&mut self) {
        self.zoom = self.zoom.clamp(0.25, 8.0);
        let mut pane_ids = HashSet::with_capacity(self.panes.len());
        self.panes
            .retain(|pane| pane.id != 0 && pane_ids.insert(pane.id));
        if self
            .active_pane
            .is_some_and(|id| !self.panes.iter().any(|pane| pane.id == id))
        {
            self.active_pane = self.panes.first().map(|pane| pane.id);
        }
        if viewer_document(&self.selected_viewer_document).is_none() {
            self.selected_viewer_document = default_viewer_document();
        }
        self.next_identity = self.next_identity.max(
            self.panes
                .iter()
                .map(|pane| pane.id)
                .chain(self.annotations.iter().map(|annotation| annotation.id))
                .chain(self.markers.iter().map(|marker| marker.id))
                .chain(self.measurements.iter().map(|measurement| measurement.id))
                .chain(self.difference_trace_sets.iter().flat_map(|trace_set| {
                    [
                        trace_set.id,
                        trace_set.absolute.id,
                        trace_set.relative.id,
                        trace_set.normalized.id,
                    ]
                }))
                .max()
                .unwrap_or_default()
                .saturating_add(1)
                .max(1),
        );
        self.tile_memory_mib = self.tile_memory_mib.clamp(64, 16_384);
        self.significant_digits = self.significant_digits.clamp(3, 17);
        self.linked_x_ranges
            .retain(|_, range| range.0.is_finite() && range.1.is_finite() && range.0 < range.1);
        self.pane_x_ranges
            .retain(|_, range| range.0.is_finite() && range.1.is_finite() && range.0 < range.1);
        self.linked_cursor_positions
            .retain(|_, (a, b)| a.is_none_or(f64::is_finite) && b.is_none_or(f64::is_finite));
        self.pane_cursor_positions.retain(|pane_id, (a, b)| {
            pane_ids.contains(pane_id)
                && a.is_none_or(f64::is_finite)
                && b.is_none_or(f64::is_finite)
        });
        self.family_policies
            .retain(|pane_id, policy| pane_ids.contains(pane_id) && policy.validate().is_ok());
        self.report_page_policies.retain(|page, policy| {
            !page.trim().is_empty()
                && page == page.trim()
                && page.len() <= MAX_REPORT_PAGE_TITLE_BYTES
                && !page.chars().any(char::is_control)
                && policy.revision != 0
                && REPORT_PAGE_TEMPLATES.contains(&policy.template.as_str())
        });
        self.comparison_receipts
            .retain(|receipt| receipt.validate_structure().is_ok());
        if self.comparison_receipts.len() > MAX_COMPARISON_RECEIPTS {
            self.comparison_receipts
                .drain(..self.comparison_receipts.len() - MAX_COMPARISON_RECEIPTS);
        }
        let mut retained_difference_values = 0_usize;
        self.difference_trace_sets.retain(|trace_set| {
            if trace_set.validate().is_err() {
                return false;
            }
            let Ok(values) = trace_set.retained_numeric_values() else {
                return false;
            };
            let Some(next) = retained_difference_values.checked_add(values) else {
                return false;
            };
            if next > MAX_DIFFERENCE_TRACE_NUMERIC_VALUES {
                return false;
            }
            retained_difference_values = next;
            true
        });
        if self.difference_trace_sets.len() > MAX_DIFFERENCE_TRACE_SETS {
            self.difference_trace_sets
                .truncate(MAX_DIFFERENCE_TRACE_SETS);
        }
    }
}

fn report_visualization_commit(app: &mut RSpiceApp, result: Result<(), String>) -> bool {
    match result {
        Ok(()) => true,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            false
        }
    }
}

fn commit_visualization_revision(app: &mut RSpiceApp) -> bool {
    let result = app.state.workbench.visualization_studio.commit_revision();
    report_visualization_commit(app, result)
}

fn active_project_visualization_document_id(state: &AppState) -> Option<ResultDocumentId> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => Some(*document_id),
        _ => None,
    }
}

fn transact_active_project_document(
    app: &mut RSpiceApp,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, String> {
    let document_id = active_project_visualization_document_id(&app.state)
        .ok_or_else(|| "Open a project-owned result document before editing it.".to_owned())?;
    let revision = app
        .state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?
        .revision();
    app.state
        .workspace
        .transact_visualization_document(document_id, revision, edits)
        .map_err(|error| error.to_string())
}

fn real_cursor_position(cursor: &crate::results::visualization_document::Cursor) -> Option<f64> {
    match &cursor.position {
        TypedValue::Real(position) => Some(*position),
        _ => None,
    }
}

fn same_cursor_position(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
}

fn canonical_cursor_pair(
    document: &VisualizationDocument,
    pane_id: PaneId,
) -> Result<(Option<f64>, Option<f64>), String> {
    let mut pair = (None, None);
    for cursor in document
        .cursors()
        .iter()
        .filter(|cursor| cursor.pane_id == pane_id)
    {
        let slot = match cursor.label.as_str() {
            "A" => &mut pair.0,
            "B" => &mut pair.1,
            _ => continue,
        };
        if slot.is_some() {
            return Err(format!(
                "Pane {} contains more than one retained {} cursor.",
                pane_id.get(),
                cursor.label
            ));
        }
        *slot = Some(real_cursor_position(cursor).ok_or_else(|| {
            format!(
                "Retained {} cursor {} is not a real-valued horizontal cursor.",
                cursor.label,
                cursor.id.get()
            )
        })?);
    }
    Ok(pair)
}

fn canonical_cursor_pair_edits(
    document: &VisualizationDocument,
    pane_id: PaneId,
    desired: (Option<f64>, Option<f64>),
) -> Result<Vec<DocumentEdit>, String> {
    document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .ok_or_else(|| format!("Result-document pane {} no longer exists.", pane_id.get()))?;
    let axis_id = document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
        .map(|axis| axis.id)
        .ok_or_else(|| "The active result pane has no horizontal cursor axis.".to_owned())?;
    if desired
        .0
        .into_iter()
        .chain(desired.1)
        .any(|position| !position.is_finite())
    {
        return Err("A retained cursor position must be finite.".to_owned());
    }

    let mut removed_groups = BTreeSet::new();
    let mut removed_cursors = BTreeSet::new();
    let mut moves = BTreeMap::<CursorId, f64>::new();
    let mut additions = Vec::new();

    for (label, target) in [("A", desired.0), ("B", desired.1)] {
        let existing = document
            .cursors()
            .iter()
            .filter(|cursor| cursor.pane_id == pane_id && cursor.label == label)
            .collect::<Vec<_>>();
        if existing.len() > 1 {
            return Err(format!(
                "Pane {} contains duplicate retained {label} cursors.",
                pane_id.get()
            ));
        }
        match (existing.first().copied(), target) {
            (Some(cursor), Some(position)) => {
                let mut targets = BTreeSet::from([cursor.id]);
                for group in document.link_groups().iter().filter(|group| {
                    group.kind == LinkKind::CursorPosition
                        && group.members.contains(&EntityRef::Cursor(cursor.id))
                }) {
                    for member in &group.members {
                        if let EntityRef::Cursor(cursor_id) = member {
                            targets.insert(*cursor_id);
                        }
                    }
                }
                for cursor_id in targets {
                    let retained = document
                        .cursors()
                        .iter()
                        .find(|candidate| candidate.id == cursor_id)
                        .ok_or_else(|| {
                            format!("Linked cursor {} no longer exists.", cursor_id.get())
                        })?;
                    let retained_position = real_cursor_position(retained).ok_or_else(|| {
                        format!("Linked cursor {} is not real-valued.", cursor_id.get())
                    })?;
                    if !same_cursor_position(retained_position, position) {
                        moves.insert(cursor_id, position);
                    }
                }
            }
            (Some(cursor), None) => {
                let linked_groups = document
                    .link_groups()
                    .iter()
                    .filter(|group| {
                        group.kind == LinkKind::CursorPosition
                            && group.members.contains(&EntityRef::Cursor(cursor.id))
                    })
                    .collect::<Vec<_>>();
                if linked_groups.is_empty() {
                    removed_cursors.insert(cursor.id);
                } else {
                    for group in linked_groups {
                        removed_groups.insert(group.id);
                        for member in &group.members {
                            if let EntityRef::Cursor(cursor_id) = member {
                                removed_cursors.insert(*cursor_id);
                            }
                        }
                    }
                }
            }
            (None, Some(position)) => additions.push(DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position: TypedValue::Real(position),
                label: label.to_owned(),
            }),
            (None, None) => {}
        }
    }

    let mut edits = removed_groups
        .into_iter()
        .map(|group_id| DocumentEdit::Remove(EntityRef::LinkGroup(group_id)))
        .collect::<Vec<_>>();
    edits.extend(
        moves
            .into_iter()
            .filter(|(cursor_id, _)| !removed_cursors.contains(cursor_id))
            .map(|(cursor_id, position)| DocumentEdit::MoveCursor {
                cursor_id,
                position: TypedValue::Real(position),
            }),
    );
    edits.extend(
        removed_cursors
            .into_iter()
            .map(|cursor_id| DocumentEdit::Remove(EntityRef::Cursor(cursor_id))),
    );
    edits.extend(additions);
    Ok(edits)
}

fn commit_active_project_cursor_pair(
    app: &mut RSpiceApp,
    pane_id: u64,
    desired: (Option<f64>, Option<f64>),
) -> bool {
    let Some(document_id) = active_project_visualization_document_id(&app.state) else {
        return false;
    };
    let plan = app
        .state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())
        .and_then(|document| {
            let pane_id = document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == pane_id)
                .map(|pane| pane.id)
                .ok_or_else(|| "The active result pane no longer exists.".to_owned())?;
            canonical_cursor_pair_edits(document, pane_id, desired)
        });
    let edits = match plan {
        Ok(edits) => edits,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            app.state
                .workbench
                .visualization_studio
                .pane_cursor_positions
                .remove(&pane_id);
            app.state.workbench.visualization_studio.applied_link_pane = None;
            reconcile_document(app);
            return true;
        }
    };
    if edits.is_empty() {
        return true;
    }
    let result = transact_active_project_document(app, edits).map(|_| ());
    let committed = report_visualization_commit(app, result);
    if committed {
        reconcile_document(app);
    } else {
        app.state.workbench.visualization_studio.applied_link_pane = None;
        reconcile_document(app);
    }
    true
}

fn set_active_project_cursor_links(app: &mut RSpiceApp, enabled: bool) -> bool {
    let Some(document_id) = active_project_visualization_document_id(&app.state) else {
        return false;
    };
    let active_pane_id = app.state.workbench.visualization_studio.active_pane;
    let plan = app
        .state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())
        .and_then(|document| {
            let mut edits = document
                .link_groups()
                .iter()
                .filter(|group| group.kind == LinkKind::CursorPosition)
                .map(|group| DocumentEdit::Remove(EntityRef::LinkGroup(group.id)))
                .collect::<Vec<_>>();
            if enabled {
                let mut added = 0_usize;
                for label in ["A", "B"] {
                    let candidates = document
                        .cursors()
                        .iter()
                        .filter(|cursor| cursor.label == label)
                        .collect::<Vec<_>>();
                    let Some(reference) = candidates
                        .iter()
                        .copied()
                        .find(|cursor| Some(cursor.pane_id.get()) == active_pane_id)
                        .or_else(|| candidates.first().copied())
                    else {
                        continue;
                    };
                    let reference_axis = document
                        .axes()
                        .iter()
                        .find(|axis| axis.id == reference.axis_id)
                        .ok_or_else(|| {
                            format!("Cursor {} has no retained axis.", reference.id.get())
                        })?;
                    let compatible = candidates
                        .into_iter()
                        .filter(|cursor| {
                            document
                                .axes()
                                .iter()
                                .find(|axis| axis.id == cursor.axis_id)
                                .is_some_and(|axis| {
                                    axis.orientation == AxisOrientation::Horizontal
                                        && axis.scale == reference_axis.scale
                                        && axis.unit == reference_axis.unit
                                })
                        })
                        .collect::<Vec<_>>();
                    if compatible.len() >= 2 {
                        let reference_position =
                            real_cursor_position(reference).ok_or_else(|| {
                                format!("Cursor {} is not real-valued.", reference.id.get())
                            })?;
                        for cursor in &compatible {
                            let position = real_cursor_position(cursor).ok_or_else(|| {
                                format!("Cursor {} is not real-valued.", cursor.id.get())
                            })?;
                            if !same_cursor_position(position, reference_position) {
                                edits.push(DocumentEdit::MoveCursor {
                                    cursor_id: cursor.id,
                                    position: TypedValue::Real(reference_position),
                                });
                            }
                        }
                        edits.push(DocumentEdit::AddLinkGroup {
                            label: format!("{label} cursor link"),
                            kind: LinkKind::CursorPosition,
                            members: compatible
                                .into_iter()
                                .map(|cursor| EntityRef::Cursor(cursor.id))
                                .collect(),
                        });
                        added += 1;
                    }
                }
                if added == 0 {
                    return Err(
                        "Place the same A or B cursor on at least two panes before linking it."
                            .to_owned(),
                    );
                }
            }
            Ok(edits)
        });
    let edits = match plan {
        Ok(edits) => edits,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::warning(error));
            return true;
        }
    };
    if edits.is_empty() {
        return true;
    }
    let result = transact_active_project_document(app, edits).map(|_| ());
    if report_visualization_commit(app, result) {
        reconcile_document(app);
    }
    true
}

fn active_project_pane_and_trace(
    state: &AppState,
    preferred_signal: Option<&str>,
) -> Result<
    (
        crate::results::visualization_document::PaneId,
        crate::results::visualization_document::TraceId,
    ),
    String,
> {
    let document_id = active_project_visualization_document_id(state).ok_or_else(|| {
        "Open a project-owned result document before authoring entities.".to_owned()
    })?;
    let document = state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
    let pane_id = state
        .workbench
        .visualization_studio
        .active_pane
        .and_then(|active| {
            document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == active)
                .map(|pane| pane.id)
        })
        .or_else(|| document.panes().first().map(|pane| pane.id))
        .ok_or_else(|| "The active result document has no pane.".to_owned())?;
    let trace = preferred_signal
        .and_then(|signal| {
            document.traces().iter().find(|trace| {
                trace.pane_id == pane_id && (trace.signal_key == signal || trace.label == signal)
            })
        })
        .or_else(|| {
            document
                .traces()
                .iter()
                .find(|trace| trace.pane_id == pane_id)
        })
        .ok_or_else(|| "The selected result pane has no retained trace.".to_owned())?;
    Ok((pane_id, trace.id))
}

pub(crate) fn open(app: &mut RSpiceApp) {
    if let Err(error) = navigate_to_visualization_studio(app) {
        app.state.push_user_message(ConsoleMessage::warning(error));
    }
}

fn navigate_to_visualization_studio(app: &mut RSpiceApp) -> Result<(), String> {
    let route = SurfaceRoute::surface(SurfaceId::VisualizationStudio);
    app.state
        .workbench
        .navigate(route, RouteTransitionSource::User)
        .map_err(|error| error.to_string())?;
    app.state.workbench.workspace = Workspace::Results;
    app.state.workbench.visualization_studio.normalize();
    app.state
        .workbench
        .specialist_tool_browser
        .record_recent(SurfaceId::VisualizationStudio);
    Ok(())
}

pub(crate) fn open_add_pane(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::AddPane);
}

pub(crate) fn open_trace_manager(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::TraceManager);
}

pub(crate) fn open_cursor_manager(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::CursorManager);
}

pub(crate) fn open_document_properties(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::DocumentProperties);
}

pub(crate) fn open_measurement_editor(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Measurement);
}

pub(crate) fn open_annotation_editor(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Annotation);
}

pub(crate) fn open_family_slicing(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::FamilySlice);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResultsComparisonSource {
    dataset_id: DatasetId,
    analysis_sequence: u64,
    viewer: ResultViewer,
}

fn retained_document_analysis(
    run: &SimulationRun,
    source_id: AnalysisInstanceId,
) -> Option<&AnalysisResult> {
    run.analyses.iter().find(|analysis| {
        analysis.provenance().map_or_else(
            || {
                let name = format!("legacy-analysis-v1/{}", analysis.id);
                AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
                    == source_id
            },
            |provenance| {
                provenance.source_instance_id() == source_id
                    || provenance.authored_source_instance_id() == source_id
            },
        )
    })
}

fn active_results_comparison_source(state: &AppState) -> Result<ResultsComparisonSource, String> {
    if state.workbench.workspace != Workspace::Results {
        return Err("Open a result document before comparing datasets.".to_owned());
    }
    let document = crate::workbench::chrome::document_bar::active_document_id(state)
        .ok_or_else(|| "No active result document is available.".to_owned())?;
    let mut authored_analysis = None;
    let mut viewer = state.ui.results.viewer;
    let (dataset_id, expected_digest) = match document {
        WorkspaceDocumentId::ResultDataset(dataset_id) => {
            state
                .simulation
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset_id)
                .ok_or_else(|| "The active result dataset is no longer retained.".to_owned())?;
            (dataset_id, None)
        }
        WorkspaceDocumentId::VisualizationDocument(document_id) => {
            let document = state
                .workspace
                .visualization_document(document_id)
                .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
            if let Some((pane, pane_binding)) = document
                .panes()
                .iter()
                .find_map(|pane| pane.binding.map(|binding| (pane, binding)))
            {
                authored_analysis = Some(pane_binding.analysis_id);
                viewer = ResultViewer::from_viewer_document_id(&pane.viewer_id)
                    .unwrap_or(ResultViewer::Waves);
                (
                    pane_binding.dataset.dataset_id,
                    Some(pane_binding.dataset.content_digest),
                )
            } else {
                let binding = document
                    .datasets()
                    .first()
                    .map(SourceDataset::binding)
                    .ok_or_else(|| {
                        "The active result document has no immutable dataset binding.".to_owned()
                    })?;
                (binding.dataset_id, Some(binding.content_digest))
            }
        }
        _ => return Err("The active document is not a result dataset.".to_owned()),
    };
    let run = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The active result document's dataset is no longer retained.".to_owned())?;
    if expected_digest.is_some_and(|digest| run.dataset_content_digest() != digest) {
        return Err(
            "The active result document's immutable dataset digest does not match retained data."
                .to_owned(),
        );
    }
    let analysis = if let Some(authored_analysis) = authored_analysis {
        retained_document_analysis(run, authored_analysis).ok_or_else(|| {
            "The active result document's authored analysis is no longer retained.".to_owned()
        })?
    } else if state
        .simulation
        .active_run()
        .is_some_and(|active| active.dataset_id == dataset_id)
    {
        state
            .simulation
            .active_analysis()
            .or_else(|| run.analyses.first())
            .ok_or_else(|| "The active result dataset contains no analysis.".to_owned())?
    } else {
        run.analyses
            .first()
            .ok_or_else(|| "The active result dataset contains no analysis.".to_owned())?
    };
    viewer = result_document::project_viewer_for_analysis(viewer, analysis);
    Ok(ResultsComparisonSource {
        dataset_id,
        analysis_sequence: analysis.id,
        viewer,
    })
}

pub(crate) fn results_comparison_available(state: &AppState) -> bool {
    active_results_comparison_source(state).is_ok_and(|source| {
        !dock::compatible_comparison_dataset_ids(state, source.dataset_id, source.analysis_sequence)
            .is_empty()
    })
}

fn select_results_comparison_source(
    app: &mut RSpiceApp,
    source: ResultsComparisonSource,
) -> Result<(), String> {
    let run_index = app
        .state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == source.dataset_id)
        .ok_or_else(|| "The candidate dataset is no longer retained.".to_owned())?;
    let analysis_index = app.state.simulation.runs[run_index]
        .analyses
        .iter()
        .position(|analysis| analysis.id == source.analysis_sequence)
        .ok_or_else(|| "The candidate analysis is no longer retained.".to_owned())?;
    if !app.state.simulation.select_run(run_index)
        || !app.state.simulation.select_analysis(analysis_index)
    {
        return Err("The candidate dataset could not be activated.".to_owned());
    }
    Ok(())
}

fn bind_comparison_owner(
    app: &mut RSpiceApp,
    source: ResultsComparisonSource,
) -> Result<(), String> {
    let viewer_document = source
        .viewer
        .viewer_document_id()
        .ok_or_else(|| {
            "Dataset-native result projections cannot be bound as Visualization Studio panes"
                .to_owned()
        })?
        .to_owned();
    let studio = &mut app.state.workbench.visualization_studio;
    studio.normalize();
    let pane_id = studio
        .panes
        .iter()
        .find(|pane| {
            pane.dataset_id == source.dataset_id
                && pane.analysis_sequence == source.analysis_sequence
                && pane.viewer == source.viewer
        })
        .map(|pane| pane.id);
    let pane_id = if let Some(pane_id) = pane_id {
        pane_id
    } else {
        studio.transact(|studio| {
            let pane_id = studio
                .allocate_identity()
                .ok_or_else(|| "Visualization pane identity space is exhausted.".to_owned())?;
            studio.panes.push(VisualizationPane {
                id: pane_id,
                viewer: source.viewer,
                viewer_document_id: viewer_document.clone(),
                dataset_id: source.dataset_id,
                analysis_sequence: source.analysis_sequence,
                x_link: None,
                cursor_group: None,
                page: "Engineering".to_owned(),
                placement: VisualizationPanePlacement::BelowSelected,
            });
            Ok(pane_id)
        })?
    };
    studio.active_pane = Some(pane_id);
    studio.selected_viewer_document = viewer_document;
    studio.applied_link_pane = None;
    studio.section = VisualizationSection::Viewers;
    app.state.ui.results.viewer = source.viewer;
    Ok(())
}

pub(crate) fn open_results_comparison(app: &mut RSpiceApp) {
    if let Err(error) = open_results_comparison_inner(app) {
        app.state.push_user_message(ConsoleMessage::warning(error));
    }
}

fn open_results_comparison_inner(app: &mut RSpiceApp) -> Result<(), String> {
    let source = active_results_comparison_source(&app.state)?;
    let comparison_datasets = dock::compatible_comparison_dataset_ids(
        &app.state,
        source.dataset_id,
        source.analysis_sequence,
    );
    if comparison_datasets.is_empty() {
        return Err(
            "A second compatible immutable dataset with an exact matching analysis and coordinate axis is required."
                .to_owned(),
        );
    }
    navigate_to_visualization_studio(app)?;
    select_results_comparison_source(app, source)?;
    bind_comparison_owner(app, source)?;
    let comparison_data_version = app.state.simulation.data_version;
    initialize_comparison_dock(
        &mut app.state.workbench.visualization_studio,
        comparison_datasets,
        comparison_data_version,
    );
    Ok(())
}

pub(crate) fn export_document(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Export);
}

fn initialize_comparison_dock(
    studio: &mut VisualizationStudioState,
    comparison_datasets: Vec<DatasetId>,
    comparison_data_version: u64,
) {
    studio.draft_comparison_dataset = comparison_datasets.first().copied();
    studio.draft_comparison_candidates = comparison_datasets;
    studio.draft_comparison_data_version = comparison_data_version;
    studio.draft_comparison_absolute_tolerance = 0.0;
    studio.draft_comparison_relative_tolerance = 0.0;
    studio.draft_comparison_alignment = ComparisonAlignmentDraft::default();
    studio.draft_comparison_alignment_signal.clear();
    studio.draft_comparison_threshold = 0.0;
    studio.draft_comparison_maximum_lag_samples = 128;
    studio.draft_comparison_difference_trace = true;
    studio.dock = Some(VisualizationDock::Comparison);
}

fn open_dock(app: &mut RSpiceApp, dock: VisualizationDock) {
    if dock == VisualizationDock::CursorManager
        && app.state.ui.results.viewer != ResultViewer::Waves
    {
        app.state.push_user_message(ConsoleMessage::warning(
            "Exact source cursor management is available in the waveform renderer.",
        ));
        return;
    }
    let active_pane = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned();
    let active_binding = app.state.simulation.active_run().and_then(|run| {
        app.state
            .simulation
            .active_analysis()
            .map(|analysis| (run.dataset_id, analysis.id))
    });
    let trace_source = app.state.simulation.active_run().and_then(|run| {
        app.state
            .simulation
            .active_analysis()
            .map(|analysis| (run.dataset_id, analysis.id, analysis.waveforms.clone()))
    });
    let phase_continuous = app.state.ui.results.phase_continuous;
    let active_viewer = app.state.ui.results.viewer;
    let family_manifest = app
        .state
        .simulation
        .active_analysis()
        .and_then(|analysis| FamilyManifest::from_analysis(analysis).ok().flatten());
    let active_family_policy = active_pane.as_ref().and_then(|pane| {
        app.state
            .workbench
            .visualization_studio
            .family_policies
            .get(&pane.id)
            .cloned()
    });
    let comparison_datasets = if dock == VisualizationDock::Comparison {
        active_binding.map_or_else(Vec::new, |(dataset, analysis)| {
            dock::compatible_comparison_dataset_ids(&app.state, dataset, analysis)
        })
    } else {
        Vec::new()
    };
    let comparison_data_version = app.state.simulation.data_version;
    let studio = &mut app.state.workbench.visualization_studio;
    match dock {
        VisualizationDock::AddPane => {
            studio.draft_viewer = active_viewer;
            studio.draft_dataset_id = active_binding.map(|binding| binding.0);
            studio.draft_analysis_sequence = active_binding.map(|binding| binding.1);
            studio.draft_pane_placement = VisualizationPanePlacement::BelowSelected;
            let page_count = studio
                .panes
                .iter()
                .map(|pane| pane.page.as_str())
                .collect::<HashSet<_>>()
                .len();
            studio.draft_page_title = format!("Page {}", page_count.saturating_add(1));
        }
        VisualizationDock::TraceManager => {
            if let Some((dataset_id, analysis_id, waveforms)) = trace_source {
                studio.draft_trace_dataset = Some(dataset_id);
                studio.draft_trace_analysis = Some(analysis_id);
                studio.draft_trace_visibility = waveforms
                    .into_iter()
                    .map(|waveform| (waveform.name, waveform.visible))
                    .collect();
            } else {
                studio.draft_trace_dataset = None;
                studio.draft_trace_analysis = None;
                studio.draft_trace_visibility.clear();
            }
        }
        VisualizationDock::DocumentProperties => {
            studio.draft_significant_digits = Some(studio.significant_digits);
            studio.draft_phase_continuous = Some(phase_continuous);
        }
        VisualizationDock::ReorderPanes => {
            studio.draft_pane_order = studio.panes.iter().map(|pane| pane.id).collect();
        }
        VisualizationDock::LinkGroups => {
            studio.draft_link_pane = active_pane.as_ref().map(|pane| pane.id);
            studio.draft_x_link = active_pane
                .as_ref()
                .and_then(|pane| pane.x_link)
                .unwrap_or_default();
            studio.draft_cursor_group = active_pane
                .as_ref()
                .and_then(|pane| pane.cursor_group)
                .unwrap_or_default();
        }
        VisualizationDock::PageEditor => {
            studio.draft_page_pane = active_pane.as_ref().map(|pane| pane.id);
            studio.draft_page = active_pane.map_or_else(String::new, |pane| pane.page);
            if let Some(policy) = studio.report_page_policies.get(&studio.draft_page) {
                studio.draft_report_template = policy.template.clone();
                studio.draft_report_freeze =
                    policy.update_policy == PageUpdatePolicy::FreezeFigureRevision;
            } else {
                studio.draft_report_template = REPORT_PAGE_TEMPLATES[0].to_owned();
                studio.draft_report_freeze = false;
            }
        }
        VisualizationDock::Measurement => {}
        VisualizationDock::FamilySlice => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
            if active_family_policy.is_none() {
                studio.family_query = studio.family_query.replacen(" and ", " · ", 1);
            }
        }
        VisualizationDock::FamilyFilter => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
        }
        VisualizationDock::FamilyEncoding => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
        }
        VisualizationDock::Comparison => {
            initialize_comparison_dock(studio, comparison_datasets, comparison_data_version);
        }
        VisualizationDock::CursorManager
        | VisualizationDock::Annotation
        | VisualizationDock::Export => {}
    }
    if dock != VisualizationDock::Comparison {
        studio.dock = Some(dock);
    }
}

fn initialize_family_draft(
    studio: &mut VisualizationStudioState,
    manifest: Option<&FamilyManifest>,
    policy: Option<&FamilyPresentationPolicy>,
) {
    let Some(manifest) = manifest else {
        studio.draft_family_x_dimension.clear();
        studio.draft_family_dimension.clear();
        studio.draft_family_color_dimension.clear();
        studio.draft_family_dash_dimension.clear();
        studio.draft_family_marker_dimension.clear();
        studio.family_query.clear();
        return;
    };

    let preferred_x = manifest
        .dimensions
        .iter()
        .find(|dimension| {
            dimension.kind == FamilyValueKind::Number
                && !matches!(dimension.id.as_str(), "temperature" | "sample")
        })
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                matches!(
                    dimension.kind,
                    FamilyValueKind::Number | FamilyValueKind::Integer
                ) && dimension.id != "temperature"
            })
        })
        .map(|dimension| dimension.id.clone())
        .unwrap_or_else(|| "sample".to_owned());
    studio.draft_family_x_dimension = policy
        .map(|policy| policy.x_dimension.dimension.key.clone())
        .unwrap_or(preferred_x);

    let preferred_family = manifest
        .dimension("process")
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                dimension.id != studio.draft_family_x_dimension
                    && !matches!(dimension.id.as_str(), "sample" | "status")
            })
        })
        .or_else(|| manifest.dimension("sample"))
        .map(|dimension| dimension.id.clone())
        .unwrap_or_default();
    studio.draft_family_dimension = policy
        .and_then(|policy| policy.family_dimensions.first())
        .map(|dimension| dimension.key.clone())
        .unwrap_or(preferred_family);

    let encoding_dimension = |predicate: fn(&FamilyEncodingMap) -> bool| {
        policy.and_then(|policy| {
            policy
                .encodings
                .iter()
                .find(|encoding| predicate(encoding))
                .map(|encoding| encoding.dimension().key.clone())
        })
    };
    studio.draft_family_color_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Color { .. }))
            .unwrap_or_else(|| studio.draft_family_dimension.clone());
    studio.draft_family_dash_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Dash { .. }))
            .or_else(|| {
                manifest
                    .dimension("temperature")
                    .map(|dimension| dimension.id.clone())
            })
            .unwrap_or_default();
    studio.draft_family_marker_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Marker { .. }))
            .unwrap_or_else(|| studio.draft_family_color_dimension.clone());
    studio.draft_family_exclude_missing = policy.is_some_and(|policy| {
        policy.missing_points == MissingPointPolicy::ExcludeWithOmissionRecord
    });
    studio.family_query = policy
        .and_then(|policy| policy.filter.as_ref())
        .map(|filter| filter.source.clone())
        .unwrap_or_else(|| {
            if manifest.dimension("temperature").is_some() {
                "temperature in {27,125} and status != not-run".to_owned()
            } else {
                "status != not-run".to_owned()
            }
        });
}

pub(crate) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    reconcile_document(app);
    synchronize_runtime_policies(app);
    let compact = ui.available_width() <= COMPACT_BREAKPOINT
        || app.state.workbench.coarse_pointer
        || ui.ctx().input(|input| input.has_touch_screen());
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);

    workspace_header(ui, app);
    status_strip(ui, app);
    if !compact {
        section_navigation(ui, app);
    }

    let dock_height = if compact { TOUCH_DOCK_HEIGHT } else { 0.0 };
    let body_height = (ui.available_height() - dock_height).max(1.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), body_height),
        Layout::top_down(Align::Min),
        |ui| {
            if compact
                && app.state.workbench.visualization_studio.touch_pane
                    == VisualizationTouchPane::Sections
            {
                compact_section_picker(ui, app);
                return;
            }
            show_active_section(ui, app, compact);
        },
    );
    if compact {
        touch_dock(ui, app);
    }
    show_dock_if_open(ui, app, compact);
}

fn reconcile_document(app: &mut RSpiceApp) {
    let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
    let active_analysis_sequence = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.id);
    let requested_viewer = app.state.ui.results.viewer;
    let (viewer, viewer_document_id) = requested_viewer.viewer_document_id().map_or_else(
        || (ResultViewer::Waves, "viewer-waveform".to_owned()),
        |document_id| (requested_viewer, document_id.to_owned()),
    );
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let projected = app
            .state
            .workspace
            .visualization_document(document_id)
            .map(|document| {
                let analysis_sequence_for =
                    |dataset_id: DatasetId, analysis_id: AnalysisInstanceId| {
                        app.state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)
                            .and_then(|run| retained_document_analysis(run, analysis_id))
                            .map(|analysis| analysis.id)
                    };
                let mut ordered_panes = document.panes().iter().collect::<Vec<_>>();
                let x_links = document
                    .link_groups()
                    .iter()
                    .filter(|group| group.kind == LinkKind::HorizontalViewport)
                    .flat_map(|group| {
                        group.members.iter().filter_map(move |member| {
                            let crate::results::visualization_document::EntityRef::Axis(axis_id) =
                                member
                            else {
                                return None;
                            };
                            document
                                .axes()
                                .iter()
                                .find(|axis| axis.id == *axis_id)
                                .map(|axis| (axis.pane_id.get(), group.id.get()))
                        })
                    })
                    .collect::<BTreeMap<_, _>>();
                ordered_panes.sort_by_key(|pane| {
                    (
                        document
                            .pages()
                            .iter()
                            .position(|page| page.id == pane.page_id)
                            .unwrap_or(usize::MAX),
                        pane.order,
                        pane.id,
                    )
                });
                let panes = ordered_panes
                    .into_iter()
                    .map(|pane| {
                        let viewer = ResultViewer::from_viewer_document_id(&pane.viewer_id)
                            .unwrap_or(ResultViewer::Waves);
                        let dataset_id = pane
                            .binding
                            .map(|binding| binding.dataset.dataset_id)
                            .or_else(|| {
                                document
                                    .datasets()
                                    .first()
                                    .map(|dataset| dataset.binding().dataset_id)
                            })
                            .unwrap_or_else(DatasetId::new);
                        let analysis_sequence = pane
                            .binding
                            .and_then(|binding| {
                                app.state
                                    .simulation
                                    .runs
                                    .iter()
                                    .find(|run| run.dataset_id == dataset_id)
                                    .and_then(|run| {
                                        retained_document_analysis(run, binding.analysis_id)
                                    })
                                    .map(|analysis| analysis.id)
                            })
                            .unwrap_or_default();
                        let page = document
                            .pages()
                            .iter()
                            .find(|page| page.id == pane.page_id)
                            .map_or_else(|| "Engineering".to_owned(), |page| page.title.clone());
                        let placement = match pane.placement {
                            crate::results::visualization_document::PanePlacement::RightOf {
                                ..
                            } => VisualizationPanePlacement::RightOfSelected,
                            crate::results::visualization_document::PanePlacement::Primary
                            | crate::results::visualization_document::PanePlacement::Below {
                                ..
                            } => VisualizationPanePlacement::BelowSelected,
                        };
                        VisualizationPane {
                            id: pane.id.get(),
                            viewer,
                            viewer_document_id: pane.viewer_id.clone(),
                            dataset_id,
                            analysis_sequence,
                            x_link: x_links.get(&pane.id.get()).copied(),
                            cursor_group: None,
                            page,
                            placement,
                        }
                    })
                    .collect::<Vec<_>>();
                let markers = document
                    .markers()
                    .iter()
                    .filter_map(|marker| {
                        let trace = document
                            .traces()
                            .iter()
                            .find(|trace| trace.id == marker.trace_id)?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == marker.pane_id)?;
                        let binding = pane.binding?;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence =
                            analysis_sequence_for(dataset_id, binding.analysis_id)?;
                        let TypedValue::Real(x) = &marker.coordinate else {
                            return None;
                        };
                        let x = *x;
                        let waveform = app
                            .state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)?
                            .analyses
                            .iter()
                            .find(|analysis| analysis.id == analysis_sequence)?
                            .waveforms
                            .iter()
                            .find(|waveform| waveform.name == trace.label)?;
                        let sample_index = waveform
                            .x
                            .iter()
                            .position(|candidate| candidate.to_bits() == x.to_bits())?;
                        let y = *waveform.y.get(sample_index)?;
                        Some(VisualizationMarker {
                            id: marker.id.get(),
                            dataset_id,
                            analysis_sequence,
                            waveform_name: trace.label.clone(),
                            sample_index,
                            x,
                            y,
                            label: marker.label.clone(),
                        })
                    })
                    .collect::<Vec<_>>();
                let measurements = document
                    .measurements()
                    .iter()
                    .filter_map(|measurement| {
                        let expression = measurement.expression.clone()?;
                        let value = measurement.value?;
                        let trace = measurement.trace_ids.first().and_then(|trace_id| {
                            document.traces().iter().find(|trace| trace.id == *trace_id)
                        })?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == measurement.pane_id)?;
                        let analysis_id = pane.binding?.analysis_id;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence = analysis_sequence_for(dataset_id, analysis_id)?;
                        Some(VisualizationMeasurement {
                            id: measurement.id.get(),
                            dataset_id,
                            analysis_sequence,
                            expression,
                            value,
                        })
                    })
                    .collect::<Vec<_>>();
                let annotations = document
                    .annotations()
                    .iter()
                    .filter_map(|annotation| {
                        let crate::results::visualization_document::AnnotationAnchor::Trace {
                            trace_id,
                            coordinate: TypedValue::Real(x),
                        } = &annotation.anchor
                        else {
                            return None;
                        };
                        let trace_id = *trace_id;
                        let x = *x;
                        let trace = document
                            .traces()
                            .iter()
                            .find(|trace| trace.id == trace_id)?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == annotation.pane_id)?;
                        let analysis_id = pane.binding?.analysis_id;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence = analysis_sequence_for(dataset_id, analysis_id)?;
                        Some(VisualizationAnnotation {
                            id: annotation.id.get(),
                            dataset_id,
                            analysis_sequence,
                            x,
                            text: annotation.text.clone(),
                        })
                    })
                    .collect::<Vec<_>>();
                let family_policies = document
                    .panes()
                    .iter()
                    .filter_map(|pane| {
                        pane.family_policy
                            .clone()
                            .map(|policy| (pane.id.get(), policy))
                    })
                    .collect::<BTreeMap<_, _>>();
                let report_page_policies = document
                    .pages()
                    .iter()
                    .map(|page| {
                        (
                            page.title.clone(),
                            VisualizationReportPagePolicy {
                                template: page.template_id.clone(),
                                update_policy: page.update_policy,
                                revision: document.revision().get(),
                            },
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                let cursor_pairs = document
                    .panes()
                    .iter()
                    .map(|pane| {
                        let mut pair = (None, None);
                        for cursor in document
                            .cursors()
                            .iter()
                            .filter(|cursor| cursor.pane_id == pane.id)
                        {
                            let TypedValue::Real(position) = &cursor.position else {
                                continue;
                            };
                            match cursor.label.as_str() {
                                "A" => pair.0 = Some(*position),
                                "B" => pair.1 = Some(*position),
                                _ => {}
                            }
                        }
                        (pane.id.get(), pair)
                    })
                    .collect::<BTreeMap<_, _>>();
                let cursors_linked = document
                    .link_groups()
                    .iter()
                    .any(|group| group.kind == LinkKind::CursorPosition);
                (
                    panes,
                    markers,
                    measurements,
                    annotations,
                    family_policies,
                    report_page_policies,
                    document.comparisons().to_vec(),
                    document.revision().get(),
                    document.presentation(),
                    cursor_pairs,
                    cursors_linked,
                )
            });
        if let Some((
            panes,
            markers,
            measurements,
            annotations,
            family_policies,
            report_page_policies,
            comparison_receipts,
            revision,
            presentation,
            cursor_pairs,
            cursors_linked,
        )) = projected
        {
            let studio = &mut app.state.workbench.visualization_studio;
            let previous_active = studio.active_pane;
            studio.panes = panes;
            studio.markers = markers;
            studio.measurements = measurements;
            studio.annotations = annotations;
            studio.family_policies = family_policies;
            studio.report_page_policies = report_page_policies;
            studio.comparison_receipts = comparison_receipts;
            studio.active_pane = previous_active
                .filter(|active| studio.panes.iter().any(|pane| pane.id == *active))
                .or_else(|| studio.panes.first().map(|pane| pane.id));
            studio.next_identity = studio
                .panes
                .iter()
                .map(|pane| pane.id)
                .max()
                .unwrap_or_default()
                .saturating_add(1);
            studio.revision = revision;
            studio.significant_digits = presentation.significant_digits;
            if studio.pane_cursor_positions != cursor_pairs {
                studio.pane_cursor_positions = cursor_pairs;
                studio.applied_link_pane = None;
            }
            studio.linked_cursor_positions.clear();
            app.state.ui.results.phase_continuous = presentation.phase_continuous;
            app.state.ui.results.linked_cursors = cursors_linked;
        }
        if let Some(active_pane_id) = app.state.workbench.visualization_studio.active_pane {
            let visibility = app
                .state
                .workspace
                .visualization_document(document_id)
                .map(|document| {
                    document
                        .traces()
                        .iter()
                        .filter(|trace| trace.pane_id.get() == active_pane_id)
                        .map(|trace| (trace.label.clone(), trace.visible))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let projection = app
                .state
                .workbench
                .visualization_studio
                .panes
                .iter()
                .find(|pane| pane.id == active_pane_id)
                .cloned()
                .and_then(|pane| {
                    let run = app
                        .state
                        .simulation
                        .runs
                        .iter()
                        .find(|run| run.dataset_id == pane.dataset_id)?;
                    let analysis = run
                        .analyses
                        .iter()
                        .find(|analysis| analysis.id == pane.analysis_sequence)?;
                    let analysis_key =
                        result_document::AnalysisPresentationKey::new(run.dataset_id, analysis);
                    let traces = analysis
                        .waveforms
                        .iter()
                        .filter_map(|waveform| {
                            visibility
                                .iter()
                                .find(|(label, _)| label == &waveform.name)
                                .map(|(_, visible)| {
                                    (waveform.name.clone(), waveform.visible, *visible)
                                })
                        })
                        .collect::<Vec<_>>();
                    (!traces.is_empty()).then_some((analysis_key, traces))
                });
            if let Some((analysis_key, traces)) = projection {
                app.state
                    .ui
                    .results
                    .project_waveform_visibility(analysis_key, traces);
            }
        }
    }
    let studio = &mut app.state.workbench.visualization_studio;
    studio.normalize();
    if studio.panes.is_empty()
        && let Some(dataset_id) = active_dataset
        && let Some(id) = studio.allocate_identity()
    {
        studio.panes.push(VisualizationPane {
            id,
            viewer,
            viewer_document_id: viewer_document_id.clone(),
            dataset_id,
            analysis_sequence: active_analysis_sequence.unwrap_or_default(),
            x_link: Some(1),
            cursor_group: Some(1),
            page: "Engineering".to_owned(),
            placement: VisualizationPanePlacement::BelowSelected,
        });
        studio.active_pane = Some(id);
        studio.selected_viewer_document = viewer_document_id;
    }
    if let Some(pane) = studio.active_pane_mut() {
        app.state.ui.results.viewer = pane.viewer;
    }
    normalize_fit_policy_for_renderer(&mut studio.autoscale, app.state.ui.results.viewer);
    let binding = studio
        .active_pane
        .and_then(|id| studio.panes.iter().find(|pane| pane.id == id))
        .map(|pane| (pane.dataset_id, pane.analysis_sequence));
    if let Some((dataset_id, _)) = binding
        && app
            .state
            .simulation
            .active_run()
            .is_none_or(|run| run.dataset_id != dataset_id)
        && let Some(run_index) = app
            .state
            .simulation
            .runs
            .iter()
            .position(|run| run.dataset_id == dataset_id)
    {
        let _ = app.state.simulation.select_run(run_index);
    }
    if let Some((dataset_id, analysis_sequence)) = binding
        && let Some(run) = app.state.simulation.active_run()
        && run.dataset_id == dataset_id
        && app
            .state
            .simulation
            .active_analysis()
            .is_none_or(|analysis| analysis.id != analysis_sequence)
        && let Some(analysis_index) = run
            .analyses
            .iter()
            .position(|analysis| analysis.id == analysis_sequence)
    {
        let _ = app.state.simulation.select_analysis(analysis_index);
    }
    apply_active_link_state(app);
}

fn apply_active_link_state(app: &mut RSpiceApp) {
    let Some(pane) = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
    else {
        return;
    };
    if app.state.workbench.visualization_studio.applied_link_pane == Some(pane.id) {
        return;
    }
    let x_range = pane.x_link.and_then(|group| {
        app.state
            .workbench
            .visualization_studio
            .linked_x_ranges
            .get(&group)
            .copied()
    });
    let x_range = x_range.or_else(|| {
        app.state
            .workbench
            .visualization_studio
            .pane_x_ranges
            .get(&pane.id)
            .copied()
    });
    let cursors = pane.cursor_group.and_then(|group| {
        app.state
            .workbench
            .visualization_studio
            .linked_cursor_positions
            .get(&group)
            .copied()
    });
    let cursors = cursors.or_else(|| {
        app.state
            .workbench
            .visualization_studio
            .pane_cursor_positions
            .get(&pane.id)
            .copied()
    });
    if let Some(x_range) = x_range {
        result_document::request_view_gesture(
            &mut app.state,
            result_document::ViewGesture::SetRanges {
                x: Some(x_range),
                y: None,
            },
        );
    }
    if let Some((a, b)) = cursors {
        app.state.ui.results.cursors.a = a;
        app.state.ui.results.cursors.b = b;
    } else {
        app.state.ui.results.cursors.clear();
    }
    app.state.workbench.visualization_studio.applied_link_pane = Some(pane.id);
}

fn capture_active_link_state(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(pane) = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
    else {
        return;
    };
    let x_range = result_document::active_renderer_axis_range(
        ctx,
        &mut app.state,
        result_document::PaneAxis::X,
    );
    let requested_cursors = (
        app.state.ui.results.cursors.a,
        app.state.ui.results.cursors.b,
    );
    commit_active_project_cursor_pair(app, pane.id, requested_cursors);
    let cursors = (
        app.state.ui.results.cursors.a,
        app.state.ui.results.cursors.b,
    );
    let studio = &mut app.state.workbench.visualization_studio;
    if let Some(x_range) = x_range {
        studio.pane_x_ranges.insert(pane.id, x_range);
        if let Some(group) = pane.x_link {
            studio.linked_x_ranges.insert(group, x_range);
        }
    } else {
        studio.pane_x_ranges.remove(&pane.id);
        if let Some(group) = pane.x_link {
            studio.linked_x_ranges.remove(&group);
        }
    }
    studio.pane_cursor_positions.insert(pane.id, cursors);
    if let Some(group) = pane.cursor_group {
        studio.linked_cursor_positions.insert(group, cursors);
    }
}

fn synchronize_runtime_policies(app: &mut RSpiceApp) {
    let studio = &app.state.workbench.visualization_studio;
    let complex_projection = match studio.complex_projection {
        ComplexProjection::MagnitudePhase => 0,
        ComplexProjection::RealImaginary => 1,
    };
    let display_lod = match studio.display_lod {
        DisplayLodPolicy::EnvelopePreserving => 0,
        DisplayLodPolicy::UniformSampling => 1,
        DisplayLodPolicy::ExactVisibleSamples => 2,
    };
    let significant_digits = u32::from(studio.significant_digits);
    let tile_memory_mib = studio.tile_memory_mib;

    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::ComplexNumberDisplay, complex_projection)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::LargeDatasetDisplay, display_lod)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    if let Err(error) = app.state.ui.preferences.set_scalar(
        ScalarPreference::DisplayedSignificantDigits,
        significant_digits,
    ) {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    app.state
        .ui
        .results
        .cache
        .set_memory_budget_mib(tile_memory_mib);
}

fn resolved_viewer_availability_for_binding(
    state: &AppState,
    definition: &ViewerDocumentDefinition,
    dataset_id: Option<DatasetId>,
    analysis_sequence: Option<u64>,
) -> Result<ResultViewer, String> {
    let dataset_id = dataset_id.ok_or_else(|| "Select a retained dataset".to_owned())?;
    let analysis_sequence =
        analysis_sequence.ok_or_else(|| "Select a retained analysis".to_owned())?;
    let run = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The selected dataset is no longer retained".to_owned())?;
    let analysis_index = run
        .analyses
        .iter()
        .position(|analysis| analysis.id == analysis_sequence)
        .ok_or_else(|| "The selected analysis is no longer retained".to_owned())?;
    let analysis = &run.analyses[analysis_index];
    let analysis_ids = [analysis_manifest_id(analysis.analysis_type)];
    match viewer_compatibility(
        definition.id,
        ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        },
    ) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(format!(
                "Requires {} analysis data",
                accepted_analysis_ids.join(" / ")
            ));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(format!("Requires {capability_id} result capability"));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err("Viewer identity is not registered".to_owned());
        }
    }
    let viewer = ResultViewer::from_viewer_document_id(definition.id)
        .ok_or_else(|| "No exact Rust renderer is registered for this viewer".to_owned())?;
    let viewer = result_document::project_viewer_for_analysis(viewer, analysis);
    let binding_is_active = state
        .simulation
        .active_run()
        .is_some_and(|active| active.dataset_id == dataset_id)
        && state
            .simulation
            .active_analysis()
            .is_some_and(|active| active.id == analysis_sequence);
    let available = match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => !analysis.waveforms.is_empty(),
        ResultViewer::Bode => result_document::bode_analysis_is_renderable(analysis),
        ResultViewer::Fft | ResultViewer::Eye => {
            crate::simulation::SimulationController::analysis_supports_transient_derivation(
                analysis.analysis_type,
            ) && !analysis.waveforms.is_empty()
        }
        ResultViewer::HarmonicBalance => {
            result_document::harmonic_balance_analysis_is_renderable(analysis)
        }
        ResultViewer::PhaseNoise => result_document::phase_noise_analysis_is_renderable(analysis),
        ResultViewer::Specs => {
            !analysis.measurements.is_empty() || !state.workspace.specs.is_empty()
        }
        // The table lists whatever samples the analysis retained.
        ResultViewer::Table => !analysis.waveforms.is_empty(),
        ResultViewer::PoleZero => retained_pole_zero_payload(analysis).is_some(),
        ResultViewer::Contribution => retained_sensitivity_payload(analysis).is_some(),
        ResultViewer::TransferFunction => analysis.result_payload.as_ref().is_some_and(|payload| {
            matches!(payload, AnalysisResultPayload::TransferFunction { .. })
                && payload.validate_for(analysis.analysis_type).is_ok()
        }),
        ResultViewer::Smith => result_document::smith_analysis_is_renderable(analysis),
        ResultViewer::Hist
        | ResultViewer::Op
        | ResultViewer::NoiseContrib
        | ResultViewer::Nyquist => {
            binding_is_active && result_document::viewer_is_available(state, viewer)
        }
        // Dataset-native Results projections, which therefore can never be
        // resolved from a Visualization Studio document definition.
        ResultViewer::Manifest
        | ResultViewer::Events
        | ResultViewer::Soa
        | ResultViewer::Reliability
        | ResultViewer::Optimization => false,
    };
    if !available {
        return Err(if binding_is_active {
            result_document::viewer_unavailability_reason(state, viewer)
                .unwrap_or("The selected analysis does not satisfy this renderer contract")
                .to_owned()
        } else {
            "This renderer requires derived state owned by the currently active analysis".to_owned()
        });
    }
    Ok(viewer)
}

fn active_studio_exact_export_available(state: &AppState) -> bool {
    let Some(pane) = state.workbench.visualization_studio.active_pane() else {
        return false;
    };
    let Some(run) = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == pane.dataset_id)
    else {
        return false;
    };
    if run.lifecycle != crate::state::SimulationRunLifecycle::Completed {
        return false;
    }
    let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == pane.analysis_sequence)
    else {
        return false;
    };
    if !analysis.success {
        return false;
    }
    pane.viewer
        .viewer_document_id()
        .and_then(viewer_document)
        .is_some_and(|definition| {
            resolved_viewer_availability_for_binding(
                state,
                definition,
                Some(pane.dataset_id),
                Some(pane.analysis_sequence),
            )
            .is_ok()
        })
}

fn active_studio_figure_export_available(state: &AppState) -> bool {
    active_studio_exact_export_available(state)
        && crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(state)
}

fn available_analysis_ids(state: &AppState) -> Vec<&'static str> {
    state
        .simulation
        .active_analysis()
        .map(|analysis| vec![analysis_manifest_id(analysis.analysis_type)])
        .unwrap_or_default()
}

const fn analysis_manifest_id(analysis: crate::state::AnalysisType) -> &'static str {
    use crate::state::AnalysisType;
    match analysis {
        AnalysisType::DcOp => "op",
        AnalysisType::DcSweep | AnalysisType::Parametric => "dc",
        AnalysisType::Ac => "ac",
        AnalysisType::Disto => "disto",
        AnalysisType::Transient => "tran",
        AnalysisType::Noise => "noise",
        AnalysisType::PoleZero => "pz",
        AnalysisType::Tf => "xf",
        AnalysisType::Sensitivity => "sens",
        AnalysisType::Pac => "pac",
        AnalysisType::Pnoise => "pnoise",
        AnalysisType::Pxf => "pxf",
        AnalysisType::Pstb => "pstb",
        AnalysisType::Stb => "stb",
        AnalysisType::MonteCarlo => "mc",
        AnalysisType::Corner => "corner",
        AnalysisType::Reliability => "reliability",
        AnalysisType::Optimization => "opt",
        AnalysisType::Soa => "soa",
        AnalysisType::SParameter => "sp",
        AnalysisType::Envelope => "envelope",
        AnalysisType::Fourier => "fourier",
        AnalysisType::HarmonicBalance => "hb",
        AnalysisType::Pss => "pss",
        AnalysisType::Qpss => "qpss",
        AnalysisType::Hbsp => "hbsp",
        AnalysisType::Hbnoise => "hbnoise",
        AnalysisType::Psp => "psp",
        AnalysisType::Qpac => "qpac",
        AnalysisType::Qpnoise => "qpnoise",
        AnalysisType::Qpxf => "qpxf",
        AnalysisType::TransientNoise => "tnoise",
        AnalysisType::DcMismatch => "dcmatch",
    }
}

fn add_viewer_pane(app: &mut RSpiceApp, document_id: &str, viewer: ResultViewer) {
    let Some(dataset_id) = app.state.simulation.active_run().map(|run| run.dataset_id) else {
        app.state.push_user_message(ConsoleMessage::warning(
            "A visualization pane requires an active immutable result dataset.",
        ));
        return;
    };
    let Some(analysis_sequence) = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.id)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "A visualization pane requires a selected retained analysis.",
        ));
        return;
    };
    add_viewer_pane_bound(
        app,
        document_id,
        viewer,
        dataset_id,
        analysis_sequence,
        VisualizationPanePlacement::BelowSelected,
        String::new(),
    );
}

fn add_viewer_pane_bound(
    app: &mut RSpiceApp,
    document_id: &str,
    viewer: ResultViewer,
    dataset_id: DatasetId,
    analysis_sequence: u64,
    placement: VisualizationPanePlacement,
    requested_page_title: String,
) {
    let binding = app
        .state
        .simulation
        .runs
        .iter()
        .enumerate()
        .find_map(|(run_index, run)| {
            (run.dataset_id == dataset_id).then(|| {
                run.analyses
                    .iter()
                    .position(|analysis| analysis.id == analysis_sequence)
                    .map(|analysis_index| (run_index, analysis_index))
            })?
        });
    let Some((run_index, analysis_index)) = binding else {
        app.state.push_user_message(ConsoleMessage::warning(
            "The selected immutable dataset or analysis is no longer retained.",
        ));
        return;
    };
    let Some(definition) = viewer_document(document_id) else {
        app.state.push_user_message(ConsoleMessage::error(
            "The selected visualization viewer is not registered.",
        ));
        return;
    };
    if let Err(error) = resolved_viewer_availability_for_binding(
        &app.state,
        definition,
        Some(dataset_id),
        Some(analysis_sequence),
    ) {
        app.state.push_user_message(ConsoleMessage::warning(error));
        return;
    }
    let active_pane = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned();
    let page = if placement == VisualizationPanePlacement::NewWorksheetPage {
        requested_page_title.trim().to_owned()
    } else {
        active_pane
            .as_ref()
            .map_or_else(|| "Engineering".to_owned(), |pane| pane.page.clone())
    };
    if page.is_empty() {
        app.state.push_user_message(ConsoleMessage::warning(
            "A new worksheet page requires a non-blank title.",
        ));
        return;
    }
    if active_project_visualization_document_id(&app.state).is_some() {
        let analysis_id = {
            let run = &app.state.simulation.runs[run_index];
            let analysis = &run.analyses[analysis_index];
            analysis.provenance().map_or_else(
                || {
                    let name = format!("legacy-analysis-v1/{}", analysis.id);
                    AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
                },
                |provenance| provenance.source_instance_id(),
            )
        };
        let dataset_binding = {
            let run = &app.state.simulation.runs[run_index];
            DatasetBinding::new(run.dataset_id, run.dataset_content_digest())
        };
        let binding = crate::results::visualization_document::PaneDataBinding {
            analysis_id,
            dataset: dataset_binding,
        };
        let existing_dataset = active_project_visualization_document_id(&app.state)
            .and_then(|document_id| app.state.workspace.visualization_document(document_id))
            .is_some_and(|document| {
                document
                    .datasets()
                    .iter()
                    .any(|dataset| dataset.binding() == dataset_binding)
            });
        let source = {
            let run = &app.state.simulation.runs[run_index];
            let analysis = &run.analyses[analysis_index];
            match result_document::visualization_source_dataset(run, analysis) {
                Ok(source) => source,
                Err(error) => {
                    app.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
            }
        };
        let mut edits = vec![if existing_dataset {
            DocumentEdit::MergeDatasetProjection(source)
        } else {
            DocumentEdit::AttachDataset(source)
        }];
        if placement == VisualizationPanePlacement::NewWorksheetPage {
            edits.push(DocumentEdit::AddPaneOnNewPage {
                page: crate::results::visualization_document::NewPage {
                    title: page,
                    layout: crate::results::visualization_document::PageLayout::Rows,
                    template_id: "engineering-dark".to_owned(),
                    update_policy: crate::results::visualization_document::PageUpdatePolicy::RefreshLinkedFigures,
                },
                pane: crate::results::visualization_document::NewPagePane {
                    title: definition.title.to_owned(),
                    kind: document_pane_kind(definition.art),
                    viewer_id: document_id.to_owned(),
                    binding: Some(binding),
                },
            });
        } else {
            let Some(active) = active_pane.as_ref() else {
                app.state.push_user_message(ConsoleMessage::warning(
                    "Select a result-document pane before inserting another pane.",
                ));
                return;
            };
            let anchor = app
                .state
                .workspace
                .visualization_document(
                    active_project_visualization_document_id(&app.state)
                        .expect("canonical branch has active document"),
                )
                .and_then(|document| {
                    document
                        .panes()
                        .iter()
                        .find(|pane| pane.id.get() == active.id)
                        .map(|pane| pane.id)
                });
            let Some(anchor) = anchor else {
                app.state.push_user_message(ConsoleMessage::error(
                    "The selected project result pane no longer exists.",
                ));
                return;
            };
            let placement = match placement {
                VisualizationPanePlacement::RightOfSelected => {
                    crate::results::visualization_document::PanePlacement::RightOf {
                        anchor_pane_id: anchor,
                    }
                }
                VisualizationPanePlacement::BelowSelected => {
                    crate::results::visualization_document::PanePlacement::Below {
                        anchor_pane_id: anchor,
                    }
                }
                VisualizationPanePlacement::NewWorksheetPage => unreachable!(),
            };
            let page_id = app
                .state
                .workspace
                .visualization_document(
                    active_project_visualization_document_id(&app.state)
                        .expect("canonical branch has active document"),
                )
                .and_then(|document| {
                    document
                        .panes()
                        .iter()
                        .find(|pane| pane.id.get() == active.id)
                        .map(|pane| pane.page_id)
                })
                .expect("resolved active pane owns a page");
            edits.push(DocumentEdit::AddBoundPane(
                crate::results::visualization_document::NewPane {
                    page_id,
                    title: definition.title.to_owned(),
                    kind: document_pane_kind(definition.art),
                    viewer_id: document_id.to_owned(),
                    binding: Some(binding),
                    placement,
                },
            ));
        }
        match transact_active_project_document(app, edits) {
            Ok(receipt) => {
                if let Some(pane_id) = receipt.created.iter().find_map(|entity| match entity {
                    crate::results::visualization_document::EntityRef::Pane(id) => Some(id.get()),
                    _ => None,
                }) {
                    app.state.workbench.visualization_studio.active_pane = Some(pane_id);
                }
                let _ = app.state.simulation.select_run(run_index);
                let _ = app.state.simulation.select_analysis(analysis_index);
                app.state.ui.results.viewer = viewer;
                reconcile_document(app);
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
        }
        return;
    }
    let x_link = if placement == VisualizationPanePlacement::NewWorksheetPage {
        None
    } else {
        active_pane.as_ref().and_then(|pane| pane.x_link)
    };
    let cursor_group = if placement == VisualizationPanePlacement::NewWorksheetPage {
        None
    } else {
        active_pane.as_ref().and_then(|pane| pane.cursor_group)
    };
    let studio = &mut app.state.workbench.visualization_studio;
    let document_id = document_id.to_owned();
    let result = studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization pane identity space is exhausted".to_owned())?;
        let insertion_index = if placement == VisualizationPanePlacement::NewWorksheetPage {
            studio.panes.len()
        } else {
            studio
                .active_pane
                .and_then(|active| studio.panes.iter().position(|pane| pane.id == active))
                .map_or(studio.panes.len(), |index| index + 1)
        };
        studio.panes.insert(
            insertion_index,
            VisualizationPane {
                id,
                viewer,
                viewer_document_id: document_id.clone(),
                dataset_id,
                analysis_sequence,
                x_link,
                cursor_group,
                page,
                placement,
            },
        );
        studio.active_pane = Some(id);
        studio.selected_viewer_document = document_id;
        studio.applied_link_pane = None;
        Ok(id)
    });
    match result {
        Ok(_) => {
            let _ = app.state.simulation.select_run(run_index);
            let _ = app.state.simulation.select_analysis(analysis_index);
            app.state.ui.results.viewer = viewer;
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
    }
}

#[cfg(test)]
mod integrity_scan_tests {
    use super::dock::{
        commit_comparison_execution, evaluate_scalar_measurement, execute_comparison_draft,
        execute_comparison_draft_with_differences, retain_difference_trace_sets,
        save_document_properties,
    };
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecEntry, WaveformData};
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, WavePanePresentationKey,
    };

    fn app_with_exact_source() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let analysis =
            AnalysisResult::new(17, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![-1.25, 2.5, 4.0],
                    "#00aaff",
                ),
                WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
            ]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        app
    }

    fn activate_voltage_wave_pane(app: &mut RSpiceApp) -> AnalysisPresentationKey {
        let run = app.state.simulation.active_run().expect("active run");
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .expect("active analysis");
        let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        app.state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
            analysis: key,
            unit: "V".to_owned(),
        });
        key
    }

    fn apply_queued_view_gesture(ctx: &egui::Context, app: &mut RSpiceApp) {
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                result_document::apply_pending_view_gesture(ui, &mut app.state);
            });
        });
    }

    fn activate_project_visualization_document(app: &mut RSpiceApp) -> ResultDocumentId {
        let (source, analysis_id) = {
            let run = app.state.simulation.active_run().expect("active run");
            let analysis = app
                .state
                .simulation
                .active_analysis()
                .expect("active analysis");
            let source = result_document::visualization_source_dataset(run, analysis)
                .expect("retained source projects");
            let analysis_id = analysis.provenance().map_or_else(
                || {
                    let name = format!("legacy-analysis-v1/{}", analysis.id);
                    AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
                },
                |provenance| provenance.source_instance_id(),
            );
            (source, analysis_id)
        };
        let binding = crate::results::visualization_document::PaneDataBinding {
            analysis_id,
            dataset: source.binding(),
        };
        let mut document = crate::results::visualization_document::VisualizationDocument::new(
            "Engineering review",
            vec![source],
        )
        .expect("visualization document");
        let pane_id = document.panes()[0].id;
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(binding),
                }],
            )
            .expect("pane binding commits");
        let document_id = app
            .state
            .workspace
            .insert_visualization_document(document)
            .expect("document inserted");
        app.state.workbench.activate(Workspace::Results);
        assert!(
            crate::workbench::chrome::document_bar::activate_document_by_id(
                &mut app.state,
                &WorkspaceDocumentId::VisualizationDocument(document_id),
            )
        );
        document_id
    }

    #[test]
    fn document_properties_commit_to_the_active_project_document() {
        let mut app = app_with_exact_source();
        let document_id = activate_project_visualization_document(&mut app);
        let before = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document")
            .revision();

        save_document_properties(&mut app, 13, true).expect("properties commit");

        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert!(document.revision() > before);
        assert_eq!(document.presentation().significant_digits, 13);
        assert!(document.presentation().phase_continuous);
        assert!(app.state.workspace.visualization_documents_dirty);
    }

    #[test]
    fn studio_export_requires_the_selected_successful_completed_binding() {
        let mut app = app_with_exact_source();
        activate_project_visualization_document(&mut app);
        reconcile_document(&mut app);
        assert!(!active_studio_exact_export_available(&app.state));

        app.state.simulation.runs[0].lifecycle = crate::state::SimulationRunLifecycle::Completed;
        assert!(active_studio_exact_export_available(&app.state));
        assert!(active_studio_figure_export_available(&app.state));

        app.state.simulation.runs[0].analyses[0].success = false;
        assert!(!active_studio_exact_export_available(&app.state));
        assert!(!active_studio_figure_export_available(&app.state));
    }

    #[test]
    fn canonical_result_entities_commit_and_project_without_parallel_authority() {
        let mut app = app_with_exact_source();
        let document_id = activate_project_visualization_document(&mut app);
        reconcile_document(&mut app);

        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert_eq!(document.traces().len(), 2);
        assert!(document.traces().iter().all(|trace| {
            trace.coordinate_key == "x"
                && trace.signal_key == "y"
                && trace.row_predicates.len() == 4
        }));

        add_marker_at_midpoint(&mut app);
        let (pane_id, trace_id) =
            active_project_pane_and_trace(&app.state, Some("V(out)")).expect("bound trace");
        transact_active_project_document(
            &mut app,
            vec![DocumentEdit::AddScalarMeasurement {
                pane_id,
                trace_ids: vec![trace_id],
                expression: "rms(V(out))".to_owned(),
                value: 2.75,
            }],
        )
        .expect("measurement commits");
        transact_active_project_document(
            &mut app,
            vec![DocumentEdit::AddAnnotation {
                pane_id,
                anchor: crate::results::visualization_document::AnnotationAnchor::Trace {
                    trace_id,
                    coordinate: TypedValue::Real(0.5),
                },
                text: "Review this exact source point".to_owned(),
            }],
        )
        .expect("annotation commits");
        reconcile_document(&mut app);

        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert_eq!(document.markers().len(), 1);
        assert_eq!(document.measurements().len(), 1);
        assert_eq!(document.annotations().len(), 1);
        assert_eq!(app.state.workbench.visualization_studio.markers.len(), 1);
        assert_eq!(app.state.workbench.visualization_studio.markers[0].x, 0.5);
        assert_eq!(
            app.state.workbench.visualization_studio.measurements[0].expression,
            "rms(V(out))"
        );
        assert_eq!(
            app.state.workbench.visualization_studio.annotations[0].text,
            "Review this exact source point"
        );

        transact_active_project_document(
            &mut app,
            vec![DocumentEdit::ClearMarkers { pane_id: None }],
        )
        .expect("marker clear commits atomically");
        reconcile_document(&mut app);
        assert!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("active document")
                .markers()
                .is_empty()
        );
        assert!(app.state.workbench.visualization_studio.markers.is_empty());
    }

    #[test]
    fn canonical_ab_cursors_persist_link_move_and_clear_as_document_entities() {
        let mut app = app_with_exact_source();
        let document_id = activate_project_visualization_document(&mut app);
        reconcile_document(&mut app);
        let first_pane = app
            .state
            .workbench
            .visualization_studio
            .active_pane
            .expect("project pane");

        add_cursor_at_midpoint(&mut app);
        add_cursor_at_midpoint(&mut app);
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert_eq!(document.cursors().len(), 2);
        assert_eq!(
            canonical_cursor_pair(document, document.panes()[0].id).unwrap(),
            (Some(0.5), Some(0.5))
        );

        assert!(commit_active_project_cursor_pair(
            &mut app,
            first_pane,
            (None, None)
        ));
        assert!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("active document")
                .cursors()
                .is_empty()
        );

        let (page_id, anchor_pane, binding) = {
            let document = app
                .state
                .workspace
                .visualization_document(document_id)
                .expect("active document");
            let pane = &document.panes()[0];
            (pane.page_id, pane.id, pane.binding)
        };
        let receipt = transact_active_project_document(
            &mut app,
            vec![DocumentEdit::AddBoundPane(
                crate::results::visualization_document::NewPane {
                    page_id,
                    title: "Linked waveform".to_owned(),
                    kind: crate::results::visualization_document::PaneKind::Cartesian,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding,
                    placement: crate::results::visualization_document::PanePlacement::Below {
                        anchor_pane_id: anchor_pane,
                    },
                },
            )],
        )
        .expect("second pane commits");
        let second_pane = receipt
            .created
            .iter()
            .find_map(|entity| match entity {
                EntityRef::Pane(pane_id) => Some(*pane_id),
                _ => None,
            })
            .expect("second pane id");
        let cursor_edits = {
            let document = app
                .state
                .workspace
                .visualization_document(document_id)
                .expect("active document");
            [anchor_pane, second_pane]
                .into_iter()
                .flat_map(|pane_id| {
                    let axis_id = document
                        .axes()
                        .iter()
                        .find(|axis| {
                            axis.pane_id == pane_id
                                && axis.orientation == AxisOrientation::Horizontal
                        })
                        .expect("horizontal axis")
                        .id;
                    [("A", 0.1), ("B", 0.2)].map(|(label, position)| DocumentEdit::AddCursor {
                        pane_id,
                        axis_id,
                        position: TypedValue::Real(position),
                        label: label.to_owned(),
                    })
                })
                .collect::<Vec<_>>()
        };
        transact_active_project_document(&mut app, cursor_edits).expect("cursors commit");
        reconcile_document(&mut app);
        assert!(set_active_project_cursor_links(&mut app, true));

        assert!(commit_active_project_cursor_pair(
            &mut app,
            first_pane,
            (Some(0.25), Some(0.75))
        ));
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert_eq!(document.link_groups().len(), 2);
        assert!(document.cursors().iter().all(|cursor| {
            let expected = if cursor.label == "A" { 0.25 } else { 0.75 };
            real_cursor_position(cursor)
                .is_some_and(|position| same_cursor_position(position, expected))
        }));

        assert!(commit_active_project_cursor_pair(
            &mut app,
            first_pane,
            (None, None)
        ));
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        assert!(document.cursors().is_empty());
        assert!(document.link_groups().is_empty());
    }

    #[test]
    fn restored_exact_extrema_policy_normalizes_for_non_wave_renderer() {
        let mut app = app_with_exact_source();
        app.state.ui.results.viewer = ResultViewer::Bode;
        app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::ExactExtrema;

        reconcile_document(&mut app);

        assert_eq!(app.state.ui.results.viewer, ResultViewer::Bode);
        assert_eq!(
            app.state.workbench.visualization_studio.autoscale,
            VisualizationAutoscale::RobustVisible
        );
        assert_eq!(fit_block_reason(&app.state), None);
    }

    #[test]
    fn fit_contract_reports_exact_source_and_specification_blockers() {
        let mut app = app_with_exact_source();
        app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::ExactExtrema;
        assert_eq!(fit_block_reason(&app.state), None);

        for waveform in &mut app.state.simulation.runs[0].analyses[0].waveforms {
            waveform.visible = false;
        }
        assert_eq!(
            fit_block_reason(&app.state),
            Some(
                "Exact-extrema fitting requires at least one visible waveform with finite samples."
            )
        );

        app.state.workbench.visualization_studio.autoscale =
            VisualizationAutoscale::SpecificationBounds;
        assert_eq!(
            fit_block_reason(&app.state),
            Some(
                "Specification-bound fitting requires a visible waveform whose exact quantity name matches a configured project specification."
            )
        );

        app.state.ui.results.viewer = ResultViewer::Smith;
        assert_eq!(
            fit_block_reason(&app.state),
            Some("Specification-bound fitting is available only for the waveform renderer.")
        );
    }

    #[test]
    fn robust_fit_is_available_for_non_wave_renderers() {
        let mut app = app_with_exact_source();
        app.state.ui.results.viewer = ResultViewer::Bode;
        app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::RobustVisible;
        app.state.workbench.visualization_studio.zoom = 2.5;

        assert_eq!(fit_block_reason(&app.state), None);
        fit_active_view(&mut app);

        assert_eq!(app.state.workbench.visualization_studio.zoom, 1.0);
    }

    #[test]
    fn specification_bound_fit_requires_an_exact_visible_quantity_binding() {
        let mut app = app_with_exact_source();
        app.state.workspace.specs.push(SpecEntry {
            measurement: "v(OUT)".to_owned(),
            expression: String::new(),
            min: Some(-2.0),
            max: Some(5.0),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });

        assert_eq!(
            specification_bound_fit(&app.state),
            Some(((0.0, 20.0), (-2.0, 5.0)))
        );

        app.state.workspace.specs[0].measurement = "V(unrelated)".to_owned();
        assert_eq!(specification_bound_fit(&app.state), None);
    }

    #[test]
    fn specification_bound_autoscale_commits_the_exact_data_and_limit_envelope() {
        let mut app = app_with_exact_source();
        let analysis_key = activate_voltage_wave_pane(&mut app);
        app.state.workspace.specs.push(SpecEntry {
            measurement: "V(out)".to_owned(),
            expression: String::new(),
            min: Some(-2.0),
            max: Some(5.0),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });
        app.state.workbench.visualization_studio.autoscale =
            VisualizationAutoscale::SpecificationBounds;
        app.state.workbench.visualization_studio.zoom = 3.0;

        fit_active_view(&mut app);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        apply_queued_view_gesture(&ctx, &mut app);

        let view =
            app.state
                .ui
                .results
                .analysis_plot_view_pane(ResultViewer::Waves, analysis_key, 0);
        assert_eq!(view.x, Some((0.0, 20.0)));
        assert_eq!(view.y, Some((-2.0, 5.0)));
        assert_eq!(app.state.workbench.visualization_studio.zoom, 1.0);
    }

    #[test]
    fn transfer_function_result_resolves_the_canonical_xf_viewer_contract() {
        let analysis_ids = [analysis_manifest_id(AnalysisType::Tf)];
        assert_eq!(analysis_ids, ["xf"]);
        assert_eq!(
            viewer_compatibility(
                "viewer-transfer-function",
                ViewerCapabilities {
                    analysis_ids: &analysis_ids,
                    external_capabilities: &[],
                },
            ),
            ViewerCompatibility::Compatible
        );
    }

    #[test]
    fn comparison_source_projection_preserves_dc_and_hb_specialist_modes() {
        let dc = AnalysisResult::new(1, AnalysisType::DcSweep, "DC");
        assert_eq!(
            result_document::project_viewer_for_analysis(ResultViewer::Waves, &dc),
            ResultViewer::DcSweep
        );

        let hb = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
            WaveformData::new(
                "|V(out) Spectrum|",
                vec![1.0e9, 2.0e9],
                vec![1.0, 0.1],
                "#00aaff",
            )
            .with_complex_components(
                "V(out) Spectrum",
                vec![1.0, 0.1],
                vec![0.0, 0.02],
            ),
        ]);
        assert_eq!(
            result_document::project_viewer_for_analysis(ResultViewer::Fft, &hb),
            ResultViewer::HarmonicBalance
        );

        let transient = AnalysisResult::new(3, AnalysisType::Transient, "TRAN");
        assert_eq!(
            result_document::project_viewer_for_analysis(ResultViewer::Waves, &transient),
            ResultViewer::Waves
        );
    }

    fn exact_source_checksum(app: &RSpiceApp) -> u64 {
        let mut checksum = 0xcbf2_9ce4_8422_2325_u64;
        for (&x, &y) in app
            .state
            .simulation
            .active_analysis()
            .expect("test analysis must remain selected")
            .waveforms
            .iter()
            .flat_map(|waveform| waveform.x.iter().zip(waveform.y.iter()))
        {
            checksum ^= x.to_bits();
            checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
            checksum ^= y.to_bits();
            checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
        }
        checksum
    }

    #[test]
    fn processes_every_exact_sample_and_completes_with_expected_checksum() {
        let mut app = app_with_exact_source();
        let expected_checksum = exact_source_checksum(&app);
        let expected_binding = source_integrity_scan_binding(&app.state)
            .expect("exact retained samples must produce a scan binding");

        start_source_integrity_scan(&mut app);

        let studio = &app.state.workbench.visualization_studio;
        assert_eq!(studio.operation_state, OperationState::Running);
        assert_eq!(studio.operation_dataset_id, Some(expected_binding.0));
        assert_eq!(studio.operation_analysis_sequence, Some(17));
        assert_eq!(studio.operation_processed, 0);
        assert_eq!(studio.operation_total, 5);

        advance_source_integrity_scan(&mut app).expect("first exact chunk must scan");
        assert_eq!(
            app.state.workbench.visualization_studio.operation_processed,
            2
        );
        advance_source_integrity_scan(&mut app).expect("second exact chunk must scan");
        assert_eq!(
            app.state.workbench.visualization_studio.operation_processed,
            4
        );
        advance_source_integrity_scan(&mut app).expect("final exact chunk must scan");

        let studio = &app.state.workbench.visualization_studio;
        assert_eq!(studio.operation_state, OperationState::Completed);
        assert_eq!(studio.operation_processed, studio.operation_total);
        assert_eq!(studio.operation_checksum, expected_checksum);
        assert_eq!(
            advance_source_integrity_scan(&mut app),
            Err("The source-integrity scan is already complete".to_owned())
        );
    }

    #[test]
    fn recovery_fails_closed_when_immutable_source_binding_disappears() {
        let mut app = app_with_exact_source();
        start_source_integrity_scan(&mut app);
        advance_source_integrity_scan(&mut app).expect("initial exact chunk must scan");
        app.state.workbench.visualization_studio.operation_state = OperationState::Cancelled;

        app.state.simulation.runs.clear();

        assert_eq!(
            recover_source_integrity_scan(&mut app),
            Err("The cancelled integrity scan's immutable source is unavailable".to_owned())
        );
        assert_eq!(
            app.state.workbench.visualization_studio.operation_state,
            OperationState::Cancelled
        );
    }

    #[test]
    fn scalar_measurements_validate_against_the_exact_active_analysis() {
        let app = app_with_exact_source();
        let (dataset_id, analysis_sequence, value) =
            evaluate_scalar_measurement(&app.state, "rms(V(out))")
                .expect("RMS must produce a finite scalar measurement");
        assert_eq!(
            Some(dataset_id),
            app.state.simulation.active_run().map(|run| run.dataset_id)
        );
        assert_eq!(analysis_sequence, 17);
        assert!((value - (7.9375_f64).sqrt()).abs() < 1.0e-14);
        assert_eq!(
            evaluate_scalar_measurement(&app.state, "V(out)").unwrap_err(),
            "The expression produces a trace; reduce it with avg(), rms(), or another scalar function"
        );
    }

    #[test]
    fn explicit_comparison_executes_exact_contract_without_mutating_sources() {
        let mut app = app_with_exact_source();
        let mut baseline = SimulationRun::new(2);
        baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![-1.20, 2.45, 3.95],
                    "#00aaff",
                ),
                WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
            ]),
        );
        let baseline_id = baseline.dataset_id;
        app.state.simulation.runs.push(baseline);
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_dataset = Some(baseline_id);
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_absolute_tolerance = 0.1;
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;
        let candidate_digest = app
            .state
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest();

        let execution = execute_comparison_draft_with_differences(&app)
            .expect("exact comparison and checked difference traces must execute");
        let receipt = execution.receipt.clone();
        let result =
            app.state.workbench.visualization_studio.transact(|studio| {
                retain_difference_trace_sets(studio, execution.difference_traces)
            });
        result.expect("derived series identities must commit atomically");

        assert_eq!(receipt.rows_compared, 3);
        assert!(matches!(
            receipt.policy.execution.alignment,
            ComparisonAlignmentMethod::AbsoluteXAxis
        ));
        let trace_set = &app
            .state
            .workbench
            .visualization_studio
            .difference_trace_sets[0];
        assert_eq!(trace_set.coordinates, vec![0.0, 0.5, 1.0]);
        assert!(
            trace_set
                .absolute
                .values
                .iter()
                .all(|value| (*value - 0.05).abs() <= 1.0e-12)
        );
        assert!(
            trace_set
                .normalized
                .values
                .iter()
                .all(|value| (*value - 0.5).abs() <= 1.0e-12)
        );
        assert_eq!(
            HashSet::from([
                trace_set.id,
                trace_set.absolute.id,
                trace_set.relative.id,
                trace_set.normalized.id,
            ])
            .len(),
            4
        );
        assert_eq!(
            receipt.disposition,
            crate::results::visualization_document::ComparisonDisposition::Passed
        );
        assert_eq!(
            app.state
                .simulation
                .active_run()
                .unwrap()
                .dataset_content_digest(),
            candidate_digest
        );
    }

    #[test]
    fn project_document_owns_comparison_receipts_and_studio_only_projects_them() {
        let mut app = app_with_exact_source();
        let document_id = activate_project_visualization_document(&mut app);
        reconcile_document(&mut app);
        let mut baseline = SimulationRun::new(2);
        baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![-1.20, 2.45, 3.95],
                    "#00aaff",
                ),
                WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
            ]),
        );
        let baseline_id = baseline.dataset_id;
        app.state.simulation.runs.push(baseline);
        let studio = &mut app.state.workbench.visualization_studio;
        studio.draft_comparison_dataset = Some(baseline_id);
        studio.draft_comparison_absolute_tolerance = 0.1;
        studio.draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;

        let execution = execute_comparison_draft_with_differences(&app).unwrap();
        let expected_receipt = execution.receipt.clone();
        commit_comparison_execution(&mut app, execution).unwrap();

        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .unwrap();
        assert_eq!(document.comparisons(), &[expected_receipt.clone()]);
        assert_eq!(document.datasets().len(), 2);
        assert_eq!(
            app.state.workbench.visualization_studio.comparison_receipts,
            vec![expected_receipt]
        );
        assert!(
            !app.state
                .workbench
                .visualization_studio
                .difference_trace_sets
                .is_empty()
        );
    }

    #[test]
    fn comparison_records_threshold_and_cross_correlation_alignment_parameters() {
        let mut threshold_app = app_with_exact_source();
        threshold_app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![-1.0, 1.0, 3.0],
            "#00aaff",
        )];
        let mut threshold_baseline = SimulationRun::new(2);
        threshold_baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![10.0, 11.0, 12.0],
                    vec![-2.0, 2.0, 4.0],
                    "#00aaff",
                ),
            ]),
        );
        let threshold_baseline_id = threshold_baseline.dataset_id;
        threshold_app.state.simulation.runs.push(threshold_baseline);
        let threshold_studio = &mut threshold_app.state.workbench.visualization_studio;
        threshold_studio.draft_comparison_dataset = Some(threshold_baseline_id);
        threshold_studio.draft_comparison_alignment =
            ComparisonAlignmentDraft::FirstThresholdCrossing;
        threshold_studio.draft_comparison_alignment_signal = "V(out)".to_owned();
        threshold_studio.draft_comparison_threshold = 0.0;
        threshold_studio.draft_comparison_difference_trace = false;

        let threshold_receipt =
            execute_comparison_draft(&threshold_app).expect("threshold alignment must execute");
        assert!(matches!(
            threshold_receipt.policy.execution.alignment,
            ComparisonAlignmentMethod::FirstThresholdCrossing {
                signal_key,
                threshold: 0.0,
                baseline_crossing: 10.5,
                candidate_crossing: 0.5,
            } if signal_key == "signal:0"
        ));
        assert_eq!(
            threshold_receipt.policy.execution.resampling,
            ComparisonResamplingPolicy::BaselineOntoCandidateGrid
        );

        let mut correlation_app = app_with_exact_source();
        correlation_app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            vec![0.0, 0.0, 1.0, 0.0, -1.0, 0.0],
            "#00aaff",
        )];
        let mut correlation_baseline = SimulationRun::new(2);
        correlation_baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                    vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );
        let correlation_baseline_id = correlation_baseline.dataset_id;
        correlation_app
            .state
            .simulation
            .runs
            .push(correlation_baseline);
        let correlation_studio = &mut correlation_app.state.workbench.visualization_studio;
        correlation_studio.draft_comparison_dataset = Some(correlation_baseline_id);
        correlation_studio.draft_comparison_alignment = ComparisonAlignmentDraft::CrossCorrelation;
        correlation_studio.draft_comparison_alignment_signal = "V(out)".to_owned();
        correlation_studio.draft_comparison_maximum_lag_samples = 2;
        correlation_studio.draft_comparison_difference_trace = false;

        let correlation_receipt =
            execute_comparison_draft(&correlation_app).expect("correlation alignment must execute");
        assert!(matches!(
            correlation_receipt.policy.execution.alignment,
            ComparisonAlignmentMethod::CrossCorrelation {
                selected_lag_samples: 1,
                sample_interval: 1.0,
                baseline_shift: 1.0,
                ..
            }
        ));
    }

    #[test]
    fn comparison_fails_closed_for_nonmonotonic_source_coordinates() {
        let mut app = app_with_exact_source();
        app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0],
            "#00aaff",
        )];
        let mut baseline = SimulationRun::new(2);
        baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 0.5],
                    vec![0.0, 1.0, 2.0],
                    "#00aaff",
                ),
            ]),
        );
        let baseline_id = baseline.dataset_id;
        app.state.simulation.runs.push(baseline);
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_dataset = Some(baseline_id);
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;

        let error = execute_comparison_draft(&app)
            .expect_err("nonmonotonic immutable data must never be resampled");
        assert!(error.contains("nonmonotonic"));
    }

    #[test]
    fn results_comparison_handoff_rebinds_a_stale_owner_to_the_active_document() {
        let mut app = app_with_exact_source();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Results;
        let candidate_id = app.state.simulation.runs[0].dataset_id;
        app.state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::ResultDataset(candidate_id));

        let mut baseline = SimulationRun::new(2);
        baseline.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![-1.20, 2.45, 3.95],
                    "#00aaff",
                ),
            ]),
        );
        let baseline_id = baseline.dataset_id;
        app.state.simulation.runs.push(baseline);

        app.state.workbench.visualization_studio.panes = vec![VisualizationPane {
            id: 1,
            viewer: ResultViewer::Waves,
            viewer_document_id: ResultViewer::Waves
                .viewer_document_id()
                .expect("waveform viewer has a catalog document")
                .to_owned(),
            dataset_id: baseline_id,
            analysis_sequence: 29,
            x_link: None,
            cursor_group: None,
            page: "Engineering".to_owned(),
            placement: VisualizationPanePlacement::BelowSelected,
        }];
        app.state.workbench.visualization_studio.active_pane = Some(1);
        app.state.workbench.visualization_studio.next_identity = 2;

        open_results_comparison_inner(&mut app)
            .expect("a compatible retained baseline must open the real comparison owner");

        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::VisualizationStudio
        );
        assert_eq!(
            app.state.simulation.active_run().map(|run| run.dataset_id),
            Some(candidate_id)
        );
        assert_eq!(
            app.state
                .simulation
                .active_analysis()
                .map(|analysis| analysis.id),
            Some(17)
        );
        let owner = app
            .state
            .workbench
            .visualization_studio
            .active_pane()
            .expect("comparison handoff must activate an exact owner pane");
        assert_eq!(owner.dataset_id, candidate_id);
        assert_eq!(owner.analysis_sequence, 17);
        assert_eq!(
            app.state
                .workbench
                .visualization_studio
                .draft_comparison_dataset,
            Some(baseline_id)
        );
        assert_eq!(
            app.state.workbench.visualization_studio.dock,
            Some(VisualizationDock::Comparison)
        );
    }

    #[test]
    fn results_comparison_fails_closed_before_navigation_without_a_compatible_baseline() {
        let mut app = app_with_exact_source();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Results;
        let candidate_id = app.state.simulation.runs[0].dataset_id;
        app.state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::ResultDataset(candidate_id));
        let route_before = app.state.workbench.current_route();

        let mut incompatible = SimulationRun::new(2);
        incompatible.add_analysis(
            AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(other)",
                    vec![0.0, 0.25, 1.0],
                    vec![0.0, 1.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );
        app.state.simulation.runs.push(incompatible);

        assert!(!results_comparison_available(&app.state));
        let error = open_results_comparison_inner(&mut app)
            .expect_err("an incompatible retained run must not open a comparison owner");
        assert!(error.contains("second compatible immutable dataset"));
        assert_eq!(app.state.workbench.current_route(), route_before);
        assert_eq!(app.state.workbench.visualization_studio.dock, None);
    }

    /// What "feature complete" means for the Results workspace, stated as a
    /// gate rather than a claim: a sheet answers for every view the product
    /// manifest puts in this release's scope. Publish a release-target row with
    /// no sheet behind it and this fails.
    #[test]
    fn every_release_target_viewer_document_ships_a_renderer() {
        use crate::results::viewer_catalog::ViewerReleaseClass;

        let mut in_scope = 0;
        for definition in VIEWER_DOCUMENTS {
            if definition.release != ViewerReleaseClass::ReleaseTarget {
                continue;
            }
            in_scope += 1;
            assert!(
                ResultViewer::from_viewer_document_id(definition.id).is_some(),
                "{} is in this release's scope but no sheet draws it",
                definition.id
            );
        }
        assert!(
            in_scope > 0,
            "the catalog declares no view in release scope"
        );
    }

    /// The viewer library publishes every designed view, and most of them this
    /// build does not draw. The row a reader cannot click has to say which kind
    /// of unavailable it is: "requires X analysis data" is an instruction they
    /// can act on, so a view no sheet draws must not borrow that phrasing — it
    /// reports the manifest's own release scope instead.
    #[test]
    fn a_view_no_sheet_draws_reports_its_release_scope_not_a_data_requirement() {
        use sections::resolved_viewer_availability;

        let app = app_with_exact_source();
        let analysis_ids = available_analysis_ids(&app.state);
        let mut drawable = 0;
        for definition in VIEWER_DOCUMENTS {
            let availability = resolved_viewer_availability(
                &app.state,
                definition,
                ViewerCapabilities {
                    analysis_ids: &analysis_ids,
                    external_capabilities: &[],
                },
            );
            let ships_a_renderer = ResultViewer::from_viewer_document_id(definition.id).is_some()
                && definition.release
                    == crate::results::viewer_catalog::ViewerReleaseClass::ReleaseTarget;
            drawable += usize::from(ships_a_renderer);
            match availability {
                Ok(_) => assert!(ships_a_renderer, "{} drew without a sheet", definition.id),
                Err(reason) => assert_eq!(
                    reason == definition.release.unavailable_reason(),
                    !ships_a_renderer,
                    "{} reports the wrong kind of unavailable: {reason}",
                    definition.id
                ),
            }
        }
        assert!(
            drawable > 0 && drawable < VIEWER_DOCUMENTS.len(),
            "the catalog and the shipped renderers have stopped disagreeing; \
             fold this expectation into whichever is now the whole truth"
        );
    }

    /// Three sheets render `viewer-table`, so a retained pane's sheet cannot be
    /// recovered from its viewer document — only checked against it. Reading
    /// that check backwards would reject every retained Specs and OP pane as
    /// having no renderer, because the inverse can only name one of the three.
    #[test]
    fn a_retained_pane_sharing_a_viewer_document_still_validates() {
        let mut app = app_with_exact_source();
        app.state.simulation.runs[0].analyses[0].measurements =
            vec![rspice_core::MeasureResult::success("V(out)", 4.0)];
        reconcile_document(&mut app);
        let panes = &mut app.state.workbench.visualization_studio.panes;
        let pane = panes
            .first_mut()
            .expect("the reconciled document has a pane");
        pane.viewer = ResultViewer::Specs;
        pane.viewer_document_id = ResultViewer::Specs
            .viewer_document_id()
            .expect("Specs renders a catalog viewer document")
            .to_owned();
        assert_eq!(pane.viewer_document_id, "viewer-table");
        assert_ne!(
            ResultViewer::from_viewer_document_id("viewer-table"),
            Some(ResultViewer::Specs),
            "this test is only meaningful while the inverse names a different sheet"
        );

        visualization_configuration_status(&app.state)
            .expect("a retained Specs pane names viewer-table truthfully");
    }

    #[test]
    fn configuration_status_fails_closed_when_a_retained_binding_disappears() {
        let mut app = app_with_exact_source();
        reconcile_document(&mut app);
        visualization_configuration_status(&app.state)
            .expect("a compatible retained pane binding must validate");

        app.state.simulation.runs.clear();

        assert!(
            visualization_configuration_status(&app.state)
                .unwrap_err()
                .contains("unavailable dataset")
        );
    }

    fn append_retained_pole_zero_run(app: &mut RSpiceApp) -> (DatasetId, u64) {
        let analysis_sequence = 29;
        let analysis = AnalysisResult::new(analysis_sequence, AnalysisType::PoleZero, "PZ 29")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![
                    crate::state::ComplexResultValue {
                        real: -10.0,
                        imaginary: 20.0,
                    },
                    crate::state::ComplexResultValue {
                        real: -10.0,
                        imaginary: -20.0,
                    },
                ],
                zeros: vec![crate::state::ComplexResultValue {
                    real: -3.0,
                    imaginary: 0.0,
                }],
                gain: 4.25,
            });
        let mut run = SimulationRun::new(2);
        run.add_analysis(analysis);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs.push(run);
        (dataset_id, analysis_sequence)
    }

    fn append_retained_sensitivity_run(app: &mut RSpiceApp) -> (DatasetId, u64) {
        let analysis_sequence = 31;
        let analysis = AnalysisResult::new(analysis_sequence, AnalysisType::Sensitivity, "SENS 31")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Ac {
                    frequency_hz: 2.5e6,
                },
                rows: vec![
                    SensitivityResultRow {
                        parameter: "c1".to_owned(),
                        raw: -1.25e3,
                        normalized: -0.75,
                    },
                    SensitivityResultRow {
                        parameter: "r1".to_owned(),
                        raw: 4.5e-3,
                        normalized: 0.25,
                    },
                ],
            });
        let mut run = SimulationRun::new(3);
        run.add_analysis(analysis);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs.push(run);
        (dataset_id, analysis_sequence)
    }

    #[test]
    fn historical_sensitivity_binding_uses_its_retained_payload() {
        let mut app = app_with_exact_source();
        let (dataset_id, analysis_sequence) = append_retained_sensitivity_run(&mut app);
        assert_eq!(app.state.simulation.active_run_idx, Some(0));

        let definition =
            viewer_document("viewer-contribution").expect("registered contribution viewer");
        assert_eq!(
            resolved_viewer_availability_for_binding(
                &app.state,
                definition,
                Some(dataset_id),
                Some(analysis_sequence),
            ),
            Ok(ResultViewer::Contribution)
        );
    }

    #[test]
    fn sensitivity_exact_data_rows_preserve_parameter_values_output_and_basis() {
        let mut app = app_with_exact_source();
        append_retained_sensitivity_run(&mut app);
        assert!(app.state.simulation.select_run(1));

        let rows = exact_source_rows(&app.state);
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].stable_row, "31:sensitivity[0].raw");
        assert_eq!(rows[0].value, format!("{:.17e}", -1.25e3));
        assert_eq!(rows[0].origin, "V(out)");
        assert!(rows[0].coordinate.contains("parameter=c1"));
        assert!(rows[0].coordinate.contains("ac@2.50000000000000000e6Hz"));
        assert_eq!(rows[1].stable_row, "31:sensitivity[0].normalized");
        assert_eq!(rows[1].value, format!("{:.17e}", -0.75));
        assert_eq!(rows[2].stable_row, "31:sensitivity[1].raw");
        assert_eq!(rows[3].stable_row, "31:sensitivity[1].normalized");
    }

    #[test]
    fn historical_pole_zero_binding_uses_its_retained_payload_without_derived_state() {
        let mut app = app_with_exact_source();
        let (dataset_id, analysis_sequence) = append_retained_pole_zero_run(&mut app);
        assert_eq!(app.state.simulation.active_run_idx, Some(0));
        assert!(app.state.analysis.pole_zero_state.is_empty());

        let definition = viewer_document("viewer-pz").expect("registered PZ viewer");
        assert_eq!(
            resolved_viewer_availability_for_binding(
                &app.state,
                definition,
                Some(dataset_id),
                Some(analysis_sequence),
            ),
            Ok(ResultViewer::PoleZero)
        );
    }

    #[test]
    fn pole_zero_exact_data_rows_preserve_root_order_components_and_gain() {
        let mut app = app_with_exact_source();
        let (_, _) = append_retained_pole_zero_run(&mut app);
        assert!(app.state.simulation.select_run(1));

        let rows = exact_source_rows(&app.state);
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0].stable_row, "29:gain");
        assert_eq!(rows[0].value, format!("{:.17e}", 4.25));
        assert_eq!(rows[1].stable_row, "29:pole[0].real");
        assert_eq!(rows[1].value, format!("{:.17e}", -10.0));
        assert_eq!(rows[2].stable_row, "29:pole[0].imaginary");
        assert_eq!(rows[2].value, format!("{:.17e}", 20.0));
        assert_eq!(rows[3].stable_row, "29:pole[1].real");
        assert_eq!(rows[4].stable_row, "29:pole[1].imaginary");
        assert_eq!(rows[5].stable_row, "29:zero[0].real");
        assert_eq!(rows[6].stable_row, "29:zero[0].imaginary");
    }

    #[test]
    fn link_groups_apply_the_same_exact_x_range_and_cursor_pair() {
        let mut app = app_with_exact_source();
        reconcile_document(&mut app);
        let analysis_key = activate_voltage_wave_pane(&mut app);
        let first = app
            .state
            .workbench
            .visualization_studio
            .active_pane()
            .cloned()
            .expect("reconciliation must create the first pane");
        app.state
            .ui
            .results
            .analysis_plot_view_pane_mut(first.viewer, analysis_key, 0)
            .x = Some((0.25, 0.75));
        app.state.ui.results.cursors.a = Some(0.3);
        app.state.ui.results.cursors.b = Some(0.7);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        capture_active_link_state(&ctx, &mut app);

        let mut second = first.clone();
        second.id = first.id + 1;
        app.state
            .workbench
            .visualization_studio
            .panes
            .push(second.clone());
        app.state.workbench.visualization_studio.next_identity = second.id + 1;
        app.state.workbench.visualization_studio.active_pane = Some(second.id);
        app.state.workbench.visualization_studio.applied_link_pane = None;
        app.state
            .ui
            .results
            .analysis_plot_view_pane_mut(second.viewer, analysis_key, 0)
            .x = None;
        app.state.ui.results.cursors.clear();

        apply_active_link_state(&mut app);
        apply_queued_view_gesture(&ctx, &mut app);

        assert_eq!(
            app.state
                .ui
                .results
                .analysis_plot_view_pane(second.viewer, analysis_key, 0)
                .x,
            Some((0.25, 0.75))
        );
        assert_eq!(app.state.ui.results.cursors.a, Some(0.3));
        assert_eq!(app.state.ui.results.cursors.b, Some(0.7));
    }

    #[test]
    fn add_pane_binds_the_requested_retained_analysis_and_selects_its_run() {
        let mut app = app_with_exact_source();
        reconcile_document(&mut app);

        let historical_analysis =
            AnalysisResult::new(23, AnalysisType::Transient, "TRAN 23").with_waveforms(vec![
                WaveformData::new("V(history)", vec![0.0, 1.0], vec![0.0, 1.0], "#55ddaa"),
            ]);
        let mut historical_run = SimulationRun::new(2);
        historical_run.add_analysis(historical_analysis);
        let historical_dataset = historical_run.dataset_id;
        app.state.simulation.runs.push(historical_run);

        add_viewer_pane_bound(
            &mut app,
            "viewer-waveform",
            ResultViewer::Waves,
            historical_dataset,
            23,
            VisualizationPanePlacement::RightOfSelected,
            String::new(),
        );

        let studio = &app.state.workbench.visualization_studio;
        let pane = studio.active_pane().expect("new pane becomes active");
        assert_eq!(pane.dataset_id, historical_dataset);
        assert_eq!(pane.analysis_sequence, 23);
        assert_eq!(pane.placement, VisualizationPanePlacement::RightOfSelected);
        assert_eq!(pane.page, "Engineering");
        assert_eq!(pane.x_link, Some(1));
        assert_eq!(pane.cursor_group, Some(1));
        assert_eq!(app.state.simulation.active_run_idx, Some(1));
        assert_eq!(app.state.simulation.active_analysis_idx, Some(0));
    }

    #[test]
    fn new_page_pane_is_unlinked_and_commits_as_one_valid_transaction() {
        let mut app = app_with_exact_source();
        reconcile_document(&mut app);
        let binding = app
            .state
            .simulation
            .active_run()
            .and_then(|run| {
                app.state
                    .simulation
                    .active_analysis()
                    .map(|analysis| (run.dataset_id, analysis.id))
            })
            .expect("fixture has an active immutable binding");
        let before_revision = app.state.workbench.visualization_studio.revision;

        add_viewer_pane_bound(
            &mut app,
            "viewer-waveform",
            ResultViewer::Waves,
            binding.0,
            binding.1,
            VisualizationPanePlacement::NewWorksheetPage,
            "Statistics".to_owned(),
        );

        let studio = &app.state.workbench.visualization_studio;
        assert_eq!(studio.revision, before_revision + 1);
        assert_eq!(studio.panes.len(), 2);
        let pane = studio.active_pane().expect("new page pane becomes active");
        assert_eq!(pane.page, "Statistics");
        assert_eq!(pane.placement, VisualizationPanePlacement::NewWorksheetPage);
        assert_eq!(pane.x_link, None);
        assert_eq!(pane.cursor_group, None);
        studio
            .validate_presentation()
            .expect("the aggregate pane edit remains valid");
    }

    #[test]
    fn unavailable_add_pane_binding_leaves_the_document_unchanged() {
        let mut app = app_with_exact_source();
        reconcile_document(&mut app);
        let before = app.state.workbench.visualization_studio.clone();

        add_viewer_pane_bound(
            &mut app,
            "viewer-waveform",
            ResultViewer::Waves,
            DatasetId::new(),
            17,
            VisualizationPanePlacement::BelowSelected,
            String::new(),
        );

        assert_eq!(app.state.workbench.visualization_studio, before);
    }

    #[test]
    fn versioned_entity_projection_retains_exact_bindings_and_stable_identities() {
        let mut app = app_with_exact_source();
        let dataset_id = app
            .state
            .simulation
            .active_run()
            .expect("fixture retains an active run")
            .dataset_id;
        app.state.ui.results.cursors.a = Some(0.5);
        app.state
            .workbench
            .visualization_studio
            .markers
            .push(VisualizationMarker {
                id: 31,
                dataset_id,
                analysis_sequence: 17,
                waveform_name: "V(out)".to_owned(),
                sample_index: 1,
                x: 0.5,
                y: 2.5,
                label: "source sample".to_owned(),
            });
        app.state
            .workbench
            .visualization_studio
            .measurements
            .push(VisualizationMeasurement {
                id: 32,
                dataset_id,
                analysis_sequence: 17,
                expression: "rms(V(out))".to_owned(),
                value: 2.0,
            });
        app.state
            .workbench
            .visualization_studio
            .annotations
            .push(VisualizationAnnotation {
                id: 33,
                dataset_id,
                analysis_sequence: 17,
                x: 0.5,
                text: "review exact point".to_owned(),
            });

        let rows = result_entity_rows(&app.state);
        assert_eq!(
            rows.iter().map(|row| row.kind).collect::<Vec<_>>(),
            [
                "axis",
                "trace",
                "trace",
                "cursor",
                "marker",
                "measurement",
                "annotation",
            ]
        );
        assert_eq!(rows[0].identity, "axis:17:x");
        assert_eq!(rows[1].identity, "trace:17:0");
        assert!(rows[1].binding.contains("V(out)"));
        assert_eq!(rows[3].identity, "cursor:A");
        assert_eq!(rows[4].identity, "marker:31");
        assert_eq!(rows[5].identity, "measurement:32");
        assert_eq!(rows[6].identity, "annotation:33");
        let dataset_prefix = short_dataset(dataset_id);
        assert!(rows[1].binding.starts_with(&dataset_prefix));
        assert!(rows[4].binding.starts_with(&dataset_prefix));
        assert!(rows[6].binding.starts_with(&dataset_prefix));
    }

    #[test]
    fn viewer_columns_preserve_the_mockup_side_widths_exactly() {
        assert_eq!(visible_available_width(1_312.0, 50.0, 1_280.0), 1_230.0);
        let desktop = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_230.0, 540.0));
        let [library, stage, inspector] = viewer_column_rects(desktop, 190.0, 224.0);
        assert_eq!(library.width(), 190.0);
        assert_eq!(inspector.width(), 224.0);
        assert_eq!(stage.width(), 814.0);
        assert_eq!(stage.left() - library.right(), 1.0);
        assert_eq!(inspector.left() - stage.right(), 1.0);

        let tablet = Rect::from_min_size(egui::Pos2::ZERO, vec2(900.0, 430.0));
        let [library, stage, inspector] = viewer_column_rects(tablet, 158.0, 196.0);
        assert_eq!(library.width(), 158.0);
        assert_eq!(inspector.width(), 196.0);
        assert_eq!(stage.width(), 544.0);
    }

    #[test]
    fn compact_dialog_body_stays_inside_phone_and_tablet_frames() {
        assert_eq!(compact_dock_geometry(390.0), (372.0, 348.0));
        assert_eq!(compact_dock_geometry(800.0), (520.0, 496.0));
        let (window, body) = compact_dock_geometry(180.0);
        assert!(window <= 180.0);
        assert!(body < window);
    }
}
