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

use std::sync::Arc;

use egui::{Context, Frame, TopBottomPanel};

use crate::state::{SchematicState, SimulationState};
use crate::waveform::WaveformViewerState;

use super::theme::RSpiceTheme;

#[path = "app_shell_state.rs"]
mod app_shell_state;
pub use app_shell_state::{
    BottomPanelTab, ConfirmationAction, ConfirmationDialogState, ConfirmationResponse, PanelSizes,
    PanelVisibility,
};

#[path = "app_dialog_state.rs"]
mod app_dialog_state;
pub use app_dialog_state::DialogState;

#[path = "app_serialization.rs"]
mod app_serialization;
#[cfg(test)]
use app_serialization::{PanelSizesSer, PanelVisibilitySer};

#[path = "app_console.rs"]
mod app_console;
pub use app_console::{ConsoleLevel, ConsoleMessage};

#[path = "app_veriloga_library.rs"]
mod app_veriloga_library;
use app_veriloga_library::{
    restore_global_veriloga_library, save_global_veriloga_library, VERILOGA_LIBRARY_NAME,
};

#[path = "app_property_edit.rs"]
mod app_property_edit;
use app_property_edit::apply_component_property_edits;

#[path = "app_shortcuts.rs"]
mod app_shortcuts;

#[path = "app_actions.rs"]
mod app_actions;

#[path = "app_icon_rail.rs"]
mod app_icon_rail;

#[path = "app_viewer_panels.rs"]
mod app_viewer_panels;

#[path = "app_simulation_dialogs.rs"]
mod app_simulation_dialogs;

#[path = "app_library_dialogs.rs"]
mod app_library_dialogs;

#[path = "app_help_dialogs.rs"]
mod app_help_dialogs;

#[path = "app_confirmation_dialog.rs"]
mod app_confirmation_dialog;

#[path = "app_workspace_layout.rs"]
mod app_workspace_layout;

#[path = "app_veriloga_workflow.rs"]
mod app_veriloga_workflow;

#[path = "app_pdk_workflow.rs"]
mod app_pdk_workflow;

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
        Self {
            schematic: SchematicState::default(),
            simulation: SimulationState::default(),
            panels: PanelVisibility::default(),
            panel_sizes: PanelSizes::default(),
            dialogs: DialogState::default(),
            theme: RSpiceTheme::dark(),
            console_messages: Vec::new(),
            log_buffer: crate::panels::LogBuffer::default(),
            log_panel_state: crate::panels::LogPanelState::default(),
            property_editor: crate::properties::dialog::PropertyEditorState::default(),
            script_console: crate::panels::ScriptConsoleState::default(),
            active_viewer: crate::viewers::ActiveViewer::default(),
            waveform_viewer: WaveformViewerState::default(),
            library_manager: crate::state::LibraryManager::with_primitives(),
            pending_delete_cell: None,
            pending_delete_view: None,
            tabbed_property_dialog: crate::properties::TabbedPropertyDialogState::default(),
            property_registry: crate::state::PropertyRegistry::new(),
            calculator_panel: crate::panels::calculator::CalculatorPanel::new(),
            op_annotation_renderer: crate::schematic::op_annotation::OpAnnotationRenderer::new(),
            pdk_settings_dialog: crate::panels::PdkSettingsDialogState::new(),
            pdk_config: crate::state::pdk_config::PdkConfig::load_or_default(),
            model_library_manager: {
                let mut mgr = crate::state::model_library::ModelLibraryManager::new();
                mgr.load_builtin_models();
                mgr
            },
            model_browser_state: crate::properties::model_browser::ModelBrowserState::default(),
            exit_requested: false,
            pole_zero_state: crate::analysis::pole_zero::PoleZeroState::default(),
            bode_plot_state: crate::analysis::bode::BodePlotState::default(),
            nyquist_state: crate::analysis::nyquist::NyquistState::default(),
            eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState::default(),
            fft_state: crate::analysis::fft::FftState::default(),
            smith_chart_state: crate::analysis::smith_chart::SmithChartState::default(),
            histogram_state: crate::analysis::histogram::HistogramState::default(),
        }
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
                super::menu_bar::render_menu_bar(ui, &mut self.state);
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

        // Component Properties Dialog

        {
            use crate::properties::dialog::{render_properties_dialog, PropertiesDialogResult};
            let result = render_properties_dialog(ctx, &mut self.state.property_editor);
            match result {
                PropertiesDialogResult::Apply(id, props) => {
                    let _ = apply_component_property_edits(&mut self.state, id, props);
                }
                PropertiesDialogResult::Cancel => {
                    // Dialog was cancelled, nothing to do
                }
                PropertiesDialogResult::None => {
                    // Dialog still open or not shown
                }
            }
        }

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

        // Model Browser Dialog (standalone access from menu)
        // Reuses the existing model browser from the property editor
        {
            use crate::properties::model_browser::render_model_browser;
            let _ = render_model_browser(
                ctx,
                &mut self.state.model_browser_state,
                &self.state.model_library_manager,
            );
        }

        self.process_new_cell_dialog(ctx);
        self.process_new_view_dialog(ctx);
        self.process_pending_library_deletions();
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
mod tests {
    use super::*;

