use super::*;
use crate::analysis::harmonic_balance::{
    HbConfig, HbSolver, PeriodicNoiseSource, PeriodicSidebandWindow,
};

fn sine_state() -> HbSolverState {
    let mut state = HbSolverState::new(1, 2);
    state.x[0][1] = Complex64::new(0.0, -0.5);
    state
}

fn prepare(request: &PeriodicNoiseSampling) -> Result<PreparedSampling, SimulationError> {
    PreparedSampling::prepare(
        request,
        &sine_state(),
        &["out".into()],
        (0, None),
        1.0,
        10000,
        &NoAbort,
    )
}

fn source() -> PeriodicNoiseSource {
    PeriodicNoiseSource {
        correlated: None,
        name: "modulated noise".into(),
        node_pos: 0,
        node_neg: usize::MAX,
        psd: vec![Complex64::new(2.0, 0.0), Complex64::new(0.2, 0.3)],
        binary_scale_exponent: 0,
        flicker: None,
    }
}

fn density(request: &PeriodicNoiseSampling, offset: f64) -> f64 {
    let mut solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(2), 1);
    solver.add_conductance(0, 0, 1.0);
    let projection = prepare(request).unwrap().projection(offset, 1.0, 1);
    let mut density = None;
    solver
        .solve_periodic_noise_projected_correlations_each(
            &sine_state(),
            PeriodicSidebandWindow {
                offset_hz: offset,
                sideband_min: -1,
                sideband_max: 1,
            },
            &[projection],
            &[source()],
            &NoAbort,
            |_, covariance| {
                density = Some(covariance[0].re);
                Ok(())
            },
        )
        .unwrap();
    density.unwrap()
}

#[test]
fn sampled_pnoise_retains_phase_dependent_sideband_correlations() {
    for phase in [0.0_f64, 90.0, 180.0, -90.0] {
        let actual = density(
            &PeriodicNoiseSampling::Phase {
                phase_degrees: phase,
            },
            0.25,
        );
        // Three diagonal bands plus four adjacent-band cross terms.
        let expected =
            6.0 + 4.0 * (0.2 * phase.to_radians().cos() - 0.3 * phase.to_radians().sin());
        assert!(
            (actual - expected).abs() < 1e-12,
            "phase={phase}: {actual} != {expected}"
        );
    }
}

#[test]
fn sampled_pnoise_resolves_crossings_and_correlated_edge_delay() {
    let rising = PeriodicNoiseEdge::default();
    let output = prepare(&PeriodicNoiseSampling::Edge {
        edge: rising.clone(),
    })
    .unwrap();
    assert_eq!(output.evidence.output.phase_degrees, 0.0);
    assert!((output.evidence.output.slew_volts_per_second - TAU).abs() < 1e-12);
    let falling = PeriodicNoiseEdge {
        direction: PeriodicNoiseEdgeDirection::Falling,
        ..rising.clone()
    };
    let output = prepare(&PeriodicNoiseSampling::Edge { edge: falling }).unwrap();
    assert!((output.evidence.output.phase_degrees - 180.0).abs() < 1e-8);
    assert!((output.evidence.output.slew_volts_per_second + TAU).abs() < 1e-12);
    let edge_density = density(
        &PeriodicNoiseSampling::Edge {
            edge: rising.clone(),
        },
        0.25,
    );
    assert!((edge_density - 6.8 / TAU.powi(2)).abs() < 1e-12);
    for periods in [0, 1, 2] {
        let request = PeriodicNoiseSampling::Delay {
            edge: rising.clone(),
            reference_node: "out".into(),
            reference_ref: None,
            reference_edge: rising.clone(),
            periods,
        };
        let actual = density(&request, 0.25);
        let expected = edge_density
            * 4.0
            * (std::f64::consts::PI * 0.25 * f64::from(periods))
                .sin()
                .powi(2);
        assert!(
            (actual - expected).abs() < 1e-12,
            "periods={periods}: {actual} != {expected}"
        );
        assert_eq!(
            prepare(&request).unwrap().evidence.nominal_delay_seconds,
            Some(f64::from(periods))
        );
    }
}

