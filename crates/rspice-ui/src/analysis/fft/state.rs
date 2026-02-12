//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftData, SpectrumAnalysis};
use super::pipeline::PreparedFftInput;
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
            Self::Linear => "Linear",
            Self::DBm => "dBm",
        }
    }

    /// All modes
    pub fn all() -> &'static [MagnitudeScale] {
        &[Self::DB, Self::Linear, Self::DBm]
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
    /// Window function
    pub window: WindowFunction,
    /// Magnitude scale
    pub mag_scale: MagnitudeScale,
    /// Frequency scale
    pub freq_scale: FrequencyScale,
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
}

impl Default for FftState {
    fn default() -> Self {
        Self {
            data: None,
            analysis: None,
            source_cache: None,
            selected_source: None,
            window: WindowFunction::Hanning,
            mag_scale: MagnitudeScale::DB,
            freq_scale: FrequencyScale::Linear,
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
        }
    }
}

impl FftState {
    /// Create new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load FFT data and analyze
    pub fn load_data(&mut self, data: FftData) {
        let analysis = SpectrumAnalysis::analyze(&data, self.num_harmonics);
        self.data = Some(data);
        self.analysis = Some(analysis);
        self.source_cache = None;
        self.update_auto_scale();
    }

    /// Load prepared uniformly sampled source and compute FFT using current settings.
    pub fn load_prepared_input(&mut self, input: PreparedFftInput) {
        self.selected_source = Some(input.name.clone());
        self.source_cache = Some(FftSourceCache {
            name: input.name,
            samples: input.samples,
            sample_rate: input.sample_rate,
            original_count: input.original_count,
            decimation_factor: input.decimation_factor,
        });
        self.recompute_from_source();
    }

    /// Select preferred source trace name.
    pub fn set_selected_source(&mut self, source_name: Option<String>) {
        self.selected_source = source_name;
    }

    /// Recompute FFT data from cached source using current window.
    pub fn recompute_from_source(&mut self) {
        let Some(source) = self.source_cache.as_ref() else {
            return;
        };
        let data = FftData::from_time_domain(
            &format!("FFT({})", source.name),
            &source.samples,
            source.sample_rate,
            self.window,
        );
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
                    let value = match self.mag_scale {
                        MagnitudeScale::DB => point.magnitude_db(),
                        MagnitudeScale::DBm => point.magnitude_dbm(self.z0),
                        MagnitudeScale::Linear => point.magnitude,
                    };
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
                        _ => {
                            self.mag_min = (min - padding).floor().max(-300.0);
                            self.mag_max = (max + padding).ceil().min(120.0);
                        }
                    }
                }
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
        self.marker_frequency = marker_frequency;
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
        assert_eq!(MagnitudeScale::DBm.display_name(), "dBm");
    }

    #[test]
    fn test_mag_scale_all() {
        let all = MagnitudeScale::all();
        assert_eq!(all.len(), 3);
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
        assert!(state.mag_auto);
        assert!(state.freq_auto);
        assert_eq!(state.z0, 50.0);
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

        state.load_data(data);

        assert!(state.has_data());
        assert!(!state.is_empty());
        assert!(state.analysis.is_some());
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
        data.sample_rate = 100.0;
        data.fft_size = 8;
        state.load_data(data);

        state.set_mag_scale(MagnitudeScale::Linear);
        assert!(state.mag_max > 2.0);
        assert!(state.mag_min >= 0.0);
    }
}
