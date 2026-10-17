//! Stable source-diagnostic identity and collection bounds shared by producers and views.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod metadata;
mod source;
pub use metadata::{
    CanonicalDiagnosticMetadata, CanonicalQuickFix, DiagnosticConsumer, DiagnosticCurrentness,
    DiagnosticRelatedLocation, DiagnosticSuppression, DiagnosticTextEdit,
};
pub use source::{DiagnosticSeverity, SourceDiagnostic};

pub const MAX_DIAGNOSTICS: usize = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticPosition {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRange {
    pub start: DiagnosticPosition,
    pub end: DiagnosticPosition,
}

pub fn diagnostic_range(
    byte_range: Option<&std::ops::Range<usize>>,
    line: Option<usize>,
    column: Option<usize>,
) -> Option<DiagnosticRange> {
    if byte_range.is_none() && line.is_none() && column.is_none() {
        return None;
    }
    let byte_range = byte_range.cloned().unwrap_or(0..0);
    let start = DiagnosticPosition {
        line: line.unwrap_or(0),
        column: column.unwrap_or(0),
        byte_offset: byte_range.start,
    };
    Some(DiagnosticRange {
        start,
        end: DiagnosticPosition {
            byte_offset: byte_range.end,
            ..start
        },
    })
}

pub fn stable_diagnostic_id(
    source: &str,
    code: &str,
    document_id: &str,
    range: Option<DiagnosticRange>,
    revision: u64,
    validation_id: &str,
    message: &str,
) -> uuid::Uuid {
    const NAMESPACE: uuid::Uuid = uuid::Uuid::from_u128(0x9cc9d141_951a_5553_b787_b2c80c98c43f);
    let mut identity = String::with_capacity(
        source.len() + code.len() + document_id.len() + validation_id.len() + message.len() + 96,
    );
    for value in [source, code, document_id, validation_id, message] {
        identity.push_str(&value.len().to_string());
        identity.push(':');
        identity.push_str(value);
        identity.push('|');
    }
    identity.push_str(&revision.to_string());
    if let Some(range) = range {
        use std::fmt::Write as _;
        let _ = write!(
            identity,
            "|{}:{}:{}-{}:{}:{}",
            range.start.line,
            range.start.column,
            range.start.byte_offset,
            range.end.line,
            range.end.column,
            range.end.byte_offset
        );
    }
    uuid::Uuid::new_v5(&NAMESPACE, identity.as_bytes())
}

pub fn validate_diagnostic_count(count: usize) -> Result<(), String> {
    if count > MAX_DIAGNOSTICS {
        return Err(format!(
            "Diagnostic collection contains {count} records; the supported maximum is {MAX_DIAGNOSTICS}."
        ));
    }
    Ok(())
}

pub fn insert_diagnostic_id(ids: &mut HashSet<uuid::Uuid>, id: uuid::Uuid) -> Result<(), String> {
    if !ids.insert(id) {
        return Err(format!(
            "Diagnostic collection contains duplicate canonical ID {id}."
        ));
    }
    Ok(())
}

/// Log severity level following Spectre conventions
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum LogSeverity {
    /// Critical errors that halt simulation
    Error,
    /// Non-fatal issues that may affect results
    Warning,
    /// Important status information
    #[default]
    Info,
    /// Detailed diagnostic information
    Debug,
    /// Fine-grained tracing for development
    Trace,
}

impl LogSeverity {
    /// Display name for the severity level
    pub fn name(&self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warning => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }
}

/// The part of the product a log line came from.
///
/// One vocabulary for both logs the Console reads: the session's transcript
/// holds studio, plan, netlist and run start and end lines, and each run's own
/// log holds that run's lines and those of the engines it drove.
///
/// Design checks have no source here: their findings are the Problems page's,
/// tied to the place they were found, not lines in a transcript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogSource {
    /// Every studio event, whether the user asked for it or a background
    /// service did it: opened, saved, exported, imported, restored, failed.
    #[serde(alias = "User", alias = "System", alias = "Drc")]
    Studio,
    /// Simulation-plan lifecycle outcomes: the receipt a committed plan
    /// command left, or the refusal that kept one from committing.
    ///
    /// Its own source because the Console is this product's session log of
    /// plan commands, the way a Spectre log records every command with its
    /// outcome, and because the activity mirror has to be able to leave these
    /// where they are. A receipt is a record of what was committed, not a
    /// notice to chase the reader with.
    Plan,
    /// A run's lifecycle: started, each analysis finished, completed or
    /// failed.
    #[serde(alias = "Simulation")]
    Run,
    /// The circuit engine's own account of a solve.
    Engine,
    /// Netlist import and parse notices.
    Netlist,
    /// The digital (HDL) engine's own account of a co-simulation.
    Hdl,
    /// The Verilog-A compiler and runtime.
    VerilogA,
}

impl LogSource {
    /// The tag the Console's source column prints.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Studio => "APP",
            Self::Plan => "PLAN",
            Self::Run => "RUN",
            Self::Engine => "ENG",
            Self::Netlist => "NET",
            Self::Hdl => "HDL",
            Self::VerilogA => "VA",
        }
    }
}
