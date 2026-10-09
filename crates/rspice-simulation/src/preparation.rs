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
    interruption: Option<PreparationInterruption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PreparationInterruption {
    Aborted,
    ResourceLimit(rspice_core::ResourceLimitError),
}

impl PreparationError {
    pub fn new(stage: PreparationStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
            line: None,
            interruption: None,
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

    pub fn is_aborted(&self) -> bool {
        matches!(self.interruption, Some(PreparationInterruption::Aborted))
    }

    pub fn resource_limit(&self) -> Option<&rspice_core::ResourceLimitError> {
        match &self.interruption {
            Some(PreparationInterruption::ResourceLimit(error)) => Some(error),
            _ => None,
        }
    }

    pub(crate) fn check_abort(
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<(), Self> {
        if abort.is_aborted() {
            let mut error = Self::new(PreparationStage::SourceChecks, "Preparation aborted");
            error.interruption = Some(PreparationInterruption::Aborted);
            Err(error)
        } else {
            Ok(())
        }
    }

    pub(crate) fn from_parse(
        stage: PreparationStage,
        context: &str,
        error: rspice_core::netlist::ParseWithAbortError,
    ) -> Self {
        use rspice_core::netlist::{ParseError, ParseWithAbortError};
        let mut failure = Self::new(stage, format!("{context}: {error}"));
        match error {
            ParseWithAbortError::Aborted => {
                failure.interruption = Some(PreparationInterruption::Aborted);
            }
            ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                failure.interruption = Some(PreparationInterruption::ResourceLimit(error));
            }
            ParseWithAbortError::Parse(error) => {
                failure.line = crate::execution::parse_error_line(&error);
            }
        }
        failure
    }

    /// Keep admission failures typed when a prepared analysis performs bounded
    /// elaboration or checkpoint validation before execution authorization.
    pub(crate) fn from_simulation(
        stage: PreparationStage,
        context: &str,
        error: crate::error::SimulationError,
    ) -> Self {
        use crate::error::SimulationError;
        let mut failure = Self::new(stage, format!("{context}: {error}"));
        failure.interruption = match error {
            SimulationError::Aborted => Some(PreparationInterruption::Aborted),
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => rspice_core::ResourceKind::from_name(&resource).map(|resource| {
                PreparationInterruption::ResourceLimit(rspice_core::ResourceLimitError {
                    resource,
                    requested,
                    limit,
                })
            }),
            _ => None,
        };
        failure
    }

    pub(crate) fn check_limit(
        resource: rspice_core::ResourceKind,
        requested: usize,
        limit: usize,
    ) -> Result<(), Self> {
        if requested > limit {
            Err(Self::from_parse(
                PreparationStage::SourceChecks,
                "Prepared source exceeds its resource policy",
                rspice_core::netlist::ParseError::ResourceLimit(rspice_core::ResourceLimitError {
                    resource,
                    requested,
                    limit,
                })
                .into(),
            ))
        } else {
            Ok(())
        }
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
mod technology;
pub mod touchstone;
pub use technology::{TechnologyDemand, TechnologyDemandReason, technology_demand};
