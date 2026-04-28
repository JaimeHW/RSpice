//! PDK Settings Dialog
//!
//! Professional interface for configuring PDK (Process Design Kit) library paths,
//! environment variables, and model file discovery settings.
//!
//! ## Features
//!
//! - Library path management (add/remove/enable/disable)
//! - Environment variable configuration (e.g., $PDK_HOME)
//! - Model file discovery with preview
//! - Recent files tracking
//! - Persistent configuration

use std::path::PathBuf;

use crate::state::pdk_config::{DiscoveredFile, PdkConfig};

// =============================================================================
// Dialog State
// =============================================================================

/// State for the PDK Settings dialog
#[derive(Debug, Clone, Default)]
pub struct PdkSettingsDialogState {
    /// Whether the dialog is open
    pub open: bool,
    /// Working copy of the PDK configuration
    pub config: PdkConfig,
    /// Original config for detecting changes
    original_config: Option<PdkConfig>,
    /// Input field for new library path
    pub new_path_input: String,
    /// Input field for new environment variable name
    pub new_env_name: String,
    /// Input field for new environment variable value
    pub new_env_value: String,
    /// Currently selected tab
    pub selected_tab: PdkSettingsTab,
    /// Whether a scan is in progress
    pub scanning: bool,
    /// Filter text for discovered files
    pub file_filter: String,
    /// Whether to show only enabled paths
    pub show_only_enabled: bool,
    /// Index of path being edited (if any)
    pub editing_path_index: Option<usize>,
    /// Temporary edit buffer for path
    pub edit_path_buffer: String,
    /// Index of env var being edited (if any)
    pub editing_env_index: Option<usize>,
    /// Temporary edit buffer for env var name
    pub edit_env_name_buffer: String,
    /// Temporary edit buffer for env var value
    pub edit_env_value_buffer: String,
}

impl PdkSettingsDialogState {
    /// Create a new dialog state
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog with current configuration
    pub fn open(&mut self, config: PdkConfig) {
        self.config = config.clone();
        self.original_config = Some(config);
        self.open = true;
        self.reset_inputs();

        // Auto-discover files if there are library paths configured
        if !self.config.library_paths().is_empty() {
            self.rescan();
        }
    }

    /// Open the dialog, loading from default location
    pub fn open_default(&mut self) {
        self.config = PdkConfig::load_or_default();
        self.original_config = Some(self.config.clone());
        self.open = true;
        self.reset_inputs();
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.open = false;
        self.reset_inputs();
    }

    /// Reset all input fields
    fn reset_inputs(&mut self) {
        self.new_path_input.clear();
        self.new_env_name.clear();
        self.new_env_value.clear();
        self.file_filter.clear();
        self.editing_path_index = None;
        self.edit_path_buffer.clear();
        self.editing_env_index = None;
        self.edit_env_name_buffer.clear();
        self.edit_env_value_buffer.clear();
    }

    /// Check if configuration has been modified
    pub fn has_changes(&self) -> bool {
        self.original_config
            .as_ref()
            .map(|orig| {
                // Compare library paths
                if self.config.library_paths().len() != orig.library_paths().len() {
                    return true;
                }
                for (a, b) in self.config.library_paths().iter().zip(orig.library_paths()) {
                    if a.path != b.path || a.enabled != b.enabled || a.recursive != b.recursive {
                        return true;
                    }
                }
                // Compare env vars
                if self.config.env_overrides().len() != orig.env_overrides().len() {
                    return true;
                }
                for (k, v) in self.config.env_overrides() {
                    if orig.env_overrides().get(k) != Some(v) {
                        return true;
                    }
                }
                false
            })
            .unwrap_or(false)
    }

    /// Get discovered files filtered by search
    pub fn filtered_files(&self) -> Vec<&DiscoveredFile> {
        let filter = self.file_filter.to_lowercase();
        self.config
            .discovered_files()
            .iter()
            .filter(|f| {
                if filter.is_empty() {
                    true
                } else {
                    f.path_str().to_lowercase().contains(&filter)
                        || f.file_type().to_lowercase().contains(&filter)
                }
            })
            .collect()
    }

    /// Trigger a rescan of library paths
    pub fn rescan(&mut self) {
        self.scanning = true;
        self.config.discover_model_files();
        self.scanning = false;
    }

