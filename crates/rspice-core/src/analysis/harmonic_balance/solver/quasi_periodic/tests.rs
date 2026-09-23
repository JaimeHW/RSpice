//! Physical circuit oracles for the independent-tone Newton path.
use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::analysis::quasi_periodic::{QuasiPeriodicGridConfig, QuasiPeriodicSampling};

fn grid(harmonics: usize) -> Arc<QuasiPeriodicGrid> {
    let mut config = QuasiPeriodicGridConfig::new(
        vec![1e3, std::f64::consts::SQRT_2 * 1e3],
        vec![harmonics; 2],
    );
    config.sampling = QuasiPeriodicSampling::Exact(vec![32; 2]);
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    )
}

fn cosine(
    grid: &QuasiPeriodicGrid,
    spectrum: &mut [Complex64],
    tuple: &[i32],
    amplitude: Value,
    phase: Value,
) {
    let k = grid.index_of(tuple).unwrap();
    spectrum[k] = Complex64::from_polar(amplitude * 0.5, phase);
    spectrum[grid.len() - 1 - k] = spectrum[k].conj();
}

fn close(actual: Complex64, expected: Complex64, tolerance: Value) {
    assert!(
        (actual - expected).norm() <= tolerance,
        "actual={actual:?}, expected={expected:?}, tolerance={tolerance:e}"
    );
}

fn config() -> QuasiPeriodicSolveConfig {
    QuasiPeriodicSolveConfig {
        relative_tolerance: 1e-9,
        current_absolute_tolerance: 1e-14,
        voltage_absolute_tolerance: 1e-12,
        ..Default::default()
    }
}

#[test]
fn qpac_mna_matches_shifted_rlc_and_ac_resistance_overrides_beyond_dense_limit() {
    for harmonics in [1, 7] {
        let grid = grid(harmonics);
        let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 1);
        solver.add_conductance_with_small_signal(0, 0, 1e-3, 0.02);
        solver.add_capacitance(0, 0, 1e-6);
        solver
            .try_add_periodic_inductor_branch(1, 0, 2e-3, 1, "L1")
            .unwrap();
        solver
            .try_add_periodic_resistor_branch(1, 0, 500.0, 75.0, 2, "Rbranch")
            .unwrap();
        let orbit = vec![vec![Complex64::ZERO; grid.len()]; 3];
        let mut sources = orbit.clone();
        for (k, value) in sources[0].iter_mut().enumerate() {
            *value = Complex64::new(0.003, k as Value * 1e-5);
        }
        let settings = QuasiPeriodicAcConfig::default();
        let results = solver
            .solve_quasi_periodic_ac_with_abort(
                grid.clone(),
                &settings,
                &orbit,
                &[1e3],
                &sources,
                &ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap();
        let result = &results[0];
        for (k, &f) in grid.frequencies_hz().iter().enumerate() {
            let frequency = 1e3 + f;
            if frequency == 0.0 {
                close(result.spectra[0][k], Complex64::ZERO, 1e-12);
                close(result.spectra[1][k], sources[0][k], 1e-12);
                close(result.spectra[2][k], Complex64::ZERO, 1e-12);
            } else {
                let omega = std::f64::consts::TAU * frequency;
                let yl = Complex64::new(0.0, -1.0 / (omega * 2e-3));
                let voltage =
                    sources[0][k] / (Complex64::new(0.02 + 1.0 / 75.0, omega * 1e-6) + yl);
                close(result.spectra[0][k], voltage, 1e-10);
                close(result.spectra[1][k], voltage * yl, 1e-12);
                close(result.spectra[2][k], voltage / 75.0, 1e-12);
            }
        }
        assert!(result.normalized_residual <= 1.0);
    }
}

