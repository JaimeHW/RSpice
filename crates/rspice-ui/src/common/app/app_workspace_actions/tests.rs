//! Tests for workspace-view and symbol-document actions.

use crate::common::app::AppState;
use crate::panels::{LogAnchor, LogSource};
use crate::services::drc::{DrcLocation, DrcViolationType};
use crate::shell::WorkspaceView;
use crate::state::{
    Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point,
    PortDirection, PortSpec, ResolvedSymbolSource, Rotation, SYMBOL_DOCUMENT_METADATA_KEY,
    SchematicState, SymbolDocument, SymbolPin, SymbolResolver, View, ViewType, Wire,
};

fn symbol_document(pins: &[(&str, PortDirection, Point)]) -> SymbolDocument {
    SymbolDocument {
        pins: pins
            .iter()
            .map(|(name, direction, position)| SymbolPin::new(*name, *direction, Some(*position)))
            .collect(),
        ..SymbolDocument::default()
    }
}

fn amp_binding(pins: &[(&str, PortDirection)]) -> LibraryCellInstance {
    let ports: Vec<PortSpec> = pins
        .iter()
        .map(|(name, direction)| PortSpec {
            name: (*name).to_owned(),
            direction: *direction,
        })
        .collect();
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&ports);
    binding
}

fn state_with_amp_symbol(document: SymbolDocument) -> AppState {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("initial symbol stores");
    amp.add_view(symbol_view);
    library.add_cell(amp);
    state.library_manager.add_library(library);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
}

fn state_with_amp_symbol_pin(pin_name: &str, position: Option<Point>) -> AppState {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = pin_name.to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
        .store_active_symbol_document(&SymbolDocument {
            pins: vec![SymbolPin::new(pin_name, PortDirection::In, position)],
            ..SymbolDocument::default()
        })
        .expect("symbol document stores");

    state
}

fn state_with_unplaced_amp_symbol_pins(count: usize) -> AppState {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    for index in 0..count {
        let port_id = schematic.add_component(ComponentType::Port, Point::new(index as i32, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = format!("P{index}");
    }
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
        .store_active_symbol_document(&SymbolDocument::default())
        .expect("symbol document stores");
    state
}

/// A persisted session whose active tab points into the legacy seeded
/// "primitives" library must come back with the drawn content moved to
/// the user library and the placeholder library gone — otherwise
/// `ensure_library_model` resurrects the placeholder cell every launch.
#[test]
fn legacy_primitives_content_migrates_to_user_library() {
    let mut state = AppState::default();

    // Simulate the legacy session: seeded library + an open tab with
    // a drawn schematic inside it.
    let mut legacy = Library::new("primitives");
    let mut cell = Cell::new("ISource AC");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    legacy.add_cell(cell);
    state.library_manager.add_library(legacy);

    let mut drawn = crate::state::SchematicState::default();
    drawn.add_component(ComponentType::Njfet, Point::new(0, 0));
    state
        .workspace
        .schematic_buffers
        .insert("primitives/ISource AC/schematic".to_owned(), drawn);
    state.workspace.active_view =
        crate::state::CellViewRef::new("primitives", "ISource AC", "schematic");

    state.migrate_legacy_primitives();

    assert!(
        state.library_manager.get_library("primitives").is_none(),
        "legacy library must be gone"
    );
    assert_eq!(state.workspace.active_view.library, "user");
    let migrated = state
        .workspace
        .schematic_buffers
        .get("user/ISource AC/schematic")
        .expect("buffer migrated to the user library");
    assert_eq!(migrated.components.len(), 1, "drawn content preserved");
    assert!(
        state
            .library_manager
            .get_library("user")
            .and_then(|lib| lib.get_cell("ISource AC"))
            .is_some(),
        "migrated cell exists in the user library"
    );
}

#[test]
fn leaving_symbol_view_does_not_save_stale_schematic_under_symbol_key() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    let mut top = Cell::new("top2");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    state.library_manager.add_library(library);

    let amp_schematic = CellViewRef::new("work", "amp", "schematic");
    let amp_symbol = CellViewRef::new("work", "amp", "symbol");
    let top_schematic = CellViewRef::new("work", "top2", "schematic");

    let mut amp_buffer = SchematicState::default();
    amp_buffer.add_component(ComponentType::Port, Point::new(0, 0));
    state
        .workspace
        .schematic_buffers
        .insert(amp_schematic.key(), amp_buffer.clone());
    state.schematic = amp_buffer;
    state.workspace.active_view = amp_schematic.clone();
    state.workspace.open_views = vec![crate::state::OpenCellView::new(
        amp_schematic.clone(),
        ViewType::Schematic,
    )];

    state.open_workspace_view(amp_symbol.clone());
    assert_eq!(state.workspace.active_view_type(), ViewType::Symbol);

    state.open_workspace_view(top_schematic);

    assert!(
        !state
            .workspace
            .schematic_buffers
            .contains_key(&amp_symbol.key()),
        "switching away from symbol must not persist the live schematic under the symbol key"
    );
}

#[test]
fn active_symbol_ports_read_the_paired_schematic_contract() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "OUT".to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let ports = state.active_symbol_ports();

    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0].name, "OUT");
}

