//! The Design workspace's toolbars: the schematic's placement and transform
//! tools, the symbol cellview's authoring tools, and the Stimulus Library's
//! verbs. The design feature registers them; the shell chooses which shows
//! by its scope and the view in front of it.

use egui::Vec2;

use super::DesignFeature;
use super::commands::DesignCommand;
use crate::state::command_catalog::CommandKey;
use crate::state::{ComponentType, GridStyle, Tool};
use crate::ui::tokens::Tokens;
use crate::workbench::app::in_place_session;
use crate::workbench::chrome::toolbar::toolbar_icon_button_size;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
use crate::workbench::framework::feature::{
    Cx, Reads, ToolbarControl, ToolbarEntry, ToolbarIcon, ToolbarId, ToolbarTool,
};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::lifecycle::session::SymbolTool;

/// The schematic's toolbar.
pub(crate) const SCHEMATIC_TOOLBAR: ToolbarId = ToolbarId::new("design.schematic");
/// The toolbar of an open symbol cellview.
pub(crate) const SYMBOL_TOOLBAR: ToolbarId = ToolbarId::new("design.symbol");
/// The Stimulus Library's toolbar.
pub(crate) const STIMULUS_TOOLBAR: ToolbarId = ToolbarId::new("design.stimulus");

/// Every toolbar the design feature registers.
pub(crate) const TOOLBARS: &[(ToolbarId, &[ToolbarEntry<DesignFeature>])] = &[
    (SCHEMATIC_TOOLBAR, SCHEMATIC),
    (SYMBOL_TOOLBAR, SYMBOL),
    (STIMULUS_TOOLBAR, STIMULUS),
];

pub(crate) const DESIGN_DIRECT_TOOLBAR_COMMANDS: [(CommandKey, WorkbenchIcon, &str); 11] = [
    (
        DesignCommand::SelectTool.key(),
        WorkbenchIcon::Select,
        "Select (Esc)",
    ),
    (
        DesignCommand::PlaceInstance.key(),
        WorkbenchIcon::Instance,
        "Place instance",
    ),
    (
        DesignCommand::PlaceWire.key(),
        WorkbenchIcon::Wire,
        "Draw wire",
    ),
    (
        DesignCommand::PlaceBus.key(),
        WorkbenchIcon::Bus,
        "Draw bus",
    ),
    (
        DesignCommand::PlaceBusTap.key(),
        WorkbenchIcon::BusTap,
        "Place bus tap",
    ),
    (
        DesignCommand::PlaceJunction.key(),
        WorkbenchIcon::Junction,
        "Place junction",
    ),
    (
        DesignCommand::PlaceLabel.key(),
        WorkbenchIcon::Label,
        "Net label",
    ),
    // Not `Layers`: that glyph already sits in this same row for descending
    // into an instance, and a sheet crossing is a sideways continuation of a
    // net rather than a step down the hierarchy.
    (
        DesignCommand::PlaceOffSheetConnector.key(),
        WorkbenchIcon::ArrowRight,
        "Off-sheet connector",
    ),
    (
        DesignCommand::PlacePin.key(),
        WorkbenchIcon::Pin,
        "Create pin",
    ),
    (
        DesignCommand::PlaceProbe.key(),
        WorkbenchIcon::Probe,
        "Probe signal",
    ),
    (
        DesignCommand::PlaceText.key(),
        WorkbenchIcon::Text,
        "Place text or note",
    ),
];

/// Authoring tools of the symbol document, in the order the editor's
/// keyboard shortcuts declare them. A symbol cellview is a design document,
/// so its tools live in the shared workspace toolbar rather than in a bar
/// of the editor's own.
const SYMBOL_DIRECT_TOOLBAR_COMMANDS: [(CommandKey, WorkbenchIcon, SymbolTool); 8] = [
    (
        DesignCommand::SelectTool.key(),
        WorkbenchIcon::Select,
        SymbolTool::Select,
    ),
    (
        DesignCommand::SymbolPinTool.key(),
        WorkbenchIcon::Probe,
        SymbolTool::PlacePin,
    ),
    (
        DesignCommand::SymbolPolylineTool.key(),
        WorkbenchIcon::Wire,
        SymbolTool::Line,
    ),
    (
        DesignCommand::SymbolRectangleTool.key(),
        WorkbenchIcon::Grid,
        SymbolTool::Rectangle,
    ),
    (
        DesignCommand::SymbolCircleTool.key(),
        WorkbenchIcon::Target,
        SymbolTool::Circle,
    ),
    (
        DesignCommand::SymbolArcTool.key(),
        WorkbenchIcon::Refresh,
        SymbolTool::Arc,
    ),
    (
        DesignCommand::SymbolPolygonTool.key(),
        WorkbenchIcon::Layers,
        SymbolTool::Polygon,
    ),
    (
        DesignCommand::SymbolTextTool.key(),
        WorkbenchIcon::Label,
        SymbolTool::Text,
    ),
];

