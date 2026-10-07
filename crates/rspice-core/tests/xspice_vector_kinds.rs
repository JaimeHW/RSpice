use rspice_core::{Engine, Netlist};

fn deck(instance: &str, definitions: &str, scoped: bool) -> String {
    let body = if scoped {
        format!(".SUBCKT cell in out\n{instance}\n.ENDS\nX1 in out cell")
    } else {
        instance.to_owned()
    };
    format!("* deferred vector types\nV1 in 0 .5\n{body}\n{definitions}\n.END\n")
}

#[test]
fn bare_numeric_vectors_use_forward_names_in_both_scopes() {
    for (vector, definitions, expected) in [
        ("[scale {scale*2}]", ".PARAM scale=2", 3.0),
        (
            "[value(1, 2) {value(1,2)+1}]",
            ".FUNC value(a,b) {a+b-1}",
            2.5,
        ),
        ("[sqrt(4) max(2, 4)]", "", 3.0),
        ("[real(2) imag(0)]", "", 1.0),
        ("[aunif(2, 0) gauss(4, 0)]", "", 3.0),
    ] {
        for quoted in [false, true] {
            for scoped in [false, true] {
                let vector = if quoted {
                    format!("\"{vector}\"")
                } else {
                    vector.to_owned()
                };
                let source = deck(
                    &format!("A1 in out lookup y_array={vector}"),
                    &format!(
                        ".MODEL lookup pwl(x_array=[0 1] input_domain=.01 fraction=false)\n{definitions}"
                    ),
                    scoped,
                );
                let actual = Engine::default()
                    .run_dc_op(&Netlist::parse(&source).unwrap())
                    .unwrap()
                    .try_voltage_named("out")
                    .unwrap();
                assert!((actual - expected).abs() < 1e-12, "{source}: {actual}");
            }
        }
    }
}

#[test]
fn vector_literals_and_aliases_see_sibling_and_model_numeric_fields() {
    for scoped in [false, true] {
        for model_default in [false, true] {
            for alias in [false, true] {
                for alias_first in [false, true] {
                    let vector = if alias {
                        "{payload}"
                    } else {
                        "[input_domain 1]"
                    };
                    let override_field = if model_default {
                        ""
                    } else {
                        "input_domain=.01"
                    };
                    let alias_card = ".PARAM payload=\"[input_domain 1]\"";
                    let source = deck(
                        &format!("A1 in out lookup y_array={vector} {override_field}"),
                        &format!(
                            ".MODEL lookup pwl(x_array=[0 1] {} fraction=false)\n{}",
                            if model_default {
                                "input_domain=.01"
                            } else {
                                ""
                            },
                            if !alias_first { alias_card } else { "" }
                        ),
                        scoped,
                    )
                    .replace(
                        "V1 in 0 .5",
                        &format!("{}\nV1 in 0 .5", if alias_first { alias_card } else { "" }),
                    );
                    let actual = Engine::default()
                        .run_dc_op(&Netlist::parse(&source).unwrap())
                        .unwrap()
                        .try_voltage_named("out")
                        .unwrap();
                    assert!((actual - 0.505).abs() < 1e-12, "{source}: {actual}");
                }
            }
        }
    }
}

#[test]
fn literal_word_vectors_keep_quoting_and_adjacency_in_subcircuits() {
    use rspice_core::netlist::{ElementKind, flatten_netlist_with_models};
    for (vector, expected) in [
        ("[alpha beta]", vec!["alpha", "beta"]),
        ("[hello(world) next]", vec!["hello(world)", "next"]),
        (
            "\"[alpha-beta path/to/file]\"",
            vec!["alpha-beta", "path/to/file"],
        ),
        (
            "[alpha \"[nested]\" \"two words\"]",
            vec!["alpha", "[nested]", "two words"],
        ),
        ("[\"alpha beta\" \"gamma\"]", vec!["alpha beta", "gamma"]),
    ] {
        for scoped in [false, true] {
            let source = deck(
                &format!("A1 [in] print_param_types string_array={vector}"),
                "",
                scoped,
            );
            let netlist = Netlist::parse(&source).unwrap();
            Engine::default().build_circuit(&netlist).unwrap();
            if scoped {
                let flat = flatten_netlist_with_models(&netlist).unwrap();
                let strings = flat
                    .elements
                    .iter()
                    .find_map(|element| match &element.kind {
                        ElementKind::Xspice {
                            string_vector_params,
                            ..
                        } => Some(&string_vector_params[0].1),
                        _ => None,
                    })
                    .unwrap();
                assert_eq!(strings, &expected, "{source}");
            }
        }
    }
}

