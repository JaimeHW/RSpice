use super::*;
use crate::abort_signal::CountingAbort;

fn observe(node: Option<usize>, reference: Option<usize>, sideband: i32) -> PeriodicNoiseOutput {
    PeriodicNoiseOutput {
        node_pos: node,
        node_neg: reference,
        sideband,
    }
}

fn covariance(
    solver: &mut HbSolver,
    window: PeriodicSidebandWindow,
    outputs: &[PeriodicNoiseOutput],
    sources: &[PeriodicNoiseSource],
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<Complex64>>, HbError> {
    let state = HbSolverState::new(solver.num_nodes, solver.num_harmonics);
    let mut values = Vec::new();
    solver.solve_periodic_noise_correlations_each(
        &state,
        window,
        outputs,
        sources,
        abort,
        |index, matrix| {
            assert_eq!(index, values.len());
            values.push(matrix.to_vec());
            Ok(())
        },
    )?;
    Ok(values)
}

fn source() -> PeriodicNoiseSource {
    PeriodicNoiseSource {
        name: "Idevice".into(),
        node_pos: 0,
        node_neg: usize::MAX,
        psd: vec![Complex64::new(2.0, 0.0), Complex64::new(0.2, 0.3)],
        binary_scale_exponent: 0,
        flicker: None,
    }
}

#[test]
fn periodic_noise_covariance_matches_analytic_rc_port_and_sideband_correlations() {
    let window = PeriodicSidebandWindow {
        offset_hz: 0.25,
        sideband_min: -1,
        sideband_max: 1,
    };
    let outputs = [
        observe(Some(0), None, -1),
        observe(Some(1), None, 0),
        observe(Some(0), Some(1), 1),
        observe(None, Some(0), -1),
        observe(Some(0), Some(0), 0),
    ];
    for iterative in [false, true] {
        let mut config = HbConfig::new(1.0).with_harmonics(2);
        config.use_krylov = iterative;
        let mut solver = HbSolver::new(config, 2);
        // Y = [[3+jwC1, -1], [-1, 4+jwC2]]. No frequency conversion
        // in the circuit: all cross-band covariance comes from the source.
        solver.add_resistor(0, usize::MAX, 0.5);
        solver.add_resistor(1, usize::MAX, 1.0 / 3.0);
        solver.add_resistor(0, 1, 1.0);
        solver.add_capacitance(0, 0, 0.1);
        solver.add_capacitance(1, 1, 0.2);
        let noise = source();
        let actual = covariance(
            &mut solver,
            window,
            &outputs,
            std::slice::from_ref(&noise),
            &NoAbort,
        )
        .unwrap();
        let transfer = |output: &PeriodicNoiseOutput| {
            let omega = 2.0 * PI * (window.offset_hz + output.sideband as Value);
            let a = Complex64::new(3.0, omega * 0.1);
            let d = Complex64::new(4.0, omega * 0.2);
            let determinant = a * d - Complex64::new(1.0, 0.0);
            let voltage = |node| match node {
                Some(0) => d / determinant,
                Some(1) => Complex64::new(1.0, 0.0) / determinant,
                _ => Complex64::ZERO,
            };
            voltage(output.node_pos) - voltage(output.node_neg)
        };
        for (row, left) in outputs.iter().enumerate() {
            for (column, right) in outputs.iter().enumerate() {
                let difference = left.sideband - right.sideband;
                let intensity = noise
                    .psd
                    .get(difference.unsigned_abs() as usize)
                    .copied()
                    .unwrap_or(Complex64::ZERO);
                let intensity = if difference < 0 {
                    intensity.conj()
                } else {
                    intensity
                };
                let expected = transfer(left) * transfer(right).conj() * intensity;
                let observed = actual[0][row * outputs.len() + column];
                assert!(
                    (observed - expected).norm() < 2e-13,
                    "iterative={iterative}, ({row},{column}): {observed} != {expected}"
                );
                assert_eq!(observed, actual[0][column * outputs.len() + row].conj());
            }
        }
        assert!(
            actual[0][1].im.abs() > 1e-4,
            "cross-spectrum must retain phase"
        );
    }
}

#[test]
fn periodic_noise_covariance_retains_signed_flicker_modulation_and_outside_window_sources() {
    let mut solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(2), 1);
    solver.add_conductance(0, 0, 2.0);
    let window = PeriodicSidebandWindow {
        offset_hz: 0.25,
        sideband_min: -1,
        sideband_max: 1,
    };
    let outputs = [-1, 0, 1].map(|sideband| observe(Some(0), None, sideband));
    let modulation = [Complex64::new(-0.5, 0.0), Complex64::new(0.2, -0.4)];
    let noise = PeriodicNoiseSource {
        psd: vec![Complex64::ZERO],
        flicker: Some(PeriodicFlickerNoise {
            coefficient: 3.0,
            exponent: 1.3,
            modulation: modulation.to_vec(),
        }),
        ..source()
    };
    let actual = covariance(&mut solver, window, &outputs, &[noise], &NoAbort).unwrap();
    let amplitude = |harmonic: i32| {
        let value = modulation
            .get(harmonic.unsigned_abs() as usize)
            .copied()
            .unwrap_or(Complex64::ZERO);
        if harmonic < 0 { value.conj() } else { value }
    };
    for (r, left) in outputs.iter().enumerate() {
        for (c, right) in outputs.iter().enumerate() {
            // Independent stationary noise at m=-2 and +2 must contribute
            // although the circuit conversion window ends at -1 and +1.
            let expected: Complex64 = (-2..=2)
                .map(|m| {
                    let density = 3.0 / (0.25 + m as Value).abs().powf(1.3);
                    amplitude(left.sideband - m) * amplitude(right.sideband - m).conj() * density
                        / 4.0
                })
                .sum();
            assert!((actual[0][r * 3 + c] - expected).norm() < 2e-13);
        }
    }
    assert!(actual[0][1].im.abs() > 0.1);
}

