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
        ..limits.clone()
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
        ..limits.clone()
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
