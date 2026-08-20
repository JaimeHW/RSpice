//! That the checker and the deck describe one design.
//!
//! The exit criterion of this module is [`erc_and_netlist_agree_on_nets`]: over
//! every fixture the corpus carries, every terminal's node according to the
//! design check equals its node in the emitted deck. A disagreement is fixed in
//! the shared extraction, never patched into one consumer.

use super::*;
use crate::services::drc::DrcLocation;
use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTapOrientation, Cell, CellViewRef, Library,
    LibraryCellInstance, LibraryManager, NetLabel, Point, PortDirection, PortSpec, SchematicState,
    SymbolDocument, SymbolPin, View, ViewType, Wire,
};
use std::collections::HashMap;

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
    }
}

fn named_port(schematic: &mut SchematicState, at: Point, name: &str) {
    let id = schematic.add_component(ComponentType::Port, at);
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed port")
        .value = name.to_owned();
}

fn reference_of(schematic: &SchematicState, id: u64) -> String {
    schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("placed component")
        .name
        .clone()
}

fn run(schematic: &SchematicState, config: DrcConfig) -> DrcResult {
    run_drc_check_with_hierarchy_and_config(schematic, &HierarchySource::empty(), config)
}

fn of_type(result: &DrcResult, violation_type: DrcViolationType) -> Vec<&DrcViolation> {
    result
        .violations()
        .iter()
        .filter(|violation| violation.violation_type == violation_type)
        .collect()
}

fn library_with_authored_amp_symbol() -> (LibraryManager, HashMap<String, SchematicState>) {
    let mut libraries = LibraryManager::new();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
    cell.add_view(symbol_view);
    library.add_cell(cell);
    libraries.add_library(library);

    let mut master = SchematicState::default();
    for (idx, name) in ["IN", "OUT"].iter().enumerate() {
        let id = master.add_component(ComponentType::Port, Point::new(idx as i32 * 40, 0));
        master
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("port component")
            .value = (*name).to_owned();
    }

    let mut buffers = HashMap::new();
    buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), master);
    (libraries, buffers)
}

fn authored_amp_instance() -> Component {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);
    Component::new(1, ComponentType::CellInstance, Point::new(100, 50)).with_library_cell(binding)
}

/// A declared bus, a scalar tap onto an ordinary wire, and an instance terminal
/// on the tapped conductor.
fn tapped_bus_schematic() -> SchematicState {
    let mut schematic = SchematicState::default();
    let bus = schematic
        .add_bus(
            vec![Point::new(0, 0), Point::new(120, 0)],
            Some(BusDeclaration::parse("DATA[7:0]").expect("bus declaration")),
        )
        .expect("bus geometry");
    schematic
        .wires
        .push(Wire::segment(100, Point::new(20, 40), Point::new(60, 40)));
    schematic
        .place_bus_tap(
            bus,
            Point::new(20, 0),
            Point::new(20, 40),
            BusSlice::parse("DATA[3]").expect("tap selector"),
            BusTapOrientation::Down,
        )
        .expect("tap geometry");
    // Terminals at (60, 40) and (100, 40).
    schematic.add_component(ComponentType::Resistor, Point::new(80, 40));
    schematic
}

#[test]
fn hierarchy_extraction_uses_authored_symbol_pin_coordinates() {
    let (libraries, buffers) = library_with_authored_amp_symbol();
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
    let mut schematic = SchematicState::default();
    schematic.components.push(authored_amp_instance());

    let (components, _) = extract_checked_design(&schematic, &hierarchy);
    let pins: HashMap<_, _> = components[0]
        .pins
        .iter()
        .map(|pin| (pin.name.as_str(), pin.point))
        .collect();

    assert_eq!(pins.get("IN"), Some(&Point::new(60, 40)));
    assert_eq!(pins.get("OUT"), Some(&Point::new(170, 70)));
}

