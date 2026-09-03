//! CLI Error Types
//!
//! Professional error handling with structured error types,
//! exit codes following GNU conventions, and helpful diagnostics.
//!
//! # One exit code per category
//!
//! An exit status is the only thing a shell script sees, so it must say what
//! kind of failure happened. Every code comes from exactly one table,
//! [`exit_code_for`], keyed by a category — and for anything the engine
//! produced, that category is the engine's own
//! [`rspice_core::SimulationErrorCategory`]. The CLI therefore never
//! re-decides what an engine failure means, and a new engine category cannot
//! reach a user as an undifferentiated `1`.

use serde::Serialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

use rspice_core::SimulationErrorCategory;
use rspice_output::AtomicArtifactError;

/// Stable machine-readable metadata attached to CLI failures and run reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorDetails {
    pub code: &'static str,
    pub category: &'static str,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<String>,
    /// Stable identity of the failing analysis card, e.g. `ac-002`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    /// Stable identity of the failing run coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_id: Option<String>,
    /// Netlist line the failing construct was authored on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    /// Netlist file the failing construct was authored in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Dotted token naming a refused capability boundary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_instance_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_dependency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'static str>,
}

impl ErrorDetails {
    fn new(code: &'static str, category: &'static str, retryable: bool) -> Self {
        Self {
            code,
            category,
            retryable,
            analysis: None,
            analysis_id: None,
            coordinate_id: None,
            line: None,
            path: None,
            capability: None,
            iterations: None,
            resource: None,
            requested: None,
            limit: None,
            instance_name: None,
            canonical_instance_name: None,
            missing_dependency: None,
            reason: None,
        }
    }
}

/// Exit codes following GNU conventions
///
/// 65-78 keep their `sysexits.h` meanings. 80-85 are an RSpice block for the
/// engine-domain outcomes `sysexits` has no name for; they are deliberately
/// below 125, which shells reserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    /// Successful execution
    Success = 0,
    /// A failure with no typed engine category: a Verilog-A compile failure or
    /// a result-format conversion failure.
    GeneralError = 1,
    /// Misuse of command (invalid arguments)
    MisuseOfCommand = 2,
    /// Verification failure: the simulation ran, but a .MEAS check failed
    /// or results did not match the golden reference
    VerificationFailed = 3,
    /// The run exceeded --timeout (GNU timeout convention)
    TimedOut = 124,
    /// Interrupted by Ctrl-C (128 + SIGINT)
    Interrupted = 130,
    /// Input file not found
    InputNotFound = 66,
    /// Input file format error
    InputError = 65,
    /// The deck is well formed and this build does not execute it
    /// (`EX_UNAVAILABLE`)
    Capability = 69,
    /// Internal software error
    InternalError = 70,
    /// A completed artifact could not be published (`EX_CANTCREAT`)
    OutputCommitFailed = 73,
    /// I/O error
    IoError = 74,
    /// A configured resource budget was exceeded (`EX_TEMPFAIL`: the same
    /// workload succeeds under a larger budget)
    ResourceLimit = 75,
    /// A persisted artifact was written by an incompatible format version
    /// (`EX_PROTOCOL`)
    PersistenceIncompatible = 76,
    /// Configuration error
    ConfigError = 78,
    /// Circuit construction or device evaluation failed
    SimulationFailed = 80,
    /// The numerical solver failed
    SolverFailed = 81,
    /// An iterative analysis exhausted its convergence strategy
    ConvergenceFailed = 82,
    /// A valid authored output symbol is absent from the produced result
    SignalUnavailable = 83,
    /// A produced result violates its own published schema
    ResultSchemaMismatch = 84,
    /// A materialized run disagrees with the plan that produced it
    MaterializationMismatch = 85,
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        std::process::ExitCode::from(code as u8)
    }
}