#[test]
fn quasi_periodic_rlc_uses_exact_dc_branches_and_signed_mixing_frequencies() {
    let grid = grid(1);
    // An unrelated HB basis deliberately cannot represent either QP tone.
    let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 1);
    solver.add_conductance_with_small_signal(0, 0, 1e-3, 0.02);
    solver.add_capacitance(0, 0, 1e-6);
    solver
        .try_add_periodic_inductor_branch(1, 0, 2e-3, 1, "L1")
        .unwrap();
    solver
        .try_add_periodic_resistor_branch(1, 0, 500.0, 75.0, 2, "Rbranch")
        .unwrap();
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; 3];
    sources[0][grid.dc_index()] = Complex64::new(0.01, 0.0);
    cosine(&grid, &mut sources[0], &[1, 0], 0.003, 0.4);
    cosine(&grid, &mut sources[0], &[0, 1], 0.002, -0.7);
    cosine(&grid, &mut sources[0], &[1, -1], 0.001, 1.1);
    let result = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    assert!(result.normalized_residual() <= 1.0);
    assert_eq!(result.iterations(), 1);
    for (k, &frequency) in grid.frequencies_hz().iter().enumerate() {
        if frequency == 0.0 {
            close(result.spectra()[0][k], Complex64::ZERO, 1e-14);
            close(result.spectra()[1][k], sources[0][k], 1e-14);
            close(result.spectra()[2][k], Complex64::ZERO, 1e-14);
        } else {
            let omega = std::f64::consts::TAU * frequency;
            let yl = Complex64::new(0.0, -1.0 / (omega * 2e-3));
            let y = Complex64::new(1e-3 + 1.0 / 500.0, omega * 1e-6) + yl;
            let voltage = sources[0][k] / y;
            close(result.spectra()[0][k], voltage, 1e-12);
            close(result.spectra()[1][k], voltage * yl, 1e-14);
            close(result.spectra()[2][k], voltage / 500.0, 1e-14);
        }
    }
}

