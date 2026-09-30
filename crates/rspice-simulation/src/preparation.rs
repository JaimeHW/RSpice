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

/// Run the generated design's source checks before snapshot preparation.
pub fn check_generated_design(
    schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
    hierarchy: &rspice_design::hierarchy::HierarchySource<'_>,
) -> Result<rspice_design::drc::DrcResult, PreparationError> {
    let drc = rspice_design::drc::run_check_with_hierarchy(
        schematic,
        hierarchy,
        rspice_design::drc::DrcConfig {
            check_missing_ground: true,
            ..rspice_design::drc::DrcConfig::default()
        },
    );
    if !drc.completed {
        return Err(PreparationError::new(
            PreparationStage::DesignChecks,
            "Schematic source checks did not complete",
        ));
    }
    if drc.has_errors() {
        let summary = drc.summary();
        return Err(PreparationError::new(
            PreparationStage::DesignChecks,
            format!(
                "Fix schematic source-check errors before simulation ({} critical, {} error{})",
                summary.critical,
                summary.errors,
                if summary.errors == 1 { "" } else { "s" }
            ),
        ));
    }
    Ok(drc)
}

mod periodic_sources;
pub use periodic_sources::validate_prepared_periodic_sources;
pub mod touchstone;
