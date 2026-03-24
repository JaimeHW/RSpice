//! Fourier Analysis (.FOUR directive)
//!
//! Computes Fourier components and Total Harmonic Distortion (THD) from
//! transient simulation results.
//!
//! # SPICE Syntax
//! ```text
//! .FOUR <freq> [nharms] <output> [output...]
//! .FOUR 1kHz V(out)
//! .FOUR 60Hz 15 V(load) I(Rsense)
//! ```
//!
//! # Output
//! Computes magnitude and phase of DC, fundamental, and harmonics up to nharms.
//! Also computes THD (Total Harmonic Distortion).
//!
//! # Algorithm
//! Uses DFT (Discrete Fourier Transform) on the last period of the waveform.
//! For large datasets, considers FFT optimization.

use crate::Value;
use std::f64::consts::PI;

//=============================================================================
// Fourier Analysis Settings
//=============================================================================

/// Configuration for Fourier analysis
#[derive(Debug, Clone)]
pub struct FourierConfig {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: Value,
    /// Number of harmonics to compute (default 9)
    pub num_harmonics: usize,
    /// Number of periods to analyze (default 1)
    pub num_periods: usize,
}

impl FourierConfig {
    /// Create new Fourier config with fundamental frequency
    pub fn new(freq: Value) -> Self {
        Self {
            fundamental_freq: freq,
            num_harmonics: 9,
            num_periods: 1,
        }
    }

    /// Set number of harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n;
        self
    }

    /// Get period of fundamental
    pub fn period(&self) -> Value {
        1.0 / self.fundamental_freq
    }

    /// Get analysis window duration
    pub fn window_duration(&self) -> Value {
        self.period() * self.num_periods as f64
    }
}

//=============================================================================
// Fourier Analysis Engine
//=============================================================================

/// Fourier Analysis Engine
#[derive(Debug, Clone)]
pub struct FourierAnalysis {
    /// Configuration
    config: FourierConfig,
}

impl FourierAnalysis {
    /// Create new Fourier analysis
    pub fn new(config: FourierConfig) -> Self {
        Self { config }
    }

    /// Perform Fourier analysis on a waveform
    ///
    /// # Arguments
    /// * `time` - Time points
    /// * `values` - Waveform values at each time point
    ///
    /// # Returns
    /// FourierResult with harmonic components and THD
    pub fn analyze(&self, time: &[Value], values: &[Value]) -> FourierResult {
        if time.len() != values.len() || time.is_empty() {
            return FourierResult::empty(self.config.num_harmonics);
        }

        // Find analysis window (last periods of waveform)
        let _period = self.config.period();
        let window_duration = self.config.window_duration();
        let t_end = time[time.len() - 1];
        let t_start = (t_end - window_duration).max(time[0]);

        // Find indices for analysis window
        let (start_idx, end_idx) = find_window_indices(time, t_start, t_end);

        if end_idx <= start_idx + 1 {
            return FourierResult::empty(self.config.num_harmonics);
        }

        // Extract window
        let window_time: Vec<Value> = time[start_idx..=end_idx].to_vec();
        let window_values: Vec<Value> = values[start_idx..=end_idx].to_vec();

        // Compute Fourier coefficients using DFT
        let mut harmonics = Vec::with_capacity(self.config.num_harmonics + 1);

        for n in 0..=self.config.num_harmonics {
            let freq = n as f64 * self.config.fundamental_freq;
            let (mag, phase) = self.compute_harmonic(&window_time, &window_values, freq);

            harmonics.push(HarmonicComponent {
                harmonic_number: n,
                frequency: freq,
                magnitude: mag,
                phase,
            });
        }

        // Calculate THD
        let dc = harmonics[0].magnitude;
        let fundamental = harmonics.get(1).map(|h| h.magnitude).unwrap_or(0.0);

        let harmonic_sum_sq: Value = harmonics
            .iter()
            .skip(2) // Skip DC and fundamental
            .map(|h| h.magnitude * h.magnitude)
            .sum();

        let thd = if fundamental > 1e-15 {
            harmonic_sum_sq.sqrt() / fundamental * 100.0
        } else {
            0.0
        };

        FourierResult {
            fundamental_freq: self.config.fundamental_freq,
            dc_component: dc,
            harmonics,
            thd,
        }
    }

