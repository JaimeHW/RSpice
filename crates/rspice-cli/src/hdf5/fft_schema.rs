//! FFT descriptors and validation, independent of HDF5 transport.
use super::*;

/// Canonical run-axis identity attached to an FFT artifact. Scalar runs omit
/// this object rather than inventing a synthetic coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hdf5FftCoordinate {
    pub coordinate_id: String,
    pub ordinal: usize,
    pub tag: String,
    pub assignment: String,
}

/// One magnitude-ranked harmonic retained by `.OPTIONS FFT FFTOUT=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5FftHarmonic {
    pub rank: usize,
    pub bin: usize,
    pub frequency_hz: f64,
    pub magnitude: f64,
    pub magnitude_db: f64,
    pub phase_degrees: f64,
}

/// Typed optional FFT metric payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5FftMetrics {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency_hz: Option<f64>,
    pub largest_harmonics: Vec<Hdf5FftHarmonic>,
}

/// Complete typed representation of one source-authored transient `.FFT`.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5FftResult {
    pub status: rspice_core::engine::TransientFftStatus,
    pub analysis_id: String,
    pub ordinal: usize,
    pub source_kind: String,
    pub source_text: String,
    pub authored_output: String,
    pub output_name: String,
    pub physical_type: String,
    /// Effective unit of Cartesian coefficients, magnitudes, and
    /// magnitude-like metrics. Normalized spectra use `1` while
    /// `physical_type` retains source provenance.
    pub value_unit: Option<String>,
    pub start_time_s: f64,
    pub stop_time_s: f64,
    pub sample_interval_s: f64,
    pub point_count: usize,
    pub accurate_sampling: bool,
    pub format: String,
    pub mode: String,
    pub window: String,
    pub window_name: String,
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution_hz: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    /// First bin included in SFDR spur selection. This preserves whether an
    /// authored FMIN overrode the core's default search starting at FREQ.
    pub sfdr_search_minimum_bin: usize,
    pub bin_indices: Vec<u64>,
    pub frequency_hz: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub magnitude: Vec<f64>,
    pub phase_degrees: Vec<f64>,
    pub metrics: Option<Hdf5FftMetrics>,
}

/// One atomic HDF5 FFT artifact, containing every directive evaluated by one
/// parent transient in exact source order.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5FftSection {
    pub parent_analysis_id: String,
    pub coordinate: Option<Hdf5FftCoordinate>,
    pub results: Vec<Hdf5FftResult>,
}

pub(super) const FFT_DB_NOISE_FLOOR: f64 = 1.0e-10;
const FFT_MAX_RANKED_HARMONICS: usize = 30;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FftMetricExpectations {
    pub(crate) fundamental_magnitude: f64,
    pub(crate) thd_ratio: f64,
    pub(crate) thd_db: f64,
    pub(crate) sndr_db: f64,
    pub(crate) enob_bits: f64,
    pub(crate) snr_db: f64,
    pub(crate) sfdr_db: f64,
    pub(crate) sfdr_spur_bin: Option<usize>,
    pub(crate) ranked_bins: Vec<usize>,
}

pub(crate) fn fft_values_close(actual: f64, expected: f64) -> bool {
    if actual == expected {
        return true;
    }
    let scale = actual.abs().max(expected.abs());
    let tolerance = 128.0 * f64::EPSILON * scale;
    (actual - expected).abs() <= tolerance
}

pub(crate) fn fft_phase_distance_degrees(actual: f64, expected: f64) -> f64 {
    let delta = (actual - expected).rem_euclid(360.0);
    delta.min(360.0 - delta)
}

pub(crate) fn fft_source_identity_is_valid(kind: &str, text: &str, authored: &str) -> bool {
    if text.is_empty() || authored.is_empty() {
        return false;
    }
    match kind {
        "probe" => authored == text,
        "expression" => {
            authored
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
                == Some(text)
        }
        _ => false,
    }
}

