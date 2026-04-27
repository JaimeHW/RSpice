//! Analysis Configuration Builders
//!
//! Configuration structures for each analysis type that can be built from
//! the UI dialog state and converted to rspice-core analysis parameters.

use crate::state::AnalysisType;

mod ac;
mod dc;
mod noise;
mod pole_zero;
mod sensitivity;
mod transient;

pub use ac::{AcAnalysisConfig, AcSweepType};
pub use dc::DcSweepConfig;
pub use noise::NoiseAnalysisConfig;
pub use pole_zero::{PoleZeroConfig, PzAnalysisType};
pub use sensitivity::SensitivityConfig;
pub use transient::TransientAnalysisConfig;

//=============================================================================
// Analysis Configuration
//=============================================================================

/// Unified analysis configuration
#[derive(Debug, Clone)]
pub enum AnalysisConfig {
    /// DC operating point (no parameters)
    DcOp,

    /// DC sweep configuration
    DcSweep(DcSweepConfig),

    /// Transient analysis configuration
    Transient(TransientAnalysisConfig),

    /// AC analysis configuration
    Ac(AcAnalysisConfig),

    /// Noise analysis configuration
    Noise(NoiseAnalysisConfig),

    /// Pole-zero analysis configuration
    PoleZero(PoleZeroConfig),

    /// Sensitivity analysis configuration
    Sensitivity(SensitivityConfig),
}

impl AnalysisConfig {
    /// Get the analysis type
    pub fn analysis_type(&self) -> AnalysisType {
        match self {
            AnalysisConfig::DcOp => AnalysisType::DcOp,
            AnalysisConfig::DcSweep(_) => AnalysisType::DcSweep,
            AnalysisConfig::Transient(_) => AnalysisType::Transient,
            AnalysisConfig::Ac(_) => AnalysisType::Ac,
            AnalysisConfig::Noise(_) => AnalysisType::Noise,
            AnalysisConfig::PoleZero(_) => AnalysisType::PoleZero,
            AnalysisConfig::Sensitivity(_) => AnalysisType::Sensitivity,
        }
    }

    /// Generate SPICE analysis command
    pub fn to_spice(&self) -> String {
        match self {
            AnalysisConfig::DcOp => ".op".to_string(),
            AnalysisConfig::DcSweep(cfg) => cfg.to_spice(),
            AnalysisConfig::Transient(cfg) => cfg.to_spice(),
            AnalysisConfig::Ac(cfg) => cfg.to_spice(),
            AnalysisConfig::Noise(cfg) => cfg.to_spice(),
            AnalysisConfig::PoleZero(cfg) => cfg.to_spice(),
            AnalysisConfig::Sensitivity(cfg) => cfg.to_spice(),
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        match self {
            AnalysisConfig::DcOp => Ok(()),
            AnalysisConfig::DcSweep(cfg) => cfg.validate(),
            AnalysisConfig::Transient(cfg) => cfg.validate(),
            AnalysisConfig::Ac(cfg) => cfg.validate(),
            AnalysisConfig::Noise(cfg) => cfg.validate(),
            AnalysisConfig::PoleZero(cfg) => cfg.validate(),
            AnalysisConfig::Sensitivity(cfg) => cfg.validate(),
        }
    }
}
