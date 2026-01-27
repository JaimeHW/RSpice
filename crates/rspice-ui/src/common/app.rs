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
//! ┌────────────────────────────────────────────────────────────────┐
//! │ Menu Bar (File, Edit, View, Simulate, Tools, Help)            │
//! ├────────────────────────────────────────────────────────────────┤
//! │ Toolbar (Simulation controls, zoom, etc.)                      │
//! ├──┬───────────────────────────────────────────────────────┬────┤
//! │  │ Project      │                                        │    │
//! │ I│ Browser      │     Schematic Editor                   │ P  │
//! │ C│ (Library/    │     (GPU-rendered via wgpu)            │ r  │
//! │ O│  Cell/View)  │                                        │ o  │
//! │ N│              │                                        │ p  │
//! │  ├──────────────┴────────────────────────────────────────┤ s  │
//! │ R│               Waveform Viewer (resizable)             │    │
//! │ A├───────────────────────────────────────────────────────┤────┤
//! │ I│               Console (resizable)                     │    │
//! │ L│                                                       │    │
//! └──┴───────────────────────────────────────────────────────┴────┘
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

use egui::{
    CentralPanel, Color32, Context, Frame, Key, Modifiers, RichText, SidePanel, TopBottomPanel, Ui,
    Vec2,
};

use crate::state::{SchematicState, SimulationState};
use crate::waveform::WaveformViewerState;

use super::theme::RSpiceTheme;

// =============================================================================
// Application State
// =============================================================================

/// Panel visibility state
#[derive(Debug, Clone)]
pub struct PanelVisibility {
    /// Project browser (Library/Cell/View tree)
    pub project_browser: bool,
    /// Properties panel (right side)
    pub properties: bool,
    /// Waveform viewer (bottom)
    pub waveform: bool,
    /// Console output (bottom)
    pub console: bool,
    /// Smith chart viewer
    pub smith_chart: bool,
    /// Cross-probe signal browser
    pub signal_browser: bool,
    /// Scripting/Automation console
    pub script_console: bool,
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self {
            project_browser: false,
            properties: true,
            waveform: false, // Hidden by default, shown when sim results arrive
            console: true,
            smith_chart: false,
            signal_browser: false,
            script_console: false,
        }
    }
}

/// Resizable panel heights (in pixels)
#[derive(Debug, Clone)]
pub struct PanelSizes {
    /// Waveform panel height
    pub waveform_height: f32,
    /// Console panel height
    pub console_height: f32,
    /// Project browser width
    pub browser_width: f32,
    /// Properties panel width
    pub properties_width: f32,
}

impl Default for PanelSizes {
    fn default() -> Self {
        Self {
            waveform_height: 300.0,
            console_height: 120.0,
            browser_width: 220.0,
            properties_width: 250.0,
        }
    }
}

