//! Engine error types.
//!
//! # The taxonomy
//!
//! [`SimulationError`] is the one typed failure channel the engine offers its
//! frontends. Every variant answers [`SimulationError::descriptor`] with a
//! stable [`SimulationErrorCode`] and a coarser [`SimulationErrorCategory`],
//! and the categories are the contract a CLI exit code, a Python exception
//! class, a WASM error object, or an engine-adapter wire code is derived from.
//! A frontend that branches on the display message is reading the wrong thing.
//!
//! The categories separate failures that call for different operator action:
//! invalid authored input, a well-formed construct this build deliberately
//! does not execute, an internal materialization mismatch, an unavailable
//! requested signal, a result-schema mismatch, an exceeded resource budget,
//! cancellation, an expired time budget, an incompatible persisted artifact,
//! and a failed output commit.

use thiserror::Error;

use crate::abort_signal::{AbortReason, AbortSignal};
use crate::identity::{AnalysisInstanceId, AnalysisKind, RunCoordinateId};
use crate::netlist::NetlistSourceLocation;
use crate::resource::ResourceLimitError;
use crate::solver::SolverError;

/// Stable machine-readable code for a simulation failure.
///
/// Display messages are intended for people and may gain additional context.
/// Service and language-binding integrations should branch on this code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SimulationErrorCode {
    /// The supplied [`super::SimulationConfig`] violates an invariant.
    InvalidConfiguration,
    /// A configured resource budget was exceeded.
    ResourceLimit,
    /// Circuit construction or device evaluation failed.
    CircuitError,
    /// A behavioral expression names an invalid node or branch-current operand.
    BehavioralReferenceError,
    /// The numerical solver failed.
    SolverError,
    /// A netlist-dependent simulation operation failed.
    NetlistError,
    /// The authored construct is well formed, but this build does not execute
    /// it.
    UnsupportedCapability,
    /// A materialized run disagrees with the deck plan that produced it.
    MaterializationMismatch,
    /// An authored output symbol is valid, but the selected analysis result
    /// cannot supply it.
    RequestedSignalUnavailable,
    /// An analysis result's signal names and numeric payload disagree with
    /// the schema promised by that result type.
    ResultSchemaMismatch,
    /// A persisted artifact was written by an incompatible format version.
    PersistenceIncompatible,
    /// A completed artifact could not be published to its destination.
    OutputCommitFailed,
    /// An iterative analysis exhausted its convergence strategy.
    ConvergenceError,
    /// The caller cancelled the operation.
    Aborted,
    /// A wall-clock or deadline budget expired before the operation finished.
    TimeLimitExceeded,
}

impl SimulationErrorCode {
    /// Stable snake-case representation used by API and report payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidConfiguration => "invalid_configuration",
            Self::ResourceLimit => "resource_limit",
            Self::CircuitError => "circuit_error",
            Self::BehavioralReferenceError => "behavioral_reference_error",
            Self::SolverError => "solver_error",
            Self::NetlistError => "netlist_error",
            Self::UnsupportedCapability => "unsupported_capability",
            Self::MaterializationMismatch => "materialization_mismatch",
            Self::RequestedSignalUnavailable => "requested_signal_unavailable",
            Self::ResultSchemaMismatch => "result_schema_mismatch",
            Self::PersistenceIncompatible => "persistence_incompatible",
            Self::OutputCommitFailed => "output_commit_failed",
            Self::ConvergenceError => "convergence_error",
            Self::Aborted => "aborted",
            Self::TimeLimitExceeded => "time_limit_exceeded",
        }
    }
}

