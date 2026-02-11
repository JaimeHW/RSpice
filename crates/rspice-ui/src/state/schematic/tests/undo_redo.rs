use super::*;

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