/// Dialog visibility state
#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// Simulation setup dialog
    pub simulation_dialog: bool,
    /// Simulation options dialog
    pub simulation_options: bool,
    /// About dialog
    pub about: bool,
    /// Preferences dialog
    pub preferences: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// New Cell creation dialog
    pub new_cell_dialog: bool,
    /// New Cell name input
    pub new_cell_name: String,
    /// Active simulation tab (0=OP, 1=Tran, 2=AC, 3=DC, 4=Noise, 5=PZ, 6=Sens, 7=MC, 8=PSS, 9=STB, 10=Temp)
    pub sim_active_tab: usize,
    // --- Transient Analysis ---
    /// Transient stop time
    pub tran_stop: String,
    /// Transient step time
    pub tran_step: String,
    /// Transient start time
    pub tran_start: String,
    /// Transient max step
    pub tran_maxstep: String,
    /// Use initial conditions
    pub tran_uic: bool,
    // --- AC Analysis ---
    /// AC start frequency
    pub ac_fstart: String,
    /// AC stop frequency
    pub ac_fstop: String,
    /// AC points per decade
    pub ac_points: String,
    /// AC sweep type (0=decade, 1=octave, 2=linear)
    pub ac_sweep_type: usize,
    // --- DC Analysis ---
    /// DC source name
    pub dc_source: String,
    /// DC start value
    pub dc_start: String,
    /// DC stop value
    pub dc_stop: String,
    /// DC step value
    pub dc_step: String,
    /// DC nested sweep enabled
    pub dc_nested: bool,
    /// DC source 2 name
    pub dc_source2: String,
    /// DC start2 value
    pub dc_start2: String,
    /// DC stop2 value
    pub dc_stop2: String,
    /// DC step2 value
    pub dc_step2: String,
    // --- Noise Analysis ---
    /// Noise output node
    pub noise_output: String,
    /// Noise reference node
    pub noise_ref: String,
    /// Noise input source
    pub noise_input: String,
    /// Noise frequency start
    pub noise_fstart: String,
    /// Noise frequency stop
    pub noise_fstop: String,
    // --- Pole-Zero Analysis ---
    /// PZ input node
    pub pz_input: String,
    /// PZ output node
    pub pz_output: String,
    /// PZ type (0=both, 1=poles only, 2=zeros only)
    pub pz_type: usize,
    // --- Sensitivity Analysis ---
    /// Sensitivity output variable
    pub sens_output: String,
    /// Sensitivity type (0=DC, 1=AC)
    pub sens_type: usize,
    // --- Monte Carlo Analysis ---
    /// Number of MC runs
    pub mc_runs: String,
    /// MC seed (0=random)
    pub mc_seed: String,
    /// MC variation type (0=uniform, 1=gaussian)
    pub mc_variation: usize,
    /// MC analysis type (0=tran, 1=ac, 2=dc)
    pub mc_analysis: usize,
    // --- PSS (Periodic Steady State) Analysis ---
    /// PSS fundamental frequency
    pub pss_fund: String,
    /// PSS number of harmonics
    pub pss_harmonics: String,
    /// PSS oscillator mode
    pub pss_oscmode: bool,
    /// PSS max iterations
    pub pss_maxiter: String,
    // --- STB (Stability) Analysis ---
    /// STB probe source
    pub stb_probe: String,
    /// STB start frequency
    pub stb_fstart: String,
    /// STB stop frequency
    pub stb_fstop: String,
    // --- Temperature Sweep ---
    /// Temperature start (°C)
    pub temp_start: String,
    /// Temperature stop (°C)
    pub temp_stop: String,
    /// Temperature step (°C)
    pub temp_step: String,
    /// Temp sweep analysis type
    pub temp_analysis: usize,
    // --- Harmonic Balance (HB) ---
    pub hb_state: crate::simulation::dialog::hb::HbDialogState,
    // --- S-Parameter ---
    pub sp_state: crate::simulation::dialog::sp::SpDialogState,
    // --- PAC (Periodic AC) ---
    pub pac_state: crate::simulation::dialog::pac::PacDialogState,
    // --- PNoise (Periodic Noise) ---
    pub pnoise_state: crate::simulation::dialog::pnoise::PnoiseDialogState,
    // --- PXF (Periodic Transfer) ---
    pub pxf_state: crate::simulation::dialog::pxf::PxfDialogState,
    // --- PSTB (Periodic Stability) ---
    pub pstb_state: crate::simulation::dialog::pstb::PstbDialogState,
    // --- XF (Transfer Function) ---
    pub xf_state: crate::simulation::dialog::xf::XfDialogState,
    // --- Corner Analysis ---
    pub corner_state: crate::simulation::dialog::corner::CornerDialogState,
    // --- Envelope Transient ---
    pub envelope_state: crate::simulation::dialog::envelope::EnvelopeDialogState,
    // --- Fourier ---
    pub fourier_state: crate::simulation::dialog::fourier::FourierDialogState,
    // --- DC Operating Point ---
    pub op_state: crate::simulation::dialog::op::OpDialogState,
    // --- Pole-Zero ---
    pub pz_state: crate::simulation::dialog::pz::PzDialogState,
    // --- Sensitivity ---
    pub sens_state: crate::simulation::dialog::sens::SensDialogState,
    // --- Monte Carlo ---
    pub mc_state: crate::simulation::dialog::mc::McDialogState,
    // --- PSS ---
    pub pss_state: crate::simulation::dialog::pss::PssDialogState,
    // --- STB ---
    pub stb_state: crate::simulation::dialog::stb::StbDialogState,
    // --- Temperature Sweep ---
    pub temp_state: crate::simulation::dialog::temp::TempDialogState,

    /// Starting position of selection drag (grid coords)
    pub drag_start: Option<(i32, i32)>,
    /// Last drag position for computing delta (grid coords)
    pub last_drag_pos: Option<(i32, i32)>,

    // =========================================================================
    // Commercial-Grade Tool Dialogs
    // =========================================================================

    // --- DRC/ERC Dialog ---
    /// DRC results dialog open
    pub drc_dialog: bool,
    /// DRC results (cached from last run)
    pub drc_results: Option<crate::services::drc::DrcResult>,
    /// DRC running indicator
    pub drc_running: bool,

    // --- PDF Export Dialog ---
    /// PDF export dialog open
    pub pdf_export_dialog: bool,
    /// PDF export configuration
    pub pdf_config: crate::services::pdf_export::PdfExportConfig,

    // --- Waveform Calculator Dialog ---
    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,
    /// Calculator expression input
    pub calc_expression: String,
    /// Calculator error message (if any)
    pub calc_error: Option<String>,

    // --- Measurements Panel ---
    /// Measurements panel open
    pub measurements_panel: bool,

    // --- Model Browser Dialog ---
    /// Model browser dialog open
    pub model_browser_dialog: bool,
    /// Model browser search filter
    pub model_browser_filter: String,

    // --- Engineering Calculators ---
    /// Unit converter dialog open
    pub unit_converter_dialog: bool,
    /// Filter calculator dialog open
    pub filter_calculator_dialog: bool,
    /// Impedance calculator dialog open
    pub impedance_calculator_dialog: bool,
    /// S-parameter converter dialog open
    pub sparam_converter_dialog: bool,
}

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
    /// Component property editor state
    pub property_editor: crate::properties::dialog::PropertyEditorState,
    /// Scripting/Automation console state
    pub script_console: crate::panels::ScriptConsoleState,
    /// Active specialized viewer state
    pub active_viewer: crate::viewers::ActiveViewer,
    /// Waveform viewer state (persists across frames for pan/zoom)
    pub waveform_viewer: WaveformViewerState,
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
            property_editor: crate::properties::dialog::PropertyEditorState::default(),
            script_console: crate::panels::ScriptConsoleState::default(),
            active_viewer: crate::viewers::ActiveViewer::default(),
            waveform_viewer: WaveformViewerState::default(),
        }
    }
}

/// Console message with severity level
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message severity
    pub level: ConsoleLevel,
    /// Timestamp (epoch seconds)
    pub timestamp: f64,
    /// Message content
    pub message: String,
}

/// Console message severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleLevel {
    Info,
    Warning,
    Error,
}

