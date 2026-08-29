//! Waveform Measurements for Transient Analysis
//!
//! Provides comprehensive signal processing and measurement functions
//! for transient simulation results, matching commercial simulator capabilities:
//!
//! # Timing Measurements
//! - Rise time (10%-90%, configurable thresholds)
//! - Fall time (90%-10%, configurable thresholds)
//! - Delay (propagation delay between signals)
//! - Period and frequency
//! - Pulse width
//! - Duty cycle
//!
//! # Amplitude Measurements  
//! - Peak-to-peak
//! - Overshoot / undershoot
//! - Average (DC component)
//! - RMS
//! - Min / max
//!
//! # Threshold Crossing
//! - Cross time (first crossing of threshold)
//! - All crossings with direction
//! - Slew rate at crossing
//!
//! # Spectral Analysis
//! - Qualified one-sided FFT peak-amplitude spectra
//! - Explicit-fundamental THD (Total Harmonic Distortion)
//! - Dominant non-DC frequency estimation
//!
//! # Usage
//! ```ignore
//! let waveform = Waveform::new(&time_points, &voltage_values)?;
//! let rise_time = waveform.rise_time(0.1, 0.9)?;
//! let thd = waveform.thd(fundamental_freq, DEFAULT_THD_HARMONICS)?;
//! ```

use crate::Value;
use crate::analysis::fourier::{FourierAnalysis, FourierConfig, FourierError};
use rustfft::{FftPlanner, num_complex::Complex};

//=============================================================================
// Constants
//=============================================================================

/// Default low threshold for rise/fall time (10%)
pub const THRESHOLD_LOW: Value = 0.1;
/// Default high threshold for rise/fall time (90%)
pub const THRESHOLD_HIGH: Value = 0.9;
/// Minimum number of samples for FFT
pub const MIN_FFT_SAMPLES: usize = 8;
/// Bound FFT planning before entering `rustfft`'s infallible plan allocations.
const MAX_QUALIFIED_FFT_SAMPLES: usize = 1_048_576;
/// Bound resampling before allocating both the output grid and values.
const MAX_QUALIFIED_RESAMPLE_POINTS: usize = 1_048_576;
/// Default number of harmonics for THD calculation
pub const DEFAULT_THD_HARMONICS: usize = 10;

//=============================================================================
// Edge Direction
//=============================================================================

/// Direction of threshold crossing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EdgeDirection {
    /// Rising edge (crossing from below)
    Rising,
    /// Falling edge (crossing from above)
    Falling,
    /// Either direction
    #[default]
    Either,
}

//=============================================================================
// Threshold Crossing
//=============================================================================

/// A threshold crossing event
#[derive(Debug, Clone)]
pub struct CrossingEvent {
    /// Time of crossing (interpolated)
    pub time: Value,
    /// Index of sample before crossing
    pub index: usize,
    /// Direction of crossing
    pub direction: EdgeDirection,
    /// Slew rate at crossing (dV/dt)
    pub slew_rate: Value,
    /// Value at crossing (should be ~threshold)
    pub value: Value,
}

//=============================================================================
// Measurement Result
//=============================================================================

/// Result of a measurement operation
#[derive(Debug, Clone)]
pub struct MeasurementResult {
    /// Measured value
    pub value: Value,
    /// Time at which measurement applies (if applicable)
    pub time: Option<Value>,
    /// Index in waveform (if applicable)
    pub index: Option<usize>,
    /// Unit of measurement (for display)
    pub unit: &'static str,
    /// Description of measurement
    pub description: String,
}

impl MeasurementResult {
    /// Create a new measurement result
    pub fn new(value: Value, unit: &'static str, description: impl Into<String>) -> Self {
        Self {
            value,
            time: None,
            index: None,
            unit,
            description: description.into(),
        }
    }
}

//=============================================================================
// Measurement Error
//=============================================================================

/// Errors that can occur during measurement
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementError {
    /// Invalid waveform input
    InvalidWaveform(String),
    /// Insufficient data points
    InsufficientData(String),
    /// Threshold never crossed
    ThresholdNotCrossed(String),
    /// Invalid threshold values
    InvalidThreshold(String),
    /// Calculation error
    CalculationError(String),
    /// FFT error
    FftError(String),
}

impl std::fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWaveform(msg) => write!(f, "Invalid waveform: {}", msg),
            Self::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            Self::ThresholdNotCrossed(msg) => write!(f, "Threshold not crossed: {}", msg),
            Self::InvalidThreshold(msg) => write!(f, "Invalid threshold: {}", msg),
            Self::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
            Self::FftError(msg) => write!(f, "FFT error: {}", msg),
        }
    }
}

impl std::error::Error for MeasurementError {}

//=============================================================================
// Waveform
//=============================================================================

/// Waveform data for measurement operations
///
/// Provides comprehensive measurement functions on time-domain data.
#[derive(Debug, Clone)]
pub struct Waveform {
    /// Time points
    time: Vec<Value>,
    /// Signal values at each time point
    values: Vec<Value>,
    /// Cached min value
    min_value: Value,
    /// Cached max value
    max_value: Value,
    /// Cached min time
    min_time: Value,
    /// Cached max time
    max_time: Value,
}

impl Waveform {
    /// Create a new waveform from time and value arrays
    ///
    /// # Arguments
    /// * `time` - Time points (must be monotonically increasing)
    /// * `values` - Signal values at each time point
    ///
    pub fn new(time: &[Value], values: &[Value]) -> Result<Self, MeasurementError> {
        Self::try_new(time, values)
    }

    /// Create a new waveform from time and value arrays.
    pub fn try_new(time: &[Value], values: &[Value]) -> Result<Self, MeasurementError> {
        validate_waveform_input(time, values)?;
        let mut owned_time = Vec::new();
        owned_time.try_reserve_exact(time.len()).map_err(|error| {
            MeasurementError::CalculationError(format!(
                "failed to allocate {} waveform time points: {error}",
                time.len()
            ))
        })?;
        owned_time.extend_from_slice(time);

        let mut owned_values = Vec::new();
        owned_values
            .try_reserve_exact(values.len())
            .map_err(|error| {
                MeasurementError::CalculationError(format!(
                    "failed to allocate {} waveform values: {error}",
                    values.len()
                ))
            })?;
        owned_values.extend_from_slice(values);

        Ok(Self::from_owned_validated_parts(owned_time, owned_values))
    }

    fn from_owned_validated_parts(time: Vec<Value>, values: Vec<Value>) -> Self {
        let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_time = time[0];
        let max_time = time[time.len() - 1];

        Self {
            time,
            values,
            min_value,
            max_value,
            min_time,
            max_time,
        }
    }

