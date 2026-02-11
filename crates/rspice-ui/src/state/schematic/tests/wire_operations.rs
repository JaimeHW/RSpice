use super::*;

// =============================================================================
// Wire Operations Tests (Robust)
// =============================================================================

#[test]
fn test_split_wire_at_midpoint() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Split at midpoint
    let result = state.split_wire(wire_id, Point::new(5, 0));
    assert!(result.is_some());

    let (id1, id2) = result.unwrap();
    assert_eq!(state.wires.len(), 2);

    // Verify first wire goes from (0,0) to (5,0)
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    assert_eq!(wire1.start(), Some(Point::new(0, 0)));
    assert_eq!(wire1.end(), Some(Point::new(5, 0)));

    // Verify second wire goes from (5,0) to (10,0)
    let wire2 = state.wires.iter().find(|w| w.id == id2).unwrap();
    assert_eq!(wire2.start(), Some(Point::new(5, 0)));
    assert_eq!(wire2.end(), Some(Point::new(10, 0)));
}

#[test]
fn test_split_wire_at_vertex() {
    let mut state = SchematicState::default();
    // L-shaped wire with corner at (10, 0)
    let wire_id = state
        .add_wire(vec![
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
        ])
        .unwrap();

    // Split at the corner vertex
    let result = state.split_wire(wire_id, Point::new(10, 0));
    assert!(result.is_some());

    let (id1, id2) = result.unwrap();
    assert_eq!(state.wires.len(), 2);

    // First wire: (0,0) -> (10,0)
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    assert_eq!(wire1.points.len(), 2);

    // Second wire: (10,0) -> (10,10)
    let wire2 = state.wires.iter().find(|w| w.id == id2).unwrap();
    assert_eq!(wire2.points.len(), 2);
}

#[test]
fn test_split_wire_at_endpoint_fails() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Cannot split at start
    assert!(state.split_wire(wire_id, Point::new(0, 0)).is_none());

    // Cannot split at end
    assert!(state.split_wire(wire_id, Point::new(10, 0)).is_none());
}

#[test]
fn test_split_wire_off_wire_fails() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Point not on wire
    assert!(state.split_wire(wire_id, Point::new(5, 5)).is_none());
}

#[test]
fn test_split_wire_invalid_id_fails() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Invalid wire ID
    assert!(state.split_wire(9999, Point::new(5, 0)).is_none());
}

#[test]
fn test_split_wire_at_segment_midpoint() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();

    // Split at segment 0
    let result = state.split_wire_at_segment(wire_id, 0);
    assert!(result.is_some());

    let wire = state.wires.iter().find(|w| w.id == wire_id).unwrap();
    assert_eq!(wire.points.len(), 3); // Now has 3 vertices
    assert_eq!(wire.vertex_at(1), Some(Point::new(10, 0))); // Midpoint inserted
}

#[test]
fn test_split_wire_at_segment_invalid_index() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Segment index out of range
    assert!(state.split_wire_at_segment(wire_id, 5).is_none());
}

#[test]
fn test_merge_wires_end_to_start() {
    let mut state = SchematicState::default();
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap();

    // Merge wires that share endpoint
    let merged_id = state.merge_wires(id1, id2);
    assert!(merged_id.is_some());

    assert_eq!(state.wires.len(), 1);
    let merged = &state.wires[0];
    assert_eq!(merged.points.len(), 3);
    assert_eq!(merged.start(), Some(Point::new(0, 0)));
    assert_eq!(merged.end(), Some(Point::new(20, 0)));
}

#[test]
fn test_merge_wires_end_to_end() {
    let mut state = SchematicState::default();
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(20, 0), Point::new(10, 0)])
        .unwrap(); // Reversed

    let merged_id = state.merge_wires(id1, id2);
    assert!(merged_id.is_some());

    assert_eq!(state.wires.len(), 1);
    let merged = &state.wires[0];
    assert_eq!(merged.start(), Some(Point::new(0, 0)));
    assert_eq!(merged.end(), Some(Point::new(20, 0)));
}

