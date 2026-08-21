//! Mockup-owned selection context menu for the schematic canvas.
//!
//! Right-click, Shift+F10 and a touch long-press all open the same command
//! contract. Every visible command is backed by a real schematic or results
//! operation; unsupported targets stay visibly disabled with an explanation.

use std::collections::BTreeSet;

use egui::{
    Align2, Color32, Context, CornerRadius, Frame, Id, Key, Margin, Modifiers, Popup, Rect,
    RectAlign, Response, ScrollArea, Sense, Shadow, Stroke, StrokeKind, Ui, WidgetInfo, WidgetType,
    pos2, vec2,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{Point, Selection, Tool, ViewType};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus};
use crate::workbench::app_state::{AppState, ContextTarget};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::Workspace;
use crate::workbench::{
    ResultViewer,
    app::{open_replace_instance_dialog, replace_instance_available},
};

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::interaction::{PointerHit, PointerTarget, pointer_target};
use super::sheet_visibility::{
    object_is_on_active_sheet, retain_selection_on_active_sheet,
    selection_filtered_to_active_sheet, with_hidden_wire_topology_preserved,
};
use super::viewport::Viewport;

const DESKTOP_WIDTH: f32 = 286.0;
const DESKTOP_MAX_HEIGHT: f32 = 520.0;
const DESKTOP_VIEWPORT_INSET: f32 = 6.0;
/// The mockup's `.menu-item { min-height: 27px }`. The row was drawn three
/// pixels taller here, which cost the surface a row's worth of height for no
/// design reason and pushed the last entries below the ceiling.
const DESKTOP_ROW_HEIGHT: f32 = 27.0;
const DESKTOP_RADIUS: u8 = 3;
const TOUCH_MAX_WIDTH: f32 = 420.0;
const TOUCH_VIEWPORT_INSET: f32 = 8.0;
const TOUCH_MAX_HEIGHT: f32 = 560.0;
const TOUCH_VIEWPORT_FRACTION: f32 = 0.70;
const TOUCH_ROW_HEIGHT: f32 = 44.0;
const TOUCH_RADIUS: u8 = 7;
const HEADER_HEIGHT: f32 = 47.0;
const SEPARATOR_HEIGHT: f32 = 9.0;
const ICON_SIDE: f32 = 17.0;
const ROW_HORIZONTAL_PADDING: f32 = 7.0;
const ICON_LABEL_GAP: f32 = 7.0;
const SURFACE_BORDER_WIDTH: f32 = 2.0;
const KEYBOARD_ANCHOR_OFFSET: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextInvocation {
    Pointer,
    Keyboard,
    TouchSheet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextAction {
    Properties,
    Rotate,
    Mirror,
    Copy,
    Duplicate,
    Delete,
    DescendHierarchy,
    UpdateInstanceInterface,
    ReplaceInstance,
    CreateHierarchy,
    CreateSymbolFromPorts,
    PageSetup,
    FitContent,
    ShowInNetlist,
    Probe,
    OperatingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextIcon {
    Sliders,
    Rotate,
    Mirror,
    Copy,
    Trash,
    Hierarchy,
    Sheet,
    Fit,
    Code,
    Probe,
    Waveform,
}

#[derive(Debug, Clone, Copy)]
struct ContextCommand {
    action: ContextAction,
    icon: ContextIcon,
    label: &'static str,
    shortcut_command: Option<Command>,
}

#[derive(Debug, Clone, Copy)]
enum ContextEntry {
    Command(ContextCommand),
    Separator,
}

const CONTEXT_ENTRIES: &[ContextEntry] = &[
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Properties,
        icon: ContextIcon::Sliders,
        label: "Object properties…",
        shortcut_command: Some(Command::ObjectProperties),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Rotate,
        icon: ContextIcon::Rotate,
        label: "Rotate 90°",
        shortcut_command: Some(Command::RotateSelection),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Mirror,
        icon: ContextIcon::Mirror,
        label: "Mirror",
        shortcut_command: Some(Command::MirrorSelectionHorizontal),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Copy,
        icon: ContextIcon::Copy,
        label: "Copy selection",
        shortcut_command: Some(Command::Copy),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Duplicate,
        icon: ContextIcon::Copy,
        label: "Duplicate and place",
        shortcut_command: Some(Command::Duplicate),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Delete,
        icon: ContextIcon::Trash,
        label: "Delete selection…",
        shortcut_command: Some(Command::Delete),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::DescendHierarchy,
        icon: ContextIcon::Hierarchy,
        label: "Descend into selected instance",
        shortcut_command: Some(Command::DescendHierarchyDirect),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::UpdateInstanceInterface,
        icon: ContextIcon::Hierarchy,
        label: "Update instance interface",
        shortcut_command: Some(Command::UpdateInstanceInterface),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ReplaceInstance,
        icon: ContextIcon::Hierarchy,
        label: "Replace instance…",
        shortcut_command: Some(Command::ReplaceInstance),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateHierarchy,
        icon: ContextIcon::Hierarchy,
        label: "Create hierarchy from selection…",
        shortcut_command: Some(Command::CreateHierarchy),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateSymbolFromPorts,
        icon: ContextIcon::Hierarchy,
        label: "Create symbol from schematic ports…",
        shortcut_command: None,
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PageSetup,
        icon: ContextIcon::Sheet,
        label: "Page setup…",
        shortcut_command: Some(Command::PageSetup),
    }),
    // Fitting the drawing sheet is not here. `Command::ZoomFit` already has a
    // toolbar button on this canvas, a status-bar route, a mobile canvas
    // control and the `F` key; a selection menu that fits without scrolling is
    // worth more than a sixth route to it.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::FitContent,
        icon: ContextIcon::Fit,
        label: "Fit schematic content",
        shortcut_command: Some(Command::FitSchematicContent),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ShowInNetlist,
        icon: ContextIcon::Code,
        label: "Show in netlist",
        shortcut_command: Some(Command::ShowInNetlist),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Probe,
        icon: ContextIcon::Probe,
        label: "Add voltage or current probe…",
        shortcut_command: Some(Command::PlaceProbe),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OperatingPoint,
        icon: ContextIcon::Waveform,
        label: "Open operating point",
        shortcut_command: Some(Command::ResultViewer(ResultViewer::Op)),
    }),
];

#[derive(Debug, Clone)]
struct DeleteSelectionRequest {
    selection: Selection,
    topology_version: u64,
    expected_junctions: Vec<crate::state::Junction>,
    expected_design_notes: Vec<crate::state::DesignNote>,
    expected_documentation_shapes: Vec<crate::state::DocumentationShape>,
}

#[derive(Debug, Clone)]
struct DeleteReview {
    selection: String,
    affected_nets: String,
    dependent_records: String,
}

#[derive(Clone)]
struct ContextRow {
    response: Response,
    enabled: bool,
}

