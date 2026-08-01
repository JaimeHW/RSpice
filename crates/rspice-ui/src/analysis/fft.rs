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

pub(crate) mod data;
pub(crate) mod pipeline;
pub(crate) mod state;
pub(crate) mod window;

#[cfg(test)]
pub use data::FftData;
pub use pipeline::{
    FftInputOptions, MIN_FFT_SAMPLES, PreparedFftInput, prepare_fft_input_with_options,
};
pub use state::{FftState, InputFidelity};
pub use window::WindowFunction;
