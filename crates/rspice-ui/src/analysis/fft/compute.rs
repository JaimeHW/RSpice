//! FFT Analysis Module
//!
//! Provides FFT computation for frequency-domain analysis of waveforms.

use super::data::FftData;
use super::pipeline::{MIN_FFT_SAMPLES, prepare_fft_input};
use super::window::WindowFunction;

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
        window,
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