#[test]
fn sampled_pnoise_delay_preserves_different_probe_slopes_and_fractional_phase() {
    let mut state = HbSolverState::new(2, 2);
    state.x[0][1] = Complex64::new(0.0, -0.5);
    state.x[1][1] = Complex64::new(0.0, -1.0) * Complex64::from_polar(1.0, TAU / 6.0);
    let request = PeriodicNoiseSampling::Delay {
        edge: Default::default(),
        reference_node: "reference".into(),
        reference_ref: None,
        reference_edge: Default::default(),
        periods: 1,
    };
    let prepared = PreparedSampling::prepare(
        &request,
        &state,
        &["out".into(), "reference".into()],
        (0, None),
        1.0,
        10000,
        &NoAbort,
    )
    .unwrap();
    assert!((prepared.evidence.nominal_delay_seconds.unwrap() - 1.0 / 6.0).abs() < 1e-10);
    let mut solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(2), 2);
    // The same source induces Vout=2*I and Vreference=I. Their noise is
    // correlated even though the carrier amplitudes and crossing times differ.
    solver.add_resistor(0, 1, 1.0);
    solver.add_resistor(1, usize::MAX, 1.0);
    let mut noise = source();
    noise.psd.truncate(1);
    let offset = 0.25;
    let mut actual = 0.0;
    solver
        .solve_periodic_noise_projected_correlations_each(
            &state,
            PeriodicSidebandWindow {
                offset_hz: offset,
                sideband_min: -1,
                sideband_max: 1,
            },
            &[prepared.projection(offset, 1.0, 1)],
            &[noise],
            &NoAbort,
            |_, covariance| {
                actual = covariance[0].re;
                Ok(())
            },
        )
        .unwrap();
    let expected = 2.0 / TAU.powi(2) * (12.75 - 4.0 * (TAU * offset / 6.0).cos());
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn sampled_pnoise_crossing_search_resolves_close_pairs_and_refuses_tangencies() {
    let edge = PeriodicNoiseEdge {
        threshold_volts: 1.0 - 1e-8,
        direction: PeriodicNoiseEdgeDirection::Either,
        ..Default::default()
    };
    for occurrence in [1, 2] {
        let selected = PeriodicNoiseEdge {
            occurrence,
            ..edge.clone()
        };
        let output = prepare(&PeriodicNoiseSampling::Edge { edge: selected }).unwrap();
        let phase = output.evidence.output.phase_degrees.to_radians();
        let expected = if occurrence == 1 {
            edge.threshold_volts.asin()
        } else {
            std::f64::consts::PI - edge.threshold_volts.asin()
        };
        assert!((phase - expected).abs() < 2e-9, "{phase} != {expected}");
    }
    for edge in [
        PeriodicNoiseEdge {
            threshold_volts: 1.0,
            ..edge.clone()
        },
        PeriodicNoiseEdge {
            occurrence: 3,
            ..edge.clone()
        },
        PeriodicNoiseEdge {
            minimum_slew_volts_per_second: 100.0,
            ..Default::default()
        },
    ] {
        assert!(prepare(&PeriodicNoiseSampling::Edge { edge }).is_err());
    }
    let request = PeriodicNoiseSampling::Edge {
        edge: Default::default(),
    };
    assert!(matches!(
        PreparedSampling::prepare(
            &request,
            &sine_state(),
            &["out".into()],
            (0, None),
            1.0,
            1,
            &NoAbort
        ),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        PreparedSampling::prepare(
            &request,
            &sine_state(),
            &["out".into()],
            (0, None),
            1.0,
            1000,
            &crate::abort_signal::ImmediateAbort
        ),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn sampled_pnoise_public_hb_request_folds_voltage_and_timing_with_input_referral() {
    let netlist = Netlist::parse(
        "Sampled noise\nI1 0 out SIN(0 1m 1k) AC 1\nR1 out 0 1k\n.options temp=27\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let carrier = engine
        .run_hb_with_abort(&netlist, HbConfig::new(1000.0).with_harmonics(4), &NoAbort)
        .unwrap();
    let voltage = PeriodicNoiseSampling::Phase {
        phase_degrees: 37.0,
    };
    let timing = PeriodicNoiseSampling::Edge {
        edge: Default::default(),
    };
    let thermal = 4.0 * crate::constants::K_BOLTZMANN * 300.15 * 1000.0;
    for sampling in [&voltage, &timing] {
        let request = PeriodicNoiseRequest {
            sampling: Some(sampling),
            offsets: &[100.0, 250.0],
            output_node: "out",
            output_ref: None,
            input_source: Some("I1"),
            max_sideband: 1,
            sidebands: Default::default(),
        };
        let actual = engine
            .run_pnoise_from_hb_request_with_abort(
                &netlist,
                &request,
                &carrier.operating_point,
                &NoAbort,
            )
            .unwrap();
        let expected = thermal * 3.0
            / if sampling.is_timing() {
                (TAU * 1000.0).powi(2)
            } else {
                1.0
            };
        assert_eq!(
            actual.output_spectral_unit(),
            if sampling.is_timing() {
                "s^2/Hz"
            } else {
                "V^2/Hz"
            }
        );
        assert_eq!(actual.sampling.as_ref().unwrap().request, *sampling);
        for value in &actual.output_noise {
            assert!((value / expected - 1.0).abs() < 1e-8, "{actual:?}");
        }
        for value in actual.input_noise.as_ref().unwrap() {
            assert!((value / (thermal * 3.0 / 1e6) - 1.0).abs() < 1e-8);
        }
        let invalid = PeriodicNoiseRequest {
            offsets: &[501.0],
            ..request
        };
        assert!(
            engine
                .run_pnoise_from_hb_request_with_abort(
                    &netlist,
                    &invalid,
                    &carrier.operating_point,
                    &NoAbort
                )
                .is_err()
        );
    }
}
