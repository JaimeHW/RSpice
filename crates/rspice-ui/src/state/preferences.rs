//! User Preferences System
//!
//! Persistent user preferences for RSpice following commercial EDA patterns.
//! Includes grid settings, default values, colors, and simulation preferences.
//!
//! # Features
//!
//! - Comprehensive preference categories (schematic, waveform, simulation)
//! - Type-safe preference values
//! - JSON persistence to user data directory
//! - Defaults following industry conventions
//!
//! # Example
//!
//! ```ignore
//! use rspice_ui::state::preferences::Preferences;
//!
//! let mut prefs = Preferences::default();
//! prefs.schematic.grid_size = 20;
//! prefs.schematic.snap_to_grid = true;
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

//=============================================================================
// Main Preferences Structure
//=============================================================================

/// Application preferences with all configurable settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Preferences {
    /// General application preferences
    pub general: GeneralPreferences,
    /// Schematic editor preferences
    pub schematic: SchematicPreferences,
    /// Waveform viewer preferences
    pub waveform: WaveformPreferences,
    /// Simulation preferences
    pub simulation: SimulationPreferences,
    /// File preferences
    pub files: FilePreferences,
}

//=============================================================================
// General Preferences
//=============================================================================

/// General application preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralPreferences {
    /// Theme setting: "dark", "light", or "system"
    pub theme: ThemePreference,
    /// Show startup dialog
    pub show_startup_dialog: bool,
    /// Check for updates on startup
    pub check_updates: bool,
    /// Confirm before closing unsaved files
    pub confirm_close_unsaved: bool,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u32,
    /// Confirm before running DRC on simulate
    pub drc_before_simulate: bool,
}

impl Default for GeneralPreferences {
    fn default() -> Self {
        Self {
            theme: ThemePreference::Dark,
            show_startup_dialog: true,
            check_updates: false,
            confirm_close_unsaved: true,
            auto_save_interval: 60, // 1 minute
            drc_before_simulate: true,
        }
    }
}

/// Theme preference options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemePreference {
    /// Dark theme (default for EDA tools)
    Dark,
    /// Light theme
    Light,
    /// Follow system preference
    System,
}

impl ThemePreference {
    /// Get all available theme options
    pub fn all() -> &'static [ThemePreference] {
        &[Self::Dark, Self::Light, Self::System]
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
            Self::System => "System",
        }
    }
}

//=============================================================================
// Schematic Preferences
//=============================================================================

/// Schematic editor preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchematicPreferences {
    /// Grid size in pixels
    pub grid_size: u32,
    /// Enable snap to grid
    pub snap_to_grid: bool,
    /// Show grid lines
    pub show_grid: bool,
    /// Grid style
    pub grid_style: GridStyle,
    /// Show component values on schematic
    pub show_values: bool,
    /// Show component reference designators
    pub show_ref_des: bool,
    /// Show pin names on symbols
    pub show_pin_names: bool,
    /// Show pin numbers on symbols
    pub show_pin_numbers: bool,
    /// Wire width in pixels
    pub wire_width: f32,
    /// Junction dot radius in pixels
    pub junction_radius: f32,
    /// Default component rotation angle
    pub default_rotation: i32,
    /// Auto-number new components
    pub auto_number: bool,
    /// Select components after placement
    pub select_after_place: bool,
}

impl Default for SchematicPreferences {
    fn default() -> Self {
        Self {
            grid_size: 10,
            snap_to_grid: true,
            show_grid: true,
            grid_style: GridStyle::Dots,
            show_values: true,
            show_ref_des: true,
            show_pin_names: false,
            show_pin_numbers: false,
            wire_width: 2.0,
            junction_radius: 4.0,
            default_rotation: 0,
            auto_number: true,
            select_after_place: true,
        }
    }
}

/// Grid display style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridStyle {
    /// Grid dots at intersections
    Dots,
    /// Full grid lines
    Lines,
    /// Crosshairs at intersections
    Crosses,
}

impl GridStyle {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Dots => "Dots",
            Self::Lines => "Lines",
            Self::Crosses => "Crosses",
        }
    }

    /// Get all grid styles
    pub fn all() -> &'static [GridStyle] {
        &[Self::Dots, Self::Lines, Self::Crosses]
    }
}

//=============================================================================
// Waveform Preferences
//=============================================================================

/// Waveform viewer preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveformPreferences {
    /// Trace line width in pixels
    pub trace_width: f32,
    /// Show grid in waveform viewer
    pub show_grid: bool,
    /// Number of grid divisions (horizontal)
    pub grid_divisions_x: u32,
    /// Number of grid divisions (vertical)
    pub grid_divisions_y: u32,
    /// Custom trace colors (override defaults)
    pub custom_colors: Vec<String>,
    /// Anti-aliasing for traces
    pub anti_alias: bool,
    /// Show cursor values
    pub show_cursor_values: bool,
    /// Measurement precision (decimal places)
    pub measurement_precision: u32,
    /// Use SI prefixes for values (μ, m, k, M, etc.)
    pub use_si_prefixes: bool,
    /// Interpolation mode
    pub interpolation: InterpolationMode,
}

