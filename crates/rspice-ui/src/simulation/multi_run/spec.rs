//! Multi-run analysis specifications.
//!
//! The typed description of one analysis inside a multi-run job, its run
//! type, and the validation that rejects a specification the engine could
//! not execute.

mod qpac;
mod qpss;
pub use qpac::QpacControls;
mod run_type;
mod types;
pub use qpss::QpssControls;
mod validation;

pub use types::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, PssMethod,
    SensitivitySweepSpec, SpPort, TfAccuracy, TfNormalization,
};
