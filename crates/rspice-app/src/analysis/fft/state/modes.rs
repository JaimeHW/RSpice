//! FFT input fidelity policy.

use super::super::pipeline::FftInputPolicy;

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
    /// Pipeline policy for this fidelity.
    pub fn input_policy(&self) -> FftInputPolicy {
        match self {
            Self::Reference => FftInputPolicy::reference(),
            Self::Interactive => FftInputPolicy::interactive_default(),
        }
    }
}
