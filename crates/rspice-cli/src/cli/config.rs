//! Configuration File Support
//!
//! Handles loading and merging configuration from:
//! 1. Default values
//! 2. System-wide config (/etc/rspice/config.toml on Unix)
//! 3. User config (~/.config/rspice/config.toml or ~/.rspicerc)
//! 4. Project config (./.rspicerc)
//! 5. Environment variables (RSPICE_*)
//! 6. Command-line arguments (highest priority)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the RSpice CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Config {
    /// Simulation settings
    pub simulation: SimulationConfig,

    /// Output settings
    pub output: OutputConfig,

    /// Path settings
    pub paths: PathConfig,
}

/// Simulation-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SimulationConfig {
    /// Default temperature in Celsius
    pub temperature: f64,

    /// Maximum Newton-Raphson iterations
    pub max_iterations: usize,

    /// Convergence tolerance
    pub abstol: f64,

    /// Relative tolerance
    pub reltol: f64,

    /// Relative residual tolerance for equation convergence checks
    pub residual_reltol: f64,

    /// Minimum timestep for transient
    pub min_timestep: f64,

    /// Maximum timestep for transient
    pub max_timestep: f64,

    /// Enable waveform compression by default
    pub compress_waveforms: bool,

    /// Compression tolerance
    pub compression_tolerance: f64,

    /// Convergence mode: "fast", "default", or "robust"
    pub convergence_mode: String,
}

/// Output-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    /// Default output format
    pub format: String,

    /// Show progress by default
    pub show_progress: bool,

    /// Include node names in output
    pub include_node_names: bool,

    /// Default output directory (None = same as input)
    pub output_directory: Option<PathBuf>,
}

/// Path-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct PathConfig {
    /// Include paths for .include directives
    pub include_paths: Vec<PathBuf>,

    /// Model library paths
    pub library_paths: Vec<PathBuf>,

    /// Verilog-A include paths
    pub veriloga_includes: Vec<PathBuf>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            temperature: 27.0,
            max_iterations: 50,
            abstol: 1e-12,
            reltol: 1e-3,
            residual_reltol: 1e-3,
            min_timestep: 1e-15,
            max_timestep: 1e-3,
            compress_waveforms: false,
            compression_tolerance: 1e-4,
            convergence_mode: "default".to_string(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: "raw".to_string(),
            show_progress: false,
            include_node_names: false,
            output_directory: None,
        }
    }
}

impl Config {
    /// Load configuration from default locations
    ///
    /// Configuration is loaded in order of priority (lowest to highest):
    /// 1. Default values
    /// 2. User config (~/.rspicerc or ~/.config/rspice/config.toml)
    /// 3. Project config (./.rspicerc)
    /// 4. Environment variables
    pub fn load() -> Self {
        let mut config = Config::default();

        // Try to load user config
        if let Some(user_config) = Self::load_user_config() {
            config = config.merge(user_config);
        }

        // Try to load project config
        if let Some(project_config) = Self::load_project_config() {
            config = config.merge(project_config);
        }

        // Apply environment variables
        config.apply_env();

        config
    }

    /// Load configuration from a specific file
    pub fn load_file(path: &std::path::Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(|e| ConfigError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| ConfigError::ParseError {
            path: path.to_path_buf(),
            message: e.to_string(),
        })
    }

    /// Load user configuration from standard locations
    fn load_user_config() -> Option<Self> {
        // Try ~/.config/rspice/config.toml first
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("rspice").join("config.toml");
            if config_path.exists()
                && let Ok(config) = Self::load_file(&config_path)
            {
                return Some(config);
            }
        }

        // Try ~/.rspicerc
        if let Some(home) = dirs::home_dir() {
            let rc_path = home.join(".rspicerc");
            if rc_path.exists()
                && let Ok(config) = Self::load_file(&rc_path)
            {
                return Some(config);
            }
        }

        None
    }

    /// Load project configuration from current directory
    fn load_project_config() -> Option<Self> {
        let rc_path = PathBuf::from(".rspicerc");
        if rc_path.exists()
            && let Ok(config) = Self::load_file(&rc_path)
        {
            return Some(config);
        }
        None
    }

    /// Merge another config into this one (other takes priority)
    fn merge(mut self, other: Self) -> Self {
        // Simulation settings - only override if non-default
        if other.simulation.temperature != SimulationConfig::default().temperature {
            self.simulation.temperature = other.simulation.temperature;
        }
        if other.simulation.max_iterations != SimulationConfig::default().max_iterations {
            self.simulation.max_iterations = other.simulation.max_iterations;
        }
        if other.simulation.abstol != SimulationConfig::default().abstol {
            self.simulation.abstol = other.simulation.abstol;
        }
        if other.simulation.reltol != SimulationConfig::default().reltol {
            self.simulation.reltol = other.simulation.reltol;
        }
        if other.simulation.residual_reltol != SimulationConfig::default().residual_reltol {
            self.simulation.residual_reltol = other.simulation.residual_reltol;
        }

        // Output settings
        if other.output.format != OutputConfig::default().format {
            self.output.format = other.output.format;
        }
        self.output.show_progress |= other.output.show_progress;
        self.output.include_node_names |= other.output.include_node_names;
        if other.output.output_directory.is_some() {
            self.output.output_directory = other.output.output_directory;
        }

        // Paths - append
        self.paths.include_paths.extend(other.paths.include_paths);
        self.paths.library_paths.extend(other.paths.library_paths);
        self.paths
            .veriloga_includes
            .extend(other.paths.veriloga_includes);

        self
    }

    /// Apply environment variable overrides
    fn apply_env(&mut self) {
        if let Ok(temp) = std::env::var("RSPICE_TEMPERATURE")
            && let Ok(t) = temp.parse()
        {
            self.simulation.temperature = t;
        }

        if let Ok(format) = std::env::var("RSPICE_OUTPUT_FORMAT") {
            self.output.format = format;
        }

        if let Ok(includes) = std::env::var("RSPICE_INCLUDE_PATH") {
            for path in includes.split(';') {
                self.paths.include_paths.push(PathBuf::from(path));
            }
        }

        if let Ok(libs) = std::env::var("RSPICE_LIBRARY_PATH") {
            for path in libs.split(';') {
                self.paths.library_paths.push(PathBuf::from(path));
            }
        }
    }
}

/// Configuration loading errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {path}")]
    IoError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse config file {path}: {message}")]
    ParseError { path: PathBuf, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.simulation.temperature, 27.0);
        assert_eq!(config.simulation.max_iterations, 50);
        assert_eq!(config.simulation.residual_reltol, 1e-3);
        assert_eq!(config.output.format, "raw");
    }

    #[test]
    fn test_config_merge() {
        let base = Config::default();
        let mut override_config = Config::default();
        override_config.simulation.temperature = 85.0;
        override_config.simulation.residual_reltol = 2e-4;
        override_config.output.format = "csv".to_string();
        override_config
            .paths
            .include_paths
            .push(PathBuf::from("/models"));

        let merged = base.merge(override_config);
        assert_eq!(merged.simulation.temperature, 85.0);
        assert_eq!(merged.simulation.residual_reltol, 2e-4);
        assert_eq!(merged.output.format, "csv");
        assert_eq!(merged.paths.include_paths.len(), 1);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("temperature"));
        assert!(toml_str.contains("format"));

        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.simulation.temperature, config.simulation.temperature);
    }
}
