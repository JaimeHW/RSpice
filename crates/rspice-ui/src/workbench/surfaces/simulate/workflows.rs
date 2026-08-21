//! The plan-editing workflows: clone a plan, add a design variable, save an output.
//!
//! Each workflow is a draft that is validated before it is committed, so a
//! dialog can be dismissed at any point without leaving the plan half-edited.
//! The draft also reports who consumes the thing being edited, because these
//! edits are shared across every analysis in the plan.

use super::*;

pub(super) fn simulation_workflow_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(workflow) = app.state.workbench.simulation_workflow.clone() else {
        return;
    };
    match workflow {
        SimulationWorkflowDialog::PlanManager(draft) => {
            super::plan_manager::plan_manager_dialog(ctx, app, draft);
        }
        SimulationWorkflowDialog::ClonePlan(draft) => clone_plan_dialog(ctx, app, draft),
        SimulationWorkflowDialog::DesignVariable(draft) => design_variable_dialog(ctx, app, draft),
        SimulationWorkflowDialog::SavedOutput(draft) => saved_output_dialog(ctx, app, draft),
        SimulationWorkflowDialog::RenameAnalysis(draft) => {
            rename_analysis_dialog(ctx, app, draft);
        }
        SimulationWorkflowDialog::AnalysisRunPoints(draft) => {
            super::participation::analysis_run_points_dialog(ctx, app, draft);
        }
        SimulationWorkflowDialog::DesignVariableImport(draft) => {
            super::variable_import::design_variable_import_dialog(ctx, app, draft);
        }
    }
}

