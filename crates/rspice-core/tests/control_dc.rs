//! DC control routes use the same solver and preserve the physical sweep grid.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind, DcSweepResult,
};
use rspice_core::execution::control::{ControlCommand, ControlLimits, ControlProgram};
use rspice_core::execution::{AnalysisResultDocument, SignalUnit};
use rspice_core::netlist::{AnalysisCommand, DcSweepSpec};
use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, SimulationConfig, SimulationError};

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 7,
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

fn sweep(circuit: &ControlCircuit, index: usize) -> &DcSweepResult {
    let ControlAnalysisResult::DcSweep(result) = &circuit.datasets()[index].result else {
        panic!("expected DC sweep");
    };
    result
}

fn direct(
    netlist: &Netlist,
    engine: &Engine,
    analysis: &AnalysisCommand,
) -> Vec<rspice_core::engine::DcSweepPointResult> {
    let AnalysisCommand::Dc {
        source,
        start,
        stop,
        step,
        mode,
        sweep2,
    } = analysis
    else {
        panic!("DC")
    };
    engine
        .run_dc_sweep2_spec_with_report_and_abort(
            netlist,
            source,
            &DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            },
            sweep2.as_ref(),
            &NoAbort,
        )
        .unwrap()
}

const NETWORK: &str = "DC\nV1 in 0 0\nV2 bias 0 0\nR1 in out 1k\nR2 bias out 1k\n";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cancellation_after_work_and_runtime_options_preserve_the_session() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct CancelAfter(AtomicUsize);
    impl AbortSignal for CancelAfter {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, Ordering::Relaxed) >= 2048
        }
    }
    let netlist =
        Netlist::parse("DC\nV1 in 0 1\nR1 in 0 1k tc1=0.01\n.options tnom=27 temp=27\n.end\n")
            .unwrap();
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    let engine = Engine::default();
    let cancel = CancelAfter(AtomicUsize::new(0));
    let cancelled = circuit.execute(
        &engine,
        &command("dc", "V1 0 1 .001"),
        &netlist.params,
        &cancel,
    );
    assert!(
        matches!(
            &cancelled,
            Err(ControlExecutionError::Simulation {
                source: SimulationError::Aborted,
                ..
            })
        ),
        "{cancelled:?}, polls={}",
        cancel.0.load(Ordering::Relaxed)
    );
    assert!(cancel.0.load(Ordering::Relaxed) > 2048);
    assert!(circuit.datasets().is_empty());
    execute(&mut circuit, &engine, "option", "temp=127").unwrap();
    execute(&mut circuit, &engine, "dc", "V1 1 2 1").unwrap();
    assert_eq!(circuit.datasets()[0].name, "dc1");
    let result = sweep(&circuit, 0);
    let current = result.points[0].result.branch_current(0).unwrap();
    assert!(
        (current + 0.5e-3).abs() < 1e-10,
        "temperature-adjusted current {current}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nested_dc_run_matches_direct_and_independent_resistor_equations() {
    let engine = Engine::default();
    let netlist =
        Netlist::parse(&format!("{NETWORK}.dc V1 list -2 0 3 V2 list 1 4\n.end\n")).unwrap();
    let expected = direct(&netlist, &engine, &netlist.analyses[0]);
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, &engine, "run", "").unwrap();
    let result = sweep(&circuit, 0);
    assert_eq!(circuit.datasets()[0].name, "dc1");
    assert_eq!(result.points.len(), 6);
    for (row, (actual, expected)) in result.points.iter().zip(&expected).enumerate() {
        assert_eq!(actual.sweep_value, expected.sweep_value);
        assert_eq!(actual.result.node_voltages, expected.result.node_voltages);
        assert_eq!(
            actual.result.branch_currents,
            expected.result.branch_currents
        );
        assert_eq!(
            format!("{:?}", actual.device_op_report),
            format!("{:?}", expected.device_op_report)
        );
        let input = result.axis_value(1, row).unwrap();
        let bias = result.axis_value(0, row).unwrap();
        let index = actual
            .result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((actual.result.try_voltage(index).unwrap() - (input + bias) / 2.0).abs() < 1e-8);
        let branch = actual
            .result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("v1"))
            .unwrap();
        assert!(
            (actual.result.branch_current(branch).unwrap() - (bias - input) / 2000.0).abs() < 1e-10
        );
    }
    let document =
        AnalysisResultDocument::from_dc_analysis(circuit.datasets()[0].analysis_id, result)
            .unwrap()
            .build()
            .unwrap();
    assert_eq!(document.axes().len(), 2);
    assert!(
        document
            .axes()
            .iter()
            .any(|a| a.name() == "sweep:v2" && a.unit() == &SignalUnit::Volt)
    );
    let mut malformed = result.clone();
    malformed.axes[1].values[0] = 99.0;
    assert!(
        AnalysisResultDocument::from_dc_analysis(circuit.datasets()[0].analysis_id, &malformed)
            .is_err()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dc_modes_current_parameter_and_temperature_units_are_retained() {
    let engine = Engine::default();
    for (args, expected) in [
        ("I1 0 2m 1m", vec![0.0, 1e-3, 2e-3]),
        ("I1 list 2m 0 -1m", vec![2e-3, 0.0, -1e-3]),
        ("dec I1 1u 100u 1", vec![1e-6, 1e-5, 1e-4]),
        ("oct I1 1u 4u 1", vec![1e-6, 2e-6, 4e-6]),
    ] {
        let netlist = Netlist::parse("DC\nI1 0 out 0\nR1 out 0 1k\n.end\n").unwrap();
        let mut circuit = ControlCircuit::new(netlist).unwrap();
        execute(&mut circuit, &engine, "dc", args).unwrap();
        let ControlCommandEffect::Presentation(presentation) =
            execute(&mut circuit, &engine, "print", "v(out) i1").unwrap()
        else {
            panic!("print")
        };
        let ControlPresentationKind::Print(traces) = presentation.kind else {
            panic!("print")
        };
        assert_eq!(traces[0].x.unit, SignalUnit::Ampere);
        assert_eq!(traces[1].y.unit, SignalUnit::Ampere);
        for (row, value) in expected.iter().enumerate() {
            assert!((traces[0].x.samples[row].re - value).abs() < 1e-15);
            assert!((traces[0].y.samples[row].re - 1000.0 * value).abs() < 1e-8);
        }
        assert_eq!(traces[0].y.samples.len(), expected.len());
    }
    for (args, unit) in [
        ("r list 1k 2k", SignalUnit::Unspecified),
        ("temp 20 40 20", SignalUnit::Custom("degC".into())),
    ] {
        let mut circuit = ControlCircuit::new(
            Netlist::parse("DC\n.param r=1k\nI1 0 out 1m\nR1 out 0 {r}\n.end\n").unwrap(),
        )
        .unwrap();
        execute(&mut circuit, &engine, "dc", args).unwrap();
        assert_eq!(sweep(&circuit, 0).axes[0].unit, unit);
        let expected = direct(circuit.netlist(), &engine, &circuit.datasets()[0].command);
        assert_eq!(
            sweep(&circuit, 0).points[1].result.node_voltages,
            expected[1].result.node_voltages
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn presentation_compares_both_nested_coordinates_and_retains_prior_datasets() {
    let engine = Engine::default();
    let mut circuit =
        ControlCircuit::new(Netlist::parse(&format!("{NETWORK}.end\n")).unwrap()).unwrap();
    execute(&mut circuit, &engine, "dc", "V1 0 1 1 V2 0 1 1").unwrap();
    let before = sweep(&circuit, 0).points[3].result.node_voltages.clone();
    execute(&mut circuit, &engine, "alter", "R1 2k").unwrap();
    execute(&mut circuit, &engine, "dc", "V1 0 1 1 V2 0 1 1").unwrap();
    execute(&mut circuit, &engine, "print", "dc1.v(out) + dc2.v(out)").unwrap();
    execute(&mut circuit, &engine, "dc", "V1 0 1 1 V2 1 2 1").unwrap();
    assert!(
        execute(&mut circuit, &engine, "print", "dc1.v(out) + dc3.v(out)")
            .unwrap_err()
            .to_string()
            .contains("different sample grids")
    );
    assert_eq!(sweep(&circuit, 0).points[3].result.node_voltages, before);
    assert_ne!(
        circuit.datasets()[0].analysis_id,
        circuit.datasets()[1].analysis_id
    );
    let ControlCommandEffect::Presentation(presentation) =
        execute(&mut circuit, &engine, "print", "v(out) vs v2").unwrap()
    else {
        panic!("print")
    };
    let ControlPresentationKind::Print(traces) = presentation.kind else {
        panic!("print")
    };
    assert_eq!(
        traces[0].x.samples.iter().map(|s| s.re).collect::<Vec<_>>(),
        vec![1.0, 1.0, 2.0, 2.0]
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failures_cancellation_and_cumulative_limits_do_not_publish_or_consume_ids() {
    let netlist = Netlist::parse(&format!("{NETWORK}.end\n")).unwrap();
    let engine = Engine::default();
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    execute(&mut circuit, &engine, "dc", "V1 0 1 .5").unwrap();
    let result = sweep(&circuit, 0);
    let count = result.axes.iter().map(|a| a.values.len()).sum::<usize>()
        + result
            .points
            .iter()
            .map(|p| {
                1 + p.result.retained_value_count()
                    + p.device_op_report
                        .entries
                        .iter()
                        .map(|e| e.params.len())
                        .sum::<usize>()
            })
            .sum::<usize>();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = count * 2 - 1;
    let limited = Engine::new(config);
    assert!(matches!(
        execute(&mut circuit, &limited, "dc", "V1 0 1 .5"),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::ResourceLimit(_),
            ..
        })
    ));
    assert!(execute(&mut circuit, &engine, "dc", "Vmissing 0 1 .5").is_err());
    struct Cancel;
    impl AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        circuit.execute(
            &engine,
            &command("dc", "V1 0 1 .5"),
            &netlist.params,
            &Cancel
        ),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::Aborted,
            ..
        })
    ));
    assert_eq!(circuit.datasets().len(), 1);
    execute(&mut circuit, &engine, "dc", "V1 0 1 .5").unwrap();
    assert_eq!(circuit.datasets()[1].name, "dc2");
    let mut config = SimulationConfig::default();
    config.resource_limits.max_analysis_points = 3;
    assert!(matches!(
        execute(
            &mut circuit,
            &Engine::new(config),
            "dc",
            "V1 0 1 1 V2 0 1 1"
        ),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::ResourceLimit(_),
            ..
        })
    ));
    assert_eq!(circuit.datasets().len(), 2);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn mixed_control_program_runs_dc_in_source_order() {
    let source = format!(
        "{NETWORK}.dc V1 0 2 1\n.control\nop\nrun\nalter R1 2k\ndc V1 0 2 1\nac lin 2 1 10\nprint dc1.v(out) dc2.v(out)\n.endc\n.end\n"
    );
    let program =
        ControlProgram::parse_deck_with_abort(&source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::default();
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        circuit
            .execute(&engine, &command, session.variables(), &NoAbort)
            .unwrap();
    }
    assert_eq!(
        circuit
            .datasets()
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>(),
        vec!["op1", "dc1", "dc2", "ac1"]
    );
    let out = sweep(&circuit, 1).points[2]
        .result
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        (sweep(&circuit, 1).points[2]
            .result
            .try_voltage(out)
            .unwrap()
            - 1.0)
            .abs()
            < 1e-8
    );
    assert!(
        (sweep(&circuit, 2).points[2]
            .result
            .try_voltage(out)
            .unwrap()
            - 2.0 / 3.0)
            .abs()
            < 1e-8
    );
}
