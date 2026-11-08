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
    symbol::{SymbolDocument, SymbolPin, SymbolShape, generated},
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
        .library_mut_for_test("work")
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

/// A catalog holding `work/dev` with only `views`, and no schematic buffers.
fn library_with_dev(views: Vec<View>) -> (LibraryCatalog, HashMap<String, SchematicDocument>) {
    let mut libraries = LibraryCatalog::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("dev");
    for view in views {
        cell.add_view(view);
    }
    library.add_cell(cell);
    libraries.add_library(library);
    (libraries, HashMap::new())
}

/// New Cell stores an empty generated symbol for a cell with no ports yet.
/// Read as authored, every port of the module that later defined the cell
/// resolved unplaced, and the netlister refused the instance.
#[test]
fn an_empty_generated_symbol_resolves_from_the_bound_interface() {
    let mut symbol = View::new(generated::GENERATED_SYMBOL_VIEW, ViewType::Symbol);
    generated::store_generated_symbol(&mut symbol, &[], |_| None).expect("symbol stores");
    let (libraries, buffers) =
        library_with_dev(vec![View::new("veriloga", ViewType::VerilogA), symbol]);
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let mut binding = LibraryCellInstance::new("work", "dev", "veriloga");
    binding.bind_interface(&[
        port("p", PortDirection::InOut),
        port("n", PortDirection::InOut),
    ]);

    let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");

    assert!(resolved.issues().is_empty(), "{:?}", resolved.issues());
    assert_eq!(
        resolved
            .connectable_pins()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["p", "n"]
    );
    assert_eq!(
        resolved.source(),
        ResolvedSymbolSource::Authored,
        "it is the cell's symbol view, regenerated, and drawn as one"
    );
    assert_eq!(
        resolved.document(),
        &SymbolDocument::generated_from_ports(&[
            port("p", PortDirection::InOut),
            port("n", PortDirection::InOut),
        ])
    );
}

/// A compiled module's generated symbol is the cell's symbol: the stored
/// document, typed pins and all, resolved as a view the canvas draws. Read as
/// a fallback derived from the ports, every instance of the module drew as an
/// unresolved cell.
#[test]
fn a_generated_symbol_that_states_the_interface_is_drawn_as_stored() {
    let ports = [
        port("inp", PortDirection::In),
        port("clk", PortDirection::In),
        port("out", PortDirection::Out),
    ];
    let mut symbol = View::new(generated::GENERATED_SYMBOL_VIEW, ViewType::Symbol);
    generated::store_generated_symbol(&mut symbol, &ports, |port| {
        (port.name == "clk").then_some(rspice_design_model::symbol_pin::SymbolElectricalType::Logic)
    })
    .expect("symbol stores");
    let stored = SymbolDocument::load_from_view(&symbol).expect("the stored document loads");
    let (libraries, buffers) =
        library_with_dev(vec![View::new("veriloga", ViewType::VerilogA), symbol]);
    let mut binding = LibraryCellInstance::new("work", "dev", "veriloga");
    binding.bind_interface(&ports);

    let resolved = SymbolResolver::new(&libraries, &buffers)
        .resolve_binding(&binding)
        .expect("symbol resolves");

    assert_eq!(resolved.source(), ResolvedSymbolSource::Authored);
    assert!(resolved.issues().is_empty(), "{:?}", resolved.issues());
    assert_eq!(resolved.document(), &stored);
    assert_eq!(
        resolved
            .connectable_pins()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["inp", "clk", "out"]
    );
}

/// A bus pin in a generated symbol's `ports` keeps its whole name.
#[test]
fn a_generated_symbols_bus_pin_resolves_under_its_whole_name() {
    let mut symbol = View::new(generated::GENERATED_SYMBOL_VIEW, ViewType::Symbol);
    symbol.metadata.insert(
        generated::GENERATED_MARKER_KEY.to_owned(),
        "ports".to_owned(),
    );
    symbol.metadata.insert(
        generated::GENERATED_PORTS_KEY.to_owned(),
        "D[7:0]:inout en:in".to_owned(),
    );
    let (libraries, buffers) = library_with_dev(vec![symbol]);
    let resolver = SymbolResolver::new(&libraries, &buffers);

    let resolved = resolver
        .resolve_reference(&CellViewRef::new("work", "dev", "symbol"))
        .expect("symbol resolves");

    assert_eq!(
        resolved
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), pin.direction))
            .collect::<Vec<_>>(),
        [("D[7:0]", PortDirection::InOut), ("en", PortDirection::In)]
    );
}

/// A subcircuit placed from a vendor model states its pins' roles, and the
/// amplifier it describes is drawn as its symbol, not as an unresolved cell.
#[test]
fn an_instance_that_states_pin_roles_draws_the_symbol_they_describe() {
    use rspice_design_model::port::PinRole;
    let (libraries, buffers) = library_with_amp(None);
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let roles = [
        PinRole::NonInvertingInput,
        PinRole::InvertingInput,
        PinRole::PositiveSupply,
        PinRole::NegativeSupply,
        PinRole::Output,
    ];
    let ports = ["1", "2", "99", "50", "45"]
        .into_iter()
        .zip(roles)
        .map(|(name, role)| port(name, role.direction()))
        .collect::<Vec<_>>();
    let mut binding = LibraryCellInstance::new("vendor", "OPAMP", "spice");
    binding.bind_interface_with_roles(&ports, &roles);

    let resolved = resolver
        .resolve_binding(&binding)
        .expect("the stated interface resolves");

    assert_eq!(resolved.source(), ResolvedSymbolSource::Authored);
    assert_eq!(resolved.connectable_pins().count(), 5);
    assert!(resolved.issues().is_empty());
}

/// A wire on a catalog part's visible pin is on that pin: where the part is
/// drawn and where extraction looks for its terminals are one layout. Compact
/// models join the walk when the catalog is compiled in.
#[test]
fn every_catalog_part_connects_where_it_is_drawn() {
    use crate::schematic::device_catalog::{
        builtin_xspice_library_binding, engine_only_xspice_devices,
    };
    use crate::schematic::functional_catalog::{functional_library_binding, functional_parts};
    use crate::schematic::generated_veriloga_catalog::{
        generated_veriloga_devices, generated_veriloga_library_binding,
    };

    let libraries = LibraryCatalog::default();
    let buffers = HashMap::<String, SchematicDocument>::new();
    let resolver = SymbolResolver::new(&libraries, &buffers);
    let bindings =
        functional_parts()
            .iter()
            .map(functional_library_binding)
            .chain(
                engine_only_xspice_devices()
                    .iter()
                    .map(|device| builtin_xspice_library_binding(device).expect("xspice binding")),
            )
            .chain(generated_veriloga_devices().iter().map(|device| {
                generated_veriloga_library_binding(device).expect("generated binding")
            }));

    for binding in bindings {
        let cell = binding.cell.clone();
        let resolved = resolver.resolve_binding(&binding).expect("resolves");
        let component = crate::schematic::component::Component::new(
            1,
            ComponentType::CellInstance,
            Point::new(100, 100),
        )
        .with_library_cell(binding);
        let drawn: Vec<Point> = component
            .terminal_positions()
            .into_iter()
            .map(|(_, point)| point)
            .collect();
        let extracted: Vec<Point> = component
            .terminal_positions_resolved(Some(&resolved))
            .into_iter()
            .map(|(_, point)| point)
            .collect();
        assert_eq!(drawn, extracted, "{cell}");
    }
}
