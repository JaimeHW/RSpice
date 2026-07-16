//! Desktop/tablet primary workspace rail.

use egui::{Context, Frame, Sense, SidePanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{ACTIVITY_RAIL_W, WorkbenchIcon};
use super::super::state::Workspace;

const ACTIVITY_TOP_PADDING: f32 = 5.0;
const ACTIVITY_BUTTON_WIDTH: f32 = 50.0;
const ACTIVITY_BUTTON_HEIGHT: f32 = 45.0;
const ACTIVITY_ICON_SIZE: f32 = 19.0;
const ACTIVITY_ACTIVE_EDGE: f32 = 2.0;
const ACTIVITY_BADGE_SIZE: f32 = 15.0;
const ACTIVITY_BADGE_TOP: f32 = 5.0;
const ACTIVITY_BADGE_RIGHT: f32 = 7.0;

fn activity_badge_rect(button_rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            button_rect.right() - ACTIVITY_BADGE_RIGHT - ACTIVITY_BADGE_SIZE,
            button_rect.top() + ACTIVITY_BADGE_TOP,
        ),
        Vec2::splat(ACTIVITY_BADGE_SIZE),
    )
}

pub(super) fn paint_numeric_badge(
    ui: &egui::Ui,
    rect: egui::Rect,
    border_color: egui::Color32,
    count: usize,
) {
    if count == 0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    ui.painter().rect(
        rect,
        ACTIVITY_BADGE_SIZE * 0.5,
        t.color.accent,
        egui::Stroke::new(1.0, border_color),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.accent_ink,
    );
}

pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    let shown = SidePanel::left("workbench.activity_rail")
        .exact_width(ACTIVITY_RAIL_W)
        .resizable(false)
        .frame(Frame::new().fill(t.color.bg_app))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.add_space(ACTIVITY_TOP_PADDING);
            if app.state.workbench.workspace == Workspace::Results {
                app.state.ui.results_seen_version = app.state.simulation.data_version;
            }
            for workspace in Workspace::ALL {
                let active = app.state.workbench.workspace == workspace;
                let new_result_count = usize::from(
                    workspace == Workspace::Results
                        && !active
                        && app.state.simulation.has_results()
                        && app.state.ui.results_seen_version != app.state.simulation.data_version,
                );
                let command = Command::OpenWorkspace(workspace);
                let response = ui
                    .add_enabled_ui(command.is_enabled(app), |ui| {
                        activity_button(ui, workspace, active, new_result_count)
                    })
                    .inner;
                if response.clicked() {
                    if workspace == Workspace::Results {
                        app.state.ui.results_seen_version = app.state.simulation.data_version;
                    }
                    command.execute(app);
                    ui.ctx().request_repaint();
                }
            }
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Navigation);
        node.set_label("Primary workspaces");
    });
}

fn activity_button(
    ui: &mut egui::Ui,
    workspace: Workspace,
    active: bool,
    badge_count: usize,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let label = if badge_count > 0 {
        format!(
            "{} ({}) · {badge_count} new result",
            workspace.label(),
            workspace.shortcut()
        )
    } else {
        format!("{} ({})", workspace.label(), workspace.shortcut())
    };
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ACTIVITY_BUTTON_WIDTH, ACTIVITY_BUTTON_HEIGHT),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            active,
            workspace.label(),
        )
    });
    if active && enabled {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + ACTIVITY_ACTIVE_EDGE, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    } else if enabled && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    workspace_icon(workspace).paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), Vec2::splat(ACTIVITY_ICON_SIZE)),
        if !enabled {
            t.color.text_faint.gamma_multiply(0.45)
        } else if active {
            t.color.accent
        } else if response.hovered() {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    paint_numeric_badge(ui, activity_badge_rect(rect), t.color.bg_app, badge_count);
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(label)
}

pub const fn workspace_icon(workspace: Workspace) -> WorkbenchIcon {
    match workspace {
        Workspace::Project => WorkbenchIcon::Project,
        Workspace::Design => WorkbenchIcon::Design,
        Workspace::Simulate => WorkbenchIcon::Simulate,
        Workspace::Results => WorkbenchIcon::Results,
        Workspace::Verify => WorkbenchIcon::Verify,
        Workspace::Models => WorkbenchIcon::Models,
        Workspace::Netlist => WorkbenchIcon::Netlist,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_activity_rail_geometry_matches_the_mockup() {
        assert_eq!(ACTIVITY_RAIL_W, 51.0);
        assert_eq!(ACTIVITY_TOP_PADDING, 5.0);
        assert_eq!(ACTIVITY_BUTTON_WIDTH, 50.0);
        assert_eq!(ACTIVITY_BUTTON_HEIGHT, 45.0);
        assert_eq!(ACTIVITY_ICON_SIZE, 19.0);
        assert_eq!(ACTIVITY_ACTIVE_EDGE, 2.0);
        assert_eq!(ACTIVITY_BADGE_SIZE, 15.0);
        let badge = activity_badge_rect(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(ACTIVITY_BUTTON_WIDTH, ACTIVITY_BUTTON_HEIGHT),
        ));
        assert_eq!(badge.min, egui::pos2(28.0, 5.0));
        assert_eq!(badge.size(), Vec2::splat(15.0));
        let button = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(ACTIVITY_BUTTON_WIDTH, ACTIVITY_BUTTON_HEIGHT),
        );
        assert!(button.contains_rect(badge));
        assert_eq!(button.right() - badge.right(), ACTIVITY_BADGE_RIGHT);
    }
}
