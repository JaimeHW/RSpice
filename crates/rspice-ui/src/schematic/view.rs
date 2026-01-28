//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use egui::{Color32, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::common::app::AppState;
use crate::state::{ComponentType, Point, Rotation};

use super::symbols::SymbolLibrary;

/// Render the schematic view (central canvas)
pub fn render_schematic_view(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    // Get available space
    let available = ui.available_rect_before_wrap();

    // Handle deferred zoom-to-fit (uses actual canvas dimensions)
    if state.schematic.needs_fit {
        state.schematic.needs_fit = false;
        state
            .schematic
            .zoom_to_fit(available.width() as f64, available.height() as f64);
    }

    // Create a sense for interaction
    let response = ui.allocate_rect(available, Sense::click_and_drag());

    // Get the painter for drawing
    let painter = ui.painter_at(available);

    // Draw background
    painter.rect_filled(available, 0.0, state.theme.canvas_bg);

    // Draw grid
    draw_grid(&painter, available, state);

    // Apply viewport transform
    let viewport = Viewport {
        offset: Pos2::new(state.schematic.pan.0 as f32, state.schematic.pan.1 as f32),
        zoom: state.schematic.zoom as f32,
        bounds: available,
    };

    // Get preview selection bounds if a selection rect is active (for live preview highlighting)
    let preview_bounds = if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

    // Draw wires
    for wire in &state.schematic.wires {
        let mut is_selected = state.schematic.selection.wires.contains(&wire.id);

        // Live preview: if dragging selection rect, highlight wires inside the rect
        if !is_selected {
            if let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
                // Check if any point of the wire is inside the selection rect
                is_selected = wire
                    .points
                    .iter()
                    .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
            }
        }

        let is_highlighted = state.schematic.net_highlight.is_wire_highlighted(wire.id);
        draw_wire(
            &painter,
            &viewport,
            wire,
            is_selected,
            is_highlighted,
            state,
        );
    }

    // Draw components
    for component in &state.schematic.components {
        let mut is_selected = state.schematic.selection.components.contains(&component.id);

        // Live preview: if dragging selection rect, highlight components inside the rect
        if !is_selected {
            if let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
                // Check if component center is inside the selection rect
                is_selected = component.pos.x >= min_x
                    && component.pos.x <= max_x
                    && component.pos.y >= min_y
                    && component.pos.y <= max_y;
            }
        }

        draw_component(
            &painter,
            &viewport,
            component,
            is_selected,
            state,
            symbol_library,
        );
    }

    // Draw junctions
    for junction in &state.schematic.junctions {
        draw_junction(&painter, &viewport, junction.pos, state);
    }

    // Draw hover highlight for wire vertices (if not already a junction)
    // This provides visual feedback that wire corners/endpoints can be dragged
    if let Some((hx, hy)) = state.dialogs.interaction.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        // Only draw if not already a junction (junctions draw their own highlight)
        let is_junction = state.schematic.junctions.iter().any(|j| j.pos == hover_pos);
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                egui::Stroke::new(1.0 * viewport.zoom, egui::Color32::from_rgb(100, 200, 255)),
            );
        }
    }

    // Handle interactions
    if response.dragged_by(egui::PointerButton::Middle)
        || (response.dragged_by(egui::PointerButton::Primary) && ui.input(|i| i.modifiers.shift))
    {
        // Pan with middle mouse or shift+left
        let delta = response.drag_delta();
        state.schematic.pan.0 += delta.x as f64;
        state.schematic.pan.1 += delta.y as f64;
    }

    // Zoom with scroll wheel - CURSOR-CENTERED ZOOM (like professional CAD tools)
    if response.hovered() {
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            // Get cursor position
            if let Some(cursor_pos) = response.hover_pos() {
                let zoom_factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
                let old_zoom = state.schematic.zoom;
                let new_zoom = (old_zoom * zoom_factor as f64).clamp(0.1, 10.0);

                // Calculate the schematic coordinate under the cursor (before zoom)
                // screen_pos = bounds.min + pan + schematic_pos * zoom
                // schematic_pos = (screen_pos - bounds.min - pan) / zoom
                let cursor_schematic_x =
                    (cursor_pos.x as f64 - available.min.x as f64 - state.schematic.pan.0)
                        / old_zoom;
                let cursor_schematic_y =
                    (cursor_pos.y as f64 - available.min.y as f64 - state.schematic.pan.1)
                        / old_zoom;

                // Apply zoom
                state.schematic.zoom = new_zoom;

                // Adjust pan so the schematic point under cursor stays at the same screen position
                // new_screen_pos = bounds.min + new_pan + schematic_pos * new_zoom
                // We want new_screen_pos == cursor_pos, so:
                // new_pan = cursor_pos - bounds.min - schematic_pos * new_zoom
                state.schematic.pan.0 =
                    cursor_pos.x as f64 - available.min.x as f64 - cursor_schematic_x * new_zoom;
                state.schematic.pan.1 =
                    cursor_pos.y as f64 - available.min.y as f64 - cursor_schematic_y * new_zoom;
            }
        }
    }

    // ==========================================================================
    // Tool-based interactions (placing, wiring, selecting)
    // ==========================================================================
    use crate::state::Point;
    use crate::state::Tool;

    // Create fresh viewport AFTER pan/zoom changes for tool interactions
    let tool_viewport = Viewport {
        offset: Pos2::new(state.schematic.pan.0 as f32, state.schematic.pan.1 as f32),
        zoom: state.schematic.zoom as f32,
        bounds: available,
    };
    let grid_size = state.schematic.grid_size;

    // Convert screen position to snapped grid position (full grid - for components)
    let screen_to_grid = |screen_pos: Pos2| -> Point {
        let zoom = tool_viewport.zoom as f64;
        let pan_x = tool_viewport.offset.x as f64;
        let pan_y = tool_viewport.offset.y as f64;
        let bounds_min_x = tool_viewport.bounds.min.x as f64;
        let bounds_min_y = tool_viewport.bounds.min.y as f64;

        let schematic_x = (screen_pos.x as f64 - bounds_min_x - pan_x) / zoom;
        let schematic_y = (screen_pos.y as f64 - bounds_min_y - pan_y) / zoom;
        // Snap to full grid
        let grid_x = (schematic_x / grid_size as f64).round() as i32;
        let grid_y = (schematic_y / grid_size as f64).round() as i32;
        Point::new(grid_x * grid_size as i32, grid_y * grid_size as i32)
    };

    // Convert screen position to grid snapped position (for wires)
    // Commercial-grade: wires snap to FULL grid (same as components) for consistent alignment
    // This ensures all connection points are grid-aligned (Cadence Virtuoso standard)
    let screen_to_wire_grid = |screen_pos: Pos2| -> Point {
        let zoom = tool_viewport.zoom as f64;
        let pan_x = tool_viewport.offset.x as f64;
        let pan_y = tool_viewport.offset.y as f64;
        let bounds_min_x = tool_viewport.bounds.min.x as f64;
        let bounds_min_y = tool_viewport.bounds.min.y as f64;

        let schematic_x = (screen_pos.x as f64 - bounds_min_x - pan_x) / zoom;
        let schematic_y = (screen_pos.y as f64 - bounds_min_y - pan_y) / zoom;
        // Snap to full grid (same as components) for commercial-grade alignment
        let grid_x = (schematic_x / grid_size as f64).round() as i32;
        let grid_y = (schematic_y / grid_size as f64).round() as i32;
        Point::new(grid_x * grid_size as i32, grid_y * grid_size as i32)
    };

    // Left click handling - now includes drag detection for box selection
    let current_tool = state.schematic.tool; // Copy to avoid borrow conflict

    // Handle box selection drag for Select tool
    if matches!(current_tool, Tool::Select) {
        // Detect hover over wire vertices for visual feedback
        if let Some(pos) = response.hover_pos() {
            let wire_grid_pos = screen_to_wire_grid(pos);
            if state.schematic.wire_vertex_at(wire_grid_pos).is_some() {
                state.dialogs.interaction.hover_wire_vertex =
                    Some((wire_grid_pos.x, wire_grid_pos.y));
            } else {
                state.dialogs.interaction.hover_wire_vertex = None;
            }
        } else {
            state.dialogs.interaction.hover_wire_vertex = None;
        }
        // On drag start, check if we're dragging a selected component (move) or empty space (box select)
        if response.drag_started_by(egui::PointerButton::Primary)
            && !ui.input(|i| i.modifiers.shift)
        {
            if let Some(pos) = response.interact_pointer_pos() {
                let grid_pos = screen_to_grid(pos);
                // Also check wire grid for more precise vertex detection
                let wire_grid_pos = screen_to_wire_grid(pos);

                // Priority 1: Check for wire vertex at position (for wire corner/junction dragging)
                // Use wire grid for more precise positioning
                if state.schematic.wire_vertex_at(wire_grid_pos).is_some() {
                    // Start vertex/junction drag - track the POSITION, not specific wire
                    // This allows dragging all wires meeting at a junction together
                    state.dialogs.interaction.vertex_drag_pos =
                        Some((wire_grid_pos.x, wire_grid_pos.y));
                    state.dialogs.interaction.drag.start(
                        (wire_grid_pos.x, wire_grid_pos.y),
                        crate::ui::dialog_state::interaction::DragType::WireVertex,
                    );
                }
                // Priority 2: Check if there's a component at the drag start position
                else if let Some(comp_id) = state.schematic.component_at(grid_pos) {
                    // Drag started on a component
                    if !state.schematic.selection.has_component(comp_id) {
                        // Component not selected - select it first (professional behavior)
                        // This enables click-and-drag in one motion like Cadence Virtuoso
                        state.schematic.selection.clear();
                        state.schematic.selection.select_component(comp_id);
                    }
                    // Now start dragging the selection
                    state.dialogs.drag_start = Some((grid_pos.x, grid_pos.y));
                    state.dialogs.last_drag_pos = Some((grid_pos.x, grid_pos.y));
                } else {
                    // Drag started on empty space - start box selection
                    state.schematic.selection_rect.start_at(grid_pos);
                }
            }
        }

        // Handle ongoing drag - either move selection, update box selection, or move wire vertex
        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(pos) = response.hover_pos() {
                let grid_pos = screen_to_grid(pos);
                let wire_grid_pos = screen_to_wire_grid(pos);

                // Check if we're dragging a wire vertex/junction
                if let Some((old_x, old_y)) = state.dialogs.interaction.vertex_drag_pos {
                    let old_pos = Point::new(old_x, old_y);
                    // Move ALL wire vertices at the old position to the new position
                    // This is professional EDA behavior - junctions move as a unit
                    if state.schematic.move_all_vertices_at(old_pos, wire_grid_pos) {
                        // Update the tracked position to the new location
                        state.dialogs.interaction.vertex_drag_pos =
                            Some((wire_grid_pos.x, wire_grid_pos.y));
                        state
                            .dialogs
                            .interaction
                            .drag
                            .update((wire_grid_pos.x, wire_grid_pos.y));
                    }
                } else if state.dialogs.last_drag_pos.is_some() {
                    // Moving selection with rubber-banding
                    let (last_x, last_y) = state.dialogs.last_drag_pos.unwrap();
                    let delta = Point::new(grid_pos.x - last_x, grid_pos.y - last_y);

                    if delta.x != 0 || delta.y != 0 {
                        state.schematic.move_selection_with_rubber_band(delta);
                        state.dialogs.last_drag_pos = Some((grid_pos.x, grid_pos.y));
                    }
                } else if state.schematic.selection_rect.is_active() {
                    // Update box selection rectangle position
                    state.schematic.selection_rect.update(grid_pos);
                }
            }
        }

        // Finish drag - complete box selection, finalize move, or finish wire vertex drag
        if response.drag_stopped_by(egui::PointerButton::Primary) {
            // Check if we were dragging a wire vertex
            if state.dialogs.interaction.vertex_drag_pos.is_some() {
                // Clean up wire topology after vertex move
                state.schematic.cleanup_wire_topology();
                state.dialogs.interaction.vertex_drag_pos = None;
                state.dialogs.interaction.drag.cancel();
            } else if state.dialogs.last_drag_pos.is_some() {
                // Finished moving selection - clean up any degenerate wires
                // (e.g., zero-length wires created by moving component terminal to wire endpoint)
                state.schematic.cleanup_wire_topology();
                state.dialogs.drag_start = None;
                state.dialogs.last_drag_pos = None;
            } else if let Some((min_x, min_y, max_x, max_y)) =
                state.schematic.selection_rect.finish()
            {
                // Finished box selection - Ctrl key adds to selection; otherwise replace
                let add_mode = ui.input(|i| i.modifiers.ctrl);
                state
                    .schematic
                    .select_in_rect(min_x, min_y, max_x, max_y, add_mode);
            }
        }
    }

    // Single click handling (for single item selection and placement)
    if response.clicked_by(egui::PointerButton::Primary) {
        if let Some(pos) = response.interact_pointer_pos() {
            match current_tool {
                Tool::Place(component_type) => {
                    // Components snap to full grid
                    let grid_pos = screen_to_grid(pos);
                    state.schematic.add_component(component_type, grid_pos);
                    log::info!("Placed {:?} at {:?}", component_type, grid_pos);
                }
                Tool::Wire => {
                    // Wires snap to half grid for connecting to component terminals
                    let wire_pos = screen_to_wire_grid(pos);
                    if state.schematic.wire_drawing.active {
                        state.schematic.extend_wire(wire_pos);
                    } else {
                        state.schematic.start_wire(wire_pos);
                    }
                }
                Tool::Select => {
                    // Single click: select component or wire at position
                    // Only trigger if not dragging (prevent double action)
                    let grid_pos = screen_to_grid(pos);
                    let ctrl_held = ui.input(|i| i.modifiers.ctrl);
                    let alt_held = ui.input(|i| i.modifiers.alt);

                    let comp_id = state.schematic.component_at(grid_pos);
                    let wire_id = state.schematic.wire_at(grid_pos);

                    if let Some(id) = comp_id {
                        // Clear any net highlighting when selecting components
                        state.schematic.net_highlight.clear();
                        if ctrl_held {
                            state.schematic.selection.toggle_component(id);
                        } else {
                            state.schematic.selection.clear();
                            state.schematic.selection.select_component(id);
                        }
                    } else if let Some(id) = wire_id {
                        if alt_held {
                            // Alt+Click: Highlight entire connected net
                            // Build net graph and find all connected wires
                            use crate::state::NetGraph;
                            let net_graph = NetGraph::build_from_wires(&state.schematic.wires);
                            let connected_wires = net_graph.get_connected_wires(id);

                            // Clear selection and highlight the net
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
                        // Click on empty space clears selection and highlighting
                        state.schematic.selection.clear();
                        state.schematic.net_highlight.clear();
                    }
                }
                Tool::Probe => {
                    // Probe mode: click on wire/node to add/toggle signal in waveform viewer
                    let grid_pos = screen_to_grid(pos);

                    // Check if we clicked on a wire
                    if let Some(_wire_id) = state.schematic.wire_at(grid_pos) {
                        // Look up net name from cross-probe mapping
                        if let Some(net_name) = state.simulation.cross_probe.net_at(grid_pos) {
                            let net_name = net_name.clone();
                            log::info!("Probe: clicked net '{}' at {:?}", net_name, grid_pos);

                            // Check if this is ground
                            if net_name == "0" {
                                log::info!("Probe: ground net (0V reference)");
                                state.console_messages.push(
                                    crate::common::app::ConsoleMessage::info(
                                        "Ground node: 0V reference".to_string(),
                                    ),
                                );
                            } else {
                                // Try to toggle waveform visibility for this net
                                let toggled =
                                    state.simulation.toggle_waveform_visibility(&net_name);

                                if toggled {
                                    log::info!("Probe: toggled waveform for net '{}'", net_name);
                                    state.console_messages.push(
                                        crate::common::app::ConsoleMessage::info(format!(
                                            "Toggled waveform: V({})",
                                            net_name
                                        )),
                                    );
                                } else {
                                    // Waveform not found - simulation may not have run yet
                                    log::info!(
                                        "Probe: no waveform found for net '{}' (run simulation first?)",
                                        net_name
                                    );
                                    state.console_messages.push(
                                        crate::common::app::ConsoleMessage::warning(format!(
                                            "No waveform for {}: run simulation first",
                                            net_name
                                        )),
                                    );
                                }

                                // Highlight the probed net on schematic
                                // Build net graph and use it to highlight connected wires
                                let net_graph = crate::state::NetGraph::build_from_wires(
                                    &state.schematic.wires,
                                );
                                state
                                    .schematic
                                    .net_highlight
                                    .highlight_net(&net_graph, grid_pos);
                            }
                        } else {
                            // Wire exists but not in netlist (disconnected or schematic changed)
                            log::info!(
                                "Probe: wire at {:?} not in netlist (regenerate netlist?)",
                                grid_pos
                            );
                            state.console_messages.push(
                                crate::common::app::ConsoleMessage::warning(
                                    "Wire not in netlist. Run simulation to update.".to_string(),
                                ),
                            );
                        }
                    } else if let Some(comp_id) = state.schematic.component_at(grid_pos) {
                        // Component probing - look up component and probe its nodes
                        if let Some(component) =
                            state.schematic.components.iter().find(|c| c.id == comp_id)
                        {
                            let comp_name = component.name.clone();
                            log::info!(
                                "Probe: clicked component '{}' ({})",
                                comp_name,
                                component.kind.display_name()
                            );

                            // For voltage sources, probe the current I(Vname)
                            // For other components, probe voltage at first terminal
                            let probe_name = if component.kind.spice_prefix() == "V" {
                                format!("I(V{})", comp_name)
                            } else {
                                // Get first terminal position and look up its net
                                let terminals = component.terminal_positions();
                                if let Some((_, term_pos)) = terminals.first() {
                                    if let Some(net_name) =
                                        state.simulation.cross_probe.net_at(*term_pos)
                                    {
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
                                state.console_messages.push(
                                    crate::common::app::ConsoleMessage::info(format!(
                                        "Toggled waveform: {}",
                                        probe_name
                                    )),
                                );
                            } else {
                                state.console_messages.push(
                                    crate::common::app::ConsoleMessage::warning(format!(
                                        "No waveform for {}",
                                        probe_name
                                    )),
                                );
                            }
                        }
                    } else {
                        // Clear highlight when clicking empty space
                        state.schematic.net_highlight.clear();
                        log::debug!("Probe: clicked empty space at {:?}", grid_pos);
                    }
                }
                _ => {}
            }
        }
    }

    // Double-click handling - open properties panel for selected component
    if response.double_clicked_by(egui::PointerButton::Primary) {
        if let Some(pos) = response.interact_pointer_pos() {
            let grid_pos = screen_to_grid(pos);
            if let Some(comp_id) = state.schematic.component_at(grid_pos) {
                // Select the component and open properties panel
                state.schematic.selection.clear();
                state.schematic.selection.select_component(comp_id);
                state.panels.properties = true;
                log::info!(
                    "Double-clicked component {}, opening properties panel",
                    comp_id
                );
            }
        }
    }

    // Right click to finish wire or cancel
    if response.clicked_by(egui::PointerButton::Secondary) {
        if state.schematic.wire_drawing.active {
            state.schematic.finish_wire();
        }
    }

    // Draw wire preview if active
    let wire_active = state.schematic.wire_drawing.active;

    // Update wire preview position first (mutable borrow)
    if wire_active {
        if let Some(hover_pos) = response.hover_pos() {
            let grid_pos = screen_to_wire_grid(hover_pos);
            state.schematic.update_wire_preview(grid_pos);
        }
    }

    // Now draw the wire (gather data, then draw)
    if wire_active {
        let wire_points: Vec<Point> = state.schematic.wire_drawing.points.clone();
        let preview_pos_opt = state.schematic.wire_drawing.preview_pos;

        if !wire_points.is_empty() {
            let wire_color = Color32::from_rgb(100, 200, 255);
            let stroke = Stroke::new(1.0 * tool_viewport.zoom, wire_color); // Match wire stroke

            // Draw existing segments
            for segment in wire_points.windows(2) {
                let p1 = tool_viewport.schematic_to_screen(segment[0]);
                let p2 = tool_viewport.schematic_to_screen(segment[1]);
                painter.line_segment([p1, p2], stroke);
            }

            // Draw preview to current mouse position
            if let Some(preview) = preview_pos_opt {
                if let Some(last) = wire_points.last() {
                    let p1 = tool_viewport.schematic_to_screen(*last);
                    let p2 = tool_viewport.schematic_to_screen(preview);
                    painter.line_segment(
                        [p1, p2],
                        Stroke::new(1.0 * tool_viewport.zoom, wire_color.gamma_multiply(0.6)),
                    );
                }
            }

            // Draw start point marker
            if let Some(start) = wire_points.first() {
                let start_screen = tool_viewport.schematic_to_screen(*start);
                painter.circle_filled(start_screen, 4.0 * tool_viewport.zoom, wire_color);
            }
        }
    }

    // Draw component preview if placing (copy values to avoid borrow conflict)
    let preview_tool = state.schematic.tool;
    // SVG symbols expect rotation in degrees (0, 90, 180, 270)
    // Procedural drawing uses rotation_to_delta() which expects index (0-3)
    let preview_rotation_degrees = state.schematic.preview_rotation.degrees();
    let preview_rotation_index = rotation_to_index(state.schematic.preview_rotation);
    if let Tool::Place(component_type) = preview_tool {
        if let Some(hover_pos) = response.hover_pos() {
            let grid_pos = screen_to_grid(hover_pos);
            let preview_pos = tool_viewport.schematic_to_screen(grid_pos);

            let preview_stroke = Stroke::new(
                1.0 * tool_viewport.zoom,
                Color32::from_rgba_unmultiplied(100, 200, 100, 180),
            );

            // Try to use SVG symbol for preview (matches placed component appearance)
            let svg_rendered = if let Some(library) = symbol_library {
                if let Some(symbol) = library.get(component_type) {
                    super::symbols::draw_symbol(
                        &painter,
                        symbol,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_degrees,
                        false, // Preview doesn't use mirror_h
                        false, // Preview doesn't use mirror_v
                        preview_stroke,
                    );
                    true
                } else {
                    false
                }
            } else {
                false
            };

            // Fall back to procedural drawing for preview if SVG not available
            if !svg_rendered {
                match component_type {
                    ComponentType::Resistor => draw_resistor_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::Capacitor => draw_capacitor_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::Inductor => draw_inductor_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::VoltageSource => draw_vsource_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::CurrentSource => draw_isource_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::Ground => {
                        draw_ground_symbol(&painter, preview_pos, viewport.zoom, preview_stroke)
                    }
                    ComponentType::Diode => draw_diode_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::Nmos => draw_nmos_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::Pmos => draw_pmos_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::NpnBjt => draw_npn_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    ComponentType::PnpBjt => draw_pnp_symbol(
                        &painter,
                        preview_pos,
                        viewport.zoom,
                        preview_rotation_index,
                        preview_stroke,
                    ),
                    _ => {
                        // Generic preview
                        let rect =
                            Rect::from_center_size(preview_pos, Vec2::splat(30.0 * viewport.zoom));
                        painter.rect_stroke(rect, 2.0, preview_stroke);
                    }
                }
            }
        }
    }

    // Draw box selection rectangle if active (commercial-grade dashed rectangle)
    if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        let top_left = tool_viewport.schematic_to_screen(Point::new(min_x, min_y));
        let bottom_right = tool_viewport.schematic_to_screen(Point::new(max_x, max_y));

        let selection_rect = Rect::from_min_max(top_left, bottom_right);

        // Draw filled rectangle with semi-transparent blue (matches Cadence Virtuoso style)
        painter.rect_filled(
            selection_rect,
            0.0,
            Color32::from_rgba_unmultiplied(100, 150, 255, 40),
        );

        // Draw border with solid blue stroke
        painter.rect_stroke(
            selection_rect,
            0.0,
            Stroke::new(1.0, Color32::from_rgb(100, 150, 255)),
        );
    }

    // Draw in-canvas status bar overlay (bottom of schematic area only)
    draw_canvas_status_bar(&painter, available, state, response.hover_pos());
}

/// Draw an in-canvas status bar overlay at the bottom of the schematic area
fn draw_canvas_status_bar(
    painter: &Painter,
    bounds: Rect,
    state: &AppState,
    hover_pos: Option<Pos2>,
) {
    let bar_height = 22.0;
    let bar_rect = Rect::from_min_max(
        Pos2::new(bounds.min.x, bounds.max.y - bar_height),
        bounds.max,
    );

    // Semi-transparent background
    painter.rect_filled(
        bar_rect,
        0.0,
        Color32::from_rgba_unmultiplied(30, 30, 35, 220),
    );

    // Top border line
    painter.line_segment(
        [bar_rect.left_top(), bar_rect.right_top()],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );

    let text_color = Color32::from_rgb(180, 180, 190);
    let font = egui::FontId::proportional(11.0);
    let mut x = bounds.min.x + 10.0;
    let y = bounds.max.y - bar_height / 2.0;

    // Cursor position in schematic coordinates
    if let Some(cursor) = hover_pos {
        let schematic_x =
            ((cursor.x - bounds.min.x) as f64 - state.schematic.pan.0) / state.schematic.zoom;
        let schematic_y =
            ((cursor.y - bounds.min.y) as f64 - state.schematic.pan.1) / state.schematic.zoom;

        let cursor_text = format!("({:.0}, {:.0})", schematic_x, schematic_y);
        painter.text(
            Pos2::new(x, y),
            egui::Align2::LEFT_CENTER,
            &cursor_text,
            font.clone(),
            text_color,
        );
        x += 100.0;
    }

    // Separator
    painter.line_segment(
        [
            Pos2::new(x, bar_rect.min.y + 4.0),
            Pos2::new(x, bar_rect.max.y - 4.0),
        ],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );
    x += 12.0;

    // Zoom level
    let zoom_pct = (state.schematic.zoom * 100.0) as i32;
    let zoom_text = format!("{}%", zoom_pct);
    painter.text(
        Pos2::new(x, y),
        egui::Align2::LEFT_CENTER,
        &zoom_text,
        font.clone(),
        text_color,
    );
    x += 60.0;

    // Separator
    painter.line_segment(
        [
            Pos2::new(x, bar_rect.min.y + 4.0),
            Pos2::new(x, bar_rect.max.y - 4.0),
        ],
        Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
    );
    x += 12.0;

    // Grid size
    let grid_text = format!("Grid: {}", state.schematic.grid_size);
    painter.text(
        Pos2::new(x, y),
        egui::Align2::LEFT_CENTER,
        &grid_text,
        font.clone(),
        text_color,
    );

    // Right side: selection info
    let comp_count = state.schematic.selection.components.len();
    let wire_count = state.schematic.selection.wires.len();
    let selection_text = if comp_count > 0 || wire_count > 0 {
        format!("{} comp, {} wire", comp_count, wire_count)
    } else {
        "Ready".to_string()
    };

    painter.text(
        Pos2::new(bounds.max.x - 10.0, y),
        egui::Align2::RIGHT_CENTER,
        &selection_text,
        font,
        text_color,
    );
}

/// Viewport transformation helper
struct Viewport {
    offset: Pos2,
    zoom: f32,
    bounds: Rect,
}

impl Viewport {
    /// Convert schematic coordinates to screen coordinates
    fn schematic_to_screen(&self, point: Point) -> Pos2 {
        Pos2::new(
            self.bounds.min.x + self.offset.x + (point.x as f32) * self.zoom,
            self.bounds.min.y + self.offset.y + (point.y as f32) * self.zoom,
        )
    }
}

/// Draw the schematic grid
fn draw_grid(painter: &Painter, bounds: Rect, state: &AppState) {
    let grid_size = state.schematic.grid_size as f32;
    let zoom = state.schematic.zoom as f32;
    let pan_x = state.schematic.pan.0 as f32;
    let pan_y = state.schematic.pan.1 as f32;

    let base_grid_spacing = grid_size * zoom;

    // Dynamically adjust grid step to ensure reasonable visual spacing
    // At very low zoom, show fewer grid lines (every 10th or 100th)
    let (grid_step, is_all_major) = if base_grid_spacing >= 5.0 {
        // Normal: draw every grid line, with every 10th as major
        (1, false)
    } else if base_grid_spacing * 10.0 >= 5.0 {
        // Medium zoom out: draw every 10th grid line (all appear as major)
        (10, true)
    } else if base_grid_spacing * 100.0 >= 5.0 {
        // Very zoomed out: draw every 100th grid line
        (100, true)
    } else {
        // Extremely zoomed out: skip grid entirely
        return;
    };

    let grid_spacing = base_grid_spacing * grid_step as f32;

    let minor_color = state.theme.grid_minor;
    let major_color = state.theme.grid_major;

    // Grid lines are at schematic coordinates that are multiples of grid_size.
    // In screen space: screen_x = bounds.min.x + pan_x + (schematic_x * zoom)
    // For grid line at schematic_x = gx * grid_size:
    //   screen_x = bounds.min.x + pan_x + gx * grid_size * zoom
    //            = bounds.min.x + pan_x + gx * grid_spacing

    // Calculate visible grid index range (now in terms of grid_step multiples)
    let left_grid = ((0.0 - pan_x) / grid_spacing).floor() as i32 - 1;
    let right_grid = ((bounds.width() - pan_x) / grid_spacing).ceil() as i32 + 1;
    let top_grid = ((0.0 - pan_y) / grid_spacing).floor() as i32 - 1;
    let bottom_grid = ((bounds.height() - pan_y) / grid_spacing).ceil() as i32 + 1;

    // Draw vertical lines (for each visible grid x-coordinate)
    for gx in left_grid..=right_grid {
        // Convert grid index to screen coordinate (include bounds.min.x offset)
        let screen_x = bounds.min.x + pan_x + (gx as f32) * grid_spacing;
        if screen_x < bounds.min.x || screen_x > bounds.max.x {
            continue;
        }
        // In stepped mode, determine if this is a "super major" line
        let actual_gx = gx * grid_step;
        let is_major = is_all_major || (actual_gx % 10 == 0);
        let color = if is_major { major_color } else { minor_color };
        painter.line_segment(
            [
                Pos2::new(screen_x, bounds.min.y),
                Pos2::new(screen_x, bounds.max.y),
            ],
            Stroke::new(if is_major { 1.0 } else { 0.5 }, color),
        );
    }

    // Draw horizontal lines (for each visible grid y-coordinate)
    for gy in top_grid..=bottom_grid {
        // Convert grid index to screen coordinate (include bounds.min.y offset)
        let screen_y = bounds.min.y + pan_y + (gy as f32) * grid_spacing;
        if screen_y < bounds.min.y || screen_y > bounds.max.y {
            continue;
        }
        let actual_gy = gy * grid_step;
        let is_major = is_all_major || (actual_gy % 10 == 0);
        let color = if is_major { major_color } else { minor_color };
        painter.line_segment(
            [
                Pos2::new(bounds.min.x, screen_y),
                Pos2::new(bounds.max.x, screen_y),
            ],
            Stroke::new(if is_major { 1.0 } else { 0.5 }, color),
        );
    }
}

/// Draw a wire on the canvas
fn draw_wire(
    painter: &Painter,
    viewport: &Viewport,
    wire: &crate::state::Wire,
    selected: bool,
    highlighted: bool, // Net highlighting - orange color for connected net
    state: &AppState,
) {
    // Wire is a polyline - draw all segments
    // Priority: selected > highlighted > default
    let (color, width) = if selected {
        (Color32::from_rgb(0, 255, 255), 2.0) // Cyan for selected
    } else if highlighted {
        (Color32::from_rgb(255, 165, 0), 2.0) // Orange for net highlight (Cadence Virtuoso style)
    } else {
        (state.theme.wire_default, 1.0)
    };

    // Draw each segment of the wire polyline
    for segment in wire.points.windows(2) {
        let start = viewport.schematic_to_screen(segment[0]);
        let end = viewport.schematic_to_screen(segment[1]);
        painter.line_segment([start, end], Stroke::new(width * viewport.zoom, color));
    }
}

/// Draw a component on the canvas
fn draw_component(
    painter: &Painter,
    viewport: &Viewport,
    component: &crate::state::Component,
    selected: bool,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    // Component uses `pos` not `position`, `kind` not `component_type`
    let pos = viewport.schematic_to_screen(component.pos);
    let scale = viewport.zoom;

    // Grid lines now visible through components (no opaque background)

    let outline_color = if selected {
        Color32::from_rgb(0, 255, 255) // Cyan for selected
    } else {
        state.theme.component_outline
    };

    let stroke = Stroke::new(if selected { 1.5 } else { 1.0 } * scale, outline_color);

    // SVG symbols expect rotation in degrees (0, 90, 180, 270)
    // Procedural drawing uses rotation_to_delta() which expects index (0-3)
    let rotation_degrees = component.rotation.degrees();
    let rotation_index = rotation_to_index(component.rotation);

    // Try to use SVG symbol if available
    let svg_rendered = if let Some(library) = symbol_library {
        if let Some(symbol) = library.get(component.kind) {
            // Note: SVG symbol is being used (removed per-frame logging)
            super::symbols::draw_symbol(
                painter,
                symbol,
                pos,
                scale,
                rotation_degrees,
                component.mirror_h,
                component.mirror_v,
                stroke,
            );
            true
        } else {
            // No SVG available for this component type, will use procedural fallback
            false
        }
    } else {
        // Symbol library not available, procedural fallback will be used
        false
    };

    // Fall back to procedural drawing if SVG not available
    if !svg_rendered {
        // Using procedural drawing fallback
        match component.kind {
            ComponentType::Resistor => {
                draw_resistor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Capacitor => {
                draw_capacitor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Inductor => {
                draw_inductor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::VoltageSource => {
                draw_vsource_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::CurrentSource => {
                draw_isource_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Ground => {
                draw_ground_symbol(painter, pos, scale, stroke);
            }
            ComponentType::Diode => {
                draw_diode_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Nmos => {
                draw_nmos_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Pmos => {
                draw_pmos_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::NpnBjt => {
                draw_npn_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::PnpBjt => {
                draw_pnp_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Vcvs => {
                draw_vcvs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Vccs => {
                draw_vccs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Ccvs => {
                draw_ccvs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Cccs => {
                draw_cccs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            _ => {
                // Generic component: draw a rectangle
                let rect = Rect::from_center_size(pos, Vec2::splat(30.0 * scale));
                painter.rect_stroke(rect, 2.0, stroke);
            }
        }
    }

    // Smart label placement based on component type, rotation, and dimensions
    // Commercial EDA tools (Cadence Virtuoso) place labels to avoid overlapping
    // terminals and component body, with name/value on opposite sides
    draw_component_labels(painter, pos, scale, component, state);
}

/// Smart label placement for components
///
/// Places name and value labels optimally based on:
/// - Component dimensions and aspect ratio
/// - Rotation (horizontal vs vertical orientation)
/// - Terminal positions (avoid overlapping terminals)
///
/// Matches commercial EDA tool behavior (Cadence Virtuoso style).
fn draw_component_labels(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &crate::state::Component,
    state: &AppState,
) {
    // Skip labels for Ground (too small, clutters schematic)
    if matches!(component.kind, crate::state::ComponentType::Ground) {
        return;
    }

    let (width, height) = component.kind.symbol_dimensions();
    let is_rotated_vertical = component.rotation.is_vertical(); // R90 or R270

    // Determine effective dimensions after rotation
    let (eff_w, eff_h) = if is_rotated_vertical {
        (height as f32, width as f32) // Swap for rotated
    } else {
        (width as f32, height as f32)
    };

    // Determine label placement based on component shape and orientation
    // Horizontal components: name above, value below
    // Vertical components: name to the left, value to the right
    // Tall components (BJTs, MOSFETs): name above, value below regardless of rotation

    let is_tall_device = matches!(
        component.kind,
        crate::state::ComponentType::NpnBjt
            | crate::state::ComponentType::PnpBjt
            | crate::state::ComponentType::Nmos
            | crate::state::ComponentType::Pmos
            | crate::state::ComponentType::Njfet
            | crate::state::ComponentType::Pjfet
            | crate::state::ComponentType::NVdmos
            | crate::state::ComponentType::PVdmos
    );

    let is_source = component.kind.is_source()
        && !matches!(
            component.kind,
            crate::state::ComponentType::Vcvs
                | crate::state::ComponentType::Vccs
                | crate::state::ComponentType::Ccvs
                | crate::state::ComponentType::Cccs
        );

    // Calculate label margin from component edge
    let margin = 4.0 * scale;
    let name_font = egui::FontId::proportional(10.0 * scale);
    let value_font = egui::FontId::proportional(9.0 * scale);

    // Check if ORIGINAL component is horizontally-oriented (width > height)
    // This is important because terminals are on left/right for horizontal components
    let is_horizontal_component = width > height;

    // Choose placement strategy based on component type and orientation
    let (name_pos, name_align, value_pos, value_align) = if is_tall_device {
        // Tall devices (transistors): name above, value on the terminal side
        // For transistors, value goes on the side with terminals (right side typically)
        let half_h = (eff_h / 2.0) * scale;
        let half_w = (eff_w / 2.0) * scale;

        let name_y = pos.y - half_h - margin;
        let value_x = pos.x + half_w + margin;

        (
            Pos2::new(pos.x, name_y),
            egui::Align2::CENTER_BOTTOM,
            Pos2::new(value_x, pos.y),
            egui::Align2::LEFT_CENTER,
        )
    } else if is_source && !is_rotated_vertical {
        // Vertical sources (V, I): name to the left, value to the right
        let half_w = (eff_w / 2.0) * scale;

        (
            Pos2::new(pos.x - half_w - margin, pos.y),
            egui::Align2::RIGHT_CENTER,
            Pos2::new(pos.x + half_w + margin, pos.y),
            egui::Align2::LEFT_CENTER,
        )
    } else if is_source && is_rotated_vertical {
        // Rotated sources: name above, value below
        let half_h = (eff_h / 2.0) * scale;

        (
            Pos2::new(pos.x, pos.y - half_h - margin),
            egui::Align2::CENTER_BOTTOM,
            Pos2::new(pos.x, pos.y + half_h + margin),
            egui::Align2::CENTER_TOP,
        )
    } else if is_horizontal_component && !is_rotated_vertical {
        // Horizontal components (R, L, etc.) not rotated: name above, value below
        // Terminals are on left/right, so top/bottom are safe for labels
        let half_h = (eff_h / 2.0) * scale;

        (
            Pos2::new(pos.x, pos.y - half_h - margin),
            egui::Align2::CENTER_BOTTOM,
            Pos2::new(pos.x, pos.y + half_h + margin),
            egui::Align2::CENTER_TOP,
        )
    } else if is_horizontal_component && is_rotated_vertical {
        // Horizontal component rotated 90°/270°: terminals now at top/bottom
        // Place labels on left/right to avoid terminals
        let half_w = (eff_w / 2.0) * scale;

        (
            Pos2::new(pos.x - half_w - margin, pos.y),
            egui::Align2::RIGHT_CENTER,
            Pos2::new(pos.x + half_w + margin, pos.y),
            egui::Align2::LEFT_CENTER,
        )
    } else {
        // Default: name above, value below (for square-ish components)
        let half_h = (eff_h / 2.0).max(10.0) * scale;

        (
            Pos2::new(pos.x, pos.y - half_h - margin),
            egui::Align2::CENTER_BOTTOM,
            Pos2::new(pos.x, pos.y + half_h + margin),
            egui::Align2::CENTER_TOP,
        )
    };

    // Draw component name (reference designator)
    if !component.name.is_empty() {
        painter.text(
            name_pos,
            name_align,
            &component.name,
            name_font,
            state.theme.text_primary,
        );
    }

    // Draw component value
    if !component.value.is_empty() {
        painter.text(
            value_pos,
            value_align,
            &component.value,
            value_font,
            state.theme.text_secondary,
        );
    }
}

/// Draw a junction (net connection point)
fn draw_junction(painter: &Painter, viewport: &Viewport, position: Point, state: &AppState) {
    let pos = viewport.schematic_to_screen(position);
    let radius = 1.5 * viewport.zoom; // Match wire/symbol stroke width

    // Check if this junction is being hovered (for visual feedback)
    let is_hovered = state
        .dialogs
        .interaction
        .hover_wire_vertex
        .map(|(x, y)| x == position.x && y == position.y)
        .unwrap_or(false);

    if is_hovered {
        // Draw larger highlight ring when hovered
        let highlight_radius = radius * 2.5;
        painter.circle_stroke(
            pos,
            highlight_radius,
            Stroke::new(1.0 * viewport.zoom, Color32::from_rgb(100, 200, 255)),
        );
    }

    painter.circle_filled(pos, radius, state.theme.wire_default);
}

/// Convert Rotation enum to index (0-3) for drawing functions
fn rotation_to_index(rotation: Rotation) -> i32 {
    match rotation {
        Rotation::R0 => 0,
        Rotation::R90 => 1,
        Rotation::R180 => 2,
        Rotation::R270 => 3,
    }
}

// =============================================================================
// Component Symbol Drawing Functions
// =============================================================================

fn draw_resistor_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    let len = 30.0 * scale;
    let width = 8.0 * scale;
    let segments = 6;

    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Start point
    let start = Pos2::new(pos.x - len / 2.0 * dx, pos.y - len / 2.0 * dy);

    // Draw zigzag
    let seg_len = len / (segments as f32);
    let mut points = vec![start];

    for i in 1..segments {
        let t = i as f32 * seg_len;
        let offset = if i % 2 == 1 { width } else { -width };
        points.push(Pos2::new(
            start.x + t * dx + offset * perp.0,
            start.y + t * dy + offset * perp.1,
        ));
    }
    points.push(Pos2::new(pos.x + len / 2.0 * dx, pos.y + len / 2.0 * dy));

    for i in 0..points.len() - 1 {
        painter.line_segment([points[i], points[i + 1]], stroke);
    }
}

fn draw_capacitor_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    let gap = 4.0 * scale;
    let plate_len = 16.0 * scale;

    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Two parallel plates
    let p1a = Pos2::new(
        pos.x - gap / 2.0 * dx + plate_len / 2.0 * perp.0,
        pos.y - gap / 2.0 * dy + plate_len / 2.0 * perp.1,
    );
    let p1b = Pos2::new(
        pos.x - gap / 2.0 * dx - plate_len / 2.0 * perp.0,
        pos.y - gap / 2.0 * dy - plate_len / 2.0 * perp.1,
    );

    let p2a = Pos2::new(
        pos.x + gap / 2.0 * dx + plate_len / 2.0 * perp.0,
        pos.y + gap / 2.0 * dy + plate_len / 2.0 * perp.1,
    );
    let p2b = Pos2::new(
        pos.x + gap / 2.0 * dx - plate_len / 2.0 * perp.0,
        pos.y + gap / 2.0 * dy - plate_len / 2.0 * perp.1,
    );

    painter.line_segment([p1a, p1b], stroke);
    painter.line_segment([p2a, p2b], stroke);

    // Lead lines
    let lead_len = 10.0 * scale;
    painter.line_segment(
        [
            Pos2::new(
                pos.x - (gap / 2.0 + lead_len) * dx,
                pos.y - (gap / 2.0 + lead_len) * dy,
            ),
            Pos2::new(pos.x - gap / 2.0 * dx, pos.y - gap / 2.0 * dy),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x + gap / 2.0 * dx, pos.y + gap / 2.0 * dy),
            Pos2::new(
                pos.x + (gap / 2.0 + lead_len) * dx,
                pos.y + (gap / 2.0 + lead_len) * dy,
            ),
        ],
        stroke,
    );
}

fn draw_inductor_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    let len = 30.0 * scale;
    let radius = 5.0 * scale;
    let num_coils = 4;

    let (dx, dy) = rotation_to_delta(rotation);

    // Draw series of arcs
    let coil_spacing = len / (num_coils as f32);
    for i in 0..num_coils {
        let cx = pos.x - len / 2.0 * dx + (i as f32 + 0.5) * coil_spacing * dx;
        let cy = pos.y - len / 2.0 * dy + (i as f32 + 0.5) * coil_spacing * dy;

        // Approximate arc with circle (simplified)
        painter.circle_stroke(Pos2::new(cx, cy), radius, stroke);
    }
}

fn draw_vsource_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    let radius = 12.0 * scale;

    // Circle
    painter.circle_stroke(pos, radius, stroke);

    // Plus sign at top
    let plus_y = pos.y - 5.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x - 4.0 * scale, plus_y),
            Pos2::new(pos.x + 4.0 * scale, plus_y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, plus_y - 4.0 * scale),
            Pos2::new(pos.x, plus_y + 4.0 * scale),
        ],
        stroke,
    );

    // Minus at bottom
    let minus_y = pos.y + 5.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x - 4.0 * scale, minus_y),
            Pos2::new(pos.x + 4.0 * scale, minus_y),
        ],
        stroke,
    );
}

fn draw_isource_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    let radius = 12.0 * scale;

    // Circle
    painter.circle_stroke(pos, radius, stroke);

    // Arrow
    let arrow_len = 10.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + arrow_len / 2.0),
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
        ],
        stroke,
    );

    // Arrow head
    let head_size = 3.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
            Pos2::new(pos.x - head_size, pos.y - arrow_len / 2.0 + head_size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
            Pos2::new(pos.x + head_size, pos.y - arrow_len / 2.0 + head_size),
        ],
        stroke,
    );
}

