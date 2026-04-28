//! Sensitivity Analysis Configuration
//!
//! Configuration for sensitivity analysis (.sens).
//! Computes the sensitivity of an output to all circuit parameters.

use egui::Ui;

/// Sensitivity analysis type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SensType {
    #[default]
    Dc,
    Ac,
}

impl SensType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Dc => "DC",
            Self::Ac => "AC",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Dc => "dc",
            Self::Ac => "ac",
        }
    }
    pub fn all() -> &'static [SensType] {
        &[Self::Dc, Self::Ac]
    }
}

/// Sensitivity analysis configuration
#[derive(Debug, Clone)]
pub struct SensConfig {
    /// Output expression (node voltage or current)
    pub output_expr: String,
    /// Analysis type (DC or AC)
    pub sens_type: SensType,
    /// AC frequency (only used for AC sens)
    pub ac_freq: f64,
    /// Include parameter sensitivities
    pub include_params: bool,
    /// Include device sensitivities
    pub include_devices: bool,
    /// Threshold for reporting
    pub threshold: f64,
}

impl Default for SensConfig {
    fn default() -> Self {
        Self {
            output_expr: "V(OUT)".into(),
            sens_type: SensType::Dc,
            ac_freq: 1e6,
            include_params: true,
            include_devices: true,
            threshold: 1e-9,
        }
    }
}

impl SensConfig {
    pub fn new(output: &str) -> Self {
        Self {
            output_expr: output.to_string(),
            ..Default::default()
        }
    }

    pub fn with_type(mut self, t: SensType) -> Self {
        self.sens_type = t;
        self
    }
    pub fn with_ac_freq(mut self, f: f64) -> Self {
        self.ac_freq = f;
        self
    }

    pub fn to_spice(&self) -> String {
        match self.sens_type {
            SensType::Dc => format!(".sens {}", self.output_expr),
            SensType::Ac => format!(
                ".sens {} ac {}",
                self.output_expr,
                format_freq(self.ac_freq)
            ),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.output_expr.is_empty() {
            return Err("Output expression required".into());
        }
        if self.sens_type == SensType::Ac && self.ac_freq <= 0.0 {
            return Err("AC frequency must be positive".into());
        }
        if self.threshold < 0.0 {
            return Err("Threshold cannot be negative".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct SensDialogState {
    pub output_expr: String,
    pub sens_type_idx: usize,
    pub ac_freq: String,
    pub include_params: bool,
    pub include_devices: bool,
    pub initialized: bool,
}

impl SensDialogState {
    pub fn from_config(config: &SensConfig) -> Self {
        Self {
            output_expr: config.output_expr.clone(),
            sens_type_idx: match config.sens_type {
                SensType::Dc => 0,
                SensType::Ac => 1,
            },
            ac_freq: format_freq(config.ac_freq),
            include_params: config.include_params,
            include_devices: config.include_devices,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<SensConfig, String> {
        let sens_type = match self.sens_type_idx {
            0 => SensType::Dc,
            _ => SensType::Ac,
        };
        let freq = super::options::parse_si_value(&self.ac_freq).unwrap_or(1e6);
        let config = SensConfig {
            output_expr: self.output_expr.clone(),
            sens_type,
            ac_freq: freq,
            include_params: self.include_params,
            include_devices: self.include_devices,
            threshold: 1e-9,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&SensConfig::default());
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Sensitivity Analysis");
        ui.label(egui::RichText::new("Compute output sensitivity to all parameters").weak());
        ui.add_space(8.0);

        egui::Grid::new("sens_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Output:");
                ui.add(egui::TextEdit::singleline(&mut self.output_expr).desired_width(120.0));
                ui.end_row();
            });
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.sens_type_idx, 0, "DC Sensitivity");
            ui.selectable_value(&mut self.sens_type_idx, 1, "AC Sensitivity");
        });

        if self.sens_type_idx == 1 {
            ui.horizontal(|ui| {
                ui.label("AC Frequency:");
                ui.add(egui::TextEdit::singleline(&mut self.ac_freq).desired_width(100.0));
            });
        }
        ui.add_space(4.0);
        ui.checkbox(&mut self.include_params, "Include Parameter Sensitivities");
        ui.checkbox(&mut self.include_devices, "Include Device Sensitivities");
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

