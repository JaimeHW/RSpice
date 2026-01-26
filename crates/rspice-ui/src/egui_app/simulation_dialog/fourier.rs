//! Fourier Analysis Configuration
//!
//! Configuration for Fourier/spectral analysis of transient waveforms.
//! Computes harmonic distortion (THD), spectral content, and fundamental metrics.

use super::options::parse_si_value;
use egui::Ui;

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

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Fourier Analysis");
        ui.label(egui::RichText::new("Spectral analysis and harmonic distortion").weak());
        ui.add_space(8.0);

        egui::Grid::new("four_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Fundamental:");
                ui.add(egui::TextEdit::singleline(&mut self.fundamental).desired_width(100.0));
                ui.end_row();
                ui.label("Harmonics:");
                ui.add(egui::TextEdit::singleline(&mut self.harmonics).desired_width(60.0));
                ui.end_row();
                ui.label("Output Node:");
                ui.add(egui::TextEdit::singleline(&mut self.output_node).desired_width(100.0));
                ui.end_row();
                ui.label("Start Time:");
                ui.add(egui::TextEdit::singleline(&mut self.start_time).desired_width(100.0));
                ui.end_row();
                ui.label("Stop Time:");
                ui.add(egui::TextEdit::singleline(&mut self.stop_time).desired_width(100.0));
                ui.end_row();
            });
        ui.add_space(4.0);
        ui.checkbox(&mut self.compute_thd, "Compute THD");
        ui.checkbox(&mut self.normalize, "Normalize to Fundamental");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let c = FourierConfig::default();
        assert_eq!(c.num_harmonics, 10);
    }
    #[test]
    fn test_new() {
        let c = FourierConfig::new(1e9, 20);
        assert_eq!(c.fundamental_freq, 1e9);
    }
    #[test]
    fn test_with_output() {
        let c = FourierConfig::default().with_output("vtest");
        assert_eq!(c.output_node, "VTEST");
    }
    #[test]
    fn test_with_window() {
        let c = FourierConfig::default().with_window(1e-6, 10e-6);
        assert_eq!(c.start_time, 1e-6);
    }
    #[test]
    fn test_validate_ok() {
        assert!(FourierConfig::default().validate().is_ok());
    }
    #[test]
    fn test_validate_zero_freq() {
        let mut c = FourierConfig::default();
        c.fundamental_freq = 0.0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_zero_harm() {
        let mut c = FourierConfig::default();
        c.num_harmonics = 0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_empty_node() {
        let mut c = FourierConfig::default();
        c.output_node.clear();
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_bad_window() {
        let mut c = FourierConfig::default();
        c.start_time = 10e-6;
        c.stop_time = 1e-6;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_validate_short_window() {
        let mut c = FourierConfig::new(1e9, 10);
        c.start_time = 0.0;
        c.stop_time = 1e-12;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_periods() {
        let c = FourierConfig::new(1e6, 10).with_window(0.0, 10e-6);
        assert!((c.periods_in_window() - 10.0).abs() < 0.01);
    }
    #[test]
    fn test_to_spice() {
        let s = FourierConfig::default().to_spice();
        assert!(s.contains(".four"));
    }
    #[test]
    fn test_dialog_roundtrip() {
        let s = FourierDialogState::from_config(&FourierConfig::default());
        assert!(s.to_config().is_ok());
    }
    #[test]
    fn test_reset() {
        let mut c = FourierConfig::new(5e9, 50);
        c.reset();
        assert_eq!(c.fundamental_freq, 1e6);
    }
}
