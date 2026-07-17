//! CLI Error Types
//!
//! Professional error handling with structured error types,
//! exit codes following GNU conventions, and helpful diagnostics.

use std::path::{Path, PathBuf};
use thiserror::Error;

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
            CliError::VerificationFailed { .. } => ExitCode::VerificationFailed,
            CliError::TimedOut { .. } => ExitCode::TimedOut,
            CliError::Interrupted => ExitCode::Interrupted,
            CliError::OutputError { .. } => ExitCode::IoError,
            CliError::AddResistorsMaterialization { .. } => ExitCode::InputError,
            CliError::AddResistorsArtifactIo { .. } => ExitCode::IoError,
            CliError::OutputSerializationError { .. } => ExitCode::InternalError,
            CliError::InvalidArgument { .. } => ExitCode::MisuseOfCommand,
            CliError::ConfigError { .. } => ExitCode::ConfigError,
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
        CliError::SimulationError {
            message: err.to_string(),
            analysis: None,
        }
    }
}
