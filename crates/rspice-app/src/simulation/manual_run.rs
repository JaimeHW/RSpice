//! The handoff between a manual-deck run and the deck it ran.
//!
//! The run controller captures the editor buffer when a manual-deck run
//! starts, and promotes it to the baseline when that run completes. The
//! netlist editor reads the baseline to mark the lines edited since, and the
//! run-snapshot document shows it.
//!
//! Runtime state only: none of it is persisted. A project change resets it
//! together with the editor.

use std::collections::{HashMap, HashSet};

/// The deck a manual-deck run was started from, and the baseline the last
/// completed one left.
#[derive(Debug, Clone, Default)]
pub(crate) struct ManualRunHandoff {
    /// Zero-based lines edited since the last completed manual-deck run.
    pub(crate) edited_lines: HashSet<usize>,
    /// Exact editor buffer from the last successful manual-deck run.
    pub(crate) last_run_buffer: Option<String>,
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub(crate) last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual-deck run started.
    pub(crate) pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub(crate) pending_manual_run_id: Option<u64>,
    /// Run sequence the manual-run baseline was sealed with. This is the only
    /// deck-to-run binding: the run's label, deck digest and revision are read
    /// back off that run's own receipt rather than copied here.
    pub(crate) last_run_id: Option<u64>,
}
