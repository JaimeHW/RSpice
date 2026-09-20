//! Analytic covariance and independent signed-convolution noise oracles.
use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicLinearConfig, QuasiPeriodicSampling,
    small_signal::Linearization,
    solve::{Circuit, LinearEntry, Sample},
};
use std::collections::BTreeMap;

fn grid(tones: usize) -> Arc<QuasiPeriodicGrid> {
    let frequencies = vec![2.0, std::f64::consts::SQRT_2, std::f64::consts::PI][..tones].to_vec();
    let mut config = QuasiPeriodicGridConfig::new(frequencies, vec![1; tones]);
    config.max_mixing_order = Some(1);
    config.sampling = QuasiPeriodicSampling::Exact(vec![7; tones]);
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    )
}
fn adjoint(grid: &QuasiPeriodicGrid, rows: usize) -> QuasiPeriodicAdjointSolution {
    QuasiPeriodicAdjointSolution {
        frequency_hz: 0.37,
        frequency_lattice: vec![0; grid.dimensions().len()],
        sensitivities: vec![vec![Complex64::ZERO; grid.len()]; rows],
        normalized_residual: 0.0,
    }
}
fn projector(grid: &Arc<QuasiPeriodicGrid>, rows: usize) -> QuasiPeriodicNoiseProjector {
    QuasiPeriodicNoiseProjector::new_with_abort(
        grid.clone(),
        rows,
        grid.indices(),
        &ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap()
}
fn source(spectrum: QuasiPeriodicNoiseSpectrum) -> QuasiPeriodicNoiseSource {
    QuasiPeriodicNoiseSource {
        name: "physical mechanism".into(),
        injections: vec![(0, Complex64::ONE)],
        spectrum,
    }
}
fn white(q: f64, scale: i32) -> QuasiPeriodicNoiseSpectrum {
    QuasiPeriodicNoiseSpectrum::White {
        density: vec![q],
        binary_scale_exponent: scale,
    }
}
fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() <= 3e-12 * expected.norm().max(1e-300),
        "{actual:?} != {expected:?}"
    );
}

struct Rc;
impl Circuit for Rc {
    fn unknowns(&self) -> usize {
        1
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, f: f64) -> Result<Vec<LinearEntry>, Error> {
        Ok(vec![(
            0,
            0,
            Complex64::new(0.001, std::f64::consts::TAU * f * 2e-6),
        )])
    }
    fn sample(&mut self, _: &[f64], _: bool) -> Result<Sample, Error> {
        Ok(Sample::default())
    }
}

