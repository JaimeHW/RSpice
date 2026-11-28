//! Design variables.
//!
//! The typed parameters this plan resolves. The registry is the whole page's
//! subject: a selected row opens its record beside the table, and every
//! mutation goes through a plan-configuration transaction so the change
//! produces a receipt and invalidates preflight.

use egui::Ui;

use crate::product::{DesignVariableId, SimulationPlanId};
use crate::state::DesignVariableQuantity;
use crate::state::workspace::SimulationPlanPayload;
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;

use crate::ui::widgets::{mono_input, select};

use super::page_kit::{
    RowPress, Tone, card, card_body, card_head_row, card_note, field_pair, ledger_head, ledger_row,
    rule_row,
};
use super::workflows::{commit_plan_change, unique_copy_name};

const REGISTRY_COLUMNS: [f32; 6] = [0.14, 0.12, 0.18, 0.20, 0.18, 0.18];

pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let Some((plan_id, payload)) = plan_payload(cx.model) else {
        card(
            ui,
            "Variable registry",
            Some(("no stable plan", Tone::Error)),
            |ui| {
                card_note(
                    ui,
                    "This workspace has no stable analysis plan yet, so it owns no variables. \
                     Add an analysis first; the plan it creates is what a variable belongs to.",
                );
            },
        );
        return;
    };
    registry(ui, cx, plan_id, &payload);
    selected_record(ui, cx, &payload);
}

fn plan_payload(model: &Model) -> Option<(SimulationPlanId, SimulationPlanPayload)> {
    let plan_id = model.sim_setup.stable_analysis_plan().ok()?.id();
    let payload = model.workspace.content.plan_data(plan_id)?.clone();
    Some((plan_id, payload))
}

