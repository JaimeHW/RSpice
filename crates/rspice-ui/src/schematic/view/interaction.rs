use egui::{Response, Ui};

use crate::common::app::{AppState, ConsoleMessage, DragType};
use crate::state::{ComponentType, NetGraph, Point, Tool};

use super::SchematicSymbolContext;
use super::bus_interaction::{BusTapCandidateError, resolve_bus_tap_candidate};
use super::coordinates::{screen_to_grid, screen_to_schematic, screen_to_wire_grid};
use super::design_notes::design_note_at;
use super::documentation_shapes::documentation_shape_at;
use super::drawing::{bus_tap_at, nearest_bus_hit, nearest_terminal};
use super::net_labels::net_label_at;
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
    if state.dialogs.move_selection.armed && current_tool != Tool::MoveSelection {
        state.dialogs.move_selection.close();
        // The pointer event that exposed an inconsistent tool/draft pair belongs
        // to neither workflow. Do not hand it to Select after cancelling the
        // transactional move or the same gesture could mutate geometry through
        // the legacy direct-drag path.
        return;
    } else if current_tool == Tool::MoveSelection && !state.dialogs.move_selection.armed {
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    }
    let shape_double_click = current_tool == Tool::DocumentationShape
        && response.double_clicked_by(egui::PointerButton::Primary);

    if current_tool == Tool::DocumentationShape
        && ui.input(|input| input.pointer.delta() != egui::Vec2::ZERO)
        && let Some(pos) = response.hover_pos()
    {
        let drawing = &mut state.schematic.documentation_shape_drawing;
        drawing.keyboard_cursor = Some(screen_to_grid(viewport, grid_size, pos));
        drawing.keyboard_active = false;
    }

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, grid_size, symbol_context);
    } else if current_tool == Tool::MoveSelection {
        handle_armed_move_selection(ui, response, state, viewport, grid_size, symbol_context);
    }

    if shape_double_click && let Some(pos) = response.interact_pointer_pos() {
        let grid_pos = screen_to_grid(viewport, grid_size, pos);
        handle_documentation_shape_click(ui, state, grid_pos, true);
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && !shape_double_click
        && let Some(pos) = response.interact_pointer_pos()
    {
        match current_tool {
            // Read-only views take no edits; the console names the library.
            Tool::Place(_)
            | Tool::Wire
            | Tool::Bus
            | Tool::BusTap
            | Tool::Junction
            | Tool::DesignNote
            | Tool::DocumentationShape
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
            Tool::DesignNote => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                place_pending_design_note(state, grid_pos);
            }
            Tool::DocumentationShape => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                handle_documentation_shape_click(ui, state, grid_pos, false);
            }
            Tool::Select => {
                let grid_pos = screen_to_grid(viewport, grid_size, pos);
                let hit_pos = screen_to_schematic(viewport, pos);
                let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
                handle_select_click(
                    ui,
                    state,
                    PointerHit::new(grid_pos, hit_pos),
                    hit_radius,
                    symbol_context,
                    viewport,
                    pos,
                );
            }
            Tool::MoveSelection => {}
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
            let hit = PointerHit::new(grid_pos, hit_pos);
            let target = pointer_target(
                state,
                hit,
                hit_radius,
                symbol_context,
                ui.ctx(),
                viewport,
                pos,
            );
            if let Some(PointerTarget::DesignNote(id)) = target
                && activate_requirement_link(state, id, ui.ctx())
            {
                state.schematic.selection.select_only_design_note(id);
            } else {
                open_object_properties(
                    state,
                    hit,
                    hit_radius,
                    symbol_context,
                    ui.ctx(),
                    viewport,
                    pos,
                );
            }
        }
    }

    if response.clicked_by(egui::PointerButton::Secondary) {
        if state.schematic.wire_drawing.active {
            state.schematic.finish_wire();
        } else if state.schematic.bus_drawing.active
            && let Err(error) = state.schematic.finish_bus()
        {
            report_bus_error(ui, state, "Bus could not be committed", error.to_string());
        } else if state.schematic.tool == Tool::DocumentationShape
            && !state
                .schematic
                .documentation_shape_drawing
                .points
                .is_empty()
        {
            finish_documentation_polygon(ui, state);
        }
    }

    if state.schematic.tool == Tool::DocumentationShape && response.has_focus() {
        handle_documentation_shape_keyboard(ui, response, state, viewport, grid_size);
    }
}

