//! Tests for guarded moves and shove.
//!
//! Every case here is about atomicity: a blocked shove, an unrelated conductor
//! at a moved endpoint, or a coordinate overflow must all leave the schematic
//! unmutated rather than half-moved.

use super::*;
use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Cell, Component, ComponentType,
    DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Junction, Library,
    LibraryCellInstance, LibraryManager, PortDirection, PortSpec, ResolvedCellSymbol,
    SchematicState, SymbolDocument, SymbolPin, SymbolResolver, View, ViewType, Wire,
    WireConnection,
};
use std::collections::HashMap;

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
    }
}

fn resolved_amp_symbol() -> ResolvedCellSymbol {
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };

    let mut libraries = LibraryManager::new();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
    cell.add_view(symbol_view);
    library.add_cell(cell);
    libraries.add_library(library);

    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    SymbolResolver::new(&libraries, &HashMap::new())
        .resolve_binding(&binding)
        .expect("symbol resolves")
}

fn resolved_terminal_points(component: &Component, resolved: &ResolvedCellSymbol) -> Vec<Point> {
    component
        .terminal_positions_resolved(Some(resolved))
        .into_iter()
        .map(|(_, pos)| pos)
        .collect()
}

fn component_terminal_points(component: &Component) -> Vec<Point> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| point)
        .collect()
}

fn selected_amp_with_wire(resolved_pin: Point) -> SchematicState {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let mut schematic = SchematicState::default();
    schematic.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    schematic.wires.push(Wire::segment(
        2,
        resolved_pin,
        Point::new(resolved_pin.x, 0),
    ));
    schematic.selection.select_component(1);
    schematic
}

fn bus_tap_and_scalar_target() -> SchematicState {
    let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
    let bus = Bus::segment(1, Point::new(0, 0), Point::new(10, 0), Some(declaration)).unwrap();
    let tap = BusTap::new(
        2,
        &bus,
        Point::new(5, 0),
        Point::new(0, 10),
        BusSlice::parse("DATA[2]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    let mut state = SchematicState::default();
    state.buses.push(bus);
    state.bus_taps.push(tap);
    state
        .wires
        .push(Wire::segment(3, Point::new(0, 10), Point::new(10, 10)));
    state
}

#[test]
fn moving_selected_cell_uses_resolved_symbol_terminals_for_wire_updates() {
    let resolved = resolved_amp_symbol();
    let mut schematic = selected_amp_with_wire(Point::new(60, 40));

    schematic.move_selection_with_rubber_band_resolved(Point::new(10, 5), |component| {
        resolved_terminal_points(component, &resolved)
    });

    assert_eq!(schematic.components[0].pos, Point::new(110, 55));
    assert_eq!(schematic.wires[0].points[0], Point::new(70, 45));
    assert_eq!(schematic.wires[0].points[1], Point::new(60, 0));
}

#[test]
fn source_only_target_only_and_joint_moves_preserve_tap_attachments() {
    let mut source_only = bus_tap_and_scalar_target();
    source_only.selection.select_only_bus(1);
    source_only.move_selection(Point::new(10, 0));
    assert_eq!(source_only.bus_taps[0].bus_point, Point::new(15, 0));
    assert_eq!(source_only.bus_taps[0].connection_point, Point::new(0, 10));

    let mut target_only = bus_tap_and_scalar_target();
    target_only.selection.select_only_wire(3);
    target_only.move_selection(Point::new(0, 10));
    assert_eq!(target_only.bus_taps[0].bus_point, Point::new(5, 0));
    assert_eq!(target_only.bus_taps[0].connection_point, Point::new(0, 20));

    let mut joint = bus_tap_and_scalar_target();
    joint.selection.select_bus(1);
    joint.selection.select_wire(3);
    joint.move_selection(Point::new(4, 6));
    assert_eq!(joint.bus_taps[0].bus_point, Point::new(9, 6));
    assert_eq!(joint.bus_taps[0].connection_point, Point::new(4, 16));
}

#[test]
fn component_rubber_band_and_junction_moves_keep_scalar_tap_attached() {
    let delta = Point::new(3, 4);
    let mut direct = bus_tap_and_scalar_target();
    direct
        .components
        .push(Component::new(4, ComponentType::Resistor, Point::origin()));
    direct.move_component_with_wires_resolved(4, delta, |_| vec![Point::new(0, 10)]);
    assert_eq!(direct.bus_taps[0].connection_point, Point::new(3, 14));

    let mut selected = bus_tap_and_scalar_target();
    selected
        .components
        .push(Component::new(4, ComponentType::Resistor, Point::origin()));
    selected.selection.select_only_component(4);
    selected.move_selection_resolved(delta, |_| vec![Point::new(0, 10)]);
    assert_eq!(selected.bus_taps[0].connection_point, Point::new(3, 14));

    let mut junction = bus_tap_and_scalar_target();
    junction.junctions.push(Junction::new(5, Point::new(0, 10)));
    junction.move_junction(Point::new(0, 10), Point::new(-2, 12));
    assert_eq!(junction.bus_taps[0].connection_point, Point::new(-2, 12));
}

#[test]
fn move_wire_rejects_missing_and_zero_delta_without_document_side_effects() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.is_dirty = false;
    let topology_before = schematic.topology_version();

    schematic.move_wire(999, Point::new(10, 10));
    schematic.move_wire(3, Point::origin());

    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), topology_before);
}

