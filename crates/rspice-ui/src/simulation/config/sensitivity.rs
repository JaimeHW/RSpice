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
            && self
                .frequency
                .is_some_and(|frequency| !frequency.is_finite() || frequency <= 0.0)
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
    use super::*;

    #[test]
    fn validate_rejects_nonfinite_ac_frequency() {
        for frequency in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let config = SensitivityConfig {
                ac_mode: true,
                frequency: Some(frequency),
                ..SensitivityConfig::default()
            };

            assert!(
                config.validate().is_err(),
                "expected {frequency:?} to be rejected"
            );
        }
    }
}
