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
use crate::workbench::app::{PlanRemovalConsequence, PlanRemovalTarget, PlanRemovalTone};

use crate::ui::widgets::{mono_input, select};

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, field_pair, ledger_head, ledger_row,
    rule_row,
};
use super::workflows::{commit_plan_change, unique_copy_name};

const REGISTRY_COLUMNS: [f32; 7] = [0.16, 0.12, 0.17, 0.12, 0.14, 0.14, 0.15];

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
    super::pages::plan_configuration_receipts(ui, app);
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
    // A spec sheet the picker has finished reading is adopted before the head
    // is drawn, so the rows it adds are in this frame's registry.
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
                    "Used by",
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
                // Which analyses resolve this variable, from the derivation the
                // Create-variable dialog previews it with. The registry showed
                // scope but not reach: "Project" says where the variable is
                // declared and nothing about which of this plan's analyses
                // actually read it.
                let used_by = super::workflows::design_variable_consumers_scoped(
                    app,
                    match &variable.scope {
                        crate::state::DesignVariableScope::SelectedAnalysis { analysis_id } => {
                            Some(Some(*analysis_id))
                        }
                        _ => None,
                    },
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
                        (used_by.as_str(), Tone::Neutral),
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
        super::variable_import::import_from_file(ui.ctx(), &mut app.state);
    }
    // A removal the reader confirmed in the destructive review is applied here,
    // by the page that staged it: this is the registry the row is in, so the
    // one route that can commit it is the one route that can show the reader it
    // happened. Taken before the button is read so a confirmed answer is never
    // left standing behind a click in the same frame.
    //
    // The table above was already drawn from the payload this is about to
    // change, so the frame is asked to run again: the confirming click landed
    // in the modal, and nothing else is coming to repaint the row it removed.
    let mut removal = app
        .state
        .dialogs
        .plan_removal_review
        .take_confirmed_variable(plan_id)
        .inspect(|_| ui.ctx().request_repaint())
        .and_then(|id| {
            variables
                .iter()
                .find(|variable| variable.id == id)
                .map(|variable| (id, variable.name.clone()))
        });
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
            // Removing a variable the plan resolves into an analysis is a
            // removal that costs something: the expressions that named it are
            // left naming nothing. Removing one nothing reads costs nothing, so
            // it is not worth a modal.
            match removal_consequences(app, &variables[index]) {
                Some(consequences) => app.state.dialogs.plan_removal_review.open(
                    PlanRemovalTarget::Variable { plan: plan_id, id },
                    name,
                    consequences,
                ),
                None => removal = Some((id, name)),
            }
        }
    }
    if let Some((id, name)) = removal {
        let detail = format!("Removed design variable {name}.");
        // A refused removal leaves the variable in the registry, so the
        // selection has to survive with it — clearing regardless would
        // close the editor on a row that is still there.
        if commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
            workspace
                .remove_design_variable(plan_id, id)
                .map(|_| ())
                .map_err(|error| error.to_string())
        }) && app.state.workbench.selected_design_variable.as_deref() == Some(name.as_str())
        {
            app.state.workbench.selected_design_variable = None;
        }
    }
}

/// What removing this variable orphans, or `None` when nothing reads it.
///
/// The readers come from [`design_variable_consumers_scoped`], which is the
/// same derivation the registry's "Used by" column and the Create-variable
/// dialog's preview both state — so what the review says is about to lose this
/// variable is exactly what the row said was using it. The two sentences that
/// derivation answers with when there is no reader are its own vocabulary, and
/// matching them here is what keeps the gate and the column from disagreeing
/// about the same plan.
///
/// [`design_variable_consumers_scoped`]: super::workflows::design_variable_consumers_scoped
fn removal_consequences(
    app: &RSpiceApp,
    variable: &crate::state::DesignVariable,
) -> Option<Vec<PlanRemovalConsequence>> {
    let readers = super::workflows::design_variable_consumers_scoped(
        app,
        match &variable.scope {
            DesignVariableScope::SelectedAnalysis { analysis_id } => Some(Some(*analysis_id)),
            _ => None,
        },
    );
    if readers == "no enabled analysis consumers" || readers == "plan unavailable" {
        return None;
    }
    Some(vec![
        PlanRemovalConsequence::stated("Analyses resolving it", readers).explained(
            PlanRemovalTone::Warn,
            "The analyses listed above resolve this variable. Removing it leaves every expression \
             that names it unresolved, and they will refuse to build a deck until the name is \
             gone from them or the variable is declared again.",
        ),
        PlanRemovalConsequence::stated("Scope it is declared at", variable.scope.label()).explained(
            PlanRemovalTone::Aside,
            "Retained results from runs that already resolved this variable stay in the project \
             and stay readable. What removal takes is the declaration, not the evidence.",
        ),
    ])
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
        // A refused commit keeps its draft. The workspace re-validates the
        // whole configuration, so an expression that breaks a bound or a
        // dimensional rule leaves the plan exactly as it was — and clearing the
        // draft regardless snapped the field back to the stored value, taking
        // the engineer's typed text with it and leaving a toast as the only
        // trace of what they had written. The field keeps the refused text so
        // the correction is one character rather than one retype.
        let expression_settled = draft.trim() == variable.expression.trim()
            || commit_expression(app, variable_id, &draft, &variable.name);
        let bounds_settled =
            bounds == stored_bounds || commit_bounds(app, variable_id, &bounds, &variable.name);
        if expression_settled {
            app.state.workbench.design_variable_expression_draft = None;
        } else {
            app.state.workbench.design_variable_expression_draft = Some(draft);
        }
        if bounds_settled {
            app.state.workbench.design_variable_bounds_draft = None;
        } else {
            app.state.workbench.design_variable_bounds_draft = Some(bounds);
        }
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
                        format!(
                            "Only {}",
                            plan.instance_list_label(index)
                                .unwrap_or_else(|| instance.display_name().to_owned())
                        ),
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
///
/// Reports whether the transaction was adopted, so an in-place editor can tell
/// a refusal from a change and keep the refused text on the field.
fn replace_variable(
    app: &mut RSpiceApp,
    variable_id: DesignVariableId,
    detail: &str,
    apply: impl FnOnce(&mut crate::state::DesignVariable),
) -> bool {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return false;
    };
    let Some(mut replacement) = app.state.workspace.plan_data(plan_id).and_then(|payload| {
        payload
            .design_variables
            .iter()
            .find(|variable| variable.id == variable_id)
            .cloned()
    }) else {
        return false;
    };
    apply(&mut replacement);
    commit_replacement(app, plan_id, variable_id, replacement, detail)
}

