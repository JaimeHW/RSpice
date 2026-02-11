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
        }
    }

    /// Whether this analysis requires a prior DC OP
    pub fn requires_dc_op(&self) -> bool {
        matches!(
            self,
            AnalysisRunType::Ac
                | AnalysisRunType::Disto
                | AnalysisRunType::Noise
                | AnalysisRunType::Tf
                | AnalysisRunType::Sensitivity
                | AnalysisRunType::PoleZero
                | AnalysisRunType::Stb
                | AnalysisRunType::Reliability
        )
    }

    /// Whether this analysis requires PSS first
    pub fn requires_pss(&self) -> bool {
        matches!(
            self,
            AnalysisRunType::Pac
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Pxf
                | AnalysisRunType::Pstb
        )
    }

    /// Estimated relative complexity (for progress estimation)
    pub fn complexity(&self) -> u32 {
        match self {
            AnalysisRunType::DcOp => 1,
            AnalysisRunType::Tf => 1,
            AnalysisRunType::DcSweep => 5,
            AnalysisRunType::Ac => 3,
            AnalysisRunType::Disto => 7,
            AnalysisRunType::Transient => 10,
            AnalysisRunType::Noise => 5,
            AnalysisRunType::Sensitivity => 3,
            AnalysisRunType::PoleZero => 3,
            AnalysisRunType::HarmonicBalance => 15,
            AnalysisRunType::Pss => 20,
            AnalysisRunType::Pac => 5,
            AnalysisRunType::Pnoise => 10,
            AnalysisRunType::Pxf => 8,
            AnalysisRunType::Pstb => 9,
            AnalysisRunType::Stb => 4,
            AnalysisRunType::MonteCarlo => 50,
            AnalysisRunType::Parametric => 30,
            AnalysisRunType::Corner => 25,
            AnalysisRunType::Reliability => 12,
            AnalysisRunType::Optimization => 45,
            AnalysisRunType::Soa => 14,
            AnalysisRunType::SParameter => 6,
            AnalysisRunType::Envelope => 12,
            AnalysisRunType::Fourier => 7,
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

