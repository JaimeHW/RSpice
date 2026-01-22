//! Period Detection for Autonomous Oscillators
//!
//! Algorithms for detecting the oscillation period of autonomous circuits
//! (oscillators) where the period is not known a priori.

use crate::Value;
use std::f64::consts::PI;

/// Result of period estimation
#[derive(Debug, Clone)]
pub struct PeriodEstimate {
    /// Estimated period (seconds)
    pub period: Value,

    /// Confidence in the estimate (0.0 to 1.0)
    pub confidence: Value,

    /// Method used for estimation
    pub method: EstimationMethod,
}

/// Method used for period estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimationMethod {
    /// Zero-crossing detection
    ZeroCrossing,
    /// Peak detection
    PeakDetection,
    /// FFT-based frequency estimation
    Fft,
    /// Autocorrelation
    Autocorrelation,
}

/// Period detector for autonomous oscillators
///
/// Provides multiple algorithms for estimating the oscillation period
/// from waveform data. Uses a combination of methods and selects the
/// most reliable estimate.
#[derive(Debug)]
pub struct PeriodDetector {
    /// Minimum expected period (to filter out noise)
    min_period: Value,
    /// Maximum expected period
    max_period: Value,
    /// Initial guess for period
    period_guess: Value,
    /// Number of FFT points (must be power of 2)
    fft_size: usize,
}

impl PeriodDetector {
    /// Create a new period detector with expected period range
    ///
    /// # Arguments
    /// * `min_period` - Minimum expected period
    /// * `max_period` - Maximum expected period
    pub fn new(min_period: Value, max_period: Value) -> Self {
        Self {
            min_period,
            max_period,
            period_guess: (min_period + max_period) / 2.0,
            fft_size: 4096,
        }
    }

    /// Create a detector with initial period guess
    pub fn with_guess(period_guess: Value) -> Self {
        Self {
            min_period: period_guess * 0.1,
            max_period: period_guess * 10.0,
            period_guess,
            fft_size: 4096,
        }
    }

    /// Set FFT size for frequency estimation (must be power of 2)
    pub fn with_fft_size(mut self, size: usize) -> Self {
        self.fft_size = size.next_power_of_two();
        self
    }

    /// Detect period from waveform data using multiple methods
    ///
    /// # Arguments
    /// * `time` - Time points
    /// * `values` - Waveform values
    ///
    /// # Returns
    /// Best period estimate with confidence
    pub fn detect(&self, time: &[Value], values: &[Value]) -> PeriodEstimate {
        if time.len() < 4 || values.len() != time.len() {
            return PeriodEstimate {
                period: self.period_guess,
                confidence: 0.0,
                method: EstimationMethod::ZeroCrossing,
            };
        }

        // Try multiple methods and collect all estimates
        let mut estimates = Vec::new();

        // 1. Zero-crossing detection (most reliable for clean signals)
        if let Some(est) = self.detect_zero_crossing(time, values) {
            estimates.push(est);
        }

        // 2. Peak detection
        if let Some(est) = self.detect_peaks(time, values) {
            estimates.push(est);
        }

        // 3. FFT-based detection
        if let Some(est) = self.detect_fft(time, values) {
            estimates.push(est);
        }

        // 4. Autocorrelation
        if let Some(est) = self.detect_autocorrelation(time, values) {
            estimates.push(est);
        }

        if estimates.is_empty() {
            return PeriodEstimate {
                period: self.period_guess,
                confidence: 0.0,
                method: EstimationMethod::ZeroCrossing,
            };
        }

        // Selection strategy:
        // 1. Prefer estimates close to the initial guess (within 2x)
        // 2. Among those, prefer estimates that multiple methods agree on
        // 3. Weight by confidence

        // First, filter estimates within reasonable range of guess
        let near_guess: Vec<_> = estimates
            .iter()
            .filter(|e| {
                let ratio = e.period / self.period_guess;
                ratio > 0.5 && ratio < 2.0
            })
            .collect();

        // If we have estimates near the guess, prefer those
        let candidates = if !near_guess.is_empty() {
            near_guess
        } else {
            estimates.iter().collect()
        };

        // Find the most common period cluster (robust to outliers)
        // Group estimates that are within 10% of each other
        let mut best_estimate = candidates[0].clone();
        let mut best_score = 0.0;

        for candidate in &candidates {
            // Count how many other estimates agree (within 10%)
            let agreement_count = candidates
                .iter()
                .filter(|other| {
                    let ratio = candidate.period / other.period;
                    ratio > 0.9 && ratio < 1.1
                })
                .count();

            // Score = confidence * agreement_bonus
            let agreement_bonus = 1.0 + (agreement_count as f64 - 1.0) * 0.5;
            let score = candidate.confidence * agreement_bonus;

            if score > best_score {
                best_score = score;
                best_estimate = (*candidate).clone();
            }
        }

        best_estimate
    }

