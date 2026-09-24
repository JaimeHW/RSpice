//! What projecting a vector net has to leave alone, and what it has to add.
//!
//! The first test is the golden: a deck that only ever used scalar taps must
//! come out of the vector-net path byte for byte the way it went in, because
//! the alias a tap always applied is now the same node the projection created.
//! Everything after it is the behaviour that did not exist before.

use super::*;
use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, ComponentType, LibraryCellInstance,
    Wire,
};

/// The deck's element and instance cards: everything that is not a comment, a
/// directive, or blank. This is the part of a deck a projection must not move.
fn cards(netlist: &str) -> Vec<String> {
    netlist
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('*') && !line.starts_with('.'))
        .map(str::to_owned)
        .collect()
}

fn declared_bus(id: u64, declaration: &str, start: Point, end: Point) -> Bus {
    Bus::segment(
        id,
        start,
        end,
        Some(BusDeclaration::parse(declaration).expect("fixture declaration")),
    )
    .expect("fixture bus")
}

fn resistor(state: &mut SchematicState, pos: Point, value: &str) {
    let id = state.add_component(ComponentType::Resistor, pos);
    state
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed resistor")
        .value = value.to_owned();
}

fn place_port(state: &mut SchematicState, name: &str, pos: Point) {
    let id = state.add_component(ComponentType::Port, pos);
    state
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed port")
        .value = name.to_owned();
}

/// `DATA[7:0]` tapped at bit 3 onto a wire, one resistor from that wire to
/// ground. Exactly the shape scalar taps have always had.
fn scalar_tap_design() -> SchematicState {
    let mut state = SchematicState::default();
    let bus = declared_bus(1, "DATA[7:0]", Point::new(0, -20), Point::new(80, -20));
    let tap = BusTap::new(
        2,
        &bus,
        Point::new(40, -20),
        Point::new(40, 0),
        BusSlice::parse("DATA[3]").expect("fixture slice"),
        BusTapOrientation::Down,
    )
    .expect("fixture tap");
    state.buses.push(bus);
    state.bus_taps.push(tap);
    state
        .wires
        .push(Wire::segment(3, Point::new(20, 0), Point::new(40, 0)));
    resistor(&mut state, Point::new(40, 0), "1k");
    state.add_component(ComponentType::Ground, Point::new(60, 10));
    state
}

#[test]
fn a_scalar_tap_deck_is_unchanged_by_vector_projection() {
    let state = scalar_tap_design();

    let result = generate_netlist(&state);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    // The tap aliased its wire to `DATA#3` before vector nets existed and it
    // aliases onto the projected `DATA#3` now. One card, one node, no rename.
    assert_eq!(cards(&result.netlist), ["R1 DATA#3 0 1k"]);
    // And exactly one net answers to that name: the tap merged into the
    // projected bit instead of minting a second node spelled the same way.
    assert_eq!(
        result
            .nets
            .keys()
            .filter(|name| name.eq_ignore_ascii_case("DATA#3"))
            .count(),
        1
    );
}

