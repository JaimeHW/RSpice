//! Publication contract cases independent of the editor surface.

use super::*;
use crate::symbol::SymbolPin;
use rspice_design_model::Point;

/// The row used to state a count nothing computed. Every row now answers
/// from the document, and the off-grid row answers from the same finding the
/// symbol check publishes.
#[test]
fn save_checks_compute_the_off_grid_row_from_the_document() {
    let ports = [
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(43, 0))),
        ],
        ..SymbolDocument::default()
    };

    let checks = symbol_save_checks(None, "amp", &document, &ports);
    let labels: Vec<&str> = checks.iter().map(|check| check.label).collect();

    assert!(
        !labels.contains(&"Hidden power pins"),
        "a row that states a count nothing computes is not a check: {labels:?}"
    );
    let off_grid = checks
        .iter()
        .find(|check| check.label == "Off-grid terminals")
        .expect("the off-grid row is computed");
    assert_eq!(off_grid.observed, "1");
    assert!(!off_grid.passed);
    assert!(off_grid.refusal().contains("off-grid terminals"));
}

#[test]
fn save_checks_pass_a_symbol_that_matches_its_interface() {
    let ports = [PortSpec {
        name: "IN".to_owned(),
        direction: PortDirection::In,
    }];
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-40, 0)),
        )],
        ..SymbolDocument::default()
    };

    let checks = symbol_save_checks(None, "amp", &document, &ports);

    assert!(
        checks.iter().all(|check| check.passed),
        "{:?}",
        checks
            .iter()
            .filter(|check| !check.passed)
            .map(|check| check.label)
            .collect::<Vec<_>>()
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

    let ports = [PortSpec {
        name: "OUT".to_owned(),
        direction: PortDirection::Out,
    }];
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
        let mut result = check_symbol_pins(document, &ports, reference);
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
    let found = check_symbol_pins(&drawn, &ports, &original);
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
