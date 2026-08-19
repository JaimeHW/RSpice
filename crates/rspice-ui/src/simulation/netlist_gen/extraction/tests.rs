//! What the one extraction decides: which points are one net, which name wins,
//! and which terminal the drawing leaves open.
//!
//! Every consumer reads these answers, so they are asserted here once rather
//! than re-asserted against each consumer's projection of them.

use super::*;
use crate::state::{BusDeclaration, BusSlice, BusTapOrientation, Wire};

fn named_port(schematic: &mut SchematicState, at: Point, name: &str) {
    let id = schematic.add_component(ComponentType::Port, at);
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("placed port")
        .value = name.to_owned();
}

fn terminal<'a>(
    extracted: &'a ExtractedConnectivity,
    component: u64,
    pin: &str,
) -> &'a ExtractedTerminal {
    extracted
        .terminals
        .iter()
        .find(|terminal| terminal.component_id == component && terminal.pin == pin)
        .expect("extracted terminal")
}

#[test]
fn net_identity_folds_case_under_every_policy() {
    assert_eq!(net_name_key("Out"), net_name_key("out"));
    assert!(net_names_equal("Out", "out"));

    for policy in [
        NetNamingPolicy::StrictCaseSensitive,
        NetNamingPolicy::SpiceCompatibleRelaxed,
    ] {
        let mut schematic = SchematicState::default();
        schematic.document_policy.net_naming = policy;
        schematic
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(40, 0)));
        schematic
            .wires
            .push(Wire::segment(2, Point::new(0, 40), Point::new(40, 40)));
        schematic
            .net_labels
            .push(NetLabel::new(1, Point::new(20, 0), "Out"));
        schematic
            .net_labels
            .push(NetLabel::new(2, Point::new(20, 40), "out"));

        let nets = super::super::design_nets(&schematic);
        assert_eq!(
            nets.len(),
            1,
            "{policy:?}: the deck joins `Out` and `out` into one node, so the drawing must show \
             one net"
        );
        assert_eq!(nets[0].name, "Out");
        let mut wire_ids = nets[0].wire_ids.clone();
        wire_ids.sort_unstable();
        assert_eq!(wire_ids, vec![1, 2]);
    }
}

#[test]
fn one_conductor_binds_its_terminals_and_a_lone_terminal_stands_alone() {
    let mut schematic = SchematicState::default();
    let source = schematic.add_component(ComponentType::VoltageSource, Point::new(0, 0));
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(60, -20));
    schematic.add_component(ComponentType::Ground, Point::new(0, 30));
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, -20), Point::new(40, -20)));

    let extracted = extract(&schematic, None);

    assert_eq!(
        terminal(&extracted, source, "+").net,
        terminal(&extracted, resistor, "+").net,
        "the conductor between them is one node"
    );
    assert_ne!(
        terminal(&extracted, source, "+").net,
        terminal(&extracted, resistor, "-").net
    );

    // The ground symbol's terminal shares the source's negative terminal, so
    // the deck emits node 0 for both.
    assert_eq!(terminal(&extracted, source, "-").net_name, "0");
    assert_eq!(
        extracted.ground_net,
        Some(terminal(&extracted, source, "-").net)
    );

    assert!(terminal(&extracted, source, "+").attached);
    assert!(terminal(&extracted, source, "-").attached);
    assert!(
        !terminal(&extracted, resistor, "-").attached,
        "nothing in the drawing meets it"
    );
}

#[test]
fn a_terminal_is_attached_by_a_conductor_a_twin_or_a_label() {
    let mut schematic = SchematicState::default();
    let left = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
    let right = schematic.add_component(ComponentType::Resistor, Point::new(60, 0));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(0, 0), "bias"));

    let extracted = extract(&schematic, None);

    assert!(
        terminal(&extracted, left, "+").attached,
        "a label names the node under the terminal"
    );
    assert!(
        terminal(&extracted, left, "-").attached,
        "the two resistors meet at one point"
    );
    assert!(terminal(&extracted, right, "+").attached);
    assert!(!terminal(&extracted, right, "-").attached);
}

#[test]
fn an_unmarked_crossing_stays_disconnected_until_a_junction_marks_it() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, -100), Point::new(0, 100)));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(-100, 0), Point::new(100, 0)));

    assert_eq!(extract(&schematic, None).nets.len(), 2);

    schematic.add_junction(Point::new(0, 0));
    assert_eq!(extract(&schematic, None).nets.len(), 1);
}

#[test]
fn an_endpoint_meeting_a_segment_connects_without_a_junction() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(0, 0), Point::new(0, 100)));

    assert_eq!(extract(&schematic, None).nets.len(), 1);
}

