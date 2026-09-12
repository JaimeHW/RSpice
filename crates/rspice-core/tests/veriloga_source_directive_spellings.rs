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

/// A Verilog-A source names a global instance master, exactly as Spectre's
/// `ahdl_include` does, so writing one inside a `.SUBCKT` body does not scope
/// it to that subcircuit. The scope stays global — a deck that relies on it
/// keeps working — and the deck is told once that the placement does not mean
/// what it looks like.
#[test]
fn a_directive_inside_a_subcircuit_is_hoisted_with_one_warning() {
    let deck = "* a source directive written inside a subcircuit body\n\
                X1 in 0 wrapper\n\
                .subckt wrapper a b\n\
                .va \"d.va\" dres\n\
                XA a b dres\n\
                .ends\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("the deck parses");
    assert_eq!(
        netlist.veriloga_includes.len(),
        1,
        "the directive keeps its global scope"
    );
    let hoisted: Vec<&str> = netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "veriloga-directive-hoisted")
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();
    assert_eq!(hoisted.len(), 1, "expected one warning, got {hoisted:?}");
    assert!(
        hoisted[0].contains("wrapper") && hoisted[0].contains("line 4"),
        "the warning names neither the subcircuit nor the line: {}",
        hoisted[0]
    );
}

/// At the top level there is nothing to hoist, so nothing is said.
#[test]
fn a_top_level_directive_warns_about_nothing() {
    let deck = "* a source directive at the top level\n\
                X1 in 0 dres\n\
                .va \"d.va\" dres\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("the deck parses");
    assert!(
        netlist
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "veriloga-directive-hoisted"),
        "a top-level directive is not hoisted: {:?}",
        netlist.diagnostics
    );
}

/// The directive is consumed whole and leaves no card open, so a `+` line
/// after it continues nothing. It used to open a fresh logical line out of the
/// continuation's own text, and the deck silently gained a card nobody wrote.
#[test]
fn a_continuation_after_a_directive_is_a_syntax_error() {
    let deck = "* a continuation line with nothing to continue\n\
                X1 in 0 dres\n\
                .va \"d.va\" dres\n\
                + r=2000\n\
                .end\n";
    let error = Netlist::parse(deck).expect_err("the orphaned continuation is refused");
    let rendered = error.to_string();
    assert!(
        rendered.contains("line 4") && rendered.to_lowercase().contains("continuation"),
        "the refusal does not name the orphaned line: {rendered}"
    );
}

/// A card inside a false `.IF` branch does not exist, and a source directive
/// is no different: it used to be collected before the conditional machinery
/// ever saw it, so a deck that switched a master off still bound it.
#[test]
fn a_veriloga_directive_inside_a_false_if_branch_is_not_collected() {
    let deck = "* a source directive the deck switched off\n\
                X1 in 0 dres\n\
                .if 0\n\
                .va \"d.va\" dres\n\
                .endif\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("the deck parses");
    assert!(
        netlist.veriloga_includes.is_empty(),
        "a directive in a false branch was collected anyway: {:?}",
        netlist.veriloga_includes
    );
}

#[test]
fn the_taken_branch_of_a_conditional_still_collects_its_directive() {
    for (deck, taken) in [
        (
            "* the true branch collects\n\
             X1 in 0 dres\n\
             .if 1\n\
             .va \"true.va\" dres\n\
             .else\n\
             .va \"false.va\" dres\n\
             .endif\n\
             .end\n",
            "true.va",
        ),
        (
            "* the else branch collects\n\
             X1 in 0 dres\n\
             .if 0\n\
             .va \"true.va\" dres\n\
             .else\n\
             .va \"false.va\" dres\n\
             .endif\n\
             .end\n",
            "false.va",
        ),
    ] {
        let netlist = Netlist::parse(deck).expect("the deck parses");
        let collected: Vec<String> = netlist
            .veriloga_includes
            .iter()
            .map(|include| include.file_path.display().to_string())
            .collect();
        assert_eq!(collected.len(), 1, "expected exactly one: {collected:?}");
        assert!(
            collected[0].ends_with(taken),
            "the branch that runs collected {collected:?} instead of {taken}"
        );
    }
}