#[test]
fn test_merge_wires_start_to_start() {
    let mut state = SchematicState::default();
    let id1 = state
        .add_wire(vec![Point::new(10, 0), Point::new(0, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap();

    let merged_id = state.merge_wires(id1, id2);
    assert!(merged_id.is_some());

    assert_eq!(state.wires.len(), 1);
}

#[test]
fn test_merge_wires_not_connected_fails() {
    let mut state = SchematicState::default();
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(5, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap(); // Not connected

    assert!(state.merge_wires(id1, id2).is_none());
    assert_eq!(state.wires.len(), 2); // Both wires still exist
}

#[test]
fn test_merge_wires_same_wire_fails() {
    let mut state = SchematicState::default();
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    assert!(state.merge_wires(id1, id1).is_none());
}

#[test]
fn test_straighten_wire() {
    let mut state = SchematicState::default();
    // Wire with unnecessary intermediate points on straight line
    let wire_id = state
        .add_wire(vec![
            Point::new(0, 0),
            Point::new(5, 0), // Collinear - should be removed
            Point::new(10, 0),
        ])
        .unwrap();

    state.straighten_wire(wire_id);

    let wire = state.wires.iter().find(|w| w.id == wire_id).unwrap();
    assert_eq!(wire.points.len(), 2); // Intermediate point removed
}

#[test]
fn test_straighten_wire_preserves_corners() {
    let mut state = SchematicState::default();
    // L-shaped wire - corner should be preserved
    let wire_id = state
        .add_wire(vec![
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
        ])
        .unwrap();

    state.straighten_wire(wire_id);

    let wire = state.wires.iter().find(|w| w.id == wire_id).unwrap();
    assert_eq!(wire.points.len(), 3); // Corner preserved
}

#[test]
fn test_optimize_all_wires() {
    let mut state = SchematicState::default();
    // Multiple wires with collinear points
    state
        .add_wire(vec![Point::new(0, 0), Point::new(5, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![
            Point::new(0, 10),
            Point::new(5, 10),
            Point::new(10, 10),
        ])
        .unwrap();

    state.optimize_all_wires();

    for wire in &state.wires {
        assert_eq!(wire.points.len(), 2);
    }
}

#[test]
fn test_delete_wire() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();

    assert_eq!(state.wires.len(), 2);

    let deleted = state.delete_wire(wire_id);
    assert!(deleted);
    assert_eq!(state.wires.len(), 1);
    assert!(state.wires.iter().find(|w| w.id == wire_id).is_none());
}

#[test]
fn test_delete_wire_invalid_id() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    let deleted = state.delete_wire(9999);
    assert!(!deleted);
    assert_eq!(state.wires.len(), 1);
}

#[test]
fn test_insert_wire_corner() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();

    // Insert corner at (10, 0) with offset (0, 5)
    let success = state.insert_wire_corner(wire_id, Point::new(10, 0), Point::new(0, 5));
    assert!(success);

    let wire = state.wires.iter().find(|w| w.id == wire_id).unwrap();
    assert_eq!(wire.points.len(), 4); // Two new vertices inserted
}

#[test]
fn test_insert_wire_corner_not_on_wire() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();

    // Point not on wire
    let success = state.insert_wire_corner(wire_id, Point::new(10, 5), Point::new(0, 5));
    assert!(!success);
}

#[test]
fn test_move_wire_vertex() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    let success = state.move_wire_vertex(wire_id, 1, Point::new(15, 5));
    assert!(success);

    let wire = state.wires.iter().find(|w| w.id == wire_id).unwrap();
    assert_eq!(wire.end(), Some(Point::new(15, 5)));
}

#[test]
fn test_move_wire_vertex_invalid_index() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    let success = state.move_wire_vertex(wire_id, 10, Point::new(15, 5));
    assert!(!success);
}

#[test]
fn test_move_wire_vertex_invalid_wire() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    let success = state.move_wire_vertex(9999, 0, Point::new(15, 5));
    assert!(!success);
}

#[test]
fn test_split_and_merge_roundtrip() {
    let mut state = SchematicState::default();
    let original_wire = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Split at midpoint
    let (id1, id2) = state.split_wire(original_wire, Point::new(5, 0)).unwrap();

    // Merge back
    let merged = state.merge_wires(id1, id2).unwrap();

    // Should have one wire again
    assert_eq!(state.wires.len(), 1);
    assert_eq!(state.wires[0].id, merged);

    // Straighten to clean up any extra vertices
    state.straighten_wire(merged);

    // Verify final state
    let wire = &state.wires[0];
    assert_eq!(wire.start(), Some(Point::new(0, 0)));
    assert_eq!(wire.end(), Some(Point::new(10, 0)));
}

#[test]
fn test_wire_operations_mark_dirty() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();
    state.is_dirty = false;

    state.split_wire_at_segment(wire_id, 0);
    assert!(state.is_dirty);
}

#[test]
fn test_wire_operations_bump_topology() {
    let mut state = SchematicState::default();
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();
    let initial_version = state.topology_version();

    state.split_wire_at_segment(wire_id, 0);
    assert!(state.topology_version() > initial_version);
}
