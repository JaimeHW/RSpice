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

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use egui::Color32;
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

    /// Short prefix for compact display
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Error => "E",
            Self::Warning => "W",
            Self::Info => "I",
            Self::Debug => "D",
            Self::Trace => "T",
        }
    }

    /// Color for UI rendering
    pub fn color(&self) -> Color32 {
        match self {
            Self::Error => Color32::from_rgb(231, 76, 60), // Red
            Self::Warning => Color32::from_rgb(241, 196, 15), // Yellow
            Self::Info => Color32::from_rgb(52, 152, 219), // Blue
            Self::Debug => Color32::from_rgb(149, 165, 166), // Gray
            Self::Trace => Color32::from_rgb(127, 140, 141), // Dark gray
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

    /// Color for UI rendering
    pub fn color(&self) -> Color32 {
        match self {
            Self::Simulation => Color32::from_rgb(46, 204, 113), // Green
            Self::Engine => Color32::from_rgb(155, 89, 182),     // Purple
            Self::Netlist => Color32::from_rgb(52, 152, 219),    // Blue
            Self::Drc => Color32::from_rgb(230, 126, 34),        // Orange
            Self::User => Color32::from_rgb(241, 196, 15),       // Yellow
            Self::System => Color32::from_rgb(149, 165, 166),    // Gray
        }
    }
}

// =============================================================================
// Log Entry
// =============================================================================

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
        };
        self.next_id += 1;

        // Remove oldest if at capacity
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }

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
    }

    /// Count entries by severity
    pub fn count_by_severity(&self, severity: LogSeverity) -> usize {
        self.entries
            .iter()
            .filter(|e| e.severity == severity)
            .count()
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
// Log Panel State
// =============================================================================

/// State for the log panel UI
#[derive(Debug, Clone)]
pub struct LogPanelState {
    /// Text filter for searching
    pub filter_text: String,
    /// Minimum severity to display
    pub filter_severity: LogSeverity,
    /// Source filters (checked = visible)
    pub source_filters: [bool; 6],
    /// Auto-scroll to bottom
    pub auto_scroll: bool,
    /// Show timestamps
    pub show_timestamps: bool,
    /// Show source column
    pub show_source: bool,
}

impl Default for LogPanelState {
    fn default() -> Self {
        Self {
            filter_text: String::new(),
            filter_severity: LogSeverity::Info,
            source_filters: [true; 6], // All sources visible
            auto_scroll: true,
            show_timestamps: true,
            show_source: true,
        }
    }
}

impl LogPanelState {
    /// Check if a source is visible
    pub fn is_source_visible(&self, source: LogSource) -> bool {
        let idx = match source {
            LogSource::Simulation => 0,
            LogSource::Engine => 1,
            LogSource::Netlist => 2,
            LogSource::Drc => 3,
            LogSource::User => 4,
            LogSource::System => 5,
        };
        self.source_filters.get(idx).copied().unwrap_or(true)
    }

    /// Check if an entry passes all filters
    pub fn passes_filter(&self, entry: &LogEntry) -> bool {
        // Severity filter
        if entry.severity > self.filter_severity {
            return false;
        }

        // Source filter
        if !self.is_source_visible(entry.source) {
            return false;
        }

        // Text filter
        entry.matches_filter(&self.filter_text)
    }
}

// =============================================================================
// Tests
// =============================================================================
