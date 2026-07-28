//! What kind of run an analysis is: a single point, a sweep, or a
//! statistical sample set.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisRunType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// AC analysis
    Ac,
    /// Distortion analysis
    Disto,
    /// Transient analysis
    Transient,
    /// Noise analysis
    Noise,
    /// Transfer function
    Tf,
    /// Sensitivity analysis
    Sensitivity,
    /// Pole-zero analysis
    PoleZero,
    /// Harmonic balance
    HarmonicBalance,
    /// Periodic steady-state
    Pss,
    /// Periodic AC
    Pac,
    /// Periodic noise
    Pnoise,
    /// Periodic transfer function
    Pxf,
    /// Periodic stability
    Pstb,
    /// Loop stability
    Stb,
    /// Monte Carlo
    MonteCarlo,
    /// Parametric
    Parametric,
    /// Corner analysis
    Corner,
    /// Reliability aging analysis
    Reliability,
    /// Optimization analysis
    Optimization,
    /// Safety/SOA analysis
    Soa,
    /// S-parameter analysis
    SParameter,
    /// Envelope transient analysis
    Envelope,
    /// Fourier analysis
    Fourier,
    Qpss,
    Hbsp,
    Hbnoise,
    Psp,
    Qpac,
    Qpnoise,
    Qpxf,
    TransientNoise,
    DcMismatch,
}

impl AnalysisRunType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisRunType::DcOp => "DC Operating Point",
            AnalysisRunType::DcSweep => "DC Sweep",
            AnalysisRunType::Ac => "AC Analysis",
            AnalysisRunType::Disto => "DISTO",
            AnalysisRunType::Transient => "Transient",
            AnalysisRunType::Noise => "Noise",
            AnalysisRunType::Tf => "Transfer Function",
            AnalysisRunType::Sensitivity => "Sensitivity",
            AnalysisRunType::PoleZero => "Pole-Zero",
            AnalysisRunType::HarmonicBalance => "Harmonic Balance",
            AnalysisRunType::Pss => "PSS",
            AnalysisRunType::Pac => "PAC",
            AnalysisRunType::Pnoise => "PNoise",
            AnalysisRunType::Pxf => "PXF",
            AnalysisRunType::Pstb => "PSTB",
            AnalysisRunType::Stb => "STB",
            AnalysisRunType::MonteCarlo => "Monte Carlo",
            AnalysisRunType::Parametric => "Parametric",
            AnalysisRunType::Corner => "Corner",
            AnalysisRunType::Reliability => "Reliability",
            AnalysisRunType::Optimization => "Optimization",
            AnalysisRunType::Soa => "Safety (SOA)",
            AnalysisRunType::SParameter => "S-Parameter",
            AnalysisRunType::Envelope => "Envelope",
            AnalysisRunType::Fourier => "Fourier",
            AnalysisRunType::Qpss => "QPSS",
            AnalysisRunType::Hbsp => "HBSP",
            AnalysisRunType::Hbnoise => "HBNOISE",
            AnalysisRunType::Psp => "PSP",
            AnalysisRunType::Qpac => "QPAC",
            AnalysisRunType::Qpnoise => "QPNOISE",
            AnalysisRunType::Qpxf => "QPXF",
            AnalysisRunType::TransientNoise => "Transient Noise",
            AnalysisRunType::DcMismatch => "DC Mismatch Contribution",
        }
    }

}

// =============================================================================
// Analysis Specification
// =============================================================================

/// Frequency sweep mode used by AC/noise analyses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum FrequencySweep {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl FrequencySweep {
    /// Keyword expected by the simulation runner.
    pub fn runner_keyword(self) -> &'static str {
        match self {
            FrequencySweep::Decade => "dec",
            FrequencySweep::Octave => "oct",
            FrequencySweep::Linear => "lin",
        }
    }
}
