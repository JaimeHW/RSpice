//! Phone bottom navigation.  Four task-critical owners remain one tap away;
//! Project, Models, and Netlist live in a discoverable More sheet.

use egui::{Align2, Context, Frame, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{PHONE_NAV_H, WorkbenchIcon};
use super::super::state::{Drawer, Workspace};
use super::activity_rail::workspace_icon;

pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    TopBottomPanel::bottom("workbench.phone_navigation")
        .exact_height(PHONE_NAV_H)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let width = ui.available_width() / 5.0;
                for workspace in Workspace::PHONE_PRIMARY {
                    if nav_item(
                        ui,
                        workspace_icon(workspace),
                        workspace.label(),
                        app.state.workbench.workspace == workspace,
                        width,
                    ) {
                        Command::OpenWorkspace(workspace).execute(app);
                    }
                }
                if nav_item(
                    ui,
                    WorkbenchIcon::More,
                    "More",
                    app.state.workbench.drawer == Some(Drawer::Workspaces),
                    width,
                ) {
                    app.state.workbench.toggle_drawer(Drawer::Workspaces);
                }
            });
        });
}

fn nav_item(ui: &mut egui::Ui, icon: WorkbenchIcon, label: &str, active: bool, width: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width, PHONE_NAV_H), egui::Sense::click());
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
            egui::Pos2::new(rect.center().x, rect.top() + 22.0),
            Vec2::splat(19.0),
        ),
        if active {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.center().x, rect.bottom() - 13.0),
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
    response.clicked()
}
