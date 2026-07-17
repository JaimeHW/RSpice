//! Persistent Visualization Studio surface.
//!
//! This is the authoring projection of the same immutable result datasets
//! rendered by the Results quick view. It owns no solver samples and never
//! copies a result into a parallel data store. Viewer compatibility is
//! fail-closed: a catalog entry is selectable only when both its data contract
//! and a real Rust renderer are available.

use std::collections::{BTreeMap, HashSet};

use egui::{
    Align, Align2, Color32, Frame, Grid, Id, Layout, Margin, Rect, RichText, ScrollArea, Sense,
    Stroke, Ui, Vec2, vec2,
};
use serde::{Deserialize, Serialize};

use crate::analysis::calculator;
use crate::common::{AppState, RSpiceApp, app::ConsoleMessage};
use crate::product::DatasetId;
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerGroup, viewer_compatibility, viewer_document,
};
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};

use super::{
    ChoicePreference, ResultViewer, RouteTransitionSource, ScalarPreference, SurfaceId,
    SurfaceRoute, design_system::WorkbenchIcon, result_document, state::Workspace,
};

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
const NATIVE_VIEWERS: [ResultViewer; 8] = [
    ResultViewer::Waves,
    ResultViewer::Bode,
    ResultViewer::Fft,
    ResultViewer::Eye,
    ResultViewer::Hist,
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

    const fn title(self) -> &'static str {
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
    draft_overlay_ids: Vec<DatasetId>,
    #[serde(skip)]
    draft_significant_digits: Option<u8>,
    #[serde(skip)]
    draft_phase_continuous: Option<bool>,
    #[serde(skip)]
    applied_link_pane: Option<u64>,
    #[serde(skip)]
    family_query: String,
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
            draft_link_pane: None,
            draft_x_link: 0,
            draft_cursor_group: 0,
            draft_pane_order: Vec::new(),
            draft_trace_dataset: None,
            draft_trace_analysis: None,
            draft_trace_visibility: Vec::new(),
            draft_overlay_ids: Vec::new(),
            draft_significant_digits: None,
            draft_phase_continuous: None,
            applied_link_pane: None,
            family_query: String::new(),
            operation_state: OperationState::NotStarted,
            operation_dataset_id: None,
            operation_analysis_sequence: None,
            operation_processed: 0,
            operation_total: 0,
            operation_checksum: 0,
        }
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
    let overlay_ids = app.state.simulation.overlay_dataset_ids.clone();
    let phase_continuous = app.state.ui.results.phase_continuous;
    let active_viewer = app.state.ui.results.viewer;
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
        }
        VisualizationDock::Measurement => {}
        VisualizationDock::FamilySlice | VisualizationDock::FamilyFilter => {
            studio.draft_overlay_ids = overlay_ids;
        }
        VisualizationDock::FamilyEncoding => {
            studio.draft_phase_continuous = Some(phase_continuous);
        }
        VisualizationDock::CursorManager
        | VisualizationDock::Annotation
        | VisualizationDock::Export => {}
    }
    studio.dock = Some(dock);
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
                    }
                });
            });
        },
    );
}

fn show_active_section(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    if compact {
        match app.state.workbench.visualization_studio.touch_pane {
            VisualizationTouchPane::Sections => return,
            VisualizationTouchPane::Inspector => {
                viewer_inspector(ui, app, true);
                return;
            }
            VisualizationTouchPane::Actions => {
                actions_sheet(ui, app);
                return;
            }
            VisualizationTouchPane::Stage => {}
        }
    }

    match app.state.workbench.visualization_studio.section {
        VisualizationSection::Document => document_section(ui, app),
        VisualizationSection::Viewers => viewers_section(ui, app, compact),
        VisualizationSection::Axes => axes_section(ui, app),
        VisualizationSection::Families => families_section(ui, app),
        VisualizationSection::Measurements => measurements_section(ui, app),
        VisualizationSection::LargeData => large_data_section(ui, app),
        VisualizationSection::ExportReport => export_section(ui, app),
    }
}

fn section_heading(ui: &mut Ui, section: VisualizationSection) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(format!("VIZ · {}", section.label().to_uppercase()))
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_faint),
            );
            ui.label(
                RichText::new(section.title())
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(section.description())
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
    separator(ui, t.color.border);
}

fn section_scroll(ui: &mut Ui, id: &'static str, content: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 12,
            right: 12,
            top: 0,
            bottom: 12,
        })
        .show(ui, |ui| {
            ScrollArea::both()
                .id_salt(id)
                .auto_shrink([false, false])
                .show(ui, content);
        });
}

fn document_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Document);
    section_scroll(ui, "visualization.document", |ui| {
        Grid::new("visualization.document.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in [
                    "Pane",
                    "Viewer",
                    "Dataset",
                    "X link",
                    "Cursor group",
                    "Page",
                ] {
                    table_header(ui, label);
                }
                ui.end_row();
                if app.state.workbench.visualization_studio.panes.is_empty() {
                    ui.label("—");
                    ui.label("No panes in this result document");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                for pane in &app.state.workbench.visualization_studio.panes {
                    ui.monospace(format!("{:02}", pane.id));
                    ui.label(pane.viewer.label());
                    ui.monospace(short_dataset(pane.dataset_id));
                    ui.monospace(
                        pane.x_link
                            .map_or_else(|| "none".to_owned(), |id| format!("x-{id}")),
                    );
                    ui.monospace(
                        pane.cursor_group
                            .map_or_else(|| "none".to_owned(), |id| format!("cursor-{id}")),
                    );
                    ui.label(&pane.page);
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Add pane…", VisualizationDock::AddPane);
            dock_action(ui, app, "Reorder panes…", VisualizationDock::ReorderPanes);
            dock_action(ui, app, "Link groups…", VisualizationDock::LinkGroups);
            dock_action(ui, app, "Page editor…", VisualizationDock::PageEditor);
        });
    });
}

fn axes_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Axes);
    section_scroll(ui, "visualization.axes", |ui| {
        Grid::new("visualization.axes.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Axis", "Quantity", "Transform", "Range", "Ticks", "Unit"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let Some(analysis) = app.state.simulation.active_analysis() else {
                    ui.label("—");
                    ui.label("No active result dataset");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                    return;
                };
                if let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible)
                {
                    let (x0, x1) = waveform.x_range();
                    let (y0, y1) = waveform.y_range();
                    let frequency = matches!(
                        analysis.analysis_type,
                        crate::state::AnalysisType::Ac | crate::state::AnalysisType::Noise
                    );
                    axis_row(
                        ui,
                        "X1",
                        if frequency {
                            "frequency"
                        } else {
                            "time / sweep"
                        },
                        if frequency { "log10" } else { "linear" },
                        (x0, x1),
                        if frequency { "decade" } else { "engineering" },
                        if frequency { "Hz" } else { "source" },
                    );
                    axis_row(
                        ui,
                        "Y1L",
                        &waveform.name,
                        "linear",
                        (y0, y1),
                        "engineering",
                        "source",
                    );
                    if waveform.complex.is_some() {
                        axis_row(
                            ui,
                            "Y1R",
                            "complex projection",
                            "phase",
                            (-180.0, 180.0),
                            "45°",
                            "deg",
                        );
                    }
                } else {
                    ui.label("—");
                    ui.label("Active analysis has no visible waveform");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
                labeled_combo(
                    ui,
                    "Autoscale",
                    app.state.workbench.visualization_studio.autoscale.label(),
                    |ui| {
                        for value in VisualizationAutoscale::ALL {
                            let configured = match value {
                                VisualizationAutoscale::RobustVisible => true,
                                VisualizationAutoscale::ExactExtrema => {
                                    app.state.ui.results.viewer == ResultViewer::Waves
                                }
                                VisualizationAutoscale::SpecificationBounds => false,
                            };
                            ui.add_enabled_ui(configured, |ui| {
                                ui.selectable_value(
                                    &mut app.state.workbench.visualization_studio.autoscale,
                                    value,
                                    value.label(),
                                );
                            })
                            .response
                            .on_disabled_hover_text(
                                "This fit policy is unavailable for the active renderer or requires a quantity-mapped axis limit.",
                            );
                        }
                    },
                );
                labeled_combo(
                    ui,
                    "Complex projection",
                    app.state
                        .workbench
                        .visualization_studio
                        .complex_projection
                        .label(),
                    |ui| {
                        for value in ComplexProjection::ALL {
                            ui.selectable_value(
                                &mut app.state.workbench.visualization_studio.complex_projection,
                                value,
                                value.label(),
                            );
                        }
                    },
                );
                if Button::new("Fit active view").show(ui).clicked() {
                    fit_active_view(app);
                }
            });
    });
}

fn axis_row(
    ui: &mut Ui,
    axis: &str,
    quantity: &str,
    transform: &str,
    range: (f64, f64),
    ticks: &str,
    unit: &str,
) {
    ui.monospace(axis);
    ui.label(quantity);
    ui.monospace(transform);
    ui.monospace(format!("{:.6e}…{:.6e}", range.0, range.1));
    ui.label(ticks);
    ui.monospace(unit);
    ui.end_row();
}

fn families_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Families);
    let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
    let rows: Vec<_> = app
        .state
        .simulation
        .runs
        .iter()
        .map(|run| {
            let samples = run
                .analyses
                .iter()
                .flat_map(|analysis| &analysis.waveforms)
                .map(|waveform| waveform.x.len().min(waveform.y.len()))
                .sum::<usize>();
            (
                run.dataset_id,
                run.label.clone(),
                run.analyses.len(),
                samples,
                Some(run.dataset_id) == active_dataset,
                app.state.simulation.is_dataset_overlaid(run.dataset_id),
            )
        })
        .collect();
    section_scroll(ui, "visualization.families", |ui| {
        Grid::new("visualization.families.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Dataset", "Run", "Analyses", "Samples", "Role", "Display"] {
                    table_header(ui, label);
                }
                ui.end_row();
                if rows.is_empty() {
                    ui.label("—");
                    ui.label("No retained datasets");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                let mut overlay_change = None;
                for (dataset_id, label, analyses, samples, active, overlaid) in &rows {
                    ui.monospace(short_dataset(*dataset_id));
                    ui.label(label);
                    ui.monospace(analyses.to_string());
                    ui.monospace(engineering_count(*samples));
                    ui.label(if *active {
                        "active family"
                    } else {
                        "retained family"
                    });
                    if *active {
                        ui.label("always visible");
                    } else if ui.checkbox(&mut overlaid.clone(), "Overlay").changed() {
                        overlay_change = Some(*dataset_id);
                    }
                    ui.end_row();
                }
                if let Some(dataset_id) = overlay_change {
                    app.state.simulation.toggle_dataset_overlay(dataset_id);
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Slice and pivot…", VisualizationDock::FamilySlice);
            dock_action(
                ui,
                app,
                "Visual encoding…",
                VisualizationDock::FamilyEncoding,
            );
            dock_action(ui, app, "Advanced filter…", VisualizationDock::FamilyFilter);
        });
        concept_banner(
            ui,
            "Dataset overlays use stable dataset identities. Missing analyses remain absent; the viewer never invents family points or generated trace indices.",
        );
    });
}

