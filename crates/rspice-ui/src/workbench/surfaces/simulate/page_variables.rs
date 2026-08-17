//! Design variables.
//!
//! The typed parameters this plan resolves. The registry is the whole page's
//! subject: a selected row opens its record beside the table, and every
//! mutation goes through a plan-configuration transaction so the change
//! produces a receipt and invalidates preflight.

use egui::Ui;

use crate::product::{DesignVariableId, SimulationPlanId};
use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

use crate::ui::widgets::{mono_input, select};

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, field_pair, ledger_head, ledger_row,
    rule_row,
};
use super::workflows::{commit_plan_change, unique_copy_name};

const REGISTRY_COLUMNS: [f32; 6] = [0.18, 0.14, 0.20, 0.14, 0.16, 0.18];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some((plan_id, payload)) = plan_payload(app) else {
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
    registry(ui, app, plan_id, &payload);
    card_row(
        ui,
        app,
        |ui, app| selected_record(ui, app, &payload),
        |ui, app| change_impact(ui, app, &payload),
    );
}

fn plan_payload(app: &RSpiceApp) -> Option<(SimulationPlanId, SimulationPlanPayload)> {
    let plan_id = app.state.sim_setup.stable_analysis_plan().ok()?.id();
    let payload = app.state.workspace.plan_data(plan_id)?.clone();
    Some((plan_id, payload))
}

fn registry(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
    payload: &SimulationPlanPayload,
) {
    let variables = &payload.design_variables;
    let selected = app.state.workbench.selected_design_variable.clone();
    let selected_index = selected
        .as_ref()
        .and_then(|name| variables.iter().position(|variable| &variable.name == name));
    // "Optimizable" rather than "sweepable": the sweep role is enforced at
    // dispatch by the optimizer, which refuses a variable declared fixed.
    // There is no design-variable axis in the run set yet, so counting these
    // as sweepable would name a capability the plan cannot execute.
    let status = format!(
        "{} typed · {} optimizable",
        variables.len(),
        variables
            .iter()
            .filter(|variable| !matches!(
                variable.sweep_eligibility,
                crate::state::DesignVariableSweepEligibility::FixedParameter
            ))
            .count()
    );
    // A spec sheet the browser picker has finished reading is adopted before
    // the head is drawn, so the row it adds is in this frame's registry.
    #[cfg(target_arch = "wasm32")]
    super::variable_import::poll_pending_import(ui.ctx(), app, plan_id);

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
                Some((status.as_str(), Tone::Ok)),
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
                    "Bounds",
                    "Scope",
                    "Sweep role",
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
                );
            }
            for (index, variable) in variables.iter().enumerate() {
                let bounds = variable.allowed_range.as_ref().map_or_else(
                    || "unbounded".to_owned(),
                    |range| format!("{} … {}", range.minimum, range.maximum),
                );
                if ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        (variable.name.as_str(), Tone::Accent),
                        (variable.quantity.label(), Tone::Neutral),
                        (variable.expression.as_str(), Tone::Neutral),
                        (bounds.as_str(), Tone::Neutral),
                        (variable.scope.label(), Tone::Neutral),
                        (variable.sweep_eligibility.label(), Tone::Neutral),
                    ],
                    selected_index == Some(index),
                )
                .clicked()
                {
                    pick = Some(variable.name.clone());
                }
            }
            card_note(
                ui,
                "A variable is plan-scoped: every analysis in this plan resolves the same value \
                 unless it declares an explicit override. A bound is hard and is never clamped: \
                 the same check refuses the record at construction, refuses to emit the \
                 deck's parameter lines, refuses to validate the stored simulation configuration, \
                 and refuses to export the plan's parameters to the PDK callback. An out-of-range \
                 value therefore cannot reach a run by any route.",
            );
        },
    );

    if let Some(name) = pick {
        // A draft belongs to the row it was typed into. Selecting another row
        // must not carry a half-typed expression or bound onto it.
        if app.state.workbench.selected_design_variable.as_deref() != Some(name.as_str()) {
            app.state.workbench.design_variable_expression_draft = None;
            app.state.workbench.design_variable_bounds_draft = None;
        }
        app.state.workbench.selected_design_variable = Some(name);
    }
    if import {
        super::variable_import::import_from_file(ui.ctx(), app, plan_id);
    }
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
            if commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .duplicate_design_variable(plan_id, id, copy)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            }) {
                // The copy carries the original's whole typed contract, so the
                // record editor should open on it — that is the field the copy
                // exists to change.
                app.state.workbench.design_variable_expression_draft = None;
                app.state.workbench.design_variable_bounds_draft = None;
                app.state.workbench.selected_design_variable = Some(committed);
            }
        } else if remove {
            let detail = format!("Removed design variable {name}.");
            // A refused removal leaves the variable in the registry, so the
            // selection has to survive with it — clearing regardless would
            // close the editor on a row that is still there.
            if commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .remove_design_variable(plan_id, id)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            }) {
                app.state.workbench.selected_design_variable = None;
            }
        }
    }
}

