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
    SchematicProbe, SchematicSnapshot, SchematicState, SymbolDocument, SymbolPin, SymbolResolver,
    View, ViewType, Wire, WireConnection,
};
use rspice_design::connectivity::summary::net_names_by_terminal;
use std::collections::HashMap;

/// The moves under test, in documents with one sheet, which hide nothing.
impl SchematicState {
    fn rubber_band_on_one_sheet(&mut self, delta: Point) {
        self.rubber_band_resolved_on_one_sheet(
            delta,
            DragRubberBand::default(),
            legacy_terminal_points,
        );
    }

    fn rubber_band_resolved_on_one_sheet(
        &mut self,
        delta: Point,
        rubber_band: DragRubberBand,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        self.with_single_sheet_topology(|state, scope| {
            state.move_selection_with_rubber_band_resolved(
                delta,
                rubber_band,
                terminal_points_for,
                scope,
            );
        });
    }

    fn move_with_mode_on_one_sheet(
        &mut self,
        delta: Point,
        mode: MoveSelectionMode,
    ) -> Result<bool, MoveSelectionError> {
        self.move_with_mode_resolved_on_one_sheet(delta, mode, legacy_terminal_points)
    }

    fn move_with_mode_resolved_on_one_sheet(
        &mut self,
        delta: Point,
        mode: MoveSelectionMode,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> Result<bool, MoveSelectionError> {
        self.with_single_sheet_topology(|state, scope| {
            state.move_selection_with_mode_resolved(delta, mode, terminal_points_for, scope)
        })
    }

    fn move_component_on_one_sheet(
        &mut self,
        component_id: u64,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        self.with_single_sheet_topology(|state, scope| {
            state.move_components_with_wires_resolved(
                &[(component_id, delta)],
                terminal_points_for,
                scope,
            );
        });
    }
}

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

    SymbolResolver::<rspice_design::schematic::owned::Schematic>::new(&libraries, &HashMap::new())
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
    schematic.design.document_mut_for_test().components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(
            2,
            resolved_pin,
            Point::new(resolved_pin.x, 0),
        ));
    schematic.session.editor.selection.select_component(1);
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
    state.design.document_mut_for_test().buses.push(bus);
    state.design.document_mut_for_test().bus_taps.push(tap);
    state
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(0, 10), Point::new(10, 10)));
    state
}

/// The wire follows the cell's resolved pin, not a primitive's, and stays
/// square on the way: it leaves the moved pin, turns back onto where the
/// pin was and runs on as drawn.
#[test]
fn moving_selected_cell_uses_resolved_symbol_terminals_for_wire_updates() {
    let resolved = resolved_amp_symbol();
    let mut schematic = selected_amp_with_wire(Point::new(60, 40));

    schematic.rubber_band_resolved_on_one_sheet(
        Point::new(10, 5),
        DragRubberBand::Orthogonal,
        |component| resolved_terminal_points(component, &resolved),
    );

    assert_eq!(
        schematic.design.document().components[0].pos,
        Point::new(110, 55)
    );
    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![
            Point::new(70, 45),
            Point::new(70, 40),
            Point::new(60, 40),
            Point::new(60, 0),
        ]
    );
}

/// Under angled routing the wire stretches straight from the cell's
/// resolved pin.
#[test]
fn an_angled_drag_stretches_the_wire_straight_from_the_resolved_pin() {
    let resolved = resolved_amp_symbol();
    let mut schematic = selected_amp_with_wire(Point::new(60, 40));

    schematic.rubber_band_resolved_on_one_sheet(
        Point::new(10, 5),
        DragRubberBand::Direct,
        |component| resolved_terminal_points(component, &resolved),
    );

    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![Point::new(70, 45), Point::new(60, 0)]
    );
}

