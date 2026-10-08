//! DISTO scripts retain every spectrum and use the direct solver's physical units.
use rspice_core::analysis::{DistortionAnalysisResult, DistortionProduct};
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind, ControlTrace,
};
use rspice_core::execution::control::ControlCommand;
use rspice_core::execution::{AnalysisKind, AnalysisResultDocument, SignalUnit};
use rspice_core::{
    AbortSignal, ComplexValue, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig,
    SimulationError,
};
use std::sync::atomic::{AtomicBool, Ordering};

const DECK: &str = "Distortion control\nV1 out 0 DC .5 DISTOF1 1m 30 DISTOF2 .5m -20\nD1 out 0 DM\n.model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n";

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

fn result(circuit: &ControlCircuit, index: usize) -> &DistortionAnalysisResult {
    let ControlAnalysisResult::Distortion(result) = &circuit.datasets()[index].result else {
        panic!("DISTO");
    };
    result
}

fn document(circuit: &ControlCircuit, index: usize) -> AnalysisResultDocument {
    AnalysisResultDocument::from_distortion(
        circuit.datasets()[index].analysis_id,
        result(circuit, index),
    )
    .unwrap()
    .build()
    .unwrap()
}

fn print(circuit: &mut ControlCircuit, expression: &str) -> Vec<ControlTrace> {
    let ControlCommandEffect::Presentation(presentation) =
        execute(circuit, "print", expression).unwrap()
    else {
        panic!("presentation");
    };
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("PRINT");
    };
    traces
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_and_run_distortion_preserve_every_direct_spectrum() {
    for ratio in [None, Some(0.9)] {
        let args = format!(
            "lin 3 1k 2k{}",
            ratio.map_or(String::new(), |r| format!(" {r}"))
        );
        let mut circuit = circuit(&format!(".disto {args}"));
        let direct = Engine::default()
            .run_distortion(circuit.netlist(), &[1000.0, 1500.0, 2000.0], ratio)
            .unwrap();
        execute(&mut circuit, "disto", &args).unwrap();
        execute(&mut circuit, "run", "").unwrap();
        for index in 0..2 {
            let dataset = &circuit.datasets()[index];
            assert_eq!(dataset.name, format!("disto{}", index + 1));
            assert_eq!(dataset.analysis_id.kind(), AnalysisKind::Distortion);
            let expected = AnalysisResultDocument::from_distortion(dataset.analysis_id, &direct)
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(document(&circuit, index), expected);
            assert_eq!(
                result(&circuit, index).retained_value_count(),
                if ratio.is_some() { 75 } else { 45 }
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn product_selection_preserves_peak_phase_units_and_physical_frequency() {
    use rspice_core::constants::{TEMP_REFERENCE, thermal_voltage};
    let vt = thermal_voltage(TEMP_REFERENCE);
    let bias = 1e-12 * (0.5 / vt).exp();
    for (args, cases) in [
        (
            "lin 3 1k 2k",
            vec![
                (
                    "2f1",
                    bias * 1e-6 / (4.0 * vt.powi(2)),
                    60.0,
                    [2000.0, 3000.0, 4000.0],
                ),
                (
                    "3f1",
                    bias * 1e-9 / (24.0 * vt.powi(3)),
                    90.0,
                    [3000.0, 4500.0, 6000.0],
                ),
            ],
        ),
        (
            "lin 3 1k 2k .9",
            vec![
                (
                    "f1+f2",
                    bias * 5e-7 / (2.0 * vt.powi(2)),
                    10.0,
                    [1900.0, 2400.0, 2900.0],
                ),
                (
                    "f1-f2",
                    bias * 5e-7 / (2.0 * vt.powi(2)),
                    50.0,
                    [100.0, 600.0, 1100.0],
                ),
                (
                    "2f1-f2",
                    bias * 5e-10 / (8.0 * vt.powi(3)),
                    80.0,
                    [1100.0, 2100.0, 3100.0],
                ),
            ],
        ),
    ] {
        let mut circuit = circuit("");
        execute(&mut circuit, "disto", args).unwrap();
        for (label, amplitude, phase, frequencies) in cases {
            let traces = print(
                &mut circuit,
                &format!(
                    "disto(\"{label}\",i(V1)) disto(\"{label}\",v(out,0)) disto(\"{label}\",frequency)"
                ),
            );
            assert_eq!(traces.len(), 3);
            assert_eq!(traces[0].y.unit, SignalUnit::Ampere);
            assert_eq!(traces[1].y.unit, SignalUnit::Volt);
            assert_eq!(traces[2].y.unit, SignalUnit::Hertz);
            assert_eq!(
                traces[0].x.samples,
                [1000.0.into(), 1500.0.into(), 2000.0.into()]
            );
            assert_eq!(traces[2].y.samples, frequencies.map(ComplexValue::from));
            let expected = -ComplexValue::from_polar(amplitude, f64::to_radians(phase));
            for value in &traces[0].y.samples {
                assert!(
                    (*value - expected).norm() < 0.003 * amplitude,
                    "{label}: {value} != {expected}"
                );
            }
            assert!(traces[1].y.samples.iter().all(|value| value.norm() < 1e-12));
        }
        let traces = print(&mut circuit, "v(out) disto(\"f1\",v(out))");
        assert_eq!(traces[0].y.samples, traces[1].y.samples);
        if args.ends_with(".9") {
            assert_eq!(
                print(&mut circuit, "disto(\"f2\",frequency)")[0].y.samples,
                [900.0.into(), 900.0.into(), 900.0.into()]
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn qualified_product_expressions_keep_dataset_and_unit_identity() {
    let mut circuit = circuit("");
    execute(&mut circuit, "disto", "lin 3 1k 2k").unwrap();
    let first = document(&circuit, 0);
    execute(&mut circuit, "alter", "V1 .55").unwrap();
    execute(&mut circuit, "disto", "lin 3 1k 2k").unwrap();
    assert_eq!(document(&circuit, 0), first);
    assert_ne!(
        result(&circuit, 0).points[0]
            .product(DistortionProduct::SecondHarmonic)
            .unwrap()
            .response
            .currents,
        result(&circuit, 1).points[0]
            .product(DistortionProduct::SecondHarmonic)
            .unwrap()
            .response
            .currents
    );
    let traces = print(
        &mut circuit,
        "disto(\"2f1\",disto1.i(V1))-disto(\"2f1\",disto2.i(V1))",
    );
    assert_eq!(traces[0].y.unit, SignalUnit::Ampere);
    assert!(traces[0].y.samples.iter().all(|value| value.norm() > 1e-8));
    execute(&mut circuit, "settype", "impedance v(out)").unwrap();
    assert_eq!(
        print(&mut circuit, "disto(\"f1\",v(out))")[0].y.unit,
        SignalUnit::Ohm
    );
    execute(&mut circuit, "settype", "voltage disto(\"f1\",v(out))").unwrap();
    assert_eq!(print(&mut circuit, "v(out)")[0].y.unit, SignalUnit::Volt);
    assert_eq!(
        print(&mut circuit, "disto(\"2f1\",v(out))")[0].y.unit,
        SignalUnit::Volt
    );
    execute(&mut circuit, "settype", "impedance disto(\"2f1\",i(V1))").unwrap();
    assert_eq!(
        print(&mut circuit, "disto(\"2f1\",i(V1))")[0].y.unit,
        SignalUnit::Ohm
    );
    assert_eq!(
        print(&mut circuit, "disto(\"3f1\",i(V1))")[0].y.unit,
        SignalUnit::Ampere
    );
    for expression in [
        "disto(\"f2\",v(0))",
        "disto(\"f1+f2\",v(out))",
        "disto(\"unknown\",v(out))",
        "disto(\"2f1\",3)",
        "disto(\"2f1\",disto(\"3f1\",v(out)))",
        "disto1.disto(\"2f1\",v(out))",
    ] {
        assert!(
            execute(&mut circuit, "print", expression).is_err(),
            "{expression}"
        );
    }
    execute(&mut circuit, "op", "").unwrap();
    assert!(execute(&mut circuit, "print", "disto(\"2f1\",v(out))").is_err());
    assert!(execute(&mut circuit, "print", "disto(\"2f1\",disto1.i(V1))").is_ok());
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
fn failure_cancellation_and_cumulative_limits_preserve_datasets_and_ordinals() {
    let mut circuit = circuit("");
    let variables = circuit.netlist().params.clone();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = 4096;
    let engine = Engine::try_new(config).unwrap();
    let request = command("disto", "lin 3 1k 2k .9");
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
    for invalid in ["lin 3 1k 2k 1.1", "lin 3 0 2k", "lin 3 1k 2k junk"] {
        assert!(execute(&mut circuit, "disto", invalid).is_err());
    }
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(document(&circuit, 0), original);
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
    execute(&mut circuit, "disto", &request.arguments).unwrap();
    assert_eq!(
        circuit.datasets()[previous].name,
        format!("disto{}", previous + 1)
    );
    assert_eq!(document(&circuit, 0), original);
}
