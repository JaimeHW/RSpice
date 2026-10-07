//! PZ control uses the direct solver and retains physical complex roots.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind,
};
use rspice_core::execution::control::{ControlCommand, ControlErrorKind, ControlScalarEvaluator};
use rspice_core::execution::{AnalysisKind, AnalysisResultDocument, SignalUnit};
use rspice_core::netlist::expr::{ParamContext, eval_expression_complex};
use rspice_core::{
    AbortSignal, ComplexValue, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig,
    SimulationError,
};
use std::sync::atomic::{AtomicBool, Ordering};

const RLC: &str = "PZ control\nV1 in 0 0\nR1 in mid 1\nL1 mid out 1\nC1 out 0 1\n";

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 9,
    }
}

fn execute(
    circuit: &mut ControlCircuit,
    name: &str,
    arguments: &str,
) -> Result<ControlCommandEffect, ControlExecutionError> {
    let variables = circuit.netlist().params.clone();
    circuit.execute(
        &Engine::default(),
        &command(name, arguments),
        &variables,
        &NoAbort,
    )
}

fn close(actual: ComplexValue, expected: ComplexValue) {
    assert!(
        (actual - expected).norm() < 1e-9,
        "{actual:?} != {expected:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_and_declarative_pz_runs_retain_complex_roots_and_immutable_names() {
    let mut circuit = ControlCircuit::new(
        Netlist::parse(&format!("{RLC}.pz in 0 out 0 vol pz\n.end\n")).unwrap(),
    )
    .unwrap();
    execute(&mut circuit, "pz", "in 0 out 0 vol pz").unwrap();
    assert_eq!(circuit.datasets()[0].name, "pz1");
    let variables = circuit.netlist().params.clone();
    let root = ComplexValue::new(-0.5, 3.0_f64.sqrt() / 2.0);
    close(
        circuit
            .evaluate_scalar("pole(1)", &variables, 12, &NoAbort)
            .unwrap(),
        root,
    );
    close(
        circuit
            .evaluate_scalar("pz1.pole(2)", &variables, 12, &NoAbort)
            .unwrap(),
        root.conj(),
    );
    execute(&mut circuit, "alter", "R1 3").unwrap();
    execute(&mut circuit, "run", "").unwrap();
    assert_eq!(circuit.datasets()[1].name, "pz2");
    close(
        circuit
            .evaluate_scalar("pz1.pole(1)", &variables, 12, &NoAbort)
            .unwrap(),
        root,
    );
    close(
        circuit
            .evaluate_scalar("pz2.pole(1)", &variables, 12, &NoAbort)
            .unwrap(),
        ComplexValue::from((-3.0 + 5.0_f64.sqrt()) / 2.0),
    );
    close(
        circuit
            .evaluate_scalar("dc_gain", &variables, 12, &NoAbort)
            .unwrap(),
        ComplexValue::from(1.0),
    );
    close(
        circuit
            .evaluate_scalar("hf_gain", &variables, 12, &NoAbort)
            .unwrap(),
        ComplexValue::ZERO,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn pz_control_documents_match_direct_modes_ports_and_physical_units() {
    for (body, ports, gain, unit) in [
        (
            "R1 out 0 1k\nC1 out 0 1u\n",
            "out 0 out 0 cur",
            1000.0,
            SignalUnit::Ohm,
        ),
        (
            "V1 in ref 0\nR1 in out 1k\nC1 out ref 1u\nRref ref 0 1k\n",
            "in ref out ref vol",
            1.0,
            SignalUnit::Dimensionless,
        ),
    ] {
        for mode in ["pz", "pol", "zer"] {
            let args = format!("{ports} {mode}");
            let netlist = Netlist::parse(&format!("ports\n{body}.pz {args}\n.end\n")).unwrap();
            let direct = Engine::default()
                .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
                .unwrap();
            let mut circuit = ControlCircuit::new(netlist).unwrap();
            execute(&mut circuit, "pz", &args).unwrap();
            let dataset = &circuit.datasets()[0];
            assert_eq!(dataset.analysis_id.kind(), AnalysisKind::PoleZero);
            let ControlAnalysisResult::PoleZero(actual) = &dataset.result else {
                panic!("PZ")
            };
            let project = |result| {
                AnalysisResultDocument::from_pole_zero(dataset.analysis_id, result)
                    .unwrap()
                    .build()
                    .unwrap()
            };
            assert_eq!(project(actual), project(&direct));
            assert_eq!(actual.gain_unit, unit);
            assert!((actual.dc_gain.unwrap() - gain).abs() < 1e-8);
            if mode != "zer" {
                close(actual.poles[0], ComplexValue::from(-1000.0));
            }
            assert!(actual.has_consistent_root_evidence());
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_lookup_is_lazy_checked_and_preserves_user_functions_and_random_draws() {
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{RLC}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, "pz", "in 0 out 0 vol pz").unwrap();
    let mut variables = ParamContext::new();
    variables.define_function("saved", vec!["n".into()], "pz1.pole(n)");
    variables.define_function("broken", vec![], "(");
    let root = ComplexValue::new(-0.5, 3.0_f64.sqrt() / 2.0);
    for (expr, expected) in [
        ("saved(1)", root),
        ("if(1,pole(1),broken())", root),
        ("if(0,pz99.pole(1),7)", ComplexValue::from(7.0)),
    ] {
        close(
            circuit
                .evaluate_scalar(expr, &variables, 17, &NoAbort)
                .unwrap(),
            expected,
        );
    }
    for expr in [
        "pole(0)",
        "pole(-1)",
        "pole(1.5)",
        "pole(1+1e-300j)",
        "pole(1e100)",
        "pole(3)",
        "zero(1)",
        "pole()",
        "pole(1,2)",
        "pz99.pole(1)",
    ] {
        let error = circuit
            .evaluate_scalar(expr, &variables, 17, &NoAbort)
            .unwrap_err();
        assert_eq!(error.line, 17);
        assert_eq!(error.kind, ControlErrorKind::Expression);
    }
    variables.set_random_seed(19);
    let mut reference = ParamContext::new();
    reference.set_random_seed(19);
    let args = "pz1.pole(1+0*aunif(0,1))";
    // PRINT's determination preflight must not evaluate the index a second time.
    circuit
        .execute(
            &Engine::default(),
            &command("print", args),
            &variables,
            &NoAbort,
        )
        .unwrap();
    eval_expression_complex("aunif(0,1)", &reference).unwrap();
    assert_eq!(
        eval_expression_complex("aunif(0,1)", &variables).unwrap(),
        eval_expression_complex("aunif(0,1)", &reference).unwrap()
    );
    variables.define_function("pole", vec!["n".into()], "n+4j");
    close(
        circuit
            .evaluate_scalar("pole(1)", &variables, 17, &NoAbort)
            .unwrap(),
        ComplexValue::new(1.0, 4.0),
    );
    close(
        circuit
            .evaluate_scalar("pz1.pole(1)", &variables, 17, &NoAbort)
            .unwrap(),
        root,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_presentations_keep_angular_units_complex_parts_and_atomic_type_changes() {
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{RLC}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, "pz", "in 0 out 0 vol pz").unwrap();
    let ControlCommandEffect::Presentation(print) = execute(
        &mut circuit,
        "print",
        "pole(1) real(pz1.pole(1)) imag(pole(1)) dc_gain hf_gain",
    )
    .unwrap() else {
        panic!("print")
    };
    let ControlPresentationKind::Print(traces) = print.kind else {
        panic!("print")
    };
    assert_eq!(traces.len(), 5);
    for trace in &traces[..3] {
        assert_eq!(trace.y.unit, SignalUnit::RadianPerSecond);
    }
    close(
        traces[0].y.samples[0],
        ComplexValue::new(-0.5, 3.0_f64.sqrt() / 2.0),
    );
    close(traces[1].y.samples[0], ComplexValue::from(-0.5));
    close(
        traces[2].y.samples[0],
        ComplexValue::from(3.0_f64.sqrt() / 2.0),
    );
    assert_eq!(traces[3].y.unit, SignalUnit::Dimensionless);
    assert!(execute(&mut circuit, "settype", "frequency pole(1) pole(9)").is_err());
    let ControlCommandEffect::Presentation(print) =
        execute(&mut circuit, "print", "pole(1)").unwrap()
    else {
        panic!("print")
    };
    let ControlPresentationKind::Print(traces) = print.kind else {
        panic!("print")
    };
    assert_eq!(traces[0].y.unit, SignalUnit::RadianPerSecond);
    execute(&mut circuit, "settype", "frequency pole(1)").unwrap();
    let ControlCommandEffect::Presentation(plot) =
        execute(&mut circuit, "plot", "imag(pole(1)) vs real(pole(1))").unwrap()
    else {
        panic!("plot")
    };
    let ControlPresentationKind::Plot { traces, .. } = plot.kind else {
        panic!("plot")
    };
    assert_eq!(traces[0].y.unit, SignalUnit::Hertz);
    assert_eq!(traces[0].x.unit, SignalUnit::Hertz);
}

struct StopAtCompletion(AtomicBool);
impl AbortSignal for StopAtCompletion {
    fn is_aborted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn observe_progress(&self, fraction: f64) {
        if fraction == 1.0 {
            self.0.store(true, Ordering::Relaxed);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_or_cancelled_pz_does_not_publish_or_advance_dataset_ordinals() {
    for dynamic in ["", "C1 out 0 1u\n"] {
        let mut circuit = ControlCircuit::new(
            Netlist::parse(&format!("PZ\nR1 out 0 1k\n{dynamic}.end\n")).unwrap(),
        )
        .unwrap();
        let variables = circuit.netlist().params.clone();
        let abort = StopAtCompletion(AtomicBool::new(false));
        let error = circuit
            .execute(
                &Engine::default(),
                &command("pz", "out 0 out 0 cur pz"),
                &variables,
                &abort,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ControlExecutionError::Simulation {
                line: 9,
                source: SimulationError::Aborted
            }
        ));
        assert!(circuit.datasets().is_empty());
        assert!(execute(&mut circuit, "pz", "missing 0 out 0 cur pz").is_err());
        execute(&mut circuit, "pz", "out 0 out 0 cur pz").unwrap();
        assert_eq!(circuit.datasets()[0].name, "pz1");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cumulative_pz_retention_exhausts_limits_without_losing_completed_runs() {
    let mut circuit =
        ControlCircuit::new(Netlist::parse("PZ\nR1 out 0 1k\nC1 out 0 1u\n.end\n").unwrap())
            .unwrap();
    let mut config = SimulationConfig::default();
    // Admit the solver's temporary workspace as well as one result; repeated
    // retained roots must eventually reduce that allowance enough to refuse.
    config.resource_limits.max_result_values = 4096;
    let engine = Engine::try_new(config).unwrap();
    let variables = circuit.netlist().params.clone();
    let mut failed = false;
    for _ in 0..1025 {
        match circuit.execute(
            &engine,
            &command("pz", "out 0 out 0 cur pz"),
            &variables,
            &NoAbort,
        ) {
            Ok(_) => {}
            Err(ControlExecutionError::Simulation {
                source: SimulationError::ResourceLimit(limit),
                ..
            }) => {
                assert_eq!(limit.resource, ResourceKind::ResultValues);
                assert!(
                    !circuit.datasets().is_empty(),
                    "initial analysis requires {limit:?}"
                );
                failed = true;
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert!(failed);
    let count = circuit.datasets().len();
    assert!(count > 0);
    execute(&mut circuit, "pz", "out 0 out 0 cur pz").unwrap();
    assert_eq!(circuit.datasets()[count].name, format!("pz{}", count + 1));
    for dataset in circuit.datasets() {
        let ControlAnalysisResult::PoleZero(result) = &dataset.result else {
            panic!("PZ")
        };
        close(result.poles[0], ComplexValue::from(-1000.0));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn highpass_zeros_and_unavailable_integrator_gain_are_not_fabricated() {
    let mut circuit = ControlCircuit::new(
        Netlist::parse("HP\nV1 in 0 0\nC1 in out 1u\nR1 out 0 1k\n.end\n").unwrap(),
    )
    .unwrap();
    execute(&mut circuit, "pz", "in 0 out 0 vol pz").unwrap();
    let variables = ParamContext::new();
    close(
        circuit
            .evaluate_scalar("zero(1)", &variables, 10, &NoAbort)
            .unwrap(),
        ComplexValue::ZERO,
    );
    close(
        circuit
            .evaluate_scalar("high_frequency_gain", &variables, 10, &NoAbort)
            .unwrap(),
        ComplexValue::from(1.0),
    );
    let mut circuit =
        ControlCircuit::new(Netlist::parse("Integrator\nC1 out 0 1u\n.end\n").unwrap()).unwrap();
    execute(&mut circuit, "pz", "out 0 out 0 cur pz").unwrap();
    let ControlAnalysisResult::PoleZero(result) = &circuit.datasets()[0].result else {
        panic!("PZ")
    };
    assert_eq!(result.dc_gain, None);
    let mut variables = ParamContext::new();
    variables.set("dc_gain", 42.0);
    assert!(
        circuit
            .evaluate_scalar("dc_gain", &variables, 10, &NoAbort)
            .unwrap_err()
            .message
            .contains("no finite value")
    );
    close(
        circuit
            .evaluate_scalar("if(0,dc_gain,7)", &variables, 10, &NoAbort)
            .unwrap(),
        ComplexValue::from(7.0),
    );
    assert!(execute(&mut circuit, "print", "dc_gain").is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn executed_temperature_options_reach_pz_roots_and_gains() {
    let mut circuit = ControlCircuit::new(
        Netlist::parse(
            "Temperature\nR1 out 0 1k tc1=.01 tnom=27\nC1 out 0 1u\n.options temp=27\n.end\n",
        )
        .unwrap(),
    )
    .unwrap();
    execute(&mut circuit, "option", "temp=127").unwrap();
    execute(&mut circuit, "pz", "out 0 out 0 cur pz").unwrap();
    let ControlAnalysisResult::PoleZero(result) = &circuit.datasets()[0].result else {
        panic!("PZ")
    };
    close(result.poles[0], ComplexValue::from(-500.0));
    assert!((result.dc_gain.unwrap() - 2000.0).abs() < 1e-8);
}