    /// Detect period via zero-crossing analysis
    pub fn detect_zero_crossing(&self, time: &[Value], values: &[Value]) -> Option<PeriodEstimate> {
        // Find DC offset (mean value)
        let dc: Value = values.iter().sum::<Value>() / values.len() as f64;

        // Find zero crossings (relative to DC)
        let mut crossings = Vec::new();

        for i in 1..values.len() {
            let v0 = values[i - 1] - dc;
            let v1 = values[i] - dc;

            // Positive-going zero crossing
            if v0 <= 0.0 && v1 > 0.0 {
                // Linear interpolation for precise crossing time
                let t0 = time[i - 1];
                let t1 = time[i];
                let t_cross = t0 + (0.0 - v0) * (t1 - t0) / (v1 - v0);
                crossings.push(t_cross);
            }
        }

        if crossings.len() < 2 {
            return None;
        }

        // Calculate periods between consecutive crossings
        let mut periods: Vec<Value> = crossings
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|&p| p >= self.min_period && p <= self.max_period)
            .collect();

        if periods.is_empty() {
            return None;
        }

        // Use median for robustness against outliers
        periods.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_period = periods[periods.len() / 2];

        // Calculate confidence based on consistency
        let mean_period: Value = periods.iter().sum::<Value>() / periods.len() as f64;
        let variance: Value = periods
            .iter()
            .map(|p| (p - mean_period).powi(2))
            .sum::<Value>()
            / periods.len() as f64;
        let std_dev = variance.sqrt();

        // Confidence decreases with higher relative std dev
        let confidence = if mean_period > 0.0 {
            (1.0 - std_dev / mean_period).max(0.0).min(1.0)
        } else {
            0.0
        };

