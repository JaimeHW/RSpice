//! The capture-group card: what each group holds, and what it costs.
//!
//! Every number on this card comes from one
//! [`CaptureLedger`](crate::simulation::capture_ledger::CaptureLedger), which is
//! the same fold preparation prices the run with. The card therefore has no
//! arithmetic of its own — it draws rows and reads
//! [`total_bytes`](crate::simulation::capture_ledger::CaptureLedger::total_bytes)
//! for the status line, so the forecast an engineer approves here and the
//! forecast that refuses their run cannot be two numbers.
//!
//! Group rows are drawn in resolution order with the fallback group last,
//! because that order *is* the precedence between two rules that both match.
//! Reordering is offered for the same reason: it is a policy edit, not a
//! presentation one.

use egui::Ui;

use crate::product::CaptureGroupId;
use crate::simulation::capture_ledger::CaptureLedger;
use crate::state::{
    CaptureGroup, CaptureGroupMembership, OutputSelectionMode, SavedOutput, UNGROUPED_NAME,
};
use crate::ui::icons::Icon;
use crate::ui::widgets::{Button, select};
use crate::workbench::app::{PlanRemovalConsequence, PlanRemovalTarget, PlanRemovalTone};
use crate::workbench::state::{CaptureGroupDraft, SimulationWorkflowDialog};
use crate::workbench::{AppState, RSpiceApp};

use super::super::page_kit::{
    Tone, card_body, card_head_row, card_note, card_with_head, field_pair, ledger_head, ledger_row,
    rule_row,
};
use super::{STORAGE_BUDGET_CHOICES, commit_save_policy, format_bytes};

/// `GROUP · POLICY · OUTPUTS · FORECAST · INDETERMINATE`.
const GROUP_COLUMNS: [f32; 5] = [0.26, 0.26, 0.12, 0.20, 0.16];

/// What the card says once nothing about the plan explains the number.
const GROUP_NOTE: &str = "A group's forecast is its own outputs summed across the tasks each is \
                          compatible with, priced over the Run Set. Every output is counted in \
                          exactly one group: an output a group names outranks every rule, and \
                          among rules the first group listed here takes it. Retained engine \
                          source is shared between deferred outputs and is held at plan level \
                          rather than charged to whichever group holds the first of them.";

