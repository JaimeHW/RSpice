//! Runner errors, and cooperative abort.
//!
//! Every runner polls the abort signal between stages rather than only at
//! the end, so cancelling a long analysis takes effect promptly instead of
//! at the next natural boundary.

use rspice_core::abort_signal::AbortSignal;
use rspice_results::convergence_attribution::ConvergenceAttribution;
use rspice_results::monte_carlo_checkpoint::CheckpointError;
use rspice_results::validation::ResultSchemaMismatch;
use rspice_simulation_contract::worker_error::WorkerSimulationError;

/// Errors that can occur during simulation
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    /// Netlist parsing error
    ParseError(String),

    /// A node or device referenced by a behavioral expression could not be bound.
    BehavioralReference {
        owner_name: String,
        canonical_owner_name: String,
        dependency_name: String,
        canonical_dependency_name: String,
        reason: String,
    },

    /// Circuit building error
    CircuitError(String),

    /// Binding an instance to a Verilog-A or mixed Verilog-AMS master failed.
    ///
    /// Kept apart from [`Self::ParseError`] and [`Self::CircuitError`] — which
    /// is where the whole `.VERILOGA` seam used to land, split between them by
    /// whichever untyped engine variant each site happened to reach for —
    /// because the workbench acts on this one: `instance` names the symbol to
    /// mark on the schematic, `module` the master to open, `location` the
    /// source file to show, and `kind` decides between "the file is missing",
    /// "the name is wrong", "this value is wrong", "this port is wrong", and
    /// "this is ours to fix" without reading a word of the message.
    Elaboration {
        /// Deck name of the instance, when one instance owns the failure.
        instance: Option<String>,
        /// The master or source the failure is about.
        module: Option<String>,
        /// Stable token from `rspice_core::ElaborationErrorKind::as_str`.
        kind: String,
        /// File and line the engine could point at, rendered as it renders it.
        location: Option<String>,
        /// The engine's full sentence, which is what a person reads.
        message: String,
    },

    /// Solver error
    SolverError(String),

    /// A well-formed output request the finished analysis does not carry.
    ///
    /// The signal is the spelling that was authored, not the registry's
    /// canonical form: what a person has to fix is the request they wrote,
    /// and naming it back to them in another spelling would not identify it.
    RequestedSignalUnavailable {
        signal: String,
        analysis: String,
        coordinate: Option<String>,
    },

    /// A result whose signal registry and numeric payload disagree with the
    /// schema its own result type promises.
    ///
    /// Nothing in the design is at fault and no edit fixes it, so the detail
    /// exists to be reported rather than acted on. Boxed because the payload
    /// is wider than every other variant and this enum is the error half of
    /// `Result` throughout the crate.
    ResultSchemaMismatch(Box<ResultSchemaMismatch>),

    /// Convergence failure
    ConvergenceFailed { iterations: usize, message: String },

    /// A failure the engine could attribute to named design objects.
    ///
    /// `message` is the exact text the unattributed variant would have
    /// displayed, so nothing a person reads changes when the engine gains
    /// the ability to name the objects behind it. The attribution is the
    /// addition, and it is what lets a schematic mark them.
    Attributed {
        message: String,
        attribution: ConvergenceAttribution,
    },

    /// Simulation was aborted
    Aborted,

    /// A simulation is already running
    AlreadyRunning,

    /// Thread panicked
    ThreadPanic,

    /// Invalid configuration
    InvalidConfig(String),

    /// An engine outcome the result-only runner cannot represent.
    UnsupportedOutcome(String),

    /// A configurable production resource budget was exceeded.
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SimulationError::BehavioralReference {
                canonical_owner_name,
                canonical_dependency_name,
                reason,
                ..
            } => write!(
                f,
                "Device instance {canonical_owner_name}: Problem with value for \
                 {canonical_dependency_name} in {canonical_owner_name} ({reason})"
            ),
            SimulationError::CircuitError(msg) => write!(f, "Circuit error: {}", msg),
            // The engine's own rendering, verbatim: it already leads with the
            // span, the instance and the master, so re-labelling it here would
            // only push that identification further from the reader.
            SimulationError::Elaboration { message, .. } => write!(f, "{message}"),
            SimulationError::SolverError(msg) => write!(f, "Solver error: {}", msg),
            SimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => {
                write!(
                    f,
                    "Requested signal '{signal}' is unavailable for {analysis} analysis"
                )?;
                match coordinate {
                    Some(coordinate) => write!(f, " at {coordinate}"),
                    None => Ok(()),
                }
            }
            SimulationError::ResultSchemaMismatch(mismatch) => write!(f, "{mismatch}"),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => {
                write!(
                    f,
                    "Convergence failed after {} iterations: {}",
                    iterations, message
                )
            }
            SimulationError::Attributed { message, .. } => write!(f, "{message}"),
            SimulationError::Aborted => write!(f, "Simulation aborted"),
            SimulationError::AlreadyRunning => write!(f, "A simulation is already running"),
            SimulationError::ThreadPanic => write!(f, "Simulation thread panicked"),
            SimulationError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            SimulationError::UnsupportedOutcome(msg) => {
                write!(f, "Unsupported simulation outcome: {msg}")
            }
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => write!(
                f,
                "Resource limit exceeded for {resource}: requested {requested}, limit {limit}"
            ),
        }
    }
}

