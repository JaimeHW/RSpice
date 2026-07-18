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

use crate::common::app::{AppState, ConsoleMessage, ContextTarget};
use crate::state::{Point, Selection, Tool};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus};
use crate::workbench::ResultViewer;
use crate::workbench::commands::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::Workspace;

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::drawing::{bus_tap_at, nearest_bus_hit};
use super::viewport::Viewport;

const DESKTOP_WIDTH: f32 = 286.0;
const DESKTOP_MAX_HEIGHT: f32 = 520.0;
const DESKTOP_VIEWPORT_INSET: f32 = 6.0;
const DESKTOP_ROW_HEIGHT: f32 = 30.0;
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
    routing_was_active: bool,
    symbol_context: &SchematicSymbolContext,
) {
    if show_delete_confirmation(&response.ctx, state, symbol_context) {
        return;
    }
    if routing_was_active
        || state.schematic.wire_drawing.active
        || state.schematic.bus_drawing.active
    {
        return;
    }

    let ctx = &response.ctx;
    let popup_id = Popup::default_response_id(response);
    let invocation_id = popup_id.with("invocation");
    let surface_anchor_id = popup_id.with("surface-anchor");
    let was_open = Popup::is_id_open(ctx, popup_id);

    let keyboard_open = response.has_focus()
        && ctx.input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10));
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
    let target = select_pointer_target(state, grid_pos, hit_pos, hit_radius, symbol_context);
    state.dialogs.interaction.context_target = Some((target, (grid_pos.x, grid_pos.y)));
    Some(pos)
}

fn select_pointer_target(
    state: &mut AppState,
    grid_pos: Point,
    hit_pos: Point,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
) -> ContextTarget {
    if let Some(id) = symbol_context
        .component_at_resolved_symbol(&state.schematic.components, grid_pos)
        .or_else(|| state.schematic.component_at(grid_pos))
    {
        state.schematic.net_highlight.clear();
        if !state.schematic.selection.has_component(id) {
            state.schematic.selection.clear();
            state.schematic.selection.select_component(id);
        }
        ContextTarget::Component(id)
    } else if let Some(id) = bus_tap_at(&state.schematic.bus_taps, hit_pos, hit_radius) {
        state.schematic.net_highlight.clear();
        if !state.schematic.selection.has_bus_tap(id) {
            state.schematic.selection.select_only_bus_tap(id);
        }
        ContextTarget::Canvas
    } else if state.schematic.has_junction(grid_pos) {
        state.schematic.net_highlight.clear();
        if !state.schematic.selection.has_junction(grid_pos) {
            state.schematic.selection.clear();
            state.schematic.selection.select_junction(grid_pos);
        }
        // ContextTarget has no junction variant; the retained selection is
        // authoritative for action availability and deletion review.
        ContextTarget::Canvas
    } else if let Some(id) =
        nearest_bus_hit(&state.schematic.buses, hit_pos, hit_radius).map(|hit| hit.bus_id)
    {
        state.schematic.net_highlight.clear();
        if !state.schematic.selection.has_bus(id) {
            state.schematic.selection.select_only_bus(id);
        }
        ContextTarget::Canvas
    } else if let Some(id) = state.schematic.wire_at(grid_pos) {
        state.schematic.net_highlight.clear();
        if !state.schematic.selection.has_wire(id) {
            state.schematic.selection.clear();
            state.schematic.selection.select_wire(id);
        }
        ContextTarget::Wire(id)
    } else {
        ContextTarget::Canvas
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

    let mut rows = Vec::with_capacity(8);
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
                                crate::common::app::runtime_command_platform(ui.ctx()),
                                ui.ctx().os(),
                            )
                        });
                let response = menu_item(ui, command, &shortcut, enabled, reason, row_height);
                let clicked = enabled && response.clicked();
                rows.push(ContextRow { response, enabled });
                if clicked {
                    execute_context_action(command.action, ui, state, click_pos, symbol_context);
                }
            }
        }
    }
    manage_menu_focus(ui, &rows, focus_first);
}

