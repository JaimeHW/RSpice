//! Forward declarations must feed the ordinary independent-source grammar.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{ElementKind, NetlistParseOptions, SourceSpec, parse_source_spec_text};
use rspice_core::{Engine, Netlist};

const DIALECTS: [ExpressionDialect; 2] = [ExpressionDialect::Ngspice, ExpressionDialect::Xyce];

fn options(expression_dialect: ExpressionDialect) -> NetlistParseOptions {
    NetlistParseOptions {
        expression_dialect,
        ..Default::default()
    }
}

fn reference() -> ParamContext {
    let mut params = ParamContext::new();
    params.set_random_seed(37);
    params
}

fn draw(params: &ParamContext) -> f64 {
    eval_expression("aunif(0,1)", params).unwrap()
}

fn source<'a>(netlist: &'a Netlist, name: &str) -> &'a SourceSpec {
    match &netlist
        .elements
        .iter()
        .find(|element| element.name == name)
        .unwrap()
        .kind
    {
        ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => spec,
        kind => panic!("expected a resolved source, got {kind:?}"),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_voltage_and_current_declarations_reach_the_operating_point() {
    for dialect in DIALECTS {
        for value in ["{value}", "value", "+value", "{value+img(z)}", "DC"] {
            let expected = if value.contains("img") { 0.0 } else { 1.0 };
            let source = format!(
                "Forward source\n.param value={{later}} DC={{later}} z={{sqrt(-1)}}\nV1 voltage 0 DC = {value}\nI1 0 current DC = {value}\nR1 voltage 0 1\nR2 current 0 1\n.param later=1\n.end\n"
            );
            let netlist = Netlist::parse_with_options(&source, options(dialect)).unwrap();
            let op = Engine::default().run_dc_op(&netlist).unwrap();
            for node in ["voltage", "current"] {
                let actual = op.try_voltage_named(node).unwrap();
                assert!(
                    (actual - expected).abs() < 1e-10,
                    "{dialect:?} {value}: {actual}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_waveform_fields_keep_the_shared_source_grammar() {
    for dialect in DIALECTS {
        let prefix = "Forward waveform\n.param high={base+1} delay={later} phase={angle}\n";
        for spec in [
            "DC {high} AC {high} {phase}",
            "PULSE(0 {high} {delay} 1n 1n 1u 2u)",
            "PWL(0 0 {delay} {high})",
            "SIN(0 {high} 1k)",
        ] {
            let netlist = Netlist::parse_with_options(
                &format!(
                    "{prefix}V1 out 0 {spec}\nR1 out 0 1k\n.param base=1 later=1u angle=90\n.end\n"
                ),
                options(dialect),
            )
            .unwrap();
            match source(&netlist, "V1") {
                SourceSpec::DcAc {
                    dc_value,
                    ac_magnitude,
                    ac_phase,
                } => {
                    assert_eq!((*dc_value, *ac_magnitude), (2.0, 2.0));
                    assert!((*ac_phase - std::f64::consts::FRAC_PI_2).abs() < 1e-14);
                }
                SourceSpec::Pulse {
                    v1,
                    v2,
                    delay,
                    rise,
                    fall,
                    width,
                    period,
                    ..
                } => {
                    assert_eq!((*v1, *v2, *delay), (0.0, 2.0, 1e-6));
                    assert_eq!((*rise, *fall, *width, *period), (1e-9, 1e-9, 1e-6, 2e-6));
                }
                SourceSpec::Pwl { points, .. } => assert_eq!(points, &[(0.0, 0.0), (1e-6, 2.0)]),
                SourceSpec::Sin {
                    offset,
                    amplitude,
                    frequency,
                    ..
                } => {
                    assert_eq!((*offset, *amplitude, *frequency), (0.0, 2.0, 1000.0));
                }
                spec => panic!("unexpected source: {spec:?}"),
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_sources_models_and_parameters_share_one_sample() {
    for dialect in DIALECTS {
        let netlist = Netlist::parse_with_options("Shared deferred sample\n.options seed=37\n.param amplitude={aunif(0,1)+base}\nV1 voltage 0 DC {aunif(0,1)} AC {amplitude}\nI1 0 current DC {amplitude}\n.model rm R (R={amplitude})\n.param base=85 eager={aunif(0,1)}\n.end\n", options(dialect)).unwrap();
        let expected = reference();
        assert_eq!(netlist.params.get("eager"), Some(draw(&expected)));
        let amplitude = 85.0 + draw(&expected);
        assert_eq!(netlist.params.get("amplitude"), Some(amplitude));
        let SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ..
        } = source(&netlist, "V1")
        else {
            panic!("DC/AC source");
        };
        assert_eq!(*dc_value, draw(&expected));
        assert_eq!(*ac_magnitude, amplitude);
        assert!(matches!(source(&netlist, "I1"), SourceSpec::Dc(value) if *value == amplitude));
        assert!(
            netlist.models[0]
                .params
                .iter()
                .any(|(name, value)| name == "R" && *value == amplitude)
        );
        assert_eq!(draw(&netlist.params), draw(&expected));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_source_probes_and_deferral_classification_do_not_consume_draws() {
    for dialect in DIALECTS {
        for tail in ["AC later", "AC {later}", "AC {sample(later)}"] {
            let netlist = Netlist::parse_with_options(&format!("Failed source probes\n.options seed=37\n.func sample(x) {{x+aunif(0,1)}}\nV1 v 0 DC {{aunif(0,1)}} {tail}\nI1 0 i DC {{aunif(0,1)}} {tail}\n.param eager={{aunif(0,1)}} later=2\n.end\n"), options(dialect)).unwrap();
            let expected = reference();
            assert_eq!(netlist.params.get("eager"), Some(draw(&expected)));
            for name in ["V1", "I1"] {
                let SourceSpec::DcAc {
                    dc_value,
                    ac_magnitude,
                    ..
                } = source(&netlist, name)
                else {
                    panic!("DC/AC source");
                };
                assert_eq!(*dc_value, draw(&expected));
                let ac = if tail.contains("sample") {
                    2.0 + draw(&expected)
                } else {
                    2.0
                };
                assert_eq!(*ac_magnitude, ac);
            }
            assert_eq!(draw(&netlist.params), draw(&expected));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn public_source_retry_retains_the_live_random_position() {
    for expression in ["aunif(0,1)", "limit(0,1)", "agauss(0,1,1)"] {
        let mut params = reference();
        let expected = reference();
        let spec = format!("DC {{{expression}}} AC {{missing}}");
        assert!(parse_source_spec_text(&spec, 12, &params).is_err());
        params.set("missing", 2.0);
        let SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ..
        } = parse_source_spec_text(&spec, 12, &params).unwrap()
        else {
            panic!("DC/AC source");
        };
        assert_eq!(dc_value, eval_expression(expression, &expected).unwrap());
        assert_eq!(ac_magnitude, 2.0);
        assert_eq!(draw(&params), draw(&expected));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn source_probes_preserve_lazy_branches_and_function_sampling() {
    let mut params = reference();
    params.define_function("sample", vec!["x".into()], "aunif(0,1)+x");
    let expected = reference();
    let spec =
        parse_source_spec_text("DC {1 ? aunif(0,1) : missing} AC {sample(2)}", 0, &params).unwrap();
    let SourceSpec::DcAc {
        dc_value,
        ac_magnitude,
        ..
    } = spec
    else {
        panic!("DC/AC source");
    };
    assert_eq!(dc_value, draw(&expected));
    assert_eq!(ac_magnitude, 2.0 + draw(&expected));
    assert_eq!(draw(&params), draw(&expected));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn local_forward_sources_keep_their_instance_scope() {
    for dialect in DIALECTS {
        let netlist = Netlist::parse_with_options("Local sources\n.subckt cell out base=1\n.param value={later}\nV1 out 0 {value}\n.param later={base+1}\n.ends\nX1 first cell base=1\nX2 second cell base=6\nR1 first 0 1k\nR2 second 0 1k\n.end\n", options(dialect)).unwrap();
        let op = Engine::default().run_dc_op(&netlist).unwrap();
        assert!((op.try_voltage_named("first").unwrap() - 2.0).abs() < 1e-10);
        assert!((op.try_voltage_named("second").unwrap() - 7.0).abs() < 1e-10);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deferred_source_errors_keep_the_include_card_origin() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::env::temp_dir().join("rspice-forward-source-root.cir");
    let child = root.with_file_name("rspice-forward-source-child.inc");
    let source = "Included source\n.include child.inc\n.param later=2\n.end\n";
    for dialect in DIALECTS {
        for spec in [
            "DC {missing}",
            "PULSE(0 1 {missing})",
            "DC {later} AC missing",
        ] {
            let include = format!("* child source\nV1 out 0 {spec}\n");
            let bundle = SealedSourceBundle::try_new_with_edges(
                [(root.clone(), source.to_owned()), (child.clone(), include)],
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
                options(dialect),
                &rspice_core::NoAbort,
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains(&format!("{}:2", child.display())), "{error}");
            assert!(error.contains("V1"), "{error}");
            assert!(!error.contains(&root.display().to_string()), "{error}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_source_parameters_preserve_dc_and_ac_sensitivity() {
    for dialect in DIALECTS {
        let netlist = Netlist::parse_with_options("Forward sensitivity\n.param value={base*base}\nV1 out 0 DC {value} AC {value}\nR1 out 0 1k\n.param base=2\n.end\n", options(dialect)).unwrap();
        let engine = Engine::default();
        let node = engine
            .build_circuit(&netlist)
            .unwrap()
            .get_node_by_name("out")
            .unwrap();
        let dc = engine
            .run_sensitivity(&netlist, node, "base", 2.0, None)
            .unwrap();
        let ac = engine
            .run_sensitivity_ac(&netlist, node, "base", 2.0, &[1.0], None)
            .unwrap();
        assert!((dc - 4.0).abs() < 1e-8, "{dialect:?}: DC {dc}");
        assert!((ac[0] - 4.0).abs() < 1e-8, "{dialect:?}: AC {ac:?}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_piecewise_source_executes_the_authored_ramp() {
    for dialect in DIALECTS {
        let netlist = Netlist::parse_with_options("Forward ramp\n.param high={base*2} duration={last}\nV1 out 0 PWL(0 0 {duration} {high})\nR1 out 0 1k\n.param base=1 last=1u\n.end\n", options(dialect)).unwrap();
        let result = Engine::default().run_tran(&netlist, 2e-6, 1e-7).unwrap();
        let voltage = result.try_voltage_waveform_named("out").unwrap();
        assert!(result.time.len() > 2);
        for (&time, &actual) in result.time.iter().zip(voltage) {
            let expected = 2.0 * (time / 1e-6).min(1.0);
            assert!(
                (actual - expected).abs() < 1e-9,
                "{dialect:?} t={time}: {actual} != {expected}"
            );
        }
        assert!((result.time.last().unwrap() - 2e-6).abs() < 1e-15);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn grouped_runtime_source_stays_dynamic_after_forward_resolution() {
    let netlist = Netlist::parse_with_options("Runtime source\n.param value={base+TIME}\nV1 out 0 {value}\nR1 out 0 1k\n.param base=1\n.end\n", options(ExpressionDialect::Xyce)).unwrap();
    let result = Engine::default().run_tran(&netlist, 0.01, 0.001).unwrap();
    let voltage = result.try_voltage_waveform_named("out").unwrap();
    for (&time, &actual) in result.time.iter().zip(voltage) {
        assert!((actual - (1.0 + time)).abs() < 1e-9, "t={time}: {actual}");
    }
    assert!((result.time.last().unwrap() - 0.01).abs() < 1e-15);
}
