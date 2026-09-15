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
                .execute(&engine, &command("plot", text), &variables, &NoAbort)
                .is_err(),
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
