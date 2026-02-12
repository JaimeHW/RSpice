// Noise Analysis Configuration
//=============================================================================

use super::AcSweepType;
use crate::simulation::dialog::NoiseConfig;

/// Noise analysis configuration
#[derive(Debug, Clone)]
pub struct NoiseAnalysisConfig {
    /// Output node (voltage probe point)
    pub output_node: String,
    /// Reference node (usually ground)
    pub reference_node: String,
    /// Input source (for input-referred noise)
    pub input_source: String,
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Number of points per decade/octave
    pub num_points: usize,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
}

impl Default for NoiseAnalysisConfig {
    fn default() -> Self {
        Self {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "Vin".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        }
    }
}

impl NoiseAnalysisConfig {
    /// Generate SPICE .noise command
    pub fn to_spice(&self) -> String {
        format!(
            ".noise V({},{}) {} {} {} {} {}",
            self.output_node,
            self.reference_node,
            self.input_source,
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_node.is_empty() {
            errors.push("Output node is required".to_string());
        }
        if self.input_source.is_empty() {
            errors.push("Input source is required".to_string());
        }
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
            errors.push("Frequencies must be positive".to_string());
        }
        if self.start_freq >= self.stop_freq {
            errors.push("Start frequency must be less than stop frequency".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Generate array of frequency points based on sweep configuration
    ///
    /// Uses same logic as AcAnalysisConfig for consistency.
    pub fn generate_frequencies(&self) -> Vec<f64> {
        match self.sweep_type {
            AcSweepType::Decade => self.generate_logarithmic_sweep(10.0),
            AcSweepType::Octave => self.generate_logarithmic_sweep(2.0),
            AcSweepType::Linear => self.generate_linear_sweep(),
        }
    }

    /// Generate logarithmic frequency sweep
    fn generate_logarithmic_sweep(&self, base: f64) -> Vec<f64> {
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 || self.start_freq >= self.stop_freq {
            return vec![];
        }

        let log_start = self.start_freq.log(base);
        let log_stop = self.stop_freq.log(base);
        let num_units = log_stop - log_start;

        let total_points = ((num_units * self.num_points as f64) as usize).max(1) + 1;

        let mut frequencies = Vec::with_capacity(total_points);
        for i in 0..total_points {
            let t = i as f64 / (total_points - 1).max(1) as f64;
            let log_freq = log_start + t * (log_stop - log_start);
            frequencies.push(base.powf(log_freq));
        }

        if let Some(first) = frequencies.first_mut() {
            *first = self.start_freq;
        }
        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }

    /// Generate linear frequency sweep
    fn generate_linear_sweep(&self) -> Vec<f64> {
        if self.start_freq >= self.stop_freq || self.num_points == 0 {
            return vec![];
        }

        let mut frequencies = Vec::with_capacity(self.num_points);
        let step = (self.stop_freq - self.start_freq) / (self.num_points - 1).max(1) as f64;

        for i in 0..self.num_points {
            frequencies.push(self.start_freq + i as f64 * step);
        }

        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }

    /// Get default temperature for noise analysis (300K = 27°C)
    pub fn default_temperature(&self) -> f64 {
        300.0
    }
}

impl From<NoiseConfig> for NoiseAnalysisConfig {
    fn from(cfg: NoiseConfig) -> Self {
        use crate::simulation::dialog::ac::FrequencySweep;
        let sweep_type = match cfg.sweep_type {
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
            FrequencySweep::Decade => AcSweepType::Decade,
        };
        Self {
            output_node: cfg.output_node,
            reference_node: cfg.reference_node,
            input_source: cfg.input_source,
            sweep_type,
            num_points: cfg.num_points as usize,
            start_freq: cfg.start_freq,
            stop_freq: cfg.stop_freq,
        }
    }
}
