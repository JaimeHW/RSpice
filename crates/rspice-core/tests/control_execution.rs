//! Electrical execution of the original control scripts. Presentation requests
//! are retained for a frontend; these tests do not claim plot rendering.

use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlPresentation,
    ControlPresentationKind,
};
use rspice_core::execution::SignalUnit;
use rspice_core::execution::control::{ControlCommand, ControlLimits, ControlProgram};
use rspice_core::{ConvergenceConfig, Engine, Netlist, NoAbort, SimulationConfig};

fn drive(
    source: &str,
    engine: &Engine,
    configure: impl FnOnce(&mut Netlist),
) -> (ControlCircuit, Vec<ControlPresentation>) {
    let program =
        ControlProgram::parse_deck_with_abort(source, ControlLimits::default(), &NoAbort).unwrap();
    let mut netlist = Netlist::parse(program.declarative_source()).unwrap();
    configure(&mut netlist);
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let mut presentation = Vec::new();
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        if let ControlCommandEffect::Presentation(request) = circuit
            .execute(engine, &command, session.variables(), &NoAbort)
            .unwrap()
        {
            presentation.push(request);
        }
    }
    (circuit, presentation)
}

#[test]
fn original_foreach_bjt_script_runs_six_biases_against_independent_complex_currents() {
    let source = include_str!("../../../tests/paranoia/control_structs/foreach_bjt_ft.sp");
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig::robust(),
        ..SimulationConfig::default()
    });
    let (circuit, presentation) = drive(source, &engine, |netlist| {
        // Same numerical qualification settings as bjt_excess_phase.rs; the
        // authored script, bias loop, PTF and model remain unchanged.
        netlist.options.reltol = Some(1e-10);
        netlist.options.abstol = Some(1e-18);
        netlist.options.vntol = Some(1e-12);
        netlist.options.gmin = Some(0.0);
    });
    let reference = include_str!("testdata/gp_bfs17_ptf_ngspice46.tsv");
    assert_eq!(circuit.datasets().len(), 6);
    for (index, (dataset, bias)) in circuit
        .datasets()
        .iter()
        .zip(["0.5e-3", "1e-3", "5e-3", "10e-3", "50e-3", "100e-3"])
        .enumerate()
    {
        assert_eq!(dataset.name, format!("ac{}", index + 1));
        let ControlAnalysisResult::Ac(actual) = &dataset.result else {
            panic!("expected AC data");
        };
        let rows = reference
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .filter(|row| row[0] == bias && row[1] == "21")
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), 57);
        assert_eq!(actual.len(), rows.len());
        for (point, row) in actual.iter().zip(rows) {
            let frequency: f64 = row[2].parse().unwrap();
            assert!((point.frequency / frequency - 1.0).abs() < 1e-12);
            let expected =
                num_complex::Complex64::new(row[3].parse().unwrap(), row[4].parse().unwrap());
            let branch = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("vgain"))
                .unwrap();
            let error = (point.currents[branch] - expected).norm();
            assert!(
                error <= 2e-6 * expected.norm() + 1e-12,
                "bias {bias}, frequency {frequency}: {error}"
            );
        }
    }
    assert_eq!(presentation.len(), 1);
    let ControlPresentationKind::Plot { traces, options } = &presentation[0].kind else {
        panic!("expected resolved plot");
    };
    assert_eq!(traces.len(), 6);
    assert!(options.x_logarithmic && options.y_logarithmic);
    assert_eq!(options.y_limits, Some([0.1, 100.0]));
    for (trace, dataset) in traces.iter().zip(circuit.datasets()) {
        assert_eq!(trace.y.dataset, dataset.name);
        assert_eq!(trace.y.unit, SignalUnit::Ampere);
        assert_eq!(trace.x.unit, SignalUnit::Hertz);
        let ControlAnalysisResult::Ac(points) = &dataset.result else {
            unreachable!()
        };
        for ((x, y), point) in trace.x.samples.iter().zip(&trace.y.samples).zip(points) {
            let branch = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("vgain"))
                .unwrap();
            assert_eq!(x.re, point.frequency);
            assert_eq!(x.im, 0.0);
            assert_eq!(*y, point.currents[branch].norm().into());
        }
    }
}