fn measurements_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::Measurements);
    section_scroll(ui, "visualization.measurements", |ui| {
        Grid::new("visualization.measurements.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Item", "Type", "Definition", "Unit", "Consumers", "Status"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
                let expressions = app
                    .state
                    .ui
                    .results
                    .exprs
                    .get(&strip)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                for measurement in &app.state.workbench.visualization_studio.measurements {
                    measurement_row(
                        ui,
                        &format!("M{}", measurement.id),
                        "scalar measurement",
                        &measurement.expression,
                        "source-derived",
                        &short_dataset(measurement.dataset_id),
                        &format!("{:.9e}", measurement.value),
                    );
                }
                for (index, expression) in expressions.iter().enumerate() {
                    measurement_row(
                        ui,
                        &format!("expr-{}", index + 1),
                        "expression",
                        &expression.text,
                        "source-derived",
                        "active pane",
                        if expression.visible {
                            "visible"
                        } else {
                            "hidden"
                        },
                    );
                }
                if app.state.ui.results.cursors.a.is_some()
                    || app.state.ui.results.cursors.b.is_some()
                {
                    measurement_row(
                        ui,
                        "A / B",
                        "linked cursors",
                        "exact source coordinates",
                        "source",
                        "compatible panes",
                        if app.state.ui.results.linked_cursors {
                            "linked"
                        } else {
                            "pane local"
                        },
                    );
                }
                for marker in &app.state.workbench.visualization_studio.markers {
                    measurement_row(
                        ui,
                        &marker.label,
                        "sample marker",
                        &format!(
                            "{}[{}] @ {:.9e}",
                            marker.waveform_name, marker.sample_index, marker.x
                        ),
                        "source",
                        "active pane",
                        "exact",
                    );
                }
                for annotation in &app.state.workbench.visualization_studio.annotations {
                    measurement_row(
                        ui,
                        &format!("NOTE-{}", annotation.id),
                        "review annotation",
                        &annotation.text,
                        "—",
                        "result document",
                        "open",
                    );
                }
                if app
                    .state
                    .workbench
                    .visualization_studio
                    .measurements
                    .is_empty()
                    && expressions.is_empty()
                    && app.state.ui.results.cursors.a.is_none()
                    && app.state.workbench.visualization_studio.markers.is_empty()
                    && app
                        .state
                        .workbench
                        .visualization_studio
                        .annotations
                        .is_empty()
                {
                    ui.label("—");
                    ui.label("No derived or review entities");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "New measurement…", VisualizationDock::Measurement);
            dock_action(ui, app, "Cursor manager…", VisualizationDock::CursorManager);
            dock_action(ui, app, "New annotation…", VisualizationDock::Annotation);
        });
    });
}

fn measurement_row(
    ui: &mut Ui,
    item: &str,
    kind: &str,
    definition: &str,
    unit: &str,
    consumers: &str,
    status: &str,
) {
    ui.monospace(item);
    ui.label(kind);
    ui.monospace(definition);
    ui.monospace(unit);
    ui.label(consumers);
    ui.label(status);
    ui.end_row();
}

fn large_data_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::LargeData);
    section_scroll(ui, "visualization.large-data", |ui| {
        let previous = app.state.workbench.visualization_studio.display_lod;
        ui.horizontal_wrapped(|ui| {
            labeled_combo(
                ui,
                "Display LOD",
                app.state.workbench.visualization_studio.display_lod.label(),
                |ui| {
                    for value in DisplayLodPolicy::ALL {
                        ui.selectable_value(
                            &mut app.state.workbench.visualization_studio.display_lod,
                            value,
                            value.label(),
                        );
                    }
                },
            );
            numeric_policy(
                ui,
                "Tile memory",
                &mut app.state.workbench.visualization_studio.tile_memory_mib,
                64..=16_384,
                "MiB",
            );
            property_row(ui, "Disk cache", "Not configured · no filesystem writes");
        });
        ui.add_space(10.0);
        Grid::new("visualization.large-data.policies")
            .num_columns(2)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                policy_row(
                    ui,
                    "Exact cursor query",
                    "Read original f64/complex source samples on demand",
                );
                policy_row(
                    ui,
                    "Remote streaming",
                    "Local immutable dataset registry; remote sources fail closed",
                );
                policy_row(
                    ui,
                    "Backpressure",
                    "Preserve solver output · delay presentation cache",
                );
                policy_row(
                    ui,
                    "Source precision",
                    "Measurements and exports bypass display LOD",
                );
            });
        if previous != app.state.workbench.visualization_studio.display_lod {
            apply_lod_policy(app);
        }
        concept_banner(
            ui,
            "Decimation and level-of-detail affect rendering only. Measurements, exports, and cursor exact-value requests operate on the immutable source dataset.",
        );
    });
}

fn export_section(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, VisualizationSection::ExportReport);
    section_scroll(ui, "visualization.export", |ui| {
        Grid::new("visualization.export.table")
            .num_columns(5)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Output", "Format", "Precision", "Layout", "Provenance"] {
                    table_header(ui, label);
                }
                ui.end_row();
                export_row(
                    ui,
                    "Active engineering viewer",
                    "PNG",
                    "rendered pixels",
                    "active viewport",
                    "dataset + revision in document",
                );
                export_row(
                    ui,
                    "Engineering dataset",
                    "CSV",
                    "full stored f64",
                    "shared-axis table",
                    "source analysis identity",
                );
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, app, "Edit report pages…", VisualizationDock::PageEditor);
            if Button::new("Export exact data…")
                .accent()
                .show(ui)
                .clicked()
            {
                app.state.ui.export_csv_requested = true;
            }
            if Button::new("Export viewer image…").show(ui).clicked() {
                app.state.ui.export_png_requested = true;
            }
        });
        concept_banner(
            ui,
            "Every enabled export action is backed by a real writer. Formats without an installed writer are not offered and no placeholder artifact is created.",
        );
    });
}

fn export_row(
    ui: &mut Ui,
    output: &str,
    format: &str,
    precision: &str,
    layout: &str,
    provenance: &str,
) {
    ui.label(output);
    ui.monospace(format);
    ui.label(precision);
    ui.label(layout);
    ui.label(provenance);
    ui.end_row();
}

fn viewers_section(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    section_heading(ui, VisualizationSection::Viewers);
    viewer_toolbar(ui, app, compact);
    let height = ui.available_height().max(1.0);
    if compact {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            Layout::top_down(Align::Min),
            |ui| viewer_stage(ui, app),
        );
        return;
    }

    // A horizontally scrollable ancestor may expose a logical available
    // width wider than the visible canvas. The mockup columns are viewport
    // columns, so clamp their allocation to the active clip rectangle.
    let available = visible_available_width(
        ui.available_width(),
        ui.cursor().left(),
        ui.clip_rect().right(),
    );
    let (library_width, inspector_width) = if available <= NARROW_VIEWER_BREAKPOINT {
        (158.0, 196.0)
    } else {
        (190.0, 224.0)
    };
    // `allocate_ui_with_layout` is allowed to grow beyond its requested size
    // when a descendant reports a larger minimum. The exact-data table and
    // long status strings therefore used to steal width from the inspector at
    // 1280 px even though the mockup declares fixed 190/224 px side columns.
    // Reserve and clip all three column rectangles up front so content can
    // scroll or elide within its owner, never resize a sibling pane.
    let (rect, _) = ui.allocate_exact_size(vec2(available, height), Sense::hover());
    let [library_rect, stage_rect, inspector_rect] =
        viewer_column_rects(rect, library_width, inspector_width);
    let t = Tokens::get(ui.ctx());
    for x in [library_rect.right() + 0.5, inspector_rect.left() - 0.5] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border_strong));
    }

    let mut library_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(library_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    library_ui.set_clip_rect(library_ui.clip_rect().intersect(library_rect));
    viewer_library(&mut library_ui, app);

    let mut stage_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(stage_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    stage_ui.set_clip_rect(stage_ui.clip_rect().intersect(stage_rect));
    viewer_stage(&mut stage_ui, app);

    let mut inspector_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inspector_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    inspector_ui.set_clip_rect(inspector_ui.clip_rect().intersect(inspector_rect));
    viewer_inspector(&mut inspector_ui, app, false);
}

fn viewer_toolbar(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.set_min_height(if compact {
                44.0
            } else {
                bar_content_height(VIEWER_TOOLBAR_HEIGHT, VIEWER_TOOLBAR_VERTICAL_MARGIN)
            });
            ScrollArea::horizontal()
                .id_salt("visualization.viewer-toolbar")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let waveform_coordinates =
                            app.state.ui.results.viewer == ResultViewer::Waves;
                        for tool in ViewerTool::ALL {
                            let active = app.state.workbench.visualization_studio.tool == tool;
                            if ui
                                .add_sized(
                                    [
                                        if compact { 64.0 } else { 54.0 },
                                        if compact { 42.0 } else { 26.0 },
                                    ],
                                    egui::Button::new(tool.label()).selected(active),
                                )
                                .clicked()
                            {
                                app.state.workbench.visualization_studio.tool = tool;
                            }
                        }
                        toolbar_action(ui, "Add trace", || open_trace_manager(app));
                        toolbar_action(ui, "Edit axis", || {
                            app.state.workbench.visualization_studio.section =
                                VisualizationSection::Axes;
                        });
                        toolbar_action_enabled(
                            ui,
                            "Add cursor",
                            waveform_coordinates,
                            "Exact source cursors are available in the waveform renderer",
                            || add_cursor_at_midpoint(app),
                        );
                        toolbar_action_enabled(
                            ui,
                            "Add marker",
                            waveform_coordinates,
                            "Exact source markers are available in the waveform renderer",
                            || add_marker_at_midpoint(app),
                        );
                        toolbar_action(ui, "Measure", || {
                            open_dock(app, VisualizationDock::Measurement);
                        });
                        toolbar_action(ui, "Annotate", || {
                            open_dock(app, VisualizationDock::Annotation);
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            toolbar_action(ui, "Export…", || export_document(app));
                            toolbar_action(ui, "Axes & display…", || {
                                app.state.workbench.visualization_studio.section =
                                    VisualizationSection::Axes;
                            });
                            if ui
                                .add_enabled(waveform_coordinates, egui::Button::new("Fit"))
                                .clicked()
                            {
                                fit_active_view(app);
                            }
                            if ui
                                .add_enabled(waveform_coordinates, egui::Button::new("+"))
                                .on_hover_text("Zoom in")
                                .clicked()
                            {
                                zoom_active(app, 1.25);
                            }
                            ui.label(
                                RichText::new(format!(
                                    "{}%",
                                    (app.state.workbench.visualization_studio.zoom * 100.0).round()
                                        as u32
                                ))
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text_dim),
                            );
                            if ui
                                .add_enabled(waveform_coordinates, egui::Button::new("−"))
                                .on_hover_text("Zoom out")
                                .clicked()
                            {
                                zoom_active(app, 0.8);
                            }
                        });
                    });
                });
        });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