/// What kind of failure an exit code is derived from.
///
/// Anything the engine produced answers with the engine's own category, so
/// the CLI translates rather than re-deciding. The remaining variants are
/// failures that only a command-line frontend can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureCategory {
    /// A typed engine failure, carrying the engine's category verbatim.
    Engine(SimulationErrorCategory),
    /// The requested input file does not exist.
    InputNotFound,
    /// Reading or writing a file failed outside a publication transaction.
    Io,
    /// The invocation itself was wrong.
    Usage,
    /// The run completed and a check the user asked for did not pass.
    Verification,
    /// A defect in this program.
    Internal,
    /// Compiling a model failed.
    Compilation,
    /// Converting between result formats failed.
    Conversion,
}

impl FailureCategory {
    /// Stable snake-case representation used by JSON diagnostics and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Engine(category) => category.as_str(),
            Self::InputNotFound => "input_not_found",
            Self::Io => "io",
            Self::Usage => "usage",
            Self::Verification => "verification",
            Self::Internal => "internal",
            Self::Compilation => "compilation",
            Self::Conversion => "conversion",
        }
    }

    /// Recover a category from the stable text a run report recorded.
    ///
    /// A multi-run invocation records each deck's failure in its report before
    /// the process decides one exit status for the whole plan, so that status
    /// has to be reconstructed from what the report already published.
    /// Unrecognized text is a defect in this program, never a user error, so
    /// this returns `None` rather than guessing a category.
    pub fn parse(text: &str) -> Option<Self> {
        for category in SimulationErrorCategory::ALL {
            if category.as_str() == text {
                return Some(Self::Engine(*category));
            }
        }
        Some(match text {
            "input_not_found" => Self::InputNotFound,
            "io" => Self::Io,
            "usage" => Self::Usage,
            "verification" => Self::Verification,
            "internal" => Self::Internal,
            "compilation" => Self::Compilation,
            "conversion" => Self::Conversion,
            _ => return None,
        })
    }
}

/// The one table mapping a failure category to a process exit status.
///
/// No typed engine category may map to [`ExitCode::GeneralError`]: automation
/// that sees `1` learns nothing, which is exactly the ambiguity the taxonomy
/// exists to remove. `tests/exit_codes.rs` drives one deck or flag per
/// category through the real binary to prove these are reachable, and
/// [`engine_categories_never_collapse_onto_the_general_error_code`] proves the
/// table stays total as categories are added.
pub const fn exit_code_for(category: FailureCategory) -> ExitCode {
    match category {
        FailureCategory::Engine(SimulationErrorCategory::Configuration) => ExitCode::ConfigError,
        FailureCategory::Engine(SimulationErrorCategory::Netlist) => ExitCode::InputError,
        FailureCategory::Engine(SimulationErrorCategory::Capability) => ExitCode::Capability,
        FailureCategory::Engine(SimulationErrorCategory::Materialization) => {
            ExitCode::MaterializationMismatch
        }
        FailureCategory::Engine(SimulationErrorCategory::ResourceLimit) => ExitCode::ResourceLimit,
        FailureCategory::Engine(SimulationErrorCategory::Simulation) => ExitCode::SimulationFailed,
        FailureCategory::Engine(SimulationErrorCategory::Solver) => ExitCode::SolverFailed,
        FailureCategory::Engine(SimulationErrorCategory::Convergence) => {
            ExitCode::ConvergenceFailed
        }
        FailureCategory::Engine(SimulationErrorCategory::SignalUnavailable) => {
            ExitCode::SignalUnavailable
        }
        FailureCategory::Engine(SimulationErrorCategory::ResultSchema) => {
            ExitCode::ResultSchemaMismatch
        }
        FailureCategory::Engine(SimulationErrorCategory::Persistence) => {
            ExitCode::PersistenceIncompatible
        }
        FailureCategory::Engine(SimulationErrorCategory::OutputCommit) => {
            ExitCode::OutputCommitFailed
        }
        FailureCategory::Engine(SimulationErrorCategory::Cancellation) => ExitCode::Interrupted,
        FailureCategory::Engine(SimulationErrorCategory::Timeout) => ExitCode::TimedOut,
        // `SimulationErrorCategory` is `#[non_exhaustive]`, so the compiler
        // cannot prove this match total. An engine category this build does
        // not know is a defect in this table, and reporting it as an internal
        // error is more honest than guessing a domain code.
        FailureCategory::Engine(_) => ExitCode::InternalError,
        FailureCategory::InputNotFound => ExitCode::InputNotFound,
        FailureCategory::Io => ExitCode::IoError,
        FailureCategory::Usage => ExitCode::MisuseOfCommand,
        FailureCategory::Verification => ExitCode::VerificationFailed,
        FailureCategory::Internal => ExitCode::InternalError,
        FailureCategory::Compilation | FailureCategory::Conversion => ExitCode::GeneralError,
    }
}

