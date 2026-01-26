//! Integration Tests for Schematic State
//!
//! Tests for the complete SchematicState workflow.

use super::*;

#[test]
fn test_schematic_state_default() {
    let state = SchematicState::default();
    assert!(state.components.is_empty());
    assert!(state.wires.is_empty());
    assert!(state.selection.is_empty());
    assert_eq!(state.grid_size, 10);
    assert_eq!(state.zoom, 1.0);
}

#[test]
fn test_add_component() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Resistor, Point::new(10, 20));

    assert_eq!(state.components.len(), 1);
    assert_eq!(state.components[0].id, id);
    assert_eq!(state.components[0].kind, ComponentType::Resistor);
    assert_eq!(state.components[0].pos, Point::new(10, 20));
    assert_eq!(state.components[0].name, "R1");
    assert!(state.is_dirty);
}

#[test]
fn test_add_multiple_components_naming() {
    let mut state = SchematicState::default();
    state.add_component(ComponentType::Resistor, Point::new(0, 0));
    state.add_component(ComponentType::Resistor, Point::new(10, 0));
    state.add_component(ComponentType::Capacitor, Point::new(20, 0));
    state.add_component(ComponentType::Resistor, Point::new(30, 0));

    assert_eq!(state.components[0].name, "R1");
    assert_eq!(state.components[1].name, "R2");
    assert_eq!(state.components[2].name, "C1");
    assert_eq!(state.components[3].name, "R3");
}

#[test]
fn test_add_wire() {
    let mut state = SchematicState::default();
    let wire_id = state.add_wire(vec![Point::new(0, 0), Point::new(10, 0)]);

    assert!(wire_id.is_some());
    assert_eq!(state.wires.len(), 1);
    assert!(state.is_dirty);
}

#[test]
fn test_add_wire_too_short() {
    let mut state = SchematicState::default();

    // Single point - not a valid wire
    let wire_id = state.add_wire(vec![Point::new(0, 0)]);
    assert!(wire_id.is_none());
    assert!(state.wires.is_empty());

    // Empty points - not a valid wire
    let wire_id = state.add_wire(vec![]);
    assert!(wire_id.is_none());
}

#[test]
fn test_component_at() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Resistor, Point::new(10, 20));

    assert_eq!(state.component_at(Point::new(10, 20)), Some(id));
    assert_eq!(state.component_at(Point::new(0, 0)), None);
}

#[test]
fn test_wire_at() {
    let mut state = SchematicState::default();
    state.add_wire(vec![Point::new(0, 0), Point::new(10, 0)]);

    assert!(state.wire_at(Point::new(5, 0)).is_some()); // On the wire
    assert!(state.wire_at(Point::new(5, 5)).is_none()); // Not on wire
}

#[test]
fn test_add_junction() {
    let mut state = SchematicState::default();
    let id1 = state.add_junction(Point::new(10, 10));
    let id2 = state.add_junction(Point::new(10, 10)); // Same position

    assert_eq!(id1, id2); // Should return existing junction
    assert_eq!(state.junctions.len(), 1);
}

#[test]
fn test_has_junction() {
    let mut state = SchematicState::default();
    state.add_junction(Point::new(10, 10));

    assert!(state.has_junction(Point::new(10, 10)));
    assert!(!state.has_junction(Point::new(20, 20)));
}

#[test]
fn test_delete_selection() {
    let mut state = SchematicState::default();
    let c1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let c2 = state.add_component(ComponentType::Capacitor, Point::new(10, 0));
    let w1 = state
        .add_wire(vec![Point::new(2, 0), Point::new(8, 0)])
        .unwrap();

    state.selection.select_component(c1);
    state.selection.select_wire(w1);
    state.delete_selection();

    assert_eq!(state.components.len(), 1);
    assert_eq!(state.components[0].id, c2);
    assert!(state.wires.is_empty());
    assert!(state.selection.is_empty());
}

#[test]
fn test_rotate_selection() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    state.selection.select_component(id);

    assert_eq!(state.components[0].rotation, Rotation::R0);

    state.rotate_selection();
    assert_eq!(state.components[0].rotation, Rotation::R90);

    state.rotate_selection();
    assert_eq!(state.components[0].rotation, Rotation::R180);
}

#[test]
fn test_copy_paste() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.selection.select_component(id);

    state.copy_selection();
    assert!(state.can_paste());

    state.paste_at(Point::new(30, 30));

    assert_eq!(state.components.len(), 2);
    assert_eq!(state.components[1].pos, Point::new(30, 30));
    assert_eq!(state.components[1].name, "R2"); // New name
}

#[test]
fn test_content_bounds_empty() {
    let state = SchematicState::default();
    assert!(state.content_bounds().is_none());
}

#[test]
fn test_content_bounds_with_content() {
    let mut state = SchematicState::default();
    state.add_component(ComponentType::Resistor, Point::new(0, 0));
    state.add_component(ComponentType::Resistor, Point::new(100, 100));

    let bounds = state.content_bounds();
    assert!(bounds.is_some());

    let (min_x, min_y, max_x, max_y) = bounds.unwrap();
    assert!(min_x < 0); // Component footprint extends left
    assert!(min_y < 0);
    assert!(max_x > 100);
    assert!(max_y > 100);
}

