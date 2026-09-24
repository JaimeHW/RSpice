//! Spectrum analysis.
//!
//! Derived spectral figures — THD, SNR, SFDR, ENOB, and the noise floor —
//! computed from a transformed record.

use super::{
    FftData, SpectrumAnalysisAllocationStage, SpectrumAnalysisError, SpectrumNormalization,
    error::MAX_SPECTRUM_HARMONIC_ORDER,
};
use crate::analysis::fft::window::WindowFunction;
// =============================================================================
// Spectrum Analysis
// =============================================================================

/// How the carrier used by relative spectrum metrics was selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FundamentalSelection {
    /// The largest authenticated non-DC physical RMS bin was treated as the
    /// carrier. This is auto-detection, not an authored fundamental, and is
    /// independent of the spectrum's display normalization.
    DominantNonDcPhysicalRms,
}

/// Evidence describing how much of the requested harmonic series was
/// actually observable and independently resolvable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HarmonicCoverage {
    /// No positive non-DC carrier exists in the record.
    NoFundamental { requested_order: usize },
    /// Every requested order was inside the retained bandwidth and had a
    /// disjoint measurement region.
    Complete { requested_order: usize },
    /// The transform bandwidth ended before the requested series.
    InsufficientBandwidth {
        requested_order: usize,
        analyzed_through: usize,
        nyquist: f64,
    },
    /// A requested harmonic could not be separated from a previously claimed
    /// carrier/harmonic main lobe at the retained bin width.
    InsufficientResolution {
        requested_order: usize,
        failed_order: usize,
        target_frequency: f64,
        bin_width: f64,
    },
}

/// One uniquely assigned, nonzero harmonic measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HarmonicMeasurement {
    /// Integer multiple of the selected carrier.
    pub order: usize,
    /// Interpolated frequency in hertz.
    pub frequency: f64,
    /// Level in the spectrum's selected display normalization.
    pub level_db: f64,
}

/// Analysis results from an FFT spectrum.
#[derive(Debug, Clone)]
pub struct SpectrumAnalysis {
    /// Policy that selected the carrier used by relative metrics.
    pub fundamental_selection: Option<FundamentalSelection>,
    /// Discrete bin that owns the selected carrier.
    pub fundamental_bin: Option<usize>,
    /// Fundamental frequency (Hz)
    pub fundamental_frequency: Option<f64>,
    /// Fundamental magnitude (dB)
    pub fundamental_db: Option<f64>,
    /// Total Harmonic Distortion (%)
    pub thd_percent: Option<f64>,
    /// THD in dB
    pub thd_db: Option<f64>,
    /// Spurious-Free Dynamic Range (dB)
    pub sfdr_db: Option<f64>,
    /// Signal-to-Noise Ratio (dB)
    pub snr_db: Option<f64>,
    /// Signal-to-Noise-and-Distortion Ratio (dB)
    pub sinad_db: Option<f64>,
    /// Noise floor (dB)
    pub noise_floor_db: Option<f64>,
    /// Coverage evidence for the requested harmonic series.
    pub harmonic_coverage: HarmonicCoverage,
    /// Uniquely assigned, order-labelled harmonic measurements.
    pub harmonics: Vec<HarmonicMeasurement>,
}

impl Default for SpectrumAnalysis {
    fn default() -> Self {
        Self {
            fundamental_selection: None,
            fundamental_bin: None,
            fundamental_frequency: None,
            fundamental_db: None,
            thd_percent: None,
            thd_db: None,
            sfdr_db: None,
            snr_db: None,
            sinad_db: None,
            noise_floor_db: None,
            harmonic_coverage: HarmonicCoverage::NoFundamental { requested_order: 1 },
            harmonics: Vec::new(),
        }
    }
}

