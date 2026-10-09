//! Periodic Transfer Function (PXF) Analysis Module
//!
//! PXF analysis computes the frequency-domain transfer function from a small-signal
//! input to circuit outputs around a periodic steady-state operating point. This is
//! essential for:
//!
//! - **Mixer noise figure**: Transfer of noise sources to output sidebands
//! - **LNA gain compression**: Small-signal gain vs. large-signal interference
//! - **Frequency converter sensitivity**: Input-to-output sideband mapping
//! - **Oscillator injection locking**: Response to external perturbations
//!
//! # Theory
//!
//! PXF analysis extends standard AC transfer function analysis to time-varying
//! systems. For a periodic operating point with period T, the transfer function
//! H(s, t) is also periodic in t. Using Floquet/LPTV theory:
//!
//! H(s, t) = Σₖ Hₖ(s) · exp(jk·ω₀·t)
//!
//! where ω₀ = 2π/T is the fundamental frequency and Hₖ(s) are the harmonic
//! transfer functions (conversion gains).
//!
//! # Relationship to PAC
//!
//! PXF and PAC are complementary:
//! - **PAC**: Sweeps input frequency, observes response at all sidebands
//! - **PXF**: Fixes sideband relationship, sweeps both input/output frequency
//!
//! PXF provides the complete transfer function matrix H(f_in, f_out) for any
//! frequency pair related by the LO frequency.

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, frequency_point_count, generate_frequency_grid,
    validate_generated_sweep,
};
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// PXF Configuration
//=============================================================================

/// Configuration for Periodic Transfer Function (PXF) analysis
#[derive(Debug, Clone)]
pub struct PxfConfig {
    /// Start frequency for sweep (Hz)
    pub freq_start: Value,

    /// Stop frequency for sweep (Hz)
    pub freq_stop: Value,

    /// Number of frequency points
    pub num_points: usize,

    /// Sweep type
    pub sweep_type: PxfSweepType,

    /// Input sideband index (relative to LO)
    pub input_sideband: i32,

    /// Output sideband index (relative to LO)
    pub output_sideband: i32,

    /// Maximum number of sidebands to include in computation
    pub max_sidebands: usize,

    /// Input source name
    pub input_source: Option<String>,

    /// Output node name
    pub output_node: Option<String>,

    /// Reference node (usually ground)
    pub ref_node: String,

    /// Fundamental (LO) frequency from PSS
    pub fundamental_freq: Value,

    /// Include noise transfer (for noise figure computation)
    pub include_noise: bool,
}

/// Sweep type for PXF frequency sweep
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PxfSweepType {
    /// Linear frequency sweep
    Linear,
    /// Decade (logarithmic) sweep
    #[default]
    Decade,
    /// Octave sweep
    Octave,
}

impl Default for PxfConfig {
    fn default() -> Self {
        Self {
            freq_start: 1e3,
            freq_stop: 1e9,
            num_points: 50,
            sweep_type: PxfSweepType::Decade,
            input_sideband: 1,  // Default: RF input (LO + IF)
            output_sideband: 0, // Default: IF output (baseband)
            max_sidebands: 5,
            input_source: None,
            output_node: None,
            ref_node: "0".to_string(),
            fundamental_freq: 0.0,
            include_noise: false,
        }
    }
}

impl PxfConfig {
    /// Create new PXF configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency sweep range
    pub fn with_sweep(mut self, start: Value, stop: Value, points: usize) -> Self {
        self.freq_start = start;
        self.freq_stop = stop;
        self.num_points = points;
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PxfSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set input/output sideband pair
    pub fn with_sidebands(mut self, input: i32, output: i32) -> Self {
        self.input_sideband = input;
        self.output_sideband = output;
        self
    }

    /// Set input source
    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = Some(source.to_uppercase());
        self
    }

