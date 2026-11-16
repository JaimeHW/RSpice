//! What the Simulate and Verify workspaces keep for the session and for
//! themselves alone: the verification review's working state, the run-set
//! budgets being typed, the specification evidence filter and the Jobs
//! manager's view.
//!
//! None of it is evidence or project configuration. Of what is here, the
//! session keeps only the regression baseline and how Corners is arranged;
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

pub const fn default_analysis_index() -> usize {
    rspice_simulation_contract::analysis_kind::AnalysisKind::Transient.singleton_index()
}

/// The receipt of comparing a run against its reference: kept with the
/// Reference view's comparison and quoted by its CI export. The runs and
/// their datasets remain the evidence it describes.
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

/// The Reference view's tolerance cell open for entry.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceToleranceDraft {
    /// The target whose tolerance is being entered.
    pub target: crate::state::RegressionTargetSelector,
    /// What the reader has typed.
    pub text: String,
    /// What the cell said when it opened: entering it unchanged changes
    /// nothing.
    pub seed: String,
    /// Set until the cell first paints, so the field takes the keyboard once.
    pub focus: bool,
}

/// The limit cell of the Monte Carlo table, open for entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldLimitDraft {
    /// The measurement whose limit is being entered.
    pub output: String,
    /// What the reader has typed: `≥ 40 dB`, `≤ 1.2 mA`, `10 … 20 dB`.
    pub text: String,
    /// What the cell said when it opened. Applying it unchanged changes
    /// nothing, so a limit spelled to four digits is never rounded by a click.
    pub seed: String,
    /// Set until the cell first paints, so the field takes the keyboard once.
    pub focus: bool,
}

/// Which cell of a Specifications row is open for entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecificationCell {
    Name,
    Limit,
}

/// One cell of the Specifications table, open for entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificationCellDraft {
    /// The row's measurement, which names the specification within its plan.
    pub measurement: String,
    pub cell: SpecificationCell,
    /// What the reader has typed.
    pub text: String,
    /// What the cell said when it opened. Applying it unchanged changes
    /// nothing, so a limit spelled to four digits is never rounded by a click.
    pub seed: String,
    /// Set until the cell first paints, so the field takes the keyboard once.
    pub focus: bool,
}

/// What a Specifications menu offers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificationsMenuKind {
    /// The run's measurements no specification names, then a new one.
    Add,
    /// One row's actions, by the row's measurement.
    Row(String),
}

/// A menu the Specifications view has open, and where.
#[derive(Debug, Clone, PartialEq)]
pub struct SpecificationsMenu {
    pub kind: SpecificationsMenuKind,
    pub position: egui::Pos2,
}

/// The specification the Verify inspector is editing, as the reader has typed
/// it so far.
#[derive(Debug, Clone, Default)]
pub struct SpecificationInspectorDraft {
    /// The specification the draft was taken from.
    pub specification: Option<rspice_vocabulary::product::SpecificationId>,
    /// The plan revision it was taken at: a change from anywhere else takes a
    /// fresh draft rather than leaving stale words over a new definition.
    pub revision: Option<crate::product::ObjectRevision>,
    pub draft: rspice_results_ui::specs::editor::SpecDraft,
}

/// Two drafts are the same draft when they edit the same specification at the
/// same revision: the words in them are the reader's typing, not state a
/// comparison should read.
impl PartialEq for SpecificationInspectorDraft {
    fn eq(&self, other: &Self) -> bool {
        self.specification == other.specification && self.revision == other.revision
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VerificationSessionState {
    #[serde(default)]
    pub regression_baseline_run: Option<crate::product::RunId>,
    /// The Monte Carlo table's limit cell open for entry. Transient: a limit
    /// is the plan's only once it is applied.
    #[serde(skip)]
    pub yield_limit_draft: Option<YieldLimitDraft>,
    /// Why the last Verify action the reader took could not be carried out,
    /// stated under the toolbar until the next action. An action that worked
    /// leaves nothing here: its result is the page itself.
    #[serde(skip)]
    pub action_problem: Option<String>,
    /// The Specifications view's table: its selection, sort, scroll and
    /// filter. Review state only.
    #[serde(skip)]
    pub specifications_table: crate::workbench::surfaces::verify::table::TableState,
    /// The Specifications cell open for entry. Transient: a name or a limit
    /// is the plan's only once it is applied.
    #[serde(skip)]
    pub specification_cell_draft: Option<SpecificationCellDraft>,
    /// The menu the Specifications view has open.
    #[serde(skip)]
    pub specifications_menu: Option<SpecificationsMenu>,
    /// The inspector's draft of the selected specification. Transient for
    /// the same reason as the cell draft.
    #[serde(skip)]
    pub specification_draft: Option<SpecificationInspectorDraft>,
    /// The Corners table's selected cell, sort and sideways scroll.
    #[serde(skip)]
    pub corners_table: crate::workbench::surfaces::verify::table::TableState,
    /// Corners lists the points as rows and the specifications as columns.
    #[serde(default)]
    pub corners_transposed: bool,
    /// Corners hides the specifications with no failing cell; transposed,
    /// the points with none.
    #[serde(default)]
    pub corners_failing_only: bool,
    /// The Monte Carlo table's selected specification, sort and scroll.
    #[serde(skip)]
    pub monte_carlo_table: crate::workbench::surfaces::verify::table::TableState,
    /// The selected specification's failing trials: the selected trial.
    #[serde(skip)]
    pub monte_carlo_trials_table: crate::workbench::surfaces::verify::table::TableState,
    /// The point the Monte Carlo view states its trials at, by its place in
    /// the run's points; `None` states every point's trials together.
    #[serde(skip)]
    pub monte_carlo_point: Option<usize>,
    /// The Checks view's table: its selection, sort and sideways scroll.
    #[serde(skip)]
    pub checks_table: crate::workbench::surfaces::verify::table::TableState,
    /// The Checks view leaves out the checks that passed or only informed.
    #[serde(skip)]
    pub checks_violations_only: bool,
    /// The Checks view's rows in the order it shows them.
    #[serde(skip)]
    pub checks_rows: CheckRowOrder,
    /// The Reference view's table: its selection, sort and sideways scroll.
    #[serde(skip)]
    pub reference_table: crate::workbench::surfaces::verify::table::TableState,
    /// The Reference view's tolerance cell open for entry. Transient: a
    /// tolerance is the plan's only once it is entered.
    #[serde(skip)]
    pub reference_edit: Option<ReferenceToleranceDraft>,
    /// The Reference view's comparison, built again only when the run read,
    /// its reference or the plan's tolerances change.
    #[serde(skip)]
    pub(crate) reference: crate::workbench::surfaces::verify::reference::ReferenceComparison,
}

/// The Checks view's rows in the order it shows them: worst first, and only
/// the violations when the reader asked for them. Built again only when the
/// run's checks are rebuilt or the filter changes, so an idle frame orders
/// nothing.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckRowOrder {
    /// The checks' revision and the filter the order was built for.
    pub basis: Option<(u64, bool)>,
    /// Each shown row's position in the run's checks.
    pub rows: Vec<usize>,
    /// Each shown row's key: what the check is, so a selection outlives the
    /// run's checks being read again.
    pub keys: Vec<crate::workbench::surfaces::verify::table::RowKey>,
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