fn handle_armed_move_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
    symbol_context: &SchematicSymbolContext,
) {
    if let Err(message) = crate::common::app::armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        crate::common::app::cancel_armed_move_selection(state);
        return;
    }

    retain_move_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) =
        consume_armed_move_keyboard(ui, response.has_focus(), grid_size);
    if keyboard_step != Point::origin() {
        let draft = &mut state.dialogs.move_selection;
        draft.anchor = None;
        draft.pointer_drag = false;
        draft.preview_error = None;
        draft.preview_delta = Point::new(
            draft.preview_delta.x.saturating_add(keyboard_step.x),
            draft.preview_delta.y.saturating_add(keyboard_step.y),
        );
    }
    if keyboard_commit {
        commit_armed_move_selection(state, symbol_context);
        return;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let anchor = screen_to_grid(viewport, grid_size, position);
        if pointer_is_in_frozen_move_selection(
            state,
            ui.ctx(),
            viewport,
            symbol_context,
            position,
            anchor,
        ) {
            let draft = &mut state.dialogs.move_selection;
            draft.anchor = Some(anchor);
            draft.preview_delta = Point::origin();
            draft.pointer_drag = true;
            draft.preview_error = None;
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && state.dialogs.move_selection.pointer_drag
        && let (Some(anchor), Some(position)) = (
            state.dialogs.move_selection.anchor,
            response
                .hover_pos()
                .or_else(|| response.interact_pointer_pos()),
        )
    {
        let destination = screen_to_grid(viewport, grid_size, position);
        state.dialogs.move_selection.preview_delta = Point::new(
            destination.x.saturating_sub(anchor.x),
            destination.y.saturating_sub(anchor.y),
        );
    }

    if response.drag_stopped_by(egui::PointerButton::Primary)
        && state.dialogs.move_selection.pointer_drag
    {
        state.dialogs.move_selection.pointer_drag = false;
        commit_armed_move_selection(state, symbol_context);
        return;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point = screen_to_grid(viewport, grid_size, position);
        if let Some(anchor) = state.dialogs.move_selection.anchor {
            state.dialogs.move_selection.preview_delta = Point::new(
                point.x.saturating_sub(anchor.x),
                point.y.saturating_sub(anchor.y),
            );
            commit_armed_move_selection(state, symbol_context);
        } else if pointer_is_in_frozen_move_selection(
            state,
            ui.ctx(),
            viewport,
            symbol_context,
            position,
            point,
        ) {
            let draft = &mut state.dialogs.move_selection;
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            draft.preview_error = None;
        }
    } else if !state.dialogs.move_selection.pointer_drag
        && let (Some(anchor), Some(position)) =
            (state.dialogs.move_selection.anchor, response.hover_pos())
    {
        let destination = screen_to_grid(viewport, grid_size, position);
        state.dialogs.move_selection.preview_delta = Point::new(
            destination.x.saturating_sub(anchor.x),
            destination.y.saturating_sub(anchor.y),
        );
    }
}

fn retain_move_canvas_focus_from_pointer(response: &Response) {
    // Touch taps are reported as primary clicks by egui. A drag claims focus as
    // soon as it crosses the drag threshold so keyboard continuation works after
    // either pointer interaction without stealing focus from unrelated controls.
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

fn consume_armed_move_keyboard(ui: &Ui, canvas_has_focus: bool, grid_size: i32) -> (Point, bool) {
    if !canvas_has_focus {
        return (Point::origin(), false);
    }

    ui.input_mut(|input| {
        let mut step = Point::origin();
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
            step.x = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
            step.x = grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
            step.y = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
            step.y = grid_size;
        }
        let commit = input.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
        (step, commit)
    })
}

fn pointer_is_in_frozen_move_selection(
    state: &AppState,
    context: &egui::Context,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    position: egui::Pos2,
    grid_position: Point,
) -> bool {
    let hit_position = screen_to_schematic(viewport, position);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let Some(target) = pointer_target(
        state,
        PointerHit::new(grid_position, hit_position),
        hit_radius,
        symbol_context,
        context,
        viewport,
        position,
    ) else {
        return false;
    };
    let selection = &state.schematic.selection;
    match target {
        PointerTarget::Component(id) => selection.has_component(id),
        PointerTarget::Wire(id) => selection.has_wire(id),
        PointerTarget::Bus(id) => selection.has_bus(id),
        PointerTarget::BusTap(id) => selection.has_bus_tap(id),
        PointerTarget::NetLabel(id) => selection.has_net_label(id),
        PointerTarget::DesignNote(id) => selection.has_design_note(id),
        PointerTarget::DocumentationShape(id) => selection.has_documentation_shape(id),
        PointerTarget::Junction(_) => false,
    }
}

fn commit_armed_move_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = crate::common::app::armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        crate::common::app::cancel_armed_move_selection(state);
        return;
    }
    let delta = state.dialogs.move_selection.preview_delta;
    let mode = state.dialogs.move_selection.mode;
    if delta == Point::origin() {
        state.push_user_message(ConsoleMessage::info(
            "Move selection finished without changing geometry; no undo record was created."
                .to_owned(),
        ));
        crate::common::app::cancel_armed_move_selection(state);
        return;
    }
    state.schematic.begin_operation("move selection");
    let movement = state
        .schematic
        .move_selection_with_mode_resolved(delta, mode, |component| {
            symbol_context.terminal_points(component)
        });
    match movement {
        Ok(true) => {
            let automatic_junctions = state
                .schematic
                .document_policy
                .wire_junctions
                .automatic_junctions();
            state
                .schematic
                .cleanup_wire_topology_with_junction_policy(automatic_junctions);
            let recorded = state.schematic.end_operation();
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Moved {} selected objects by ({}, {}) in {} mode; {}.",
                state.schematic.live_movable_selection_count(),
                delta.x,
                delta.y,
                mode.label(),
                if recorded {
                    "one undo record committed"
                } else {
                    "geometry was unchanged"
                }
            )));
            crate::common::app::cancel_armed_move_selection(state);
        }
        Ok(false) => {
            state.schematic.cancel_operation();
            state.push_user_message(ConsoleMessage::info(
                "Move selection produced no geometry change; no undo record was created."
                    .to_owned(),
            ));
            crate::common::app::cancel_armed_move_selection(state);
        }
        Err(error) => {
            state.schematic.cancel_operation();
            state.dialogs.move_selection.preview_error = Some(error.to_string());
            state.dialogs.move_selection.anchor = None;
            state.dialogs.move_selection.preview_delta = Point::origin();
            state.push_user_message(ConsoleMessage::warning(format!(
                "Move selection was not committed: {error}"
            )));
        }
    }
}

