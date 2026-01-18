//! Simulation Command Configuration
//!
//! Defines analysis types and simulation commands that can be configured
//! via the simulation dialog and executed by the simulation engine.
//!
//! This module provides a clean abstraction over raw SPICE command strings,
//! allowing the UI to work with structured data while generating valid
//! SPICE syntax for the simulation backend.

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// AC Analysis Types
// =============================================================================

/// AC sweep type (frequency variation method)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AcSweepType {
    /// Decade - points per decade (logarithmic, default)
    #[default]
    Decade,
    /// Octave - points per octave (logarithmic)
    Octave,
    /// Linear - equal frequency spacing
    Linear,
}

impl AcSweepType {
    /// Get the SPICE keyword for this sweep type
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "DEC",
            AcSweepType::Octave => "OCT",
            AcSweepType::Linear => "LIN",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "Decade",
            AcSweepType::Octave => "Octave",
            AcSweepType::Linear => "Linear",
        }
    }

    /// All sweep types for UI iteration
    pub const ALL: [AcSweepType; 3] = [
        AcSweepType::Decade,
        AcSweepType::Octave,
        AcSweepType::Linear,
    ];
}

// =============================================================================
// DC Sweep Source Type
// =============================================================================

/// Type of source to sweep in DC analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DcSourceType {
    /// Voltage source
    #[default]
    Voltage,
    /// Current source
    Current,
}

impl DcSourceType {
    pub fn display_name(&self) -> &'static str {
        match self {
            DcSourceType::Voltage => "Voltage Source",
            DcSourceType::Current => "Current Source",
        }
    }
}

// =============================================================================
// Simulation Commands
// =============================================================================

/// Transient analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientConfig {
    /// Stop time in seconds
    pub stop_time: f64,
    /// Suggested time step in seconds
    pub time_step: f64,
    /// Start saving data at this time (default 0)
    pub start_time: f64,
    /// Maximum internal time step (optional, limits adaptive stepping)
    pub max_step: Option<f64>,
    /// Use initial conditions (skip DC operating point)
    pub use_initial_conditions: bool,
}

impl Default for TransientConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-3, // 1ms default
            time_step: 1e-6, // 1µs default
            start_time: 0.0,
            max_step: None,
            use_initial_conditions: false,
        }
    }
}

impl TransientConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(
            ".TRAN {} {}",
            format_engineering(self.time_step),
            format_engineering(self.stop_time)
        );

        if self.start_time > 0.0 {
            cmd.push_str(&format!(" {}", format_engineering(self.start_time)));

            if let Some(max) = self.max_step {
                cmd.push_str(&format!(" {}", format_engineering(max)));
            }
        } else if let Some(max) = self.max_step {
            // Need to include start time as 0 to specify max step
            cmd.push_str(&format!(" 0 {}", format_engineering(max)));
        }

        if self.use_initial_conditions {
            cmd.push_str(" UIC");
        }

        cmd
    }
}

/// AC analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcConfig {
    /// Start frequency in Hz
    pub start_freq: f64,
    /// Stop frequency in Hz
    pub stop_freq: f64,
    /// Number of points (per decade/octave for log sweep, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: AcSweepType,
}

impl Default for AcConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0, // 1 Hz
            stop_freq: 1e6,  // 1 MHz
            num_points: 10,  // 10 points per decade
            sweep_type: AcSweepType::Decade,
        }
    }
}

impl AcConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        format!(
            ".AC {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_engineering(self.start_freq),
            format_engineering(self.stop_freq)
        )
    }
}

/// DC sweep analysis configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcSweepConfig {
    /// Source name to sweep (e.g., "V1")
    pub source_name: String,
    /// Start value
    pub start_value: f64,
    /// Stop value
    pub stop_value: f64,
    /// Increment step
    pub step_value: f64,
    /// Optional second source for nested sweep
    pub source2: Option<Box<DcSweepSource>>,
}

/// Second source for nested DC sweep
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcSweepSource {
    pub source_name: String,
    pub start_value: f64,
    pub stop_value: f64,
    pub step_value: f64,
}

