use rspice_core::{
    Engine, Netlist,
    netlist::{ParamContext, expr::eval_expression},
};

fn deck(fields: &str, definitions: &str, scoped: bool) -> String {
    let mut body = format!("A1 in out gain {fields}");
    if scoped {
        body = format!(".SUBCKT cell in out\n{body}\n.ENDS\nX1 in out cell");
    }
    format!("* scalar dependencies\n.OPTIONS SEED=37\nV1 in 0 1\n{body}\n{definitions}\n.END\n")
}

#[test]
fn scalar_siblings_resolve_in_either_order_at_root_and_in_subcircuits() {
    for fields in [
        "gain={in_offset+1} in_offset={later}",
        "in_offset={later} gain={in_offset+1}",
        "gain={in_offset+1} in_offset=3",
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3", scoped);
            let netlist = Netlist::parse(&source).unwrap();
            let result = Engine::default().run_dc_op(&netlist).unwrap();
            let actual = result.try_voltage_named("out").unwrap();
            assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
        }
    }
}

#[test]
fn failed_sibling_resolution_does_not_consume_statistical_samples() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for fields in [
        "gain={aunif(100,1)+in_offset} in_offset={later}",
        "in_offset={later} gain={aunif(100,1)+in_offset}",
        "gain={sample()+in_offset} in_offset={later}",
    ] {
        for scoped in [false, true] {
            let source = deck(
                fields,
                ".PARAM later=0\n.FUNC sample() {aunif(100,1)}",
                scoped,
            );
            let netlist = Netlist::parse(&source).unwrap();
            let result = Engine::default().run_dc_op(&netlist).unwrap();
            let actual = result.try_voltage_named("out").unwrap();
            assert!(
                (actual - expected).abs() < 1e-10,
                "{source}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn scalar_self_reference_keeps_the_enclosing_binding() {
    for scoped in [false, true] {
        let source = deck("gain={gain*2}", ".PARAM gain=3", scoped);
        let netlist = Netlist::parse(&source).unwrap();
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        assert!(
            (result.try_voltage_named("out").unwrap() - 6.0).abs() < 1e-12,
            "{source}"
        );
    }
}

#[test]
fn unresolved_scalar_cycles_and_invalid_values_are_rejected() {
    for (fields, definitions, diagnostic) in [
        (
            "gain={in_offset} in_offset={gain}",
            "",
            "could not be resolved",
        ),
        (
            "gain={in_offset} in_offset={later}",
            ".PARAM later={2+1j}",
            "real value",
        ),
        (
            "gain={in_offset} in_offset={RE(exp(later))}",
            ".PARAM later=1000",
            "finite",
        ),
    ] {
        for scoped in [false, true] {
            let source = deck(fields, definitions, scoped);
            let netlist = Netlist::parse(&source).unwrap();
            let error = Engine::default()
                .build_circuit(&netlist)
                .map(|_| ())
                .expect_err(&source)
                .to_string();
            assert!(error.contains(diagnostic), "{source}: {error}");
        }
    }
}

#[test]
fn deferred_scalar_evaluation_preserves_typed_cancellation() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for scoped in [false, true] {
        let netlist = Netlist::parse(&deck(
            "gain={f18()+in_offset} in_offset=0",
            &functions,
            scoped,
        ))
        .unwrap();
        let abort = rspice_core::abort_signal::CountingAbort::new(1024);
        let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(rspice_core::SimulationError::Aborted)),
            "scoped={scoped}: {:?}",
            result.err()
        );
        assert_eq!(abort.count(), 1025);
        assert_eq!(abort.polls_after_abort(), 0);
    }
}

#[test]
fn scoped_retained_expressions_resolve_with_instance_bindings() {
    let netlist = Netlist::parse(
        "* retained scalar binding\n.PARAM base=1\n.GLOBAL_PARAM dependent={base*2}\n\
         V1 in 0 1\n.SUBCKT cell in out PARAMS: base=0\n\
         A1 in out gain gain={dependent+in_offset} in_offset={later}\n\
         .ENDS\nX1 in out cell base=3\n.PARAM later=0\n.END\n",
    )
    .unwrap();
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    let actual = result.try_voltage_named("out").unwrap();
    assert!((actual - 6.0).abs() < 1e-12, "{actual} != 6");
}