fn toolbar_action(ui: &mut Ui, label: &'static str, action: impl FnOnce()) {
    if Button::new(label).ghost().show(ui).clicked() {
        action();
    }
}

fn toolbar_action_enabled(
    ui: &mut Ui,
    label: &'static str,
    enabled: bool,
    unavailable_reason: &'static str,
    action: impl FnOnce(),
) {
    if ui
        .add_enabled(enabled, egui::Button::new(label))
        .on_disabled_hover_text(unavailable_reason)
        .clicked()
    {
        action();
    }
}

fn viewer_library(ui: &mut Ui, app: &mut RSpiceApp) {
    panel_heading(ui, "Viewer library", &VIEWER_DOCUMENTS.len().to_string());
    let query = &mut app.state.workbench.visualization_studio.viewer_query;
    let t = Tokens::get(ui.ctx());
    Frame::NONE.inner_margin(Margin::same(8)).show(ui, |ui| {
        let response = ui.add_sized(
            [ui.available_width(), 28.0],
            egui::TextEdit::singleline(query)
                .hint_text("Filter viewers")
                .margin(Margin {
                    left: 29,
                    right: 8,
                    top: 4,
                    bottom: 4,
                })
                .desired_width(f32::INFINITY),
        );
        let icon_rect = Rect::from_center_size(
            egui::pos2(response.rect.left() + 13.5, response.rect.center().y),
            Vec2::splat(13.0),
        );
        WorkbenchIcon::Search.paint(ui.painter(), icon_rect, t.color.text_faint);
    });

    let query = query.trim().to_ascii_lowercase();
    let analysis_ids = available_analysis_ids(&app.state);
    let capabilities = ViewerCapabilities {
        analysis_ids: &analysis_ids,
        external_capabilities: &[],
    };
    let mut selected = None;
    ScrollArea::vertical()
        .id_salt("visualization.viewer-library")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for group in ViewerGroup::ALL {
                let rows: Vec<_> = VIEWER_DOCUMENTS
                    .iter()
                    .filter(|definition| definition.group == group)
                    .filter(|definition| {
                        query.is_empty()
                            || definition.title.to_ascii_lowercase().contains(&query)
                            || definition.domain.to_ascii_lowercase().contains(&query)
                    })
                    .collect();
                if rows.is_empty() {
                    continue;
                }
                viewer_group_heading(ui, group.label());
                for definition in rows {
                    let availability =
                        resolved_viewer_availability(&app.state, definition, capabilities);
                    let active = app
                        .state
                        .workbench
                        .visualization_studio
                        .selected_viewer_document
                        == definition.id;
                    let response = viewer_library_row(
                        ui,
                        definition,
                        active,
                        availability.is_ok(),
                        availability.as_ref().err().map(String::as_str),
                    );
                    if availability.is_ok() && response.clicked() {
                        selected = Some((definition.id, availability));
                    }
                }
            }
        });
    if let Some((id, availability)) = selected {
        app.state
            .workbench
            .visualization_studio
            .selected_viewer_document = id.to_owned();
        if let Ok(viewer) = availability {
            add_viewer_pane(app, id, viewer);
        }
    }
}

fn viewer_group_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let heading = Frame::NONE
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 5,
        })
        .show(ui, |ui| {
            ui.label(
                RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
        });
    paint_top_rule(ui, heading.response.rect, t.color.border);
    paint_bottom_rule(ui, heading.response.rect, t.color.border);
}

fn viewer_library_row(
    ui: &mut Ui,
    definition: &ViewerDocumentDefinition,
    active: bool,
    available: bool,
    reason: Option<&str>,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), 36.0),
        if available {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        },
    );
    if active {
        ui.painter().vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }
    let color = if available {
        t.color.text
    } else {
        t.color.text_faint
    };
    ui.painter().text(
        rect.left_top() + vec2(8.0, 8.0),
        egui::Align2::LEFT_TOP,
        definition.title,
        theme::sans(tokens::FS_1, FontWeight::Medium),
        color,
    );
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let detail = elide_text_to_width(
        ui,
        if available {
            definition.domain
        } else {
            reason.unwrap_or("Unavailable")
        },
        &detail_font,
        (rect.width() - 16.0).max(1.0),
    );
    ui.painter().text(
        rect.left_bottom() + vec2(8.0, -5.0),
        egui::Align2::LEFT_BOTTOM,
        detail,
        detail_font,
        t.color.text_faint,
    );
    if let Some(reason) = reason {
        response.on_hover_text(reason)
    } else {
        response
    }
}

fn elide_text_to_width(ui: &Ui, text: &str, font: &egui::FontId, maximum_width: f32) -> String {
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return text.to_owned();
    }
    let mut candidate = text.to_owned();
    while candidate.pop().is_some() {
        let elided = format!("{}…", candidate.trim_end());
        if ui
            .painter()
            .layout_no_wrap(elided.clone(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            return elided;
        }
    }
    "…".to_owned()
}

fn resolved_viewer_availability(
    state: &AppState,
    definition: &ViewerDocumentDefinition,
    capabilities: ViewerCapabilities<'_>,
) -> Result<ResultViewer, String> {
    match viewer_compatibility(definition.id, capabilities) {
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
    let Some(viewer) = renderer_for_viewer_document(definition.id) else {
        return Err("No exact Rust renderer is registered for this viewer".to_owned());
    };
    if !result_document::viewer_is_available(state, viewer) {
        return Err(result_document::viewer_unavailability_reason(state, viewer)
            .unwrap_or("The retained result does not satisfy this viewer contract")
            .to_owned());
    }
    Ok(viewer)
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
        ResultViewer::Hist
        | ResultViewer::Smith
        | ResultViewer::PoleZero
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
        ResultViewer::Op | ResultViewer::Specs => "viewer-table",
        ResultViewer::Smith => "viewer-smith",
        ResultViewer::PoleZero => "viewer-pz",
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
        AnalysisType::Tf => "tf",
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

fn viewer_stage(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let selected = app
        .state
        .workbench
        .visualization_studio
        .selected_viewer_document
        .clone();
    let definition = viewer_document(&selected);
    let analysis_ids = available_analysis_ids(&app.state);
    let capabilities = ViewerCapabilities {
        analysis_ids: &analysis_ids,
        external_capabilities: &[],
    };
    let availability = active_binding_error(app).map_or_else(
        || {
            definition
                .ok_or_else(|| "Viewer identity is not registered".to_owned())
                .and_then(|definition| {
                    resolved_viewer_availability(&app.state, definition, capabilities)
                })
        },
        Err,
    );
    let compatible = availability.is_ok();
    let header = Frame::NONE
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(9, 6))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_HEADER_HEIGHT,
                VIEWER_STAGE_HEADER_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(definition.map_or("RESULT VIEWER", |meta| meta.domain))
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new(definition.map_or("Unknown viewer", |meta| meta.title))
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let source = app.state.simulation.active_run().map_or_else(
                        || "No active dataset".to_owned(),
                        |run| format!("{} · {}", run.label, short_dataset(run.dataset_id)),
                    );
                    status_label(
                        ui,
                        &source,
                        if availability.is_ok() {
                            t.color.ok
                        } else {
                            t.color.warn
                        },
                    );
                });
            });
        });
    paint_bottom_rule(ui, header.response.rect, t.color.border_strong);

    let dock_height = exact_rows_height();
    let plot_height = (ui.available_height() - dock_height - VIEWER_STAGE_STATUS_HEIGHT).max(80.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), plot_height),
        Layout::top_down(Align::Min),
        |ui| match availability {
            Ok(viewer) => {
                app.state.ui.results.viewer = viewer;
                let interaction = match app.state.workbench.visualization_studio.tool {
                    ViewerTool::Select => crate::ui::plot::InteractionMode::Select,
                    ViewerTool::Pan => crate::ui::plot::InteractionMode::Pan,
                    ViewerTool::Zoom => crate::ui::plot::InteractionMode::Zoom,
                };
                crate::ui::plot::set_interaction_mode(ui.ctx(), interaction);
                result_document::show_embedded(ui, app);
                capture_active_link_state(app);
                paint_visualization_markers(ui, app);
                crate::ui::plot::set_interaction_mode(
                    ui.ctx(),
                    crate::ui::plot::InteractionMode::All,
                );
            }
            Err(reason) => unavailable_viewer(ui, definition, &reason),
        },
    );
    viewer_stage_status(ui, app, compatible);
    exact_data_dock(ui, app);
}

