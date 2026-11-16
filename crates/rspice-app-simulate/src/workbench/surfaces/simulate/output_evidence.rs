//! The dataset the plan's pages read: the active run, when it is this plan's.

/// The active retained dataset only when its frozen receipt belongs to the
/// active simulation plan.
///
/// Legacy datasets predate receipt ownership and remain usable for project
/// compatibility, but every current prepared run is fail-closed: a manual
/// deck or another plan can never become evidence merely because it is the
/// selected result.
pub(super) fn selected_plan_dataset(
    model: &crate::workbench::app_state::Model,
) -> Option<&crate::state::SimulationRun> {
    let plan_id = model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())?;
    model.simulation.active_run_for_plan(plan_id)
}
