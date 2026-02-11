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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_options_defaults() {
        let opts = EngineOptions::engine_defaults();
        assert_eq!(opts.temp, 27.0);
        assert!(opts.reltol > 0.0);
    }

    #[test]
    fn test_engine_options_with_temp() {
        let opts = EngineOptions::engine_defaults().with_temp(85.0);
        assert_eq!(opts.temp, 85.0);
    }

    #[test]
    fn test_engine_options_high_precision() {
        let opts = EngineOptions::engine_defaults().high_precision();
        assert!(opts.reltol < 1e-4);
        assert!(opts.itl1 >= 200);
    }

    #[test]
    fn test_engine_options_fast_mode() {
        let opts = EngineOptions::engine_defaults().fast_mode();
        assert!(opts.reltol >= 1e-2);
    }

    #[test]
    fn test_to_engine_options_string() {
        let opts = EngineOptions::engine_defaults();
        let s = opts.to_engine_options_string();
        assert!(s.contains("temp=27"));
        assert!(s.contains("reltol="));
    }

    #[test]
    fn test_to_spice_options() {
        let opts = EngineOptions::engine_defaults();
        let s = opts.to_spice_options();
        assert!(s.contains(".OPTIONS"));
        assert!(s.contains("TEMP="));
        assert!(s.contains("TNOM="));
        assert!(s.contains("ITL4="));
        assert!(!s.contains("\n.TEMP "));
    }

    #[test]
    fn test_to_spice_options_temperature_is_read_as_netlist_option() {
        let opts = EngineOptions::engine_defaults().with_temp(85.0);
        let netlist = format!("* opts\nR1 in 0 1k\n{}\n.end\n", opts.to_spice_options());
        let parsed = rspice_core::netlist::parse_netlist(&netlist)
            .expect("options-augmented netlist should parse");
        assert_eq!(parsed.options.temp, Some(85.0));
    }

    #[test]
    fn test_to_spice_options_includes_custom_options_in_stable_order() {
        let mut opts = EngineOptions::engine_defaults();
        opts.custom.insert("zeta".to_string(), "2".to_string());
        opts.custom.insert("alpha".to_string(), "1".to_string());
        opts.custom.insert("nopage".to_string(), String::new());

        let spice = opts.to_spice_options();
        let alpha_pos = spice
            .find(".OPTIONS ALPHA=1")
            .expect("alpha option should be emitted");
        let zeta_pos = spice
            .find(".OPTIONS ZETA=2")
            .expect("zeta option should be emitted");
        assert!(
            alpha_pos < zeta_pos,
            "custom options should be sorted by key"
        );
        assert!(spice.contains(".OPTIONS NOPAGE"));
    }

    #[test]
    fn test_integration_method_to_engine_keyword() {
        assert_eq!(IntegrationMethod::Trapezoidal.to_engine_keyword(), "trap");
        assert_eq!(IntegrationMethod::Gear.to_engine_keyword(), "gear");
    }

    #[test]
    fn test_pvt_corner_default() {
        let corner = PvtCorner::default();
        assert_eq!(corner.process, "tt");
        assert_eq!(corner.temperature, 27.0);
    }

    #[test]
    fn test_pvt_standard_corners() {
        let corners = PvtCorner::standard_corners(1.8);
        assert_eq!(corners.len(), 5);

        let tt = corners.iter().find(|c| c.process == "tt").unwrap();
        assert_eq!(tt.temperature, 27.0);

        let ff = corners.iter().find(|c| c.process == "ff").unwrap();
        assert_eq!(ff.temperature, -40.0);
    }

    #[test]
    fn test_options_translator_new() {
        let translator = OptionsTranslator::new();
        assert!(translator.base.reltol > 0.0);
    }
}
