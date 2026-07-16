//! RSpice Application Core
//!
//! The main eframe/egui application. Each frame:
//!
//! 1. [`RSpiceApp::prepare_frame`] applies the active theme (on change),
//!    resolves keyboard shortcuts, and pumps the simulation controller.
//! 2. [`crate::workbench::show`] renders the IDE chrome — menu bar, toolbar,
//!    workspace tabs, contextual side panels, the active center view,
//!    console, status bar, and toasts (see `crate::workbench` for the layout).
//! 3. [`RSpiceApp::render_frame_dialogs`] renders modal dialogs.
//!
//! # State Management
//!
//! Application state is managed in a centralized `AppState` struct:
//! - SchematicState: circuit topology, components, wires
//! - SimulationState: simulation results, waveforms, run history
//! - WorkbenchState: canonical workspace navigation and responsive layout
//! - UiSessionState: theme and document-engine interaction state
//!
//! This follows the commercial EDA pattern where state is:
//! 1. Centralized for consistency
//! 2. Observable for efficient updates
//! 3. Serializable for session recovery

use egui::Context;

use crate::state::{SchematicState, SimulationState};

const CONTEXT_LONG_PRESS_DURATION_SECONDS: f64 = 0.56;
const CONTEXT_LONG_PRESS_MOVE_TOLERANCE_POINTS: f32 = 9.0;

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

mod app_confirmation_state;
pub use app_confirmation_state::{
    ConfirmationAction, ConfirmationDialogState, ConfirmationResponse,
};
pub(crate) use app_confirmation_state::{ProjectReviewDialogState, ProjectReviewRequest};

mod app_dialog_state;
pub use app_dialog_state::{DialogState, LicenseDialogState, LicensePhase};

mod app_preference_runtime;
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
pub(crate) use app_shortcuts::{
    accessibility_shortcut_summary, report_engineering_canvas_focus, runtime_command_platform,
};

mod app_shortcut_library_persistence;

mod app_actions;

mod app_autosave;
mod app_file_actions;

mod app_viewer_capabilities;
pub use app_viewer_capabilities::ViewerCapability;

mod sim_setup_state;
pub use sim_setup_state::{
    AcSetup, DcSetup, NoiseSetup, ReferencePvtPoint, SimSetupState, TranSetup,
};
mod sim_setup_analysis_drafts;

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
pub(crate) use app_state_init::default_model_library_manager;

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
    /// Transactional document registry, accepted project baseline, and
    /// canonical persistence identity. Runtime-only; session recovery keeps
    /// the working set separately and reconstructs this boundary at startup.
    pub(crate) project_lifecycle: crate::common::project_lifecycle::ProjectLifecycleState,
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
    /// Exact native canonical-file authority saved with the working session.
    /// A restored pathname alone never grants ordinary Save authority.
    pub(crate) native_project_binding_receipt:
        Option<crate::common::project_lifecycle::NativeBindingReceipt>,
    /// Exact browser canonical-binding authority saved with the working-set
    /// session.  The binding UUID is opaque and intentionally independent of
    /// the logical project UUID. Runtime file handles are never serialized.
    pub(crate) browser_project_binding_receipt:
        Option<crate::common::project_lifecycle::BrowserBindingReceipt>,
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
    pub(crate) shortcut_resolver: app_shortcuts::ShortcutResolverState,
    /// Canonical device-local shortcut-library CAS authority. The recoverable
    /// eframe session copy never replaces this owner after startup.
    pub(crate) shortcut_library_persistence:
        app_shortcut_library_persistence::ShortcutLibraryPersistenceRuntime,
    /// UI transaction paired with the single browser shortcut-library CAS.
    pub(crate) shortcut_library_publication_continuation:
        Option<app_shortcut_library_persistence::ShortcutLibraryPublicationContinuation>,
}

impl Default for AppState {
    fn default() -> Self {
        app_state_init::default_app_state()
    }
}

