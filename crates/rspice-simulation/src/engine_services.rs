//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use rspice_core::engine::Engine;

mod context;
pub use context::ServiceContext;
mod dcmatch;
pub use dcmatch::run_dc_mismatch_analysis_with_context;
mod disto;
pub use disto::run_disto_analysis_with_context;
mod envelope_fourier;
mod hb;
mod hbnoise;
mod helpers;
mod monte_carlo;
mod optimization;
mod pac_pxf;
mod periodic_carrier;
mod pnoise;
mod psp;
mod pss;
mod pstb;
mod qpac;
mod qpnoise;
mod qpss;
mod qpxf;
pub(crate) use qpac::run_qpac_analysis_from_qpss_on_materialized_with_abort;
pub(crate) use qpac::run_qpac_analysis_from_qpss_on_materialized_with_context;
pub(crate) use qpnoise::run_qpnoise_analysis_from_qpss_on_materialized_with_abort;
pub(crate) use qpnoise::run_qpnoise_analysis_from_qpss_on_materialized_with_context;
#[cfg(test)]
pub use qpnoise::run_qpnoise_analysis_from_qpss_with_source_path_and_abort;
pub(crate) use qpss::{
    run_qpss_analysis_on_materialized_with_abort,
    run_qpss_analysis_with_dc_seed_on_materialized_with_abort,
    run_qpss_analysis_with_dc_seed_on_materialized_with_context,
};
pub(crate) use qpxf::run_qpxf_analysis_from_qpss_on_materialized_with_abort;
pub(crate) use qpxf::run_qpxf_analysis_from_qpss_on_materialized_with_context;
mod soa;
mod sparameter;
pub use sparameter::run_sparameter_analysis_with_context;
mod stb;
pub use stb::run_stb_analysis_with_context;
mod sweeps;
mod tf;
pub use tf::run_tf_analysis_with_context;
mod transient;
// Each analysis re-exports the request types a caller must construct and the
// entry point it calls. Result types are not re-exported: callers receive them
// from the entry point and never name them here.
pub use crate::error::{ServiceRunError, ServiceRunResult};
pub use disto::{DistoFrequencySweep, DistoRunConfig};
#[cfg(test)]
pub(crate) use envelope_fourier::run_fourier_from_signal_with_abort;
#[cfg(test)]
pub use envelope_fourier::{EnvelopeInitializationConfig, EnvelopeShootingIntegration};
pub use envelope_fourier::{
    EnvelopeRunConfig, FourierData, FourierRunConfig,
    run_envelope_analysis_with_source_path_and_abort,
};
pub(crate) use envelope_fourier::{
    fourier_output_is_current, run_fourier_from_impulses_with_abort, split_fourier_output,
};
pub(crate) use hb::HbData;
pub(crate) use hb::HbSpectrum;
pub(crate) use hb::run_hb_analysis_on_materialized_with_abort;
pub(crate) use hb::run_hb_analysis_with_dc_seed_on_materialized_with_abort;
pub(crate) use hb::run_hb_analysis_with_dc_seed_on_materialized_with_context;
#[cfg(test)]
pub use hb::run_hb_analysis_with_source_path_and_abort;
pub use hb::{HbRunConfig, HbToneRunConfig};
pub use hbnoise::{HbNoiseReference, HbnoiseFrequencySweep, HbnoiseRunConfig};
pub(crate) use hbnoise::{integrate_psd, run_hbnoise_analysis_from_hb_on_materialized_with_abort};
pub(crate) use helpers::parse_runner_netlist_with_abort;
#[cfg(test)]
use helpers::parse_runner_netlist_with_statistical_sampling_and_abort;
use helpers::{
    build_voltage_output_expr, generate_freq_points_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, normalize_voltage_signal_name,
};
pub(crate) use monte_carlo::{MonteCarloData, finish_monte_carlo_result};
#[cfg(test)]
pub(crate) use monte_carlo::{
    run_monte_carlo_analysis_with_environment_and_source_path_and_abort,
    run_statistical_monte_carlo_with_environment_and_source_path_and_abort,
};
pub use optimization::{
    OptimizationAlgorithmMode, OptimizationGoalMode, OptimizationRunConfig, OptimizationVariable,
    run_optimization_analysis_with_config_and_source_path_and_abort,
};
pub(crate) use optimization::{
    OptimizationData, OptimizationEvaluation, materialize_optimization_candidate,
    objective_to_cost as optimization_objective_cost,
    run_optimization_analysis_with_environment_and_source_path_and_abort,
    run_optimization_with_cost_evaluator,
};
pub(crate) use pac_pxf::{
    PacData, PxfData, run_pac_analysis_on_materialized_with_abort,
    run_pxf_analysis_on_materialized_with_abort,
};
#[cfg(test)]
pub use pac_pxf::{
    run_pac_analysis_from_hb_with_source_path_and_abort,
    run_pxf_analysis_from_hb_with_source_path_and_abort,
};
#[cfg(test)]
pub use periodic_carrier::PeriodicCarrier;
pub(crate) use periodic_carrier::PeriodicCarrierState;
pub(crate) use pnoise::PnoiseData;
#[cfg(test)]
pub use pnoise::run_pnoise_analysis_from_hb_with_source_path_and_abort;
pub(crate) use pnoise::run_pnoise_analysis_on_materialized_with_abort;
pub(crate) use psp::run_psp_analysis_from_pss_on_materialized_with_abort;
pub(crate) use psp::{PspData, run_hbsp_analysis_from_hb_on_materialized_with_abort};
pub use psp::{PspRunConfig, PspSweep};
pub(crate) use pss::PssData;
pub(crate) use pss::run_pss_analysis_on_materialized_with_abort;
#[cfg(test)]
pub(crate) use pss::run_pss_analysis_with_config_and_source_path_and_abort;
pub use pss::{PssRunConfig, run_pss_analysis_with_source_path_and_abort};
pub(crate) use pss::{PssSeedEnvironment, run_pss_analysis_with_dc_seed_and_context};
pub(crate) use pstb::{PstbData, run_pstb_analysis_on_materialized_with_abort};
#[cfg(test)]
pub use qpss::run_qpss_analysis_with_source_path_and_abort;
// DC sweep, noise, pole-zero, and sensitivity have no entry here, and that is
// the module boundary rather than an omission. The seven fundamental analyses
// -- DC op, DC sweep, transient, AC, noise, pole-zero, sensitivity -- ship
// through `simulation::engine_bridge`, dispatched from `AnalysisConfig`. This
// module is the RF and advanced layer. Duplicates of all four once sat here
// unreachable; adding a fifth would mean the same thing again.
pub use soa::{SoaRunConfig, run_soa_analysis_with_context};
pub use sparameter::{SParameterPort, SParameterRunConfig, SParameterSweep};
pub use sweeps::{
    run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
pub use tf::{TfAccuracy, TfNormalization, TfQuantity, TfRunConfig, infer_tf_run_config};
pub use transient::TransientData;
#[cfg(test)]
pub use transient::run_transient_analysis_with_source_path_and_abort;

// =============================================================================
// Platform-agnostic timing utilities
// =============================================================================

/// Get current time in milliseconds (for performance measurement)
#[cfg(test)]
fn now_ms() -> f64 {
    web_time::SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0
}

#[allow(dead_code, reason = "retained by Monte Carlo compatibility runners")]
const DEFAULT_MONTE_CARLO_SEED: u64 = 0x5EED_5EED;

pub(crate) use crate::netlist_preparation::build_engine_config;

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
    ServiceContext {
        source_path: None,
        limits: Default::default(),
        abort: &rspice_core::NoAbort,
    }
    .periodic_engine(netlist, tolerance, context)
}

// =============================================================================
// Tests
// =============================================================================
