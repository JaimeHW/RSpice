use egui::{Response, Ui};

use crate::common::app::{AppState, ConsoleMessage, DragType};
use crate::state::{ComponentType, NetGraph, Point, Tool};

use super::SchematicSymbolContext;
use super::bus_interaction::{BusTapCandidateError, resolve_bus_tap_candidate};
use super::coordinates::{screen_to_grid, screen_to_schematic, screen_to_wire_grid};
use super::drawing::{bus_tap_at, nearest_bus_hit, nearest_terminal};
use super::viewport::Viewport;

pub(super) fn handle_tool_interactions(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    let grid_size = state.schematic.grid_size;
    let current_tool = state.schematic.tool;

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, grid_size, symbol_context);
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        match current_tool {
            // Read-only views take no edits; the console names the library.
            Tool::Place(_)
            | Tool::Wire
            | Tool::Bus
            | Tool::BusTap
            | Tool::Junction
            | Tool::Label
                if state.schematic.read_only =>
            {
                state.deny_read_only_edit();
            }
            Tool::Place(component_type) => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                place_component(state, component_type, grid_pos);
            }
            Tool::Wire => {
                let wire_pos = resolved_snap_position(
                    state,
                    symbol_context,
                    screen_to_wire_grid(viewport, grid_size, pos),
                );
                if state.schematic.wire_drawing.active {
                    state.schematic.extend_wire(wire_pos);
                } else {
                    state.schematic.start_wire(wire_pos);
                }
            }
            Tool::Bus => {
                let bus_pos = screen_to_wire_grid(viewport, grid_size, pos);
                if state.schematic.bus_drawing.active {
                    state.schematic.extend_bus(bus_pos);
                } else if let Err(error) = state.schematic.start_bus(bus_pos, None) {
                    report_bus_error(ui, state, "Bus could not be started", error.to_string());
                }
            }
            Tool::BusTap => {
                let requested = screen_to_schematic(viewport, pos);
                let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
                handle_bus_tap_click(ui, state, requested, hit_radius);
            }
            Tool::Junction => {
                let grid_pos = screen_to_wire_grid(viewport, grid_size, pos);
                handle_junction_click(ui, state, grid_pos);
            }
            Tool::Select => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                let hit_pos = screen_to_schematic(viewport, pos);
                let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
                handle_select_click(ui, state, grid_pos, hit_pos, hit_radius, symbol_context);
            }
            Tool::Probe => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                handle_probe_click(ui, state, grid_pos, symbol_context);
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

    if matches!(current_tool, Tool::Select)
        && response.double_clicked_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        let hit_pos = screen_to_schematic(viewport, pos);
        let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
        // Hierarchical instances descend on double-click (the Virtuoso
        // gesture); the breadcrumb pops back out. Everything else opens
        // its properties.
        let cell_instance = symbol_context
            .component_at_resolved_symbol(&state.schematic.components, grid_pos)
            .or_else(|| state.schematic.component_at(grid_pos))
            .and_then(|id| state.schematic.components.iter().find(|c| c.id == id))
            .filter(|c| c.kind == ComponentType::CellInstance)
            .map(|c| c.id);
        if let Some(id) = cell_instance {
            state.schematic.selection.clear();
            state.schematic.selection.select_component(id);
            state.open_selected_instance_master();
        } else {
            open_object_properties(state, grid_pos, hit_pos, hit_radius, symbol_context);
        }
    }

    if response.clicked_by(egui::PointerButton::Secondary) {
        if state.schematic.wire_drawing.active {
            state.schematic.finish_wire();
        } else if state.schematic.bus_drawing.active
            && let Err(error) = state.schematic.finish_bus()
        {
            report_bus_error(ui, state, "Bus could not be committed", error.to_string());
        }
    }
}

