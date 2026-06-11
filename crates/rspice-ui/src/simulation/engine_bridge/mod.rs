//! Engine Bridge - Connect UI to rspice-core Simulation Engine
//!
//! This module provides a clean interface between the UI simulation runner
//! and the rspice-core Engine. It handles:
//!
//! - Netlist parsing from UI-generated strings
//! - Mapping UI AnalysisConfig to core engine calls
//! - Converting core results to UI WaveformData format
//! - Error translation
//!
//! # Architecture
//!
//! The bridge follows the adapter pattern:
//! ```text
//! UI SimulationRunner -> EngineBridge -> rspice-core Engine
//!      (config)           (parse)         (run)
//!      (result) <-        (convert) <-    (result)
//! ```

use std::path::Path;

use super::config::AnalysisConfig;
use super::results::SimulationResult;
use super::runner::SimulationError;

mod ac_noise;
mod dc;
mod error;
mod measure;
mod parsing;
mod pole_zero;
mod sensitivity;
mod transient;

/// Bridge between UI and rspice-core engine
///
/// Handles parsing, execution, and result conversion for all analysis types.
pub struct EngineBridge {
    /// Core engine instance
    engine: rspice_core::Engine,
}

struct SimulationInput<'a> {
    config: &'a AnalysisConfig,
    netlist_str: &'a str,
    source_path: Option<&'a Path>,
}

impl Default for EngineBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineBridge {
    /// Create a new engine bridge with default configuration
    pub fn new() -> Self {
        Self {
            engine: rspice_core::Engine::default(),
        }
    }

    /// Create a new engine bridge with custom configuration
    pub fn with_config(config: rspice_core::SimulationConfig) -> Self {
        Self {
            engine: rspice_core::Engine::new(config),
        }
    }

    /// Run simulation with the given configuration and netlist
    pub fn run(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            None,
        )
    }

    /// Run simulation with a source path used to resolve relative includes and
    /// model file references.
    pub fn run_with_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            None,
        )
    }

    /// Run simulation with abort signal for cooperative cancellation.
    pub fn run_with_abort(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        abort_flag: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            Some(abort_flag),
        )
    }

    /// Run simulation with cooperative cancellation and a source path for
    /// relative include/model resolution.
    pub fn run_with_abort_and_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
        abort_flag: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            Some(abort_flag),
        )
    }

    fn run_request(
        &self,
        input: SimulationInput<'_>,
        abort_flag: Option<&dyn rspice_core::abort_signal::AbortSignal>,
    ) -> Result<SimulationResult, SimulationError> {
        let netlist = self.parse_netlist_with_source_path(input.netlist_str, input.source_path)?;
        self.dispatch_analysis(input.config, &netlist, abort_flag)
    }

    fn dispatch_analysis(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        abort_flag: Option<&dyn rspice_core::abort_signal::AbortSignal>,
    ) -> Result<SimulationResult, SimulationError> {
        match config {
            AnalysisConfig::DcOp => self.run_dc_op(netlist),
            AnalysisConfig::DcSweep(dc_config) => self.run_dc_sweep(netlist, dc_config),
            AnalysisConfig::Transient(tran_config) => {
                if let Some(abort) = abort_flag {
                    self.run_transient_with_abort(netlist, tran_config, abort)
                } else {
                    self.run_transient(netlist, tran_config)
                }
            }
            AnalysisConfig::Ac(ac_config) => self.run_ac(netlist, ac_config),
            AnalysisConfig::Noise(noise_config) => self.run_noise(netlist, noise_config),
            AnalysisConfig::PoleZero(pz_config) => self.run_pz(netlist, pz_config),
            AnalysisConfig::Sensitivity(sens_config) => self.run_sensitivity(netlist, sens_config),
        }
    }
}