#[test]
fn qpnoise_stationary_rc_matches_johnson_noise_through_certified_adjoint() {
    let grid = grid(2);
    let orbit = vec![vec![Complex64::ZERO; grid.len()]];
    let mut rc = Rc;
    let mut work = Linearization::prepare_adjoint(
        &mut rc,
        grid.clone(),
        &orbit,
        &QuasiPeriodicLinearConfig::default(),
        &ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    let mut observation = orbit;
    observation[0][grid.dc_index()] = Complex64::ONE;
    let solution = work
        .solve_adjoint_at_frequency(&rc, 1000.0, &[0, 0], &observation, &NoAbort)
        .unwrap();
    assert!(solution.normalized_residual <= 1.0);
    let q = 4.0 * 1.380649e-23 * 300.0 * 0.001;
    let actual = projector(&grid, 1)
        .source_covariance_with_abort(&[solution], &source(white(q, 0)), &NoAbort)
        .unwrap();
    let expected = q / Complex64::new(0.001, std::f64::consts::TAU * 1000.0 * 2e-6).norm_sqr();
    close(actual.values[0], Complex64::new(expected, 0.0));
}

#[test]
fn qpnoise_white_retains_multitone_and_shared_injection_cross_correlations() {
    for tones in [2, 3] {
        let grid = grid(tones);
        let mut one = adjoint(&grid, 2);
        let mut two = one.clone();
        let dc = grid.dc_index();
        let mut tuple = vec![0; tones];
        tuple[0] = 1;
        let plus = grid.index_of(&tuple).unwrap();
        // b=(1,-1): common stochastic injection paths subtract BEFORE power.
        one.sensitivities[0][dc] = Complex64::new(3.0, 0.0);
        one.sensitivities[1][dc] = Complex64::new(2.0, 0.0);
        one.sensitivities[0][plus] = Complex64::new(0.0, 1.0);
        two.sensitivities[0][dc] = Complex64::new(2.0, 0.0);
        two.sensitivities[0][plus] = Complex64::new(1.0, -1.0);
        // q=2+cos(theta_0): Q0=2 and Q(+/-1)=1/2. Independent
        // Fourier covariance oracle: Q(k-l), not phase sampling or FFT.
        let gains = [
            [Complex64::ONE, Complex64::new(0.0, 1.0)],
            [Complex64::new(2.0, 0.0), Complex64::new(1.0, -1.0)],
        ];
        let density = (0..grid.sample_count())
            .map(|s| 2.0 + grid.phases(s).unwrap()[0].cos())
            .collect();
        let mut source = source(QuasiPeriodicNoiseSpectrum::White {
            density,
            binary_scale_exponent: 0,
        });
        source.injections.push((1, -Complex64::ONE));
        let actual = projector(&grid, 2)
            .source_covariance_with_abort(&[one, two], &source, &NoAbort)
            .unwrap();
        for r in 0..2 {
            for c in 0..2 {
                let mut expected = Complex64::ZERO;
                for k in 0..2 {
                    for l in 0..2 {
                        expected +=
                            gains[r][k].conj() * gains[c][l] * if k == l { 2.0 } else { 0.5 };
                    }
                }
                close(actual.values[2 * r + c], expected);
            }
        }
        assert!(actual.values[1].im.abs() > 1.0);
        assert_eq!(actual.values[2], actual.values[1].conj());
        assert!(actual.values[1].norm_sqr() <= actual.values[0].re * actual.values[3].re);
    }
}

#[test]
fn qpnoise_colored_matches_noncyclic_convolution_including_outside_window() {
    let grid = grid(3);
    let mut one = adjoint(&grid, 1);
    let mut two = one.clone();
    let mut modulation = vec![Complex64::ZERO; grid.len()];
    modulation[grid.dc_index()] = Complex64::new(0.4, 0.0);
    for (tuple, amplitude) in [
        (vec![1, 0, 0], Complex64::new(0.2, 0.3)),
        (vec![0, 1, 0], Complex64::new(-0.15, 0.1)),
    ] {
        let k = grid.index_of(&tuple).unwrap();
        modulation[k] = amplitude;
        modulation[grid.len() - 1 - k] = amplitude.conj();
    }
    for k in 0..grid.len() {
        one.sensitivities[0][k] = Complex64::new(0.2 + k as f64 * 0.03, -0.1);
        two.sensitivities[0][k] = Complex64::new(-0.1, 0.3 - k as f64 * 0.02);
    }
    // Independently form FORWARD transfers t_m=sum_k conj(lambda_k)*a_(k-m).
    let solutions = [one, two];
    let mut transfers: BTreeMap<Vec<i32>, [Complex64; 2]> = BTreeMap::new();
    for k in 0..grid.len() {
        for j in 0..grid.len() {
            let m: Vec<_> = grid.indices()[k]
                .iter()
                .zip(&grid.indices()[j])
                .map(|(k, j)| k - j)
                .collect();
            let transfer = transfers.entry(m).or_default();
            for r in 0..2 {
                transfer[r] += solutions[r].sensitivities[0][k].conj() * modulation[j];
            }
        }
    }
    assert!(
        transfers
            .iter()
            .any(|(m, t)| grid.index_of(m).is_none() && t[0].norm() > 0.01)
    );
    let law = QuasiPeriodicNoiseSpectrum::PowerLaw {
        coefficient: 0.7,
        exponent: 1.3,
        modulation,
        binary_scale_exponent: 0,
    };
    let actual = projector(&grid, 1)
        .source_covariance_with_abort(&solutions, &source(law), &NoAbort)
        .unwrap();
    for r in 0..2 {
        for c in 0..2 {
            let expected = transfers
                .iter()
                .map(|(m, t)| {
                    let f = 0.37
                        + m.iter()
                            .zip(&grid.config().frequencies_hz)
                            .map(|(m, f)| *m as f64 * f)
                            .sum::<f64>();
                    t[r] * t[c].conj() * (0.7 / f.abs().powf(1.3))
                })
                .sum();
            close(actual.values[2 * r + c], expected);
        }
    }
}

#[test]
fn qpnoise_colored_preserves_signed_cancellation_and_authored_low_frequency() {
    let mut config = QuasiPeriodicGridConfig::new(vec![1e20, std::f64::consts::SQRT_2], vec![1, 1]);
    config.max_mixing_order = Some(1);
    let grid = Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    );
    let mut one = adjoint(&grid, 1);
    let dc = grid.dc_index();
    let carrier = grid.index_of(&[1, 0]).unwrap();
    one.frequency_hz = 0.125;
    one.frequency_lattice = vec![1, 0];
    one.sensitivities[0][carrier] = Complex64::ONE;
    let mut modulation = vec![Complex64::ZERO; grid.len()];
    modulation[dc] = Complex64::ONE;
    let source = source(QuasiPeriodicNoiseSpectrum::PowerLaw {
        coefficient: 1.0,
        exponent: 1.0,
        modulation,
        binary_scale_exponent: 0,
    });
    let actual = projector(&grid, 1)
        .source_covariance_with_abort(&[one.clone()], &source, &NoAbort)
        .unwrap();
    close(actual.values[0], Complex64::new(8.0, 0.0)); // retain .125 beside 1e20

    // a=2 cos(theta), g=2j sin(theta). Stationary DC paths cancel exactly;
    // only m=+/-2 remains. Summing individual powers would see singular 1/0.
    one.frequency_hz = 0.0;
    one.frequency_lattice = vec![0, 0];
    one.sensitivities[0][carrier] = Complex64::ONE;
    one.sensitivities[0][grid.len() - 1 - carrier] = -Complex64::ONE;
    let mut source = source;
    let QuasiPeriodicNoiseSpectrum::PowerLaw { modulation, .. } = &mut source.spectrum else {
        unreachable!()
    };
    modulation[dc] = Complex64::ZERO;
    modulation[carrier] = Complex64::ONE;
    modulation[grid.len() - 1 - carrier] = Complex64::ONE;
    let actual = projector(&grid, 1)
        .source_covariance_with_abort(&[one], &source, &NoAbort)
        .unwrap();
    close(actual.values[0], Complex64::new(1e-20, 0.0));
}

