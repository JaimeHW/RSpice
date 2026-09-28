
use super::*;
use crate::{
    library::{Cell, Library, LibraryCatalog, View, ViewType},
    resolved_symbol::{ResolvedSymbolIssueKind, ResolvedSymbolSource},
    schematic::{
        component::LibraryCellInstance,
        component_edit::{self, ComponentPlacement},
        component_type::ComponentType,
        document::SchematicDocument,
        identity::SchematicIdentity,
    },
    symbol::{SymbolDocument, SymbolPin, SymbolShape},
};
use rspice_design_model::{
    Point,
    cell_view::CellViewRef,
    port::{PortDirection, PortSpec},
};
use std::collections::HashMap;

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
    }
}

fn library_with_amp(
    symbol: Option<SymbolDocument>,
) -> (LibraryCatalog, HashMap<String, SchematicDocument>) {
    library_with_amp_and_ports(
        symbol,
        &[("IN", PortDirection::In), ("OUT", PortDirection::Out)],
    )
}

fn library_with_amp_and_ports(
    symbol: Option<SymbolDocument>,
    ports: &[(&str, PortDirection)],
) -> (LibraryCatalog, HashMap<String, SchematicDocument>) {
    let mut libraries = LibraryCatalog::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    if let Some(document) = symbol {
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
    }
    cell.add_view(symbol_view);
    library.add_cell(cell);
    libraries.add_library(library);

    let mut schematic = SchematicDocument::default();
    let mut identity = SchematicIdentity::with_cursor(1);
    for (idx, (name, _direction)) in ports.iter().enumerate() {
        let id = component_edit::add_component(
            &mut schematic,
            &mut identity,
            ComponentType::Port,
            ComponentPlacement {
                position: Point::new(idx as i32 * 40, 0),
                rotation: Default::default(),
                mirror_h: false,
            },
            None,
        );
        schematic
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .value = (*name).to_owned();
    }
    let mut buffers = HashMap::new();
    buffers.insert(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    (libraries, buffers)
}

fn library_with_invalid_symbol_metadata() -> (LibraryCatalog, HashMap<String, SchematicDocument>) {
    let (mut libraries, buffers) = library_with_amp(None);
    let symbol_view = libraries
        .get_library_mut("work")
        .and_then(|library| library.get_cell_mut("amp"))
        .and_then(|cell| cell.get_view_mut("symbol"))
        .expect("symbol view exists");
    symbol_view.metadata.insert(
        SYMBOL_DOCUMENT_METADATA_KEY.to_owned(),
        "{not-valid-json".to_owned(),
    );
    (libraries, buffers)
}

#[test]
fn authored_symbol_positions_override_generated_geometry_in_interface_order() {
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(-20, -20),
                Point::new(20, -20),
                Point::new(20, 20),
            ],
            closed: false,
        }],
        ..SymbolDocument::default()
    };
    let (libraries, buffers) = library_with_amp(Some(document));
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
    let pins: Vec<(&str, Point)> = resolved
        .connectable_pins()
        .map(|pin| (pin.name.as_str(), pin.offset))
        .collect();

    assert_eq!(
        pins,
        vec![("IN", Point::new(-40, -10)), ("OUT", Point::new(70, 20))]
    );
    assert!(resolved.issues().is_empty());
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Authored));
}

#[test]
fn resolver_falls_back_to_generated_symbol_when_no_authored_metadata_exists() {
    let (libraries, buffers) = library_with_amp(None);
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let resolved = resolver
        .resolve_binding(&binding)
        .expect("fallback resolves");
    let pins: Vec<(&str, Point)> = resolved
        .connectable_pins()
        .map(|pin| (pin.name.as_str(), pin.offset))
        .collect();

    assert_eq!(pins.len(), 2);
    assert_eq!(pins[0].0, "IN");
    assert_eq!(pins[1].0, "OUT");
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
}

#[test]
fn authored_unplaced_pin_reports_issue_and_is_not_connectable() {
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, None),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(30, 0))),
        ],
        ..SymbolDocument::default()
    };
    let (libraries, buffers) = library_with_amp(Some(document));
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
    let pins: Vec<&str> = resolved
        .connectable_pins()
        .map(|pin| pin.name.as_str())
        .collect();

    assert_eq!(pins, vec!["OUT"]);
    assert!(resolved.issues().iter().any(|issue| {
        matches!(issue.kind, ResolvedSymbolIssueKind::UnplacedPin) && issue.pin_name == "IN"
    }));
}

#[test]
fn placed_binding_prefers_saved_interface_over_live_master_ports() {
    let (libraries, buffers) = library_with_amp_and_ports(
        None,
        &[
            ("IN", PortDirection::In),
            ("OUT", PortDirection::Out),
            ("BIAS", PortDirection::In),
        ],
    );
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
    let pins: Vec<&str> = resolved
        .connectable_pins()
        .map(|pin| pin.name.as_str())
        .collect();

    assert_eq!(pins, vec!["IN", "OUT"]);
}

#[test]
fn invalid_authored_metadata_falls_back_to_generated_symbol_with_issue() {
    let (libraries, buffers) = library_with_invalid_symbol_metadata();
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);

    let resolved = resolver
        .resolve_binding(&binding)
        .expect("fallback resolves");
    let pins: Vec<&str> = resolved
        .connectable_pins()
        .map(|pin| pin.name.as_str())
        .collect();

    assert_eq!(pins, vec!["IN", "OUT"]);
    assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
    assert!(resolved.issues().iter().any(|issue| {
        matches!(issue.kind, ResolvedSymbolIssueKind::InvalidMetadata) && issue.pin_name == "symbol"
    }));
}
