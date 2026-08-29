//! The transformed record.

use rspice_core::numerics::rustfft_qualification::{
    MAX_QUALIFIED_RUSTFFT_LENGTH, RustfftQualificationError, qualify_rustfft_forward_length,
};
use rustfft::num_complex::Complex;

use super::{
    FftAllocationStage, FftBuildError, FftPoint, SpectrumNormalization,
    cache::{cached_fft_plan, cached_window},
    error::MIN_FFT_DATA_SAMPLES,
};
use crate::analysis::fft::window::WindowFunction;
// =============================================================================
// FFT Data
// =============================================================================

/// Complete FFT/spectrum data
#[derive(Debug, Clone)]
pub struct FftData {
    /// Name/label
    pub name: String,
    /// Spectrum points (positive frequencies only, DC to Nyquist)
    pub points: Vec<FftPoint>,
    /// Sample rate of original data
    pub sample_rate: f64,
    /// Number of points in original FFT
    pub fft_size: usize,
    /// Window function used
    pub window: WindowFunction,
    /// Amplitude normalization used for `points`.
    pub normalization: SpectrumNormalization,
    /// Exact finite-length equivalent noise bandwidth used for this record.
    pub(crate) equivalent_noise_bandwidth_bins: f64,
}

impl FftData {
    /// Create from uniformly sampled time-domain data using FFT.
    #[cfg(test)]
    pub fn from_time_domain(
        name: &str,
        data: &[f64],
        sample_rate: f64,
        window: WindowFunction,
    ) -> Result<Self, FftBuildError> {
        Self::from_time_domain_with_normalization(
            name,
            data,
            sample_rate,
            window,
            SpectrumNormalization::Peak,
        )
    }

    /// Create from uniformly sampled time-domain data using FFT.
    pub fn from_time_domain_with_normalization(
        name: &str,
        data: &[f64],
        sample_rate: f64,
        window: WindowFunction,
        normalization: SpectrumNormalization,
    ) -> Result<Self, FftBuildError> {
        let n = data.len();
        preflight_fft_build(n)?;
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err(FftBuildError::InvalidSampleRate { sample_rate });
        }
        validate_frequency_grid(n, sample_rate)?;

        let mut maximum_input_magnitude = 0.0_f64;
        for (index, &sample) in data.iter().enumerate() {
            if !sample.is_finite() {
                return Err(FftBuildError::NonFiniteInputSample {
                    index,
                    value: sample,
                });
            }
            maximum_input_magnitude = maximum_input_magnitude.max(sample.abs());
        }

        // The analytic zero path below does not need a backend plan. Every
        // nonzero record does, so reject an unsafe decomposition route before
        // allocating its name, window, output points, or transform buffers.
        // The plan cache repeats this qualification at its own planning seam.
        if maximum_input_magnitude != 0.0 {
            qualify_rustfft_forward_length(n)?;
        }

        let owned_name = try_owned_name(name)?;
        let window_entry = cached_window(window, n)?;
        if window_entry.coefficients.len() != n {
            return Err(FftBuildError::WindowLengthMismatch {
                expected: n,
                actual: window_entry.coefficients.len(),
            });
        }
        for (index, &coefficient) in window_entry.coefficients.iter().enumerate() {
            if !coefficient.is_finite() {
                return Err(FftBuildError::NonFiniteWindowCoefficient {
                    index,
                    value: coefficient,
                });
            }
        }

        // Coherent gain from actual generated coefficients, not a table constant.
        let cg = window_entry.coherent_gain;
        let equivalent_noise_bandwidth_bins = window_entry.equivalent_noise_bandwidth_bins;
        if !cg.is_finite()
            || cg <= 0.0
            || !equivalent_noise_bandwidth_bins.is_finite()
            || equivalent_noise_bandwidth_bins <= 0.0
        {
            return Err(FftBuildError::InvalidWindowCalibration {
                coherent_gain: cg,
                equivalent_noise_bandwidth_bins,
            });
        }

        // One-sided spectrum: include DC..Nyquist (Nyquist exists only for even n).
        // AC bins are amplitude-calibrated to peak values for FFT plot compatibility.
        let n_freqs = n / 2 + 1;
        let mut points = Vec::new();
        try_reserve_exact(&mut points, n_freqs, FftAllocationStage::SpectrumPoints)?;

