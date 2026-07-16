//! Simulation Execution Module
//!
//! Commercial-grade simulation execution and configuration.
//! Manages the complete simulation workflow from netlist generation to result display.

pub mod automation;
pub mod config;
pub mod controller;
pub mod convergence;
pub mod dialog;
pub mod engine_bridge;
pub(crate) mod execution;
pub mod multi_run;
pub mod netlist_gen;
pub mod netlist_viewer;
pub mod optimizer;
pub mod options_translator;
pub(crate) mod output_contract;
pub use output_contract::{
    SavedOutputPreflightReport, SavedOutputSemanticStatus, SavedOutputStorageEstimate,
    materialize_deferred_saved_output,
};
pub mod plan;
pub mod reliability_engine;
pub mod results;
pub mod runner;
pub mod status;

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