pub(super) fn handle_context_menu(
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    _routing_was_active: bool,
    symbol_context: &SchematicSymbolContext,
) {
    retain_selection_on_active_sheet(state);
    if show_delete_confirmation(&response.ctx, state, symbol_context) {
        return;
    }
    let ctx = &response.ctx;
    let popup_id = Popup::default_response_id(response);
    let invocation_id = popup_id.with("invocation");
    let surface_anchor_id = popup_id.with("surface-anchor");
    let was_open = Popup::is_id_open(ctx, popup_id);

    #[cfg(target_arch = "wasm32")]
    let browser_keyboard_open =
        crate::workbench::browser::accessibility::take_schematic_context_menu_request();
    #[cfg(not(target_arch = "wasm32"))]
    let browser_keyboard_open = false;
    let keyboard_open = (response.has_focus()
        && ctx.input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10)))
        || browser_keyboard_open;
    let opened_this_frame = response.secondary_clicked() || keyboard_open;

    if response.secondary_clicked() {
        let invocation = if response.long_touched() {
            ContextInvocation::TouchSheet
        } else {
            ContextInvocation::Pointer
        };
        if let Some(anchor) = capture_pointer_target(response, state, viewport, symbol_context) {
            ctx.data_mut(|data| {
                data.insert_temp(invocation_id, invocation);
                data.insert_temp(surface_anchor_id, anchor);
            });
        }
    } else if keyboard_open {
        let (target, click_pos) = keyboard_target(state, viewport, response.rect.center());
        state.dialogs.interaction.context_target = Some((target, (click_pos.x, click_pos.y)));
        ctx.data_mut(|data| {
            data.insert_temp(invocation_id, ContextInvocation::Keyboard);
            data.insert_temp(surface_anchor_id, keyboard_surface_anchor(response.rect));
        });
    }

    let invocation = ctx
        .data(|data| data.get_temp::<ContextInvocation>(invocation_id))
        .unwrap_or(ContextInvocation::Pointer);
    let geometry = SurfaceGeometry::resolve(ctx, invocation);
    let t = Tokens::get(ctx);
    let frame = Frame::new()
        .fill(t.color.bg_elevated)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .corner_radius(CornerRadius::same(geometry.radius))
        .shadow(context_shadow(&t));

    let mut popup = Popup::context_menu(response)
        .id(popup_id)
        .width(geometry.width)
        .frame(frame);
    if keyboard_open {
        popup = popup.open_memory(Some(egui::SetOpenCommand::Bool(true)));
    }
    match invocation {
        ContextInvocation::Pointer | ContextInvocation::Keyboard => {
            if let Some(requested) = ctx.data(|data| data.get_temp::<egui::Pos2>(surface_anchor_id))
            {
                let anchor = clamp_desktop_surface_origin(ctx.content_rect(), requested, geometry);
                popup = popup
                    .at_position(anchor)
                    .align(RectAlign::BOTTOM_START)
                    .align_alternatives(&[]);
            }
        }
        ContextInvocation::TouchSheet => {
            let screen = ctx.content_rect();
            let anchor = pos2(screen.center().x, screen.bottom() - TOUCH_VIEWPORT_INSET);
            popup = popup
                .at_position(anchor)
                .align(RectAlign::TOP)
                .align_alternatives(&[]);
        }
    }

    let restore_focus = was_open && ctx.input(|input| input.key_pressed(Key::Escape));
    let popup_response = popup.show(|ui| {
        let content_width = (geometry.width - 2.0).max(1.0);
        ui.set_min_width(content_width);
        ui.set_max_width(content_width);
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

        ScrollArea::vertical()
            .id_salt(popup_id.with("scroll"))
            // CSS max-height includes the one-pixel border on both sides;
            // egui's Frame adds those outside its content UI.
            .max_height((geometry.max_height - SURFACE_BORDER_WIDTH).max(1.0))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);
                render_context_contents(
                    ui,
                    state,
                    symbol_context,
                    geometry.row_height,
                    opened_this_frame,
                );
            });
    });
    if let Some(popup_response) = &popup_response {
        ctx.accesskit_node_builder(popup_response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Schematic selection");
        });
    }

    if restore_focus {
        response.request_focus();
    }
    if !Popup::is_id_open(ctx, popup_id) {
        state.dialogs.interaction.context_target = None;
        ctx.data_mut(|data| {
            data.remove::<ContextInvocation>(invocation_id);
            data.remove::<egui::Pos2>(surface_anchor_id);
        });
    }
}

fn capture_pointer_target(
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) -> Option<egui::Pos2> {
    let pos = response.interact_pointer_pos()?;
    let grid_pos = screen_to_grid(viewport, state.schematic.grid_size, pos);
    let hit_pos = screen_to_schematic(viewport, pos);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let target = select_pointer_target(
        state,
        PointerHit::new(grid_pos, hit_pos),
        hit_radius,
        symbol_context,
        &response.ctx,
        viewport,
        pos,
    );
    state.dialogs.interaction.context_target = Some((target, (grid_pos.x, grid_pos.y)));
    Some(pos)
}