    /// Add a new library path
    pub fn add_library_path(&mut self, path: String) {
        if !path.is_empty() {
            self.config.add_library_path(path);
            self.new_path_input.clear();
            // Auto-rescan to show discovered files immediately
            self.rescan();
        }
    }

    /// Remove a library path by index
    pub fn remove_library_path(&mut self, index: usize) {
        self.config.remove_library_path(index);
    }

    /// Toggle path enabled state
    pub fn toggle_path_enabled(&mut self, index: usize) {
        self.config.toggle_path_enabled(index);
    }

    /// Toggle path recursive state
    pub fn toggle_path_recursive(&mut self, index: usize) {
        self.config.toggle_path_recursive(index);
    }

    /// Add an environment variable override
    pub fn add_env_override(&mut self, name: String, value: String) {
        if !name.is_empty() {
            self.config.set_env_override(name, value);
            self.new_env_name.clear();
            self.new_env_value.clear();
        }
    }

    /// Remove an environment variable override
    pub fn remove_env_override(&mut self, name: &str) {
        self.config.remove_env_override(name);
    }

    /// Apply changes and return the updated config
    pub fn apply(&mut self) -> PdkConfig {
        // Save to persistent storage
        if let Err(e) = self.config.save() {
            eprintln!("Warning: Failed to save PDK config: {}", e);
        }
        self.original_config = Some(self.config.clone());
        self.config.clone()
    }
}

// =============================================================================
// Tab Selection
// =============================================================================

/// Available tabs in the PDK settings dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PdkSettingsTab {
    /// Library paths configuration
    #[default]
    LibraryPaths,
    /// Environment variables
    Environment,
    /// Discovered files browser
    DiscoveredFiles,
    /// Recent files
    RecentFiles,
}

impl PdkSettingsTab {
    /// Get display name for the tab
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::LibraryPaths => "Library Paths",
            Self::Environment => "Environment",
            Self::DiscoveredFiles => "Discovered Files",
            Self::RecentFiles => "Recent Files",
        }
    }

    /// Get all tabs
    pub fn all() -> &'static [PdkSettingsTab] {
        &[
            Self::LibraryPaths,
            Self::Environment,
            Self::DiscoveredFiles,
            Self::RecentFiles,
        ]
    }
}

// =============================================================================
// Dialog Result
// =============================================================================

/// Result of the PDK settings dialog
#[derive(Debug, Clone, PartialEq)]
pub enum PdkSettingsDialogResult {
    /// Dialog is still open, no action
    None,
    /// User cancelled without saving
    Cancelled,
    /// User applied changes
    Applied(PdkConfig),
    /// User requested to load a specific file
    LoadFile(PathBuf),
}

// =============================================================================
// Rendering
// =============================================================================

use egui::{Context, RichText, Ui, Window};

