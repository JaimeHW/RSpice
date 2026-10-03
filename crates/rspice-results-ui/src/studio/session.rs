//! Studio viewer session, editor drafts and local presentation transactions.
//!
//! Canonical project documents, source reconciliation and execution remain with the host.
use super::{dock::VisualizationDock, inspector::OperationState};
use rspice_app_types::product::DatasetId;
use rspice_results::{
    result_presentation::ResultViewer,
    studio_presentation::{
        ComparisonAlignmentDraft, ViewerTool, VisualizationPane, VisualizationPanePlacement,
        VisualizationStudioPresentation,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VisualizationStudioState {
    pub presentation: VisualizationStudioPresentation,
    #[serde(skip)]
    pub viewer_query: String,
    #[serde(skip)]
    pub dock: Option<VisualizationDock>,
    #[serde(skip)]
    pub draft_viewer: ResultViewer,
    #[serde(skip)]
    pub draft_dataset_id: Option<DatasetId>,
    #[serde(skip)]
    pub draft_analysis_sequence: Option<u64>,
    #[serde(skip)]
    pub draft_pane_placement: VisualizationPanePlacement,
    #[serde(skip)]
    pub draft_page_title: String,
    #[serde(skip)]
    pub draft_annotation: String,
    #[serde(skip)]
    pub draft_measurement: String,
    #[serde(skip)]
    pub draft_page_pane: Option<u64>,
    #[serde(skip)]
    pub draft_page: String,
    #[serde(skip)]
    pub draft_report_template: String,
    #[serde(skip)]
    pub draft_report_freeze: bool,
    #[serde(skip)]
    pub draft_link_pane: Option<u64>,
    #[serde(skip)]
    pub draft_x_link: u64,
    #[serde(skip)]
    pub draft_cursor_group: u64,
    #[serde(skip)]
    pub draft_pane_order: Vec<u64>,
    #[serde(skip)]
    pub draft_trace_dataset: Option<DatasetId>,
    #[serde(skip)]
    pub draft_trace_analysis: Option<u64>,
    #[serde(skip)]
    pub draft_trace_visibility: Vec<(String, bool)>,
    #[serde(skip)]
    pub draft_significant_digits: Option<u8>,
    #[serde(skip)]
    pub draft_phase_continuous: Option<bool>,
    #[serde(skip)]
    pub applied_link_pane: Option<u64>,
    #[serde(skip)]
    pub family_query: String,
    #[serde(skip)]
    pub draft_family_x_dimension: String,
    #[serde(skip)]
    pub draft_family_dimension: String,
    #[serde(skip)]
    pub draft_family_color_dimension: String,
    #[serde(skip)]
    pub draft_family_dash_dimension: String,
    #[serde(skip)]
    pub draft_family_marker_dimension: String,
    #[serde(skip)]
    pub draft_family_exclude_missing: bool,
    #[serde(skip)]
    pub draft_comparison_dataset: Option<DatasetId>,
    #[serde(skip)]
    pub draft_comparison_candidates: Vec<DatasetId>,
    #[serde(skip)]
    pub draft_comparison_data_version: u64,
    #[serde(skip)]
    pub draft_comparison_absolute_tolerance: f64,
    #[serde(skip)]
    pub draft_comparison_relative_tolerance: f64,
    #[serde(skip)]
    pub draft_comparison_alignment: ComparisonAlignmentDraft,
    #[serde(skip)]
    pub draft_comparison_alignment_signal: String,
    #[serde(skip)]
    pub draft_comparison_threshold: f64,
    #[serde(skip)]
    pub draft_comparison_maximum_lag_samples: u32,
    #[serde(skip)]
    pub draft_comparison_difference_trace: bool,
    #[serde(skip)]
    pub operation_state: OperationState,
    #[serde(skip)]
    pub operation_dataset_id: Option<DatasetId>,
    #[serde(skip)]
    pub operation_analysis_sequence: Option<u64>,
    #[serde(skip)]
    pub operation_processed: usize,
    #[serde(skip)]
    pub operation_total: usize,
    #[serde(skip)]
    pub operation_checksum: u64,
}

impl std::ops::Deref for VisualizationStudioState {
    type Target = VisualizationStudioPresentation;

    fn deref(&self) -> &Self::Target {
        &self.presentation
    }
}

impl std::ops::DerefMut for VisualizationStudioState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.presentation
    }
}

impl Default for VisualizationStudioState {
    fn default() -> Self {
        Self {
            presentation: VisualizationStudioPresentation::default(),
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
    pub fn reset_transient_view(&mut self) {
        self.tool = ViewerTool::Select;
        self.zoom = VisualizationStudioPresentation::default().zoom;
        self.viewer_query.clear();
        self.family_query.clear();
        self.dock = None;
        self.linked_x_ranges.clear();
        self.linked_cursor_positions.clear();
        self.pane_x_ranges.clear();
        self.pane_cursor_positions.clear();
    }
}

impl VisualizationStudioState {
    pub fn allocate_identity(&mut self) -> Option<u64> {
        let id = self.next_identity;
        self.next_identity = self.next_identity.checked_add(1)?;
        Some(id)
    }

    pub fn active_pane_mut(&mut self) -> Option<&mut VisualizationPane> {
        let active = self.active_pane?;
        self.panes.iter_mut().find(|pane| pane.id == active)
    }

    pub fn active_pane(&self) -> Option<&VisualizationPane> {
        let active = self.active_pane?;
        self.panes.iter().find(|pane| pane.id == active)
    }

    pub fn transact<T>(
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

    pub fn commit_revision(&mut self) -> Result<(), String> {
        self.transact(|_| Ok(()))
    }
}
