//! Symbol pin check cases independent of the editor surface.

use super::*;
use crate::symbol::{PinSummary, SymbolPin};
use rspice_design_model::{Point, port::PortDirection};

fn port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
    }
}

fn pin(name: &str, direction: PortDirection, at: (i32, i32)) -> SymbolPin {
    SymbolPin::new(name, direction, Some(Point::new(at.0, at.1)))
}

fn symbol(pins: Vec<SymbolPin>) -> SymbolDocument {
    SymbolDocument {
        pins,
        ..SymbolDocument::default()
    }
}

/// Every finding's rule and the object it is about.
fn findings(
    document: &SymbolDocument,
    interface: &SymbolInterface,
) -> Vec<(DrcViolationType, String)> {
    let reference = CellViewRef::new("work", "amp", "symbol");
    check_symbol_pins(document, interface, None, &reference)
        .violations()
        .iter()
        .map(|finding| (finding.violation_type, finding.location.display()))
        .collect()
}

/// A symbol drawn before any schematic defines the cell's interface: none of
/// its pins is reported missing from anything.
#[test]
fn a_symbol_with_no_schematic_defines_the_interface() {
    let document = symbol(vec![
        pin("IN", PortDirection::In, (-40, 0)),
        pin("OUT", PortDirection::Out, (40, 0)),
    ]);

    assert!(findings(&document, &SymbolInterface::SymbolDefined).is_empty());
    assert!(
        document
            .pin_summary(&SymbolInterface::SymbolDefined)
            .matches()
    );
}

/// A schematic that declares no pins is a different statement from no
/// schematic at all, and every pin the symbol carries is outside it.
#[test]
fn a_schematic_that_declares_no_pins_is_said_so() {
    let document = symbol(vec![pin("IN", PortDirection::In, (-40, 0))]);
    let empty = SymbolInterface::Declared(Vec::new());

    assert_eq!(
        document.pin_summary(&empty),
        PinSummary {
            interface_empty: true,
            ..PinSummary::default()
        }
    );
    let found = findings(&document, &empty);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].0, DrcViolationType::SymbolOrphanedPin);
    assert_eq!(
        DrcViolationType::SymbolOrphanedPin.default_severity(),
        crate::drc::DrcSeverity::Warning,
        "a pin the schematic does not declare yet is a warning"
    );
    assert!(
        symbol(Vec::new()).pin_summary(&empty).matches(),
        "an empty symbol and an empty schematic agree"
    );
}

/// The counts the canvas states are exact: pins missing from the symbol,
/// pins the schematic does not declare, and pins without a terminal.
#[test]
fn a_mismatch_is_counted_by_kind() {
    let interface = SymbolInterface::Declared(vec![
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
        port("EN", PortDirection::In),
    ]);
    let mut unplaced = pin("OUT", PortDirection::Out, (40, 0));
    unplaced.position = None;
    let document = symbol(vec![
        pin("IN", PortDirection::In, (-40, 0)),
        unplaced,
        pin("SPARE", PortDirection::In, (-40, 10)),
    ]);

    assert_eq!(
        document.pin_summary(&interface),
        PinSummary {
            interface_empty: false,
            missing: 1,
            unplaced: 1,
            orphaned: 1,
        }
    );
}

#[test]
fn a_symbol_that_matches_its_interface_has_no_findings() {
    let interface = SymbolInterface::Declared(vec![port("IN", PortDirection::In)]);
    let document = symbol(vec![pin("IN", PortDirection::In, (-40, 0))]);

    assert!(findings(&document, &interface).is_empty());
}

/// The terminal grid is checked whatever the interface is.
#[test]
fn an_off_grid_terminal_is_found_against_any_interface() {
    let document = symbol(vec![pin("OUT", PortDirection::Out, (43, 0))]);

    for interface in [
        SymbolInterface::SymbolDefined,
        SymbolInterface::Declared(vec![port("OUT", PortDirection::Out)]),
    ] {
        let found = findings(&document, &interface);
        assert_eq!(found.len(), 1, "{interface:?}: {found:?}");
        assert_eq!(found[0].0, DrcViolationType::SymbolPinOffGrid);
    }
}

