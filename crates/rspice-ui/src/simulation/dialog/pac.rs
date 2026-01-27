//! Periodic AC (PAC) Analysis Configuration
//!
//! Configuration for periodic AC analysis around a PSS operating point.
//! PAC computes the frequency conversion (mixing) behavior of periodically
//! operating circuits like mixers, samplers, and switched-capacitor filters.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Sideband range specification
//! - Input source and output node configuration
//! - Frequency sweep with decade/linear options
//! - Conversion matrix results
//!
//! # Example SPICE Output
//!
//! ```text
//! .pac dec 10 1k 1G
//! + maxsideband=5 input=VRF output=VOUT
//! ```

use super::options::parse_si_value;
use egui::Ui;

// =============================================================================
// PAC Sweep Type
// =============================================================================

/// Type of frequency sweep for PAC analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PacSweepType {
    /// Decades (logarithmic)
    #[default]
    Decade,
    /// Octaves (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl PacSweepType {
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
    pub fn all() -> &'static [PacSweepType] {
        &[Self::Decade, Self::Octave, Self::Linear]
    }
}

// =============================================================================
// PAC Configuration
// =============================================================================

/// Periodic AC (PAC) analysis configuration
///
/// Commercial-grade configuration matching Cadence Spectre PAC parameters.
#[derive(Debug, Clone)]
pub struct PacConfig {
    /// Start frequency (Hz)
    pub start_freq: f64,
    /// Stop frequency (Hz)
    pub stop_freq: f64,
    /// Number of points (per decade for log, total for linear)
    pub num_points: u32,
    /// Sweep type
    pub sweep_type: PacSweepType,
    /// Maximum sideband index (both positive and negative)
    pub max_sideband: i32,
    /// Input source name (AC stimulus)
    pub input_source: String,
    /// Output node name
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// PAC magnitude (input signal magnitude)
    pub pac_magnitude: f64,
    /// Include DC sideband (band 0)
    pub include_dc: bool,
    /// Fundamental frequency from PSS (set from PSS result)
    pub fundamental_freq: f64,
}

impl Default for PacConfig {
    fn default() -> Self {
        Self {
            start_freq: 1e3, // 1 kHz
            stop_freq: 1e9,  // 1 GHz
            num_points: 10,  // 10 per decade
            sweep_type: PacSweepType::Decade,
            max_sideband: 5, // Sidebands -5 to +5
            input_source: "VRF".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            pac_magnitude: 1.0, // 1V default
            include_dc: true,
            fundamental_freq: 0.0, // Set from PSS
        }
    }
}

impl PacConfig {
    /// Create new PAC config
    pub fn new(start: f64, stop: f64, points: u32) -> Self {
        Self {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            ..Default::default()
        }
    }

    /// Set sideband range
    pub fn with_sidebands(mut self, max: i32) -> Self {
        self.max_sideband = max.abs();
        self
    }

    /// Set input source
    pub fn with_input(mut self, source: &str) -> Self {
        self.input_source = source.to_uppercase();
        self
    }

    /// Set output node
    pub fn with_output(mut self, node: &str) -> Self {
        self.output_node = node.to_uppercase();
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: PacSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set PAC magnitude
    pub fn with_magnitude(mut self, mag: f64) -> Self {
        self.pac_magnitude = mag;
        self
    }

    /// Set fundamental frequency
    pub fn with_fundamental(mut self, freq: f64) -> Self {
        self.fundamental_freq = freq;
        self
    }

    /// Total number of sidebands
    pub fn num_sidebands(&self) -> usize {
        // -max to +max inclusive
        (2 * self.max_sideband + 1) as usize
    }

    /// Get sideband indices
    pub fn sideband_indices(&self) -> Vec<i32> {
        (-self.max_sideband..=self.max_sideband).collect()
    }

    /// Total number of frequency points
    pub fn total_points(&self) -> u32 {
        match self.sweep_type {
            PacSweepType::Decade => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let decades = (self.stop_freq / self.start_freq).log10();
                (decades * self.num_points as f64).ceil() as u32 + 1
            }
            PacSweepType::Octave => {
                if self.start_freq <= 0.0 || self.stop_freq <= 0.0 {
                    return self.num_points;
                }
                let octaves = (self.stop_freq / self.start_freq).log2();
                (octaves * self.num_points as f64).ceil() as u32 + 1
            }
            PacSweepType::Linear => self.num_points,
        }
    }

