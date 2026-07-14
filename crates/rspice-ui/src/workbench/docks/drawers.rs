//! Overlay drawers for tablet and phone layouts.

use egui::{Align2, Area, Context, Frame, Id, Order, Sense};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use super::super::chrome::activity_rail::workspace_icon;
use super::super::commands::Command;
use super::super::design_system::{PHONE_NAV_H, STATUS_BAR_H, TITLE_BAR_H, TOOL_BAR_H};
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, Workspace};

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let Some(drawer) = app.state.workbench.drawer else {
        return;
    };
    let screen = ctx.content_rect();
    let top = TITLE_BAR_H + TOOL_BAR_H;
    let bottom = STATUS_BAR_H
        + if layout.show_phone_navigation {
            PHONE_NAV_H
        } else {
            0.0
        };
    let available_height = (screen.height() - top - bottom).max(180.0);
    let t = Tokens::get(ctx);

    let scrim = Area::new(Id::new("workbench.drawer.scrim"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(screen.size(), Sense::click());
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::from_black_alpha(112));
            response
        })
        .inner;
    if scrim.clicked() {
        app.state.workbench.drawer = None;
        return;
    }

    match drawer {
        Drawer::Navigator => {
            Area::new(Id::new("workbench.drawer.navigator"))
                .order(Order::Tooltip)
                .fixed_pos(egui::pos2(screen.left(), screen.top() + top))
                .show(ctx, |ui| {
                    Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
                        ui.set_width(screen.width().min(360.0));
                        ui.set_height(available_height);
                        super::navigator::show(ui, app);
                    });
                });
        }
        Drawer::Inspector => {
            let width = screen.width().min(380.0);
            Area::new(Id::new("workbench.drawer.inspector"))
                .order(Order::Tooltip)
                .fixed_pos(egui::pos2(screen.right() - width, screen.top() + top))
                .show(ctx, |ui| {
                    Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
                        ui.set_width(width);
                        ui.set_height(available_height);
                        super::inspector::show(ui, app);
                    });
                });
        }
        Drawer::Workspaces => {
            let width = screen.width().min(440.0);
            Area::new(Id::new("workbench.drawer.workspaces"))
                .order(Order::Tooltip)
                .anchor(Align2::CENTER_BOTTOM, egui::vec2(0.0, -bottom))
                .show(ctx, |ui| {
                    Frame::new()
                        .fill(t.color.bg_panel)
                        .corner_radius(12.0)
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_width(width - 24.0);
                            ui.heading("All workspaces");
                            ui.add_space(6.0);
                            for workspace in
                                [Workspace::Project, Workspace::Models, Workspace::Netlist]
                            {
                                let selected = app.state.workbench.workspace == workspace;
                                if super::super::design_system::labeled_icon_button(
                                    ui,
                                    workspace_icon(workspace),
                                    workspace.label(),
                                    selected,
                                    ui.available_width(),
                                )
                                .clicked()
                                {
                                    Command::OpenWorkspace(workspace).execute(app);
                                }
                            }
                        });
                });
        }
    }
}
