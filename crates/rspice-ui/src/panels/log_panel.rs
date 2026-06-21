//! Simulation Log Panel
//!
//! Professional-grade logging infrastructure for RSpice following Spectre conventions.
//! Provides structured log entries with severity levels, timestamps, and source attribution.
//!
//! # Architecture
//!
//! ```text
//! SimulationController ─┐
//! EngineBridge ─────────┼──► LogBuffer ──► LogPanel (UI)
//! DRC Checker ──────────┘
//! ```
//!
//! # Features
//!
//! - Ring buffer for memory-efficient log storage
//! - Severity-based filtering (Error, Warning, Info, Debug, Trace)
//! - Source attribution (Simulation, Engine, Netlist, DRC, User)
//! - Timestamp with sub-millisecond precision
//! - Search/filter by text pattern

use crate::common::time_compat::Instant;
use std::collections::VecDeque;
use std::time::Duration;

use serde::{Deserialize, Serialize};

// =============================================================================
// Log Severity Levels
// =============================================================================

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

// =============================================================================
// Log Source Attribution
// =============================================================================

/// Source of the log entry for filtering and attribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LogSource {
    /// Simulation controller events (start, stop, progress)
    Simulation,
    /// Core engine events (convergence, timestep adaptation)
    Engine,
    /// Netlist parsing and validation
    Netlist,
    /// Design rule checking
    Drc,
    /// User-initiated actions
    User,
    /// System/infrastructure messages
    #[default]
    System,
}

impl LogSource {
    /// Display name for the source
    pub fn name(&self) -> &'static str {
        match self {
            Self::Simulation => "SIM",
            Self::Engine => "ENG",
            Self::Netlist => "NET",
            Self::Drc => "DRC",
            Self::User => "USR",
            Self::System => "SYS",
        }
    }
}

// =============================================================================
// Log Entry
// =============================================================================

/// A canvas location a console row can jump to (click-to-source).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogAnchor {
    /// A schematic-space point, optionally with the object to select on
    /// arrival.
    Schematic {
        x: i32,
        y: i32,
        component: Option<u64>,
        wire: Option<u64>,
    },
    /// A symbol pin in a symbol view.
    Symbol {
        reference: crate::state::CellViewRef,
        pin_name: String,
        point: Option<crate::state::Point>,
    },
}

/// A single log entry with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Unique sequential ID for ordering
    pub id: u64,
    /// Timestamp relative to session start
    pub timestamp: Duration,
    /// Severity level
    pub severity: LogSeverity,
    /// Source component
    pub source: LogSource,
    /// Main message text
    pub message: String,
    /// Optional context (analysis name, component, etc.)
    pub context: Option<String>,
    /// Optional jump target — anchored rows render clickable in the console.
    #[serde(default)]
    pub anchor: Option<LogAnchor>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(
        id: u64,
        timestamp: Duration,
        severity: LogSeverity,
        source: LogSource,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            timestamp,
            severity,
            source,
            message: message.into(),
            context: None,
            anchor: None,
        }
    }

    /// Add context to the entry
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Format timestamp as HH:MM:SS.mmm
    pub fn format_timestamp(&self) -> String {
        let total_secs = self.timestamp.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        let millis = self.timestamp.subsec_millis();
        format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
    }

    /// Check if entry matches a text filter
    pub fn matches_filter(&self, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }
        let lower_filter = filter.to_lowercase();
        self.message.to_lowercase().contains(&lower_filter)
            || self
                .context
                .as_ref()
                .is_some_and(|c| c.to_lowercase().contains(&lower_filter))
    }
}

// =============================================================================
// Log Buffer (Ring Buffer)
// =============================================================================

/// Ring buffer for memory-efficient log storage
///
/// Maintains a fixed-size buffer, automatically discarding oldest entries
/// when capacity is reached. This is critical for long-running sessions.
#[derive(Debug, Clone)]
pub struct LogBuffer {
    /// Circular buffer of log entries
    entries: VecDeque<LogEntry>,
    /// Maximum capacity
    capacity: usize,
    /// Next entry ID (monotonically increasing)
    next_id: u64,
    /// Session start time for relative timestamps
    session_start: Instant,
    /// Minimum severity level to store
    min_severity: LogSeverity,
    /// Running per-severity counts — the console header reads these every
    /// frame, so they must not be O(buffer) scans.
    severity_counts: [usize; 5],
}

