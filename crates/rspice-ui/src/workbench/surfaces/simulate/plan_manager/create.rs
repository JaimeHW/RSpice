//! Creating a fresh root plan.
//!
//! One of the five routes the plan-manager shell dispatches to. The rendering
//! here cannot reach the application: it takes the draft and the catalog
//! projection and reports an action, and [`commit_create_plan`] is the only
//! thing in this file handed the whole application. That split is the point —
//! a route's render path has no way to mutate the run controller or the
//! workspace, so whole-application access lives in one named function per
//! route instead of being spread across five render paths.

use super::*;

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    _records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · NEW PLAN · FRESH ROOT IDENTITY",
        "New simulation plan",
        "Create plan",
    )
    .description(
        "A new plan is created as a root with its own stable identity, and becomes the active editable plan.",
    )
    .size(DialogSize::SimulationWorkflow)
    .ghost("Cancel")
    .primary_enabled(!draft.name.trim().is_empty())
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("New plan name");
            mono_input(ui, &mut draft.name, ui.available_width().min(360.0));
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyCreate),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// Create a plan and activate it, moving the setup and the workspace payload
/// together or not at all.
pub(super) fn commit_create_plan(
    app: &mut RSpiceApp,
    name: &str,
) -> Result<(SimulationPlanId, String), String> {
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    let id = setup.create_plan(name).map_err(|error| error.to_string())?;
    workspace.migrate_inactive_plan_data(id);
    workspace.sync_legacy_specs_projection(id);
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok((
        id,
        format!("Created and activated fresh root plan '{name}' with identity {id}."),
    ))
}
