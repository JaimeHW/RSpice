//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
use super::pipeline::{
    FftInputOptions, FftInputPolicy, FftTimeWindow, MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES,
    PreparedFftInput,
};
use super::window::WindowFunction;
use std::sync::Arc;

// =============================================================================
// Scale Mode
// =============================================================================

/// Magnitude scale mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MagnitudeScale {
    /// dB scale (20 * log10)
    #[default]
    DB,
    /// dBc relative to fundamental level
    DBc,
    /// Linear scale
    Linear,
    /// dBm (power into 50Ω)
    DBm,
}

impl MagnitudeScale {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DB => "dB",
            Self::DBc => "dBc",
            Self::Linear => "Linear",
            Self::DBm => "dBm",
        }
    }

    /// All modes
    pub fn all() -> &'static [MagnitudeScale] {
        &[Self::DB, Self::DBc, Self::Linear, Self::DBm]
    }
}

/// Frequency axis mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FrequencyScale {
    /// Linear frequency axis
    #[default]
    Linear,
    /// Logarithmic frequency axis
    Log,
}

impl FrequencyScale {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::Log => "Log",
        }
    }

    /// All modes
    pub fn all() -> &'static [FrequencyScale] {
        &[Self::Linear, Self::Log]
    }
}

/// FFT input fidelity mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputFidelity {
    /// Preserve source detail for analysis-grade spectra (default).
    #[default]
    Reference,
    /// Enforce capped point count for faster interaction on large datasets.
    Interactive,
}

impl InputFidelity {
    /// Display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Reference => "Reference",
            Self::Interactive => "Interactive",
        }
    }

    /// All modes.
    pub fn all() -> &'static [InputFidelity] {
        &[Self::Reference, Self::Interactive]
    }

    /// Pipeline policy for this fidelity.
    pub fn input_policy(&self) -> FftInputPolicy {
        match self {
            Self::Reference => FftInputPolicy::reference(),
            Self::Interactive => FftInputPolicy::interactive_default(),
        }
    }
}

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

