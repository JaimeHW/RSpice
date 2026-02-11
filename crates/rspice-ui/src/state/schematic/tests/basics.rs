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
