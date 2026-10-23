//! The Design workspace's toolbars: the schematic's placement and transform
//! tools, the symbol cellview's authoring tools, and the Stimulus Library's
//! verbs.

use egui::Vec2;

use crate::state::{ComponentType, Tool};
use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::lifecycle::session::SymbolTool;

use super::{
    ToolbarCommandPresentation, context_separator, overflow, take_projected_tool_slot,
    toolbar_icon_button_size, toolbar_icon_command, toolbar_icon_command_selected,
    toolbar_icon_command_selected_as, toolbar_text_command,
};

pub(super) const DESIGN_DIRECT_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, &str); 11] = [
    (Command::SelectTool, WorkbenchIcon::Select, "Select (Esc)"),
    (
        Command::PlaceInstance,
        WorkbenchIcon::Instance,
        "Place instance",
    ),
    (Command::PlaceWire, WorkbenchIcon::Wire, "Draw wire"),
    (Command::PlaceBus, WorkbenchIcon::Bus, "Draw bus"),
    (Command::PlaceBusTap, WorkbenchIcon::BusTap, "Place bus tap"),
    (
        Command::PlaceJunction,
        WorkbenchIcon::Junction,
        "Place junction",
    ),
    (Command::PlaceLabel, WorkbenchIcon::Label, "Net label"),
    // Not `Layers`: that glyph already sits in this same row for descending
    // into an instance, and a sheet crossing is a sideways continuation of a
    // net rather than a step down the hierarchy.
    (
        Command::PlaceOffSheetConnector,
        WorkbenchIcon::ArrowRight,
        "Off-sheet connector",
    ),
    (Command::PlacePin, WorkbenchIcon::Pin, "Create pin"),
    (Command::PlaceProbe, WorkbenchIcon::Probe, "Probe signal"),
    (
        Command::PlaceText,
        WorkbenchIcon::Text,
        "Place text or note",
    ),
];

/// Authoring tools of the symbol document, in the order the editor's
/// keyboard shortcuts declare them. A symbol cellview is a design document,
/// so its tools live in the shared workspace toolbar rather than in a bar
/// of the editor's own.
const SYMBOL_DIRECT_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, SymbolTool); 8] = [
    (
        Command::SelectTool,
        WorkbenchIcon::Select,
        SymbolTool::Select,
    ),
    (
        Command::SymbolPinTool,
        WorkbenchIcon::Probe,
        SymbolTool::PlacePin,
    ),
    (
        Command::SymbolPolylineTool,
        WorkbenchIcon::Wire,
        SymbolTool::Line,
    ),
    (
        Command::SymbolRectangleTool,
        WorkbenchIcon::Grid,
        SymbolTool::Rectangle,
    ),
    (
        Command::SymbolCircleTool,
        WorkbenchIcon::Target,
        SymbolTool::Circle,
    ),
    (
        Command::SymbolArcTool,
        WorkbenchIcon::Refresh,
        SymbolTool::Arc,
    ),
    (
        Command::SymbolPolygonTool,
        WorkbenchIcon::Layers,
        SymbolTool::Polygon,
    ),
    (
        Command::SymbolTextTool,
        WorkbenchIcon::Label,
        SymbolTool::Text,
    ),
];

const STIMULUS_TOOLBAR_COMMANDS: [(Command, WorkbenchIcon, Option<&str>); 6] = [
    (
        Command::StimulusNewDefinition,
        WorkbenchIcon::Add,
        Some("New definition"),
    ),
    (
        Command::StimulusPlaceDefinition,
        WorkbenchIcon::Source,
        Some("Place source from definition"),
    ),
    (
        Command::StimulusDuplicateDefinition,
        WorkbenchIcon::Copy,
        None,
    ),
    (
        Command::StimulusDeleteDefinition,
        WorkbenchIcon::Trash,
        None,
    ),
    (
        Command::StimulusValidateLibrary,
        WorkbenchIcon::Check,
        Some("Validate library"),
    ),
    (Command::StimulusShowAdopter, WorkbenchIcon::Target, None),
];

pub(super) fn design_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    for (index, (command, icon, label)) in DESIGN_DIRECT_TOOLBAR_COMMANDS.into_iter().enumerate() {
        if index == 1 {
            context_separator(ui, layout);
        }
        let selected =
            design_toolbar_command_selected(app.state.model.schematic.session.editor.tool, command);
        toolbar_icon_command_selected_as(ui, app, command, icon, selected, label, layout);
    }
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::RotateSelection,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::MirrorSelectionHorizontal,
        WorkbenchIcon::Mirror,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::MirrorSelectionVertical,
        WorkbenchIcon::MirrorVertical,
        layout,
    );
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::Duplicate,
        WorkbenchIcon::Copy,
        false,
        "Duplicate",
        layout,
    );
    toolbar_icon_command(ui, app, Command::Delete, WorkbenchIcon::Trash, layout);
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_icon_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::DescendHierarchyDirect,
        WorkbenchIcon::Layers,
        layout,
    );
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::AscendHierarchy,
        WorkbenchIcon::ArrowUp,
        false,
        "Ascend to parent sheet · double-click the sheet background also ascends",
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
    toolbar_icon_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::ZoomFit,
        WorkbenchIcon::ZoomFit,
        false,
        "Fit drawing sheet",
        layout,
    );
    design_grid_and_snap_menu(ui, app, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        Command::VisibilityOptions,
        WorkbenchIcon::Visibility,
        false,
        "Display options",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::CheckCellview,
        WorkbenchIcon::Check,
        "Check cellview",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::CheckDesign,
        WorkbenchIcon::Check,
        "Check design",
        layout,
    );
}

