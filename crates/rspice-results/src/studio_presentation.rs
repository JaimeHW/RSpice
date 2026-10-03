//! Retained Visualization Studio pane bindings and display declarations.

use crate::viewer_catalog::viewer_document;
use crate::visualization_document::{
    ComparisonExecutionContract, ComparisonReceipt, FamilyPresentationPolicy, NumericTolerance,
    PageUpdatePolicy,
};
use rspice_app_types::product::{DatasetBinding, DatasetId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

use crate::result_presentation::ResultViewer;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComparisonAlignmentDraft {
    #[default]
    FirstThresholdCrossing,
    AbsoluteXAxis,
    CrossCorrelation,
}

impl ComparisonAlignmentDraft {
    pub const ALL: [Self; 3] = [
        Self::FirstThresholdCrossing,
        Self::AbsoluteXAxis,
        Self::CrossCorrelation,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstThresholdCrossing => "First threshold crossing",
            Self::AbsoluteXAxis => "Absolute X axis",
            Self::CrossCorrelation => "Cross-correlation alignment",
        }
    }
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
    pub const ALL: [Self; 3] = [
        Self::BelowSelected,
        Self::RightOfSelected,
        Self::NewWorksheetPage,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BelowSelected => "Below selected pane",
            Self::RightOfSelected => "Right of selected pane",
            Self::NewWorksheetPage => "New worksheet page",
        }
    }
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisualizationAutoscale {
    #[default]
    RobustVisible,
    ExactExtrema,
    SpecificationBounds,
}

impl VisualizationAutoscale {
    pub const ALL: [Self; 3] = [
        Self::RobustVisible,
        Self::ExactExtrema,
        Self::SpecificationBounds,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RobustVisible => "Robust visible data + 5% margin",
            Self::ExactExtrema => "Exact extrema",
            Self::SpecificationBounds => "Specification bounds",
        }
    }
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
    pub const ALL: [Self; 7] = [
        Self::Document,
        Self::Viewers,
        Self::Axes,
        Self::Families,
        Self::Measurements,
        Self::LargeData,
        Self::ExportReport,
    ];

    pub const fn label(self) -> &'static str {
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

    pub const fn title(self) -> &'static str {
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

    pub const fn description(self) -> &'static str {
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
    pub const ALL: [Self; 3] = [Self::Select, Self::Pan, Self::Zoom];

    pub const fn label(self) -> &'static str {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationReportPagePolicy {
    pub template: String,
    pub update_policy: PageUpdatePolicy,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationMeasurement {
    pub id: u64,
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub expression: String,
    pub value: f64,
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
    pub fn retained_numeric_values(&self) -> Result<usize, String> {
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
pub enum ComplexProjection {
    #[default]
    MagnitudePhase,
    RealImaginary,
}

impl ComplexProjection {
    pub const ALL: [Self; 2] = [Self::MagnitudePhase, Self::RealImaginary];

    pub const fn label(self) -> &'static str {
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
    pub const ALL: [Self; 3] = [
        Self::EnvelopePreserving,
        Self::UniformSampling,
        Self::ExactVisibleSamples,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EnvelopePreserving => "Envelope-preserving multiresolution",
            Self::UniformSampling => "Uniform display sampling",
            Self::ExactVisibleSamples => "Exact visible samples",
        }
    }
}

/// Saved Studio presentation, independent of editor drafts and active operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "VisualizationStudioState")]
pub struct VisualizationStudioPresentation {
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
}

impl Default for VisualizationStudioPresentation {
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
    DEFAULT_DISPLAY_CACHE_MIB
}

const fn default_significant_digits() -> u8 {
    7
}

pub const REPORT_PAGE_TEMPLATES: [&str; 3] = [
    "Release verification 4.2",
    "Design review",
    "Model qualification",
];
pub const MAX_REPORT_PAGE_TITLE_BYTES: usize = 120;
const MAX_COMPARISON_RECEIPTS: usize = 512;
pub const MAX_DIFFERENCE_TRACE_SETS: usize = 4_096;
pub const MAX_DIFFERENCE_TRACE_NUMERIC_VALUES: usize = 8_000_000;

impl VisualizationStudioPresentation {
    pub fn validate_presentation(&self) -> Result<(), String> {
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

    pub fn normalize(&mut self) {
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

/// Default resident budget for reconstructable display envelopes.
///
/// Every entry here is derived data that costs one O(n) pass to rebuild, so
/// the budget buys latency, not correctness. A workstation can spare half a
/// gigabyte for it; a phone browser cannot — the whole WebAssembly heap is
/// often smaller than that, and the retained waveforms have to live in it too.
#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_DISPLAY_CACHE_MIB: u32 = 512;
#[cfg(target_arch = "wasm32")]
pub const DEFAULT_DISPLAY_CACHE_MIB: u32 = 96;
