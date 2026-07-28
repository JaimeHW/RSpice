//! Periodic steady-state (PSS) analysis configuration.
//!
//! The dialog owns the exact nine-field Simulation Studio contract. Text is
//! retained until preflight so incomplete edits can be saved without silently
//! changing the last valid execution request.

use serde::{Deserialize, Deserializer, Serialize};

use super::options::parse_si_value;

/// PSS numerical formulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PssSolverMethod {
    /// Authenticated time-domain shooting solve.
    #[default]
    Shooting,
    /// Legacy value retained only so old projects fail closed with a useful
    /// diagnostic. Harmonic balance has its own analysis surface and contract.
    HarmonicBalance,
}

impl PssSolverMethod {
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Shooting => "shooting",
            Self::HarmonicBalance => "hb",
        }
    }
}

/// Fully parsed nine-field PSS contract.
#[derive(Debug, Clone, PartialEq)]
pub struct PssConfig {
    /// The only production PSS path is shooting. The field remains explicit in
    /// execution identity so legacy HB-PSS state cannot alias a shooting run.
    pub method: PssSolverMethod,
    /// Driven fundamental, or the initial frequency estimate for an oscillator.
    pub fund_freq: f64,
    /// Exact named periodic large-signal sources participating in this solve.
    pub tone_sources: Vec<String>,
    /// Number of periods used to settle before the shooting solve.
    pub tstab_periods: usize,
    /// Integration samples retained per solved period.
    pub points_per_period: usize,
    /// Relative periodicity tolerance.
    pub tolerance: f64,
    /// Enable autonomous period refinement.
    pub osc_mode: bool,
    /// Voltage node used for autonomous period detection.
    pub osc_node: String,
    /// Number of harmonics retained in the result; zero retains none.
    pub num_harmonics: usize,
}

impl Default for PssConfig {
    fn default() -> Self {
        Self {
            method: PssSolverMethod::Shooting,
            fund_freq: 1.0e3,
            tone_sources: vec!["VIN_DIFF".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-7,
            osc_mode: false,
            osc_node: String::new(),
            num_harmonics: 20,
        }
    }
}

impl PssConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.method != PssSolverMethod::Shooting {
            return Err(
                "Legacy HB-PSS mode is not executable; use the Harmonic Balance analysis instead"
                    .to_owned(),
            );
        }
        if !self.fund_freq.is_finite() || self.fund_freq <= 0.0 {
            return Err("Fundamental frequency must be finite and positive".to_owned());
        }
        if !self.osc_mode && self.tone_sources.is_empty() {
            return Err("At least one periodic tone source is required".to_owned());
        }
        for (index, source) in self.tone_sources.iter().enumerate() {
            if source.trim().is_empty() || source.chars().any(char::is_control) {
                return Err(format!(
                    "Tone source {} is not a valid source name",
                    index + 1
                ));
            }
            if self.tone_sources[..index]
                .iter()
                .any(|prior| prior.eq_ignore_ascii_case(source))
            {
                return Err(format!("Tone source '{source}' is listed more than once"));
            }
        }
        if self.points_per_period < 16 {
            return Err("Shooting points must be at least 16".to_owned());
        }
        let minimum_points = self
            .num_harmonics
            .max(1)
            .checked_mul(2)
            .ok_or_else(|| "Save harmonics exceeds the supported numeric range".to_owned())?;
        if self.points_per_period < minimum_points {
            return Err(
                "Shooting points must be at least twice the retained harmonic count".to_owned(),
            );
        }
        if !self.tolerance.is_finite() || self.tolerance <= 0.0 {
            return Err("Period tolerance must be finite and positive".to_owned());
        }
        if self.osc_mode && self.osc_node.trim().is_empty() {
            return Err("Oscillator node is required for an autonomous oscillator".to_owned());
        }
        Ok(())
    }

    pub fn to_spice(&self) -> String {
        format!(
            ".pss {} mode={} tones={} tstab_periods={} points_per_period={} tolerance={:.17e} autonomous={} oscnode={} save_harmonics={}",
            format_freq(self.fund_freq),
            self.method.spice_keyword(),
            self.tone_sources.join(","),
            self.tstab_periods,
            self.points_per_period,
            self.tolerance,
            if self.osc_mode { "yes" } else { "no" },
            if self.osc_node.is_empty() {
                "-"
            } else {
                self.osc_node.as_str()
            },
            self.num_harmonics,
        )
    }

}

