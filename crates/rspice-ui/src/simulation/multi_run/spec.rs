//! Multi-run analysis specifications.
//!
//! The typed description of one analysis inside a multi-run job, its run
//! type, and the validation that rejects a specification the engine could
//! not execute.

mod run_type;
mod types;
mod validation;

pub use types::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, PssMethod, SpPort,
    TfAccuracy, TfNormalization,
};
