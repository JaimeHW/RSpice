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
fn test_mirror_selection_h() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Nmos, Point::new(10, 10));
    state.selection.select_component(id);

    assert!(!state.components[0].mirror_h);
    assert!(!state.components[0].mirror_v);

    state.mirror_selection_h();
    assert!(state.components[0].mirror_h);
    assert!(!state.components[0].mirror_v);

    // Toggle back
    state.mirror_selection_h();
    assert!(!state.components[0].mirror_h);
}

#[test]
fn test_mirror_selection_v() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Nmos, Point::new(10, 10));
    state.selection.select_component(id);

    assert!(!state.components[0].mirror_v);

    state.mirror_selection_v();
    assert!(state.components[0].mirror_v);

    // Toggle back
    state.mirror_selection_v();
    assert!(!state.components[0].mirror_v);
}

#[test]
fn test_mirror_selection_multiple_components() {
    let mut state = SchematicState::default();
    let id1 = state.add_component(ComponentType::Nmos, Point::new(0, 0));
    let id2 = state.add_component(ComponentType::Pmos, Point::new(10, 0));
    let _id3 = state.add_component(ComponentType::Resistor, Point::new(20, 0)); // Not selected

    state.selection.select_component(id1);
    state.selection.select_component(id2);

    state.mirror_selection_h();

    assert!(state.components[0].mirror_h); // id1 selected -> mirrored
    assert!(state.components[1].mirror_h); // id2 selected -> mirrored
    assert!(!state.components[2].mirror_h); // id3 not selected -> not mirrored
}

#[test]
fn test_mirror_selection_marks_dirty() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Nmos, Point::new(0, 0));
    state.is_dirty = false;
    state.selection.select_component(id);

    state.mirror_selection_h();
    assert!(state.is_dirty);
}

#[test]
fn test_mirror_selection_bumps_topology() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Nmos, Point::new(0, 0));
    state.selection.select_component(id);
    let initial_version = state.topology_version();

    state.mirror_selection_h();
    assert!(state.topology_version() > initial_version);
}

#[test]
fn test_mirror_and_rotate_combined() {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Nmos, Point::new(0, 0));
    state.selection.select_component(id);

    // Mirror horizontal, then rotate
    state.mirror_selection_h();
    state.rotate_selection();

    // Both transformations should be applied
    assert!(state.components[0].mirror_h);
    assert_eq!(state.components[0].rotation, Rotation::R90);
}

// =========================================================================
// Box Selection (select_in_rect) Tests
// =========================================================================

#[test]
fn test_select_in_rect_components() {
    let mut state = SchematicState::default();
    // Create components at various positions
    let r1 = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    let r2 = state.add_component(ComponentType::Resistor, Point::new(20, 20));
    let r3 = state.add_component(ComponentType::Resistor, Point::new(50, 50)); // Outside rect

    // Select rectangle from (0,0) to (30,30)
    let count = state.select_in_rect(0, 0, 30, 30, false);

    assert_eq!(count, 2);
    assert!(state.selection.has_component(r1));
    assert!(state.selection.has_component(r2));
    assert!(!state.selection.has_component(r3)); // Outside
}

#[test]
fn test_select_in_rect_wires() {
    let mut state = SchematicState::default();
    // Wire inside rectangle
    let w1 = state
        .add_wire(vec![Point::new(5, 5), Point::new(15, 5)])
        .unwrap();
    // Wire outside rectangle
    let w2 = state
        .add_wire(vec![Point::new(50, 50), Point::new(60, 50)])
        .unwrap();

    let count = state.select_in_rect(0, 0, 30, 30, false);

    assert_eq!(count, 1);
    assert!(state.selection.has_wire(w1));
    assert!(!state.selection.has_wire(w2));
}

#[test]
fn test_select_in_rect_mixed() {
    let mut state = SchematicState::default();
    let c1 = state.add_component(ComponentType::Capacitor, Point::new(10, 10));
    let w1 = state
        .add_wire(vec![Point::new(5, 20), Point::new(25, 20)])
        .unwrap();

    let count = state.select_in_rect(0, 0, 30, 30, false);

    assert_eq!(count, 2);
    assert!(state.selection.has_component(c1));
    assert!(state.selection.has_wire(w1));
}

#[test]
fn test_select_in_rect_add_mode() {
    let mut state = SchematicState::default();
    let c1 = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    let c2 = state.add_component(ComponentType::Resistor, Point::new(50, 50));

    // First select c1
    state.selection.select_component(c1);

    // Now add c2 with add_mode=true
    state.select_in_rect(40, 40, 60, 60, true);

    assert!(state.selection.has_component(c1)); // Still selected
    assert!(state.selection.has_component(c2)); // Added
}