/// Name one analysis instance.
///
/// The plan is asked what it would refuse *while the field is being typed in*,
/// so the refusal that would stop the commit is already on screen and the
/// primary control is already disabled. Committing then re-asks the plan: the
/// preview is a courtesy, and the plan remains the only authority.
pub(super) fn rename_analysis_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: RenameAnalysisDraft,
) {
    let (validation_error, clear_refusal, default_label) =
        match app.state.sim_setup.stable_analysis_plan() {
            Ok(plan) => (
                plan.instance_name_refusal(draft.instance_id, &draft.name)
                    .map(|refusal| refusal.to_string()),
                plan.instance_name_clear_refusal(draft.instance_id)
                    .map(|refusal| refusal.to_string()),
                plan.instance(draft.instance_id)
                    .map(|instance| instance.kind().label().to_owned()),
            ),
            Err(error) => (Some(error.clone()), Some(error), None),
        };
    draft.validation_error = validation_error;
    let enabled = draft.validation_error.is_none();
    // Giving the name back is offered only when it would be taken. A control
    // that is present and refused teaches the reader nothing; a disabled one
    // beside the reason teaches them why.
    let can_clear = clear_refusal.is_none();
    let default_note = match (clear_refusal, default_label) {
        (None, Some(label)) => {
            format!("Give up the name. This analysis would be shown as {label} again.")
        }
        (Some(reason), _) => reason,
        (None, None) => "This analysis is no longer in the plan.".to_owned(),
    };
    let choice = Dialog::new(
        "SIMULATION · ANALYSIS IDENTITY",
        "Name analysis",
        "Apply name",
    )
    .description(
        "Name this analysis instance so every receipt, job and result that reports it says which analysis it was, rather than repeating the kind of analysis it is.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .secondary("Use kind label")
    .secondary_enabled(can_clear)
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_setting_row(
            ui,
            "Name",
            "Unique within this plan, ignoring case.",
            |ui| {
                mono_input(ui, &mut draft.name, ui.available_width().min(330.0));
            },
        );
        workflow_setting_row(ui, "Use kind label", &default_note, |ui| {
            ui.label(if can_clear {
                "Available"
            } else {
                "Unavailable"
            });
        });
        workflow_setting_row(ui, "Analysis", "Kind and stable identity.", |ui| {
            ui.label(
                egui::RichText::new(&draft.subject)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            );
        });
        workflow_setting_row(
            ui,
            "Effect",
            "A rename is a plan edit and advances the plan revision.",
            |ui| {
                ui.label("Pinned preflight artifacts go stale; retained datasets are unchanged.");
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    // Clearing is a different intent from renaming, so it takes a different
    // control and a different commit rather than being inferred from an empty
    // field. `finish_workflow_choice` treats a secondary press as "stay open",
    // which is why this is answered before delegating to it.
    if matches!(choice, DialogChoice::Secondary) {
        match commit_clear_analysis_name(app, &draft) {
            Ok(message) => {
                app.state.workbench.simulation_workflow = None;
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Simulation plan updated", message);
            }
            Err(error) => {
                set_workflow_error(&mut draft, error);
                app.state.workbench.simulation_workflow = Some(draft.into());
            }
        }
        return;
    }
    finish_workflow_choice(ctx, app, choice, draft, commit_rename_analysis);
}

/// Return the analysis to its kind label, through the same funnel a rename
/// uses, so both earn a receipt and both invalidate preflight identically.
fn commit_clear_analysis_name(
    app: &mut RSpiceApp,
    draft: &RenameAnalysisDraft,
) -> Result<String, String> {
    let receipt = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .and_then(|plan| {
            plan.clear_instance_name(draft.instance_id)
                .map_err(|error| error.to_string())
        })?;
    super::lifecycle::refresh_analysis_projections(app);
    app.state.workbench.active_analysis_instance = Some(draft.instance_id);
    let message = receipt.detail().to_owned();
    super::lifecycle::record_receipt(&mut app.state, &receipt);
    Ok(message)
}

fn commit_rename_analysis(
    app: &mut RSpiceApp,
    draft: &RenameAnalysisDraft,
) -> Result<String, String> {
    let receipt = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .and_then(|plan| {
            plan.set_instance_name(draft.instance_id, &draft.name)
                .map_err(|error| error.to_string())
        })?;
    super::lifecycle::refresh_analysis_projections(app);
    // Selection is held by identity, and a rename does not mint one, so the
    // renamed analysis stays selected. Saying so here is what keeps a later
    // change from quietly moving selection to a position instead.
    app.state.workbench.active_analysis_instance = Some(draft.instance_id);
    let message = receipt.detail().to_owned();
    super::lifecycle::record_receipt(&mut app.state, &receipt);
    Ok(message)
}

pub(super) fn clone_plan_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: ClonePlanDraft,
) {
    draft.validation_error = validate_clone_plan_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let source = simulation_plan_catalog_entry(&app.state.sim_setup, draft.source_plan_id)
        .map_or_else(
            || format!("missing plan {}", draft.source_plan_id),
            |(name, revision, _, _)| format!("{name} · revision {}", revision.get()),
        );
    let choice = Dialog::new(
        "SIMULATION · EXPLICIT PLAN LINEAGE",
        "Clone simulation plan",
        "Create cloned plan",
    )
    .description(
        "Create a separately owned simulation plan while preserving source lineage and immutable result manifests.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_setting_row(ui, "New plan name", "Unique within this project.", |ui| {
            mono_input(ui, &mut draft.name, ui.available_width().min(330.0));
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
                    workflow_checkbox(
                        ui,
                        "Analyses and advanced options",
                        &mut draft.copy_analyses_options,
                    );
                    workflow_checkbox(
                        ui,
                        "Variables, outputs and specifications",
                        &mut draft.copy_variables_outputs_specs,
                    );
                    workflow_checkbox(
                        ui,
                        "PVT and model bindings",
                        &mut draft.copy_pvt_model_bindings,
                    );
                    workflow_checkbox(
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
                ui.label("none · manifests remain linked");
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_clone_plan);
}

pub(super) fn design_variable_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: DesignVariableDraft,
) {
    draft.validation_error = validate_design_variable_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let consumer_summary = design_variable_consumers(app, &draft);
    let resolved_value = design_variable_from_draft(app, &draft)
        .and_then(|variable| variable.resolved_value_si())
        .map_or_else(
            |_| "unresolved".to_owned(),
            |value| format!("{value:.8e} SI"),
        );
    let name_conflicts = if draft
        .validation_error
        .as_deref()
        .is_some_and(|error| error.to_ascii_lowercase().contains("name"))
    {
        "resolve validation error"
    } else {
        "none"
    };
    let choice = Dialog::new(
        "SIMULATION PLAN · TYPED PARAMETER · DEPENDENCY PREVIEW",
        "Create design variable",
        "Create variable",
    )
    .description(
        "Create a typed, scoped simulation parameter with explicit constraints and deterministic netlist ownership.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .show(ctx, |ui| {
        workflow_split(ui, |ui| {
            workflow_text_field(ui, "Name", &mut draft.name, true);
            workflow_text_field(ui, "Expression", &mut draft.expression, true);
            workflow_select_field(
                ui,
                "design-variable-quantity",
                "Quantity",
                &mut draft.quantity,
                &[
                    "Resistance",
                    "Capacitance",
                    "Voltage",
                    "Current",
                    "Temperature",
                    "Dimensionless",
                ],
            );
            workflow_select_field(
                ui,
                "design-variable-scope",
                "Ownership scope",
                &mut draft.scope,
                &[
                    "Lab characterization · testbench",
                    "Project",
                    "Selected cell",
                    "Selected analysis only",
                ],
            );
            workflow_text_field(ui, "Description", &mut draft.description, false);
        }, |ui| {
            workflow_section_heading(ui, "Constraints and consumers");
            workflow_text_field(ui, "Allowed range", &mut draft.allowed_range, true);
            workflow_select_field(
                ui,
                "design-variable-sweep",
                "Sweep eligibility",
                &mut draft.sweep_eligibility,
                &["Nested sweep + optimization", "Optimization only", "Fixed parameter"],
            );
            workflow_select_field(
                ui,
                "design-variable-override",
                "Override policy",
                &mut draft.override_policy,
                &["Explicit test-local override", "Inherit owner only"],
            );
            ui.add_space(8.0);
            property_row(ui, "Resolved value", &resolved_value);
            property_row(ui, "Name conflicts", name_conflicts);
            property_row(ui, "Prospective consumers", &consumer_summary);
            property_row(ui, "Result effect", "dependent future runs only");
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_design_variable);
}

pub(super) fn saved_output_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: SavedOutputDraft,
) {
    draft.validation_error = validate_saved_output_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let candidate = saved_output_from_draft(app, &draft).ok();
    let preflight = candidate.as_ref().map(|output| {
        app.simulation_controller
            .saved_output_preflight(&app.state, output)
    });
    let inferred_unit = candidate.as_ref().map_or_else(
        || inferred_output_unit(&draft.expression),
        |output| output.inferred_unit(),
    );
    let consumers = saved_output_consumers(app, &draft);
    let expression_validity = preflight.as_ref().map_or_else(
        || {
            draft
                .validation_error
                .clone()
                .unwrap_or_else(|| "candidate output is incomplete".to_owned())
        },
        |report| saved_output_semantic_summary(report.semantic_status()),
    );
    let storage_increment = preflight.as_ref().map_or_else(
        || "indeterminate until the output contract is valid".to_owned(),
        |report| {
            saved_output_storage_summary(
                report.storage_estimate(),
                report.compatible_analysis_count(),
                report.retained_engine_source_analysis_ids().len(),
            )
        },
    );
    let choice = Dialog::new(
        "SIMULATION PLAN · DATA CONTRACT · STORAGE IMPACT",
        "Add saved output or derived expression",
        "Add output",
    )
    .description(
        "Add a validated output contract with explicit analysis compatibility, retention, precision, and streaming behavior.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        workflow_split(ui, |ui| {
            workflow_select_field(
                ui,
                "saved-output-kind",
                "Output kind",
                &mut draft.kind,
                &[
                    "Raw voltage / current",
                    "Derived expression",
                    "Device operating-point quantity",
                    "Noise contributor",
                    "RF port quantity",
                ],
            );
            workflow_text_field(ui, "Name", &mut draft.name, true);
            workflow_multiline_field(ui, "Source or expression", &mut draft.expression);
            workflow_select_field(
                ui,
                "saved-output-analyses",
                "Compatible analyses",
                &mut draft.compatible_analyses,
                &["OP + TRAN + AC", "All compatible analyses", "Selected analysis only"],
            );
        }, |ui| {
            workflow_section_heading(ui, "Save, precision, and provenance");
            workflow_select_field(
                ui,
                "saved-output-save-policy",
                "Save policy",
                &mut draft.save_policy,
                &[
                    "Every accepted point",
                    "Selected + final points",
                    "On demand from retained state",
                    "Failure diagnostics only",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-precision",
                "Stored precision",
                &mut draft.precision,
                &[
                    "f64 / complex128",
                    "f32 display cache + full source precision",
                ],
            );
            workflow_select_field(
                ui,
                "saved-output-streaming",
                "Streaming",
                &mut draft.streaming,
                &["Live plot · adaptive display decimation", "Store only"],
            );
            ui.add_space(8.0);
            property_row(ui, "Inferred unit", inferred_unit);
            property_row(ui, "Estimated increment", &storage_increment);
            property_row(ui, "Expression validity", &expression_validity);
            property_row(ui, "Consumers", &consumers);
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(ctx, app, choice, draft, commit_saved_output);
}

pub(super) fn validate_clone_plan_draft(
    app: &RSpiceApp,
    draft: &ClonePlanDraft,
) -> Result<(), String> {
    let name = crate::workbench::app_state::SimulationPlanName::new(draft.name.clone())
        .map_err(|error| error.to_string())?;
    let key = name.as_str().to_lowercase();
    if app
        .state
        .sim_setup
        .active_plan_name()
        .as_str()
        .to_lowercase()
        == key
        || app
            .state
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
        simulation_plan_catalog_entry(&app.state.sim_setup, draft.source_plan_id)
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

pub(super) fn commit_clone_plan(
    app: &mut RSpiceApp,
    draft: &ClonePlanDraft,
) -> Result<String, String> {
    validate_clone_plan_draft(app, draft)?;
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_plan_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_plan_id);
    if current_plan_id != draft.source_plan_id {
        setup
            .activate_plan(draft.source_plan_id)
            .map_err(|error| error.to_string())?;
        workspace.sync_legacy_specs_projection(draft.source_plan_id);
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
        .clone_plan_data(
            outcome.source_plan_id,
            outcome.cloned_plan_id,
            outcome.contents.copy_variables_outputs_and_specifications,
            outcome.contents.copy_regression_baseline_ownership,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(format!(
            "Cloned plan {} from {} at revision {}; result manifests were not duplicated.",
            outcome.cloned_plan_id,
            outcome.source_plan_id,
            outcome.source_revision.get()
        ));
    Ok(format!(
        "Created and activated '{}' with independent plan identity {}.",
        app.state.sim_setup.active_plan_name(),
        outcome.cloned_plan_id
    ))
}

pub(super) fn simulation_plan_catalog_entry(
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

pub(super) fn design_variable_from_draft(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, String> {
    design_variable_from_draft_categorized(app, draft).map_err(|(_, message)| message)
}

/// [`design_variable_from_draft`], with the failed contract named beside the
/// sentence.
///
/// The selections a draft carries are indices into option lists this surface
/// owns, so an index no list claims is a defect of the *record* rather than of
/// the variable — nothing was asserted about a name, a dimension or a bound. A
/// half-written allowed range is a bound; everything past the conversion is
/// [`DesignVariable::validate_defect`]'s answer, unchanged.
pub(super) fn design_variable_from_draft_categorized(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, (crate::state::DesignVariableDefect, String)> {
    use crate::state::{
        DesignVariable, DesignVariableDefect as Defect, DesignVariableOverridePolicy,
        DesignVariableQuantity, DesignVariableScope, DesignVariableSweepEligibility,
    };

    let record = |message: String| (Defect::Record, message);
    let quantity = DesignVariableQuantity::ALL
        .get(draft.quantity)
        .copied()
        .ok_or_else(|| record("Quantity selection is invalid.".to_owned()))?;
    let scope = match draft.scope {
        0 => DesignVariableScope::Testbench,
        1 => DesignVariableScope::Project,
        2 => DesignVariableScope::SelectedCell {
            cell: app.state.workspace.active_view.clone(),
        },
        3 => DesignVariableScope::SelectedAnalysis {
            analysis_id: app
                .state
                .workbench
                .active_analysis_instance
                .ok_or_else(|| {
                    record("Select an analysis before using analysis-only scope.".to_owned())
                })?,
        },
        _ => return Err(record("Ownership scope selection is invalid.".to_owned())),
    };
    let allowed_range = parse_design_variable_range(&draft.allowed_range)
        .map_err(|message| (Defect::Bounds, message))?;
    let sweep_eligibility = DesignVariableSweepEligibility::ALL
        .get(draft.sweep_eligibility)
        .copied()
        .ok_or_else(|| record("Sweep eligibility selection is invalid.".to_owned()))?;
    let override_policy = DesignVariableOverridePolicy::ALL
        .get(draft.override_policy)
        .copied()
        .ok_or_else(|| record("Override policy selection is invalid.".to_owned()))?;
    DesignVariable::new_defect(
        draft.name.clone(),
        draft.expression.clone(),
        quantity,
        scope,
        draft.description.clone(),
        allowed_range,
        sweep_eligibility,
        override_policy,
    )
}

pub(super) fn parse_design_variable_range(
    value: &str,
) -> Result<Option<crate::state::DesignVariableRange>, String> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    let split = value
        .split_once('…')
        .or_else(|| value.split_once("..="))
        .or_else(|| value.split_once(".."));
    let Some((minimum, maximum)) = split else {
        return Err("Allowed range must be written as minimum … maximum.".to_owned());
    };
    let minimum = minimum.trim();
    let maximum = maximum.trim();
    if minimum.is_empty() || maximum.is_empty() {
        return Err("Allowed range requires both a minimum and maximum.".to_owned());
    }
    Ok(Some(crate::state::DesignVariableRange {
        minimum: minimum.to_owned(),
        maximum: maximum.to_owned(),
    }))
}

pub(super) fn validate_design_variable_draft(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<(), String> {
    let variable = design_variable_from_draft(app, draft)?;
    let plan_id = app.state.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = app.state.workspace.clone();
    workspace
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())
}

pub(super) fn commit_design_variable(
    app: &mut RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<String, String> {
    let variable = design_variable_from_draft(app, draft)?;
    let variable_name = variable.name.clone();
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Created design variable {variable_name}."
        ))
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.invalidate_simulation_preflight();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(receipt.status_line());
    Ok(format!(
        "Created design variable {variable_name} in plan {}.",
        app.state.sim_setup.active_plan_name()
    ))
}

pub(super) fn saved_output_from_draft(
    app: &RSpiceApp,
    draft: &SavedOutputDraft,
) -> Result<crate::state::SavedOutput, String> {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };
    let kind = SavedOutputKind::ALL
        .get(draft.kind)
        .copied()
        .ok_or_else(|| "Output kind selection is invalid.".to_owned())?;
    let compatible_analyses = match draft.compatible_analyses {
        0 => SavedOutputCompatibility::OpTranAc,
        1 => SavedOutputCompatibility::AllCompatibleAnalyses,
        2 => SavedOutputCompatibility::SelectedAnalysis {
            analysis_id: app
                .state
                .workbench
                .active_analysis_instance
                .ok_or_else(|| "Select an analysis before using analysis-only scope.".to_owned())?,
        },
        _ => return Err("Compatible analyses selection is invalid.".to_owned()),
    };
    let save_policy = SavedOutputPolicy::ALL
        .get(draft.save_policy)
        .copied()
        .ok_or_else(|| "Save policy selection is invalid.".to_owned())?;
    let stored_precision = SavedOutputPrecision::ALL
        .get(draft.precision)
        .copied()
        .ok_or_else(|| "Stored precision selection is invalid.".to_owned())?;
    let streaming = SavedOutputStreaming::ALL
        .get(draft.streaming)
        .copied()
        .ok_or_else(|| "Streaming selection is invalid.".to_owned())?;
    SavedOutput::new(
        kind,
        draft.name.clone(),
        draft.expression.clone(),
        compatible_analyses,
        save_policy,
        stored_precision,
        streaming,
    )
}

pub(super) fn validate_saved_output_draft(
    app: &RSpiceApp,
    draft: &SavedOutputDraft,
) -> Result<(), String> {
    let output = saved_output_from_draft(app, draft)?;
    let report = app
        .simulation_controller
        .saved_output_preflight(&app.state, &output);
    if let SavedOutputSemanticStatus::Invalid { reason } = report.semantic_status() {
        return Err(reason.clone());
    }
    let plan_id = app.state.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = app.state.workspace.clone();
    workspace
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub(super) fn commit_saved_output(
    app: &mut RSpiceApp,
    draft: &SavedOutputDraft,
) -> Result<String, String> {
    validate_saved_output_draft(app, draft)?;
    let output = saved_output_from_draft(app, draft)?;
    let output_name = output.name.clone();
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!("Added saved output {output_name}."))
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.invalidate_simulation_preflight();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(receipt.status_line());
    Ok(format!(
        "Added saved output {output_name} to plan {}.",
        app.state.sim_setup.active_plan_name()
    ))
}

/// Adopt one plan-data mutation as a single configuration transaction.
///
/// The workspace and the setup are mutated on clones and adopted only once the
/// change, the configuration validation, and the receipt have all succeeded, so
/// a rejected edit leaves no partial state behind and reports why. Every
/// registry edit routes through here, which is what makes "an edit produces a
/// receipt and invalidates preflight" one rule rather than a convention each
/// page repeats. Returns whether the transaction was adopted, which lets a
/// caller that renamed a record carry the selection onto its new name.
pub(super) fn commit_plan_change(
    app: &mut RSpiceApp,
    plan_id: crate::product::SimulationPlanId,
    detail: &str,
    change: impl FnOnce(
        &mut crate::state::ProjectWorkspace,
        crate::product::SimulationPlanId,
    ) -> Result<(), String>,
) -> bool {
    let mut workspace = app.state.workspace.clone();
    let mut setup = app.state.sim_setup.clone();
    let outcome = change(&mut workspace, plan_id)
        .and_then(|()| {
            workspace
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())
        })
        .and_then(|()| {
            setup
                .commit_active_plan_configuration_change(detail.to_owned())
                .map_err(|error| error.to_string())
        });
    match outcome {
        Ok(receipt) => {
            app.state.workspace = workspace;
            app.state.sim_setup = setup;
            app.invalidate_simulation_preflight();
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(receipt.status_line());
            true
        }
        Err(error) => {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(error);
            false
        }
    }
}

/// `base_copy`, then `base_copy_2`, and so on until the registry has no such
/// record.
///
/// Deriving the name here keeps a duplicate a single transaction rather than a
/// dialog opened over a registry that is about to change. `taken` is supplied
/// by the caller because each registry decides its own collision rule; both of
/// them compare case-insensitively, which is how the plan validates uniqueness.
pub(super) fn unique_copy_name(base: &str, taken: impl Fn(&str) -> bool) -> String {
    let first = format!("{base}_copy");
    if !taken(&first) {
        return first;
    }
    (2u32..)
        .map(|index| format!("{base}_copy_{index}"))
        .find(|candidate| !taken(candidate))
        .unwrap_or(first)
}

pub(super) fn design_variable_consumers(app: &RSpiceApp, draft: &DesignVariableDraft) -> String {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return "plan unavailable".to_owned();
    };
    let selected_only = (draft.scope == 3).then_some(app.state.workbench.active_analysis_instance);
    let labels = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .filter(|instance| selected_only.is_none_or(|selected| selected == Some(instance.id())))
        .map(|instance| instance.kind().code())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        "no enabled analysis consumers".to_owned()
    } else {
        labels.join(" · ")
    }
}

pub(super) fn saved_output_consumers(app: &RSpiceApp, draft: &SavedOutputDraft) -> String {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return "plan unavailable".to_owned();
    };
    let selected = app.state.workbench.active_analysis_instance;
    let labels = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .filter(|instance| match draft.compatible_analyses {
            0 => matches!(
                instance.kind(),
                AnalysisKind::OperatingPoint | AnalysisKind::Transient | AnalysisKind::Ac
            ),
            1 => true,
            2 => selected == Some(instance.id()),
            _ => false,
        })
        .map(|instance| instance.kind().code())
        .collect::<Vec<_>>();
    if labels.is_empty() {
        "no compatible enabled analyses".to_owned()
    } else {
        labels.join(" · ")
    }
}

pub(super) fn inferred_output_unit(expression: &str) -> &'static str {
    let expression = expression.trim();
    if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
    {
        "volts"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
    {
        "amperes"
    } else if expression
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
    {
        "dimensionless"
    } else {
        "resolved from output schema"
    }
}

pub(super) fn saved_output_semantic_summary(status: &SavedOutputSemanticStatus) -> String {
    match status {
        SavedOutputSemanticStatus::Valid { detail } => detail.clone(),
        SavedOutputSemanticStatus::RuntimeBound { reason } => reason.clone(),
        SavedOutputSemanticStatus::Invalid { reason } => reason.clone(),
    }
}

pub(super) fn saved_output_storage_summary(
    estimate: &SavedOutputStorageEstimate,
    compatible_analyses: usize,
    retained_engine_source_analyses: usize,
) -> String {
    if retained_engine_source_analyses > 0 {
        return format!(
            "deferred · shared source ceiling counted at plan level · {compatible_analyses} compatible {}",
            if compatible_analyses == 1 {
                "analysis"
            } else {
                "analyses"
            }
        );
    }
    match estimate {
        SavedOutputStorageEstimate::ExactBytes(bytes) => format!(
            "{} · {compatible_analyses} compatible {}",
            format_storage_bytes(*bytes),
            if compatible_analyses == 1 {
                "analysis"
            } else {
                "analyses"
            }
        ),
        SavedOutputStorageEstimate::Indeterminate { reason } => {
            format!("indeterminate · {reason}")
        }
    }
}

pub(super) fn format_storage_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;
    let bytes_f64 = bytes as f64;
    if bytes_f64 >= TIB {
        format!("{:.2} TiB", bytes_f64 / TIB)
    } else if bytes_f64 >= GIB {
        format!("{:.2} GiB", bytes_f64 / GIB)
    } else if bytes_f64 >= MIB {
        format!("{:.2} MiB", bytes_f64 / MIB)
    } else if bytes_f64 >= KIB {
        format!("{:.2} KiB", bytes_f64 / KIB)
    } else {
        format!("{bytes} B")
    }
}

pub(super) fn finish_workflow_choice<D>(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    choice: DialogChoice,
    mut draft: D,
    commit: fn(&mut RSpiceApp, &D) -> Result<String, String>,
) where
    D: Clone + Into<SimulationWorkflowDialog> + WorkflowDraft,
{
    match choice {
        DialogChoice::Primary => match commit(app, &draft) {
            Ok(message) => {
                app.state.workbench.simulation_workflow = None;
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Simulation plan updated", message);
            }
            Err(error) => {
                set_workflow_error(&mut draft, error);
                app.state.workbench.simulation_workflow = Some(draft.into());
            }
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.simulation_workflow = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {
            app.state.workbench.simulation_workflow = Some(draft.into());
        }
    }
}

pub(super) trait WorkflowDraft {
    fn set_error(&mut self, error: String);
}

impl WorkflowDraft for ClonePlanDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for DesignVariableDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for SavedOutputDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for RenameAnalysisDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for AnalysisRunPointsDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for crate::workbench::state::DesignVariableImportDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

pub(super) fn set_workflow_error(draft: &mut impl WorkflowDraft, error: String) {
    draft.set_error(error);
}

impl From<ClonePlanDraft> for SimulationWorkflowDialog {
    fn from(draft: ClonePlanDraft) -> Self {
        Self::ClonePlan(draft)
    }
}

impl From<DesignVariableDraft> for SimulationWorkflowDialog {
    fn from(draft: DesignVariableDraft) -> Self {
        Self::DesignVariable(draft)
    }
}

impl From<AnalysisRunPointsDraft> for SimulationWorkflowDialog {
    fn from(draft: AnalysisRunPointsDraft) -> Self {
        Self::AnalysisRunPoints(draft)
    }
}

impl From<crate::workbench::state::DesignVariableImportDraft> for SimulationWorkflowDialog {
    fn from(draft: crate::workbench::state::DesignVariableImportDraft) -> Self {
        Self::DesignVariableImport(draft)
    }
}

impl From<SavedOutputDraft> for SimulationWorkflowDialog {
    fn from(draft: SavedOutputDraft) -> Self {
        Self::SavedOutput(draft)
    }
}

impl From<RenameAnalysisDraft> for SimulationWorkflowDialog {
    fn from(draft: RenameAnalysisDraft) -> Self {
        Self::RenameAnalysis(draft)
    }
}

pub(super) fn workflow_split(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    if ui.available_width() >= 620.0 {
        let width = ui.available_width();
        let divider = 1.0;
        let pane_width = ((width - divider) * 0.5).max(1.0);
        let response = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, left)
            });
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, right)
            });
        });
        let divider_x = response.response.rect.left() + pane_width;
        ui.painter().vline(
            divider_x,
            response.response.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    } else {
        let left_response = workflow_split_pane(ui, left);
        ui.painter().hline(
            left_response.rect.x_range(),
            left_response.rect.bottom(),
            Stroke::new(1.0, t.color.border_strong),
        );
        workflow_split_pane(ui, right);
    }
}

