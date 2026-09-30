//! Analysis preparation over borrowed domain inputs; this module does not authorize or execute runs.

mod helpers;
mod run_config;
mod spec;

pub use helpers::{map_frequency_sweep, parse_optional_spice_value, parse_positive_points};
pub use run_config::{
    corner_run_config_from_dialog, pac_run_config_from_dialog, pnoise_run_config_from_dialog,
    pstb_run_config_from_dialog, pxf_run_config_from_dialog, temp_run_config_from_dialog,
};
pub use spec::{
    AnalysisInputs, analysis_spec_to_config, build_envelope_spec, build_fourier_spec,
    build_harmonic_balance_spec, build_op_spec, build_optimization_spec, build_pole_zero_spec,
    build_pss_spec, build_sensitivity_spec, build_soa_spec, build_sp_spec, build_stb_spec,
    build_tf_spec,
};