#[test]
fn hierarchy_resolved_unconnected_pin_check_uses_authored_terminal_geometry() {
    let (libraries, buffers) = library_with_authored_amp_symbol();
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
    let mut instance = authored_amp_instance();
    instance.name = "X1".to_owned();
    let mut schematic = SchematicState::default();
    schematic.components.push(instance);
    schematic
        .wires
        .push(Wire::segment(90, Point::new(40, 40), Point::new(60, 40)));

    let result = run_drc_check_with_hierarchy_and_config(
        &schematic,
        &hierarchy,
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        },
    );
    let unconnected = of_type(&result, DrcViolationType::UnconnectedPin);

    assert_eq!(unconnected.len(), 1);
    assert!(unconnected[0].message.contains("X1.OUT"));
    assert_eq!(
        unconnected[0].location,
        DrcLocation::Component {
            id: 1,
            name: "X1".to_owned()
        }
    );
}

#[test]
fn canonical_property_schema_extracts_only_definite_parameter_failures() {
    let mut schematic = SchematicState::default();
    let mut resistor = Component::new(20, ComponentType::Resistor, Point::origin());
    resistor.name = "R20".to_owned();
    resistor.value.clear(); // The canonical 1k default is an effective value.
    resistor.params = "m=0".to_owned();
    schematic.components.push(resistor);

    let (components, _) = extract_checked_design(&schematic, &HierarchySource::empty());
    assert!(components[0].missing_parameters.is_empty());
    assert_eq!(
        components[0].out_of_range_parameters,
        vec![ParameterRangeIssue {
            name: "m".to_owned(),
            display_name: "Multiplier".to_owned(),
            value: 0.0,
            min: Some(1.0),
            max: Some(10_000.0),
        }]
    );
}

/// A cell the hierarchy cannot resolve is reported as definitively unknown
/// rather than left undetermined, so the rule that follows can raise it.
/// `component_known` is only ever `Some` because every caller reaches
/// extraction through a hierarchy-resolved entry point.
#[test]
fn unresolvable_project_cell_is_reported_as_definitively_unknown() {
    let binding = LibraryCellInstance::new("work", "missing", "schematic");
    let mut instance =
        Component::new(30, ComponentType::CellInstance, Point::origin()).with_library_cell(binding);
    instance.name = "X30".to_owned();
    let mut schematic = SchematicState::default();
    schematic.components.push(instance);

    let (components, _) = extract_checked_design(&schematic, &HierarchySource::empty());
    assert_eq!(components[0].component_known, Some(false));
}

