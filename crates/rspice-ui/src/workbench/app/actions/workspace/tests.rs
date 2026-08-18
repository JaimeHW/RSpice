//! Tests for workspace-level edit actions.
//!
//! These pin what each action does to the whole active document, and - just as
//! importantly - what a read-only document refuses to let it do.

mod physical_layouts;

use crate::diagnostics::{LogAnchor, LogSource};
use crate::services::drc::{DrcLocation, DrcViolationType};
use crate::state::{
    Cell, CellViewRef, Component, ComponentType, LayoutEdit, LayoutInstance, LayoutObjectId,
    LayoutOrientation, LayoutPoint, LayoutTransform, Library, LibraryCellInstance, Point,
    PortDirection, PortSpec, ResolvedSymbolSource, Rotation, SYMBOL_DOCUMENT_METADATA_KEY,
    SchematicState, SymbolDocument, SymbolEditorMetadata, SymbolPin, SymbolResolver, View,
    ViewType, Wire,
};
use crate::workbench::ChoicePreference;
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

#[test]
fn generated_symbol_sync_advances_the_catalog_only_for_real_changes() {
    let mut state = AppState::default();
    let empty_revision = state.library_manager.revision();

    state.sync_active_schematic_to_workspace();

    assert_eq!(
        state.library_manager.revision(),
        empty_revision,
        "an empty generated-symbol synchronization must be revision-stable"
    );

    let port_id = state
        .schematic
        .add_component(ComponentType::Port, Point::origin());
    state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("new interface port")
        .value = "IN".to_owned();

    state.sync_active_schematic_to_workspace();
    let generated_revision = state.library_manager.revision();
    assert!(
        generated_revision > empty_revision,
        "creating a generated symbol must advance the catalog revision"
    );

    state.sync_active_schematic_to_workspace();
    assert_eq!(
        state.library_manager.revision(),
        generated_revision,
        "repeating an identical generated-symbol synchronization must be a no-op"
    );
}

#[test]
fn new_document_defaults_are_resolved_once_and_existing_buffers_are_preserved() {
    let mut state = AppState::default();
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::PropertyCommitPolicy, 1)
        .unwrap();
    let created = state.new_schematic_document();
    assert_eq!(
        created.document_policy.property_commit,
        crate::state::PropertyCommitPolicy::ApplyValidFields
    );

    let reference = CellViewRef::new("work", "preserved", "schematic");
    let mut library = Library::new("work");
    let mut cell = Cell::new("preserved");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    let mut existing = SchematicState::default();
    existing.document_policy.property_commit = crate::state::PropertyCommitPolicy::Atomic;
    state
        .workspace
        .schematic_buffers
        .insert(reference.key(), existing);

    state.open_workspace_view(reference);

    assert_eq!(
        state.schematic.document_policy.property_commit,
        crate::state::PropertyCommitPolicy::Atomic
    );
}

#[test]
fn new_document_preserves_the_exact_runtime_wire_and_bus_routing_mode() {
    let mut state = AppState::default();
    state.ui.schematic_routing_mode = crate::state::WireRoutingMode::VerticalFirst;

    let created = state.new_schematic_document();

    assert_eq!(
        created.wire_drawing.routing_mode,
        crate::state::WireRoutingMode::VerticalFirst
    );
    assert_eq!(
        created.bus_drawing.routing_mode,
        crate::state::WireRoutingMode::VerticalFirst
    );
}

#[test]
fn new_document_inherits_snap_targets_and_reconciles_the_preferred_pitch() {
    let mut state = AppState::default();
    state.ui.schematic_snap.snap_radius = 9;
    state.ui.schematic_snap.snap_to_grid = false;
    state.ui.schematic_snap.snap_to_wire_segments = false;
    state.ui.schematic_snap.grid_size = 777;
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::SchematicGrid, 1)
        .unwrap();
    let mut expected_snap = state.ui.schematic_snap.clone();
    expected_snap.grid_size = crate::state::SchematicGridPitch::Mil25.canvas_grid_size();

    let created = state.new_schematic_document();

    assert_eq!(
        created.document_policy.grid_pitch,
        crate::state::SchematicGridPitch::Mil25
    );
    assert_eq!(created.grid_size, expected_snap.grid_size);
    assert_eq!(created.snap_engine, expected_snap);
}

#[test]
fn schematic_edit_authority_includes_late_safe_mode_project_read_only() {
    let mut state = AppState::default();
    assert!(!state.schematic_edit_read_only());

    state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    assert!(state.schematic_edit_read_only());
    assert!(state.deny_read_only_edit());
}

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

fn state_with_work_cell(cell_name: &str) -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new(cell_name);
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("work", cell_name, "schematic"));
    state
}

