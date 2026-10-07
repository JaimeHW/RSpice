use rspice_core::netlist::{
    ElementKind, ParamContext, expr::eval_expression, flatten_netlist_with_models,
};
use rspice_core::{Engine, Netlist};

#[test]
fn real_vector_aliases_work_before_and_after_the_instance_in_both_scopes() {
    for alias_first in [false, true] {
        for scoped in [false, true] {
            for spelling in ["payload", "{payload}"] {
                let card = ".PARAM payload=\"[0 {scale}]\"";
                let mut body = format!("A1 in out lookup y_array={spelling}");
                if scoped {
                    body = format!(
                        ".SUBCKT cell in out PARAMS: scale=2\n{body}\n.ENDS\nX1 in out cell scale=6"
                    );
                }
                let source = format!(
                    "* vector alias\n.PARAM scale=2\n.MODEL lookup pwl(x_array=[0 1] input_domain=.01 fraction=false)\nV1 in 0 .5\n{}\n{body}\n{}\n.END\n",
                    if alias_first { card } else { "" },
                    if alias_first { "" } else { card }
                );
                let actual = Engine::default()
                    .run_dc_op(&Netlist::parse(&source).unwrap())
                    .unwrap()
                    .try_voltage_named("out")
                    .unwrap();
                let expected = if scoped { 3.0 } else { 1.0 };
                assert!((actual - expected).abs() < 1e-12, "{source}: {actual}");
            }
        }
    }
}

#[test]
fn deferred_aliases_preserve_numeric_string_and_complex_vector_types() {
    for (field, payload) in [
        ("real_array", "[1 {value()}]"),
        ("integer_array", "[1 {value()}]"),
        ("string_array", "[alpha beta]"),
        ("complex_array", "[<value() 3>]"),
    ] {
        for scoped in [false, true] {
            let mut body = format!("A1 [in] print_param_types {field}={{payload}}");
            if scoped {
                body = format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell");
            }
            let source = format!(
                "* typed vector alias\n{body}\n.PARAM payload=\"{payload}\"\n.FUNC value() {{2}}\n.END\n"
            );
            Engine::default()
                .build_circuit(&Netlist::parse(&source).unwrap())
                .unwrap();
        }
    }
}

#[test]
fn aliased_complex_entries_use_the_instance_scope_and_seeded_stream() {
    let source = "* scoped complex aliases\n.OPTIONS SEED=37\n.PARAM payload=\"[<sample() {level}>]\"\n.FUNC sample() {aunif(100,1)}\n.SUBCKT cell in PARAMS: level=0\nA1 [in] print_param_types complex_array={payload}\n.ENDS\nX1 in cell level=2\nX2 in cell level=4\n.END\n";
    let netlist = Netlist::parse(source).unwrap();
    let flattened = flatten_netlist_with_models(&netlist).unwrap();
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    for (element, level) in flattened.elements.iter().zip([2, 4]) {
        let ElementKind::Xspice {
            string_vector_params,
            ..
        } = &element.kind
        else {
            panic!("expected XSPICE")
        };
        let expected = eval_expression("aunif(100,1)", &reference).unwrap();
        assert_eq!(string_vector_params[0].1, [format!("<{expected} {level}>")]);
    }
    assert_eq!(flattened.elements.len(), 2);
    Engine::default().build_circuit(&netlist).unwrap();
}

#[test]
fn malformed_alias_vectors_are_rejected_after_scope_resolution() {
    for payload in ["[1 2] trailing", "[1 2] [3]", "[1 {missing}]"] {
        for scoped in [false, true] {
            let mut body = String::from("A1 [in] print_param_types real_array={payload}");
            if scoped {
                body = format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell");
            }
            let source = format!("* invalid alias\n{body}\n.PARAM payload=\"{payload}\"\n.END\n");
            let netlist = Netlist::parse(&source).unwrap();
            assert!(
                Engine::default().build_circuit(&netlist).is_err(),
                "{source}"
            );
        }
    }
}

#[test]
fn deferred_alias_evaluation_preserves_typed_cancellation() {
    for (field, payload) in [
        ("real_array", "[0 {f18()}]"),
        ("complex_array", "[<1 f18()>]"),
    ] {
        for scoped in [false, true] {
            let mut body = format!("A1 [in] print_param_types {field}={{payload}}");
            if scoped {
                body = format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell");
            }
            let mut netlist = Netlist::parse(&format!(
                "* alias cancellation\n{body}\n.PARAM payload=\"{payload}\"\n.END\n"
            ))
            .unwrap();
            netlist.params.define_function("f0", vec![], "1");
            for index in 1..=18 {
                netlist.params.define_function(
                    &format!("f{index}"),
                    vec![],
                    &format!("f{}()+f{}()", index - 1, index - 1),
                );
            }
            let abort = rspice_core::abort_signal::CountingAbort::new(1024);
            let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
            assert!(
                matches!(result, Err(rspice_core::SimulationError::Aborted)),
                "{body}: {:?}",
                result.err()
            );
            assert_eq!(abort.count(), 1025);
            assert_eq!(abort.polls_after_abort(), 0);
        }
    }
}

#[test]
fn alias_parsing_and_binding_preserve_cancellation_at_every_poll() {
    use rspice_core::abort_signal::CountingAbort;
    use rspice_core::netlist::{ParseWithAbortError, flatten_netlist_with_models_with_abort};
    let netlist = Netlist::parse("* alias parse boundaries\n.PARAM payload=\"[<min(2,3) 4> <5 6>]\"\n.SUBCKT cell in\nA1 [in] print_param_types complex_array={payload}\n.ENDS\nX1 in cell\n.END\n").unwrap();
    let count = CountingAbort::new(usize::MAX);
    let expected = flatten_netlist_with_models_with_abort(&netlist, &count).unwrap();
    for limit in 0..count.count() {
        let abort = CountingAbort::new(limit);
        let result = flatten_netlist_with_models_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "limit {limit}: {:?}",
            result.err()
        );
        assert_eq!(abort.count(), limit + 1);
        assert_eq!(abort.polls_after_abort(), 0);
    }
    let actual = flatten_netlist_with_models(&netlist).unwrap();
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
