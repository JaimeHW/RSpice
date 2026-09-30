//! Analysis preparation inputs and typed failures shared by source, model and dispatch checks.

use crate::execution_options::SpecExecutionOptions;
use rspice_simulation_contract::{
    analysis_spec::AnalysisSpec, config::AnalysisConfig, numeric_override::AnalysisNumericOverride,
};

/// An authored analysis awaiting snapshot validation and execution authorization.
#[derive(Debug, Clone)]
pub struct QueuedAnalysis {
    pub spec: AnalysisSpec,
    pub config: Option<AnalysisConfig>,
    pub spec_options: SpecExecutionOptions,
    pub analysis_line: String,
    /// Numerical departures authored against this analysis. Snapshot
    /// preparation turns them into a second `.OPTIONS` block in this task's own
    /// deck; a manual deck states its options in the deck itself and therefore
    /// never carries one.
    pub numeric_override: Option<AnalysisNumericOverride>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparationStage {
    DesignChecks,
    SourceChecks,
    AnalysisPlan,
    ModelBindings,
    Netlist,
    Authorization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparationError {
    stage: PreparationStage,
    message: String,
    /// The 1-based deck line this failure named, where it named one.
    line: Option<usize>,
}

impl PreparationError {
    pub fn new(stage: PreparationStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
            line: None,
        }
    }

    /// The same failure, at the line the parser reported it on.
    pub fn at_line(mut self, line: Option<usize>) -> Self {
        self.line = line;
        self
    }

    pub const fn stage(&self) -> PreparationStage {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn line(&self) -> Option<usize> {
        self.line
    }
}

impl std::fmt::Display for PreparationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

mod periodic_sources;
pub use periodic_sources::validate_prepared_periodic_sources;
pub mod touchstone;