#[test]
fn projection_names_every_bit_of_a_declared_bus_once() {
    let mut state = SchematicState::default();
    // Two buses of the same declaration, touching end to end: one vector net,
    // and eight nodes rather than sixteen.
    state.buses.push(declared_bus(
        1,
        "DATA[7:0]",
        Point::new(0, 0),
        Point::new(40, 0),
    ));
    state.buses.push(declared_bus(
        2,
        "DATA[7:0]",
        Point::new(40, 0),
        Point::new(80, 0),
    ));

    let connectivity = crate::state::vector_connectivity(&state, |component| {
        component.terminal_positions_resolved(None)
    });
    assert_eq!(connectivity.nets.len(), 1);
    assert!(connectivity.mismatches.is_empty());

    let names = generate_netlist(&state)
        .nets
        .into_keys()
        .filter(|name| name.starts_with("DATA#"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names.len(), 8);
    assert!(names.contains("DATA#7") && names.contains("DATA#0"));
}

#[test]
fn a_sixty_four_bit_bus_adds_exactly_sixty_four_named_nets() {
    let mut state = SchematicState::default();
    state.buses.push(declared_bus(
        1,
        "W[63:0]",
        Point::new(0, 0),
        Point::new(80, 0),
    ));

    let named = generate_netlist(&state)
        .nets
        .into_keys()
        .filter(|name| name.starts_with("W#"))
        .count();

    assert_eq!(named, 64);
}

#[test]
fn touching_buses_that_declare_different_ranges_are_not_one_vector_net() {
    let mut state = SchematicState::default();
    state.buses.push(declared_bus(
        1,
        "DATA[7:0]",
        Point::new(0, 0),
        Point::new(40, 0),
    ));
    state.buses.push(declared_bus(
        2,
        "DATA[15:0]",
        Point::new(40, 0),
        Point::new(80, 0),
    ));

    let connectivity = crate::state::vector_connectivity(&state, |component| {
        component.terminal_positions_resolved(None)
    });

    assert_eq!(connectivity.nets.len(), 2);
    // The conflict itself is reported once, by the bus-tap projection that has
    // always owned touching-declaration checks.
    let result = generate_netlist(&state);
    assert_eq!(
        result
            .errors
            .iter()
            .filter(|error| error.contains("incompatible declarations"))
            .count(),
        1
    );
}

#[test]
fn a_slice_tap_resolves_to_the_same_bit_under_either_index_order() {
    for (declaration, slice) in [("DATA[7:0]", "DATA[3]"), ("DATA[0:7]", "DATA[3]")] {
        let mut state = SchematicState::default();
        let bus = declared_bus(1, declaration, Point::new(0, -20), Point::new(80, -20));
        let tap = BusTap::new(
            2,
            &bus,
            Point::new(40, -20),
            Point::new(40, 0),
            BusSlice::parse(slice).expect("fixture slice"),
            BusTapOrientation::Down,
        )
        .expect("fixture tap");
        state.buses.push(bus);
        state.bus_taps.push(tap);
        state
            .wires
            .push(Wire::segment(3, Point::new(20, 0), Point::new(40, 0)));
        resistor(&mut state, Point::new(40, 0), "1k");
        state.add_component(ComponentType::Ground, Point::new(60, 10));

        let result = generate_netlist(&state);

        assert!(
            result.errors.is_empty(),
            "{declaration}: {:?}",
            result.errors
        );
        // Index order decides the position of a bit in a port list, never its
        // name: bit three is `DATA#3` whichever end the declaration starts at.
        assert_eq!(
            cards(&result.netlist),
            ["R1 DATA#3 0 1k"],
            "{declaration} resolved the tap to a different node"
        );
    }
}

#[test]
fn interface_formals_flatten_vectors_from_the_declared_msb_end() {
    let mut master = SchematicState::default();
    place_port(&mut master, "DATA[3:0]", Point::new(100, 0));
    place_port(&mut master, "EN", Point::new(100, 40));
    place_port(&mut master, "ADDR<0:2>", Point::new(100, 80));

    assert_eq!(
        interface_formals(&master),
        [
            "DATA#3", "DATA#2", "DATA#1", "DATA#0", "EN", "ADDR#0", "ADDR#1", "ADDR#2"
        ]
    );
    // The unexpanded contract is untouched: it is what a placement binds to.
    assert_eq!(
        master
            .interface_ports()
            .into_iter()
            .map(|port| port.name)
            .collect::<Vec<_>>(),
        ["DATA[3:0]", "EN", "ADDR<0:2>"]
    );
}

/// A master whose interface is `DATA[3:0]` plus `EN`, with the vector reaching
/// the body through a bus and a scalar tap.
fn reg4_master() -> SchematicState {
    let mut master = SchematicState::default();
    place_port(&mut master, "DATA[3:0]", Point::new(100, 0));
    let bus = declared_bus(1, "DATA[3:0]", Point::new(90, 0), Point::new(150, 0));
    let tap = BusTap::new(
        2,
        &bus,
        Point::new(120, 0),
        Point::new(120, 40),
        BusSlice::parse("DATA[3]").expect("fixture slice"),
        BusTapOrientation::Down,
    )
    .expect("fixture tap");
    master.buses.push(bus);
    master.bus_taps.push(tap);
    master
        .wires
        .push(Wire::segment(3, Point::new(120, 40), Point::new(160, 40)));
    resistor(&mut master, Point::new(180, 40), "1k");
    place_port(&mut master, "EN", Point::new(210, 40));
    master
}

#[test]
fn a_vector_port_becomes_deck_formals_and_per_bit_instance_nodes() {
    let master = reg4_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "reg4", &master);

    let mut top = SchematicState::default();
    let mut binding = LibraryCellInstance::new("work", "reg4", "schematic");
    binding.terminal_order = vec!["DATA[3:0]".to_owned(), "EN".to_owned()];
    let instance = top.add_library_cell_component(Point::new(200, 100), binding);
    let terminals = top
        .components
        .iter()
        .find(|component| component.id == instance)
        .expect("placed instance")
        .terminal_positions();
    let (data, enable) = (terminals[0].1, terminals[1].1);
    top.buses.push(declared_bus(
        1,
        "DATA[3:0]",
        data,
        Point::new(data.x.saturating_add(40), data.y),
    ));
    top.add_component(ComponentType::Ground, Point::new(enable.x, enable.y + 10));

    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let lowered = result.netlist.to_lowercase();
    // Four formals from one drawn pin, in declared MSB-to-LSB order, spelled
    // the way the engine can lex them.
    assert!(
        lowered.contains(".subckt reg4 data#3 data#2 data#1 data#0 en"),
        "netlist:\n{}",
        result.netlist
    );
    // Five nodes on the instance: the vector's four bits and the scalar.
    assert!(
        lowered.contains("x1 data#3 data#2 data#1 data#0 0 reg4"),
        "netlist:\n{}",
        result.netlist
    );
    // The body reaches bit three by the same name the header declares.
    assert!(
        lowered.contains("r1 data#3 en 1k"),
        "netlist:\n{}",
        result.netlist
    );

    // And the whole thing is a deck the engine reads, with the node names
    // surviving the round trip.
    let parsed = rspice_core::netlist::Netlist::parse(&result.netlist)
        .expect("the generated vector deck parses");
    let child = parsed
        .subcircuits
        .iter()
        .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("reg4"))
        .expect("reg4 is defined");
    assert_eq!(child.ports, ["DATA#3", "DATA#2", "DATA#1", "DATA#0", "EN"]);
    let x1 = parsed
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("X1"))
        .expect("X1 is in the parsed deck");
    assert_eq!(x1.nodes, ["DATA#3", "DATA#2", "DATA#1", "DATA#0", "0"]);
}