pub(super) fn workflow_split_pane(ui: &mut Ui, body: impl FnOnce(&mut Ui)) -> egui::Response {
    let outer_width = ui.available_width();
    egui::Frame::new()
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width((outer_width - 20.0).max(1.0));
            ui.spacing_mut().item_spacing.y = 6.0;
            body(ui);
        })
        .response
}

pub(super) fn workflow_setting_row(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    value: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let response = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width((width - 24.0).max(1.0));
            if width >= 560.0 {
                ui.columns(2, |columns| {
                    columns[0].label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
                    );
                    columns[0].label(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    value(&mut columns[1]);
                });
            } else {
                ui.label(
                    egui::RichText::new(title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold)),
                );
                ui.label(
                    egui::RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                ui.add_space(5.0);
                value(ui);
            }
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

pub(super) fn workflow_text_field(ui: &mut Ui, label: &str, value: &mut String, monospace: bool) {
    workflow_field_label(ui, label);
    if monospace {
        mono_input(ui, value, ui.available_width());
    } else {
        let height = Tokens::get(ui.ctx()).metrics.ctl_h;
        ui.add_sized(
            vec2(ui.available_width(), height),
            egui::TextEdit::singleline(value).margin(egui::Margin::symmetric(8, 4)),
        );
    }
}

pub(super) fn workflow_multiline_field(ui: &mut Ui, label: &str, value: &mut String) {
    workflow_field_label(ui, label);
    ui.add_sized(
        vec2(ui.available_width(), 74.0),
        egui::TextEdit::multiline(value)
            .font(egui::TextStyle::Monospace)
            .margin(egui::Margin::symmetric(8, 6)),
    );
}

pub(super) fn workflow_select_field(
    ui: &mut Ui,
    salt: &str,
    label: &str,
    value: &mut usize,
    options: &[&str],
) {
    workflow_field_label(ui, label);
    let choices = options
        .iter()
        .map(|option| (*option).to_owned())
        .collect::<Vec<_>>();
    *value = (*value).min(choices.len().saturating_sub(1));
    let current = choices.get(*value).map_or("", String::as_str);
    if let Some(selected) = select(ui, salt, label, current, &choices, ui.available_width()) {
        *value = selected;
    }
}

pub(super) fn workflow_field_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

pub(super) fn workflow_section_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 25.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
}

/// One boolean, at the module's row height and against the column's leading
/// edge.
///
/// The row spans the whole track so a column of these keeps one vertical
/// rhythm, but the checkbox itself keeps its natural compact width at the left.
/// `add_sized` centres a widget in the space it is given, which put labels of
/// different lengths at different left edges and read as a broken list rather
/// than as several choices about the same thing.
pub(super) fn workflow_checkbox(ui: &mut Ui, label: &str, value: &mut bool) {
    let height = Tokens::get(ui.ctx()).metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.checkbox(value, label);
        },
    );
}

pub(super) fn workflow_validation_message(ui: &mut Ui, error: Option<&str>) {
    if let Some(error) = error {
        let t = Tokens::get(ui.ctx());
        let response = egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    )
                    .wrap(),
                );
            })
            .response;
        ui.painter().hline(
            response.rect.x_range(),
            response.rect.top(),
            Stroke::new(1.0, t.color.border),
        );
    }
}
