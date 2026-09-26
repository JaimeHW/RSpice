//! FFT formats, retained spectrum evidence, and exact coefficient storage.

#[cfg(feature = "engine-evidence")]
pub mod recorded;
#[cfg(feature = "engine-evidence")]
pub mod spectrum;
pub mod window;

use serde::{Deserialize, Serialize};

/// Effective `.FFT FORMAT`, as the engine resolved it for this spectrum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftSpectrumFormatEvidence {
    #[default]
    Normalized,
    Unnormalized,
}

impl FftSpectrumFormatEvidence {
    /// The keyword `.FFT FORMAT=` spells this with.
    #[must_use]
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Normalized => "NORM",
            Self::Unnormalized => "UNORM",
        }
    }
}

/// Convert the engine's resolved FFT format to its portable result representation.
#[cfg(feature = "engine-evidence")]
#[must_use]
pub const fn fft_format_from_core(
    format: rspice_core::netlist::FftFormat,
) -> FftSpectrumFormatEvidence {
    match format {
        rspice_core::netlist::FftFormat::Normalized => FftSpectrumFormatEvidence::Normalized,
        rspice_core::netlist::FftFormat::Unnormalized => FftSpectrumFormatEvidence::Unnormalized,
    }
}

/// Convert retained FFT format evidence to the engine's unit-policy vocabulary.
#[cfg(feature = "engine-evidence")]
#[must_use]
pub const fn fft_format_to_core(
    format: FftSpectrumFormatEvidence,
) -> rspice_core::netlist::FftFormat {
    match format {
        FftSpectrumFormatEvidence::Normalized => rspice_core::netlist::FftFormat::Normalized,
        FftSpectrumFormatEvidence::Unnormalized => rspice_core::netlist::FftFormat::Unnormalized,
    }
}
