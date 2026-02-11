use super::*;

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