fn paint_visualization_markers(ui: &Ui, app: &RSpiceApp) {
    let Some(well) = app.state.ui.results.well_rect else {
        return;
    };
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let Some(analysis_index) = app.state.simulation.active_analysis_idx else {
        return;
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return;
    };
    let (source_min, source_max) = waveform.x_range();
    let viewer = app.state.ui.results.viewer;
    let (x_min, x_max) = app
        .state
        .ui
        .results
        .plot_view(viewer, analysis_index)
        .x
        .unwrap_or((source_min, source_max));
    if !x_min.is_finite() || !x_max.is_finite() || x_min >= x_max {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let plot = well.shrink2(vec2(28.0, 22.0));
    for marker in app
        .state
        .workbench
        .visualization_studio
        .markers
        .iter()
        .filter(|marker| {
            marker.dataset_id == run.dataset_id && marker.analysis_sequence == analysis.id
        })
    {
        let fraction = ((marker.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        ui.painter()
            .vline(x, plot.y_range(), Stroke::new(1.0, t.color.accent));
        ui.painter().text(
            egui::pos2(x + 4.0, plot.top() + 3.0),
            egui::Align2::LEFT_TOP,
            &marker.label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.accent,
        );
    }
    for annotation in app
        .state
        .workbench
        .visualization_studio
        .annotations
        .iter()
        .filter(|annotation| {
            annotation.dataset_id == run.dataset_id && annotation.analysis_sequence == analysis.id
        })
    {
        let fraction = ((annotation.x - x_min) / (x_max - x_min)).clamp(0.0, 1.0) as f32;
        let x = egui::lerp(plot.x_range(), fraction);
        let anchor = egui::pos2(x, plot.bottom() - 8.0);
        ui.painter().circle_filled(anchor, 3.0, t.color.info);
        ui.painter().text(
            anchor + vec2(5.0, -1.0),
            egui::Align2::LEFT_CENTER,
            format!("NOTE-{}", annotation.id),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.info,
        );
    }
}

fn active_binding_error(app: &RSpiceApp) -> Option<String> {
    let studio = &app.state.workbench.visualization_studio;
    let active = studio.active_pane?;
    let pane = studio.panes.iter().find(|pane| pane.id == active)?;
    let Some(run) = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == pane.dataset_id)
    else {
        return Some(format!(
            "Bound dataset {} is no longer retained; the pane was not retargeted",
            short_dataset(pane.dataset_id)
        ));
    };
    (!run
        .analyses
        .iter()
        .any(|analysis| analysis.id == pane.analysis_sequence))
    .then(|| {
        format!(
            "Bound analysis {} is no longer retained in dataset {}; the pane was not retargeted",
            pane.analysis_sequence,
            short_dataset(pane.dataset_id)
        )
    })
}

fn unavailable_viewer(ui: &mut Ui, definition: Option<&ViewerDocumentDefinition>, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.allocate_ui_at_rect(rect.shrink(24.0), |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                WorkbenchIcon::Warning.paint(ui.painter(), icon, t.color.warn);
                ui.label(
                    RichText::new(format!(
                        "{} unavailable",
                        definition.map_or("Viewer", |meta| meta.title)
                    ))
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    RichText::new(reason)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.label(
                    RichText::new("No fallback viewer or fabricated data was substituted.")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
            });
        });
    });
}

fn viewer_stage_status(ui: &mut Ui, app: &RSpiceApp, compatible: bool) {
    let t = Tokens::get(ui.ctx());
    let status = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                VIEWER_STAGE_STATUS_HEIGHT,
                VIEWER_STAGE_STATUS_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Document  VIS-{:04} · revision {}",
                        1, app.state.workbench.visualization_studio.revision
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Source  immutable result samples")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.separator();
                ui.label(
                    RichText::new("Interpolation  source exact on dock queries")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(if compatible {
                            "COMPATIBLE-RUNTIME"
                        } else {
                            "VIEWER-UNAVAILABLE"
                        })
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if compatible {
                            t.color.ok
                        } else {
                            t.color.warn
                        }),
                    );
                });
            });
        });
    paint_top_rule(ui, status.response.rect, t.color.border_strong);
}

const fn exact_rows_height() -> f32 {
    EXACT_DATA_DOCK_HEIGHT
}

fn exact_data_dock(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let rows = exact_source_rows(&app.state);
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(8.0)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            panel_heading(
                ui,
                "Exact-data dock",
                &format!("{} source rows · no display interpolation", rows.len()),
            );
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), EXACT_DATA_TABLE_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ScrollArea::both()
                        .id_salt("visualization.exact-data")
                        .show(ui, |ui| {
                            Grid::new("visualization.exact-data.grid")
                                .num_columns(5)
                                .striped(true)
                                .spacing(vec2(14.0, 5.0))
                                .show(ui, |ui| {
                                    table_header(ui, "Binding");
                                    table_header(ui, "Stable row");
                                    table_header(ui, "Typed coordinate");
                                    table_header(ui, "Exact f64 value");
                                    table_header(ui, "Origin");
                                    ui.end_row();
                                    if rows.is_empty() {
                                        ui.label(
                                            RichText::new("No exact source row is available")
                                                .color(t.color.warn),
                                        );
                                        for _ in 0..4 {
                                            ui.label("—");
                                        }
                                        ui.end_row();
                                    }
                                    for row in rows {
                                        ui.monospace(row.binding);
                                        ui.monospace(row.stable_row);
                                        ui.monospace(row.coordinate);
                                        ui.monospace(row.value);
                                        ui.label(row.origin);
                                        ui.end_row();
                                    }
                                });
                        });
                },
            );
        });
}

struct ExactSourceRow {
    binding: String,
    stable_row: String,
    coordinate: String,
    value: String,
    origin: String,
}

fn exact_source_rows(state: &AppState) -> Vec<ExactSourceRow> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let Some(analysis_index) = state.simulation.active_analysis_idx else {
        return Vec::new();
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return Vec::new();
    };
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return Vec::new();
    };
    let count = waveform.x.len().min(waveform.y.len());
    if count == 0 {
        return Vec::new();
    }
    let mut indices = vec![0, count / 2, count - 1];
    for cursor in [state.ui.results.cursors.a, state.ui.results.cursors.b]
        .into_iter()
        .flatten()
    {
        if let Some(index) = waveform.x[..count]
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_finite())
            .min_by(|(_, left), (_, right)| {
                (*left - cursor).abs().total_cmp(&(*right - cursor).abs())
            })
            .map(|(index, _)| index)
        {
            indices.push(index);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    indices
        .into_iter()
        .take(5)
        .map(|index| ExactSourceRow {
            binding: short_dataset(run.dataset_id),
            stable_row: format!("{}:{index}", analysis.id),
            coordinate: format!("x={:.17e}", waveform.x[index]),
            value: format!("{:.17e}", waveform.y[index]),
            origin: waveform.name.clone(),
        })
        .collect()
}

fn short_dataset(id: DatasetId) -> String {
    id.to_string().chars().take(8).collect()
}

struct ResultEntityRow {
    identity: String,
    kind: &'static str,
    binding: String,
    state: String,
}

fn result_entity_rows(state: &AppState) -> Vec<ResultEntityRow> {
    let mut rows = Vec::new();
    if let (Some(run), Some(analysis)) = (
        state.simulation.active_run(),
        state.simulation.active_analysis(),
    ) {
        let dataset = short_dataset(run.dataset_id);
        if let Some(waveform) = analysis.waveforms.first() {
            rows.push(ResultEntityRow {
                identity: format!("axis:{}:x", analysis.id),
                kind: "axis",
                binding: format!("{} · source X coordinate", dataset),
                state: format!("{} exact rows", waveform.x.len()),
            });
        }
        for (index, waveform) in analysis.waveforms.iter().enumerate() {
            rows.push(ResultEntityRow {
                identity: format!("trace:{}:{index}", analysis.id),
                kind: "trace",
                binding: format!("{} · {}", dataset, waveform.name),
                state: if waveform.visible {
                    "visible · source exact".to_owned()
                } else {
                    "hidden · retained".to_owned()
                },
            });
        }
    }
    for (label, coordinate) in [
        ("A", state.ui.results.cursors.a),
        ("B", state.ui.results.cursors.b),
    ] {
        if let Some(coordinate) = coordinate {
            rows.push(ResultEntityRow {
                identity: format!("cursor:{label}"),
                kind: "cursor",
                binding: format!("x={coordinate:.17e}"),
                state: "nearest source row".to_owned(),
            });
        }
    }
    for marker in &state.workbench.visualization_studio.markers {
        rows.push(ResultEntityRow {
            identity: format!("marker:{}", marker.id),
            kind: "marker",
            binding: format!(
                "{} · {}[{}]",
                short_dataset(marker.dataset_id),
                marker.waveform_name,
                marker.sample_index
            ),
            state: marker.label.clone(),
        });
    }
    for measurement in &state.workbench.visualization_studio.measurements {
        rows.push(ResultEntityRow {
            identity: format!("measurement:{}", measurement.id),
            kind: "measurement",
            binding: measurement.expression.clone(),
            state: format!("{:.17e}", measurement.value),
        });
    }
    for annotation in &state.workbench.visualization_studio.annotations {
        rows.push(ResultEntityRow {
            identity: format!("annotation:{}", annotation.id),
            kind: "annotation",
            binding: format!(
                "{} · analysis {} · x={:.9e}",
                short_dataset(annotation.dataset_id),
                annotation.analysis_sequence,
                annotation.x
            ),
            state: annotation.text.clone(),
        });
    }
    rows
}

fn fixed_table_row<const N: usize>(
    ui: &mut Ui,
    fractions: [f32; N],
    cells: [&str; N],
    header: bool,
    minimum_width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 28.0 };
    let width = ui.available_width().max(minimum_width);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    let mut left = rect.left();
    for index in 0..N {
        let right = if index + 1 == N {
            rect.right()
        } else {
            left + rect.width() * fractions[index]
        };
        let cell = Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        if index + 1 < N {
            ui.painter().vline(
                right - 0.5,
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter().with_clip_rect(cell).text(
            egui::pos2(cell.left() + 8.0, cell.center().y),
            Align2::LEFT_CENTER,
            cells[index],
            font.clone(),
            color,
        );
        left = right;
    }
}

fn result_entity_table(ui: &mut Ui, rows: &[ResultEntityRow]) {
    const FRACTIONS: [f32; 4] = [0.23, 0.16, 0.37, 0.24];
    ScrollArea::both()
        .id_salt("visualization.result-entities")
        .max_height(196.0)
        .show(ui, |ui| {
            fixed_table_row(
                ui,
                FRACTIONS,
                ["IDENTITY", "TYPE", "BINDING / DEFINITION", "STATE"],
                true,
                520.0,
            );
            if rows.is_empty() {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    ["—", "none", "No versioned result entities", "empty"],
                    false,
                    520.0,
                );
            }
            for row in rows {
                fixed_table_row(
                    ui,
                    FRACTIONS,
                    [&row.identity, row.kind, &row.binding, &row.state],
                    false,
                    520.0,
                );
            }
        });
}