#[test]
fn production_copy_uses_authored_symbol_terminal_geometry() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-50, 0)),
        ("OUT", PortDirection::Out, Point::new(50, 0)),
    ]));
    state.schematic = SchematicState::default();
    let left = Component::new(1, ComponentType::CellInstance, Point::new(0, 0)).with_library_cell(
        amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
    );
    let right =
        Component::new(2, ComponentType::CellInstance, Point::new(200, 0)).with_library_cell(
            amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
        );
    state.schematic.components = vec![left, right];
    state
        .schematic
        .wires
        .push(Wire::segment(3, Point::new(50, 0), Point::new(150, 0)));
    state.schematic.selection.select_component(1);
    state.schematic.selection.select_component(2);

    assert!(state.copy_active_schematic_selection());
    assert_eq!(state.schematic.clipboard.components.len(), 2);
    assert_eq!(
        state.schematic.clipboard.wires.len(),
        1,
        "the conductor between authored pins must travel with both selected instances"
    );
}

fn add_cell_veriloga_source(state: &mut AppState, cell: &str) -> crate::state::ProjectSourceId {
    state
        .library_manager
        .get_library_mut("work")
        .and_then(|library| library.get_cell_mut(cell))
        .expect("fixture cell")
        .add_view(View::new("behavior", ViewType::VerilogA));
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(CellViewRef::new("work", cell, "behavior")),
        crate::state::ProjectSourceLanguage::VerilogA,
        "behavior.va",
        "module behavior(p, n); inout p, n; endmodule",
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid source bundle");
    let id = bundle.id();
    state
        .workspace
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    id
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
fn storing_symbol_document_remaps_instance_wires_when_origin_moves() {
    let before = SymbolDocument {
        origin: Point::new(20, 0),
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(30, 0)),
        )],
        ..SymbolDocument::default()
    };
    let mut state = state_with_amp_symbol(before);

    let mut parent = SchematicState::default();
    parent.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(amp_binding(&[("IN", PortDirection::In)])),
    );
    parent
        .wires
        .push(Wire::segment(7, Point::new(110, 50), Point::new(0, 50)));
    state
        .workspace
        .schematic_buffers
        .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

    state
        .store_active_symbol_document(&SymbolDocument {
            origin: Point::origin(),
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(30, 0)),
            )],
            ..SymbolDocument::default()
        })
        .expect("updated symbol stores");

    let parent = state
        .workspace
        .schematic_buffers
        .get("work/top/schematic")
        .expect("parent schematic remains open");
    assert_eq!(parent.wires[0].points[0], Point::new(130, 50));
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
fn copy_cell_flushes_live_active_schematic_before_copying_buffers() {
    let mut state = state_with_work_cell("amp");
    let active_key = CellViewRef::new("work", "amp", "schematic").key();
    state
        .workspace
        .schematic_buffers
        .insert(active_key, SchematicState::default());
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let project_revision_before = state.workspace.project.revision().get();

    let copied = state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("copy succeeds");

    let copy = state
        .workspace
        .schematic_buffers
        .get("work/amp_copy/schematic")
        .expect("copy buffer exists");
    assert_eq!(copied, 1);
    assert_eq!(copy.components.len(), 1);
    assert_eq!(copy.components[0].kind, ComponentType::Resistor);
    assert_eq!(
        state.workspace.project.revision().get(),
        project_revision_before + 1
    );
    assert!(state.workspace.project_metadata_dirty);
}

#[test]
fn copy_and_rename_cell_keep_veriloga_source_ownership_exact() {
    let mut state = state_with_work_cell("amp");
    let original_id = add_cell_veriloga_source(&mut state, "amp");
    let configuration_id = state
        .workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Release".to_owned(),
            root: CellViewRef::new("work", "amp", "schematic"),
            dut_path: "/top".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("configuration root");
    let inactive_configuration_id = state
        .workspace
        .configuration_sets
        .clone_configuration(configuration_id, 1, "Characterization")
        .expect("inactive configuration root");
    let project_revision_before_copy = state.workspace.project.revision().get();

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("copy succeeds");
    assert_eq!(
        state.workspace.project.revision().get(),
        project_revision_before_copy + 1
    );
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::CopyCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        }) if source_library == "work"
            && source_cell == "amp"
            && target_library == "work"
            && target_cell == "amp_copy"
    ));
    let copied_owner = crate::state::ProjectSourceOwner::cell_view(CellViewRef::new(
        "work", "amp_copy", "behavior",
    ));
    let copied = state
        .workspace
        .project_sources
        .bundle_for_owner(&copied_owner)
        .expect("copied source exists");
    assert_ne!(copied.id(), original_id);
    assert_eq!(
        copied.root().content(),
        state
            .workspace
            .project_sources
            .get_bundle(original_id)
            .expect("original source")
            .root()
            .content()
    );

    let revision = state
        .workspace
        .project_sources
        .get_bundle(original_id)
        .expect("original source")
        .revision();
    let project_revision_before_rename = state.workspace.project.revision().get();
    state
        .rename_cell("work", "amp", "amp_renamed")
        .expect("rename succeeds");
    assert_eq!(
        state.workspace.project.revision().get(),
        project_revision_before_rename + 1
    );
    assert_eq!(state.workspace.project.library_mutation_audit().len(), 2);
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::RenameCell {
            library,
            from_cell,
            to_cell,
        }) if library == "work" && from_cell == "amp" && to_cell == "amp_renamed"
    ));
    let renamed_owner = crate::state::ProjectSourceOwner::cell_view(CellViewRef::new(
        "work",
        "amp_renamed",
        "behavior",
    ));
    let renamed = state
        .workspace
        .project_sources
        .bundle_for_owner(&renamed_owner)
        .expect("renamed source exists");
    assert_eq!(renamed.id(), original_id);
    assert!(renamed.revision() > revision);
    assert!(
        state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
                CellViewRef::new("work", "amp", "behavior")
            ))
            .is_none()
    );
    assert!(state.workspace.project_sources_dirty);
    for id in [configuration_id, inactive_configuration_id] {
        let configuration = state
            .workspace
            .configuration_sets
            .find(id)
            .expect("configuration remains");
        assert_eq!(configuration.root().cell, "amp_renamed");
        assert_eq!(configuration.revision(), 2);
    }
    assert!(state.workspace.project_metadata_dirty);
}

