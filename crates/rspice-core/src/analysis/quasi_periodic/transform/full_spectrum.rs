//! Full real trigonometric interpolation, independent of the circuit's retained modes.
use super::*;
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};

/// Real-waveform Fourier coefficients on a full signed sample lattice.
#[derive(Debug, Clone, PartialEq)]
pub struct QuasiPeriodicSampleSpectrum {
    pub tuples: Vec<Vec<i32>>,
    pub coefficients: Vec<Complex64>,
}

impl QuasiPeriodicTransform {
    /// Fold a full signed interpolant back onto the collocation grid. Nyquist
    /// partners share bins and must be added, not overwritten. This keeps
    /// prescribed-state preparation linearithmic in the sample count.
    pub(crate) fn complete_real_samples_with_abort(
        &mut self,
        spectrum: &QuasiPeriodicSampleSpectrum,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        check_abort(abort)?;
        if spectrum.tuples.len() != spectrum.coefficients.len() {
            return Err(Error::InvalidConfig(
                "complete spectrum has mismatched coordinates".into(),
            ));
        }
        let mut values = zero_buffer(self.grid.sample_count())?;
        for (i, (tuple, coefficient)) in spectrum
            .tuples
            .iter()
            .zip(&spectrum.coefficients)
            .enumerate()
        {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if tuple.len() != self.grid.dimensions().len() || !finite(*coefficient) {
                return Err(Error::InvalidConfig(
                    "complete spectrum has invalid tone coordinates or coefficients".into(),
                ));
            }
            let mut bin = 0;
            let mut stride = 1;
            for (&k, &size) in tuple.iter().zip(self.grid.dimensions()) {
                if k.unsigned_abs() as usize > size / 2 {
                    return Err(Error::InvalidConfig(
                        "complete spectrum exceeds its collocation grid".into(),
                    ));
                }
                bin += k.rem_euclid(size as i32) as usize * stride;
                stride *= size;
            }
            values[bin] += coefficient;
        }
        self.transform(&mut values, true, abort)?;
        let scale = values.iter().map(|v| v.re.abs()).fold(0.0_f64, Value::max);
        if values
            .iter()
            .any(|v| !finite(*v) || v.im.abs() > 128.0 * Value::EPSILON * scale)
        {
            return Err(Error::Numerical(
                "complete interpolant does not produce finite real phase samples".into(),
            ));
        }
        Ok(values.into_iter().map(|v| v.re).collect())
    }

