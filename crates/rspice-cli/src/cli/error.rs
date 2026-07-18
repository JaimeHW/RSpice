//! CLI Error Types
//!
//! Professional error handling with structured error types,
//! exit codes following GNU conventions, and helpful diagnostics.

use serde::Serialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Stable machine-readable metadata attached to CLI failures and run reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorDetails {
    pub code: &'static str,
    pub category: &'static str,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl ErrorDetails {
    fn new(code: &'static str, category: &'static str, retryable: bool) -> Self {
        Self {
            code,
            category,
            retryable,
            analysis: None,
            iterations: None,
            resource: None,
            requested: None,
            limit: None,
        }
    }
}

/// Exit codes following GNU conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    /// Successful execution
    Success = 0,
    /// General error
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
    /// Internal software error
    InternalError = 70,
    /// I/O error
    IoError = 74,
    /// Configuration error
    ConfigError = 78,
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        std::process::ExitCode::from(code as u8)
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

    #[error("Simulation failed: {source}")]
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
        source: std::io::Error,
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

    #[error("Internal error: {message}")]
    InternalError { message: String },
}

impl CliError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> ExitCode {
        match self {
            CliError::InputNotFound { .. } => ExitCode::InputNotFound,
            CliError::InputReadError { .. } => ExitCode::IoError,
            CliError::ParseError { .. } => ExitCode::InputError,
            CliError::SimulationError { .. } => ExitCode::GeneralError,
            CliError::CoreSimulationError { .. } => ExitCode::GeneralError,
            CliError::VerificationFailed { .. } => ExitCode::VerificationFailed,
            CliError::TimedOut { .. } => ExitCode::TimedOut,
            CliError::Interrupted => ExitCode::Interrupted,
            CliError::OutputError { .. } => ExitCode::IoError,
            CliError::AddResistorsMaterialization { .. } => ExitCode::InputError,
            CliError::AddResistorsArtifactIo { .. } => ExitCode::IoError,
            CliError::OutputSerializationError { .. } => ExitCode::InternalError,
            CliError::InvalidArgument { .. } => ExitCode::MisuseOfCommand,
            CliError::ConfigError { .. } => ExitCode::ConfigError,
            CliError::CoreConfigError { .. } => ExitCode::ConfigError,
            CliError::VerilogAError { .. } => ExitCode::GeneralError,
            CliError::ConversionError { .. } => ExitCode::GeneralError,
            CliError::InternalError { .. } => ExitCode::InternalError,
        }
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
    pub fn details(&self) -> ErrorDetails {
        match self {
            Self::CoreSimulationError { source, analysis } => {
                let descriptor = source.descriptor();
                let mut details = ErrorDetails::new(
                    descriptor.code.as_str(),
                    descriptor.category.as_str(),
                    descriptor.retryable,
                );
                details.analysis = analysis.clone();
                details.iterations = descriptor.iterations;
                if let Some(limit) = descriptor.resource_limit {
                    details.resource = Some(limit.resource.as_str());
                    details.requested = Some(limit.requested);
                    details.limit = Some(limit.limit);
                }
                details
            }
            Self::SimulationError { analysis, .. } => {
                let mut details = ErrorDetails::new("simulation_error", "simulation", false);
                details.analysis = analysis.clone();
                details
            }
            Self::InputNotFound { .. } => ErrorDetails::new("input_not_found", "input", false),
            Self::InputReadError { .. } => ErrorDetails::new("input_read_error", "io", true),
            Self::ParseError { .. } => ErrorDetails::new("parse_error", "netlist", false),
            Self::VerificationFailed { .. } => {
                ErrorDetails::new("verification_failed", "verification", false)
            }
            Self::TimedOut { .. } => ErrorDetails::new("timed_out", "cancellation", false),
            Self::Interrupted => ErrorDetails::new("interrupted", "cancellation", true),
            Self::OutputError { .. } | Self::AddResistorsArtifactIo { .. } => {
                ErrorDetails::new("output_error", "io", true)
            }
            Self::AddResistorsMaterialization { .. } => {
                ErrorDetails::new("addresistors_materialization", "netlist", false)
            }
            Self::OutputSerializationError { .. } => {
                ErrorDetails::new("output_serialization", "internal", false)
            }
            Self::InvalidArgument { .. } => {
                ErrorDetails::new("invalid_argument", "input_validation", false)
            }
            Self::ConfigError { .. } => {
                ErrorDetails::new("invalid_configuration", "configuration", false)
            }
            Self::CoreConfigError { source } => match source {
                rspice_core::SimulationConfigError::ResourceLimit(limit) => {
                    let mut details = ErrorDetails::new("resource_limit", "resource_limit", false);
                    details.resource = Some(limit.resource.as_str());
                    details.requested = Some(limit.requested);
                    details.limit = Some(limit.limit);
                    details
                }
                _ => ErrorDetails::new("invalid_configuration", "configuration", false),
            },
            Self::VerilogAError { .. } => ErrorDetails::new("veriloga_error", "compilation", false),
            Self::ConversionError { .. } => {
                ErrorDetails::new("conversion_error", "conversion", false)
            }
            Self::InternalError { .. } => ErrorDetails::new("internal_error", "internal", false),
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

    /// Create a simulation error
    pub fn simulation_error(message: impl Into<String>) -> Self {
        CliError::SimulationError {
            message: message.into(),
            analysis: None,
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

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::InternalError {
            message: err.to_string(),
        }
    }
}

impl From<rspice_core::error::ParseError> for CliError {
    fn from(err: rspice_core::error::ParseError) -> Self {
        CliError::ParseError {
            message: err.to_string(),
            line: None,
            suggestion: None,
        }
    }
}

impl From<rspice_core::SimulationError> for CliError {
    fn from(err: rspice_core::SimulationError) -> Self {
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
        assert_eq!(error.exit_code(), ExitCode::ConfigError);
        assert_eq!(details.code, "resource_limit");
        assert_eq!(details.resource, Some("matrix_unknowns"));
        assert_eq!(details.requested, Some(12));
        assert_eq!(details.limit, Some(10));
    }

    #[test]
    fn engine_errors_preserve_shared_machine_metadata() {
        let error = CliError::from(rspice_core::SimulationError::ConvergenceFailed(31));
        assert_eq!(error.exit_code(), ExitCode::GeneralError);
        let details = error.details();
        assert_eq!(details.code, "convergence_error");
        assert_eq!(details.category, "convergence");
        assert_eq!(details.iterations, Some(31));
        assert!(!details.retryable);
    }
}