fn select_pointer_target(
    state: &mut AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) -> ContextTarget {
    let Some(target) = pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    ) else {
        return ContextTarget::Canvas;
    };
    state.schematic.net_highlight.clear();
    match target {
        PointerTarget::Component(id) => {
            if !state.schematic.selection.has_component(id) {
                state.schematic.selection.select_only_component(id);
            }
            ContextTarget::Component(id)
        }
        PointerTarget::DesignNote(id) => {
            if !state.schematic.selection.has_design_note(id) {
                state.schematic.selection.select_only_design_note(id);
            }
            ContextTarget::Canvas
        }
        // A probe selects like any other annotation. `ContextTarget` has no
        // probe case, so the menu opens against the canvas.
        PointerTarget::Probe(id) => {
            if !state.schematic.selection.has_probe(id) {
                state.schematic.selection.select_only_probe(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::DocumentationShape(id) => {
            if !state.schematic.selection.has_documentation_shape(id) {
                state
                    .schematic
                    .selection
                    .select_only_documentation_shape(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::NetLabel(id) => {
            if !state.schematic.selection.has_net_label(id) {
                state.schematic.selection.select_only_net_label(id);
            }
            // ContextTarget has no net-label variant; stable Selection identity
            // remains authoritative for properties and lifecycle actions.
            ContextTarget::Canvas
        }
        PointerTarget::BusTap(id) => {
            if !state.schematic.selection.has_bus_tap(id) {
                state.schematic.selection.select_only_bus_tap(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Junction(position) => {
            if !state.schematic.selection.has_junction(position) {
                state.schematic.selection.select_only_junction(position);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Bus(id) => {
            if !state.schematic.selection.has_bus(id) {
                state.schematic.selection.select_only_bus(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Wire(id) => {
            if !state.schematic.selection.has_wire(id) {
                state.schematic.selection.select_only_wire(id);
            }
            ContextTarget::Wire(id)
        }
    }
}

fn keyboard_surface_anchor(trigger: Rect) -> egui::Pos2 {
    pos2(
        trigger.left() + KEYBOARD_ANCHOR_OFFSET.min(trigger.width() * 0.5),
        trigger.top() + KEYBOARD_ANCHOR_OFFSET.min(trigger.height() * 0.5),
    )
}

fn clamp_desktop_surface_origin(
    screen: Rect,
    requested: egui::Pos2,
    geometry: SurfaceGeometry,
) -> egui::Pos2 {
    let min_x = screen.left() + DESKTOP_VIEWPORT_INSET;
    let min_y = screen.top() + DESKTOP_VIEWPORT_INSET;
    let max_x = (screen.right() - geometry.width - DESKTOP_VIEWPORT_INSET).max(min_x);
    let max_y = (screen.bottom() - geometry.outer_height() - DESKTOP_VIEWPORT_INSET).max(min_y);
    pos2(
        requested.x.clamp(min_x, max_x),
        requested.y.clamp(min_y, max_y),
    )
}

fn point_midpoint(first: Point, last: Point) -> Point {
    Point::new(
        ((i64::from(first.x) + i64::from(last.x)) / 2) as i32,
        ((i64::from(first.y) + i64::from(last.y)) / 2) as i32,
    )
}

fn keyboard_target(
    state: &AppState,
    viewport: &Viewport,
    fallback_screen_pos: egui::Pos2,
) -> (ContextTarget, Point) {
    if let Some(id) = state.schematic.selection.single_bus_tap()
        && let Some(tap) = state.schematic.bus_taps.iter().find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, tap.connection_point);
    }
    if let Some(id) = state.schematic.selection.single_net_label()
        && let Some(label) = state.schematic.net_labels.iter().find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, label.pos);
    }
    if let Some(id) = state.schematic.selection.single_bus()
        && let Some(bus) = state.schematic.buses.iter().find(|item| item.id == id)
        && let (Some(first), Some(last)) = (bus.points.first(), bus.points.last())
    {
        return (ContextTarget::Canvas, point_midpoint(*first, *last));
    }
    if let Some(id) = state.schematic.selection.single_component()
        && let Some(component) = state.schematic.components.iter().find(|item| item.id == id)
    {
        return (ContextTarget::Component(id), component.pos);
    }
    if let Some(id) = state.schematic.selection.single_wire()
        && let Some(wire) = state.schematic.wires.iter().find(|item| item.id == id)
        && let (Some(first), Some(last)) = (wire.points.first(), wire.points.last())
    {
        return (ContextTarget::Wire(id), point_midpoint(*first, *last));
    }
    if let Some(point) = state.schematic.selection.single_junction() {
        return (ContextTarget::Canvas, point);
    }
    if let Some(id) = state.schematic.selection.components.iter().copied().min()
        && let Some(component) = state.schematic.components.iter().find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, component.pos);
    }
    if let Some((x, y)) = state.dialogs.interaction.last_click_pos {
        return (ContextTarget::Canvas, Point::new(x, y));
    }
    (
        ContextTarget::Canvas,
        screen_to_grid(viewport, state.schematic.grid_size, fallback_screen_pos),
    )
}

fn render_context_contents(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    row_height: f32,
    focus_first: bool,
) {
    let Some((target, (x, y))) = state.dialogs.interaction.context_target else {
        ui.close();
        return;
    };
    let click_pos = Point::new(x, y);
    let summary = selection_summary(state, target);
    menu_header(ui, &summary);

    let mut rows = Vec::with_capacity(14);
    let mut keyboard_or_pointer_action = None;
    for entry in CONTEXT_ENTRIES {
        match *entry {
            ContextEntry::Separator => menu_separator(ui),
            ContextEntry::Command(command) => {
                let (enabled, reason) = action_availability(command.action, state);
                let shortcut =
                    command
                        .shortcut_command
                        .map_or_else(String::new, |product_command| {
                            state.ui.preferences.shortcuts().resolved_label(
                                product_command,
                                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                                ui.ctx().os(),
                            )
                        });
                let response = ui
                    .push_id(("schematic-context-command", command.label), |ui| {
                        menu_item(ui, command, &shortcut, enabled, reason, row_height)
                    })
                    .inner;
                // Consume a focused Enter/Space before consulting the egui
                // click synthesis. If `clicked` is checked first, the
                // short-circuit can execute the command while leaving the
                // same key event available to the canvas behind the menu.
                let activated =
                    enabled && (menu_row_keyboard_activated(ui, &response) || response.clicked());
                rows.push(ContextRow { response, enabled });
                if activated && keyboard_or_pointer_action.is_none() {
                    keyboard_or_pointer_action = Some(command.action);
                }
            }
        }
    }
    manage_menu_focus(ui, &rows, focus_first);
    if let Some(action) = keyboard_or_pointer_action {
        execute_context_action(action, ui, state, click_pos, symbol_context);
    }
}

fn menu_row_keyboard_activated(ui: &Ui, response: &Response) -> bool {
    response.has_focus()
        && ui.input_mut(|input| {
            input.consume_key(Modifiers::NONE, Key::Enter)
                || input.consume_key(Modifiers::NONE, Key::Space)
        })
}

fn selection_summary(state: &AppState, target: ContextTarget) -> String {
    let selection = &state.schematic.selection;
    let count = selection.components.len()
        + selection.wires.len()
        + selection.wire_segments.len()
        + selection.wire_vertices.len()
        + selection.junctions.len()
        + selection.buses.len()
        + selection.bus_taps.len()
        + selection.net_labels.len()
        + selection.probes.len()
        + selection.design_notes.len()
        + selection.documentation_shapes.len();
    let path = format!("/{}", state.workspace.active_view.display_path());
    if count > 1 {
        return format!("{count} selected objects · {path}");
    }
    if let Some(id) = selection.single_bus_tap()
        && let Some(tap) = state.schematic.bus_taps.iter().find(|tap| tap.id == id)
    {
        return format!("bus tap · {} · {path}", tap.slice);
    }
    if let Some(id) = selection.single_bus()
        && let Some(bus) = state.schematic.buses.iter().find(|bus| bus.id == id)
    {
        return format!(
            "bus · {} · {path}",
            bus.declaration
                .as_ref()
                .map_or_else(|| "unnamed".to_owned(), ToString::to_string)
        );
    }
    if let Some(id) = selection.single_net_label()
        && let Some(label) = state
            .schematic
            .net_labels
            .iter()
            .find(|label| label.id == id)
    {
        return format!(
            "net label · {} · {path}",
            if label.name.trim().is_empty() {
                "<unnamed>"
            } else {
                &label.name
            }
        );
    }
    if let Some(id) = selection.single_probe()
        && let Some(probe) = state.schematic.probes.iter().find(|probe| probe.id == id)
    {
        return format!("probe · {} · {path}", probe.reference);
    }
    if let Some(id) = selection.single_design_note()
        && let Some(note) = state
            .schematic
            .design_notes
            .iter()
            .find(|note| note.id == id)
    {
        return format!("{} · {} · {path}", note.kind.label(), note.text);
    }
    if let Some(id) = selection.single_documentation_shape()
        && let Some(shape) = state
            .schematic
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == id)
    {
        return format!(
            "{} \u{b7} drawing / documentation \u{b7} {path}",
            shape.kind().label()
        );
    }
    let target = selection
        .single_component()
        .map(ContextTarget::Component)
        .or_else(|| selection.single_wire().map(ContextTarget::Wire))
        .unwrap_or(target);
    match target {
        ContextTarget::Component(id) => state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .map(|component| {
                let value = if component.value.trim().is_empty() {
                    component.library_cell.as_ref().map_or_else(
                        || component.kind.display_name(),
                        |binding| {
                            binding
                                .module_name
                                .as_deref()
                                .unwrap_or(binding.cell.as_str())
                        },
                    )
                } else {
                    component.value.trim()
                };
                format!("{} · {value} · {path}", component.name)
            })
            .unwrap_or_else(|| format!("Schematic object · {path}")),
        ContextTarget::Wire(id) => {
            let net = state
                .schematic
                .wires
                .iter()
                .find(|wire| wire.id == id)
                .and_then(|wire| {
                    wire.points.iter().find_map(|point| {
                        state.simulation.cross_probe.net_at_in(
                            &state.workspace.active_view,
                            state.schematic.topology_version(),
                            *point,
                        )
                    })
                })
                .map_or("wire", String::as_str);
            format!("wire · {net} · {path}")
        }
        ContextTarget::Canvas if count == 1 => format!("Selected schematic object · {path}"),
        ContextTarget::Canvas => format!("No object selected · {path}"),
    }
}

fn menu_header(ui: &mut Ui, summary: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), HEADER_HEIGHT), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            true,
            format!("Schematic selection: {summary}"),
        )
    });
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border_strong),
    );
    painter.text(
        pos2(rect.left() + 10.0, rect.top() + 8.0),
        Align2::LEFT_TOP,
        "Schematic selection",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let summary_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let summary = fit_text(
        painter,
        summary,
        &summary_font,
        (rect.width() - 20.0).max(1.0),
        t.color.text_faint,
    );
    painter.text(
        pos2(rect.left() + 10.0, rect.top() + 25.0),
        Align2::LEFT_TOP,
        summary,
        summary_font,
        t.color.text_faint,
    );
}