        Some(PeriodEstimate {
            period: median_period,
            confidence,
            method: EstimationMethod::ZeroCrossing,
        })
    }

    /// Detect period via peak detection
    pub fn detect_peaks(&self, time: &[Value], values: &[Value]) -> Option<PeriodEstimate> {
        // Find local maxima
        let mut peaks = Vec::new();

        for i in 1..(values.len() - 1) {
            if values[i] > values[i - 1] && values[i] > values[i + 1] {
                peaks.push(time[i]);
            }
        }

        if peaks.len() < 2 {
            return None;
        }

        // Calculate periods between consecutive peaks
        let mut periods: Vec<Value> = peaks
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|&p| p >= self.min_period && p <= self.max_period)
            .collect();

        if periods.is_empty() {
            return None;
        }

        // Use median
        periods.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_period = periods[periods.len() / 2];

        // Calculate confidence
        let mean_period: Value = periods.iter().sum::<Value>() / periods.len() as f64;
        let variance: Value = periods
            .iter()
            .map(|p| (p - mean_period).powi(2))
            .sum::<Value>()
            / periods.len() as f64;
        let std_dev = variance.sqrt();

        let confidence = if mean_period > 0.0 {
            (1.0 - std_dev / mean_period).max(0.0).min(1.0)
        } else {
            0.0
        };

        Some(PeriodEstimate {
            period: median_period,
            confidence: confidence * 0.9, // Slightly lower confidence than zero-crossing
            method: EstimationMethod::PeakDetection,
        })
    }

    /// Detect period via FFT analysis
    ///
    /// Uses parabolic interpolation for sub-bin frequency accuracy,
    /// as implemented in industry simulators.
    pub fn detect_fft(&self, time: &[Value], values: &[Value]) -> Option<PeriodEstimate> {
        use rustfft::{FftPlanner, num_complex::Complex};

        let n = values.len().min(self.fft_size);
        if n < 16 {
            return None;
        }

        // Estimate sample rate from the portion of data we're using
        // CRITICAL: Use time[n-1], not time[time.len()-1], since we only use first n samples
        let actual_duration = time[n - 1] - time[0];
        let sample_rate = (n as f64 - 1.0) / actual_duration;

        // Prepare FFT input (with Hann window for spectral leakage reduction)
        let mut buffer: Vec<Complex<f64>> = (0..n)
            .map(|i| {
                let window = 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos());
                Complex::new(values[i] * window, 0.0)
            })
            .collect();

        // Pad to power of 2 if needed
        let fft_len = n.next_power_of_two();
        buffer.resize(fft_len, Complex::new(0.0, 0.0));

        // Perform FFT
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_len);
        fft.process(&mut buffer);

        // Compute magnitude spectrum
        let magnitudes: Vec<f64> = buffer.iter().map(|c| c.norm()).collect();

        // Find peak in magnitude spectrum (excluding DC bin)
        let freq_resolution = sample_rate / fft_len as f64;
        let min_bin = (1.0 / self.max_period / freq_resolution).ceil() as usize;
        let max_bin = (1.0 / self.min_period / freq_resolution).floor() as usize;

        let search_range = min_bin.max(1)..max_bin.min(fft_len / 2 - 1);
        if search_range.is_empty() {
            return None;
        }

        let (peak_bin, peak_magnitude) = search_range
            .clone()
            .map(|i| (i, magnitudes[i]))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())?;

        // Parabolic interpolation for sub-bin frequency accuracy
        // Using 3-point quadratic interpolation (Jacobsen's method)
        // p = 0.5 * (α - γ) / (α - 2β + γ)
        // where α = mag[k-1], β = mag[k], γ = mag[k+1]
        let refined_bin = if peak_bin > 0 && peak_bin < fft_len / 2 - 1 {
            let alpha = magnitudes[peak_bin - 1];
            let beta = magnitudes[peak_bin];
            let gamma = magnitudes[peak_bin + 1];

            let denominator = alpha - 2.0 * beta + gamma;
            if denominator.abs() > 1e-15 {
                let p = 0.5 * (alpha - gamma) / denominator;
                // Clamp interpolation to ±0.5 bins
                peak_bin as f64 + p.clamp(-0.5, 0.5)
            } else {
                peak_bin as f64
            }
        } else {
            peak_bin as f64
        };

        let peak_freq = refined_bin * freq_resolution;
        let period = 1.0 / peak_freq;

        // Confidence based on peak sharpness (ratio to average)
        let avg_magnitude: f64 =
            magnitudes[1..fft_len / 2].iter().sum::<f64>() / (fft_len / 2 - 1) as f64;

        let confidence = if avg_magnitude > 0.0 {
            ((peak_magnitude / avg_magnitude - 1.0) / 10.0)
                .min(1.0)
                .max(0.0)
        } else {
            0.0
        };

        Some(PeriodEstimate {
            period,
            confidence,
            method: EstimationMethod::Fft,
        })
    }

    /// Detect period via autocorrelation
    pub fn detect_autocorrelation(
        &self,
        time: &[Value],
        values: &[Value],
    ) -> Option<PeriodEstimate> {
        let n = values.len();
        if n < 20 {
            return None;
        }

        // Remove DC
        let mean: Value = values.iter().sum::<Value>() / n as f64;
        let centered: Vec<Value> = values.iter().map(|v| v - mean).collect();

        // Estimate sample period
        let sample_period = (time[n - 1] - time[0]) / (n - 1) as f64;

        // Compute lag range
        let min_lag = (self.min_period / sample_period).ceil() as usize;
        let max_lag = (self.max_period / sample_period).floor() as usize;

        if min_lag >= max_lag || max_lag >= n {
            return None;
        }

        // Normalized autocorrelation at lag 0
        let r0: Value = centered.iter().map(|v| v * v).sum();
        if r0.abs() < 1e-15 {
            return None;
        }

        // Find first peak in autocorrelation after initial decay
        let mut autocorr = Vec::new();
        for lag in min_lag..max_lag.min(n / 2) {
            let r: Value = (0..(n - lag))
                .map(|i| centered[i] * centered[i + lag])
                .sum();
            autocorr.push((lag, r / r0));
        }

        // Find peak
        let mut peak_lag = min_lag;
        let mut peak_r = f64::NEG_INFINITY;

        for (lag, r) in autocorr.iter() {
            if *r > peak_r {
                peak_r = *r;
                peak_lag = *lag;
            }
        }

        let period = peak_lag as f64 * sample_period;
        let confidence = peak_r.max(0.0).min(1.0);

        Some(PeriodEstimate {
            period,
            confidence: confidence * 0.85, // Slightly lower confidence
            method: EstimationMethod::Autocorrelation,
        })
    }

    /// Refine period estimate using Newton iteration on the BVP residual
    ///
    /// Given an approximate period T, refines it by finding T* where
    /// the derivative of the residual with respect to period is zero.
    pub fn refine_period(
        &self,
        initial_period: Value,
        state_end: &[Value],
        state_start: &[Value],
        d_state_end_dt: &[Value],
    ) -> Value {
        // The residual is r(T) = x(T) - x(0)
        // We want dr/dT = dx/dT(T) = 0 at the saddle point
        // For autonomous circuits, dx/dT = f(x(T)) where f is the RHS

        // Newton step: T_new = T - r · (dr/dT)^(-1)
        // Approximate dr/dT using the state derivative at T

        let residual_norm_sq: Value = state_end
            .iter()
            .zip(state_start.iter())
            .map(|(e, s)| (e - s).powi(2))
            .sum();

        if residual_norm_sq < 1e-20 {
            return initial_period; // Already converged
        }

        // Inner product of residual with state derivative
        let residual_dot_deriv: Value = state_end
            .iter()
            .zip(state_start.iter())
            .zip(d_state_end_dt.iter())
            .map(|((e, s), d)| (e - s) * d)
            .sum();

        let deriv_norm_sq: Value = d_state_end_dt.iter().map(|d| d * d).sum();

        if deriv_norm_sq < 1e-20 {
            return initial_period;
        }

        // Newton update for period
        let delta_t = -residual_dot_deriv / deriv_norm_sq;

        // Limit period change
        let max_change = initial_period * 0.1;
        let delta_t_limited = delta_t.clamp(-max_change, max_change);

        (initial_period + delta_t_limited).max(self.min_period)
    }
}