impl SpectrumAnalysis {
    /// Analyze authenticated FFT data without infallible allocation or
    /// scale-dependent power arithmetic.
    pub fn analyze(fft: &FftData, num_harmonics: usize) -> Result<Self, SpectrumAnalysisError> {
        validate_analysis_input(fft, num_harmonics)?;
        let mut analysis = Self {
            harmonic_coverage: HarmonicCoverage::NoFundamental {
                requested_order: num_harmonics,
            },
            ..Self::default()
        };
        let Some(fund_idx) = dominant_physical_non_dc_bin(fft)? else {
            return Ok(analysis);
        };

        let Some(fundamental) = Self::measure_bin(fft, fund_idx) else {
            return Ok(analysis);
        };
        let (fund_bin, fund_freq, fund_mag, fund_db) = fundamental;
        let fund_log_rms = physical_log_magnitude(fft, fund_bin, fund_mag)?;
        let fund_binary_rms = physical_binary_magnitude(fft, fund_bin, fund_mag)?;

        analysis.fundamental_selection = Some(FundamentalSelection::DominantNonDcPhysicalRms);
        analysis.fundamental_bin = Some(fund_bin);
        analysis.fundamental_frequency = Some(fund_freq);
        analysis.fundamental_db = Some(fund_db);

        let guard_bins = Self::guard_bins(fft.window);
        let mut excluded_for_noise = try_false_mask(
            fft.points.len(),
            SpectrumAnalysisAllocationStage::NoiseExclusionMask,
        )?;
        let mut excluded_for_spur = try_false_mask(
            fft.points.len(),
            SpectrumAnalysisAllocationStage::SpurExclusionMask,
        )?;
        Self::exclude_bin_region(&mut excluded_for_noise, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_noise, fund_bin, guard_bins);
        Self::exclude_bin_region(&mut excluded_for_spur, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_spur, fund_bin, guard_bins);

        let nyquist = fft.nyquist();
        let max_observable_order = ((nyquist / fund_freq).floor() as usize)
            .max(1)
            .min(fft.fft_size.max(1));
        let analyzed_through = num_harmonics.min(max_observable_order);
        let mut harmonic_norm = LogSumSquares::default();
        let mut harmonic_binary_norm = BinaryScaledNorm::default();
        let mut coverage = if analyzed_through == num_harmonics {
            HarmonicCoverage::Complete {
                requested_order: num_harmonics,
            }
        } else {
            HarmonicCoverage::InsufficientBandwidth {
                requested_order: num_harmonics,
                analyzed_through,
                nyquist,
            }
        };
        let bin_width = fft.frequency_resolution();
        for order in 2..=analyzed_through {
            let target = fund_freq * order as f64;
            let center = (target / bin_width).round() as usize;
            if center == 0 || center >= fft.points.len() {
                coverage = HarmonicCoverage::InsufficientBandwidth {
                    requested_order: num_harmonics,
                    analyzed_through: order - 1,
                    nyquist,
                };
                break;
            }
            let start = center.saturating_sub(guard_bins);
            let end = center
                .saturating_add(guard_bins)
                .min(excluded_for_noise.len() - 1);
            if excluded_for_noise[start..=end]
                .iter()
                .any(|claimed| *claimed)
            {
                coverage = HarmonicCoverage::InsufficientResolution {
                    requested_order: num_harmonics,
                    failed_order: order,
                    target_frequency: target,
                    bin_width,
                };
                break;
            }

            let Some((harm_idx, harm_freq, harm_mag, harm_db)) = Self::measure_bin(fft, center)
            else {
                continue;
            };
            harmonic_norm.add_log(physical_log_magnitude(fft, harm_idx, harm_mag)?);
            harmonic_binary_norm.add(physical_binary_magnitude(fft, harm_idx, harm_mag)?);
            try_reserve_for_push(
                &mut analysis.harmonics,
                SpectrumAnalysisAllocationStage::Harmonics,
            )?;
            analysis.harmonics.push(HarmonicMeasurement {
                order,
                frequency: harm_freq,
                level_db: harm_db,
            });
            Self::exclude_bin_region(&mut excluded_for_noise, harm_idx, guard_bins);
        }
        analysis.harmonic_coverage = coverage;

        if matches!(coverage, HarmonicCoverage::Complete { .. }) {
            match harmonic_norm.log_norm() {
                Some(harmonic_log_rms) => {
                    let log_ratio = harmonic_log_rms - fund_log_rms;
                    let thd_percent =
                        materialize_thd_percent(&harmonic_binary_norm, fund_binary_rms, log_ratio)?;
                    analysis.thd_percent = Some(thd_percent);
                    analysis.thd_db = Some(db_from_log_amplitude_ratio(log_ratio)?);
                }
                None => {
                    analysis.thd_percent = Some(0.0);
                    analysis.thd_db = Some(f64::NEG_INFINITY);
                }
            }
        }

        // SFDR includes harmonics: only DC and the selected carrier are
        // excluded from the spur search. Noise metrics separately exclude
        // each uniquely assigned harmonic region.
        let mut largest_spur_log_rms: Option<f64> = None;
        let mut noise_norm = LogSumSquares::default();
        let mut noise_db_bins = Vec::new();
        try_reserve_exact(
            &mut noise_db_bins,
            fft.points.len(),
            SpectrumAnalysisAllocationStage::NoiseLevelBins,
        )?;
        for (idx, point) in fft.points.iter().enumerate().skip(1) {
            let mag = point.magnitude;
            if mag == 0.0 {
                continue;
            }
            let db = point.magnitude_db();
            let log_rms = physical_log_magnitude(fft, idx, mag)?;

            if !excluded_for_spur.get(idx).copied().unwrap_or(false) {
                largest_spur_log_rms =
                    Some(largest_spur_log_rms.map_or(log_rms, |largest| largest.max(log_rms)));
            }

            if excluded_for_noise.get(idx).copied().unwrap_or(false) {
                continue;
            }
            noise_norm.add_log(log_rms);
            noise_db_bins.push(db);
        }

        analysis.sfdr_db = Some(match largest_spur_log_rms {
            Some(spur_log_rms) => db_from_log_amplitude_ratio(fund_log_rms - spur_log_rms)?,
            None => f64::INFINITY,
        });

        if !noise_db_bins.is_empty() {
            noise_db_bins.sort_unstable_by(|a, b| a.total_cmp(b));
            let mid = noise_db_bins.len() / 2;
            let noise_floor = if noise_db_bins.len() % 2 == 0 {
                0.5 * (noise_db_bins[mid - 1] + noise_db_bins[mid])
            } else {
                noise_db_bins[mid]
            };
            analysis.noise_floor_db = Some(noise_floor);
        }

        // The window spreads each noise bin's power over its equivalent noise
        // bandwidth, so summing the bins counts broadband noise ENBW times
        // over. Without this division the SNR of the same record depends on
        // the window it was viewed through — 3 dB apart between a rectangular
        // and a Blackman-Harris window on identical data. Harmonics are
        // coherent tones concentrated in their own bins and are not divided.
        let noise_log_rms = noise_norm
            .log_norm()
            .map(|log_norm| log_norm - 0.5 * fft.equivalent_noise_bandwidth_bins().ln());
        analysis.snr_db = Some(match noise_log_rms {
            Some(noise) => db_from_log_amplitude_ratio(fund_log_rms - noise)?,
            None => f64::INFINITY,
        });

        if matches!(coverage, HarmonicCoverage::Complete { .. }) {
            analysis.sinad_db = Some(match log_hypot(noise_log_rms, harmonic_norm.log_norm()) {
                Some(noise_and_distortion) => {
                    db_from_log_amplitude_ratio(fund_log_rms - noise_and_distortion)?
                }
                None => f64::INFINITY,
            });
        }

        Ok(analysis)
    }

    fn interpolate_peak(fft: &FftData, idx: usize) -> Option<(usize, f64, f64, f64)> {
        let point = fft.points.get(idx)?;
        let mut freq = point.frequency;
        let mut db = point.magnitude_db();
        let mut mag = point.magnitude;
        if !db.is_finite() {
            return None;
        }

        if idx > 0 && idx + 1 < fft.points.len() {
            let left = fft.points[idx - 1].magnitude_db();
            let center = fft.points[idx].magnitude_db();
            let right = fft.points[idx + 1].magnitude_db();
            if left.is_finite() && center.is_finite() && right.is_finite() {
                let denom = left - 2.0 * center + right;
                if denom.abs() > 1e-12 {
                    let delta = (0.5 * (left - right) / denom).clamp(-0.5, 0.5);
                    let df = fft.frequency_resolution();
                    freq = point.frequency + delta * df;
                    let interpolated_db = center - 0.25 * (left - right) * delta;
                    if interpolated_db != center {
                        db = interpolated_db;
                        mag = 10.0_f64.powf(db / 20.0);
                    }
                }
            }
        }

        if !mag.is_finite() || mag <= 0.0 {
            return None;
        }

        Some((idx, freq, mag, db))
    }