/// Index into `severity_counts`.
fn severity_index(severity: LogSeverity) -> usize {
    match severity {
        LogSeverity::Error => 0,
        LogSeverity::Warning => 1,
        LogSeverity::Info => 2,
        LogSeverity::Debug => 3,
        LogSeverity::Trace => 4,
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new(10_000) // 10K entries default
    }
}

impl LogBuffer {
    /// Create a new log buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity.min(1024)), // Pre-allocate up to 1K
            capacity,
            next_id: 0,
            session_start: Instant::now(),
            min_severity: LogSeverity::Trace, // Store all by default
            severity_counts: [0; 5],
        }
    }

    /// Set minimum severity level to store
    pub fn set_min_severity(&mut self, severity: LogSeverity) {
        self.min_severity = severity;
    }

    /// Log a message with full parameters
    pub fn log(
        &mut self,
        severity: LogSeverity,
        source: LogSource,
        message: impl Into<String>,
        context: Option<String>,
    ) {
        self.log_anchored(severity, source, message, context, None);
    }

    /// Log with a jump target — the console renders anchored rows
    /// clickable and jumps to the anchor on click.
    pub fn log_anchored(
        &mut self,
        severity: LogSeverity,
        source: LogSource,
        message: impl Into<String>,
        context: Option<String>,
        anchor: Option<LogAnchor>,
    ) {
        // Filter by minimum severity
        if severity > self.min_severity {
            return;
        }

        let entry = LogEntry {
            id: self.next_id,
            timestamp: self.session_start.elapsed(),
            severity,
            source,
            message: message.into(),
            context,
            anchor,
        };
        self.next_id += 1;

        // Remove oldest if at capacity
        if self.entries.len() >= self.capacity
            && let Some(evicted) = self.entries.pop_front()
        {
            self.severity_counts[severity_index(evicted.severity)] -= 1;
        }

        self.severity_counts[severity_index(entry.severity)] += 1;
        self.entries.push_back(entry);
    }

    /// Convenience: Log an error
    pub fn error(&mut self, source: LogSource, message: impl Into<String>) {
        self.log(LogSeverity::Error, source, message, None);
    }

    /// Convenience: Log a warning
    pub fn warning(&mut self, source: LogSource, message: impl Into<String>) {
        self.log(LogSeverity::Warning, source, message, None);
    }

    /// Convenience: Log info
    pub fn info(&mut self, source: LogSource, message: impl Into<String>) {
        self.log(LogSeverity::Info, source, message, None);
    }

    /// Convenience: Log debug
    pub fn debug(&mut self, source: LogSource, message: impl Into<String>) {
        self.log(LogSeverity::Debug, source, message, None);
    }

    /// Convenience: Log with context
    pub fn info_with_context(
        &mut self,
        source: LogSource,
        message: impl Into<String>,
        context: impl Into<String>,
    ) {
        self.log(LogSeverity::Info, source, message, Some(context.into()));
    }

    /// Get all entries
    pub fn entries(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// Get an entry by position (oldest = 0), for virtualized rendering.
    pub fn entry(&self, index: usize) -> Option<&LogEntry> {
        self.entries.get(index)
    }

    /// Monotonic revision: changes whenever the buffer contents change
    /// (paired with `len()` to also detect `clear`).
    pub fn revision(&self) -> u64 {
        self.next_id
    }

    /// Get entries filtered by severity
    pub fn entries_by_severity(
        &self,
        min_severity: LogSeverity,
    ) -> impl Iterator<Item = &LogEntry> {
        self.entries
            .iter()
            .filter(move |e| e.severity <= min_severity)
    }

    /// Get entries filtered by source
    pub fn entries_by_source(&self, source: LogSource) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter().filter(move |e| e.source == source)
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.severity_counts = [0; 5];
    }

    /// Clear entries from one source while preserving unrelated console history.
    pub fn clear_source(&mut self, source: LogSource) {
        self.entries.retain(|entry| {
            if entry.source == source {
                self.severity_counts[severity_index(entry.severity)] -= 1;
                false
            } else {
                true
            }
        });
    }

    /// Count entries by severity (O(1) — maintained on log/evict/clear)
    pub fn count_by_severity(&self, severity: LogSeverity) -> usize {
        self.severity_counts[severity_index(severity)]
    }

    /// Get error count (useful for status bar)
    pub fn error_count(&self) -> usize {
        self.count_by_severity(LogSeverity::Error)
    }

    /// Get warning count (useful for status bar)
    pub fn warning_count(&self) -> usize {
        self.count_by_severity(LogSeverity::Warning)
    }
}

// =============================================================================
// Tests
// =============================================================================
