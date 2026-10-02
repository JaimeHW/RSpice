//! Canvas pointer and keyboard interaction.
//!
//! Resolves what is under the pointer, then dispatches to the operation the
//! armed tool implies. Hit resolution is ordered — a terminal beats a wire
//! vertex, which beats the wire body — so a click near a junction does what
//! the designer meant.

use egui::{Response, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    ComponentType, OccurrenceProbeSpelling, Point, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe,
    Tool, ViewType,
};
use crate::workbench::app_state::AppState;
use rspice_design::connectivity::summary::{DesignNet, projection_nets};
use rspice_schematic_editor::view::documentation_shape_input::{
    self, ShapeInputAction, ShapeInputRequest, ShapeInputTransition, ShapeInputView,
};

use super::SchematicSymbolContext;
use super::array_interaction::handle_armed_array_selection;
use super::bus_interaction::{BusTapCandidateError, resolve_bus_tap_candidate_on_active_sheet};
use super::coordinates::screen_to_schematic;
use super::drawing::WireScreenHit;
use super::move_interaction::handle_armed_move_selection;
use super::navigation::primary_pan_gesture_active;
use super::selection_drag::handle_select_dragging;
use super::sheet_visibility::{
    active_junction_at, active_wire_at, objects_on_active_sheet, retain_selection_on_active_sheet,
};
use super::snap_resolution::{
    nearest_active_wire_screen_hit, resolve_grid_pointer, resolve_target_pointer,
    target_acquisition_radius,
};
use super::stretch_interaction::handle_armed_stretch_selection;
use super::viewport::Viewport;
#[cfg(test)]
use rspice_schematic_editor::view::selection_drag::{
    select_drag_can_start, select_drag_is_authorized,
};

mod pin_placement;
mod pointer_target;
use pin_placement::place_pending_port;
use pointer_target::pointer_target_with_filter;
pub(super) use pointer_target::{PointerHit, PointerTarget, pointer_target};

