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
#[serde(default, deny_unknown_fields)]
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
#[serde(default, deny_unknown_fields)]
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
#[serde(default, deny_unknown_fields)]
pub struct OutputConfig {
    /// Default output format
    pub format: String,

    /// Show progress by default
    pub show_progress: bool,

    /// Default output directory (None = same as input)
    pub output_directory: Option<PathBuf>,
}

/// Path-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
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
            output_directory: None,
        }
    }
}

/// One configuration file's contents: every field optional, so a layer
/// only overrides what it actually sets. This is what distinguishes
/// "explicitly set back to the default" from "not mentioned" — comparing
/// concrete values against defaults cannot.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ConfigLayer {
    simulation: SimulationLayer,
    output: OutputLayer,
    paths: PathConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct SimulationLayer {
    temperature: Option<f64>,
    max_iterations: Option<usize>,
    abstol: Option<f64>,
    reltol: Option<f64>,
    residual_reltol: Option<f64>,
    min_timestep: Option<f64>,
    max_timestep: Option<f64>,
    compress_waveforms: Option<bool>,
    compression_tolerance: Option<f64>,
    convergence_mode: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct OutputLayer {
    format: Option<String>,
    show_progress: Option<bool>,
    output_directory: Option<PathBuf>,
}

impl Config {
    /// Load configuration from default locations
    ///
    /// Configuration is loaded in order of priority (lowest to highest):
    /// 1. Default values
    /// 2. User config (~/.config/rspice/config.toml or ~/.rspicerc)
    /// 3. Project config (./.rspicerc)
    /// 4. Environment variables
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::default();

        if let Some(user_layer) = Self::load_user_layer()? {
            config.apply_layer(user_layer);
        }
        if let Some(project_layer) = Self::load_project_layer()? {
            config.apply_layer(project_layer);
        }

        config.apply_env()?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from a specific file (applied over defaults)
    pub fn load_file(path: &std::path::Path) -> Result<Self, ConfigError> {
        let mut config = Config::default();
        config.apply_layer(Self::load_layer(path)?);
        config.apply_env()?;
        config.validate()?;
        Ok(config)
    }

    fn load_layer(path: &std::path::Path) -> Result<ConfigLayer, ConfigError> {
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
    fn load_user_layer() -> Result<Option<ConfigLayer>, ConfigError> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("rspice").join("config.toml");
            if config_path.exists() {
                return Self::load_layer(&config_path).map(Some);
            }
        }

        if let Some(home) = dirs::home_dir() {
            let rc_path = home.join(".rspicerc");
            if rc_path.exists() {
                return Self::load_layer(&rc_path).map(Some);
            }
        }

        Ok(None)
    }

    /// Load project configuration from current directory
    fn load_project_layer() -> Result<Option<ConfigLayer>, ConfigError> {
        let rc_path = PathBuf::from(".rspicerc");
        if rc_path.exists() {
            return Self::load_layer(&rc_path).map(Some);
        }
        Ok(None)
    }

    /// Apply one file layer; only the fields the file set are overridden.
    fn apply_layer(&mut self, layer: ConfigLayer) {
        let sim = layer.simulation;
        if let Some(v) = sim.temperature {
            self.simulation.temperature = v;
        }
        if let Some(v) = sim.max_iterations {
            self.simulation.max_iterations = v;
        }
        if let Some(v) = sim.abstol {
            self.simulation.abstol = v;
        }
        if let Some(v) = sim.reltol {
            self.simulation.reltol = v;
        }
        if let Some(v) = sim.residual_reltol {
            self.simulation.residual_reltol = v;
        }
        if let Some(v) = sim.min_timestep {
            self.simulation.min_timestep = v;
        }
        if let Some(v) = sim.max_timestep {
            self.simulation.max_timestep = v;
        }
        if let Some(v) = sim.compress_waveforms {
            self.simulation.compress_waveforms = v;
        }
        if let Some(v) = sim.compression_tolerance {
            self.simulation.compression_tolerance = v;
        }
        if let Some(v) = sim.convergence_mode {
            self.simulation.convergence_mode = v;
        }

        let out = layer.output;
        if let Some(v) = out.format {
            self.output.format = v;
        }
        if let Some(v) = out.show_progress {
            self.output.show_progress = v;
        }
        if let Some(v) = out.output_directory {
            self.output.output_directory = Some(v);
        }

        self.paths.include_paths.extend(layer.paths.include_paths);
        self.paths.library_paths.extend(layer.paths.library_paths);
        self.paths
            .veriloga_includes
            .extend(layer.paths.veriloga_includes);
    }

    /// Apply environment variable overrides
    fn apply_env(&mut self) -> Result<(), ConfigError> {
        if let Ok(temp) = std::env::var("RSPICE_TEMPERATURE") {
            let t = temp
                .parse::<f64>()
                .map_err(|e| ConfigError::EnvironmentError {
                    variable: "RSPICE_TEMPERATURE".to_string(),
                    value: temp.clone(),
                    message: e.to_string(),
                })?;
            if !t.is_finite() {
                return Err(ConfigError::EnvironmentError {
                    variable: "RSPICE_TEMPERATURE".to_string(),
                    value: temp,
                    message: "temperature must be finite".to_string(),
                });
            }
            self.simulation.temperature = t;
        }

        if let Ok(format) = std::env::var("RSPICE_OUTPUT_FORMAT") {
            self.output.format = format;
        }

        // Path lists use the platform separator (';' on Windows, ':' elsewhere)
        if let Some(includes) = std::env::var_os("RSPICE_INCLUDE_PATH") {
            self.paths
                .include_paths
                .extend(std::env::split_paths(&includes));
        }

        if let Some(libs) = std::env::var_os("RSPICE_LIBRARY_PATH") {
            self.paths
                .library_paths
                .extend(std::env::split_paths(&libs));
        }

        Ok(())
    }

    fn validate(&self) -> Result<(), ConfigError> {
        validate_celsius("simulation.temperature", self.simulation.temperature)?;
        validate_positive_usize("simulation.max_iterations", self.simulation.max_iterations)?;
        validate_positive("simulation.abstol", self.simulation.abstol)?;
        validate_positive("simulation.reltol", self.simulation.reltol)?;
        validate_positive(
            "simulation.residual_reltol",
            self.simulation.residual_reltol,
        )?;
        validate_positive("simulation.min_timestep", self.simulation.min_timestep)?;
        validate_positive("simulation.max_timestep", self.simulation.max_timestep)?;
        if self.simulation.min_timestep > self.simulation.max_timestep {
            return Err(ConfigError::InvalidValue {
                field: "simulation.min_timestep".to_string(),
                value: self.simulation.min_timestep.to_string(),
                message: format!(
                    "must be <= simulation.max_timestep ({})",
                    self.simulation.max_timestep
                ),
            });
        }
        validate_positive(
            "simulation.compression_tolerance",
            self.simulation.compression_tolerance,
        )?;
        if !matches!(
            self.simulation.convergence_mode.as_str(),
            "fast" | "default" | "robust"
        ) {
            return Err(ConfigError::InvalidValue {
                field: "simulation.convergence_mode".to_string(),
                value: self.simulation.convergence_mode.clone(),
                message: "must be one of fast, default, robust".to_string(),
            });
        }
        if !matches!(
            self.output.format.to_ascii_lowercase().as_str(),
            "raw" | "ascii" | "csv" | "json" | "tsv" | "hdf5"
        ) {
            return Err(ConfigError::InvalidValue {
                field: "output.format".to_string(),
                value: self.output.format.clone(),
                message: "must be one of raw, ascii, csv, json, tsv, hdf5".to_string(),
            });
        }
        Ok(())
    }
}

fn validate_positive(field: &str, value: f64) -> Result<(), ConfigError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(ConfigError::InvalidValue {
            field: field.to_string(),
            value: value.to_string(),
            message: "must be a positive finite number".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_usize(field: &str, value: usize) -> Result<(), ConfigError> {
    if value == 0 {
        return Err(ConfigError::InvalidValue {
            field: field.to_string(),
            value: value.to_string(),
            message: "must be at least 1".to_string(),
        });
    }
    Ok(())
}

fn validate_celsius(field: &str, value: f64) -> Result<(), ConfigError> {
    if !value.is_finite() || value <= -273.15 {
        return Err(ConfigError::InvalidValue {
            field: field.to_string(),
            value: value.to_string(),
            message: "must be finite and above absolute zero".to_string(),
        });
    }
    Ok(())
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

    #[error("Invalid environment variable {variable}={value:?}: {message}")]
    EnvironmentError {
        variable: String,
        value: String,
        message: String,
    },

    #[error("Invalid config value {field}={value:?}: {message}")]
    InvalidValue {
        field: String,
        value: String,
        message: String,
    },
}
