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
use app_serialization::{PanelSizesSer, PanelVisibilitySer, ViewerWorkspaceSer};

mod app_console;
pub use app_console::{ConsoleLevel, ConsoleMessage};

mod app_interaction_state;
pub use app_interaction_state::{DragType, InteractionState};

mod app_veriloga_library;
use app_veriloga_library::{
    VERILOGA_LIBRARY_NAME, restore_global_veriloga_library, save_global_veriloga_library,
};

mod app_property_edit;
use app_property_edit::apply_component_property_edits;

mod app_modal_workflows;

mod app_shortcuts;

mod app_actions;

mod app_file_actions;

mod app_icon_rail;

mod app_simulation_analysis_options;
mod app_viewer_capabilities;
mod app_viewer_panels;
pub use app_viewer_capabilities::ViewerCapability;

mod app_simulation_dialogs;

mod app_library_dialogs;

mod app_help_dialogs;

mod app_confirmation_dialog;

mod app_workspace_layout;

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
    /// Panel visibility
    pub(crate) panels: PanelVisibility,
    /// Panel sizes
    pub(crate) panel_sizes: PanelSizes,
    /// Dialog visibility
    pub(crate) dialogs: DialogState,
    /// Current theme
    pub(crate) theme: RSpiceTheme,
    /// Console messages
    pub(crate) console_messages: Vec<ConsoleMessage>,
    /// Structured log history buffer (ring-buffer, filterable).
    pub(crate) log_buffer: crate::panels::LogBuffer,
    /// UI state for the structured log panel.
    pub(crate) log_panel_state: crate::panels::LogPanelState,
    /// Component property editor state
    pub(crate) property_editor: crate::properties::dialog::PropertyEditorState,
    /// Scripting/Automation console state
    pub(crate) script_console: crate::panels::ScriptConsoleState,
    /// Open specialized viewer workspace tabs.
    pub(crate) viewer_workspace: crate::viewers::ViewerWorkspace,
    /// Waveform viewer state (persists across frames for pan/zoom)
    pub(crate) waveform_viewer: WaveformViewerState,
    /// Library/Cell/View manager for design hierarchy
    pub(crate) library_manager: crate::state::LibraryManager,
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
}

/// Errors returned when applying a waveform-view range from external callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformViewRangeError {
    /// The provided bounds contain `NaN` or infinity.
    NonFiniteBounds,
    /// The provided bounds do not describe a positive range.
    NonPositiveRange,
}

impl std::fmt::Display for WaveformViewRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteBounds => f.write_str("waveform view bounds must be finite"),
            Self::NonPositiveRange => {
                f.write_str("waveform view bounds must define a positive range")
            }
        }
    }
}

impl std::error::Error for WaveformViewRangeError {}

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

    /// Replace the waveform results that drive the shared waveform viewer.
    pub fn replace_waveform_results(&mut self, waveforms: Vec<crate::state::WaveformData>) {
        self.simulation.replace_waveforms(waveforms);
    }

    /// Set the visible X-axis window for the waveform viewer.
    pub fn set_waveform_view_x_range(
        &mut self,
        x_min: f64,
        x_max: f64,
    ) -> Result<(), WaveformViewRangeError> {
        if !x_min.is_finite() || !x_max.is_finite() {
            return Err(WaveformViewRangeError::NonFiniteBounds);
        }
        if x_max <= x_min {
            return Err(WaveformViewRangeError::NonPositiveRange);
        }

        self.waveform_viewer.view.x_min = x_min;
        self.waveform_viewer.view.x_max = x_max;
        self.waveform_viewer.view.enforce_minimum_range();

        let bounds = self.waveform_viewer.data_bounds.clone();
        if bounds.valid {
            self.waveform_viewer.view.clamp_to_bounds(&bounds);
        }

        Ok(())
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
    /// SVG symbol library for component rendering
    pub(crate) symbol_library: Option<crate::schematic::symbols::SymbolLibrary>,
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
            symbol_library,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    fn prepare_frame(&mut self, ctx: &Context) {
        if self.first_frame {
            self.state.theme.apply_to_egui(ctx);
            self.first_frame = false;
        }

        self.handle_shortcuts(ctx);
        self.simulation_controller.update(&mut self.state);
    }

    fn render_frame_chrome(&mut self, ctx: &Context) {
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

        TopBottomPanel::top("toolbar")
            .frame(
                Frame::none()
                    .fill(egui::Color32::from_rgb(35, 38, 48))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
            )
            .show(ctx, |ui| {
                crate::schematic::toolbar::render_toolbar(ui, &mut self.state);
            });

        self.render_workspace_layout(ctx);
    }

    fn render_frame_dialogs(&mut self, ctx: &Context) {
        self.render_confirmation_dialog(ctx);
        self.process_component_properties_dialog(ctx);
        self.process_veriloga_load_dialog(ctx);
        crate::panels::render_property_dialog(ctx, &mut self.state);
        self.process_pdk_settings_dialog(ctx);
        self.render_simulation_setup_dialog(ctx);
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
}

// =============================================================================
// Tests
// =============================================================================