#[test]
fn canonical_cell_collisions_reject_copy_and_rename_without_partial_mutation() {
    let mut state = state_with_work_cell("\u{c9}tage");
    let original_id = add_cell_veriloga_source(&mut state, "\u{c9}tage");
    let persisted_key = CellViewRef::new("work", "\u{c9}tage", "schematic").key();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let before_sources = state.workspace.project_sources.clone();

    let copy_error = state
        .copy_cell("work", "\u{c9}tage", "work", "\u{e9}TAGE")
        .expect_err("accented case aliases cannot create a second cell identity");
    assert!(copy_error.contains("canonical cell identity"));
    assert_eq!(state.workspace.project_sources, before_sources);
    assert_eq!(
        state
            .workspace
            .schematic_buffers
            .get(&persisted_key)
            .expect("source buffer remains")
            .components
            .len(),
        0,
        "a rejected copy must not flush or partially copy live document state"
    );
    assert_eq!(
        state
            .library_manager
            .get_library("work")
            .expect("work library")
            .cell_count(),
        1
    );

    state
        .library_manager
        .get_library_mut("work")
        .expect("work library")
        .add_cell(Cell::new("Cible"));
    let before_sources = state.workspace.project_sources.clone();
    let before_revision = state
        .workspace
        .project_sources
        .get_bundle(original_id)
        .expect("owned source")
        .revision();
    let rename_error = state
        .rename_cell("work", "\u{c9}tage", "cIBLE")
        .expect_err("rename cannot alias an existing canonical identity");
    assert!(rename_error.contains("canonical cell identity"));
    assert_eq!(state.workspace.project_sources, before_sources);
    assert_eq!(
        state
            .workspace
            .project_sources
            .get_bundle(original_id)
            .expect("owned source remains")
            .revision(),
        before_revision
    );
    let library = state
        .library_manager
        .get_library("work")
        .expect("work library");
    assert!(library.get_cell("\u{c9}tage").is_some());
    assert!(library.get_cell("Cible").is_some());
    assert!(library.get_cell("cIBLE").is_none());
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
    assert_eq!(state.ui.symbol.selected_pin.as_deref(), Some("IN"));
    assert_eq!(state.workbench.workspace, Workspace::Design);
}

