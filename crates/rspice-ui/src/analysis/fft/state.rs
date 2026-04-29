//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
use super::window::WindowFunction;
use std::sync::Arc;

mod data_ops;
mod markers;
mod metrics;
mod modes;
mod view;

pub use modes::{FrequencyScale, InputFidelity, MagnitudeScale};
const DEFAULT_MANUAL_SAMPLE_COUNT: usize = 4096;
const MAX_USER_MARKERS: usize = 16;
const MARKER_MERGE_EPS_HZ: f64 = 1e-12;

#[derive(Debug, Clone, Default)]
struct PeakCache {
    spectrum_revision: u64,
    threshold_bits: u64,
    peak_indices: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct FftSourceCache {
    pub name: String,
    pub samples: Arc<[f64]>,
    pub sample_rate: f64,
    pub original_count: usize,
    pub decimation_factor: usize,
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
    /// User-selected source trace name preference.
    pub selected_source: Option<String>,
    /// Amplitude normalization mode for FFT magnitudes.
    pub normalization: SpectrumNormalization,
    /// Window function
    pub window: WindowFunction,
    /// Magnitude scale
    pub mag_scale: MagnitudeScale,
    /// Frequency scale
    pub freq_scale: FrequencyScale,
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
    /// Show grid
    pub show_grid: bool,
    /// Show peaks
    pub show_peaks: bool,
    /// Show harmonics
    pub show_harmonics: bool,
    /// Peak threshold (dB)
    pub peak_threshold_db: f64,
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
    /// Reference impedance for dBm
    pub z0: f64,
    /// Interactive marker frequencies (Hz) placed by user.
    pub marker_frequencies: Vec<f64>,
    /// Optional user-resized right info pane width in pixels. `None` means auto-fit.
    pub info_pane_width: Option<f32>,
    /// Runtime auto-fit width hint captured from rendered content.
    pub info_pane_auto_width_hint: f32,
    /// Runtime spectrum revision for display caches.
    spectrum_revision: u64,
    /// Cached local-maximum bins for the current threshold and spectrum revision.
    peak_cache: PeakCache,
}

impl Default for FftState {
    fn default() -> Self {
        Self {
            data: None,
            analysis: None,
            source_cache: None,
            selected_source: None,
            normalization: SpectrumNormalization::Rms,
            window: WindowFunction::Hanning,
            mag_scale: MagnitudeScale::DB,
            freq_scale: FrequencyScale::Linear,
            input_fidelity: InputFidelity::Reference,
            time_window_auto: true,
            time_window_start: 0.0,
            time_window_end: 0.0,
            sample_count_auto: true,
            sample_count: DEFAULT_MANUAL_SAMPLE_COUNT,
            show_grid: true,
            show_peaks: true,
            show_harmonics: true,
            peak_threshold_db: -60.0,
            num_harmonics: 10,
            mag_min: -120.0,
            mag_max: 0.0,
            mag_auto: true,
            freq_min: 0.0,
            freq_max: 1000.0,
            freq_auto: true,
            z0: 50.0,
            marker_frequencies: Vec::new(),
            info_pane_width: None,
            info_pane_auto_width_hint: 0.0,
            spectrum_revision: 0,
            peak_cache: PeakCache::default(),
        }
    }
}
