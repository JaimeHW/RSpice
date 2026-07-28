//! Persistent Visualization Studio surface.
//!
//! This is the authoring projection of the same immutable result datasets
//! rendered by the Results quick view. It owns no solver samples and never
//! copies a result into a parallel data store. Viewer compatibility is
//! fail-closed: a catalog entry is selectable only when both its data contract
//! and a real Rust renderer are available.

mod actions;
mod dock;

use actions::*;
mod sections;
mod stage;

use sections::*;
use stage::*;

use dock::{
    active_family_sample_selection, concept_banner, dock_action, dock_body, empty_note,
    labeled_combo, numeric_policy,
    paint_bottom_rule, paint_top_rule, panel_heading, policy_row, property_row, separator,
    table_header,
};

use std::collections::{BTreeMap, HashSet};

use egui::{
    Align, Align2, Color32, Frame, Grid, Id, Layout, Margin, Rect, RichText, ScrollArea, Sense,
    Stroke, Ui, Vec2, vec2,
};
use serde::{Deserialize, Serialize};

use crate::analysis::calculator;
use crate::diagnostics::ConsoleMessage;
use crate::product::{DatasetBinding, DatasetId};
use crate::results::plot_export_preset::{
    ColorProfile, DeterministicNamingTemplate, ExportBackground, ExportPageSize,
    FontEmbeddingPolicy, FontPolicy, MetadataProvenancePolicy, PageGeometry, PageMargins,
    PageOrientation, PdfAConformance, PlotExportFormat, PlotExportPresetDefinition,
    PlotExportPresetScope, RasterResampling, VectorHandling, VectorRasterPolicy,
};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerGroup, viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AccessibleColorPalette, ColumnRole, ComparisonExecutionContract, ComparisonPolicy,
    ComparisonReceipt, ComparisonRequest, FamilyAggregationMethod, FamilyAggregationPolicy,
    FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap, FamilyPresentationPolicy,
    FamilyXDimension, FamilyXOrdering, MissingPointPolicy, NumericTolerance, PageUpdatePolicy,
    RowAlignmentPolicy, SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType,
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
    SurfaceRoute, design_system::WorkbenchIcon, state::Workspace,
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
const NATIVE_VIEWERS: [ResultViewer; 9] = [
    ResultViewer::Waves,
    ResultViewer::Bode,
    ResultViewer::Fft,
    ResultViewer::Eye,
    ResultViewer::Hist,
    ResultViewer::Contribution,
    ResultViewer::Specs,
    ResultViewer::Smith,
    ResultViewer::PoleZero,
];

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
    ExportPreset,
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
    family_query: String,
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
    draft_comparison_absolute_tolerance: f64,
    #[serde(skip)]
    draft_comparison_relative_tolerance: f64,
    #[serde(skip)]
    draft_export_preset_name: String,
    #[serde(skip)]
    draft_export_preset_scope: Option<PlotExportPresetScope>,
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
            draft_comparison_absolute_tolerance: 0.0,
            draft_comparison_relative_tolerance: 0.0,
            draft_export_preset_name: "Publication vector · A4".to_owned(),
            draft_export_preset_scope: Some(PlotExportPresetScope::Project),
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
    512
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
            if pane.viewer_document_id != viewer_document_id(pane.viewer) {
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

pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::VisualizationStudio);
    match app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        Ok(_) => {
            app.state.workbench.workspace = Workspace::Results;
            app.state.workbench.visualization_studio.normalize();
            app.state
                .workbench
                .specialist_tool_browser
                .record_recent(SurfaceId::VisualizationStudio);
        }
        Err(error) => app
            .state
            .push_user_message(ConsoleMessage::warning(error.to_string())),
    }
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