impl AppState {
    /// Whether any retained application workflow owns exclusive keyboard and
    /// pointer intent for this frame.
    pub(crate) fn application_modal_open(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        let browser_file_operation_open =
            crate::common::project_workflow::browser_file_operation_label(self).is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let browser_file_operation_open = false;

        self.workbench.application_modal_open()
            || self.dialogs.application_modal_open()
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
        let plan = match self.sim_setup.stable_analysis_plan() {
            Ok(plan) => plan,
            Err(error) => return Some(error),
        };
        let enabled = plan
            .instances()
            .iter()
            .filter(|instance| instance.enabled())
            .collect::<Vec<_>>();
        if enabled.is_empty() {
            return Some("Enable at least one analysis instance in the simulation plan".to_owned());
        }
        if self.schematic.components.is_empty() {
            return Some("Add a component before running a schematic simulation".to_string());
        }
        if let Some(issue) = plan.validation_issues().first() {
            return Some(format!("Correct simulation plan: {issue}"));
        }
        if let Some((instance, error)) = enabled.iter().find_map(|instance| {
            self.sim_setup
                .analysis_draft_validation_error(instance.draft())
                .map(|error| (*instance, error))
        }) {
            return Some(format!(
                "Correct {} instance {}: {error}",
                instance.kind().label(),
                instance.id()
            ));
        }
        if let Err(error) = self
            .model_library_manager
            .reference_process_model_cards(self.sim_setup.reference_pvt.process)
        {
            return Some(error);
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
        let previous = self
            .recent_files
            .iter()
            .find(|recent| recent.path == path)
            .cloned();
        let entry = RecentFile {
            kind,
            path: path.to_path_buf(),
            opened_at_unix_ms: crate::common::time_compat::unix_epoch()
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
    /// changes the workbench theme).
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

fn configure_platform_input_contract(ctx: &Context) {
    ctx.options_mut(|options| {
        // The mockup registers this gesture document-wide: a 560 ms press
        // opens the contextual touch sheet unless movement exceeds 9 px.
        options.input_options.max_click_duration = CONTEXT_LONG_PRESS_DURATION_SECONDS;
        options.input_options.max_click_dist = CONTEXT_LONG_PRESS_MOVE_TOLERANCE_POINTS;

        // Ctrl+± / Ctrl+0 zoom the schematic rather than the application UI.
        options.zoom_with_keyboard = false;
    });
}

impl RSpiceApp {
    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn test_instance() -> Self {
        Self {
            state: AppState::default(),
            first_frame: false,
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load persisted state if available
        let mut state: AppState = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppState::default()
        };

        let console_open = state
            .ui
            .preferences
            .workspace()
            .map(crate::workbench::WorkspacePreferences::console_on_launch)
            .unwrap_or_default()
            .is_open();
        state.workbench.apply_console_launch_behavior(console_open);

        state.initialize_shortcut_library_persistence(&cc.egui_ctx);
        state
            .ui
            .set_number_locale(crate::quantity::platform_number_locale());

        // Apply the design-system theme (canvas painters read the active
        // palette directly).
        state.ui.theme.apply(&cc.egui_ctx);

        // Persisted sessions may still carry the legacy seeded
        // "primitives" library (and tabs pointing into it); migrate any
        // drawn content to the user library and drop the placeholders.
        state.migrate_legacy_primitives();

        configure_platform_input_contract(&cc.egui_ctx);
        cc.egui_ctx.set_zoom_factor(1.0);

        // Restore global user Verilog-A library (commercial-style user library).
        restore_global_veriloga_library(&mut state.library_manager);
        state.restore_active_schematic_from_workspace();
        crate::common::project_lifecycle::initialize_from_session(&mut state);
        #[cfg(target_arch = "wasm32")]
        initialize_browser_surface_navigation(&mut state, &cc.egui_ctx);

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
        let initializing_frame = self.first_frame;
        let desired_zoom = self.state.ui.preferences.interface_scale();
        if (ctx.zoom_factor() - desired_zoom).abs() > f32::EPSILON {
            ctx.set_zoom_factor(desired_zoom);
        }
        // (Re)apply the theme when it changes — this maps the design tokens
        // onto the egui style and republishes the active palette.
        let system_mode_changed = self.state.ui.theme.mode == crate::ui::Mode::System
            && crate::ui::tokens::Tokens::get(ctx).mode != self.state.ui.theme.mode.effective(ctx);
        if self.first_frame
            || self.applied_theme != Some(self.state.ui.theme)
            || system_mode_changed
        {
            self.state.ui.theme.apply(ctx);
            self.applied_theme = Some(self.state.ui.theme);
            self.first_frame = false;
        }
        // The first egui pass establishes font metrics and persistent panel
        // rectangles. Request the settled pass explicitly; otherwise a WASM
        // deep link can remain provisional until the first user input,
        // leaving painter-owned title text absent while icons are visible.
        if initializing_frame {
            ctx.request_repaint();
        }

        #[cfg(target_arch = "wasm32")]
        synchronize_browser_surface_navigation(&mut self.state);
        #[cfg(target_arch = "wasm32")]
        self.state.poll_shortcut_library_persistence();
        self.handle_shortcuts(ctx);
        #[cfg(target_arch = "wasm32")]
        crate::common::browser_file_import::register_text_import_repaint_context(ctx);
        #[cfg(target_arch = "wasm32")]
        crate::common::project_lifecycle::poll_browser_binding_restore(&mut self.state);
        #[cfg(target_arch = "wasm32")]
        if crate::common::project_workflow::poll_browser_project_import(&mut self.state) {
            self.restore_workspace_after_project_load();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(event) =
            crate::common::project_workflow::poll_browser_project_save(&mut self.state)
        {
            self.handle_save_continuation_event(event);
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
            crate::common::project_lifecycle::has_unsaved_changes(&self.state),
            false,
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
        crate::workbench::show(ctx, self);
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
        #[cfg(not(target_arch = "wasm32"))]
        self.process_autosave_restore_dialog(ctx);
        self.process_pending_library_deletions();
        self.process_exit_request(ctx);
        crate::workbench::show_route_overlays(ctx, self);
    }
}

#[cfg(target_arch = "wasm32")]
fn initialize_browser_surface_navigation(state: &mut AppState, ctx: &Context) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser_navigation::{
        current_location, install_popstate_listener, restart_history_session,
    };

    match current_location() {
        Ok(location) => {
            if let Some(route) = location.route()
                && let Err(error) = state
                    .workbench
                    .navigate(route, RouteTransitionSource::BrowserPop)
            {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "The requested deep link was not opened: {error}"
                )));
            }
        }
        Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
            "The browser deep link is malformed and was rejected; the address was recovered to the active task: {error}"
        ))),
    }

    // A restored native-style stack is not browser traversal authority. The
    // new process starts with exactly its active route and takes ownership of
    // the current host entry through one canonical replaceState transaction.
    state
        .workbench
        .reset_navigation_history_for_fresh_browser_session();
    let listener_ready = match install_popstate_listener(ctx) {
        Ok(()) => true,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Browser route synchronization is disabled and listener installation will retry automatically: {error}"
            )));
            false
        }
    };
    if listener_ready && let Err(error) = restart_history_session(state.workbench.current_route()) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and will retry automatically; history could not be initialized at the active task: {error}"
        )));
    }
}

