//! Startup operands bind in their authored scope before circuit preparation.
use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{NetlistParseOptions, flatten_netlist_with_models};
use rspice_core::{Engine, Netlist};

fn parse(source: &str, expression_dialect: ExpressionDialect) -> Netlist {
    Netlist::parse_with_options(
        source,
        NetlistParseOptions {
            expression_dialect,
            ..Default::default()
        },
    )
    .unwrap_or_else(|error| panic!("{expression_dialect:?}: {error}\n{source}"))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_forward_initial_conditions_reach_the_first_transient_sample() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for directive in [".IC V(out)=", ".INITCOND C1 IC="] {
            for operand in ["{level}", "level", "+level"] {
                let source = format!(
                    "Forward startup\nR1 out 0 1k\nC1 out 0 1u\n{directive}{operand}\n.param level={{later/2}}\n.param later=2\n.tran 1u 2u uic\n.end\n"
                );
                let netlist = parse(&source, dialect);
                let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
                assert_eq!(result.try_voltage_waveform_named("out").unwrap()[0], 1.0);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_forward_nodesets_are_materialized_then_released() {
    let netlist = parse(
        "Forward nodeset\nV1 in 0 2\nR1 in out 1k\nR2 out 0 1k\n.nodeset V(out)={level}\n.global_param level={later}\n.param later=7\n.end\n",
        ExpressionDialect::Ngspice,
    );
    assert_eq!(netlist.node_sets[0].voltage, 7.0);
    assert_eq!(netlist.startup_directives()[0].entries()[0].voltage(), 7.0);
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    assert!((result.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-9);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_cards_capture_known_values_functions_and_authored_order() {
    for directive in [".IC", ".NODESET"] {
        let source = format!(
            "Captured hints\nR1 a 0 1k\nR2 b 0 1k\nR3 c 0 1k\n.param gain=2\n.func scale(x) {{gain*x}}\n{directive} V(a)={{scale(later)}}\n{directive} V(b)=3\n.param gain=20\n.func scale(x) {{100*x}}\n{directive} V(c)={{later+gain}}\n.param later=4\n.end\n"
        );
        let netlist = parse(&source, ExpressionDialect::Ngspice);
        let records = netlist.startup_directives();
        assert_eq!(
            records
                .iter()
                .map(|record| record.origin().line)
                .collect::<Vec<_>>(),
            [7, 8, 11]
        );
        assert_eq!(
            records
                .iter()
                .map(|record| record.entries()[0].voltage())
                .collect::<Vec<_>>(),
            [8.0, 3.0, 24.0]
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nested_forward_parent_bindings_preserve_instance_overrides() {
    for directive in [".IC", ".NODESET"] {
        let source = format!(
            "Nested hints\n.subckt parent p params: gain=1\n.subckt child q\n{directive} V(q)={{level*gain}}\nR1 q 0 1k\nC1 q 0 1u\n.ends\n.param level={{base+1}}\n.param base=2\nXinner p child\n.ends\nX1 a parent gain=2\nX2 b parent gain=3\n.tran 1u 2u uic\n.end\n"
        );
        let netlist = parse(&source, ExpressionDialect::Ngspice);
        assert_eq!(
            netlist.startup_directives()[0].entries()[0]
                .qualified_nodes()
                .len(),
            2
        );
        let flat = flatten_netlist_with_models(&netlist).unwrap();
        let values: Vec<_> = if directive == ".IC" {
            flat.scoped_initial_conditions
                .iter()
                .map(|entry| entry.voltage)
                .collect()
        } else {
            flat.scoped_node_sets
                .iter()
                .map(|entry| entry.voltage)
                .collect()
        };
        assert_eq!(values, [6.0, 9.0]);
        if directive == ".IC" {
            let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
            assert_eq!(result.try_voltage_waveform_named("a").unwrap()[0], 6.0);
            assert_eq!(result.try_voltage_waveform_named("b").unwrap()[0], 9.0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn instance_only_startup_parameters_remain_valid() {
    for directive in [".IC", ".NODESET"] {
        let source = format!(
            "Instance-only hint\n.subckt cell p\n{directive} V(p)={{level}}\nR1 p 0 1k\nC1 p 0 1u\n.ends\nX1 out cell level=2\n.tran 1u 2u uic\n.end\n"
        );
        let netlist = parse(&source, ExpressionDialect::Ngspice);
        let flat = flatten_netlist_with_models(&netlist).unwrap();
        if directive == ".IC" {
            assert_eq!(flat.scoped_initial_conditions[0].voltage, 2.0);
            let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
            assert_eq!(result.try_voltage_waveform_named("out").unwrap()[0], 2.0);
        } else {
            assert_eq!(flat.scoped_node_sets[0].voltage, 2.0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn device_cards_use_their_lexical_parent_and_preserve_duplicate_priority() {
    use rspice_core::netlist::{DeviceInitialConditionError, ParseError};
    let netlist = parse(
        "Scoped device card\nC1 out 0 1u\nR1 out 0 1k\n.subckt parent p\n.subckt child q\n.initcond C1 IC={level}\nR2 q 0 1k\n.ends\n.param level={base+1}\n.param base=2\n.ends\n.end\n",
        ExpressionDialect::Ngspice,
    );
    assert_eq!(
        netlist.device_initial_conditions.as_ref().unwrap().entries[0].values,
        [3.0]
    );
    for first in ["1", "{later}"] {
        for second in ["C1 IC=2", "C1 IC={missing}", "FILE"] {
            let source = format!(
                "Duplicate device cards\n.initcond C1 IC={first}\n.initcond {second}\n.param later=1\n.end\n"
            );
            let error = Netlist::parse(&source).unwrap_err();
            assert!(
                matches!(error, ParseError::DeviceInitialCondition(ref error) if matches!(**error, DeviceInitialConditionError::DuplicateDirective { ref first, ref duplicate } if first.line == 2 && duplicate.line == 3)),
                "{error}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_failures_keep_physical_ownership_even_when_overwritten() {
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        for value in ["missing", "later/0", "later+"] {
            let source = format!(
                "Invalid forward startup\n{directive}={{{value}}}\n{directive}=2\n.param later=1\n.end\n"
            );
            let error = Netlist::parse(&source).unwrap_err();
            assert!(error.to_string().contains("line 2"), "{error}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn deferred_cards_sample_after_declarations_without_repeating_operands() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        let source = format!(
            "Deferred startup samples\n.options seed=37\nR1 out 0 1k\nC1 out 0 1u\n{directive}={{aunif(0,1)+level+aunif(0,1)}}\n.param level={{base+aunif(0,1)}}\n.param base=2 tail={{aunif(0,1)}}\n.end\n"
        );
        let netlist = parse(&source, ExpressionDialect::Ngspice);
        let mut expected = ParamContext::new();
        expected.set_random_seed(37);
        let draw = || eval_expression("aunif(0,1)", &expected).unwrap();
        let tail = draw();
        let level = 2.0 + draw();
        let value = draw() + level + draw();
        assert_eq!(netlist.params.get("tail"), Some(tail));
        assert_eq!(netlist.params.get("level"), Some(level));
        let actual = if directive.starts_with(".INITCOND") {
            netlist.device_initial_conditions.as_ref().unwrap().entries[0].values[0]
        } else {
            netlist.startup_directives()[0].entries()[0].voltage()
        };
        assert_eq!(actual, value);
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            draw()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn mixed_declaration_failures_preserve_the_first_invalid_card() {
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        for value in ["missing", "later/0", "later+"] {
            let source = format!(
                "Mixed failures\n{directive}={{{value}}}\n.param bad={{1/0}} later=1\n.end\n"
            );
            let error = Netlist::parse(&source).unwrap_err();
            assert!(error.to_string().contains("line 2"), "{error}");
        }
        let source =
            format!("Earlier declaration\n.param bad={{1/0}}\n{directive}={{missing}}\n.end\n");
        let error = Netlist::parse(&source).unwrap_err();
        assert!(error.to_string().contains("line 2"), "{error}");
        let source = format!(
            "Valid forward card\n{directive}={{later}}\n.param bad={{1/0}} later=1\n.end\n"
        );
        let error = Netlist::parse(&source).unwrap_err();
        assert!(error.to_string().contains("line 3"), "{error}");
        for scope in ["", ".subckt cell p\n"] {
            let expected_line = if scope.is_empty() { 2 } else { 3 };
            let source =
                format!("Early startup failure\n{scope}{directive}={{1/0}}\n.model\n.end\n");
            let error = Netlist::parse(&source).unwrap_err();
            assert!(
                error.to_string().contains(&format!("line {expected_line}")),
                "{error}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn forward_binding_and_error_preference_stop_at_every_cancellation_boundary() {
    use rspice_core::abort_signal::CountingAbort;
    use rspice_core::netlist::ParseWithAbortError;
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        for failed in [false, true] {
            let value = if failed { "missing" } else { "level" };
            let tail = if failed {
                ".param bad={1/0}\n"
            } else {
                ".temp {ambient}\n.param ambient=85\n"
            };
            let source = format!(
                "Forward startup cancellation\nR1 out 0 1k\nC1 out 0 1u\n{directive}={{aunif(1,.1)+{value}}}\n.param level={{base+1}}\n.param base=2\n{tail}.end\n"
            );
            let mut finished = false;
            for limit in 0..2048 {
                let abort = CountingAbort::new(limit);
                let result = Netlist::parse_with_abort(&source, &abort);
                assert_eq!(abort.polls_after_abort(), 0, "{directive}, limit={limit}");
                match result {
                    Err(ParseWithAbortError::Aborted) => {}
                    Err(ParseWithAbortError::Parse(error)) if failed => {
                        assert!(error.to_string().contains("line 4"), "{error}");
                        finished = true;
                        break;
                    }
                    Ok(netlist) if !failed => {
                        assert_eq!(netlist.options.temp, Some(85.0));
                        finished = true;
                        break;
                    }
                    result => panic!("unexpected result: {result:?}"),
                }
            }
            assert!(finished);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn included_forward_cards_keep_continuation_origins_and_selected_temperature() {
    use rspice_core::netlist::{SealedSourceBundle, SealedSourceEdge};
    let root = std::path::PathBuf::from(if cfg!(windows) {
        "C:/forward-startup/root.cir"
    } else {
        "/forward-startup/root.cir"
    });
    let child = root.with_file_name("startup.inc");
    let source = "Included forward startup\nR1 out 0 1k\nC1 out 0 1u\n.include startup.inc\n.param level=58\n.temp 85\n.end\n";
    for directive in [".IC V(out)", ".NODESET V(out)", ".INITCOND C1 IC"] {
        for denominator in ["TEMP-27", "TEMP-85"] {
            let bundle = SealedSourceBundle::try_new_with_edges(
                [
                    (root.clone(), source.into()),
                    (
                        child.clone(),
                        format!("* included startup\n{directive}=\n+ {{level/({denominator})}}\n"),
                    ),
                ],
                [SealedSourceEdge {
                    owner: root.clone(),
                    requested_path: "startup.inc".into(),
                    target: child.clone(),
                }],
            )
            .unwrap();
            let result = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                source,
                &root,
                bundle,
                Default::default(),
                &rspice_core::NoAbort,
            );
            if denominator == "TEMP-85" {
                let error = result.unwrap_err();
                assert!(error.to_string().contains("startup.inc:2"), "{error}");
            } else {
                let netlist = result.unwrap();
                if let Some(device) = netlist.device_initial_conditions {
                    assert_eq!(device.origin.line, 2);
                    assert_eq!(device.origin.path.as_ref(), Some(&child));
                    assert_eq!(device.entries[0].values, [1.0]);
                } else {
                    let record = &netlist.startup_directives()[0];
                    assert_eq!(record.origin().line, 2);
                    assert_eq!(record.origin().path.as_ref(), Some(&child));
                    assert_eq!(record.entries()[0].voltage(), 1.0);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn queued_scoped_cards_update_the_registry_and_nested_definition_copies() {
    for directive in [".IC", ".NODESET"] {
        let source = format!(
            "Queued scoped hints\nRtop top 0 1k\nCtop top 0 1u\n{directive} V(top)={{level}}\n.subckt parent p\n.subckt child q\n{directive} V(q)={{gain}}\nR1 q 0 1k\nC1 q 0 1u\n.ends\nXinner p child gain=2\n.ends\nX1 out parent\n.param level=1\n.tran 1u 2u uic\n.end\n"
        );
        let netlist = parse(&source, ExpressionDialect::Ngspice);
        let parent = netlist
            .subcircuits
            .iter()
            .find(|definition| definition.name.eq_ignore_ascii_case("parent"))
            .unwrap();
        let nested = &parent.nested_subcircuits[0];
        assert_eq!(nested.initial_conditions.len() + nested.node_sets.len(), 1);
        let records = netlist.startup_directives();
        assert_eq!(
            records
                .iter()
                .map(|record| record.origin().line)
                .collect::<Vec<_>>(),
            [4, 7]
        );
        if directive == ".IC" {
            let result = Engine::default().run_tran(&netlist, 2e-6, 1e-6).unwrap();
            assert_eq!(result.try_voltage_waveform_named("top").unwrap()[0], 1.0);
            assert_eq!(result.try_voltage_waveform_named("out").unwrap()[0], 2.0);
        } else {
            let flat = flatten_netlist_with_models(&netlist).unwrap();
            assert_eq!(netlist.node_sets[0].voltage, 1.0);
            assert_eq!(flat.scoped_node_sets[0].voltage, 2.0);
        }
    }
}
