//! Exporting a plan to a portable package and importing one back.
//!
//! One of the five routes the plan-manager shell dispatches to. Both directions
//! share this file because they share the package format: the version and format
//! tags below are the compatibility contract, and a reader that accepted a
//! package it did not recognise would import a plan it could not honour.
//!
//! An import mints a new local identity and retains the source lineage. It does
//! not copy result references — a run's receipt names the plan it was dispatched
//! from, and importing a plan elsewhere cannot manufacture runs of it.

use super::*;

const SIMULATION_PLAN_PACKAGE_VERSION: u16 = 1;
const SIMULATION_PLAN_PACKAGE_FORMAT: &str = "rspice.simulation-plan";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PortableSimulationPlanPackage {
    format: String,
    version: u16,
    plan: crate::workbench::app_state::SimulationPlanImportDocument,
    payload: crate::state::SimulationPlanPayload,
}

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    _records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    if draft.mode == SimulationPlanManagerMode::Import {
        return import(ctx, draft);
    }
    export(ctx, draft)
}

fn export(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
) -> Option<PlanManagerAction> {
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · EXPORT PLAN · PORTABLE PACKAGE",
        "Export simulation plan",
        "Copy JSON",
    )
    .description(
        "The complete plan and its plan-owned payload, as a portable package. Exporting reads the catalog and changes nothing.",
    )
    .size(DialogSize::SimulationWorkflow)
    .ghost("Close export")
    .show(ctx, |ui| {
        ui.label("Portable RSpice simulation-plan JSON");
        ui.add(
            egui::TextEdit::multiline(&mut draft.exchange_text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .desired_rows(12)
                .desired_width(f32::INFINITY)
                .interactive(false),
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::CopyExport),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

fn import(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
) -> Option<PlanManagerAction> {
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · IMPORT PLAN · NEW LOCAL IDENTITY",
        "Import simulation plan",
        "Import plan",
    )
    .description(
        "An imported plan is created under a new local identity and activated. Its source lineage is retained; no result reference is copied.",
    )
    .size(DialogSize::SimulationWorkflow)
    .ghost("Cancel")
    .primary_enabled(!draft.name.trim().is_empty() && !draft.exchange_text.trim().is_empty())
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Imported plan name");
            mono_input(ui, &mut draft.name, ui.available_width().min(360.0));
        });
        ui.label("Paste portable RSpice simulation-plan JSON");
        ui.add(
            egui::TextEdit::multiline(&mut draft.exchange_text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .desired_rows(12)
                .desired_width(f32::INFINITY),
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyImport),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

pub(in crate::workbench::surfaces::simulate) fn export_simulation_plan_package(
    app: &RSpiceApp,
    id: SimulationPlanId,
) -> Result<String, String> {
    let plan = app
        .state
        .sim_setup
        .export_plan(id)
        .map_err(|error| error.to_string())?;
    let payload = app
        .state
        .workspace
        .plan_data(id)
        .cloned()
        .ok_or_else(|| format!("Simulation plan {id} has no plan-owned payload to export."))?;
    serde_json::to_string_pretty(&PortableSimulationPlanPackage {
        format: SIMULATION_PLAN_PACKAGE_FORMAT.to_owned(),
        version: SIMULATION_PLAN_PACKAGE_VERSION,
        plan,
        payload,
    })
    .map_err(|error| format!("Could not serialize simulation plan {id}: {error}"))
}

pub(in crate::workbench::surfaces::simulate) fn commit_import_simulation_plan(
    app: &mut RSpiceApp,
    json: &str,
    name: &str,
) -> Result<(SimulationPlanId, String), String> {
    if json.trim().is_empty() {
        return Err("Paste a portable RSpice simulation-plan package before importing.".to_owned());
    }
    let mut package: PortableSimulationPlanPackage = serde_json::from_str(json)
        .map_err(|error| format!("Simulation-plan package JSON is invalid: {error}"))?;
    if package.format != SIMULATION_PLAN_PACKAGE_FORMAT {
        return Err(format!(
            "Unsupported simulation-plan package format '{}'.",
            package.format
        ));
    }
    if package.version != SIMULATION_PLAN_PACKAGE_VERSION {
        return Err(format!(
            "Unsupported simulation-plan package version {}; this build accepts version {}.",
            package.version, SIMULATION_PLAN_PACKAGE_VERSION
        ));
    }
    package.plan.name = crate::workbench::app_state::SimulationPlanName::new(name.to_owned())
        .map_err(|error| error.to_string())?;
    app.state
        .model_library_manager
        .validate_simulation_plan_bindings(&package.plan.model_bindings)
        .map_err(|error| format!("Imported model bindings require review: {error}"))?;

    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    let outcome = setup
        .import_plan(package.plan)
        .map_err(|error| error.to_string())?;
    workspace
        .import_plan_data(
            outcome.cloned_plan_id,
            &package.payload,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    crate::io::ProjectExecutionContext::from_state(
        workspace.project.id(),
        &setup,
        &app.state.model_library_manager,
    )?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let id = outcome.cloned_plan_id;
    let imported_name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok((
        id,
        format!(
            "Imported and activated '{imported_name}' as new local identity {id}; source lineage was retained and result references were not copied."
        ),
    ))
}