/// The golden hierarchical deck: one four-bit vector between two instances of
/// one cell. Four formals from one drawn pin, four nodes on each X-line, one
/// master body for both occurrences, and a deck the engine reads.
#[test]
fn one_four_bit_vector_joins_two_instances_of_one_master() {
    let master = reg4_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "reg4", &master);

    let mut top = SchematicState::default();
    let mut data_terminals = Vec::new();
    for row in [100, 300] {
        let mut binding = LibraryCellInstance::new("work", "reg4", "schematic");
        binding.terminal_order = vec!["DATA[3:0]".to_owned(), "EN".to_owned()];
        let instance = top.add_library_cell_component(Point::new(200, row), binding);
        let terminals = top
            .components
            .iter()
            .find(|component| component.id == instance)
            .expect("placed instance")
            .terminal_positions();
        data_terminals.push(terminals[0].1);
        let enable = terminals[1].1;
        top.add_component(ComponentType::Ground, Point::new(enable.x, enable.y + 10));
    }

    // One bus, declared once, reaching both vector terminals.
    let (first, second) = (data_terminals[0], data_terminals[1]);
    top.buses.push(
        Bus::new(
            1,
            vec![
                first,
                Point::new(first.x.saturating_sub(40), first.y),
                Point::new(second.x.saturating_sub(40), second.y),
                second,
            ],
            Some(BusDeclaration::parse("DATA[3:0]").expect("fixture declaration")),
        )
        .expect("fixture bus"),
    );

    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let parsed = rspice_core::netlist::Netlist::parse(&result.netlist)
        .expect("the generated vector deck parses");

    // One body, whatever the occurrence count: both instances resolve to the
    // same master, so the four formals are declared once.
    let bodies: Vec<_> = parsed
        .subcircuits
        .iter()
        .filter(|subcircuit| subcircuit.name.eq_ignore_ascii_case("reg4"))
        .collect();
    assert_eq!(bodies.len(), 1);
    assert_eq!(
        bodies[0].ports,
        ["DATA#3", "DATA#2", "DATA#1", "DATA#0", "EN"]
    );

    // Both X-lines carry the vector's four conductors in the declared order,
    // and both name the one master.
    let instances: Vec<_> = parsed
        .elements
        .iter()
        .filter(|element| element.name.to_ascii_uppercase().starts_with('X'))
        .collect();
    assert_eq!(instances.len(), 2);
    for instance in instances {
        assert_eq!(
            instance.nodes,
            ["DATA#3", "DATA#2", "DATA#1", "DATA#0", "0"],
            "{} bound a different set of conductors",
            instance.name
        );
    }

    // Four conductors reached the deck, not eight: the two terminals joined one
    // vector net rather than each minting its own.
    assert_eq!(
        result
            .nets
            .keys()
            .filter(|name| name.starts_with("DATA#"))
            .count(),
        4
    );
}

