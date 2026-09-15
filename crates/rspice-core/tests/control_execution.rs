//! Electrical execution of the original control scripts. Presentation requests
//! are retained for a frontend; these tests do not claim plot rendering.

use rspice_core::engine::{ControlAnalysisResult, ControlCircuit, ControlCommandEffect};
use rspice_core::execution::control::{ControlCommand, ControlLimits, ControlProgram};
use rspice_core::{ConvergenceConfig, Engine, Netlist, NoAbort, SimulationConfig};

fn drive(
    source: &str,
    engine: &Engine,
    configure: impl FnOnce(&mut Netlist),
) -> (ControlCircuit, Vec<ControlCommand>) {
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
    assert_eq!(presentation[0].name, "plot");
    assert!(presentation[0].arguments.contains("ac1.vgain#branch"));
    assert!(presentation[0].arguments.contains("ac6.vgain#branch"));
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
            .filter(|command| command.name == "plot")
            .count(),
        4
    );
    assert_eq!(
        presentation
            .iter()
            .filter(|command| command.name == "settype")
            .count(),
        1
    );
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