impl Default for DcSweepConfig {
    fn default() -> Self {
        Self {
            source_name: "V1".to_string(),
            start_value: 0.0,
            stop_value: 5.0,
            step_value: 0.1,
            source2: None,
        }
    }
}

impl DcSweepConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        let mut cmd = format!(
            ".DC {} {} {} {}",
            self.source_name,
            format_engineering(self.start_value),
            format_engineering(self.stop_value),
            format_engineering(self.step_value)
        );

        if let Some(src2) = &self.source2 {
            cmd.push_str(&format!(
                " {} {} {} {}",
                src2.source_name,
                format_engineering(src2.start_value),
                format_engineering(src2.stop_value),
                format_engineering(src2.step_value)
            ));
        }

        cmd
    }
}

/// Operating point analysis configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpConfig {
    /// Whether OP analysis is enabled
    pub enabled: bool,
}

impl OpConfig {
    /// Generate the SPICE command string
    pub fn to_spice_string(&self) -> String {
        if self.enabled {
            ".OP".to_string()
        } else {
            String::new()
        }
    }
}

// =============================================================================
// Unified Simulation Configuration
// =============================================================================

/// Complete simulation configuration containing all analysis settings
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Transient analysis (enabled if Some)
    pub transient: Option<TransientConfig>,
    /// AC analysis (enabled if Some)
    pub ac: Option<AcConfig>,
    /// DC sweep analysis (enabled if Some)
    pub dc_sweep: Option<DcSweepConfig>,
    /// Operating point analysis
    pub op: OpConfig,
}

impl SimulationConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any analysis is configured
    pub fn has_analysis(&self) -> bool {
        self.transient.is_some() || self.ac.is_some() || self.dc_sweep.is_some() || self.op.enabled
    }

    /// Generate all SPICE command strings
    pub fn to_spice_commands(&self) -> Vec<String> {
        let mut commands = Vec::new();

        if self.op.enabled {
            commands.push(self.op.to_spice_string());
        }

        if let Some(dc) = &self.dc_sweep {
            commands.push(dc.to_spice_string());
        }

        if let Some(ac) = &self.ac {
            commands.push(ac.to_spice_string());
        }

        if let Some(tran) = &self.transient {
            commands.push(tran.to_spice_string());
        }

        commands
    }

    /// Generate commands as a single string (for insertion into netlist)
    pub fn to_spice_string(&self) -> String {
        self.to_spice_commands().join("\n")
    }
}

// =============================================================================
// Formatting Utilities
// =============================================================================

/// Format a number using engineering notation with SI prefixes
/// This matches how SPICE values are typically written
pub fn format_engineering(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let abs_val = value.abs();
    let sign = if value < 0.0 { "-" } else { "" };

    let (scaled, suffix) = if abs_val >= 1e12 {
        (abs_val / 1e12, "T")
    } else if abs_val >= 1e9 {
        (abs_val / 1e9, "G")
    } else if abs_val >= 1e6 {
        (abs_val / 1e6, "MEG")
    } else if abs_val >= 1e3 {
        (abs_val / 1e3, "k")
    } else if abs_val >= 1.0 {
        (abs_val, "")
    } else if abs_val >= 1e-3 {
        (abs_val * 1e3, "m")
    } else if abs_val >= 1e-6 {
        (abs_val * 1e6, "u")
    } else if abs_val >= 1e-9 {
        (abs_val * 1e9, "n")
    } else if abs_val >= 1e-12 {
        (abs_val * 1e12, "p")
    } else if abs_val >= 1e-15 {
        (abs_val * 1e15, "f")
    } else {
        // Fall back to scientific notation for very small values
        return format!("{:e}", value);
    };

    // Format with minimum necessary precision
    let formatted = if scaled == scaled.floor() {
        format!("{}{}{}", sign, scaled as i64, suffix)
    } else if (scaled * 10.0).fract().abs() < 1e-9 {
        format!("{}{:.1}{}", sign, scaled, suffix)
    } else if (scaled * 100.0).fract().abs() < 1e-9 {
        format!("{}{:.2}{}", sign, scaled, suffix)
    } else {
        format!("{}{:.3}{}", sign, scaled, suffix)
    };

    formatted
}

