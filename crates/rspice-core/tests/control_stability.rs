//! Control STB retains the direct solver's sweep, evidence and determinations.
use rspice_core::analysis::StbConfig;
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentation, ControlPresentationKind, StbAnalysisResult,
};
use rspice_core::execution::control::{
    ControlCommand, ControlLimits, ControlProgram, ControlScalarEvaluator,
};
use rspice_core::execution::result_document::{ScalarUnavailability, ScalarValue};
use rspice_core::execution::{AnalysisKind, AnalysisResultDocument, SignalUnit};
use rspice_core::netlist::expr::ParamContext;
use rspice_core::{
    AbortSignal, ComplexValue, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig,
    SimulationError,
};
use std::sync::atomic::{AtomicBool, Ordering};

const DECK: &str = "single-pole feedback\nE1 out 0 sense 0 -100\nVprobe out drive 0\nR1 drive sense 1k\nC1 sense 0 159.154943091895n\n";
const SWEEP: &str = "dec 40 10 1meg probe=Vprobe nyquist=yes";

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 7,
    }
}

fn execute(
    circuit: &mut ControlCircuit,
    name: &str,
    args: &str,
) -> Result<ControlCommandEffect, ControlExecutionError> {
    let variables = circuit.netlist().params.clone();
    circuit.execute(
        &Engine::default(),
        &command(name, args),
        &variables,
        &NoAbort,
    )
}

fn circuit(card: &str) -> ControlCircuit {
    ControlCircuit::new(Netlist::parse(&format!("{DECK}{card}\n.end\n")).unwrap()).unwrap()
}

fn result(circuit: &ControlCircuit, index: usize) -> &StbAnalysisResult {
    let ControlAnalysisResult::Stability(result) = &circuit.datasets()[index].result else {
        panic!("STB");
    };
    result
}

fn document(circuit: &ControlCircuit, index: usize) -> AnalysisResultDocument {
    AnalysisResultDocument::from_stability(
        circuit.datasets()[index].analysis_id,
        &result(circuit, index).result,
    )
    .unwrap()
    .build()
    .unwrap()
}

