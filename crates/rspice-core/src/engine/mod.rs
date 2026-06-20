//! Simulation Engine - Wires netlist -> circuit -> solver -> results
//!
//! This module provides the main simulation loop that connects all components.
//!
//! # Architecture
//!
//! The engine is organized into focused submodules:
//!
//! - [`core`] - public `Engine` type and shared orchestration helpers
//! - [`config`] - simulation and convergence configuration
//! - [`result`] - public result structures returned by engine runs
//! - [`builder`] - circuit construction from netlist
//! - [`matrix`] - matrix topology building
//! - [`stamping`] - DC stamping functions
//! - [`convergence`] - Newton-Raphson and convergence helpers
//! - [`dc`] - DC operating point and sweep analysis
//! - [`transient`] - Time-domain transient simulation
//! - [`ac`] - Frequency-domain AC analysis
//! - [`advanced`] - Noise, Monte Carlo, pole-zero, sensitivity analysis

mod ac;
mod advanced;
mod behavioral_expr;
mod builder;
mod config;
mod config_resolver;
mod convergence;
mod core;
mod dc;
mod error;
mod hb;
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
#[cfg(feature = "veriloga")]
pub use builder::{
    VerilogACacheEntry, VerilogACachePruneReport, VerilogACacheStats, clear_veriloga_cache,
    prune_veriloga_cache, register_precompiled_veriloga_model,
    register_precompiled_veriloga_model_with_dependencies, veriloga_cache_entries,
    veriloga_cache_stats,
};
pub use config::{
    BypassConfig, ConvergenceConfig, DampingStrategy, JfetLevel2Model, SimulationConfig,
    SpiceDialect,
};
pub use config_resolver::{
    ConvergencePreset, SimulationConfigOverrides, resolve_simulation_config,
};
pub use core::Engine;
pub use error::SimulationError;
pub use hb::{HbAnalysisResult, PacAnalysisResult, PnoiseAnalysisResult};
pub use pss::{PssAnalysisResult, PssError};
pub use pss_noise::OscPnoiseResult;
pub use result::TransientResult;
pub use stb::StbAnalysisResult;
pub use transient::{TransientCheckpoint, netlist_fingerprint};

pub(crate) use source_values::{extract_ac_value, extract_dc_value};
