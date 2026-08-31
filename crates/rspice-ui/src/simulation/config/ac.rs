//! AC analysis configuration: sweep type, range, and points per decade.

// AC Analysis Configuration
//=============================================================================

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

    pub(super) fn freq_variation(&self) -> rspice_core::netlist::FreqVariation {
        match self {
            AcSweepType::Decade => rspice_core::netlist::FreqVariation::Dec,
            AcSweepType::Octave => rspice_core::netlist::FreqVariation::Oct,
            AcSweepType::Linear => rspice_core::netlist::FreqVariation::Lin,
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

        if !self.start_freq.is_finite() {
            errors.push("Start frequency must be finite".to_string());
        } else if self.start_freq <= 0.0 {
            errors.push("Start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() {
            errors.push("Stop frequency must be finite".to_string());
        } else if self.stop_freq <= 0.0 {
            errors.push("Stop frequency must be positive".to_string());
        }
        if self.start_freq.is_finite()
            && self.stop_freq.is_finite()
            && self.start_freq >= self.stop_freq
        {
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

    /// Generate array of frequency points based on sweep configuration
    ///
    /// Delegates to the core ngspice-exact generator so interactive runs
    /// match what the exported .ac deck produces in the CLI or ngspice.
    pub fn generate_frequencies(&self) -> Vec<f64> {
        rspice_core::analysis::ac::ac_sweep_frequencies(
            self.sweep_type.freq_variation(),
            self.num_points,
            self.start_freq,
            self.stop_freq,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_non_finite_frequency_bounds() {
        for (start_freq, stop_freq) in [
            (f64::NAN, 1.0),
            (f64::INFINITY, 1.0),
            (f64::NEG_INFINITY, 1.0),
            (1.0, f64::NAN),
            (1.0, f64::INFINITY),
            (1.0, f64::NEG_INFINITY),
        ] {
            let config = AcAnalysisConfig {
                start_freq,
                stop_freq,
                ..AcAnalysisConfig::default()
            };
            assert!(
                config.validate().is_err(),
                "accepted start={start_freq}, stop={stop_freq}"
            );
        }
    }

    #[test]
    fn validation_keeps_finite_positive_ordered_bounds() {
        assert!(AcAnalysisConfig::default().validate().is_ok());
    }
}