#[test]
fn symbol_violation_cycle_opens_symbol_view_and_selects_pin() {
    let mut state = state_with_amp_symbol_pin("IN", None);
    state.run_active_symbol_pin_checks();
    state.ui.symbol.clear_selection();

    crate::schematic::view::violations::cycle_violation(&mut state, 1);

    assert_eq!(
        state.workspace.active_view,
        CellViewRef::new("work", "amp", "symbol")
    );
    assert_eq!(state.ui.symbol.selected_pin.as_deref(), Some("IN"));
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
fn symbol_undo_and_redo_restore_editor_sidecar_atomically() {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let document = SymbolDocument::default();
    let baseline = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &baseline)
        .expect("baseline symbol bundle stores");
    state.record_symbol_edit(&document);
    let mut edited = baseline.clone();
    edited
        .attribute_mut(crate::state::SymbolAttributeKind::Value)
        .expect("the value attribute exists")
        .default_value = "durable annotation".to_owned();
    state
        .store_active_symbol_editor_bundle(&document, &edited)
        .expect("edited symbol bundle stores");

    let annotation = |state: &mut AppState| {
        state
            .load_active_symbol_editor_metadata(&document)
            .expect("the sidecar loads")
            .attribute(crate::state::SymbolAttributeKind::Value)
            .expect("the value attribute survives")
            .default_value
            .clone()
    };

    assert!(
        state
            .undo_active_symbol_document()
            .expect("symbol bundle undo succeeds")
    );
    assert_eq!(annotation(&mut state), "VALUE");
    assert!(
        state
            .redo_active_symbol_document()
            .expect("symbol bundle redo succeeds")
    );
    assert_eq!(annotation(&mut state), "durable annotation");
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
fn copy_cell_regenerates_design_management_sheet_and_port_ids_without_cloning_variants() {
    let mut state = state_with_work_cell("amp");
    let main_component = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    let boundary_component = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    let moved_component = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(80, 0));
    let owner = CellViewRef::new("work", "amp", "schematic").key();
    let main_sheet = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(
            &owner,
            "Main",
            [main_component, boundary_component, moved_component],
        )
        .expect("source sheet catalog");
    let source_catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&owner)
        .expect("source catalog");
    let auxiliary_sheet = source_catalog
        .create_sheet(
            crate::state::SheetDefinition {
                name: "Auxiliary".to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(main_sheet),
        )
        .expect("auxiliary sheet");
    source_catalog
        .move_selection(crate::state::MoveSelectionRequest {
            expected_catalog_revision: source_catalog.revision(),
            object_ids: vec![moved_component],
            destination_sheet_id: auxiliary_sheet,
            boundary_resolution: crate::state::MoveBoundaryResolution::ExplicitPorts {
                ports: vec![crate::state::CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: crate::state::CrossSheetPortEndpoint {
                        sheet_id: main_sheet,
                        anchor: crate::state::CrossSheetPortAnchor::ComponentTerminal {
                            component_id: boundary_component,
                            terminal_name: "BIAS_OUT".to_owned(),
                        },
                    },
                    second: crate::state::CrossSheetPortEndpoint {
                        sheet_id: auxiliary_sheet,
                        anchor: crate::state::CrossSheetPortAnchor::ComponentTerminal {
                            component_id: moved_component,
                            terminal_name: "BIAS_IN".to_owned(),
                        },
                    },
                    direction: crate::state::CrossSheetPortDirection::Output,
                    signal_type: crate::state::CrossSheetSignalType::Analog,
                    discipline: crate::state::CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .expect("reviewed cross-sheet move");
    let variant_id = state
        .workspace
        .design_management
        .variants_mut()
        .create(crate::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: crate::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                crate::state::SchematicObjectKey::new(&owner, main_component)
                    .expect("scoped source object"),
                crate::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-42".to_owned(),
                },
            )]),
        })
        .expect("source variant");

    let source_sheet_ids = state
        .workspace
        .design_management
        .sheet_catalog(&owner)
        .expect("source catalog")
        .sheets()
        .iter()
        .map(crate::state::DesignSheet::id)
        .collect::<std::collections::BTreeSet<_>>();
    let source_port_ids = state
        .workspace
        .design_management
        .sheet_catalog(&owner)
        .expect("source catalog")
        .cross_sheet_ports()
        .iter()
        .map(crate::state::CrossSheetPortContract::id)
        .collect::<std::collections::BTreeSet<_>>();

    state
        .copy_cell("work", "amp", "work", "amp_copy")
        .expect("cell copy succeeds");

    let copied_owner = CellViewRef::new("work", "amp_copy", "schematic").key();
    let copied_catalog = state
        .workspace
        .design_management
        .sheet_catalog(&copied_owner)
        .expect("copied sheet catalog");
    let copied_sheet_ids = copied_catalog
        .sheets()
        .iter()
        .map(crate::state::DesignSheet::id)
        .collect::<std::collections::BTreeSet<_>>();
    let copied_port_ids = copied_catalog
        .cross_sheet_ports()
        .iter()
        .map(crate::state::CrossSheetPortContract::id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(copied_sheet_ids.len(), source_sheet_ids.len());
    assert!(copied_sheet_ids.is_disjoint(&source_sheet_ids));
    assert_eq!(copied_port_ids.len(), source_port_ids.len());
    assert!(copied_port_ids.is_disjoint(&source_port_ids));

    let resolved = state
        .workspace
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("source variant remains resolvable");
    assert!(
        resolved
            .override_for(&owner, main_component)
            .expect("source key is valid")
            .is_some()
    );
    assert!(
        resolved
            .override_for(&copied_owner, main_component)
            .expect("copied key is valid")
            .is_none(),
        "copying a cell must not silently clone project assembly-variant ownership"
    );
}