    fn make_test_app() -> RSpiceApp {
        RSpiceApp {
            state: AppState::default(),
            first_frame: false,
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
        }
    }

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(
            !state.panels.project_browser,
            "Browser should be hidden by default"
        );
        assert!(
            state.panels.properties,
            "Properties should be visible by default"
        );
        assert!(
            state.panels.bottom_panel,
            "Bottom panel should be visible by default"
        );
        assert_eq!(
            state.panels.active_bottom_tab,
            BottomPanelTab::Log,
            "Log tab should be active by default"
        );
    }

    #[test]
    fn test_panel_sizes_default() {
        let sizes = PanelSizes::default();
        assert_eq!(sizes.waveform_height, 300.0);
        assert_eq!(sizes.console_height, 120.0);
        assert_eq!(sizes.browser_width, 220.0);
        assert_eq!(sizes.properties_width, 250.0);
    }

    #[test]
    fn test_console_message_info() {
        let msg = ConsoleMessage::info("Test message");
        assert_eq!(msg.level, ConsoleLevel::Info);
        assert_eq!(msg.message, "Test message");
    }

    #[test]
    fn test_console_message_warning() {
        let msg = ConsoleMessage::warning("Warning message");
        assert_eq!(msg.level, ConsoleLevel::Warning);
    }

    #[test]
    fn test_console_message_error() {
        let msg = ConsoleMessage::error("Error message");
        assert_eq!(msg.level, ConsoleLevel::Error);
    }

    #[test]
    fn test_dialog_state_default() {
        let dialogs = DialogState::default();
        assert!(!dialogs.simulation_dialog);
        assert!(!dialogs.simulation_options);
        assert!(!dialogs.about);
        assert!(!dialogs.preferences);
        assert!(!dialogs.shortcuts_help);
        assert_eq!(dialogs.simulation_options_state.active_tab, 0);
        assert!(dialogs.simulation_options_errors.is_empty());
        assert!((dialogs.simulation_options_config.reltol - 1e-3).abs() < 1e-15);
    }

    #[test]
    fn test_panel_visibility_serialization() {
        let panels = PanelVisibility {
            project_browser: true,
            results_browser: false,
            properties: false,
            bottom_panel: true,
            active_bottom_tab: BottomPanelTab::Waveform,
            smith_chart: false,
            signal_browser: false,
            script_console: false,
        };
        let ser = PanelVisibilitySer::from(&panels);
        assert!(ser.project_browser);
        assert!(!ser.properties);
        assert!(ser.bottom_panel);
        assert_eq!(ser.active_bottom_tab, 1); // Waveform = 1

        let panels2: PanelVisibility = ser.into();
        assert!(panels2.project_browser);
        assert!(!panels2.properties);
        assert!(panels2.bottom_panel);
        assert_eq!(panels2.active_bottom_tab, BottomPanelTab::Waveform);
    }

    #[test]
    fn test_panel_visibility_deserialization_legacy_console_index_maps_to_log() {
        let legacy = PanelVisibilitySer {
            project_browser: false,
            results_browser: false,
            properties: true,
            bottom_panel: true,
            active_bottom_tab: 0,
            smith_chart: false,
            signal_browser: false,
            script_console: false,
        };
        let panels: PanelVisibility = legacy.into();
        assert_eq!(panels.active_bottom_tab, BottomPanelTab::Log);
    }

    #[test]
    fn test_panel_sizes_serialization() {
        let sizes = PanelSizes {
            waveform_height: 300.0,
            console_height: 150.0,
            browser_width: 280.0,
            properties_width: 320.0,
        };
        let ser = PanelSizesSer::from(&sizes);
        assert_eq!(ser.waveform_height, 300.0);

        let sizes2: PanelSizes = ser.into();
        assert_eq!(sizes2.waveform_height, 300.0);
        assert_eq!(sizes2.console_height, 150.0);
    }

    #[test]
    fn test_theme_is_dark_by_default() {
        let state = AppState::default();
        assert!(
            state.theme.is_dark,
            "Theme should be dark by default for EDA"
        );
    }

    // =========================================================================
    // Save Confirmation Dialog Tests
    // Commercial-grade testing for unsaved changes workflow
    // =========================================================================

    #[test]
    fn test_confirmation_action_dialog_titles() {
        // Verify all actions have appropriate dialog titles
        assert_eq!(
            ConfirmationAction::FileNew.dialog_title(),
            "Create New Schematic",
            "FileNew should have descriptive title"
        );
        assert_eq!(
            ConfirmationAction::FileOpen.dialog_title(),
            "Open Schematic",
            "FileOpen should have descriptive title"
        );
        assert_eq!(
            ConfirmationAction::Exit.dialog_title(),
            "Exit RSpice",
            "Exit should have descriptive title"
        );
    }

    #[test]
    fn test_confirmation_action_prompt_messages() {
        // All actions should have clear, user-friendly prompts
        let message = ConfirmationAction::FileNew.prompt_message();
        assert!(
            message.contains("unsaved"),
            "Prompt should mention unsaved changes"
        );
        assert!(message.contains("save"), "Prompt should mention saving");
    }

    #[test]
    fn test_confirmation_dialog_state_default() {
        let state = ConfirmationDialogState::default();
        assert!(!state.visible, "Dialog should be hidden by default");
        assert!(
            state.pending_action.is_none(),
            "No pending action by default"
        );
    }

    #[test]
    fn test_confirmation_dialog_state_show() {
        let mut state = ConfirmationDialogState::default();

        // Test showing dialog for FileNew
        state.show(ConfirmationAction::FileNew);
        assert!(state.visible, "Dialog should be visible after show()");
        assert_eq!(
            state.pending_action,
            Some(ConfirmationAction::FileNew),
            "Pending action should be set"
        );

        // Test showing dialog for different action
        state.show(ConfirmationAction::FileOpen);
        assert!(state.visible, "Dialog should remain visible");
        assert_eq!(
            state.pending_action,
            Some(ConfirmationAction::FileOpen),
            "Pending action should be updated"
        );
    }

    #[test]
    fn test_confirmation_dialog_state_close() {
        let mut state = ConfirmationDialogState::default();

        // Show then close
        state.show(ConfirmationAction::FileNew);
        state.close();

        assert!(!state.visible, "Dialog should be hidden after close()");
        assert!(
            state.pending_action.is_none(),
            "Pending action should be cleared after close()"
        );
    }

    #[test]
    fn test_confirmation_dialog_state_is_showing() {
        let mut state = ConfirmationDialogState::default();

        // Not showing anything initially
        assert!(
            !state.is_showing(ConfirmationAction::FileNew),
            "Should not be showing FileNew initially"
        );

        // Show FileNew
        state.show(ConfirmationAction::FileNew);
        assert!(
            state.is_showing(ConfirmationAction::FileNew),
            "Should be showing FileNew after show()"
        );
        assert!(
            !state.is_showing(ConfirmationAction::FileOpen),
            "Should not be showing FileOpen"
        );
        assert!(
            !state.is_showing(ConfirmationAction::Exit),
            "Should not be showing Exit"
        );
    }

    #[test]
    fn test_confirmation_response_enum_completeness() {
        // Verify all three commercial-standard responses exist
        let responses = [
            ConfirmationResponse::Yes,
            ConfirmationResponse::No,
            ConfirmationResponse::Cancel,
        ];
        assert_eq!(
            responses.len(),
            3,
            "Should have exactly 3 response options (Yes/No/Cancel)"
        );

        // Verify they are all distinct
        assert_ne!(ConfirmationResponse::Yes, ConfirmationResponse::No);
        assert_ne!(ConfirmationResponse::Yes, ConfirmationResponse::Cancel);
        assert_ne!(ConfirmationResponse::No, ConfirmationResponse::Cancel);
    }

    #[test]
    fn test_app_state_has_confirmation_dialog() {
        let state = AppState::default();

        // Verify confirmation dialog is accessible and properly initialized
        assert!(
            !state.dialogs.confirmation_dialog.visible,
            "Confirmation dialog should be hidden by default"
        );
        assert!(
            state.dialogs.confirmation_dialog.pending_action.is_none(),
            "No pending action on fresh AppState"
        );
    }

    #[test]
    fn test_app_state_exit_requested_default() {
        let state = AppState::default();
        assert!(
            !state.exit_requested,
            "Exit should not be requested by default"
        );
    }

    #[test]
    fn test_confirmation_action_is_copy_and_eq() {
        // Verify ConfirmationAction implements Copy and Eq for efficiency
        let action = ConfirmationAction::FileNew;
        let action_copy = action; // Copy
        assert_eq!(
            action, action_copy,
            "ConfirmationAction should implement Eq"
        );

        // Verify all variants can be compared
        let actions = [
            ConfirmationAction::FileNew,
            ConfirmationAction::FileOpen,
            ConfirmationAction::Exit,
        ];
        for (i, a) in actions.iter().enumerate() {
            for (j, b) in actions.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b, "Same action should be equal");
                } else {
                    assert_ne!(a, b, "Different actions should not be equal");
                }
            }
        }
    }

    #[test]
    fn test_confirmation_dialog_workflow_complete_cycle() {
        // Simulate a complete workflow: dirty state -> show dialog -> close
        let mut state = ConfirmationDialogState::default();

        // Initial state
        assert!(!state.visible);

        // User triggers action that requires confirmation
        state.show(ConfirmationAction::Exit);
        assert!(state.visible);
        assert_eq!(state.pending_action, Some(ConfirmationAction::Exit));

        // User cancels
        state.close();
        assert!(!state.visible);
        assert!(state.pending_action.is_none());

        // Trigger another action
        state.show(ConfirmationAction::FileNew);
        assert!(state.is_showing(ConfirmationAction::FileNew));

        // Complete the action
        let action = state.pending_action;
        state.close();
        assert_eq!(action, Some(ConfirmationAction::FileNew));
    }

    #[test]
    fn test_confirmation_yes_executes_pending_exit_after_successful_save() {
        let mut app = make_test_app();
        let temp = tempfile::tempdir().expect("tempdir should create");
        let save_path = temp.path().join("confirmation-save-success.rsch");
        app.state.schematic.current_file = Some(save_path.clone());
        app.state.schematic.is_dirty = true;
        app.state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::Exit);

        app.handle_confirmation_response(ConfirmationResponse::Yes);

        assert!(
            app.state.exit_requested,
            "successful save should allow pending exit action to proceed"
        );
        assert!(
            save_path.exists(),
            "successful confirmation save should persist schematic file"
        );
        assert!(
            !app.state.schematic.is_dirty,
            "successful save should clear dirty flag"
        );
        assert!(!app.state.dialogs.confirmation_dialog.visible);
        assert!(app
            .state
            .dialogs
            .confirmation_dialog
            .pending_action
            .is_none());
    }

    #[test]
    fn test_confirmation_yes_does_not_execute_pending_exit_when_save_fails() {
        let mut app = make_test_app();
        let temp = tempfile::tempdir().expect("tempdir should create");
        let invalid_file_target = temp.path().to_path_buf(); // existing directory: save should fail
        app.state.schematic.current_file = Some(invalid_file_target);
        app.state.schematic.is_dirty = true;
        app.state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::Exit);

        app.handle_confirmation_response(ConfirmationResponse::Yes);

        assert!(
            !app.state.exit_requested,
            "failed save must not continue to pending exit action"
        );
        assert!(
            app.state.schematic.is_dirty,
            "failed save should keep schematic dirty"
        );
        assert!(
            app.state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Save failed")),
            "failed save should emit an error message"
        );
        assert!(!app.state.dialogs.confirmation_dialog.visible);
        assert!(app
            .state
            .dialogs
            .confirmation_dialog
            .pending_action
            .is_none());
    }

    #[test]
    fn test_confirmation_yes_does_not_execute_pending_new_when_save_fails() {
        use crate::state::{Component, ComponentType, Point};

        let mut app = make_test_app();
        app.state.schematic.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(100, 100))
                .with_name_value("R1", "1k"),
        );
        let temp = tempfile::tempdir().expect("tempdir should create");
        let invalid_file_target = temp.path().to_path_buf(); // existing directory: save should fail
        app.state.schematic.current_file = Some(invalid_file_target);
        app.state.schematic.is_dirty = true;
        app.state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::FileNew);

        app.handle_confirmation_response(ConfirmationResponse::Yes);

        assert!(
            !app.state.schematic.components.is_empty(),
            "failed save must not continue into destructive FileNew action"
        );
        assert!(
            app.state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Save failed")),
            "failed save should emit an error message"
        );
    }
}
