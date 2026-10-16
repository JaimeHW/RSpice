//! The session log the Console page shows.
//!
//! A chronological, verbatim transcript: each entry carries its severity, the
//! part of the product that wrote it and the time on the session clock.
//!
//! ```text
//! studio events ────┐
//! run lifecycle ────┼──► LogBuffer ──► Console page
//! engine log ───────┘
//! ```

use crate::time_compat::Instant;
use std::collections::VecDeque;
use std::time::Duration;

use serde::{Deserialize, Serialize};

// =============================================================================
// Log Severity Levels
// =============================================================================

pub use rspice_app_types::diagnostics::LogSeverity;

// =============================================================================
// Log Source Attribution
// =============================================================================

/// The part of the product an entry came from.
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
    /// A run's lifecycle: queued, started, finished, failed.
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

// =============================================================================
// Log Entry
// =============================================================================

/// A canvas location a console row can jump to (click-to-source).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogAnchor {
    /// A schematic-space point in one cell view, optionally with the object
    /// to select on arrival.
    ///
    /// The cell view is part of the place: a point and an object id mean
    /// something only in the drawing they were read off, and following one
    /// into whichever view happened to be open selected an unrelated object
    /// there.
    Schematic {
        cell_view: crate::state::CellViewRef,
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
    /// The design objects a failed run named, addressed by name.
    ///
    /// [`Self::Schematic`] stores geometry captured when the finding was
    /// made; this stores names and resolves them when clicked, against the
    /// cross-probe map the run left behind. That is the difference that
    /// makes staleness answerable: a schematic edited since the run has no
    /// current map, so the jump refuses instead of centering on where a
    /// conductor used to be.
    Simulation {
        /// Node names, as the engine spelled them.
        nets: Vec<String>,
        /// Device instance names, for a failure attributed to a branch.
        devices: Vec<String>,
    },
    /// The retained dataset one run produced, addressed by display sequence.
    ///
    /// A run's outcome is reported twice — once as a console line and once as
    /// a session notice — and before this the two had nothing in common but
    /// their wording, so neither could tell it was describing the same event
    /// as the other. This is the identity they now share: a typed run, not a
    /// sentence about one.
    ResultRun { run_sequence: u64 },
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
    /// Optional jump target — anchored rows render clickable in the console.
    #[serde(default)]
    pub anchor: Option<LogAnchor>,
}

impl LogEntry {
    /// Format timestamp as HH:MM:SS.mmm
    pub fn format_timestamp(&self) -> String {
        let total_secs = self.timestamp.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        let millis = self.timestamp.subsec_millis();
        format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
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
        }
    }

    /// Append an entry and return its id.
    pub fn log(
        &mut self,
        severity: LogSeverity,
        source: LogSource,
        message: impl Into<String>,
    ) -> u64 {
        self.log_anchored(severity, source, message, None)
    }

    /// Append an entry with a jump target and return its id — the console
    /// renders anchored rows clickable and jumps to the anchor on click.
    pub fn log_anchored(
        &mut self,
        severity: LogSeverity,
        source: LogSource,
        message: impl Into<String>,
        anchor: Option<LogAnchor>,
    ) -> u64 {
        let id = self.next_id;
        let entry = LogEntry {
            id,
            timestamp: self.session_start.elapsed(),
            severity,
            source,
            message: message.into(),
            anchor,
        };
        self.next_id += 1;

        // Remove oldest if at capacity
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
        id
    }

    /// Convenience: Log a warning
    #[cfg(test)]
    pub fn warning(&mut self, source: LogSource, message: impl Into<String>) {
        self.log(LogSeverity::Warning, source, message);
    }

    /// Get all entries
    pub fn entries(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// Monotonic revision: changes whenever the buffer contents change
    /// (paired with `len()` to also detect `clear`).
    pub fn revision(&self) -> u64 {
        self.next_id
    }

    /// Elapsed time on the same session clock used by [`LogEntry::timestamp`].
    ///
    /// UI projections can subtract an entry timestamp from this value to
    /// preserve the event's original age when mapping it onto another clock.
    pub fn session_elapsed(&self) -> Duration {
        self.session_start.elapsed()
    }

    /// Get entries filtered by source
    #[cfg(test)]
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
    }

    /// Count entries of one severity.
    #[cfg(test)]
    pub fn count_by_severity(&self, severity: LogSeverity) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.severity == severity)
            .count()
    }
}