    /// Compute single harmonic component using numerical integration
    fn compute_harmonic(&self, time: &[Value], values: &[Value], freq: Value) -> (Value, Value) {
        if time.len() < 2 {
            return (0.0, 0.0);
        }

        let t_start = time[0];
        let t_end = time[time.len() - 1];
        let duration = t_end - t_start;

        if duration <= 0.0 {
            return (0.0, 0.0);
        }

        // For DC (n=0), just compute average
        if freq.abs() < 1e-10 {
            let avg = trapezoidal_integrate(time, values) / duration;
            return (avg, 0.0);
        }

        // Compute a_n and b_n using trapezoidal integration
        // a_n = (2/T) * integral(f(t) * cos(2*pi*n*f*t) dt)
        // b_n = (2/T) * integral(f(t) * sin(2*pi*n*f*t) dt)

        let omega = 2.0 * PI * freq;

        let cos_values: Vec<Value> = time
            .iter()
            .zip(values.iter())
            .map(|(&t, &v)| v * (omega * (t - t_start)).cos())
            .collect();

        let sin_values: Vec<Value> = time
            .iter()
            .zip(values.iter())
            .map(|(&t, &v)| v * (omega * (t - t_start)).sin())
            .collect();

        let a_n = 2.0 / duration * trapezoidal_integrate(time, &cos_values);
        let b_n = 2.0 / duration * trapezoidal_integrate(time, &sin_values);

        let magnitude = (a_n * a_n + b_n * b_n).sqrt();
        let phase = (-b_n).atan2(a_n) * 180.0 / PI; // Convert to degrees

        (magnitude, phase)
    }
}

/// Find start and end indices for time window
fn find_window_indices(time: &[Value], t_start: Value, t_end: Value) -> (usize, usize) {
    let start_idx = time.iter().position(|&t| t >= t_start).unwrap_or(0);

    let end_idx = time
        .iter()
        .rposition(|&t| t <= t_end)
        .unwrap_or(time.len() - 1);

    (start_idx, end_idx)
}

/// Trapezoidal integration
///
/// When compiled with the `simd` feature, uses SIMD-accelerated integration
/// for 2-3x faster performance on large waveforms.
fn trapezoidal_integrate(time: &[Value], values: &[Value]) -> Value {
    #[cfg(feature = "simd")]
    {
        crate::simd::trapezoidal_integrate(time, values)
    }
    #[cfg(not(feature = "simd"))]
    {
        if time.len() < 2 {
            return 0.0;
        }

        let mut integral = 0.0;
        for i in 1..time.len() {
            let dt = time[i] - time[i - 1];
            integral += 0.5 * (values[i] + values[i - 1]) * dt;
        }
        integral
    }
}

//=============================================================================
// Results
//=============================================================================

/// Single harmonic component
#[derive(Debug, Clone)]
pub struct HarmonicComponent {
    /// Harmonic number (0 = DC, 1 = fundamental, 2 = 2nd harmonic, etc.)
    pub harmonic_number: usize,
    /// Frequency (Hz)
    pub frequency: Value,
    /// Magnitude
    pub magnitude: Value,
    /// Phase (degrees)
    pub phase: Value,
}

impl HarmonicComponent {
    /// Get normalized magnitude (relative to fundamental)
    pub fn normalized(&self, fundamental_mag: Value) -> Value {
        if fundamental_mag > 1e-15 {
            self.magnitude / fundamental_mag * 100.0
        } else {
            0.0
        }
    }

    /// Get magnitude in dB relative to fundamental
    pub fn db(&self, fundamental_mag: Value) -> Value {
        if fundamental_mag > 1e-15 && self.magnitude > 1e-15 {
            20.0 * (self.magnitude / fundamental_mag).log10()
        } else {
            -200.0
        }
    }
}

/// Fourier analysis result
#[derive(Debug, Clone)]
pub struct FourierResult {
    /// Fundamental frequency analyzed
    pub fundamental_freq: Value,
    /// DC component
    pub dc_component: Value,
    /// All harmonic components (including DC and fundamental)
    pub harmonics: Vec<HarmonicComponent>,
    /// Total Harmonic Distortion (%)
    pub thd: Value,
}