pub(crate) fn export_document(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Export);
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
    let comparison_dataset = active_binding.and_then(|(active, _)| {
        app.state
            .simulation
            .runs
            .iter()
            .find(|run| run.dataset_id != active)
            .map(|run| run.dataset_id)
    });
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
            studio.draft_comparison_dataset = comparison_dataset;
            studio.draft_comparison_absolute_tolerance = 0.0;
            studio.draft_comparison_relative_tolerance = 0.0;
        }
        VisualizationDock::ExportPreset => {
            if studio.draft_export_preset_name.trim().is_empty() {
                studio.draft_export_preset_name = "Publication vector · A4".to_owned();
            }
            studio
                .draft_export_preset_scope
                .get_or_insert(PlotExportPresetScope::Project);
        }
        VisualizationDock::CursorManager
        | VisualizationDock::Annotation
        | VisualizationDock::Export => {}
    }
    studio.dock = Some(dock);
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
    let viewer = app.state.ui.results.viewer;
    let viewer_document_id = viewer_document_id(viewer).to_owned();
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
    if let Some(analysis_index) = app.state.simulation.active_analysis_idx {
        app.state
            .ui
            .results
            .plot_view_mut(pane.viewer, analysis_index)
            .x = x_range;
    }
    if let Some((a, b)) = cursors {
        app.state.ui.results.cursors.a = a;
        app.state.ui.results.cursors.b = b;
    } else {
        app.state.ui.results.cursors.clear();
    }
    app.state.workbench.visualization_studio.applied_link_pane = Some(pane.id);
}

fn capture_active_link_state(app: &mut RSpiceApp) {
    let Some(pane) = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
    else {
        return;
    };
    let x_range = app
        .state
        .simulation
        .active_analysis_idx
        .and_then(|analysis_index| {
            app.state
                .ui
                .results
                .plot_view(pane.viewer, analysis_index)
                .x
        });
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

fn workspace_header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let configuration = visualization_configuration_status(&app.state);
    let (configuration_label, configuration_color) = if configuration.is_ok() {
        ("configuration valid", t.color.ok)
    } else {
        ("configuration blocked", t.color.warn)
    };
    let wide = ui.available_width() > 1_120.0;
    let phone = ui.available_width() <= 600.0;
    let show_origin = ui.available_width() <= 760.0;
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(14, 7))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                WORKSPACE_HEADER_HEIGHT,
                WORKSPACE_HEADER_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                let (mark, _) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::hover());
                ui.painter().rect(
                    mark,
                    0.0,
                    t.color.accent_dim,
                    Stroke::new(1.0, t.color.accent),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    mark.center(),
                    egui::Align2::CENTER_CENTER,
                    "XY",
                    theme::mono(tokens::FS_1, FontWeight::SemiBold),
                    t.color.accent,
                );
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    if show_origin
                        && app.state.workbench.previous_route().is_some()
                        && Button::new("← Source")
                            .ghost()
                            .show(ui)
                            .on_hover_text("Return to the exact navigation origin")
                            .clicked()
                    {
                        let _ = app
                            .state
                            .workbench
                            .navigate_back(RouteTransitionSource::User);
                    }
                    ui.label(
                        RichText::new("RESULT DOCUMENT · VIEWER-SPECIFIC CONTROLS")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new("Lab characterization data display")
                            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    if phone {
                        let response = status_label(ui, configuration_label, configuration_color);
                        if let Err(reason) = &configuration {
                            response.on_hover_text(reason);
                        }
                    }
                    if wide {
                        ui.label(
                            RichText::new(SUMMARY)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    }
                });
                if !phone {
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        let response = status_label(ui, configuration_label, configuration_color);
                        if let Err(reason) = &configuration {
                            response.on_hover_text(reason);
                        }
                        if wide {
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("RESULT PRESENTATION")
                                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                        .color(t.color.text_faint),
                                );
                                ui.label(
                                    RichText::new(OWNERSHIP)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                );
                            });
                        }
                    });
                }
            });
        });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

