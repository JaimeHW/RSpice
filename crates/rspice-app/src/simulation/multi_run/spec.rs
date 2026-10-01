//! App-facing alias for the portable analysis specification contract.

#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::SpPort;
pub use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
#[cfg(test)]
pub use rspice_simulation_contract::analysis_spec::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