#[test]
fn qpac_native_diode_conversion_matches_independent_charge_quadrature() {
    let grid = grid(2);
    let mut solver = HbSolver::new(HbConfig::new(9.0), 1);
    solver
        .try_add_periodic_voltage_source_branch(1, 0, 0, 1, "Vdrive")
        .unwrap();
    let saturation = 2e-6;
    let thermal = 0.025;
    let transit = 8e-5;
    solver.add_nonlinear_device(
        NonlinearDeviceInstance::diode(0, 1, saturation, 1.0)
            .with_thermal_voltage(thermal)
            .with_junction_caps(DepletionCap::none(), DepletionCap::none(), transit),
    );
    let mut large = vec![vec![Complex64::ZERO; grid.len()]; 2];
    large[1][grid.dc_index()] = Complex64::new(0.07, 0.0);
    cosine(&grid, &mut large[1], &[1, 0], 0.016, 0.3);
    cosine(&grid, &mut large[1], &[0, 1], 0.011, -0.8);
    let point = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &large,
            None,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    let mut probe = vec![vec![Complex64::ZERO; grid.len()]; 2];
    let input = grid.index_of(&[2, 0]).unwrap();
    let amplitude = Complex64::new(0.3, -0.7);
    probe[1][input] = amplitude;
    let results = solver
        .solve_quasi_periodic_ac_with_abort(
            grid.clone(),
            &QuasiPeriodicAcConfig::default(),
            point.spectra(),
            &[137.0],
            &probe,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    for (k, tuple) in grid.indices().iter().enumerate() {
        let mut derivative = Complex64::ZERO;
        for a in 0..64 {
            for b in 0..64 {
                let p = std::f64::consts::TAU * a as Value / 64.0;
                let q = std::f64::consts::TAU * b as Value / 64.0;
                let voltage = 0.07 + 0.016 * (p + 0.3).cos() + 0.011 * (q - 0.8).cos();
                let conductance = saturation / thermal * (voltage / thermal).exp();
                derivative += Complex64::from_polar(
                    conductance / 4096.0,
                    -((tuple[0] - 2) as Value * p + tuple[1] as Value * q),
                );
            }
        }
        let omega = std::f64::consts::TAU * (137.0 + grid.frequencies_hz()[k]);
        close(
            results[0].spectra[0][k],
            if k == input {
                amplitude
            } else {
                Complex64::ZERO
            },
            1e-11,
        );
        close(
            results[0].spectra[1][k],
            -amplitude * Complex64::new(1.0, omega * transit) * derivative,
            1e-12,
        );
    }
    // This conversion uses G[-4,0], beyond the retained orbit's H=2.
    assert!(results[0].spectra[1][grid.index_of(&[-2, 0]).unwrap()].norm() > 1e-7);
}

#[test]
fn quasi_periodic_diode_diffusion_current_matches_independent_two_phase_quadrature() {
    let grid = grid(2);
    let mut solver = HbSolver::new(HbConfig::new(9.0), 1);
    solver
        .try_add_periodic_voltage_source_branch(1, 0, 0, 1, "Vdrive")
        .unwrap();
    let saturation = 2e-6;
    let thermal = 0.025;
    let transit = 8e-5;
    solver.add_nonlinear_device(
        NonlinearDeviceInstance::diode(0, 1, saturation, 1.0)
            .with_thermal_voltage(thermal)
            .with_junction_caps(DepletionCap::none(), DepletionCap::none(), transit),
    );
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
    sources[1][grid.dc_index()] = Complex64::new(0.07, 0.0);
    cosine(&grid, &mut sources[1], &[1, 0], 0.016, 0.3);
    cosine(&grid, &mut sources[1], &[0, 1], 0.011, -0.8);
    let result = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    assert!(result.iterations() >= 2);
    for (k, tuple) in grid.indices().iter().enumerate() {
        // Independent direct quadrature uses twice the solver's phase count,
        // explicit authored voltages, and the analytic diode equation.
        let mut current = Complex64::ZERO;
        for a in 0..64 {
            for b in 0..64 {
                let p = std::f64::consts::TAU * a as Value / 64.0;
                let q = std::f64::consts::TAU * b as Value / 64.0;
                let v = 0.07 + 0.016 * (p + 0.3).cos() + 0.011 * (q - 0.8).cos();
                let id = saturation * (v / thermal).exp_m1();
                current += Complex64::from_polar(
                    id / 4096.0,
                    -(tuple[0] as Value * p + tuple[1] as Value * q),
                );
            }
        }
        let jw = Complex64::new(0.0, std::f64::consts::TAU * grid.frequencies_hz()[k]);
        close(result.spectra()[0][k], sources[1][k], 1e-12);
        close(
            result.spectra()[1][k],
            -(Complex64::new(1.0, 0.0) + jw * transit) * current,
            2e-13,
        );
    }
    for tuple in [[1, 1], [1, -1], [2, 0]] {
        assert!(result.spectra()[1][grid.index_of(&tuple).unwrap()].norm() > 1e-7);
    }
}

#[test]
fn quasi_periodic_nonlinear_voltage_matches_independent_pointwise_circuit_solution() {
    let grid = grid(2);
    let mut solver = HbSolver::new(HbConfig::new(17.0), 1);
    solver.add_conductance(0, 0, 1e-3);
    solver.add_nonlinear_device(
        NonlinearDeviceInstance::diode(0, 1, 1e-6, 1.0).with_thermal_voltage(0.025),
    );
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]];
    sources[0][grid.dc_index()] = Complex64::new(2e-5, 0.0);
    cosine(&grid, &mut sources[0], &[1, 0], 1e-6, 0.4);
    cosine(&grid, &mut sources[0], &[0, 1], 2e-6, -0.6);
    let result = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    assert!(result.iterations() >= 2);
    for (k, tuple) in grid.indices().iter().enumerate() {
        let mut expected = Complex64::ZERO;
        for a in 0..64 {
            for b in 0..64 {
                let p = std::f64::consts::TAU * a as Value / 64.0;
                let q = std::f64::consts::TAU * b as Value / 64.0;
                let source = 2e-5 + 1e-6 * (p + 0.4).cos() + 2e-6 * (q - 0.6).cos();
                let mut voltage: Value = 0.018;
                for _ in 0..8 {
                    let e = (voltage / 0.025).exp();
                    voltage -=
                        (voltage * 1e-3 + 1e-6 * (e - 1.0) - source) / (1e-3 + 1e-6 * e / 0.025);
                }
                expected += Complex64::from_polar(
                    voltage / 4096.0,
                    -(tuple[0] as Value * p + tuple[1] as Value * q),
                );
            }
        }
        close(result.spectra()[0][k], expected, 2e-9);
    }
    assert!(result.spectra()[0][grid.index_of(&[1, 1]).unwrap()].norm() > 1e-7);
}

