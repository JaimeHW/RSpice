//! FFT Viewer State Management
//!
//! Viewer state for FFT/spectrum display.

use super::data::{FftData, SpectrumAnalysis};
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
}

impl Default for FftState {
    fn default() -> Self {
        Self {
            data: None,
            analysis: None,
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
        self.update_auto_scale();
    }

    /// Clear data
    pub fn clear(&mut self) {
        self.data = None;
        self.analysis = None;
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
                    self.freq_min = min;
                    self.freq_max = max;
                }
            }

            if self.mag_auto {
                if let Some((min, max)) = data.magnitude_range_db() {
                    let padding = (max - min) * 0.1;
                    self.mag_min = (min - padding).floor().max(-140.0);
                    self.mag_max = (max + padding).ceil().min(40.0);
                }
            }
        }
    }

    /// Set window function
    pub fn set_window(&mut self, window: WindowFunction) {
        self.window = window;
        // Would need to recalculate FFT in real implementation
    }

    /// Set magnitude scale
    pub fn set_mag_scale(&mut self, scale: MagnitudeScale) {
        self.mag_scale = scale;
    }

    /// Set frequency scale
    pub fn set_freq_scale(&mut self, scale: FrequencyScale) {
        self.freq_scale = scale;
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::data::FftPoint;
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
}