#[test]
fn generating_active_symbol_document_writes_symbol_view_metadata() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    state
        .generate_active_symbol_document()
        .expect("symbol document generated");

    let view = state
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    let doc =
        crate::state::SymbolDocument::load_from_view(view).expect("stored symbol document parses");
    assert_eq!(
        doc.pin("IN").expect("IN pin exists").position,
        Some(Point::new(30, 0))
    );
    assert!(
        !view.metadata.contains_key("generated"),
        "a stored symbol document is now hand-authored state, not a generated fallback"
    );
    assert!(
        !view.metadata.contains_key("ports"),
        "stored symbol metadata must not keep stale fallback port text"
    );
}

#[test]
fn generate_symbol_document_is_one_undoable_transaction() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    state
        .generate_active_symbol_document()
        .expect("symbol document generated");

    assert!(
        state.can_undo_active_symbol_document(),
        "generation should create one undoable editor transaction"
    );
    assert!(
        state
            .undo_active_symbol_document()
            .expect("undo generated symbol"),
        "the generated document should undo in one step"
    );
    let undone = state
        .load_active_symbol_document()
        .expect("undone symbol document loads");
    assert!(
        undone.body.is_empty() && undone.pins.is_empty(),
        "undo should restore the pre-generation symbol document"
    );
    assert!(
        state
            .redo_active_symbol_document()
            .expect("redo generated symbol"),
        "the generated document should redo in one step"
    );
    assert!(
        !state
            .load_active_symbol_document()
            .expect("redone symbol document loads")
            .body
            .is_empty(),
        "redo should restore generated symbol artwork"
    );
}

#[test]
fn undo_generate_symbol_restores_generated_fallback_metadata_state() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    assert!(
        !state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists")
            .metadata
            .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "fixture starts without authored symbol metadata"
    );

    state
        .generate_active_symbol_document()
        .expect("symbol document generated");
    assert!(
        state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists")
            .metadata
            .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "generate writes authored symbol metadata"
    );

    assert!(
        state
            .undo_active_symbol_document()
            .expect("undo generated symbol"),
        "undo should apply"
    );

    let view = state
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    assert!(
        !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "undo must restore the missing authored metadata state"
    );
    let resolver = SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
    let resolved = resolver
        .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
        .expect("symbol resolves after undo");
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
}

#[test]
fn undo_first_manual_symbol_edit_restores_generated_fallback_source() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let before = state
        .load_active_symbol_document()
        .expect("pre-edit fallback document loads");
    assert!(
        !state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists")
            .metadata
            .contains_key(SYMBOL_DOCUMENT_METADATA_KEY)
    );

    state.record_symbol_edit(&before);
    state
        .store_active_symbol_document(&SymbolDocument {
            body: vec![crate::state::SymbolShape::Dot {
                center: Point::origin(),
                radius: 3,
            }],
            ..before.clone()
        })
        .expect("first manual edit stores authored metadata");

    assert!(
        state
            .undo_active_symbol_document()
            .expect("undo first manual edit")
    );
    let view = state
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    assert!(
        !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "undo must restore the missing authored metadata state"
    );
    let resolver = SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
    let resolved = resolver
        .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
        .expect("symbol resolves after undo");
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
}

#[test]
fn storing_symbol_document_remaps_open_instance_wires_by_pin_name() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
        ],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut symbol_view)
    .expect("initial symbol stores");
    amp.add_view(symbol_view);
    library.add_cell(amp);
    let mut top = Cell::new("top");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    state.library_manager.add_library(library);

    let ports = vec![
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&ports);

    let mut parent = SchematicState::default();
    parent.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    parent
        .wires
        .push(Wire::segment(7, Point::new(60, 50), Point::new(0, 50)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
        .store_active_symbol_document(&SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-20, 10))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            ],
            ..SymbolDocument::default()
        })
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.wires[0].points[0], Point::new(80, 60));
    assert_eq!(parent.wires[0].points[1], Point::new(0, 50));
}

