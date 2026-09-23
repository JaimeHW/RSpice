use super::*;
use crate::abort_signal::{ImmediateAbort, NoAbort};
use crate::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicGridConfig};
use std::sync::Arc;

fn grid() -> Arc<QuasiPeriodicGrid> {
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(
            QuasiPeriodicGridConfig::new(vec![1e6, 2.0_f64.sqrt() * 1e6], vec![1, 1]),
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
    )
}
fn policy() -> QuasiPeriodicSolveConfig {
    QuasiPeriodicSolveConfig {
        relative_tolerance: 1e-9,
        current_absolute_tolerance: 1e-14,
        ..Default::default()
    }
}
fn drive(grid: &QuasiPeriodicGrid, rows: usize, scale: Value) -> Vec<Vec<Complex64>> {
    let mut sources = vec![vec![Complex64::ZERO; grid.len()]; rows];
    sources[0][grid.dc_index()] = Complex64::new(scale, 0.0);
    for tuple in [[1, 0], [0, 1]] {
        let k = grid.index_of(&tuple).unwrap();
        sources[0][k] = Complex64::new(scale * 0.2, scale * 0.1);
        sources[0][grid.len() - 1 - k] = sources[0][k].conj();
    }
    sources
}
fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() <= 1e-10 * expected.norm().max(1.0),
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn spectral_envelope_variable_steps_preserve_charge_across_slow_circuit_changes() {
    let grid = grid();
    let limits = ResourceLimits::default();
    let config = policy();
    let g = 1e-3;
    let c = 1e-6;
    let mut solver = HbSolver::new(HbConfig::new(1e6), 1);
    solver.add_conductance(0, 0, g);
    solver.add_capacitance(0, 0, c);
    let initial = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config,
            &drive(&grid, 1, 1e-3),
            None,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let mut state = solver
        .initialize_spectral_envelope_with_abort(0.0, &initial, &limits, &NoAbort)
        .unwrap();
    let mut expected = initial.spectra()[0].clone();
    let mut older = expected.clone();
    let mut prior_h = 0.0;
    for (index, time) in [0.001, 0.0018, 0.003, 0.008].into_iter().enumerate() {
        let h = time - state.time();
        let ratio = h / prior_h;
        let (a0, a1, a2) = if index > 0 && ratio <= 2.0 {
            (
                (1.0 + 2.0 * ratio) / (1.0 + ratio) / h,
                -(1.0 + ratio) / h,
                ratio * ratio / (1.0 + ratio) / h,
            )
        } else {
            (1.0 / h, -1.0 / h, 0.0)
        };
        let sources = drive(&grid, 1, (index as f64 + 2.0) * 1e-3);
        let next = solver
            .step_spectral_envelope_with_abort(
                &state,
                time,
                SpectralEnvelopeMethod::Bdf2,
                &config,
                &sources,
                &limits,
                &NoAbort,
            )
            .unwrap();
        for (k, &frequency) in grid.frequencies_hz().iter().enumerate() {
            let answer = (sources[0][k] - c * (a1 * expected[k] + a2 * older[k]))
                / Complex64::new(g + a0 * c, std::f64::consts::TAU * frequency * c);
            close(next.spectra()[0][k], answer);
        }
        older = expected;
        expected = next.spectra()[0].clone();
        assert_eq!(next.order(), if a2 == 0.0 { 1 } else { 2 });
        assert!(next.normalized_residual() <= 1.0);
        // These accepted steps each span hundreds of carrier periods.
        assert!(h * 1e6 > 100.0);
        state = next;
        prior_h = h;
    }
    // Old Q must not be reconstructed with the newly changed capacitor.
    solver.add_capacitance(0, 0, c);
    let sources = drive(&grid, 1, 0.0);
    let next = solver
        .step_spectral_envelope_with_abort(
            &state,
            0.009,
            SpectralEnvelopeMethod::BackwardEuler,
            &config,
            &sources,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let a = 1.0 / (0.009 - state.time());
    for (k, &frequency) in grid.frequencies_hz().iter().enumerate() {
        close(
            next.spectra()[0][k],
            a * c * state.spectra()[0][k]
                / Complex64::new(g + a * 2.0 * c, std::f64::consts::TAU * frequency * 2.0 * c),
        );
    }
    assert!(matches!(
        solver.step_spectral_envelope_with_abort(
            &state,
            0.009,
            SpectralEnvelopeMethod::Bdf2,
            &config,
            &sources,
            &limits,
            &ImmediateAbort
        ),
        Err(Error::Aborted)
    ));
    let mut tiny = limits;
    tiny.max_result_values = 1;
    assert!(matches!(
        solver.step_spectral_envelope_with_abort(
            &state,
            0.009,
            SpectralEnvelopeMethod::Bdf2,
            &config,
            &sources,
            &tiny,
            &NoAbort
        ),
        Err(Error::ResourceLimit(_))
    ));
    assert!(
        solver
            .step_spectral_envelope_with_abort(
                &state,
                state.time(),
                SpectralEnvelopeMethod::Bdf2,
                &config,
                &sources,
                &limits,
                &NoAbort
            )
            .is_err()
    );
    assert_eq!(
        state.spectra()[0],
        expected,
        "failed trials cannot alter accepted history"
    );
}

#[test]
fn spectral_envelope_retains_exact_mna_inductor_flux() {
    let grid = grid();
    let limits = ResourceLimits::default();
    let config = policy();
    let (g, c, l) = (0.1, 1e-6, 1e-3);
    let mut solver = HbSolver::new(HbConfig::new(1e6), 1);
    solver.add_conductance(0, 0, g);
    solver.add_capacitance(0, 0, c);
    solver
        .try_add_periodic_inductor_branch(1, 0, l, 1, "L1")
        .unwrap();
    let initial = solver
        .solve_quasi_periodic_with_abort(
            grid.clone(),
            &config,
            &drive(&grid, 2, 0.01),
            None,
            &limits,
            &NoAbort,
        )
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.0, &initial, &limits, &NoAbort)
        .unwrap();
    let sources = drive(&grid, 2, 0.02);
    let h = 1e-4;
    let next = solver
        .step_spectral_envelope_with_abort(
            &state,
            h,
            SpectralEnvelopeMethod::BackwardEuler,
            &config,
            &sources,
            &limits,
            &NoAbort,
        )
        .unwrap();
    for (k, &frequency) in grid.frequencies_hz().iter().enumerate() {
        let derivative = Complex64::new(1.0 / h, std::f64::consts::TAU * frequency);
        let flux_history = -l * state.spectra()[1][k] / h;
        let rhs = sources[0][k] + c * state.spectra()[0][k] / h;
        let voltage =
            (rhs + flux_history / (l * derivative)) / (g + c * derivative + 1.0 / (l * derivative));
        close(next.spectra()[0][k], voltage);
        close(
            next.spectra()[1][k],
            (voltage - flux_history) / (l * derivative),
        );
    }
}

