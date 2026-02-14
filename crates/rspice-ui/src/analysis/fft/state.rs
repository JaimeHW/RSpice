//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
use super::pipeline::{
    FftInputOptions, FftInputPolicy, FftTimeWindow, PreparedFftInput,
    MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES,
};
use super::window::WindowFunction;

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

/// Active FFT user marker slot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MarkerSlot {
    /// Primary user marker.
    #[default]
    M1,
    /// Secondary user marker.
    M2,
}

impl MarkerSlot {
    /// Display name for UI.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::M1 => "M1",
            Self::M2 => "M2",
        }
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

#[derive(Debug, Clone)]
pub struct FftSourceCache {
    pub name: String,
    pub samples: Vec<f64>,
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
    /// Interactive marker frequency (Hz) placed by user.
    pub marker_frequency: Option<f64>,
    /// Secondary interactive marker frequency (Hz) placed by user.
    pub marker_frequency_secondary: Option<f64>,
    /// Active marker slot used by click placement.
    pub active_marker_slot: MarkerSlot,
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
            marker_frequency: None,
            marker_frequency_secondary: None,
            active_marker_slot: MarkerSlot::M1,
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
        self.update_auto_scale();
    }

    /// Load prepared uniformly sampled source and compute FFT using current settings.
    pub fn load_prepared_input(&mut self, input: PreparedFftInput) {
        if self.selected_source.is_none() {
            self.selected_source = Some(input.name.clone());
        }
        self.source_cache = Some(FftSourceCache {
            name: input.name,
            samples: input.samples,
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
            return;
        }
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
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
            if self.freq_auto {
                if let Some((min, max)) = data.frequency_range() {
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
            }

            if self.mag_auto {
                let mut values = Vec::new();
                for point in &data.points {
                    let value = self.display_magnitude(point);
                    if value.is_finite() {
                        values.push(value);
                    }
                }

                if !values.is_empty() {
                    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
                    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let span = (max - min).abs();
                    let padding = if span > 0.0 { span * 0.1 } else { 1.0 };

                    match self.mag_scale {
                        MagnitudeScale::Linear => {
                            self.mag_min = (min - padding).max(0.0);
                            self.mag_max = (max + padding).max(self.mag_min + 1e-9);
                        }
                        MagnitudeScale::DB | MagnitudeScale::DBm | MagnitudeScale::DBc => {
                            self.mag_min = (min - padding).floor().max(-300.0);
                            self.mag_max = (max + padding).ceil().min(120.0);
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

    /// Set marker frequency.
    pub fn set_marker_frequency(&mut self, marker_frequency: Option<f64>) {
        self.set_marker_frequency_for_slot(self.active_marker_slot, marker_frequency);
    }

    /// Set marker frequency in a specific slot.
    pub fn set_marker_frequency_for_slot(
        &mut self,
        slot: MarkerSlot,
        marker_frequency: Option<f64>,
    ) {
        match slot {
            MarkerSlot::M1 => self.marker_frequency = marker_frequency,
            MarkerSlot::M2 => self.marker_frequency_secondary = marker_frequency,
        }
    }

    /// Read marker frequency from a specific slot.
    pub fn marker_frequency_for_slot(&self, slot: MarkerSlot) -> Option<f64> {
        match slot {
            MarkerSlot::M1 => self.marker_frequency,
            MarkerSlot::M2 => self.marker_frequency_secondary,
        }
    }

    /// Set active marker slot used by click placement.
    pub fn set_active_marker_slot(&mut self, slot: MarkerSlot) {
        self.active_marker_slot = slot;
    }

    /// Clear both user markers.
    pub fn clear_markers(&mut self) {
        self.marker_frequency = None;
        self.marker_frequency_secondary = None;
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

#[cfg(test)]
mod tests {
    use super::super::data::FftPoint;
    use super::super::pipeline::PreparedFftInput;
    use super::*;

    // =========================================================================
    // MagnitudeScale Tests
    // =========================================================================

    #[test]
    fn test_mag_scale_default() {
        let scale = MagnitudeScale::default();
        assert_eq!(scale, MagnitudeScale::DB);
    }

    #[test]
    fn test_mag_scale_names() {
        assert_eq!(MagnitudeScale::DB.display_name(), "dB");
        assert_eq!(MagnitudeScale::DBc.display_name(), "dBc");
        assert_eq!(MagnitudeScale::DBm.display_name(), "dBm");
    }

    #[test]
    fn test_mag_scale_all() {
        let all = MagnitudeScale::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_marker_slot_display_names() {
        assert_eq!(MarkerSlot::M1.display_name(), "M1");
        assert_eq!(MarkerSlot::M2.display_name(), "M2");
    }

    // =========================================================================
    // FrequencyScale Tests
    // =========================================================================

    #[test]
    fn test_freq_scale_default() {
        let scale = FrequencyScale::default();
        assert_eq!(scale, FrequencyScale::Linear);
    }

    #[test]
    fn test_freq_scale_names() {
        assert_eq!(FrequencyScale::Log.display_name(), "Log");
    }

    #[test]
    fn test_freq_scale_all() {
        let all = FrequencyScale::all();
        assert_eq!(all.len(), 2);
    }

    // =========================================================================
    // InputFidelity Tests
    // =========================================================================

    #[test]
    fn test_input_fidelity_default() {
        let fidelity = InputFidelity::default();
        assert_eq!(fidelity, InputFidelity::Reference);
    }

    #[test]
    fn test_input_fidelity_names() {
        assert_eq!(InputFidelity::Reference.display_name(), "Reference");
        assert_eq!(InputFidelity::Interactive.display_name(), "Interactive");
    }

    #[test]
    fn test_input_fidelity_all() {
        let all = InputFidelity::all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_input_fidelity_policy_mapping() {
        assert_eq!(
            InputFidelity::Reference.input_policy(),
            FftInputPolicy::Reference
        );
        assert_eq!(
            InputFidelity::Interactive.input_policy(),
            FftInputPolicy::interactive_default()
        );
    }

    // =========================================================================
    // FftState Tests
    // =========================================================================

    #[test]
    fn test_state_new() {
        let state = FftState::new();
        assert!(state.is_empty());
        assert!(state.show_grid);
        assert!(state.show_peaks);
    }

    #[test]
    fn test_state_default() {
        let state = FftState::default();
        assert_eq!(state.window, WindowFunction::Hanning);
        assert_eq!(state.normalization, SpectrumNormalization::Rms);
        assert_eq!(state.input_fidelity, InputFidelity::Reference);
        assert!(state.time_window_auto);
        assert!(state.sample_count_auto);
        assert_eq!(state.sample_count, DEFAULT_MANUAL_SAMPLE_COUNT);
        assert!(state.mag_auto);
        assert!(state.freq_auto);
        assert_eq!(state.z0, 50.0);
        assert!(state.marker_frequency.is_none());
        assert!(state.marker_frequency_secondary.is_none());
        assert_eq!(state.active_marker_slot, MarkerSlot::M1);
    }

    #[test]
    fn test_state_load_data() {
        let mut state = FftState::new();
        let mut data = FftData::new("Test");
        data.points = vec![
            FftPoint::new(0.0, 0.1, 0.0),
            FftPoint::new(1000.0, 1.0, 0.0),
        ];
        data.sample_rate = 10000.0;
        data.normalization = SpectrumNormalization::Peak;

        state.load_data(data);

        assert!(state.has_data());
        assert!(!state.is_empty());
        assert!(state.analysis.is_some());
        assert_eq!(
            state.data.as_ref().map(|d| d.normalization),
            Some(SpectrumNormalization::Rms)
        );
    }

    #[test]
    fn test_state_set_normalization_rescales_existing_data() {
        let mut state = FftState::new();
        state.normalization = SpectrumNormalization::Peak;
        let mut data = FftData::new("Tone");
        data.points = vec![
            FftPoint::new(0.0, 0.0, 0.0),
            FftPoint::new(1000.0, 1.0, 0.0),
        ];
        data.sample_rate = 10_000.0;
        data.fft_size = 1024;
        data.normalization = SpectrumNormalization::Peak;
        state.load_data(data);

        let before = state
            .data
            .as_ref()
            .and_then(|d| d.find_peak().map(|(_, p)| p.magnitude_db()))
            .expect("peak before");
        state.set_normalization(SpectrumNormalization::Rms);
        let after = state
            .data
            .as_ref()
            .and_then(|d| d.find_peak().map(|(_, p)| p.magnitude_db()))
            .expect("peak after");

        assert!((before - after - 3.0103).abs() < 0.05);
    }

    #[test]
    fn test_state_clear() {
        let mut state = FftState::new();
        let data = FftData::new("Test");
        state.load_data(data);

        state.clear();

        assert!(state.is_empty());
        assert!(state.analysis.is_none());
    }

    #[test]
    fn test_state_set_window() {
        let mut state = FftState::new();
        state.set_window(WindowFunction::Blackman);
        assert_eq!(state.window, WindowFunction::Blackman);
    }

    #[test]
    fn test_state_set_scales() {
        let mut state = FftState::new();

        state.set_mag_scale(MagnitudeScale::DBm);
        assert_eq!(state.mag_scale, MagnitudeScale::DBm);

        state.set_freq_scale(FrequencyScale::Log);
        assert_eq!(state.freq_scale, FrequencyScale::Log);
    }

    #[test]
    fn test_state_set_input_fidelity_updates_policy() {
        let mut state = FftState::new();
        assert_eq!(state.input_policy(), FftInputPolicy::Reference);

        state.set_input_fidelity(InputFidelity::Interactive);
        assert_eq!(state.input_policy(), FftInputPolicy::interactive_default());
    }

    #[test]
    fn test_state_input_options_auto_uses_policy_only() {
        let mut state = FftState::new();
        state.set_input_fidelity(InputFidelity::Reference);
        state.time_window_auto = true;
        state.sample_count_auto = true;

        let time: Vec<f64> = (0..100).map(|i| i as f64 * 1e-3).collect();
        let options = state.input_options_for_waveform(&time);

        assert_eq!(options.policy, FftInputPolicy::Reference);
        assert!(options.time_window.is_none());
        assert!(options.target_samples.is_none());
    }

    #[test]
    fn test_state_input_options_manual_window_and_samples_are_clamped_to_source_bounds() {
        let mut state = FftState::new();
        state.time_window_auto = false;
        state.time_window_start = -10.0;
        state.time_window_end = 10.0;
        state.sample_count_auto = false;
        state.sample_count = MAX_REFERENCE_RESAMPLE_POINTS * 2;

        let options = state.input_options_for_bounds(Some((0.25, 0.75)));
        let time_window = options.time_window.expect("time window");

        assert!((time_window.start - 0.25).abs() < 1e-12);
        assert!((time_window.end - 0.75).abs() < 1e-12);
        assert_eq!(options.target_samples, Some(MAX_REFERENCE_RESAMPLE_POINTS));
    }

    #[test]
    fn test_state_input_options_manual_invalid_window_falls_back_to_full_bounds() {
        let mut state = FftState::new();
        state.time_window_auto = false;
        state.time_window_start = 0.7;
        state.time_window_end = 0.3;

        let options = state.input_options_for_bounds(Some((0.2, 0.8)));
        let time_window = options.time_window.expect("time window");

        assert!((time_window.start - 0.2).abs() < 1e-12);
        assert!((time_window.end - 0.8).abs() < 1e-12);
    }

    #[test]
    fn test_state_set_freq_scale_log_auto_uses_positive_min() {
        let mut state = FftState::new();
        let mut data = FftData::new("Test");
        data.points = vec![
            FftPoint::new(0.0, 1.0, 0.0),
            FftPoint::new(5.0, 0.8, 0.0),
            FftPoint::new(10.0, 0.6, 0.0),
        ];
        data.sample_rate = 20.0;
        data.fft_size = 4;
        state.load_data(data);

        state.set_freq_scale(FrequencyScale::Log);
        assert_eq!(state.freq_scale, FrequencyScale::Log);
        assert!((state.freq_min - 5.0).abs() < 1e-12);
        assert!(state.freq_max > state.freq_min);
    }

    #[test]
    fn test_state_set_freq_scale_log_manual_clamps_nonpositive_min() {
        let mut state = FftState::new();
        state.freq_auto = false;
        state.freq_min = 0.0;
        state.freq_max = 100.0;

        state.set_freq_scale(FrequencyScale::Log);
        assert!(state.freq_min > 0.0);
        assert!(state.freq_max > state.freq_min);
    }

    #[test]
    fn test_state_toggle_grid() {
        let mut state = FftState::new();
        let initial = state.show_grid;

        state.toggle_grid();
        assert_ne!(state.show_grid, initial);
    }

    #[test]
    fn test_state_toggle_peaks() {
        let mut state = FftState::new();
        let initial = state.show_peaks;

        state.toggle_peaks();
        assert_ne!(state.show_peaks, initial);
    }

    #[test]
    fn test_state_toggle_harmonics() {
        let mut state = FftState::new();
        let initial = state.show_harmonics;

        state.toggle_harmonics();
        assert_ne!(state.show_harmonics, initial);
    }

    #[test]
    fn test_state_marker_slots_track_independent_frequencies() {
        let mut state = FftState::new();
        state.set_active_marker_slot(MarkerSlot::M1);
        state.set_marker_frequency(Some(1_000.0));
        state.set_active_marker_slot(MarkerSlot::M2);
        state.set_marker_frequency(Some(2_500.0));

        assert_eq!(
            state.marker_frequency_for_slot(MarkerSlot::M1),
            Some(1_000.0)
        );
        assert_eq!(
            state.marker_frequency_for_slot(MarkerSlot::M2),
            Some(2_500.0)
        );

        state.clear_markers();
        assert!(state.marker_frequency_for_slot(MarkerSlot::M1).is_none());
        assert!(state.marker_frequency_for_slot(MarkerSlot::M2).is_none());
    }

    #[test]
    fn test_state_empty_measurements() {
        let state = FftState::new();
        assert!(state.fundamental_freq().is_none());
        assert!(state.thd_percent().is_none());
        assert!(state.sfdr_db().is_none());
        assert!(state.snr_db().is_none());
    }

    #[test]
    fn test_state_load_prepared_input_sets_source_and_computes_fft() {
        let mut state = FftState::new();
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0],
            sample_rate: 8_000.0,
            original_count: 8,
            decimation_factor: 1,
        };

        state.load_prepared_input(input);
        assert!(state.data.is_some());
        assert!(state.analysis.is_some());
        assert!(state.source_cache.is_some());
        assert_eq!(state.selected_source.as_deref(), Some("V(out)"));
    }

    #[test]
    fn test_state_load_prepared_input_syncs_sample_count_when_auto_enabled() {
        let mut state = FftState::new();
        state.sample_count_auto = true;
        state.sample_count = 4096;
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: (0..1024).map(|i| (i as f64).sin()).collect(),
            sample_rate: 8_000.0,
            original_count: 1024,
            decimation_factor: 1,
        };

        state.load_prepared_input(input);
        assert_eq!(state.sample_count, 1024);
    }

    #[test]
    fn test_state_load_prepared_input_preserves_manual_sample_count_when_auto_disabled() {
        let mut state = FftState::new();
        state.sample_count_auto = false;
        state.sample_count = 2048;
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: (0..1024).map(|i| (i as f64).sin()).collect(),
            sample_rate: 8_000.0,
            original_count: 1024,
            decimation_factor: 1,
        };

        state.load_prepared_input(input);
        assert_eq!(state.sample_count, 2048);
    }

    #[test]
    fn test_state_load_prepared_input_preserves_existing_selected_source() {
        let mut state = FftState::new();
        state.set_selected_source(Some("trace_key".to_string()));
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0],
            sample_rate: 8_000.0,
            original_count: 8,
            decimation_factor: 1,
        };

        state.load_prepared_input(input);
        assert_eq!(state.selected_source.as_deref(), Some("trace_key"));
    }

    #[test]
    fn test_state_load_prepared_input_honors_rms_normalization() {
        let mut state = FftState::new();
        state.normalization = SpectrumNormalization::Rms;
        let fs = 10_240.0;
        let n = 1024usize;
        let f_sig = 1_000.0;
        let input = PreparedFftInput {
            name: "tone".to_string(),
            samples: (0..n)
                .map(|i| (2.0 * std::f64::consts::PI * f_sig * i as f64 / fs).sin())
                .collect(),
            sample_rate: fs,
            original_count: n,
            decimation_factor: 1,
        };

        state.load_prepared_input(input);
        let peak_db = state
            .data
            .as_ref()
            .and_then(|d| d.find_peak().map(|(_, p)| p.magnitude_db()))
            .expect("peak db");
        assert!((peak_db + 3.0103).abs() < 0.1);
        assert_eq!(
            state.data.as_ref().map(|d| d.normalization),
            Some(SpectrumNormalization::Rms)
        );
    }

    #[test]
    fn test_state_set_window_recomputes_from_cached_source() {
        let mut state = FftState::new();
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: (0..256).map(|i| ((i as f64) * 0.2).sin()).collect(),
            sample_rate: 10_000.0,
            original_count: 256,
            decimation_factor: 1,
        };
        state.load_prepared_input(input);
        let before = state
            .data
            .as_ref()
            .and_then(|d| d.find_peak().map(|(_, p)| p.magnitude))
            .unwrap_or(0.0);

        state.set_window(WindowFunction::FlatTop);
        let after = state
            .data
            .as_ref()
            .and_then(|d| d.find_peak().map(|(_, p)| p.magnitude))
            .unwrap_or(0.0);

        assert_eq!(state.window, WindowFunction::FlatTop);
        assert!((before - after).abs() > 1e-6);
    }

    #[test]
    fn test_state_set_num_harmonics_updates_analysis() {
        let mut state = FftState::new();
        let input = PreparedFftInput {
            name: "V(out)".to_string(),
            samples: (0..1024)
                .map(|i| {
                    let t = i as f64 / 10_000.0;
                    (2.0 * std::f64::consts::PI * 1_000.0 * t).sin()
                        + 0.1 * (2.0 * std::f64::consts::PI * 2_000.0 * t).sin()
                        + 0.05 * (2.0 * std::f64::consts::PI * 3_000.0 * t).sin()
                })
                .collect(),
            sample_rate: 10_000.0,
            original_count: 1024,
            decimation_factor: 1,
        };
        state.load_prepared_input(input);

        state.set_num_harmonics(2);
        let h2_count = state
            .analysis
            .as_ref()
            .map(|a| a.harmonics.len())
            .unwrap_or(0);
        state.set_num_harmonics(10);
        let h10_count = state
            .analysis
            .as_ref()
            .map(|a| a.harmonics.len())
            .unwrap_or(0);
        assert!(h10_count >= h2_count);
    }

    #[test]
    fn test_state_mag_auto_updates_for_linear_scale() {
        let mut state = FftState::new();
        let mut data = FftData::new("Test");
        data.points = vec![
            FftPoint::new(0.0, 0.0, 0.0),
            FftPoint::new(10.0, 0.1, 0.0),
            FftPoint::new(20.0, 2.5, 0.0),
            FftPoint::new(30.0, 1.0, 0.0),
        ];
        data.normalization = SpectrumNormalization::Peak;
        data.sample_rate = 100.0;
        data.fft_size = 8;
        state.load_data(data);

        state.set_mag_scale(MagnitudeScale::Linear);
        // State defaults to RMS normalization, so the loaded Peak spectrum is rescaled.
        assert!(state.mag_max > 1.5);
        assert!(state.mag_min >= 0.0);
    }

    #[test]
    fn test_state_display_magnitude_dbc_uses_fundamental_reference() {
        let mut state = FftState::new();
        let mut data = FftData::new("tone");
        data.points = vec![
            FftPoint::new(0.0, 0.0, 0.0),
            FftPoint::new(1_000.0, 1.0, 0.0),
            FftPoint::new(2_000.0, 0.1, 0.0),
        ];
        data.sample_rate = 10_000.0;
        data.fft_size = 8;
        state.load_data(data);
        state.set_mag_scale(MagnitudeScale::DBc);

        let fundamental = state
            .data
            .as_ref()
            .and_then(|d| d.points.get(1))
            .expect("fundamental point");
        let harmonic = state
            .data
            .as_ref()
            .and_then(|d| d.points.get(2))
            .expect("harmonic point");
        let fund_display = state.display_magnitude(fundamental);
        let harm_display = state.display_magnitude(harmonic);

        assert!(fund_display.abs() < 1e-6);
        assert!(harm_display < -15.0);
    }

    #[test]
    fn test_finite_time_bounds_uses_first_and_last_finite_samples() {
        let time = vec![f64::NAN, 0.1, 0.2, 0.3, f64::INFINITY];
        let bounds = finite_time_bounds(&time).expect("bounds");
        assert!((bounds.0 - 0.1).abs() < 1e-12);
        assert!((bounds.1 - 0.3).abs() < 1e-12);
    }
}
