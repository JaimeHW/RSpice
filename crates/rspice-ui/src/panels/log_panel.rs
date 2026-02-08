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

use egui::{Color32, RichText, ScrollArea, Ui};
use serde::{Deserialize, Serialize};

// =============================================================================
// Log Severity Levels
// =============================================================================

/// Log severity level following Spectre conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LogSeverity {
    /// Critical errors that halt simulation
    Error,
    /// Non-fatal issues that may affect results
    Warning,
    /// Important status information
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

impl Default for LogSeverity {
    fn default() -> Self {
        Self::Info
    }
}

// =============================================================================
// Log Source Attribution
// =============================================================================

/// Source of the log entry for filtering and attribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Default for LogSource {
    fn default() -> Self {
        Self::System
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
// Log Panel Rendering
// =============================================================================

/// Render the log panel UI
pub fn render_log_panel(ui: &mut Ui, buffer: &LogBuffer, state: &mut LogPanelState) {
    // Toolbar
    ui.horizontal(|ui| {
        // Filter text
        ui.label("Filter:");
        ui.add(
            egui::TextEdit::singleline(&mut state.filter_text)
                .hint_text("Search logs...")
                .desired_width(150.0),
        );

        ui.separator();

        // Severity dropdown
        ui.label("Level:");
        egui::ComboBox::from_id_salt("log_severity")
            .selected_text(state.filter_severity.name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.filter_severity, LogSeverity::Error, "Error");
                ui.selectable_value(&mut state.filter_severity, LogSeverity::Warning, "Warning");
                ui.selectable_value(&mut state.filter_severity, LogSeverity::Info, "Info");
                ui.selectable_value(&mut state.filter_severity, LogSeverity::Debug, "Debug");
                ui.selectable_value(&mut state.filter_severity, LogSeverity::Trace, "Trace");
            });

        ui.separator();

        // Auto-scroll toggle
        ui.checkbox(&mut state.auto_scroll, "Auto-scroll");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Error/warning counts
            let errors = buffer.error_count();
            let warnings = buffer.warning_count();

            if errors > 0 {
                ui.label(RichText::new(format!("{}E", errors)).color(LogSeverity::Error.color()));
            }
            if warnings > 0 {
                ui.label(
                    RichText::new(format!("{}W", warnings)).color(LogSeverity::Warning.color()),
                );
            }
            ui.label(format!("{} entries", buffer.len()));
        });
    });

    ui.separator();

    // Log entries
    let scroll = ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(state.auto_scroll);

    scroll.show(ui, |ui| {
        for entry in buffer.entries() {
            if !state.passes_filter(entry) {
                continue;
            }

            ui.horizontal(|ui| {
                // Timestamp
                if state.show_timestamps {
                    ui.label(RichText::new(entry.format_timestamp()).weak().monospace());
                }

                // Severity badge
                ui.label(
                    RichText::new(format!("[{}]", entry.severity.prefix()))
                        .color(entry.severity.color())
                        .monospace(),
                );

                // Source badge
                if state.show_source {
                    ui.label(
                        RichText::new(format!("[{}]", entry.source.name()))
                            .color(entry.source.color())
                            .monospace(),
                    );
                }

                // Message
                ui.label(&entry.message);

                // Context (if present)
                if let Some(ctx) = &entry.context {
                    ui.label(RichText::new(format!("({})", ctx)).weak());
                }
            });
        }
    });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // LogSeverity Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_severity_ordering() {
        // Error is most severe (lowest value for prioritization)
        assert!(LogSeverity::Error < LogSeverity::Warning);
        assert!(LogSeverity::Warning < LogSeverity::Info);
        assert!(LogSeverity::Info < LogSeverity::Debug);
        assert!(LogSeverity::Debug < LogSeverity::Trace);
    }

    #[test]
    fn test_severity_names() {
        assert_eq!(LogSeverity::Error.name(), "ERROR");
        assert_eq!(LogSeverity::Warning.name(), "WARN");
        assert_eq!(LogSeverity::Info.name(), "INFO");
        assert_eq!(LogSeverity::Debug.name(), "DEBUG");
        assert_eq!(LogSeverity::Trace.name(), "TRACE");
    }

    #[test]
    fn test_severity_prefixes() {
        assert_eq!(LogSeverity::Error.prefix(), "E");
        assert_eq!(LogSeverity::Warning.prefix(), "W");
        assert_eq!(LogSeverity::Info.prefix(), "I");
        assert_eq!(LogSeverity::Debug.prefix(), "D");
        assert_eq!(LogSeverity::Trace.prefix(), "T");
    }

    #[test]
    fn test_severity_colors_are_unique() {
        let colors = [
            LogSeverity::Error.color(),
            LogSeverity::Warning.color(),
            LogSeverity::Info.color(),
            LogSeverity::Debug.color(),
            LogSeverity::Trace.color(),
        ];
        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(colors[i], colors[j], "Colors should be unique");
            }
        }
    }

    // -------------------------------------------------------------------------
    // LogSource Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_source_names() {
        assert_eq!(LogSource::Simulation.name(), "SIM");
        assert_eq!(LogSource::Engine.name(), "ENG");
        assert_eq!(LogSource::Netlist.name(), "NET");
        assert_eq!(LogSource::Drc.name(), "DRC");
        assert_eq!(LogSource::User.name(), "USR");
        assert_eq!(LogSource::System.name(), "SYS");
    }

    #[test]
    fn test_source_colors_are_unique() {
        let colors = [
            LogSource::Simulation.color(),
            LogSource::Engine.color(),
            LogSource::Netlist.color(),
            LogSource::Drc.color(),
            LogSource::User.color(),
            LogSource::System.color(),
        ];
        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(colors[i], colors[j], "Colors should be unique");
            }
        }
    }

    // -------------------------------------------------------------------------
    // LogEntry Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_entry_creation() {
        let entry = LogEntry::new(
            0,
            Duration::from_secs(5),
            LogSeverity::Info,
            LogSource::Simulation,
            "Test message",
        );
        assert_eq!(entry.id, 0);
        assert_eq!(entry.severity, LogSeverity::Info);
        assert_eq!(entry.source, LogSource::Simulation);
        assert_eq!(entry.message, "Test message");
        assert!(entry.context.is_none());
    }

    #[test]
    fn test_entry_with_context() {
        let entry = LogEntry::new(
            1,
            Duration::from_secs(10),
            LogSeverity::Warning,
            LogSource::Engine,
            "Convergence warning",
        )
        .with_context("tran analysis");

        assert_eq!(entry.context, Some("tran analysis".to_string()));
    }

    #[test]
    fn test_entry_format_timestamp() {
        let entry = LogEntry::new(
            0,
            Duration::from_millis(3723456), // 1h 2m 3s 456ms
            LogSeverity::Info,
            LogSource::System,
            "Test",
        );
        assert_eq!(entry.format_timestamp(), "01:02:03.456");
    }

    #[test]
    fn test_entry_format_timestamp_zero() {
        let entry = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::System,
            "Test",
        );
        assert_eq!(entry.format_timestamp(), "00:00:00.000");
    }

    #[test]
    fn test_entry_matches_filter_empty() {
        let entry = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::System,
            "Some message",
        );
        assert!(entry.matches_filter(""));
    }

    #[test]
    fn test_entry_matches_filter_message() {
        let entry = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::System,
            "Simulation completed successfully",
        );
        assert!(entry.matches_filter("simulation"));
        assert!(entry.matches_filter("SIMULATION"));
        assert!(entry.matches_filter("completed"));
        assert!(!entry.matches_filter("error"));
    }

    #[test]
    fn test_entry_matches_filter_context() {
        let entry = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Simulation,
            "Analysis done",
        )
        .with_context("transient analysis");

        assert!(entry.matches_filter("transient"));
        assert!(entry.matches_filter("analysis"));
        assert!(!entry.matches_filter("ac"));
    }

    // -------------------------------------------------------------------------
    // LogBuffer Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_buffer_creation() {
        let buffer = LogBuffer::new(100);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_log_entries() {
        let mut buffer = LogBuffer::new(100);
        buffer.info(LogSource::Simulation, "First message");
        buffer.warning(LogSource::Engine, "Second message");
        buffer.error(LogSource::Drc, "Third message");

        assert_eq!(buffer.len(), 3);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_buffer_ring_behavior() {
        let mut buffer = LogBuffer::new(3);

        buffer.info(LogSource::System, "Message 1");
        buffer.info(LogSource::System, "Message 2");
        buffer.info(LogSource::System, "Message 3");
        assert_eq!(buffer.len(), 3);

        // Add 4th message - should evict "Message 1"
        buffer.info(LogSource::System, "Message 4");
        assert_eq!(buffer.len(), 3);

        let entries: Vec<_> = buffer.entries().collect();
        assert_eq!(entries[0].message, "Message 2");
        assert_eq!(entries[1].message, "Message 3");
        assert_eq!(entries[2].message, "Message 4");
    }

    #[test]
    fn test_buffer_ids_monotonic() {
        let mut buffer = LogBuffer::new(100);
        buffer.info(LogSource::System, "First");
        buffer.info(LogSource::System, "Second");
        buffer.info(LogSource::System, "Third");

        let entries: Vec<_> = buffer.entries().collect();
        assert_eq!(entries[0].id, 0);
        assert_eq!(entries[1].id, 1);
        assert_eq!(entries[2].id, 2);
    }

    #[test]
    fn test_buffer_severity_filter() {
        let mut buffer = LogBuffer::new(100);
        buffer.error(LogSource::Engine, "Error");
        buffer.warning(LogSource::Engine, "Warning");
        buffer.info(LogSource::Engine, "Info");
        buffer.debug(LogSource::Engine, "Debug");

        let errors: Vec<_> = buffer.entries_by_severity(LogSeverity::Error).collect();
        assert_eq!(errors.len(), 1);

        let warnings: Vec<_> = buffer.entries_by_severity(LogSeverity::Warning).collect();
        assert_eq!(warnings.len(), 2); // Error + Warning

        let info: Vec<_> = buffer.entries_by_severity(LogSeverity::Info).collect();
        assert_eq!(info.len(), 3); // Error + Warning + Info
    }

    #[test]
    fn test_buffer_source_filter() {
        let mut buffer = LogBuffer::new(100);
        buffer.info(LogSource::Simulation, "Sim 1");
        buffer.info(LogSource::Engine, "Eng 1");
        buffer.info(LogSource::Simulation, "Sim 2");
        buffer.info(LogSource::Drc, "DRC 1");

        let sim: Vec<_> = buffer.entries_by_source(LogSource::Simulation).collect();
        assert_eq!(sim.len(), 2);

        let eng: Vec<_> = buffer.entries_by_source(LogSource::Engine).collect();
        assert_eq!(eng.len(), 1);
    }

    #[test]
    fn test_buffer_count_by_severity() {
        let mut buffer = LogBuffer::new(100);
        buffer.error(LogSource::System, "Error 1");
        buffer.error(LogSource::System, "Error 2");
        buffer.warning(LogSource::System, "Warning 1");
        buffer.info(LogSource::System, "Info 1");
        buffer.info(LogSource::System, "Info 2");
        buffer.info(LogSource::System, "Info 3");

        assert_eq!(buffer.error_count(), 2);
        assert_eq!(buffer.warning_count(), 1);
        assert_eq!(buffer.count_by_severity(LogSeverity::Info), 3);
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = LogBuffer::new(100);
        buffer.info(LogSource::System, "Message 1");
        buffer.info(LogSource::System, "Message 2");
        assert_eq!(buffer.len(), 2);

        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_min_severity_filter() {
        let mut buffer = LogBuffer::new(100);
        buffer.set_min_severity(LogSeverity::Warning);

        buffer.error(LogSource::System, "Error"); // Should be stored
        buffer.warning(LogSource::System, "Warning"); // Should be stored
        buffer.info(LogSource::System, "Info"); // Should be filtered out

        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_buffer_with_context() {
        let mut buffer = LogBuffer::new(100);
        buffer.info_with_context(LogSource::Simulation, "Analysis done", "tran 1ms");

        let entry = buffer.entries().next().unwrap();
        assert_eq!(entry.context, Some("tran 1ms".to_string()));
    }

    // -------------------------------------------------------------------------
    // LogPanelState Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_panel_state_default() {
        let state = LogPanelState::default();
        assert!(state.filter_text.is_empty());
        assert_eq!(state.filter_severity, LogSeverity::Info);
        assert!(state.auto_scroll);
        assert!(state.show_timestamps);
        assert!(state.show_source);
    }

    #[test]
    fn test_panel_state_source_visibility() {
        let state = LogPanelState::default();
        assert!(state.is_source_visible(LogSource::Simulation));
        assert!(state.is_source_visible(LogSource::Engine));
        assert!(state.is_source_visible(LogSource::Netlist));
        assert!(state.is_source_visible(LogSource::Drc));
        assert!(state.is_source_visible(LogSource::User));
        assert!(state.is_source_visible(LogSource::System));
    }

    #[test]
    fn test_panel_state_source_filter_toggle() {
        let mut state = LogPanelState::default();
        state.source_filters[0] = false; // Disable Simulation

        assert!(!state.is_source_visible(LogSource::Simulation));
        assert!(state.is_source_visible(LogSource::Engine));
    }

    #[test]
    fn test_panel_state_passes_filter_severity() {
        let state = LogPanelState {
            filter_severity: LogSeverity::Warning,
            ..Default::default()
        };

        let error = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Error,
            LogSource::System,
            "E",
        );
        let warning = LogEntry::new(
            1,
            Duration::ZERO,
            LogSeverity::Warning,
            LogSource::System,
            "W",
        );
        let info = LogEntry::new(2, Duration::ZERO, LogSeverity::Info, LogSource::System, "I");

        assert!(state.passes_filter(&error));
        assert!(state.passes_filter(&warning));
        assert!(!state.passes_filter(&info)); // Below threshold
    }

    #[test]
    fn test_panel_state_passes_filter_source() {
        let mut state = LogPanelState::default();
        state.source_filters[1] = false; // Disable Engine

        let sim = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Simulation,
            "Sim",
        );
        let eng = LogEntry::new(
            1,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Engine,
            "Eng",
        );

        assert!(state.passes_filter(&sim));
        assert!(!state.passes_filter(&eng)); // Source disabled
    }

    #[test]
    fn test_panel_state_passes_filter_text() {
        let state = LogPanelState {
            filter_text: "convergence".to_string(),
            ..Default::default()
        };

        let matches = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Engine,
            "Convergence achieved",
        );
        let no_match = LogEntry::new(
            1,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Engine,
            "Timestep reduced",
        );

        assert!(state.passes_filter(&matches));
        assert!(!state.passes_filter(&no_match));
    }

    #[test]
    fn test_panel_state_combined_filters() {
        let mut state = LogPanelState {
            filter_text: "error".to_string(),
            filter_severity: LogSeverity::Warning,
            ..Default::default()
        };
        state.source_filters[0] = false; // Disable Simulation

        // Entry that passes all filters
        let passes = LogEntry::new(
            0,
            Duration::ZERO,
            LogSeverity::Error,
            LogSource::Engine,
            "Error occurred",
        );
        assert!(state.passes_filter(&passes));

        // Fails severity filter
        let info = LogEntry::new(
            1,
            Duration::ZERO,
            LogSeverity::Info,
            LogSource::Engine,
            "Error occurred",
        );
        assert!(!state.passes_filter(&info));

        // Fails source filter
        let sim = LogEntry::new(
            2,
            Duration::ZERO,
            LogSeverity::Error,
            LogSource::Simulation,
            "Error occurred",
        );
        assert!(!state.passes_filter(&sim));

        // Fails text filter
        let no_text = LogEntry::new(
            3,
            Duration::ZERO,
            LogSeverity::Error,
            LogSource::Engine,
            "Something else",
        );
        assert!(!state.passes_filter(&no_text));
    }
}