#[test]
fn typed_bus_member_conflict_is_reported_by_drc_and_honors_severity_policy() {
    let mut schematic = SchematicState::default();
    let bus_id = schematic
        .add_bus(
            vec![Point::new(0, 0), Point::new(20, 0)],
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
    schematic
        .wires
        .push(Wire::segment(100, Point::new(5, 10), Point::new(20, 10)));
    schematic
        .place_bus_tap(
            bus_id,
            Point::new(5, 0),
            Point::new(5, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
    schematic
        .net_labels
        .push(NetLabel::new(101, Point::new(15, 10), "FOO"));

    let mut config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    };
    config
        .severity_overrides
        .insert(DrcViolationType::BusRangeConflict, DrcSeverity::Critical);

    let result = run(&schematic, config);
    let conflict = result
        .violations()
        .iter()
        .find(|violation| {
            violation.violation_type == DrcViolationType::BusRangeConflict
                && violation.message.contains("FOO")
        })
        .expect("typed member conflict");
    assert_eq!(conflict.severity, DrcSeverity::Critical);
    assert!(conflict.message.contains("DATA[3]"));
    assert_eq!(
        conflict.location,
        DrcLocation::Node {
            net_name: "DATA[3]".to_owned()
        }
    );
}

#[test]
fn duplicate_authored_cell_outputs_on_bus_member_are_reported_with_policy_severity() {
    let (libraries, buffers) = library_with_authored_amp_symbol();
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
    let mut first = authored_amp_instance();
    first.id = 10;
    first.name = "X1".to_owned();
    let mut second = authored_amp_instance();
    second.id = 11;
    second.name = "X2".to_owned();
    second.pos = Point::new(300, 50);

    let mut schematic = SchematicState::default();
    schematic.components.extend([first, second]);
    schematic
        .wires
        .push(Wire::segment(20, Point::new(170, 70), Point::new(370, 70)));
    let bus_id = schematic
        .add_bus(
            vec![Point::new(270, 0), Point::new(370, 0)],
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
    schematic
        .place_bus_tap(
            bus_id,
            Point::new(270, 0),
            Point::new(270, 70),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();

    let mut config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    };
    config.severity_overrides.insert(
        DrcViolationType::DuplicateBusMemberDriver,
        DrcSeverity::Critical,
    );
    let result = run_drc_check_with_hierarchy_and_config(&schematic, &hierarchy, config);
    let violation = result
        .violations()
        .iter()
        .find(|violation| violation.violation_type == DrcViolationType::DuplicateBusMemberDriver)
        .expect("duplicate typed output driver");

    assert_eq!(violation.severity, DrcSeverity::Critical);
    assert!(
        violation.message.contains("DATA[3]"),
        "the drawing's spelling, not the deck's: {}",
        violation.message
    );
    assert!(violation.message.contains("X1"));
    assert!(violation.message.contains("X2"));
}

#[test]
fn bus_drc_locations_preserve_full_u64_identity() {
    let mut schematic = SchematicState::default();
    schematic
        .buses
        .push(Bus::segment(u64::MAX, Point::new(0, 0), Point::new(20, 0), None).unwrap());
    let result = run(
        &schematic,
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        },
    );
    assert!(result.completed);
    assert!(result.violations().iter().any(|violation| {
        violation.violation_type == DrcViolationType::UnnamedBus
            && violation.location == DrcLocation::Bus { id: u64::MAX }
    }));
}

/// An autonamed node is still a real dangling connection, and the finding
/// quotes the name the deck will emit for it.
#[test]
fn an_autonamed_single_connection_node_is_reported_floating() {
    let mut schematic = SchematicState::default();
    schematic.add_component(ComponentType::VoltageSource, Point::new(0, 0));
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(60, -20));
    schematic.add_component(ComponentType::Ground, Point::new(0, 30));
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, -20), Point::new(40, -20)));

    let result = run(
        &schematic,
        DrcConfig {
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );
    let floating = of_type(&result, DrcViolationType::FloatingNode);
    assert_eq!(floating.len(), 1, "{:?}", result.violations());
    assert_eq!(
        floating[0].related_items,
        vec![reference_of(&schematic, resistor)]
    );
    let DrcLocation::Node { net_name } = &floating[0].location else {
        panic!("a floating node is located on its node");
    };
    assert!(
        net_name.starts_with("net") && net_name[3..].chars().all(|c| c.is_ascii_digit()),
        "the deck's autoname, not a coordinate: {net_name}"
    );
}

#[test]
fn an_orphan_label_and_a_dangling_wire_chain_are_exact_and_deterministic() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(20, Point::new(0, 0), Point::new(20, 0)));
    schematic
        .wires
        .push(Wire::segment(10, Point::new(20, 0), Point::new(40, 0)));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(10, 0), "onwire"));
    schematic
        .net_labels
        .push(NetLabel::new(2, Point::new(100, 100), "orphan"));

    let config = DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    };
    let result = run(&schematic, config.clone());

    let orphan = of_type(&result, DrcViolationType::OrphanNetLabel);
    assert_eq!(orphan.len(), 1);
    assert_eq!(
        orphan[0].location,
        DrcLocation::NetLabel {
            name: "orphan".to_owned()
        }
    );
    let dangling = of_type(&result, DrcViolationType::DanglingWire);
    assert_eq!(
        dangling
            .iter()
            .map(|finding| match &finding.location {
                DrcLocation::Wire { id } => *id,
                other => panic!("a dangling conductor is located on the wire, got {other:?}"),
            })
            .collect::<Vec<_>>(),
        vec![10, 20]
    );

    let mut disabled = config;
    disabled.check_unconnected_pins = false;
    let result = run(&schematic, disabled);
    assert!(of_type(&result, DrcViolationType::OrphanNetLabel).is_empty());
    assert!(of_type(&result, DrcViolationType::DanglingWire).is_empty());
}

