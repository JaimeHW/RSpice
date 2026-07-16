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
use super::activity_rail::{paint_numeric_badge, workspace_icon};

const PHONE_ACTIVE_EDGE: f32 = 2.0;
const PHONE_ICON_SIZE: f32 = 17.0;
const PHONE_ICON_CENTER_Y: f32 = 20.0;
const PHONE_LABEL_CENTER_Y: f32 = 36.5;
const PHONE_BADGE_SIZE: f32 = 15.0;
const PHONE_BADGE_TOP: f32 = 2.0;
const PHONE_BADGE_RIGHT_OF_CENTER: f32 = 18.0;

fn phone_badge_rect(item_rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            item_rect.center().x + PHONE_BADGE_RIGHT_OF_CENTER - PHONE_BADGE_SIZE,
            item_rect.top() + PHONE_BADGE_TOP,
        ),
        Vec2::splat(PHONE_BADGE_SIZE),
    )
}

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let shown = TopBottomPanel::bottom("workbench.phone_navigation")
        .exact_height(layout.phone_navigation_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            if app.state.workbench.workspace == Workspace::Results {
                app.state.ui.results_seen_version = app.state.simulation.data_version;
            }
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
                    let active = app.state.workbench.workspace == workspace;
                    let new_result_count = usize::from(
                        workspace == Workspace::Results
                            && !active
                            && app.state.simulation.has_results()
                            && app.state.ui.results_seen_version
                                != app.state.simulation.data_version,
                    );
                    let command = Command::OpenWorkspace(workspace);
                    let response = ui
                        .add_enabled_ui(command.is_enabled(app), |ui| {
                            nav_item(
                                ui,
                                workspace_icon(workspace),
                                workspace.label(),
                                active,
                                new_result_count,
                                width,
                                layout.phone_navigation_height,
                            )
                        })
                        .inner;
                    if response.clicked() {
                        if workspace == Workspace::Results {
                            app.state.ui.results_seen_version = app.state.simulation.data_version;
                        }
                        command.execute(app);
                    }
                }
                if layout.workspaces_uses_drawer {
                    let more = nav_item(
                        ui,
                        WorkbenchIcon::More,
                        "More",
                        app.state.workbench.drawer == Some(Drawer::Workspaces),
                        0,
                        width,
                        layout.phone_navigation_height,
                    );
                    if more.clicked() {
                        ui.ctx().data_mut(|data| {
                            data.insert_temp(
                                egui::Id::new("workbench.mobile_navigation.invoker"),
                                more.id,
                            );
                        });
                        app.state.workbench.toggle_drawer(Drawer::Workspaces);
                    }
                }
            });
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Navigation);
        node.set_label("Primary workspaces");
    });
}

fn nav_item(
    ui: &mut egui::Ui,
    icon: WorkbenchIcon,
    label: &str,
    active: bool,
    badge_count: usize,
    width: f32,
    height: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), active, label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Button);
        node.set_selected(active);
    });
    if active {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::Pos2::new(rect.right(), rect.top() + PHONE_ACTIVE_EDGE),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.center().x, rect.top() + PHONE_ICON_CENTER_Y),
            Vec2::splat(PHONE_ICON_SIZE),
        ),
        if !ui.is_enabled() {
            t.color.text_faint
        } else if active {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    paint_numeric_badge(ui, phone_badge_rect(rect), t.color.bg_panel, badge_count);
    ui.painter().text(
        egui::Pos2::new(rect.center().x, rect.top() + PHONE_LABEL_CENTER_Y),
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
        if !ui.is_enabled() {
            t.color.text_faint
        } else if active {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_navigation_uses_the_mockup_five_and_seven_item_projections() {
        assert_eq!(Workspace::PHONE_PRIMARY.len() + 1, 5);
        assert_eq!(Workspace::ALL.len(), 7);
    }

    #[test]
    fn compact_navigation_geometry_matches_the_mockup() {
        assert_eq!(PHONE_ACTIVE_EDGE, 2.0);
        assert_eq!(PHONE_ICON_SIZE, 17.0);
        assert_eq!(PHONE_ICON_CENTER_Y, 20.0);
        assert_eq!(PHONE_LABEL_CENTER_Y, 36.5);
        assert_eq!(PHONE_BADGE_SIZE, 15.0);
        let item = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(80.0, 52.0));
        let badge = phone_badge_rect(item);
        assert_eq!(badge.min, egui::pos2(43.0, 2.0));
        assert_eq!(badge.size(), Vec2::splat(15.0));
    }
}
