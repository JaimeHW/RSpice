//! Spectrum analysis.
//!
//! Derived spectral figures — THD, SNR, SFDR, ENOB, and the noise floor —
//! computed from a transformed record.

use super::{
    FftData, SpectrumAnalysisAllocationStage, SpectrumAnalysisError, SpectrumNormalization,
    error::MAX_SPECTRUM_HARMONIC_ORDER,
};
use crate::fft::binary_parts::{
    materialize_binary_parts, normalize_binary_parts, positive_binary_parts,
};
use crate::fft::window::WindowFunction;
// =============================================================================
// Spectrum Analysis
// =============================================================================

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
    /// The harmonic's own frequency in hertz: its order times the
    /// fundamental's.
    pub frequency: f64,
    /// Level in the spectrum's selected display normalization, read at the
    /// harmonic's own bin as the fundamental's is read at its.
    pub level_db: f64,
    /// Whether the harmonic stands out of what lies under it: its own bin is
    /// the highest its measurement reads, and its level rises over what the
    /// noise floor alone reads across those bins. A harmonic under the floor
    /// reads the floor, or a neighbour's skirt, not itself.
    pub above_floor: bool,
}

/// How a spectrum's figures are measured, beyond the record itself: the
/// choices ViVA's spectrum assistant and Eldo's FFT offer beside the window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectrumMeasurement {
    /// The harmonic orders THD and SINAD count, HD2 through this one.
    pub harmonics: usize,
    /// The bins either side of a tone claimed for it rather than counted as
    /// noise; `None` takes the window's main lobe.
    pub signal_bins: Option<usize>,
    /// The fundamental the reader named, in hertz; `None` takes the largest
    /// tone in the record.
    pub fundamental: Option<f64>,
    /// The band, in hertz, noise and spurs are measured over and harmonics
    /// are counted in; `None` is DC to Nyquist.
    pub band: Option<(f64, f64)>,
}

impl SpectrumMeasurement {
    /// The default measurement counting harmonics through `harmonics`.
    #[must_use]
    pub const fn with_harmonics(harmonics: usize) -> Self {
        Self {
            harmonics,
            signal_bins: None,
            fundamental: None,
            band: None,
        }
    }
}

impl Default for SpectrumMeasurement {
    fn default() -> Self {
        Self::with_harmonics(10)
    }
}

/// Analysis results from an FFT spectrum.
#[derive(Debug, Clone)]
pub struct SpectrumAnalysis {
    /// Discrete bin that owns the selected carrier.
    pub fundamental_bin: Option<usize>,
    /// Fundamental frequency (Hz)
    pub fundamental_frequency: Option<f64>,
    /// Fundamental magnitude (dB)
    pub fundamental_db: Option<f64>,
    /// Total Harmonic Distortion (%), over the harmonics that rise out of
    /// the noise floor
    pub thd_percent: Option<f64>,
    /// THD in dB
    pub thd_db: Option<f64>,
    /// Spurious-Free Dynamic Range (dB)
    pub sfdr_db: Option<f64>,
    /// Signal-to-Noise Ratio (dB)
    pub snr_db: Option<f64>,
    /// Signal-to-Noise-and-Distortion Ratio (dB)
    pub sinad_db: Option<f64>,
    /// The mean noise power per bin, in dB of the spectrum's level unit:
    /// the floor the plotted noise sits at, not its median.
    pub noise_floor_db: Option<f64>,
    /// The bins either side of each tone the measurement claimed for it.
    pub signal_bins: usize,
    /// Coverage evidence for the requested harmonic series.
    pub harmonic_coverage: HarmonicCoverage,
    /// Uniquely assigned, order-labelled harmonic measurements.
    pub harmonics: Vec<HarmonicMeasurement>,
}

impl Default for SpectrumAnalysis {
    fn default() -> Self {
        Self {
            fundamental_bin: None,
            fundamental_frequency: None,
            fundamental_db: None,
            thd_percent: None,
            thd_db: None,
            sfdr_db: None,
            snr_db: None,
            sinad_db: None,
            noise_floor_db: None,
            signal_bins: 0,
            harmonic_coverage: HarmonicCoverage::NoFundamental { requested_order: 1 },
            harmonics: Vec::new(),
        }
    }
}

