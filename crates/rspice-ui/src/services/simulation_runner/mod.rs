//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use rspice_core::engine::SimulationConfig;
use rspice_core::{SimulationConfigOverrides, resolve_simulation_config};

mod harmonic_basis;
use harmonic_basis::{
    build_disto_two_tone_harmonic_plan_with_abort, build_multi_tone_hb_layout_with_abort,
};

mod ac;
mod dc_sweep;
mod disto;
mod envelope_fourier;
mod error;
mod hb;
mod helpers;
mod monte_carlo;
mod noise;
mod optimization;
mod pac_pxf;
mod pnoise;
mod pnoise_sideband;
mod pole_zero;
mod pss;
mod pstb;
mod reliability;
mod sensitivity;
mod soa;
mod sparameter;
mod stb;
mod sweeps;
mod tf;
mod transient;
pub use ac::{
    AcData, run_ac_analysis, run_ac_analysis_with_abort, run_ac_analysis_with_source_path,
    run_ac_analysis_with_source_path_and_abort,
};
pub use dc_sweep::{
    DcSweepData, run_dc_sweep, run_dc_sweep_with_abort, run_dc_sweep_with_source_path,
    run_dc_sweep_with_source_path_and_abort,
};
pub use disto::{
    DistoData, DistoFrequencySweep, DistoRunConfig, DistoTrace, run_disto_analysis,
    run_disto_analysis_with_abort, run_disto_analysis_with_source_path,
    run_disto_analysis_with_source_path_and_abort,
};
pub use envelope_fourier::{
    EnvelopeData, EnvelopeRunConfig, FourierData, FourierRunConfig, run_envelope_analysis,
    run_envelope_analysis_with_abort, run_envelope_analysis_with_source_path,
    run_envelope_analysis_with_source_path_and_abort, run_fourier_analysis,
    run_fourier_analysis_with_abort, run_fourier_analysis_with_source_path,
    run_fourier_analysis_with_source_path_and_abort,
};
pub use error::{ServiceRunError, ServiceRunResult};
pub use hb::{
    HbData, HbRunConfig, HbToneRunConfig, run_hb_analysis, run_hb_analysis_with_abort,
    run_hb_analysis_with_source_path, run_hb_analysis_with_source_path_and_abort,
};
use helpers::{
    build_voltage_output_expr, generate_freq_points_with_abort,
    infer_primary_output_node_with_abort, infer_primary_source_name_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, normalize_voltage_signal_name,
    parse_runner_netlist_with_abort,
};
pub use monte_carlo::{
    MonteCarloData, MonteCarloVariableData, run_monte_carlo_analysis,
    run_monte_carlo_analysis_with_abort, run_monte_carlo_analysis_with_source_path,
    run_monte_carlo_analysis_with_source_path_and_abort,
};
pub use noise::{
    NoiseData, NoiseRunSpec, run_noise_analysis, run_noise_analysis_with_abort,
    run_noise_analysis_with_source_path, run_noise_analysis_with_source_path_and_abort,
};
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationData, OptimizationGoalMode, OptimizationRunConfig,
    OptimizationVariable, run_optimization_analysis, run_optimization_analysis_with_abort,
    run_optimization_analysis_with_config, run_optimization_analysis_with_config_and_abort,
    run_optimization_analysis_with_config_and_source_path,
    run_optimization_analysis_with_config_and_source_path_and_abort,
    run_optimization_analysis_with_source_path,
    run_optimization_analysis_with_source_path_and_abort,
};
pub use pac_pxf::{
    PacData, PacFrequencySweep, PacRunConfig, PxfData, PxfFrequencySweep, PxfRunConfig,
    run_pac_analysis, run_pac_analysis_auto, run_pac_analysis_auto_with_abort,
    run_pac_analysis_auto_with_source_path, run_pac_analysis_auto_with_source_path_and_abort,
    run_pac_analysis_with_abort, run_pac_analysis_with_source_path,
    run_pac_analysis_with_source_path_and_abort, run_pxf_analysis, run_pxf_analysis_with_abort,
    run_pxf_analysis_with_config, run_pxf_analysis_with_config_and_abort,
    run_pxf_analysis_with_config_and_source_path,
    run_pxf_analysis_with_config_and_source_path_and_abort, run_pxf_analysis_with_source_path,
    run_pxf_analysis_with_source_path_and_abort,
};
pub use pnoise::{
    PnoiseData, PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig, run_pnoise_analysis,
    run_pnoise_analysis_with_abort, run_pnoise_analysis_with_config,
    run_pnoise_analysis_with_config_and_abort, run_pnoise_analysis_with_config_and_source_path,
    run_pnoise_analysis_with_config_and_source_path_and_abort,
    run_pnoise_analysis_with_source_path, run_pnoise_analysis_with_source_path_and_abort,
};
pub use pole_zero::{
    PoleZeroData, PoleZeroRunSpec, run_pole_zero_analysis, run_pole_zero_analysis_with_abort,
    run_pole_zero_analysis_with_source_path, run_pole_zero_analysis_with_source_path_and_abort,
};
pub use pss::{
    PssData, run_pss_analysis, run_pss_analysis_with_abort, run_pss_analysis_with_source_path,
    run_pss_analysis_with_source_path_and_abort,
};
pub use pstb::{
    PstbData, PstbRunConfig, run_pstb_analysis, run_pstb_analysis_with_abort,
    run_pstb_analysis_with_config, run_pstb_analysis_with_config_and_abort,
    run_pstb_analysis_with_config_and_source_path,
    run_pstb_analysis_with_config_and_source_path_and_abort, run_pstb_analysis_with_source_path,
    run_pstb_analysis_with_source_path_and_abort,
};
pub use reliability::{
    ReliabilityData, ReliabilityRunConfig, run_reliability_analysis,
    run_reliability_analysis_with_abort, run_reliability_analysis_with_config,
    run_reliability_analysis_with_config_and_abort,
    run_reliability_analysis_with_config_and_source_path,
    run_reliability_analysis_with_config_and_source_path_and_abort,
    run_reliability_analysis_with_source_path, run_reliability_analysis_with_source_path_and_abort,
};
pub use sensitivity::{
    SensitivityData, run_sensitivity_analysis, run_sensitivity_analysis_with_abort,
    run_sensitivity_analysis_with_source_path, run_sensitivity_analysis_with_source_path_and_abort,
};
pub use soa::{
    SoaData, SoaRunConfig, run_soa_analysis, run_soa_analysis_with_abort,
    run_soa_analysis_with_config, run_soa_analysis_with_config_and_abort,
    run_soa_analysis_with_config_and_source_path,
    run_soa_analysis_with_config_and_source_path_and_abort, run_soa_analysis_with_source_path,
    run_soa_analysis_with_source_path_and_abort,
};
pub use sparameter::{
    SParameterData, SParameterPort, SParameterRunConfig, SParameterSweep, run_sparameter_analysis,
    run_sparameter_analysis_with_abort, run_sparameter_analysis_with_source_path,
    run_sparameter_analysis_with_source_path_and_abort,
};
pub use stb::{
    StbData, run_stb_analysis, run_stb_analysis_with_abort, run_stb_analysis_with_source_path,
    run_stb_analysis_with_source_path_and_abort, run_stb_analysis_with_sweep_and_source_path,
    run_stb_analysis_with_sweep_and_source_path_and_abort,
};
pub use sweeps::{
    CornerBaseMode, CornerData, CornerFrequencySweep, CornerModelBinding, CornerProcess,
    CornerRunConfig, ParametricData, TempRunConfig, run_corner_analysis,
    run_corner_analysis_with_abort, run_corner_analysis_with_config,
    run_corner_analysis_with_config_and_abort, run_corner_analysis_with_config_and_source_path,
    run_corner_analysis_with_config_and_source_path_and_abort,
    run_corner_analysis_with_source_path, run_corner_analysis_with_source_path_and_abort,
    run_parametric_analysis, run_parametric_analysis_with_abort,
    run_parametric_analysis_with_config, run_parametric_analysis_with_config_and_abort,
    run_parametric_analysis_with_config_and_source_path,
    run_parametric_analysis_with_config_and_source_path_and_abort,
    run_parametric_analysis_with_source_path, run_parametric_analysis_with_source_path_and_abort,
};
pub(crate) use sweeps::{
    REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END, expand_corner_pvt_points,
    expand_step_sweep_values,
};
pub use tf::{
    TfData, TfFrequencySweep, TfRunConfig, run_tf_analysis, run_tf_analysis_with_abort,
    run_tf_analysis_with_config, run_tf_analysis_with_config_and_abort,
    run_tf_analysis_with_config_and_source_path,
    run_tf_analysis_with_config_and_source_path_and_abort, run_tf_analysis_with_source_path,
    run_tf_analysis_with_source_path_and_abort,
};
pub use transient::{
    SimulationResult, SimulationStats, TransientData, run_simulation, run_simulation_with_abort,
    run_simulation_with_options, run_simulation_with_options_and_abort,
    run_simulation_with_options_and_source_path,
    run_simulation_with_options_and_source_path_and_abort, run_simulation_with_source_path,
    run_simulation_with_source_path_and_abort, run_transient_analysis,
    run_transient_analysis_with_abort, run_transient_analysis_with_source_path,
    run_transient_analysis_with_source_path_and_abort,
};

// =============================================================================
// Platform-agnostic timing utilities
// =============================================================================

/// Get current time in milliseconds (for performance measurement)
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

const DEFAULT_MONTE_CARLO_SEED: u64 = 0x5EED_5EED;

fn build_engine_config(
    netlist: &rspice_core::Netlist,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
) -> SimulationConfig {
    match options {
        Some(opts) => opts.resolve_simulation_config(Some(&netlist.options)),
        None => resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================
