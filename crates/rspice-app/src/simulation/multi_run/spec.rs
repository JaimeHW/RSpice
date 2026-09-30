//! App-facing alias for the portable analysis specification contract.

pub use rspice_simulation_contract::analysis_spec::{
    AnalysisSpec, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort, TfAccuracy, TfNormalization,
};
#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
