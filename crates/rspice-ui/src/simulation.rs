//! Simulation Execution Module
//!
//! Commercial-grade simulation execution and configuration.
//! Manages the complete simulation workflow from netlist generation to result display.

pub(crate) mod automation;
pub(crate) mod config;
pub(crate) mod controller;
pub(crate) mod convergence;
pub(crate) mod dependency_contract;
pub(crate) mod dialog;
pub(crate) mod engine_bridge;
pub(crate) mod execution;
pub(crate) mod multi_run;
pub(crate) mod netlist_gen;
pub(crate) mod netlist_viewer;
pub(crate) mod optimizer;
pub(crate) mod options_translator;
pub(crate) mod output_contract;
pub use output_contract::{
    SavedOutputPreflightReport, SavedOutputSemanticStatus, SavedOutputStorageEstimate,
    materialize_deferred_saved_output,
};
pub(crate) mod plan;
pub(crate) mod reliability_engine;
pub(crate) mod results;
pub(crate) mod runner;
pub(crate) mod spice_value;
pub(crate) mod status;
pub(crate) mod veriloga;

pub use automation::{CommandOutput, ScriptExecutor};
pub use optimizer::{
    DesignVar, GoalStrategy, OptimizationGoal, OptimizationResult, OptimizerAlgo, OptimizerEngine,
};
pub use reliability_engine::{
    AgingMechanism, ParamShift, ReliabilityEngine, ReliabilityResult, StressMetrics,
};

pub use config::AnalysisConfig;
pub use controller::SimulationController;
pub use engine_bridge::EngineBridge;
pub use netlist_gen::{
    Net, NetlistGenerator, NetlistResult, generate_netlist, generate_netlist_with_analysis,
};
pub use options_translator::{EngineOptions, OptionsTranslator, PvtCorner};
pub use results::{SimulationResult, WaveformData};
pub use runner::SimulationRunner;
pub use status::{SimulationProgress, SimulationStatus};

//=============================================================================
// Tests
//=============================================================================
