//! Typed workbench commands and their single dispatch boundary.
//!
//! The workbench never paints a visible action without routing it here.  A
//! command is omitted from a menu when its behavior is not implemented; the
//! UI does not advertise speculative or placeholder capability.

use crate::common::RSpiceApp;
use crate::common::menu_bar::{FileMenuAction, dispatch_file_menu_action};
use crate::state::{ComponentType, Tool};

use super::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    OpenWorkspace(Workspace),
    ProjectLauncher,
    NewProject,
    OpenProject,
    Save,
    SaveAs,
    NewCell,
    OpenDocument,
    ImportNetlist,
    ImportVerilogA,
    ExportSchematicSvg,
    ExportWaveformsCsv,
    ExportNetlist(crate::io::NetlistFormat),
    Exit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Delete,
    SelectAll,
    ObjectProperties,
    FindInDesign,
    Preferences,
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomOneToOne,
    CycleGrid,
    ToggleNavigator,
    ToggleInspector,
    ToggleConsole,
    ToggleFocusMode,
    ResetLayout,
    PreviousWorkspace,
    NextWorkspace,
    PlaceInstance,
    PlaceWire,
    PlaceLabel,
    PlaceProbe,
    Place(ComponentType),
    RotateSelection,
    MirrorSelectionHorizontal,
    AscendHierarchy,
    DescendHierarchy,
    RunChecks,
    ClearChecks,
    RunSimulation,
    StopSimulation,
    SimulationOptions,
    GenerateNetlist,
    ClearResults,
    WaveformCalculator,
    ResultViewer(crate::shell::ResultViewer),
    EditSpecifications,
    VerificationPage(VerificationPage),
    ProjectPage(ProjectPage),
    ModelsPage(ModelsPage),
    ModelBrowser,
    PdkSettings,
    CompileVerilogA,
    AutomationConsole,
    CommandPalette,
    KeyboardShortcuts,
    Documentation(&'static str, &'static str),
    License,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: &'static str,
    pub group: &'static str,
}