impl ConsoleMessage {
    /// Get current timestamp as epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Info,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Warning,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Error,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
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
        let state = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppState::default()
        };

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

    /// Handle keyboard shortcuts
    fn handle_shortcuts(&mut self, ctx: &Context) {
        // File shortcuts
        if ctx.input(|i| i.key_pressed(Key::N) && i.modifiers.ctrl) {
            self.action_file_new();
        }
        if ctx.input(|i| i.key_pressed(Key::O) && i.modifiers.ctrl) {
            self.action_file_open();
        }
        if ctx.input(|i| i.key_pressed(Key::S) && i.modifiers.ctrl) {
            self.action_file_save();
        }

        // Edit shortcuts
        if ctx.input(|i| i.key_pressed(Key::Z) && i.modifiers.ctrl && !i.modifiers.shift) {
            self.action_edit_undo();
        }
        if ctx.input(|i| i.key_pressed(Key::Y) && i.modifiers.ctrl)
            || ctx.input(|i| i.key_pressed(Key::Z) && i.modifiers.ctrl && i.modifiers.shift)
        {
            self.action_edit_redo();
        }
        if ctx.input(|i| i.key_pressed(Key::C) && i.modifiers.ctrl) {
            self.action_edit_copy();
        }
        if ctx.input(|i| i.key_pressed(Key::V) && i.modifiers.ctrl) {
            self.action_edit_paste();
        }
        if ctx.input(|i| i.key_pressed(Key::X) && i.modifiers.ctrl) {
            self.action_edit_cut();
        }
        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            self.action_edit_delete();
        }
        if ctx.input(|i| i.key_pressed(Key::A) && i.modifiers.ctrl) {
            self.action_edit_select_all();
        }

        // View shortcuts
        if ctx.input(|i| i.key_pressed(Key::L) && i.modifiers.ctrl && i.modifiers.shift) {
            self.toggle_panel_browser();
        }
        if ctx.input(|i| i.key_pressed(Key::Backtick) && i.modifiers.ctrl) {
            self.toggle_panel_console();
        }

        // Help shortcuts
        if ctx.input(|i| i.key_pressed(Key::F1)) {
            self.state.dialogs.shortcuts_help = true;
        }

        // Tool switching shortcuts (no modifiers - for schematic editing)
        // Only active when not typing in a text field
        let has_focus = ctx.memory(|m| m.focused().is_some());
        if !has_focus {
            use crate::state::{ComponentType, Tool};

            // Select tool
            if ctx.input(|i| i.key_pressed(Key::S) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Select;
            }
            // Wire tool
            if ctx.input(|i| i.key_pressed(Key::W) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Wire;
            }
            // Ground
            if ctx.input(|i| i.key_pressed(Key::G) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::Ground);
            }
            // Voltage source
            if ctx.input(|i| i.key_pressed(Key::V) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
            }
            // Current source
            if ctx.input(|i| i.key_pressed(Key::I) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::CurrentSource);
            }
            // Capacitor
            if ctx.input(|i| i.key_pressed(Key::C) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::Capacitor);
            }
            // Inductor
            if ctx.input(|i| i.key_pressed(Key::L) && !i.modifiers.ctrl && !i.modifiers.shift) {
                self.state.schematic.tool = Tool::Place(ComponentType::Inductor);
            }
            // Diode
            if ctx.input(|i| i.key_pressed(Key::D) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::Diode);
            }
            // NMOS
            if ctx.input(|i| i.key_pressed(Key::M) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::Nmos);
            }
            // NPN BJT
            if ctx.input(|i| i.key_pressed(Key::Q) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::NpnBjt);
            }
            // Probe tool
            if ctx.input(|i| i.key_pressed(Key::P) && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Probe;
            }
            // Rotate preview/selection (R key)
            if ctx.input(|i| i.key_pressed(Key::R) && !i.modifiers.ctrl) {
                // Rotate preview rotation for component placement
                self.state.schematic.preview_rotation =
                    self.state.schematic.preview_rotation.rotate_cw();
                // Also rotate selected components
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.rotate_selection();
                }
            }
            // Mirror horizontal (H key) - Cadence Virtuoso convention
            if ctx.input(|i| i.key_pressed(Key::H) && !i.modifiers.ctrl) {
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.mirror_selection_h();
                }
            }
            // Mirror vertical (Y key) - since V is voltage source
            if ctx.input(|i| i.key_pressed(Key::Y) && !i.modifiers.ctrl) {
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.mirror_selection_v();
                }
            }
            // Edit properties (E key) - Cadence Virtuoso convention
            if ctx.input(|i| i.key_pressed(Key::E) && !i.modifiers.ctrl) {
                if let Some(comp_id) = self.state.schematic.selection.single_component() {
                    if let Some(comp) = self
                        .state
                        .schematic
                        .components
                        .iter()
                        .find(|c| c.id == comp_id)
                    {
                        let props = crate::properties::dialog::EditedProperties {
                            name: comp.name.clone(),
                            value: comp.value.clone(),
                            model: String::new(), // Component doesn't have model field yet
                            parameters: vec![],
                        };
                        self.state.property_editor.open_for(comp_id, props);
                    }
                }
            }
            // Escape to cancel/deselect
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                // Cancel property editor if open
                if self.state.property_editor.open {
                    self.state.property_editor.cancel();
                } else {
                    self.state.schematic.tool = Tool::Select;
                    self.state.schematic.cancel_wire();
                    self.state.schematic.selection.clear();
                    // Cancel box selection
                    self.state.schematic.selection_rect.cancel();
                }
            }
        }
    }

    // =========================================================================
    // Action Handlers
    // =========================================================================

    fn action_file_new(&mut self) {
        if self.state.schematic.is_dirty {
            // TODO: Show save confirmation dialog
            log::warn!("New schematic requested but current has unsaved changes");
        }
        self.state.schematic = SchematicState::default();
        self.state
            .console_messages
            .push(ConsoleMessage::info("Created new schematic"));
    }

    fn action_file_open(&mut self) {
        use crate::io::{load_schematic, show_open_dialog, SchematicIoError};

        match show_open_dialog() {
            Ok(path) => match load_schematic(&path) {
                Ok(schematic) => {
                    self.state.schematic = schematic;
                    self.state
                        .console_messages
                        .push(ConsoleMessage::info(format!("Opened: {}", path.display())));
                }
                Err(e) => {
                    self.state
                        .console_messages
                        .push(ConsoleMessage::error(format!("Failed to open: {}", e)));
                }
            },
            Err(SchematicIoError::Cancelled) => {
                // User cancelled - no message needed
            }
            Err(e) => {
                self.state
                    .console_messages
                    .push(ConsoleMessage::error(format!("Open failed: {}", e)));
            }
        }
    }

    fn action_file_save(&mut self) {
        use crate::io::{save_schematic, show_save_dialog, SchematicIoError};

        // If we have a current file path, save directly
        // Otherwise, show Save As dialog
        if let Some(ref path) = self.state.schematic.current_file.clone() {
            match save_schematic(&self.state.schematic, path) {
                Ok(()) => {
                    self.state.schematic.is_dirty = false;
                    self.state
                        .console_messages
                        .push(ConsoleMessage::info(format!("Saved: {}", path.display())));
                }
                Err(e) => {
                    self.state
                        .console_messages
                        .push(ConsoleMessage::error(format!("Save failed: {}", e)));
                }
            }
        } else {
            // No current file - do Save As
            self.action_file_save_as();
        }
    }

    fn action_file_save_as(&mut self) {
        use crate::io::{save_schematic, show_save_dialog, SchematicIoError};

        // Get default filename from current file or use "untitled"
        let default_name = self
            .state
            .schematic
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string());

        match show_save_dialog(default_name.as_deref()) {
            Ok(path) => match save_schematic(&self.state.schematic, &path) {
                Ok(()) => {
                    self.state.schematic.current_file = Some(path.clone());
                    self.state.schematic.is_dirty = false;
                    self.state
                        .console_messages
                        .push(ConsoleMessage::info(format!("Saved: {}", path.display())));
                }
                Err(e) => {
                    self.state
                        .console_messages
                        .push(ConsoleMessage::error(format!("Save failed: {}", e)));
                }
            },
            Err(SchematicIoError::Cancelled) => {
                // User cancelled - no message needed
            }
            Err(e) => {
                self.state
                    .console_messages
                    .push(ConsoleMessage::error(format!("Save As failed: {}", e)));
            }
        }
    }

    fn action_edit_undo(&mut self) {
        if self.state.schematic.can_undo() {
            let desc = self
                .state
                .schematic
                .undo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.undo() {
                self.state
                    .console_messages
                    .push(ConsoleMessage::info(format!("Undo: {}", desc)));
            }
        } else {
            self.state
                .console_messages
                .push(ConsoleMessage::info("Nothing to undo"));
        }
    }

    fn action_edit_redo(&mut self) {
        if self.state.schematic.can_redo() {
            let desc = self
                .state
                .schematic
                .redo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.redo() {
                self.state
                    .console_messages
                    .push(ConsoleMessage::info(format!("Redo: {}", desc)));
            }
        } else {
            self.state
                .console_messages
                .push(ConsoleMessage::info("Nothing to redo"));
        }
    }

    fn action_edit_copy(&mut self) {
        self.state.schematic.copy_selection();
    }

    fn action_edit_paste(&mut self) {
        use crate::state::Point;
        self.state.schematic.paste_at(Point::new(200, 200));
    }

    fn action_edit_cut(&mut self) {
        self.state.schematic.copy_selection();
        self.state.schematic.delete_selection();
    }

    fn action_edit_delete(&mut self) {
        self.state.schematic.delete_selection();
    }

    fn action_edit_select_all(&mut self) {
        self.state.schematic.selection.clear();
        for comp in &self.state.schematic.components {
            self.state.schematic.selection.select_component(comp.id);
        }
        for wire in &self.state.schematic.wires {
            self.state.schematic.selection.select_wire(wire.id);
        }
    }

    fn toggle_panel_browser(&mut self) {
        self.state.panels.project_browser = !self.state.panels.project_browser;
    }

    fn toggle_panel_console(&mut self) {
        self.state.panels.console = !self.state.panels.console;
    }

    fn toggle_panel_waveform(&mut self) {
        self.state.panels.waveform = !self.state.panels.waveform;
    }

    fn toggle_panel_properties(&mut self) {
        self.state.panels.properties = !self.state.panels.properties;
    }

    /// Render the analysis options for the simulation dialog
    fn render_analysis_options(&mut self, ui: &mut egui::Ui) {
        match self.state.dialogs.sim_active_tab {
            0 => {
                self.state.dialogs.op_state.render(ui);
            }
            1 => {
                ui.heading("Transient Analysis");
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("Time-domain analysis for switching and dynamic circuits")
                        .weak(),
                );
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Time Parameters").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("tran_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Stop Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_stop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Step Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_step)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Start Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_start)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Max Step:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_maxstep)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Options").strong());
                    ui.add_space(5.0);
                    ui.checkbox(
                        &mut self.state.dialogs.tran_uic,
                        "Use Initial Conditions (UIC)",
                    );
                });
            }
            2 => {
                ui.heading("AC Analysis");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Small-signal frequency response analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Frequency Range").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("ac_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Start Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstart)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Points/Decade:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_points)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Sweep Type:");
                            let sweep_types = ["Decade", "Octave", "Linear"];
                            egui::ComboBox::from_id_salt("ac_sweep")
                                .selected_text(sweep_types[self.state.dialogs.ac_sweep_type])
                                .show_ui(ui, |ui| {
                                    for (idx, name) in sweep_types.iter().enumerate() {
                                        if ui
                                            .selectable_label(
                                                self.state.dialogs.ac_sweep_type == idx,
                                                *name,
                                            )
                                            .clicked()
                                        {
                                            self.state.dialogs.ac_sweep_type = idx;
                                        }
                                    }
                                });
                            ui.end_row();
                        });
                });
            }
            3 => {
                ui.heading("DC Sweep");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("DC parameter sweep analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Primary Sweep").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("dc_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Source:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_source)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Start:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_start)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_stop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Step:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_step)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.checkbox(&mut self.state.dialogs.dc_nested, "Enable Nested Sweep");
                if self.state.dialogs.dc_nested {
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Secondary Sweep").strong());
                        ui.add_space(5.0);
                        egui::Grid::new("dc_grid2")
                            .num_columns(2)
                            .spacing([20.0, 6.0])
                            .show(ui, |ui| {
                                ui.label("Source:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_source2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Start:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_start2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Stop:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_stop2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Step:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_step2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                            });
                    });
                }
            }
            4 => {
                ui.heading("Noise Analysis");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Spectral noise analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Node Configuration").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("noise_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Output Node:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_output)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Reference:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_ref)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Input Source:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_input)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Frequency Range").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("noise_freq")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Start Freq:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_fstart)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop Freq:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_fstop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
            }
            5 => {
                self.state.dialogs.pz_state.render(ui);
            }
            6 => {
                self.state.dialogs.sens_state.render(ui);
            }
            7 => {
                self.state.dialogs.mc_state.render(ui);
            }
            8 => {
                self.state.dialogs.pss_state.render(ui);
            }
            9 => {
                self.state.dialogs.stb_state.render(ui);
            }
            10 => {
                self.state.dialogs.temp_state.render(ui);
            }
            11 => {
                self.state.dialogs.hb_state.render(ui);
            }
            12 => {
                self.state.dialogs.sp_state.render(ui);
            }
            13 => {
                self.state.dialogs.pac_state.render(ui);
            }
            14 => {
                self.state.dialogs.pnoise_state.render(ui);
            }
            15 => {
                self.state.dialogs.pxf_state.render(ui);
            }
            16 => {
                self.state.dialogs.pstb_state.render(ui);
            }
            17 => {
                self.state.dialogs.xf_state.render(ui);
            }
            18 => {
                self.state.dialogs.corner_state.render(ui);
            }
            19 => {
                self.state.dialogs.envelope_state.render(ui);
            }
            20 => {
                self.state.dialogs.fourier_state.render(ui);
            }
            21 => {
                ui.heading("Reliability Analysis (Aging)");
                ui.add_space(5.0);
                ui.label(
                    RichText::new(
                        "Analyze long-term circuit degradation due to HCI and NBTI effects.",
                    )
                    .weak(),
                );
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Aging Parameters").strong());
                    ui.checkbox(&mut true, "Enable HCI (Hot Carrier Injection)");
                    ui.checkbox(
                        &mut true,
                        "Enable NBTI (Negative Bias Temperature Instability)",
                    );
                    ui.horizontal(|ui| {
                        ui.label("Simulation Lifetime:");
                        ui.add(egui::DragValue::new(&mut 10.0));
                        ui.label("Years");
                    });
                });
            }
            22 => {
                ui.heading("Optimization Engine");
                ui.add_space(5.0);
                ui.label(RichText::new("Automated parameter tuning to meet design goals.").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Optimizer Strategy").strong());
                    ui.horizontal(|ui| {
                        ui.label("Goal:");
                        ui.label(
                            RichText::new("Minimize Power").color(Color32::from_rgb(100, 200, 255)),
                        );
                    });
                    ui.label("Iterations: 100");
                });
            }
            23 => {
                ui.heading("Safety Checking (SOA)");
                ui.add_space(5.0);
                ui.label(
                    RichText::new("Monitor Safe Operating Area violations during simulation.")
                        .weak(),
                );
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(RichText::new("SOA Rules").strong());
                    ui.checkbox(&mut true, "Check Vgs Max");
                    ui.checkbox(&mut true, "Check Vds Max");
                    ui.checkbox(&mut true, "Check Id Max");
                });
            }
            _ => {}
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
            .frame(Frame::none().fill(egui::Color32::from_rgb(38, 42, 52)))
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

        // =====================================================================
        // Panel Layout Order:
        // 1. Icon Rail (leftmost) - first so all other content is to its right
        // 2. Console/Waveform (bottom) - spans full width except icon rail
        // 3. Side panels (project browser, properties)
        // 4. Central panel (schematic)
        // =====================================================================

        // =====================================================================
        // Icon Rail (left side) - FIRST! Darkest shade
        // =====================================================================
        SidePanel::left("icon_rail")
            .resizable(false)
            .exact_width(42.0)
            .frame(Frame::none().fill(egui::Color32::from_rgb(22, 24, 30)))
            .show(ctx, |ui| {
                self.render_icon_rail(ui);
            });

        // =====================================================================
        // Waveform Panel (bottom) - rendered first to claim space for plots
        // =====================================================================
        if self.state.panels.waveform {
            TopBottomPanel::bottom("waveform")
                .resizable(true)
                .default_height(self.state.panel_sizes.waveform_height)
                .height_range(150.0..=600.0)
                .frame(
                    Frame::none()
                        .fill(egui::Color32::from_rgb(25, 27, 33))
                        .inner_margin(egui::Margin {
                            left: 0.0,
                            right: 0.0,
                            top: 8.0,
                            bottom: 4.0,
                        }),
                )
                .show(ctx, |ui| {
                    self.render_waveform_panel(ui);
                });
        }

        // =====================================================================
        // Console Panel (bottom) - spans full width except icon rail
        // =====================================================================
        if self.state.panels.console {
            TopBottomPanel::bottom("console")
                .resizable(true)
                .default_height(self.state.panel_sizes.console_height)
                .height_range(60.0..=400.0)
                .frame(Frame::none().fill(egui::Color32::from_rgb(25, 27, 33)))
                .show(ctx, |ui| {
                    self.render_console_panel(ui);
                });
        }

        // =====================================================================
        // Automation Console (bottom)
        // =====================================================================
        if self.state.panels.script_console {
            TopBottomPanel::bottom("script_console")
                .resizable(true)
                .default_height(200.0)
                .frame(Frame::none().fill(egui::Color32::from_rgb(20, 22, 28)))
                .show(ctx, |ui| {
                    crate::panels::render_script_console(
                        ui,
                        &mut self.state.script_console,
                        &mut self.state.simulation,
                    );

                    // Add close button to console area
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("x").clicked() {
                            self.state.panels.script_console = false;
                        }
                    });
                });
        }

        // =====================================================================
        // Project Browser (left, optional) - slightly lighter
        // =====================================================================
        if self.state.panels.project_browser {
            SidePanel::left("project_browser")
                .resizable(true)
                .default_width(self.state.panel_sizes.browser_width)
                .width_range(150.0..=400.0)
                .frame(
                    Frame::none()
                        .fill(egui::Color32::from_rgb(30, 33, 40))
                        .inner_margin(egui::Margin::same(8.0)),
                )
                .show(ctx, |ui| {
                    crate::panels::render_project_browser(ui, &mut self.state);
                });
        }

        // =====================================================================
        // Properties Panel (right) - matching side panel style
        // =====================================================================
        if self.state.panels.properties {
            SidePanel::right("properties")
                .resizable(true)
                .default_width(self.state.panel_sizes.properties_width)
                .width_range(180.0..=400.0)
                .frame(
                    Frame::none()
                        .fill(egui::Color32::from_rgb(30, 33, 40))
                        .inner_margin(egui::Margin::same(8.0)),
                )
                .show(ctx, |ui| {
                    crate::panels::render_properties_panel(ui, &mut self.state);
                });
        }

        // =====================================================================
        // Central Schematic Editor - darkest for the canvas
        // =====================================================================
        CentralPanel::default()
            .frame(Frame::none().fill(egui::Color32::from_rgb(18, 20, 24)))
            .show(ctx, |ui| {
                crate::schematic::view::render_schematic_view(
                    ui,
                    &mut self.state,
                    self.symbol_library.as_ref(),
                );
            });

        // =====================================================================
        // Modal Dialogs
        // =====================================================================

        // Component Properties Dialog
        {
            use crate::properties::dialog::{render_properties_dialog, PropertiesDialogResult};
            let result = render_properties_dialog(ctx, &mut self.state.property_editor);
            match result {
                PropertiesDialogResult::Apply(id, props) => {
                    // TODO: Create undo checkpoint before modifying (Phase 2: Undo/Redo)
                    // Update component in schematic
                    if let Some(comp) = self
                        .state
                        .schematic
                        .components
                        .iter_mut()
                        .find(|c| c.id == id)
                    {
                        comp.name = props.name;
                        comp.value = props.value;
                        // Note: model field can be used for params if needed
                    }
                    self.state.schematic.is_dirty = true;
                    self.state.schematic.bump_topology_version();
                }
                PropertiesDialogResult::Cancel => {
                    // Dialog was cancelled, nothing to do
                }
                PropertiesDialogResult::None => {
                    // Dialog still open or not shown
                }
            }
        }

        // Simulation Setup Dialog
        if self.state.dialogs.simulation_dialog {
            // Initialize defaults if empty
            if self.state.dialogs.tran_stop.is_empty() {
                // Transient
                self.state.dialogs.tran_stop = "1m".to_string();
                self.state.dialogs.tran_step = "10n".to_string();
                self.state.dialogs.tran_start = "0".to_string();
                self.state.dialogs.tran_maxstep = "auto".to_string();
                // AC
                self.state.dialogs.ac_fstart = "1".to_string();
                self.state.dialogs.ac_fstop = "1G".to_string();
                self.state.dialogs.ac_points = "101".to_string();
                // DC
                self.state.dialogs.dc_source = "V1".to_string();
                self.state.dialogs.dc_start = "0".to_string();
                self.state.dialogs.dc_stop = "5".to_string();
                self.state.dialogs.dc_step = "0.01".to_string();
                self.state.dialogs.dc_source2 = "V2".to_string();
                self.state.dialogs.dc_start2 = "0".to_string();
                self.state.dialogs.dc_stop2 = "3.3".to_string();
                self.state.dialogs.dc_step2 = "0.1".to_string();
                // Noise
                self.state.dialogs.noise_output = "out".to_string();
                self.state.dialogs.noise_ref = "0".to_string();
                self.state.dialogs.noise_input = "V1".to_string();
                self.state.dialogs.noise_fstart = "1".to_string();
                self.state.dialogs.noise_fstop = "100Meg".to_string();
                // Pole-Zero
                self.state.dialogs.pz_input = "in".to_string();
                self.state.dialogs.pz_output = "out".to_string();
                // Sensitivity
                self.state.dialogs.sens_output = "V(out)".to_string();
                // Monte Carlo
                self.state.dialogs.mc_runs = "100".to_string();
                self.state.dialogs.mc_seed = "0".to_string();
                // PSS
                self.state.dialogs.pss_fund = "1Meg".to_string();
                self.state.dialogs.pss_harmonics = "10".to_string();
                self.state.dialogs.pss_maxiter = "100".to_string();
                // STB
                self.state.dialogs.stb_probe = "istb".to_string();
                self.state.dialogs.stb_fstart = "1".to_string();
                self.state.dialogs.stb_fstop = "100Meg".to_string();
                // Temperature
                self.state.dialogs.temp_start = "-40".to_string();
                self.state.dialogs.temp_stop = "125".to_string();
                self.state.dialogs.temp_step = "25".to_string();
            }

            // Use egui::Window with proper layout that avoids height feedback loops
            // The key is using allocate_ui_with_layout with fixed heights for scroll areas
            egui::Window::new("Simulation Setup")
                .collapsible(false)
                .resizable(true)
                .default_width(700.0)
                .default_height(520.0)
                .min_width(600.0)
                .min_height(400.0)
                .show(ctx, |ui| {
                    // Calculate content height dynamically from available space
                    // Reserve ~60px for the bottom buttons section
                    // The scroll areas use auto_shrink([false, false]) so they won't cause feedback loops
                    let content_height = (ui.available_height() - 60.0).max(200.0);

                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            egui::vec2(180.0, content_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.label(egui::RichText::new("Analyses").strong());
                                ui.separator();

                                egui::ScrollArea::vertical()
                                    .id_salt("sim_list")
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        let item_width = ui.available_width();
                                        let item_height = 24.0;

                                        // Readable names, organized by category
                                        let analyses = [
                                            // Core Time/Freq Domain
                                            (2, "AC Analysis"),
                                            (3, "DC Sweep"),
                                            (0, "DC Operating Point"),
                                            (4, "Noise"),
                                            (1, "Transient"),
                                            // Steady-State
                                            (11, "Harmonic Balance (HB)"),
                                            (8, "PSS (Periodic Steady State)"),
                                            // Periodic Small-Signal
                                            (13, "PAC (Periodic AC)"),
                                            (14, "PNoise (Periodic Noise)"),
                                            (16, "PSTB (Periodic Stability)"),
                                            (15, "PXF (Periodic Transfer)"),
                                            // RF / S-Parameters
                                            (12, "S-Parameter"),
                                            // Transfer / Stability
                                            (5, "Pole-Zero"),
                                            (6, "Sensitivity"),
                                            (9, "Stability (STB)"),
                                            (17, "Transfer Function (XF)"),
                                            // Statistical / Sweep
                                            (18, "Corner Analysis"),
                                            (19, "Envelope Transient"),
                                            (20, "Fourier Analysis"),
                                            (7, "Monte Carlo"),
                                            (10, "Temperature Sweep"),
                                            // Advanced Analysis (Integrated)
                                            (21, "Reliability (Aging)"),
                                            (22, "Optimization Engine"),
                                            (23, "Safety (SOA)"),
                                        ];

                                        let selection_color = ui.visuals().selection.bg_fill;
                                        let hover_color = ui.visuals().widgets.hovered.bg_fill;
                                        let text_color = ui.visuals().text_color();

                                        for (idx, name) in analyses {
                                            let selected = self.state.dialogs.sim_active_tab == idx;

                                            // Allocate full-width clickable rect
                                            let (rect, response) = ui.allocate_exact_size(
                                                egui::vec2(item_width, item_height),
                                                egui::Sense::click(),
                                            );

                                            // Draw background
                                            if selected {
                                                ui.painter().rect_filled(
                                                    rect,
                                                    3.0,
                                                    selection_color,
                                                );
                                            } else if response.hovered() {
                                                ui.painter().rect_filled(rect, 3.0, hover_color);
                                            }

                                            // Draw left-aligned text
                                            ui.painter().text(
                                                rect.left_center() + egui::vec2(8.0, 0.0),
                                                egui::Align2::LEFT_CENTER,
                                                name,
                                                egui::FontId::proportional(13.0),
                                                text_color,
                                            );

                                            if response.clicked() {
                                                self.state.dialogs.sim_active_tab = idx;
                                            }
                                        }
                                    });
                            },
                        );

                        ui.separator();

                        // Right panel - Options for selected analysis
                        ui.allocate_ui_with_layout(
                            egui::vec2(480.0, content_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::ScrollArea::vertical()
                                    .id_salt("sim_opts")
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        self.render_analysis_options(ui);
                                    });
                            },
                        );
                    });

                    // Bottom buttons
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        let analysis_names = [
                            "DC OP",        // 0
                            "Transient",    // 1
                            "AC",           // 2
                            "DC Sweep",     // 3
                            "Noise",        // 4
                            "Pole-Zero",    // 5
                            "Sensitivity",  // 6
                            "Monte Carlo",  // 7
                            "PSS",          // 8
                            "STB",          // 9
                            "Temp Sweep",   // 10
                            "HB",           // 11
                            "S-Param",      // 12
                            "PAC",          // 13
                            "PNoise",       // 14
                            "PXF",          // 15
                            "PSTB",         // 16
                            "XF",           // 17
                            "Corner",       // 18
                            "Envelope",     // 19
                            "Fourier",      // 20
                            "Reliability",  // 21
                            "Optimization", // 22
                            "Safety/SOA",   // 23
                        ];
                        let tab = self
                            .state
                            .dialogs
                            .sim_active_tab
                            .min(analysis_names.len() - 1);

                        if ui
                            .button(format!("▶ Run {}", analysis_names[tab]))
                            .clicked()
                        {
                            self.state
                                .console_messages
                                .push(ConsoleMessage::info(format!(
                                    "Starting {} analysis...",
                                    analysis_names[tab]
                                )));
                            self.state.dialogs.simulation_dialog = false;
                        }
                        if ui.button("Apply").clicked() {
                            self.state
                                .console_messages
                                .push(ConsoleMessage::info("Simulation settings saved"));
                        }
                        if ui.button("Cancel").clicked() {
                            self.state.dialogs.simulation_dialog = false;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Help").clicked() {
                                self.state
                                    .console_messages
                                    .push(ConsoleMessage::info("Analysis help: See documentation"));
                            }
                        });
                    });
                });
        }

        // About Dialog
        if self.state.dialogs.about {
            egui::Window::new("About RSpice")
                .collapsible(false)
                .resizable(false)
                .default_width(300.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.heading("RSpice");
                    ui.label("Commercial-grade SPICE simulator");
                    ui.add_space(10.0);
                    ui.label("Version 0.1.0");
                    ui.add_space(20.0);
                    if ui.button("Close").clicked() {
                        self.state.dialogs.about = false;
                    }
                });
        }

        // Keyboard Shortcuts Help
        if self.state.dialogs.shortcuts_help {
            egui::Window::new("Keyboard Shortcuts")
                .collapsible(false)
                .resizable(true)
                .default_width(400.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    egui::Grid::new("shortcuts_grid")
                        .num_columns(2)
                        .spacing([40.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong("File");
                            ui.label("");
                            ui.end_row();
                            ui.label("Ctrl+N");
                            ui.label("New schematic");
                            ui.end_row();
                            ui.label("Ctrl+O");
                            ui.label("Open file");
                            ui.end_row();
                            ui.label("Ctrl+S");
                            ui.label("Save");
                            ui.end_row();

                            ui.strong("Edit");
                            ui.label("");
                            ui.end_row();
                            ui.label("Ctrl+Z");
                            ui.label("Undo");
                            ui.end_row();
                            ui.label("Ctrl+Y");
                            ui.label("Redo");
                            ui.end_row();
                            ui.label("Ctrl+C");
                            ui.label("Copy");
                            ui.end_row();
                            ui.label("Ctrl+V");
                            ui.label("Paste");
                            ui.end_row();
                            ui.label("Delete");
                            ui.label("Delete selection");
                            ui.end_row();
                        });
                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.state.dialogs.shortcuts_help = false;
                    }
                });
        }
    }

    /// Save state on exit
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
}