impl FourierResult {
    /// Create empty result
    fn empty(num_harmonics: usize) -> Self {
        Self {
            fundamental_freq: 0.0,
            dc_component: 0.0,
            harmonics: vec![
                HarmonicComponent {
                    harmonic_number: 0,
                    frequency: 0.0,
                    magnitude: 0.0,
                    phase: 0.0,
                };
                num_harmonics + 1
            ],
            thd: 0.0,
        }
    }

    /// Get fundamental component
    pub fn fundamental(&self) -> Option<&HarmonicComponent> {
        self.harmonics.get(1)
    }

    /// Get specific harmonic
    pub fn harmonic(&self, n: usize) -> Option<&HarmonicComponent> {
        self.harmonics.get(n)
    }

    /// Get THD in dB
    pub fn thd_db(&self) -> Value {
        if self.thd > 0.0 {
            20.0 * (self.thd / 100.0).log10()
        } else {
            -200.0
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fourier_config() {
        let config = FourierConfig::new(1000.0);

        assert_eq!(config.fundamental_freq, 1000.0);
        assert_eq!(config.period(), 0.001);
        assert_eq!(config.num_harmonics, 9);
    }

    #[test]
    fn test_pure_sine() {
        let config = FourierConfig::new(1000.0).with_harmonics(5);
        let analysis = FourierAnalysis::new(config);

        // Generate 1kHz sine wave
        let num_points = 1000;
        let duration = 0.002; // 2 periods
        let time: Vec<Value> = (0..num_points)
            .map(|i| i as f64 * duration / (num_points - 1) as f64)
            .collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| (2.0 * PI * 1000.0 * t).sin())
            .collect();

        let result = analysis.analyze(&time, &values);

        // DC should be ~0
        assert!(result.dc_component.abs() < 0.01);

        // Fundamental should be ~1.0
        let fund = result.fundamental().unwrap();
        assert!((fund.magnitude - 1.0).abs() < 0.05);

        // THD should be very low (pure sine)
        assert!(result.thd < 1.0);
    }

    #[test]
    fn test_square_wave_thd() {
        let config = FourierConfig::new(1000.0).with_harmonics(9);
        let analysis = FourierAnalysis::new(config);

        // Generate square wave
        let num_points = 2000;
        let duration = 0.002;
        let time: Vec<Value> = (0..num_points)
            .map(|i| i as f64 * duration / (num_points - 1) as f64)
            .collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| {
                let phase = (t * 1000.0).fract();
                if phase < 0.5 { 1.0 } else { -1.0 }
            })
            .collect();

        let result = analysis.analyze(&time, &values);

        // Square wave has significant odd harmonics -> high THD
        assert!(result.thd > 40.0); // Should be ~48% for ideal square
    }

    #[test]
    fn test_dc_component() {
        let config = FourierConfig::new(1000.0);
        let analysis = FourierAnalysis::new(config);

        // Generate sine with DC offset
        let num_points = 1000;
        let duration = 0.001;
        let dc_offset = 2.5;
        let time: Vec<Value> = (0..num_points)
            .map(|i| i as f64 * duration / (num_points - 1) as f64)
            .collect();
        let values: Vec<Value> = time
            .iter()
            .map(|&t| dc_offset + (2.0 * PI * 1000.0 * t).sin())
            .collect();

        let result = analysis.analyze(&time, &values);

        // DC component should be ~2.5
        assert!((result.dc_component - dc_offset).abs() < 0.1);
    }

    #[test]
    fn test_harmonic_db() {
        let h = HarmonicComponent {
            harmonic_number: 2,
            frequency: 2000.0,
            magnitude: 0.1,
            phase: 0.0,
        };

        // 0.1 relative to 1.0 = -20dB
        let db = h.db(1.0);
        assert!((db - (-20.0)).abs() < 0.1);
    }

    #[test]
    fn test_thd_db() {
        let result = FourierResult {
            fundamental_freq: 1000.0,
            dc_component: 0.0,
            harmonics: vec![],
            thd: 10.0, // 10%
        };

        // 10% = 0.1 -> 20*log10(0.1) = -20dB
        let thd_db = result.thd_db();
        assert!((thd_db - (-20.0)).abs() < 0.1);
    }

    #[test]
    fn test_empty_input() {
        let config = FourierConfig::new(1000.0);
        let analysis = FourierAnalysis::new(config);

        let result = analysis.analyze(&[], &[]);

        assert_eq!(result.thd, 0.0);
        assert_eq!(result.dc_component, 0.0);
    }
}
