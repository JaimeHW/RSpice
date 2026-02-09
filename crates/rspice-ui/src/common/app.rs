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

use std::path::{Path, PathBuf};
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
/// Active tab in the unified bottom panel
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BottomPanelTab {
    /// Console output and messages
    #[default]
    Console,
    /// Waveform viewer / results
    Waveform,
    /// Log / history
    Log,
    /// Automation / scripting console
    Automation,
}

impl BottomPanelTab {
    /// Display name for tab
    pub fn name(&self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Waveform => "Waveform",
            Self::Log => "Log",
            Self::Automation => "Automation",
        }
    }

    /// All available tabs in display order
    pub fn all() -> &'static [BottomPanelTab] {
        &[Self::Console, Self::Waveform, Self::Log, Self::Automation]
    }
}

/// Panel visibility state
#[derive(Debug, Clone)]
pub struct PanelVisibility {
    /// Project browser (Library/Cell/View tree)
    pub project_browser: bool,
    /// Results browser (Simulation runs/analyses tree)
    pub results_browser: bool,
    /// Properties panel (right side)
    pub properties: bool,
    /// Unified bottom panel visible
    pub bottom_panel: bool,
    /// Active tab in bottom panel
    pub active_bottom_tab: BottomPanelTab,
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
            results_browser: false,
            properties: true,
            bottom_panel: true, // Visible by default with Console tab
            active_bottom_tab: BottomPanelTab::Console,
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

// =============================================================================
// Save Confirmation Dialog State
// =============================================================================

/// Actions that can trigger a save confirmation dialog
///
/// Commercial EDA tools like Cadence Virtuoso always prompt the user before
/// discarding unsaved work. This enum captures the pending action so it can
/// be executed after the user responds to the confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    /// Create new schematic (discard current)
    FileNew,
    /// Open another schematic (discard current)
    FileOpen,
    /// Close the application
    Exit,
}

impl ConfirmationAction {
    /// Get the dialog title for this action
    pub fn dialog_title(&self) -> &'static str {
        match self {
            ConfirmationAction::FileNew => "Create New Schematic",
            ConfirmationAction::FileOpen => "Open Schematic",
            ConfirmationAction::Exit => "Exit RSpice",
        }
    }

    /// Get the prompt message for this action
    pub fn prompt_message(&self) -> &'static str {
        "The current schematic has unsaved changes.\nDo you want to save before continuing?"
    }
}

/// State for the save confirmation dialog
///
/// When visible is true, the confirmation dialog is shown. The pending_action
/// field stores what should happen after the user responds.
#[derive(Debug, Clone, Default)]
pub struct ConfirmationDialogState {
    /// Whether the dialog is currently visible
    pub visible: bool,
    /// The action pending user confirmation
    pub pending_action: Option<ConfirmationAction>,
}

impl ConfirmationDialogState {
    /// Open the confirmation dialog for a specific action
    pub fn show(&mut self, action: ConfirmationAction) {
        self.visible = true;
        self.pending_action = Some(action);
    }

    /// Close the dialog and clear pending action
    pub fn close(&mut self) {
        self.visible = false;
        self.pending_action = None;
    }

    /// Check if dialog is open for a specific action
    pub fn is_showing(&self, action: ConfirmationAction) -> bool {
        self.visible && self.pending_action == Some(action)
    }
}

/// User response to a save confirmation dialog
///
/// Standard Yes/No/Cancel pattern matching commercial EDA tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationResponse {
    /// Save changes and proceed with action
    Yes,
    /// Discard changes and proceed with action
    No,
    /// Cancel the action, keep changes
    Cancel,
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
    /// New Cell target library
    pub new_cell_library: String,
    /// New Cell description
    pub new_cell_description: String,
    /// New Cell view types to create
    pub new_cell_create_schematic: bool,
    /// Create symbol view for new cell
    pub new_cell_create_symbol: bool,
    /// Create testbench view for new cell
    pub new_cell_create_testbench: bool,
    /// New Cell validation error message
    pub new_cell_error: Option<String>,
    /// New View creation dialog
    pub new_view_dialog: bool,
    /// New View target library
    pub new_view_library: String,
    /// New View target cell
    pub new_view_cell: String,
    /// New View name input
    pub new_view_name: String,
    /// New View type selection
    pub new_view_type: crate::state::ViewType,
    /// New View validation error message
    pub new_view_error: Option<String>,
    /// Active simulation tab (0=OP, 1=Tran, 2=AC, 3=DC, 4=Noise, 5=PZ, 6=Sens, 7=MC, 8=PSS, 9=STB, 10=Temp)
    pub sim_active_tab: usize,
    /// Set of enabled analysis indices
    pub enabled_analyses: std::collections::HashSet<usize>,
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
    // --- Reliability ---
    pub reliability_state: crate::simulation::dialog::reliability::ReliabilityDialogState,
    // --- Optimization ---
    pub optimization_state: crate::simulation::dialog::optimization::OptimizationDialogState,
    // --- Safety (SOA) ---
    pub soa_state: crate::simulation::dialog::soa::SoaDialogState,
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
    // --- Simulation Options ---
    /// Effective simulation options used for engine configuration.
    pub simulation_options_config: crate::simulation::dialog::SimulationOptions,
    /// Editable UI buffers for the simulation options dialog.
    pub simulation_options_state: crate::simulation::dialog::OptionsDialogState,
    /// Validation/parse errors shown in the simulation options dialog.
    pub simulation_options_errors: Vec<String>,

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

    // --- Verilog-A Model Loading ---
    /// Verilog-A model loading dialog state
    pub veriloga_dialog: crate::panels::VerilogALoadDialogState,

    // --- Runtime Interaction State ---
    /// Runtime interaction state for drag, hover, etc.
    pub interaction: crate::ui::dialog_state::InteractionState,

    // --- Save Confirmation Dialog ---
    /// State for save confirmation modal (unsaved changes warning)
    pub confirmation_dialog: ConfirmationDialogState,
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
    /// Structured log history buffer (ring-buffer, filterable).
    pub log_buffer: crate::panels::LogBuffer,
    /// UI state for the structured log panel.
    pub log_panel_state: crate::panels::LogPanelState,
    /// Number of console messages already mirrored into `log_buffer`.
    pub log_sync_cursor: usize,
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
    /// Eye diagram viewer state
    pub eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState,
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
            log_sync_cursor: 0,
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
            eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState::default(),
            smith_chart_state: crate::analysis::smith_chart::SmithChartState::default(),
            histogram_state: crate::analysis::histogram::HistogramState::default(),
        }
    }
}

const VERILOGA_LIBRARY_NAME: &str = "veriloga";
const VERILOGA_LIBRARY_CONFIG_FILE: &str = "veriloga_library.json";
const VERILOGA_LIBRARY_FORMAT_VERSION: u32 = 1;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedVerilogALibrary {
    version: u32,
    library: crate::state::Library,
}

fn global_veriloga_library_path() -> PathBuf {
    dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rspice")
        .join(VERILOGA_LIBRARY_CONFIG_FILE)
}

fn load_global_veriloga_library() -> Option<crate::state::Library> {
    let path = global_veriloga_library_path();
    if !path.exists() {
        return None;
    }

    let text = std::fs::read_to_string(&path).ok()?;
    let parsed: PersistedVerilogALibrary = serde_json::from_str(&text).ok()?;
    if parsed.version != VERILOGA_LIBRARY_FORMAT_VERSION {
        return None;
    }
    Some(parsed.library)
}

