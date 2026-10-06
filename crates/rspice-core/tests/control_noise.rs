//! Public noise control execution against circuit equations and direct results.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind, ControlTrace,
};
use rspice_core::execution::SignalUnit;
use rspice_core::execution::control::ControlCommand;
use rspice_core::{Engine, Netlist, NoAbort, SimulationConfig, SimulationError, SpiceDialect};

fn engine() -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::BestAvailable,
        ..SimulationConfig::default()
    })
}
fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 12,
    }
}
fn execute(
    circuit: &mut ControlCircuit,
    engine: &Engine,
    name: &str,
    args: &str,
) -> Result<ControlCommandEffect, ControlExecutionError> {
    let variables = circuit.netlist().params.clone();
    circuit.execute(engine, &command(name, args), &variables, &NoAbort)
}
fn print(circuit: &mut ControlCircuit, engine: &Engine, args: &str) -> Vec<ControlTrace> {
    let ControlCommandEffect::Presentation(p) = execute(circuit, engine, "print", args).unwrap()
    else {
        panic!("presentation")
    };
    let ControlPresentationKind::Print(traces) = p.kind else {
        panic!("print")
    };
    traces
}
fn relative(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9,
        "{actual:e} != {expected:e}"
    );
}
const RC: &str =
    "Noise\nV1 in 0 DC 0 AC 2\nV2 ref 0 1\nR1 in out 1k\nC1 out ref 1u\n.options temp=27\n";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rc_noise_control_matches_direct_and_johnson_nyquist_equations() {
    let engine = engine();
    let netlist = Netlist::parse(&format!("{RC}.noise V(out,ref) V1 dec 2 10 1k\n.end\n")).unwrap();
    let frequencies = rspice_core::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        rspice_core::netlist::FreqVariation::Dec,
        2,
        10.0,
        1000.0,
        100,
        &NoAbort,
    )
    .unwrap();
    let expected = engine
        .run_noise_named_with_input_source_and_abort(
            &netlist,
            "out",
            Some("ref"),
            "V1",
            &frequencies,
            300.15,
            &NoAbort,
        )
        .unwrap();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, &engine, "run", "").unwrap();
    let ControlAnalysisResult::Noise(points) = &circuit.datasets()[0].result else {
        panic!("noise")
    };
    assert_eq!(circuit.datasets()[0].name, "noise1");
    assert_eq!(points.len(), expected.len());
    for (actual, direct) in points.iter().zip(&expected) {
        assert_eq!(format!("{actual:?}"), format!("{direct:?}"));
        let omega_rc = std::f64::consts::TAU * actual.frequency * 1e-3;
        let input = 4.0 * 1.380649e-23 * 300.15 * 1000.0;
        relative(actual.input_referred_density, input);
        relative(
            actual.output_noise_density,
            input / (1.0 + omega_rc * omega_rc),
        );
        relative(actual.input_gain_squared, 1.0 / (1.0 + omega_rc * omega_rc));
    }
    let traces = print(
        &mut circuit,
        &engine,
        "onoise_spectrum inoise_spectrum onoise inoise input_gain_squared dno(R1) dni(R1) v(out,ref) i(V1)",
    );
    assert_eq!(traces.len(), 9);
    assert_eq!(traces[0].x.unit, SignalUnit::Hertz);
    assert_eq!(traces[0].y.unit, SignalUnit::Custom("V/sqrt(Hz)".into()));
    assert_eq!(traces[2].y.unit, SignalUnit::Custom("V^2/Hz".into()));
    for (row, expected_point) in expected.iter().enumerate() {
        relative(
            traces[0].y.samples[row].re.powi(2),
            expected_point.output_noise_density,
        );
        relative(
            traces[1].y.samples[row].re.powi(2),
            expected_point.input_referred_density,
        );
        relative(
            traces[5].y.samples[row].re,
            expected_point.output_noise_density,
        );
        relative(
            traces[6].y.samples[row].re,
            expected_point.input_referred_density,
        );
        let omega_rc = std::f64::consts::TAU * expected_point.frequency * 1e-3;
        relative(traces[7].y.samples[row].re, 2.0 / (1.0 + omega_rc.powi(2)));
        relative(
            traces[7].y.samples[row].im,
            -2.0 * omega_rc / (1.0 + omega_rc.powi(2)),
        );
    }
    assert_eq!(traces[8].y.unit, SignalUnit::Ampere);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_referred_noise_and_contribution_vectors_keep_ampere_units() {
    let engine = engine();
    let mut circuit = ControlCircuit::new(
        Netlist::parse("Noise\nI1 0 out DC 0 AC 2u\nR1 out 0 1k\n.end\n").unwrap(),
    )
    .unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) I1 lin 3 10 100").unwrap();
    let traces = print(
        &mut circuit,
        &engine,
        "inoise_spectrum inoise input_gain_squared dni(R1) dno(R1)",
    );
    assert_eq!(traces[0].y.unit, SignalUnit::Custom("A/sqrt(Hz)".into()));
    assert_eq!(traces[1].y.unit, SignalUnit::Custom("A^2/Hz".into()));
    assert_eq!(traces[2].y.unit, SignalUnit::Custom("ohm^2".into()));
    assert_eq!(traces[3].y.unit, SignalUnit::Custom("A^2/Hz".into()));
    let input = 4.0 * 1.380649e-23 * 300.15 / 1000.0;
    relative(traces[0].y.samples[0].re.powi(2), input);
    relative(traces[3].y.samples[0].re, input);
    relative(traces[4].y.samples[0].re, input * 1e6);
    for expression in [
        "dno(MISSING)",
        "dni(R1,missing)",
        "dno(R1,R2,R3)",
        "dno(42)",
    ] {
        assert!(
            execute(&mut circuit, &engine, "print", expression).is_err(),
            "{expression}"
        );
    }
    execute(&mut circuit, &engine, "settype", "current inoise_spectrum").unwrap();
    assert_eq!(
        print(&mut circuit, &engine, "inoise_spectrum")[0].y.unit,
        SignalUnit::Ampere
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_options_and_alterations_preserve_earlier_datasets_and_ordinal_order() {
    let engine = engine();
    let mut circuit = ControlCircuit::new(
        Netlist::parse("Noise\nI1 0 out DC 0 AC 1\nR1 out 0 1k\n.options temp=27\n.end\n").unwrap(),
    )
    .unwrap();
    execute(&mut circuit, &engine, "op", "").unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) I1 lin 3 1 100").unwrap();
    let before = print(&mut circuit, &engine, "noise1.onoise")[0]
        .y
        .samples
        .clone();
    execute(&mut circuit, &engine, "option", "temp=127").unwrap();
    execute(&mut circuit, &engine, "alter", "R1 2k").unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) I1 lin 3 1 100").unwrap();
    assert_eq!(circuit.datasets()[2].name, "noise2");
    let traces = print(
        &mut circuit,
        &engine,
        "noise1.onoise noise2.onoise noise1.dni(R1)",
    );
    assert_eq!(traces[0].y.samples, before);
    relative(
        traces[1].y.samples[0].re / before[0].re,
        2.0 * 400.15 / 300.15,
    );
    assert_ne!(
        circuit.datasets()[1].analysis_id,
        circuit.datasets()[2].analysis_id
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_failure_and_cumulative_limits_are_typed_and_atomic() {
    let engine = engine();
    let mut circuit = ControlCircuit::new(Netlist::parse(&format!("{RC}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) V1 lin 3 10 100").unwrap();
    let ControlAnalysisResult::Noise(points) = &circuit.datasets()[0].result else {
        panic!("noise")
    };
    let count = points
        .iter()
        .map(|p| p.retained_value_count())
        .sum::<usize>();
    let mut config = engine.config().clone();
    config.resource_limits.max_result_values = count * 2 - 1;
    assert!(matches!(
        execute(
            &mut circuit,
            &Engine::new(config),
            "noise",
            "V(out) V1 lin 3 10 100"
        ),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::ResourceLimit(_),
            ..
        })
    ));
    for args in [
        "V(out) V1 lin 3 0 100",
        "V(missing) V1 lin 3 10 100",
        "V(out) missing lin 3 10 100",
    ] {
        assert!(
            execute(&mut circuit, &engine, "noise", args).is_err(),
            "{args}"
        );
    }
    let mut config = engine.config().clone();
    config.resource_limits.max_analysis_points = 2;
    for analysis in ["noise", "ac"] {
        let args = if analysis == "noise" {
            "V(out) V1 lin 3 10 100"
        } else {
            "lin 3 10 100"
        };
        assert!(matches!(
            execute(&mut circuit, &Engine::new(config.clone()), analysis, args),
            Err(ControlExecutionError::Simulation {
                source: SimulationError::ResourceLimit(_),
                ..
            })
        ));
    }
    assert_eq!(circuit.datasets().len(), 1);
    execute(&mut circuit, &engine, "noise", "V(out) V1 lin 3 10 100").unwrap();
    assert_eq!(circuit.datasets()[1].name, "noise2");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_cancellation_after_work_does_not_publish_or_consume_an_ordinal() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct CancelAfter {
        polls: AtomicUsize,
        limit: usize,
    }
    impl rspice_core::AbortSignal for CancelAfter {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
        }
    }
    let engine = engine();
    let netlist = Netlist::parse(&format!("{RC}.end\n")).unwrap();
    let request = command("noise", "V(out) V1 lin 30 10 1000");
    let count = CancelAfter {
        polls: AtomicUsize::new(0),
        limit: usize::MAX,
    };
    let mut baseline = ControlCircuit::new(netlist.clone()).unwrap();
    baseline
        .execute(&engine, &request, &netlist.params, &count)
        .unwrap();
    let total = count.polls.load(Ordering::Relaxed);
    assert!(total > 10);
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    for limit in [0, total / 2, total - 1] {
        let cancel = CancelAfter {
            polls: AtomicUsize::new(0),
            limit,
        };
        assert!(matches!(
            circuit.execute(&engine, &request, &netlist.params, &cancel),
            Err(ControlExecutionError::Simulation {
                source: SimulationError::Aborted,
                ..
            })
        ));
        assert_eq!(cancel.polls.load(Ordering::Relaxed), limit + 1);
        assert!(circuit.datasets().is_empty());
    }
    execute(&mut circuit, &engine, "noise", &request.arguments).unwrap();
    assert_eq!(circuit.datasets()[0].name, "noise1");
}