fn draw_ground_symbol(painter: &Painter, pos: Pos2, scale: f32, stroke: Stroke) {
    let width = 14.0 * scale;

    // Three horizontal lines, decreasing in width
    for i in 0..3 {
        let y = pos.y + (i as f32) * 4.0 * scale;
        let w = width - (i as f32) * 4.0 * scale;
        painter.line_segment(
            [Pos2::new(pos.x - w / 2.0, y), Pos2::new(pos.x + w / 2.0, y)],
            stroke,
        );
    }

    // Vertical lead
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - 6.0 * scale),
            Pos2::new(pos.x, pos.y),
        ],
        stroke,
    );
}

fn draw_diode_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    let size = 10.0 * scale;
    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Triangle
    let tip = Pos2::new(pos.x + size * dx, pos.y + size * dy);
    let base1 = Pos2::new(
        pos.x - size * dx + size * perp.0,
        pos.y - size * dy + size * perp.1,
    );
    let base2 = Pos2::new(
        pos.x - size * dx - size * perp.0,
        pos.y - size * dy - size * perp.1,
    );

    painter.line_segment([tip, base1], stroke);
    painter.line_segment([base1, base2], stroke);
    painter.line_segment([base2, tip], stroke);

    // Bar at cathode
    painter.line_segment(
        [
            Pos2::new(tip.x + size * perp.0, tip.y + size * perp.1),
            Pos2::new(tip.x - size * perp.0, tip.y - size * perp.1),
        ],
        stroke,
    );
}

