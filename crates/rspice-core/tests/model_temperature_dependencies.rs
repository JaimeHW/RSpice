use rspice_core::{Engine, Netlist};

fn assert_resistance(declarations: &str, devices: &str, expected: f64) {
    let source =
        format!("* model temperature dependencies\nV1 out 0 1\n{declarations}\n{devices}\n.END\n");
    let netlist = Netlist::parse(&source).unwrap();
    let current = Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .branch_current_named("V1")
        .unwrap();
    let actual = -1.0 / current;
    assert!(
        (actual - expected).abs() < expected.abs() * 1e-9,
        "expected {expected}, got {actual}: {source}"
    );
}

#[test]
fn bare_signed_and_parenthesized_temperature_values_match_expressions() {
    for value in [
        "TEMP", "+TEMP", "+ TEMP", "(TEMP)", "( TEMP )", "{TEMP}", "TNOM", "+TNOM", "+ TNOM",
        "(TNOM)", "{TNOM}",
    ] {
        assert_resistance(
            &format!(".MODEL rm R(RSH={value} TNOM=47)"),
            "R1 out 0 rm L=1 W=1 TEMP=47",
            47.0,
        );
    }
}

#[test]
fn temperature_dependencies_follow_user_functions_without_capturing_formals() {
    for (functions, expression) in [
        (".FUNC thermal() {TEMP}", "thermal()"),
        (
            ".FUNC thermal() {TEMP}\n.FUNC outer() {thermal()}",
            "outer()",
        ),
        (".FUNC nominal() {TNOM}", "nominal()"),
        (".FUNC identity(temp) {temp}", "identity(47)"),
        (
            ".FUNC thermal() {TEMP}\n.FUNC outer(temp) {thermal()}",
            "outer(3)",
        ),
    ] {
        assert_resistance(
            &format!("{functions}\n.MODEL rm R(RSH={{{expression}}} TNOM=47)"),
            "R1 out 0 rm L=1 W=1 TEMP=47",
            47.0,
        );
    }
}

#[test]
fn negative_and_positional_temperature_values_are_deferred() {
    for card in [
        "RSH=-TEMP",
        "RSH=- TEMP",
        "RSH=-(TEMP)",
        "RSH -TEMP",
        "RSH - TEMP",
    ] {
        assert_resistance(
            &format!(".MODEL rm R({card})"),
            "R1 out 0 rm L=1 W=1 TEMP=-10",
            10.0,
        );
    }
    assert_resistance(".MODEL rm R(RSH TNOM TNOM 47)", "R1 out 0 rm L=1 W=1", 47.0);
}

#[test]
fn forward_function_definitions_keep_temperature_dependencies() {
    assert_resistance(
        ".MODEL rm R(RSH={thermal()})\n.FUNC thermal() {TEMP}",
        "R1 out 0 rm L=1 W=1 TEMP=47",
        47.0,
    );
}

#[test]
fn function_formals_named_temperature_do_not_make_a_static_card_dynamic() {
    let netlist = Netlist::parse(
        "* formal shadowing\n.FUNC identity(temp) {temp}\n.MODEL rm R(RSH={identity(47)})\n.END\n",
    )
    .unwrap();
    let model = &netlist.models[0];
    assert!(model.expr_params.is_empty());
    assert!(
        model
            .params
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("RSH") && *value == 47.0)
    );
}
