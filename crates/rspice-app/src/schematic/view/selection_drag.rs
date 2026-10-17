//! Bind editor drag intents to the app's checked transaction owner.

use super::{
    SchematicSymbolContext, SelectionWindow,
    navigation::primary_pan_gesture_active,
    requests::editor_request_source,
    sheet_visibility::{select_in_rect_on_active_sheet, with_active_wire_topology},
    viewport::Viewport,
};
use crate::state::DragRubberBand;
use crate::workbench::app_state::{AppState, DragType};
use rspice_schematic_editor::view::selection_drag::{
    self, SelectionDragAction, SelectionDragRequest, SelectionDragView, select_drag_can_start,
    select_drag_is_authorized, select_drag_target,
};

fn view<'a>(state: &'a AppState, ctx: &egui::Context) -> SelectionDragView<'a> {
    SelectionDragView {
        design: super::schematic_design_view(state),
        editor: &state.schematic.session.editor,
        filter: state.ui.schematic_selection_filter,
        drag: state.schematic_drag_snapshot(ctx),
    }
}

pub(super) fn handle_select_dragging(
    ui: &egui::Ui,
    response: &egui::Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbols: &SchematicSymbolContext,
) {
    if !select_drag_is_authorized(
        state.schematic.session.editor.tool,
        state.dialogs.move_selection.armed,
    ) {
        return;
    }
    state.dialogs.interaction.hover_wire_vertex =
        selection_drag::hover_wire_vertex(response, &view(state, ui.ctx()), viewport, symbols)
            .map(|point| (point.x, point.y));
    if response.drag_started_by(egui::PointerButton::Primary)
        && !select_drag_can_start(primary_pan_gesture_active(ui, response))
    {
        return;
    }
    let action = selection_drag::start(
        ui,
        response,
        &view(state, ui.ctx()),
        viewport,
        symbols,
        state.schematic_edit_read_only(),
        || state.workspace.content.active_view.display_path(),
    );
    if let Some(action) = action {
        dispatch(state, ui.ctx(), symbols, action);
    }
    if let Some(action) = selection_drag::update(response, &view(state, ui.ctx()), viewport) {
        dispatch(state, ui.ctx(), symbols, action);
    }
    if let Some(action) = selection_drag::finish(ui, response, &view(state, ui.ctx())) {
        dispatch(state, ui.ctx(), symbols, action);
    }
}

fn capture(state: &AppState, action: SelectionDragAction) -> SelectionDragRequest {
    SelectionDragRequest {
        source: editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        rectangle: state.schematic.session.editor.selection_rect,
        action,
    }
}

fn dispatch(
    state: &mut AppState,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    action: SelectionDragAction,
) {
    let request = capture(state, action);
    apply(state, ctx, symbols, request);
}