/// Imported geometry can span the whole design lattice, and an attachment on it
/// is still exact: the pass filters candidates by the segment itself rather
/// than by any approximation of it.
#[test]
fn a_full_range_diagonal_keeps_its_exact_attachments() {
    let mut schematic = SchematicState::default();
    schematic.wires.push(Wire::segment(
        1,
        Point::new(i32::MIN, i32::MIN),
        Point::new(i32::MAX, i32::MAX),
    ));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "diag"));

    let extracted = extract(&schematic, None);
    assert_eq!(extracted.nets.len(), 1);
    assert_eq!(extracted.nets[0].spice_name(), "diag");
    assert!(extracted.diagnostics.is_empty());
}

#[test]
fn a_label_on_no_conductor_is_reported_as_an_orphan() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(20, 0)));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(10, 0), "mid"));
    schematic
        .net_labels
        .push(NetLabel::new(2, Point::new(100, 100), "orphan"));

    let extracted = extract(&schematic, None);
    let orphans: Vec<_> = extracted
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.kind == ConnectivityDiagnosticKind::OrphanNetLabel)
        .collect();

    assert_eq!(orphans.len(), 1);
    assert!(!orphans[0].blocking, "the deck still emits");
    assert_eq!(
        orphans[0].anchor,
        ConnectivityAnchor::NetLabel("orphan".to_owned())
    );
    assert_eq!(extracted.nets.len(), 1);
    assert_eq!(extracted.nets[0].spice_name(), "mid");
}

#[test]
fn an_interface_port_outranks_a_label() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(60, 0)));
    named_port(&mut schematic, Point::new(10, 0), "vin");
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(40, 0), "sense"));

    let extracted = extract(&schematic, None);
    assert_eq!(extracted.nets.len(), 1);
    assert_eq!(extracted.nets[0].spice_name(), "vin");
}

#[test]
fn the_lowest_numbered_label_wins_a_conflict() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(60, 0)));
    schematic
        .net_labels
        .push(NetLabel::new(7, Point::new(40, 0), "late"));
    schematic
        .net_labels
        .push(NetLabel::new(2, Point::new(20, 0), "early"));

    let extracted = extract(&schematic, None);
    assert_eq!(extracted.nets.len(), 1);
    assert_eq!(extracted.nets[0].spice_name(), "early");
    let conflict = extracted
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.kind == ConnectivityDiagnosticKind::NetNaming)
        .expect("the second label is refused rather than applied");
    assert!(!conflict.blocking, "the deck still emits under the winner");
}

#[test]
fn a_ground_symbol_outranks_every_other_name() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(60, 0)));
    schematic
        .net_labels
        .push(NetLabel::new(1, Point::new(20, 0), "sense"));
    schematic.add_component(ComponentType::Ground, Point::new(0, 10));

    let extracted = extract(&schematic, None);
    assert_eq!(extracted.nets.len(), 1);
    assert_eq!(extracted.nets[0].spice_name(), "0");
    assert_eq!(extracted.ground_net, Some(extracted.nets[0].id));
}

#[test]
fn a_typed_bus_member_outranks_a_disagreeing_label() {
    let mut schematic = SchematicState::default();
    let bus = schematic
        .add_bus(
            vec![Point::new(0, 0), Point::new(20, 0)],
            Some(BusDeclaration::parse("DATA[7:0]").expect("bus declaration")),
        )
        .expect("bus geometry");
    schematic
        .wires
        .push(Wire::segment(100, Point::new(5, 10), Point::new(20, 10)));
    schematic
        .place_bus_tap(
            bus,
            Point::new(5, 0),
            Point::new(5, 10),
            BusSlice::parse("DATA[3]").expect("tap selector"),
            BusTapOrientation::Down,
        )
        .expect("tap geometry");
    schematic
        .net_labels
        .push(NetLabel::new(101, Point::new(15, 10), "foo"));

    let extracted = extract(&schematic, None);
    let tapped = extracted
        .point_to_net
        .get(&Point::new(5, 10))
        .and_then(|id| extracted.nets.iter().find(|net| net.id == *id))
        .expect("the tap binds a scalar net");
    assert_eq!(tapped.spice_name(), "DATA#3");
    assert!(extracted.tapped_nets.contains(&tapped.id));

    let conflict = extracted
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.kind == ConnectivityDiagnosticKind::BusRangeConflict)
        .expect("the label disagrees with the member");
    assert!(conflict.blocking);
    assert!(conflict.message.contains("DATA[3]"));
    assert!(conflict.message.contains("foo"));
}