impl Default for PeriodDetector {
    fn default() -> Self {
        Self::with_guess(1e-9) // Default 1 ns period (1 GHz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine(freq: Value, duration: Value, n_points: usize) -> (Vec<Value>, Vec<Value>) {
        let time: Vec<Value> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<Value> = time.iter().map(|&t| (2.0 * PI * freq * t).sin()).collect();
        (time, values)
    }

    #[test]
    fn test_zero_crossing_detection() {
        let freq = 1e6; // 1 MHz
        let period = 1e-6;
        let (time, values) = generate_sine(freq, 20.0 * period, 2000);

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect_zero_crossing(&time, &values);

        assert!(estimate.is_some());
        let est = estimate.unwrap();
        assert!((est.period - period).abs() < period * 0.01); // Within 1%
        assert!(est.confidence > 0.9);
    }

    #[test]
    fn test_peak_detection() {
        let freq = 10e6; // 10 MHz
        let period = 100e-9;
        let (time, values) = generate_sine(freq, 30.0 * period, 3000);

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect_peaks(&time, &values);

        assert!(estimate.is_some());
        let est = estimate.unwrap();
        assert!((est.period - period).abs() < period * 0.02); // Within 2%
    }

    #[test]
    fn test_fft_detection() {
        let freq = 100e6; // 100 MHz
        let period = 10e-9;
        let (time, values) = generate_sine(freq, 50.0 * period, 4096);

        let detector = PeriodDetector::with_guess(period).with_fft_size(4096);
        let estimate = detector.detect_fft(&time, &values);

        assert!(estimate.is_some());
        let est = estimate.unwrap();
        // FFT has limited resolution, allow 5% error
        assert!((est.period - period).abs() < period * 0.05);
    }

