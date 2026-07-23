//! Runtime presentation state for the project-owned measurement-correlation
//! workspace.
//!
//! Correlation suites, imported datasets, metric definitions, dispositions,
//! and immutable evidence are engineering records owned by the model-library
//! state. This module retains only the active section and transactional dialog
//! drafts. A cancelled or interrupted dialog can therefore never mutate the
//! project.

use crate::state::model_library::ModelSourceEvidenceBinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModelCorrelationSection {
    #[default]
    Datasets,
    Conditions,
    Metrics,
    Comparison,
    Outliers,
    Evidence,
    Gate,
}

impl ModelCorrelationSection {
    pub const ALL: [Self; 7] = [
        Self::Datasets,
        Self::Conditions,
        Self::Metrics,
        Self::Comparison,
        Self::Outliers,
        Self::Evidence,
        Self::Gate,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Datasets => "Datasets",
            Self::Conditions => "Conditions",
            Self::Metrics => "Metrics",
            Self::Comparison => "Comparison",
            Self::Outliers => "Outliers",
            Self::Evidence => "Evidence",
            Self::Gate => "Gate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationDatasetClassDraft {
    #[default]
    Bench,
    Silicon,
    Vendor,
    IndependentOracle,
    Simulated,
}

impl CorrelationDatasetClassDraft {
    pub const ALL: [Self; 5] = [
        Self::Bench,
        Self::Silicon,
        Self::Vendor,
        Self::IndependentOracle,
        Self::Simulated,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Bench => "Bench measurement",
            Self::Silicon => "Silicon characterization",
            Self::Vendor => "Vendor published",
            Self::IndependentOracle => "Independent oracle",
            Self::Simulated => "Model simulation",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorrelationDatasetDraft {
    pub id: String,
    pub name: String,
    /// Accountable owner of a newly created suite. Subsequent revisions retain
    /// the immutable suite owner and review decisions require another identity.
    pub suite_owner_id: String,
    pub class: CorrelationDatasetClassDraft,
    pub authority: String,
    pub device_or_lot: String,
    pub fixture: String,
    pub calibration: String,
    pub source_name: String,
    /// Stable identity of the completed retained run used for a model-generated
    /// dataset. The controller resolves every other provenance field from run
    /// history; the dialog never accepts self-asserted execution identity.
    pub retained_run_id: String,
    /// Run-local analysis identity selected from the retained run.
    pub retained_analysis_id: String,
    /// Exact retained waveform name exported into canonical correlation CSV.
    pub retained_trace_name: String,
    /// UTF-8 CSV retained verbatim and hashed by the domain importer.
    pub csv: String,
    pub validation_error: Option<String>,
}

impl Default for CorrelationDatasetDraft {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            suite_owner_id: String::new(),
            class: CorrelationDatasetClassDraft::Bench,
            authority: String::new(),
            device_or_lot: String::new(),
            fixture: String::new(),
            calibration: String::new(),
            source_name: String::new(),
            retained_run_id: String::new(),
            retained_analysis_id: String::new(),
            retained_trace_name: String::new(),
            csv: "id,quantity,value,unit,uncertainty,weight,condition:temperature[degC]\n"
                .to_owned(),
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationCalculationDraft {
    #[default]
    AbsoluteLinear,
    AbsoluteDecibels,
    Relative,
    WeightedRelative,
    PhaseWrapped,
}

impl CorrelationCalculationDraft {
    pub const ALL: [Self; 5] = [
        Self::AbsoluteLinear,
        Self::AbsoluteDecibels,
        Self::Relative,
        Self::WeightedRelative,
        Self::PhaseWrapped,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::AbsoluteLinear => "Absolute linear error",
            Self::AbsoluteDecibels => "Absolute dB error",
            Self::Relative => "Relative error",
            Self::WeightedRelative => "Weighted relative error",
            Self::PhaseWrapped => "Phase-wrapped",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationAggregationDraft {
    #[default]
    EveryPoint,
    WorstCase,
    Percentile95,
    RootMeanSquare,
}

impl CorrelationAggregationDraft {
    pub const ALL: [Self; 4] = [
        Self::EveryPoint,
        Self::WorstCase,
        Self::Percentile95,
        Self::RootMeanSquare,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EveryPoint => "Every point",
            Self::WorstCase => "Worst case",
            Self::Percentile95 => "95th percentile",
            Self::RootMeanSquare => "Root mean square",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationAlignmentDraft {
    #[default]
    Exact,
    MonotoneInterpolation,
}

impl CorrelationAlignmentDraft {
    pub const ALL: [Self; 2] = [Self::Exact, Self::MonotoneInterpolation];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact coordinates",
            Self::MonotoneInterpolation => "Monotone linear interpolation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationExtrapolationDraft {
    #[default]
    Forbid,
    Limited,
}

impl CorrelationExtrapolationDraft {
    pub const ALL: [Self; 2] = [Self::Forbid, Self::Limited];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Forbid => "Forbid",
            Self::Limited => "Limited span",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationReleaseRoleDraft {
    #[default]
    Review,
    Advisory,
}

impl CorrelationReleaseRoleDraft {
    pub const ALL: [Self; 2] = [Self::Review, Self::Advisory];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Review => "Review",
            Self::Advisory => "Advisory",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorrelationMetricDraft {
    pub id: String,
    pub name: String,
    pub quantity: String,
    pub reference_dataset_id: String,
    pub simulation_dataset_id: String,
    pub calculation: CorrelationCalculationDraft,
    pub domain_enabled: bool,
    pub domain_axis: String,
    pub domain_unit: String,
    pub domain_minimum: String,
    pub domain_maximum: String,
    pub limit: String,
    pub uncertainty_multiplier: String,
    /// Minimum included-reference fraction required for a passing metric.
    /// This prevents exclusions from manufacturing passing evidence.
    pub minimum_coverage: String,
    pub aggregation: CorrelationAggregationDraft,
    pub alignment: CorrelationAlignmentDraft,
    pub alignment_axis: String,
    pub extrapolation: CorrelationExtrapolationDraft,
    pub extrapolation_fraction: String,
    pub release_role: CorrelationReleaseRoleDraft,
    pub validation_error: Option<String>,
}

impl Default for CorrelationMetricDraft {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            quantity: String::new(),
            reference_dataset_id: String::new(),
            simulation_dataset_id: String::new(),
            calculation: CorrelationCalculationDraft::Relative,
            domain_enabled: false,
            domain_axis: String::new(),
            domain_unit: String::new(),
            domain_minimum: String::new(),
            domain_maximum: String::new(),
            limit: "1".to_owned(),
            uncertainty_multiplier: "1".to_owned(),
            minimum_coverage: "1".to_owned(),
            aggregation: CorrelationAggregationDraft::RootMeanSquare,
            alignment: CorrelationAlignmentDraft::Exact,
            alignment_axis: String::new(),
            extrapolation: CorrelationExtrapolationDraft::Forbid,
            extrapolation_fraction: "0.05".to_owned(),
            release_role: CorrelationReleaseRoleDraft::Review,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationOutlierDecisionDraft {
    #[default]
    Retain,
    ExcludeFixtureFault,
    LimitOnlyEvidence,
}

impl CorrelationOutlierDecisionDraft {
    pub const ALL: [Self; 3] = [
        Self::Retain,
        Self::ExcludeFixtureFault,
        Self::LimitOnlyEvidence,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Retain => "Retain in gate",
            Self::ExcludeFixtureFault => "Exclude: fixture fault",
            Self::LimitOnlyEvidence => "Limit-only evidence",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CorrelationOutlierDraft {
    pub metric_id: String,
    pub reference_observation_id: String,
    pub decision: CorrelationOutlierDecisionDraft,
    pub reason: String,
    pub owner_id: String,
    pub reviewer_id: String,
    pub validation_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CorrelationReviewDraft {
    pub reviewer_id: String,
    pub decision: CorrelationReviewDecisionDraft,
    pub conclusion: String,
    pub validation_error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationReviewDecisionDraft {
    Approve,
    #[default]
    Reject,
}

impl CorrelationReviewDecisionDraft {
    pub const ALL: [Self; 2] = [Self::Approve, Self::Reject];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Approve => "Approve for qualification",
            Self::Reject => "Reject for qualification",
        }
    }
}

impl Default for CorrelationReviewDraft {
    fn default() -> Self {
        Self {
            reviewer_id: String::new(),
            decision: CorrelationReviewDecisionDraft::Reject,
            conclusion: String::new(),
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModelCorrelationDialog {
    AddDataset(CorrelationDatasetDraft),
    AddMetric(CorrelationMetricDraft),
    Review(CorrelationReviewDraft),
    Disposition(CorrelationOutlierDraft),
}

#[derive(Debug, Clone)]
pub struct ModelCorrelationWorkspaceState {
    pub section: ModelCorrelationSection,
    pub selected_suite_id: Option<String>,
    pub selected_dataset_id: Option<String>,
    pub selected_metric_id: Option<String>,
    pub dialog: Option<ModelCorrelationDialog>,
    /// Exact model source that owned the dialog when it opened. Submitting
    /// against a different source is forbidden even if the user changed the
    /// selection behind the modal through another input path.
    pub dialog_source: Option<ModelSourceEvidenceBinding>,
    pub notice: Option<String>,
    pub scroll_offset: f32,
}

impl Default for ModelCorrelationWorkspaceState {
    fn default() -> Self {
        Self {
            section: ModelCorrelationSection::Datasets,
            selected_suite_id: None,
            selected_dataset_id: None,
            selected_metric_id: None,
            dialog: None,
            dialog_source: None,
            notice: None,
            scroll_offset: 0.0,
        }
    }
}

impl ModelCorrelationWorkspaceState {
    #[must_use]
    pub const fn dialog_open(&self) -> bool {
        self.dialog.is_some()
    }

    pub fn reset_for_navigation(&mut self) {
        self.section = ModelCorrelationSection::Datasets;
        self.dialog = None;
        self.dialog_source = None;
        self.selected_suite_id = None;
        self.selected_dataset_id = None;
        self.selected_metric_id = None;
        self.notice = None;
        self.scroll_offset = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_contract_matches_the_mockup_order() {
        assert_eq!(
            ModelCorrelationSection::ALL.map(ModelCorrelationSection::label),
            [
                "Datasets",
                "Conditions",
                "Metrics",
                "Comparison",
                "Outliers",
                "Evidence",
                "Gate",
            ]
        );
    }

    #[test]
    fn transactional_dialogs_are_not_authoritative_project_state() {
        let mut state = ModelCorrelationWorkspaceState {
            dialog: Some(ModelCorrelationDialog::AddDataset(
                CorrelationDatasetDraft::default(),
            )),
            ..ModelCorrelationWorkspaceState::default()
        };
        state.reset_for_navigation();
        assert!(state.dialog.is_none());
        assert_eq!(state.section, ModelCorrelationSection::Datasets);
    }
}