    /// Get number of samples
    #[inline]
    pub fn len(&self) -> usize {
        self.time.len()
    }

    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.time.is_empty()
    }

    /// Get the representable authored time span.
    ///
    /// A one-point waveform has an exact zero duration. A multi-point span
    /// that overflows or otherwise cannot be represented fails closed.
    pub fn duration(&self) -> Result<Value, MeasurementError> {
        if self.time.is_empty() || self.time.len() != self.values.len() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "duration requires a nonempty matched time/value grid, got {} time point(s) and {} value(s)",
                self.time.len(),
                self.values.len()
            )));
        }
        if self.time.len() == 1 {
            return Ok(0.0);
        }
        representable_positive_difference(
            self.max_time,
            self.min_time,
            self.time.len() - 2,
            "waveform duration",
        )
    }

    /// Get the average sample rate over the authored time span.
    pub fn sample_rate(&self) -> Result<Value, MeasurementError> {
        if self.time.len() < 2 {
            return Err(MeasurementError::InsufficientData(format!(
                "sample rate requires at least 2 points, got {}",
                self.time.len()
            )));
        }
        if self.time.len() != self.values.len() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "sample rate requires matching time/value lengths, got {} and {}",
                self.time.len(),
                self.values.len()
            )));
        }
        let segment = self.time.len() - 2;
        let time_span = scaled_positive_difference(
            self.max_time,
            self.min_time,
            segment,
            "sample-rate time span",
        )?;
        let interval_count = self.time.len() - 1;
        let interval_count_value = interval_count as Value;
        if !interval_count_value.is_finite() || interval_count_value as usize != interval_count {
            return Err(MeasurementError::CalculationError(format!(
                "sample interval count {interval_count} cannot be represented exactly"
            )));
        }
        let scaled_interval_count =
            scaled_positive_value(interval_count_value, segment, "sample interval count")?;
        scaled_positive_ratio(
            scaled_interval_count,
            time_span,
            segment,
            "average sample rate",
        )
    }

    /// Get value at index
    #[inline]
    pub fn value_at(&self, index: usize) -> Option<Value> {
        self.values.get(index).copied()
    }

    /// Get time at index
    #[inline]
    pub fn time_at(&self, index: usize) -> Option<Value> {
        self.time.get(index).copied()
    }

    //=========================================================================
    // Amplitude Measurements
    //=========================================================================

    /// Get minimum value
    #[inline]
    pub fn min(&self) -> Value {
        self.min_value
    }

    /// Get maximum value
    #[inline]
    pub fn max(&self) -> Value {
        self.max_value
    }

    /// Get the representable peak-to-peak amplitude.
    ///
    /// A constant waveform has an exact zero amplitude. A nonzero range that
    /// cannot be represented as a finite value returns an error.
    pub fn peak_to_peak(&self) -> Result<Value, MeasurementError> {
        if !self.min_value.is_finite()
            || !self.max_value.is_finite()
            || self.min_value > self.max_value
        {
            return Err(MeasurementError::InvalidWaveform(format!(
                "cached waveform range is invalid: [{}, {}]",
                self.min_value, self.max_value
            )));
        }
        if self.min_value == self.max_value {
            return Ok(0.0);
        }
        representable_positive_difference(
            self.max_value,
            self.min_value,
            0,
            "peak-to-peak amplitude",
        )
    }

    /// Calculate the time-weighted average using trapezoidal integration.
    ///
    /// At least two samples spanning a finite positive duration are required.
    /// Nonuniform sample intervals are weighted by their duration.
    ///
    /// Returns an error when the time span is invalid or the qualified
    /// floating-point result cannot be represented without losing evidence.
    pub fn average(&self) -> Result<Value, MeasurementError> {
        let (scale, normalized_moment) = self.normalized_time_weighted_moment(false)?;
        scaled_moment_result(scale, normalized_moment, "average")
    }

    /// Calculate the time-weighted RMS using trapezoidal integration.
    ///
    /// At least two samples spanning a finite positive duration are required.
    /// Nonuniform sample intervals are weighted by their duration. Each stored
    /// endpoint is squared before trapezoidal integration; this is not the
    /// analytic integral of a squared piecewise-linear interpolant.
    ///
    /// Returns an error when the time span is invalid or the qualified
    /// floating-point result cannot be represented without losing evidence.
    pub fn rms(&self) -> Result<Value, MeasurementError> {
        let (scale, normalized_mean_square) = self.normalized_time_weighted_moment(true)?;
        if !(0.0..=1.0).contains(&normalized_mean_square) {
            return Err(MeasurementError::CalculationError(format!(
                "normalized RMS moment is outside [0, 1] ({normalized_mean_square})"
            )));
        }
        let normalized_rms = normalized_mean_square.sqrt();
        if !normalized_rms.is_finite() {
            return Err(MeasurementError::CalculationError(format!(
                "normalized RMS is non-finite ({normalized_rms})"
            )));
        }
        scaled_moment_result(scale, normalized_rms, "RMS")
    }

    fn normalized_time_weighted_moment(
        &self,
        square_values: bool,
    ) -> Result<(Value, Value), MeasurementError> {
        let sample_count = self.time.len();
        if sample_count != self.values.len() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "time and values must have the same length (got {sample_count} time point(s), {} value(s))",
                self.values.len()
            )));
        }
        if sample_count < 2 {
            return Err(MeasurementError::InsufficientData(format!(
                "time-weighted statistics require at least 2 samples, got {sample_count}"
            )));
        }

        let duration = scaled_positive_difference(
            self.time[sample_count - 1],
            self.time[0],
            sample_count - 2,
            "time-weighted statistics duration",
        )?;

        let mut scale = 0.0_f64;
        for (index, &value) in self.values.iter().enumerate() {
            if !value.is_finite() {
                return Err(MeasurementError::InvalidWaveform(format!(
                    "values[{index}] must be finite"
                )));
            }
            scale = scale.max(value.abs());
        }

        let mut weight_sum = 0.0;
        let mut weight_compensation = 0.0;
        let mut moment_sum = ExactFloatSum::default();
        for index in 1..sample_count {
            let interval = scaled_positive_difference(
                self.time[index],
                self.time[index - 1],
                index - 1,
                "time-weighted segment width",
            )?;
            let weight =
                scaled_positive_ratio(interval, duration, index - 1, "normalized interval weight")?;
            if weight > 1.0 {
                return Err(MeasurementError::CalculationError(format!(
                    "normalized interval weight for segment {} exceeds one ({weight})",
                    index - 1
                )));
            }

            let left = if scale == 0.0 {
                0.0
            } else {
                self.values[index - 1] / scale
            };
            let right = if scale == 0.0 {
                0.0
            } else {
                self.values[index] / scale
            };
            if self.values[index - 1] != 0.0 && left == 0.0 {
                return Err(MeasurementError::CalculationError(format!(
                    "normalizing values[{}] underflowed a nonzero sample",
                    index - 1
                )));
            }
            if self.values[index] != 0.0 && right == 0.0 {
                return Err(MeasurementError::CalculationError(format!(
                    "normalizing values[{index}] underflowed a nonzero sample"
                )));
            }
            compensated_add(
                &mut weight_sum,
                &mut weight_compensation,
                weight,
                "normalized interval weights",
            )?;

            if square_values {
                accumulate_rms_segment(&mut moment_sum, left, right, weight, index - 1)?;
            } else {
                accumulate_average_segment(&mut moment_sum, left, right, weight, index - 1)?;
            }
        }
        let weight_sum = compensated_total(
            weight_sum,
            weight_compensation,
            "normalized interval weights",
        )?;
        if weight_sum <= 0.0 {
            return Err(MeasurementError::CalculationError(format!(
                "normalized interval weight sum is invalid ({weight_sum})"
            )));
        }

        let moment_sum = moment_sum.finish()?;
        let normalized_moment = moment_sum / weight_sum;
        if moment_sum != 0.0 && normalized_moment == 0.0 {
            return Err(MeasurementError::CalculationError(
                "normalizing the accumulated waveform moment underflowed".to_string(),
            ));
        }
        let moment_bound = if square_values { 0.0..=1.0 } else { -1.0..=1.0 };
        if !normalized_moment.is_finite() || !moment_bound.contains(&normalized_moment) {
            return Err(MeasurementError::CalculationError(format!(
                "normalized time-weighted moment is invalid ({normalized_moment})"
            )));
        }
        Ok((scale, normalized_moment))
    }

    /// Calculate overshoot relative to the authored initial-to-final step.
    ///
    /// Overshoot = (peak - final) / (final - initial) × 100%
    /// An exact zero step has an undefined normalization and returns an error.
    /// Every nonzero finite step is evaluated without an absolute cutoff.
    pub fn overshoot(&self) -> Result<Value, MeasurementError> {
        if self.values.len() < 2 {
            return Err(MeasurementError::InsufficientData(
                "overshoot requires at least 2 points".to_string(),
            ));
        }

        let initial = self.values[0];
        let final_val = self.values[self.values.len() - 1];
        if final_val == initial {
            return Err(MeasurementError::CalculationError(
                "overshoot is undefined for an exact zero initial-to-final step".to_string(),
            ));
        }

        if final_val > initial {
            qualified_difference_percentage(
                self.max_value,
                final_val,
                final_val,
                initial,
                "overshoot",
            )
        } else {
            qualified_difference_percentage(
                final_val,
                self.min_value,
                initial,
                final_val,
                "overshoot",
            )
        }
    }

    /// Calculate undershoot relative to the authored initial-to-final step.
    ///
    /// An exact zero step has an undefined normalization and returns an error.
    /// Every nonzero finite step is evaluated without an absolute cutoff.
    pub fn undershoot(&self) -> Result<Value, MeasurementError> {
        if self.values.len() < 2 {
            return Err(MeasurementError::InsufficientData(
                "undershoot requires at least 2 points".to_string(),
            ));
        }

        let initial = self.values[0];
        let final_val = self.values[self.values.len() - 1];
        if final_val == initial {
            return Err(MeasurementError::CalculationError(
                "undershoot is undefined for an exact zero initial-to-final step".to_string(),
            ));
        }

        if final_val > initial {
            qualified_difference_percentage(
                initial,
                self.min_value,
                final_val,
                initial,
                "undershoot",
            )
        } else {
            qualified_difference_percentage(
                self.max_value,
                initial,
                initial,
                final_val,
                "undershoot",
            )
        }
    }

    //=========================================================================
    // Threshold Crossing
    //=========================================================================

    /// Find all threshold crossings
    ///
    /// # Arguments
    /// * `threshold` - Voltage level to detect crossings
    /// * `direction` - Filter by edge direction
    ///
    /// # Returns
    /// Vector of crossing events, or an error if the threshold, crossing time,
    /// slew rate, or event storage cannot be represented.
    pub fn find_crossings(
        &self,
        threshold: Value,
        direction: EdgeDirection,
    ) -> Result<Vec<CrossingEvent>, MeasurementError> {
        validate_crossing_threshold(threshold)?;
        let mut crossings = Vec::new();

        for i in 0..self.values.len().saturating_sub(1) {
            if let Some(event) = self.crossing_event(i, threshold, direction)? {
                crossings.try_reserve(1).map_err(|error| {
                    MeasurementError::CalculationError(format!(
                        "failed to allocate crossing event {} at segment {i}: {error}",
                        crossings.len()
                    ))
                })?;
                crossings.push(event);
            }
        }

        Ok(crossings)
    }

    /// Find first crossing of threshold
    pub fn first_crossing(
        &self,
        threshold: Value,
        direction: EdgeDirection,
    ) -> Result<Option<CrossingEvent>, MeasurementError> {
        validate_crossing_threshold(threshold)?;
        for segment in 0..self.values.len().saturating_sub(1) {
            if let Some(event) = self.crossing_event(segment, threshold, direction)? {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    /// Find the time of a crossing by zero-based event index.
    ///
    /// This is an idiomatic Rust index. SPICE deck crossing ordinals are a
    /// separate one-based user-facing convention.
    pub fn cross_time(
        &self,
        threshold: Value,
        direction: EdgeDirection,
        event_index: usize,
    ) -> Result<Option<Value>, MeasurementError> {
        validate_crossing_threshold(threshold)?;
        let mut matched_events = 0usize;
        for segment in 0..self.values.len().saturating_sub(1) {
            if let Some(event) = self.crossing_event(segment, threshold, direction)? {
                if matched_events == event_index {
                    return Ok(Some(event.time));
                }
                matched_events = matched_events.checked_add(1).ok_or_else(|| {
                    MeasurementError::CalculationError(
                        "crossing event index exceeds this platform".to_string(),
                    )
                })?;
            }
        }
        Ok(None)
    }

    fn crossing_event(
        &self,
        segment: usize,
        threshold: Value,
        requested_direction: EdgeDirection,
    ) -> Result<Option<CrossingEvent>, MeasurementError> {
        let v0 = self.values[segment];
        let v1 = self.values[segment + 1];
        let t0 = self.time[segment];
        let t1 = self.time[segment + 1];
        if !v0.is_finite() || !v1.is_finite() || !t0.is_finite() || !t1.is_finite() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "crossing segment {segment} contains a non-finite sample"
            )));
        }
        if t1 <= t0 {
            return Err(MeasurementError::InvalidWaveform(format!(
                "crossing segment {segment} has non-increasing time ({t0} then {t1})"
            )));
        }

        let arrival_direction = if v0 < threshold && v1 >= threshold {
            Some(EdgeDirection::Rising)
        } else if v0 > threshold && v1 <= threshold {
            Some(EdgeDirection::Falling)
        } else {
            None
        };
        let Some(arrival_direction) = arrival_direction else {
            return Ok(None);
        };
        if requested_direction != EdgeDirection::Either && requested_direction != arrival_direction
        {
            return Ok(None);
        }

        let (fraction_numerator, voltage_span, signed_slew) = match arrival_direction {
            EdgeDirection::Rising => (
                scaled_positive_difference(threshold, v0, segment, "crossing numerator")?,
                scaled_positive_difference(v1, v0, segment, "voltage span")?,
                1.0,
            ),
            EdgeDirection::Falling => (
                scaled_positive_difference(v0, threshold, segment, "crossing numerator")?,
                scaled_positive_difference(v0, v1, segment, "voltage span")?,
                -1.0,
            ),
            EdgeDirection::Either => {
                return Err(MeasurementError::CalculationError(
                    "crossing event has no concrete arrival direction".to_string(),
                ));
            }
        };
        let fraction = scaled_positive_ratio(
            fraction_numerator,
            voltage_span,
            segment,
            "crossing fraction",
        )?;
        if fraction > 1.0 {
            return Err(MeasurementError::CalculationError(format!(
                "crossing fraction for segment {segment} is outside (0, 1] ({fraction})"
            )));
        }
        if fraction == 0.0 && v0 != threshold {
            return Err(MeasurementError::CalculationError(format!(
                "crossing fraction for segment {segment} falsely resolves to the prior sample"
            )));
        }
        if fraction == 1.0 && v1 != threshold {
            return Err(MeasurementError::CalculationError(format!(
                "crossing fraction for segment {segment} falsely resolves to the arrival sample"
            )));
        }
        let time = interpolate_crossing_time(t0, t1, fraction, segment)?;
        let time_span = scaled_positive_difference(t1, t0, segment, "time span")?;
        let slew_magnitude =
            scaled_positive_ratio(voltage_span, time_span, segment, "crossing slew rate")?;
        let slew_rate = signed_slew * slew_magnitude;
        if !slew_rate.is_finite() || slew_rate == 0.0 {
            return Err(MeasurementError::CalculationError(format!(
                "signed crossing slew rate for segment {segment} is not representable ({slew_rate})"
            )));
        }

        Ok(Some(CrossingEvent {
            time,
            index: segment,
            direction: arrival_direction,
            slew_rate,
            value: threshold,
        }))
    }

    //=========================================================================
    // Timing Measurements
    //=========================================================================

    /// Calculate rise time between two threshold levels
    ///
    /// # Arguments
    /// * `low_pct` - Low threshold as fraction (e.g., 0.1 for 10%)
    /// * `high_pct` - High threshold as fraction (e.g., 0.9 for 90%)
    ///
    /// # Returns
    /// Rise time in seconds
    pub fn rise_time(&self, low_pct: Value, high_pct: Value) -> Result<Value, MeasurementError> {
        let (low_level, high_level) =
            qualified_timing_levels(self.min_value, self.max_value, low_pct, high_pct)?;
        let mut low_arrival = None;

        for segment in 0..self.values.len().saturating_sub(1) {
            if low_arrival.is_none() {
                low_arrival = self.crossing_event(segment, low_level, EdgeDirection::Rising)?;
            }
            if let Some(low) = low_arrival.as_ref() {
                if let Some(high) =
                    self.crossing_event(segment, high_level, EdgeDirection::Rising)?
                {
                    return representable_positive_difference(
                        high.time,
                        low.time,
                        segment,
                        "rise time",
                    );
                }
                if self
                    .crossing_event(segment, low_level, EdgeDirection::Falling)?
                    .is_some()
                    || self.values[segment + 1] < low_level
                {
                    low_arrival = None;
                }
            }
        }

        Err(MeasurementError::ThresholdNotCrossed(
            "no complete low-to-high rising excursion was found".to_string(),
        ))
    }

    /// Calculate fall time between two threshold levels within one excursion.
    pub fn fall_time(&self, high_pct: Value, low_pct: Value) -> Result<Value, MeasurementError> {
        let (low_level, high_level) =
            qualified_timing_levels(self.min_value, self.max_value, low_pct, high_pct)?;
        let mut high_arrival = None;

        for segment in 0..self.values.len().saturating_sub(1) {
            if high_arrival.is_none() {
                high_arrival = self.crossing_event(segment, high_level, EdgeDirection::Falling)?;
            }
            if let Some(high) = high_arrival.as_ref() {
                if let Some(low) =
                    self.crossing_event(segment, low_level, EdgeDirection::Falling)?
                {
                    return representable_positive_difference(
                        low.time,
                        high.time,
                        segment,
                        "fall time",
                    );
                }
                if self
                    .crossing_event(segment, high_level, EdgeDirection::Rising)?
                    .is_some()
                    || self.values[segment + 1] > high_level
                {
                    high_arrival = None;
                }
            }
        }

        Err(MeasurementError::ThresholdNotCrossed(
            "no complete high-to-low falling excursion was found".to_string(),
        ))
    }

    /// Calculate delay between this waveform and a reference
    ///
    /// Measures time from reference crossing to this waveform crossing
    pub fn delay(
        &self,
        reference: &Waveform,
        ref_threshold: Value,
        self_threshold: Value,
        direction: EdgeDirection,
    ) -> Result<Value, MeasurementError> {
        let ref_cross = reference
            .first_crossing(ref_threshold, direction)?
            .ok_or_else(|| MeasurementError::ThresholdNotCrossed("Reference signal".to_string()))?;

        let self_cross = self
            .first_crossing(self_threshold, direction)?
            .ok_or_else(|| MeasurementError::ThresholdNotCrossed("Target signal".to_string()))?;

        representable_signed_difference(
            self_cross.time,
            ref_cross.time,
            self_cross.index,
            "propagation delay",
        )
    }

    /// Calculate period from the first two consecutive authored rising arrivals.
    pub fn period(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let (span, segment) = self.first_rising_period_span(threshold)?;
        materialize_scaled_positive(span, segment, "period")
    }

    fn first_rising_period_span(
        &self,
        threshold: Value,
    ) -> Result<(ScaledPositiveDifference, usize), MeasurementError> {
        validate_crossing_threshold(threshold)?;
        let mut first_rising: Option<CrossingEvent> = None;
        for segment in 0..self.values.len().saturating_sub(1) {
            let Some(rising) = self.crossing_event(segment, threshold, EdgeDirection::Rising)?
            else {
                continue;
            };
            if let Some(first) = first_rising.as_ref() {
                let span =
                    scaled_positive_difference(rising.time, first.time, rising.index, "period")?;
                return Ok((span, rising.index));
            }
            first_rising = Some(rising);
        }

        Err(MeasurementError::ThresholdNotCrossed(
            "Need at least 2 rising edges".to_string(),
        ))
    }

    /// Calculate frequency from the scaled separation of rising arrivals.
    ///
    /// The reciprocal may be representable even when the period itself is not.
    pub fn frequency(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let (period, segment) = self.first_rising_period_span(threshold)?;
        let unity = scaled_positive_value(1.0, segment, "frequency numerator")?;
        scaled_positive_ratio(unity, period, segment, "frequency")
    }

    /// Calculate the first complete authored rising-to-falling pulse width.
    ///
    /// A record that begins above the threshold does not imply a rising event,
    /// and no wrap across the record boundary is inferred.
    pub fn pulse_width(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let (rising, falling) = self.first_complete_pulse(threshold)?.ok_or_else(|| {
            MeasurementError::ThresholdNotCrossed(
                "no complete authored rising-to-falling pulse was found".to_string(),
            )
        })?;
        representable_positive_difference(falling.time, rising.time, falling.index, "pulse width")
    }

    /// Calculate duty cycle from one coherent rising-falling-rising cycle.
    pub fn duty_cycle(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let (rising, falling, next_rising) =
            self.first_complete_pulse_cycle(threshold)?.ok_or_else(|| {
                MeasurementError::ThresholdNotCrossed(
                    "no complete authored rising-to-falling-to-rising cycle was found".to_string(),
                )
            })?;
        let width = scaled_positive_difference(
            falling.time,
            rising.time,
            falling.index,
            "duty-cycle pulse width",
        )?;
        let period = scaled_positive_difference(
            next_rising.time,
            rising.time,
            next_rising.index,
            "duty-cycle period",
        )?;
        let duty_cycle = scaled_positive_ratio_with_factor(
            width,
            period,
            100.0,
            next_rising.index,
            "duty cycle",
        )?;
        if !duty_cycle.is_finite() || duty_cycle <= 0.0 || duty_cycle >= 100.0 {
            return Err(MeasurementError::CalculationError(format!(
                "duty cycle is not representable strictly inside (0, 100): {duty_cycle}"
            )));
        }

        Ok(duty_cycle)
    }

    /// Calculate the maximum absolute slew over every positive sample interval.
    pub fn slew_rate(&self) -> Result<Value, MeasurementError> {
        if self.time.len() < 2 {
            return Err(MeasurementError::InsufficientData(
                "Need at least 2 points".to_string(),
            ));
        }

        let mut max_slew = 0.0_f64;

        for segment in 0..self.time.len() - 1 {
            let time_span = scaled_positive_difference(
                self.time[segment + 1],
                self.time[segment],
                segment,
                "slew time span",
            )?;
            let left = self.values[segment];
            let right = self.values[segment + 1];
            if left != right {
                let (high, low) = if left > right {
                    (left, right)
                } else {
                    (right, left)
                };
                let value_span = scaled_positive_difference(high, low, segment, "slew value span")?;
                let slew =
                    scaled_positive_ratio(value_span, time_span, segment, "absolute slew rate")?;
                max_slew = max_slew.max(slew);
            }
        }

        Ok(max_slew)
    }

    fn first_complete_pulse(
        &self,
        threshold: Value,
    ) -> Result<Option<(CrossingEvent, CrossingEvent)>, MeasurementError> {
        validate_crossing_threshold(threshold)?;
        let mut rising_arrival = None;
        for segment in 0..self.values.len().saturating_sub(1) {
            if rising_arrival.is_none() {
                rising_arrival = self.crossing_event(segment, threshold, EdgeDirection::Rising)?;
            } else if let Some(falling) =
                self.crossing_event(segment, threshold, EdgeDirection::Falling)?
            {
                return Ok(rising_arrival.map(|rising| (rising, falling)));
            }

            if rising_arrival.is_some() && self.values[segment + 1] < threshold {
                rising_arrival = None;
            }
        }
        Ok(None)
    }

    fn first_complete_pulse_cycle(
        &self,
        threshold: Value,
    ) -> Result<Option<(CrossingEvent, CrossingEvent, CrossingEvent)>, MeasurementError> {
        validate_crossing_threshold(threshold)?;
        let mut rising_arrival = None;
        let mut falling_arrival = None;

        for segment in 0..self.values.len().saturating_sub(1) {
            if rising_arrival.is_none() {
                rising_arrival = self.crossing_event(segment, threshold, EdgeDirection::Rising)?;
                continue;
            }

            if falling_arrival.is_none() {
                if let Some(falling) =
                    self.crossing_event(segment, threshold, EdgeDirection::Falling)?
                {
                    falling_arrival = Some(falling);
                } else if self.values[segment + 1] < threshold {
                    rising_arrival = None;
                }
                continue;
            }

            if let Some(next_rising) =
                self.crossing_event(segment, threshold, EdgeDirection::Rising)?
            {
                let rising = rising_arrival.take().ok_or_else(|| {
                    MeasurementError::CalculationError(
                        "duty-cycle state lost its initial rising event".to_string(),
                    )
                })?;
                let falling = falling_arrival.take().ok_or_else(|| {
                    MeasurementError::CalculationError(
                        "duty-cycle state lost its falling event".to_string(),
                    )
                })?;
                return Ok(Some((rising, falling, next_rising)));
            }
            if self.values[segment + 1] > threshold {
                rising_arrival = None;
                falling_arrival = None;
            }
        }
        Ok(None)
    }

    //=========================================================================
    // Spectral Analysis
    //=========================================================================

    /// Compute the qualified one-sided FFT peak-amplitude spectrum.
    ///
    /// Returns `(frequencies, magnitudes_db)`. DC and, for even-length
    /// records, Nyquist are not doubled; every other positive-frequency bin
    /// is doubled. Magnitudes are normalized to the authored record length.
    /// Decibels are relative to a peak amplitude of one signal unit; an
    /// exactly zero magnitude is represented by negative infinity.
    ///
    /// The time axis must contain at least [`MIN_FFT_SAMPLES`] samples on a
    /// uniform grid whose interval and timestamp resolution can be qualified.
    /// The authored record is analyzed with a rectangular window, so a tone
    /// that is not centered on a returned bin exhibits spectral leakage.
    pub fn fft(&self) -> Result<(Vec<Value>, Vec<Value>), MeasurementError> {
        let spectrum = self.one_sided_linear_spectrum(false)?;
        let mut magnitudes_db = Vec::new();
        magnitudes_db
            .try_reserve_exact(spectrum.spectral_weights.len())
            .map_err(|error| {
                MeasurementError::FftError(format!("failed to allocate FFT dB spectrum: {error}"))
            })?;
        for spectral_weight in spectrum.spectral_weights {
            magnitudes_db.push(scaled_amplitude_db(
                spectral_weight,
                spectrum.log10_normalization,
            )?);
        }
        Ok((spectrum.frequencies, magnitudes_db))
    }

    /// Calculate Total Harmonic Distortion (THD) for an explicit fundamental.
    ///
    /// `highest_harmonic` is inclusive and must be at least two. The retained
    /// waveform must cover a complete period and resolve every requested
    /// harmonic; unavailable harmonics are errors, never silently omitted.
    /// Coefficients are integrated over the trailing exact fundamental period.
    /// `Ok(None)` means the measured fundamental magnitude is exactly zero.
    pub fn thd(
        &self,
        fundamental_frequency: Value,
        highest_harmonic: usize,
    ) -> Result<Option<Value>, MeasurementError> {
        if highest_harmonic < 2 {
            return Err(MeasurementError::CalculationError(
                "THD highest harmonic must be at least 2".to_string(),
            ));
        }
        let analysis = FourierAnalysis::new(
            FourierConfig::new(fundamental_frequency).with_harmonics(highest_harmonic),
        );
        analysis
            .analyze(&self.time, &self.values)
            .map(|result| result.thd)
            .map_err(map_fourier_measurement_error)
    }

    /// Estimate the dominant non-DC frequency from the mean-removed FFT.
    ///
    /// This is a dominant-bin estimate, not proof that a weaker physical
    /// fundamental is absent. Exact constant or zero waveforms return
    /// `Ok(None)`. Equal-amplitude ties choose the lowest positive bin.
    pub fn dominant_frequency(&self) -> Result<Option<Value>, MeasurementError> {
        let spectrum = self.one_sided_linear_spectrum(true)?;
        let Some((&first, remaining)) = spectrum.spectral_weights.split_first() else {
            return Err(MeasurementError::FftError(
                "FFT produced no spectral bins".to_string(),
            ));
        };
        let _dc = first;
        let Some((&first_positive, remaining)) = remaining.split_first() else {
            return Err(MeasurementError::FftError(
                "FFT produced no positive-frequency bins".to_string(),
            ));
        };
        let mut dominant_index = 1usize;
        let mut dominant_amplitude = first_positive;
        for (offset, &amplitude) in remaining.iter().enumerate() {
            if amplitude > dominant_amplitude {
                dominant_amplitude = amplitude;
                dominant_index = offset + 2;
            }
        }
        if dominant_amplitude == 0.0 {
            Ok(None)
        } else {
            Ok(Some(spectrum.frequencies[dominant_index]))
        }
    }

    fn qualified_sample_interval(&self) -> Result<Value, MeasurementError> {
        let sample_count = self.time.len();
        validate_fft_sample_count(sample_count)?;
        let last_segment = sample_count - 2;
        let duration = scaled_positive_difference(
            self.time[sample_count - 1],
            self.time[0],
            last_segment,
            "spectral time span",
        )
        .map_err(|error| {
            MeasurementError::FftError(format!("spectral time-span qualification failed: {error}"))
        })?;
        let interval_count = sample_count - 1;
        let interval_count_value = interval_count as Value;
        if !interval_count_value.is_finite() || interval_count_value as usize != interval_count {
            return Err(MeasurementError::FftError(format!(
                "spectral interval count {interval_count} cannot be represented exactly"
            )));
        }
        let scaled_interval_count = scaled_positive_value(
            interval_count_value,
            last_segment,
            "spectral interval count",
        )
        .map_err(|error| {
            MeasurementError::FftError(format!(
                "spectral interval-count qualification failed: {error}"
            ))
        })?;
        let interval = scaled_positive_ratio(
            duration,
            scaled_interval_count,
            last_segment,
            "spectral sample interval",
        )
        .map_err(|error| {
            MeasurementError::FftError(format!(
                "spectral sample-interval qualification failed: {error}"
            ))
        })?;
        // Permit negligible authored roundoff while bounding the admitted
        // sample-rate error to one part per billion. Timestamp ULPs are
        // qualified separately: a large absolute time origin must not hide
        // material interval uncertainty behind a permissive tolerance.
        const GRID_RELATIVE_TOLERANCE: Value = 1.0e-9;
        let grid_budget = GRID_RELATIVE_TOLERANCE * interval;
        if !grid_budget.is_finite() || grid_budget <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "uniform-grid tolerance cannot be represented for sample interval {interval}"
            )));
        }
        let interval_ulp = value_ulp(interval);
        for (index, pair) in self.time.windows(2).enumerate() {
            let timestamp_resolution = value_ulp(pair[0]).max(value_ulp(pair[1]));
            if !timestamp_resolution.is_finite() || timestamp_resolution > grid_budget {
                return Err(MeasurementError::FftError(format!(
                    "timestamp resolution {timestamp_resolution} s at interval {index} cannot qualify nominal sample interval {interval} s"
                )));
            }
            let actual = pair[1] - pair[0];
            let arithmetic_slack = 4.0 * (value_ulp(actual) + interval_ulp);
            let tolerance = grid_budget + arithmetic_slack;
            if !actual.is_finite()
                || actual <= 0.0
                || !tolerance.is_finite()
                || (actual - interval).abs() > tolerance
            {
                return Err(MeasurementError::FftError(format!(
                    "spectral analysis requires uniform sampling: interval {index} is {actual}, expected {interval} within {tolerance}"
                )));
            }
        }
        let sample_rate = interval.recip();
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "spectral analysis sample rate is invalid ({sample_rate})"
            )));
        }
        Ok(interval)
    }

    fn one_sided_linear_spectrum(
        &self,
        remove_dc: bool,
    ) -> Result<LinearSpectrum, MeasurementError> {
        let interval = self.qualified_sample_interval()?;
        let sample_count = self.values.len();
        let bin_count = sample_count
            .checked_div(2)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                MeasurementError::FftError(
                    "one-sided FFT bin count exceeds this platform".to_string(),
                )
            })?;
        let sample_rate = interval.recip();
        let bin_spacing = sample_rate / sample_count as Value;
        if !bin_spacing.is_finite() || bin_spacing <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "FFT bin spacing is invalid ({bin_spacing})"
            )));
        }

        let mut frequencies = Vec::new();
        frequencies.try_reserve_exact(bin_count).map_err(|error| {
            MeasurementError::FftError(format!(
                "failed to allocate {bin_count} FFT frequencies: {error}"
            ))
        })?;
        let mut spectral_weights = Vec::new();
        spectral_weights
            .try_reserve_exact(bin_count)
            .map_err(|error| {
                MeasurementError::FftError(format!(
                    "failed to allocate {bin_count} FFT amplitudes: {error}"
                ))
            })?;
        for index in 0..bin_count {
            let frequency = index as Value * bin_spacing;
            if !frequency.is_finite() {
                return Err(MeasurementError::FftError(format!(
                    "FFT frequency at bin {index} is non-finite"
                )));
            }
            frequencies.push(frequency);
            spectral_weights.push(0.0);
        }

        let is_constant = self.values.iter().all(|value| *value == self.values[0]);
        if is_constant {
            let scale = self.values[0].abs();
            if !remove_dc && scale > 0.0 {
                spectral_weights[0] = 1.0;
            }
            return Ok(LinearSpectrum {
                frequencies,
                spectral_weights,
                log10_normalization: (scale > 0.0 && !remove_dc).then(|| scale.log10()),
            });
        }

        let source_scale = self
            .values
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        if !source_scale.is_finite() || source_scale <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "FFT input scale is invalid ({source_scale})"
            )));
        }
        let normalized_mean = if remove_dc {
            normalized_mean(&self.values, source_scale)?
        } else {
            0.0
        };
        let mut centered_scale = 0.0_f64;
        for (index, value) in self.values.iter().enumerate() {
            let normalized = qualified_fft_scaling(*value, source_scale, index, "source scaling")?;
            let centered = normalized - normalized_mean;
            if !centered.is_finite() {
                return Err(MeasurementError::FftError(
                    "mean removal produced a non-finite FFT sample".to_string(),
                ));
            }
            if normalized != normalized_mean && centered == 0.0 {
                return Err(MeasurementError::FftError(format!(
                    "mean removal erased a nonzero normalized difference at sample {index}"
                )));
            }
            centered_scale = centered_scale.max(centered.abs());
        }
        if centered_scale == 0.0 {
            return Ok(LinearSpectrum {
                frequencies,
                spectral_weights,
                log10_normalization: None,
            });
        }
        let log10_normalization =
            source_scale.log10() + centered_scale.log10() - (sample_count as Value).log10();
        if !log10_normalization.is_finite() {
            return Err(MeasurementError::FftError(format!(
                "FFT amplitude normalization cannot be represented in logarithmic form ({log10_normalization})"
            )));
        }

        let mut buffer = Vec::new();
        buffer.try_reserve_exact(sample_count).map_err(|error| {
            MeasurementError::FftError(format!(
                "failed to allocate {sample_count} FFT samples: {error}"
            ))
        })?;
        for (index, value) in self.values.iter().enumerate() {
            let normalized = qualified_fft_scaling(*value, source_scale, index, "source scaling")?;
            let centered = normalized - normalized_mean;
            let transform_sample =
                qualified_fft_scaling(centered, centered_scale, index, "centered scaling")?;
            buffer.push(Complex::new(transform_sample, 0.0));
        }
        let fft = FftPlanner::<Value>::new().plan_fft_forward(sample_count);
        let scratch_len = fft.get_inplace_scratch_len();
        let mut scratch = Vec::new();
        scratch.try_reserve_exact(scratch_len).map_err(|error| {
            MeasurementError::FftError(format!(
                "failed to allocate {scratch_len} FFT scratch samples: {error}"
            ))
        })?;
        scratch.resize(scratch_len, Complex::new(0.0, 0.0));
        fft.process_with_scratch(&mut buffer, &mut scratch);
        if let Some((index, value)) = buffer
            .iter()
            .enumerate()
            .find(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(MeasurementError::FftError(format!(
                "FFT produced non-finite coefficient {value} at bin {index}"
            )));
        }

        for index in 0..bin_count {
            let is_nyquist = sample_count.is_multiple_of(2) && index == sample_count / 2;
            let one_sided_scale = if index == 0 || is_nyquist { 1.0 } else { 2.0 };
            let spectral_weight = buffer[index].norm() * one_sided_scale;
            if !spectral_weight.is_finite() {
                return Err(MeasurementError::FftError(format!(
                    "FFT one-sided spectral weight at bin {index} is non-finite ({spectral_weight})"
                )));
            }
            spectral_weights[index] = spectral_weight;
        }

        Ok(LinearSpectrum {
            frequencies,
            spectral_weights,
            log10_normalization: Some(log10_normalization),
        })
    }

    //=========================================================================
    // Interpolation
    //=========================================================================

    /// Interpolate the waveform at a finite query time.
    ///
    /// Returns `Ok(None)` only when a finite query lies outside the authored
    /// time range. Exact authored sample times return the authored value.
    /// Interior interpolation is performed with a qualified affine operation;
    /// an unrepresentable time fraction or result returns an error rather than
    /// an endpoint or fabricated value.
    pub fn interpolate(&self, t: Value) -> Result<Option<Value>, MeasurementError> {
        if !t.is_finite() {
            return Err(MeasurementError::CalculationError(format!(
                "interpolation query time must be finite, got {t}"
            )));
        }
        if self.time.is_empty() || self.time.len() != self.values.len() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "interpolation requires a nonempty matched time/value grid, got {} time point(s) and {} value(s)",
                self.time.len(),
                self.values.len()
            )));
        }
        if t < self.min_time || t > self.max_time {
            return Ok(None);
        }

        let idx = self.time.partition_point(|&x| x < t);
        if idx < self.time.len() && self.time[idx] == t {
            return Ok(Some(self.values[idx]));
        }
        if idx == 0 || idx >= self.time.len() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "interpolation search could not bracket in-range time {t}"
            )));
        }

        let t0 = self.time[idx - 1];
        let t1 = self.time[idx];
        let v0 = self.values[idx - 1];
        let v1 = self.values[idx];
        let fraction_numerator =
            scaled_positive_difference(t, t0, idx - 1, "interpolation query offset")?;
        let time_span = scaled_positive_difference(t1, t0, idx - 1, "interpolation time span")?;
        let fraction = scaled_positive_ratio(
            fraction_numerator,
            time_span,
            idx - 1,
            "interpolation fraction",
        )?;
        if !fraction.is_finite() || fraction <= 0.0 || fraction >= 1.0 {
            return Err(MeasurementError::CalculationError(format!(
                "interior interpolation fraction at segment {} is not representable strictly inside (0, 1): {fraction}",
                idx - 1
            )));
        }
        qualified_affine_value(v0, v1, fraction, "waveform interpolation").map(Some)
    }

    /// Resample the waveform onto an inclusive uniform time grid.
    ///
    /// At least two source and destination points are required. Allocation,
    /// grid-representation, and interpolation failures are returned rather
    /// than silently cloning the source or substituting zero-valued samples.
    pub fn resample(&self, num_points: usize) -> Result<Self, MeasurementError> {
        if num_points < 2 {
            return Err(MeasurementError::InsufficientData(format!(
                "resampling requires at least 2 destination points, got {num_points}"
            )));
        }
        if num_points > MAX_QUALIFIED_RESAMPLE_POINTS {
            return Err(MeasurementError::CalculationError(format!(
                "requested {num_points} resampling points exceeds the qualified limit of {MAX_QUALIFIED_RESAMPLE_POINTS}"
            )));
        }
        if self.time.len() < 2 {
            return Err(MeasurementError::InsufficientData(format!(
                "resampling requires at least 2 source points, got {}",
                self.time.len()
            )));
        }
        validate_waveform_input(&self.time, &self.values)?;

        let mut new_time = Vec::new();
        new_time.try_reserve_exact(num_points).map_err(|error| {
            MeasurementError::CalculationError(format!(
                "failed to allocate {num_points} resampled time points: {error}"
            ))
        })?;
        let mut new_values = Vec::new();
        new_values.try_reserve_exact(num_points).map_err(|error| {
            MeasurementError::CalculationError(format!(
                "failed to allocate {num_points} resampled values: {error}"
            ))
        })?;

        let interval_count = num_points - 1;
        let interval_count_value = interval_count as Value;
        if !interval_count_value.is_finite() || interval_count_value as usize != interval_count {
            return Err(MeasurementError::CalculationError(format!(
                "resampling interval count {interval_count} cannot be represented exactly"
            )));
        }

        for i in 0..num_points {
            let t = if i == 0 {
                self.min_time
            } else if i == interval_count {
                self.max_time
            } else {
                let index_value = i as Value;
                if index_value as usize != i {
                    return Err(MeasurementError::CalculationError(format!(
                        "resampling grid index {i} cannot be represented exactly"
                    )));
                }
                let fraction = index_value / interval_count_value;
                if !fraction.is_finite() || fraction <= 0.0 || fraction >= 1.0 {
                    return Err(MeasurementError::CalculationError(format!(
                        "resampling grid fraction at index {i} is invalid ({fraction})"
                    )));
                }
                qualified_affine_value(
                    self.min_time,
                    self.max_time,
                    fraction,
                    "resampling time grid",
                )?
            };
            if let Some(previous) = new_time.last()
                && t <= *previous
            {
                return Err(MeasurementError::CalculationError(format!(
                    "resampling time point {i} is not representable after its predecessor ({t} <= {previous})"
                )));
            }
            let value = self.interpolate(t)?.ok_or_else(|| {
                MeasurementError::CalculationError(format!(
                    "resampling time point {i} unexpectedly lies outside the source grid ({t})"
                ))
            })?;
            new_time.push(t);
            new_values.push(value);
        }

        validate_waveform_input(&new_time, &new_values)?;
        Ok(Self::from_owned_validated_parts(new_time, new_values))
    }
}

