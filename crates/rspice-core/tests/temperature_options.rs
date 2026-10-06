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
