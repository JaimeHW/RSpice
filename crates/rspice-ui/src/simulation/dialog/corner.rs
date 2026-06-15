//! Corner Analysis Configuration
//!
//! Configuration for PVT (Process, Voltage, Temperature) corner analysis.
//! Corner analysis runs simulations across multiple operating conditions
//! to verify design robustness.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Standard process corners (TT, SS, FF, SF, FS)
//! - Voltage corner sweep
//! - Temperature corner sweep
//! - Full matrix or diagonal sweep modes
//! - Summary statistics and worst-case identification
//!
//! # Example Usage
//!
//! Run transient analysis at SS corner with reduced voltage and hot temperature.

use super::options::parse_si_value;

// =============================================================================
// Process Corner
// =============================================================================

/// Standard process corner types
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ProcessCorner {
    /// Typical-Typical (nominal)
    #[default]
    TT,
    /// Slow-Slow (worst delay)
    SS,
    /// Fast-Fast (worst power)
    FF,
    /// Slow-Fast (skewed)
    SF,
    /// Fast-Slow (skewed)
    FS,
}

impl ProcessCorner {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::TT => "TT (Typical)",
            Self::SS => "SS (Slow-Slow)",
            Self::FF => "FF (Fast-Fast)",
            Self::SF => "SF (Slow-Fast)",
            Self::FS => "FS (Fast-Slow)",
        }
    }

    /// Short name
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
        }
    }

    /// All corners
    pub fn all() -> &'static [ProcessCorner] {
        &[Self::TT, Self::SS, Self::FF, Self::SF, Self::FS]
    }

    /// Standard 5-corner set
    pub fn standard_five() -> Vec<ProcessCorner> {
        vec![Self::TT, Self::SS, Self::FF, Self::SF, Self::FS]
    }

    /// Speed corners only (SS, TT, FF)
    pub fn speed_corners() -> Vec<ProcessCorner> {
        vec![Self::SS, Self::TT, Self::FF]
    }
}

// =============================================================================
// Base Analysis Type
// =============================================================================

/// Base analysis to run at each corner
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CornerBaseAnalysis {
    /// Transient analysis
    #[default]
    Transient,
    /// AC analysis
    Ac,
    /// DC analysis
    Dc,
    /// Operating point
    Op,
}

impl CornerBaseAnalysis {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Transient => "Transient",
            Self::Ac => "AC",
            Self::Dc => "DC",
            Self::Op => "Operating Point",
        }
    }

    /// All types
    pub fn all() -> &'static [CornerBaseAnalysis] {
        &[Self::Transient, Self::Ac, Self::Dc, Self::Op]
    }
}

// =============================================================================
// Corner Configuration
// =============================================================================

/// Corner analysis configuration
#[derive(Debug, Clone)]
pub struct CornerConfig {
    /// Process corners to simulate
    pub process_corners: Vec<ProcessCorner>,
    /// Voltage values to sweep (V)
    pub voltages: Vec<f64>,
    /// Temperature values to sweep (°C)
    pub temperatures: Vec<f64>,
    /// Use Kelvin for temperatures
    pub temp_in_kelvin: bool,
    /// Full matrix (all combinations) or diagonal sweep
    pub full_matrix: bool,
    /// Base analysis type
    pub base_analysis: CornerBaseAnalysis,
    /// Enabled (run corner analysis)
    pub enabled: bool,
}

impl Default for CornerConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![ProcessCorner::TT],
            voltages: vec![1.0],
            temperatures: vec![25.0],
            temp_in_kelvin: false,
            full_matrix: true,
            base_analysis: CornerBaseAnalysis::Transient,
            enabled: true,
        }
    }
}

impl CornerConfig {
    /// Create with standard 5 process corners
    pub fn standard_corners() -> Self {
        Self {
            process_corners: ProcessCorner::standard_five(),
            ..Default::default()
        }
    }

    /// Create typical commercial PVT setup
    pub fn commercial_pvt() -> Self {
        Self {
            process_corners: ProcessCorner::speed_corners(),
            voltages: vec![0.9, 1.0, 1.1],          // ±10%
            temperatures: vec![-40.0, 25.0, 125.0], // Mil-spec range
            full_matrix: true,
            ..Default::default()
        }
    }

    /// Set process corners
    pub fn with_process_corners(mut self, corners: Vec<ProcessCorner>) -> Self {
        self.process_corners = corners;
        self
    }

    /// Set voltage sweep
    pub fn with_voltages(mut self, voltages: Vec<f64>) -> Self {
        self.voltages = voltages;
        self
    }

    /// Set temperature sweep
    pub fn with_temperatures(mut self, temps: Vec<f64>) -> Self {
        self.temperatures = temps;
        self
    }

    /// Set to diagonal sweep (matched corners)
    pub fn diagonal(mut self) -> Self {
        self.full_matrix = false;
        self
    }

    /// Set base analysis
    pub fn with_base_analysis(mut self, analysis: CornerBaseAnalysis) -> Self {
        self.base_analysis = analysis;
        self
    }

    /// Total number of corners
    pub fn num_corners(&self) -> usize {
        if self.full_matrix {
            self.process_corners.len() * self.voltages.len() * self.temperatures.len()
        } else {
            self.process_corners
                .len()
                .max(self.voltages.len())
                .max(self.temperatures.len())
        }
    }