fn activate_requirement_link(state: &mut AppState, note_id: u64, ctx: &egui::Context) -> bool {
    let target = state
        .schematic
        .design_notes
        .iter()
        .find(|note| note.id == note_id)
        .and_then(|note| note.requirement_target());
    match target {
        Some(crate::state::RequirementTarget::ExternalUri(uri)) => {
            let uri = uri.to_owned();
            ctx.open_url(egui::OpenUrl::new_tab(&uri));
            state.push_user_message(ConsoleMessage::info(format!(
                "Opened requirement link {uri}."
            )));
            true
        }
        Some(crate::state::RequirementTarget::ProjectSpecification(reference)) => {
            let reference = reference.to_owned();
            state.ui.results.viewer = crate::workbench::ResultViewer::Specs;
            crate::workbench::result_document::open_specification_editor(state);
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Results);
            state.push_user_message(ConsoleMessage::info(format!(
                "Opened project specifications for requirement {reference}."
            )));
            true
        }
        None => false,
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
    if !select_drag_is_authorized(state.schematic.tool, state.dialogs.move_selection.armed) {
        return;
    }

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
        let target = pointer_target(
            state,
            PointerHit::new(grid_pos, hit_pos),
            hit_radius,
            symbol_context,
            ui.ctx(),
            viewport,
            pos,
        );

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
                Some(PointerTarget::DesignNote(id)) => {
                    if !state.schematic.selection.has_design_note(id) {
                        state.schematic.selection.select_only_design_note(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::DocumentationShape(id)) => {
                    if !state.schematic.selection.has_documentation_shape(id) {
                        state
                            .schematic
                            .selection
                            .select_only_documentation_shape(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::NetLabel(id)) => {
                    if !state.schematic.selection.has_net_label(id) {
                        state.schematic.selection.select_only_net_label(id);
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

fn select_drag_is_authorized(tool: Tool, move_selection_armed: bool) -> bool {
    tool == Tool::Select && !move_selection_armed
}

fn place_component(state: &mut AppState, component_type: ComponentType, grid_pos: Point) {
    match component_type {
        ComponentType::Port => place_pending_port(state, grid_pos),
        ComponentType::CellInstance => {
            let Some(library_cell) = state.schematic.pending_library_cell.clone() else {
                state.push_user_message(ConsoleMessage::warning(
                    "No library cell selected for placement".to_string(),
                ));
                crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
                return;
            };
            let changed = state
                .schematic
                .with_undo("place library cell", |schematic| {
                    schematic.add_library_cell_component(grid_pos, library_cell);
                });
            if changed {
                log::info!("Placed library cell instance at {:?}", grid_pos);
            }
        }
        _ => {
            let changed = state.schematic.with_undo(
                format!("place {}", component_type.display_name()),
                |schematic| {
                    schematic.add_component(component_type, grid_pos);
                },
            );
            if changed {
                log::info!("Placed {:?} at {:?}", component_type, grid_pos);
            }
        }
    }
}

fn place_pending_port(state: &mut AppState, grid_pos: Point) {
    let Some(pending) = state.schematic.pending_port.clone() else {
        state.push_user_message(ConsoleMessage::warning(
            "Port placement requires a validated interface contract; reopen Place pin or port."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    };
    let authority_matches = pending
        .document_authority
        .as_ref()
        .is_some_and(|authority| {
            authority.design_execution_epoch == state.design_execution_epoch
                && authority.active_schematic_epoch == state.active_schematic_epoch
                && authority.view_path == state.workspace.active_view.display_path()
        });
    if state.schematic.read_only || state.active_view_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Interface port was not placed: the active schematic authority changed; reopen Place pin or port."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    }
    let name = pending.name.clone();
    match state.schematic.place_pending_port(grid_pos, pending) {
        Ok(stable_id) => {
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Placed interface port {name} as stable object {stable_id}; generated symbol synchronization completed where applicable."
            )));
            log::info!("Placed interface port {name} at {:?}", grid_pos);
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Interface port was not placed: {error}."
            )));
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        }
    }
}

fn place_pending_design_note(state: &mut AppState, grid_pos: Point) {
    let Some(pending) = state.schematic.pending_design_note.clone() else {
        state.push_user_message(ConsoleMessage::warning(
            "Design-note placement requires a validated documentation contract; reopen Place text or note."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    };
    let authority_matches = pending
        .document_authority
        .as_ref()
        .is_some_and(|authority| {
            authority.design_execution_epoch == state.design_execution_epoch
                && authority.active_schematic_epoch == state.active_schematic_epoch
                && authority.view_path == state.workspace.active_view.display_path()
        });
    if state.schematic.read_only || state.active_view_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Design note was not placed: the active schematic authority changed; reopen Place text or note."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    }
    let kind = pending.kind.label();
    match state.schematic.place_pending_design_note(grid_pos, pending) {
        Ok(stable_id) => {
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Placed {kind} as stable non-electrical object {stable_id}."
            )));
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Design note was not placed: {error}"
            )));
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        }
    }
}

