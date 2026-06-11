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
            // Matches the engine's MIN_TIMESTEP: a deeper floor only lets
            // failing timepoints grind a thousand times longer through the
            // dt-cut ladder before the rescue machinery fires.
            min_timestep: 1e-12,
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
        if other.simulation.min_timestep != SimulationConfig::default().min_timestep {
            self.simulation.min_timestep = other.simulation.min_timestep;
        }
        if other.simulation.max_timestep != SimulationConfig::default().max_timestep {
            self.simulation.max_timestep = other.simulation.max_timestep;
        }
        self.simulation.compress_waveforms |= other.simulation.compress_waveforms;
        if other.simulation.compression_tolerance != SimulationConfig::default().compression_tolerance {
            self.simulation.compression_tolerance = other.simulation.compression_tolerance;
        }
        if other.simulation.convergence_mode != SimulationConfig::default().convergence_mode {
            self.simulation.convergence_mode = other.simulation.convergence_mode.clone();
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

        // Path lists use the platform separator (';' on Windows, ':' elsewhere)
        if let Some(includes) = std::env::var_os("RSPICE_INCLUDE_PATH") {
            self.paths.include_paths.extend(std::env::split_paths(&includes));
        }

        if let Some(libs) = std::env::var_os("RSPICE_LIBRARY_PATH") {
            self.paths.library_paths.extend(std::env::split_paths(&libs));
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
