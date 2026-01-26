//! DC Analysis Configuration
//!
//! Configuration for DC sweep analysis (.dc).

use super::framework::{labeled_input, numeric_input, DialogTab};
use egui::Ui;

// =============================================================================
// DC Sweep Source
// =============================================================================

/// DC sweep source type
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepSource {
    /// Source name (e.g., "VIN", "R1")
    pub name: String,
    /// Start value
    pub start: f64,
    /// Stop value
    pub stop: f64,
    /// Step value
    pub step: f64,
}

impl Default for DcSweepSource {
    /// Commercial simulator default: sweep VIN from 0 to 5V with 0.1V step
    fn default() -> Self {
        Self {
            name: "VIN".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
        }
    }
}

impl DcSweepSource {
    /// Create new source
    pub fn new(name: &str, start: f64, stop: f64, step: f64) -> Self {
        Self {
            name: name.to_string(),
            start,
            stop,
            step,
        }
    }

    /// Number of points
    pub fn num_points(&self) -> u32 {
        if self.step.abs() < 1e-15 {
            return 1;
        }
        ((self.stop - self.start) / self.step).abs().ceil() as u32 + 1
    }

    /// Validate source
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Source name is required".to_string());
        }

        if self.step.abs() < 1e-15 {
            return Err("Step size cannot be zero".to_string());
        }

        if (self.stop > self.start && self.step < 0.0)
            || (self.stop < self.start && self.step > 0.0)
        {
            return Err("Step sign must match sweep direction".to_string());
        }

        Ok(())
    }

    /// Generate SPICE source string
    pub fn to_spice(&self) -> String {
        format!("{} {} {} {}", self.name, self.start, self.stop, self.step)
    }
}

// =============================================================================
// DC Config
// =============================================================================

/// DC analysis configuration
#[derive(Debug, Clone, Default)]
pub struct DcConfig {
    /// Primary sweep source
    pub source1: DcSweepSource,
    /// Optional secondary sweep source (nested sweep)
    pub source2: Option<DcSweepSource>,
    /// Save all nodes
    pub save_all: bool,
}

impl DcConfig {
    /// Create new config with single source
    pub fn new(source_name: &str, start: f64, stop: f64, step: f64) -> Self {
        Self {
            source1: DcSweepSource::new(source_name, start, stop, step),
            source2: None,
            save_all: true,
        }
    }

    /// Add second sweep source (nested)
    pub fn with_second_source(mut self, source: DcSweepSource) -> Self {
        self.source2 = Some(source);
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(".dc {}", self.source1.to_spice());

        if let Some(ref src2) = self.source2 {
            cmd.push_str(&format!(" {}", src2.to_spice()));
        }

        cmd
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        self.source1.validate()?;

        if let Some(ref src2) = self.source2 {
            src2.validate()?;
        }

        Ok(())
    }

    /// Total number of analysis points
    pub fn total_points(&self) -> u32 {
        let p1 = self.source1.num_points();
        match &self.source2 {
            Some(src2) => p1 * src2.num_points(),
            None => p1,
        }
    }

    /// Has nested sweep?
    pub fn is_nested(&self) -> bool {
        self.source2.is_some()
    }
}

impl DialogTab for DcConfig {
    fn name(&self) -> &str {
        "DC"
    }

    fn description(&self) -> &str {
        "DC sweep analysis"
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("DC Analysis");
        ui.add_space(8.0);

        // Primary source
        ui.label("Primary Sweep");
        ui.group(|ui| {
            labeled_input(ui, "Source", &mut self.source1.name, "VIN, R1, etc.");
            numeric_input(ui, "Start", &mut self.source1.start, "V");
            numeric_input(ui, "Stop", &mut self.source1.stop, "V");
            numeric_input(ui, "Step", &mut self.source1.step, "V");
        });

        ui.add_space(8.0);

        // Secondary source (nested sweep)
        let has_second = self.source2.is_some();
        let mut enable_second = has_second;

        ui.checkbox(&mut enable_second, "Enable Nested Sweep");

        if enable_second != has_second {
            if enable_second {
                self.source2 = Some(DcSweepSource::default());
            } else {
                self.source2 = None;
            }
        }

        if let Some(ref mut src2) = self.source2 {
            ui.group(|ui| {
                labeled_input(ui, "Source 2", &mut src2.name, "VIN2, etc.");
                numeric_input(ui, "Start", &mut src2.start, "V");
                numeric_input(ui, "Stop", &mut src2.stop, "V");
                numeric_input(ui, "Step", &mut src2.step, "V");
            });
        }

        ui.add_space(8.0);

        // Info
        ui.label(
            egui::RichText::new(format!("Total points: {}", self.total_points()))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
        );
    }

