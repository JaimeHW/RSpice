//! Canvas pointer and keyboard interaction.
//!
//! Resolves what is under the pointer, then dispatches to the operation the
//! armed tool implies. Hit resolution is ordered — a terminal beats a wire
//! vertex, which beats the wire body — so a click near a junction does what
//! the designer meant.

use egui::{Response, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::simulation::netlist_gen::{DesignNet, projection_nets};
use crate::state::{
    ComponentType, NetGraph, Point, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe, Tool, ViewType,
};
use crate::workbench::app_state::{AppState, DragType};

use super::SchematicSymbolContext;
use super::array_interaction::handle_armed_array_selection;
use super::bus_interaction::{BusTapCandidateError, resolve_bus_tap_candidate_on_active_sheet};
use super::coordinates::screen_to_schematic;
use super::design_notes::design_note_at;
use super::documentation_shapes::documentation_shape_at;
use super::drawing::{
    WireScreenHit, bus_tap_at, nearest_bus_hit, nearest_wire_screen_hit, probe_at_screen,
};
use super::navigation::primary_pan_gesture_active;
use super::net_labels::net_label_at;
use super::scene::visible_design_notes;
use super::sheet_visibility::{
    active_junction_at, active_wire_at, active_wire_point_is_draggable, objects_on_active_sheet,
    retain_selection_on_active_sheet, select_in_rect_on_active_sheet, with_active_wire_topology,
};
use super::snap_resolution::{
    resolve_grid_pointer, resolve_target_pointer, target_acquisition_radius,
};
use super::stretch_interaction::handle_armed_stretch_selection;
use super::viewport::Viewport;

pub(super) fn handle_tool_interactions(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    retain_selection_on_active_sheet(state);
    if primary_pan_gesture_active(ui, response) {
        return;
    }
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
        state.schematic.cancel_tool();
        return;
    } else if state.dialogs.stretch_selection.armed && current_tool != Tool::StretchSelection {
        state.dialogs.stretch_selection.close();
        return;
    } else if current_tool == Tool::StretchSelection && !state.dialogs.stretch_selection.armed {
        state.schematic.cancel_tool();
        return;
    } else if state.dialogs.array_selection.armed && current_tool != Tool::ArraySelection {
        state.dialogs.array_selection.close();
        return;
    } else if current_tool == Tool::ArraySelection && !state.dialogs.array_selection.armed {
        state.schematic.cancel_tool();
        return;
    }
    let shape_double_click = current_tool == Tool::DocumentationShape
        && response.double_clicked_by(egui::PointerButton::Primary);
    let route_double_click = matches!(current_tool, Tool::Wire | Tool::Bus)
        && response.double_clicked_by(egui::PointerButton::Primary)
        && (state.schematic.wire_drawing.active || state.schematic.bus_drawing.active);
    let route_enter = response.has_focus()
        && (state.schematic.wire_drawing.active || state.schematic.bus_drawing.active)
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    if route_double_click || route_enter {
        finish_active_route(ui, state);
        return;
    }

    if current_tool == Tool::DocumentationShape
        && ui.input(|input| input.pointer.delta() != egui::Vec2::ZERO)
        && let Some(pos) = response.hover_pos()
    {
        let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
        let drawing = &mut state.schematic.documentation_shape_drawing;
        drawing.keyboard_cursor = Some(position);
        drawing.keyboard_active = false;
    }

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::MoveSelection {
        handle_armed_move_selection(ui, response, state, viewport, grid_size, symbol_context);
    } else if current_tool == Tool::StretchSelection {
        handle_armed_stretch_selection(ui, response, state, viewport, grid_size, symbol_context);
    } else if current_tool == Tool::ArraySelection {
        handle_armed_array_selection(ui, response, state, viewport, grid_size, symbol_context);
    }

    if shape_double_click && let Some(pos) = response.interact_pointer_pos() {
        let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
        handle_documentation_shape_click(ui, state, position, true);
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && !shape_double_click
        && !route_double_click
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
            | Tool::OffSheetConnector
            | Tool::Probe
                if state.schematic_edit_read_only() =>
            {
                state.deny_read_only_edit();
            }
            Tool::Place(component_type) => {
                let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
                place_component(state, component_type, position);
            }
            Tool::Wire => {
                let conductor_hit = nearest_active_wire_screen_hit(state, viewport, pos);
                let fallback =
                    resolve_target_pointer(state, symbol_context, viewport, pos).snapped_position;
                match resolved_wire_attachment(conductor_hit, fallback) {
                    Some(wire_pos) if state.schematic.wire_drawing.active => {
                        state.schematic.extend_wire(wire_pos);
                        if conductor_hit.is_some() {
                            let _ = state.schematic.finish_wire();
                        }
                    }
                    Some(wire_pos) => state.schematic.start_wire(wire_pos),
                    None => report_unrepresentable_conductor_attachment(ui, state),
                }
            }
            Tool::Bus => {
                let bus_pos = resolve_grid_pointer(state, viewport, pos).snapped_position;
                if state.schematic.bus_drawing.active {
                    state.schematic.extend_bus(bus_pos);
                } else {
                    let declaration = state.schematic.bus_drawing.declaration.clone();
                    if let Err(error) = state.schematic.start_bus(bus_pos, declaration) {
                        report_bus_error(ui, state, "Bus could not be started", error.to_string());
                    }
                }
            }
            Tool::BusTap => {
                let requested = screen_to_schematic(viewport, pos);
                let hit_radius = target_acquisition_radius(viewport);
                handle_bus_tap_click(ui, state, requested, hit_radius);
            }
            Tool::Junction => {
                let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
                handle_junction_click(ui, state, position);
            }
            Tool::DesignNote => {
                let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
                place_pending_design_note(state, position);
            }
            Tool::DocumentationShape => {
                let position = resolve_grid_pointer(state, viewport, pos).snapped_position;
                handle_documentation_shape_click(ui, state, position, false);
            }
            Tool::Select => {
                let grid_pos = resolve_grid_pointer(state, viewport, pos).snapped_position;
                let hit_pos = screen_to_schematic(viewport, pos);
                let hit_radius = target_acquisition_radius(viewport);
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
            Tool::MoveSelection | Tool::StretchSelection | Tool::ArraySelection => {}
            Tool::Probe => {
                let position =
                    resolve_target_pointer(state, symbol_context, viewport, pos).snapped_position;
                handle_probe_click(ui, state, position, symbol_context);
            }
            // Both naming tools capture the same snapped anchor; the armed
            // tool is what tells the placement transaction which label it is.
            Tool::Label | Tool::OffSheetConnector => {
                let anchor =
                    resolve_target_pointer(state, symbol_context, viewport, pos).snapped_position;
                crate::workbench::app::open_net_label_placement(state, anchor);
            }
        }
    }

    if matches!(current_tool, Tool::Select)
        && response.double_clicked_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = resolve_grid_pointer(state, viewport, pos).snapped_position;
        let hit_pos = screen_to_schematic(viewport, pos);
        let hit_radius = target_acquisition_radius(viewport);
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
        let canvas_is_empty_at_pointer = target.is_none()
            && pointer_target_with_filter(
                state,
                hit,
                hit_radius,
                symbol_context,
                ui.ctx(),
                viewport,
                pos,
                crate::state::SchematicSelectionFilter::default(),
            )
            .is_none();
        match select_double_click_action(state, target, canvas_is_empty_at_pointer) {
            SelectDoubleClickAction::Descend(id) => {
                state.schematic.selection.select_only_component(id);
                state.open_selected_instance_master();
            }
            SelectDoubleClickAction::OpenVerilogA(id) => {
                state.schematic.selection.select_only_component(id);
                let _ = state.open_veriloga_source_for_component(id);
            }
            SelectDoubleClickAction::ActivateRequirement(id) => {
                if activate_requirement_link(state, id, ui.ctx()) {
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
            SelectDoubleClickAction::OpenProperties => {
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
            SelectDoubleClickAction::Ascend => state.ascend_workspace_level(),
            SelectDoubleClickAction::None => {}
        }
    }

    if state.schematic.tool == Tool::DocumentationShape && response.has_focus() {
        handle_documentation_shape_keyboard(ui, response, state, viewport, grid_size);
    }
}

fn finish_active_route(ui: &Ui, state: &mut AppState) -> bool {
    if state.schematic.wire_drawing.active {
        return state.schematic.finish_wire().is_some();
    }
    if state.schematic.bus_drawing.active {
        return match state.schematic.finish_bus() {
            Ok(bus) => bus.is_some(),
            Err(error) => {
                report_bus_error(ui, state, "Bus could not be committed", error.to_string());
                false
            }
        };
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectDoubleClickAction {
    Descend(u64),
    OpenVerilogA(u64),
    ActivateRequirement(u64),
    OpenProperties,
    Ascend,
    None,
}

fn select_double_click_action(
    state: &AppState,
    target: Option<PointerTarget>,
    canvas_is_empty_at_pointer: bool,
) -> SelectDoubleClickAction {
    match target {
        Some(PointerTarget::Component(id))
            if state.hierarchy_master_for_component(id).is_some() =>
        {
            SelectDoubleClickAction::Descend(id)
        }
        Some(PointerTarget::Component(id)) if state.veriloga_source_for_component(id).is_some() => {
            SelectDoubleClickAction::OpenVerilogA(id)
        }
        Some(PointerTarget::DesignNote(id)) => SelectDoubleClickAction::ActivateRequirement(id),
        Some(_) => SelectDoubleClickAction::OpenProperties,
        None if canvas_is_empty_at_pointer && state.workspace.hierarchy_stack.len() > 1 => {
            SelectDoubleClickAction::Ascend
        }
        None => SelectDoubleClickAction::None,
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
    if let Err(message) = crate::workbench::app::armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_move_selection(state);
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
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let anchor = resolve_grid_pointer(state, viewport, position).snapped_position;
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
        let destination = resolve_grid_pointer(state, viewport, position).snapped_position;
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
        let point = resolve_grid_pointer(state, viewport, position).snapped_position;
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
        let destination = resolve_grid_pointer(state, viewport, position).snapped_position;
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
        PointerTarget::Probe(id) => selection.has_probe(id),
        PointerTarget::Junction(_) => false,
    }
}

fn commit_armed_move_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = crate::workbench::app::armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_move_selection(state);
        return;
    }
    let delta = state.dialogs.move_selection.preview_delta;
    let mode = state.dialogs.move_selection.mode;
    if delta == Point::origin() {
        state.push_user_message(ConsoleMessage::info(
            "Move selection finished without changing geometry; no undo record was created."
                .to_owned(),
        ));
        crate::workbench::app::cancel_armed_move_selection(state);
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
            crate::workbench::app::cancel_armed_move_selection(state);
        }
        Ok(false) => {
            state.schematic.cancel_operation();
            state.push_user_message(ConsoleMessage::info(
                "Move selection produced no geometry change; no undo record was created."
                    .to_owned(),
            ));
            crate::workbench::app::cancel_armed_move_selection(state);
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
            crate::workbench::documents::result_document::open_specification_editor(state);
            state.push_user_message(ConsoleMessage::info(format!(
                "Opened project specifications for requirement {reference}."
            )));
            true
        }
        None => false,
    }
}

fn handle_bus_tap_click(ui: &Ui, state: &mut AppState, requested: Point, hit_radius: i32) {
    let candidate = match resolve_bus_tap_candidate_on_active_sheet(state, requested, hit_radius) {
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

fn nearest_active_wire_screen_hit(
    state: &AppState,
    viewport: &Viewport,
    pointer: egui::Pos2,
) -> Option<WireScreenHit> {
    let wires = objects_on_active_sheet(state, &state.schematic.wires, |item| item.id);
    nearest_wire_screen_hit(viewport, wires.as_ref(), pointer, 6.0)
}

/// A visual conductor acquisition owns the click. If no exact integer
/// attachment can be represented, fail closed instead of silently falling
/// back to a nearby grid point and creating a disconnected route.
fn resolved_wire_attachment(hit: Option<WireScreenHit>, fallback: Point) -> Option<Point> {
    hit.map_or(Some(fallback), |hit| hit.attachment)
}

fn report_unrepresentable_conductor_attachment(ui: &Ui, state: &mut AppState) {
    let message = "The conductor was acquired visually, but no exact schematic attachment point could be represented; the wire was not started."
        .to_owned();
    state
        .ui
        .toasts
        .warn_with_title(ui.ctx(), "Wire attachment unavailable", message.clone());
    state.push_user_message(ConsoleMessage::warning(message));
}

fn handle_select_dragging(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if !select_drag_is_authorized(state.schematic.tool, state.dialogs.move_selection.armed) {
        return;
    }

    let filter = state.ui.schematic_selection_filter;
    if filter.wires
        && let Some(pos) = response.hover_pos()
    {
        let wire_position =
            resolve_target_pointer(state, symbol_context, viewport, pos).snapped_position;
        if active_wire_point_is_draggable(state, wire_position) {
            state.dialogs.interaction.hover_wire_vertex = Some((wire_position.x, wire_position.y));
        } else {
            state.dialogs.interaction.hover_wire_vertex = None;
        }
    } else {
        state.dialogs.interaction.hover_wire_vertex = None;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(pos) = response.interact_pointer_pos()
    {
        let grid_pos = resolve_grid_pointer(state, viewport, pos).snapped_position;
        let wire_position =
            resolve_target_pointer(state, symbol_context, viewport, pos).snapped_position;
        let hit_pos = screen_to_schematic(viewport, pos);
        let hit_radius = target_acquisition_radius(viewport);
        let target = pointer_target(
            state,
            PointerHit::new(grid_pos, hit_pos),
            hit_radius,
            symbol_context,
            ui.ctx(),
            viewport,
            pos,
        );

        if !select_drag_can_start(primary_pan_gesture_active(ui, response)) {
            return;
        } else if state.schematic_edit_read_only() {
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
                Some(PointerTarget::Probe(id)) => {
                    if !state.schematic.selection.has_probe(id) {
                        state.schematic.selection.select_only_probe(id);
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
                    if filter.wires && active_wire_point_is_draggable(state, wire_position) =>
                {
                    start_wire_vertex_drag(state, wire_position);
                }
                Some(PointerTarget::Bus(id)) => {
                    if !state.schematic.selection.has_bus(id) {
                        state.schematic.selection.select_only_bus(id);
                    }
                    start_selection_drag(state, grid_pos);
                }
                Some(PointerTarget::Wire(_))
                    if filter.wires && active_wire_point_is_draggable(state, wire_position) =>
                {
                    start_wire_vertex_drag(state, wire_position);
                }
                _ => state.schematic.selection_rect.start_at(grid_pos),
            }
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && let Some(pos) = response.hover_pos()
    {
        let grid_pos = resolve_grid_pointer(state, viewport, pos).snapped_position;

        if let Some((old_x, old_y)) = state.dialogs.interaction.vertex_drag_pos {
            let old_pos = Point::new(old_x, old_y);
            if with_active_wire_topology(state, |schematic| {
                schematic.move_all_vertices_at(old_pos, grid_pos)
            }) {
                state.dialogs.interaction.vertex_drag_pos = Some((grid_pos.x, grid_pos.y));
                state
                    .dialogs
                    .interaction
                    .drag
                    .update((grid_pos.x, grid_pos.y));
            }
        } else if let Some((last_x, last_y)) = state.dialogs.last_drag_pos {
            let delta = Point::new(
                grid_pos.x.saturating_sub(last_x),
                grid_pos.y.saturating_sub(last_y),
            );

            if delta.x != 0 || delta.y != 0 {
                with_active_wire_topology(state, |schematic| {
                    schematic.move_selection_with_rubber_band_resolved(delta, |component| {
                        symbol_context.terminal_points(component)
                    })
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
            with_active_wire_topology(state, |schematic| {
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions)
            });
            // One undo entry for the whole gesture (no-ops deduplicate).
            if state.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            state.dialogs.interaction.vertex_drag_pos = None;
            state.dialogs.interaction.drag.cancel();
        } else if state.dialogs.last_drag_pos.is_some() {
            let automatic_junctions = state
                .schematic
                .document_policy
                .wire_junctions
                .automatic_junctions();
            with_active_wire_topology(state, |schematic| {
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions)
            });
            if state.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            state.dialogs.drag_start = None;
            state.dialogs.last_drag_pos = None;
        } else {
            let left_to_right =
                state.schematic.selection_rect.current.x >= state.schematic.selection_rect.start.x;
            let Some((min_x, min_y, max_x, max_y)) = state.schematic.selection_rect.finish() else {
                return;
            };
            let add_mode =
                ui.input(|i| i.modifiers.ctrl || i.modifiers.shift || i.modifiers.command);
            let enclosed_only = state
                .schematic
                .document_policy
                .selection_crossing
                .enclosed_only(left_to_right);
            select_in_rect_on_active_sheet(
                state,
                symbol_context,
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

fn select_drag_can_start(primary_pan_requested: bool) -> bool {
    !primary_pan_requested
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
                state.schematic.cancel_tool();
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
        state.schematic.cancel_tool();
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
    if state.schematic_edit_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Interface port was not placed: the active schematic authority changed; reopen Place pin or port."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    }
    let name = pending.name.clone();
    match state.schematic.place_pending_port(grid_pos, pending) {
        Ok(stable_id) => {
            state.schematic.cancel_tool();
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
            state.schematic.cancel_tool();
        }
    }
}

fn place_pending_design_note(state: &mut AppState, grid_pos: Point) {
    let Some(pending) = state.schematic.pending_design_note.clone() else {
        state.push_user_message(ConsoleMessage::warning(
            "Design-note placement requires a validated documentation contract; reopen Place text or note."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
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
    if state.schematic_edit_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Design note was not placed: the active schematic authority changed; reopen Place text or note."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    }
    let kind = pending.kind.label();
    match state.schematic.place_pending_design_note(grid_pos, pending) {
        Ok(stable_id) => {
            state.schematic.cancel_tool();
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Placed {kind} as stable non-electrical object {stable_id}."
            )));
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Design note was not placed: {error}"
            )));
            state.schematic.cancel_tool();
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
        state.schematic.cancel_tool();
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
    if state.schematic_edit_read_only() || !authority_matches {
        state.push_user_message(ConsoleMessage::warning(
            "Documentation shape was not placed: the active schematic authority changed; reopen Draw documentation shape."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
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
            .map(|position| resolve_grid_pointer(state, viewport, position).snapped_position)
            .or_else(|| {
                state
                    .schematic
                    .documentation_shape_drawing
                    .points
                    .last()
                    .copied()
            })
            .unwrap_or_else(Point::origin);
        let step =
            if state.schematic.snap_engine.enabled && state.schematic.snap_engine.snap_to_grid {
                grid_size.max(1)
            } else {
                1
            };
        let drawing = &mut state.schematic.documentation_shape_drawing;
        let mut cursor = drawing.keyboard_cursor.unwrap_or(fallback);
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
                .map(|position| resolve_grid_pointer(state, viewport, position).snapped_position)
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
            state.schematic.cancel_tool();
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
                state.schematic.cancel_tool();
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
    let grid_size = state.schematic.grid_size;
    let active_wires = objects_on_active_sheet(state, &state.schematic.wires, |item| item.id);
    let mut hit_schematic = crate::state::SchematicState::default();
    hit_schematic.wires = active_wires.into_owned();
    let Some(target) = hit_schematic.nearest_junction_candidate(requested, grid_size) else {
        return JunctionPlacementOutcome::NoIntersection;
    };

    let buses = objects_on_active_sheet(state, &state.schematic.buses, |item| item.id);
    if buses.iter().any(|bus| bus.contains_point(target)) {
        return JunctionPlacementOutcome::MixedBus;
    }

    if let Some(junction_id) = active_junction_at(state, target) {
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
    Probe(u64),
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
    pointer_target_with_filter(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
        state.ui.schematic_selection_filter,
    )
}

fn pointer_target_with_filter(
    state: &AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
    filter: crate::state::SchematicSelectionFilter,
) -> Option<PointerTarget> {
    let notes = visible_design_notes(state);
    let labels = objects_on_active_sheet(state, &state.schematic.net_labels, |item| item.id);
    let components = objects_on_active_sheet(state, &state.schematic.components, |item| item.id);
    let taps = objects_on_active_sheet(state, &state.schematic.bus_taps, |item| item.id);
    let buses = objects_on_active_sheet(state, &state.schematic.buses, |item| item.id);
    let shapes =
        objects_on_active_sheet(state, &state.schematic.documentation_shapes, |item| item.id);
    let probes = objects_on_active_sheet(state, &state.schematic.probes, |item| item.id);
    if filter.annotations
        && let Some(id) = probe_at_screen(viewport, probes.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::Probe(id));
    }
    if filter.annotations
        && let Some(id) = design_note_at(ctx, viewport, notes.as_ref(), state, pointer_pos)
    {
        return Some(PointerTarget::DesignNote(id));
    }
    if filter.labels
        && let Some(id) = net_label_at(ctx, viewport, labels.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::NetLabel(id));
    }
    if filter.instances
        && let Some(id) = symbol_context.component_at_resolved_symbol(components.as_ref(), hit.grid)
    {
        return Some(PointerTarget::Component(id));
    }
    if filter.wires {
        if let Some(id) = bus_tap_at(taps.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::BusTap(id));
        }
        if active_junction_at(state, hit.grid).is_some() {
            return Some(PointerTarget::Junction(hit.grid));
        }
        if let Some(hit) = nearest_bus_hit(buses.as_ref(), hit.schematic, hit_radius) {
            return Some(PointerTarget::Bus(hit.bus_id));
        }
        if let Some(id) = active_wire_at(state, hit.grid) {
            return Some(PointerTarget::Wire(id));
        }
    }
    if filter.annotations
        && let Some(id) = documentation_shape_at(viewport, shapes.as_ref(), pointer_pos)
    {
        return Some(PointerTarget::DocumentationShape(id));
    }
    None
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
    // Ctrl, Shift, and the platform command modifier extend the selection;
    // a plain click replaces it.
    let additive = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift || i.modifiers.command);
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
        Some(PointerTarget::Probe(id)) => {
            state.schematic.net_highlight.clear();
            if additive {
                state.schematic.selection.toggle_probe(id);
            } else {
                state.schematic.selection.select_only_probe(id);
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProbeSignalOutcome {
    WaveformShown,
    WaveformHidden,
    WaveformAlreadyVisible,
    GroundReference,
    SavedOutputCreated { plan_name: String },
    SavedOutputAlreadyPresent { plan_name: String },
    Rejected { reason: String },
}

/// Toggle an already-materialized trace and return its resulting visibility.
///
/// Comparing visibility before and after the simulation state's canonical
/// resolver keeps this path correct for every supported waveform alias,
/// including bare net names and generated numeric node names.
fn toggle_materialized_waveform(state: &mut AppState, probe_name: &str) -> Option<bool> {
    let before = state
        .simulation
        .waveforms
        .iter()
        .map(|waveform| waveform.visible)
        .collect::<Vec<_>>();
    if !state.simulation.toggle_waveform_visibility(probe_name) {
        return None;
    }
    state
        .simulation
        .waveforms
        .iter()
        .zip(before)
        .find_map(|(waveform, was_visible)| {
            (waveform.visible != was_visible).then_some(waveform.visible)
        })
}

fn raw_output_expression_key(expression: &str) -> String {
    expression
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn is_ground_voltage_expression(expression: &str) -> bool {
    raw_output_expression_key(expression) == "v(0)"
}

fn unique_probe_output_name(outputs: &[SavedOutput], preferred: &str) -> String {
    if preferred.len() <= 256
        && !outputs
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(preferred))
    {
        return preferred.to_owned();
    }

    for ordinal in 1..=outputs.len().saturating_add(1) {
        let candidate = format!("Schematic probe {ordinal}");
        if !outputs
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("one more generated name than existing outputs must be available")
}

fn select_materialized_probe_trace(state: &mut AppState, probe_name: &str) {
    let selected = {
        let run = state.simulation.active_run();
        let analysis = state.simulation.active_analysis();
        run.zip(analysis).and_then(|(run, analysis)| {
            let analysis_index = run
                .analyses
                .iter()
                .position(|candidate| std::ptr::eq(candidate, analysis))?;
            let waveform_index = analysis.waveforms.iter().position(|waveform| {
                raw_output_expression_key(&waveform.name) == raw_output_expression_key(probe_name)
                    || waveform.name.eq_ignore_ascii_case(probe_name)
            })?;
            crate::workbench::documents::result_document::SelectedResultTrace::from_run_indices(
                run,
                analysis_index,
                waveform_index,
            )
        })
    };
    if let Some(selected) = selected {
        state.ui.results.selected_trace = Some(selected);
    }
}

#[derive(Debug, Clone)]
struct ProbeOutputBinding {
    plan_name: String,
    created: bool,
}

fn ensure_plan_probe_output(
    state: &mut AppState,
    expression: &str,
) -> Result<ProbeOutputBinding, String> {
    let mut setup = state.sim_setup.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    let plan_name = setup.active_plan_name().to_string();
    let expression = expression.trim();
    let expression_key = raw_output_expression_key(expression);
    if state
        .workspace
        .plan_data(plan_id)
        .and_then(|payload| {
            payload.saved_outputs.iter().find_map(|output| {
                (output.kind == SavedOutputKind::RawVoltageOrCurrent
                    && raw_output_expression_key(&output.source_expression) == expression_key)
                    .then_some(output.id)
            })
        })
        .is_some()
    {
        return Ok(ProbeOutputBinding {
            plan_name,
            created: false,
        });
    }

    let output_name = unique_probe_output_name(
        state
            .workspace
            .plan_data(plan_id)
            .map_or(&[], |payload| payload.saved_outputs.as_slice()),
        expression,
    );
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        output_name,
        expression,
        SavedOutputCompatibility::AllCompatibleAnalyses,
        // A schematic probe is the ordinary design-to-results path. It must
        // compile into a storage-bounded request for every default analysis.
        // Transient preparation maps this policy onto the configured output
        // grid and retains the exact final point.
        SavedOutputPolicy::SelectedAndFinalPoints,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )?
    .with_origin(crate::state::SavedOutputOrigin::SchematicProbe);
    let preflight =
        crate::simulation::SimulationController::new().saved_output_preflight(state, &output);
    if let crate::simulation::SavedOutputSemanticStatus::Invalid { reason } =
        preflight.semantic_status()
    {
        return Err(format!(
            "the active plan cannot materialize this probe: {reason}"
        ));
    }

    let mut workspace = state.workspace.clone();
    workspace
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Added schematic probe output {expression}."
        ))
        .map_err(|error| error.to_string())?;

    state.sim_setup = setup;
    state.workspace = workspace;
    state.workbench.preflight.invalidate();
    state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(receipt.status_line());
    Ok(ProbeOutputBinding {
        plan_name,
        created: true,
    })
}

/// Resolve a probe into either an immediate plot transaction or a durable
/// plan-owned output request.
///
/// The saved-output fallback publishes a cloned, fully validated workspace
/// and plan setup together. Failures therefore leave the live configuration
/// unchanged. Existing raw outputs are identified by their source expression,
/// making repeated probes idempotent even if an output was named elsewhere.
fn request_probe_signal(
    state: &mut AppState,
    waveform_name: &str,
    expression: &str,
) -> ProbeSignalOutcome {
    if is_ground_voltage_expression(expression) {
        return ProbeSignalOutcome::GroundReference;
    }
    let binding = match ensure_plan_probe_output(state, expression) {
        Ok(binding) => binding,
        Err(reason) => return ProbeSignalOutcome::Rejected { reason },
    };
    if let Some(visible) = toggle_materialized_waveform(state, waveform_name) {
        select_materialized_probe_trace(state, expression);
        return if visible {
            ProbeSignalOutcome::WaveformShown
        } else {
            ProbeSignalOutcome::WaveformHidden
        };
    }
    if binding.created {
        ProbeSignalOutcome::SavedOutputCreated {
            plan_name: binding.plan_name,
        }
    } else {
        ProbeSignalOutcome::SavedOutputAlreadyPresent {
            plan_name: binding.plan_name,
        }
    }
}

fn request_probe_signal_visible(
    state: &mut AppState,
    waveform_name: &str,
    expression: &str,
) -> ProbeSignalOutcome {
    if is_ground_voltage_expression(expression) {
        return ProbeSignalOutcome::GroundReference;
    }
    let binding = match ensure_plan_probe_output(state, expression) {
        Ok(binding) => binding,
        Err(reason) => return ProbeSignalOutcome::Rejected { reason },
    };
    match state.simulation.ensure_waveform_visible(waveform_name) {
        Some(true) => {
            select_materialized_probe_trace(state, expression);
            ProbeSignalOutcome::WaveformShown
        }
        Some(false) => {
            select_materialized_probe_trace(state, expression);
            ProbeSignalOutcome::WaveformAlreadyVisible
        }
        None if binding.created => ProbeSignalOutcome::SavedOutputCreated {
            plan_name: binding.plan_name,
        },
        None => ProbeSignalOutcome::SavedOutputAlreadyPresent {
            plan_name: binding.plan_name,
        },
    }
}

/// Resolve a probed signal and report the exact committed outcome.
///
/// The canvas probe tool and the inspector's plot action commit the same
/// transaction, so a net can never be "plotted" by one surface and absent
/// from the other. Materialized data plots immediately; otherwise the active
/// simulation plan receives an idempotent saved-output contract.
pub(crate) fn toggle_probe_with_feedback(
    ui: &Ui,
    state: &mut AppState,
    name: &str,
    display: &str,
) -> bool {
    let outcome = request_probe_signal(state, name, display);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(ui, state, display, outcome);
    configuration_changed
}

/// Inspector/navigation action: reveal a probe without ever toggling a
/// currently visible trace off.
pub(crate) fn ensure_probe_visible_with_feedback(
    ui: &Ui,
    state: &mut AppState,
    name: &str,
    display: &str,
) -> bool {
    let outcome = request_probe_signal_visible(state, name, display);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(ui, state, display, outcome);
    configuration_changed
}

/// Reveal already-retained evidence without authoring a future output. This
/// path keeps cross-probing useful for read-only library/testbench views while
/// preserving their write boundary.
pub(crate) fn ensure_retained_probe_visible_with_feedback(
    ui: &Ui,
    state: &mut AppState,
    name: &str,
    display: &str,
) -> bool {
    let outcome = if is_ground_voltage_expression(display) {
        ProbeSignalOutcome::GroundReference
    } else {
        match state.simulation.ensure_waveform_visible(name) {
            Some(true) => {
                select_materialized_probe_trace(state, display);
                ProbeSignalOutcome::WaveformShown
            }
            Some(false) => {
                select_materialized_probe_trace(state, display);
                ProbeSignalOutcome::WaveformAlreadyVisible
            }
            None => ProbeSignalOutcome::Rejected {
                reason: "no retained compatible waveform is available in this read-only view"
                    .to_owned(),
            },
        }
    };
    let shown = matches!(
        &outcome,
        ProbeSignalOutcome::WaveformShown | ProbeSignalOutcome::WaveformAlreadyVisible
    );
    report_probe_outcome(ui, state, display, outcome);
    shown
}

fn report_probe_outcome(ui: &Ui, state: &mut AppState, display: &str, outcome: ProbeSignalOutcome) {
    match outcome {
        ProbeSignalOutcome::WaveformShown => {
            let message = format!("{display} added to plot");
            state
                .ui
                .toasts
                .success(ui.ctx(), "Trace shown", format!("{message}."));
            state.push_user_message(ConsoleMessage::info(message));
        }
        ProbeSignalOutcome::WaveformHidden => {
            let message = format!("{display} removed from plot");
            state
                .ui
                .toasts
                .success(ui.ctx(), "Trace hidden", format!("{message}."));
            state.push_user_message(ConsoleMessage::info(message));
        }
        ProbeSignalOutcome::WaveformAlreadyVisible => {
            let message = format!("{display} is already visible in the plot");
            state.ui.toasts.info_with_title(
                ui.ctx(),
                "Trace already visible",
                format!("{message}."),
            );
            state.push_user_message(ConsoleMessage::info(message));
        }
        ProbeSignalOutcome::GroundReference => {
            state.ui.toasts.success(
                ui.ctx(),
                "Ground reference selected",
                "Node 0 is the 0 V reference.",
            );
            state.push_user_message(ConsoleMessage::info(
                "Ground node: 0 V reference".to_owned(),
            ));
        }
        ProbeSignalOutcome::SavedOutputCreated { plan_name } => {
            let message = format!(
                "{display} was added to saved outputs for {plan_name}; use Run active plan to materialize and plot it"
            );
            state
                .ui
                .toasts
                .success(ui.ctx(), "Pending next run", format!("{message}."));
            state.push_user_message(ConsoleMessage::info(message));
        }
        ProbeSignalOutcome::SavedOutputAlreadyPresent { plan_name } => {
            let message = format!(
                "{display} is already saved for {plan_name}; use Run active plan to materialize and plot it"
            );
            state
                .ui
                .toasts
                .info_with_title(ui.ctx(), "Pending next run", format!("{message}."));
            state.push_user_message(ConsoleMessage::info(message));
        }
        ProbeSignalOutcome::Rejected { reason } => {
            let message = format!("Could not save {display}: {reason}");
            state.ui.toasts.warn_with_title(
                ui.ctx(),
                "Probe output unavailable",
                format!("{message}."),
            );
            state.push_user_message(ConsoleMessage::warning(message));
        }
    }
}

fn probe_edit_identity_is_current(state: &AppState) -> Result<(), String> {
    if !matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        return Err("the active cell/view is not a schematic document".to_owned());
    }
    if state.workspace.active_schematic_reference() != state.workspace.active_view {
        return Err("the active schematic identity changed before the probe was placed".to_owned());
    }
    if state.workspace.active_schematic().is_none() {
        return Err("the active schematic has no project-owned document buffer".to_owned());
    }
    if state.schematic_edit_read_only() {
        return Err("the active schematic is read-only".to_owned());
    }
    Ok(())
}

fn current_probe_output_binding(
    state: &AppState,
    expression: &str,
) -> Option<(
    crate::product::SimulationPlanId,
    crate::product::SavedOutputId,
)> {
    let plan_id = state.sim_setup.stable_analysis_plan().ok()?.id();
    let expression_key = raw_output_expression_key(expression);
    let output_id = state
        .workspace
        .plan_data(plan_id)?
        .saved_outputs
        .iter()
        .find_map(|output| {
            (output.kind == SavedOutputKind::RawVoltageOrCurrent
                && raw_output_expression_key(&output.source_expression) == expression_key)
                .then_some(output.id)
        })?;
    Some((plan_id, output_id))
}

fn retain_probe_flag(
    state: &mut AppState,
    position: Point,
    source_expression: Option<&str>,
    binding: Option<(
        crate::product::SimulationPlanId,
        crate::product::SavedOutputId,
    )>,
) -> Result<u64, String> {
    probe_edit_identity_is_current(state)?;
    let mut probe_id = 0;
    let source_expression = source_expression.map(str::trim).map(str::to_owned);
    let validation_reference = source_expression.as_deref().unwrap_or("P1");
    SchematicProbe::new(1, position, validation_reference, source_expression.clone())?;
    let source_key = source_expression.as_deref().map(raw_output_expression_key);
    if let Some(existing_id) = state.schematic.probes.iter().find_map(|probe| {
        (probe.position == position
            && probe
                .source_expression
                .as_deref()
                .map(raw_output_expression_key)
                == source_key)
            .then_some(probe.id)
    }) {
        let needs_binding_refresh = state
            .schematic
            .probes
            .iter()
            .find(|probe| probe.id == existing_id)
            .is_some_and(|probe| {
                binding.is_some_and(|(plan_id, output_id)| {
                    probe.plan_id != Some(plan_id) || probe.saved_output_id != Some(output_id)
                })
            });
        if needs_binding_refresh {
            state
                .schematic
                .with_undo("bind schematic probe output", |schematic| {
                    if let Some(probe) = schematic
                        .probes
                        .iter_mut()
                        .find(|probe| probe.id == existing_id)
                        && let Some((plan_id, output_id)) = binding
                    {
                        probe.bind_saved_output(plan_id, output_id);
                        schematic.is_dirty = true;
                    }
                });
            state.sync_active_schematic_to_workspace();
        }
        let id = existing_id;
        state.schematic.selection.select_only_probe(id);
        state.dialogs.interaction.schematic_keyboard_focus = Some(
            crate::workbench::app_state::SchematicKeyboardFocus::Probe(id),
        );
        return Ok(id);
    }
    let changed = state
        .schematic
        .with_undo("place schematic probe", |schematic| {
            let id = schematic.next_id();
            let reference = source_expression
                .clone()
                .unwrap_or_else(|| format!("P{id}"));
            if let Ok(mut probe) =
                SchematicProbe::new(id, position, reference, source_expression.clone())
            {
                if let Some((plan_id, output_id)) = binding {
                    probe.bind_saved_output(plan_id, output_id);
                }
                schematic.probes.push(probe);
                schematic.selection.select_only_probe(id);
                schematic.is_dirty = true;
                probe_id = id;
            }
        });
    if !changed || probe_id == 0 {
        return Err("the probe marker did not change the active schematic".to_owned());
    }
    state.dialogs.interaction.schematic_keyboard_focus = Some(
        crate::workbench::app_state::SchematicKeyboardFocus::Probe(probe_id),
    );
    state.sync_active_schematic_to_workspace();
    Ok(probe_id)
}

/// Nets of the open view as the configured design resolves it.
///
/// A configuration that does not resolve yields no name at all rather than
/// the editor buffer's answer: a probe carries its net name into a run
/// receipt, and a name taken from a hierarchy the design does not have is
/// wrong rather than approximate. The reason is logged so the gesture that
/// found no name is explicable.
fn live_design_nets(state: &AppState) -> std::sync::Arc<Vec<DesignNet>> {
    match state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection_nets(
            &state.library_manager,
            &projection,
            &state.workspace.active_view.key(),
        ),
        Err(error) => {
            log::warn!("Probe naming has no design projection: {error}");
            std::sync::Arc::new(Vec::new())
        }
    }
}

fn exactly_one_net_name<'a>(mut matches: impl Iterator<Item = &'a DesignNet>) -> Option<String> {
    let name = matches.next()?.name.clone();
    matches.next().is_none().then_some(name)
}

/// Resolve the electrical name from the current schematic rather than
/// requiring a retained simulation run. Retained cross-probe data is only a
/// compatibility fallback when the live netlist projection cannot resolve a
/// source identity.
fn live_wire_probe_net_name(state: &AppState, wire_id: u64) -> Option<String> {
    let nets = live_design_nets(state);
    exactly_one_net_name(nets.iter().filter(|net| net.wire_ids.contains(&wire_id)))
}

fn live_terminal_probe_net_name(
    state: &AppState,
    component_id: u64,
    pin: &str,
    position: Point,
) -> Option<String> {
    let nets = live_design_nets(state);
    exactly_one_net_name(nets.iter().filter(|net| {
        net.terminals
            .iter()
            .any(|terminal| terminal.component_id == component_id && terminal.pin == pin)
    }))
    .or_else(|| {
        let wire_id = active_wire_at(state, position)?;
        exactly_one_net_name(nets.iter().filter(|net| net.wire_ids.contains(&wire_id)))
    })
}

fn retained_probe_net_name(state: &AppState, position: Point) -> Option<String> {
    state
        .simulation
        .cross_probe
        .net_at_in(
            &state.workspace.active_view,
            state.schematic.topology_version(),
            position,
        )
        .cloned()
}

fn component_probe_expression(
    state: &AppState,
    component_id: u64,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<String> {
    let component = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    let resolved_symbol = symbol_context.resolved_symbol(component);
    // A cell instance without an authored/resolved symbol has no authoritative
    // pin identity. Its generic two-pin placeholder geometry must never be
    // treated as an electrical source contract for a retained probe.
    if component.kind == ComponentType::CellInstance && resolved_symbol.is_none() {
        return None;
    }
    let terminals = component.terminal_positions_resolved(resolved_symbol);

    // A snapped click on an exact pin is a node-voltage gesture. This matters
    // for unwired/dangling pins too: connectivity can still give that node an
    // authoritative generated name. A body click must never silently turn
    // into the voltage at whichever terminal happened to be nearest.
    if let Some((pin, terminal_position)) = terminals
        .iter()
        .find(|(_, terminal_position)| *terminal_position == grid_pos)
        .map(|(pin, terminal_position)| (pin.as_str(), *terminal_position))
    {
        let net_name = live_terminal_probe_net_name(state, component.id, pin, terminal_position)
            .or_else(|| retained_probe_net_name(state, terminal_position))?;
        return Some(format!("V({net_name})"));
    }

    // Structural objects and synthesized/multi-port blocks do not own one
    // unambiguous device-current observable. Their users must choose a
    // conductor or author an exact winding/lead expression. Ordinary emitted
    // SPICE devices use the conventional positive-reference I(instance)
    // quantity, including multi-terminal devices whose dialect defines that
    // accessor (for example the drain/reference lead of a MOS device).
    if matches!(
        component.kind,
        ComponentType::Ground
            | ComponentType::Port
            | ComponentType::Transformer
            | ComponentType::CoupledInductor
            | ComponentType::CellInstance
    ) || component.kind.is_xspice()
    {
        return None;
    }

    Some(format!("I({})", component.spice_instance_name()))
}

fn handle_probe_click(
    ui: &Ui,
    state: &mut AppState,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    if let Err(reason) = probe_edit_identity_is_current(state) {
        if state.schematic_edit_read_only() {
            state.deny_read_only_edit();
        } else {
            state
                .ui
                .toasts
                .warn_with_title(ui.ctx(), "Probe could not be placed", reason.clone());
            state.push_user_message(ConsoleMessage::warning(reason));
        }
        return;
    }

    if let Some(wire_id) = active_wire_at(state, grid_pos) {
        if let Some(net_name) = live_wire_probe_net_name(state, wire_id)
            .or_else(|| retained_probe_net_name(state, grid_pos))
        {
            log::info!("Probe: clicked net '{}' at {:?}", net_name, grid_pos);

            let display = format!("V({net_name})");
            let outcome = request_probe_signal(state, &net_name, &display);
            let retain_marker = !matches!(
                &outcome,
                ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
            );
            report_probe_outcome(ui, state, &display, outcome);
            let binding = current_probe_output_binding(state, &display);
            if retain_marker
                && let Err(reason) = retain_probe_flag(state, grid_pos, Some(&display), binding)
            {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Probe marker could not be retained",
                    reason.clone(),
                );
                state.push_user_message(ConsoleMessage::warning(reason));
            }
            if net_name != "0"
                && state
                    .ui
                    .preferences
                    .toggle(crate::workbench::TogglePreference::CrossProbeBehavior)
            {
                let wires = objects_on_active_sheet(state, &state.schematic.wires, |item| item.id);
                let junctions =
                    objects_on_active_sheet(state, &state.schematic.junctions, |item| item.id);
                let net_graph = NetGraph::build(wires.as_ref(), junctions.as_ref());
                state
                    .schematic
                    .net_highlight
                    .highlight_net(&net_graph, grid_pos);
            }
        } else {
            log::info!(
                "Probe: wire at {:?} has no unambiguous live or retained net identity",
                grid_pos
            );
            state.ui.toasts.warn_with_title(
                ui.ctx(),
                "Wire has no probeable net",
                "The current schematic connectivity could not resolve this conductor to one net",
            );
            state.push_user_message(ConsoleMessage::warning(
                "Wire has no unambiguous probeable net in the current schematic.".to_string(),
            ));
        }
    } else {
        let components =
            objects_on_active_sheet(state, &state.schematic.components, |item| item.id);
        let Some(comp_id) =
            symbol_context.component_at_resolved_symbol(components.as_ref(), grid_pos)
        else {
            state.schematic.net_highlight.clear();
            match retain_probe_flag(state, grid_pos, None, None) {
                Ok(id) => {
                    let message = format!(
                        "P{id} was added to the saved-output marker set; place it on a conductor to bind an exact signal"
                    );
                    state
                        .ui
                        .toasts
                        .success(ui.ctx(), "Probe placed", format!("{message}."));
                    state.push_user_message(ConsoleMessage::info(message));
                }
                Err(reason) => {
                    state.ui.toasts.warn_with_title(
                        ui.ctx(),
                        "Probe could not be placed",
                        reason.clone(),
                    );
                    state.push_user_message(ConsoleMessage::warning(reason));
                }
            }
            return;
        };
        handle_component_probe(ui, state, comp_id, grid_pos, symbol_context);
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

        let Some(probe_name) = component_probe_expression(state, comp_id, grid_pos, symbol_context)
        else {
            let message = format!(
                "{comp_name} has no unambiguous current at that body location; probe an exact terminal or conductor for voltage, or add an exact lead/winding current expression in Saved outputs"
            );
            state.ui.toasts.warn_with_title(
                ui.ctx(),
                "Component current is ambiguous",
                format!("{message}."),
            );
            state.push_user_message(ConsoleMessage::warning(message));
            return;
        };

        let display = probe_name.clone();
        let outcome = request_probe_signal(state, &probe_name, &display);
        let retain_marker = !matches!(
            &outcome,
            ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
        );
        report_probe_outcome(ui, state, &display, outcome);
        let binding = current_probe_output_binding(state, &display);
        if retain_marker
            && let Err(reason) = retain_probe_flag(state, grid_pos, Some(&display), binding)
        {
            state.ui.toasts.warn_with_title(
                ui.ctx(),
                "Probe marker could not be retained",
                reason.clone(),
            );
            state.push_user_message(ConsoleMessage::warning(reason));
        }
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
        Some(PointerTarget::Probe(id)) => {
            state.schematic.selection.select_only_probe(id);
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
    crate::workbench::app::open_selected_object_properties(state);
}

#[cfg(test)]
mod tests;
