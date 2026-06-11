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


mod active_viewer;
pub use active_viewer::ActiveViewer;

mod app_shell_state;
pub use app_shell_state::{ConfirmationAction, ConfirmationDialogState, ConfirmationResponse};

mod app_dialog_state;
pub use app_dialog_state::{DialogState, LibraryDeleteTarget};

mod app_serialization;

mod app_console;
pub use app_console::{ConsoleLevel, ConsoleMessage};

mod app_interaction_state;
pub use app_interaction_state::{DragType, InteractionState};

mod app_veriloga_library;
use app_veriloga_library::{
    VERILOGA_LIBRARY_NAME, restore_global_veriloga_library, save_global_veriloga_library,
};

mod app_property_edit;
pub(crate) use app_property_edit::open_property_editor;

mod app_modal_workflows;

mod app_shortcuts;

mod app_actions;

mod app_file_actions;

mod app_viewer_capabilities;
pub use app_viewer_capabilities::ViewerCapability;

mod sim_setup_state;
pub use sim_setup_state::{AcSetup, DcSetup, NoiseSetup, SimSetupState, TranSetup};

mod app_simulation_dialogs;

mod app_library_dialogs;

mod app_help_dialogs;

mod app_confirmation_dialog;

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

/// Main application state container
#[derive(Clone)]
pub struct AppState {
    /// Circuit schematic state (components, wires, topology)
    pub(crate) schematic: SchematicState,
    /// Simulation results and waveforms
    pub(crate) simulation: SimulationState,
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
        self.simulation_controller.update(&mut self.state);
        self.state
            .workspace
            .set_active_dirty(self.state.schematic.is_dirty);
        self.sync_window_title(ctx);
    }

    /// Keep the OS window title (or browser tab title) in sync with the
    /// active document: `cell* — project — RSpice`.
    fn sync_window_title(&mut self, ctx: &Context) {
        let dirty = if self.state.schematic.is_dirty { "*" } else { "" };
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
        self.render_about_dialog(ctx);
        self.render_waveform_calculator_dialog(ctx);
        self.render_shortcuts_help_dialog(ctx);
        self.process_model_browser_dialog(ctx);
        self.process_new_cell_dialog(ctx);
        self.process_new_view_dialog(ctx);
        self.process_pending_library_deletions();
        self.process_exit_request(ctx);
    }
}

impl eframe::App for RSpiceApp {
    /// Called on each frame
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.prepare_frame(ctx);
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

// =============================================================================
// Tests
// =============================================================================
