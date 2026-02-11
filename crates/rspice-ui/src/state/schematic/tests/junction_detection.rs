use super::*;

// =============================================================================
// Junction Detection Tests (Robust)
// =============================================================================

#[test]
fn test_find_wire_intersections_none() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(0, 20), Point::new(10, 20)])
        .unwrap();

    // Parallel wires don't intersect
    let intersections = state.find_wire_intersections();
    assert!(intersections.is_empty());
}

#[test]
fn test_find_wire_intersections_shared_endpoint() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap();

    let intersections = state.find_wire_intersections();
    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].0, Point::new(10, 0));
}

#[test]
fn test_find_wire_intersections_cross() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 5), Point::new(10, 5)])
        .unwrap();
    state
        .add_wire(vec![Point::new(5, 0), Point::new(5, 10)])
        .unwrap();

    let intersections = state.find_wire_intersections();
    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].0, Point::new(5, 5));
}

#[test]
fn test_find_wire_intersections_t_junction() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();

    let intersections = state.find_wire_intersections();
    assert_eq!(intersections.len(), 1);
    let (point, wire_ids) = &intersections[0];
    assert_eq!(*point, Point::new(10, 0));
    assert_eq!(wire_ids.len(), 2);
}

#[test]
fn test_detect_junction_points_t_junction() {
    let mut state = SchematicState::default();
    // Create T-junction with 3 wires meeting
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(20, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(10, 20)])
        .unwrap();

    let junction_points = state.detect_junction_points();
    assert_eq!(junction_points.len(), 1);
    assert_eq!(junction_points[0], Point::new(10, 10));
}

#[test]
fn test_detect_junction_points_corner_no_junction() {
    let mut state = SchematicState::default();
    // L-shape: two wires forming a corner (NOT a junction for rendering)
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();

    let junction_points = state.detect_junction_points();
    // Only 2 wires meet = corner, not junction marker needed
    assert!(junction_points.is_empty());
}

#[test]
fn test_classify_junction_type() {
    let mut state = SchematicState::default();
    // Create cross junction (4 wires)
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(20, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(10, 20)])
        .unwrap();

    let junction_type = state.classify_junction_type(Point::new(10, 10));
    assert_eq!(
        junction_type,
        crate::state::schematic::wire::JunctionType::CrossJunction
    );
}

#[test]
fn test_auto_place_junctions() {
    let mut state = SchematicState::default();
    // Create T-junction
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(20, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(10, 20)])
        .unwrap();

    assert!(state.junctions.is_empty());

    state.auto_place_junctions();

    assert_eq!(state.junctions.len(), 1);
    assert_eq!(state.junctions[0].pos, Point::new(10, 10));
}

#[test]
fn test_remove_orphan_junctions() {
    let mut state = SchematicState::default();
    let _wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Add junction on wire
    state.add_junction(Point::new(5, 0));
    // Add orphan junction NOT on wire
    state.add_junction(Point::new(100, 100));

    assert_eq!(state.junctions.len(), 2);

    let removed = state.remove_orphan_junctions();
    assert_eq!(removed, 1);
    assert_eq!(state.junctions.len(), 1);
    assert_eq!(state.junctions[0].pos, Point::new(5, 0));
}

#[test]
fn test_count_connections_at_endpoint() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap();

    // Two endpoints meet at (10, 0)
    let count = state.count_connections_at(Point::new(10, 0));
    assert_eq!(count, 2);
}

#[test]
fn test_count_connections_at_t_junction() {
    let mut state = SchematicState::default();
    state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();

    // First wire passes through (10, 0) but not as an endpoint
    // Second wire starts there as an endpoint
    // count_connections_at counts segments, so endpoint of second wire = 1
    let count = state.count_connections_at(Point::new(10, 0));
    assert!(count >= 1, "Expected at least 1 connection, got {}", count);
}

#[test]
fn test_find_potential_splits() {
    let mut state = SchematicState::default();
    // Horizontal wire
    state
        .add_wire(vec![Point::new(0, 5), Point::new(20, 5)])
        .unwrap();
    // Vertical wire that crosses it
    state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();

    let splits = state.find_potential_splits();
    // Both wires could be split at intersection
    assert!(!splits.is_empty());
}

#[test]
fn test_update_wire_junctions_comprehensive() {
    let mut state = SchematicState::default();

    // Create complex topology
    state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(20, 10)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 10), Point::new(10, 20)])
        .unwrap();

    // Add orphan junction
    state.add_junction(Point::new(99, 99));

    state.update_wire_junctions();

    // Should have junction at T-intersection, orphan removed
    assert_eq!(state.junctions.len(), 1);
    assert_eq!(state.junctions[0].pos, Point::new(10, 10));
}