#[test]
fn rename_cell_remaps_design_management_scoped_variant_and_annotation_ownership() {
    let mut state = state_with_work_cell("amp");
    let object_id = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let old_owner = CellViewRef::new("work", "amp", "schematic").key();
    let sheet_id = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&old_owner, "Main", [object_id])
        .expect("source sheet catalog");
    let old_object =
        crate::state::SchematicObjectKey::new(&old_owner, object_id).expect("source scoped object");
    let variant_id = state
        .workspace
        .design_management
        .variants_mut()
        .create(crate::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: crate::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                old_object.clone(),
                crate::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-17".to_owned(),
                },
            )]),
        })
        .expect("source variant");
    let renumber_request = crate::state::RenumberRequest {
        scope: crate::state::RenumberScope::WholeProject,
        order: crate::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references: crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![crate::state::AnnotationObject {
            object: old_object.clone(),
            current_reference: "R9".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(sheet_id),
            hierarchy_path: "/top".to_owned(),
            position: crate::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = state
        .workspace
        .design_management
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    state
        .workspace
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");

    state
        .rename_cell("work", "amp", "amp_rev_b")
        .expect("cell rename succeeds");

    let new_owner = CellViewRef::new("work", "amp_rev_b", "schematic").key();
    assert!(
        state
            .workspace
            .design_management
            .sheet_catalog(&old_owner)
            .is_none()
    );
    assert_eq!(
        state
            .workspace
            .design_management
            .sheet_catalog(&new_owner)
            .expect("renamed sheet catalog")
            .active_sheet_id(),
        Some(sheet_id),
        "renaming ownership must preserve stable sheet identity"
    );
    let resolved = state
        .workspace
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("renamed variant remains resolvable");
    assert!(
        resolved
            .override_for(&old_owner, object_id)
            .expect("old key is valid")
            .is_none()
    );
    assert!(
        resolved
            .override_for(&new_owner, object_id)
            .expect("new key is valid")
            .is_some()
    );
    assert!(
        state
            .workspace
            .design_management
            .annotation()
            .effective_mapping_for(&old_owner, object_id)
            .expect("old annotation lookup")
            .is_none()
    );
    assert!(
        state
            .workspace
            .design_management
            .annotation()
            .effective_mapping_for(&new_owner, object_id)
            .expect("renamed annotation lookup")
            .is_some()
    );
}

#[test]
fn rename_cell_publishes_scoped_design_management_remaps_without_a_sheet_catalog() {
    let mut state = state_with_work_cell("amp");
    let object_id = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(20, 20));
    let old_owner = CellViewRef::new("work", "amp", "schematic").key();
    let old_object =
        crate::state::SchematicObjectKey::new(&old_owner, object_id).expect("source scoped object");
    let variant_id = state
        .workspace
        .design_management
        .variants_mut()
        .create(crate::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: crate::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                old_object.clone(),
                crate::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-18".to_owned(),
                },
            )]),
        })
        .expect("source variant");
    let renumber_request = crate::state::RenumberRequest {
        scope: crate::state::RenumberScope::WholeProject,
        order: crate::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references: crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![crate::state::AnnotationObject {
            object: old_object,
            current_reference: "R19".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: crate::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = state
        .workspace
        .design_management
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    state
        .workspace
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");

    state
        .rename_cell("work", "amp", "amp_rev_c")
        .expect("cell rename succeeds");

    let new_owner = CellViewRef::new("work", "amp_rev_c", "schematic").key();
    let resolved = state
        .workspace
        .design_management
        .variants()
        .resolve(variant_id)
        .expect("renamed variant remains resolvable");
    assert!(
        resolved
            .override_for(&old_owner, object_id)
            .expect("old key is valid")
            .is_none()
    );
    assert!(
        resolved
            .override_for(&new_owner, object_id)
            .expect("new key is valid")
            .is_some()
    );
    assert!(
        state
            .workspace
            .design_management
            .annotation()
            .effective_mapping_for(&new_owner, object_id)
            .expect("renamed annotation lookup")
            .is_some()
    );
}

#[test]
fn rename_top_cell_then_serialize_succeeds() {
    let mut state = AppState::default();
    state.provision_test_project_technology_contract();
    assert_eq!(state.workspace.project.root_library, "user");
    assert_eq!(state.workspace.project.top_cell, "top");

    state
        .rename_cell("user", "top", "system")
        .expect("the project root cell is writable");

    assert_eq!(state.workspace.project.top_cell, "system");
    crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("a renamed project root cell must still serialize and validate");
}

#[test]
fn create_library_rejects_a_canonical_identity_collision() {
    let mut state = AppState::default();
    let before = state.library_manager.library_count();

    state
        .create_library("vendor")
        .expect("a fresh library name is accepted");
    assert_eq!(state.library_manager.library_count(), before + 1);
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::CreateLibrary { library }) if library == "vendor"
    ));

    let error = state
        .create_library("VENDOR")
        .expect_err("a case-folded duplicate is the same library identity");
    assert!(error.contains("vendor"), "{error}");
    assert_eq!(state.library_manager.library_count(), before + 1);
}

/// The default project owns `user/top/schematic`; this adds the second cell,
/// the instance that binds it, its owned source, and its sheet catalog, so one
/// rename has something to move in every propagation row.
fn state_with_populated_user_library() -> AppState {
    let mut state = AppState::default();
    let top = CellViewRef::new("user", "top", "schematic");
    let amp = CellViewRef::new("user", "amp", "schematic");
    if let Some(library) = state.library_manager.get_library_mut("user") {
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("behavior", ViewType::VerilogA));
        library.add_cell(cell);
    }
    let mut top_schematic = SchematicState::default();
    top_schematic.add_library_cell_component(
        Point::new(10, 10),
        LibraryCellInstance::new("user", "amp", "schematic"),
    );
    state
        .workspace
        .schematic_buffers
        .insert(top.key(), top_schematic.clone());
    state
        .workspace
        .schematic_buffers
        .insert(amp.key(), SchematicState::default());
    state.workspace.open_views = vec![crate::state::OpenCellView::new(
        top.clone(),
        ViewType::Schematic,
    )];
    state.workspace.active_view = top.clone();
    state.workspace.hierarchy_stack = vec![top];
    state.workspace.hierarchy_instances.clear();
    state.schematic = top_schematic;

    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(CellViewRef::new("user", "amp", "behavior")),
        crate::state::ProjectSourceLanguage::VerilogA,
        "behavior.va",
        "module behavior(p, n); inout p, n; endmodule",
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid source bundle");
    state
        .workspace
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&amp.key(), "Main", [])
        .expect("owned sheet catalog");
    state
        .workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Release".to_owned(),
            root: CellViewRef::new("user", "top", "schematic"),
            dut_path: "/XAMP".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("valid configuration root");
    state
}