    /// Generate SPICE directive
    pub fn to_spice(&self) -> String {
        let mut cmd = format!(
            ".pac {} {} {} {}",
            self.sweep_type.spice_keyword(),
            self.num_points,
            format_freq(self.start_freq),
            format_freq(self.stop_freq)
        );

        cmd.push_str(&format!(" maxsideband={}", self.max_sideband));

        if !self.input_source.is_empty() {
            cmd.push_str(&format!(" input={}", self.input_source));
        }

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

        if (self.pac_magnitude - 1.0).abs() > 0.001 {
            cmd.push_str(&format!(" pacmag={}", self.pac_magnitude));
        }

        if !self.include_dc {
            cmd.push_str(" includedc=no");
        }

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

        if self.max_sideband < 0 {
            return Err("Maximum sideband must be non-negative".to_string());
        }

        if self.input_source.is_empty() {
            return Err("Input source must be specified".to_string());
        }

        if self.output_node.is_empty() {
            return Err("Output node must be specified".to_string());
        }

        if self.pac_magnitude <= 0.0 {
            return Err("PAC magnitude must be positive".to_string());
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
pub struct PacDialogState {
    /// Start frequency buffer
    pub start_freq: String,
    /// Stop frequency buffer
    pub stop_freq: String,
    /// Points buffer
    pub num_points: String,
    /// Sweep type index
    pub sweep_type_idx: usize,
    /// Max sideband buffer
    pub max_sideband: String,
    /// Input source buffer
    pub input_source: String,
    /// Output node buffer
    pub output_node: String,
    /// Output reference buffer
    pub output_ref: String,
    /// PAC magnitude buffer
    pub pac_magnitude: String,
    /// Include DC
    pub include_dc: bool,
    /// Initialized flag
    pub initialized: bool,
}

impl PacDialogState {
    /// Initialize from config
    pub fn from_config(config: &PacConfig) -> Self {
        Self {
            start_freq: format_freq(config.start_freq),
            stop_freq: format_freq(config.stop_freq),
            num_points: config.num_points.to_string(),
            sweep_type_idx: match config.sweep_type {
                PacSweepType::Decade => 0,
                PacSweepType::Octave => 1,
                PacSweepType::Linear => 2,
            },
            max_sideband: config.max_sideband.to_string(),
            input_source: config.input_source.clone(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            pac_magnitude: config.pac_magnitude.to_string(),
            include_dc: config.include_dc,
            initialized: true,
        }
    }

    /// Convert to config
    pub fn to_config(&self) -> Result<PacConfig, String> {
        let start = parse_si_value(&self.start_freq)
            .map_err(|e| format!("Invalid start frequency: {}", e))?;

        let stop = parse_si_value(&self.stop_freq)
            .map_err(|e| format!("Invalid stop frequency: {}", e))?;

        let points: u32 = self.num_points.parse().map_err(|_| "Invalid point count")?;

        let max_sb: i32 = self.max_sideband.parse().map_err(|_| "Invalid sideband")?;

        let mag: f64 = self
            .pac_magnitude
            .parse()
            .map_err(|_| "Invalid magnitude")?;

        let sweep_type = match self.sweep_type_idx {
            0 => PacSweepType::Decade,
            1 => PacSweepType::Octave,
            _ => PacSweepType::Linear,
        };

        let config = PacConfig {
            start_freq: start,
            stop_freq: stop,
            num_points: points,
            sweep_type,
            max_sideband: max_sb,
            input_source: self.input_source.clone(),
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.clone(),
            pac_magnitude: mag,
            include_dc: self.include_dc,
            fundamental_freq: 0.0,
        };

        config.validate()?;
        Ok(config)
    }

    /// Ensure initialized
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PacConfig::default());
        }
    }

    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Periodic AC Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Small-signal AC analysis around periodic steady-state (requires PSS)",
            )
            .weak(),
        );
        ui.add_space(12.0);

        // Frequency Sweep
        ui.group(|ui| {
            ui.label(egui::RichText::new("Frequency Sweep").strong());
            ui.add_space(4.0);

            egui::Grid::new("pac_freq_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Start:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.start_freq)
                            .desired_width(120.0)
                            .hint_text("1k"),
                    );
                    ui.end_row();

                    ui.label("Stop:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.stop_freq)
                            .desired_width(120.0)
                            .hint_text("1G"),
                    );
                    ui.end_row();

                    ui.label("Points:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.num_points)
                            .desired_width(120.0)
                            .hint_text("10"),
                    );
                    ui.end_row();

                    ui.label("Sweep:");
                    let sweeps = ["Decade", "Octave", "Linear"];
                    egui::ComboBox::from_id_salt("pac_sweep")
                        .selected_text(sweeps[self.sweep_type_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in sweeps.iter().enumerate() {
                                if ui
                                    .selectable_label(self.sweep_type_idx == idx, *name)
                                    .clicked()
                                {
                                    self.sweep_type_idx = idx;
                                }
                            }
                        });
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Sideband Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Sideband Configuration").strong());
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Max Sideband:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.max_sideband)
                        .desired_width(60.0)
                        .hint_text("5"),
                );
                ui.label("(±N around LO)");
            });

            ui.checkbox(&mut self.include_dc, "Include DC (sideband 0)");
        });

        ui.add_space(8.0);

        // I/O Configuration
        ui.group(|ui| {
            ui.label(egui::RichText::new("Input/Output").strong());
            ui.add_space(4.0);

            egui::Grid::new("pac_io_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Input Source:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.input_source)
                            .desired_width(120.0)
                            .hint_text("VRF"),
                    );
                    ui.end_row();

                    ui.label("Output Node:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.output_node)
                            .desired_width(120.0)
                            .hint_text("VOUT"),
                    );
                    ui.end_row();

                    ui.label("Reference:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.output_ref)
                            .desired_width(120.0)
                            .hint_text("(ground)"),
                    );
                    ui.end_row();

                    ui.label("PAC Magnitude:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.pac_magnitude)
                            .desired_width(120.0)
                            .hint_text("1"),
                    );
                    ui.end_row();
                });
        });

        // Info footer
        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "{} sidebands × ~{} freq points",
                    config.num_sidebands(),
                    config.total_points()
                ))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
            );
        }
    }
}