#[test]
fn original_memristor_script_changes_frequency_and_restarts_each_uic_analysis() {
    let source = include_str!("../../../tests/paranoia/memristor/memristor.sp");
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig::robust(),
        ..SimulationConfig::default()
    });
    let (circuit, presentation) = drive(source, &engine, |_| {});
    assert_eq!(circuit.datasets().len(), 3);
    for (index, (dataset, frequency)) in circuit
        .datasets()
        .iter()
        .zip([1e8, 1.1e8, 1.4e8])
        .enumerate()
    {
        assert_eq!(dataset.name, format!("tran{}", index + 1));
        let ControlAnalysisResult::Transient(result) = &dataset.result else {
            panic!("expected transient data");
        };
        assert!(result.time.len() > 50);
        let source_node = result
            .node_names
            .iter()
            .position(|name| name == "1")
            .unwrap();
        let state_node = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("xmem.x1"))
            .unwrap();
        assert!(
            (result.voltages[state_node][0] - 7000.0).abs() < 1e-8,
            "each UIC run must start from its authored Rinit"
        );
        assert!((result.time.last().unwrap() * frequency - 1.0).abs() < 1e-12);
        for (&time, &voltage) in result.time.iter().zip(&result.voltages[source_node]) {
            let expected = -3.0 * (std::f64::consts::TAU * frequency * time).sin();
            assert!(
                (voltage - expected).abs() < 1e-9,
                "tran{} t={time}: {voltage} != {expected}",
                index + 1
            );
        }
        let movement = result.voltages[state_node]
            .iter()
            .map(|value| (value - 7000.0).abs())
            .fold(0.0_f64, f64::max);
        assert!(movement > 100.0, "the memristor state must actually evolve");
    }
    assert_eq!(
        presentation
            .iter()
            .filter(|request| request.command.name == "plot")
            .count(),
        4
    );
    assert_eq!(
        presentation
            .iter()
            .filter(|request| request.command.name == "settype")
            .count(),
        1
    );
    let ControlPresentationKind::Plot {
        traces: programming,
        options,
    } = &presentation[0].kind
    else {
        panic!("expected current groups");
    };
    assert_eq!(
        options.title.as_deref(),
        Some("Memristor with threshold: Internal Programming currents")
    );
    assert_eq!(
        programming.len(),
        circuit
            .datasets()
            .iter()
            .map(|dataset| {
                let ControlAnalysisResult::Transient(result) = &dataset.result else {
                    unreachable!()
                };
                result.branch_names.len()
            })
            .sum::<usize>()
    );
    for trace in programming {
        assert_eq!(trace.y.unit, SignalUnit::Ampere);
        assert_eq!(trace.y.current_sources.len(), 1);
        assert_eq!(trace.y.current_sources[0].dataset, trace.y.dataset);
    }
    for (request, voltage_axis, current) in [
        (&presentation[2], false, false),
        (&presentation[3], true, false),
        (&presentation[4], true, true),
    ] {
        let ControlPresentationKind::Plot { traces, .. } = &request.kind else {
            unreachable!()
        };
        assert_eq!(traces.len(), 3);
        for (trace, expected_name) in traces.iter().zip(["tran3", "tran1", "tran2"]) {
            assert_eq!(trace.y.dataset, expected_name);
            assert_eq!(trace.x.dataset, expected_name);
            assert_eq!(
                trace.y.unit,
                if current {
                    SignalUnit::Ampere
                } else {
                    SignalUnit::Ohm
                }
            );
            let dataset = circuit
                .datasets()
                .iter()
                .find(|dataset| dataset.name == expected_name)
                .unwrap();
            let ControlAnalysisResult::Transient(result) = &dataset.result else {
                unreachable!()
            };
            let expected_x = if voltage_axis {
                let index = result
                    .node_names
                    .iter()
                    .position(|name| name == "1")
                    .unwrap();
                &result.voltages[index]
            } else {
                &result.time
            };
            let expected_y = if current {
                let index = result
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("v1"))
                    .unwrap();
                &result.branch_currents[index]
            } else {
                let index = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("xmem.x1"))
                    .unwrap();
                &result.voltages[index]
            };
            assert_eq!(trace.y.samples.len(), expected_y.len());
            assert_eq!(trace.x.samples.len(), expected_x.len());
            for (actual, expected) in trace.y.samples.iter().zip(expected_y) {
                assert_eq!(*actual, (*expected).into());
            }
            for (actual, expected) in trace.x.samples.iter().zip(expected_x) {
                assert_eq!(*actual, (*expected).into());
            }
        }
    }
}

#[test]
fn failed_control_alteration_and_run_limit_preserve_the_circuit_and_completed_dataset() {
    let netlist =
        Netlist::parse("atomic control\nV1 in 0 dc 2 sin(0 1 1k)\nR1 in 0 1k\n.end\n").unwrap();
    let variables = netlist.params.clone();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_batch_runs = 1;
    let engine = Engine::new(config);
    let op = ControlCommand {
        line: 10,
        name: "op".into(),
        arguments: String::new(),
    };
    circuit.execute(&engine, &op, &variables, &NoAbort).unwrap();
    let before = format!("{:?}", circuit.netlist().elements);
    let bad = ControlCommand {
        line: 11,
        name: "alter".into(),
        arguments: "@V1[sin] [ 9 8 7 6 5 1e999 ]".into(),
    };
    assert!(
        circuit
            .execute(&engine, &bad, &variables, &NoAbort)
            .is_err()
    );
    assert_eq!(format!("{:?}", circuit.netlist().elements), before);
    assert!(circuit.execute(&engine, &op, &variables, &NoAbort).is_err());
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(circuit.datasets()[0].name, "op1");
    let ControlAnalysisResult::OperatingPoint(result) = &circuit.datasets()[0].result else {
        panic!("expected OP data");
    };
    assert_eq!(result.voltage(1), 2.0);
}

