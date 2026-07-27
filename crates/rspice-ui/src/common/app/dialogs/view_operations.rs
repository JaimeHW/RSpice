//! Mockup-authored View workflows.
//!
//! Full-screen entry is reviewed because it changes the host-window contract
//! and may suppress context panels. Reset active view is likewise explicit:
//! it clears only transient presentation state and cannot mutate documents,
//! saved selections, dock geometry, or preferences.

use egui::{Context, Frame, Margin, Stroke, Ui, vec2};

use crate::common::app::{
    AppState, ConsoleMessage, FullScreenPanels, FullScreenScope, RSpiceApp, ViewOperation,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, SelectionImpact, SelectionPreview,
    select_mono_with_response, selection_command_workflow, workflow_preview_status,
};

pub(crate) fn open_full_screen_workflow(state: &mut AppState) {
    state
        .dialogs
        .view_operation
        .open_full_screen(state.workbench.workspace);
}

pub(crate) fn open_reset_active_view_workflow(state: &mut AppState) {
    state
        .dialogs
        .view_operation
        .open_reset_active_view(state.workbench.workspace);
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_view_operation_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.view_operation.open {
            return;
        }

        let operation = self.state.dialogs.view_operation.operation;
        let (eyebrow, title, primary, description) = match operation {
            ViewOperation::FullScreen => (
                "VIEW \u{00b7} WINDOW STATE",
                "Full-screen workspace",
                "Enter full screen",
                "Hide operating-system and application chrome while retaining explicit exit control and the current workspace layout.",
            ),
            ViewOperation::ResetActiveView => (
                "VIEW \u{00b7} DISPLAY STATE",
                "Reset active view",
                "Reset view",
                "Reset pan, zoom, filters, temporary selection, and viewer transforms without changing documents or preferences.",
            ),
        };

        let choice = Dialog::new(eyebrow, title, primary)
            .description(description)
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .initial_focus(DialogInitialFocus::BodyControl)
            .show_with_initial_body_focus(ctx, |ui| {
                Some(match operation {
                    ViewOperation::FullScreen => {
                        full_screen_body(ui, &mut self.state.dialogs.view_operation)
                    }
                    ViewOperation::ResetActiveView => {
                        reset_view_body(ui, &self.state.dialogs.view_operation)
                    }
                })
            });

        match choice {
            DialogChoice::Primary => match operation {
                ViewOperation::FullScreen => {
                    let draft = self.state.dialogs.view_operation.clone();
                    self.state.dialogs.view_operation.close();
                    crate::workbench::enter_full_screen_presentation(
                        self,
                        draft.full_screen_scope,
                        draft.full_screen_panels,
                    );
                }
                ViewOperation::ResetActiveView => {
                    let workspace = self.state.dialogs.view_operation.workspace;
                    if workspace != self.state.workbench.workspace {
                        self.state.push_user_message(ConsoleMessage::warning(
                            "The active workspace changed. Reopen Reset active view to review the new target.",
                        ));
                        self.state.dialogs.view_operation.close();
                    } else {
                        crate::workbench::commands::reset_active_view(self);
                        self.state.dialogs.view_operation.close();
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "{} view state was reset; documents, docks, saved selections, and preferences were preserved.",
                            workspace.label()
                        )));
                    }
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.view_operation.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn full_screen_body(
    ui: &mut Ui,
    draft: &mut crate::common::app::ViewOperationDialogState,
) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    selection_command_workflow(
        ui,
        "FULL",
        &SelectionPreview::Component {
            label: "Active RSpice window and current workspace layout".to_owned(),
        },
        SelectionImpact {
            scope: "active RSpice window",
            effect: "hide application chrome while preserving task state",
            recovery: "Escape restores layout",
        },
        "scope resolved",
        true,
        |ui| {
            let scope = enum_row(
                ui,
                "Scope",
                "Choose whether the host window or only the active RSpice canvas enters the presentation.",
                "view-full-screen-scope",
                draft.full_screen_scope.label(),
                &FullScreenScope::ALL.map(FullScreenScope::label),
            );
            if let Some(index) = scope.0 {
                draft.full_screen_scope = FullScreenScope::ALL[index];
            }

            let panels = enum_row(
                ui,
                "Panels",
                "Context-panel visibility changes only for this full-screen session.",
                "view-full-screen-panels",
                draft.full_screen_panels.label(),
                &FullScreenPanels::ALL.map(FullScreenPanels::label),
            );
            if let Some(index) = panels.0 {
                draft.full_screen_panels = FullScreenPanels::ALL[index];
            }

            readonly_row(
                ui,
                "Exit",
                "F11 or Esc",
                "An explicit exit action remains visible over the active workspace.",
            );
            workflow_preview_status(
                ui,
                true,
                "Presentation state only",
                "Locked, hidden, protected, and out-of-hierarchy objects are unchanged.",
            );
            scope.1
        },
    )
}

fn reset_view_body(ui: &mut Ui, draft: &crate::common::app::ViewOperationDialogState) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let first = readonly_row(
        ui,
        "Workspace",
        draft.workspace.label(),
        "The target is captured when this review opens and is checked again before commit.",
    );
    readonly_row(
        ui,
        "Reset",
        "pan \u{00b7} zoom \u{00b7} temporary filters",
        "Transient selection and viewer transforms return to their canonical defaults.",
    );
    readonly_row(
        ui,
        "Preserve",
        "open documents \u{00b7} docks \u{00b7} saved selections",
        "Project data, window layout, and user preferences are not modified.",
    );
    first
}

fn enum_row(
    ui: &mut Ui,
    label: &str,
    detail: &str,
    id_salt: &str,
    selected: &str,
    options: &[&str],
) -> (Option<usize>, egui::Id) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            let options = options.iter().map(ToString::to_string).collect::<Vec<_>>();
            if ui.available_width() < 440.0 {
                setting_copy(ui, label, detail);
                ui.add_space(7.0);
                let control_width = ui.available_width();
                return select_mono_with_response(
                    ui,
                    id_salt,
                    label,
                    selected,
                    &options,
                    control_width,
                );
            }
            let control_width = (ui.available_width() * 0.47).clamp(220.0, 330.0);
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    vec2(
                        (ui.available_width() - control_width - 16.0).max(150.0),
                        44.0,
                    ),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.label(
                            egui::RichText::new(label)
                                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    },
                );
                ui.add_space(16.0);
                select_mono_with_response(ui, id_salt, label, selected, &options, control_width)
            })
            .inner
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let output = row.inner;
    (output.picked, output.response.id)
}

fn readonly_row(ui: &mut Ui, label: &str, value: &str, detail: &str) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            if ui.available_width() < 440.0 {
                setting_copy(ui, label, detail);
                ui.add_space(7.0);
                return ui
                    .label(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    )
                    .id;
            }
            let response = ui
                .horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        vec2((ui.available_width() * 0.43).clamp(170.0, 270.0), 44.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.label(
                                egui::RichText::new(label)
                                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            ui.label(
                                egui::RichText::new(detail)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                        },
                    );
                    ui.add_space(16.0);
                    ui.label(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    )
                })
                .inner;
            response.id
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    row.inner
}

fn setting_copy(ui: &mut Ui, label: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(detail)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}