pub(super) fn handle_tool_interactions(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.reconcile_schematic_drag(ui.ctx(), false) {
        return;
    }
    retain_selection_on_active_sheet(state);
    if primary_pan_gesture_active(ui, response) {
        return;
    }
    let grid_size = state.schematic.document().grid_size;
    let current_tool = state.schematic.session.editor.tool;
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
        && (state.schematic.session.editor.wire_drawing.active
            || state.schematic.session.editor.bus_drawing.active);
    let route_enter = response.has_focus()
        && (state.schematic.session.editor.wire_drawing.active
            || state.schematic.session.editor.bus_drawing.active)
        && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    if route_double_click || route_enter {
        finish_active_route(ui, state);
        return;
    }

    if current_tool == Tool::DocumentationShape
        && let Some(transition) = documentation_shape_input::pointer_motion(
            ui,
            response,
            viewport,
            shape_input_view(state),
        )
    {
        let request = capture_shape_input(state, transition);
        apply_shape_input(ui, state, request);
    }

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::MoveSelection {
        handle_armed_move_selection(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::StretchSelection {
        handle_armed_stretch_selection(ui, response, state, viewport, symbol_context);
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
                    Some(wire_pos) if state.schematic.session.editor.wire_drawing.active => {
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
                if state.schematic.session.editor.bus_drawing.active {
                    state.schematic.extend_bus(bus_pos);
                } else {
                    let declaration = state
                        .schematic
                        .session
                        .editor
                        .bus_drawing
                        .declaration
                        .clone();
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
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
                state.open_selected_instance_master();
            }
            SelectDoubleClickAction::OpenVerilogA(id) => {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
                let _ = state.open_veriloga_source_for_component(id);
            }
            SelectDoubleClickAction::ActivateRequirement(id) => {
                if activate_requirement_link(state, id, ui.ctx()) {
                    state
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_only_design_note(id);
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

    if state.schematic.session.editor.tool == Tool::DocumentationShape && response.has_focus() {
        handle_documentation_shape_keyboard(ui, response, state, viewport, grid_size);
    }
}

fn finish_active_route(ui: &Ui, state: &mut AppState) -> bool {
    if state.schematic.session.editor.wire_drawing.active {
        return state.schematic.finish_wire().is_some();
    }
    if state.schematic.session.editor.bus_drawing.active {
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
        // Ascend belongs to the document under the pointer, so the depth read
        // is the open document's own occurrence rather than the session-global
        // breadcrumb, which describes whichever document was activated last.
        None if canvas_is_empty_at_pointer && state.workspace.content.occurrence_depth() > 1 => {
            SelectDoubleClickAction::Ascend
        }
        None => SelectDoubleClickAction::None,
    }
}

fn activate_requirement_link(state: &mut AppState, note_id: u64, ctx: &egui::Context) -> bool {
    let target = state
        .schematic
        .document()
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
    let Some(pending) = state.schematic.session.editor.pending_bus_tap.clone() else {
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

fn place_component(state: &mut AppState, component_type: ComponentType, grid_pos: Point) {
    match component_type {
        ComponentType::Port => place_pending_port(state, grid_pos),
        ComponentType::CellInstance => {
            let Some(library_cell) = state.schematic.session.editor.pending_library_cell.clone()
            else {
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
        // An armed stimulus definition is placed as an adopter in this undo group.
        _ => {
            let changed = state.schematic.with_undo(
                format!("place {}", component_type.display_name()),
                |schematic| {
                    schematic.add_armed_component(component_type, grid_pos);
                },
            );
            if changed {
                log::info!("Placed {:?} at {:?}", component_type, grid_pos);
            }
        }
    }
}

fn place_pending_design_note(state: &mut AppState, grid_pos: Point) {
    let Some(pending) = state.schematic.session.editor.pending_design_note.clone() else {
        state.push_user_message(ConsoleMessage::warning(
            "Design-note placement requires a validated documentation contract; reopen Place text or note."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    };
    let authority_matches = pending
        .source
        .as_ref()
        .is_some_and(|source| *source == super::requests::editor_request_source(state));
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
    state
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .keyboard_cursor = Some(grid_pos);
    state
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .keyboard_active = false;
    let Some(pending) = state
        .schematic
        .session
        .editor
        .pending_documentation_shape
        .as_ref()
    else {
        state.push_user_message(ConsoleMessage::warning(
            "Documentation-shape placement requires a validated graphics contract; reopen Draw documentation shape."
                .to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    };
    let authority_matches = pending
        .source
        .as_ref()
        .is_some_and(|source| *source == super::requests::editor_request_source(state));
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
        .session
        .editor
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

fn shape_input_view(state: &AppState) -> ShapeInputView<'_> {
    ShapeInputView {
        drawing: &state.schematic.session.editor.documentation_shape_drawing,
        kind: state
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .as_ref()
            .map(|pending| pending.kind),
        snap_engine: &state.schematic.session.editor.snap_engine,
        grid_size: state.schematic.document().grid_size,
    }
}

fn handle_documentation_shape_keyboard(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
) {
    if let Some(transition) = documentation_shape_input::keyboard(
        ui,
        response,
        viewport,
        shape_input_view(state),
        grid_size,
    ) {
        let request = capture_shape_input(state, transition);
        apply_shape_input(ui, state, request);
    }
}

fn capture_shape_input(state: &AppState, transition: ShapeInputTransition) -> ShapeInputRequest {
    ShapeInputRequest {
        source: super::requests::editor_request_source(state),
        kind: state
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .as_ref()
            .map(|pending| pending.kind),
        transition,
    }
}

fn apply_shape_input(ui: &Ui, state: &mut AppState, request: ShapeInputRequest) {
    if request.source != super::requests::editor_request_source(state)
        || request.kind
            != state
                .schematic
                .session
                .editor
                .pending_documentation_shape
                .as_ref()
                .map(|pending| pending.kind)
        || request.transition.expected != state.schematic.session.editor.documentation_shape_drawing
        || state.schematic.session.editor.tool != Tool::DocumentationShape
        || state.application_modal_open()
    {
        return;
    }
    state.schematic.session.editor.documentation_shape_drawing = request.transition.next;
    match request.transition.action {
        Some(ShapeInputAction::PlacePoint(cursor)) => {
            handle_documentation_shape_click(ui, state, cursor, false);
            if state.schematic.session.editor.tool == Tool::DocumentationShape {
                state
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing
                    .keyboard_active = true;
            }
        }
        Some(ShapeInputAction::FinishPolygon) => finish_documentation_polygon(ui, state),
        None => {}
    }
}

fn finish_documentation_polygon(ui: &Ui, state: &mut AppState) {
    let Some(kind) = state
        .schematic
        .session
        .editor
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
    let geometry = match state
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .geometry(kind)
    {
        Ok(geometry) => geometry,
        Err(error) => {
            report_documentation_shape_error(ui, state, error);
            return;
        }
    };
    let Some(pending) = state
        .schematic
        .session
        .editor
        .pending_documentation_shape
        .clone()
    else {
        return;
    };
    let authority_error = if state.schematic_edit_read_only() {
        Some(crate::state::DocumentationShapeError::ReadOnly)
    } else if pending.source.as_ref() != Some(&super::requests::editor_request_source(state)) {
        Some(crate::state::DocumentationShapeError::StaleDocument)
    } else {
        None
    };
    if let Some(error) = authority_error {
        report_documentation_shape_error(ui, state, error);
        state.schematic.cancel_tool();
        return;
    }
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
    let grid_size = state.schematic.document().grid_size;
    let Some(target) =
        super::schematic_design_view(state).nearest_junction_candidate(requested, grid_size)
    else {
        return JunctionPlacementOutcome::NoIntersection;
    };

    let buses = objects_on_active_sheet(state, &state.schematic.document().buses, |item| item.id);
    if buses.iter().any(|bus| bus.contains_point(target)) {
        return JunctionPlacementOutcome::MixedBus;
    }

    if let Some(junction_id) = active_junction_at(state, target) {
        state.schematic.with_undo("remove junction", |schematic| {
            schematic.remove_junction(junction_id);
        });
        state.schematic.session.editor.net_highlight.clear();
        return JunctionPlacementOutcome::Removed(target);
    }

    state.schematic.with_undo("place junction", |schematic| {
        schematic.add_junction(target);
    });
    state.schematic.session.editor.net_highlight.clear();
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

    let target = pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ui.ctx(),
        viewport,
        pointer_pos,
    );
    let request = rspice_schematic_editor::requests::EditorRequest {
        source: super::requests::editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        action: rspice_schematic_editor::requests::EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        },
    };
    super::requests::apply_editor_request(state, request);
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
    spelling: &OccurrenceProbeSpelling,
) -> Result<ProbeOutputBinding, String> {
    let mut setup = state.sim_setup.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    let plan_name = setup.active_plan_name().to_string();
    let expression = spelling.engine().trim();
    let expression_key = raw_output_expression_key(expression);
    if state
        .workspace
        .content
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
            .content
            .plan_data(plan_id)
            .map_or(&[], |payload| payload.saved_outputs.as_slice()),
        spelling.display().trim(),
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
    if let rspice_simulation::output_contract::SavedOutputSemanticStatus::Invalid { reason } =
        preflight.semantic_status()
    {
        return Err(format!(
            "the active plan cannot materialize this probe: {reason}"
        ));
    }

    let mut workspace = state.workspace.clone();
    workspace
        .content
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    workspace
        .content
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
    state.record_plan_receipt(receipt.status_line());
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
/// That expression is the engine's, never the reader's: two instances of one
/// master show the same `V(n1)` and request two different nodes.
fn request_probe_signal(
    state: &mut AppState,
    spelling: &OccurrenceProbeSpelling,
) -> ProbeSignalOutcome {
    let engine = spelling.engine();
    if is_ground_voltage_expression(engine) {
        return ProbeSignalOutcome::GroundReference;
    }
    let binding = match ensure_plan_probe_output(state, spelling) {
        Ok(binding) => binding,
        Err(reason) => return ProbeSignalOutcome::Rejected { reason },
    };
    if let Some(visible) = toggle_materialized_waveform(state, engine) {
        select_materialized_probe_trace(state, engine);
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
    spelling: &OccurrenceProbeSpelling,
) -> ProbeSignalOutcome {
    let engine = spelling.engine();
    if is_ground_voltage_expression(engine) {
        return ProbeSignalOutcome::GroundReference;
    }
    let binding = match ensure_plan_probe_output(state, spelling) {
        Ok(binding) => binding,
        Err(reason) => return ProbeSignalOutcome::Rejected { reason },
    };
    match state.simulation.ensure_waveform_visible(engine) {
        Some(true) => {
            select_materialized_probe_trace(state, engine);
            ProbeSignalOutcome::WaveformShown
        }
        Some(false) => {
            select_materialized_probe_trace(state, engine);
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

/// Re-spell a probe request at the occurrence the active tab is editing.
///
/// `display` is the expression the caller means and `name` is the local leaf
/// inside it. The two only compose when `display` is exactly `V(name)` or
/// `I(name)`; anything else — a marker's stored expression, an authored
/// quantity — already names an exact node and is used as written.
fn probe_spelling_for(
    state: &AppState,
    name: &str,
    display: &str,
) -> Result<OccurrenceProbeSpelling, String> {
    let Some(quantity) = ['V', 'I'].into_iter().find(|quantity| {
        super::wrapped_signal_name(display, *quantity).is_some_and(|leaf| leaf == name)
    }) else {
        return Ok(OccurrenceProbeSpelling::verbatim(display));
    };
    let occurrence = state.workspace.content.occurrence_path();
    OccurrenceProbeSpelling::for_leaf(&occurrence, quantity, name).ok_or_else(|| {
        format!("instance path {occurrence} has no name the engine can be asked for")
    })
}

/// Resolve a probe at the active occurrence and commit it through `request`.
///
/// The spelling comes back with the outcome because every caller reports the
/// result to the reader, and the reader is owed the design's address rather
/// than the flattened node the engine answered on.
fn commit_probe_request(
    state: &mut AppState,
    name: &str,
    display: &str,
    request: fn(&mut AppState, &OccurrenceProbeSpelling) -> ProbeSignalOutcome,
) -> (OccurrenceProbeSpelling, ProbeSignalOutcome) {
    match probe_spelling_for(state, name, display) {
        Ok(spelling) => {
            let outcome = request(state, &spelling);
            (spelling, outcome)
        }
        Err(reason) => (
            OccurrenceProbeSpelling::verbatim(display),
            ProbeSignalOutcome::Rejected { reason },
        ),
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
    let (spelling, outcome) = commit_probe_request(state, name, display, request_probe_signal);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(ui, state, spelling.display(), outcome);
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
    let (spelling, outcome) =
        commit_probe_request(state, name, display, request_probe_signal_visible);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(ui, state, spelling.display(), outcome);
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
    let spelling = match probe_spelling_for(state, name, display) {
        Ok(spelling) => spelling,
        Err(reason) => {
            report_probe_outcome(ui, state, display, ProbeSignalOutcome::Rejected { reason });
            return false;
        }
    };
    let engine = spelling.engine().to_owned();
    let outcome = if is_ground_voltage_expression(&engine) {
        ProbeSignalOutcome::GroundReference
    } else {
        match state.simulation.ensure_waveform_visible(&engine) {
            Some(true) => {
                select_materialized_probe_trace(state, &engine);
                ProbeSignalOutcome::WaveformShown
            }
            Some(false) => {
                select_materialized_probe_trace(state, &engine);
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
    report_probe_outcome(ui, state, spelling.display(), outcome);
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
        state.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        return Err("the active cell/view is not a schematic document".to_owned());
    }
    if state.workspace.content.active_schematic_reference() != state.workspace.content.active_view {
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
        .content
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

/// Retain the marker for a probe.
///
/// The marker's reference is the reader's spelling and its source expression is
/// the engine's, because the label is read on the drawing while the expression
/// is matched against a plan's saved outputs.
fn retain_probe_flag(
    state: &mut AppState,
    position: Point,
    spelling: Option<&OccurrenceProbeSpelling>,
    binding: Option<(
        crate::product::SimulationPlanId,
        crate::product::SavedOutputId,
    )>,
) -> Result<u64, String> {
    probe_edit_identity_is_current(state)?;
    let source_expression = spelling.map(|spelling| spelling.engine().trim().to_owned());
    let label = spelling.map(|spelling| spelling.display().trim().to_owned());
    let validation_reference = label.as_deref().unwrap_or("P1");
    SchematicProbe::new(1, position, validation_reference, source_expression.clone())?;
    let source_key = source_expression.as_deref().map(raw_output_expression_key);
    if let Some(existing_id) = state.schematic.document().probes.iter().find_map(|probe| {
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
            .document()
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
                .bind_probe_saved_output(existing_id, binding);
            state.sync_active_schematic_to_workspace();
        }
        let id = existing_id;
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_probe(id);
        state.dialogs.interaction.schematic_keyboard_focus = Some(
            crate::workbench::app_state::SchematicKeyboardFocus::Probe(id),
        );
        return Ok(id);
    }
    let probe_id =
        state
            .schematic
            .place_schematic_probe(position, label, source_expression, binding)?;
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
        &state.workspace.content.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection_nets(
            state.library_manager.catalog(),
            &projection,
            &state.workspace.content.active_view.key(),
        ),
        Err(error) => {
            log::warn!("Probe naming has no design projection: {error}");
            std::sync::Arc::new(Vec::new())
        }
    }
}

/// Light the net a canvas gesture selects, under the name the deck gives it.
///
/// The canvas traces nothing of its own: two separated wire groups one label
/// or one interface port joins are one net to the netlister, so they light
/// together and carry the node name a probe would emit. A gesture the design
/// resolves no net for lights nothing rather than falling back to whichever
/// group the pointer happened to land on.
pub(super) fn highlight_canvas_net(state: &mut AppState, select: impl Fn(&DesignNet) -> bool) {
    let lit = live_design_nets(state)
        .iter()
        .find(|net| select(net))
        .map(|net| {
            let wire_ids: std::collections::HashSet<u64> = net.wire_ids.iter().copied().collect();
            (net.name.clone(), wire_ids)
        });
    match lit {
        Some((name, wire_ids)) => {
            log::info!("Highlighted net '{name}' with {} wires", wire_ids.len());
            state
                .schematic
                .session
                .editor
                .net_highlight
                .highlight_named_wires(name, wire_ids);
        }
        None => state.schematic.session.editor.net_highlight.clear(),
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
            &state.workspace.content.active_view,
            state.schematic.topology_version(),
            position,
        )
        .cloned()
}

/// The quantity and the local leaf a click on a component probes, unscoped.
///
/// The occurrence is applied later, at the request boundary: what the drawing
/// knows is `n1` and `R2`, and what makes those addresses is the tab.
fn component_probe_expression(
    state: &AppState,
    component_id: u64,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<(char, String)> {
    let component = state
        .schematic
        .document()
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
        return Some(('V', net_name));
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

    Some(('I', component.spice_instance_name()))
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
            let (spelling, outcome) =
                commit_probe_request(state, &net_name, &display, request_probe_signal);
            let retain_marker = !matches!(
                &outcome,
                ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
            );
            report_probe_outcome(ui, state, spelling.display(), outcome);
            let binding = current_probe_output_binding(state, spelling.engine());
            if retain_marker
                && let Err(reason) = retain_probe_flag(state, grid_pos, Some(&spelling), binding)
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
                highlight_canvas_net(state, |net| net.name.eq_ignore_ascii_case(&net_name));
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
            objects_on_active_sheet(state, &state.schematic.document().components, |item| {
                item.id
            });
        let Some(comp_id) =
            symbol_context.component_at_resolved_symbol(components.as_ref(), grid_pos)
        else {
            state.schematic.session.editor.net_highlight.clear();
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
    if let Some(component) = state
        .schematic
        .document()
        .components
        .iter()
        .find(|c| c.id == comp_id)
    {
        let comp_name = component.name.clone();
        log::info!(
            "Probe: clicked component '{}' ({})",
            comp_name,
            component.kind.display_name()
        );

        let Some((quantity, leaf)) =
            component_probe_expression(state, comp_id, grid_pos, symbol_context)
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

        let display = format!("{quantity}({leaf})");
        let (spelling, outcome) =
            commit_probe_request(state, &leaf, &display, request_probe_signal);
        let retain_marker = !matches!(
            &outcome,
            ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
        );
        report_probe_outcome(ui, state, spelling.display(), outcome);
        let binding = current_probe_output_binding(state, spelling.engine());
        if retain_marker
            && let Err(reason) = retain_probe_flag(state, grid_pos, Some(&spelling), binding)
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
    let target = pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    );
    let selection = &mut state.schematic.session.editor.selection;
    match target {
        Some(PointerTarget::Component(id)) => selection.select_only_component(id),
        Some(PointerTarget::DesignNote(id)) => selection.select_only_design_note(id),
        Some(PointerTarget::DocumentationShape(id)) => {
            selection.select_only_documentation_shape(id)
        }
        Some(PointerTarget::Probe(id)) => selection.select_only_probe(id),
        Some(PointerTarget::NetLabel(id)) => selection.select_only_net_label(id),
        Some(PointerTarget::BusTap(id)) => selection.select_only_bus_tap(id),
        Some(PointerTarget::Bus(id)) => selection.select_only_bus(id),
        Some(PointerTarget::Junction(_)) | Some(PointerTarget::Wire(_)) | None => return,
    }
    crate::workbench::app::open_selected_object_properties(state);
}

#[cfg(test)]
mod tests;
