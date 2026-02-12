// AC Analysis Configuration
//=============================================================================

use crate::simulation::dialog::AcConfig;

/// AC sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcSweepType {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave
    Octave,
    /// Linear
    Linear,
}

impl AcSweepType {
    pub(super) fn spice_name(&self) -> &'static str {
        match self {
            AcSweepType::Decade => "dec",
            AcSweepType::Octave => "oct",
            AcSweepType::Linear => "lin",
        }
    }
}

/// AC analysis configuration
#[derive(Debug, Clone)]
pub struct AcAnalysisConfig {
    /// Sweep type
    pub sweep_type: AcSweepType,
    /// Number of points (per decade/octave, or total for linear)
    pub num_points: usize,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
}

impl Default for AcAnalysisConfig {
    fn default() -> Self {
        Self {
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
        }
    }
}

impl AcAnalysisConfig {
    /// Generate SPICE .ac command
    pub fn to_spice(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.start_freq <= 0.0 {
            errors.push("Start frequency must be positive".to_string());
        }
        if self.stop_freq <= 0.0 {
            errors.push("Stop frequency must be positive".to_string());
        }
        if self.start_freq >= self.stop_freq {
            errors.push("Start frequency must be less than stop frequency".to_string());
        }
        if self.num_points == 0 {
            errors.push("Number of points must be positive".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate total number of frequency points
    pub fn total_points(&self) -> usize {
        match self.sweep_type {
            AcSweepType::Decade => {
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64) as usize + 1
            }
            AcSweepType::Octave => {
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64) as usize + 1
            }
            AcSweepType::Linear => self.num_points,
        }
    }

    /// Generate array of frequency points based on sweep configuration
    ///
    /// This is the definitive frequency generation used by the engine.
    /// Matches Spectre/SPICE semantics:
    /// - Decade: num_points per decade, logarithmically spaced
    /// - Octave: num_points per octave, logarithmically spaced  
    /// - Linear: num_points total, linearly spaced
    pub fn generate_frequencies(&self) -> Vec<f64> {
        match self.sweep_type {
            AcSweepType::Decade => self.generate_logarithmic_sweep(10.0),
            AcSweepType::Octave => self.generate_logarithmic_sweep(2.0),
            AcSweepType::Linear => self.generate_linear_sweep(),
        }
    }

    /// Generate logarithmic frequency sweep
    ///
    /// # Arguments
    /// * `base` - Base for logarithm (10.0 for decade, 2.0 for octave)
    fn generate_logarithmic_sweep(&self, base: f64) -> Vec<f64> {
        if self.start_freq <= 0.0 || self.stop_freq <= 0.0 || self.start_freq >= self.stop_freq {
            return vec![];
        }

        let log_start = self.start_freq.log(base);
        let log_stop = self.stop_freq.log(base);
        let num_units = log_stop - log_start; // Number of decades/octaves

        // Total points = num_points per unit * num_units + 1
        let total_points = ((num_units * self.num_points as f64) as usize).max(1) + 1;

        let mut frequencies = Vec::with_capacity(total_points);
        for i in 0..total_points {
            let t = i as f64 / (total_points - 1).max(1) as f64;
            let log_freq = log_start + t * (log_stop - log_start);
            frequencies.push(base.powf(log_freq));
        }

        // Ensure we hit exact start and stop values
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

        // Ensure exact stop value
        if let Some(last) = frequencies.last_mut() {
            *last = self.stop_freq;
        }

        frequencies
    }
}

impl From<AcConfig> for AcAnalysisConfig {
    fn from(cfg: AcConfig) -> Self {
        use crate::simulation::dialog::ac::FrequencySweep;
        let sweep_type = match cfg.sweep_type {
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
            FrequencySweep::Decade => AcSweepType::Decade,
        };
        Self {
            sweep_type,
            num_points: cfg.num_points as usize,
            start_freq: cfg.start_freq,
            stop_freq: cfg.stop_freq,
        }
    }
}
