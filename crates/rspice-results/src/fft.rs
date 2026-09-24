//! Portable format shared by FFT requests and retained spectrum evidence.

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