fn menu_item(
    ui: &mut Ui,
    command: ContextCommand,
    shortcut: &str,
    enabled: bool,
    disabled_reason: &'static str,
    row_height: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), row_height),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, command.label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(command.label);
        if !enabled {
            node.set_disabled();
            node.set_description(disabled_reason);
        }
        if enabled && !shortcut.is_empty() {
            node.set_keyboard_shortcut(shortcut);
        }
    });

    let focused = enabled && response.has_focus();
    let hovered = enabled && response.hovered();
    if response.is_pointer_button_down_on() && enabled {
        ui.painter().rect_filled(rect, 3.0, t.color.bg_active);
    } else if hovered || focused {
        ui.painter().rect_filled(rect, 3.0, t.color.bg_hover);
    }

    let row_color = if hovered || focused {
        t.color.text
    } else {
        t.color.text_dim
    };
    let row_color = if enabled {
        row_color
    } else {
        row_color.gamma_multiply(0.4)
    };
    let faint = if enabled {
        t.color.text_faint
    } else {
        t.color.text_faint.gamma_multiply(0.4)
    };
    let icon_rect = Rect::from_min_size(
        pos2(
            rect.left() + ROW_HORIZONTAL_PADDING,
            rect.center().y - ICON_SIDE * 0.5,
        ),
        vec2(ICON_SIDE, ICON_SIDE),
    );
    command.icon.paint(ui.painter(), icon_rect, row_color);

    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let shortcut_width = if shortcut.is_empty() {
        0.0
    } else {
        ui.painter()
            .layout_no_wrap(shortcut.to_owned(), shortcut_font.clone(), faint)
            .size()
            .x
    };
    if !shortcut.is_empty() {
        ui.painter().text(
            pos2(rect.right() - ROW_HORIZONTAL_PADDING, rect.center().y),
            Align2::RIGHT_CENTER,
            shortcut,
            shortcut_font,
            faint,
        );
    }

    let label_left = icon_rect.right() + ICON_LABEL_GAP;
    let label_right = if !shortcut.is_empty() {
        rect.right() - ROW_HORIZONTAL_PADDING - shortcut_width - ICON_LABEL_GAP
    } else {
        rect.right() - ROW_HORIZONTAL_PADDING
    };
    let label_clip = Rect::from_min_max(
        pos2(label_left, rect.top()),
        pos2(label_right.max(label_left), rect.bottom()),
    );
    ui.painter().with_clip_rect(label_clip).text(
        pos2(label_left, rect.center().y),
        Align2::LEFT_CENTER,
        command.label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        row_color,
    );

    theme::paint_focus_ring(ui, &response, rect);

    if enabled {
        response
    } else {
        response.on_hover_text(disabled_reason)
    }
}

fn menu_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), SEPARATOR_HEIGHT), Sense::hover());
    response.widget_info(|| WidgetInfo::new(WidgetType::Other));
    ui.painter().hline(
        egui::Rangef::new(rect.left() + 5.0, rect.right() - 5.0),
        rect.center().y,
        Stroke::new(1.0, t.color.border),
    );
}

fn manage_menu_focus(ui: &mut Ui, rows: &[ContextRow], focus_first: bool) {
    let enabled: Vec<&ContextRow> = rows.iter().filter(|row| row.enabled).collect();
    let Some(first) = enabled.first() else {
        return;
    };
    if focus_first {
        first.response.request_focus();
        return;
    }

    let current = enabled.iter().position(|row| row.response.has_focus());
    let movement = ui.input_mut(|input| {
        let modifiers = input.modifiers;
        if input.consume_key(modifiers, Key::ArrowDown) {
            Some(FocusMove::Next)
        } else if input.consume_key(modifiers, Key::ArrowUp) {
            Some(FocusMove::Previous)
        } else if input.consume_key(modifiers, Key::Home) {
            Some(FocusMove::First)
        } else if input.consume_key(modifiers, Key::End) {
            Some(FocusMove::Last)
        } else {
            None
        }
    });
    let Some(movement) = movement else {
        return;
    };
    let next = match movement {
        FocusMove::Next => current.map_or(0, |index| (index + 1) % enabled.len()),
        FocusMove::Previous => current.map_or(enabled.len() - 1, |index| {
            (index + enabled.len() - 1) % enabled.len()
        }),
        FocusMove::First => 0,
        FocusMove::Last => enabled.len() - 1,
    };
    enabled[next].response.request_focus();
    enabled[next].response.scroll_to_me(None);
}