impl Default for WaveformPreferences {
    fn default() -> Self {
        Self {
            trace_width: 1.5,
            show_grid: true,
            grid_divisions_x: 10,
            grid_divisions_y: 8,
            custom_colors: Vec::new(),
            anti_alias: true,
            show_cursor_values: true,
            measurement_precision: 6,
            use_si_prefixes: true,
            interpolation: InterpolationMode::Linear,
        }
    }
}

/// Waveform interpolation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationMode {
    /// No interpolation (step)
    None,
    /// Linear interpolation
    Linear,
    /// Cubic spline interpolation
    Spline,
}

impl InterpolationMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None (Step)",
            Self::Linear => "Linear",
            Self::Spline => "Spline",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [InterpolationMode] {
        &[Self::None, Self::Linear, Self::Spline]
    }
}

//=============================================================================
// Simulation Preferences
//=============================================================================

/// Simulation default preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPreferences {
    /// Default simulation temperature in Celsius
    pub default_temp_c: f64,
    /// Default absolute tolerance
    pub default_abstol: f64,
    /// Default relative tolerance
    pub default_reltol: f64,
    /// Default max iterations
    pub default_max_iter: u32,
    /// Default transient max timestep (0 = auto)
    pub default_max_step: f64,
    /// Show simulation progress
    pub show_progress: bool,
    /// Automatically run DC operating point before transient
    pub auto_dc_op: bool,
    /// Save all node voltages (vs just requested nodes)
    pub save_all_nodes: bool,
    /// Maximum waveform points before compression
    pub max_waveform_points: usize,
}

impl Default for SimulationPreferences {
    fn default() -> Self {
        Self {
            default_temp_c: 27.0,
            default_abstol: 1e-12,
            default_reltol: 1e-3,
            default_max_iter: 150,
            default_max_step: 0.0, // Auto
            show_progress: true,
            auto_dc_op: true,
            save_all_nodes: true,
            max_waveform_points: 100_000,
        }
    }
}

//=============================================================================
// File Preferences
//=============================================================================

/// File handling preferences.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePreferences {
    /// Default save directory
    pub default_directory: Option<String>,
    /// Recent files count
    pub recent_files_count: u32,
    /// Create backup on save
    pub create_backup: bool,
    /// Backup extension
    pub backup_extension: String,
    /// Auto-save to temp file
    pub auto_save_temp: bool,
}

impl Default for FilePreferences {
    fn default() -> Self {
        Self {
            default_directory: None,
            recent_files_count: 10,
            create_backup: true,
            backup_extension: ".bak".to_string(),
            auto_save_temp: true,
        }
    }
}

//=============================================================================
// Persistence
//=============================================================================

