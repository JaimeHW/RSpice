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

pub use rspice_simulation_contract::analysis_spec::AnalysisSpec;
#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::{PssMethod, SpPort};
#[cfg(test)]
pub use rspice_simulation_contract::config::FrequencySweep;