fn selection_summary(state: &AppState, target: ContextTarget) -> String {
    let selection = &state.schematic.selection;
    let count = selection.components.len()
        + selection.wires.len()
        + selection.wire_segments.len()
        + selection.wire_vertices.len()
        + selection.junctions.len()
        + selection.buses.len()
        + selection.bus_taps.len();
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
                    wire.points
                        .iter()
                        .find_map(|point| state.simulation.cross_probe.net_at(*point))
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
    let has_copyable_object = has_live_component
        || has_live_wire
        || has_live_junction
        || has_live_bus
        || has_live_bus_tap;
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
            .all(|id| state.schematic.bus_taps.iter().any(|tap| tap.id == *id));
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
    let duplicable_objects_only =
        (has_live_component || has_live_wire || has_live_bus || has_live_bus_tap)
            && all_whole_object_ids_are_live
            && all_junctions_are_live
            && !has_wire_sub_object;
    let deletable_objects_only = (has_copyable_object || has_live_junction)
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    let has_component = has_live_component;
    let writable = !state.schematic.read_only;
    match action {
        ContextAction::Properties => (
            writable
                && selection.single_component().is_some_and(|id| {
                    state
                        .schematic
                        .components
                        .iter()
                        .any(|component| component.id == id)
                }),
            "Select one editable component to open its properties",
        ),
        ContextAction::Rotate | ContextAction::Mirror => (
            writable && has_component,
            "Select at least one editable component",
        ),
        ContextAction::Copy => (
            copyable_objects_only,
            "Select at least one component, wire, bus, tap, or junction",
        ),
        ContextAction::Duplicate => (
            writable && duplicable_objects_only,
            "Select at least one editable component, wire, bus, or tap",
        ),
        ContextAction::Delete => (
            writable && deletable_objects_only,
            "Select at least one editable component, wire, bus, tap, or junction",
        ),
        ContextAction::Probe => (true, ""),
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
    match action {
        ContextAction::Properties => {
            if let Some(id) = state.schematic.selection.single_component() {
                crate::common::app::open_property_editor(state, id);
            }
        }
        ContextAction::Rotate => state
            .schematic
            .rotate_selection_resolved(|component| symbol_context.terminal_points(component)),
        ContextAction::Mirror => state
            .schematic
            .mirror_selection_h_resolved(|component| symbol_context.terminal_points(component)),
        ContextAction::Copy => state.schematic.copy_selection(),
        ContextAction::Duplicate => duplicate_selection_at(state, click_pos),
        ContextAction::Delete => request_delete_confirmation(ui.ctx(), state),
        ContextAction::Probe => {
            crate::workbench::commands::arm_schematic_tool(&mut state.schematic, Tool::Probe)
        }
        ContextAction::OperatingPoint => open_operating_point(state),
    }
    ui.close();
}

fn duplicate_selection_at(state: &mut AppState, click_pos: Point) {
    state.schematic.copy_selection();
    if !state.schematic.paste_at(click_pos + Point::new(2, 2)) {
        state.push_user_message(ConsoleMessage::warning(
            "Duplicate could not be completed at the current canvas target".to_owned(),
        ));
    }
}

fn open_operating_point(state: &mut AppState) {
    state.ui.results.viewer = ResultViewer::Op;
    state.workbench.activate(Workspace::Results);
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

fn request_delete_confirmation(ctx: &Context, state: &mut AppState) {
    let mut selection = state.schematic.selection.clone();
    // Keep the reviewed payload to complete objects. Junctions are complete
    // objects and must remain in the retained request.
    selection.wire_segments.clear();
    selection.wire_vertices.clear();
    let request = DeleteSelectionRequest {
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
        if request.selection.has_component(component.id) {
            objects.push(component.name.clone());
        }
    }
    for wire in &state.schematic.wires {
        if request.selection.has_wire(wire.id) {
            objects.push(format!("wire #{}", wire.id));
        }
    }
    for junction in &state.schematic.junctions {
        if request.selection.has_junction(junction.pos) {
            objects.push(format!("junction ({}, {})", junction.pos.x, junction.pos.y));
        }
    }
    for bus in &state.schematic.buses {
        if request.selection.has_bus(bus.id) {
            objects.push(format!(
                "bus {}",
                bus.declaration
                    .as_ref()
                    .map_or_else(|| format!("#{} (unnamed)", bus.id), ToString::to_string)
            ));
        }
    }
    for tap in &state.schematic.bus_taps {
        if request.selection.has_bus_tap(tap.id) {
            objects.push(format!("bus tap {}", tap.slice));
        }
    }
    let selection = if objects.is_empty() {
        "No live schematic objects".to_owned()
    } else {
        objects.join(" · ")
    };

    let mut nets = BTreeSet::new();
    for component in &state.schematic.components {
        if !request.selection.has_component(component.id) {
            continue;
        }
        for point in symbol_context.terminal_points(component) {
            if let Some(net) = state.simulation.cross_probe.net_at(point) {
                nets.insert(net.clone());
            }
        }
    }
    for wire in &state.schematic.wires {
        if !request.selection.has_wire(wire.id) {
            continue;
        }
        for point in &wire.points {
            if let Some(net) = state.simulation.cross_probe.net_at(*point) {
                nets.insert(net.clone());
            }
        }
    }
    for junction in &state.schematic.junctions {
        if request.selection.has_junction(junction.pos)
            && let Some(net) = state.simulation.cross_probe.net_at(junction.pos)
        {
            nets.insert(net.clone());
        }
    }
    for tap in &state.schematic.bus_taps {
        if request.selection.has_bus_tap(tap.id) || request.selection.has_bus(tap.bus_id) {
            // The review surface describes a vector selector as one typed
            // object. Expanding a legal wide range into hundreds of thousands
            // of strings would freeze the modal without adding useful review
            // information; scalar selectors already format identically.
            nets.insert(tap.slice.to_string());
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
    if state.schematic.topology_version() != request.topology_version {
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
            .any(|tap| request.selection.has_bus_tap(tap.id));
    if !has_live_object {
        state.push_user_message(ConsoleMessage::warning(
            "The reviewed selection no longer contains deletable objects.".to_owned(),
        ));
        return;
    }
    state.schematic.selection = request.selection;
    if !state.schematic.delete_selection() {
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
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Junction, LibraryCellInstance, Wire};

    #[test]
    fn command_catalog_matches_the_mockup_exactly() {
        let labels: Vec<_> = CONTEXT_ENTRIES
            .iter()
            .filter_map(|entry| match entry {
                ContextEntry::Command(command) => Some((command.label, command.shortcut_command)),
                ContextEntry::Separator => None,
            })
            .collect();
        assert_eq!(
            labels,
            vec![
                ("Object properties…", Some(Command::ObjectProperties)),
                ("Rotate 90°", Some(Command::RotateSelection)),
                ("Mirror", Some(Command::MirrorSelectionHorizontal)),
                ("Copy selection", Some(Command::Copy)),
                ("Duplicate and place", Some(Command::Duplicate)),
                ("Delete selection…", Some(Command::Delete)),
                ("Add voltage or current probe…", Some(Command::PlaceProbe)),
                (
                    "Open operating point",
                    Some(Command::ResultViewer(ResultViewer::Op)),
                ),
            ]
        );
        assert_eq!(
            CONTEXT_ENTRIES
                .iter()
                .filter(|entry| matches!(entry, ContextEntry::Separator))
                .count(),
            2
        );
    }

    #[test]
    fn header_summary_uses_the_selected_instance_identity_and_master() {
        let mut state = AppState::default();
        state.schematic.components.clear();
        let mut component =
            Component::new(7, ComponentType::CellInstance, Point::origin()).with_library_cell(
                LibraryCellInstance::new("vendor_analog", "OPA189_A", "schematic"),
            );
        component.name = "U1".to_owned();
        state.schematic.components.push(component);
        state.schematic.selection.select_component(7);

        let summary = selection_summary(&state, ContextTarget::Canvas);

        assert!(summary.starts_with("U1 · OPA189_A · /"));
    }

    #[test]
    fn surface_geometry_matches_desktop_and_touch_contracts() {
        let desktop =
            SurfaceGeometry::for_viewport(vec2(1440.0, 900.0), ContextInvocation::Pointer);
        assert_eq!(desktop.width, 286.0);
        assert_eq!(desktop.max_height, 520.0);
        assert_eq!(desktop.row_height, 30.0);
        assert_eq!(desktop.radius, 3);
        assert_eq!(desktop.outer_height(), 307.0);

        let touch =
            SurfaceGeometry::for_viewport(vec2(390.0, 844.0), ContextInvocation::TouchSheet);
        assert_eq!(touch.width, 374.0);
        assert_eq!(touch.max_height, 560.0);
        assert_eq!(touch.row_height, 44.0);
        assert_eq!(touch.radius, 7);
        assert_eq!(touch.outer_height(), 419.0);

        let short_touch =
            SurfaceGeometry::for_viewport(vec2(1024.0, 500.0), ContextInvocation::TouchSheet);
        assert_eq!(short_touch.width, 420.0);
        assert_eq!(short_touch.max_height, 350.0);
        assert_eq!(short_touch.outer_height(), 350.0);
    }

    #[test]
    fn desktop_origin_and_keyboard_anchor_match_the_mockup_contract() {
        let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
        let desktop = SurfaceGeometry::for_viewport(screen.size(), ContextInvocation::Pointer);

        assert_eq!(
            clamp_desktop_surface_origin(screen, pos2(-20.0, -10.0), desktop),
            pos2(6.0, 6.0)
        );
        assert_eq!(
            clamp_desktop_surface_origin(screen, pos2(790.0, 590.0), desktop),
            pos2(508.0, 287.0)
        );
        assert_eq!(
            keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(1000.0, 500.0),)),
            pos2(124.0, 224.0)
        );
        assert_eq!(
            keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(20.0, 10.0),)),
            pos2(110.0, 205.0)
        );
    }

    #[test]
    fn context_shadow_matches_dark_and_light_mockup_tokens() {
        let dark = Tokens::new(
            tokens::Direction::Instrument,
            tokens::Mode::Dark,
            tokens::Density::default(),
        );
        let light = Tokens::new(
            tokens::Direction::Instrument,
            tokens::Mode::Light,
            tokens::Density::default(),
        );

        assert_eq!(
            context_shadow(&dark),
            Shadow {
                offset: [0, 16],
                blur: 40,
                spread: 0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 97),
            }
        );
        assert_eq!(
            context_shadow(&light),
            Shadow {
                offset: [0, 16],
                blur: 40,
                spread: 0,
                color: Color32::from_rgba_premultiplied(9, 10, 11, 56),
            }
        );
    }

    #[test]
    fn actions_are_truthfully_disabled_without_a_compatible_selection() {
        let mut state = AppState::default();
        assert!(!action_availability(ContextAction::Properties, &state).0);
        assert!(!action_availability(ContextAction::Rotate, &state).0);
        assert!(!action_availability(ContextAction::Copy, &state).0);
        assert!(!action_availability(ContextAction::Delete, &state).0);
        assert!(action_availability(ContextAction::Probe, &state).0);
        assert!(!action_availability(ContextAction::OperatingPoint, &state).0);

        state.schematic.selection.select_wire_segment(17, 0);
        assert!(!action_availability(ContextAction::Copy, &state).0);
        assert!(!action_availability(ContextAction::Duplicate, &state).0);
        assert!(!action_availability(ContextAction::Delete, &state).0);

        state.schematic.selection.select_component(999);
        assert!(!action_availability(ContextAction::Copy, &state).0);
        state.schematic.read_only = true;
        assert!(action_availability(ContextAction::Probe, &state).0);

        let mut junction_state = AppState::default();
        let point = Point::new(4, 4);
        junction_state
            .schematic
            .junctions
            .push(Junction::new(81, point));
        junction_state
            .schematic
            .selection
            .select_only_junction(point);
        assert!(action_availability(ContextAction::Delete, &junction_state).0);
        assert!(action_availability(ContextAction::Copy, &junction_state).0);
        assert!(!action_availability(ContextAction::Duplicate, &junction_state).0);
    }

    #[test]
    fn pointer_target_prefers_a_junction_over_its_underlying_wire() {
        let mut state = AppState::default();
        let point = Point::new(10, 10);
        state.schematic.wires = vec![Wire::new(17, vec![Point::new(0, 10), Point::new(20, 10)])];
        state.schematic.junctions = vec![Junction::new(18, point)];
        let symbol_context = SchematicSymbolContext::from_state(&state);

        let target = select_pointer_target(&mut state, point, point, 1, &symbol_context);

        assert!(matches!(target, ContextTarget::Canvas));
        assert_eq!(state.schematic.selection.single_junction(), Some(point));
        assert!(state.schematic.selection.wires.is_empty());
    }

    #[test]
    fn duplicate_and_delete_are_real_undoable_transactions() {
        let mut state = AppState::default();
        state.schematic.components.clear();
        state.schematic.wires.clear();
        state.schematic.init_undo_history();
        let mut component = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
        component.name = "R1".to_owned();
        state.schematic.components.push(component);
        state.schematic.selection.select_component(41);

        duplicate_selection_at(&mut state, Point::new(20, 30));
        assert_eq!(state.schematic.components.len(), 2);
        assert!(state.schematic.can_undo());
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components.len(), 1);

        state.schematic.selection.select_component(41);
        let request = DeleteSelectionRequest {
            selection: state.schematic.selection.clone(),
            topology_version: state.schematic.topology_version(),
        };
        apply_delete_request(&mut state, request);
        assert!(state.schematic.components.is_empty());
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components.len(), 1);
    }

    #[test]
    fn stale_delete_review_fails_closed() {
        let mut state = AppState::default();
        state.schematic.components.clear();
        let component = Component::new(9, ComponentType::Capacitor, Point::new(0, 0));
        state.schematic.components.push(component);
        state.schematic.selection.select_component(9);
        let request = DeleteSelectionRequest {
            selection: state.schematic.selection.clone(),
            topology_version: state.schematic.topology_version(),
        };
        state.schematic.bump_topology_version();

        apply_delete_request(&mut state, request);

        assert_eq!(state.schematic.components.len(), 1);
    }

    #[test]
    fn delete_review_owns_modal_shortcuts_and_fails_closed_without_payload() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.selection.select_component(23);
        state.schematic.selection.select_wire_segment(17, 0);
        state.schematic.selection.select_junction(Point::new(5, 8));
        request_delete_confirmation(&ctx, &mut state);

        assert!(state.dialogs.interaction.schematic_delete_confirmation_open);
        assert!(state.dialogs.application_modal_open());
        let request = ctx
            .data(|data| data.get_temp::<DeleteSelectionRequest>(delete_request_id()))
            .expect("delete request is retained");
        assert!(request.selection.has_component(23));
        assert!(request.selection.wire_segments.is_empty());
        assert!(request.selection.has_junction(Point::new(5, 8)));

        ctx.data_mut(|data| data.remove::<DeleteSelectionRequest>(delete_request_id()));
        let symbol_context = SchematicSymbolContext::from_state(&state);
        assert!(!show_delete_confirmation(&ctx, &mut state, &symbol_context,));
        assert!(!state.dialogs.interaction.schematic_delete_confirmation_open);
    }
}
