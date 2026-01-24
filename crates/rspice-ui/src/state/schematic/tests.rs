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
