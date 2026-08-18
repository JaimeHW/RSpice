//! Cases for the symbol document's edit, save and reconciliation
//! transactions.
//!
//! The claims here are about what a commit carries beyond geometry: whether
//! the editor knows it has unpublished work, whether a save its own checks
//! refuse can still happen, and whether a pin rename reaches the instances
//! that are wired to it.

use super::*;

use crate::state::{
    Cell, Component, ComponentType, Library, LibraryCellInstance, Point, PortDirection,
    SchematicState, SymbolPin, SymbolShape, View, ViewType, Wire, WireConnection,
};

const PARENT: &str = "work/top/schematic";

fn interface() -> Vec<PortSpec> {
    vec![
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ]
}

/// A cell whose schematic declares `IN` and `OUT`, with its symbol view open.
///
/// `parent_instances` are placed in a sibling sheet's stored buffer and one
/// more on the live sheet, so a transaction claiming to reach "every buffer
/// and the live sheet" is held to both.
fn state_with_open_symbol(parent_instances: usize) -> AppState {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    let mut top = Cell::new("top");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    state.library_manager.add_library(library);

    let mut declaring = SchematicState::default();
    for (index, name) in ["IN", "OUT"].into_iter().enumerate() {
        let port = declaring.add_component(ComponentType::Port, Point::new(0, index as i32 * 20));
        declaring
            .components
            .iter_mut()
            .find(|component| component.id == port)
            .expect("port exists")
            .value = name.to_owned();
    }
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        declaring,
    );

    let mut parent = SchematicState::default();
    for index in 0..parent_instances {
        place_instance(&mut parent, 900 + index as u64, index as i32 * 100);
    }
    state
        .workspace
        .schematic_buffers
        .insert(PARENT.to_owned(), parent);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    // The live sheet is replaced when the view opens, so its instance is
    // placed afterwards.
    place_instance(&mut state.schematic, 800, 0);
    state
}

fn place_instance(schematic: &mut SchematicState, id: u64, x: i32) {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&interface());
    schematic.components.push(
        Component::new(id, ComponentType::CellInstance, Point::new(x, 0))
            .with_library_cell(binding),
    );
    schematic.wires.push(Wire::segment(
        id + 50,
        Point::new(x, 0),
        Point::new(x + 10, 0),
    ));
    schematic.connections.push(WireConnection {
        wire_id: id + 50,
        point_index: 0,
        component_id: id,
        terminal_name: "OUT".to_owned(),
    });
}

fn terminal_names(state: &AppState) -> Vec<String> {
    let mut names: Vec<String> = state
        .workspace
        .schematic_buffers
        .get(PARENT)
        .expect("the parent buffer exists")
        .connections
        .iter()
        .map(|connection| connection.terminal_name.clone())
        .collect();
    names.extend(
        state
            .schematic
            .connections
            .iter()
            .map(|connection| connection.terminal_name.clone()),
    );
    names
}

/// The interface's own generated symbol: pin order, electrical types and
/// terminal placement all agree with the contract by construction.
fn contract_matching_document(state: &AppState) -> SymbolDocument {
    SymbolDocument::generated_from_ports(&state.active_symbol_ports())
}

fn pin_index(document: &SymbolDocument, name: &str) -> usize {
    document
        .pins
        .iter()
        .position(|pin| pin.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} is declared by the interface"))
}

