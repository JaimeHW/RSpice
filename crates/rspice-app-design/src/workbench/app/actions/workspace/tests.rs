//! Tests for workspace-level edit actions.
//!
//! These pin what each action does to the whole active document, and - just as
//! importantly - what a read-only document refuses to let it do.

mod deletion_integrity;
#[cfg(test)]
mod symbol_wire_remap;

#[cfg(test)]
use crate::diagnostics::LogAnchor;
#[cfg(test)]
use crate::state::PortSpec;
use crate::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, Point, PortDirection,
    SchematicState, SymbolDocument, SymbolPin, View, ViewType, Wire,
};
#[cfg(test)]
use crate::state::{
    Component, Rotation, SYMBOL_DOCUMENT_METADATA_KEY, SymbolEditorMetadata, SymbolResolver,
};
#[cfg(test)]
use crate::workbench::ChoicePreference;
#[cfg(test)]
use crate::workbench::app::copy_active_schematic_selection;
use crate::workbench::design::test_host::DesignTestHost;
#[cfg(test)]
use crate::workbench::state::Workspace;
#[cfg(test)]
use rspice_design::resolved_symbol::ResolvedSymbolSource;

#[test]
fn generated_symbol_sync_advances_the_catalog_only_for_real_changes() {
    let mut state = DesignTestHost::default();
    let empty_revision = state.model.library_manager.revision();

    state.sync_active_schematic_to_workspace();

    assert_eq!(
        state.model.library_manager.revision(),
        empty_revision,
        "an empty generated-symbol synchronization must be revision-stable"
    );

    let port_id = state
        .model
        .schematic
        .add_component(ComponentType::Port, Point::origin());
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("new interface port")
        .value = "IN".to_owned();

    state.sync_active_schematic_to_workspace();
    let generated_revision = state.model.library_manager.revision();
    assert!(
        generated_revision > empty_revision,
        "creating a generated symbol must advance the catalog revision"
    );

    state.sync_active_schematic_to_workspace();
    assert_eq!(
        state.model.library_manager.revision(),
        generated_revision,
        "repeating an identical generated-symbol synchronization must be a no-op"
    );
}

/// A generated symbol made while the schematic has no ports, as Create symbol
/// makes on a new cell, survives the sync: it never had a port to lose.
#[test]
fn a_generated_symbol_made_before_any_port_survives_the_sync() {
    let mut state = DesignTestHost::default();
    let active = state.model.workspace.content.active_view.clone();
    let mut symbol = View::new("symbol", ViewType::Symbol);
    rspice_design::symbol::generated::store_generated_symbol(&mut symbol, &[], |_| None)
        .expect("an empty interface encodes");
    state
        .model
        .library_manager
        .edit_library(&active.library)
        .expect("the active library")
        .add_view(&active.cell, symbol)
        .expect("the active cell");
    assert!(state.model.schematic.interface_ports().is_empty());

    state.sync_active_schematic_to_workspace();

    let kept = state
        .model
        .library_manager
        .get_library(&active.library)
        .and_then(|library| library.get_cell(&active.cell))
        .and_then(|cell| cell.get_view("symbol"))
        .map(|view| view.view_type);
    assert_eq!(kept, Some(ViewType::Symbol));
}

#[test]
fn new_document_defaults_are_resolved_once_and_existing_buffers_are_preserved() {
    let mut state = DesignTestHost::default();
    state
        .model
        .preferences
        .set_choice(ChoicePreference::NetNamingPolicy, 1)
        .unwrap();
    let created = state.new_schematic_document();
    assert_eq!(
        created.document().document_policy.net_naming,
        crate::state::NetNamingPolicy::SpiceCompatibleRelaxed
    );

    let reference = CellViewRef::new("work", "preserved", "schematic");
    let mut library = Library::new("work");
    let mut cell = Cell::new("preserved");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    let mut existing = SchematicState::default();
    existing.document_mut_for_test().document_policy.net_naming =
        crate::state::NetNamingPolicy::StrictCaseSensitive;
    state
        .model
        .workspace
        .insert_schematic_editor(reference.key(), existing);

    state.open_workspace_view(reference);

    assert_eq!(
        state.model.schematic.document().document_policy.net_naming,
        crate::state::NetNamingPolicy::StrictCaseSensitive
    );
}

