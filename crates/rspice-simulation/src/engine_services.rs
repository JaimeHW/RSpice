//! Engine services
//!
//! One entry point per analysis over rspice-core: each runs the analysis on a
//! deck or an already parsed netlist, observes the caller's abort signal, and
//! translates the engine's result and errors. The runner, the engine bridge
//! and the study executors call them, so the desktop application, the browser
//! worker and the CLI reach the engine through the same code.

mod context;
pub use context::ServiceContext;
mod dcmatch;
pub use dcmatch::run_dc_mismatch_analysis_with_context;
mod disto;
pub(crate) use disto::run_disto_analysis_with_bias;
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
pub(crate) use qpac::run_qpac_analysis_from_qpss_on_materialized_with_context;
pub(crate) use qpnoise::run_qpnoise_analysis_from_qpss_on_materialized_with_context;
pub(crate) use qpss::run_qpss_analysis_with_dc_seed_on_materialized_with_context;
pub(crate) use qpxf::run_qpxf_analysis_from_qpss_on_materialized_with_context;
mod soa;
mod sparameter;
pub(crate) use sparameter::run_sparameter_analysis_with_bias;
mod stb;
pub(crate) use stb::run_stb_analysis_with_bias;
mod sweeps;
#[cfg(test)]
pub(crate) mod test_entry;
mod tf;
pub(crate) use tf::run_tf_analysis_with_bias;
mod transient;
mod transient_state;
pub(crate) use transient_state::{
    Bias, StateCard, StateRunError, TransientStateHandoff, TransientStateRole,
    install_transient_state_handoff, linearize_at_state, publish_transient_states,
    state_capture_times,
};
// Only a native runner keeps a store; the browser worker runs each card's own
// transient.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use transient_state::TransientStateStore;
// Each analysis re-exports the request types a caller must construct and the
// entry point it calls. Result types are not re-exported: callers receive them
// from the entry point and never name them here.
pub use crate::error::{ServiceRunError, ServiceRunResult};
pub use disto::DistoRunConfig;
#[cfg(test)]
pub(crate) use envelope_fourier::run_fourier_from_signal_with_abort;
#[cfg(test)]
pub use envelope_fourier::{EnvelopeInitializationConfig, EnvelopeShootingIntegration};
pub use envelope_fourier::{
    EnvelopeRunConfig, FourierData, FourierRunConfig, run_envelope_analysis_with_context,
};
pub(crate) use envelope_fourier::{
    fourier_output_is_current, run_fourier_from_impulses_with_abort, split_fourier_output,
};
pub(crate) use hb::HbData;
pub(crate) use hb::HbSpectrum;
pub(crate) use hb::run_hb_analysis_with_dc_seed_on_materialized_with_context;
pub use hb::{HbRunConfig, HbToneRunConfig};
pub use hbnoise::{HbNoiseReference, HbnoiseRunConfig};
pub(crate) use hbnoise::{
    integrate_psd, run_hbnoise_analysis_from_hb_on_materialized_with_context,
};
pub(crate) use helpers::parse_runner_netlist_with_abort;
use helpers::{
    build_voltage_output_expr, is_ground_like, netlist_has_independent_source_named_with_abort,
    normalize_voltage_signal_name,
};
pub(crate) use monte_carlo::{MonteCarloData, finish_monte_carlo_result};
pub use optimization::{OptimizationAlgorithmMode, OptimizationGoalMode, OptimizationRunConfig};
pub(crate) use optimization::{
    OptimizationData, OptimizationEvaluation, materialize_optimization_candidate,
    objective_to_cost as optimization_objective_cost, run_optimization_analysis_with_context,
    run_optimization_with_cost_evaluator,
};
pub(crate) use pac_pxf::{
    PacData, PxfData, run_pac_analysis_on_materialized_with_context,
    run_pxf_analysis_on_materialized_with_context,
};
#[cfg(test)]
pub use periodic_carrier::PeriodicCarrier;
pub(crate) use periodic_carrier::PeriodicCarrierState;
pub(crate) use pnoise::PnoiseData;
pub(crate) use pnoise::run_pnoise_analysis_on_materialized_with_context;
pub use psp::PspRunConfig;
pub(crate) use psp::{
    PspData, run_hbsp_analysis_from_hb_on_materialized_with_context,
    run_psp_analysis_from_pss_on_materialized_with_context,
};
pub use pss::PssRunConfig;
pub(crate) use pss::{PssData, run_pss_analysis_on_materialized_with_context};
pub(crate) use pss::{PssSeedEnvironment, run_pss_analysis_with_dc_seed_and_context};
pub(crate) use pstb::{PstbData, run_pstb_analysis_on_materialized_with_context};
// DC sweep, noise, pole-zero, and sensitivity have no entry here, and that is
// the module boundary rather than an omission. The seven fundamental analyses
// -- DC op, DC sweep, transient, AC, noise, pole-zero, sensitivity -- ship
// through `simulation::engine_bridge`, dispatched from `AnalysisConfig`. This
// module is the RF and advanced layer. Duplicates of all four once sat here
// unreachable; adding a fifth would mean the same thing again.
pub use soa::{SoaRunConfig, run_soa_analysis_with_context};
pub use sparameter::{SParameterPort, SParameterRunConfig};
pub use sweeps::{
    run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
pub use tf::{TfAccuracy, TfQuantity, TfRunConfig, infer_tf_run_config};
pub use transient::TransientData;

pub(crate) use crate::netlist_preparation::build_engine_config;

// =============================================================================
// Tests
// =============================================================================
