// DC Sweep Configuration
//=============================================================================

use crate::simulation::dialog::DcConfig;

/// DC sweep analysis configuration
#[derive(Debug, Clone)]
pub struct DcSweepConfig {
    /// Source to sweep (e.g., "Vin")
    pub source: String,
    /// Start value
    pub start: f64,
    /// Stop value
    pub stop: f64,
    /// Step size
    pub step: f64,
    /// Secondary sweep source (optional)
    pub source2: Option<String>,
    /// Secondary start value
    pub start2: Option<f64>,
    /// Secondary stop value
    pub stop2: Option<f64>,
    /// Secondary step size
    pub step2: Option<f64>,
}

impl Default for DcSweepConfig {
    fn default() -> Self {
        Self {
            source: "Vin".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        }
    }
}

impl DcSweepConfig {
    /// Generate SPICE .dc command
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".dc {} {} {} {}",
            self.source, self.start, self.stop, self.step
        );
        if let (Some(src2), Some(start2), Some(stop2), Some(step2)) =
            (&self.source2, self.start2, self.stop2, self.step2)
        {
            cmd.push_str(&format!(" {} {} {} {}", src2, start2, stop2, step2));
        }
        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.source.is_empty() {
            errors.push("Source name is required".to_string());
        }
        if self.step == 0.0 {
            errors.push("Step size cannot be zero".to_string());
        }
        if (self.stop - self.start).signum() != self.step.signum() {
            errors.push("Step direction must match sweep direction".to_string());
        }

        match (&self.source2, self.start2, self.stop2, self.step2) {
            (None, None, None, None) => {}
            (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
                if source2.trim().is_empty() {
                    errors.push("Secondary source name is required".to_string());
                }
                if source2.eq_ignore_ascii_case(&self.source) {
                    errors.push("Secondary source must differ from primary source".to_string());
                }
                if step2 == 0.0 {
                    errors.push("Secondary step size cannot be zero".to_string());
                }
                if (stop2 - start2).signum() != step2.signum() {
                    errors.push("Secondary step direction must match sweep direction".to_string());
                }
            }
            _ => errors
                .push("Secondary sweep requires source2/start2/stop2/step2 values".to_string()),
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate number of points
    pub fn num_points(&self) -> usize {
        if self.step == 0.0 {
            return 0;
        }
        ((self.stop - self.start) / self.step).abs() as usize + 1
    }
}

impl From<DcConfig> for DcSweepConfig {
    fn from(cfg: DcConfig) -> Self {
        let (source2, start2, stop2, step2) = if let Some(src2) = cfg.source2 {
            (
                Some(src2.name),
                Some(src2.start),
                Some(src2.stop),
                Some(src2.step),
            )
        } else {
            (None, None, None, None)
        };
        Self {
            source: cfg.source1.name,
            start: cfg.source1.start,
            stop: cfg.source1.stop,
            step: cfg.source1.step,
            source2,
            start2,
            stop2,
            step2,
        }
    }
}