/// The label shares the terminal's point, so the node takes the label's name
/// and the finding is reported against it.
#[test]
fn a_labelled_terminal_with_no_conductor_is_floating_under_the_label_name() {
    let mut schematic = SchematicState::default();
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(40, 0), "dangling"));

    let result = run(
        &schematic,
        DrcConfig {
            check_missing_ground: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );
    let finding = of_type(&result, DrcViolationType::FloatingNode)
        .into_iter()
        .find(|finding| {
            finding.location
                == DrcLocation::Node {
                    net_name: "dangling".to_owned(),
                }
        })
        .expect("the label names the node the finding quotes");
    assert_eq!(
        finding.related_items,
        vec![reference_of(&schematic, resistor)]
    );
}

#[test]
fn a_bus_bit_terminal_carries_the_deck_node_in_the_check() {
    let schematic = tapped_bus_schematic();
    let (components, connectivity) = extract_checked_design(&schematic, &HierarchySource::empty());
    let tapped = components
        .iter()
        .flat_map(|component| component.pins.iter())
        .find(|pin| pin.point == Point::new(60, 40))
        .expect("the instance terminal on the tapped conductor");

    assert_eq!(tapped.net_name, "DATA#3");
    assert_eq!(
        connectivity.display_names.get("DATA#3").map(String::as_str),
        Some("DATA[3]"),
        "a message quotes the spelling the drawing shows"
    );

    let deck = super::super::netlist_gen::generate_netlist_hierarchical(
        &schematic,
        &[],
        &HierarchySource::empty(),
    );
    assert_eq!(
        deck.point_to_net.get(&Point::new(60, 40)),
        Some(&"DATA#3".to_owned())
    );
}

/// Every drawing shape the corpus carries, so a disagreement between the two
/// consumers cannot hide in a geometry nobody exercised.
fn fixture_corpus() -> Vec<(&'static str, SchematicState)> {
    let mut corpus: Vec<(&'static str, SchematicState)> = Vec::new();

    corpus.push(("empty sheet", SchematicState::default()));

    let mut divider = SchematicState::default();
    divider.add_component(ComponentType::VoltageSource, Point::new(0, 0));
    divider.add_component(ComponentType::Resistor, Point::new(60, -20));
    divider.add_component(ComponentType::Ground, Point::new(0, 30));
    divider
        .wires
        .push(Wire::segment(1, Point::new(0, -20), Point::new(40, -20)));
    corpus.push(("grounded divider", divider));

    let mut crossing = SchematicState::default();
    crossing
        .wires
        .push(Wire::segment(1, Point::new(0, -100), Point::new(0, 100)));
    crossing
        .wires
        .push(Wire::segment(2, Point::new(-100, 0), Point::new(100, 0)));
    crossing.add_junction(Point::new(0, 0));
    crossing.add_component(ComponentType::Resistor, Point::new(120, 0));
    corpus.push(("marked crossing", crossing));

    let mut diagonal = SchematicState::default();
    diagonal
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(80, 80)));
    diagonal.add_component(ComponentType::Resistor, Point::new(60, 40));
    diagonal
        .net_labels
        .push(NetLabel::new(1, Point::new(20, 20), "slant"));
    corpus.push(("any-angle conductor", diagonal));

    let mut named = SchematicState::default();
    named
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(60, 0)));
    named
        .wires
        .push(Wire::segment(2, Point::new(0, 40), Point::new(60, 40)));
    named
        .net_labels
        .push(NetLabel::new(1, Point::new(20, 0), "Out"));
    named
        .net_labels
        .push(NetLabel::new(2, Point::new(20, 40), "out"));
    named_port(&mut named, Point::new(70, 0), "vin");
    named.add_component(ComponentType::Resistor, Point::new(80, 40));
    corpus.push(("case-folded labels and a port", named));

    corpus.push(("typed bus tap", tapped_bus_schematic()));

    let mut lone = SchematicState::default();
    lone.add_component(ComponentType::Resistor, Point::new(0, 0));
    corpus.push(("instance on nothing", lone));

    corpus
}

