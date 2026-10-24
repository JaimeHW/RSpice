//! The toolbar of the report authoring route.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::layout::LayoutSpec;

use super::toolbar_text_command;

pub(in crate::workbench) fn report_authoring_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_text_command(
        ui,
        app,
        Command::SaveReportDocument,
        WorkbenchIcon::Save,
        "Save report document",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::AddReportPage,
        WorkbenchIcon::Add,
        "Add report page",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ReportPageProperties,
        WorkbenchIcon::Sliders,
        "Page properties",
        layout,
    );
    // The mockup's Build review draft action remains absent until exact plot
    // artwork publication and the release-handoff executor complete the full
    // contract. Report authoring never advertises a non-executable artifact.
}