impl std::fmt::Display for SimulationErrorCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable high-level category for a simulation failure.
///
/// Categories, not variants, are what a frontend maps to an exit code, an
/// exception class, or a wire failure code. Two variants share a category
/// exactly when a caller would take the same action for both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SimulationErrorCategory {
    /// Invalid engine configuration.
    Configuration,
    /// Invalid authored netlist input.
    Netlist,
    /// Well-formed authored input this build deliberately does not execute.
    Capability,
    /// A materialized run does not match the plan that produced it.
    Materialization,
    /// A configured resource budget was exceeded.
    ResourceLimit,
    /// Circuit construction or device evaluation failed.
    Simulation,
    /// The numerical solver failed.
    Solver,
    /// An iterative analysis did not converge.
    Convergence,
    /// A valid authored output symbol is absent from the produced result.
    SignalUnavailable,
    /// A produced result violates its own published schema.
    ResultSchema,
    /// A persisted artifact cannot be read by this build.
    Persistence,
    /// A completed artifact could not be published.
    OutputCommit,
    /// The caller cancelled the run.
    Cancellation,
    /// A time budget expired.
    Timeout,
}

impl SimulationErrorCategory {
    /// Stable snake-case representation used by API and report payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Netlist => "netlist",
            Self::Capability => "capability",
            Self::Materialization => "materialization",
            Self::ResourceLimit => "resource_limit",
            Self::Simulation => "simulation",
            Self::Solver => "solver",
            Self::Convergence => "convergence",
            Self::SignalUnavailable => "signal_unavailable",
            Self::ResultSchema => "result_schema",
            Self::Persistence => "persistence",
            Self::OutputCommit => "output_commit",
            Self::Cancellation => "cancellation",
            Self::Timeout => "timeout",
        }
    }

    /// Every category, in declaration order.
    ///
    /// Frontends use this to prove their category-keyed tables are total
    /// rather than discovering a gap when a new category first fires.
    pub const ALL: &'static [Self] = &[
        Self::Configuration,
        Self::Netlist,
        Self::Capability,
        Self::Materialization,
        Self::ResourceLimit,
        Self::Simulation,
        Self::Solver,
        Self::Convergence,
        Self::SignalUnavailable,
        Self::ResultSchema,
        Self::Persistence,
        Self::OutputCommit,
        Self::Cancellation,
        Self::Timeout,
    ];
}

impl std::fmt::Display for SimulationErrorCategory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Portable metadata for a [`SimulationError`].
///
/// `retryable` is deliberately conservative: it is true only when a fresh
/// request context may safely retry the same workload without changing the
/// netlist or configuration. Deterministic numerical and resource failures are
/// false to prevent automatic retry storms.
///
/// `analysis` and `coordinate` are the typed identities the failure belongs
/// to, so a frontend never has to recover them by parsing a label. Both are
/// `None` when the failure happened outside any one analysis instance or
/// sweep point. The source span is not here because it is not `Copy`; read it
/// with [`SimulationError::source_location`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SimulationErrorDescriptor {
    pub code: SimulationErrorCode,
    pub category: SimulationErrorCategory,
    pub retryable: bool,
    pub iterations: Option<usize>,
    pub resource_limit: Option<ResourceLimitError>,
    pub analysis: Option<AnalysisInstanceId>,
    pub coordinate: Option<RunCoordinateId>,
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedCapabilityError {
    /// Stable dotted token naming the refused capability boundary.
    pub capability: &'static str,
    /// Analysis instance the refusal belongs to, when one is in scope.
    pub analysis: Option<AnalysisInstanceId>,
    /// Run coordinate the refusal belongs to, when one is in scope.
    pub coordinate: Option<RunCoordinateId>,
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
            coordinate: None,
            detail: detail.into(),
            location: None,
        }
    }

    #[must_use]
    pub fn with_analysis(mut self, analysis: AnalysisInstanceId) -> Self {
        self.analysis = Some(analysis);
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
        }
        if let Some(coordinate) = self.coordinate {
            write!(formatter, " at run {coordinate}")?;
        }
        write!(formatter, ": {}", self.detail)
    }
}

impl std::error::Error for UnsupportedCapabilityError {}

