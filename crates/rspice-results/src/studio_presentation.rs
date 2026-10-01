//! Retained Visualization Studio pane bindings and display declarations.

use rspice_app_types::product::DatasetId;
use serde::{Deserialize, Serialize};

use crate::result_presentation::ResultViewer;

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