#[test]
fn new_document_preserves_the_exact_runtime_wire_and_bus_routing_mode() {
    let mut state = DesignTestHost::default();
    state.design.schematic_routing_mode = crate::state::WireRoutingMode::VerticalFirst;

    let created = state.new_schematic_document();

    assert_eq!(
        created.session.editor.wire_drawing.routing_mode,
        crate::state::WireRoutingMode::VerticalFirst
    );
    assert_eq!(
        created.session.editor.bus_drawing.routing_mode,
        crate::state::WireRoutingMode::VerticalFirst
    );
}

#[test]
fn new_document_inherits_snap_targets_and_reconciles_the_preferred_pitch() {
    let mut state = DesignTestHost::default();
    state.design.schematic_snap.snap_radius = 9;
    state.design.schematic_snap.snap_to_grid = false;
    state.design.schematic_snap.snap_to_wire_segments = false;
    state.design.schematic_snap.grid_size = 777;
    state
        .model
        .preferences
        .set_choice(ChoicePreference::SchematicGrid, 1)
        .unwrap();
    let mut expected_snap = state.design.schematic_snap.clone();
    expected_snap.grid_size = crate::state::SchematicGridPitch::Mil25.canvas_grid_size();

    let created = state.new_schematic_document();

    assert_eq!(
        created.document().document_policy.grid_pitch,
        crate::state::SchematicGridPitch::Mil25
    );
    assert_eq!(created.document().grid_size, expected_snap.grid_size);
    assert_eq!(created.session.editor.snap_engine, expected_snap);
}

#[test]
fn schematic_edit_authority_includes_late_safe_mode_project_read_only() {
    let mut state = DesignTestHost::default();
    assert!(!state.model.schematic_edit_read_only());

    state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    assert!(state.model.schematic_edit_read_only());
    assert!(crate::workbench::app::deny_read_only_edit(
        &state.model,
        &mut state.workbench
    ));
}

#[cfg(test)]
fn symbol_document(pins: &[(&str, PortDirection, Point)]) -> SymbolDocument {
    SymbolDocument {
        pins: pins
            .iter()
            .map(|(name, direction, position)| SymbolPin::new(*name, *direction, Some(*position)))
            .collect(),
        ..SymbolDocument::default()
    }
}

#[cfg(test)]
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

#[cfg(test)]
fn state_with_amp_symbol(document: SymbolDocument) -> DesignTestHost {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("initial symbol stores");
    amp.add_view(symbol_view);
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    state
}

#[test]
fn production_copy_uses_authored_symbol_terminal_geometry() {
    let mut state = state_with_amp_symbol(symbol_document(&[
        ("IN", PortDirection::In, Point::new(-50, 0)),
        ("OUT", PortDirection::Out, Point::new(50, 0)),
    ]));
    state.model.schematic = SchematicState::default();
    let left = Component::new(1, ComponentType::CellInstance, Point::new(0, 0)).with_library_cell(
        amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
    );
    let right =
        Component::new(2, ComponentType::CellInstance, Point::new(200, 0)).with_library_cell(
            amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
        );
    state.model.schematic.document_mut_for_test().components = vec![left, right];
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(3, Point::new(50, 0), Point::new(150, 0)));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(1);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(2);

    assert!(copy_active_schematic_selection(
        &mut state.design.schematic_clipboard,
        &state.model
    ));
    assert_eq!(state.design.schematic_clipboard.components.len(), 2);
    assert_eq!(
        state.design.schematic_clipboard.wires.len(),
        1,
        "the conductor between authored pins must travel with both selected instances"
    );
}

pub fn state_with_amp_symbol_pin(pin_name: &str, position: Option<Point>) -> DesignTestHost {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = pin_name.to_owned();
    state.model.workspace.insert_schematic_editor(
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

#[test]
fn leaving_symbol_view_does_not_save_stale_schematic_under_symbol_key() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    let mut top = Cell::new("top2");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    state.model.library_manager.add_library(library);

    let amp_schematic = CellViewRef::new("work", "amp", "schematic");
    let amp_symbol = CellViewRef::new("work", "amp", "symbol");
    let top_schematic = CellViewRef::new("work", "top2", "schematic");

    let mut amp_buffer = SchematicState::default();
    amp_buffer.add_component(ComponentType::Port, Point::new(0, 0));
    state
        .model
        .workspace
        .insert_schematic_editor(amp_schematic.key(), amp_buffer.clone());
    state.model.schematic = amp_buffer;
    state.model.workspace.content.active_view = amp_schematic.clone();
    state.model.workspace.content.open_views = vec![crate::state::OpenCellView::new(
        amp_schematic.clone(),
        ViewType::Schematic,
    )];

    state.open_workspace_view(amp_symbol.clone());
    assert_eq!(
        state.model.workspace.content.active_view_type(),
        Some(ViewType::Symbol)
    );

    state.open_workspace_view(top_schematic);

    assert!(
        !state
            .model
            .workspace
            .content
            .schematic_buffers
            .contains_key(&amp_symbol.key()),
        "switching away from symbol must not persist the live schematic under the symbol key"
    );
}

#[test]
fn active_symbol_ports_read_the_paired_schematic_contract() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "OUT".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let ports = crate::workbench::app::active_symbol_ports(&state.model);

    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0].name, "OUT");
}

