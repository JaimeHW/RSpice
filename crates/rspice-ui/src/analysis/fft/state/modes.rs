use super::super::pipeline::FftInputPolicy;

// =============================================================================
// Scale Mode
// =============================================================================

/// Magnitude scale mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MagnitudeScale {
    /// dB scale (20 * log10)
    #[default]
    DB,
    /// dBc relative to fundamental level
    DBc,
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
            Self::DBc => "dBc",
            Self::Linear => "Linear",
            Self::DBm => "dBm",
        }
    }

    /// All modes
    pub fn all() -> &'static [MagnitudeScale] {
        &[Self::DB, Self::DBc, Self::Linear, Self::DBm]
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

/// FFT input fidelity mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputFidelity {
    /// Preserve source detail for analysis-grade spectra (default).
    #[default]
    Reference,
    /// Enforce capped point count for faster interaction on large datasets.
    Interactive,
}

impl InputFidelity {
    /// Display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Reference => "Reference",
            Self::Interactive => "Interactive",
        }
    }

    /// All modes.
    pub fn all() -> &'static [InputFidelity] {
        &[Self::Reference, Self::Interactive]
    }

    /// Pipeline policy for this fidelity.
    pub fn input_policy(&self) -> FftInputPolicy {
        match self {
            Self::Reference => FftInputPolicy::reference(),
            Self::Interactive => FftInputPolicy::interactive_default(),
        }
    }
}
