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
use crate::workbench::design_system::{WorkbenchIcon, icon_button, labeled_icon_button};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{
    Cx, Reads, ToolbarControl, ToolbarEntry, ToolbarIcon, ToolbarId, ToolbarTool,
};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::lifecycle::session::SymbolTool;

/// The schematic's toolbar.
pub const SCHEMATIC_TOOLBAR: ToolbarId = ToolbarId::new("design.schematic");
/// The toolbar of an open symbol cellview.
pub const SYMBOL_TOOLBAR: ToolbarId = ToolbarId::new("design.symbol");
/// The Stimulus Library's toolbar.
pub const STIMULUS_TOOLBAR: ToolbarId = ToolbarId::new("design.stimulus");

/// Every toolbar the design feature registers.
pub(crate) const TOOLBARS: &[(ToolbarId, &[ToolbarEntry<DesignFeature>])] = &[
    (SCHEMATIC_TOOLBAR, SCHEMATIC),
    (SYMBOL_TOOLBAR, SYMBOL),
    (STIMULUS_TOOLBAR, STIMULUS),
];

/// The schematic's direct tools, each under the name its command declares,
/// without the ellipsis a menu row gives a command that asks for more. The
/// Select tool also names Escape, which returns to it from any tool but is
/// Cancel's key.
pub const DESIGN_DIRECT_TOOLBAR_COMMANDS: [(CommandKey, WorkbenchIcon, &str); 11] = [
    (
        DesignCommand::SelectTool.key(),
        WorkbenchIcon::Select,
        "Select tool (Esc)",
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
        "Place net label",
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
        "Create pins",
    ),
    (
        DesignCommand::PlaceProbe.key(),
        WorkbenchIcon::Probe,
        "Probe",
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
        WorkbenchIcon::SymbolPin,
        SymbolTool::PlacePin,
    ),
    (
        DesignCommand::SymbolPolylineTool.key(),
        WorkbenchIcon::Line,
        SymbolTool::Line,
    ),
    (
        DesignCommand::SymbolRectangleTool.key(),
        WorkbenchIcon::Rectangle,
        SymbolTool::Rectangle,
    ),
    (
        DesignCommand::SymbolCircleTool.key(),
        WorkbenchIcon::Circle,
        SymbolTool::Circle,
    ),
    (
        DesignCommand::SymbolArcTool.key(),
        WorkbenchIcon::Arc,
        SymbolTool::Arc,
    ),
    (
        DesignCommand::SymbolPolygonTool.key(),
        WorkbenchIcon::Polygon,
        SymbolTool::Polygon,
    ),
    (
        DesignCommand::SymbolTextTool.key(),
        WorkbenchIcon::Text,
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
    // A narrow row sets the flyout aside as its first shape.
    ToolbarEntry::Control(ToolbarControl {
        show: shape_flyout,
        overflow: Some(ToolbarIcon {
            key: DesignCommand::PlaceRectangle.key(),
            icon: WorkbenchIcon::Rectangle,
            label: None,
            pressed: Some(shape_armed),
        }),
    }),
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
    named(ids::DUPLICATE, WorkbenchIcon::Copy, "Duplicate"),
    icon(ids::DELETE, WorkbenchIcon::Trash),
    ToolbarEntry::Separator,
    icon(ids::UNDO, WorkbenchIcon::Undo),
    icon(ids::REDO, WorkbenchIcon::Redo),
    ToolbarEntry::Separator,
    icon(
        DesignCommand::DescendHierarchyDirect.key(),
        WorkbenchIcon::Layers,
    ),
    icon(DesignCommand::AscendHierarchy.key(), WorkbenchIcon::ArrowUp),
    ToolbarEntry::Separator,
    icon(ids::ZOOM_OUT, WorkbenchIcon::ZoomOut),
    icon(ids::ZOOM_IN, WorkbenchIcon::ZoomIn),
    named(ids::ZOOM_FIT, WorkbenchIcon::ZoomFit, "Fit drawing sheet"),
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

pub fn design_toolbar_command_selected(tool: Tool, key: CommandKey) -> bool {
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
        Some(
            DesignCommand::PlaceRectangle
            | DesignCommand::PlaceLine
            | DesignCommand::PlacePolygon
            | DesignCommand::PlaceArc
            | DesignCommand::PlaceCallout,
        ) => tool == Tool::DocumentationShape,
        _ => false,
    }
}

/// The shapes the Shape flyout lists, in the order Place, Shape lists them.
const SHAPES: [(DesignCommand, WorkbenchIcon); 5] = [
    (DesignCommand::PlaceRectangle, WorkbenchIcon::Rectangle),
    (DesignCommand::PlaceLine, WorkbenchIcon::Line),
    (DesignCommand::PlacePolygon, WorkbenchIcon::Polygon),
    (DesignCommand::PlaceArc, WorkbenchIcon::Arc),
    (DesignCommand::PlaceCallout, WorkbenchIcon::Callout),
];

/// What the flyout's button is called, on hover and to a screen reader.
const SHAPE_FLYOUT: &str = "Draw shape";

fn shape_armed(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.model.schematic.session.editor.tool == Tool::DocumentationShape
}

/// The Shape flyout: one button, wearing the armed shape or else the
/// rectangle, that opens the five shapes; a pick arms its shape at once
/// through its Place, Shape command, as KiCad's and Altium's drawing tools
/// arm.
fn shape_flyout(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    let editor = &cx.model.schematic.session.editor;
    let armed = (editor.tool == Tool::DocumentationShape)
        .then(|| {
            editor
                .pending_documentation_shape
                .as_ref()
                .map(|pending| pending.kind)
        })
        .flatten();
    let shown = SHAPES
        .into_iter()
        .find(|(command, _)| armed.is_some() && command.documentation_shape() == armed)
        .map_or(WorkbenchIcon::Rectangle, |(_, icon)| icon);
    let editable = !cx.model.schematic_edit_read_only();
    let popup_id = ui.id().with("shape-flyout");
    let open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    let size = toolbar_icon_button_size(layout);
    let response = ui
        .add_enabled_ui(editable, |ui| {
            icon_button(ui, shown, SHAPE_FLYOUT, armed.is_some() || open, size)
        })
        .inner;
    // A notch in the corner says the button opens a list.
    let t = Tokens::get(ui.ctx());
    let corner = response.rect.right_bottom() + Vec2::new(-4.0, -6.0);
    ui.painter().add(egui::Shape::convex_polygon(
        vec![
            corner,
            corner + Vec2::new(-4.0, 0.0),
            corner + Vec2::new(0.0, -4.0),
        ],
        if editable {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
        egui::Stroke::NONE,
    ));
    let mut picked = None;
    egui::Popup::menu(&response).id(popup_id).show(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        for (command, icon) in SHAPES {
            let kind = command
                .documentation_shape()
                .expect("every flyout row names its shape");
            let selected = armed == Some(kind);
            if labeled_icon_button(ui, icon, kind.label(), selected, 160.0).clicked() {
                picked = Some(command);
                ui.close();
            }
        }
    });
    if let Some(command) = picked {
        cx.execute(command.key());
        crate::workbench::app_state::return_engineering_canvas_focus(ui.ctx());
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
    // The turns and flips the Arrange menu and Ctrl+R and Ctrl+M apply.
    icon(DesignCommand::RotateSelection.key(), WorkbenchIcon::Rotate),
    icon(
        DesignCommand::MirrorSelectionHorizontal.key(),
        WorkbenchIcon::Mirror,
    ),
    // Update pins, named for the drawing the pins come from.
    icon(
        DesignCommand::SymbolUpdatePinsFromContract.key(),
        WorkbenchIcon::Pin,
    ),
    ToolbarEntry::Separator,
    icon(ids::UNDO, WorkbenchIcon::Undo),
    icon(ids::REDO, WorkbenchIcon::Redo),
    ToolbarEntry::Separator,
    icon(ids::ZOOM_OUT, WorkbenchIcon::ZoomOut),
    icon(ids::ZOOM_IN, WorkbenchIcon::ZoomIn),
    named(ids::ZOOM_FIT, WorkbenchIcon::ZoomFit, "Fit"),
    // Each control a row with no room for it sets aside behind More tools
    // as the command it stands for, under the name its tooltip gives it.
    ToolbarEntry::Control(ToolbarControl {
        show: show_grid_toggle,
        overflow: Some(ToolbarIcon {
            key: DesignCommand::CycleGrid.key(),
            icon: WorkbenchIcon::Grid,
            label: Some(SYMBOL_TOGGLES[0].1),
            pressed: Some(symbol_grid_shown),
        }),
    }),
    ToolbarEntry::Control(ToolbarControl {
        show: snap_to_grid_toggle,
        overflow: Some(ToolbarIcon {
            key: DesignCommand::SymbolSnapToGrid.key(),
            icon: WorkbenchIcon::Snap,
            label: Some(SYMBOL_TOGGLES[1].1),
            pressed: None,
        }),
    }),
    ToolbarEntry::Control(ToolbarControl {
        show: symbol_grid_spacing,
        overflow: Some(ToolbarIcon {
            key: DesignCommand::SymbolCycleGridSpacing.key(),
            icon: WorkbenchIcon::Sliders,
            label: Some(SYMBOL_GRID_SPACING),
            pressed: None,
        }),
    }),
    ToolbarEntry::Separator,
    // Edited in place, the symbol already stands placed on its sheet, so the
    // preview gives way to the way back.
    ToolbarEntry::Only(
        symbol_in_own_view,
        &[ToolbarEntry::Control(ToolbarControl {
            show: preview_as_placed_toggle,
            overflow: Some(ToolbarIcon {
                key: DesignCommand::SymbolPlacedPreview.key(),
                icon: WorkbenchIcon::Preview,
                label: Some(SYMBOL_TOGGLES[2].1),
                pressed: None,
            }),
        })],
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

fn symbol_in_own_view(reads: &Reads<'_, DesignFeature>) -> bool {
    !symbol_in_place(reads)
}

fn symbol_grid_shown(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.state.symbol.editor.show_grid
}

/// The grid spacing control's name, which its tooltip and its row in More
/// tools give it.
const SYMBOL_GRID_SPACING: &str = "Grid spacing";

/// The symbol row's toggles, each drawn as its icon under its name: show
/// grid, snap to grid, and preview as placed.
const SYMBOL_TOGGLES: [(WorkbenchIcon, &str); 3] = [
    (WorkbenchIcon::Grid, "Show grid"),
    (WorkbenchIcon::Snap, "Snap to grid"),
    (WorkbenchIcon::Preview, "Preview as placed on a sheet"),
];

fn show_grid_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(ui, &mut cx.state.symbol.editor.show_grid, 0, layout);
}

fn snap_to_grid_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(ui, &mut cx.state.symbol.editor.snap_to_grid, 1, layout);
}

fn preview_as_placed_toggle(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    symbol_toggle(ui, &mut cx.state.symbol.editor.preview_as_placed, 2, layout);
}

/// The symbol row's toggle `index` of [`SYMBOL_TOGGLES`], flipping `value`.
fn symbol_toggle(ui: &mut egui::Ui, value: &mut bool, index: usize, layout: LayoutSpec) {
    let (icon, label) = SYMBOL_TOGGLES[index];
    let response = icon_button(ui, icon, label, *value, toolbar_icon_button_size(layout));
    if response.clicked() {
        *value = !*value;
        crate::workbench::app_state::return_engineering_canvas_focus(ui.ctx());
    }
}

fn symbol_grid_spacing(cx: &mut Cx<'_, DesignFeature>, ui: &mut egui::Ui, _layout: LayoutSpec) {
    let editor = &mut cx.state.symbol.editor;
    let response = crate::ui::widgets::Combo::from_id_salt("symbol-editor.grid-spacing")
        .selected_text(editor.grid_spacing.label())
        .width(72.0)
        .show_ui(ui, |ui| {
            for spacing in crate::workbench::SymbolGridSpacing::ALL {
                if ui
                    .selectable_value(&mut editor.grid_spacing, spacing, spacing.label())
                    .clicked()
                {
                    crate::workbench::app_state::return_engineering_canvas_focus(ui.ctx());
                }
            }
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(SYMBOL_GRID_SPACING);
    });
    response.on_hover_text(SYMBOL_GRID_SPACING);
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