/// A materialized run that disagrees with the deck plan that produced it.
///
/// Every variant is an internal consistency failure, not an authored one: the
/// plan and the checked materializer are two derivations of the same deck and
/// one of them is wrong. They are typed rather than collapsed into a message
/// because the coordinate and the analysis identities are what a bug report
/// needs, and because the deck materializer used to own a second, parallel
/// error enum that wrapped [`SimulationError`] from the outside. Wrapping now
/// runs one way: mismatches are `SimulationError`s.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum MaterializationMismatchError {
    #[error(
        "DeckPlan axes or ordered analysis identities do not match the supplied netlist's canonical plan"
    )]
    PlanNetlist,

    #[error(
        "DeckPlan has {planned} run axes, but its netlist exposes {materialized} materializer dimensions"
    )]
    AxisMaterializer { planned: usize, materialized: usize },

    #[error("DeckPlan has {planned} coordinates, but its checked materializer has {materialized}")]
    CoordinateCardinality { planned: usize, materialized: usize },

    #[error("run coordinate index {index} is outside {coordinate_count} planned coordinates")]
    CoordinateIndex {
        index: usize,
        coordinate_count: usize,
    },

    #[error("checked materializer selected different values for coordinate {coordinate}")]
    CoordinateIdentity { coordinate: RunCoordinateId },

    #[error(
        "materialized analysis identity changed at coordinate {coordinate}: expected {expected:?}, got {actual:?}; the authored physical-analysis and post-processing card set must be unconditional across every coordinate"
    )]
    AnalysisIdentity {
        coordinate: RunCoordinateId,
        expected: Vec<AnalysisKind>,
        actual: Vec<AnalysisKind>,
    },
}

impl MaterializationMismatchError {
    /// The run coordinate the mismatch was detected at, when it names one.
    pub const fn coordinate(&self) -> Option<RunCoordinateId> {
        match self {
            Self::CoordinateIdentity { coordinate } | Self::AnalysisIdentity { coordinate, .. } => {
                Some(*coordinate)
            }
            Self::PlanNetlist
            | Self::AxisMaterializer { .. }
            | Self::CoordinateCardinality { .. }
            | Self::CoordinateIndex { .. } => None,
        }
    }
}

/// A persisted artifact this build cannot read.
///
/// Kept separate from a corrupt or truncated artifact: a version mismatch is
/// a compatibility boundary an operator resolves by using the matching build,
/// not by repairing the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceIncompatibleError {
    /// Persisted artifact family, e.g. `"transient checkpoint"`.
    pub format: &'static str,
    /// Version read from the artifact, when the header was legible.
    pub found_version: Option<u32>,
    /// Versions this build accepts, e.g. `"1..=17"`.
    pub supported: String,
    /// Extra context, such as the containing envelope or the path.
    pub detail: Option<String>,
}

impl PersistenceIncompatibleError {
    pub fn new(
        format: &'static str,
        found_version: Option<u32>,
        supported: impl Into<String>,
    ) -> Self {
        Self {
            format,
            found_version,
            supported: supported.into(),
            detail: None,
        }
    }

    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

impl std::fmt::Display for PersistenceIncompatibleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.found_version {
            Some(version) => write!(
                formatter,
                "{} format version {version} is not readable by this build (supported: {})",
                self.format, self.supported
            )?,
            None => write!(
                formatter,
                "{} format version is unreadable by this build (supported: {})",
                self.format, self.supported
            )?,
        }
        if let Some(detail) = &self.detail {
            write!(formatter, ": {detail}")?;
        }
        Ok(())
    }
}

impl std::error::Error for PersistenceIncompatibleError {}

/// Phase of transactional artifact publication that failed.
///
/// Mirrors the phases of [`rspice_output::AtomicArtifactError`] without
/// re-exporting its writer-generic type through the engine's error surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutputCommitPhase {
    /// The staging file could not be created.
    Prepare,
    /// Content could not be written into the staging file.
    Write,
    /// The staged bytes could not be flushed or synchronized.
    Flush,
    /// The destination entry could not be replaced.
    ///
    /// `destination_intact` is true when the previous artifact survived
    /// byte-identical, which is the difference between "nothing was published"
    /// and "a replacement landed but its durability is unproven".
    Commit { destination_intact: bool },
}