        if maximum_input_magnitude == 0.0 {
            for index in 0..n_freqs {
                let frequency = frequency_at(index, n, sample_rate);
                points.push(FftPoint::from_complex(frequency, 0.0, 0.0));
            }
            return Ok(Self {
                name: owned_name,
                points,
                sample_rate,
                fft_size: n,
                window,
                normalization,
                equivalent_noise_bandwidth_bins,
            });
        }

        // Scaling the finite input into [-1, 1] bounds every unscaled FFT sum
        // before RustFFT. The original scale is restored only after division
        // by transform length and coherent gain, avoiding avoidable overflow.
        let mut buffer = Vec::new();
        try_reserve_exact(&mut buffer, n, FftAllocationStage::TransformBuffer)?;
        for (index, (&sample, &coefficient)) in data
            .iter()
            .zip(window_entry.coefficients.iter())
            .enumerate()
        {
            let normalized_sample = sample / maximum_input_magnitude;
            if sample != 0.0 && normalized_sample == 0.0 {
                return Err(FftBuildError::ErasedInputScale {
                    index,
                    value: sample,
                    scale: maximum_input_magnitude,
                });
            }
            let windowed_sample = normalized_sample * coefficient;
            if normalized_sample != 0.0 && coefficient != 0.0 && windowed_sample == 0.0 {
                return Err(FftBuildError::ErasedWindowedSample {
                    index,
                    normalized_sample,
                    coefficient,
                });
            }
            buffer.push(Complex::new(windowed_sample, 0.0));
        }

        // An exact zero record is resolved analytically above and never asks
        // RustFFT to allocate a plan. The nonzero route was qualified before
        // its first owned allocation; the cache checks again defensively.
        let fft = cached_fft_plan(n)?;
        if fft.len() != n || buffer.len() != n {
            return Err(FftBuildError::PlanInvariant {
                requested: n,
                plan_length: fft.len(),
                buffer_length: buffer.len(),
            });
        }
        let scratch_length = fft.get_inplace_scratch_len();
        let mut scratch = Vec::new();
        try_reserve_exact(
            &mut scratch,
            scratch_length,
            FftAllocationStage::TransformScratch,
        )?;
        scratch.resize(scratch_length, Complex::new(0.0, 0.0));
        fft.process_with_scratch(&mut buffer, &mut scratch);
        for (index, bin) in buffer.iter().enumerate() {
            if !bin.re.is_finite() || !bin.im.is_finite() {
                return Err(FftBuildError::NonFiniteTransformBin {
                    bin: index,
                    real: bin.re,
                    imaginary: bin.im,
                });
            }
        }

        let has_nyquist = n.is_multiple_of(2);
        let n_as_f64 = n as f64;

        for (k, bin) in buffer.iter().take(n_freqs).enumerate() {
            let frequency = frequency_at(k, n, sample_rate);
            let one_sided_scale = if k != 0 && !(has_nyquist && k == n / 2) {
                2.0
            } else {
                1.0
            };
            let normalization_scale = if one_sided_scale == 1.0 {
                1.0
            } else {
                normalization.scale_from_peak()
            };
            let normalized_magnitude = bin.re.hypot(bin.im);
            let numerator_scale = one_sided_scale * normalization_scale;
            let denominator_scale = n_as_f64 * cg;
            let magnitude = qualified_magnitude(
                k,
                normalized_magnitude,
                maximum_input_magnitude,
                numerator_scale,
                denominator_scale,
            )?;
            let phase = if magnitude == 0.0 {
                0.0
            } else {
                bin.im.atan2(bin.re)
            };
            if !phase.is_finite() {
                return Err(FftBuildError::NonFiniteSpectrumPoint {
                    bin: k,
                    frequency,
                    real: bin.re,
                    imaginary: bin.im,
                    magnitude,
                    phase,
                });
            }
            points.push(FftPoint {
                frequency,
                magnitude,
                phase,
            });
        }

