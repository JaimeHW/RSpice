//! Transfer-function control execution retains the ordinary solver's contract.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentation, ControlPresentationKind,
};
use rspice_core::execution::control::ControlCommand;
use rspice_core::execution::{AnalysisKind, AnalysisResultDocument, SignalUnit};
use rspice_core::{
    AbortSignal, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig, SimulationError,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const DIVIDER: &str = "TF control\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n";

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 9,
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_transfer_function_creates_a_typed_dataset() {
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{DIVIDER}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, &Engine::default(), "tf", "V(out) V1").unwrap();
    assert_eq!(circuit.datasets()[0].name, "tf1");
    assert_eq!(
        circuit.datasets()[0].analysis_id.kind(),
        AnalysisKind::TransferFunction
    );
    let direct = Engine::default()
        .run_transfer_function(circuit.netlist(), "OUT", None, false, "V1")
        .unwrap();
    let dataset = &circuit.datasets()[0];
    let ControlAnalysisResult::TransferFunction(actual) = &dataset.result else {
        panic!("TF result");
    };
    let project = |result: &rspice_core::analysis::TransferFunctionResult| {
        AnalysisResultDocument::from_transfer_function(dataset.analysis_id, result)
            .unwrap()
            .build()
            .unwrap()
    };
    assert_eq!(project(actual), project(&direct));
    close(actual.gain, 2.0 / 3.0);
    close(actual.input_impedance, 3000.0);
    close(actual.output_impedance, 2000.0 / 3.0);
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < expected.abs() * 1e-9 + 1e-14,
        "{actual} != {expected}"
    );
}