#[test]
fn extreme_selection_moves_saturate_every_attached_geometry() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.selection.select_bus(1);
    schematic.selection.select_wire(3);

    schematic.move_selection(Point::new(i32::MAX, i32::MAX));

    assert_eq!(schematic.buses[0].points[1], Point::new(i32::MAX, i32::MAX));
    assert_eq!(schematic.wires[0].points[1], Point::new(i32::MAX, i32::MAX));
    assert_eq!(
        schematic.bus_taps[0].connection_point,
        Point::new(i32::MAX, i32::MAX)
    );
}

#[test]
fn bus_and_tap_move_is_one_undoable_redoable_drag_transaction() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.selection.select_only_bus(1);
    let original_bus = schematic.buses[0].clone();
    let original_tap = schematic.bus_taps[0].clone();

    schematic.begin_operation("move selection");
    schematic.move_selection(Point::new(3, 4));
    schematic.move_selection(Point::new(2, 1));
    assert!(schematic.end_operation());
    let moved_bus = schematic.buses[0].clone();
    let moved_tap = schematic.bus_taps[0].clone();

    assert!(schematic.undo());
    assert_eq!(schematic.buses[0], original_bus);
    assert_eq!(schematic.bus_taps[0], original_tap);
    assert!(schematic.redo());
    assert_eq!(schematic.buses[0], moved_bus);
    assert_eq!(schematic.bus_taps[0], moved_tap);
}

#[test]
fn selected_label_moves_with_saturation_and_one_drag_undo_transaction() {
    let original = NetLabel::new(72, Point::new(i32::MAX - 5, -10), "sense");
    let mut schematic = SchematicState::default();
    schematic.net_labels.push(original.clone());
    schematic.selection.select_only_net_label(original.id);
    schematic.init_undo_history();

    schematic.begin_operation("move selection");
    schematic.move_selection_with_rubber_band(Point::new(3, 4));
    schematic.move_selection(Point::new(10, 6));
    assert!(schematic.end_operation());

    assert_eq!(schematic.net_labels[0].pos, Point::new(i32::MAX, 0));
    assert_eq!(schematic.undo_description(), Some("move selection"));
    assert!(schematic.undo());
    assert_eq!(schematic.net_labels, vec![original]);
    assert!(!schematic.can_undo(), "one drag must create one undo step");
    assert!(schematic.redo());
    assert_eq!(schematic.net_labels[0].pos, Point::new(i32::MAX, 0));
}

