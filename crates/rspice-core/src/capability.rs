//! The typed refusal of a well-formed construct this build does not execute.
//!
//! Any layer from configuration up may refuse a construct by name. The refusal
//! names only identities and a source location, so it sits below every layer
//! that raises it, and `analysis::error` re-exports it beside the failure
//! taxonomy that carries it.

use crate::identity::{AnalysisInstanceId, AnalysisKind, RunCoordinateId};
use crate::source_location::NetlistSourceLocation;

/// A well-formed authored construct that this build deliberately refuses.
///
/// This is the typed form of "RSpice understood the deck and declines to run
/// it", which is categorically different from "the deck is wrong". A frontend
/// reports it as a capability gap rather than a user error, and a regression
/// suite can mark the deck expected-unsupported without matching on prose.
///
/// `capability` is a stable, dotted, lowercase token naming the refused
/// boundary (`"analysis.hb.device"`, `"device.ltra.rg_finite_length"`). It is
/// coarser than the message: several messages may share one token, and the
/// token is what automation branches on.
///
/// `analysis_kind`, `instance` and `module` are what a refusal site knows
/// that the token does not: which kind of analysis declined, and the deck
/// instance and module it declined for. They are typed so a frontend can
/// place the refusal on a schematic and in a run log without parsing the
/// detail, which still names all three for a person reading it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedCapabilityError {
    /// Stable dotted token naming the refused capability boundary.
    pub capability: &'static str,
    /// Analysis instance the refusal belongs to, when one is in scope.
    pub analysis: Option<AnalysisInstanceId>,
    /// Kind of analysis that refused, when the site knows the kind but not
    /// the authored card.
    pub analysis_kind: Option<AnalysisKind>,
    /// Run coordinate the refusal belongs to, when one is in scope.
    pub coordinate: Option<RunCoordinateId>,
    /// Deck instance name the refusal belongs to, as elaborated.
    pub instance: Option<String>,
    /// The module or model the refused instance is built from.
    pub module: Option<String>,
    /// Human-facing explanation, naming the construct and the supported
    /// alternative where one exists.
    pub detail: String,
    /// Netlist location that authored the refused construct, when known.
    pub location: Option<NetlistSourceLocation>,
}

impl UnsupportedCapabilityError {
    pub fn new(capability: &'static str, detail: impl Into<String>) -> Self {
        Self {
            capability,
            analysis: None,
            analysis_kind: None,
            coordinate: None,
            instance: None,
            module: None,
            detail: detail.into(),
            location: None,
        }
    }

    #[must_use]
    pub fn with_analysis(mut self, analysis: AnalysisInstanceId) -> Self {
        self.analysis = Some(analysis);
        self
    }

    /// Record the kind of analysis that refused.
    #[must_use]
    pub fn for_analysis_kind(mut self, kind: AnalysisKind) -> Self {
        self.analysis_kind = Some(kind);
        self
    }

    /// Record the deck instance the refusal belongs to.
    #[must_use]
    pub fn for_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Record the module or model the refused instance is built from.
    #[must_use]
    pub fn for_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }

    #[must_use]
    pub fn with_coordinate(mut self, coordinate: RunCoordinateId) -> Self {
        self.coordinate = Some(coordinate);
        self
    }

    #[must_use]
    pub fn at(mut self, location: NetlistSourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl std::fmt::Display for UnsupportedCapabilityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = &self.location {
            write!(formatter, "{location}: ")?;
        }
        write!(formatter, "unsupported capability [{}]", self.capability)?;
        if let Some(analysis) = self.analysis {
            write!(formatter, " in {analysis}")?;
        } else if let Some(kind) = self.analysis_kind {
            write!(formatter, " in {}", kind.tag())?;
        }
        if let Some(coordinate) = self.coordinate {
            write!(formatter, " at run {coordinate}")?;
        }
        if let Some(instance) = &self.instance {
            write!(formatter, " for '{instance}'")?;
        }
        write!(formatter, ": {}", self.detail)
    }
}

impl std::error::Error for UnsupportedCapabilityError {}