fn validate_crossing_threshold(threshold: Value) -> Result<(), MeasurementError> {
    if threshold.is_finite() {
        Ok(())
    } else {
        Err(MeasurementError::InvalidThreshold(format!(
            "crossing threshold must be finite, got {threshold}"
        )))
    }
}

fn qualified_timing_levels(
    minimum: Value,
    maximum: Value,
    low_fraction: Value,
    high_fraction: Value,
) -> Result<(Value, Value), MeasurementError> {
    if !low_fraction.is_finite() || !high_fraction.is_finite() {
        return Err(MeasurementError::InvalidThreshold(format!(
            "timing threshold fractions must be finite, got {low_fraction} and {high_fraction}"
        )));
    }
    if !(0.0..=1.0).contains(&low_fraction)
        || !(0.0..=1.0).contains(&high_fraction)
        || low_fraction >= high_fraction
    {
        return Err(MeasurementError::InvalidThreshold(format!(
            "timing threshold fractions must satisfy 0 <= low < high <= 1, got {low_fraction} and {high_fraction}"
        )));
    }
    if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
        return Err(MeasurementError::CalculationError(format!(
            "timing threshold levels require a finite nonzero waveform range, got [{minimum}, {maximum}]"
        )));
    }

    let low = qualified_convex_level(minimum, maximum, low_fraction, "low timing threshold")?;
    let high = qualified_convex_level(minimum, maximum, high_fraction, "high timing threshold")?;
    if low >= high {
        return Err(MeasurementError::CalculationError(format!(
            "qualified timing thresholds are not distinct and ordered ({low}, {high})"
        )));
    }
    Ok((low, high))
}

