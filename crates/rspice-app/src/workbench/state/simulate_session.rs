//! What the Simulate and Verify workspaces keep for the session and for
//! themselves alone: the verification review's working state, the run-set
//! budgets being typed, the outcome of the last plan command, the specification
//! evidence filter and the Jobs manager's view.
//!
//! None of it is evidence or project configuration. Of what is here, the
//! session keeps only the regression baseline and the corner comparison;
//! everything else is rebuilt on reopen.

use super::*;

/// Authored text of the run-set execution budgets while they are being edited.
///
/// A budget is committed on release, not on keystroke: a partially typed
/// "4 Mi" would otherwise parse to four bytes and refuse the whole space for
/// as long as the field had focus.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetBudgetDrafts {
    pub maximum_tasks: String,
    pub maximum_storage: String,
    pub cost_per_point_ms: String,
    pub bytes_per_point: String,
}

/// Whether a Simulation Studio plan command changed the plan or was refused.
///
/// A receipt and a refusal are told apart here rather than by reading the
/// message, because the two are shown differently and only one of them has to
/// follow the reader off the Analyses page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnalysisLifecycleSeverity {
    /// Everything the command asked for was committed, and the message is the
    /// transaction's receipt.
    #[default]
    Receipt,
    /// Something the command asked for was refused, and the message names what
    /// and why. A command that commits one step and is refused the next is a
    /// refusal: the reader has to be told about the part that did not happen.
    Refusal,
}

/// The last outcome any Simulation Studio plan command announced.
///
/// Only the Analyses route draws this; the Console holds the record of each
/// outcome. An outcome is new only when it *changes*, not on every
/// restatement: three of the writers sit on the render path rather than in a
/// click handler, and recording each restatement would record the same
/// refusal forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisLifecycleOutcome {
    severity: AnalysisLifecycleSeverity,
    message: String,
}

impl Default for AnalysisLifecycleOutcome {
    /// The field is runtime-only, so a restored session gets this rather than
    /// the outcome it was saved with. It has to read as a real status line.
    fn default() -> Self {
        Self {
            severity: AnalysisLifecycleSeverity::Receipt,
            message: "No lifecycle command has been committed this session.".to_owned(),
        }
    }
}

impl AnalysisLifecycleOutcome {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn is_refusal(&self) -> bool {
        matches!(self.severity, AnalysisLifecycleSeverity::Refusal)
    }

    /// Announce a committed change. Returns whether this is a new outcome, so
    /// that a reporter which also records it elsewhere records it once per
    /// event.
    pub fn record_receipt(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Receipt, message)
    }

    /// Announce a command that changed nothing, and why. Returns whether this
    /// is a new outcome, as [`Self::record_receipt`] does.
    pub fn record_refusal(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Refusal, message)
    }

    /// Whether the announced outcome differs from the one already held.
    ///
    /// Three announcing sites sit on the render path rather than in a click
    /// handler, so a standing refusal is restated every frame. Restating it is
    /// not an event, and a reporter reading the answer does not report it
    /// again.
    fn record(&mut self, severity: AnalysisLifecycleSeverity, message: impl Into<String>) -> bool {
        let message = message.into();
        if self.severity == severity && self.message == message {
            return false;
        }
        self.severity = severity;
        self.message = message;
        true
    }
}

/// Which specifications the specification registry lists.
///
/// The classes name evidence, not release policy. A specification carries no
/// severity, so a "blocking only" view would be a control with nothing behind
/// it; what the page does resolve for every row is whether the active dataset
/// answered the limit, and how completely. The registry owns that
/// classification — this only says which of its outcomes to show.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpecificationEvidenceFilter {
    #[default]
    All,
    /// Limits the dataset answered with a failure, including a measurement
    /// that ran and did not succeed.
    Failing,
    /// Limits no measurement in the dataset answers at all.
    WithoutEvidence,
    /// Limits judged on the worst of several retained measurements, so the
    /// verdict speaks for the points the dataset kept and no others.
    Partial,
}

impl SpecificationEvidenceFilter {
    pub const ALL: [Self; 4] = [
        Self::All,
        Self::Failing,
        Self::WithoutEvidence,
        Self::Partial,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All requirements",
            Self::Failing => "Failing only",
            Self::WithoutEvidence => "Without results",
            Self::Partial => "Measured at several points",
        }
    }
}

pub(in crate::workbench) const fn default_analysis_index() -> usize {
    rspice_simulation_contract::analysis_kind::AnalysisKind::Transient.legacy_index()
}

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
