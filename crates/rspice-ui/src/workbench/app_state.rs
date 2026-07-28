//! The application data.
//!
//! [`AppState`] is everything a session owns and everything a workflow reads:
//! the design, the simulation, the document engines' session state, and the
//! dialogs' visibility. It is deliberately separate from
//! [`RSpiceApp`](crate::workbench::app::RSpiceApp), which owns the frame loop
//! and drives this state — a module that needs to read or mutate the session
//! should depend on the data, not on the application root.
//!
//! Behaviour that queries a specific concern lives with that concern, as an
//! `impl AppState` block in the owning module (netlist run gating in
//! `documents::netlist_document`, modal arbitration in
//! `workflows::project_workflow`). Only accessors over this struct's own
//! fields are defined here.

pub(in crate::workbench) mod active_viewer;
pub(in crate::workbench) mod design_history;
pub(in crate::workbench) mod interaction_state;
mod run_preflight;
pub(in crate::workbench) mod session;
pub(in crate::workbench) mod sim_setup;
pub(in crate::workbench) mod viewer_capabilities;

use crate::diagnostics::{ConsoleLevel, ConsoleMessage};
use crate::state::{SchematicState, SimulationState};
use crate::workbench::app::DialogState;

pub use active_viewer::ActiveViewer;
pub(crate) use design_history::{
    DesignManagementHistoryEntry, SymbolDefinitionFixtureDelta,
    publish_symbol_definition_candidate, publish_symbol_definition_candidate_with_fixture,
};
pub(crate) use interaction_state::SchematicKeyboardFocus;
pub use interaction_state::{ContextTarget, DragType, InteractionState};
pub(crate) use session::shortcuts::{
    accessibility_shortcut_summary, report_engineering_canvas_focus, runtime_command_platform,
};
pub(crate) use session::state_init::default_model_library_manager;
pub use sim_setup::plan_catalog::{
    SimulationPlanCloneOptions,
    SimulationPlanLineage, SimulationPlanName, StoredSimulationPlan,
};
pub use sim_setup::{AcSetup, DcSetup, NoiseSetup, ReferencePvtPoint, SimSetupState, TranSetup};

/// Analysis viewer state grouped behind a dedicated workspace surface.
#[derive(Clone, Default)]
pub struct AnalysisWorkspaceState {
    /// Pole-Zero viewer state
    pub(crate) pole_zero_state: crate::analysis::pole_zero::PoleZeroState,
    /// Bode viewer state
    pub(crate) bode_plot_state: crate::analysis::bode::BodePlotState,
    /// Nyquist viewer state
    pub(crate) nyquist_state: crate::analysis::nyquist::NyquistState,
    /// Eye diagram viewer state
    pub(crate) eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState,
    /// FFT viewer state
    pub(crate) fft_state: crate::analysis::fft::FftState,
    /// Smith chart viewer state
    pub(crate) smith_chart_state: crate::analysis::smith_chart::SmithChartState,
    /// Histogram viewer state
    pub(crate) histogram_state: crate::analysis::histogram::HistogramState,
}

/// What kind of document a recent-files entry points at. `.json` is a valid
/// extension for both schematics and projects, so the kind is recorded at
/// open/save time instead of being inferred from the path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum RecentKind {
    Schematic,
    Project,
}

/// One entry in the File ▸ Open recent list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RecentFile {
    pub kind: RecentKind,
    pub path: std::path::PathBuf,
    /// Milliseconds since the Unix epoch when this entry most recently
    /// became active. Legacy sessions deserialize to zero and retain their
    /// stored list order.
    #[serde(default)]
    pub opened_at_unix_ms: u64,
    /// User pin state for Project Launcher filtering.
    #[serde(default)]
    pub pinned: bool,
    /// Governed project owner. This is never guessed from the operating-
    /// system account when project metadata does not provide it.
    #[serde(default)]
    pub owner: Option<String>,
    /// Searchable project classifications retained by the launcher.
    #[serde(default)]
    pub tags: Vec<String>,
}

impl RecentFile {
    #[cfg(test)]
    pub(crate) fn is_shared_project(&self) -> bool {
        self.kind == RecentKind::Project && path_is_shared(&self.path)
    }
}

#[cfg(test)]
fn path_is_shared(path: &std::path::Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    text.starts_with(r"\\") || text.starts_with("//")
}