pub(crate) fn fft_metric_expectations(
    magnitudes: &[f64],
    fundamental_bin: usize,
    maximum_metric_bin: usize,
    sfdr_search_minimum_bin: usize,
) -> Option<FftMetricExpectations> {
    let fundamental_magnitude = *magnitudes.get(fundamental_bin)?;
    if !fundamental_magnitude.is_finite() || fundamental_magnitude <= FFT_DB_NOISE_FLOOR {
        return None;
    }

    let mut distortion_power = 0.0;
    for bin in
        (fundamental_bin.saturating_mul(2)..=maximum_metric_bin).step_by(fundamental_bin.max(1))
    {
        distortion_power += magnitudes.get(bin)?.powi(2);
    }
    let thd_ratio = distortion_power.sqrt() / fundamental_magnitude;
    let thd_db = 20.0 * thd_ratio.max(FFT_DB_NOISE_FLOOR).log10();

    let noise_and_distortion_power = magnitudes
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(bin, _)| *bin != fundamental_bin)
        .map(|(_, magnitude)| magnitude.powi(2))
        .sum::<f64>();
    let sndr_denominator = noise_and_distortion_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let sndr_db = 20.0 * (fundamental_magnitude / sndr_denominator).log10();
    let enob_bits = (sndr_db - 1.76) / 6.02;

    let signal_frequency_limit = maximum_metric_bin.max(fundamental_bin);
    let noise_power = magnitudes
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(bin, _)| *bin % fundamental_bin != 0 || *bin > signal_frequency_limit)
        .map(|(_, magnitude)| magnitude.powi(2))
        .sum::<f64>();
    let snr_ratio = fundamental_magnitude / noise_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let snr_db = 20.0 * snr_ratio.log10();

    let mut sfdr_spur_bin = None;
    for bin in sfdr_search_minimum_bin..=maximum_metric_bin {
        if bin != fundamental_bin
            && magnitudes[bin] > sfdr_spur_bin.map_or(0.0, |spur| magnitudes[spur])
        {
            sfdr_spur_bin = Some(bin);
        }
    }
    let sfdr_spur_magnitude = sfdr_spur_bin.map_or(0.0, |bin| magnitudes[bin]);
    let sfdr_db =
        20.0 * (fundamental_magnitude / sfdr_spur_magnitude.max(FFT_DB_NOISE_FLOOR)).log10();

    let ranked_len = magnitudes
        .len()
        .saturating_sub(1)
        .min(FFT_MAX_RANKED_HARMONICS);
    let mut ranked_bins = Vec::with_capacity(ranked_len);
    for bin in 1..magnitudes.len() {
        let position = ranked_bins
            .iter()
            .position(|retained| {
                magnitudes[bin] > magnitudes[*retained]
                    || (magnitudes[bin] == magnitudes[*retained] && bin < *retained)
            })
            .unwrap_or(ranked_bins.len());
        if ranked_bins.len() < ranked_len {
            ranked_bins.insert(position, bin);
        } else if position < ranked_len {
            ranked_bins.pop();
            ranked_bins.insert(position, bin);
        }
    }

    Some(FftMetricExpectations {
        fundamental_magnitude,
        thd_ratio,
        thd_db,
        sndr_db,
        enob_bits,
        snr_db,
        sfdr_db,
        sfdr_spur_bin,
        ranked_bins,
    })
}

impl Hdf5FftSection {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.parent_analysis_id.is_empty() {
            return Err(Hdf5Error::InvalidSchema(
                "FFT parent_analysis_id must not be empty".to_string(),
            ));
        }
        if let Some(coordinate) = &self.coordinate
            && (coordinate.coordinate_id.is_empty()
                || coordinate.ordinal == 0
                || coordinate.tag.is_empty()
                || coordinate.assignment.is_empty())
        {
            return Err(Hdf5Error::InvalidSchema(
                "FFT coordinate identity fields must be complete".to_string(),
            ));
        }
        if self.results.is_empty() {
            return Err(Hdf5Error::InvalidSchema(
                "FFT section must contain at least one result".to_string(),
            ));
        }
        // The identity of each authored `.FFT` request comes from the
        // canonical planner, so a decoded section is checked against the same
        // minting the writer used rather than a second spelling of it.
        let canonical_ids = crate::commands::run::canonical_analysis_identities(
            rspice_core::execution::AnalysisKind::Fft,
            self.results.len(),
        )
        .map_err(|error| {
            Hdf5Error::InvalidSchema(format!("cannot mint canonical FFT identities: {error}"))
        })?;
        for (index, result) in self.results.iter().enumerate() {
            let expected = canonical_ids.get(index).ok_or_else(|| {
                Hdf5Error::InvalidSchema(format!(
                    "FFT section has no canonical identity for result {}",
                    index.saturating_add(1)
                ))
            })?;
            result.validate(index + 1, &expected.tag())?;
        }
        Ok(())
    }
}

