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
//! - `noise`, `monte_carlo`, `pole_zero`, `sensitivity`, `step` - the rest,
//!   one module each
//!
//! Those last five used to sit together under an `advanced` submodule, which
//! drew a line no rule justified: `distortion` and `stb` are no less advanced
//! than `noise`, and the split meant knowing which of two directories to look
//! in. Every driver is now a sibling.

mod ac;
mod builder;
pub use builder::{
    XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION, build_native_xyce_memristor,
    validate_native_xyce_ltra_model_contract,
};

mod config_resolver;
mod convergence;
mod core;
mod dc;
mod distortion;
mod error;
mod hb;
mod health;
mod matrix;
mod monte_carlo;
mod noise;
mod pole_zero;
mod pss;
mod pss_noise;
mod result;
mod sensitivity;
mod source_values;
mod stamping;
mod stb;
mod step;
mod transfer;
mod transient;
pub mod waveform;

pub use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
pub use step::{MaterializedStepRun, StepPlan, StepPlanLimits};
#[cfg(feature = "veriloga")]
pub use builder::{
    ProjectVerilogARuntimeRegistration, VerilogACacheEntry, VerilogACachePruneReport,
    VerilogACacheStats, VerilogACacheTelemetry, clear_veriloga_cache, prune_veriloga_cache,
    register_precompiled_veriloga_model, register_precompiled_veriloga_model_with_dependencies,
    register_precompiled_veriloga_runtime_with_dependencies,
    register_project_veriloga_runtime_for_session, register_project_veriloga_runtimes_for_session,
    register_project_veriloga_runtimes_for_session_with_limits, veriloga_cache_entries,
    veriloga_cache_stats, veriloga_cache_telemetry,
};
pub use crate::config::{
    BypassConfig, ConvergenceConfig, DampingStrategy, JfetLevel2Model, SimulationConfig,
    SimulationConfigError, SpiceDialect, XyceTraInterpolation,
};
pub use config_resolver::{
    ConvergencePreset, SimulationConfigOverrides, resolve_simulation_config,
};
pub use core::Engine;
pub use dc::{DcSweepPointResult, bounded_dc_sweep_points, canonical_device_parameter_sweep_source};
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
pub use waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
pub use stb::StbAnalysisResult;
pub use transient::{TransientCheckpoint, netlist_fingerprint};

/// Independent-source introspection: the DC and AC quantities a parsed
/// [`SourceSpec`](crate::netlist::SourceSpec) carries, as the engine reads
/// them. Frontends showing a source's operating value need the engine's own
/// interpretation rather than a second parse of the same spec.
pub use source_values::{extract_ac_value, extract_dc_value};
pub(crate) use source_values::extract_dc_value_with_limits;