/// Raw persisted PSS editor state.
#[derive(Debug, Clone, Serialize)]
pub struct PssDialogState {
    /// `0` is driven shooting. Legacy `1` is retained and fails closed until
    /// the user selects the supported mode.
    pub method_idx: usize,
    pub fund_freq: String,
    /// Comma-, semicolon-, or whitespace-separated exact source names.
    pub tone_sources: String,
    pub tstab_periods: String,
    pub points_per_period: String,
    pub tolerance: String,
    pub osc_mode: bool,
    pub osc_node: String,
    /// Numeric retained harmonic count. Zero explicitly retains no harmonics.
    pub num_harmonics: String,
    #[serde(skip)]
    pub initialized: bool,
}

impl Default for PssDialogState {
    fn default() -> Self {
        let mut state = Self::from_config(&PssConfig::default());
        state.initialized = false;
        state
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPssDialogState {
    #[serde(default = "default_pss_frequency")]
    fund_freq: String,
    #[serde(default = "default_pss_harmonics")]
    num_harmonics: String,
    /// Removed from the visible/executable contract. Accepted so projects from
    /// the temporary shell still open without assigning it hidden semantics.
    #[serde(default)]
    max_iter: Option<String>,
    #[serde(default)]
    method_idx: usize,
    #[serde(default)]
    osc_mode: bool,
    #[serde(default)]
    osc_node: String,
    /// Old projects stored harmonic retention as a boolean in addition to the
    /// count. `false` migrates exactly to a retained count of zero.
    #[serde(default)]
    save_harmonics: Option<bool>,
    #[serde(default = "default_pss_tones")]
    tone_sources: String,
    #[serde(default = "default_pss_stabilization_cycles")]
    tstab_periods: String,
    #[serde(default = "default_pss_shooting_points")]
    points_per_period: String,
    #[serde(default = "default_pss_tolerance")]
    tolerance: String,
}

impl<'de> Deserialize<'de> for PssDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedPssDialogState::deserialize(deserializer)?;
        let _retired_max_iterations = persisted.max_iter;
        let num_harmonics = if persisted.save_harmonics == Some(false) {
            "0".to_owned()
        } else {
            persisted.num_harmonics
        };
        Ok(Self {
            method_idx: persisted.method_idx,
            fund_freq: persisted.fund_freq,
            tone_sources: persisted.tone_sources,
            tstab_periods: persisted.tstab_periods,
            points_per_period: persisted.points_per_period,
            tolerance: persisted.tolerance,
            osc_mode: persisted.osc_mode,
            osc_node: persisted.osc_node,
            num_harmonics,
            initialized: false,
        })
    }
}