#[test]
fn generating_active_symbol_document_writes_symbol_view_metadata() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    super::symbol_document::generate_active_symbol_document(
        &mut state.design.symbol,
        &mut state.model,
    )
    .expect("symbol document generated");

    let view = state
        .model
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    let doc =
        crate::state::SymbolDocument::load_from_view(view).expect("stored symbol document parses");
    // A port named IN reads as an input, so it takes the left edge.
    assert_eq!(
        doc.pin("IN").expect("IN pin exists").position,
        Some(Point::new(-40, 0))
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
fn generating_a_symbol_again_keeps_what_its_parts_letter() {
    use rspice_design::symbol::SymbolAttributeKind;
    let mut state = DesignTestHost::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);
    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let generate = |state: &mut DesignTestHost| {
        super::symbol_document::generate_active_symbol_document(
            &mut state.design.symbol,
            &mut state.model,
        )
        .expect("symbol document generated");
    };
    generate(&mut state);
    // The user keeps the part's name and value off its instances.
    let mut hidden = state
        .model
        .load_active_symbol_document()
        .expect("generated symbol loads");
    hidden.show_on_instances(SymbolAttributeKind::Reference, false);
    hidden.show_on_instances(SymbolAttributeKind::Value, false);
    super::symbol_document::store_active_symbol_document(
        &mut state.design.symbol,
        &mut state.model,
        &hidden,
    )
    .expect("hidden lettering stored");

    generate(&mut state);

    let regenerated = state
        .model
        .load_active_symbol_document()
        .expect("regenerated symbol loads");
    assert!(
        regenerated.name_hidden && regenerated.value_hidden,
        "drawing the symbol again redraws its artwork, never what its parts letter"
    );
}

#[test]
fn generate_symbol_document_is_one_undoable_transaction() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    super::symbol_document::generate_active_symbol_document(
        &mut state.design.symbol,
        &mut state.model,
    )
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
        .model
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
            .model
            .load_active_symbol_document()
            .expect("redone symbol document loads")
            .body
            .is_empty(),
        "redo should restore generated symbol artwork"
    );
}

#[test]
fn undo_generate_symbol_restores_generated_fallback_metadata_state() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    assert!(
        !state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists")
            .metadata
            .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "fixture starts without authored symbol metadata"
    );

    super::symbol_document::generate_active_symbol_document(
        &mut state.design.symbol,
        &mut state.model,
    )
    .expect("symbol document generated");
    assert!(
        state
            .model
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
        .model
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    assert!(
        !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "undo must restore the missing authored metadata state"
    );
    let resolver = SymbolResolver::new(
        &state.model.library_manager,
        &state.model.workspace.content.schematic_buffers,
    );
    let resolved = resolver
        .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
        .expect("symbol resolves after undo");
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
}

#[test]
fn undo_first_manual_symbol_edit_restores_generated_fallback_source() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let before = state
        .model
        .load_active_symbol_document()
        .expect("pre-edit fallback document loads");
    assert!(
        !state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists")
            .metadata
            .contains_key(SYMBOL_DOCUMENT_METADATA_KEY)
    );

    crate::workbench::app::record_symbol_edit(
        &mut state.design.symbol,
        &state.model,
        &before,
        "Edit",
    );
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
        .model
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("symbol view exists");
    assert!(
        !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
        "undo must restore the missing authored metadata state"
    );
    let resolver = SymbolResolver::new(
        &state.model.library_manager,
        &state.model.workspace.content.schematic_buffers,
    );
    let resolved = resolver
        .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
        .expect("symbol resolves after undo");
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
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

    assert_eq!(state.model.workspace.content.active_view, reference);
    assert_eq!(
        state.design.symbol.editor.selected_pin.as_deref(),
        Some("IN")
    );
    assert_eq!(state.workbench.workspace, Workspace::Design);
}