fn print(circuit: &mut ControlCircuit, expression: &str) -> ControlPresentation {
    let ControlCommandEffect::Presentation(presentation) =
        execute(circuit, "print", expression).unwrap()
    else {
        panic!("PRINT");
    };
    presentation
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_and_run_stability_preserve_direct_results_and_analytic_return_ratio() {
    for grid in ["lin 5 10 1meg", "dec 12 10 1meg", "oct 4 10 1meg"] {
        for nyquist in ["yes", "no"] {
            let args = format!("{grid} probe=Vprobe nyquist={nyquist}");
            let mut circuit = circuit(&format!(".stb {args}"));
            let config = StbConfig::try_from(&circuit.netlist().analyses[0]).unwrap();
            let direct = Engine::default()
                .run_stb(circuit.netlist(), config)
                .unwrap();
            execute(&mut circuit, "stb", &args).unwrap();
            execute(&mut circuit, "run", "").unwrap();
            for index in 0..2 {
                let dataset = &circuit.datasets()[index];
                assert_eq!(dataset.name, format!("stb{}", index + 1));
                assert_eq!(dataset.analysis_id.kind(), AnalysisKind::Stb);
                let expected =
                    AnalysisResultDocument::from_stability(dataset.analysis_id, &direct.result)
                        .unwrap()
                        .build()
                        .unwrap();
                assert_eq!(document(&circuit, index), expected);
                let result = result(&circuit, index);
                assert_eq!(result.frequencies, direct.frequencies);
                assert_eq!(result.loop_gains, direct.loop_gains);
                for (&frequency, &gain) in result.frequencies.iter().zip(&result.loop_gains) {
                    let expected = 100.0 / ComplexValue::new(1.0, frequency / 1000.0);
                    assert!(
                        (gain - expected).norm() < 1e-7 * expected.norm(),
                        "{frequency}: {gain} != {expected}"
                    );
                }
                let spectrum = result.result.circuit_poles.spectrum().unwrap();
                assert_eq!(spectrum.poles.len(), 1);
                let expected_pole = -101.0 * std::f64::consts::TAU * 1000.0;
                assert!((spectrum.poles[0] - expected_pole).norm() < expected_pole.abs() * 1e-7);
                assert_eq!(
                    result.result.nyquist_points.len(),
                    if nyquist == "yes" {
                        result.frequencies.len()
                    } else {
                        0
                    }
                );
                assert_eq!(
                    result.retained_value_count(),
                    result.frequencies.len() * 9
                        + result.result.nyquist_points.len() * 3
                        + 6
                        + 2 * spectrum.poles.len()
                        + 5
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn presentations_preserve_complex_vectors_units_scalar_positions_and_margin_availability() {
    let mut circuit = circuit("");
    execute(&mut circuit, "stb", SWEEP).unwrap();
    let presentation = print(
        &mut circuit,
        "loop_gain gain_margin_db dc_loop_gain phase_margin loop_gain_db",
    );
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("PRINT");
    };
    assert_eq!(traces.len(), 2);
    assert_eq!(traces[0].y.unit, SignalUnit::Dimensionless);
    assert_eq!(traces[0].x.unit, SignalUnit::Hertz);
    assert_eq!(traces[1].y.unit, SignalUnit::Custom("dB".into()));
    assert_eq!(traces[0].y.samples, result(&circuit, 0).loop_gains);
    assert!(traces[0].y.samples[0].im.abs() > 0.1);
    assert_eq!(
        presentation
            .scalars
            .iter()
            .map(|s| s.position)
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert_eq!(
        presentation.scalars[0].scalar.value(),
        &ScalarValue::Unavailable {
            reason: ScalarUnavailability::NoCrossover
        }
    );
    let ScalarValue::Complex { value: Some(dc) } = presentation.scalars[1].scalar.value() else {
        panic!("DC");
    };
    assert!((dc.real - 100.0).abs() < 1e-7 && dc.imaginary == 0.0);
    assert_eq!(
        presentation.scalars[2].scalar.unit(),
        Some(&SignalUnit::Degree)
    );
    let ScalarValue::Real { value: Some(pm) } = presentation.scalars[2].scalar.value() else {
        panic!("PM");
    };
    let bandwidth = 1000.0 * (10000.0_f64 - 1.0).sqrt();
    let expected_pm = 180.0 - (bandwidth / 1000.0).atan().to_degrees();
    assert!((pm - expected_pm).abs() < 0.01);
    assert!(
        (result(&circuit, 0)
            .result
            .margins
            .unity_gain_bandwidth()
            .unwrap()
            / bandwidth
            - 1.0)
            .abs()
            < 0.001
    );
    let presentation = print(
        &mut circuit,
        "loopgain_phase_deg loop_gain_magnitude frequency",
    );
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("PRINT");
    };
    assert_eq!(traces[0].y.unit, SignalUnit::Degree);
    assert_eq!(traces[2].y.unit, SignalUnit::Hertz);
    for (index, point) in result(&circuit, 0).result.bode_points.iter().enumerate() {
        assert_eq!(
            traces[0].y.samples[index],
            ComplexValue::from(point.phase_deg.unwrap())
        );
        assert_eq!(
            traces[1].y.samples[index],
            ComplexValue::from(point.magnitude.unwrap())
        );
        assert_eq!(
            traces[2].y.samples[index],
            ComplexValue::from(point.frequency)
        );
    }
    execute(&mut circuit, "settype", "voltage phase_margin").unwrap();
    assert_eq!(
        print(&mut circuit, "phase_margin_degrees").scalars[0]
            .scalar
            .unit(),
        Some(&SignalUnit::Volt)
    );
    execute(&mut circuit, "op", "").unwrap();
    assert!(execute(&mut circuit, "plot", "stb1.loop_gain_phase").is_ok());
    assert!(execute(&mut circuit, "print", "loop_gain").is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_gain_keeps_undefined_phase_and_unbounded_decibels_out_of_finite_expressions() {
    let source = format!("{}.end\n", DECK.replace("-100", "0"));
    let mut circuit = ControlCircuit::new(Netlist::parse(&source).unwrap()).unwrap();
    execute(&mut circuit, "stb", SWEEP).unwrap();
    let presentation = print(
        &mut circuit,
        "loopgain dc_loop_gain_db phase_margin gain_margin",
    );
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("PRINT");
    };
    assert!(
        traces[0]
            .y
            .samples
            .iter()
            .all(|v| *v == ComplexValue::default())
    );
    assert_eq!(
        presentation.scalars[0].scalar.value(),
        &ScalarValue::Unavailable {
            reason: ScalarUnavailability::NegativeInfinity
        }
    );
    for scalar in &presentation.scalars[1..] {
        assert_eq!(
            scalar.scalar.value(),
            &ScalarValue::Unavailable {
                reason: ScalarUnavailability::NoCrossover
            }
        );
    }
    assert!(execute(&mut circuit, "print", "loop_gain_phase").is_err());
    assert!(execute(&mut circuit, "print", "loop_gain_db").is_err());
    assert!(
        result(&circuit, 0)
            .result
            .bode_points
            .iter()
            .all(|p| p.phase_deg.is_none() && p.magnitude_db.is_none())
    );
    document(&circuit, 0);
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let mut variables = ParamContext::new();
        variables.set_expression_dialect(dialect);
        variables.set("gain_margin_db", 42.0);
        assert_eq!(
            circuit
                .evaluate_scalar("if(0,gain_margin_db,7)", &variables, 9, &NoAbort)
                .unwrap(),
            7.0.into()
        );
        for name in ["gain_margin_db", "stb1.phase_margin", "dc_loop_gain_db"] {
            let error = circuit
                .evaluate_scalar(name, &variables, 9, &NoAbort)
                .unwrap_err();
            assert!(
                error.to_string().contains("no finite determination"),
                "{error}"
            );
        }
    }
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
fn cancellation_invalid_probes_and_cumulative_limits_preserve_datasets_and_ordinals() {
    let mut circuit = circuit("");
    let variables = circuit.netlist().params.clone();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = 4096;
    let engine = Engine::try_new(config).unwrap();
    let request = command("stb", "lin 3 10 1meg probe=Vprobe");
    circuit
        .execute(&engine, &request, &variables, &NoAbort)
        .unwrap();
    let original = document(&circuit, 0);
    assert!(matches!(
        circuit.execute(
            &engine,
            &request,
            &variables,
            &StopAtCompletion(AtomicBool::new(false))
        ),
        Err(ControlExecutionError::Simulation {
            line: 7,
            source: SimulationError::Aborted
        })
    ));
    for invalid in [
        "lin 3 10 1meg probe=missing",
        "lin 0 10 1meg probe=Vprobe",
        "lin 3 10 1meg probe=Vprobe junk",
    ] {
        assert!(execute(&mut circuit, "stb", invalid).is_err());
    }
    assert_eq!(circuit.datasets().len(), 1);
    let mut limited = false;
    for _ in 0..100 {
        match circuit.execute(&engine, &request, &variables, &NoAbort) {
            Ok(_) => {}
            Err(ControlExecutionError::Simulation {
                source: SimulationError::ResourceLimit(limit),
                ..
            }) => {
                assert_eq!(limit.resource, ResourceKind::ResultValues);
                limited = true;
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert!(limited);
    let previous = circuit.datasets().len();
    execute(&mut circuit, "stb", &request.arguments).unwrap();
    assert_eq!(
        circuit.datasets()[previous].name,
        format!("stb{}", previous + 1)
    );
    assert_eq!(document(&circuit, 0), original);
    for source in [
        DECK.replace("Vprobe out drive 0", "Vprobe out drive 1"),
        DECK.replace("Vprobe out drive 0", "Vprobe out 0 0"),
    ] {
        let mut invalid =
            ControlCircuit::new(Netlist::parse(&format!("{source}.end\n")).unwrap()).unwrap();
        assert!(execute(&mut invalid, "stb", &request.arguments).is_err());
        assert!(invalid.datasets().is_empty());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scalar_assignments_drive_alter_and_preserve_previous_sweeps() {
    let source = format!(
        "{DECK}.control\nstb {SWEEP}\nlet old_bw=stb1.unity_gain_bandwidth\nalter R1 2k\nstb {SWEEP}\nlet ratio=old_bw/unity_gain_bandwidth\nif ratio > 1.99\nlet accepted=1\nelse\nlet accepted=0\nend\n.endc\n.end\n"
    );
    let program =
        ControlProgram::parse_deck_with_abort(&source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let mut first = None;
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        circuit
            .execute(&Engine::default(), &command, session.variables(), &NoAbort)
            .unwrap();
        if first.is_none() && !circuit.datasets().is_empty() {
            first = Some(document(&circuit, 0));
        }
    }
    assert_eq!(circuit.datasets().len(), 2);
    assert!((session.variables().get("ratio").unwrap() - 2.0).abs() < 0.001);
    assert_eq!(session.variables().get("accepted"), Some(1.0));
    assert_eq!(document(&circuit, 0), first.unwrap());
    assert_ne!(
        result(&circuit, 0).loop_gains,
        result(&circuit, 1).loop_gains
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn executed_options_reach_stability_and_analysis_limits_publish_nothing() {
    let source = format!(
        "{}.options temp=27\n.end\n",
        DECK.replace("R1 drive sense 1k", "R1 drive sense 1k tc1=.01 tnom=27")
    );
    let mut circuit = ControlCircuit::new(Netlist::parse(&source).unwrap()).unwrap();
    execute(&mut circuit, "stb", SWEEP).unwrap();
    let first = document(&circuit, 0);
    execute(&mut circuit, "option", "temp=127").unwrap();
    execute(&mut circuit, "stb", SWEEP).unwrap();
    let cold = result(&circuit, 0)
        .result
        .margins
        .unity_gain_bandwidth()
        .unwrap();
    let hot = result(&circuit, 1)
        .result
        .margins
        .unity_gain_bandwidth()
        .unwrap();
    assert!((cold / hot - 2.0).abs() < 0.001);
    let mut config = SimulationConfig::default();
    config.resource_limits.max_analysis_points = 2;
    let engine = Engine::try_new(config).unwrap();
    let variables = circuit.netlist().params.clone();
    assert!(
        matches!(circuit.execute(&engine,&command("stb",SWEEP),&variables,&NoAbort),Err(ControlExecutionError::Simulation {source:SimulationError::ResourceLimit(limit),..}) if limit.resource==ResourceKind::AnalysisPoints)
    );
    assert_eq!(circuit.datasets().len(), 2);
    assert_eq!(document(&circuit, 0), first);
    execute(&mut circuit, "stb", SWEEP).unwrap();
    assert_eq!(circuit.datasets()[2].name, "stb3");
}
