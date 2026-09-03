//! HDF5 storage for simulation results.
//!
//! The CLI uses a stable, self-describing layout with path-safe dataset names
//! so exported files remain robust even when signal names contain SPICE syntax.

use crate::commands::publish;
use rspice_output::AtomicArtifactError;
use rustyhdf5::{AttrValue, File as Hdf5File, FileBuilder};
use thiserror::Error;

use std::collections::HashMap;
use std::path::Path;

const SCHEMA_VERSION: &str = "1";
const FFT_SECTION_SCHEMA_VERSION: &str = "2";

#[derive(Debug, Error)]
pub enum Hdf5Error {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error("invalid HDF5 schema: {0}")]
    InvalidSchema(String),
    #[error("failed to prepare staged HDF5 artifact: {0}")]
    ArtifactPreparation(#[source] std::io::Error),
    #[error("failed while writing staged HDF5 artifact: {0}")]
    ArtifactWrite(#[source] std::io::Error),
    #[error("failed to flush staged HDF5 artifact: {0}")]
    ArtifactFlush(#[source] std::io::Error),
    #[error("failed to atomically commit staged HDF5 artifact: {0}")]
    ArtifactCommit(#[source] std::io::Error),
}

#[derive(Debug, Error)]
enum Hdf5StagingError {
    #[error(transparent)]
    Backend(#[from] rustyhdf5::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Hdf5Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Signal {
    pub name: String,
    pub var_type: String,
    pub values: Vec<f64>,
}

impl Hdf5Signal {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self::new_typed(name, "value", values)
    }

    pub fn new_typed(
        name: impl Into<String>,
        var_type: impl Into<String>,
        values: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            var_type: var_type.into(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5WaveformSection {
    pub independent_name: String,
    pub independent_values: Vec<f64>,
    pub signals: Vec<Hdf5Signal>,
}

impl Hdf5WaveformSection {
    pub fn new(independent_name: impl Into<String>, independent_values: Vec<f64>) -> Self {
        Self {
            independent_name: independent_name.into(),
            independent_values,
            signals: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, name: impl Into<String>, values: Vec<f64>) {
        self.signals.push(Hdf5Signal::new(name, values));
    }

    pub fn add_typed_signal(
        &mut self,
        name: impl Into<String>,
        var_type: impl Into<String>,
        values: Vec<f64>,
    ) {
        self.signals
            .push(Hdf5Signal::new_typed(name, var_type, values));
    }

    fn validate(&self, section_name: &str) -> Result<()> {
        for signal in &self.signals {
            if signal.values.len() != self.independent_values.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "{section_name} signal '{}' has {} points, expected {}",
                    signal.name,
                    signal.values.len(),
                    self.independent_values.len()
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5ComplexSignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl Hdf5ComplexSignal {
    pub fn new(name: impl Into<String>, real: Vec<f64>, imag: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            real,
            imag,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5AcSection {
    pub frequency: Vec<f64>,
    pub signals: Vec<Hdf5ComplexSignal>,
}

impl Hdf5AcSection {
    pub fn new(frequency: Vec<f64>) -> Self {
        Self {
            frequency,
            signals: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, name: impl Into<String>, real: Vec<f64>, imag: Vec<f64>) {
        self.signals.push(Hdf5ComplexSignal::new(name, real, imag));
    }

    fn validate(&self) -> Result<()> {
        for signal in &self.signals {
            if signal.real.len() != self.frequency.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "AC signal '{}' real part has {} points, expected {}",
                    signal.name,
                    signal.real.len(),
                    self.frequency.len()
                )));
            }
            if signal.imag.len() != self.frequency.len() {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "AC signal '{}' imaginary part has {} points, expected {}",
                    signal.name,
                    signal.imag.len(),
                    self.frequency.len()
                )));
            }
        }
        Ok(())
    }
}

/// One complex signal within an F1 fundamental, F2 fundamental, or nonlinear
/// product series in a `.DISTO` result.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSignal {
    pub name: String,
    pub var_type: String,
    /// Actual sinusoidal peak phasor components, never an internal Volterra
    /// kernel.
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
    pub magnitude: Vec<f64>,
    pub phase_degrees: Vec<f64>,
    /// Magnitude divided by the F1 magnitude for the same signal. F1 itself
    /// has no ratio because it is the normalization reference.
    pub magnitude_ratio_to_f1: Option<Vec<f64>>,
}

/// A stable spectral identity and its response across the swept F1 values.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSeries {
    /// `f1`, `f2`, `2f1`, `3f1`, `f1+f2`, `f1-f2`, or `2f1-f2`.
    pub label: String,
    pub is_product: bool,
    /// Physical output frequency for every swept F1 row.
    pub physical_frequency: Vec<f64>,
    pub signals: Vec<Hdf5DistortionSignal>,
}

/// Typed third-order Volterra distortion results.
///
/// This is an additive schema-v1 section. Older readers that ignore unknown
/// root groups remain compatible; updated readers retain product identity,
/// phasor convention, and normalization provenance without pretending the
/// result is an ordinary AC sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5DistortionSection {
    pub mode: String,
    pub f2_over_f1: Option<f64>,
    pub phasor_convention: String,
    pub ratio_normalization: String,
    pub f1_frequency: Vec<f64>,
    pub series: Vec<Hdf5DistortionSeries>,
}

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

const FFT_DB_NOISE_FLOOR: f64 = 1.0e-10;
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
    fn validate(&self) -> Result<()> {
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
        let canonical_ids = crate::analysis_identity::post_process_ids(
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
    fn validate(&self, expected_ordinal: usize, expected_analysis_id: &str) -> Result<()> {
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
        if self.point_count == 0 || bin_count != self.point_count / 2 + 1 {
            return Err(Hdf5Error::InvalidSchema(format!(
                "FFT result '{}' has {bin_count} bins for {} input points",
                self.analysis_id, self.point_count
            )));
        }
        if self.fundamental_bin >= bin_count
            || self.fundamental_bin == 0
            || self.minimum_metric_bin >= bin_count
            || self.maximum_metric_bin >= bin_count
            || self.minimum_metric_bin > self.maximum_metric_bin
            || self.sfdr_search_minimum_bin >= bin_count
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

impl Hdf5DistortionSection {
    fn validate(&self) -> Result<()> {
        if self.mode != "harmonic" && self.mode != "two_tone" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "distortion mode must be 'harmonic' or 'two_tone', got '{}'",
                self.mode
            )));
        }
        if self.phasor_convention != "actual_sinusoidal_peak" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "unsupported distortion phasor convention '{}'",
                self.phasor_convention
            )));
        }
        if self.ratio_normalization != "magnitude_over_same_signal_f1_magnitude" {
            return Err(Hdf5Error::InvalidSchema(format!(
                "unsupported distortion ratio normalization '{}'",
                self.ratio_normalization
            )));
        }
        match (self.mode.as_str(), self.f2_over_f1) {
            ("harmonic", None) => {}
            ("two_tone", Some(ratio)) if ratio.is_finite() && ratio > 0.0 && ratio < 1.0 => {}
            ("harmonic", Some(ratio)) => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "harmonic distortion section unexpectedly has f2_over_f1={ratio}"
                )));
            }
            ("two_tone", ratio) => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "two-tone distortion section requires 0 < f2_over_f1 < 1, got {ratio:?}"
                )));
            }
            _ => unreachable!("mode was validated above"),
        }
        if self.f1_frequency.is_empty() {
            return Err(Hdf5Error::InvalidSchema(
                "distortion section has no F1 frequencies".to_string(),
            ));
        }

        let count = self.f1_frequency.len();
        let expected_series: &[(&str, bool)] = if self.mode == "two_tone" {
            &[
                ("f1", false),
                ("f2", false),
                ("f1+f2", true),
                ("f1-f2", true),
                ("2f1-f2", true),
            ]
        } else {
            &[("f1", false), ("2f1", true), ("3f1", true)]
        };
        if self.series.len() != expected_series.len() {
            return Err(Hdf5Error::InvalidSchema(format!(
                "{} distortion section has {} spectral series, expected {}",
                self.mode,
                self.series.len(),
                expected_series.len()
            )));
        }

        for (series, &(expected_label, expected_product)) in self.series.iter().zip(expected_series)
        {
            if series.label != expected_label || series.is_product != expected_product {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "distortion series expected label '{expected_label}' with is_product={expected_product}, got '{}' with is_product={}",
                    series.label, series.is_product
                )));
            }
            if series.physical_frequency.len() != count {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "distortion series '{}' has {} frequencies, expected {count}",
                    series.label,
                    series.physical_frequency.len()
                )));
            }
            if series.label == "f1" && series.physical_frequency != self.f1_frequency {
                return Err(Hdf5Error::InvalidSchema(
                    "distortion F1 series frequency does not match the independent F1 scale"
                        .to_string(),
                ));
            }
            for signal in &series.signals {
                for (quantity, actual) in [
                    ("real", signal.real.len()),
                    ("imaginary", signal.imag.len()),
                    ("magnitude", signal.magnitude.len()),
                    ("phase", signal.phase_degrees.len()),
                ] {
                    if actual != count {
                        return Err(Hdf5Error::InvalidSchema(format!(
                            "distortion series '{}' signal '{}' {quantity} data has {actual} points, expected {count}",
                            series.label, signal.name
                        )));
                    }
                }
                if let Some(ratio) = &signal.magnitude_ratio_to_f1
                    && ratio.len() != count
                {
                    return Err(Hdf5Error::InvalidSchema(format!(
                        "distortion series '{}' signal '{}' ratio data has {} points, expected {count}",
                        series.label,
                        signal.name,
                        ratio.len()
                    )));
                }
                if (series.label == "f1") != signal.magnitude_ratio_to_f1.is_none() {
                    return Err(Hdf5Error::InvalidSchema(format!(
                        "distortion series '{}' signal '{}' has inconsistent F1 normalization provenance",
                        series.label, signal.name
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Measurement {
    pub name: String,
    pub value: f64,
}

impl Hdf5Measurement {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Identity one HDF5 document publishes under.
///
/// A `run` artifact names the canonical analysis instance that produced it,
/// and, for an axis coordinate, the coordinate and topology it belongs to. A
/// document `convert` produced from a file that declared none carries none.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5ResultIdentity {
    /// Canonical `AnalysisInstanceId::tag()`, which is also the group name.
    pub analysis_id: String,
    pub coordinate_id: Option<String>,
    pub coordinate_tag: Option<String>,
    pub coordinate_assignment: Option<String>,
    pub topology_fingerprint: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5SimulationData {
    pub title: String,
    /// The analysis instance this document publishes. When present, the
    /// section group is named by it instead of by the result family, so two
    /// `.AC` cards cannot collide in one file and a reader can tell which card
    /// a group came from.
    pub identity: Option<Hdf5ResultIdentity>,
    pub operating_point: Option<Hdf5WaveformSection>,
    pub transient: Option<Hdf5WaveformSection>,
    pub dc_sweep: Option<Hdf5WaveformSection>,
    pub noise: Option<Hdf5WaveformSection>,
    pub ac: Option<Hdf5AcSection>,
    pub distortion: Option<Hdf5DistortionSection>,
    pub fft: Option<Hdf5FftSection>,
    pub measurements: Vec<Hdf5Measurement>,
}

impl Hdf5SimulationData {
    pub fn new() -> Self {
        Self::default()
    }

    fn validate(&self) -> Result<()> {
        if let Some(operating_point) = &self.operating_point {
            operating_point.validate("operating_point")?;
        }
        if let Some(transient) = &self.transient {
            transient.validate("transient")?;
        }
        if let Some(dc_sweep) = &self.dc_sweep {
            dc_sweep.validate("dc_sweep")?;
        }
        if let Some(noise) = &self.noise {
            noise.validate("noise")?;
        }
        if let Some(ac) = &self.ac {
            ac.validate()?;
        }
        if let Some(distortion) = &self.distortion {
            distortion.validate()?;
        }
        if let Some(fft) = &self.fft {
            fft.validate()?;
        }
        Ok(())
    }
}

pub fn write_hdf5(path: &Path, data: &Hdf5SimulationData) -> Result<()> {
    let builder = build_hdf5(data)?;
    write_hdf5_builder(path, builder)
}

/// Serialize an HDF5 document into an already prepared artifact. Callers that
/// publish a logical multi-file result use this to finish every sibling before
/// committing any destination.
pub(crate) fn write_hdf5_to_writer(
    writer: &mut dyn std::io::Write,
    data: &Hdf5SimulationData,
) -> Result<()> {
    let bytes = build_hdf5(data)?.finish()?;
    writer.write_all(&bytes).map_err(Hdf5Error::ArtifactWrite)
}

fn build_hdf5(data: &Hdf5SimulationData) -> Result<FileBuilder> {
    data.validate()?;

    let mut builder = FileBuilder::new();
    builder.set_attr(
        "schema_version",
        AttrValue::String(SCHEMA_VERSION.to_string()),
    );
    builder.set_attr("simulator", AttrValue::String("RSpice".to_string()));
    builder.set_attr("title", AttrValue::String(data.title.clone()));
    // The identity travels on the root as well as in the group name, so a
    // reader that walks groups by their `section_type` still learns which
    // analysis instance and coordinate produced them.
    if let Some(identity) = &data.identity {
        builder.set_attr(
            "analysis_id",
            AttrValue::String(identity.analysis_id.clone()),
        );
        for (name, value) in [
            ("coordinate_id", identity.coordinate_id.as_ref()),
            ("coordinate_tag", identity.coordinate_tag.as_ref()),
            (
                "coordinate_assignment",
                identity.coordinate_assignment.as_ref(),
            ),
            (
                "topology_fingerprint",
                identity.topology_fingerprint.as_ref(),
            ),
        ] {
            if let Some(value) = value {
                builder.set_attr(name, AttrValue::String(value.clone()));
            }
        }
    }
    // One document carries one analysis, so the identity names its single
    // section group. A converted document with no identity keeps the family
    // name it was decoded under.
    let section_name = |family: &'static str| {
        data.identity.as_ref().map_or_else(
            || family.to_string(),
            |identity| identity.analysis_id.clone(),
        )
    };

    if let Some(operating_point) = &data.operating_point {
        add_waveform_section(
            &mut builder,
            &section_name("operating_point"),
            "operating_point",
            operating_point,
        )?;
    }
    if let Some(transient) = &data.transient {
        add_waveform_section(
            &mut builder,
            &section_name("transient"),
            "transient",
            transient,
        )?;
    }
    if let Some(dc_sweep) = &data.dc_sweep {
        add_waveform_section(
            &mut builder,
            &section_name("dc_sweep"),
            "dc_sweep",
            dc_sweep,
        )?;
    }
    if let Some(noise) = &data.noise {
        add_waveform_section(&mut builder, &section_name("noise"), "noise", noise)?;
    }
    if let Some(ac) = &data.ac {
        add_ac_section(&mut builder, &section_name("ac"), ac)?;
    }
    if let Some(distortion) = &data.distortion {
        add_distortion_section(&mut builder, &section_name("distortion"), distortion)?;
    }
    if let Some(fft) = &data.fft {
        add_fft_section(&mut builder, &section_name("fft"), fft)?;
    }
    if !data.measurements.is_empty() {
        add_measurements(&mut builder, &data.measurements)?;
    }

    Ok(builder)
}

fn write_hdf5_builder(path: &Path, builder: FileBuilder) -> Result<()> {
    publish::artifact(path, |file| {
        let bytes = builder.finish()?;
        file.write_all(&bytes)?;
        Ok(())
    })
    .map_err(|error| match error {
        AtomicArtifactError::Prepare(error) => Hdf5Error::ArtifactPreparation(error),
        AtomicArtifactError::Write(Hdf5StagingError::Backend(error)) => Hdf5Error::Backend(error),
        AtomicArtifactError::Write(Hdf5StagingError::Io(error)) => Hdf5Error::ArtifactWrite(error),
        AtomicArtifactError::Flush { source, .. } => Hdf5Error::ArtifactFlush(source),
        AtomicArtifactError::Commit { source, .. } => Hdf5Error::ArtifactCommit(source),
    })
}

pub fn read_hdf5(path: &Path) -> Result<Hdf5SimulationData> {
    let file = Hdf5File::open(path)?;
    let root = file.root();
    let root_attrs = root.attrs()?;
    let schema_version = read_required_string_attr(&root_attrs, "schema_version")?;
    if schema_version != SCHEMA_VERSION {
        return Err(Hdf5Error::InvalidSchema(format!(
            "unsupported HDF5 schema version '{schema_version}', expected '{SCHEMA_VERSION}'"
        )));
    }

    let title = read_string_attr(&root_attrs, "title")?.unwrap_or_default();
    let identity = read_string_attr(&root_attrs, "analysis_id")?.map(|analysis_id| {
        Ok::<_, Hdf5Error>(Hdf5ResultIdentity {
            analysis_id,
            coordinate_id: read_string_attr(&root_attrs, "coordinate_id")?,
            coordinate_tag: read_string_attr(&root_attrs, "coordinate_tag")?,
            coordinate_assignment: read_string_attr(&root_attrs, "coordinate_assignment")?,
            topology_fingerprint: read_string_attr(&root_attrs, "topology_fingerprint")?,
        })
    });
    let identity = identity.transpose()?;
    let root_groups = root.groups()?;

    // A section group is named by the analysis instance that produced it, so
    // the family it belongs to is read from its own `section_type` attribute
    // rather than guessed from the group name.
    let mut data = Hdf5SimulationData {
        title,
        identity,
        ..Hdf5SimulationData::default()
    };
    for group_name in &root_groups {
        if group_name == "measurements" {
            data.measurements = read_measurements(&file)?;
            continue;
        }
        let family = read_required_string_attr(&file.group(group_name)?.attrs()?, "section_type")
            .map_err(|_| {
            Hdf5Error::InvalidSchema(format!(
                "group '{group_name}' declares no section_type, so its result family is unknown"
            ))
        })?;
        match family.as_str() {
            "operating_point" => {
                data.operating_point = Some(read_waveform_section(&file, group_name)?);
            }
            "transient" => data.transient = Some(read_waveform_section(&file, group_name)?),
            "dc_sweep" => data.dc_sweep = Some(read_waveform_section(&file, group_name)?),
            "noise" => data.noise = Some(read_waveform_section(&file, group_name)?),
            "ac" => data.ac = Some(read_ac_section(&file, group_name)?),
            "distortion" => data.distortion = Some(read_distortion_section(&file, group_name)?),
            "fft" => data.fft = Some(read_fft_section(&file, group_name)?),
            other => {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "group '{group_name}' declares unknown section_type '{other}'"
                )));
            }
        }
    }

    Ok(data)
}

fn add_waveform_section(
    builder: &mut FileBuilder,
    name: &str,
    family: &str,
    section: &Hdf5WaveformSection,
) -> Result<()> {
    let mut group = builder.create_group(name);
    group.set_attr("section_type", AttrValue::String(family.to_string()));
    group.set_attr(
        "independent_name",
        AttrValue::String(section.independent_name.clone()),
    );
    group.set_attr("signal_count", AttrValue::I64(section.signals.len() as i64));
    group
        .create_dataset("independent")
        .with_f64_data(&section.independent_values);

    for (index, signal) in section.signals.iter().enumerate() {
        let dataset_name = format!("signal_{index:04}");
        group.set_attr(
            &format!("{dataset_name}_name"),
            AttrValue::String(signal.name.clone()),
        );
        group.set_attr(
            &format!("{dataset_name}_type"),
            AttrValue::String(signal.var_type.clone()),
        );
        group
            .create_dataset(&dataset_name)
            .with_f64_data(&signal.values);
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_waveform_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5WaveformSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;

    let independent_name = read_required_string_attr(&attrs, "independent_name")?;
    let signal_count = read_required_i64_attr(&attrs, "signal_count")? as usize;
    let independent_values = group.dataset("independent")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let dataset_name = format!("signal_{index:04}");
        let signal_name = read_required_string_attr(&attrs, &format!("{dataset_name}_name"))?;
        let signal_type = read_string_attr(&attrs, &format!("{dataset_name}_type"))?
            .unwrap_or_else(|| "value".to_string());
        let values = group.dataset(&dataset_name)?.read_f64()?;
        signals.push(Hdf5Signal::new_typed(signal_name, signal_type, values));
    }

    let section = Hdf5WaveformSection {
        independent_name,
        independent_values,
        signals,
    };
    section.validate(group_name)?;
    Ok(section)
}

fn add_ac_section(builder: &mut FileBuilder, name: &str, section: &Hdf5AcSection) -> Result<()> {
    let mut group = builder.create_group(name);
    group.set_attr("section_type", AttrValue::String("ac".to_string()));
    group.set_attr("signal_count", AttrValue::I64(section.signals.len() as i64));
    group
        .create_dataset("frequency")
        .with_f64_data(&section.frequency);

    for (index, signal) in section.signals.iter().enumerate() {
        let prefix = format!("signal_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            AttrValue::String(signal.name.clone()),
        );
        group
            .create_dataset(&format!("{prefix}_real"))
            .with_f64_data(&signal.real);
        group
            .create_dataset(&format!("{prefix}_imag"))
            .with_f64_data(&signal.imag);
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_ac_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5AcSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let signal_count = read_required_i64_attr(&attrs, "signal_count")? as usize;
    let frequency = group.dataset("frequency")?.read_f64()?;

    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        let name = read_required_string_attr(&attrs, &format!("{prefix}_name"))?;
        let real = group.dataset(&format!("{prefix}_real"))?.read_f64()?;
        let imag = group.dataset(&format!("{prefix}_imag"))?.read_f64()?;
        signals.push(Hdf5ComplexSignal::new(name, real, imag));
    }

    let section = Hdf5AcSection { frequency, signals };
    section.validate()?;
    Ok(section)
}

fn add_distortion_section(
    builder: &mut FileBuilder,
    name: &str,
    section: &Hdf5DistortionSection,
) -> Result<()> {
    let mut group = builder.create_group(name);
    group.set_attr("section_type", AttrValue::String("distortion".to_string()));
    group.set_attr("mode", AttrValue::String(section.mode.clone()));
    group.set_attr(
        "phasor_convention",
        AttrValue::String(section.phasor_convention.clone()),
    );
    group.set_attr(
        "ratio_normalization",
        AttrValue::String(section.ratio_normalization.clone()),
    );
    if let Some(ratio) = section.f2_over_f1 {
        group.set_attr("f2_over_f1", AttrValue::F64(ratio));
    }
    group.set_attr("series_count", AttrValue::I64(section.series.len() as i64));
    group
        .create_dataset("f1_frequency")
        .with_f64_data(&section.f1_frequency);

    for (series_index, series) in section.series.iter().enumerate() {
        let series_prefix = format!("series_{series_index:04}");
        group.set_attr(
            &format!("{series_prefix}_label"),
            AttrValue::String(series.label.clone()),
        );
        group.set_attr(
            &format!("{series_prefix}_is_product"),
            AttrValue::I64(i64::from(series.is_product)),
        );
        group.set_attr(
            &format!("{series_prefix}_signal_count"),
            AttrValue::I64(series.signals.len() as i64),
        );
        group
            .create_dataset(&format!("{series_prefix}_frequency"))
            .with_f64_data(&series.physical_frequency);

        for (signal_index, signal) in series.signals.iter().enumerate() {
            let signal_prefix = format!("{series_prefix}_signal_{signal_index:04}");
            group.set_attr(
                &format!("{signal_prefix}_name"),
                AttrValue::String(signal.name.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_type"),
                AttrValue::String(signal.var_type.clone()),
            );
            group.set_attr(
                &format!("{signal_prefix}_has_ratio"),
                AttrValue::I64(i64::from(signal.magnitude_ratio_to_f1.is_some())),
            );
            group
                .create_dataset(&format!("{signal_prefix}_real"))
                .with_f64_data(&signal.real);
            group
                .create_dataset(&format!("{signal_prefix}_imag"))
                .with_f64_data(&signal.imag);
            group
                .create_dataset(&format!("{signal_prefix}_magnitude"))
                .with_f64_data(&signal.magnitude);
            group
                .create_dataset(&format!("{signal_prefix}_phase_degrees"))
                .with_f64_data(&signal.phase_degrees);
            if let Some(ratio) = &signal.magnitude_ratio_to_f1 {
                group
                    .create_dataset(&format!("{signal_prefix}_magnitude_ratio_to_f1"))
                    .with_f64_data(ratio);
            }
        }
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_distortion_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5DistortionSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let mode = read_required_string_attr(&attrs, "mode")?;
    let f2_over_f1 = read_f64_attr(&attrs, "f2_over_f1")?;
    let phasor_convention = read_required_string_attr(&attrs, "phasor_convention")?;
    let ratio_normalization = read_required_string_attr(&attrs, "ratio_normalization")?;
    let series_count = non_negative_count(
        read_required_i64_attr(&attrs, "series_count")?,
        "distortion series_count",
    )?;
    let f1_frequency = group.dataset("f1_frequency")?.read_f64()?;
    let mut series = Vec::with_capacity(series_count);

    for series_index in 0..series_count {
        let series_prefix = format!("series_{series_index:04}");
        let label = read_required_string_attr(&attrs, &format!("{series_prefix}_label"))?;
        let is_product = read_binary_flag(&attrs, &format!("{series_prefix}_is_product"))?;
        let signal_count = non_negative_count(
            read_required_i64_attr(&attrs, &format!("{series_prefix}_signal_count"))?,
            &format!("{series_prefix}_signal_count"),
        )?;
        let physical_frequency = group
            .dataset(&format!("{series_prefix}_frequency"))?
            .read_f64()?;
        let mut signals = Vec::with_capacity(signal_count);

        for signal_index in 0..signal_count {
            let signal_prefix = format!("{series_prefix}_signal_{signal_index:04}");
            let name = read_required_string_attr(&attrs, &format!("{signal_prefix}_name"))?;
            let var_type = read_required_string_attr(&attrs, &format!("{signal_prefix}_type"))?;
            let has_ratio = read_binary_flag(&attrs, &format!("{signal_prefix}_has_ratio"))?;
            let real = group
                .dataset(&format!("{signal_prefix}_real"))?
                .read_f64()?;
            let imag = group
                .dataset(&format!("{signal_prefix}_imag"))?
                .read_f64()?;
            let magnitude = group
                .dataset(&format!("{signal_prefix}_magnitude"))?
                .read_f64()?;
            let phase_degrees = group
                .dataset(&format!("{signal_prefix}_phase_degrees"))?
                .read_f64()?;
            let magnitude_ratio_to_f1 = if has_ratio {
                Some(
                    group
                        .dataset(&format!("{signal_prefix}_magnitude_ratio_to_f1"))?
                        .read_f64()?,
                )
            } else {
                None
            };
            signals.push(Hdf5DistortionSignal {
                name,
                var_type,
                real,
                imag,
                magnitude,
                phase_degrees,
                magnitude_ratio_to_f1,
            });
        }

        series.push(Hdf5DistortionSeries {
            label,
            is_product,
            physical_frequency,
            signals,
        });
    }

    let section = Hdf5DistortionSection {
        mode,
        f2_over_f1,
        phasor_convention,
        ratio_normalization,
        f1_frequency,
        series,
    };
    section.validate()?;
    Ok(section)
}

fn checked_i64(value: usize, name: &str) -> Result<i64> {
    i64::try_from(value).map_err(|_| {
        Hdf5Error::InvalidSchema(format!("{name} exceeds the HDF5 signed-integer range"))
    })
}

fn add_fft_section(builder: &mut FileBuilder, name: &str, section: &Hdf5FftSection) -> Result<()> {
    let mut group = builder.create_group(name);
    group.set_attr("section_type", AttrValue::String("fft".to_string()));
    group.set_attr(
        "schema_version",
        AttrValue::String(FFT_SECTION_SCHEMA_VERSION.to_string()),
    );
    group.set_attr(
        "parent_analysis_id",
        AttrValue::String(section.parent_analysis_id.clone()),
    );
    group.set_attr(
        "has_coordinate",
        AttrValue::I64(i64::from(section.coordinate.is_some())),
    );
    if let Some(coordinate) = &section.coordinate {
        group.set_attr(
            "coordinate_id",
            AttrValue::String(coordinate.coordinate_id.clone()),
        );
        group.set_attr(
            "coordinate_ordinal",
            AttrValue::I64(checked_i64(coordinate.ordinal, "FFT coordinate ordinal")?),
        );
        group.set_attr("coordinate_tag", AttrValue::String(coordinate.tag.clone()));
        group.set_attr(
            "coordinate_assignment",
            AttrValue::String(coordinate.assignment.clone()),
        );
    }
    group.set_attr(
        "result_count",
        AttrValue::I64(checked_i64(section.results.len(), "FFT result count")?),
    );

    for (index, result) in section.results.iter().enumerate() {
        let prefix = format!("result_{index:04}");
        for (suffix, value) in [
            ("analysis_id", result.analysis_id.as_str()),
            ("source_kind", result.source_kind.as_str()),
            ("source_text", result.source_text.as_str()),
            ("authored_output", result.authored_output.as_str()),
            ("output_name", result.output_name.as_str()),
            ("physical_type", result.physical_type.as_str()),
            ("format", result.format.as_str()),
            ("mode", result.mode.as_str()),
            ("window", result.window.as_str()),
            ("window_name", result.window_name.as_str()),
        ] {
            group.set_attr(
                &format!("{prefix}_{suffix}"),
                AttrValue::String(value.to_string()),
            );
        }
        group.set_attr(
            &format!("{prefix}_has_value_unit"),
            AttrValue::I64(i64::from(result.value_unit.is_some())),
        );
        if let Some(unit) = &result.value_unit {
            group.set_attr(
                &format!("{prefix}_value_unit"),
                AttrValue::String(unit.clone()),
            );
        }
        for (suffix, value) in [
            ("start_time_s", result.start_time_s),
            ("stop_time_s", result.stop_time_s),
            ("sample_interval_s", result.sample_interval_s),
            ("alpha", result.alpha),
            ("coherent_gain", result.coherent_gain),
            ("frequency_resolution_hz", result.frequency_resolution_hz),
        ] {
            group.set_attr(&format!("{prefix}_{suffix}"), AttrValue::F64(value));
        }
        for (suffix, value) in [
            ("ordinal", result.ordinal),
            ("point_count", result.point_count),
            ("fundamental_bin", result.fundamental_bin),
            ("minimum_metric_bin", result.minimum_metric_bin),
            ("maximum_metric_bin", result.maximum_metric_bin),
            ("sfdr_search_minimum_bin", result.sfdr_search_minimum_bin),
        ] {
            group.set_attr(
                &format!("{prefix}_{suffix}"),
                AttrValue::I64(checked_i64(value, &format!("{prefix}_{suffix}"))?),
            );
        }
        group.set_attr(
            &format!("{prefix}_accurate_sampling"),
            AttrValue::I64(i64::from(result.accurate_sampling)),
        );
        group.set_attr(
            &format!("{prefix}_has_metrics"),
            AttrValue::I64(i64::from(result.metrics.is_some())),
        );
        let bin_indices = result
            .bin_indices
            .iter()
            .map(|value| {
                i64::try_from(*value).map_err(|_| {
                    Hdf5Error::InvalidSchema(format!(
                        "{prefix} FFT bin index exceeds the HDF5 signed-integer range"
                    ))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        group
            .create_dataset(&format!("{prefix}_bin_index"))
            .with_i64_data(&bin_indices);
        for (suffix, values) in [
            ("frequency_hz", &result.frequency_hz),
            ("real", &result.real),
            ("imaginary", &result.imaginary),
            ("magnitude", &result.magnitude),
            ("phase_degrees", &result.phase_degrees),
        ] {
            group
                .create_dataset(&format!("{prefix}_{suffix}"))
                .with_f64_data(values);
        }

        if let Some(metrics) = &result.metrics {
            for (suffix, value) in [
                ("fundamental_magnitude", metrics.fundamental_magnitude),
                ("thd_ratio", metrics.thd_ratio),
                ("thd_db", metrics.thd_db),
                ("sndr_db", metrics.sndr_db),
                ("enob_bits", metrics.enob_bits),
                ("snr_db", metrics.snr_db),
                ("sfdr_db", metrics.sfdr_db),
            ] {
                group.set_attr(&format!("{prefix}_metrics_{suffix}"), AttrValue::F64(value));
            }
            group.set_attr(
                &format!("{prefix}_metrics_has_spur"),
                AttrValue::I64(i64::from(metrics.sfdr_spur_bin.is_some())),
            );
            if let (Some(bin), Some(frequency)) =
                (metrics.sfdr_spur_bin, metrics.sfdr_spur_frequency_hz)
            {
                group.set_attr(
                    &format!("{prefix}_metrics_sfdr_spur_bin"),
                    AttrValue::I64(checked_i64(bin, "FFT SFDR spur bin")?),
                );
                group.set_attr(
                    &format!("{prefix}_metrics_sfdr_spur_frequency_hz"),
                    AttrValue::F64(frequency),
                );
            }
            let ranks = metrics
                .largest_harmonics
                .iter()
                .map(|harmonic| checked_i64(harmonic.rank, "FFT harmonic rank"))
                .collect::<Result<Vec<_>>>()?;
            let bins = metrics
                .largest_harmonics
                .iter()
                .map(|harmonic| checked_i64(harmonic.bin, "FFT harmonic bin"))
                .collect::<Result<Vec<_>>>()?;
            group
                .create_dataset(&format!("{prefix}_metrics_harmonic_rank"))
                .with_i64_data(&ranks);
            group
                .create_dataset(&format!("{prefix}_metrics_harmonic_bin"))
                .with_i64_data(&bins);
            for (suffix, values) in [
                (
                    "frequency_hz",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.frequency_hz)
                        .collect::<Vec<_>>(),
                ),
                (
                    "magnitude",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.magnitude)
                        .collect::<Vec<_>>(),
                ),
                (
                    "magnitude_db",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.magnitude_db)
                        .collect::<Vec<_>>(),
                ),
                (
                    "phase_degrees",
                    metrics
                        .largest_harmonics
                        .iter()
                        .map(|harmonic| harmonic.phase_degrees)
                        .collect::<Vec<_>>(),
                ),
            ] {
                group
                    .create_dataset(&format!("{prefix}_metrics_harmonic_{suffix}"))
                    .with_f64_data(&values);
            }
        }
    }

    builder.add_group(group.finish());
    Ok(())
}

fn read_fft_section(file: &Hdf5File, group_name: &str) -> Result<Hdf5FftSection> {
    let group = file.group(group_name)?;
    let attrs = group.attrs()?;
    let schema_version = read_required_string_attr(&attrs, "schema_version")?;
    if schema_version != FFT_SECTION_SCHEMA_VERSION {
        return Err(Hdf5Error::InvalidSchema(format!(
            "unsupported FFT HDF5 schema version '{schema_version}', expected '{FFT_SECTION_SCHEMA_VERSION}'"
        )));
    }
    let parent_analysis_id = read_required_string_attr(&attrs, "parent_analysis_id")?;
    let coordinate = if read_binary_flag(&attrs, "has_coordinate")? {
        Some(Hdf5FftCoordinate {
            coordinate_id: read_required_string_attr(&attrs, "coordinate_id")?,
            ordinal: non_negative_count(
                read_required_i64_attr(&attrs, "coordinate_ordinal")?,
                "FFT coordinate ordinal",
            )?,
            tag: read_required_string_attr(&attrs, "coordinate_tag")?,
            assignment: read_required_string_attr(&attrs, "coordinate_assignment")?,
        })
    } else {
        None
    };
    let result_count = non_negative_count(
        read_required_i64_attr(&attrs, "result_count")?,
        "FFT result_count",
    )?;
    let mut results = Vec::with_capacity(result_count);

    for index in 0..result_count {
        let prefix = format!("result_{index:04}");
        let metrics = if read_binary_flag(&attrs, &format!("{prefix}_has_metrics"))? {
            let has_spur = read_binary_flag(&attrs, &format!("{prefix}_metrics_has_spur"))?;
            let ranks = group
                .dataset(&format!("{prefix}_metrics_harmonic_rank"))?
                .read_i64()?;
            let bins = group
                .dataset(&format!("{prefix}_metrics_harmonic_bin"))?
                .read_i64()?;
            let frequencies = group
                .dataset(&format!("{prefix}_metrics_harmonic_frequency_hz"))?
                .read_f64()?;
            let magnitudes = group
                .dataset(&format!("{prefix}_metrics_harmonic_magnitude"))?
                .read_f64()?;
            let magnitudes_db = group
                .dataset(&format!("{prefix}_metrics_harmonic_magnitude_db"))?
                .read_f64()?;
            let phases = group
                .dataset(&format!("{prefix}_metrics_harmonic_phase_degrees"))?
                .read_f64()?;
            let harmonic_count = ranks.len();
            if [
                bins.len(),
                frequencies.len(),
                magnitudes.len(),
                magnitudes_db.len(),
                phases.len(),
            ]
            .iter()
            .any(|count| *count != harmonic_count)
            {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "{prefix} FFT harmonic datasets have inconsistent lengths"
                )));
            }
            let mut largest_harmonics = Vec::with_capacity(harmonic_count);
            for harmonic_index in 0..harmonic_count {
                largest_harmonics.push(Hdf5FftHarmonic {
                    rank: non_negative_count(
                        ranks[harmonic_index],
                        &format!("{prefix} FFT harmonic rank"),
                    )?,
                    bin: non_negative_count(
                        bins[harmonic_index],
                        &format!("{prefix} FFT harmonic bin"),
                    )?,
                    frequency_hz: frequencies[harmonic_index],
                    magnitude: magnitudes[harmonic_index],
                    magnitude_db: magnitudes_db[harmonic_index],
                    phase_degrees: phases[harmonic_index],
                });
            }
            Some(Hdf5FftMetrics {
                fundamental_magnitude: read_required_f64_attr(
                    &attrs,
                    &format!("{prefix}_metrics_fundamental_magnitude"),
                )?,
                thd_ratio: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_thd_ratio"))?,
                thd_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_thd_db"))?,
                sndr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_sndr_db"))?,
                enob_bits: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_enob_bits"))?,
                snr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_snr_db"))?,
                sfdr_db: read_required_f64_attr(&attrs, &format!("{prefix}_metrics_sfdr_db"))?,
                sfdr_spur_bin: has_spur
                    .then(|| {
                        non_negative_count(
                            read_required_i64_attr(
                                &attrs,
                                &format!("{prefix}_metrics_sfdr_spur_bin"),
                            )?,
                            "FFT SFDR spur bin",
                        )
                    })
                    .transpose()?,
                sfdr_spur_frequency_hz: has_spur
                    .then(|| {
                        read_required_f64_attr(
                            &attrs,
                            &format!("{prefix}_metrics_sfdr_spur_frequency_hz"),
                        )
                    })
                    .transpose()?,
                largest_harmonics,
            })
        } else {
            None
        };
        let has_value_unit = read_binary_flag(&attrs, &format!("{prefix}_has_value_unit"))?;
        results.push(Hdf5FftResult {
            analysis_id: read_required_string_attr(&attrs, &format!("{prefix}_analysis_id"))?,
            ordinal: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_ordinal"))?,
                "FFT ordinal",
            )?,
            source_kind: read_required_string_attr(&attrs, &format!("{prefix}_source_kind"))?,
            source_text: read_required_string_attr(&attrs, &format!("{prefix}_source_text"))?,
            authored_output: read_required_string_attr(
                &attrs,
                &format!("{prefix}_authored_output"),
            )?,
            output_name: read_required_string_attr(&attrs, &format!("{prefix}_output_name"))?,
            physical_type: read_required_string_attr(&attrs, &format!("{prefix}_physical_type"))?,
            value_unit: has_value_unit
                .then(|| read_required_string_attr(&attrs, &format!("{prefix}_value_unit")))
                .transpose()?,
            start_time_s: read_required_f64_attr(&attrs, &format!("{prefix}_start_time_s"))?,
            stop_time_s: read_required_f64_attr(&attrs, &format!("{prefix}_stop_time_s"))?,
            sample_interval_s: read_required_f64_attr(
                &attrs,
                &format!("{prefix}_sample_interval_s"),
            )?,
            point_count: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_point_count"))?,
                "FFT point_count",
            )?,
            accurate_sampling: read_binary_flag(&attrs, &format!("{prefix}_accurate_sampling"))?,
            format: read_required_string_attr(&attrs, &format!("{prefix}_format"))?,
            mode: read_required_string_attr(&attrs, &format!("{prefix}_mode"))?,
            window: read_required_string_attr(&attrs, &format!("{prefix}_window"))?,
            window_name: read_required_string_attr(&attrs, &format!("{prefix}_window_name"))?,
            alpha: read_required_f64_attr(&attrs, &format!("{prefix}_alpha"))?,
            coherent_gain: read_required_f64_attr(&attrs, &format!("{prefix}_coherent_gain"))?,
            frequency_resolution_hz: read_required_f64_attr(
                &attrs,
                &format!("{prefix}_frequency_resolution_hz"),
            )?,
            fundamental_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_fundamental_bin"))?,
                "FFT fundamental_bin",
            )?,
            minimum_metric_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_minimum_metric_bin"))?,
                "FFT minimum_metric_bin",
            )?,
            maximum_metric_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_maximum_metric_bin"))?,
                "FFT maximum_metric_bin",
            )?,
            sfdr_search_minimum_bin: non_negative_count(
                read_required_i64_attr(&attrs, &format!("{prefix}_sfdr_search_minimum_bin"))?,
                "FFT sfdr_search_minimum_bin",
            )?,
            bin_indices: group
                .dataset(&format!("{prefix}_bin_index"))?
                .read_i64()?
                .into_iter()
                .map(|value| {
                    u64::try_from(value).map_err(|_| {
                        Hdf5Error::InvalidSchema(format!(
                            "{prefix} FFT bin index must be non-negative, got {value}"
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?,
            frequency_hz: group
                .dataset(&format!("{prefix}_frequency_hz"))?
                .read_f64()?,
            real: group.dataset(&format!("{prefix}_real"))?.read_f64()?,
            imaginary: group.dataset(&format!("{prefix}_imaginary"))?.read_f64()?,
            magnitude: group.dataset(&format!("{prefix}_magnitude"))?.read_f64()?,
            phase_degrees: group
                .dataset(&format!("{prefix}_phase_degrees"))?
                .read_f64()?,
            metrics,
        });
    }

    let section = Hdf5FftSection {
        parent_analysis_id,
        coordinate,
        results,
    };
    section.validate()?;
    Ok(section)
}

fn add_measurements(builder: &mut FileBuilder, measurements: &[Hdf5Measurement]) -> Result<()> {
    let mut group = builder.create_group("measurements");
    group.set_attr(
        "measurement_count",
        AttrValue::I64(measurements.len() as i64),
    );
    for (index, measurement) in measurements.iter().enumerate() {
        let prefix = format!("measurement_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            AttrValue::String(measurement.name.clone()),
        );
        group.set_attr(
            &format!("{prefix}_value"),
            AttrValue::F64(measurement.value),
        );
    }
    builder.add_group(group.finish());
    Ok(())
}

fn read_measurements(file: &Hdf5File) -> Result<Vec<Hdf5Measurement>> {
    let group = file.group("measurements")?;
    let attrs = group.attrs()?;
    let measurement_count = read_required_i64_attr(&attrs, "measurement_count")? as usize;

    let mut measurements = Vec::with_capacity(measurement_count);
    for index in 0..measurement_count {
        let prefix = format!("measurement_{index:04}");
        let name = read_required_string_attr(&attrs, &format!("{prefix}_name"))?;
        let value = read_required_f64_attr(&attrs, &format!("{prefix}_value"))?;
        measurements.push(Hdf5Measurement::new(name, value));
    }
    Ok(measurements)
}

fn read_string_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<Option<String>> {
    match attrs.get(name) {
        None => Ok(None),
        Some(AttrValue::String(value)) => Ok(Some(value.clone())),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected string, found {other:?}"
        ))),
    }
}