#[test]
fn selected_design_note_moves_as_one_non_electrical_drag_transaction() {
    let original = DesignNote::new(
        74,
        Point::new(i32::MAX - 5, -10),
        DesignNoteKind::PlainText,
        "Bias network",
    )
    .unwrap();
    let mut schematic = SchematicState::default();
    schematic.design_notes.push(original.clone());
    schematic.selection.select_only_design_note(original.id);
    schematic.init_undo_history();
    let topology = schematic.topology_version();

    schematic.begin_operation("move selection");
    schematic.move_selection_with_rubber_band(Point::new(3, 4));
    schematic.move_selection(Point::new(10, 6));
    assert!(schematic.end_operation());

    assert_eq!(schematic.design_notes[0].pos, Point::new(i32::MAX, 0));
    assert_eq!(schematic.topology_version(), topology);
    assert!(schematic.undo());
    assert_eq!(schematic.design_notes, vec![original]);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn documentation_shape_move_clamps_one_rigid_delta_for_the_entire_selection() {
    let boundary_shape = DocumentationShape::new(
        75,
        DocumentationShapeGeometry::Rectangle {
            first: Point::new(i32::MAX - 10, i32::MIN + 20),
            opposite: Point::new(i32::MAX - 5, i32::MIN + 30),
        },
    )
    .unwrap();
    let companion_shape = DocumentationShape::new(
        76,
        DocumentationShapeGeometry::Line {
            start: Point::new(i32::MAX - 100, i32::MIN + 200),
            end: Point::new(i32::MAX - 90, i32::MIN + 210),
        },
    )
    .unwrap();
    let original = vec![boundary_shape.clone(), companion_shape.clone()];
    let mut schematic = SchematicState::default();
    schematic.documentation_shapes = original.clone();
    schematic
        .selection
        .select_documentation_shape(boundary_shape.id);
    schematic
        .selection
        .select_documentation_shape(companion_shape.id);
    schematic.init_undo_history();
    let topology = schematic.topology_version();

    schematic.begin_operation("move selection");
    schematic.move_selection(Point::new(100, -100));
    assert!(schematic.end_operation());

    assert_eq!(
        schematic.documentation_shapes[0].geometry,
        DocumentationShapeGeometry::Rectangle {
            first: Point::new(i32::MAX - 5, i32::MIN),
            opposite: Point::new(i32::MAX, i32::MIN + 10),
        }
    );
    assert_eq!(
        schematic.documentation_shapes[1].geometry,
        DocumentationShapeGeometry::Line {
            start: Point::new(i32::MAX - 95, i32::MIN + 180),
            end: Point::new(i32::MAX - 85, i32::MIN + 190),
        },
        "every selected shape must receive the same clamped (+5, -20) delta"
    );
    assert_eq!(schematic.topology_version(), topology);
    assert_eq!(schematic.undo_description(), Some("move selection"));
    assert!(schematic.undo());
    assert_eq!(schematic.documentation_shapes, original);
    assert_eq!(schematic.topology_version(), topology);
    assert!(!schematic.can_undo(), "one drag must create one undo step");
}

#[test]
fn unselected_and_read_only_labels_do_not_move() {
    let label = NetLabel::new(73, Point::new(4, 8), "fixed");
    let mut schematic = SchematicState::default();
    schematic.net_labels.push(label.clone());
    schematic.move_selection(Point::new(1, 2));
    assert_eq!(schematic.net_labels, vec![label.clone()]);

    schematic.selection.select_only_net_label(label.id);
    schematic.read_only = true;
    schematic.move_selection(Point::new(1, 2));
    assert_eq!(schematic.net_labels, vec![label]);
}

#[test]
fn stale_label_selection_is_a_clean_move_noop() {
    let mut schematic = SchematicState::default();
    schematic.selection.select_only_net_label(999);
    schematic.is_dirty = false;
    let topology_before = schematic.topology_version();

    schematic.move_selection(Point::new(1, 2));

    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), topology_before);
}

#[test]
fn move_selection_modes_have_stable_dialog_contract() {
    assert_eq!(MoveSelectionMode::default(), MoveSelectionMode::Connected);
    assert_eq!(
        MoveSelectionMode::ALL,
        [
            MoveSelectionMode::Connected,
            MoveSelectionMode::BreakConnections,
            MoveSelectionMode::Shove,
        ]
    );
    assert_eq!(MoveSelectionMode::Connected.label(), "Connected move");
    assert_eq!(
        MoveSelectionMode::BreakConnections.label(),
        "Break connections"
    );
    assert_eq!(MoveSelectionMode::Shove.label(), "Move with shove");
}

#[test]
fn connected_mode_builds_a_deterministic_orthogonal_rubber_band() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 0)));
    schematic.selection.select_only_component(1);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::Connected,
            |_| vec![Point::origin()],
        ),
        Ok(true)
    );

    assert_eq!(schematic.components[0].pos, Point::new(0, 5));
    assert_eq!(
        schematic.wires[0].points,
        vec![Point::new(0, 5), Point::origin(), Point::new(10, 0)]
    );
    assert!(schematic.wires[0].is_orthogonal());
}

#[test]
fn connected_mode_rejects_an_attached_non_orthogonal_wire_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 10)));
    schematic.selection.select_only_component(1);
    schematic.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::Connected,
            |_| vec![Point::origin()],
        ),
        Err(MoveSelectionError::NonOrthogonalWire { wire_id: 2 })
    );
    assert_eq!(schematic.components, original.components);
    assert_eq!(schematic.wires, original.wires);
    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn break_mode_moves_selected_objects_without_attached_conductors() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 0)));
    schematic
        .connections
        .push(WireConnection::new(2, 0, 1, "1"));
    schematic.selection.select_only_component(1);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::BreakConnections,
            |_| vec![Point::origin()],
        ),
        Ok(true)
    );

    assert_eq!(schematic.components[0].pos, Point::new(0, 5));
    assert_eq!(
        schematic.wires[0].points,
        vec![Point::origin(), Point::new(10, 0)]
    );
    assert!(schematic.connections.is_empty());
}