/// Render the PDK Settings dialog
///
/// Returns the dialog result indicating user action.
pub fn render_pdk_settings_dialog(
    ctx: &Context,
    state: &mut PdkSettingsDialogState,
) -> PdkSettingsDialogResult {
    if !state.open {
        return PdkSettingsDialogResult::None;
    }

    let mut result = PdkSettingsDialogResult::None;
    let mut should_close = false;
    let mut should_rescan = false;

    Window::new("PDK Settings")
        .resizable(true)
        .collapsible(false)
        .default_width(650.0)
        .default_height(500.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;

            // =========================================================
            // Tab Bar
            // =========================================================
            ui.horizontal(|ui| {
                for tab in PdkSettingsTab::all() {
                    let selected = state.selected_tab == *tab;
                    if ui.selectable_label(selected, tab.display_name()).clicked() {
                        state.selected_tab = *tab;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state.scanning {
                        ui.spinner();
                        ui.label("Scanning...");
                    } else if ui.button("🔄 Rescan").clicked() {
                        should_rescan = true;
                    }
                });
            });

            ui.separator();

            // =========================================================
            // Tab Content
            // =========================================================
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(380.0)
                .show(ui, |ui| match state.selected_tab {
                    PdkSettingsTab::LibraryPaths => {
                        render_library_paths_tab(ui, state);
                    }
                    PdkSettingsTab::Environment => {
                        render_environment_tab(ui, state);
                    }
                    PdkSettingsTab::DiscoveredFiles => {
                        if let Some(path) = render_discovered_files_tab(ui, state) {
                            result = PdkSettingsDialogResult::LoadFile(path);
                        }
                    }
                    PdkSettingsTab::RecentFiles => {
                        if let Some(path) = render_recent_files_tab(ui, state) {
                            result = PdkSettingsDialogResult::LoadFile(path);
                        }
                    }
                });

            // =========================================================
            // Status Bar
            // =========================================================
            ui.add_space(8.0);
            ui.separator();
            ui.horizontal(|ui| {
                // Statistics
                let file_count = state.config.discovered_files().len();
                let path_count = state.config.library_paths().len();
                let enabled_count = state
                    .config
                    .library_paths()
                    .iter()
                    .filter(|p| p.enabled)
                    .count();

                ui.label(
                    RichText::new(format!(
                        "{} files discovered | {} paths ({} enabled)",
                        file_count, path_count, enabled_count
                    ))
                    .color(egui::Color32::GRAY)
                    .size(11.0),
                );

                if state.has_changes() {
                    ui.label(
                        RichText::new("• Modified")
                            .color(egui::Color32::YELLOW)
                            .size(11.0),
                    );
                }
            });

            // =========================================================
            // Action Buttons
            // =========================================================
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                        result = PdkSettingsDialogResult::Cancelled;
                    }

                    let apply_enabled = state.has_changes();
                    if ui
                        .add_enabled(apply_enabled, egui::Button::new("Apply"))
                        .clicked()
                    {
                        let config = state.apply();
                        result = PdkSettingsDialogResult::Applied(config);
                        should_close = true;
                    }

                    if ui.button("OK").clicked() {
                        if state.has_changes() {
                            let config = state.apply();
                            result = PdkSettingsDialogResult::Applied(config);
                        }
                        should_close = true;
                    }
                });
            });
        });

    if should_rescan {
        state.rescan();
    }

    if should_close {
        state.close();
    }

    result
}

// =============================================================================
// Tab Renderers
// =============================================================================

/// Render the Library Paths tab
fn render_library_paths_tab(ui: &mut Ui, state: &mut PdkSettingsDialogState) {
    ui.heading("Library Search Paths");
    ui.add_space(4.0);

    ui.label(
        RichText::new("Configure directories where RSpice will search for PDK model files (.lib, .scs, .mod).")
            .color(egui::Color32::GRAY)
            .size(11.0),
    );

    ui.add_space(8.0);

    // Path list
    let mut action: Option<PathListAction> = None;

    for (idx, entry) in state.config.library_paths().iter().enumerate() {
        ui.horizontal(|ui| {
            // Enable checkbox
            let mut enabled = entry.enabled;
            if ui.checkbox(&mut enabled, "").changed() {
                action = Some(PathListAction::ToggleEnabled(idx));
            }

            // Recursive checkbox
            let mut recursive = entry.recursive;
            if ui
                .checkbox(&mut recursive, "")
                .on_hover_text("Scan subdirectories")
                .changed()
            {
                action = Some(PathListAction::ToggleRecursive(idx));
            }
            ui.label(RichText::new("R").size(10.0).color(if recursive {
                egui::Color32::GREEN
            } else {
                egui::Color32::GRAY
            }));

            // Path display or edit
            if state.editing_path_index == Some(idx) {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut state.edit_path_buffer).desired_width(350.0),
                );
                if response.lost_focus() {
                    action = Some(PathListAction::FinishEdit(idx));
                }
            } else {
                let path_text = if entry.enabled {
                    RichText::new(&entry.path)
                } else {
                    RichText::new(&entry.path).color(egui::Color32::GRAY)
                };
                if ui.link(path_text).clicked() {
                    state.editing_path_index = Some(idx);
                    state.edit_path_buffer = entry.path.clone();
                }
            }

            // File count badge
            if entry.file_count > 0 {
                ui.label(
                    RichText::new(format!("({} files)", entry.file_count))
                        .color(egui::Color32::from_rgb(100, 180, 100))
                        .size(10.0),
                );
            }

            // Delete button
            if ui.small_button("🗑").on_hover_text("Remove path").clicked() {
                action = Some(PathListAction::Remove(idx));
            }

            // Browse button
            if ui.small_button("📁").on_hover_text("Browse...").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .set_directory(&entry.path)
                    .pick_folder()
            {
                action = Some(PathListAction::Update(
                    idx,
                    path.to_string_lossy().to_string(),
                ));
            }
        });
    }

    // Process actions
    if let Some(act) = action {
        match act {
            PathListAction::Remove(idx) => state.remove_library_path(idx),
            PathListAction::ToggleEnabled(idx) => state.toggle_path_enabled(idx),
            PathListAction::ToggleRecursive(idx) => state.toggle_path_recursive(idx),
            PathListAction::FinishEdit(idx) => {
                if !state.edit_path_buffer.is_empty() {
                    let paths = state.config.library_paths_mut();
                    if idx < paths.len() {
                        paths[idx].path = state.edit_path_buffer.clone();
                    }
                }
                state.editing_path_index = None;
                state.edit_path_buffer.clear();
            }
            PathListAction::Update(idx, new_path) => {
                let paths = state.config.library_paths_mut();
                if idx < paths.len() {
                    paths[idx].path = new_path;
                }
            }
        }
    }

    // Add new path
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let text_edit = egui::TextEdit::singleline(&mut state.new_path_input)
            .desired_width(400.0)
            .hint_text("Add library path...");
        let response = ui.add(text_edit);

        if (ui.button("Add").clicked()
            || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
            && !state.new_path_input.is_empty()
        {
            state.add_library_path(state.new_path_input.clone());
        }

        if ui.button("Browse...").clicked()
            && let Some(path) = rfd::FileDialog::new().pick_folder()
        {
            state.add_library_path(path.to_string_lossy().to_string());
        }
    });

    // Help text
    ui.add_space(12.0);
    ui.label(
        RichText::new("💡 Tip: Use environment variables like $PDK_HOME in paths. Configure them in the Environment tab.")
            .color(egui::Color32::from_rgb(150, 150, 200))
            .size(11.0),
    );
}