/// A command drawn as an icon, named by its own label and pressed as its
/// own check mark says.
const fn icon(key: CommandKey, icon: WorkbenchIcon) -> ToolbarEntry<DesignFeature> {
    ToolbarEntry::Icon(ToolbarIcon {
        key,
        icon,
        label: None,
        pressed: None,
    })
}

/// A command drawn as an icon under the toolbar's own name for it.
const fn named(
    key: CommandKey,
    icon: WorkbenchIcon,
    label: &'static str,
) -> ToolbarEntry<DesignFeature> {
    ToolbarEntry::Icon(ToolbarIcon {
        key,
        icon,
        label: Some(label),
        pressed: None,
    })
}

/// A command drawn as a labelled button.
const fn labelled(
    key: CommandKey,
    icon: WorkbenchIcon,
    label: &'static str,
) -> ToolbarEntry<DesignFeature> {
    ToolbarEntry::Tool(ToolbarTool { key, icon, label })
}

/// The schematic's direct tool `I`, pressed while it is the armed tool.
const fn direct<const I: usize>() -> ToolbarEntry<DesignFeature> {
    let (key, icon, label) = DESIGN_DIRECT_TOOLBAR_COMMANDS[I];
    ToolbarEntry::Icon(ToolbarIcon {
        key,
        icon,
        label: Some(label),
        pressed: Some(direct_tool_armed::<I>),
    })
}

fn direct_tool_armed<const I: usize>(reads: &Reads<'_, DesignFeature>) -> bool {
    design_toolbar_command_selected(
        reads.model.schematic.session.editor.tool,
        DESIGN_DIRECT_TOOLBAR_COMMANDS[I].0,
    )
}

/// The symbol document's authoring tool `I`, pressed while it is the
/// editor's tool.
const fn symbol_tool<const I: usize>() -> ToolbarEntry<DesignFeature> {
    let (key, icon, _) = SYMBOL_DIRECT_TOOLBAR_COMMANDS[I];
    ToolbarEntry::Icon(ToolbarIcon {
        key,
        icon,
        label: None,
        pressed: Some(symbol_tool_armed::<I>),
    })
}

fn symbol_tool_armed<const I: usize>(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.state.symbol.editor.tool == SYMBOL_DIRECT_TOOLBAR_COMMANDS[I].2
}