fn save_global_veriloga_library(
    library_manager: &crate::state::LibraryManager,
) -> Result<(), String> {
    let Some(library) = library_manager.get_library(VERILOGA_LIBRARY_NAME) else {
        return Ok(());
    };

    let path = global_veriloga_library_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let persisted = PersistedVerilogALibrary {
        version: VERILOGA_LIBRARY_FORMAT_VERSION,
        library: library.clone(),
    };
    let json = serde_json::to_string_pretty(&persisted).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

fn restore_global_veriloga_library(library_manager: &mut crate::state::LibraryManager) {
    let Some(mut library) = load_global_veriloga_library() else {
        return;
    };
    library.name = VERILOGA_LIBRARY_NAME.to_string();
    library.read_only = false;
    library_manager.add_library(library);
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

fn apply_component_property_edits(
    state: &mut AppState,
    id: u64,
    props: crate::properties::dialog::EditedProperties,
) -> bool {
    let (old_name, changed) = match state.schematic.components.iter().find(|c| c.id == id) {
        Some(comp) => (
            comp.name.clone(),
            comp.name != props.name || comp.value != props.value,
        ),
        None => return false,
    };
    if !changed {
        return false;
    }

    let description = if old_name.is_empty() {
        format!("Edit properties for component {}", id)
    } else {
        format!("Edit properties for {}", old_name)
    };
    let new_name = props.name;
    let new_value = props.value;

    state.schematic.with_undo(description, move |schematic| {
        if let Some(comp) = schematic.components.iter_mut().find(|c| c.id == id) {
            comp.name = new_name;
            comp.value = new_value;
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        }
    })
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
            self.toggle_panel_console_new();
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
            // Resistor (Shift+R)
            if ctx.input(|i| i.key_pressed(Key::R) && i.modifiers.shift && !i.modifiers.ctrl) {
                self.state.schematic.tool = Tool::Place(ComponentType::Resistor);
            }
            // Rotate preview/selection (R key without shift)
            if ctx.input(|i| i.key_pressed(Key::R) && !i.modifiers.ctrl && !i.modifiers.shift) {
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

    /// Request a new schematic (prompts to save if dirty)
    fn action_file_new(&mut self) {
        if self.state.schematic.is_dirty {
            // Show save confirmation dialog - don't discard unsaved changes
            self.state
                .dialogs
                .confirmation_dialog
                .show(ConfirmationAction::FileNew);
        } else {
            self.do_file_new();
        }
    }

    /// Internal: Actually create a new schematic (after confirmation)
    fn do_file_new(&mut self) {
        self.state.schematic = SchematicState::default();
        self.state
            .console_messages
            .push(ConsoleMessage::info("Created new schematic"));
    }

    /// Request to open a schematic (prompts to save if dirty)
    fn action_file_open(&mut self) {
        if self.state.schematic.is_dirty {
            // Show save confirmation dialog before opening
            self.state
                .dialogs
                .confirmation_dialog
                .show(ConfirmationAction::FileOpen);
        } else {
            self.do_file_open();
        }
    }

    /// Internal: Actually open a schematic (after confirmation)
    fn do_file_open(&mut self) {
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

    /// Handle user response to save confirmation dialog
    ///
    /// This is called when the user clicks Yes, No, or Cancel in the
    /// save confirmation dialog. Commercial EDA pattern:
    /// - Yes: Save first, then execute pending action
    /// - No: Discard changes and execute pending action
    /// - Cancel: Close dialog, do nothing
    fn handle_confirmation_response(&mut self, response: ConfirmationResponse) {
        let pending = self.state.dialogs.confirmation_dialog.pending_action;
        self.state.dialogs.confirmation_dialog.close();

        match response {
            ConfirmationResponse::Cancel => {
                // User cancelled - do nothing
            }
            ConfirmationResponse::No => {
                // Discard changes and proceed
                if let Some(action) = pending {
                    self.execute_pending_action(action);
                }
            }
            ConfirmationResponse::Yes => {
                // Save first, then proceed
                self.action_file_save();
                if let Some(action) = pending {
                    self.execute_pending_action(action);
                }
            }
        }
    }

    /// Execute a pending action after confirmation dialog
    fn execute_pending_action(&mut self, action: ConfirmationAction) {
        match action {
            ConfirmationAction::FileNew => self.do_file_new(),
            ConfirmationAction::FileOpen => self.do_file_open(),
            ConfirmationAction::Exit => {
                // Signal exit request - this will be handled by the frame update
                self.state.exit_requested = true;
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
        // Close results browser when opening library browser (mutually exclusive)
        if !self.state.panels.project_browser {
            self.state.panels.results_browser = false;
        }
        self.state.panels.project_browser = !self.state.panels.project_browser;
    }

    fn toggle_panel_console(&mut self) {
        // Show bottom panel and switch to Console tab
        if self.state.panels.bottom_panel
            && self.state.panels.active_bottom_tab == BottomPanelTab::Console
        {
            self.state.panels.bottom_panel = false;
        } else {
            self.state.panels.bottom_panel = true;
            self.state.panels.active_bottom_tab = BottomPanelTab::Console;
        }
    }

    fn toggle_panel_waveform(&mut self) {
        // Show bottom panel and switch to Waveform tab
        if self.state.panels.bottom_panel
            && self.state.panels.active_bottom_tab == BottomPanelTab::Waveform
        {
            self.state.panels.bottom_panel = false;
        } else {
            self.state.panels.bottom_panel = true;
            self.state.panels.active_bottom_tab = BottomPanelTab::Waveform;
        }
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
                self.state.dialogs.reliability_state.render(ui);
            }
            22 => {
                self.state.dialogs.optimization_state.render(ui);
            }
            23 => {
                self.state.dialogs.soa_state.render(ui);
            }
            _ => {}
        }
    }

    /// Render the global simulation options dialog.
    ///
    /// This dialog mirrors commercial SPICE options editors:
    /// - tabbed organization by option category
    /// - strict parsing and validation on apply
    /// - quick presets for common workflows
    /// - live SPICE `.OPTIONS` preview
    fn render_simulation_options_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.simulation_options {
            return;
        }

        let mut dialog_open = self.state.dialogs.simulation_options;
        let mut close_requested = false;

        egui::Window::new("Simulation Options")
            .open(&mut dialog_open)
            .collapsible(false)
            .resizable(true)
            .default_width(760.0)
            .default_height(580.0)
            .min_width(680.0)
            .min_height(320.0)
            .show(ctx, |ui| {
                ui.label("Configure numerical accuracy, convergence, and algorithm behavior.");
                ui.label(egui::RichText::new("Changes apply to all new simulation runs.").weak());
                ui.add_space(6.0);

                ui.horizontal_wrapped(|ui| {
                    let tabs = [
                        "Accuracy",
                        "Convergence",
                        "Algorithm",
                        "Limits",
                        "Temperature",
                    ];
                    for (idx, tab_name) in tabs.iter().enumerate() {
                        let selected =
                            self.state.dialogs.simulation_options_state.active_tab == idx;
                        if ui.selectable_label(selected, *tab_name).clicked() {
                            self.state.dialogs.simulation_options_state.active_tab = idx;
                        }
                    }
                });

                ui.separator();
                ui.add_space(4.0);

                let preview_text = match self.state.dialogs.simulation_options_state.to_options() {
                    Ok(opts) => opts.to_spice_options(),
                    Err(_) => self
                        .state
                        .dialogs
                        .simulation_options_config
                        .to_spice_options(),
                };
                // Keep preview fixed at bottom, with a dedicated reserved region.
                let preview_height = 170.0;
                let footer_height = 60.0;
                let content_height =
                    (ui.available_height() - preview_height - footer_height).max(0.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), content_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("sim_options_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                match self.state.dialogs.simulation_options_state.active_tab {
                                    0 => {
                                        ui.heading("Accuracy");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_accuracy_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.reltol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("RESIDUAL_RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.residual_reltol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("ABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.abstol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("VNTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.vntol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("IABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.iabstol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("CHGTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.chgtol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    1 => {
                                        ui.heading("Convergence");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_convergence_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("ITL1");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.itl1)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("ITL4");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.itl4)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("GMIN");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.gmin)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.gmin_stepping,
                                            "Enable GMIN stepping",
                                        );
                                        ui.checkbox(
                                            &mut opts.source_stepping,
                                            "Enable source stepping",
                                        );
                                        ui.checkbox(
                                            &mut opts.pseudo_transient,
                                            "Enable pseudo-transient continuation",
                                        );

                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label("Damping strategy");
                                            let damping_options =
                                                crate::simulation::dialog::DampingStrategy::all();
                                            let selected = damping_options
                                                [opts.damping.min(damping_options.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_damping")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, damping) in
                                                        damping_options.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.damping,
                                                            idx,
                                                            damping.display_name(),
                                                        );
                                                    }
                                                });
                                        });
                                    }
                                    2 => {
                                        ui.heading("Algorithm");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;

                                        ui.horizontal(|ui| {
                                            ui.label("Integration method");
                                            let methods =
                                                crate::simulation::dialog::IntegrationMethod::all();
                                            let selected = methods
                                                [opts.method.min(methods.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_method")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, method) in methods.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.method,
                                                            idx,
                                                            method.display_name(),
                                                        );
                                                    }
                                                });
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Matrix solver");
                                            let solvers =
                                                crate::simulation::dialog::MatrixSolver::all();
                                            let selected = solvers
                                                [opts.solver.min(solvers.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_solver")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, solver) in solvers.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.solver,
                                                            idx,
                                                            solver.display_name(),
                                                        );
                                                    }
                                                });
                                        });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.bypass_enabled,
                                            "Enable model bypass",
                                        );
                                        egui::Grid::new("sim_opts_bypass_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("Bypass RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.bypass_reltol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Bypass ABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.bypass_abstol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    3 => {
                                        ui.heading("Limits");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_limits_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("Minimum timestep");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.min_timestep,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Maximum timestep");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.max_timestep,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Timestep reduction factor");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.timestep_factor,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    4 => {
                                        ui.heading("Temperature and Output");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_temperature_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("TEMP (C)");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.temp)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("TNOM (C)");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.tnom)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.verbose,
                                            "Enable verbose convergence diagnostics",
                                        );
                                        ui.checkbox(
                                            &mut opts.save_internals,
                                            "Save internal node/branch data when supported",
                                        );
                                    }
                                    _ => {}
                                }

                                if !self.state.dialogs.simulation_options_errors.is_empty() {
                                    ui.add_space(8.0);
                                    ui.group(|ui| {
                                        ui.label(
                                            egui::RichText::new("Validation issues")
                                                .color(Color32::from_rgb(255, 120, 120))
                                                .strong(),
                                        );
                                        for error in &self.state.dialogs.simulation_options_errors {
                                            ui.label(
                                                egui::RichText::new(format!("- {}", error))
                                                    .color(Color32::from_rgb(255, 120, 120)),
                                            );
                                        }
                                    });
                                }
                            });
                    },
                );

                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("SPICE .OPTIONS preview").strong());
                    ui.add_space(4.0);
                    let mut preview_buf = preview_text;
                    ui.add_enabled(
                        false,
                        egui::TextEdit::multiline(&mut preview_buf)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(6)
                            .desired_width(f32::INFINITY),
                    );
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Default").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::default();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Fast").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::fast();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Accurate").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::accurate();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Robust").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::robust();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Revert").clicked() {
                        let current = self.state.dialogs.simulation_options_config.clone();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&current);
                        self.state.dialogs.simulation_options_errors.clear();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ok_clicked = ui.button("OK").clicked();
                        let apply_clicked = ui.button("Apply").clicked();
                        if ui.button("Cancel").clicked() {
                            close_requested = true;
                        }

                        if apply_clicked || ok_clicked {
                            let parsed = self.state.dialogs.simulation_options_state.to_options();
                            let validated = match parsed {
                                Ok(opts) => match opts.validate() {
                                    Ok(()) => Some(opts),
                                    Err(validation_errors) => {
                                        self.state.dialogs.simulation_options_errors =
                                            validation_errors
                                                .into_iter()
                                                .map(|err| err.to_string())
                                                .collect();
                                        None
                                    }
                                },
                                Err(parse_errors) => {
                                    self.state.dialogs.simulation_options_errors = parse_errors;
                                    None
                                }
                            };

                            if let Some(opts) = validated {
                                self.state.dialogs.simulation_options_config = opts.clone();
                                self.state.dialogs.simulation_options_state =
                                    crate::simulation::dialog::OptionsDialogState::from_options(
                                        &opts,
                                    );
                                self.state.dialogs.simulation_options_errors.clear();
                                self.state
                                    .console_messages
                                    .push(ConsoleMessage::info("Simulation options updated"));
                                if ok_clicked {
                                    close_requested = true;
                                }
                            }
                        }
                    });
                });
            });

        self.state.dialogs.simulation_options = dialog_open && !close_requested;
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
        // Unified Bottom Panel (tabbed: Console, Waveform, Log)
        // =====================================================================
        if self.state.panels.bottom_panel {
            TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .default_height(self.state.panel_sizes.waveform_height)
                .height_range(100.0..=500.0)
                .frame(Frame::none().fill(egui::Color32::from_rgb(25, 27, 33)))
                .show(ctx, |ui| {
                    // Professional tab bar with underline-style selection
                    // Remove vertical spacing to eliminate gap below tabs
                    ui.spacing_mut().item_spacing.y = 0.0;
                    // Tab bar styling constants
                    let accent_color = egui::Color32::from_rgb(100, 150, 255);
                    let inactive_text = egui::Color32::from_rgb(160, 165, 175);
                    let active_text = egui::Color32::WHITE;
                    let hover_text = egui::Color32::from_rgb(200, 205, 215);
                    let tab_height = 28.0; // Full bar height
                    let tab_min_width = 80.0;
                    let tab_padding = 16.0;
                    let underline_height = 2.0;

                    ui.horizontal(|ui| {
                        // Remove gaps between tabs
                        ui.spacing_mut().item_spacing.x = 0.0;

                        for &tab in BottomPanelTab::all() {
                            let selected = self.state.panels.active_bottom_tab == tab;

                            // Calculate tab size
                            let text = tab.name();
                            let galley = ui.fonts(|f| {
                                f.layout_no_wrap(
                                    text.to_string(),
                                    egui::FontId::proportional(12.0),
                                    active_text,
                                )
                            });
                            let text_width = galley.rect.width();
                            let tab_width = (text_width + tab_padding * 2.0).max(tab_min_width);

                            // Allocate space for the tab
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(tab_width, tab_height),
                                egui::Sense::click(),
                            );

                            // Determine visual state
                            let text_color = if selected {
                                active_text
                            } else if response.hovered() {
                                hover_text
                            } else {
                                inactive_text
                            };

                            // Draw the tab
                            let painter = ui.painter();

                            // Hover background (subtle)
                            if response.hovered() && !selected {
                                painter.rect_filled(
                                    rect,
                                    egui::Rounding::ZERO,
                                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8),
                                );
                            }

                            // Tab text (centered)
                            painter.text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                text,
                                egui::FontId::proportional(12.0),
                                text_color,
                            );

                            // Selected indicator (underline)
                            if selected {
                                let underline_rect = egui::Rect::from_min_size(
                                    egui::pos2(rect.min.x, rect.max.y - underline_height),
                                    egui::vec2(rect.width(), underline_height),
                                );
                                painter.rect_filled(
                                    underline_rect,
                                    egui::Rounding::ZERO,
                                    accent_color,
                                );
                            }

                            // Handle click
                            if response.clicked() {
                                self.state.panels.active_bottom_tab = tab;
                            }
                        }

                        // Right-aligned close button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Close button (square, minimal style)
                            let close_size = egui::vec2(tab_height, tab_height);
                            let (close_rect, close_response) =
                                ui.allocate_exact_size(close_size, egui::Sense::click());

                            let close_color = if close_response.hovered() {
                                egui::Color32::from_rgb(255, 100, 100)
                            } else {
                                inactive_text
                            };

                            // Draw hover background for close button
                            if close_response.hovered() {
                                ui.painter().rect_filled(
                                    close_rect,
                                    egui::Rounding::same(2.0),
                                    egui::Color32::from_rgba_unmultiplied(255, 80, 80, 30),
                                );
                            }

                            // Draw X using lines for a clean look
                            let center = close_rect.center();
                            let cross_size = 4.0;
                            let stroke = egui::Stroke::new(1.5, close_color);
                            ui.painter().line_segment(
                                [
                                    center + egui::vec2(-cross_size, -cross_size),
                                    center + egui::vec2(cross_size, cross_size),
                                ],
                                stroke,
                            );
                            ui.painter().line_segment(
                                [
                                    center + egui::vec2(cross_size, -cross_size),
                                    center + egui::vec2(-cross_size, cross_size),
                                ],
                                stroke,
                            );

                            if close_response.on_hover_text("Close panel").clicked() {
                                self.state.panels.bottom_panel = false;
                            }
                        });
                    });

                    // Subtle separator line (painted directly, no spacing)
                    let separator_rect = ui.available_rect_before_wrap();
                    ui.painter().hline(
                        separator_rect.x_range(),
                        separator_rect.top(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 54, 62)),
                    );

                    // Render active tab content
                    match self.state.panels.active_bottom_tab {
                        BottomPanelTab::Console => self.render_console_panel(ui),
                        BottomPanelTab::Waveform => self.render_waveform_panel(ui),
                        BottomPanelTab::Log => self.render_log_panel(ui),
                        BottomPanelTab::Automation => self.render_automation_panel(ui),
                    }
                });
        }

        // =====================================================================
        // Project Browser (left, optional) - slightly lighter
        // Mutually exclusive with Results Browser (Results takes precedence)
        // =====================================================================
        if self.state.panels.project_browser && !self.state.panels.results_browser {
            SidePanel::left("left_browser")
                .resizable(true)
                .default_width(self.state.panel_sizes.browser_width)
                .width_range(150.0..=400.0)
                .frame(
                    Frame::none()
                        .fill(egui::Color32::from_rgb(30, 33, 40))
                        .inner_margin(egui::Margin::same(8.0)),
                )
                .show(ctx, |ui| {
                    // Header row matching Properties panel style (without close button)
                    let header_height = 16.0;
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), header_height),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.label(
                                egui::RichText::new("Library Browser")
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(180, 180, 190)),
                            );
                        },
                    );

                    // Full-width separator line
                    let separator_rect = ui.available_rect_before_wrap();
                    let panel_left = separator_rect.left() - 8.0;
                    let panel_right = separator_rect.right() + 8.0;
                    ui.painter().hline(
                        panel_left..=panel_right,
                        separator_rect.top(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 54, 62)),
                    );

                    ui.add_space(8.0);

                    crate::panels::render_project_browser(ui, &mut self.state);
                });
        }

        // =====================================================================
        // Results Browser (left, optional) - for simulation results navigation
        // =====================================================================
        if self.state.panels.results_browser {
            SidePanel::left("left_browser")
                .resizable(true)
                .default_width(self.state.panel_sizes.browser_width)
                .width_range(150.0..=400.0)
                .frame(
                    Frame::none()
                        .fill(egui::Color32::from_rgb(30, 33, 40))
                        .inner_margin(egui::Margin::same(8.0)),
                )
                .show(ctx, |ui| {
                    // Header row matching Properties panel style (without close button)
                    let header_height = 16.0;
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), header_height),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.label(
                                egui::RichText::new("Simulation Results")
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(180, 180, 190)),
                            );
                        },
                    );

                    // Full-width separator line
                    let separator_rect = ui.available_rect_before_wrap();
                    let panel_left = separator_rect.left() - 8.0;
                    let panel_right = separator_rect.right() + 8.0;
                    ui.painter().hline(
                        panel_left..=panel_right,
                        separator_rect.top(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 54, 62)),
                    );

                    ui.add_space(8.0);

                    crate::panels::render_results_browser(ui, &mut self.state);
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
                    // Header row with title and close button (matching tab bar style)
                    let header_height = 16.0;
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), header_height),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.label(
                                egui::RichText::new("Properties")
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(180, 180, 190)),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    // Close button (square, matching tab bar style)
                                    let close_size = egui::vec2(18.0, 18.0);
                                    let (close_rect, close_response) =
                                        ui.allocate_exact_size(close_size, egui::Sense::click());

                                    let inactive_text = egui::Color32::from_rgb(120, 125, 135);
                                    let close_color = if close_response.hovered() {
                                        egui::Color32::from_rgb(255, 100, 100)
                                    } else {
                                        inactive_text
                                    };

                                    // Draw hover background
                                    if close_response.hovered() {
                                        ui.painter().rect_filled(
                                            close_rect,
                                            egui::Rounding::same(2.0),
                                            egui::Color32::from_rgba_unmultiplied(255, 80, 80, 30),
                                        );
                                    }

                                    // Draw X using lines
                                    let center = close_rect.center();
                                    let cross_size = 4.0;
                                    let stroke = egui::Stroke::new(1.5, close_color);
                                    ui.painter().line_segment(
                                        [
                                            center + egui::vec2(-cross_size, -cross_size),
                                            center + egui::vec2(cross_size, cross_size),
                                        ],
                                        stroke,
                                    );
                                    ui.painter().line_segment(
                                        [
                                            center + egui::vec2(cross_size, -cross_size),
                                            center + egui::vec2(-cross_size, cross_size),
                                        ],
                                        stroke,
                                    );

                                    if close_response.on_hover_text("Close panel").clicked() {
                                        self.state.panels.properties = false;
                                    }
                                },
                            );
                        },
                    );

                    // Full-width separator line (extends past inner margin)
                    let separator_rect = ui.available_rect_before_wrap();
                    let panel_left = separator_rect.left() - 8.0; // Extend past left margin
                    let panel_right = separator_rect.right() + 8.0; // Extend past right margin
                    ui.painter().hline(
                        panel_left..=panel_right,
                        separator_rect.top(),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 54, 62)),
                    );

                    // Add spacing after header
                    ui.add_space(8.0);

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

        // Save Confirmation Dialog (unsaved changes warning)
        // Matches commercial EDA pattern: Yes = Save and proceed, No = Discard, Cancel = Abort
        if self.state.dialogs.confirmation_dialog.visible {
            let mut response: Option<ConfirmationResponse> = None;

            egui::Area::new(egui::Id::new("save_confirmation_backdrop"))
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    // Semi-transparent backdrop to prevent interaction with main UI
                    let screen = ui.ctx().screen_rect();
                    ui.painter().rect_filled(
                        screen,
                        egui::Rounding::ZERO,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                    );
                });

            egui::Window::new("Save Changes?")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    ui.set_min_width(350.0);

                    // Icon and message
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("⚠")
                                .size(28.0)
                                .color(egui::Color32::from_rgb(255, 191, 0)),
                        );
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            if let Some(action) =
                                &self.state.dialogs.confirmation_dialog.pending_action
                            {
                                ui.label(RichText::new(action.dialog_title()).size(14.0).strong());
                            }
                            ui.add_space(4.0);
                            ui.label("The current schematic has unsaved changes.");
                            ui.label("Do you want to save before continuing?");
                        });
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Button row: Yes | No | Cancel (commercial pattern)
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Cancel button (rightmost)
                            if ui.button("Cancel").clicked() {
                                response = Some(ConfirmationResponse::Cancel);
                            }

                            ui.add_space(8.0);

                            // No button
                            if ui.button("Don't Save").clicked() {
                                response = Some(ConfirmationResponse::No);
                            }

                            ui.add_space(8.0);

                            // Yes button (primary action)
                            if ui
                                .add(
                                    egui::Button::new(RichText::new("Save").strong())
                                        .fill(egui::Color32::from_rgb(59, 130, 246)),
                                )
                                .clicked()
                            {
                                response = Some(ConfirmationResponse::Yes);
                            }
                        });
                    });
                });

            // Handle the response after dialog rendering
            if let Some(r) = response {
                self.handle_confirmation_response(r);
            }
        }

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

        // Verilog-A Model Loading Dialog
        let veriloga_result = crate::panels::render_veriloga_load_dialog(
            ctx,
            &mut self.state.dialogs.veriloga_dialog,
        );
        if veriloga_result == crate::panels::VerilogADialogResult::AddToLibrary {
            if let Some(module) = self.state.dialogs.veriloga_dialog.compiled_module.clone() {
                let compiled_artifact =
                    self.state.dialogs.veriloga_dialog.compiled_artifact.clone();
                let compiled_dependencies = self
                    .state
                    .dialogs
                    .veriloga_dialog
                    .compiled_dependencies
                    .clone()
                    .unwrap_or_default();
                let source_path_text = module.source_path.to_string_lossy().to_string();
                let serialized_ports =
                    serde_json::to_string(&module.ports).unwrap_or_else(|_| module.ports.join(","));

                // Create or get the global user Verilog-A library.
                if self
                    .state
                    .library_manager
                    .get_library(VERILOGA_LIBRARY_NAME)
                    .is_none()
                {
                    let mut lib =
                        crate::state::library_browser::Library::new(VERILOGA_LIBRARY_NAME);
                    lib.read_only = false;
                    self.state.library_manager.add_library(lib);
                }

                // Upsert cell/view metadata for robust placement + netlisting.
                let mut add_ok = false;
                if let Some(lib) = self
                    .state
                    .library_manager
                    .get_library_mut(VERILOGA_LIBRARY_NAME)
                {
                    let mut cell = lib
                        .get_cell(&module.name)
                        .cloned()
                        .unwrap_or_else(|| crate::state::library_browser::Cell::new(&module.name));
                    cell.description = format!(
                        "Verilog-A model with {} terminals: {}",
                        module.ports.len(),
                        module.ports.join(", ")
                    );
                    cell.category = "Verilog-A".to_string();
                    cell.metadata
                        .insert("veriloga.module".to_string(), module.name.clone());
                    cell.metadata
                        .insert("veriloga.source_path".to_string(), source_path_text.clone());
                    cell.metadata
                        .insert("veriloga.ports".to_string(), serialized_ports.clone());

                    // Store parameter defaults in cell metadata for property/editor binding.
                    for param in &module.parameters {
                        cell.metadata
                            .insert(format!("param_{}", param.name), param.default_value.clone());
                    }

                    let mut view = cell.get_view("veriloga").cloned().unwrap_or_else(|| {
                        crate::state::library_browser::View::new(
                            "veriloga",
                            crate::state::library_browser::ViewType::VerilogA,
                        )
                    });
                    view.view_type = crate::state::library_browser::ViewType::VerilogA;
                    view.file_path = Some(module.source_path.clone());
                    view.metadata
                        .insert("veriloga.module".to_string(), module.name.clone());
                    view.metadata
                        .insert("veriloga.source_path".to_string(), source_path_text.clone());
                    view.metadata
                        .insert("veriloga.ports".to_string(), serialized_ports);
                    cell.add_view(view);

                    lib.add_cell(cell);
                    add_ok = true;
                }

                if !add_ok {
                    self.state
                        .console_messages
                        .push(ConsoleMessage::error(format!(
                            "Failed to add Verilog-A model '{}' to library '{}'",
                            module.name, VERILOGA_LIBRARY_NAME
                        )));
                } else {
                    log::info!(
                        "Registered Verilog-A model '{}' with {} terminals and {} parameters",
                        module.name,
                        module.ports.len(),
                        module.parameters.len()
                    );
                    self.state
                        .console_messages
                        .push(ConsoleMessage::info(format!(
                            "Verilog-A model '{}' added to library with terminals: {}",
                            module.name,
                            module.ports.join(", ")
                        )));
                }

                if let Some(compiled_model) = compiled_artifact {
                    match rspice_core::register_precompiled_veriloga_model_with_dependencies(
                        &module.source_path,
                        &compiled_dependencies,
                        compiled_model,
                    ) {
                        Ok(()) => {
                            self.state
                                .console_messages
                                .push(ConsoleMessage::info(format!(
                                    "Registered Verilog-A compile cache for '{}' ({} dependency file(s))",
                                    module.name,
                                    compiled_dependencies.len()
                                )));
                        }
                        Err(err) => {
                            self.state
                                .console_messages
                                .push(ConsoleMessage::warning(format!(
                                    "Verilog-A compile cache registration failed for '{}': {}",
                                    module.name, err
                                )));
                        }
                    }
                } else {
                    self.state.console_messages.push(ConsoleMessage::warning(format!(
                        "No compiled Verilog-A artifact available for '{}'; simulation will recompile if needed",
                        module.name
                    )));
                }

                if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
                    log::warn!("Failed to persist global Verilog-A library: {}", err);
                    self.state
                        .console_messages
                        .push(ConsoleMessage::warning(format!(
                            "Failed to persist Verilog-A library: {}",
                            err
                        )));
                }
            } else {
                self.state.console_messages.push(ConsoleMessage::warning(
                    "No compiled Verilog-A module available to add".to_string(),
                ));
            }
        }

        // Property Dialog (commercial-grade tabbed property editor)
        crate::panels::render_property_dialog(ctx, &mut self.state);

        // PDK Settings Dialog
        {
            let result =
                crate::panels::render_pdk_settings_dialog(ctx, &mut self.state.pdk_settings_dialog);
            match result {
                crate::panels::PdkSettingsDialogResult::Applied(config) => {
                    // Load models from the updated PDK configuration
                    match self
                        .state
                        .model_library_manager
                        .load_from_pdk_config(&config)
                    {
                        Ok(count) => {
                            self.state.pdk_config = config;
                            let _ = self.state.pdk_config.save();
                            self.state
                                .console_messages
                                .push(ConsoleMessage::info(format!(
                                    "PDK settings applied: {} libraries loaded",
                                    count
                                )));
                        }
                        Err(errors) => {
                            self.state.pdk_config = config;
                            let _ = self.state.pdk_config.save();
                            self.state
                                .console_messages
                                .push(ConsoleMessage::warning(format!(
                                    "PDK settings applied with {} errors",
                                    errors.len()
                                )));
                            for err in errors {
                                self.state.console_messages.push(ConsoleMessage::error(err));
                            }
                        }
                    }
                }
                crate::panels::PdkSettingsDialogResult::LoadFile(path) => {
                    // Load a single model file
                    match self
                        .state
                        .model_library_manager
                        .load_library_file(&path, None)
                    {
                        Ok(lib_name) => {
                            // Track in recent files
                            self.state.pdk_config.add_recent_file(&path);
                            let _ = self.state.pdk_config.save();

                            self.state
                                .console_messages
                                .push(ConsoleMessage::info(format!(
                                    "Loaded library '{}' from {}",
                                    lib_name,
                                    path.display()
                                )));

                            // Log model count
                            if let Some(lib) =
                                self.state.model_library_manager.get_library(&lib_name)
                            {
                                self.state
                                    .console_messages
                                    .push(ConsoleMessage::info(format!(
                                        "  {} models, {} corners available",
                                        lib.model_count(),
                                        lib.corner_count()
                                    )));
                            }
                        }
                        Err(err) => {
                            self.state
                                .console_messages
                                .push(ConsoleMessage::error(format!(
                                    "Failed to load {}: {}",
                                    path.display(),
                                    err
                                )));
                        }
                    }
                }
                crate::panels::PdkSettingsDialogResult::Cancelled => {
                    // User cancelled - no action needed
                }
                crate::panels::PdkSettingsDialogResult::None => {
                    // Dialog still open, no action
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
            // Note: We extract dialog_open to avoid borrow conflict with .open() and closure
            let mut dialog_open = self.state.dialogs.simulation_dialog;
            egui::Window::new("Simulation Setup")
                .open(&mut dialog_open)
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
                                        let item_width = ui.available_width() - 4.0;
                                        let item_height = 28.0;
                                        let header_height = 26.0;

                                        // Organized analysis categories
                                        let categories: &[(&str, &[(usize, &str)])] = &[
                                            (
                                                "Time & Frequency Domain",
                                                &[
                                                    (1, "Transient"),
                                                    (2, "AC Analysis"),
                                                    (3, "DC Sweep"),
                                                    (0, "DC Operating Point"),
                                                    (4, "Noise"),
                                                ],
                                            ),
                                            (
                                                "Steady-State",
                                                &[(8, "PSS (Periodic)"), (11, "Harmonic Balance")],
                                            ),
                                            (
                                                "Periodic Small-Signal",
                                                &[
                                                    (13, "PAC"),
                                                    (14, "PNoise"),
                                                    (15, "PXF"),
                                                    (16, "PSTB"),
                                                ],
                                            ),
                                            (
                                                "Transfer & Stability",
                                                &[
                                                    (5, "Pole-Zero"),
                                                    (6, "Sensitivity"),
                                                    (9, "Stability (STB)"),
                                                    (17, "Transfer Func (XF)"),
                                                ],
                                            ),
                                            ("RF & S-Parameters", &[(12, "S-Parameter")]),
                                            (
                                                "Statistical & Sweep",
                                                &[
                                                    (7, "Monte Carlo"),
                                                    (10, "Temperature"),
                                                    (18, "Corner"),
                                                    (19, "Envelope"),
                                                    (20, "Fourier"),
                                                ],
                                            ),
                                            (
                                                "Advanced",
                                                &[
                                                    (21, "Reliability"),
                                                    (22, "Optimization"),
                                                    (23, "Safety (SOA)"),
                                                ],
                                            ),
                                        ];

                                        let selection_color = ui.visuals().selection.bg_fill;
                                        let hover_color = ui.visuals().widgets.hovered.bg_fill;
                                        let text_color = ui.visuals().text_color();
                                        let dim_color = text_color.gamma_multiply(0.6);
                                        let header_bg = ui.visuals().faint_bg_color;

                                        for (category_name, analyses) in categories {
                                            // Category header
                                            let (header_rect, _) = ui.allocate_exact_size(
                                                egui::vec2(item_width, header_height),
                                                egui::Sense::hover(),
                                            );
                                            ui.painter().rect_filled(header_rect, 0.0, header_bg);
                                            ui.painter().text(
                                                header_rect.left_center() + egui::vec2(8.0, 0.0),
                                                egui::Align2::LEFT_CENTER,
                                                *category_name,
                                                egui::FontId::proportional(11.0),
                                                dim_color,
                                            );

                                            // Analysis items in this category
                                            for &(idx, name) in *analyses {
                                                let selected =
                                                    self.state.dialogs.sim_active_tab == idx;
                                                let enabled = self
                                                    .state
                                                    .dialogs
                                                    .enabled_analyses
                                                    .contains(&idx);

                                                // Allocate full-width clickable rect
                                                let (rect, response) = ui.allocate_exact_size(
                                                    egui::vec2(item_width, item_height),
                                                    egui::Sense::click(),
                                                );

                                                // Draw background
                                                if selected {
                                                    ui.painter().rect_filled(
                                                        rect,
                                                        4.0,
                                                        selection_color,
                                                    );
                                                } else if response.hovered() {
                                                    ui.painter().rect_filled(
                                                        rect,
                                                        4.0,
                                                        hover_color,
                                                    );
                                                }

                                                // Draw modern checkbox (left side)
                                                let checkbox_center =
                                                    rect.left_center() + egui::vec2(16.0, 0.0);
                                                let box_size = 16.0;
                                                let checkbox_rect = egui::Rect::from_center_size(
                                                    checkbox_center,
                                                    egui::vec2(box_size, box_size),
                                                );

                                                if enabled {
                                                    // Filled checkbox when enabled
                                                    ui.painter().rect_filled(
                                                        checkbox_rect,
                                                        3.0,
                                                        egui::Color32::from_rgb(80, 160, 80),
                                                    );
                                                    // Checkmark
                                                    let check_color = egui::Color32::WHITE;
                                                    let s = box_size * 0.25;
                                                    let c = checkbox_center;
                                                    ui.painter().line_segment(
                                                        [
                                                            egui::pos2(c.x - s * 1.2, c.y),
                                                            egui::pos2(
                                                                c.x - s * 0.3,
                                                                c.y + s * 0.9,
                                                            ),
                                                        ],
                                                        egui::Stroke::new(2.0, check_color),
                                                    );
                                                    ui.painter().line_segment(
                                                        [
                                                            egui::pos2(
                                                                c.x - s * 0.3,
                                                                c.y + s * 0.9,
                                                            ),
                                                            egui::pos2(
                                                                c.x + s * 1.2,
                                                                c.y - s * 0.8,
                                                            ),
                                                        ],
                                                        egui::Stroke::new(2.0, check_color),
                                                    );
                                                } else {
                                                    // Empty checkbox
                                                    ui.painter().rect_stroke(
                                                        checkbox_rect,
                                                        3.0,
                                                        egui::Stroke::new(1.5, dim_color),
                                                    );
                                                }

                                                // Draw text (shifted right for checkbox)
                                                let text_col = if selected {
                                                    egui::Color32::WHITE
                                                } else {
                                                    text_color
                                                };
                                                ui.painter().text(
                                                    rect.left_center() + egui::vec2(34.0, 0.0),
                                                    egui::Align2::LEFT_CENTER,
                                                    name,
                                                    egui::FontId::proportional(13.0),
                                                    text_col,
                                                );

                                                // Click handling
                                                if response.clicked() {
                                                    let click_pos = response
                                                        .interact_pointer_pos()
                                                        .unwrap_or_default();
                                                    if checkbox_rect.contains(click_pos) {
                                                        // Toggle enabled state
                                                        if self
                                                            .state
                                                            .dialogs
                                                            .enabled_analyses
                                                            .contains(&idx)
                                                        {
                                                            self.state
                                                                .dialogs
                                                                .enabled_analyses
                                                                .remove(&idx);
                                                        } else {
                                                            self.state
                                                                .dialogs
                                                                .enabled_analyses
                                                                .insert(idx);
                                                        }
                                                    } else {
                                                        // Select this analysis
                                                        self.state.dialogs.sim_active_tab = idx;
                                                    }
                                                }
                                                if response.double_clicked() {
                                                    if self
                                                        .state
                                                        .dialogs
                                                        .enabled_analyses
                                                        .contains(&idx)
                                                    {
                                                        self.state
                                                            .dialogs
                                                            .enabled_analyses
                                                            .remove(&idx);
                                                    } else {
                                                        self.state
                                                            .dialogs
                                                            .enabled_analyses
                                                            .insert(idx);
                                                    }
                                                }
                                            }

                                            // Space between categories
                                            ui.add_space(4.0);
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
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Cancel").clicked() {
                                self.state.dialogs.simulation_dialog = false;
                            }
                            if ui.button("OK").clicked() {
                                self.state.dialogs.simulation_dialog = false;
                            }
                        });
                    });
                });
            // Only write back dialog state if dialog wasn't closed by buttons
            // (X button close sets dialog_open to false)
            if !dialog_open {
                self.state.dialogs.simulation_dialog = false;
            }
        }

        // Simulation Options Dialog
        self.render_simulation_options_dialog(ctx);

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

        // Waveform Calculator Dialog
        if self.state.dialogs.waveform_calculator_dialog {
            egui::Window::new("🧮 Calculator")
                .open(&mut self.state.dialogs.waveform_calculator_dialog)
                .default_width(400.0)
                .show(ctx, |ui| {
                    self.state.calculator_panel.show(ui, &self.state.simulation);
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

        if self.state.dialogs.new_cell_dialog {
            let mut should_close = false;
            let mut should_create = false;
            let mut persist_global_veriloga = false;

            egui::Window::new("📦 Create New Cell")
                .collapsible(false)
                .resizable(false)
                .default_width(400.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;

                    // Library selection
                    ui.horizontal(|ui| {
                        ui.label("Library:");
                        ui.add_space(20.0);

                        // Get editable library names (non-readonly)
                        let lib_names: Vec<String> = self
                            .state
                            .library_manager
                            .libraries_sorted()
                            .iter()
                            .filter(|lib| !lib.read_only)
                            .map(|lib| lib.name.clone())
                            .collect();

                        // Default to user library if empty
                        if self.state.dialogs.new_cell_library.is_empty() && !lib_names.is_empty() {
                            self.state.dialogs.new_cell_library = lib_names[0].clone();
                        }

                        egui::ComboBox::from_id_salt("cell_library_combo")
                            .selected_text(&self.state.dialogs.new_cell_library)
                            .width(200.0)
                            .show_ui(ui, |ui| {
                                for name in &lib_names {
                                    ui.selectable_value(
                                        &mut self.state.dialogs.new_cell_library,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    });

                    // Cell name input
                    ui.horizontal(|ui| {
                        ui.label("Cell Name:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.state.dialogs.new_cell_name)
                                .hint_text("e.g., my_opamp")
                                .desired_width(200.0),
                        );
                    });

                    // Description input
                    ui.horizontal(|ui| {
                        ui.label("Description:");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut self.state.dialogs.new_cell_description,
                            )
                            .hint_text("Optional description")
                            .desired_width(200.0),
                        );
                    });

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // View types to create
                    ui.label("Views to Create:");
                    ui.indent("views_indent", |ui| {
                        ui.checkbox(
                            &mut self.state.dialogs.new_cell_create_schematic,
                            "📋 Schematic",
                        );
                        ui.checkbox(&mut self.state.dialogs.new_cell_create_symbol, "🔲 Symbol");
                        ui.checkbox(
                            &mut self.state.dialogs.new_cell_create_testbench,
                            "🧪 Testbench",
                        );
                    });

                    // Error message display
                    if let Some(ref error) = self.state.dialogs.new_cell_error {
                        ui.add_space(4.0);
                        ui.colored_label(egui::Color32::RED, format!("⚠ {}", error));
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            should_create = true;
                        }
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                        }
                    });
                });

            // Handle create action outside of UI closure (borrow checker)
            if should_create {
                let name = self.state.dialogs.new_cell_name.trim();
                let library = self.state.dialogs.new_cell_library.clone();

                // Validation
                if name.is_empty() {
                    self.state.dialogs.new_cell_error =
                        Some("Cell name cannot be empty".to_string());
                } else if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    self.state.dialogs.new_cell_error = Some(
                        "Cell name must contain only letters, numbers, and underscores".to_string(),
                    );
                } else if library.is_empty() {
                    self.state.dialogs.new_cell_error = Some("Please select a library".to_string());
                } else if self
                    .state
                    .library_manager
                    .get_library(&library)
                    .and_then(|lib| lib.get_cell(name))
                    .is_some()
                {
                    self.state.dialogs.new_cell_error = Some(format!(
                        "Cell '{}' already exists in library '{}'",
                        name, library
                    ));
                } else {
                    // Create the cell
                    use crate::state::{Cell, View, ViewType};

                    let mut cell = Cell::new(name);
                    cell.description = self.state.dialogs.new_cell_description.clone();

                    if self.state.dialogs.new_cell_create_schematic {
                        cell.add_view(View::new("schematic", ViewType::Schematic));
                    }
                    if self.state.dialogs.new_cell_create_symbol {
                        cell.add_view(View::new("symbol", ViewType::Symbol));
                    }
                    if self.state.dialogs.new_cell_create_testbench {
                        cell.add_view(View::new("testbench", ViewType::Testbench));
                    }

                    // Add cell to library
                    if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
                        lib.add_cell(cell);
                        self.state
                            .console_messages
                            .push(ConsoleMessage::info(format!(
                                "Created cell '{}' in library '{}'",
                                name, library
                            )));
                        should_close = true;
                        persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
                    } else {
                        self.state.dialogs.new_cell_error =
                            Some(format!("Library '{}' not found", library));
                    }
                }
            }

            if persist_global_veriloga {
                if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
                    log::warn!("Failed to persist global Verilog-A library: {}", err);
                    self.state
                        .console_messages
                        .push(ConsoleMessage::warning(format!(
                            "Failed to persist Verilog-A library: {}",
                            err
                        )));
                }
            }

            if should_close {
                // Reset dialog state
                self.state.dialogs.new_cell_dialog = false;
                self.state.dialogs.new_cell_name.clear();
                self.state.dialogs.new_cell_description.clear();
                self.state.dialogs.new_cell_error = None;
                self.state.dialogs.new_cell_create_schematic = true;
                self.state.dialogs.new_cell_create_symbol = false;
                self.state.dialogs.new_cell_create_testbench = false;
            }
        }

        // New View Creation Dialog
        if self.state.dialogs.new_view_dialog {
            let mut should_close = false;
            let mut should_create = false;
            let mut persist_global_veriloga = false;

            egui::Window::new("📐 Create New View")
                .collapsible(false)
                .resizable(false)
                .default_width(350.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;

                    // Show target library and cell (read-only)
                    ui.horizontal(|ui| {
                        ui.label("Library:");
                        ui.add_space(16.0);
                        ui.label(&self.state.dialogs.new_view_library);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Cell:");
                        ui.add_space(38.0);
                        ui.label(&self.state.dialogs.new_view_cell);
                    });

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // View name input
                    ui.horizontal(|ui| {
                        ui.label("View Name:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.state.dialogs.new_view_name)
                                .hint_text("e.g., schematic")
                                .desired_width(150.0),
                        );
                    });

                    // View type selection
                    ui.horizontal(|ui| {
                        ui.label("View Type:");
                        ui.add_space(4.0);
                        egui::ComboBox::from_id_salt("view_type_combo")
                            .selected_text(self.state.dialogs.new_view_type.display_name())
                            .width(150.0)
                            .show_ui(ui, |ui| {
                                use crate::state::ViewType;
                                for vt in ViewType::ALL.iter() {
                                    ui.selectable_value(
                                        &mut self.state.dialogs.new_view_type,
                                        *vt,
                                        vt.display_name(),
                                    );
                                }
                            });
                    });

                    // Error message display
                    if let Some(ref error) = self.state.dialogs.new_view_error {
                        ui.add_space(4.0);
                        ui.colored_label(egui::Color32::RED, format!("⚠ {}", error));
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            should_create = true;
                        }
                        if ui.button("Cancel").clicked() {
                            should_close = true;
                        }
                    });
                });

            // Handle create action
            if should_create {
                let view_name = self.state.dialogs.new_view_name.trim();
                let library = self.state.dialogs.new_view_library.clone();
                let cell = self.state.dialogs.new_view_cell.clone();

                // Validation
                if view_name.is_empty() {
                    self.state.dialogs.new_view_error =
                        Some("View name cannot be empty".to_string());
                } else if !view_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    self.state.dialogs.new_view_error = Some(
                        "View name must contain only letters, numbers, and underscores".to_string(),
                    );
                } else {
                    // Check if view exists
                    let exists = self
                        .state
                        .library_manager
                        .get_library(&library)
                        .and_then(|lib| lib.get_cell(&cell))
                        .map(|c| c.get_view(view_name).is_some())
                        .unwrap_or(false);

                    if exists {
                        self.state.dialogs.new_view_error = Some(format!(
                            "View '{}' already exists in cell '{}'",
                            view_name, cell
                        ));
                    } else {
                        // Create the view
                        use crate::state::View;
                        if let Some(lib) = self.state.library_manager.get_library_mut(&library) {
                            if let Some(cell_ref) = lib.get_cell_mut(&cell) {
                                cell_ref.add_view(View::new(
                                    view_name,
                                    self.state.dialogs.new_view_type,
                                ));
                                self.state
                                    .console_messages
                                    .push(ConsoleMessage::info(format!(
                                        "Created view '{}' in cell '{}'",
                                        view_name, cell
                                    )));
                                should_close = true;
                                persist_global_veriloga = library == VERILOGA_LIBRARY_NAME;
                            }
                        }
                    }
                }
            }

            if persist_global_veriloga {
                if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
                    log::warn!("Failed to persist global Verilog-A library: {}", err);
                    self.state
                        .console_messages
                        .push(ConsoleMessage::warning(format!(
                            "Failed to persist Verilog-A library: {}",
                            err
                        )));
                }
            }

            if should_close {
                self.state.dialogs.new_view_dialog = false;
                self.state.dialogs.new_view_name.clear();
                self.state.dialogs.new_view_error = None;
            }
        }

        // Process pending cell deletion
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    self.state
                        .console_messages
                        .push(ConsoleMessage::info(format!(
                            "Deleted cell '{}' from library '{}'",
                            cell_name, lib_name
                        )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
                    log::warn!("Failed to persist global Verilog-A library: {}", err);
                    self.state
                        .console_messages
                        .push(ConsoleMessage::warning(format!(
                            "Failed to persist Verilog-A library: {}",
                            err
                        )));
                }
            }
        }

        // Process pending view deletion
        if let Some((lib_name, cell_name, view_name)) = self.state.pending_delete_view.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                if let Some(cell) = lib.get_cell_mut(&cell_name) {
                    deleted = cell.remove_view(&view_name);
                    if deleted {
                        self.state
                            .console_messages
                            .push(ConsoleMessage::info(format!(
                                "Deleted view '{}' from cell '{}'",
                                view_name, cell_name
                            )));
                    }
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                if let Err(err) = save_global_veriloga_library(&self.state.library_manager) {
                    log::warn!("Failed to persist global Verilog-A library: {}", err);
                    self.state
                        .console_messages
                        .push(ConsoleMessage::warning(format!(
                            "Failed to persist Verilog-A library: {}",
                            err
                        )));
                }
            }
        }
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

    /// Toggle the console panel visibility
    pub fn toggle_panel_console_new(&mut self) {
        // Switch to console tab if not active
        if self.state.panels.active_bottom_tab != BottomPanelTab::Console {
            self.state.panels.active_bottom_tab = BottomPanelTab::Console;
            self.state.panels.bottom_panel = true;
        } else {
            // Toggle visibility if already active
            self.state.panels.bottom_panel = !self.state.panels.bottom_panel;
        }
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
// Icon Rail Helper Functions
// =============================================================================

/// Create an icon button for the icon rail (32x32 procedural icon)
/// Uses direct positioning to ensure pixel-perfect centering
fn rail_icon_button(
    ui: &mut Ui,
    icon: crate::schematic::toolbar::IconType,
    active: bool,
    accent: Color32,
) -> egui::Response {
    use crate::schematic::toolbar::paint_icon;

    let size = Vec2::splat(32.0);
    let fill = if active {
        accent
    } else {
        Color32::from_rgb(58, 62, 74)
    };

    // Allocate vertical space for the button height
    let (full_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), size.y),
        egui::Sense::hover(),
    );

    // Calculate centered rect within the allocated space
    let center_x = full_rect.center().x;
    let rect = egui::Rect::from_center_size(egui::pos2(center_x, full_rect.center().y), size);

    // Register the actual button interaction area
    let response = ui.interact(
        rect,
        ui.id().with("rail_btn").with(icon as u8),
        egui::Sense::click(),
    );

    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals().widgets.style(&response);
        let rounding = egui::Rounding::same(4.0);
        let stroke = visuals.bg_stroke;

        ui.painter().rect(rect, rounding, fill, stroke);

        let icon_color = if active {
            Color32::WHITE
        } else {
            visuals.text_color()
        };
        paint_icon(ui, rect, icon, icon_color);
    }

    response
}

