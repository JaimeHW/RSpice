//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, ac_output_value, parse_output_spec, resolve_node_or_ground_index,
};
#[cfg(test)]
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
#[cfg(test)]
use num_complex::Complex64;
#[cfg(test)]
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Element, ElementKind, StepSweep};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use rspice_core::{SimulationConfigOverrides, Value, resolve_simulation_config};

#[path = "simulation_runner/harmonic_basis.rs"]
mod harmonic_basis;
use harmonic_basis::{build_disto_two_tone_harmonic_plan, build_multi_tone_hb_layout};

#[path = "simulation_runner/ac.rs"]
mod ac;
#[path = "simulation_runner/dc_sweep.rs"]
mod dc_sweep;
#[path = "simulation_runner/disto.rs"]
mod disto;
#[path = "simulation_runner/envelope_fourier.rs"]
mod envelope_fourier;
#[path = "simulation_runner/hb.rs"]
mod hb;
#[path = "simulation_runner/helpers.rs"]
mod helpers;
#[path = "simulation_runner/monte_carlo.rs"]
mod monte_carlo;
#[path = "simulation_runner/noise.rs"]
mod noise;
#[path = "simulation_runner/optimization.rs"]
mod optimization;
#[path = "simulation_runner/pac_pxf.rs"]
mod pac_pxf;
#[path = "simulation_runner/pnoise.rs"]
mod pnoise;
#[path = "simulation_runner/pnoise_sideband.rs"]
mod pnoise_sideband;
#[path = "simulation_runner/pole_zero.rs"]
mod pole_zero;
#[path = "simulation_runner/pss.rs"]
mod pss;
#[path = "simulation_runner/pstb.rs"]
mod pstb;
#[path = "simulation_runner/reliability.rs"]
mod reliability;
#[path = "simulation_runner/sensitivity.rs"]
mod sensitivity;
#[path = "simulation_runner/soa.rs"]
mod soa;
#[path = "simulation_runner/sparameter.rs"]
mod sparameter;
#[path = "simulation_runner/stb.rs"]
mod stb;
#[path = "simulation_runner/sweeps.rs"]
mod sweeps;
#[path = "simulation_runner/tf.rs"]
mod tf;
#[path = "simulation_runner/transient.rs"]
mod transient;
pub use ac::{AcData, run_ac_analysis};
pub use dc_sweep::{DcSweepData, run_dc_sweep};
#[cfg(test)]
use disto::interpolate_magnitude_at_for_tests;
pub use disto::{DistoData, DistoFrequencySweep, DistoRunConfig, DistoTrace, run_disto_analysis};
pub use envelope_fourier::{
    EnvelopeData, EnvelopeRunConfig, FourierData, FourierRunConfig, run_envelope_analysis,
    run_fourier_analysis,
};
pub use hb::{HbData, HbRunConfig, HbToneRunConfig, run_hb_analysis};
use helpers::{
    build_voltage_output_expr, generate_freq_points, infer_primary_output_node,
    infer_primary_source_name, is_ground_like, netlist_has_independent_source_named,
    normalize_voltage_signal_name,
};
pub use monte_carlo::{MonteCarloData, MonteCarloVariableData, run_monte_carlo_analysis};
pub use noise::{NoiseData, run_noise_analysis};
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationData, OptimizationGoalMode, OptimizationRunConfig,
    OptimizationVariable, run_optimization_analysis, run_optimization_analysis_with_config,
};
pub use pac_pxf::{
    PacData, PacFrequencySweep, PacRunConfig, PxfData, PxfFrequencySweep, PxfRunConfig,
    run_pac_analysis, run_pac_analysis_auto, run_pxf_analysis, run_pxf_analysis_with_config,
};
pub use pnoise::{
    PnoiseData, PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig, run_pnoise_analysis,
    run_pnoise_analysis_with_config,
};
#[cfg(test)]
use pnoise_sideband::{build_pnoise_sideband_translated_frequencies, fold_sideband_samples};
pub use pole_zero::{PoleZeroData, run_pole_zero_analysis};
pub use pss::{PssData, run_pss_analysis};
pub use pstb::{PstbData, PstbRunConfig, run_pstb_analysis, run_pstb_analysis_with_config};
pub use reliability::{
    ReliabilityData, ReliabilityRunConfig, run_reliability_analysis,
    run_reliability_analysis_with_config,
};
pub use sensitivity::{SensitivityData, run_sensitivity_analysis};
pub use soa::{SoaData, SoaRunConfig, run_soa_analysis, run_soa_analysis_with_config};
pub use sparameter::{
    SParameterData, SParameterPort, SParameterRunConfig, SParameterSweep, run_sparameter_analysis,
};
pub use stb::{StbData, run_stb_analysis};
pub use sweeps::{
    CornerBaseMode, CornerData, CornerFrequencySweep, CornerProcess, CornerRunConfig,
    ParametricData, TempRunConfig, run_corner_analysis, run_corner_analysis_with_config,
    run_parametric_analysis, run_parametric_analysis_with_config,
};
pub use tf::{TfData, TfFrequencySweep, TfRunConfig, run_tf_analysis, run_tf_analysis_with_config};
pub use transient::{
    SimulationResult, SimulationStats, TransientData, run_simulation, run_simulation_with_options,
    run_transient_analysis,
};

#[cfg(test)]
fn inject_param_overrides(
    netlist_text: &str,
    vars: &std::collections::HashMap<String, Value>,
) -> String {
    optimization::inject_param_overrides_for_tests(netlist_text, vars)
}

#[cfg(test)]
fn format_param_override_value(value: Value) -> String {
    optimization::format_param_override_value_for_tests(value)
}

#[cfg(test)]
fn extract_reliability_stress_data(
    elements: &[Element],
    node_voltages: &std::collections::HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
) -> std::collections::HashMap<String, StressMetrics> {
    reliability::extract_reliability_stress_data_for_tests(
        elements,
        node_voltages,
        temperature_k,
        min_stress_voltage,
    )
}

#[cfg(test)]
fn apply_reliability_mechanism_scaling(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
) {
    reliability::apply_reliability_mechanism_scaling_for_tests(results, config)
}

#[cfg(test)]
fn expand_step_sweep_values(sweep: &StepSweep) -> Result<Vec<Value>, String> {
    sweeps::expand_step_sweep_values_for_tests(sweep)
}

#[cfg(test)]
fn extract_temp_points(netlist: &rspice_core::Netlist) -> Vec<Value> {
    sweeps::extract_temp_points_for_tests(netlist)
}

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
    use wasm_bindgen::JsCast;
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

#[cfg(test)]
#[path = "simulation_runner/tests.rs"]
mod tests;