fn draw_nmos_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    let size = 12.0 * scale;

    // Channel (vertical line)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size),
            Pos2::new(pos.x, pos.y + size),
        ],
        stroke,
    );

    // Gate (vertical line to the left)
    painter.line_segment(
        [
            Pos2::new(pos.x - size * 0.5, pos.y - size * 0.7),
            Pos2::new(pos.x - size * 0.5, pos.y + size * 0.7),
        ],
        stroke,
    );

    // Gate lead
    painter.line_segment(
        [
            Pos2::new(pos.x - size, pos.y),
            Pos2::new(pos.x - size * 0.5, pos.y),
        ],
        stroke,
    );

    // Drain/Source leads
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size),
            Pos2::new(pos.x + size, pos.y - size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

fn draw_pmos_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    // Same as NMOS but with bubble on gate
    draw_nmos_symbol(painter, pos, scale, rotation, stroke);

    let size = 12.0 * scale;
    let bubble_radius = 2.5 * scale;
    painter.circle_stroke(
        Pos2::new(pos.x - size * 0.5 - bubble_radius, pos.y),
        bubble_radius,
        stroke,
    );
}

fn draw_npn_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    let size = 12.0 * scale;

    // Base line (vertical)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.6),
            Pos2::new(pos.x, pos.y + size * 0.6),
        ],
        stroke,
    );

    // Base lead
    painter.line_segment(
        [Pos2::new(pos.x - size, pos.y), Pos2::new(pos.x, pos.y)],
        stroke,
    );

    // Collector
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.3),
            Pos2::new(pos.x + size, pos.y - size),
        ],
        stroke,
    );

    // Emitter with arrow
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size * 0.3),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