#[test]
fn source_only_target_only_and_joint_moves_preserve_tap_attachments() {
    let mut source_only = bus_tap_and_scalar_target();
    source_only.session.editor.selection.select_only_bus(1);
    source_only.move_selection(Point::new(10, 0));
    assert_eq!(
        source_only.design.document().bus_taps[0].bus_point,
        Point::new(15, 0)
    );
    assert_eq!(
        source_only.design.document().bus_taps[0].connection_point,
        Point::new(0, 10)
    );

    let mut target_only = bus_tap_and_scalar_target();
    target_only.session.editor.selection.select_only_wire(3);
    target_only.move_selection(Point::new(0, 10));
    assert_eq!(
        target_only.design.document().bus_taps[0].bus_point,
        Point::new(5, 0)
    );
    assert_eq!(
        target_only.design.document().bus_taps[0].connection_point,
        Point::new(0, 20)
    );

    let mut joint = bus_tap_and_scalar_target();
    joint.session.editor.selection.select_bus(1);
    joint.session.editor.selection.select_wire(3);
    joint.move_selection(Point::new(4, 6));
    assert_eq!(
        joint.design.document().bus_taps[0].bus_point,
        Point::new(9, 6)
    );
    assert_eq!(
        joint.design.document().bus_taps[0].connection_point,
        Point::new(4, 16)
    );
}

#[test]
fn component_rubber_band_and_junction_moves_keep_scalar_tap_attached() {
    let delta = Point::new(3, 4);
    let mut direct = bus_tap_and_scalar_target();
    direct
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(4, ComponentType::Resistor, Point::origin()));
    direct.move_component_on_one_sheet(4, delta, |component| {
        vec![Point::new(component.pos.x, component.pos.y + 10)]
    });
    assert_eq!(
        direct.design.document().bus_taps[0].connection_point,
        Point::new(3, 14)
    );

    let mut selected = bus_tap_and_scalar_target();
    selected
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(4, ComponentType::Resistor, Point::origin()));
    selected.session.editor.selection.select_only_component(4);
    selected.move_selection_resolved(delta, |_| vec![Point::new(0, 10)]);
    assert_eq!(
        selected.design.document().bus_taps[0].connection_point,
        Point::new(3, 14)
    );

    let mut junction = bus_tap_and_scalar_target();
    junction
        .design
        .document_mut_for_test()
        .junctions
        .push(Junction::new(5, Point::new(0, 10)));
    junction
        .design
        .move_junction(Point::new(0, 10), Point::new(-2, 12));
    assert_eq!(
        junction.design.document().bus_taps[0].connection_point,
        Point::new(-2, 12)
    );
}

#[test]
fn move_wire_rejects_missing_and_zero_delta_without_document_side_effects() {
    let mut schematic = bus_tap_and_scalar_target();
    let topology_before = schematic.topology_version();

    assert!(!schematic.design.move_wire(999, Point::new(10, 10)).changed);
    assert!(!schematic.design.move_wire(3, Point::origin()).changed);

    assert_eq!(schematic.topology_version(), topology_before);
}

#[test]
fn extreme_selection_moves_saturate_every_attached_geometry() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.session.editor.selection.select_bus(1);
    schematic.session.editor.selection.select_wire(3);

    schematic.move_selection(Point::new(i32::MAX, i32::MAX));

    assert_eq!(
        schematic.design.document().buses[0].points[1],
        Point::new(i32::MAX, i32::MAX)
    );
    assert_eq!(
        schematic.design.document().wires[0].points[1],
        Point::new(i32::MAX, i32::MAX)
    );
    assert_eq!(
        schematic.design.document().bus_taps[0].connection_point,
        Point::new(i32::MAX, i32::MAX)
    );
}

#[test]
fn bus_and_tap_move_is_one_undoable_redoable_drag_transaction() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.session.editor.selection.select_only_bus(1);
    let original_bus = schematic.design.document().buses[0].clone();
    let original_tap = schematic.design.document().bus_taps[0].clone();

    schematic.begin_operation("move selection");
    schematic.move_selection(Point::new(3, 4));
    schematic.move_selection(Point::new(2, 1));
    assert!(schematic.end_operation());
    let moved_bus = schematic.design.document().buses[0].clone();
    let moved_tap = schematic.design.document().bus_taps[0].clone();

    assert!(schematic.undo());
    assert_eq!(schematic.design.document().buses[0], original_bus);
    assert_eq!(schematic.design.document().bus_taps[0], original_tap);
    assert!(schematic.redo());
    assert_eq!(schematic.design.document().buses[0], moved_bus);
    assert_eq!(schematic.design.document().bus_taps[0], moved_tap);
}

