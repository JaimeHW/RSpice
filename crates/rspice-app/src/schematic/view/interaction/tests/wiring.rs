//! Wire, bus and label tools: where a conductor or a name lands, and what it
//! joins.

use super::*;
use crate::schematic::view::sheet_visibility::two_sheet_state;

fn crossing(state: &mut AppState, first: u64, second: u64) {
    state.schematic.document_mut_for_test().wires.extend([
        Wire::segment(first, Point::new(-100, 0), Point::new(100, 0)),
        Wire::segment(second, Point::new(0, -100), Point::new(0, 100)),
    ]);
}

/// A label exactly on a crossing of two wire interiors would join neither;
/// the tool refuses it there and accepts it anywhere along either wire.
#[test]
fn the_label_tool_refuses_an_unmarked_crossing() {
    let mut state = AppState::default();
    crossing(&mut state, 1, 2);

    assert!(label_anchor_on_unmarked_crossing(&state, Point::origin()));
    assert!(!label_anchor_on_unmarked_crossing(
        &state,
        Point::new(40, 0)
    ));

    state
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(3, Point::origin()));
    assert!(
        !label_anchor_on_unmarked_crossing(&state, Point::origin()),
        "a junction says the wires are one net"
    );
}

/// Where a resistor must be placed for its first pin to land on `pin`.
fn resistor_with_first_pin_at(pin: Point) -> Point {
    let probe = Component::new(0, ComponentType::Resistor, Point::origin());
    let offset = probe.terminal_positions()[0].1;
    Point::new(pin.x - offset.x, pin.y - offset.y)
}

/// A part dropped with a pin inside a wire is connected there, so the wire
/// splits and the pin owns the vertex, with the dot three conductors earn.
#[test]
fn a_part_dropped_with_a_pin_inside_a_wire_splits_it() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(1, Point::new(0, -100), Point::new(0, 100)));
    state.schematic.init_undo_history();

    place_component(
        &mut state,
        ComponentType::Resistor,
        resistor_with_first_pin_at(Point::origin()),
    );

    let wires = &state.schematic.document().wires;
    assert_eq!(
        wires
            .iter()
            .filter(|wire| wire.points.contains(&Point::origin()))
            .count(),
        2
    );
    assert_eq!(
        state
            .schematic
            .document()
            .junctions
            .iter()
            .map(|junction| junction.pos)
            .collect::<Vec<_>>(),
        vec![Point::origin()]
    );
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().wires.len(), 1, "one undo step");
}

/// Sheet 2's wire under the dropped pin is on another page: it stays whole.
#[test]
fn a_part_dropped_over_another_sheets_wire_leaves_it_whole() {
    let mut state = two_sheet_state(&[], &[1]);
    let hidden = Wire::segment(1, Point::new(-100, 0), Point::new(100, 0));
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(hidden.clone());
    state.schematic.init_undo_history();

    place_component(
        &mut state,
        ComponentType::Resistor,
        resistor_with_first_pin_at(Point::origin()),
    );

    assert_eq!(state.schematic.document().wires, vec![hidden]);
    assert!(state.schematic.document().junctions.is_empty());
}

/// A pin dropped exactly where two wires cross is ambiguous: neither wire is
/// split, and the design check asks which one it means.
#[test]
fn a_part_dropped_on_an_unmarked_crossing_splits_neither_wire() {
    let mut state = AppState::default();
    crossing(&mut state, 1, 2);

    place_component(
        &mut state,
        ComponentType::Resistor,
        resistor_with_first_pin_at(Point::origin()),
    );

    assert!(
        !state
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.points.contains(&Point::origin())),
        "a split at the crossing would join both wires"
    );
}

/// A route drawn along Sheet 2's wire lies on another page: the two are not
/// folded into one, and the hidden wire keeps its id and points.
#[test]
fn a_route_drawn_along_another_sheets_wire_is_not_folded_into_it() {
    let mut state = two_sheet_state(&[], &[1]);
    let hidden = Wire::segment(1, Point::new(0, 0), Point::new(40, 0));
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(hidden.clone());
    state.schematic.init_undo_history();

    state.schematic.start_wire(Point::new(20, 0));
    state.schematic.extend_wire(Point::new(60, 0));
    finish_wire_on_active_sheet(&mut state).expect("the route is drawn");

    let wires = &state.schematic.document().wires;
    assert_eq!(wires.iter().find(|wire| wire.id == 1), Some(&hidden));
    assert_eq!(wires.len(), 2);
}

/// Only this sheet's wires can cross under the label: Sheet 2's wire through
/// the same point is on another page.
#[test]
fn another_sheets_wire_makes_no_crossing_for_the_label_tool() {
    let mut state = two_sheet_state(&[1], &[2]);
    crossing(&mut state, 1, 2);

    assert!(!label_anchor_on_unmarked_crossing(&state, Point::origin()));
}