fn qualified_convex_level(
    minimum: Value,
    maximum: Value,
    fraction: Value,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    if fraction == 0.0 {
        return Ok(minimum);
    }
    if fraction == 1.0 {
        return Ok(maximum);
    }
    qualified_affine_value(minimum, maximum, fraction, quantity)
}

fn qualified_affine_value(
    start: Value,
    end: Value,
    fraction: Value,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    if !start.is_finite() || !end.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} requires finite endpoints, got {start} and {end}"
        )));
    }
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} fraction must be finite and inside [0, 1], got {fraction}"
        )));
    }
    if fraction == 0.0 || start == end {
        return Ok(start);
    }
    if fraction == 1.0 {
        return Ok(end);
    }
    let calculation_error = || {
        MeasurementError::CalculationError(format!(
            "{quantity} cannot be represented without losing interpolation evidence"
        ))
    };
    let (complement, complement_residual) =
        error_free_sum(1.0, -fraction, 0).map_err(|_| calculation_error())?;
    if complement + complement_residual <= 0.0 {
        return Err(calculation_error());
    }

    let mut level = BoundedExactFloatSum::<6>::new();
    for (weight, endpoint) in [
        (complement, start),
        (complement_residual, start),
        (fraction, end),
    ] {
        let (product, residual) =
            error_free_product(weight, endpoint, 0, quantity).map_err(|_| calculation_error())?;
        level.add(product).map_err(|_| calculation_error())?;
        level.add(residual).map_err(|_| calculation_error())?;
    }
    let level = level.finish().map_err(|_| calculation_error())?;
    let lower = start.min(end);
    let upper = start.max(end);
    if !level.is_finite() || level <= lower || level >= upper {
        return Err(MeasurementError::CalculationError(format!(
            "interior {quantity} is not representable strictly inside ({lower}, {upper}): {level}"
        )));
    }
    Ok(level)
}