pub(super) fn design_toolbar_command_selected(tool: Tool, command: Command) -> bool {
    match command {
        Command::SelectTool => tool == Tool::Select,
        Command::PlaceInstance => {
            matches!(tool, Tool::Place(component) if component != ComponentType::Port)
        }
        Command::PlaceWire => tool == Tool::Wire,
        Command::PlaceBus => tool == Tool::Bus,
        Command::PlaceBusTap => tool == Tool::BusTap,
        Command::PlaceJunction => tool == Tool::Junction,
        Command::PlaceLabel => tool == Tool::Label,
        Command::PlaceOffSheetConnector => tool == Tool::OffSheetConnector,
        Command::PlacePin => tool == Tool::Place(ComponentType::Port),
        Command::PlaceProbe => tool == Tool::Probe,
        Command::PlaceText => tool == Tool::DesignNote,
        _ => false,
    }
}

fn design_grid_and_snap_menu(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let grid_visible = app.state.design.grid != crate::state::GridStyle::Off;
    if !take_projected_tool_slot(ui, layout) {
        let presentation = ToolbarCommandPresentation::Icon {
            icon: WorkbenchIcon::Grid,
            selected: grid_visible,
            label: Some("Cycle grid display"),
        };
        let tool = overflow::OverflowTool::command(Command::CycleGrid, presentation);
        overflow::set_aside(ui, tool);
        return;
    }
    let height = layout.toolbar_control_height;
    let response = ui.add(egui::Button::selectable(grid_visible, "").min_size(Vec2::splat(height)));
    if response.clicked() {
        Command::CycleGrid.execute(app);
    }
    let grid_visible = app.state.design.grid != crate::state::GridStyle::Off;
    let t = Tokens::get(ui.ctx());
    WorkbenchIcon::Grid.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(16.0)),
        if grid_visible {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Cycle grid display");
        node.set_description("Cycle the schematic grid between dots, lines, and off.");
    });
    response.on_hover_text("Cycle grid display (G)");
}

/// Toolbar of an open symbol cellview.
///
/// Symbol geometry has its own undo stack, so undo and redo route through
/// the symbol document rather than the schematic's.
pub(super) fn symbol_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let active = app.state.design.symbol.editor.tool;
    for (command, icon, tool) in SYMBOL_DIRECT_TOOLBAR_COMMANDS {
        toolbar_icon_command_selected(ui, app, command, icon, active == tool, layout);
    }
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::SymbolRotatePin,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::SymbolMirrorPin,
        WorkbenchIcon::Mirror,
        layout,
    );
    // Reconciling the pin list against the interface contract belongs beside
    // the two gestures that edit one pin. `Refresh` already means the arc tool
    // in this row, so the pin mark carries it instead.
    toolbar_icon_command(
        ui,
        app,
        Command::SymbolUpdatePinsFromContract,
        WorkbenchIcon::Pin,
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_icon_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
    toolbar_icon_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
    toolbar_icon_command(ui, app, Command::ZoomFit, WorkbenchIcon::ZoomFit, layout);
    symbol_toggle(
        ui,
        &mut app.state.design.symbol.editor.show_grid,
        WorkbenchIcon::Grid,
        "Show grid",
        layout,
    );
    symbol_toggle(
        ui,
        &mut app.state.design.symbol.editor.snap_to_grid,
        WorkbenchIcon::Check,
        "Snap to grid",
        layout,
    );
    symbol_grid_spacing(ui, app, layout);
    context_separator(ui, layout);
    symbol_toggle(
        ui,
        &mut app.state.design.symbol.editor.preview_as_placed,
        WorkbenchIcon::Focus,
        "Preview as placed on a sheet",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::SymbolSave,
        WorkbenchIcon::Save,
        "Save symbol",
        layout,
    );
    if app.state.symbol_in_place().is_some() {
        let (icon, label) = (WorkbenchIcon::ArrowUp, "Return to parent");
        toolbar_text_command(ui, app, Command::ExitSymbolInPlace, icon, label, layout);
    }
}

fn symbol_toggle(
    ui: &mut egui::Ui,
    value: &mut bool,
    icon: WorkbenchIcon,
    label: &'static str,
    layout: LayoutSpec,
) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let response = icon_button(ui, icon, label, *value, toolbar_icon_button_size(layout));
    if response.clicked() {
        *value = !*value;
    }
}

fn symbol_grid_spacing(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    egui::ComboBox::from_id_salt("symbol-editor.grid-spacing")
        .selected_text(app.state.design.symbol.editor.grid_spacing.label())
        .width(72.0)
        .show_ui(ui, |ui| {
            for spacing in crate::workbench::SymbolGridSpacing::ALL {
                ui.selectable_value(
                    &mut app.state.design.symbol.editor.grid_spacing,
                    spacing,
                    spacing.label(),
                );
            }
        });
}

/// The Stimulus Library's own verbs.
///
/// New leads because an empty library is the state this workspace opens in
/// most often; the separator divides the verbs that act on one definition from
/// the two that act on the library and the design around it.
pub(super) fn stimulus_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    for (index, (command, icon, label)) in STIMULUS_TOOLBAR_COMMANDS.into_iter().enumerate() {
        // After Place: the first two verbs make and use a definition, the rest
        // act on the one the browser is reading.
        if index == 2 || index == 4 {
            context_separator(ui, layout);
        }
        if let Some(label) = label {
            toolbar_text_command(ui, app, command, icon, label, layout);
        } else {
            toolbar_icon_command(ui, app, command, icon, layout);
        }
    }
}
