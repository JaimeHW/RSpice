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
        Ok(Self::from_validated_parts(time, values))
    }

    fn from_validated_parts(time: &[Value], values: &[Value]) -> Self {
        let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        Self {
            time: time.to_vec(),
            values: values.to_vec(),
            min_value,
            max_value,
            min_time: time[0],
            max_time: time[time.len() - 1],
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

    /// Get time span
    #[inline]
    pub fn duration(&self) -> Value {
        self.max_time - self.min_time
    }

    /// Get sample rate (average)
    #[inline]
    pub fn sample_rate(&self) -> Value {
        if self.time.len() < 2 {
            return 0.0;
        }
        let duration = self.duration();
        if !duration.is_finite() || duration <= 0.0 {
            return 0.0;
        }
        (self.time.len() - 1) as Value / duration
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

    /// Get peak-to-peak amplitude
    #[inline]
    pub fn peak_to_peak(&self) -> Value {
        self.max_value - self.min_value
    }

    /// Get average (DC component)
    pub fn average(&self) -> Value {
        if self.values.is_empty() {
            return 0.0;
        }
        self.values.iter().sum::<Value>() / self.values.len() as Value
    }

    /// Get RMS (Root Mean Square) value
    pub fn rms(&self) -> Value {
        if self.values.is_empty() {
            return 0.0;
        }
        let sum_sq: Value = self.values.iter().map(|v| v * v).sum();
        (sum_sq / self.values.len() as Value).sqrt()
    }

    /// Calculate overshoot as percentage of final value
    ///
    /// Overshoot = (peak - final) / (final - initial) × 100%
    pub fn overshoot(&self) -> Result<Value, MeasurementError> {
        if self.values.len() < 3 {
            return Err(MeasurementError::InsufficientData(
                "Need at least 3 points".to_string(),
            ));
        }

        let initial = self.values[0];
        let final_val = *self.values.last().unwrap();
        let step_size = (final_val - initial).abs();

        if step_size < 1e-15 {
            return Ok(0.0); // No step, no overshoot
        }

        if final_val > initial {
            // Rising step
            let peak = self.max_value;
            Ok((peak - final_val) / step_size * 100.0)
        } else {
            // Falling step
            let trough = self.min_value;
            Ok((final_val - trough) / step_size * 100.0)
        }
    }

    /// Calculate undershoot as percentage
    pub fn undershoot(&self) -> Result<Value, MeasurementError> {
        if self.values.len() < 3 {
            return Err(MeasurementError::InsufficientData(
                "Need at least 3 points".to_string(),
            ));
        }

        let initial = self.values[0];
        let final_val = *self.values.last().unwrap();
        let step_size = (final_val - initial).abs();

        if step_size < 1e-15 {
            return Ok(0.0);
        }

        if final_val > initial {
            // Rising step - undershoot is below initial
            let trough = self.min_value;
            Ok((initial - trough) / step_size * 100.0)
        } else {
            // Falling step - undershoot is above initial
            let peak = self.max_value;
            Ok((peak - initial) / step_size * 100.0)
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
    /// Vector of crossing events
    pub fn find_crossings(&self, threshold: Value, direction: EdgeDirection) -> Vec<CrossingEvent> {
        let mut crossings = Vec::new();

        for i in 0..self.values.len().saturating_sub(1) {
            let v0 = self.values[i];
            let v1 = self.values[i + 1];
            let t0 = self.time[i];
            let t1 = self.time[i + 1];

            let is_rising = v0 < threshold && v1 >= threshold;
            let is_falling = v0 >= threshold && v1 < threshold;

            let should_include = match direction {
                EdgeDirection::Rising => is_rising,
                EdgeDirection::Falling => is_falling,
                EdgeDirection::Either => is_rising || is_falling,
            };

            if should_include {
                // Interpolate crossing time
                let dv = v1 - v0;
                let dt = t1 - t0;
                let fraction = if dv.abs() > 1e-15 {
                    (threshold - v0) / dv
                } else {
                    0.5
                };
                let cross_time = t0 + fraction * dt;
                let slew_rate = if dt > 1e-15 { dv / dt } else { 0.0 };

                crossings.push(CrossingEvent {
                    time: cross_time,
                    index: i,
                    direction: if is_rising {
                        EdgeDirection::Rising
                    } else {
                        EdgeDirection::Falling
                    },
                    slew_rate,
                    value: threshold,
                });
            }
        }

        crossings
    }

    /// Find first crossing of threshold
    pub fn first_crossing(
        &self,
        threshold: Value,
        direction: EdgeDirection,
    ) -> Option<CrossingEvent> {
        self.find_crossings(threshold, direction).into_iter().next()
    }

    /// Find time of nth crossing
    pub fn cross_time(
        &self,
        threshold: Value,
        direction: EdgeDirection,
        n: usize,
    ) -> Option<Value> {
        self.find_crossings(threshold, direction)
            .get(n)
            .map(|c| c.time)
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
        if low_pct >= high_pct {
            return Err(MeasurementError::InvalidThreshold(
                "Low threshold must be less than high threshold".to_string(),
            ));
        }
        if low_pct < 0.0 || high_pct > 1.0 {
            return Err(MeasurementError::InvalidThreshold(
                "Thresholds must be between 0 and 1".to_string(),
            ));
        }

        let amplitude = self.peak_to_peak();
        let low_level = self.min_value + low_pct * amplitude;
        let high_level = self.min_value + high_pct * amplitude;

        let low_cross = self.first_crossing(low_level, EdgeDirection::Rising);
        let high_cross = self.first_crossing(high_level, EdgeDirection::Rising);

        match (low_cross, high_cross) {
            (Some(low), Some(high)) if high.time > low.time => Ok(high.time - low.time),
            _ => Err(MeasurementError::ThresholdNotCrossed(
                "Could not find rising edge".to_string(),
            )),
        }
    }

    /// Calculate fall time between two threshold levels
    pub fn fall_time(&self, high_pct: Value, low_pct: Value) -> Result<Value, MeasurementError> {
        if low_pct >= high_pct {
            return Err(MeasurementError::InvalidThreshold(
                "Low threshold must be less than high threshold".to_string(),
            ));
        }

        let amplitude = self.peak_to_peak();
        let low_level = self.min_value + low_pct * amplitude;
        let high_level = self.min_value + high_pct * amplitude;

        let high_cross = self.first_crossing(high_level, EdgeDirection::Falling);
        let low_cross = self.first_crossing(low_level, EdgeDirection::Falling);

        match (high_cross, low_cross) {
            (Some(high), Some(low)) if low.time > high.time => Ok(low.time - high.time),
            _ => Err(MeasurementError::ThresholdNotCrossed(
                "Could not find falling edge".to_string(),
            )),
        }
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
            .first_crossing(ref_threshold, direction)
            .ok_or_else(|| MeasurementError::ThresholdNotCrossed("Reference signal".to_string()))?;

        let self_cross = self
            .first_crossing(self_threshold, direction)
            .ok_or_else(|| MeasurementError::ThresholdNotCrossed("Target signal".to_string()))?;

        Ok(self_cross.time - ref_cross.time)
    }

    /// Calculate period from consecutive crossings
    pub fn period(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let crossings = self.find_crossings(threshold, EdgeDirection::Rising);

        if crossings.len() < 2 {
            return Err(MeasurementError::ThresholdNotCrossed(
                "Need at least 2 rising edges".to_string(),
            ));
        }

        Ok(crossings[1].time - crossings[0].time)
    }

    /// Calculate frequency from period
    pub fn frequency(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let period = self.period(threshold)?;
        if period <= 0.0 {
            return Err(MeasurementError::CalculationError(
                "Invalid period".to_string(),
            ));
        }
        Ok(1.0 / period)
    }

    /// Calculate pulse width (time above threshold)
    pub fn pulse_width(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let rising = self.first_crossing(threshold, EdgeDirection::Rising);
        let falling = self.first_crossing(threshold, EdgeDirection::Falling);

        match (rising, falling) {
            (Some(r), Some(f)) if f.time > r.time => Ok(f.time - r.time),
            (Some(r), Some(f)) => {
                // Falling before rising - measure from falling to end + start to rising
                let width = (self.max_time - f.time) + (r.time - self.min_time);
                Ok(width)
            }
            _ => Err(MeasurementError::ThresholdNotCrossed(
                "Need both rising and falling edges".to_string(),
            )),
        }
    }

    /// Calculate duty cycle as percentage
    pub fn duty_cycle(&self, threshold: Value) -> Result<Value, MeasurementError> {
        let period = self.period(threshold)?;
        let pulse_width = self.pulse_width(threshold)?;

        Ok(pulse_width / period * 100.0)
    }

    /// Calculate slew rate (max dV/dt)
    pub fn slew_rate(&self) -> Result<Value, MeasurementError> {
        if self.time.len() < 2 {
            return Err(MeasurementError::InsufficientData(
                "Need at least 2 points".to_string(),
            ));
        }

        let mut max_slew = 0.0_f64;

        for i in 0..self.time.len() - 1 {
            let dt = self.time[i + 1] - self.time[i];
            if dt > 1e-15 {
                let dv = (self.values[i + 1] - self.values[i]).abs();
                let slew = dv / dt;
                max_slew = max_slew.max(slew);
            }
        }

        Ok(max_slew)
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
        let duration = self.time[sample_count - 1] - self.time[0];
        if !duration.is_finite() || duration <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "spectral analysis requires a finite positive time span, got {duration}"
            )));
        }
        let interval = duration / (sample_count - 1) as Value;
        if !interval.is_finite() || interval <= 0.0 {
            return Err(MeasurementError::FftError(format!(
                "spectral analysis sample interval is invalid ({interval})"
            )));
        }
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
        for value in &self.values {
            let centered = *value / source_scale - normalized_mean;
            if !centered.is_finite() {
                return Err(MeasurementError::FftError(
                    "mean removal produced a non-finite FFT sample".to_string(),
                ));
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
        for value in &self.values {
            let centered = *value / source_scale - normalized_mean;
            buffer.push(Complex::new(centered / centered_scale, 0.0));
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

    /// Interpolate value at arbitrary time
    pub fn interpolate(&self, t: Value) -> Option<Value> {
        if t < self.min_time || t > self.max_time {
            return None;
        }

        // Binary search for interval
        let idx = self.time.partition_point(|&x| x < t);

        if idx == 0 {
            return Some(self.values[0]);
        }
        if idx >= self.time.len() {
            return Some(*self.values.last()?);
        }

        // Linear interpolation
        let t0 = self.time[idx - 1];
        let t1 = self.time[idx];
        let v0 = self.values[idx - 1];
        let v1 = self.values[idx];

        let alpha = (t - t0) / (t1 - t0);
        Some(v0 + alpha * (v1 - v0))
    }

    /// Resample waveform at uniform intervals
    pub fn resample(&self, num_points: usize) -> Self {
        if num_points < 2 || self.time.len() < 2 {
            return self.clone();
        }

        let dt = self.duration() / (num_points - 1) as Value;
        let mut new_time = Vec::with_capacity(num_points);
        let mut new_values = Vec::with_capacity(num_points);

        for i in 0..num_points {
            let t = self.min_time + i as Value * dt;
            new_time.push(t);
            new_values.push(self.interpolate(t).unwrap_or(0.0));
        }

        Self::from_validated_parts(&new_time, &new_values)
    }
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
    (value.next_up() - value)
        .abs()
        .max((value - value.next_down()).abs())
}

fn normalized_mean(values: &[Value], scale: Value) -> Result<Value, MeasurementError> {
    let count = values.len() as Value;
    let mut sum = 0.0;
    let mut compensation = 0.0;
    for value in values {
        let term = (*value / scale) / count;
        let corrected = term - compensation;
        let next = sum + corrected;
        compensation = (next - sum) - corrected;
        sum = next;
    }
    let mean = sum.clamp(-1.0, 1.0);
    if mean.is_finite() {
        Ok(mean)
    } else {
        Err(MeasurementError::FftError(
            "scale-safe waveform mean is non-finite".to_string(),
        ))
    }
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