#[test]
fn rename_library_propagation_matrix() {
    let mut state = state_with_populated_user_library();
    let source_id = state
        .workspace
        .project_sources
        .iter_bundles()
        .find(|bundle| {
            matches!(
                bundle.owner(),
                crate::state::ProjectSourceOwner::CellView { .. }
            )
        })
        .expect("fixture cell-view source bundle")
        .id();

    let remapped = state
        .rename_library("user", "project_lib")
        .expect("a writable library renames");

    assert_eq!(remapped, 1, "the bound instance follows the library");
    assert!(state.library_manager.get_library("user").is_none());
    assert!(state.library_manager.get_library("project_lib").is_some());
    assert!(
        state
            .workspace
            .schematic_buffers
            .keys()
            .all(|key| key.starts_with("project_lib/")),
        "every buffer is re-keyed under the renamed library"
    );
    assert_eq!(state.workspace.active_view.library, "project_lib");
    assert!(
        state
            .workspace
            .open_views
            .iter()
            .all(|open| open.reference.library == "project_lib")
    );
    assert!(
        state
            .workspace
            .hierarchy_stack
            .iter()
            .all(|reference| reference.library == "project_lib")
    );
    assert_eq!(
        state.schematic.components[0]
            .library_cell
            .as_ref()
            .expect("instance binding")
            .library,
        "project_lib"
    );
    assert_eq!(
        state
            .workspace
            .project_sources
            .get_bundle(source_id)
            .expect("bundle identity is stable across a rename")
            .owner(),
        &crate::state::ProjectSourceOwner::cell_view(CellViewRef::new(
            "project_lib",
            "amp",
            "behavior"
        ))
    );
    assert!(
        state
            .workspace
            .design_management
            .sheet_catalog(&CellViewRef::new("project_lib", "amp", "schematic").key())
            .is_some(),
        "sheet-catalog ownership moves with the library"
    );
    assert!(
        state
            .workspace
            .design_management
            .sheet_catalog(&CellViewRef::new("user", "amp", "schematic").key())
            .is_none()
    );
    assert_eq!(
        state.workspace.configuration_sets.configurations()[0]
            .root()
            .library,
        "project_lib",
        "configuration roots follow the renamed library"
    );
    assert_eq!(state.workspace.project.root_library, "project_lib");
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::RenameLibrary {
            from_library,
            to_library,
        }) if from_library == "user" && to_library == "project_lib"
    ));
}

#[test]
fn rename_library_is_blocked_by_a_lease_on_either_name() {
    for locked in ["user", "project_lib"] {
        let mut state = state_with_populated_user_library();
        let snapshot = crate::state::library_browser::ProjectLibraryLockSnapshot::try_new(
            state.workspace.project.id(),
            1,
            state.workspace.project.revision(),
            state.library_manager.revision(),
            "org-lock-service",
            vec![crate::state::library_browser::ProjectLibraryEditLock::new(
                uuid::Uuid::new_v4(),
                "engineer@example.test",
                "org-lock-service",
                crate::state::library_browser::ProjectLibraryEditLockScope::Library {
                    library: locked.to_owned(),
                },
                10,
                20,
            )],
        )
        .expect("valid lock snapshot");
        state
            .library_edit_locks
            .install_authoritative(snapshot)
            .expect("authority installs");

        let error = state
            .rename_library("user", "project_lib")
            .expect_err("a lease on either name blocks the rename");

        assert!(error.contains("is locked by"), "{error}");
        assert!(state.library_manager.get_library("user").is_some());
    }
}

#[test]
fn delete_library_is_blocked_by_root_config_root_and_referenced_master() {
    let mut state = state_with_populated_user_library();

    let root_error = state
        .delete_library("user")
        .expect_err("the project root library cannot be deleted");
    assert!(
        root_error.contains("holds the project root cell"),
        "{root_error}"
    );

    state.workspace.project.root_library = "spare".to_owned();
    let mut spare = Library::new("spare");
    let mut consumer = Cell::new("consumer");
    consumer.add_view(View::new("schematic", ViewType::Schematic));
    spare.add_cell(consumer);
    state.library_manager.add_library(spare);
    let mut consumer_schematic = SchematicState::default();
    consumer_schematic.add_library_cell_component(
        Point::new(20, 20),
        LibraryCellInstance::new("user", "amp", "schematic"),
    );
    state.workspace.schematic_buffers.insert(
        CellViewRef::new("spare", "consumer", "schematic").key(),
        consumer_schematic,
    );

    let configuration_error = state
        .delete_library("user")
        .expect_err("a configuration root cannot be left dangling");
    assert!(
        configuration_error.contains("Configuration set roots still reference"),
        "{configuration_error}"
    );

    state.workspace.configuration_sets = Default::default();
    let reference_error = state
        .delete_library("user")
        .expect_err("a loaded instance master cannot be deleted out from under it");
    assert!(
        reference_error.contains("outside library 'user' still reference a master in it"),
        "{reference_error}"
    );
    assert!(state.library_manager.get_library("user").is_some());
}