fn viewer_inspector(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let entities = result_entity_rows(&app.state);
    panel_heading(ui, "Versioned result entities", &entities.len().to_string());
    ScrollArea::vertical()
        .id_salt("visualization.entity-inspector")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            result_entity_table(ui, &entities);

            separator(ui, t.color.border);
            panel_heading(ui, "Comparison receipt", "none");
            ui.label(
                RichText::new(
                    "Select explicit dataset alignment, units, interpolation, resampling, extrapolation, and precision before comparing immutable sources.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
            );
            empty_note(
                ui,
                "No comparison receipt has been created for the selected immutable datasets.",
            );

            separator(ui, t.color.border);
            let operation = app.state.workbench.visualization_studio.operation_state;
            panel_heading(ui, "Progressive operation", operation_label(operation));
            ui.label(
                RichText::new("Exact source-sample integrity scan")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            let studio = &app.state.workbench.visualization_studio;
            let progress = if studio.operation_total == 0 {
                0.0
            } else {
                studio.operation_processed as f32 / studio.operation_total as f32
            };
            ui.add(egui::ProgressBar::new(progress).show_percentage());
            if operation == OperationState::Completed {
                ui.monospace(format!(
                    "{} samples · checksum {:016x}",
                    studio.operation_total, studio.operation_checksum
                ));
            }
            let source_available = source_integrity_scan_binding(&app.state).is_some();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(
                        source_available
                            && matches!(
                                operation,
                                OperationState::NotStarted | OperationState::Completed
                            ),
                        egui::Button::new("Start"),
                    )
                    .on_disabled_hover_text(
                        "Start requires a selected retained analysis with exact source samples",
                    )
                    .clicked()
                {
                    start_source_integrity_scan(app);
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Advance"),
                    )
                    .on_disabled_hover_text("Advance requires a running selected operation")
                    .clicked()
                    && let Err(error) = advance_source_integrity_scan(app)
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Running,
                        egui::Button::new("Cancel"),
                    )
                    .on_disabled_hover_text("Cancel requires a running selected operation")
                    .clicked()
                {
                    app.state.workbench.visualization_studio.operation_state =
                        OperationState::Cancelled;
                }
                if ui
                    .add_enabled(
                        operation == OperationState::Cancelled,
                        egui::Button::new("Recover"),
                    )
                    .on_disabled_hover_text("Recover requires a cancelled operation")
                    .clicked()
                    && let Err(error) = recover_source_integrity_scan(app)
                {
                    app.state.push_user_message(ConsoleMessage::error(error));
                }
            });
            if compact {
                ui.add_space(16.0);
            }
        });
}

const fn operation_label(state: OperationState) -> &'static str {
    match state {
        OperationState::NotStarted => "not started",
        OperationState::Running => "running",
        OperationState::Cancelled => "cancelled",
        OperationState::Completed => "completed",
    }
}

fn source_integrity_scan_binding(state: &AppState) -> Option<(DatasetId, u64, usize)> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let total = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum::<usize>();
    (total > 0).then_some((run.dataset_id, analysis.id, total))
}

fn start_source_integrity_scan(app: &mut RSpiceApp) {
    let Some((dataset_id, analysis_sequence, total)) = source_integrity_scan_binding(&app.state)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact retained analysis is required before an integrity scan can start.",
        ));
        return;
    };
    let studio = &mut app.state.workbench.visualization_studio;
    studio.operation_state = OperationState::Running;
    studio.operation_dataset_id = Some(dataset_id);
    studio.operation_analysis_sequence = Some(analysis_sequence);
    studio.operation_processed = 0;
    studio.operation_total = total;
    studio.operation_checksum = 0xcbf2_9ce4_8422_2325;
}

fn advance_source_integrity_scan(app: &mut RSpiceApp) -> Result<(), String> {
    let studio = &app.state.workbench.visualization_studio;
    let dataset_id = studio
        .operation_dataset_id
        .ok_or_else(|| "The source-integrity scan has no bound dataset".to_owned())?;
    let analysis_sequence = studio
        .operation_analysis_sequence
        .ok_or_else(|| "The source-integrity scan has no bound analysis".to_owned())?;
    let processed = studio.operation_processed;
    let retained_total = studio.operation_total;
    let mut checksum = studio.operation_checksum;

    let run = app
        .state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The integrity-scan dataset is no longer retained".to_owned())?;
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_sequence)
        .ok_or_else(|| "The integrity-scan analysis is no longer retained".to_owned())?;
    let actual_total = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum::<usize>();
    if actual_total != retained_total {
        return Err(
            "The immutable source sample count changed during the integrity scan".to_owned(),
        );
    }
    if processed >= retained_total {
        return Err("The source-integrity scan is already complete".to_owned());
    }

    let chunk = retained_total.div_ceil(3).max(1);
    let next = processed.saturating_add(chunk).min(retained_total);
    for (&x, &y) in analysis
        .waveforms
        .iter()
        .flat_map(|waveform| waveform.x.iter().zip(waveform.y.iter()))
        .skip(processed)
        .take(next - processed)
    {
        checksum ^= x.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
        checksum ^= y.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
    }

    let studio = &mut app.state.workbench.visualization_studio;
    studio.operation_processed = next;
    studio.operation_checksum = checksum;
    if next == retained_total {
        studio.operation_state = OperationState::Completed;
    }
    Ok(())
}

fn recover_source_integrity_scan(app: &mut RSpiceApp) -> Result<(), String> {
    let studio = &app.state.workbench.visualization_studio;
    let dataset_id = studio
        .operation_dataset_id
        .ok_or_else(|| "The cancelled integrity scan has no bound dataset".to_owned())?;
    let analysis_sequence = studio
        .operation_analysis_sequence
        .ok_or_else(|| "The cancelled integrity scan has no bound analysis".to_owned())?;
    if studio.operation_processed >= studio.operation_total {
        return Err("A completed integrity scan cannot be recovered".to_owned());
    }
    let binding_exists = app.state.simulation.runs.iter().any(|run| {
        run.dataset_id == dataset_id
            && run
                .analyses
                .iter()
                .any(|analysis| analysis.id == analysis_sequence)
    });
    if !binding_exists {
        return Err("The cancelled integrity scan's immutable source is unavailable".to_owned());
    }
    app.state.workbench.visualization_studio.operation_state = OperationState::Running;
    Ok(())
}

fn actions_sheet(ui: &mut Ui, app: &mut RSpiceApp) {
    section_heading(ui, app.state.workbench.visualization_studio.section);
    ScrollArea::vertical()
        .id_salt("visualization.actions-sheet")
        .show(ui, |ui| {
            let actions = [
                ("Add visualization pane", VisualizationDock::AddPane),
                ("Trace manager", VisualizationDock::TraceManager),
                ("Cursor manager", VisualizationDock::CursorManager),
                ("Document properties", VisualizationDock::DocumentProperties),
                ("Export document", VisualizationDock::Export),
            ];
            for (label, dock) in actions {
                if ui
                    .add_sized([ui.available_width(), 44.0], egui::Button::new(label))
                    .clicked()
                {
                    open_dock(app, dock);
                    app.state.workbench.visualization_studio.touch_pane =
                        VisualizationTouchPane::Stage;
                }
            }
        });
}

fn add_cursor_at_midpoint(app: &mut RSpiceApp) {
    let midpoint = app
        .state
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.waveforms.iter().find(|waveform| waveform.visible))
        .and_then(|waveform| {
            let count = waveform.x.len().min(waveform.y.len());
            (count > 0).then(|| waveform.x[count / 2])
        });
    if let Some(x) = midpoint {
        app.state.ui.results.cursors.place(x);
        app.state.ui.results.cursor_strip = app.state.simulation.active_analysis_idx;
    } else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact source waveform is required before a cursor can be placed.",
        ));
    }
}

fn fit_active_view(app: &mut RSpiceApp) {
    let viewer = app.state.ui.results.viewer;
    let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
    match app.state.workbench.visualization_studio.autoscale {
        VisualizationAutoscale::RobustVisible => {
            app.state.ui.results.reset_plot_view(viewer, strip);
        }
        VisualizationAutoscale::ExactExtrema if viewer == ResultViewer::Waves => {
            let extrema = app.state.simulation.active_analysis().and_then(|analysis| {
                let mut x_min = f64::INFINITY;
                let mut x_max = f64::NEG_INFINITY;
                let mut y_min = f64::INFINITY;
                let mut y_max = f64::NEG_INFINITY;
                for waveform in analysis
                    .waveforms
                    .iter()
                    .filter(|waveform| waveform.visible)
                {
                    for &x in waveform.x.iter() {
                        if x.is_finite() {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                    }
                    for &y in waveform.y.iter() {
                        if y.is_finite() {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }
                    }
                }
                (x_min.is_finite() && x_max.is_finite() && y_min.is_finite() && y_max.is_finite())
                    .then_some((
                        nondegenerate_range(x_min, x_max),
                        nondegenerate_range(y_min, y_max),
                    ))
            });
            if let Some((x, y)) = extrema {
                let view = app.state.ui.results.plot_view_mut(viewer, strip);
                view.x = Some(x);
                view.y = Some(y);
            } else {
                app.state.push_user_message(ConsoleMessage::warning(
                    "Exact-extrema fitting requires at least one visible finite waveform.",
                ));
                return;
            }
        }
        VisualizationAutoscale::ExactExtrema => {
            app.state.push_user_message(ConsoleMessage::warning(
                "Exact-extrema fitting is not implemented for the active renderer.",
            ));
            return;
        }
        VisualizationAutoscale::SpecificationBounds => {
            app.state.push_user_message(ConsoleMessage::warning(
                "Specification-bound fitting is unavailable because no quantity-mapped axis limit is configured.",
            ));
            return;
        }
    }
    app.state.workbench.visualization_studio.zoom = 1.0;
}

fn nondegenerate_range(minimum: f64, maximum: f64) -> (f64, f64) {
    if minimum < maximum {
        return (minimum, maximum);
    }
    let padding = (minimum.abs() * 1.0e-9).max(1.0e-12);
    (minimum - padding, maximum + padding)
}

fn zoom_active(app: &mut RSpiceApp, factor: f32) {
    let next_zoom = (app.state.workbench.visualization_studio.zoom * factor).clamp(0.25, 8.0);
    let Some(analysis) = app.state.simulation.active_analysis() else {
        return;
    };
    let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
        return;
    };
    let (x_min, x_max) = waveform.x_range();
    let (y_min, y_max) = waveform.y_range();
    if !x_min.is_finite() || !x_max.is_finite() || !y_min.is_finite() || !y_max.is_finite() {
        return;
    }
    let scale = f64::from(1.0 / next_zoom);
    let x_mid = (x_min + x_max) * 0.5;
    let y_mid = (y_min + y_max) * 0.5;
    let viewer = app.state.ui.results.viewer;
    let strip = app.state.simulation.active_analysis_idx.unwrap_or_default();
    let view = app.state.ui.results.plot_view_mut(viewer, strip);
    view.x = Some((
        x_mid - (x_max - x_min) * 0.5 * scale,
        x_mid + (x_max - x_min) * 0.5 * scale,
    ));
    view.y = Some((
        y_mid - (y_max - y_min) * 0.5 * scale,
        y_mid + (y_max - y_min) * 0.5 * scale,
    ));
    app.state.workbench.visualization_studio.zoom = next_zoom;
}

