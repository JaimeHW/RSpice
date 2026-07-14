//! Stability (STB) Analysis Configuration
//!
//! Configuration for loop stability analysis (.stb).
//! Computes loop gain, phase margin, and gain margin.

use super::options::parse_si_value;

/// Stability analysis configuration
#[derive(Debug, Clone)]
pub struct StbConfig {
    /// Probe source element name
    pub probe_source: String,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Points per decade
    pub points_per_decade: u32,
    /// Compute gain margin
    pub gain_margin: bool,
    /// Compute phase margin
    pub phase_margin: bool,
    /// Compute crossover frequency
    pub crossover_freq: bool,
    /// Relative tolerance
    pub reltol: f64,
}

impl Default for StbConfig {
    fn default() -> Self {
        Self {
            probe_source: "LSTB".into(),
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_decade: 10,
            gain_margin: true,
            phase_margin: true,
            crossover_freq: true,
            reltol: 1e-3,
        }
    }
}

impl StbConfig {
    pub fn new(probe: &str) -> Self {
        Self {
            probe_source: probe.to_uppercase(),
            ..Default::default()
        }
    }
    pub fn with_freq_range(mut self, start: f64, stop: f64) -> Self {
        self.start_freq = start;
        self.stop_freq = stop;
        self
    }
    pub fn with_points(mut self, ppd: u32) -> Self {
        self.points_per_decade = ppd;
        self
    }

    pub fn to_spice(&self) -> String {
        format!(
            ".stb probe={} dec {} {} {}",
            self.probe_source,
            self.points_per_decade,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.probe_source.is_empty() {
            return Err("Probe source required".into());
        }
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".into());
        }
        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".into());
        }
        if self.start_freq >= self.stop_freq {
            return Err("Start must be less than stop".into());
        }
        if self.points_per_decade == 0 {
            return Err("Points per decade must be at least 1".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn total_points(&self) -> u32 {
        let decades = (self.stop_freq / self.start_freq).log10();
        (decades * self.points_per_decade as f64).ceil() as u32 + 1
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StbDialogState {
    pub probe_source: String,
    pub start_freq: String,
    pub stop_freq: String,
    pub points_per_decade: String,
    pub gain_margin: bool,
    pub phase_margin: bool,
    pub crossover_freq: bool,
    #[serde(skip)]
    pub initialized: bool,
}

impl StbDialogState {
    pub fn from_config(config: &StbConfig) -> Self {
        Self {
            probe_source: config.probe_source.clone(),
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            points_per_decade: config.points_per_decade.to_string(),
            gain_margin: config.gain_margin,
            phase_margin: config.phase_margin,
            crossover_freq: config.crossover_freq,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<StbConfig, String> {
        let start = parse_si_value(&self.start_freq).map_err(|e| format!("Bad start: {}", e))?;
        let stop = parse_si_value(&self.stop_freq).map_err(|e| format!("Bad stop: {}", e))?;
        let ppd: u32 = self
            .points_per_decade
            .parse()
            .map_err(|_| "Invalid points")?;
        let config = StbConfig {
            probe_source: self.probe_source.clone(),
            start_freq: start,
            stop_freq: stop,
            points_per_decade: ppd,
            gain_margin: self.gain_margin,
            phase_margin: self.phase_margin,
            crossover_freq: self.crossover_freq,
            reltol: 1e-3,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&StbConfig::default());
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
