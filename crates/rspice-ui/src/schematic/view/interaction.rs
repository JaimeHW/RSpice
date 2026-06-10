use egui::{Response, Ui};

use crate::common::app::{AppState, ConsoleMessage, DragType};
use crate::state::{ComponentType, NetGraph, Point, Tool};

use super::coordinates::{screen_to_grid, screen_to_wire_grid};
use super::drawing::nearest_terminal;
use super::viewport::Viewport;

pub(super) fn handle_tool_interactions(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    let grid_size = state.schematic.grid_size;
    let current_tool = state.schematic.tool;

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, grid_size);
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        match current_tool {
            Tool::Place(component_type) => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                place_component(state, component_type, grid_pos);
            }
            Tool::Wire => {
                let wire_pos = screen_to_wire_grid(viewport, grid_size, pos);
                if state.schematic.wire_drawing.active {
                    state.schematic.extend_wire(wire_pos);
                } else {
                    state.schematic.start_wire(wire_pos);
                }
            }
            Tool::Select => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                handle_select_click(ui, state, grid_pos);
            }
            Tool::Probe => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                handle_probe_click(state, grid_pos);
            }
            Tool::Label => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                let name = format!("net{}", state.schematic.net_labels.len() + 1);
                state.schematic.with_undo("place net label", |schematic| {
                    schematic.add_net_label(grid_pos, name);
                });
                state.schematic.is_dirty = true;
            }
        }
    }

    if response.double_clicked_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        open_component_properties(state, grid_pos);
    }

    if response.clicked_by(egui::PointerButton::Secondary) && state.schematic.wire_drawing.active {
        state.schematic.finish_wire();
    }
}