#[test]
fn storing_symbol_document_remaps_rotated_and_mirrored_instance_wires() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-30, 10)),
        ("OUT", PortDirection::Out, Point::new(40, 0)),
    ]));

    let mut parent = SchematicState::default();
    parent.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_rotation(Rotation::R90)
            .with_mirror_h(true)
            .with_library_cell(amp_binding(&[
                ("IN", PortDirection::In),
                ("OUT", PortDirection::Out),
            ])),
    );
    parent
        .wires
        .push(Wire::segment(8, Point::new(90, 80), Point::new(90, 120)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(-10, -20)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]))
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.wires[0].points[0], Point::new(120, 60));
    assert_eq!(parent.wires[0].points[1], Point::new(90, 120));
}

#[test]
fn storing_symbol_document_applies_wire_remaps_once() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(0, 0)),
        ("OUT", PortDirection::Out, Point::new(10, 0)),
    ]));

    let mut parent = SchematicState::default();
    parent.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50)).with_library_cell(
            amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
        ),
    );
    parent
        .wires
        .push(Wire::segment(9, Point::new(100, 50), Point::new(100, 0)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(10, 0)),
            ("OUT", PortDirection::Out, Point::new(20, 0)),
        ]))
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(
        parent.wires[0].points[0],
        Point::new(110, 50),
        "the IN endpoint must stop at IN's new location, not then match OUT's old location"
    );
}

#[test]
fn storing_symbol_document_remaps_all_open_parent_buffers() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-40, 0)),
        ("OUT", PortDirection::Out, Point::new(40, 0)),
    ]));
    let binding = amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]);

    let mut top = SchematicState::default();
    top.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding.clone()),
    );
    top.components.push(
        Component::new(2, ComponentType::CellInstance, Point::new(200, 0))
            .with_library_cell(binding.clone()),
    );
    top.wires
        .push(Wire::segment(10, Point::new(60, 50), Point::new(0, 50)));
    top.wires
        .push(Wire::segment(11, Point::new(160, 0), Point::new(160, -50)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "top", "schematic").key(), top);

    let mut tb = SchematicState::default();
    tb.components.push(
        Component::new(3, ComponentType::CellInstance, Point::new(-10, 20))
            .with_library_cell(binding),
    );
    tb.wires
        .push(Wire::segment(12, Point::new(-50, 20), Point::new(-90, 20)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "tb", "schematic").key(), tb);

    state
        .store_active_symbol_document(&symbol_document(&[
            ("IN", PortDirection::In, Point::new(-20, 10)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]))
        .expect("updated symbol stores");

    let top = state
        .workspace
        .schematic_buffers
        .get("work/top/schematic")
        .expect("top schematic remains open");
    assert_eq!(top.wires[0].points[0], Point::new(80, 60));
    assert_eq!(top.wires[1].points[0], Point::new(180, 10));

    let tb = state
        .workspace
        .schematic_buffers
        .get("work/tb/schematic")
        .expect("testbench schematic remains open");
    assert_eq!(tb.wires[0].points[0], Point::new(-30, 30));
}

#[test]
fn symbol_pin_checks_store_structured_drc_results_with_symbol_anchors() {
    let mut state = state_with_amp_symbol_pin("IN", None);

    state.run_active_symbol_pin_checks();

    let result = state.dialogs.drc_results.as_ref().expect("result stored");
    assert!(result.violations().iter().any(|violation| {
        violation.violation_type == DrcViolationType::SymbolUnplacedPin
            && matches!(
                &violation.location,
                DrcLocation::SymbolPin { pin_name, .. } if pin_name == "IN"
            )
    }));
    assert!(state.log_buffer.entries().any(|entry| {
        matches!(
            &entry.anchor,
            Some(LogAnchor::Symbol { pin_name, .. }) if pin_name == "IN"
        )
    }));
}

#[test]
fn symbol_pin_checks_cap_console_rows_and_keep_all_results() {
    let mut state = state_with_unplaced_amp_symbol_pins(55);

    state.run_active_symbol_pin_checks();

    let result = state.dialogs.drc_results.as_ref().expect("result stored");
    assert_eq!(result.violations().len(), 55);

    let drc_rows: Vec<_> = state
        .log_buffer
        .entries()
        .filter(|entry| entry.source == LogSource::Drc)
        .collect();
    let anchored_symbol_rows = drc_rows
        .iter()
        .filter(|entry| matches!(entry.anchor, Some(LogAnchor::Symbol { .. })))
        .count();
    assert_eq!(anchored_symbol_rows, 50);
    assert!(
        drc_rows
            .iter()
            .any(|entry| entry.message.contains("+5 more findings"))
    );
}

