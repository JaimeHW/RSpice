//! Cross-Probe State
//!
//! UI state for cross-probe system.

use super::registry::CrossProbeRegistry;
use super::signal::{ProbeableSignal, SignalType};

// =============================================================================
// Selection Mode
// =============================================================================

/// Selection behavior mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single selection (replace previous)
    #[default]
    Single,
    /// Multiple selection (add to previous)
    Multiple,
    /// Toggle selection
    Toggle,
    /// Range selection (shift-click style)
    Range,
}

impl SelectionMode {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Single => "Single",
            Self::Multiple => "Multiple",
            Self::Toggle => "Toggle",
            Self::Range => "Range",
        }
    }

    /// All modes
    pub fn all() -> &'static [SelectionMode] {
        &[Self::Single, Self::Multiple, Self::Toggle, Self::Range]
    }
}

// =============================================================================
// View Filter
// =============================================================================

/// Filter for signal types to display
#[derive(Debug, Clone, Default)]
pub struct SignalFilter {
    /// Show voltage signals
    pub show_voltages: bool,
    /// Show current signals
    pub show_currents: bool,
    /// Show power signals
    pub show_power: bool,
    /// Show digital signals
    pub show_digital: bool,
    /// Show other signals
    pub show_other: bool,
    /// Name pattern filter
    pub name_pattern: Option<String>,
}

impl SignalFilter {
    /// Create filter showing all signals
    pub fn all() -> Self {
        Self {
            show_voltages: true,
            show_currents: true,
            show_power: true,
            show_digital: true,
            show_other: true,
            name_pattern: None,
        }
    }

    /// Create filter showing only voltages
    pub fn voltages_only() -> Self {
        Self {
            show_voltages: true,
            ..Default::default()
        }
    }

    /// Create filter showing only currents
    pub fn currents_only() -> Self {
        Self {
            show_currents: true,
            ..Default::default()
        }
    }

    /// Check if signal matches filter
    pub fn matches(&self, signal: &ProbeableSignal) -> bool {
        // Check type filter
        let type_match = match signal.signal_type {
            SignalType::Voltage => self.show_voltages,
            SignalType::Current => self.show_currents,
            SignalType::Power => self.show_power,
            SignalType::Digital => self.show_digital,
            SignalType::Impedance => self.show_other,
            SignalType::Other => self.show_other,
        };

        if !type_match {
            return false;
        }

        // Check name pattern
        if let Some(ref pattern) = self.name_pattern
            && !signal.id.matches(pattern)
        {
            return false;
        }

        true
    }

    /// Set name pattern
    pub fn with_pattern(mut self, pattern: &str) -> Self {
        self.name_pattern = Some(pattern.to_string());
        self
    }

    /// Toggle voltage visibility
    pub fn toggle_voltages(&mut self) {
        self.show_voltages = !self.show_voltages;
    }

    /// Toggle current visibility
    pub fn toggle_currents(&mut self) {
        self.show_currents = !self.show_currents;
    }

    /// Any type enabled?
    pub fn any_enabled(&self) -> bool {
        self.show_voltages
            || self.show_currents
            || self.show_power
            || self.show_digital
            || self.show_other
    }
}

// =============================================================================
// Cross-Probe State
// =============================================================================

/// Complete cross-probe state
#[derive(Default)]
pub struct CrossProbeState {
    /// Central registry
    pub registry: CrossProbeRegistry,
    /// Selection mode
    pub selection_mode: SelectionMode,
    /// Signal filter
    pub filter: SignalFilter,
    /// Is signal browser expanded?
    pub browser_expanded: bool,
    /// Show signal types in browser
    pub show_signal_types: bool,
    /// Show full hierarchical paths
    pub show_full_paths: bool,
    /// Auto-scroll to selected
    pub auto_scroll: bool,
    /// Sync waveform cursor with schematic probe
    pub sync_cursor: bool,
    /// Last clicked coordinates (for context menu)
    pub last_click: Option<(f32, f32)>,
}

impl CrossProbeState {
    /// Create new state
    pub fn new() -> Self {
        Self {
            filter: SignalFilter::all(),
            browser_expanded: true,
            show_signal_types: true,
            show_full_paths: false,
            auto_scroll: true,
            sync_cursor: true,
            ..Default::default()
        }
    }

    /// Get filtered signals from registry
    pub fn filtered_signals(&self) -> Vec<&ProbeableSignal> {
        self.registry
            .signals()
            .filter(|s| self.filter.matches(s))
            .collect()
    }

    /// Get signals at current path
    pub fn signals_at_current_path(&self) -> Vec<&ProbeableSignal> {
        let path = self.registry.current_path();
        self.registry
            .signals_in_hierarchy(path)
            .into_iter()
            .filter(|s| self.filter.matches(s))
            .collect()
    }

    /// Set selection mode
    pub fn set_selection_mode(&mut self, mode: SelectionMode) {
        self.selection_mode = mode;
    }

    /// Toggle browser
    pub fn toggle_browser(&mut self) {
        self.browser_expanded = !self.browser_expanded;
    }

    /// Toggle full paths
    pub fn toggle_full_paths(&mut self) {
        self.show_full_paths = !self.show_full_paths;
    }

    /// Toggle auto-scroll
    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
    }

    /// Toggle cursor sync
    pub fn toggle_sync_cursor(&mut self) {
        self.sync_cursor = !self.sync_cursor;
    }
}

// =============================================================================
// Tests
// =============================================================================