    /// Keep every collocation bin, including harmonics above the circuit basis.
    /// An even dimension's Nyquist bin is shared equally between +/-N/2;
    /// this gives the real cosine interpolant, applied along each tone axis.
    /// See https://docs.scipy.org/doc/scipy/reference/generated/scipy.signal.resample.html
    /// for the corresponding treatment of unpaired Fourier bins. Resolution
    /// remains the caller's phase-grid choice; interpolation does not certify
    /// convergence of the physical noise law beyond that sampling resolution.
    pub fn to_complete_real_spectrum_with_abort(
        &mut self,
        samples: &[Value],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicSampleSpectrum, Error> {
        check_abort(abort)?;
        if samples.len() != self.grid.sample_count() {
            return Err(Error::InvalidConfig(
                "complete real transform differs from its phase sample count".into(),
            ));
        }
        let dimensions = self.grid.dimensions();
        let widths: Vec<_> = dimensions.iter().map(|n| 2 * (n / 2) + 1).collect();
        let count = widths.iter().fold(1usize, |a, n| a.saturating_mul(*n));
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            count,
            limits.max_analysis_points,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            count
                .saturating_mul(dimensions.len() + 2)
                .saturating_add(samples.len().saturating_mul(8)),
            limits.max_result_values.min(32_000_000),
        )?;
        let mut values = zero_buffer(samples.len())?;
        for (i, (value, sample)) in values.iter_mut().zip(samples).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if !sample.is_finite() {
                return Err(Error::Numerical(
                    "complete real transform has a nonfinite sample".into(),
                ));
            }
            *value = Complex64::new(*sample, 0.0);
        }
        self.transform(&mut values, false, abort)?;
        let dimensions = self.grid.dimensions();
        let mut tuples = Vec::with_capacity(count);
        let mut coefficients = Vec::with_capacity(count);
        for index in 0..count {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let mut remaining = index;
            let mut tuple = vec![0; dimensions.len()];
            for d in (0..dimensions.len()).rev() {
                tuple[d] = (remaining % widths[d]) as i32 - (dimensions[d] / 2) as i32;
                remaining /= widths[d];
            }
            let mut bin = 0usize;
            let mut stride = 1usize;
            let mut nyquist_axes = 0;
            for (d, &k) in tuple.iter().enumerate() {
                let n = dimensions[d];
                // Collocation storage has the FIRST authored tone fastest.
                bin += (k.rem_euclid(n as i32) as usize) * stride;
                stride *= n;
                if n.is_multiple_of(2) && k.unsigned_abs() as usize == n / 2 {
                    nyquist_axes += 1;
                }
            }
            let value = values[bin];
            let scale = |v| {
                crate::numerics::scaled_noise::scale_complex_component_exactly(v, -nyquist_axes)
                    .map_err(|error| {
                        Error::Numerical(format!("complete real Fourier coefficient: {error}"))
                    })
            };
            coefficients.push(Complex64::new(scale(value.re)?, scale(value.im)?));
            tuples.push(tuple);
        }
        // Input is exactly real: use one member of each Fourier pair to make
        // that physical symmetry exact, without treating FFT roundoff as an
        // independent complex modulation. No small real coefficient is pruned.
        for i in 0..count / 2 {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            coefficients[count - 1 - i] = coefficients[i].conj();
        }
        coefficients[count / 2].im = 0.0;
        Ok(QuasiPeriodicSampleSpectrum {
            tuples,
            coefficients,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        abort_signal::{CountingAbort, NoAbort},
        analysis::quasi_periodic::{QuasiPeriodicGridConfig, QuasiPeriodicSampling},
    };
    #[test]
    fn qpnoise_complete_real_spectrum_preserves_higher_modes_and_multiaxis_nyquist() {
        for dimensions in [vec![4, 6], vec![5, 4, 3]] {
            let tones = dimensions.len();
            let mut config = QuasiPeriodicGridConfig::new(
                vec![1.0, 1.414, 3.14][..tones].to_vec(),
                vec![1; tones],
            );
            config.sampling = QuasiPeriodicSampling::Exact(dimensions.clone());
            let grid = Arc::new(
                QuasiPeriodicGrid::new_with_abort(config, &ResourceLimits::default(), &NoAbort)
                    .unwrap(),
            );
            let wave = |p: &[f64]| {
                if tones == 2 {
                    0.5 * (2.0 * p[0]).cos()
                        + 0.3 * p[1].sin() * (2.0 * p[0]).cos()
                        + 0.7 * (2.0 * p[1]).cos()
                        + 0.9 * (2.0 * p[0]).cos() * (3.0 * p[1]).cos()
                } else {
                    1.0 + 0.6 * (2.0 * p[0]).cos() * (2.0 * p[1]).cos() + 0.3 * p[2].sin()
                }
            };
            let samples: Vec<_> = (0..grid.sample_count())
                .map(|s| wave(&grid.phases(s).unwrap()))
                .collect();
            let mut transform =
                QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap();
            let spectrum = transform
                .to_complete_real_spectrum_with_abort(
                    &samples,
                    &ResourceLimits::default(),
                    &NoAbort,
                )
                .unwrap();
            let restored = transform
                .complete_real_samples_with_abort(&spectrum, &NoAbort)
                .unwrap();
            for (actual, expected) in restored.iter().zip(&samples) {
                assert!((actual - expected).abs() < 3e-14);
            }
            assert!(
                spectrum
                    .tuples
                    .iter()
                    .zip(&spectrum.coefficients)
                    .any(|(tuple, value)| grid.index_of(tuple).is_none() && value.norm() > 0.01)
            );
            for i in 0..31 {
                let phases =
                    vec![0.173 * i as f64, 0.321 * i as f64, 0.117 * i as f64][..tones].to_vec();
                let reconstructed: Complex64 = spectrum
                    .tuples
                    .iter()
                    .zip(&spectrum.coefficients)
                    .map(|(tuple, a)| {
                        *a * Complex64::from_polar(
                            1.0,
                            tuple.iter().zip(&phases).map(|(k, p)| *k as f64 * p).sum(),
                        )
                    })
                    .sum();
                assert!((reconstructed - Complex64::new(wave(&phases), 0.0)).norm() < 3e-14);
            }
            let abort = CountingAbort::new(2);
            assert!(matches!(
                transform.to_complete_real_spectrum_with_abort(
                    &samples,
                    &ResourceLimits::default(),
                    &abort
                ),
                Err(Error::Aborted)
            ));
            assert_eq!(abort.count(), 3);
            let limits = ResourceLimits {
                max_result_values: 1,
                ..ResourceLimits::default()
            };
            assert!(matches!(
                transform.to_complete_real_spectrum_with_abort(&samples, &limits, &NoAbort),
                Err(Error::ResourceLimit(_))
            ));
        }
    }
}
