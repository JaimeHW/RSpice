//! RSpice Application Core
//!
//! The main eframe/egui application that provides commercial-grade
//! GPU-accelerated rendering for schematic capture and waveform viewing.
//!
//! # Layout Architecture
//!
//! The layout mirrors the Dioxus version for consistency:
//!
//! ```text
//! +------------------------------------------------------------------+
//! | Menu Bar (File, Edit, View, Simulate, Tools, Help)              |
//! +------------------------------------------------------------------+
//! | Toolbar (simulation controls, zoom, etc.)                        |
//! +------------------------------------------------------------------+
//! | Icon Rail | Project Browser | Schematic Editor | Properties      |
//! |           |                 | + Waveform Viewer  |               |
//! |           |                 | + Log Panel        |               |
//! +------------------------------------------------------------------+
//! ```
//!
//! # State Management
//!
//! Application state is managed in a centralized `AppState` struct:
//! - SchematicState: circuit topology, components, wires
//! - SimulationState: simulation results, waveforms
//! - ViewState: pan, zoom, selection, tool mode
//!
//! This follows the commercial EDA pattern where state is:
//! 1. Centralized for consistency
//! 2. Observable for efficient updates
//! 3. Serializable for session recovery

use egui::{Context, Frame, TopBottomPanel};

use crate::state::{SchematicState, SimulationState};
use crate::waveform::WaveformViewerState;

use super::theme::RSpiceTheme;

mod app_shell_state;
pub use app_shell_state::{
    BottomPanelTab, ConfirmationAction, ConfirmationDialogState, ConfirmationResponse, PanelSizes,
    PanelVisibility,
};

mod app_dialog_state;
pub use app_dialog_state::DialogState;

mod app_serialization;
#[cfg(test)]
use app_serialization::{PanelSizesSer, PanelVisibilitySer};

mod app_console;
pub use app_console::{ConsoleLevel, ConsoleMessage};

mod app_veriloga_library;
use app_veriloga_library::{
    restore_global_veriloga_library, save_global_veriloga_library, VERILOGA_LIBRARY_NAME,
};

mod app_property_edit;
use app_property_edit::apply_component_property_edits;

mod app_modal_workflows;

mod app_shortcuts;

mod app_actions;

mod app_file_actions;

mod app_icon_rail;

mod app_simulation_analysis_options;
mod app_viewer_panels;

mod app_simulation_dialogs;

mod app_library_dialogs;

mod app_help_dialogs;

mod app_confirmation_dialog;

mod app_workspace_layout;

mod app_veriloga_workflow;

mod app_pdk_workflow;

mod app_state_init;

/// Main application state container
#[derive(Clone)]
pub struct AppState {
    /// Circuit schematic state (components, wires, topology)
    pub schematic: SchematicState,
    /// Simulation results and waveforms
    pub simulation: SimulationState,
    /// Panel visibility
    pub panels: PanelVisibility,
    /// Panel sizes
    pub panel_sizes: PanelSizes,
    /// Dialog visibility
    pub dialogs: DialogState,
    /// Current theme
    pub theme: RSpiceTheme,
    /// Console messages
    pub console_messages: Vec<ConsoleMessage>,
    /// Structured log history buffer (ring-buffer, filterable).
    pub log_buffer: crate::panels::LogBuffer,
    /// UI state for the structured log panel.
    pub log_panel_state: crate::panels::LogPanelState,
    /// Component property editor state
    pub property_editor: crate::properties::dialog::PropertyEditorState,
    /// Scripting/Automation console state
    pub script_console: crate::panels::ScriptConsoleState,
    /// Active specialized viewer state
    pub active_viewer: crate::viewers::ActiveViewer,
    /// Waveform viewer state (persists across frames for pan/zoom)
    pub waveform_viewer: WaveformViewerState,
    /// Library/Cell/View manager for design hierarchy
    pub library_manager: crate::state::LibraryManager,
    /// Pending cell deletion (library, cell_name)
    pub pending_delete_cell: Option<(String, String)>,
    /// Pending view deletion (library, cell, view_name)
    pub pending_delete_view: Option<(String, String, String)>,
    /// Tabbed property dialog state (commercial-grade property editing)
    pub tabbed_property_dialog: crate::properties::TabbedPropertyDialogState,
    /// Property registry (component property schemas)
    pub property_registry: crate::state::PropertyRegistry,
    /// Calculator panel state
    pub calculator_panel: crate::panels::calculator::CalculatorPanel,
    /// Operating point annotation renderer for schematic overlay
    pub op_annotation_renderer: crate::schematic::op_annotation::OpAnnotationRenderer,
    /// PDK Settings dialog state
    pub pdk_settings_dialog: crate::panels::PdkSettingsDialogState,
    /// PDK configuration (library paths, environment variables)
    pub pdk_config: crate::state::pdk_config::PdkConfig,
    /// Model library manager (PDK models, device libraries)
    pub model_library_manager: crate::state::model_library::ModelLibraryManager,
    /// Standalone model browser state (for Tools menu access)
    pub model_browser_state: crate::properties::model_browser::ModelBrowserState,
    /// Flag to signal that application exit has been requested (after confirmation)
    pub exit_requested: bool,
    /// Pole-Zero viewer state
    pub pole_zero_state: crate::analysis::pole_zero::PoleZeroState,
    /// Bode viewer state
    pub bode_plot_state: crate::analysis::bode::BodePlotState,
    /// Nyquist viewer state
    pub nyquist_state: crate::analysis::nyquist::NyquistState,
    /// Eye diagram viewer state
    pub eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState,
    /// FFT viewer state
    pub fft_state: crate::analysis::fft::FftState,
    /// Smith chart viewer state
    pub smith_chart_state: crate::analysis::smith_chart::SmithChartState,
    /// Histogram viewer state
    pub histogram_state: crate::analysis::histogram::HistogramState,
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
        self.log_buffer
            .log(severity, source, message.message.clone(), None);
        self.console_messages.push(message);
    }

    pub fn push_user_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::panels::LogSource::User, message);
    }

    pub fn push_sim_message(&mut self, message: ConsoleMessage) {
        self.push_console_message_with_source(crate::panels::LogSource::Simulation, message);
    }

    pub fn clear_primary_log(&mut self) {
        self.console_messages.clear();
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
    pub state: AppState,
    /// First frame flag (for initialization)
    first_frame: bool,
    /// SVG symbol library for component rendering
    pub symbol_library: Option<crate::schematic::symbols::SymbolLibrary>,
    /// Simulation controller for running analyses
    simulation_controller: crate::simulation::SimulationController,
    /// File workflow IO backend (native in production, injectable in tests).
    file_workflow_io: Box<dyn crate::common::file_workflow::FileWorkflowIo>,
    /// Export workflow IO backend (native in production, injectable in tests).
    export_workflow_io: Box<dyn crate::common::export_workflow::ExportWorkflowIo>,
}

