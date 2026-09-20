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