struct BoundedExactFloatSum<const CAPACITY: usize> {
    partials: [Value; CAPACITY],
    len: usize,
}

impl<const CAPACITY: usize> BoundedExactFloatSum<CAPACITY> {
    fn new() -> Self {
        Self {
            partials: [0.0; CAPACITY],
            len: 0,
        }
    }

    fn add(&mut self, mut value: Value) -> Result<(), MeasurementError> {
        if !value.is_finite() {
            return Err(MeasurementError::CalculationError(
                "bounded floating-point expansion received a non-finite component".to_string(),
            ));
        }
        if value == 0.0 {
            return Ok(());
        }

        let existing_count = self.len;
        let mut retained_count = 0usize;
        for index in 0..existing_count {
            let mut partial = self.partials[index];
            if value.abs() < partial.abs() {
                std::mem::swap(&mut value, &mut partial);
            }
            let high = value + partial;
            if !high.is_finite() {
                return Err(MeasurementError::CalculationError(
                    "bounded floating-point expansion accumulation became non-finite".to_string(),
                ));
            }
            let low = partial - (high - value);
            if low != 0.0 {
                self.partials[retained_count] = low;
                retained_count += 1;
            }
            value = high;
        }
        if value != 0.0 {
            if retained_count == CAPACITY {
                return Err(MeasurementError::CalculationError(format!(
                    "bounded floating-point expansion exceeded its {CAPACITY}-component capacity"
                )));
            }
            self.partials[retained_count] = value;
            retained_count += 1;
        }
        self.len = retained_count;
        Ok(())
    }