#[test]
fn control_options_drive_the_integrator_and_survive_source_replay_atomically() {
    let source = "ordered solver options\n.param rval=1k\nV1 in 0 1\nR1 in out {rval}\nC1 out 0 1u ic=0\n.options reltol=1e-3\n.step param rval list 1k 2k\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let variables = netlist.params.clone();
    let engine = Engine::new(SimulationConfig::default());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let command = |name: &str, arguments: &str| ControlCommand {
        line: 9,
        name: name.into(),
        arguments: arguments.into(),
    };
    circuit
        .execute(
            &engine,
            &command("option", "method=trap xmu=0 trtol=2"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    circuit
        .execute(
            &engine,
            &command("set", "num_threads = 1"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    circuit
        .execute(&engine, &command("set", "noinit"), &variables, &NoAbort)
        .unwrap();
    assert!(circuit.settings().suppress_initial_listing);
    assert_eq!(circuit.settings().maximum_parallel_workers, Some(1));
    circuit
        .execute(
            &engine,
            &command("tran", ".1m 2m uic"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    let ControlAnalysisResult::Transient(result) = &circuit.datasets()[0].result else {
        panic!("transient");
    };
    let column = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let voltage = &result.voltages[column];
    // XMU=0 is backward Euler even when METHOD=TRAP was selected. Check
    // every accepted interval against the independent RC recurrence.
    for index in 1..result.time.len() {
        let ratio = (result.time[index] - result.time[index - 1]) / 1e-3;
        let expected = (voltage[index - 1] + ratio) / (1.0 + ratio);
        assert!((voltage[index] - expected).abs() < 1e-9, "row {index}");
    }
    let completed = voltage.clone();
    circuit
        .execute(
            &engine,
            &command("options", "xmu=.49 reltol=1e-4"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    for arguments in [
        "xmu=.1 reltol=-1",
        "xmu=.1 misspelled=4",
        "xmu=.1 restart file=data",
    ] {
        assert!(
            circuit
                .execute(&engine, &command("option", arguments), &variables, &NoAbort)
                .is_err()
        );
        assert_eq!(circuit.netlist().options.xmu, Some(0.49));
        assert_eq!(circuit.netlist().options.reltol, Some(1e-4));
    }
    assert!(
        circuit
            .execute(
                &engine,
                &command("set", "num_threads=0"),
                &variables,
                &NoAbort
            )
            .is_err()
    );
    assert_eq!(circuit.settings().maximum_parallel_workers, Some(1));
    let steps = circuit
        .netlist()
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            rspice_core::netlist::AnalysisCommand::Step(step) => Some(step.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let plan = engine
        .plan_step_commands(
            circuit.netlist(),
            &steps,
            rspice_core::engine::StepPlanLimits::from_resource_limits(
                engine.config().resource_limits,
            ),
        )
        .unwrap();
    let replayed = engine.materialize_step_run(&plan, 1).unwrap();
    assert_eq!(replayed.netlist().options.xmu, Some(0.49));
    assert_eq!(replayed.netlist().options.reltol, Some(1e-4));
    assert_eq!(replayed.netlist().options.trtol, Some(2.0));
    let ControlAnalysisResult::Transient(result) = &circuit.datasets()[0].result else {
        panic!("retained transient");
    };
    assert_eq!(result.voltages[column], completed);
}

#[test]
fn control_vectors_preserve_probe_spelling_and_refuse_misaligned_or_unavailable_data() {
    let source =
        "control vectors\nV1 001 0 dc 0 ac 2 30\nV2 1 0 dc 0 ac 5\nR1 001 0 1k\nR2 1 0 1k\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    let variables = netlist.params.clone();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let command = |name: &str, arguments: &str| ControlCommand {
        line: 7,
        name: name.into(),
        arguments: arguments.into(),
    };
    for arguments in ["lin 2 1k 2k", "lin 2 2k 3k"] {
        circuit
            .execute(&engine, &command("ac", arguments), &variables, &NoAbort)
            .unwrap();
    }
    let read = |circuit: &mut ControlCircuit, text: &str| {
        let ControlCommandEffect::Presentation(request) = circuit
            .execute(&engine, &command("print", text), &variables, &NoAbort)
            .unwrap()
        else {
            unreachable!()
        };
        let ControlPresentationKind::Print(traces) = request.kind else {
            unreachable!()
        };
        traces
    };
    let traces = read(&mut circuit, "ac1.v(001,1) abs(ac1.v(001)) ac2.v(001)");
    let expected = num_complex::Complex64::from_polar(2.0, 30f64.to_radians());
    assert!((traces[0].y.samples[0] - (expected - 5.0)).norm() < 1e-12);
    assert!((traces[1].y.samples[0].re - 2.0).abs() < 1e-12);
    assert!((traces[2].y.samples[0] - expected).norm() < 1e-12);
    assert_eq!(traces[0].x.samples[0].re, 1000.0);
    assert_eq!(traces[2].x.samples[0].re, 2000.0);
    // Even equal-length vectors cannot be combined by row across different grids.
    for text in [
        "ac1.v(001) + ac2.v(001)",
        "ac1.v(001) vs ac2.v(1)",
        "ac9.v(001)",
        "v(missing)",
    ] {
        assert!(
            circuit
                .execute(&engine, &command("print", text), &variables, &NoAbort)
                .is_err(),
            "{text}"
        );
        // ngspice -b never draws, so a plot of such data is not drawn and the
        // script continues; it is never drawn on a wrong grid either.
        assert!(
            matches!(
                circuit
                    .execute(&engine, &command("plot", text), &variables, &NoAbort)
                    .unwrap(),
                ControlCommandEffect::Presentation(ControlPresentation {
                    kind: ControlPresentationKind::Ignored(_),
                    ..
                })
            ),
            "{text}"
        );
    }
    assert!(
        circuit
            .execute(
                &engine,
                &command("settype", "impedance ac1.v(001) missing"),
                &variables,
                &NoAbort
            )
            .is_err()
    );
    assert_eq!(read(&mut circuit, "ac1.v(001)")[0].y.unit, SignalUnit::Volt);
    circuit
        .execute(
            &engine,
            &command("settype", "impedance ac1.v(001)"),
            &variables,
            &NoAbort,
        )
        .unwrap();
    let typed = read(&mut circuit, "ac1.v(001) ac2.v(001)");
    assert_eq!(typed[0].y.unit, SignalUnit::Ohm);
    assert_eq!(typed[1].y.unit, SignalUnit::Volt);
    assert_eq!(typed[0].y.samples, traces[2].y.samples);

    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = 1;
    let bounded = Engine::new(config);
    assert!(
        circuit
            .execute(
                &bounded,
                &command("plot", "ac1.v(001)"),
                &variables,
                &NoAbort
            )
            .is_err()
    );
    assert_eq!(circuit.datasets().len(), 2);
}

#[test]
fn sealed_source_control_retention_preserves_commands_and_author_locations() {
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};
    let root = std::path::PathBuf::from("C:/sealed-control/root.sp");
    let child = std::path::PathBuf::from("C:/sealed-control/commands.inc");
    let source = "sealed control\nV1 out 0 0\nR1 out 0 1k\n.include commands.inc\n.end\n";
    let commands =
        ".control\nforeach bias 1 2\nalter v1 $bias\nop\nend\nprint op1.v(out) op2.v(out)\n.endc\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [
            (root.clone(), source.into()),
            (child.clone(), commands.into()),
        ],
        [SealedSourceEdge {
            owner: root.clone(),
            requested_path: "commands.inc".into(),
            target: child.clone(),
        }],
    )
    .unwrap();
    let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &root,
        bundle,
        NetlistParseOptions {
            retain_control_script: true,
            ..NetlistParseOptions::default()
        },
        &NoAbort,
    )
    .unwrap();
    assert!(netlist.analyses.is_empty());
    assert!(netlist.control_dispositions.is_empty());
    let script = netlist.control_script.clone().unwrap();
    let program =
        ControlProgram::parse_deck_with_abort(script.text(), ControlLimits::default(), &NoAbort)
            .unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        let origin = script.origin(command.line).unwrap();
        assert_eq!(origin.path.as_ref(), Some(&child));
        if command.name == "op" {
            assert_eq!(origin.line, 4);
        }
        circuit
            .execute(&engine, &command, session.variables(), &NoAbort)
            .unwrap();
    }
    assert_eq!(circuit.datasets().len(), 2);
    assert_eq!(circuit.datasets()[1].analysis_id.tag(), "op-002");
}

#[test]
fn a_transient_host_gets_runtime_identity_bound_postprocesses_and_resolved_options() {
    use rspice_core::engine::ControlExecutionError;
    use rspice_core::netlist::AnalysisCommand;
    let deck = Netlist::parse("hosted transients\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 10u 2m\n.four 1k V(out)\n.tran 10u 3m\n.four 2k V(out)\n.fft V(out) np=16\n.end\n").unwrap();
    let vars = deck.params.clone();
    let mut circuit = ControlCircuit::new(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let command = |name: &str, arguments: &str| ControlCommand {
        line: 7,
        name: name.into(),
        arguments: arguments.into(),
    };
    circuit
        .execute(&engine, &command("option", "temp=50"), &vars, &NoAbort)
        .unwrap();
    let mut observed = Vec::new();
    circuit
        .execute_with_transient_runner(
            &engine,
            &command("run", ""),
            &vars,
            &NoAbort,
            &mut |engine, deck, command, id, abort| -> Result<_, ControlExecutionError> {
                assert_eq!(engine.config().temperature, 323.15);
                let four = deck
                    .analyses
                    .iter()
                    .filter(|card| matches!(card, AnalysisCommand::Four { .. }))
                    .count();
                observed.push((
                    id.tag(),
                    four,
                    deck.fft_analyses.len(),
                    engine.config().resource_limits.max_result_values,
                ));
                let AnalysisCommand::Tran { step, stop, .. } = command else {
                    panic!("transient callback only")
                };
                engine
                    .run_tran_with_abort(deck, *stop, *step, abort)
                    .map_err(|source| ControlExecutionError::Simulation { line: 7, source })
            },
        )
        .unwrap();
    assert_eq!(
        (&observed[0].0, observed[0].1, observed[0].2),
        (&"tran-001".into(), 1, 1)
    );
    assert_eq!(
        (&observed[1].0, observed[1].1, observed[1].2),
        (&"tran-002".into(), 1, 0)
    );
    assert!(
        observed[1].3 < observed[0].3,
        "later callbacks receive the remaining result budget"
    );
    assert_eq!(circuit.datasets().len(), 2);
    assert_eq!(circuit.datasets()[1].analysis_id.tag(), "tran-002");
}

#[test]
fn a_transient_host_failure_is_typed_and_does_not_consume_a_dataset_identity() {
    use rspice_core::engine::ControlExecutionError;
    use rspice_core::execution::control::ControlError;
    #[derive(Debug, PartialEq)]
    enum HostError {
        Core,
        Output,
    }
    impl From<ControlExecutionError> for HostError {
        fn from(_: ControlExecutionError) -> Self {
            Self::Core
        }
    }
    impl From<ControlError> for HostError {
        fn from(_: ControlError) -> Self {
            Self::Core
        }
    }
    let deck = Netlist::parse("host failure\nV1 n 0 1\nR1 n 0 1k\n.end\n").unwrap();
    let vars = deck.params.clone();
    let mut circuit = ControlCircuit::new(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let command = ControlCommand {
        line: 4,
        name: "tran".into(),
        arguments: "1u 10u".into(),
    };
    let error = circuit
        .execute_with_transient_runner(&engine, &command, &vars, &NoAbort, &mut |_, _, _, _, _| {
            Err::<rspice_core::engine::TransientResult, _>(HostError::Output)
        })
        .unwrap_err();
    assert_eq!(error, HostError::Output);
    assert!(circuit.datasets().is_empty());
    circuit.execute(&engine, &command, &vars, &NoAbort).unwrap();
    assert_eq!(circuit.datasets()[0].analysis_id.tag(), "tran-001");
}

#[test]
fn control_voltage_expressions_retain_unique_node_and_dataset_provenance() {
    let (_, output) = drive(
        "Voltage provenance\nV1 out 0 1\nV2 ref 0 2\n.control\ntran 1u 10u\nprint v(out,ref)+v(out)\n.endc\n.end\n",
        &Engine::new(SimulationConfig::default()),
        |_| {},
    );
    let ControlPresentationKind::Print(traces) = &output[0].kind else {
        panic!("expected PRINT");
    };
    assert_eq!(traces.len(), 1);
    let vector = &traces[0].y;
    assert!(vector.current_sources.is_empty());
    assert_eq!(vector.voltage_sources.len(), 2);
    let mut names = vector
        .voltage_sources
        .iter()
        .map(|source| {
            assert_eq!(source.dataset, "tran1");
            source.node_name.to_ascii_lowercase()
        })
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["out", "ref"]);
    assert!(traces[0].x.voltage_sources.is_empty());
    assert!(vector.samples.iter().all(|value| value.norm() < 1e-12));
}

#[test]
fn batch_commands_print_or_continue_as_ngspice_b_runs_them() {
    let netlist =
        Netlist::parse("batch deck\nV1 in 0 dc 2\nR1 in out 1k\nR2 out 0 3k\n.end\n").unwrap();
    let variables = netlist.params.clone();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let command = |name: &str, arguments: &str| ControlCommand {
        line: 9,
        name: name.into(),
        arguments: arguments.into(),
    };
    let kind = |circuit: &mut ControlCircuit, name: &str, arguments: &str| match circuit
        .execute(&engine, &command(name, arguments), &variables, &NoAbort)
        .unwrap()
    {
        ControlCommandEffect::Presentation(request) => Some(request.kind),
        _ => None,
    };
    let text = |kind: Option<ControlPresentationKind>| match kind {
        Some(ControlPresentationKind::Text(text)) => text,
        other => panic!("expected batch text, got {other:?}"),
    };
    assert_eq!(
        text(kind(&mut circuit, "display", "")),
        "There are no vectors currently active.\n"
    );
    kind(&mut circuit, "op", "");
    // com_echo.c: words unquoted and joined by one space; -n drops the newline.
    assert_eq!(
        text(kind(&mut circuit, "echo", "hello   \"a  b\" 'c'")),
        "hello a  b c\n"
    );
    assert_eq!(text(kind(&mut circuit, "echo", "-n x")), "x");
    assert_eq!(text(kind(&mut circuit, "echo", "")), "\n");
    // pvec.c: "    %-20s: type, real, N long", sorted by name.
    assert_eq!(
        text(kind(&mut circuit, "display", "")),
        "Here are the vectors currently active:\n\nTitle: batch deck\n\
         Name: op1 (Operating Point)\n\n\
         \x20   in                  : voltage, real, 1 long\n\
         \x20   out                 : voltage, real, 1 long\n\
         \x20   v1#branch           : current, real, 1 long\n"
    );
    kind(&mut circuit, "dc", "v1 0 1 0.5");
    assert_eq!(
        text(kind(&mut circuit, "setplot", "")),
        "List of plots available:\n\nCurrent dc1\tbatch deck (DC transfer characteristic)\n\
         \top1\tbatch deck (Operating Point)\n"
    );
    // setplot selects the plot unqualified names resolve against.
    assert!(kind(&mut circuit, "setplot", "op1").is_none());
    let Some(ControlPresentationKind::Print(traces)) = kind(&mut circuit, "print", "v(out)") else {
        panic!("expected a print")
    };
    assert_eq!(traces[0].y.dataset, "op1");
    assert_eq!(traces[0].y.samples.len(), 1);
    assert!(
        circuit
            .execute(&engine, &command("setplot", "tran9"), &variables, &NoAbort)
            .is_err()
    );
    assert!(matches!(
        kind(&mut circuit, "listing", ""),
        Some(ControlPresentationKind::Listing)
    ));
    assert!(matches!(
        kind(&mut circuit, "rusage", "all"),
        Some(ControlPresentationKind::ResourceUsage)
    ));
    // ngspice -b ignores what has no batch meaning, and so does a plot it
    // cannot resolve, since com_plot.c never evaluates it in batch mode.
    for (name, arguments) in [
        ("gnuplot", "out v(out)"),
        ("hardcopy", "out.ps v(out)"),
        ("shell", "rm -rf out"),
        ("edit", ""),
        ("plot", "v(missing)"),
    ] {
        assert!(
            matches!(
                kind(&mut circuit, name, arguments),
                Some(ControlPresentationKind::Ignored(_))
            ),
            "{name}"
        );
    }
    // Every ngspice plot keyword is accepted, and the plot is still drawn.
    let Some(ControlPresentationKind::Plot { options, traces }) = kind(
        &mut circuit,
        "plot",
        "v(out) ylabel 'Output voltage' xlabel volts xl 0 1 nogrid xdelta 0.1 title out",
    ) else {
        panic!("expected a plot")
    };
    assert_eq!(options.title.as_deref(), Some("out"));
    assert_eq!(options.x_limits, Some([0.0, 1.0]));
    assert_eq!(traces.len(), 1);
    // Analysis commands rspice cannot run still refuse.
    assert!(
        circuit
            .execute(&engine, &command("pss", "1k"), &variables, &NoAbort)
            .is_err()
    );
}

#[test]
fn set_accepts_presentation_variables_and_applies_assignments_atomically() {
    let netlist = Netlist::parse("set deck\nV1 in 0 1\nR1 in 0 1k\n.end\n").unwrap();
    let variables = netlist.params.clone();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let set = |circuit: &mut ControlCircuit, arguments: &str| {
        circuit.execute(
            &engine,
            &ControlCommand {
                line: 3,
                name: "set".into(),
                arguments: arguments.into(),
            },
            &variables,
            &NoAbort,
        )
    };
    let error = set(&mut circuit, "noinit num_threads=2 units=degrees").unwrap_err();
    assert!(error.to_string().contains("'units'"), "{error}");
    assert!(!circuit.settings().suppress_initial_listing);
    assert_eq!(circuit.settings().maximum_parallel_workers, None);
    set(
        &mut circuit,
        "noaskquit xbrushwidth=2 color0=white color2=\"#ee82ee\" hcopydevtype = postscript \
         nolegend gnuplot_terminal=png noinit num_threads = 2",
    )
    .unwrap();
    assert!(circuit.settings().suppress_initial_listing);
    assert_eq!(circuit.settings().maximum_parallel_workers, Some(2));
    assert!(set(&mut circuit, "colorx=red").is_err());
    // ngspice's noacct also suppresses the initial transient listing.
    let netlist = Netlist::parse("set deck\nV1 in 0 1\nR1 in 0 1k\n.end\n").unwrap();
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    set(&mut circuit, "noacct").unwrap();
    assert!(circuit.settings().suppress_initial_listing);
}

/// Run a script to its first failure, returning what it presented and that
/// failure's message.
fn run_until_error(source: &str) -> (Vec<ControlPresentation>, Option<String>) {
    let engine = Engine::new(SimulationConfig::default());
    let program =
        ControlProgram::parse_deck_with_abort(source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let mut presentation = Vec::new();
    loop {
        let command = match session.next_command(&mut circuit, &NoAbort) {
            Ok(Some(command)) => command,
            Ok(None) => return (presentation, None),
            Err(error) => return (presentation, Some(error.to_string())),
        };
        match circuit.execute(&engine, &command, session.variables(), &NoAbort) {
            Ok(ControlCommandEffect::Presentation(request)) => presentation.push(request),
            Ok(_) => {}
            Err(error) => return (presentation, Some(error.to_string())),
        }
    }
}

fn printed(request: &ControlPresentation) -> &[rspice_core::engine::ControlTrace] {
    match &request.kind {
        ControlPresentationKind::Print(traces) => traces,
        other => panic!("expected a print, got {other:?}"),
    }
}

/// ngspice com_let.c binds a vector in the current plot (vectors.c vec_get
/// reads the current plot, then the constants plot), with the source's scale;
/// evaluate.c doop pads the shorter operand with its last element.
#[test]
fn let_binds_vectors_per_plot_and_print_reads_them_as_ngspice_does() {
    let source = "let vectors\nV1 in 0 PULSE(2 4 1u 1u 1u 5u 20u)\nR1 in out 1k\n\
        R2 out 0 3k\nC1 out 0 1n\n.control\nlet a = 2*3\necho $a\nop\nlet x = v(out)\n\
        let y = i(v1) * a\nprint x y\nif x > 1.4\necho over\nend\ntran 0.1u 4u\n\
        let z = v(out) * 2 + v(in)\nlet q = op1.x + z\nprint z q v(out) v(in)\n\
        let x = op1.x - 1\nprint x op1.x\nlet a = 1\necho $a\nprint v(x)\n.endc\n.end\n";
    let (presented, error) = run_until_error(source);
    assert_eq!(error, None);
    let text = |index: usize| match &presented[index].kind {
        ControlPresentationKind::Text(text) => text.clone(),
        other => panic!("expected text, got {other:?}"),
    };
    // A `let` that reads no vector stays a session parameter, as before.
    assert_eq!(text(0), format!("{:.17e}\n", 6.0));
    let op = printed(&presented[1]);
    assert_eq!(op[0].y.expression, "x");
    assert_eq!(op[0].y.samples.len(), 1);
    assert!((op[0].y.samples[0].re - 1.5).abs() < 1e-9);
    assert_eq!(op[0].y.unit, SignalUnit::Volt);
    assert!((op[1].y.samples[0].re + 3e-3).abs() < 1e-12);
    assert_eq!(text(2), "over\n");
    let tran = printed(&presented[3]);
    let (z, q, vout, vin) = (&tran[0], &tran[1], &tran[2], &tran[3]);
    assert!(vout.y.samples.len() > 10);
    assert_eq!(z.y.samples.len(), vout.y.samples.len());
    assert_eq!(q.y.samples.len(), vout.y.samples.len());
    assert_eq!(z.x.expression, "time");
    assert_eq!(z.x.samples, vout.x.samples);
    assert_eq!(q.x.samples, vout.x.samples);
    for row in 0..vout.y.samples.len() {
        let expected = vout.y.samples[row] * 2.0 + vin.y.samples[row];
        assert!((z.y.samples[row] - expected).norm() < 1e-12, "z[{row}]");
        let padded = op[0].y.samples[0] + expected;
        assert!((q.y.samples[row] - padded).norm() < 1e-12, "q[{row}]");
    }
    // `x` was bound in op1; the new `x` belongs to tran1.
    let rebound = printed(&presented[4]);
    assert!((rebound[0].y.samples[0].re - 0.5).abs() < 1e-9);
    assert!((rebound[1].y.samples[0].re - 1.5).abs() < 1e-9);
    assert_eq!(text(5), format!("{:.17e}\n", 1.0));
    // ngspice reads v(x) as the vector x.
    assert_eq!(printed(&presented[6])[0].y.samples, rebound[0].y.samples);

    // A vector of another plot is not in scope, and a vector is not a shell
    // variable: `$y` cannot expand it.
    let base = "let scope\nV1 in 0 2\nR1 in 0 1k\n.control\nop\nlet y = v(in)\n";
    let (_, error) = run_until_error(&format!("{base}tran 1u 2u\nprint y\n.endc\n.end\n"));
    assert!(error.unwrap().contains("'Y' is unavailable"));
    let (_, error) = run_until_error(&format!("{base}echo $y\n.endc\n.end\n"));
    assert!(error.unwrap().contains("names a vector"));
    // A scalar `let` of the same name replaces the vector.
    let (presented, error) =
        run_until_error(&format!("{base}let y = 7\necho $y\nprint y\n.endc\n.end\n"));
    assert_eq!(error, None);
    assert!(
        matches!(&presented[0].kind, ControlPresentationKind::Text(text) if text.starts_with("7.0"))
    );
    assert_eq!(printed(&presented[1])[0].y.samples[0].re, 7.0);
    // A condition reads one element; a longer vector is no scalar.
    let (_, error) = run_until_error(
        "let long\nV1 in 0 2\nR1 in 0 1k\n.control\ntran 1u 4u\nlet w = v(in)\nif w > 1\n\
         end\n.endc\n.end\n",
    );
    assert!(error.unwrap().contains("where a scalar is required"));
}

/// ngspice com_define (frontend/define.c:41) binds a user function; a call
/// substitutes the argument trees for its formals (define.c:299 trcopy), and
/// a later definition of the name replaces it.
#[test]
fn define_binds_user_functions_for_scalars_conditions_and_vectors() {
    let source = "define\nV1 in 0 2\nR1 in out 1k\nR2 out 0 3k\n.control\n\
        define mismatch(a,b,err) abs(a-b)>err\ndefine sq(x) x*x\nop\nlet s = sq(3)\necho $s\n\
        let w = sq(v(out))\nprint w sq(v(out)) mismatch(v(out),1,0.1)\n\
        if mismatch(v(out), 1.5, 1e-9)\necho bad\nelse\necho good\nend\n\
        define sq ( x ) x*x*x\nprint sq(v(out))\n.endc\n.end\n";
    let (presented, error) = run_until_error(source);
    assert_eq!(error, None);
    let text = |index: usize| match &presented[index].kind {
        ControlPresentationKind::Text(text) => text.clone(),
        other => panic!("expected text, got {other:?}"),
    };
    // ngspice-46 prints s = 9, w = 2.25, sq(v(out)) = 2.25,
    // mismatch(v(out),1,0.1) = 1, then good, then sq(v(out)) = 3.375.
    // A call that reads no vector stays a session scalar.
    assert_eq!(text(0), format!("{:.17e}\n", 9.0));
    let values: Vec<_> = printed(&presented[1])
        .iter()
        .map(|trace| trace.y.samples[0].re)
        .collect();
    for (value, expected) in values.iter().zip([2.25, 2.25, 1.0]) {
        assert!((value - expected).abs() < 1e-9, "{values:?}");
    }
    assert_eq!(text(2), "good\n");
    assert!((printed(&presented[3])[0].y.samples[0].re - 3.375).abs() < 1e-9);

    for (definition, message) in [
        ("define abs(x) x", "predefined"),
        ("define f(x)", "body"),
        ("define f x", "body"),
        ("define f(1x) x", "invalid"),
    ] {
        let (_, error) = run_until_error(&format!(
            "bad define\nV1 in 0 1\nR1 in 0 1k\n.control\n{definition}\n.endc\n.end\n"
        ));
        let error = error.unwrap();
        assert!(error.contains(message), "{definition}: {error}");
    }
}

/// ngspice's whole-vector functions (maths/cmaths) and its predefined
/// functions and constants (frontend/cpitf.c), against ngspice-46 on this
/// script. ngspice prints seven significant digits, six for a negative
/// value; each value is checked at half a unit of its last printed digit,
/// with a relative 1e-9 for a value that rounds at the half.
#[test]
fn vector_functions_match_ngspice() {
    let source = "vector functions\nV1 in 0 dc 0 ac 1\nR1 in out 1k\nC1 out 0 1u\nR2 out 0 3k\n\
        .control\ndc v1 0 2 0.5\nlet y = v(out)*v(out)*v(out)\n\
        print deriv(y) deriv(v(out)*v(out))\nprint vecmax(y) vecmin(y) mean(y) length(y)\n\
        print vector(4) unitvec(3)\nlet s = sqrt(v(out) - 1)\nprint s\n\
        print floor(y) nint(y) pos(v(out) - 0.7)\nprint max(v(out), 0.6) min(v(out), 0.6)\n\
        print exp(v(out)) log(v(out)+1) log10(v(out)+1)\nac dec 2 10 1k\n\
        print db(v(out)) ph(v(out)) cph(v(out)) mag(v(out))\nprint vdb(out) vp(out) vm(out)\n\
        print j(v(out)) conj(v(out)) sqrt(v(out)) mean(v(out)) vecmax(v(out))\n\
        print log(v(out)) log10(v(out)) exp(v(out))\nprint pi e boltz i*2\n.endc\n.end\n";
    let (presented, error) = run_until_error(source);
    assert_eq!(error, None);
    let samples = |print: usize, trace: usize| printed(&presented[print])[trace].y.samples.clone();
    let close = |actual: f64, printed: &str| {
        let expected: f64 = printed.parse().unwrap();
        let tolerance = match printed.split_once('e') {
            Some((mantissa, exponent)) => {
                let decimals = mantissa.split_once('.').map_or(0, |(_, d)| d.len()) as i32;
                0.5 * 10f64.powi(exponent.parse::<i32>().unwrap() - decimals)
            }
            None => 1e-12,
        };
        (actual - expected).abs() <= tolerance + 1e-9 * expected.abs()
    };
    let check = |print: usize, trace: usize, expected: &[&str]| {
        let actual = samples(print, trace);
        assert_eq!(actual.len(), expected.len(), "print {print} trace {trace}");
        for (value, expected) in actual.iter().zip(expected) {
            let (re, im) = expected.split_once(',').unwrap_or((expected, "0"));
            assert!(
                close(value.re, re) && close(value.im, im),
                "print {print} trace {trace}: {actual:?} against {expected:?}"
            );
        }
    };
    check(
        0,
        0,
        &[
            "-2.10938e-01",
            "4.218750e-01",
            "1.371094e+00",
            "2.953125e+00",
            "4.851562e+00",
        ],
    );
    check(
        0,
        1,
        &[
            "0.000000e+00",
            "5.625000e-01",
            "1.125000e+00",
            "1.687500e+00",
            "2.250000e+00",
        ],
    );
    for (trace, expected) in [
        "3.375000e+00",
        "0.000000e+00",
        "1.054688e+00",
        "5.000000e+00",
    ]
    .into_iter()
    .enumerate()
    {
        check(1, trace, &[expected]);
    }
    check(2, 0, &["0", "1", "2", "3"]);
    check(2, 1, &["1", "1", "1"]);
    check(
        3,
        0,
        &[
            "0,1.000000e+00",
            "0,7.905694e-01",
            "0,5.000000e-01",
            "3.535534e-01,0",
            "7.071068e-01,0",
        ],
    );
    check(4, 0, &["0", "0", "0", "1", "3"]);
    check(4, 1, &["0", "0", "0", "1", "3"]);
    check(4, 2, &["0", "0", "1", "1", "1"]);
    check(
        5,
        0,
        &[
            "6.000000e-01",
            "6.000000e-01",
            "7.500000e-01",
            "1.125000e+00",
            "1.500000e+00",
        ],
    );
    check(
        5,
        1,
        &[
            "0",
            "3.750000e-01",
            "6.000000e-01",
            "6.000000e-01",
            "6.000000e-01",
        ],
    );
    assert!(close(samples(6, 0)[1].re, "1.454991e+00"));
    assert!(close(samples(6, 1)[1].re, "3.184537e-01"));
    assert!(close(samples(6, 2)[1].re, "1.383027e-01"));
    let db = [
        "-2.50841e+00",
        "-2.59416e+00",
        "-3.36972e+00",
        "-7.57822e+00",
        "-1.61549e+01",
    ];
    let ph = [
        "-4.70891e-02",
        "-1.47930e-01",
        "-4.40375e-01",
        "-9.79761e-01",
        "-1.36169e+00",
    ];
    let mag = [
        "7.491686e-01",
        "7.418087e-01",
        "6.784439e-01",
        "4.179158e-01",
        "1.556881e-01",
    ];
    for (print, traces) in [(7, [0, 1, 2, 3]), (8, [0, 1, 1, 2])] {
        check(print, traces[0], &db);
        check(print, traces[1], &ph);
        check(print, traces[2], &ph);
        check(print, traces[3], &mag);
    }
    assert_eq!(printed(&presented[7])[0].x.expression, "frequency");
    let first = |print: usize, trace: usize, expected: &str| {
        check_first(&samples(print, trace), expected, &close)
    };
    first(9, 0, "3.526461e-02,7.483382e-01");
    first(9, 1, "7.483382e-01,3.526461e-02");
    first(9, 2, "-8.65305e-01,2.037697e-02");
    first(9, 3, "4.721899e-01,-1.86625e-01");
    first(9, 4, "7.483382e-01,-3.52646e-02");
    first(10, 0, "-2.88791e-01,-4.70891e-02");
    first(10, 1, "-1.25420e-01,-4.70891e-02");
    first(10, 2, "2.112171e+00,-7.45158e-02");
    first(11, 0, "3.141593e+00,0");
    first(11, 1, "2.718282e+00,0");
    first(11, 2, "1.380649e-23,0");
    first(11, 3, "0,2.000000e+00");
}

fn check_first(
    samples: &[num_complex::Complex64],
    expected: &str,
    close: &dyn Fn(f64, &str) -> bool,
) {
    let (re, im) = expected.split_once(',').unwrap();
    assert!(
        close(samples[0].re, re) && close(samples[0].im, im),
        "{samples:?} against {expected}"
    );
}