#[test]
fn symbol_log_anchor_opens_symbol_view_and_selects_pin() {
    let mut state = state_with_amp_symbol_pin("IN", Some(Point::new(-30, 0)));
    let reference = CellViewRef::new("work", "amp", "symbol");

    state.jump_to_log_anchor(LogAnchor::Symbol {
        reference: reference.clone(),
        pin_name: "IN".to_owned(),
        point: Some(Point::new(-30, 0)),
    });

    assert_eq!(state.workspace.active_view, reference);
    assert_eq!(state.shell.symbol.selected_pin.as_deref(), Some("IN"));
    assert_eq!(state.shell.view, WorkspaceView::Schematic);
}

#[test]
fn symbol_violation_cycle_opens_symbol_view_and_selects_pin() {
    let mut state = state_with_amp_symbol_pin("IN", None);
    state.run_active_symbol_pin_checks();
    state.shell.symbol.clear_selection();

    crate::schematic::view::violations::cycle_violation(&mut state, 1);

    assert_eq!(
        state.workspace.active_view,
        CellViewRef::new("work", "amp", "symbol")
    );
    assert_eq!(state.shell.symbol.selected_pin.as_deref(), Some("IN"));
}

#[test]
fn symbol_undo_history_is_scoped_to_active_view() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    for cell_name in ["amp_a", "amp_b"] {
        let mut cell = Cell::new(cell_name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
    }
    state.library_manager.add_library(library);

    state.open_workspace_view(CellViewRef::new("work", "amp_a", "symbol"));
    let before_a = SymbolDocument {
        name_anchor: Point::new(-10, -10),
        ..SymbolDocument::default()
    };
    let after_a = SymbolDocument {
        name_anchor: Point::new(-20, -20),
        ..SymbolDocument::default()
    };
    state.record_symbol_edit(&before_a);
    state
        .store_active_symbol_document(&after_a)
        .expect("store amp_a symbol");

    state.open_workspace_view(CellViewRef::new("work", "amp_b", "symbol"));
    let before_b = SymbolDocument {
        name_anchor: Point::new(10, 10),
        ..SymbolDocument::default()
    };
    state
        .store_active_symbol_document(&before_b)
        .expect("store amp_b symbol");

    assert!(
        !state
            .undo_active_symbol_document()
            .expect("undo amp_b symbol"),
        "undo from amp_a must not apply to amp_b"
    );
    assert_eq!(
        state
            .load_active_symbol_document()
            .expect("amp_b symbol loads")
            .name_anchor,
        Point::new(10, 10)
    );

    state.open_workspace_view(CellViewRef::new("work", "amp_a", "symbol"));
    assert!(
        state
            .undo_active_symbol_document()
            .expect("undo amp_a symbol"),
        "amp_a keeps its own undo stack"
    );
    assert_eq!(
        state
            .load_active_symbol_document()
            .expect("amp_a symbol loads")
            .name_anchor,
        Point::new(-10, -10)
    );
}

#[test]
fn symbol_store_refuses_read_only_libraries() {
    let mut state = AppState::default();

    let mut library = Library::new("readonly");
    library.read_only = true;
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

    let error = state
        .store_active_symbol_document(&SymbolDocument::default())
        .expect_err("read-only symbol store is rejected");

    assert_eq!(error, "Read-only - 'readonly' masters cannot be edited");
}

#[test]
fn read_only_symbol_edit_paths_use_consistent_refusal_text() {
    let mut state = AppState::default();

    let mut library = Library::new("readonly");
    library.read_only = true;
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

    let expected = "Read-only - 'readonly' masters cannot be edited";

    assert_eq!(
        state
            .store_active_symbol_document(&SymbolDocument::default())
            .expect_err("store should be refused"),
        expected
    );
    assert_eq!(
        state
            .generate_active_symbol_document()
            .expect_err("generate should be refused"),
        expected
    );
    assert_eq!(
        state
            .undo_active_symbol_document()
            .expect_err("undo should be refused"),
        expected
    );
    assert_eq!(
        state
            .redo_active_symbol_document()
            .expect_err("redo should be refused"),
        expected
    );

    assert!(state.deny_read_only_edit());
    let warning = state
        .log_buffer
        .entries()
        .last()
        .expect("read-only warning is logged");
    assert_eq!(warning.message, expected);
}

#[test]
fn opening_symbol_view_loads_the_paired_schematic_context() {
    let mut state = AppState::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut paired = SchematicState::default();
    let port_id = paired.add_component(ComponentType::Port, Point::new(0, 0));
    paired
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "PAIR".to_owned();
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "amp", "schematic").key(), paired);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let ports = state.schematic.interface_ports();
    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0].name, "PAIR");
}

#[test]
fn symbol_dirty_state_routes_ordinary_save_to_project() {
    let mut state = AppState::default();
    let reference = CellViewRef::new("user", "top", "symbol");
    state.workspace.open_view(reference, ViewType::Symbol);
    state.workspace.set_active_dirty(true);

    assert!(state.should_save_project_for_active_document());
}