impl SimulationError {
    /// The design objects the engine named for this failure, if it named any.
    ///
    /// One reader of the [`Self::Attributed`] payload, so the console anchor
    /// and any later marker are looking at the same answer rather than each
    /// re-matching the variant.
    #[must_use]
    pub fn attribution(&self) -> Option<&ConvergenceAttribution> {
        match self {
            Self::Attributed { attribution, .. } => Some(attribution),
            _ => None,
        }
    }
}

impl std::error::Error for SimulationError {}

impl From<rspice_core::SimulationError> for SimulationError {
    fn from(err: rspice_core::SimulationError) -> Self {
        match err {
            // Result-only entry points cannot represent completion before a
            // numerical result exists. Keep the model diagnostic and identify
            // the runner limitation rather than blaming the circuit or solver.
            rspice_core::SimulationError::ModelFinished(finish) => {
                SimulationError::UnsupportedOutcome(format!(
                    "the UI result runner cannot represent model-requested completion: {finish}"
                ))
            }
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => {
                SimulationError::ResourceLimit {
                    resource: error.resource.as_str().to_string(),
                    requested: error.requested,
                    limit: error.limit,
                }
            }
            rspice_core::SimulationError::Configuration(error) => {
                SimulationError::InvalidConfig(error.to_string())
            }
            rspice_core::SimulationError::BehavioralReference(error) => {
                SimulationError::BehavioralReference {
                    owner_name: error.owner_name,
                    canonical_owner_name: error.canonical_owner_name,
                    dependency_name: error.dependency_name,
                    canonical_dependency_name: error.canonical_dependency_name,
                    reason: error.reason.as_str().to_string(),
                }
            }
            rspice_core::SimulationError::Circuit(msg) => SimulationError::CircuitError(msg),
            // The one place the Verilog-A and mixed elaboration seam is
            // classified. It used to arrive here as `Circuit` or `Netlist`
            // depending on which untyped variant each of some forty sites
            // reached for, so a missing `.va` and a mis-wired discrete port
            // were the same thing to this bridge and a wrong terminal count
            // and an unreadable source were different things. The kind is now
            // the engine's answer and this carries it through unchanged; the
            // message is the engine's own rendering, which already names the
            // instance, the master and the source span.
            rspice_core::SimulationError::Elaboration(error) => {
                let message = error.to_string();
                let rspice_core::ElaborationError {
                    instance,
                    module,
                    kind,
                    span,
                    ..
                } = *error;
                SimulationError::Elaboration {
                    instance,
                    module,
                    kind: kind.as_str().to_owned(),
                    location: span.map(|span| span.to_string()),
                    message,
                }
            }
            // A device that could not evaluate finitely at a trial iterate is
            // a circuit error to the UI: the classification exists for the
            // solver's retry ladder, and by the time a run ends the engine has
            // no retries left. Its display is the device's own diagnostic.
            rspice_core::SimulationError::NonFiniteTrial(error) => {
                SimulationError::CircuitError(error.to_string())
            }
            rspice_core::SimulationError::Solver(solver_err) => {
                SimulationError::SolverError(solver_err.to_string())
            }
            rspice_core::SimulationError::Netlist(msg) => SimulationError::ParseError(msg),
            rspice_core::SimulationError::RfPort(error) => {
                SimulationError::ParseError(error.to_string())
            }
            rspice_core::SimulationError::RequestedSignalUnavailable(error) => {
                let error = *error;
                SimulationError::RequestedSignalUnavailable {
                    signal: error.signal,
                    analysis: error.analysis_label,
                    coordinate: error.coordinate_label,
                }
            }
            rspice_core::SimulationError::ResultSchemaMismatch(error) => {
                let error = *error;
                SimulationError::ResultSchemaMismatch(Box::new(ResultSchemaMismatch {
                    analysis: error.analysis_label,
                    coordinate: error.coordinate_label,
                    signal_family: error.signal_family,
                    expected_names: error.expected_names,
                    actual_names: error.actual_names,
                    expected_value_count: error.expected_value_count,
                    actual_value_count: error.actual_value_count,
                }))
            }
            rspice_core::SimulationError::ConvergenceFailed(iterations) => {
                SimulationError::ConvergenceFailed {
                    iterations,
                    message: "Newton-Raphson iteration limit exceeded".to_string(),
                }
            }
            rspice_core::SimulationError::Aborted => SimulationError::Aborted,
            // An expired time budget is a stop, not a failed circuit; the UI
            // treats it exactly as it treats a user cancellation until it
            // grows a distinct presentation for it.
            rspice_core::SimulationError::TimeLimitExceeded => SimulationError::Aborted,
            // Categories this bridge has no dedicated presentation for yet.
            // They keep their full message, which already names the capability
            // token, the coordinate, or the artifact that failed.
            other @ (rspice_core::SimulationError::UnsupportedCapability(_)
            | rspice_core::SimulationError::ParameterDomain(_)
            | rspice_core::SimulationError::MaterializationMismatch(_)
            | rspice_core::SimulationError::PersistenceIncompatible(_)
            | rspice_core::SimulationError::OutputCommitFailed(_)) => {
                SimulationError::CircuitError(other.to_string())
            }
        }
    }
}