/// CLI-specific errors with context and suggestions
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Input file not found: {path}")]
    InputNotFound {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read input file: {path}")]
    InputReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse netlist: {message}")]
    ParseError {
        message: String,
        line: Option<usize>,
        suggestion: Option<String>,
    },

    #[error("Simulation failed: {message}")]
    SimulationError {
        message: String,
        analysis: Option<String>,
    },

    #[error(
        "Simulation failed{context}: {source}",
        context = analysis
            .as_ref()
            .map(|analysis| format!(" during {analysis}"))
            .unwrap_or_default()
    )]
    CoreSimulationError {
        #[source]
        source: rspice_core::SimulationError,
        analysis: Option<String>,
    },

    #[error("Verification failed: {message}")]
    VerificationFailed { message: String },

    #[error("Simulation timed out after {seconds}s")]
    TimedOut { seconds: f64 },

    #[error("Simulation interrupted")]
    Interrupted,

    #[error("Failed to write output: {path}")]
    OutputError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to materialize Xyce ADDRESISTORS derived netlist: {source}")]
    AddResistorsMaterialization {
        #[source]
        source: rspice_core::netlist::XyceAddResistorsMaterializationError,
    },

    #[error("Failed to write Xyce ADDRESISTORS derived netlist: {path}")]
    AddResistorsArtifactIo {
        path: PathBuf,
        #[source]
        source: AtomicArtifactError<std::io::Error>,
    },

    #[error("Failed to serialize output: {path}")]
    OutputSerializationError {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid argument: {message}")]
    InvalidArgument {
        message: String,
        suggestion: Option<String>,
    },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Configuration error: {source}")]
    CoreConfigError {
        #[source]
        source: rspice_core::SimulationConfigError,
    },

    #[error("Verilog-A compilation failed: {message}")]
    VerilogAError { message: String },

    #[error("Format conversion failed: {message}")]
    ConversionError { message: String },

    #[error("Resource limit exceeded while reading {path}: {source}")]
    ResourceLimit {
        path: PathBuf,
        #[source]
        source: rspice_core::ResourceLimitError,
    },

    #[error("Internal error: {message}")]
    InternalError { message: String },

    /// A failure already recorded in a run report, replayed as the process
    /// status.
    ///
    /// A multi-run plan finishes every deck and writes its reports before the
    /// invocation picks one exit status, so by then the original typed error
    /// is gone. Carrying the details it published keeps the category, the
    /// code, and the exit status the same as they would have been for a
    /// single-deck run — otherwise every plan failure collapsed into one
    /// generic simulation error.
    #[error("{message}")]
    Reported {
        message: String,
        category: FailureCategory,
        details: Box<ErrorDetails>,
    },
}

