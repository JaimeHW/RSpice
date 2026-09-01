//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::{SimulationConfigOverrides, resolve_simulation_config};

mod harmonic_basis;
use harmonic_basis::build_multi_tone_hb_layout_with_abort;

mod disto;
mod envelope_fourier;
mod error;
mod hb;
mod hbnoise;
mod helpers;
mod monte_carlo;
mod optimization;
mod pac_pxf;
mod pnoise;
mod psp;
mod pss;
mod pstb;
mod soa;
mod sparameter;
mod stb;
mod sweeps;
mod tf;
mod transient;
// Each analysis re-exports the request types a caller must construct and the
// entry point it calls. Result types are not re-exported: callers receive them
// from the entry point and never name them here.
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
pub(crate) use hb::build_core_hb_config;
pub use hb::{HbRunConfig, HbToneRunConfig, run_hb_analysis_with_source_path_and_abort};
pub(crate) use hbnoise::integrate_psd;
pub use hbnoise::{
    HbnoiseFrequencySweep, HbnoiseRunConfig,
    run_hbnoise_analysis_from_hb_with_source_path_and_abort,
};
use helpers::{
    build_voltage_output_expr, generate_freq_points_with_abort,
    infer_primary_output_node_with_abort, infer_primary_source_name_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, normalize_voltage_signal_name,
    parse_runner_netlist_with_abort, parse_runner_netlist_with_statistical_sampling_and_abort,
};
pub(crate) use monte_carlo::{
    run_monte_carlo_analysis_with_environment_and_source_path_and_abort,
    run_statistical_monte_carlo_with_environment_and_source_path_and_abort,
};
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
pub(crate) use psp::PspData;
pub use psp::{
    PspRunConfig, PspSweep, run_hbsp_analysis_from_hb_with_source_path_and_abort,
    run_psp_analysis_from_pss_with_source_path_and_abort,
};
pub use pss::{
    PssRunConfig, compute_fft_harmonics_with_abort,
    run_pss_analysis_with_dc_seed_and_source_path_and_abort,
    run_pss_analysis_with_source_path_and_abort,
};
pub use pstb::{PstbRunConfig, run_pstb_analysis_from_pss_with_source_path_and_abort};
// DC sweep, noise, pole-zero, and sensitivity have no entry here, and that is
// the module boundary rather than an omission. The seven fundamental analyses
// -- DC op, DC sweep, transient, AC, noise, pole-zero, sensitivity -- ship
// through `simulation::engine_bridge`, dispatched from `AnalysisConfig`. This
// module is the RF and advanced layer. Duplicates of all four once sat here
// unreachable; adding a fifth would mean the same thing again.
pub use soa::{SoaRunConfig, run_soa_analysis_with_config_and_source_path_and_abort};
pub use sparameter::{
    SParameterPort, SParameterRunConfig, SParameterSweep,
    run_sparameter_analysis_with_source_path_and_abort,
};
pub use stb::run_stb_analysis_with_sweep_and_source_path_and_abort;
pub use sweeps::{
    CornerBaseMode, CornerFrequencySweep, CornerModelBinding, CornerPoint, CornerProcess,
    CornerRunConfig, TempRunConfig, run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
pub(crate) use sweeps::{
    REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END, SweepPointResult,
    apply_run_environment, apply_voltage_corner, expand_corner_pvt_points,
    expand_step_sweep_values, infer_nominal_supply_voltage, map_corner_results,
    map_temperature_results, materialize_corner_process_source,
};
pub use tf::{
    TfAccuracy, TfNormalization, TfQuantity, TfRunConfig,
    run_tf_analysis_with_config_and_source_path_and_abort,
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

/// Construct the authoritative engine used to produce or consume an
/// authenticated periodic operating point.
///
/// `build_engine_config` has already resolved deck options.  Periodic run
/// settings then own the Newton tolerance, so the resulting configuration
/// must be marked resolved; passing it through `Engine::new` would apply the
/// deck a second time and could overwrite that run-owned value.  Producer and
/// dependent-analysis services share this helper so retained-state identity
/// is based on exactly the same configuration contract.
fn build_resolved_periodic_engine(
    netlist: &rspice_core::Netlist,
    tolerance: rspice_core::Value,
    context: &str,
) -> ServiceRunResult<Engine> {
    let mut config = build_engine_config(netlist, None);
    config.tolerance = tolerance;
    Engine::try_new_with_resolved_config(config).map_err(|error| {
        ServiceRunError::from_core(context, rspice_core::SimulationError::Configuration(error))
    })
}

// =============================================================================
// Tests
// =============================================================================