#[test]
fn the_save_point_tracks_publication_not_the_stored_view() {
    let mut state = state_with_open_symbol(0);
    let key = state.workspace.active_key();
    let document = contract_matching_document(&state);
    let mut metadata = SymbolEditorMetadata::for_document(&document);

    assert!(
        !state.ui.symbol.is_dirty(&key),
        "an untouched symbol is not dirty"
    );

    let before = state.load_active_symbol_document().expect("document loads");
    state.record_symbol_edit(&before);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    assert!(
        state.ui.symbol.is_dirty(&key),
        "an edit that was never published is unpublished work, however \
         faithfully the project library was written"
    );

    state
        .publish_active_symbol_revision(&document, &mut metadata, "first cut")
        .expect("a contract-matching symbol publishes");

    assert_eq!(metadata.revision, 1);
    assert!(
        !state.ui.symbol.is_dirty(&key),
        "publication is the save point"
    );

    let published = state.load_active_symbol_document().expect("document loads");
    state.record_symbol_edit(&published);
    let mut edited = document.clone();
    edited.body.push(SymbolShape::Circle {
        center: Point::new(30, 30),
        radius: 5,
    });
    state
        .store_active_symbol_editor_bundle(&edited, &metadata)
        .expect("second edit stores");

    assert!(state.ui.symbol.is_dirty(&key));

    state
        .undo_active_symbol_document()
        .expect("undo runs on a writable view");

    assert!(
        !state.ui.symbol.is_dirty(&key),
        "undoing back to the save point is as clean as never having edited"
    );
}

#[test]
fn a_symbol_that_fails_its_own_checks_is_refused_by_name() {
    let mut state = state_with_open_symbol(0);
    let mut document = contract_matching_document(&state);
    let out = pin_index(&document, "OUT");
    document.pins[out].position = Some(Point::new(43, 0));
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    let refusal = state
        .refuse_unsavable_active_symbol()
        .expect("an off-grid terminal blocks publication");
    assert!(
        refusal.contains("pin placement") || refusal.contains("off-grid terminals"),
        "the refusal names the row that blocked it: {refusal}"
    );

    let error = state
        .publish_active_symbol_revision(&document, &mut metadata, "note")
        .expect_err("publication is refused");
    assert_eq!(error, refusal);
    assert_eq!(
        metadata.revision, 0,
        "a refused publication must not consume a revision number"
    );
}

#[test]
fn safe_mode_locks_the_symbol_editor_and_the_banner_names_it() {
    let mut state = state_with_open_symbol(0);
    assert!(!state.active_view_read_only());
    assert!(state.symbol_editor_copy_available());

    state.workbench.safe_mode.active = true;
    state.workbench.safe_mode.applied.open_project_read_only = true;

    assert!(
        state.active_view_read_only(),
        "safe mode opened the project read-only, so its symbol views are too"
    );
    assert!(state.schematic_edit_read_only());
    let message = state.symbol_editor_lock_message();
    assert!(
        message.contains("Safe mode"),
        "the banner names the owner of the lock: {message}"
    );
    assert!(
        !state.symbol_editor_copy_available(),
        "copying the cell is a project write, which safe mode refuses too"
    );

    let document = contract_matching_document(&state);
    let metadata = SymbolEditorMetadata::for_document(&document);
    assert!(
        state
            .store_active_symbol_editor_bundle(&document, &metadata)
            .is_err()
    );
    assert!(state.update_active_symbol_pins_from_contract().is_err());
}

#[test]
fn updating_pins_from_the_contract_adds_only_what_is_missing() {
    let mut state = state_with_open_symbol(0);
    let mut document = contract_matching_document(&state);
    let kept = document.pins[pin_index(&document, "OUT")].clone();
    document.pins.remove(pin_index(&document, "IN"));
    document.pins.push(SymbolPin::new(
        "STALE",
        PortDirection::InOut,
        Some(Point::new(0, 40)),
    ));
    let art = document.body.clone();
    let metadata = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    let summary = state
        .update_active_symbol_pins_from_contract()
        .expect("the contract update runs");
    assert!(summary.contains("1 added"), "{summary}");
    assert!(summary.contains("1 left orphaned"), "{summary}");

    let updated = state.load_active_symbol_document().expect("document loads");
    assert_eq!(updated.body, art, "body artwork is never regenerated");
    let names: Vec<&str> = updated.pins.iter().map(|pin| pin.name.as_str()).collect();
    assert!(names.contains(&"IN"), "{names:?}");
    assert!(
        names.contains(&"STALE"),
        "an orphan is kept so the symbol check can report it: {names:?}"
    );
    let added = updated.pin("IN").expect("the missing pin was created");
    assert!(added.position.is_some(), "a created pin is placed");
    assert_eq!(
        updated.pin("OUT").expect("OUT survives").position,
        kept.position,
        "an existing pin keeps its authored geometry"
    );
    assert!(
        names != ["IN", "OUT"],
        "order is only rewritten when the name sets agree exactly: {names:?}"
    );
}

