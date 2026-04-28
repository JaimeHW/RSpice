//! Periodic Transfer Function (PXF) Analysis Configuration
//!
//! Configuration for periodic transfer function analysis around a PSS operating point.
//! PXF computes the transfer function from input to output including frequency
//! conversion effects.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband-to-sideband transfer functions
//! - Input/output node specification
//! - Frequency sweep with multiple sweep types
//!
//! # Example SPICE Output
//!
//! ```text
//! .pxf dec 10 1k 1G
//! + output=VOUT outsideband=1 input=VIN
//! ```

use super::options::parse_si_value;
use egui::Ui;

// =============================================================================
// PXF Sweep Type
// =============================================================================

/// Type of frequency sweep for PXF analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PxfSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PxfSweepType {
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
    pub fn all() -> &'static [PxfSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// PXF Configuration
// =============================================================================

/// Periodic transfer function (PXF) analysis configuration
#[derive(Debug, Clone)]
pub struct PxfConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PxfSweepType,
    /// Output node name
    pub output_node: String,
    /// Output reference node
    pub output_ref: String,
    /// Output sideband index
    pub output_sideband: i32,
    /// Input source name
    pub input_source: String,
    /// Maximum sideband index
    pub max_sideband: i32,
    /// Fundamental frequency from PSS
    pub fundamental_freq: f64,
}

impl Default for PxfConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e3,
            stop_freq: 1e9,
            num_points: 10,
            sweep_type: PxfSweepType::Decade,
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            output_sideband: 1,
            input_source: "VIN".to_string(),
            max_sideband: 5,
            fundamental_freq: 0.0,
        }
    }
}

impl PxfConfig {
    /// Create new PXF config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Set output node and sideband
    pub fn with_output(mut self, node: &str, sideband: i32) -> Self {
        self.output_node = node.to_uppercase();
        self.output_sideband = sideband;
        self
    }

    /// Set input source
    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = source.to_uppercase();
        self
    }

    /// Set sideband range
    pub fn with_sidebands(mut self, max: i32) -> Self {
        self.max_sideband = max.abs();
        self
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            PxfSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            PxfSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            PxfSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pxf {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        if !self.output_node.is_empty() {
            if self.output_ref.is_empty() {
                cmd.push_str(&format!(" output={}", self.output_node));
            } else {
                cmd.push_str(&format!(
                    " output=({},{})",
                    self.output_node, self.output_ref
                ));
            }
        }

        cmd.push_str(&format!(" outsideband={}", self.output_sideband));

        if !self.input_source.is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source));
        }

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

        cmd
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
        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }
        if self.input_source.is_empty() {
            return Err("Input source must be specified".to_string());
        }
        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }
        Ok(())
    }

    /// Reset to defaults
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// Dialog state with string buffers
#[derive(Debug, Clone, Default)]
pub struct PxfDialogState {
    pub start_freq: String,
    pub stop_freq: String,
    pub num_points: String,
    pub sweep_type_idx: usize,
    pub output_node: String,
    pub output_ref: String,
    pub output_sideband: String,
    pub input_source: String,
    pub max_sideband: String,
    pub initialized: bool,
}

impl PxfDialogState {
    /// Initialize from config
    pub fn from_config(config: &PxfConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PxfSweepType::Decade => 0,
                PxfSweepType::Octave => 1,
                PxfSweepType::Linear => 2,
            },
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            output_sideband: config.output_sideband.to_string(),
            input_source: config.input_source.clone(),
            max_sideband: config.max_sideband.to_string(),
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PxfConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;
        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;
        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;
        let out_sb: i32 = self
            .output_sideband
            .parse()
            .map_err(|_| "Invalid sideband")?;
        let max_sb: i32 = self
            .max_sideband
            .parse()
            .map_err(|_| "Invalid max sideband")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PxfSweepType::Decade,
            1 => PxfSweepType::Octave,
            _ => PxfSweepType::Linear,
        };

        let config = PxfConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            output_sideband: out_sb,
            input_source: self.input_source.clone(),
            max_sideband: max_sb,
            fundamental_freq: 0.0,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PxfConfig::default());
        }
    }

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Periodic Transfer Function");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Transfer function around PSS with frequency conversion").weak(),
        );
        ui.add_space(12.0);

        // Frequency Sweep
        ui.group(|ui| {
            ui.label(egui::RichText::new("Frequency Sweep").strong());
            egui::Grid::new("pxf_freq_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Start:");
                    ui.add(egui::TextEdit::singleline(&mut self.start_freq).desired_width(120.0));
                    ui.end_row();
                    ui.label("Stop:");
                    ui.add(egui::TextEdit::singleline(&mut self.stop_freq).desired_width(120.0));
                    ui.end_row();
                    ui.label("Points:");
                    ui.add(egui::TextEdit::singleline(&mut self.num_points).desired_width(120.0));
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // I/O Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Input/Output").strong());
            egui::Grid::new("pxf_io_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Input Source:");
                    ui.add(egui::TextEdit::singleline(&mut self.input_source).desired_width(120.0));
                    ui.end_row();
                    ui.label("Output Node:");
                    ui.add(egui::TextEdit::singleline(&mut self.output_node).desired_width(120.0));
                    ui.end_row();
                    ui.label("Out Sideband:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.output_sideband).desired_width(60.0),
                    );
                    ui.end_row();
                    ui.label("Max Sideband:");
                    ui.add(egui::TextEdit::singleline(&mut self.max_sideband).desired_width(60.0));
                    ui.end_row();
                });
        });
    }
}

// =============================================================================
// Helper
// =============================================================================

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