    #[test]
    fn test_autocorrelation_detection() {
        let freq = 5e6; // 5 MHz
        let period = 200e-9;
        let (time, values) = generate_sine(freq, 100.0 * period, 5000);

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect_autocorrelation(&time, &values);

        assert!(estimate.is_some());
        let est = estimate.unwrap();
        assert!((est.period - period).abs() < period * 0.03); // Within 3%
    }

    #[test]
    fn test_combined_detection() {
        let freq = 1e9; // 1 GHz
        let period = 1e-9;
        let (time, values) = generate_sine(freq, 20.0 * period, 2048);

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        assert!((estimate.period - period).abs() < period * 0.05);
        assert!(estimate.confidence > 0.5);
    }

    #[test]
    fn test_square_wave_detection() {
        let freq = 10e6;
        let period = 100e-9;
        let n_points = 1000;
        let duration = 20.0 * period;

        let time: Vec<Value> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| if (t * freq).fract() < 0.5 { 1.0 } else { -1.0 })
            .collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        assert!((estimate.period - period).abs() < period * 0.05);
    }

    #[test]
    fn test_period_refinement() {
        let detector = PeriodDetector::with_guess(1e-9);

        // Simulate a case where we're close to the correct period
        // For a sinusoidal oscillator on unit circle at period T=1ns:
        // state_start = [1.0, 0.0] (starting point)
        // state_end = [0.999, 0.001] (slightly off due to period error)
        // d_state_dt = [-0.001, 6.283] (derivative = omega * perpendicular)
        let state_start = vec![1.0, 0.0];
        let state_end = vec![0.999, 0.001]; // Small residual
        let d_state_dt = vec![-0.001, 6.283]; // Derivative at T (omega ≈ 2π/1ns)

        let initial_period = 1e-9;
        let refined = detector.refine_period(initial_period, &state_end, &state_start, &d_state_dt);

        // The refined period should be close to initial (within 10% change limit)
        // and should be a valid positive period
        assert!(refined > 0.0, "Refined period must be positive");
        assert!(
            refined >= detector.min_period,
            "Refined period must be >= min_period"
        );
        // The algorithm limits period change to 10% per iteration
        assert!(
            (refined - initial_period).abs() <= initial_period * 0.1 + 1e-15,
            "Period change should be limited: initial={}, refined={}, diff={}",
            initial_period,
            refined,
            (refined - initial_period).abs()
        );
    }

    #[test]
    fn test_noisy_signal() {
        let freq = 1e6;
        let period = 1e-6;
        let n_points = 2000;
        let duration = 30.0 * period;

        let time: Vec<Value> = (0..n_points)
            .map(|i| i as f64 * duration / (n_points - 1) as f64)
            .collect();

        // Add some noise
        let values: Vec<Value> = time
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let signal = (2.0 * PI * freq * t).sin();
                let noise = ((i as f64 * 1234.5).sin()) * 0.1;
                signal + noise
            })
            .collect();

        let detector = PeriodDetector::with_guess(period);
        let estimate = detector.detect(&time, &values);

        // Should still detect approximate period despite noise
        assert!((estimate.period - period).abs() < period * 0.1);
    }
}