fn apply(
    state: &mut AppState,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    request: SelectionDragRequest,
) {
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || request.rectangle != state.schematic.session.editor.selection_rect
        || !select_drag_is_authorized(
            state.schematic.session.editor.tool,
            state.dialogs.move_selection.armed,
        )
    {
        return;
    }
    match request.action {
        SelectionDragAction::BeginSelection { target, position } => {
            if state.schematic_edit_read_only() {
                return;
            }
            select_drag_target(&mut state.schematic.session.editor.selection, target);
            state.begin_schematic_drag((position.x, position.y), DragType::MoveSelection, ctx);
        }
        SelectionDragAction::BeginWireVertex(position) => {
            if state.schematic_edit_read_only() {
                return;
            }
            state.begin_schematic_drag((position.x, position.y), DragType::WireVertex, ctx);
        }
        SelectionDragAction::BeginMarquee(position) => state
            .schematic
            .session
            .editor
            .selection_rect
            .start_at(position),
        SelectionDragAction::BeginTouchPan => {
            selection_drag::set_touch_pan(ctx, true);
            // The finger crossed the drag threshold before the pan began;
            // the sheet catches up with it now, and follows it from here.
            if let Some(travel) = ctx
                .input(|input| Some(input.pointer.interact_pos()? - input.pointer.press_origin()?))
            {
                let pan = &mut state.schematic.session.editor.pan;
                pan.0 += f64::from(travel.x);
                pan.1 += f64::from(travel.y);
            }
        }
        SelectionDragAction::UpdateMarquee(position) => state
            .schematic
            .session
            .editor
            .selection_rect
            .update(position),
        SelectionDragAction::MoveWireVertex {
            operation_id,
            from,
            position,
        } => {
            if !owns_operation(state, ctx, operation_id, Some(DragType::WireVertex))
                || state.dialogs.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            if with_active_wire_topology(state, |schematic, scope| {
                schematic.move_all_vertices_at(from, position, scope)
            }) {
                state
                    .dialogs
                    .interaction
                    .drag
                    .update((position.x, position.y));
            }
        }
        SelectionDragAction::MoveSelection {
            operation_id,
            from,
            delta,
            position,
        } => {
            if !owns_operation(state, ctx, operation_id, Some(DragType::MoveSelection))
                || state.dialogs.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            // The rubber band follows the routing style: square unless the
            // user draws at 45° or free angles.
            let rubber_band = if state
                .schematic
                .session
                .editor
                .wire_drawing
                .routing_mode
                .is_orthogonal()
            {
                DragRubberBand::Orthogonal
            } else {
                DragRubberBand::Direct
            };
            with_active_wire_topology(state, |schematic, scope| {
                schematic.move_selection_with_rubber_band_resolved(
                    delta,
                    rubber_band,
                    |component| symbols.terminal_points(component),
                    scope,
                )
            });
            state
                .dialogs
                .interaction
                .drag
                .update((position.x, position.y));
        }
        SelectionDragAction::Finish { operation_id } => {
            if !owns_operation(state, ctx, operation_id, None) {
                return;
            }
            let automatic_junctions = state
                .schematic
                .document()
                .document_policy
                .wire_junctions
                .automatic_junctions();
            let moved = state.schematic.session.editor.selection.components.clone();
            with_active_wire_topology(state, |schematic, scope| {
                // A pin dropped inside a wire is connected there; the vertex
                // it gets is what lets the next move carry the wire. A pin
                // dragged off a pin it touched keeps the connection with a
                // wire.
                schematic.connect_moved_pins(
                    &moved,
                    |component| symbols.terminal_points(component),
                    scope,
                );
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope)
            });
            // One undo entry for the whole gesture (no-ops deduplicate).
            if state.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            state.dialogs.interaction.drag.cancel();
        }
        SelectionDragAction::FinishMarquee { additive } => {
            let rectangle = &mut state.schematic.session.editor.selection_rect;
            let (start, current) = (rectangle.start, rectangle.current);
            if rectangle.finish().is_none() {
                return;
            }
            // The window the live preview drew from, so release selects what
            // the drag showed selected.
            let window = SelectionWindow::from_band(
                start,
                current,
                state
                    .schematic
                    .document()
                    .document_policy
                    .selection_crossing,
            );
            select_in_rect_on_active_sheet(state, symbols, window, additive);
        }
    }
}

