//=============================================================================

/// Sensitivity analysis configuration
#[derive(Debug, Clone)]
pub struct SensitivityConfig {
    /// Output variable (e.g., "V(out)", "I(R1)")
    pub output_var: String,
    /// AC analysis (if true, does AC sensitivity)
    pub ac_mode: bool,
    /// Frequency for AC sensitivity
    pub frequency: Option<f64>,
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        }
    }
}

impl SensitivityConfig {
    /// Generate SPICE .sens command
    pub fn to_spice(&self) -> String {
        if self.ac_mode {
            if let Some(freq) = self.frequency {
                format!(".sens {} AC DEC 1 {} {}", self.output_var, freq, freq)
            } else {
                format!(".sens {} AC", self.output_var)
            }
        } else {
            format!(".sens {}", self.output_var)
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_var.is_empty() {
            errors.push("Output variable is required".to_string());
        }
        if self.ac_mode
            && let Some(freq) = self.frequency
            && (!freq.is_finite() || freq <= 0.0)
        {
            errors.push("Frequency must be finite and positive for AC sensitivity".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SensitivityConfig;

    #[test]
    fn ac_sensitivity_config_rejects_non_finite_frequency() {
        let config = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(f64::NAN),
        };

        let errors = config
            .validate()
            .expect_err("NaN AC sensitivity frequency must be rejected");
        assert!(
            errors
                .iter()
                .any(|message| message.contains("Frequency must be finite and positive"))
        );
    }
}
