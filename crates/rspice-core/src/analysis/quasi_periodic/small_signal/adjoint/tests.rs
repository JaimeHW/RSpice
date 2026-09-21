//! Independent nonsymmetric polynomial-convolution oracles for QPXF adjoints.
use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicLinearMethod, QuasiPeriodicSampling, solve::Sample,
};
use std::collections::BTreeMap;

const G: [[f64; 2]; 2] = [[0.3, 0.1], [-0.2, 0.4]];
const C: [[f64; 2]; 2] = [[0.07, -0.02], [0.015, 0.04]];

struct Coupled {
    singular: bool,
}
impl Circuit for Coupled {
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
        let jw = std::f64::consts::TAU * frequency;
        Ok(vec![
            (0, 0, Complex64::new(2.0, jw * 0.01)),
            (0, 1, Complex64::new(-0.7, jw * 0.002)),
            (1, 0, Complex64::new(0.4, -jw * 0.003)),
            (1, 1, Complex64::new(1.7, jw * 0.02)),
        ])
    }
    fn sample(&mut self, state: &[Value], _: bool) -> Result<Sample, Error> {
        let mut sample = Sample::default();
        for r in 0..2 {
            for c in 0..2 {
                sample.conductance.push((
                    r,
                    c,
                    if self.singular {
                        1.0 + state[1].powi(2)
                    } else {
                        G[r][c] * state[1].powi(2)
                    },
                ));
                if !self.singular {
                    sample.capacitance.push((r, c, C[r][c] * state[1].powi(2)));
                }
            }
        }
        Ok(sample)
    }
}

fn fixture() -> (Arc<QuasiPeriodicGrid>, Vec<Vec<Complex64>>) {
    let mut config = QuasiPeriodicGridConfig::new(vec![1.0, std::f64::consts::SQRT_2], vec![1, 1]);
    config.max_mixing_order = Some(1);
    config.sampling = QuasiPeriodicSampling::Exact(vec![9, 11]);
    let grid = Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    );
    let mut orbit = vec![vec![Complex64::ZERO; grid.len()]; 2];
    for (tuple, value) in [
        ([1, 0], Complex64::new(0.3, 0.1)),
        ([0, 1], Complex64::new(0.15, -0.23)),
    ] {
        let index = grid.index_of(&tuple).unwrap();
        orbit[1][index] = value;
        orbit[1][grid.len() - 1 - index] = value.conj();
    }
    (grid, orbit)
}

fn oracle(
    grid: &QuasiPeriodicGrid,
    orbit: &[Vec<Complex64>],
    offset: Value,
) -> Vec<Vec<Complex64>> {
    let n = grid.len();
    let mut square: BTreeMap<Vec<i32>, Complex64> = BTreeMap::new();
    for (i, a) in grid.indices().iter().enumerate() {
        for (j, b) in grid.indices().iter().enumerate() {
            *square
                .entry(a.iter().zip(b).map(|(a, b)| a + b).collect())
                .or_default() += orbit[1][i] * orbit[1][j];
        }
    }
    assert!(!grid.indices().contains(&vec![2, 0]));
    assert!(square[&vec![2, 0]].norm() > 0.05);
    let mut matrix = vec![vec![Complex64::ZERO; 2 * n]; 2 * n];
    for (k, out) in grid.indices().iter().enumerate() {
        let f = offset + grid.frequencies_hz()[k];
        for (r, c, value) in (Coupled { singular: false }).linear_entries(f).unwrap() {
            matrix[r * n + k][c * n + k] += value;
        }
        for (l, input) in grid.indices().iter().enumerate() {
            let delta: Vec<_> = out.iter().zip(input).map(|(a, b)| a - b).collect();
            if let Some(value) = square.get(&delta) {
                for r in 0..2 {
                    for c in 0..2 {
                        matrix[r * n + k][c * n + l] +=
                            Complex64::new(G[r][c], std::f64::consts::TAU * f * C[r][c]) * value;
                    }
                }
            }
        }
    }
    matrix
}

#[test]
fn qpxf_adjoint_equilibrates_integral_units_without_changing_transfer() {
    struct Integral {
        rate: Value,
    }
    impl Circuit for Integral {
        fn unknowns(&self) -> usize {
            2
        }
        fn voltage_equation(&self, row: usize) -> bool {
            row == 0
        }
        fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
            Ok(vec![(0, 0, Complex64::ONE)])
        }
        fn sample(&mut self, _: &[Value], _: bool) -> Result<Sample, Error> {
            // v = rate*z; dz/dt = source-v, so v/source = rate/(rate+jw).
            Ok(Sample {
                conductance: vec![(0, 1, -self.rate), (1, 0, 1.0)],
                capacitance: vec![(1, 1, 1.0)],
                ..Default::default()
            })
        }
    }
    let (grid, _) = fixture();
    let orbit = vec![vec![Complex64::ZERO; grid.len()]; 2];
    let mut observation = orbit.clone();
    observation[0][grid.dc_index()] = Complex64::ONE;
    for (rate, method) in [
        (1e3, QuasiPeriodicLinearMethod::Direct),
        (1e9, QuasiPeriodicLinearMethod::Krylov),
    ] {
        let mut circuit = Integral { rate };
        let config = QuasiPeriodicLinearConfig {
            method,
            ..Default::default()
        };
        let mut work = Linearization::prepare_adjoint(
            &mut circuit,
            grid.clone(),
            &orbit,
            &config,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        let offset = 0.13 * rate;
        let result = work
            .solve_adjoint(&circuit, offset, &observation, &NoAbort)
            .unwrap();
        let expected = rate / Complex64::new(rate, -std::f64::consts::TAU * offset);
        assert!(result.normalized_residual <= 1.0);
        for k in 0..grid.len() {
            let (a, b) = if k == grid.dc_index() {
                (1.0 - expected, expected)
            } else {
                (Complex64::ZERO, Complex64::ZERO)
            };
            assert!((result.sensitivities[0][k] - a).norm() < 1e-10);
            assert!((result.sensitivities[1][k] - b).norm() < 1e-10);
        }
    }
}