fn status_label(ui: &mut Ui, label: &str, color: Color32) -> egui::Response {
    ui.horizontal(|ui| {
        let (dot, _) = ui.allocate_exact_size(vec2(7.0, 13.0), Sense::hover());
        ui.painter().circle_filled(dot.center(), 3.0, color);
        ui.label(
            RichText::new(label)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
    })
    .response
}

fn visualization_configuration_status(state: &AppState) -> Result<(), String> {
    let studio = &state.workbench.visualization_studio;
    studio.validate_presentation()?;
    if studio.panes.is_empty() {
        return Err("No visualization pane is bound to an immutable result dataset".to_owned());
    }
    for pane in &studio.panes {
        let run = state
            .simulation
            .runs
            .iter()
            .find(|run| run.dataset_id == pane.dataset_id)
            .ok_or_else(|| format!("Pane {:02} references an unavailable dataset", pane.id))?;
        let analysis = run
            .analyses
            .iter()
            .find(|analysis| analysis.id == pane.analysis_sequence)
            .ok_or_else(|| format!("Pane {:02} references an unavailable analysis", pane.id))?;
        let definition = viewer_document(&pane.viewer_document_id)
            .ok_or_else(|| format!("Pane {:02} references an unknown viewer", pane.id))?;
        let analysis_ids = [analysis_manifest_id(analysis.analysis_type)];
        match viewer_compatibility(
            definition.id,
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        ) {
            ViewerCompatibility::Compatible => {}
            ViewerCompatibility::MissingAnalysis { .. } => {
                return Err(format!(
                    "Pane {:02} viewer is incompatible with its retained analysis",
                    pane.id
                ));
            }
            ViewerCompatibility::MissingExternalCapability { capability_id } => {
                return Err(format!(
                    "Pane {:02} requires unavailable capability {capability_id}",
                    pane.id
                ));
            }
            ViewerCompatibility::UnknownDocument | ViewerCompatibility::UnknownQuickMode => {
                return Err(format!("Pane {:02} viewer is not registered", pane.id));
            }
        }
        if renderer_for_viewer_document(definition.id) != Some(pane.viewer) {
            return Err(format!(
                "Pane {:02} has no exact renderer for its retained viewer",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::PoleZero && retained_pole_zero_payload(analysis).is_none() {
            return Err(format!(
                "Pane {:02} has no valid retained pole-zero payload",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::Contribution
            && retained_sensitivity_payload(analysis).is_none()
        {
            return Err(format!(
                "Pane {:02} has no valid retained sensitivity payload",
                pane.id
            ));
        }
    }
    for annotation in &studio.annotations {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == annotation.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == annotation.analysis_sequence)
        }) {
            return Err(format!(
                "Annotation {:02} references an unavailable source",
                annotation.id
            ));
        }
    }
    for marker in &studio.markers {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == marker.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == marker.analysis_sequence)
        }) {
            return Err(format!(
                "Marker {:02} references an unavailable source",
                marker.id
            ));
        }
    }
    for measurement in &studio.measurements {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == measurement.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == measurement.analysis_sequence)
        }) {
            return Err(format!(
                "Measurement {:02} references an unavailable source",
                measurement.id
            ));
        }
    }
    Ok(())
}

fn status_strip(ui: &mut Ui, app: &RSpiceApp) {
    let studio = &app.state.workbench.visualization_studio;
    let bound_datasets = studio
        .panes
        .iter()
        .map(|pane| pane.dataset_id)
        .collect::<HashSet<_>>();
    let dataset_count = bound_datasets.len();
    let pane_count = studio.panes.len();
    let linked_groups = studio
        .panes
        .iter()
        .flat_map(|pane| [pane.x_link, pane.cursor_group])
        .flatten()
        .collect::<HashSet<_>>()
        .len();
    let expression_count: usize = app.state.ui.results.exprs.values().map(Vec::len).sum();
    let samples: usize = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| bound_datasets.contains(&run.dataset_id))
        .flat_map(|run| &run.analyses)
        .flat_map(|analysis| &analysis.waveforms)
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum();
    let metrics = [
        (
            "Datasets",
            dataset_count.to_string(),
            if dataset_count == 0 {
                "No immutable dataset".to_owned()
            } else {
                format!("1 active · {} overlay", dataset_count.saturating_sub(1))
            },
        ),
        (
            "View panes",
            pane_count.to_string(),
            format!(
                "{linked_groups} linked groups · revision {}",
                studio.revision
            ),
        ),
        (
            "Expressions",
            expression_count.to_string(),
            "calculator-owned".to_owned(),
        ),
        (
            "Sample span",
            engineering_count(samples),
            "exact source samples".to_owned(),
        ),
    ];
    let touch_screen = ui.ctx().input(|input| input.has_touch_screen());
    let horizontal_strip = uses_horizontal_kpi_strip(
        ui.available_width(),
        app.state.workbench.coarse_pointer,
        touch_screen,
    );
    let t = Tokens::get(ui.ctx());
    if horizontal_strip {
        ScrollArea::horizontal()
            .id_salt("visualization.status-strip.mobile")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for (label, value, detail) in &metrics {
                        status_metric_card(ui, &t, 142.0, label, value, detail);
                    }
                });
            });
        return;
    }
    let card_width = (ui.available_width() / 4.0).max(1.0);
    Grid::new("visualization.status-strip")
        .num_columns(4)
        .spacing(Vec2::ZERO)
        .show(ui, |ui| {
            for (index, (label, value, detail)) in metrics.iter().enumerate() {
                status_metric_card(ui, &t, card_width, label, value, detail);
                if (index + 1) % 4 == 0 {
                    ui.end_row();
                }
            }
        });
}

