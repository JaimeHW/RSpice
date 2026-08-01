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
mod disto;
mod envelope_fourier;
mod error;
mod hb;
mod helpers;
mod monte_carlo;
mod optimization;
mod pac_pxf;
mod pnoise;
mod pnoise_sideband;
mod pss;
mod pstb;
mod reliability;
mod soa;
mod sparameter;
mod stb;
mod sweeps;
mod tf;
mod transient;
// Each analysis re-exports the request types a caller must construct and the
// entry point it calls. Result types are not re-exported: callers receive them
// from the entry point and never name them here.
pub use ac::run_ac_analysis_with_source_path_and_abort;
pub use disto::{
    DistoFrequencySweep, DistoRunConfig, run_disto_analysis_with_source_path_and_abort,
};
pub use envelope_fourier::{
    EnvelopeRunConfig, FourierData, FourierRunConfig,
    run_envelope_analysis_with_source_path_and_abort,
};
pub(crate) use envelope_fourier::{
    fourier_output_is_current, run_fourier_from_signal_with_abort, validate_fourier_output_accessor,
};
pub use error::{ServiceRunError, ServiceRunResult};
pub use hb::{HbRunConfig, HbToneRunConfig, run_hb_analysis_with_source_path_and_abort};
use helpers::{
    build_voltage_output_expr, generate_freq_points_with_abort,
    infer_primary_output_node_with_abort, infer_primary_source_name_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, normalize_voltage_signal_name,
    parse_runner_netlist_with_abort,
};
pub use monte_carlo::run_monte_carlo_analysis_with_source_path_and_abort;
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationGoalMode, OptimizationRunConfig, OptimizationVariable,
    run_optimization_analysis_with_config_and_source_path_and_abort,
};
pub use pac_pxf::{
    PacFrequencySweep, PacRunConfig, PxfFrequencySweep, PxfRunConfig,
    run_pac_analysis_from_pss_with_source_path_and_abort,
    run_pxf_analysis_from_pss_with_source_path_and_abort,
};
pub use pnoise::{
    PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig,
    run_pnoise_analysis_from_pss_with_source_path_and_abort,
};
pub use pss::{
    PssData, PssRunConfig, run_pss_analysis_with_dc_seed_and_source_path_and_abort,
    run_pss_analysis_with_source_path_and_abort,
};
pub use pstb::{PstbRunConfig, run_pstb_analysis_from_pss_with_source_path_and_abort};
pub use reliability::{
    ReliabilityRunConfig, run_reliability_analysis_with_config_and_source_path_and_abort,
};
// DC sweep, noise, pole-zero, and sensitivity have no entry here, and that is
// the module boundary rather than an omission. The seven fundamental analyses
// -- DC op, DC sweep, transient, AC, noise, pole-zero, sensitivity -- ship
// through `simulation::engine_bridge`, dispatched from `AnalysisConfig`. This
// module is the RF and advanced layer. Duplicates of all four once sat here
// unreachable; adding a fifth would mean the same thing again.
pub use soa::{SoaRunConfig, run_soa_analysis_with_config_and_source_path_and_abort};
pub use sparameter::{
    SParameterPort, SParameterRunConfig, SParameterSweep, run_sparameter_analysis_with_abort,
};
pub use stb::run_stb_analysis_with_sweep_and_source_path_and_abort;
pub use sweeps::{
    CornerBaseMode, CornerFrequencySweep, CornerModelBinding, CornerProcess, CornerRunConfig,
    TempRunConfig, run_corner_analysis_with_config_and_source_path_and_abort,
    run_corner_analysis_with_source_path_and_abort,
    run_parametric_analysis_with_config_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
pub(crate) use sweeps::{
    REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END, apply_voltage_corner,
    expand_corner_pvt_points, expand_step_sweep_values, infer_nominal_supply_voltage,
    materialize_corner_process_source,
};
pub use tf::{
    TfAccuracy, TfGainUnit, TfNormalization, TfQuantity, TfRunConfig,
    run_tf_analysis_with_config_and_abort,
};
pub use transient::{TransientData, run_transient_analysis_with_source_path_and_abort};

// =============================================================================
// Platform-agnostic timing utilities
// =============================================================================

/// Get current time in milliseconds (for performance measurement)
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
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