impl OutputCommitPhase {
    /// Stable snake-case representation used by API and report payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Write => "write",
            Self::Flush => "flush",
            Self::Commit { .. } => "commit",
        }
    }
}

/// A completed artifact that could not be published to its destination.
///
/// Distinct from an ordinary I/O error: the run produced correct results and
/// only their publication failed, so the operator's next step is a filesystem
/// or permission fix rather than a change to the deck.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputCommitError {
    /// Destination the transaction was publishing to.
    pub path: std::path::PathBuf,
    /// Publication phase that failed.
    pub phase: OutputCommitPhase,
    /// Underlying failure text.
    pub detail: String,
}

impl OutputCommitError {
    pub fn new(
        path: impl Into<std::path::PathBuf>,
        phase: OutputCommitPhase,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            phase,
            detail: detail.into(),
        }
    }

    /// Classify a failure reported by the shared transactional writer.
    pub fn from_atomic<E>(
        path: impl Into<std::path::PathBuf>,
        error: &rspice_output::AtomicArtifactError<E>,
    ) -> Self
    where
        E: std::error::Error + 'static,
    {
        use rspice_output::AtomicArtifactError as Atomic;
        let phase = match error {
            Atomic::Prepare(_) => OutputCommitPhase::Prepare,
            Atomic::Write(_) => OutputCommitPhase::Write,
            Atomic::Flush { .. } => OutputCommitPhase::Flush,
            Atomic::Commit {
                destination_state, ..
            } => OutputCommitPhase::Commit {
                destination_intact: matches!(
                    destination_state,
                    rspice_output::DestinationState::Unchanged
                ),
            },
        };
        Self::new(path, phase, error.to_string())
    }
}

impl std::fmt::Display for OutputCommitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "output commit failed during {} for '{}': {}",
            self.phase.as_str(),
            self.path.display(),
            self.detail
        )
    }
}

impl std::error::Error for OutputCommitError {}

/// A well-formed authored output symbol that is absent from one analysis
/// result.
///
/// The original spelling is retained verbatim so frontends never have to
/// reverse a canonicalized registry name to identify the failing request.
/// `analysis` and `coordinate` are typed identities; `analysis_label` and
/// `coordinate_label` exist only so the human-facing message can name the
/// analysis the way the deck did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedSignalUnavailableError {
    pub signal: String,
    pub analysis: Option<AnalysisInstanceId>,
    pub analysis_label: String,
    pub coordinate: Option<RunCoordinateId>,
    pub coordinate_label: Option<String>,
}

impl RequestedSignalUnavailableError {
    pub fn new(
        signal: impl Into<String>,
        analysis_label: impl Into<String>,
        coordinate_label: Option<String>,
    ) -> Self {
        Self {
            signal: signal.into(),
            analysis: None,
            analysis_label: analysis_label.into(),
            coordinate: None,
            coordinate_label,
        }
    }

    #[must_use]
    pub fn with_analysis(mut self, analysis: AnalysisInstanceId) -> Self {
        self.analysis = Some(analysis);
        self
    }

    #[must_use]
    pub fn with_coordinate(mut self, coordinate: RunCoordinateId) -> Self {
        self.coordinate = Some(coordinate);
        self
    }
}

impl std::fmt::Display for RequestedSignalUnavailableError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "requested signal '{}' is unavailable for {} analysis",
            self.signal, self.analysis_label
        )?;
        if let Some(coordinate) = &self.coordinate_label {
            write!(formatter, " at {coordinate}")?;
        }
        Ok(())
    }
}

impl std::error::Error for RequestedSignalUnavailableError {}