impl FftState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load FFT data and analyze
    pub fn load_data(&mut self, mut data: FftData) {
        data.convert_normalization(self.normalization);
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.source_cache = None;
        self.mark_spectrum_changed();
        self.update_auto_scale();
    }

    /// Load prepared uniformly sampled source and compute FFT using current settings.
    pub fn load_prepared_input(&mut self, input: PreparedFftInput) {
        if self.selected_source.is_none() {
            self.selected_source = Some(input.name.clone());
        }
        self.source_cache = Some(FftSourceCache {
            name: input.name,
            samples: Arc::from(input.samples),
            sample_rate: input.sample_rate,
            original_count: input.original_count,
            decimation_factor: input.decimation_factor,
        });
        self.sync_sample_count_control_value();
        self.recompute_from_source();
    }

    /// Select preferred source trace name.
    pub fn set_selected_source(&mut self, source_name: Option<String>) {
        self.selected_source = source_name;
    }

    /// Set FFT input fidelity mode.
    pub fn set_input_fidelity(&mut self, input_fidelity: InputFidelity) {
        self.input_fidelity = input_fidelity;
    }

    /// Active FFT input pipeline policy.
    pub fn input_policy(&self) -> FftInputPolicy {
        self.input_fidelity.input_policy()
    }

    /// Build pipeline input options for a source timeline.
    pub fn input_options_for_waveform(&self, source_time: &[f64]) -> FftInputOptions {
        self.input_options_for_bounds(finite_time_bounds(source_time))
    }

    /// Build pipeline input options from source bounds.
    pub fn input_options_for_bounds(&self, source_bounds: Option<(f64, f64)>) -> FftInputOptions {
        let time_window = if self.time_window_auto {
            None
        } else if let Some((min_t, max_t)) = source_bounds {
            let (mut start, mut end) =
                if self.time_window_start.is_finite() && self.time_window_end.is_finite() {
                    (
                        self.time_window_start.clamp(min_t, max_t),
                        self.time_window_end.clamp(min_t, max_t),
                    )
                } else {
                    (min_t, max_t)
                };
            if end <= start {
                start = min_t;
                end = max_t;
            }
            if end > start {
                Some(FftTimeWindow::new(start, end))
            } else {
                None
            }
        } else {
            None
        };

        let target_samples = if self.sample_count_auto {
            None
        } else {
            Some(
                self.sample_count
                    .clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS),
            )
        };

        FftInputOptions::with_policy(self.input_policy())
            .with_time_window(time_window)
            .with_target_samples(target_samples)
    }

    /// Keep the UI `N` control value synchronized with the effective FFT input.
    ///
    /// - In auto mode, mirror the prepared source sample count (when available).
    /// - In manual mode, clamp to valid FFT bounds.
    pub fn sync_sample_count_control_value(&mut self) {
        if self.sample_count_auto {
            if let Some(sample_len) = self
                .source_cache
                .as_ref()
                .map(|source| source.samples.len())
            {
                self.sample_count =
                    sample_len.clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS);
            }
        } else {
            self.sample_count = self
                .sample_count
                .clamp(MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS);
        }
    }

    /// Recompute FFT data from cached source using current window.
    pub fn recompute_from_source(&mut self) {
        let Some(source) = self.source_cache.as_ref() else {
            return;
        };
        let mut data = FftData::from_time_domain(
            &format!("FFT({})", source.name),
            &source.samples,
            source.sample_rate,
            self.window,
        );
        data.convert_normalization(self.normalization);
        if data.is_empty() {
            self.data = None;
            self.analysis = None;
            self.mark_spectrum_changed();
            return;
        }
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.mark_spectrum_changed();
        self.update_auto_scale();
    }

    /// Set amplitude normalization mode.
    ///
    /// This performs an in-place O(N) rescale on loaded bins to avoid expensive
    /// FFT recomputation for simple RMS/Peak toggles.
    pub fn set_normalization(&mut self, normalization: SpectrumNormalization) {
        if self.normalization == normalization {
            return;
        }
        self.normalization = normalization;

        if let Some(data) = self.data.as_mut() {
            data.convert_normalization(normalization);
            self.recompute_analysis();
            self.mark_spectrum_changed();
            self.update_auto_scale();
            return;
        }

        if self.source_cache.is_some() {
            self.recompute_from_source();
        }
    }

    /// Recompute scalar analysis from currently loaded spectrum data.
    pub fn recompute_analysis(&mut self) {
        if let Some(data) = self.data.as_ref() {
            self.analysis = Some(SpectrumAnalysis::analyze(data, self.num_harmonics));
        } else {
            self.analysis = None;
        }
    }

    /// Clear data
    pub fn clear(&mut self) {
        self.data = None;
        self.analysis = None;
        self.source_cache = None;
        self.clear_markers();
        self.mark_spectrum_changed();
    }

    /// Has data?
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// Update auto-scale ranges
    pub fn update_auto_scale(&mut self) {
        if let Some(ref data) = self.data {
            if self.freq_auto
                && let Some((min, max)) = data.frequency_range()
            {
                match self.freq_scale {
                    FrequencyScale::Linear => {
                        self.freq_min = min;
                        self.freq_max = max;
                    }
                    FrequencyScale::Log => {
                        self.freq_min = first_positive_frequency(data).unwrap_or(1e-12);
                        self.freq_max = max.max(self.freq_min * 1.01);
                    }
                }
            }

            if self.mag_auto {
                let mut min_value = f64::INFINITY;
                let mut max_value = f64::NEG_INFINITY;
                let mut has_finite = false;
                for point in &data.points {
                    let value = self.display_magnitude(point);
                    if value.is_finite() {
                        has_finite = true;
                        min_value = min_value.min(value);
                        max_value = max_value.max(value);
                    }
                }

                if has_finite {
                    let span = (max_value - min_value).abs();
                    let padding = if span > 0.0 { span * 0.1 } else { 1.0 };

                    match self.mag_scale {
                        MagnitudeScale::Linear => {
                            self.mag_min = (min_value - padding).max(0.0);
                            self.mag_max = (max_value + padding).max(self.mag_min + 1e-9);
                        }
                        MagnitudeScale::DB | MagnitudeScale::DBm | MagnitudeScale::DBc => {
                            self.mag_min = (min_value - padding).floor().max(-300.0);
                            self.mag_max = (max_value + padding).ceil().min(120.0);
                        }
                    }
                }
            }
        }
    }

    /// Convert a spectrum point to currently selected display magnitude.
    pub fn display_magnitude(&self, point: &super::data::FftPoint) -> f64 {
        match self.mag_scale {
            MagnitudeScale::DB => point.magnitude_db(),
            MagnitudeScale::DBm => point.magnitude_dbm(self.z0),
            MagnitudeScale::Linear => point.magnitude,
            MagnitudeScale::DBc => {
                let fundamental_db = self
                    .analysis
                    .as_ref()
                    .and_then(|analysis| analysis.fundamental_db)
                    .unwrap_or(0.0);
                point.magnitude_db() - fundamental_db
            }
        }
    }

    /// Set window function
    pub fn set_window(&mut self, window: WindowFunction) {
        if self.window == window {
            return;
        }
        self.window = window;
        self.recompute_from_source();
    }

    /// Set magnitude scale
    pub fn set_mag_scale(&mut self, scale: MagnitudeScale) {
        if self.mag_scale == scale {
            return;
        }
        self.mag_scale = scale;
        if self.mag_auto {
            self.update_auto_scale();
        }
    }

    /// Set frequency scale
    pub fn set_freq_scale(&mut self, scale: FrequencyScale) {
        if self.freq_scale == scale {
            return;
        }
        self.freq_scale = scale;
        if self.freq_auto {
            self.update_auto_scale();
        } else if self.freq_scale == FrequencyScale::Log {
            self.freq_min = self.freq_min.max(1e-12);
            if self.freq_max <= self.freq_min {
                self.freq_max = self.freq_min * 1.01;
            }
        }
    }

    /// Set number of harmonics for distortion analysis.
    pub fn set_num_harmonics(&mut self, num_harmonics: usize) {
        self.num_harmonics = num_harmonics.max(1);
        self.recompute_analysis();
    }

    pub fn ensure_peak_cache(&mut self) {
        let Some(data) = self.data.as_ref() else {
            self.peak_cache = PeakCache::default();
            return;
        };
        let threshold_bits = self.peak_threshold_db.to_bits();
        if self.peak_cache.spectrum_revision == self.spectrum_revision
            && self.peak_cache.threshold_bits == threshold_bits
        {
            return;
        }

        self.peak_cache.spectrum_revision = self.spectrum_revision;
        self.peak_cache.threshold_bits = threshold_bits;
        self.peak_cache.peak_indices = data.find_peak_indices(self.peak_threshold_db);
    }

    pub fn cached_peak_indices(&self) -> &[usize] {
        &self.peak_cache.peak_indices
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle peaks
    pub fn toggle_peaks(&mut self) {
        self.show_peaks = !self.show_peaks;
    }

    /// Toggle harmonics
    pub fn toggle_harmonics(&mut self) {
        self.show_harmonics = !self.show_harmonics;
    }

    /// Add a user marker frequency. Maintains sorted order and bounded count.
    pub fn add_marker(&mut self, frequency_hz: f64) {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return;
        }
        if self
            .marker_frequencies
            .iter()
            .any(|f| (*f - frequency_hz).abs() <= MARKER_MERGE_EPS_HZ)
        {
            return;
        }
        self.marker_frequencies.push(frequency_hz);
        self.marker_frequencies.sort_by(|a, b| a.total_cmp(b));
        if self.marker_frequencies.len() > MAX_USER_MARKERS {
            self.marker_frequencies.remove(0);
        }
    }

    /// Remove the nearest marker within a tolerance window.
    pub fn remove_nearest_marker(&mut self, frequency_hz: f64, tolerance_hz: f64) -> bool {
        if !frequency_hz.is_finite() || !tolerance_hz.is_finite() || tolerance_hz < 0.0 {
            return false;
        }
        let Some((idx, dist)) = self
            .marker_frequencies
            .iter()
            .enumerate()
            .map(|(idx, marker)| (idx, (*marker - frequency_hz).abs()))
            .min_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return false;
        };
        if dist <= tolerance_hz {
            self.marker_frequencies.remove(idx);
            true
        } else {
            false
        }
    }

    /// Number of user marker slots with assigned frequencies.
    pub fn marker_count(&self) -> usize {
        self.marker_frequencies.len()
    }

    /// Clear all user markers.
    pub fn clear_markers(&mut self) {
        self.marker_frequencies.clear();
    }

    /// Remove marker at explicit index.
    pub fn remove_marker_at(&mut self, index: usize) -> bool {
        if index >= self.marker_frequencies.len() {
            return false;
        }
        self.marker_frequencies.remove(index);
        true
    }

    /// Get fundamental frequency
    pub fn fundamental_freq(&self) -> Option<f64> {
        self.analysis.as_ref()?.fundamental_frequency
    }

    /// Get THD
    pub fn thd_percent(&self) -> Option<f64> {
        self.analysis.as_ref()?.thd_percent
    }

    /// Get SFDR
    pub fn sfdr_db(&self) -> Option<f64> {
        self.analysis.as_ref()?.sfdr_db
    }

    /// Get SNR
    pub fn snr_db(&self) -> Option<f64> {
        self.analysis.as_ref()?.snr_db
    }

    fn mark_spectrum_changed(&mut self) {
        self.spectrum_revision = self.spectrum_revision.wrapping_add(1);
        self.peak_cache = PeakCache::default();
    }
}

fn first_positive_frequency(data: &FftData) -> Option<f64> {
    data.points
        .iter()
        .map(|p| p.frequency)
        .find(|freq| freq.is_finite() && *freq > 0.0)
}

fn finite_time_bounds(time: &[f64]) -> Option<(f64, f64)> {
    let start = time.iter().copied().find(|t| t.is_finite())?;
    let end = time.iter().copied().rfind(|t| t.is_finite())?;
    if end > start {
        Some((start, end))
    } else {
        None
    }
}

// =============================================================================
// Tests
// =============================================================================

