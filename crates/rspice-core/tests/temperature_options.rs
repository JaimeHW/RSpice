//! Temperature option expressions must agree with eagerly parsed values.
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::{Engine, Netlist};

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-30,
        "{actual} != {expected}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_temperature_options_reconcile_earlier_values_and_nominal_dependencies() {
    for expression_dialect in [
        rspice_core::config::ExpressionDialect::Ngspice,
        rspice_core::config::ExpressionDialect::Xyce,
    ] {
        let source = "Forward temperatures\n.param observed={TEMP}\nI1 0 out 1m\nR1 out 0 {TEMP}\n.options DEVICE temp={ambient+TNOM} tnom=+nominal\n.param ambient=50 nominal=35\n.end\n";
        let netlist = Netlist::parse_with_options(
            source,
            rspice_core::netlist::NetlistParseOptions {
                expression_dialect,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(netlist.options.temp, Some(85.0));
        assert_eq!(netlist.options.tnom, Some(35.0));
        close(eval_expression("observed", &netlist.params).unwrap(), 85.0);
        close(
            Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .try_voltage_named("out")
                .unwrap(),
            0.085,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_options_capture_known_bindings_and_local_functions() {
    let source = "Forward scope capture\n.param offset=5\n.func adjust(x) {x+offset}\n.options temp={adjust(ambient)} tnom=-nominal\n.param offset=90 ambient=80 nominal=10\n.func adjust(x) {x+100}\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(-10.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_options_preserve_complex_bindings_before_real_projection() {
    let netlist = Netlist::parse("Complex temperature dependency\n.param z={sqrt(-1)} expected={85+img(z)}\n.options temp={85+img(z)+ambient}\n.param z={sqrt(-4)} ambient=0\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, netlist.params.get("expected"));
    assert_eq!(netlist.options.temp, Some(84.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_options_resolve_in_their_scope_and_keep_global_assignment_order() {
    for (body, temperature) in [
        (
            ".options temp={ambient}\n.param ambient=80\n.options temp=65\n",
            65.0,
        ),
        (
            ".options temp={ambient}\n.subckt child p n\n.options temp={local}\n.param local=75\n.ends\n.param ambient=80\n",
            75.0,
        ),
        (
            ".subckt child p n\n.options temp={local}\n.subckt inner p n\n.options temp=65\n.ends\n.param local=75\n.ends\n",
            65.0,
        ),
        (
            ".subckt child p n\n.options temp={local}\n.param local=75\n.ends\n.options temp={ambient}\n.param ambient=80\n",
            80.0,
        ),
    ] {
        let source = format!("Forward option precedence\n{body}.end\n");
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.options.temp, Some(temperature), "{body}");
    }
    let error = Netlist::parse("No sibling binding\n.subckt a p n\n.options temp={ambient}\n.ends\n.subckt b p n\n.param ambient=75\n.ends\n.end\n").unwrap_err();
    assert!(
        error.to_string().contains("Undefined parameter: AMBIENT"),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_temperature_draws_follow_scope_declarations_without_failed_probe_draws() {
    let source = "Forward sampled options\n.options seed=37 temp={aunif(0,1)+ambient} tnom={nominal+aunif(0,1)}\n.param ambient=85 nominal=35 after={aunif(0,1)}\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    assert_eq!(
        netlist.params.get("after"),
        Some(eval_expression("aunif(0,1)", &reference).unwrap())
    );
    assert_eq!(
        netlist.options.temp,
        Some(85.0 + eval_expression("aunif(0,1)", &reference).unwrap())
    );
    assert_eq!(
        netlist.options.tnom,
        Some(35.0 + eval_expression("aunif(0,1)", &reference).unwrap())
    );
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &reference).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_options_validate_superseded_cards_and_keep_their_origin() {
    for tail in [".param ambient=-274\n", ".param other=85\n"] {
        let source = format!(
            "Deferred error\n.options reltol=1e-5\n+ temp={{ambient}}\n.options temp=65\n{tail}.end\n"
        );
        let error = Netlist::parse(&source).unwrap_err();
        assert!(
            matches!(
                error,
                rspice_core::netlist::ParseError::Syntax { line: 2, .. }
            ),
            "{error}"
        );
        assert!(error.to_string().contains("TEMP"), "{error}");
    }
    let netlist = Netlist::parse("Inactive forward option\n.if 0\n.options temp={missing}\n.endif\n.options temp=65\n.end\n.options tnom={also_missing}\n").unwrap();
    assert_eq!(netlist.options.temp, Some(65.0));
    assert_eq!(netlist.options.tnom, None);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lazy_and_user_function_options_preserve_statistical_evaluation() {
    let source = "Function sampled options\n.options seed=37\n.func twice(x) {x+x}\n.options temp={85+twice(aunif(0,1))+ambient}\n.options tnom={if(1,35+aunif(0,1),missing)}\n.param ambient=0 after={aunif(0,1)}\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let reference = Netlist::parse("Reference draws\n.options seed=37\n.func twice(x) {x+x}\n.param nominal={35+aunif(0,1)} after={aunif(0,1)} temperature={85+twice(aunif(0,1))}\n.end\n").unwrap();
    assert_eq!(netlist.options.tnom, reference.params.get("nominal"));
    assert_eq!(netlist.params.get("after"), reference.params.get("after"));
    assert_eq!(netlist.options.temp, reference.params.get("temperature"));
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &reference.params).unwrap()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn parameter_options_apply_to_earlier_primitives_and_temperature_builtins() {
    let netlist = Netlist::parse("Temperature parameters\n.param ambient=85 nominal=35\n.param before={TEMP} nominal_before={TNOM} vt_before={VT}\nI1 0 out 1m\nR1 out 0 {TEMP}\n.options DEVICE temp={ambient} tnom={nominal}\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.options.tnom, Some(35.0));
    assert_eq!(netlist.params.get("before"), Some(85.0));
    assert_eq!(netlist.params.get("nominal_before"), Some(35.0));
    assert_eq!(netlist.params.get("TEMP"), netlist.params.get("TEMPER"));
    close(
        netlist.params.get("vt_before").unwrap(),
        rspice_core::constants::thermal_voltage(358.15),
    );
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    close(result.try_voltage_named("out").unwrap(), 0.085);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_invalid_and_post_end_options_do_not_set_temperature() {
    for suppressed in ["temp=-274", "temp={missing}", "temp=999 tnom=444"] {
        let source = format!(
            "Scoped options\n.param ambient=65 select=1\n.if select\n.options temp={{ambient}}\n.else\n.options {suppressed}\n.endif\n.param sampled={{TEMP}}\n.end\n.options temp=888 tnom=333\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.options.temp, Some(65.0));
        assert_eq!(netlist.options.tnom, None);
        assert_eq!(netlist.params.get("sampled"), Some(65.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lexical_parameter_scope_and_redefinitions_survive_temperature_replay() {
    let netlist = Netlist::parse("Scoped temperature\n.param ambient=65\n.subckt local p n params: ambient=99\nRlocal p n {ambient}\n.ends\n.options temp={ambient}\n.param ambient=75\n.param observed={TEMP}\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, Some(65.0));
    assert_eq!(netlist.params.get("ambient"), Some(75.0));
    assert_eq!(netlist.params.get("observed"), Some(65.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn random_temperature_expressions_consume_the_authored_sequence_once() {
    for temperature in [
        ".options temp={aunif(50,5)}\n",
        ".param ambient={aunif(50,5)}\n.options temp={ambient}\n",
    ] {
        let source = format!(
            "Sampled temperature\n.options seed=37\n{temperature}.param after={{aunif(0,1)}}\n.end\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let mut reference = ParamContext::new();
        reference.set_random_seed(37);
        assert_eq!(
            netlist.options.temp,
            Some(eval_expression("aunif(50,5)", &reference).unwrap())
        );
        assert_eq!(
            netlist.params.get("after"),
            Some(eval_expression("aunif(0,1)", &reference).unwrap())
        );
        assert_eq!(netlist.params.get("TEMP"), netlist.options.temp);
        assert_eq!(netlist.options.seed, Some(37));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nominal_temperature_dependencies_resolve_and_cycles_are_rejected() {
    let netlist = Netlist::parse("Nominal dependency\n.param nominal=55\n.options temp={TNOM+10} tnom={nominal}\n.param observed={TEMP}\n.end\n").unwrap();
    assert_eq!(netlist.options.tnom, Some(55.0));
    assert_eq!(netlist.options.temp, Some(65.0));
    assert_eq!(netlist.params.get("observed"), Some(65.0));
    let error = Netlist::parse("Temperature cycle\n.options temp={TEMP+1}\n.end\n").unwrap_err();
    assert!(
        error.to_string().contains("TEMP/TNOM selection changes"),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn single_temp_directive_and_table_coordinates_keep_physical_precedence() {
    let netlist = Netlist::parse("Temperature replay\n.param ambient=27\n.options temp={ambient}\n.temp 45\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.data rows FREQ TEMP\n100 85\n100 125\n.enddata\n.end\n").unwrap();
    assert_eq!(netlist.options.temp, Some(45.0));
    assert_eq!(netlist.params.get("TEMP"), Some(45.0));
    let engine = Engine::default();
    let (rows, _) = engine.run_ac_data(&netlist, "rows").unwrap();
    for (row, expected) in rows.iter().zip([85.0, 125.0]) {
        assert_eq!(row.options.temp, Some(expected));
        assert_eq!(row.params.get("TEMP"), Some(expected));
        assert_eq!(row.params.get("TEMPER"), Some(expected));
        close(
            row.params.get("VT").unwrap(),
            rspice_core::constants::thermal_voltage(expected + 273.15),
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn parameter_table_rows_set_physical_noise_temperature_through_authored_options() {
    let source = "Parameter temperature rows\n.param ambient=27\n.options temp={ambient}\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.data rows FREQ ambient\n100 85\n100 125\n.enddata\n.end\n";
    for expression_dialect in [
        rspice_core::config::ExpressionDialect::Ngspice,
        rspice_core::config::ExpressionDialect::Xyce,
    ] {
        let netlist = Netlist::parse_with_options(
            source,
            rspice_core::netlist::NetlistParseOptions {
                expression_dialect,
                ..Default::default()
            },
        )
        .unwrap();
        let engine = Engine::new(rspice_core::SimulationConfig {
            spice_dialect: rspice_core::SpiceDialect::BestAvailable,
            ..Default::default()
        });
        let result = engine
            .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "rows", 999.0)
            .unwrap();
        assert_eq!(result.points.len(), 2);
        for (point, temperature) in result.points.iter().zip([358.15, 398.15]) {
            close(
                point.input_referred_density,
                4.0 * 1.380649e-23 * temperature * 1000.0,
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn active_invalid_temperature_reports_its_option_line() {
    let error = Netlist::parse(
        "Invalid temperature\n.param ambient=-274\n.options reltol=1e-5\n+ temp={ambient}\n.end\n",
    )
    .unwrap_err();
    assert!(
        matches!(
            error,
            rspice_core::netlist::ParseError::Syntax { line: 3, .. }
        ),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn default_temperature_builtins_do_not_add_stored_parameter_bindings() {
    let netlist = Netlist::parse("Default temperature\nR1 out 0 1k\n.end\n").unwrap();
    assert!(netlist.params.numeric_parameters().is_empty());
    for name in ["TEMP", "TEMPER", "TNOM"] {
        assert_eq!(netlist.params.get(name), Some(27.0));
        assert!(!netlist.params.has_any_parameter_binding(name));
    }
    assert_eq!(netlist.options.temp, None);
    assert_eq!(netlist.options.tnom, None);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn expanded_source_boundaries_and_parameter_options_share_the_same_replay() {
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};
    let root = std::env::temp_dir().join("rspice-temperature-root.cir");
    let child = root.with_file_name("rspice-temperature-child.inc");
    let source = "Included temperature\n.include child.inc\n.param observed={TEMP}\n.end\n.options temp=999\n";
    let include = ".param ambient=85\n.options temp={ambient}\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "child.inc".into(),
            target: child,
        }],
    )
    .unwrap();
    let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &root,
        bundle,
        NetlistParseOptions::default(),
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    assert_eq!(netlist.params.get("observed"), Some(85.0));
    assert_eq!(netlist.source_path.as_ref(), Some(&root));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deferred_included_option_errors_keep_the_card_path_and_local_line() {
    use rspice_core::netlist::{
        NetlistParseOptions, ParseError, SealedSourceBundle, SealedSourceEdge,
    };
    let root = std::env::temp_dir().join("rspice-forward-temperature-root.cir");
    let child = root.with_file_name("rspice-forward-temperature-child.inc");
    for (source, include, expected_line) in [
        (
            "Included root option\n.include child.inc\n.param ambient=-274\n.end\n",
            ".options temp={ambient}\n",
            1,
        ),
        (
            "Included child scope\n.include child.inc\n.end\n",
            ".subckt local p n\n.options temp={ambient}\n.param ambient=-274\n.ends\n",
            2,
        ),
    ] {
        let bundle = SealedSourceBundle::try_new_with_edges(
            [
                (root.clone(), source.to_owned()),
                (child.clone(), include.to_owned()),
            ],
            [SealedSourceEdge {
                owner: root.clone(),
                requested_path: "child.inc".into(),
                target: child.clone(),
            }],
        )
        .unwrap();
        let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            source,
            &root,
            bundle,
            NetlistParseOptions::default(),
            &rspice_core::NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(&error, rspice_core::netlist::ParseWithAbortError::Parse(ParseError::Syntax { line, .. }) if *line == expected_line),
            "{error}"
        );
        let message = error.to_string();
        assert!(
            message.contains(&format!("{}:{expected_line}", child.display())),
            "{message}"
        );
        assert!(!message.contains(&root.display().to_string()), "{message}");
    }
}