/// Actions for path list manipulation
enum PathListAction {
    Remove(usize),
    ToggleEnabled(usize),
    ToggleRecursive(usize),
    FinishEdit(usize),
    Update(usize, String),
}

/// Render the Environment tab
fn render_environment_tab(ui: &mut Ui, state: &mut PdkSettingsDialogState) {
    ui.heading("Environment Variables");
    ui.add_space(4.0);

    ui.label(
        RichText::new(
            "Define environment variable overrides for path expansion (e.g., $PDK_HOME).",
        )
        .color(egui::Color32::GRAY)
        .size(11.0),
    );

    ui.add_space(8.0);

    // Environment variable list
    let mut remove_key: Option<String> = None;

    // Collect keys first to avoid borrow issues
    let env_entries: Vec<(String, String)> = state
        .config
        .env_overrides()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    for (name, value) in &env_entries {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            // Variable name
            ui.label(RichText::new(format!("${}", name)).strong().monospace());

            ui.label("=");

            // Value
            ui.label(RichText::new(value).monospace());

            // Resolved value (if different)
            let resolved = state.config.expand_path(value);
            if resolved != *value {
                ui.label(
                    RichText::new(format!("→ {}", resolved))
                        .color(egui::Color32::GRAY)
                        .size(10.0),
                );
            }

            // Delete button
            if ui.small_button("🗑").on_hover_text("Remove").clicked() {
                remove_key = Some(name.clone());
            }
        });
    }

    if let Some(key) = remove_key {
        state.remove_env_override(&key);
    }

    // Add new environment variable
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("$");
        ui.add(
            egui::TextEdit::singleline(&mut state.new_env_name)
                .desired_width(120.0)
                .hint_text("VAR_NAME"),
        );
        ui.label("=");
        ui.add(
            egui::TextEdit::singleline(&mut state.new_env_value)
                .desired_width(250.0)
                .hint_text("value or path"),
        );

        if ui.button("Add").clicked() && !state.new_env_name.is_empty() {
            state.add_env_override(state.new_env_name.clone(), state.new_env_value.clone());
        }
    });

    // System environment variables section
    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    egui::CollapsingHeader::new("System Environment Variables")
        .default_open(false)
        .show(ui, |ui| {
            ui.label(
                RichText::new("Read-only view of relevant system environment variables.")
                    .color(egui::Color32::GRAY)
                    .size(11.0),
            );
            ui.add_space(4.0);

            // Show common PDK-related env vars
            let common_vars = [
                "PDK_HOME",
                "PDK_ROOT",
                "MY_TECH",
                "SPICE_LIB_DIR",
                "MODEL_PATH",
            ];
            for var in common_vars {
                if let Ok(value) = std::env::var(var) {
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("${}", var)).monospace());
                        ui.label("=");
                        ui.label(RichText::new(&value).size(11.0));
                    });
                }
            }
        });
}