#[test]
fn delete_library_removes_its_cells_and_restores_valid_focus() {
    let mut state = state_with_populated_user_library();
    state.workspace.project.root_library = "spare".to_owned();
    state.workspace.project.top_cell = "keep".to_owned();
    let mut spare = Library::new("spare");
    let mut keep = Cell::new("keep");
    keep.add_view(View::new("schematic", ViewType::Schematic));
    spare.add_cell(keep);
    state.library_manager.add_library(spare);
    let survivor = CellViewRef::new("spare", "keep", "schematic");
    state
        .workspace
        .schematic_buffers
        .insert(survivor.key(), SchematicState::default());
    state
        .workspace
        .open_views
        .push(crate::state::OpenCellView::new(
            survivor.clone(),
            ViewType::Schematic,
        ));
    state.workspace.project_sources = Default::default();
    state.workspace.configuration_sets = Default::default();
    if let Some(library) = state.library_manager.get_library_mut("user") {
        library
            .get_cell_mut("amp")
            .expect("fixture amp cell")
            .remove_view("behavior");
    }

    let cells = state
        .delete_library("user")
        .expect("an unreferenced library is deletable");

    assert_eq!(cells, 2);
    assert!(state.library_manager.get_library("user").is_none());
    assert!(
        state
            .workspace
            .schematic_buffers
            .keys()
            .all(|key| !key.starts_with("user/"))
    );
    assert!(
        state
            .workspace
            .open_views
            .iter()
            .all(|open| open.reference.library != "user")
    );
    assert_eq!(state.workspace.active_view, survivor);
    assert!(state.workspace.active_context_schematic().is_some());
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::DeleteLibrary { library }) if library == "user"
    ));
}

#[test]
fn rename_view_moves_the_buffer_source_and_view_exact_bindings() {
    let mut state = state_with_populated_user_library();

    let remapped = state
        .rename_view("user", "top", "schematic", "netlist_view")
        .expect("the configuration root view renames");
    assert_eq!(remapped, 0, "no instance binds the renamed root view");
    assert_eq!(
        state.workspace.configuration_sets.configurations()[0]
            .root()
            .view,
        "netlist_view",
        "configuration roots follow the renamed view"
    );

    let remapped = state
        .rename_view("user", "amp", "schematic", "netlist_view")
        .expect("a writable view renames");

    assert_eq!(
        remapped, 1,
        "only the instances bound to that exact view move"
    );
    assert!(
        state
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("netlist_view"))
            .is_some()
    );
    assert!(
        state
            .workspace
            .schematic_buffers
            .contains_key(&CellViewRef::new("user", "amp", "netlist_view").key())
    );
    assert!(
        !state
            .workspace
            .schematic_buffers
            .contains_key(&CellViewRef::new("user", "amp", "schematic").key())
    );
    assert_eq!(
        state.schematic.components[0]
            .library_cell
            .as_ref()
            .expect("instance binding")
            .view,
        "netlist_view"
    );
    assert!(
        state
            .workspace
            .design_management
            .sheet_catalog(&CellViewRef::new("user", "amp", "netlist_view").key())
            .is_some()
    );
    assert!(matches!(
        state
            .workspace
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::RenameView {
            library,
            cell,
            from_view,
            to_view,
        }) if library == "user" && cell == "amp" && from_view == "schematic"
            && to_view == "netlist_view"
    ));

    let error = state
        .rename_view("user", "amp", "behavior", "NETLIST_VIEW")
        .expect_err("a case-folded duplicate is the same view identity");
    assert!(error.contains("canonical view identity"), "{error}");
}

const DECK: &str = "show-in-netlist fixture\nV1 in 0 1\nR1 in out 1k\n+ tc1=0\n.op\n.end\n";

/// A project whose retained generated deck states two of its three instances,
/// with `R1` spread over a card and its continuation.
fn located_instance_state() -> (AppState, u64, u64, u64) {
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
        NetlistDocument, NetlistDocumentId,
    };

    let mut state = AppState::default();
    let origin = Point::new(20, 20);
    let source_id = state
        .schematic
        .add_component(ComponentType::VoltageSource, origin);
    let load_id = state
        .schematic
        .add_component(ComponentType::Resistor, origin + Point::new(20, 0));
    let unmapped_id = state
        .schematic
        .add_component(ComponentType::Capacitor, origin + Point::new(40, 0));
    for (id, name) in [(source_id, "V1"), (load_id, "R1"), (unmapped_id, "C9")] {
        if let Some(component) = state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
        {
            component.name = name.to_owned();
        }
    }

    let view = state.workspace.active_view.key();
    let cell = format!(
        "{}/{}",
        state.workspace.active_view.library, state.workspace.active_view.cell
    );
    let entry = |line: usize, instance: &str, id: u64| {
        GeneratedSourceMapEntry::try_new(
            line,
            cell.clone(),
            view.clone(),
            Some(instance.to_owned()),
            Some(GeneratedSourceMapEntry::component_identity_for(&view, id)),
        )
        .expect("mapping")
    };
    let digest = crate::state::content_digest("show-in-netlist-input");
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "rspice-show-in-netlist-test",
            GenerationInput::new(crate::product::ObjectRevision::INITIAL, digest),
        )
        .expect("provenance"),
        DECK.as_bytes().to_vec(),
        Vec::new(),
        vec![
            entry(2, "V1", source_id),
            entry(3, "R1", load_id),
            entry(4, "R1", load_id),
        ],
    )
    .expect("artifact");

    state.ui.netlist.generated_source = DECK.to_owned();
    state.ui.netlist.generated_document = Some(
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).expect("document"),
    );
    state.ui.netlist.generated_input_digest = Some(digest);
    state.ui.netlist.current_generation_input_digest = Some(digest);
    // The engineer is looking at their own deck, so the jump has a document
    // transition to make rather than a no-op reactivation to skip.
    state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    (state, source_id, load_id, unmapped_id)
}