#[derive(Debug, Clone, Copy)]
enum FocusMove {
    Next,
    Previous,
    First,
    Last,
}

fn action_availability(action: ContextAction, state: &AppState) -> (bool, &'static str) {
    let selection = &state.schematic.selection;
    // Components, complete wires, and explicit junctions are clipboard and
    // deletion objects. Wire segments and vertices remain edit handles.
    let has_live_component = state
        .schematic
        .components
        .iter()
        .any(|component| selection.has_component(component.id));
    let has_live_wire = state
        .schematic
        .wires
        .iter()
        .any(|wire| selection.has_wire(wire.id));
    let has_live_junction = state
        .schematic
        .junctions
        .iter()
        .any(|junction| selection.has_junction(junction.pos));
    let has_live_bus = state
        .schematic
        .buses
        .iter()
        .any(|bus| selection.has_bus(bus.id));
    let has_live_bus_tap = state
        .schematic
        .bus_taps
        .iter()
        .any(|tap| selection.has_bus_tap(tap.id));
    let has_live_net_label = state
        .schematic
        .net_labels
        .iter()
        .any(|label| selection.has_net_label(label.id));
    let has_live_probe = state
        .schematic
        .probes
        .iter()
        .any(|probe| selection.has_probe(probe.id));
    let has_live_design_note = state
        .schematic
        .design_notes
        .iter()
        .any(|note| selection.has_design_note(note.id));
    let has_live_documentation_shape = state
        .schematic
        .documentation_shapes
        .iter()
        .any(|shape| selection.has_documentation_shape(shape.id));
    let has_copyable_object = has_live_component
        || has_live_wire
        || has_live_junction
        || has_live_bus
        || has_live_bus_tap
        || has_live_net_label
        || has_live_probe
        || has_live_design_note
        || has_live_documentation_shape;
    let all_whole_object_ids_are_live = selection.components.iter().all(|id| {
        state
            .schematic
            .components
            .iter()
            .any(|component| component.id == *id)
    }) && selection
        .wires
        .iter()
        .all(|id| state.schematic.wires.iter().any(|wire| wire.id == *id))
        && selection
            .buses
            .iter()
            .all(|id| state.schematic.buses.iter().any(|bus| bus.id == *id))
        && selection
            .bus_taps
            .iter()
            .all(|id| state.schematic.bus_taps.iter().any(|tap| tap.id == *id))
        && selection.net_labels.iter().all(|id| {
            state
                .schematic
                .net_labels
                .iter()
                .any(|label| label.id == *id)
        })
        && selection
            .probes
            .iter()
            .all(|id| state.schematic.probes.iter().any(|probe| probe.id == *id))
        && selection.design_notes.iter().all(|id| {
            state
                .schematic
                .design_notes
                .iter()
                .any(|note| note.id == *id)
        })
        && selection.documentation_shapes.iter().all(|id| {
            state
                .schematic
                .documentation_shapes
                .iter()
                .any(|shape| shape.id == *id)
        });
    let all_junctions_are_live = selection.junctions.iter().all(|selected| {
        state
            .schematic
            .junctions
            .iter()
            .any(|junction| junction.pos == selected.pos)
    });
    let has_wire_sub_object =
        !selection.wire_segments.is_empty() || !selection.wire_vertices.is_empty();
    let copyable_objects_only = has_copyable_object
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    // Junction-only paste requires a separately chosen valid intersection, so
    // fixed-offset Duplicate intentionally stays unavailable for that case.
    let duplicable_objects_only = (has_live_component
        || has_live_wire
        || has_live_bus
        || has_live_bus_tap
        || has_live_net_label
        || has_live_probe
        || has_live_design_note
        || has_live_documentation_shape)
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    let deletable_objects_only = (has_copyable_object || has_live_junction)
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    let has_component = has_live_component;
    let writable = !state.schematic.read_only && !state.active_view_read_only();
    match action {
        ContextAction::Properties => (
            crate::workbench::app::selected_object_properties_available(state),
            "Select one editable component, bus, bus tap, net label, design note, or documentation shape to open its properties",
        ),
        ContextAction::Rotate | ContextAction::Mirror => (
            writable && has_component,
            "Select at least one editable component",
        ),
        ContextAction::Copy => (
            copyable_objects_only,
            "Select at least one component, wire, bus, tap, junction, net label, probe, design note, or documentation shape",
        ),
        ContextAction::Duplicate => (
            writable && duplicable_objects_only,
            "Select at least one editable component, wire, bus, tap, net label, probe, design note, or documentation shape",
        ),
        ContextAction::Delete => (
            writable && deletable_objects_only,
            "Select at least one editable component, wire, bus, tap, junction, net label, probe, or design note",
        ),
        ContextAction::DescendHierarchy => (
            state.selected_hierarchy_master().is_some(),
            "Select one instance with a resolved schematic master",
        ),
        ContextAction::UpdateInstanceInterface => (
            writable
                && state
                    .schematic
                    .selected_instance_interface_is_stale(&state.workspace.schematic_buffers),
            "Select one instance whose master interface changed after it was placed",
        ),
        // The command owns whether a replacement can be made at all — one
        // instance, editable, and a different ready master that preserves the
        // connected terminal contract. Restating any of that here would offer
        // the row where the dialog would immediately refuse.
        ContextAction::ReplaceInstance => (
            replace_instance_available(state),
            "Select one editable instance that a different ready master can stand in for",
        ),
        ContextAction::CreateHierarchy => (
            crate::workbench::app::create_hierarchy_available(state),
            "Select one or more complete editable instances and no partial objects",
        ),
        ContextAction::CreateSymbolFromPorts => (
            writable
                && !state.schematic.interface_ports().is_empty()
                && state
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .any(|library| !library.read_only),
            "Add at least one schematic port and make a writable design library available",
        ),
        ContextAction::PageSetup => (
            writable
                && matches!(
                    state.workspace.active_view_type(),
                    ViewType::Schematic | ViewType::Testbench
                ),
            "Page setup requires a writable schematic or testbench",
        ),
        ContextAction::FitContent => (
            matches!(
                state.workspace.active_view_type(),
                ViewType::Schematic | ViewType::Testbench
            ),
            "Fit is available on a schematic or testbench canvas",
        ),
        // The locator is the one owner of why a jump cannot be made, so this
        // row explains itself with the reason the command would report.
        ContextAction::ShowInNetlist => {
            let blocked = state.selected_instance_netlist_block();
            (
                blocked.is_none(),
                blocked.unwrap_or("Select one instance the generated netlist states"),
            )
        }
        ContextAction::Probe => (
            writable,
            "The active schematic view is read-only; reopen it in an editable context to place a probe",
        ),
        ContextAction::OperatingPoint => (
            operating_point_available(state),
            "Run a DC operating-point analysis with device OP reporting first",
        ),
    }
}