#[test]
fn quasi_periodic_solver_refuses_singular_malformed_unbounded_and_cancelled_work() {
    let grid = grid(1);
    let mut solver = HbSolver::new(HbConfig::new(1.0), 1);
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]];
    let limits = ResourceLimits::default();
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &limits,
            &NoAbort
        ),
        Err(Error::LinearSolve(_))
    ));
    solver.add_conductance(0, 0, 1e-3);
    let restricted = ResourceLimits {
        max_matrix_unknowns: 8,
        ..limits
    };
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &restricted,
            &NoAbort
        ),
        Err(Error::ResourceLimit(_))
    ));
    let restricted = ResourceLimits {
        max_result_values: 100,
        ..limits
    };
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &restricted,
            &NoAbort
        ),
        Err(Error::ResourceLimit(_))
    ));
    let cancel = CountingAbort::new(60);
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &limits,
            &cancel
        ),
        Err(Error::Aborted)
    ));
    assert_eq!(cancel.polls_after_abort(), 0);
    sources[0][grid.dc_index()] = Complex64::new(1e12, 0.0);
    sources[0][grid.index_of(&[1, 0]).unwrap()] = Complex64::new(1e-9, 0.0);
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(
            grid.clone(),
            &config(),
            &sources,
            None,
            &limits,
            &NoAbort
        ),
        Err(Error::InvalidConfig(_))
    ));
    let bad = QuasiPeriodicSolveConfig {
        relative_tolerance: Value::NAN,
        ..config()
    };
    assert!(matches!(
        solver.solve_quasi_periodic_with_abort(grid, &bad, &sources, None, &limits, &NoAbort),
        Err(Error::InvalidConfig(_))
    ));
}