impl RSpiceApp {
    /// Render the left icon rail (VSCode style)
    fn render_icon_rail(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(4.0);

            // Project browser toggle
            let browser_active = self.state.panels.project_browser;
            if ui
                .add(
                    egui::Button::new("📁")
                        .min_size(Vec2::splat(32.0))
                        .fill(if browser_active {
                            self.state.theme.accent
                        } else {
                            egui::Color32::from_rgb(58, 62, 74)
                        }),
                )
                .on_hover_text("Library Browser (Ctrl+Shift+L)")
                .clicked()
            {
                self.toggle_panel_browser();
            }

            ui.add_space(4.0);

            // Spacer to push bottom items down
            ui.add_space(ui.available_height() - 80.0);

            // Console toggle
            let console_active = self.state.panels.console;
            if ui
                .add(
                    egui::Button::new("⌨")
                        .min_size(Vec2::splat(32.0))
                        .fill(if console_active {
                            self.state.theme.accent
                        } else {
                            egui::Color32::from_rgb(58, 62, 74)
                        }),
                )
                .on_hover_text("Toggle Console")
                .clicked()
            {
                self.toggle_panel_console();
            }

            ui.add_space(4.0);

            // Waveform toggle
            let has_waveforms = !self.state.simulation.waveforms.is_empty();
            let waveform_active = self.state.panels.waveform && has_waveforms;
            let waveform_btn =
                egui::Button::new("∿")
                    .min_size(Vec2::splat(32.0))
                    .fill(if waveform_active {
                        self.state.theme.accent
                    } else if has_waveforms {
                        egui::Color32::from_rgb(58, 62, 74)
                    } else {
                        egui::Color32::from_rgb(45, 48, 56)
                    });

            let response = ui.add_enabled(has_waveforms, waveform_btn);
            if has_waveforms {
                if response.on_hover_text("Toggle Waveform Viewer").clicked() {
                    self.toggle_panel_waveform();
                }
            } else {
                response.on_disabled_hover_text("No waveforms available");
            }

            ui.add_space(4.0);

            // Scripting console toggle
            let script_active = self.state.panels.script_console;
            if ui
                .add(
                    egui::Button::new("🐚") // Seashell/Console icon
                        .min_size(Vec2::splat(32.0))
                        .fill(if script_active {
                            self.state.theme.accent
                        } else {
                            egui::Color32::from_rgb(58, 62, 74)
                        }),
                )
                .on_hover_text("Automation Console")
                .clicked()
            {
                self.state.panels.script_console = !self.state.panels.script_console;
            }
        });
    }

    /// Render the console panel
    fn render_console_panel(&mut self, ui: &mut Ui) {
        // Top padding for header
        ui.add_space(6.0);

        // Header row with proper vertical centering
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 26.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Console")
                        .size(13.0)
                        .strong()
                        .color(egui::Color32::from_rgb(200, 200, 210)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);

                    // Close button - styled like Clear button
                    let close_btn = ui.add(
                        egui::Button::new(egui::RichText::new("x").size(11.0).strong())
                            .fill(egui::Color32::from_rgb(55, 60, 75))
                            .min_size(egui::vec2(28.0, 22.0)),
                    );
                    if close_btn.on_hover_text("Close Console").clicked() {
                        self.state.panels.console = false;
                    }

                    // ui.add_space(8.0);

                    // Clear button
                    let clear_btn = ui.add(
                        egui::Button::new(egui::RichText::new("Clear").size(11.0))
                            .fill(egui::Color32::from_rgb(55, 60, 75))
                            .min_size(egui::vec2(50.0, 22.0)),
                    );
                    if clear_btn.clicked() {
                        self.state.console_messages.clear();
                    }
                });
            },
        );

        // Custom separator line with no extra spacing
        let rect = ui.available_rect_before_wrap();
        let y = rect.top();
        ui.painter().hline(
            rect.left()..=rect.right(),
            y,
            egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 52, 58)),
        );
        ui.add_space(1.0); // Just account for the line we painted

        // Scrollable message area
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.add_space(4.0);
                for msg in &self.state.console_messages {
                    let color = match msg.level {
                        ConsoleLevel::Info => self.state.theme.text_primary,
                        ConsoleLevel::Warning => egui::Color32::from_rgb(255, 180, 50),
                        ConsoleLevel::Error => egui::Color32::from_rgb(255, 80, 80),
                    };
                    ui.colored_label(color, &msg.message);
                }
            });
    }

    /// Render the waveform panel
    fn render_waveform_panel(&mut self, ui: &mut Ui) {
        // Use the new commercial-grade waveform viewer
        crate::waveform::render_waveform_panel(ui, &mut self.state);
    }
}