    fn validate(&self) -> Result<(), String> {
        DcConfig::validate(self)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep_source_new() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, 0.1);
        assert_eq!(src.name, "VIN");
        assert!((src.start - 0.0).abs() < 1e-10);
        assert!((src.stop - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_sweep_source_num_points() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, 1.0);
        assert_eq!(src.num_points(), 6); // 0, 1, 2, 3, 4, 5
    }

    #[test]
    fn test_sweep_source_validate_valid() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, 0.1);
        assert!(src.validate().is_ok());
    }

    #[test]
    fn test_sweep_source_validate_no_name() {
        let src = DcSweepSource::new("", 0.0, 5.0, 0.1);
        assert!(src.validate().is_err());
    }

    #[test]
    fn test_sweep_source_validate_zero_step() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, 0.0);
        assert!(src.validate().is_err());
    }

    #[test]
    fn test_sweep_source_validate_wrong_direction() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, -0.1);
        assert!(src.validate().is_err());
    }

    #[test]
    fn test_sweep_source_to_spice() {
        let src = DcSweepSource::new("VIN", 0.0, 5.0, 0.1);
        let spice = src.to_spice();
        assert!(spice.contains("VIN"));
    }

    #[test]
    fn test_config_new() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 0.1);
        assert_eq!(cfg.source1.name, "VIN");
        assert!(cfg.source2.is_none());
    }

    #[test]
    fn test_config_with_second() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 0.1)
            .with_second_source(DcSweepSource::new("R1", 1e3, 10e3, 1e3));
        assert!(cfg.source2.is_some());
        assert!(cfg.is_nested());
    }

    #[test]
    fn test_config_validate_valid() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 0.1);
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_config_validate_invalid_source1() {
        let cfg = DcConfig::new("", 0.0, 5.0, 0.1);
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_config_to_spice_single() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 0.1);
        let spice = cfg.to_spice();
        assert!(spice.starts_with(".dc"));
        assert!(spice.contains("VIN"));
    }

    #[test]
    fn test_config_to_spice_nested() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 1.0)
            .with_second_source(DcSweepSource::new("R1", 1e3, 2e3, 100.0));
        let spice = cfg.to_spice();
        assert!(spice.contains("VIN"));
        assert!(spice.contains("R1"));
    }

    #[test]
    fn test_total_points_single() {
        let cfg = DcConfig::new("VIN", 0.0, 5.0, 1.0);
        assert_eq!(cfg.total_points(), 6);
    }

    #[test]
    fn test_total_points_nested() {
        let cfg = DcConfig::new("VIN", 0.0, 2.0, 1.0) // 3 points
            .with_second_source(DcSweepSource::new("R1", 0.0, 1.0, 0.5)); // 3 points
        assert_eq!(cfg.total_points(), 9); // 3 * 3
    }

    #[test]
    fn test_dialog_tab_name() {
        let cfg = DcConfig::default();
        assert_eq!(cfg.name(), "DC");
    }

    #[test]
    fn test_dialog_tab_reset() {
        let mut cfg = DcConfig::new("VTEST", 1.0, 10.0, 1.0);
        cfg.reset();
        // After reset, should have valid default values
        assert_eq!(cfg.source1.name, "VIN"); // Reset to default
        assert!((cfg.source1.stop - 5.0).abs() < 0.01); // Reset to default (0-5V)
        assert!(cfg.validate().is_ok()); // Defaults should be valid
    }
}