impl CliError {
    /// The failure category this error belongs to.
    ///
    /// Engine failures answer with the engine's own category, read from the
    /// descriptor, so this function never restates what a `SimulationError`
    /// means. The remaining arms are the frontend's own failure modes.
    pub fn category(&self) -> FailureCategory {
        use FailureCategory as Category;
        match self {
            CliError::CoreSimulationError { source, .. } => {
                Category::Engine(source.descriptor().category)
            }
            CliError::CoreConfigError { source } => Category::Engine(match source {
                rspice_core::SimulationConfigError::ResourceLimit(_) => {
                    SimulationErrorCategory::ResourceLimit
                }
                _ => SimulationErrorCategory::Configuration,
            }),
            // An untyped simulation failure raised by the CLI itself. It is
            // still a circuit-domain failure, so it shares the engine's
            // simulation category rather than inventing a frontend one.
            CliError::SimulationError { .. } => {
                Category::Engine(SimulationErrorCategory::Simulation)
            }
            CliError::ParseError { .. } | CliError::AddResistorsMaterialization { .. } => {
                Category::Engine(SimulationErrorCategory::Netlist)
            }
            CliError::ResourceLimit { .. } => {
                Category::Engine(SimulationErrorCategory::ResourceLimit)
            }
            CliError::ConfigError { .. } => {
                Category::Engine(SimulationErrorCategory::Configuration)
            }
            CliError::TimedOut { .. } => Category::Engine(SimulationErrorCategory::Timeout),
            CliError::Interrupted => Category::Engine(SimulationErrorCategory::Cancellation),
            CliError::AddResistorsArtifactIo { .. } => {
                Category::Engine(SimulationErrorCategory::OutputCommit)
            }
            CliError::InputNotFound { .. } => Category::InputNotFound,
            CliError::InputReadError { .. } | CliError::OutputError { .. } => Category::Io,
            CliError::VerificationFailed { .. } => Category::Verification,
            CliError::InvalidArgument { .. } => Category::Usage,
            CliError::OutputSerializationError { .. } | CliError::InternalError { .. } => {
                Category::Internal
            }
            CliError::VerilogAError { .. } => Category::Compilation,
            CliError::ConversionError { .. } => Category::Conversion,
            CliError::Reported { category, .. } => *category,
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> ExitCode {
        exit_code_for(self.category())
    }

    /// Get a suggestion for fixing this error, if available
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            CliError::ParseError { suggestion, .. } => suggestion.as_deref(),
            CliError::InvalidArgument { suggestion, .. } => suggestion.as_deref(),
            _ => None,
        }
    }

    /// Stable metadata suitable for JSON logs, automation, and run reports.
    ///
    /// The category always comes from [`Self::category`], so the JSON
    /// diagnostic, the run report, and the exit status can never disagree
    /// about what kind of failure this was.
    pub fn details(&self) -> ErrorDetails {
        let category = self.category().as_str();
        match self {
            Self::CoreSimulationError { source, analysis } => {
                let descriptor = source.descriptor();
                let mut details =
                    ErrorDetails::new(descriptor.code.as_str(), category, descriptor.retryable);
                details.analysis = analysis.clone();
                details.analysis_id = descriptor.analysis.map(|id| id.tag());
                details.coordinate_id = descriptor.coordinate.map(|id| id.to_string());
                if let Some(location) = source.source_location() {
                    details.line = Some(location.line);
                    details.path = location
                        .path
                        .as_ref()
                        .map(|path| path.display().to_string());
                }
                details.iterations = descriptor.iterations;
                if let Some(limit) = descriptor.resource_limit {
                    details.resource = Some(limit.resource.as_str());
                    details.requested = Some(limit.requested);
                    details.limit = Some(limit.limit);
                }
                match source {
                    rspice_core::SimulationError::BehavioralReference(error) => {
                        details.instance_name = Some(error.owner_name.clone());
                        details.canonical_instance_name = Some(error.canonical_owner_name.clone());
                        details.missing_dependency = Some(error.canonical_dependency_name.clone());
                        details.reason = Some(error.reason.as_str());
                    }
                    rspice_core::SimulationError::UnsupportedCapability(refusal) => {
                        details.capability = Some(refusal.capability);
                    }
                    _ => {}
                }
                details
            }
            Self::SimulationError { analysis, .. } => {
                let mut details = ErrorDetails::new("simulation_error", category, false);
                details.analysis = analysis.clone();
                details
            }
            Self::InputNotFound { .. } => ErrorDetails::new("input_not_found", category, false),
            Self::InputReadError { .. } => ErrorDetails::new("input_read_error", category, true),
            Self::ParseError { line, .. } => {
                let mut details = ErrorDetails::new("parse_error", category, false);
                details.line = *line;
                details
            }
            Self::VerificationFailed { .. } => {
                ErrorDetails::new("verification_failed", category, false)
            }
            Self::TimedOut { .. } => ErrorDetails::new("timed_out", category, true),
            Self::Interrupted => ErrorDetails::new("interrupted", category, true),
            Self::OutputError { .. } => ErrorDetails::new("output_error", category, true),
            Self::AddResistorsArtifactIo { .. } => {
                ErrorDetails::new("output_commit_failed", category, true)
            }
            Self::AddResistorsMaterialization { .. } => {
                ErrorDetails::new("addresistors_materialization", category, false)
            }
            Self::OutputSerializationError { .. } => {
                ErrorDetails::new("output_serialization", category, false)
            }
            Self::InvalidArgument { .. } => ErrorDetails::new("invalid_argument", category, false),
            Self::ConfigError { .. } => ErrorDetails::new("invalid_configuration", category, false),
            Self::CoreConfigError { source } => match source {
                rspice_core::SimulationConfigError::ResourceLimit(limit) => {
                    let mut details = ErrorDetails::new("resource_limit", category, false);
                    details.resource = Some(limit.resource.as_str());
                    details.requested = Some(limit.requested);
                    details.limit = Some(limit.limit);
                    details
                }
                _ => ErrorDetails::new("invalid_configuration", category, false),
            },
            Self::VerilogAError { .. } => ErrorDetails::new("veriloga_error", category, false),
            Self::ConversionError { .. } => ErrorDetails::new("conversion_error", category, false),
            Self::ResourceLimit { source, .. } => {
                let mut details = ErrorDetails::new("resource_limit", category, false);
                details.resource = Some(source.resource.as_str());
                details.requested = Some(source.requested);
                details.limit = Some(source.limit);
                details
            }
            Self::InternalError { .. } => ErrorDetails::new("internal_error", category, false),
            Self::Reported { details, .. } => (**details).clone(),
        }
    }