// =============================================================================
// Serialization
// =============================================================================

impl serde::Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize minimal state needed for session recovery
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppState", 2)?;
        state.serialize_field("panels", &PanelVisibilitySer::from(&self.panels))?;
        state.serialize_field("panel_sizes", &PanelSizesSer::from(&self.panel_sizes))?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize minimal state and use defaults for the rest
        #[derive(serde::Deserialize)]
        struct AppStateDe {
            panels: PanelVisibilitySer,
            panel_sizes: PanelSizesSer,
        }

        let de = AppStateDe::deserialize(deserializer)?;
        Ok(Self {
            panels: de.panels.into(),
            panel_sizes: de.panel_sizes.into(),
            ..Default::default()
        })
    }
}

// Serialization helpers
#[derive(serde::Serialize, serde::Deserialize)]
struct PanelVisibilitySer {
    project_browser: bool,
    properties: bool,
    waveform: bool,
    console: bool,
    #[serde(default)]
    smith_chart: bool,
    #[serde(default)]
    signal_browser: bool,
    #[serde(default)]
    script_console: bool,
}

impl From<&PanelVisibility> for PanelVisibilitySer {
    fn from(p: &PanelVisibility) -> Self {
        Self {
            project_browser: p.project_browser,
            properties: p.properties,
            waveform: p.waveform,
            console: p.console,
            smith_chart: p.smith_chart,
            signal_browser: p.signal_browser,
            script_console: p.script_console,
        }
    }
}