#[test]
fn qpnoise_mna_streams_correlated_voltage_branch_current_and_sideband_outputs() {
    use crate::analysis::quasi_periodic::{QuasiPeriodicLinearMethod, QuasiPeriodicNoiseSpectrum};
    let grid = grid(1);
    for method in [
        QuasiPeriodicLinearMethod::Direct,
        QuasiPeriodicLinearMethod::Krylov,
    ] {
        let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 1);
        solver.add_conductance(0, 0, 0.001);
        solver.add_capacitance(0, 0, 2e-6);
        solver
            .try_add_periodic_resistor_branch(1, 0, 1000.0, 500.0, 1, "R1")
            .unwrap();
        let orbit = vec![vec![Complex64::ZERO; grid.len()]; 2];
        let mut observations = vec![orbit.clone(); 3];
        observations[0][0][grid.dc_index()] = Complex64::ONE;
        observations[1][1][grid.dc_index()] = Complex64::new(0.0, 1.0); // y=-j I(R1)
        observations[2][0][grid.index_of(&[1, 0]).unwrap()] = Complex64::ONE;
        let sources: Vec<_> = [(0, 2e-20), (1, 3e-18)]
            .into_iter()
            .map(|(row, q)| QuasiPeriodicNoiseSource {
                name: format!("source{row}"),
                injections: vec![(row, Complex64::ONE)],
                spectrum: QuasiPeriodicNoiseSpectrum::White {
                    density: vec![q],
                    binary_scale_exponent: 0,
                },
            })
            .collect();
        let mut config = QuasiPeriodicNoiseConfig {
            frequencies_hz: vec![100.0, 500.0],
            frequency_lattice: vec![0, 0],
            input_lattices: grid.indices().to_vec(),
            linear: QuasiPeriodicLinearConfig {
                method,
                ..Default::default()
            },
        };
        for dc_only in [false, true] {
            if dc_only {
                config.input_lattices = vec![vec![0, 0]];
            }
            let mut delivered = 0;
            solver
                .visit_quasi_periodic_noise_with_abort(
                    grid.clone(),
                    &config,
                    &orbit,
                    &observations,
                    &sources,
                    &ResourceLimits::default(),
                    &NoAbort,
                    |index, point| {
                        assert_eq!(index, delivered);
                        delivered += 1;
                        assert_eq!(point.source_covariances.len(), 2);
                        assert_eq!(point.adjoints.len(), 3);
                        let f = config.frequencies_hz[index];
                        for adjoint in &point.adjoints {
                            assert!(adjoint.normalized_residual <= 1.0);
                            assert_eq!(adjoint.frequency_hz, f);
                        }
                        for (source, q) in [2e-20, 3e-18].into_iter().enumerate() {
                            let z = Complex64::ONE
                                / Complex64::new(
                                    0.001 + 1.0 / 500.0,
                                    std::f64::consts::TAU * f * 2e-6,
                                );
                            let high_z = Complex64::ONE
                                / Complex64::new(
                                    0.001 + 1.0 / 500.0,
                                    std::f64::consts::TAU * (f + 1000.0) * 2e-6,
                                );
                            let gains = if source == 0 {
                                [z, Complex64::new(0.0, -1.0) * z / 500.0, high_z]
                            } else {
                                [
                                    z / 500.0,
                                    Complex64::new(0.0, -1.0) * (z / 500.0 - Complex64::ONE)
                                        / 500.0,
                                    high_z / 500.0,
                                ]
                            };
                            let actual = &point.source_covariances[source];
                            assert_eq!(actual.outputs, 3);
                            for r in 0..3 {
                                for c in 0..3 {
                                    let expected = if (r == 2) != (c == 2) || (dc_only && r == 2) {
                                        Complex64::ZERO
                                    } else {
                                        gains[r] * gains[c].conj() * q
                                    };
                                    close(
                                        actual.values[r * 3 + c],
                                        expected,
                                        expected.norm() * 3e-9 + 1e-35,
                                    );
                                }
                            }
                        }
                        Ok(())
                    },
                )
                .unwrap();
            assert_eq!(delivered, 2);
        }
    }
}

#[test]
fn qpnoise_stream_propagates_consumer_errors_and_preflights_combined_workspaces() {
    let grid = grid(1);
    let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 1);
    solver.add_conductance(0, 0, 1.0);
    let orbit = vec![vec![Complex64::ZERO; grid.len()]];
    let mut observation = orbit.clone();
    observation[0][grid.dc_index()] = Complex64::ONE;
    let config = QuasiPeriodicNoiseConfig {
        frequencies_hz: vec![1.0, 2.0],
        frequency_lattice: vec![0, 0],
        input_lattices: grid.indices().to_vec(),
        linear: Default::default(),
    };
    let mut calls = 0;
    let result = solver.visit_quasi_periodic_noise_with_abort(
        grid.clone(),
        &config,
        &orbit,
        &[observation.clone()],
        &[],
        &ResourceLimits::default(),
        &NoAbort,
        |index, point| {
            calls += 1;
            assert_eq!(index, 0);
            assert!(point.source_covariances.is_empty());
            Err(Error::InvalidConfig("consumer declined".into()))
        },
    );
    assert!(
        matches!(result,Err(Error::InvalidConfig(ref message)) if message=="consumer declined")
    );
    assert_eq!(calls, 1);
    let abort = CountingAbort::new(5);
    let result = solver.visit_quasi_periodic_noise_with_abort(
        grid.clone(),
        &config,
        &orbit,
        &[observation.clone()],
        &[],
        &ResourceLimits::default(),
        &abort,
        |_, _| panic!("must abort before delivery"),
    );
    assert!(matches!(result, Err(Error::Aborted)));
    assert_eq!(abort.count(), 6);
    // Enough room for the projector alone, but not its simultaneously retained
    // adjoints and circuit linearization. Charge the complete live workspace.
    let limits = ResourceLimits {
        max_result_values: grid.sample_count() * 10 + 200,
        ..ResourceLimits::default()
    };
    let result = solver.visit_quasi_periodic_noise_with_abort(
        grid,
        &config,
        &orbit,
        &[observation],
        &[],
        &limits,
        &NoAbort,
        |_, _| panic!("must refuse before delivery"),
    );
    assert!(matches!(result, Err(Error::ResourceLimit(_))));
}

