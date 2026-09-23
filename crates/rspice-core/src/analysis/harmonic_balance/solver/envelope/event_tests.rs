use super::*;
use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGrid, QuasiPeriodicGridConfig, QuasiPeriodicTransform,
};
use std::sync::Arc;

fn settings(n: usize) -> SpectralEnvelopeEventConfig {
    SpectralEnvelopeEventConfig {
        solver: QuasiPeriodicSolveConfig {
            relative_tolerance: 1e-9,
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-11,
            ..Default::default()
        },
        charge_tolerances: vec![1e-15; n],
        rate_tolerances: vec![1e-11; n],
    }
}
fn basis(order: usize) -> Arc<QuasiPeriodicGrid> {
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(
            QuasiPeriodicGridConfig::new(vec![1e6, 2.0_f64.sqrt() * 1e6], vec![order; 2]),
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
    )
}
fn close(a: Complex64, b: Complex64, tolerance: Value) {
    assert!(
        (a - b).norm() <= tolerance,
        "{a:?} != {b:?}; tolerance {tolerance:e}"
    );
}
fn cosine(row: &mut [Complex64], grid: &QuasiPeriodicGrid, tuple: &[i32], value: Value) {
    let k = grid.index_of(tuple).unwrap();
    row[k] = Complex64::new(value * 0.5, 0.0);
    row[grid.len() - 1 - k] = row[k].conj();
}

#[test]
fn spectral_envelope_event_retains_rc_charge_rl_flux_and_separate_source_impulses() {
    let grid = basis(1);
    let limits = ResourceLimits::default();
    let config = settings(4);
    let (g, c, cd, l) = (1e-3, 1e-6, 2e-6, 1e-3);
    let mut solver = HbSolver::new(HbConfig::new(1e6), 2);
    solver.add_conductance(1, 1, g);
    for (r, col, v) in [(0, 0, c + cd), (0, 1, -c), (1, 0, -c), (1, 1, c)] {
        solver.add_capacitance(r, col, v);
    }
    solver
        .try_add_periodic_voltage_source_branch(1, 0, 0, 1, "Vdrive")
        .unwrap();
    solver
        .try_add_periodic_inductor_branch(2, 0, l, 2, "Lout")
        .unwrap();
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; 4];
    let dc = grid.dc_index();
    sources[2][dc] = Complex64::ONE;
    cosine(&mut sources[2], &grid, &[1, 0], 0.1);
    cosine(&mut sources[2], &grid, &[0, 1], 0.03);
    let initial = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config.solver,
            &sources,
            None,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.001, &initial, &limits, &NoAbort)
        .unwrap();
    sources[2][dc] = Complex64::new(2.0, 0.0);
    let mut slopes = vec![vec![Complex64::ZERO; grid.len()]; 4];
    slopes[2][dc] = Complex64::new(200.0, 0.0);
    use SpectralEnvelopeEventEquation::*;
    let rows = [
        Charge,
        Charge,
        VoltageSource {
            positive: Some(0),
            negative: None,
        },
        Charge,
    ];
    let event = solver
        .transition_spectral_envelope_with_abort(
            &state, &rows, &config, &sources, &slopes, &limits, &NoAbort,
        )
        .unwrap();
    close(
        event.state.spectra()[0][dc],
        Complex64::new(2.0, 0.0),
        1e-10,
    );
    close(event.state.spectra()[1][dc], Complex64::ONE, 1e-10);
    close(event.state.spectra()[3][dc], Complex64::ZERO, 1e-10);
    close(
        event.current_impulses[2][dc],
        Complex64::new(-cd, 0.0),
        1e-14,
    );
    close(event.slow_rates[0][dc], Complex64::new(200.0, 0.0), 1e-6);
    close(event.slow_rates[1][dc], Complex64::new(-800.0, 0.0), 1e-6);
    close(event.slow_rates[3][dc], Complex64::new(1000.0, 0.0), 1e-6);
    close(
        event.state.spectra()[2][dc],
        Complex64::new(-0.0014, 0.0),
        1e-10,
    );
    for k in 0..grid.len() {
        close(event.state.spectra()[3][k], state.spectra()[3][k], 1e-10);
        if k != dc {
            for r in 0..4 {
                close(event.state.spectra()[r][k], state.spectra()[r][k], 1e-9);
            }
            close(event.current_impulses[2][k], Complex64::ZERO, 1e-14);
        }
    }
    assert_eq!(event.state.time(), state.time());
    assert!(event.state.normalized_residual() <= 1.0);
    assert!(event.current_impulses[0].is_empty() && event.slow_rates[2].is_empty());
    let next = solver
        .step_spectral_envelope_with_abort(
            &event.state,
            0.0011,
            SpectralEnvelopeMethod::Bdf2,
            &config.solver,
            &sources,
            &limits,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(next.order(), 1, "event must restart BDF history");
    assert!(matches!(
        solver.transition_spectral_envelope_with_abort(
            &state,
            &rows,
            &config,
            &sources,
            &slopes,
            &limits,
            &ImmediateAbort
        ),
        Err(Error::Aborted)
    ));
    assert!(matches!(
        solver.transition_spectral_envelope_with_abort(
            &state,
            &rows,
            &config,
            &sources,
            &slopes,
            &limits,
            &CountingAbort::new(50)
        ),
        Err(Error::Aborted)
    ));
    let mut tiny = limits;
    tiny.max_result_values = 1;
    assert!(matches!(
        solver.transition_spectral_envelope_with_abort(
            &state, &rows, &config, &sources, &slopes, &tiny, &NoAbort
        ),
        Err(Error::ResourceLimit(_))
    ));
    assert!(
        solver
            .transition_spectral_envelope_with_abort(
                &state,
                &rows[..3],
                &config,
                &sources,
                &slopes,
                &limits,
                &NoAbort
            )
            .is_err()
    );
    close(state.spectra()[0][dc], Complex64::ONE, 1e-10);
}