fn registry(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    plan_id: SimulationPlanId,
    payload: &SimulationPlanPayload,
) {
    let variables = &payload.design_variables;
    // Every value is resolved the way the run resolves it, so the number
    // beside the text is the number the deck gives the engine.
    let resolved =
        rspice_simulation_contract::design_variable_resolution::resolve_design_variables(variables);
    let digits = usize::from(
        cx.model
            .preferences
            .result_presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let selected = cx.state.selected_design_variable.clone();
    let selected_index = selected
        .as_ref()
        .and_then(|name| variables.iter().position(|variable| &variable.name == name));
    // The optimizer tunes only ticked variables; a run-set axis may sweep
    // any of them.
    let status = format!(
        "{} \u{b7} {} tunable",
        match variables.len() {
            1 => "1 variable".to_owned(),
            count => format!("{count} variables"),
        },
        variables.iter().filter(|variable| variable.tunable).count()
    );
    // A spec sheet the picker has finished reading is adopted before the head
    // is drawn, so the rows it adds are in this frame's registry.
    super::variable_import::poll_pending_import(ui.ctx(), cx, plan_id);

    let mut duplicate = false;
    let mut remove = false;
    let mut import = false;
    let mut pick = None;
    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Variable registry",
                Some((status.as_str(), Tone::Neutral)),
                |ui| {
                    import = Button::new("Import…")
                        .show(ui)
                        .on_hover_text(
                            "Add every variable in a spec-sheet CSV as one transaction. The \
                             header row names the columns; a row that fails is reported by line \
                             and nothing is imported.",
                        )
                        .clicked();
                    remove = Button::new("Remove")
                        .enabled(selected_index.is_some())
                        .show(ui)
                        .clicked();
                    duplicate = Button::new("Duplicate")
                        .enabled(selected_index.is_some())
                        .show(ui)
                        .clicked();
                },
            );
        },
        |ui| {
            ledger_head(
                ui,
                &REGISTRY_COLUMNS,
                &[
                    "Name",
                    "Quantity",
                    "Expression",
                    "Resolved",
                    "Bounds",
                    "Tunable",
                ],
            );
            if variables.is_empty() {
                ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        ("No design variables", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                    RowPress::Ignored,
                );
            }
            for (index, variable) in variables.iter().enumerate() {
                let bounds = variable.allowed_range.as_ref().map_or_else(
                    || "unbounded".to_owned(),
                    |range| format!("{} … {}", range.minimum, range.maximum),
                );
                let (value, value_tone) = match resolved.value(index) {
                    Some(Ok(value)) => (
                        resolved_text(*value, variable.quantity, digits),
                        Tone::Neutral,
                    ),
                    Some(Err(error)) => (error.clone(), Tone::Error),
                    None => (String::new(), Tone::Neutral),
                };
                let row = ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        (variable.name.as_str(), Tone::Text),
                        (variable.quantity.label(), Tone::Neutral),
                        (variable.expression.as_str(), Tone::Neutral),
                        (value.as_str(), value_tone),
                        (bounds.as_str(), Tone::Neutral),
                        (if variable.tunable { "yes" } else { "no" }, Tone::Neutral),
                    ],
                    selected_index == Some(index),
                    RowPress::Taken,
                );
                // An error is longer than its cell; the whole sentence is one
                // hover away.
                let row = if value_tone == Tone::Error {
                    row.on_hover_text(format!("{}: {value}", variable.name))
                } else {
                    row
                };
                if row.clicked() {
                    pick = Some(variable.name.clone());
                }
            }
            card_note(
                ui,
                "Every analysis in this plan reads the same value. A bound is hard and is never \
                 clamped: \
                 the same check refuses the record at construction, refuses to emit the \
                 deck's parameter lines and refuses to validate the stored simulation \
                 configuration. An out-of-range value therefore cannot reach a run by any route. \
                 The optimizer tunes only a variable ticked Tunable, within its bounds; a run-set \
                 axis may sweep any variable.",
            );
        },
    );

    if let Some(name) = pick {
        // A draft belongs to the row it was typed into. Selecting another row
        // must not carry a half-typed expression or bound onto it.
        if cx.state.selected_design_variable.as_deref() != Some(name.as_str()) {
            cx.state.design_variable_expression_draft = None;
            cx.state.design_variable_bounds_draft = None;
        }
        cx.state.selected_design_variable = Some(name);
    }
    if import {
        super::variable_import::import_from_file(ui.ctx(), cx);
    }
    let mut removal = None;
    if let Some(index) = selected_index {
        let name = variables[index].name.clone();
        let id = variables[index].id;
        if duplicate {
            let copy = unique_copy_name(&name, |candidate| {
                variables
                    .iter()
                    .any(|variable| variable.name.eq_ignore_ascii_case(candidate))
            });
            let detail = format!("Duplicated design variable {name} as {copy}.");
            let committed = copy.clone();
            if commit_plan_change(cx, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .content
                    .duplicate_design_variable(plan_id, id, copy)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            }) {
                // The copy carries the original's whole typed contract, so the
                // record editor should open on it — that is the field the copy
                // exists to change.
                cx.state.design_variable_expression_draft = None;
                cx.state.design_variable_bounds_draft = None;
                cx.state.selected_design_variable = Some(committed);
            }
        } else if remove {
            // One click and one step of Undo. A variable whose value named
            // this one says so on its own row, and Check says the same.
            removal = Some((id, name));
        }
    }
    if let Some((id, name)) = removal {
        let detail = format!("Removed design variable {name}.");
        // A refused removal leaves the variable in the registry, so the
        // selection has to survive with it — clearing regardless would
        // close the editor on a row that is still there.
        if commit_plan_change(cx, plan_id, &detail, move |workspace, plan_id| {
            workspace
                .content
                .remove_design_variable(plan_id, id)
                .map(|_| ())
                .map_err(|error| error.to_string())
        }) && cx.state.selected_design_variable.as_deref() == Some(name.as_str())
        {
            cx.state.selected_design_variable = None;
        }
    }
}

/// A resolved value as the Resolved column reads it: the reader's significant
/// digits, an SI prefix, and the unit the variable's label names, if any.
fn resolved_text(value: f64, quantity: DesignVariableQuantity, digits: usize) -> String {
    use rspice_results::number_format::{fmt_column_reading, fmt_register_reading};
    let unit = match quantity {
        DesignVariableQuantity::Resistance => "\u{3a9}",
        DesignVariableQuantity::Capacitance => "F",
        DesignVariableQuantity::Voltage => "V",
        DesignVariableQuantity::Current => "A",
        DesignVariableQuantity::Temperature => "\u{b0}C",
        DesignVariableQuantity::Dimensionless => "",
    };
    if unit.is_empty() {
        fmt_register_reading(value, unit, digits)
    } else {
        fmt_column_reading(value, unit, digits)
    }
}