#[test]
fn selected_label_moves_with_saturation_and_one_drag_undo_transaction() {
    let original = NetLabel::new(72, Point::new(i32::MAX - 5, -10), "sense");
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .net_labels
        .push(original.clone());
    schematic
        .session
        .editor
        .selection
        .select_only_net_label(original.id);
    schematic.init_undo_history();

    schematic.begin_operation("move selection");
    schematic.rubber_band_on_one_sheet(Point::new(3, 4));
    schematic.move_selection(Point::new(10, 6));
    assert!(schematic.end_operation());

    assert_eq!(
        schematic.design.document().net_labels[0].pos,
        Point::new(i32::MAX, 0)
    );
    assert_eq!(schematic.undo_description(), Some("move selection"));
    assert!(schematic.undo());
    assert_eq!(schematic.design.document().net_labels, vec![original]);
    assert!(!schematic.can_undo(), "one drag must create one undo step");
    assert!(schematic.redo());
    assert_eq!(
        schematic.design.document().net_labels[0].pos,
        Point::new(i32::MAX, 0)
    );
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
    schematic
        .design
        .document_mut_for_test()
        .design_notes
        .push(original.clone());
    schematic
        .session
        .editor
        .selection
        .select_only_design_note(original.id);
    schematic.init_undo_history();
    let topology = schematic.topology_version();

    schematic.begin_operation("move selection");
    schematic.rubber_band_on_one_sheet(Point::new(3, 4));
    schematic.move_selection(Point::new(10, 6));
    assert!(schematic.end_operation());

    assert_eq!(
        schematic.design.document().design_notes[0].pos,
        Point::new(i32::MAX, 0)
    );
    assert_eq!(schematic.topology_version(), topology);
    assert!(schematic.undo());
    assert_eq!(schematic.design.document().design_notes, vec![original]);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn selected_probe_moves_as_one_non_electrical_drag_transaction() {
    let original =
        SchematicProbe::new(75, Point::new(10, 20), "V(out)", Some("V(out)".to_owned())).unwrap();
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .probes
        .push(original.clone());
    schematic
        .session
        .editor
        .selection
        .select_only_probe(original.id);
    schematic.init_undo_history();
    let topology = schematic.topology_version();

    schematic.begin_operation("move selection");
    schematic.rubber_band_on_one_sheet(Point::new(3, 4));
    schematic.move_selection(Point::new(7, 6));
    assert!(schematic.end_operation());

    assert_eq!(
        schematic.design.document().probes[0].position,
        Point::new(20, 30)
    );
    assert_eq!(schematic.topology_version(), topology);
    assert_eq!(schematic.undo_description(), Some("move selection"));
    assert!(schematic.undo());
    assert_eq!(schematic.design.document().probes, vec![original]);
    assert_eq!(schematic.topology_version(), topology);
    assert!(!schematic.can_undo());
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
    schematic
        .design
        .document_mut_for_test()
        .documentation_shapes = original.clone();
    schematic
        .session
        .editor
        .selection
        .select_documentation_shape(boundary_shape.id);
    schematic
        .session
        .editor
        .selection
        .select_documentation_shape(companion_shape.id);
    schematic.init_undo_history();
    let topology = schematic.topology_version();

    schematic.begin_operation("move selection");
    schematic.move_selection(Point::new(100, -100));
    assert!(schematic.end_operation());

    assert_eq!(
        schematic.design.document().documentation_shapes[0].geometry,
        DocumentationShapeGeometry::Rectangle {
            first: Point::new(i32::MAX - 5, i32::MIN),
            opposite: Point::new(i32::MAX, i32::MIN + 10),
        }
    );
    assert_eq!(
        schematic.design.document().documentation_shapes[1].geometry,
        DocumentationShapeGeometry::Line {
            start: Point::new(i32::MAX - 95, i32::MIN + 180),
            end: Point::new(i32::MAX - 85, i32::MIN + 190),
        },
        "every selected shape must receive the same clamped (+5, -20) delta"
    );
    assert_eq!(schematic.topology_version(), topology);
    assert_eq!(schematic.undo_description(), Some("move selection"));
    assert!(schematic.undo());
    assert_eq!(schematic.design.document().documentation_shapes, original);
    assert_eq!(schematic.topology_version(), topology);
    assert!(!schematic.can_undo(), "one drag must create one undo step");
}

#[test]
fn unselected_and_read_only_labels_do_not_move() {
    let label = NetLabel::new(73, Point::new(4, 8), "fixed");
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    schematic.move_selection(Point::new(1, 2));
    assert_eq!(schematic.design.document().net_labels, vec![label.clone()]);

    schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);
    schematic.session.read_only = true;
    schematic.move_selection(Point::new(1, 2));
    assert_eq!(schematic.design.document().net_labels, vec![label]);
}