    /// Generate corner names
    pub fn corner_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if self.full_matrix {
            for p in &self.process_corners {
                for v in &self.voltages {
                    for t in &self.temperatures {
                        names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
                    }
                }
            }
        } else {
            let n = self.num_corners();
            for i in 0..n {
                let p = self
                    .process_corners
                    .get(i % self.process_corners.len())
                    .copied()
                    .unwrap_or(ProcessCorner::TT);
                let v = self
                    .voltages
                    .get(i % self.voltages.len())
                    .copied()
                    .unwrap_or(1.0);
                let t = self
                    .temperatures
                    .get(i % self.temperatures.len())
                    .copied()
                    .unwrap_or(25.0);
                names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
            }
        }

        names
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("At least one process corner required".to_string());
        }

        if self.voltages.is_empty() {
            return Err("At least one voltage value required".to_string());
        }

        for v in &self.voltages {
            if *v <= 0.0 {
                return Err("Voltage values must be positive".to_string());
            }
        }

        if self.temperatures.is_empty() {
            return Err("At least one temperature value required".to_string());
        }

        // Check temperature range (assuming Celsius)
        if !self.temp_in_kelvin {
            for t in &self.temperatures {
                if *t < -273.15 {
                    return Err("Temperature cannot be below absolute zero".to_string());
                }
            }
        }

        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with UI buffers
#[derive(Debug, Clone, Default)]
pub struct CornerDialogState {
    /// Selected process corners (as bit flags or indices)
    pub process_tt: bool,
    pub process_ss: bool,
    pub process_ff: bool,
    pub process_sf: bool,
    pub process_fs: bool,
    /// Voltage values buffer
    pub voltage_min: String,
    pub voltage_nom: String,
    pub voltage_max: String,
    pub enable_voltage_sweep: bool,
    /// Temperature values buffer
    pub temp_cold: String,
    pub temp_room: String,
    pub temp_hot: String,
    pub enable_temp_sweep: bool,
    /// Full matrix mode
    pub full_matrix: bool,
    /// Base analysis type index
    pub base_analysis_idx: usize,
    /// Initialized flag
    pub initialized: bool,
}

impl CornerDialogState {
    /// Initialize from config
    pub fn from_config(config: &CornerConfig) -> Self {
        let has_voltage_sweep = config.voltages.len() > 1;
        let has_temp_sweep = config.temperatures.len() > 1;

        Self {
            process_tt: config.process_corners.contains(&ProcessCorner::TT),
            process_ss: config.process_corners.contains(&ProcessCorner::SS),
            process_ff: config.process_corners.contains(&ProcessCorner::FF),
            process_sf: config.process_corners.contains(&ProcessCorner::SF),
            process_fs: config.process_corners.contains(&ProcessCorner::FS),
            voltage_min: config
                .voltages
                .first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0.9".to_string()),
            voltage_nom: config
                .voltages
                .get(1)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "1.0".to_string()),
            voltage_max: config
                .voltages
                .last()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "1.1".to_string()),
            enable_voltage_sweep: has_voltage_sweep,
            temp_cold: config
                .temperatures
                .first()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-40".to_string()),
            temp_room: config
                .temperatures
                .get(1)
                .map(|t| t.to_string())
                .unwrap_or_else(|| "25".to_string()),
            temp_hot: config
                .temperatures
                .last()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "125".to_string()),
            enable_temp_sweep: has_temp_sweep,
            full_matrix: config.full_matrix,
            base_analysis_idx: match config.base_analysis {
                CornerBaseAnalysis::Transient => 0,
                CornerBaseAnalysis::Ac => 1,
                CornerBaseAnalysis::Dc => 2,
                CornerBaseAnalysis::Op => 3,
            },
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<CornerConfig, String> {
        // Process corners
        let mut process_corners = Vec::new();
        if self.process_tt {
            process_corners.push(ProcessCorner::TT);
        }
        if self.process_ss {
            process_corners.push(ProcessCorner::SS);
        }
        if self.process_ff {
            process_corners.push(ProcessCorner::FF);
        }
        if self.process_sf {
            process_corners.push(ProcessCorner::SF);
        }
        if self.process_fs {
            process_corners.push(ProcessCorner::FS);
        }

        if process_corners.is_empty() {
            return Err("Select at least one process corner".to_string());
        }

        // Voltages
        let voltages = if self.enable_voltage_sweep {
            let vmin: f64 = parse_si_value(&self.voltage_min).map_err(|_| "Invalid min voltage")?;
            let vnom: f64 = parse_si_value(&self.voltage_nom).map_err(|_| "Invalid nom voltage")?;
            let vmax: f64 = parse_si_value(&self.voltage_max).map_err(|_| "Invalid max voltage")?;
            vec![vmin, vnom, vmax]
        } else {
            let vnom: f64 = parse_si_value(&self.voltage_nom).map_err(|_| "Invalid voltage")?;
            vec![vnom]
        };

        // Temperatures
        let temperatures = if self.enable_temp_sweep {
            let tcold: f64 = self.temp_cold.parse().map_err(|_| "Invalid cold temp")?;
            let troom: f64 = self.temp_room.parse().map_err(|_| "Invalid room temp")?;
            let thot: f64 = self.temp_hot.parse().map_err(|_| "Invalid hot temp")?;
            vec![tcold, troom, thot]
        } else {
            let troom: f64 = self.temp_room.parse().map_err(|_| "Invalid temperature")?;
            vec![troom]
        };

        let base_analysis = match self.base_analysis_idx {
            0 => CornerBaseAnalysis::Transient,
            1 => CornerBaseAnalysis::Ac,
            2 => CornerBaseAnalysis::Dc,
            _ => CornerBaseAnalysis::Op,
        };

        let config = CornerConfig {
            process_corners,
            voltages,
            temperatures,
            temp_in_kelvin: false,
            full_matrix: self.full_matrix,
            base_analysis,
            enabled: true,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&CornerConfig::commercial_pvt());
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
