//! Independent derivative and convergence-scale checks for the spectral kernel.
use super::*;
use crate::abort_signal::NoAbort;
use crate::analysis::quasi_periodic::{QuasiPeriodicGridConfig, QuasiPeriodicSampling};

struct Polynomial;
impl Circuit for Polynomial {
    fn unknowns(&self) -> usize {
        1
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(Vec::new())
    }
    fn sample(&mut self, x: &[Value], jacobian: bool) -> Result<Sample, Error> {
        Ok(Sample {
            current: vec![(0, -x[0].powi(3))],
            charge: vec![(0, -0.5 * x[0].powi(2))],
            conductance: if jacobian {
                vec![(0, 0, 3.0 * x[0].powi(2))]
            } else {
                vec![]
            },
            capacitance: if jacobian { vec![(0, 0, x[0])] } else { vec![] },
        })
    }
}

fn workspace(config: &QuasiPeriodicSolveConfig) -> Workspace<'_> {
    let mut basis = QuasiPeriodicGridConfig::new(vec![1.0, std::f64::consts::SQRT_2], vec![1, 1]);
    basis.max_mixing_order = Some(1);
    basis.sampling = QuasiPeriodicSampling::Exact(vec![9, 11]);
    let grid = Arc::new(
        QuasiPeriodicGrid::new_with_abort(basis, &ResourceLimits::default(), &NoAbort).unwrap(),
    );
    Workspace {
        transform: QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap(),
        linear: vec![vec![]; grid.len()],
        grid,
        config,
        unknowns: 1,
        voltage_rows: vec![false],
        base_values: 0,
        value_limit: 25_000_000,
    }
}

#[test]
fn quasi_periodic_analytic_jacobian_preserves_coupling_outside_retained_mixed_grid() {
    let config = QuasiPeriodicSolveConfig::default();
    let mut work = workspace(&config);
    let state = vec![0.2, 0.03, -0.02, 0.07, 0.01];
    let size = work.grid.len();
    assert_eq!(size, 5);
    let sources = vec![vec![Complex64::ZERO; size]];
    let spectra = coordinates::decode(&state, size);
    let evaluation = work
        .evaluate(&mut Polynomial, &spectra, &sources, true, &NoAbort)
        .unwrap();
    for column in 0..size {
        let analytic = work.jacobian_column(&evaluation, column, &NoAbort).unwrap();
        let mut plus = state.clone();
        let mut minus = state.clone();
        plus[column] += 1e-6;
        minus[column] -= 1e-6;
        let rp = work
            .evaluate(
                &mut Polynomial,
                &coordinates::decode(&plus, size),
                &sources,
                false,
                &NoAbort,
            )
            .unwrap();
        let rm = work
            .evaluate(
                &mut Polynomial,
                &coordinates::decode(&minus, size),
                &sources,
                false,
                &NoAbort,
            )
            .unwrap();
        for ((actual, p), m) in analytic
            .iter()
            .zip(coordinates::encode(&rp.residual))
            .zip(coordinates::encode(&rm.residual))
        {
            let expected = -(p - m) / 2e-6;
            assert!(
                (actual - expected).abs() < 1e-8,
                "column {column}: {actual:e} != {expected:e}"
            );
        }
    }
}

struct CancellingBias;
impl Circuit for CancellingBias {
    fn unknowns(&self) -> usize {
        1
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(Vec::new())
    }
    fn sample(&mut self, x: &[Value], _: bool) -> Result<Sample, Error> {
        Ok(Sample {
            current: vec![(0, 2.0 + x[0]), (0, -2.0 - 0.99 * x[0])],
            ..Default::default()
        })
    }
}

#[test]
fn quasi_periodic_certificate_keeps_dc_out_of_small_ac_tolerance() {
    let config = QuasiPeriodicSolveConfig {
        relative_tolerance: 1e-3,
        current_absolute_tolerance: 1e-15,
        ..Default::default()
    };
    let mut work = workspace(&config);
    let size = work.grid.len();
    let spectra = coordinates::decode(&[0.0, 5e-5, 0.0, 0.0, 0.0], size);
    let evaluation = work
        .evaluate(
            &mut CancellingBias,
            &spectra,
            &vec![vec![Complex64::ZERO; size]],
            false,
            &NoAbort,
        )
        .unwrap();
    let expected = (5e-5 * 0.01) / (1e-15 + 1e-3 * 5e-5 * 1.99);
    assert!(
        (evaluation.merit - expected).abs() < 2e-6,
        "{} != {expected}",
        evaluation.merit
    );
    assert!(evaluation.merit > 1.0);
}