#[test]
fn stale_label_selection_is_a_clean_move_noop() {
    let mut schematic = SchematicState::default();
    schematic
        .session
        .editor
        .selection
        .select_only_net_label(999);
    schematic.session.is_dirty = false;
    let topology_before = schematic.topology_version();

    schematic.move_selection(Point::new(1, 2));

    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), topology_before);
}

#[test]
fn connected_mode_builds_a_deterministic_orthogonal_rubber_band() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 0)));
    schematic.session.editor.selection.select_only_component(1);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::Connected,
            |_| vec![Point::origin()],
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().components[0].pos,
        Point::new(0, 5)
    );
    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![Point::new(0, 5), Point::origin(), Point::new(10, 0)]
    );
    assert!(schematic.design.document().wires[0].is_orthogonal());
}

/// Moving a part diagonally toward its wire's far end turns the wire once
/// under the moved pin; the old pin position is not kept as a corner that
/// makes the wire double back on itself.
#[test]
fn connected_mode_toward_the_far_end_turns_once_without_a_hook() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 0)));
    schematic.session.editor.selection.select_only_component(1);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(4, 5),
            MoveSelectionMode::Connected,
            |_| vec![Point::origin()],
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![Point::new(4, 5), Point::new(4, 0), Point::new(10, 0)]
    );
}

#[test]
fn connected_mode_rejects_an_attached_non_orthogonal_wire_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 10)));
    schematic.session.editor.selection.select_only_component(1);
    schematic.session.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::Connected,
            |_| vec![Point::origin()],
        ),
        Err(MoveSelectionError::NonOrthogonalWire { wire_id: 2 })
    );
    assert_eq!(
        schematic.design.document().components,
        original.design.document().components
    );
    assert_eq!(
        schematic.design.document().wires,
        original.design.document().wires
    );
    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn break_mode_moves_selected_objects_without_attached_conductors() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::origin(), Point::new(10, 0)));
    schematic
        .design
        .document_mut_for_test()
        .connections
        .push(WireConnection::new(2, 0, 1, "1"));
    schematic.session.editor.selection.select_only_component(1);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::BreakConnections,
            |_| vec![Point::origin()],
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().components[0].pos,
        Point::new(0, 5)
    );
    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![Point::origin(), Point::new(10, 0)]
    );
    assert!(schematic.design.document().connections.is_empty());
}

#[test]
fn break_mode_translates_both_endpoints_of_an_explicitly_selected_tap() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.session.editor.selection.select_only_bus_tap(2);

    assert_eq!(
        schematic
            .move_with_mode_on_one_sheet(Point::new(1, 0), MoveSelectionMode::BreakConnections,),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().buses[0].points[0],
        Point::new(0, 0)
    );
    assert_eq!(
        schematic.design.document().wires[0].points[0],
        Point::new(0, 10)
    );
    assert_eq!(
        schematic.design.document().bus_taps[0].bus_point,
        Point::new(6, 0)
    );
    assert_eq!(
        schematic.design.document().bus_taps[0].connection_point,
        Point::new(1, 10)
    );
}

