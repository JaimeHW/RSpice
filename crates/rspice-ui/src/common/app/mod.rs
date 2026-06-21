//! RSpice Application Core
//!
//! The main eframe/egui application. Each frame:
//!
//! 1. [`RSpiceApp::prepare_frame`] applies the active theme (on change),
//!    resolves keyboard shortcuts, and pumps the simulation controller.
//! 2. [`crate::shell::show`] renders the IDE chrome — menu bar, toolbar,
//!    workspace tabs, contextual side panels, the active center view,
//!    console, status bar, and toasts (see `crate::shell` for the layout).
//! 3. [`RSpiceApp::render_frame_dialogs`] renders modal dialogs.
//!
//! # State Management
//!
//! Application state is managed in a centralized `AppState` struct:
//! - SchematicState: circuit topology, components, wires
//! - SimulationState: simulation results, waveforms, run history
//! - ShellState: workspace view, theme selection, console, toasts
//!
//! This follows the commercial EDA pattern where state is:
//! 1. Centralized for consistency
//! 2. Observable for efficient updates
//! 3. Serializable for session recovery

use egui::Context;

use crate::state::{SchematicState, SimulationState};

#[cfg(target_arch = "wasm32")]
const BROWSER_UNLOAD_WARNING: &str = "RSpice has unsaved changes.";

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_UNLOAD_DIRTY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static BROWSER_UNLOAD_LISTENER: std::cell::RefCell<
        Option<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::BeforeUnloadEvent)>>
    > = const { std::cell::RefCell::new(None) };
}

mod active_viewer;
pub use active_viewer::ActiveViewer;

mod app_shell_state;
pub use app_shell_state::{ConfirmationAction, ConfirmationDialogState, ConfirmationResponse};

mod app_dialog_state;
pub use app_dialog_state::{DialogState, LibraryDeleteTarget, LicenseDialogState, LicensePhase};

mod app_serialization;

mod app_console;
pub use app_console::{ConsoleLevel, ConsoleMessage};

mod app_interaction_state;
pub use app_interaction_state::{ContextTarget, DragType, InteractionState};

mod app_veriloga_library;
use app_veriloga_library::{
    VERILOGA_LIBRARY_NAME, restore_global_veriloga_library, save_global_veriloga_library,
};

mod app_property_edit;
pub(crate) use app_property_edit::open_property_editor;

mod app_modal_workflows;

mod app_shortcuts;

mod app_actions;

mod app_autosave;
mod app_file_actions;

mod app_viewer_capabilities;
pub use app_viewer_capabilities::ViewerCapability;

mod sim_setup_state;
pub use sim_setup_state::{AcSetup, DcSetup, NoiseSetup, SimSetupState, TranSetup};

mod app_simulation_dialogs;

mod app_library_dialogs;

mod app_help_dialogs;

mod app_confirmation_dialog;

mod app_preferences_dialog;

mod app_license_dialog;

mod app_command_palette;

mod app_export_image;

mod app_workspace_actions;

mod app_veriloga_workflow;

mod app_pdk_workflow;

mod app_state_init;

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
    /// Dialog visibility
    pub(crate) dialogs: DialogState,
    /// Typed analysis configuration behind the Simulate view.
    pub(crate) sim_setup: SimSetupState,
    /// Structured log history buffer (ring-buffer, filterable).
    pub(crate) log_buffer: crate::panels::LogBuffer,
    /// Scripting/Automation console state
    pub(crate) script_console: crate::panels::ScriptConsoleState,
    /// Library/Cell/View manager for design hierarchy
    pub(crate) library_manager: crate::state::LibraryManager,
    /// Project/workspace model for active design context and open LCV views.
    pub(crate) workspace: crate::state::ProjectWorkspace,
    /// Pending cell deletion (library, cell_name)
    pub(crate) pending_delete_cell: Option<(String, String)>,
    /// Pending view deletion (library, cell, view_name)
    pub(crate) pending_delete_view: Option<(String, String, String)>,
    /// Tabbed property dialog state (commercial-grade property editing)
    pub(crate) tabbed_property_dialog: crate::properties::TabbedPropertyDialogState,
    /// Property registry (component property schemas)
    pub(crate) property_registry: crate::state::PropertyRegistry,
    /// Calculator panel state
    pub(crate) calculator_panel: crate::panels::calculator::CalculatorPanel,
    /// PDK Settings dialog state
    pub(crate) pdk_settings_dialog: crate::panels::PdkSettingsDialogState,
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
    /// The activated license key as pasted (persisted; re-verified on load).
    pub(crate) license_key: Option<String>,
    /// The verified grant behind `license_key` (derived, never persisted).
    pub(crate) license: Option<crate::services::license::LicenseInfo>,
    /// Specialized analysis viewer state grouped by analysis workspace.
    pub(crate) analysis: AnalysisWorkspaceState,
    /// IDE shell state (workspace view, theme, console, toasts).
    pub(crate) shell: crate::shell::ShellState,
}

