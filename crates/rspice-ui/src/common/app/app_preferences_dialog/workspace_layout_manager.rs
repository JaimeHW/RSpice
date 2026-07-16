//! Mockup-defined device-local workspace layout manager.

use egui::{Context, Grid, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, select};
use crate::workbench::{WorkspacePreset, state::Workspace};

use super::AppState;
use super::preferences_shell::{right_aligned, setting_row};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LayoutTemplate {
    #[default]
    Engineering,
    Canvas,
    Diagnostics,
    ResultsReview,
}

impl LayoutTemplate {
    const ALL: [Self; 4] = [
        Self::Engineering,
        Self::Canvas,
        Self::Diagnostics,
        Self::ResultsReview,
    ];

    const LABELS: [&'static str; 4] = ["Engineering", "Canvas", "Diagnostics", "Results review"];
    const DETAILS: [&'static str; 4] = [
        "Navigator · editor · inspector · console",
        "Editor-focused with hidden docks",
        "Wide console and inspector",
        "Wide plot · measurements · no navigator",
    ];

    const fn index(self) -> usize {
        match self {
            Self::Engineering => 0,
            Self::Canvas => 1,
            Self::Diagnostics => 2,
            Self::ResultsReview => 3,
        }
    }

    const fn workspace_preset(self) -> Option<WorkspacePreset> {
        match self {
            Self::Engineering => Some(WorkspacePreset::Engineering),
            Self::Canvas => Some(WorkspacePreset::Canvas),
            Self::Diagnostics => Some(WorkspacePreset::Diagnostics),
            Self::ResultsReview => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LayoutScope {
    #[default]
    CurrentWorkspace,
    AllWorkspaces,
}

impl LayoutScope {
    const LABELS: [&'static str; 2] = [
        "Current workspace on this device",
        "All workspaces on this device",
    ];

    const fn index(self) -> usize {
        match self {
            Self::CurrentWorkspace => 0,
            Self::AllWorkspaces => 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkspaceLayoutManagerState {
    pub(crate) open: bool,
    template: LayoutTemplate,
    scope: LayoutScope,
}

impl WorkspaceLayoutManagerState {
    pub(crate) fn open(&mut self, preset: WorkspacePreset) {
        self.open = true;
        self.template = match preset {
            WorkspacePreset::Engineering => LayoutTemplate::Engineering,
            WorkspacePreset::Canvas => LayoutTemplate::Canvas,
            WorkspacePreset::Diagnostics => LayoutTemplate::Diagnostics,
        };
        self.scope = LayoutScope::CurrentWorkspace;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
    }
}

pub(super) fn render(ctx: &Context, state: &mut AppState) {
    if !state.dialogs.workspace_layout_manager.open {
        return;
    }

    let mut draft = state.dialogs.workspace_layout_manager.clone();
    let choice = Dialog::new(
        "WINDOW \u{00b7} DOCKS \u{00b7} DOCUMENTS \u{00b7} DEVICE LOCAL",
        "Workspace layout manager",
        "Apply selected layout",
    )
    .description("Apply a device-local dock and console composition without changing project data.")
    .size(DialogSize::Manager)
    .show(ctx, |ui| {
        render_templates(ui, &mut draft);
        render_workspace_table(ui, state);
        render_scope(ui, &mut draft);
    });

    match choice {
        DialogChoice::Primary => {
            apply_layout(state, draft.template, draft.scope);
            state.dialogs.workspace_layout_manager.close();
        }
        DialogChoice::Cancelled => state.dialogs.workspace_layout_manager.close(),
        _ => state.dialogs.workspace_layout_manager = draft,
    }
}

fn render_templates(ui: &mut Ui, state: &mut WorkspaceLayoutManagerState) {
    ui.heading("Layout preset");
    ui.label("Choose the dock, document and console arrangement to apply.");
    ui.add_space(8.0);
    let columns = if ui.ctx().content_rect().width() <= 560.0 {
        1
    } else {
        2
    };
    for chunk in LayoutTemplate::ALL.chunks(columns) {
        ui.columns(columns, |cells| {
            for (column, template) in chunk.iter().copied().enumerate() {
                if layout_template_card(
                    &mut cells[column],
                    LayoutTemplate::LABELS[template.index()],
                    LayoutTemplate::DETAILS[template.index()],
                    state.template == template,
                ) {
                    state.template = template;
                }
            }
        });
        ui.add_space(8.0);
    }
}

fn layout_template_card(ui: &mut Ui, label: &str, detail: &str, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 62.0), Sense::click());
    ui.painter().rect(
        rect,
        t.radius,
        if selected {
            t.color.bg_active
        } else {
            t.color.bg_inset
        },
        Stroke::new(
            1.0,
            if selected {
                t.color.accent
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    let mut content = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(12.0, 8.0)))
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    content.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    content.add(
        egui::Label::new(
            egui::RichText::new(detail)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::RadioButton, true, selected, label)
    });
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

fn render_workspace_table(ui: &mut Ui, state: &AppState) {
    ui.add_space(12.0);
    ui.heading("Remembered layouts");
    egui::ScrollArea::horizontal()
        .id_salt("preferences.workspace.layout-manager.table-scroll")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(600.0);
            Grid::new("preferences.workspace.layout-manager.table")
                .num_columns(5)
                .striped(true)
                .show(ui, |ui| {
                    for header in [
                        "Workspace",
                        "Navigator",
                        "Inspector",
                        "Console",
                        "Remembered",
                    ] {
                        ui.strong(header);
                    }
                    ui.end_row();
                    for workspace in [
                        Workspace::Design,
                        Workspace::Simulate,
                        Workspace::Results,
                        Workspace::Verify,
                    ] {
                        let layout = state.workbench.workspace_layout(workspace);
                        ui.label(if workspace == Workspace::Verify {
                            "DRC"
                        } else {
                            workspace.label()
                        });
                        ui.label(if layout.navigator_visible && !layout.focus_mode {
                            format!("{:.0} px", layout.navigator_width)
                        } else {
                            "collapsed".to_owned()
                        });
                        ui.label(if layout.inspector_visible && !layout.focus_mode {
                            format!("{:.0} px", layout.inspector_width)
                        } else {
                            "collapsed".to_owned()
                        });
                        ui.label(if layout.console_visible && !layout.focus_mode {
                            format!("{:.0} px", layout.console_height)
                        } else {
                            "collapsed".to_owned()
                        });
                        ui.label("this device");
                        ui.end_row();
                    }
                });
        });
}

fn render_scope(ui: &mut Ui, state: &mut WorkspaceLayoutManagerState) {
    ui.add_space(12.0);
    setting_row(
        ui,
        "Scope",
        "Layout never changes portable project documents or result data.",
        |ui| {
            right_aligned(ui, |ui| {
                let labels = LayoutScope::LABELS.map(str::to_owned);
                let selected = select(
                    ui,
                    "preferences.workspace.layout-manager.scope",
                    "Workspace layout scope",
                    LayoutScope::LABELS[state.scope.index()],
                    &labels,
                    ui.available_width().min(360.0),
                );
                if let Some(selected) = selected {
                    state.scope = if selected == 0 {
                        LayoutScope::CurrentWorkspace
                    } else {
                        LayoutScope::AllWorkspaces
                    };
                }
            });
        },
    );
}

fn apply_layout(state: &mut AppState, template: LayoutTemplate, scope: LayoutScope) {
    if let Some(preset) = template.workspace_preset() {
        match scope {
            LayoutScope::CurrentWorkspace => state.workbench.apply_workspace_preset(preset),
            LayoutScope::AllWorkspaces => state.workbench.apply_preset_to_all_workspaces(preset),
        }
        state
            .ui
            .preferences
            .workspace_mut()
            .expect("the manager opens only for a compatible Workspace preference domain")
            .set_preset(preset);
    } else {
        match scope {
            LayoutScope::CurrentWorkspace => state.workbench.apply_results_review_layout(),
            LayoutScope::AllWorkspaces => state.workbench.apply_results_review_to_all_workspaces(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_template_and_scope_order_are_stable() {
        assert_eq!(
            LayoutTemplate::LABELS,
            ["Engineering", "Canvas", "Diagnostics", "Results review"]
        );
        assert_eq!(
            LayoutScope::LABELS,
            [
                "Current workspace on this device",
                "All workspaces on this device"
            ]
        );
    }
}
