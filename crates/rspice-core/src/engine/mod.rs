//! Simulation Engine - Wires netlist -> circuit -> solver -> results
//!
//! This module provides the main simulation loop that connects all components.
//!
//! # Architecture
//!
//! The engine is organized into focused submodules. They are private: the
//! public surface is [`Engine`] plus the configuration, result, and error
//! types re-exported below.
//!
//! Orchestration and setup:
//!
//! - `core` - the [`Engine`] type itself and shared orchestration helpers
//! - `builder` - circuit construction from a parsed netlist
//! - `config`, `config_resolver` - simulation/convergence configuration and
//!   the precedence rules that resolve it
//! - `matrix`, `stamping` - MNA topology reservation and DC stamping
//! - `source_values` - independent-source value evaluation over time/frequency
//! - `convergence` - Newton driver plus the GMIN/source/pseudo-transient aids
//! - `result`, `error`, `health` - result structures, typed errors, and the
//!   [`EngineHealthReport`] diagnostics summary
//!
//! Analysis drivers, one per family:
//!
//! - `dc` - operating point and DC sweep
//! - `ac` - frequency-domain small-signal sweep
//! - `transient` - time-domain integration, including checkpoint/resume
//! - `distortion` - third-order Volterra distortion (`.DISTO`)
//! - `hb` - harmonic balance
//! - `pss`, `pss_noise` - periodic steady state and periodic noise
//! - `stb` - loop-gain stability
//! - `transfer` - small-signal transfer function (`.TF`)
//! - `advanced` - noise, Monte Carlo, pole-zero, sensitivity, and `.STEP`

mod ac;
mod advanced;
mod builder;
pub(crate) use builder::{
    XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION, build_native_xyce_memristor,
    validate_native_xyce_ltra_model_contract,
};
mod config;
mod config_resolver;
mod convergence;
mod core;
mod dc;
mod distortion;
mod error;
mod hb;
mod health;
mod matrix;
mod pss;
mod pss_noise;
mod result;
mod source_values;
mod stamping;
mod stb;
mod transfer;
mod transient;

// Re-export CompressionConfig for public API
pub use crate::analysis::waveform::CompressionConfig;
pub use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
pub use advanced::{MaterializedStepRun, StepPlan, StepPlanLimits};
#[cfg(feature = "veriloga")]
pub use builder::{
    ProjectVerilogARuntimeRegistration, VerilogACacheEntry, VerilogACachePruneReport,
    VerilogACacheStats, clear_veriloga_cache, prune_veriloga_cache,
    register_precompiled_veriloga_model, register_precompiled_veriloga_model_with_dependencies,
    register_precompiled_veriloga_runtime_with_dependencies,
    register_project_veriloga_runtime_for_session, register_project_veriloga_runtimes_for_session,
    register_project_veriloga_runtimes_for_session_with_limits, veriloga_cache_entries,
    veriloga_cache_stats,
};
pub use config::{
    BypassConfig, ConvergenceConfig, DampingStrategy, JfetLevel2Model, SimulationConfig,
    SimulationConfigError, SpiceDialect, XyceTraInterpolation,
};
pub use config_resolver::{
    ConvergencePreset, SimulationConfigOverrides, resolve_simulation_config,
};
pub use core::Engine;
pub use dc::DcSweepPointResult;
pub(crate) use dc::bounded_dc_sweep_points;
pub use error::{
    SimulationError, SimulationErrorCategory, SimulationErrorCode, SimulationErrorDescriptor,
};
pub use hb::{
    HbAnalysisResult, HbEnvelopeContinuationState, HbEnvelopeStateGuarantee, PacAnalysisResult,
    PnoiseAnalysisResult,
};
pub use health::EngineHealthReport;
pub use pss::{
    PssAnalysisResult, PssContinuationState, PssDcOperatingPointSeed, PssError, PssOperatingPoint,
};
pub use pss_noise::OscPnoiseResult;
pub use result::{TransientDeviceOpTrace, TransientResult, TransientStoreTrace};
pub use stb::StbAnalysisResult;
pub use transient::{TransientCheckpoint, netlist_fingerprint};

pub(crate) use source_values::{extract_ac_value, extract_dc_value, extract_dc_value_with_limits};