fn selected_record(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let selected = app
        .state
        .workbench
        .selected_design_variable
        .as_ref()
        .and_then(|name| {
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
                    "Select a row above to see its complete record: quantity, bounds, scope, and \
                     the policy that decides whether an analysis may override it.",
                );
            },
        );
        return;
    };
    let variable = variable.clone();
    let title = format!("Selected variable · {}", variable.name);
    let variable_id = variable.id;
    let revision = variable.revision.get();
    let mut draft = app
        .state
        .workbench
        .design_variable_expression_draft
        .clone()
        .unwrap_or_else(|| variable.expression.clone());
    let stored_bounds = variable.allowed_range.as_ref().map_or_else(
        || (String::new(), String::new()),
        |range| (range.minimum.clone(), range.maximum.clone()),
    );
    let mut bounds = app
        .state
        .workbench
        .design_variable_bounds_draft
        .clone()
        .unwrap_or_else(|| stored_bounds.clone());
    let quantities: Vec<String> = DesignVariableQuantity::ALL
        .iter()
        .map(|quantity| quantity.label().to_owned())
        .collect();
    let sweep_roles: Vec<String> = DesignVariableSweepEligibility::ALL
        .iter()
        .map(|role| role.label().to_owned())
        .collect();
    let override_policies: Vec<String> = DesignVariableOverridePolicy::ALL
        .iter()
        .map(|policy| policy.label().to_owned())
        .collect();
    let scopes = scope_options(app, &variable.scope);
    let scope_labels: Vec<String> = scopes.iter().map(|(label, _)| label.clone()).collect();
    let current_scope = scope_label(&variable.scope, &scopes);

    // One slot for the whole card: `field_pair` holds two controls at once,
    // and two closures cannot borrow the same variable mutably.
    let edit: std::cell::Cell<Option<VariableEdit>> = std::cell::Cell::new(None);
    let released = std::cell::Cell::new(false);
    let status = format!("revision {revision}");
    card(ui, &title, Some((status.as_str(), Tone::Ok)), |ui| {
        card_body(ui, |ui| {
            // The expression is the one field an engineer changes repeatedly
            // while exploring, so it is editable in place rather than only
            // through the create dialog. It commits on release, as one
            // validated transaction that advances the variable's revision.
            field_pair(
                ui,
                ("Expression", &mut |ui: &mut Ui, width: f32| {
                    released.set(released.get() | mono_input(ui, &mut draft, width).lost_focus());
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
            field_pair(
                ui,
                ("Scope", &mut |ui: &mut Ui, width: f32| {
                    if let Some(index) = select(
                        ui,
                        "simulation.variables.scope",
                        "Scope",
                        &current_scope,
                        &scope_labels,
                        width,
                    ) {
                        edit.set(Some(VariableEdit::Scope(scopes[index].1.clone())));
                    }
                }),
                Some(("Sweep role", &mut |ui: &mut Ui, width: f32| {
                    if let Some(index) = select(
                        ui,
                        "simulation.variables.sweep",
                        "Sweep role",
                        variable.sweep_eligibility.label(),
                        &sweep_roles,
                        width,
                    ) {
                        edit.set(Some(VariableEdit::SweepRole(
                            DesignVariableSweepEligibility::ALL[index],
                        )));
                    }
                })),
            );
            field_pair(
                ui,
                ("Override policy", &mut |ui: &mut Ui, width: f32| {
                    if let Some(index) = select(
                        ui,
                        "simulation.variables.override",
                        "Override policy",
                        variable.override_policy.label(),
                        &override_policies,
                        width,
                    ) {
                        edit.set(Some(VariableEdit::OverridePolicy(
                            DesignVariableOverridePolicy::ALL[index],
                        )));
                    }
                }),
                None,
            );
            // Bounds are expressions, not numbers, so suffixes and owner
            // variables survive the round trip. Clearing both makes the
            // variable unbounded, which is a real choice rather than a
            // missing value.
            field_pair(
                ui,
                ("Minimum", &mut |ui: &mut Ui, width: f32| {
                    released
                        .set(released.get() | mono_input(ui, &mut bounds.0, width).lost_focus());
                }),
                Some(("Maximum", &mut |ui: &mut Ui, width: f32| {
                    released
                        .set(released.get() | mono_input(ui, &mut bounds.1, width).lost_focus());
                })),
            );
            // Unit handling and out-of-bounds handling are stated, not
            // offered. Each has exactly one implementation and no second one
            // worth building: there is no unit algebra that could compute a
            // conversion factor — the check is a required suffix on a typed
            // quantity — and silently moving a value its author bounded is not
            // a policy a signed-off result could be built on.
            rule_row(
                ui,
                "Unit policy",
                "strict · a value whose unit does not match its quantity is rejected, never \
                 converted",
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
        let detail = edit.detail(&variable.name, &scopes);
        replace_variable(app, variable_id, &detail, |target| edit.apply(target));
        return;
    }
    if released.get() {
        if draft.trim() != variable.expression.trim() {
            commit_expression(app, variable_id, &draft, &variable.name);
        }
        if bounds != stored_bounds {
            commit_bounds(app, variable_id, &bounds, &variable.name);
        }
        app.state.workbench.design_variable_expression_draft = None;
        app.state.workbench.design_variable_bounds_draft = None;
        return;
    }
    if draft != variable.expression {
        app.state.workbench.design_variable_expression_draft = Some(draft);
    }
    if bounds != stored_bounds {
        app.state.workbench.design_variable_bounds_draft = Some(bounds);
    }
}

/// One accepted change to a design variable's typed contract.
enum VariableEdit {
    Quantity(DesignVariableQuantity),
    Scope(DesignVariableScope),
    SweepRole(DesignVariableSweepEligibility),
    OverridePolicy(DesignVariableOverridePolicy),
}

impl VariableEdit {
    fn apply(self, target: &mut crate::state::DesignVariable) {
        match self {
            Self::Quantity(quantity) => target.quantity = quantity,
            Self::Scope(scope) => target.scope = scope,
            Self::SweepRole(role) => target.sweep_eligibility = role,
            Self::OverridePolicy(policy) => target.override_policy = policy,
        }
    }

    fn detail(&self, name: &str, scopes: &[(String, DesignVariableScope)]) -> String {
        match self {
            Self::Quantity(quantity) => format!(
                "Typed design variable {name} as {}.",
                quantity.label().to_lowercase()
            ),
            Self::Scope(scope) => format!(
                "Scoped design variable {name} to {}.",
                scope_label(scope, scopes)
            ),
            Self::SweepRole(role) => format!(
                "Set design variable {name} to {}.",
                role.label().to_lowercase()
            ),
            Self::OverridePolicy(policy) => format!(
                "Set design variable {name} to {}.",
                policy.label().to_lowercase()
            ),
        }
    }
}

/// The scopes a variable may take: the two project-wide ones, one entry per
/// analysis the plan owns, and — when it already holds one — the cell scope it
/// was authored with, so selecting nothing else preserves it.
fn scope_options(
    app: &RSpiceApp,
    current: &DesignVariableScope,
) -> Vec<(String, DesignVariableScope)> {
    let mut options = vec![
        (
            DesignVariableScope::Project.label().to_owned(),
            DesignVariableScope::Project,
        ),
        (
            DesignVariableScope::Testbench.label().to_owned(),
            DesignVariableScope::Testbench,
        ),
    ];
    if let DesignVariableScope::SelectedCell { cell } = current {
        options.push((
            format!("Only cell {}", cell.cell),
            DesignVariableScope::SelectedCell { cell: cell.clone() },
        ));
    }
    if let Ok(plan) = app.state.sim_setup.stable_analysis_plan() {
        options.extend(
            plan.instances()
                .iter()
                .enumerate()
                .map(|(index, instance)| {
                    (
                        format!("Only #{} · {}", index + 1, instance.kind().code()),
                        DesignVariableScope::SelectedAnalysis {
                            analysis_id: instance.id(),
                        },
                    )
                }),
        );
    }
    options
}

/// The label for a stored scope, resolved against the offered options. A scope
/// bound to an analysis the plan no longer owns says so rather than silently
/// reading as one of the offered choices.
fn scope_label(scope: &DesignVariableScope, options: &[(String, DesignVariableScope)]) -> String {
    options
        .iter()
        .find(|(_, candidate)| candidate == scope)
        .map_or_else(
            || "Selected analysis · no longer in the plan".to_owned(),
            |(label, _)| label.clone(),
        )
}

/// Replace one variable's typed contract as a single validated transaction.
fn replace_variable(
    app: &mut RSpiceApp,
    variable_id: DesignVariableId,
    detail: &str,
    apply: impl FnOnce(&mut crate::state::DesignVariable),
) {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return;
    };
    let Some(mut replacement) = app
        .state
        .workspace
        .plan_data(plan_id)
        .and_then(|payload| {
            payload
                .design_variables
                .iter()
                .find(|variable| variable.id == variable_id)
                .cloned()
        })
    else {
        return;
    };
    apply(&mut replacement);
    commit_replacement(app, plan_id, variable_id, replacement, detail);
}

/// Replace one variable's inclusive bounds. Clearing both fields removes the
/// range outright, which is what makes the variable unbounded again.
fn commit_bounds(
    app: &mut RSpiceApp,
    variable_id: DesignVariableId,
    bounds: &(String, String),
    name: &str,
) {
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
    replace_variable(app, variable_id, &detail, |target| {
        target.allowed_range = range;
    });
}

fn commit_replacement(
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
    variable_id: DesignVariableId,
    replacement: crate::state::DesignVariable,
    detail: &str,
) {
    commit_plan_change(app, plan_id, detail, move |workspace, plan_id| {
        workspace
            .replace_design_variable(plan_id, variable_id, replacement)
            .map(|_| ())
            .map_err(|error| error.to_string())
    });
}

/// Replace one variable's expression as a single validated transaction.
///
/// The workspace mutator advances the variable's revision and re-validates the
/// whole configuration, so an expression that breaks a bound or a dimensional
/// rule leaves the plan exactly as it was and reports why.
fn commit_expression(
    app: &mut RSpiceApp,
    variable_id: DesignVariableId,
    expression: &str,
    name: &str,
) {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return;
    };
    let detail = format!("Updated design variable {name} to {expression}.");
    let expression = expression.to_owned();
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        workspace
            .update_design_variable_expression(plan_id, variable_id, expression)
            .map(|_| ())
            .map_err(|error| error.to_string())
    });
}

fn change_impact(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    let variables = &payload.design_variables;
    let analyses = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.instances().len());
    let referenced: Vec<&crate::state::DesignVariable> = variables
        .iter()
        .filter(|variable| {
            payload
                .saved_outputs
                .iter()
                .any(|output| output.source_expression.contains(&variable.name))
        })
        .collect();
    card(
        ui,
        "Change impact",
        Some(("rerun required", Tone::Warn)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "Analyses in this plan",
                    &format!("{analyses} · every one resolves these values"),
                );
                rule_row(
                    ui,
                    "Referenced by a saved output",
                    &if referenced.is_empty() {
                        "none".to_owned()
                    } else {
                        referenced
                            .iter()
                            .map(|variable| variable.name.as_str())
                            .collect::<Vec<_>>()
                            .join(" · ")
                    },
                );
                rule_row(
                    ui,
                    "Retained results",
                    "unchanged · a committed dataset is immutable",
                );
            });
            card_note(
                ui,
                "Editing a variable changes what the next run resolves; it never rewrites a \
                 dataset already committed. Comparing across a variable change is a regression \
                 comparison, not a refresh.",
            );
        },
    );
}