#[test]
fn test_select_in_rect_replace_mode() {
    let mut state = SchematicState::default();
    let c1 = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    let c2 = state.add_component(ComponentType::Resistor, Point::new(50, 50));

    // First select c1
    state.selection.select_component(c1);

    // Now replace with c2 (add_mode=false)
    state.select_in_rect(40, 40, 60, 60, false);

    assert!(!state.selection.has_component(c1)); // Cleared
    assert!(state.selection.has_component(c2)); // Selected
}

#[test]
fn test_select_in_rect_empty() {
    let mut state = SchematicState::default();
    state.add_component(ComponentType::Resistor, Point::new(50, 50));

    // Select in area with no components
    let count = state.select_in_rect(0, 0, 20, 20, false);

    assert_eq!(count, 0);
    assert!(state.selection.is_empty());
}

#[test]
fn test_select_in_rect_boundary() {
    let mut state = SchematicState::default();
    // Component exactly on boundary
    let c1 = state.add_component(ComponentType::Resistor, Point::new(20, 20));

    // Rectangle boundary includes the component position
    let count = state.select_in_rect(20, 20, 30, 30, false);

    assert_eq!(count, 1);
    assert!(state.selection.has_component(c1));
}

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

// =============================================================================
// Undo/Redo Integration Tests (SchematicState Transaction-Based API)
// =============================================================================

#[test]
fn test_undo_initial_state_cannot_undo() {
    let state = SchematicState::default();
    assert!(!state.can_undo());
    assert!(!state.can_redo());
}

#[test]
fn test_undo_add_component() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    assert!(!state.can_undo());

    // Transaction: Add resistor R1
    state.begin_operation("Add resistor R1");
    let _id = state.add_component(ComponentType::Resistor, Point::new(10, 20));
    state.end_operation();

    assert_eq!(state.components.len(), 1);
    assert!(state.can_undo());
    assert!(!state.can_redo());

    // Undo
    assert!(state.undo());
    assert!(state.components.is_empty());
    assert!(!state.can_undo());
    assert!(state.can_redo());
}

#[test]
fn test_redo_add_component() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add resistor R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 20));
    state.end_operation();

    state.undo();

    assert!(state.components.is_empty());
    assert!(state.can_redo());

    // Redo
    assert!(state.redo());
    assert_eq!(state.components.len(), 1);
    assert!(state.can_undo());
    assert!(!state.can_redo());
}

#[test]
fn test_undo_multiple_operations() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Add R1
    state.begin_operation("Add R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    // Add C1
    state.begin_operation("Add C1");
    state.add_component(ComponentType::Capacitor, Point::new(20, 20));
    state.end_operation();

    // Add wire
    state.begin_operation("Add wire");
    state.add_wire(vec![Point::new(12, 10), Point::new(18, 20)]);
    state.end_operation();

    assert_eq!(state.components.len(), 2);
    assert_eq!(state.wires.len(), 1);

    // Undo wire
    state.undo();
    assert_eq!(state.components.len(), 2);
    assert!(state.wires.is_empty());

    // Undo C1
    state.undo();
    assert_eq!(state.components.len(), 1);
    assert_eq!(state.components[0].kind, ComponentType::Resistor);

    // Undo R1
    state.undo();
    assert!(state.components.is_empty());
}

#[test]
fn test_undo_delete_selection() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Add component
    state.begin_operation("Add R1");
    let id = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    state.selection.select_component(id);

    // Delete it
    state.begin_operation("Delete selection");
    state.delete_selection();
    state.end_operation();

    assert!(state.components.is_empty());

    // Undo delete
    state.undo();
    assert_eq!(state.components.len(), 1);
}

#[test]
fn test_undo_rotate_selection() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add R1");
    let id = state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    state.selection.select_component(id);

    assert_eq!(state.components[0].rotation, Rotation::R0);

    // Rotate
    state.begin_operation("Rotate");
    state.rotate_selection();
    state.end_operation();

    assert_eq!(state.components[0].rotation, Rotation::R90);

    // Undo rotation
    state.undo();
    assert_eq!(state.components[0].rotation, Rotation::R0);
}

#[test]
fn test_undo_mirror_selection() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add NMOS");
    let id = state.add_component(ComponentType::Nmos, Point::new(10, 10));
    state.end_operation();

    state.selection.select_component(id);

    assert!(!state.components[0].mirror_h);

    // Mirror
    state.begin_operation("Mirror H");
    state.mirror_selection_h();
    state.end_operation();

    assert!(state.components[0].mirror_h);

    // Undo mirror
    state.undo();
    assert!(!state.components[0].mirror_h);
}

