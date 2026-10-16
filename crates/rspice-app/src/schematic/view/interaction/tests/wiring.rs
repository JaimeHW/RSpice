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
    assert!(!label_anchor_on_unmarked_crossing(&state, Point::new(40, 0)));

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

/// Only this sheet's wires can cross under the label: Sheet 2's wire through
/// the same point is on another page.
#[test]
fn another_sheets_wire_makes_no_crossing_for_the_label_tool() {
    let mut state = two_sheet_state(&[1], &[2]);
    crossing(&mut state, 1, 2);

    assert!(!label_anchor_on_unmarked_crossing(&state, Point::origin()));
}