fn handle_documentation_shape_click(
    ui: &Ui,
    state: &mut AppState,
    grid_pos: Point,
    finish_polygon: bool,
) {
    state.schematic.documentation_shape_drawing.keyboard_cursor = Some(grid_pos);
    state.schematic.documentation_shape_drawing.keyboard_active = false;
    let Some(pending) = state.schematic.pending_documentation_shape.as_ref() else {
        state.push_user_message(ConsoleMessage::warning(
            "Documentation-shape placement requires a validated graphics contract; reopen Draw documentation shape."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    };
    let authority_matches = pending
        .document_authority
        .as_ref()
        .is_some_and(|authority| {
            authority.design_execution_epoch == state.design_execution_epoch
                && authority.active_schematic_epoch == state.active_schematic_epoch
                && authority.view_path == state.workspace.active_view.display_path()
        });
    if state.schematic.read_only || state.active_view_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Documentation shape was not placed: the active schematic authority changed; reopen Draw documentation shape."
                .to_owned(),
        ));
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
        return;
    }
    let kind = pending.kind;
    match state
        .schematic
        .documentation_shape_drawing
        .add_point(kind, grid_pos)
    {
        Ok(auto_commit) if auto_commit || finish_polygon => {
            finish_documentation_shape(ui, state, kind);
        }
        Ok(_) => {}
        Err(error) => report_documentation_shape_error(ui, state, error),
    }
}

fn handle_documentation_shape_keyboard(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
) {
    let (left, right, up, down, place, finish, backspace) = ui.input_mut(|input| {
        (
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp),
            input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Space),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter),
            input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace),
        )
    });
    let directional = left || right || up || down;
    if directional {
        let fallback = response
            .hover_pos()
            .map(|position| screen_to_grid(viewport, grid_size, position))
            .or_else(|| {
                state
                    .schematic
                    .documentation_shape_drawing
                    .points
                    .last()
                    .copied()
            })
            .unwrap_or_else(Point::origin);
        let drawing = &mut state.schematic.documentation_shape_drawing;
        let mut cursor = drawing.keyboard_cursor.unwrap_or(fallback);
        let step = grid_size.max(1);
        if left {
            cursor.x = cursor.x.saturating_sub(step);
        }
        if right {
            cursor.x = cursor.x.saturating_add(step);
        }
        if up {
            cursor.y = cursor.y.saturating_sub(step);
        }
        if down {
            cursor.y = cursor.y.saturating_add(step);
        }
        drawing.keyboard_cursor = Some(cursor);
        drawing.keyboard_active = true;
    }
    if backspace {
        state.schematic.documentation_shape_drawing.points.pop();
    }

    let Some(cursor) = state
        .schematic
        .documentation_shape_drawing
        .keyboard_cursor
        .or_else(|| {
            response
                .hover_pos()
                .map(|position| screen_to_grid(viewport, grid_size, position))
        })
    else {
        return;
    };
    if place {
        state.schematic.documentation_shape_drawing.keyboard_active = true;
        handle_documentation_shape_click(ui, state, cursor, false);
        if state.schematic.tool == Tool::DocumentationShape {
            state.schematic.documentation_shape_drawing.keyboard_active = true;
        }
    } else if finish {
        let can_finish_polygon = state
            .schematic
            .pending_documentation_shape
            .as_ref()
            .is_some_and(|pending| {
                pending.kind == crate::state::DocumentationShapeKind::Polygon
                    && state.schematic.documentation_shape_drawing.points.len() >= 3
            });
        if can_finish_polygon {
            finish_documentation_polygon(ui, state);
        } else {
            state.schematic.documentation_shape_drawing.keyboard_active = true;
            handle_documentation_shape_click(ui, state, cursor, false);
            if state.schematic.tool == Tool::DocumentationShape {
                state.schematic.documentation_shape_drawing.keyboard_active = true;
            }
        }
    }
}

fn finish_documentation_polygon(ui: &Ui, state: &mut AppState) {
    let Some(kind) = state
        .schematic
        .pending_documentation_shape
        .as_ref()
        .map(|pending| pending.kind)
    else {
        return;
    };
    if kind == crate::state::DocumentationShapeKind::Polygon {
        finish_documentation_shape(ui, state, kind);
    }
}

fn finish_documentation_shape(
    ui: &Ui,
    state: &mut AppState,
    kind: crate::state::DocumentationShapeKind,
) {
    let geometry = match state.schematic.documentation_shape_drawing.geometry(kind) {
        Ok(geometry) => geometry,
        Err(error) => {
            report_documentation_shape_error(ui, state, error);
            return;
        }
    };
    let Some(pending) = state.schematic.pending_documentation_shape.clone() else {
        return;
    };
    match state
        .schematic
        .commit_documentation_shape(pending, geometry)
    {
        Ok(stable_id) => {
            crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
            state.sync_active_schematic_to_workspace();
            let message = format!(
                "Placed {} as stable non-electrical documentation shape {stable_id}.",
                kind.label()
            );
            state
                .ui
                .toasts
                .success(ui.ctx(), "Documentation shape placed", message.clone());
            state.push_user_message(ConsoleMessage::info(message));
        }
        Err(error) => {
            report_documentation_shape_error(ui, state, error);
            if matches!(
                error,
                crate::state::DocumentationShapeError::ReadOnly
                    | crate::state::DocumentationShapeError::StaleDocument
            ) {
                crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
            }
        }
    }
}

