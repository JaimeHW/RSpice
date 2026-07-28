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

pub(crate) mod compute;
pub(crate) mod data;
pub(crate) mod pipeline;
pub(crate) mod state;
pub(crate) mod window;

pub use compute::compute_fft;
pub use data::FftData;
pub use pipeline::{
    FftInputOptions, MIN_FFT_SAMPLES, PreparedFftInput, prepare_fft_input_with_options,
};
pub use state::{FftState, InputFidelity};
pub use window::WindowFunction;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod window_type_identity_tests {
    use super::{WindowFunction, compute_fft};

    /// This module used to declare `WindowFunction` twice: a four-variant copy
    /// in `compute` that `compute_fft` accepted, and the real eight-variant one
    /// in `window` that `mod.rs` re-exported. Importing both names from this
    /// module therefore did not compile, the two disagreed on the default
    /// window, and the module doc advertised Kaiser and Flat-top windows that
    /// the compute path could not accept.
    ///
    /// This test does not assert a value; it asserts that the re-exported type
    /// *is* the parameter type. If the duplicate is ever reintroduced, the
    /// call below stops compiling.
    #[test]
    fn the_reexported_window_type_is_the_one_compute_fft_accepts() {
        let time: Vec<f64> = (0..64).map(|i| i as f64 * 1e-6).collect();
        let values: Vec<f64> = time
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 10_000.0 * t).sin())
            .collect();

        for window in WindowFunction::all() {
            let result = compute_fft(&time, &values, *window)
                .expect("a 64-sample sine must transform for every window");
            assert_eq!(
                result.window, *window,
                "the result must report back the window it was given"
            );
        }
    }
}