/// Create a disabled icon button for the icon rail
/// Uses direct positioning to ensure pixel-perfect centering
fn rail_icon_button_disabled(
    ui: &mut Ui,
    icon: crate::schematic::toolbar::IconType,
) -> egui::Response {
    use crate::schematic::toolbar::paint_icon;

    let size = Vec2::splat(32.0);
    let fill = Color32::from_rgb(45, 48, 56);

    // Allocate vertical space for the button height
    let (full_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), size.y),
        egui::Sense::hover(),
    );

    // Calculate centered rect within the allocated space
    let center_x = full_rect.center().x;
    let rect = egui::Rect::from_center_size(egui::pos2(center_x, full_rect.center().y), size);

    // Register the actual button interaction area (hover only for disabled)
    let response = ui.interact(
        rect,
        ui.id().with("rail_btn_dis").with(icon as u8),
        egui::Sense::hover(),
    );

    if ui.is_rect_visible(rect) {
        let rounding = egui::Rounding::same(4.0);

        ui.painter().rect(rect, rounding, fill, egui::Stroke::NONE);

        let icon_color = ui.visuals().widgets.noninteractive.text_color();
        paint_icon(ui, rect, icon, icon_color);
    }

    response
}

impl RSpiceApp {
    /// Render the left icon rail (VSCode style)
    fn render_icon_rail(&mut self, ui: &mut Ui) {
        use crate::schematic::toolbar::{paint_icon, IconType};
        use egui::{pos2, Rect, Sense};

        // Use zero item spacing for precise centering control
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        ui.vertical(|ui| {
            ui.add_space(4.0);

            // Project browser toggle
            let browser_active = self.state.panels.project_browser;
            if rail_icon_button(
                ui,
                IconType::Folder,
                browser_active,
                self.state.theme.accent,
            )
            .on_hover_text("Library Browser (Ctrl+Shift+L)")
            .clicked()
            {
                self.toggle_panel_browser();
            }

            ui.add_space(4.0);

            // Results browser toggle - always clickable, shows empty state if no results
            let results_active = self.state.panels.results_browser;
            let has_results = self.state.simulation.has_results();
            let results_response = rail_icon_button(
                ui,
                IconType::Results,
                results_active,
                self.state.theme.accent,
            );
            if results_response
                .on_hover_text(if has_results {
                    "Results Browser"
                } else {
                    "Results Browser (no results yet)"
                })
                .clicked()
            {
                // Toggle results browser, and close library browser if opening results
                if !self.state.panels.results_browser {
                    self.state.panels.project_browser = false;
                }
                self.state.panels.results_browser = !self.state.panels.results_browser;
            }

            ui.add_space(4.0);

            // Spacer to push bottom items down
            ui.add_space(ui.available_height() - 40.0);

            // Unified Bottom Panel Toggle
            // Replaces separate Console and Waveform buttons for a cleaner UI
            // The active tab is managed within the panel itself
            let panel_active = self.state.panels.bottom_panel;
            if rail_icon_button(
                ui,
                IconType::PanelBottom,
                panel_active,
                self.state.theme.accent,
            )
            .on_hover_text("Toggle Bottom Panel (Ctrl+`)")
            .clicked()
            {
                self.toggle_bottom_panel();
            }
        });
    }