// =============================================================================
// Helper Functions
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // PacConfig Basic Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let cfg = PacConfig::default();
        assert_eq!(cfg.start_freq, 1e3);
        assert_eq!(cfg.stop_freq, 1e9);
        assert_eq!(cfg.max_sideband, 5);
        assert_eq!(cfg.input_source, "VRF");
        assert_eq!(cfg.output_node, "VOUT");
    }

    #[test]
    fn test_config_new() {
        let cfg = PacConfig::new(1e6, 1e9, 20);
        assert_eq!(cfg.start_freq, 1e6);
        assert_eq!(cfg.stop_freq, 1e9);
        assert_eq!(cfg.num_points, 20);
    }

    #[test]
    fn test_config_builder_chain() {
        let cfg = PacConfig::new(1e6, 1e9, 10)
            .with_sidebands(7)
            .with_input("VIN")
            .with_output("VIF")
            .with_magnitude(0.5);

        assert_eq!(cfg.max_sideband, 7);
        assert_eq!(cfg.input_source, "VIN");
        assert_eq!(cfg.output_node, "VIF");
        assert_eq!(cfg.pac_magnitude, 0.5);
    }

    #[test]
    fn test_config_with_sweep_type() {
        let cfg = PacConfig::default().with_sweep_type(PacSweepType::Linear);
        assert_eq!(cfg.sweep_type, PacSweepType::Linear);
    }

    #[test]
    fn test_config_with_fundamental() {
        let cfg = PacConfig::default().with_fundamental(1e9);
        assert_eq!(cfg.fundamental_freq, 1e9);
    }

    // =========================================================================
    // Sideband Tests
    // =========================================================================

    #[test]
    fn test_num_sidebands() {
        let cfg = PacConfig::default().with_sidebands(5);
        // -5 to +5 = 11 sidebands
        assert_eq!(cfg.num_sidebands(), 11);
    }

    #[test]
    fn test_sideband_indices() {
        let cfg = PacConfig::default().with_sidebands(2);
        let indices = cfg.sideband_indices();
        assert_eq!(indices, vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_sideband_zero() {
        let cfg = PacConfig::default().with_sidebands(0);
        assert_eq!(cfg.num_sidebands(), 1);
        assert_eq!(cfg.sideband_indices(), vec![0]);
    }

    #[test]
    fn test_negative_sideband_converted() {
        let cfg = PacConfig::default().with_sidebands(-3);
        assert_eq!(cfg.max_sideband, 3);
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    #[test]
    fn test_validate_valid() {
        let cfg = PacConfig::default();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_start() {
        let mut cfg = PacConfig::default();
        cfg.start_freq = 0.0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_freq() {
        let mut cfg = PacConfig::default();
        cfg.start_freq = -1e6;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_start_after_stop() {
        let mut cfg = PacConfig::default();
        cfg.start_freq = 1e9;
        cfg.stop_freq = 1e6;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_points() {
        let mut cfg = PacConfig::default();
        cfg.num_points = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_negative_sideband() {
        let mut cfg = PacConfig::default();
        cfg.max_sideband = -1;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_empty_input() {
        let mut cfg = PacConfig::default();
        cfg.input_source = String::new();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_empty_output() {
        let mut cfg = PacConfig::default();
        cfg.output_node = String::new();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_zero_magnitude() {
        let mut cfg = PacConfig::default();
        cfg.pac_magnitude = 0.0;
        assert!(cfg.validate().is_err());
    }

    // =========================================================================
    // SPICE Generation Tests
    // =========================================================================

    #[test]
    fn test_to_spice_basic() {
        let cfg = PacConfig::default();
        let spice = cfg.to_spice();

        assert!(spice.starts_with(".pac"));
        assert!(spice.contains("dec"));
        assert!(spice.contains("maxsideband=5"));
    }

    #[test]
    fn test_to_spice_input_output() {
        let cfg = PacConfig::default();
        let spice = cfg.to_spice();

        assert!(spice.contains("input=VRF"));
        assert!(spice.contains("output=VOUT"));
    }

    #[test]
    fn test_to_spice_differential_output() {
        let mut cfg = PacConfig::default();
        cfg.output_ref = "VSS".to_string();
        let spice = cfg.to_spice();

        assert!(spice.contains("output=(VOUT,VSS)"));
    }

    #[test]
    fn test_to_spice_custom_magnitude() {
        let cfg = PacConfig::default().with_magnitude(0.5);
        let spice = cfg.to_spice();

        assert!(spice.contains("pacmag=0.5"));
    }

    #[test]
    fn test_to_spice_no_dc() {
        let mut cfg = PacConfig::default();
        cfg.include_dc = false;
        let spice = cfg.to_spice();

        assert!(spice.contains("includedc=no"));
    }

    // =========================================================================
    // Total Points Tests
    // =========================================================================

    #[test]
    fn test_total_points_decade() {
        let cfg = PacConfig::new(1e3, 1e9, 10);
        let points = cfg.total_points();
        // 6 decades * 10 points + 1 = 61
        assert!(points >= 60);
    }

    #[test]
    fn test_total_points_linear() {
        let cfg = PacConfig::new(1e6, 1e9, 100).with_sweep_type(PacSweepType::Linear);
        assert_eq!(cfg.total_points(), 100);
    }

    // =========================================================================
    // PacSweepType Tests
    // =========================================================================

    #[test]
    fn test_sweep_type_names() {
        assert_eq!(PacSweepType::Decade.display_name(), "Decade");
        assert_eq!(PacSweepType::Octave.display_name(), "Octave");
        assert_eq!(PacSweepType::Linear.display_name(), "Linear");
    }

    #[test]
    fn test_sweep_type_keywords() {
        assert_eq!(PacSweepType::Decade.spice_keyword(), "dec");
        assert_eq!(PacSweepType::Octave.spice_keyword(), "oct");
        assert_eq!(PacSweepType::Linear.spice_keyword(), "lin");
    }

    #[test]
    fn test_sweep_type_all() {
        assert_eq!(PacSweepType::all().len(), 3);
    }

    // =========================================================================
    // Reset Test
    // =========================================================================

    #[test]
    fn test_config_reset() {
        let mut cfg = PacConfig::new(1e9, 10e9, 50).with_sidebands(10);
        cfg.reset();

        assert_eq!(cfg.max_sideband, 5);
        assert_eq!(cfg.start_freq, 1e3);
    }

    // =========================================================================
    // Dialog State Tests
    // =========================================================================

    #[test]
    fn test_dialog_state_from_config() {
        let cfg = PacConfig::default().with_sidebands(7);
        let state = PacDialogState::from_config(&cfg);

        assert!(state.initialized);
        assert_eq!(state.max_sideband, "7");
    }

    #[test]
    fn test_dialog_state_to_config() {
        let mut state = PacDialogState::from_config(&PacConfig::default());
        state.max_sideband = "10".to_string();

        let result = state.to_config();
        assert!(result.is_ok());

        let cfg = result.unwrap();
        assert_eq!(cfg.max_sideband, 10);
    }

    #[test]
    fn test_dialog_state_invalid_freq() {
        let mut state = PacDialogState::from_config(&PacConfig::default());
        state.start_freq = "invalid".to_string();

        let result = state.to_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_dialog_state_ensure_initialized() {
        let mut state = PacDialogState::default();
        assert!(!state.initialized);

        state.ensure_initialized();
        assert!(state.initialized);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_large_sideband() {
        let cfg = PacConfig::default().with_sidebands(20);
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.num_sidebands(), 41);
    }

    #[test]
    fn test_mixer_typical_config() {
        // Typical mixer: RF input, IF output, ±5 sidebands
        let cfg = PacConfig::new(1e6, 100e6, 20)
            .with_sidebands(5)
            .with_input("VRF")
            .with_output("VIF");

        assert!(cfg.validate().is_ok());
        let spice = cfg.to_spice();
        assert!(spice.contains("VRF"));
        assert!(spice.contains("VIF"));
    }
}