#[test]
fn break_mode_rejects_a_selected_tap_that_would_leave_its_source_bus() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.session.editor.selection.select_only_bus_tap(2);
    schematic.session.is_dirty = false;
    let taps = schematic.design.document().bus_taps.clone();
    let topology = schematic.topology_version();

    assert_eq!(
        schematic
            .move_with_mode_on_one_sheet(Point::new(0, 1), MoveSelectionMode::BreakConnections,),
        Err(MoveSelectionError::InvalidTapAttachment { tap_id: 2 })
    );

    assert_eq!(schematic.design.document().bus_taps, taps);
    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn shove_mode_chooses_a_deterministic_clear_orthogonal_route() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(60, 0)));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(22, 5), Point::new(58, 5)));
    schematic
        .design
        .document_mut_for_test()
        .connections
        .push(WireConnection::new(2, 0, 1, "1"));
    schematic.session.editor.selection.select_only_component(1);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().components[0].pos,
        Point::new(0, 5)
    );
    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![Point::new(20, 5), Point::new(20, 0), Point::new(60, 0)]
    );
    assert!(schematic.design.document().wires[0].is_orthogonal());
    assert_eq!(schematic.design.document().connections[0].point_index, 0);
}

#[test]
fn shove_keeps_a_selected_wire_attached_to_an_unselected_component() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(100, 0)));
    schematic
        .design
        .document_mut_for_test()
        .connections
        .push(WireConnection::new(2, 0, 1, "-"));
    schematic.session.editor.selection.select_only_wire(2);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().wires[0].points[0],
        Point::new(20, 0)
    );
    assert_eq!(
        *schematic.design.document().wires[0].points.last().unwrap(),
        Point::new(100, 20)
    );
    assert!(schematic.design.document().wires[0].is_orthogonal());
    assert_eq!(schematic.design.document().connections[0].point_index, 0);
    assert!(
        component_terminal_points(&schematic.design.document().components[0]).contains(
            &schematic.design.document().wires[0].points
                [schematic.design.document().connections[0].point_index]
        )
    );
}

#[test]
fn shove_search_lanes_remain_aligned_to_the_active_grid() {
    let mut schematic = SchematicState::default();
    schematic.design.document_mut_for_test().grid_size = 10;
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(80, 0)));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(25, 10), Point::new(75, 10)));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(4, Point::new(50, -5), Point::new(50, 5)));
    schematic.session.editor.selection.select_only_component(1);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 10),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().wires[0].points,
        vec![
            Point::new(20, 10),
            Point::new(20, -10),
            Point::new(80, -10),
            Point::new(80, 0),
        ]
    );
    assert!(
        schematic.design.document().wires[0]
            .points
            .iter()
            .all(|point| point.x % 10 == 0 && point.y % 10 == 0)
    );
}

#[test]
fn shove_mode_keeps_a_scalar_tap_on_the_rerouted_wire_endpoint() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(
            4,
            ComponentType::Resistor,
            Point::new(-20, 10),
        ));
    schematic.session.editor.selection.select_only_component(4);

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            |_| vec![Point::new(0, 10)],
        ),
        Ok(true)
    );

    assert_eq!(
        schematic.design.document().bus_taps[0].connection_point,
        Point::new(0, 15)
    );
    assert!(schematic.design.document().wires[0].contains_point(Point::new(0, 15)));
}

#[test]
fn shove_failure_is_atomic_when_the_bounded_search_is_blocked() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(60, 0)));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(40, -100), Point::new(40, 100)));
    schematic.session.editor.selection.select_only_component(1);
    schematic.session.is_dirty = false;
    let components = schematic.design.document().components.clone();
    let wires = schematic.design.document().wires.clone();
    let topology = schematic.topology_version();

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 5),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 2 })
    );

    assert_eq!(schematic.design.document().components, components);
    assert_eq!(schematic.design.document().wires, wires);
    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), topology);
}

#[test]
fn shove_rejects_an_unrelated_conductor_at_a_moved_endpoint_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(2, Point::new(20, 0), Point::new(100, 0)));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(20, 15), Point::new(20, 25)));
    schematic.session.editor.selection.select_only_component(1);
    schematic.session.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 2 })
    );

    assert_eq!(
        schematic.design.document().components,
        original.design.document().components
    );
    assert_eq!(
        schematic.design.document().wires,
        original.design.document().wires
    );
    assert_eq!(
        schematic.design.document().connections,
        original.design.document().connections
    );
    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn shove_rejects_an_unrelated_component_terminal_at_a_moved_endpoint_atomically() {
    let mut schematic = SchematicState::default();
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    schematic
        .design
        .document_mut_for_test()
        .components
        .push(Component::new(
            2,
            ComponentType::Resistor,
            Point::new(40, 20),
        ));
    schematic
        .design
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(20, 0), Point::new(100, 0)));
    schematic.session.editor.selection.select_only_component(1);
    schematic.session.is_dirty = false;
    let original = schematic.clone();

    assert_eq!(
        schematic.move_with_mode_resolved_on_one_sheet(
            Point::new(0, 20),
            MoveSelectionMode::Shove,
            component_terminal_points,
        ),
        Err(MoveSelectionError::NoLegalShoveRoute { wire_id: 3 })
    );

    assert_eq!(
        schematic.design.document().components,
        original.design.document().components
    );
    assert_eq!(
        schematic.design.document().wires,
        original.design.document().wires
    );
    assert_eq!(
        schematic.design.document().connections,
        original.design.document().connections
    );
    assert!(!schematic.session.is_dirty);
    assert_eq!(schematic.topology_version(), original.topology_version());
}

