use rspice_core::{Engine, Netlist};

fn deck(fields: &str, definitions: &str, scoped: bool) -> String {
    let mut body = format!("A1 in out gain {fields}");
    if scoped {
        body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
    }
    format!(
        "* retained sibling bindings\n.OPTIONS SEED=37\nV1 in 0 1\n{body}\n{definitions}\n.END\n"
    )
}

#[test]
fn retained_global_definitions_do_not_freeze_pending_sibling_bindings() {
    for fields in [
        "gain={in_offset+1} in_offset={later}",
        "gain={field_gain()} in_offset={later}",
        "in_offset={later} gain={in_offset+1}",
        "gain={in_offset+1} in_offset={in_offset+2}",
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3", scoped).replace(
                ".OPTIONS SEED=37",
                ".OPTIONS SEED=37\n.PARAM base=1\n.GLOBAL_PARAM in_offset={base}\n.FUNC field_gain() {in_offset+1}",
            );
            let actual = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap()
                .try_voltage_named("out")
                .unwrap();
            assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
        }
    }
}

#[test]
fn transitive_retained_reads_use_completed_instance_fields_in_both_scopes() {
    for fields in [
        "gain={dependent} in_offset=3",
        "gain={dependent} in_offset={later}",
        "in_offset={later} gain={dependent}",
        "gain={read_gain()} in_offset={later}",
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3", scoped).replace(
                ".OPTIONS SEED=37",
                ".OPTIONS SEED=37\n.PARAM in_offset=1\n.GLOBAL_PARAM inner={in_offset+1}\n.GLOBAL_PARAM dependent={inner}\n.FUNC read_gain() {dependent}",
            );
            let actual = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap()
                .try_voltage_named("out")
                .unwrap();
            assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
        }
    }
}

#[test]
fn transitive_retained_sibling_probes_preserve_random_draws_and_cycles() {
    use rspice_core::netlist::{ParamContext, expr::eval_expression};
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for scoped in [false, true] {
        let source = deck(
            "gain={aunif(100,1)+dependent} in_offset={later}",
            ".PARAM later=0",
            scoped,
        )
        .replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.PARAM in_offset=1\n.GLOBAL_PARAM dependent={in_offset}",
        );
        let actual = Engine::default()
            .run_dc_op(&Netlist::parse(&source).unwrap())
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!(
            (actual - expected).abs() < 1e-10,
            "{source}: {actual} != {expected}"
        );
        let source = source.replace("in_offset={later}", "in_offset={gain}");
        let error = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .map(|_| ())
            .expect_err("transitive cycle")
            .to_string();
        assert!(error.contains("could not be resolved"), "{source}: {error}");
    }
}

#[test]
fn retained_vector_entries_follow_scalar_overrides_in_both_scopes() {
    for scoped in [false, true] {
        let mut body = "A1 in out lookup y_array=[0 {dependent}] input_domain=.01".to_owned();
        if scoped {
            body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
        }
        let source = format!(
            "* retained vector\n.PARAM input_domain=1\n.GLOBAL_PARAM dependent={{input_domain*2}}\n.MODEL lookup pwl(x_array=[0 1] fraction=false)\nV1 in 0 .5\n{body}\n.END\n"
        );
        let actual = Engine::default()
            .run_dc_op(&Netlist::parse(&source).unwrap())
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((actual - 0.01).abs() < 1e-12, "{source}: {actual}");
    }
}

#[test]
fn retained_scalar_dependencies_preserve_complex_values() {
    for scoped in [false, true] {
        for (expression, expected) in [
            ("RE(dependent)", Some(2.0)),
            ("IMAG(dependent)", Some(3.0)),
            ("dependent", None),
        ] {
            let source = deck(&format!("gain={{{expression}}}"), "", scoped).replace(
                ".OPTIONS SEED=37",
                ".OPTIONS SEED=37\n.PARAM z={2+3j}\n.GLOBAL_PARAM dependent={z}",
            );
            let result = Netlist::parse(&source)
                .map_err(|e| e.to_string())
                .and_then(|netlist| {
                    Engine::default()
                        .run_dc_op(&netlist)
                        .map_err(|e| e.to_string())
                });
            if let Some(expected) = expected {
                let actual = result.unwrap().try_voltage_named("out").unwrap();
                assert!((actual - expected).abs() < 1e-12, "{source}: {actual}");
            } else {
                let error = result.expect_err("complex gain is invalid");
                assert!(error.contains("real value"), "{source}: {error}");
            }
        }
    }
}

#[test]
fn model_defaults_do_not_freeze_transitive_instance_dependencies() {
    for scoped in [false, true] {
        let source = deck("gain={dependent} in_offset={later}", ".PARAM later=3", scoped)
            .replace("in out gain ", "in out alias ")
            .replace(".OPTIONS SEED=37", ".OPTIONS SEED=37\n.PARAM in_offset=1\n.GLOBAL_PARAM dependent={in_offset+1}\n.MODEL alias gain(in_offset=2)");
        let actual = Engine::default()
            .run_dc_op(&Netlist::parse(&source).unwrap())
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
    }
}

#[test]
fn retained_dependency_evaluation_cancels_and_can_be_retried() {
    use rspice_core::{SimulationError, abort_signal::CountingAbort};
    for scoped in [false, true] {
        let source = deck("gain={f12()+dependent} in_offset=0", "", scoped).replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.PARAM in_offset=1\n.GLOBAL_PARAM dependent={in_offset}",
        );
        let mut netlist = Netlist::parse(&source).unwrap();
        netlist.params.define_function("f0", vec![], "1");
        for index in 1..=12 {
            netlist.params.define_function(
                &format!("f{index}"),
                vec![],
                &format!("f{}()+f{}()", index - 1, index - 1),
            );
        }
        let abort = CountingAbort::new(1024);
        let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(SimulationError::Aborted)),
            "scoped={scoped}: {:?}",
            result.err()
        );
        assert_eq!(abort.count(), 1025);
        assert_eq!(abort.polls_after_abort(), 0);
        let actual = Engine::default()
            .run_dc_op(&netlist)
            .unwrap()
            .try_voltage_named("out")
            .unwrap();
        assert_eq!(actual, 4096.0, "scoped={scoped}");
    }
}
