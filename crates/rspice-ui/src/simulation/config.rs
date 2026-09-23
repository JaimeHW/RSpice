//! Analysis Configuration Builders
//!
//! Configuration structures for each analysis type that can be built from
//! the UI dialog state and converted to rspice-core analysis parameters.

use crate::simulation::dialog::OpConfig;

mod ac;
mod dc;
mod dc_mode;
mod frequency_table;
mod noise;
mod pole_zero;
mod recorded_fft;
mod sensitivity;
mod transient;

pub use crate::services::simulation_runner::{DcAxisMode, DcSweepModes};
pub use ac::{AcAnalysisConfig, AcDataAnalysisConfig, AcSweepType};
pub use dc::DcSweepConfig;
pub use frequency_table::{
    AC_FREQUENCY_TABLE, parse_ac_frequency_list, parse_explicit_frequency_list,
};
pub use noise::{
    NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
pub use pole_zero::{PoleZeroConfig, PzAnalysisType};
pub use recorded_fft::{
    FFT_DEFAULT_POINTS, FFT_WINDOWS, FftFormatChoice, FftRequest, fft_point_counts, window_keyword,
};
pub use sensitivity::{
    DESIGN_PARAMETERS_FILTER, SensitivityConfig, SensitivitySweep, canonical_sensitivity_filter,
    design_parameters_filter,
};
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

/// Legacy Fourier requests analyze one complete period.
pub(crate) const fn default_fourier_periods() -> usize {
    1
}
