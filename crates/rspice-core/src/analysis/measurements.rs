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
//! - FFT (Fast Fourier Transform)
//! - THD (Total Harmonic Distortion)
//! - SFDR (Spurious-Free Dynamic Range)
//! - Fundamental frequency detection
//!
//! # Usage
//! ```ignore
//! let waveform = Waveform::new(&time_points, &voltage_values);
//! let rise_time = waveform.rise_time(0.1, 0.9)?;
//! let thd = waveform.thd(fundamental_freq)?;
//! ```

use crate::Value;
use std::f64::consts::PI;

//=============================================================================
// Constants
//=============================================================================

/// Default low threshold for rise/fall time (10%)
pub const THRESHOLD_LOW: Value = 0.1;
/// Default high threshold for rise/fall time (90%)
pub const THRESHOLD_HIGH: Value = 0.9;
/// Minimum number of samples for FFT
pub const MIN_FFT_SAMPLES: usize = 8;
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

    /// Add time information
    pub fn with_time(mut self, time: Value) -> Self {
        self.time = Some(time);
        self
    }

    /// Add index information
    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }
}

//=============================================================================
// Measurement Error
//=============================================================================

/// Errors that can occur during measurement
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementError {
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
    /// # Panics
    /// Panics if time and values have different lengths or are empty
    pub fn new(time: &[Value], values: &[Value]) -> Self {
        assert_eq!(
            time.len(),
            values.len(),
            "Time and values must have same length"
        );
        assert!(!time.is_empty(), "Waveform cannot be empty");

        let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        Self {
            time: time.to_vec(),
            values: values.to_vec(),
            min_value,
            max_value,
            min_time: time[0],
            max_time: *time.last().unwrap(),
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

    /// Get AC RMS (RMS with DC removed)
    pub fn ac_rms(&self) -> Value {
        if self.values.is_empty() {
            return 0.0;
        }
        let dc = self.average();
        let sum_sq: Value = self.values.iter().map(|v| (v - dc).powi(2)).sum();
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

    /// Compute FFT of waveform
    ///
    /// Returns (frequencies, magnitudes) where magnitudes are in dB
    pub fn fft(&self) -> Result<(Vec<Value>, Vec<Value>), MeasurementError> {
        let n = self.values.len();
        if n < MIN_FFT_SAMPLES {
            return Err(MeasurementError::FftError(format!(
                "Need at least {} samples, got {}",
                MIN_FFT_SAMPLES, n
            )));
        }

        // Find next power of 2
        let n_fft = n.next_power_of_two();

        // Zero-pad input
        let mut real: Vec<Value> = self.values.clone();
        real.resize(n_fft, 0.0);
        let mut imag: Vec<Value> = vec![0.0; n_fft];

        // In-place FFT (Cooley-Tukey)
        self.fft_in_place(&mut real, &mut imag);

        // Calculate sample rate
        let fs = self.sample_rate();
        let df = fs / n_fft as Value;

        // Calculate magnitudes and frequencies (positive half only)
        let n_bins = n_fft / 2 + 1;
        let mut freqs = Vec::with_capacity(n_bins);
        let mut mags = Vec::with_capacity(n_bins);

        for i in 0..n_bins {
            freqs.push(i as Value * df);
            let mag = (real[i].powi(2) + imag[i].powi(2)).sqrt() / n_fft as Value;
            // Convert to dB, with floor to avoid log(0)
            let mag_db = 20.0 * (mag.max(1e-15)).log10();
            mags.push(mag_db);
        }

        Ok((freqs, mags))
    }

    /// In-place FFT using Cooley-Tukey algorithm
    fn fft_in_place(&self, real: &mut [Value], imag: &mut [Value]) {
        let n = real.len();
        if n <= 1 {
            return;
        }

        // Bit-reversal permutation
        let mut j = 0;
        for i in 0..n {
            if i < j {
                real.swap(i, j);
                imag.swap(i, j);
            }
            let mut m = n >> 1;
            while m >= 1 && j >= m {
                j -= m;
                m >>= 1;
            }
            j += m;
        }

        // Cooley-Tukey FFT
        let mut len = 2;
        while len <= n {
            let half_len = len / 2;
            let angle_step = -2.0 * PI / len as Value;

            for i in (0..n).step_by(len) {
                for k in 0..half_len {
                    let angle = angle_step * k as Value;
                    let wr = angle.cos();
                    let wi = angle.sin();

                    let idx1 = i + k;
                    let idx2 = i + k + half_len;

                    let tr = real[idx2] * wr - imag[idx2] * wi;
                    let ti = real[idx2] * wi + imag[idx2] * wr;

                    real[idx2] = real[idx1] - tr;
                    imag[idx2] = imag[idx1] - ti;
                    real[idx1] += tr;
                    imag[idx1] += ti;
                }
            }
            len <<= 1;
        }
    }

    /// Calculate Total Harmonic Distortion (THD)
    ///
    /// THD = sqrt(V2² + V3² + ... + Vn²) / V1 × 100%
    pub fn thd(&self, num_harmonics: usize) -> Result<Value, MeasurementError> {
        let (freqs, mags) = self.fft()?;

        if freqs.len() < 2 {
            return Err(MeasurementError::FftError(
                "Insufficient FFT bins".to_string(),
            ));
        }

        // Find fundamental (largest magnitude, excluding DC)
        let mut fund_idx = 1;
        let mut fund_mag = mags[1];

        for (i, &mag) in mags.iter().enumerate().skip(2) {
            if mag > fund_mag {
                fund_mag = mag;
                fund_idx = i;
            }
        }

        // Convert fundamental from dB to linear
        let fund_linear = 10.0_f64.powf(fund_mag / 20.0);

        // Sum harmonics
        let mut harmonic_sum_sq = 0.0;
        for h in 2..=num_harmonics {
            let harmonic_idx = fund_idx * h;
            if harmonic_idx < mags.len() {
                let harmonic_linear = 10.0_f64.powf(mags[harmonic_idx] / 20.0);
                harmonic_sum_sq += harmonic_linear * harmonic_linear;
            }
        }

        if fund_linear < 1e-15 {
            return Err(MeasurementError::CalculationError(
                "Fundamental too small".to_string(),
            ));
        }

        Ok(harmonic_sum_sq.sqrt() / fund_linear * 100.0)
    }

    /// Calculate Spurious-Free Dynamic Range (SFDR)
    ///
    /// SFDR = 20 × log10(V_fundamental / V_largest_spur) in dB
    pub fn sfdr(&self) -> Result<Value, MeasurementError> {
        let (freqs, mags) = self.fft()?;

        if freqs.len() < 3 {
            return Err(MeasurementError::FftError(
                "Insufficient FFT bins".to_string(),
            ));
        }

        // Find fundamental (largest magnitude, excluding DC)
        let mut fund_idx = 1;
        let mut fund_mag = mags[1];

        for (i, &mag) in mags.iter().enumerate().skip(2) {
            if mag > fund_mag {
                fund_mag = mag;
                fund_idx = i;
            }
        }

        // Find largest spur (excluding fundamental and its immediate neighbors)
        let mut spur_mag = f64::NEG_INFINITY;

        for (i, &mag) in mags.iter().enumerate().skip(1) {
            if (i as i64 - fund_idx as i64).abs() > 2 {
                spur_mag = spur_mag.max(mag);
            }
        }

        if spur_mag == f64::NEG_INFINITY {
            return Err(MeasurementError::CalculationError(
                "No spurs found".to_string(),
            ));
        }

        Ok(fund_mag - spur_mag)
    }

    /// Detect fundamental frequency from FFT
    pub fn fundamental_frequency(&self) -> Result<Value, MeasurementError> {
        let (freqs, mags) = self.fft()?;

        if freqs.len() < 2 {
            return Err(MeasurementError::FftError(
                "Insufficient FFT bins".to_string(),
            ));
        }

        // Find peak (excluding DC)
        let mut max_idx = 1;
        let mut max_mag = mags[1];

        for (i, &mag) in mags.iter().enumerate().skip(2) {
            if mag > max_mag {
                max_mag = mag;
                max_idx = i;
            }
        }

        Ok(freqs[max_idx])
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
        if num_points < 2 {
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

        Waveform::new(&new_time, &new_values)
    }
}

//=============================================================================
// Tests
//=============================================================================
