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
    (specification.rules, specification.disciplines)
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

/// A logic-only library does not select a four-state body for a real net.
#[test]
fn a_logic_only_library_leaves_real_conversion_policy_unchanged() {
    let (table, db) = table(&builtin_connect_library_source());
    for kind in [Kind::RealToV, Kind::VToReal] {
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

fn law(
    kind: Kind,
    source: &str,
    vcc: crate::Value,
) -> Result<ConverterLaw, crate::SimulationError> {
    let mut selected = select(kind, source);
    selected.execution = Some(std::sync::Arc::new(
        super::super::connect_execution::ConnectExecution::Delegated(BuiltinSignature::Full),
    ));
    let level = selected.stated_supply().unwrap_or(vcc);
    converter_law(
        Some(&selected),
        kind,
        ConverterSupply::new(level, None, ParameterSource::Default(vcc)),
    )
}

fn delegated(kind: Kind, source: &str, vcc: crate::Value) -> Vec<(String, crate::Value)> {
    law(kind, source, vcc)
        .unwrap_or_else(|error| panic!("delegates: {error}"))
        .model_parameters()
}

/// With no section 7.7.3 override the converter takes the shipped
/// declaration's defaults, the levels derived from the deck's supply.
#[test]
fn an_unparameterized_connect_module_stamps_the_declarations_defaults() {
    let source = builtin_connect_library_source();
    let vcc = 3.3;
    let half = vcc / 2.0;
    let named = |values: &[(&str, f64)]| -> Vec<(String, f64)> {
        values
            .iter()
            .map(|(name, value)| (name.to_string(), *value))
            .collect()
    };
    assert_eq!(
        delegated(Kind::Adc, &source, vcc),
        named(&[
            ("vsupmin", 0.0),
            ("vthi", half),
            ("vtlo", half),
            ("tdrise", 0.0),
            ("tdfall", 0.0),
        ])
    );
    assert_eq!(
        delegated(Kind::Dac, &source, vcc),
        named(&[
            ("vsupmin", 0.0),
            ("vlo", 0.0),
            ("vhi", vcc),
            ("rlo", 20.0),
            ("rhi", 20.0),
            ("rpull", 10e3),
            ("rweak", 100e3),
            ("trise", 0.0),
            ("tfall", 0.0),
            ("tdelay", 0.0),
        ])
    );
}

/// A default the converter derives from other parameters (`vx`, `vz`, `rx`,
/// `rsup`) is left to it: stamping one would be a second statement of what
/// the midpoint, or the stiffest strong drive, is.
#[test]
fn derived_defaults_are_left_to_the_converter() {
    let stamped = delegated(Kind::Dac, &builtin_connect_library_source(), 5.0);
    for derived in ["vx", "vz", "rx", "rsup"] {
        assert!(
            !stamped.iter().any(|(name, _)| name == derived),
            "{derived} must be derived, not restated: {stamped:?}"
        );
    }
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

    let value = |kind, name: &str| {
        delegated(kind, &source, 3.3)
            .into_iter()
            .find(|(parameter, _)| parameter == name)
            .map(|(_, value)| value)
    };
    assert_eq!(
        value(Kind::Adc, "vthi"),
        Some(0.9),
        "the deck's vcc is overridden by the connect statement's vsup"
    );
    assert_eq!(value(Kind::Adc, "vtlo"), Some(0.9));
    assert_eq!(value(Kind::Dac, "vhi"), Some(1.8));
    assert_eq!(value(Kind::Dac, "trise"), Some(2e-9));
    assert_eq!(value(Kind::Dac, "tfall"), Some(3e-9));
}

/// Without an override the supply is the deck's, because a node's supply is a
/// property of the deck.
#[test]
fn the_supply_defaults_to_the_decks() {
    let law = law(Kind::Adc, &builtin_connect_library_source(), 5.0).expect("resolves");
    assert_eq!(law.get("vthi").map(|parameter| parameter.value), Some(2.5));
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
    let error = law(Kind::Adc, &source, 3.3).expect_err("vlo is refused");
    let error = format!("{error}");
    assert!(error.contains("'vlo'"), "unexpected: {error}");
    assert!(
        error.contains("vthi, vtlo, txdel"),
        "lists the accepted set: {error}"
    );
}

/// Authored declarations retain their source instead of a name-based fallback.
#[test]
fn an_authored_connect_module_selects_its_executable_body() {
    let source = "connectmodule my_a2d(a,d); input a; output d; electrical a; logic d; endmodule\nconnectrules mine; connect my_a2d; endconnectrules";
    let mut design = DesignConnectRules::default();
    design
        .register(
            std::path::Path::new("library.va"),
            VerilogACompiler::default()
                .connect_specification_from_preprocessed(source)
                .unwrap(),
        )
        .unwrap();
    design.finish_selection().unwrap();
    let selected = design
        .select_for_boundary_node(Kind::Adc, "din", "a1", "in")
        .unwrap()
        .unwrap();
    assert!(matches!(
        selected.execution.as_deref(),
        Some(super::super::connect_execution::ConnectExecution::Authored(
            _
        ))
    ));
    check_execution(&selected, Kind::Adc, "din", "a1").unwrap();
}

/// A design whose rules do not cover the boundary is an error naming the net
/// and both disciplines, which is the diagnostic section 7.8.4 asks for, and
/// the instance whose port the boundary is, which is where it is shown.
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
    assert_eq!(error.instance_name(), Some("a1"));
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
    connect a2d #(.tdrise(delay));
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

/// `vsup` is the one parameter a bare identifier may give: it names the deck
/// net the boundary's supply binds to, and leaves the numbers to fold.
#[test]
fn a_supply_named_as_a_net_is_carried_beside_the_numbers() {
    let mut source = library_modules();
    source.push_str(
        "\
connectrules net_supply;
    connect a2d #(.vsup(vdd), .tdrise(2e-9));
endconnectrules
",
    );
    let (table, db) = table(&source);
    let selected = select_for_boundary(&table, &db, Kind::Adc, "din", "a1", "in")
        .expect("a net-named supply selects")
        .expect("the a2d rule applies");
    assert_eq!(selected.supply_net(), Some("vdd"));
    assert!(!selected.states_supply(), "a net is not a stated level");
    assert_eq!(selected.parameter("tdrise"), Some(2e-9));
}
