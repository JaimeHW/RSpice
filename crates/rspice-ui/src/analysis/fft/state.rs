//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftBuildError, FftData, SpectrumAnalysis, SpectrumNormalization};
use super::window::WindowFunction;
use std::sync::Arc;

mod data_ops;
mod modes;
mod view;

pub use modes::InputFidelity;
const DEFAULT_MANUAL_SAMPLE_COUNT: usize = 4096;

#[derive(Debug, Clone)]
pub struct FftSourceCache {
    pub name: String,
    pub samples: Arc<Vec<f64>>,
    pub sample_rate: f64,
}

// =============================================================================
// FFT State
// =============================================================================

/// Complete FFT viewer state
#[derive(Debug, Clone)]
pub struct FftState {
    /// FFT data
    pub data: Option<FftData>,
    /// Analysis results
    pub analysis: Option<SpectrumAnalysis>,
    /// Cached source used to derive current FFT data.
    pub source_cache: Option<FftSourceCache>,
    /// Most recent typed construction failure, cleared by a successful build.
    pub last_error: Option<FftBuildError>,
    /// User-selected source trace name preference.
    pub selected_source: Option<String>,
    /// Amplitude normalization mode for FFT magnitudes.
    pub normalization: SpectrumNormalization,
    /// Window function
    pub window: WindowFunction,
    /// Input preparation fidelity policy.
    pub input_fidelity: InputFidelity,
    /// Auto-select full source time range for FFT.
    pub time_window_auto: bool,
    /// Manual FFT time-window start.
    pub time_window_start: f64,
    /// Manual FFT time-window end.
    pub time_window_end: f64,
    /// Auto-select FFT sample count from source fidelity policy.
    pub sample_count_auto: bool,
    /// Manual FFT sample count target.
    pub sample_count: usize,
    /// Number of harmonics to analyze
    pub num_harmonics: usize,
    /// Magnitude axis min (dB)
    pub mag_min: f64,
    /// Magnitude axis max (dB)
    pub mag_max: f64,
    /// Auto-scale magnitude
    pub mag_auto: bool,
    /// Frequency min (Hz)
    pub freq_min: f64,
    /// Frequency max (Hz)
    pub freq_max: f64,
    /// Auto-scale frequency
    pub freq_auto: bool,
    /// Interactive marker frequencies (Hz) placed by user.
    pub marker_frequencies: Vec<f64>,
    /// Runtime spectrum revision for display caches.
    spectrum_revision: u64,
}

impl Default for FftState {
    fn default() -> Self {
        Self {
            data: None,
            analysis: None,
            source_cache: None,
            last_error: None,
            selected_source: None,
            normalization: SpectrumNormalization::Rms,
            window: WindowFunction::Hanning,
            input_fidelity: InputFidelity::Reference,
            time_window_auto: true,
            time_window_start: 0.0,
            time_window_end: 0.0,
            sample_count_auto: true,
            sample_count: DEFAULT_MANUAL_SAMPLE_COUNT,
            num_harmonics: 10,
            mag_min: -120.0,
            mag_max: 0.0,
            mag_auto: true,
            freq_min: 0.0,
            freq_max: 1000.0,
            freq_auto: true,
            marker_frequencies: Vec::new(),
            spectrum_revision: 0,
        }
    }
}