    /// Set output node
    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = Some(node.to_uppercase());
        self
    }

    /// Set fundamental frequency
    pub fn with_fundamental(mut self, freq: Value) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Generate frequency points while preserving validation and resource failures.
    pub fn frequency_points(&self) -> Result<Vec<Value>, PxfError> {
        self.try_frequency_points()
    }

    /// Generate frequency points, preserving validation failures for callers
    /// that need to distinguish invalid input from a deliberately empty grid.
    pub fn try_frequency_points(&self) -> Result<Vec<Value>, PxfError> {
        self.try_frequency_points_with_abort(&NoAbort)
    }

    /// Generate frequency points with cooperative cancellation.
    pub(crate) fn try_frequency_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, PxfError> {
        generate_frequency_grid(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            false,
            abort,
        )
        .map_err(PxfError::FrequencyGrid)
    }

    /// Number of points the configured sweep will retain without allocating it.
    pub fn frequency_point_count(&self) -> Result<usize, PxfError> {
        self.validate_frequency_sweep()?;
        frequency_point_count(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
        )
        .map_err(PxfError::FrequencyGrid)
    }

    fn grid_scale(&self) -> FrequencyGridScale {
        match self.sweep_type {
            PxfSweepType::Linear => FrequencyGridScale::Linear,
            PxfSweepType::Decade => FrequencyGridScale::Decade,
            PxfSweepType::Octave => FrequencyGridScale::Octave,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PxfError> {
        self.validate_frequency_sweep()
    }

    fn validate_frequency_sweep(&self) -> Result<(), PxfError> {
        validate_generated_sweep(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            false,
        )
        .map_err(PxfError::FrequencyGrid)
    }
}

//=============================================================================
// PXF Error
//=============================================================================

/// Errors during PXF analysis
#[derive(Debug, Clone)]
pub enum PxfError {
    /// Frequency-grid validation, capacity, allocation, or cancellation failure.
    FrequencyGrid(FrequencyGridError),
    /// Invalid frequency specification
    InvalidFrequency(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing PSS solution
    MissingPssSolution(String),
    /// Computation error
    ComputationError(String),
}

impl std::fmt::Display for PxfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PxfError::FrequencyGrid(error) => write!(f, "PXF frequency grid: {error}"),
            PxfError::InvalidFrequency(s) => write!(f, "Invalid frequency: {}", s),
            PxfError::InvalidConfiguration(s) => write!(f, "Invalid configuration: {}", s),
            PxfError::MissingPssSolution(s) => write!(f, "Missing PSS solution: {}", s),
            PxfError::ComputationError(s) => write!(f, "Computation error: {}", s),
        }
    }
}

impl std::error::Error for PxfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FrequencyGrid(source) => Some(source),
            _ => None,
        }
    }
}

//=============================================================================
// Transfer Function Point
//=============================================================================

/// A single transfer function point in PXF analysis
#[derive(Debug, Clone)]
pub struct TransferPoint {
    /// Swept offset frequency, in hertz: the abscissa the card authored, and
    /// the one every curve metric below is stated against. The absolute
    /// frequency the drive is applied at is `freq_in + sideband_in * f0`.
    pub freq_in: Value,

    /// Absolute frequency the converted response appears at, in hertz:
    /// `freq_in + sideband_out * f0`.
    pub freq_out: Value,

    /// Complex transfer function H(f_in → f_out)
    pub transfer: Complex64,

    /// Input sideband index
    pub sideband_in: i32,

    /// Output sideband index
    pub sideband_out: i32,
}

impl TransferPoint {
    /// Get magnitude in linear scale
    pub fn magnitude(&self) -> Value {
        self.transfer.norm()
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Value {
        20.0 * crate::numerics::complex_log10_magnitude(self.transfer)
    }

    /// Get phase in radians
    pub fn phase(&self) -> Value {
        self.transfer.arg()
    }

    /// Get phase in degrees
    pub fn phase_degrees(&self) -> Value {
        self.transfer.arg() * 180.0 / PI
    }

    /// Difference-derived group delay over the next interval.
    ///
    /// NaN denotes an unavailable delay: zero or invalid transfer values,
    /// invalid frequency spacing, or a result outside the finite range.
    pub fn group_delay(&self, next: &TransferPoint) -> Value {
        let df = next.freq_in - self.freq_in;
        if !self.freq_in.is_finite()
            || !next.freq_in.is_finite()
            || !df.is_finite()
            || df <= 0.0
            || [self, next].iter().any(|point| {
                !point.transfer.re.is_finite()
                    || !point.transfer.im.is_finite()
                    || point.transfer == Complex64::new(0.0, 0.0)
            })
        {
            return Value::NAN;
        }
        let dphi = next.phase() - self.phase();
        // Unwrap phase if needed
        let dphi_unwrapped = if dphi > PI {
            dphi - 2.0 * PI
        } else if dphi < -PI {
            dphi + 2.0 * PI
        } else {
            dphi
        };
        // Preserve every resolvable interval. Multiplying a very large finite
        // interval by 2π can overflow even when the delay is representable.
        let delay = if df.abs() > f64::MAX / (2.0 * PI) {
            (-dphi_unwrapped / (2.0 * PI)) / df
        } else {
            -dphi_unwrapped / (2.0 * PI * df)
        };
        if delay.is_finite() { delay } else { Value::NAN }
    }
}

//=============================================================================
// PXF Result
//=============================================================================

/// Result of PXF analysis
#[derive(Debug, Clone)]
pub struct PxfResult {
    /// Fundamental (LO) frequency
    pub fundamental_freq: Value,