fn presentation(
    circuit: &mut ControlCircuit,
    engine: &Engine,
    name: &str,
    args: &str,
) -> ControlPresentation {
    let ControlCommandEffect::Presentation(result) = execute(circuit, engine, name, args).unwrap()
    else {
        panic!("presentation");
    };
    result
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn transfer_vectors_preserve_aliases_arithmetic_units_and_immutable_runs() {
    let engine = Engine::default();
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{DIVIDER}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, &engine, "tf", "V(out) V1").unwrap();
    let printed = presentation(
        &mut circuit,
        &engine,
        "print",
        "transfer_function V1#input_impedance output_impedance_at_v(out)",
    );
    let ControlPresentationKind::Print(traces) = printed.kind else {
        panic!("print");
    };
    for (trace, (value, unit)) in traces.iter().zip([
        (2.0 / 3.0, SignalUnit::Dimensionless),
        (3000.0, SignalUnit::Ohm),
        (2000.0 / 3.0, SignalUnit::Ohm),
    ]) {
        assert_eq!(trace.y.unit, unit);
        assert_eq!(trace.y.samples.len(), 1);
        close(trace.y.samples[0].re, value);
        assert_eq!(trace.x.expression, "index");
    }
    execute(&mut circuit, &engine, "alter", "R1 2k").unwrap();
    execute(&mut circuit, &engine, "tf", "V(out) V1").unwrap();
    let printed = presentation(
        &mut circuit,
        &engine,
        "print",
        "tf1.transfer_function/tf2.transfer_gain tf1.output_impedance tf2.output_impedance_at_v(out)",
    );
    let ControlPresentationKind::Print(traces) = printed.kind else {
        panic!("print");
    };
    close(traces[0].y.samples[0].re, 4.0 / 3.0);
    close(traces[1].y.samples[0].re, 2000.0 / 3.0);
    close(traces[2].y.samples[0].re, 1000.0);
    execute(
        &mut circuit,
        &engine,
        "settype",
        "conductance tf1.transfer_function",
    )
    .unwrap();
    let printed = presentation(
        &mut circuit,
        &engine,
        "print",
        "tf1.transfer_gain tf2.transfer_gain",
    );
    let ControlPresentationKind::Print(traces) = printed.kind else {
        panic!("print");
    };
    assert_eq!(traces[0].y.unit, SignalUnit::Siemens);
    assert_eq!(traces[1].y.unit, SignalUnit::Dimensionless);
    assert!(
        execute(
            &mut circuit,
            &engine,
            "print",
            "tf1.output_impedance_at_v(in)"
        )
        .is_err()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn executed_temperature_and_differential_probe_reach_the_transfer_solver() {
    let engine = Engine::default();
    let source = DIVIDER.replace("R1 in out 1k", "R1 in out 1k tc1=.01 tnom=27");
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{source}.options temp=27\n.end\n")).unwrap())
            .unwrap();
    execute(&mut circuit, &engine, "option", "temp=127").unwrap();
    execute(&mut circuit, &engine, "tf", "V(out,in) V1").unwrap();
    let ControlAnalysisResult::TransferFunction(result) = &circuit.datasets()[0].result else {
        panic!("TF");
    };
    close(result.gain, -0.5);
    close(result.input_impedance, 4000.0);
    assert_eq!(result.output, "V(OUT,IN)");
    let printed = presentation(
        &mut circuit,
        &engine,
        "print",
        "output_impedance_at_v(out,in)",
    );
    let ControlPresentationKind::Print(traces) = printed.kind else {
        panic!("print");
    };
    close(traces[0].y.samples[0].re, 1000.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unbounded_impedance_is_a_printable_ordered_scalar_determination() {
    use rspice_core::execution::result_document::{ScalarUnavailability, ScalarValue};
    let engine = Engine::default();
    let mut circuit =
        ControlCircuit::new(Netlist::parse("Open input\nV1 in 0 1\nR1 out 0 1k\n.end\n").unwrap())
            .unwrap();
    execute(&mut circuit, &engine, "tf", "I(V1) V1").unwrap();
    let printed = presentation(
        &mut circuit,
        &engine,
        "print",
        "transfer_function input_impedance transfer_gain V1#output_impedance",
    );
    let ControlPresentationKind::Print(traces) = printed.kind else {
        panic!("print");
    };
    assert_eq!(traces.len(), 2);
    assert_eq!(printed.scalars.len(), 2);
    for (scalar, position) in printed.scalars.iter().zip([1, 3]) {
        assert_eq!(scalar.position, position);
        assert_eq!(scalar.dataset, "tf1");
        assert_eq!(scalar.scalar.unit(), Some(&SignalUnit::Ohm));
        assert_eq!(
            scalar.scalar.value(),
            &ScalarValue::Unavailable {
                reason: ScalarUnavailability::PositiveInfinity
            }
        );
    }
    let printed = presentation(&mut circuit, &engine, "print", "input_impedance");
    assert_eq!(printed.scalars.len(), 1);
    assert!(execute(&mut circuit, &engine, "plot", "input_impedance").is_err());
    assert_eq!(circuit.datasets().len(), 1);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_and_over_budget_requests_do_not_publish_or_consume_ordinals() {
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = 5;
    let limited = Engine::new(config);
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{DIVIDER}.end\n")).unwrap()).unwrap();
    assert!(execute(&mut circuit, &limited, "tf", "V(missing) V1").is_err());
    assert!(circuit.datasets().is_empty());
    execute(&mut circuit, &limited, "tf", "V(out) V1").unwrap();
    let ControlExecutionError::Simulation {
        source: SimulationError::ResourceLimit(error),
        line,
    } = execute(&mut circuit, &limited, "tf", "V(out) V1").unwrap_err()
    else {
        panic!("resource failure");
    };
    assert_eq!(line, 9);
    assert_eq!(error.resource, ResourceKind::ResultValues);
    assert_eq!((error.requested, error.limit), (6, 5));
    assert_eq!(circuit.datasets().len(), 1);
    execute(&mut circuit, &Engine::default(), "tf", "V(out) V1").unwrap();
    assert_eq!(circuit.datasets()[1].name, "tf2");
}

struct CancelAfter {
    polls: AtomicUsize,
    limit: usize,
}
impl AbortSignal for CancelAfter {
    fn is_aborted(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn transfer_cancellation_during_work_keeps_publication_atomic() {
    let engine = Engine::default();
    let netlist = Netlist::parse(&format!("{DIVIDER}.end\n")).unwrap();
    let request = command("tf", "V(out) V1");
    let counted = CancelAfter {
        polls: AtomicUsize::new(0),
        limit: usize::MAX,
    };
    ControlCircuit::new(netlist.clone())
        .unwrap()
        .execute(&engine, &request, &netlist.params, &counted)
        .unwrap();
    let total = counted.polls.load(Ordering::Relaxed);
    assert!(total > 10);
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    for limit in [0, total / 2, total - 1] {
        let abort = CancelAfter {
            polls: AtomicUsize::new(0),
            limit,
        };
        assert!(matches!(
            circuit.execute(&engine, &request, &netlist.params, &abort),
            Err(ControlExecutionError::Simulation {
                source: SimulationError::Aborted,
                ..
            })
        ));
        assert!(circuit.datasets().is_empty());
    }
    execute(&mut circuit, &engine, "tf", "V(out) V1").unwrap();
    assert_eq!(circuit.datasets()[0].name, "tf1");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn direct_and_control_transfer_stop_on_the_final_progress_callback() {
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
    let netlist = Netlist::parse(&format!("{DIVIDER}.end\n")).unwrap();
    let engine = Engine::default();
    let abort = StopAtCompletion(AtomicBool::new(false));
    assert!(matches!(
        engine.run_transfer_function_with_abort(&netlist, "out", None, false, "V1", &abort),
        Err(SimulationError::Aborted)
    ));
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    let abort = StopAtCompletion(AtomicBool::new(false));
    assert!(matches!(
        circuit.execute(
            &engine,
            &command("tf", "V(out) V1"),
            &netlist.params,
            &abort
        ),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::Aborted,
            ..
        })
    ));
    assert!(circuit.datasets().is_empty());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn declarative_transfer_function_executes_through_run() {
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{DIVIDER}.tf V(out) V1\n.end\n")).unwrap())
            .unwrap();
    execute(&mut circuit, &Engine::default(), "run", "").unwrap();
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(circuit.datasets()[0].name, "tf1");
}
