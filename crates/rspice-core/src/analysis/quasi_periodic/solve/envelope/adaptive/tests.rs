use super::*;
use crate::abort_signal::{ImmediateAbort, NoAbort};
use crate::analysis::harmonic_balance::{HbConfig, HbSolver};
use crate::analysis::quasi_periodic::QuasiPeriodicGridConfig;

fn fixture() -> (
    HbSolver,
    SpectralEnvelopeState,
    QuasiPeriodicSolveConfig,
    Vec<Vec<Complex64>>,
    SpectralEnvelopeControl,
) {
    let limits = ResourceLimits::default();
    let grid = Arc::new(
        QuasiPeriodicGrid::new_with_abort(
            QuasiPeriodicGridConfig::new(vec![1e6, 2.0_f64.sqrt() * 1e6], vec![1, 1]),
            &limits,
            &NoAbort,
        )
        .unwrap(),
    );
    let mut solver = HbSolver::new(HbConfig::new(1e6), 1);
    solver.add_conductance(0, 0, 1e-3);
    solver.add_capacitance(0, 0, 1e-6);
    let settings = QuasiPeriodicSolveConfig {
        relative_tolerance: 1e-10,
        current_absolute_tolerance: 1e-14,
        ..Default::default()
    };
    let mut source = vec![vec![Complex64::ZERO; grid.len()]];
    source[0][grid.dc_index()] = Complex64::new(1e-3, 0.0);
    let initial = solver
        .solve_quasi_periodic_with_abort(grid, &settings, &source, None, &limits, &NoAbort)
        .unwrap();
    let state = solver
        .initialize_spectral_envelope_with_abort(0.0, &initial, &limits, &NoAbort)
        .unwrap();
    let control = SpectralEnvelopeControl {
        method: SpectralEnvelopeMethod::Bdf2,
        minimum_step: 1e-8,
        maximum_step: 1e-3,
        relative_tolerance: 2e-3,
        absolute_tolerances: vec![1e-8],
        max_rejections: 12,
    };
    (solver, state, settings, source, control)
}

#[test]
fn adaptive_spectral_envelope_tracks_rc_decay_and_exact_deadlines() {
    let (mut solver, mut state, settings, mut source, control) = fixture();
    source[0].fill(Complex64::ZERO);
    let mut request = control.maximum_step;
    let mut rejected = 0;
    let mut count = 0;
    let mut saw_bdf2 = false;
    for deadline in [0.00073_f64, 0.002] {
        while state.time() < deadline {
            let previous = state.time();
            let accepted = advance_spectral_envelope_with_abort(
                &state,
                request,
                deadline,
                &control,
                &ResourceLimits::default(),
                &NoAbort,
                |state, time, method, limits, abort| {
                    solver.step_spectral_envelope_with_abort(
                        state, time, method, &settings, &source, limits, abort,
                    )
                },
            )
            .unwrap();
            assert!(accepted.state.time() > previous && accepted.state.time() <= deadline);
            assert!(accepted.error_ratio <= 1.0);
            assert!(
                accepted.suggested_step >= control.minimum_step
                    && accepted.suggested_step <= control.maximum_step
            );
            rejected += accepted.rejected_steps;
            saw_bdf2 |= accepted.state.order() == 2;
            state = accepted.state;
            request = accepted.suggested_step;
            let expected = (-1000.0 * state.time()).exp();
            let actual = state.spectra()[0][state.grid().dc_index()].re;
            assert!(
                (actual - expected).abs() < 6e-3,
                "{}: {actual} != {expected}",
                state.time()
            );
            count += 1;
            assert!(
                count < 250,
                "slow evolution must not resolve every RF cycle"
            );
        }
        assert_eq!(state.time().to_bits(), deadline.to_bits());
    }
    assert!(rejected > 0 && saw_bdf2);
    let before = state.spectra().to_vec();
    state.restart_integration_history();
    assert_eq!(state.spectra(), before);
    let next = advance_spectral_envelope_with_abort(
        &state,
        request,
        0.003,
        &control,
        &ResourceLimits::default(),
        &NoAbort,
        |state, time, method, limits, abort| {
            solver.step_spectral_envelope_with_abort(
                state, time, method, &settings, &source, limits, abort,
            )
        },
    )
    .unwrap();
    assert_eq!(
        next.state.order(),
        1,
        "a discontinuity restarts the stencil"
    );
}

