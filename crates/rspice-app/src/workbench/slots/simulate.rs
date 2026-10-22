//! The simulate feature's dialogs: the analysis plan's removal review, Run
//! details… and the configuration-set manager.

/// What the simulate feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct SimulateState {
    /// Destructive review for removing one record from the analysis plan —
    /// an analysis, a design variable, a saved output or a capture group —
    /// raised only where removal orphans something.
    pub(crate) plan_removal_review: crate::workbench::app::dialogs::plan_removal::PlanRemovalReview,
    /// Why a removal the plan will not perform was refused before it was
    /// staged. The review above asks; this one only states.
    pub(crate) plan_removal_refusal:
        crate::workbench::app::dialogs::plan_removal::PlanRemovalRefusal,

    /// Run details…: the provenance of one retained run.
    pub(crate) run_details: Option<crate::workbench::docks::RunDetailsDialogState>,

    /// Project-owned hierarchy/view configuration manager.
    pub(crate) configuration_sets: crate::workbench::app::ConfigurationSetsDialogState,
}
