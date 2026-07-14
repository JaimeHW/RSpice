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

impl SensType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Dc => "DC",
            Self::Ac => "AC",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Dc => "dc",
            Self::Ac => "ac",
        }
    }
    pub fn all() -> &'static [SensType] {
        &[Self::Dc, Self::Ac]
    }
}

/// Sensitivity analysis configuration
#[derive(Debug, Clone)]
pub struct SensConfig {
    /// Output expression (node voltage or current)
    pub output_expr: String,
    /// Analysis type (DC or AC)
    pub sens_type: SensType,
    /// AC frequency (only used for AC sens)
    pub ac_freq: f64,
    /// Include parameter sensitivities
    pub include_params: bool,
    /// Include device sensitivities
    pub include_devices: bool,
    /// Threshold for reporting
    pub threshold: f64,
}

impl Default for SensConfig {
    fn default() -> Self {
        Self {
            output_expr: "V(OUT)".into(),
            sens_type: SensType::Dc,
            ac_freq: 1e6,
            include_params: true,
            include_devices: true,
            threshold: 1e-9,
        }
    }
}

impl SensConfig {
    pub fn new(output: &str) -> Self {
        Self {
            output_expr: output.to_string(),
            ..Default::default()
        }
    }

    pub fn with_type(mut self, t: SensType) -> Self {
        self.sens_type = t;
        self
    }
    pub fn with_ac_freq(mut self, f: f64) -> Self {
        self.ac_freq = f;
        self
    }

    pub fn to_spice(&self) -> String {
        match self.sens_type {
            SensType::Dc => format!(".sens {}", self.output_expr),
            SensType::Ac => format!(
                ".sens {} ac {}",
                self.output_expr,
                format_freq(self.ac_freq)
            ),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.output_expr.is_empty() {
            return Err("Output expression required".into());
        }
        if self.sens_type == SensType::Ac && (!self.ac_freq.is_finite() || self.ac_freq <= 0.0) {
            return Err("AC frequency must be finite and positive".into());
        }
        if self.threshold < 0.0 {
            return Err("Threshold cannot be negative".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensDialogState {
    pub output_expr: String,
    pub sens_type_idx: usize,
    pub ac_freq: String,
    pub include_params: bool,
    pub include_devices: bool,
    #[serde(skip)]
    pub initialized: bool,
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
            include_params: config.include_params,
            include_devices: config.include_devices,
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
            include_params: self.include_params,
            include_devices: self.include_devices,
            threshold: 1e-9,
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