    /// Render the console panel
    fn render_console_panel(&mut self, ui: &mut Ui) {
        // Header row with Clear button only (no close button - that's in tab bar)
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 26.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Output")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(160, 160, 170)),
                );

                // Small "Clear" button
                ui.add_space(6.0);
                if ui.small_button("Clear").clicked() {
                    self.state.console_messages.clear();
                }
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

    fn sync_console_messages_into_log_buffer(&mut self) {
        if self.state.log_sync_cursor > self.state.console_messages.len() {
            self.state.log_sync_cursor = self.state.console_messages.len();
        }

        for message in self
            .state
            .console_messages
            .iter()
            .skip(self.state.log_sync_cursor)
        {
            let severity = match message.level {
                ConsoleLevel::Info => crate::panels::LogSeverity::Info,
                ConsoleLevel::Warning => crate::panels::LogSeverity::Warning,
                ConsoleLevel::Error => crate::panels::LogSeverity::Error,
            };
            self.state.log_buffer.log(
                severity,
                crate::panels::LogSource::System,
                message.message.clone(),
                None,
            );
        }

        self.state.log_sync_cursor = self.state.console_messages.len();
    }

    /// Render the structured log panel.
    fn render_log_panel(&mut self, ui: &mut Ui) {
        self.sync_console_messages_into_log_buffer();
        crate::panels::render_log_panel(
            ui,
            &self.state.log_buffer,
            &mut self.state.log_panel_state,
        );
    }

