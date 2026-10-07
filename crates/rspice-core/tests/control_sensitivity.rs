//! SENS control retains physical derivatives through the shared card runner.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind, SensitivityCardResult,
};
use rspice_core::execution::control::{ControlCommand, ControlScalarEvaluator};
use rspice_core::execution::{AnalysisKind, AnalysisResultDocument, SignalUnit};
use rspice_core::{
    AbortSignal, ComplexValue, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig,
    SimulationError,
};
use std::sync::atomic::{AtomicBool, Ordering};

const RC: &str = "Sensitivity control\n.param p=1k\nV1 in 0 DC 1 AC 1\nR1 in out {p}\nR2 out 0 1k\nC1 out 0 1n\n";

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
        (actual - expected).norm() <= 2e-8 * expected.norm().max(1e-12),
        "{actual} != {expected}"
    );
}

fn result(circuit: &ControlCircuit, index: usize) -> &SensitivityCardResult {
    let ControlAnalysisResult::Sensitivity(result) = &circuit.datasets()[index].result else {
        panic!("sensitivity")
    };
    result
}

fn document(circuit: &ControlCircuit, index: usize) -> AnalysisResultDocument {
    let id = circuit.datasets()[index].analysis_id;
    match result(circuit, index) {
        SensitivityCardResult::Dc(result) => AnalysisResultDocument::from_sensitivity(id, result),
        SensitivityCardResult::Ac(result) => {
            AnalysisResultDocument::from_ac_sensitivity(id, result)
        }
    }
    .unwrap()
    .build()
    .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_and_declarative_sens_match_direct_and_analytic_derivatives() {
    for (probe, unit, sign) in [
        ("V(out)", SignalUnit::Volt, 1.0),
        ("V(0,out)", SignalUnit::Volt, -1.0),
        ("I(V1)", SignalUnit::Ampere, 1.0),
    ] {
        for filter in ["R1", "PARAM:p"] {
            for sweep in ["", " AC LIN 3 1000 3000"] {
                let args = format!("{probe} {filter}{sweep}");
                let netlist = Netlist::parse(&format!("{RC}.sens {args}\n.end\n")).unwrap();
                let direct = Engine::default()
                    .run_sensitivity_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
                    .unwrap();
                let mut circuit = ControlCircuit::new(netlist).unwrap();
                execute(&mut circuit, "sens", &args).unwrap();
                execute(&mut circuit, "run", "").unwrap();
                for index in 0..2 {
                    let dataset = &circuit.datasets()[index];
                    assert_eq!(dataset.name, format!("sens{}", index + 1));
                    assert_eq!(dataset.analysis_id.kind(), AnalysisKind::Sensitivity);
                    let expected = match &direct {
                        SensitivityCardResult::Dc(r) => {
                            AnalysisResultDocument::from_sensitivity(dataset.analysis_id, r)
                        }
                        SensitivityCardResult::Ac(r) => {
                            AnalysisResultDocument::from_ac_sensitivity(dataset.analysis_id, r)
                        }
                    }
                    .unwrap()
                    .build()
                    .unwrap();
                    assert_eq!(document(&circuit, index), expected);
                    match result(&circuit, index) {
                        SensitivityCardResult::Dc(r) => {
                            assert_eq!(r.output_unit, unit);
                            close(
                                r.output_value.into(),
                                if probe == "I(V1)" {
                                    (-0.0005).into()
                                } else {
                                    (sign * 0.5).into()
                                },
                            );
                            close(
                                r.sensitivities[0].absolute.into(),
                                if probe == "I(V1)" {
                                    2.5e-7.into()
                                } else {
                                    (-sign * 0.00025).into()
                                },
                            );
                        }
                        SensitivityCardResult::Ac(r) => {
                            assert_eq!(r.output_unit, unit);
                            assert_eq!(r.frequencies, [1000.0, 2000.0, 3000.0]);
                            for (i, frequency) in r.frequencies.iter().enumerate() {
                                let load = ComplexValue::new(
                                    1.0,
                                    std::f64::consts::TAU * frequency * 1e-6,
                                );
                                let denominator = 1000.0 * (1.0 + load);
                                let (nominal, derivative) = if probe == "I(V1)" {
                                    (-load / denominator, load.powi(2) / denominator.powi(2))
                                } else {
                                    (
                                        sign * 1000.0 / denominator,
                                        -sign * 1000.0 * load / denominator.powi(2),
                                    )
                                };
                                close(r.output_values[i], nominal);
                                close(r.sensitivities[0].absolute[i], derivative);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sensitivity_vectors_preserve_complex_values_units_and_scalar_laziness() {
    let mut circuit = ControlCircuit::new(Netlist::parse(&format!("{RC}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, "sens", "V(out) R1 PARAM:p AC LIN 1 1000 1000").unwrap();
    let variables = circuit.netlist().params.clone();
    let SensitivityCardResult::Ac(r) = result(&circuit, 0) else {
        panic!("AC")
    };
    let derivative = r.get("R1").unwrap().absolute[0];
    let nominal = r.output_values[0];
    for expression in [
        "R1",
        "sens1.r1",
        r#""PARAM:p""#,
        r#""sens1.PARAM:p""#,
        "1?R1:2*R1",
    ] {
        close(
            circuit
                .evaluate_scalar(expression, &variables, 9, &NoAbort)
                .unwrap(),
            derivative,
        );
    }
    close(
        circuit
            .evaluate_scalar("if(0,sens99.R1,7)", &variables, 9, &NoAbort)
            .unwrap(),
        7.0.into(),
    );
    let ControlCommandEffect::Presentation(presentation) = execute(
        &mut circuit,
        "print",
        r#"sens1.R1 real(R1) imag(R1) output output_value "PARAM:p""#,
    )
    .unwrap() else {
        panic!("print")
    };
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("print")
    };
    assert_eq!(traces.len(), 6);
    close(traces[0].y.samples[0], derivative);
    close(traces[1].y.samples[0], derivative.re.into());
    close(traces[2].y.samples[0], derivative.im.into());
    close(traces[3].y.samples[0], nominal);
    assert_eq!(traces[3].y.unit, SignalUnit::Volt);
    assert_eq!(traces[4].y.unit, SignalUnit::Volt);
    assert_eq!(traces[0].y.unit, SignalUnit::Unspecified);
    assert_eq!(traces[0].x.unit, SignalUnit::Hertz);
    execute(&mut circuit, "sens", "V(out) R1 AC LIN 3 1000 3000").unwrap();
    assert!(
        circuit
            .evaluate_scalar("sens2.R1", &variables, 9, &NoAbort)
            .unwrap_err()
            .message
            .contains("multiple samples")
    );
    close(
        circuit
            .evaluate_scalar("if(0,sens2.R1,7)", &variables, 9, &NoAbort)
            .unwrap(),
        7.0.into(),
    );
    let ControlCommandEffect::Presentation(presentation) =
        execute(&mut circuit, "plot", "real(R1) imag(R1)").unwrap()
    else {
        panic!("plot")
    };
    let ControlPresentationKind::Plot { traces, .. } = presentation.kind else {
        panic!("plot")
    };
    assert_eq!(traces[0].y.samples.len(), 3);
    close(traces[0].y.samples[0], derivative.re.into());
    close(traces[1].y.samples[0], derivative.im.into());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn executed_changes_reach_sensitivity_and_preserve_old_datasets() {
    let source =
        "Temperature\nV1 in 0 DC 1 AC 1\nR1 in 0 1k tc1=.01 tnom=27\n.options temp=27\n.end\n";
    let mut circuit = ControlCircuit::new(Netlist::parse(source).unwrap()).unwrap();
    execute(&mut circuit, "sens", "I(V1) R1").unwrap();
    let original = document(&circuit, 0);
    execute(&mut circuit, "option", "temp=127").unwrap();
    execute(&mut circuit, "sens", "I(V1) R1").unwrap();
    execute(&mut circuit, "alter", "R1 2k").unwrap();
    execute(&mut circuit, "sens", "I(V1) R1").unwrap();
    for (index, nominal, derivative) in [
        (0, -0.001, 1e-6),
        (1, -0.0005, 5e-7),
        (2, -0.00025, 1.25e-7),
    ] {
        let SensitivityCardResult::Dc(r) = result(&circuit, index) else {
            panic!("DC")
        };
        close(r.output_value.into(), nominal.into());
        close(r.sensitivities[0].absolute.into(), derivative.into());
    }
    assert_eq!(document(&circuit, 0), original);
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
fn failed_and_cancelled_sensitivity_preserve_results_and_ordinals() {
    for sweep in ["", " AC LIN 3 1000 3000"] {
        let mut circuit =
            ControlCircuit::new(Netlist::parse(&format!("{RC}.end\n")).unwrap()).unwrap();
        let args = format!("V(out) R1{sweep}");
        execute(&mut circuit, "sens", &args).unwrap();
        let original = document(&circuit, 0);
        let variables = circuit.netlist().params.clone();
        let error = circuit
            .execute(
                &Engine::default(),
                &command("sens", &args),
                &variables,
                &StopAtCompletion(AtomicBool::new(false)),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ControlExecutionError::Simulation {
                line: 9,
                source: SimulationError::Aborted
            }
        ));
        for invalid in ["V(missing) R1", "V(out) NOT_A_PARAMETER"] {
            assert!(execute(&mut circuit, "sens", invalid).is_err());
        }
        assert_eq!(circuit.datasets().len(), 1);
        assert_eq!(document(&circuit, 0), original);
        execute(&mut circuit, "sens", &args).unwrap();
        assert_eq!(circuit.datasets()[1].name, "sens2");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cumulative_sensitivity_retention_exhausts_limits_atomically() {
    for sweep in ["", " AC LIN 3 1000 3000"] {
        let mut circuit =
            ControlCircuit::new(Netlist::parse(&format!("{RC}.end\n")).unwrap()).unwrap();
        let mut config = SimulationConfig::default();
        config.resource_limits.max_result_values = 4096;
        let engine = Engine::try_new(config).unwrap();
        let variables = circuit.netlist().params.clone();
        let args = format!("V(0) R1{sweep}");
        let mut failed = false;
        for _ in 0..1025 {
            match circuit.execute(&engine, &command("sens", &args), &variables, &NoAbort) {
                Ok(_) => {}
                Err(ControlExecutionError::Simulation {
                    source: SimulationError::ResourceLimit(limit),
                    ..
                }) => {
                    assert_eq!(limit.resource, ResourceKind::ResultValues);
                    assert!(!circuit.datasets().is_empty(), "{limit:?}");
                    failed = true;
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
        assert!(failed);
        let count = circuit.datasets().len();
        execute(&mut circuit, "sens", &args).unwrap();
        assert_eq!(circuit.datasets()[count].name, format!("sens{}", count + 1));
        for index in 0..circuit.datasets().len() {
            let r = result(&circuit, index);
            // Native retention charges unavailable derived slots as well.
            assert_eq!(
                r.retained_value_count(),
                if sweep.is_empty() { 4 } else { 28 }
            );
            match r {
                SensitivityCardResult::Dc(r) => {
                    assert_eq!(r.output_value, 0.0);
                    assert!(r.sensitivities[0].normalized.reason().is_some());
                }
                SensitivityCardResult::Ac(r) => {
                    assert_eq!(r.output_values, [ComplexValue::ZERO; 3]);
                    assert!(r.sensitivities[0].normalized[0].reason().is_some());
                }
            }
        }
    }
}
