//! Safety / SOA analysis dialog configuration.
//!
//! Defines transient-window SOA checks against scoped terminal stress limits.

use crate::options::parse_si_value;
use crate::soa_observation::SoaObservationConfig;
use crate::soa_rule::SoaRuleConfig;
mod curves;
mod rules;
pub use curves::SoaEnvelopeDraft;
pub use rules::SoaRuleDraft;

/// Typed SOA analysis configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaConfig {
    /// Import authored native model voltage limits before applying scoped rules.
    pub import_model_voltage_ratings: bool,
    pub observation: SoaObservationConfig,
    pub rules: Vec<SoaRuleConfig>,
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
            observation: SoaObservationConfig::default(),
            rules: Vec::new(),
            import_model_voltage_ratings: false,
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
        self.observation.validate(self.stop_time)?;
        for rule in &self.rules {
            rule.validate()?;
            if let Some(curve) = &rule.current_envelope {
                curve.validate_window(self.observation.start_time, self.stop_time)?;
            }
        }
        if self.stop_time <= 0.0 || !self.stop_time.is_finite() {
            return Err("SOA stop_time must be finite and > 0".to_string());
        }
        if self.step_time <= 0.0 || !self.step_time.is_finite() {
            return Err("SOA step_time must be finite and > 0".to_string());
        }
        if self.step_time > self.stop_time {
            return Err("SOA step_time must be <= stop_time".to_string());
        }
        if !self.import_model_voltage_ratings
            && self.rules.is_empty()
            && !self.check_vgs_max
            && !self.check_vds_max
            && !self.check_vbe_max
            && !self.check_vce_max
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
        let mut card = format!(
            ".soa stop={} step={} vgs={}({}) vds={}({}) vbe={}({}) vce={}({})",
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
        );
        if self.observation != SoaObservationConfig::default() {
            card.push_str(&format!(
                " start={} maxstep={} uic={} devices=({}) models=({})",
                self.observation.start_time,
                self.observation
                    .max_step
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "auto".into()),
                yes_no(self.observation.use_initial_conditions),
                self.observation.devices.join(" "),
                self.observation.models.join(" ")
            ));
        }
        if !self.observation.thresholds.is_default() {
            let threshold = |value: Option<f64>| {
                value
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "off".into())
            };
            card.push_str(&format!(
                " warning_fraction={} critical_fraction={}",
                threshold(self.observation.thresholds.warning_fraction),
                threshold(self.observation.thresholds.critical_fraction)
            ));
        }
        if self.import_model_voltage_ratings {
            card.push_str(" model_voltage_ratings=on");
        }
        for rule in &self.rules {
            card.push_str(&format!(
                " rule=({} {}{} devices=({}) models=({}))",
                rule.parameter.stress_code(),
                rule.max_value,
                if rule.voltage_basis == rspice_results::safety::SoaVoltageBasis::IntrinsicNodes {
                    " basis=intrinsic"
                } else {
                    ""
                },
                rule.devices.join(" "),
                rule.models.join(" ")
            ));
            if let Some(curve) = &rule.current_envelope {
                card.push_str(&format!(" current_voltage_curve=({} source={:?} conditions={:?} pulse={} voltage_interpolation={:?} pulse_interpolation={:?} voltages={:?} dc={:?} pulses={:?})",
                    rule.parameter.stress_code(), curve.source, curve.conditions,
                    curve.pulse_width_s.map_or_else(|| "DC".into(), |v| v.to_string()),
                    curve.voltage_interpolation, curve.pulse_interpolation, curve.voltages_v, curve.dc_currents_a, curve.pulses));
            }
            if let Some(duration) = rule.minimum_duration_s {
                card.push_str(&format!(
                    " min_duration=({} {}s)",
                    rule.parameter.stress_code(),
                    duration
                ));
            }
            if let rspice_results::safety::SoaDurationMode::Cumulative { recovery_time_s } =
                rule.duration_mode
            {
                card.push_str(&format!(
                    " cumulative_duration=({} recovery={})",
                    rule.parameter.stress_code(),
                    recovery_time_s
                        .map(|value| format!("{value}s"))
                        .unwrap_or_else(|| "off".into())
                ));
            }
            if let Some(curve) = rule.power_derating {
                card.push_str(&format!(
                    " derating=({} reference_k={} watts_per_k={})",
                    rule.parameter.stress_code(),
                    curve.reference_temperature_kelvin,
                    curve.watts_per_kelvin
                ));
            }
        }
        card
    }
}

fn default_warning_percent() -> String {
    "90".into()
}
fn default_critical_percent() -> String {
    "120".into()
}

fn threshold_fraction(text: &str, label: &str) -> Result<Option<f64>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    parse_si_value(text)
        .map(|value| Some(value / 100.0))
        .map_err(|error| format!("Invalid SOA {label} threshold percentage: {error}"))
}

fn zero_time() -> String {
    "0".into()
}

