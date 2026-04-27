//! Multi-Run Orchestration
//!
//! Analysis sequence queuing and automated simulation workflow management.
//!
//! # Features
//!
//! - Queue multiple analyses for sequential execution
//! - Dependency-aware ordering (e.g., DC OP before AC)
//! - Progress tracking with cancellation support
//! - Result aggregation across runs
//! - Corner sweep automation

mod plan;
mod run_state;
mod run_type;
mod spec;

pub use plan::AnalysisPlan;
pub use run_state::{AnalysisRun, RunQueue, RunStatus};
pub use run_type::{AnalysisRunType, FrequencySweep};
pub use spec::{
    AnalysisSpec, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, SpPort,
};
