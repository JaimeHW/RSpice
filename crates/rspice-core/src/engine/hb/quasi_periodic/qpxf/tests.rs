//! Native exact-MNA transfer oracles and source/tuple request boundaries.
use super::*;
use crate::abort_signal::CountingAbort;
use crate::analysis::quasi_periodic::{QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod};

fn request() -> QpxfRequest {
    QpxfRequest {
        frequencies_hz: vec![-37.0, 0.0, 127.0],
        frequency_axis: QpxfFrequencyAxis::Output,
        input_sources: QpxfSources::AllIndependent,
        input_lattices: QpxfInputLattices::Explicit(vec![vec![0, 0, 0], vec![1, -1, 0]]),
        output: QpxfOutput::BranchCurrent {
            branch: "L1".into(),
        },
        output_lattice: vec![0, 0, 0],
        linear: QuasiPeriodicLinearConfig::default(),
        group_delay: false,
        group_delay_magnitude_floor: 0.0,
    }
}

#[test]
fn qpxf_engine_all_sources_and_current_output_match_rlc_above_dense_limit() {
    let deck = "QPXF exact MNA\nVprobe in ref SIN(1 .1 1k)\nVref ref 0 SIN(.1 .1 1414.2135623730951)\nIprobe ref mid DC 0\nR1 in mid 1k\nC1 mid ref 1u\nR2 mid out 2k\nL1 out ref 1m\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(Default::default());
    let point = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(
                vec![
                    1000.0,
                    std::f64::consts::SQRT_2 * 1000.0,
                    std::f64::consts::PI * 1000.0,
                ],
                vec![2, 2, 2],
            ),
        )
        .unwrap();
    let identity = point.retained_identity().to_owned();
    assert!(point.spectra().len() * 125 > 512);
    let result = engine
        .run_qpxf_from_qpss(&netlist, request(), &point)
        .unwrap();
    assert_eq!(result.operating_point_identity, identity);
    assert_eq!(result.solutions.len(), 3);
    assert_eq!(result.input_sources.len(), 3);
    assert_eq!(result.transfers.len(), 6);
    assert_eq!(result.output_frequencies_hz, [-37.0, 0.0, 127.0]);
    assert!(
        result
            .solutions
            .iter()
            .all(|s| s.normalized_residual <= 1.0)
    );
    for transfer in &result.transfers {
        let source = &result.input_sources[transfer.input_source];
        assert!(transfer.group_delay.is_none());
        for (i, value) in transfer.values.iter().enumerate() {
            let jw = Complex64::new(0.0, std::f64::consts::TAU * result.output_frequencies_hz[i]);
            let z2 = Complex64::new(2000.0, 0.0) + jw * 0.001;
            let ymid = Complex64::new(0.001, 0.0) + jw * 1e-6 + Complex64::ONE / z2;
            let expected = if transfer.input_lattice != [0, 0, 0]
                || source.name.eq_ignore_ascii_case("Vref")
            {
                Complex64::ZERO
            } else if source.quantity == QpxfQuantity::Voltage {
                Complex64::new(0.001, 0.0) / ymid / z2
            } else {
                Complex64::ONE / ymid / z2
            };
            assert!(
                (*value - expected).norm() < 1e-10,
                "{} {:?} at {}: {value:?} != {expected:?}",
                source.name,
                transfer.input_lattice,
                result.output_frequencies_hz[i]
            );
        }
    }
    // A different observable consumes the same immutable operating point.
    let mut voltage = request();
    voltage.output = QpxfOutput::Voltage {
        positive: "mid".into(),
        negative: "ref".into(),
    };
    voltage.input_sources = QpxfSources::Named(vec!["vprobe".into()]);
    voltage.input_lattices = QpxfInputLattices::MaxOrders(vec![0, 0, 0]);
    voltage.group_delay = true;
    let result = engine
        .run_qpxf_from_qpss(&netlist, voltage, &point)
        .unwrap();
    assert_eq!(result.input_sources[0].name, "VPROBE");
    assert_eq!(result.transfers.len(), 1);
    assert!(
        result.transfers[0]
            .group_delay
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| matches!(v, QpxfGroupDelay::Finite(_)))
    );
    assert_eq!(point.retained_identity(), identity);
    let mut direct = request();
    direct.linear.method = QuasiPeriodicLinearMethod::Direct;
    assert!(engine.run_qpxf_from_qpss(&netlist, direct, &point).is_err());
    assert!(
        engine
            .run_qpxf_from_qpss(
                &Netlist::parse(&deck.replace("R1 in mid 1k", "R1 in mid 2k")).unwrap(),
                request(),
                &point
            )
            .is_err()
    );
    assert!(matches!(
        engine.run_qpxf_from_qpss_with_abort(&netlist, request(), &point, &CountingAbort::new(2)),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn qpxf_engine_output_frequency_anchor_preserves_small_signal_and_full_input_lattice() {
    // The output is below the carrier's floating-point spacing. Subtracting
    // the carrier and adding it back would erase the requested frequency.
    let netlist = Netlist::parse(
        "QPXF output anchor\nV1 in 0 DC 1\nI1 0 out DC 0\nR1 in out 1k\nC1 out 0 1e-18\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(Default::default());
    let point = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(vec![1e18, std::f64::consts::SQRT_2 * 1e18], vec![1, 1]),
        )
        .unwrap();
    let mut req = request();
    req.frequencies_hz = vec![0.03125, 0.0625, 0.125];
    req.output_lattice = vec![1, 0];
    req.output = QpxfOutput::Voltage {
        positive: "out".into(),
        negative: "0".into(),
    };
    req.input_lattices = QpxfInputLattices::AllRetained;
    // Other translated input frequencies cannot resolve this tiny step on a
    // 1e18 Hz carrier. Keep those paths valid and retain their rounded physical
    // coordinates; the independently authored OUTPUT grid is the sweep axis.
    let result = engine
        .run_qpxf_from_qpss(&netlist, req.clone(), &point)
        .unwrap();
    assert_eq!(result.input_lattices.len(), 9);
    assert_eq!(result.transfers.len(), 18);
    assert_eq!(result.output_frequencies_hz, req.frequencies_hz);
    for transfer in &result.transfers {
        let source = &result.input_sources[transfer.input_source];
        for (i, value) in transfer.values.iter().enumerate() {
            let frequency = req.frequencies_hz[i];
            let expected = if transfer.input_lattice == req.output_lattice {
                let gain = if source.quantity == QpxfQuantity::Voltage {
                    1.0
                } else {
                    1000.0
                };
                Complex64::new(gain, 0.0)
                    / Complex64::new(1.0, std::f64::consts::TAU * frequency * 1e-15)
            } else {
                Complex64::ZERO
            };
            assert!((*value - expected).norm() < 1e-8);
            if transfer.input_lattice == req.output_lattice {
                assert_eq!(transfer.input_frequencies_hz[i], frequency);
            }
        }
    }
    let mut offset = req.clone();
    offset.frequency_axis = QpxfFrequencyAxis::Offset;
    assert!(
        engine.run_qpxf_from_qpss(&netlist, offset, &point).is_err(),
        "a physical output grid that collapses must be refused"
    );
    let mut bad = req.clone();
    bad.input_sources = QpxfSources::Named(vec!["V1".into(), "v1".into()]);
    assert!(bad.validate().is_err());
    let mut bad = req.clone();
    bad.input_sources = QpxfSources::Named(vec!["missing".into()]);
    assert!(engine.run_qpxf_from_qpss(&netlist, bad, &point).is_err());
    let mut bad = req.clone();
    bad.input_lattices = QpxfInputLattices::Explicit(vec![vec![2, 0]]);
    assert!(engine.run_qpxf_from_qpss(&netlist, bad, &point).is_err());
    let mut bad = req.clone();
    bad.input_lattices = QpxfInputLattices::MaxOrders(vec![2, 1]);
    assert!(engine.run_qpxf_from_qpss(&netlist, bad, &point).is_err());
    let mut bad = req.clone();
    bad.output = QpxfOutput::BranchCurrent {
        branch: "R1".into(),
    };
    assert!(engine.run_qpxf_from_qpss(&netlist, bad, &point).is_err());
    let mut config = engine.config.clone();
    config.resource_limits.max_result_values = 100;
    assert!(matches!(
        Engine::new(config).run_qpxf_from_qpss(&netlist, req, &point),
        Err(SimulationError::ResourceLimit(_))
    ));
}
