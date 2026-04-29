use std::path::PathBuf;

use crate::state::pdk_config::{DiscoveredFile, PdkConfig};

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
}

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
