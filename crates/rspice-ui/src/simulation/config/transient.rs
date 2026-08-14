//! Transient analysis configuration: stop time, step, and the initial
//! condition handling that decides whether the run starts from a DC
//! operating point or from stored conditions.

// Transient Analysis Configuration
//=============================================================================

/// Transient analysis configuration
#[derive(Debug, Clone)]
pub struct TransientAnalysisConfig {
    /// Stop time
    pub stop_time: f64,
    /// Step time (output interval)
    pub step_time: f64,
    /// Start time (default 0)
    pub start_time: f64,
    /// Maximum internal timestep
    pub max_timestep: Option<f64>,
    /// Use initial conditions
    pub uic: bool,
}

impl Default for TransientAnalysisConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        }
    }
}

impl TransientAnalysisConfig {
    /// Generate SPICE .tran command
    pub fn to_spice(&self) -> String {
        let mut cmd = if self.start_time > 0.0 {
            format!(
                ".tran {} {} {}",
                self.step_time, self.stop_time, self.start_time
            )
        } else {
            format!(".tran {} {}", self.step_time, self.stop_time)
        };
        if let Some(max_ts) = self.max_timestep {
            cmd.push_str(&format!(" {}", max_ts));
        }
        if self.uic {
            cmd.push_str(" UIC");
        }
        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            errors.push("Stop time must be finite and positive".to_string());
        }
        if !self.step_time.is_finite() || self.step_time <= 0.0 {
            errors.push("Step time must be finite and positive".to_string());
        }
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            errors.push("Start time must be finite and non-negative".to_string());
        } else if self.start_time >= self.stop_time {
            errors.push("Start time must be less than stop time".to_string());
        }
        if self.step_time.is_finite()
            && self.stop_time.is_finite()
            && self.step_time > self.stop_time
        {
            errors.push("Step time should not exceed stop time".to_string());
        }
        if self
            .max_timestep
            .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            errors.push("Maximum timestep must be finite and positive when provided".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
