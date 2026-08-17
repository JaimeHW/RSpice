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

use crate::workbench::state::NewSimulationPlanDraft;

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
///
/// The four configuration domains the new plan owns come from `new_plan` rather
/// than staying at whatever [`SimSetupState::create_plan`] mints, because a plan
/// created at defaults is a plan whose corner, model closure, solver options and
/// retention all have to be found and changed afterwards.
///
/// [`SimSetupState::create_plan`]: crate::workbench::app_state::SimSetupState::create_plan
///
/// Inheritance reads the setup that is active *here*, at commit time. That is the
/// only point at which "the active plan" is a definite thing: the manager stays
/// open across frames, and the plan under it can be switched in between.
pub(super) fn commit_create_plan(
    app: &mut RSpiceApp,
    name: &str,
    new_plan: &NewSimulationPlanDraft,
) -> Result<(SimulationPlanId, String), String> {
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    // Read before the transaction, because `create_plan` retires the active plan
    // into the catalog and installs an empty closure and default options in its
    // place. After it returns, the plan being inherited from is no longer active.
    let inherited = InheritedPlanConfiguration {
        model_bindings: setup.model_bindings.clone(),
        options: setup.options.clone(),
        save_policy: setup.save_policy,
    };
    let id = setup.create_plan(name).map_err(|error| error.to_string())?;
    apply_new_plan_configuration(&mut setup, new_plan, inherited)?;
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

/// What the plan that was active before the transaction owned, captured so the
/// three inheritance flags have something to inherit from.
///
/// It is a struct rather than three parameters because it is one snapshot taken
/// at one instant. Three separate arguments could be read at three different
/// points, and the point they are read at is the whole correctness question here:
/// `create_plan` retires the active plan and installs an empty closure, default
/// options and a default policy in its place.
struct InheritedPlanConfiguration {
    model_bindings: Vec<crate::state::model_library::SimulationPlanModelBinding>,
    options: crate::simulation::dialog::SimulationOptions,
    save_policy: crate::workbench::app_state::SimulationSavePolicy,
}

/// Install the created plan's own configuration on `setup`.
///
/// Ordering is load-bearing twice over. Inherited options are installed before
/// the reference point, because `set_reference_pvt` is the one owner of keeping
/// the solver's `TEMP` option, its editor draft and the operating-point
/// temperature equal to the plan's reference temperature — the other way round,
/// an inherited options block would overwrite all three with the temperature of
/// the plan it came from. And the options editor's draft is rebuilt from the
/// options actually installed, because `create_plan` built it from the defaults it
/// minted and an inherited block replaces those.
///
/// The catalog is revalidated at the end. `create_plan` validated the catalog it
/// committed, and every assignment here happens after it returned, so what this
/// installs is checked rather than trusted.
fn apply_new_plan_configuration(
    setup: &mut crate::workbench::app_state::SimSetupState,
    new_plan: &NewSimulationPlanDraft,
    inherited: InheritedPlanConfiguration,
) -> Result<(), String> {
    if new_plan.inherit_solver_options {
        setup.options = inherited.options;
        setup.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&setup.options);
    }
    if new_plan.inherit_model_closure {
        setup.model_bindings = inherited.model_bindings;
    }
    if new_plan.inherit_save_policy {
        setup.save_policy = inherited.save_policy;
    }
    setup.set_reference_pvt(
        new_plan.reference_pvt.process,
        new_plan.reference_pvt.temperature_celsius,
    )?;
    setup
        .validate_plan_catalog()
        .map_err(|error| error.to_string())
}