#[test]
fn the_contract_update_reorders_only_on_an_exact_name_match() {
    let mut state = state_with_open_symbol(0);
    let declared: Vec<String> = state
        .active_symbol_ports()
        .into_iter()
        .map(|port| port.name)
        .collect();
    let mut document = contract_matching_document(&state);
    document.pins.reverse();
    let metadata = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    state
        .update_active_symbol_pins_from_contract()
        .expect("the contract update runs");

    let updated = state.load_active_symbol_document().expect("document loads");
    let names: Vec<String> = updated.pins.iter().map(|pin| pin.name.clone()).collect();
    assert_eq!(names, declared, "netlist order follows the interface");
}

#[test]
fn a_pin_rename_reaches_every_placed_instance() {
    let mut state = state_with_open_symbol(2);
    let document = contract_matching_document(&state);
    let metadata = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    let mut renamed = document.clone();
    let out = pin_index(&renamed, "OUT");
    renamed.pins[out].name = "VOUT".to_owned();
    let before = state.load_active_symbol_document().expect("document loads");
    state.record_symbol_edit(&before);
    state
        .commit_active_symbol_edit(
            &renamed,
            &metadata,
            &SymbolCommitIntent {
                renames: BTreeMap::from([("OUT".to_owned(), "VOUT".to_owned())]),
            },
        )
        .expect("the rename commits");

    assert_eq!(
        terminal_names(&state),
        ["VOUT", "VOUT", "VOUT"],
        "every placed instance follows the pin, in stored buffers and on the \
         live sheet alike"
    );
    for component in &state.schematic.components {
        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        assert!(
            binding.terminal_order.contains(&"VOUT".to_owned()),
            "the binding's netlist order follows too: {:?}",
            binding.terminal_order
        );
    }

    state
        .undo_active_symbol_document()
        .expect("undo runs on a writable view");

    assert_eq!(
        terminal_names(&state),
        ["OUT", "OUT", "OUT"],
        "undoing a rename puts the instance terminals back rather than \
         leaving them attached to a name the symbol no longer declares"
    );
}

/// A pin that only moved is a move. Inferring a rename from geometry is what
/// made a pin renamed in place invisible and a moved pin a false rename.
#[test]
fn a_moved_pin_is_not_a_rename() {
    let mut state = state_with_open_symbol(1);
    let document = contract_matching_document(&state);
    let metadata = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &metadata)
        .expect("edit stores");

    let mut moved = document.clone();
    let out = pin_index(&moved, "OUT");
    moved.pins[out].position = Some(Point::new(60, 0));
    state
        .store_active_symbol_editor_bundle(&moved, &metadata)
        .expect("the move commits");

    assert_eq!(
        terminal_names(&state),
        ["OUT", "OUT"],
        "a move leaves the terminal name alone"
    );
}

/// Every seeded symbol comes from one constructor, so a cell created by the
/// hierarchy transaction and one created by the new-view dialog cannot draw
/// different bodies for the same interface.
#[test]
fn generated_symbols_come_from_one_constructor() {
    let ports = interface();

    let direct = SymbolDocument::generated_from_ports(&ports);
    let mut view = View::new("symbol", ViewType::Symbol);
    direct.store_in_view(&mut view).expect("stores");
    let round_tripped = SymbolDocument::load_from_view(&view).expect("loads");

    assert_eq!(direct, round_tripped);
    let source = include_str!("../symbol_document.rs");
    assert!(
        !source.contains("SymbolShape::Polyline {"),
        "symbol seeding belongs to state::symbol::generated_from_ports; this \
         module must not grow a second body generator"
    );
}