/// Main application state container
#[derive(Clone)]
pub struct AppState {
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
    pub(crate) project_design_history: design_history::ProjectDesignHistory,
    /// Dialog visibility
    pub(crate) dialogs: DialogState,
    /// Typed analysis configuration behind the Simulate view.
    pub(crate) sim_setup: SimSetupState,
    /// Structured log history buffer (ring-buffer, filterable).
    pub(crate) log_buffer: crate::diagnostics::LogBuffer,
    /// Scripting/Automation console state
    pub(crate) script_console: session::script_console::ScriptConsoleState,
    /// Library/Cell/View manager for design hierarchy
    pub(crate) library_manager: crate::state::LibraryManager,
    /// Project/workspace model for active design context and open LCV views.
    pub(crate) workspace: crate::state::ProjectWorkspace,
    /// Transactional document registry, accepted project baseline, and
    /// canonical persistence identity. Runtime-only; session recovery keeps
    /// the working set separately and reconstructs this boundary at startup.
    pub(crate) project_lifecycle:
        crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState,
    /// Pending cell deletion (library, cell_name)
    pub(crate) pending_delete_cell: Option<(String, String)>,
    /// Pending view deletion (library, cell, view_name)
    pub(crate) pending_delete_view: Option<(String, String, String)>,
    /// Tabbed property dialog state (commercial-grade property editing)
    pub(crate) tabbed_property_dialog: crate::properties::TabbedPropertyDialogState,
    /// Property registry (component property schemas)
    pub(crate) property_registry: crate::state::PropertyRegistry,
    /// Calculator panel state
    pub(crate) calculator_panel: session::calculator::CalculatorPanel,
    /// PDK Settings dialog state
    pub(crate) pdk_settings_dialog: session::pdk_settings::PdkSettingsDialogState,
    /// PDK configuration (library paths, environment variables)
    pub(crate) pdk_config: crate::state::pdk_config::PdkConfig,
    /// Model library manager (PDK models, device libraries)
    pub(crate) model_library_manager: crate::state::model_library::ModelLibraryManager,
    /// Standalone model browser state (for Tools menu access)
    pub(crate) model_browser_state: crate::properties::model_browser::ModelBrowserState,
    /// Flag to signal that application exit has been requested (after confirmation)
    pub(crate) exit_requested: bool,
    /// Recently opened/saved schematic and project files, most recent first
    /// (persisted across sessions; drives File ▸ Open recent).
    pub(crate) recent_files: Vec<RecentFile>,
    /// Browser-only suggested filename for the next schematic download.
    pub(crate) browser_schematic_save_name: Option<String>,
    /// Browser-only suggested filename for the next project download.
    pub(crate) browser_project_save_name: Option<String>,
    /// Exact native canonical-file authority saved with the working session.
    /// A restored pathname alone never grants ordinary Save authority.
    pub(crate) native_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::NativeBindingReceipt>,
    /// Exact browser canonical-binding authority saved with the working-set
    /// session.  The binding UUID is opaque and intentionally independent of
    /// the logical project UUID. Runtime file handles are never serialized.
    pub(crate) browser_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::BrowserBindingReceipt>,
    /// The activated license key as pasted (persisted; re-verified on load).
    pub(crate) license_key: Option<String>,
    /// The verified grant behind `license_key` (derived, never persisted).
    pub(crate) license: Option<crate::services::license::LicenseInfo>,
    /// Specialized analysis viewer state grouped by analysis workspace.
    pub(crate) analysis: AnalysisWorkspaceState,
    /// Document-engine and interaction session state (theme, canvas, result
    /// viewers, symbol editor, and netlist editor).
    pub(crate) ui: crate::workbench::UiSessionState,
    /// Canonical workbench navigation and responsive layout state.
    pub(crate) workbench: crate::workbench::WorkbenchState,
    /// Runtime-only chord/prefix authority. Partial sequences are never
    /// serialized or restored across application sessions.
    pub(crate) shortcut_resolver: session::shortcuts::ShortcutResolverState,
    /// Canonical device-local shortcut-library CAS authority. The recoverable
    /// eframe session copy never replaces this owner after startup.
    pub(crate) shortcut_library_persistence:
        session::shortcut_library::ShortcutLibraryPersistenceRuntime,
    /// UI transaction paired with the single browser shortcut-library CAS.
    pub(crate) shortcut_library_publication_continuation:
        Option<session::shortcut_library::ShortcutLibraryPublicationContinuation>,
}

impl Default for AppState {
    fn default() -> Self {
        session::state_init::default_app_state()
    }
}

impl AppState {
    /// Whether any retained application workflow owns exclusive keyboard and
    /// pointer intent for this frame.
    pub(crate) fn application_modal_open(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        let browser_file_operation_open =
            crate::workbench::workflows::project_workflow::browser_file_operation_label(self)
                .is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let browser_file_operation_open = false;

        self.workbench.application_modal_open()
            || self.dialogs.application_modal_open()
            || (self.workbench.workspace == crate::workbench::state::Workspace::Netlist
                && self.ui.netlist.application_modal_open())
            || self.sim_setup.options_open
            || self.sim_setup.palette_open
            || self.tabbed_property_dialog.open
            || self.pdk_settings_dialog.open
            || self.model_browser_state.open
            || browser_file_operation_open
    }