#[test]
fn deferred_word_classification_does_not_sample_other_fields() {
    use rspice_core::netlist::{ParamContext, expr::eval_expression};
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for scoped in [false, true] {
        let source = deck(
            "A1 [in] print_param_types string_array=[alpha beta]",
            ".PARAM marker={aunif(100,1)}",
            scoped,
        )
        .replace("V1 in 0 .5", ".OPTIONS SEED=37\nV1 in 0 .5");
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.params.get("marker"), Some(expected));
        Engine::default().build_circuit(&netlist).unwrap();
    }
}

#[test]
fn ambiguous_vectors_reject_unclosed_nested_and_trailing_syntax() {
    for vector in [
        "[later",
        "[later [1]]",
        "\"[later] junk\"",
        "\"[later] [2]\"",
    ] {
        for scoped in [false, true] {
            let source = deck(
                &format!("A1 [in] print_param_types real_array={vector}"),
                ".PARAM later=1",
                scoped,
            );
            assert!(Netlist::parse(&source).is_err(), "{source}");
        }
    }
}

#[test]
fn ambiguous_vector_capture_and_binding_preserve_typed_cancellation() {
    use rspice_core::{SimulationError, abort_signal::CountingAbort, netlist::ParseWithAbortError};
    for scoped in [false, true] {
        let source = deck(
            "A1 [in] print_param_types string_array=[alpha \"two words\" beta]",
            "",
            scoped,
        );
        let count = CountingAbort::new(usize::MAX);
        let netlist = Netlist::parse_with_abort(&source, &count).unwrap();
        for limit in 0..count.count() {
            let abort = CountingAbort::new(limit);
            assert!(
                matches!(
                    Netlist::parse_with_abort(&source, &abort),
                    Err(ParseWithAbortError::Aborted)
                ),
                "parse limit={limit}, scoped={scoped}"
            );
            assert_eq!(abort.count(), limit + 1);
            assert_eq!(abort.polls_after_abort(), 0);
        }
        let count = CountingAbort::new(usize::MAX);
        Engine::default()
            .build_circuit_with_abort(&netlist, &count)
            .unwrap();
        for limit in 0..count.count() {
            let abort = CountingAbort::new(limit);
            let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
            assert!(
                matches!(result, Err(SimulationError::Aborted)),
                "build limit={limit}, scoped={scoped}: {:?}",
                result.err()
            );
            assert_eq!(abort.count(), limit + 1);
            assert_eq!(abort.polls_after_abort(), 0);
        }
        Engine::default().build_circuit(&netlist).unwrap();
    }
}

#[test]
fn vector_classification_respects_string_shadowing_and_numeric_overrides() {
    for vector in ["[value]", "\"[value]\""] {
        for default in ["1", "\"default\""] {
            let source = format!(
                "* lexical type shadowing\n.PARAM value=1\n.SUBCKT cell in PARAMS: value={default}\nA1 [in] print_param_types string_array={vector}\n.ENDS\nX1 in cell value=\"text\"\n.END\n"
            );
            Engine::default()
                .build_circuit(&Netlist::parse(&source).unwrap())
                .unwrap();
        }
    }
    for scoped in [false, true] {
        let source = deck(
            "A1 [in] alias string_array=[real]",
            ".MODEL alias print_param_types(real=4)",
            scoped,
        )
        .replace("V1 in 0 .5", ".PARAM real=\"text\"\nV1 in 0 .5");
        Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .unwrap();
        let source = source.replace("string_array=[real]", "real_array=[real] real=5");
        Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .unwrap();
    }
}

#[test]
fn unmaterialized_retained_names_still_classify_as_numeric_vectors() {
    for scoped in [false, true] {
        let source = deck(
            "A1 in out lookup y_array=[bound {bound+1}]",
            ".MODEL lookup pwl(x_array=[0 1] input_domain=.01 fraction=false)",
            scoped,
        )
        .replace("V1 in 0 .5", ".OPTIONS TEMP=47\nV1 in 0 .5");
        let mut netlist = Netlist::parse(&source).unwrap();
        // A public ParamContext definition need not carry a cached projection.
        netlist
            .params
            .define_global_expression("bound", "TEMP+1", None);
        let actual = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((actual - 48.5).abs() < 1e-12, "{source}: {actual}");
    }
}

#[test]
fn vector_alias_fields_are_not_mistaken_for_scalar_bindings() {
    for scoped in [false, true] {
        for fields in [
            "string_array=[real_array] real_array={payload}",
            "real_array={payload} string_array=[real_array]",
        ] {
            let source = deck(
                &format!("A1 [in] print_param_types {fields}"),
                ".PARAM payload=\"[1 2]\"",
                scoped,
            );
            Engine::default()
                .build_circuit(&Netlist::parse(&source).unwrap())
                .unwrap();
        }
    }
}
