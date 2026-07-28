//! Exclusive canvas interaction for the transactional Stretch command.

use egui::{Response, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::{Point, StretchTarget};
use crate::workbench::app_state::AppState;

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::sheet_visibility::object_is_on_active_sheet;
use super::viewport::Viewport;

const DELTA_OVERFLOW: &str = "The requested stretch exceeds the schematic coordinate range.";

pub(super) fn handle_armed_stretch_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
    symbol_context: &SchematicSymbolContext,
) {
    if let Err(message) = crate::workbench::app::armed_stretch_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_stretch_selection(state);
        return;
    }

    retain_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) = consume_keyboard(ui, response.has_focus(), grid_size);
    if keyboard_step != Point::origin() {
        let target = state
            .dialogs
            .stretch_selection
            .target
            .expect("validated stretch target");
        let requested =
            checked_accumulate_delta(state.dialogs.stretch_selection.preview_delta, keyboard_step);
        let draft = &mut state.dialogs.stretch_selection;
        draft.anchor = None;
        draft.pointer_drag = false;
        update_preview_delta(state, target, requested);
    }
    if keyboard_commit {
        commit_armed_stretch_selection(state, symbol_context);
        return;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let anchor = screen_to_grid(viewport, grid_size, position);
        if let Some(target) = stretch_target_at(state, viewport, position) {
            let draft = &mut state.dialogs.stretch_selection;
            draft.target = Some(target);
            draft.anchor = Some(anchor);
            draft.preview_delta = Point::origin();
            draft.pointer_drag = true;
            draft.preview_error = None;
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && state.dialogs.stretch_selection.pointer_drag
        && let (Some(anchor), Some(position), Some(target)) = (
            state.dialogs.stretch_selection.anchor,
            response
                .hover_pos()
                .or_else(|| response.interact_pointer_pos()),
            state.dialogs.stretch_selection.target,
        )
    {
        let destination = screen_to_grid(viewport, grid_size, position);
        update_preview_delta(state, target, checked_pointer_delta(anchor, destination));
    }

    if response.drag_stopped_by(egui::PointerButton::Primary)
        && state.dialogs.stretch_selection.pointer_drag
    {
        state.dialogs.stretch_selection.pointer_drag = false;
        commit_armed_stretch_selection(state, symbol_context);
        return;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point = screen_to_grid(viewport, grid_size, position);
        if let (Some(anchor), Some(target)) = (
            state.dialogs.stretch_selection.anchor,
            state.dialogs.stretch_selection.target,
        ) {
            update_preview_delta(state, target, checked_pointer_delta(anchor, point));
            commit_armed_stretch_selection(state, symbol_context);
        } else if let Some(target) = stretch_target_at(state, viewport, position) {
            let draft = &mut state.dialogs.stretch_selection;
            draft.target = Some(target);
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            draft.preview_error = None;
        }
    } else if !state.dialogs.stretch_selection.pointer_drag
        && let (Some(anchor), Some(position), Some(target)) = (
            state.dialogs.stretch_selection.anchor,
            response.hover_pos(),
            state.dialogs.stretch_selection.target,
        )
    {
        let destination = screen_to_grid(viewport, grid_size, position);
        update_preview_delta(state, target, checked_pointer_delta(anchor, destination));
    }
}

fn checked_accumulate_delta(current: Point, step: Point) -> Result<Point, &'static str> {
    let Some(x) = current.x.checked_add(step.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = current.y.checked_add(step.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn checked_pointer_delta(anchor: Point, destination: Point) -> Result<Point, &'static str> {
    let Some(x) = destination.x.checked_sub(anchor.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = destination.y.checked_sub(anchor.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn update_preview_delta(
    state: &mut AppState,
    target: StretchTarget,
    requested: Result<Point, &'static str>,
) {
    match requested {
        Ok(requested) => {
            let delta = crate::workbench::app::stretch_delta_for_policy(
                requested,
                target,
                state.dialogs.stretch_selection.policy,
                state,
            );
            let draft = &mut state.dialogs.stretch_selection;
            draft.preview_delta = delta;
            draft.preview_error = None;
        }
        Err(message) => {
            let draft = &mut state.dialogs.stretch_selection;
            draft.preview_delta = Point::origin();
            draft.preview_error = Some(message.to_owned());
        }
    }
}

fn retain_canvas_focus_from_pointer(response: &Response) {
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

fn consume_keyboard(ui: &Ui, canvas_has_focus: bool, grid_size: i32) -> (Point, bool) {
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

fn stretch_target_at(
    state: &AppState,
    viewport: &Viewport,
    position: egui::Pos2,
) -> Option<StretchTarget> {
    let point = screen_to_schematic(viewport, position);
    let tolerance = f64::from((6.0 / viewport.zoom.max(0.1)).ceil() as i32);
    let selection = &state.schematic.selection;
    let mut candidates: Vec<(f64, u8, u64, usize, StretchTarget)> = Vec::new();

    for wire in state.schematic.wires.iter().filter(|wire| {
        object_is_on_active_sheet(state, wire.id)
            && (selection.has_wire(wire.id)
                || selection
                    .wire_segments
                    .iter()
                    .any(|selected| selected.wire_id == wire.id)
                || selection
                    .wire_vertices
                    .iter()
                    .any(|selected| selected.wire_id == wire.id))
    }) {
        for (segment_index, endpoints) in wire.points.windows(2).enumerate() {
            let distance =
                crate::state::WireSegment::new(endpoints[0], endpoints[1]).distance_to_point(point);
            let target = StretchTarget::WireSegment {
                wire_id: wire.id,
                segment_index,
            };
            if distance <= tolerance && state.schematic.is_stretch_target_eligible(target) {
                candidates.push((distance, 1, wire.id, segment_index, target));
            }
        }
    }
    for bus in state
        .schematic
        .buses
        .iter()
        .filter(|bus| selection.has_bus(bus.id) && object_is_on_active_sheet(state, bus.id))
    {
        for (segment_index, endpoints) in bus.points.windows(2).enumerate() {
            let distance =
                crate::state::WireSegment::new(endpoints[0], endpoints[1]).distance_to_point(point);
            let target = StretchTarget::BusSegment {
                bus_id: bus.id,
                segment_index,
            };
            if distance <= tolerance && state.schematic.is_stretch_target_eligible(target) {
                candidates.push((distance, 2, bus.id, segment_index, target));
            }
        }
    }
    for shape in state.schematic.documentation_shapes.iter().filter(|shape| {
        selection.has_documentation_shape(shape.id) && object_is_on_active_sheet(state, shape.id)
    }) {
        for (point_index, control) in shape.geometry.points().into_iter().enumerate() {
            let dx = f64::from(control.x) - f64::from(point.x);
            let dy = f64::from(control.y) - f64::from(point.y);
            let distance = dx.hypot(dy);
            let target = StretchTarget::DocumentationShapePoint {
                shape_id: shape.id,
                point_index,
            };
            if distance <= tolerance && state.schematic.is_stretch_target_eligible(target) {
                candidates.push((distance, 0, shape.id, point_index, target));
            }
        }
    }

    candidates.sort_by(|left, right| {
        left.0
            .total_cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
            .then(left.3.cmp(&right.3))
    });
    candidates.first().map(|candidate| candidate.4)
}

fn commit_armed_stretch_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = crate::workbench::app::armed_stretch_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_stretch_selection(state);
        return;
    }
    if let Some(message) = state.dialogs.stretch_selection.preview_error.clone() {
        let draft = &mut state.dialogs.stretch_selection;
        draft.anchor = None;
        draft.preview_delta = Point::origin();
        draft.pointer_drag = false;
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection was not committed: {message}"
        )));
        return;
    }
    let delta = state.dialogs.stretch_selection.preview_delta;
    let policy = state.dialogs.stretch_selection.policy;
    let target = state
        .dialogs
        .stretch_selection
        .target
        .expect("validated stretch target");
    if delta == Point::origin() {
        state.push_user_message(ConsoleMessage::info(
            "Stretch selection finished without changing geometry; no undo record was created."
                .to_owned(),
        ));
        crate::workbench::app::cancel_armed_stretch_selection(state);
        return;
    }

    state.schematic.begin_operation("stretch selection");
    match state.schematic.stretch_target_resolved(
        delta,
        target,
        policy,
        |component| symbol_context.terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
    ) {
        Ok(true) => {
            let recorded = state.schematic.end_operation();
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Stretched the selected geometry by ({}, {}) with {}; {}.",
                delta.x,
                delta.y,
                policy.label(),
                if recorded {
                    "one undo record committed"
                } else {
                    "geometry was unchanged"
                }
            )));
            crate::workbench::app::cancel_armed_stretch_selection(state);
        }
        Ok(false) => {
            state.schematic.cancel_operation();
            state.push_user_message(ConsoleMessage::info(
                "Stretch selection produced no geometry change; no undo record was created."
                    .to_owned(),
            ));
            crate::workbench::app::cancel_armed_stretch_selection(state);
        }
        Err(error) => {
            state.schematic.cancel_operation();
            let draft = &mut state.dialogs.stretch_selection;
            draft.preview_error = Some(error.to_string());
            draft.anchor = None;
            draft.preview_delta = Point::origin();
            draft.pointer_drag = false;
            state.push_user_message(ConsoleMessage::warning(format!(
                "Stretch selection was not committed: {error}"
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Junction, StretchOrthogonalPolicy, Wire};

    fn selected_stretch_wire() -> AppState {
        let mut state = AppState::default();
        state.schematic.wires.push(Wire::new(
            7,
            vec![
                Point::new(0, 0),
                Point::new(0, 20),
                Point::new(20, 20),
                Point::new(20, 0),
            ],
        ));
        state.schematic.selection.select_only_wire_segment(7, 1);
        state.schematic.init_undo_history();
        state
    }

    fn arm_test_stretch(state: &mut AppState) {
        crate::workbench::app::open_stretch_selection_dialog(state);
        assert!(state.dialogs.stretch_selection.open);
        state.dialogs.stretch_selection.arm();
        state
            .schematic
            .arm_tool(crate::state::Tool::StretchSelection);
    }

    #[test]
    fn target_resolution_is_limited_to_the_frozen_selection() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(100, 0)));
        state
            .schematic
            .wires
            .push(Wire::segment(2, Point::new(0, 10), Point::new(100, 10)));
        state.schematic.selection.select_only_wire_segment(2, 0);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(200.0)),
        };
        let position = viewport.schematic_to_screen(Point::new(50, 10));
        assert_eq!(
            stretch_target_at(&state, &viewport, position),
            Some(StretchTarget::WireSegment {
                wire_id: 2,
                segment_index: 0,
            })
        );
        assert_eq!(
            StretchOrthogonalPolicy::default(),
            StretchOrthogonalPolicy::PreserveOrthogonal
        );
    }

    #[test]
    fn vertex_selection_can_resolve_either_authorized_incident_segment() {
        let mut state = AppState::default();
        state.schematic.wires.push(Wire::new(
            4,
            vec![Point::new(0, 0), Point::new(0, 20), Point::new(30, 20)],
        ));
        state.schematic.selection.select_only_wire_vertex(4, 1);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(200.0)),
        };
        let position = viewport.schematic_to_screen(Point::new(20, 20));
        assert_eq!(
            stretch_target_at(&state, &viewport, position),
            Some(StretchTarget::WireSegment {
                wire_id: 4,
                segment_index: 1,
            })
        );
    }

    #[test]
    fn pointer_and_keyboard_delta_overflow_is_rejected_not_saturated() {
        assert_eq!(
            checked_pointer_delta(Point::new(i32::MIN, 0), Point::new(i32::MAX, 0)),
            Err(DELTA_OVERFLOW)
        );
        assert_eq!(
            checked_accumulate_delta(Point::new(i32::MAX, 0), Point::new(1, 0)),
            Err(DELTA_OVERFLOW)
        );
    }

    #[test]
    fn commit_records_once_syncs_workspace_and_retains_selection() {
        let mut state = selected_stretch_wire();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.preview_delta = Point::new(0, 10);
        let symbols = SchematicSymbolContext::from_state(&state);

        commit_armed_stretch_selection(&mut state, &symbols);

        assert_eq!(state.schematic.wires[0].points[1], Point::new(0, 30));
        assert_eq!(state.schematic.wires[0].points[2], Point::new(20, 30));
        assert_eq!(
            state.schematic.undo_description(),
            Some("stretch selection")
        );
        assert!(state.schematic.selection.has_wire_segment(7, 1));
        assert_eq!(state.schematic.tool, crate::state::Tool::Select);
        assert!(!state.dialogs.stretch_selection.armed);
        assert_eq!(
            state
                .workspace
                .active_schematic()
                .expect("active workspace buffer")
                .wires[0]
                .points[1],
            Point::new(0, 30)
        );
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.wires[0].points[1], Point::new(0, 20));
        assert!(
            !state.schematic.can_undo(),
            "the gesture owns one undo record"
        );
    }

    #[test]
    fn cancel_preserves_geometry_selection_and_history() {
        let mut state = selected_stretch_wire();
        let baseline = state.schematic.wires[0].clone();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.preview_delta = Point::new(0, 10);

        crate::workbench::app::cancel_armed_stretch_selection(&mut state);

        assert_eq!(state.schematic.wires[0], baseline);
        assert!(state.schematic.selection.has_wire_segment(7, 1));
        assert_eq!(state.schematic.tool, crate::state::Tool::Select);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn rejected_commit_preserves_authority_selection_and_undo_history() {
        let mut state = selected_stretch_wire();
        state
            .schematic
            .junctions
            .push(Junction::new(1, Point::new(10, 20)));
        let baseline = state.schematic.wires[0].clone();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.preview_delta = Point::new(0, 10);
        let symbols = SchematicSymbolContext::from_state(&state);

        commit_armed_stretch_selection(&mut state, &symbols);

        assert_eq!(state.schematic.wires[0], baseline);
        assert!(state.schematic.selection.has_wire_segment(7, 1));
        assert_eq!(state.schematic.tool, crate::state::Tool::StretchSelection);
        assert!(state.dialogs.stretch_selection.armed);
        assert!(state.dialogs.stretch_selection.preview_error.is_some());
        assert!(!state.schematic.can_undo());
    }
}