        Ok(Self {
            name: owned_name,
            points,
            sample_rate,
            fft_size: n,
            window,
            normalization,
            equivalent_noise_bandwidth_bins,
        })
    }

    /// Create from magnitude/phase arrays
    #[cfg(test)]
    pub fn from_spectrum(
        name: &str,
        frequencies: &[f64],
        magnitudes: &[f64],
        phases: &[f64],
        sample_rate: f64,
    ) -> Self {
        Self::from_spectrum_with_normalization(
            name,
            frequencies,
            magnitudes,
            phases,
            sample_rate,
            SpectrumNormalization::Peak,
        )
    }

    /// Create from magnitude/phase arrays with explicit normalization metadata.
    #[cfg(test)]
    pub fn from_spectrum_with_normalization(
        name: &str,
        frequencies: &[f64],
        magnitudes: &[f64],
        phases: &[f64],
        sample_rate: f64,
        normalization: SpectrumNormalization,
    ) -> Self {
        let n = frequencies.len().min(magnitudes.len()).min(phases.len());
        let points: Vec<FftPoint> = (0..n)
            .map(|i| FftPoint::new(frequencies[i], magnitudes[i], phases[i]))
            .collect();
        let fft_size = n.saturating_sub(1).saturating_mul(2);

        Self {
            name: name.to_string(),
            points,
            sample_rate,
            fft_size,
            window: WindowFunction::Rectangular,
            normalization,
            equivalent_noise_bandwidth_bins: 1.0,
        }
    }

    /// Convert all spectrum magnitudes to a different normalization mode.
    pub fn convert_normalization(
        &mut self,
        target: SpectrumNormalization,
    ) -> Result<(), FftBuildError> {
        if self.normalization == target {
            return Ok(());
        }
        for (index, point) in self.points.iter().enumerate() {
            converted_magnitude(
                index,
                point.magnitude,
                self.fft_size,
                self.normalization,
                target,
            )?;
        }
        for (index, point) in self.points.iter_mut().enumerate() {
            point.magnitude = converted_magnitude(
                index,
                point.magnitude,
                self.fft_size,
                self.normalization,
                target,
            )?;
        }
        self.normalization = target;
        Ok(())
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Frequency resolution (bin width)
    pub fn frequency_resolution(&self) -> f64 {
        if self.fft_size > 0 {
            self.sample_rate / self.fft_size as f64
        } else {
            f64::NAN
        }
    }

    /// Equivalent noise bandwidth of the applied window, in FFT bins.
    ///
    /// The exact finite-length figure for the cached coefficients rather than
    /// a nominal table value. One bin for a rectangular window; more for
    /// every window that tapers, which is how much broadband noise each bin
    /// collects beyond its own width.
    pub fn equivalent_noise_bandwidth_bins(&self) -> f64 {
        self.equivalent_noise_bandwidth_bins
    }

    /// Equivalent-noise resolution bandwidth for the applied window.
    ///
    /// The FFT bin width multiplied by [`Self::equivalent_noise_bandwidth_bins`].
    pub fn resolution_bandwidth(&self) -> f64 {
        let bin_width = self.frequency_resolution();
        if !bin_width.is_finite() || bin_width <= 0.0 {
            return f64::NAN;
        }
        let bandwidth = bin_width * self.equivalent_noise_bandwidth_bins();
        if bandwidth.is_finite() && bandwidth > 0.0 {
            bandwidth
        } else {
            f64::NAN
        }
    }

    /// Nyquist frequency
    pub fn nyquist(&self) -> f64 {
        self.sample_rate / 2.0
    }

    /// Frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        Some((
            self.points.first()?.frequency,
            self.points.last()?.frequency,
        ))
    }
}

fn preflight_fft_build(length: usize) -> Result<(), FftBuildError> {
    if length < MIN_FFT_DATA_SAMPLES {
        return Err(FftBuildError::InsufficientSamples {
            length,
            minimum: MIN_FFT_DATA_SAMPLES,
        });
    }
    if length > MAX_QUALIFIED_RUSTFFT_LENGTH {
        return Err(FftBuildError::PlanningQualification(
            RustfftQualificationError::LengthLimit {
                length,
                limit: MAX_QUALIFIED_RUSTFFT_LENGTH,
            },
        ));
    }
    Ok(())
}

