//! Opening, renaming, archiving and restoring a plan.
//!
//! One of the five routes the plan-manager shell dispatches to. Two of its
//! modes render — renaming and confirming an archive — while opening and
//! restoring commit straight from the browse surface without a dialog of their
//! own.
//!
//! The four commits below differ in how much they are allowed to touch, and the
//! difference is deliberate. Opening and renaming invalidate the simulation
//! preflight, so they are handed the application. Archiving and restoring only
//! move a flag in the plan catalog, so they are handed the catalog and could not
//! reach the run controller if they tried.

use super::*;

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    if draft.mode == SimulationPlanManagerMode::ConfirmArchive {
        return confirm_archive(ctx, draft, records);
    }
    rename(ctx, draft)
}

fn rename(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
) -> Option<PlanManagerAction> {
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · RENAME PLAN · IDENTITY PRESERVED",
        "Rename simulation plan",
        "Rename plan",
    )
    .description(
        "Renaming changes the plan's name only. Its stable identity, its revision, and every result reference that names it are preserved.",
    )
    .size(DialogSize::SimulationWorkflow)
    .ghost("Cancel")
    .primary_enabled(!draft.name.trim().is_empty())
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Plan name");
            mono_input(ui, &mut draft.name, ui.available_width().min(360.0));
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyRename),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

fn confirm_archive(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id);
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · ARCHIVE PLAN · RECOVERABLE",
        "Archive simulation plan",
        "Archive plan",
    )
    .description(
        "Archiving is reversible, and the catalog refuses it on the active plan. The plan's configuration and its result provenance both remain recoverable.",
    )
    .size(DialogSize::Transaction)
    .destructive()
    .ghost("Cancel")
    .primary_enabled(selected.is_some())
    .show(ctx, |ui| {
        if let Some(selected) = selected {
            ui.label(format!(
                "Archive '{}'? The plan remains recoverable and its {} immutable result reference{} remain unchanged.",
                selected.name,
                selected.results,
                plan_plural_suffix(selected.results)
            ));
        }
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ConfirmArchive),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// Open a plan: the setup and the plan-owned workspace payload move together,
/// on a copy installed only once the whole switch validates.
pub(super) fn commit_activate_plan(
    app: &mut RSpiceApp,
    id: SimulationPlanId,
) -> Result<String, String> {
    if app.state.sim_setup.stable_analysis_plan()?.id() == id {
        return Ok(format!(
            "'{}' remains the active editable plan.",
            app.state.sim_setup.active_plan_name()
        ));
    }
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    workspace.migrate_inactive_plan_data(id);
    setup.activate_plan(id).map_err(|error| error.to_string())?;
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
    Ok(format!(
        "Opened '{name}' atomically with its complete plan-owned configuration."
    ))
}

pub(super) fn commit_rename_plan(
    app: &mut RSpiceApp,
    id: SimulationPlanId,
    name: &str,
) -> Result<String, String> {
    let mut setup = app.state.sim_setup.clone();
    setup
        .rename_plan(id, name)
        .map_err(|error| error.to_string())?;
    setup
        .validate_plan_catalog()
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.invalidate_simulation_preflight();
    Ok(format!(
        "Renamed simulation plan {id} to '{}'; identity and immutable result references were preserved.",
        name.trim()
    ))
}

/// Archiving and restoring reach no further than the plan catalog.
///
/// They take the catalog rather than the whole application, unlike the two
/// commits above: neither invalidates the preflight nor touches the run
/// controller, so neither has any business being handed a value it could mutate
/// those through.
pub(super) fn commit_archive_plan(
    setup: &mut crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> Result<String, String> {
    let mut candidate = setup.clone();
    candidate
        .archive_plan(id)
        .map_err(|error| error.to_string())?;
    *setup = candidate;
    Ok(format!(
        "Archived simulation plan {id}; configuration and result provenance remain recoverable."
    ))
}

pub(super) fn commit_restore_plan(
    setup: &mut crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> Result<String, String> {
    let mut candidate = setup.clone();
    candidate
        .restore_plan(id)
        .map_err(|error| error.to_string())?;
    *setup = candidate;
    Ok(format!(
        "Restored simulation plan {id} to the working catalog."
    ))
}
