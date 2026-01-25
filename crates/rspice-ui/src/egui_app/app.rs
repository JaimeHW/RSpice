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

use egui::{CentralPanel, Context, Frame, Key, Modifiers, SidePanel, TopBottomPanel, Ui, Vec2};

use crate::state::{SchematicState, SimulationState};

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
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self {
            project_browser: false, // Hidden by default to maximize canvas
            properties: true,
            waveform: true,
            console: true,
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
            waveform_height: 200.0,
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
    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Info,
            timestamp: 0.0, // TODO: Use proper timestamp
            message: message.into(),
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Warning,
            timestamp: 0.0,
            message: message.into(),
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Error,
            timestamp: 0.0,
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

        // Log startup
        log::info!("RSpice egui application initialized");

        Self {
            state,
            first_frame: true,
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
        // TODO: Implement file open dialog
        self.state
            .console_messages
            .push(ConsoleMessage::info("Open: Coming soon"));
    }

    fn action_file_save(&mut self) {
        // TODO: Implement file save
        self.state
            .console_messages
            .push(ConsoleMessage::info("Save: Coming soon"));
    }

    fn action_edit_undo(&mut self) {
        // TODO: Implement undo stack in SchematicState
        log::debug!("Undo not yet implemented");
        self.state
            .console_messages
            .push(ConsoleMessage::info("Undo: Coming soon"));
    }

    fn action_edit_redo(&mut self) {
        // TODO: Implement redo stack in SchematicState
        log::debug!("Redo not yet implemented");
        self.state
            .console_messages
            .push(ConsoleMessage::info("Redo: Coming soon"));
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

        // =====================================================================
        // Menu Bar
        // =====================================================================
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            super::menu_bar::render_menu_bar(ui, &mut self.state);
        });

        // =====================================================================
        // Toolbar
        // =====================================================================
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            super::toolbar::render_toolbar(ui, &mut self.state);
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
        // Icon Rail (left side) - FIRST!
        // =====================================================================
        SidePanel::left("icon_rail")
            .resizable(false)
            .exact_width(42.0)
            .show(ctx, |ui| {
                self.render_icon_rail(ui);
            });

        // =====================================================================
        // Console Panel (bottom) - spans full width except icon rail
        // =====================================================================
        if self.state.panels.console {
            TopBottomPanel::bottom("console")
                .resizable(true)
                .default_height(self.state.panel_sizes.console_height)
                .height_range(60.0..=400.0)
                .show(ctx, |ui| {
                    self.render_console_panel(ui);
                });
        }

        // =====================================================================
        // Waveform Panel (optional, resizable from top)
        // =====================================================================
        if self.state.panels.waveform && !self.state.simulation.waveforms.is_empty() {
            TopBottomPanel::bottom("waveform")
                .resizable(true)
                .default_height(self.state.panel_sizes.waveform_height)
                .height_range(100.0..=600.0)
                .show(ctx, |ui| {
                    self.render_waveform_panel(ui);
                });
        }

        // =====================================================================
        // Project Browser (left, optional)
        // =====================================================================
        if self.state.panels.project_browser {
            SidePanel::left("project_browser")
                .resizable(true)
                .default_width(self.state.panel_sizes.browser_width)
                .width_range(150.0..=400.0)
                .show(ctx, |ui| {
                    super::panels::render_project_browser(ui, &mut self.state);
                });
        }

        // =====================================================================
        // Properties Panel (right)
        // =====================================================================
        if self.state.panels.properties {
            SidePanel::right("properties")
                .resizable(true)
                .default_width(self.state.panel_sizes.properties_width)
                .width_range(180.0..=400.0)
                .show(ctx, |ui| {
                    super::panels::render_properties_panel(ui, &mut self.state);
                });
        }

        // =====================================================================
        // Central Schematic Editor
        // =====================================================================
        CentralPanel::default().show(ctx, |ui| {
            super::schematic_view::render_schematic_view(ui, &mut self.state);
        });
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
        });
    }

    /// Render the console panel
    fn render_console_panel(&mut self, ui: &mut Ui) {
        // Header with proper vertical centering and close button
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Console")
                    .size(13.0)
                    .strong()
                    .color(egui::Color32::from_rgb(200, 200, 210)),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Close button
                let close_btn = ui.add(
                    egui::Button::new(egui::RichText::new("✕").size(12.0))
                        .frame(false)
                        .min_size(egui::vec2(20.0, 20.0)),
                );
                if close_btn.on_hover_text("Close Console").clicked() {
                    self.state.panels.console = false;
                }

                ui.add_space(4.0);

                // Clear button
                let clear_btn = ui.add(
                    egui::Button::new(egui::RichText::new("Clear").size(11.0))
                        .min_size(egui::vec2(50.0, 20.0)),
                );
                if clear_btn.clicked() {
                    self.state.console_messages.clear();
                }
            });
        });

        // Subtle separator
        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
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
        ui.horizontal(|ui| {
            ui.heading("Waveform Viewer");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("✕").clicked() {
                    self.state.panels.waveform = false;
                }
            });
        });

        ui.separator();

        // TODO: Render actual waveforms using egui plot or custom wgpu
        ui.centered_and_justified(|ui| {
            ui.label("Waveform rendering coming soon...");
        });
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
}

impl From<&PanelVisibility> for PanelVisibilitySer {
    fn from(p: &PanelVisibility) -> Self {
        Self {
            project_browser: p.project_browser,
            properties: p.properties,
            waveform: p.waveform,
            console: p.console,
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