#[test]
fn guarded_modes_reject_coordinate_overflow_without_mutation() {
    for mode in MoveSelectionMode::ALL {
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .components
            .push(Component::new(
                1,
                ComponentType::Resistor,
                Point::new(i32::MAX, 0),
            ));
        schematic.session.editor.selection.select_only_component(1);
        schematic.session.is_dirty = false;
        let topology = schematic.topology_version();

        assert_eq!(
            schematic.move_with_mode_on_one_sheet(Point::new(1, 0), mode),
            Err(MoveSelectionError::CoordinateOverflow)
        );
        assert_eq!(
            schematic.design.document().components[0].pos,
            Point::new(i32::MAX, 0)
        );
        assert!(!schematic.session.is_dirty);
        assert_eq!(schematic.topology_version(), topology);
    }
}

#[test]
fn mode_aware_move_has_clean_zero_stale_and_read_only_noops() {
    let mut zero = SchematicState::default();
    zero.design
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    zero.session.editor.selection.select_only_component(1);
    assert_eq!(
        zero.move_with_mode_on_one_sheet(Point::origin(), MoveSelectionMode::Shove),
        Ok(false)
    );
    assert!(!zero.session.is_dirty);

    let mut stale = SchematicState::default();
    stale.session.editor.selection.select_only_component(999);
    assert_eq!(
        stale.move_with_mode_on_one_sheet(Point::new(1, 1), MoveSelectionMode::BreakConnections,),
        Ok(false)
    );
    assert!(!stale.session.is_dirty);

    let mut read_only = zero;
    read_only.session.read_only = true;
    assert_eq!(
        read_only.move_with_mode_on_one_sheet(Point::new(1, 1), MoveSelectionMode::Connected,),
        Ok(false)
    );
    assert_eq!(
        read_only.design.document().components[0].pos,
        Point::origin()
    );
    assert!(!read_only.session.is_dirty);
}