fn status_metric_card(ui: &mut Ui, t: &Tokens, width: f32, label: &str, value: &str, detail: &str) {
    Frame::NONE
        .fill(t.color.bg_app)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_min_width((width - 1.0).max(1.0));
            ui.set_max_width((width - 1.0).max(1.0));
            ui.set_min_height(38.0);
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(
                    RichText::new(label.to_uppercase())
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
                ui.label(
                    RichText::new(value)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
        });
}

fn engineering_count(value: usize) -> String {
    match value {
        1_000_000_000.. => format!("{:.2}B", value as f64 / 1_000_000_000.0),
        1_000_000.. => format!("{:.2}M", value as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", value as f64 / 1_000.0),
        _ => value.to_string(),
    }
}

fn section_navigation(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ScrollArea::horizontal()
            .id_salt("visualization.sections")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                        let active = app.state.workbench.visualization_studio.section == section;
                        let id = Id::new(("visualization.section", section));
                        let response = ui
                            .push_id(id, |ui| {
                                let (rect, response) = ui.allocate_exact_size(
                                    vec2(142.0, SECTION_NAVIGATION_HEIGHT),
                                    Sense::click(),
                                );
                                let fill = if active {
                                    t.color.bg_active
                                } else if response.hovered() {
                                    t.color.bg_hover
                                } else {
                                    Color32::TRANSPARENT
                                };
                                ui.painter().rect_filled(rect, 0.0, fill);
                                ui.painter().vline(
                                    rect.right(),
                                    rect.y_range(),
                                    Stroke::new(1.0, t.color.border),
                                );
                                if active {
                                    ui.painter().hline(
                                        rect.x_range(),
                                        rect.bottom() - 1.0,
                                        Stroke::new(2.0, t.color.accent),
                                    );
                                }
                                ui.painter().text(
                                    rect.left_center() + vec2(10.0, 0.0),
                                    egui::Align2::LEFT_CENTER,
                                    format!("{:02}  {}", index + 1, section.label()),
                                    theme::sans(tokens::FS_1, FontWeight::Medium),
                                    if active {
                                        t.color.text
                                    } else {
                                        t.color.text_dim
                                    },
                                );
                                response
                            })
                            .inner;
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                active,
                                section.label(),
                            )
                        });
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role(egui::accesskit::Role::Tab);
                            node.set_selected(active);
                            node.set_label(section.label());
                        });
                        if response.clicked() {
                            app.state.workbench.visualization_studio.section = section;
                        }
                        if active && response.has_focus() {
                            let next = ui.input_mut(|input| {
                                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                    Some((index + 1) % VisualizationSection::ALL.len())
                                } else if input
                                    .consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)
                                {
                                    Some(
                                        (index + VisualizationSection::ALL.len() - 1)
                                            % VisualizationSection::ALL.len(),
                                    )
                                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home)
                                {
                                    Some(0)
                                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                                    Some(VisualizationSection::ALL.len() - 1)
                                } else {
                                    None
                                }
                            });
                            if let Some(next) = next {
                                let section = VisualizationSection::ALL[next];
                                app.state.workbench.visualization_studio.section = section;
                                ui.ctx().memory_mut(|memory| {
                                    memory
                                        .request_focus(Id::new(("visualization.section", section)))
                                });
                            }
                        }
                        theme::paint_focus_ring(ui, &response, response.rect);
                    }
                });
            });
    });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

