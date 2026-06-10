//! Safety / SOA analysis dialog configuration.
//!
//! Defines transient-window SOA checks against per-device voltage limits.

use super::options::parse_si_value;

/// Typed SOA analysis configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaConfig {
    /// Transient stop time.
    pub stop_time: f64,
    /// Transient step time.
    pub step_time: f64,
    /// Enable MOS/JFET/MESFET Vgs checking.
    pub check_vgs_max: bool,
    /// Vgs upper limit.
    pub max_vgs: f64,
    /// Enable MOS/JFET/MESFET Vds checking.
    pub check_vds_max: bool,
    /// Vds upper limit.
    pub max_vds: f64,
    /// Enable BJT Vbe checking.
    pub check_vbe_max: bool,
    /// Vbe upper limit.
    pub max_vbe: f64,
    /// Enable BJT Vce checking.
    pub check_vce_max: bool,
    /// Vce upper limit.
    pub max_vce: f64,
}

impl Default for SoaConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            step_time: 1e-9,
            check_vgs_max: true,
            max_vgs: 1.8,
            check_vds_max: true,
            max_vds: 3.3,
            check_vbe_max: true,
            max_vbe: 0.9,
            check_vce_max: true,
            max_vce: 5.0,
        }
    }
}

impl SoaConfig {
    /// Validate configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.stop_time <= 0.0 || !self.stop_time.is_finite() {
            return Err("SOA stop_time must be finite and > 0".to_string());
        }
        if self.step_time <= 0.0 || !self.step_time.is_finite() {
            return Err("SOA step_time must be finite and > 0".to_string());
        }
        if self.step_time > self.stop_time {
            return Err("SOA step_time must be <= stop_time".to_string());
        }
        if !self.check_vgs_max && !self.check_vds_max && !self.check_vbe_max && !self.check_vce_max
        {
            return Err("SOA requires at least one enabled check".to_string());
        }
        if self.check_vgs_max && (!self.max_vgs.is_finite() || self.max_vgs <= 0.0) {
            return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
        }
        if self.check_vds_max && (!self.max_vds.is_finite() || self.max_vds <= 0.0) {
            return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
        }
        if self.check_vbe_max && (!self.max_vbe.is_finite() || self.max_vbe <= 0.0) {
            return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
        }
        if self.check_vce_max && (!self.max_vce.is_finite() || self.max_vce <= 0.0) {
            return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
        }
        Ok(())
    }

    /// SPICE-like logging line.
    pub fn to_spice(&self) -> String {
        format!(
            ".soa stop={:.6e} step={:.6e} vgs={}({:.6e}) vds={}({:.6e}) vbe={}({:.6e}) vce={}({:.6e})",
            self.stop_time,
            self.step_time,
            yes_no(self.check_vgs_max),
            self.max_vgs,
            yes_no(self.check_vds_max),
            self.max_vds,
            yes_no(self.check_vbe_max),
            self.max_vbe,
            yes_no(self.check_vce_max),
            self.max_vce
        )
    }
}

fn yes_no(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

/// UI state for SOA tab.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoaDialogState {
    /// Stop time input.
    pub stop_time: String,
    /// Step time input.
    pub step_time: String,
    /// Check Vgs max.
    pub check_vgs_max: bool,
    /// Max Vgs input.
    pub max_vgs: String,
    /// Check Vds max.
    pub check_vds_max: bool,
    /// Max Vds input.
    pub max_vds: String,
    /// Check Vbe max.
    pub check_vbe_max: bool,
    /// Max Vbe input.
    pub max_vbe: String,
    /// Check Vce max.
    pub check_vce_max: bool,
    /// Max Vce input.
    pub max_vce: String,
    /// Lazy default init.
    pub initialized: bool,
}

impl SoaDialogState {
    /// Build UI state from config.
    pub fn from_config(config: &SoaConfig) -> Self {
        Self {
            stop_time: format_scalar(config.stop_time),
            step_time: format_scalar(config.step_time),
            check_vgs_max: config.check_vgs_max,
            max_vgs: format_scalar(config.max_vgs),
            check_vds_max: config.check_vds_max,
            max_vds: format_scalar(config.max_vds),
            check_vbe_max: config.check_vbe_max,
            max_vbe: format_scalar(config.max_vbe),
            check_vce_max: config.check_vce_max,
            max_vce: format_scalar(config.max_vce),
            initialized: true,
        }
    }

    /// Convert state to config.
    pub fn to_config(&self) -> Result<SoaConfig, String> {
        let cfg = SoaConfig {
            stop_time: parse_si_value(&self.stop_time)
                .map_err(|e| format!("Invalid SOA stop time: {}", e))?,
            step_time: parse_si_value(&self.step_time)
                .map_err(|e| format!("Invalid SOA step time: {}", e))?,
            check_vgs_max: self.check_vgs_max,
            max_vgs: parse_si_value(&self.max_vgs)
                .map_err(|e| format!("Invalid max Vgs: {}", e))?,
            check_vds_max: self.check_vds_max,
            max_vds: parse_si_value(&self.max_vds)
                .map_err(|e| format!("Invalid max Vds: {}", e))?,
            check_vbe_max: self.check_vbe_max,
            max_vbe: parse_si_value(&self.max_vbe)
                .map_err(|e| format!("Invalid max Vbe: {}", e))?,
            check_vce_max: self.check_vce_max,
            max_vce: parse_si_value(&self.max_vce)
                .map_err(|e| format!("Invalid max Vce: {}", e))?,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    /// One-time defaults.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&SoaConfig::default());
        }
    }

}

fn format_scalar(v: f64) -> String {
    if v.abs() >= 1e4 || (v.abs() > 0.0 && v.abs() < 1e-3) {
        format!("{:.6e}", v)
    } else {
        format!("{:.6}", v)
    }
}