fn yes_no(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

/// UI state for SOA tab.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaDialogState {
    #[serde(default = "default_warning_percent")]
    pub warning_percent: String,
    #[serde(default = "default_critical_percent")]
    pub critical_percent: String,
    #[serde(default)]
    pub import_model_voltage_ratings: bool,
    #[serde(default)]
    pub rules: Vec<SoaRuleDraft>,
    #[serde(default = "zero_time")]
    pub start_time: String,
    #[serde(default)]
    pub max_step: String,
    #[serde(default)]
    pub use_initial_conditions: bool,
    #[serde(default)]
    pub devices: String,
    #[serde(default)]
    pub models: String,
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
    #[serde(skip)]
    pub initialized: bool,
}

impl SoaDialogState {
    /// Build UI state from config.
    pub fn from_config(config: &SoaConfig) -> Self {
        Self {
            warning_percent: config
                .observation
                .thresholds
                .warning_fraction
                .map(|value| (value * 100.0).to_string())
                .unwrap_or_default(),
            critical_percent: config
                .observation
                .thresholds
                .critical_fraction
                .map(|value| (value * 100.0).to_string())
                .unwrap_or_default(),
            import_model_voltage_ratings: config.import_model_voltage_ratings,
            rules: config.rules.iter().map(SoaRuleDraft::from_config).collect(),
            start_time: config.observation.start_time.to_string(),
            max_step: config
                .observation
                .max_step
                .map(|value| value.to_string())
                .unwrap_or_default(),
            use_initial_conditions: config.observation.use_initial_conditions,
            devices: config.observation.devices.join(" "),
            models: config.observation.models.join(" "),
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
            import_model_voltage_ratings: self.import_model_voltage_ratings,
            rules: self
                .rules
                .iter()
                .map(SoaRuleDraft::to_config)
                .collect::<Result<_, _>>()?,
            observation: SoaObservationConfig {
                thresholds: rspice_results::safety::SoaThresholds {
                    warning_fraction: threshold_fraction(&self.warning_percent, "warning")?,
                    critical_fraction: threshold_fraction(&self.critical_percent, "critical")?,
                },
                start_time: if self.start_time.trim().is_empty() {
                    0.0
                } else {
                    parse_si_value(&self.start_time)
                        .map_err(|error| format!("Invalid SOA start time: {error}"))?
                },
                max_step: if self.max_step.trim().is_empty() {
                    None
                } else {
                    Some(
                        parse_si_value(&self.max_step)
                            .map_err(|error| format!("Invalid SOA maximum step: {error}"))?,
                    )
                },
                use_initial_conditions: self.use_initial_conditions,
                devices: self.devices.split_whitespace().map(str::to_owned).collect(),
                models: self.models.split_whitespace().map(str::to_owned).collect(),
            },
            stop_time: parse_si_value(&self.stop_time)
                .map_err(|e| format!("Invalid SOA stop time: {}", e))?,
            step_time: parse_si_value(&self.step_time)
                .map_err(|e| format!("Invalid SOA step time: {}", e))?,
            check_vgs_max: self.check_vgs_max,
            max_vgs: parse_limit(
                &self.max_vgs,
                self.check_vgs_max,
                SoaConfig::default().max_vgs,
                "Vgs",
            )?,
            check_vds_max: self.check_vds_max,
            max_vds: parse_limit(
                &self.max_vds,
                self.check_vds_max,
                SoaConfig::default().max_vds,
                "Vds",
            )?,
            check_vbe_max: self.check_vbe_max,
            max_vbe: parse_limit(
                &self.max_vbe,
                self.check_vbe_max,
                SoaConfig::default().max_vbe,
                "Vbe",
            )?,
            check_vce_max: self.check_vce_max,
            max_vce: parse_limit(
                &self.max_vce,
                self.check_vce_max,
                SoaConfig::default().max_vce,
                "Vce",
            )?,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    /// One-time defaults.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            if self.stop_time.is_empty()
                && self.step_time.is_empty()
                && self.max_vgs.is_empty()
                && self.max_vds.is_empty()
                && self.max_vbe.is_empty()
                && self.max_vce.is_empty()
                && self.max_step.is_empty()
                && self.devices.is_empty()
                && self.models.is_empty()
                && self.rules.is_empty()
                && (self.warning_percent.is_empty() || self.warning_percent == "90")
                && (self.critical_percent.is_empty() || self.critical_percent == "120")
                && !self.import_model_voltage_ratings
                && !self.use_initial_conditions
                && (self.start_time.is_empty() || self.start_time == "0")
            {
                *self = Self::from_config(&SoaConfig::default());
            }
            self.initialized = true;
        }
    }
}

fn parse_limit(text: &str, enabled: bool, fallback: f64, name: &str) -> Result<f64, String> {
    let value = parse_si_value(text)
        .map_err(|error| format!("Invalid maximum {name}: {error}"))
        .and_then(|value| {
            if value.is_finite() && value > 0.0 {
                Ok(value)
            } else {
                Err(format!("Maximum {name} must be finite and positive"))
            }
        });
    if enabled {
        value
    } else {
        Ok(value.unwrap_or(fallback))
    }
}

fn format_scalar(v: f64) -> String {
    v.to_string()
}