/// Parse a SPICE-style value string (with SI suffix) to f64
pub fn parse_spice_value(s: &str) -> Option<f64> {
    let s = s.trim().to_uppercase();

    if s.is_empty() {
        return None;
    }

    // Find where the number ends and suffix begins
    let mut num_end = s.len();
    for (i, c) in s.char_indices() {
        if !c.is_ascii_digit() && c != '.' && c != '-' && c != '+' && c != 'E' {
            // Check if this is part of scientific notation
            if i > 0 {
                let prev = s.chars().nth(i - 1).unwrap_or(' ');
                if prev == 'E' && (c == '-' || c == '+') {
                    continue;
                }
            }
            num_end = i;
            break;
        }
    }

    let num_str = &s[..num_end];
    let suffix = s[num_end..].trim();

    let base: f64 = num_str.parse().ok()?;

    let multiplier = match suffix {
        "T" => 1e12,
        "G" => 1e9,
        "MEG" => 1e6, // MEG = mega (1e6)
        "K" => 1e3,
        "" => 1.0,
        "M" => 1e-3,      // M = milli in SPICE (not mega!)
        "MIL" => 25.4e-6, // 1 mil = 25.4 micrometers
        "U" | "µ" => 1e-6,
        "N" => 1e-9,
        "P" => 1e-12,
        "F" => 1e-15,
        _ => return None,
    };

    Some(base * multiplier)
}

impl fmt::Display for SimulationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_spice_string())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transient_to_spice() {
        let tran = TransientConfig {
            stop_time: 1e-3,
            time_step: 1e-6,
            start_time: 0.0,
            max_step: None,
            use_initial_conditions: false,
        };
        assert_eq!(tran.to_spice_string(), ".TRAN 1u 1m");
    }

    #[test]
    fn test_transient_with_max_step() {
        let tran = TransientConfig {
            stop_time: 10e-3,
            time_step: 10e-6,
            start_time: 0.0,
            max_step: Some(1e-6),
            use_initial_conditions: false,
        };
        assert_eq!(tran.to_spice_string(), ".TRAN 10u 10m 0 1u");
    }

    #[test]
    fn test_ac_to_spice() {
        let ac = AcConfig {
            start_freq: 1.0,
            stop_freq: 1e6,
            num_points: 20,
            sweep_type: AcSweepType::Decade,
        };
        assert_eq!(ac.to_spice_string(), ".AC DEC 20 1 1MEG");
    }

    #[test]
    fn test_dc_sweep_to_spice() {
        let dc = DcSweepConfig {
            source_name: "V1".to_string(),
            start_value: 0.0,
            stop_value: 5.0,
            step_value: 0.1,
            source2: None,
        };
        assert_eq!(dc.to_spice_string(), ".DC V1 0 5 100m");
    }

    #[test]
    fn test_format_engineering() {
        assert_eq!(format_engineering(1e-6), "1u");
        assert_eq!(format_engineering(10e-3), "10m");
        assert_eq!(format_engineering(1e3), "1k");
        assert_eq!(format_engineering(1e6), "1MEG");
        assert_eq!(format_engineering(2.5e-9), "2.5n");
    }

    #[test]
    fn test_parse_spice_value() {
        // Use approximate comparison for floating-point values
        fn approx_eq(a: Option<f64>, b: f64) -> bool {
            match a {
                Some(v) => (v - b).abs() < 1e-15 * b.abs().max(1.0),
                None => false,
            }
        }
        assert!(approx_eq(parse_spice_value("1k"), 1e3));
        assert!(approx_eq(parse_spice_value("10u"), 10e-6));
        assert!(approx_eq(parse_spice_value("1MEG"), 1e6));
        assert!(approx_eq(parse_spice_value("2.5n"), 2.5e-9));
        assert!(approx_eq(parse_spice_value("1m"), 1e-3)); // Test milli
    }
}
