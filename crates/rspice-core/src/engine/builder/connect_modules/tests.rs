//! The engine's half of the delegation contract.
//!
//! The other half is `rspice_veriloga::connect::library`'s parameter pin. The
//! two exist so that a rename on either side fails a test rather than
//! producing a bridge with a threshold nobody asked for.

use super::*;
use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::connect::library::builtin_connect_library_source;

use super::super::XspiceAutoBridgeKind as Kind;

fn table(source: &str) -> (ConnectRuleTable, DisciplineDb) {
    let specification = VerilogACompiler::default()
        .connect_specification_from_preprocessed(source)
        .unwrap_or_else(|error| panic!("connect rules read: {error}"));
    (specification.rules, DisciplineDb::with_standard())
}

fn select(kind: Kind, source: &str) -> PlannedConnectModule {
    let (table, db) = table(source);
    select_for_boundary(&table, &db, kind, "din", "a_inv", "in")
        .unwrap_or_else(|error| panic!("selects: {error}"))
        .expect("a connect module is selected")
}

/// The library selects for all three boundary kinds, and the direction clause
/// 7 derives from the port matches the one the bridge planner derived from the
/// port types — [`select_for_boundary`] refuses if they ever disagree.
#[test]
fn the_built_in_library_selects_for_every_boundary_kind() {
    let source = builtin_connect_library_source();
    for (kind, expected) in [
        (Kind::Adc, "a2d"),
        (Kind::Dac, "d2a"),
        (Kind::Bidi, "bidir"),
    ] {
        let selected = select(kind, &source);
        assert_eq!(selected.name, expected, "{expected}");
        assert_eq!(
            selected.name,
            expected_library_module(kind).expect("a library module"),
            "the delegation and the selection name the same module"
        );
    }
}

/// Section 7.8.5's `merged` name: the signal, the module, and the bottom
/// discipline, separated by double underscores.
#[test]
fn the_generated_instance_name_is_section_7_8_5s() {
    let selected = select(Kind::Adc, &builtin_connect_library_source());
    assert_eq!(selected.instance, "din__a2d__logic");
}

/// Real-valued event traffic is not a discipline boundary, so clause 7 is not
/// consulted for it at all.
#[test]
fn a_real_event_boundary_selects_nothing() {
    let (table, db) = table(&builtin_connect_library_source());
    for kind in [Kind::RealToV, Kind::VToReal] {
        assert!(boundary_direction(kind).is_none());
        assert!(
            select_for_boundary(&table, &db, kind, "w", "a1", "r")
                .expect("no error")
                .is_none()
        );
    }
}

// ---------------------------------------------------------------------------
// The delegation
// ---------------------------------------------------------------------------

fn delegated(kind: Kind, source: &str, vcc: crate::Value) -> Vec<(String, crate::Value)> {
    let selected = select(kind, source);
    delegated_parameters(&selected, kind, vcc).unwrap_or_else(|error| panic!("delegates: {error}"))
}

/// With no section 7.7.3 override the delegation stamps exactly what
/// `add_planned_xspice_auto_bridge` stamps without a connect module: the same
/// parameters, the same values, derived the same way from the same supply.
///
/// This is the equivalence that lets the connect-module route land without
/// moving a single existing number. The literals are copied from that
/// function's own arms deliberately — if either moves, this fails.
#[test]
fn an_unparameterized_connect_module_stamps_the_auto_bridges_own_numbers() {
    let source = builtin_connect_library_source();
    let vcc = 3.3;
    let half = vcc / 2.0;

    assert_eq!(
        delegated(Kind::Adc, &source, vcc),
        vec![("in_low".to_string(), half), ("in_high".to_string(), half)]
    );
    assert_eq!(
        delegated(Kind::Dac, &source, vcc),
        vec![("out_low".to_string(), 0.0), ("out_high".to_string(), vcc)]
    );
    assert_eq!(
        delegated(Kind::Bidi, &source, vcc),
        vec![
            ("out_high".to_string(), vcc),
            ("in_low".to_string(), half),
            ("in_high".to_string(), half),
        ]
    );
}

/// `dac_bridge` reads `out_undef` as the midpoint of the two levels exactly
/// when they are given and it is not. The delegation therefore must not stamp
/// it: leaving it out is how the midpoint is obtained, and stamping one would
/// be a second statement of what half is.
#[test]
fn the_undefined_level_is_left_to_the_code_model() {
    let stamped = delegated(Kind::Dac, &builtin_connect_library_source(), 5.0);
    assert!(
        !stamped
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("out_undef")),
        "out_undef must be delegated, not restated: {stamped:?}"
    );
}

