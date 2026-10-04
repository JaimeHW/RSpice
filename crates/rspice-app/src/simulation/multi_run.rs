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

#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::SpPort;
pub use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
#[cfg(test)]
pub use rspice_simulation_contract::config::FrequencySweep;