fn frequency_at(index: usize, length: usize, sample_rate: f64) -> f64 {
    sample_rate * (index as f64 / length as f64)
}

fn validate_frequency_grid(length: usize, sample_rate: f64) -> Result<(), FftBuildError> {
    let mut previous_frequency = 0.0;
    for bin in 1..=(length / 2) {
        let frequency = frequency_at(bin, length, sample_rate);
        if !frequency.is_finite() || frequency <= previous_frequency {
            return Err(FftBuildError::UnrepresentableFrequency {
                bin,
                previous_frequency,
                frequency,
            });
        }
        previous_frequency = frequency;
    }
    Ok(())
}

fn converted_magnitude(
    bin: usize,
    magnitude: f64,
    fft_size: usize,
    from: SpectrumNormalization,
    to: SpectrumNormalization,
) -> Result<f64, FftBuildError> {
    if !magnitude.is_finite() || magnitude < 0.0 {
        return Err(FftBuildError::UnrepresentableMagnitude {
            bin,
            normalized_magnitude: magnitude,
            source_scale: 1.0,
            numerator_scale: 1.0,
            denominator_scale: 1.0,
        });
    }
    let is_endpoint = bin == 0 || (fft_size.is_multiple_of(2) && bin == fft_size / 2);
    let scale = if is_endpoint {
        1.0
    } else {
        SpectrumNormalization::relative_scale(from, to)
    };
    qualified_magnitude(bin, magnitude, 1.0, scale, 1.0)
}

fn qualified_magnitude(
    bin: usize,
    normalized_magnitude: f64,
    source_scale: f64,
    numerator_scale: f64,
    denominator_scale: f64,
) -> Result<f64, FftBuildError> {
    if normalized_magnitude == 0.0 {
        return Ok(0.0);
    }
    positive_product_ratio(
        [normalized_magnitude, source_scale, numerator_scale],
        [denominator_scale],
    )
    .ok_or(FftBuildError::UnrepresentableMagnitude {
        bin,
        normalized_magnitude,
        source_scale,
        numerator_scale,
        denominator_scale,
    })
}

fn positive_product_ratio<const N: usize, const D: usize>(
    numerators: [f64; N],
    denominators: [f64; D],
) -> Option<f64> {
    let mut mantissa = 1.0;
    let mut exponent = 0_i32;
    for value in numerators {
        let (factor_mantissa, factor_exponent) = positive_binary_parts(value)?;
        mantissa *= factor_mantissa;
        exponent = exponent.checked_add(factor_exponent)?;
        normalize_binary_parts(&mut mantissa, &mut exponent)?;
    }
    for value in denominators {
        let (factor_mantissa, factor_exponent) = positive_binary_parts(value)?;
        mantissa /= factor_mantissa;
        exponent = exponent.checked_sub(factor_exponent)?;
        normalize_binary_parts(&mut mantissa, &mut exponent)?;
    }
    materialize_binary_parts(mantissa, exponent)
}

fn positive_binary_parts(value: f64) -> Option<(f64, i32)> {
    const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
    const EXPONENT_ONE: u64 = 1023_u64 << 52;

    if !value.is_finite() || value <= 0.0 {
        return None;
    }
    let bits = value.to_bits();
    let raw_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & FRACTION_MASK;
    if raw_exponent != 0 {
        return Some((f64::from_bits(EXPONENT_ONE | fraction), raw_exponent - 1023));
    }

    let highest_bit = 63_i32 - fraction.leading_zeros() as i32;
    let shift = u32::try_from(52_i32 - highest_bit).ok()?;
    let normalized_significand = fraction.checked_shl(shift)?;
    Some((
        f64::from_bits(EXPONENT_ONE | (normalized_significand & FRACTION_MASK)),
        highest_bit - 1074,
    ))
}

fn normalize_binary_parts(mantissa: &mut f64, exponent: &mut i32) -> Option<()> {
    if !mantissa.is_finite() || *mantissa <= 0.0 {
        return None;
    }
    while *mantissa >= 2.0 {
        *mantissa *= 0.5;
        *exponent = exponent.checked_add(1)?;
    }
    while *mantissa < 1.0 {
        *mantissa *= 2.0;
        *exponent = exponent.checked_sub(1)?;
    }
    Some(())
}

