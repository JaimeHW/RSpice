//! Reliability (Aging) analysis configuration.
//!
//! Provides a typed configuration surface for long-term degradation analysis.

use super::options::parse_si_value;

/// Reliability analysis configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct ReliabilityConfig {
    /// Target lifetime points in years.
    pub target_years: Vec<f64>,
    /// Enable Hot Carrier Injection contribution.
    pub enable_hci: bool,
    /// Enable Negative Bias Temperature Instability contribution.
    pub enable_nbti: bool,
    /// Enable electromigration contribution.
    pub enable_em: bool,
    /// Minimum stress voltage needed to include a device.
    pub min_stress_voltage: f64,
}

impl Default for ReliabilityConfig {
    fn default() -> Self {
        Self {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.1,
        }
    }
}

impl ReliabilityConfig {
    /// Validate reliability configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.target_years.is_empty() {
            return Err("At least one lifetime point must be specified".to_string());
        }
        if self
            .target_years
            .iter()
            .any(|years| !years.is_finite() || *years <= 0.0)
        {
            return Err("Lifetime points must be finite and > 0".to_string());
        }
        if !self.enable_hci && !self.enable_nbti && !self.enable_em {
            return Err("Enable at least one aging mechanism".to_string());
        }
        if !self.min_stress_voltage.is_finite() || self.min_stress_voltage < 0.0 {
            return Err("Minimum stress voltage must be finite and >= 0".to_string());
        }
        Ok(())
    }

    /// SPICE-like representation for logging/audit trails.
    pub fn to_spice(&self) -> String {
        let mechanisms = self.enabled_mechanisms().join(",");
        let years = self
            .target_years
            .iter()
            .map(|v| format!("{:.6}", v))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            ".rel mechanisms={} years={} minstress={:.6}",
            mechanisms, years, self.min_stress_voltage
        )
    }

    /// Enabled mechanism list.
    pub fn enabled_mechanisms(&self) -> Vec<&'static str> {
        let mut mechanisms = Vec::with_capacity(3);
        if self.enable_hci {
            mechanisms.push("hci");
        }
        if self.enable_nbti {
            mechanisms.push("nbti");
        }
        if self.enable_em {
            mechanisms.push("em");
        }
        mechanisms
    }
}

/// UI state for reliability dialog tab.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReliabilityDialogState {
    /// Comma-separated lifetime points (years).
    pub years_csv: String,
    /// Enable Hot Carrier Injection.
    pub enable_hci: bool,
    /// Enable NBTI.
    pub enable_nbti: bool,
    /// Enable electromigration.
    pub enable_em: bool,
    /// Minimum stress threshold.
    pub min_stress_voltage: String,
    /// Deferred defaults initialization.
    pub initialized: bool,
}

impl ReliabilityDialogState {
    /// Build UI state from typed config.
    pub fn from_config(config: &ReliabilityConfig) -> Self {
        let years_csv = config
            .target_years
            .iter()
            .map(|v| format_year(*v))
            .collect::<Vec<_>>()
            .join(", ");
        Self {
            years_csv,
            enable_hci: config.enable_hci,
            enable_nbti: config.enable_nbti,
            enable_em: config.enable_em,
            min_stress_voltage: format!("{}", config.min_stress_voltage),
            initialized: true,
        }
    }

    /// Convert UI state into typed config.
    pub fn to_config(&self) -> Result<ReliabilityConfig, String> {
        let mut years = parse_years_list(&self.years_csv)?;
        years.sort_by(|a, b| a.total_cmp(b));
        years.dedup_by(|a, b| (*a - *b).abs() < 1e-12);

        let min_stress = parse_si_value(&self.min_stress_voltage)
            .map_err(|e| format!("Invalid minimum stress voltage: {}", e))?;

        let config = ReliabilityConfig {
            target_years: years,
            enable_hci: self.enable_hci,
            enable_nbti: self.enable_nbti,
            enable_em: self.enable_em,
            min_stress_voltage: min_stress,
        };
        config.validate()?;
        Ok(config)
    }

    /// Initialize with defaults once.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&ReliabilityConfig::default());
        }
    }

}

fn parse_years_list(input: &str) -> Result<Vec<f64>, String> {
    let mut years = Vec::new();
    for token in input.split(|c: char| c == ',' || c == ';' || c.is_whitespace()) {
        let raw = token.trim();
        if raw.is_empty() {
            continue;
        }
        let value = raw
            .parse::<f64>()
            .map_err(|_| format!("Invalid lifetime value '{}'", raw))?;
        if !value.is_finite() || value <= 0.0 {
            return Err(format!("Lifetime value '{}' must be finite and > 0", raw));
        }
        years.push(value);
    }
    if years.is_empty() {
        return Err("At least one lifetime point must be provided".to_string());
    }
    Ok(years)
}

fn format_year(v: f64) -> String {
    if (v.fract()).abs() < 1e-12 {
        format!("{:.0}", v)
    } else {
        format!("{:.4}", v)
    }
}
