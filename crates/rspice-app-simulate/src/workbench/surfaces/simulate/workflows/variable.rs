//! Adding or editing a design variable.

use super::*;

pub(super) fn design_variable_dialog(
    ctx: &egui::Context,
    cx: &mut Cx<'_, SimulateFeature>,
    mut draft: DesignVariableDraft,
) {
    draft.validation_error = validate_design_variable_draft(cx, &draft).err();
    let enabled = draft.validation_error.is_none();
    let resolved_value = design_variable_from_draft(&draft)
        .and_then(|variable| resolved_in_plan(cx.model, variable))
        .map_or_else(
            |_| "unresolved".to_owned(),
            // Engineering notation: `1.00000000e4 SI` spent eight mantissa
            // digits and an exponent on `10k`, which is what the author typed
            // and how every other surface shows the same quantity. The display
            // spelling, not the deck's: this row is read, not parsed, and the
            // deck spelling put `1.000Meg SI` in front of a reader who writes
            // that quantity `1.000M`.
            |value| {
                format!(
                    "{} SI",
                    crate::state::property_types::format_engineering_display(value)
                )
            },
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
    let choice = Dialog::new("Create design variable", "Create variable")
    .description(
        "Create a simulation parameter every analysis in this plan reads. Its value may name other variables.",
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
            workflow_text_field(ui, "Description", &mut draft.description, false);
        }, |ui| {
            workflow_section_heading(ui, "Constraints");
            workflow_text_field(ui, "Allowed range", &mut draft.allowed_range, true);
            workflow_switch(ui, "Tunable", &mut draft.tunable);
            ui.add_space(8.0);
            property_row(ui, "Resolved value", &resolved_value);
            property_row(ui, "Name conflicts", name_conflicts);
            property_row(ui, "Result effect", "dependent future runs only");
        });
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(cx, choice, draft, commit_design_variable);
}

/// The value `variable` resolves to beside the plan's other variables, as the
/// run would resolve it.
fn resolved_in_plan(model: &Model, variable: crate::state::DesignVariable) -> Result<f64, String> {
    let mut variables = model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| model.workspace.content.plan_data(plan.id()))
        .map(|payload| payload.design_variables.clone())
        .unwrap_or_default();
    variables.push(variable);
    let resolved = rspice_simulation_contract::design_variable_resolution::resolve_design_variables(
        &variables,
    );
    resolved
        .value(variables.len() - 1)
        .cloned()
        .unwrap_or_else(|| Err("unresolved".to_owned()))
}

pub(crate) fn design_variable_from_draft(
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, String> {
    design_variable_from_draft_categorized(draft).map_err(|(_, message)| message)
}

/// [`design_variable_from_draft`], with the failed contract named beside the
/// sentence.
///
/// The selections a draft carries are indices into option lists this surface
/// owns, so an index no list claims is a defect of the *record* rather than of
/// the variable — nothing was asserted about a name, a dimension or a bound. A
/// half-written allowed range is a bound; everything past the conversion is
/// [`DesignVariable::validate_defect`]'s answer, unchanged.
pub(crate) fn design_variable_from_draft_categorized(
    draft: &DesignVariableDraft,
) -> Result<crate::state::DesignVariable, (crate::state::DesignVariableDefect, String)> {
    use crate::state::{DesignVariable, DesignVariableDefect as Defect, DesignVariableQuantity};

    let record = |message: String| (Defect::Record, message);
    let quantity = DesignVariableQuantity::ALL
        .get(draft.quantity)
        .copied()
        .ok_or_else(|| record("Quantity selection is invalid.".to_owned()))?;
    let allowed_range = parse_design_variable_range(&draft.allowed_range)
        .map_err(|message| (Defect::Bounds, message))?;
    DesignVariable::new_defect(
        draft.name.clone(),
        draft.expression.clone(),
        quantity,
        draft.description.clone(),
        allowed_range,
        draft.tunable,
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
    cx: &Cx<'_, SimulateFeature>,
    draft: &DesignVariableDraft,
) -> Result<(), String> {
    let variable = design_variable_from_draft(draft)?;
    let plan_id = cx.model.sim_setup.stable_analysis_plan()?.id();
    let mut workspace = cx.model.workspace.clone();
    workspace
        .content
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())
}

pub(crate) fn commit_design_variable(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &DesignVariableDraft,
) -> Result<String, String> {
    let variable = design_variable_from_draft(draft)?;
    let variable_name = variable.name.clone();
    let step = cx.model.setup_snapshot(SetupParts::PLAN_DATA);
    let mut setup = cx.model.sim_setup.clone();
    let mut workspace = cx.model.workspace.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    workspace
        .content
        .add_design_variable(plan_id, variable)
        .map_err(|error| error.to_string())?;
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Created design variable {variable_name}."
        ))
        .map_err(|error| error.to_string())?;
    cx.model.sim_setup = setup;
    cx.model.workspace = workspace;
    cx.model
        .record_setup_change(cx.state.simulation_page.label(), step);
    crate::workbench::simulate::invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(format!(
        "Created design variable {variable_name} in plan {}.",
        cx.model.sim_setup.active_plan_name()
    ))
}