fn execute_context_action(
    action: ContextAction,
    ui: &mut Ui,
    state: &mut AppState,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    retain_selection_on_active_sheet(state);
    match action {
        ContextAction::Properties => {
            crate::workbench::app::open_selected_object_properties(state);
        }
        ContextAction::Rotate => with_hidden_wire_topology_preserved(state, |schematic| {
            schematic
                .rotate_selection_resolved(|component| symbol_context.terminal_points(component))
        }),
        ContextAction::Mirror => with_hidden_wire_topology_preserved(state, |schematic| {
            schematic
                .mirror_selection_h_resolved(|component| symbol_context.terminal_points(component))
        }),
        ContextAction::Copy => {
            state.copy_active_schematic_selection();
        }
        ContextAction::Duplicate => {
            crate::workbench::app::open_duplicate_selection_dialog_at(
                state,
                click_pos + Point::new(2, 2),
            );
        }
        ContextAction::Delete => {
            crate::workbench::app::open_delete_selection_dialog(state);
        }
        ContextAction::DescendHierarchy => state.open_selected_instance_master(),
        ContextAction::UpdateInstanceInterface => {
            let outcome = state.schematic.update_selected_instance_interface(
                &state.library_manager,
                &state.workspace.schematic_buffers,
            );
            state.push_user_message(match outcome {
                Ok(summary) => ConsoleMessage::info(summary),
                Err(error) => ConsoleMessage::warning(error.to_string()),
            });
        }
        ContextAction::ReplaceInstance => open_replace_instance_dialog(state),
        ContextAction::CreateHierarchy => {
            crate::workbench::app::open_create_hierarchy_dialog(state);
        }
        ContextAction::CreateSymbolFromPorts => {
            crate::workbench::app::open_create_model_bound_symbol_dialog(state);
        }
        ContextAction::PageSetup => {
            crate::workbench::app::open_drawing_sheet_setup_for_state(state);
        }
        ContextAction::FitContent => {
            state.schematic.needs_fit = true;
            state.schematic.needs_drawing_sheet_fit = false;
        }
        ContextAction::ShowInNetlist => state.show_selected_instance_in_netlist(),
        ContextAction::Probe => state.schematic.arm_tool(Tool::Probe),
        ContextAction::OperatingPoint => open_operating_point(state),
    }
    ui.close();
}

#[cfg(test)]
fn duplicate_selection_at(state: &mut AppState, click_pos: Point) {
    state.copy_active_schematic_selection();
    if !state.schematic.paste_at(click_pos + Point::new(2, 2)) {
        state.push_user_message(ConsoleMessage::warning(
            "Duplicate could not be completed at the current canvas target".to_owned(),
        ));
    }
}

fn open_operating_point(state: &mut AppState) {
    // The device under the pointer is what this hop is about. The Op inspector
    // reads one device-name selection — its docbar filter — so the hop writes
    // that, exactly as the reverse direction writes the schematic selection.
    // Leaving it unset would open the whole report and make the reader find
    // the row again.
    if let Some(device) = selected_operating_point_device(state) {
        state.ui.results.op_filter = device;
    }
    state.ui.results.viewer = ResultViewer::Op;
    state.workbench.activate(Workspace::Results);
}

/// The clicked instance's exact deck name, when one instance is selected and
/// the retained device report holds a row under that name.
///
/// The membership test is deliberate: a filter that matched nothing would
/// hide the report the reader asked to open, so an unreported device leaves
/// the inspector unfiltered rather than empty. The name comparison is the one
/// [`crate::workbench::documents::result_document::op_inspector`] uses to walk
/// the other way, so the two directions agree on what "this device" means.
fn selected_operating_point_device(state: &AppState) -> Option<String> {
    let id = state.schematic.selection.single_component()?;
    let name = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)?
        .spice_instance_name();
    operating_point_reports_device(state, &name).then_some(name)
}

fn operating_point_reports_device(state: &AppState, name: &str) -> bool {
    state.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            analysis.device_op.as_ref().is_some_and(|report| {
                report
                    .entries
                    .iter()
                    .any(|entry| entry.name.eq_ignore_ascii_case(name))
            })
        })
    })
}

fn operating_point_available(state: &AppState) -> bool {
    state.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            analysis
                .device_op
                .as_ref()
                .is_some_and(|report| !report.is_empty())
        })
    })
}

fn delete_request_id() -> Id {
    Id::new("rspice.schematic.delete-selection-request")
}

#[cfg(test)]
fn request_delete_confirmation(ctx: &Context, state: &mut AppState) {
    let mut selection = state.schematic.selection.clone();
    // Keep the reviewed payload to complete objects. Junctions are complete
    // objects and must remain in the retained request.
    selection.wire_segments.clear();
    selection.wire_vertices.clear();
    let request = DeleteSelectionRequest {
        expected_junctions: state
            .schematic
            .junctions
            .iter()
            .filter(|junction| {
                selection.has_junction(junction.pos)
                    && object_is_on_active_sheet(state, junction.id)
            })
            .copied()
            .collect(),
        expected_design_notes: state
            .schematic
            .design_notes
            .iter()
            .filter(|note| selection.has_design_note(note.id))
            .cloned()
            .collect(),
        expected_documentation_shapes: state
            .schematic
            .documentation_shapes
            .iter()
            .filter(|shape| selection.has_documentation_shape(shape.id))
            .cloned()
            .collect(),
        selection,
        topology_version: state.schematic.topology_version(),
    };
    state.dialogs.interaction.schematic_delete_confirmation_open = true;
    ctx.data_mut(|data| data.insert_temp(delete_request_id(), request));
}

