//! Right-click context menus — the editor's third hand
//! (`design/app/volta-schematic-editor.html` §08).
//!
//! One menu per target: instance, wire segment, empty canvas. Every item
//! is a route the keyboard or menubar already provides — the menu teaches
//! the shortcut, it never invents behavior. While a wire run is live,
//! right-click keeps its shipped meaning (finish the run) and no menu
//! opens, not even on the click that finishes it.

use egui::{Response, Ui};

use crate::common::app::{AppState, ConsoleMessage, ContextTarget};
use crate::shell::menubar::{item, item_disabled, separator};
use crate::state::{ComponentType, NetGraph, Point};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::SchematicSymbolContext;
use super::coordinates::screen_to_grid;
use super::viewport::Viewport;

pub(super) fn handle_context_menu(
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    wire_was_active: bool,
    symbol_context: &SchematicSymbolContext,
) {
    if wire_was_active || state.schematic.wire_drawing.active {
        return;
    }

    // Capture the target on the click frame; the menu re-renders from it
    // for as long as egui keeps the popup open. Right-click also moves the
    // selection to the thing under the cursor — the menu header names it,
    // so the selection must agree.
    if response.secondary_clicked()
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = screen_to_grid(viewport, state.schematic.grid_size, pos);
        let target = if let Some(id) = symbol_context
            .component_at_resolved_symbol(&state.schematic.components, grid_pos)
            .or_else(|| state.schematic.component_at(grid_pos))
        {
            state.schematic.net_highlight.clear();
            state.schematic.selection.clear();
            state.schematic.selection.select_component(id);
            ContextTarget::Component(id)
        } else if let Some(id) = state.schematic.wire_at(grid_pos) {
            state.schematic.net_highlight.clear();
            state.schematic.selection.clear();
            state.schematic.selection.select_wire(id);
            ContextTarget::Wire(id)
        } else {
            ContextTarget::Canvas
        };
        state.dialogs.interaction.context_target = Some((target, (grid_pos.x, grid_pos.y)));
    }

    response.clone().context_menu(|ui| {
        ui.set_min_width(224.0);
        let Some((target, (x, y))) = state.dialogs.interaction.context_target else {
            ui.close_menu();
            return;
        };
        let click_pos = Point::new(x, y);
        match target {
            ContextTarget::Component(id) => instance_menu(ui, state, id, symbol_context),
            ContextTarget::Wire(id) => wire_menu(ui, state, id, click_pos),
            ContextTarget::Canvas => canvas_menu(ui, state, click_pos),
        }
    });
}

/// A menu row that disables on read-only views instead of vanishing —
/// the verb stays discoverable, the refusal stays honest.
fn edit_item(ui: &mut Ui, read_only: bool, label: &str, kbd: Option<&str>) -> bool {
    if read_only {
        item_disabled(ui, label, kbd);
        false
    } else {
        item(ui, label, kbd)
    }
}

/// Faint mono header naming what the menu is for.
fn menu_header(ui: &mut Ui, label: &str) {
    let c = Tokens::get(ui.ctx()).color;
    let mut job = egui::text::LayoutJob::default();
    job.append(
        label,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: c.text_faint,
            extra_letter_spacing: 0.06 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.add_space(9.0);
        ui.label(job);
    });
    ui.add_space(2.0);
}