#[test]
fn explicit_pending_fields_shadow_model_defaults_in_sibling_expressions() {
    for fields in [
        "gain={in_offset+1} in_offset={later}",
        "in_offset={later} gain={in_offset+1}",
        "gain={field_gain()} in_offset={later}",
        "gain={apply(in_offset)} in_offset={later}",
        "gain={apply(3)} in_offset={later}",
        "gain={if(1,4,in_offset)} in_offset={later}",
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3\n.FUNC field_gain() {in_offset+1}\n.FUNC apply(in_offset) {in_offset+1}", scoped)
                .replace("in out gain ", "in out alias ")
                .replace(
                    ".OPTIONS SEED=37",
                    ".OPTIONS SEED=37\n.MODEL alias gain(gain=5 in_offset=1)",
                );
            let result = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap();
            let actual = result.try_voltage_named("out").unwrap();
            assert!((actual - 16.0).abs() < 1e-12, "{source}: {actual}");
        }
    }
}

#[test]
fn model_defaults_cannot_hide_instance_dependency_cycles() {
    let source = deck("gain={in_offset} in_offset={gain}", "", false)
        .replace("in out gain ", "in out alias ")
        .replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.MODEL alias gain(gain=1 in_offset=1)",
        );
    let error = Engine::default()
        .build_circuit(&Netlist::parse(&source).unwrap())
        .map(|_| ())
        .expect_err("explicit cyclic overrides cannot use stale defaults")
        .to_string();
    assert!(error.contains("could not be resolved"), "{error}");
}

#[test]
fn a_self_reference_can_still_use_its_model_default() {
    let source = deck("gain={gain*2}", "", false)
        .replace("in out gain ", "in out alias ")
        .replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.MODEL alias gain(gain=3)",
        );
    let result = Engine::default()
        .run_dc_op(&Netlist::parse(&source).unwrap())
        .unwrap();
    assert!((result.try_voltage_named("out").unwrap() - 6.0).abs() < 1e-12);
}

#[test]
fn masking_model_defaults_does_not_consume_speculative_samples() {
    let source = deck(
        "gain={aunif(100,1)+in_offset} in_offset={later}",
        ".PARAM later=0",
        false,
    )
    .replace("in out gain ", "in out alias ")
    .replace(
        ".OPTIONS SEED=37",
        ".OPTIONS SEED=37\n.MODEL alias gain(in_offset=1)",
    );
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    let result = Engine::default()
        .run_dc_op(&Netlist::parse(&source).unwrap())
        .unwrap();
    let actual = result.try_voltage_named("out").unwrap();
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn sibling_overrides_shadow_existing_global_bindings_before_eager_folding() {
    for (fields, expected) in [
        ("gain={in_offset+1} in_offset=3", 16.0),
        ("in_offset=3 gain={in_offset+1}", 16.0),
        ("gain=in_offset in_offset=3", 12.0),
        ("gain=IN_OFFSET+1 in_offset=3", 16.0),
        ("gain={in_offset+1} in_offset=2 IN_OFFSET=3", 16.0),
        ("gain={in_offset+1} in_offset={later}", 16.0),
        ("gain={field_gain()} in_offset=3", 16.0),
        ("gain={apply(in_offset)} in_offset=3", 16.0),
        ("gain={apply(3)} in_offset=3", 16.0),
        ("gain={if(1,4,in_offset)} in_offset=3", 16.0),
        ("gain={gain*2} in_offset=3", 24.0),
    ] {
        for scoped in [false, true] {
            let source = deck(fields, ".PARAM later=3", scoped).replace(
                ".OPTIONS SEED=37",
                ".OPTIONS SEED=37\n.PARAM in_offset=1 gain=3\n.FUNC field_gain() {in_offset+1}\n.FUNC apply(in_offset) {in_offset+1}",
            );
            let result = Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap();
            let actual = result.try_voltage_named("out").unwrap();
            assert!(
                (actual - expected).abs() < 1e-12,
                "{source}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn eager_global_bindings_cannot_hide_explicit_instance_cycles() {
    for scoped in [false, true] {
        let source = deck("gain={in_offset} in_offset={gain}", "", scoped).replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.PARAM gain=1 in_offset=1",
        );
        let error = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .map(|_| ())
            .expect_err(&source)
            .to_string();
        assert!(error.contains("could not be resolved"), "{source}: {error}");
    }
}

#[test]
fn masked_sibling_reads_preserve_other_parse_time_random_draws() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let first = eval_expression("aunif(100,1)", &reference).unwrap();
    for expression in ["aunif(100,1)+in_offset", "sample()+in_offset"] {
        let source = deck(
            &format!("gain={{{expression}}} in_offset=0"),
            ".PARAM marker={aunif(100,1)}",
            false,
        )
        .replace(
            ".OPTIONS SEED=37",
            ".OPTIONS SEED=37\n.PARAM in_offset=1\n.FUNC sample() {aunif(100,1)}",
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(netlist.params.get("marker"), Some(first), "{source}");
        Engine::default().build_circuit(&netlist).unwrap();
    }
}