#[test]
fn break_mode_translates_both_endpoints_of_an_explicitly_selected_tap() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.selection.select_only_bus_tap(2);

    assert_eq!(
        schematic.move_selection_with_mode(Point::new(1, 0), MoveSelectionMode::BreakConnections,),
        Ok(true)
    );

    assert_eq!(schematic.buses[0].points[0], Point::new(0, 0));
    assert_eq!(schematic.wires[0].points[0], Point::new(0, 10));
    assert_eq!(schematic.bus_taps[0].bus_point, Point::new(6, 0));
    assert_eq!(schematic.bus_taps[0].connection_point, Point::new(1, 10));
}

#[test]
fn break_mode_rejects_a_selected_tap_that_would_leave_its_source_bus() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.selection.select_only_bus_tap(2);
    schematic.is_dirty = false;
    let taps = schematic.bus_taps.clone();
    let topology = schematic.topology_version();

    assert_eq!(
        schematic.move_selection_with_mode(Point::new(0, 1), MoveSelectionMode::BreakConnections,),
        Err(MoveSelectionError::InvalidTapAttachment { tap_id: 2 })
    );

    assert_eq!(schematic.bus_taps, taps);
    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn shove_mode_chooses_a_deterministic_clear_orthogonal_route() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(60, 0)));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(22, 5), Point::new(58, 5)));
    schematic
        .connections
        .push(WireConnection::new(2, 0, 1, "1"));
    schematic.selection.select_only_component(1);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(schematic.components[0].pos, Point::new(0, 5));
    assert_eq!(
        schematic.wires[0].points,
        vec![Point::new(20, 5), Point::new(20, 0), Point::new(60, 0)]
    );
    assert!(schematic.wires[0].is_orthogonal());
    assert_eq!(schematic.connections[0].point_index, 0);
}

#[test]
fn shove_keeps_a_selected_wire_attached_to_an_unselected_component() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(100, 0)));
    schematic
        .connections
        .push(WireConnection::new(2, 0, 1, "-"));
    schematic.selection.select_only_wire(2);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(schematic.wires[0].points[0], Point::new(20, 0));
    assert_eq!(
        *schematic.wires[0].points.last().unwrap(),
        Point::new(100, 20)
    );
    assert!(schematic.wires[0].is_orthogonal());
    assert_eq!(schematic.connections[0].point_index, 0);
    assert!(
        component_terminal_points(&schematic.components[0])
            .contains(&schematic.wires[0].points[schematic.connections[0].point_index])
    );
}

#[test]
fn shove_routes_around_component_bodies() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic.components.push(Component::new(
        2,
        ComponentType::Resistor,
        Point::new(50, 20),
    ));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(20, 0), Point::new(100, 0)));
    schematic.selection.select_only_component(1);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    let obstacle = &schematic.components[1];
    let obstacle_terminals = component_terminal_points(obstacle);
    assert!(schematic.wires[0].is_orthogonal());
    assert!(
        schematic.wires[0]
            .segments()
            .all(|segment| !segment_enters_component_body(segment, obstacle, &obstacle_terminals,))
    );
    assert_eq!(
        schematic.wires[0].points,
        vec![Point::new(20, 20), Point::new(20, 0), Point::new(100, 0)]
    );
}

#[test]
fn component_body_collision_only_allows_outward_terminal_access() {
    let component = Component::new(1, ComponentType::Resistor, Point::origin());
    let terminals = component_terminal_points(&component);

    assert!(!segment_enters_component_body(
        WireSegment::new(Point::new(20, 0), Point::new(100, 0)),
        &component,
        &terminals,
    ));
    assert!(segment_enters_component_body(
        WireSegment::new(Point::new(-100, 0), Point::new(20, 0)),
        &component,
        &terminals,
    ));
}

#[test]
fn shove_reroutes_a_selected_wire_around_a_component_body() {
    let mut schematic = SchematicState::default();
    schematic.grid_size = 10;
    schematic.components.push(Component::new(
        1,
        ComponentType::Resistor,
        Point::new(50, 20),
    ));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(0, 0), Point::new(100, 0)));
    schematic.selection.select_only_wire(2);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    let obstacle = &schematic.components[0];
    let terminals = component_terminal_points(obstacle);
    assert!(
        schematic.wires[0]
            .segments()
            .all(|segment| !segment_enters_component_body(segment, obstacle, &terminals))
    );
    assert_eq!(
        schematic.wires[0].points,
        vec![
            Point::new(0, 20),
            Point::new(0, 10),
            Point::new(100, 10),
            Point::new(100, 20),
        ]
    );
}

