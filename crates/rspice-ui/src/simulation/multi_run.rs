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

pub use run_state::RunQueue;
pub use run_type::{AnalysisRunType, FrequencySweep};
pub use spec::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, PssMethod, SpPort,
    TfAccuracy, TfNormalization,
};
