//! Session-scoped state for verification and the capability views.
//!
//! Everything here is retained only for the active session and never treated
//! as evidence: a regression receipt records what a review saw, but the source
//! runs and datasets remain the authoritative owners. That is why none of it
//! is durable — reopening the project must re-derive it.

use super::*;

/// Immutable regression comparison receipt retained only for the active
/// verification review session. The source runs and datasets remain the
/// authoritative evidence owners.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionComparisonReceipt {
    pub plan_id: crate::product::SimulationPlanId,
    pub plan_revision: crate::product::ObjectRevision,
    pub tolerance_digest: crate::product::ContentDigest,
    pub baseline_run: crate::product::RunId,
    pub candidate_run: crate::product::RunId,
    pub baseline_dataset: crate::product::DatasetId,
    pub candidate_dataset: crate::product::DatasetId,
    pub baseline_content_digest: crate::product::ContentDigest,
    pub candidate_content_digest: crate::product::ContentDigest,
    pub baseline_authority_digest: crate::product::ContentDigest,
    pub candidate_authority_digest: crate::product::ContentDigest,
    pub aligned_checks: usize,
    pub aligned_waveforms: usize,
    pub changed_checks: usize,
    #[serde(default)]
    pub passed_checks: usize,
    #[serde(default)]
    pub failed_checks: usize,
    #[serde(default)]
    pub passed_waveforms: usize,
    #[serde(default)]
    pub failed_waveforms: usize,
    #[serde(default)]
    pub unconfigured_targets: usize,
    #[serde(default)]
    pub unevaluated_targets: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegressionToleranceDraft {
    pub target: crate::state::RegressionTargetSelector,
    pub method: crate::state::RegressionComparisonMethod,
    pub absolute_tolerance: String,
    pub relative_tolerance_percent: String,
    pub time_skew_allowance: String,
    pub comparison_window: String,
    pub dirty: bool,
    pub validation_error: Option<String>,
}

/// Runtime-only candidate value owned by the non-destructive parameter tuner.
/// The persisted `DesignVariable` remains authoritative until the user commits
/// the complete candidate set as one simulation-plan revision.
#[derive(Debug, Clone, PartialEq)]
pub struct TuningVariableDraft {
    pub variable_id: crate::product::DesignVariableId,
    pub baseline_expression: String,
    pub candidate_expression: String,
    pub validation_error: Option<String>,
    /// A proposed variable does not exist in the authoritative plan yet.
    /// Its candidate expression is still initialized from the selected
    /// instance literal, so this bit participates in dirty detection.
    pub proposed: bool,
}

impl TuningVariableDraft {
    pub fn is_dirty(&self) -> bool {
        self.proposed || self.candidate_expression.trim() != self.baseline_expression.trim()
    }
}

/// Runtime-only bridge between a selected schematic instance and the tuning
/// sandbox.  The proposed variable and binding are deliberately kept outside
/// both authoritative stores until the review dialog commits them together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuningInstanceBindingDraft {
    pub component_id: u64,
    pub component_name: String,
    pub source_view: crate::state::CellViewRef,
    pub source_topology_version: u64,
    pub source_value: String,
    pub binding_expression: String,
    pub variable: crate::state::DesignVariable,
    pub creates_variable: bool,
}

impl TuningInstanceBindingDraft {
    #[must_use]
    pub fn requires_schematic_edit(&self) -> bool {
        self.source_value.trim() != self.binding_expression
    }
}

/// One limit cell of the Monte Carlo yield table, open for entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldLimitDraft {
    /// The output whose limit is being entered, as the trials name it.
    pub output: String,
    pub bound: YieldLimitBound,
    /// What the reader has typed.
    pub text: String,
    /// What the cell said when it opened. Applying it unchanged changes
    /// nothing, so a limit spelled to four digits is never rounded by a click.
    pub seed: String,
    /// Set until the cell first paints, so the field takes the keyboard once.
    pub focus: bool,
}

/// Which bound of a limit a yield cell states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YieldLimitBound {
    Lower,
    Upper,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VerificationSessionState {
    #[serde(default)]
    pub regression_baseline_run: Option<crate::product::RunId>,
    /// When enabled, process-corner cells are expressed as exact deltas from
    /// the revision-matched TT/nominal/room point retained by the active
    /// corner plan. The flag is review state only; result datasets remain
    /// immutable.
    #[serde(default)]
    pub corner_compare_nominal: bool,
    /// The Monte Carlo yield row whose histogram the page shows, by the
    /// specification's measurement name. Review state only.
    #[serde(skip)]
    pub yield_selected_spec: Option<String>,
    /// The yield table's limit cell open for entry. Transient: a limit is
    /// the plan's only once it is applied.
    #[serde(skip)]
    pub yield_limit_draft: Option<YieldLimitDraft>,
    #[serde(skip)]
    pub regression_comparison: Option<RegressionComparisonReceipt>,
    #[serde(skip)]
    pub regression_baseline_picker_open: bool,
    #[serde(skip)]
    pub regression_baseline_picker_selection: Option<crate::product::RunId>,
    #[serde(skip)]
    pub regression_selected_target: Option<crate::state::RegressionTargetSelector>,
    #[serde(skip)]
    pub regression_tolerance_drafts: Vec<RegressionToleranceDraft>,
    /// Stable plan/revision that owns `tuning_variables`. All three fields are
    /// transient so an interrupted sandbox can never restore as project data.
    #[serde(skip)]
    pub tuning_plan_id: Option<crate::product::SimulationPlanId>,
    #[serde(skip)]
    pub tuning_plan_revision: Option<crate::product::ObjectRevision>,
    #[serde(skip)]
    pub tuning_variables: Vec<TuningVariableDraft>,
    /// Optional instance Value-row proposal that opened this sandbox.
    #[serde(skip)]
    pub tuning_instance_binding: Option<TuningInstanceBindingDraft>,
    /// Selected variable card and one-shot editor focus request. Both are
    /// transient presentation state and never confer commit authority.
    #[serde(skip)]
    pub tuning_selected_variable: Option<crate::product::DesignVariableId>,
    #[serde(skip)]
    pub tuning_focus_variable: Option<crate::product::DesignVariableId>,
    /// Immutable retained run selected when the sandbox was opened. This is
    /// presentation state only; the run and dataset remain owned by
    /// `SimulationState` and are never copied into the tuner.
    #[serde(skip)]
    pub tuning_baseline_run: Option<crate::product::RunId>,
    /// Review gate opened by the mockup-specified tuning commit action. The
    /// dialog is transient and never restores across an application session.
    #[serde(skip)]
    pub tuning_review_open: bool,
    /// Why the last Verify action the reader took could not be carried out,
    /// stated under the toolbar until the next action. An action that worked
    /// leaves nothing here: its result is the page itself.
    #[serde(skip)]
    pub action_problem: Option<String>,
}

/// Local presentation state for the canonical Jobs manager. Selection uses
/// stable run identity so history insertion, pruning, and reload cannot
/// silently retarget the inspector or exported manifest.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JobsPlanScope {
    ActivePlan,
    #[default]
    AllPlans,
    ManualDeck,
}

impl JobsPlanScope {
    pub const ALL: [Self; 3] = [Self::ActivePlan, Self::AllPlans, Self::ManualDeck];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActivePlan => "Active plan",
            Self::AllPlans => "All plans",
            Self::ManualDeck => "Manual decks",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JobsManagerState {
    pub selected_run_id: Option<crate::product::RunId>,
    pub scroll_offset: f32,
    pub plan_scope: JobsPlanScope,
}
