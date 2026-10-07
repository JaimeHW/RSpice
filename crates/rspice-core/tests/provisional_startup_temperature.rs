//! Startup cards must validate at selected temperatures before publication.
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_initial_conditions_reconcile_before_transient_startup() {
    let netlist = Netlist::parse("Provisional initial condition\nR1 out 0 1k\nC1 out 0 1u\n.ic V(out)={58/(TEMP-27)}\n.temp 85\n.tran 1u 2u uic\n.end\n").unwrap();
    assert_eq!(netlist.initial_conditions[0].voltage, 1.0);
    assert_eq!(netlist.startup_directives()[0].entries()[0].voltage(), 1.0);
    let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
    assert_eq!(result.try_voltage_waveform_named("out").unwrap()[0], 1.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_nodesets_reconcile_forward_nominal_temperature_and_release_the_hint() {
    let netlist = Netlist::parse("Provisional node set\nV1 in 0 2\nR1 in out 1k\nR2 out 0 1k\n.nodeset V(out)={196/(TNOM-27)}\n.options tnom={nominal}\n.param nominal=55\n.end\n").unwrap();
    assert_eq!(netlist.node_sets[0].voltage, 7.0);
    assert_eq!(netlist.startup_directives()[0].entries()[0].voltage(), 7.0);
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    let actual = result.try_voltage_named("out").unwrap();
    assert!((actual - 1.0).abs() < 1e-9, "{actual}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn device_initial_conditions_reconcile_before_transient_startup() {
    let netlist = Netlist::parse("Provisional device initial condition\nR1 out 0 1k\nC1 out 0 1u\n.initcond C1 IC={58/(TEMP-27)}\n.temp 85\n.tran 1u 2u uic\n.end\n").unwrap();
    assert_eq!(
        netlist.device_initial_conditions.as_ref().unwrap().entries[0].values,
        [1.0]
    );
    let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
    assert_eq!(result.try_voltage_waveform_named("out").unwrap()[0], 1.0);
}

fn hint_value(netlist: &Netlist, directive: &str, index: usize) -> f64 {
    if directive == ".IC" {
        netlist.initial_conditions[index].voltage
    } else {
        netlist.node_sets[index].voltage
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn voltage_hint_grammar_and_source_order_survive_recovery() {
    for directive in [".IC", ".NODESET"] {
        for fields in [
            "V(out)={58*gain/(TEMP-27)} V(other)=2",
            "out {58*gain/(TEMP-27)} other 2",
            "V(out)={58*gain/(TEMP-27)}\n+ V(other)=2",
        ] {
            let source = format!(
                "Source ordered hints\nR1 out 0 1k\nR2 other 0 1k\n.param gain=1\n{directive} {fields}\n.param gain=3\n.temp 85\n.end\n"
            );
            let netlist = Netlist::parse(&source).unwrap();
            assert_eq!(hint_value(&netlist, directive, 0), 1.0);
            assert_eq!(hint_value(&netlist, directive, 1), 2.0);
            assert_eq!(netlist.startup_directives().len(), 1);
            assert_eq!(netlist.startup_directives()[0].entries().len(), 2);
            assert_eq!(netlist.startup_directives()[0].origin().line, 5);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_differential_hints_keep_instance_expressions_and_provenance() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::netlist::{NetlistParseOptions, flatten_netlist_with_models};
    for expression_dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for directive in [".IC", ".NODESET"] {
            let source = format!(
                "Scoped hints\n.subckt cell p n params: gain=1\n{directive} V(p,n)={{gain*58/(TEMP-27)}}\nR1 p n 1k\nC1 p n 1u\n.ends\nX1 a 0 cell gain=2\nX2 b 0 cell gain=3\n.temp 85\n.tran 1u 2u uic\n.end\n"
            );
            let netlist = Netlist::parse_with_options(
                &source,
                NetlistParseOptions {
                    expression_dialect,
                    ..Default::default()
                },
            )
            .unwrap();
            let entry = &netlist.startup_directives()[0].entries()[0];
            assert!(entry.authored_node().eq_ignore_ascii_case("p"));
            assert!(
                entry
                    .authored_reference()
                    .unwrap()
                    .eq_ignore_ascii_case("n")
            );
            assert_eq!(entry.qualified_nodes().len(), 2);
            let flat = flatten_netlist_with_models(&netlist).unwrap();
            let values: Vec<_> = if directive == ".IC" {
                flat.scoped_initial_conditions
                    .iter()
                    .map(|hint| hint.voltage)
                    .collect()
            } else {
                flat.scoped_node_sets
                    .iter()
                    .map(|hint| hint.voltage)
                    .collect()
            };
            assert_eq!(values, [2.0, 3.0]);
            if directive == ".IC" {
                let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
                for (node, value) in [("a", 2.0), ("b", 3.0)] {
                    assert_eq!(result.try_voltage_waveform_named(node).unwrap()[0], value);
                }
            } else {
                let result = Engine::default().run_dc_op(&netlist).unwrap();
                assert_eq!(result.try_voltage_named("a"), Some(0.0));
                assert_eq!(result.try_voltage_named("b"), Some(0.0));
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn invalid_hint_values_cannot_be_hidden_by_later_assignments_or_unused_nodes() {
    for directive in [".IC", ".NODESET"] {
        for value in ["{1/(TEMP-85)}", "{1/(TEMP-TEMP)}", "{1+}", "1e999"] {
            for later in [" V(out)=2", "\n.IC V(out)=2", ""] {
                let source =
                    format!("Invalid hint\n{directive} V(out)={value}{later}\n.temp 85\n.end\n");
                let error = Netlist::parse(&source).unwrap_err();
                assert!(error.to_string().contains("line 2"), "{error}");
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn device_directive_recovery_preserves_typed_duplicate_and_value_errors() {
    use rspice_core::netlist::{DeviceInitialConditionError, ParseError};
    let prefix = "Device errors\nR1 out 0 1k\nC1 out 0 1u\n";
    let duplicate = Netlist::parse(&format!(
        "{prefix}.initcond C1 IC={{58/(TEMP-27)}}\n.initcond C1 IC=2\n.temp 85\n.end\n"
    ))
    .unwrap_err();
    assert!(
        matches!(duplicate, ParseError::DeviceInitialCondition(ref error) if matches!(**error, DeviceInitialConditionError::DuplicateDirective { .. })),
        "{duplicate}"
    );
    for value in ["{1/(TEMP-85)}", "{1+}", "1e999"] {
        let error = Netlist::parse(&format!(
            "{prefix}.initcond C1 IC={value}\n.temp 85\n.end\n"
        ))
        .unwrap_err();
        assert!(
            matches!(error, ParseError::DeviceInitialCondition(_)),
            "{error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn recovered_hints_preserve_newly_reachable_statistical_draws() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    for directive in [".IC", ".NODESET"] {
        for first in ["1/(TNOM-27)+aunif(0,1)", "aunif(0,1)+1/(TNOM-27)"] {
            let source = format!(
                "Startup sampling\n.options seed=37\nR1 out 0 1k\nR2 other 0 1k\n{directive} V(out)={{{first}}} V(other)={{aunif(0,1)}}\n.options tnom={{nominal}}\n.param nominal={{55+aunif(0,1)}} tail={{aunif(0,1)}}\n.end\n"
            );
            let netlist = Netlist::parse(&source).unwrap();
            let mut expected = ParamContext::new();
            expected.set_random_seed(37);
            let draw = || eval_expression("aunif(0,1)", &expected).unwrap();
            let first = draw();
            let second = draw();
            let nominal = 55.0 + draw();
            assert_eq!(netlist.options.tnom, Some(nominal));
            assert_eq!(
                hint_value(&netlist, directive, 0),
                first + 1.0 / (nominal - 27.0)
            );
            assert_eq!(hint_value(&netlist, directive, 1), second);
            assert_eq!(netlist.params.get("tail"), Some(draw()));
            assert_eq!(
                eval_expression("aunif(0,1)", &netlist.params).unwrap(),
                draw()
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn device_value_lists_retain_source_order_and_exact_sampling() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse("Device sample list\n.options seed=37\n.initcond Q1 IC={1/(TNOM-27)+aunif(0,1)},{aunif(0,1)} C1 IC=2\n.options tnom={nominal}\n.param nominal={55+aunif(0,1)} tail={aunif(0,1)}\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let draw = || eval_expression("aunif(0,1)", &expected).unwrap();
    let first = draw();
    let second = draw();
    let nominal = 55.0 + draw();
    let entries = &netlist.device_initial_conditions.as_ref().unwrap().entries;
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].values, [first + 1.0 / (nominal - 27.0), second]);
    assert_eq!(entries[1].values, [2.0]);
    assert_eq!(netlist.params.get("tail"), Some(draw()));
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        draw()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inactive_and_unconfirmed_startup_cards_cannot_select_temperatures() {
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        let inactive = Netlist::parse(&format!(
            "Inactive startup\n.if 0\n{directive}={{1/0}}\n.endif\n.temp 85\n.end\n"
        ))
        .unwrap();
        assert!(inactive.startup_directives().is_empty());
        assert!(inactive.device_initial_conditions.is_none());
        for temperature in [".if 0\n.temp 85\n.endif", ".temp {if(TEMP==27,85,27)}"] {
            assert!(
                Netlist::parse(&format!(
                    "Unconfirmed startup\n{directive}={{58/(TEMP-27)}}\n{temperature}\n.end\n"
                ))
                .is_err()
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn included_continuation_errors_keep_the_physical_card_origin() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/rspice-startup/root.cir"
    } else {
        "/rspice-startup/root.cir"
    });
    let child = root.with_file_name("startup.inc");
    let source = "Included startup\n.include startup.inc\n.temp 85\n.end\n";
    for card in [
        ".IC V(first)=2\n+ V(out)={1/(TEMP-85)}",
        ".NODESET V(first)=2\n+ V(out)={1/(TEMP-85)}",
        ".INITCOND C1 IC=2\n+ C2 IC={1/(TEMP-85)}",
    ] {
        let bundle = SealedSourceBundle::try_new_with_edges(
            [
                (root.clone(), source.to_owned()),
                (child.clone(), format!("* startup\n{card}\n")),
            ],
            [SealedSourceEdge {
                owner: root.clone(),
                requested_path: "startup.inc".into(),
                target: child.clone(),
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
        assert!(error.to_string().contains("startup.inc:2"), "{error}");
        assert!(error.to_string().contains("Division by zero"), "{error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn resource_errors_take_priority_over_retained_startup_failures() {
    use rspice_core::netlist::{NetlistParseOptions, ParseError};
    for card in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        let mut options = NetlistParseOptions::default();
        options.resource_limits.max_analysis_points = 2;
        let error = Netlist::parse_with_options(&format!("Startup resource priority\n{card}={{58/(TEMP-27)}}\n.data samples x\n1\n2\n3\n.enddata\n.temp 85\n.end\n"), options).unwrap_err();
        assert!(matches!(error, ParseError::ResourceLimit(_)), "{error}");
    }
}