impl Default for AppState {
    fn default() -> Self {
        app_state_init::default_app_state()
    }
}

impl AppState {
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
        if self.sim_setup.enabled.is_empty() {
            return Some("Tick at least one analysis in the Simulate view".to_string());
        }
        if self.schematic.components.is_empty() {
            return Some("Add a component before running a schematic simulation".to_string());
        }
        if let Some(result) = self.current_blocking_drc_result() {
            let summary = result.summary();
            return Some(format!(
                "Fix current DRC errors before simulation ({} critical, {} error{})",
                summary.critical,
                summary.errors,
                if summary.errors == 1 { "" } else { "s" }
            ));
        }
        None
    }

    /// User-facing reason the Netlist workspace cannot run the current deck.
    pub fn manual_deck_run_block_reason(&self) -> Option<String> {
        if self.simulation.is_running {
            return Some("A simulation is already running".to_string());
        }
        let source = self
            .workspace
            .netlist_source
            .as_deref()
            .unwrap_or(self.simulation.netlist_content.as_str());
        if source.trim().is_empty() {
            return Some("Enter a netlist before running".to_string());
        }
        None
    }

    /// Request a Netlist workspace run, queuing one re-run if the engine is busy.
    pub(crate) fn request_netlist_manual_deck_run(&mut self) {
        self.simulation.run_intent = crate::state::SimulationRunIntent::ManualDeck;
        if self.simulation.is_running {
            self.shell.netlist.rerun_queued = true;
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
        self.shell.netlist.rerun_queued = false;
        self.simulation.request_run_set();
    }

    fn log_severity_for_console(level: ConsoleLevel) -> crate::panels::LogSeverity {
        match level {
            ConsoleLevel::Info => crate::panels::LogSeverity::Info,
            ConsoleLevel::Warning => crate::panels::LogSeverity::Warning,
            ConsoleLevel::Error => crate::panels::LogSeverity::Error,
        }
    }

    /// Push a legacy console message and mirror it into the structured log.
    pub fn push_console_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::panels::LogSource::System, message);
    }

    /// Push a console message with an explicit structured-log source.
    pub fn push_console_message_with_source(
        &mut self,
        source: crate::panels::LogSource,
        message: ConsoleMessage,
    ) {
        let severity = Self::log_severity_for_console(message.level);
        self.log_buffer.log(severity, source, message.message, None);
    }

    pub fn push_user_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::panels::LogSource::User, message);
    }

    pub fn push_sim_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::panels::LogSource::Simulation, message);
    }

    pub fn clear_primary_log(&mut self) {
        self.log_buffer.clear();
    }

    /// Record a file in the recent-files list (most recent first, deduped,
    /// capped at 8 — the conventional depth for an EDA File menu).
    pub(crate) fn remember_recent_file(&mut self, kind: RecentKind, path: &std::path::Path) {
        const MAX_RECENT_FILES: usize = 8;
        let entry = RecentFile {
            kind,
            path: path.to_path_buf(),
        };
        self.recent_files.retain(|r| r.path != entry.path);
        self.recent_files.insert(0, entry);
        self.recent_files.truncate(MAX_RECENT_FILES);
    }

    /// Where a paste should land, snapped to the schematic grid: under the
    /// cursor when it hovers the canvas, otherwise the center of the visible
    /// canvas (menu-driven paste), otherwise a sane fixed spot.
    pub(crate) fn schematic_paste_anchor(&self) -> crate::state::Point {
        let grid = self.schematic.grid_size.max(1);
        let (x, y) = self
            .shell
            .canvas_hover
            .or(self.shell.canvas_view_center)
            .unwrap_or((20.0, 20.0));
        crate::state::Point::new((x.round() as i32) * grid, (y.round() as i32) * grid)
    }
}

