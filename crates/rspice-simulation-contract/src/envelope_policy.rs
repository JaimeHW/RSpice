//! Portable serialized choices for envelope initialization and extraction.

use serde::{Deserialize, Serialize};

/// Periodic initialization used before an envelope-following solve.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeInitialPeriodicSolve {
    HarmonicBalance,
    PeriodicSteadyState,
    /// Compatibility default for specifications saved before this control existed.
    #[default]
    TransientSpectralEstimate,
}

/// Slow-time step policy for envelope analysis.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeAdaptiveMode {
    Enabled,
    /// Compatibility default preserving legacy fixed-step envelope requests.
    #[default]
    FixedEnvelopeStep,
    EventAlignedOnly,
}

/// Available envelope result-extraction implementation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeExtractionPath {
    #[serde(alias = "preview")]
    #[default]
    Projection,
}