impl From<PanelVisibilitySer> for PanelVisibility {
    fn from(s: PanelVisibilitySer) -> Self {
        Self {
            project_browser: s.project_browser,
            properties: s.properties,
            waveform: s.waveform,
            console: s.console,
            smith_chart: s.smith_chart,
            signal_browser: s.signal_browser,
            script_console: s.script_console,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PanelSizesSer {
    waveform_height: f32,
    console_height: f32,
    browser_width: f32,
    properties_width: f32,
}

impl From<&PanelSizes> for PanelSizesSer {
    fn from(p: &PanelSizes) -> Self {
        Self {
            waveform_height: p.waveform_height,
            console_height: p.console_height,
            browser_width: p.browser_width,
            properties_width: p.properties_width,
        }
    }
}

impl From<PanelSizesSer> for PanelSizes {
    fn from(s: PanelSizesSer) -> Self {
        Self {
            waveform_height: s.waveform_height,
            console_height: s.console_height,
            browser_width: s.browser_width,
            properties_width: s.properties_width,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
            state.panels.waveform,
            "Waveform should be visible by default"
        );
        assert!(state.panels.console, "Console should be visible by default");
    }

    #[test]
    fn test_panel_sizes_default() {
        let sizes = PanelSizes::default();
        assert_eq!(sizes.waveform_height, 200.0);
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
    }

    #[test]
    fn test_panel_visibility_serialization() {
        let panels = PanelVisibility {
            project_browser: true,
            properties: false,
            waveform: true,
            console: false,
            smith_chart: false,
            signal_browser: false,
            script_console: false,
        };
        let ser = PanelVisibilitySer::from(&panels);
        assert!(ser.project_browser);
        assert!(!ser.properties);

        let panels2: PanelVisibility = ser.into();
        assert!(panels2.project_browser);
        assert!(!panels2.properties);
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
}
