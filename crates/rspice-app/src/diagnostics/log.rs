//! The session log the Console page shows.
//!
//! A chronological, verbatim transcript of the session: each entry carries its
//! severity, the part of the product that wrote it and the wall-clock time it
//! was written. A run writes two lines here, its start and its end, both
//! anchored to it; everything else it says is in its own log, which the run
//! owns.
//!
//! ```text
//! studio events ──────────┐
//! run start and end ──────┼──► LogBuffer ──► Console page
//! plan and netlist ───────┘
//! run lines, engine log ─────► SimulationRun::log
//! ```

use std::collections::VecDeque;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub use rspice_app_types::diagnostics::{LogSeverity, LogSource};

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
    /// there. So is the sheet, where the drawing has sheets: an object on a
    /// sheet other than the one in view is drawn nowhere until that sheet is
    /// brought forward.
    Schematic {
        cell_view: crate::state::CellViewRef,
        x: i32,
        y: i32,
        component: Option<u64>,
        wire: Option<u64>,
        /// The sheet the place is drawn on. `None` where the drawing has no
        /// sheets, or no sheet claims the object and it is drawn on the sheet
        /// in view.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sheet: Option<crate::state::SheetId>,
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
    /// Wall-clock time the entry was written, since the Unix epoch.
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
    /// The time on the reader's clock, as HH:MM:SS.mmm.
    pub fn format_timestamp(&self) -> String {
        u64::try_from(self.timestamp.as_millis())
            .ok()
            .and_then(crate::time_compat::local_date_time)
            .map(|local| {
                format!(
                    "{:02}:{:02}:{:02}.{:03}",
                    local.hour(),
                    local.minute(),
                    local.second(),
                    local.millisecond()
                )
            })
            .unwrap_or_default()
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
    /// Entries dropped to stay within the capacity since the buffer was last
    /// cleared, so the Console can say that earlier history is gone rather
    /// than present a truncated log as the whole session.
    evicted: u64,
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
            evicted: 0,
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
            timestamp: crate::time_compat::unix_epoch(),
            severity,
            source,
            message: message.into(),
            anchor,
        };
        self.next_id += 1;

        // Remove oldest if at capacity
        if self.entries.len() >= self.capacity && self.entries.pop_front().is_some() {
            self.evicted = self.evicted.saturating_add(1);
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

    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The most entries the buffer keeps.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Entries dropped to stay within [`Self::capacity`] since the buffer was
    /// last cleared.
    pub fn evicted(&self) -> u64 {
        self.evicted
    }

    /// Clear all entries. What was cleared was removed on purpose, so it is
    /// not counted as evicted.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.evicted = 0;
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