/// The whole capture-group card, including the plan-level selection controls
/// that decide which outputs there are to group at all.
pub(super) fn capture_groups(
    ui: &mut Ui,
    state: &mut AppState,
    outputs: &[SavedOutput],
    ledger: &CaptureLedger,
    membership: &CaptureGroupMembership,
    automatic_fallback: bool,
    selection_error: Option<&str>,
) -> Option<GroupCommand> {
    let selection_mode = state.sim_setup.save_policy.output_selection_mode;
    let budget = state.sim_setup.save_policy.maximum_storage_bytes;
    let total = ledger.total_bytes();
    let indeterminate = ledger.indeterminate_count();
    let over_budget = total > budget;
    let status = if selection_error.is_some() {
        "Selection preflight unavailable".to_owned()
    } else if over_budget {
        format!(
            "{} forecast exceeds {} ceiling",
            format_bytes(total),
            format_bytes(budget)
        )
    } else if selection_mode == OutputSelectionMode::SaveAll {
        format!("{} Save All ceiling", format_bytes(total))
    } else if automatic_fallback {
        format!("{} automatic forecast", format_bytes(total))
    } else if indeterminate == 0 {
        format!("{} explicit forecast", format_bytes(total))
    } else {
        format!(
            "{} minimum · {indeterminate} indeterminate",
            format_bytes(total)
        )
    };
    let groups = plan_capture_groups(state);
    let selected = state.workbench.selected_capture_group;
    let mut command = None;
    // The head strip and the row list both answer with a command, and one
    // variable cannot be borrowed by both closures at once. Row selection wins
    // nothing here: the strip acts on whatever was already selected, so a click
    // in the head and a click in the list cannot both happen in one frame.
    let mut head_command = None;
    let mut picked_budget = None;
    let mut picked_mode = None;
    card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Capture groups",
                Some((
                    status.as_str(),
                    if selection_error.is_some() || over_budget {
                        Tone::Error
                    } else if indeterminate == 0 {
                        Tone::Ok
                    } else {
                        Tone::Warn
                    },
                )),
                |ui| head_command = group_commands(ui, &groups, selected),
            );
        },
        |ui| {
            let budget_choices = STORAGE_BUDGET_CHOICES
                .iter()
                .map(|bytes| format_bytes(*bytes))
                .collect::<Vec<_>>();
            let selected_budget = format_bytes(budget);
            let mode_choices = OutputSelectionMode::ALL
                .iter()
                .map(|mode| mode.label().to_owned())
                .collect::<Vec<_>>();
            card_body(ui, |ui| {
                field_pair(
                    ui,
                    ("Output selection", &mut |ui: &mut Ui, width: f32| {
                        picked_mode = select(
                            ui,
                            "simulation.save.output-selection",
                            "Plan output selection mode",
                            selection_mode.label(),
                            &mode_choices,
                            width,
                        );
                    }),
                    Some((
                        "Retained-evidence ceiling",
                        &mut |ui: &mut Ui, width: f32| {
                            picked_budget = select(
                                ui,
                                "simulation.save.storage-budget",
                                "Plan retained-evidence storage ceiling",
                                &selected_budget,
                                &budget_choices,
                                width,
                            );
                        },
                    )),
                );
                rule_row(ui, "Selection behavior", selection_mode.description());
            });
            ledger_head(
                ui,
                &GROUP_COLUMNS,
                &["Group", "Policy", "Outputs", "Forecast", "Indeterminate"],
            );
            for row in ledger.rows() {
                // The fallback group is listed even when it is empty: a reader
                // has to be able to see that nothing is escaping their groups,
                // and an absent row cannot say that.
                let outputs_cell = row.outputs.to_string();
                let bytes_cell = format_bytes(row.bytes);
                let indeterminate_cell = if row.indeterminate == 0 {
                    "—".to_owned()
                } else {
                    row.indeterminate.to_string()
                };
                let response = ledger_row(
                    ui,
                    &GROUP_COLUMNS,
                    &[
                        (row.name.as_str(), Tone::Accent),
                        (row.policy.as_str(), Tone::Neutral),
                        (
                            outputs_cell.as_str(),
                            if row.outputs == 0 {
                                Tone::Neutral
                            } else {
                                Tone::Accent
                            },
                        ),
                        (bytes_cell.as_str(), Tone::Neutral),
                        (
                            indeterminate_cell.as_str(),
                            if row.indeterminate == 0 {
                                Tone::Neutral
                            } else {
                                Tone::Warn
                            },
                        ),
                    ],
                    selected == Some(row.group),
                );
                let response =
                    response.on_hover_text(row_hover(&groups, row.group, outputs, membership));
                // The fallback group is not a record, so selecting it would arm
                // commands that have nothing to act on.
                if response.clicked() && row.group != CaptureGroup::ungrouped_id() {
                    command = Some(GroupCommand::Select(row.group));
                }
            }
            if ledger.shared_source_bytes() > 0 {
                plan_level_row(
                    ui,
                    "Retained engine source",
                    "shared by deferred outputs",
                    ledger.shared_source_bytes(),
                );
            }
            if let Some(engine_ceiling) = ledger.engine_ceiling_bytes() {
                plan_level_row(
                    ui,
                    "Engine result set",
                    "reserved for every enabled analysis",
                    engine_ceiling,
                );
            }
            if let Some(error) = selection_error {
                card_note(
                    ui,
                    &format!("Output selection cannot be forecast yet: {error}"),
                );
            }
            card_note(
                ui,
                if selection_mode == OutputSelectionMode::SaveAll {
                    "Save All reserves the engine's complete bounded result-value allowance for \
                     every enabled analysis and Run Set point. Authored outputs and probes still \
                     control initial plotting, and a capture group's overrides still decide how \
                     its own members are sampled and stored. The authenticated ceiling is \
                     rechecked before each terminal analysis is adopted."
                } else {
                    GROUP_NOTE
                },
            );
        },
    );
    let mut policy = state.sim_setup.save_policy;
    if let Some(index) = picked_mode
        && let Some(mode) = OutputSelectionMode::ALL.get(index).copied()
    {
        policy.output_selection_mode = mode;
    }
    if let Some(index) = picked_budget
        && let Some(bytes) = STORAGE_BUDGET_CHOICES.get(index).copied()
    {
        policy.maximum_storage_bytes = bytes;
    }
    if policy != state.sim_setup.save_policy {
        commit_save_policy(state, policy, "Save policy · output selection and storage");
    }
    // A group that holds outputs is not removed on one click. What falls back
    // to the fallback group, and which overrides stop applying to it, is the
    // entire cost of removing a capture policy — and none of it is legible from
    // the strip that removes it.
    let plan = plan_id(state);
    let requested = match head_command.or(command) {
        Some(GroupCommand::Remove(id)) => {
            stage_group_removal(state, plan, &groups, id, outputs, membership)
        }
        other => other,
    };
    // A removal the reader confirmed in the destructive review comes back as
    // the command the strip would have raised, so it is committed by the same
    // transaction, with the same receipt, as a group that cost nothing to
    // remove. It outranks a click in the same frame: it is an answered
    // question, and the click is not.
    //
    // The ledger above was already drawn from the groups this is about to
    // change, so the frame is asked to run again: the confirming click landed
    // in the modal, and nothing else is coming to repaint the row it removed.
    plan.and_then(|plan| {
        state
            .dialogs
            .plan_removal_review
            .take_confirmed_capture_group(plan)
    })
    .inspect(|_| ui.ctx().request_repaint())
    .map(GroupCommand::Remove)
    .or(requested)
}

