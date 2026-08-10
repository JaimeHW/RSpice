//! Design variables.
//!
//! The typed parameters this plan resolves. The registry is the whole page's
//! subject: a selected row opens its record beside the table, and every
//! mutation goes through a plan-configuration transaction so the change
//! produces a receipt and invalidates preflight.

use egui::Ui;

use crate::product::SimulationPlanId;
use crate::state::workspace::SimulationPlanPayload;
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, ledger_head, ledger_row, rule_row,
};

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
    let payload = app.state.workspace.active_plan_data(plan_id)?.clone();
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
    let status = format!(
        "{} typed · {} sweepable",
        variables.len(),
        variables
            .iter()
            .filter(|variable| !matches!(
                variable.sweep_eligibility,
                crate::state::DesignVariableSweepEligibility::FixedParameter
            ))
            .count()
    );
    let mut duplicate = false;
    let mut remove = false;
    let mut pick = None;
    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Variable registry",
                Some((status.as_str(), Tone::Ok)),
                |ui| {
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
                 unless it declares an explicit override. Bounds are enforced at preflight, not \
                 clamped at run time.",
            );
        },
    );

    if let Some(name) = pick {
        app.state.workbench.selected_design_variable = Some(name);
    }
    if let Some(index) = selected_index {
        let name = variables[index].name.clone();
        let id = variables[index].id;
        if duplicate {
            // The copy needs a name that is free in this plan; deriving it here
            // keeps the transaction a single step rather than opening a dialog
            // over a registry that is about to change.
            let copy = unique_name(variables, &name);
            let detail = format!("Duplicated design variable {name} as {copy}.");
            mutate(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .duplicate_design_variable(plan_id, id, copy)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            });
        } else if remove {
            let detail = format!("Removed design variable {name}.");
            mutate(app, plan_id, &detail, move |workspace, plan_id| {
                workspace
                    .remove_design_variable(plan_id, id)
                    .map(|_| ())
                    .map_err(|error| error.to_string())
            });
            app.state.workbench.selected_design_variable = None;
        }
    }
}

/// `name_copy`, then `name_copy_2`, and so on until the plan has no such
/// variable. Names are compared case-insensitively because that is how the
/// registry itself validates uniqueness.
fn unique_name(variables: &[crate::state::DesignVariable], base: &str) -> String {
    let taken = |candidate: &str| {
        variables
            .iter()
            .any(|variable| variable.name.eq_ignore_ascii_case(candidate))
    };
    let first = format!("{base}_copy");
    if !taken(&first) {
        return first;
    }
    (2u32..)
        .map(|index| format!("{base}_copy_{index}"))
        .find(|candidate| !taken(candidate))
        .unwrap_or(first)
}

fn selected_record(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
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
    let title = format!("Selected variable · {}", variable.name);
    card(
        ui,
        &title,
        Some(("plan revision retained", Tone::Ok)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Expression", &variable.expression);
                rule_row(ui, "Quantity", variable.quantity.label());
                rule_row(ui, "Scope", variable.scope.label());
                rule_row(ui, "Sweep role", variable.sweep_eligibility.label());
                rule_row(ui, "Override policy", variable.override_policy.label());
                rule_row(
                    ui,
                    "Bounds",
                    &variable.allowed_range.as_ref().map_or_else(
                        || "unbounded · preflight cannot reject an out-of-range value".to_owned(),
                        |range| format!("{} … {}", range.minimum, range.maximum),
                    ),
                );
                if !variable.description.trim().is_empty() {
                    rule_row(ui, "Description", &variable.description);
                }
            });
        },
    );
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

/// Apply a plan-data mutation as one configuration transaction.
///
/// The candidate workspace and setup are mutated on clones and only adopted
/// once both the mutation and the configuration validation succeed, so a
/// rejected edit leaves no partial state behind.
fn mutate(
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
    detail: &str,
    change: impl FnOnce(&mut crate::state::ProjectWorkspace, SimulationPlanId) -> Result<(), String>,
) {
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
            app.state.workbench.analysis_lifecycle_status = format!(
                "Configuration receipt #{} · revision {} to {} · {}",
                receipt.sequence(),
                receipt.source_revision().get(),
                receipt.committed_revision().get(),
                receipt.detail()
            );
        }
        Err(error) => {
            app.state.workbench.analysis_lifecycle_status = error;
        }
    }
}