#[test]
fn spectral_envelope_event_projects_nonlinear_charge_and_outgoing_carrier_currents() {
    let grid = basis(2);
    let limits = ResourceLimits::default();
    let config = settings(2);
    let mut solver = HbSolver::new(HbConfig::new(1e6), 1);
    solver.add_conductance(0, 0, 1e-3);
    let mut diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
    diode.params.cap_a = DepletionCap::new(1e-6, 0.8, 0.5, 0.5);
    solver.add_nonlinear_device(diode);
    solver
        .try_add_periodic_voltage_source_branch(1, 0, 0, 1, "Vdrive")
        .unwrap();
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
    sources[1][grid.dc_index()] = Complex64::new(-0.2, 0.0);
    cosine(&mut sources[1], &grid, &[1, 0], 0.05);
    cosine(&mut sources[1], &grid, &[0, 1], 0.02);
    let initial = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config.solver,
            &sources,
            None,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.002, &initial, &limits, &NoAbort)
        .unwrap();
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let old_v = transform
        .to_real_samples_with_abort(&sources[1], &NoAbort)
        .unwrap();
    sources[1][grid.dc_index()] = Complex64::new(-0.1, 0.0);
    cosine(&mut sources[1], &grid, &[1, 0], 0.03);
    let new_v = transform
        .to_real_samples_with_abort(&sources[1], &NoAbort)
        .unwrap();
    let mut slopes = vec![vec![Complex64::ZERO; grid.len()]; 2];
    slopes[1][grid.dc_index()] = Complex64::new(40.0, 0.0);
    cosine(&mut slopes[1], &grid, &[0, 1], 20.0);
    let v_rate = transform
        .to_real_samples_with_abort(&slopes[1], &NoAbort)
        .unwrap();
    use SpectralEnvelopeEventEquation::*;
    let event = solver
        .transition_spectral_envelope_with_abort(
            &state,
            &[
                Charge,
                VoltageSource {
                    positive: Some(0),
                    negative: None,
                },
            ],
            &config,
            &sources,
            &slopes,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let q = |v: Value| 1.6e-6 * (1.0 - (1.0 - v / 0.8).sqrt());
    let c = |v: Value| 1e-6 / (1.0 - v / 0.8).sqrt();
    let impulse_samples = new_v
        .iter()
        .zip(&old_v)
        .map(|(&a, &b)| Complex64::new(q(b) - q(a), 0.0))
        .collect::<Vec<_>>();
    let expected_impulses = transform
        .to_spectrum_with_abort(&impulse_samples, &NoAbort)
        .unwrap();
    let charge_samples = new_v
        .iter()
        .map(|&v| Complex64::new(q(v), 0.0))
        .collect::<Vec<_>>();
    let charge = transform
        .to_spectrum_with_abort(&charge_samples, &NoAbort)
        .unwrap();
    let fast_current = grid.differentiate_with_abort(&charge, &NoAbort).unwrap();
    let current_samples = new_v
        .iter()
        .zip(&v_rate)
        .map(|(&v, &rate)| {
            Complex64::new(
                -1e-3 * v - 1e-14 * (v / 0.02585).exp_m1() - c(v) * rate,
                0.0,
            )
        })
        .collect::<Vec<_>>();
    let finite = transform
        .to_spectrum_with_abort(&current_samples, &NoAbort)
        .unwrap();
    for k in 0..grid.len() {
        close(event.state.spectra()[0][k], sources[1][k], 1e-10);
        close(event.current_impulses[1][k], expected_impulses[k], 2e-14);
        close(
            event.state.spectra()[1][k],
            finite[k] - fast_current[k],
            2e-9,
        );
        close(event.slow_rates[0][k], slopes[1][k], 1e-6);
    }
    assert!(
        event.current_impulses[1][grid.index_of(&[1, 1]).unwrap()].norm() > 1e-12,
        "nonlinear event charge must retain mixed-tone impulse coefficients"
    );
}

