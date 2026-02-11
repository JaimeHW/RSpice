use super::*;

// =============================================================================
// Degenerate Segment Cleanup Tests (Commercial-Grade)
// =============================================================================

#[test]
fn test_remove_degenerate_segments_single_wire() {
    let mut state = SchematicState::default();
    // Wire with consecutive duplicate points (zero-length segments)
    let wire_id = state.next_id();
    state.wires.push(Wire::new(
        wire_id,
        vec![
            Point::new(0, 0),
            Point::new(5, 0),
            Point::new(5, 0), // Duplicate - zero-length segment
            Point::new(10, 0),
        ],
    ));

    let (modified, removed) = state.remove_degenerate_segments();

    assert_eq!(modified, 1);
    assert_eq!(removed, 0);
    assert_eq!(state.wires[0].points.len(), 3);
}

#[test]
fn test_remove_degenerate_segments_multiple_duplicates() {
    let mut state = SchematicState::default();
    let wire_id = state.next_id();
    state.wires.push(Wire::new(
        wire_id,
        vec![
            Point::new(0, 0),
            Point::new(0, 0), // Duplicate at start
            Point::new(5, 0),
            Point::new(5, 0), // Duplicate in middle
            Point::new(5, 0), // Triple duplicate
            Point::new(10, 0),
            Point::new(10, 0), // Duplicate at end
        ],
    ));

    let (modified, _) = state.remove_degenerate_segments();

    assert_eq!(modified, 1);
    // Should only have: (0,0), (5,0), (10,0)
    assert_eq!(state.wires[0].points.len(), 3);
}

#[test]
fn test_remove_degenerate_segments_wire_becomes_invalid() {
    let mut state = SchematicState::default();
    // Wire that's entirely duplicates - should be removed
    let wire_id = state.next_id();
    state.wires.push(Wire::new(
        wire_id,
        vec![
            Point::new(5, 5),
            Point::new(5, 5), // Same point
        ],
    ));

    let (_, removed) = state.remove_degenerate_segments();

    assert_eq!(removed, 1);
    assert!(state.wires.is_empty());
}

#[test]
fn test_remove_degenerate_segments_no_changes_needed() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    state.is_dirty = false;

    let (modified, removed) = state.remove_degenerate_segments();

    assert_eq!(modified, 0);
    assert_eq!(removed, 0);
    assert!(!state.is_dirty); // No changes, should remain clean
}

#[test]
fn test_remove_degenerate_segments_marks_dirty() {
    let mut state = SchematicState::default();
    let wire_id = state.next_id();
    state.wires.push(Wire::new(
        wire_id,
        vec![Point::new(0, 0), Point::new(0, 0), Point::new(10, 0)],
    ));
    state.is_dirty = false;
    let initial_version = state.topology_version();

    state.remove_degenerate_segments();

    assert!(state.is_dirty);
    assert!(state.topology_version() > initial_version);
}

#[test]
fn test_remove_degenerate_segments_for_wire_specific() {
    let mut state = SchematicState::default();
    // Wire 1: has duplicates
    let id1 = state.next_id();
    state.wires.push(Wire::new(
        id1,
        vec![Point::new(0, 0), Point::new(0, 0), Point::new(10, 0)],
    ));
    // Wire 2: no duplicates
    let id2 = state.next_id();
    state
        .wires
        .push(Wire::new(id2, vec![Point::new(0, 10), Point::new(10, 10)]));

    let modified = state.remove_degenerate_segments_for_wire(id1);

    assert!(modified);
    // Wire 1 should be cleaned
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    assert_eq!(wire1.points.len(), 2);
    // Wire 2 should be unchanged
    let wire2 = state.wires.iter().find(|w| w.id == id2).unwrap();
    assert_eq!(wire2.points.len(), 2);
}

#[test]
fn test_remove_degenerate_segments_for_wire_invalid_id() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    let modified = state.remove_degenerate_segments_for_wire(9999);

    assert!(!modified);
}

#[test]
fn test_remove_degenerate_segments_for_wire_removes_invalid() {
    let mut state = SchematicState::default();
    let id = state.next_id();
    state.wires.push(Wire::new(
        id,
        vec![Point::new(5, 5), Point::new(5, 5)], // Entirely degenerate
    ));

    let modified = state.remove_degenerate_segments_for_wire(id);

    assert!(modified);
    assert!(state.wires.is_empty()); // Wire removed entirely
}

#[test]
fn test_cleanup_wire_topology_comprehensive() {
    let mut state = SchematicState::default();

    // Wire with duplicates AND collinear points
    let id1 = state.next_id();
    state.wires.push(Wire::new(
        id1,
        vec![
            Point::new(0, 0),
            Point::new(5, 0),
            Point::new(5, 0),  // Duplicate
            Point::new(10, 0), // Collinear with (0,0) and (5,0)
        ],
    ));

    // Create T-junction topology for junction test
    state
        .add_wire(vec![Point::new(20, 0), Point::new(30, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(30, 0), Point::new(40, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(30, 0), Point::new(30, 10)])
        .unwrap();

    state.cleanup_wire_topology();

    // First wire should be cleaned: no duplicates, collinear points removed
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    assert_eq!(wire1.points.len(), 2); // Just start and end

    // T-junction should have junction marker
    assert!(state.junctions.iter().any(|j| j.pos == Point::new(30, 0)));
}