    fn measure_bin(fft: &FftData, idx: usize) -> Option<(usize, f64, f64, f64)> {
        Self::interpolate_peak(fft, idx).or_else(|| {
            let point = fft.points.get(idx)?;
            (point.magnitude > 0.0)
                .then(|| (idx, point.frequency, point.magnitude, point.magnitude_db()))
        })
    }

    fn guard_bins(window: WindowFunction) -> usize {
        match window {
            WindowFunction::Rectangular => 1,
            WindowFunction::Hanning | WindowFunction::Hamming => 2,
            WindowFunction::Blackman | WindowFunction::Kaiser | WindowFunction::Gaussian => 3,
            WindowFunction::BlackmanHarris | WindowFunction::FlatTop => 4,
        }
    }

    fn exclude_bin_region(mask: &mut [bool], center: usize, radius: usize) {
        if mask.is_empty() {
            return;
        }
        let start = center.saturating_sub(radius);
        let end = center.saturating_add(radius).min(mask.len() - 1);
        for value in mask.iter_mut().take(end + 1).skip(start) {
            *value = true;
        }
    }
}

#[derive(Debug, Default)]
struct LogSumSquares {
    max_log: f64,
    scaled_sum: f64,
    has_value: bool,
}

impl LogSumSquares {
    fn add_log(&mut self, log_magnitude: f64) {
        if !self.has_value {
            self.max_log = log_magnitude;
            self.scaled_sum = 1.0;
            self.has_value = true;
        } else if log_magnitude > self.max_log {
            self.scaled_sum = 1.0 + self.scaled_sum * (2.0 * (self.max_log - log_magnitude)).exp();
            self.max_log = log_magnitude;
        } else {
            self.scaled_sum += (2.0 * (log_magnitude - self.max_log)).exp();
        }
    }

    fn log_norm(&self) -> Option<f64> {
        self.has_value
            .then(|| self.max_log + 0.5 * self.scaled_sum.ln())
    }
}

/// A positive finite value represented without forcing it into the normal or
/// subnormal exponent range. Keeping this normalized binary pair lets a
/// physically positive ratio survive until its final, ties-to-even `f64`
/// materialization.
#[derive(Debug, Clone, Copy)]
struct PositiveBinaryMagnitude {
    mantissa: f64,
    exponent: i32,
}

impl PositiveBinaryMagnitude {
    fn new(value: f64) -> Option<Self> {
        let (mantissa, exponent) = positive_binary_parts(value)?;
        Some(Self { mantissa, exponent })
    }

    fn normalize(mut self) -> Option<Self> {
        normalize_binary_parts(&mut self.mantissa, &mut self.exponent)?;
        Some(self)
    }

    fn total_cmp(self, other: Self) -> std::cmp::Ordering {
        self.exponent
            .cmp(&other.exponent)
            .then_with(|| self.mantissa.total_cmp(&other.mantissa))
    }

    fn ratio_le_one(self, larger: Self) -> f64 {
        debug_assert!(self.total_cmp(larger).is_le());
        let exponent_delta = self.exponent - larger.exponent;
        (self.mantissa / larger.mantissa) * 2.0_f64.powi(exponent_delta)
    }
}

/// BLAS-style scaled sum of squares, retaining a binary scale so the norm can
/// participate in a subnormal-safe final ratio without first underflowing.
#[derive(Debug, Default)]
struct BinaryScaledNorm {
    scale: Option<PositiveBinaryMagnitude>,
    scaled_sum_squares: f64,
    value_count: usize,
}

impl BinaryScaledNorm {
    fn add(&mut self, value: PositiveBinaryMagnitude) {
        let Some(scale) = self.scale else {
            self.scale = Some(value);
            self.scaled_sum_squares = 1.0;
            self.value_count = 1;
            return;
        };
        self.value_count = self.value_count.saturating_add(1);

        if value.total_cmp(scale).is_gt() {
            let ratio = scale.ratio_le_one(value);
            self.scaled_sum_squares = 1.0 + self.scaled_sum_squares * ratio * ratio;
            self.scale = Some(value);
        } else {
            let ratio = value.ratio_le_one(scale);
            self.scaled_sum_squares += ratio * ratio;
        }
    }

    fn normalized_parts(&self) -> Option<PositiveBinaryMagnitude> {
        let scale = self.scale?;
        PositiveBinaryMagnitude {
            mantissa: scale.mantissa * self.scaled_sum_squares.sqrt(),
            exponent: scale.exponent,
        }
        .normalize()
    }

    fn single_value(&self) -> Option<PositiveBinaryMagnitude> {
        if self.value_count == 1 {
            self.scale
        } else {
            None
        }
    }
}

fn physical_log_magnitude(
    fft: &FftData,
    bin: usize,
    magnitude: f64,
) -> Result<f64, SpectrumAnalysisError> {
    let physical = physical_binary_magnitude(fft, bin, magnitude)?;
    let log_rms = physical.mantissa.ln() + f64::from(physical.exponent) * 2.0_f64.ln();
    if log_rms.is_finite() {
        Ok(log_rms)
    } else {
        Err(SpectrumAnalysisError::InvalidSpectrum {
            reason: "a physical RMS bin magnitude is not logarithmically representable",
        })
    }
}