// =============================================================================
// Main Application
// =============================================================================

/// RSpice Application
///
/// The main egui application providing commercial-grade CAD interface.
pub struct RSpiceApp {
    /// Application state
    pub(crate) state: AppState,
    /// First frame flag (for initialization)
    first_frame: bool,
    /// When the autosave timer was armed (first dirty frame) or last fired.
    #[cfg(not(target_arch = "wasm32"))]
    autosave_last: Option<std::time::Instant>,
    /// Theme last applied to the egui context (re-applied when the user
    /// changes the shell theme).
    applied_theme: Option<crate::ui::Theme>,
    /// Window title last pushed to the OS (avoids a viewport command per frame).
    last_window_title: String,
    /// SVG symbol library for component rendering
    pub(crate) symbol_library: Option<crate::schematic::symbols::SymbolLibrary>,
    /// Simulation controller for running analyses
    pub(crate) simulation_controller: crate::simulation::SimulationController,
    /// File workflow IO backend (native in production, injectable in tests).
    pub(crate) file_workflow_io: Box<dyn crate::common::file_workflow::FileWorkflowIo>,
    /// Export workflow IO backend (native in production, injectable in tests).
    pub(crate) export_workflow_io: Box<dyn crate::common::export_workflow::ExportWorkflowIo>,
}

