//! What the generator's view of the design has to answer correctly.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, LibraryManager, Point, ProjectWorkspace, View, ViewType, Wire,
};

fn place_port(state: &mut SchematicState, name: &str, pos: Point) {
    let id = state.add_component(ComponentType::Port, pos);
    let component = state
        .components
        .iter_mut()
        .find(|c| c.id == id)
        .expect("port exists");
    component.value = name.to_string();
}

fn binding(cell: &str, terminals: &[&str]) -> LibraryCellInstance {
    let mut binding = LibraryCellInstance::new("work", cell, "schematic");
    binding.terminal_order = terminals.iter().map(|t| t.to_string()).collect();
    binding
}

/// `div`: resistor between ports a and b. Port terminals coincide with the
/// resistor terminals, so connectivity needs no wires.
fn div_master() -> SchematicState {
    let mut master = SchematicState::default();
    place_port(&mut master, "a", Point::new(20, 0));
    master.add_component(ComponentType::Resistor, Point::new(30, 0));
    place_port(&mut master, "b", Point::new(60, 0));
    master
}

#[test]
fn projection_nets_extract_the_root_once_and_stay_with_the_projection() {
    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);

    let mut workspace = ProjectWorkspace::default();
    workspace.schematic_buffers.insert(
        CellViewRef::new("work", "div", "schematic").key(),
        div_master(),
    );
    let mut top = SchematicState::default();
    top.add_library_cell_component(Point::new(100, 0), binding("div", &["a", "b"]));
    top.add_component(ComponentType::VoltageSource, Point::new(40, 40));
    top.add_component(ComponentType::Ground, Point::new(130, 20));
    top.add_component(ComponentType::Ground, Point::new(40, 70));
    top.wires.push(Wire::new(
        1,
        vec![Point::new(40, 20), Point::new(40, 0), Point::new(70, 0)],
    ));
    top.wires
        .push(Wire::new(2, vec![Point::new(130, 0), Point::new(130, 10)]));
    let active = workspace.active_view.clone();
    workspace
        .schematic_buffers
        .insert(active.key(), top.clone());

    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &top)
        .expect("a legacy workspace projects without a configuration");
    let root_key = projection.root().key();

    let nets = projection_nets(&libraries, &projection, &root_key);
    assert!(!nets.is_empty(), "the fixture root resolves nets");
    assert!(
        std::sync::Arc::ptr_eq(&nets, &projection_nets(&libraries, &projection, &root_key)),
        "a second request must reuse the projection's retained extraction"
    );
    assert!(
        projection_nets(&libraries, &projection, "user/absent/schematic").is_empty(),
        "a cell view the projection does not carry has no nets"
    );
}

#[test]
fn a_master_is_looked_up_by_its_exact_view() {
    let master = div_master();
    let mut buffers = HashMap::new();
    buffers.insert(
        CellViewRef::new("work", "div", "schematic").key(),
        master.clone(),
    );
    let hierarchy = HierarchySource::from_buffers(&buffers);

    let placed = LibraryCellInstance::new("work", "div", "schematic");
    assert!(hierarchy.schematic_master_for_binding(&placed).is_some());
    let other_view = LibraryCellInstance::new("work", "div", "extracted");
    assert!(
        hierarchy
            .schematic_master_for_binding(&other_view)
            .is_none(),
        "two views of one cell are two masters, and only one of them is indexed"
    );
}
