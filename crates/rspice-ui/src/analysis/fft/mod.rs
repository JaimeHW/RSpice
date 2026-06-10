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
pub mod state;
pub mod window;

pub use compute::{FftResult, compute_fft};
pub use data::{FftData, FftPoint, SpectrumAnalysis};
pub use pipeline::{
    DEFAULT_MAX_FFT_POINTS, FftInputOptions, FftInputPolicy, FftTimeWindow,
    MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES, PreparedFftInput, prepare_fft_input,
    prepare_fft_input_with_options, prepare_fft_input_with_policy,
};
pub use state::{FftState, InputFidelity};
pub use window::WindowFunction;


// =============================================================================
// Tests
// =============================================================================