fn read_required_string_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<String> {
    read_string_attr(attrs, name)?.ok_or_else(|| {
        Hdf5Error::InvalidSchema(format!("missing required string attribute '{name}'"))
    })
}

fn read_required_i64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<i64> {
    match attrs.get(name) {
        Some(AttrValue::I64(value)) => Ok(*value),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected i64, found {other:?}"
        ))),
        None => Err(Hdf5Error::InvalidSchema(format!(
            "missing required integer attribute '{name}'"
        ))),
    }
}

fn read_f64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<Option<f64>> {
    match attrs.get(name) {
        None => Ok(None),
        Some(AttrValue::F64(value)) => Ok(Some(*value)),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected f64, found {other:?}"
        ))),
    }
}

fn non_negative_count(value: i64, name: &str) -> Result<usize> {
    usize::try_from(value)
        .map_err(|_| Hdf5Error::InvalidSchema(format!("{name} must be non-negative, got {value}")))
}

fn read_binary_flag(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<bool> {
    match read_required_i64_attr(attrs, name)? {
        0 => Ok(false),
        1 => Ok(true),
        value => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' must be 0 or 1, got {value}"
        ))),
    }
}

fn read_required_f64_attr(attrs: &HashMap<String, AttrValue>, name: &str) -> Result<f64> {
    match attrs.get(name) {
        Some(AttrValue::F64(value)) => Ok(*value),
        Some(other) => Err(Hdf5Error::InvalidSchema(format!(
            "attribute '{name}' expected f64, found {other:?}"
        ))),
        None => Err(Hdf5Error::InvalidSchema(format!(
            "missing required float attribute '{name}'"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(std::path::PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-hdf5-atomic-{}-{id}-{tag}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create unique HDF5 test directory");
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn assert_only_destination_remains(directory: &Path, destination: &Path, exists: bool) {
        let entries: Vec<std::path::PathBuf> = std::fs::read_dir(directory)
            .expect("read HDF5 test directory")
            .map(|entry| entry.expect("read HDF5 directory entry").path())
            .collect();
        if exists {
            assert_eq!(entries, vec![destination.to_path_buf()]);
        } else {
            assert!(
                entries.is_empty(),
                "unexpected staged artifacts: {entries:?}"
            );
        }
    }

    #[test]
    fn backend_serialization_failure_preserves_old_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("backend-failure");
            let destination = directory.0.join("result.h5");
            if preexisting {
                std::fs::write(&destination, b"old complete HDF5 artifact")
                    .expect("seed existing HDF5 destination");
            }

            let mut incomplete_builder = FileBuilder::new();
            incomplete_builder.create_dataset("missing_data");
            let error = write_hdf5_builder(&destination, incomplete_builder)
                .expect_err("incomplete dataset must fail serialization");
            assert!(matches!(error, Hdf5Error::Backend(_)));

            if preexisting {
                assert_eq!(
                    std::fs::read(&destination).expect("read preserved HDF5 destination"),
                    b"old complete HDF5 artifact"
                );
            } else {
                assert!(!destination.exists());
            }
            assert_only_destination_remains(&directory.0, &destination, preexisting);
        }
    }

    #[test]
    fn successful_hdf5_write_atomically_replaces_existing_bytes() {
        let directory = TestDirectory::new("success");
        let destination = directory.0.join("result.h5");
        std::fs::write(&destination, b"old complete HDF5 artifact")
            .expect("seed existing HDF5 destination");

        let mut data = Hdf5SimulationData::new();
        data.title = "atomic result".to_string();
        let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
        transient.add_signal("V(out)", vec![0.0, 2.0]);
        data.transient = Some(transient);
        write_hdf5(&destination, &data).expect("write complete HDF5 destination");

        assert_eq!(
            read_hdf5(&destination).expect("read committed HDF5 destination"),
            data
        );
        assert_only_destination_remains(&directory.0, &destination, true);
    }

    fn fft_result(
        ordinal: usize,
        point_count: usize,
        physical_type: &str,
        value_unit: Option<&str>,
        with_metrics: bool,
    ) -> Hdf5FftResult {
        let bin_count = point_count / 2 + 1;
        let frequency_hz = (0..bin_count).map(|bin| bin as f64).collect::<Vec<_>>();
        let real = (0..bin_count)
            .map(|bin| if bin == 1 { 1.0 } else { 0.0 })
            .collect::<Vec<_>>();
        let imaginary = vec![0.0; bin_count];
        let magnitude = real.clone();
        let phase_degrees = vec![0.0; bin_count];
        let fundamental_bin = 1;
        let maximum_metric_bin = bin_count - 1;
        let sfdr_search_minimum_bin = fundamental_bin;
        let expected_metrics = fft_metric_expectations(
            &magnitude,
            fundamental_bin,
            maximum_metric_bin,
            sfdr_search_minimum_bin,
        )
        .expect("valid FFT metric fixture");
        let sfdr_spur_frequency_hz = expected_metrics.sfdr_spur_bin.map(|bin| frequency_hz[bin]);
        let largest_harmonics = expected_metrics
            .ranked_bins
            .iter()
            .copied()
            .enumerate()
            .map(|(index, bin)| Hdf5FftHarmonic {
                rank: index + 1,
                bin,
                frequency_hz: frequency_hz[bin],
                magnitude: magnitude[bin],
                magnitude_db: 20.0 * magnitude[bin].max(FFT_DB_NOISE_FLOOR).log10(),
                phase_degrees: phase_degrees[bin],
            })
            .collect();
        Hdf5FftResult {
            analysis_id: format!("fft-{ordinal:03}"),
            ordinal,
            source_kind: if physical_type == "parameter" {
                "expression".to_string()
            } else {
                "probe".to_string()
            },
            source_text: "V(OUT)".to_string(),
            authored_output: if physical_type == "parameter" {
                "{V(OUT)}".to_string()
            } else {
                "V(OUT)".to_string()
            },
            output_name: "V(OUT)".to_string(),
            physical_type: physical_type.to_string(),
            value_unit: value_unit.map(str::to_string),
            start_time_s: 0.0,
            stop_time_s: 1.0,
            sample_interval_s: 1.0 / point_count as f64,
            point_count,
            accurate_sampling: true,
            format: if ordinal == 1 {
                "normalized".to_string()
            } else {
                "unnormalized".to_string()
            },
            mode: "hspice_compatible".to_string(),
            window: if ordinal == 1 {
                "hann".to_string()
            } else {
                "rectangular".to_string()
            },
            window_name: if ordinal == 1 {
                "HANN".to_string()
            } else {
                "RECT".to_string()
            },
            alpha: 3.0,
            coherent_gain: 1.0,
            frequency_resolution_hz: 1.0,
            fundamental_bin,
            minimum_metric_bin: 0,
            maximum_metric_bin,
            sfdr_search_minimum_bin,
            bin_indices: (0..u64::try_from(bin_count).expect("bounded bin count")).collect(),
            frequency_hz,
            real,
            imaginary,
            magnitude,
            phase_degrees,
            metrics: with_metrics.then_some(Hdf5FftMetrics {
                fundamental_magnitude: expected_metrics.fundamental_magnitude,
                thd_ratio: expected_metrics.thd_ratio,
                thd_db: expected_metrics.thd_db,
                sndr_db: expected_metrics.sndr_db,
                enob_bits: expected_metrics.enob_bits,
                snr_db: expected_metrics.snr_db,
                sfdr_db: expected_metrics.sfdr_db,
                sfdr_spur_bin: expected_metrics.sfdr_spur_bin,
                sfdr_spur_frequency_hz,
                largest_harmonics,
            }),
        }
    }

    #[test]
    fn typed_fft_section_round_trips_ragged_results_and_coordinate_metadata() {
        let directory = TestDirectory::new("fft-round-trip");
        let destination = directory.0.join("fft.h5");
        let mut data = Hdf5SimulationData::new();
        data.title = "typed FFT".to_string();
        data.fft = Some(Hdf5FftSection {
            parent_analysis_id: "tran-002".to_string(),
            coordinate: Some(Hdf5FftCoordinate {
                coordinate_id: "0123456789abcdef0123456789abcdef-001".to_string(),
                ordinal: 2,
                tag: "run-0123456789abcdef0123456789abcdef-001".to_string(),
                assignment: "PARAM gain = 2, TEMP = 75".to_string(),
            }),
            results: vec![
                fft_result(1, 8, "voltage", Some("1"), true),
                fft_result(2, 16, "parameter", None, false),
            ],
        });

        write_hdf5(&destination, &data).expect("write typed FFT HDF5 artifact");
        assert_eq!(
            read_hdf5(&destination).expect("read typed FFT HDF5 artifact"),
            data
        );
        assert_only_destination_remains(&directory.0, &destination, true);
    }

    #[test]
    fn malformed_fft_section_is_rejected_before_publication() {
        let directory = TestDirectory::new("fft-malformed");
        let destination = directory.0.join("fft.h5");
        let mut malformed = fft_result(1, 8, "voltage", Some("1"), true);
        malformed.analysis_id = "fft-002".to_string();
        let mut data = Hdf5SimulationData::new();
        data.fft = Some(Hdf5FftSection {
            parent_analysis_id: "tran-001".to_string(),
            coordinate: None,
            results: vec![malformed],
        });

        let error = write_hdf5(&destination, &data).expect_err("reject malformed FFT identity");
        assert!(matches!(error, Hdf5Error::InvalidSchema(_)));
        assert!(!destination.exists());
        assert_only_destination_remains(&directory.0, &destination, false);
    }

    #[test]
    fn fft_units_and_normalization_are_validated_against_transform_semantics() {
        assert!(fft_source_identity_is_valid("probe", "V(OUT)", "V(OUT)"));
        assert!(fft_source_identity_is_valid(
            "expression",
            "2*V(OUT)",
            "{2*V(OUT)}"
        ));
        assert!(!fft_source_identity_is_valid("probe", "V(OUT)", "V(IN)"));
        assert!(!fft_source_identity_is_valid(
            "expression",
            "2*V(OUT)",
            "2*V(OUT)"
        ));
        assert!(
            fft_result(1, 8, "voltage", Some("1"), true)
                .validate(1, "fft-001")
                .is_ok()
        );
        assert!(
            fft_result(1, 8, "current", Some("1"), true)
                .validate(1, "fft-001")
                .is_ok()
        );
        assert!(
            fft_result(2, 8, "voltage", Some("V"), false)
                .validate(2, "fft-002")
                .is_ok()
        );
        assert!(
            fft_result(2, 8, "current", Some("A"), false)
                .validate(2, "fft-002")
                .is_ok()
        );

        assert!(
            fft_result(1, 8, "voltage", Some("V"), true)
                .validate(1, "fft-001")
                .is_err()
        );
        assert!(
            fft_result(2, 8, "current", Some("1"), false)
                .validate(2, "fft-002")
                .is_err()
        );
        assert!(
            fft_result(1, 8, "unsupported", Some("1"), false)
                .validate(1, "fft-001")
                .is_err()
        );

        let mut inconsistent_expression = fft_result(2, 8, "parameter", None, false);
        inconsistent_expression.authored_output = inconsistent_expression.source_text.clone();
        assert!(inconsistent_expression.validate(2, "fft-002").is_err());

        let mut impossible_bounds = fft_result(2, 8, "voltage", Some("V"), false);
        impossible_bounds.fundamental_bin = 2;
        impossible_bounds.minimum_metric_bin = 0;
        impossible_bounds.maximum_metric_bin = 0;
        impossible_bounds.sfdr_search_minimum_bin = 0;
        assert!(impossible_bounds.validate(2, "fft-002").is_err());

        let mut not_normalized = fft_result(1, 8, "voltage", Some("1"), false);
        not_normalized.real[1] = 0.5;
        not_normalized.magnitude[1] = 0.5;
        assert!(not_normalized.validate(1, "fft-001").is_err());

        let mut negative_sub_pico = fft_result(1, 8, "voltage", Some("1"), false);
        negative_sub_pico.magnitude[0] = -1.0e-300;
        assert!(negative_sub_pico.validate(1, "fft-001").is_err());
    }

    #[test]
    fn fft_metric_mutations_are_rejected_against_the_spectrum() {
        let valid = fft_result(1, 8, "voltage", Some("1"), true);
        assert!(valid.validate(1, "fft-001").is_ok());

        let mut wrong_fundamental = valid.clone();
        wrong_fundamental
            .metrics
            .as_mut()
            .expect("metric fixture")
            .fundamental_magnitude += 0.25;
        assert!(wrong_fundamental.validate(1, "fft-001").is_err());

        for mutate in [
            |metrics: &mut Hdf5FftMetrics| metrics.thd_ratio += 0.25,
            |metrics: &mut Hdf5FftMetrics| metrics.thd_db += 1.0,
            |metrics: &mut Hdf5FftMetrics| metrics.sndr_db += 1.0,
            |metrics: &mut Hdf5FftMetrics| metrics.enob_bits += 1.0,
            |metrics: &mut Hdf5FftMetrics| metrics.snr_db += 1.0,
            |metrics: &mut Hdf5FftMetrics| metrics.sfdr_db += 1.0,
        ] {
            let mut malformed = valid.clone();
            mutate(malformed.metrics.as_mut().expect("metric fixture"));
            assert!(malformed.validate(1, "fft-001").is_err());
        }

        let mut wrong_spur = valid.clone();
        let metrics = wrong_spur.metrics.as_mut().expect("metric fixture");
        metrics.sfdr_spur_bin = Some(2);
        metrics.sfdr_spur_frequency_hz = Some(2.0);
        assert!(wrong_spur.validate(1, "fft-001").is_err());

        let mut wrong_harmonic = valid;
        wrong_harmonic
            .metrics
            .as_mut()
            .expect("metric fixture")
            .largest_harmonics[0]
            .magnitude += 1.0e-6;
        assert!(wrong_harmonic.validate(1, "fft-001").is_err());
    }

    #[test]
    fn unsupported_root_and_fft_section_schemas_are_rejected() {
        let directory = TestDirectory::new("fft-future-schema");

        let future_root = directory.0.join("future-root.h5");
        let mut root_builder = FileBuilder::new();
        root_builder.set_attr("schema_version", AttrValue::String("2".to_string()));
        std::fs::write(
            &future_root,
            root_builder.finish().expect("encode future root schema"),
        )
        .expect("write future root schema");
        let root_error = read_hdf5(&future_root).expect_err("reject future root schema");
        assert!(matches!(root_error, Hdf5Error::InvalidSchema(_)));

        for (label, version) in [("old", "1"), ("future", "3")] {
            let fft_path = directory.0.join(format!("{label}-fft.h5"));
            let mut fft_builder = FileBuilder::new();
            fft_builder.set_attr(
                "schema_version",
                AttrValue::String(SCHEMA_VERSION.to_string()),
            );
            let mut fft_group = fft_builder.create_group("fft");
            fft_group.set_attr("schema_version", AttrValue::String(version.to_string()));
            fft_builder.add_group(fft_group.finish());
            std::fs::write(
                &fft_path,
                fft_builder.finish().expect("encode unsupported FFT schema"),
            )
            .expect("write unsupported FFT schema");
            let fft_error = read_hdf5(&fft_path).expect_err("reject unsupported FFT schema");
            assert!(matches!(fft_error, Hdf5Error::InvalidSchema(_)));
        }
    }
}
