use rspice_core::analysis::{MeasureEngine, bind_error_measurement_reference};
use rspice_core::netlist::measure::{MeasureStatement, MeasureType};
use std::collections::HashMap;

fn statement(family: &str, norm: &str) -> MeasureStatement {
    let source = format!(
        "Bound comparison\nV1 out 0 1\nR1 out 0 1k\n.MEAS {family} fit ERROR V(out) FILE=unavailable-reference.csv COMP_FUNCTION={norm} INDEPVARCOL=0 DEPVARCOL=1\n.end\n"
    );
    rspice_core::Netlist::parse(&source)
        .unwrap()
        .measurements
        .remove(0)
}

fn evaluated(statement: MeasureStatement) -> rspice_core::MeasureResult {
    let mut engine = MeasureEngine::new();
    engine.add(statement);
    let values = [0.0, 1.0, 2.0];
    let signals = HashMap::from([("V(OUT)".into(), values.as_slice())]);
    engine.evaluate(&[0.0, 1.0, 2.0], &signals).remove(0)
}

#[test]
fn sealed_measurement_reference_preserves_family_interpolation_and_norms() {
    // The middle reference coordinate is deliberately between accepted points.
    // DC retains its positional comparison semantics; other families interpolate.
    let contents = "TIME,V(out)\n0,0\n0.5,0\n2,0\n";
    for family in ["TRAN", "AC", "DC", "NOISE"] {
        for (norm, expected) in [
            ("L1NORM", if family == "DC" { 3.0 } else { 2.5 }),
            (
                "L2NORM",
                if family == "DC" {
                    5.0_f64.sqrt()
                } else {
                    4.25_f64.sqrt()
                },
            ),
            ("INFNORM", 2.0),
        ] {
            let mut statement = statement(family, norm);
            if let MeasureType::FileError { file, .. } = &statement.measure_type {
                assert_eq!(format!("{file:?}"), format!("{:?}", file.path()));
            }
            bind_error_measurement_reference(&mut statement, contents).unwrap();
            let result = evaluated(statement);
            assert!(result.passed, "{family}/{norm}: {result:?}");
            assert!(
                (result.value.unwrap() - expected).abs() < 1e-12,
                "{family}/{norm}: {result:?}"
            );
        }
    }
}

#[test]
fn sealed_measurement_reference_clones_are_isolated_and_invalid_rebinds_are_atomic() {
    let mut first = statement("TRAN", "L2NORM");
    bind_error_measurement_reference(&mut first, "TIME,V(out)\n0,0\n1,0\n2,0\n").unwrap();
    let mut second = first.clone();
    bind_error_measurement_reference(&mut second, "TIME,V(out)\n0,0\n1,1\n2,2\n").unwrap();
    assert_eq!(evaluated(second.clone()).value, Some(0.0));
    assert_eq!(evaluated(first).value, Some(5.0_f64.sqrt()));
    for invalid in [
        "",
        "TIME,V(out)\n0,0\n1,NaN\n",
        "TIME\n0\n1\n",
        "TIME,V(out)\n0,0\n2,2\n1,1\n",
        "TIME,V(out)\n-1,0\n0,1\n",
    ] {
        assert!(bind_error_measurement_reference(&mut second, invalid).is_err());
        assert_eq!(evaluated(second.clone()).value, Some(0.0));
    }
    let MeasureType::FileError { file, .. } = &second.measure_type else {
        panic!("ERROR expected")
    };
    assert!(file.contents().is_some());
}

#[test]
fn sealed_measurement_reference_reaches_live_equation_reads_without_file_access() {
    let source = "Bound live reference\nV1 out 0 1\nR1 out 0 1k\n.MEAS TRAN fit ERROR V(out) FILE=unavailable-live.csv COMP_FUNCTION=L2NORM INDEPVARCOL=0 DEPVARCOL=1\n.MEAS TRAN doubled EQN='fit*2'\n.end\n";
    let mut netlist = rspice_core::Netlist::parse(source).unwrap();
    let identity = |netlist: &rspice_core::Netlist| {
        rspice_core::Engine::default()
            .new_monte_carlo_checkpoint(
                netlist,
                &rspice_core::engine::MonteCarloStudyConfig::new(1, 7, vec!["fit".into()]),
                [0; 32],
                &rspice_core::NoAbort,
            )
            .unwrap()
            .population_identity()
    };
    let unbound_identity = identity(&netlist);
    bind_error_measurement_reference(&mut netlist.measurements[0], "TIME,V(out)\n0,2\n").unwrap();
    assert_ne!(unbound_identity, identity(&netlist));
    let waveform = rspice_core::engine::TransientResult {
        current_impulses: None,
        time: vec![0.0, 1.0, 2.0],
        step_sizes: vec![0.0; 3],
        voltages: vec![vec![0.0, 1.0, 2.0]],
        branch_currents: Vec::new(),
        num_nodes: 1,
        node_names: vec!["out".into()],
        branch_names: Vec::new(),
        digital_traces: Vec::new(),
        digital_buses: Vec::new(),
        real_traces: Vec::new(),
        device_op_traces: Vec::new(),
        store_traces: Vec::new(),
        fft_results: Vec::new(),
    };
    let results = rspice_core::analysis::evaluate_tran_measurements(&netlist, &waveform);
    assert_eq!(results[0].value, Some(2.0), "{:?}", results[0]);
    assert_eq!(results[1].value, Some(4.0), "{:?}", results[1]);
}