    /// Replay a failure a run report already recorded as the process status.
    ///
    /// Without recorded details there is nothing typed to preserve, so the
    /// failure keeps the engine's simulation category — the same thing a
    /// single-deck run would have produced. Unrecognized category text can
    /// only come from a defect in this program's own report writer.
    pub(crate) fn reported(message: String, details: Option<ErrorDetails>) -> Self {
        let Some(details) = details else {
            return Self::SimulationError {
                message,
                analysis: None,
            };
        };
        match FailureCategory::parse(details.category) {
            Some(category) => Self::Reported {
                message,
                category,
                details: Box::new(details),
            },
            None => Self::InternalError {
                message: format!(
                    "run report recorded the unknown failure category '{}' for: {message}",
                    details.category
                ),
            },
        }
    }

    /// Create a parse error with context
    pub fn parse_error(message: impl Into<String>) -> Self {
        CliError::ParseError {
            message: message.into(),
            line: None,
            suggestion: None,
        }
    }

    /// Create a simulation error with analysis context
    pub fn simulation_error_in(message: impl Into<String>, analysis: impl Into<String>) -> Self {
        CliError::SimulationError {
            message: message.into(),
            analysis: Some(analysis.into()),
        }
    }

    /// Create an output I/O error with path context.
    pub fn output_error(path: &Path, source: std::io::Error) -> Self {
        CliError::OutputError {
            path: path.to_path_buf(),
            source,
        }
    }

    /// Create an output JSON error, preserving underlying I/O failures.
    pub fn output_json_error(path: &Path, source: serde_json::Error) -> Self {
        if source.is_io() {
            return CliError::output_error(path, source.into());
        }

        CliError::OutputSerializationError {
            path: path.to_path_buf(),
            source,
        }
    }
}

