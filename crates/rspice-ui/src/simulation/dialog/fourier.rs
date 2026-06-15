//! Fourier Analysis Configuration
//!
//! Configuration for Fourier/spectral analysis of transient waveforms.
//! Computes harmonic distortion (THD), spectral content, and fundamental metrics.

use super::options::parse_si_value;

/// Fourier analysis configuration
#[derive(Debug, Clone)]
pub struct FourierConfig {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: f64,
    /// Number of harmonics to compute
    pub num_harmonics: u32,
    /// Output node to analyze
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// Analysis window start time
    pub start_time: f64,
    /// Analysis window stop time
    pub stop_time: f64,
    /// Compute THD
    pub compute_thd: bool,
    /// Normalize to fundamental
    pub normalize: bool,
}

impl Default for FourierConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 1e6,
            num_harmonics: 10,
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            start_time: 0.0,
            stop_time: 10e-6,
            compute_thd: true,
            normalize: true,
        }
    }
}

impl FourierConfig {
    pub fn new(fundamental: f64, harmonics: u32) -> Self {
        Self {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            ..Default::default()
        }
    }

    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = node.to_uppercase();
        self
    }

    pub fn with_window(mut self, start: f64, stop: f64) -> Self {
        self.start_time = start;
        self.stop_time = stop;
        self
    }

    /// Number of periods in analysis window
    pub fn periods_in_window(&self) -> f64 {
        if self.fundamental_freq > 0.0 {
            (self.stop_time - self.start_time) * self.fundamental_freq
        } else {
            0.0
        }
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".four {} {}",
            format_freq(self.fundamental_freq),
            self.num_harmonics
        );
        if !self.output_node.is_empty() {
            cmd.push_str(&format!(" V({})", self.output_node));
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.fundamental_freq <= 0.0 {
            return Err("Fundamental frequency must be positive".into());
        }
        if self.num_harmonics == 0 {
            return Err("Number of harmonics must be at least 1".into());
        }
        if self.output_node.is_empty() {
            return Err("Output node must be specified".into());
        }
        if self.stop_time <= self.start_time {
            return Err("Stop time must be after start time".into());
        }
        if self.periods_in_window() < 1.0 {
            return Err("Analysis window must contain at least one period".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct FourierDialogState {
    pub fundamental: String,
    pub harmonics: String,
    pub output_node: String,
    pub start_time: String,
    pub stop_time: String,
    pub compute_thd: bool,
    pub normalize: bool,
    pub initialized: bool,
}

impl FourierDialogState {
    pub fn from_config(config: &FourierConfig) -> Self {
        Self {
            fundamental: format_freq(config.fundamental_freq),
            harmonics: config.num_harmonics.to_string(),
            output_node: config.output_node.clone(),
            start_time: format_time(config.start_time),
            stop_time: format_time(config.stop_time),
            compute_thd: config.compute_thd,
            normalize: config.normalize,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<FourierConfig, String> {
        let fund = parse_si_value(&self.fundamental).map_err(|e| format!("Bad freq: {}", e))?;
        let harm: u32 = self.harmonics.parse().map_err(|_| "Bad harmonics")?;
        let start = parse_si_value(&self.start_time).unwrap_or(0.0);
        let stop = parse_si_value(&self.stop_time).map_err(|e| format!("Bad stop: {}", e))?;

        let config = FourierConfig {
            fundamental_freq: fund,
            num_harmonics: harm,
            output_node: self.output_node.clone(),
            output_ref: String::new(),
            start_time: start,
            stop_time: stop,
            compute_thd: self.compute_thd,
            normalize: self.normalize,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&FourierConfig::default());
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

fn format_time(t: f64) -> String {
    if t == 0.0 {
        "0".into()
    } else if t >= 1e-3 {
        format!("{}m", t / 1e-3)
    } else if t >= 1e-6 {
        format!("{}u", t / 1e-6)
    } else if t >= 1e-9 {
        format!("{}n", t / 1e-9)
    } else {
        format!("{}p", t / 1e-12)
    }
}