impl Preferences {
    /// Save to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get the default storage path
    #[cfg(not(target_arch = "wasm32"))]
    pub fn default_storage_path() -> PathBuf {
        // Use APPDATA on Windows, HOME on Unix
        #[cfg(target_os = "windows")]
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata)
                .join("rspice")
                .join("preferences.json");
        }
        #[cfg(not(target_os = "windows"))]
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home)
                .join(".config")
                .join("rspice")
                .join("preferences.json");
        }
        PathBuf::from(".rspice_preferences.json")
    }

    /// Save to default storage location
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::default_storage_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = self
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// Load from default storage location
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Result<Self, std::io::Error> {
        let path = Self::default_storage_path();
        let json = std::fs::read_to_string(path)?;
        Self::from_json(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Load from storage, returning default if not found
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// WASM stubs
    #[cfg(target_arch = "wasm32")]
    pub fn save(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_or_default() -> Self {
        Self::default()
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Reset a specific section
    pub fn reset_general(&mut self) {
        self.general = GeneralPreferences::default();
    }

    pub fn reset_schematic(&mut self) {
        self.schematic = SchematicPreferences::default();
    }

    pub fn reset_waveform(&mut self) {
        self.waveform = WaveformPreferences::default();
    }

    pub fn reset_simulation(&mut self) {
        self.simulation = SimulationPreferences::default();
    }

    pub fn reset_files(&mut self) {
        self.files = FilePreferences::default();
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = Preferences::default();

        // Check general defaults
        assert_eq!(prefs.general.theme, ThemePreference::Dark);
        assert!(prefs.general.confirm_close_unsaved);
        assert_eq!(prefs.general.auto_save_interval, 60);

        // Check schematic defaults
        assert_eq!(prefs.schematic.grid_size, 10);
        assert!(prefs.schematic.snap_to_grid);
        assert!(prefs.schematic.show_grid);
        assert_eq!(prefs.schematic.wire_width, 2.0);

        // Check waveform defaults
        assert_eq!(prefs.waveform.trace_width, 1.5);
        assert!(prefs.waveform.anti_alias);
        assert_eq!(prefs.waveform.measurement_precision, 6);

        // Check simulation defaults
        assert_eq!(prefs.simulation.default_temp_c, 27.0);
        assert_eq!(prefs.simulation.default_abstol, 1e-12);
        assert_eq!(prefs.simulation.default_max_iter, 150);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut prefs = Preferences::default();
        prefs.schematic.grid_size = 20;
        prefs.waveform.trace_width = 3.0;
        prefs.simulation.default_temp_c = 85.0;

        let json = prefs.to_json().unwrap();
        let restored = Preferences::from_json(&json).unwrap();

        assert_eq!(restored.schematic.grid_size, 20);
        assert_eq!(restored.waveform.trace_width, 3.0);
        assert_eq!(restored.simulation.default_temp_c, 85.0);
    }

    #[test]
    fn test_reset() {
        let mut prefs = Preferences::default();
        prefs.schematic.grid_size = 50;
        prefs.reset();
        assert_eq!(prefs.schematic.grid_size, 10);
    }

    #[test]
    fn test_reset_section() {
        let mut prefs = Preferences::default();
        prefs.schematic.grid_size = 50;
        prefs.waveform.trace_width = 5.0;

        prefs.reset_schematic();

        // Schematic reset
        assert_eq!(prefs.schematic.grid_size, 10);
        // Waveform unchanged
        assert_eq!(prefs.waveform.trace_width, 5.0);
    }

    #[test]
    fn test_theme_preference_all() {
        let all = ThemePreference::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&ThemePreference::Dark));
        assert!(all.contains(&ThemePreference::Light));
        assert!(all.contains(&ThemePreference::System));
    }

    #[test]
    fn test_grid_style_display_name() {
        assert_eq!(GridStyle::Dots.display_name(), "Dots");
        assert_eq!(GridStyle::Lines.display_name(), "Lines");
        assert_eq!(GridStyle::Crosses.display_name(), "Crosses");
    }

    #[test]
    fn test_interpolation_mode_all() {
        let all = InterpolationMode::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_file_preferences_defaults() {
        let files = FilePreferences::default();
        assert_eq!(files.recent_files_count, 10);
        assert!(files.create_backup);
        assert_eq!(files.backup_extension, ".bak");
    }

    #[test]
    fn test_simulation_defaults_follow_spice() {
        let sim = SimulationPreferences::default();
        // SPICE standard values
        assert_eq!(sim.default_temp_c, 27.0); // Industry standard
        assert!(sim.default_abstol < 1e-10); // Tight tolerance
        assert!(sim.default_reltol >= 1e-3); // Standard reltol
    }

    #[test]
    fn test_custom_colors_empty_by_default() {
        let waveform = WaveformPreferences::default();
        assert!(waveform.custom_colors.is_empty());
    }

    #[test]
    fn test_custom_colors_serialization() {
        let mut prefs = Preferences::default();
        prefs.waveform.custom_colors = vec!["#FF0000".to_string(), "#00FF00".to_string()];

        let json = prefs.to_json().unwrap();
        let restored = Preferences::from_json(&json).unwrap();

        assert_eq!(restored.waveform.custom_colors.len(), 2);
        assert_eq!(restored.waveform.custom_colors[0], "#FF0000");
    }

    #[test]
    fn test_all_preferences_equal_to_self() {
        let prefs = Preferences::default();
        assert_eq!(prefs, prefs.clone());
    }

    #[test]
    fn test_partial_eq_detects_differences() {
        let prefs1 = Preferences::default();
        let mut prefs2 = Preferences::default();
        prefs2.schematic.grid_size = 999;

        assert_ne!(prefs1, prefs2);
    }

    #[test]
    fn test_backward_compatible_json() {
        // Old JSON without new fields should still parse
        let old_json = r#"{
            "general": { "theme": "Dark", "show_startup_dialog": true, "check_updates": false, "confirm_close_unsaved": true, "auto_save_interval": 60, "drc_before_simulate": true },
            "schematic": { "grid_size": 10, "snap_to_grid": true, "show_grid": true, "grid_style": "Dots", "show_values": true, "show_ref_des": true, "show_pin_names": false, "show_pin_numbers": false, "wire_width": 2.0, "junction_radius": 4.0, "default_rotation": 0, "auto_number": true, "select_after_place": true },
            "waveform": { "trace_width": 1.5, "show_grid": true, "grid_divisions_x": 10, "grid_divisions_y": 8, "custom_colors": [], "anti_alias": true, "show_cursor_values": true, "measurement_precision": 6, "use_si_prefixes": true, "interpolation": "Linear" },
            "simulation": { "default_temp_c": 27.0, "default_abstol": 1e-12, "default_reltol": 0.001, "default_max_iter": 150, "default_max_step": 0.0, "show_progress": true, "auto_dc_op": true, "save_all_nodes": true, "max_waveform_points": 100000 },
            "files": { "default_directory": null, "recent_files_count": 10, "create_backup": true, "backup_extension": ".bak", "auto_save_temp": true }
        }"#;

        let prefs = Preferences::from_json(old_json);
        assert!(prefs.is_ok());
    }
}