impl PssDialogState {
    pub fn from_config(config: &PssConfig) -> Self {
        Self {
            method_idx: match config.method {
                PssSolverMethod::Shooting => 0,
                PssSolverMethod::HarmonicBalance => 1,
            },
            fund_freq: format_freq(config.fund_freq),
            tone_sources: config.tone_sources.join(", "),
            tstab_periods: config.tstab_periods.to_string(),
            points_per_period: config.points_per_period.to_string(),
            tolerance: format!("{:.e}", config.tolerance),
            osc_mode: config.osc_mode,
            osc_node: config.osc_node.clone(),
            num_harmonics: config.num_harmonics.to_string(),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<PssConfig, String> {
        let fund_freq =
            parse_si_value(&self.fund_freq).map_err(|error| format!("Bad frequency: {error}"))?;
        let tone_sources = parse_tone_sources(&self.tone_sources)?;
        let tstab_periods = parse_usize(&self.tstab_periods, "stabilization cycles")?;
        let points_per_period = parse_usize(&self.points_per_period, "shooting points")?;
        let tolerance = parse_si_value(&self.tolerance)
            .map_err(|error| format!("Bad period tolerance: {error}"))?;
        let num_harmonics = parse_usize(&self.num_harmonics, "save harmonics")?;
        let method = match self.method_idx {
            0 => PssSolverMethod::Shooting,
            _ => PssSolverMethod::HarmonicBalance,
        };
        let config = PssConfig {
            method,
            fund_freq,
            tone_sources,
            tstab_periods,
            points_per_period,
            tolerance,
            osc_mode: self.osc_mode,
            osc_node: self.osc_node.trim().to_owned(),
            num_harmonics,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            // Deserialized drafts carry initialized=false but already contain
            // authoritative text. Only the all-empty legacy/default state is
            // replaced; otherwise mark it ready without losing migration data.
            if self.fund_freq.is_empty()
                && self.tone_sources.is_empty()
                && self.tstab_periods.is_empty()
                && self.points_per_period.is_empty()
                && self.tolerance.is_empty()
                && self.num_harmonics.is_empty()
            {
                *self = Self::from_config(&PssConfig::default());
            } else {
                self.initialized = true;
            }
        }
    }
}

fn parse_tone_sources(value: &str) -> Result<Vec<String>, String> {
    let sources = value
        .split(|character: char| character == ',' || character == ';' || character.is_whitespace())
        .filter(|source| !source.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok(sources)
}

fn parse_usize(value: &str, label: &str) -> Result<usize, String> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("Invalid {label}"))
}

fn default_pss_frequency() -> String {
    "1k".to_owned()
}

fn default_pss_harmonics() -> String {
    "20".to_owned()
}

fn default_pss_tones() -> String {
    "VIN_DIFF".to_owned()
}

fn default_pss_stabilization_cycles() -> String {
    "20".to_owned()
}

fn default_pss_shooting_points() -> String {
    "512".to_owned()
}

fn default_pss_tolerance() -> String {
    "1e-7".to_owned()
}

fn format_freq(frequency: f64) -> String {
    if frequency >= 1e9 {
        format!("{}G", frequency / 1e9)
    } else if frequency >= 1e6 {
        format!("{}Meg", frequency / 1e6)
    } else if frequency >= 1e3 {
        format!("{}k", frequency / 1e3)
    } else {
        format!("{frequency}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_nine_field_dialog_round_trips_to_config() {
        let state = PssDialogState {
            method_idx: 0,
            fund_freq: "2.5Meg".to_owned(),
            tone_sources: "VIN_LO, VIN_MOD".to_owned(),
            tstab_periods: "37".to_owned(),
            points_per_period: "1024".to_owned(),
            tolerance: "2e-9".to_owned(),
            osc_mode: true,
            osc_node: "osc_out".to_owned(),
            num_harmonics: "31".to_owned(),
            initialized: true,
        };
        let config = state.to_config().expect("valid exact PSS contract");
        assert_eq!(config.method, PssSolverMethod::Shooting);
        assert_eq!(config.fund_freq, 2.5e6);
        assert_eq!(config.tone_sources, ["VIN_LO", "VIN_MOD"]);
        assert_eq!(config.tstab_periods, 37);
        assert_eq!(config.points_per_period, 1024);
        assert_eq!(config.tolerance, 2.0e-9);
        assert!(config.osc_mode);
        assert_eq!(config.osc_node, "osc_out");
        assert_eq!(config.num_harmonics, 31);
    }

    #[test]
    fn old_dialog_boolean_retention_migrates_without_guessing() {
        let migrated: PssDialogState = serde_json::from_str(
            r#"{"fund_freq":"1Meg","num_harmonics":"12","max_iter":"50","method_idx":0,"osc_mode":false,"osc_node":"","save_harmonics":false}"#,
        )
        .expect("legacy state migrates");
        assert_eq!(migrated.num_harmonics, "0");
        assert_eq!(migrated.tone_sources, "VIN_DIFF");
        assert_eq!(migrated.tstab_periods, "20");
        assert_eq!(migrated.points_per_period, "512");
        assert_eq!(migrated.tolerance, "1e-7");
    }

    #[test]
    fn legacy_hb_pss_fails_closed() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.method_idx = 1;
        assert!(state.to_config().unwrap_err().contains("HB-PSS"));
    }

    #[test]
    fn autonomous_contract_can_authenticate_an_empty_drive_set() {
        let mut state = PssDialogState::from_config(&PssConfig::default());
        state.osc_mode = true;
        state.osc_node = "osc".to_owned();
        state.tone_sources.clear();
        assert!(state.to_config().is_ok());
    }
}