const SCHEMATIC: &[ToolbarEntry<DesignFeature>] = &[
    direct::<0>(),
    ToolbarEntry::Separator,
    direct::<1>(),
    direct::<2>(),
    direct::<3>(),
    direct::<4>(),
    direct::<5>(),
    direct::<6>(),
    direct::<7>(),
    direct::<8>(),
    direct::<9>(),
    direct::<10>(),
    ToolbarEntry::Separator,
    icon(DesignCommand::RotateSelection.key(), WorkbenchIcon::Rotate),
    icon(
        DesignCommand::MirrorSelectionHorizontal.key(),
        WorkbenchIcon::Mirror,
    ),
    icon(
        DesignCommand::MirrorSelectionVertical.key(),
        WorkbenchIcon::MirrorVertical,
    ),
    named(Command::Duplicate.key(), WorkbenchIcon::Copy, "Duplicate"),
    icon(Command::Delete.key(), WorkbenchIcon::Trash),
    ToolbarEntry::Separator,
    icon(Command::Undo.key(), WorkbenchIcon::Undo),
    icon(Command::Redo.key(), WorkbenchIcon::Redo),
    ToolbarEntry::Separator,
    icon(
        DesignCommand::DescendHierarchyDirect.key(),
        WorkbenchIcon::Layers,
    ),
    named(
        DesignCommand::AscendHierarchy.key(),
        WorkbenchIcon::ArrowUp,
        "Ascend to parent sheet · double-click the sheet background also ascends",
    ),
    ToolbarEntry::Separator,
    icon(Command::ZoomOut.key(), WorkbenchIcon::ZoomOut),
    icon(Command::ZoomIn.key(), WorkbenchIcon::ZoomIn),
    named(
        Command::ZoomFit.key(),
        WorkbenchIcon::ZoomFit,
        "Fit drawing sheet",
    ),
    ToolbarEntry::Control(ToolbarControl {
        show: grid_display,
        overflow: Some(ToolbarIcon {
            key: DesignCommand::CycleGrid.key(),
            icon: WorkbenchIcon::Grid,
            label: Some("Cycle grid display"),
            pressed: Some(grid_visible),
        }),
    }),
    named(
        DesignCommand::VisibilityOptions.key(),
        WorkbenchIcon::Visibility,
        "Display options",
    ),
    labelled(
        DesignCommand::CheckCellview.key(),
        WorkbenchIcon::Check,
        "Check cellview",
    ),
    labelled(
        DesignCommand::CheckDesign.key(),
        WorkbenchIcon::Check,
        "Check design",
    ),
];

pub(crate) fn design_toolbar_command_selected(tool: Tool, key: CommandKey) -> bool {
    match DesignCommand::from_key(key) {
        Some(DesignCommand::SelectTool) => tool == Tool::Select,
        Some(DesignCommand::PlaceInstance) => {
            matches!(tool, Tool::Place(component) if component != ComponentType::Port)
        }
        Some(DesignCommand::PlaceWire) => tool == Tool::Wire,
        Some(DesignCommand::PlaceBus) => tool == Tool::Bus,
        Some(DesignCommand::PlaceBusTap) => tool == Tool::BusTap,
        Some(DesignCommand::PlaceJunction) => tool == Tool::Junction,
        Some(DesignCommand::PlaceLabel) => tool == Tool::Label,
        Some(DesignCommand::PlaceOffSheetConnector) => tool == Tool::OffSheetConnector,
        Some(DesignCommand::PlaceProbe) => tool == Tool::Probe,
        Some(DesignCommand::PlacePin) => tool == Tool::Place(ComponentType::Port),
        Some(DesignCommand::PlaceText) => tool == Tool::DesignNote,
        _ => false,
    }
}

fn grid_visible(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.state.grid != GridStyle::Off
}