fn materialize_binary_parts(mantissa: f64, exponent: i32) -> Option<f64> {
    const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
    const MAX_MANTISSA: f64 = f64::from_bits((1023_u64 << 52) | FRACTION_MASK);

    if !(1.0..2.0).contains(&mantissa) || !(-1075..=1023).contains(&exponent) {
        return None;
    }
    if exponent == 1023 && mantissa > MAX_MANTISSA {
        return None;
    }

    let fraction = mantissa.to_bits() & FRACTION_MASK;
    if exponent >= -1022 {
        let raw_exponent = u64::try_from(exponent + 1023).ok()?;
        let value = f64::from_bits((raw_exponent << 52) | fraction);
        return (value.is_finite() && value > 0.0).then_some(value);
    }

    let significand = (1_u64 << 52) | fraction;
    let shift = u32::try_from(-1022 - exponent).ok()?;
    let truncated = significand >> shift;
    let remainder_mask = (1_u64 << shift) - 1;
    let remainder = significand & remainder_mask;
    let halfway = 1_u64 << (shift - 1);
    let round_up = remainder > halfway || (remainder == halfway && !truncated.is_multiple_of(2));
    let rounded = truncated.checked_add(u64::from(round_up))?;
    let value = f64::from_bits(rounded);
    (value > 0.0).then_some(value)
}

fn try_owned_name(name: &str) -> Result<String, FftBuildError> {
    let mut owned = String::new();
    owned
        .try_reserve_exact(name.len())
        .map_err(|_| FftBuildError::Allocation {
            stage: FftAllocationStage::Name,
            requested: name.len(),
        })?;
    owned.push_str(name);
    Ok(owned)
}

fn try_reserve_exact<T>(
    values: &mut Vec<T>,
    additional: usize,
    stage: FftAllocationStage,
) -> Result<(), FftBuildError> {
    values
        .try_reserve_exact(additional)
        .map_err(|_| FftBuildError::Allocation {
            stage,
            requested: additional,
        })
}

#[cfg(test)]
mod build_tests {
    use super::*;
    #[test]
    fn preflight_enforces_record_bounds_and_defers_backend_routes_until_planning() {
        assert!(matches!(
            preflight_fft_build(1),
            Err(FftBuildError::InsufficientSamples { .. })
        ));
        assert!(matches!(
            preflight_fft_build(MAX_QUALIFIED_RUSTFFT_LENGTH + 1),
            Err(FftBuildError::PlanningQualification(
                RustfftQualificationError::LengthLimit { .. }
            ))
        ));
        preflight_fft_build(1_048_573)
            .expect("an in-bounds analytic zero record does not require a RustFFT route");
        assert!(matches!(
            qualify_rustfft_forward_length(1_048_573),
            Err(RustfftQualificationError::BluesteinInnerLimit { .. })
        ));
        preflight_fft_build(MAX_QUALIFIED_RUSTFFT_LENGTH).expect("maximum in-bounds record length");
        qualify_rustfft_forward_length(MAX_QUALIFIED_RUSTFFT_LENGTH)
            .expect("maximum qualified power-of-two plan");
        preflight_fft_build(786_432).expect("in-bounds smooth record");
        qualify_rustfft_forward_length(786_432).expect("qualified smooth plan");
    }

