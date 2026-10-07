//! Table control execution retains physical coordinates and publishes atomically.
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind, FrequencyDataResult,
};
use rspice_core::execution::control::ControlCommand;
use rspice_core::{Engine, Netlist, NoAbort, SimulationConfig, SimulationError};

const NETWORK: &str = "Table control\n.param resistance=1k ambient=27\n.options temp={ambient}\nV1 in 0 AC 1\nR1 in out {resistance} tc1=.01 tnom=27\nC1 out 0 1u\n";
const TABLE: &str =
    ".data points FREQ resistance ambient\n100 1k 27\n10 2k 127\n100 3k 77\n.enddata\n";

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 19,
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

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-30,
        "{actual:e} != {expected:e}"
    );
}

fn ac(
    circuit: &ControlCircuit,
    index: usize,
) -> &FrequencyDataResult<rspice_core::analysis::AcResult> {
    let ControlAnalysisResult::AcTable(result) = &circuit.datasets()[index].result else {
        panic!("AC table")
    };
    result
}

fn noise(
    circuit: &ControlCircuit,
    index: usize,
) -> &FrequencyDataResult<rspice_core::analysis::NoiseResult> {
    let ControlAnalysisResult::NoiseTable(result) = &circuit.datasets()[index].result else {
        panic!("noise table")
    };
    result
}

