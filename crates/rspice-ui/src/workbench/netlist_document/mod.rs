//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion,
//! parameter tuning, and run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

use std::collections::{HashMap, HashSet};

use crate::common::AppState;

mod baseline;
mod completion;
pub mod diagnostics;
mod editor;
mod highlight;
mod summary;
mod tuner;

pub use diagnostics::{Diagnostic, DiagnosticSeverity};
pub use editor::show as show_editor;
pub use tuner::right_panel as show_parameter_inspector;

/// Transient state for one netlist document.
#[derive(Debug, Clone, Default)]
pub struct NetlistDocumentState {
    /// Buffer revision, bumped on every edit.
    pub revision: u64,
    /// Revision the diagnostics were parsed for.
    diag_revision: Option<u64>,
    /// `ui.input(..).time` of the last edit, for parse debounce.
    last_edit_time: f64,
    /// Current parse diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// Zero-based lines edited since the last completed manual-deck run.
    pub edited_lines: HashSet<usize>,
    /// Result data version last reconciled with the editor baseline.
    seen_data_version: u64,
    /// Exact editor buffer from the last successful manual-deck run.
    pub last_run_buffer: Option<String>,
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual-deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub pending_manual_run_id: Option<u64>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
    /// Whether parameter tuning re-runs on every slider movement.
    pub tuner_live: bool,
    /// A re-run requested while the engine was busy.
    pub rerun_queued: bool,
    /// Stable parameter slider ranges.
    pub param_ranges: HashMap<String, (f64, f64)>,
    /// Whether the completion popover was open last frame.
    pub completion_open: bool,
    /// Selected completion row.
    pub completion_index: usize,
    /// Revision at which completion was dismissed.
    pub completion_dismissed_at: Option<u64>,
    /// Harvested `.model` and `.subckt` symbols.
    symbols: Vec<completion::SymbolEntry>,
}

/// Reconcile queued execution and diff state before rendering the document.
pub fn prepare(state: &mut AppState) {
    if state.ui.netlist.rerun_queued && !state.simulation.is_running {
        state.ui.netlist.rerun_queued = false;
        state.request_netlist_manual_deck_run();
    }

    let data_version = state.simulation.data_version;
    if state.ui.netlist.seen_data_version == data_version {
        return;
    }
    state.ui.netlist.seen_data_version = data_version;
    if let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() {
        state.ui.netlist.edited_lines =
            baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
    }
}

pub(super) fn request_run(state: &mut AppState) {
    state.request_netlist_manual_deck_run();
}

pub(super) fn refresh_diff_pips_from_baseline(state: &mut AppState) {
    let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() else {
        return;
    };
    state.ui.netlist.edited_lines =
        baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
}

/// Numeric assignments in the active source, used by automation and tests.
pub fn buffer_assignments(buffer: &str) -> Vec<(String, usize, usize)> {
    tuner::buffer_assignments(buffer)
}