fn draw_pnp_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    let size = 12.0 * scale;

    // Base line (vertical)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.6),
            Pos2::new(pos.x, pos.y + size * 0.6),
        ],
        stroke,
    );

    // Base lead
    painter.line_segment(
        [Pos2::new(pos.x - size, pos.y), Pos2::new(pos.x, pos.y)],
        stroke,
    );

    // Emitter (arrow pointing in)
    painter.line_segment(
        [
            Pos2::new(pos.x + size, pos.y - size),
            Pos2::new(pos.x, pos.y - size * 0.3),
        ],
        stroke,
    );

    // Collector
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size * 0.3),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

// =============================================================================
// Controlled Source Symbols
// =============================================================================

/// Draw VCVS (Voltage-Controlled Voltage Source) - diamond with + and -
fn draw_vcvs_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    draw_diamond_source(painter, pos, scale, stroke);
    // Add + and - for voltage output
    let size = 12.0 * scale;
    painter.text(
        Pos2::new(pos.x - size * 0.3, pos.y - size * 0.3),
        egui::Align2::CENTER_CENTER,
        "+",
        egui::FontId::proportional(10.0 * scale),
        stroke.color,
    );
    painter.text(
        Pos2::new(pos.x + size * 0.3, pos.y + size * 0.3),
        egui::Align2::CENTER_CENTER,
        "−",
        egui::FontId::proportional(12.0 * scale),
        stroke.color,
    );
}