impl RSpiceApp {
    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme to egui context
        let theme = RSpiceTheme::dark();
        theme.apply_to_egui(&cc.egui_ctx);

        // Load persisted state if available
        let mut state = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppState::default()
        };

        // Restore global user Verilog-A library (commercial-style user library).
        restore_global_veriloga_library(&mut state.library_manager);

        // Load symbol library
        let symbol_library = match crate::schematic::symbols::SymbolLibrary::load_embedded() {
            Ok(lib) => {
                log::info!("Loaded {} SVG component symbols", lib.len());
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
            symbol_library,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }
}

impl eframe::App for RSpiceApp {
    /// Called on each frame
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Apply theme on first frame
        if self.first_frame {
            self.state.theme.apply_to_egui(ctx);
            self.first_frame = false;
        }

        // Handle global keyboard shortcuts
        self.handle_shortcuts(ctx);

        // Process simulation state (handles trigger_simulation flag)
        self.simulation_controller.update(&mut self.state);

        // =====================================================================
        // Menu Bar - slightly lighter than panels
        // =====================================================================
        TopBottomPanel::top("menu_bar")
            .frame(
                Frame::none()
                    .fill(egui::Color32::from_rgb(38, 42, 52))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
            )
            .show(ctx, |ui| {
                let (state, file_workflow_io, export_workflow_io) = (
                    &mut self.state,
                    self.file_workflow_io.as_ref(),
                    self.export_workflow_io.as_ref(),
                );
                super::menu_bar::render_menu_bar(ui, state, file_workflow_io, export_workflow_io);
            });

        // =====================================================================
        // Toolbar - distinct mid-tone
        // =====================================================================
        TopBottomPanel::top("toolbar")
            .frame(
                Frame::none()
                    .fill(egui::Color32::from_rgb(35, 38, 48))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
            )
            .show(ctx, |ui| {
                crate::schematic::toolbar::render_toolbar(ui, &mut self.state);
            });

        // Note: Status bar is now rendered as an in-canvas overlay within schematic_view

        self.render_workspace_layout(ctx);

        // =====================================================================
        // Modal Dialogs
        // =====================================================================

        self.render_confirmation_dialog(ctx);

        self.process_component_properties_dialog(ctx);

        self.process_veriloga_load_dialog(ctx);

        // Property Dialog (commercial-grade tabbed property editor)
        crate::panels::render_property_dialog(ctx, &mut self.state);

        self.process_pdk_settings_dialog(ctx);

        self.render_simulation_setup_dialog(ctx);

        // Simulation Options Dialog
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

    /// Save state on exit
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
            log::warn!(
                "Failed to persist global Verilog-A library during app save: {}",
                err
            );
        }
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
}

impl RSpiceApp {
    #[cfg(test)]
    pub(super) fn new_for_tests(state: AppState) -> Self {
        Self {
            state,
            first_frame: false,
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    #[cfg(test)]
    fn set_file_workflow_io_for_test(
        &mut self,
        io: Box<dyn crate::common::file_workflow::FileWorkflowIo>,
    ) {
        self.file_workflow_io = io;
    }

    #[cfg(test)]
    fn set_export_workflow_io_for_test(
        &mut self,
        io: Box<dyn crate::common::export_workflow::ExportWorkflowIo>,
    ) {
        self.export_workflow_io = io;
    }

    fn process_exit_request(&mut self, ctx: &Context) {
        if !self.state.exit_requested {
            return;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        self.state.exit_requested = false;
    }

    /// Toggle the bottom panel visibility
    pub fn toggle_bottom_panel(&mut self) {
        self.state.panels.bottom_panel = !self.state.panels.bottom_panel;
    }

    /// Toggle the log panel visibility.
    pub fn toggle_panel_log_new(&mut self) {
        self.toggle_panel_log();
    }

    /// Toggle the waveform panel visibility
    pub fn toggle_panel_waveform_new(&mut self) {
        // Switch to waveform tab if not active
        if self.state.panels.active_bottom_tab != BottomPanelTab::Waveform {
            self.state.panels.active_bottom_tab = BottomPanelTab::Waveform;
            self.state.panels.bottom_panel = true;
        } else {
            // Toggle visibility if already active
            self.state.panels.bottom_panel = !self.state.panels.bottom_panel;
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