fn handle_select_dragging(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
) {
    if let Some(pos) = response.hover_pos() {
        let wire_grid_pos = screen_to_wire_grid(viewport, grid_size, pos);
        if state.schematic.is_draggable_wire_point(wire_grid_pos) {
            state.dialogs.interaction.hover_wire_vertex = Some((wire_grid_pos.x, wire_grid_pos.y));
        } else {
            state.dialogs.interaction.hover_wire_vertex = None;
        }
    } else {
        state.dialogs.interaction.hover_wire_vertex = None;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && !ui.input(|i| i.modifiers.shift)
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        let wire_grid_pos = screen_to_wire_grid(viewport, grid_size, pos);

        if state.schematic.is_draggable_wire_point(wire_grid_pos) {
            state.schematic.begin_operation("drag wire vertex");
            state.dialogs.interaction.vertex_drag_pos = Some((wire_grid_pos.x, wire_grid_pos.y));
            state
                .dialogs
                .interaction
                .drag
                .start((wire_grid_pos.x, wire_grid_pos.y), DragType::WireVertex);
        } else if let Some(comp_id) = state.schematic.component_at(grid_pos) {
            if !state.schematic.selection.has_component(comp_id) {
                state.schematic.selection.clear();
                state.schematic.selection.select_component(comp_id);
            }
            state.schematic.begin_operation("move selection");
            state.dialogs.drag_start = Some((grid_pos.x, grid_pos.y));
            state.dialogs.last_drag_pos = Some((grid_pos.x, grid_pos.y));
        } else {
            state.schematic.selection_rect.start_at(grid_pos);
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(pos) = response.hover_pos()
    {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        let wire_grid_pos = screen_to_wire_grid(viewport, grid_size, pos);

        if let Some((old_x, old_y)) = state.dialogs.interaction.vertex_drag_pos {
            let old_pos = Point::new(old_x, old_y);
            if state.schematic.move_all_vertices_at(old_pos, wire_grid_pos) {
                state.dialogs.interaction.vertex_drag_pos =
                    Some((wire_grid_pos.x, wire_grid_pos.y));
                state
                    .dialogs
                    .interaction
                    .drag
                    .update((wire_grid_pos.x, wire_grid_pos.y));
            }
        } else if let Some((last_x, last_y)) = state.dialogs.last_drag_pos {
            let delta = Point::new(grid_pos.x - last_x, grid_pos.y - last_y);

            if delta.x != 0 || delta.y != 0 {
                state.schematic.move_selection_with_rubber_band(delta);
                state.dialogs.last_drag_pos = Some((grid_pos.x, grid_pos.y));
            }
        } else if state.schematic.selection_rect.is_active() {
            state.schematic.selection_rect.update(grid_pos);
        }
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) {
        if state.dialogs.interaction.vertex_drag_pos.is_some() {
            state.schematic.cleanup_wire_topology();
            // One undo entry for the whole gesture (no-ops deduplicate).
            state.schematic.end_operation();
            state.dialogs.interaction.vertex_drag_pos = None;
            state.dialogs.interaction.drag.cancel();
        } else if state.dialogs.last_drag_pos.is_some() {
            state.schematic.cleanup_wire_topology();
            state.schematic.end_operation();
            state.dialogs.drag_start = None;
            state.dialogs.last_drag_pos = None;
        } else if let Some((min_x, min_y, max_x, max_y)) = state.schematic.selection_rect.finish() {
            let add_mode = ui.input(|i| i.modifiers.ctrl);
            state
                .schematic
                .select_in_rect(min_x, min_y, max_x, max_y, add_mode);
        }
    }
}

fn place_component(state: &mut AppState, component_type: ComponentType, grid_pos: Point) {
    if component_type == ComponentType::CellInstance {
        if let Some(library_cell) = state.schematic.pending_library_cell.clone() {
            state
                .schematic
                .add_library_cell_component(grid_pos, library_cell);
            log::info!("Placed library cell instance at {:?}", grid_pos);
        } else {
            state.push_user_message(ConsoleMessage::warning(
                "No library cell selected for placement".to_string(),
            ));
            state.schematic.tool = Tool::Select;
        }
    } else {
        state.schematic.add_component(component_type, grid_pos);
        log::info!("Placed {:?} at {:?}", component_type, grid_pos);
    }
}

fn handle_select_click(ui: &Ui, state: &mut AppState, grid_pos: Point) {
    let ctrl_held = ui.input(|i| i.modifiers.ctrl);
    let alt_held = ui.input(|i| i.modifiers.alt);

    let comp_id = state.schematic.component_at(grid_pos);
    let wire_id = state.schematic.wire_at(grid_pos);

    if let Some(id) = comp_id {
        state.schematic.net_highlight.clear();
        if ctrl_held {
            state.schematic.selection.toggle_component(id);
        } else {
            state.schematic.selection.clear();
            state.schematic.selection.select_component(id);
        }
    } else if let Some(id) = wire_id {
        if alt_held {
            let net_graph = NetGraph::build_from_wires(&state.schematic.wires);
            let connected_wires = net_graph.get_connected_wires(id);

            state.schematic.selection.clear();
            state
                .schematic
                .net_highlight
                .highlight_wires(connected_wires);
            log::info!(
                "Highlighted net with {} wires",
                state.schematic.net_highlight.highlighted_wires.len()
            );
        } else if ctrl_held {
            state.schematic.net_highlight.clear();
            state.schematic.selection.toggle_wire(id);
        } else {
            state.schematic.net_highlight.clear();
            state.schematic.selection.clear();
            state.schematic.selection.select_wire(id);
        }
    } else if !ctrl_held {
        state.schematic.selection.clear();
        state.schematic.net_highlight.clear();
    }
}

fn handle_probe_click(state: &mut AppState, grid_pos: Point) {
    if let Some(_wire_id) = state.schematic.wire_at(grid_pos) {
        if let Some(net_name) = state.simulation.cross_probe.net_at(grid_pos) {
            let net_name = net_name.clone();
            log::info!("Probe: clicked net '{}' at {:?}", net_name, grid_pos);

            if net_name == "0" {
                log::info!("Probe: ground net (0V reference)");
                state.push_user_message(ConsoleMessage::info(
                    "Ground node: 0V reference".to_string(),
                ));
            } else {
                let toggled = state.simulation.toggle_waveform_visibility(&net_name);

                if toggled {
                    log::info!("Probe: toggled waveform for net '{}'", net_name);
                    state.push_user_message(ConsoleMessage::info(format!(
                        "Toggled waveform: V({})",
                        net_name
                    )));
                } else {
                    log::info!(
                        "Probe: no waveform found for net '{}' (run simulation first?)",
                        net_name
                    );
                    state.push_user_message(ConsoleMessage::warning(format!(
                        "No waveform for {}: run simulation first",
                        net_name
                    )));
                }

                let net_graph = NetGraph::build_from_wires(&state.schematic.wires);
                state
                    .schematic
                    .net_highlight
                    .highlight_net(&net_graph, grid_pos);
            }
        } else {
            log::info!(
                "Probe: wire at {:?} not in netlist (regenerate netlist?)",
                grid_pos
            );
            state.push_user_message(ConsoleMessage::warning(
                "Wire not in netlist. Run simulation to update.".to_string(),
            ));
        }
    } else if let Some(comp_id) = state.schematic.component_at(grid_pos) {
        handle_component_probe(state, comp_id, grid_pos);
    } else {
        state.schematic.net_highlight.clear();
        log::debug!("Probe: clicked empty space at {:?}", grid_pos);
    }
}

fn handle_component_probe(state: &mut AppState, comp_id: u64, grid_pos: Point) {
    if let Some(component) = state.schematic.components.iter().find(|c| c.id == comp_id) {
        let comp_name = component.name.clone();
        log::info!(
            "Probe: clicked component '{}' ({})",
            comp_name,
            component.kind.display_name()
        );

        let probe_name = if component.kind.spice_prefix() == "V" {
            format!("I(V{})", comp_name)
        } else {
            let terminals = component.terminal_positions();
            if let Some((_, term_pos)) = nearest_terminal(&terminals, grid_pos) {
                if let Some(net_name) = state.simulation.cross_probe.net_at(term_pos) {
                    format!("V({})", net_name)
                } else {
                    format!("V({})", comp_name)
                }
            } else {
                format!("V({})", comp_name)
            }
        };

        let toggled = state.simulation.toggle_waveform_visibility(&probe_name);
        if toggled {
            state.push_user_message(ConsoleMessage::info(format!(
                "Toggled waveform: {}",
                probe_name
            )));
        } else {
            state.push_user_message(ConsoleMessage::warning(format!(
                "No waveform for {}",
                probe_name
            )));
        }
    }
}

fn open_component_properties(state: &mut AppState, grid_pos: Point) {
    if let Some(comp_id) = state.schematic.component_at(grid_pos) {
        state.schematic.selection.clear();
        state.schematic.selection.select_component(comp_id);

        if let Some(component) = state.schematic.components.iter().find(|c| c.id == comp_id) {
            let component_type = component.kind;
            let properties = crate::properties::property_bridge::collect_properties_from_component(
                component,
                &state.property_registry,
            );

            if let Some(sheet) = state.property_registry.get(component_type) {
                state.tabbed_property_dialog.open_for_component(
                    comp_id,
                    &component.name,
                    component_type,
                    sheet,
                    properties,
                );
                log::info!(
                    "Double-clicked component {} ({:?}), opening properties dialog",
                    comp_id,
                    component_type
                );
            } else {
                log::warn!("No property sheet found for {:?}", component_type);
            }
        }

    }
}
