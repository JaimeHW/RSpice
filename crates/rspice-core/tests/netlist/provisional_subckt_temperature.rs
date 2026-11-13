//! Subcircuit defaults must be validated at the selected parser temperatures.
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn header_defaults_reconcile_before_instance_expansion() {
    let source = "Provisional subcircuit default\n.subckt cell p params: gain={58/(TEMP-27)}\nR1 p 0 {gain}\n.ends\nI1 0 out 1m\nX1 out cell\nI2 0 overridden 1m\nX2 overridden cell gain=2\n.temp 85\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    assert_eq!(netlist.options.temp, Some(85.0));
    let cell = &netlist.subcircuits[0];
    assert!(cell.expr_params.iter().any(|(name, expression)| {
        name.eq_ignore_ascii_case("gain") && expression == "58/(TEMP-27)"
    }));
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    for (node, expected) in [("out", 0.001), ("overridden", 0.002)] {
        let actual = result.try_voltage_named(node).unwrap();
        assert!((actual - expected).abs() < 1e-12, "{node}: {actual}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn derived_header_defaults_reach_forward_nominal_temperature() {
    let source = "Forward nominal default\n.subckt cell p params: derived={2*gain} gain={28/(TNOM-27)}\nV1 p 0 {derived}\n.ends\nX1 out cell\nX2 overridden cell gain=3\n.options tnom={nominal}\n.param nominal=55\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    assert_eq!(netlist.options.tnom, Some(55.0));
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    for (node, expected) in [("out", 2.0), ("overridden", 6.0)] {
        let actual = result.try_voltage_named(node).unwrap();
        assert!((actual - expected).abs() < 1e-10, "{node}: {actual}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nested_defaults_keep_formal_ownership_and_instance_overrides() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::netlist::NetlistParseOptions;
    for expression_dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let source = "Nested defaults\n.param gain=999\n.subckt parent p params: gain={58/(TEMP-27)}\n.subckt child q params: derived={2*gain}\nV1 q 0 {derived}\n.options temp={ambient}\n.ends\nXinner p child\n.param ambient=85\n.ends\nX1 out parent\nX2 overridden parent gain=3\n.end\n";
        let netlist = Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                expression_dialect,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(netlist.params.get("gain"), Some(999.0));
        assert_eq!(netlist.options.temp, Some(85.0));
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        for (node, expected) in [("out", 2.0), ("overridden", 6.0)] {
            let actual = result.try_voltage_named(node).unwrap();
            assert!((actual - expected).abs() < 1e-10, "{node}: {actual}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn header_forward_progress_can_resolve_an_intermediate_domain_error() {
    let netlist = Netlist::parse("Header progress\n.param bias=27\n.subckt cell p params: gain={58/(bias-27)} bias=85\nV1 p 0 {gain}\n.ends\nX1 out cell\n.end\n").unwrap();
    let actual = Engine::default()
        .run_dc_op(&netlist)
        .unwrap()
        .try_voltage_named("out")
        .unwrap();
    assert!((actual - 1.0).abs() < 1e-12, "{actual}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn duplicate_defaults_keep_the_selected_definition_policy() {
    use rspice_core::netlist::{NetlistParseOptions, ParameterRedefinitionPolicy};
    for (parameter_redefinition_policy, defaults) in [
        (
            ParameterRedefinitionPolicy::UseFirst,
            "gain={58/(TEMP-27)} GAIN={1/0}",
        ),
        (
            ParameterRedefinitionPolicy::UseLast,
            "gain={1/0} GAIN={58/(TEMP-27)}",
        ),
    ] {
        let netlist = Netlist::parse_with_options(
            &format!("Duplicate default\n.subckt cell p params: {defaults}\nV1 p 0 {{gain}}\n.ends\nX1 out cell\n.temp 85\n.end\n"),
            NetlistParseOptions {parameter_redefinition_policy, ..Default::default()}
        ).unwrap();
        let actual = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((actual - 1.0).abs() < 1e-12, "{actual}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn invalid_defaults_cannot_be_hidden_by_usage_or_body_redefinitions() {
    for expression in ["{1/(TEMP-TEMP)}", "{1/(TEMP-85)}", "{1+}"] {
        for body_and_instance in [
            ".ends\n",
            ".ends\nX1 out cell gain=2\n",
            ".param gain=2\n.ends\n",
        ] {
            let source = format!(
                "Invalid default\n.subckt cell p params: gain={expression}\n{body_and_instance}.temp 85\n.end\n"
            );
            let error = Netlist::parse(&source).unwrap_err();
            assert!(error.to_string().contains("line 2"), "{error}");
        }
    }
    // A dependent forward reference must not hide the actual domain error.
    let error = Netlist::parse("Invalid derived default\n.subckt cell p params: derived={2*gain} gain={1/0}\n.ends\n.temp 85\n.end\n").unwrap_err();
    assert!(error.to_string().contains("Division by zero"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_and_cyclic_temperature_hints_cannot_publish_a_provisional_default() {
    for cards in [
        ".if 0\n.temp 85\n.endif",
        ".temp {if(TEMP==27,85,27)}",
        ".temp {gain}",
    ] {
        let source = format!(
            "Unconfirmed default\n.subckt cell p params: gain={{58/(TEMP-27)}}\n.ends\n{cards}\n.end\n"
        );
        assert!(Netlist::parse(&source).is_err(), "{source}");
    }
    let netlist = Netlist::parse("Inactive definition\n.if 0\n.subckt unused p params: gain={1/0}\n.ends\n.endif\n.temp 85\n.end\n").unwrap();
    assert!(netlist.subcircuits.is_empty());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn provisional_defaults_do_not_consume_the_live_statistical_stream() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Default statistics\n.options seed=37\n.subckt cell p params: gain={aunif(1,.1)+58/(TEMP-27)}\nR1 p 0 {gain}\n.ends\n.param ambient={85+aunif(0,1)} tail={aunif(0,1)}\n.options temp={ambient}\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let first = eval_expression("aunif(0,1)", &expected).unwrap();
    let ambient = 85.0 + first;
    assert_eq!(netlist.options.temp, Some(ambient));
    assert_eq!(
        netlist.params.get("tail"),
        Some(eval_expression("aunif(0,1)", &expected).unwrap())
    );
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
    let gain = netlist.subcircuits[0]
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
        .unwrap()
        .1;
    assert_eq!(gain, 1.0 + 0.1 * first + 58.0 / (ambient - 27.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn resource_errors_take_priority_over_retained_header_errors() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    let mut options = NetlistParseOptions::default();
    options.resource_limits.max_analysis_points = 2;
    let error = Netlist::parse_with_options("Header resource priority\n.subckt cell p params: gain={58/(TEMP-27)}\n.ends\n.data samples x\n1\n2\n3\n.enddata\n.temp 85\n.end\n", options).unwrap_err();
    assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn header_errors_keep_the_included_definition_origin() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-provisional-subckt/root.cir"
    } else {
        "/rspice-provisional-subckt/root.cir"
    });
    let child = root.with_file_name("cell.inc");
    let source = "Included default\n.include cell.inc\n.temp 85\n.end\n";
    let include = "* default\n.subckt cell p params:\n+ gain={1/(TEMP-85)}\n.ends\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.to_owned()),
            (child.clone(), include.to_owned()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "cell.inc".into(),
            target: child,
        }],
    )
    .unwrap();
    let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &root,
        bundle,
        Default::default(),
        &rspice_core::NoAbort,
    )
    .unwrap_err();
    assert!(error.to_string().contains("cell.inc:2:"), "{error}");
    assert!(error.to_string().contains("Division by zero"), "{error}");
}
