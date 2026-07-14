//! Phone bottom navigation.  Four task-critical owners remain one tap away;
//! Project, Models, and Netlist live in a discoverable More sheet.

use egui::{Align2, Context, Frame, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, Workspace};
use super::activity_rail::workspace_icon;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::bottom("workbench.phone_navigation")
        .exact_height(layout.phone_navigation_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.horizontal(|ui| {
                let item_count = if layout.workspaces_uses_drawer {
                    5.0
                } else {
                    7.0
                };
                let width = ui.available_width() / item_count;
                let workspaces: &[Workspace] = if layout.workspaces_uses_drawer {
                    &Workspace::PHONE_PRIMARY
                } else {
                    &Workspace::ALL
                };
                for workspace in workspaces.iter().copied() {
                    if nav_item(
                        ui,
                        workspace_icon(workspace),
                        workspace.label(),
                        app.state.workbench.workspace == workspace,
                        width,
                        layout.phone_navigation_height,
                    ) {
                        Command::OpenWorkspace(workspace).execute(app);
                    }
                }
                if layout.workspaces_uses_drawer
                    && nav_item(
                        ui,
                        WorkbenchIcon::More,
                        "More",
                        app.state.workbench.drawer == Some(Drawer::Workspaces),
                        width,
                        layout.phone_navigation_height,
                    )
                {
                    app.state.workbench.toggle_drawer(Drawer::Workspaces);
                }
            });
        });
}

fn nav_item(
    ui: &mut egui::Ui,
    icon: WorkbenchIcon,
    label: &str,
    active: bool,
    width: f32,
    height: f32,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            active,
            label,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
    });
    if active {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::Pos2::new(rect.right(), rect.top() + 2.0),
            ),
            0.0,
            t.color.accent,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.center().x, rect.top() + 20.0),
            Vec2::splat(17.0),
        ),
        if active {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.center().x, rect.top() + 36.5),
        Align2::CENTER_CENTER,
        label,
        theme::sans(
            tokens::FS_0,
            if active {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        ),
        if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}
