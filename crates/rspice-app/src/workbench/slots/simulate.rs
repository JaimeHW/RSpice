//! The simulate feature's dialogs: the analysis plan's removal review; and
//! the Simulation and Verification
//! Studios' session: their pages, selections, drafts and filters, the
//! analysis lifecycle outcome and the Jobs manager's review context.

use crate::workbench::framework::feature::SimulatePort;

/// What the simulate feature keeps between frames.
#[derive(Debug, Clone)]
pub(crate) struct SimulateState {
    /// Destructive review for removing one record from the analysis plan —
    /// an analysis, a design variable, a saved output or a capture group —
    /// raised only where removal orphans something.
    pub(crate) plan_removal_review: crate::workbench::app::dialogs::plan_removal::PlanRemovalReview,
    /// Why a removal the plan will not perform was refused before it was
    /// staged. The review above asks; this one only states.
    pub(crate) plan_removal_refusal:
        crate::workbench::app::dialogs::plan_removal::PlanRemovalRefusal,

    pub verification_page: crate::workbench::state::VerificationPage,
    /// Selected Simulation Studio setup route.
    pub simulation_page: crate::workbench::state::SimulationPage,
    pub verification: crate::workbench::state::VerificationSessionState,
    /// Analysis row whose configuration is shown in the simulation plan.
    /// Retained only to migrate pre-instance session selection.
    pub active_analysis: usize,
    /// Stable analysis instance whose configuration is shown in the plan.
    pub active_analysis_instance: Option<crate::product::AnalysisInstanceId>,
    /// Device-local Simulation Studio viewport. Keeping this outside egui's
    /// transient widget memory prevents a committed form edit from snapping
    /// the editor back to the top when the selected instance is rebuilt.
    pub simulation_surface_scroll_y: f32,
    /// One-frame correction for structural edits above the stacked analysis
    /// editor. It is consumed after egui reports the current ScrollArea offset.
    pub simulation_surface_pending_scroll_delta_y: f32,
    /// Screen-space top edge of the selected analysis form immediately before
    /// a stacked structural edit. The next frame measures the actual displaced
    /// position so scroll compensation follows egui's rendered geometry and
    /// status-message height changes cannot move the editable controls.
    pub simulation_surface_editor_anchor_y: Option<f32>,
    /// Last analysis lifecycle outcome announced by the transaction owner.
    pub analysis_lifecycle_status: crate::workbench::state::AnalysisLifecycleOutcome,
    /// Active Simulation Studio transaction. Drafts are runtime-only and are
    /// committed atomically by the dialog's primary action.
    pub simulation_workflow: Option<crate::workbench::state::SimulationWorkflowDialog>,
    /// Selected specification row in the verification matrix, by exact
    /// measurement name. That name is the specification's identity — it is the
    /// key the limit is joined to a result by, and it is unique within a plan.
    /// A row index would silently re-point at a different requirement when the
    /// specification editor, which owns authoring, removes or reorders one.
    ///
    /// The field name differs from the older `selected_spec` deliberately: that
    /// key held an index, and leaving it unread is what makes an older project
    /// start with no selection instead of resolving one to the wrong row.
    pub selected_specification: Option<String>,
    /// Selected design-variable row on the Simulation Studio variables page,
    /// by exact name. Names are unique within a plan, and a name survives the
    /// registry being reordered where a row index would not.
    pub selected_design_variable: Option<String>,
    /// Selected saved-output row on the outputs page, by exact name.
    pub selected_saved_output: Option<String>,
    /// Selected capture group on the Save page, by its stable identity.
    ///
    /// By identity rather than by name because the page offers reordering, and
    /// order is this registry's precedence: a position would follow the wrong
    /// group the moment one was raised.
    pub selected_capture_group: Option<crate::product::CaptureGroupId>,
    /// Selected run-set dimension on the PVT, sweeps & variation page, by its
    /// stable identity. Identities survive reordering, which is a command the
    /// page offers, where a position would not.
    pub selected_run_set_dimension: Option<String>,
    /// In-progress edit of the selected dimension's authored value list.
    /// Runtime-only: half-typed values must never be restored as a declared
    /// run space.
    pub run_set_values_draft: Option<(String, String)>,
    /// In-progress edit of the execution budgets, as authored text.
    pub run_set_budget_drafts: Option<crate::workbench::state::RunSetBudgetDrafts>,
    /// In-progress edit of the selected design variable's expression.
    /// Runtime-only: a partially typed expression must never be restored as
    /// authoritative plan data.
    pub design_variable_expression_draft: Option<String>,
    /// In-progress edit of the selected design variable's inclusive bounds, as
    /// `(minimum, maximum)`. Runtime-only for the same reason as the
    /// expression draft: a half-typed bound is not a bound.
    pub design_variable_bounds_draft: Option<(String, String)>,
    /// In-progress edit of the selected saved output's name. Runtime-only: a
    /// half-typed name must never be restored as the record's identity.
    pub saved_output_name_draft: Option<String>,
    /// In-progress edit of the selected saved output's source expression.
    pub saved_output_expression_draft: Option<String>,
    /// Filter over the saved-output registry on the outputs page.
    ///
    /// Runtime-only, like the drafts above but for a different reason: a
    /// filter narrows what one reader is looking at and changes nothing the
    /// plan holds. Restoring it would reopen the project on someone's partial
    /// reading of the registry and make the missing rows look deleted.
    pub saved_output_filter: String,
    /// Filter over the specification registry, runtime-only for the same
    /// reason as the saved-output filter.
    pub specification_filter: String,
    /// Which evidence classes the specification registry shows.
    pub specification_evidence_filter: crate::workbench::state::SpecificationEvidenceFilter,
    /// Runtime-only Jobs manager review context. Job and run records live in
    /// `SimulationState`; this state owns presentation only.
    pub jobs_manager: crate::workbench::state::JobsManagerState,
}

/// Written out because the plan opens on the transient analysis, not on the
/// first row a derived default would pick.
impl Default for SimulateState {
    fn default() -> Self {
        Self {
            plan_removal_review: Default::default(),
            plan_removal_refusal: Default::default(),
            verification_page: crate::workbench::state::VerificationPage::Yield,
            simulation_page: crate::workbench::state::SimulationPage::Analyses,
            verification: Default::default(),
            active_analysis: crate::workbench::state::default_analysis_index(),
            active_analysis_instance: None,
            simulation_surface_scroll_y: 0.0,
            simulation_surface_pending_scroll_delta_y: 0.0,
            simulation_surface_editor_anchor_y: None,
            analysis_lifecycle_status: Default::default(),
            simulation_workflow: None,
            selected_specification: None,
            selected_design_variable: None,
            selected_saved_output: None,
            selected_capture_group: None,
            selected_run_set_dimension: None,
            run_set_values_draft: None,
            run_set_budget_drafts: None,
            design_variable_expression_draft: None,
            design_variable_bounds_draft: None,
            saved_output_name_draft: None,
            saved_output_expression_draft: None,
            saved_output_filter: String::new(),
            specification_filter: String::new(),
            specification_evidence_filter: Default::default(),
            jobs_manager: Default::default(),
        }
    }
}

/// The specification the verification matrix has selected is what another
/// feature reads of simulate.
impl SimulatePort for SimulateState {
    fn selected_specification(&self) -> Option<&str> {
        self.selected_specification.as_deref()
    }
}
