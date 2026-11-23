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
    /// Where in `message` the parser's own words begin, after the line it
    /// names: what a finding on that line shows. Zero when the message is
    /// all one finding. An offset rather than a copy, so a failure stays as
    /// small as a `Result` should carry.
    finding_from: u32,
    interruption: Option<PreparationInterruption>,
    refusal: Option<PreparationRefusal>,
    /// The SPICE directive note line the failure is on, where a note wrote
    /// the card that failed.
    directive_note: Option<Box<crate::netlist_gen::DirectiveNoteLine>>,
}

/// What a failure that refuses a deck, rather than finding it wrong, says
/// the host cannot do: the capability by its token, and the instances that
/// need it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparationRefusal {
    pub capability: &'static str,
    pub instances: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PreparationInterruption {
    Aborted,
    ResourceLimit(rspice_core::ResourceLimitError),
    /// A limit sized to the machine the deck runs on, which the message
    /// says in the reader's words rather than the engine's.
    SizedLimit(rspice_core::ResourceLimitError),
}

impl PreparationError {
    pub fn new(stage: PreparationStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
            line: None,
            finding_from: 0,
            interruption: None,
            refusal: None,
            directive_note: None,
        }
    }

    /// The same failure, on a line of a SPICE directive note.
    pub fn on_directive_note(mut self, place: crate::netlist_gen::DirectiveNoteLine) -> Self {
        self.directive_note = Some(Box::new(place));
        self
    }

    /// The same failure, on the directive note line `deck` wrote at the deck
    /// line the failure names, when a note wrote that line, and said there:
    /// the note's line, not the deck's, is the one its reader can find.
    pub fn located_in(self, deck: &str) -> Self {
        let Some(place) = self
            .line
            .and_then(|line| crate::netlist_gen::directive_note_at_deck_line(deck, line))
        else {
            return self;
        };
        let at = match &place.cell_view {
            Some(cell_view) => format!("SPICE directive on {}", cell_view.display_path()),
            None => "SPICE directive".to_owned(),
        };
        let message = format!("{at}, line {}: {}", place.line, self.finding());
        self.said_as(message).on_directive_note(place)
    }

    /// The SPICE directive note line the failure is on, if a note wrote the
    /// card that failed.
    pub fn directive_note(&self) -> Option<&crate::netlist_gen::DirectiveNoteLine> {
        self.directive_note.as_deref()
    }

    /// The same failure, as a refusal: the host lacks `capability`, which
    /// `instances` need.
    pub fn refusing(mut self, capability: &'static str, instances: Vec<String>) -> Self {
        self.refusal = Some(PreparationRefusal {
            capability,
            instances,
        });
        self
    }

    /// The capability this failure says the host lacks, when it refuses the
    /// deck for one.
    pub fn refusal(&self) -> Option<&PreparationRefusal> {
        self.refusal.as_ref()
    }

    /// The same failure, said as `message`.
    pub(crate) fn said_as(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self.finding_from = 0;
        self
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

    /// What is wrong, without where: for a syntax error the parser's own
    /// words, which a caller that names the [`line`](Self::line) puts after
    /// it; else the whole message.
    pub fn finding(&self) -> &str {
        usize::try_from(self.finding_from)
            .ok()
            .and_then(|from| self.message.get(from..))
            .unwrap_or(&self.message)
    }

    pub fn is_aborted(&self) -> bool {
        matches!(self.interruption, Some(PreparationInterruption::Aborted))
    }

    pub fn resource_limit(&self) -> Option<&rspice_core::ResourceLimitError> {
        match &self.interruption {
            Some(
                PreparationInterruption::ResourceLimit(error)
                | PreparationInterruption::SizedLimit(error),
            ) => Some(error),
            _ => None,
        }
    }

    /// The limit this failure went past when a host says it in its own
    /// catalogue's words. `None` for one sized to the machine the deck runs
    /// on, which the [`message`](Self::message) already says: that the
    /// machine has not the memory for the deck.
    pub fn catalogued_limit(&self) -> Option<&rspice_core::ResourceLimitError> {
        match &self.interruption {
            Some(PreparationInterruption::ResourceLimit(error)) => Some(error),
            _ => None,
        }
    }

    /// The same failure past a limit sized to the machine, said as `words`.
    pub(crate) fn sized_as(mut self, words: impl Into<String>) -> Self {
        if let Some(PreparationInterruption::ResourceLimit(limit)) = self.interruption {
            self.interruption = Some(PreparationInterruption::SizedLimit(limit));
        }
        self.said_as(words)
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
        match error {
            ParseWithAbortError::Aborted => {
                let mut failure = Self::new(stage, format!("{context}: {error}"));
                failure.interruption = Some(PreparationInterruption::Aborted);
                failure
            }
            ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                let mut failure = Self::new(stage, format!("{context}: {error}"));
                failure.interruption = Some(PreparationInterruption::ResourceLimit(error));
                failure
            }
            // A deck the parser refuses is wrong as written: the parser's
            // words say what and where, once, with no step of the
            // preparation in front of them.
            ParseWithAbortError::Parse(error) => {
                let mut failure = Self::new(stage, error.to_string());
                failure.line = crate::execution::parse_error_line(&error);
                if let ParseError::Syntax { message, .. } = &error
                    && let Some(from) = failure.message.len().checked_sub(message.len())
                    && failure.message.ends_with(message.as_str())
                {
                    failure.finding_from = u32::try_from(from).unwrap_or(0);
                }
                failure
            }
        }
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
///
/// The check reads the whole hierarchy below `root`, which names the root's
/// own cellview: under a configuration it reads what the deck runs, and an
/// instance of the root inside itself closes its cycle there. The run is
/// refused on an Error or a Critical finding and on nothing milder; a warning
/// travels with the run rather than stopping it.
///
/// `config` is the configuration the project checks `root` under, from the
/// caller that owns the project: its rule deck and its connectivity policy.
/// The gate does not build its own. When it did, a rule the deck demoted
/// still refused the run, and one it promoted to an error passed every run
/// path that skipped preflight.
pub fn check_generated_design(
    schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
    root: &rspice_design_model::cell_view::CellViewRef,
    hierarchy: &rspice_design::hierarchy::HierarchySource<'_>,
    config: rspice_design::drc::DrcConfig,
) -> Result<rspice_design::drc::DrcResult, PreparationError> {
    let drc = rspice_design::drc::run_check_of_cellview(
        schematic,
        root,
        hierarchy,
        config,
        rspice_design::drc::CheckReach::Hierarchy,
    );
    if !drc.completed {
        return Err(PreparationError::new(
            PreparationStage::DesignChecks,
            "Schematic source checks did not complete",
        ));
    }
    if drc.has_errors() {
        // Each finding is named, as the design check words it: a run refused
        // with only a count sends the engineer to the checks to learn why,
        // and a headless run has no checks panel to go to.
        let summary = drc.summary();
        let findings = drc
            .errors()
            .iter()
            .map(|violation| violation.message.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(PreparationError::new(
            PreparationStage::DesignChecks,
            format!(
                "Fix schematic source-check errors before simulation ({} critical, {} error{}): \
                 {findings}",
                summary.critical,
                summary.errors,
                if summary.errors == 1 { "" } else { "s" }
            ),
        ));
    }
    Ok(drc)
}

mod periodic_sources;
pub(crate) use periodic_sources::validate_prepared_periodic_sources;
mod technology;
pub mod touchstone;
pub use technology::{TechnologyDemand, TechnologyDemandReason, technology_demand};
