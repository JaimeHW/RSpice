//! Tests for the stale-interface repair.
//!
//! Each case is about what the repair may and may not decide on the author's
//! behalf: it re-binds the interface, it carries wiring across for every pin
//! the master still names, and it refuses to invent a destination for a pin
//! the master dropped.

use std::collections::HashMap;

use crate::state::{
    Cell, ComponentType, Library, LibraryCellInstance, LibraryManager, Point,
    SchematicReplacementError, SchematicState, SymbolResolver, View, ViewType, Wire,
    WireConnection,
};

struct Fixture {
    top: SchematicState,
    libraries: LibraryManager,
    masters: HashMap<String, SchematicState>,
}

const MASTER_KEY: &str = "work/amp/schematic";

fn amp_master(ports: &[&str]) -> SchematicState {
    let mut master = SchematicState::default();
    for (index, name) in ports.iter().enumerate() {
        let position = Point::new(i32::try_from(index).unwrap_or(0) * 40, 0);
        let id = master.add_component(ComponentType::Port, position);
        master
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed port is retained")
            .value = (*name).to_owned();
    }
    master
}

fn work_library() -> LibraryManager {
    let mut libraries = LibraryManager::new();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    libraries.add_library(library);
    libraries
}

fn placed_instance(ports: &[&str]) -> Fixture {
    let master = amp_master(ports);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&master.interface_ports());
    let mut masters = HashMap::new();
    masters.insert(MASTER_KEY.to_owned(), master);

    let mut top = SchematicState::default();
    let id = top.add_library_cell_component(Point::new(200, 200), binding);
    top.components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed instance is retained")
        .name = "X1".to_owned();
    top.selection.select_only_component(id);
    top.recalculate_runtime_state();
    top.clear_undo_history();
    Fixture {
        top,
        libraries: work_library(),
        masters,
    }
}

fn terminal_point(fixture: &Fixture, terminal: &str) -> Point {
    let component = &fixture.top.components[0];
    let binding = component
        .library_cell
        .as_ref()
        .expect("the fixture places a bound cell instance");
    let symbol = SymbolResolver::new(&fixture.libraries, &fixture.masters).resolve_binding(binding);
    component
        .terminal_positions_resolved(symbol.as_ref())
        .into_iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(terminal))
        .unwrap_or_else(|| panic!("the instance presents terminal '{terminal}'"))
        .1
}

/// Draw one wire from a terminal straight out of the body, so the segment
/// cannot pass through a sibling pin and make it look connected too.
fn wire_terminal(fixture: &mut Fixture, terminal: &str) {
    let point = terminal_point(fixture, terminal);
    let centre = fixture.top.components[0].pos;
    let away = if point.x == centre.x {
        Point::new(point.x, point.y + if point.y < centre.y { -60 } else { 60 })
    } else {
        Point::new(point.x + if point.x < centre.x { -60 } else { 60 }, point.y)
    };
    let component_id = fixture.top.components[0].id;
    let wire_id = fixture.top.next_id();
    fixture.top.wires.push(Wire::segment(wire_id, point, away));
    fixture
        .top
        .connections
        .push(WireConnection::new(wire_id, 0, component_id, terminal));
    fixture.top.recalculate_runtime_state();
    fixture.top.clear_undo_history();
}

fn rename_master_port(fixture: &mut Fixture, from: &str, to: &str) {
    fixture
        .masters
        .get_mut(MASTER_KEY)
        .expect("the fixture registers the master")
        .components
        .iter_mut()
        .find(|component| component.value == from)
        .unwrap_or_else(|| panic!("the master declares port '{from}'"))
        .value = to.to_owned();
}

fn placed_interface(fixture: &Fixture) -> Vec<String> {
    fixture.top.components[0]
        .library_cell
        .as_ref()
        .expect("the instance stays bound")
        .terminal_order
        .clone()
}

#[test]
fn a_renamed_master_port_is_repaired_in_one_undo_step() {
    let mut fixture = placed_instance(&["IN", "OUT"]);
    wire_terminal(&mut fixture, "OUT");
    rename_master_port(&mut fixture, "IN", "INP");

    assert!(
        fixture
            .top
            .selected_instance_interface_is_stale(&fixture.masters)
    );
    let summary = fixture
        .top
        .update_selected_instance_interface(&fixture.libraries, &fixture.masters)
        .expect("the repair applies");

    assert_eq!(
        summary,
        "X1 now presents the current work/amp interface (INP, OUT)"
    );
    assert_eq!(placed_interface(&fixture), ["INP", "OUT"]);
    assert!(
        !fixture
            .top
            .selected_instance_interface_is_stale(&fixture.masters)
    );
    assert_eq!(
        fixture
            .top
            .connections
            .iter()
            .map(|connection| connection.terminal_name.clone())
            .collect::<Vec<_>>(),
        ["OUT"],
        "the wiring of a surviving pin is carried across"
    );
    assert_eq!(
        fixture.top.undo_description(),
        Some("update instance interface")
    );
    assert!(fixture.top.undo());
    assert_eq!(placed_interface(&fixture), ["IN", "OUT"]);
    assert!(
        !fixture.top.can_undo(),
        "the repair records exactly one undo entry"
    );
}

#[test]
fn an_unmapped_but_connected_terminal_is_reported_by_name_and_left_disconnected() {
    let mut fixture = placed_instance(&["IN", "OUT"]);
    wire_terminal(&mut fixture, "IN");
    let drawn = fixture.top.wires[0].points.clone();
    rename_master_port(&mut fixture, "IN", "INP");

    let summary = fixture
        .top
        .update_selected_instance_interface(&fixture.libraries, &fixture.masters)
        .expect("the repair applies");

    assert_eq!(
        summary,
        "X1 now presents the current work/amp interface (INP, OUT); the master no longer names \
         IN, so that wiring was left in place and disconnected",
        "the dropped pin is named"
    );
    assert_eq!(
        fixture.top.wires[0].points, drawn,
        "the wire keeps the shape the author drew"
    );
    assert!(
        fixture.top.connections.is_empty(),
        "the dropped pin's connection is not guessed onto another terminal: {:?}",
        fixture.top.connections
    );
}

#[test]
fn an_instance_that_still_matches_its_master_is_offered_no_repair() {
    let mut fixture = placed_instance(&["IN", "OUT"]);

    assert!(
        !fixture
            .top
            .selected_instance_interface_is_stale(&fixture.masters)
    );
    assert_eq!(
        fixture
            .top
            .update_selected_instance_interface(&fixture.libraries, &fixture.masters),
        Err(SchematicReplacementError::NoChanges)
    );
}