fn add_marker_at_midpoint(app: &mut RSpiceApp) {
    let Some((dataset_id, analysis_sequence, waveform_name, sample_index, x, y)) =
        source_midpoint(&app.state)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "An exact source waveform is required before a marker can be created.",
        ));
        return;
    };
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization marker identity space is exhausted".to_owned())?;
        studio.markers.push(VisualizationMarker {
            id,
            dataset_id,
            analysis_sequence,
            waveform_name,
            sample_index,
            x,
            y,
            label: format!("M{id}"),
        });
        Ok(())
    });
    report_visualization_commit(app, result);
}

fn source_midpoint(state: &AppState) -> Option<(DatasetId, u64, String, usize, f64, f64)> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.visible)?;
    let count = waveform.x.len().min(waveform.y.len());
    if count == 0 {
        return None;
    }
    let index = count / 2;
    Some((
        run.dataset_id,
        analysis.id,
        waveform.name.clone(),
        index,
        waveform.x[index],
        waveform.y[index],
    ))
}

fn apply_lod_policy(app: &mut RSpiceApp) {
    let policy = app.state.workbench.visualization_studio.display_lod;
    let index = match policy {
        DisplayLodPolicy::EnvelopePreserving => 0,
        DisplayLodPolicy::UniformSampling => 1,
        DisplayLodPolicy::ExactVisibleSamples => 2,
    };
    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::LargeDatasetDisplay, index)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
}

fn show_dock_if_open(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    let Some(dock) = app.state.workbench.visualization_studio.dock else {
        return;
    };
    let mut window_open = true;
    let mut close_requested = false;
    let viewport_width = ui.ctx().content_rect().width();
    let dock_width = if compact {
        (viewport_width - 18.0).clamp(180.0, 520.0)
    } else {
        460.0
    };
    egui::Window::new(dock.title())
        .id(Id::new(("visualization.dock", dock as u8)))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(true)
        .default_width(dock_width)
        .min_width(if compact {
            dock_width.min(240.0)
        } else {
            300.0
        })
        .max_width(if compact { dock_width } else { 620.0 })
        .show(ui.ctx(), |ui| {
            ui.set_max_width(if compact { 520.0 } else { 620.0 });
            close_requested = dock_body(ui, app, dock);
        });
    if !window_open || close_requested {
        app.state.workbench.visualization_studio.dock = None;
    }
}

impl VisualizationDock {
    const fn title(self) -> &'static str {
        match self {
            Self::AddPane => "Add visualization pane",
            Self::TraceManager => "Trace and family manager",
            Self::CursorManager => "Cursor and marker manager",
            Self::DocumentProperties => "Document properties",
            Self::ReorderPanes => "Reorder visualization panes",
            Self::LinkGroups => "Axis and cursor link groups",
            Self::PageEditor => "Assign pane to report page",
            Self::Measurement => "Create result measurement",
            Self::Annotation => "Create result annotation",
            Self::FamilySlice => "Family slicing and pivot",
            Self::FamilyEncoding => "Family visual encoding",
            Self::FamilyFilter => "Advanced family filter",
            Self::Export => "Export visualization document",
        }
    }
}

fn dock_body(ui: &mut Ui, app: &mut RSpiceApp, dock: VisualizationDock) -> bool {
    match dock {
        VisualizationDock::AddPane => add_pane_dock(ui, app),
        VisualizationDock::TraceManager => trace_manager_dock(ui, app),
        VisualizationDock::CursorManager => cursor_manager_dock(ui, app),
        VisualizationDock::DocumentProperties => properties_dock(ui, app),
        VisualizationDock::ReorderPanes => reorder_panes_dock(ui, app),
        VisualizationDock::LinkGroups => link_groups_dock(ui, app),
        VisualizationDock::PageEditor => page_editor_dock(ui, app),
        VisualizationDock::Measurement => measurement_dock(ui, app),
        VisualizationDock::Annotation => annotation_dock(ui, app),
        VisualizationDock::FamilySlice => family_slice_dock(ui, app),
        VisualizationDock::FamilyEncoding => family_encoding_dock(ui, app),
        VisualizationDock::FamilyFilter => family_filter_dock(ui, app),
        VisualizationDock::Export => export_dock(ui, app),
    }
}

fn normalize_add_pane_draft(state: &mut AppState) {
    let draft_dataset = state.workbench.visualization_studio.draft_dataset_id;
    let draft_analysis = state.workbench.visualization_studio.draft_analysis_sequence;
    let draft_is_valid =
        draft_dataset
            .zip(draft_analysis)
            .is_some_and(|(dataset_id, analysis_sequence)| {
                state.simulation.runs.iter().any(|run| {
                    run.dataset_id == dataset_id
                        && run
                            .analyses
                            .iter()
                            .any(|analysis| analysis.id == analysis_sequence)
                })
            });
    if draft_is_valid {
        return;
    }
    let fallback = state
        .simulation
        .active_run()
        .and_then(|run| {
            state
                .simulation
                .active_analysis()
                .map(|analysis| (run.dataset_id, analysis.id))
        })
        .or_else(|| {
            state.simulation.runs.iter().find_map(|run| {
                run.analyses
                    .first()
                    .map(|analysis| (run.dataset_id, analysis.id))
            })
        });
    state.workbench.visualization_studio.draft_dataset_id = fallback.map(|binding| binding.0);
    state.workbench.visualization_studio.draft_analysis_sequence =
        fallback.map(|binding| binding.1);
}

fn selected_draft_analysis(state: &AppState) -> Option<&crate::state::AnalysisResult> {
    let studio = &state.workbench.visualization_studio;
    let dataset_id = studio.draft_dataset_id?;
    let analysis_sequence = studio.draft_analysis_sequence?;
    state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)?
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_sequence)
}

fn add_pane_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Create a compatible viewer pane without disturbing existing link groups.",
    );
    normalize_add_pane_draft(&mut app.state);
    let draft_dataset = app.state.workbench.visualization_studio.draft_dataset_id;
    let draft_analysis = app
        .state
        .workbench
        .visualization_studio
        .draft_analysis_sequence;
    let options = NATIVE_VIEWERS.map(|viewer| {
        let definition = viewer_document(viewer_document_id(viewer));
        let availability = definition
            .ok_or_else(|| "Viewer document is not registered".to_owned())
            .and_then(|definition| {
                resolved_viewer_availability_for_binding(
                    &app.state,
                    definition,
                    draft_dataset,
                    draft_analysis,
                )
            });
        (viewer, availability)
    });
    egui::ComboBox::from_label("Viewer")
        .selected_text(
            app.state
                .workbench
                .visualization_studio
                .draft_viewer
                .label(),
        )
        .show_ui(ui, |ui| {
            for (viewer, availability) in &options {
                let response = ui.add_enabled_ui(availability.is_ok(), |ui| {
                    ui.selectable_value(
                        &mut app.state.workbench.visualization_studio.draft_viewer,
                        *viewer,
                        viewer.label(),
                    )
                });
                if let Err(reason) = availability {
                    response.response.on_hover_text(reason);
                }
            }
        });

    let selected_dataset_text = draft_dataset
        .and_then(|dataset_id| {
            app.state
                .simulation
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset_id)
        })
        .map_or_else(
            || "Select retained dataset".to_owned(),
            |run| format!("{} · {}", run.label, short_dataset(run.dataset_id)),
        );
    egui::ComboBox::from_label("Dataset")
        .selected_text(selected_dataset_text)
        .show_ui(ui, |ui| {
            let rows: Vec<_> = app
                .state
                .simulation
                .runs
                .iter()
                .map(|run| {
                    (
                        run.dataset_id,
                        run.label.clone(),
                        run.analyses.first().map(|a| a.id),
                    )
                })
                .collect();
            for (dataset_id, label, first_analysis) in rows {
                if ui
                    .selectable_value(
                        &mut app.state.workbench.visualization_studio.draft_dataset_id,
                        Some(dataset_id),
                        format!("{} · {}", label, short_dataset(dataset_id)),
                    )
                    .clicked()
                {
                    app.state
                        .workbench
                        .visualization_studio
                        .draft_analysis_sequence = first_analysis;
                }
            }
        });

    let draft_dataset = app.state.workbench.visualization_studio.draft_dataset_id;
    let selected_analysis_text = selected_draft_analysis(&app.state).map_or_else(
        || "Select retained analysis".to_owned(),
        |analysis| format!("{} · {}", analysis.label, analysis.id),
    );
    ui.add_enabled_ui(draft_dataset.is_some(), |ui| {
        egui::ComboBox::from_label("Analysis")
            .selected_text(selected_analysis_text)
            .show_ui(ui, |ui| {
                let rows: Vec<_> = draft_dataset
                    .and_then(|dataset_id| {
                        app.state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)
                    })
                    .map(|run| {
                        run.analyses
                            .iter()
                            .map(|analysis| {
                                (
                                    analysis.id,
                                    analysis.label.clone(),
                                    analysis_manifest_id(analysis.analysis_type),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                for (analysis_id, label, kind) in rows {
                    ui.selectable_value(
                        &mut app
                            .state
                            .workbench
                            .visualization_studio
                            .draft_analysis_sequence,
                        Some(analysis_id),
                        format!("{label} · {kind} · {analysis_id}"),
                    );
                }
            });
    });

    let placement = app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement;
    egui::ComboBox::from_label("Placement")
        .selected_text(placement.label())
        .show_ui(ui, |ui| {
            for placement in VisualizationPanePlacement::ALL {
                ui.selectable_value(
                    &mut app
                        .state
                        .workbench
                        .visualization_studio
                        .draft_pane_placement,
                    placement,
                    placement.label(),
                );
            }
        });
    if app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement
        == VisualizationPanePlacement::NewWorksheetPage
    {
        ui.label("New page title");
        ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_page_title);
    }

    let selected_viewer = app.state.workbench.visualization_studio.draft_viewer;
    let selected_compatibility = options
        .iter()
        .find_map(|(viewer, availability)| (*viewer == selected_viewer).then_some(availability));
    let page_valid = app
        .state
        .workbench
        .visualization_studio
        .draft_pane_placement
        != VisualizationPanePlacement::NewWorksheetPage
        || !app
            .state
            .workbench
            .visualization_studio
            .draft_page_title
            .trim()
            .is_empty();
    let enabled = selected_compatibility.is_some_and(Result::is_ok) && page_valid;
    ui.add_space(10.0);
    let add = ui
        .add_enabled(enabled, egui::Button::new("Add pane"))
        .on_disabled_hover_text(
            selected_compatibility
                .and_then(|result| result.as_ref().err())
                .map_or(
                    "A retained compatible result analysis is required",
                    String::as_str,
                ),
        )
        .clicked();
    if add {
        let viewer = app.state.workbench.visualization_studio.draft_viewer;
        let dataset_id = app
            .state
            .workbench
            .visualization_studio
            .draft_dataset_id
            .expect("enabled add-pane action has a retained dataset");
        let analysis_sequence = app
            .state
            .workbench
            .visualization_studio
            .draft_analysis_sequence
            .expect("enabled add-pane action has a retained analysis");
        let placement = app
            .state
            .workbench
            .visualization_studio
            .draft_pane_placement;
        let page_title = app
            .state
            .workbench
            .visualization_studio
            .draft_page_title
            .trim()
            .to_owned();
        add_viewer_pane_bound(
            app,
            viewer_document_id(viewer),
            viewer,
            dataset_id,
            analysis_sequence,
            placement,
            page_title,
        );
    }
    add
}

fn trace_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SIGNALS · EXPRESSIONS",
        "Show or hide native source traces and create a derived expression for the selected analysis.",
    );
    let dataset_id = app.state.workbench.visualization_studio.draft_trace_dataset;
    let analysis_id = app
        .state
        .workbench
        .visualization_studio
        .draft_trace_analysis;
    let binding_exists = dataset_id
        .zip(analysis_id)
        .is_some_and(|(dataset, analysis)| {
            app.state.simulation.runs.iter().any(|run| {
                run.dataset_id == dataset
                    && run
                        .analyses
                        .iter()
                        .any(|candidate| candidate.id == analysis)
            })
        });
    if app
        .state
        .workbench
        .visualization_studio
        .draft_trace_visibility
        .is_empty()
    {
        empty_note(ui, "No active analysis exposes traces.");
    } else {
        for (waveform, visible) in &mut app
            .state
            .workbench
            .visualization_studio
            .draft_trace_visibility
        {
            ui.checkbox(visible, waveform.as_str());
        }
    }
    ui.add_space(8.0);
    if Button::new("Add expression…").show(ui).clicked() {
        open_dock(app, VisualizationDock::Measurement);
        return false;
    }
    let apply = ui
        .add_enabled(binding_exists, egui::Button::new("Apply trace changes"))
        .on_disabled_hover_text(
            "The immutable analysis bound when this dialog opened is unavailable",
        )
        .clicked();
    if apply {
        let visibility = app
            .state
            .workbench
            .visualization_studio
            .draft_trace_visibility
            .clone();
        if let Some((dataset_id, analysis_id)) = dataset_id.zip(analysis_id)
            && let Some(analysis) = app
                .state
                .simulation
                .runs
                .iter_mut()
                .find(|run| run.dataset_id == dataset_id)
                .and_then(|run| {
                    run.analyses
                        .iter_mut()
                        .find(|analysis| analysis.id == analysis_id)
                })
        {
            for waveform in &mut analysis.waveforms {
                if let Some((_, visible)) =
                    visibility.iter().find(|(name, _)| name == &waveform.name)
                {
                    waveform.visible = *visible;
                }
            }
            commit_visualization_revision(app);
        }
    }
    apply
}

