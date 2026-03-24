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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sine_wave(
        freq: Value,
        amplitude: Value,
        offset: Value,
        num_points: usize,
    ) -> Waveform {
        let duration = 3.0 / freq; // 3 periods
        let dt = duration / (num_points - 1) as Value;
        // Start with small negative phase so first zero crossing is captured
        // This ensures we start below the offset and cross through it
        let phase_offset = -0.01 * PI; // Start just before 0 phase

        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| offset + amplitude * (2.0 * PI * freq * t + phase_offset).sin())
            .collect();

        Waveform::new(&time, &values)
    }

    fn create_step_response(
        tau: Value,
        v_init: Value,
        v_final: Value,
        num_points: usize,
    ) -> Waveform {
        let duration = 5.0 * tau;
        let dt = duration / (num_points - 1) as Value;

        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| v_init + (v_final - v_init) * (1.0 - (-t / tau).exp()))
            .collect();

        Waveform::new(&time, &values)
    }

    fn create_pulse(period: Value, duty: Value, amplitude: Value, num_points: usize) -> Waveform {
        let duration = 2.5 * period; // Ensure we capture complete pulses
        let dt = duration / (num_points - 1) as Value;
        // Start with phase offset past the duty cycle so signal begins LOW
        // This ensures the first crossing is a rising edge
        let phase_offset = duty + 0.05; // Start just after the pulse ends

        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| {
                let phase = ((t / period) + phase_offset) % 1.0;
                if phase < duty { amplitude } else { 0.0 }
            })
            .collect();

        Waveform::new(&time, &values)
    }

    #[test]
    fn test_waveform_creation() {
        let time = vec![0.0, 1.0, 2.0, 3.0];
        let values = vec![0.0, 1.0, 2.0, 1.0];
        let wf = Waveform::new(&time, &values);

        assert_eq!(wf.len(), 4);
        assert!(!wf.is_empty());
        assert!((wf.duration() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_amplitude_measurements() {
        let wf = create_sine_wave(1000.0, 1.0, 0.0, 1000);

        assert!((wf.min() - (-1.0)).abs() < 0.05);
        assert!((wf.max() - 1.0).abs() < 0.05);
        assert!((wf.peak_to_peak() - 2.0).abs() < 0.1);
        assert!(wf.average().abs() < 0.05); // DC should be ~0

        // RMS of sine wave = amplitude / sqrt(2)
        let expected_rms = 1.0 / 2.0_f64.sqrt();
        assert!((wf.rms() - expected_rms).abs() < 0.1);
    }

    #[test]
    fn test_dc_offset() {
        let wf = create_sine_wave(1000.0, 1.0, 2.5, 1000);

        assert!((wf.average() - 2.5).abs() < 0.1);
        assert!((wf.min() - 1.5).abs() < 0.1);
        assert!((wf.max() - 3.5).abs() < 0.1);
    }

    #[test]
    fn test_threshold_crossing() {
        let wf = create_sine_wave(1000.0, 1.0, 0.0, 1000);

        // Should find 6 zero crossings (3 rising, 3 falling for 3 periods)
        let all_crossings = wf.find_crossings(0.0, EdgeDirection::Either);
        assert!(all_crossings.len() >= 6);

        let rising = wf.find_crossings(0.0, EdgeDirection::Rising);
        assert!(rising.len() >= 3);

        let falling = wf.find_crossings(0.0, EdgeDirection::Falling);
        assert!(falling.len() >= 3);
    }

    #[test]
    fn test_first_crossing() {
        let wf = create_step_response(1e-6, 0.0, 3.3, 1000);

        let crossing = wf.first_crossing(1.65, EdgeDirection::Rising);
        assert!(crossing.is_some());

        let event = crossing.unwrap();
        assert_eq!(event.direction, EdgeDirection::Rising);
        assert!(event.time > 0.0);
    }

    #[test]
    fn test_rise_time() {
        let wf = create_step_response(1e-6, 0.0, 3.3, 10000);

        let rise_time = wf.rise_time(0.1, 0.9).unwrap();

        // For RC step response: t_rise = τ × ln(0.9/0.1) = τ × 2.197
        let expected = 1e-6 * (0.9_f64.ln() - 0.1_f64.ln());
        assert!((rise_time - expected).abs() / expected < 0.1);
    }

    #[test]
    fn test_rise_time_invalid_thresholds() {
        let wf = create_step_response(1e-6, 0.0, 3.3, 1000);

        // Low >= high should fail
        assert!(wf.rise_time(0.9, 0.1).is_err());
        assert!(wf.rise_time(0.5, 0.5).is_err());

        // Out of range should fail
        assert!(wf.rise_time(-0.1, 0.9).is_err());
        assert!(wf.rise_time(0.1, 1.1).is_err());
    }

    #[test]
    fn test_period_frequency() {
        let freq = 1e6; // 1 MHz
        let wf = create_sine_wave(freq, 1.0, 0.0, 10000);

        let measured_period = wf.period(0.0).unwrap();
        let expected_period = 1.0 / freq;

        assert!((measured_period - expected_period).abs() / expected_period < 0.05);

        let measured_freq = wf.frequency(0.0).unwrap();
        assert!((measured_freq - freq).abs() / freq < 0.05);
    }

    #[test]
    fn test_pulse_width_duty_cycle() {
        let period = 1e-6;
        let duty = 0.3; // 30%
        let wf = create_pulse(period, duty, 3.3, 10000);

        let measured_width = wf.pulse_width(1.65).unwrap();
        let expected_width = period * duty;

        assert!((measured_width - expected_width).abs() / expected_width < 0.1);

        let measured_duty = wf.duty_cycle(1.65).unwrap();
        assert!((measured_duty - 30.0).abs() < 5.0);
    }

    #[test]
    fn test_overshoot_undershoot() {
        // Create damped step response with overshoot
        let num_points = 1000;
        let tau = 1e-6;
        let duration = 10.0 * tau;
        let dt = duration / (num_points - 1) as f64;

        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| {
                // Underdamped step response
                let omega_n: f64 = 1e6;
                let zeta: f64 = 0.3;
                let omega_d = omega_n * (1.0_f64 - zeta * zeta).sqrt();
                3.3 * (1.0
                    - (-zeta * omega_n * t).exp()
                        * ((omega_d * t).cos()
                            + zeta / (1.0_f64 - zeta * zeta).sqrt() * (omega_d * t).sin()))
            })
            .collect();

        let wf = Waveform::new(&time, &values);
        let overshoot = wf.overshoot().unwrap();

        // Should have positive overshoot
        assert!(overshoot > 0.0);
        assert!(overshoot < 100.0); // Reasonable range
    }

    #[test]
    fn test_slew_rate() {
        let wf = create_step_response(1e-9, 0.0, 3.3, 10000);

        let slew = wf.slew_rate().unwrap();

        // Should be positive and reasonable
        assert!(slew > 0.0);
        assert!(slew < 1e15); // Not infinity
    }

    #[test]
    fn test_fft_sine() {
        let freq = 1e6;
        let wf = create_sine_wave(freq, 1.0, 0.0, 1024);

        let (freqs, mags) = wf.fft().unwrap();

        // Should have positive frequencies
        assert!(freqs.len() > 10);
        assert!(freqs[0] >= 0.0);

        // Find peak
        let mut max_idx = 1;
        let mut max_mag = mags[1];
        for (i, &m) in mags.iter().enumerate().skip(2) {
            if m > max_mag {
                max_mag = m;
                max_idx = i;
            }
        }

        // Peak should be near fundamental
        let peak_freq = freqs[max_idx];
        assert!((peak_freq - freq).abs() / freq < 0.1);
    }

    #[test]
    fn test_fundamental_frequency() {
        let freq = 1e6;
        let wf = create_sine_wave(freq, 1.0, 0.0, 1024);

        let detected_freq = wf.fundamental_frequency().unwrap();

        assert!((detected_freq - freq).abs() / freq < 0.1);
    }

    #[test]
    fn test_thd_pure_sine() {
        let wf = create_sine_wave(1e6, 1.0, 0.0, 1024);

        let thd = wf.thd(10).unwrap();

        // Pure sine should have very low THD
        assert!(thd < 5.0, "Pure sine THD should be low, got {}%", thd);
    }

    #[test]
    fn test_thd_square_wave() {
        // Create square wave (has significant harmonics)
        let period = 1e-6;
        let num_points = 4096;
        let duration = 10.0 * period;
        let dt = duration / (num_points - 1) as f64;

        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| {
                if (t / period * 2.0) as i64 % 2 == 0 {
                    1.0
                } else {
                    -1.0
                }
            })
            .collect();

        let wf = Waveform::new(&time, &values);
        let thd = wf.thd(10).unwrap();

        // Square wave has ~48% THD theoretically
        assert!(thd > 20.0, "Square wave should have significant THD");
    }

    #[test]
    fn test_interpolate() {
        let time = vec![0.0, 1.0, 2.0, 3.0];
        let values = vec![0.0, 2.0, 4.0, 6.0];
        let wf = Waveform::new(&time, &values);

        // Exact points
        assert!((wf.interpolate(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((wf.interpolate(1.0).unwrap() - 2.0).abs() < 1e-10);

        // Interpolated
        assert!((wf.interpolate(0.5).unwrap() - 1.0).abs() < 1e-10);
        assert!((wf.interpolate(2.5).unwrap() - 5.0).abs() < 1e-10);

        // Out of range
        assert!(wf.interpolate(-1.0).is_none());
        assert!(wf.interpolate(4.0).is_none());
    }

    #[test]
    fn test_resample() {
        let wf = create_sine_wave(1e6, 1.0, 0.0, 100);
        let resampled = wf.resample(500);

        assert_eq!(resampled.len(), 500);
        assert!((resampled.duration() - wf.duration()).abs() < 1e-12);
    }

    #[test]
    fn test_crossing_slew_rate() {
        let wf = create_sine_wave(1e6, 1.0, 0.0, 10000);

        let crossings = wf.find_crossings(0.0, EdgeDirection::Rising);
        assert!(!crossings.is_empty());

        // Slew rate at crossing should be positive for rising
        let slew = crossings[0].slew_rate;
        assert!(slew > 0.0);
    }

    #[test]
    fn test_ac_rms() {
        let wf = create_sine_wave(1e6, 1.0, 5.0, 1000);

        // AC RMS should be amplitude / sqrt(2) regardless of DC offset
        let expected_ac_rms = 1.0 / 2.0_f64.sqrt();
        assert!((wf.ac_rms() - expected_ac_rms).abs() < 0.1);
    }

    #[test]
    fn test_measurement_result() {
        let result = MeasurementResult::new(1.5e-9, "s", "Rise time")
            .with_time(1e-6)
            .with_index(42);

        assert!((result.value - 1.5e-9).abs() < 1e-15);
        assert_eq!(result.unit, "s");
        assert_eq!(result.time, Some(1e-6));
        assert_eq!(result.index, Some(42));
    }

    #[test]
    fn test_edge_direction_default() {
        assert_eq!(EdgeDirection::default(), EdgeDirection::Either);
    }

    #[test]
    fn test_measurement_error_display() {
        let err = MeasurementError::InsufficientData("test".to_string());
        assert!(err.to_string().contains("Insufficient"));

        let err = MeasurementError::ThresholdNotCrossed("test".to_string());
        assert!(err.to_string().contains("Threshold"));
    }

    #[test]
    fn test_delay_measurement() {
        // Create input and output with known delay
        let freq = 1e6;
        let num_points = 10000;
        let delay_time = 100e-9; // 100ns delay

        let input = create_sine_wave(freq, 1.0, 0.0, num_points);

        // Create delayed version
        let duration = 3.0 / freq;
        let dt = duration / (num_points - 1) as Value;
        let time: Vec<Value> = (0..num_points).map(|i| i as Value * dt).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| (2.0 * PI * freq * (t - delay_time)).sin())
            .collect();
        let output = Waveform::new(&time, &values);

        let measured_delay = output
            .delay(&input, 0.0, 0.0, EdgeDirection::Rising)
            .unwrap();

        // Should be close to expected delay
        assert!((measured_delay - delay_time).abs() < 10e-9);
    }

    #[test]
    fn test_sample_rate() {
        let time = vec![0.0, 1e-9, 2e-9, 3e-9, 4e-9];
        let values = vec![0.0, 1.0, 2.0, 1.0, 0.0];
        let wf = Waveform::new(&time, &values);

        let sr = wf.sample_rate();
        assert!((sr - 1e9).abs() / 1e9 < 0.01);
    }

    #[test]
    fn test_sample_rate_zero_duration_returns_zero() {
        let time = vec![1.0, 1.0, 1.0];
        let values = vec![0.0, 1.0, 2.0];
        let wf = Waveform::new(&time, &values);

        assert_eq!(wf.sample_rate(), 0.0);
    }

    #[test]
    fn test_sample_rate_non_finite_duration_returns_zero() {
        let time = vec![0.0, f64::INFINITY];
        let values = vec![0.0, 1.0];
        let wf = Waveform::new(&time, &values);

        assert_eq!(wf.sample_rate(), 0.0);
    }

    #[test]
    fn test_fft_insufficient_data() {
        let time = vec![0.0, 1.0];
        let values = vec![0.0, 1.0];
        let wf = Waveform::new(&time, &values);

        assert!(wf.fft().is_err());
    }

    #[test]
    fn test_period_insufficient_crossings() {
        let time = vec![0.0, 1.0, 2.0];
        let values = vec![0.0, 0.5, 1.0]; // Only one rising crossing of 0.5
        let wf = Waveform::new(&time, &values);

        assert!(wf.period(0.5).is_err());
    }

    #[test]
    fn test_value_at_time_at() {
        let time = vec![0.0, 1.0, 2.0];
        let values = vec![1.0, 2.0, 3.0];
        let wf = Waveform::new(&time, &values);

        assert_eq!(wf.value_at(0), Some(1.0));
        assert_eq!(wf.value_at(1), Some(2.0));
        assert_eq!(wf.value_at(3), None);

        assert_eq!(wf.time_at(0), Some(0.0));
        assert_eq!(wf.time_at(2), Some(2.0));
        assert_eq!(wf.time_at(5), None);
    }

    #[test]
    fn test_sfdr_calculation() {
        // Pure sine should have high SFDR
        let wf = create_sine_wave(1e6, 1.0, 0.0, 1024);

        let sfdr = wf.sfdr().unwrap();

        // SFDR should be positive (fundamental larger than spurs)
        assert!(sfdr > 0.0);
    }
}