impl SpectrumAnalysis {
    /// The effective number of bits SINAD is worth, `(SINAD − 1.76)/6.02`:
    /// the resolution of an ideal quantizer whose noise alone reads that
    /// SINAD on a full-scale sine.
    #[must_use]
    pub fn enob_bits(&self) -> Option<f64> {
        self.sinad_db.map(|sinad| (sinad - 1.76) / 6.02)
    }

    /// Analyze authenticated FFT data without infallible allocation or
    /// scale-dependent power arithmetic, counting harmonics through
    /// `num_harmonics` and otherwise measuring by default.
    pub fn analyze(fft: &FftData, num_harmonics: usize) -> Result<Self, SpectrumAnalysisError> {
        Self::measure(fft, &SpectrumMeasurement::with_harmonics(num_harmonics))
    }

    /// Analyze `fft` as `measurement` asks.
    pub fn measure(
        fft: &FftData,
        measurement: &SpectrumMeasurement,
    ) -> Result<Self, SpectrumAnalysisError> {
        let num_harmonics = measurement.harmonics;
        validate_analysis_input(fft, num_harmonics)?;
        // The bins a tone claims are the record's; a padded transform has
        // that many more of its own across the same width.
        let guard_bins = measurement
            .signal_bins
            .unwrap_or_else(|| fft.window.signal_bins())
            .saturating_mul(fft.padding());
        let mut analysis = Self {
            harmonic_coverage: HarmonicCoverage::NoFundamental {
                requested_order: num_harmonics,
            },
            signal_bins: guard_bins,
            ..Self::default()
        };
        let bin_width = fft.frequency_resolution();
        let named_fundamental = measurement
            .fundamental
            .filter(|frequency| frequency.is_finite() && *frequency > 0.0);
        let fund_idx = match named_fundamental {
            Some(frequency) => largest_bin_near(fft, frequency / bin_width, guard_bins),
            None => dominant_physical_non_dc_bin(fft)?,
        };
        let Some(fund_idx) = fund_idx else {
            return Ok(analysis);
        };

        let Some(fundamental) = Self::measure_bin(fft, fund_idx, guard_bins) else {
            return Ok(analysis);
        };
        let (fund_bin, measured_freq, fund_mag, fund_db) = fundamental;
        // Harmonics stand at multiples of the fundamental the reader named,
        // or of the one measured.
        let fund_freq = named_fundamental.unwrap_or(measured_freq);
        let fund_log_rms = physical_log_magnitude(fft, fund_bin, fund_mag)?;
        let fund_binary_rms = physical_binary_magnitude(fft, fund_bin, fund_mag)?;

        analysis.fundamental_bin = Some(fund_bin);
        analysis.fundamental_frequency = Some(measured_freq);
        analysis.fundamental_db = Some(fund_db);

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
        // A band the reader set ends the series where it ends: harmonics
        // past it are not asked for, so the series inside it is complete.
        let band = measurement
            .band
            .filter(|(low, high)| low.is_finite() && high.is_finite() && high > low);
        let num_harmonics = match band {
            Some((_, high)) => num_harmonics.min(((high / fund_freq).floor() as usize).max(1)),
            None => num_harmonics,
        };
        let analyzed_through = num_harmonics.min(max_observable_order);
        // Each harmonic's RMS, beside its measurement: THD sums those that
        // rise out of the floor, which is known only once the noise is.
        let mut harmonic_rms = Vec::new();
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

            let Some((harm_idx, _, harm_mag, harm_db)) = Self::measure_bin(fft, center, guard_bins)
            else {
                continue;
            };
            // A tone peaks at its own bin; the floor and a neighbour's skirt
            // have their highest bin anywhere else.
            let own = fft.points[center].magnitude;
            let peaks_at_its_bin = fft.points[start.max(1)..=end]
                .iter()
                .all(|point| point.magnitude <= own);
            try_reserve_for_push(
                &mut harmonic_rms,
                SpectrumAnalysisAllocationStage::Harmonics,
            )?;
            harmonic_rms.push((
                physical_log_magnitude(fft, harm_idx, harm_mag)?,
                physical_binary_magnitude(fft, harm_idx, harm_mag)?,
            ));
            try_reserve_for_push(
                &mut analysis.harmonics,
                SpectrumAnalysisAllocationStage::Harmonics,
            )?;
            analysis.harmonics.push(HarmonicMeasurement {
                order,
                frequency: target,
                level_db: harm_db,
                above_floor: peaks_at_its_bin,
            });
            Self::exclude_bin_region(&mut excluded_for_noise, harm_idx, guard_bins);
        }
        analysis.harmonic_coverage = coverage;

