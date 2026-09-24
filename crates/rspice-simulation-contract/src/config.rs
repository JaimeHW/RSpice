//! Typed analysis fields, validation, and SPICE card generation.

mod ac;
mod dc;
mod dc_axis;
mod frequency_sweep;
mod frequency_table;
mod noise;
mod pole_zero;
mod sensitivity;
mod transient;

pub use ac::{
    AcAnalysisConfig, AcDataAnalysisConfig, AcDataParameterColumn, AcDataTableOptions, AcSweepType,
};
pub use dc::DcSweepConfig;
pub use dc_axis::{DcAxisMode, DcSweepModes};
pub use frequency_sweep::{FrequencySweep, SensitivitySweepSpec};
pub use frequency_table::{
    AC_FREQUENCY_TABLE, parse_ac_frequency_list, parse_explicit_frequency_list,
};
pub use noise::{
    NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
pub use pole_zero::{PoleZeroConfig, PzAnalysisType};
pub use sensitivity::{
    DESIGN_PARAMETERS_FILTER, SensitivityConfig, SensitivitySweep, canonical_sensitivity_filter,
    design_parameters_filter,
};
pub use transient::TransientAnalysisConfig;
