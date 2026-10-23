//! The shared project model.
//!
//! [`Model`] is the part of [`AppState`](super::AppState) that the project or
//! session persists and that two or more features read, plus the policy and
//! evidence they act on: the design and its libraries, the simulation and its
//! setup, the checks and diagnostics, the notices not yet raised, the
//! reader's preferences, and the safe-mode and live-session write policy. The
//! application holds it inline, so moving a state carries what it carried
//! before these fields were gathered here.

use crate::state::notices::NoticeQueue;
use crate::state::{SchematicState, SimulationState};

use super::design_history::ProjectDesignHistory;
use super::{DesignCheckRuntime, DiagnosticStore, SimSetupState};

/// The project model every feature reads.
#[derive(Clone)]
pub(crate) struct Model {
    /// Circuit schematic state (components, wires, topology)
    pub(crate) schematic: SchematicState,
    /// Simulation results and waveforms
    pub(crate) simulation: SimulationState,
    /// Monotonic token that invalidates in-flight controller work whenever an
    /// unrelated design document replaces the active project/schematic.
    pub(crate) design_execution_epoch: u64,
    /// Monotonic identity for the active schematic buffer. Unlike the
    /// execution epoch, this advances on ordinary cell/view navigation so an
    /// A→B→A round trip cannot revive a stale editor transaction.
    pub(crate) active_schematic_epoch: u64,
    /// Runtime-only cross-document design transactions. Schematic-local undo
    /// cannot own operations that also create cells, views, or buffers.
    pub(crate) project_design_history: ProjectDesignHistory,
    /// Per-cell-view interactive check receipts, bound to exact live inputs.
    pub(crate) design_checks: DesignCheckRuntime,
    /// The conditions that stand against the design, from every producer, in
    /// the one shape the Problems page, the tab badge and the status bar read.
    pub(crate) diagnostics: DiagnosticStore,
    /// Typed analysis configuration behind the Simulate view.
    pub(crate) sim_setup: SimSetupState,
    /// Structured log history buffer (ring-buffer, filterable).
    pub(crate) log_buffer: crate::diagnostics::LogBuffer,
    /// Failures and warnings reported and not yet toasted. Each one's line is
    /// already in [`Self::log_buffer`]; the shell raises the toasts. Runtime
    /// only: never persisted, and a clone starts empty.
    pub(crate) notices: NoticeQueue,
    /// The generated deck and its validation evidence. Runtime only. Boxed,
    /// as it was inside the UI session state, so a moved state stays the same
    /// size.
    pub(crate) netlist_artifacts: Box<crate::state::NetlistArtifacts>,
    /// Library/Cell/View manager for design hierarchy
    pub(crate) library_manager: crate::state::LibraryManager,
    /// Runtime collaboration-lock authority for governed project library
    /// mutations. Local projects remain explicitly unmanaged; connected
    /// sessions install revision-bound authoritative snapshots.
    pub(crate) library_edit_locks: crate::state::ProjectLibraryLockAuthority,
    /// Project/workspace model for active design context and open LCV views.
    pub(crate) workspace: crate::state::ProjectWorkspace,
    /// Whether a project is open, as model code reads it. Only the lifecycle
    /// writes it, in the call that opens or closes the project.
    pub(crate) project_open: crate::state::ProjectOpenFact,
    /// PDK model paths plus signed technology packages, trust roots, exact
    /// active binding, and administrative receipts.
    pub(crate) pdk_config: crate::state::pdk_config::PdkConfig,
    /// Model library manager (PDK models, device libraries)
    pub(crate) model_library_manager: crate::state::model_library::ModelLibraryManager,
    /// Canonical user/device preference overrides not already owned by the
    /// theme or autosave runtime. Boxed, as they were inside the UI session
    /// state, so a moved state stays the same size.
    pub preferences: Box<crate::workbench::preferences::UserPreferences>,
    /// Safe mode applies only to this process. The prior session remains the
    /// persisted source of truth for the next ordinary launch.
    pub safe_mode: crate::state::write_policy::LocalSafeModeState,
    /// Live-session write leases held by other participants, projected by
    /// the live-session engine every frame it is attached.
    pub live_write_locks: crate::state::write_policy::LiveWriteLocks,
}

impl Model {
    /// Record an event in the Console and return its log id, as
    /// [`AppState::record`](super::AppState::record) states it.
    pub fn record(
        &mut self,
        severity: crate::diagnostics::LogSeverity,
        source: crate::diagnostics::LogSource,
        text: impl Into<String>,
    ) -> u64 {
        self.log_buffer.log(severity, source, text)
    }
}

/// What model code reads of the features' state.
///
/// The model never reaches up into a feature's slot for these. The caller
/// holding the whole application state reads them and hands them down with
/// the operation, read when the operation starts, which is when the model
/// read them before.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SlotFacts {
    /// Whether the run gate's report is retained: the dispatch findings are
    /// about that report, and read stale once it is dropped.
    pub(crate) preflight_report_retained: bool,
    /// Which review markers the drawing shows, part of the view a finding's
    /// place is read through.
    pub(crate) review_markers:
        rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility,
}
