//! FFT Analysis Module
//!
//! Provides FFT computation for frequency-domain analysis of waveforms.

use rustfft::{num_complex::Complex, FftPlanner};

/// Window function types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowFunction {
    /// No windowing (rectangular)
    #[default]
    Rectangular,
    /// Hanning window
    Hanning,
    /// Hamming window
    Hamming,
    /// Blackman window
    Blackman,
}

impl WindowFunction {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            WindowFunction::Rectangular => "Rectangular",
            WindowFunction::Hanning => "Hanning",
            WindowFunction::Hamming => "Hamming",
            WindowFunction::Blackman => "Blackman",
        }
    }

    /// Apply window function to a sample at position i of n total samples
    pub fn apply(&self, i: usize, n: usize) -> f64 {
        let i = i as f64;
        let n = n as f64;

        match self {
            WindowFunction::Rectangular => 1.0,
            WindowFunction::Hanning => {
                0.5 * (1.0 - (2.0 * std::f64::consts::PI * i / (n - 1.0)).cos())
            }
            WindowFunction::Hamming => {
                0.54 - 0.46 * (2.0 * std::f64::consts::PI * i / (n - 1.0)).cos()
            }
            WindowFunction::Blackman => {
                let a0 = 0.42;
                let a1 = 0.5;
                let a2 = 0.08;
                let pi2 = 2.0 * std::f64::consts::PI;
                a0 - a1 * (pi2 * i / (n - 1.0)).cos() + a2 * (2.0 * pi2 * i / (n - 1.0)).cos()
            }
        }
    }
}

/// FFT result containing frequency and magnitude data
#[derive(Debug, Clone)]
pub struct FftResult {
    /// Frequency values (Hz)
    pub frequencies: Vec<f64>,
    /// Magnitude values (typically in dB)
    pub magnitudes: Vec<f64>,
    /// Phase values (radians)
    pub phases: Vec<f64>,
    /// Sample rate used
    pub sample_rate: f64,
    /// Window function used
    pub window: WindowFunction,
}

impl FftResult {
    /// Get magnitude in dB (20 * log10(magnitude))
    pub fn magnitude_db(&self) -> Vec<f64> {
        self.magnitudes
            .iter()
            .map(|m| {
                if *m > 1e-15 {
                    20.0 * m.log10()
                } else {
                    -300.0 // Floor at -300 dB
                }
            })
            .collect()
    }

    /// Find peak frequency
    pub fn peak_frequency(&self) -> Option<(f64, f64)> {
        let (idx, max_mag) = self
            .magnitudes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;
        Some((self.frequencies[idx], *max_mag))
    }
}

/// Compute FFT of time-domain data
///
/// # Arguments
/// * `time` - Time values (x-axis)
/// * `values` - Signal values (y-axis)
/// * `window` - Window function to apply
///
/// # Returns
/// FFT result with frequency and magnitude data
pub fn compute_fft(time: &[f64], values: &[f64], window: WindowFunction) -> Option<FftResult> {
    if time.len() < 4 || time.len() != values.len() {
        return None;
    }

    let n = values.len();

    // Calculate sample rate from time data
    let dt = (time.last()? - time.first()?) / (n - 1) as f64;
    if dt <= 0.0 {
        return None;
    }
    let sample_rate = 1.0 / dt;

    // Apply window and convert to complex
    let mut buffer: Vec<Complex<f64>> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let w = window.apply(i, n);
            Complex::new(v * w, 0.0)
        })
        .collect();

    // Compute FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buffer);

    // Extract frequencies, magnitudes, and phases
    // Only take the first half (positive frequencies) due to symmetry
    let half_n = n / 2;
    let freq_resolution = sample_rate / n as f64;

    let frequencies: Vec<f64> = (0..half_n).map(|i| i as f64 * freq_resolution).collect();

    let scale = 2.0 / n as f64; // Normalization factor

    let magnitudes: Vec<f64> = buffer[..half_n].iter().map(|c| c.norm() * scale).collect();

    let phases: Vec<f64> = buffer[..half_n].iter().map(|c| c.arg()).collect();

    Some(FftResult {
        frequencies,
        magnitudes,
        phases,
        sample_rate,
        window,
    })
}

/// Compute power spectral density (magnitude squared)
pub fn compute_psd(time: &[f64], values: &[f64], window: WindowFunction) -> Option<FftResult> {
    let mut result = compute_fft(time, values, window)?;

    // Convert to power (magnitude squared)
    result.magnitudes = result.magnitudes.iter().map(|m| m * m).collect();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_sine_wave() {
        // Generate 1kHz sine wave sampled at 10kHz
        let n = 1024;
        let sample_rate = 10000.0;
        let freq = 1000.0;

        let time: Vec<f64> = (0..n).map(|i| i as f64 / sample_rate).collect();
        let values: Vec<f64> = time
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * freq * t).sin())
            .collect();

        let result = compute_fft(&time, &values, WindowFunction::Hanning).unwrap();

        // Peak should be around 1kHz
        let (peak_freq, _) = result.peak_frequency().unwrap();
        assert!((peak_freq - freq).abs() < sample_rate / n as f64 * 2.0);
    }

    #[test]
    fn test_window_functions() {
        // Window functions should be 1.0 at center for symmetric windows
        let n = 100;
        let mid = n / 2;

        assert!((WindowFunction::Hanning.apply(mid, n) - 1.0).abs() < 0.01);
        assert!((WindowFunction::Hamming.apply(mid, n) - 1.0).abs() < 0.01);
    }
}