fn physical_binary_magnitude(
    fft: &FftData,
    bin: usize,
    magnitude: f64,
) -> Result<PositiveBinaryMagnitude, SpectrumAnalysisError> {
    let mut value =
        PositiveBinaryMagnitude::new(magnitude).ok_or(SpectrumAnalysisError::InvalidSpectrum {
            reason: "a measured spectrum magnitude is not finite and positive",
        })?;
    let is_endpoint = bin == 0 || (fft.fft_size.is_multiple_of(2) && bin == fft.fft_size / 2);
    if matches!(fft.normalization, SpectrumNormalization::Peak) && !is_endpoint {
        value.mantissa *= std::f64::consts::FRAC_1_SQRT_2;
        value = value
            .normalize()
            .ok_or(SpectrumAnalysisError::InvalidSpectrum {
                reason: "a physical RMS bin magnitude has no normalized binary representation",
            })?;
    }
    Ok(value)
}

fn materialize_thd_percent(
    harmonic_norm: &BinaryScaledNorm,
    fundamental: PositiveBinaryMagnitude,
    log_amplitude_ratio: f64,
) -> Result<f64, SpectrumAnalysisError> {
    if let Some(harmonic) = harmonic_norm.single_value() {
        return materialize_exact_percent_ratio(harmonic, fundamental).ok_or(
            SpectrumAnalysisError::UnrepresentableMetric {
                metric: "THD percent",
                log_amplitude_ratio,
            },
        );
    }

    let Some(harmonic) = harmonic_norm.normalized_parts() else {
        return Err(SpectrumAnalysisError::UnrepresentableMetric {
            metric: "THD percent",
            log_amplitude_ratio,
        });
    };
    let mut percentage = PositiveBinaryMagnitude {
        mantissa: (harmonic.mantissa / fundamental.mantissa) * 100.0,
        exponent: harmonic.exponent.checked_sub(fundamental.exponent).ok_or(
            SpectrumAnalysisError::UnrepresentableMetric {
                metric: "THD percent",
                log_amplitude_ratio,
            },
        )?,
    };
    percentage = percentage
        .normalize()
        .ok_or(SpectrumAnalysisError::UnrepresentableMetric {
            metric: "THD percent",
            log_amplitude_ratio,
        })?;
    let materialized = materialize_binary_parts(percentage.mantissa, percentage.exponent).ok_or(
        SpectrumAnalysisError::UnrepresentableMetric {
            metric: "THD percent",
            log_amplitude_ratio,
        },
    )?;
    // The scaled multi-harmonic norm necessarily rounds its sum and square
    // root before the final division. A subnormal result has no spare bits to
    // absorb that uncertainty, so fail closed instead of claiming a specific
    // subnormal cell. A single retained harmonic takes the exact integer path
    // above and is rounded only once.
    if materialized.is_subnormal() {
        return Err(SpectrumAnalysisError::UnrepresentableMetric {
            metric: "THD percent",
            log_amplitude_ratio,
        });
    }
    Ok(materialized)
}

/// Select the strongest physical non-DC bin without consulting display
/// magnitudes. Peak normalization stores interior bins `sqrt(2)` above their
/// RMS values while preserving DC and Nyquist, so comparing stored values can
/// change the selected carrier when only the display normalization changes.
fn dominant_physical_non_dc_bin(fft: &FftData) -> Result<Option<usize>, SpectrumAnalysisError> {
    let mut strongest: Option<(usize, PositiveBinaryMagnitude)> = None;
    for (bin, point) in fft.points.iter().enumerate().skip(1) {
        if point.magnitude == 0.0 {
            continue;
        }
        let physical_rms = physical_binary_magnitude(fft, bin, point.magnitude)?;
        if strongest.is_none_or(|(_, strongest_rms)| physical_rms.total_cmp(strongest_rms).is_gt())
        {
            strongest = Some((bin, physical_rms));
        }
    }
    Ok(strongest.map(|(bin, _)| bin))
}

/// Correctly round `100 * numerator / denominator` from the exact binary
/// values represented by the two normalized pairs. The integer quotient is
/// rounded once, at the destination binary64 cell, avoiding division-then-
/// multiplication double rounding at subnormal ties.
fn materialize_exact_percent_ratio(
    numerator: PositiveBinaryMagnitude,
    denominator: PositiveBinaryMagnitude,
) -> Option<f64> {
    const HIDDEN_BIT: u128 = 1_u128 << 52;

    let numerator_significand = u128::from(significand(numerator.mantissa));
    let denominator_significand = u128::from(significand(denominator.mantissa));
    let scaled_numerator = numerator_significand.checked_mul(100)?;
    let base_exponent = numerator.exponent.checked_sub(denominator.exponent)?;

    // 100 * m_n / m_d lies in [50, 200), so its binary exponent is 5..=7.
    let mut ratio_exponent = 0_i32;
    while scaled_numerator
        >= denominator_significand.checked_shl(u32::try_from(ratio_exponent + 1).ok()?)?
    {
        ratio_exponent += 1;
    }
    let mut value_exponent = base_exponent.checked_add(ratio_exponent)?;

    if value_exponent >= -1022 {
        if value_exponent > 1023 {
            return None;
        }
        let integer_numerator = scaled_numerator.checked_shl(52)?;
        let integer_denominator =
            denominator_significand.checked_shl(u32::try_from(ratio_exponent).ok()?)?;
        let mut rounded_significand =
            round_positive_ratio_ties_even(integer_numerator, integer_denominator)?;
        if rounded_significand == 2 * HIDDEN_BIT {
            rounded_significand = HIDDEN_BIT;
            value_exponent = value_exponent.checked_add(1)?;
        }
        if value_exponent > 1023 || !(HIDDEN_BIT..2 * HIDDEN_BIT).contains(&rounded_significand) {
            return None;
        }
        let raw_exponent = u64::try_from(value_exponent + 1023).ok()?;
        let fraction = u64::try_from(rounded_significand - HIDDEN_BIT).ok()?;
        let value = f64::from_bits((raw_exponent << 52) | fraction);
        return (value.is_finite() && value > 0.0).then_some(value);
    }

    // Express the exact percentage directly in units of the minimum
    // subnormal, 2^-1074, and round that rational integer ties-to-even.
    let subnormal_shift = base_exponent.checked_add(1074)?;
    let (integer_numerator, integer_denominator) = if subnormal_shift >= 0 {
        (
            scaled_numerator.checked_shl(u32::try_from(subnormal_shift).ok()?)?,
            denominator_significand,
        )
    } else {
        let right_shift = u32::try_from(-subnormal_shift).ok()?;
        // Since the unshifted ratio is below 200, a shift of nine or more
        // places is strictly below half a minimum-subnormal unit.
        if right_shift >= 9 {
            return None;
        }
        (
            scaled_numerator,
            denominator_significand.checked_shl(right_shift)?,
        )
    };
    let rounded_units = round_positive_ratio_ties_even(integer_numerator, integer_denominator)?;
    if rounded_units == 0 || rounded_units > HIDDEN_BIT {
        return None;
    }
    Some(f64::from_bits(u64::try_from(rounded_units).ok()?))
}

