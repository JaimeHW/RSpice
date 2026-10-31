//! The toolbar's project tools: the launcher row shown while no project is
//! open, and the Project workspace's tools.

use crate::workbench::RSpiceApp;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::project::commands::ProjectCommand;

use super::toolbar_text_command;

pub(in crate::workbench) fn no_project_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_text_command(
        ui,
        app,
        ProjectCommand::ProjectLauncher.key(),
        WorkbenchIcon::Folder,
        "Open project launcher",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        ProjectCommand::NewProject.key(),
        WorkbenchIcon::Add,
        "New project",
        layout,
    );
}

pub(in crate::workbench) fn project_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_text_command(
        ui,
        app,
        ProjectCommand::NewCell.key(),
        WorkbenchIcon::Add,
        "New cell",
        layout,
    );
}