#[test]
fn test_new_action_clears_redo_stack() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    state.begin_operation("Add C1");
    state.add_component(ComponentType::Capacitor, Point::new(20, 20));
    state.end_operation();

    // Undo once
    state.undo();
    assert!(state.can_redo());

    // New action should clear redo
    state.begin_operation("Add L1 instead");
    state.add_component(ComponentType::Inductor, Point::new(30, 30));
    state.end_operation();

    assert!(!state.can_redo());
}

#[test]
fn test_undo_preserves_view_state() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Set some view state
    state.zoom = 2.5;
    state.pan = (100.0, 200.0);

    state.begin_operation("Add R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    // Change view state
    state.zoom = 1.5;
    state.pan = (50.0, 50.0);

    // Undo
    state.undo();

    // Component is gone, but view state SHOULD be preserved (not part of undo)
    assert!(state.components.is_empty());
    assert_eq!(state.zoom, 1.5); // View state not restored
    assert_eq!(state.pan, (50.0, 50.0));
}

#[test]
fn test_redo_description() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add resistor R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    state.undo();

    assert_eq!(state.redo_description(), Some("Add resistor R1"));
}

#[test]
fn test_undo_description() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add resistor R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    assert_eq!(state.undo_description(), Some("Add resistor R1"));
}

#[test]
fn test_reset_undo_history() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add R1");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));
    state.end_operation();

    state.begin_operation("Add C1");
    state.add_component(ComponentType::Capacitor, Point::new(20, 20));
    state.end_operation();

    assert!(state.can_undo());

    // Reset history (e.g., after loading a file)
    state.reset_undo_history();

    assert!(!state.can_undo());
    assert!(!state.can_redo());
    assert_eq!(state.components.len(), 2); // Content still there
}

#[test]
fn test_undo_add_wire() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add wire");
    state.add_wire(vec![Point::new(0, 0), Point::new(10, 0)]);
    state.end_operation();

    assert_eq!(state.wires.len(), 1);

    state.undo();
    assert!(state.wires.is_empty());
}

#[test]
fn test_undo_add_junction() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add junction");
    state.add_junction(Point::new(10, 10));
    state.end_operation();

    assert_eq!(state.junctions.len(), 1);

    state.undo();
    assert!(state.junctions.is_empty());
}

#[test]
fn test_full_edit_workflow_with_undo_redo() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Build a simple circuit
    state.begin_operation("Add R1");
    let r1 = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    state.end_operation();

    state.begin_operation("Add C1");
    let _c1 = state.add_component(ComponentType::Capacitor, Point::new(20, 0));
    state.end_operation();

    state.begin_operation("Add wire");
    let _w1 = state.add_wire(vec![Point::new(2, 0), Point::new(18, 0)]);
    state.end_operation();

    // Select and rotate R1
    state.selection.select_component(r1);
    state.begin_operation("Rotate R1");
    state.rotate_selection();
    state.end_operation();

    // Verify state
    assert_eq!(state.components.len(), 2);
    assert_eq!(state.wires.len(), 1);
    assert_eq!(state.components[0].rotation, Rotation::R90);

    // Undo rotate
    state.undo();
    assert_eq!(state.components[0].rotation, Rotation::R0);

    // Undo wire
    state.undo();
    assert!(state.wires.is_empty());

    // Redo wire
    state.redo();
    assert_eq!(state.wires.len(), 1);

    // Redo rotate
    state.redo();
    assert_eq!(state.components[0].rotation, Rotation::R90);
}

#[test]
fn test_with_undo_helper() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Use the convenience helper
    let created = state.with_undo("Add resistor", |s| {
        s.add_component(ComponentType::Resistor, Point::new(10, 10));
    });

    assert!(created);
    assert_eq!(state.components.len(), 1);
    assert!(state.can_undo());

    state.undo();
    assert!(state.components.is_empty());
}

#[test]
fn test_no_undo_entry_when_nothing_changes() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    // Start an operation but don't actually change anything
    state.begin_operation("Do nothing");
    // (no modifications)
    let entry_created = state.end_operation();

    assert!(!entry_created);
    assert!(!state.can_undo());
}

#[test]
fn test_cancel_operation() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    state.begin_operation("Add component");
    state.add_component(ComponentType::Resistor, Point::new(10, 10));

    // Cancel instead of end
    state.cancel_operation();

    // No undo entry, but component still exists
    assert_eq!(state.components.len(), 1);
    assert!(!state.can_undo()); // No undo entry was created
}

#[test]
fn test_has_pending_operation() {
    let mut state = SchematicState::default();
    state.init_undo_history();

    assert!(!state.has_pending_operation());

    state.begin_operation("Test op");
    assert!(state.has_pending_operation());

    state.end_operation();
    assert!(!state.has_pending_operation());
}