fn report_documentation_shape_error(
    ui: &Ui,
    state: &mut AppState,
    error: crate::state::DocumentationShapeError,
) {
    let message = format!("Documentation shape was not placed: {error}");
    state.ui.toasts.warn_with_title(
        ui.ctx(),
        "Documentation shape needs attention",
        message.clone(),
    );
    state.push_user_message(ConsoleMessage::warning(message));
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
pub(super) enum PointerTarget {
    Component(u64),
    DesignNote(u64),
    DocumentationShape(u64),
    NetLabel(u64),
    BusTap(u64),
    Junction(Point),
    Bus(u64),
    Wire(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PointerHit {
    pub(super) grid: Point,
    pub(super) schematic: Point,
}

impl PointerHit {
    pub(super) const fn new(grid: Point, schematic: Point) -> Self {
        Self { grid, schematic }
    }
}

pub(super) fn pointer_target(
    state: &AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) -> Option<PointerTarget> {
    design_note_at(
        ctx,
        viewport,
        &state.schematic.design_notes,
        state,
        pointer_pos,
    )
    .map(PointerTarget::DesignNote)
    .or_else(|| {
        net_label_at(ctx, viewport, &state.schematic.net_labels, pointer_pos)
            .map(PointerTarget::NetLabel)
    })
    .or_else(|| {
        symbol_context
            .component_at_resolved_symbol(&state.schematic.components, hit.grid)
            .or_else(|| state.schematic.component_at(hit.grid))
            .map(PointerTarget::Component)
    })
    .or_else(|| {
        bus_tap_at(&state.schematic.bus_taps, hit.schematic, hit_radius).map(PointerTarget::BusTap)
    })
    .or_else(|| {
        state
            .schematic
            .junction_at(hit.grid)
            .map(|_| PointerTarget::Junction(hit.grid))
    })
    .or_else(|| {
        nearest_bus_hit(&state.schematic.buses, hit.schematic, hit_radius)
            .map(|hit| PointerTarget::Bus(hit.bus_id))
    })
    .or_else(|| state.schematic.wire_at(hit.grid).map(PointerTarget::Wire))
    .or_else(|| {
        documentation_shape_at(viewport, &state.schematic.documentation_shapes, pointer_pos)
            .map(PointerTarget::DocumentationShape)
    })
}

fn handle_select_click(
    ui: &Ui,
    state: &mut AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) {
    // Ctrl and Shift both extend the selection (toggle the clicked item);
    // a plain click replaces it.
    let additive = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);
    let alt_held = ui.input(|i| i.modifiers.alt);

    match pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ui.ctx(),
        viewport,
        pointer_pos,
    ) {
        Some(PointerTarget::Component(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_component(id);
            } else {
                state.schematic.selection.clear();
                state.schematic.selection.select_component(id);
            }
        }
        Some(PointerTarget::DesignNote(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_design_note(id);
            } else {
                state.schematic.selection.select_only_design_note(id);
            }
        }
        Some(PointerTarget::DocumentationShape(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_documentation_shape(id);
            } else {
                state
                    .schematic
                    .selection
                    .select_only_documentation_shape(id);
            }
        }
        Some(PointerTarget::NetLabel(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_net_label(id);
            } else {
                state.schematic.selection.select_only_net_label(id);
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
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) {
    match pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    ) {
        Some(PointerTarget::Component(id)) => {
            state.schematic.selection.select_only_component(id);
        }
        Some(PointerTarget::DesignNote(id)) => {
            state.schematic.selection.select_only_design_note(id);
        }
        Some(PointerTarget::DocumentationShape(id)) => {
            state
                .schematic
                .selection
                .select_only_documentation_shape(id);
        }
        Some(PointerTarget::NetLabel(id)) => {
            state.schematic.selection.select_only_net_label(id);
        }
        Some(PointerTarget::BusTap(id)) => {
            state.schematic.selection.select_only_bus_tap(id);
        }
        Some(PointerTarget::Bus(id)) => {
            state.schematic.selection.select_only_bus(id);
        }
        Some(PointerTarget::Junction(_)) | Some(PointerTarget::Wire(_)) | None => return,
    }
    crate::common::app::open_selected_object_properties(state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
        DesignNoteKind, DocumentationShapeKind, Junction, NetLabel, PendingDesignNotePlacement,
        PendingDocumentationShapePlacement, PendingPortPlacement, PortDirection, PortDirectionType,
        PortDiscipline, PortSignalType, Tool, Wire,
    };

    fn pointer_viewport() -> Viewport {
        Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
        }
    }

    fn with_test_ui(mut body: impl FnMut(&egui::Ui)) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| body(ui));
        });
    }

    fn arm_test_move(state: &mut AppState, mode: crate::state::MoveSelectionMode) {
        crate::common::app::open_move_selection_dialog(state);
        state.dialogs.move_selection.mode = mode;
        state.dialogs.move_selection.arm();
        crate::workbench::commands::arm_schematic_tool(&mut state.schematic, Tool::MoveSelection);
    }

    fn move_keyboard_input() -> egui::RawInput {
        egui::RawInput {
            events: [egui::Key::ArrowRight, egui::Key::Enter]
                .into_iter()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn armed_move_keyboard_leaves_keys_unconsumed_without_canvas_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("unfocused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                assert!(!response.has_focus());
                intent = Some(consume_armed_move_keyboard(ui, response.has_focus(), 10));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(intent, Some((Point::origin(), false)));
        assert_eq!(keys_remain, Some((true, true)));
    }

    #[test]
    fn armed_move_keyboard_consumes_keys_when_canvas_has_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("focused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                response.request_focus();
                assert!(response.has_focus());
                intent = Some(consume_armed_move_keyboard(ui, response.has_focus(), 10));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(intent, Some((Point::new(10, 0), true)));
        assert_eq!(keys_remain, Some((false, false)));
    }

    #[test]
    fn armed_move_exclusively_owns_selection_drag_routing() {
        assert!(select_drag_is_authorized(Tool::Select, false));
        assert!(!select_drag_is_authorized(Tool::Select, true));
        assert!(!select_drag_is_authorized(Tool::MoveSelection, true));
        assert!(!select_drag_is_authorized(Tool::MoveSelection, false));
    }

    #[test]
    fn armed_move_commits_once_syncs_workspace_and_retains_selection() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
        let terminal = state.schematic.components[0].terminal_positions()[0].1;
        state
            .schematic
            .wires
            .push(Wire::segment(2, terminal, Point::new(20, 0)));
        state.schematic.selection.select_only_component(1);
        state.schematic.init_undo_history();
        arm_test_move(&mut state, crate::state::MoveSelectionMode::Connected);
        state.dialogs.move_selection.preview_delta = Point::new(0, 10);
        let symbols = SchematicSymbolContext::from_state(&state);

        commit_armed_move_selection(&mut state, &symbols);

        assert_eq!(state.schematic.components[0].pos, Point::new(0, 10));
        assert_eq!(
            state.schematic.wires[0].points[0],
            Point::new(terminal.x, terminal.y + 10)
        );
        assert_eq!(state.schematic.undo_description(), Some("move selection"));
        assert!(state.schematic.selection.has_component(1));
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(!state.dialogs.move_selection.armed);
        assert_eq!(
            state
                .workspace
                .active_schematic()
                .expect("active workspace buffer")
                .components[0]
                .pos,
            Point::new(0, 10)
        );
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].pos, Point::origin());
        assert!(
            !state.schematic.can_undo(),
            "the gesture owns one undo record"
        );
    }

    #[test]
    fn cancelling_armed_move_preserves_geometry_selection_and_history() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
        state.schematic.selection.select_only_component(1);
        state.schematic.init_undo_history();
        arm_test_move(&mut state, crate::state::MoveSelectionMode::Shove);
        state.dialogs.move_selection.preview_delta = Point::new(40, 10);

        crate::common::app::cancel_armed_move_selection(&mut state);

        assert_eq!(state.schematic.components[0].pos, Point::origin());
        assert!(state.schematic.selection.has_component(1));
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn validated_port_contract_places_once_and_undo_redo_is_exact() {
        let mut state = AppState::default();
        let pending = PendingPortPlacement::new(
            "BIAS_EN",
            PortDirectionType::InputLogic,
            PortDiscipline::Logic,
            state.schematic.topology_version(),
            state.schematic.next_interface_order(),
        )
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        );
        state.schematic.pending_port = Some(pending);
        state.schematic.tool = Tool::Place(ComponentType::Port);

        place_component(&mut state, ComponentType::Port, Point::new(20, 30));

        assert_eq!(state.schematic.components.len(), 1);
        let placed = state.schematic.components[0].clone();
        assert_eq!(placed.pos, Point::new(20, 30));
        assert_eq!(placed.value, "BIAS_EN");
        let contract = placed.port_contract().expect("typed interface contract");
        assert_eq!(contract.direction, PortDirection::In);
        assert_eq!(contract.signal_type, PortSignalType::Logic);
        assert_eq!(contract.discipline, PortDiscipline::Logic);
        assert!(!contract.documentation.is_empty());
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(state.schematic.pending_port.is_none());
        assert_eq!(
            state.schematic.undo_description(),
            Some("place interface port")
        );

        assert!(state.schematic.undo());
        assert!(state.schematic.components.is_empty());
        assert!(state.schematic.redo());
        assert_eq!(state.schematic.components, [placed]);
    }

    #[test]
    fn validated_design_note_contract_places_once_without_changing_topology() {
        let mut state = AppState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::PlainText,
            "Bias network",
            state.schematic.topology_version(),
            &state.schematic.design_notes,
        )
        .unwrap()
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        );
        let topology = state.schematic.topology_version();
        state.schematic.pending_design_note = Some(pending);
        state.schematic.tool = Tool::DesignNote;

        place_pending_design_note(&mut state, Point::new(20, 30));

        assert_eq!(state.schematic.design_notes.len(), 1);
        assert_eq!(state.schematic.design_notes[0].pos, Point::new(20, 30));
        assert_eq!(state.schematic.topology_version(), topology);
        assert!(state.schematic.pending_design_note.is_none());
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(state.schematic.undo());
        assert!(state.schematic.design_notes.is_empty());
    }

    #[test]
    fn every_documentation_shape_gesture_commits_once_and_remains_non_electrical() {
        let cases = [
            (
                DocumentationShapeKind::Rectangle,
                vec![Point::new(0, 0), Point::new(20, 10)],
                false,
            ),
            (
                DocumentationShapeKind::Line,
                vec![Point::new(0, 0), Point::new(20, 10)],
                false,
            ),
            (
                DocumentationShapeKind::Polygon,
                vec![Point::new(0, 0), Point::new(20, 0), Point::new(10, 10)],
                true,
            ),
            (
                DocumentationShapeKind::Arc,
                vec![Point::new(0, 10), Point::new(10, 0), Point::new(20, 10)],
                false,
            ),
            (
                DocumentationShapeKind::Callout,
                vec![Point::new(0, 0), Point::new(10, 10), Point::new(30, 20)],
                false,
            ),
        ];

        for (kind, points, finish_on_last_click) in cases {
            let mut state = AppState::default();
            let topology = state.schematic.topology_version();
            state.schematic.pending_documentation_shape = Some(
                PendingDocumentationShapePlacement::new(
                    kind,
                    topology,
                    &state.schematic.documentation_shapes,
                )
                .with_document_authority(
                    state.design_execution_epoch,
                    state.active_schematic_epoch,
                    state.workspace.active_view.display_path(),
                ),
            );
            state.schematic.tool = Tool::DocumentationShape;

            for (index, point) in points.iter().copied().enumerate() {
                let finish = finish_on_last_click && index + 1 == points.len();
                with_test_ui(|ui| handle_documentation_shape_click(ui, &mut state, point, finish));
            }

            assert_eq!(state.schematic.documentation_shapes.len(), 1, "{kind:?}");
            assert_eq!(state.schematic.documentation_shapes[0].kind(), kind);
            assert_eq!(state.schematic.topology_version(), topology);
            assert!(state.schematic.components.is_empty());
            assert!(state.schematic.wires.is_empty());
            assert_eq!(state.schematic.tool, Tool::Select);
            assert!(state.schematic.pending_documentation_shape.is_none());
            assert!(
                state
                    .schematic
                    .documentation_shape_drawing
                    .points
                    .is_empty()
            );
            assert_eq!(
                state.schematic.undo_description(),
                Some("draw documentation shape")
            );
            assert!(state.schematic.undo());
            assert!(state.schematic.documentation_shapes.is_empty());
            assert!(
                !state.schematic.can_undo(),
                "{kind:?} must create one undo step"
            );
        }
    }

    #[test]
    fn stale_documentation_shape_authority_is_consumed_without_document_mutation() {
        let mut state = AppState::default();
        state.schematic.pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                DocumentationShapeKind::Line,
                state.schematic.topology_version(),
                &state.schematic.documentation_shapes,
            )
            .with_document_authority(
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
        state.schematic.tool = Tool::DocumentationShape;
        state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

        with_test_ui(|ui| {
            handle_documentation_shape_click(ui, &mut state, Point::new(0, 0), false)
        });

        assert!(state.schematic.documentation_shapes.is_empty());
        assert!(state.schematic.pending_documentation_shape.is_none());
        assert!(
            state
                .schematic
                .documentation_shape_drawing
                .points
                .is_empty()
        );
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn focused_keyboard_cursor_places_exact_grid_resolved_shape_points() {
        let mut state = AppState::default();
        let grid = state.schematic.grid_size;
        state.schematic.pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                DocumentationShapeKind::Line,
                state.schematic.topology_version(),
                &state.schematic.documentation_shapes,
            )
            .with_document_authority(
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
        state.schematic.tool = Tool::DocumentationShape;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let keyboard_frame = |keys: &[egui::Key], state: &mut AppState| {
            let input = egui::RawInput {
                events: keys
                    .iter()
                    .copied()
                    .map(|key| egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    })
                    .collect(),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let response = ui.interact(
                        ui.max_rect(),
                        egui::Id::new("documentation-shape-keyboard-test"),
                        egui::Sense::click_and_drag(),
                    );
                    let viewport = pointer_viewport();
                    handle_documentation_shape_keyboard(ui, &response, state, &viewport, grid);
                });
            });
        };

        keyboard_frame(&[egui::Key::ArrowRight, egui::Key::Space], &mut state);
        assert_eq!(
            state.schematic.documentation_shape_drawing.points,
            vec![Point::new(grid, 0)]
        );
        keyboard_frame(&[egui::Key::ArrowDown, egui::Key::Enter], &mut state);

        assert_eq!(state.schematic.documentation_shapes.len(), 1);
        assert_eq!(
            state.schematic.documentation_shapes[0].geometry,
            crate::state::DocumentationShapeGeometry::Line {
                start: Point::new(grid, 0),
                end: Point::new(grid, grid),
            }
        );
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(
            state
                .schematic
                .documentation_shape_drawing
                .points
                .is_empty()
        );
        assert!(
            state
                .schematic
                .documentation_shape_drawing
                .keyboard_cursor
                .is_none()
        );
    }

    #[test]
    fn stale_design_note_authority_is_consumed_without_document_mutation() {
        let mut state = AppState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::ReviewNote,
            "Review bias path",
            state.schematic.topology_version(),
            &state.schematic.design_notes,
        )
        .unwrap()
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        );
        state.schematic.pending_design_note = Some(pending);
        state.schematic.tool = Tool::DesignNote;
        state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

        place_pending_design_note(&mut state, Point::new(20, 30));

        assert!(state.schematic.design_notes.is_empty());
        assert!(state.schematic.pending_design_note.is_none());
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn port_placement_without_a_current_validated_contract_fails_closed() {
        let mut state = AppState::default();
        state.schematic.tool = Tool::Place(ComponentType::Port);

        place_component(&mut state, ComponentType::Port, Point::new(20, 30));

        assert!(state.schematic.components.is_empty());
        assert!(!state.schematic.can_undo());
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(state.schematic.pending_port.is_none());
    }

    #[test]
    fn topology_change_rejects_frozen_port_without_partial_mutation() {
        let mut state = AppState::default();
        state.schematic.pending_port = Some(
            PendingPortPlacement::new(
                "OUT",
                PortDirectionType::OutputAnalog,
                PortDiscipline::Electrical,
                state.schematic.topology_version(),
                state.schematic.next_interface_order(),
            )
            .with_document_authority(
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
        state.schematic.tool = Tool::Place(ComponentType::Port);
        state.schematic.bump_topology_version();

        place_component(&mut state, ComponentType::Port, Point::new(40, 10));

        assert!(state.schematic.components.is_empty());
        assert!(!state.schematic.can_undo());
        assert_eq!(state.schematic.tool, Tool::Select);
    }

    #[test]
    fn armed_port_rejects_a_replaced_active_document_even_when_topology_matches() {
        let mut state = AppState::default();
        state.schematic.pending_port = Some(
            PendingPortPlacement::new(
                "OUT",
                PortDirectionType::OutputAnalog,
                PortDiscipline::Electrical,
                state.schematic.topology_version(),
                state.schematic.next_interface_order(),
            )
            .with_document_authority(
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
        state.schematic.tool = Tool::Place(ComponentType::Port);
        state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

        place_component(&mut state, ComponentType::Port, Point::new(40, 10));

        assert!(state.schematic.components.is_empty());
        assert!(!state.schematic.can_undo());
        assert_eq!(state.schematic.tool, Tool::Select);
    }

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
        let ctx = egui::Context::default();
        let viewport = pointer_viewport();
        let screen_point = egui::pos2(point.x as f32, point.y as f32);

        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(point, point),
                1,
                &context,
                &ctx,
                &viewport,
                screen_point,
            ),
            Some(PointerTarget::Component(10))
        );
        state.schematic.components.clear();
        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(point, point),
                1,
                &context,
                &ctx,
                &viewport,
                screen_point,
            ),
            Some(PointerTarget::BusTap(21))
        );
        state.schematic.bus_taps.clear();
        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(point, point),
                1,
                &context,
                &ctx,
                &viewport,
                screen_point,
            ),
            Some(PointerTarget::Junction(point))
        );
        state.schematic.junctions.clear();
        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(point, point),
                1,
                &context,
                &ctx,
                &viewport,
                screen_point,
            ),
            Some(PointerTarget::Bus(20))
        );
        state.schematic.buses.clear();
        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(point, point),
                1,
                &context,
                &ctx,
                &viewport,
                screen_point,
            ),
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
        let ctx = egui::Context::default();
        let viewport = pointer_viewport();
        let screen_point = egui::pos2(10.0, 0.0);

        open_object_properties(
            &mut state,
            PointerHit::new(Point::new(10, 0), Point::new(10, 0)),
            1,
            &symbol_context,
            &ctx,
            &viewport,
            screen_point,
        );

        assert_eq!(state.schematic.selection.single_bus_tap(), Some(21));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(crate::common::app::ObjectPropertiesDraft::BusTap(_))
        ));
    }

    #[test]
    fn net_label_text_bounds_are_a_first_class_pointer_target() {
        let mut state = AppState::default();
        let label = NetLabel::new(31, Point::new(40, 40), "afe_out");
        state.schematic.net_labels.push(label.clone());
        state
            .schematic
            .components
            .push(Component::new(10, ComponentType::Resistor, label.pos));
        state
            .schematic
            .wires
            .push(Wire::segment(11, Point::new(0, 40), Point::new(100, 40)));
        let symbol_context = SchematicSymbolContext::default();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        let viewport = pointer_viewport();
        let hit = super::super::net_labels::hit_bounds(&ctx, &viewport, &label)
            .expect("visible label")
            .center();

        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(label.pos, label.pos),
                1,
                &symbol_context,
                &ctx,
                &viewport,
                hit,
            ),
            Some(PointerTarget::NetLabel(31))
        );

        open_object_properties(
            &mut state,
            PointerHit::new(label.pos, label.pos),
            1,
            &symbol_context,
            &ctx,
            &viewport,
            hit,
        );
        assert!(state.dialogs.object_properties.open);
        assert!(matches!(
            state.dialogs.object_properties.draft.as_ref(),
            Some(crate::common::app::ObjectPropertiesDraft::NetLabel(draft))
                if draft.original.id == label.id
        ));

        state.schematic.net_labels.clear();
        assert_eq!(
            pointer_target(
                &state,
                PointerHit::new(label.pos, label.pos),
                1,
                &symbol_context,
                &ctx,
                &viewport,
                hit,
            ),
            Some(PointerTarget::Component(10)),
            "once the visually topmost label is absent the component receives the pointer"
        );
    }

    #[test]
    fn requirement_link_activation_uses_owned_specifications_or_safe_external_url() {
        let mut state = AppState::default();
        state.schematic.design_notes.push(
            crate::state::DesignNote::new(
                32,
                Point::new(20, 20),
                DesignNoteKind::RequirementLink,
                "REQ-19",
            )
            .unwrap(),
        );
        let ctx = egui::Context::default();
        assert!(activate_requirement_link(&mut state, 32, &ctx));
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
        assert_eq!(
            state.ui.results.viewer,
            crate::workbench::ResultViewer::Specs
        );
        assert!(state.ui.results.spec_drafts.is_some());

        state.schematic.design_notes.push(
            crate::state::DesignNote::new(
                33,
                Point::new(30, 20),
                DesignNoteKind::RequirementLink,
                "https://tracker.example/item?id=19&from=schematic%20note",
            )
            .unwrap(),
        );
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            assert!(activate_requirement_link(&mut state, 33, ctx));
        });
        assert!(
            output
                .platform_output
                .commands
                .iter()
                .any(|command| matches!(
                    command,
                    egui::OutputCommand::OpenUrl(open)
                        if open.url == "https://tracker.example/item?id=19&from=schematic%20note"
                            && open.new_tab
                ))
        );
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