/// Preserve writer-domain failures while typing publication failures reported
/// by the shared atomic artifact publisher.
///
/// A writer failure is the caller's own error and passes through. Everything
/// else happened to a result the run had already produced correctly, so it
/// becomes the engine's `OutputCommitFailed` — exit 73, not the generic I/O
/// 74 that used to hide it.
pub(crate) fn map_atomic_output_error(
    path: &Path,
    error: AtomicArtifactError<CliError>,
) -> CliError {
    match error {
        AtomicArtifactError::Write(error) => error,
        other => {
            let commit = rspice_core::OutputCommitError::from_atomic(path, &other);
            CliError::CoreSimulationError {
                source: rspice_core::SimulationError::from(commit),
                analysis: None,
            }
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::InternalError {
            message: err.to_string(),
        }
    }
}

impl From<rspice_core::error::ParseError> for CliError {
    fn from(err: rspice_core::error::ParseError) -> Self {
        // A construct the grammar recognized and this build declines to lower
        // is a capability gap, not a malformed deck. Routing it through the
        // engine's typed refusal keeps the token, the span, and the exit code
        // identical whether it was caught while parsing or while elaborating.
        if let rspice_core::error::ParseError::UnsupportedCapability {
            origin,
            capability,
            detail,
        } = err
        {
            return CliError::CoreSimulationError {
                source: rspice_core::UnsupportedCapabilityError::new(capability, detail)
                    .at(origin)
                    .into(),
                analysis: None,
            };
        }
        let line = err.source_location().map(|location| location.line);
        CliError::ParseError {
            message: err.to_string(),
            line,
            suggestion: None,
        }
    }
}

impl From<rspice_core::SimulationError> for CliError {
    fn from(err: rspice_core::SimulationError) -> Self {
        // The engine cannot know whether its stop flag was set by Ctrl-C or by
        // `--timeout`; this process does. Re-labelling once here is what makes
        // an expired budget exit 124 instead of 130 on every path that does
        // not already classify the stop itself.
        let err = err.with_abort_reason(&crate::abort::ProcessAbort);
        match err {
            rspice_core::SimulationError::Configuration(error) => error.into(),
            other => CliError::CoreSimulationError {
                source: other,
                analysis: None,
            },
        }
    }
}

impl From<rspice_core::SimulationConfigError> for CliError {
    fn from(err: rspice_core::SimulationConfigError) -> Self {
        CliError::CoreConfigError { source: err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_configuration_errors_use_configuration_exit_code() {
        let error = CliError::from(rspice_core::SimulationConfigError::InvalidCount {
            field: "max_iterations",
            value: 0,
        });

        assert_eq!(error.exit_code(), ExitCode::ConfigError);
        assert!(error.to_string().contains("max_iterations"));

        let wrapped = CliError::from(rspice_core::SimulationError::Configuration(
            rspice_core::SimulationConfigError::InvalidCount {
                field: "max_iterations",
                value: 0,
            },
        ));
        assert_eq!(wrapped.exit_code(), ExitCode::ConfigError);
    }

    #[test]
    fn resource_configuration_errors_keep_numeric_limit_details() {
        let error = CliError::from(rspice_core::SimulationConfigError::ResourceLimit(
            rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::MatrixUnknowns,
                requested: 12,
                limit: 10,
            },
        ));
        let details = error.details();
        // A budget the deck outgrew is a resource-limit outcome wherever it is
        // detected; it used to exit 78 here and 65 elsewhere for the same cause.
        assert_eq!(error.exit_code(), ExitCode::ResourceLimit);
        assert_eq!(details.code, "resource_limit");
        assert_eq!(details.category, "resource_limit");
        assert_eq!(details.resource, Some("matrix_unknowns"));
        assert_eq!(details.requested, Some(12));
        assert_eq!(details.limit, Some(10));
    }

    #[test]
    fn engine_errors_preserve_shared_machine_metadata() {
        let error = CliError::from(rspice_core::SimulationError::ConvergenceFailed(31));
        assert_eq!(error.exit_code(), ExitCode::ConvergenceFailed);
        let details = error.details();
        assert_eq!(details.code, "convergence_error");
        assert_eq!(details.category, "convergence");
        assert_eq!(details.iterations, Some(31));
        assert!(!details.retryable);
    }

    #[test]
    fn engine_categories_never_collapse_onto_the_general_error_code() {
        let mut codes = std::collections::BTreeMap::new();
        for category in SimulationErrorCategory::ALL {
            let code = exit_code_for(FailureCategory::Engine(*category));
            assert_ne!(
                code,
                ExitCode::GeneralError,
                "engine category {category} must not exit 1: automation cannot act on it"
            );
            assert_ne!(
                code,
                ExitCode::InternalError,
                "engine category {category} has no entry in the exit-code table"
            );
            assert!(
                codes.insert(code as u8, *category).is_none(),
                "engine category {category} shares an exit code with {:?}",
                codes.get(&(code as u8))
            );
        }
        assert_eq!(codes.len(), SimulationErrorCategory::ALL.len());
    }

    #[test]
    fn the_json_category_and_the_exit_code_come_from_one_decision() {
        let errors = [
            CliError::from(rspice_core::SimulationError::unsupported_capability(
                "analysis.hb.device",
                "no HB stamp",
            )),
            CliError::from(rspice_core::SimulationError::ConvergenceFailed(2)),
            CliError::Interrupted,
            CliError::TimedOut { seconds: 1.0 },
            CliError::InvalidArgument {
                message: "bad flag".to_string(),
                suggestion: None,
            },
        ];
        for error in errors {
            assert_eq!(
                error.details().category,
                error.category().as_str(),
                "details() and category() disagreed for {error}"
            );
            assert_eq!(error.exit_code(), exit_code_for(error.category()));
        }
    }

    #[test]
    fn capability_refusals_publish_their_token_span_and_exit_code() {
        let error = CliError::from(rspice_core::SimulationError::from(
            rspice_core::UnsupportedCapabilityError::new(
                "device.ltra.rg_finite_length",
                "finite-length RG LTRA is not stamped",
            )
            .at(rspice_core::netlist::NetlistSourceLocation::in_file(
                "deck.cir", 12,
            )),
        ));

        assert_eq!(error.exit_code(), ExitCode::Capability);
        let details = error.details();
        assert_eq!(details.category, "capability");
        assert_eq!(details.code, "unsupported_capability");
        assert_eq!(details.capability, Some("device.ltra.rg_finite_length"));
        assert_eq!(details.line, Some(12));
        assert_eq!(details.path.as_deref(), Some("deck.cir"));
    }

    #[test]
    fn parse_capability_refusals_become_engine_refusals_with_their_span() {
        let error = CliError::from(rspice_core::error::ParseError::UnsupportedCapability {
            origin: rspice_core::netlist::NetlistSourceLocation::in_memory(7),
            capability: "netlist.xyce.ydevice.digital_separate_effort",
            detail: "owned by the separate digital effort".to_string(),
        });
        assert_eq!(error.exit_code(), ExitCode::Capability);
        let details = error.details();
        assert_eq!(
            details.capability,
            Some("netlist.xyce.ydevice.digital_separate_effort")
        );
        assert_eq!(details.line, Some(7));
    }

    #[test]
    fn ordinary_parse_errors_keep_the_netlist_category_and_their_line() {
        let error = CliError::from(rspice_core::error::ParseError::Syntax {
            line: 41,
            message: "unexpected token".to_string(),
        });
        assert_eq!(error.exit_code(), ExitCode::InputError);
        assert_eq!(error.details().category, "netlist");
        assert_eq!(error.details().line, Some(41));
    }

    #[test]
    fn publication_failures_are_output_commit_failures_not_plain_io() {
        let error = map_atomic_output_error(
            Path::new("results/out.csv"),
            AtomicArtifactError::Commit {
                operation: rspice_output::CommitOperation::Replace,
                destination_state: rspice_output::DestinationState::Unchanged,
                source: std::io::Error::from(std::io::ErrorKind::PermissionDenied),
            },
        );
        assert_eq!(error.exit_code(), ExitCode::OutputCommitFailed);
        assert_eq!(error.details().category, "output_commit");
        assert_eq!(error.details().code, "output_commit_failed");
    }

    #[test]
    fn behavioral_reference_errors_preserve_device_identity() {
        let error = CliError::from(rspice_core::SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        )));
        let details = error.details();
        assert_eq!(details.code, "behavioral_reference_error");
        assert_eq!(details.instance_name.as_deref(), Some("b2"));
        assert_eq!(details.canonical_instance_name.as_deref(), Some("B2"));
        assert_eq!(details.missing_dependency.as_deref(), Some("B1"));
        assert_eq!(details.reason, Some("lead_current_not_solution_variable"));
    }
}