fn cursor_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT VALUES",
        "Manage linked cursors, source-sample markers, and exact-value behavior.",
    );
    ui.checkbox(
        &mut app.state.ui.results.linked_cursors,
        "Link A/B cursors across compatible panes",
    );
    property_row(
        ui,
        "Cursor A",
        &app.state
            .ui
            .results
            .cursors
            .a
            .map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    property_row(
        ui,
        "Cursor B",
        &app.state
            .ui
            .results
            .cursors
            .b
            .map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    ui.horizontal_wrapped(|ui| {
        if Button::new("Place next at midpoint").show(ui).clicked() {
            add_cursor_at_midpoint(app);
        }
        if Button::new("Add exact marker").show(ui).clicked() {
            add_marker_at_midpoint(app);
        }
        if Button::new("Clear cursors").show(ui).clicked() {
            app.state.ui.results.clear_cursors();
        }
        if Button::new("Clear markers").show(ui).clicked() {
            let result = app.state.workbench.visualization_studio.transact(|studio| {
                studio.markers.clear();
                Ok(())
            });
            report_visualization_commit(app, result);
        }
    });
    Button::new("Done").show(ui).clicked()
}

fn properties_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULT DOCUMENT · PRESENTATION POLICY",
        "Edit the current worksheet's retained display properties.",
    );
    let current_significant_digits = app.state.workbench.visualization_studio.significant_digits;
    let significant_digits = app
        .state
        .workbench
        .visualization_studio
        .draft_significant_digits
        .get_or_insert(current_significant_digits);
    ui.add(egui::Slider::new(significant_digits, 3..=17).text("Significant digits"));
    property_row(ui, "Engineering grid", "Renderer-managed major grid");
    property_row(ui, "Legend placement", "Inside plot · compact");
    let current_phase_continuous = app.state.ui.results.phase_continuous;
    let phase_continuous = app
        .state
        .workbench
        .visualization_studio
        .draft_phase_continuous
        .get_or_insert(current_phase_continuous);
    ui.checkbox(phase_continuous, "Continuous (unwrapped) phase display");
    let save = Button::new("Save properties").accent().show(ui).clicked();
    if save {
        let significant_digits = app
            .state
            .workbench
            .visualization_studio
            .draft_significant_digits;
        let phase_continuous = app
            .state
            .workbench
            .visualization_studio
            .draft_phase_continuous;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            if let Some(significant_digits) = significant_digits {
                studio.significant_digits = significant_digits;
            }
            Ok(())
        });
        if report_visualization_commit(app, result)
            && let Some(phase_continuous) = phase_continuous
        {
            app.state.ui.results.phase_continuous = phase_continuous;
        }
    }
    save
}

fn reorder_panes_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Move panes while preserving their stable identity, traces, and link groups.",
    );
    let active = app.state.workbench.visualization_studio.active_pane;
    let index = active.and_then(|id| {
        app.state
            .workbench
            .visualization_studio
            .draft_pane_order
            .iter()
            .position(|pane_id| *pane_id == id)
    });
    property_row(
        ui,
        "Selected pane",
        &active.map_or_else(|| "none".to_owned(), |id| format!("Pane {id:02}")),
    );
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                index.is_some_and(|index| index > 0),
                egui::Button::new("Move before"),
            )
            .clicked()
            && let Some(index) = index
        {
            app.state
                .workbench
                .visualization_studio
                .draft_pane_order
                .swap(index, index - 1);
        }
        if ui
            .add_enabled(
                index.is_some_and(|index| {
                    index + 1
                        < app
                            .state
                            .workbench
                            .visualization_studio
                            .draft_pane_order
                            .len()
                }),
                egui::Button::new("Move after"),
            )
            .clicked()
            && let Some(index) = index
        {
            app.state
                .workbench
                .visualization_studio
                .draft_pane_order
                .swap(index, index + 1);
        }
    });
    let apply = Button::new("Apply pane order").accent().show(ui).clicked();
    if apply {
        let order = app
            .state
            .workbench
            .visualization_studio
            .draft_pane_order
            .clone();
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            studio.panes.sort_by_key(|pane| {
                order
                    .iter()
                    .position(|pane_id| *pane_id == pane.id)
                    .unwrap_or(usize::MAX)
            });
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn link_groups_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SYNCHRONIZED NAVIGATION",
        "Define which panes share X ranges and cursor positions.",
    );
    let active_pane = app.state.workbench.visualization_studio.active_pane;
    let Some(pane_id) = active_pane else {
        empty_note(ui, "Select a pane before editing link groups.");
        return Button::new("Close").show(ui).clicked();
    };
    if app.state.workbench.visualization_studio.draft_link_pane != Some(pane_id) {
        let draft = app
            .state
            .workbench
            .visualization_studio
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| {
                (
                    pane.x_link.unwrap_or_default(),
                    pane.cursor_group.unwrap_or_default(),
                )
            });
        if let Some((x_link, cursor_group)) = draft {
            app.state.workbench.visualization_studio.draft_link_pane = Some(pane_id);
            app.state.workbench.visualization_studio.draft_x_link = x_link;
            app.state.workbench.visualization_studio.draft_cursor_group = cursor_group;
        }
    }
    ui.horizontal(|ui| {
        ui.label("X range group");
        ui.add(
            egui::DragValue::new(&mut app.state.workbench.visualization_studio.draft_x_link)
                .range(0..=999),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Cursor group");
        ui.add(
            egui::DragValue::new(&mut app.state.workbench.visualization_studio.draft_cursor_group)
                .range(0..=999),
        );
    });
    let apply = Button::new("Save link groups").accent().show(ui).clicked();
    if apply {
        let x_link = app.state.workbench.visualization_studio.draft_x_link;
        let cursor_group = app.state.workbench.visualization_studio.draft_cursor_group;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let pane = studio
                .panes
                .iter_mut()
                .find(|pane| pane.id == pane_id)
                .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
            pane.x_link = (x_link != 0).then_some(x_link);
            pane.cursor_group = (cursor_group != 0).then_some(cursor_group);
            studio.applied_link_pane = None;
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn page_editor_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "REPORTING · DOCUMENT COMPOSITION",
        "Assign the active pane to a versioned worksheet or report page.",
    );
    let Some(pane_id) = app.state.workbench.visualization_studio.active_pane else {
        empty_note(ui, "Select a pane before editing its page.");
        return Button::new("Close").show(ui).clicked();
    };
    if app.state.workbench.visualization_studio.draft_page_pane != Some(pane_id) {
        let page = app
            .state
            .workbench
            .visualization_studio
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page.clone())
            .unwrap_or_default();
        app.state.workbench.visualization_studio.draft_page_pane = Some(pane_id);
        app.state.workbench.visualization_studio.draft_page = page;
    }
    ui.label("Page");
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_page);
    let page = app
        .state
        .workbench
        .visualization_studio
        .draft_page
        .trim()
        .to_owned();
    let valid = !page.is_empty();
    let apply = ui
        .add_enabled(valid, egui::Button::new("Save report page"))
        .clicked();
    if apply {
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let pane = studio
                .panes
                .iter_mut()
                .find(|pane| pane.id == pane_id)
                .ok_or_else(|| "The selected visualization pane no longer exists".to_owned())?;
            pane.page = page;
            Ok(())
        });
        report_visualization_commit(app, result);
    }
    apply
}