    #[test]
    fn nonfinite_inputs_and_invalid_rates_are_typed() {
        let finite = [0.0; 16];
        for sample_rate in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
            assert!(matches!(
                FftData::from_time_domain(
                    "rate",
                    &finite,
                    sample_rate,
                    WindowFunction::Rectangular
                ),
                Err(FftBuildError::InvalidSampleRate { .. })
            ));
        }

        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut samples = finite;
            samples[7] = value;
            assert!(matches!(
                FftData::from_time_domain("sample", &samples, 16.0, WindowFunction::Rectangular),
                Err(FftBuildError::NonFiniteInputSample { index: 7, .. })
            ));
        }
    }

    #[test]
    fn zero_signal_is_a_valid_zero_spectrum() {
        let spectrum =
            FftData::from_time_domain("zero", &[0.0; 16], 16.0, WindowFunction::Rectangular)
                .expect("zero is a valid finite signal");

        assert_eq!(spectrum.fft_size, 16);
        assert_eq!(spectrum.points.len(), 9);
        assert!(
            spectrum
                .points
                .iter()
                .all(|point| point.magnitude == 0.0 && point.phase == 0.0)
        );
        assert_eq!(spectrum.frequency_range(), Some((0.0, 8.0)));
    }

    #[test]
    fn maximum_finite_sample_rate_keeps_every_displayed_bin_finite() {
        let spectrum = FftData::from_time_domain(
            "maximum rate",
            &[0.0; 16],
            f64::MAX,
            WindowFunction::Rectangular,
        )
        .expect("half-rate frequency products remain representable");

        assert!(
            spectrum
                .points
                .windows(2)
                .all(|pair| pair[0].frequency < pair[1].frequency)
        );
        assert!(
            spectrum
                .points
                .iter()
                .all(|point| point.frequency.is_finite())
        );
        assert_eq!(spectrum.points.last().unwrap().frequency, f64::MAX / 2.0);
        assert_eq!(spectrum.nyquist(), f64::MAX / 2.0);
    }

    #[test]
    fn unrepresentable_subnormal_frequency_grid_is_rejected() {
        assert!(matches!(
            FftData::from_time_domain(
                "subnormal rate",
                &[0.0; 16],
                f64::from_bits(1),
                WindowFunction::Rectangular
            ),
            Err(FftBuildError::UnrepresentableFrequency { bin: 1, .. })
        ));
    }

    #[test]
    fn exponent_safe_scaling_preserves_representable_extremes() {
        let maximum_dc = FftData::from_time_domain(
            "maximum DC",
            &[f64::MAX; 16],
            16.0,
            WindowFunction::Rectangular,
        )
        .expect("representable maximum DC amplitude");
        assert_eq!(maximum_dc.points[0].magnitude, f64::MAX);

        let mut maximum_nyquist_samples = [f64::MAX; 16];
        for (index, sample) in maximum_nyquist_samples.iter_mut().enumerate() {
            if !index.is_multiple_of(2) {
                *sample = -f64::MAX;
            }
        }
        let maximum_nyquist = FftData::from_time_domain(
            "maximum Nyquist",
            &maximum_nyquist_samples,
            16.0,
            WindowFunction::Rectangular,
        )
        .expect("representable maximum Nyquist amplitude");
        assert_eq!(maximum_nyquist.points[8].magnitude, f64::MAX);

        let minimum_subnormal = f64::from_bits(1);
        let subnormal_dc = FftData::from_time_domain(
            "subnormal DC",
            &[minimum_subnormal; 16],
            16.0,
            WindowFunction::Rectangular,
        )
        .expect("representable subnormal DC amplitude");
        assert_eq!(subnormal_dc.points[0].magnitude, minimum_subnormal);

        assert_eq!(
            positive_product_ratio([f64::MAX, 2.0], [2.0]),
            Some(f64::MAX)
        );
        assert_eq!(
            positive_product_ratio([minimum_subnormal, 2.0], [2.0]),
            Some(minimum_subnormal)
        );
        assert_eq!(positive_product_ratio([f64::MAX, 2.0], [1.0]), None);
        assert_eq!(
            positive_product_ratio([minimum_subnormal, 1.0], [2.0]),
            None
        );
        assert_eq!(
            positive_product_ratio([minimum_subnormal, 3.0], [4.0]),
            Some(minimum_subnormal)
        );
        assert_eq!(
            qualified_magnitude(0, 12.0, minimum_subnormal, 1.0, 16.0),
            Ok(minimum_subnormal)
        );
    }

    #[test]
    fn mixed_scale_records_fail_when_nonzero_evidence_is_erased() {
        let mut erased_input = [0.0; 16];
        erased_input[0] = f64::MAX;
        erased_input[1] = f64::MIN_POSITIVE;
        assert!(matches!(
            FftData::from_time_domain(
                "erased input",
                &erased_input,
                16.0,
                WindowFunction::Rectangular
            ),
            Err(FftBuildError::ErasedInputScale { index: 1, .. })
        ));

        let mut erased_window_product = [0.0; 16];
        erased_window_product[0] = 1.0;
        erased_window_product[1] = f64::from_bits(1);
        assert!(matches!(
            FftData::from_time_domain(
                "erased window product",
                &erased_window_product,
                16.0,
                WindowFunction::Hanning
            ),
            Err(FftBuildError::ErasedWindowedSample { index: 1, .. })
        ));
    }

    #[test]
    fn rms_scaling_preserves_self_conjugate_bins_and_scales_paired_bins() {
        let samples = (0_usize..16)
            .map(|index| {
                1.0 + 2.0 * (std::f64::consts::TAU * 2.0 * index as f64 / 16.0).cos()
                    + if index.is_multiple_of(2) { 3.0 } else { -3.0 }
            })
            .collect::<Vec<_>>();
        let peak = FftData::from_time_domain_with_normalization(
            "peak",
            &samples,
            16.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Peak,
        )
        .expect("finite peak-normalized fixture");
        let rms = FftData::from_time_domain_with_normalization(
            "rms",
            &samples,
            16.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Rms,
        )
        .expect("finite RMS-normalized fixture");

        assert!((peak.points[0].magnitude - 1.0).abs() < 1.0e-12);
        assert!((rms.points[0].magnitude - 1.0).abs() < 1.0e-12);
        assert!((peak.points[2].magnitude - 2.0).abs() < 1.0e-12);
        assert!((rms.points[2].magnitude - std::f64::consts::SQRT_2).abs() < 1.0e-12);
        assert!((peak.points[8].magnitude - 3.0).abs() < 1.0e-12);
        assert!((rms.points[8].magnitude - 3.0).abs() < 1.0e-12);

        let mut converted = peak.clone();
        converted
            .convert_normalization(SpectrumNormalization::Rms)
            .expect("representable Peak-to-RMS conversion");
        assert_eq!(converted.points, rms.points);
        converted
            .convert_normalization(SpectrumNormalization::Peak)
            .expect("representable RMS-to-Peak conversion");
        for (actual, expected) in converted.points.iter().zip(peak.points.iter()) {
            assert!((actual.magnitude - expected.magnitude).abs() <= 1.0e-15);
        }
    }

    #[test]
    fn odd_length_last_bin_keeps_the_paired_one_sided_factor() {
        let samples = (0..17)
            .map(|index| 2.0 * (std::f64::consts::TAU * 8.0 * index as f64 / 17.0).cos())
            .collect::<Vec<_>>();
        let peak = FftData::from_time_domain_with_normalization(
            "odd peak",
            &samples,
            17.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Peak,
        )
        .expect("qualified odd-length FFT");
        let rms = FftData::from_time_domain_with_normalization(
            "odd rms",
            &samples,
            17.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Rms,
        )
        .expect("qualified odd-length RMS FFT");
        assert_eq!(peak.points.len(), 9);
        assert!((peak.points[8].magnitude - 2.0).abs() < 1.0e-12);
        assert!((rms.points[8].magnitude - std::f64::consts::SQRT_2).abs() < 1.0e-12);
    }

    #[test]
    fn every_supported_window_builds_authenticated_calibration() {
        for &window in WindowFunction::all() {
            let spectrum = FftData::from_time_domain("window", &[0.0; 16], 16.0, window)
                .unwrap_or_else(|error| panic!("{window:?} failed: {error}"));
            assert!(spectrum.equivalent_noise_bandwidth_bins().is_finite());
            assert!(spectrum.equivalent_noise_bandwidth_bins() > 0.0);
        }
    }

    #[test]
    fn allocation_failures_preserve_the_requested_stage() {
        for stage in [
            FftAllocationStage::WindowCoefficients,
            FftAllocationStage::TransformBuffer,
            FftAllocationStage::TransformScratch,
            FftAllocationStage::SpectrumPoints,
        ] {
            let mut values = Vec::<u8>::new();
            assert!(matches!(
                try_reserve_exact(&mut values, usize::MAX, stage),
                Err(FftBuildError::Allocation {
                    stage: actual,
                    requested: usize::MAX
                }) if actual == stage
            ));
        }
    }
}
