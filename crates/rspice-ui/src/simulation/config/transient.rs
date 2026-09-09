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
        let mut cmd = if self.start_time > 0.0 || self.max_timestep.is_some() {
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

    pub(crate) fn resolved_maximum_step(
        &self,
    ) -> Result<f64, rspice_core::execution::TransientMaximumStepError> {
        rspice_core::execution::resolve_transient_maximum_step(
            self.step_time,
            self.stop_time,
            Some(self.start_time),
            self.max_timestep,
        )
    }

    /// Validate the same authored fields and derived solver limit used at execution.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        self.resolved_maximum_step()
            .map(|_| ())
            .map_err(|error| vec![error.to_string()])
    }
}
