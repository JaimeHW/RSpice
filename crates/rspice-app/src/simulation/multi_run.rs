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

mod spec;

#[cfg(test)]
pub use rspice_simulation_contract::config::FrequencySweep;
#[cfg(test)]
pub use spec::SpPort;
pub use spec::{AnalysisSpec, PssMethod};
#[cfg(test)]
pub use spec::{EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve};
