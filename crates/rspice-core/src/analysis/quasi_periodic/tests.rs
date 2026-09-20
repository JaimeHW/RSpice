//! Independent Fourier-series and nonlinear-mixing oracles for the torus basis.
use super::*;
use crate::abort_signal::{CountingAbort, NoAbort};
use crate::{ResourceLimits, Value};
use std::sync::Arc;

fn grid(harmonics: Vec<usize>) -> Arc<QuasiPeriodicGrid> {
    Arc::new(
        QuasiPeriodicGrid::new_with_abort(
            QuasiPeriodicGridConfig::new(vec![1e6, std::f64::consts::SQRT_2 * 1e6], harmonics),
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
    )
}

fn add_cosine(
    grid: &QuasiPeriodicGrid,
    spectrum: &mut [Complex64],
    tuple: &[i32],
    amplitude: f64,
    phase: f64,
) {
    let positive = grid.index_of(tuple).unwrap();
    let negative = grid
        .index_of(&tuple.iter().map(|k| -*k).collect::<Vec<_>>())
        .unwrap();
    spectrum[positive] += Complex64::from_polar(amplitude * 0.5, phase);
    spectrum[negative] += Complex64::from_polar(amplitude * 0.5, -phase);
}

#[test]
fn torus_transform_matches_independent_real_series_and_physical_derivative() {
    let grid = grid(vec![2, 3]);
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let mut coefficients = vec![Complex64::ZERO; grid.len()];
    coefficients[grid.dc_index()] = Complex64::new(0.75, 0.0);
    add_cosine(&grid, &mut coefficients, &[1, 0], 1.2, 0.3);
    add_cosine(&grid, &mut coefficients, &[0, 2], 0.7, -0.5);
    add_cosine(&grid, &mut coefficients, &[1, -1], 0.25, 0.2);
    let samples = transform
        .to_real_samples_with_abort(&coefficients, &NoAbort)
        .unwrap();
    let derivative = grid
        .differentiate_with_abort(&coefficients, &NoAbort)
        .unwrap();
    let slopes = transform
        .to_real_samples_with_abort(&derivative, &NoAbort)
        .unwrap();
    let [f1, f2] = grid.config().frequencies_hz.as_slice() else {
        unreachable!()
    };
    for (i, actual) in samples.iter().enumerate() {
        let phase = grid.phases(i).unwrap();
        let expected = 0.75
            + 1.2 * (phase[0] + 0.3).cos()
            + 0.7 * (2.0 * phase[1] - 0.5).cos()
            + 0.25 * (phase[0] - phase[1] + 0.2).cos();
        let slope = -std::f64::consts::TAU
            * (1.2 * f1 * (phase[0] + 0.3).sin()
                + 1.4 * f2 * (2.0 * phase[1] - 0.5).sin()
                + 0.25 * (f1 - f2) * (phase[0] - phase[1] + 0.2).sin());
        assert!((actual - expected).abs() < 2e-14);
        assert!((slopes[i] - slope).abs() < 1e-7);
    }
    let complex = samples
        .iter()
        .map(|x| Complex64::new(*x, 0.0))
        .collect::<Vec<_>>();
    let restored = transform
        .to_spectrum_with_abort(&complex, &NoAbort)
        .unwrap();
    for (actual, expected) in restored.iter().zip(&coefficients) {
        assert!((actual - expected).norm() < 2e-14);
    }
    let spectral_power = coefficients.iter().map(|c| c.norm_sqr()).sum::<Value>();
    let mean_square = samples.iter().map(|x| x * x).sum::<Value>() / samples.len() as Value;
    assert!((mean_square - spectral_power).abs() < 2e-14);
    assert!(grid.frequencies_hz()[grid.index_of(&[1, -1]).unwrap()] < 0.0);
}

#[test]
fn nonlinear_torus_projection_resolves_sum_difference_and_second_harmonics() {
    let grid = grid(vec![2, 2]);
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let samples = (0..grid.sample_count())
        .map(|i| {
            let p = grid.phases(i).unwrap();
            Complex64::new((2.0 * p[0].cos() + 3.0 * p[1].cos()).powi(2), 0.0)
        })
        .collect::<Vec<_>>();
    let spectrum = transform
        .to_spectrum_with_abort(&samples, &NoAbort)
        .unwrap();
    for (slot, tuple) in grid.indices().iter().enumerate() {
        let expected = match tuple.as_slice() {
            [0, 0] => 6.5,
            [2, 0] | [-2, 0] => 1.0,
            [0, 2] | [0, -2] => 2.25,
            [1, 1] | [-1, -1] | [1, -1] | [-1, 1] => 3.0,
            _ => 0.0,
        };
        assert!(
            (spectrum[slot] - Complex64::new(expected, 0.0)).norm() < 1e-13,
            "{tuple:?}: {:?}",
            spectrum[slot]
        );
    }
}

#[test]
fn three_tone_odd_grid_matches_direct_complex_fourier_sum() {
    let mut config =
        QuasiPeriodicGridConfig::new(vec![1.0, 2.0_f64.sqrt(), 3.0_f64.sqrt()], vec![1, 2, 1]);
    config.sampling = QuasiPeriodicSampling::Exact(vec![3, 5, 3]);
    let grid = Arc::new(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).unwrap(),
    );
    let spectrum = (0..grid.len())
        .map(|i| Complex64::new((i as f64 * 0.7).sin(), (i as f64 * 0.3).cos()))
        .collect::<Vec<_>>();
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    let samples = transform
        .to_samples_with_abort(&spectrum, &NoAbort)
        .unwrap();
    for (i, actual) in samples.iter().enumerate() {
        let phase = grid.phases(i).unwrap();
        let direct = grid
            .indices()
            .iter()
            .zip(&spectrum)
            .map(|(tuple, coefficient)| {
                let angle = tuple
                    .iter()
                    .zip(&phase)
                    .map(|(k, p)| f64::from(*k) * p)
                    .sum::<Value>();
                coefficient * Complex64::from_polar(1.0, angle)
            })
            .sum::<Complex64>();
        assert!((actual - direct).norm() < 2e-13);
    }
    let restored = transform
        .to_spectrum_with_abort(&samples, &NoAbort)
        .unwrap();
    for (actual, expected) in restored.iter().zip(spectrum) {
        assert!((actual - expected).norm() < 1e-13);
    }
}

