//! The shared project model.
//!
//! [`Model`] is the part of [`AppState`](super::AppState) that the project or
//! session persists and that two or more features read, plus the policy and
//! evidence they act on: the design and its libraries, the simulation and its
//! setup, the checks and diagnostics, the notices not yet raised, the
//! reader's preferences, and the safe-mode and live-session write policy. The
//! application holds it inline, so moving a state carries what it carried
//! before these fields were gathered here.

mod drawing_sheet;
mod schematic_sync;
mod sheet_catalog;
mod symbol_context;

use crate::state::notices::{Notice, NoticeKind, NoticeQueue};
use crate::state::{SchematicState, SimulationState};

use super::design_checks::DesignCheckRuntime;
use super::design_history::ProjectDesignHistory;
use super::diagnostic_store::DiagnosticStore;
use super::sim_setup::SimSetupState;

/// The project model every feature reads.
#[derive(Clone)]
pub struct Model {
    /// Circuit schematic state (components, wires, topology)
    pub schematic: SchematicState,
    /// Simulation results and waveforms
    pub simulation: SimulationState,
    /// Monotonic token that invalidates in-flight controller work whenever an
    /// unrelated design document replaces the active project/schematic.
    pub design_execution_epoch: u64,
    /// Monotonic identity for the active schematic buffer. Unlike the
    /// execution epoch, this advances on ordinary cell/view navigation so an
    /// A→B→A round trip cannot revive a stale editor transaction.
    pub active_schematic_epoch: u64,
    /// Runtime-only cross-document design transactions. Schematic-local undo
    /// cannot own operations that also create cells, views, or buffers.
    pub project_design_history: ProjectDesignHistory,
    /// Per-cell-view interactive check receipts, bound to exact live inputs.
    pub design_checks: DesignCheckRuntime,
    /// The conditions that stand against the design, from every producer, in
    /// the one shape the Problems page, the tab badge and the status bar read.
    pub diagnostics: DiagnosticStore,
    /// Typed analysis configuration behind the Simulate view.
    pub sim_setup: SimSetupState,
    /// Structured log history buffer (ring-buffer, filterable).
    pub log_buffer: crate::diagnostics::LogBuffer,
    /// Failures and warnings reported and not yet toasted. Each one's line is
    /// already in [`Self::log_buffer`]; the shell raises the toasts. Runtime
    /// only: never persisted, and a clone starts empty.
    pub notices: NoticeQueue,
    /// The generated deck and its validation evidence. Runtime only. Boxed,
    /// as it was inside the UI session state, so a moved state stays the same
    /// size.
    pub netlist_artifacts: Box<crate::state::NetlistArtifacts>,
    /// Library/Cell/View manager for design hierarchy
    pub library_manager: crate::state::LibraryManager,
    /// Runtime collaboration-lock authority for governed project library
    /// mutations. Local projects remain explicitly unmanaged; connected
    /// sessions install revision-bound authoritative snapshots.
    pub library_edit_locks: crate::state::ProjectLibraryLockAuthority,
    /// Project/workspace model for active design context and open LCV views.
    pub workspace: crate::state::ProjectWorkspace,
    /// Whether a project is open, as model code reads it. Only the lifecycle
    /// writes it, in the call that opens or closes the project.
    pub project_open: crate::state::ProjectOpenFact,
    /// PDK model paths plus signed technology packages, trust roots, exact
    /// active binding, and administrative receipts.
    pub pdk_config: crate::state::pdk_config::PdkConfig,
    /// Model library manager (PDK models, device libraries)
    pub model_library_manager: crate::state::model_library::ModelLibraryManager,
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
    /// The sheet-membership and generated-symbol failures the drawing sync
    /// last recorded. Runtime only.
    pub(crate) recorded_sync_failures: schematic_sync::SyncFailures,
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

    /// [`Self::record`], with a place the Console row jumps to.
    pub fn record_anchored(
        &mut self,
        severity: crate::diagnostics::LogSeverity,
        source: crate::diagnostics::LogSource,
        text: impl Into<String>,
        anchor: crate::diagnostics::LogAnchor,
    ) -> u64 {
        self.log_buffer
            .log_anchored(severity, source, text, Some(anchor))
    }

    /// What the project's technology contract is resolved against: the
    /// project, its plan, both execution catalogs and the layouts.
    pub fn technology_inputs(
        &self,
    ) -> rspice_simulation::project_technology::ProjectTechnologyInputs<'_> {
        rspice_simulation::project_technology::ProjectTechnologyInputs {
            project: &self.workspace.content.project,
            sim_setup: &self.sim_setup,
            models: self.model_library_manager.catalog(),
            resolutions: self.model_library_manager.resolution_records(),
            registry: &self.pdk_config.technology_registry,
            layouts: self.workspace.content.physical_layout_documents(),
        }
    }

    /// The chain the project's `.include` and `.lib` files resolve through:
    /// its own search paths, then the library folders set in Preferences.
    pub fn include_search_chain(
        &self,
    ) -> rspice_simulation::netlist_preparation::IncludeSearchChain {
        rspice_simulation::netlist_preparation::IncludeSearchChain::for_project(
            &self.workspace.content.project,
            self.library_folders(),
        )
    }

    /// Exact immutable result currently selected in the Results workspace.
    pub fn active_specialized_viewer_cache_provenance(
        &self,
    ) -> Option<crate::state::SpecializedViewerCacheProvenance> {
        let run = self.simulation.active_run()?;
        let analysis = self.simulation.active_analysis()?;
        Some(crate::state::SpecializedViewerCacheProvenance::for_analysis(run.dataset_id, analysis))
    }

    /// The folders searched for includes after every project's own paths.
    pub fn library_folders(&self) -> &[std::path::PathBuf] {
        self.preferences
            .folders(crate::workbench::preferences::FolderListPreference::LibraryFolders)
    }

    /// Report a failure of something the user asked for, as
    /// [`AppState::report_failure`](super::AppState::report_failure) states it.
    pub fn report_failure(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        self.report(NoticeKind::Error, title.into(), detail.into());
    }

    /// Report something that completed, but not the way the user asked, as
    /// [`AppState::report_warning`](super::AppState::report_warning) states it.
    pub fn report_warning(&mut self, title: impl Into<String>, detail: impl Into<String>) {
        self.report(NoticeKind::Warning, title.into(), detail.into());
    }

    /// The line is written first, so its identity is the one the toast offers;
    /// the toast itself is raised by the shell's next
    /// [`AppState::drain_notices`](super::AppState::drain_notices).
    fn report(&mut self, kind: NoticeKind, title: String, detail: String) {
        use crate::diagnostics::{LogSeverity, LogSource};

        let severity = match kind {
            NoticeKind::Error => LogSeverity::Error,
            NoticeKind::Warning => LogSeverity::Warning,
        };
        let log_id = self.record(severity, LogSource::Studio, format!("{title}: {detail}"));
        self.notices.push(Notice {
            kind,
            title,
            detail,
            log_id,
        });
    }
}

/// What model code reads of the features' state.
///
/// The model never reaches up into a feature's slot for these. The caller
/// holding the whole application state reads them and hands them down with
/// the operation, read when the operation starts, which is when the model
/// read them before.
#[derive(Debug, Clone, Copy)]
pub struct SlotFacts {
    /// Whether the run gate's report is retained: the dispatch findings are
    /// about that report, and read stale once it is dropped.
    pub preflight_report_retained: bool,
    /// Which review markers the drawing shows, part of the view a finding's
    /// place is read through.
    pub review_markers:
        rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility,
}