    /// Whether a run can start. Every Run affordance (toolbar, run bar, menu,
    /// F5) gates on this so schematic preflight is consistent everywhere.
    pub fn can_run_simulation(&self) -> bool {
        self.simulation_run_block_reason().is_none()
    }

    /// User-facing reason the Run command is currently blocked.
    pub fn simulation_run_block_reason(&self) -> Option<String> {
        if self.simulation.is_running {
            return Some("A simulation is already running".to_string());
        }
        self.simulation_run_preflight_block_reason()
    }

    /// User-facing preflight reason a new run cannot start, excluding the
    /// transient "already running" state so queued re-runs can share it.
    pub fn simulation_run_preflight_block_reason(&self) -> Option<String> {
        run_preflight::run_preflight_block_reason(
            &self.sim_setup,
            &self.workspace,
            &self.schematic,
            &self.model_library_manager,
            self.current_blocking_drc_result(),
        )
    }

    /// User-facing reason the Netlist workspace cannot run the current deck.
    pub fn manual_deck_run_block_reason(&self) -> Option<String> {
        if self.simulation.is_running {
            return Some("A simulation is already running".to_string());
        }
        let active_document = if self.ui.netlist.active_document_initialized {
            self.ui.netlist.active_document
        } else if self.workspace.netlist_source.is_some() {
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        } else {
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
        };
        let source = if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        {
            self.workspace
                .netlist_source
                .as_deref()
                .unwrap_or(self.simulation.netlist_content.as_str())
        } else {
            self.simulation.netlist_content.as_str()
        };
        if source.trim().is_empty() {
            return Some("Enter a netlist before running".to_string());
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
        {
            return Some("Generated comparison documents cannot be executed".to_owned());
        }
        let current_digest =
            crate::workbench::documents::netlist_document::source_content_digest(source);
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
            && (self.ui.netlist.generation_error.is_some()
                || self.ui.netlist.generated_input_digest
                    != self.ui.netlist.current_generation_input_digest)
        {
            return Some(
                self.ui
                    .netlist
                    .generation_error
                    .clone()
                    .unwrap_or_else(|| "Regenerate the stale netlist before running".to_owned()),
            );
        }
        let validated = self.ui.netlist.validation.as_ref().is_some_and(|receipt| {
            receipt.visible_content_digest == current_digest
                && receipt.project_revision == self.workspace.project.revision().get()
        });
        if !validated {
            return Some(
                "Validate the exact current source and project revision before running".to_owned(),
            );
        }
        if active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
            && self.ui.netlist.externally_saved_content_digest != Some(current_digest)
        {
            return Some("Save the validated owned source deck before running".to_owned());
        }
        None
    }

    /// Request a Netlist workspace run, queuing one re-run if the engine is busy.
    pub(crate) fn request_netlist_manual_deck_run(&mut self) {
        self.simulation.run_intent = crate::state::SimulationRunIntent::ManualDeck;
        if self.simulation.is_running {
            self.ui.netlist.rerun_queued = true;
        } else {
            self.simulation.request_manual_deck_run();
        }
    }

    /// Current error-level schematic DRC result that blocks generated runs.
    /// Stale DRC is non-blocking because it no longer describes the current
    /// topology. Manual deck runs use `manual_deck_run_block_reason` instead.
    pub fn current_blocking_drc_result(&self) -> Option<&crate::services::drc::DrcResult> {
        if self.dialogs.drc_checked_version != self.schematic.topology_version() {
            return None;
        }
        self.dialogs
            .drc_results
            .as_ref()
            .filter(|result| result.has_errors())
    }

    /// Request a run from the Simulate workspace run set.
    pub fn request_run_set_simulation(&mut self) {
        self.ui.netlist.rerun_queued = false;
        self.simulation.request_simulate_run_set();
    }

    fn log_severity_for_console(level: ConsoleLevel) -> crate::diagnostics::LogSeverity {
        match level {
            ConsoleLevel::Info => crate::diagnostics::LogSeverity::Info,
            ConsoleLevel::Warning => crate::diagnostics::LogSeverity::Warning,
            ConsoleLevel::Error => crate::diagnostics::LogSeverity::Error,
        }
    }

    /// Push a legacy console message and mirror it into the structured log.
    pub fn push_console_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::System, message);
    }

    /// Push a console message with an explicit structured-log source.
    pub fn push_console_message_with_source(
        &mut self,
        source: crate::diagnostics::LogSource,
        message: ConsoleMessage,
    ) {
        let severity = Self::log_severity_for_console(message.level);
        self.log_buffer.log(severity, source, message.message, None);
    }

    pub fn push_user_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::User, message);
    }

    pub fn push_sim_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::diagnostics::LogSource::Simulation, message);
    }

    pub fn clear_primary_log(&mut self) {
        self.log_buffer.clear();
    }

    /// Record a file in the recent-files list (most recent first, deduped,
    /// capped at 8 — the conventional depth for an EDA File menu).
    pub(crate) fn remember_recent_file(&mut self, kind: RecentKind, path: &std::path::Path) {
        const MAX_RECENT_FILES: usize = 8;
        let previous = self
            .recent_files
            .iter()
            .find(|recent| recent.path == path)
            .cloned();
        let entry = RecentFile {
            kind,
            path: path.to_path_buf(),
            opened_at_unix_ms: crate::time_compat::unix_epoch()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX),
            pinned: previous.as_ref().is_some_and(|recent| recent.pinned),
            owner: previous.as_ref().and_then(|recent| recent.owner.clone()),
            tags: previous.map_or_else(Vec::new, |recent| recent.tags),
        };
        self.recent_files.retain(|recent| recent.path != entry.path);
        self.recent_files.insert(0, entry);
        self.recent_files.truncate(MAX_RECENT_FILES);
    }

    /// Change a project pin without changing its recency or file identity.
    #[cfg(test)]
    pub(crate) fn set_recent_project_pinned(
        &mut self,
        path: &std::path::Path,
        pinned: bool,
    ) -> bool {
        let Some(recent) = self
            .recent_files
            .iter_mut()
            .find(|recent| recent.kind == RecentKind::Project && recent.path == path)
        else {
            return false;
        };
        recent.pinned = pinned;
        true
    }

    /// Where a paste should land, snapped to the schematic grid: under the
    /// cursor when it hovers the canvas, otherwise the center of the visible
    /// canvas (menu-driven paste), otherwise a sane fixed spot.
    pub(crate) fn schematic_paste_anchor(&self) -> crate::state::Point {
        let grid = self.schematic.grid_size.max(1);
        let (x, y) = self
            .ui
            .canvas_hover
            .or(self.ui.canvas_view_center)
            .unwrap_or((20.0, 20.0));
        crate::state::Point::new((x.round() as i32) * grid, (y.round() as i32) * grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_project_refresh_preserves_launcher_metadata() {
        let mut state = AppState::default();
        let path = std::path::Path::new("C:/Engineering/afe.rspiceproj");
        state.remember_recent_file(RecentKind::Project, path);
        state.recent_files[0].pinned = true;
        state.recent_files[0].owner = Some("Analog Design".to_owned());
        state.recent_files[0].tags = vec!["afe".to_owned(), "release".to_owned()];
        let first_timestamp = state.recent_files[0].opened_at_unix_ms;

        state.remember_recent_file(RecentKind::Project, path);

        let recent = &state.recent_files[0];
        assert!(recent.pinned);
        assert_eq!(recent.owner.as_deref(), Some("Analog Design"));
        assert_eq!(recent.tags, ["afe", "release"]);
        assert!(recent.opened_at_unix_ms >= first_timestamp);
    }

    #[test]
    fn launcher_metadata_is_backward_compatible_and_shared_paths_are_detected() {
        let legacy = r#"{"kind":"Project","path":"C:/Engineering/afe.rspiceproj"}"#;
        let recent: RecentFile = serde_json::from_str(legacy).expect("legacy recent entry loads");
        assert_eq!(recent.opened_at_unix_ms, 0);
        assert!(!recent.pinned);
        assert!(recent.owner.is_none());
        assert!(recent.tags.is_empty());
        assert!(!recent.is_shared_project());

        let shared: RecentFile =
            serde_json::from_str(r#"{"kind":"Project","path":"\\\\server\\share\\rf.rspiceproj"}"#)
                .expect("UNC recent entry loads");
        assert!(shared.is_shared_project());
    }

    #[test]
    fn pinning_is_project_only_and_does_not_reorder_recents() {
        let mut state = AppState::default();
        let project = std::path::Path::new("C:/Engineering/afe.rspiceproj");
        let schematic = std::path::Path::new("C:/Engineering/standalone.rsch");
        state.remember_recent_file(RecentKind::Project, project);
        state.remember_recent_file(RecentKind::Schematic, schematic);

        assert!(state.set_recent_project_pinned(project, true));
        assert!(!state.set_recent_project_pinned(schematic, true));
        assert_eq!(state.recent_files[0].path, schematic);
        assert_eq!(state.recent_files[1].path, project);
        assert!(state.recent_files[1].pinned);
    }
}