/// A sheet with something attached to every pin and wire R1 carries: the
/// label OUT on R1's right pin, joined by name to the label on R3's pin; a
/// typed tap whose wire end sits on R1's left pin; a probe on R1's output
/// wire; and a dot where R2's wire tees into it. Nothing but R1 is selected.
fn attached_sheet() -> SchematicState {
    let mut state = SchematicState::default();
    let document = state.design.document_mut_for_test();
    document.components.push(Component::new(
        1,
        ComponentType::Resistor,
        Point::new(100, 100),
    ));
    let mut r2 = Component::new(2, ComponentType::Resistor, Point::new(180, 180));
    r2.rotation = crate::state::Rotation::R90;
    document.components.push(r2);
    document.components.push(Component::new(
        3,
        ComponentType::Resistor,
        Point::new(400, 200),
    ));
    document.wires.push(Wire::segment(
        10,
        Point::new(120, 100),
        Point::new(240, 100),
    ));
    document.wires.push(Wire::segment(
        11,
        Point::new(180, 100),
        Point::new(180, 160),
    ));
    document
        .wires
        .push(Wire::segment(12, Point::new(80, 100), Point::new(80, 140)));
    document
        .junctions
        .push(Junction::new(20, Point::new(180, 100)));
    document
        .net_labels
        .push(NetLabel::new(30, Point::new(120, 100), "OUT"));
    document
        .net_labels
        .push(NetLabel::new(31, Point::new(380, 200), "OUT"));
    let bus = Bus::segment(
        40,
        Point::new(60, 60),
        Point::new(60, 140),
        Some(BusDeclaration::parse("D[1:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        41,
        &bus,
        Point::new(60, 100),
        Point::new(80, 100),
        BusSlice::parse("D[0]").unwrap(),
        BusTapOrientation::Right,
    )
    .unwrap();
    document.buses.push(bus);
    document.bus_taps.push(tap);
    document.probes.push(
        SchematicProbe::new(
            50,
            Point::new(150, 100),
            "V(OUT)",
            Some("V(OUT)".to_owned()),
        )
        .unwrap(),
    );
    state.session.editor.selection.select_only_component(1);
    state
}

/// The edit left every pin on the net it had, under the name it had, with
/// every wire square, the probe on a wire, the tap valid and the dot on the
/// wires it joins.
fn assert_nets_kept(
    schematic: &SchematicState,
    before: &HashMap<(u64, String), String>,
    edit: &str,
) {
    let document = schematic.design.document();
    assert_eq!(
        &net_names_by_terminal(document),
        before,
        "{edit} changed the netlist"
    );
    for wire in &document.wires {
        assert!(
            wire.is_orthogonal(),
            "{edit} left wire {} slanted: {:?}",
            wire.id,
            wire.points
        );
    }
    let probe = document.probes[0].position;
    assert!(
        document.wires.iter().any(|wire| wire.contains_point(probe)),
        "{edit} left the probe off its wire at {probe:?}"
    );
    let tap = &document.bus_taps[0];
    assert!(
        document.buses[0].contains_point(tap.bus_point)
            && document
                .wires
                .iter()
                .any(|wire| wire.contains_point(tap.connection_point)),
        "{edit} left the tap off its bus or wire: {tap:?}"
    );
    for junction in &document.junctions {
        assert!(
            document
                .wires
                .iter()
                .filter(|wire| wire.contains_point(junction.pos))
                .count()
                >= 2,
            "{edit} left a dot off the wires it joined at {:?}",
            junction.pos
        );
    }
}

/// What the editor does after a move: a pin taken off a pin keeps a wire to
/// it, a pin dropped in a wire splits it, and the junction policy tidies.
fn tidy_after_move(schematic: &mut SchematicState) {
    let moved = schematic.session.editor.selection.components.clone();
    let automatic_junctions = schematic.session.editor.automatic_junctions;
    schematic.with_single_sheet_topology(|state, scope| {
        state.connect_moved_pins(&moved, legacy_terminal_points, scope);
        state.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope);
    });
}

#[test]
fn a_drag_carries_the_label_tap_and_probe_on_the_parts_pins_and_wires() {
    let mut schematic = attached_sheet();
    let before = net_names_by_terminal(schematic.design.document());
    schematic.begin_operation("drag");
    // Two frames, as a drag lays them: the second is laid from where the
    // gesture began, not from the first.
    schematic.rubber_band_on_one_sheet(Point::new(0, 30));
    schematic.rubber_band_on_one_sheet(Point::new(40, 0));
    tidy_after_move(&mut schematic);
    assert!(schematic.end_operation());

    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(160, 130));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(120, 130));
    assert_eq!(document.net_labels[1].pos, Point::new(380, 200));
    assert_nets_kept(&schematic, &before, "a drag");
}

#[test]
fn a_move_carries_the_label_tap_and_probe_on_the_parts_pins_and_wires() {
    let mut schematic = attached_sheet();
    let before = net_names_by_terminal(schematic.design.document());
    schematic.begin_operation("move selection");
    assert_eq!(
        schematic.move_with_mode_on_one_sheet(Point::new(40, 30), MoveSelectionMode::Connected),
        Ok(true)
    );
    tidy_after_move(&mut schematic);
    assert!(schematic.end_operation());

    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(160, 130));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(120, 130));
    assert_nets_kept(&schematic, &before, "Move");
}