#[test]
fn test_recalculate_runtime_state() {
    let mut state = SchematicState::default();

    // Add components with specific IDs and names
    let mut comp = Component::new(100, ComponentType::Resistor, Point::new(0, 0));
    comp.name = "R5".to_string();
    state.components.push(comp);

    // After deserialization, runtime state needs to be rebuilt
    state.recalculate_runtime_state();

    // Next ID should be > 100
    let new_id = state.next_id();
    assert!(new_id > 100);

    // Next resistor should be R6
    let name = state.generate_name(ComponentType::Resistor);
    assert_eq!(name, "R6");
}

#[test]
fn test_wire_drawing_workflow() {
    let mut state = SchematicState::default();

    // Start drawing wire
    state.start_wire(Point::new(0, 0));
    assert!(state.wire_drawing.is_active());
    assert_eq!(state.wire_drawing.point_count(), 1);

    // Update preview
    state.update_wire_preview(Point::new(10, 5));
    assert_eq!(state.wire_drawing.preview_pos, Some(Point::new(10, 5)));

    // Extend wire
    state.extend_wire(Point::new(10, 0));
    state.extend_wire(Point::new(10, 10));

    // Finish wire
    let wire_id = state.finish_wire();
    assert!(wire_id.is_some());
    assert!(!state.wires.is_empty());
    assert!(!state.wire_drawing.is_active());
}

#[test]
fn test_cancel_wire() {
    let mut state = SchematicState::default();

    state.start_wire(Point::new(0, 0));
    state.extend_wire(Point::new(10, 0));
    state.cancel_wire();

    assert!(!state.wire_drawing.is_active());
    assert!(state.wire_drawing.points.is_empty());
    assert!(state.wires.is_empty()); // No wire was created
}

#[test]
fn test_move_component_with_wires() {
    let mut state = SchematicState::default();

    // Add a resistor at (10, 10)
    let comp_id = state.add_component(ComponentType::Resistor, Point::new(10, 10));

    // Get one of its terminal positions
    let terminal_pos = state.components[0].terminal_positions()[0].1;

    // Add a wire starting at the terminal
    state.add_wire(vec![
        terminal_pos,
        Point::new(terminal_pos.x + 20, terminal_pos.y),
    ]);

    // Move the component by (5, 5)
    state.move_component_with_wires(comp_id, Point::new(5, 5));

    // Component should have moved
    assert_eq!(state.components[0].pos, Point::new(15, 15));

    // Wire endpoint should have moved with it
    let new_terminal = Point::new(terminal_pos.x + 5, terminal_pos.y + 5);
    assert_eq!(state.wires[0].points[0], new_terminal);
}

#[test]
fn test_find_terminal_at() {
    let mut state = SchematicState::default();
    state.add_component(ComponentType::Resistor, Point::new(10, 10));

    // Terminal should be near the component
    let terminal_pos = state.components[0].terminal_positions()[0].1;
    let found = state.find_terminal_at(terminal_pos);

    assert!(found.is_some());
    let (comp_id, term_name, _) = found.unwrap();
    assert_eq!(comp_id, state.components[0].id);
    assert!(!term_name.is_empty());
}

#[test]
fn test_net_label() {
    let mut state = SchematicState::default();
    let id = state.add_net_label(Point::new(10, 10), "VCC".to_string());

    assert_eq!(state.net_labels.len(), 1);
    assert_eq!(state.net_labels[0].id, id);
    assert_eq!(state.net_labels[0].name, "VCC");
    assert!(state.is_dirty);
}

#[test]
fn test_simplify_wire_path() {
    // A straight horizontal wire with extra points
    let points = vec![Point::new(0, 0), Point::new(5, 0), Point::new(10, 0)];
    let simplified = SchematicState::simplify_wire_path(points);
    assert_eq!(simplified.len(), 2); // Start and end only
    assert_eq!(simplified[0], Point::new(0, 0));
    assert_eq!(simplified[1], Point::new(10, 0));
}

#[test]
fn test_simplify_wire_path_with_corner() {
    // An L-shaped wire
    let points = vec![
        Point::new(0, 0),
        Point::new(10, 0), // Corner
        Point::new(10, 10),
    ];
    let simplified = SchematicState::simplify_wire_path(points);
    assert_eq!(simplified.len(), 3); // Corner should be preserved
}

#[test]
fn test_unique_ids() {
    let mut state = SchematicState::default();

    let c1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let c2 = state.add_component(ComponentType::Resistor, Point::new(10, 0));
    let w1 = state
        .add_wire(vec![Point::new(2, 0), Point::new(8, 0)])
        .unwrap();
    let j1 = state.add_junction(Point::new(5, 0));

    // All IDs should be unique
    let ids = vec![c1, c2, w1, j1];
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len());
}

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
    let wire_id = state
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