fn handle_bus_tap_click(ui: &Ui, state: &mut AppState, requested: Point, hit_radius: i32) {
    let candidate = match resolve_bus_tap_candidate(&state.schematic, requested, hit_radius) {
        Ok(candidate) => candidate,
        Err(error) => {
            report_bus_candidate_error(ui, state, error);
            return;
        }
    };
    let Some(pending) = state.schematic.pending_bus_tap.clone() else {
        report_bus_candidate_error(ui, state, BusTapCandidateError::MissingConfiguration);
        return;
    };
    let configured = crate::state::PendingBusTap {
        orientation: candidate.orientation,
        ..pending.clone()
    };
    match state.schematic.place_configured_bus_tap(
        candidate.bus_id,
        candidate.bus_point,
        candidate.connection_point,
        &configured,
    ) {
        Ok(_) => {
            let target = if pending.slice.is_scalar() {
                format!("scalar net {}", pending.slice)
            } else {
                format!("bus slice {}", pending.slice)
            };
            let message = format!(
                "Placed {target} from ({}, {}) to ({}, {}).",
                candidate.bus_point.x,
                candidate.bus_point.y,
                candidate.connection_point.x,
                candidate.connection_point.y
            );
            state
                .ui
                .toasts
                .success(ui.ctx(), "Bus tap placed", message.clone());
            state.push_user_message(ConsoleMessage::info(message));
        }
        Err(error) => report_bus_error(ui, state, "Bus tap rejected", error.to_string()),
    }
}

fn report_bus_candidate_error(ui: &Ui, state: &mut AppState, error: BusTapCandidateError) {
    report_bus_error(ui, state, "Invalid bus-tap target", error.message());
}

fn report_bus_error(ui: &Ui, state: &mut AppState, title: &str, message: String) {
    state
        .ui
        .toasts
        .warn_with_title(ui.ctx(), title, message.clone());
    state.push_user_message(ConsoleMessage::warning(message));
}

fn resolved_snap_position(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    grid_pos: Point,
) -> Point {
    state
        .schematic
        .snap_engine
        .find_snap_target_resolved(
            grid_pos,
            &state.schematic.components,
            &state.schematic.wires,
            &state.schematic.junctions,
            |component| symbol_context.resolved_symbol(component),
        )
        .snapped_position
}

