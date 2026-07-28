//! Simulation Execution Module
//!
//! Commercial-grade simulation execution and configuration.
//! Manages the complete simulation workflow from netlist generation to result display.

pub(crate) mod automation;
pub(crate) mod config;
pub(crate) mod controller;
pub(crate) mod dependency_contract;
pub(crate) mod dialog;
pub(crate) mod engine_bridge;
pub(crate) mod execution;
pub(crate) mod multi_run;
pub(crate) mod netlist_gen;
pub(crate) mod netlist_viewer;
pub(crate) mod optimizer;
pub(crate) mod output_contract;
pub use output_contract::{
    SavedOutputPreflightReport, SavedOutputSemanticStatus, SavedOutputStorageEstimate,
};
pub(crate) mod plan;
pub(crate) mod reliability_engine;
pub(crate) mod results;
pub(crate) mod runner;
pub(crate) mod spice_value;
pub(crate) mod status;
pub(crate) mod veriloga;

// The optimizer, reliability engine, netlist generator, options translator, and
// engine bridge are all reached through their own modules; flattening their
// types here duplicated the path without shortening any call site.
pub use config::AnalysisConfig;
pub use controller::SimulationController;
pub use engine_bridge::EngineBridge;
pub use reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
pub use results::{SimulationResult, WaveformData};
pub use runner::SimulationRunner;
pub use status::SimulationStatus;

//=============================================================================
// Tests
//=============================================================================