#[test]
fn qpxf_adjoint_matches_independent_nonsymmetric_complex_convolution_and_duality() {
    let (grid, orbit) = fixture();
    let n = 2 * grid.len();
    let expected: Vec<_> = (0..n)
        .map(|i| Complex64::new(0.11 + i as f64 * 0.03, -0.2 + i as f64 * 0.02))
        .collect();
    let forward: Vec<_> = (0..n)
        .map(|i| Complex64::new(-0.3 + i as f64 * 0.01, 0.2 - i as f64 * 0.04))
        .collect();
    for offset in [0.0, 0.37, -1.0] {
        let a = oracle(&grid, &orbit, offset);
        let c: Vec<_> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| a[j][i].conj() * expected[j])
                    .sum::<Complex64>()
            })
            .collect();
        let b: Vec<_> = a
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&forward)
                    .map(|(a, x)| *a * *x)
                    .sum::<Complex64>()
            })
            .collect();
        let reference: Complex64 = c.iter().zip(&forward).map(|(c, x)| c.conj() * x).sum();
        for method in [
            QuasiPeriodicLinearMethod::Direct,
            QuasiPeriodicLinearMethod::Krylov,
        ] {
            let mut circuit = Coupled { singular: false };
            let linear = QuasiPeriodicLinearConfig {
                method,
                relative_tolerance: 1e-12,
                ..Default::default()
            };
            let mut work = Linearization::prepare_adjoint(
                &mut circuit,
                grid.clone(),
                &orbit,
                &linear,
                &ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap();
            let frequencies: Vec<_> = grid.frequencies_hz().iter().map(|f| *f + offset).collect();
            let entries: Vec<_> = frequencies
                .iter()
                .map(|f| circuit.linear_entries(*f).unwrap())
                .collect();
            let applied = work
                .apply(&frequencies, &entries, &expected, &NoAbort)
                .unwrap();
            for (actual, expected) in applied.iter().zip(&c) {
                assert!((*actual - *expected).norm() < 2e-12);
            }
            let observation: Vec<_> = c.chunks_exact(grid.len()).map(<[_]>::to_vec).collect();
            let result = work
                .solve_adjoint(&circuit, offset, &observation, &NoAbort)
                .unwrap();
            assert!(result.normalized_residual <= 1.0);
            for (actual, expected) in result.sensitivities.iter().flatten().zip(&expected) {
                assert!(
                    (*actual - *expected).norm() < 2e-11,
                    "offset={offset}, method={method:?}"
                );
            }
            let transfer: Complex64 = result
                .sensitivities
                .iter()
                .flatten()
                .zip(&b)
                .map(|(lambda, b)| lambda.conj() * b)
                .sum();
            assert!((transfer - reference).norm() < 2e-11);
        }
    }
}

#[test]
fn qpxf_adjoint_refuses_compatible_singular_systems_and_observes_abort_and_budget() {
    let (grid, orbit) = fixture();
    let mut observation = vec![vec![Complex64::ZERO; grid.len()]; 2];
    observation[0][grid.dc_index()] = Complex64::ONE;
    observation[1][grid.dc_index()] = Complex64::ONE;
    for method in [
        QuasiPeriodicLinearMethod::Direct,
        QuasiPeriodicLinearMethod::Krylov,
    ] {
        let linear = QuasiPeriodicLinearConfig {
            method,
            ..Default::default()
        };
        let mut circuit = Coupled { singular: true };
        let mut work = Linearization::prepare_adjoint(
            &mut circuit,
            grid.clone(),
            &orbit,
            &linear,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        assert!(
            work.solve_adjoint(&circuit, 0.3, &observation, &NoAbort)
                .is_err()
        );
        assert!(matches!(
            work.solve_adjoint(&circuit, 0.3, &observation, &CountingAbort::new(2)),
            Err(Error::Aborted)
        ));
        let limits = ResourceLimits {
            max_result_values: 10,
            ..Default::default()
        };
        assert!(matches!(
            Linearization::prepare_adjoint(
                &mut circuit,
                grid.clone(),
                &orbit,
                &linear,
                &limits,
                &NoAbort
            ),
            Err(Error::ResourceLimit(_))
        ));
    }
}