#[cfg(target_arch = "wasm32")]
fn synchronize_browser_surface_navigation(state: &mut AppState) {
    use crate::workbench::browser_navigation::{
        ensure_popstate_listener, history_session_ready, poll_popstate, push_route, replace_route,
        traversal_in_flight, traversal_watchdog_expired, traverse_history,
    };
    use crate::workbench::{BrowserHistoryEffect, RouteTransitionSource};

    while let Some(event) = poll_popstate() {
        match event {
            Ok(event) => {
                let route = event.route();
                let availability = crate::workbench::route_availability(route);
                let pending_effects = state.workbench.has_pending_browser_history_effects();
                let external_race = !event.initiated_by_app() && pending_effects;
                let applied = if !availability.can_open() {
                    Err(format!(
                        "surface `{}` is unavailable: {}",
                        route.surface_id(),
                        availability
                            .reason()
                            .unwrap_or("no complete route executor is registered")
                    ))
                } else if external_race {
                    Err(
                        "an external browser traversal raced with newer in-app navigation"
                            .to_owned(),
                    )
                } else if event.initiated_by_app() {
                    (state.workbench.current_route() == route || pending_effects)
                        .then_some(())
                        .ok_or_else(|| {
                            "the in-app traversal did not reach its authenticated browser entry"
                                .to_owned()
                        })
                } else {
                    apply_authenticated_browser_pop(state, event.delta(), route)
                };

                if let Err(error) = applied {
                    state.push_user_message(ConsoleMessage::warning(format!(
                        "Browser history could not restore the requested task and was recovered at a canonical route: {error}"
                    )));
                    if availability.can_open() && !external_race {
                        // The browser entry is authenticated and available;
                        // make it authoritative, then start a bounded session
                        // rather than guessing from stale in-app stacks.
                        if state.workbench.current_route() != route {
                            let _ = state
                                .workbench
                                .navigate(route, RouteTransitionSource::BrowserPop);
                        }
                    }
                    recover_browser_history_at_active_task(state);
                    return;
                }
            }
            Err(error) => {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "An unauthenticated or malformed browser history entry was rejected and canonicalized to the active task: {error}"
                )));
                recover_browser_history_at_active_task(state);
                return;
            }
        }
    }

    if state
        .workbench
        .take_browser_history_effect_queue_overflowed()
    {
        state.push_user_message(ConsoleMessage::warning(
            "Browser synchronization commands exceeded the bounded queue; the address was recovered directly to the active task.",
        ));
        recover_browser_history_at_active_task(state);
        return;
    }

    if traversal_watchdog_expired() {
        state.push_user_message(ConsoleMessage::warning(
            "Browser traversal did not produce an authenticated history event before the watchdog deadline; the address was recovered to the active task.",
        ));
        recover_browser_history_at_active_task(state);
        return;
    }

    if !history_session_ready() {
        state.workbench.clear_browser_history_effects();
        if ensure_popstate_listener().is_err() {
            return;
        }
        let canonical = state.workbench.current_route();
        match crate::workbench::browser_navigation::restart_history_session(canonical) {
            Ok(()) => {
                state
                    .workbench
                    .reset_navigation_history_for_fresh_browser_session();
                state.push_user_message(ConsoleMessage::info(
                    "Browser route synchronization resumed at the active task; browser traversal history was restarted.",
                ));
            }
            Err(_) => return,
        }
    }

    // `history.go` is asynchronous. No later effect may observe or mutate the
    // browser ledger until its exact destination is authenticated by popstate.
    if traversal_in_flight() {
        return;
    }

    while let Some(effect) = state.workbench.take_browser_history_effect() {
        let (result, traversal_started) = match effect {
            BrowserHistoryEffect::Push(route) => (push_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Replace(route) => (replace_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Traverse { delta, destination } => {
                (traverse_history(delta, destination), true)
            }
        };
        if let Err(error) = result {
            rollback_browser_navigation_after_sync_failure(state, error);
            return;
        }
        if traversal_started {
            return;
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn recover_browser_history_at_active_task(state: &mut AppState) {
    use crate::workbench::browser_navigation::{ensure_popstate_listener, restart_history_session};

    state.workbench.clear_browser_history_effects();
    let canonical = state.workbench.current_route();
    if let Err(error) = ensure_popstate_listener() {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and listener installation will retry automatically: {error}"
        )));
        return;
    }
    match restart_history_session(canonical) {
        Ok(()) => state
            .workbench
            .reset_navigation_history_for_fresh_browser_session(),
        Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and will retry automatically; the address could not be recovered to the active task: {error}"
        ))),
    }
}