    /// Input sideband index
    pub input_sideband: i32,

    /// Output sideband index
    pub output_sideband: i32,

    /// Transfer function points
    pub points: Vec<TransferPoint>,

    /// Node names
    pub node_names: Vec<String>,

    /// Transfer at zero offset, when actually evaluated there.
    pub dc_gain: Option<Complex64>,

    /// Peak gain and frequency
    pub peak_gain: Option<(Value, Value)>, // (frequency, gain_db)

    /// 3dB bandwidth (if applicable)
    pub bandwidth_3db: Option<Value>,

    /// Unity gain frequency
    pub unity_gain_freq: Option<Value>,
}

impl PxfResult {
    /// Create new PXF result
    pub fn new(fundamental_freq: Value, input_sb: i32, output_sb: i32) -> Self {
        Self {
            fundamental_freq,
            input_sideband: input_sb,
            output_sideband: output_sb,
            points: Vec::new(),
            node_names: Vec::new(),
            dc_gain: None,
            peak_gain: None,
            bandwidth_3db: None,
            unity_gain_freq: None,
        }
    }

    /// Add a transfer point
    pub fn add_point(&mut self, point: TransferPoint) {
        self.points.push(point);
    }

    /// Get number of frequency points
    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    /// Get magnitude curve (frequency, magnitude_db)
    pub fn magnitude_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.freq_in, p.magnitude_db()))
            .collect()
    }

    /// Get phase curve (frequency, phase_degrees)
    pub fn phase_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.freq_in, p.phase_degrees()))
            .collect()
    }

    /// Midpoint group-delay curve, with NaN gaps for unavailable intervals.
    /// Every interval is retained so a plot cannot bridge undefined phase.
    pub fn group_delay_curve(&self) -> Vec<(Value, Value)> {
        if self.points.len() < 2 {
            return Vec::new();
        }

        self.points
            .windows(2)
            .map(|w| {
                let gd = w[0].group_delay(&w[1]);
                (w[0].freq_in.midpoint(w[1].freq_in), gd)
            })
            .collect()
    }

    /// Find the highest finite decibel gain on a valid offset sweep.
    pub fn find_peak_gain(&self) -> Option<(Value, Value)> {
        if !self.valid_metric_sweep() {
            return None;
        }
        self.peak_index()
            .map(|(index, gain)| (self.points[index].freq_in, gain))
    }

    fn peak_index(&self) -> Option<(usize, Value)> {
        self.points
            .iter()
            .enumerate()
            .map(|(index, point)| (index, point.magnitude_db()))
            .filter(|(_, db)| db.is_finite())
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }

    fn valid_metric_sweep(&self) -> bool {
        self.points.iter().all(|point| {
            point.freq_in.is_finite()
                && point.freq_in >= 0.0
                && point.transfer.re.is_finite()
                && point.transfer.im.is_finite()
        }) && self
            .points
            .windows(2)
            .all(|pair| pair[0].freq_in < pair[1].freq_in)
    }

    /// Width of the peak's contiguous passband, three decibels below its peak.
    ///
    /// Cutoffs are interpolated linearly in decibels versus offset frequency,
    /// as for unity gain. Both edges must be observed; a sweep starting at DC
    /// above the threshold has a known lower edge of zero. A positive sweep
    /// endpoint or an interval touching a zero transfer cannot supply a missing
    /// cutoff. Such an unresolved bandwidth is `None`.
    pub fn find_bandwidth_3db(&self) -> Option<Value> {
        if !self.valid_metric_sweep() {
            return None;
        }
        let (peak_index, peak_db) = self.peak_index()?;
        let threshold = peak_db - 3.0;
        let lower = if let Some(pair) = self
            .points
            .windows(2)
            .take(peak_index)
            .rev()
            .find(|pair| pair[0].magnitude_db() <= threshold)
        {
            crossing(&pair[0], &pair[1], threshold)?
        } else if self.points.first()?.freq_in == 0.0 {
            0.0
        } else {
            return None;
        };
        let pair = self
            .points
            .windows(2)
            .skip(peak_index)
            .find(|pair| pair[1].magnitude_db() <= threshold)?;
        let upper = crossing(&pair[0], &pair[1], threshold)?;
        Some(upper - lower)
    }

    /// First observed unity-gain sample or finite, interpolated 0 dB crossing.
    ///
    /// Interpolation is linear in decibels versus offset frequency. Intervals
    /// touching a zero transfer have no finite logarithmic interpolation and
    /// are skipped; subsequent finite intervals are still examined.
    pub fn find_unity_gain_freq(&self) -> Option<Value> {
        if !self.valid_metric_sweep() {
            return None;
        }
        for (index, point) in self.points.iter().enumerate() {
            if point.magnitude_db() == 0.0 {
                return Some(point.freq_in);
            }
            if let Some(next) = self.points.get(index + 1)
                && let Some(frequency) = crossing(point, next, 0.0)
            {
                return Some(frequency);
            }
        }
        None
    }

    /// Recompute every derived metric, clearing determinations no longer held.
    pub fn compute_metrics(&mut self) {
        self.peak_gain = self.find_peak_gain();
        self.bandwidth_3db = self.find_bandwidth_3db();
        self.unity_gain_freq = self.find_unity_gain_freq();
        // A positive offset, however small, is not a measurement at DC.
        self.dc_gain = self
            .points
            .first()
            .filter(|point| self.valid_metric_sweep() && point.freq_in == 0.0)
            .map(|point| point.transfer);
    }
}