impl From<CheckpointError> for SimulationError {
    fn from(error: CheckpointError) -> Self {
        match error {
            CheckpointError::InvalidConfig(message) => Self::InvalidConfig(message),
            CheckpointError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource: resource.as_str().into(),
                requested,
                limit,
            },
            CheckpointError::Aborted => Self::Aborted,
            CheckpointError::Numerical(error) => Self::from(*error),
        }
    }
}

impl From<SimulationError> for WorkerSimulationError {
    fn from(value: SimulationError) -> Self {
        match value {
            SimulationError::ParseError(message) => Self::ParseError(message),
            SimulationError::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            } => Self::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            },
            SimulationError::CircuitError(message) => Self::CircuitError(message),
            SimulationError::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            } => Self::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            },
            SimulationError::SolverError(message) => Self::SolverError(message),
            SimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => Self::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            },
            SimulationError::ResultSchemaMismatch(mismatch) => Self::ResultSchemaMismatch(mismatch),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            SimulationError::Attributed {
                message,
                attribution,
            } => Self::Attributed {
                message,
                attribution,
            },
            SimulationError::Aborted => Self::Aborted,
            SimulationError::AlreadyRunning => Self::AlreadyRunning,
            SimulationError::ThreadPanic => Self::ThreadPanic,
            SimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            SimulationError::UnsupportedOutcome(message) => Self::UnsupportedOutcome(message),
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