#[test]
fn spectral_envelope_integrates_nonlinear_junction_charge() {
    let grid = grid();
    let limits = ResourceLimits::default();
    let config = policy();
    let mut solver = HbSolver::new(HbConfig::new(1e6), 1);
    let mut diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
    diode.params.cap_a = DepletionCap::new(1e-6, 0.8, 0.5, 0.5);
    let current = |v: Value| 1e-3 * v + 1e-14 * (v / 0.02585).exp_m1();
    let charge = |v: Value| 2.0 * 1e-6 * 0.8 * (1.0 - (1.0 - v / 0.8).sqrt());
    solver.add_conductance(0, 0, 1e-3);
    solver.add_nonlinear_device(diode);
    let mut source = vec![vec![Complex64::ZERO; grid.len()]];
    source[0][grid.dc_index()] = Complex64::new(current(-0.1), 0.0);
    let initial = solver
        .solve_quasi_periodic_with_abort(grid.clone(), &config, &source, None, &limits, &NoAbort)
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.0, &initial, &limits, &NoAbort)
        .unwrap();
    let (h, expected) = (1e-4, -0.7);
    source[0][grid.dc_index()] = Complex64::new(
        current(expected) + (charge(expected) - charge(-0.1)) / h,
        0.0,
    );
    let next = solver
        .step_spectral_envelope_with_abort(
            &state,
            h,
            SpectralEnvelopeMethod::BackwardEuler,
            &config,
            &source,
            &limits,
            &NoAbort,
        )
        .unwrap();
    close(
        next.spectra()[0][grid.dc_index()],
        Complex64::new(expected, 0.0),
    );
    for (k, value) in next.spectra()[0].iter().enumerate() {
        if k != grid.dc_index() {
            close(*value, Complex64::ZERO);
        }
    }
}