/// Render the Discovered Files tab
fn render_discovered_files_tab(ui: &mut Ui, state: &mut PdkSettingsDialogState) -> Option<PathBuf> {
    let mut load_file: Option<PathBuf> = None;

    ui.heading("Discovered Model Files");
    ui.add_space(4.0);

    // Filter bar
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.add(
            egui::TextEdit::singleline(&mut state.file_filter)
                .desired_width(200.0)
                .hint_text("Search files..."),
        );

        if !state.file_filter.is_empty() && ui.small_button("✖").clicked() {
            state.file_filter.clear();
        }
    });

    ui.add_space(4.0);

    // File list
    let filtered = state.filtered_files();

    if filtered.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(
                RichText::new("No model files discovered.")
                    .color(egui::Color32::GRAY)
                    .italics(),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Add library paths and click 'Rescan' to discover files.")
                    .color(egui::Color32::GRAY)
                    .size(11.0),
            );
        });
    } else {
        // Header
        ui.horizontal(|ui| {
            ui.label(RichText::new("Type").strong().size(11.0));
            ui.add_space(40.0);
            ui.label(RichText::new("Path").strong().size(11.0));
        });
        ui.separator();

        for file in filtered {
            ui.horizontal(|ui| {
                // File type badge
                let (type_color, type_text) = match file.file_type() {
                    "lib" => (egui::Color32::from_rgb(100, 180, 100), "LIB"),
                    "scs" => (egui::Color32::from_rgb(100, 150, 200), "SCS"),
                    "mod" => (egui::Color32::from_rgb(200, 150, 100), "MOD"),
                    "sp" | "cir" => (egui::Color32::from_rgb(150, 150, 150), "SP"),
                    _ => (egui::Color32::GRAY, "???"),
                };
                ui.label(
                    RichText::new(format!("[{}]", type_text))
                        .color(type_color)
                        .monospace()
                        .size(10.0),
                );

                // File path as clickable link
                let path_display = file.path_str();
                if ui.link(&path_display).clicked() {
                    load_file = Some(file.path.clone());
                }

                // Sections preview
                if !file.sections.is_empty() {
                    ui.label(
                        RichText::new(format!("({})", file.sections.join(", ")))
                            .color(egui::Color32::GRAY)
                            .size(10.0),
                    );
                }
            });
        }
    }

    load_file
}

/// Render the Recent Files tab
fn render_recent_files_tab(ui: &mut Ui, state: &mut PdkSettingsDialogState) -> Option<PathBuf> {
    let mut load_file: Option<PathBuf> = None;

    ui.heading("Recent Files");
    ui.add_space(4.0);

    let recent = state.config.recent_files();

    if recent.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(
                RichText::new("No recent files.")
                    .color(egui::Color32::GRAY)
                    .italics(),
            );
        });
    } else {
        let mut clear_all = false;

        for file_path in recent.iter() {
            ui.horizontal(|ui| {
                // File icon
                ui.label("📄");

                // Clickable path
                if ui.link(file_path).clicked() {
                    load_file = Some(PathBuf::from(file_path));
                }

                // Check if file exists
                if !std::path::Path::new(file_path).exists() {
                    ui.label(
                        RichText::new("(not found)")
                            .color(egui::Color32::from_rgb(200, 100, 100))
                            .size(10.0),
                    );
                }
            });
        }

        ui.add_space(8.0);
        if ui.button("Clear Recent Files").clicked() {
            clear_all = true;
        }

        if clear_all {
            state.config.clear_recent_files();
        }
    }

    load_file
}

// =============================================================================
// Tests
// =============================================================================