impl From<WorkerSimulationError> for SimulationError {
    fn from(value: WorkerSimulationError) -> Self {
        match value {
            WorkerSimulationError::ParseError(message) => Self::ParseError(message),
            WorkerSimulationError::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            } => Self::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            },
            WorkerSimulationError::CircuitError(message) => Self::CircuitError(message),
            WorkerSimulationError::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            } => Self::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            },
            WorkerSimulationError::SolverError(message) => Self::SolverError(message),
            WorkerSimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => Self::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            },
            WorkerSimulationError::ResultSchemaMismatch(mismatch) => {
                Self::ResultSchemaMismatch(mismatch)
            }
            WorkerSimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            WorkerSimulationError::Attributed {
                message,
                attribution,
            } => Self::Attributed {
                message,
                attribution,
            },
            WorkerSimulationError::Aborted => Self::Aborted,
            WorkerSimulationError::AlreadyRunning => Self::AlreadyRunning,
            WorkerSimulationError::ThreadPanic => Self::ThreadPanic,
            WorkerSimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            WorkerSimulationError::UnsupportedOutcome(message) => Self::UnsupportedOutcome(message),
            WorkerSimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

/// Typed error returned by cancellable simulation-service APIs.
///
/// Legacy synchronous APIs continue to expose `String` for compatibility,
/// but production execution uses this type so cancellation and resource
/// exhaustion can never be mistaken for configuration or solver failures.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ServiceRunError {
    /// Cooperative cancellation was requested.
    #[error("Simulation aborted")]
    Aborted,
    /// A configurable production resource budget was exceeded.
    #[error(transparent)]
    ResourceLimit(#[from] rspice_core::ResourceLimitError),
    /// Validation, parsing, circuit, or solver failure.
    #[error("{0}")]
    Failure(String),
}

/// Result type for cancellable simulation-service APIs.
pub type ServiceRunResult<T> = Result<T, ServiceRunError>;

impl ServiceRunError {
    /// Preserve typed cancellation while adding analysis-specific context to
    /// all other core errors.
    pub fn from_core(context: &str, error: rspice_core::SimulationError) -> Self {
        match error {
            rspice_core::SimulationError::Aborted => Self::Aborted,
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => Self::ResourceLimit(error),
            other => Self::Failure(format!("{context}: {other}")),
        }
    }

    /// Create a typed resource-limit error without relying on display-string
    /// parsing in UI-owned expansion code.
    pub fn resource_limit(
        resource: rspice_core::ResourceKind,
        requested: usize,
        limit: usize,
    ) -> Self {
        Self::ResourceLimit(rspice_core::ResourceLimitError {
            resource,
            requested,
            limit,
        })
    }

    /// Add context to ordinary failures while preserving cancellation and
    /// structured resource-limit errors.
    #[cfg(test)]
    pub fn with_context(self, context: &str) -> Self {
        match self {
            Self::Failure(message) => Self::Failure(format!("{context}: {message}")),
            other => other,
        }
    }

    #[inline]
    pub fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted)
    }
}

impl From<rspice_core::SimulationError> for ServiceRunError {
    fn from(error: rspice_core::SimulationError) -> Self {
        match error {
            rspice_core::SimulationError::Aborted => Self::Aborted,
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => Self::ResourceLimit(error),
            other => Self::Failure(other.to_string()),
        }
    }
}

impl From<String> for ServiceRunError {
    fn from(error: String) -> Self {
        Self::Failure(error)
    }
}

impl From<&str> for ServiceRunError {
    fn from(error: &str) -> Self {
        Self::Failure(error.to_string())
    }
}

#[inline]
pub fn ensure_not_aborted(abort: &dyn AbortSignal) -> ServiceRunResult<()> {
    if abort.is_aborted() {
        Err(ServiceRunError::Aborted)
    } else {
        Ok(())
    }
}