fn selected_record(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, payload: &SimulationPlanPayload) {
    let selected = cx.state.selected_design_variable.as_ref().and_then(|name| {
        payload
            .design_variables
            .iter()
            .find(|variable| &variable.name == name)
    });
    let Some(variable) = selected else {
        card(
            ui,
            "Selected variable",
            Some(("none selected", Tone::Neutral)),
            |ui| {
                card_note(
                    ui,
                    "Select a row above to see its complete record: value, quantity, bounds \
                     and whether the optimizer may tune it.",
                );
            },
        );
        return;
    };
    let variable = variable.clone();
    let title = format!("Selected variable · {}", variable.name);
    let variable_id = variable.id;
    let mut draft = cx
        .state
        .design_variable_expression_draft
        .clone()
        .unwrap_or_else(|| variable.expression.clone());
    let stored_bounds = variable.allowed_range.as_ref().map_or_else(
        || (String::new(), String::new()),
        |range| (range.minimum.clone(), range.maximum.clone()),
    );
    let mut bounds = cx
        .state
        .design_variable_bounds_draft
        .clone()
        .unwrap_or_else(|| stored_bounds.clone());
    let quantities: Vec<String> = DesignVariableQuantity::ALL
        .iter()
        .map(|quantity| quantity.label().to_owned())
        .collect();

    // One slot for the whole card: `field_pair` holds two controls at once,
    // and two closures cannot borrow the same variable mutably.
    let edit: std::cell::Cell<Option<VariableEdit>> = std::cell::Cell::new(None);
    let released = std::cell::Cell::new(false);
    card(ui, &title, None, |ui| {
        card_body(ui, |ui| {
            // The expression is the one field an engineer changes repeatedly
            // while exploring, so it is editable in place rather than only
            // through the create dialog. It commits on release, as one
            // validated transaction that advances the variable's revision.
            field_pair(
                ui,
                ("Expression", &mut |ui: &mut Ui, width: f32| {
                    released.set(
                        released.get()
                            | mono_input(ui, "Expression", &mut draft, width).lost_focus(),
                    );
                }),
                Some(("Quantity", &mut |ui: &mut Ui, width: f32| {
                    if let Some(index) = select(
                        ui,
                        "simulation.variables.quantity",
                        "Quantity",
                        variable.quantity.label(),
                        &quantities,
                        width,
                    ) {
                        edit.set(Some(VariableEdit::Quantity(
                            DesignVariableQuantity::ALL[index],
                        )));
                    }
                })),
            );
            let mut tunable = variable.tunable;
            if super::page_kit::switch_row(ui, "Tunable", &mut tunable) {
                edit.set(Some(VariableEdit::Tunable(tunable)));
            }
            // Bounds are expressions, not numbers, so suffixes and owner
            // variables survive the round trip. Clearing both makes the
            // variable unbounded, which is a real choice rather than a
            // missing value.
            field_pair(
                ui,
                ("Minimum", &mut |ui: &mut Ui, width: f32| {
                    released.set(
                        released.get()
                            | mono_input(ui, "Minimum", &mut bounds.0, width).lost_focus(),
                    );
                }),
                Some(("Maximum", &mut |ui: &mut Ui, width: f32| {
                    released.set(
                        released.get()
                            | mono_input(ui, "Maximum", &mut bounds.1, width).lost_focus(),
                    );
                })),
            );
            // Units and out-of-bounds handling are stated, not offered. The
            // quantity is a label, so a unit is read when typed and never
            // required, and silently moving a value its author bounded is not
            // something a signed-off result could be built on.
            rule_row(
                ui,
                "Unit",
                "optional · 1.8, 1.8V and 1.8 V are the same number; the quantity only labels it",
            );
            rule_row(
                ui,
                "Out of bounds",
                "the edit is refused · a resolved value outside the range is never clamped into it",
            );
            if variable.allowed_range.is_none() {
                card_note(
                    ui,
                    "This variable is unbounded, so nothing can reject an out-of-range \
                     value. Give it both a minimum and a maximum to have the bound enforced.",
                );
            }
            if !variable.description.trim().is_empty() {
                rule_row(ui, "Description", &variable.description);
            }
        });
    });

    if let Some(edit) = edit.take() {
        let detail = edit.detail(&variable.name);
        replace_variable(cx, variable_id, &detail, |target| edit.apply(target));
        return;
    }
    if released.get() {
        // A refused commit keeps its draft. The workspace re-validates the
        // whole configuration, so an expression that breaks a bound or a
        // dimensional rule leaves the plan exactly as it was — and clearing the
        // draft regardless snapped the field back to the stored value, taking
        // the engineer's typed text with it and leaving a toast as the only
        // trace of what they had written. The field keeps the refused text so
        // the correction is one character rather than one retype.
        let expression_settled = draft.trim() == variable.expression.trim()
            || commit_expression(cx, variable_id, &draft, &variable.name);
        let bounds_settled =
            bounds == stored_bounds || commit_bounds(cx, variable_id, &bounds, &variable.name);
        if expression_settled {
            cx.state.design_variable_expression_draft = None;
        } else {
            cx.state.design_variable_expression_draft = Some(draft);
        }
        if bounds_settled {
            cx.state.design_variable_bounds_draft = None;
        } else {
            cx.state.design_variable_bounds_draft = Some(bounds);
        }
        return;
    }
    if draft != variable.expression {
        cx.state.design_variable_expression_draft = Some(draft);
    }
    if bounds != stored_bounds {
        cx.state.design_variable_bounds_draft = Some(bounds);
    }
}

