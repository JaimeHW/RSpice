use super::*;

// =========================================================================
// Move Selection with Rubber-banding Tests
// =========================================================================

#[test]
fn test_move_selection_rubber_band_single_component() {
    let mut state = SchematicState::default();
    // Resistor at (0,0) has terminals at (-20,0) and (20,0) with 40x20 dimensions
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    // Wire connected to terminal at (20,0)
    state
        .add_wire(vec![Point::new(20, 0), Point::new(50, 0)])
        .unwrap();

    // Select the resistor
    state.selection.select_component(r1);

    // Move by (+5, +5)
    state.move_selection_with_rubber_band(Point::new(5, 5));

    // Resistor should have moved
    let comp = state.components.iter().find(|c| c.id == r1).unwrap();
    assert_eq!(comp.pos, Point::new(5, 5));

    // Wire endpoint at terminal should have stretched
    let wire = &state.wires[0];
    assert_eq!(wire.points[0], Point::new(25, 5)); // Moved with terminal (20+5, 0+5)
    assert_eq!(wire.points[1], Point::new(50, 0)); // Original position
}

#[test]
fn test_move_selection_rubber_band_wire_between_selected() {
    let mut state = SchematicState::default();
    // Two resistors: R1 at (0,0), R2 at (80,0) - separated so terminals can connect
    // With 40x20 dimensions: R1 terminal at (20,0), R2 terminal at (60,0)
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let r2 = state.add_component(ComponentType::Resistor, Point::new(80, 0));
    // Wire connecting R1's right terminal (20,0) to R2's left terminal (60,0)
    state
        .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
        .unwrap();

    // Select both resistors
    state.selection.select_component(r1);
    state.selection.select_component(r2);

    // Move by (+10, +10)
    state.move_selection_with_rubber_band(Point::new(10, 10));

    // Both resistors moved
    let comp1 = state.components.iter().find(|c| c.id == r1).unwrap();
    let comp2 = state.components.iter().find(|c| c.id == r2).unwrap();
    assert_eq!(comp1.pos, Point::new(10, 10));
    assert_eq!(comp2.pos, Point::new(90, 10));

    // Wire should have moved entirely (both ends connected to selection)
    let wire = &state.wires[0];
    assert_eq!(wire.points[0], Point::new(30, 10)); // (20+10, 0+10)
    assert_eq!(wire.points[1], Point::new(70, 10)); // (60+10, 0+10)
}

#[test]
fn test_move_selection_rubber_band_no_connection() {
    let mut state = SchematicState::default();
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    // Wire not connected to any terminal
    state
        .add_wire(vec![Point::new(100, 100), Point::new(120, 100)])
        .unwrap();

    state.selection.select_component(r1);
    state.move_selection_with_rubber_band(Point::new(5, 5));

    // Wire should be unchanged
    let wire = &state.wires[0];
    assert_eq!(wire.points[0], Point::new(100, 100));
    assert_eq!(wire.points[1], Point::new(120, 100));
}

#[test]
fn test_move_selection_rubber_band_empty_selection() {
    let mut state = SchematicState::default();
    state.add_component(ComponentType::Resistor, Point::new(0, 0));
    // Wire at terminal position (20,0) for 40x20 resistor
    state
        .add_wire(vec![Point::new(20, 0), Point::new(50, 0)])
        .unwrap();

    // Empty selection - should do nothing
    state.move_selection_with_rubber_band(Point::new(5, 5));

    // Component unchanged
    assert_eq!(state.components[0].pos, Point::new(0, 0));
    // Wire unchanged
    assert_eq!(state.wires[0].points[0], Point::new(20, 0));
}

#[test]
fn test_move_selection_rubber_band_selected_wire() {
    let mut state = SchematicState::default();
    let w1 = state
        .add_wire(vec![Point::new(0, 0), Point::new(20, 0)])
        .unwrap();

    // Select the wire
    state.selection.select_wire(w1);

    // Move by (+10, +10)
    state.move_selection_with_rubber_band(Point::new(10, 10));

    // Wire should have moved entirely
    let wire = &state.wires[0];
    assert_eq!(wire.points[0], Point::new(10, 10));
    assert_eq!(wire.points[1], Point::new(30, 10));
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
fn test_copy_paste_includes_connected_wires() {
    let mut state = SchematicState::default();
    // Two resistors at (0,0) and (80,0) - terminals at (20,0) and (60,0) with 40x20 dimensions
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let r2 = state.add_component(ComponentType::Resistor, Point::new(80, 0));
    // Wire connecting them (NOT explicitly selected)
    state
        .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
        .unwrap();

    // Select only the components (not the wire)
    state.selection.select_component(r1);
    state.selection.select_component(r2);

    state.copy_selection();

    // Wire should be included because both endpoints connect to selected components
    assert!(state.clipboard.has_content());
    assert_eq!(state.clipboard.components.len(), 2);
    assert_eq!(state.clipboard.wires.len(), 1);
}

#[test]
fn test_copy_paste_excludes_disconnected_wires() {
    let mut state = SchematicState::default();
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    // Wire not connected to component terminals
    state
        .add_wire(vec![Point::new(100, 100), Point::new(120, 100)])
        .unwrap();

    state.selection.select_component(r1);
    state.copy_selection();

    // Disconnected wire should NOT be included
    assert_eq!(state.clipboard.components.len(), 1);
    assert_eq!(state.clipboard.wires.len(), 0);
}

#[test]
fn test_copy_paste_partial_connected_wire_excluded() {
    let mut state = SchematicState::default();
    // Resistor at (0,0) with terminal at (20,0) with 40x20 dimensions
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    // Wire from terminal to unselected location
    state
        .add_wire(vec![Point::new(20, 0), Point::new(80, 0)])
        .unwrap();

    // Only select one component - wire only connects to one selected component
    state.selection.select_component(r1);
    state.copy_selection();

    // Wire with only one endpoint connected should NOT be included
    // (it would be stretched/disconnected on paste)
    assert_eq!(state.clipboard.wires.len(), 0);
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