#[test]
fn qpnoise_compact_sources_follow_independent_phases_temperature_and_ground() {
    use crate::analysis::{
        noise::NoisePhysicalConstants, quasi_periodic::QuasiPeriodicNoiseSpectrum,
    };
    let grid = grid(1);
    let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 2);
    solver.add_named_nonlinear_device_with_noise_temperature_offset(
        "M1",
        NonlinearDeviceInstance::nmos(0, 1, 2, 2, 0.7, 2e-5, 0.0)
            .with_channel_noise_gamma(2.0 / 3.0),
        50.0,
    );
    solver
        .try_add_periodic_resistor_branch(1, 0, 10.0, 10.0, 1, "R1")
        .unwrap();
    let mut orbit = vec![vec![Complex64::ZERO; grid.len()]; 3];
    orbit[0][grid.dc_index()] = Complex64::new(2.0, 0.0);
    orbit[1][grid.dc_index()] = Complex64::new(2.0, 0.0);
    orbit[2][grid.dc_index()] = Complex64::new(1e6, 0.0); // ground must not read this branch
    cosine(&grid, &mut orbit[1], &[1, 0], 0.2, 0.0);
    cosine(&grid, &mut orbit[1], &[0, 1], 0.08, 0.0);
    let constants = NoisePhysicalConstants::XYCE_7_10;
    let sources = solver
        .quasi_periodic_device_noise_sources_with_abort(
            grid.clone(),
            &orbit,
            300.0,
            constants,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[1].name, "M1 drain-bulk shot");
    assert_eq!(sources[1].injections, vec![(0, Complex64::ONE)]);
    let QuasiPeriodicNoiseSpectrum::White {
        density: shot,
        binary_scale_exponent: shot_scale,
    } = &sources[1].spectrum
    else {
        unreachable!()
    };
    for value in shot {
        assert!(
            (libm::scalbn(*value, *shot_scale) / (2.0 * constants.electron_charge * 1e-14) - 1.0)
                .abs()
                < 1e-13
        );
    }
    assert_eq!(sources[0].name, "M1 channel thermal");
    assert_eq!(sources[0].injections, vec![(0, Complex64::ONE)]);
    let QuasiPeriodicNoiseSpectrum::White {
        density,
        binary_scale_exponent,
    } = &sources[0].spectrum
    else {
        unreachable!()
    };
    for (i, q) in density.iter().enumerate() {
        let phases = grid.phases(i).unwrap();
        let gm = 2e-5 * (1.3 + 0.2 * phases[0].cos() + 0.08 * phases[1].cos());
        let expected = 4.0 * constants.boltzmann * 350.0 * (2.0 / 3.0) * gm;
        assert!((libm::scalbn(*q, *binary_scale_exponent) / expected - 1.0).abs() < 2e-13);
    }
    let abort = CountingAbort::new(10);
    assert!(matches!(
        solver.quasi_periodic_device_noise_sources_with_abort(
            grid.clone(),
            &orbit,
            300.0,
            constants,
            &ResourceLimits::default(),
            &abort
        ),
        Err(Error::Aborted)
    ));
    assert_eq!(abort.count(), 11);
    let limits = ResourceLimits {
        max_result_values: 1,
        ..ResourceLimits::default()
    };
    assert!(matches!(
        solver.quasi_periodic_device_noise_sources_with_abort(
            grid, &orbit, 300.0, constants, &limits, &NoAbort
        ),
        Err(Error::ResourceLimit(_))
    ));
}
