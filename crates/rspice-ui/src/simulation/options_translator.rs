//! Options Translator
//!
//! Converts UI convergence and simulation options to rspice-core format.
//! Provides a clean interface for passing settings to the engine.
//!
//! # Features
//!
//! - ConvergenceOptions to engine config translation
//! - Temperature sweep configuration
//! - Tolerance and iteration settings
//! - Method selection (GEAR, Trapezoidal, etc.)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Convergence Options Translation
// =============================================================================

/// Translated simulation options for the engine
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EngineOptions {
    /// Absolute current tolerance (A)
    pub abstol: f64,
    /// Relative tolerance
    pub reltol: f64,
    /// Voltage tolerance
    pub vntol: f64,
    /// Charge tolerance
    pub chgtol: f64,
    /// GMIN conductance
    pub gmin: f64,
    /// Temperature (Celsius)
    pub temp: f64,
    /// Nominal temperature
    pub tnom: f64,
    /// Maximum iterations
    pub itl1: u32,
    /// DC iteration limit
    pub itl2: u32,
    /// Transient iteration limit per point
    pub itl4: u32,
    /// Integration method
    pub method: IntegrationMethod,
    /// Use GMIN stepping
    pub gmin_stepping: bool,
    /// Use source stepping
    pub source_stepping: bool,
    /// Maximum transient step size
    pub max_step: Option<f64>,
    /// Additional options
    pub custom: HashMap<String, String>,
}

/// Integration method for transient analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IntegrationMethod {
    /// Trapezoidal method (default)
    #[default]
    Trapezoidal,
    /// Backward Euler
    BackwardEuler,
    /// Gear method (stiff systems)
    Gear,
    /// Gear 2nd order
    Gear2,
}

impl IntegrationMethod {
    /// Convert to engine keyword
    pub fn to_engine_keyword(&self) -> &'static str {
        match self {
            IntegrationMethod::Trapezoidal => "trap",
            IntegrationMethod::BackwardEuler => "euler",
            IntegrationMethod::Gear => "gear",
            IntegrationMethod::Gear2 => "gear2only",
        }
    }

    /// Convert to string for SPICE
    pub fn to_spice(&self) -> &'static str {
        match self {
            IntegrationMethod::Trapezoidal => "TRAP",
            IntegrationMethod::BackwardEuler => "EULER",
            IntegrationMethod::Gear => "GEAR",
            IntegrationMethod::Gear2 => "GEAR",
        }
    }
}

impl EngineOptions {
    /// Create with robust default settings for the simulation engine.
    pub fn engine_defaults() -> Self {
        Self {
            abstol: 1e-12,
            reltol: 1e-3,
            vntol: 1e-6,
            chgtol: 1e-14,
            gmin: 1e-12,
            temp: 27.0,
            tnom: 27.0,
            itl1: 100,
            itl2: 50,
            itl4: 10,
            method: IntegrationMethod::Trapezoidal,
            gmin_stepping: true,
            source_stepping: false,
            max_step: None,
            custom: HashMap::new(),
        }
    }

    /// Set temperature
    pub fn with_temp(mut self, temp: f64) -> Self {
        self.temp = temp;
        self
    }

    /// Set tolerances
    pub fn with_tolerances(mut self, reltol: f64, abstol: f64, vntol: f64) -> Self {
        self.reltol = reltol;
        self.abstol = abstol;
        self.vntol = vntol;
        self
    }

    /// Enable tighter tolerances for precision
    pub fn high_precision(mut self) -> Self {
        self.reltol = 1e-6;
        self.abstol = 1e-15;
        self.vntol = 1e-9;
        self.itl1 = 200;
        self.itl4 = 20;
        self
    }

    /// Enable relaxed tolerances for speed
    pub fn fast_mode(mut self) -> Self {
        self.reltol = 1e-2;
        self.abstol = 1e-10;
        self.vntol = 1e-4;
        self.itl1 = 50;
        self
    }

    /// Generate engine options string
    pub fn to_engine_options_string(&self) -> String {
        let mut opts = Vec::new();
        opts.push(format!("temp={}", self.temp));
        opts.push(format!("tnom={}", self.tnom));
        opts.push(format!("reltol={}", self.reltol));
        opts.push(format!("abstol={}", self.abstol));
        opts.push(format!("vntol={}", self.vntol));
        opts.push(format!("gmin={}", self.gmin));
        opts.push(format!("method={}", self.method.to_engine_keyword()));

        if self.gmin_stepping {
            opts.push("gmin_stepping=yes".to_string());
        }
        if self.source_stepping {
            opts.push("source_stepping=yes".to_string());
        }
        if let Some(step) = self.max_step {
            opts.push(format!("maxstep={}", step));
        }

        opts.join(" ")
    }

