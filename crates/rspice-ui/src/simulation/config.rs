//! Analysis Configuration Builders
//!
//! Configuration structures for each analysis type that can be built from
//! the UI dialog state and converted to rspice-core analysis parameters.

use crate::simulation::dialog::OpConfig;

mod ac;
mod dc;
mod noise;
mod pole_zero;
mod sensitivity;
mod transient;

pub use ac::{AcAnalysisConfig, AcSweepType};
pub use dc::DcSweepConfig;
pub use noise::{
    NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
pub use pole_zero::{PoleZeroConfig, PzAnalysisType};
pub use sensitivity::SensitivityConfig;
pub use transient::TransientAnalysisConfig;

//=============================================================================
// Analysis Configuration
//=============================================================================

/// Unified analysis configuration
#[derive(Debug, Clone)]
pub enum AnalysisConfig {
    /// DC operating point solve and retained-output contract.
    DcOp(OpConfig),

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
    #[must_use]
    pub fn dc_op() -> Self {
        Self::DcOp(OpConfig::default())
    }

    /// Generate SPICE analysis command
    pub fn to_spice(&self) -> String {
        match self {
            AnalysisConfig::DcOp(config) => config.to_spice(),
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
            AnalysisConfig::DcOp(config) => config.validate().map_err(|error| vec![error]),
            AnalysisConfig::DcSweep(cfg) => cfg.validate(),
            AnalysisConfig::Transient(cfg) => cfg.validate(),
            AnalysisConfig::Ac(cfg) => cfg.validate(),
            AnalysisConfig::Noise(cfg) => cfg.validate(),
            AnalysisConfig::PoleZero(cfg) => cfg.validate(),
            AnalysisConfig::Sensitivity(cfg) => cfg.validate(),
        }
    }
}