fn instance_menu(
    ui: &mut Ui,
    state: &mut AppState,
    id: u64,
    symbol_context: &SchematicSymbolContext,
) {
    let Some((name, kind)) = state
        .schematic
        .components
        .iter()
        .find(|c| c.id == id)
        .map(|c| (c.name.clone(), c.kind))
    else {
        // Deleted out from under the open menu.
        ui.close_menu();
        return;
    };

    let ro = state.schematic.read_only;
    menu_header(ui, &format!("{name} · {}", kind.display_name()));
    if edit_item(ui, ro, "Properties…", Some("E")) {
        crate::common::app::open_property_editor(state, id);
        ui.close_menu();
    }
    if edit_item(ui, ro, "Rotate", Some("R")) {
        state
            .schematic
            .rotate_selection_resolved(|component| symbol_context.terminal_points(component));
        ui.close_menu();
    }
    if edit_item(ui, ro, "Mirror horizontal", Some("H")) {
        state
            .schematic
            .mirror_selection_h_resolved(|component| symbol_context.terminal_points(component));
        ui.close_menu();
    }
    if edit_item(ui, ro, "Mirror vertical", Some("Y")) {
        state
            .schematic
            .mirror_selection_v_resolved(|component| symbol_context.terminal_points(component));
        ui.close_menu();
    }
    separator(ui);
    if edit_item(ui, ro, "Cut", Some("Ctrl+X")) {
        state.schematic.copy_selection();
        state.schematic.delete_selection();
        ui.close_menu();
    }
    if item(ui, "Copy", Some("Ctrl+C")) {
        state.schematic.copy_selection();
        ui.close_menu();
    }
    if edit_item(ui, ro, "Delete", Some("Del")) {
        state.schematic.delete_selection();
        ui.close_menu();
    }
    separator(ui);
    // Only cell instances have a master to enter; the row stays visible on
    // primitives so the gesture is discoverable, disabled so it is honest.
    if kind == ComponentType::CellInstance {
        if item(ui, "Descend into master", Some("Shift+E")) {
            state.open_selected_instance_master();
            ui.close_menu();
        }
    } else {
        item_disabled(ui, "Descend into master", Some("Shift+E"));
    }
}

fn wire_menu(ui: &mut Ui, state: &mut AppState, id: u64, click_pos: Point) {
    let net = state
        .simulation
        .cross_probe
        .net_at(click_pos)
        .cloned()
        .map(|net| format!("wire · {net}"))
        .unwrap_or_else(|| "wire".to_owned());
    let ro = state.schematic.read_only;
    menu_header(ui, &net);

    if edit_item(ui, ro, "Add net label", Some("N")) {
        let name = format!("net{}", state.schematic.net_labels.len() + 1);
        state.schematic.with_undo("place net label", |schematic| {
            schematic.add_net_label(click_pos, name);
        });
        state.schematic.is_dirty = true;
        ui.close_menu();
    }
    if item(ui, "Highlight net", Some("P")) {
        let net_graph = NetGraph::build_from_wires(&state.schematic.wires);
        let connected = net_graph.get_connected_wires(id);
        state.schematic.selection.clear();
        state.schematic.net_highlight.highlight_wires(connected);
        state.push_user_message(ConsoleMessage::info(format!(
            "Highlighted net with {} wires",
            state.schematic.net_highlight.highlighted_wires.len()
        )));
        ui.close_menu();
    }
    separator(ui);
    if edit_item(ui, ro, "Delete segment", Some("Del")) {
        state.schematic.delete_selection();
        ui.close_menu();
    }
}

fn canvas_menu(ui: &mut Ui, state: &mut AppState, click_pos: Point) {
    let ro = state.schematic.read_only;
    if edit_item(ui, ro, "Paste", Some("Ctrl+V")) {
        // Paste lands where the menu was summoned — the user already
        // pointed at the destination.
        state.schematic.paste_at(click_pos);
        ui.close_menu();
    }
    if item(ui, "Select all", Some("Ctrl+A")) {
        let schematic = &mut state.schematic;
        schematic.selection.clear();
        schematic.selection.components = schematic.components.iter().map(|c| c.id).collect();
        schematic.selection.wires = schematic.wires.iter().map(|w| w.id).collect();
        ui.close_menu();
    }
    separator(ui);
    if item(ui, "Zoom to fit", Some("F")) {
        state.schematic.needs_fit = true;
        ui.close_menu();
    }
    if item(ui, "Zoom 100 %", Some("Ctrl+0")) {
        state.schematic.zoom = 1.0;
        ui.close_menu();
    }
    separator(ui);
    if item(ui, "Run design checks", Some("Ctrl+E")) {
        crate::common::menu_bar::run_design_rule_check(state);
        ui.close_menu();
    }
}
