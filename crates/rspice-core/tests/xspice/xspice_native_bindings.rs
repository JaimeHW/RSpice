use rspice_core::{Engine, Netlist};

fn deck(fields: &str, definitions: &str, scoped: bool, alias: bool) -> String {
    let model = if alias { "capmod" } else { "capacitor" };
    let mut body = format!("A1 %hd[out 0] {model} {fields}");
    if scoped {
        body = format!(".SUBCKT cell out\n{body}\n.ENDS\nX1 out cell");
    }
    format!(
        "* native instance bindings\n.OPTIONS SEED=37\n.PARAM IC=7\n{body}\n{definitions}\n.MODEL capmod capacitor(c=1n ic=9)\n.END\n"
    )
}

#[test]
fn native_capacitors_resolve_siblings_before_enclosing_values() {
    for read in ["IC", "dependent", "read_ic()"] {
        for scoped in [false, true] {
            for alias in [false, true] {
                for reverse in [false, true] {
                    let fields = if reverse {
                        format!("IC={{later}} C={{{read}*1e-9}}")
                    } else {
                        format!("C={{{read}*1e-9}} IC={{later}}")
                    };
                    let source = deck(
                        &fields,
                        ".PARAM later=2\n.GLOBAL_PARAM dependent={IC}\n.FUNC read_ic() {IC}",
                        scoped,
                        alias,
                    );
                    let netlist = Netlist::parse(&source).unwrap();
                    let circuit = Engine::default().build_circuit(&netlist).unwrap();
                    let capacitors = circuit.capacitor_storage();
                    assert!(
                        (capacitors.capacitances[0] - 2e-9).abs() < 1e-22,
                        "{source}: {:?}",
                        capacitors.capacitances
                    );
                    assert_eq!(capacitors.ic[0], Some(2.0));
                }
            }
        }
    }
}

#[test]
fn native_capacitor_cycles_cannot_be_satisfied_by_numeric_fallbacks() {
    for scoped in [false, true] {
        let source = deck("C={IC*1e-9} IC={C/1e-9}", ".PARAM C=1e-9", scoped, true);
        let error = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .map(|_| ())
            .expect_err(&source)
            .to_string();
        assert!(error.contains("cyclic"), "{source}: {error}");
    }
}

#[test]
fn native_capacitor_forward_attempts_do_not_consume_random_samples() {
    use rspice_core::netlist::{ParamContext, expr::eval_expression};
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)*1e-9", &reference).unwrap();
    for scoped in [false, true] {
        let source = deck(
            "C={aunif(100,1)*1e-9+IC} IC={later}",
            ".PARAM later=0",
            scoped,
            false,
        )
        .replace(".PARAM IC=7\n", "");
        let circuit = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .unwrap();
        assert_eq!(
            circuit.capacitor_storage().capacitances[0],
            expected,
            "{source}"
        );
    }
}

#[test]
fn native_capacitors_keep_self_references_and_report_missing_leaves() {
    for scoped in [false, true] {
        let source = deck("C={IC*1e-9} IC={IC+1}", "", scoped, true);
        let circuit = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .unwrap();
        assert!((circuit.capacitor_storage().capacitances[0] - 8e-9).abs() < 1e-22);
        assert_eq!(circuit.capacitor_storage().ic[0], Some(8.0));
        let source =
            deck("C={IC*1e-9} IC={missing_leaf}", "", scoped, true).replace(".PARAM IC=7\n", "");
        let error = Engine::default()
            .build_circuit(&Netlist::parse(&source).unwrap())
            .map(|_| ())
            .expect_err(&source)
            .to_string();
        assert!(
            error.to_ascii_uppercase().contains("MISSING_LEAF"),
            "{error}"
        );
        assert!(!error.contains("cyclic"), "{error}");
    }
}

#[test]
fn native_instance_resolution_preserves_cancellation_and_retry() {
    use rspice_core::{SimulationError, abort_signal::CountingAbort};
    for scoped in [false, true] {
        let source = deck(
            "C={dependent*1e-9} IC={later}",
            ".GLOBAL_PARAM dependent={IC}\n.PARAM later=2",
            scoped,
            true,
        );
        let netlist = Netlist::parse(&source).unwrap();
        let count = CountingAbort::new(usize::MAX);
        Engine::default()
            .build_circuit_with_abort(&netlist, &count)
            .unwrap();
        for limit in 0..count.count() {
            let abort = CountingAbort::new(limit);
            let error = Engine::default()
                .build_circuit_with_abort(&netlist, &abort)
                .map(|_| ())
                .expect_err("cancelled native binding");
            assert!(
                matches!(error, SimulationError::Aborted),
                "limit={limit}, scoped={scoped}: {error}"
            );
            assert_eq!(abort.count(), limit + 1);
            assert_eq!(abort.polls_after_abort(), 0);
        }
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        assert!((circuit.capacitor_storage().capacitances[0] - 2e-9).abs() < 1e-22);
    }
}