    /// Generate SPICE .OPTIONS string
    pub fn to_spice_options(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            ".OPTIONS RELTOL={} ABSTOL={} VNTOL={}",
            self.reltol, self.abstol, self.vntol
        ));
        lines.push(format!(
            ".OPTIONS GMIN={} ITL1={} ITL2={} ITL4={}",
            self.gmin, self.itl1, self.itl2, self.itl4
        ));
        lines.push(format!(
            ".OPTIONS METHOD={} TEMP={} TNOM={}",
            self.method.to_spice(),
            self.temp,
            self.tnom
        ));
        let mut custom_options: Vec<_> = self.custom.iter().collect();
        custom_options.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
        for (key, value) in custom_options {
            let key = key.trim();
            if key.is_empty() {
                continue;
            }
            let key_upper = key.to_ascii_uppercase();
            let value = value.trim();
            if value.is_empty() {
                lines.push(format!(".OPTIONS {}", key_upper));
            } else {
                lines.push(format!(".OPTIONS {}={}", key_upper, value));
            }
        }

        lines.join("\n")
    }
}

// =============================================================================
// Options Translator
// =============================================================================

/// Translator between UI options and engine options
#[derive(Debug, Clone, Default)]
pub struct OptionsTranslator {
    /// Base options
    base: EngineOptions,
}

impl OptionsTranslator {
    /// Create new translator with defaults
    pub fn new() -> Self {
        Self {
            base: EngineOptions::engine_defaults(),
        }
    }

    /// Translate from convergence options
    pub fn from_convergence(&self, conv: &super::convergence::ConvergenceOptions) -> EngineOptions {
        let mut opts = self.base.clone();

        opts.reltol = conv.tolerances.reltol;
        opts.abstol = conv.tolerances.abstol;
        opts.vntol = conv.tolerances.vntol;
        opts.chgtol = conv.tolerances.chgtol;
        opts.gmin = conv.gmin.gmin_final;
        opts.gmin_stepping = conv.gmin.enabled;
        opts.source_stepping = conv.source_stepping.enabled;
        opts.itl1 = conv.max_iterations as u32;
        opts.temp = conv.temperature;
        opts.tnom = conv.tnom;

        opts
    }

    /// Apply temperature
    pub fn apply_temperature(&self, opts: &mut EngineOptions, temp: f64) {
        opts.temp = temp;
    }

    /// Apply PVT corner
    pub fn apply_corner(&self, opts: &mut EngineOptions, corner: &PvtCorner) {
        opts.temp = corner.temperature;
        // VDD and process would be applied via model section selection
    }
}

/// PVT (Process-Voltage-Temperature) corner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvtCorner {
    /// Process corner name (tt, ff, ss, sf, fs)
    pub process: String,
    /// Supply voltage
    pub voltage: f64,
    /// Temperature in Celsius
    pub temperature: f64,
}

impl Default for PvtCorner {
    fn default() -> Self {
        Self {
            process: "tt".to_string(),
            voltage: 1.8,
            temperature: 27.0,
        }
    }
}

impl PvtCorner {
    /// Create standard corners
    pub fn standard_corners(vdd: f64) -> Vec<PvtCorner> {
        vec![
            PvtCorner {
                process: "tt".to_string(),
                voltage: vdd,
                temperature: 27.0,
            },
            PvtCorner {
                process: "ff".to_string(),
                voltage: vdd * 1.1,
                temperature: -40.0,
            },
            PvtCorner {
                process: "ss".to_string(),
                voltage: vdd * 0.9,
                temperature: 125.0,
            },
            PvtCorner {
                process: "sf".to_string(),
                voltage: vdd,
                temperature: 27.0,
            },
            PvtCorner {
                process: "fs".to_string(),
                voltage: vdd,
                temperature: 27.0,
            },
        ]
    }

    /// Create extreme corners
    pub fn extreme_corners(vdd: f64) -> Vec<PvtCorner> {
        vec![
            PvtCorner {
                process: "ff".to_string(),
                voltage: vdd * 1.1,
                temperature: -40.0,
            },
            PvtCorner {
                process: "ss".to_string(),
                voltage: vdd * 0.9,
                temperature: 125.0,
            },
        ]
    }
}

// =============================================================================
// Tests
// =============================================================================
