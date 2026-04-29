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
    #[inline]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Peak => "Peak",
            Self::Rms => "RMS",
        }
    }

    #[inline]
    pub fn all() -> &'static [SpectrumNormalization] {
        &[Self::Rms, Self::Peak]
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
