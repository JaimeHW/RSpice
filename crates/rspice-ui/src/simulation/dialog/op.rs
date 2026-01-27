//! DC Operating Point Configuration
//!
//! Configuration for DC operating point analysis (.op).
//! The simplest analysis - computes the DC bias point with all AC sources zeroed.

use egui::Ui;

/// DC operating point analysis configuration
#[derive(Debug, Clone)]
pub struct OpConfig {
    /// Save all node voltages
    pub save_all: bool,
    /// Include device operating point parameters
    pub save_op_info: bool,
    /// Temperature for analysis (Celsius)
    pub temperature: f64,
    /// Enable specific node saves
    pub save_nodes: Vec<String>,
    /// Number of Gmin steps (convergence aid)
    pub gmin_steps: u32,
    /// Source stepping for convergence
    pub source_stepping: bool,
}

impl Default for OpConfig {
    fn default() -> Self {
        Self {
            save_all: true,
            save_op_info: true,
            temperature: 27.0,
            save_nodes: Vec::new(),
            gmin_steps: 0,
            source_stepping: false,
        }
    }
}

impl OpConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_temperature(mut self, temp: f64) -> Self {
        self.temperature = temp;
        self
    }

    pub fn with_save_nodes(mut self, nodes: Vec<String>) -> Self {
        self.save_nodes = nodes;
        self.save_all = false;
        self
    }

    pub fn with_gmin_steps(mut self, steps: u32) -> Self {
        self.gmin_steps = steps;
        self
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = ".op".to_string();
        if self.gmin_steps > 0 {
            cmd.push_str(&format!(" gmin_steps={}", self.gmin_steps));
        }
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.temperature < -273.15 {
            return Err("Temperature cannot be below absolute zero".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpDialogState {
    pub save_all: bool,
    pub save_op_info: bool,
    pub temperature: String,
    pub source_stepping: bool,
    pub gmin_steps: String,
    pub initialized: bool,
}

impl OpDialogState {
    pub fn from_config(config: &OpConfig) -> Self {
        Self {
            save_all: config.save_all,
            save_op_info: config.save_op_info,
            temperature: format!("{}", config.temperature),
            source_stepping: config.source_stepping,
            gmin_steps: config.gmin_steps.to_string(),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<OpConfig, String> {
        let temp: f64 = self
            .temperature
            .parse()
            .map_err(|_| "Invalid temperature")?;
        let gmin: u32 = self.gmin_steps.parse().unwrap_or(0);
        let config = OpConfig {
            save_all: self.save_all,
            save_op_info: self.save_op_info,
            temperature: temp,
            save_nodes: Vec::new(),
            gmin_steps: gmin,
            source_stepping: self.source_stepping,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&OpConfig::default());
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("DC Operating Point");
        ui.label(egui::RichText::new("Computes DC bias point with AC sources zeroed").weak());
        ui.add_space(8.0);

        egui::Grid::new("op_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Temperature (°C):");
                ui.add(egui::TextEdit::singleline(&mut self.temperature).desired_width(80.0));
                ui.end_row();
                ui.label("Gmin Steps:");
                ui.add(egui::TextEdit::singleline(&mut self.gmin_steps).desired_width(60.0));
                ui.end_row();
            });
        ui.add_space(4.0);
        ui.checkbox(&mut self.save_all, "Save All Nodes");
        ui.checkbox(&mut self.save_op_info, "Save Device Operating Points");
        ui.checkbox(&mut self.source_stepping, "Enable Source Stepping");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let c = OpConfig::default();
        assert!(c.save_all);
        assert_eq!(c.temperature, 27.0);
    }
    #[test]
    fn test_new() {
        let c = OpConfig::new();
        assert!(c.save_all);
    }
    #[test]
    fn test_with_temp() {
        let c = OpConfig::new().with_temperature(85.0);
        assert_eq!(c.temperature, 85.0);
    }
    #[test]
    fn test_with_gmin() {
        let c = OpConfig::new().with_gmin_steps(10);
        assert_eq!(c.gmin_steps, 10);
    }
    #[test]
    fn test_with_nodes() {
        let c = OpConfig::new().with_save_nodes(vec!["N1".into()]);
        assert!(!c.save_all);
    }
    #[test]
    fn test_validate_ok() {
        assert!(OpConfig::default().validate().is_ok());
    }
    #[test]
    fn test_validate_cold() {
        let mut c = OpConfig::default();
        c.temperature = -300.0;
        assert!(c.validate().is_err());
    }
    #[test]
    fn test_to_spice() {
        let s = OpConfig::default().to_spice();
        assert!(s.contains(".op"));
    }
    #[test]
    fn test_to_spice_gmin() {
        let s = OpConfig::new().with_gmin_steps(5).to_spice();
        assert!(s.contains("gmin_steps=5"));
    }
    #[test]
    fn test_dialog_roundtrip() {
        let s = OpDialogState::from_config(&OpConfig::default());
        assert!(s.to_config().is_ok());
    }
    #[test]
    fn test_reset() {
        let mut c = OpConfig::new().with_temperature(125.0);
        c.reset();
        assert_eq!(c.temperature, 27.0);
    }
}