    fn finish(self) -> Result<Value, MeasurementError> {
        if self.len == 0 {
            return Ok(0.0);
        }
        let mut result = 0.0;
        for partial in self.partials.into_iter().take(self.len) {
            let next = result + partial;
            if !next.is_finite() {
                return Err(MeasurementError::CalculationError(
                    "bounded floating-point expansion became non-finite".to_string(),
                ));
            }
            result = next;
        }
        if result == 0.0 {
            return Err(MeasurementError::CalculationError(
                "bounded floating-point expansion residual cannot be represented".to_string(),
            ));
        }
        Ok(result)
    }
}

#[derive(Clone, Copy)]
struct ScaledPositiveDifference {
    mantissa: Value,
    exponent: i32,
}

fn scaled_positive_value(
    value: Value,
    segment: usize,
    quantity: &str,
) -> Result<ScaledPositiveDifference, MeasurementError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} for segment {segment} must be positive and finite, got {value}"
        )));
    }
    let exponent = libm::ilogb(value);
    let mantissa = libm::scalbn(value, -exponent);
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "scaled {quantity} for segment {segment} is invalid ({mantissa})"
        )));
    }
    Ok(ScaledPositiveDifference { mantissa, exponent })
}

fn scaled_positive_difference(
    high: Value,
    low: Value,
    segment: usize,
    quantity: &str,
) -> Result<ScaledPositiveDifference, MeasurementError> {
    if !high.is_finite() || !low.is_finite() || high <= low {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} for segment {segment} requires finite ordered endpoints, got {high} and {low}"
        )));
    }
    let scale = high.abs().max(low.abs());
    if !scale.is_finite() || scale <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} scale for segment {segment} is invalid ({scale})"
        )));
    }
    let exponent = libm::ilogb(scale);
    let high_scaled = libm::scalbn(high, -exponent);
    let low_scaled = libm::scalbn(low, -exponent);
    let mantissa = high_scaled - low_scaled;
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "scaled {quantity} for segment {segment} is not representable ({mantissa})"
        )));
    }
    Ok(ScaledPositiveDifference { mantissa, exponent })
}

fn scaled_positive_ratio(
    numerator: ScaledPositiveDifference,
    denominator: ScaledPositiveDifference,
    segment: usize,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    scaled_positive_ratio_with_factor(numerator, denominator, 1.0, segment, quantity)
}

fn scaled_positive_ratio_with_factor(
    numerator: ScaledPositiveDifference,
    denominator: ScaledPositiveDifference,
    factor: Value,
    segment: usize,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    if !factor.is_finite() || factor <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} scale factor for segment {segment} is invalid ({factor})"
        )));
    }
    let mantissa_ratio = numerator.mantissa / denominator.mantissa;
    let scaled_mantissa = mantissa_ratio * factor;
    if !scaled_mantissa.is_finite() || scaled_mantissa <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "scaled {quantity} mantissa for segment {segment} is not representable ({scaled_mantissa})"
        )));
    }
    let exponent_delta = numerator
        .exponent
        .checked_sub(denominator.exponent)
        .ok_or_else(|| {
            MeasurementError::CalculationError(format!(
                "{quantity} exponent for segment {segment} exceeds this platform"
            ))
        })?;
    let ratio = libm::scalbn(scaled_mantissa, exponent_delta);
    if !ratio.is_finite() || ratio <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} for segment {segment} is not representable ({ratio})"
        )));
    }
    Ok(ratio)
}

fn qualified_difference_percentage(
    numerator_high: Value,
    numerator_low: Value,
    denominator_high: Value,
    denominator_low: Value,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    for (name, value) in [
        ("numerator high", numerator_high),
        ("numerator low", numerator_low),
        ("denominator high", denominator_high),
        ("denominator low", denominator_low),
    ] {
        if !value.is_finite() {
            return Err(MeasurementError::CalculationError(format!(
                "{quantity} {name} endpoint must be finite, got {value}"
            )));
        }
    }
    if numerator_high < numerator_low {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} numerator endpoints are not ordered ({numerator_high} < {numerator_low})"
        )));
    }
    if numerator_high == numerator_low {
        return Ok(0.0);
    }
    let numerator = scaled_positive_difference(numerator_high, numerator_low, 0, quantity)?;
    let denominator = scaled_positive_difference(denominator_high, denominator_low, 0, quantity)?;
    let percentage = scaled_positive_ratio_with_factor(numerator, denominator, 100.0, 0, quantity)?;
    if !percentage.is_finite() || percentage <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} percentage is not representable as a positive finite value ({percentage})"
        )));
    }
    Ok(percentage)
}

fn representable_positive_difference(
    later: Value,
    earlier: Value,
    segment: usize,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    let difference = scaled_positive_difference(later, earlier, segment, quantity)?;
    materialize_scaled_positive(difference, segment, quantity)
}

