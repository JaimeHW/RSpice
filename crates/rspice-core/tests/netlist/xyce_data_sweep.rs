//! Xyce `.DATA` sweep expansion, checked all the way to a solved row.
//!
//! The expansion itself is unit-tested inside `netlist::multi_run`. This case
//! lives out here because its claim is end-to-end: every row the continuation
//! header produces is a standalone deck that the engine solves to the divider
//! voltage its own resistor values imply. Asserting that from inside the
//! parser would mean the deck-reading layer naming the engine.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::multi_run::try_expand_multi_run;
use rspice_core::netlist::{AnalysisCommand, ElementKind, Netlist};

#[test]
fn xyce_continuation_header_expands_rows_and_owns_the_dc_analysis() {
    let source = "Xyce continuation-header data\n\
        V1 4 0 10\n\
        R1 4 5 10\n\
        R2 5 0 5\n\
        .data test\n\
        + r1 r2\n\
        + 8 4\n\
        * comments do not become table values\n\
        + 9 5 ; inline comments do not become table values\n\
        .enddata\n\
        .dc data=test\n\
        .dc V1 10 15 1\n\
        .print dc {R1:R} {R2:R} V(4) V(5)\n\
        .end\n";

    let decks = try_expand_multi_run(source).expect("Xyce .DATA form expands");
    assert_eq!(decks.len(), 2);
    assert_eq!(decks[0].label.as_deref(), Some("test row 1"));
    for (deck, expected_r1, expected_r2) in
        [(&decks[0], 8.0_f64, 4.0_f64), (&decks[1], 9.0_f64, 5.0_f64)]
    {
        assert!(!deck.source.to_ascii_lowercase().contains("data=test"));
        assert!(!deck.source.contains(".dc V1 10 15 1"));
        let parsed = Netlist::parse(&deck.source).expect("each expanded row is a standalone deck");
        assert_eq!(parsed.analyses.len(), 1);
        assert!(matches!(parsed.analyses.first(), Some(AnalysisCommand::Op)));
        let resistance = |name: &str| {
            parsed
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .and_then(|element| match &element.kind {
                    ElementKind::Resistor { value, .. } => Some(*value),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing resistor {name}"))
        };
        assert_eq!(resistance("R1").to_bits(), expected_r1.to_bits());
        assert_eq!(resistance("R2").to_bits(), expected_r2.to_bits());

        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let result = engine
            .run_dc_op(&parsed)
            .expect("expanded passive DATA row solves natively");
        let actual = result.try_voltage_named("5").expect("V(5) retained");
        let expected = 10.0 * expected_r2 / (expected_r1 + expected_r2);
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "expanded DATA row produced V(5)={actual:.17e}, expected {expected:.17e}"
        );
    }
}