/// One accepted change to a design variable's typed contract.
enum VariableEdit {
    Quantity(DesignVariableQuantity),
    Tunable(bool),
}

impl VariableEdit {
    fn apply(self, target: &mut crate::state::DesignVariable) {
        match self {
            Self::Quantity(quantity) => target.quantity = quantity,
            Self::Tunable(tunable) => target.tunable = tunable,
        }
    }

    fn detail(&self, name: &str) -> String {
        match self {
            Self::Quantity(quantity) => format!(
                "Typed design variable {name} as {}.",
                quantity.label().to_lowercase()
            ),
            Self::Tunable(true) => format!("Made design variable {name} tunable."),
            Self::Tunable(false) => format!("Made design variable {name} not tunable."),
        }
    }
}

/// Replace one variable's typed contract as a single validated transaction.
///
/// Reports whether the transaction was adopted, so an in-place editor can tell
/// a refusal from a change and keep the refused text on the field.
fn replace_variable(
    cx: &mut Cx<'_, SimulateFeature>,
    variable_id: DesignVariableId,
    detail: &str,
    apply: impl FnOnce(&mut crate::state::DesignVariable),
) -> bool {
    let Ok(plan_id) = cx
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return false;
    };
    let Some(mut replacement) = cx
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .and_then(|payload| {
            payload
                .design_variables
                .iter()
                .find(|variable| variable.id == variable_id)
                .cloned()
        })
    else {
        return false;
    };
    apply(&mut replacement);
    commit_replacement(cx, plan_id, variable_id, replacement, detail)
}

/// Replace one variable's inclusive bounds. Clearing both fields removes the
/// range outright, which is what makes the variable unbounded again.
///
/// Reports whether the bounds were adopted.
fn commit_bounds(
    cx: &mut Cx<'_, SimulateFeature>,
    variable_id: DesignVariableId,
    bounds: &(String, String),
    name: &str,
) -> bool {
    let range = (!bounds.0.trim().is_empty() || !bounds.1.trim().is_empty()).then(|| {
        crate::state::DesignVariableRange {
            minimum: bounds.0.trim().to_owned(),
            maximum: bounds.1.trim().to_owned(),
        }
    });
    let detail = range.as_ref().map_or_else(
        || format!("Removed the bounds on design variable {name}."),
        |range| {
            format!(
                "Bounded design variable {name} to {} … {}.",
                range.minimum, range.maximum
            )
        },
    );
    replace_variable(cx, variable_id, &detail, |target| {
        target.allowed_range = range;
    })
}

/// Reports whether the replacement was adopted.
fn commit_replacement(
    cx: &mut Cx<'_, SimulateFeature>,
    plan_id: SimulationPlanId,
    variable_id: DesignVariableId,
    replacement: crate::state::DesignVariable,
    detail: &str,
) -> bool {
    commit_plan_change(cx, plan_id, detail, move |workspace, plan_id| {
        workspace
            .content
            .replace_design_variable(plan_id, variable_id, replacement)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
}

/// Replace one variable's expression as a single validated transaction.
///
/// The workspace mutator advances the variable's revision and re-validates the
/// whole configuration, so an expression that breaks a bound or a dimensional
/// rule leaves the plan exactly as it was and reports why. Reports whether the
/// expression was adopted, which is what lets the field keep a refused edit.
fn commit_expression(
    cx: &mut Cx<'_, SimulateFeature>,
    variable_id: DesignVariableId,
    expression: &str,
    name: &str,
) -> bool {
    let Ok(plan_id) = cx
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return false;
    };
    let detail = format!("Updated design variable {name} to {expression}.");
    let expression = expression.to_owned();
    commit_plan_change(cx, plan_id, &detail, move |workspace, plan_id| {
        workspace
            .content
            .update_design_variable_expression(plan_id, variable_id, expression)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
}