#[cfg(target_arch = "wasm32")]
fn rollback_browser_navigation_after_sync_failure(
    state: &mut AppState,
    error: crate::workbench::browser_navigation::BrowserNavigationError,
) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser_navigation::{active_browser_route, restart_history_session};

    match active_browser_route() {
        Ok(browser_route) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The browser address could not be synchronized; the active task was rolled back to the last committed browser route: {error}"
            )));
            if state.workbench.current_route() != browser_route {
                let _ = state
                    .workbench
                    .navigate(browser_route, RouteTransitionSource::BrowserPop);
            }
            state.workbench.clear_browser_history_effects();
            state
                .workbench
                .reset_navigation_history_for_fresh_browser_session();
            if let Err(recovery_error) = restart_history_session(browser_route) {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "Browser route synchronization is disabled and will retry automatically; the history session could not be restarted after rollback: {recovery_error}"
                )));
            }
        }
        Err(_) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The browser address could not be synchronized and no committed browser session was available for rollback: {error}"
            )));
            recover_browser_history_at_active_task(state);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_authenticated_browser_pop(
    state: &mut AppState,
    delta: i32,
    route: crate::workbench::SurfaceRoute,
) -> Result<(), String> {
    use crate::workbench::RouteTransitionSource;

    if delta < 0 {
        let steps = usize::try_from(delta.unsigned_abs())
            .map_err(|_| "browser back distance is not representable".to_owned())?;
        if steps > state.workbench.back_route_count() {
            return Err("browser back entry is outside the in-app task stack".to_owned());
        }
        state
            .workbench
            .navigate_back_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(|| "browser back entry is outside the in-app task stack".to_owned())?;
    } else if delta > 0 {
        let steps = usize::try_from(delta)
            .map_err(|_| "browser forward distance is not representable".to_owned())?;
        if steps > state.workbench.forward_route_count() {
            return Err("browser forward entry is outside the in-app task stack".to_owned());
        }
        state
            .workbench
            .navigate_forward_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(|| "browser forward entry is outside the in-app task stack".to_owned())?;
    }

    if state.workbench.current_route() == route {
        Ok(())
    } else {
        Err("authenticated browser entry and in-app task stack disagree".to_owned())
    }
}