/// The plan a capture-group edit belongs to.
fn plan_id(state: &AppState) -> Option<crate::product::SimulationPlanId> {
    state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
}

/// Stage the review for a group that holds outputs, or pass the removal
/// straight through for one that holds none.
///
/// Holdings come from the membership the page already resolved, so the count
/// the review states is the count the group's own row states. An empty group is
/// a policy with nothing under it: removing it orphans nothing, and stopping to
/// ask would only teach the reader to click through the reviews that matter.
fn stage_group_removal(
    state: &mut AppState,
    plan: Option<crate::product::SimulationPlanId>,
    groups: &[CaptureGroup],
    id: CaptureGroupId,
    outputs: &[SavedOutput],
    membership: &CaptureGroupMembership,
) -> Option<GroupCommand> {
    let (Some(plan), Some(group)) = (plan, groups.iter().find(|group| group.id == id)) else {
        return Some(GroupCommand::Remove(id));
    };
    let held = outputs
        .iter()
        .enumerate()
        .filter(|(index, _)| membership.owner(*index) == id)
        .map(|(_, output)| output.name.as_str())
        .collect::<Vec<_>>();
    if held.is_empty() {
        return Some(GroupCommand::Remove(id));
    }
    let overrides = [
        group.points.map(|points| points.label()),
        group.streaming.map(|streaming| streaming.label()),
        group.precision.map(|precision| precision.label()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let consequences = vec![
        PlanRemovalConsequence::stated(
            "Outputs it holds",
            format!("{} · {}", held.len(), held.join(", ")),
        )
        .explained(
            PlanRemovalTone::Warn,
            format!(
                "The outputs above fall back to {UNGROUPED_NAME}, which overrides nothing: each \
                 one reverts to the save policy, precision and streaming its own record declares, \
                 and the plan's forecast changes with them."
            ),
        ),
        PlanRemovalConsequence::stated(
            "Overrides it applies",
            if overrides.is_empty() {
                "none".to_owned()
            } else {
                overrides.join(" · ")
            },
        ),
        PlanRemovalConsequence::stated("Rules it resolves by", group.rules.len().to_string())
            .explained(
                PlanRemovalTone::Aside,
                "Removing a group removes a capture policy, never an output. Every output it held \
                 stays in the registry and is still captured by the run.",
            ),
    ];
    state.dialogs.plan_removal_review.open(
        PlanRemovalTarget::CaptureGroup { plan, id },
        group.name.clone(),
        consequences,
    );
    None
}

/// One plan-level allowance, drawn in the group table but deliberately not a
/// group: it is charged to the plan, and giving it a group's row would let a
/// reader add it to a group's subtotal.
fn plan_level_row(ui: &mut Ui, name: &str, detail: &str, bytes: u64) {
    ledger_row(
        ui,
        &GROUP_COLUMNS,
        &[
            (name, Tone::Neutral),
            (detail, Tone::Neutral),
            ("plan", Tone::Neutral),
            (format_bytes(bytes).as_str(), Tone::Neutral),
            ("—", Tone::Neutral),
        ],
        false,
    );
}

/// What a group row explains that its cells cannot.
fn row_hover(
    groups: &[CaptureGroup],
    id: CaptureGroupId,
    outputs: &[SavedOutput],
    membership: &CaptureGroupMembership,
) -> String {
    let Some(group) = groups.iter().find(|group| group.id == id) else {
        return format!(
            "{UNGROUPED_NAME} holds every output no group claims. Its members keep the save \
             policy, precision and streaming their own records declare."
        );
    };
    let mut hover = format!("{} · {}", group.name, group.id);
    if group.rules.is_empty() {
        hover.push_str("\nNo rule: this group holds only the outputs it names.");
    } else {
        hover.push_str("\nRules: ");
        hover.push_str(
            &group
                .rules
                .iter()
                .map(|rule| rule.summary())
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
    // Naming the explicitly held outputs is the one thing the table cannot
    // show, and it is exactly what an engineer checks when a row's count
    // surprises them.
    let named = outputs
        .iter()
        .enumerate()
        .filter(|(index, output)| {
            membership.owner(*index) == id && group.members.contains(&output.id)
        })
        .map(|(_, output)| output.name.as_str())
        .collect::<Vec<_>>();
    if !named.is_empty() {
        hover.push_str(&format!("\nNamed outputs: {}", named.join(", ")));
    }
    hover
}

/// What the card's action strip can ask for.
#[derive(Debug, Clone, Copy)]
pub(super) enum GroupCommand {
    Select(CaptureGroupId),
    Add,
    Edit(CaptureGroupId),
    Remove(CaptureGroupId),
    Raise(CaptureGroupId),
    Lower(CaptureGroupId),
}

/// The action strip, in the card head where a card's own commands belong.
///
/// Every command past "Add group" needs a selected group, so they are disabled
/// rather than absent — a control that appears when a row is clicked teaches
/// nothing about what the card can do. The buttons are the workbench's, not
/// egui's: `ui.button` painted five controls in a chrome nothing else on this
/// page draws, and taller than the card head's own rhythm.
///
/// Authored in reverse because the head lays its controls out right-to-left, so
/// the strip reads Add group · Edit · Remove · Raise · Lower.
fn group_commands(
    ui: &mut Ui,
    groups: &[CaptureGroup],
    selected: Option<CaptureGroupId>,
) -> Option<GroupCommand> {
    let position = selected.and_then(|id| groups.iter().position(|group| group.id == id));
    let mut command = None;
    if Button::new("Lower")
        .enabled(position.is_some_and(|index| index + 1 < groups.len()))
        .show(ui)
        .clicked()
        && let Some(id) = selected
    {
        command = Some(GroupCommand::Lower(id));
    }
    if Button::new("Raise")
        .enabled(position.is_some_and(|index| index > 0))
        .show(ui)
        .on_hover_text(
            "Move this group earlier in resolution order, so its rules take an output another \
             group's rules would otherwise have claimed.",
        )
        .clicked()
        && let Some(id) = selected
    {
        command = Some(GroupCommand::Raise(id));
    }
    if Button::new("Remove")
        .enabled(position.is_some())
        .show(ui)
        .clicked()
        && let Some(id) = selected
    {
        command = Some(GroupCommand::Remove(id));
    }
    if Button::new("Edit")
        .enabled(position.is_some())
        .show(ui)
        .clicked()
        && let Some(id) = selected
    {
        command = Some(GroupCommand::Edit(id));
    }
    if Button::new("Add group").icon(Icon::Add).show(ui).clicked() {
        command = Some(GroupCommand::Add);
    }
    command
}

/// The plan's authored capture groups, in resolution order.
fn plan_capture_groups(state: &AppState) -> Vec<CaptureGroup> {
    state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| state.workspace.plan_data(plan_id))
        .map(|payload| payload.capture_groups.clone())
        .unwrap_or_default()
}

/// Route one command. Removal and reordering are registry edits and go through
/// the shared plan transaction, so each produces a receipt and re-resolves
/// membership exactly like every other edit to the output registry.
///
/// Takes the frame because both of those reach `commit_plan_change`, which owns
/// preflight invalidation as well as the workspace — the two must move
/// together, and a narrower parameter could only move one of them.
pub(super) fn apply_group_command(
    app: &mut RSpiceApp,
    groups: &[CaptureGroup],
    command: GroupCommand,
) {
    let named = |id: CaptureGroupId| {
        groups
            .iter()
            .find(|group| group.id == id)
            .map_or_else(|| UNGROUPED_NAME.to_owned(), |group| group.name.clone())
    };
    match command {
        GroupCommand::Select(id) => {
            app.state.workbench.selected_capture_group = Some(id);
        }
        GroupCommand::Add => {
            app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::CaptureGroup(
                CaptureGroupDraft::default(),
            ));
        }
        GroupCommand::Edit(id) => {
            if let Some(group) = groups.iter().find(|group| group.id == id) {
                app.state.workbench.simulation_workflow =
                    Some(SimulationWorkflowDialog::CaptureGroup(
                        super::super::workflows::group_draft(group),
                    ));
            }
        }
        GroupCommand::Remove(id) => {
            let Ok(plan_id) = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .map(|plan| plan.id())
            else {
                return;
            };
            let detail = format!("Removed capture group {}.", named(id));
            if super::super::workflows::commit_plan_change(
                app,
                plan_id,
                &detail,
                move |workspace, plan_id| {
                    workspace
                        .remove_capture_group(plan_id, id)
                        .map(|_| ())
                        .map_err(|error| error.to_string())
                },
            ) {
                app.state.workbench.selected_capture_group = None;
            }
        }
        GroupCommand::Raise(id) | GroupCommand::Lower(id) => {
            let Ok(plan_id) = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .map(|plan| plan.id())
            else {
                return;
            };
            let toward_front = matches!(command, GroupCommand::Raise(_));
            let detail = format!(
                "Moved capture group {} {} in resolution order.",
                named(id),
                if toward_front { "earlier" } else { "later" }
            );
            super::super::workflows::commit_plan_change(
                app,
                plan_id,
                &detail,
                move |workspace, plan_id| {
                    workspace
                        .reorder_capture_group(plan_id, id, toward_front)
                        .map_err(|error| error.to_string())
                },
            );
        }
    }
}
