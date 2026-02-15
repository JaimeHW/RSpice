//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

#[cfg(test)]
use crate::output_spec::{
    ac_output_value, parse_output_spec, resolve_node_or_ground_index, OutputSpec, OutputVoltageSpec,
};
#[cfg(test)]
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
#[cfg(test)]
use num_complex::Complex64;
#[cfg(test)]
use rspice_core::analysis::ac::AcResult;
#[cfg(test)]
use rspice_core::engine::Engine;
use rspice_core::engine::SimulationConfig;
#[cfg(test)]
use rspice_core::netlist::{Element, ElementKind, StepSweep};
#[cfg(test)]
use rspice_core::solver::SimulationResult as CoreSimulationResult;
#[cfg(test)]
use rspice_core::Value;
use rspice_core::{resolve_simulation_config, SimulationConfigOverrides};

mod harmonic_basis;
use harmonic_basis::{build_disto_two_tone_harmonic_plan, build_multi_tone_hb_layout};

mod ac;
mod dc_sweep;
mod disto;
mod envelope_fourier;
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
pub use ac::{run_ac_analysis, AcData};
pub use dc_sweep::{run_dc_sweep, DcSweepData};
#[cfg(test)]
use disto::interpolate_magnitude_at_for_tests;
pub use disto::{run_disto_analysis, DistoData, DistoFrequencySweep, DistoRunConfig, DistoTrace};
pub use envelope_fourier::{
    run_envelope_analysis, run_fourier_analysis, EnvelopeData, EnvelopeRunConfig, FourierData,
    FourierRunConfig,
};
pub use hb::{run_hb_analysis, HbData, HbRunConfig, HbToneRunConfig};
use helpers::{
    build_voltage_output_expr, generate_freq_points, infer_primary_output_node,
    infer_primary_source_name, is_ground_like, netlist_has_independent_source_named,
    normalize_voltage_signal_name,
};
pub use monte_carlo::{run_monte_carlo_analysis, MonteCarloData, MonteCarloVariableData};
pub use noise::{run_noise_analysis, NoiseData};
pub use optimization::{
    run_optimization_analysis, run_optimization_analysis_with_config, OptimizationAlgorithmMode,
    OptimizationData, OptimizationGoalMode, OptimizationRunConfig, OptimizationVariable,
};
pub use pac_pxf::{
    run_pac_analysis, run_pac_analysis_auto, run_pxf_analysis, run_pxf_analysis_with_config,
    PacData, PacFrequencySweep, PacRunConfig, PxfData, PxfFrequencySweep, PxfRunConfig,
};
pub use pnoise::{
    run_pnoise_analysis, run_pnoise_analysis_with_config, PnoiseData, PnoiseFrequencySweep,
    PnoiseReference, PnoiseRunConfig,
};
#[cfg(test)]
use pnoise_sideband::{build_pnoise_sideband_translated_frequencies, fold_sideband_samples};
pub use pole_zero::{run_pole_zero_analysis, PoleZeroData};
pub use pss::{run_pss_analysis, PssData};
pub use pstb::{run_pstb_analysis, run_pstb_analysis_with_config, PstbData, PstbRunConfig};
pub use reliability::{
    run_reliability_analysis, run_reliability_analysis_with_config, ReliabilityData,
    ReliabilityRunConfig,
};
pub use sensitivity::{run_sensitivity_analysis, SensitivityData};
pub use soa::{run_soa_analysis, run_soa_analysis_with_config, SoaData, SoaRunConfig};
pub use sparameter::{
    run_sparameter_analysis, SParameterData, SParameterPort, SParameterRunConfig, SParameterSweep,
};
pub use stb::{run_stb_analysis, StbData};
pub use sweeps::{
    run_corner_analysis, run_corner_analysis_with_config, run_parametric_analysis,
    run_parametric_analysis_with_config, CornerBaseMode, CornerData, CornerFrequencySweep,
    CornerProcess, CornerRunConfig, ParametricData, TempRunConfig,
};
pub use tf::{run_tf_analysis, run_tf_analysis_with_config, TfData, TfFrequencySweep, TfRunConfig};
pub use transient::{
    run_simulation, run_simulation_with_options, run_transient_analysis, SimulationResult,
    SimulationStats, TransientData,
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
mod tests;