#[test]
fn shove_search_lanes_remain_aligned_to_the_active_grid() {
    let mut schematic = SchematicState::default();
    schematic.grid_size = 10;
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(80, 0)));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(25, 10), Point::new(75, 10)));
    schematic
        .wires
        .push(Wire::segment(4, Point::new(50, -5), Point::new(50, 5)));
    schematic.selection.select_only_component(1);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 10),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.wires[0].points,
        vec![
            Point::new(20, 10),
            Point::new(20, -10),
            Point::new(80, -10),
            Point::new(80, 0),
        ]
    );
    assert!(
        schematic.wires[0]
            .points
            .iter()
            .all(|point| point.x % 10 == 0 && point.y % 10 == 0)
    );
}

#[test]
fn shove_mode_keeps_a_scalar_tap_on_the_rerouted_wire_endpoint() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.components.push(Component::new(
        4,
        ComponentType::Resistor,
        Point::new(-20, 10),
    ));
    schematic.selection.select_only_component(4);

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            |_| vec![Point::new(0, 10)],
        ),
        Ok(true)
    );

    assert_eq!(schematic.bus_taps[0].connection_point, Point::new(0, 15));
    assert!(schematic.wires[0].contains_point(Point::new(0, 15)));
}

#[test]
fn shove_failure_is_atomic_when_the_bounded_search_is_blocked() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(60, 0)));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(40, -100), Point::new(40, 100)));
    schematic.selection.select_only_component(1);
    schematic.is_dirty = false;
    let components = schematic.components.clone();
    let wires = schematic.wires.clone();
    let topology = schematic.topology_version();

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 2 })
    );

    assert_eq!(schematic.components, components);
    assert_eq!(schematic.wires, wires);
    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn shove_rejects_an_unrelated_conductor_at_a_moved_endpoint_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(100, 0)));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(20, 15), Point::new(20, 25)));
    schematic.selection.select_only_component(1);
    schematic.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 2 })
    );

    assert_eq!(schematic.components, original.components);
    assert_eq!(schematic.wires, original.wires);
    assert_eq!(schematic.connections, original.connections);
    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn shove_rejects_an_unrelated_component_terminal_at_a_moved_endpoint_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic.components.push(Component::new(
        2,
        ComponentType::Resistor,
        Point::new(40, 20),
    ));
    schematic
        .wires
        .push(Wire::segment(3, Point::new(20, 0), Point::new(100, 0)));
    schematic.selection.select_only_component(1);
    schematic.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_selection_with_mode_resolved(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 3 })
    );

    assert_eq!(schematic.components, original.components);
    assert_eq!(schematic.wires, original.wires);
    assert_eq!(schematic.connections, original.connections);
    assert!(!schematic.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn guarded_modes_reject_coordinate_overflow_without_mutation() {
    for mode in MoveSelectionMode::ALL {
        let mut schematic = SchematicState::default();
        schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(i32::MAX, 0),
        ));
        schematic.selection.select_only_component(1);
        schematic.is_dirty = false;
        let topology = schematic.topology_version();

        assert_eq!(
            schematic.move_selection_with_mode(Point::new(1, 0), mode),
            Err(MoveSelectionError::CoordinateOverflow)
        );
        assert_eq!(schematic.components[0].pos, Point::new(i32::MAX, 0));
        assert!(!schematic.is_dirty);
        assert_eq!(schematic.topology_version(), topology);
    }
}

#[test]
fn mode_aware_move_has_clean_zero_stale_and_read_only_noops() {
    let mut zero = SchematicState::default();
    zero.components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    zero.selection.select_only_component(1);
    assert_eq!(
        zero.move_selection_with_mode(Point::origin(), MoveSelectionMode::Shove),
        Ok(false)
    );
    assert!(!zero.is_dirty);

    let mut stale = SchematicState::default();
    stale.selection.select_only_component(999);
    assert_eq!(
        stale.move_selection_with_mode(Point::new(1, 1), MoveSelectionMode::BreakConnections,),
        Ok(false)
    );
    assert!(!stale.is_dirty);

    let mut read_only = zero;
    read_only.read_only = true;
    assert_eq!(
        read_only.move_selection_with_mode(Point::new(1, 1), MoveSelectionMode::Connected,),
        Ok(false)
    );
    assert_eq!(read_only.components[0].pos, Point::origin());
    assert!(!read_only.is_dirty);
}