impl Hdf5FftResult {
    pub(super) fn validate(&self, expected_ordinal: usize, expected_analysis_id: &str) -> Result<()> {
        self.status
            .validate_history(self.start_time_s, self.stop_time_s, self.point_count)
            .map_err(Hdf5Error::InvalidSchema)?;
        if !self.status.is_complete() && self.metrics.is_some() {
            return Err(Hdf5Error::InvalidSchema(
                "incomplete FFT contains computed metrics".into(),
            ));
        }
        if self.ordinal != expected_ordinal || self.analysis_id != expected_analysis_id {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result {} does not match source-order identity {expected_analysis_id}",
                self.analysis_id
            )));
        }
        if !fft_source_identity_is_valid(
            &self.source_kind,
            &self.source_text,
            &self.authored_output,
        ) || self.output_name.is_empty()
            || self.physical_type.is_empty()
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' has incomplete source or signal metadata",
                self.analysis_id
            )));
        }
        let physical_unit = match self.physical_type.as_str() {
            "voltage" => Some("V"),
            "current" => Some("A"),
            "parameter" => None,
            other => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' has unsupported physical type '{other}'",
                    self.analysis_id
                )));
            }
        };
        let expected_unit = match self.format.as_str() {
            "normalized" => Some("1"),
            "unnormalized" => physical_unit,
            _ => None,
        };
        if self.value_unit.as_deref() != expected_unit
            || !matches!(self.format.as_str(), "normalized" | "unnormalized")
            || !matches!(
                self.mode.as_str(),
                "hspice_compatible" | "spectre_compatible"
            )
            || !matches!(
                self.window.as_str(),
                "rectangular"
                    | "bartlett"
                    | "bartlett_hann"
                    | "hamming"
                    | "hann"
                    | "blackman_67db"
                    | "blackman"
                    | "blackman_harris"
                    | "nuttall"
                    | "half_cycle_sine"
                    | "half_cycle_sine_3"
                    | "half_cycle_sine_6"
                    | "cosine_2"
                    | "cosine_4"
                    | "gaussian"
                    | "kaiser"
            )
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' has inconsistent units or transform enums",
                self.analysis_id
            )));
        }
        let bin_count = self.bin_indices.len();
        for (name, count) in [
            ("frequency_hz", self.frequency_hz.len()),
            ("real", self.real.len()),
            ("imaginary", self.imaginary.len()),
            ("magnitude", self.magnitude.len()),
            ("phase_degrees", self.phase_degrees.len()),
        ] {
            if count != bin_count {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' has {count} {name} values for {bin_count} bins",
                    self.analysis_id
                )));
            }
        }
        let configured_bin_count = self.point_count / 2 + 1;
        let expected_bins = if self.status.is_complete() {
            configured_bin_count
        } else {
            0
        };
        if self.point_count < 4 || !self.point_count.is_power_of_two() || bin_count != expected_bins
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' has {bin_count} bins for {} input points",
                self.analysis_id, self.point_count
            )));
        }
        if self.fundamental_bin >= configured_bin_count
            || self.fundamental_bin == 0
            || self.minimum_metric_bin >= configured_bin_count
            || self.maximum_metric_bin >= configured_bin_count
            || self.minimum_metric_bin > self.maximum_metric_bin
            || self.sfdr_search_minimum_bin >= configured_bin_count
            || self.sfdr_search_minimum_bin > self.maximum_metric_bin
            || !matches!(
                self.sfdr_search_minimum_bin,
                value if value == self.minimum_metric_bin || value == self.fundamental_bin
            )
            || (self.fundamental_bin == 1 && self.maximum_metric_bin < 2)
            || (self.fundamental_bin > 1 && self.maximum_metric_bin < 1)
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' metric bin bounds exceed its spectrum",
                self.analysis_id
            )));
        }
        for (expected, actual) in self.bin_indices.iter().copied().enumerate() {
            let expected_index = u64::try_from(expected).map_err(|_| {
                Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' bin index {expected} cannot be represented",
                    self.analysis_id
                ))
            })?;
            if actual != expected_index {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' bin index {actual} is out of order at {expected}",
                    self.analysis_id
                )));
            }
        }
        if ![
            self.start_time_s,
            self.stop_time_s,
            self.sample_interval_s,
            self.alpha,
            self.coherent_gain,
            self.frequency_resolution_hz,
        ]
        .iter()
        .all(|value| value.is_finite())
            || self.stop_time_s <= self.start_time_s
            || self.sample_interval_s <= 0.0
            || self.frequency_resolution_hz <= 0.0
            || self
                .frequency_hz
                .iter()
                .chain(&self.real)
                .chain(&self.imaginary)
                .chain(&self.magnitude)
                .chain(&self.phase_degrees)
                .any(|value| !value.is_finite())
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' contains invalid numeric metadata or bins",
                self.analysis_id
            )));
        }
        for bin in 0..bin_count {
            let expected_frequency = bin as f64 * self.frequency_resolution_hz;
            let derived_magnitude = self.real[bin].hypot(self.imaginary[bin]);
            if self.magnitude[bin] < 0.0
                || !fft_values_close(self.frequency_hz[bin], expected_frequency)
                || !fft_values_close(self.magnitude[bin], derived_magnitude)
                || (derived_magnitude > 1.0e-14
                    && fft_phase_distance_degrees(
                        self.phase_degrees[bin],
                        self.imaginary[bin].atan2(self.real[bin]).to_degrees(),
                    ) > 1.0e-9)
            {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' has inconsistent bin {bin}",
                    self.analysis_id
                )));
            }
        }
        if self.format == "normalized" {
            let maximum_magnitude = self.magnitude.iter().copied().fold(0.0, f64::max);
            if maximum_magnitude != 0.0 && !fft_values_close(maximum_magnitude, 1.0) {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{}' normalized spectrum does not peak at 1",
                    self.analysis_id
                )));
            }
        }
        if let Some(metrics) = &self.metrics {
            metrics.validate(self)?;
        }
        Ok(())
    }
}