impl eframe::App for RSpiceApp {
    /// Called on each frame
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(enabled) = crate::common::browser_accessibility::spoken_feedback_override()
            {
                self.state.ui.browser_spoken_feedback = enabled;
            }
            ctx.options_mut(|options| {
                options.screen_reader = self.state.ui.browser_spoken_feedback;
            });
        }
        self.prepare_frame(&ctx);
        if let Some(text) = self.state.ui.clipboard_text_request.take() {
            ctx.copy_text(text);
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.autosave_tick(&ctx);
        self.render_frame_chrome(&ctx);
        self.refresh_incremental_connectivity_checks();
        self.render_frame_dialogs(&ctx);
    }

    /// Called by eframe when the application is shutting down.
    fn on_exit(&mut self) {
        log::info!("eframe on_exit — application shutting down");
        #[cfg(target_arch = "wasm32")]
        if let Err(error) = crate::workbench::browser_navigation::uninstall_popstate_listener() {
            log::warn!("Failed to remove browser route listener: {error}");
        }
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

    /// Persist the complete recoverable working session on the same cadence
    /// selected in Preferences. Native builds additionally publish their
    /// path-bound exact-byte checkpoint; browser/mobile builds use eframe's
    /// durable application storage as their device-local recovery owner.
    fn auto_save_interval(&self) -> std::time::Duration {
        configured_autosave_interval(self.state.ui.autosave_minutes)
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
        self.state.workbench.console_visible = !self.state.workbench.console_visible;
    }
}

fn should_warn_before_browser_unload(schematic_dirty: bool, workspace_dirty: bool) -> bool {
    schematic_dirty || workspace_dirty
}

fn configured_autosave_interval(minutes: u8) -> std::time::Duration {
    // Deserialization normalizes legacy values to 2/5/10. Keep this boundary
    // defensive so a malformed in-memory integration can never request an
    // every-frame persistence loop.
    std::time::Duration::from_secs(u64::from(minutes.max(1)) * 60)
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
            state
                .sim_setup
                .stable_analysis_plan()
                .is_ok_and(|plan| plan.instances().iter().any(|instance| instance.enabled())),
            "default plan should contain a runnable analysis instance"
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
    fn platform_input_contract_matches_context_gesture_mockup() {
        let ctx = Context::default();

        configure_platform_input_contract(&ctx);

        ctx.options(|options| {
            assert_eq!(options.input_options.max_click_duration, 0.56);
            assert_eq!(options.input_options.max_click_dist, 9.0);
            assert!(!options.zoom_with_keyboard);
        });
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
        assert!(state.ui.netlist.rerun_queued);

        state.simulation.is_running = false;
        state.ui.netlist.rerun_queued = false;
        state.request_netlist_manual_deck_run();

        assert!(state.simulation.trigger_simulation);
        assert!(!state.ui.netlist.rerun_queued);
    }

    #[test]
    fn browser_unload_warning_tracks_schematic_or_workspace_dirty_state() {
        assert!(!should_warn_before_browser_unload(false, false));
        assert!(should_warn_before_browser_unload(true, false));
        assert!(should_warn_before_browser_unload(false, true));
        assert!(should_warn_before_browser_unload(true, true));
    }

    #[test]
    fn preference_autosave_cadence_drives_platform_session_persistence() {
        assert_eq!(
            configured_autosave_interval(2),
            std::time::Duration::from_secs(120)
        );
        assert_eq!(
            configured_autosave_interval(5),
            std::time::Duration::from_secs(300)
        );
        assert_eq!(
            configured_autosave_interval(10),
            std::time::Duration::from_secs(600)
        );
        assert_eq!(
            configured_autosave_interval(0),
            std::time::Duration::from_secs(60)
        );
    }

    #[test]
    fn application_modal_gate_covers_every_app_level_modal_owner() {
        let mut state = AppState::default();
        assert!(!state.application_modal_open());

        state.dialogs.preferences_open = true;
        assert!(state.application_modal_open());
        state.dialogs.preferences_open = false;

        state.sim_setup.options_open = true;
        assert!(state.application_modal_open());
        state.sim_setup.options_open = false;

        state.sim_setup.palette_open = true;
        assert!(state.application_modal_open());
        state.sim_setup.palette_open = false;

        state.tabbed_property_dialog.open = true;
        assert!(state.application_modal_open());
        state.tabbed_property_dialog.open = false;

        state.pdk_settings_dialog.open = true;
        assert!(state.application_modal_open());
        state.pdk_settings_dialog.open = false;

        state.model_browser_state.open = true;
        assert!(state.application_modal_open());
        state.model_browser_state.open = false;

        state.workbench.open_project_launcher();
        assert!(state.application_modal_open());
    }

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
