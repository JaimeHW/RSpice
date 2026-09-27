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
    FftInputError, FftInputOptions, MIN_FFT_SAMPLES, PreparedFftInput,
    prepare_fft_input_with_options,
};
#[cfg(test)]
pub use state::FftFailure;
pub use state::{FftState, InputFidelity};
pub use window::WindowFunction;

/// Build arbitrary spectrum fixtures while retaining calibrated rectangular metadata.
#[cfg(test)]
pub(crate) fn spectrum_fixture(
    name: &str,
    frequencies: &[f64],
    magnitudes: &[f64],
    phases: &[f64],
    sample_rate: f64,
    normalization: data::SpectrumNormalization,
) -> FftData {
    let mut data = FftData::from_time_domain_with_normalization(
        name,
        &[0.0; MIN_FFT_SAMPLES],
        1.0,
        WindowFunction::Rectangular,
        normalization,
    )
    .expect("qualified zero-spectrum fixture");
    let n = frequencies.len().min(magnitudes.len()).min(phases.len());
    data.points = (0..n)
        .map(|i| data::FftPoint {
            frequency: frequencies[i],
            magnitude: magnitudes[i],
            phase: phases[i],
        })
        .collect();
    data.sample_rate = sample_rate;
    data.fft_size = n.saturating_sub(1).saturating_mul(2);
    data
}