/// Draw VCCS (Voltage-Controlled Current Source) - diamond with arrow
fn draw_vccs_symbol(painter: &Painter, pos: Pos2, scale: f32, _rotation: i32, stroke: Stroke) {
    draw_diamond_source(painter, pos, scale, stroke);
    // Add arrow for current output
    let arr_len = 8.0 * scale;
    let head = 3.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + arr_len / 2.0),
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
            Pos2::new(pos.x - head, pos.y - arr_len / 2.0 + head),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
            Pos2::new(pos.x + head, pos.y - arr_len / 2.0 + head),
        ],
        stroke,
    );
}

/// Draw CCVS (Current-Controlled Voltage Source) - diamond with + and -
fn draw_ccvs_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    // CCVS looks like VCVS but input is current-sensing
    draw_vcvs_symbol(painter, pos, scale, rotation, stroke);
}

/// Draw CCCS (Current-Controlled Current Source) - diamond with arrow
fn draw_cccs_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation: i32, stroke: Stroke) {
    // CCCS looks like VCCS but input is current-sensing
    draw_vccs_symbol(painter, pos, scale, rotation, stroke);
}

/// Helper: Draw diamond shape for controlled sources
fn draw_diamond_source(painter: &Painter, pos: Pos2, scale: f32, stroke: Stroke) {
    let size = 12.0 * scale;
    let top = Pos2::new(pos.x, pos.y - size);
    let right = Pos2::new(pos.x + size, pos.y);
    let bottom = Pos2::new(pos.x, pos.y + size);
    let left = Pos2::new(pos.x - size, pos.y);

    painter.line_segment([top, right], stroke);
    painter.line_segment([right, bottom], stroke);
    painter.line_segment([bottom, left], stroke);
    painter.line_segment([left, top], stroke);
}