#[test]
fn qpnoise_retains_representable_extremes_and_refuses_unrepresentable_results() {
    let grid = grid(2);
    let mut one = adjoint(&grid, 1);
    let dc = grid.dc_index();
    let mut projector = projector(&grid, 1);
    for power in [-1000, 1000] {
        one.sensitivities[0][dc] = Complex64::new(libm::scalbn(1.0, power), 0.0);
        for colored in [false, true] {
            let spectrum = if colored {
                let mut modulation = vec![Complex64::ZERO; grid.len()];
                modulation[dc] = Complex64::ONE;
                QuasiPeriodicNoiseSpectrum::PowerLaw {
                    coefficient: 1.0,
                    exponent: 0.0,
                    modulation,
                    binary_scale_exponent: -2 * power,
                }
            } else {
                white(1.0, -2 * power)
            };
            let actual = projector
                .source_covariance_with_abort(&[one.clone()], &source(spectrum), &NoAbort)
                .unwrap();
            close(actual.values[0], Complex64::ONE);
        }
    }
    one.sensitivities[0][dc] = Complex64::ONE;
    let tiny = projector
        .source_covariance_with_abort(&[one.clone()], &source(white(1.0, -1074)), &NoAbort)
        .unwrap();
    assert_eq!(tiny.values[0].re.to_bits(), 1);
    for scale in [-1075, 1024] {
        assert!(
            projector
                .source_covariance_with_abort(&[one.clone()], &source(white(1.0, scale)), &NoAbort)
                .is_err()
        );
    }
}

#[test]
fn qpnoise_rejects_invalid_evidence_and_bounds_work_and_cancellation() {
    let grid = grid(2);
    let mut one = adjoint(&grid, 1);
    one.sensitivities[0][grid.dc_index()] = Complex64::ONE;
    let mut projector = projector(&grid, 1);
    let good = source(white(1.0, 0));
    let mut bad = good.clone();
    bad.spectrum = white(-1.0, 0);
    assert!(
        projector
            .source_covariance_with_abort(&[one.clone()], &bad, &NoAbort)
            .is_err()
    );
    let mut modulation = vec![Complex64::ZERO; grid.len()];
    modulation[0] = Complex64::ONE;
    bad.spectrum = QuasiPeriodicNoiseSpectrum::PowerLaw {
        coefficient: 1.0,
        exponent: 1.0,
        modulation,
        binary_scale_exponent: 0,
    };
    assert!(
        projector
            .source_covariance_with_abort(&[one.clone()], &bad, &NoAbort)
            .is_err()
    );
    bad = good.clone();
    bad.injections.push((0, Complex64::ONE));
    assert!(
        projector
            .source_covariance_with_abort(&[one.clone()], &bad, &NoAbort)
            .is_err()
    );
    let mut uncertified = one.clone();
    uncertified.normalized_residual = 1.1;
    assert!(
        projector
            .source_covariance_with_abort(&[uncertified], &good, &NoAbort)
            .is_err()
    );
    for tuples in [vec![vec![0, 0], vec![0, 0]], vec![vec![2, 0]]] {
        assert!(
            QuasiPeriodicNoiseProjector::new_with_abort(
                grid.clone(),
                1,
                &tuples,
                &ResourceLimits::default(),
                &NoAbort
            )
            .is_err()
        );
    }
    let limits = ResourceLimits {
        max_result_values: 1,
        ..ResourceLimits::default()
    };
    assert!(
        QuasiPeriodicNoiseProjector::new_with_abort(
            grid.clone(),
            1,
            grid.indices(),
            &limits,
            &NoAbort
        )
        .is_err()
    );
    for polls in [0, 8, 20] {
        let abort = CountingAbort::new(polls);
        assert!(matches!(
            projector.source_covariance_with_abort(&[one.clone()], &good, &abort),
            Err(Error::Aborted)
        ));
        assert_eq!(abort.count(), polls + 1);
    }
}
