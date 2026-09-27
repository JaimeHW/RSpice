//! Spectrum normalization.
//!
//! How magnitudes are scaled — none, coherent gain, or power — which decides
//! whether a peak reads as its true amplitude.

/// Amplitude normalization used for FFT magnitudes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpectrumNormalization {
    /// Frequency components represent peak amplitude.
    #[default]
    Peak,
    /// Frequency components represent RMS amplitude.
    Rms,
}

impl SpectrumNormalization {
    /// Stable UI order for the supported amplitude conventions.
    pub const fn all() -> &'static [Self] {
        &[Self::Peak, Self::Rms]
    }

    /// User-facing name for the amplitude convention.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Peak => "Peak amplitude",
            Self::Rms => "RMS amplitude",
        }
    }

    #[inline]
    pub(super) fn scale_from_peak(&self) -> f64 {
        match self {
            Self::Peak => 1.0,
            Self::Rms => std::f64::consts::FRAC_1_SQRT_2,
        }
    }

    #[inline]
    pub fn relative_scale(from: SpectrumNormalization, to: SpectrumNormalization) -> f64 {
        to.scale_from_peak() / from.scale_from_peak()
    }
}