fn owns_operation(
    state: &AppState,
    ctx: &egui::Context,
    operation_id: u64,
    kind: Option<DragType>,
) -> bool {
    !state.schematic_edit_read_only()
        && state.schematic_drag_snapshot(ctx).is_some_and(|drag| {
            drag.operation_id == operation_id && kind.is_none_or(|kind| drag.kind == kind)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{CellViewRef, ComponentType, Point, ViewType};
    use rspice_schematic_editor::view::pointer_target::PointerTarget;

    /// A vertex drag follows the pointer for the whole gesture, including over
    /// a point that carries a junction and no wire vertex.
    ///
    /// The anchor only advances when the edit reports a move, so a point whose
    /// move went unreported moved once and was then dragged from an anchor it
    /// had already left: every later frame matched nothing and the object sat
    /// still under a moving pointer for as long as the button was held.
    #[test]
    fn a_vertex_drag_over_a_lone_junction_follows_the_pointer_every_frame() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let junction = Point::new(100, 100);
        state.schematic.add_junction(junction);
        // No component takes part, so one context serves the whole gesture.
        let symbols = super::super::schematic_symbol_context(&state);

        dispatch(
            &mut state,
            &ctx,
            &symbols,
            SelectionDragAction::BeginWireVertex(junction),
        );
        let operation_id = state
            .schematic
            .pending_operation_id()
            .expect("the gesture owns a transaction");

        let mut from = junction;
        for step in 1..=3 {
            let position = Point::new(junction.x, junction.y - step * 10);
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::MoveWireVertex {
                    operation_id,
                    from,
                    position,
                },
            );
            assert_eq!(
                state.schematic.document().junctions[0].pos,
                position,
                "step {step} left the junction behind the pointer"
            );
            assert_eq!(
                state.dialogs.interaction.drag.last_pos,
                Some((position.x, position.y)),
                "step {step} left the drag anchored where the junction no longer is"
            );
            from = position;
        }
    }

    /// Drag the selected part by `delta` as one whole gesture.
    fn drag(
        state: &mut AppState,
        ctx: &egui::Context,
        symbols: &crate::schematic::view::SchematicSymbolContext,
        target: PointerTarget,
        from: Point,
        delta: Point,
    ) {
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::BeginSelection {
                target,
                position: from,
            },
        );
        let operation_id = state.schematic.pending_operation_id().expect("a gesture");
        let position = Point::new(from.x + delta.x, from.y + delta.y);
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::MoveSelection {
                operation_id,
                from,
                delta,
                position,
            },
        );
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::Finish { operation_id },
        );
    }

    /// Drag the selected part through `steps`, each an offset from `from`,
    /// one frame per step, as one whole gesture.
    fn drag_through(
        state: &mut AppState,
        ctx: &egui::Context,
        symbols: &crate::schematic::view::SchematicSymbolContext,
        target: PointerTarget,
        from: Point,
        steps: &[Point],
    ) {
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::BeginSelection {
                target,
                position: from,
            },
        );
        let operation_id = state.schematic.pending_operation_id().expect("a gesture");
        let mut last = from;
        for step in steps {
            let position = Point::new(from.x + step.x, from.y + step.y);
            dispatch(
                state,
                ctx,
                symbols,
                SelectionDragAction::MoveSelection {
                    operation_id,
                    from: last,
                    delta: Point::new(position.x - last.x, position.y - last.y),
                    position,
                },
            );
            last = position;
        }
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::Finish { operation_id },
        );
    }

    /// A resistor at the origin whose right pin is wired 100 units to the
    /// right, ready to drag.
    fn resistor_wired_right(state: &mut AppState) -> (PointerTarget, Point, Point) {
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let pin = state.schematic.document().components[0].terminal_positions()[1].1;
        let far = Point::new(pin.x + 100, pin.y);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(9, pin, far));
        state.schematic.init_undo_history();
        (PointerTarget::Component(id), pin, far)
    }

    /// A part dragged with the mouse keeps its wires square, the way the
    /// Move command does: the wire leaves the moved pin, turns once onto
    /// its old line and runs on to where it was drawn to.
    #[test]
    fn a_dragged_part_keeps_its_wires_square() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let (target, pin, far) = resistor_wired_right(&mut state);
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            &[Point::new(0, 10), Point::new(0, 20)],
        );

        let moved = Point::new(pin.x, pin.y + 20);
        assert_eq!(
            state.schematic.document().wires.len(),
            1,
            "{:?}",
            state.schematic.document().wires
        );
        assert_eq!(
            state.schematic.document().wires[0].points,
            vec![moved, pin, far]
        );
    }

    /// A diagonal drag toward the wire's far end turns the wire once, under
    /// the moved pin, and runs on along its old line. It does not reach back
    /// to where the pin was and double over itself in a hook.
    #[test]
    fn a_diagonal_drag_toward_the_far_end_leaves_no_hook() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let (target, pin, far) = resistor_wired_right(&mut state);
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            &[Point::new(10, 10), Point::new(30, 20)],
        );

        let moved = Point::new(pin.x + 30, pin.y + 20);
        assert_eq!(
            state.schematic.document().wires[0].points,
            vec![moved, Point::new(moved.x, pin.y), far]
        );
    }

    /// Two parts touching pin to pin are one node with no wire. Dragging one
    /// off the other keeps them one node with a square wire from the pin
    /// that stayed to the pin that moved.
    #[test]
    fn dragging_a_part_off_a_pin_it_touched_wires_the_two_pins() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let pin = state.schematic.document().components[0].terminal_positions()[1].1;
        let left = state.schematic.document().components[0].terminal_positions()[0].1;
        let pitch = Point::new(pin.x - left.x, pin.y - left.y);
        let right = state
            .schematic
            .add_component(ComponentType::Resistor, pitch);
        state.schematic.init_undo_history();
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            PointerTarget::Component(right),
            pitch,
            &[Point::new(10, 10), Point::new(30, 20)],
        );

        let moved = Point::new(pin.x + 30, pin.y + 20);
        let wires = &state.schematic.document().wires;
        assert!(
            wires.iter().any(|wire| wire.is_orthogonal()
                && wire.points.first() == Some(&pin)
                && wire.points.last() == Some(&moved)),
            "{wires:?}"
        );
    }

    /// Each frame is laid from where the drag began, not from the frame
    /// before: a pointer that wanders and comes home leaves the wire exactly
    /// as it was drawn, with no jog from the trip.
    #[test]
    fn a_drag_that_wanders_and_comes_home_leaves_the_wire_as_drawn() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let (target, _, _) = resistor_wired_right(&mut state);
        let drawn = state.schematic.document().wires.clone();
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            &[
                Point::new(0, 20),
                Point::new(30, 20),
                Point::new(30, -10),
                Point::origin(),
            ],
        );

        assert_eq!(state.schematic.document().wires, drawn);
    }

    /// The part passes over the end of a wire it is not connected to. That
    /// wire stays where it was drawn; it is not picked up and carried along.
    #[test]
    fn a_drag_passing_over_a_loose_wire_end_leaves_it_behind() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let (target, pin, _) = resistor_wired_right(&mut state);
        let loose = crate::state::Wire::segment(
            11,
            Point::new(pin.x, pin.y + 20),
            Point::new(pin.x + 60, pin.y + 20),
        );
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(loose.clone());
        state.schematic.init_undo_history();
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            &[Point::new(0, 20), Point::new(0, 40)],
        );

        assert!(
            state.schematic.document().wires.contains(&loose),
            "{:?}",
            state.schematic.document().wires
        );
    }

    /// Under 45° or free routing the user has asked for angled wires, so the
    /// drag stretches the wire straight from the moved pin.
    #[test]
    fn an_angled_routing_style_stretches_the_dragged_wire_straight() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let (target, pin, far) = resistor_wired_right(&mut state);
        state
            .schematic
            .session
            .editor
            .wire_drawing
            .set_routing_mode(crate::state::WireRoutingMode::Diagonal);
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            &[Point::new(0, 20)],
        );

        assert_eq!(
            state.schematic.document().wires[0].points,
            vec![Point::new(pin.x, pin.y + 20), far]
        );
    }

    /// Sheet 2's wire ends where the dragged part's pin sits on Sheet 1. The
    /// drag lays its own wiring from where it began; the other sheet's wire
    /// is neither squared nor carried.
    #[test]
    fn a_drag_leaves_another_sheets_wire_end_in_place() {
        let ctx = egui::Context::default();
        let mut state = super::super::sheet_visibility::two_sheet_state(&[1, 9], &[20]);
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(crate::state::Component::new(
                1,
                ComponentType::Resistor,
                Point::origin(),
            ));
        let pin = state.schematic.document().components[0].terminal_positions()[1].1;
        let hidden = crate::state::Wire::segment(20, pin, Point::new(pin.x, pin.y - 100));
        state.schematic.document_mut_for_test().wires = vec![
            crate::state::Wire::segment(9, pin, Point::new(pin.x + 100, pin.y)),
            hidden.clone(),
        ];
        state.schematic.init_undo_history();
        let symbols = super::super::schematic_symbol_context(&state);

        drag_through(
            &mut state,
            &ctx,
            &symbols,
            PointerTarget::Component(1),
            Point::origin(),
            &[Point::new(0, 10), Point::new(0, 20)],
        );

        let wires = &state.schematic.document().wires;
        assert_eq!(wires.iter().find(|wire| wire.id == 20), Some(&hidden));
        assert!(
            wires
                .iter()
                .find(|wire| wire.id == 9)
                .is_some_and(|wire| wire.is_orthogonal()
                    && wire.points.first() == Some(&Point::new(pin.x, pin.y + 20)))
        );
    }

    /// A part dragged until its pin lies inside a wire is connected there,
    /// so the wire splits and the pin owns the vertex: the next drag carries
    /// the wire instead of silently leaving it behind.
    #[test]
    fn a_pin_dragged_into_a_wire_splits_it_and_the_next_drag_carries_it() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let pin = state.schematic.document().components[0].terminal_positions()[1].1;
        let landing = Point::new(pin.x + 20, pin.y);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(
                9,
                Point::new(landing.x, landing.y - 100),
                Point::new(landing.x, landing.y + 100),
            ));
        state.schematic.init_undo_history();
        let symbols = super::super::schematic_symbol_context(&state);
        let target = PointerTarget::Component(id);

        drag(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::origin(),
            Point::new(20, 0),
        );
        let at_pin = |state: &AppState, point: Point| {
            state
                .schematic
                .document()
                .wires
                .iter()
                .filter(|wire| wire.points.contains(&point))
                .count()
        };
        assert_eq!(at_pin(&state, landing), 2, "the wire was split at the pin");

        drag(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::new(20, 0),
            Point::new(0, 20),
        );
        assert_eq!(
            at_pin(&state, Point::new(landing.x, landing.y + 20)),
            2,
            "both halves followed the pin"
        );
    }

    #[test]
    fn queued_drag_requests_reject_changed_sources_and_replacement_transactions() {
        let ctx = egui::Context::default();
        for change in 0..4 {
            let mut state = AppState::default();
            let id = state
                .schematic
                .add_component(ComponentType::Resistor, Point::new(100, 100));
            let master = CellViewRef::new("user", "amp", "schematic");
            state
                .workspace
                .descend_into("X1".into(), master.clone(), ViewType::Schematic);
            let symbols = super::super::schematic_symbol_context(&state);
            let action = SelectionDragAction::BeginSelection {
                target: PointerTarget::Component(id),
                position: Point::new(100, 100),
            };
            let request = capture(&state, action);
            match change {
                0 => {
                    state.workspace.ascend_one().unwrap();
                    state
                        .workspace
                        .descend_into("X2".into(), master, ViewType::Schematic);
                }
                1 => {
                    state
                        .schematic
                        .add_component(ComponentType::Resistor, Point::origin());
                }
                2 => state
                    .schematic
                    .session
                    .editor
                    .selection_rect
                    .start_at(Point::origin()),
                _ => state.schematic.session.read_only = true,
            }
            let before = state.schematic.document().components.clone();
            apply(&mut state, &ctx, &symbols, request);
            assert!(!state.schematic.has_pending_operation(), "change {change}");
            assert_eq!(state.schematic.document().components, before);
            assert!(state.schematic.session.editor.selection.is_empty());

            state.schematic.session.read_only = false;
            dispatch(&mut state, &ctx, &symbols, action);
            let operation_id = state.schematic.pending_operation_id().expect("fresh begin");
            state.cancel_schematic_drag();
            assert!(state.begin_schematic_drag((100, 100), DragType::MoveSelection, &ctx));
            let replacement = state.schematic.pending_operation_id();
            assert_ne!(replacement, Some(operation_id));
            // Capture current source revisions to isolate the operation-token guard.
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::MoveSelection {
                    operation_id,
                    from: Point::new(100, 100),
                    delta: Point::new(10, 0),
                    position: Point::new(110, 100),
                },
            );
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::Finish { operation_id },
            );
            assert_eq!(state.schematic.document().components, before);
            assert_eq!(state.schematic.pending_operation_id(), replacement);
            state.cancel_schematic_drag();
        }
    }
}