fn materialize_scaled_positive(
    value: ScaledPositiveDifference,
    segment: usize,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    let value = libm::scalbn(value.mantissa, value.exponent);
    if !value.is_finite() || value <= 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} for segment {segment} is not representable as a positive finite value ({value})"
        )));
    }
    Ok(value)
}

fn representable_signed_difference(
    left: Value,
    right: Value,
    segment: usize,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    if !left.is_finite() || !right.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} requires finite event times, got {left} and {right}"
        )));
    }
    if left == right {
        return Ok(0.0);
    }
    let (high, low, sign) = if left > right {
        (left, right, 1.0)
    } else {
        (right, left, -1.0)
    };
    let magnitude = representable_positive_difference(high, low, segment, quantity)?;
    let result = sign * magnitude;
    if !result.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "signed {quantity} is not representable ({result})"
        )));
    }
    Ok(result)
}

fn interpolate_crossing_time(
    start: Value,
    end: Value,
    fraction: Value,
    segment: usize,
) -> Result<Value, MeasurementError> {
    if !fraction.is_finite() || fraction <= 0.0 || fraction > 1.0 {
        return Err(MeasurementError::CalculationError(format!(
            "crossing fraction for segment {segment} is invalid ({fraction})"
        )));
    }
    if fraction == 1.0 {
        return Ok(end);
    }
    let (complement, complement_residual) = error_free_sum(1.0, -fraction, segment)?;
    if !complement.is_finite()
        || !complement_residual.is_finite()
        || complement + complement_residual <= 0.0
    {
        return Err(MeasurementError::CalculationError(format!(
            "crossing interpolation complement for segment {segment} is invalid ({complement} + {complement_residual})"
        )));
    }

    let mut interpolation = ExactFloatSum::default();
    for (weight, time) in [
        (complement, start),
        (complement_residual, start),
        (fraction, end),
    ] {
        let (product, residual) =
            error_free_product(weight, time, segment, "crossing time interpolation")?;
        interpolation.add(product)?;
        interpolation.add(residual)?;
    }
    let time = interpolation.finish()?;
    if !time.is_finite() || time <= start || time >= end {
        return Err(MeasurementError::CalculationError(format!(
            "interpolated crossing time for interior segment fraction {fraction} at segment {segment} is not strictly inside ({start}, {end}): {time}"
        )));
    }
    Ok(time)
}

#[derive(Default)]
struct ExactFloatSum {
    partials: Vec<Value>,
}

impl ExactFloatSum {
    fn add(&mut self, mut value: Value) -> Result<(), MeasurementError> {
        if !value.is_finite() {
            return Err(MeasurementError::CalculationError(
                "floating-point expansion received a non-finite component".to_string(),
            ));
        }
        if value == 0.0 {
            return Ok(());
        }

        let existing_count = self.partials.len();
        let mut retained_count = 0usize;
        for index in 0..existing_count {
            let mut partial = self.partials[index];
            if value.abs() < partial.abs() {
                std::mem::swap(&mut value, &mut partial);
            }
            let high = value + partial;
            if !high.is_finite() {
                return Err(MeasurementError::CalculationError(
                    "floating-point expansion accumulation became non-finite".to_string(),
                ));
            }
            let low = partial - (high - value);
            if low != 0.0 {
                self.partials[retained_count] = low;
                retained_count += 1;
            }
            value = high;
        }
        self.partials.truncate(retained_count);
        if value != 0.0 {
            self.partials.try_reserve(1).map_err(|error| {
                MeasurementError::CalculationError(format!(
                    "failed to retain a floating-point residual: {error}"
                ))
            })?;
            self.partials.push(value);
        }
        Ok(())
    }

    fn finish(self) -> Result<Value, MeasurementError> {
        if self.partials.is_empty() {
            return Ok(0.0);
        }
        let mut result = 0.0;
        for partial in self.partials {
            let next = result + partial;
            if !next.is_finite() {
                return Err(MeasurementError::CalculationError(
                    "floating-point expansion became non-finite".to_string(),
                ));
            }
            result = next;
        }
        if result == 0.0 {
            return Err(MeasurementError::CalculationError(
                "floating-point expansion residual cannot be represented".to_string(),
            ));
        }
        Ok(result)
    }
}

fn accumulate_average_segment(
    accumulator: &mut ExactFloatSum,
    left: Value,
    right: Value,
    weight: Value,
    segment: usize,
) -> Result<(), MeasurementError> {
    let (sample_sum, sample_residual) = error_free_sum(left, right, segment)?;
    for endpoint_component in [sample_sum, sample_residual] {
        accumulate_half_weighted_component(
            accumulator,
            endpoint_component,
            weight,
            segment,
            "endpoint average",
        )?;
    }
    Ok(())
}

fn accumulate_rms_segment(
    accumulator: &mut ExactFloatSum,
    left: Value,
    right: Value,
    weight: Value,
    segment: usize,
) -> Result<(), MeasurementError> {
    for sample in [left, right] {
        let (square, square_residual) =
            error_free_product(sample, sample, segment, "normalized RMS square")?;
        for square_component in [square, square_residual] {
            accumulate_half_weighted_component(
                accumulator,
                square_component,
                weight,
                segment,
                "RMS trapezoidal scaling",
            )?;
        }
    }
    Ok(())
}

fn accumulate_half_weighted_component(
    accumulator: &mut ExactFloatSum,
    component: Value,
    weight: Value,
    segment: usize,
    quantity: &str,
) -> Result<(), MeasurementError> {
    let (half_component, half_residual) = error_free_product(0.5, component, segment, quantity)?;
    for moment_component in [half_component, half_residual] {
        let (weighted_component, product_residual) =
            error_free_product(weight, moment_component, segment, "interval weighting")?;
        accumulator.add(weighted_component)?;
        accumulator.add(product_residual)?;
    }
    Ok(())
}

fn error_free_sum(
    left: Value,
    right: Value,
    segment: usize,
) -> Result<(Value, Value), MeasurementError> {
    let sum = left + right;
    if !sum.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "normalized endpoint sum for segment {segment} is non-finite"
        )));
    }
    let right_virtual = sum - left;
    let residual = (left - (sum - right_virtual)) + (right - right_virtual);
    if !residual.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "normalized endpoint residual for segment {segment} is non-finite"
        )));
    }
    Ok((sum, residual))
}

fn error_free_product(
    left: Value,
    right: Value,
    segment: usize,
    quantity: &str,
) -> Result<(Value, Value), MeasurementError> {
    let product = left * right;
    if !product.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} product for segment {segment} is non-finite"
        )));
    }
    if left != 0.0 && right != 0.0 && product == 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} product for segment {segment} underflowed"
        )));
    }
    let residual = left.mul_add(right, -product);
    if !residual.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} residual for segment {segment} is non-finite"
        )));
    }
    if !product_is_exact(left, right, product) && (residual == 0.0 || residual.is_subnormal()) {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} product for segment {segment} has an uncertified underflow-scale residual"
        )));
    }
    Ok((product, residual))
}

fn product_is_exact(left: Value, right: Value, product: Value) -> bool {
    if left == 0.0 || right == 0.0 {
        return product == 0.0;
    }
    let (left_significand, left_exponent) = finite_binary_components(left);
    let (right_significand, right_exponent) = finite_binary_components(right);
    let exact_significand = u128::from(left_significand) * u128::from(right_significand);
    let exact_exponent = left_exponent + right_exponent;
    let (rounded_significand, rounded_exponent) = finite_binary_components(product);
    canonical_dyadic(exact_significand, exact_exponent)
        == canonical_dyadic(u128::from(rounded_significand), rounded_exponent)
}

fn canonical_dyadic(significand: u128, exponent: i32) -> (u128, i32) {
    debug_assert!(significand != 0);
    let trailing_zeros = significand.trailing_zeros();
    (
        significand >> trailing_zeros,
        exponent + trailing_zeros as i32,
    )
}

fn finite_binary_components(value: Value) -> (u64, i32) {
    const FRACTION_BITS: u64 = (1_u64 << 52) - 1;
    let bits = value.to_bits() & !(1_u64 << 63);
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & FRACTION_BITS;
    if exponent_bits == 0 {
        (fraction, -1074)
    } else {
        ((1_u64 << 52) | fraction, exponent_bits - 1023 - 52)
    }
}

fn compensated_add(
    sum: &mut Value,
    compensation: &mut Value,
    term: Value,
    quantity: &str,
) -> Result<(), MeasurementError> {
    let corrected = term - *compensation;
    let next = *sum + corrected;
    let next_compensation = (next - *sum) - corrected;
    if !corrected.is_finite() || !next.is_finite() || !next_compensation.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "compensated accumulation of {quantity} became non-finite"
        )));
    }
    *sum = next;
    *compensation = next_compensation;
    Ok(())
}

fn compensated_total(
    sum: Value,
    compensation: Value,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    let (high, low) = error_free_sum(sum, -compensation, usize::MAX)?;
    let total = high + low;
    if !total.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "compensated total for {quantity} is non-finite"
        )));
    }
    if total == 0.0 && (high != 0.0 || low != 0.0) {
        return Err(MeasurementError::CalculationError(format!(
            "compensated total for {quantity} underflowed"
        )));
    }
    Ok(total)
}

fn scaled_moment_result(
    scale: Value,
    normalized_result: Value,
    quantity: &str,
) -> Result<Value, MeasurementError> {
    if !scale.is_finite() || scale < 0.0 || !normalized_result.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} normalization is invalid (scale {scale}, normalized result {normalized_result})"
        )));
    }
    let result = scale * normalized_result;
    if !result.is_finite() {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} result is non-finite ({result})"
        )));
    }
    if scale > 0.0 && normalized_result != 0.0 && result == 0.0 {
        return Err(MeasurementError::CalculationError(format!(
            "{quantity} result underflowed the representable range"
        )));
    }
    if result == 0.0 { Ok(0.0) } else { Ok(result) }
}