struct CoupledPolynomial;
impl Circuit for CoupledPolynomial {
    fn unknowns(&self) -> usize {
        2
    }
    fn voltage_equation(&self, row: usize) -> bool {
        row == 1
    }
    fn linear_entries(&self, frequency: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(vec![
            (0, 0, Complex64::new(0.7, 0.0)),
            (0, 1, Complex64::new(1.0, 0.0)),
            (1, 0, Complex64::new(-1.0, 0.0)),
            (
                1,
                1,
                Complex64::new(0.0, std::f64::consts::TAU * frequency * 0.01),
            ),
        ])
    }
    fn sample(&mut self, x: &[Value], jacobian: bool) -> Result<Sample, Error> {
        Ok(Sample {
            current: vec![(0, -x[0].powi(3) - 2.0 * x[1].powi(2)), (1, -x[0] * x[1])],
            charge: vec![
                (0, -0.5 * x[0].powi(2) - 0.3 * x[1].powi(3)),
                (1, -x[0] * x[1]),
            ],
            conductance: if jacobian {
                vec![
                    (0, 0, 3.0 * x[0].powi(2)),
                    (0, 1, 4.0 * x[1]),
                    (1, 0, x[1]),
                    (1, 1, x[0]),
                ]
            } else {
                vec![]
            },
            capacitance: if jacobian {
                vec![
                    (0, 0, x[0]),
                    (0, 1, 0.9 * x[1].powi(2)),
                    (1, 0, x[1]),
                    (1, 1, x[0]),
                ]
            } else {
                vec![]
            },
        })
    }
}

#[test]
fn quasi_periodic_matrix_free_action_matches_coupled_physical_residual_derivatives() {
    let config = QuasiPeriodicSolveConfig::default();
    let mut work = workspace(&config);
    work.unknowns = 2;
    work.voltage_rows = vec![false, true];
    work.linear = work
        .grid
        .frequencies_hz()
        .iter()
        .map(|f| CoupledPolynomial.linear_entries(*f).unwrap())
        .collect();
    let entries = work.grid.len();
    let state = vec![0.2, 0.03, -0.02, 0.07, 0.01, -0.1, 0.02, 0.04, -0.08, 0.06];
    let sources = vec![vec![Complex64::ZERO; entries]; 2];
    let evaluation = work
        .evaluate(
            &mut CoupledPolynomial,
            &coordinates::decode(&state, entries),
            &sources,
            true,
            &NoAbort,
        )
        .unwrap();
    // Dense directions exercise both MNA rows and both quadratures together;
    // this oracle differentiates the physical residual, not assembled columns.
    for direction in [
        vec![1.0; state.len()],
        vec![0.2, -0.6, 0.5, -0.3, 0.7, 0.1, 0.4, -0.9, 0.8, -0.2],
    ] {
        let actual = work
            .jacobian_action(&evaluation, &direction, &NoAbort)
            .unwrap();
        let mut residuals = Vec::new();
        for step in [1e-6, -1e-6] {
            let trial: Vec<_> = state
                .iter()
                .zip(&direction)
                .map(|(x, v)| x + step * v)
                .collect();
            let value = work
                .evaluate(
                    &mut CoupledPolynomial,
                    &coordinates::decode(&trial, entries),
                    &sources,
                    false,
                    &NoAbort,
                )
                .unwrap();
            residuals.push(coordinates::encode(&value.residual));
        }
        for ((actual, plus), minus) in actual.iter().zip(&residuals[0]).zip(&residuals[1]) {
            let expected = -(plus - minus) / 2e-6;
            assert!(
                (actual - expected).abs() < 2e-8 * (1.0 + expected.abs()),
                "{actual:e} != {expected:e}"
            );
        }
    }
    let abort = crate::abort_signal::CountingAbort::new(4);
    assert!(matches!(
        work.jacobian_action(&evaluation, &state, &abort),
        Err(Error::Aborted)
    ));
    assert!(
        work.jacobian_action(&evaluation, &state[..9], &NoAbort)
            .is_err()
    );
}

struct SingularVarying;
impl Circuit for SingularVarying {
    fn unknowns(&self) -> usize {
        2
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(vec![])
    }
    fn sample(&mut self, x: &[Value], jacobian: bool) -> Result<Sample, Error> {
        Ok(Sample {
            current: vec![(0, -x[0] * x[1]), (1, -x[0] * x[1])],
            conductance: if jacobian {
                vec![(0, 0, x[1]), (0, 1, x[0]), (1, 0, x[1]), (1, 1, x[0])]
            } else {
                vec![]
            },
            ..Default::default()
        })
    }
}
#[test]
fn qpss_krylov_refuses_singular_zero_residual_and_observes_cancellation() {
    let mut config = QuasiPeriodicSolveConfig::default();
    config.linear.method = QuasiPeriodicLinearMethod::Krylov;
    let mut work = workspace(&config);
    work.unknowns = 2;
    work.voltage_rows = vec![false; 2];
    work.linear = vec![SingularVarying.linear_entries(0.0).unwrap(); work.grid.len()];
    let mut seed = vec![vec![Complex64::ZERO; work.grid.len()]; 2];
    let tone = work.grid.index_of(&[1, 0]).unwrap();
    let conjugate = work.grid.len() - 1 - tone;
    seed[0][tone] = Complex64::new(0.5, 0.0);
    seed[0][conjugate] = Complex64::new(0.5, 0.0);
    let sources = vec![vec![Complex64::ZERO; work.grid.len()]; 2];
    let evaluation = work
        .evaluate(&mut SingularVarying, &seed, &sources, true, &NoAbort)
        .unwrap();
    assert_eq!(evaluation.merit, 0.0);
    assert!(work.iterative_correction(&evaluation, &NoAbort).is_err());
    let abort = crate::abort_signal::CountingAbort::new(15);
    assert!(matches!(
        work.iterative_correction(&evaluation, &abort),
        Err(Error::Aborted)
    ));
    assert_eq!(abort.polls_after_abort(), 0);
}