    /// Render the automation/scripting panel
    fn render_automation_panel(&mut self, ui: &mut Ui) {
        // Delegate to the existing script console renderer
        crate::panels::render_script_console(
            ui,
            &mut self.state.script_console,
            &mut self.state.simulation,
        );
    }

    /// Render the Bode plot panel (AC analysis magnitude/phase)
    fn render_bode_panel(&mut self, ui: &mut Ui) {
        crate::analysis::bode::render_bode_panel(ui, &mut self.state);
    }

    /// Render the Pole-Zero map panel
    fn render_polezero_panel(&mut self, ui: &mut Ui) {
        crate::analysis::pole_zero::render_pz_plot(ui, &mut self.state.pole_zero_state);
    }

    /// Render the Eye diagram panel
    fn render_eye_panel(&mut self, ui: &mut Ui) {
        crate::analysis::eye_diagram::render_eye_diagram(ui, &mut self.state.eye_diagram_state);
    }

    /// Render the Smith chart panel
    fn render_smith_panel(&mut self, ui: &mut Ui) {
        crate::analysis::smith_chart::render_smith_chart(ui, &mut self.state.smith_chart_state);
    }

    /// Render the Histogram panel (Monte Carlo/corners)
    fn render_histogram_panel(&mut self, ui: &mut Ui) {
        crate::analysis::histogram::render_histogram(ui, &self.state.histogram_state);
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
    #[serde(default)]
    results_browser: bool,
    properties: bool,
    #[serde(default = "default_bottom_panel")]
    bottom_panel: bool,
    #[serde(default)]
    active_bottom_tab: usize, // Serialize as index for backwards compat
    #[serde(default)]
    smith_chart: bool,
    #[serde(default)]
    signal_browser: bool,
    #[serde(default)]
    script_console: bool,
}

fn default_bottom_panel() -> bool {
    true
}

impl From<&PanelVisibility> for PanelVisibilitySer {
    fn from(p: &PanelVisibility) -> Self {
        Self {
            project_browser: p.project_browser,
            results_browser: p.results_browser,
            properties: p.properties,
            bottom_panel: p.bottom_panel,
            active_bottom_tab: match p.active_bottom_tab {
                BottomPanelTab::Console => 0,
                BottomPanelTab::Waveform => 1,
                BottomPanelTab::Log => 2,
                BottomPanelTab::Automation => 3,
            },
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
            results_browser: s.results_browser,
            properties: s.properties,
            bottom_panel: s.bottom_panel,
            active_bottom_tab: match s.active_bottom_tab {
                0 => BottomPanelTab::Console,
                1 => BottomPanelTab::Waveform,
                2 => BottomPanelTab::Log,
                3 => BottomPanelTab::Automation,
                _ => BottomPanelTab::Console,
            },
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
    use crate::properties::dialog::EditedProperties;
    use crate::state::{Component, ComponentType, Point};

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
            BottomPanelTab::Console,
            "Console tab should be active by default"
        );
    }

