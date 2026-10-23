//! The toolbars of the Models workspace and of the model editor route.

use crate::workbench::RSpiceApp;
use crate::workbench::RouteTransitionSource;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, labeled_icon_button_sized};
use crate::workbench::layout::LayoutSpec;

use super::{
    context_separator, explicit_label_width, take_projected_tool_slot, toolbar_icon_command,
    toolbar_text_command,
};

const MODELS_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, Option<&str>); 4] = [
    (Command::Save, WorkbenchIcon::Save, None),
    (
        Command::PdkSettings,
        WorkbenchIcon::Add,
        Some("Add library"),
    ),
    (Command::RescanModelLibraries, WorkbenchIcon::Refresh, None),
    (Command::CompileVerilogA, WorkbenchIcon::Netlist, None),
];

pub(super) fn model_editor_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if let Some(origin) = app.state.workbench.previous_route() {
        let label = format!("Source: {}", origin.surface_id().label());
        if take_projected_tool_slot(ui, layout)
            && labeled_icon_button_sized(
                ui,
                WorkbenchIcon::ArrowLeft,
                &label,
                false,
                explicit_label_width(&label),
                layout.toolbar_control_height,
            )
            .clicked()
        {
            app.state
                .workbench
                .navigate_back(RouteTransitionSource::User);
            return;
        }
        context_separator(ui, layout);
    }
    toolbar_text_command(
        ui,
        app,
        Command::ModelSaveRevision,
        WorkbenchIcon::Save,
        "Save model revision",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelValidate,
        WorkbenchIcon::Check,
        "Validate model",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelRunQualificationTests,
        WorkbenchIcon::Run,
        "Run qualification tests",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ModelCompareRelease,
        WorkbenchIcon::Compare,
        "Compare with release",
        layout,
    );
}

pub(super) fn models_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    for (index, (command, icon, label)) in MODELS_TOOLBAR_COMMANDS.into_iter().enumerate() {
        if matches!(index, 1 | 3) {
            context_separator(ui, layout);
        }
        if let Some(label) = label {
            toolbar_text_command(ui, app, command, icon, label, layout);
        } else {
            toolbar_icon_command(ui, app, command, icon, layout);
        }
    }
}
