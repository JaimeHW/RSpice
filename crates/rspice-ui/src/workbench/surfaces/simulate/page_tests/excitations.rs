//! The Excitations page over the whole design, not one sheet.
//!
//! Split out of `page_tests.rs` when that file crossed its line budget: the
//! family arrived as one self-contained block and reads as one.

use super::{RSpiceApp, SimulationPage, render_with};

/// Give the fixture project one more schematic master: the library cell view,
/// and the content the projection will materialize for it.
fn add_master(app: &mut RSpiceApp, cell: &str, schematic: crate::state::SchematicState) {
    use crate::state::{CellViewRef, Library, View, ViewType};

    if app.state.library_manager.get_library("work").is_none() {
        app.state.library_manager.add_library(Library::new("work"));
    }
    let owner = app
        .state
        .library_manager
        .get_library_mut("work")
        .expect("the fixture library exists");
    let target = owner.get_or_create_cell(cell);
    if target.get_view("schematic").is_none() {
        target.add_view(View::new("schematic", ViewType::Schematic));
    }
    app.state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", cell, "schematic").key(), schematic);
}

/// One component of the fixture design, under the name the deck carries.
fn seeded(
    id: u64,
    kind: crate::state::ComponentType,
    name: &str,
    params: &str,
) -> crate::state::Component {
    let mut component = crate::state::Component::new(id, kind, crate::state::Point::new(60, 60));
    component.name = name.to_owned();
    component.params = params.to_owned();
    component
}

/// A root placing one supply and two instances of a master that places a sine
/// source and an RF port.
fn hierarchical_excitations(app: &mut RSpiceApp) {
    use crate::state::{ComponentType, LibraryCellInstance, Point};

    app.state
        .schematic
        .components
        .push(seeded(801, ComponentType::VoltageSource, "VDD", "dc=5"));
    for (id, name) in [(802, "XA"), (803, "XB")] {
        let mut instance =
            crate::state::Component::new(id, ComponentType::CellInstance, Point::new(80, 80))
                .with_library_cell(LibraryCellInstance::new("work", "afe", "schematic"));
        instance.name = name.to_owned();
        app.state.schematic.components.push(instance);
    }
    app.state.sync_active_schematic_to_workspace();

    let mut child = crate::state::SchematicState::default();
    child.components.push(seeded(
        811,
        ComponentType::VoltageSourceSin,
        "V1",
        "freq=1k",
    ));
    child
        .components
        .push(seeded(812, ComponentType::RfPort, "P1", "port=1 z0=50"));
    add_master(app, "afe", child);
}

/// The page that answers what drives this circuit answers about the whole
/// design.
///
/// It read the editor's buffer, so the root of every hierarchical design was
/// told it places nothing but its own supplies — while the run drove every
/// source drawn below it, and the eyebrow counted the wrong number above a
/// table that listed none of them. The Occurrence column is what tells the two
/// rows one drawn source becomes apart.
#[test]
fn the_excitations_page_states_every_occurrence_the_run_drives_a_source_at() {
    let rendered = render_with(
        SimulationPage::Excitations,
        1200.0,
        hierarchical_excitations,
    );

    assert_eq!(
        rendered
            .lines()
            .filter(|line| line.trim() == "OCCURRENCE")
            .count(),
        2,
        "both ledger heads name the column that qualifies a reference:\n{rendered}"
    );
    for occurrence in ["/XA", "/XB"] {
        assert_eq!(
            rendered
                .lines()
                .filter(|line| line.trim() == occurrence)
                .count(),
            2,
            "{occurrence} reaches one source and one RF port, and each is a row:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("DESIGN \u{00b7} 3 SOURCES \u{00b7} 2 RF PORTS \u{00b7} 0 UNREAD"),
        "the eyebrow counts what the run drives:\n{rendered}"
    );
    assert!(
        rendered.contains("Port number 1 is claimed by more than one placed port"),
        "one port drawn once and reached twice claims one index of one matrix:\n{rendered}"
    );
}

/// The design's own root is stated as `/` rather than left blank.
///
/// An empty cell in a ledger reads as a value the page failed to find, and `/`
/// is the spelling every other surface renders the design root in.
#[test]
fn a_root_placed_excitation_states_the_root_rather_than_an_empty_cell() {
    let rendered = render_with(
        SimulationPage::Excitations,
        1200.0,
        hierarchical_excitations,
    );
    let rows: Vec<&str> = rendered.lines().collect();
    let reference = rows
        .iter()
        .position(|line| line.trim() == "VDD")
        .expect("the root supply is listed");

    assert_eq!(
        rows[reference + 2].trim(),
        "/",
        "reference, quantity, then the occurrence:\n{rendered}"
    );
}