    #[test]
    fn test_apply_component_property_edits_creates_undo_and_restores_on_undo() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(100, 100))
                .with_name_value("R1", "1k"),
        );

        let changed = apply_component_property_edits(
            &mut state,
            1,
            EditedProperties {
                name: "R2".to_string(),
                value: "2k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(changed, "property edit should create an undo checkpoint");
        assert!(state.schematic.is_dirty);
        assert!(state.schematic.can_undo());
        assert_eq!(
            state.schematic.undo_description(),
            Some("Edit properties for R1")
        );

        let updated = state
            .schematic
            .components
            .iter()
            .find(|c| c.id == 1)
            .expect("component should remain present");
        assert_eq!(updated.name, "R2");
        assert_eq!(updated.value, "2k");

        assert!(
            state.schematic.undo(),
            "undo should restore original properties"
        );
        let restored = state
            .schematic
            .components
            .iter()
            .find(|c| c.id == 1)
            .expect("component should still be present after undo");
        assert_eq!(restored.name, "R1");
        assert_eq!(restored.value, "1k");
    }

    #[test]
    fn test_apply_component_property_edits_noop_when_values_unchanged() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(100, 100))
                .with_name_value("R1", "1k"),
        );

        let changed = apply_component_property_edits(
            &mut state,
            1,
            EditedProperties {
                name: "R1".to_string(),
                value: "1k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(!changed, "no-op edits should not create undo entries");
        assert!(!state.schematic.can_undo());
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
}
