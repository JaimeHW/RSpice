//! Desktop/tablet primary workspace rail.

use egui::{Context, Frame, Sense, SidePanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::{ACTIVITY_RAIL_W, WorkbenchIcon};
use super::super::state::Workspace;

const ACTIVITY_TOP_PADDING: f32 = 5.0;
const ACTIVITY_BUTTON_WIDTH: f32 = 50.0;
const ACTIVITY_BUTTON_HEIGHT: f32 = 45.0;
const ACTIVITY_ICON_SIZE: f32 = 19.0;
const ACTIVITY_ACTIVE_EDGE: f32 = 2.0;

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
            for workspace in Workspace::ALL {
                let active = app.state.workbench.workspace == workspace;
                let command = Command::OpenWorkspace(workspace);
                let response = ui
                    .add_enabled_ui(command.is_enabled(app), |ui| {
                        activity_button(ui, workspace, active)
                    })
                    .inner;
                if response.clicked() {
                    command.execute(app);
                }
            }
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Navigation);
        node.set_label("Primary workspaces");
    });
}

fn activity_button(ui: &mut egui::Ui, workspace: Workspace, active: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let label = format!("{} ({})", workspace.label(), workspace.shortcut());
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
    if active {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + ACTIVITY_ACTIVE_EDGE, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    workspace_icon(workspace).paint(
        ui.painter(),
        egui::Rect::from_center_size(rect.center(), Vec2::splat(ACTIVITY_ICON_SIZE)),
        if active {
            t.color.accent
        } else if response.hovered() {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
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
    }
}
