//! Canvas drags bound to the application's checked transaction owner, driven
//! through a whole application.

use crate::schematic::view::selection_drag::*;
use crate::state::{CellViewRef, ComponentType, Point, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::DragType;
use rspice_schematic_editor::view::pointer_target::PointerTarget;
use rspice_schematic_editor::view::selection_drag::SelectionDragAction;

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
    state.model.schematic.add_junction(junction);
    // No component takes part, so one context serves the whole gesture.
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

    dispatch(
        &mut crate::schematic::view::canvas_parts(&mut state),
        &ctx,
        &symbols,
        SelectionDragAction::BeginWireVertex(junction),
    );
    let operation_id = state
        .model
        .schematic
        .pending_operation_id()
        .expect("the gesture owns a transaction");

    let mut from = junction;
    for step in 1..=3 {
        let position = Point::new(junction.x, junction.y - step * 10);
        dispatch(
            &mut crate::schematic::view::canvas_parts(&mut state),
            &ctx,
            &symbols,
            SelectionDragAction::MoveWireVertex {
                operation_id,
                from,
                position,
            },
        );
        assert_eq!(
            state.model.schematic.document().junctions[0].pos,
            position,
            "step {step} left the junction behind the pointer"
        );
        assert_eq!(
            state.design.interaction.drag.last_pos,
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
        &mut crate::schematic::view::canvas_parts(state),
        ctx,
        symbols,
        SelectionDragAction::BeginSelection {
            target,
            position: from,
        },
    );
    let operation_id = state
        .model
        .schematic
        .pending_operation_id()
        .expect("a gesture");
    let position = Point::new(from.x + delta.x, from.y + delta.y);
    dispatch(
        &mut crate::schematic::view::canvas_parts(state),
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
        &mut crate::schematic::view::canvas_parts(state),
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
        &mut crate::schematic::view::canvas_parts(state),
        ctx,
        symbols,
        SelectionDragAction::BeginSelection {
            target,
            position: from,
        },
    );
    let operation_id = state
        .model
        .schematic
        .pending_operation_id()
        .expect("a gesture");
    let mut last = from;
    for step in steps {
        let position = Point::new(from.x + step.x, from.y + step.y);
        dispatch(
            &mut crate::schematic::view::canvas_parts(state),
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
        &mut crate::schematic::view::canvas_parts(state),
        ctx,
        symbols,
        SelectionDragAction::Finish { operation_id },
    );
}

/// A resistor at the origin whose right pin is wired 100 units to the
/// right, ready to drag.
fn resistor_wired_right(state: &mut AppState) -> (PointerTarget, Point, Point) {
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    let pin = state.model.schematic.document().components[0].terminal_positions()[1].1;
    let far = Point::new(pin.x + 100, pin.y);
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(9, pin, far));
    state.model.schematic.init_undo_history();
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
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

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
        state.model.schematic.document().wires.len(),
        1,
        "{:?}",
        state.model.schematic.document().wires
    );
    assert_eq!(
        state.model.schematic.document().wires[0].points,
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
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

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
        state.model.schematic.document().wires[0].points,
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
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    let pin = state.model.schematic.document().components[0].terminal_positions()[1].1;
    let left = state.model.schematic.document().components[0].terminal_positions()[0].1;
    let pitch = Point::new(pin.x - left.x, pin.y - left.y);
    let right = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, pitch);
    state.model.schematic.init_undo_history();
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

    drag_through(
        &mut state,
        &ctx,
        &symbols,
        PointerTarget::Component(right),
        pitch,
        &[Point::new(10, 10), Point::new(30, 20)],
    );

    let moved = Point::new(pin.x + 30, pin.y + 20);
    let wires = &state.model.schematic.document().wires;
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
    let drawn = state.model.schematic.document().wires.clone();
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

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

    assert_eq!(state.model.schematic.document().wires, drawn);
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(loose.clone());
    state.model.schematic.init_undo_history();
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

    drag_through(
        &mut state,
        &ctx,
        &symbols,
        target,
        Point::origin(),
        &[Point::new(0, 20), Point::new(0, 40)],
    );

    assert!(
        state.model.schematic.document().wires.contains(&loose),
        "{:?}",
        state.model.schematic.document().wires
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
        .model
        .schematic
        .session
        .editor
        .wire_drawing
        .set_routing_mode(crate::state::WireRoutingMode::Diagonal);
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

    drag_through(
        &mut state,
        &ctx,
        &symbols,
        target,
        Point::origin(),
        &[Point::new(0, 20)],
    );

    assert_eq!(
        state.model.schematic.document().wires[0].points,
        vec![Point::new(pin.x, pin.y + 20), far]
    );
}

/// Sheet 2's wire ends where the dragged part's pin sits on Sheet 1. The
/// drag lays its own wiring from where it began; the other sheet's wire
/// is neither squared nor carried.
#[test]
fn a_drag_leaves_another_sheets_wire_end_in_place() {
    let ctx = egui::Context::default();
    let mut state = crate::schematic::view::sheet_visibility::two_sheet_state(&[1, 9], &[20]);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(crate::state::Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
    let pin = state.model.schematic.document().components[0].terminal_positions()[1].1;
    let hidden = crate::state::Wire::segment(20, pin, Point::new(pin.x, pin.y - 100));
    state.model.schematic.document_mut_for_test().wires = vec![
        crate::state::Wire::segment(9, pin, Point::new(pin.x + 100, pin.y)),
        hidden.clone(),
    ];
    state.model.schematic.init_undo_history();
    let symbols = crate::schematic::view::schematic_symbol_context(&state);

    drag_through(
        &mut state,
        &ctx,
        &symbols,
        PointerTarget::Component(1),
        Point::origin(),
        &[Point::new(0, 10), Point::new(0, 20)],
    );

    let wires = &state.model.schematic.document().wires;
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
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    let pin = state.model.schematic.document().components[0].terminal_positions()[1].1;
    let landing = Point::new(pin.x + 20, pin.y);
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            9,
            Point::new(landing.x, landing.y - 100),
            Point::new(landing.x, landing.y + 100),
        ));
    state.model.schematic.init_undo_history();
    let symbols = crate::schematic::view::schematic_symbol_context(&state);
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
            .model
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
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        let master = CellViewRef::new("user", "amp", "schematic");
        state
            .model
            .workspace
            .descend_into("X1".into(), master.clone(), ViewType::Schematic);
        let symbols = crate::schematic::view::schematic_symbol_context(&state);
        let action = SelectionDragAction::BeginSelection {
            target: PointerTarget::Component(id),
            position: Point::new(100, 100),
        };
        let request = capture(&crate::schematic::view::canvas_parts(&mut state), action);
        match change {
            0 => {
                state.model.workspace.ascend_one().unwrap();
                state
                    .model
                    .workspace
                    .descend_into("X2".into(), master, ViewType::Schematic);
            }
            1 => {
                state
                    .model
                    .schematic
                    .add_component(ComponentType::Resistor, Point::origin());
            }
            2 => state
                .model
                .schematic
                .session
                .editor
                .selection_rect
                .start_at(Point::origin()),
            _ => state.model.schematic.session.read_only = true,
        }
        let before = state.model.schematic.document().components.clone();
        apply(
            &mut crate::schematic::view::canvas_parts(&mut state),
            &ctx,
            &symbols,
            request,
        );
        assert!(
            !state.model.schematic.has_pending_operation(),
            "change {change}"
        );
        assert_eq!(state.model.schematic.document().components, before);
        assert!(state.model.schematic.session.editor.selection.is_empty());

        state.model.schematic.session.read_only = false;
        dispatch(
            &mut crate::schematic::view::canvas_parts(&mut state),
            &ctx,
            &symbols,
            action,
        );
        let operation_id = state
            .model
            .schematic
            .pending_operation_id()
            .expect("fresh begin");
        state.cancel_schematic_drag();
        assert!(state.begin_schematic_drag((100, 100), DragType::MoveSelection, &ctx));
        let replacement = state.model.schematic.pending_operation_id();
        assert_ne!(replacement, Some(operation_id));
        // Capture current source revisions to isolate the operation-token guard.
        dispatch(
            &mut crate::schematic::view::canvas_parts(&mut state),
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
            &mut crate::schematic::view::canvas_parts(&mut state),
            &ctx,
            &symbols,
            SelectionDragAction::Finish { operation_id },
        );
        assert_eq!(state.model.schematic.document().components, before);
        assert_eq!(state.model.schematic.pending_operation_id(), replacement);
        state.cancel_schematic_drag();
    }
}