/// A module declares its rails `inout`, and the generated symbol draws them
/// as the rails they are. That is no finding.
#[test]
fn a_rail_drawn_on_a_bidirectional_port_is_no_finding() {
    let interface = SymbolInterface::Declared(vec![port("vdd", PortDirection::InOut)]);
    let document = symbol(vec![pin("vdd", PortDirection::Supply, (0, -40))]);

    assert!(findings(&document, &interface).is_empty());
}

/// A pin turned against its port is named.
#[test]
fn a_pin_turned_against_its_port_is_named() {
    let interface = SymbolInterface::Declared(vec![port("IN", PortDirection::In)]);
    let document = symbol(vec![pin("IN", PortDirection::Out, (-40, 0))]);

    let found = findings(&document, &interface);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].0, DrcViolationType::SymbolPinDirection);
    assert!(found[0].1.contains("IN"), "{found:?}");
}

/// The same pins in another order is an order finding, not missing pins.
#[test]
fn the_interface_pins_in_another_order_are_an_order_finding() {
    let interface = SymbolInterface::Declared(vec![
        port("IN", PortDirection::In),
        port("OUT", PortDirection::Out),
    ]);
    let document = symbol(vec![
        pin("OUT", PortDirection::Out, (40, 0)),
        pin("IN", PortDirection::In, (-40, 0)),
    ]);

    let reference = CellViewRef::new("work", "amp", "symbol");
    let result = check_symbol_pins(&document, &interface, None, &reference);
    let found = result.violations();
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].violation_type, DrcViolationType::SymbolPinOrder);
    assert!(
        found[0].message.ends_with("expected IN OUT, found OUT IN"),
        "{}",
        found[0].message
    );
}

/// A symbol-pin waiver is stored inside its symbol, so it names the pin
/// alone: a copy of the cell carries the symbol, waiver and all, and the
/// waiver still covers the copy's pin. One written by an earlier build names
/// its symbol too. It still covers the pin of the symbol that stores it, and
/// in a copy it covers nothing and says so.
#[test]
fn a_symbol_pin_waiver_travels_with_its_symbol() {
    use crate::drc::{CheckWaivers, DrcWaiver};

    let interface = SymbolInterface::Declared(vec![port("OUT", PortDirection::Out)]);
    let drawn = SymbolDocument {
        pins: vec![SymbolPin::new(
            "OUT",
            PortDirection::Out,
            Some(Point::new(43, 0)),
        )],
        ..SymbolDocument::default()
    };
    let original = CellViewRef::new("work", "amp", "symbol");
    let copy = CellViewRef::new("work", "amp_copy", "symbol");
    let waived = |object: String| {
        let mut document = drawn.clone();
        document
            .waivers
            .waive(DrcWaiver {
                rule: DrcViolationType::SymbolPinOffGrid.rule_code().to_owned(),
                object,
                reason: "matches the vendor footprint".to_owned(),
                author: "ana".to_owned(),
                waived_at_ms: 1_760_000_000_000,
            })
            .expect("an off-grid pin may be waived");
        document
    };
    // Whether each finding is waived, and how many waivers cover nothing.
    let checked = |document: &SymbolDocument, reference: &CellViewRef| {
        let mut result = check_symbol_pins(document, &interface, None, reference);
        let waivers = CheckWaivers::for_subject(&document.waivers);
        waivers.apply(&mut result);
        let waived = result
            .violations()
            .iter()
            .map(|finding| finding.waiver.is_some())
            .collect::<Vec<_>>();
        (waived, waivers.unmatched_in(&result).len())
    };

    // Recorded as the page records it, against the original's finding.
    let found = check_symbol_pins(&drawn, &interface, None, &original);
    let object = crate::drc::waiver_site(&found.violations()[0].location).1;
    let document = waived(object);
    assert_eq!(checked(&document, &original), (vec![true], 0));
    assert_eq!(
        checked(&document, &copy),
        (vec![true], 0),
        "the copy's waiver covers its pin"
    );

    let earlier = waived(format!("symbol-pin:{}:OUT", original.key()));
    assert_eq!(checked(&earlier, &original), (vec![true], 0));
    assert_eq!(checked(&earlier, &copy), (vec![false], 1));
}
