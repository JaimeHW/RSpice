//! The plan-editing workflows: clone a plan, add a design variable, save an output.
//!
//! Each workflow is a draft that is validated before it is committed, so a
//! dialog can be dismissed at any point without leaving the plan half-edited.
//! The draft also reports who consumes the thing being edited, because these
//! edits are shared across every analysis in the plan.

mod capture_group;
mod clone_plan;
mod commit;
mod output;
mod rename_analysis;
mod variable;

pub use capture_group::*;
pub use clone_plan::*;
pub use commit::*;
pub(crate) use output::*;
use rename_analysis::*;
pub(crate) use variable::*;

use super::*;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::design_history::SetupParts;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::slots::SimulateState;

use crate::state::{
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming,
};
use crate::ui::widgets::IconButton;
use crate::workbench::state::{CaptureGroupDraft, CaptureGroupRuleDraft};

pub(super) fn simulation_workflow_dialog(ctx: &egui::Context, cx: &mut Cx<'_, SimulateFeature>) {
    let Some(workflow) = cx.state.simulation_workflow.clone() else {
        return;
    };
    match workflow {
        SimulationWorkflowDialog::PlanManager(draft) => {
            super::plan_manager::plan_manager_dialog(ctx, cx, draft);
        }
        SimulationWorkflowDialog::ClonePlan(draft) => clone_plan_dialog(ctx, cx, draft),
        SimulationWorkflowDialog::DesignVariable(draft) => design_variable_dialog(ctx, cx, draft),
        SimulationWorkflowDialog::SavedOutput(draft) => saved_output_dialog(ctx, cx, draft),
        SimulationWorkflowDialog::RenameAnalysis(draft) => {
            rename_analysis_dialog(ctx, cx, draft);
        }
        SimulationWorkflowDialog::CaptureGroup(draft) => capture_group_dialog(ctx, cx, draft),
        SimulationWorkflowDialog::AnalysisRunPoints(draft) => {
            super::participation::analysis_run_points_dialog(ctx, cx, draft);
        }
        SimulationWorkflowDialog::DesignVariableImport(draft) => {
            super::variable_import::design_variable_import_dialog(ctx, cx, draft);
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

/// Two panes of one dialog body, side by side above a breakpoint and stacked
/// under it, over a state both of them edit.
///
/// The state is threaded through rather than captured, because two closures
/// built at one call site cannot both hold `&mut` on the same draft — and a
/// split whose halves may not edit the thing the dialog is editing is not much
/// of a split. [`workflow_split`] is this with no state.
pub(super) fn workflow_split_over<S>(
    ui: &mut Ui,
    state: &mut S,
    left: impl FnOnce(&mut Ui, &mut S),
    right: impl FnOnce(&mut Ui, &mut S),
) {
    let t = Tokens::get(ui.ctx());
    if ui.available_width() >= 620.0 {
        let width = ui.available_width();
        let divider = 1.0;
        let pane_width = ((width - divider) * 0.5).max(1.0);
        let response = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, |ui| left(ui, state))
            });
            ui.allocate_ui_with_layout(vec2(pane_width, 0.0), Layout::top_down(Align::Min), |ui| {
                workflow_split_pane(ui, |ui| right(ui, state))
            });
        });
        let divider_x = response.response.rect.left() + pane_width;
        ui.painter().vline(
            divider_x,
            response.response.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    } else {
        let left_response = workflow_split_pane(ui, |ui| left(ui, state));
        ui.painter().hline(
            left_response.rect.x_range(),
            left_response.rect.bottom(),
            Stroke::new(1.0, t.color.border_strong),
        );
        workflow_split_pane(ui, |ui| right(ui, state));
    }
}

pub(super) fn workflow_split(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    workflow_split_over(ui, &mut (), |ui, ()| left(ui), |ui, ()| right(ui));
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
        mono_input(ui, label, value, ui.available_width());
    } else {
        let height = Tokens::get(ui.ctx()).metrics.ctl_h;
        let response = ui.add_sized(
            vec2(ui.available_width(), height),
            egui::TextEdit::singleline(value).margin(egui::Margin::symmetric(8, 4)),
        );
        crate::ui::widgets::name_control(ui, &response, label);
    }
}

pub(super) fn workflow_multiline_field(ui: &mut Ui, label: &str, value: &mut String) {
    workflow_field_label(ui, label);
    let response = ui.add_sized(
        vec2(ui.available_width(), 74.0),
        egui::TextEdit::multiline(value)
            .font(crate::ui::theme::field_mono())
            .margin(egui::Margin::symmetric(8, 6)),
    );
    crate::ui::widgets::name_control(ui, &response, label);
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
/// rhythm, but the switch itself keeps its natural compact width at the left.
/// `add_sized` centres a widget in the space it is given, which put labels of
/// different lengths at different left edges and read as a broken list rather
/// than as several choices about the same thing.
///
/// Named for what it paints. It was `workflow_checkbox`, and it painted egui's
/// tick box; the dialogs are part of the simulation stage, where the mockup's
/// every boolean is a `label.switch`, so it paints the studio's own switch and
/// says so.
pub(super) fn workflow_switch(ui: &mut Ui, label: &str, value: &mut bool) {
    let height = Tokens::get(ui.ctx()).metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), height),
        Layout::left_to_right(Align::Center),
        |ui| {
            super::page_kit::switch_row(ui, label, value);
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
                // The mark owns the colour; the refusal itself reads in the
                // same ink as every other line in the dialog.
                crate::ui::widgets::problem_line(ui, crate::ui::widgets::Problem::Failure, error);
            })
            .response;
        ui.painter().hline(
            response.rect.x_range(),
            response.rect.top(),
            Stroke::new(1.0, t.color.border),
        );
    }
}