/// A multi-bit tap is an electrical join even though it projects no scalar
/// binding: both ends of it name the same conductors.
#[test]
fn a_multi_bit_tap_lands_its_slice_on_the_source_bus_conductors() {
    let mut state = SchematicState::default();
    let source = declared_bus(1, "DATA[7:0]", Point::new(0, -40), Point::new(160, -40));
    let destination = declared_bus(2, "DATA[3:0]", Point::new(0, 40), Point::new(160, 40));
    let slice_tap = BusTap::new(
        3,
        &source,
        Point::new(40, -40),
        Point::new(40, 40),
        BusSlice::parse("DATA[3:0]").expect("fixture slice"),
        BusTapOrientation::Down,
    )
    .expect("fixture slice tap");
    // One scalar tap off each bus, at the same bit, each loading a resistor.
    let from_source = BusTap::new(
        4,
        &source,
        Point::new(100, -40),
        Point::new(100, -80),
        BusSlice::parse("DATA[3]").expect("fixture member"),
        BusTapOrientation::Up,
    )
    .expect("fixture source tap");
    let from_destination = BusTap::new(
        5,
        &destination,
        Point::new(100, 40),
        Point::new(100, 80),
        BusSlice::parse("DATA[3]").expect("fixture member"),
        BusTapOrientation::Down,
    )
    .expect("fixture destination tap");
    state.buses = vec![source, destination];
    state.bus_taps = vec![slice_tap, from_source, from_destination];
    state
        .wires
        .push(Wire::segment(6, Point::new(80, -80), Point::new(100, -80)));
    state
        .wires
        .push(Wire::segment(7, Point::new(80, 80), Point::new(100, 80)));
    resistor(&mut state, Point::new(100, -80), "1k");
    state.add_component(ComponentType::Ground, Point::new(120, -70));
    resistor(&mut state, Point::new(100, 80), "2k");
    state.add_component(ComponentType::Ground, Point::new(120, 90));

    let result = generate_netlist(&state);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    // Bit three of the slice is bit three of the bus it was sliced from: one
    // node, reached from either end.
    assert_eq!(cards(&result.netlist), ["R1 DATA#3 0 1k", "R2 DATA#3 0 2k"]);
    // Eight conductors, not twelve: the destination declared four of the
    // source's bits and added none of its own.
    assert_eq!(
        result
            .nets
            .keys()
            .filter(|name| name.starts_with("DATA#"))
            .count(),
        8
    );
}

#[test]
fn a_projected_bit_is_shown_in_the_notation_its_own_bus_declared() {
    let mut square = SchematicState::default();
    square.buses.push(declared_bus(
        1,
        "DATA[3:0]",
        Point::new(0, 0),
        Point::new(40, 0),
    ));
    let mut angle = SchematicState::default();
    angle.buses.push(declared_bus(
        1,
        "DATA<3:0>",
        Point::new(0, 0),
        Point::new(40, 0),
    ));

    use super::super::extraction::display_net_name;

    assert_eq!(display_net_name(&square, "DATA#3"), "DATA[3]");
    assert_eq!(display_net_name(&angle, "DATA#3"), "DATA<3>");
    // A name no declaration claims is already the form the drawing shows.
    assert_eq!(display_net_name(&square, "out"), "out");
}

/// A cell that widens one interface port has changed its interface, and the
/// placement that bound the narrower one is stale — not silently truncated to
/// the bits both happen to share.
#[test]
fn widening_a_vector_port_makes_every_placement_of_it_stale() {
    let mut master = SchematicState::default();
    place_port(&mut master, "DATA[7:0]", Point::new(100, 0));
    place_port(&mut master, "EN", Point::new(210, 40));
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "reg", &master);

    let mut top = SchematicState::default();
    let mut binding = LibraryCellInstance::new("work", "reg", "schematic");
    binding.terminal_order = vec!["DATA[3:0]".to_owned(), "EN".to_owned()];
    top.add_library_cell_component(Point::new(200, 100), binding);

    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    let stale = result
        .defects
        .iter()
        .find(|defect| defect.kind() == "stale-interface")
        .unwrap_or_else(|| panic!("no stale interface reported: {:?}", result.defects));
    let message = stale.to_string();
    assert!(
        message.contains("DATA[7:0]") && message.contains("DATA[3:0]"),
        "{message}"
    );
}