fn significand(mantissa: f64) -> u64 {
    const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
    (1_u64 << 52) | (mantissa.to_bits() & FRACTION_MASK)
}

fn round_positive_ratio_ties_even(numerator: u128, denominator: u128) -> Option<u128> {
    if denominator == 0 {
        return None;
    }
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    let doubled_remainder = remainder.checked_mul(2)?;
    let round_up = doubled_remainder > denominator
        || (doubled_remainder == denominator && !quotient.is_multiple_of(2));
    quotient.checked_add(u128::from(round_up))
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

fn db_from_log_amplitude_ratio(log_ratio: f64) -> Result<f64, SpectrumAnalysisError> {
    let value = 20.0 * log_ratio / 10.0_f64.ln();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SpectrumAnalysisError::UnrepresentableMetric {
            metric: "decibel amplitude ratio",
            log_amplitude_ratio: log_ratio,
        })
    }
}

fn log_hypot(left: Option<f64>, right: Option<f64>) -> Option<f64> {
    match (left, right) {
        (None, None) => None,
        (Some(value), None) | (None, Some(value)) => Some(value),
        (Some(left), Some(right)) => {
            let largest = left.max(right);
            let scaled = (2.0 * (left - largest)).exp() + (2.0 * (right - largest)).exp();
            Some(largest + 0.5 * scaled.ln())
        }
    }
}

fn try_false_mask(
    length: usize,
    stage: SpectrumAnalysisAllocationStage,
) -> Result<Vec<bool>, SpectrumAnalysisError> {
    let mut mask = Vec::new();
    try_reserve_exact(&mut mask, length, stage)?;
    mask.resize(length, false);
    Ok(mask)
}

fn try_reserve_exact<T>(
    values: &mut Vec<T>,
    additional: usize,
    stage: SpectrumAnalysisAllocationStage,
) -> Result<(), SpectrumAnalysisError> {
    values
        .try_reserve_exact(additional)
        .map_err(|_| SpectrumAnalysisError::Allocation {
            stage,
            requested: additional,
        })
}

fn try_reserve_for_push<T>(
    values: &mut Vec<T>,
    stage: SpectrumAnalysisAllocationStage,
) -> Result<(), SpectrumAnalysisError> {
    if values.len() == values.capacity() {
        values
            .try_reserve(1)
            .map_err(|_| SpectrumAnalysisError::Allocation {
                stage,
                requested: 1,
            })?;
    }
    Ok(())
}

