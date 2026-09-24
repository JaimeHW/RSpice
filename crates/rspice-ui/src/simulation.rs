//! Simulation Execution Module
//!
//! Commercial-grade simulation execution and configuration.
//! Manages the complete simulation workflow from netlist generation to result display.

pub(crate) mod accuracy;
pub(crate) mod automation;
pub(crate) mod capture_ledger;
pub(crate) mod config;
pub(crate) mod controller;
#[cfg(test)]
pub(crate) mod cost_probe;
pub(crate) mod dependency_contract;
pub(crate) mod dialog;
pub(crate) mod engine_bridge;
pub(crate) mod execution;
pub(crate) mod measurement_references;
pub(crate) mod multi_run;
pub(crate) mod netlist_gen;
pub(crate) mod optimizer;
pub(crate) mod output_contract;
pub use output_contract::{
    SavedOutputPreflightReport, SavedOutputSemanticStatus, SavedOutputStorageEstimate,
};
pub(crate) mod placed_sources;
pub(crate) mod plan;
pub(crate) mod point_family;
pub(crate) mod results;
pub(crate) mod run_set;
pub(crate) mod runner;
pub(crate) mod spice_value;
pub(crate) mod status;
pub(crate) mod stimulus_realize;
pub(crate) mod table_route;
pub(crate) mod veriloga;

// The optimizer, netlist generator, options translator, and
// engine bridge are all reached through their own modules; flattening their
// types here duplicated the path without shortening any call site.
pub use config::AnalysisConfig;
pub use controller::SimulationController;
// Test-only aliases for private modules: execution reaches the bridge and
// these helpers through their module paths.
#[cfg(test)]
pub use engine_bridge::EngineBridge;
#[cfg(test)]
pub use results::{SimulationResult, WaveformData};
pub use runner::SimulationRunner;
pub use status::SimulationStatus;

//=============================================================================
// Tests
//=============================================================================