impl Command {
    pub const fn spec(self) -> CommandSpec {
        match self {
            Self::OpenWorkspace(Workspace::Project) => spec(
                "workspace.project",
                "Open Project workspace",
                "Alt+1",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Design) => spec(
                "workspace.design",
                "Open Design workspace",
                "Alt+2",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Simulate) => spec(
                "workspace.simulate",
                "Open Simulation workspace",
                "Alt+3",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Results) => spec(
                "workspace.results",
                "Open Results workspace",
                "Alt+4",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Verify) => spec(
                "workspace.verify",
                "Open Verification workspace",
                "Alt+5",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Models) => spec(
                "workspace.models",
                "Open Models workspace",
                "Alt+6",
                "Navigate",
            ),
            Self::OpenWorkspace(Workspace::Netlist) => spec(
                "workspace.netlist",
                "Open Netlist workspace",
                "Alt+7",
                "Navigate",
            ),
            Self::ProjectLauncher => spec(
                "project.launcher",
                "Project launcher…",
                "Ctrl+Shift+O",
                "File",
            ),
            Self::NewProject => spec("project.new", "New project…", "Ctrl+Shift+N", "File"),
            Self::OpenProject => spec("project.open", "Open project…", "Ctrl+O", "File"),
            Self::Save => spec("file.save", "Save", "Ctrl+S", "File"),
            Self::SaveAs => spec("file.save_as", "Save as…", "Ctrl+Shift+S", "File"),
            Self::NewCell => spec("design.new_cell", "New cell…", "Ctrl+N", "File"),
            Self::OpenDocument => spec("design.open", "Open schematic…", "", "File"),
            Self::ImportNetlist => spec("netlist.import", "Import SPICE deck…", "", "File"),
            Self::ImportVerilogA => spec("models.import_veriloga", "Import Verilog-A…", "", "File"),
            Self::ExportSchematicSvg => {
                spec("design.export_svg", "Export schematic SVG…", "", "File")
            }
            Self::ExportWaveformsCsv => {
                spec("results.export_csv", "Export waveform data…", "", "File")
            }
            Self::ExportNetlist(_) => spec("netlist.export", "Export netlist", "", "File"),
            Self::Exit => spec("application.exit", "Exit RSpice…", "Alt+F4", "File"),
            Self::Undo => spec("edit.undo", "Undo", "Ctrl+Z", "Edit"),
            Self::Redo => spec("edit.redo", "Redo", "Ctrl+Shift+Z", "Edit"),
            Self::Cut => spec("edit.cut", "Cut selection", "Ctrl+X", "Edit"),
            Self::Copy => spec("edit.copy", "Copy selection", "Ctrl+C", "Edit"),
            Self::Paste => spec("edit.paste", "Paste", "Ctrl+V", "Edit"),
            Self::Duplicate => spec("edit.duplicate", "Duplicate selection", "Ctrl+D", "Edit"),
            Self::Delete => spec("edit.delete", "Delete selection", "Delete", "Edit"),
            Self::SelectAll => spec("edit.select_all", "Select all", "Ctrl+A", "Edit"),
            Self::ObjectProperties => spec("edit.properties", "Object properties…", "Q", "Edit"),
            Self::FindInDesign => spec("design.find", "Find in design…", "Ctrl+F", "Edit"),
            Self::Preferences => spec("application.preferences", "Preferences…", "Ctrl+,", "Edit"),
            Self::ZoomIn => spec("view.zoom_in", "Zoom in", "+", "View"),
            Self::ZoomOut => spec("view.zoom_out", "Zoom out", "−", "View"),
            Self::ZoomFit => spec("view.zoom_fit", "Zoom active canvas to fit", "F", "View"),
            Self::ZoomOneToOne => spec("view.zoom_100", "Zoom 100%", "Ctrl+0", "View"),
            Self::CycleGrid => spec("view.grid", "Cycle grid and snap", "G", "View"),
            Self::ToggleNavigator => {
                spec("window.navigator", "Toggle navigator", "Ctrl+B", "Window")
            }
            Self::ToggleInspector => {
                spec("window.inspector", "Toggle inspector", "Ctrl+I", "Window")
            }
            Self::ToggleConsole => spec("window.console", "Toggle console", "Ctrl+J", "Window"),
            Self::ToggleFocusMode => {
                spec("window.focus", "Focus workspace", "Ctrl+Shift+F", "Window")
            }
            Self::ResetLayout => spec("window.reset", "Reset workspace layout…", "", "Window"),
            Self::PreviousWorkspace => spec(
                "window.previous_workspace",
                "Previous workspace",
                "Ctrl+Shift+Tab",
                "Window",
            ),
            Self::NextWorkspace => spec(
                "window.next_workspace",
                "Next workspace",
                "Ctrl+Tab",
                "Window",
            ),
            Self::PlaceInstance => spec(
                "design.place_instance",
                "Place instance…",
                "Shift+I",
                "Design",
            ),
            Self::PlaceWire => spec("design.place_wire", "Draw wire", "W", "Design"),
            Self::PlaceLabel => spec("design.place_label", "Place net label", "N", "Design"),
            Self::PlaceProbe => spec("design.place_probe", "Place probe", "P", "Design"),
            Self::Place(_) => spec("design.place_component", "Place component", "", "Design"),
            Self::RotateSelection => spec("design.rotate", "Rotate clockwise", "R", "Design"),
            Self::MirrorSelectionHorizontal => {
                spec("design.mirror", "Mirror horizontally", "M", "Design")
            }
            Self::AscendHierarchy => spec("design.ascend", "Ascend hierarchy", "Shift+H", "Design"),
            Self::DescendHierarchy => spec(
                "design.descend",
                "Descend into selected instance",
                "H",
                "Design",
            ),
            Self::RunChecks => spec("design.check", "Run schematic checks", "Ctrl+E", "Design"),
            Self::ClearChecks => spec("design.clear_checks", "Clear check results", "", "Design"),
            Self::RunSimulation => spec("simulation.run", "Run active plan", "F5", "Simulate"),
            Self::StopSimulation => {
                spec("simulation.stop", "Stop active run", "Shift+F5", "Simulate")
            }
            Self::SimulationOptions => spec(
                "simulation.options",
                "Global solver and convergence…",
                "",
                "Simulate",
            ),
            Self::GenerateNetlist => spec("netlist.generate", "Generate netlist", "", "Simulate"),
            Self::ClearResults => spec("results.clear", "Clear result history", "", "Results"),
            Self::WaveformCalculator => {
                spec("results.calculator", "Waveform calculator…", "", "Results")
            }
            Self::ResultViewer(_) => spec("results.viewer", "Open result viewer", "", "Results"),
            Self::EditSpecifications => spec(
                "verify.specifications",
                "Edit specification matrix",
                "",
                "Verify",
            ),
            Self::VerificationPage(_) => {
                spec("verify.page", "Open verification page", "", "Verify")
            }
            Self::ProjectPage(_) => spec("project.page", "Open project page", "", "Project"),
            Self::ModelsPage(_) => spec("models.page", "Open models page", "", "Models"),
            Self::ModelBrowser => spec("models.browser", "Model browser…", "", "Models"),
            Self::PdkSettings => spec("models.pdk", "PDK and model paths…", "", "Models"),
            Self::CompileVerilogA => spec("models.veriloga", "Compile Verilog-A…", "", "Models"),
            Self::AutomationConsole => spec(
                "automation.console",
                "Automation console",
                "Ctrl+`",
                "Automation",
            ),
            Self::CommandPalette => spec(
                "application.command_palette",
                "Search and run a command",
                "Ctrl+K",
                "Navigate",
            ),
            Self::KeyboardShortcuts => spec("help.shortcuts", "Keyboard shortcuts", "F1", "Help"),
            Self::Documentation(_, _) => spec("help.documentation", "Documentation", "", "Help"),
            Self::License => spec("help.license", "License and activation…", "", "Help"),
            Self::About => spec("help.about", "About RSpice", "", "Help"),
        }
    }

    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        match self {
            Self::Undo => state.schematic.can_undo(),
            Self::Redo => state.schematic.can_redo(),
            Self::Cut
            | Self::Copy
            | Self::Duplicate
            | Self::Delete
            | Self::RotateSelection
            | Self::MirrorSelectionHorizontal => {
                !state.schematic.read_only && !state.schematic.selection.is_empty()
            }
            Self::ObjectProperties => state.schematic.selection.single_component().is_some(),
            Self::AscendHierarchy => state.workspace.hierarchy_stack.len() > 1,
            Self::DescendHierarchy => state.schematic.selection.single_component().is_some(),
            Self::ClearChecks => state.dialogs.drc_results.is_some(),
            Self::RunSimulation => state.can_run_simulation(),
            Self::StopSimulation => state.simulation.is_running,
            Self::ClearResults => state.simulation.has_results(),
            Self::ExportWaveformsCsv => state.simulation.has_results(),
            _ => true,
        }
    }

    pub fn execute(self, app: &mut RSpiceApp) {
        match self {
            Self::OpenWorkspace(workspace) => activate_workspace(app, workspace),
            Self::ProjectLauncher => activate_workspace(app, Workspace::Project),
            Self::NewProject => file_action(app, FileMenuAction::NewProject),
            Self::OpenProject => file_action(app, FileMenuAction::OpenProject),
            Self::Save => file_action(app, FileMenuAction::Save),
            Self::SaveAs => file_action(app, FileMenuAction::SaveAs),
            Self::NewCell => file_action(app, FileMenuAction::New),
            Self::OpenDocument => file_action(app, FileMenuAction::Open),
            Self::ImportNetlist => {
                file_action(app, FileMenuAction::ImportNetlist);
                activate_workspace(app, Workspace::Netlist);
            }
            Self::ImportVerilogA => file_action(app, FileMenuAction::ImportVerilogA),
            Self::ExportSchematicSvg => file_action(app, FileMenuAction::ExportSvg),
            Self::ExportWaveformsCsv => file_action(app, FileMenuAction::ExportCsvWaveforms),
            Self::ExportNetlist(format) => crate::common::menu_bar::action_export_netlist_with_io(
                &mut app.state,
                format,
                app.export_workflow_io.as_ref(),
            ),
            Self::Exit => file_action(app, FileMenuAction::Exit),
            Self::Undo => {
                app.state.schematic.undo();
            }
            Self::Redo => {
                app.state.schematic.redo();
            }
            Self::Cut => {
                app.state.schematic.copy_selection();
                app.state.schematic.delete_selection();
            }
            Self::Copy => {
                app.state.schematic.copy_selection();
            }
            Self::Paste => {
                let anchor = app.state.schematic_paste_anchor();
                app.state.schematic.paste_at(anchor);
            }
            Self::Duplicate => {
                app.state.schematic.copy_selection();
                let anchor = app.state.schematic_paste_anchor() + crate::state::Point::new(2, 2);
                app.state.schematic.paste_at(anchor);
            }
            Self::Delete => {
                app.state.schematic.delete_selection();
            }
            Self::SelectAll => {
                let component_ids: Vec<u64> = app
                    .state
                    .schematic
                    .components
                    .iter()
                    .map(|component| component.id)
                    .collect();
                let wire_ids: Vec<u64> = app
                    .state
                    .schematic
                    .wires
                    .iter()
                    .map(|wire| wire.id)
                    .collect();
                app.state.schematic.selection.clear();
                for id in component_ids {
                    app.state.schematic.selection.select_component(id);
                }
                for id in wire_ids {
                    app.state.schematic.selection.select_wire(id);
                }
            }
            Self::ObjectProperties => {
                if let Some(id) = app.state.schematic.selection.single_component() {
                    crate::common::app::open_property_editor(&mut app.state, id);
                }
            }
            Self::FindInDesign => {
                activate_workspace(app, Workspace::Design);
                app.state.workbench.navigator_visible = true;
                app.state.shell.focus_nav_search = true;
            }
            Self::Preferences => app.state.dialogs.preferences_open = true,
            Self::ZoomIn => app.state.schematic.zoom = (app.state.schematic.zoom * 1.25).min(8.0),
            Self::ZoomOut => app.state.schematic.zoom = (app.state.schematic.zoom / 1.25).max(0.1),
            Self::ZoomFit => app.state.schematic.needs_fit = true,
            Self::ZoomOneToOne => app.state.schematic.zoom = 1.0,
            Self::CycleGrid => app.state.shell.grid = app.state.shell.grid.cycled(),
            Self::ToggleNavigator => {
                app.state.workbench.navigator_visible = !app.state.workbench.navigator_visible
            }
            Self::ToggleInspector => {
                app.state.workbench.inspector_visible = !app.state.workbench.inspector_visible
            }
            Self::ToggleConsole => {
                app.state.workbench.console_visible = !app.state.workbench.console_visible
            }
            Self::ToggleFocusMode => {
                app.state.workbench.focus_mode = !app.state.workbench.focus_mode
            }
            Self::ResetLayout => app.state.workbench.reset_layout(),
            Self::PreviousWorkspace => app.state.workbench.cycle_workspace(true),
            Self::NextWorkspace => app.state.workbench.cycle_workspace(false),
            Self::PlaceInstance => {
                activate_workspace(app, Workspace::Design);
                app.state.shell.focus_cell_search = true;
            }
            Self::PlaceWire => set_tool(app, Tool::Wire),
            Self::PlaceLabel => set_tool(app, Tool::Label),
            Self::PlaceProbe => set_tool(app, Tool::Probe),
            Self::Place(kind) => set_tool(app, Tool::Place(kind)),
            Self::RotateSelection => {
                app.state.schematic.rotate_selection();
            }
            Self::MirrorSelectionHorizontal => {
                app.state.schematic.mirror_selection_h();
            }
            Self::AscendHierarchy => {
                app.state.ascend_workspace_level();
            }
            Self::DescendHierarchy => {
                app.state.open_selected_instance_master();
            }
            Self::RunChecks => crate::common::menu_bar::run_design_rule_check(&mut app.state),
            Self::ClearChecks => app.state.dialogs.drc_results = None,
            Self::RunSimulation => {
                app.state.request_run_set_simulation();
                activate_workspace(app, Workspace::Simulate);
            }
            Self::StopSimulation => app.state.simulation.trigger_abort = true,
            Self::SimulationOptions => {
                crate::common::menu_bar::open_simulation_options(&mut app.state)
            }
            Self::GenerateNetlist => {
                crate::common::menu_bar::action_view_netlist(&mut app.state);
                activate_workspace(app, Workspace::Netlist);
            }
            Self::ClearResults => app.state.clear_simulation_results(),
            Self::WaveformCalculator => app.state.dialogs.waveform_calculator_dialog = true,
            Self::ResultViewer(viewer) => {
                app.state.shell.results.viewer = viewer;
                activate_workspace(app, Workspace::Results);
            }
            Self::EditSpecifications => {
                app.state.shell.results.viewer = crate::shell::ResultViewer::Specs;
                activate_workspace(app, Workspace::Verify);
                app.state.workbench.verification_page = VerificationPage::Specifications;
            }
            Self::VerificationPage(page) => {
                activate_workspace(app, Workspace::Verify);
                app.state.workbench.verification_page = page;
            }
            Self::ProjectPage(page) => {
                activate_workspace(app, Workspace::Project);
                app.state.workbench.project_page = page;
            }
            Self::ModelsPage(page) => {
                activate_workspace(app, Workspace::Models);
                app.state.workbench.models_page = page;
            }
            Self::ModelBrowser => app.state.model_browser_state.open = true,
            Self::PdkSettings => app
                .state
                .pdk_settings_dialog
                .open(app.state.pdk_config.clone()),
            Self::CompileVerilogA => app.state.dialogs.veriloga_dialog.open(),
            Self::AutomationConsole => {
                activate_workspace(app, Workspace::Netlist);
                app.state.workbench.console_visible = true;
                app.state.workbench.console_page = super::state::ConsolePage::Console;
            }
            Self::CommandPalette => app.state.dialogs.command_palette.open(),
            Self::KeyboardShortcuts => app.state.dialogs.shortcuts_help = true,
            Self::Documentation(title, path) => {
                crate::common::menu_bar::open_documentation_reference(&mut app.state, title, path)
            }
            Self::License => app.open_license_dialog(),
            Self::About => app.state.dialogs.about = true,
        }
    }
}

