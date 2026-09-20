//! Analytic convolution oracles for translated complex F/Q systems.
use super::super::solve::Sample;
use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicLinearMethod, QuasiPeriodicSampling,
};
use std::collections::BTreeMap;

struct Polynomial {
    singular: bool,
}
impl Circuit for Polynomial {
    fn unknowns(&self) -> usize {
        2
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, frequency: Value) -> Result<Vec<LinearEntry>, Error> {
        if self.singular {
            return Ok(vec![]);
        }
        Ok(vec![
            (
                0,
                0,
                Complex64::new(2.0, std::f64::consts::TAU * frequency * 0.03),
            ),
            (1, 1, Complex64::new(1.0, 0.0)),
        ])
    }
    fn sample(&mut self, state: &[Value], _: bool) -> Result<Sample, Error> {
        if self.singular {
            // Both equations are identical, including phase-varying entries.
            let g = 1.0 + state[1].powi(2);
            return Ok(Sample {
                conductance: vec![(0, 0, g), (0, 1, g), (1, 0, g), (1, 1, g)],
                ..Default::default()
            });
        }
        Ok(Sample {
            conductance: vec![(0, 0, 0.3 * state[1].powi(2))],
            capacitance: vec![(0, 0, 0.07 * state[1].powi(2))],
            ..Default::default()
        })
    }
}

fn basis() -> Arc<QuasiPeriodicGrid> {
    let mut config = QuasiPeriodicGridConfig::new(vec![1.0, std::f64::consts::SQRT_2], vec![1, 1]);
    config.max_mixing_order = Some(1);
    config.sampling = QuasiPeriodicSampling::Exact(vec![9, 11]);
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    )
}
fn orbit(grid: &QuasiPeriodicGrid) -> Vec<Vec<Complex64>> {
    let mut orbit = vec![vec![Complex64::ZERO; grid.len()]; 2];
    for (tuple, v) in [
        ([1, 0], Complex64::new(0.2, 0.1)),
        ([0, 1], Complex64::new(-0.15, 0.05)),
    ] {
        let k = grid.index_of(&tuple).unwrap();
        orbit[1][k] = v;
        orbit[1][grid.len() - 1 - k] = v.conj();
    }
    orbit
}

#[test]
fn qpac_complex_coupling_and_charge_translation_match_analytic_polynomial_convolution() {
    let grid = basis();
    let orbit = orbit(&grid);
    // Square the authored orbit in tuple space independently of the sampled
    // FFT operator. Keep differences (2,0), (1,-1), etc. outside this basis.
    let mut squared: BTreeMap<Vec<i32>, Complex64> = BTreeMap::new();
    for (i, a) in grid.indices().iter().enumerate() {
        for (j, b) in grid.indices().iter().enumerate() {
            let tuple = a.iter().zip(b).map(|(a, b)| a + b).collect();
            *squared.entry(tuple).or_default() += orbit[1][i] * orbit[1][j];
        }
    }
    assert!(!grid.indices().contains(&vec![2, 0]));
    assert!(squared[&vec![2, 0]].norm() > 0.01);
    let expected: Vec<_> = (0..2 * grid.len())
        .map(|i| Complex64::new(0.11 + i as Value * 0.03, -0.2 + i as Value * 0.02))
        .collect();
    for offset in [0.0, 0.37, -1.0] {
        let mut rhs = vec![vec![Complex64::ZERO; grid.len()]; 2];
        for (k, out) in grid.indices().iter().enumerate() {
            let omega = std::f64::consts::TAU * (offset + grid.frequencies_hz()[k]);
            rhs[0][k] = Complex64::new(2.0, omega * 0.03) * expected[k];
            rhs[1][k] = expected[grid.len() + k];
            for (m, input) in grid.indices().iter().enumerate() {
                let delta: Vec<_> = out.iter().zip(input).map(|(a, b)| a - b).collect();
                if let Some(coefficient) = squared.get(&delta) {
                    rhs[0][k] += Complex64::new(0.3, omega * 0.07) * coefficient * expected[m];
                }
            }
        }
        for method in [
            QuasiPeriodicLinearMethod::Direct,
            QuasiPeriodicLinearMethod::Krylov,
        ] {
            let mut config = QuasiPeriodicAcConfig::default();
            config.linear.method = method;
            config.linear.relative_tolerance = 1e-12;
            let mut circuit = Polynomial { singular: false };
            let mut work = Linearization::prepare(
                &mut circuit,
                grid.clone(),
                &orbit,
                &config,
                &ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap();
            let result = work.solve(&circuit, offset, &rhs, &NoAbort).unwrap();
            assert!(result.normalized_residual <= 1.0);
            for (a, b) in result.spectra.iter().flatten().zip(&expected) {
                assert!(
                    (*a - *b).norm() < 2e-11,
                    "offset={offset} method={method:?}: {a:?} != {b:?}"
                );
            }
        }
    }
}

#[test]
fn qpac_refuses_singular_unexcited_operator_and_honors_abort_and_budget() {
    let grid = basis();
    let orbit = orbit(&grid);
    let sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
    for method in [
        QuasiPeriodicLinearMethod::Direct,
        QuasiPeriodicLinearMethod::Krylov,
    ] {
        let mut config = QuasiPeriodicAcConfig::default();
        config.linear.method = method;
        let mut circuit = Polynomial { singular: true };
        let mut work = Linearization::prepare(
            &mut circuit,
            grid.clone(),
            &orbit,
            &config,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        assert!(work.solve(&circuit, 0.3, &sources, &NoAbort).is_err());
        let abort = CountingAbort::new(2);
        assert!(matches!(
            work.solve(&circuit, 0.3, &sources, &abort),
            Err(Error::Aborted)
        ));
        let limits = ResourceLimits {
            max_result_values: 10,
            ..Default::default()
        };
        assert!(matches!(
            Linearization::prepare(
                &mut circuit,
                grid.clone(),
                &orbit,
                &config,
                &limits,
                &NoAbort
            ),
            Err(Error::ResourceLimit(_))
        ));
    }
}
