//! Desktop/tablet primary workspace rail.

use egui::{Context, Frame, SidePanel, Vec2};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::{ACTIVITY_RAIL_W, WorkbenchIcon, icon_button};
use super::super::state::Workspace;

pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    SidePanel::left("workbench.activity_rail")
        .exact_width(ACTIVITY_RAIL_W)
        .resizable(false)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.add_space(4.0);
            for workspace in Workspace::ALL {
                let active = app.state.workbench.workspace == workspace;
                if icon_button(
                    ui,
                    workspace_icon(workspace),
                    &format!("{} ({})", workspace.label(), workspace.shortcut()),
                    active,
                    Vec2::new(ACTIVITY_RAIL_W, 46.0),
                )
                .clicked()
                {
                    Command::OpenWorkspace(workspace).execute(app);
                }
            }
        });
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