        // SFDR includes harmonics: only DC and the selected carrier are
        // excluded from the spur search. Noise metrics separately exclude
        // each uniquely assigned harmonic region.
        let mut largest_spur_log_rms: Option<f64> = None;
        let mut noise_norm = LogSumSquares::default();
        let mut band_bins = 0usize;
        let mut noise_db_bins = Vec::new();
        try_reserve_exact(
            &mut noise_db_bins,
            fft.points.len(),
            SpectrumAnalysisAllocationStage::NoiseLevelBins,
        )?;
        let mut noise_log_bins = Vec::new();
        try_reserve_exact(
            &mut noise_log_bins,
            fft.points.len(),
            SpectrumAnalysisAllocationStage::NoiseLevelBins,
        )?;
        for (idx, point) in fft.points.iter().enumerate().skip(1) {
            if band.is_some_and(|(low, high)| point.frequency < low || point.frequency > high) {
                continue;
            }
            band_bins += 1;
            let mag = point.magnitude;
            let noise_bin = !excluded_for_noise.get(idx).copied().unwrap_or(false);
            if mag == 0.0 {
                if noise_bin {
                    noise_db_bins.push(f64::NEG_INFINITY);
                    noise_log_bins.push(f64::NEG_INFINITY);
                }
                continue;
            }
            let log_rms = physical_log_magnitude(fft, idx, mag)?;

            if !excluded_for_spur.get(idx).copied().unwrap_or(false) {
                largest_spur_log_rms =
                    Some(largest_spur_log_rms.map_or(log_rms, |largest| largest.max(log_rms)));
            }

            if !noise_bin {
                continue;
            }
            noise_norm.add_log(log_rms);
            noise_db_bins.push(point.magnitude_db());
            noise_log_bins.push(log_rms);
        }
        // Noise lies under the tones too. The bins claimed for the carrier
        // and its harmonics are counted at the noise's typical power: the
        // median noise bin over its ratio to the mean, ln 2 for one record,
        // since a noise bin's power is exponential. The power averaged over
        // k records is a chi-square of 2k degrees, whose median sits
        // (1 - 1/9k)^3 of its mean (Wilson and Hilferty). The median, not the
        // mean, so a spur among the noise bins is counted once, not again
        // under every tone.
        let claimed = band_bins.saturating_sub(noise_log_bins.len());
        if claimed > 0 && !noise_log_bins.is_empty() {
            let middle = noise_log_bins.len() / 2;
            let (_, median, _) = noise_log_bins.select_nth_unstable_by(middle, f64::total_cmp);
            let records = fft.averages() as f64;
            let median_to_mean = if fft.averages() <= 1 {
                std::f64::consts::LN_2
            } else {
                (1.0 - 1.0 / (9.0 * records)).powi(3)
            };
            if median.is_finite() {
                noise_norm.add_log(*median + 0.5 * (claimed as f64 / median_to_mean).ln());
            }
        }

        analysis.sfdr_db = Some(match largest_spur_log_rms {
            Some(spur_log_rms) => db_from_log_amplitude_ratio(fund_log_rms - spur_log_rms)?,
            None => f64::INFINITY,
        });

        analysis.noise_floor_db = mean_power_db(&noise_db_bins);
        if let Some(floor) = analysis.noise_floor_db {
            // What the floor alone reads where a harmonic is read: its one
            // bin through the rectangular window, else the power of the
            // bins either side over the window's noise bandwidth.
            let floor_read = if fft.window == WindowFunction::Rectangular {
                floor
            } else {
                let bins = (2 * guard_bins + 1) as f64;
                floor + 10.0 * (bins / fft.equivalent_noise_bandwidth_bins()).log10()
            };
            for harmonic in &mut analysis.harmonics {
                harmonic.above_floor &= harmonic.level_db > floor_read;
            }
        }