fn measurement_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · SCALAR EXPRESSION",
        "Evaluate and retain a finite scalar measurement against the exact selected analysis.",
    );
    ui.label("Expression");
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.draft_measurement);
    property_row(
        ui,
        "Scope",
        "Current pane · active analysis · immutable source inputs",
    );
    let definition = app
        .state
        .workbench
        .visualization_studio
        .draft_measurement
        .trim()
        .to_owned();
    let evaluation = evaluate_scalar_measurement(&app.state, &definition);
    match &evaluation {
        Ok((_, _, value)) => property_row(ui, "Validated value", &format!("{value:.17e}")),
        Err(error) if !definition.is_empty() => {
            ui.label(
                RichText::new(error)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        Err(_) => property_row(
            ui,
            "Validation",
            "Enter a scalar expression such as rms(V(out))",
        ),
    }
    let valid = evaluation.is_ok();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create measurement"))
        .clicked();
    if add {
        let (dataset_id, analysis_sequence, value) =
            evaluation.expect("enabled measurement has a validated scalar result");
        let expression = definition;
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let id = studio.allocate_identity().ok_or_else(|| {
                "Visualization measurement identity space is exhausted".to_owned()
            })?;
            studio.measurements.push(VisualizationMeasurement {
                id,
                dataset_id,
                analysis_sequence,
                expression,
                value,
            });
            Ok(())
        });
        if report_visualization_commit(app, result) {
            app.state
                .workbench
                .visualization_studio
                .draft_measurement
                .clear();
        }
    }
    add
}

fn evaluate_scalar_measurement(
    state: &AppState,
    expression: &str,
) -> Result<(DatasetId, u64, f64), String> {
    if expression.trim().is_empty() {
        return Err("A measurement expression is required".to_owned());
    }
    let run = state
        .simulation
        .active_run()
        .ok_or_else(|| "A retained result dataset is required".to_owned())?;
    let analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "A retained analysis must be selected".to_owned())?;
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let context = calculator::WaveformsContext::new(&analysis.waveforms);
    let value = match calculator::evaluator::evaluate(&parsed, &context)
        .map_err(|error| error.to_string())?
    {
        calculator::CalcValue::Scalar(value) => value,
        calculator::CalcValue::Waveform(_, _) => {
            return Err(
                "The expression produces a trace; reduce it with avg(), rms(), or another scalar function"
                    .to_owned(),
            );
        }
    };
    if !value.is_finite() {
        return Err("The measurement result is not finite".to_owned());
    }
    Ok((run.dataset_id, analysis.id, value))
}

fn annotation_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · REVIEW ANCHOR",
        "Anchor a review note to an immutable dataset and exact source coordinate.",
    );
    ui.label("Annotation text");
    ui.text_edit_multiline(&mut app.state.workbench.visualization_studio.draft_annotation);
    let anchor = source_midpoint(&app.state);
    property_row(
        ui,
        "Anchor",
        &anchor.as_ref().map_or_else(
            || "No exact source row".to_owned(),
            |(dataset, analysis, waveform, index, x, _)| {
                format!(
                    "{} · analysis {} · {}[{}] · {:.17e}",
                    short_dataset(*dataset),
                    analysis,
                    waveform,
                    index,
                    x
                )
            },
        ),
    );
    let valid = anchor.is_some()
        && !app
            .state
            .workbench
            .visualization_studio
            .draft_annotation
            .trim()
            .is_empty();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create annotation"))
        .clicked();
    if add {
        let (dataset_id, analysis_sequence, _, _, x, _) =
            anchor.expect("enabled annotation has an exact anchor");
        let text = app
            .state
            .workbench
            .visualization_studio
            .draft_annotation
            .trim()
            .to_owned();
        let result = app.state.workbench.visualization_studio.transact(|studio| {
            let id = studio
                .allocate_identity()
                .ok_or_else(|| "Visualization annotation identity space is exhausted".to_owned())?;
            studio.annotations.push(VisualizationAnnotation {
                id,
                dataset_id,
                analysis_sequence,
                x,
                text,
            });
            Ok(())
        });
        if report_visualization_commit(app, result) {
            app.state
                .workbench
                .visualization_studio
                .draft_annotation
                .clear();
        }
    }
    add
}

fn family_slice_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · N-DIMENSIONAL DATA",
        "Choose retained dataset families to overlay using stable dataset identities.",
    );
    family_overlay_controls(ui, app, "Overlay family");
    let apply = Button::new("Apply slice and pivot")
        .accent()
        .show(ui)
        .clicked();
    if apply {
        apply_family_overlay_draft(app);
    }
    apply
}

fn family_encoding_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · ACCESSIBLE TRACE FAMILIES",
        "Configure redundant visual encoding supported by the retained renderer.",
    );
    let current_phase_continuous = app.state.ui.results.phase_continuous;
    let phase_continuous = app
        .state
        .workbench
        .visualization_studio
        .draft_phase_continuous
        .get_or_insert(current_phase_continuous);
    ui.checkbox(phase_continuous, "Unwrap phase traces continuously");
    property_row(ui, "Label placement", "Inside plot · compact");
    property_row(
        ui,
        "Run encoding",
        "active run full weight · retained overlays reduced weight",
    );
    property_row(
        ui,
        "Signal encoding",
        "stable trace color · marker and label redundancy",
    );
    let apply = Button::new("Apply visual encoding")
        .accent()
        .show(ui)
        .clicked();
    if apply {
        if let Some(phase_continuous) = app
            .state
            .workbench
            .visualization_studio
            .draft_phase_continuous
        {
            app.state.ui.results.phase_continuous = phase_continuous;
        }
        commit_visualization_revision(app);
    }
    apply
}

fn family_filter_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · DATA QUERY",
        "Filter retained runs by exact label or stable dataset identity, then select overlay membership.",
    );
    ui.text_edit_singleline(&mut app.state.workbench.visualization_studio.family_query);
    family_overlay_controls(ui, app, "Include");
    let apply = Button::new("Apply filter").accent().show(ui).clicked();
    if apply {
        apply_family_overlay_draft(app);
    }
    apply
}

fn family_overlay_controls(ui: &mut Ui, app: &mut RSpiceApp, label: &str) {
    let query = app
        .state
        .workbench
        .visualization_studio
        .family_query
        .trim()
        .to_ascii_lowercase();
    let active = app.state.simulation.active_run().map(|run| run.dataset_id);
    let rows: Vec<_> = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| {
            query.is_empty()
                || run.label.to_ascii_lowercase().contains(&query)
                || run
                    .dataset_id
                    .to_string()
                    .to_ascii_lowercase()
                    .contains(&query)
        })
        .map(|run| {
            (
                run.dataset_id,
                run.label.clone(),
                app.state
                    .workbench
                    .visualization_studio
                    .draft_overlay_ids
                    .contains(&run.dataset_id),
            )
        })
        .collect();
    let mut toggles = Vec::new();
    for (dataset, run_label, overlaid) in rows {
        let mut selected = Some(dataset) == active || overlaid;
        let response = ui.add_enabled_ui(Some(dataset) != active, |ui| {
            ui.checkbox(
                &mut selected,
                format!("{label}: {run_label} · {}", short_dataset(dataset)),
            )
        });
        if response.inner.changed() {
            toggles.push(dataset);
        }
    }
    for dataset in toggles {
        let overlays = &mut app.state.workbench.visualization_studio.draft_overlay_ids;
        if let Some(index) = overlays.iter().position(|candidate| *candidate == dataset) {
            overlays.remove(index);
        } else {
            overlays.push(dataset);
        }
    }
}

fn apply_family_overlay_draft(app: &mut RSpiceApp) {
    let desired = app
        .state
        .workbench
        .visualization_studio
        .draft_overlay_ids
        .clone();
    let existing = app.state.simulation.overlay_dataset_ids.clone();
    for dataset in existing {
        if !desired.contains(&dataset) {
            app.state.simulation.toggle_dataset_overlay(dataset);
        }
    }
    for dataset in desired {
        if !app.state.simulation.is_dataset_overlaid(dataset) {
            app.state.simulation.toggle_dataset_overlay(dataset);
        }
    }
    commit_visualization_revision(app);
}

fn export_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT DATA OR RENDERED VIEW",
        "Choose a writer backed by the active immutable result dataset or viewer crop.",
    );
    let enabled = app.state.simulation.has_results();
    let mut close = false;
    if ui
        .add_enabled(enabled, egui::Button::new("Export exact engineering data…"))
        .clicked()
    {
        app.state.ui.export_csv_requested = true;
        close = true;
    }
    if ui
        .add_enabled(enabled, egui::Button::new("Export active viewer PNG…"))
        .clicked()
    {
        app.state.ui.export_png_requested = true;
        close = true;
    }
    if !enabled {
        empty_note(
            ui,
            "A completed immutable result is required before export.",
        );
    }
    close
}

fn dock_intro(ui: &mut Ui, eyebrow: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(eyebrow)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.accent),
    );
    ui.label(
        RichText::new(description)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(8.0);
}

fn dock_action(ui: &mut Ui, app: &mut RSpiceApp, label: &'static str, dock: VisualizationDock) {
    if Button::new(label).show(ui).clicked() {
        open_dock(app, dock);
    }
}

fn separator(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter()
        .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, color));
}

fn paint_top_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.top() + 0.5, Stroke::new(1.0, color));
}

fn paint_bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, Stroke::new(1.0, color));
}

fn panel_heading(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(8, 0))
        .show(ui, |ui| {
            ui.set_min_height(PANEL_HEADING_HEIGHT);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(title.to_uppercase())
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(detail)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
}

fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
}

fn empty_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

fn concept_banner(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    Frame::NONE
        .fill(t.color.accent_dim)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
                WorkbenchIcon::Info.paint(ui.painter(), icon_rect, t.color.info);
                ui.label(
                    RichText::new(text)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        });
}

fn policy_row(ui: &mut Ui, label: &str, value: &str) {
    table_header(ui, label);
    ui.label(value);
    ui.end_row();
}

fn property_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).strong());
        ui.label(value);
    });
}

fn labeled_combo(ui: &mut Ui, label: &str, selected: &str, add_contents: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(("visualization.combo", label))
            .selected_text(selected)
            .show_ui(ui, add_contents);
    });
}

fn numeric_policy(
    ui: &mut Ui,
    label: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    suffix: &str,
) {
    ui.vertical(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(value).range(range));
            ui.monospace(suffix);
        });
    });
}

#[cfg(test)]
mod integrity_scan_tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

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
}
