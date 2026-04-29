use crate::simulation::multi_run::AnalysisRunType;

use super::AnalysisSpec;

impl AnalysisSpec {
    /// Get the corresponding run type.
    pub fn run_type(&self) -> AnalysisRunType {
        match self {
            AnalysisSpec::DcOp => AnalysisRunType::DcOp,
            AnalysisSpec::DcSweep { .. } => AnalysisRunType::DcSweep,
            AnalysisSpec::Ac { .. } => AnalysisRunType::Ac,
            AnalysisSpec::Disto { .. } => AnalysisRunType::Disto,
            AnalysisSpec::Transient { .. } => AnalysisRunType::Transient,
            AnalysisSpec::Noise { .. } => AnalysisRunType::Noise,
            AnalysisSpec::Pss { .. } => AnalysisRunType::Pss,
            AnalysisSpec::HarmonicBalance { .. } => AnalysisRunType::HarmonicBalance,
            AnalysisSpec::Tf => AnalysisRunType::Tf,
            AnalysisSpec::Sensitivity { .. } => AnalysisRunType::Sensitivity,
            AnalysisSpec::PoleZero { .. } => AnalysisRunType::PoleZero,
            AnalysisSpec::Pac => AnalysisRunType::Pac,
            AnalysisSpec::Pnoise => AnalysisRunType::Pnoise,
            AnalysisSpec::Pxf => AnalysisRunType::Pxf,
            AnalysisSpec::Pstb => AnalysisRunType::Pstb,
            AnalysisSpec::Stb { .. } => AnalysisRunType::Stb,
            AnalysisSpec::MonteCarlo => AnalysisRunType::MonteCarlo,
            AnalysisSpec::Parametric => AnalysisRunType::Parametric,
            AnalysisSpec::Corner => AnalysisRunType::Corner,
            AnalysisSpec::Reliability { .. } => AnalysisRunType::Reliability,
            AnalysisSpec::Optimization { .. } => AnalysisRunType::Optimization,
            AnalysisSpec::Soa { .. } => AnalysisRunType::Soa,
            AnalysisSpec::SParameter { .. } => AnalysisRunType::SParameter,
            AnalysisSpec::Envelope { .. } => AnalysisRunType::Envelope,
            AnalysisSpec::Fourier { .. } => AnalysisRunType::Fourier,
        }
    }
}