        // THD and SINAD's distortion sum the harmonics that rise out of the
        // floor. One under it is the floor read at its bin: its bins are
        // already counted as noise, at the noise's typical power.
        let mut harmonic_norm = LogSumSquares::default();
        let mut harmonic_binary_norm = BinaryScaledNorm::default();
        for (harmonic, (log_rms, binary_rms)) in analysis.harmonics.iter().zip(&harmonic_rms) {
            if harmonic.above_floor {
                harmonic_norm.add_log(*log_rms);
                harmonic_binary_norm.add(*binary_rms);
            }
        }
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

        // The window spreads each noise bin's power over its equivalent noise
        // bandwidth, so summing the bins counts broadband noise ENBW times
        // over. Without this division the SNR of the same record depends on
        // the window it was viewed through — 3 dB apart between a rectangular
        // and a Blackman-Harris window on identical data. Harmonics are
        // coherent tones concentrated in their own bins and are not divided.
        let noise_log_rms = noise_norm
            .log_norm()
            .map(|log_norm| log_norm - 0.5 * fft.equivalent_noise_bandwidth_bins().ln());
        // A harmonic the record cannot resolve from the carrier is summed
        // into the noise, so the noise figure is then noise and distortion:
        // SNR is withheld with THD and SINAD rather than print SINAD under
        // its name.
        if matches!(coverage, HarmonicCoverage::Complete { .. }) {
            analysis.snr_db = Some(match noise_log_rms {
                Some(noise) => db_from_log_amplitude_ratio(fund_log_rms - noise)?,
                None => f64::INFINITY,
            });
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

    /// One tone's bin, frequency and level.
    ///
    /// Through a tapering window the tone's power is the power of its
    /// `signal_bins` either side, divided by the window's noise bandwidth,
    /// which is Parseval's answer wherever between bins the tone falls; its
    /// frequency is interpolated between the bins. Through the rectangular
    /// window the bin is read as it stands, since a coherent tone's bin is
    /// then exact and its neighbours are noise that would drag it.
    fn measure_bin(
        fft: &FftData,
        idx: usize,
        signal_bins: usize,
    ) -> Option<(usize, f64, f64, f64)> {
        let point = fft.points.get(idx)?;
        if fft.window == WindowFunction::Rectangular {
            return (point.magnitude > 0.0)
                .then(|| (idx, point.frequency, point.magnitude, point.magnitude_db()));
        }
        let frequency = Self::interpolate_peak(fft, idx).map_or(point.frequency, |peak| peak.1);
        let low = idx.saturating_sub(signal_bins).max(1);
        let high = idx.saturating_add(signal_bins).min(fft.points.len() - 1);
        let scale = fft.points[low..=high]
            .iter()
            .fold(0.0_f64, |largest, point| largest.max(point.magnitude));
        if scale <= 0.0 || !scale.is_finite() {
            return None;
        }
        let power = fft.points[low..=high]
            .iter()
            .map(|point| (point.magnitude / scale).powi(2))
            .sum::<f64>()
            / fft.equivalent_noise_bandwidth_bins();
        let magnitude = scale * power.sqrt();
        (magnitude.is_finite() && magnitude > 0.0)
            .then(|| (idx, frequency, magnitude, 20.0 * magnitude.log10()))
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
/// The largest bin within `radius` bins of the fractional bin `center`.
fn largest_bin_near(fft: &FftData, center: f64, radius: usize) -> Option<usize> {
    if !center.is_finite() || center < 0.0 {
        return None;
    }
    let center = center.round() as usize;
    let last = fft.points.len().checked_sub(1)?;
    if center == 0 || center > last {
        return None;
    }
    let low = center.saturating_sub(radius).max(1);
    let high = center.saturating_add(radius).min(last);
    (low..=high)
        .filter(|bin| fft.points[*bin].magnitude > 0.0)
        .max_by(|a, b| {
            fft.points[*a]
                .magnitude
                .total_cmp(&fft.points[*b].magnitude)
        })
}

/// The mean of the powers whose levels, in dB, are `levels`, in dB.
fn mean_power_db(levels: &[f64]) -> Option<f64> {
    let peak = levels.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if levels.is_empty() || !peak.is_finite() {
        return None;
    }
    let sum = levels
        .iter()
        .map(|level| 10.0_f64.powf((level - peak) / 10.0))
        .sum::<f64>();
    Some(peak + 10.0 * (sum / levels.len() as f64).log10())
}

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
    use crate::fft::data::FftPoint;
    use crate::fft::data::SpectrumNormalization;

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
            padding: 1,
            averages: 1,
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

    /// A coherent record: `cycles` periods of `amplitude·sin` in `length`
    /// samples, with `harmonics` as (order, fraction of the amplitude, phase)
    /// and seeded white noise of `sigma`. Returns the samples and the noise's
    /// own mean power.
    fn coherent_record(
        length: usize,
        cycles: usize,
        amplitude: f64,
        harmonics: &[(usize, f64, f64)],
        sigma: f64,
        seed: u64,
    ) -> (Vec<f64>, f64) {
        let mut rng = Xorshift(seed);
        let noise: Vec<f64> = (0..length).map(|_| sigma * rng.next_gaussian()).collect();
        let noise_power = noise.iter().map(|v| v * v).sum::<f64>() / length as f64;
        let samples = (0..length)
            .map(|n| {
                let phase = std::f64::consts::TAU * cycles as f64 * n as f64 / length as f64;
                let tones = harmonics
                    .iter()
                    .map(|(order, fraction, offset)| {
                        fraction * (*order as f64 * phase + offset).sin()
                    })
                    .sum::<f64>();
                amplitude * (phase.sin() + tones) + noise[n]
            })
            .collect();
        (samples, noise_power)
    }

    fn spectrum(samples: &[f64], window: WindowFunction) -> FftData {
        FftData::from_time_domain_with_normalization(
            "V(out)",
            samples,
            samples.len() as f64,
            window,
            SpectrumNormalization::Rms,
        )
        .expect("a spectrum")
    }

    /// The Kaiser window measures at its default β = 38, whose sidelobes sit
    /// below rounding: a clean coherent tone reads SNR and SFDR over 250 dB,
    /// and a tone with HD2 1 %, HD3 0.1 % and 1 mV of noise reads numpy's
    /// SNR 10·log10(½/σ²) and SINAD 10·log10(½/(σ² + ½(0.01² + 0.001²))). At
    /// the β of 5 it was fixed at, every clean tone read SNR 41 dB.
    #[test]
    fn the_kaiser_window_measures_below_its_own_sidelobes() {
        let (clean, _) = coherent_record(4096, 64, 1.0, &[], 0.0, 1);
        let analysis = SpectrumAnalysis::analyze(&spectrum(&clean, WindowFunction::KAISER), 10)
            .expect("figures");
        assert!(analysis.snr_db.unwrap() > 250.0, "{analysis:?}");
        assert!(analysis.sfdr_db.unwrap() > 250.0, "{analysis:?}");
        assert!(analysis.thd_percent.unwrap() < 1.0e-10, "{analysis:?}");

        let (noisy, noise_power) =
            coherent_record(4096, 64, 1.0, &[(2, 0.01, 0.3), (3, 0.001, 1.1)], 1.0e-3, 7);
        let analysis = SpectrumAnalysis::analyze(&spectrum(&noisy, WindowFunction::KAISER), 10)
            .expect("figures");
        let snr = 10.0 * (0.5 / noise_power).log10();
        let sinad =
            10.0 * (0.5 / (noise_power + 0.5 * (0.01_f64.powi(2) + 0.001_f64.powi(2)))).log10();
        assert!(
            (analysis.snr_db.unwrap() - snr).abs() < 0.3,
            "{analysis:?} against {snr}"
        );
        assert!(
            (analysis.sinad_db.unwrap() - sinad).abs() < 0.1,
            "{analysis:?} against {sinad}"
        );
        assert!(
            (analysis.sfdr_db.unwrap() - 40.0).abs() < 0.05,
            "{analysis:?}"
        );
        let thd = 100.0 * (0.01_f64.powi(2) + 0.001_f64.powi(2)).sqrt();
        assert!(
            (analysis.thd_percent.unwrap() - thd).abs() < 0.01,
            "{analysis:?}"
        );
    }

    /// SINAD, ENOB and SFDR of a seeded ADC record against their closed
    /// forms. An ideal 12-bit quantizer on a full-scale coherent sine (127
    /// cycles, a prime, in 4096 samples) leaves noise of one LSB²/12, so
    /// SINAD is 10·log10((A²/2)/(1/12)) and ENOB 12 bits; the same record
    /// with HD3 at 1 % reads SFDR 40 dB.
    #[test]
    fn an_adc_record_reads_its_closed_form_sinad_enob_and_sfdr() {
        let amplitude = 2047.5;
        let (sine, _) = coherent_record(4096, 127, amplitude, &[], 0.0, 1);
        let quantized: Vec<f64> = sine.iter().map(|v| v.round()).collect();
        let analysis =
            SpectrumAnalysis::analyze(&spectrum(&quantized, WindowFunction::Rectangular), 10)
                .expect("figures");
        let sinad = 10.0 * (amplitude * amplitude / 2.0 * 12.0).log10();
        assert!(
            (analysis.sinad_db.unwrap() - sinad).abs() < 0.3,
            "{analysis:?} against {sinad}"
        );
        let enob = analysis.enob_bits().unwrap();
        assert!((enob - 12.0).abs() < 0.05, "ENOB {enob}");

        let (distorted, _) = coherent_record(4096, 127, amplitude, &[(3, 0.01, 0.4)], 0.0, 1);
        let quantized: Vec<f64> = distorted.iter().map(|v| v.round()).collect();
        let analysis =
            SpectrumAnalysis::analyze(&spectrum(&quantized, WindowFunction::Rectangular), 10)
                .expect("figures");
        assert!(
            (analysis.sfdr_db.unwrap() - 40.0).abs() < 0.01,
            "{analysis:?}"
        );
    }

    /// Through the conventional Gaussian window a tone between bins reads its
    /// true level, −3.0103 dBV rms for 1 V, and no distortion. Through the
    /// near-rectangle the window once was, it read −4.58 dBV and THD 0.53 %.
    #[test]
    fn the_gaussian_window_reads_a_tone_between_bins() {
        let samples: Vec<f64> = (0..4096)
            .map(|n| (std::f64::consts::TAU * 64.37 * n as f64 / 4096.0).sin())
            .collect();
        let analysis = SpectrumAnalysis::analyze(&spectrum(&samples, WindowFunction::Gaussian), 10)
            .expect("figures");
        let level = analysis.fundamental_db.unwrap();
        assert!((level + 3.0103).abs() < 0.01, "level {level}");
        assert!(analysis.thd_percent.unwrap() < 0.1, "{analysis:?}");
    }

    /// When the record cannot resolve a harmonic from the carrier, the
    /// harmonic is summed into the noise, so SNR is withheld with THD and
    /// SINAD. It once printed SINAD under the SNR label.
    #[test]
    fn an_unresolved_harmonic_withholds_snr_with_thd_and_sinad() {
        let (samples, _) = coherent_record(8192, 8, 1.0, &[(3, 0.01, 0.0)], 0.0, 1);
        let analysis =
            SpectrumAnalysis::analyze(&spectrum(&samples, WindowFunction::BlackmanHarris), 10)
                .expect("figures");
        assert!(
            matches!(
                analysis.harmonic_coverage,
                HarmonicCoverage::InsufficientResolution {
                    failed_order: 2,
                    ..
                }
            ),
            "{analysis:?}"
        );
        assert_eq!(analysis.snr_db, None);
        assert_eq!(analysis.sinad_db, None);
        assert_eq!(analysis.thd_percent, None);
        assert!(
            (analysis.sfdr_db.unwrap() - 40.0).abs() < 0.05,
            "{analysis:?}"
        );
    }

    /// The noise floor is the mean noise power per bin, 10·log10(2σ²/N) for
    /// white noise of power σ² in an rms spectrum of N samples: −93.11 dBV
    /// at 1 mV and N = 4096. It was the median of the bins' levels, 1.6 dB
    /// under the mean, since a noise bin's power is exponential.
    #[test]
    fn the_noise_floor_is_the_mean_noise_power_per_bin() {
        let (samples, noise_power) = coherent_record(4096, 64, 1.0, &[], 1.0e-3, 11);
        let analysis =
            SpectrumAnalysis::analyze(&spectrum(&samples, WindowFunction::Rectangular), 10)
                .expect("figures");
        let floor = 10.0 * (2.0 * noise_power / 4096.0).log10();
        let measured = analysis.noise_floor_db.unwrap();
        assert!(
            (measured - floor).abs() < 0.25,
            "floor {measured} against {floor}"
        );
    }

    /// Through the rectangular window a coherent tone's bin is exact and its
    /// neighbours are noise, so the carrier is read from its bin: level and
    /// frequency are the bin's own, whatever the noise. Interpolating through
    /// the noise bins moved the level up to 0.13 dB.
    #[test]
    fn a_rectangular_coherent_carrier_is_read_from_its_bin() {
        for seed in 1..=8 {
            let (samples, _) = coherent_record(4096, 127, 1.0, &[], 1.0e-3, seed);
            let fft = spectrum(&samples, WindowFunction::Rectangular);
            let analysis = SpectrumAnalysis::analyze(&fft, 10).expect("figures");
            assert_eq!(analysis.fundamental_bin, Some(127));
            assert_eq!(
                analysis.fundamental_frequency,
                Some(fft.points[127].frequency)
            );
            assert_eq!(
                analysis.fundamental_db,
                Some(fft.points[127].magnitude_db())
            );
        }
    }

    /// A fundamental the reader names places the harmonics; a band ends the
    /// harmonic series and the noise where it ends.
    #[test]
    fn a_named_fundamental_and_a_band_are_measured_as_set() {
        let (samples, _) =
            coherent_record(4096, 64, 1.0, &[(2, 0.01, 0.0), (5, 0.02, 0.0)], 1.0e-4, 3);
        let fft = spectrum(&samples, WindowFunction::BlackmanHarris);
        // A larger interferer than the carrier at bin 400 would be taken for
        // the fundamental; the reader names 64 Hz instead.
        let mut interfered = samples.clone();
        for (n, value) in interfered.iter_mut().enumerate() {
            *value += 3.0 * (std::f64::consts::TAU * 400.0 * n as f64 / 4096.0).sin();
        }
        let named = SpectrumAnalysis::measure(
            &spectrum(&interfered, WindowFunction::BlackmanHarris),
            &SpectrumMeasurement {
                fundamental: Some(64.0),
                band: Some((0.0, 1000.0)),
                ..SpectrumMeasurement::with_harmonics(10)
            },
        )
        .expect("figures");
        assert_eq!(named.fundamental_bin, Some(64));
        let banded = SpectrumAnalysis::measure(
            &fft,
            &SpectrumMeasurement {
                band: Some((0.0, 200.0)),
                ..SpectrumMeasurement::with_harmonics(10)
            },
        )
        .expect("figures");
        // HD5 at 320 Hz lies past the band: only HD2 and HD3 are counted.
        assert_eq!(
            banded.harmonics.iter().map(|h| h.order).collect::<Vec<_>>(),
            [2, 3]
        );
        assert!(
            (banded.thd_percent.unwrap() - 1.0).abs() < 0.01,
            "{banded:?}"
        );
        let explicit = SpectrumAnalysis::measure(
            &fft,
            &SpectrumMeasurement {
                signal_bins: Some(6),
                ..SpectrumMeasurement::with_harmonics(10)
            },
        )
        .expect("figures");
        assert_eq!(explicit.signal_bins, 6);
    }

    fn transformed(samples: &[f64], padding: usize, averages: usize) -> FftData {
        FftData::from_record(
            "V(out)",
            samples,
            samples.len() as f64,
            &crate::fft::data::SpectrumTransform {
                padding,
                averages,
                ..crate::fft::data::SpectrumTransform::new(
                    WindowFunction::Hanning,
                    SpectrumNormalization::Rms,
                )
            },
        )
        .expect("a spectrum")
    }

    /// Padding draws the same spectrum on a finer grid: the padded transform
    /// of a coherent tone through Hann passes through every bin of the
    /// unpadded one, at its level, and peaks at the tone's −3.0103 dBV rms.
    #[test]
    fn a_padded_record_draws_the_record_on_a_finer_grid() {
        let (samples, _) = coherent_record(4096, 64, 1.0, &[(3, 0.01, 0.2)], 1.0e-3, 5);
        let plain = transformed(&samples, 1, 1);
        let padded = transformed(&samples, 4, 1);
        assert_eq!(padded.frequency_resolution(), 0.25);
        for (bin, point) in plain.points.iter().enumerate() {
            let fine = &padded.points[4 * bin];
            assert_eq!(fine.frequency, point.frequency);
            assert!(
                (fine.magnitude - point.magnitude).abs() <= 1.0e-12 * plain.points[64].magnitude,
                "bin {bin}: {} against {}",
                fine.magnitude,
                point.magnitude
            );
        }
        let level = padded.points[256].magnitude_db();
        assert!((level + 3.010299956639812).abs() < 1.0e-3, "level {level}");
    }

    /// Averaging the powers of eight records of a tone in noise reads the
    /// tone's level and numpy's SNR of the whole record, and the noise floor
    /// of one record of an eighth the length, 10·log10(2σ²·ENBW/1024) through
    /// Hann.
    #[test]
    fn averaged_records_read_the_tone_and_noise_of_the_record() {
        let (samples, noise_power) = coherent_record(8192, 256, 1.0, &[], 1.0e-3, 9);
        let fft = transformed(&samples, 1, 8);
        assert_eq!((fft.fft_size, fft.averages()), (1024, 8));
        let analysis = SpectrumAnalysis::analyze(&fft, 10).expect("figures");
        let level = analysis.fundamental_db.unwrap();
        assert!((level + 3.010299956639812).abs() < 0.01, "level {level}");
        let snr = 10.0 * (0.5 / noise_power).log10();
        assert!(
            (analysis.snr_db.unwrap() - snr).abs() < 0.2,
            "{analysis:?} against {snr}"
        );
        let floor =
            10.0 * (2.0 * noise_power * fft.equivalent_noise_bandwidth_bins() / 1024.0).log10();
        let measured = analysis.noise_floor_db.unwrap();
        assert!(
            (measured - floor).abs() < 0.2,
            "floor {measured} against {floor}"
        );
    }

    /// Each harmonic stands at its order times the fundamental and is read
    /// at its own bin: HD2 at 1 % and HD3 at 0.1 % rise out of 1 mV of
    /// noise, while HD4 to HD10, which the record does not hold, read the
    /// floor and say so rather than a noise bin's level at a noise bin's
    /// frequency, and THD sums HD2 and HD3 alone.
    #[test]
    fn a_harmonic_stands_at_its_order_and_one_under_the_floor_says_so() {
        let (samples, _) =
            coherent_record(4096, 64, 1.0, &[(2, 0.01, 0.3), (3, 0.001, 1.1)], 1.0e-3, 7);
        let analysis = SpectrumAnalysis::analyze(&spectrum(&samples, WindowFunction::KAISER), 10)
            .expect("figures");
        let fundamental = analysis.fundamental_frequency.expect("a fundamental");
        for harmonic in &analysis.harmonics {
            assert_eq!(
                harmonic.frequency,
                fundamental * harmonic.order as f64,
                "{harmonic:?}"
            );
            assert_eq!(harmonic.above_floor, harmonic.order <= 3, "{harmonic:?}");
        }
        assert_eq!(analysis.harmonics.len(), 9);
        let fundamental_db = analysis.fundamental_db.expect("a level");
        let hd2 = analysis.harmonics[0].level_db - fundamental_db;
        assert!((hd2 + 40.0).abs() < 0.05, "HD2 {hd2} dBc");
        let thd_of = |counted: &dyn Fn(&HarmonicMeasurement) -> bool| {
            100.0
                * analysis
                    .harmonics
                    .iter()
                    .filter(|harmonic| counted(harmonic))
                    .map(|harmonic| 10f64.powf((harmonic.level_db - fundamental_db) / 10.0))
                    .sum::<f64>()
                    .sqrt()
        };
        let thd = analysis.thd_percent.expect("THD");
        let above = thd_of(&|harmonic| harmonic.above_floor);
        let every = thd_of(&|_| true);
        assert!(
            (thd - above).abs() <= 1.0e-9 * above,
            "THD {thd} against {above}"
        );
        assert!(
            (every - above).abs() > 1.0e-9 * above,
            "the floor reads nothing"
        );
    }
}