impl Hdf5FftMetrics {
    fn validate(&self, result: &Hdf5FftResult) -> Result<()> {
        let analysis_id = &result.analysis_id;
        let bin_count = result.bin_indices.len();
        if ![
            self.fundamental_magnitude,
            self.thd_ratio,
            self.thd_db,
            self.sndr_db,
            self.enob_bits,
            self.snr_db,
            self.sfdr_db,
        ]
        .iter()
        .all(|value| value.is_finite())
            || self
                .sfdr_spur_frequency_hz
                .is_some_and(|value| !value.is_finite())
            || self.sfdr_spur_bin.is_some() != self.sfdr_spur_frequency_hz.is_some()
            || self.sfdr_spur_bin.is_some_and(|bin| bin >= bin_count)
            || self.largest_harmonics.len() > FFT_MAX_RANKED_HARMONICS
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{analysis_id}' has invalid metric scalars"
            )));
        }
        let expected = fft_metric_expectations(
            &result.magnitude,
            result.fundamental_bin,
            result.maximum_metric_bin,
            result.sfdr_search_minimum_bin,
        )
        .ok_or_else(|| {
            Hdf5Error::InvalidSchema(format!(
                "FFT result '{analysis_id}' cannot produce valid metrics"
            ))
        })?;
        let spur_frequency_matches = match (
            self.sfdr_spur_frequency_hz,
            expected.sfdr_spur_bin.map(|bin| result.frequency_hz[bin]),
        ) {
            (Some(actual), Some(expected)) => fft_values_close(actual, expected),
            (None, None) => true,
            _ => false,
        };
        if !fft_values_close(self.fundamental_magnitude, expected.fundamental_magnitude)
            || !fft_values_close(self.thd_ratio, expected.thd_ratio)
            || !fft_values_close(self.thd_db, expected.thd_db)
            || !fft_values_close(self.sndr_db, expected.sndr_db)
            || !fft_values_close(self.enob_bits, expected.enob_bits)
            || !fft_values_close(self.snr_db, expected.snr_db)
            || !fft_values_close(self.sfdr_db, expected.sfdr_db)
            || self.sfdr_spur_bin != expected.sfdr_spur_bin
            || !spur_frequency_matches
            || self.largest_harmonics.len() != expected.ranked_bins.len()
        {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{analysis_id}' metrics do not match its spectrum"
            )));
        }
        for (index, (harmonic, expected_bin)) in self
            .largest_harmonics
            .iter()
            .zip(expected.ranked_bins)
            .enumerate()
        {
            if harmonic.rank != index + 1
                || harmonic.bin != expected_bin
                || ![
                    harmonic.frequency_hz,
                    harmonic.magnitude,
                    harmonic.magnitude_db,
                    harmonic.phase_degrees,
                ]
                .iter()
                .all(|value| value.is_finite())
                || !fft_values_close(harmonic.frequency_hz, result.frequency_hz[expected_bin])
                || !fft_values_close(harmonic.magnitude, result.magnitude[expected_bin])
                || !fft_values_close(
                    harmonic.magnitude_db,
                    20.0 * result.magnitude[expected_bin]
                        .max(FFT_DB_NOISE_FLOOR)
                        .log10(),
                )
                || fft_phase_distance_degrees(
                    harmonic.phase_degrees,
                    result.phase_degrees[expected_bin],
                ) > 1.0e-9
            {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "FFT result '{analysis_id}' has an invalid ranked harmonic at position {}",
                    index + 1
                )));
            }
        }
        Ok(())
    }
}