/// An internally produced analysis result whose signal registry and numeric
/// payload do not satisfy the result type's public schema.
///
/// Names are retained in their original order because ordering is part of the
/// result contract. The optional coordinate identifies the particular sweep,
/// frequency, time, or other analysis point at which the mismatch occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultSchemaMismatchError {
    pub analysis: Option<AnalysisInstanceId>,
    pub analysis_label: String,
    pub coordinate: Option<RunCoordinateId>,
    pub coordinate_label: Option<String>,
    pub signal_family: String,
    pub expected_names: Vec<String>,
    pub actual_names: Vec<String>,
    pub expected_value_count: usize,
    pub actual_value_count: usize,
}

impl ResultSchemaMismatchError {
    pub fn new(
        analysis_label: impl Into<String>,
        coordinate_label: Option<String>,
        signal_family: impl Into<String>,
        expected_names: Vec<String>,
        actual_names: Vec<String>,
        expected_value_count: usize,
        actual_value_count: usize,
    ) -> Self {
        Self {
            analysis: None,
            analysis_label: analysis_label.into(),
            coordinate: None,
            coordinate_label,
            signal_family: signal_family.into(),
            expected_names,
            actual_names,
            expected_value_count,
            actual_value_count,
        }
    }

    #[must_use]
    pub fn with_analysis(mut self, analysis: AnalysisInstanceId) -> Self {
        self.analysis = Some(analysis);
        self
    }

    #[must_use]
    pub fn with_coordinate(mut self, coordinate: RunCoordinateId) -> Self {
        self.coordinate = Some(coordinate);
        self
    }
}

impl std::fmt::Display for ResultSchemaMismatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "result schema mismatch for {} analysis",
            self.analysis_label
        )?;
        if let Some(coordinate) = &self.coordinate_label {
            write!(formatter, " at {coordinate}")?;
        }
        write!(
            formatter,
            " in {}: expected names {:?} with {} value(s), got names {:?} with {} value(s)",
            self.signal_family,
            self.expected_names,
            self.expected_value_count,
            self.actual_names,
            self.actual_value_count
        )
    }
}

impl std::error::Error for ResultSchemaMismatchError {}

