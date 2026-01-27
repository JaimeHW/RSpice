//! AC Analysis Configuration
//!
//! Configuration for small-signal AC analysis (.ac).

use super::framework::{labeled_checkbox, numeric_input, DialogTab};
use egui::Ui;

// =============================================================================
// Frequency Sweep Type
// =============================================================================

/// Type of frequency sweep
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FrequencySweep {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl FrequencySweep {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Decade => "Decade",
            Self::Octave => "Octave",
            Self::Linear => "Linear",
        }
    }

    /// SPICE keyword
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }

    /// All types
    pub fn all() -> &'static [FrequencySweep] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// AC Config
// =============================================================================

/// AC analysis configuration
#[derive(Debug, Clone)]
pub struct AcConfig {
    /// Sweep type
    pub sweep_type: FrequencySweep,
    /// Number of points (per decade/octave for log, total for linear)
    pub num_points: u32,
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Save all nodes
    pub save_all: bool,
}

impl Default for AcConfig {
    fn default() -> Self {
        Self {
            sweep_type: FrequencySweep::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
            save_all: true,
        }
    }
}

impl AcConfig {
    /// Create new config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Set sweep type
    pub fn with_sweep(mut self, sweep: FrequencySweep) -> Self {
        self.sweep_type = sweep;
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        format!(
            ".ac {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.start_freq <= 0.0 {
            return Err("Start frequency must be positive".to_string());
        }

        if self.stop_freq <= 0.0 {
            return Err("Stop frequency must be positive".to_string());
        }

        if self.start_freq >= self.stop_freq {
            return Err("Start frequency must be less than stop frequency".to_string());
        }

        if self.num_points == 0 {
            return Err("Number of points must be at least 1".to_string());
        }

        Ok(())
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            FrequencySweep::Decade => {
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            FrequencySweep::Octave => {
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            FrequencySweep::Linear => self.num_points,
        }
    }
}

impl DialogTab for AcConfig {
    fn name(&self) -> &str {
        "AC"
    }

    fn description(&self) -> &str {
        "Small-signal frequency analysis"
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("AC Analysis");
        ui.add_space(8.0);

        // Sweep type
        ui.label("Sweep Type");
        egui::ComboBox::from_id_salt("sweep_type")
            .selected_text(self.sweep_type.display_name())
            .show_ui(ui, |ui| {
                for s in FrequencySweep::all() {
                    ui.selectable_value(&mut self.sweep_type, *s, s.display_name());
                }
            });

        ui.add_space(8.0);

        // Frequency range
        ui.label("Frequency Range");
        ui.group(|ui| {
            numeric_input(ui, "Start", &mut self.start_freq, "Hz");
            numeric_input(ui, "Stop", &mut self.stop_freq, "Hz");

            let mut points = self.num_points as f64;
            if numeric_input(
                ui,
                "Points",
                &mut points,
                match self.sweep_type {
                    FrequencySweep::Decade => "/decade",
                    FrequencySweep::Octave => "/octave",
                    FrequencySweep::Linear => "total",
                },
            ) {
                self.num_points = points.max(1.0) as u32;
            }
        });

        ui.add_space(8.0);

        // Info
        ui.label(
            egui::RichText::new(format!("Total points: ~{}", self.total_points()))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
        );

        ui.add_space(8.0);
        labeled_checkbox(ui, "Save All Nodes", &mut self.save_all);
    }

    fn validate(&self) -> Result<(), String> {
        AcConfig::validate(self)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

fn format_freq(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{}G", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{}Meg", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{}k", freq / 1e3)
    } else {
        format!("{}", freq)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let cfg = AcConfig::default();
        assert!(cfg.start_freq > 0.0);
        assert!(cfg.stop_freq > cfg.start_freq);
    }

    #[test]
    fn test_config_new() {
        let cfg = AcConfig::new(100.0, 1e6, 20);
        assert!((cfg.start_freq - 100.0).abs() < 0.01);
        assert!((cfg.stop_freq - 1e6).abs() < 0.01);
        assert_eq!(cfg.num_points, 20);
    }

    #[test]
    fn test_with_sweep() {
        let cfg = AcConfig::default().with_sweep(FrequencySweep::Linear);
        assert_eq!(cfg.sweep_type, FrequencySweep::Linear);
    }

    #[test]
    fn test_validate_valid() {
        let cfg = AcConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_negative_start() {
        let mut cfg = AcConfig::default();
        cfg.start_freq = -100.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_start_after_stop() {
        let mut cfg = AcConfig::default();
        cfg.start_freq = 1e12;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_points() {
        let mut cfg = AcConfig::default();
        cfg.num_points = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_to_spice_decade() {
        let cfg = AcConfig::new(1.0, 1e6, 10);
        let spice = cfg.to_spice();
        assert!(spice.starts_with(".ac dec"));
    }

    #[test]
    fn test_to_spice_linear() {
        let cfg = AcConfig::new(1.0, 1e6, 100).with_sweep(FrequencySweep::Linear);
        let spice = cfg.to_spice();
        assert!(spice.contains("lin"));
    }

    #[test]
    fn test_total_points_decade() {
        let cfg = AcConfig::new(1.0, 1e6, 10);
        let points = cfg.total_points();
        // 6 decades * 10 points + 1 = 61
        assert!(points >= 60);
    }

    #[test]
    fn test_total_points_linear() {
        let cfg = AcConfig::new(1.0, 1e6, 100).with_sweep(FrequencySweep::Linear);
        assert_eq!(cfg.total_points(), 100);
    }

    #[test]
    fn test_sweep_names() {
        assert!(!FrequencySweep::Decade.display_name().is_empty());
        assert!(!FrequencySweep::Octave.display_name().is_empty());
        assert!(!FrequencySweep::Linear.display_name().is_empty());
    }

    #[test]
    fn test_sweep_keywords() {
        assert_eq!(FrequencySweep::Decade.spice_keyword(), "dec");
        assert_eq!(FrequencySweep::Octave.spice_keyword(), "oct");
        assert_eq!(FrequencySweep::Linear.spice_keyword(), "lin");
    }

    #[test]
    fn test_dialog_tab_name() {
        let cfg = AcConfig::default();
        assert_eq!(cfg.name(), "AC");
    }

    #[test]
    fn test_dialog_tab_reset() {
        let mut cfg = AcConfig::new(1.0, 100.0, 5);
        cfg.reset();
        assert!((cfg.stop_freq - 1e9).abs() < 0.01);
    }

    #[test]
    fn test_format_freq_giga() {
        let s = format_freq(1e9);
        assert!(s.contains("G"));
    }

    #[test]
    fn test_format_freq_mega() {
        let s = format_freq(1e6);
        assert!(s.contains("Meg"));
    }

    #[test]
    fn test_format_freq_kilo() {
        let s = format_freq(1e3);
        assert!(s.contains("k"));
    }
}