const SUPPLIED: &str = "\
connectrules supplied;
    connect a2d #(.vsup(1.8));
    connect d2a #(.vsup(1.8), .trise(2e-9), .tfall(3e-9));
endconnectrules
";

/// The three built-in modules with no `connectrules` block, so a test can
/// supply its own without making section 7.8.4's rule 3 ambiguous.
fn library_modules() -> String {
    let mut source = String::new();
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source
}

/// Section 7.7.3's parameters reach the code model, and the supply moves every
/// level derived from it.
#[test]
fn a_supplied_connect_module_moves_every_derived_level() {
    // Two rules for one direction pair would be ambiguous under section
    // 7.8.4's rule 3, so the supplied block stands alone.
    let source = format!("{}{SUPPLIED}", library_modules());

    assert_eq!(
        delegated(Kind::Adc, &source, 3.3),
        vec![("in_low".to_string(), 0.9), ("in_high".to_string(), 0.9)],
        "the deck's vcc is overridden by the connect statement's vsup"
    );
    assert_eq!(
        delegated(Kind::Dac, &source, 3.3),
        vec![
            ("out_low".to_string(), 0.0),
            ("out_high".to_string(), 1.8),
            ("t_rise".to_string(), 2e-9),
            ("t_fall".to_string(), 3e-9),
        ]
    );
}

/// Without an override the supply is the deck's, because a node's supply is a
/// property of the deck.
#[test]
fn the_supply_defaults_to_the_decks() {
    assert_eq!(
        delegated(Kind::Adc, &builtin_connect_library_source(), 5.0),
        vec![("in_low".to_string(), 2.5), ("in_high".to_string(), 2.5)]
    );
}

/// A parameter the delegation cannot carry is refused rather than dropped: a
/// knob a deck sets and nothing hears is worse than an error.
#[test]
fn an_uncarried_parameter_is_refused() {
    let mut source = library_modules();
    source.push_str(
        "\
connectrules unknown_parameter;
    connect a2d #(.tdrise(2e-9), .vlo(0.4));
endconnectrules
",
    );
    let selected = select(Kind::Adc, &source);
    let error = delegated_parameters(&selected, Kind::Adc, 3.3).expect_err("vlo is refused");
    assert!(format!("{error}").contains("vlo"), "unexpected: {error}");
}

/// A connect module outside the library has a body only a Verilog-AMS mixed
/// host could run, and the one this crate has is not wired to the engine.
#[test]
fn a_connect_module_outside_the_library_is_refused_with_the_blocker() {
    let source = "\
connectmodule my_a2d(a, d);
    input a;
    output d;
    electrical a;
    logic d;
endmodule
connectrules mine;
    connect my_a2d;
endconnectrules
";
    let selected = select(Kind::Adc, source);
    assert_eq!(selected.name, "my_a2d");
    let error = check_delegable(&selected, Kind::Adc, "din").expect_err("refused");
    let error = format!("{error}");
    assert!(error.contains("my_a2d"), "names the module: {error}");
    assert!(
        error.contains("integer-nanosecond grid"),
        "names the blocker: {error}"
    );
}

/// A design whose rules do not cover the boundary is an error naming the net
/// and both disciplines, which is the diagnostic section 7.8.4 asks for.
#[test]
fn an_uncovered_boundary_names_the_net_and_both_disciplines() {
    let source = "\
connectmodule d2a(d, a);
    input d;
    output a;
    logic d;
    electrical a;
endmodule
connectrules only_d2a;
    connect d2a;
endconnectrules
";
    let (table, db) = table(source);
    let error = select_for_boundary(&table, &db, Kind::Adc, "din", "a1", "in")
        .expect_err("no analog-to-discrete rule");
    let error = format!("{error}");
    assert!(error.contains("din"), "names the net: {error}");
    assert!(error.contains("electrical"), "{error}");
    assert!(error.contains("logic"), "{error}");
}

/// Section 7.7.3 binds by name, and a value written where no design scope
/// exists has to be a literal.
#[test]
fn a_connect_parameter_that_cannot_be_folded_is_refused() {
    let mut source = library_modules();
    source.push_str(
        "\
connectrules folded;
    connect a2d #(.vsup(vdd));
endconnectrules
",
    );
    let (table, db) = table(&source);
    let error = select_for_boundary(&table, &db, Kind::Adc, "din", "a1", "in")
        .expect_err("an identifier is refused");
    assert!(
        format!("{error}").contains("numeric literal"),
        "unexpected: {error}"
    );
}