/// Simulation errors
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Invalid simulation configuration: {0}")]
    Configuration(#[from] super::SimulationConfigError),

    #[error(transparent)]
    ResourceLimit(#[from] crate::resource::ResourceLimitError),

    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error(transparent)]
    BehavioralReference(Box<crate::device::BehavioralReferenceError>),

    #[error(transparent)]
    UnsupportedCapability(Box<UnsupportedCapabilityError>),

    #[error(transparent)]
    MaterializationMismatch(Box<MaterializationMismatchError>),

    #[error("Solver error: {0}")]
    Solver(#[from] crate::solver::SolverError),

    #[error("Netlist error: {0}")]
    Netlist(String),

    #[error(transparent)]
    RequestedSignalUnavailable(Box<RequestedSignalUnavailableError>),

    #[error(transparent)]
    ResultSchemaMismatch(Box<ResultSchemaMismatchError>),

    #[error(transparent)]
    PersistenceIncompatible(Box<PersistenceIncompatibleError>),

    #[error(transparent)]
    OutputCommitFailed(Box<OutputCommitError>),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Simulation aborted by user")]
    Aborted,

    #[error("Simulation stopped: the run's time budget expired")]
    TimeLimitExceeded,
}

impl From<RequestedSignalUnavailableError> for SimulationError {
    fn from(error: RequestedSignalUnavailableError) -> Self {
        Self::RequestedSignalUnavailable(Box::new(error))
    }
}

impl From<ResultSchemaMismatchError> for SimulationError {
    fn from(error: ResultSchemaMismatchError) -> Self {
        Self::ResultSchemaMismatch(Box::new(error))
    }
}

impl From<UnsupportedCapabilityError> for SimulationError {
    fn from(error: UnsupportedCapabilityError) -> Self {
        Self::UnsupportedCapability(Box::new(error))
    }
}

impl From<MaterializationMismatchError> for SimulationError {
    fn from(error: MaterializationMismatchError) -> Self {
        Self::MaterializationMismatch(Box::new(error))
    }
}

impl From<PersistenceIncompatibleError> for SimulationError {
    fn from(error: PersistenceIncompatibleError) -> Self {
        Self::PersistenceIncompatible(Box::new(error))
    }
}

impl From<OutputCommitError> for SimulationError {
    fn from(error: OutputCommitError) -> Self {
        Self::OutputCommitFailed(Box::new(error))
    }
}

impl From<crate::circuit::CircuitError> for SimulationError {
    fn from(error: crate::circuit::CircuitError) -> Self {
        match error {
            crate::circuit::CircuitError::BehavioralReference(error) => {
                Self::BehavioralReference(error)
            }
            error => Self::Circuit(error.to_string()),
        }
    }
}

/// The device layer cannot name this type, so the conversion lives here.
///
/// `device` is ranked below `engine`, which means the generated Verilog-A
/// adapter returns its own [`BuiltinInstantiationError`] and the engine widens
/// it at the boundary. Reading down is what the layer order permits; the
/// reverse is what `tests/module_layering.rs` rejects.
///
/// [`BuiltinInstantiationError`]: crate::device::veriloga_builtins::BuiltinInstantiationError
#[cfg(feature = "veriloga-builtins-base")]
impl From<crate::device::veriloga_builtins::BuiltinInstantiationError> for SimulationError {
    fn from(error: crate::device::veriloga_builtins::BuiltinInstantiationError) -> Self {
        Self::Circuit(error.0)
    }
}

impl SimulationError {
    /// Return stable metadata without requiring consumers to parse the display
    /// message or duplicate knowledge of nested error variants.
    pub fn descriptor(&self) -> SimulationErrorDescriptor {
        let (code, category, retryable) = match self {
            Self::Configuration(super::SimulationConfigError::ResourceLimit(_))
            | Self::ResourceLimit(_) => (
                SimulationErrorCode::ResourceLimit,
                SimulationErrorCategory::ResourceLimit,
                false,
            ),
            Self::Configuration(_) => (
                SimulationErrorCode::InvalidConfiguration,
                SimulationErrorCategory::Configuration,
                false,
            ),
            Self::Circuit(_) => (
                SimulationErrorCode::CircuitError,
                SimulationErrorCategory::Simulation,
                false,
            ),
            Self::BehavioralReference(_) => (
                SimulationErrorCode::BehavioralReferenceError,
                SimulationErrorCategory::Simulation,
                false,
            ),
            Self::UnsupportedCapability(_) => (
                SimulationErrorCode::UnsupportedCapability,
                SimulationErrorCategory::Capability,
                false,
            ),
            Self::MaterializationMismatch(_) => (
                SimulationErrorCode::MaterializationMismatch,
                SimulationErrorCategory::Materialization,
                false,
            ),
            Self::Solver(_) => (
                SimulationErrorCode::SolverError,
                SimulationErrorCategory::Solver,
                false,
            ),
            Self::Netlist(_) => (
                SimulationErrorCode::NetlistError,
                SimulationErrorCategory::Netlist,
                false,
            ),
            Self::RequestedSignalUnavailable(_) => (
                SimulationErrorCode::RequestedSignalUnavailable,
                SimulationErrorCategory::SignalUnavailable,
                false,
            ),
            Self::ResultSchemaMismatch(_) => (
                SimulationErrorCode::ResultSchemaMismatch,
                SimulationErrorCategory::ResultSchema,
                false,
            ),
            Self::PersistenceIncompatible(_) => (
                SimulationErrorCode::PersistenceIncompatible,
                SimulationErrorCategory::Persistence,
                false,
            ),
            Self::OutputCommitFailed(_) => (
                SimulationErrorCode::OutputCommitFailed,
                SimulationErrorCategory::OutputCommit,
                false,
            ),
            Self::ConvergenceFailed(_) => (
                SimulationErrorCode::ConvergenceError,
                SimulationErrorCategory::Convergence,
                false,
            ),
            Self::Aborted => (
                SimulationErrorCode::Aborted,
                SimulationErrorCategory::Cancellation,
                true,
            ),
            Self::TimeLimitExceeded => (
                SimulationErrorCode::TimeLimitExceeded,
                SimulationErrorCategory::Timeout,
                true,
            ),
        };

        SimulationErrorDescriptor {
            code,
            category,
            retryable,
            iterations: match self {
                Self::ConvergenceFailed(iterations)
                | Self::Solver(SolverError::ConvergenceFailed(iterations)) => Some(*iterations),
                _ => None,
            },
            resource_limit: match self {
                Self::Configuration(super::SimulationConfigError::ResourceLimit(error))
                | Self::ResourceLimit(error) => Some(*error),
                _ => None,
            },
            analysis: self.analysis_instance(),
            coordinate: self.run_coordinate(),
        }
    }

    /// Typed analysis instance this failure belongs to, when it names one.
    pub fn analysis_instance(&self) -> Option<AnalysisInstanceId> {
        match self {
            Self::UnsupportedCapability(error) => error.analysis,
            Self::RequestedSignalUnavailable(error) => error.analysis,
            Self::ResultSchemaMismatch(error) => error.analysis,
            _ => None,
        }
    }

    /// Typed run coordinate this failure belongs to, when it names one.
    pub fn run_coordinate(&self) -> Option<RunCoordinateId> {
        match self {
            Self::UnsupportedCapability(error) => error.coordinate,
            Self::RequestedSignalUnavailable(error) => error.coordinate,
            Self::ResultSchemaMismatch(error) => error.coordinate,
            Self::MaterializationMismatch(error) => error.coordinate(),
            _ => None,
        }
    }

    /// Netlist location that authored the offending construct, when known.
    pub fn source_location(&self) -> Option<&NetlistSourceLocation> {
        match self {
            Self::UnsupportedCapability(error) => error.location.as_ref(),
            _ => None,
        }
    }

    /// Whether this failure is a cooperative stop rather than a defect.
    ///
    /// Both a caller cancellation and an expired time budget answer true: they
    /// share the "the run did not finish, and nothing is wrong with the deck"
    /// contract that every propagation site cares about.
    pub(crate) const fn is_stopped(&self) -> bool {
        matches!(self, Self::Aborted | Self::TimeLimitExceeded)
    }

    /// The typed stop error for whichever reason `abort` recorded.
    ///
    /// Core's inner loops raise the reason-free [`Self::Aborted`] because they
    /// cannot know why the flag is set. The surface that owns the signal can,
    /// so it re-labels once at its boundary rather than every check guessing.
    pub(crate) fn from_abort(abort: &dyn AbortSignal) -> Self {
        match abort.abort_reason() {
            AbortReason::Cancelled => Self::Aborted,
            AbortReason::TimeLimit => Self::TimeLimitExceeded,
        }
    }

    /// Re-label a propagated stop with the reason its signal recorded.
    ///
    /// Non-stop errors pass through untouched, so a frontend can apply this to
    /// every error crossing its boundary without inspecting it first.
    #[must_use]
    pub fn with_abort_reason(self, abort: &dyn AbortSignal) -> Self {
        if self.is_stopped() {
            Self::from_abort(abort)
        } else {
            self
        }
    }

    /// Construct a typed capability refusal for a well-formed authored
    /// construct this build does not execute.
    pub fn unsupported_capability(capability: &'static str, detail: impl Into<String>) -> Self {
        UnsupportedCapabilityError::new(capability, detail).into()
    }

    /// Construct a typed missing-output error while retaining the authored
    /// signal spelling and optional analysis coordinate.
    pub fn requested_signal_unavailable(
        signal: impl Into<String>,
        analysis_label: impl Into<String>,
        coordinate_label: Option<String>,
    ) -> Self {
        RequestedSignalUnavailableError::new(signal, analysis_label, coordinate_label).into()
    }

    /// Construct a typed result-schema error while retaining both ordered
    /// signal registries and their associated payload cardinalities.
    pub fn result_schema_mismatch(
        analysis_label: impl Into<String>,
        coordinate_label: Option<String>,
        signal_family: impl Into<String>,
        expected_names: Vec<String>,
        actual_names: Vec<String>,
        expected_value_count: usize,
        actual_value_count: usize,
    ) -> Self {
        ResultSchemaMismatchError::new(
            analysis_label,
            coordinate_label,
            signal_family,
            expected_names,
            actual_names,
            expected_value_count,
            actual_value_count,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_signal_descriptor_and_message_preserve_authored_symbol() {
        let error = SimulationError::requested_signal_unavailable(
            "@Mdriver[Id]",
            "DC",
            Some("sweep point 3 (7.0000000000000000e-1)".to_string()),
        );
        assert_eq!(
            error.to_string(),
            "requested signal '@Mdriver[Id]' is unavailable for DC analysis at sweep point 3 (7.0000000000000000e-1)"
        );
        let descriptor = error.descriptor();
        assert_eq!(
            descriptor.code,
            SimulationErrorCode::RequestedSignalUnavailable
        );
        assert_eq!(
            descriptor.category,
            SimulationErrorCategory::SignalUnavailable
        );
        assert!(!descriptor.retryable);
        let SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("typed unavailable-signal variant was lost");
        };
        assert_eq!(detail.signal, "@Mdriver[Id]");
        assert_eq!(detail.analysis_label, "DC");
    }

    #[test]
    fn capability_refusals_publish_the_token_analysis_and_source_span() {
        let location = NetlistSourceLocation::in_file("decks/rf.cir", 42);
        let error = SimulationError::from(
            UnsupportedCapabilityError::new(
                "analysis.hb.device",
                "device 'Q1' has no harmonic-balance stamp",
            )
            .with_analysis(AnalysisInstanceId::new(AnalysisKind::HarmonicBalance, 1))
            .at(location.clone()),
        );

        let descriptor = error.descriptor();
        assert_eq!(descriptor.code, SimulationErrorCode::UnsupportedCapability);
        assert_eq!(descriptor.category, SimulationErrorCategory::Capability);
        assert_eq!(
            descriptor.analysis.map(|id| id.tag()).as_deref(),
            Some("hb-002")
        );
        assert_eq!(error.source_location(), Some(&location));
        assert!(
            error
                .to_string()
                .contains("unsupported capability [analysis.hb.device] in hb-002"),
            "message must carry the token and the analysis: {error}"
        );
    }

    #[test]
    fn stop_errors_are_relabelled_from_the_signal_that_owns_the_reason() {
        struct Deadline;
        impl AbortSignal for Deadline {
            fn is_aborted(&self) -> bool {
                true
            }
            fn abort_reason(&self) -> AbortReason {
                AbortReason::TimeLimit
            }
        }

        let relabelled = SimulationError::Aborted.with_abort_reason(&Deadline);
        assert_eq!(
            relabelled.descriptor().code,
            SimulationErrorCode::TimeLimitExceeded
        );
        assert_eq!(
            relabelled.descriptor().category,
            SimulationErrorCategory::Timeout
        );
        assert!(relabelled.descriptor().retryable);

        let untouched = SimulationError::ConvergenceFailed(4).with_abort_reason(&Deadline);
        assert_eq!(
            untouched.descriptor().code,
            SimulationErrorCode::ConvergenceError
        );

        let cancelled = SimulationError::TimeLimitExceeded
            .with_abort_reason(&crate::abort_signal::ImmediateAbort);
        assert_eq!(cancelled.descriptor().code, SimulationErrorCode::Aborted);
    }

    #[test]
    fn materialization_mismatches_carry_their_coordinate() {
        let coordinate = RunCoordinateId::from_parts([7; 16], 0);
        let error =
            SimulationError::from(MaterializationMismatchError::CoordinateIdentity { coordinate });
        assert_eq!(
            error.descriptor().category,
            SimulationErrorCategory::Materialization
        );
        assert_eq!(error.run_coordinate(), Some(coordinate));
    }
}
