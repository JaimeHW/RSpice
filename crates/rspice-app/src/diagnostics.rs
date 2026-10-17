//! Diagnostics the application reports about itself: the session's log, which
//! the Console shows, the engine's lines forwarded into each run's own log, and
//! the redaction applied when a log leaves the machine.
//!
//! They live here, below everything that produces a diagnostic and above
//! `state`, which a log entry may anchor to, so the simulation and schematic
//! layers report about their own work without reaching up through the shell.

pub(crate) mod engine_log;
mod log;
mod redaction;

pub use log::{LogAnchor, LogBuffer, LogEntry, LogSeverity, LogSource};
pub(crate) use redaction::{home_directory, redact_log_message};
