//! Noise Analysis Configuration
//!
//! Configuration for noise analysis (.noise).

use super::ac::FrequencySweep;
use super::framework::{DialogTab, labeled_checkbox, labeled_input, numeric_input};
use egui::Ui;

// =============================================================================
// Noise Source Type
// =============================================================================

/// Type of noise source contribution
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NoiseSourceType {
    /// All sources
    #[default]
    All,
    /// Resistor thermal noise only
    ResistorOnly,
    /// Device noise only
    DeviceOnly,
}

impl NoiseSourceType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All Sources",
            Self::ResistorOnly => "Resistor Only",
            Self::DeviceOnly => "Device Only",
        }
    }

    /// All types
    pub fn all() -> &'static [NoiseSourceType] {
        &[Self::All, Self::ResistorOnly, Self::DeviceOnly]
    }
}

// =============================================================================
// Noise Config
// =============================================================================

/// Noise analysis configuration
#[derive(Debug, Clone)]
pub struct NoiseConfig {
    /// Output node
    pub output_node: String,
    /// Reference node (ground if empty)
    pub reference_node: String,
    /// Input source (for input-referred noise)
    pub input_source: String,
    /// Sweep type
    pub sweep_type: FrequencySweep,
    /// Points per decade/octave
    pub num_points: u32,
    /// Start frequency
    pub start_freq: f64,
    /// Stop frequency
    pub stop_freq: f64,
    /// Points per summary
    pub summary_points: u32,
    /// Noise source filter
    pub source_filter: NoiseSourceType,
    /// Save noise contributions
    pub save_contributions: bool,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            output_node: "out".to_string(),  // Commercial default: output node
            reference_node: String::new(),   // Ground reference is optional
            input_source: "VIN".to_string(), // Commercial default: input source
            sweep_type: FrequencySweep::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e9,
            summary_points: 1,
            source_filter: NoiseSourceType::All,
            save_contributions: true,
        }
    }
}

impl NoiseConfig {
    /// Create new config
    pub fn new(output: &str, input_source: &str) -> Self {
        Self {
            output_node: output.to_string(),
            input_source: input_source.to_string(),
            ..Default::default()
        }
    }

    /// Set frequency range
    pub fn with_freq_range(mut self, start: f64, stop: f64, points: u32) -> Self {
        self.start_freq = start;
        self.stop_freq = stop;
        self.num_points = points;
        self
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let output = if self.reference_node.is_empty() {
            format!("V({})", self.output_node)
        } else {
            format!("V({},{})", self.output_node, self.reference_node)
        };

        format!(
            ".noise {} {} {} {} {} {}",
            output,
            self.input_source,
            self.sweep_type.spice_keyword(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.output_node.is_empty() {
            return Err("Output node is required".to_string());
        }

        if self.input_source.is_empty() {
            return Err("Input source is required".to_string());
        }

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
}

impl DialogTab for NoiseConfig {
    fn name(&self) -> &str {
        "Noise"
    }

    fn description(&self) -> &str {
        "Small-signal noise analysis"
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Noise Analysis");
        ui.add_space(8.0);

        // Output specification
        ui.label("Output");
        ui.group(|ui| {
            labeled_input(ui, "Output Node", &mut self.output_node, "out, vout, etc.");
            labeled_input(ui, "Reference", &mut self.reference_node, "(optional)");
        });

        ui.add_space(8.0);

        // Input source
        ui.label("Input Source");
        labeled_input(ui, "Source", &mut self.input_source, "VIN, etc.");

        ui.add_space(8.0);

        // Frequency sweep
        ui.label("Frequency Sweep");
        ui.group(|ui| {
            egui::ComboBox::from_id_salt("noise_sweep")
                .selected_text(self.sweep_type.display_name())
                .show_ui(ui, |ui| {
                    for s in FrequencySweep::all() {
                        ui.selectable_value(&mut self.sweep_type, *s, s.display_name());
                    }
                });

            numeric_input(ui, "Start", &mut self.start_freq, "Hz");
            numeric_input(ui, "Stop", &mut self.stop_freq, "Hz");

            let mut points = self.num_points as f64;
            if numeric_input(ui, "Points", &mut points, "/decade") {
                self.num_points = points.max(1.0) as u32;
            }
        });

        ui.add_space(8.0);

        // Options
        ui.label("Options");
        ui.group(|ui| {
            egui::ComboBox::from_id_salt("noise_filter")
                .selected_text(self.source_filter.display_name())
                .show_ui(ui, |ui| {
                    for t in NoiseSourceType::all() {
                        ui.selectable_value(&mut self.source_filter, *t, t.display_name());
                    }
                });

            labeled_checkbox(ui, "Save Contributions", &mut self.save_contributions);
        });
    }

    fn validate(&self) -> Result<(), String> {
        NoiseConfig::validate(self)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// Tests
// =============================================================================

