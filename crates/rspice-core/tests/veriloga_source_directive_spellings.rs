//! The spellings that name a Verilog-A source.
//!
//! `.va` and `.veriloga` were the only two the parser recognized. Decks
//! written for other tools spell the same directive `.hdl`, `.vams` or
//! `.verilog`; all five name one directive, reach one resolver and one
//! compiled model. Nothing outside that set is a source directive.

#![cfg(feature = "veriloga")]

#[path = "common/veriloga_source_fixture.rs"]
mod fixture;

use fixture::{DIVIDER_VALUES, Fixture, HALF_DIVIDER_OHMS, divider_deck, divider_out};
use rspice_core::netlist::parse_veriloga_source_directive;
use rspice_core::{Engine, Netlist};

/// Every spelling the parser accepts, in the canonical lower case decks use.
const ACCEPTED: [&str; 5] = [".va", ".veriloga", ".hdl", ".vams", ".verilog"];

#[test]
fn every_accepted_spelling_names_the_same_deck_relative_source() {
    for spelling in ACCEPTED {
        let fixture = Fixture::new("spelling");
        fixture.write("sub/d.va", &fixture::resistor_module(HALF_DIVIDER_OHMS));
        fixture.write("sub/vals.cir", DIVIDER_VALUES);
        let deck = fixture.write(
            "sub/deck.cir",
            &divider_deck(&format!("{spelling} \"d.va\" dres")),
        );

        let netlist = Netlist::parse_file(&deck)
            .unwrap_or_else(|error| panic!("{spelling} must parse: {error}"));
        assert_eq!(
            netlist.veriloga_includes.len(),
            1,
            "{spelling} must produce exactly one Verilog-A source"
        );
        let result = Engine::default()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{spelling} must solve: {error}"));
        let out = result
            .try_voltage_named("out")
            .unwrap_or_else(|| panic!("{spelling} produced no node 'out'"));
        assert!(
            (out - divider_out(HALF_DIVIDER_OHMS)).abs() < 1e-9,
            "{spelling} must read the model beside the deck, got {out}"
        );
    }
}

#[test]
fn every_accepted_spelling_is_case_insensitive_like_every_other_directive() {
    for spelling in [".VA", ".VerilogA", ".HDL", ".Vams", ".VERILOG"] {
        assert!(
            parse_veriloga_source_directive(&format!("{spelling} \"d.va\" dres")).is_some(),
            "{spelling} must name a Verilog-A source"
        );
    }
}

#[test]
fn a_spelling_outside_the_accepted_set_is_not_a_source_directive() {
    for spelling in [".v", ".vam", ".vamsx", ".hdlx", ".verilogams", ".vaa"] {
        assert!(
            parse_veriloga_source_directive(&format!("{spelling} \"d.va\" dres")).is_none(),
            "{spelling} must not name a Verilog-A source"
        );
    }
}

#[test]
fn an_unknown_spelling_contributes_no_source_to_the_deck() {
    let deck = "* an unknown spelling is not a source directive\n\
                V1 in 0 1\n\
                R1 in out 1k\n\
                .vamsx \"d.va\" dres\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("the deck still parses");
    assert!(
        netlist.veriloga_includes.is_empty(),
        "an unknown spelling must not import a Verilog-A source"
    );
}
