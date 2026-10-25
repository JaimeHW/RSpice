//! The toolbar of the report authoring route.

use crate::workbench::RSpiceApp;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::hardcopy::commands::HardcopyCommand;
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
        HardcopyCommand::SaveReportDocument.key(),
        WorkbenchIcon::Save,
        "Save report document",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        HardcopyCommand::AddReportPage.key(),
        WorkbenchIcon::Add,
        "Add report page",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        HardcopyCommand::ReportPageProperties.key(),
        WorkbenchIcon::Sliders,
        "Page properties",
        layout,
    );
    // The mockup's Build review draft action remains absent until exact plot
    // artwork publication and the release-handoff executor complete the full
    // contract. Report authoring never advertises a non-executable artifact.
}