#[test]
fn symbol_violation_cycle_opens_symbol_view_and_selects_pin() {
    let mut state = state_with_amp_symbol_pin("IN", None);
    let facts = state.slot_facts();
    super::symbol_document::run_active_symbol_pin_checks(&mut state.model, facts);
    state.design.symbol.editor.clear_selection();

    state.navigation().cycle_violation(1);

    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new("work", "amp", "symbol")
    );
    assert_eq!(
        state.design.symbol.editor.selected_pin.as_deref(),
        Some("IN")
    );
}

#[test]
fn symbol_undo_history_is_scoped_to_active_view() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    for cell_name in ["amp_a", "amp_b"] {
        let mut cell = Cell::new(cell_name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
    }
    state.model.library_manager.add_library(library);

    state.open_workspace_view(CellViewRef::new("work", "amp_a", "symbol"));
    let before_a = SymbolDocument {
        name_anchor: Point::new(-10, -10),
        ..SymbolDocument::default()
    };
    let after_a = SymbolDocument {
        name_anchor: Point::new(-20, -20),
        ..SymbolDocument::default()
    };
    crate::workbench::app::record_symbol_edit(
        &mut state.design.symbol,
        &state.model,
        &before_a,
        "Edit",
    );
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
            .model
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
            .model
            .load_active_symbol_document()
            .expect("amp_a symbol loads")
            .name_anchor,
        Point::new(-10, -10)
    );
}

#[test]
fn symbol_undo_and_redo_restore_editor_sidecar_atomically() {
    let mut state = DesignTestHost::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let document = SymbolDocument::default();
    let baseline = SymbolEditorMetadata::for_document(&document);
    state
        .store_active_symbol_editor_bundle(&document, &baseline)
        .expect("baseline symbol bundle stores");
    crate::workbench::app::record_symbol_edit(
        &mut state.design.symbol,
        &state.model,
        &document,
        "Edit",
    );
    let mut edited = baseline.clone();
    edited
        .attribute_mut(crate::state::SymbolAttributeKind::Value)
        .expect("the value attribute exists")
        .default_value = "durable annotation".to_owned();
    state
        .store_active_symbol_editor_bundle(&document, &edited)
        .expect("edited symbol bundle stores");

    let annotation = |state: &mut DesignTestHost| {
        crate::workbench::app::load_active_symbol_editor_metadata(&state.model, &document)
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
    let mut state = DesignTestHost::default();

    let mut library = Library::new("readonly");
    library.read_only = true;
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

    let error = state
        .store_active_symbol_document(&SymbolDocument::default())
        .expect_err("read-only symbol store is rejected");

    assert_eq!(error, "Read-only - 'readonly' masters cannot be edited");
}

#[test]
fn read_only_symbol_edit_paths_use_consistent_refusal_text() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("readonly");
    library.read_only = true;
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

    let expected = "Read-only - 'readonly' masters cannot be edited";

    assert_eq!(
        state
            .store_active_symbol_document(&SymbolDocument::default())
            .expect_err("store should be refused"),
        expected
    );
    assert_eq!(
        super::symbol_document::generate_active_symbol_document(
            &mut state.design.symbol,
            &mut state.model,
        )
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

    assert!(crate::workbench::app::deny_read_only_edit(
        &state.model,
        &mut state.workbench
    ));
    assert_eq!(state.prompt_text(), Some(expected));
}

#[test]
fn opening_symbol_view_loads_the_paired_schematic_context() {
    let mut state = DesignTestHost::default();

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);

    let mut paired = SchematicState::default();
    let port_id = paired.add_component(ComponentType::Port, Point::new(0, 0));
    paired
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port exists")
        .value = "PAIR".to_owned();
    state
        .model
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "amp", "schematic").key(), paired);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let ports = state.model.schematic.interface_ports();
    assert_eq!(ports.len(), 1);
    assert_eq!(ports[0].name, "PAIR");
}

#[test]
fn create_library_rejects_a_canonical_identity_collision() {
    let mut state = DesignTestHost::default();
    let before = state.model.library_manager.library_count();

    state
        .model
        .create_library("vendor")
        .expect("a fresh library name is accepted");
    assert_eq!(state.model.library_manager.library_count(), before + 1);
    assert!(matches!(
        state
            .model.workspace
            .content.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::CreateLibrary { library }) if library == "vendor"
    ));

    let error = state
        .model
        .create_library("VENDOR")
        .expect_err("a case-folded duplicate is the same library identity");
    assert!(error.contains("vendor"), "{error}");
    assert_eq!(state.model.library_manager.library_count(), before + 1);
}