#[test]
fn spectral_envelope_event_preserves_floating_charge_and_audits_replaced_kcl() {
    let grid = basis(1);
    let limits = ResourceLimits::default();
    let config = settings(2);
    let mut solver = HbSolver::new(HbConfig::new(1e6), 2);
    solver.add_conductance(0, 0, 1e-3);
    solver.add_conductance(1, 1, 2e-3);
    for (r, c, v) in [(0, 0, 1e-6), (0, 1, -1e-6), (1, 0, -1e-6), (1, 1, 1e-6)] {
        solver.add_capacitance(r, c, v);
    }
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
    cosine(&mut sources[0], &grid, &[1, 0], 1e-4);
    let initial = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config.solver,
            &sources,
            None,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.001, &initial, &limits, &NoAbort)
        .unwrap();
    let dc = grid.dc_index();
    sources[0][dc] = Complex64::new(1e-3, 0.0);
    let slopes = vec![vec![Complex64::ZERO; grid.len()]; 2];
    use SpectralEnvelopeEventEquation::*;
    let rows = [Algebraic(vec![(0, 1.0), (1, 1.0)]), Charge];
    let event = solver
        .transition_spectral_envelope_with_abort(
            &state, &rows, &config, &sources, &slopes, &limits, &NoAbort,
        )
        .unwrap();
    // The capacitor voltage cannot jump. Summed KCL fixes the common-mode
    // jump; differentiated summed KCL then fixes the two individual rates.
    for r in 0..2 {
        close(
            event.state.spectra()[r][dc],
            Complex64::new(1.0 / 3.0, 0.0),
            1e-10,
        );
    }
    close(
        event.slow_rates[0][dc],
        Complex64::new(4000.0 / 9.0, 0.0),
        1e-6,
    );
    close(
        event.slow_rates[1][dc],
        Complex64::new(-2000.0 / 9.0, 0.0),
        1e-6,
    );
    for k in 0..grid.len() {
        close(
            event.state.spectra()[0][k] - event.state.spectra()[1][k],
            state.spectra()[0][k] - state.spectra()[1][k],
            1e-10,
        );
    }
    assert!(event.current_impulses.iter().all(Vec::is_empty));
    let bad = [Algebraic(vec![(0, 1.0)]), Charge];
    let error = solver
        .transition_spectral_envelope_with_abort(
            &state, &bad, &config, &sources, &slopes, &limits, &NoAbort,
        )
        .unwrap_err();
    assert!(
        error.to_string().contains("finite-current equations"),
        "{error}"
    );
}