/// Interpolate a finite decibel bracket, including exact endpoint samples.
fn crossing(first: &TransferPoint, next: &TransferPoint, threshold: Value) -> Option<Value> {
    let left = first.magnitude_db();
    let right = next.magnitude_db();
    if left == threshold {
        return Some(first.freq_in);
    }
    if right == threshold {
        return Some(next.freq_in);
    }
    if !left.is_finite() || !right.is_finite() || (left < threshold) == (right < threshold) {
        return None;
    }
    let alpha = (threshold - left) / (right - left);
    let frequency = alpha.mul_add(next.freq_in - first.freq_in, first.freq_in);
    frequency.is_finite().then_some(frequency)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn response(samples: &[(f64, Complex64)]) -> PxfResult {
        let mut result = PxfResult::new(1000.0, 0, 0);
        result.points = samples
            .iter()
            .map(|&(frequency, transfer)| TransferPoint {
                freq_in: frequency,
                freq_out: frequency,
                transfer,
                sideband_in: 0,
                sideband_out: 0,
            })
            .collect();
        result
    }

    #[test]
    fn bandwidth_interpolates_both_edges_of_the_peak_passband() {
        let result = response(&[
            (10.0, Complex64::new(1.0, 0.0)),
            (20.0, Complex64::new(10f64.powf(6.0 / 20.0), 0.0)),
            (30.0, Complex64::new(1.0, 0.0)),
        ]);
        assert!((result.find_bandwidth_3db().unwrap() - 10.0).abs() < 1e-12);
        for points in [&result.points[..2], &result.points[1..]] {
            let mut truncated = result.clone();
            truncated.points = points.to_vec();
            assert_eq!(
                truncated.find_bandwidth_3db(),
                None,
                "an unresolved edge is not a cutoff"
            );
        }
        let lowpass = response(&[
            (0.0, Complex64::new(10f64.powf(6.0 / 20.0), 0.0)),
            (20.0, Complex64::new(1.0, 0.0)),
        ]);
        assert!((lowpass.find_bandwidth_3db().unwrap() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn unity_gain_detects_exact_samples_including_singletons_and_endpoints() {
        for gains in [vec![1.0], vec![2.0, 1.0], vec![1.0, 2.0], vec![1.0, 1.0]] {
            let samples = gains
                .iter()
                .enumerate()
                .map(|(i, &gain)| ((i + 1) as f64, Complex64::new(gain, 0.0)))
                .collect::<Vec<_>>();
            let expected = gains.iter().position(|&gain| gain == 1.0).unwrap() as f64 + 1.0;
            assert_eq!(response(&samples).find_unity_gain_freq(), Some(expected));
        }
    }

    #[test]
    fn unity_gain_does_not_interpolate_through_undefined_log_magnitudes() {
        for gains in [[0.0, 2.0], [2.0, 0.0]] {
            let samples = [
                (1.0, Complex64::new(gains[0], 0.0)),
                (2.0, Complex64::new(gains[1], 0.0)),
            ];
            assert_eq!(response(&samples).find_unity_gain_freq(), None);
        }
        let result = response(&[
            (1.0, Complex64::new(0.0, 0.0)),
            (2.0, Complex64::new(2.0, 0.0)),
            (4.0, Complex64::new(0.5, 0.0)),
        ]);
        assert_eq!(result.find_unity_gain_freq(), Some(3.0));
    }

    #[test]
    fn finite_complex_transfers_keep_representable_decibel_magnitudes() {
        for value in [f64::MAX, f64::from_bits(1)] {
            let result = response(&[(1.0, Complex64::new(value, value))]);
            let expected = 20.0 * value.log10() + 10.0 * 2.0_f64.log10();
            assert!((result.points[0].magnitude_db() - expected).abs() < 1e-10);
            assert!((result.find_peak_gain().unwrap().1 - expected).abs() < 1e-10);
        }
    }

    #[test]
    fn recomputed_dc_gain_requires_an_actual_dc_sample() {
        let mut result = response(&[(0.0, Complex64::new(2.0, 0.0))]);
        result.compute_metrics();
        assert_eq!(result.dc_gain, Some(Complex64::new(2.0, 0.0)));
        result.points[0].freq_in = 10.0;
        result.compute_metrics();
        assert_eq!(result.dc_gain, None, "10 Hz does not establish DC gain");
        result.points.clear();
        result.dc_gain = Some(Complex64::new(2.0, 0.0));
        result.compute_metrics();
        assert_eq!(
            result.dc_gain, None,
            "removed samples cannot leave stale metrics"
        );
    }

    #[test]
    fn invalid_sweeps_cannot_produce_finite_curve_metrics() {
        for frequency in [-1.0, 0.0, f64::NAN, f64::INFINITY] {
            let mut result = response(&[
                (0.0, Complex64::new(2.0, 0.0)),
                (frequency, Complex64::new(0.5, 0.0)),
            ]);
            result.compute_metrics();
            assert_eq!(result.peak_gain, None);
            assert_eq!(result.bandwidth_3db, None);
            assert_eq!(result.unity_gain_freq, None);
            assert_eq!(result.dc_gain, None);
        }
    }

    #[test]
    fn group_delay_keeps_gaps_at_zero_transfer_and_resumes_after_them() {
        let result = response(&[
            (1.0, Complex64::new(1.0, 0.0)),
            (2.0, Complex64::new(0.0, -1.0)),
            (3.0, Complex64::new(0.0, 0.0)),
            (4.0, Complex64::new(1.0, 0.0)),
            (5.0, Complex64::new(0.0, -1.0)),
        ]);
        let curve = result.group_delay_curve();
        assert_eq!(
            curve.iter().map(|sample| sample.0).collect::<Vec<_>>(),
            vec![1.5, 2.5, 3.5, 4.5]
        );
        assert_eq!(curve[0].1, 0.25);
        assert!(curve[1].1.is_nan());
        assert!(curve[2].1.is_nan());
        assert_eq!(curve[3].1, 0.25);
    }

    #[test]
    fn unrepresentable_or_invalid_delay_is_unavailable_without_clipping() {
        let result = response(&[
            (1e-320, Complex64::new(1.0, 0.0)),
            (2e-320, Complex64::new(0.0, -1.0)),
        ]);
        assert!(result.points[0].group_delay(&result.points[1]).is_nan());
        let first = &result.points[0];
        for frequency in [first.freq_in, 0.0, f64::INFINITY, f64::NAN] {
            let mut next = result.points[1].clone();
            next.freq_in = frequency;
            assert!(first.group_delay(&next).is_nan());
        }
    }

    #[test]
    fn group_delay_preserves_resolvable_small_and_large_intervals() {
        for (start, stop) in [(1e-16, 2e-16), (1e307, 1e308)] {
            let first = TransferPoint {
                freq_in: start,
                freq_out: start,
                transfer: Complex64::new(1.0, 0.0),
                sideband_in: 0,
                sideband_out: 0,
            };
            let next = TransferPoint {
                freq_in: stop,
                freq_out: stop,
                transfer: Complex64::new(0.0, -1.0),
                ..first.clone()
            };
            // A quarter-cycle phase fall over the interval gives 1/(4 df).
            let expected = 0.25 / (stop - start);
            let actual = first.group_delay(&next);
            assert!((actual / expected - 1.0).abs() < 1e-14, "{start}: {actual}");
        }
    }

    #[test]
    fn group_delay_midpoint_does_not_overflow_a_finite_sweep() {
        let mut result = PxfResult::new(1.0, 0, 0);
        for freq_in in [1e308, 1.5e308] {
            result.add_point(TransferPoint {
                freq_in,
                freq_out: freq_in,
                transfer: Complex64::new(1.0, 0.0),
                sideband_in: 0,
                sideband_out: 0,
            });
        }
        assert_eq!(result.group_delay_curve(), vec![(1.25e308, 0.0)]);
    }

    #[test]
    fn validate_rejects_non_finite_sweep_values() {
        for config in [
            PxfConfig::new().with_sweep(f64::NAN, 1.0e3, 10),
            PxfConfig::new().with_sweep(1.0, f64::INFINITY, 10),
            PxfConfig::new().with_sweep(1.0, 1.0e3, 0),
        ] {
            assert!(
                config.validate().is_err(),
                "invalid PXF sweep config unexpectedly accepted: {config:?}"
            );
        }
    }

    #[test]
    fn try_frequency_points_preserves_validation_error() {
        let config = PxfConfig::new().with_sweep(1.0e6, 1.0, 10);

        let err = config
            .try_frequency_points()
            .expect_err("invalid PXF sweep should return the validation error");

        assert!(matches!(
            err,
            PxfError::FrequencyGrid(FrequencyGridError::InvalidStopFrequency)
        ));
        assert!(matches!(
            config.frequency_points(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::InvalidStopFrequency
            ))
        ));
    }

    #[test]
    fn frequency_grid_is_checked_fallible_and_cancellable() {
        assert_eq!(
            PxfConfig::new()
                .with_sweep(1.0, 2.0, 3)
                .with_sweep_type(PxfSweepType::Linear)
                .frequency_points()
                .expect("ordinary PXF grid"),
            vec![1.0, 1.5, 2.0]
        );
        assert!(matches!(
            PxfConfig::new()
                .with_sweep(1.0, 2.0, usize::MAX)
                .with_sweep_type(PxfSweepType::Linear)
                .frequency_points(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::Allocation { .. }
            ))
        ));
        assert!(matches!(
            PxfConfig::new().try_frequency_points_with_abort(&crate::abort_signal::ImmediateAbort),
            Err(PxfError::FrequencyGrid(FrequencyGridError::Aborted))
        ));
        assert!(matches!(
            PxfConfig::new()
                .with_sweep(f64::MIN_POSITIVE, f64::MAX, usize::MAX)
                .frequency_point_count(),
            Err(PxfError::FrequencyGrid(
                FrequencyGridError::PointCountOverflow
            ))
        ));
        assert_eq!(
            PxfConfig::new()
                .with_sweep(1.0e3, 1.0e3, 10)
                .frequency_points()
                .expect("equal PXF endpoints remain valid"),
            vec![1.0e3]
        );
    }
}