/// Shove lays each wire on R1's pins afresh. The output wire, which R2's
/// wire tees into, would lose the tee, so shove refuses it untouched; with
/// the tee gone the rest moves and keeps its label and tap.
#[test]
fn a_shove_move_carries_the_label_and_tap_on_the_parts_pins() {
    let mut teed = attached_sheet();
    let untouched = SchematicSnapshot::capture(teed.design.document());
    assert_eq!(
        teed.move_with_mode_on_one_sheet(Point::new(0, 30), MoveSelectionMode::Shove),
        Err(MoveSelectionError::UnsupportedInteriorConnection { wire_id: 10 })
    );
    assert!(untouched.is_equal(&SchematicSnapshot::capture(teed.design.document())));

    let mut schematic = attached_sheet();
    let document = schematic.design.document_mut_for_test();
    document.wires.retain(|wire| wire.id != 11);
    document.junctions.clear();
    let before = net_names_by_terminal(schematic.design.document());
    schematic.begin_operation("move selection");
    assert_eq!(
        schematic.move_with_mode_on_one_sheet(Point::new(0, 30), MoveSelectionMode::Shove),
        Ok(true)
    );
    tidy_after_move(&mut schematic);
    assert!(schematic.end_operation());

    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(120, 130));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(80, 130));
    assert_nets_kept(&schematic, &before, "a shove Move");
}

/// A Move that breaks connections pulls the pins out of their wires, and
/// what sits on a wire stays with the wire.
#[test]
fn a_move_that_breaks_connections_leaves_the_label_with_the_wire() {
    let mut schematic = attached_sheet();
    assert_eq!(
        schematic
            .move_with_mode_on_one_sheet(Point::new(0, 30), MoveSelectionMode::BreakConnections),
        Ok(true)
    );
    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(120, 100));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(80, 100));
}

/// Turn or flip the selection about `pivot` as the editor does, tidying
/// included.
fn transform_on_one_sheet(
    schematic: &mut SchematicState,
    transform: rspice_design::schematic::component_edit::SelectionTransform,
    pivot: Point,
) -> bool {
    schematic.with_single_sheet_topology(|state, scope| {
        state.transform_selection(transform, pivot, legacy_terminal_points, scope)
    })
}

#[test]
fn a_turn_carries_the_label_and_tap_on_the_parts_pins() {
    let mut schematic = attached_sheet();
    let before = net_names_by_terminal(schematic.design.document());
    assert!(transform_on_one_sheet(
        &mut schematic,
        rspice_design::schematic::component_edit::SelectionTransform::RotateClockwise,
        Point::new(100, 100),
    ));
    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(100, 120));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(100, 80));
    assert_nets_kept(&schematic, &before, "Rotate");
}

#[test]
fn a_flip_carries_the_label_probe_tap_and_dot_on_the_flipped_pins_and_wires() {
    let mut schematic = attached_sheet();
    let selection = &mut schematic.session.editor.selection;
    selection.select_component(2);
    selection.select_wire(10);
    selection.select_wire(11);
    let before = net_names_by_terminal(schematic.design.document());
    assert!(transform_on_one_sheet(
        &mut schematic,
        rspice_design::schematic::component_edit::SelectionTransform::MirrorVertical,
        Point::new(100, 250),
    ));
    let document = schematic.design.document();
    assert_eq!(document.net_labels[0].pos, Point::new(120, 400));
    assert_eq!(document.probes[0].position, Point::new(150, 400));
    assert_eq!(document.junctions[0].pos, Point::new(180, 400));
    assert_eq!(document.bus_taps[0].connection_point, Point::new(80, 400));
    assert_nets_kept(&schematic, &before, "Mirror");
}

/// A bus and its wire turned without the tap between them take the tap's
/// two ends with them, so its bit stays joined.
#[test]
fn a_turned_bus_and_wire_take_the_unselected_tap_along() {
    let mut schematic = bus_tap_and_scalar_target();
    schematic.session.editor.selection.select_bus(1);
    schematic.session.editor.selection.select_wire(3);
    assert!(transform_on_one_sheet(
        &mut schematic,
        rspice_design::schematic::component_edit::SelectionTransform::RotateClockwise,
        Point::new(5, 5),
    ));
    let document = schematic.design.document();
    let tap = &document.bus_taps[0];
    assert_eq!(tap.bus_point, Point::new(10, 5));
    assert_eq!(tap.connection_point, Point::new(0, 0));
    assert!(tap.validate_against_bus(&document.buses[0]).is_ok());
    assert!(document.wires[0].contains_point(tap.connection_point));
}