/// Poll cancellation at a bounded cadence inside large scalar loops.
///
/// Callers retain explicit entry/exit and outer-stage checks. This helper
/// bounds cancellation latency without paying for a virtual/atomic poll on
/// every copied or transformed sample.
#[inline]
pub fn poll_periodically(abort: &dyn AbortSignal, index: usize) -> ServiceRunResult<()> {
    const POLL_STRIDE: usize = 64;
    if index.is_multiple_of(POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_abort_remains_typed() {
        assert_eq!(
            ServiceRunError::from_core("AC analysis error", rspice_core::SimulationError::Aborted),
            ServiceRunError::Aborted
        );
        assert_eq!(
            ServiceRunError::from(rspice_core::SimulationError::Aborted),
            ServiceRunError::Aborted
        );
    }

    #[test]
    fn non_abort_core_error_keeps_context() {
        let error = ServiceRunError::from_core(
            "AC analysis error",
            rspice_core::SimulationError::Circuit("invalid circuit".to_string()),
        );

        assert!(matches!(error, ServiceRunError::Failure(_)));
        assert!(error.to_string().contains("AC analysis error"));
        assert!(error.to_string().contains("invalid circuit"));
    }

    #[test]
    fn core_resource_limit_remains_structured() {
        let core_error = rspice_core::ResourceLimitError {
            resource: rspice_core::ResourceKind::BatchRuns,
            requested: 11,
            limit: 10,
        };

        assert_eq!(
            ServiceRunError::from_core(
                "Parametric analysis error",
                rspice_core::SimulationError::ResourceLimit(core_error),
            ),
            ServiceRunError::ResourceLimit(core_error)
        );
        assert_eq!(
            ServiceRunError::ResourceLimit(core_error).with_context("ignored context"),
            ServiceRunError::ResourceLimit(core_error)
        );
    }

    #[test]
    fn model_finish_reports_the_runner_limitation_across_the_worker_boundary() {
        use rspice_simulation_contract::worker_error::WorkerSimulationError;
        let finish = rspice_core::ModelFinish {
            instance: "X1".to_owned(),
            model: "startup".to_owned(),
            site: 7,
            point: rspice_core::ModelFinishPoint::Initialization,
            diagnostic_level: 1,
        };
        let detail = finish.to_string();
        let translated = SimulationError::from(rspice_core::SimulationError::ModelFinished(
            Box::new(finish),
        ));
        assert!(matches!(translated, SimulationError::UnsupportedOutcome(_)));
        assert!(translated.to_string().contains(&detail));
        let wire = serde_json::to_string(&WorkerSimulationError::from(translated.clone())).unwrap();
        let decoded: WorkerSimulationError = serde_json::from_str(&wire).unwrap();
        assert_eq!(SimulationError::from(decoded), translated);
    }

    /// The classification a workbench acts on comes off the kind, not the
    /// prose, and survives the browser worker boundary.
    ///
    /// Both halves matter. Before this, a missing `.va` reached the UI as a
    /// `ParseError` and a mis-wired discrete port as a `CircuitError` purely
    /// because the engine sites had picked different untyped variants, so
    /// nothing downstream could tell "find the file" from "rewire the pin"
    /// without matching text.
    #[test]
    fn elaboration_refusals_reach_the_ui_as_their_kind_and_named_objects() {
        use rspice_simulation_contract::worker_error::WorkerSimulationError;

        let core_error = rspice_core::SimulationError::from(rspice_core::ElaborationError {
            instance: Some("x1".to_owned()),
            module: Some("clock_divider".to_owned()),
            kind: rspice_core::ElaborationErrorKind::PortDiscipline,
            span: None,
            detail: "connects discrete port 'q' to ground; a boundary net carries a logic value \
                     and ground is the voltage reference, not a net"
                .to_owned(),
        });
        let rendered = core_error.to_string();
        let translated = SimulationError::from(core_error);

        let SimulationError::Elaboration {
            instance,
            module,
            kind,
            location,
            message,
        } = &translated
        else {
            panic!("an elaboration refusal must not collapse into prose: {translated}");
        };
        assert_eq!(instance.as_deref(), Some("x1"));
        assert_eq!(module.as_deref(), Some("clock_divider"));
        assert_eq!(kind, "port_discipline");
        assert_eq!(*location, None);
        assert_eq!(message, &rendered);
        assert_eq!(translated.to_string(), rendered);

        let wire = serde_json::to_string(&WorkerSimulationError::from(translated.clone())).unwrap();
        let decoded: WorkerSimulationError = serde_json::from_str(&wire).unwrap();
        assert_eq!(SimulationError::from(decoded), translated);
    }

    /// A source-side refusal carries the file the workbench should open, and
    /// is classified as a missing source rather than as a broken circuit.
    #[test]
    fn a_missing_verilog_a_source_is_classified_and_located() {
        let translated = SimulationError::from(rspice_core::SimulationError::from(
            rspice_core::ElaborationError {
                instance: None,
                module: Some("counter.va".to_owned()),
                kind: rspice_core::ElaborationErrorKind::MissingSource,
                span: Some(rspice_core::netlist::NetlistSourceLocation::in_file(
                    "counter.va",
                    0,
                )),
                detail: "this source does not exist or is unreadable".to_owned(),
            },
        ));
        let SimulationError::Elaboration {
            instance,
            kind,
            location,
            ..
        } = &translated
        else {
            panic!("expected a typed elaboration refusal: {translated}");
        };
        assert_eq!(*instance, None, "no instance owns a missing source");
        assert_eq!(kind, "missing_source");
        assert_eq!(location.as_deref(), Some("counter.va:0"));
    }

    #[test]
    fn behavioral_reference_error_preserves_typed_fields() {
        let core_error = rspice_core::SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        ));
        let translated = SimulationError::from(core_error);

        assert_eq!(
            translated,
            SimulationError::BehavioralReference {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason: "lead_current_not_solution_variable".to_string(),
            }
        );
        assert_eq!(
            translated.to_string(),
            "Device instance B2: Problem with value for B1 in B2 \
             (lead_current_not_solution_variable)"
        );
    }

    #[test]
    fn an_unavailable_output_request_keeps_its_authored_spelling() {
        let core_error = rspice_core::SimulationError::requested_signal_unavailable(
            "@Mdriver[Id]",
            "DC",
            Some("v1 = 1".to_string()),
        );

        let translated = SimulationError::from(core_error);

        assert_eq!(
            translated,
            SimulationError::RequestedSignalUnavailable {
                signal: "@Mdriver[Id]".to_string(),
                analysis: "DC".to_string(),
                coordinate: Some("v1 = 1".to_string()),
            },
            "the request a person has to fix must survive translation verbatim"
        );
        assert_eq!(
            translated.to_string(),
            "Requested signal '@Mdriver[Id]' is unavailable for DC analysis at v1 = 1"
        );
    }

    #[test]
    fn a_result_that_fails_its_own_schema_reports_both_registries() {
        let core_error = rspice_core::SimulationError::result_schema_mismatch(
            "TRAN",
            None,
            "node voltages",
            vec!["V(in)".to_string(), "V(out)".to_string()],
            vec!["V(in)".to_string()],
            2,
            1,
        );

        let translated = SimulationError::from(core_error);

        let SimulationError::ResultSchemaMismatch(mismatch) = &translated else {
            panic!("expected a schema mismatch, got {translated:?}");
        };
        assert_eq!(mismatch.expected_names, ["V(in)", "V(out)"]);
        assert_eq!(mismatch.actual_names, ["V(in)"]);
        assert_eq!(
            (mismatch.expected_value_count, mismatch.actual_value_count),
            (2, 1)
        );
        assert_eq!(
            translated.to_string(),
            "Result schema mismatch for TRAN analysis in node voltages: \
             expected names [\"V(in)\", \"V(out)\"] with 2 value(s), \
             got names [\"V(in)\"] with 1 value(s)"
        );
    }
}