#[test]
fn lattice_controls_preserve_axis_harmonics_and_reject_degenerate_frequencies() {
    let mut config = QuasiPeriodicGridConfig::new(vec![1.0, 2.0_f64.sqrt()], vec![3, 2]);
    config.max_mixing_order = Some(2);
    let json = serde_json::to_string(&config).unwrap();
    assert_eq!(config, serde_json::from_str(&json).unwrap());
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.clone(), &ResourceLimits::default(), &NoAbort)
            .unwrap();
    assert_eq!(grid.len(), 15);
    assert!(grid.index_of(&[3, 0]).is_some());
    assert!(grid.index_of(&[1, 1]).is_some());
    assert!(grid.index_of(&[2, 1]).is_none());
    assert!(grid.index_of(&[1]).is_none());
    for (i, tuple) in grid.indices().iter().enumerate() {
        assert_eq!(
            grid.indices()[grid.len() - 1 - i],
            tuple.iter().map(|k| -*k).collect::<Vec<_>>()
        );
    }
    config.frequencies_hz[1] = 2.0;
    assert!(
        QuasiPeriodicGrid::new_with_abort(config.clone(), &ResourceLimits::default(), &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("indistinguishable")
    );
    config.frequencies_hz[1] = f64::NAN;
    assert!(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort).is_err()
    );
}

#[test]
fn lattice_and_transform_enforce_shape_finiteness_resources_and_cancellation() {
    let config = QuasiPeriodicGridConfig::new(vec![1.0, 2.0_f64.sqrt()], vec![2, 2]);
    let mut limits = ResourceLimits::default();
    limits.max_analysis_points = 16;
    assert!(matches!(
        QuasiPeriodicGrid::new_with_abort(config.clone(), &limits, &NoAbort),
        Err(QuasiPeriodicError::ResourceLimit(_))
    ));
    let mut malformed = config.clone();
    malformed.sampling = QuasiPeriodicSampling::Exact(vec![4, 5]);
    assert!(
        QuasiPeriodicGrid::new_with_abort(malformed, &ResourceLimits::default(), &NoAbort).is_err()
    );
    let abort = CountingAbort::new(1);
    assert!(matches!(
        QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &abort),
        Err(QuasiPeriodicError::Aborted)
    ));
    let grid = grid(vec![2, 2]);
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    assert!(transform.to_samples_with_abort(&[], &NoAbort).is_err());
    assert!(
        transform
            .to_spectrum_with_abort(&[Complex64::ZERO], &NoAbort)
            .is_err()
    );
    let mut spectrum = vec![Complex64::ZERO; grid.len()];
    spectrum[grid.dc_index()].im = 1.0;
    assert!(
        transform
            .to_real_samples_with_abort(&spectrum, &NoAbort)
            .is_err()
    );
    spectrum[grid.dc_index()] = Complex64::new(f64::INFINITY, 0.0);
    assert!(
        transform
            .to_samples_with_abort(&spectrum, &NoAbort)
            .is_err()
    );
    let abort = CountingAbort::new(4);
    assert!(matches!(
        transform.to_samples_with_abort(&vec![Complex64::ONE; grid.len()], &abort),
        Err(QuasiPeriodicError::Aborted)
    ));
    assert_eq!(abort.polls_after_abort(), 0);
}

#[test]
fn torus_fft_preserves_representable_extreme_constant_fields() {
    let grid = grid(vec![1, 1]);
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
    for value in [f64::from_bits(3), 1e-300, f64::MAX / 8.0] {
        let samples = vec![Complex64::new(value, 0.0); grid.sample_count()];
        let spectrum = transform
            .to_spectrum_with_abort(&samples, &NoAbort)
            .unwrap();
        assert!((spectrum[grid.dc_index()].re / value - 1.0).abs() < 1e-14);
        let restored = transform
            .to_samples_with_abort(&spectrum, &NoAbort)
            .unwrap();
        assert!(
            restored
                .iter()
                .all(|actual| (actual.re / value - 1.0).abs() < 1e-14)
        );
    }
}