#[test]
fn adaptive_spectral_envelope_bounds_retries_resources_and_deadline_probes() {
    let (mut solver, state, settings, source, mut control) = fixture();
    let limits = ResourceLimits::default();
    let mut calls = 0;
    let next = advance_spectral_envelope_with_abort(
        &state,
        1e-3,
        1e-3,
        &control,
        &limits,
        &NoAbort,
        |state, time, method, limits, abort| {
            calls += 1;
            if calls == 1 {
                return Err(Error::ConvergenceFailed {
                    iterations: 2,
                    merit: 10.0,
                });
            }
            solver.step_spectral_envelope_with_abort(
                state, time, method, &settings, &source, limits, abort,
            )
        },
    )
    .unwrap();
    assert_eq!(next.rejected_steps, 1);
    assert_eq!(next.spectral_solves, 4);
    assert_eq!(next.state.time(), 5e-4);
    control.minimum_step = 1e-4;
    let next = advance_spectral_envelope_with_abort(
        &state,
        1e-3,
        1e-5,
        &control,
        &limits,
        &NoAbort,
        |state, time, method, limits, abort| {
            solver.step_spectral_envelope_with_abort(
                state, time, method, &settings, &source, limits, abort,
            )
        },
    )
    .unwrap();
    assert_eq!(
        next.state.time(),
        1e-5,
        "the exact deadline may precede the preferred floor"
    );
    let mut tiny = limits;
    tiny.max_analysis_points = 2;
    assert!(matches!(
        advance_spectral_envelope_with_abort(
            &state,
            1e-3,
            1e-3,
            &control,
            &tiny,
            &NoAbort,
            |state, time, method, limits, abort| solver.step_spectral_envelope_with_abort(
                state, time, method, &settings, &source, limits, abort
            )
        ),
        Err(Error::ResourceLimit(_))
    ));
    assert!(matches!(
        advance_spectral_envelope_with_abort(
            &state,
            1e-3,
            1e-3,
            &control,
            &limits,
            &ImmediateAbort,
            |_, _, _, _, _| panic!("cancelled before a trial")
        ),
        Err(Error::Aborted)
    ));
    control.max_rejections = 0;
    assert!(matches!(
        advance_spectral_envelope_with_abort(
            &state,
            1e-3,
            1e-3,
            &control,
            &limits,
            &NoAbort,
            |_, _, _, _, _| Err(Error::ConvergenceFailed {
                iterations: 2,
                merit: 10.0
            })
        ),
        Err(Error::ConvergenceFailed { .. })
    ));
    assert!(matches!(
        advance_spectral_envelope_with_abort(
            &state,
            1e-3,
            1e-3,
            &control,
            &limits,
            &NoAbort,
            |_, _, _, _, _| Err(Error::InvalidCircuit("bad model".into()))
        ),
        Err(Error::InvalidCircuit(_))
    ));
    control.absolute_tolerances.clear();
    assert!(matches!(
        advance_spectral_envelope_with_abort(
            &state,
            1e-3,
            1e-3,
            &control,
            &limits,
            &NoAbort,
            |_, _, _, _, _| panic!("invalid control before a trial")
        ),
        Err(Error::InvalidConfig(_))
    ));
    assert_eq!(state.time(), 0.0);
    assert!(
        (state.spectra()[0][state.grid().dc_index()] - Complex64::new(1.0, 0.0)).norm() < 1e-12
    );
}

#[test]
fn adaptive_spectral_envelope_error_checks_combined_physical_waveforms() {
    let (_, mut zero, _, _, mut control) = fixture();
    zero.solution.spectra[0].fill(Complex64::ZERO);
    let mut perturbed = zero.clone();
    let grid = zero.grid();
    perturbed.solution.spectra[0][grid.dc_index()] = Complex64::new(8e-4, 0.0);
    let k = grid.index_of(&[1, 0]).unwrap();
    perturbed.solution.spectra[0][k] = Complex64::new(4e-4, 0.0);
    perturbed.solution.spectra[0][grid.len() - 1 - k] = Complex64::new(4e-4, 0.0);
    control.relative_tolerance = 0.0;
    control.absolute_tolerances[0] = 1e-3;
    // Every coefficient individually meets the bound; their waveform does not.
    let error = compare(&zero, &zero, &perturbed, &control, 1.0, &NoAbort).unwrap();
    assert!((error - 1.6).abs() < 1e-12);
}
