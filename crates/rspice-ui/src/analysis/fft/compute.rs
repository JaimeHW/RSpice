//! FFT Analysis Module
//!
//! Provides FFT computation for frequency-domain analysis of waveforms.

use super::data::FftData;
use super::pipeline::{MIN_FFT_SAMPLES, prepare_fft_input};

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
#[derive(Debug, Clone, PartialEq)]
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
            .filter(|(_, mag)| mag.is_finite())
            .max_by(|(_, a), (_, b)| a.total_cmp(b))?;
        Some((*self.frequencies.get(idx)?, *max_mag))
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
    let prepared = prepare_fft_input(
        "compute_fft",
        time,
        values,
        values.len().max(MIN_FFT_SAMPLES),
    )?;
    let fft = FftData::from_time_domain(
        "compute_fft",
        &prepared.samples,
        prepared.sample_rate,
        map_window(window),
    );
    if fft.is_empty() {
        return None;
    }

    let mut frequencies = Vec::with_capacity(fft.points.len());
    let mut magnitudes = Vec::with_capacity(fft.points.len());
    let mut phases = Vec::with_capacity(fft.points.len());
    for point in fft.points {
        frequencies.push(point.frequency);
        magnitudes.push(point.magnitude);
        phases.push(point.phase);
    }

    Some(FftResult {
        frequencies,
        magnitudes,
        phases,
        sample_rate: prepared.sample_rate,
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

fn map_window(window: WindowFunction) -> super::window::WindowFunction {
    match window {
        WindowFunction::Rectangular => super::window::WindowFunction::Rectangular,
        WindowFunction::Hanning => super::window::WindowFunction::Hanning,
        WindowFunction::Hamming => super::window::WindowFunction::Hamming,
        WindowFunction::Blackman => super::window::WindowFunction::Blackman,
    }
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

    #[test]
    fn test_peak_frequency_ignores_non_finite_magnitudes() {
        let result = FftResult {
            frequencies: vec![0.0, 1.0, 2.0, 3.0],
            magnitudes: vec![f64::NAN, f64::INFINITY, 0.5, 0.8],
            phases: vec![0.0; 4],
            sample_rate: 4.0,
            window: WindowFunction::Rectangular,
        };

        let (freq, mag) = result.peak_frequency().unwrap();
        assert_eq!(freq, 3.0);
        assert_eq!(mag, 0.8);
    }

    #[test]
    fn test_peak_frequency_all_non_finite_returns_none() {
        let result = FftResult {
            frequencies: vec![0.0, 1.0, 2.0],
            magnitudes: vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY],
            phases: vec![0.0; 3],
            sample_rate: 3.0,
            window: WindowFunction::Rectangular,
        };

        assert!(result.peak_frequency().is_none());
    }

    #[test]
    fn test_compute_fft_includes_nyquist_for_even_length() {
        let n = 1024usize;
        let sample_rate = 10_000.0;
        let time: Vec<f64> = (0..n).map(|i| i as f64 / sample_rate).collect();
        let values: Vec<f64> = time
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
            .collect();

        let result = compute_fft(&time, &values, WindowFunction::Hanning).expect("fft");
        assert_eq!(result.frequencies.len(), n / 2 + 1);
        assert!(
            (result.frequencies.last().copied().unwrap_or(0.0) - sample_rate * 0.5).abs() < 1e-9
        );
    }

    #[test]
    fn test_compute_fft_handles_nonuniform_timeline() {
        let n = 2048usize;
        let sample_rate = 50_000.0;
        let mut time = Vec::with_capacity(n);
        let mut t = 0.0;
        for i in 0..n {
            let jitter = if i % 3 == 0 { 0.85 } else { 1.15 };
            t += jitter / sample_rate;
            time.push(t);
        }
        let values: Vec<f64> = time
            .iter()
            .map(|tt| (2.0 * std::f64::consts::PI * 5000.0 * tt).sin())
            .collect();

        let result = compute_fft(&time, &values, WindowFunction::Hanning);
        assert!(result.is_some());
        assert!(
            result
                .as_ref()
                .map(|r| r.sample_rate.is_finite() && r.sample_rate > 0.0)
                .unwrap_or(false)
        );
    }
}