fn assert_erc_and_deck_agree(
    label: &str,
    schematic: &SchematicState,
    hierarchy: &HierarchySource<'_>,
) {
    let deck = super::super::netlist_gen::generate_netlist_hierarchical(schematic, &[], hierarchy);
    let (components, _) = extract_checked_design(schematic, hierarchy);
    for component in &components {
        for pin in &component.pins {
            assert_eq!(
                deck.point_to_net.get(&pin.point),
                Some(&pin.net_name),
                "{label}: the check and the deck disagree about {}.{} at ({}, {})",
                component.name,
                pin.name,
                pin.point.x,
                pin.point.y
            );
        }
    }
}

/// The exit criterion: zero terminals may disagree, over every fixture.
#[test]
fn erc_and_netlist_agree_on_nets() {
    for (label, schematic) in fixture_corpus() {
        assert_erc_and_deck_agree(label, &schematic, &HierarchySource::empty());
    }

    let (libraries, buffers) = library_with_authored_amp_symbol();
    let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
    let mut placed = SchematicState::default();
    let mut instance = authored_amp_instance();
    instance.name = "X1".to_owned();
    placed.components.push(instance);
    placed
        .wires
        .push(Wire::segment(90, Point::new(40, 40), Point::new(60, 40)));
    assert_erc_and_deck_agree("placed project cell", &placed, &hierarchy);
}

/// The disjoint-set and the coordinate net name it needed were the checker's
/// second extraction. Nothing may bring either back.
#[test]
fn the_second_extraction_has_no_survivors() {
    for (path, source) in [
        (
            "services/drc/extraction.rs",
            include_str!("../extraction.rs"),
        ),
        ("services/drc/checker.rs", include_str!("../checker.rs")),
        ("services/drc/net.rs", include_str!("../net.rs")),
    ] {
        let shipped = crate::source_guard::production_source(source);
        assert!(
            !shipped.contains("net_{"),
            "{path} still mints a coordinate net name; the deck's name is the only one"
        );
        assert!(
            !shipped.contains("DisjointSet"),
            "{path} re-derives connectivity instead of reading the one extraction"
        );
    }
}

/// `NetGraph` was the canvas's own connectivity owner: its own point map, T
/// rule, junction merge and flood fill, with no same-name label or interface
/// port merging and no bus bits at all. Highlighting, net-class colouring and
/// the rename authority read it, so the canvas lit, coloured and renamed nets
/// the deck does not have. It is retired, and no surface may grow a graph, a
/// flood fill or a connected-wire traversal of its own to replace it.
#[test]
fn the_canvas_keeps_no_connectivity_owner_of_its_own() {
    for (path, source) in [
        (
            "state/schematic/net_highlight.rs",
            include_str!("../../../state/schematic/net_highlight.rs"),
        ),
        (
            "schematic/view/scene.rs",
            include_str!("../../../schematic/view/scene.rs"),
        ),
        (
            "schematic/view/interaction.rs",
            include_str!("../../../schematic/view/interaction.rs"),
        ),
        (
            "workbench/app/schematic/named_net.rs",
            include_str!("../../../workbench/app/schematic/named_net.rs"),
        ),
        (
            "workbench/docks/inspector/design.rs",
            include_str!("../../../workbench/docks/inspector/design.rs"),
        ),
        (
            "workbench/docks/inspector/design/inline_instance.rs",
            include_str!("../../../workbench/docks/inspector/design/inline_instance.rs"),
        ),
    ] {
        let shipped = crate::source_guard::production_source(source);
        for banned in ["NetGraph", "find_connected_wires", "wire_adjacency"] {
            assert!(
                !shipped.contains(banned),
                "{path} names `{banned}`; net membership has one owner and the canvas reads it"
            );
        }
    }
}