fn last_user_message(state: &AppState) -> String {
    state
        .log_buffer
        .entries()
        .filter(|entry| entry.source == LogSource::User)
        .last()
        .map(|entry| entry.message.clone())
        .unwrap_or_default()
}

/// A block has to say which of the three things is missing. One generic
/// "unavailable" is what makes an engineer regenerate a deck that was never
/// the problem.
#[test]
fn locating_an_instance_reports_the_exact_reason_it_cannot_be_located() {
    let (mut state, source_id, load_id, unmapped_id) = located_instance_state();

    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("select one instance")
    );

    state.schematic.selection.select_only_component(source_id);
    assert_eq!(state.selected_instance_netlist_block(), None);

    // The instance's own card is the first of its lines, never the
    // continuation the map also carries.
    state.schematic.selection.select_only_component(load_id);
    state.show_selected_instance_in_netlist();
    assert_eq!(state.ui.netlist.requested_line, Some(3));

    state.schematic.selection.select_only_component(unmapped_id);
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("no netlist line for this instance")
    );

    state.schematic.selection.select_component(load_id);
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("select one instance")
    );

    state.schematic.selection.select_only_component(source_id);
    state.ui.netlist.generated_document = None;
    state.ui.netlist.generated_source.clear();
    assert_eq!(
        state.selected_instance_netlist_block(),
        Some("no generated netlist yet — open the Netlist workspace to generate one")
    );
}

/// One gesture, one transaction: the workspace, the document and the caret
/// move together. Any of the three left behind shows the engineer a deck that
/// is not the one their instance is in.
#[test]
fn showing_an_instance_opens_the_generated_primary_at_its_card() {
    let (mut state, _, load_id, _) = located_instance_state();
    state.schematic.selection.select_only_component(load_id);

    state.show_selected_instance_in_netlist();

    assert_eq!(state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        state.ui.netlist.active_document,
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
    );
    assert!(state.ui.netlist.active_dependency_identity.is_none());
    assert_eq!(state.simulation.netlist_content, DECK);
    // The outline and the source-mapping panel both read the active line, so
    // the caret request and the cursor have to agree about which card it is.
    assert_eq!(state.ui.netlist.requested_line, Some(3));
    assert_eq!(state.ui.netlist.cursor_line, 2);
    assert_eq!(
        last_user_message(&state),
        "Located instance R1 at generated line 3."
    );
}

/// A stale deck still resolves — it is what the project last generated — but
/// the announcement has to say the line came from one, because regeneration
/// can move the card out from under the caret.
#[test]
fn showing_an_instance_from_a_stale_deck_says_so_without_blocking_the_jump() {
    let (mut state, source_id, _, _) = located_instance_state();
    state.schematic.selection.select_only_component(source_id);
    state.ui.netlist.current_generation_input_digest =
        Some(crate::state::content_digest("edited-schematic"));

    assert_eq!(state.selected_instance_netlist_block(), None);
    state.show_selected_instance_in_netlist();

    assert_eq!(state.ui.netlist.requested_line, Some(2));
    assert_eq!(
        last_user_message(&state),
        "Located instance V1 at generated line 2 in a stale deck; regenerating it may move the card."
    );
}

/// Schematic to netlist and back has to land on the instance it started from.
/// The forward jump and the navigator's reveal read the same map, so a round
/// trip that moves the selection means the two disagree about the identity.
#[test]
fn the_round_trip_through_the_generated_deck_selects_the_same_instance() {
    let (mut state, _, load_id, _) = located_instance_state();
    state.schematic.selection.select_only_component(load_id);

    state.show_selected_instance_in_netlist();
    assert_eq!(state.schematic.selection.single_component(), Some(load_id));

    // Reading the selection back the way the navigator's source-mapping panel
    // does: the active line, its map entry, and the component it names.
    let active_line = state.ui.netlist.cursor_line.saturating_add(1);
    let revealed = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(crate::state::NetlistDocument::generated_artifact)
        .and_then(|artifact| artifact.source_map_entry(active_line))
        .and_then(crate::state::GeneratedSourceMapEntry::component_id)
        .expect("the active line names a component");
    assert_eq!(revealed, load_id);

    state.schematic.selection.clear();
    state.schematic.selection.select_component(revealed);
    assert_eq!(state.schematic.selection.single_component(), Some(load_id));
    assert_eq!(state.selected_instance_netlist_block(), None);
}
