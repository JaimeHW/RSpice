//! Cloning a simulation plan, and looking up a plan's name and standing by its id.

use super::*;

pub(super) fn clone_plan_dialog(
    ctx: &egui::Context,
    cx: &mut Cx<'_, SimulateFeature>,
    mut draft: ClonePlanDraft,
) {
    draft.validation_error = validate_clone_plan_draft(cx.model, &draft).err();
    let enabled = draft.validation_error.is_none();
    let source = simulation_plan_catalog_entry(&cx.model.sim_setup, draft.source_plan_id)
        .map_or_else(
            || format!("missing plan {}", draft.source_plan_id),
            |(name, revision, _, _)| format!("{name} · revision {}", revision.get()),
        );
    let choice = Dialog::new("Clone simulation plan", "Create cloned plan")
    .description(
        "Create a separate simulation plan from this one. The clone records its source, and existing results are not changed.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_setting_row(ui, "New plan name", "Unique within this project.", |ui| {
            mono_input(ui, "New plan name", &mut draft.name, ui.available_width().min(330.0));
        });
        workflow_setting_row(
            ui,
            "Source",
            "Frozen source plan and working revision.",
            |ui| {
                ui.label(
                    egui::RichText::new(&source)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
            },
        );
        workflow_setting_row(
            ui,
            "Copy contents",
            "Dependencies remain linked by stable identifiers.",
            |ui| {
                ui.vertical(|ui| {
                    workflow_switch(
                        ui,
                        "Analyses and advanced options",
                        &mut draft.copy_analyses_options,
                    );
                    workflow_switch(
                        ui,
                        "Variables, outputs and specifications",
                        &mut draft.copy_variables_outputs_specs,
                    );
                    workflow_switch(
                        ui,
                        "PVT and model bindings",
                        &mut draft.copy_pvt_model_bindings,
                    );
                    workflow_switch(
                        ui,
                        "Regression baseline ownership",
                        &mut draft.copy_regression_baseline,
                    );
                });
            },
        );
        workflow_setting_row(
            ui,
            "Results",
            "Result datasets are never duplicated with a plan.",
            |ui| {
                super::page_kit::note_line(
                    ui,
                    "none \u{b7} manifests remain linked",
                    super::page_kit::Tone::Dim,
                );
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(cx, choice, draft, commit_clone_plan);
}

pub(super) fn validate_clone_plan_draft(
    model: &Model,
    draft: &ClonePlanDraft,
) -> Result<(), String> {
    let name = crate::workbench::app_state::SimulationPlanName::new(draft.name.clone())
        .map_err(|error| error.to_string())?;
    let key = name.as_str().to_lowercase();
    if model.sim_setup.active_plan_name().as_str().to_lowercase() == key
        || model
            .sim_setup
            .inactive_plans()
            .iter()
            .any(|plan| plan.name().as_str().to_lowercase() == key)
    {
        return Err(format!(
            "A simulation plan named '{}' already exists.",
            name.as_str()
        ));
    }
    let Some((_, _, executing, archived)) =
        simulation_plan_catalog_entry(&model.sim_setup, draft.source_plan_id)
    else {
        return Err(format!(
            "Source simulation plan {} no longer exists.",
            draft.source_plan_id
        ));
    };
    if archived {
        return Err("Restore the archived source plan before cloning it.".to_owned());
    }
    if executing {
        return Err(
            "The source plan owns queued or executing work and cannot be cloned.".to_owned(),
        );
    }
    Ok(())
}

pub fn commit_clone_plan(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &ClonePlanDraft,
) -> Result<String, String> {
    validate_clone_plan_draft(cx.model, draft)?;
    let mut setup = cx.model.sim_setup.clone();
    let mut workspace = cx.model.workspace.clone();
    let current_plan_id = setup.stable_analysis_plan()?.id();
    workspace.content.migrate_active_plan_data(current_plan_id);
    if current_plan_id != draft.source_plan_id {
        setup
            .activate_plan(draft.source_plan_id)
            .map_err(|error| error.to_string())?;
        workspace
            .content
            .sync_legacy_specs_projection(draft.source_plan_id);
    }
    let options = crate::workbench::app_state::SimulationPlanCloneOptions {
        copy_analyses: draft.copy_analyses_options,
        copy_advanced_options: draft.copy_analyses_options,
        copy_variables_outputs_and_specifications: draft.copy_variables_outputs_specs,
        copy_pvt_and_model_bindings: draft.copy_pvt_model_bindings,
        copy_regression_baseline_ownership: draft.copy_regression_baseline,
    };
    let outcome = setup
        .clone_active_plan(draft.name.clone(), options)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .clone_plan_data(
            outcome.source_plan_id,
            outcome.cloned_plan_id,
            outcome.contents.copy_variables_outputs_and_specifications,
            outcome.contents.copy_regression_baseline_ownership,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    cx.model.sim_setup = setup;
    cx.model.workspace = workspace;
    cx.state.active_analysis_instance = first_instance;
    crate::workbench::simulate::invalidate_preflight(cx);
    cx.model.record_plan_receipt(format!(
        "Cloned plan {} from {} at revision {}; result manifests were not duplicated.",
        outcome.cloned_plan_id,
        outcome.source_plan_id,
        outcome.source_revision.get()
    ));
    Ok(format!(
        "Created and activated '{}' with independent plan identity {}.",
        cx.model.sim_setup.active_plan_name(),
        outcome.cloned_plan_id
    ))
}

pub(crate) fn simulation_plan_catalog_entry(
    setup: &crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> Option<(String, crate::product::ObjectRevision, bool, bool)> {
    if let Ok(plan) = setup.stable_analysis_plan()
        && plan.id() == id
    {
        return Some((
            setup.active_plan_name().to_string(),
            plan.revision(),
            plan.has_executing_instances(),
            false,
        ));
    }
    setup
        .inactive_plans()
        .iter()
        .find(|plan| plan.id() == id)
        .map(|plan| {
            (
                plan.name().to_string(),
                plan.revision(),
                plan.analysis_plan().has_executing_instances(),
                plan.archived(),
            )
        })
}