impl RSpiceApp {
    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load persisted state if available
        let mut state: AppState = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppState::default()
        };

        // Apply the design-system theme (canvas painters read the active
        // palette directly).
        state.shell.theme.apply(&cc.egui_ctx);

        // Persisted sessions may still carry the legacy seeded
        // "primitives" library (and tabs pointing into it); migrate any
        // drawn content to the user library and drop the placeholders.
        state.migrate_legacy_primitives();

        // Ctrl+± / Ctrl+0 zoom the *schematic*, not the UI — disable egui's
        // built-in keyboard zoom so the shortcuts don't double-fire.
        cc.egui_ctx.options_mut(|options| {
            options.zoom_with_keyboard = false;
        });
        cc.egui_ctx.set_zoom_factor(1.0);

        // Restore global user Verilog-A library (commercial-style user library).
        restore_global_veriloga_library(&mut state.library_manager);
        state.restore_active_schematic_from_workspace();

        // A license file on disk wins over (or backfills) the session copy.
        #[cfg(not(target_arch = "wasm32"))]
        if state.license.is_none()
            && let Some((key, info)) = crate::services::license::load_stored()
        {
            state.license_key = Some(key);
            state.license = Some(info);
        }

        // Load symbol library
        let symbol_library = match crate::schematic::symbols::SymbolLibrary::load_embedded() {
            Ok(lib) => {
                log::info!(
                    "Loaded {} default SVG component mappings from {} embedded SVG assets",
                    lib.len(),
                    lib.asset_count()
                );
                Some(lib)
            }
            Err(e) => {
                log::warn!("Failed to load SVG symbols, using procedural: {}", e);
                None
            }
        };

        // Log startup
        log::info!("RSpice egui application initialized");

        Self {
            state,
            first_frame: true,
            #[cfg(not(target_arch = "wasm32"))]
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    fn prepare_frame(&mut self, ctx: &Context) {
        // (Re)apply the theme when it changes — this maps the design tokens
        // onto the egui style and republishes the active palette.
        if self.first_frame || self.applied_theme != Some(self.state.shell.theme) {
            self.state.shell.theme.apply(ctx);
            self.applied_theme = Some(self.state.shell.theme);
            self.first_frame = false;
        }

        self.handle_shortcuts(ctx);
        #[cfg(target_arch = "wasm32")]
        crate::common::browser_file_import::register_text_import_repaint_context(ctx);
        #[cfg(target_arch = "wasm32")]
        if crate::common::project_workflow::poll_browser_project_import(&mut self.state) {
            self.restore_workspace_after_project_load();
        }
        #[cfg(target_arch = "wasm32")]
        if crate::common::file_workflow::poll_browser_schematic_import(&mut self.state) {
            self.state.clear_transient_specialized_viewer_data();
        }
        #[cfg(target_arch = "wasm32")]
        crate::common::netlist_workflow::poll_browser_netlist_import(&mut self.state);
        self.simulation_controller
            .update(&mut self.state, self.export_workflow_io.as_ref());
        if self.state.simulation.is_running {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
        if matches!(
            self.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ) {
            self.state
                .workspace
                .set_active_dirty(self.state.schematic.is_dirty);
        }
        self.sync_window_title(ctx);
        self.handle_image_export(ctx);
    }

    /// Keep the OS window title (or browser tab title) in sync with the
    /// active document: `cell* — project — RSpice`.
    fn sync_window_title(&mut self, ctx: &Context) {
        let has_unsaved_changes = should_warn_before_browser_unload(
            self.state.schematic.is_dirty,
            self.state.workspace.any_dirty(),
        );
        #[cfg(target_arch = "wasm32")]
        update_browser_before_unload_guard(has_unsaved_changes);

        let dirty = if has_unsaved_changes { "*" } else { "" };
        let view = &self.state.workspace.active_view;
        let title = format!(
            "{}{dirty} — {} — RSpice",
            view.cell,
            self.state.workspace.project.display_name()
        );
        if self.last_window_title != title {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            #[cfg(target_arch = "wasm32")]
            {
                let _ = ctx;
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    doc.set_title(&title);
                }
            }
            self.last_window_title = title;
        }
    }

    fn render_frame_chrome(&mut self, ctx: &Context) {
        crate::shell::show(ctx, self);
    }

    fn render_frame_dialogs(&mut self, ctx: &Context) {
        self.render_confirmation_dialog(ctx);
        self.process_veriloga_load_dialog(ctx);
        crate::panels::render_property_dialog(ctx, &mut self.state);
        self.process_pdk_settings_dialog(ctx);
        self.render_simulation_options_dialog(ctx);
        self.render_preferences_dialog(ctx);
        self.render_license_dialog(ctx);
        self.render_command_palette(ctx);
        self.render_about_dialog(ctx);
        self.render_waveform_calculator_dialog(ctx);
        self.render_shortcuts_help_dialog(ctx);
        self.process_model_browser_dialog(ctx);
        self.process_new_cell_dialog(ctx);
        self.process_new_view_dialog(ctx);
        self.process_copy_cell_dialog(ctx);
        self.process_rename_cell_dialog(ctx);
        self.process_autosave_restore_dialog(ctx);
        self.process_pending_library_deletions();
        self.process_exit_request(ctx);
    }
}

impl eframe::App for RSpiceApp {
    /// Called on each frame
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.prepare_frame(ctx);
        #[cfg(not(target_arch = "wasm32"))]
        self.autosave_tick(ctx);
        self.render_frame_chrome(ctx);
        self.render_frame_dialogs(ctx);
    }

    /// Called by eframe when the application is shutting down.
    fn on_exit(&mut self) {
        log::info!("eframe on_exit — application shutting down");
    }

    /// Save state on exit
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        log::debug!("save: sync schematic");
        self.state.sync_active_schematic_to_workspace();
        log::debug!("save: veriloga library");
        if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
            log::warn!(
                "Failed to persist global Verilog-A library during app save: {}",
                err
            );
        }
        log::debug!("save: serialize app state");
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
        log::debug!("save: done");
    }
}

impl RSpiceApp {
    fn process_exit_request(&mut self, ctx: &Context) {
        if !self.state.exit_requested {
            return;
        }

        log::info!("application exit requested — closing viewport");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        self.state.exit_requested = false;
    }

    /// Toggle the console panel between expanded and collapsed.
    pub fn toggle_console(&mut self) {
        self.state.shell.console.collapsed = !self.state.shell.console.collapsed;
    }
}

fn should_warn_before_browser_unload(schematic_dirty: bool, workspace_dirty: bool) -> bool {
    schematic_dirty || workspace_dirty
}

