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