fn compact_section_picker(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("visualization.compact-sections")
        .show(ui, |ui| {
            for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                let active = app.state.workbench.visualization_studio.section == section;
                let response = ui.add_sized(
                    [ui.available_width(), 44.0],
                    egui::Button::new(format!("{:02}  {}", index + 1, section.label()))
                        .selected(active),
                );
                if response.clicked() {
                    app.state.workbench.visualization_studio.section = section;
                    app.state.workbench.visualization_studio.touch_pane =
                        VisualizationTouchPane::Stage;
                }
            }
            ui.add_space(10.0);
            Frame::NONE
                .fill(t.color.bg_inset)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("EVIDENCE CONTRACT")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new(EVIDENCE)
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
        });
}

fn touch_dock(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().min(ui.clip_rect().width()).max(1.0);
    separator(ui, t.color.border_strong);
    ui.allocate_ui_with_layout(
        vec2(width, TOUCH_DOCK_HEIGHT - 1.0),
        Layout::top_down(Align::Min),
        |ui| {
            Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
                ui.set_width(width);
                ui.set_min_height(TOUCH_DOCK_HEIGHT - 1.0);
                ui.columns(3, |columns| {
                    let controls = [
                        (
                            VisualizationTouchPane::Sections,
                            "Sections",
                            WorkbenchIcon::Grid,
                        ),
                        (
                            VisualizationTouchPane::Inspector,
                            "Inspect",
                            WorkbenchIcon::Sliders,
                        ),
                        (
                            VisualizationTouchPane::Actions,
                            "Actions",
                            WorkbenchIcon::More,
                        ),
                    ];
                    for (column, (pane, label, icon)) in columns.iter_mut().zip(controls) {
                        let active = app.state.workbench.visualization_studio.touch_pane == pane;
                        let (rect, response) = column.allocate_exact_size(
                            vec2(column.available_width(), TOUCH_DOCK_HEIGHT - 1.0),
                            Sense::click(),
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                column.is_enabled(),
                                active,
                                label,
                            )
                        });
                        column.painter().rect_filled(
                            rect,
                            0.0,
                            if active {
                                t.color.bg_active
                            } else {
                                t.color.bg_panel
                            },
                        );
                        icon.paint(
                            column.painter(),
                            Rect::from_center_size(
                                rect.center_top() + vec2(0.0, 15.0),
                                Vec2::splat(16.0),
                            ),
                            if active {
                                t.color.accent
                            } else {
                                t.color.text_dim
                            },
                        );
                        column.painter().text(
                            rect.center_bottom() - vec2(0.0, 7.0),
                            egui::Align2::CENTER_BOTTOM,
                            label,
                            theme::sans(tokens::FS_0, FontWeight::Medium),
                            if active {
                                t.color.text
                            } else {
                                t.color.text_dim
                            },
                        );
                        if response.clicked() {
                            app.state.workbench.visualization_studio.touch_pane = if active {
                                VisualizationTouchPane::Stage
                            } else {
                                pane
                            };
                        }
                        theme::paint_focus_ring(column, &response, rect);
                    }
                });
            });
        },
    );
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
        ViewerCompatibility::UnknownDocument | ViewerCompatibility::UnknownQuickMode => {
            return Err("Viewer identity is not registered".to_owned());
        }
    }
    let viewer = renderer_for_viewer_document(definition.id)
        .ok_or_else(|| "No exact Rust renderer is registered for this viewer".to_owned())?;
    let binding_is_active = state
        .simulation
        .active_run()
        .is_some_and(|active| active.dataset_id == dataset_id)
        && state
            .simulation
            .active_analysis()
            .is_some_and(|active| active.id == analysis_sequence);
    let available = match viewer {
        ResultViewer::Waves => !analysis.waveforms.is_empty(),
        ResultViewer::Bode => {
            crate::state::ac_bode_summary_for_selection(run, Some(analysis_index)).is_some()
        }
        ResultViewer::Fft | ResultViewer::Eye => {
            crate::simulation::SimulationController::analysis_supports_transient_derivation(
                analysis.analysis_type,
            ) && !analysis.waveforms.is_empty()
        }
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
        ResultViewer::Hist
        | ResultViewer::Smith
        | ResultViewer::Op
        | ResultViewer::NoiseContrib
        | ResultViewer::Nyquist => {
            binding_is_active && result_document::viewer_is_available(state, viewer)
        }
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

fn renderer_for_viewer_document(id: &str) -> Option<ResultViewer> {
    match id {
        "viewer-waveform" => Some(ResultViewer::Waves),
        "viewer-bode" => Some(ResultViewer::Bode),
        "viewer-spectrum" => Some(ResultViewer::Fft),
        "viewer-smith" => Some(ResultViewer::Smith),
        "viewer-table" => Some(ResultViewer::Specs),
        "viewer-histogram" => Some(ResultViewer::Hist),
        "eye-viewer" => Some(ResultViewer::Eye),
        "viewer-pz" => Some(ResultViewer::PoleZero),
        "viewer-contribution" => Some(ResultViewer::Contribution),
        "viewer-transfer-function" => Some(ResultViewer::TransferFunction),
        _ => None,
    }
}

fn viewer_document_id(viewer: ResultViewer) -> &'static str {
    match viewer {
        ResultViewer::Waves => "viewer-waveform",
        ResultViewer::Bode | ResultViewer::Nyquist => "viewer-bode",
        ResultViewer::Fft | ResultViewer::NoiseContrib => "viewer-spectrum",
        ResultViewer::Eye => "eye-viewer",
        ResultViewer::Hist => "viewer-histogram",
        ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table => "viewer-table",
        ResultViewer::Smith => "viewer-smith",
        ResultViewer::PoleZero => "viewer-pz",
        ResultViewer::Contribution => "viewer-contribution",
        ResultViewer::TransferFunction => "viewer-transfer-function",
    }
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

#[allow(clippy::too_many_arguments)]
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
    use super::*;
    use super::dock::{evaluate_scalar_measurement, execute_comparison_draft};
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecEntry, WaveformData};

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
            min: Some(-2.0),
            max: Some(5.0),
            unit: "V".to_owned(),
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
        app.state.workspace.specs.push(SpecEntry {
            measurement: "V(out)".to_owned(),
            min: Some(-2.0),
            max: Some(5.0),
            unit: "V".to_owned(),
        });
        app.state.workbench.visualization_studio.autoscale =
            VisualizationAutoscale::SpecificationBounds;
        app.state.workbench.visualization_studio.zoom = 3.0;

        fit_active_view(&mut app);

        let view = app.state.ui.results.plot_view(ResultViewer::Waves, 0);
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
        let candidate_digest = app
            .state
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest();

        let receipt = execute_comparison_draft(&app).expect("exact comparison must execute");

        assert_eq!(receipt.rows_compared, 3);
        assert_eq!(
            receipt.policy.execution,
            ComparisonExecutionContract::default()
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
        let first = app
            .state
            .workbench
            .visualization_studio
            .active_pane()
            .cloned()
            .expect("reconciliation must create the first pane");
        app.state.ui.results.plot_view_mut(first.viewer, 0).x = Some((0.25, 0.75));
        app.state.ui.results.cursors.a = Some(0.3);
        app.state.ui.results.cursors.b = Some(0.7);
        capture_active_link_state(&mut app);

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
        app.state.ui.results.plot_view_mut(second.viewer, 0).x = None;
        app.state.ui.results.cursors.clear();

        apply_active_link_state(&mut app);

        assert_eq!(
            app.state.ui.results.plot_view(second.viewer, 0).x,
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
