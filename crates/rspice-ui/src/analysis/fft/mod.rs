//! FFT Viewer Module
//!
//! Commercial-grade FFT/Spectrum analyzer implementation.
//!
//! # Features
//!
//! - Multiple windowing functions (Hanning, Hamming, Blackman, Kaiser, etc.)
//! - dB and linear magnitude scales
//! - Log and linear frequency axes
//! - Peak detection with harmonic markers
//! - THD (Total Harmonic Distortion) calculation
//! - SFDR (Spurious-Free Dynamic Range) measurement
//! - Noise floor detection
//!
//! # Architecture
//!
//! Follows Cadence Spectre's spectral analysis approach.

pub mod compute;
pub mod data;
pub mod pipeline;
pub mod rendering;
pub mod state;
pub mod window;

pub use compute::{compute_fft, FftResult};
pub use data::{FftData, FftPoint, SpectrumAnalysis};
pub use pipeline::{prepare_fft_input, PreparedFftInput, DEFAULT_MAX_FFT_POINTS, MIN_FFT_SAMPLES};
pub use rendering::render_fft_plot;
pub use state::FftState;
pub use window::WindowFunction;

use crate::common::app::AppState;
use egui::Ui;

/// Render the FFT viewer panel
pub fn render_fft_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_fft_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _state = FftState::new();
    }

    #[test]
    fn test_fft_data_creation() {
        let data = FftData::new("Test");
        assert!(data.is_empty());
    }

    #[test]
    fn test_window_function_default() {
        let window = WindowFunction::default();
        assert_eq!(window, WindowFunction::Hanning);
    }
}