#[test]
fn periodic_noise_covariance_preserves_extreme_colored_cross_products() {
    let left = ScaledComplex {
        mantissa: Complex64::new(1.0, 0.5),
        exponent: 1500,
    };
    let right = ScaledComplex {
        mantissa: Complex64::new(-0.5, 1.0),
        exponent: -500,
    };
    let term =
        scaled_flicker_cross_term(left, right, 1.0, -1200, libm::scalbn(1.0, -600), 2.0).unwrap();
    let expected = Complex64::new(0.0, -1.25);
    assert_eq!(term.exponent, 1000);
    assert_eq!(term.mantissa, expected);
    let reverse =
        scaled_flicker_cross_term(right, left, 1.0, -1200, libm::scalbn(1.0, -600), 2.0).unwrap();
    assert_eq!(reverse.mantissa, term.mantissa.conj());
    assert_eq!(reverse.exponent, term.exponent);
    // The frequency law overflows alone, but the complete covariance is 1-2j.
    let term = scaled_flicker_cross_term(
        ScaledComplex {
            mantissa: Complex64::new(1.0, 0.0),
            exponent: 700,
        },
        ScaledComplex {
            mantissa: Complex64::new(0.5, 1.0),
            exponent: 501,
        },
        1.0,
        0,
        libm::scalbn(1.0, 600),
        2.0,
    )
    .unwrap();
    let (actual, _) = materialize_scaled_complex_sum(&[term]).unwrap();
    assert_eq!(actual, Complex64::new(1.0, -2.0));
}

#[test]
fn periodic_noise_covariance_rejects_impossible_correlations_and_bad_outputs() {
    let mut solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(2), 1);
    solver.add_conductance(0, 0, 1.0);
    let window = PeriodicSidebandWindow {
        offset_hz: 0.25,
        sideband_min: 0,
        sideband_max: 1,
    };
    let outputs = [observe(Some(0), None, 0), observe(Some(0), None, 1)];
    let noise = PeriodicNoiseSource {
        psd: vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0)],
        ..source()
    };
    let error = covariance(&mut solver, window, &outputs, &[noise], &NoAbort).unwrap_err();
    assert!(
        error.to_string().contains("diagonal noise-power bound"),
        "{error}"
    );
    for outputs in [
        vec![],
        vec![observe(Some(1), None, 0)],
        vec![observe(None, Some(1), 0)],
        vec![observe(Some(0), None, 2)],
    ] {
        assert!(covariance(&mut solver, window, &outputs, &[source()], &NoAbort).is_err());
    }
}

#[test]
fn periodic_noise_covariance_cancels_before_and_between_sources_and_propagates_consumer_error() {
    let mut solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(1), 1);
    solver.add_conductance(0, 0, 1.0);
    let state = HbSolverState::new(1, 1);
    let window = PeriodicSidebandWindow {
        offset_hz: 0.25,
        sideband_min: 0,
        sideband_max: 0,
    };
    let outputs = [observe(Some(0), None, 0)];
    let sources = [source(), source()];
    let count = CountingAbort::new(usize::MAX);
    covariance(&mut solver, window, &outputs, &sources, &count).unwrap();
    for threshold in 0..count.count() {
        let abort = CountingAbort::new(threshold);
        assert!(matches!(
            covariance(&mut solver, window, &outputs, &sources, &abort),
            Err(HbError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }
    let mut calls = 0;
    let error = solver
        .solve_periodic_noise_correlations_each(
            &state,
            window,
            &outputs,
            &sources,
            &NoAbort,
            |_, _| {
                calls += 1;
                Err(HbError::Aborted)
            },
        )
        .unwrap_err();
    assert!(matches!(error, HbError::Aborted));
    assert_eq!(calls, 1);
}