const fn spec(
    id: &'static str,
    label: &'static str,
    shortcut: &'static str,
    group: &'static str,
) -> CommandSpec {
    CommandSpec {
        id,
        label,
        shortcut,
        group,
    }
}

fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
    if let Some(view) = workspace.legacy_view() {
        app.state.shell.view = view;
    }
}

fn set_tool(app: &mut RSpiceApp, tool: Tool) {
    activate_workspace(app, Workspace::Design);
    app.state.schematic.tool = tool;
}

fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(
        &mut app.state,
        action,
        app.file_workflow_io.as_ref(),
        app.export_workflow_io.as_ref(),
    );
}

/// Commands shown by the new command search. Every entry has an implemented
/// dispatcher above.
pub const COMMAND_CATALOG: &[Command] = &[
    Command::ProjectLauncher,
    Command::OpenProject,
    Command::NewProject,
    Command::Save,
    Command::OpenWorkspace(Workspace::Project),
    Command::OpenWorkspace(Workspace::Design),
    Command::OpenWorkspace(Workspace::Simulate),
    Command::OpenWorkspace(Workspace::Results),
    Command::OpenWorkspace(Workspace::Verify),
    Command::OpenWorkspace(Workspace::Models),
    Command::OpenWorkspace(Workspace::Netlist),
    Command::PlaceInstance,
    Command::PlaceWire,
    Command::PlaceLabel,
    Command::RunChecks,
    Command::RunSimulation,
    Command::StopSimulation,
    Command::SimulationOptions,
    Command::GenerateNetlist,
    Command::WaveformCalculator,
    Command::EditSpecifications,
    Command::ModelBrowser,
    Command::PdkSettings,
    Command::CompileVerilogA,
    Command::AutomationConsole,
    Command::Preferences,
    Command::KeyboardShortcuts,
    Command::About,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_catalog_has_unique_stable_ids() {
        let mut ids = std::collections::HashSet::new();
        for command in COMMAND_CATALOG {
            assert!(
                ids.insert(command.spec().id),
                "duplicate command id {}",
                command.spec().id
            );
        }
    }

    #[test]
    fn all_workspace_commands_are_discoverable() {
        for workspace in Workspace::ALL {
            assert!(COMMAND_CATALOG.contains(&Command::OpenWorkspace(workspace)));
        }
    }
}
