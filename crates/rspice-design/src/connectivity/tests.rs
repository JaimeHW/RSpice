//! Connection truth the extractor owns: what joins, what does not, and the
//! name a joined net carries.

use super::*;
use crate::schematic::bus::{Bus, BusDeclaration};
use crate::schematic::net_label::Junction;
use crate::schematic::wire::Wire;

fn extracted(document: &SchematicDocument) -> ExtractedConnectivity {
    extract(document, |_| None, |_| None)
}

fn resistor(id: u64, position: Point) -> Component {
    Component::new(id, ComponentType::Resistor, position)
}

fn ambiguous_crossings(extracted: &ExtractedConnectivity) -> Vec<&ConnectivityDiagnostic> {
    extracted
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.kind == ConnectivityDiagnosticKind::AmbiguousCrossing)
        .collect()
}

/// Two wires crossing with no junction stay two nets even when a label or a
/// pin sits exactly on the crossing; the drawing is ambiguous and says so.
#[test]
fn a_label_or_pin_on_an_unmarked_crossing_joins_neither_wire_and_blocks() {
    let crossing = || SchematicDocument {
        wires: vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, -100), Point::new(0, 100)),
        ],
        ..SchematicDocument::default()
    };
    let mut labelled = crossing();
    labelled
        .net_labels
        .push(NetLabel::new(3, Point::origin(), "OUT"));
    let mut pinned = crossing();
    // A resistor at (20, 0) has a pin at the origin.
    pinned.components.push(resistor(3, Point::new(20, 0)));

    for (name, document) in [("label", labelled), ("pin", pinned)] {
        let extracted = extracted(&document);
        let first = extracted.net_of_wire(1).map(|net| net.id);
        let second = extracted.net_of_wire(2).map(|net| net.id);
        assert!(first.is_some() && second.is_some(), "{name}");
        assert_ne!(first, second, "{name}: the crossing was merged");
        let ambiguous = ambiguous_crossings(&extracted);
        assert_eq!(ambiguous.len(), 1, "{name}: one finding per crossing");
        assert!(ambiguous[0].blocking, "{name}");
        assert_eq!(
            ambiguous[0].anchor,
            ConnectivityAnchor::Point(Point::origin()),
            "{name}"
        );
        assert!(
            !extracted
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.kind == ConnectivityDiagnosticKind::OrphanNetLabel),
            "{name}: the crossing finding says it all"
        );
    }
}

/// The same crossing on a 45-degree wire.
#[test]
fn a_label_on_an_unmarked_diagonal_crossing_joins_neither_wire() {
    let document = SchematicDocument {
        wires: vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(-40, -40), Point::new(40, 40)),
        ],
        net_labels: vec![NetLabel::new(3, Point::origin(), "OUT")],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    assert_ne!(
        extracted.net_of_wire(1).map(|net| net.id),
        extracted.net_of_wire(2).map(|net| net.id)
    );
    assert_eq!(ambiguous_crossings(&extracted).len(), 1);
}

/// A junction resolves the ambiguity: everything at the crossing is one net.
#[test]
fn a_junction_makes_a_labelled_crossing_one_net() {
    let document = SchematicDocument {
        wires: vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, -100), Point::new(0, 100)),
        ],
        junctions: vec![Junction::new(3, Point::origin())],
        net_labels: vec![NetLabel::new(4, Point::origin(), "OUT")],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    assert_eq!(
        extracted.net_of_wire(1).map(|net| net.id),
        extracted.net_of_wire(2).map(|net| net.id)
    );
    assert!(ambiguous_crossings(&extracted).is_empty());
}

/// Two wires overlapping along one line share their interiors without
/// crossing; a pin there is no ambiguity.
#[test]
fn a_pin_on_overlapping_collinear_wires_is_not_a_crossing() {
    let document = SchematicDocument {
        wires: vec![
            Wire::segment(1, Point::new(-40, 0), Point::new(20, 0)),
            Wire::segment(2, Point::new(-20, 0), Point::new(40, 0)),
        ],
        components: vec![resistor(3, Point::new(20, 0))],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    assert!(ambiguous_crossings(&extracted).is_empty());
    assert_eq!(
        extracted.net_of_wire(1).map(|net| net.id),
        extracted.net_of_wire(2).map(|net| net.id)
    );
}

/// A label spelled as a bus member names that member's bit, whether or not
/// any tap anywhere selects it: the array tool writes exactly these labels.
#[test]
fn a_label_spelled_as_a_bus_member_joins_the_bit_without_a_tap() {
    let document = SchematicDocument {
        buses: vec![
            Bus::new(
                1,
                vec![Point::origin(), Point::new(100, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").expect("declaration")),
            )
            .expect("bus"),
        ],
        wires: vec![Wire::segment(2, Point::new(0, 40), Point::new(40, 40))],
        net_labels: vec![NetLabel::new(3, Point::new(20, 40), "DATA[3]")],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    let labelled = extracted.net_at(Point::new(20, 40)).expect("the label's net");
    assert_eq!(labelled.spice_name(), "DATA#3");
    assert_eq!(
        extracted
            .nets
            .iter()
            .filter(|net| display_net_name(&document, &net.spice_name()) == "DATA[3]")
            .count(),
        1,
        "one net shows as DATA[3]"
    );
}

/// Every spelling the deck folds onto node 0 is one extracted net with the
/// ground symbol, so the design has the node count the deck has.
#[test]
fn every_ground_spelling_is_one_net_with_the_ground_symbol() {
    let ground = Component::new(10, ComponentType::Ground, Point::new(200, 0));
    let ground_pin = ground.terminal_positions_resolved(None)[0].1;
    let document = SchematicDocument {
        components: vec![ground],
        wires: vec![
            Wire::segment(1, Point::new(0, 0), Point::new(40, 0)),
            Wire::segment(2, Point::new(0, 40), Point::new(40, 40)),
            Wire::segment(3, Point::new(0, 80), Point::new(40, 80)),
            Wire::segment(4, ground_pin, Point::new(ground_pin.x + 40, ground_pin.y)),
        ],
        net_labels: vec![
            NetLabel::new(5, Point::new(20, 0), "gnd"),
            NetLabel::new(6, Point::new(20, 40), "ground"),
            NetLabel::new(7, Point::new(20, 80), "GND!"),
        ],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    let ground_net = extracted.ground_net.expect("the ground symbol binds node 0");
    for wire in 1..=4 {
        assert_eq!(
            extracted.net_of_wire(wire).map(|net| net.id),
            Some(ground_net),
            "wire {wire}"
        );
    }
    assert_eq!(
        extracted
            .nets
            .iter()
            .find(|net| net.id == ground_net)
            .map(Net::spice_name)
            .as_deref(),
        Some("0")
    );
}

/// `GND!` is a ground alias every dialect folds onto node 0, so a net that
/// carries it is written as `0` even with no ground symbol in the drawing.
#[test]
fn a_global_ground_label_is_node_zero() {
    let document = SchematicDocument {
        wires: vec![Wire::segment(1, Point::new(0, 0), Point::new(40, 0))],
        net_labels: vec![NetLabel::new(2, Point::new(20, 0), "GND!")],
        ..SchematicDocument::default()
    };

    let extracted = extracted(&document);

    assert_eq!(
        extracted.net_of_wire(1).map(Net::spice_name).as_deref(),
        Some("0")
    );
}
