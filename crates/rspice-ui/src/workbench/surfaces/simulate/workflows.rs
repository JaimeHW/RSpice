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
        SimulationWorkflowDialog::ClonePlan(draft) => clone_plan_dialog(ctx, app, draft),
        SimulationWorkflowDialog::DesignVariable(draft) => design_variable_dialog(ctx, app, draft),
        SimulationWorkflowDialog::SavedOutput(draft) => saved_output_dialog(ctx, app, draft),
    }
}

pub(super) fn clone_plan_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: ClonePlanDraft,
) {
    draft.validation_error = validate_clone_plan_draft(app, &draft).err();
    let enabled = draft.validation_error.is_none();
    let source = format!(
        "{} · revision {}",
        app.state.sim_setup.active_plan_name(),
        app.state
            .sim_setup
            .stable_analysis_plan()
            .map_or(0, |plan| plan.revision().get())
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
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    if plan.has_executing_instances() {
        return Err(
            "The active plan owns queued or executing work and cannot be cloned.".to_owned(),
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
    let source_plan_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(source_plan_id);
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
    app.state.workbench.analysis_lifecycle_status = format!(
        "Cloned plan {} from {} at revision {}; result manifests were not duplicated.",
        outcome.cloned_plan_id,
        outcome.source_plan_id,
        outcome.source_revision.get()
    );
    Ok(format!(
        "Created and activated '{}' with independent plan identity {}.",
        app.state.sim_setup.active_plan_name(),
        outcome.cloned_plan_id
    ))
}

pub(super) fn design_variable_from_draft(
    app: &RSpiceApp,
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, String> {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility,
    };

    let quantity = DesignVariableQuantity::ALL
        .get(draft.quantity)
        .copied()
        .ok_or_else(|| "Quantity selection is invalid.".to_owned())?;
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
                .ok_or_else(|| "Select an analysis before using analysis-only scope.".to_owned())?,
        },
        _ => return Err("Ownership scope selection is invalid.".to_owned()),
    };
    let allowed_range = parse_design_variable_range(&draft.allowed_range)?;
    let sweep_eligibility = DesignVariableSweepEligibility::ALL
        .get(draft.sweep_eligibility)
        .copied()
        .ok_or_else(|| "Sweep eligibility selection is invalid.".to_owned())?;
    let override_policy = DesignVariableOverridePolicy::ALL
        .get(draft.override_policy)
        .copied()
        .ok_or_else(|| "Override policy selection is invalid.".to_owned())?;
    DesignVariable::new(
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
    app.state.workbench.analysis_lifecycle_status = receipt.status_line();
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
    app.state.workbench.analysis_lifecycle_status = receipt.status_line();
    Ok(format!(
        "Added saved output {output_name} to plan {}.",
        app.state.sim_setup.active_plan_name()
    ))
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
) -> String {
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
    let bytes_f64 = bytes as f64;
    if bytes_f64 >= GIB {
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

impl From<SavedOutputDraft> for SimulationWorkflowDialog {
    fn from(draft: SavedOutputDraft) -> Self {
        Self::SavedOutput(draft)
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

pub(super) fn workflow_checkbox(ui: &mut Ui, label: &str, value: &mut bool) {
    ui.add_sized(
        vec2(ui.available_width(), Tokens::get(ui.ctx()).metrics.row_h),
        egui::Checkbox::new(value, label),
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
