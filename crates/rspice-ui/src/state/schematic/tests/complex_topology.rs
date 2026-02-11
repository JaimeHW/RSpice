use super::*;

// =============================================================================
// Additional Edge Case Tests for Commercial Parity
// =============================================================================

#[test]
fn test_multi_wire_rubber_banding() {
    let mut state = SchematicState::default();

    // Add component at (10, 10) - it has terminals at (8, 10) and (12, 10)
    let comp_id = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    let terminals: Vec<Point> = state.components[0]
        .terminal_positions()
        .into_iter()
        .map(|(_, pos)| pos)
        .collect();

    // Add wires connected to BOTH terminals
    state
        .add_wire(vec![
            terminals[0],
            Point::new(terminals[0].x - 10, terminals[0].y),
        ])
        .unwrap();
    state
        .add_wire(vec![
            terminals[1],
            Point::new(terminals[1].x + 10, terminals[1].y),
        ])
        .unwrap();

    // Move component by (5, 3)
    state.move_component_with_wires(comp_id, Point::new(5, 3));

    // BOTH wire endpoints should have moved with the component
    for wire in &state.wires {
        let new_terminals: Vec<Point> = state.components[0]
            .terminal_positions()
            .into_iter()
            .map(|(_, pos)| pos)
            .collect();

        // Wire endpoint should be at new terminal position
        assert!(
            new_terminals.contains(&wire.points[0])
                || wire.points[0].x == terminals[0].x - 10 + 5
                || wire.points[0].x == terminals[1].x + 10 + 5,
            "Wire endpoint should be moved: {:?}",
            wire.points[0]
        );
    }
}

#[test]
fn test_complex_multi_junction_topology() {
    let mut state = SchematicState::default();

    // Create a star topology: 5 wires all meeting at center (10, 10)
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
    state
        .add_wire(vec![Point::new(10, 10), Point::new(20, 20)])
        .unwrap(); // Diagonal

    let intersections = state.find_wire_intersections();

    // Should find one major intersection at (10, 10)
    let center_intersection = intersections.iter().find(|(p, _)| *p == Point::new(10, 10));
    assert!(center_intersection.is_some());

    let (_, wire_ids) = center_intersection.unwrap();
    assert_eq!(wire_ids.len(), 5, "All 5 wires should meet at center");

    // Junction classification should report Complex
    let junction_type = state.classify_junction_type(Point::new(10, 10));
    assert!(
        matches!(
            junction_type,
            crate::state::schematic::wire::JunctionType::Complex { wire_count: 5 }
        ),
        "Should be Complex junction with 5 wires"
    );
}

#[test]
fn test_merge_very_short_wires() {
    let mut state = SchematicState::default();

    // Two single-segment wires, each just 1 unit long
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(1, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(1, 0), Point::new(2, 0)])
        .unwrap();

    let merged = state.merge_wires(id1, id2);
    assert!(merged.is_some());

    let merged_wire = &state.wires[0];
    assert_eq!(merged_wire.start(), Some(Point::new(0, 0)));
    assert_eq!(merged_wire.end(), Some(Point::new(2, 0)));
}

#[test]
fn test_split_two_point_wire() {
    let mut state = SchematicState::default();

    // Minimal valid wire (2 points)
    let wire_id = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();

    // Split at midpoint
    let result = state.split_wire(wire_id, Point::new(5, 0));
    assert!(result.is_some());

    let (id1, id2) = result.unwrap();
    assert_eq!(state.wires.len(), 2);

    // Both resulting wires should be valid
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    let wire2 = state.wires.iter().find(|w| w.id == id2).unwrap();
    assert_eq!(wire1.points.len(), 2);
    assert_eq!(wire2.points.len(), 2);
}

#[test]
fn test_wire_operations_preserve_other_wires() {
    let mut state = SchematicState::default();

    // Add multiple wires
    let id1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    let id2 = state
        .add_wire(vec![Point::new(0, 10), Point::new(10, 10)])
        .unwrap();
    let id3 = state
        .add_wire(vec![Point::new(0, 20), Point::new(10, 20)])
        .unwrap();

    // Perform various operations on wire 2
    state.split_wire(id2, Point::new(5, 10));

    // Wires 1 and 3 should be unchanged
    let wire1 = state.wires.iter().find(|w| w.id == id1).unwrap();
    let wire3 = state.wires.iter().find(|w| w.id == id3).unwrap();

    assert_eq!(wire1.points, vec![Point::new(0, 0), Point::new(10, 0)]);
    assert_eq!(wire3.points, vec![Point::new(0, 20), Point::new(10, 20)]);
}

#[test]
fn test_junction_update_after_wire_delete() {
    let mut state = SchematicState::default();

    // Create T-junction
    state
        .add_wire(vec![Point::new(0, 0), Point::new(10, 0)])
        .unwrap();
    state
        .add_wire(vec![Point::new(10, 0), Point::new(20, 0)])
        .unwrap();
    let id3 = state
        .add_wire(vec![Point::new(10, 0), Point::new(10, 10)])
        .unwrap();

    state.auto_place_junctions();
    assert!(!state.junctions.is_empty());

    // Delete the third wire (breaks T-junction)
    state.delete_wire(id3);
    state.update_wire_junctions();

    // Junction should be removed (only 2 wires now = corner, not junction)
    assert!(state.junctions.is_empty());
}