fn show_delete_confirmation(
    ctx: &Context,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
) -> bool {
    if !state.dialogs.interaction.schematic_delete_confirmation_open {
        // Clear a stale payload if an owning workflow reset discarded the
        // retained modal state (for example, when a project is closed).
        ctx.data_mut(|data| data.remove::<DeleteSelectionRequest>(delete_request_id()));
        return false;
    }
    let Some(request) =
        ctx.data(|data| data.get_temp::<DeleteSelectionRequest>(delete_request_id()))
    else {
        state.dialogs.interaction.schematic_delete_confirmation_open = false;
        return false;
    };
    let review = delete_review(state, &request, symbol_context);
    let choice = Dialog::new(
        "EDIT · CONNECTIVITY IMPACT",
        "Delete schematic selection",
        "Delete selection",
    )
    .description(
        "Review the exact schematic objects and named-net impact before committing one undoable deletion transaction.",
    )
    .destructive()
    .ghost("Cancel")
    .hint("One schematic undo transaction")
    .initial_focus(DialogInitialFocus::Ghost)
    .show(ctx, |ui| {
        delete_review_row(ui, "Selection", &review.selection);
        ui.add_space(6.0);
        delete_review_row(ui, "Affected nets", &review.affected_nets);
        ui.add_space(6.0);
        delete_review_row(ui, "Dependent records", &review.dependent_records);
    });

    match choice {
        DialogChoice::None => {}
        DialogChoice::Primary => {
            state.dialogs.interaction.schematic_delete_confirmation_open = false;
            ctx.data_mut(|data| data.remove::<DeleteSelectionRequest>(delete_request_id()));
            apply_delete_request(state, request);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled | DialogChoice::Secondary => {
            state.dialogs.interaction.schematic_delete_confirmation_open = false;
            ctx.data_mut(|data| data.remove::<DeleteSelectionRequest>(delete_request_id()));
        }
    }
    true
}

fn delete_review(
    state: &AppState,
    request: &DeleteSelectionRequest,
    symbol_context: &SchematicSymbolContext,
) -> DeleteReview {
    let mut objects = Vec::new();
    for component in &state.schematic.components {
        if request.selection.has_component(component.id)
            && object_is_on_active_sheet(state, component.id)
        {
            objects.push(component.name.clone());
        }
    }
    for wire in &state.schematic.wires {
        if request.selection.has_wire(wire.id) && object_is_on_active_sheet(state, wire.id) {
            objects.push(format!("wire #{}", wire.id));
        }
    }
    for junction in &state.schematic.junctions {
        if request.selection.has_junction(junction.pos)
            && object_is_on_active_sheet(state, junction.id)
        {
            objects.push(format!("junction ({}, {})", junction.pos.x, junction.pos.y));
        }
    }
    for bus in &state.schematic.buses {
        if request.selection.has_bus(bus.id) && object_is_on_active_sheet(state, bus.id) {
            objects.push(format!(
                "bus {}",
                bus.declaration
                    .as_ref()
                    .map_or_else(|| format!("#{} (unnamed)", bus.id), ToString::to_string)
            ));
        }
    }
    for tap in &state.schematic.bus_taps {
        if request.selection.has_bus_tap(tap.id) && object_is_on_active_sheet(state, tap.id) {
            objects.push(format!("bus tap {}", tap.slice));
        }
    }
    for label in &state.schematic.net_labels {
        if request.selection.has_net_label(label.id) && object_is_on_active_sheet(state, label.id) {
            objects.push(format!("net label {}", label.name));
        }
    }
    for probe in &state.schematic.probes {
        if request.selection.has_probe(probe.id) && object_is_on_active_sheet(state, probe.id) {
            objects.push(format!("probe {}", probe.reference));
        }
    }
    for note in &state.schematic.design_notes {
        if request.selection.has_design_note(note.id) && object_is_on_active_sheet(state, note.id) {
            objects.push(format!("{} {}", note.kind.label(), note.text));
        }
    }
    for shape in &state.schematic.documentation_shapes {
        if request.selection.has_documentation_shape(shape.id)
            && object_is_on_active_sheet(state, shape.id)
        {
            objects.push(format!(
                "{} documentation shape #{}",
                shape.kind().label(),
                shape.id
            ));
        }
    }
    let selection = if objects.is_empty() {
        "No live schematic objects".to_owned()
    } else {
        objects.join(" · ")
    };

    let mut nets = BTreeSet::new();
    for component in &state.schematic.components {
        if !request.selection.has_component(component.id)
            || !object_is_on_active_sheet(state, component.id)
        {
            continue;
        }
        for point in symbol_context.terminal_points(component) {
            if let Some(net) = state.simulation.cross_probe.net_at_in(
                &state.workspace.active_view,
                state.schematic.topology_version(),
                point,
            ) {
                nets.insert(net.clone());
            }
        }
    }
    for wire in &state.schematic.wires {
        if !request.selection.has_wire(wire.id) || !object_is_on_active_sheet(state, wire.id) {
            continue;
        }
        for point in &wire.points {
            if let Some(net) = state.simulation.cross_probe.net_at_in(
                &state.workspace.active_view,
                state.schematic.topology_version(),
                *point,
            ) {
                nets.insert(net.clone());
            }
        }
    }
    for junction in &state.schematic.junctions {
        if request.selection.has_junction(junction.pos)
            && object_is_on_active_sheet(state, junction.id)
            && let Some(net) = state.simulation.cross_probe.net_at_in(
                &state.workspace.active_view,
                state.schematic.topology_version(),
                junction.pos,
            )
        {
            nets.insert(net.clone());
        }
    }
    for tap in &state.schematic.bus_taps {
        if object_is_on_active_sheet(state, tap.id)
            && (request.selection.has_bus_tap(tap.id) || request.selection.has_bus(tap.bus_id))
        {
            // The review surface describes a vector selector as one typed
            // object. Expanding a legal wide range into hundreds of thousands
            // of strings would freeze the modal without adding useful review
            // information; scalar selectors already format identically.
            nets.insert(tap.slice.to_string());
        }
    }
    for label in &state.schematic.net_labels {
        if request.selection.has_net_label(label.id) && object_is_on_active_sheet(state, label.id) {
            nets.insert(label.name.clone());
        }
    }
    let affected_nets = if nets.is_empty() {
        "Unnamed or unmapped connectivity will be recomputed".to_owned()
    } else {
        nets.into_iter().collect::<Vec<_>>().join(" · ")
    };
    let dependent_records = if state.dialogs.drc_results.is_some() {
        "Schematic checks become stale · retained simulation results remain immutable".to_owned()
    } else {
        "Future generated runs bind to the new topology · retained results remain immutable"
            .to_owned()
    };
    DeleteReview {
        selection,
        affected_nets,
        dependent_records,
    }
}

fn delete_review_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(CornerRadius::same(3))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
}