struct LinearSpectrum {
    frequencies: Vec<Value>,
    /// Unnormalized one-sided coefficient norms. Keeping the common `1/N`
    /// factor logarithmic prevents a nonzero subnormal norm from underflowing
    /// to a fabricated exact zero.
    spectral_weights: Vec<Value>,
    log10_normalization: Option<Value>,
}

fn validate_fft_sample_count(sample_count: usize) -> Result<(), MeasurementError> {
    if sample_count < MIN_FFT_SAMPLES {
        return Err(MeasurementError::FftError(format!(
            "spectral analysis requires at least {MIN_FFT_SAMPLES} samples, got {sample_count}"
        )));
    }
    if sample_count > MAX_QUALIFIED_FFT_SAMPLES {
        return Err(MeasurementError::FftError(format!(
            "spectral analysis record has {sample_count} samples; the qualified FFT limit is {MAX_QUALIFIED_FFT_SAMPLES}"
        )));
    }
    Ok(())
}

fn scaled_amplitude_db(
    spectral_weight: Value,
    log10_normalization: Option<Value>,
) -> Result<Value, MeasurementError> {
    if spectral_weight == 0.0 {
        return Ok(Value::NEG_INFINITY);
    }
    if !spectral_weight.is_finite() || spectral_weight < 0.0 {
        return Err(MeasurementError::FftError(format!(
            "FFT spectral weight is invalid ({spectral_weight})"
        )));
    }
    let Some(log10_normalization) = log10_normalization else {
        return Err(MeasurementError::FftError(
            "nonzero FFT coefficient has no amplitude normalization".to_string(),
        ));
    };
    let value = 20.0 * (spectral_weight.log10() + log10_normalization);
    if !value.is_finite() {
        return Err(MeasurementError::FftError(format!(
            "FFT spectral weight {spectral_weight} cannot be represented in dB"
        )));
    }
    Ok(value)
}

fn value_ulp(value: Value) -> Value {
    if !value.is_finite() {
        return Value::INFINITY;
    }
    let upward = (value.next_up() - value).abs();
    let downward = (value - value.next_down()).abs();
    match (upward.is_finite(), downward.is_finite()) {
        (true, true) => upward.max(downward),
        (true, false) => upward,
        (false, true) => downward,
        (false, false) => Value::INFINITY,
    }
}

fn normalized_mean(values: &[Value], scale: Value) -> Result<Value, MeasurementError> {
    if values.is_empty() {
        return Err(MeasurementError::FftError(
            "cannot compute an FFT mean for an empty record".to_string(),
        ));
    }
    let count = values.len() as Value;
    if !count.is_finite() || count as usize != values.len() {
        return Err(MeasurementError::FftError(format!(
            "FFT sample count {} cannot be represented exactly",
            values.len()
        )));
    }
    let mut sum = ExactFloatSum::default();
    for (index, value) in values.iter().enumerate() {
        let normalized = qualified_fft_scaling(*value, scale, index, "mean source scaling")?;
        sum.add(normalized).map_err(|error| {
            MeasurementError::FftError(format!(
                "failed to accumulate normalized FFT mean at sample {index}: {error}"
            ))
        })?;
    }
    let total = sum.finish().map_err(|error| {
        MeasurementError::FftError(format!("failed to finalize normalized FFT mean: {error}"))
    })?;
    let mean = total / count;
    if total != 0.0 && mean == 0.0 {
        return Err(MeasurementError::FftError(
            "dividing the nonzero normalized FFT sum by the sample count underflowed".to_string(),
        ));
    }
    if !mean.is_finite() || !(-1.0..=1.0).contains(&mean) {
        return Err(MeasurementError::FftError(format!(
            "scale-safe waveform mean is outside [-1, 1] ({mean})"
        )));
    }
    Ok(if mean == 0.0 { 0.0 } else { mean })
}

fn qualified_fft_scaling(
    value: Value,
    scale: Value,
    index: usize,
    stage: &str,
) -> Result<Value, MeasurementError> {
    if !value.is_finite() || !scale.is_finite() || scale <= 0.0 {
        return Err(MeasurementError::FftError(format!(
            "FFT {stage} at sample {index} requires a finite value and positive finite scale, got {value} and {scale}"
        )));
    }
    let normalized = value / scale;
    if !normalized.is_finite() {
        return Err(MeasurementError::FftError(format!(
            "FFT {stage} produced a non-finite value at sample {index} ({normalized})"
        )));
    }
    if value != 0.0 && normalized == 0.0 {
        return Err(MeasurementError::FftError(format!(
            "FFT {stage} erased nonzero sample {index} ({value} / {scale})"
        )));
    }
    Ok(normalized)
}

fn map_fourier_measurement_error(error: FourierError) -> MeasurementError {
    let message = format!("THD qualification failed: {error}");
    match error {
        FourierError::EmptyWaveform
        | FourierError::LengthMismatch { .. }
        | FourierError::NonFiniteTime { .. }
        | FourierError::NonFiniteValue { .. }
        | FourierError::NonIncreasingTime { .. }
        | FourierError::InvalidTimeSpan { .. } => MeasurementError::InvalidWaveform(message),
        FourierError::InsufficientSamples { .. }
        | FourierError::InsufficientDuration { .. }
        | FourierError::InsufficientWindowSamples { .. }
        | FourierError::InsufficientSampleRate { .. } => {
            MeasurementError::InsufficientData(message)
        }
        FourierError::HarmonicCapacity { .. }
        | FourierError::WindowCapacity { .. }
        | FourierError::InvalidFundamentalFrequency { .. }
        | FourierError::NoHarmonics
        | FourierError::NoPeriods
        | FourierError::InvalidWindowDuration { .. }
        | FourierError::NonFiniteHarmonicFrequency { .. }
        | FourierError::NonFiniteCoefficient { .. }
        | FourierError::NonFiniteThd { .. }
        | FourierError::InvalidMagnitude { .. }
        | FourierError::InvalidThd { .. }
        | FourierError::UnrepresentableRelativeSpectrum { .. } => {
            MeasurementError::CalculationError(message)
        }
    }
}

fn validate_waveform_input(time: &[Value], values: &[Value]) -> Result<(), MeasurementError> {
    if time.len() != values.len() {
        return Err(MeasurementError::InvalidWaveform(format!(
            "time and values must have the same length (got {} time point(s), {} value(s))",
            time.len(),
            values.len()
        )));
    }
    if time.is_empty() {
        return Err(MeasurementError::InvalidWaveform(
            "waveform cannot be empty".to_string(),
        ));
    }
    for (index, (&t, &v)) in time.iter().zip(values).enumerate() {
        if !t.is_finite() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "time[{index}] must be finite"
            )));
        }
        if !v.is_finite() {
            return Err(MeasurementError::InvalidWaveform(format!(
                "values[{index}] must be finite"
            )));
        }
    }
    for (index, pair) in time.windows(2).enumerate() {
        if pair[1] <= pair[0] {
            return Err(MeasurementError::InvalidWaveform(format!(
                "time points must be strictly increasing (time[{index}]={} >= time[{}]={})",
                pair[0],
                index + 1,
                pair[1]
            )));
        }
    }
    Ok(())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waveform_new_rejects_empty_data_without_panicking() {
        let err = Waveform::new(&[], &[]).expect_err("empty waveform should be invalid");

        assert!(matches!(err, MeasurementError::InvalidWaveform(_)));
        assert!(
            err.to_string().contains("empty"),
            "error should explain the empty waveform: {err}"
        );
    }

    #[test]
    fn waveform_new_rejects_mismatched_data_without_panicking() {
        let err = Waveform::new(&[0.0, 1.0], &[1.0])
            .expect_err("mismatched waveform lengths should be invalid");

        assert!(matches!(err, MeasurementError::InvalidWaveform(_)));
        assert!(
            err.to_string().contains("same length"),
            "error should explain the length mismatch: {err}"
        );
    }

    #[test]
    fn waveform_new_rejects_non_monotonic_time_without_panicking() {
        let err = Waveform::new(&[0.0, 1.0, 0.5], &[0.0, 1.0, 2.0])
            .expect_err("non-monotonic time axis should be invalid");

        assert!(matches!(err, MeasurementError::InvalidWaveform(_)));
        assert!(
            err.to_string().contains("strictly increasing"),
            "error should explain the time ordering problem: {err}"
        );
    }

    #[test]
    fn logarithmic_fft_normalization_preserves_a_nonzero_subnormal_weight() {
        let db = scaled_amplitude_db(Value::from_bits(1), Some(0.0))
            .expect("a nonzero subnormal spectral weight has a finite logarithm");
        assert!(db.is_finite());
        assert!(db < -6_000.0);
    }

    #[test]
    fn fft_resource_limit_fails_before_planning() {
        assert!(validate_fft_sample_count(MAX_QUALIFIED_FFT_SAMPLES).is_ok());
        assert!(matches!(
            validate_fft_sample_count(MAX_QUALIFIED_FFT_SAMPLES + 1),
            Err(MeasurementError::FftError(_))
        ));
    }

    #[test]
    fn normalized_fft_mean_preserves_a_subnormal_cancellation_residual() {
        let minimum_subnormal = Value::from_bits(1);
        let values = [0.125, minimum_subnormal, -0.125, 0.0, 0.0, 0.0, 0.0, 0.0];

        assert_eq!(
            normalized_mean(&values, 0.125)
                .expect("the exact normalized cancellation residual is representable"),
            minimum_subnormal
        );
    }

    #[test]
    fn thd_capacity_failure_is_not_mislabeled_as_an_fft_failure() {
        let time: Vec<_> = (0..8).map(|index| index as Value).collect();
        let values = vec![0.0; time.len()];
        let waveform = Waveform::new(&time, &values).expect("fixture is valid");
        let error = waveform
            .thd(1.0, usize::MAX)
            .expect_err("unrepresentable harmonic capacity must fail");
        assert!(matches!(error, MeasurementError::CalculationError(_)));
        assert!(error.to_string().contains("allocate"));
    }
}
