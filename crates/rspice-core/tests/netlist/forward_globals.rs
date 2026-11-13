//! Global projections must share their resolved values with numeric consumers.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{ElementKind, NetlistParseOptions, SourceSpec};
use rspice_core::{Engine, Netlist};

const DIALECTS: [ExpressionDialect; 2] = [ExpressionDialect::Ngspice, ExpressionDialect::Xyce];

fn parse(source: &str, expression_dialect: ExpressionDialect) -> Netlist {
    Netlist::parse_with_options(
        source,
        NetlistParseOptions {
            expression_dialect,
            ..Default::default()
        },
    )
    .unwrap()
}

fn dc(netlist: &Netlist, name: &str) -> f64 {
    match &netlist
        .elements
        .iter()
        .find(|element| element.name == name)
        .unwrap()
        .kind
    {
        ElementKind::VoltageSource(SourceSpec::Dc(value))
        | ElementKind::CurrentSource(SourceSpec::Dc(value)) => *value,
        value => panic!("expected numeric DC source: {value:?}"),
    }
}

fn reference(dialect: ExpressionDialect) -> ParamContext {
    let mut context = ParamContext::new();
    context.set_random_seed(37);
    context.set_expression_dialect(dialect);
    context.define_function("sample", vec!["X".into()], "x+aunif(0,1)");
    context
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_globals_feed_voltage_current_and_complex_model_values() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Forward globals\n.global_param value={later} z={later+2j} magnitude={img(z)}\nV1 voltage 0 DC {value}\nI1 0 current DC {magnitude}\nR1 voltage 0 1\nR2 current 0 1\n.model rm R (R={img(z)})\n.param later=3\n.end\n",
            dialect,
        );
        assert_eq!(netlist.params.get("value"), Some(3.0));
        assert_eq!(netlist.params.get("magnitude"), Some(2.0));
        assert_eq!(netlist.params.get_complex("z").unwrap().im, 2.0);
        assert_eq!(netlist.params.get_global_expression("value"), Some("later"));
        assert!(
            netlist.models[0]
                .params
                .iter()
                .any(|(key, value)| key == "R" && *value == 2.0)
        );
        let op = Engine::default().run_dc_op(&netlist).unwrap();
        assert!((op.try_voltage_named("voltage").unwrap() - 3.0).abs() < 1e-10);
        assert!((op.try_voltage_named("current").unwrap() - 2.0).abs() < 1e-10);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_statistical_globals_are_shared_by_ordinary_sources_and_models() {
    for dialect in DIALECTS {
        for expression in ["aunif(0,1)", "limit(0,1)", "agauss(0,1,1)", "sample(0)"] {
            let netlist = parse(
                &format!(
                    "Shared globals\n.options seed=37\n.func sample(x) {{x+aunif(0,1)}}\n.global_param amplitude={{{expression}+base}}\n.param twice={{amplitude+amplitude}}\nV1 out 0 DC {{amplitude}}\nI1 0 other DC {{twice}}\n.model rm R (R={{amplitude}})\n.param base=85 eager={{aunif(0,1)}}\n.end\n"
                ),
                dialect,
            );
            let expected = reference(dialect);
            assert_eq!(
                netlist.params.get("eager"),
                Some(eval_expression("aunif(0,1)", &expected).unwrap())
            );
            let amplitude = eval_expression(expression, &expected).unwrap() + 85.0;
            assert_eq!(
                netlist.params.get("amplitude"),
                Some(amplitude),
                "{dialect:?} {expression}"
            );
            assert_eq!(netlist.params.get("twice"), Some(amplitude + amplitude));
            assert_eq!(dc(&netlist, "V1"), amplitude);
            assert_eq!(dc(&netlist, "I1"), amplitude + amplitude);
            assert!(
                netlist.models[0]
                    .params
                    .iter()
                    .any(|(key, value)| key == "R" && *value == amplitude)
            );
            assert_eq!(
                eval_expression("aunif(0,1)", &netlist.params).unwrap(),
                eval_expression("aunif(0,1)", &expected).unwrap()
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn captured_global_samples_survive_forward_consumers_and_repeated_finalization() {
    for dialect in DIALECTS {
        for expression in ["aunif(0,1)", "limit(0,1)", "agauss(0,1,1)", "sample(0)"] {
            let mut netlist = parse(
                &format!(
                    "Captured globals\n.options seed=37\n.func sample(x) {{x+aunif(0,1)}}\n.global_param amplitude={{{expression}+85}}\n.param total={{amplitude+later}}\nV1 out 0 DC {{amplitude}}\nI1 0 other DC {{total}}\n.model rm R (R={{amplitude}})\n.param later=1\n.end\n"
                ),
                dialect,
            );
            let expected = reference(dialect);
            let amplitude = eval_expression(expression, &expected).unwrap() + 85.0;
            assert_eq!(
                netlist.params.get("amplitude"),
                Some(amplitude),
                "{dialect:?} {expression}"
            );
            assert_eq!(netlist.params.get("total"), Some(amplitude + 1.0));
            assert_eq!(dc(&netlist, "V1"), amplitude);
            assert_eq!(dc(&netlist, "I1"), amplitude + 1.0);
            assert!(
                netlist.models[0]
                    .params
                    .iter()
                    .any(|(key, value)| key == "R" && *value == amplitude)
            );
            rspice_core::netlist::expr::finalize_parameter_expressions(&mut netlist.params)
                .unwrap();
            assert_eq!(netlist.params.get("amplitude"), Some(amplitude));
            assert_eq!(
                eval_expression("aunif(0,1)", &netlist.params).unwrap(),
                eval_expression("aunif(0,1)", &expected).unwrap()
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deterministic_global_projections_follow_redefinitions_and_shadowing() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Redefined globals\n.param base=2\n.global_param value={base+1} clipped={limit(base,0,10)}\nV1 out 0 DC {value} AC {later}\n.param base=5 later=1\n.end\n",
            dialect,
        );
        assert_eq!(netlist.params.get("value"), Some(6.0));
        assert_eq!(netlist.params.get("clipped"), Some(5.0));
        assert!(matches!(
            netlist.elements[0].kind,
            ElementKind::VoltageSource(SourceSpec::DcAc { dc_value: 6.0, .. })
        ));
        let netlist = parse(
            "Shadowed globals\n.global_param value={base+1}\n.param value={later}\nV1 out 0 DC {value}\n.param base=10 later=3\n.end\n",
            dialect,
        );
        assert_eq!(dc(&netlist, "V1"), 3.0);
        assert_eq!(
            netlist.params.get_global_expression("value"),
            Some("base+1")
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn temperature_sources_and_models_share_a_forward_global_sample() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Global temperature\n.options seed=37 temp={ambient} tnom={ambient}\n.func sample(x) {x+aunif(0,1)}\n.global_param ambient={sample(later)}\nV1 out 0 DC {ambient}\n.model rm R (R={ambient})\n.param later=85\n.end\n",
            dialect,
        );
        let expected = reference(dialect);
        let ambient = eval_expression("sample(85)", &expected).unwrap();
        assert_eq!(netlist.options.temp, Some(ambient));
        assert_eq!(netlist.options.tnom, Some(ambient));
        assert_eq!(netlist.params.get("ambient"), Some(ambient));
        assert_eq!(dc(&netlist, "V1"), ambient);
        assert!(
            netlist.models[0]
                .params
                .iter()
                .any(|(key, value)| key == "R" && *value == ambient)
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn global_source_sensitivity_tracks_the_resolved_parameter_graph() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Global sensitivity\n.global_param value={base*base}\nV1 out 0 DC {value} AC {value}\nR1 out 0 1k\n.param base=2\n.end\n",
            dialect,
        );
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
        assert!((dc - 4.0).abs() < 1e-8, "{dialect:?}: {dc}");
        assert!((ac[0] - 4.0).abs() < 1e-8, "{dialect:?}: {ac:?}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn runtime_global_sources_remain_symbolic_and_follow_time() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Runtime global\n.global_param value={base+TIME}\nV1 out 0 {value}\nR1 out 0 1k\n.param base=1\n.end\n",
            dialect,
        );
        assert!(
            netlist
                .params
                .get_global_expression("value")
                .unwrap()
                .contains("TIME")
        );
        let result = Engine::default().run_tran(&netlist, 0.01, 0.001).unwrap();
        let voltage = result.try_voltage_waveform_named("out").unwrap();
        for (&time, &actual) in result.time.iter().zip(voltage) {
            assert!(
                (actual - (1.0 + time)).abs() < 1e-9,
                "{dialect:?} t={time}: {actual}"
            );
        }
        assert!((result.time.last().unwrap() - 0.01).abs() < 1e-15);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn global_binding_keeps_lazy_branches_and_rejects_real_dependency_cycles() {
    for dialect in DIALECTS {
        let netlist = parse(
            "Lazy globals\n.global_param value={if(later,2,missing)}\nV1 out 0 DC {value}\n.param later=1\n.end\n",
            dialect,
        );
        assert_eq!(dc(&netlist, "V1"), 2.0);
        let error = Netlist::parse_with_options(
            "Cyclic globals\n.global_param a={b} b={a}\nV1 out 0 {a}\n.end\n",
            NetlistParseOptions {
                expression_dialect: dialect,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(error.to_string().contains("cyclic"), "{error}");
    }
}