/// Convert rotation index (0-3 for 0°, 90°, 180°, 270°) to direction deltas
fn rotation_to_delta(rotation: i32) -> (f32, f32) {
    match rotation % 4 {
        0 => (1.0, 0.0),  // 0°: pointing right
        1 => (0.0, 1.0),  // 90°: pointing down
        2 => (-1.0, 0.0), // 180°: pointing left
        3 => (0.0, -1.0), // 270°: pointing up
        _ => (1.0, 0.0),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_to_delta_0() {
        let (dx, dy) = rotation_to_delta(0);
        assert_eq!((dx, dy), (1.0, 0.0));
    }

    #[test]
    fn test_rotation_to_delta_90() {
        let (dx, dy) = rotation_to_delta(1);
        assert_eq!((dx, dy), (0.0, 1.0));
    }

    #[test]
    fn test_rotation_to_delta_180() {
        let (dx, dy) = rotation_to_delta(2);
        assert_eq!((dx, dy), (-1.0, 0.0));
    }

    #[test]
    fn test_rotation_to_delta_270() {
        let (dx, dy) = rotation_to_delta(3);
        assert_eq!((dx, dy), (0.0, -1.0));
    }

    #[test]
    fn test_rotation_to_delta_wraps() {
        // Should wrap around
        assert_eq!(rotation_to_delta(4), rotation_to_delta(0));
        assert_eq!(rotation_to_delta(5), rotation_to_delta(1));
    }

    #[test]
    fn test_rotation_to_index() {
        assert_eq!(rotation_to_index(Rotation::R0), 0);
        assert_eq!(rotation_to_index(Rotation::R90), 1);
        assert_eq!(rotation_to_index(Rotation::R180), 2);
        assert_eq!(rotation_to_index(Rotation::R270), 3);
    }

    #[test]
    fn test_viewport_schematic_to_screen_no_transform() {
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        };

        let point = Point::new(50, 50);
        let screen = viewport.schematic_to_screen(point);

        assert_eq!(screen.x, 50.0);
        assert_eq!(screen.y, 50.0);
    }

    #[test]
    fn test_viewport_schematic_to_screen_with_offset() {
        let viewport = Viewport {
            offset: Pos2::new(10.0, 20.0),
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        };

        let point = Point::new(50, 50);
        let screen = viewport.schematic_to_screen(point);

        assert_eq!(screen.x, 60.0);
        assert_eq!(screen.y, 70.0);
    }

    #[test]
    fn test_viewport_schematic_to_screen_with_zoom() {
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        };

        let point = Point::new(25, 25);
        let screen = viewport.schematic_to_screen(point);

        assert_eq!(screen.x, 50.0);
        assert_eq!(screen.y, 50.0);
    }

    #[test]
    fn test_viewport_schematic_to_screen_with_bounds_offset() {
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::new(100.0, 100.0), Vec2::splat(200.0)),
        };

        let point = Point::new(50, 50);
        let screen = viewport.schematic_to_screen(point);

        // Should be offset by bounds.min
        assert_eq!(screen.x, 150.0);
        assert_eq!(screen.y, 150.0);
    }
}