#[cfg(target_arch = "wasm32")]
fn update_browser_before_unload_guard(has_unsaved_changes: bool) {
    use wasm_bindgen::JsCast as _;

    BROWSER_UNLOAD_DIRTY.with(|dirty| dirty.set(has_unsaved_changes));
    BROWSER_UNLOAD_LISTENER.with(|listener| {
        if listener.borrow().is_some() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let callback = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::BeforeUnloadEvent)>::new(
            |event: web_sys::BeforeUnloadEvent| {
                let dirty = BROWSER_UNLOAD_DIRTY.with(|state| state.get());
                if dirty {
                    event.prevent_default();
                    event.set_return_value(BROWSER_UNLOAD_WARNING);
                }
            },
        );
        if window
            .add_event_listener_with_callback("beforeunload", callback.as_ref().unchecked_ref())
            .is_ok()
        {
            *listener.borrow_mut() = Some(callback);
        }
    });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::drc::{
        DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType,
    };
    use crate::state::{ComponentType, Point};

    fn runnable_state() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(
            !state.sim_setup.enabled.is_empty(),
            "default run set should contain a runnable analysis"
        );
        state
    }

    fn drc_result(violation_type: DrcViolationType, severity: Option<DrcSeverity>) -> DrcResult {
        let mut result = DrcResult::new();
        let mut violation = DrcViolation::new(
            1,
            violation_type,
            violation_type.description(),
            DrcLocation::Global,
        );
        if let Some(severity) = severity {
            violation = violation.with_severity(severity);
        }
        result.add_violation(violation);
        result.completed = true;
        result
    }

    #[test]
    fn run_readiness_blocks_current_drc_errors_for_generated_schematic_runs() {
        let mut state = runnable_state();
        state.dialogs.drc_results = Some(drc_result(DrcViolationType::MissingGround, None));
        state.dialogs.drc_checked_version = state.schematic.topology_version();

        assert!(
            !state.can_run_simulation(),
            "current error-level DRC results must block schematic simulation"
        );
    }

    #[test]
    fn run_readiness_allows_stale_or_warning_only_drc_results() {
        let mut state = runnable_state();
        state.dialogs.drc_results = Some(drc_result(DrcViolationType::MissingGround, None));
        state.dialogs.drc_checked_version = state.schematic.topology_version().wrapping_sub(1);
        assert!(
            state.can_run_simulation(),
            "stale DRC results should not block after schematic edits"
        );

        state.dialogs.drc_results = Some(drc_result(
            DrcViolationType::UnconnectedPin,
            Some(DrcSeverity::Warning),
        ));
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        assert!(
            state.can_run_simulation(),
            "warning-only DRC results should not block simulation"
        );
    }

    #[test]
    fn generated_and_manual_runs_have_separate_drc_readiness() {
        let mut state = runnable_state();
        state.workspace.netlist_source = Some("V1 in 0 1\nR1 in 0 1k\n.end\n".to_string());
        state.dialogs.drc_results = Some(drc_result(DrcViolationType::MissingGround, None));
        state.dialogs.drc_checked_version = state.schematic.topology_version();

        assert!(
            !state.can_run_simulation(),
            "schematic Simulate must still be blocked by current schematic DRC"
        );
        assert!(
            state.manual_deck_run_block_reason().is_none(),
            "manual deck runs should not be blocked by schematic DRC"
        );
    }

    #[test]
    fn netlist_manual_run_request_queues_when_engine_is_busy() {
        let mut state = AppState::default();
        state.simulation.is_running = true;
        state.request_netlist_manual_deck_run();

        assert_eq!(
            state.simulation.run_intent,
            crate::state::SimulationRunIntent::ManualDeck
        );
        assert!(!state.simulation.trigger_simulation);
        assert!(state.shell.netlist.rerun_queued);

        state.simulation.is_running = false;
        state.shell.netlist.rerun_queued = false;
        state.request_netlist_manual_deck_run();

        assert!(state.simulation.trigger_simulation);
        assert!(!state.shell.netlist.rerun_queued);
    }

    #[test]
    fn browser_unload_warning_tracks_schematic_or_workspace_dirty_state() {
        assert!(!should_warn_before_browser_unload(false, false));
        assert!(should_warn_before_browser_unload(true, false));
        assert!(should_warn_before_browser_unload(false, true));
        assert!(should_warn_before_browser_unload(true, true));
    }
}