/// The default project owns `user/top/schematic`; this adds the second cell,
/// the instance that binds it, its owned source, and its sheet catalog, so one
/// rename has something to move in every propagation row.
pub fn state_with_populated_user_library() -> DesignTestHost {
    let mut state = DesignTestHost::default();
    let top = CellViewRef::new("user", "top", "schematic");
    let amp = CellViewRef::new("user", "amp", "schematic");
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
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
        .model
        .workspace
        .insert_schematic_editor(top.key(), top_schematic.clone());
    state
        .model
        .workspace
        .insert_schematic_editor(amp.key(), SchematicState::default());
    state.model.workspace.content.open_views = vec![crate::state::OpenCellView::new(
        top.clone(),
        ViewType::Schematic,
    )];
    state.model.workspace.open_as_root(top, ViewType::Schematic);
    state.model.schematic = top_schematic;

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
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&amp.key(), "Main", [])
        .expect("owned sheet catalog");
    state
        .model
        .workspace
        .content
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
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("valid configuration root");
    state
}

// ===========================================================================
// Convergence cross-probe: a failed run's objects on the drawing
// ===========================================================================

/// A sheet drawing the conductors `OUT` and `MID`, with a cross-probe map
/// captured at the topology it is currently drawn at.
pub fn state_with_probed_conductors() -> DesignTestHost {
    use std::collections::HashMap;

    let mut state = DesignTestHost::default();
    let out_a = Point::new(0, 0);
    let out_b = Point::new(40, 0);
    let mid_a = Point::new(0, 40);
    let mid_b = Point::new(40, 40);
    let wires = &mut state.model.schematic.document_mut_for_test().wires;
    wires.push(Wire::new(91, vec![out_a, out_b]));
    wires.push(Wire::new(92, vec![mid_a, mid_b]));
    state.model.simulation.source.cross_probe.update(
        state.model.workspace.content.active_view.clone(),
        HashMap::from([
            (out_a, "OUT".to_owned()),
            (out_b, "OUT".to_owned()),
            (mid_a, "MID".to_owned()),
            (mid_b, "MID".to_owned()),
        ]),
        HashMap::from([
            ("OUT".to_owned(), vec![out_a, out_b]),
            ("MID".to_owned(), vec![mid_a, mid_b]),
        ]),
        HashMap::new(),
        state.model.schematic.topology_version(),
    );
    state
}

#[cfg(test)]
fn prompt_text(state: &DesignTestHost) -> String {
    state.prompt_text().unwrap_or_default().to_owned()
}

#[test]
fn a_simulation_anchor_marks_every_node_the_run_named() {
    let mut state = state_with_probed_conductors();

    state.jump_to_log_anchor(LogAnchor::Simulation {
        nets: vec!["OUT".to_owned(), "MID".to_owned()],
        devices: Vec::new(),
    });

    assert_eq!(
        state.model.schematic.session.editor.selection.wires.len(),
        2,
        "a failure that named two nodes must mark both, not the first"
    );
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&91)
    );
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&92)
    );
    assert!(state.model.schematic.session.editor.net_highlight.active);
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .center_request
            .is_some()
    );
    assert_eq!(state.workbench.workspace, Workspace::Design);
}

#[test]
fn a_simulation_anchor_refuses_a_schematic_that_moved() {
    let mut state = state_with_probed_conductors();
    state.workbench.activate(Workspace::Results);
    // Drawing anything advances the topology the map was captured against.
    state
        .model
        .schematic
        .add_wire(vec![Point::new(80, 0), Point::new(120, 0)]);

    state.jump_to_log_anchor(LogAnchor::Simulation {
        nets: vec!["OUT".to_owned()],
        devices: Vec::new(),
    });

    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty(),
        "a map that no longer describes this drawing must mark nothing"
    );
    assert_eq!(
        state.workbench.workspace,
        Workspace::Results,
        "a refusal must not navigate away from what the author was reading"
    );
    assert!(
        prompt_text(&state).contains("changed since this result was produced"),
        "the refusal must say why, in the sentence every stale cross-probe uses: {}",
        prompt_text(&state)
    );
}

#[test]
fn a_simulation_anchor_says_which_named_objects_this_sheet_does_not_draw() {
    let mut state = state_with_probed_conductors();

    state.jump_to_log_anchor(LogAnchor::Simulation {
        nets: vec!["OUT".to_owned(), "ELSEWHERE".to_owned()],
        devices: Vec::new(),
    });

    assert_eq!(
        state.model.schematic.session.editor.selection.wires,
        [91].into_iter().collect()
    );
    let prompt = prompt_text(&state);
    assert!(
        prompt.contains("ELSEWHERE") && prompt.contains("Marked 1 of 2"),
        "a partial marking must say how partial it is: {prompt}"
    );
}
