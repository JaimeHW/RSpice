//! The toolbar's project tools: the launcher row shown while no project is
//! open, and the Project workspace's tools.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::layout::LayoutSpec;

use super::toolbar_text_command;

pub(super) fn no_project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::ProjectLauncher,
        WorkbenchIcon::Folder,
        "Open project launcher",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::NewProject,
        WorkbenchIcon::Add,
        "New project",
        layout,
    );
}

pub(super) fn project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::NewCell,
        WorkbenchIcon::Add,
        "New cell",
        layout,
    );
}