/// The grid display button. Cycling runs once the toolbar's lane has
/// drawn, so the frame that takes the click paints the style the cycle is
/// about to set.
fn grid_display(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    let grid_visible = cx.state.grid != GridStyle::Off;
    let height = layout.toolbar_control_height;
    let response = ui.add(egui::Button::selectable(grid_visible, "").min_size(Vec2::splat(height)));
    let painted = if response.clicked() {
        cx.execute(DesignCommand::CycleGrid.key());
        cx.state.next_grid_style()
    } else {
        cx.state.grid
    };
    let t = Tokens::get(ui.ctx());
    WorkbenchIcon::Grid.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(16.0)),
        if painted != GridStyle::Off {
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
const SYMBOL: &[ToolbarEntry<DesignFeature>] = &[
    symbol_tool::<0>(),
    symbol_tool::<1>(),
    symbol_tool::<2>(),
    symbol_tool::<3>(),
    symbol_tool::<4>(),
    symbol_tool::<5>(),
    symbol_tool::<6>(),
    symbol_tool::<7>(),
    ToolbarEntry::Separator,
    icon(DesignCommand::SymbolRotatePin.key(), WorkbenchIcon::Rotate),
    icon(DesignCommand::SymbolMirrorPin.key(), WorkbenchIcon::Mirror),
    // Reconciling the pin list against the interface contract belongs beside
    // the two gestures that edit one pin. `Refresh` already means the arc tool
    // in this row, so the pin mark carries it instead.
    icon(
        DesignCommand::SymbolUpdatePinsFromContract.key(),
        WorkbenchIcon::Pin,
    ),
    ToolbarEntry::Separator,
    icon(Command::Undo.key(), WorkbenchIcon::Undo),
    icon(Command::Redo.key(), WorkbenchIcon::Redo),
    ToolbarEntry::Separator,
    icon(Command::ZoomOut.key(), WorkbenchIcon::ZoomOut),
    icon(Command::ZoomIn.key(), WorkbenchIcon::ZoomIn),
    icon(Command::ZoomFit.key(), WorkbenchIcon::ZoomFit),
    ToolbarEntry::Control(ToolbarControl {
        show: show_grid_toggle,
        overflow: None,
    }),
    ToolbarEntry::Control(ToolbarControl {
        show: snap_to_grid_toggle,
        overflow: None,
    }),
    ToolbarEntry::Control(ToolbarControl {
        show: symbol_grid_spacing,
        overflow: None,
    }),
    ToolbarEntry::Separator,
    ToolbarEntry::Control(ToolbarControl {
        show: preview_as_placed_toggle,
        overflow: None,
    }),
    labelled(
        DesignCommand::SymbolSave.key(),
        WorkbenchIcon::Save,
        "Save symbol",
    ),
    ToolbarEntry::Only(
        symbol_in_place,
        &[labelled(
            DesignCommand::ExitSymbolInPlace.key(),
            WorkbenchIcon::ArrowUp,
            "Return to parent",
        )],
    ),
];

fn symbol_in_place(reads: &Reads<'_, DesignFeature>) -> bool {
    in_place_session(&reads.state.symbol.in_place, reads.model).is_some()
}

fn show_grid_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(
        ui,
        &mut cx.state.symbol.editor.show_grid,
        WorkbenchIcon::Grid,
        "Show grid",
        layout,
    );
}

fn snap_to_grid_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(
        ui,
        &mut cx.state.symbol.editor.snap_to_grid,
        WorkbenchIcon::Check,
        "Snap to grid",
        layout,
    );
}

fn preview_as_placed_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(
        ui,
        &mut cx.state.symbol.editor.preview_as_placed,
        WorkbenchIcon::Focus,
        "Preview as placed on a sheet",
        layout,
    );
}

fn symbol_toggle(
    ui: &mut egui::Ui,
    value: &mut bool,
    icon: WorkbenchIcon,
    label: &'static str,
    layout: LayoutSpec,
) {
    let response = icon_button(ui, icon, label, *value, toolbar_icon_button_size(layout));
    if response.clicked() {
        *value = !*value;
    }
}

fn symbol_grid_spacing(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, _layout: LayoutSpec) {
    let editor = &mut cx.state.symbol.editor;
    crate::ui::widgets::Combo::from_id_salt("symbol-editor.grid-spacing")
        .selected_text(editor.grid_spacing.label())
        .width(72.0)
        .show_ui(ui, |ui| {
            for spacing in crate::workbench::SymbolGridSpacing::ALL {
                ui.selectable_value(&mut editor.grid_spacing, spacing, spacing.label());
            }
        });
}

/// The Stimulus Library's own verbs.
///
/// New leads because an empty library is the state this workspace opens in
/// most often. The first two verbs make and use a definition, the next two
/// act on the one the browser is reading, and the separator before the last
/// two divides those from the verbs that act on the library and the design
/// around it.
const STIMULUS: &[ToolbarEntry<DesignFeature>] = &[
    labelled(
        DesignCommand::NewStimulusDefinition.key(),
        WorkbenchIcon::Add,
        "New definition",
    ),
    labelled(
        DesignCommand::PlaceSourceFromDefinition.key(),
        WorkbenchIcon::Source,
        "Place source from definition",
    ),
    ToolbarEntry::Separator,
    icon(
        DesignCommand::DuplicateStimulusDefinition.key(),
        WorkbenchIcon::Copy,
    ),
    icon(
        DesignCommand::DeleteStimulusDefinition.key(),
        WorkbenchIcon::Trash,
    ),
    ToolbarEntry::Separator,
    labelled(
        DesignCommand::ValidateStimulusLibrary.key(),
        WorkbenchIcon::Check,
        "Validate library",
    ),
    icon(
        DesignCommand::ShowAdopterOnSchematic.key(),
        WorkbenchIcon::Target,
    ),
];
