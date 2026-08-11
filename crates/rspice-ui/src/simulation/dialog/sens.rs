//! Sensitivity Analysis Configuration
//!
//! Configuration for sensitivity analysis (.sens).
//! Computes the sensitivity of an output to all circuit parameters.

/// Sensitivity analysis type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SensType {
    #[default]
    Dc,
    Ac,
}

/// Sensitivity analysis configuration
///
/// `.SENS` differentiates the output against every parameter the circuit
/// exposes; the engine has no selection filter and no reporting threshold, so
/// this configuration deliberately carries neither. Narrowing the report is
/// the result viewer's job, where it can be changed without re-solving.
#[derive(Debug, Clone)]
pub struct SensConfig {
    /// Output expression (node voltage or current)
    pub output_expr: String,
    /// Analysis type (DC or AC)
    pub sens_type: SensType,
    /// AC frequency (only used for AC sens)
    pub ac_freq: f64,
}

impl Default for SensConfig {
    fn default() -> Self {
        Self {
            output_expr: "V(OUT)".into(),
            sens_type: SensType::Dc,
            ac_freq: 1e6,
        }
    }
}

impl SensConfig {
    #[cfg(test)]
    pub fn new(output: &str) -> Self {
        Self {
            output_expr: output.to_string(),
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn with_type(mut self, t: SensType) -> Self {
        self.sens_type = t;
        self
    }
    #[cfg(test)]
    pub fn with_ac_freq(mut self, f: f64) -> Self {
        self.ac_freq = f;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.output_expr.is_empty() {
            return Err("Output expression required".into());
        }
        if self.sens_type == SensType::Ac && (!self.ac_freq.is_finite() || self.ac_freq <= 0.0) {
            return Err("AC frequency must be finite and positive".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SensDialogState {
    pub output_expr: String,
    pub sens_type_idx: usize,
    pub ac_freq: String,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedSensDialogState {
    #[serde(default)]
    output_expr: String,
    #[serde(default)]
    sens_type_idx: usize,
    #[serde(default)]
    ac_freq: String,
    /// Retired. `.SENS` differentiates against everything it can reach; there
    /// was never a filter for these to select. Accepted so earlier projects
    /// still open; never written back.
    #[serde(default)]
    #[allow(dead_code)]
    include_params: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    include_devices: serde::de::IgnoredAny,
}

impl<'de> serde::Deserialize<'de> for SensDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let persisted = PersistedSensDialogState::deserialize(deserializer)?;
        Ok(Self {
            output_expr: persisted.output_expr,
            sens_type_idx: persisted.sens_type_idx,
            ac_freq: persisted.ac_freq,
            initialized: false,
        })
    }
}

impl SensDialogState {
    pub fn from_config(config: &SensConfig) -> Self {
        Self {
            output_expr: config.output_expr.clone(),
            sens_type_idx: match config.sens_type {
                SensType::Dc => 0,
                SensType::Ac => 1,
            },
            ac_freq: format_freq(config.ac_freq),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<SensConfig, String> {
        let sens_type = match self.sens_type_idx {
            0 => SensType::Dc,
            _ => SensType::Ac,
        };
        let freq = super::options::parse_si_value(&self.ac_freq)
            .map_err(|err| format!("Invalid AC frequency: {}", err))?;
        let config = SensConfig {
            output_expr: self.output_expr.clone(),
            sens_type,
            ac_freq: freq,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&SensConfig::default());
        }
    }
}

fn format_freq(f: f64) -> String {
    if f >= 1e9 {
        format!("{}G", f / 1e9)
    } else if f >= 1e6 {
        format!("{}Meg", f / 1e6)
    } else if f >= 1e3 {
        format!("{}k", f / 1e3)
    } else {
        format!("{}", f)
    }
}

#[cfg(test)]
mod tests {
    use super::{SensConfig, SensDialogState, SensType};

    #[test]
    fn ac_sensitivity_dialog_rejects_invalid_frequency_text() {
        let mut state = SensDialogState::from_config(
            &SensConfig::new("V(out)")
                .with_type(SensType::Ac)
                .with_ac_freq(1e6),
        );
        state.ac_freq = "not-a-frequency".to_string();

        let err = state
            .to_config()
            .expect_err("invalid AC frequency text must not silently default");
        assert!(err.contains("AC frequency"));
    }
}