fn apply_delete_request(state: &mut AppState, request: DeleteSelectionRequest) {
    let active_selection = selection_filtered_to_active_sheet(state, &request.selection);
    let live_junctions = state
        .schematic
        .junctions
        .iter()
        .filter(|junction| {
            request.selection.has_junction(junction.pos)
                && object_is_on_active_sheet(state, junction.id)
        })
        .copied()
        .collect::<Vec<_>>();
    let live_design_notes: Vec<_> = state
        .schematic
        .design_notes
        .iter()
        .filter(|note| request.selection.has_design_note(note.id))
        .cloned()
        .collect();
    let live_documentation_shapes: Vec<_> = state
        .schematic
        .documentation_shapes
        .iter()
        .filter(|shape| request.selection.has_documentation_shape(shape.id))
        .cloned()
        .collect();
    if state.schematic.topology_version() != request.topology_version
        || active_selection != request.selection
        || live_junctions != request.expected_junctions
        || live_design_notes != request.expected_design_notes
        || live_documentation_shapes != request.expected_documentation_shapes
    {
        state.push_user_message(ConsoleMessage::warning(
            "The schematic changed while deletion was being reviewed; nothing was deleted."
                .to_owned(),
        ));
        return;
    }
    if state.schematic.read_only {
        state.push_user_message(ConsoleMessage::warning(
            "The schematic became read-only; nothing was deleted.".to_owned(),
        ));
        return;
    }
    let has_live_object = state
        .schematic
        .components
        .iter()
        .any(|component| request.selection.has_component(component.id))
        || state
            .schematic
            .wires
            .iter()
            .any(|wire| request.selection.has_wire(wire.id))
        || state
            .schematic
            .junctions
            .iter()
            .any(|junction| request.selection.has_junction(junction.pos))
        || state
            .schematic
            .buses
            .iter()
            .any(|bus| request.selection.has_bus(bus.id))
        || state
            .schematic
            .bus_taps
            .iter()
            .any(|tap| request.selection.has_bus_tap(tap.id))
        || state
            .schematic
            .net_labels
            .iter()
            .any(|label| request.selection.has_net_label(label.id))
        || state
            .schematic
            .probes
            .iter()
            .any(|probe| request.selection.has_probe(probe.id))
        || state
            .schematic
            .design_notes
            .iter()
            .any(|note| request.selection.has_design_note(note.id))
        || state
            .schematic
            .documentation_shapes
            .iter()
            .any(|shape| request.selection.has_documentation_shape(shape.id));
    if !has_live_object {
        state.push_user_message(ConsoleMessage::warning(
            "The reviewed selection no longer contains deletable objects.".to_owned(),
        ));
        return;
    }
    state.schematic.selection = request.selection;
    if !with_hidden_wire_topology_preserved(state, |schematic| schematic.delete_selection()) {
        state.push_user_message(ConsoleMessage::warning(
            "The reviewed selection no longer contains deletable objects.".to_owned(),
        ));
    }
}

impl ContextIcon {
    fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        match self {
            Self::Sliders => WorkbenchIcon::Sliders.paint(painter, rect, color),
            Self::Rotate => WorkbenchIcon::Rotate.paint(painter, rect, color),
            Self::Mirror => WorkbenchIcon::Mirror.paint(painter, rect, color),
            Self::Copy => paint_copy_icon(painter, rect, color),
            Self::Trash => Icon::Trash.paint(painter, rect, color),
            Self::Hierarchy => WorkbenchIcon::Instance.paint(painter, rect, color),
            Self::Sheet => WorkbenchIcon::Layers.paint(painter, rect, color),
            Self::Fit => WorkbenchIcon::ZoomFit.paint(painter, rect, color),
            Self::Code => WorkbenchIcon::Code.paint(painter, rect, color),
            Self::Probe => WorkbenchIcon::Probe.paint(painter, rect, color),
            Self::Waveform => WorkbenchIcon::Simulate.paint(painter, rect, color),
        }
    }
}

fn paint_copy_icon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let scale = rect.width().min(rect.height()) / 24.0;
    let origin = rect.center() - vec2(12.0, 12.0) * scale;
    let map = |x: f32, y: f32| origin + vec2(x, y) * scale;
    let stroke = Stroke::new((1.7 * rect.width() / 16.0).max(1.0), color);
    painter.rect_stroke(
        Rect::from_min_max(map(8.0, 4.0), map(20.0, 16.0)),
        1.0,
        stroke,
        StrokeKind::Middle,
    );
    painter.rect_stroke(
        Rect::from_min_max(map(4.0, 8.0), map(16.0, 20.0)),
        1.0,
        stroke,
        StrokeKind::Middle,
    );
}

fn fit_text(
    painter: &egui::Painter,
    text: &str,
    font: &egui::FontId,
    max_width: f32,
    color: Color32,
) -> String {
    if painter
        .layout_no_wrap(text.to_owned(), font.clone(), color)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let mut output: String = text.chars().collect();
    while !output.is_empty() {
        output.pop();
        let candidate = format!("{}…", output.trim_end());
        if painter
            .layout_no_wrap(candidate.clone(), font.clone(), color)
            .size()
            .x
            <= max_width
        {
            return candidate;
        }
    }
    "…".to_owned()
}

fn context_shadow(active_tokens: &Tokens) -> Shadow {
    let color = if active_tokens.mode == tokens::Mode::Light {
        // rgb(41 46 50 / 22%), premultiplied to egui's Color32 storage.
        Color32::from_rgba_premultiplied(9, 10, 11, 56)
    } else {
        Color32::from_rgba_premultiplied(0, 0, 0, 97)
    };
    Shadow {
        offset: [0, 16],
        blur: 40,
        spread: 0,
        color,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SurfaceGeometry {
    width: f32,
    max_height: f32,
    row_height: f32,
    radius: u8,
}

impl SurfaceGeometry {
    fn resolve(ctx: &Context, invocation: ContextInvocation) -> Self {
        Self::for_viewport(ctx.content_rect().size(), invocation)
    }

    fn for_viewport(viewport: egui::Vec2, invocation: ContextInvocation) -> Self {
        if invocation == ContextInvocation::TouchSheet {
            Self {
                width: (viewport.x - 2.0 * TOUCH_VIEWPORT_INSET).clamp(1.0, TOUCH_MAX_WIDTH),
                max_height: (viewport.y * TOUCH_VIEWPORT_FRACTION).min(TOUCH_MAX_HEIGHT),
                row_height: TOUCH_ROW_HEIGHT,
                radius: TOUCH_RADIUS,
            }
        } else {
            Self {
                width: DESKTOP_WIDTH,
                max_height: DESKTOP_MAX_HEIGHT.min((viewport.y - 12.0).max(1.0)),
                row_height: DESKTOP_ROW_HEIGHT,
                radius: DESKTOP_RADIUS,
            }
        }
    }

    fn outer_height(self) -> f32 {
        let (commands, separators) = CONTEXT_ENTRIES.iter().fold(
            (0_u32, 0_u32),
            |(commands, separators), entry| match entry {
                ContextEntry::Command(_) => (commands + 1, separators),
                ContextEntry::Separator => (commands, separators + 1),
            },
        );
        (HEADER_HEIGHT
            + commands as f32 * self.row_height
            + separators as f32 * SEPARATOR_HEIGHT
            + SURFACE_BORDER_WIDTH)
            .min(self.max_height)
    }
}

#[cfg(test)]
mod tests;