fn validate_analysis_input(
    fft: &FftData,
    num_harmonics: usize,
) -> Result<(), SpectrumAnalysisError> {
    if !(1..=MAX_SPECTRUM_HARMONIC_ORDER).contains(&num_harmonics) {
        return Err(SpectrumAnalysisError::InvalidHarmonicOrder {
            value: num_harmonics,
            minimum: 1,
            maximum: MAX_SPECTRUM_HARMONIC_ORDER,
        });
    }
    if !fft.sample_rate.is_finite() || fft.sample_rate <= 0.0 {
        return Err(SpectrumAnalysisError::InvalidSpectrum {
            reason: "sample rate must be finite and positive",
        });
    }
    if fft.fft_size < 2 || fft.points.len() != fft.fft_size / 2 + 1 {
        return Err(SpectrumAnalysisError::InvalidSpectrum {
            reason: "one-sided point count does not match the transform length",
        });
    }
    if !fft.equivalent_noise_bandwidth_bins().is_finite()
        || fft.equivalent_noise_bandwidth_bins() <= 0.0
    {
        return Err(SpectrumAnalysisError::InvalidSpectrum {
            reason: "equivalent noise bandwidth must be finite and positive",
        });
    }
    let bin_width = fft.frequency_resolution();
    if !bin_width.is_finite() || bin_width <= 0.0 || fft.points[0].frequency != 0.0 {
        return Err(SpectrumAnalysisError::InvalidSpectrum {
            reason: "frequency grid must start at DC with a finite positive bin width",
        });
    }
    let mut previous_frequency = None;
    for (bin, point) in fft.points.iter().enumerate() {
        let valid_frequency = point.frequency.is_finite()
            && point.frequency >= 0.0
            && previous_frequency.is_none_or(|previous| point.frequency > previous);
        if !valid_frequency
            || !point.magnitude.is_finite()
            || point.magnitude < 0.0
            || !point.phase.is_finite()
        {
            return Err(SpectrumAnalysisError::InvalidSpectrumPoint {
                bin,
                frequency: point.frequency,
                magnitude: point.magnitude,
                phase: point.phase,
                reason: "bins must increase strictly with finite nonnegative magnitude and finite phase",
            });
        }
        let expected = fft.sample_rate * (bin as f64 / fft.fft_size as f64);
        let tolerance = 16.0 * f64::EPSILON * expected.abs().max(bin_width);
        if (point.frequency - expected).abs() > tolerance {
            return Err(SpectrumAnalysisError::InvalidSpectrumPoint {
                bin,
                frequency: point.frequency,
                magnitude: point.magnitude,
                phase: point.phase,
                reason: "bin frequency does not belong to the uniform transform grid",
            });
        }
        previous_frequency = Some(point.frequency);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::fft::data::FftPoint;
    use crate::analysis::fft::data::SpectrumNormalization;

    fn spectrum_from_magnitudes(
        magnitudes: &[f64],
        normalization: SpectrumNormalization,
    ) -> FftData {
        let fft_size = magnitudes.len().saturating_sub(1) * 2;
        FftData {
            name: "analytic".to_owned(),
            points: magnitudes
                .iter()
                .enumerate()
                .map(|(bin, &magnitude)| FftPoint::new(bin as f64, magnitude, 0.0))
                .collect(),
            sample_rate: fft_size as f64,
            fft_size,
            window: WindowFunction::Rectangular,
            normalization,
            equivalent_noise_bandwidth_bins: 1.0,
        }
    }

    /// Deterministic noise. A seeded generator written here rather than
    /// pulled in keeps the oracle's numbers fixed across dependency bumps.
    struct Xorshift(u64);

    impl Xorshift {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }

        fn next_unit(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }

        fn next_gaussian(&mut self) -> f64 {
            let u1 = self.next_unit().max(1e-12);
            let u2 = self.next_unit();
            (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
        }
    }

    /// A tone on an exact bin centre plus white noise.
    ///
    /// Returns the record and the noise's own mean-square power, so the
    /// oracle is `10·log10(signal power / noise power)` for the record that
    /// was actually generated rather than for the distribution it was drawn
    /// from — the sampling error of one 8192-point draw is a tenth of a
    /// decibel, and the effect under test is a whole one.
    fn tone_with_noise(length: usize, bin: usize, amplitude: f64, sigma: f64) -> (Vec<f64>, f64) {
        let mut rng = Xorshift(0x5eed_1234_9abc_def1);
        let noise: Vec<f64> = (0..length).map(|_| sigma * rng.next_gaussian()).collect();
        let noise_power = noise.iter().map(|v| v * v).sum::<f64>() / length as f64;
        let samples = noise
            .iter()
            .enumerate()
            .map(|(n, value)| {
                let phase = std::f64::consts::TAU * bin as f64 * n as f64 / length as f64;
                amplitude * phase.sin() + value
            })
            .collect();
        (samples, noise_power)
    }

    /// The window spreads each noise bin's power over its equivalent noise
    /// bandwidth, so a bare sum of noise-bin powers over-counts by exactly
    /// that factor — and the reported SNR of one record then depends on the
    /// window it happens to be displayed with. Three windows with ENBWs from
    /// 1.00 to 2.00 bins must agree on the same record.
    #[test]
    fn signal_to_noise_is_the_same_measurement_through_every_window() {
        const LENGTH: usize = 8192;
        const AMPLITUDE: f64 = 1.0;
        const SIGMA: f64 = 0.01;
        let (samples, noise_power) = tone_with_noise(LENGTH, 517, AMPLITUDE, SIGMA);
        let expected = 10.0 * (0.5 * AMPLITUDE.powi(2) / noise_power).log10();

        for window in [
            WindowFunction::Rectangular,
            WindowFunction::Hanning,
            WindowFunction::BlackmanHarris,
        ] {
            let fft = FftData::from_time_domain_with_normalization(
                "v(out)",
                &samples,
                1e6,
                window,
                SpectrumNormalization::Peak,
            )
            .expect("finite qualified spectrum-analysis fixture");
            let analysis =
                SpectrumAnalysis::analyze(&fft, 1).expect("qualified noisy spectrum analysis");
            let snr = analysis.snr_db.expect("a noisy tone has an SNR");
            assert!(
                (snr - expected).abs() <= 0.3,
                "{window:?} reported {snr:.3} dB, expected {expected:.3} dB"
            );
        }
    }

    /// SINAD degenerates to SNR when there is nothing but noise beside the
    /// tone, so it has to carry the same correction.
    #[test]
    fn distortion_free_sinad_agrees_with_signal_to_noise() {
        let (samples, _) = tone_with_noise(8192, 517, 1.0, 0.01);
        let fft = FftData::from_time_domain_with_normalization(
            "v(out)",
            &samples,
            1e6,
            WindowFunction::BlackmanHarris,
            SpectrumNormalization::Peak,
        )
        .expect("finite qualified SINAD fixture");
        let analysis = SpectrumAnalysis::analyze(&fft, 1)
            .expect("qualified distortion-free spectrum analysis");
        let snr = analysis.snr_db.expect("SNR");
        let sinad = analysis.sinad_db.expect("SINAD");
        assert!(
            (snr - sinad).abs() <= 1e-9,
            "SNR {snr} and SINAD {sinad} diverge with no distortion present"
        );
    }

    #[test]
    fn extreme_finite_spectra_keep_scale_safe_thd_snr_sinad_and_sfdr() {
        for fundamental in [f64::MAX / 4.0, 16.0 * f64::from_bits(1)] {
            let harmonic = fundamental / 2.0;
            let noise = fundamental / 4.0;
            let mut magnitudes = vec![0.0; 9];
            magnitudes[1] = noise;
            magnitudes[4] = fundamental;
            magnitudes[8] = harmonic;
            let fft = spectrum_from_magnitudes(&magnitudes, SpectrumNormalization::Peak);
            let analysis = SpectrumAnalysis::analyze(&fft, 2)
                .expect("finite extreme analytic spectrum must remain analyzable");

            let fundamental_log_rms = fundamental.ln() - 0.5 * 2.0_f64.ln();
            let harmonic_log_rms = harmonic.ln();
            let noise_log_rms = noise.ln() - 0.5 * 2.0_f64.ln();
            let expected_thd = (harmonic_log_rms - fundamental_log_rms).exp() * 100.0;
            let expected_snr = 20.0 * (fundamental_log_rms - noise_log_rms) / 10.0_f64.ln();
            let expected_sfdr = 20.0 * (fundamental_log_rms - harmonic_log_rms) / 10.0_f64.ln();
            let expected_sinad = 20.0
                * (fundamental_log_rms
                    - log_hypot(Some(noise_log_rms), Some(harmonic_log_rms)).unwrap())
                / 10.0_f64.ln();

            assert!((analysis.thd_percent.unwrap() - expected_thd).abs() <= 1.0e-10);
            assert!((analysis.snr_db.unwrap() - expected_snr).abs() <= 1.0e-10);
            assert!((analysis.sfdr_db.unwrap() - expected_sfdr).abs() <= 1.0e-10);
            assert!((analysis.sinad_db.unwrap() - expected_sinad).abs() <= 1.0e-10);
            assert_eq!(analysis.harmonics[0].order, 2);
            assert_eq!(analysis.harmonics[0].frequency, 8.0);
        }
    }

    #[test]
    fn extreme_finite_time_records_do_not_collapse_to_empty_analysis() {
        for amplitude in [f64::MAX, f64::MIN_POSITIVE] {
            let samples = (0_usize..64)
                .map(|index| {
                    if index.is_multiple_of(2) {
                        amplitude
                    } else {
                        -amplitude
                    }
                })
                .collect::<Vec<_>>();
            let fft = FftData::from_time_domain_with_normalization(
                "nyquist",
                &samples,
                64.0,
                WindowFunction::Rectangular,
                SpectrumNormalization::Peak,
            )
            .expect("finite alternating record has a representable Nyquist spectrum");
            let analysis = SpectrumAnalysis::analyze(&fft, 1)
                .expect("finite Nyquist spectrum has a scale-safe analysis");
            assert_eq!(analysis.fundamental_bin, Some(32));
            assert_eq!(analysis.fundamental_frequency, Some(32.0));
            assert_eq!(analysis.thd_percent, Some(0.0));
            assert_eq!(analysis.thd_db, Some(f64::NEG_INFINITY));
            assert!(analysis.snr_db.is_some());
            assert!(analysis.sinad_db.is_some());
        }
    }

    #[test]
    fn invalid_spectrum_contracts_and_unbounded_harmonic_orders_fail_typed() {
        let zero = spectrum_from_magnitudes(&[0.0; 9], SpectrumNormalization::Peak);
        let analysis = SpectrumAnalysis::analyze(&zero, 1)
            .expect("an exact zero spectrum has no carrier rather than an error");
        assert!(analysis.fundamental_frequency.is_none());

        assert!(matches!(
            SpectrumAnalysis::analyze(&zero, 0),
            Err(SpectrumAnalysisError::InvalidHarmonicOrder { value: 0, .. })
        ));
        assert!(matches!(
            SpectrumAnalysis::analyze(&zero, MAX_SPECTRUM_HARMONIC_ORDER + 1),
            Err(SpectrumAnalysisError::InvalidHarmonicOrder { value, .. })
                if value == MAX_SPECTRUM_HARMONIC_ORDER + 1
        ));

        for invalid in [f64::NAN, f64::INFINITY, -1.0] {
            let mut fft = zero.clone();
            fft.points[3].magnitude = invalid;
            assert!(matches!(
                SpectrumAnalysis::analyze(&fft, 1),
                Err(SpectrumAnalysisError::InvalidSpectrumPoint { bin: 3, .. })
            ));
        }

        let mut invalid_grid = zero.clone();
        invalid_grid.points[4].frequency = invalid_grid.points[3].frequency;
        assert!(matches!(
            SpectrumAnalysis::analyze(&invalid_grid, 1),
            Err(SpectrumAnalysisError::InvalidSpectrumPoint { bin: 4, .. })
        ));

        let mut invalid_enbw = zero;
        invalid_enbw.equivalent_noise_bandwidth_bins = f64::NAN;
        assert!(matches!(
            SpectrumAnalysis::analyze(&invalid_enbw, 1),
            Err(SpectrumAnalysisError::InvalidSpectrum { .. })
        ));
    }

    #[test]
    fn harmonic_coverage_never_reuses_the_carrier_or_claims_unobserved_orders() {
        let pure_low_tone = (0..64)
            .map(|index| (std::f64::consts::TAU * index as f64 / 64.0).sin())
            .collect::<Vec<_>>();
        let low_fft = FftData::from_time_domain_with_normalization(
            "low",
            &pure_low_tone,
            64.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Peak,
        )
        .expect("coherent low-tone fixture");
        let low = SpectrumAnalysis::analyze(&low_fft, 3)
            .expect("overlapping harmonic regions are explicit coverage evidence");
        assert!(matches!(
            low.harmonic_coverage,
            HarmonicCoverage::InsufficientResolution {
                failed_order: 2,
                ..
            }
        ));
        assert!(low.harmonics.is_empty());
        assert!(low.thd_percent.is_none());

        let high_tone = (0..64)
            .map(|index| (std::f64::consts::TAU * 24.0 * index as f64 / 64.0).sin())
            .collect::<Vec<_>>();
        let high_fft = FftData::from_time_domain_with_normalization(
            "high",
            &high_tone,
            64.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Peak,
        )
        .expect("coherent high-tone fixture");
        let high = SpectrumAnalysis::analyze(&high_fft, 2)
            .expect("limited bandwidth is explicit coverage evidence");
        assert!(matches!(
            high.harmonic_coverage,
            HarmonicCoverage::InsufficientBandwidth {
                requested_order: 2,
                analyzed_through: 1,
                ..
            }
        ));
        assert!(high.thd_percent.is_none());
    }

    #[test]
    fn physical_power_metrics_are_normalization_invariant_at_nyquist() {
        let samples = (0_usize..64)
            .map(|index| {
                (std::f64::consts::TAU * 8.0 * index as f64 / 64.0).sin()
                    + if index.is_multiple_of(2) { 0.25 } else { -0.25 }
            })
            .collect::<Vec<_>>();
        let mut results = Vec::new();
        for normalization in [SpectrumNormalization::Peak, SpectrumNormalization::Rms] {
            let fft = FftData::from_time_domain_with_normalization(
                "normalization",
                &samples,
                64.0,
                WindowFunction::Rectangular,
                normalization,
            )
            .expect("coherent normalization fixture");
            results.push(
                SpectrumAnalysis::analyze(&fft, 4)
                    .expect("Nyquist endpoint has an invariant physical-power analysis"),
            );
        }
        for metric in [
            |analysis: &SpectrumAnalysis| analysis.thd_db.unwrap(),
            |analysis: &SpectrumAnalysis| analysis.sfdr_db.unwrap(),
            |analysis: &SpectrumAnalysis| analysis.snr_db.unwrap(),
            |analysis: &SpectrumAnalysis| analysis.sinad_db.unwrap(),
        ] {
            assert!((metric(&results[0]) - metric(&results[1])).abs() <= 1.0e-10);
        }
    }

    #[test]
    fn carrier_selection_uses_physical_power_not_display_normalization() {
        let mut peak_magnitudes = [0.0; 33];
        peak_magnitudes[8] = 1.0;
        peak_magnitudes[32] = 0.8;
        let mut rms_magnitudes = peak_magnitudes;
        rms_magnitudes[8] = std::f64::consts::FRAC_1_SQRT_2;

        let peak = SpectrumAnalysis::analyze(
            &spectrum_from_magnitudes(&peak_magnitudes, SpectrumNormalization::Peak),
            1,
        )
        .expect("authenticated peak-normalized spectrum");
        let rms = SpectrumAnalysis::analyze(
            &spectrum_from_magnitudes(&rms_magnitudes, SpectrumNormalization::Rms),
            1,
        )
        .expect("the same physical spectrum in RMS normalization");

        for analysis in [&peak, &rms] {
            assert_eq!(analysis.fundamental_bin, Some(32));
            assert_eq!(analysis.fundamental_frequency, Some(32.0));
            assert_eq!(
                analysis.fundamental_selection,
                Some(FundamentalSelection::DominantNonDcPhysicalRms)
            );
            assert_eq!(
                analysis.harmonic_coverage,
                HarmonicCoverage::Complete { requested_order: 1 }
            );
            assert_eq!(analysis.thd_percent, Some(0.0));
            assert_eq!(analysis.thd_db, Some(f64::NEG_INFINITY));
        }
        for metric in [
            |analysis: &SpectrumAnalysis| analysis.sfdr_db.unwrap(),
            |analysis: &SpectrumAnalysis| analysis.snr_db.unwrap(),
            |analysis: &SpectrumAnalysis| analysis.sinad_db.unwrap(),
        ] {
            assert!((metric(&peak) - metric(&rms)).abs() <= 1.0e-12);
        }

        let mut tied_peak_magnitudes = [0.0; 33];
        tied_peak_magnitudes[8] = 1.0;
        tied_peak_magnitudes[32] = std::f64::consts::FRAC_1_SQRT_2;
        let mut tied_rms_magnitudes = tied_peak_magnitudes;
        tied_rms_magnitudes[8] = std::f64::consts::FRAC_1_SQRT_2;
        let tied_peak = SpectrumAnalysis::analyze(
            &spectrum_from_magnitudes(&tied_peak_magnitudes, SpectrumNormalization::Peak),
            1,
        )
        .expect("peak-normalized equal-power carrier tie");
        let tied_rms = SpectrumAnalysis::analyze(
            &spectrum_from_magnitudes(&tied_rms_magnitudes, SpectrumNormalization::Rms),
            1,
        )
        .expect("RMS-normalized equal-power carrier tie");
        assert_eq!(tied_peak.fundamental_bin, Some(8));
        assert_eq!(tied_rms.fundamental_bin, Some(8));
    }

    #[test]
    fn thd_percent_obeys_binary64_subnormal_rounding_without_fabricating_zero() {
        let minimum_subnormal = f64::from_bits(1);
        let analyze = |fundamental, harmonic| {
            let mut magnitudes = [0.0; 9];
            magnitudes[4] = fundamental;
            magnitudes[8] = harmonic;
            SpectrumAnalysis::analyze(
                &spectrum_from_magnitudes(&magnitudes, SpectrumNormalization::Rms),
                2,
            )
        };

        let rounds_up_from_above_half = analyze(128.0, minimum_subnormal)
            .expect("a positive representable subnormal THD percentage");
        assert_eq!(rounds_up_from_above_half.thd_percent.unwrap().to_bits(), 1);

        for fundamental in [200.0, 256.0] {
            assert!(matches!(
                analyze(fundamental, minimum_subnormal),
                Err(SpectrumAnalysisError::UnrepresentableMetric {
                    metric: "THD percent",
                    log_amplitude_ratio,
                }) if log_amplitude_ratio.is_finite()
            ));
        }

        let ties_to_even = analyze(200.0, 3.0 * minimum_subnormal)
            .expect("a halfway subnormal THD percentage rounds to the even significand");
        assert_eq!(ties_to_even.thd_percent.unwrap().to_bits(), 2);

        let avoids_double_rounding = analyze(40.0, 23.0 * minimum_subnormal)
            .expect("an exact 57.5-cell tie rounds once to the even significand");
        assert_eq!(avoids_double_rounding.thd_percent.unwrap().to_bits(), 58);

        let ordinary = analyze(1.0, 0.01).expect("ordinary THD percentage");
        assert!((ordinary.thd_percent.unwrap() - 1.0).abs() <= 1.0e-15);
        assert!((ordinary.thd_db.unwrap() + 40.0).abs() <= 1.0e-12);

        let mut extreme = [0.0; 9];
        extreme[4] = f64::MAX / 4.0;
        extreme[8] = minimum_subnormal;
        assert!(matches!(
            SpectrumAnalysis::analyze(
                &spectrum_from_magnitudes(&extreme, SpectrumNormalization::Peak),
                2,
            ),
            Err(SpectrumAnalysisError::UnrepresentableMetric {
                metric: "THD percent",
                log_amplitude_ratio,
            }) if log_amplitude_ratio.is_finite()
        ));
    }

    #[test]
    fn allocation_failures_identify_every_analysis_stage() {
        for stage in [
            SpectrumAnalysisAllocationStage::NoiseExclusionMask,
            SpectrumAnalysisAllocationStage::SpurExclusionMask,
            SpectrumAnalysisAllocationStage::Harmonics,
            SpectrumAnalysisAllocationStage::NoiseLevelBins,
        ] {
            let mut values = Vec::<u8>::new();
            assert_eq!(
                try_reserve_exact(&mut values, usize::MAX, stage),
                Err(SpectrumAnalysisError::Allocation {
                    stage,
                    requested: usize::MAX,
                })
            );
        }
    }
}