fn verify(circuit: &ControlCircuit, first: usize, temperatures: &[f64], capacitance: f64) {
    let ac = ac(circuit, first);
    let noise = noise(circuit, first + 1);
    assert_eq!(ac.columns, noise.columns);
    assert_eq!(ac.requested_rows, 3);
    assert!(ac.finish.is_none());
    assert_eq!(ac.columns[0].values, [100.0, 10.0, 100.0]);
    for (index, (a, n)) in ac.points.iter().zip(&noise.points).enumerate() {
        let temp = temperatures[index];
        let resistance = ac.columns[1].values[index] * (1.0 + 0.01 * (temp - 27.0));
        let wrc = std::f64::consts::TAU * a.frequency * resistance * capacitance;
        let out = a
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        close(a.voltages[out].re, 1.0 / (1.0 + wrc * wrc));
        close(a.voltages[out].im, -wrc / (1.0 + wrc * wrc));
        close(n.voltages[out].re, a.voltages[out].re);
        close(
            n.input_referred_density,
            4.0 * 1.380649e-23 * (temp + 273.15) * resistance,
        );
        close(
            n.output_noise_density,
            n.input_referred_density / (1.0 + wrc * wrc),
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_and_declarative_tables_match_direct_solvers_and_circuit_equations() {
    let netlist = Netlist::parse(&format!(
        "{NETWORK}{TABLE}.ac data=points\n.noise V(out) V1 data=points\n.end\n"
    ))
    .unwrap();
    let engine = Engine::default();
    let direct_ac = engine.run_ac_table(&netlist, "points").unwrap();
    let direct_noise = engine
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
        .unwrap();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, &engine, "run", "").unwrap();
    execute(&mut circuit, &engine, "ac", "data=points").unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) V1 data=points").unwrap();
    assert_eq!(
        circuit
            .datasets()
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>(),
        ["ac1", "noise1", "ac2", "noise2"]
    );
    for first in [0, 2] {
        verify(&circuit, first, &[27.0, 127.0, 77.0], 1e-6);
        assert_eq!(ac(&circuit, first).columns, direct_ac.columns);
        assert_eq!(
            format!("{:?}", ac(&circuit, first).points),
            format!("{:?}", direct_ac.points)
        );
        assert_eq!(
            format!("{:?}", noise(&circuit, first + 1).points),
            format!("{:?}", direct_noise.points)
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn row_options_and_explicit_runtime_options_have_distinct_precedence() {
    for (resolved, option, temperatures) in [
        (false, "reltol=.002", [27.0, 127.0, 77.0]),
        (false, "temp=77", [77.0; 3]),
        (true, "reltol=.002", [127.0; 3]),
        (true, "temp=77", [77.0; 3]),
    ] {
        let netlist = Netlist::parse(&format!("{NETWORK}{TABLE}.end\n")).unwrap();
        let base = Engine::default();
        let engine = if resolved {
            let mut config = base.resolved_for_netlist(&netlist).config().clone();
            config.temperature = 400.15;
            base.try_resolved_with_config(config).unwrap()
        } else {
            base
        };
        let mut circuit = ControlCircuit::new(netlist).unwrap();
        execute(&mut circuit, &engine, "option", option).unwrap();
        execute(&mut circuit, &engine, "ac", "data=points").unwrap();
        execute(&mut circuit, &engine, "noise", "V(out) V1 data=points").unwrap();
        verify(&circuit, 0, &temperatures, 1e-6);
        execute(&mut circuit, &engine, "alter", "C1=2u").unwrap();
        execute(&mut circuit, &engine, "ac", "data=points").unwrap();
        execute(&mut circuit, &engine, "noise", "V(out) V1 data=points").unwrap();
        verify(&circuit, 0, &temperatures, 1e-6);
        verify(&circuit, 2, &temperatures, 2e-6);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn physical_temperature_column_wins_over_authored_caller_and_control_options() {
    let table = TABLE.replace("ambient", "TEMP");
    let netlist = Netlist::parse(&format!("{NETWORK}{table}.end\n")).unwrap();
    let base = Engine::default();
    let mut config = base.resolved_for_netlist(&netlist).config().clone();
    config.temperature = 500.0;
    let engine = base.try_resolved_with_config(config).unwrap();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, &engine, "option", "temp=37").unwrap();
    execute(&mut circuit, &engine, "ac", "data=points").unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) V1 data=points").unwrap();
    verify(&circuit, 0, &[27.0, 127.0, 77.0], 1e-6);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn presentation_preserves_coordinates_complex_samples_and_noise_units() {
    let other = ".data other resistance HERTZ ambient\n1k 100 27\n2k 10 127\n3k 100 77\n.enddata\n.data different FREQ resistance ambient\n100 1k 27\n10 2k 127\n100 4k 77\n.enddata\n";
    let netlist = Netlist::parse(&format!("{NETWORK}{TABLE}{other}.end\n")).unwrap();
    let engine = Engine::default();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, &engine, "ac", "data=points").unwrap();
    execute(&mut circuit, &engine, "noise", "V(out) V1 data=other").unwrap();
    let ControlCommandEffect::Presentation(p) = execute(
        &mut circuit,
        &engine,
        "print",
        "ac1.v(out) ac1.resistance noise1.onoise_spectrum noise1.dno(R1) ac1.v(out)-noise1.v(out)",
    )
    .unwrap() else {
        panic!("presentation")
    };
    let ControlPresentationKind::Print(traces) = p.kind else {
        panic!("print")
    };
    assert_eq!(traces.len(), 5);
    assert_eq!(
        traces[0].x.samples.iter().map(|v| v.re).collect::<Vec<_>>(),
        [100.0, 10.0, 100.0]
    );
    assert!(traces[0].y.samples.iter().all(|v| v.im < 0.0));
    assert_eq!(
        traces[1].y.samples.iter().map(|v| v.re).collect::<Vec<_>>(),
        [1000.0, 2000.0, 3000.0]
    );
    assert_eq!(traces[2].y.unit.symbol(), "V/sqrt(Hz)");
    assert_eq!(traces[3].y.unit.symbol(), "V^2/Hz");
    assert!(traces[4].y.samples.iter().all(|v| v.norm() < 1e-14));
    execute(&mut circuit, &engine, "ac", "data=different").unwrap();
    let error = execute(&mut circuit, &engine, "print", "ac1.v(out)-ac2.v(out)").unwrap_err();
    assert!(
        error.to_string().contains("different sample grids"),
        "{error}"
    );
    execute(&mut circuit, &engine, "ac", "lin 3 10 100").unwrap();
    assert!(execute(&mut circuit, &engine, "print", "ac2.v(out)-ac3.v(out)").is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn failed_rows_and_cumulative_budgets_publish_nothing_and_do_not_consume_ordinals() {
    let netlist = Netlist::parse(&format!(
        "{NETWORK}{TABLE}.data bad FREQ TEMP\n100 27\n10 -300\n.enddata\n.end\n"
    ))
    .unwrap();
    let engine = Engine::default();
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    for (name, prefix) in [("ac", ""), ("noise", "V(out) V1 ")] {
        assert!(execute(&mut circuit, &engine, name, &format!("{prefix}data=bad")).is_err());
        assert!(circuit.datasets().is_empty());
    }
    let cost = engine
        .run_ac_table(&netlist, "points")
        .unwrap()
        .retained_value_count();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = cost;
    let bounded = Engine::new(config);
    execute(&mut circuit, &bounded, "ac", "data=points").unwrap();
    assert_eq!(circuit.datasets()[0].name, "ac1");
    let error = execute(&mut circuit, &bounded, "ac", "data=points").unwrap_err();
    assert!(matches!(
        error,
        ControlExecutionError::Simulation {
            line: 19,
            source: SimulationError::ResourceLimit(_)
        }
    ));
    assert_eq!(circuit.datasets().len(), 1);
    execute(&mut circuit, &engine, "ac", "data=points").unwrap();
    assert_eq!(circuit.datasets()[1].name, "ac2");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cancellation_after_work_and_before_publication_keeps_the_previous_dataset() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Cancel {
        polls: AtomicUsize,
        limit: usize,
    }
    impl rspice_core::AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
        }
    }
    let netlist = Netlist::parse(&format!("{NETWORK}{TABLE}.end\n")).unwrap();
    let engine = Engine::default();
    for (name, args) in [("ac", "data=points"), ("noise", "V(out) V1 data=points")] {
        let mut count = 0;
        for limit in [usize::MAX, 0, 1, 2] {
            let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
            execute(&mut circuit, &engine, "op", "").unwrap();
            let limit = match limit {
                1 => count / 2,
                2 => count - 1,
                other => other,
            };
            let abort = Cancel {
                polls: AtomicUsize::new(0),
                limit,
            };
            let outcome = circuit.execute(&engine, &command(name, args), &netlist.params, &abort);
            if limit == usize::MAX {
                outcome.unwrap();
                count = abort.polls.load(Ordering::Relaxed);
                assert!(count > 20);
            } else {
                assert!(matches!(
                    outcome,
                    Err(ControlExecutionError::Simulation {
                        source: SimulationError::Aborted,
                        ..
                    })
                ));
                assert_eq!(circuit.datasets().len(), 1);
                execute(&mut circuit, &engine, name, args).unwrap();
                assert_eq!(circuit.datasets()[1].name, format!("{name}1"));
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn tables_share_analysis_ordinals_with_ordinary_runs_in_a_mixed_session() {
    let netlist = Netlist::parse(&format!("{NETWORK}{TABLE}.end\n")).unwrap();
    let engine = Engine::default();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    for (name, args) in [
        ("op", ""),
        ("dc", "V1 0 1 1"),
        ("ac", "lin 2 10 100"),
        ("noise", "V(out) V1 lin 2 10 100"),
        ("tran", "1u 2u"),
        ("ac", "data=points"),
        ("noise", "V(out) V1 data=points"),
    ] {
        execute(&mut circuit, &engine, name, args).unwrap();
    }
    assert_eq!(
        circuit
            .datasets()
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>(),
        ["op1", "dc1", "ac1", "noise1", "tran1", "ac2", "noise2"]
    );
    assert_ne!(
        circuit.datasets()[2].analysis_id,
        circuit.datasets()[5].analysis_id
    );
    verify(&circuit, 5, &[27.0, 127.0, 77.0], 1e-6);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_table_budget_includes_coordinates_and_presentation_retention() {
    let netlist = Netlist::parse(&format!("{NETWORK}{TABLE}.end\n")).unwrap();
    let engine = Engine::default();
    let count = engine
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
        .unwrap()
        .retained_value_count();
    for limit in [count - 1, count] {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_result_values = limit;
        let bounded = Engine::new(config);
        let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
        let outcome = execute(&mut circuit, &bounded, "noise", "V(out) V1 data=points");
        if limit < count {
            assert!(matches!(
                outcome,
                Err(ControlExecutionError::Simulation {
                    source: SimulationError::ResourceLimit(_),
                    ..
                })
            ));
            assert!(circuit.datasets().is_empty());
        } else {
            outcome.unwrap();
            let error = execute(&mut circuit, &bounded, "print", "onoise_spectrum").unwrap_err();
            assert!(matches!(
                error,
                ControlExecutionError::Simulation {
                    source: SimulationError::ResourceLimit(_),
                    ..
                }
            ));
            assert_eq!(circuit.datasets().len(), 1);
            execute(&mut circuit, &engine, "print", "onoise_spectrum").unwrap();
        }
    }
}
