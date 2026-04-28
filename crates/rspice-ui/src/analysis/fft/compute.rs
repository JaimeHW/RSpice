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