/// Replace one variable's inclusive bounds. Clearing both fields removes the
/// range outright, which is what makes the variable unbounded again.
///
/// Reports whether the bounds were adopted.
fn commit_bounds(
    app: &mut RSpiceApp,
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
    replace_variable(app, variable_id, &detail, |target| {
        target.allowed_range = range;
    })
}

/// Reports whether the replacement was adopted.
fn commit_replacement(
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
    variable_id: DesignVariableId,
    replacement: crate::state::DesignVariable,
    detail: &str,
) -> bool {
    commit_plan_change(app, plan_id, detail, move |workspace, plan_id| {
        workspace
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
    app: &mut RSpiceApp,
    variable_id: DesignVariableId,
    expression: &str,
    name: &str,
) -> bool {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return false;
    };
    let detail = format!("Updated design variable {name} to {expression}.");
    let expression = expression.to_owned();
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        workspace
            .update_design_variable_expression(plan_id, variable_id, expression)
            .map(|_| ())
            .map_err(|error| error.to_string())
    })
}

/// What the Change impact card says about who resolves these values.
///
/// Enabled instances are the readers. A disabled one is still declared, so it
/// is counted separately rather than dropped: the reader who wonders where the
/// other six went has to be told, and told that a disabled analysis is not
/// waiting to be re-run.
///
/// The verb agrees with the count it follows: a plan with one enabled analysis
/// of ten read "1 of 10 enabled · those resolve these values", which names a
/// group where there is one.
fn enabled_reader_summary(enabled: usize, disabled: usize) -> String {
    if disabled == 0 {
        format!("{enabled} \u{b7} every one resolves these values")
    } else {
        format!(
            "{enabled} of {} enabled \u{b7} {} these values",
            enabled + disabled,
            if enabled == 1 {
                "that one resolves"
            } else {
                "those resolve"
            }
        )
    }
}

fn change_impact(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    let variables = &payload.design_variables;
    // Enabled instances only. A disabled analysis resolves nothing: it is not
    // in the run this plan would dispatch, so counting it here said "10
    // analyses resolve these values" over a plan that would resolve them for
    // four -- the same over-count the placed-source list stopped making when
    // it started asking whether a consumer reads.
    let (analyses, disabled) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or((0, 0), |plan| {
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .count();
            (enabled, plan.instances().len() - enabled)
        });
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
                    &enabled_reader_summary(analyses, disabled),
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

#[cfg(test)]
mod tests {
    use super::enabled_reader_summary;

    /// The Change impact card counts the analyses that would run.
    ///
    /// It counted every declared instance, so a plan holding ten analyses with
    /// six switched off told the reader that ten of them resolve these values.
    /// A disabled analysis resolves nothing and is not waiting to be re-run,
    /// which is the whole point of a card headed "rerun required".
    #[test]
    fn the_change_impact_card_counts_only_the_analyses_that_would_run() {
        assert_eq!(
            enabled_reader_summary(4, 0),
            "4 \u{b7} every one resolves these values"
        );
        assert_eq!(
            enabled_reader_summary(4, 6),
            "4 of 10 enabled \u{b7} those resolve these values"
        );
        assert_eq!(
            enabled_reader_summary(0, 3),
            "0 of 3 enabled \u{b7} those resolve these values"
        );
    }

    /// The verb agrees with the count it follows.
    ///
    /// One enabled analysis of ten read "1 of 10 enabled · those resolve these
    /// values", which names a group where there is one.
    #[test]
    fn one_enabled_analysis_is_not_a_group() {
        assert_eq!(
            enabled_reader_summary(1, 9),
            "1 of 10 enabled \u{b7} that one resolves these values"
        );
        assert_eq!(
            enabled_reader_summary(2, 8),
            "2 of 10 enabled \u{b7} those resolve these values"
        );
    }
}