#[test]
fn a_vector_terminal_on_a_narrower_bus_is_reported_with_both_widths() {
    let master = reg4_master();
    let mut hierarchy = HierarchySource::empty();
    hierarchy.insert("work", "reg4", &master);

    let mut top = SchematicState::default();
    let mut binding = LibraryCellInstance::new("work", "reg4", "schematic");
    binding.terminal_order = vec!["DATA[3:0]".to_owned(), "EN".to_owned()];
    let instance = top.add_library_cell_component(Point::new(200, 100), binding);
    let data = top
        .components
        .iter()
        .find(|component| component.id == instance)
        .expect("placed instance")
        .terminal_positions()[0]
        .1;
    top.buses.push(declared_bus(
        1,
        "DATA[1:0]",
        data,
        Point::new(data.x.saturating_add(40), data.y),
    ));

    let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

    let reported = result
        .errors
        .iter()
        .find(|error| error.contains("DATA[3:0]") && error.contains("DATA[1:0]"))
        .unwrap_or_else(|| panic!("no width mismatch reported: {:?}", result.errors));
    assert!(
        reported.contains("4 bits") && reported.contains("2 bits"),
        "{reported}"
    );

    // The same refusal, typed: a surface offers a repair by the kind rather
    // than by matching the sentence, and the two numbers are the repair's
    // subject.
    let defect = result
        .defects
        .iter()
        .find(|defect| defect.kind() == "vector-width-mismatch")
        .unwrap_or_else(|| panic!("no typed width mismatch: {:?}", result.defects));
    assert!(matches!(
        defect,
        NetlistDefect::WidthMismatch {
            declared_width: 4,
            found_width: 2,
            ..
        }
    ));
    assert_eq!(&defect.to_string(), reported);
}

/// The permissive policy relaxes what the ERC calls a mismatch, never what a
/// deck may carry: neither variant lets an implicit one through, because the
/// deck would have to invent or drop conductors to emit it.
#[test]
fn no_width_mismatch_policy_lets_an_implicit_mismatch_reach_the_deck() {
    let master = reg4_master();

    for width_mismatch in crate::state::BundleWidthMismatchPolicy::ALL {
        let contract = crate::state::ConnectivityContract {
            policy: crate::state::ConnectivityPolicy {
                width_mismatch,
                ..crate::state::ConnectivityPolicy::default()
            },
            ..crate::state::ConnectivityContract::default()
        };
        let mut hierarchy = HierarchySource::empty().with_connectivity(&contract);
        hierarchy.insert("work", "reg4", &master);

        let mut top = SchematicState::default();
        let mut binding = LibraryCellInstance::new("work", "reg4", "schematic");
        binding.terminal_order = vec!["DATA[3:0]".to_owned(), "EN".to_owned()];
        let instance = top.add_library_cell_component(Point::new(200, 100), binding);
        let data = top
            .components
            .iter()
            .find(|component| component.id == instance)
            .expect("placed instance")
            .terminal_positions()[0]
            .1;
        // A bus wide enough to slice from: the case the permissive policy
        // states as a warning in the ERC, and still refuses here because no
        // selector was authored.
        top.buses.push(declared_bus(
            1,
            "DATA[7:0]",
            data,
            Point::new(data.x.saturating_add(40), data.y),
        ));

        let result = generate_netlist_hierarchical(&top, &[], &hierarchy);

        assert!(
            result
                .defects
                .iter()
                .any(|defect| defect.kind() == "vector-width-mismatch"),
            "{width_mismatch:?} emitted a deck for an implicit mismatch: {:?}",
            result.defects
        );
    }
}

#[test]
fn width_contributions_name_every_vector_in_a_sum() {
    let terminals = vec![
        "DATA[3:0]".to_owned(),
        "EN".to_owned(),
        "ADDR<0:2>".to_owned(),
    ];
    assert_eq!(
        width_contributions(&terminals),
        "DATA[3:0] contributes 4, EN contributes 1, ADDR<0:2> contributes 3"
    );
}