fn handle_select_dragging(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
    symbol_context: &SchematicSymbolContext,
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
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        let wire_grid_pos = screen_to_wire_grid(viewport, grid_size, pos);
        let hit_pos = screen_to_schematic(viewport, pos);
        let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
        let target = pointer_target(state, grid_pos, hit_pos, hit_radius, symbol_context);

        if !select_drag_can_start(ui.input(|i| i.modifiers.shift)) {
            return;
        } else if state.schematic.read_only {
            // No moves on read-only views — every drag is a marquee.
            state.schematic.selection_rect.start_at(grid_pos);
        } else {
            match target {
                Some(PointerTarget::Component(id)) => {
                    if !state.schematic.selection.has_component(id) {
                        state.schematic.selection.clear();
                        state.schematic.selection.select_component(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::BusTap(id)) => {
                    if !state.schematic.selection.has_bus_tap(id) {
                        state.schematic.selection.select_only_bus_tap(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::Junction(_))
                    if state.schematic.is_draggable_wire_point(wire_grid_pos) =>
                {
                    start_wire_vertex_drag(state, wire_grid_pos);
                }
                Some(PointerTarget::Bus(id)) => {
                    if !state.schematic.selection.has_bus(id) {
                        state.schematic.selection.select_only_bus(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::Wire(_))
                    if state.schematic.is_draggable_wire_point(wire_grid_pos) =>
                {
                    start_wire_vertex_drag(state, wire_grid_pos);
                }
                _ => state.schematic.selection_rect.start_at(grid_pos),
            }
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
            let delta = Point::new(
                grid_pos.x.saturating_sub(last_x),
                grid_pos.y.saturating_sub(last_y),
            );

            if delta.x != 0 || delta.y != 0 {
                state
                    .schematic
                    .move_selection_with_rubber_band_resolved(delta, |component| {
                        symbol_context.terminal_points(component)
                    });
                state.dialogs.last_drag_pos = Some((grid_pos.x, grid_pos.y));
            }
        } else if state.schematic.selection_rect.is_active() {
            state.schematic.selection_rect.update(grid_pos);
        }
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) {
        if state.dialogs.interaction.vertex_drag_pos.is_some() {
            let automatic_junctions = state
                .schematic
                .document_policy
                .wire_junctions
                .automatic_junctions();
            state
                .schematic
                .cleanup_wire_topology_with_junction_policy(automatic_junctions);
            // One undo entry for the whole gesture (no-ops deduplicate).
            state.schematic.end_operation();
            state.dialogs.interaction.vertex_drag_pos = None;
            state.dialogs.interaction.drag.cancel();
        } else if state.dialogs.last_drag_pos.is_some() {
            let automatic_junctions = state
                .schematic
                .document_policy
                .wire_junctions
                .automatic_junctions();
            state
                .schematic
                .cleanup_wire_topology_with_junction_policy(automatic_junctions);
            state.schematic.end_operation();
            state.dialogs.drag_start = None;
            state.dialogs.last_drag_pos = None;
        } else {
            let left_to_right =
                state.schematic.selection_rect.current.x >= state.schematic.selection_rect.start.x;
            let Some((min_x, min_y, max_x, max_y)) = state.schematic.selection_rect.finish() else {
                return;
            };
            let add_mode = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);
            let enclosed_only = state
                .schematic
                .document_policy
                .selection_crossing
                .enclosed_only(left_to_right);
            symbol_context.select_in_rect(
                &mut state.schematic,
                super::SelectionWindow::new(min_x, min_y, max_x, max_y, enclosed_only),
                add_mode,
            );
        }
    }
}

fn start_wire_vertex_drag(state: &mut AppState, position: Point) {
    state.schematic.begin_operation("drag wire vertex");
    state.dialogs.interaction.vertex_drag_pos = Some((position.x, position.y));
    state
        .dialogs
        .interaction
        .drag
        .start((position.x, position.y), DragType::WireVertex);
}

fn start_selection_drag(state: &mut AppState, position: Point) {
    state.schematic.begin_operation("move selection");
    state.dialogs.drag_start = Some((position.x, position.y));
    state.dialogs.last_drag_pos = Some((position.x, position.y));
}

fn select_drag_can_start(shift_pressed: bool) -> bool {
    !shift_pressed
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
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        }
    } else {
        state.schematic.add_component(component_type, grid_pos);
        log::info!("Placed {:?} at {:?}", component_type, grid_pos);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JunctionPlacementOutcome {
    Placed(Point),
    Removed(Point),
    NoIntersection,
    MixedBus,
}

fn commit_explicit_junction(state: &mut AppState, requested: Point) -> JunctionPlacementOutcome {
    let Some(target) = state
        .schematic
        .nearest_junction_candidate(requested, state.schematic.grid_size)
    else {
        return JunctionPlacementOutcome::NoIntersection;
    };

    if state
        .schematic
        .buses
        .iter()
        .any(|bus| bus.contains_point(target))
    {
        return JunctionPlacementOutcome::MixedBus;
    }

    if let Some(junction_id) = state.schematic.junction_at(target) {
        state.schematic.with_undo("remove junction", |schematic| {
            schematic.remove_junction(junction_id);
        });
        state.schematic.net_highlight.clear();
        return JunctionPlacementOutcome::Removed(target);
    }

    state.schematic.with_undo("place junction", |schematic| {
        schematic.add_junction(target);
    });
    state.schematic.net_highlight.clear();
    JunctionPlacementOutcome::Placed(target)
}

fn handle_junction_click(ui: &Ui, state: &mut AppState, requested: Point) {
    let (title, message, warning) = match commit_explicit_junction(state, requested) {
        JunctionPlacementOutcome::Placed(point) => (
            "Junction placed",
            format!("Added an explicit junction at ({}, {}).", point.x, point.y),
            false,
        ),
        JunctionPlacementOutcome::Removed(point) => (
            "Junction removed",
            format!(
                "Removed the explicit junction at ({}, {}).",
                point.x, point.y
            ),
            false,
        ),
        JunctionPlacementOutcome::NoIntersection => (
            "No conductor intersection",
            "Move the pointer to a point where two conductors meet.".to_owned(),
            true,
        ),
        JunctionPlacementOutcome::MixedBus => (
            "Mixed scalar/bus junction rejected",
            "Use a typed bus tap; explicit junctions cannot connect scalar wires to buses."
                .to_owned(),
            true,
        ),
    };

    if warning {
        state
            .ui
            .toasts
            .warn_with_title(ui.ctx(), title, message.clone());
        state.push_user_message(ConsoleMessage::warning(message));
    } else {
        state.ui.toasts.success(ui.ctx(), title, message.clone());
        state.push_user_message(ConsoleMessage::info(message));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerTarget {
    Component(u64),
    BusTap(u64),
    Junction(Point),
    Bus(u64),
    Wire(u64),
}

fn pointer_target(
    state: &AppState,
    grid_pos: Point,
    hit_pos: Point,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
) -> Option<PointerTarget> {
    symbol_context
        .component_at_resolved_symbol(&state.schematic.components, grid_pos)
        .or_else(|| state.schematic.component_at(grid_pos))
        .map(PointerTarget::Component)
        .or_else(|| {
            bus_tap_at(&state.schematic.bus_taps, hit_pos, hit_radius).map(PointerTarget::BusTap)
        })
        .or_else(|| {
            state
                .schematic
                .junction_at(grid_pos)
                .map(|_| PointerTarget::Junction(grid_pos))
        })
        .or_else(|| {
            nearest_bus_hit(&state.schematic.buses, hit_pos, hit_radius)
                .map(|hit| PointerTarget::Bus(hit.bus_id))
        })
        .or_else(|| state.schematic.wire_at(grid_pos).map(PointerTarget::Wire))
}

fn handle_select_click(
    ui: &Ui,
    state: &mut AppState,
    grid_pos: Point,
    hit_pos: Point,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
) {
    // Ctrl and Shift both extend the selection (toggle the clicked item);
    // a plain click replaces it.
    let additive = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);
    let alt_held = ui.input(|i| i.modifiers.alt);

    match pointer_target(state, grid_pos, hit_pos, hit_radius, symbol_context) {
        Some(PointerTarget::Component(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_component(id);
            } else {
                state.schematic.selection.clear();
                state.schematic.selection.select_component(id);
            }
        }
        Some(PointerTarget::BusTap(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_bus_tap(id);
            } else {
                state.schematic.selection.select_only_bus_tap(id);
            }
        }
        Some(PointerTarget::Junction(pos)) => {
            state.schematic.net_highlight.clear();
            if additive {
                if state.schematic.selection.has_junction(pos) {
                    state.schematic.selection.deselect_junction(pos);
                } else {
                    state.schematic.selection.select_junction(pos);
                }
            } else {
                state.schematic.selection.select_only_junction(pos);
            }
        }
        Some(PointerTarget::Bus(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_bus(id);
            } else {
                state.schematic.selection.select_only_bus(id);
            }
        }
        Some(PointerTarget::Wire(id)) => {
            if alt_held {
                let net_graph = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
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
            } else if additive {
                state.schematic.net_highlight.clear();
                state.schematic.selection.toggle_wire(id);
            } else {
                state.schematic.net_highlight.clear();
                state.schematic.selection.clear();
                state.schematic.selection.select_wire(id);
            }
        }
        None if !additive => {
            state.schematic.selection.clear();
            state.schematic.net_highlight.clear();
        }
        None => {}
    }
}

/// Whether the named waveform is currently plotted.
fn waveform_visible(state: &AppState, name: &str) -> bool {
    state
        .simulation
        .waveforms
        .iter()
        .find(|waveform| waveform.name == name)
        .map(|waveform| waveform.visible)
        .unwrap_or(false)
}

/// Toggle a probed waveform and confirm via toast: plotted / hidden / not
/// available yet. The console gets the same line for the record.
fn toggle_probe_with_feedback(ui: &Ui, state: &mut AppState, name: &str, display: &str) {
    let toggled = state.simulation.toggle_waveform_visibility(name);
    if toggled {
        let visible = waveform_visible(state, name);
        let message = if visible {
            format!("{display} added to plot")
        } else {
            format!("{display} removed from plot")
        };
        state.ui.toasts.success(
            ui.ctx(),
            if visible {
                "Trace shown"
            } else {
                "Trace hidden"
            },
            format!("{message}."),
        );
        state.push_user_message(ConsoleMessage::info(message));
    } else {
        let message = format!("No waveform for {display} — run the simulation first");
        state
            .ui
            .toasts
            .warn_with_title(ui.ctx(), "Waveform unavailable", format!("{message}."));
        state.push_user_message(ConsoleMessage::warning(message));
    }
}

fn handle_probe_click(
    ui: &Ui,
    state: &mut AppState,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    if let Some(_wire_id) = state.schematic.wire_at(grid_pos) {
        if let Some(net_name) = state.simulation.cross_probe.net_at(grid_pos) {
            let net_name = net_name.clone();
            log::info!("Probe: clicked net '{}' at {:?}", net_name, grid_pos);

            if net_name == "0" {
                state.ui.toasts.success(
                    ui.ctx(),
                    "Ground reference selected",
                    "Node 0 is the 0 V reference.",
                );
                state.push_user_message(ConsoleMessage::info(
                    "Ground node: 0V reference".to_string(),
                ));
            } else {
                let display = format!("V({net_name})");
                toggle_probe_with_feedback(ui, state, &net_name, &display);

                if state
                    .ui
                    .preferences
                    .toggle(crate::workbench::TogglePreference::CrossProbeBehavior)
                {
                    let net_graph =
                        NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
                    state
                        .schematic
                        .net_highlight
                        .highlight_net(&net_graph, grid_pos);
                }
            }
        } else {
            log::info!(
                "Probe: wire at {:?} not in netlist (regenerate netlist?)",
                grid_pos
            );
            state.ui.toasts.warn_with_title(
                ui.ctx(),
                "Wire is not in the netlist",
                "Wire not in the netlist — run the simulation to update",
            );
            state.push_user_message(ConsoleMessage::warning(
                "Wire not in netlist. Run simulation to update.".to_string(),
            ));
        }
    } else if let Some(comp_id) = symbol_context
        .component_at_resolved_symbol(&state.schematic.components, grid_pos)
        .or_else(|| state.schematic.component_at(grid_pos))
    {
        handle_component_probe(ui, state, comp_id, grid_pos, symbol_context);
    } else {
        state.schematic.net_highlight.clear();
        log::debug!("Probe: clicked empty space at {:?}", grid_pos);
    }
}

fn handle_component_probe(
    ui: &Ui,
    state: &mut AppState,
    comp_id: u64,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
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
            let terminals =
                component.terminal_positions_resolved(symbol_context.resolved_symbol(component));
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

        let display = probe_name.clone();
        toggle_probe_with_feedback(ui, state, &probe_name, &display);
    }
}

fn open_object_properties(
    state: &mut AppState,
    grid_pos: Point,
    hit_pos: Point,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
) {
    match pointer_target(state, grid_pos, hit_pos, hit_radius, symbol_context) {
        Some(PointerTarget::Component(id)) => state.schematic.selection.select_only_component(id),
        Some(PointerTarget::BusTap(id)) => state.schematic.selection.select_only_bus_tap(id),
        Some(PointerTarget::Bus(id)) => state.schematic.selection.select_only_bus(id),
        Some(PointerTarget::Junction(_)) | Some(PointerTarget::Wire(_)) | None => return,
    }
    crate::common::app::open_selected_object_properties(state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
        Junction, Wire,
    };

    #[test]
    fn shift_primary_drag_is_reserved_for_pan() {
        assert!(select_drag_can_start(false));
        assert!(!select_drag_can_start(true));
    }

    #[test]
    fn click_and_drag_share_one_overlapping_object_priority() {
        let point = Point::new(10, 0);
        let bus = Bus::segment(
            20,
            Point::new(0, 0),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            21,
            &bus,
            point,
            Point::new(10, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut state = AppState::default();
        state
            .schematic
            .components
            .push(Component::new(10, ComponentType::Resistor, point));
        state
            .schematic
            .wires
            .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
        state.schematic.junctions.push(Junction::new(12, point));
        state.schematic.buses.push(bus);
        state.schematic.bus_taps.push(tap);
        let context = SchematicSymbolContext::default();

        assert_eq!(
            pointer_target(&state, point, point, 1, &context),
            Some(PointerTarget::Component(10))
        );
        state.schematic.components.clear();
        assert_eq!(
            pointer_target(&state, point, point, 1, &context),
            Some(PointerTarget::BusTap(21))
        );
        state.schematic.bus_taps.clear();
        assert_eq!(
            pointer_target(&state, point, point, 1, &context),
            Some(PointerTarget::Junction(point))
        );
        state.schematic.junctions.clear();
        assert_eq!(
            pointer_target(&state, point, point, 1, &context),
            Some(PointerTarget::Bus(20))
        );
        state.schematic.buses.clear();
        assert_eq!(
            pointer_target(&state, point, point, 1, &context),
            Some(PointerTarget::Wire(11))
        );
    }

    #[test]
    fn double_click_property_dispatch_selects_taps_before_their_source_bus() {
        let mut state = AppState::default();
        let bus = Bus::segment(
            20,
            Point::new(0, 0),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            21,
            &bus,
            Point::new(10, 0),
            Point::new(10, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state.schematic.buses.push(bus);
        state.schematic.bus_taps.push(tap);
        let symbol_context = SchematicSymbolContext::from_state(&state);

        open_object_properties(
            &mut state,
            Point::new(10, 0),
            Point::new(10, 0),
            1,
            &symbol_context,
        );

        assert_eq!(state.schematic.selection.single_bus_tap(), Some(21));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(crate::common::app::ObjectPropertiesDraft::BusTap(_))
        ));
    }

    #[test]
    fn explicit_junction_placement_requires_two_wires_and_is_one_undo_step() {
        let mut state = AppState::default();
        state.schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
        ];
        state.schematic.bump_topology_version();
        state
            .schematic
            .net_highlight
            .highlight_wires([1].into_iter().collect());

        assert_eq!(
            commit_explicit_junction(&mut state, Point::new(20, 20)),
            JunctionPlacementOutcome::Placed(Point::new(20, 20))
        );
        assert!(state.schematic.has_junction(Point::new(20, 20)));
        assert!(!state.schematic.net_highlight.active);
        assert!(state.schematic.net_highlight.highlighted_wires.is_empty());
        assert!(state.schematic.can_undo());
        assert!(state.schematic.undo());
        assert!(!state.schematic.has_junction(Point::new(20, 20)));

        assert_eq!(
            commit_explicit_junction(&mut state, Point::new(100, 100)),
            JunctionPlacementOutcome::NoIntersection
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn clicking_an_existing_junction_removes_it_as_one_undo_step() {
        let mut state = AppState::default();
        state.schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
        ];
        state.schematic.add_junction(Point::new(20, 20));
        state
            .schematic
            .net_highlight
            .highlight_wires([1, 2].into_iter().collect());

        assert_eq!(
            commit_explicit_junction(&mut state, Point::new(20, 20)),
            JunctionPlacementOutcome::Removed(Point::new(20, 20))
        );
        assert!(!state.schematic.has_junction(Point::new(20, 20)));
        assert!(!state.schematic.net_highlight.active);
        assert!(state.schematic.net_highlight.highlighted_wires.is_empty());
        let disconnected = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
        assert_eq!(
            disconnected.get_connected_wires(1),
            [1].into_iter().collect()
        );
        assert_eq!(
            disconnected.get_connected_wires(2),
            [2].into_iter().collect()
        );
        assert!(state.schematic.can_undo());
        assert!(state.schematic.undo());
        assert!(state.schematic.has_junction(Point::new(20, 20)));
        let connected = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
        assert_eq!(
            connected.get_connected_wires(1),
            [1, 2].into_iter().collect()
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn automatic_t_marker_is_not_an_explicit_junction_toggle_target() {
        let point = Point::new(20, 20);
        let mut state = AppState::default();
        state.schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![point, Point::new(20, 40)]),
        ];
        state.schematic.add_junction(point);

        assert_eq!(
            commit_explicit_junction(&mut state, point),
            JunctionPlacementOutcome::NoIntersection
        );
        assert!(state.schematic.has_junction(point));
        assert!(!state.schematic.can_undo());
    }
}
